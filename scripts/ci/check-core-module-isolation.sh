#!/usr/bin/env bash
set -euo pipefail

: "${PLATFORM_VERIFY_DATABASE_URL:?PLATFORM_VERIFY_DATABASE_URL is required}"
: "${CORE_TEST_DATABASE_URL:?CORE_TEST_DATABASE_URL is required}"
: "${CORE_TENANCY_DATABASE_URL:?CORE_TENANCY_DATABASE_URL is required}"
: "${CORE_EMAIL_WORKER_DATABASE_URL:?CORE_EMAIL_WORKER_DATABASE_URL is required}"
: "${CORE_OUTBOX_DATABASE_URL:?CORE_OUTBOX_DATABASE_URL is required}"

psql "$PLATFORM_VERIFY_DATABASE_URL" -v ON_ERROR_STOP=1 <<'SQL'
DO $body$
DECLARE
    pair record;
    other_schema text;
    object_privilege boolean;
BEGIN
    FOR pair IN
        SELECT * FROM (VALUES
            ('core_identity_auth_runtime', 'core_identity_auth', 'core_identity_auth_schema_owner'),
            ('core_tenancy_runtime', 'core_tenancy', 'core_tenancy_schema_owner'),
            ('core_email_worker', 'core_identity_auth', 'core_identity_auth_schema_owner'),
            ('core_outbox_relay', 'core_tenancy', 'core_tenancy_schema_owner')
        ) AS expected(runtime_role, schema_name, owner_role)
    LOOP
        IF NOT has_schema_privilege(pair.runtime_role, pair.schema_name, 'USAGE') THEN
            RAISE EXCEPTION 'runtime role % lacks USAGE on its own schema %',
                pair.runtime_role, pair.schema_name;
        END IF;
        IF has_schema_privilege(pair.runtime_role, pair.schema_name, 'CREATE')
           OR has_database_privilege(pair.runtime_role, current_database(), 'CREATE')
           OR has_database_privilege(pair.runtime_role, current_database(), 'TEMP') THEN
            RAISE EXCEPTION 'runtime role % has DDL or TEMP privilege', pair.runtime_role;
        END IF;
        IF pg_has_role(pair.runtime_role, pair.owner_role, 'MEMBER')
           OR pg_has_role(pair.runtime_role, 'core_migrator', 'MEMBER') THEN
            RAISE EXCEPTION 'runtime role % can assume an owner or migrator role',
                pair.runtime_role;
        END IF;
        IF NOT pg_has_role('core_migrator', pair.owner_role, 'MEMBER') THEN
            RAISE EXCEPTION 'core_migrator is not explicitly a member of %', pair.owner_role;
        END IF;
        IF EXISTS (
            SELECT 1 FROM pg_roles
             WHERE rolname = pair.runtime_role
               AND (rolcanlogin IS DISTINCT FROM true
                    OR rolinherit IS DISTINCT FROM false
                    OR rolsuper OR rolcreatedb OR rolcreaterole OR rolbypassrls)
        ) THEN
            RAISE EXCEPTION 'runtime role % must be LOGIN, NOINHERIT, NOSUPERUSER, NOCREATEDB, NOCREATEROLE and NOBYPASSRLS',
                pair.runtime_role;
        END IF;
        IF EXISTS (
            SELECT 1 FROM pg_roles
             WHERE rolname = pair.owner_role
               AND (rolcanlogin IS DISTINCT FROM false
                    OR rolsuper OR rolcreatedb OR rolcreaterole OR rolbypassrls)
        ) THEN
            RAISE EXCEPTION 'schema owner % must be NOLOGIN and non-privileged', pair.owner_role;
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
                RAISE EXCEPTION 'runtime role % has cross-schema USAGE on %',
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
            RAISE EXCEPTION 'runtime role % has cross-schema table/view privileges',
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
            RAISE EXCEPTION 'runtime role % has cross-schema sequence privileges',
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
            RAISE EXCEPTION 'runtime role % has cross-schema function execution',
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

    IF NOT has_table_privilege('core_identity_auth_runtime', 'core_identity_auth.identities', 'SELECT')
       OR NOT has_table_privilege('core_identity_auth_runtime', 'core_identity_auth.identities', 'INSERT')
       OR NOT has_table_privilege('core_identity_auth_runtime', 'core_identity_auth.identities', 'UPDATE')
       OR has_table_privilege('core_identity_auth_runtime', 'core_identity_auth.identities', 'DELETE')
       OR NOT has_table_privilege('core_identity_auth_runtime', 'core_identity_auth.email_outbox', 'INSERT')
       OR has_table_privilege('core_identity_auth_runtime', 'core_identity_auth.email_outbox', 'SELECT')
       OR NOT has_table_privilege('core_tenancy_runtime', 'core_tenancy.tenants', 'SELECT')
       OR NOT has_table_privilege('core_tenancy_runtime', 'core_tenancy.tenants', 'INSERT')
       OR NOT has_table_privilege('core_tenancy_runtime', 'core_tenancy.tenants', 'UPDATE')
       OR NOT has_table_privilege('core_tenancy_runtime', 'core_tenancy.outbox_events', 'INSERT')
       OR has_table_privilege('core_tenancy_runtime', 'core_tenancy.outbox_events', 'SELECT')
       OR has_table_privilege('core_tenancy_runtime', 'core_tenancy.outbox_events', 'UPDATE')
       OR has_table_privilege('core_tenancy_runtime', 'core_tenancy.outbox_events', 'DELETE')
       OR NOT has_table_privilege('core_email_worker', 'core_identity_auth.email_outbox', 'SELECT')
       OR NOT has_table_privilege('core_email_worker', 'core_identity_auth.email_outbox', 'UPDATE')
       OR has_table_privilege('core_email_worker', 'core_identity_auth.email_outbox', 'INSERT')
       OR has_table_privilege('core_email_worker', 'core_identity_auth.email_outbox', 'DELETE')
       OR NOT has_table_privilege('core_outbox_relay', 'core_tenancy.outbox_events', 'SELECT')
       OR NOT has_table_privilege('core_outbox_relay', 'core_tenancy.outbox_events', 'UPDATE')
       OR has_table_privilege('core_outbox_relay', 'core_tenancy.outbox_events', 'INSERT')
       OR has_table_privilege('core_outbox_relay', 'core_tenancy.outbox_events', 'DELETE') THEN
        RAISE EXCEPTION 'CORE runtime/worker object grants do not match least-privilege contract';
    END IF;

    IF has_schema_privilege('core_email_worker', 'core_tenancy', 'USAGE')
       OR has_schema_privilege('core_outbox_relay', 'core_identity_auth', 'USAGE')
       OR pg_has_role('core_email_worker', 'core_identity_auth_schema_owner', 'MEMBER')
       OR pg_has_role('core_outbox_relay', 'core_tenancy_schema_owner', 'MEMBER')
       OR pg_has_role('core_email_worker', 'core_migrator', 'MEMBER')
       OR pg_has_role('core_outbox_relay', 'core_migrator', 'MEMBER') THEN
        RAISE EXCEPTION 'CORE worker role crosses a schema or owner boundary';
    END IF;
