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

    /// Enforces both CORE's tenant association and PLATFORM's published capability contract.
    /// Tenant RLS is applied inside the transaction; product IDs remain opaque cross-service references.
    pub async fn require_tenant_capability(
        &self,
        pool: &sqlx::PgPool,
        tenant_id: Uuid,
        product_id: Uuid,
        capability_code: &str,
        contract_version: &str,
    ) -> Result<ProductCatalogEntry, PlatformContractError> {
        let mut transaction = pool.begin().await
            .map_err(|_| PlatformContractError::CoreUnavailable)?;
        sqlx::query("SELECT set_config('app.tenant_id', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut *transaction)
            .await
            .map_err(|_| PlatformContractError::CoreUnavailable)?;

        let association: Option<i32> = sqlx::query_scalar(
            "SELECT product_config_version FROM core.tenant_products              WHERE tenant_id = $1 AND product_id = $2 AND association_status = 'active'",
        )
        .bind(tenant_id)
        .bind(product_id)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|_| PlatformContractError::CoreUnavailable)?;

        transaction.rollback().await
            .map_err(|_| PlatformContractError::CoreUnavailable)?;
        let configuration_version = association
            .ok_or(PlatformContractError::TenantProductNotActive)?;

        self.require_capability(
            product_id,
            capability_code,
            contract_version,
            configuration_version,
        ).await
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
    TenantProductNotActive,
    CoreUnavailable,
    CapabilityUnavailable,
    ConfigurationVersionMismatch,
    ContractVersionMismatch,
    PlatformUnavailable,
}

impl std::fmt::Display for PlatformContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::ProductUnavailable => "PLATFORM product is not active or published",
            Self::TenantProductNotActive => "CORE tenant does not have an active association with this product",
            Self::CoreUnavailable => "CORE tenant product association could not be read",
            Self::CapabilityUnavailable => "PLATFORM product capability is not available",
            Self::ConfigurationVersionMismatch => "PLATFORM product configuration version does not match CORE association",
            Self::ContractVersionMismatch => "PLATFORM capability contract version does not match CORE requirement",
            Self::PlatformUnavailable => "PLATFORM product contract could not be read",
        })
    }
}
impl std::error::Error for PlatformContractError {}
