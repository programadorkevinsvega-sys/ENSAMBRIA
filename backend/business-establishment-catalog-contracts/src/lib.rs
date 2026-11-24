//! Public BUSINESS establishment catalog boundary; contains no SQLx or storage models.
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Establishment { pub id: Uuid, pub tenant_id: Uuid, pub organization_id: Uuid, pub branch_id: Option<Uuid>, pub display_name: String, pub status: String }
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CreateEstablishment { pub tenant_id: Uuid, pub organization_id: Uuid, pub branch_id: Option<Uuid>, pub display_name: String }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EstablishmentFailure { InvalidInput, Forbidden, NotFound, Unavailable }
pub trait EstablishmentCatalog: Send + Sync {
    async fn list_for_tenant(&self, tenant_id: Uuid) -> Result<Vec<Establishment>, EstablishmentFailure>;
    async fn create(&self, input: CreateEstablishment) -> Result<Establishment, EstablishmentFailure>;
}
