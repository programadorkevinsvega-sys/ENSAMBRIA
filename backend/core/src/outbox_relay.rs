use ensambria_event_bus::{jetstream::Context, publish_confirmed};
use serde::Serialize;
use serde_json::Value;
use sqlx::{PgPool, types::Json};
use std::{env, error::Error, time::Duration};
use uuid::Uuid;

const SUBJECT: &str = "ensambria.core.tenant.created.v1";
const MAX_ATTEMPTS: i32 = 12;
const LEASE_SECONDS: i32 = 45;

#[derive(Debug)]
struct ClaimedEvent {
    event_id: Uuid,
    event_type: String,
    schema_version: i32,
    aggregate_id: Uuid,
    aggregate_version: i64,
    correlation_id: Uuid,
    payload: Json<Value>,
    attempt_count: i32,
    occurred_at: String,
}

#[derive(Serialize)]
struct EventAggregate {
    #[serde(rename = "type")]
    aggregate_type: &'static str,
    id: Uuid,
    version: i64,
}

#[derive(Serialize)]
struct EventEnvelope<'a> {
    event_id: Uuid,
    event_type: &'a str,
    schema_version: i32,
    occurred_at: &'a str,
    producer: &'static str,
    tenant_id: Uuid,
    aggregate: EventAggregate,
    correlation_id: Uuid,
    payload: &'a Value,
}

pub async fn publish_pending_once(
    pool: &PgPool,
    jetstream: &Context,
    relay_id: &str,
    batch_size: i64,
) -> Result<usize, Box<dyn Error + Send + Sync>> {
    let mut tx = pool.begin().await?;
    let rows = sqlx::query_as::<_, (Uuid, String, i32, Uuid, i64, Uuid, Json<Value>, i32, String)>(
        "WITH candidates AS (
            SELECT event_id FROM core.outbox_events
             WHERE (status IN ('pending', 'retry_wait') AND available_at <= clock_timestamp())
                OR (status = 'leased' AND lease_until < clock_timestamp())
             ORDER BY occurred_at, event_id LIMIT $2 FOR UPDATE SKIP LOCKED
         )
         UPDATE core.outbox_events o
            SET status = 'leased', lease_owner = $1,
                lease_until = clock_timestamp() + make_interval(secs => $3),
                attempt_count = o.attempt_count + 1
           FROM candidates c WHERE o.event_id = c.event_id
         RETURNING o.event_id, o.event_type, o.schema_version, o.aggregate_id,
                   o.aggregate_version, o.correlation_id, o.payload, o.attempt_count,
                   to_char(o.occurred_at AT TIME ZONE 'UTC',
                           'YYYY-MM-DD\"T\"HH24:MI:SS.US\"Z\"')",
    )
    .bind(relay_id)
    .bind(batch_size.clamp(1, 100))
    .bind(configured_lease_seconds())
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;

    let mut published = 0;
    for row in rows {
        let event = ClaimedEvent {
            event_id: row.0,
            event_type: row.1,
            schema_version: row.2,
            aggregate_id: row.3,
            aggregate_version: row.4,
            correlation_id: row.5,
            payload: row.6,
            attempt_count: row.7,
            occurred_at: row.8,
        };
        let envelope = EventEnvelope {
            event_id: event.event_id,
            event_type: &event.event_type,
            schema_version: event.schema_version,
            occurred_at: &event.occurred_at,
            producer: "core",
            tenant_id: event.aggregate_id,
            aggregate: EventAggregate {
                aggregate_type: "tenant",
                id: event.aggregate_id,
                version: event.aggregate_version,
            },
            correlation_id: event.correlation_id,
            payload: &event.payload.0,
        };
        let bytes = serde_json::to_vec(&envelope)?;
        let message_id = event.event_id.to_string();
        match publish_confirmed(jetstream, SUBJECT, &message_id, bytes).await {
            Ok(()) => {
                if fault_injection_matches("ENSAMBRIA_TEST_PAUSE_AFTER_PUBACK_EVENT_ID", event.event_id) {
                    tracing::error!(
                        event_id = %event.event_id,
                        "FAULT_INJECTION: paused after JetStream PubAck and before Outbox published update"
                    );
                    tokio::time::sleep(Duration::from_secs(60)).await;
                }
                let changed = sqlx::query(
                    "UPDATE core.outbox_events SET status='published',
                        published_at=clock_timestamp(), lease_owner=NULL, lease_until=NULL,
                        last_error_code=NULL
                      WHERE event_id=$1 AND status='leased' AND lease_owner=$2",
                )
                .bind(event.event_id)
                .bind(relay_id)
                .execute(pool)
                .await?
                .rows_affected();
                if changed == 1 {
                    published += 1;
                }
            }
            Err(error) => {
                let terminal = event.attempt_count >= MAX_ATTEMPTS;
                // Stable per-event jitter spreads retry load without adding a
                // random-number dependency to the relay worker.
                let base_delay = (1_i64 << event.attempt_count.min(8)).min(299) as f64;
                let jitter_millis = f64::from(event.event_id.as_bytes()[0]) * 1000.0 / 255.0;
                let delay_seconds = base_delay + jitter_millis / 1000.0;
                sqlx::query(
                    "UPDATE core.outbox_events
                        SET status=CASE WHEN $3 THEN 'quarantined' ELSE 'retry_wait' END,
                            available_at=clock_timestamp()+make_interval(secs=>$4),
                            lease_owner=NULL, lease_until=NULL,
                            last_error_code='JETSTREAM_PUBLISH_FAILED'
                      WHERE event_id=$1 AND status='leased' AND lease_owner=$2",
                )
                .bind(event.event_id)
                .bind(relay_id)
                .bind(terminal)
                .bind(delay_seconds)
                .execute(pool)
                .await?;
                tracing::warn!(
                    event_id = %event.event_id,
                    attempt = event.attempt_count,
                    quarantined = terminal,
                    error = %error,
                    "CORE Outbox publish failed"
                );
            }
        }
    }
    Ok(published)
}

pub async fn run(
    pool: PgPool,
    jetstream: Context,
    relay_id: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut interval = tokio::time::interval(Duration::from_millis(750));
    loop {
        interval.tick().await;
        if let Err(error) = publish_pending_once(&pool, &jetstream, &relay_id, 25).await {
            tracing::error!(error = %error, "CORE Outbox relay iteration failed");
        }
    }
}

fn configured_lease_seconds() -> i32 {
    env::var("CORE_OUTBOX_LEASE_SECONDS")
        .ok()
        .and_then(|value| value.parse::<i32>().ok())
        .filter(|value| (1..=600).contains(value))
        .unwrap_or(LEASE_SECONDS)
}

fn fault_injection_matches(variable: &str, event_id: Uuid) -> bool {
    if env::var("ENSAMBRIA_ENABLE_FAULT_INJECTION").as_deref() != Ok("1") {
        return false;
    }
    env::var(variable)
        .is_ok_and(|configured_event_id| configured_event_id == event_id.to_string())
}
