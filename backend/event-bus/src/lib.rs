pub use async_nats;
pub use async_nats::jetstream;

use std::error::Error;
pub type EventBusError = Box<dyn Error + Send + Sync>;

pub async fn connect_jetstream(url: &str) -> Result<jetstream::Context, EventBusError> {
    Ok(jetstream::new(async_nats::connect(url).await?))
}

pub async fn publish_confirmed(
    context: &jetstream::Context,
    subject: &str,
    payload: Vec<u8>,
) -> Result<(), EventBusError> {
    context.publish(subject.to_owned(), payload.into()).await?.await?;
    Ok(())
}
