//! Public CORE identity and authentication boundary. No persistence or crypto types cross it.
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatedIdentity { pub identity_id: Uuid, pub session_id: Uuid, pub authenticated_at_unix: i64 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthenticationFailure { InvalidCredential, ExpiredCredential, RevokedSession, Unavailable }
pub trait IdentityAuthentication: Send + Sync {
    async fn authenticate_bearer(&self, bearer_token: &str) -> Result<AuthenticatedIdentity, AuthenticationFailure>;
}
