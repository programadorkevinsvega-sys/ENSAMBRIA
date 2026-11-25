-- Read-only catalog audit for the five ENSAMBRIA module schemas.
\set ON_ERROR_STOP on
DO $audit$
DECLARE
    expected_schemas text[] := ARRAY['business_establishment_catalog','core_identity_auth','core_tenancy','platform_product_catalog','platform_tenant_onboarding'];
    expected_owners text[] := ARRAY['business_establishment_catalog_schema_owner','core_identity_auth_schema_owner','core_tenancy_schema_owner','platform_product_catalog_schema_owner','platform_tenant_onboarding_schema_owner'];
    actual text[];
    invalid text[];
BEGIN
    SELECT array_agg(n.nspname ORDER BY n.nspname) INTO actual
      FROM pg_catalog.pg_namespace n WHERE n.nspname = ANY(expected_schemas);
    IF COALESCE(actual, ARRAY[]::text[]) <> expected_schemas THEN
        RAISE EXCEPTION 'expected five module schemas; actual=%', COALESCE(array_to_string(actual, ','), '<none>');
    END IF;
    IF EXISTS (
        SELECT 1 FROM unnest(expected_schemas, expected_owners) AS e(schema_name, owner_name)
        LEFT JOIN pg_catalog.pg_namespace n ON n.nspname=e.schema_name
        LEFT JOIN pg_catalog.pg_roles r ON r.oid=n.nspowner
        WHERE r.rolname IS DISTINCT FROM e.owner_name
    ) THEN RAISE EXCEPTION 'one or more module schemas have an incorrect owner'; END IF;
    SELECT array_agg(r.rolname ORDER BY r.rolname) INTO invalid
      FROM pg_catalog.pg_roles r WHERE r.rolname=ANY(expected_owners)
       AND (r.rolcanlogin OR r.rolsuper OR r.rolcreatedb OR r.rolcreaterole OR r.rolinherit OR r.rolbypassrls);
    IF invalid IS NOT NULL THEN RAISE EXCEPTION 'unsafe schema-owner roles: %', array_to_string(invalid, ','); END IF;
    SELECT array_agg(r.rolname ORDER BY r.rolname) INTO actual
      FROM pg_catalog.pg_auth_members m
      JOIN pg_catalog.pg_roles r ON r.oid=m.roleid
      JOIN pg_catalog.pg_roles member ON member.oid=m.member
     WHERE member.rolname='core_migrator';
    IF COALESCE(actual, ARRAY[]::text[]) <> expected_owners THEN
        RAISE EXCEPTION 'core_migrator membership set is not the five schema owners; actual=%', COALESCE(array_to_string(actual, ','), '<none>');
    END IF;
    SELECT array_agg(r.rolname ORDER BY r.rolname) INTO invalid
      FROM pg_catalog.pg_roles r
     WHERE r.rolname=ANY(ARRAY['core_migrator','core_identity_auth_runtime','core_tenancy_runtime','business_establishment_catalog_runtime','platform_product_catalog_runtime','platform_tenant_onboarding_runtime','core_email_worker','core_outbox_relay'])
       AND (NOT r.rolcanlogin OR r.rolsuper OR r.rolcreatedb OR r.rolcreaterole OR r.rolinherit OR r.rolbypassrls);
    IF invalid IS NOT NULL THEN RAISE EXCEPTION 'unsafe restricted LOGIN roles: %', array_to_string(invalid, ','); END IF;
    IF EXISTS (
        SELECT 1 FROM pg_catalog.pg_roles r
         WHERE r.rolname IN ('core_identity_auth_runtime','core_tenancy_runtime','business_establishment_catalog_runtime','platform_product_catalog_runtime','platform_tenant_onboarding_runtime','core_email_worker','core_outbox_relay')
           AND pg_catalog.pg_has_role(r.oid, 'core_migrator', 'MEMBER')
    ) THEN RAISE EXCEPTION 'runtime or worker role unexpectedly belongs to core_migrator'; END IF;
    RAISE NOTICE 'PostgreSQL module ownership catalog audit passed';
END
$audit$;

SELECT current_database() AS database, current_user AS audit_identity,
       n.nspname AS schema, pg_catalog.pg_get_userbyid(n.nspowner) AS schema_owner
  FROM pg_catalog.pg_namespace n
 WHERE n.nspname=ANY(ARRAY['core_identity_auth','core_tenancy','business_establishment_catalog','platform_product_catalog','platform_tenant_onboarding'])
 ORDER BY n.nspname;

SELECT rolname, rolcanlogin, rolsuper, rolcreatedb, rolcreaterole, rolinherit, rolbypassrls
  FROM pg_catalog.pg_roles
 WHERE rolname=ANY(ARRAY['core_migrator','core_identity_auth_runtime','core_tenancy_runtime','business_establishment_catalog_runtime','platform_product_catalog_runtime','platform_tenant_onboarding_runtime','core_email_worker','core_outbox_relay'])
 ORDER BY rolname;

SELECT member.rolname AS member, granted.rolname AS granted_role,
       m.admin_option, m.inherit_option, m.set_option
  FROM pg_catalog.pg_auth_members m
  JOIN pg_catalog.pg_roles granted ON granted.oid=m.roleid
  JOIN pg_catalog.pg_roles member ON member.oid=m.member
 WHERE member.rolname='core_migrator'
 ORDER BY granted.rolname;
