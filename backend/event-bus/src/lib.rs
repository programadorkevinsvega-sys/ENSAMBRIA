pub use async_nats;
pub use async_nats::jetstream;

use std::error::Error;

pub type EventBusError = Box<dyn Error + Send + Sync>;

pub async fn connect_jetstream(url: &str) -> Result<jetstream::Context, EventBusError> {
    Ok(jetstream::new(async_nats::connect(url).await?))
}

/// Waits for the durable JetStream PubAck and uses the Outbox event ID for
/// broker-side deduplication within the stream's configured duplicate window.
pub async fn publish_confirmed(
    context: &jetstream::Context,
    subject: &str,
    message_id: &str,
    payload: Vec<u8>,
) -> Result<(), EventBusError> {
    let mut headers = async_nats::HeaderMap::new();
    headers.append("Nats-Msg-Id", message_id.to_owned());
    context
        .publish_with_headers(subject.to_owned(), headers, payload.into())
        .await?
        .await?;
    Ok(())
}
