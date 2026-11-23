-- SECURITY DEFINER functions execute as core_schema_owner. FORCE RLS remains
-- enabled; these policies permit only the non-login schema owner to service
-- reviewed functions. core_runtime is not a member of that role.
DO $$
DECLARE
    relation_name text;
BEGIN
    FOREACH relation_name IN ARRAY ARRAY[
        'identities',
        'tenants',
        'organizations',
        'branches',
        'tenant_memberships',
        'tenant_roles',
        'tenant_role_permissions',
        'membership_role_assignments',
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
            'CREATE POLICY %I ON core.%I USING (current_user = ''core_schema_owner'') WITH CHECK (current_user = ''core_schema_owner'')',
            relation_name || '_definer_owner_access',
            relation_name
        );
    END LOOP;
END
$$;
