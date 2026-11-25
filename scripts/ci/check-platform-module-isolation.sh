#!/usr/bin/env bash
set -euo pipefail

: "${PLATFORM_VERIFY_DATABASE_URL:?PLATFORM_VERIFY_DATABASE_URL is required}"
: "${PLATFORM_PRODUCT_CATALOG_DATABASE_URL:?PLATFORM_PRODUCT_CATALOG_DATABASE_URL is required}"
: "${PLATFORM_TENANT_ONBOARDING_DATABASE_URL:?PLATFORM_TENANT_ONBOARDING_DATABASE_URL is required}"

psql "$PLATFORM_VERIFY_DATABASE_URL" -v ON_ERROR_STOP=1 <<'SQL'
DO $body$
DECLARE
    pair record;
    other_schema text;
BEGIN
    FOR pair IN
        SELECT * FROM (VALUES
            ('platform_product_catalog_runtime', 'platform_product_catalog', 'platform_product_catalog_schema_owner'),
            ('platform_tenant_onboarding_runtime', 'platform_tenant_onboarding', 'platform_tenant_onboarding_schema_owner')
        ) AS expected(runtime_role, schema_name, owner_role)
    LOOP
        IF NOT has_schema_privilege(pair.runtime_role, pair.schema_name, 'USAGE') THEN
            RAISE EXCEPTION 'PLATFORM runtime % lacks own schema USAGE', pair.runtime_role;
        END IF;
        IF has_schema_privilege(pair.runtime_role, pair.schema_name, 'CREATE')
           OR has_database_privilege(pair.runtime_role, current_database(), 'CREATE')
           OR has_database_privilege(pair.runtime_role, current_database(), 'TEMP') THEN
            RAISE EXCEPTION 'PLATFORM runtime % has DDL or TEMP privilege', pair.runtime_role;
        END IF;
        IF pg_has_role(pair.runtime_role, pair.owner_role, 'MEMBER')
           OR pg_has_role(pair.runtime_role, 'core_migrator', 'MEMBER') THEN
            RAISE EXCEPTION 'PLATFORM runtime % can assume owner/migrator roles', pair.runtime_role;
        END IF;
        IF EXISTS (
            SELECT 1 FROM pg_roles
             WHERE rolname = pair.runtime_role
               AND (rolcanlogin IS DISTINCT FROM true
                    OR rolinherit IS DISTINCT FROM false
                    OR rolsuper OR rolcreatedb OR rolcreaterole OR rolbypassrls)
        ) THEN
            RAISE EXCEPTION 'PLATFORM runtime % must be LOGIN, NOINHERIT and non-privileged',
                pair.runtime_role;
        END IF;
        IF EXISTS (
            SELECT 1 FROM pg_roles
             WHERE rolname = pair.owner_role
               AND (rolcanlogin IS DISTINCT FROM false
                    OR rolsuper OR rolcreatedb OR rolcreaterole OR rolbypassrls)
        ) THEN
            RAISE EXCEPTION 'PLATFORM owner % must be NOLOGIN and non-privileged',
                pair.owner_role;
        END IF;

        FOREACH other_schema IN ARRAY ARRAY[
            'core_identity_auth',
            'core_tenancy',
            'business_establishment_catalog',
            'platform_product_catalog',
            'platform_tenant_onboarding'
        ] LOOP
            IF other_schema <> pair.schema_name
               AND has_schema_privilege(pair.runtime_role, other_schema, 'USAGE') THEN
                RAISE EXCEPTION 'PLATFORM runtime % has cross-schema USAGE on %',
                    pair.runtime_role, other_schema;
            END IF;
        END LOOP;

        IF EXISTS (
            SELECT 1
              FROM pg_class c
              JOIN pg_namespace n ON n.oid = c.relnamespace
             WHERE n.nspname IN (
                 'core_identity_auth',
                 'core_tenancy',
                 'business_establishment_catalog',
                 'platform_product_catalog',
                 'platform_tenant_onboarding'
             )
               AND n.nspname <> pair.schema_name
               AND c.relkind IN ('r', 'p', 'v', 'm', 'f')
               AND (
                   has_table_privilege(pair.runtime_role, c.oid, 'SELECT')
                   OR has_table_privilege(pair.runtime_role, c.oid, 'INSERT')
                   OR has_table_privilege(pair.runtime_role, c.oid, 'UPDATE')
                   OR has_table_privilege(pair.runtime_role, c.oid, 'DELETE')
                   OR has_table_privilege(pair.runtime_role, c.oid, 'TRUNCATE')
                   OR has_table_privilege(pair.runtime_role, c.oid, 'REFERENCES')
                   OR has_table_privilege(pair.runtime_role, c.oid, 'TRIGGER')
               )
        ) THEN
            RAISE EXCEPTION 'PLATFORM runtime % has cross-schema table/view privileges',
                pair.runtime_role;
        END IF;

        IF EXISTS (
            SELECT 1
              FROM pg_class c
              JOIN pg_namespace n ON n.oid = c.relnamespace
             WHERE n.nspname IN (
                 'core_identity_auth',
                 'core_tenancy',
                 'business_establishment_catalog',
                 'platform_product_catalog',
                 'platform_tenant_onboarding'
             )
               AND n.nspname <> pair.schema_name
               AND c.relkind = 'S'
               AND (
                   has_sequence_privilege(pair.runtime_role, c.oid, 'USAGE')
                   OR has_sequence_privilege(pair.runtime_role, c.oid, 'SELECT')
                   OR has_sequence_privilege(pair.runtime_role, c.oid, 'UPDATE')
               )
        ) THEN
            RAISE EXCEPTION 'PLATFORM runtime % has cross-schema sequence privileges',
                pair.runtime_role;
        END IF;

        IF EXISTS (
            SELECT 1
              FROM pg_proc p
              JOIN pg_namespace n ON n.oid = p.pronamespace
             WHERE n.nspname IN (
                 'core_identity_auth',
                 'core_tenancy',
                 'business_establishment_catalog',
                 'platform_product_catalog',
                 'platform_tenant_onboarding'
             )
               AND n.nspname <> pair.schema_name
               AND has_function_privilege(pair.runtime_role, p.oid, 'EXECUTE')
        ) THEN
            RAISE EXCEPTION 'PLATFORM runtime % has cross-schema function execution',
                pair.runtime_role;
        END IF;
    END LOOP;

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

    IF NOT has_table_privilege(
            'platform_product_catalog_runtime',
            'platform_product_catalog.active_product_catalog',
            'SELECT'
       )
       OR has_table_privilege(
            'platform_product_catalog_runtime',
            'platform_product_catalog.products',
            'SELECT'
       )
       OR has_table_privilege(
            'platform_product_catalog_runtime',
            'platform_product_catalog.products',
            'INSERT'
       )
       OR has_table_privilege(
            'platform_product_catalog_runtime',
            'platform_product_catalog.products',
            'UPDATE'
       )
       OR NOT has_table_privilege(
            'platform_tenant_onboarding_runtime',
            'platform_tenant_onboarding.inbox_messages',
            'SELECT'
       )
       OR NOT has_table_privilege(
            'platform_tenant_onboarding_runtime',
            'platform_tenant_onboarding.inbox_messages',
            'INSERT'
       )
       OR NOT has_table_privilege(
            'platform_tenant_onboarding_runtime',
            'platform_tenant_onboarding.inbox_messages',
            'UPDATE'
       )
       OR NOT has_table_privilege(
            'platform_tenant_onboarding_runtime',
            'platform_tenant_onboarding.tenant_onboardings',
            'SELECT'
       )
       OR NOT has_table_privilege(
            'platform_tenant_onboarding_runtime',
            'platform_tenant_onboarding.tenant_onboardings',
            'INSERT'
       )
       OR has_table_privilege(
            'platform_tenant_onboarding_runtime',
            'platform_tenant_onboarding.tenant_onboardings',
            'DELETE'
       ) THEN
        RAISE EXCEPTION 'PLATFORM product/onboarding runtime grants violate least privilege';
    END IF;
