//! Public CORE identity and authentication boundary. No persistence or crypto types cross it.
use std::{future::Future, pin::Pin};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type ContractFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatedIdentity {
    /// Stable identity reference; session and credential internals remain CORE-owned.
    pub identity_id: Uuid,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthenticationFailure {
    InvalidCredential,
    ExpiredCredential,
    RevokedSession,
    Unavailable,
}

/// Implemented by CORE; consumers receive verified identity context, never token internals.
pub trait IdentityAuthentication: Send + Sync {
    fn authenticate_bearer<'a>(
        &'a self,
        bearer_token: &'a str,
    ) -> ContractFuture<'a, Result<AuthenticatedIdentity, AuthenticationFailure>>;
}
