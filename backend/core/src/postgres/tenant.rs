use std::{error::Error, fmt, time::Instant};

use sqlx::{Postgres, Transaction, types::Uuid};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TenantId(Uuid);

impl TenantId {
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TenantContext {
    tenant_id: TenantId,
}

impl TenantContext {
    pub fn new(tenant_id: TenantId) -> Self {
        Self { tenant_id }
    }

    /// Call only after the owner authorizes the scope; this setting is not authorization.
    #[tracing::instrument(
        name = "database.tenant_context.establish",
        skip_all,
        fields(
            component = "backend-core",
            operation = "tenant_context.establish",
            dependency = "postgresql"
        )
    )]
    pub async fn establish(
        self,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), TenantContextError> {
        let started_at = Instant::now();
        tracing::info!(
            event = "database.dependency.call",
            component = "backend-core",
            operation = "tenant_context.establish",
            dependency = "postgresql",
            result = "started"
        );

        let result = sqlx::query("SELECT set_config('app.tenant_id', $1, true)")
            .bind(self.tenant_id.as_uuid().to_string())
            .execute(&mut **transaction)
            .await;

        match result {
            Ok(_) => {
                tracing::info!(
                    event = "database.tenant_context.established",
                    component = "backend-core",
                    operation = "tenant_context.establish",
                    dependency = "postgresql",
                    result = "success",
                    dependency_duration_ms = started_at.elapsed().as_millis() as u64
                );
                Ok(())
            }
            Err(source) => {
                let error = TenantContextError::from(source);
                tracing::error!(
                    event = "database.tenant_context.establish.failed",
                    component = "backend-core",
                    operation = "tenant_context.establish",
                    dependency = "postgresql",
                    result = "failure",
                    sqlstate = error.sqlstate.as_deref().unwrap_or(""),
                    dependency_duration_ms = started_at.elapsed().as_millis() as u64
                );
                Err(error)
            }
        }
    }
}

pub struct TenantContextError {
    sqlstate: Option<String>,
    source: sqlx::Error,
}

impl fmt::Debug for TenantContextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TenantContextError")
            .field("sqlstate", &self.sqlstate)
            .finish_non_exhaustive()
    }
}

impl fmt::Display for TenantContextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.sqlstate {
            Some(sqlstate) => write!(
                formatter,
                "failed to establish PostgreSQL tenant context (SQLSTATE {sqlstate})"
            ),
            None => formatter.write_str("failed to establish PostgreSQL tenant context"),
        }
    }
}

impl Error for TenantContextError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl From<sqlx::Error> for TenantContextError {
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
    use super::TenantContextError;

    #[test]
    fn context_error_debug_and_display_do_not_expose_source_messages() {
        let source = sqlx::Error::Configuration(Box::new(std::io::Error::other(
            "private-marker-that-must-not-be-logged",
        )));
        let error = TenantContextError::from(source);

        assert!(
            !error
                .to_string()
                .contains("private-marker-that-must-not-be-logged")
        );
        assert!(!format!("{error:?}").contains("private-marker-that-must-not-be-logged"));
    }
}
