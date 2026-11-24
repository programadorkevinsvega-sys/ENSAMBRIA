SET ROLE business_schema_owner;
CREATE SCHEMA IF NOT EXISTS business AUTHORIZATION business_schema_owner;
RESET ROLE;

SET ROLE core_schema_owner;
GRANT USAGE ON SCHEMA core TO business_runtime;
GRANT SELECT ON core.tenant_memberships, core.organizations, core.branches TO business_runtime;
RESET ROLE;
