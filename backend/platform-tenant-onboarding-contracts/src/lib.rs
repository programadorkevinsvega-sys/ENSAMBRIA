//! Public PLATFORM tenant-onboarding command and result contract.
use std::{future::Future, pin::Pin};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type ContractFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TenantCreated {
    pub event_id: Uuid,
    pub tenant_id: Uuid,
    pub product_id: Uuid,
    pub product_config_version: u32,
    pub aggregate_version: i64,
    pub correlation_id: Uuid,
    pub payload_sha256: [u8; 32],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum OnboardingOutcome {
    Applied,
    Duplicate,
    Quarantined,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OnboardingFailure {
    InvalidEvent,
    EventIdCollision,
    Unavailable,
}

/// Implemented by PLATFORM. The adapter commits Inbox and onboarding atomically.
pub trait TenantOnboarding: Send + Sync {
    fn apply_tenant_created<'a>(
        &'a self,
        event: TenantCreated,
    ) -> ContractFuture<'a, Result<OnboardingOutcome, OnboardingFailure>>;
}
