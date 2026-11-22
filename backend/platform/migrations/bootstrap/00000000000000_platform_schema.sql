CREATE SCHEMA IF NOT EXISTS platform AUTHORIZATION platform_schema_owner;
REVOKE ALL ON SCHEMA platform FROM PUBLIC;
GRANT USAGE ON SCHEMA platform TO platform_runtime;
ALTER DEFAULT PRIVILEGES FOR ROLE platform_schema_owner IN SCHEMA platform
    GRANT SELECT ON TABLES TO platform_runtime;
