use std::{
    env,
    error::Error,
    fmt,
    path::{Path, PathBuf},
    str::FromStr,
    time::Instant,
};

use sqlx::{
    Connection,
    postgres::{PgConnectOptions, PgConnection},
};
use tracing::Instrument;
use uuid::Uuid;

const CORE_MIGRATION_LOCK_NAMESPACE: i32 = 0x454E5341;
const CORE_MIGRATION_LOCK_RESOURCE: i32 = 1;

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
    missing_roles: Option<String>,
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
            missing_roles: None,
        }
    }

    pub fn failure_kind(&self) -> CoreMigrationFailureKind {
        self.failure_kind
    }

    fn missing_roles(missing_roles: Vec<String>) -> Self {
        Self {
            failure_kind: CoreMigrationFailureKind::OwnerRole,
            message: "CORE required database roles are missing",
            source: None,
            missing_roles: Some(missing_roles.join(":")),
        }
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
    let migrations_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("migrations");
    run_core_migrations_from_path(database_url, &migrations_path).await
}

pub async fn run_core_migrations_from_path(
    database_url: &str,
    migrations_path: &Path,
) -> Result<(), CoreMigrationError> {
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

        let owner_roles = [
            ("core_identity_auth_schema_owner", "core_identity_auth"),
            ("core_tenancy_schema_owner", "core_tenancy"),
        ];
        for (owner_role, schema_name) in owner_roles {
            let (
                owner_can_login,
                owner_is_superuser,
                owner_can_create_database,
                owner_can_create_roles,
                owner_bypasses_rls,
                owner_can_create_schema,
            ): (bool, bool, bool, bool, bool, bool) = sqlx::query_as(
                "SELECT rolcanlogin, rolsuper, rolcreatedb, rolcreaterole, rolbypassrls, \
                        has_schema_privilege(rolname, $1, 'CREATE') \
                 FROM pg_catalog.pg_roles WHERE rolname = $2",
            )
            .bind(schema_name)
            .bind(owner_role)
            .fetch_one(&mut connection)
            .await
            .map_err(|source| {
                CoreMigrationError::new(
                    CoreMigrationFailureKind::OwnerRole,
                    "CORE module schema-owner privileges could not be verified",
                    Some(Box::new(source)),
                )
            })?;

            if owner_can_login
                || owner_is_superuser
                || owner_can_create_database
                || owner_can_create_roles
                || owner_bypasses_rls
                || !owner_can_create_schema
            {
                return Err(CoreMigrationError::new(
                    CoreMigrationFailureKind::OwnerRole,
                    "CORE module schema-owner role has invalid privileges",
                    None,
                ));
            }
        }

        let schema_owners: Vec<(String, String)> = sqlx::query_as(
            "SELECT n.nspname::text, pg_catalog.pg_get_userbyid(n.nspowner)::text \
             FROM pg_catalog.pg_namespace n \
             WHERE n.nspname IN ('core_identity_auth', 'core_tenancy') \
             ORDER BY n.nspname",
        )
        .fetch_all(&mut connection)
        .await
        .map_err(|source| {
            CoreMigrationError::new(
                CoreMigrationFailureKind::OwnerRole,
                "CORE module schema ownership could not be verified",
                Some(Box::new(source)),
            )
        })?;
        if schema_owners
            != vec![
                (
                    "core_identity_auth".to_owned(),
                    "core_identity_auth_schema_owner".to_owned(),
                ),
                (
                    "core_tenancy".to_owned(),
                    "core_tenancy_schema_owner".to_owned(),
                ),
            ]
        {
            return Err(CoreMigrationError::new(
                CoreMigrationFailureKind::OwnerRole,
                "CORE module schemas must be owned by their dedicated non-login roles",
                None,
            ));
        }

        let (
            migrator_can_assume_identity_owner,
            migrator_can_assume_tenancy_owner,
            migrator_can_assume_runtime,
        ): (bool, bool, bool) = sqlx::query_as(
            "SELECT pg_has_role('core_migrator', 'core_identity_auth_schema_owner', 'MEMBER'), \
                    pg_has_role('core_migrator', 'core_tenancy_schema_owner', 'MEMBER'), \
                    pg_has_role('core_migrator', 'core_identity_auth_runtime', 'MEMBER') \
                        OR pg_has_role('core_migrator', 'core_tenancy_runtime', 'MEMBER')",
        )
        .fetch_one(&mut connection)
        .await
        .map_err(|source| {
            CoreMigrationError::new(
                CoreMigrationFailureKind::OwnerRole,
                "CORE migration role memberships could not be verified",
                Some(Box::new(source)),
            )
        })?;
        if !migrator_can_assume_identity_owner
            || !migrator_can_assume_tenancy_owner
            || migrator_can_assume_runtime
        {
            return Err(CoreMigrationError::new(
                CoreMigrationFailureKind::OwnerRole,
                "CORE migrator must assume only the two module schema-owner roles",
                None,
            ));
        }

        let missing_roles: Vec<String> = sqlx::query_scalar(
            "SELECT required.role_name \
             FROM unnest(ARRAY[ \
                 'core_identity_auth_runtime', \
                 'core_tenancy_runtime', \
                 'core_email_worker', \
                 'core_outbox_relay' \
             ]::text[]) AS required(role_name) \
             LEFT JOIN pg_catalog.pg_roles existing ON existing.rolname = required.role_name \
             WHERE existing.rolname IS NULL \
             ORDER BY required.role_name",
        )
        .fetch_all(&mut connection)
        .await
        .map_err(|source| {
            CoreMigrationError::new(
                CoreMigrationFailureKind::OwnerRole,
                "CORE required database roles could not be verified",
                Some(Box::new(source)),
            )
        })?;
        if !missing_roles.is_empty() {
            return Err(CoreMigrationError::missing_roles(missing_roles));
        }

        sqlx::query("SET ROLE core_tenancy_schema_owner")
            .execute(&mut connection)
            .await
            .map_err(|source| {
                CoreMigrationError::new(
                    CoreMigrationFailureKind::OwnerRole,
                    "CORE migration identity cannot assume the tenancy schema owner role",
                    Some(Box::new(source)),
                )
            })?;

        let mut migrator = sqlx::migrate::Migrator::new(migrations_path)
            .await
            .map_err(|source| {
                CoreMigrationError::new(
                    CoreMigrationFailureKind::Migration,
                    "CORE migration files could not be loaded",
                    Some(Box::new(source)),
                )
            })?;
        migrator.dangerous_set_table_name("core_tenancy._sqlx_migrations");

        // Serialize all migrators, including concurrent deploys and test processes.
        sqlx::query("SELECT pg_advisory_lock($1, $2)")
            .bind(CORE_MIGRATION_LOCK_NAMESPACE)
            .bind(CORE_MIGRATION_LOCK_RESOURCE)
            .execute(&mut connection)
            .await
            .map_err(|source| {
                CoreMigrationError::new(
                    CoreMigrationFailureKind::Migration,
                    "CORE migration lock could not be acquired",
                    Some(Box::new(source)),
                )
            })?;

        let migration_result = migrator.run(&mut connection).await;
        let lock_released: bool = sqlx::query_scalar("SELECT pg_advisory_unlock($1, $2)")
            .bind(CORE_MIGRATION_LOCK_NAMESPACE)
            .bind(CORE_MIGRATION_LOCK_RESOURCE)
            .fetch_one(&mut connection)
            .await
            .map_err(|source| {
                CoreMigrationError::new(
                    CoreMigrationFailureKind::Connection,
                    "CORE migration lock could not be released",
                    Some(Box::new(source)),
                )
            })?;
        if !lock_released {
            return Err(CoreMigrationError::new(
                CoreMigrationFailureKind::Connection,
                "CORE migration lock ownership was lost",
                None,
            ));
        }
        migration_result.map_err(|source| {
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
                missing_roles = error.missing_roles.as_deref().unwrap_or(""),
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
