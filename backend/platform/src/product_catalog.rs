use serde_json::Value;
use sqlx::{PgPool, types::Json};
use std::time::SystemTime;

use uuid::Uuid;

#[derive(Clone)]
pub struct ProductCatalog {
    pool: PgPool,
}

impl ProductCatalog {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_active_product(
        &self,
        product_id: Uuid,
    ) -> Result<Option<ProductCatalogEntry>, ProductCatalogError> {
        let product = sqlx::query_as::<_, (Uuid, String, String, String, i32, String, Json<Value>, SystemTime)>(
            "SELECT product_id, product_key, display_name, brand_name, configuration_version, schema_version, configuration, published_at FROM platform.active_product_catalog WHERE product_id = $1",
        )
        .bind(product_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ProductCatalogError::Storage)?;

        let Some((id, key, display_name, brand_name, version, schema_version, configuration, published_at)) =
            product
        else {
            return Ok(None);
        };

        let domains = sqlx::query_as::<_, (String, String)>(
            "SELECT domain_name, status FROM platform.product_domains WHERE product_id = $1 AND status = 'active' ORDER BY domain_name",
        )
        .bind(product_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|_| ProductCatalogError::Storage)?
        .into_iter()
        .map(|(domain_name, status)| ProductDomain { domain_name, status })
        .collect();

        let markets = sqlx::query_as::<_, (String, String, String, String)>(
            "SELECT country_code::text, currency_code::text, locale, status FROM platform.product_markets WHERE product_id = $1 AND status = 'enabled' ORDER BY country_code, locale",
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
            "SELECT surface_code, configuration_version, status FROM platform.product_surfaces WHERE product_id = $1 AND status = 'enabled' ORDER BY surface_code",
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
            "SELECT pc.capability_code, cc.contract_version, cc.implementation_status, pc.status FROM platform.product_capabilities pc JOIN platform.capability_catalog cc USING (capability_code) WHERE pc.product_id = $1 AND pc.status = 'enabled' AND cc.implementation_status = 'available' ORDER BY pc.capability_code",
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

        Ok(Some(ProductCatalogEntry {
            product_id: id,
            product_key: key,
            display_name,
            brand_name,
            configuration_version: version,
            configuration_schema_version: schema_version,
            configuration: configuration.0,
            published_at,
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
        let product_id: Option<Uuid> = sqlx::query_scalar(
            "SELECT p.product_id FROM platform.active_product_catalog p JOIN platform.product_domains d ON d.product_id = p.product_id WHERE d.domain_name = $1 AND d.status = 'active' LIMIT 1",
        )
        .bind(domain)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ProductCatalogError::Storage)?;
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
        let configuration = sqlx::query_as::<_, (String, Json<Value>, Option<std::time::SystemTime>)>(
            "SELECT schema_version, configuration, published_at FROM platform.product_configurations WHERE product_id = $1 AND version = $2 AND status = 'published'",
        )
        .bind(product_id)
        .bind(version)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ProductCatalogError::Storage)?;
        Ok(configuration.map(|(schema_version, configuration, published_at)| {
            ProductConfiguration {
                product_id,
                version,
                schema_version,
                canonical_json: configuration.0,
                published_at,
            }
        }))
    }
}

fn normalize_domain(domain: &str) -> Result<String, ProductCatalogError> {
    let value = domain.trim().trim_end_matches('.').to_ascii_lowercase();
    if value.len() < 4
        || value.len() > 253
        || !value.is_ascii()
        || value.bytes().any(|byte| {
            !(byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'.'
                || byte == b'-')
        })
        || !value.contains('.')
        || value.split('.').any(|label| {
            label.is_empty()
                || label.len() > 63
                || label.starts_with('-')
                || label.ends_with('-')
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
    Storage,
}

impl std::fmt::Display for ProductCatalogError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::InvalidDomain => "product domain is invalid",
            Self::Storage => "PLATFORM product catalogue query failed",
        })
    }
}

impl std::error::Error for ProductCatalogError {}

#[cfg(test)]
mod tests {
    use super::{ProductCatalogError, normalize_domain};

    #[test]
    fn normalizes_ascii_domains_without_accepting_urls_or_paths() {
        assert_eq!(normalize_domain("  SHOP.Example.COM. ").unwrap(), "shop.example.com");
        assert_eq!(normalize_domain("https://shop.example.com").unwrap_err(), ProductCatalogError::InvalidDomain);
        assert_eq!(normalize_domain("shop.example.com/path").unwrap_err(), ProductCatalogError::InvalidDomain);
    }
}
