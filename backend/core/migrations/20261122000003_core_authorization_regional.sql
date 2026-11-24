SET LOCAL ROLE core_tenancy_schema_owner;

-- Explicit authorization evaluation and inherited regional context.
-- These functions are SECURITY INVOKER and require transaction-local actor/tenant context.

CREATE FUNCTION core_tenancy.has_permission(
    p_actor_id uuid,
    p_tenant_id uuid,
    p_permission_code text,
    p_organization_id uuid,
    p_branch_id uuid
)
RETURNS boolean
LANGUAGE sql
STABLE
SECURITY INVOKER
SET search_path = pg_catalog, core
AS $function$
    SELECT
        COALESCE(
            p_actor_id = NULLIF(current_setting('app.actor_id', true), '')::uuid,
            false
        )
        AND COALESCE(
            p_tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::uuid,
            false
        )
        AND (p_branch_id IS NULL OR p_organization_id IS NOT NULL)
        AND EXISTS (
            SELECT 1
            FROM core_tenancy.tenants AS t
            INNER JOIN core_tenancy.tenant_memberships AS m
                ON m.tenant_id = t.id AND m.identity_id = p_actor_id
            INNER JOIN core_tenancy.membership_role_assignments AS a
                ON a.tenant_id = m.tenant_id AND a.membership_id = m.id
            INNER JOIN core_tenancy.tenant_role_permissions AS rp
                ON rp.tenant_id = a.tenant_id AND rp.role_id = a.role_id
            WHERE t.status = 'active'
              AND m.status = 'active'
              AND rp.permission_code = p_permission_code
              AND (
                    a.scope_type = 'tenant'
                    OR (a.scope_type = 'organization' AND a.organization_id = p_organization_id)
                    OR (
                        a.scope_type = 'branch'
                        AND a.organization_id = p_organization_id
                        AND a.branch_id = p_branch_id
                    )
              )
              AND (
                    p_organization_id IS NULL
                    OR EXISTS (
                        SELECT 1 FROM core_tenancy.organizations AS o
                        WHERE o.id = p_organization_id
                          AND o.tenant_id = p_tenant_id
                          AND o.status = 'active'
                    )
              )
              AND (
                    p_branch_id IS NULL
                    OR EXISTS (
                        SELECT 1 FROM core_tenancy.branches AS b
                        WHERE b.id = p_branch_id
                          AND b.tenant_id = p_tenant_id
                          AND b.organization_id = p_organization_id
                          AND b.status = 'active'
                    )
              )
        )
$function$;

CREATE FUNCTION core_tenancy.resolve_regional_context(
    p_tenant_id uuid,
    p_organization_id uuid,
    p_branch_id uuid
)
RETURNS TABLE (
    country_code char(2),
    currency_code char(3),
    locale text,
    time_zone text
)
LANGUAGE sql
STABLE
SECURITY INVOKER
SET search_path = pg_catalog, core
AS $function$
    SELECT
        COALESCE(b.country_code, o.country_code, t.country_code),
        COALESCE(b.currency_code, o.currency_code, t.currency_code),
        COALESCE(b.locale, o.locale, t.locale),
        COALESCE(b.time_zone, o.time_zone, t.time_zone)
    FROM core_tenancy.tenants AS t
    LEFT JOIN core_tenancy.organizations AS o
        ON o.tenant_id = t.id
       AND o.id = p_organization_id
       AND o.status = 'active'
    LEFT JOIN core_tenancy.branches AS b
        ON b.tenant_id = t.id
       AND b.organization_id = o.id
       AND b.id = p_branch_id
       AND b.status = 'active'
    WHERE t.id = p_tenant_id
      AND t.status = 'active'
      AND p_tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::uuid
      AND (p_organization_id IS NULL OR o.id IS NOT NULL)
      AND (p_branch_id IS NULL OR (p_organization_id IS NOT NULL AND b.id IS NOT NULL))
$function$;

REVOKE ALL ON FUNCTION core_tenancy.has_permission(uuid, uuid, text, uuid, uuid) FROM PUBLIC;
REVOKE ALL ON FUNCTION core_tenancy.resolve_regional_context(uuid, uuid, uuid) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION core_tenancy.has_permission(uuid, uuid, text, uuid, uuid) TO core_tenancy_runtime;
GRANT EXECUTE ON FUNCTION core_tenancy.resolve_regional_context(uuid, uuid, uuid) TO core_tenancy_runtime;

COMMENT ON FUNCTION core_tenancy.has_permission(uuid, uuid, text, uuid, uuid) IS
    'Evaluates active membership permissions at tenant, organization or branch scope under transaction-local actor and tenant RLS context.';
COMMENT ON FUNCTION core_tenancy.resolve_regional_context(uuid, uuid, uuid) IS
    'Resolves regional settings with branch > organization > tenant precedence and validates the supplied scope.';

-- SQLx migration ledger is owned by core_tenancy_schema_owner.
SET LOCAL ROLE core_tenancy_schema_owner;
