-- Idempotently provision all five module owners and their restricted identities.
-- Run only through the PostgreSQL administrative channel, against the intended database.
-- Passwords are deliberately not created, embedded, or rotated by this script.
DO $guard$
DECLARE
    current_role record;
BEGIN
    SELECT rolsuper, rolcreaterole
      INTO current_role
      FROM pg_catalog.pg_roles
     WHERE rolname = current_user;

    IF NOT FOUND OR (NOT current_role.rolsuper AND NOT current_role.rolcreaterole) THEN
        RAISE EXCEPTION 'module ownership provisioning requires a PostgreSQL administrator'
            USING ERRCODE = '42501';
    END IF;
END
$guard$;

DO $roles$
DECLARE
    role_name text;
    role_login boolean;
    role_names text[] := ARRAY[
        'core_migrator',
        'core_identity_auth_schema_owner',
        'core_identity_auth_runtime',
        'core_tenancy_schema_owner',
        'core_tenancy_runtime',
        'business_establishment_catalog_schema_owner',
        'business_establishment_catalog_runtime',
        'platform_product_catalog_schema_owner',
        'platform_product_catalog_runtime',
        'platform_tenant_onboarding_schema_owner',
        'platform_tenant_onboarding_runtime',
        'core_email_worker',
        'core_outbox_relay'
    ];
BEGIN
    FOREACH role_name IN ARRAY role_names LOOP
        role_login := role_name = 'core_migrator'
            OR role_name LIKE '%_runtime'
            OR role_name IN ('core_email_worker', 'core_outbox_relay');

        IF NOT EXISTS (
            SELECT 1 FROM pg_catalog.pg_roles WHERE rolname = role_name
        ) THEN
            EXECUTE format(
                'CREATE ROLE %I %s NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT NOBYPASSRLS',
                role_name,
                CASE WHEN role_login THEN 'LOGIN' ELSE 'NOLOGIN' END
            );
        END IF;

        EXECUTE format(
            'ALTER ROLE %I %s NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT NOBYPASSRLS',
            role_name,
            CASE WHEN role_login THEN 'LOGIN' ELSE 'NOLOGIN' END
        );
    END LOOP;
END
$roles$;

-- The migrator can assume only schema-owner identities. It is not a member of
-- any runtime or worker login role.
GRANT core_identity_auth_schema_owner TO core_migrator;
GRANT core_tenancy_schema_owner TO core_migrator;
GRANT business_establishment_catalog_schema_owner TO core_migrator;
GRANT platform_product_catalog_schema_owner TO core_migrator;
GRANT platform_tenant_onboarding_schema_owner TO core_migrator;

SELECT format(
    'GRANT CONNECT ON DATABASE %I TO core_migrator, core_identity_auth_runtime, core_tenancy_runtime, business_establishment_catalog_runtime, platform_product_catalog_runtime, platform_tenant_onboarding_runtime, core_email_worker, core_outbox_relay',
    current_database()
) \\gexec

CREATE SCHEMA IF NOT EXISTS core_identity_auth AUTHORIZATION core_identity_auth_schema_owner;
CREATE SCHEMA IF NOT EXISTS core_tenancy AUTHORIZATION core_tenancy_schema_owner;
CREATE SCHEMA IF NOT EXISTS business_establishment_catalog AUTHORIZATION business_establishment_catalog_schema_owner;
CREATE SCHEMA IF NOT EXISTS platform_product_catalog AUTHORIZATION platform_product_catalog_schema_owner;
CREATE SCHEMA IF NOT EXISTS platform_tenant_onboarding AUTHORIZATION platform_tenant_onboarding_schema_owner;

ALTER SCHEMA core_identity_auth OWNER TO core_identity_auth_schema_owner;
ALTER SCHEMA core_tenancy OWNER TO core_tenancy_schema_owner;
ALTER SCHEMA business_establishment_catalog OWNER TO business_establishment_catalog_schema_owner;
ALTER SCHEMA platform_product_catalog OWNER TO platform_product_catalog_schema_owner;
ALTER SCHEMA platform_tenant_onboarding OWNER TO platform_tenant_onboarding_schema_owner;

REVOKE ALL ON SCHEMA
    core_identity_auth,
    core_tenancy,
    business_establishment_catalog,
    platform_product_catalog,
    platform_tenant_onboarding
FROM PUBLIC;

-- CORE identity DDL references the tenancy-owned security namespace. This grants
-- namespace resolution to the identity schema owner only, not tenancy data access.
GRANT USAGE ON SCHEMA core_tenancy TO core_identity_auth_schema_owner;

-- Runtime roles intentionally receive no schema USAGE/CREATE here. Each module's
-- migrations grant only the exact schema/table/sequence/function privileges required.
