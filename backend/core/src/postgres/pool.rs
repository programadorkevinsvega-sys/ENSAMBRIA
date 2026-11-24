use std::{error::Error, fmt, time::Instant};

use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};

use super::database_config::PgDatabaseConfiguration;
use super::pool_config::PgPoolConfiguration;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PgPoolFailureKind {
    Timeout,
    Network,
    Tls,
    Configuration,
    Authentication,
    Database,
    Other,
}

impl PgPoolFailureKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Timeout => "timeout",
            Self::Network => "network",
            Self::Tls => "tls",
            Self::Configuration => "configuration",
            Self::Authentication => "authentication",
            Self::Database => "database",
            Self::Other => "other",
        }
    }
}

#[tracing::instrument(
    name = "database.pool.create",
    skip_all,
    fields(
        component = "backend-core",
        operation = "pool.create",
        dependency = "postgresql"
    )
)]
pub async fn create_pool(
    connect_options: PgConnectOptions,
    configuration: PgPoolConfiguration,
) -> Result<PgPool, PgPoolConnectError> {
    let started_at = Instant::now();
    tracing::info!(
        event = "database.dependency.call",
        component = "backend-core",
        operation = "pool.create",
        dependency = "postgresql",
        result = "started"
    );

    let result = PgPoolOptions::new()
        .max_connections(configuration.max_connections)
        .acquire_timeout(configuration.acquire_timeout)
        .idle_timeout(configuration.idle_timeout)
        .max_lifetime(configuration.max_lifetime)
        .connect_with(connect_options)
        .await;

    match result {
        Ok(pool) => {
            tracing::info!(
                event = "database.pool.create.completed",
                component = "backend-core",
                operation = "pool.create",
                dependency = "postgresql",
                result = "success",
                dependency_duration_ms = started_at.elapsed().as_millis() as u64
            );
            Ok(pool)
        }
        Err(source) => {
            let error = PgPoolConnectError::from(source);
            tracing::error!(
                event = "database.pool.create.failed",
                component = "backend-core",
                operation = "pool.create",
                dependency = "postgresql",
                result = "failure",
                failure_class = error.failure_kind.as_str(),
                sqlstate = error.sqlstate.as_deref().unwrap_or(""),
                dependency_duration_ms = started_at.elapsed().as_millis() as u64
            );
            Err(error)
        }
    }
}

pub fn create_pool_lazy(
    connect_options: PgConnectOptions,
    configuration: PgPoolConfiguration,
) -> PgPool {
    tracing::info!(
        event = "database.pool.create.lazy",
        component = "backend-core",
        operation = "pool.create",
        dependency = "postgresql",
        result = "success"
    );

    PgPoolOptions::new()
        .max_connections(configuration.max_connections)
        .acquire_timeout(configuration.acquire_timeout)
        .idle_timeout(configuration.idle_timeout)
        .max_lifetime(configuration.max_lifetime)
        .connect_lazy_with(connect_options)
}

impl PgDatabaseConfiguration {
    pub async fn connect(self) -> Result<PgPool, PgPoolConnectError> {
        create_pool(self.connect_options, self.pool).await
    }

    pub fn connect_lazy(self) -> PgPool {
        create_pool_lazy(self.connect_options, self.pool)
    }
}

pub struct PgPoolConnectError {
    failure_kind: PgPoolFailureKind,
    sqlstate: Option<String>,
    source: sqlx::Error,
}

impl PgPoolConnectError {
    pub fn failure_kind(&self) -> PgPoolFailureKind {
        self.failure_kind
    }

    pub fn sqlstate(&self) -> Option<&str> {
        self.sqlstate.as_deref()
    }
}

impl fmt::Debug for PgPoolConnectError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PgPoolConnectError")
            .field("failure_kind", &self.failure_kind)
            .field("sqlstate", &self.sqlstate)
            .finish_non_exhaustive()
    }
}

impl fmt::Display for PgPoolConnectError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.sqlstate {
            Some(sqlstate) => write!(
                formatter,
                "PostgreSQL pool connection failed ({}) with SQLSTATE {sqlstate}",
                self.failure_kind.as_str()
            ),
            None => write!(
                formatter,
                "PostgreSQL pool connection failed ({})",
                self.failure_kind.as_str()
            ),
        }
    }
}

impl Error for PgPoolConnectError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl From<sqlx::Error> for PgPoolConnectError {
    fn from(source: sqlx::Error) -> Self {
        let sqlstate = match &source {
            sqlx::Error::Database(database_error) => {
                database_error.code().map(|code| code.into_owned())
            }
            _ => None,
        };
        let failure_kind = match &source {
            sqlx::Error::PoolTimedOut => PgPoolFailureKind::Timeout,
            sqlx::Error::Io(error) if error.kind() == std::io::ErrorKind::TimedOut => {
                PgPoolFailureKind::Timeout
            }
            sqlx::Error::Io(_) => PgPoolFailureKind::Network,
            sqlx::Error::Tls(_) => PgPoolFailureKind::Tls,
            sqlx::Error::Configuration(_) => PgPoolFailureKind::Configuration,
            sqlx::Error::Database(_)
                if sqlstate
                    .as_deref()
                    .is_some_and(|code| code.starts_with("28")) =>
            {
                PgPoolFailureKind::Authentication
            }
            sqlx::Error::Database(_) => PgPoolFailureKind::Database,
            _ => PgPoolFailureKind::Other,
        };

        Self {
            failure_kind,
            sqlstate,
            source,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PgPoolConnectError, PgPoolFailureKind};

    #[test]
    fn connection_error_debug_and_display_do_not_expose_source_messages() {
        let source = sqlx::Error::Configuration(Box::new(std::io::Error::other(
            "private-marker-that-must-not-be-logged",
        )));
        let error = PgPoolConnectError::from(source);

        assert_eq!(error.failure_kind(), PgPoolFailureKind::Configuration);
        assert!(
            !error
                .to_string()
                .contains("private-marker-that-must-not-be-logged")
        );
        assert!(!format!("{error:?}").contains("private-marker-that-must-not-be-logged"));
    }
}
