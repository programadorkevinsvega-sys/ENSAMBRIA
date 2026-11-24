//! Public CORE tenancy boundary. IDs are opaque references owned by CORE.
use std::{future::Future, pin::Pin};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type ContractFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ActiveTenantMembership {
    pub membership_id: Uuid,
    pub tenant_id: Uuid,
    pub identity_id: Uuid,
    pub role_codes: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ActiveOrganization {
    pub organization_id: Uuid,
    pub tenant_id: Uuid,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ActiveBranch {
    pub branch_id: Uuid,
    pub tenant_id: Uuid,
    pub organization_id: Uuid,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TenancyFailure {
    NotFound,
    Forbidden,
    Unavailable,
}

/// Implemented by CORE. BUSINESS must not query CORE tables or receive its pool.
pub trait TenancyAccess: Send + Sync {
    fn require_active_membership<'a>(
        &'a self,
        identity_id: Uuid,
        tenant_id: Uuid,
    ) -> ContractFuture<'a, Result<ActiveTenantMembership, TenancyFailure>>;

    fn require_active_organization<'a>(
        &'a self,
        tenant_id: Uuid,
        organization_id: Uuid,
    ) -> ContractFuture<'a, Result<ActiveOrganization, TenancyFailure>>;

    fn require_active_branch<'a>(
        &'a self,
        tenant_id: Uuid,
        organization_id: Uuid,
        branch_id: Uuid,
    ) -> ContractFuture<'a, Result<ActiveBranch, TenancyFailure>>;
}
