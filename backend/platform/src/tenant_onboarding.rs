use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

const TENANT_CREATED_EVENT: &str = "core.tenant.created.v1";
const CONSUMER_NAME: &str = "platform.tenant-onboarding";

#[derive(Clone, Debug)]
pub struct TenantCreatedEvent {
    pub event_id: Uuid,
    pub tenant_id: Uuid,
    pub product_id: Uuid,
    pub product_config_version: i32,
    pub aggregate_version: i64,
    pub correlation_id: Uuid,
    /// SHA-256 of the exact, validated event payload bytes.
    pub payload_hash: [u8; 32],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TenantOnboardingOutcome {
    Applied,
    Duplicate,
    Quarantined,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TenantOnboardingError {
    InvalidEvent,
    EventIdCollision,
    Storage,
}

impl TenantCreatedEvent {
    fn validate(&self) -> Result<(), TenantOnboardingError> {
        if self.event_id.is_nil()
            || self.tenant_id.is_nil()
            || self.product_id.is_nil()
            || self.correlation_id.is_nil()
            || self.product_config_version <= 0
            || self.aggregate_version <= 0
            || self.payload_hash.iter().all(|byte| *byte == 0)
        {
            return Err(TenantOnboardingError::InvalidEvent);
        }
        Ok(())
    }
}

/// Applies one CORE tenant-created fact. Inbox and commercial onboarding are
/// committed atomically; callers may ACK JetStream only after this returns.
pub async fn apply_tenant_created(
    pool: &PgPool,
    event: &TenantCreatedEvent,
) -> Result<TenantOnboardingOutcome, TenantOnboardingError> {
    event.validate()?;
    let mut transaction = pool.begin().await.map_err(|_| TenantOnboardingError::Storage)?;

    let inserted = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO platform.inbox_messages (
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
    .bind(event.product_config_version)
    .bind(event.correlation_id)
    .bind(event.payload_hash.to_vec())
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|_| TenantOnboardingError::Storage)?;

    if inserted.is_none() {
        let existing = sqlx::query_as::<_, (Uuid, i64, Uuid, i32, Uuid, Vec<u8>, String)>(
            "SELECT aggregate_id, aggregate_version, product_id, product_config_version,
                    correlation_id, payload_hash, status
               FROM platform.inbox_messages
              WHERE event_id = $1
              FOR UPDATE",
        )
        .bind(event.event_id)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|_| TenantOnboardingError::Storage)?
        .ok_or(TenantOnboardingError::Storage)?;

        if existing.0 != event.tenant_id
            || existing.1 != event.aggregate_version
            || existing.2 != event.product_id
            || existing.3 != event.product_config_version
            || existing.4 != event.correlation_id
            || existing.5.as_slice() != event.payload_hash
        {
            return Err(TenantOnboardingError::EventIdCollision);
        }

        match existing.6.as_str() {
            "applied" => {
                transaction.commit().await.map_err(|_| TenantOnboardingError::Storage)?;
                return Ok(TenantOnboardingOutcome::Duplicate);
            }
            "quarantined" => {
                transaction.commit().await.map_err(|_| TenantOnboardingError::Storage)?;
                return Ok(TenantOnboardingOutcome::Quarantined);
            }
            "received" => {}
            _ => return Err(TenantOnboardingError::Storage),
        }
    }

    let product_configuration_is_active: bool = sqlx::query_scalar(
        "SELECT EXISTS (
            SELECT 1
              FROM platform.active_product_catalog
             WHERE product_id = $1
               AND configuration_version = $2
         )",
    )
    .bind(event.product_id)
    .bind(event.product_config_version)
    .fetch_one(&mut *transaction)
    .await
    .map_err(|_| TenantOnboardingError::Storage)?;

    if !product_configuration_is_active {
        quarantine(&mut transaction, event.event_id, "PRODUCT_CONFIGURATION_UNAVAILABLE").await?;
        transaction.commit().await.map_err(|_| TenantOnboardingError::Storage)?;
        return Ok(TenantOnboardingOutcome::Quarantined);
    }

    let onboarding_inserted = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO platform.tenant_onboardings (
            tenant_id, product_id, product_config_version, source_event_id
         ) VALUES ($1, $2, $3, $4)
         ON CONFLICT (tenant_id, product_id) DO NOTHING
         RETURNING tenant_id",
    )
    .bind(event.tenant_id)
    .bind(event.product_id)
    .bind(event.product_config_version)
    .bind(event.event_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|_| TenantOnboardingError::Storage)?;

    if onboarding_inserted.is_none() {
        let existing_event_id: Uuid = sqlx::query_scalar(
            "SELECT source_event_id
               FROM platform.tenant_onboardings
              WHERE tenant_id = $1 AND product_id = $2
              FOR UPDATE",
        )
        .bind(event.tenant_id)
        .bind(event.product_id)
        .fetch_one(&mut *transaction)
        .await
        .map_err(|_| TenantOnboardingError::Storage)?;

        if existing_event_id != event.event_id {
            quarantine(&mut transaction, event.event_id, "TENANT_PRODUCT_ALREADY_ONBOARDED")
                .await?;
            transaction.commit().await.map_err(|_| TenantOnboardingError::Storage)?;
            return Ok(TenantOnboardingOutcome::Quarantined);
        }
    }

    sqlx::query(
        "UPDATE platform.inbox_messages
            SET status = 'applied', applied_at = clock_timestamp(), last_error_code = NULL
          WHERE event_id = $1 AND status = 'received'",
    )
    .bind(event.event_id)
    .execute(&mut *transaction)
    .await
    .map_err(|_| TenantOnboardingError::Storage)?;

    transaction.commit().await.map_err(|_| TenantOnboardingError::Storage)?;
    Ok(TenantOnboardingOutcome::Applied)
}

async fn quarantine(
    transaction: &mut Transaction<'_, Postgres>,
    event_id: Uuid,
    error_code: &'static str,
) -> Result<(), TenantOnboardingError> {
    sqlx::query(
        "UPDATE platform.inbox_messages
            SET status = 'quarantined', last_error_code = $2
          WHERE event_id = $1 AND status = 'received'",
    )
    .bind(event_id)
    .bind(error_code)
    .execute(&mut **transaction)
    .await
    .map_err(|_| TenantOnboardingError::Storage)?;
    Ok(())
}
