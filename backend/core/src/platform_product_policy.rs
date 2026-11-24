use std::sync::Arc;

use ensambria_platform_product_catalog_contracts::{
    ProductCatalog as ProductCatalogPort, ProductCatalogEntry, ProductCatalogFailure,
};
use uuid::Uuid;

use crate::platform_product_catalog_grpc::GrpcPlatformProductCatalog;

#[derive(Clone)]
pub struct PlatformProductPolicy {
    catalog: Arc<dyn ProductCatalogPort>,
}

impl PlatformProductPolicy {
    pub fn new(catalog: Arc<dyn ProductCatalogPort>) -> Self {
        Self { catalog }
    }

    /// Builds the transport adapter from deployment-provided mutual-TLS configuration.
    pub async fn from_env() -> Result<Self, PlatformContractError> {
        let catalog =
            GrpcPlatformProductCatalog::from_env()
                .await
                .map_err(|error| match error {
                    ProductCatalogFailure::InvalidRequest => {
                        PlatformContractError::InvalidConfiguration
                    }
                    ProductCatalogFailure::NotFound => PlatformContractError::ProductUnavailable,
                    ProductCatalogFailure::Unavailable => {
                        PlatformContractError::PlatformUnavailable
                    }
                })?;
        Ok(Self::new(Arc::new(catalog)))
    }

    /// Enforces a published PLATFORM capability through the application contract.
    /// A catalog response is context, never authorization.
    pub async fn require_capability(
        &self,
        product_id: Uuid,
        capability_code: &str,
        contract_version: &str,
        configuration_version: i32,
    ) -> Result<ProductCatalogEntry, PlatformContractError> {
        let product = self
            .catalog
            .get_active_product(product_id)
            .await
            .map_err(map_catalog_failure)?;
        let expected_version = u32::try_from(configuration_version)
            .map_err(|_| PlatformContractError::InvalidPlatformRequest)?;
        if product.configuration_version != expected_version {
            return Err(PlatformContractError::ConfigurationVersionMismatch);
        }

        let capability = product
            .capabilities
            .iter()
            .find(|capability| capability.capability_code == capability_code)
            .ok_or(PlatformContractError::CapabilityUnavailable)?;

        if capability.contract_version != contract_version
            || capability.implementation_status != "available"
            || capability.product_status != "enabled"
        {
            return Err(PlatformContractError::ContractVersionMismatch);
        }
        Ok(product)
    }

    pub async fn resolve_domain(
        &self,
        domain: &str,
    ) -> Result<Option<ProductCatalogEntry>, PlatformContractError> {
        self.catalog
            .resolve_active_domain(domain)
            .await
            .map_err(map_catalog_failure)
    }
}

fn map_catalog_failure(error: ProductCatalogFailure) -> PlatformContractError {
    match error {
        ProductCatalogFailure::InvalidRequest => PlatformContractError::InvalidPlatformRequest,
        ProductCatalogFailure::NotFound => PlatformContractError::ProductUnavailable,
        ProductCatalogFailure::Unavailable => PlatformContractError::PlatformUnavailable,
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PlatformContractError {
    ProductUnavailable,
    CapabilityUnavailable,
    ConfigurationVersionMismatch,
    ContractVersionMismatch,
    InvalidConfiguration,
    InvalidPlatformRequest,
    InvalidPlatformResponse,
    UnsupportedPlatformContract,
    PlatformUnavailable,
}

impl std::fmt::Display for PlatformContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::ProductUnavailable => "PLATFORM product is not active or published",
            Self::CapabilityUnavailable => "PLATFORM product capability is not available",
            Self::ConfigurationVersionMismatch => {
                "PLATFORM product configuration version does not match CORE association"
            }
            Self::ContractVersionMismatch => {
                "PLATFORM capability contract version does not match CORE requirement"
            }
            Self::InvalidConfiguration => "PLATFORM ProductCatalog endpoint is not configured",
            Self::InvalidPlatformRequest => "PLATFORM rejected the ProductCatalog request",
            Self::InvalidPlatformResponse => "PLATFORM returned an invalid product contract",
            Self::UnsupportedPlatformContract => {
                "PLATFORM does not support the requested product contract"
            }
            Self::PlatformUnavailable => "PLATFORM ProductCatalog service is unavailable",
        })
    }
}
impl std::error::Error for PlatformContractError {}