END
$body$;
SQL

psql "$PLATFORM_PRODUCT_CATALOG_DATABASE_URL" -v ON_ERROR_STOP=1 -c   "SELECT count(*) FROM platform_product_catalog.active_product_catalog" >/dev/null
psql "$PLATFORM_TENANT_ONBOARDING_DATABASE_URL" -v ON_ERROR_STOP=1 -c   "SELECT count(*) FROM platform_tenant_onboarding.inbox_messages" >/dev/null
psql "$PLATFORM_TENANT_ONBOARDING_DATABASE_URL" -v ON_ERROR_STOP=1 -c   "SELECT count(*) FROM platform_tenant_onboarding.tenant_onboardings" >/dev/null

expect_denied() {
    local database_url="$1"
    local description="$2"
    local statement="$3"
    if psql "$database_url" -v ON_ERROR_STOP=1 -c "$statement" >/dev/null 2>&1; then
        echo "PLATFORM runtime unexpectedly succeeded: $description" >&2
        exit 1
    fi
}

expect_denied "$PLATFORM_PRODUCT_CATALOG_DATABASE_URL" "read onboarding inbox"   "SELECT * FROM platform_tenant_onboarding.inbox_messages LIMIT 1"
expect_denied "$PLATFORM_PRODUCT_CATALOG_DATABASE_URL" "insert into product catalog"   "INSERT INTO platform_product_catalog.products (id, product_key, display_name, brand_name) VALUES (gen_random_uuid(), 'probe', 'probe', 'probe')"
expect_denied "$PLATFORM_TENANT_ONBOARDING_DATABASE_URL" "read product catalog"   "SELECT * FROM platform_product_catalog.products LIMIT 1"
expect_denied "$PLATFORM_TENANT_ONBOARDING_DATABASE_URL" "delete onboarding history"   "DELETE FROM platform_tenant_onboarding.tenant_onboardings WHERE false"
expect_denied "$PLATFORM_TENANT_ONBOARDING_DATABASE_URL" "assume onboarding schema owner"   "SET ROLE platform_tenant_onboarding_schema_owner"


