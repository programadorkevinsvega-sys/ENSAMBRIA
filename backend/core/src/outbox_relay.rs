use ensambria_event_bus::{jetstream::Context, publish_confirmed};
use serde::Serialize;
use serde_json::Value;
use sqlx::{PgPool, types::Json};
use std::{error::Error, time::Duration};
use uuid::Uuid;

const SUBJECT: &str = "ensambria.core.tenant.created.v1";
const MAX_ATTEMPTS: i32 = 12;
const LEASE_SECONDS: i32 = 45;

#[derive(Debug)]
struct ClaimedEvent {
    event_id: Uuid, event_type: String, schema_version: i32,
    aggregate_id: Uuid, aggregate_version: i64, product_id: Uuid,
    product_config_version: i32, correlation_id: Uuid,
    payload: Json<Value>, attempt_count: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EventEnvelope<'a> {
    event_id: Uuid, event_type: &'a str, schema_version: i32,
    aggregate_type: &'static str, aggregate_id: Uuid, aggregate_version: i64,
    product_id: Uuid, product_config_version: i32, correlation_id: Uuid,
    payload: &'a Value,
}

pub async fn publish_pending_once(
    pool: &PgPool, jetstream: &Context, relay_id: &str, batch_size: i64,
) -> Result<usize, Box<dyn Error + Send + Sync>> {
    let mut tx = pool.begin().await?;
    let rows = sqlx::query_as::<_, (Uuid, String, i32, Uuid, i64, Uuid, i32, Uuid, Json<Value>, i32)>(
        "WITH candidates AS (
            SELECT event_id FROM core.outbox_events
             WHERE (status IN ('pending', 'retry_wait') AND available_at <= clock_timestamp())
                OR (status = 'leased' AND lease_until < clock_timestamp())
             ORDER BY occurred_at, event_id FOR UPDATE SKIP LOCKED LIMIT $2
         )
         UPDATE core.outbox_events o
            SET status = 'leased', lease_owner = $1,
                lease_until = clock_timestamp() + make_interval(secs => $3),
                attempt_count = o.attempt_count + 1
           FROM candidates c WHERE o.event_id = c.event_id
         RETURNING o.event_id, o.event_type, o.schema_version, o.aggregate_id,
                   o.aggregate_version, o.product_id, o.product_config_version,
                   o.correlation_id, o.payload, o.attempt_count",
    )
    .bind(relay_id).bind(batch_size.clamp(1, 100)).bind(LEASE_SECONDS)
    .fetch_all(&mut *tx).await?;
    tx.commit().await?;

    let mut published = 0;
    for row in rows {
        let event = ClaimedEvent {
            event_id: row.0, event_type: row.1, schema_version: row.2,
            aggregate_id: row.3, aggregate_version: row.4, product_id: row.5,
            product_config_version: row.6, correlation_id: row.7,
            payload: row.8, attempt_count: row.9,
        };
        let envelope = EventEnvelope {
            event_id: event.event_id, event_type: &event.event_type,
            schema_version: event.schema_version, aggregate_type: "tenant",
            aggregate_id: event.aggregate_id, aggregate_version: event.aggregate_version,
            product_id: event.product_id, product_config_version: event.product_config_version,
            correlation_id: event.correlation_id, payload: &event.payload.0,
        };
        let bytes = serde_json::to_vec(&envelope)?;
        match publish_confirmed(jetstream, SUBJECT, bytes).await {
            Ok(()) => {
                let changed = sqlx::query(
                    "UPDATE core.outbox_events SET status='published',
                        published_at=clock_timestamp(), lease_owner=NULL, lease_until=NULL,
                        last_error_code=NULL
                      WHERE event_id=$1 AND status='leased' AND lease_owner=$2",
                ).bind(event.event_id).bind(relay_id).execute(pool).await?.rows_affected();
                if changed == 1 { published += 1; }
            }
            Err(error) => {
                let terminal = event.attempt_count >= MAX_ATTEMPTS;
                let delay = (1_i64 << event.attempt_count.min(8)).min(300) as i32;
                sqlx::query(
                    "UPDATE core.outbox_events
                        SET status=CASE WHEN $3 THEN 'quarantined' ELSE 'retry_wait' END,
                            available_at=clock_timestamp()+make_interval(secs=>$4),
                            lease_owner=NULL, lease_until=NULL,
                            last_error_code='JETSTREAM_PUBLISH_FAILED'
                      WHERE event_id=$1 AND status='leased' AND lease_owner=$2",
                ).bind(event.event_id).bind(relay_id).bind(terminal).bind(delay)
                 .execute(pool).await?;
                tracing::warn!(event_id=%event.event_id, attempt=event.attempt_count,
                    quarantined=terminal, error=%error, "CORE Outbox publish failed");
            }
        }
    }
    Ok(published)
}

pub async fn run(
    pool: PgPool, jetstream: Context, relay_id: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut interval = tokio::time::interval(Duration::from_millis(750));
    loop {
        interval.tick().await;
        if let Err(error) = publish_pending_once(&pool, &jetstream, &relay_id, 25).await {
            tracing::error!(error=%error, "CORE Outbox relay iteration failed");
        }
    }
}
