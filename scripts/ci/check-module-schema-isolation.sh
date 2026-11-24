#!/usr/bin/env bash
set -euo pipefail

: "${TEST_DATABASE_URL:?TEST_DATABASE_URL is required}"

# core-migrate keeps the SQLx migration ledger under core_tenancy_schema_owner.
# RESET ROLE would silently drop that owner context back to core_migrator before
# SQLx writes its migration record, causing permission failures and partial runs.
if grep -RInE '^[[:space:]]*RESET[[:space:]]+ROLE[[:space:]]*;' backend/core/migrations; then
    echo "CORE migrations must switch owners with SET LOCAL ROLE; RESET ROLE breaks SQLx metadata ownership." >&2
    exit 1
fi

# SQLx keeps its migration ledger in core_tenancy._sqlx_migrations. Every
# migration must return to that schema owner before SQLx records its version.
for migration in backend/core/migrations/*.sql; do
    if ! tail -n 3 "$migration" | grep -Eq '^[[:space:]]*SET LOCAL ROLE core_tenancy_schema_owner;[[:space:]]*psql "$TEST_DATABASE_URL" -v ON_ERROR_STOP=1 <<'SQL'
DO $$
DECLARE
    pair record;
    usage_allowed boolean;
    actual_owner text;
    login_allowed boolean;
    inherit_allowed boolean;
    bypass_rls boolean;
    owner_login boolean;
BEGIN
    FOR pair IN
        SELECT * FROM (VALUES
            ('core_identity_auth_runtime', 'core_identity_auth', 'core_identity_auth_schema_owner'),
            ('core_tenancy_runtime', 'core_tenancy', 'core_tenancy_schema_owner'),
            ('business_establishment_catalog_runtime', 'business_establishment_catalog', 'business_establishment_catalog_schema_owner'),
            ('platform_product_catalog_runtime', 'platform_product_catalog', 'platform_product_catalog_schema_owner'),
            ('platform_tenant_onboarding_runtime', 'platform_tenant_onboarding', 'platform_tenant_onboarding_schema_owner')
        ) AS expected(role_name, schema_name, owner_role)
    LOOP
        SELECT has_schema_privilege(pair.role_name, pair.schema_name, 'USAGE')
          INTO usage_allowed;
        IF usage_allowed THEN
            RAISE EXCEPTION 'runtime role % unexpectedly has USAGE on % before cutover',
                pair.role_name, pair.schema_name;
        END IF;

        SELECT pg_get_userbyid(nspowner)
          INTO actual_owner
          FROM pg_namespace
         WHERE nspname = pair.schema_name;
        IF actual_owner IS DISTINCT FROM pair.owner_role THEN
            RAISE EXCEPTION 'schema % owner is %, expected %',
                pair.schema_name, actual_owner, pair.owner_role;
        END IF;

        SELECT rolcanlogin
          INTO owner_login
          FROM pg_roles
         WHERE rolname = pair.owner_role;
        IF owner_login IS DISTINCT FROM false THEN
            RAISE EXCEPTION 'schema owner role % must be NOLOGIN', pair.owner_role;
        END IF;

        SELECT rolcanlogin, rolinherit, rolbypassrls
          INTO login_allowed, inherit_allowed, bypass_rls
          FROM pg_roles
         WHERE rolname = pair.role_name;
        IF login_allowed IS DISTINCT FROM true
           OR inherit_allowed IS DISTINCT FROM false
           OR bypass_rls IS DISTINCT FROM false THEN
            RAISE EXCEPTION 'runtime role % must LOGIN, NOINHERIT and NOBYPASSRLS',
                pair.role_name;
        END IF;
    END LOOP;

    IF has_schema_privilege('business_establishment_catalog_runtime', 'core_tenancy', 'USAGE')
       OR has_schema_privilege('business_establishment_catalog_runtime', 'core_identity_auth', 'USAGE')
       OR has_schema_privilege('platform_tenant_onboarding_runtime', 'platform_product_catalog', 'USAGE')
       OR has_schema_privilege('platform_tenant_onboarding_runtime', 'core_tenancy', 'USAGE')
    THEN
        RAISE EXCEPTION 'cross-module schema access was granted';
    END IF;

    IF EXISTS (
        SELECT 1
          FROM pg_namespace n
          CROSS JOIN LATERAL aclexplode(COALESCE(n.nspacl, acldefault('n', n.nspowner))) acl
         WHERE n.nspname IN (
             'core_identity_auth',
             'core_tenancy',
             'business_establishment_catalog',
             'platform_product_catalog',
             'platform_tenant_onboarding'
         )
           AND acl.grantee = 0
           AND acl.privilege_type IN ('USAGE', 'CREATE')
    ) THEN
        RAISE EXCEPTION 'PUBLIC has USAGE or CREATE on a module schema';
    END IF;
END
$$;
SQL
; then
        echo "CORE migration must finish as core_tenancy_schema_owner for SQLx metadata: $migration" >&2
        exit 1
    fi
done
psql "$TEST_DATABASE_URL" -v ON_ERROR_STOP=1 <<'SQL'
DO $$
DECLARE
    pair record;
    usage_allowed boolean;
    actual_owner text;
    login_allowed boolean;
    inherit_allowed boolean;
    bypass_rls boolean;
    owner_login boolean;
BEGIN
    FOR pair IN
        SELECT * FROM (VALUES
            ('core_identity_auth_runtime', 'core_identity_auth', 'core_identity_auth_schema_owner'),
            ('core_tenancy_runtime', 'core_tenancy', 'core_tenancy_schema_owner'),
            ('business_establishment_catalog_runtime', 'business_establishment_catalog', 'business_establishment_catalog_schema_owner'),
            ('platform_product_catalog_runtime', 'platform_product_catalog', 'platform_product_catalog_schema_owner'),
            ('platform_tenant_onboarding_runtime', 'platform_tenant_onboarding', 'platform_tenant_onboarding_schema_owner')
        ) AS expected(role_name, schema_name, owner_role)
    LOOP
        SELECT has_schema_privilege(pair.role_name, pair.schema_name, 'USAGE')
          INTO usage_allowed;
        IF usage_allowed THEN
            RAISE EXCEPTION 'runtime role % unexpectedly has USAGE on % before cutover',
                pair.role_name, pair.schema_name;
        END IF;

        SELECT pg_get_userbyid(nspowner)
          INTO actual_owner
          FROM pg_namespace
         WHERE nspname = pair.schema_name;
        IF actual_owner IS DISTINCT FROM pair.owner_role THEN
            RAISE EXCEPTION 'schema % owner is %, expected %',
                pair.schema_name, actual_owner, pair.owner_role;
        END IF;

        SELECT rolcanlogin
          INTO owner_login
          FROM pg_roles
         WHERE rolname = pair.owner_role;
        IF owner_login IS DISTINCT FROM false THEN
            RAISE EXCEPTION 'schema owner role % must be NOLOGIN', pair.owner_role;
        END IF;

        SELECT rolcanlogin, rolinherit, rolbypassrls
          INTO login_allowed, inherit_allowed, bypass_rls
          FROM pg_roles
         WHERE rolname = pair.role_name;
        IF login_allowed IS DISTINCT FROM true
           OR inherit_allowed IS DISTINCT FROM false
           OR bypass_rls IS DISTINCT FROM false THEN
            RAISE EXCEPTION 'runtime role % must LOGIN, NOINHERIT and NOBYPASSRLS',
                pair.role_name;
        END IF;
    END LOOP;

    IF has_schema_privilege('business_establishment_catalog_runtime', 'core_tenancy', 'USAGE')
       OR has_schema_privilege('business_establishment_catalog_runtime', 'core_identity_auth', 'USAGE')
       OR has_schema_privilege('platform_tenant_onboarding_runtime', 'platform_product_catalog', 'USAGE')
       OR has_schema_privilege('platform_tenant_onboarding_runtime', 'core_tenancy', 'USAGE')
    THEN
        RAISE EXCEPTION 'cross-module schema access was granted';
    END IF;

    IF EXISTS (
        SELECT 1
          FROM pg_namespace n
          CROSS JOIN LATERAL aclexplode(COALESCE(n.nspacl, acldefault('n', n.nspowner))) acl
         WHERE n.nspname IN (
             'core_identity_auth',
             'core_tenancy',
             'business_establishment_catalog',
             'platform_product_catalog',
             'platform_tenant_onboarding'
         )
           AND acl.grantee = 0
           AND acl.privilege_type IN ('USAGE', 'CREATE')
    ) THEN
        RAISE EXCEPTION 'PUBLIC has USAGE or CREATE on a module schema';
    END IF;
END
$$;
SQL
