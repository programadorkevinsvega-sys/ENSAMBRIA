use ensambria_platform::product_catalog::{ProductCatalog, ProductCatalogEntry};
use uuid::Uuid;

#[derive(Clone)]
pub struct PlatformProductPolicy {
    catalog: ProductCatalog,
}

impl PlatformProductPolicy {
    pub fn new(catalog: ProductCatalog) -> Self {
        Self { catalog }
    }

    /// Resolves an active PLATFORM product and enforces its published capability contract.
    /// CORE stores product IDs as opaque references and never joins PLATFORM tables directly.
    pub async fn require_capability(
        &self,
        product_id: Uuid,
        capability_code: &str,
        contract_version: &str,
        configuration_version: i32,
    ) -> Result<ProductCatalogEntry, PlatformContractError> {
        let product = self.catalog.get_active_product(product_id).await
            .map_err(|_| PlatformContractError::PlatformUnavailable)?
            .ok_or(PlatformContractError::ProductUnavailable)?;

        if product.configuration_version != configuration_version {
            return Err(PlatformContractError::ConfigurationVersionMismatch);
        }

        let capability = product.capabilities.iter().find(|capability| {
            capability.capability_code == capability_code
        }).ok_or(PlatformContractError::CapabilityUnavailable)?;

        if capability.contract_version != contract_version {
            return Err(PlatformContractError::ContractVersionMismatch);
        }
        Ok(product)
    }

    /// Resolves a verified active domain using PLATFORM's canonical domain resolver.
    pub async fn resolve_domain(
        &self,
        domain: &str,
    ) -> Result<Option<ProductCatalogEntry>, PlatformContractError> {
        self.catalog.resolve_active_domain(domain).await
            .map_err(|_| PlatformContractError::PlatformUnavailable)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PlatformContractError {
    ProductUnavailable,
    CapabilityUnavailable,
    ConfigurationVersionMismatch,
    ContractVersionMismatch,
    PlatformUnavailable,
}

impl std::fmt::Display for PlatformContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::ProductUnavailable => "PLATFORM product is not active or published",
            Self::CapabilityUnavailable => "PLATFORM product capability is not available",
            Self::ConfigurationVersionMismatch => "PLATFORM product configuration version does not match CORE association",
            Self::ContractVersionMismatch => "PLATFORM capability contract version does not match CORE requirement",
            Self::PlatformUnavailable => "PLATFORM product contract could not be read",
        })
    }
}
impl std::error::Error for PlatformContractError {}
