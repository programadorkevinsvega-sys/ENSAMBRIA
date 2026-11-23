use crate::tenant_outbox::{TenantCreatedOutboxEvent, enqueue_tenant_created};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct CreateTenant {
    pub tenant_id: Uuid,
    pub organization_id: Uuid,
    pub product_id: Uuid,
    /// Product version already validated through the PLATFORM product contract.
    pub validated_product_config_version: i32,
    pub organization_name: String,
    pub country_code: String,
    pub currency_code: String,
    pub locale: String,
    pub time_zone: String,
    pub correlation_id: Uuid,
}

pub async fn create_tenant_with_outbox(
    pool: &PgPool,
    input: &CreateTenant,
) -> Result<Uuid, TenantLifecycleError> {
    if input.tenant_id.is_nil()
        || input.organization_id.is_nil()
        || input.product_id.is_nil()
        || input.correlation_id.is_nil()
        || input.validated_product_config_version <= 0
        || input.organization_name.trim().is_empty()
        || input.country_code.len() != 2
        || input.currency_code.len() != 3
        || input.locale.trim().is_empty()
        || input.time_zone.trim().is_empty()
    {
        return Err(TenantLifecycleError::InvalidInput);
    }

    let event_id = Uuid::now_v7();
    let payload = serde_json::json!({
        "tenantId": input.tenant_id,
        "productId": input.product_id,
        "productConfigVersion": input.validated_product_config_version
    });
    let payload_json =
        serde_json::to_string(&payload).map_err(|_| TenantLifecycleError::InvalidInput)?;
    let mut tx = pool
        .begin()
        .await
        .map_err(|_| TenantLifecycleError::Storage)?;

    sqlx::query("SELECT set_config('app.tenant_id', $1, true)")
        .bind(input.tenant_id.to_string())
        .execute(&mut *tx)
        .await
        .map_err(|_| TenantLifecycleError::Storage)?;
    sqlx::query(
        "INSERT INTO core.tenants (id,status,country_code,currency_code,locale,time_zone)
         VALUES ($1,'provisioning',$2,$3,$4,$5)",
    )
    .bind(input.tenant_id)
    .bind(&input.country_code)
    .bind(&input.currency_code)
    .bind(&input.locale)
    .bind(&input.time_zone)
    .execute(&mut *tx)
    .await
    .map_err(|_| TenantLifecycleError::Storage)?;
    sqlx::query(
        "INSERT INTO core.organizations
         (id,tenant_id,display_name,country_code,currency_code,locale,time_zone)
         VALUES ($1,$2,$3,$4,$5,$6,$7)",
    )
    .bind(input.organization_id)
    .bind(input.tenant_id)
    .bind(input.organization_name.trim())
    .bind(&input.country_code)
    .bind(&input.currency_code)
    .bind(&input.locale)
    .bind(&input.time_zone)
    .execute(&mut *tx)
    .await
    .map_err(|_| TenantLifecycleError::Storage)?;
    sqlx::query(
        "INSERT INTO core.tenant_products
         (tenant_id,product_id,product_config_version,association_status,validated_at)
         VALUES ($1,$2,$3,'pending',clock_timestamp())",
    )
    .bind(input.tenant_id)
    .bind(input.product_id)
    .bind(input.validated_product_config_version)
    .execute(&mut *tx)
    .await
    .map_err(|_| TenantLifecycleError::Storage)?;

    enqueue_tenant_created(
        &mut tx,
        &TenantCreatedOutboxEvent {
            event_id,
            tenant_id: input.tenant_id,
            product_id: input.product_id,
            product_config_version: input.validated_product_config_version,
            aggregate_version: 1,
            correlation_id: input.correlation_id,
            payload_json: &payload_json,
        },
    )
    .await
    .map_err(|_| TenantLifecycleError::Storage)?;
    tx.commit()
        .await
        .map_err(|_| TenantLifecycleError::Storage)?;
    Ok(event_id)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TenantLifecycleError {
    InvalidInput,
    Storage,
}

impl std::fmt::Display for TenantLifecycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::InvalidInput => "tenant creation input is invalid",
            Self::Storage => "CORE tenant creation transaction failed",
        })
    }
}
impl std::error::Error for TenantLifecycleError {}
