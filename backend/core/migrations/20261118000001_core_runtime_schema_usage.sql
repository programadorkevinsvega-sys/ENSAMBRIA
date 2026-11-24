RESET ROLE;
SET ROLE core_identity_auth_schema_owner;
GRANT USAGE ON SCHEMA core_identity_auth TO core_identity_auth_runtime;

RESET ROLE;
SET ROLE core_tenancy_schema_owner;
GRANT USAGE ON SCHEMA core_tenancy TO core_tenancy_runtime;
