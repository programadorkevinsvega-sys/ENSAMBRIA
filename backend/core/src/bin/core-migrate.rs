use std::{error::Error, io, path::PathBuf};

use ensambria_core::{
    core_migrations::{migration_database_url_from_env, run_core_migrations_from_path},
    observability::initialize_tracing,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    initialize_tracing()?;
    let mut arguments = std::env::args_os().skip(1);
    let migrations_path = arguments
        .next()
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::other("core-migrate requires a migrations directory"))?;
    if arguments.next().is_some() {
        return Err(io::Error::other("unexpected core-migrate argument").into());
    }
    let database_url = migration_database_url_from_env()?;
    run_core_migrations_from_path(&database_url, &migrations_path).await?;
    Ok(())
}
