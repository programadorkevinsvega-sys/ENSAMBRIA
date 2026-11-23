use ensambria_core::outbox_relay;
use ensambria_event_bus::connect_jetstream;
use sqlx::postgres::PgPoolOptions;
use std::{env, error::Error, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing_subscriber::fmt().json().with_env_filter(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
    ).init();
    let pool = PgPoolOptions::new().max_connections(4)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&env::var("CORE_OUTBOX_DATABASE_URL")?).await?;
    let jetstream = connect_jetstream(&env::var("ENSAMBRIA_NATS_URL")?).await?;
    let relay_id = env::var("CORE_OUTBOX_RELAY_ID")
        .unwrap_or_else(|_| format!("core-outbox-{}", uuid::Uuid::now_v7()));
    outbox_relay::run(pool, jetstream, relay_id).await
}
