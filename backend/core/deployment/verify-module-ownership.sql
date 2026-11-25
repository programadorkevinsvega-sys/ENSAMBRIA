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

    IF pg_catalog.has_database_privilege('core_migrator', current_database(), 'CREATE') THEN
        RAISE EXCEPTION 'core_migrator has database CREATE privilege';
    END IF;
    IF EXISTS (
        SELECT 1
          FROM pg_catalog.pg_database d,
               LATERAL pg_catalog.aclexplode(COALESCE(d.datacl, pg_catalog.acldefault('d', d.datdba))) acl
         WHERE d.datname=current_database() AND acl.grantee=0 AND acl.privilege_type='CREATE'
    ) THEN RAISE EXCEPTION 'PUBLIC retains database CREATE privilege'; END IF;

    IF EXISTS (
        SELECT 1 FROM pg_catalog.pg_roles r
         WHERE r.rolname=ANY(ARRAY['core_migrator','core_identity_auth_runtime','core_tenancy_runtime','business_establishment_catalog_runtime','platform_product_catalog_runtime','platform_tenant_onboarding_runtime','core_email_worker','core_outbox_relay'])
           AND NOT pg_catalog.has_database_privilege(r.oid, current_database(), 'CONNECT')
    ) THEN RAISE EXCEPTION 'a required LOGIN role lacks database CONNECT'; END IF;

    IF EXISTS (
        SELECT 1
          FROM pg_catalog.pg_namespace n,
               LATERAL pg_catalog.aclexplode(COALESCE(n.nspacl, pg_catalog.acldefault('n', n.nspowner))) acl
         WHERE n.nspname=ANY(expected_schemas) AND acl.grantee=0
           AND acl.privilege_type IN ('USAGE','CREATE')
    ) THEN RAISE EXCEPTION 'PUBLIC retains module schema privileges'; END IF;

    IF NOT pg_catalog.has_schema_privilege('core_identity_auth_schema_owner', 'core_tenancy', 'USAGE') THEN
        RAISE EXCEPTION 'identity schema owner lacks required tenancy schema USAGE';
    END IF;

    IF EXISTS (
        SELECT 1
          FROM pg_catalog.pg_roles runtime_role
          CROSS JOIN unnest(expected_owners) AS owner_role(role_name)
         WHERE runtime_role.rolname IN (
            'core_identity_auth_runtime','core_tenancy_runtime',
            'business_establishment_catalog_runtime','platform_product_catalog_runtime',
            'platform_tenant_onboarding_runtime','core_email_worker','core_outbox_relay'
         )
           AND pg_catalog.pg_has_role(runtime_role.oid, owner_role.role_name, 'MEMBER')
    ) THEN RAISE EXCEPTION 'a runtime/worker LOGIN role is a member of a schema-owner role'; END IF;

    IF EXISTS (
        SELECT 1
          FROM pg_catalog.pg_roles runtime_role
          CROSS JOIN unnest(expected_schemas) AS module_schema(schema_name)
         WHERE runtime_role.rolname IN (
            'core_identity_auth_runtime','core_tenancy_runtime',
            'business_establishment_catalog_runtime','platform_product_catalog_runtime',
            'platform_tenant_onboarding_runtime','core_email_worker','core_outbox_relay'
         )
           AND pg_catalog.has_schema_privilege(runtime_role.oid, module_schema.schema_name, 'CREATE')
    ) THEN RAISE EXCEPTION 'a runtime/worker LOGIN role has CREATE on a module schema'; END IF;

    IF pg_catalog.pg_has_role('core_migrator', 'core_identity_auth_schema_owner', 'USAGE')
       OR pg_catalog.pg_has_role('core_migrator', 'core_tenancy_schema_owner', 'USAGE')
       OR pg_catalog.pg_has_role('core_migrator', 'business_establishment_catalog_schema_owner', 'USAGE')
       OR pg_catalog.pg_has_role('core_migrator', 'platform_product_catalog_schema_owner', 'USAGE')
       OR pg_catalog.pg_has_role('core_migrator', 'platform_tenant_onboarding_schema_owner', 'USAGE') THEN
        RAISE EXCEPTION 'NOINHERIT core_migrator unexpectedly inherits schema-owner privileges without SET ROLE';
    END IF;

    RAISE NOTICE 'PostgreSQL module ownership and effective privilege audit passed';
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
