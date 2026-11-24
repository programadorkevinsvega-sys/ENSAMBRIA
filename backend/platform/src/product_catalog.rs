use ensambria_shared_cache::TtlCache;
use ensambria_platform_product_catalog_contracts::{
    ContractFuture, ProductCatalog as ProductCatalogPort, ProductCatalogFailure,
};
use serde::Deserialize;
use serde_json::Value;
use sqlx::{PgPool, types::Json};
use std::collections::HashSet;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use uuid::Uuid;

#[derive(Clone)]
pub struct ProductCatalog {
    pool: PgPool,
    cache: Option<TtlCache>,
}

impl ProductCatalog {
    pub fn new(pool: PgPool) -> Self {
        Self { pool, cache: None }
    }

    pub fn with_cache(pool: PgPool, cache: TtlCache) -> Self {
        Self {
            pool,
            cache: Some(cache),
        }
    }

    pub async fn get_active_product(
        &self,
        product_id: Uuid,
    ) -> Result<Option<ProductCatalogEntry>, ProductCatalogError> {
        let product = sqlx::query_as::<_, (Uuid, String, String, String, i32, String, Json<Value>, i64)>(
            "SELECT product_id, product_key, display_name, brand_name, configuration_version, schema_version, configuration, extract(epoch FROM published_at)::bigint FROM platform_product_catalog.active_product_catalog WHERE product_id = $1",
        )
        .bind(product_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ProductCatalogError::Storage)?;

        let Some((
            id,
            key,
            display_name,
            brand_name,
            version,
            schema_version,
            configuration,
            published_at_epoch,
        )) = product
        else {
            return Ok(None);
        };

        let domains = sqlx::query_as::<_, (String, String)>(
            "SELECT domain_name, status FROM platform_product_catalog.product_domains WHERE product_id = $1 AND status = 'active' ORDER BY domain_name",
        )
        .bind(product_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|_| ProductCatalogError::Storage)?
        .into_iter()
        .map(|(domain_name, status)| ProductDomain { domain_name, status })
        .collect();

        let markets = sqlx::query_as::<_, (String, String, String, String)>(
            "SELECT country_code::text, currency_code::text, locale, status FROM platform_product_catalog.product_markets WHERE product_id = $1 AND status = 'enabled' ORDER BY country_code, locale",
        )
        .bind(product_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|_| ProductCatalogError::Storage)?
        .into_iter()
        .map(|(country_code, currency_code, locale, status)| ProductMarket {
            country_code,
            currency_code,
            locale,
            status,
        })
        .collect();

        let surfaces = sqlx::query_as::<_, (String, i32, String)>(
            "SELECT surface_code, configuration_version, status FROM platform_product_catalog.product_surfaces WHERE product_id = $1 AND status = 'enabled' ORDER BY surface_code",
        )
        .bind(product_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|_| ProductCatalogError::Storage)?
        .into_iter()
        .map(
            |(surface_code, configuration_version, status)| ProductSurface {
                surface_code,
                configuration_version,
                status,
            },
        )
        .collect();

        let capabilities = sqlx::query_as::<_, (String, String, String, String)>(
            "SELECT pc.capability_code, cc.contract_version, cc.implementation_status, pc.status FROM platform_product_catalog.product_capabilities pc JOIN platform_product_catalog.capability_catalog cc USING (capability_code) WHERE pc.product_id = $1 AND pc.status = 'enabled' AND cc.implementation_status = 'available' ORDER BY pc.capability_code",
        )
        .bind(product_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|_| ProductCatalogError::Storage)?
        .into_iter()
        .map(
            |(capability_code, contract_version, implementation_status, product_status)| {
                ProductCapability {
                    capability_code,
                    contract_version,
                    implementation_status,
                    product_status,
                }
            },
        )
        .collect();

        validate_product_configuration(&schema_version, &configuration.0)?;
        Ok(Some(ProductCatalogEntry {
            product_id: id,
            product_key: key,
            display_name,
            brand_name,
            configuration_version: version,
            configuration_schema_version: schema_version,
            configuration: configuration.0,
            published_at: UNIX_EPOCH + Duration::from_secs(published_at_epoch.max(0) as u64),
            domains,
            markets,
            surfaces,
            capabilities,
        }))
    }

