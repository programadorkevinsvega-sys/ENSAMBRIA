use ensambria_event_bus::{
    async_nats::jetstream::consumer::{AckPolicy, DeliverPolicy},
    jetstream::Context,
};
use ensambria_platform_tenant_onboarding_contracts::{
    OnboardingOutcome, TenantCreated, TenantOnboarding,
};
use futures_util::StreamExt;
use ring::digest::{SHA256, digest};
use serde::Deserialize;
use std::{env, error::Error, sync::Arc, time::Duration};
use uuid::Uuid;

const STREAM: &str = "ENSAMBRIA";
const DEFAULT_DURABLE: &str = "platform-tenant-onboarding";
const SUBJECT: &str = "ensambria.core.tenant.created.v1";

#[derive(Deserialize)]
struct EventAggregate {
    #[serde(rename = "type")]
    aggregate_type: String,
    id: Uuid,
    version: i64,
}

#[derive(Deserialize)]
struct TenantCreatedEnvelope {
    event_id: Uuid,
    event_type: String,
    schema_version: i32,
    occurred_at: String,
    producer: String,
    tenant_id: Uuid,
    aggregate: EventAggregate,
    correlation_id: Uuid,
    payload: serde_json::Value,
}

impl TenantCreatedEnvelope {
    fn into_event(self, raw: &[u8]) -> Result<TenantCreated, &'static str> {
        let tenant_id = self.tenant_id.to_string();
        let payload_tenant_id = self.payload.get("tenantId").and_then(|v| v.as_str());
        let payload_product_id = self.payload.get("productId").and_then(|v| v.as_str());
        let payload_config_version = self
            .payload
            .get("productConfigVersion")
            .and_then(|v| v.as_i64());
        let product_id = payload_product_id
            .and_then(|value| Uuid::parse_str(value).ok())
            .ok_or("invalid product ID in tenant-created payload")?;
        let product_id_string = product_id.to_string();
        let product_config_version = payload_config_version
            .and_then(|value| u32::try_from(value).ok())
            .filter(|value| *value > 0)
            .ok_or("invalid product configuration version in payload")?;

        if self.event_id.is_nil()
            || self.event_type != "core.tenant.created.v1"
            || self.schema_version != 1
            || self.occurred_at.trim().is_empty()
            || self.producer != "core"
            || self.tenant_id.is_nil()
            || self.aggregate.aggregate_type != "tenant"
            || self.aggregate.id != self.tenant_id
            || self.aggregate.version <= 0
            || self.correlation_id.is_nil()
            || payload_tenant_id != Some(tenant_id.as_str())
            || payload_product_id != Some(product_id_string.as_str())
        {
            return Err("invalid CORE tenant-created event contract");
        }

        let hash = digest(&SHA256, raw);
        let mut payload_hash = [0_u8; 32];
        payload_hash.copy_from_slice(hash.as_ref());
        Ok(TenantCreated {
            event_id: self.event_id,
            tenant_id: self.tenant_id,
            product_id,
            product_config_version,
            aggregate_version: self.aggregate.version,
            correlation_id: self.correlation_id,
            payload_sha256: payload_hash,
        })
    }
}

pub async fn consume_one(
    service: &dyn TenantOnboarding,
    jetstream: &Context,
) -> Result<OnboardingOutcome, Box<dyn Error + Send + Sync>> {
    let stream = jetstream.get_stream(STREAM).await?;
    let durable_name = env::var("PLATFORM_TENANT_ONBOARDING_DURABLE")
        .unwrap_or_else(|_| DEFAULT_DURABLE.to_owned());
    let start_sequence = env::var("PLATFORM_TENANT_ONBOARDING_START_SEQUENCE")
        .ok()
        .map(|value| value.parse::<u64>())
        .transpose()?;
    let mut config = ensambria_event_bus::async_nats::jetstream::consumer::pull::Config {
        durable_name: Some(durable_name.clone()),
        filter_subject: SUBJECT.to_owned(),
        ack_policy: AckPolicy::Explicit,
        ack_wait: configured_ack_wait(),
        max_deliver: 12,
        max_ack_pending: 100,
        ..Default::default()
    };
    if let Some(sequence) = start_sequence {
        if sequence == 0 {
            return Err(
                "PLATFORM_TENANT_ONBOARDING_START_SEQUENCE must be greater than zero".into(),
            );
        }
        config.deliver_policy = DeliverPolicy::ByStartSequence {
            start_sequence: sequence,
        };
    }
    let consumer = stream.get_or_create_consumer(&durable_name, config).await?;
    let mut messages = consumer.messages().await?;
    let message = messages
        .next()
        .await
        .ok_or("JetStream consumer ended without a message")??;
    let envelope: TenantCreatedEnvelope = serde_json::from_slice(&message.payload)?;
    let event = envelope
        .into_event(&message.payload)
        .map_err(std::io::Error::other)?;
    let event_id = event.event_id;
    if fault_injection_matches("ENSAMBRIA_TEST_PAUSE_BEFORE_APPLY_EVENT_ID", event_id) {
        tracing::error!(
            event_id = %event.event_id,
            "FAULT_INJECTION: paused after JetStream delivery and before Inbox transaction"
        );
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
    let outcome = service
        .apply_tenant_created(event)
        .await
        .map_err(|error| std::io::Error::other(format!("{error:?}")))?;
    if fault_injection_matches("ENSAMBRIA_TEST_PAUSE_AFTER_COMMIT_EVENT_ID", event_id) {
        tracing::error!(
            event_id = %event.event_id,
            "FAULT_INJECTION: paused after Inbox/onboarding commit and before JetStream ACK"
        );
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
    // ACK only after Inbox + onboarding have committed in PostgreSQL.
    message.ack().await?;
    Ok(outcome)
}

pub async fn run(
    service: Arc<dyn TenantOnboarding>,
    jetstream: Context,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    loop {
        match consume_one(service.as_ref(), &jetstream).await {
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

fn configured_ack_wait() -> Duration {
    let seconds = env::var("PLATFORM_JETSTREAM_ACK_WAIT_SECONDS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| (1..=3600).contains(value))
        .unwrap_or(45);
    Duration::from_secs(seconds)
}

fn fault_injection_matches(variable: &str, event_id: Uuid) -> bool {
    if !cfg!(debug_assertions)
        || env::var("ENSAMBRIA_ENABLE_FAULT_INJECTION").ok().as_deref() != Some("1")
    {
        return false;
    }
    env::var(variable).is_ok_and(|configured_event_id| configured_event_id == event_id.to_string())
}
