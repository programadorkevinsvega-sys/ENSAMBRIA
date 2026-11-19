use std::{error::Error, fmt, time::Duration};

use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PoolConfigurationError {
    MaxConnectionsMustBePositive,
    AcquireTimeoutMustBePositive,
    IdleTimeoutMustBePositive,
    MaxLifetimeMustBePositive,
}

impl fmt::Display for PoolConfigurationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::MaxConnectionsMustBePositive => "max_connections must be positive",
            Self::AcquireTimeoutMustBePositive => "acquire_timeout must be positive",
            Self::IdleTimeoutMustBePositive => "idle_timeout must be positive",
            Self::MaxLifetimeMustBePositive => "max_lifetime must be positive",
        };

        formatter.write_str(message)
    }
}

impl Error for PoolConfigurationError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PgPoolConfiguration {
    max_connections: u32,
    acquire_timeout: Duration,
    idle_timeout: Option<Duration>,
    max_lifetime: Option<Duration>,
}

impl PgPoolConfiguration {
    pub fn new(
        max_connections: u32,
        acquire_timeout: Duration,
        idle_timeout: Option<Duration>,
        max_lifetime: Option<Duration>,
    ) -> Result<Self, PoolConfigurationError> {
        if max_connections == 0 {
            return Err(PoolConfigurationError::MaxConnectionsMustBePositive);
        }
        if acquire_timeout.is_zero() {
            return Err(PoolConfigurationError::AcquireTimeoutMustBePositive);
        }
        if idle_timeout.is_some_and(|timeout| timeout.is_zero()) {
            return Err(PoolConfigurationError::IdleTimeoutMustBePositive);
        }
        if max_lifetime.is_some_and(|lifetime| lifetime.is_zero()) {
            return Err(PoolConfigurationError::MaxLifetimeMustBePositive);
        }

        Ok(Self {
            max_connections,
            acquire_timeout,
            idle_timeout,
            max_lifetime,
        })
    }
}

pub async fn create_pool(
    connect_options: PgConnectOptions,
    configuration: PgPoolConfiguration,
) -> Result<PgPool, PgPoolConnectError> {
    PgPoolOptions::new()
        .max_connections(configuration.max_connections)
        .acquire_timeout(configuration.acquire_timeout)
        .idle_timeout(configuration.idle_timeout)
        .max_lifetime(configuration.max_lifetime)
        .connect_with(connect_options)
        .await
        .map_err(PgPoolConnectError::from)
}

pub struct PgPoolConnectError {
    sqlstate: Option<String>,
    source: sqlx::Error,
}

impl fmt::Debug for PgPoolConnectError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PgPoolConnectError")
            .field("sqlstate", &self.sqlstate)
            .finish_non_exhaustive()
    }
}

impl fmt::Display for PgPoolConnectError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.sqlstate {
            Some(sqlstate) => write!(
                formatter,
                "PostgreSQL pool connection failed (SQLSTATE {sqlstate})"
            ),
            None => formatter.write_str("PostgreSQL pool connection failed"),
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

        Self { sqlstate, source }
    }
}

#[cfg(test)]
mod tests {
    use super::{PgPoolConfiguration, PoolConfigurationError};
    use std::time::Duration;

    #[test]
    fn rejects_zero_max_connections() {
        let result = PgPoolConfiguration::new(0, Duration::from_secs(1), None, None);

        assert_eq!(
            result,
            Err(PoolConfigurationError::MaxConnectionsMustBePositive)
        );
    }

    #[test]
    fn rejects_zero_acquire_timeout() {
        let result = PgPoolConfiguration::new(1, Duration::ZERO, None, None);

        assert_eq!(
            result,
            Err(PoolConfigurationError::AcquireTimeoutMustBePositive)
        );
    }

    #[test]
    fn rejects_zero_idle_timeout() {
        let result =
            PgPoolConfiguration::new(1, Duration::from_secs(1), Some(Duration::ZERO), None);

        assert_eq!(
            result,
            Err(PoolConfigurationError::IdleTimeoutMustBePositive)
        );
    }

    #[test]
    fn rejects_zero_max_lifetime() {
        let result =
            PgPoolConfiguration::new(1, Duration::from_secs(1), None, Some(Duration::ZERO));

        assert_eq!(
            result,
            Err(PoolConfigurationError::MaxLifetimeMustBePositive)
        );
    }

    #[test]
    fn accepts_positive_limits_without_assuming_production_values() {
        let configuration = PgPoolConfiguration::new(1, Duration::from_secs(1), None, None);

        assert!(configuration.is_ok());
    }
}
