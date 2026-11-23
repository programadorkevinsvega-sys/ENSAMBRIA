use crate::tenant_onboarding::{TenantCreatedEvent, TenantOnboardingOutcome, apply_tenant_created};
use ensambria_event_bus::{async_nats::jetstream::consumer::AckPolicy, jetstream::Context};
use futures_util::StreamExt;
use ring::digest::{SHA256, digest};
use serde::Deserialize;
use sqlx::PgPool;
use std::{error::Error, time::Duration};

const STREAM: &str = "ENSAMBRIA";
const DURABLE: &str = "platform-tenant-onboarding";
const SUBJECT: &str = "ensambria.core.tenant.created.v1";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct TenantCreatedEnvelope {
    event_id: uuid::Uuid,
    event_type: String,
    schema_version: i32,
    aggregate_type: String,
    aggregate_id: uuid::Uuid,
    aggregate_version: i64,
    product_id: uuid::Uuid,
    product_config_version: i32,
    correlation_id: uuid::Uuid,
    payload: serde_json::Value,
}

impl TenantCreatedEnvelope {
    fn into_event(self, raw: &[u8]) -> Result<TenantCreatedEvent, &'static str> {
        let tenant_id = self.aggregate_id.to_string();
        let product_id = self.product_id.to_string();
        let payload_tenant_id = self.payload.get("tenantId").and_then(|v| v.as_str());
        let payload_product_id = self.payload.get("productId").and_then(|v| v.as_str());
        let payload_config_version = self
            .payload
            .get("productConfigVersion")
            .and_then(|v| v.as_i64());

        if self.event_type != "core.tenant.created.v1"
            || self.schema_version != 1
            || self.aggregate_type != "tenant"
            || payload_tenant_id != Some(tenant_id.as_str())
            || payload_product_id != Some(product_id.as_str())
            || payload_config_version != Some(i64::from(self.product_config_version))
        {
            return Err("invalid tenant-created event contract");
        }

        let hash = digest(&SHA256, raw);
        let mut payload_hash = [0_u8; 32];
        payload_hash.copy_from_slice(hash.as_ref());
        Ok(TenantCreatedEvent {
            event_id: self.event_id,
            tenant_id: self.aggregate_id,
            product_id: self.product_id,
            product_config_version: self.product_config_version,
            aggregate_version: self.aggregate_version,
            correlation_id: self.correlation_id,
            payload_hash,
        })
    }
}

pub async fn consume_one(
    pool: &PgPool,
    jetstream: &Context,
) -> Result<TenantOnboardingOutcome, Box<dyn Error + Send + Sync>> {
    let stream = jetstream.get_stream(STREAM).await?;
    let consumer = stream
        .get_or_create_consumer(
            DURABLE,
            ensambria_event_bus::async_nats::jetstream::consumer::pull::Config {
                durable_name: Some(DURABLE.to_owned()),
                filter_subject: SUBJECT.to_owned(),
                ack_policy: AckPolicy::Explicit,
                ack_wait: Duration::from_secs(45),
                max_deliver: 12,
                max_ack_pending: 100,
                ..Default::default()
            },
        )
        .await?;
    let mut messages = consumer.messages().await?;
    let message = messages
        .next()
        .await
        .ok_or("JetStream consumer ended without a message")??;
    let envelope: TenantCreatedEnvelope = serde_json::from_slice(&message.payload)?;
    let event = envelope
        .into_event(&message.payload)
        .map_err(std::io::Error::other)?;
    let outcome = apply_tenant_created(pool, &event).await?;
    // ACK only after Inbox + onboarding have committed in PostgreSQL.
    message.ack().await?;
    Ok(outcome)
}

pub async fn run(pool: PgPool, jetstream: Context) -> Result<(), Box<dyn Error + Send + Sync>> {
    loop {
        match consume_one(&pool, &jetstream).await {
            Ok(outcome) => {
                tracing::info!(?outcome, "PLATFORM tenant event committed and ACKed")
            }
            Err(error) => {
                tracing::error!(error = %error, "PLATFORM tenant consumer iteration failed");
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }
}
