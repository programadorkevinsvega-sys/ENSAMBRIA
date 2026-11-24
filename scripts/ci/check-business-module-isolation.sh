#!/usr/bin/env bash
set -euo pipefail

: "${TEST_DATABASE_URL:?TEST_DATABASE_URL is required}"
: "${BUSINESS_DATABASE_URL:?BUSINESS_DATABASE_URL is required}"

psql "$TEST_DATABASE_URL" -v ON_ERROR_STOP=1 <<'SQL'
DO $body$
DECLARE
    owner_name text;
    runtime_login boolean;
    runtime_inherit boolean;
    runtime_bypass boolean;
BEGIN
    SELECT pg_get_userbyid(nspowner)
      INTO owner_name
      FROM pg_namespace
     WHERE nspname = 'business_establishment_catalog';
    IF owner_name IS DISTINCT FROM 'business_establishment_catalog_schema_owner' THEN
        RAISE EXCEPTION 'BUSINESS schema owner is %, expected business_establishment_catalog_schema_owner', owner_name;
    END IF;

    SELECT rolcanlogin, rolinherit, rolbypassrls
      INTO runtime_login, runtime_inherit, runtime_bypass
      FROM pg_roles
     WHERE rolname = 'business_establishment_catalog_runtime';
    IF runtime_login IS DISTINCT FROM true
       OR runtime_inherit IS DISTINCT FROM false
       OR runtime_bypass IS DISTINCT FROM false THEN
        RAISE EXCEPTION 'BUSINESS runtime role must LOGIN, NOINHERIT and NOBYPASSRLS';
    END IF;

    IF has_schema_privilege('business_establishment_catalog_runtime', 'business_establishment_catalog', 'CREATE')
       OR has_database_privilege('business_establishment_catalog_runtime', current_database(), 'CREATE')
       OR has_database_privilege('business_establishment_catalog_runtime', current_database(), 'TEMP')
    THEN
        RAISE EXCEPTION 'BUSINESS runtime unexpectedly has DDL or TEMP privilege';
    END IF;

    IF pg_has_role('business_establishment_catalog_runtime', 'business_establishment_catalog_schema_owner', 'MEMBER')
       OR pg_has_role('business_establishment_catalog_runtime', 'core_migrator', 'MEMBER')
    THEN
        RAISE EXCEPTION 'BUSINESS runtime can assume an owner or migrator role';
    END IF;

    IF NOT has_table_privilege('business_establishment_catalog_runtime', 'business_establishment_catalog.establishments', 'SELECT')
       OR NOT has_table_privilege('business_establishment_catalog_runtime', 'business_establishment_catalog.establishments', 'INSERT')
       OR NOT has_table_privilege('business_establishment_catalog_runtime', 'business_establishment_catalog.establishments', 'UPDATE')
       OR has_table_privilege('business_establishment_catalog_runtime', 'business_establishment_catalog.establishments', 'DELETE')
       OR has_table_privilege('business_establishment_catalog_runtime', 'business_establishment_catalog.establishments', 'TRUNCATE')
    THEN
        RAISE EXCEPTION 'BUSINESS runtime table grants are not least-privilege';
    END IF;

    IF has_schema_privilege('business_establishment_catalog_runtime', 'core_identity_auth', 'USAGE')
       OR has_schema_privilege('business_establishment_catalog_runtime', 'core_tenancy', 'USAGE')
       OR has_schema_privilege('business_establishment_catalog_runtime', 'platform_product_catalog', 'USAGE')
       OR has_schema_privilege('business_establishment_catalog_runtime', 'platform_tenant_onboarding', 'USAGE')
    THEN
        RAISE EXCEPTION 'BUSINESS runtime has cross-owner schema access';
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'business_establishment_catalog'
          AND c.relname = 'establishments'
          AND c.relrowsecurity
          AND c.relforcerowsecurity
    ) THEN
        RAISE EXCEPTION 'BUSINESS establishments must enable and FORCE row-level security';
    END IF;

    IF EXISTS (
        SELECT 1
        FROM pg_namespace n
        CROSS JOIN LATERAL aclexplode(COALESCE(n.nspacl, acldefault('n', n.nspowner))) acl
        WHERE n.nspname = 'business_establishment_catalog'
          AND acl.grantee = 0
          AND acl.privilege_type IN ('USAGE', 'CREATE')
    ) THEN
        RAISE EXCEPTION 'PUBLIC has USAGE or CREATE on BUSINESS schema';
    END IF;
END
$body$;
SQL

# The actual runtime identity must be able to perform tenant-scoped DML.
psql "$BUSINESS_DATABASE_URL" -v ON_ERROR_STOP=1 <<'SQL'
BEGIN;
SELECT set_config('app.tenant_id', '018f0f2a-7b3c-7abc-8def-0123456789af', true);
INSERT INTO business_establishment_catalog.establishments
    (id, tenant_id, organization_id, display_name)
VALUES
    ('018f0f2a-7b3c-7abc-8def-0123456789b0',
     '018f0f2a-7b3c-7abc-8def-0123456789af',
     '018f0f2a-7b3c-7abc-8def-0123456789b1',
     'isolation-probe');
UPDATE business_establishment_catalog.establishments
   SET display_name = 'isolation-probe-updated'
 WHERE id = '018f0f2a-7b3c-7abc-8def-0123456789b0';
SELECT count(*) FROM business_establishment_catalog.establishments
 WHERE id = '018f0f2a-7b3c-7abc-8def-0123456789b0';
ROLLBACK;
SQL

expect_denied() {
    local description="$1"
    local statement="$2"
    if psql "$BUSINESS_DATABASE_URL" -v ON_ERROR_STOP=1 -c "$statement" >/dev/null 2>&1; then
        echo "BUSINESS runtime unexpectedly succeeded: $description" >&2
        exit 1
    fi
}

expect_denied "read CORE memberships" "SELECT * FROM core_tenancy.tenant_memberships LIMIT 1"
expect_denied "read CORE identities" "SELECT * FROM core_identity_auth.identities LIMIT 1"
expect_denied "create a table in its own schema" "CREATE TABLE business_establishment_catalog.privilege_probe(id integer)"
expect_denied "assume the schema owner" "SET ROLE business_establishment_catalog_schema_owner"
