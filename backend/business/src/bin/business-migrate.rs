use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

use sqlx::postgres::PgPoolOptions;

const BOOTSTRAP: &str = "bootstrap/00000000000000_business_schema.sql";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .json()
        .try_init()?;
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

    let pool = match PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
    {
        Ok(pool) => pool,
        Err(error) => {
            let sqlstate = match &error {
                sqlx::Error::Database(database_error) => {
                    database_error.code().unwrap_or_default().into_owned()
                }
                _ => String::new(),
            };
            tracing::error!(
                event = "database.business_migrations.connection_failed",
                component = "backend-business",
                operation = "database.connect",
                failure_class = "database_connection",
                sqlstate = %sqlstate,
                result = "failure"
            );
            return Err(io::Error::other("BUSINESS migration database connection failed").into());
        }
    };
    let result = apply_migrations(&pool, &bootstrap, &migration_files).await;
    pool.close().await;
    if let Err(error) = result {
        let sqlstate = error
            .downcast_ref::<sqlx::Error>()
            .and_then(|database_error| match database_error {
                sqlx::Error::Database(database_error) => {
                    database_error.code().map(|code| code.into_owned())
                }
                _ => None,
            })
            .unwrap_or_default();
        let failure_class = match sqlstate.as_str() {
            "42501" | "42704" | "42P01" => "database_role_or_permission",
            _ if sqlstate.is_empty() => "migration_runtime",
            _ => "database_migration",
        };
        tracing::error!(
            event = "database.business_migrations.failed",
            component = "backend-business",
            operation = "database.migrations.apply",
            failure_class,
            sqlstate = %sqlstate,
            result = "failure"
        );
        return Err(io::Error::other("BUSINESS database migration failed").into());
    }
    tracing::info!(
        event = "database.business_migrations.completed",
        component = "backend-business",
        operation = "database.migrations.apply",
        result = "success"
    );
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
    preflight_roles(pool).await?;

    let mut transaction = pool.begin().await?;

    // The dedicated migrator is a member of the BUSINESS schema-owner role.
    // CORE access is performed through CORE's public tenancy contract.
    sqlx::raw_sql(sqlx::AssertSqlSafe(bootstrap))
        .execute(&mut *transaction)
        .await?;
    sqlx::raw_sql(
        "SET LOCAL ROLE business_establishment_catalog_schema_owner;
         CREATE TABLE IF NOT EXISTS business_establishment_catalog.schema_migrations (
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
            "SELECT EXISTS (SELECT 1 FROM business_establishment_catalog.schema_migrations WHERE version = $1)",
        )
        .bind(version)
        .fetch_one(&mut *transaction)
        .await?;
        if already_applied {
            continue;
        }

        let migration = fs::read_to_string(migration_path)?;
        sqlx::raw_sql(sqlx::AssertSqlSafe(&*migration))
            .execute(&mut *transaction)
            .await?;
        sqlx::query(
            "INSERT INTO business_establishment_catalog.schema_migrations (version) VALUES ($1)",
        )
        .bind(version)
        .execute(&mut *transaction)
        .await?;
    }

    transaction.commit().await?;
    Ok(())
}

async fn preflight_roles(pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let missing_roles: Vec<String> = sqlx::query_scalar(
        "SELECT required.role_name
         FROM (VALUES ('business_establishment_catalog_schema_owner')) AS required(role_name)
         LEFT JOIN pg_roles role ON role.rolname = required.role_name
         WHERE role.oid IS NULL OR NOT pg_has_role(current_user, role.oid, 'MEMBER')
         ORDER BY required.role_name",
    )
    .fetch_all(pool)
    .await?;

    if !missing_roles.is_empty() {
        tracing::error!(
            event = "database.business_migrations.preflight_failed",
            component = "backend-business",
            operation = "database.roles.verify",
            failure_class = "missing_or_ungranted_owner_role",
            missing_roles = %missing_roles.join(","),
            result = "failure"
        );
        return Err(io::Error::other(
            "required BUSINESS schema owner roles are missing or not granted",
        )
        .into());
    }

    let unsafe_owner_roles: Vec<String> = sqlx::query_scalar(
        "SELECT rolname
         FROM pg_roles
         WHERE rolname IN ('business_establishment_catalog_schema_owner')
           AND (rolcanlogin OR rolsuper OR rolcreatedb OR rolcreaterole OR rolbypassrls)
         ORDER BY rolname",
    )
    .fetch_all(pool)
    .await?;

    if !unsafe_owner_roles.is_empty() {
        tracing::error!(
            event = "database.business_migrations.preflight_failed",
            component = "backend-business",
            operation = "database.roles.verify",
            failure_class = "schema_owner_role_overprivileged",
            missing_roles = %unsafe_owner_roles.join(","),
            result = "failure"
        );
        return Err(
            io::Error::other("schema owner roles must be NOLOGIN and non-privileged").into(),
        );
    }

    let has_database_create: bool = sqlx::query_scalar(
        "SELECT has_database_privilege('business_establishment_catalog_schema_owner', current_database(), 'CREATE')",
    )
    .fetch_one(pool)
    .await?;
    if !has_database_create {
        tracing::error!(
            event = "database.business_migrations.preflight_failed",
            component = "backend-business",
            operation = "database.privileges.verify",
            failure_class = "business_establishment_catalog_schema_owner_missing_database_create",
            result = "failure"
        );
        return Err(io::Error::other(
            "business_establishment_catalog_schema_owner requires CREATE on the database",
        )
        .into());
    }

    Ok(())
}
