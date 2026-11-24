-- SECURITY DEFINER functions execute only as the non-login owner of the
-- corresponding bounded module schema. FORCE RLS remains enabled.
SET LOCAL ROLE core_identity_auth_schema_owner;
DO $$
DECLARE
    relation_name text;
BEGIN
    FOREACH relation_name IN ARRAY ARRAY[
        'identities',
        'identity_credentials',
        'auth_sessions',
        'account_recovery_tokens',
        'identity_verification_tokens',
        'identity_mfa_totp',
        'identity_mfa_recovery_codes',
        'auth_rate_limits'
    ]
    LOOP
        EXECUTE format(
            'CREATE POLICY %I ON core_identity_auth.%I USING (current_user = ''core_identity_auth_schema_owner'') WITH CHECK (current_user = ''core_identity_auth_schema_owner'')',
            relation_name || '_definer_owner_access',
            relation_name
        );
    END LOOP;
END
$$;

SET LOCAL ROLE core_tenancy_schema_owner;
DO $$
DECLARE
    relation_name text;
BEGIN
    FOREACH relation_name IN ARRAY ARRAY[
        'tenants',
        'organizations',
        'branches',
        'tenant_memberships',
        'tenant_roles',
        'tenant_role_permissions',
        'membership_role_assignments',
        'tenant_products'
    ]
    LOOP
        EXECUTE format(
            'CREATE POLICY %I ON core_tenancy.%I USING (current_user = ''core_tenancy_schema_owner'') WITH CHECK (current_user = ''core_tenancy_schema_owner'')',
            relation_name || '_definer_owner_access',
            relation_name
        );
    END LOOP;
END
$$;
