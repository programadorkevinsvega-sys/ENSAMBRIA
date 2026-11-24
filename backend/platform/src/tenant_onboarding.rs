use std::sync::Arc;

use ensambria_platform_product_catalog_contracts::{
    ProductCatalog as ProductCatalogPort, ProductCatalogFailure,
};
use ensambria_platform_tenant_onboarding_contracts::{
    ContractFuture, OnboardingFailure, OnboardingOutcome, TenantCreated, TenantOnboarding,
};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

const TENANT_CREATED_EVENT: &str = "core.tenant.created.v1";
const CONSUMER_NAME: &str = "platform.tenant-onboarding";

#[derive(Clone)]
pub struct PlatformTenantOnboarding {
    pool: PgPool,
    product_catalog: Arc<dyn ProductCatalogPort>,
}

impl PlatformTenantOnboarding {
    pub fn new(pool: PgPool, product_catalog: Arc<dyn ProductCatalogPort>) -> Self {
        Self {
            pool,
            product_catalog,
        }
    }

    async fn apply(&self, event: &TenantCreated) -> Result<OnboardingOutcome, OnboardingFailure> {
        validate_event(event)?;
        let active_product = match self.product_catalog.get_active_product(event.product_id).await {
            Ok(product) => Some(product),
            Err(ProductCatalogFailure::NotFound) => None,
            Err(_) => return Err(OnboardingFailure::Unavailable),
        };
        let product_configuration_is_active = active_product.is_some_and(|product| {
            product.configuration_version == event.product_config_version
        });

        let product_config_version = i32::try_from(event.product_config_version)
            .map_err(|_| OnboardingFailure::InvalidEvent)?;
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|_| OnboardingFailure::Unavailable)?;

        sqlx::query("SELECT set_config('app.tenant_id', $1, true)")
            .bind(event.tenant_id.to_string())
            .execute(&mut *transaction)
            .await
            .map_err(|_| OnboardingFailure::Unavailable)?;

