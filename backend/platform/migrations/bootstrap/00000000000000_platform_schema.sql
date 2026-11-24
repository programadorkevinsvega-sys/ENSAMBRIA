DO $ownership$
DECLARE
    product_owner text;
    onboarding_owner text;
BEGIN
    SELECT pg_get_userbyid(nspowner)
      INTO product_owner
      FROM pg_namespace
     WHERE nspname = 'platform_product_catalog';
    SELECT pg_get_userbyid(nspowner)
      INTO onboarding_owner
      FROM pg_namespace
     WHERE nspname = 'platform_tenant_onboarding';

    IF product_owner IS DISTINCT FROM 'platform_product_catalog_schema_owner' THEN
        RAISE EXCEPTION 'PLATFORM product catalog schema must be pre-provisioned by its owner';
    END IF;
    IF onboarding_owner IS DISTINCT FROM 'platform_tenant_onboarding_schema_owner' THEN
        RAISE EXCEPTION 'PLATFORM tenant onboarding schema must be pre-provisioned by its owner';
    END IF;
END
$ownership$;
