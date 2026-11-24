use sqlx::PgConnection;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct TenantCreatedOutboxEvent<'a> {
    pub event_id: Uuid,
    pub tenant_id: Uuid,
    pub product_id: Uuid,
    pub product_config_version: i32,
    pub aggregate_version: i64,
    pub correlation_id: Uuid,
    /// Minimal JSON object; credentials and contact details must never be included.
    pub payload_json: &'a str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TenantOutboxError {
    InvalidEvent,
    Storage,
}

/// Appends the tenant-created fact using the caller's existing CORE transaction.
/// The caller must establish `app.tenant_id` to the new tenant ID before calling.
/// This function deliberately does not begin or commit a transaction.
pub async fn enqueue_tenant_created(
    connection: &mut PgConnection,
    event: &TenantCreatedOutboxEvent<'_>,
) -> Result<(), TenantOutboxError> {
    if event.event_id.is_nil()
        || event.tenant_id.is_nil()
        || event.product_id.is_nil()
        || event.correlation_id.is_nil()
        || event.product_config_version <= 0
        || event.aggregate_version <= 0
        || event.payload_json.trim().is_empty()
        || event.payload_json.len() > 16 * 1024
    {
        return Err(TenantOutboxError::InvalidEvent);
    }

    sqlx::query(
        "INSERT INTO core_tenancy.outbox_events (
            event_id, event_type, schema_version, aggregate_type, aggregate_id,
            aggregate_version, product_id, product_config_version, correlation_id,
            payload, occurred_at
         ) VALUES ($1, 'core.tenant.created.v1', 1, 'tenant', $2, $3, $4, $5, $6,
                   $7::jsonb, clock_timestamp())",
    )
    .bind(event.event_id)
    .bind(event.tenant_id)
    .bind(event.aggregate_version)
    .bind(event.product_id)
    .bind(event.product_config_version)
    .bind(event.correlation_id)
    .bind(event.payload_json)
    .execute(connection)
    .await
    .map_err(|_| TenantOutboxError::Storage)?;

    Ok(())
}
