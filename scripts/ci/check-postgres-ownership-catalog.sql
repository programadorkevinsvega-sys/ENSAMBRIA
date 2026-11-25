-- Run after all five module migrations. This is an authoritative catalog-level
-- check: effective privileges include PUBLIC, inherited memberships and object ACLs.
DO $ownership_catalog$
DECLARE
    expected record;
    actual_owner text;
    role_record record;
    object_record record;
    expected_policy_count integer;
    actual_policy_count integer;
BEGIN
    FOR expected IN
        SELECT * FROM (VALUES
            ('core_identity_auth', 'core_identity_auth_schema_owner', 'core_identity_auth_runtime'),
            ('core_tenancy', 'core_tenancy_schema_owner', 'core_tenancy_runtime'),
            ('business_establishment_catalog', 'business_establishment_catalog_schema_owner', 'business_establishment_catalog_runtime'),
            ('platform_product_catalog', 'platform_product_catalog_schema_owner', 'platform_product_catalog_runtime'),
            ('platform_tenant_onboarding', 'platform_tenant_onboarding_schema_owner', 'platform_tenant_onboarding_runtime')
        ) AS modules(schema_name, owner_role, runtime_role)
    LOOP
        SELECT pg_get_userbyid(nspowner)
          INTO actual_owner
          FROM pg_namespace
         WHERE nspname = expected.schema_name;
        IF actual_owner IS DISTINCT FROM expected.owner_role THEN
            RAISE EXCEPTION 'schema % owner is %, expected %',
                expected.schema_name, actual_owner, expected.owner_role;
        END IF;

        IF NOT EXISTS (
            SELECT 1 FROM pg_roles
             WHERE rolname = expected.owner_role
               AND rolcanlogin = false
               AND rolinherit = false
               AND rolsuper = false
               AND rolcreatedb = false
               AND rolcreaterole = false
               AND rolbypassrls = false
        ) THEN
            RAISE EXCEPTION 'owner role % must be NOLOGIN, NOINHERIT, NOSUPERUSER, NOCREATEDB, NOCREATEROLE and NOBYPASSRLS',
                expected.owner_role;
        END IF;

        IF NOT EXISTS (
            SELECT 1 FROM pg_roles
             WHERE rolname = expected.runtime_role
               AND rolcanlogin = true
               AND rolinherit = false
               AND rolsuper = false
               AND rolcreatedb = false
               AND rolcreaterole = false
               AND rolbypassrls = false
        ) THEN
            RAISE EXCEPTION 'runtime role % must be LOGIN, NOINHERIT, NOSUPERUSER, NOCREATEDB, NOCREATEROLE and NOBYPASSRLS',
                expected.runtime_role;
        END IF;

        IF NOT has_schema_privilege(expected.runtime_role, expected.schema_name, 'USAGE')
           OR has_schema_privilege(expected.runtime_role, expected.schema_name, 'CREATE')
           OR has_database_privilege(expected.runtime_role, current_database(), 'CREATE')
           OR has_database_privilege(expected.runtime_role, current_database(), 'TEMP') THEN
            RAISE EXCEPTION 'runtime role % has missing own-schema USAGE or excessive DDL/TEMP privileges',
                expected.runtime_role;
        END IF;

        IF pg_has_role(expected.runtime_role, expected.owner_role, 'MEMBER')
           OR pg_has_role(expected.runtime_role, 'core_migrator', 'MEMBER') THEN
            RAISE EXCEPTION 'runtime role % can assume owner or migrator membership',
                expected.runtime_role;
        END IF;

        FOR role_record IN
            SELECT nspname
              FROM pg_namespace
             WHERE nspname IN (
                 'core_identity_auth', 'core_tenancy',
                 'business_establishment_catalog',
                 'platform_product_catalog', 'platform_tenant_onboarding'
             )
        LOOP
            IF role_record.nspname <> expected.schema_name
               AND (
                   has_schema_privilege(expected.runtime_role, role_record.nspname, 'USAGE')
                   OR has_schema_privilege(expected.runtime_role, role_record.nspname, 'CREATE')
               ) THEN
                RAISE EXCEPTION 'runtime role % has cross-module schema privilege on %',
                    expected.runtime_role, role_record.nspname;
            END IF;
        END LOOP;

        IF EXISTS (
            SELECT 1
              FROM pg_class c
              JOIN pg_namespace n ON n.oid = c.relnamespace
             WHERE n.nspname IN (
                 'core_identity_auth', 'core_tenancy',
                 'business_establishment_catalog',
                 'platform_product_catalog', 'platform_tenant_onboarding'
             )
               AND n.nspname <> expected.schema_name
               AND c.relkind IN ('r', 'p', 'v', 'm', 'f')
               AND (
                   has_table_privilege(expected.runtime_role, c.oid, 'SELECT')
                   OR has_table_privilege(expected.runtime_role, c.oid, 'INSERT')
                   OR has_table_privilege(expected.runtime_role, c.oid, 'UPDATE')
                   OR has_table_privilege(expected.runtime_role, c.oid, 'DELETE')
                   OR has_table_privilege(expected.runtime_role, c.oid, 'TRUNCATE')
                   OR has_table_privilege(expected.runtime_role, c.oid, 'REFERENCES')
                   OR has_table_privilege(expected.runtime_role, c.oid, 'TRIGGER')
               )
        ) THEN
            RAISE EXCEPTION 'runtime role % has effective cross-module relation privileges',
                expected.runtime_role;
        END IF;

        IF EXISTS (
            SELECT 1
              FROM pg_class c
              JOIN pg_namespace n ON n.oid = c.relnamespace
             WHERE n.nspname IN (
                 'core_identity_auth', 'core_tenancy',
                 'business_establishment_catalog',
                 'platform_product_catalog', 'platform_tenant_onboarding'
             )
               AND n.nspname <> expected.schema_name
               AND c.relkind = 'S'
               AND (
                   has_sequence_privilege(expected.runtime_role, c.oid, 'USAGE')
                   OR has_sequence_privilege(expected.runtime_role, c.oid, 'SELECT')
                   OR has_sequence_privilege(expected.runtime_role, c.oid, 'UPDATE')
               )
        ) THEN
            RAISE EXCEPTION 'runtime role % has effective cross-module sequence privileges',
                expected.runtime_role;
        END IF;

        IF EXISTS (
            SELECT 1
              FROM pg_proc p
              JOIN pg_namespace n ON n.oid = p.pronamespace
             WHERE n.nspname IN (
                 'core_identity_auth', 'core_tenancy',
                 'business_establishment_catalog',
                 'platform_product_catalog', 'platform_tenant_onboarding'
             )
               AND n.nspname <> expected.schema_name
               AND has_function_privilege(expected.runtime_role, p.oid, 'EXECUTE')
        ) THEN
            RAISE EXCEPTION 'runtime role % has effective cross-module function EXECUTE',
                expected.runtime_role;
        END IF;
    END LOOP;

    IF NOT EXISTS (
        SELECT 1 FROM pg_roles
         WHERE rolname = 'core_migrator'
           AND rolcanlogin = true
           AND rolinherit = false
           AND rolsuper = false
           AND rolcreatedb = false
           AND rolcreaterole = false
           AND rolbypassrls = false
    ) THEN
        RAISE EXCEPTION 'core_migrator must be LOGIN, NOINHERIT and non-superuser/non-BYPASSRLS';
    END IF;

    IF (
        SELECT count(*)
          FROM pg_auth_members m
          JOIN pg_roles member_role ON member_role.oid = m.member
          JOIN pg_roles granted_role ON granted_role.oid = m.roleid
         WHERE member_role.rolname = 'core_migrator'
           AND granted_role.rolname IN (
               'core_identity_auth_schema_owner',
               'core_tenancy_schema_owner',
               'business_establishment_catalog_schema_owner',
               'platform_product_catalog_schema_owner',
               'platform_tenant_onboarding_schema_owner'
           )
           AND NOT m.inherit_option
           AND m.set_option
    ) <> 5 THEN
        RAISE EXCEPTION 'core_migrator must have exactly five explicit NOINHERIT/SET ROLE owner memberships';
    END IF;

    IF EXISTS (
        SELECT 1
          FROM pg_auth_members m
          JOIN pg_roles member_role ON member_role.oid = m.member
          JOIN pg_roles granted_role ON granted_role.oid = m.roleid
         WHERE member_role.rolname = 'core_migrator'
           AND granted_role.rolname IN (
               'core_identity_auth_schema_owner',
               'core_tenancy_schema_owner',
               'business_establishment_catalog_schema_owner',
               'platform_product_catalog_schema_owner',
               'platform_tenant_onboarding_schema_owner'
           )
           AND m.inherit_option
    ) THEN
        RAISE EXCEPTION 'core_migrator must not inherit schema-owner privileges';
    END IF;

    IF EXISTS (
        SELECT 1
          FROM pg_namespace n
          CROSS JOIN LATERAL aclexplode(COALESCE(n.nspacl, acldefault('n', n.nspowner))) acl
         WHERE n.nspname IN (
             'core_identity_auth', 'core_tenancy',
             'business_establishment_catalog',
             'platform_product_catalog', 'platform_tenant_onboarding'
         )
           AND acl.grantee = 0
           AND acl.privilege_type IN ('USAGE', 'CREATE')
    ) THEN
        RAISE EXCEPTION 'PUBLIC has USAGE or CREATE on a module schema';
    END IF;

    IF EXISTS (
        SELECT 1
          FROM pg_class c
          JOIN pg_namespace n ON n.oid = c.relnamespace
          CROSS JOIN LATERAL aclexplode(COALESCE(c.relacl, acldefault(
              CASE WHEN c.relkind = 'S' THEN 'S' ELSE 'r' END, c.relowner
          ))) acl
         WHERE n.nspname IN (
             'core_identity_auth', 'core_tenancy',
             'business_establishment_catalog',
             'platform_product_catalog', 'platform_tenant_onboarding'
         )
           AND acl.grantee = 0
    ) THEN
        RAISE EXCEPTION 'PUBLIC has explicit privileges on module relations/sequences';
    END IF;

    IF EXISTS (
        SELECT 1
          FROM pg_proc p
          JOIN pg_namespace n ON n.oid = p.pronamespace
          CROSS JOIN LATERAL aclexplode(COALESCE(p.proacl, acldefault('f', p.proowner))) acl
         WHERE n.nspname IN (
             'core_identity_auth', 'core_tenancy',
             'business_establishment_catalog',
             'platform_product_catalog', 'platform_tenant_onboarding'
         )
           AND acl.grantee = 0
           AND acl.privilege_type = 'EXECUTE'
    ) THEN
        RAISE EXCEPTION 'PUBLIC retains EXECUTE on a module function';
    END IF;

    IF EXISTS (
        SELECT 1
          FROM pg_class c
          JOIN pg_namespace n ON n.oid = c.relnamespace
          JOIN pg_roles owner_role ON owner_role.oid = c.relowner
         WHERE n.nspname IN (
             'core_identity_auth', 'core_tenancy',
             'business_establishment_catalog',
             'platform_product_catalog', 'platform_tenant_onboarding'
         )
           AND owner_role.rolname <> CASE n.nspname
               WHEN 'core_identity_auth' THEN 'core_identity_auth_schema_owner'
               WHEN 'core_tenancy' THEN 'core_tenancy_schema_owner'
               WHEN 'business_establishment_catalog' THEN 'business_establishment_catalog_schema_owner'
               WHEN 'platform_product_catalog' THEN 'platform_product_catalog_schema_owner'
               WHEN 'platform_tenant_onboarding' THEN 'platform_tenant_onboarding_schema_owner'
           END
           AND c.relkind IN ('r', 'p', 'v', 'm', 'S', 'f')
    ) THEN
        RAISE EXCEPTION 'a module relation, view or sequence is not owned by its canonical schema owner';
    END IF;

    IF EXISTS (
        SELECT 1
          FROM pg_proc p
          JOIN pg_namespace n ON n.oid = p.pronamespace
          JOIN pg_roles owner_role ON owner_role.oid = p.proowner
         WHERE n.nspname IN (
             'core_identity_auth', 'core_tenancy',
             'business_establishment_catalog',
             'platform_product_catalog', 'platform_tenant_onboarding'
         )
           AND owner_role.rolname <> CASE n.nspname
               WHEN 'core_identity_auth' THEN 'core_identity_auth_schema_owner'
               WHEN 'core_tenancy' THEN 'core_tenancy_schema_owner'
               WHEN 'business_establishment_catalog' THEN 'business_establishment_catalog_schema_owner'
               WHEN 'platform_product_catalog' THEN 'platform_product_catalog_schema_owner'
               WHEN 'platform_tenant_onboarding' THEN 'platform_tenant_onboarding_schema_owner'
           END
    ) THEN
        RAISE EXCEPTION 'a module function is not owned by its canonical schema owner';
    END IF;

    IF EXISTS (
        SELECT 1
          FROM pg_constraint fk
          JOIN pg_class source_table ON source_table.oid = fk.conrelid
          JOIN pg_namespace source_schema ON source_schema.oid = source_table.relnamespace
          JOIN pg_class target_table ON target_table.oid = fk.confrelid
          JOIN pg_namespace target_schema ON target_schema.oid = target_table.relnamespace
         WHERE fk.contype = 'f'
           AND source_schema.nspname IN (
               'core_identity_auth', 'core_tenancy',
               'business_establishment_catalog',
               'platform_product_catalog', 'platform_tenant_onboarding'
           )
           AND target_schema.nspname IN (
               'core_identity_auth', 'core_tenancy',
               'business_establishment_catalog',
               'platform_product_catalog', 'platform_tenant_onboarding'
           )
           AND source_schema.nspname <> target_schema.nspname
    ) THEN
        RAISE EXCEPTION 'cross-module PostgreSQL foreign keys violate opaque-reference ownership';
    END IF;

    -- Every tenant/actor-scoped relation must enforce RLS even for its owner.
    FOR object_record IN
        SELECT * FROM (VALUES
            ('core_identity_auth', 'identities'),
            ('core_identity_auth', 'identity_credentials'),
            ('core_identity_auth', 'auth_sessions'),
            ('core_identity_auth', 'account_recovery_tokens'),
            ('core_identity_auth', 'identity_verification_tokens'),
            ('core_identity_auth', 'identity_mfa_totp'),
            ('core_identity_auth', 'identity_mfa_recovery_codes'),
            ('core_identity_auth', 'auth_rate_limits'),
            ('core_identity_auth', 'auth_mfa_challenges'),
            ('core_identity_auth', 'security_audit_events'),
            ('core_tenancy', 'tenants'),
            ('core_tenancy', 'organizations'),
            ('core_tenancy', 'branches'),
            ('core_tenancy', 'tenant_memberships'),
            ('core_tenancy', 'tenant_roles'),
            ('core_tenancy', 'tenant_role_permissions'),
            ('core_tenancy', 'membership_role_assignments'),
            ('core_tenancy', 'tenant_products'),
            ('core_tenancy', 'outbox_events'),
            ('business_establishment_catalog', 'establishments'),
            ('platform_product_catalog', 'saas_subscriptions'),
            ('platform_product_catalog', 'saas_subscription_audit'),
            ('platform_product_catalog', 'saas_subscription_commands'),
            ('platform_product_catalog', 'saas_subscription_outbox'),
            ('platform_tenant_onboarding', 'inbox_messages'),
            ('platform_tenant_onboarding', 'tenant_onboardings')
        ) AS expected(schema_name, table_name)
    LOOP
        IF NOT EXISTS (
            SELECT 1
              FROM pg_class c
              JOIN pg_namespace n ON n.oid = c.relnamespace
             WHERE n.nspname = object_record.schema_name
               AND c.relname = object_record.table_name
               AND c.relkind IN ('r', 'p')
               AND c.relrowsecurity
               AND c.relforcerowsecurity
        ) THEN
            RAISE EXCEPTION 'tenant/actor-scoped relation %.% must ENABLE and FORCE RLS',
                object_record.schema_name, object_record.table_name;
        END IF;
    END LOOP;

    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'platform_product_catalog'
          AND c.relname = 'commercial_offers'
          AND c.relrowsecurity
    ) OR NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'platform_product_catalog'
          AND c.relname = 'commercial_offer_versions'
          AND c.relrowsecurity
    ) THEN
        RAISE EXCEPTION 'published commercial offer tables must have RLS enabled';
    END IF;
