use std::{error::Error, fmt};

use sqlx::{Postgres, Transaction, types::Uuid};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActorId(Uuid);

impl ActorId {
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActorContext {
    actor_id: ActorId,
}

impl ActorContext {
    pub fn new(actor_id: ActorId) -> Self {
        Self { actor_id }
    }

    /// Establish only after authenticating the principal; this GUC is not authorization.
    pub async fn establish(
        self,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), ActorContextError> {
        sqlx::query("SELECT set_config('app.actor_id', $1, true)")
            .bind(self.actor_id.as_uuid().to_string())
            .execute(&mut **transaction)
            .await
            .map_err(ActorContextError::from)?;
        Ok(())
    }
}

pub struct ActorContextError(sqlx::Error);

impl fmt::Debug for ActorContextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ActorContextError")
            .finish_non_exhaustive()
    }
}

impl fmt::Display for ActorContextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("failed to establish authenticated actor context")
    }
}

impl Error for ActorContextError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

impl From<sqlx::Error> for ActorContextError {
    fn from(source: sqlx::Error) -> Self {
        Self(source)
    }
}

#[cfg(test)]
mod tests {
    use super::{ActorContextError, ActorId};

    #[test]
    fn actor_context_error_does_not_expose_database_details() {
        let source = sqlx::Error::Configuration(Box::new(std::io::Error::other(
            "private-database-credential-marker",
        )));
        let error = ActorContextError::from(source);
        assert!(!error.to_string().contains("private-database-credential-marker"));
        assert!(!format!("{error:?}").contains("private-database-credential-marker"));
        let _ = ActorId::new(sqlx::types::Uuid::nil());
    }
}
