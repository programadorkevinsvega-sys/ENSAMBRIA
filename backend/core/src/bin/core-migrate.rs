use std::error::Error;

use ensambria_core::{
    core_migrations::{migration_database_url_from_env, run_core_migrations},
    observability::initialize_tracing,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    initialize_tracing()?;
    let database_url = migration_database_url_from_env()?;
    run_core_migrations(&database_url).await?;
    Ok(())
}
