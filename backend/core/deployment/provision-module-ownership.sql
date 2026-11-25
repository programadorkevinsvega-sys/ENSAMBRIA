-- Idempotently provision five non-login schema owners and verify existing login identities.
-- Run only through the PostgreSQL administrative channel, against the intended database.
-- Login passwords are secret-managed and are deliberately never created or rotated here.
DO $guard$
DECLARE
    role_info record;
BEGIN
    SELECT rolsuper, rolcreaterole
      INTO role_info
      FROM pg_catalog.pg_roles
     WHERE rolname = current_user;

    IF NOT FOUND OR (NOT role_info.rolsuper AND NOT role_info.rolcreaterole) THEN
        RAISE EXCEPTION 'module ownership provisioning requires a PostgreSQL administrator'
            USING ERRCODE = '42501';
    END IF;
END
$guard$;

DO $login_preflight$
DECLARE
    missing_roles text[];
    invalid_roles text[];
BEGIN
    SELECT array_agg(required.role_name ORDER BY required.role_name)
      INTO missing_roles
      FROM unnest(ARRAY[
          'core_migrator',
          'core_identity_auth_runtime',
          'core_tenancy_runtime',
          'business_establishment_catalog_runtime',
          'platform_product_catalog_runtime',
          'platform_tenant_onboarding_runtime',
          'core_email_worker',
          'core_outbox_relay'
      ]::text[]) AS required(role_name)
      LEFT JOIN pg_catalog.pg_roles role ON role.rolname = required.role_name
     WHERE role.rolname IS NULL;

    IF missing_roles IS NOT NULL THEN
        RAISE EXCEPTION 'required secret-managed PostgreSQL login roles are missing'
            USING ERRCODE = '42704',
                  DETAIL = 'missing roles: ' || array_to_string(missing_roles, ',');
    END IF;

    SELECT array_agg(role.rolname ORDER BY role.rolname)
      INTO invalid_roles
      FROM pg_catalog.pg_roles role
     WHERE role.rolname = ANY(ARRAY[
          'core_migrator',
          'core_identity_auth_runtime',
          'core_tenancy_runtime',
          'business_establishment_catalog_runtime',
          'platform_product_catalog_runtime',
          'platform_tenant_onboarding_runtime',
          'core_email_worker',
          'core_outbox_relay'
     ])
       AND (NOT role.rolcanlogin OR role.rolsuper OR role.rolcreatedb
            OR role.rolcreaterole OR role.rolinherit OR role.rolbypassrls);

    IF invalid_roles IS NOT NULL THEN
        RAISE EXCEPTION 'secret-managed PostgreSQL login roles have invalid attributes'
            USING ERRCODE = '42501',
                  DETAIL = 'invalid roles: ' || array_to_string(invalid_roles, ',');
    END IF;
END
$login_preflight$;

DO $roles$
DECLARE
    role_name text;
    owner_roles text[] := ARRAY[
        'core_identity_auth_schema_owner',
        'core_tenancy_schema_owner',
        'business_establishment_catalog_schema_owner',
        'platform_product_catalog_schema_owner',
        'platform_tenant_onboarding_schema_owner'
    ];
BEGIN
    FOREACH role_name IN ARRAY owner_roles LOOP
        IF NOT EXISTS (
            SELECT 1 FROM pg_catalog.pg_roles WHERE rolname = role_name
        ) THEN
            EXECUTE format(
                'CREATE ROLE %I NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT NOBYPASSRLS',
                role_name
            );
        END IF;

        EXECUTE format(
            'ALTER ROLE %I NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT NOBYPASSRLS',
            role_name
        );
    END LOOP;
END
$roles$;

-- The migrator can assume only schema-owner identities. It is not a member of
-- any runtime or worker login role.
GRANT core_identity_auth_schema_owner TO core_migrator;
GRANT core_tenancy_schema_owner TO core_migrator;
GRANT business_establishment_catalog_schema_owner TO core_migrator;
GRANT platform_product_catalog_schema_owner TO core_migrator;
GRANT platform_tenant_onboarding_schema_owner TO core_migrator;

SELECT format(
    'GRANT CONNECT ON DATABASE %I TO core_migrator, core_identity_auth_runtime, core_tenancy_runtime, business_establishment_catalog_runtime, platform_product_catalog_runtime, platform_tenant_onboarding_runtime, core_email_worker, core_outbox_relay',
    current_database()
) \gexec

SELECT format(
    'REVOKE CREATE ON DATABASE %I FROM PUBLIC, core_migrator',
    current_database()
) \gexec

CREATE SCHEMA IF NOT EXISTS core_identity_auth AUTHORIZATION core_identity_auth_schema_owner;
CREATE SCHEMA IF NOT EXISTS core_tenancy AUTHORIZATION core_tenancy_schema_owner;
CREATE SCHEMA IF NOT EXISTS business_establishment_catalog AUTHORIZATION business_establishment_catalog_schema_owner;
CREATE SCHEMA IF NOT EXISTS platform_product_catalog AUTHORIZATION platform_product_catalog_schema_owner;
CREATE SCHEMA IF NOT EXISTS platform_tenant_onboarding AUTHORIZATION platform_tenant_onboarding_schema_owner;