# Verify tenant-scoped SaaS subscription tables enforce RLS at runtime, not only
# by inspecting catalog flags. Seed two tenants as the verifier, then execute
# reads/writes as the real PLATFORM product-catalog runtime identity.
psql "$PLATFORM_VERIFY_DATABASE_URL" -v ON_ERROR_STOP=1 <<'SQL'
DO $rls_catalog$
DECLARE
    table_name text;
BEGIN
    FOREACH table_name IN ARRAY ARRAY[
        'saas_subscriptions',
        'saas_subscription_audit',
        'saas_subscription_commands',
        'saas_subscription_outbox'
    ] LOOP
        IF NOT EXISTS (
            SELECT 1
              FROM pg_class c
              JOIN pg_namespace n ON n.oid = c.relnamespace
             WHERE n.nspname = 'platform_product_catalog'
               AND c.relname = table_name
               AND c.relrowsecurity
               AND c.relforcerowsecurity
        ) THEN
            RAISE EXCEPTION 'tenant-scoped table %.% must ENABLE and FORCE RLS',
                'platform_product_catalog', table_name;
        END IF;
        IF NOT EXISTS (
            SELECT 1 FROM pg_policies
             WHERE schemaname = 'platform_product_catalog'
               AND tablename = table_name
               AND policyname = 'tenant_scope'
               AND cmd = 'ALL'
        ) THEN
            RAISE EXCEPTION 'tenant-scoped table %.% is missing its ALL tenant_scope policy',
                'platform_product_catalog', table_name;
        END IF;
    END LOOP;
END
$rls_catalog$;

BEGIN;
INSERT INTO platform_product_catalog.saas_subscription_commands
    (tenant_id, idempotency_key, request_hash, response)
VALUES
    ('018f0f2a-7b3c-7abc-8def-0123456789ae', 'rls-tenant-a-key', decode(repeat('11', 32), 'hex'), '{"tenant":"a"}'),
    ('018f0f2a-7b3c-7abc-8def-0123456789af', 'rls-tenant-b-key', decode(repeat('22', 32), 'hex'), '{"tenant":"b"}');

SET LOCAL ROLE platform_product_catalog_runtime;
SELECT set_config('app.tenant_id', '018f0f2a-7b3c-7abc-8def-0123456789ae', true);
DO $rls_runtime$
DECLARE
    visible_rows integer;
    affected_rows integer;
    spoof_rejected boolean := false;
BEGIN
    SELECT count(*) INTO visible_rows
      FROM platform_product_catalog.saas_subscription_commands;
    IF visible_rows <> 1 THEN
        RAISE EXCEPTION 'tenant A must see exactly its own command, saw %', visible_rows;
    END IF;

    SELECT count(*) INTO visible_rows
      FROM platform_product_catalog.saas_subscription_commands
     WHERE tenant_id = '018f0f2a-7b3c-7abc-8def-0123456789af';
    IF visible_rows <> 0 THEN
        RAISE EXCEPTION 'tenant A can read tenant B command';
    END IF;

    UPDATE platform_product_catalog.saas_subscription_commands
       SET response = '{"tampered":true}'
     WHERE tenant_id = '018f0f2a-7b3c-7abc-8def-0123456789af';
    GET DIAGNOSTICS affected_rows = ROW_COUNT;
    IF affected_rows <> 0 THEN
        RAISE EXCEPTION 'tenant A modified tenant B command';
    END IF;

    BEGIN
        INSERT INTO platform_product_catalog.saas_subscription_commands
            (tenant_id, idempotency_key, request_hash)
        VALUES
            ('018f0f2a-7b3c-7abc-8def-0123456789af',
             'rls-spoofed-tenant-key', decode(repeat('33', 32), 'hex'));
    EXCEPTION WHEN insufficient_privilege THEN
        spoof_rejected := true;
    END;
    IF NOT spoof_rejected THEN
        RAISE EXCEPTION 'tenant A inserted a command for tenant B';
    END IF;

    PERFORM set_config('app.tenant_id', '', true);
    SELECT count(*) INTO visible_rows
      FROM platform_product_catalog.saas_subscription_commands;
    IF visible_rows <> 0 THEN
        RAISE EXCEPTION 'missing tenant context must fail closed';
    END IF;
END
$rls_runtime$;
ROLLBACK;
SQL

echo "PASS: PLATFORM SaaS tenant tables ENABLE/FORCE RLS; tenant A cannot read, update or insert tenant B data, and missing tenant context fails closed."

echo "PASS: PLATFORM product catalog and tenant onboarding runtime identities have positive own-object access and real PostgreSQL denial across schemas, tables, functions, sequences, DDL and owner/migrator memberships."