        let inserted = sqlx::query_scalar::<_, Uuid>(
            "INSERT INTO platform_tenant_onboarding.inbox_messages (
                event_id, consumer_name, event_type, schema_version, aggregate_id,
                aggregate_version, product_id, product_config_version, correlation_id, payload_hash
             ) VALUES ($1, $2, $3, 1, $4, $5, $6, $7, $8, $9)
             ON CONFLICT (event_id) DO NOTHING
             RETURNING event_id",
        )
        .bind(event.event_id)
        .bind(CONSUMER_NAME)
        .bind(TENANT_CREATED_EVENT)
        .bind(event.tenant_id)
        .bind(event.aggregate_version)
        .bind(event.product_id)
        .bind(product_config_version)
        .bind(event.correlation_id)
        .bind(event.payload_sha256.to_vec())
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|_| OnboardingFailure::Unavailable)?;

        if inserted.is_none() {
            let existing = sqlx::query_as::<_, (Uuid, i64, Uuid, i32, Uuid, Vec<u8>, String)>(
                "SELECT aggregate_id, aggregate_version, product_id, product_config_version,
                        correlation_id, payload_hash, status
                   FROM platform_tenant_onboarding.inbox_messages
                  WHERE event_id = $1
                  FOR UPDATE",
            )
            .bind(event.event_id)
            .fetch_optional(&mut *transaction)
            .await
            .map_err(|_| OnboardingFailure::Unavailable)?
            .ok_or(OnboardingFailure::Unavailable)?;

            if existing.0 != event.tenant_id
                || existing.1 != event.aggregate_version
                || existing.2 != event.product_id
                || existing.3 != product_config_version
                || existing.4 != event.correlation_id
                || existing.5.as_slice() != event.payload_sha256.as_slice()
            {
                return Err(OnboardingFailure::EventIdCollision);
            }

            match existing.6.as_str() {
                "applied" => {
                    transaction.commit().await.map_err(|_| OnboardingFailure::Unavailable)?;
                    return Ok(OnboardingOutcome::Duplicate);
                }
                "quarantined" => {
                    transaction.commit().await.map_err(|_| OnboardingFailure::Unavailable)?;
                    return Ok(OnboardingOutcome::Quarantined);
                }
                "received" => {}
                _ => return Err(OnboardingFailure::Unavailable),
            }
        }

        if !product_configuration_is_active {
            quarantine(
                &mut transaction,
                event.event_id,
                "PRODUCT_CONFIGURATION_UNAVAILABLE",
            )
            .await?;
            transaction.commit().await.map_err(|_| OnboardingFailure::Unavailable)?;
            return Ok(OnboardingOutcome::Quarantined);
        }

        let onboarding_inserted = sqlx::query_scalar::<_, Uuid>(
            "INSERT INTO platform_tenant_onboarding.tenant_onboardings (
                tenant_id, product_id, product_config_version, source_event_id
             ) VALUES ($1, $2, $3, $4)
             ON CONFLICT (tenant_id, product_id) DO NOTHING
             RETURNING tenant_id",
        )
        .bind(event.tenant_id)
        .bind(event.product_id)
        .bind(product_config_version)
        .bind(event.event_id)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|_| OnboardingFailure::Unavailable)?;

        if onboarding_inserted.is_none() {
            let existing_event_id: Uuid = sqlx::query_scalar(
                "SELECT source_event_id
                   FROM platform_tenant_onboarding.tenant_onboardings
                  WHERE tenant_id = $1 AND product_id = $2
                  FOR UPDATE",
            )
            .bind(event.tenant_id)
            .bind(event.product_id)
            .fetch_one(&mut *transaction)
            .await
            .map_err(|_| OnboardingFailure::Unavailable)?;

            if existing_event_id != event.event_id {
                quarantine(
                    &mut transaction,
                    event.event_id,
                    "TENANT_PRODUCT_ALREADY_ONBOARDED",
                )
                .await?;
                transaction.commit().await.map_err(|_| OnboardingFailure::Unavailable)?;
                return Ok(OnboardingOutcome::Quarantined);
            }
        }

        sqlx::query(
            "UPDATE platform_tenant_onboarding.inbox_messages
                SET status = 'applied', applied_at = clock_timestamp(), last_error_code = NULL
              WHERE event_id = $1 AND status = 'received'",
        )
        .bind(event.event_id)
        .execute(&mut *transaction)
        .await
        .map_err(|_| OnboardingFailure::Unavailable)?;

        transaction.commit().await.map_err(|_| OnboardingFailure::Unavailable)?;
        Ok(OnboardingOutcome::Applied)
    }
}

impl TenantOnboarding for PlatformTenantOnboarding {
    fn apply_tenant_created<'a>(
        &'a self,
        event: TenantCreated,
    ) -> ContractFuture<'a, Result<OnboardingOutcome, OnboardingFailure>> {
        Box::pin(async move { self.apply(&event).await })
    }
}

fn validate_event(event: &TenantCreated) -> Result<(), OnboardingFailure> {
    if event.event_id.is_nil()
        || event.tenant_id.is_nil()
        || event.product_id.is_nil()
        || event.correlation_id.is_nil()
        || event.product_config_version == 0
        || event.aggregate_version <= 0
    {
        return Err(OnboardingFailure::InvalidEvent);
    }
    Ok(())
}

async fn quarantine(
    transaction: &mut Transaction<'_, Postgres>,
    event_id: Uuid,
    error_code: &'static str,
) -> Result<(), OnboardingFailure> {
    sqlx::query(
        "UPDATE platform_tenant_onboarding.inbox_messages
            SET status = 'quarantined', last_error_code = $2
          WHERE event_id = $1 AND status = 'received'",
    )
    .bind(event_id)
    .bind(error_code)
    .execute(&mut **transaction)
    .await
    .map_err(|_| OnboardingFailure::Unavailable)?;
    Ok(())
}