    pub async fn resolve_active_domain(
        &self,
        domain_name: &str,
    ) -> Result<Option<ProductCatalogEntry>, ProductCatalogError> {
        let domain = normalize_domain(domain_name)?;
        let cache_key = format!("domain:v1:{domain}");
        if let Some(cache) = &self.cache {
            match cache.get(&cache_key).await {
                Ok(Some(cached_id)) => {
                    if let Ok(encoded_id) = std::str::from_utf8(&cached_id)
                        && let Ok(product_id) = Uuid::parse_str(encoded_id)
                        && let Some(product) = self.get_active_product(product_id).await?
                    {
                        return Ok(Some(product));
                    }
                    let _ = cache.delete(&cache_key).await;
                }
                Ok(None) => {}
                Err(_) => {
                    tracing::warn!(
                        event = "cache.redis.read_failed",
                        component = "platform",
                        operation = "product_domain.resolve",
                        result = "fallback"
                    );
                }
            }
        }

        let product_id: Option<Uuid> = sqlx::query_scalar(
            "SELECT p.product_id FROM platform_product_catalog.active_product_catalog p JOIN platform_product_catalog.product_domains d ON d.product_id = p.product_id WHERE d.domain_name = $1 AND d.status = 'active' LIMIT 1",
        )
        .bind(domain)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ProductCatalogError::Storage)?;

        if let (Some(cache), Some(product_id)) = (&self.cache, product_id)
            && cache
                .put(
                    &cache_key,
                    product_id.to_string().as_bytes(),
                    Duration::from_secs(10),
                )
                .await
                .is_err()
        {
            tracing::warn!(
                event = "cache.redis.write_failed",
                component = "platform",
                operation = "product_domain.resolve",
                result = "fallback"
            );
        }

        match product_id {
            Some(id) => self.get_active_product(id).await,
            None => Ok(None),
        }
    }

    pub async fn get_published_configuration(
        &self,
        product_id: Uuid,
        version: i32,
    ) -> Result<Option<ProductConfiguration>, ProductCatalogError> {
        let configuration = sqlx::query_as::<_, (String, Json<Value>, Option<i64>)>(
            "SELECT c.schema_version, c.configuration, extract(epoch FROM c.published_at)::bigint FROM platform_product_catalog.product_configurations c JOIN platform_product_catalog.products p ON p.id = c.product_id AND p.status = 'active' WHERE c.product_id = $1 AND c.version = $2 AND c.status = 'published'",
        )
        .bind(product_id)
        .bind(version)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ProductCatalogError::Storage)?;
        let Some((schema_version, configuration, published_at)) = configuration else {
            return Ok(None);
        };
        validate_product_configuration(&schema_version, &configuration.0)?;
        Ok(Some(ProductConfiguration {
            product_id,
            version,
            schema_version,
            canonical_json: configuration.0,
            published_at: published_at
                .map(|epoch| UNIX_EPOCH + Duration::from_secs(epoch.max(0) as u64)),
        }))
    }
}

impl ProductCatalogPort for ProductCatalog {
    fn get_active_product<'a>(
        &'a self,
        product_id: Uuid,
    ) -> ContractFuture<'a, Result<ensambria_platform_product_catalog_contracts::ProductCatalogEntry, ProductCatalogFailure>> {
        Box::pin(async move {
            let product = ProductCatalog::get_active_product(self, product_id)
                .await
                .map_err(map_contract_error)?
                .ok_or(ProductCatalogFailure::NotFound)?;
            to_contract_entry(product)
        })
    }

    fn resolve_active_domain<'a>(
        &'a self,
        domain_name: &'a str,
    ) -> ContractFuture<'a, Result<Option<ensambria_platform_product_catalog_contracts::ProductCatalogEntry>, ProductCatalogFailure>> {
        Box::pin(async move {
            ProductCatalog::resolve_active_domain(self, domain_name)
                .await
                .map_err(map_contract_error)?
                .map(to_contract_entry)
                .transpose()
        })
    }

    fn get_published_configuration<'a>(
        &'a self,
        product_id: Uuid,
        version: u32,
    ) -> ContractFuture<'a, Result<Option<ensambria_platform_product_catalog_contracts::ProductConfiguration>, ProductCatalogFailure>> {
        Box::pin(async move {
            let version = i32::try_from(version).map_err(|_| ProductCatalogFailure::InvalidRequest)?;
            ProductCatalog::get_published_configuration(self, product_id, version)
                .await
                .map_err(map_contract_error)?
                .map(|configuration| {
                    Ok(ensambria_platform_product_catalog_contracts::ProductConfiguration {
                        product_id: configuration.product_id,
                        version: u32::try_from(configuration.version)
                            .map_err(|_| ProductCatalogFailure::InvalidRequest)?,
                        schema_version: configuration.schema_version,
                        canonical_json: configuration.canonical_json,
                        published_at_unix: configuration.published_at.map(system_time_to_unix),
                    })
                })
                .transpose()
        })
    }
}

