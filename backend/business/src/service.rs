use std::time::Duration;

use ensambria_shared_cache::TtlCache;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

const ESTABLISHMENT_LIST_TTL: Duration = Duration::from_secs(10);

#[derive(Clone)]
pub struct BusinessService {
    business_pool: PgPool,
    cache: Option<TtlCache>,
}

impl BusinessService {
    pub fn new(business_pool: PgPool, cache: Option<TtlCache>) -> Self {
        Self {
            business_pool,
            cache,
        }
    }

    pub async fn list_establishments(
        &self,
        identity_id: Uuid,
        tenant_id: Uuid,
    ) -> Result<Vec<Establishment>, BusinessError> {
        let mut transaction = self
            .business_pool
            .begin()
            .await
            .map_err(|_| BusinessError::Storage)?;
        set_tenant(&mut transaction, tenant_id).await?;
        require_active_membership(&mut transaction, identity_id, tenant_id).await?;
        let cache_key = format!("tenant:{tenant_id}:establishments:v1");
        if let Some(cache) = &self.cache
            && let Ok(Some(value)) = cache.get(&cache_key).await
            && let Ok(items) = serde_json::from_slice::<Vec<Establishment>>(&value)
        {
            transaction
                .commit()
                .await
                .map_err(|_| BusinessError::Storage)?;
            return Ok(items);
        }

        let rows = sqlx::query_as::<_, Establishment>(
            "SELECT id, tenant_id, organization_id, branch_id, display_name, status, created_at::text AS created_at FROM business.establishments WHERE tenant_id = $1 ORDER BY created_at DESC, id DESC LIMIT 200"
        )
        .bind(tenant_id)
        .fetch_all(&mut *transaction)
        .await
        .map_err(|_| BusinessError::Storage)?;
        transaction
            .commit()
            .await
            .map_err(|_| BusinessError::Storage)?;

        if let (Some(cache), Ok(value)) = (&self.cache, serde_json::to_vec(&rows)) {
            let _ = cache.put(&cache_key, &value, ESTABLISHMENT_LIST_TTL).await;
        }
        Ok(rows)
    }

    pub async fn create_establishment(
        &self,
        identity_id: Uuid,
        tenant_id: Uuid,
        organization_id: Uuid,
        branch_id: Option<Uuid>,
        display_name: String,
    ) -> Result<Establishment, BusinessError> {
        let display_name = display_name.trim().to_owned();
        if display_name.is_empty() || display_name.chars().count() > 200 {
            return Err(BusinessError::InvalidInput);
        }

        let mut transaction = self
            .business_pool
            .begin()
            .await
            .map_err(|_| BusinessError::Storage)?;
        set_tenant(&mut transaction, tenant_id).await?;
        require_active_membership(&mut transaction, identity_id, tenant_id).await?;
        let organization_active: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM core.organizations WHERE tenant_id = $1 AND id = $2 AND status = 'active')"
        )
        .bind(tenant_id)
        .bind(organization_id)
        .fetch_one(&mut *transaction)
        .await
        .map_err(|_| BusinessError::Storage)?;
        if !organization_active {
            return Err(BusinessError::OrganizationUnavailable);
        }
        if let Some(branch_id) = branch_id {
            let branch_active: bool = sqlx::query_scalar(
                "SELECT EXISTS (SELECT 1 FROM core.branches WHERE tenant_id = $1 AND organization_id = $2 AND id = $3 AND status = 'active')"
            )
            .bind(tenant_id)
            .bind(organization_id)
            .bind(branch_id)
            .fetch_one(&mut *transaction)
            .await
            .map_err(|_| BusinessError::Storage)?;
            if !branch_active {
                return Err(BusinessError::OrganizationUnavailable);
            }
        }
        let row = sqlx::query_as::<_, Establishment>(
            "INSERT INTO business.establishments (id, tenant_id, organization_id, branch_id, display_name, status) VALUES ($1, $2, $3, $4, $5, 'active') RETURNING id, tenant_id, organization_id, branch_id, display_name, status, created_at::text AS created_at"
        )
        .bind(Uuid::now_v7())
        .bind(tenant_id)
        .bind(organization_id)
        .bind(branch_id)
        .bind(display_name)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|_| BusinessError::Storage)?
        .ok_or(BusinessError::OrganizationUnavailable)?;
        transaction
            .commit()
            .await
            .map_err(|_| BusinessError::Storage)?;

        if let Some(cache) = &self.cache {
            let _ = cache
                .delete(&format!("tenant:{tenant_id}:establishments:v1"))
                .await;
        }
        Ok(row)
    }
}

async fn require_active_membership(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    identity_id: Uuid,
    tenant_id: Uuid,
) -> Result<(), BusinessError> {
    let membership: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM core.tenant_memberships WHERE tenant_id = $1 AND identity_id = $2 AND status = 'active' FOR SHARE"
    )
    .bind(tenant_id)
    .bind(identity_id)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|_| BusinessError::Storage)?;
    membership.map(|_| ()).ok_or(BusinessError::Forbidden)
}

async fn set_tenant(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    tenant_id: Uuid,
) -> Result<(), BusinessError> {
    sqlx::query("SELECT set_config('app.tenant_id', $1, true)")
        .bind(tenant_id.to_string())
        .execute(&mut **transaction)
        .await
        .map_err(|_| BusinessError::Storage)?;
    Ok(())
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Establishment {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub organization_id: Uuid,
    pub branch_id: Option<Uuid>,
    pub display_name: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BusinessError {
    Forbidden,
    InvalidInput,
    OrganizationUnavailable,
    Storage,
}

impl std::fmt::Display for BusinessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Forbidden => "active tenant membership is required",
            Self::InvalidInput => "establishment input is invalid",
            Self::OrganizationUnavailable => "organization or branch is not active",
            Self::Storage => "BUSINESS persistence operation failed",
        })
    }
}
impl std::error::Error for BusinessError {}
