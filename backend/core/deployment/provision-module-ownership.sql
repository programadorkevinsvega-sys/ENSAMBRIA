-- Provision isolated PostgreSQL module identities and destination schemas.
-- Run as a database administrator after core_migrator exists.
-- This bootstraps the five canonical schemas from an empty database. Module migrations
-- create objects and grant only the runtime privileges required by their owner.
DO $roles$
DECLARE
    role_name text;
    role_names text[] := ARRAY[
        'core_identity_auth_schema_owner',
        'core_identity_auth_runtime',
        'core_tenancy_schema_owner',
        'core_tenancy_runtime',
        'business_establishment_catalog_schema_owner',
        'business_establishment_catalog_runtime',
        'platform_product_catalog_schema_owner',
        'platform_product_catalog_runtime',
        'platform_tenant_onboarding_schema_owner',
        'platform_tenant_onboarding_runtime'
    ];
BEGIN
    FOREACH role_name IN ARRAY role_names LOOP
        IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = role_name) THEN
            EXECUTE format(
                'CREATE ROLE %I %s NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT NOBYPASSRLS',
                role_name,
                CASE WHEN role_name LIKE '%_schema_owner' THEN 'NOLOGIN' ELSE 'LOGIN' END
            );
        END IF;
        EXECUTE format(
            'ALTER ROLE %I %s NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT NOBYPASSRLS',
            role_name,
            CASE WHEN role_name LIKE '%_schema_owner' THEN 'NOLOGIN' ELSE 'LOGIN' END
        );
    END LOOP;
END
$roles$;

DO $preconditions$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'core_migrator') THEN
        RAISE EXCEPTION 'core_migrator must exist before provisioning module ownership';
    END IF;
END
$preconditions$;

GRANT core_identity_auth_schema_owner TO core_migrator;
GRANT core_tenancy_schema_owner TO core_migrator;
GRANT business_establishment_catalog_schema_owner TO core_migrator;
GRANT platform_product_catalog_schema_owner TO core_migrator;
GRANT platform_tenant_onboarding_schema_owner TO core_migrator;

SELECT format('GRANT CONNECT ON DATABASE %I TO core_identity_auth_runtime, core_tenancy_runtime, business_establishment_catalog_runtime, platform_product_catalog_runtime, platform_tenant_onboarding_runtime', current_database()) \gexec

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


-- Runtime roles intentionally receive no USAGE/CREATE here; module migrations grant
-- only the exact schema/table/function privileges required by each runtime.
