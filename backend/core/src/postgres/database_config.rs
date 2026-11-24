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
            Self::InvalidDatabaseUrl => formatter.write_str("PostgreSQL database URL is invalid"),
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
        Self::from_env_with_variable_names(
            "DATABASE_URL",
            "CORE_DATABASE_MAX_CONNECTIONS",
            "CORE_DATABASE_ACQUIRE_TIMEOUT_MS",
            "CORE_DATABASE_IDLE_TIMEOUT_MS",
            "CORE_DATABASE_MAX_LIFETIME_MS",
        )
    }

    /// Loads a PostgreSQL pool using explicitly named environment variables.
    ///
    /// This lets isolated processes share one systemd EnvironmentFile while
    /// retaining distinct database credentials and pool budgets.
    pub fn from_env_with_variable_names(
        database_url_variable: &'static str,
        max_connections_variable: &'static str,
        acquire_timeout_variable: &'static str,
        idle_timeout_variable: &'static str,
        max_lifetime_variable: &'static str,
    ) -> Result<Self, PgDatabaseConfigurationError> {
        Self::from_values_with_variable_names(
            |name| match env::var(name) {
                Ok(value) => Ok(Some(value)),
                Err(env::VarError::NotPresent) => Ok(None),
                Err(env::VarError::NotUnicode(_)) => {
                    Err(PgDatabaseConfigurationError::InvalidVariable(name))
                }
            },
            database_url_variable,
            max_connections_variable,
            acquire_timeout_variable,
            idle_timeout_variable,
            max_lifetime_variable,
        )
    }

    #[cfg(test)]
    fn from_values(
        read: impl FnMut(&'static str) -> Result<Option<String>, PgDatabaseConfigurationError>,
    ) -> Result<Self, PgDatabaseConfigurationError> {
        Self::from_values_with_variable_names(
            read,
            "DATABASE_URL",
            "CORE_DATABASE_MAX_CONNECTIONS",
            "CORE_DATABASE_ACQUIRE_TIMEOUT_MS",
            "CORE_DATABASE_IDLE_TIMEOUT_MS",
            "CORE_DATABASE_MAX_LIFETIME_MS",
        )
    }

    fn from_values_with_variable_names(
        mut read: impl FnMut(&'static str) -> Result<Option<String>, PgDatabaseConfigurationError>,
        database_url_variable: &'static str,
        max_connections_variable: &'static str,
        acquire_timeout_variable: &'static str,
        idle_timeout_variable: &'static str,
        max_lifetime_variable: &'static str,
    ) -> Result<Self, PgDatabaseConfigurationError> {
        let database_url = required_value(&mut read, database_url_variable)?;
        let max_connections = parse_required::<u32>(&mut read, max_connections_variable)?;
        let acquire_timeout_ms = parse_required::<u64>(&mut read, acquire_timeout_variable)?;
        let idle_timeout_ms = parse_optional::<u64>(&mut read, idle_timeout_variable)?;
        let max_lifetime_ms = parse_optional::<u64>(&mut read, max_lifetime_variable)?;

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
    fn parses_email_worker_settings_from_its_dedicated_variables() {
        let result = PgDatabaseConfiguration::from_values_with_variable_names(
            |name| {
                let value = match name {
                    "CORE_EMAIL_WORKER_DATABASE_URL" => "postgres://worker@localhost/core",
                    "CORE_EMAIL_WORKER_DATABASE_MAX_CONNECTIONS" => "3",
                    "CORE_EMAIL_WORKER_DATABASE_ACQUIRE_TIMEOUT_MS" => "3000",
                    _ => return Ok(None),
                };
                Ok(Some(value.to_owned()))
            },
            "CORE_EMAIL_WORKER_DATABASE_URL",
            "CORE_EMAIL_WORKER_DATABASE_MAX_CONNECTIONS",
            "CORE_EMAIL_WORKER_DATABASE_ACQUIRE_TIMEOUT_MS",
            "CORE_EMAIL_WORKER_DATABASE_IDLE_TIMEOUT_MS",
            "CORE_EMAIL_WORKER_DATABASE_MAX_LIFETIME_MS",
        );

        assert!(result.is_ok());
    }

    #[test]
    fn email_worker_configuration_does_not_fall_back_to_runtime_database_url() {
        let result = PgDatabaseConfiguration::from_values_with_variable_names(
            |name| match name {
                "DATABASE_URL" => Ok(Some("postgres://runtime@localhost/core".to_owned())),
                "CORE_DATABASE_MAX_CONNECTIONS" => Ok(Some("5".to_owned())),
                "CORE_DATABASE_ACQUIRE_TIMEOUT_MS" => Ok(Some("3000".to_owned())),
                _ => Ok(None),
            },
            "CORE_EMAIL_WORKER_DATABASE_URL",
            "CORE_EMAIL_WORKER_DATABASE_MAX_CONNECTIONS",
            "CORE_EMAIL_WORKER_DATABASE_ACQUIRE_TIMEOUT_MS",
            "CORE_EMAIL_WORKER_DATABASE_IDLE_TIMEOUT_MS",
            "CORE_EMAIL_WORKER_DATABASE_MAX_LIFETIME_MS",
        );

        assert_eq!(
            result.err(),
            Some(PgDatabaseConfigurationError::MissingVariable(
                "CORE_EMAIL_WORKER_DATABASE_URL"
            ))
        );
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
