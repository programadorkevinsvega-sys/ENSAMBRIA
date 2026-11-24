#!/usr/bin/env bash
set -euo pipefail

: "${TEST_DATABASE_URL:?TEST_DATABASE_URL is required}"
psql "$TEST_DATABASE_URL" -v ON_ERROR_STOP=1 <<'SQL'
DO $$
DECLARE
    pair record;
    usage_allowed boolean;
BEGIN
    FOR pair IN
        SELECT * FROM (VALUES
            ('core_identity_auth_runtime', 'core_identity_auth'),
            ('core_tenancy_runtime', 'core_tenancy'),
            ('business_establishment_catalog_runtime', 'business_establishment_catalog'),
            ('platform_product_catalog_runtime', 'platform_product_catalog'),
            ('platform_tenant_onboarding_runtime', 'platform_tenant_onboarding')
        ) AS expected(role_name, schema_name)
    LOOP
        SELECT has_schema_privilege(pair.role_name, pair.schema_name, 'USAGE')
          INTO usage_allowed;
        IF usage_allowed THEN
            RAISE EXCEPTION 'runtime role % unexpectedly has USAGE on % before cutover',
                pair.role_name, pair.schema_name;
        END IF;
    END LOOP;

    IF has_schema_privilege('business_establishment_catalog_runtime', 'core_tenancy', 'USAGE')
       OR has_schema_privilege('business_establishment_catalog_runtime', 'core_identity_auth', 'USAGE')
       OR has_schema_privilege('platform_tenant_onboarding_runtime', 'platform_product_catalog', 'USAGE')
       OR has_schema_privilege('platform_tenant_onboarding_runtime', 'core_tenancy', 'USAGE')
    THEN
        RAISE EXCEPTION 'cross-module schema access was granted';
    END IF;

    IF has_schema_privilege('public', 'core_identity_auth', 'USAGE')
       OR has_schema_privilege('public', 'core_tenancy', 'USAGE')
       OR has_schema_privilege('public', 'business_establishment_catalog', 'USAGE')
       OR has_schema_privilege('public', 'platform_product_catalog', 'USAGE')
       OR has_schema_privilege('public', 'platform_tenant_onboarding', 'USAGE')
    THEN
        RAISE EXCEPTION 'PUBLIC unexpectedly has access to a module schema';
    END IF;
END
$$;
SQL
