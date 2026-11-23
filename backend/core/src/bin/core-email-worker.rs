use std::{error::Error, time::Duration};

use ensambria_core::{
    identity_authentication::email_outbox::run_smtp_worker_from_env,
    observability::initialize_tracing, postgres::PgDatabaseConfiguration,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    initialize_tracing()?;
    let pool = PgDatabaseConfiguration::from_env_with_variable_names(
        "CORE_EMAIL_WORKER_DATABASE_URL",
        "CORE_EMAIL_WORKER_DATABASE_MAX_CONNECTIONS",
        "CORE_EMAIL_WORKER_DATABASE_ACQUIRE_TIMEOUT_MS",
        "CORE_EMAIL_WORKER_DATABASE_IDLE_TIMEOUT_MS",
        "CORE_EMAIL_WORKER_DATABASE_MAX_LIFETIME_MS",
    )?
    .connect()
    .await?;
    tracing::info!(
        event = "auth.email_worker.started",
        component = "backend-core",
        operation = "email_worker.start",
        result = "success"
    );
    run_smtp_worker_from_env(pool.clone(), Duration::from_secs(5), 25).await?;
    pool.close().await;
    Ok(())
}
