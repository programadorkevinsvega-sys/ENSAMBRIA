use ensambria_platform_contract::platform::product_catalog::v1::{
    GetActiveProductRequest, GetPublishedConfigurationRequest, ProductCatalogClient,
    ProductCatalogEntry as ProtoEntry, ResolveActiveDomainRequest,
};
use ensambria_platform_product_catalog_contracts::{
    ContractFuture, ProductCapability, ProductCatalog as ProductCatalogPort, ProductCatalogEntry,
    ProductCatalogFailure, ProductConfiguration, ProductDomain, ProductMarket, ProductSurface,
};
use tonic::{
    Code,
    transport::{Certificate, Channel, ClientTlsConfig, Identity},
};
use uuid::Uuid;

#[derive(Clone)]
pub struct GrpcPlatformProductCatalog {
    client: ProductCatalogClient<Channel>,
}

impl GrpcPlatformProductCatalog {
    pub fn new(client: ProductCatalogClient<Channel>) -> Self {
        Self { client }
    }

    pub async fn from_env() -> Result<Self, ProductCatalogFailure> {
        let endpoint = std::env::var("PLATFORM_PRODUCT_CATALOG_ENDPOINT")
            .map_err(|_| ProductCatalogFailure::InvalidRequest)?;
        if !endpoint.starts_with("https://") {
            return Err(ProductCatalogFailure::InvalidRequest);
        }
        let ca_certificate = std::fs::read(
            std::env::var("PLATFORM_PRODUCT_CATALOG_CA_CERT_FILE")
                .map_err(|_| ProductCatalogFailure::InvalidRequest)?,
        )
        .map_err(|_| ProductCatalogFailure::InvalidRequest)?;
        let client_certificate = std::fs::read(
            std::env::var("CORE_PLATFORM_GRPC_CLIENT_CERT_FILE")
                .map_err(|_| ProductCatalogFailure::InvalidRequest)?,
        )
        .map_err(|_| ProductCatalogFailure::InvalidRequest)?;
        let client_key = std::fs::read(
            std::env::var("CORE_PLATFORM_GRPC_CLIENT_KEY_FILE")
                .map_err(|_| ProductCatalogFailure::InvalidRequest)?,
        )
        .map_err(|_| ProductCatalogFailure::InvalidRequest)?;
        let tls = ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(ca_certificate))
            .identity(Identity::from_pem(client_certificate, client_key));
        let client = ProductCatalogClient::connect_with_tls(endpoint, tls)
            .await
            .map_err(|_| ProductCatalogFailure::Unavailable)?;
        Ok(Self { client })
    }
}

impl ProductCatalogPort for GrpcPlatformProductCatalog {
    fn get_active_product<'a>(
        &'a self,
        product_id: Uuid,
    ) -> ContractFuture<'a, Result<ProductCatalogEntry, ProductCatalogFailure>> {
        let mut client = self.client.clone();
        Box::pin(async move {
            let response = client
                .get_active_product(GetActiveProductRequest {
                    product_id: product_id.to_string(),
                })
                .await
                .map_err(map_status)?
                .into_inner();
            to_domain_entry(response)
        })
    }

    fn resolve_active_domain<'a>(
        &'a self,
        domain_name: &'a str,
    ) -> ContractFuture<'a, Result<Option<ProductCatalogEntry>, ProductCatalogFailure>> {
        let mut client = self.client.clone();
        Box::pin(async move {
            match client
                .resolve_active_domain(ResolveActiveDomainRequest {
                    domain_name: domain_name.to_owned(),
                })
                .await
            {
                Ok(response) => to_domain_entry(response.into_inner()).map(Some),
                Err(status) if status.code() == Code::NotFound => Ok(None),
                Err(status) => Err(map_status(status)),
            }
        })
    }

    fn get_published_configuration<'a>(
        &'a self,
        product_id: Uuid,
        version: u32,
    ) -> ContractFuture<'a, Result<Option<ProductConfiguration>, ProductCatalogFailure>> {
        let mut client = self.client.clone();
        Box::pin(async move {
            let response = match client
                .get_published_configuration(GetPublishedConfigurationRequest {
                    product_id: product_id.to_string(),
                    configuration_version: version,
                })
                .await
            {
                Ok(response) => response.into_inner(),
                Err(status) if status.code() == Code::NotFound => return Ok(None),
                Err(status) => return Err(map_status(status)),
            };
            let returned_product_id = parse_product_id(&response.product_id)?;
            let canonical_json = serde_json::from_slice(&response.canonical_json)
                .map_err(|_| ProductCatalogFailure::InvalidRequest)?;
            Ok(Some(ProductConfiguration {
                product_id: returned_product_id,
                version: response.version,
                schema_version: response.schema_version,
                canonical_json,
                published_at_unix: response.published_at.map(|value| value.seconds),
            }))
        })
    }
}

fn to_domain_entry(value: ProtoEntry) -> Result<ProductCatalogEntry, ProductCatalogFailure> {
    Ok(ProductCatalogEntry {
        product_id: parse_product_id(&value.product_id)?,
        product_key: value.product_key,
        display_name: value.display_name,
        brand_name: value.brand_name,
        configuration_version: value.configuration_version,
        configuration_schema_version: value.configuration_schema_version,
        domains: value
            .domains
            .into_iter()
            .map(|domain| ProductDomain {
                domain_name: domain.domain_name,
                status: domain.status,
            })
            .collect(),
        markets: value
            .markets
            .into_iter()
            .map(|market| ProductMarket {
                country_code: market.country_code,
                currency_code: market.currency_code,
                locale: market.locale,
                status: market.status,
            })
            .collect(),
        surfaces: value
            .surfaces
            .into_iter()
            .map(|surface| ProductSurface {
                surface_code: surface.surface_code,
                configuration_version: surface.configuration_version,
                status: surface.status,
            })
            .collect(),
        capabilities: value
            .capabilities
            .into_iter()
            .map(|capability| ProductCapability {
                capability_code: capability.capability_code,
                contract_version: capability.contract_version,
                implementation_status: capability.implementation_status,
                product_status: capability.product_status,
            })
            .collect(),
        published_at_unix: value
            .published_at
            .map(|timestamp| timestamp.seconds)
            .unwrap_or_default(),
    })
}

fn parse_product_id(value: &str) -> Result<Uuid, ProductCatalogFailure> {
    let product_id = Uuid::parse_str(value).map_err(|_| ProductCatalogFailure::InvalidRequest)?;
    if product_id.to_string() != value {
        return Err(ProductCatalogFailure::InvalidRequest);
    }
    Ok(product_id)
}

fn map_status(status: tonic::Status) -> ProductCatalogFailure {
    match status.code() {
        Code::NotFound => ProductCatalogFailure::NotFound,
        Code::InvalidArgument | Code::FailedPrecondition => ProductCatalogFailure::InvalidRequest,
        _ => ProductCatalogFailure::Unavailable,
    }
}