END
$body$;
SQL

# Prove the four runtime/worker identities can access only the objects they own.
psql "$CORE_TEST_DATABASE_URL" -v ON_ERROR_STOP=1 -c   "SELECT count(*) FROM core_identity_auth.identities" >/dev/null
psql "$CORE_TENANCY_DATABASE_URL" -v ON_ERROR_STOP=1 -c   "SELECT count(*) FROM core_tenancy.tenants" >/dev/null
psql "$CORE_EMAIL_WORKER_DATABASE_URL" -v ON_ERROR_STOP=1 -c   "SELECT count(*) FROM core_identity_auth.email_outbox" >/dev/null
psql "$CORE_OUTBOX_DATABASE_URL" -v ON_ERROR_STOP=1 -c   "SELECT count(*) FROM core_tenancy.outbox_events" >/dev/null
psql "$CORE_EMAIL_WORKER_DATABASE_URL" -v ON_ERROR_STOP=1 -c   "UPDATE core_identity_auth.email_outbox SET status = status WHERE false" >/dev/null
psql "$CORE_OUTBOX_DATABASE_URL" -v ON_ERROR_STOP=1 -c   "UPDATE core_tenancy.outbox_events SET status = status WHERE false" >/dev/null

# Prove CORE tenancy's tenant RLS with two seeded tenants and the real
# core_tenancy_runtime identity. The verifier seeds data; runtime cannot cross
# tenant boundaries, spoof a tenant, or read without a tenant context.
psql "$PLATFORM_VERIFY_DATABASE_URL" -v ON_ERROR_STOP=1 <<'SQL'
INSERT INTO core_tenancy.tenants (id, status)
VALUES
    ('018f0f2a-7b3c-7abc-8def-0123456789c1', 'active'),
    ('018f0f2a-7b3c-7abc-8def-0123456789c2', 'active');
SQL

