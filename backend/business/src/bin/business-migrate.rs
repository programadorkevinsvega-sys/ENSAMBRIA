use std::{
    error::Error,
    fs,
    io,
    path::{Path, PathBuf},
};

use sqlx::postgres::PgPoolOptions;

const BOOTSTRAP: &str = "bootstrap/00000000000000_business_schema.sql";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut arguments = std::env::args_os().skip(1);
    let migrations_directory = arguments
        .next()
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::other("business-migrate requires a migrations directory"))?;
    if arguments.next().is_some() {
        return Err(io::Error::other("unexpected business-migrate argument").into());
    }

    let database_url = std::env::var("MIGRATIONS_DATABASE_URL")
        .map_err(|_| io::Error::other("MIGRATIONS_DATABASE_URL is required"))?;
    let bootstrap_path = migrations_directory.join(BOOTSTRAP);
    let bootstrap = fs::read_to_string(&bootstrap_path)?;

    let mut migration_files = fs::read_dir(&migrations_directory)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| is_sql_file(path))
        .collect::<Vec<_>>();
    migration_files.sort();
    if migration_files.is_empty() {
        return Err(io::Error::other("business migrations directory is empty").into());
    }

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await?;
    let result = apply_migrations(&pool, &bootstrap, &migration_files).await;
    pool.close().await;
    result?;
    Ok(())
}

fn is_sql_file(path: &Path) -> bool {
    path.extension().is_some_and(|extension| extension == "sql")
}

async fn apply_migrations(
    pool: &sqlx::PgPool,
    bootstrap: &str,
    migration_files: &[PathBuf],
) -> Result<(), Box<dyn Error>> {
    let mut transaction = pool.begin().await?;

    // The dedicated migrator is a member of both schema-owner roles. The
    // bootstrap switches roles explicitly so CORE grants remain owned by CORE.
    sqlx::raw_sql(bootstrap).execute(&mut *transaction).await?;
    sqlx::raw_sql(
        "SET LOCAL ROLE business_schema_owner;
         CREATE TABLE IF NOT EXISTS business.schema_migrations (
             version text PRIMARY KEY,
             applied_at timestamptz NOT NULL DEFAULT clock_timestamp()
         );",
    )
    .execute(&mut *transaction)
    .await?;

    for migration_path in migration_files {
        let version = migration_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| io::Error::other("invalid business migration filename"))?;
        let already_applied: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM business.schema_migrations WHERE version = $1)",
        )
        .bind(version)
        .fetch_one(&mut *transaction)
        .await?;
        if already_applied {
            continue;
        }

        let migration = fs::read_to_string(migration_path)?;
        sqlx::raw_sql(&migration)
            .execute(&mut *transaction)
            .await?;
        sqlx::query("INSERT INTO business.schema_migrations (version) VALUES ($1)")
            .bind(version)
            .execute(&mut *transaction)
            .await?;
    }

    transaction.commit().await?;
    Ok(())
}