fn map_contract_error(error: ProductCatalogError) -> ProductCatalogFailure {
    match error {
        ProductCatalogError::Storage => ProductCatalogFailure::Unavailable,
        ProductCatalogError::InvalidDomain
        | ProductCatalogError::InvalidConfiguration
        | ProductCatalogError::UnsupportedConfigurationSchema => {
            ProductCatalogFailure::InvalidRequest
        }
    }
}

fn system_time_to_unix(value: SystemTime) -> i64 {
    let seconds = value.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    i64::try_from(seconds).unwrap_or(i64::MAX)
}

fn to_contract_entry(
    product: ProductCatalogEntry,
) -> Result<ensambria_platform_product_catalog_contracts::ProductCatalogEntry, ProductCatalogFailure> {
    Ok(ensambria_platform_product_catalog_contracts::ProductCatalogEntry {
        product_id: product.product_id,
        product_key: product.product_key,
        display_name: product.display_name,
        brand_name: product.brand_name,
        configuration_version: u32::try_from(product.configuration_version)
            .map_err(|_| ProductCatalogFailure::InvalidRequest)?,
        configuration_schema_version: product.configuration_schema_version,
        configuration: product.configuration,
        domains: product.domains.into_iter().map(|domain| {
            ensambria_platform_product_catalog_contracts::ProductDomain {
                domain_name: domain.domain_name,
                status: domain.status,
            }
        }).collect(),
        markets: product.markets.into_iter().map(|market| {
            ensambria_platform_product_catalog_contracts::ProductMarket {
                country_code: market.country_code,
                currency_code: market.currency_code,
                locale: market.locale,
                status: market.status,
            }
        }).collect(),
        surfaces: product.surfaces.into_iter().map(|surface| {
            Ok(ensambria_platform_product_catalog_contracts::ProductSurface {
                surface_code: surface.surface_code,
                configuration_version: u32::try_from(surface.configuration_version)
                    .map_err(|_| ProductCatalogFailure::InvalidRequest)?,
                status: surface.status,
            })
        }).collect::<Result<Vec<_>, ProductCatalogFailure>>()?,
        capabilities: product.capabilities.into_iter().map(|capability| {
            ensambria_platform_product_catalog_contracts::ProductCapability {
                capability_code: capability.capability_code,
                contract_version: capability.contract_version,
                implementation_status: capability.implementation_status,
                product_status: capability.product_status,
            }
        }).collect(),
        published_at_unix: system_time_to_unix(product.published_at),
    })
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct ProductConfigurationV1 {
    branding: Option<ProductBrandingV1>,
    domains: Option<Vec<String>>,
    markets: Option<Vec<ProductMarketV1>>,
    surfaces: Option<Vec<ProductSurfaceV1>>,
    capabilities: Option<Vec<ProductCapabilityV1>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct ProductBrandingV1 {
    display_name: Option<String>,
    brand_name: Option<String>,
    logo_uri: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct ProductMarketV1 {
    country_code: String,
    currency_code: String,
    locale: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct ProductSurfaceV1 {
    code: String,
    configuration_version: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct ProductCapabilityV1 {
    code: String,
    contract_version: String,
}

fn validate_product_configuration(
    schema_version: &str,
    value: &Value,
) -> Result<(), ProductCatalogError> {
    if schema_version != "product-config.v1" {
        return Err(ProductCatalogError::UnsupportedConfigurationSchema);
    }
    if !value.is_object()
        || ["branding", "domains", "markets", "surfaces", "capabilities"]
            .iter()
            .any(|field| value.get(*field).is_some_and(Value::is_null))
    {
        return Err(ProductCatalogError::InvalidConfiguration);
    }
    if let Some(branding) = value.get("branding")
        && (!branding.is_object()
            || ["displayName", "brandName", "logoUri"]
                .iter()
                .any(|field| branding.get(*field).is_some_and(Value::is_null)))
    {
        return Err(ProductCatalogError::InvalidConfiguration);
    }
    let configuration: ProductConfigurationV1 = serde_json::from_value(value.clone())
        .map_err(|_| ProductCatalogError::InvalidConfiguration)?;

    if let Some(branding) = configuration.branding {
        for name in [branding.display_name, branding.brand_name]
            .into_iter()
            .flatten()
        {
            let length = name.chars().count();
            if !(1..=160).contains(&length) {
                return Err(ProductCatalogError::InvalidConfiguration);
            }
        }
        if let Some(logo_uri) = branding.logo_uri {
            let length = logo_uri.chars().count();
            let uri = http::Uri::try_from(logo_uri.as_str())
                .map_err(|_| ProductCatalogError::InvalidConfiguration)?;
            if length > 2048 || uri.scheme().is_none() || uri.authority().is_none() {
                return Err(ProductCatalogError::InvalidConfiguration);
            }
        }
    }

    if let Some(domains) = configuration.domains {
        let mut normalized_domains = HashSet::with_capacity(domains.len());
        for domain in domains {
            let normalized =
                normalize_domain(&domain).map_err(|_| ProductCatalogError::InvalidConfiguration)?;
            if normalized != domain || !normalized_domains.insert(normalized) {
                return Err(ProductCatalogError::InvalidConfiguration);
            }
        }
    }

    if let Some(markets) = configuration.markets {
        let mut unique_markets = HashSet::with_capacity(markets.len());
        for market in markets {
            let locale_length = market.locale.chars().count();
            if market.country_code.len() != 2
                || !market
                    .country_code
                    .bytes()
                    .all(|byte| byte.is_ascii_uppercase())
                || market.currency_code.len() != 3
                || !market
                    .currency_code
                    .bytes()
                    .all(|byte| byte.is_ascii_uppercase())
                || !(2..=35).contains(&locale_length)
                || !unique_markets.insert((
                    market.country_code,
                    market.currency_code,
                    market.locale,
                ))
            {
                return Err(ProductCatalogError::InvalidConfiguration);
            }
        }
    }

    if let Some(surfaces) = configuration.surfaces {
        let mut unique_surfaces = HashSet::with_capacity(surfaces.len());
        for surface in surfaces {
            if !valid_contract_code(&surface.code, 64)
                || surface.configuration_version == 0
                || !unique_surfaces.insert((surface.code, surface.configuration_version))
            {
                return Err(ProductCatalogError::InvalidConfiguration);
            }
        }
    }

    if let Some(capabilities) = configuration.capabilities {
        let mut unique_capabilities = HashSet::with_capacity(capabilities.len());
        for capability in capabilities {
            let version_length = capability.contract_version.chars().count();
            if !valid_contract_code(&capability.code, 120)
                || !(1..=64).contains(&version_length)
                || !unique_capabilities.insert((capability.code, capability.contract_version))
            {
                return Err(ProductCatalogError::InvalidConfiguration);
            }
        }
    }

    Ok(())
}

fn valid_contract_code(value: &str, maximum_length: usize) -> bool {
    let mut bytes = value.bytes();
    matches!(bytes.next(), Some(first) if first.is_ascii_lowercase())
        && (2..=maximum_length).contains(&value.len())
        && bytes.all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'_' | b'.' | b'-')
        })
}

fn normalize_domain(domain: &str) -> Result<String, ProductCatalogError> {
    let value = domain.trim().trim_end_matches('.').to_ascii_lowercase();
    if value.len() < 4
        || value.len() > 253
        || !value.is_ascii()
        || value.bytes().any(|byte| {
            !(byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'.' || byte == b'-')
        })
        || !value.contains('.')
        || value.split('.').any(|label| {
            label.is_empty() || label.len() > 63 || label.starts_with('-') || label.ends_with('-')
        })
    {
        return Err(ProductCatalogError::InvalidDomain);
    }
    Ok(value)
}

#[derive(Debug, Clone)]
pub struct ProductCatalogEntry {
    pub product_id: Uuid,
    pub product_key: String,
    pub display_name: String,
    pub brand_name: String,
    pub configuration_version: i32,
    pub configuration_schema_version: String,
    pub configuration: Value,
    pub published_at: SystemTime,
    pub domains: Vec<ProductDomain>,
    pub markets: Vec<ProductMarket>,
    pub surfaces: Vec<ProductSurface>,
    pub capabilities: Vec<ProductCapability>,
}

#[derive(Debug, Clone)]
pub struct ProductConfiguration {
    pub product_id: Uuid,
    pub version: i32,
    pub schema_version: String,
    pub canonical_json: Value,
    pub published_at: Option<std::time::SystemTime>,
}

#[derive(Debug, Clone)]
pub struct ProductDomain {
    pub domain_name: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct ProductMarket {
    pub country_code: String,
    pub currency_code: String,
    pub locale: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct ProductSurface {
    pub surface_code: String,
    pub configuration_version: i32,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct ProductCapability {
    pub capability_code: String,
    pub contract_version: String,
    pub implementation_status: String,
    pub product_status: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ProductCatalogError {
    InvalidDomain,
    InvalidConfiguration,
    UnsupportedConfigurationSchema,
    Storage,
}

impl std::fmt::Display for ProductCatalogError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::InvalidDomain => "product domain is invalid",
            Self::InvalidConfiguration => "published product configuration is invalid",
            Self::UnsupportedConfigurationSchema => {
                "product configuration schema version is unsupported"
            }
            Self::Storage => "PLATFORM product catalogue query failed",
        })
    }
}

impl std::error::Error for ProductCatalogError {}

#[cfg(test)]
mod tests {
    use super::{ProductCatalogError, normalize_domain, validate_product_configuration};

    #[test]
    fn normalizes_ascii_domains_without_accepting_urls_or_paths() {
        assert_eq!(
            normalize_domain("  SHOP.Example.COM. ").unwrap(),
            "shop.example.com"
        );
        assert_eq!(
            normalize_domain("https://shop.example.com").unwrap_err(),
            ProductCatalogError::InvalidDomain
        );
        assert_eq!(
            normalize_domain("shop.example.com/path").unwrap_err(),
            ProductCatalogError::InvalidDomain
        );
    }

    #[test]
    fn validates_the_versioned_product_configuration_contract() {
        let valid = serde_json::json!({
            "branding": {
                "displayName": "ENSAMBRIA Core",
                "brandName": "ENSAMBRIA"
            },
            "domains": ["shop.example.test"],
            "markets": [{
                "countryCode": "CO",
                "currencyCode": "COP",
                "locale": "es-CO"
            }],
            "surfaces": [{
                "code": "operator.web",
                "configurationVersion": 1
            }],
            "capabilities": [{
                "code": "identity.auth",
                "contractVersion": "v1"
            }]
        });
        assert_eq!(
            validate_product_configuration("product-config.v1", &valid),
            Ok(())
        );
        assert_eq!(
            validate_product_configuration("product-config.v2", &valid),
            Err(ProductCatalogError::UnsupportedConfigurationSchema)
        );
    }

    #[test]
    fn rejects_noncanonical_or_duplicate_product_configuration_values() {
        let uppercase_domain = serde_json::json!({"domains": ["SHOP.EXAMPLE.TEST"]});
        assert_eq!(
            validate_product_configuration("product-config.v1", &uppercase_domain),
            Err(ProductCatalogError::InvalidConfiguration)
        );
        let duplicate_surface = serde_json::json!({
            "surfaces": [
                {"code": "operator.web", "configurationVersion": 1},
                {"code": "operator.web", "configurationVersion": 1}
            ]
        });
        assert_eq!(
            validate_product_configuration("product-config.v1", &duplicate_surface),
            Err(ProductCatalogError::InvalidConfiguration)
        );
        let unknown_property = serde_json::json!({"unreviewedCapability": true});
        assert_eq!(
            validate_product_configuration("product-config.v1", &unknown_property),
            Err(ProductCatalogError::InvalidConfiguration)
        );
    }
}
