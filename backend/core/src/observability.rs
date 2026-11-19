use std::{env, error::Error, fmt, net::SocketAddr};

use metrics_exporter_prometheus::{BuildError, PrometheusBuilder};
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

#[derive(Debug)]
pub enum ObservabilityConfigurationError {
    InvalidLogFilter,
    SubscriberAlreadyInitialized,
    PrometheusExporter(BuildError),
}

impl fmt::Display for ObservabilityConfigurationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLogFilter => formatter.write_str("RUST_LOG is invalid"),
            Self::SubscriberAlreadyInitialized => {
                formatter.write_str("the tracing subscriber is already initialized")
            }
            Self::PrometheusExporter(error) => {
                write!(formatter, "Prometheus metrics exporter failed: {error}")
            }
        }
    }
}

impl Error for ObservabilityConfigurationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::PrometheusExporter(error) => Some(error),
            _ => None,
        }
    }
}

pub fn initialize_tracing() -> Result<(), ObservabilityConfigurationError> {
    let filter = match env::var("RUST_LOG") {
        Ok(value) => EnvFilter::try_new(value)
            .map_err(|_| ObservabilityConfigurationError::InvalidLogFilter)?,
        Err(env::VarError::NotPresent) => EnvFilter::new("info"),
        Err(env::VarError::NotUnicode(_)) => {
            return Err(ObservabilityConfigurationError::InvalidLogFilter);
        }
    };

    tracing_subscriber::fmt()
        .json()
        .with_env_filter(filter)
        .with_span_list(true)
        .with_span_events(FmtSpan::CLOSE)
        .try_init()
        .map_err(|_| ObservabilityConfigurationError::SubscriberAlreadyInitialized)
}

pub fn install_metrics_exporter(
    listen_address: SocketAddr,
) -> Result<(), ObservabilityConfigurationError> {
    PrometheusBuilder::new()
        .with_http_listener(listen_address)
        .install()
        .map_err(ObservabilityConfigurationError::PrometheusExporter)?;
    Ok(())
}

pub fn record_database_health_check(success: bool, duration_seconds: f64) {
    let result = if success { "success" } else { "failure" };
    metrics::counter!("ensambria_core_database_health_checks_total", "result" => result)
        .increment(1);
    metrics::histogram!("ensambria_core_database_health_check_duration_seconds")
        .record(duration_seconds);
}
