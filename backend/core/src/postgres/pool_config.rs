use std::{error::Error, fmt, time::Duration};

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
    pub(super) max_connections: u32,
    pub(super) acquire_timeout: Duration,
    pub(super) idle_timeout: Option<Duration>,
    pub(super) max_lifetime: Option<Duration>,
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