psql "$CORE_TENANCY_DATABASE_URL" -v ON_ERROR_STOP=1 <<'SQL'
BEGIN;
SELECT set_config('app.tenant_id', '018f0f2a-7b3c-7abc-8def-0123456789c1', true);
DO $tenant_rls$
DECLARE
    visible_rows integer;
    affected_rows integer;
    spoof_rejected boolean := false;
BEGIN
    SELECT count(*) INTO visible_rows FROM core_tenancy.tenants;
    IF visible_rows <> 1 THEN
        RAISE EXCEPTION 'tenant A must see exactly its own tenant row, saw %', visible_rows;
    END IF;

    SELECT count(*) INTO visible_rows
      FROM core_tenancy.tenants
     WHERE id = '018f0f2a-7b3c-7abc-8def-0123456789c2';
    IF visible_rows <> 0 THEN
        RAISE EXCEPTION 'tenant A can read tenant B';
    END IF;

    UPDATE core_tenancy.tenants
       SET status = 'suspended'
     WHERE id = '018f0f2a-7b3c-7abc-8def-0123456789c2';
    GET DIAGNOSTICS affected_rows = ROW_COUNT;
    IF affected_rows <> 0 THEN
        RAISE EXCEPTION 'tenant A modified tenant B';
    END IF;

    BEGIN
        INSERT INTO core_tenancy.tenants (id, status)
        VALUES ('018f0f2a-7b3c-7abc-8def-0123456789c3', 'active');
    EXCEPTION WHEN insufficient_privilege THEN
        spoof_rejected := true;
    END;
    IF NOT spoof_rejected THEN
        RAISE EXCEPTION 'tenant A inserted a row outside its own tenant scope';
    END IF;

    PERFORM set_config('app.tenant_id', '', true);
    SELECT count(*) INTO visible_rows FROM core_tenancy.tenants;
    IF visible_rows <> 0 THEN
        RAISE EXCEPTION 'missing tenant context must fail closed';
    END IF;
END
$tenant_rls$;
ROLLBACK;
SQL

expect_denied() {
    local database_url="$1"
    local description="$2"
    local statement="$3"
    if psql "$database_url" -v ON_ERROR_STOP=1 -c "$statement" >/dev/null 2>&1; then
        echo "CORE runtime unexpectedly succeeded: $description" >&2
        exit 1
    fi
}

expect_denied "$CORE_TEST_DATABASE_URL" "read CORE tenancy memberships"   "SELECT * FROM core_tenancy.tenant_memberships LIMIT 1"
expect_denied "$CORE_TEST_DATABASE_URL" "read email outbox"   "SELECT * FROM core_identity_auth.email_outbox LIMIT 1"
expect_denied "$CORE_TEST_DATABASE_URL" "delete identities"   "DELETE FROM core_identity_auth.identities WHERE false"
expect_denied "$CORE_TENANCY_DATABASE_URL" "read CORE identities"   "SELECT * FROM core_identity_auth.identities LIMIT 1"
expect_denied "$CORE_TENANCY_DATABASE_URL" "read Outbox relay state"   "SELECT * FROM core_tenancy.outbox_events LIMIT 1"
expect_denied "$CORE_EMAIL_WORKER_DATABASE_URL" "read CORE identities"   "SELECT * FROM core_identity_auth.identities LIMIT 1"
expect_denied "$CORE_EMAIL_WORKER_DATABASE_URL" "write CORE email payload"   "INSERT INTO core_identity_auth.email_outbox (id, recipient, subject, text_body) VALUES (gen_random_uuid(), 'probe@example.test', 'probe', 'probe')"
expect_denied "$CORE_OUTBOX_DATABASE_URL" "read CORE tenants"   "SELECT * FROM core_tenancy.tenants LIMIT 1"
expect_denied "$CORE_OUTBOX_DATABASE_URL" "create a business event"   "INSERT INTO core_tenancy.outbox_events (event_id, event_type, schema_version, aggregate_type, aggregate_id, aggregate_version, payload) VALUES (gen_random_uuid(), 'probe', 1, 'probe', gen_random_uuid(), 1, '{}'::jsonb)"
expect_denied "$CORE_TEST_DATABASE_URL" "assume the identity schema owner"   "SET ROLE core_identity_auth_schema_owner"
expect_denied "$CORE_TENANCY_DATABASE_URL" "assume core_migrator"   "SET ROLE core_migrator"

echo "PASS: CORE identity, tenancy, email worker and Outbox relay identities have positive own-object access and real PostgreSQL denial across schemas, tables, functions, sequences, DDL and owner/migrator memberships."
