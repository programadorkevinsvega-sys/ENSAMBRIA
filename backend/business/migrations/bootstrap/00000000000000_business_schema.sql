CREATE SCHEMA IF NOT EXISTS business AUTHORIZATION business_schema_owner;
GRANT USAGE ON SCHEMA core TO business_runtime;
GRANT SELECT ON core.tenant_memberships, core.organizations, core.branches TO business_runtime;
