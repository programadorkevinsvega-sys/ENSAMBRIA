use std::{env, error::Error, fmt, str::FromStr, time::Instant};

use sqlx::{
    Connection,
    postgres::{PgConnectOptions, PgConnection},
};
use tracing::Instrument;
use uuid::Uuid;

#[derive(Clone, Copy)]
pub enum CoreMigrationFailureKind {
    Configuration,
    Connection,
    OwnerRole,
    Migration,
}

impl CoreMigrationFailureKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Configuration => "configuration",
            Self::Connection => "connection",
            Self::OwnerRole => "owner_role",
            Self::Migration => "migration",
        }
    }
}

pub struct CoreMigrationError {
    failure_kind: CoreMigrationFailureKind,
    message: &'static str,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl CoreMigrationError {
    fn new(
        failure_kind: CoreMigrationFailureKind,
        message: &'static str,
        source: Option<Box<dyn Error + Send + Sync>>,
    ) -> Self {
        Self {
            failure_kind,
            message,
            source,
        }
    }

    pub fn failure_kind(&self) -> CoreMigrationFailureKind {
        self.failure_kind
    }

    fn sqlstate(&self) -> Option<String> {
        let mut source = self.source();
        while let Some(error) = source {
            if let Some(sqlx::Error::Database(database_error)) = error.downcast_ref::<sqlx::Error>()
            {
                return database_error.code().map(|code| code.into_owned());
            }
            source = error.source();
        }
        None
    }
}

impl fmt::Debug for CoreMigrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CoreMigrationError")
            .field("failure_kind", &self.failure_kind.as_str())
            .field("message", &self.message)
            .finish_non_exhaustive()
    }
}

impl fmt::Display for CoreMigrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message)
    }
}

impl Error for CoreMigrationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn Error + 'static))
    }
}

pub fn migration_database_url_from_env() -> Result<String, CoreMigrationError> {
    match env::var("MIGRATIONS_DATABASE_URL") {
        Ok(database_url) => Ok(database_url),
        Err(env::VarError::NotPresent) | Err(env::VarError::NotUnicode(_)) => {
            Err(CoreMigrationError::new(
                CoreMigrationFailureKind::Configuration,
                "MIGRATIONS_DATABASE_URL is missing or invalid",
                None,
            ))
        }
    }
}

