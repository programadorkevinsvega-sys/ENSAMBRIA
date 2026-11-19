use std::{env, error::Error, fmt, str::FromStr, time::Duration};

use sqlx::postgres::PgConnectOptions;

use super::pool_config::{PgPoolConfiguration, PoolConfigurationError};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PgDatabaseConfigurationError {
    MissingVariable(&'static str),
    InvalidVariable(&'static str),
    InvalidDatabaseUrl,
    InvalidPoolConfiguration(PoolConfigurationError),
}

impl fmt::Display for PgDatabaseConfigurationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingVariable(name) => {
                write!(formatter, "required environment variable {name} is missing")
            }
            Self::InvalidVariable(name) => {
                write!(formatter, "environment variable {name} is invalid")
            }
            Self::InvalidDatabaseUrl => formatter.write_str("DATABASE_URL is invalid"),
            Self::InvalidPoolConfiguration(error) => {
                write!(
                    formatter,
                    "PostgreSQL pool configuration is invalid: {error}"
                )
            }
        }
    }
}

impl Error for PgDatabaseConfigurationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidPoolConfiguration(error) => Some(error),
            _ => None,
        }
    }
}

pub struct PgDatabaseConfiguration {
    pub(super) connect_options: PgConnectOptions,
    pub(super) pool: PgPoolConfiguration,
}

impl PgDatabaseConfiguration {
    pub fn from_env() -> Result<Self, PgDatabaseConfigurationError> {
        Self::from_values(|name| match env::var(name) {
            Ok(value) => Ok(Some(value)),
            Err(env::VarError::NotPresent) => Ok(None),
            Err(env::VarError::NotUnicode(_)) => {
                Err(PgDatabaseConfigurationError::InvalidVariable(name))
            }
        })
    }

    fn from_values(
        mut read: impl FnMut(&'static str) -> Result<Option<String>, PgDatabaseConfigurationError>,
    ) -> Result<Self, PgDatabaseConfigurationError> {
        let database_url = required_value(&mut read, "DATABASE_URL")?;
        let max_connections = parse_required::<u32>(&mut read, "CORE_DATABASE_MAX_CONNECTIONS")?;
        let acquire_timeout_ms =
            parse_required::<u64>(&mut read, "CORE_DATABASE_ACQUIRE_TIMEOUT_MS")?;
        let idle_timeout_ms = parse_optional::<u64>(&mut read, "CORE_DATABASE_IDLE_TIMEOUT_MS")?;
        let max_lifetime_ms = parse_optional::<u64>(&mut read, "CORE_DATABASE_MAX_LIFETIME_MS")?;

        let pool = PgPoolConfiguration::new(
            max_connections,
            Duration::from_millis(acquire_timeout_ms),
            idle_timeout_ms.map(Duration::from_millis),
            max_lifetime_ms.map(Duration::from_millis),
        )
        .map_err(PgDatabaseConfigurationError::InvalidPoolConfiguration)?;
        let connect_options = PgConnectOptions::from_str(&database_url)
            .map_err(|_| PgDatabaseConfigurationError::InvalidDatabaseUrl)?;

        Ok(Self {
            connect_options,
            pool,
        })
    }
}

fn required_value(
    read: &mut impl FnMut(&'static str) -> Result<Option<String>, PgDatabaseConfigurationError>,
    name: &'static str,
) -> Result<String, PgDatabaseConfigurationError> {
    read(name)?.ok_or(PgDatabaseConfigurationError::MissingVariable(name))
}

fn parse_required<T: FromStr>(
    read: &mut impl FnMut(&'static str) -> Result<Option<String>, PgDatabaseConfigurationError>,
    name: &'static str,
) -> Result<T, PgDatabaseConfigurationError> {
    required_value(read, name)?
        .parse()
        .map_err(|_| PgDatabaseConfigurationError::InvalidVariable(name))
}

fn parse_optional<T: FromStr>(
    read: &mut impl FnMut(&'static str) -> Result<Option<String>, PgDatabaseConfigurationError>,
    name: &'static str,
) -> Result<Option<T>, PgDatabaseConfigurationError> {
    read(name)?
        .map(|value| {
            value
                .parse()
                .map_err(|_| PgDatabaseConfigurationError::InvalidVariable(name))
        })
        .transpose()
}

#[cfg(test)]
mod tests {
    use super::{PgDatabaseConfiguration, PgDatabaseConfigurationError};

    fn valid_values(name: &'static str) -> Result<Option<String>, PgDatabaseConfigurationError> {
        let value = match name {
            "DATABASE_URL" => "postgres://localhost/core",
            "CORE_DATABASE_MAX_CONNECTIONS" => "4",
            "CORE_DATABASE_ACQUIRE_TIMEOUT_MS" => "1500",
            _ => return Ok(None),
        };

        Ok(Some(value.to_owned()))
    }

    #[test]
    fn requires_database_and_bounded_pool_settings() {
        let result = PgDatabaseConfiguration::from_values(|_| Ok(None));

        assert_eq!(
            result.err(),
            Some(PgDatabaseConfigurationError::MissingVariable(
                "DATABASE_URL"
            ))
        );
    }

    #[test]
    fn parses_runtime_settings_without_logging_database_url() {
        let configuration = PgDatabaseConfiguration::from_values(valid_values);

        assert!(configuration.is_ok());
    }

    #[test]
    fn rejects_invalid_pool_settings_by_name_without_echoing_values() {
        let result = PgDatabaseConfiguration::from_values(|name| match name {
            "DATABASE_URL" => Ok(Some("postgres://localhost/core".to_owned())),
            "CORE_DATABASE_MAX_CONNECTIONS" => Ok(Some("not-a-number".to_owned())),
            _ => Ok(Some("1500".to_owned())),
        });

        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("the invalid pool limit must be rejected"),
        };

        assert_eq!(
            error,
            PgDatabaseConfigurationError::InvalidVariable("CORE_DATABASE_MAX_CONNECTIONS")
        );
        assert!(!error.to_string().contains("not-a-number"));
    }

    #[test]
    fn invalid_database_url_error_does_not_echo_url() {
        let result = PgDatabaseConfiguration::from_values(|name| {
            let value = match name {
                "DATABASE_URL" => "postgres://private-marker:invalid-port/core",
                "CORE_DATABASE_MAX_CONNECTIONS" => "4",
                "CORE_DATABASE_ACQUIRE_TIMEOUT_MS" => "1500",
                _ => return Ok(None),
            };
            Ok(Some(value.to_owned()))
        });

        assert_eq!(
            result.err(),
            Some(PgDatabaseConfigurationError::InvalidDatabaseUrl)
        );
    }
}
