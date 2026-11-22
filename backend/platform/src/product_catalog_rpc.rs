use std::time::{SystemTime, UNIX_EPOCH};

use ensambria_platform_contract::{
    google::protobuf::Timestamp,
    platform::product_catalog::v1::{
        BoxFuture, GetActiveProductRequest, GetPublishedConfigurationRequest,
        ProductCapability as ContractCapability, ProductCatalog as ProductCatalogService,
        ProductCatalogEntry as ContractEntry, ProductConfiguration as ContractConfiguration,
        ProductDomain as ContractDomain, ProductMarket as ContractMarket,
        ProductSurface as ContractSurface, ResolveActiveDomainRequest,
    },
};
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::product_catalog::{ProductCatalog, ProductCatalogError};

#[derive(Clone)]
pub struct ProductCatalogRpc {
    catalog: ProductCatalog,
}

impl ProductCatalogRpc {
    pub fn new(catalog: ProductCatalog) -> Self {
        Self { catalog }
    }
}

impl ProductCatalogService for ProductCatalogRpc {
    fn get_active_product(
        &self,
        request: Request<GetActiveProductRequest>,
    ) -> BoxFuture<Result<Response<ContractEntry>, Status>> {
        let catalog = self.catalog.clone();
        Box::pin(async move {
            let product_id = parse_product_id(&request.into_inner().product_id)?;
            let product = catalog
                .get_active_product(product_id)
                .await
                .map_err(map_catalog_error)?
                .ok_or_else(|| Status::not_found("published PLATFORM product was not found"))?;
            Ok(Response::new(to_contract_entry(product)?))
        })
    }

    fn resolve_active_domain(
        &self,
        request: Request<ResolveActiveDomainRequest>,
    ) -> BoxFuture<Result<Response<ContractEntry>, Status>> {
        let catalog = self.catalog.clone();
        Box::pin(async move {
            let product = catalog
                .resolve_active_domain(&request.into_inner().domain_name)
                .await
                .map_err(map_catalog_error)?
                .ok_or_else(|| Status::not_found("active PLATFORM domain was not found"))?;
            Ok(Response::new(to_contract_entry(product)?))
        })
    }

    fn get_published_configuration(
        &self,
        request: Request<GetPublishedConfigurationRequest>,
    ) -> BoxFuture<Result<Response<ContractConfiguration>, Status>> {
        let catalog = self.catalog.clone();
        Box::pin(async move {
            let request = request.into_inner();
            let product_id = parse_product_id(&request.product_id)?;
            let version = i32::try_from(request.configuration_version)
                .map_err(|_| Status::invalid_argument("configuration version is invalid"))?;
            let configuration = catalog
                .get_published_configuration(product_id, version)
                .await
                .map_err(map_catalog_error)?
                .ok_or_else(|| Status::not_found("published PLATFORM configuration was not found"))?;
            let canonical_json = serde_json::to_vec(&configuration.canonical_json)
                .map_err(|_| Status::internal("published configuration serialization failed"))?;
            Ok(Response::new(ContractConfiguration {
                product_id: configuration.product_id.to_string(),
                version: u32::try_from(configuration.version)
                    .map_err(|_| Status::internal("published configuration version is invalid"))?,
                schema_version: configuration.schema_version,
                canonical_json,
                published_at: configuration.published_at.map(to_timestamp),
            }))
        })
    }
}

fn parse_product_id(value: &str) -> Result<Uuid, Status> {
    Uuid::parse_str(value)
        .map_err(|_| Status::invalid_argument("product_id must be a canonical UUID"))
}

fn map_catalog_error(error: ProductCatalogError) -> Status {
    match error {
        ProductCatalogError::InvalidDomain | ProductCatalogError::InvalidConfiguration => {
            Status::invalid_argument("PLATFORM product catalog request is invalid")
        }
        ProductCatalogError::UnsupportedConfigurationSchema => {
            Status::failed_precondition("PLATFORM configuration schema is unsupported")
        }
        ProductCatalogError::Storage => {
            Status::unavailable("PLATFORM product catalog storage is unavailable")
        }
    }
}

fn to_contract_entry(
    product: crate::product_catalog::ProductCatalogEntry,
) -> Result<ContractEntry, Status> {
    Ok(ContractEntry {
        product_id: product.product_id.to_string(),
        product_key: product.product_key,
        display_name: product.display_name,
        brand_name: product.brand_name,
        configuration_version: u32::try_from(product.configuration_version)
            .map_err(|_| Status::internal("PLATFORM configuration version is invalid"))?,
        configuration_schema_version: product.configuration_schema_version,
        domains: product
            .domains
            .into_iter()
            .map(|domain| ContractDomain {
                domain_name: domain.domain_name,
                status: domain.status,
            })
            .collect(),
        markets: product
            .markets
            .into_iter()
            .map(|market| ContractMarket {
                country_code: market.country_code,
                currency_code: market.currency_code,
                locale: market.locale,
                status: market.status,
            })
            .collect(),
        surfaces: product
            .surfaces
            .into_iter()
            .map(|surface| {
                Ok(ContractSurface {
                    surface_code: surface.surface_code,
                    configuration_version: u32::try_from(surface.configuration_version)
                        .map_err(|_| Status::internal("PLATFORM surface version is invalid"))?,
                    status: surface.status,
                })
            })
            .collect::<Result<Vec<_>, Status>>()?,
        capabilities: product
            .capabilities
            .into_iter()
            .map(|capability| ContractCapability {
                capability_code: capability.capability_code,
                contract_version: capability.contract_version,
                implementation_status: capability.implementation_status,
                product_status: capability.product_status,
            })
            .collect(),
        published_at: Some(to_timestamp(product.published_at)),
    })
}

fn to_timestamp(value: SystemTime) -> Timestamp {
    let duration = value.duration_since(UNIX_EPOCH).unwrap_or_default();
    Timestamp {
        seconds: i64::try_from(duration.as_secs()).unwrap_or(i64::MAX),
        nanos: duration.subsec_nanos() as i32,
    }
}