pub async fn run_core_migrations(database_url: &str) -> Result<(), CoreMigrationError> {
    let correlation_id = Uuid::now_v7();
    let trace_id = Uuid::now_v7();
    let span = tracing::info_span!(
        "database.migrations",
        component = "backend-core",
        operation = "migrations.apply",
        dependency = "postgresql",
        correlation_id = %correlation_id,
        trace_id = %trace_id.simple()
    );
    let started_at = Instant::now();

    let result = async {
        let connect_options = PgConnectOptions::from_str(database_url).map_err(|source| {
            CoreMigrationError::new(
                CoreMigrationFailureKind::Configuration,
                "MIGRATIONS_DATABASE_URL is invalid",
                Some(Box::new(source)),
            )
        })?;
        let mut connection =
            PgConnection::connect_with(&connect_options)
                .await
                .map_err(|source| {
                    CoreMigrationError::new(
                        CoreMigrationFailureKind::Connection,
                        "CORE migration database connection failed",
                        Some(Box::new(source)),
                    )
                })?;

        let (
            migration_identity,
            is_superuser,
            can_create_database,
            can_create_roles,
            can_create_schema,
        ): (String, bool, bool, bool, bool) = sqlx::query_as(
            "SELECT session_user::text, rolsuper, rolcreatedb, rolcreaterole, \
                    has_database_privilege(session_user, current_database(), 'CREATE') \
             FROM pg_catalog.pg_roles \
             WHERE rolname = session_user",
        )
        .fetch_one(&mut connection)
        .await
        .map_err(|source| {
            CoreMigrationError::new(
                CoreMigrationFailureKind::OwnerRole,
                "CORE migration identity could not be verified",
                Some(Box::new(source)),
            )
        })?;

        if migration_identity != "core_migrator"
            || is_superuser
            || can_create_database
            || can_create_roles
            || can_create_schema
        {
            return Err(CoreMigrationError::new(
                CoreMigrationFailureKind::OwnerRole,
                "CORE migration identity does not have the required restricted privileges",
                None,
            ));
        }

        sqlx::query("SET ROLE core_schema_owner")
            .execute(&mut connection)
            .await
            .map_err(|source| {
                CoreMigrationError::new(
                    CoreMigrationFailureKind::OwnerRole,
                    "CORE migration identity cannot assume the schema owner role",
                    Some(Box::new(source)),
                )
            })?;

        let (
            owner_can_login,
            owner_is_superuser,
            owner_can_create_database,
            owner_can_create_roles,
            owner_can_create_schema,
        ): (bool, bool, bool, bool, bool) = sqlx::query_as(
            "SELECT rolcanlogin, rolsuper, rolcreatedb, rolcreaterole, \
                    has_database_privilege('core_schema_owner', current_database(), 'CREATE') \
             FROM pg_catalog.pg_roles \
             WHERE rolname = 'core_schema_owner'",
        )
        .fetch_one(&mut connection)
        .await
        .map_err(|source| {
            CoreMigrationError::new(
                CoreMigrationFailureKind::OwnerRole,
                "CORE schema owner privileges could not be verified",
                Some(Box::new(source)),
            )
        })?;

        if owner_can_login
            || owner_is_superuser
            || owner_can_create_database
            || owner_can_create_roles
            || owner_can_create_schema
        {
            return Err(CoreMigrationError::new(
                CoreMigrationFailureKind::OwnerRole,
                "CORE schema owner role has privileges beyond its schema ownership",
                None,
            ));
        }

        let schema_owner: Option<String> = sqlx::query_scalar(
            "SELECT pg_catalog.pg_get_userbyid(nspowner)::text \
             FROM pg_catalog.pg_namespace \
             WHERE nspname = 'core'",
        )
        .fetch_optional(&mut connection)
        .await
        .map_err(|source| {
            CoreMigrationError::new(
                CoreMigrationFailureKind::OwnerRole,
                "CORE schema ownership could not be verified",
                Some(Box::new(source)),
            )
        })?;

        if schema_owner.as_deref() != Some("core_schema_owner") {
            return Err(CoreMigrationError::new(
                CoreMigrationFailureKind::OwnerRole,
                "CORE schema must be owned by core_schema_owner",
                None,
            ));
        }

        let mut migrator = sqlx::migrate!("./migrations");
        migrator.dangerous_set_table_name("core._sqlx_migrations");
        migrator.run(&mut connection).await.map_err(|source| {
            CoreMigrationError::new(
                CoreMigrationFailureKind::Migration,
                "CORE database migration failed",
                Some(Box::new(source)),
            )
        })?;

        connection.close().await.map_err(|source| {
            CoreMigrationError::new(
                CoreMigrationFailureKind::Connection,
                "CORE migration database connection did not close cleanly",
                Some(Box::new(source)),
            )
        })
    }
    .instrument(span.clone())
    .await;

    span.in_scope(|| {
        let duration_ms = started_at.elapsed().as_millis() as u64;
        match &result {
            Ok(()) => tracing::info!(
                event = "database.migrations.completed",
                result = "success",
                duration_ms
            ),
            Err(error) => tracing::error!(
                event = "database.migrations.failed",
                result = "failure",
                failure_class = error.failure_kind().as_str(),
                error_code = "CORE_MIGRATION_FAILED",
                sqlstate = error.sqlstate().as_deref().unwrap_or(""),
                duration_ms
            ),
        }
    });

    result
}

#[cfg(test)]
mod tests {
    use super::{CoreMigrationFailureKind, run_core_migrations};

    #[tokio::test]
    async fn invalid_database_url_error_does_not_echo_credentials() {
        let database_url = "postgres://migration_user:private-value@";
        let error = run_core_migrations(database_url)
            .await
            .expect_err("an invalid database URL must be rejected");

        assert!(matches!(
            error.failure_kind(),
            CoreMigrationFailureKind::Configuration
        ));
        assert!(!error.to_string().contains("private-value"));
        assert!(!format!("{error:?}").contains("private-value"));
    }
}
