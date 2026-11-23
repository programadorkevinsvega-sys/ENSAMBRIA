-- Run as a PostgreSQL database administrator after CORE roles have been provisioned.
-- Safe to re-run. This does not create or rotate passwords for LOGIN roles.
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'business_schema_owner') THEN
        CREATE ROLE business_schema_owner NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT NOBYPASSRLS;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'business_runtime') THEN
        CREATE ROLE business_runtime LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT NOBYPASSRLS;
    END IF;
END
$$;

ALTER ROLE business_schema_owner NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT NOBYPASSRLS;
ALTER ROLE business_runtime LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT NOBYPASSRLS;

GRANT business_schema_owner TO core_migrator;

SELECT format('GRANT CREATE ON DATABASE %I TO business_schema_owner', current_database()) \gexec
SELECT format('GRANT CONNECT ON DATABASE %I TO business_runtime', current_database()) \gexec
