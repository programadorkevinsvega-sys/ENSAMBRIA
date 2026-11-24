-- Provision BUSINESS's own schema identity only; no CORE or PLATFORM access is granted.
-- Safe to re-run. The shared module provisioner uses the same canonical roles.
DO $roles$
DECLARE
    role_name text;
    role_names text[] := ARRAY[
        'business_establishment_catalog_schema_owner',
        'business_establishment_catalog_runtime'
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
        RAISE EXCEPTION 'core_migrator must exist before provisioning BUSINESS ownership';
    END IF;
END
$preconditions$;

GRANT business_establishment_catalog_schema_owner TO core_migrator;
SELECT format('GRANT CREATE ON DATABASE %I TO business_establishment_catalog_schema_owner', current_database()) \gexec
SELECT format('GRANT CONNECT ON DATABASE %I TO business_establishment_catalog_runtime', current_database()) \gexec

CREATE SCHEMA IF NOT EXISTS business_establishment_catalog
    AUTHORIZATION business_establishment_catalog_schema_owner;
ALTER SCHEMA business_establishment_catalog
    OWNER TO business_establishment_catalog_schema_owner;
REVOKE ALL ON SCHEMA business_establishment_catalog FROM PUBLIC;