ALTER SCHEMA core_identity_auth OWNER TO core_identity_auth_schema_owner;
ALTER SCHEMA core_tenancy OWNER TO core_tenancy_schema_owner;
ALTER SCHEMA business_establishment_catalog OWNER TO business_establishment_catalog_schema_owner;
ALTER SCHEMA platform_product_catalog OWNER TO platform_product_catalog_schema_owner;
ALTER SCHEMA platform_tenant_onboarding OWNER TO platform_tenant_onboarding_schema_owner;

REVOKE ALL ON SCHEMA
    core_identity_auth,
    core_tenancy,
    business_establishment_catalog,
    platform_product_catalog,
    platform_tenant_onboarding
FROM PUBLIC;

-- CORE identity DDL references the tenancy-owned security namespace. This grants
-- namespace resolution to the identity schema owner only, not tenancy data access.
GRANT USAGE ON SCHEMA core_tenancy TO core_identity_auth_schema_owner;

-- Runtime roles intentionally receive no schema USAGE/CREATE here. Each module's
-- migrations grant only the exact schema/table/sequence/function privileges required.


-- Fail closed if the catalog does not match the intended five-owner boundary.
DO $verify$
DECLARE
    actual_memberships text[];
    expected_memberships constant text[] := ARRAY[
        'business_establishment_catalog_schema_owner',
        'core_identity_auth_schema_owner',
        'core_tenancy_schema_owner',
        'platform_product_catalog_schema_owner',
        'platform_tenant_onboarding_schema_owner'
    ];
    invalid_roles text[];
BEGIN
    SELECT array_agg(role.rolname ORDER BY role.rolname)
      INTO actual_memberships
      FROM pg_catalog.pg_auth_members membership
      JOIN pg_catalog.pg_roles role ON role.oid = membership.roleid
      JOIN pg_catalog.pg_roles member ON member.oid = membership.member
     WHERE member.rolname = 'core_migrator';

    IF COALESCE(actual_memberships, ARRAY[]::text[]) <> expected_memberships THEN
        RAISE EXCEPTION 'core_migrator membership set is not the five schema-owner roles'
            USING ERRCODE = '42501',
                  DETAIL = 'actual memberships: ' || COALESCE(array_to_string(actual_memberships, ','), '<none>');
    END IF;

    SELECT array_agg(required.role_name ORDER BY required.role_name)
      INTO invalid_roles
      FROM (
          SELECT unnest(ARRAY[
              'core_migrator',
              'core_identity_auth_runtime',
              'core_tenancy_runtime',
              'business_establishment_catalog_runtime',
              'platform_product_catalog_runtime',
              'platform_tenant_onboarding_runtime',
              'core_email_worker',
              'core_outbox_relay'
          ]) AS role_name
      ) required
      JOIN pg_catalog.pg_roles role ON role.rolname = required.role_name
     WHERE NOT role.rolcanlogin
        OR role.rolsuper
        OR role.rolcreatedb
        OR role.rolcreaterole
        OR role.rolinherit
        OR role.rolbypassrls;

    IF invalid_roles IS NOT NULL THEN
        RAISE EXCEPTION 'restricted PostgreSQL login roles have invalid attributes'
            USING ERRCODE = '42501',
                  DETAIL = 'invalid roles: ' || array_to_string(invalid_roles, ',');
    END IF;

    IF EXISTS (
        SELECT 1
          FROM pg_catalog.pg_roles role
         WHERE role.rolname IN (
             'core_identity_auth_schema_owner',
             'core_tenancy_schema_owner',
             'business_establishment_catalog_schema_owner',
             'platform_product_catalog_schema_owner',
             'platform_tenant_onboarding_schema_owner'
         )
           AND (role.rolcanlogin OR role.rolsuper OR role.rolcreatedb
                OR role.rolcreaterole OR role.rolinherit OR role.rolbypassrls)
    ) THEN
        RAISE EXCEPTION 'a PostgreSQL schema-owner role has invalid attributes'
            USING ERRCODE = '42501';
    END IF;

    IF EXISTS (
        SELECT 1
          FROM (VALUES
              ('core_identity_auth', 'core_identity_auth_schema_owner'),
              ('core_tenancy', 'core_tenancy_schema_owner'),
              ('business_establishment_catalog', 'business_establishment_catalog_schema_owner'),
              ('platform_product_catalog', 'platform_product_catalog_schema_owner'),
              ('platform_tenant_onboarding', 'platform_tenant_onboarding_schema_owner')
          ) expected(schema_name, owner_name)
          LEFT JOIN pg_catalog.pg_namespace namespace
            ON namespace.nspname = expected.schema_name
          LEFT JOIN pg_catalog.pg_roles owner_role
            ON owner_role.oid = namespace.nspowner
         WHERE owner_role.rolname IS DISTINCT FROM expected.owner_name
    ) THEN
        RAISE EXCEPTION 'PostgreSQL module schema ownership verification failed'
            USING ERRCODE = '42501';
    END IF;
END
$verify$;
