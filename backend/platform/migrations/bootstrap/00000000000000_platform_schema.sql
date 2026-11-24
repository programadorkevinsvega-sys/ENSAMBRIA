SET ROLE platform_product_catalog_schema_owner;
CREATE SCHEMA IF NOT EXISTS platform_product_catalog
    AUTHORIZATION platform_product_catalog_schema_owner;
RESET ROLE;

SET ROLE platform_tenant_onboarding_schema_owner;
CREATE SCHEMA IF NOT EXISTS platform_tenant_onboarding
    AUTHORIZATION platform_tenant_onboarding_schema_owner;
RESET ROLE;
