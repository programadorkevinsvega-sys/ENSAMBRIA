use ensambria_core_tenancy_contracts::{
    ActiveBranch, ActiveOrganization, ActiveTenantMembership, ContractFuture, TenancyAccess,
    TenancyFailure,
};
use sqlx::PgPool;
use uuid::Uuid;

/// CORE-owned adapter for its public tenancy port. Its database pool never crosses the boundary.
#[derive(Clone)]
pub struct CoreTenancyAccess {
    pool: PgPool,
}

impl CoreTenancyAccess {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl TenancyAccess for CoreTenancyAccess {
    fn require_active_membership<'a>(
        &'a self,
        identity_id: Uuid,
        tenant_id: Uuid,
    ) -> ContractFuture<'a, Result<ActiveTenantMembership, TenancyFailure>> {
        Box::pin(async move {
            let mut transaction = self
                .pool
                .begin()
                .await
                .map_err(|_| TenancyFailure::Unavailable)?;
            sqlx::query("SELECT set_config('app.tenant_id', $1, true)")
                .bind(tenant_id.to_string())
                .execute(&mut *transaction)
                .await
                .map_err(|_| TenancyFailure::Unavailable)?;
            let row = sqlx::query_as::<_, (Uuid, Uuid, Uuid, Vec<String>)>(
                r#"
                SELECT
                    m.id,
                    m.tenant_id,
                    m.identity_id,
                    COALESCE(
                        array_agg(DISTINCT r.code) FILTER (WHERE r.code IS NOT NULL),
                        ARRAY[]::text[]
                    )
                FROM core.tenant_memberships m
                LEFT JOIN core.membership_role_assignments a
                    ON a.tenant_id = m.tenant_id
                   AND a.membership_id = m.id
                LEFT JOIN core.tenant_roles r
                    ON r.tenant_id = a.tenant_id
                   AND r.id = a.role_id
                WHERE m.tenant_id = $1
                  AND m.identity_id = $2
                  AND m.status = 'active'
                GROUP BY m.id, m.tenant_id, m.identity_id
                "#,
            )
            .bind(tenant_id)
            .bind(identity_id)
            .fetch_optional(&mut *transaction)
            .await
            .map_err(|_| TenancyFailure::Unavailable)?;
            transaction
                .commit()
                .await
                .map_err(|_| TenancyFailure::Unavailable)?;
            row.map(
                |(membership_id, tenant_id, identity_id, role_codes)| ActiveTenantMembership {
                    membership_id,
                    tenant_id,
                    identity_id,
                    role_codes,
                },
            )
            .ok_or(TenancyFailure::Forbidden)
        })
    }

    fn require_active_organization<'a>(
        &'a self,
        tenant_id: Uuid,
        organization_id: Uuid,
    ) -> ContractFuture<'a, Result<ActiveOrganization, TenancyFailure>> {
        Box::pin(async move {
            let mut transaction = self
                .pool
                .begin()
                .await
                .map_err(|_| TenancyFailure::Unavailable)?;
            sqlx::query("SELECT set_config('app.tenant_id', $1, true)")
                .bind(tenant_id.to_string())
                .execute(&mut *transaction)
                .await
                .map_err(|_| TenancyFailure::Unavailable)?;
            let row = sqlx::query_as::<_, (Uuid, Uuid)>(
                r#"
                SELECT id, tenant_id
                FROM core.organizations
                WHERE tenant_id = $1
                  AND id = $2
                  AND status = 'active'
                "#,
            )
            .bind(tenant_id)
            .bind(organization_id)
            .fetch_optional(&mut *transaction)
            .await
            .map_err(|_| TenancyFailure::Unavailable)?;
            transaction
                .commit()
                .await
                .map_err(|_| TenancyFailure::Unavailable)?;
            row.map(|(organization_id, tenant_id)| ActiveOrganization {
                organization_id,
                tenant_id,
            })
            .ok_or(TenancyFailure::NotFound)
        })
    }

    fn require_active_branch<'a>(
        &'a self,
        tenant_id: Uuid,
        organization_id: Uuid,
        branch_id: Uuid,
    ) -> ContractFuture<'a, Result<ActiveBranch, TenancyFailure>> {
        Box::pin(async move {
            let mut transaction = self
                .pool
                .begin()
                .await
                .map_err(|_| TenancyFailure::Unavailable)?;
            sqlx::query("SELECT set_config('app.tenant_id', $1, true)")
                .bind(tenant_id.to_string())
                .execute(&mut *transaction)
                .await
                .map_err(|_| TenancyFailure::Unavailable)?;
            let row = sqlx::query_as::<_, (Uuid, Uuid, Uuid)>(
                r#"
                SELECT id, tenant_id, organization_id
                FROM core.branches
                WHERE tenant_id = $1
                  AND organization_id = $2
                  AND id = $3
                  AND status = 'active'
                "#,
            )
            .bind(tenant_id)
            .bind(organization_id)
            .bind(branch_id)
            .fetch_optional(&mut *transaction)
            .await
            .map_err(|_| TenancyFailure::Unavailable)?;
            transaction
                .commit()
                .await
                .map_err(|_| TenancyFailure::Unavailable)?;
            row.map(
                |(branch_id, tenant_id, organization_id)| ActiveBranch {
                    branch_id,
                    tenant_id,
                    organization_id,
                },
            )
            .ok_or(TenancyFailure::NotFound)
        })
    }
}
