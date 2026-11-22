use ensambria_platform_contract::platform::product_catalog::v1::{
    GetActiveProductRequest, ProductCatalogClient, ProductCatalogEntry,
    ResolveActiveDomainRequest,
};
use tonic::{Code, transport::Channel};
use uuid::Uuid;

#[derive(Clone)]
pub struct PlatformProductPolicy {
    catalog: ProductCatalogClient<Channel>,
}

impl PlatformProductPolicy {
    pub fn new(catalog: ProductCatalogClient<Channel>) -> Self {
        Self { catalog }
    }

    /// Connects to the PLATFORM-owned gRPC contract; CORE never receives a PLATFORM database pool.
    pub async fn from_env() -> Result<Self, PlatformContractError> {
        let endpoint = std::env::var("PLATFORM_PRODUCT_CATALOG_ENDPOINT")
            .map_err(|_| PlatformContractError::InvalidConfiguration)?;
        let catalog = ProductCatalogClient::connect(endpoint)
            .await
            .map_err(|_| PlatformContractError::PlatformUnavailable)?;
        Ok(Self { catalog })
    }

    /// Resolves an active PLATFORM product and enforces its published capability contract.
    /// A catalog response is context, never authorization.
    pub async fn require_capability(
        &self,
        product_id: Uuid,
        capability_code: &str,
        contract_version: &str,
        configuration_version: i32,
    ) -> Result<ProductCatalogEntry, PlatformContractError> {
        let response = self
            .catalog
            .clone()
            .get_active_product(GetActiveProductRequest {
                product_id: product_id.to_string(),
            })
            .await
            .map_err(map_platform_status)?;
        let product = response.into_inner();

        let published_version = i32::try_from(product.configuration_version)
            .map_err(|_| PlatformContractError::InvalidPlatformResponse)?;
        if published_version != configuration_version {
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

    /// Resolves a verified active domain through the PLATFORM-owned gRPC contract.
    pub async fn resolve_domain(
        &self,
        domain: &str,
    ) -> Result<Option<ProductCatalogEntry>, PlatformContractError> {
        match self
            .catalog
            .clone()
            .resolve_active_domain(ResolveActiveDomainRequest {
                domain_name: domain.to_owned(),
            })
            .await
        {
            Ok(response) => Ok(Some(response.into_inner())),
            Err(status) if status.code() == Code::NotFound => Ok(None),
            Err(status) => Err(map_platform_status(status)),
        }
    }
}

fn map_platform_status(status: tonic::Status) -> PlatformContractError {
    match status.code() {
        Code::NotFound => PlatformContractError::ProductUnavailable,
        Code::InvalidArgument => PlatformContractError::InvalidPlatformRequest,
        Code::FailedPrecondition => PlatformContractError::UnsupportedPlatformContract,
        _ => PlatformContractError::PlatformUnavailable,
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
