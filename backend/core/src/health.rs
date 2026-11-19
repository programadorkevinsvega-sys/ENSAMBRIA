use std::time::{Duration, Instant};

use sqlx::PgPool;
use tokio::{
    sync::watch,
    time::{MissedTickBehavior, interval},
};
use tonic_health::{ServingStatus, server::HealthReporter};
use tracing::Instrument;
use uuid::Uuid;

use crate::observability::record_database_health_check;

pub const LIVENESS_SERVICE: &str = "ensambria.core.v1.Liveness";
pub const READINESS_SERVICE: &str = "ensambria.core.v1.Readiness";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DatabaseHealthFailureKind {
    Dependency,
    Timeout,
}

impl DatabaseHealthFailureKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Dependency => "dependency",
            Self::Timeout => "timeout",
        }
    }
}

pub struct DatabaseHealthCheckError {
    failure_kind: DatabaseHealthFailureKind,
    sqlstate: Option<String>,
    source: Option<sqlx::Error>,
}

impl std::fmt::Debug for DatabaseHealthCheckError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DatabaseHealthCheckError")
            .field("failure_kind", &self.failure_kind)
            .field("sqlstate", &self.sqlstate)
            .finish_non_exhaustive()
    }
}

impl std::fmt::Display for DatabaseHealthCheckError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.sqlstate {
            Some(sqlstate) => write!(
                formatter,
                "PostgreSQL readiness check failed ({}, SQLSTATE {sqlstate})",
                self.failure_kind.as_str()
            ),
            None => write!(
                formatter,
                "PostgreSQL readiness check failed ({})",
                self.failure_kind.as_str()
            ),
        }
    }
}

impl std::error::Error for DatabaseHealthCheckError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| source as &(dyn std::error::Error + 'static))
    }
}

impl DatabaseHealthCheckError {
    fn from_source(source: sqlx::Error) -> Self {
        let sqlstate = match &source {
            sqlx::Error::Database(database_error) => {
                database_error.code().map(|code| code.into_owned())
            }
            _ => None,
        };

        Self {
            failure_kind: DatabaseHealthFailureKind::Dependency,
            sqlstate,
            source: Some(source),
        }
    }

    fn timeout() -> Self {
        Self {
            failure_kind: DatabaseHealthFailureKind::Timeout,
            sqlstate: None,
            source: None,
        }
    }
}

pub async fn check_database(
    pool: &PgPool,
    check_timeout: Duration,
) -> Result<(), DatabaseHealthCheckError> {
    match tokio::time::timeout(check_timeout, sqlx::query("SELECT 1").execute(pool)).await {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(source)) => Err(DatabaseHealthCheckError::from_source(source)),
        Err(_) => Err(DatabaseHealthCheckError::timeout()),
    }
}

pub async fn initialize_health_status(reporter: &HealthReporter) {
    reporter
        .set_service_status(LIVENESS_SERVICE, ServingStatus::Serving)
        .await;
    reporter
        .set_service_status(READINESS_SERVICE, ServingStatus::NotServing)
        .await;
    reporter
        .set_service_status("", ServingStatus::NotServing)
        .await;
}

pub async fn monitor_database_readiness(
    pool: PgPool,
    reporter: HealthReporter,
    check_interval: Duration,
    check_timeout: Duration,
    mut shutdown: watch::Receiver<bool>,
) {
    let mut timer = interval(check_interval);
    timer.set_missed_tick_behavior(MissedTickBehavior::Delay);
    let mut last_readiness = None;

    loop {
        tokio::select! {
            _ = timer.tick() => {
                let started_at = Instant::now();
                let correlation_id = Uuid::now_v7();
                let trace_id = Uuid::now_v7();
                let span = tracing::info_span!(
                    "database.readiness",
                    component = "backend-core",
                    operation = "readiness.check",
                    dependency = "postgresql",
                    correlation_id = %correlation_id,
                    trace_id = %trace_id.simple()
                );
                match check_database(&pool, check_timeout).instrument(span).await {
                    Ok(()) => {
                        record_database_health_check(true, started_at.elapsed().as_secs_f64());
                        reporter.set_service_status(READINESS_SERVICE, ServingStatus::Serving).await;
                        reporter.set_service_status("", ServingStatus::Serving).await;
                        if last_readiness != Some(true) {
                            tracing::info!(
                                event = "service.readiness.changed",
                                component = "backend-core",
                                operation = "readiness.check",
                                dependency = "postgresql",
                                correlation_id = %correlation_id,
                                trace_id = %trace_id.simple(),
                                result = "ready",
                                dependency_duration_ms = started_at.elapsed().as_millis() as u64
                            );
                            last_readiness = Some(true);
                        }
                    }
                    Err(error) => {
                        record_database_health_check(false, started_at.elapsed().as_secs_f64());
                        reporter.set_service_status(READINESS_SERVICE, ServingStatus::NotServing).await;
                        reporter.set_service_status("", ServingStatus::NotServing).await;
                        tracing::error!(
                            event = "service.readiness.failed",
                            component = "backend-core",
                            operation = "readiness.check",
                            dependency = "postgresql",
                            correlation_id = %correlation_id,
                            trace_id = %trace_id.simple(),
                            result = "not_ready",
                            failure_class = error.failure_kind.as_str(),
                            sqlstate = error.sqlstate.as_deref().unwrap_or(""),
                            dependency_duration_ms = started_at.elapsed().as_millis() as u64
                        );
                        last_readiness = Some(false);
                    }
                }
            }
            changed = shutdown.changed() => {
                if changed.is_err() || *shutdown.borrow() {
                    break;
                }
            }
        }
    }
}