END
$ownership_catalog$;

-- Exact policy reconciliation. This catches absent and unexpected policies,
-- including policies expanded from PL/pgSQL FOREACH/EXECUTE format() loops.
DO $policy_catalog$
DECLARE
    mismatch text;
BEGIN
    WITH expected(schema_name, table_name, policy_name) AS (
        VALUES
            ('core_identity_auth', 'identities', 'identities_actor_scope'),
            ('core_identity_auth', 'security_audit_events', 'security_audit_scope'),
            ('core_identity_auth', 'account_recovery_tokens', 'account_recovery_tokens_actor_scope'),
            ('core_identity_auth', 'identity_verification_tokens', 'identity_verification_tokens_actor_scope'),
            ('core_identity_auth', 'identity_credentials', 'identity_credentials_actor_scope'),
            ('core_identity_auth', 'auth_sessions', 'auth_sessions_actor_scope'),
            ('core_identity_auth', 'identity_mfa_totp', 'identity_mfa_totp_actor_scope'),
            ('core_identity_auth', 'identity_mfa_recovery_codes', 'identity_mfa_recovery_codes_actor_scope'),
            ('core_identity_auth', 'identities', 'identities_definer_owner_access'),
            ('core_identity_auth', 'identity_credentials', 'identity_credentials_definer_owner_access'),
            ('core_identity_auth', 'auth_sessions', 'auth_sessions_definer_owner_access'),
            ('core_identity_auth', 'account_recovery_tokens', 'account_recovery_tokens_definer_owner_access'),
            ('core_identity_auth', 'identity_verification_tokens', 'identity_verification_tokens_definer_owner_access'),
            ('core_identity_auth', 'identity_mfa_totp', 'identity_mfa_totp_definer_owner_access'),
            ('core_identity_auth', 'identity_mfa_recovery_codes', 'identity_mfa_recovery_codes_definer_owner_access'),
            ('core_identity_auth', 'auth_rate_limits', 'auth_rate_limits_definer_owner_access'),
            ('core_identity_auth', 'auth_mfa_challenges', 'auth_mfa_challenges_definer_only'),
            ('core_identity_auth', 'security_audit_events', 'security_audit_events_definer_access'),
            ('core_tenancy', 'tenants', 'tenants_tenant_scope'),
            ('core_tenancy', 'tenants', 'tenants_definer_owner_access'),
            ('core_tenancy', 'organizations', 'organizations_tenant_scope'),
            ('core_tenancy', 'branches', 'branches_tenant_scope'),
            ('core_tenancy', 'tenant_memberships', 'tenant_memberships_tenant_scope'),
            ('core_tenancy', 'tenant_roles', 'tenant_roles_tenant_scope'),
            ('core_tenancy', 'tenant_role_permissions', 'tenant_role_permissions_tenant_scope'),
            ('core_tenancy', 'membership_role_assignments', 'membership_role_assignments_tenant_scope'),
            ('core_tenancy', 'tenant_products', 'tenant_products_tenant_scope'),
            ('core_tenancy', 'organizations', 'organizations_definer_owner_access'),
            ('core_tenancy', 'branches', 'branches_definer_owner_access'),
            ('core_tenancy', 'tenant_memberships', 'tenant_memberships_definer_owner_access'),
            ('core_tenancy', 'tenant_roles', 'tenant_roles_definer_owner_access'),
            ('core_tenancy', 'tenant_role_permissions', 'tenant_role_permissions_definer_owner_access'),
            ('core_tenancy', 'membership_role_assignments', 'membership_role_assignments_definer_owner_access'),
            ('core_tenancy', 'tenant_products', 'tenant_products_definer_owner_access'),
            ('core_tenancy', 'outbox_events', 'outbox_tenant_insert'),
            ('core_tenancy', 'outbox_events', 'outbox_relay_read'),
            ('core_tenancy', 'outbox_events', 'outbox_relay_update'),
            ('business_establishment_catalog', 'establishments', 'establishments_tenant_scope'),
            ('platform_product_catalog', 'commercial_offers', 'commercial_offers_read_published'),
            ('platform_product_catalog', 'commercial_offer_versions', 'commercial_offer_versions_read_published'),
            ('platform_product_catalog', 'saas_subscriptions', 'tenant_scope'),
            ('platform_product_catalog', 'saas_subscription_audit', 'tenant_scope'),
            ('platform_product_catalog', 'saas_subscription_commands', 'tenant_scope'),
            ('platform_product_catalog', 'saas_subscription_outbox', 'tenant_scope'),
            ('platform_tenant_onboarding', 'inbox_messages', 'inbox_tenant_scope'),
            ('platform_tenant_onboarding', 'tenant_onboardings', 'tenant_onboardings_tenant_scope')
    ),
    actual AS (
        SELECT schemaname, tablename, policyname
          FROM pg_policies
         WHERE schemaname IN (
             'core_identity_auth', 'core_tenancy',
             'business_establishment_catalog',
             'platform_product_catalog', 'platform_tenant_onboarding'
         )
    ),
    missing AS (
        SELECT schema_name, table_name, policy_name FROM expected
        EXCEPT
        SELECT schemaname, tablename, policyname FROM actual
    ),
    unexpected AS (
        SELECT schemaname, tablename, policyname FROM actual
        EXCEPT
        SELECT schema_name, table_name, policy_name FROM expected
    )
    SELECT string_agg(problem, E'\n')
      INTO mismatch
      FROM (
          SELECT format('missing policy %I.%I.%I', schema_name, table_name, policy_name) AS problem
            FROM missing
          UNION ALL
          SELECT format('unexpected policy %I.%I.%I', schemaname, tablename, policyname)
            FROM unexpected
      ) problems;

    IF mismatch IS NOT NULL THEN
        RAISE EXCEPTION 'PostgreSQL policy catalog differs from canonical inventory:%', E'\n' || mismatch;
    END IF;
END
$policy_catalog$;

DO $worker_catalog$
BEGIN
    IF NOT has_schema_privilege('core_email_worker', 'core_identity_auth', 'USAGE')
       OR NOT has_schema_privilege('core_outbox_relay', 'core_tenancy', 'USAGE')
       OR has_schema_privilege('core_email_worker', 'core_tenancy', 'USAGE')
       OR has_schema_privilege('core_outbox_relay', 'core_identity_auth', 'USAGE')
       OR pg_has_role('core_email_worker', 'core_identity_auth_schema_owner', 'MEMBER')
       OR pg_has_role('core_outbox_relay', 'core_tenancy_schema_owner', 'MEMBER')
       OR pg_has_role('core_email_worker', 'core_migrator', 'MEMBER')
       OR pg_has_role('core_outbox_relay', 'core_migrator', 'MEMBER') THEN
        RAISE EXCEPTION 'CORE worker identities cross a schema or role-membership boundary';
    END IF;
END
$worker_catalog$;

\echo 'PASS: five PostgreSQL owners, schemas, effective ACLs, memberships, object ownership, foreign-key boundaries, forced RLS and exact static/dynamic policy catalog verified.'
