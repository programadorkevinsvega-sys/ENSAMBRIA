//! Public BUSINESS establishment catalog boundary; contains no SQLx or storage models.
use std::{future::Future, pin::Pin};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type ContractFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Establishment {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub organization_id: Uuid,
    pub branch_id: Option<Uuid>,
    pub display_name: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CreateEstablishment {
    pub tenant_id: Uuid,
    pub organization_id: Uuid,
    pub branch_id: Option<Uuid>,
    pub display_name: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EstablishmentFailure {
    InvalidInput,
    Forbidden,
    NotFound,
    Unavailable,
}

pub trait EstablishmentCatalog: Send + Sync {
    fn list_for_tenant<'a>(
        &'a self,
        tenant_id: Uuid,
    ) -> ContractFuture<'a, Result<Vec<Establishment>, EstablishmentFailure>>;

    fn create<'a>(
        &'a self,
        input: CreateEstablishment,
    ) -> ContractFuture<'a, Result<Establishment, EstablishmentFailure>>;
}
