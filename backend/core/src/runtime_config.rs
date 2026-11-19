use std::{env, error::Error, fmt, net::SocketAddr, str::FromStr, time::Duration};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoreRuntimeConfiguration {
    pub health_listen_address: SocketAddr,
    pub health_check_interval: Duration,
    pub health_check_timeout: Duration,
    pub metrics_listen_address: SocketAddr,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoreRuntimeConfigurationError {
    MissingVariable(&'static str),
    InvalidVariable(&'static str),
    AddressMustBeLoopback(&'static str),
    HealthCheckIntervalMustBePositive,
    HealthCheckTimeoutMustBePositive,
}

impl fmt::Display for CoreRuntimeConfigurationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingVariable(name) => {
                write!(formatter, "required environment variable {name} is missing")
            }
            Self::InvalidVariable(name) => {
                write!(formatter, "environment variable {name} is invalid")
            }
            Self::AddressMustBeLoopback(name) => {
                write!(
                    formatter,
                    "environment variable {name} must use a loopback address"
                )
            }
            Self::HealthCheckIntervalMustBePositive => {
                formatter.write_str("CORE_HEALTH_CHECK_INTERVAL_MS must be positive")
            }
            Self::HealthCheckTimeoutMustBePositive => {
                formatter.write_str("CORE_HEALTH_CHECK_TIMEOUT_MS must be positive")
            }
        }
    }
}

impl Error for CoreRuntimeConfigurationError {}

impl CoreRuntimeConfiguration {
    pub fn from_env() -> Result<Self, CoreRuntimeConfigurationError> {
        let health_listen_address = parse_loopback_address("CORE_HEALTH_LISTEN_ADDRESS")?;
        let metrics_listen_address = parse_loopback_address("CORE_METRICS_LISTEN_ADDRESS")?;
        let health_check_interval_ms = parse_required::<u64>("CORE_HEALTH_CHECK_INTERVAL_MS")?;
        let health_check_timeout_ms = parse_required::<u64>("CORE_HEALTH_CHECK_TIMEOUT_MS")?;

        if health_check_interval_ms == 0 {
            return Err(CoreRuntimeConfigurationError::HealthCheckIntervalMustBePositive);
        }
        if health_check_timeout_ms == 0 {
            return Err(CoreRuntimeConfigurationError::HealthCheckTimeoutMustBePositive);
        }

        Ok(Self {
            health_listen_address,
            health_check_interval: Duration::from_millis(health_check_interval_ms),
            health_check_timeout: Duration::from_millis(health_check_timeout_ms),
            metrics_listen_address,
        })
    }
}

fn parse_loopback_address(name: &'static str) -> Result<SocketAddr, CoreRuntimeConfigurationError> {
    let value = env::var(name).map_err(|error| match error {
        env::VarError::NotPresent => CoreRuntimeConfigurationError::MissingVariable(name),
        env::VarError::NotUnicode(_) => CoreRuntimeConfigurationError::InvalidVariable(name),
    })?;
    parse_loopback_address_value(name, &value)
}

fn parse_loopback_address_value(
    name: &'static str,
    value: &str,
) -> Result<SocketAddr, CoreRuntimeConfigurationError> {
    let address = SocketAddr::from_str(value)
        .map_err(|_| CoreRuntimeConfigurationError::InvalidVariable(name))?;
    if !address.ip().is_loopback() {
        return Err(CoreRuntimeConfigurationError::AddressMustBeLoopback(name));
    }
    if address.port() == 0 {
        return Err(CoreRuntimeConfigurationError::InvalidVariable(name));
    }

    Ok(address)
}

fn parse_required<T: FromStr>(name: &'static str) -> Result<T, CoreRuntimeConfigurationError> {
    let value = env::var(name).map_err(|error| match error {
        env::VarError::NotPresent => CoreRuntimeConfigurationError::MissingVariable(name),
        env::VarError::NotUnicode(_) => CoreRuntimeConfigurationError::InvalidVariable(name),
    })?;

    value
        .parse()
        .map_err(|_| CoreRuntimeConfigurationError::InvalidVariable(name))
}

#[cfg(test)]
mod tests {
    use super::{CoreRuntimeConfigurationError, parse_loopback_address_value};

    #[test]
    fn rejects_non_loopback_listener_addresses() {
        let result = parse_loopback_address_value("CORE_HEALTH_LISTEN_ADDRESS", "0.0.0.0:50051");

        assert!(matches!(
            result,
            Err(CoreRuntimeConfigurationError::AddressMustBeLoopback(
                "CORE_HEALTH_LISTEN_ADDRESS"
            ))
        ));
    }

    #[test]
    fn rejects_ephemeral_runtime_listener_ports() {
        let result = parse_loopback_address_value("CORE_HEALTH_LISTEN_ADDRESS", "127.0.0.1:0");

        assert_eq!(
            result,
            Err(CoreRuntimeConfigurationError::InvalidVariable(
                "CORE_HEALTH_LISTEN_ADDRESS"
            ))
        );
    }
}
