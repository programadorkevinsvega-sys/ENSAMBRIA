-- Identity security primitives and executable authorization for subphase 1.2.
-- Password hashing, token generation/hashing and TOTP cryptography belong to the
-- CORE application boundary; this schema stores only encoded Argon2id hashes,
-- one-way token digests and encrypted MFA material.

CREATE TABLE core_identity_auth.identity_credentials (
    identity_id uuid PRIMARY KEY REFERENCES core_identity_auth.identities(id) ON DELETE CASCADE,
    password_hash text NOT NULL,
    password_changed_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    failed_attempts integer NOT NULL DEFAULT 0 CHECK (failed_attempts >= 0),
    locked_until timestamptz,
    last_authenticated_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    updated_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    CONSTRAINT credentials_argon2id_format
        CHECK (password_hash LIKE '$argon2id$%')
);

CREATE TABLE core_identity_auth.auth_sessions (
    id uuid PRIMARY KEY,
    identity_id uuid NOT NULL REFERENCES core_identity_auth.identities(id) ON DELETE CASCADE,
    refresh_token_hash bytea NOT NULL UNIQUE CHECK (octet_length(refresh_token_hash) = 32),
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    last_used_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    expires_at timestamptz NOT NULL,
    revoked_at timestamptz,
    replaced_by_session_id uuid REFERENCES core_identity_auth.auth_sessions(id),
    reuse_detected_at timestamptz,
    user_agent_digest bytea CHECK (user_agent_digest IS NULL OR octet_length(user_agent_digest) = 32),
    ip_prefix inet,
    CONSTRAINT auth_session_expiry_after_creation CHECK (expires_at > created_at)
);
CREATE INDEX auth_sessions_identity_active_idx
    ON core_identity_auth.auth_sessions(identity_id, expires_at DESC)
    WHERE revoked_at IS NULL;

CREATE TABLE core_identity_auth.identity_mfa_totp (
    identity_id uuid PRIMARY KEY REFERENCES core_identity_auth.identities(id) ON DELETE CASCADE,
    encrypted_secret bytea NOT NULL CHECK (octet_length(encrypted_secret) >= 29),
    encryption_key_version text NOT NULL CHECK (length(encryption_key_version) BETWEEN 1 AND 64),
    confirmed_at timestamptz,
    last_accepted_step bigint,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    updated_at timestamptz NOT NULL DEFAULT clock_timestamp()
);

CREATE TABLE core_identity_auth.identity_mfa_recovery_codes (
    identity_id uuid NOT NULL REFERENCES core_identity_auth.identities(id) ON DELETE CASCADE,
    code_hash bytea NOT NULL CHECK (octet_length(code_hash) = 32),
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    consumed_at timestamptz,
    PRIMARY KEY (identity_id, code_hash)
);

CREATE TABLE core_identity_auth.auth_rate_limits (
    subject_digest bytea NOT NULL CHECK (octet_length(subject_digest) = 32),
    action text NOT NULL CHECK (action IN ('login', 'recovery', 'mfa')),
    window_started_at timestamptz NOT NULL,
    attempts integer NOT NULL CHECK (attempts >= 0),
    blocked_until timestamptz,
    updated_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    PRIMARY KEY (subject_digest, action)
);

-- Authentication lookup is deliberately a narrowly scoped SECURITY DEFINER
-- function: login has no authenticated actor GUC yet. It exposes only the
-- credential verifier and account state, never the credential table itself.
CREATE OR REPLACE FUNCTION core_identity_auth.auth_lookup_identity(p_email_normalized text)
RETURNS TABLE (
    identity_id uuid,
    password_hash text,
    identity_status text,
    email_verified_at timestamptz,
    locked_until timestamptz
)
LANGUAGE sql
STABLE
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
    SELECT i.id, c.password_hash, i.status, i.email_verified_at, c.locked_until
      FROM core_identity_auth.identities AS i
      JOIN core_identity_auth.identity_credentials AS c ON c.identity_id = i.id
     WHERE i.email_normalized = lower(btrim(p_email_normalized))
       AND i.status = 'active'
       AND i.email_verified_at IS NOT NULL
     LIMIT 1
$$;

-- Opaque refresh/recovery tokens are SHA-256 digests at rest. These functions
-- permit exact digest lookup before a principal is authenticated. They never
-- accept or return the raw bearer token.
CREATE OR REPLACE FUNCTION core_identity_auth.auth_find_refresh_session(p_token_hash bytea)
RETURNS TABLE (
    session_id uuid,
    identity_id uuid,
    expires_at timestamptz,
    revoked_at timestamptz,
    replaced_by_session_id uuid
)
LANGUAGE sql
STABLE
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
    SELECT s.id, s.identity_id, s.expires_at, s.revoked_at, s.replaced_by_session_id
      FROM core_identity_auth.auth_sessions AS s
     WHERE s.refresh_token_hash = p_token_hash
       AND octet_length(p_token_hash) = 32
     LIMIT 1
$$;

CREATE OR REPLACE FUNCTION core_identity_auth.auth_find_recovery_token(p_token_hash bytea)
RETURNS TABLE (token_id uuid, identity_id uuid, expires_at timestamptz, consumed_at timestamptz)
LANGUAGE sql
STABLE
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
    SELECT t.id, t.identity_id, t.expires_at, t.consumed_at
      FROM core_identity_auth.account_recovery_tokens AS t
      JOIN core_identity_auth.account_recovery_token_lookup AS l
        ON l.identity_id = t.identity_id AND l.recovery_id = t.id
     WHERE l.token_hash = p_token_hash
       AND octet_length(p_token_hash) = 32
     LIMIT 1
$$;

-- Authorization is evaluated from the authenticated actor context + active membership +
-- explicit scoped role assignment. No default role or implicit membership.
RESET ROLE;
SET LOCAL ROLE core_tenancy_schema_owner;

CREATE OR REPLACE FUNCTION core_tenancy.has_permission(
    p_actor_id uuid,
    p_tenant_id uuid,
    p_permission_code text,
    p_organization_id uuid DEFAULT NULL,
    p_branch_id uuid DEFAULT NULL
)
RETURNS boolean
LANGUAGE sql
STABLE
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
    SELECT EXISTS (
        SELECT 1
          FROM core_tenancy.tenant_memberships m
          JOIN core_tenancy.tenants t ON t.id = m.tenant_id
          JOIN core_tenancy.membership_role_assignments a
            ON a.tenant_id = m.tenant_id AND a.membership_id = m.id
          JOIN core_tenancy.tenant_role_permissions rp
            ON rp.tenant_id = a.tenant_id AND rp.role_id = a.role_id
         WHERE m.identity_id = p_actor_id
           AND m.identity_id = NULLIF(current_setting('app.actor_id', true), '')::uuid
           AND p_tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::uuid
           AND m.tenant_id = p_tenant_id
           AND m.status = 'active'
           AND t.status = 'active'
           AND rp.permission_code = p_permission_code
           AND (p_branch_id IS NULL OR p_organization_id IS NOT NULL)
           AND (
                p_organization_id IS NULL
                OR EXISTS (
                    SELECT 1 FROM core_tenancy.organizations o
                     WHERE o.id = p_organization_id
                       AND o.tenant_id = p_tenant_id
                       AND o.status = 'active'
                )
           )
           AND (
                p_branch_id IS NULL
                OR EXISTS (
                    SELECT 1 FROM core_tenancy.branches b
                     WHERE b.id = p_branch_id
                       AND b.tenant_id = p_tenant_id
                       AND b.organization_id = p_organization_id
                       AND b.status = 'active'
                )
           )
           AND (
                (a.scope_type = 'tenant' AND a.organization_id IS NULL AND a.branch_id IS NULL)
             OR (a.scope_type = 'organization'
                 AND p_organization_id IS NOT NULL
                 AND a.organization_id = p_organization_id
                 AND a.branch_id IS NULL)
             OR (a.scope_type = 'branch'
                 AND p_organization_id IS NOT NULL
                 AND p_branch_id IS NOT NULL
                 AND a.organization_id = p_organization_id
                 AND a.branch_id = p_branch_id)
           )
    )
$$;

-- Regional values resolve independently per field; NULL means no approved
-- value exists and callers must not silently invent a locale/currency/zone.
CREATE OR REPLACE FUNCTION core_tenancy.resolve_regional_context(
    p_tenant_id uuid,
    p_organization_id uuid DEFAULT NULL,
    p_branch_id uuid DEFAULT NULL
)
RETURNS TABLE(country_code char(2), currency_code char(3), locale text, time_zone text)
LANGUAGE sql
STABLE
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
    SELECT
        COALESCE(b.country_code, o.country_code, t.country_code),
        COALESCE(b.currency_code, o.currency_code, t.currency_code),
        COALESCE(b.locale, o.locale, t.locale),
        COALESCE(b.time_zone, o.time_zone, t.time_zone)
      FROM core_tenancy.tenants t
      LEFT JOIN core_tenancy.organizations o
        ON o.tenant_id = t.id AND o.id = p_organization_id
      LEFT JOIN core_tenancy.branches b
        ON b.tenant_id = t.id
       AND b.organization_id = p_organization_id
       AND b.id = p_branch_id
     WHERE t.id = p_tenant_id
       AND p_tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::uuid
       AND EXISTS (
           SELECT 1
             FROM core_tenancy.tenant_memberships m
             WHERE m.tenant_id = t.id
              AND m.identity_id = NULLIF(current_setting('app.actor_id', true), '')::uuid
              AND m.status = 'active'
       )
       AND (p_organization_id IS NULL OR o.id IS NOT NULL)
       AND (p_branch_id IS NULL OR b.id IS NOT NULL)
       AND t.status = 'active'
       AND (o.id IS NULL OR o.status = 'active')
       AND (b.id IS NULL OR b.status = 'active')
$$;

RESET ROLE;
SET LOCAL ROLE core_identity_auth_schema_owner;

-- All auth records are actor-scoped after authentication. Pre-authentication
-- access is available only through the narrow functions above.
DO $$
DECLARE
    relation_name text;
BEGIN
    FOREACH relation_name IN ARRAY ARRAY[
        'identity_credentials',
        'auth_sessions',
        'identity_mfa_totp',
        'identity_mfa_recovery_codes'
    ]
    LOOP
        EXECUTE format('ALTER TABLE core_identity_auth.%I ENABLE ROW LEVEL SECURITY', relation_name);
        EXECUTE format('ALTER TABLE core_identity_auth.%I FORCE ROW LEVEL SECURITY', relation_name);
        EXECUTE format(
            'CREATE POLICY %I ON core_identity_auth.%I USING (identity_id = NULLIF(current_setting(''app.actor_id'', true), '''')::uuid) WITH CHECK (identity_id = NULLIF(current_setting(''app.actor_id'', true), '''')::uuid)',
            relation_name || '_actor_scope',
            relation_name
        );
    END LOOP;
END
$$;

ALTER TABLE core_identity_auth.auth_rate_limits ENABLE ROW LEVEL SECURITY;
ALTER TABLE core_identity_auth.auth_rate_limits FORCE ROW LEVEL SECURITY;
CREATE POLICY auth_rate_limits_no_direct_access ON core_identity_auth.auth_rate_limits
    USING (false) WITH CHECK (false);

GRANT SELECT, INSERT, UPDATE, DELETE ON
    core_identity_auth.identity_credentials,
    core_identity_auth.auth_sessions,
    core_identity_auth.account_recovery_tokens,
    core_identity_auth.identity_mfa_totp,
    core_identity_auth.identity_mfa_recovery_codes
TO core_identity_auth_runtime;
GRANT INSERT, UPDATE, DELETE ON core_identity_auth.auth_rate_limits TO core_identity_auth_runtime;

REVOKE ALL ON FUNCTION core_identity_auth.auth_lookup_identity(text) FROM PUBLIC;
REVOKE ALL ON FUNCTION core_identity_auth.auth_find_refresh_session(bytea) FROM PUBLIC;
REVOKE ALL ON FUNCTION core_identity_auth.auth_find_recovery_token(bytea) FROM PUBLIC;
REVOKE ALL ON FUNCTION core_tenancy.has_permission(uuid, uuid, text, uuid, uuid) FROM PUBLIC;
REVOKE ALL ON FUNCTION core_tenancy.resolve_regional_context(uuid, uuid, uuid) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION core_identity_auth.auth_lookup_identity(text) TO core_identity_auth_runtime;
GRANT EXECUTE ON FUNCTION core_identity_auth.auth_find_refresh_session(bytea) TO core_identity_auth_runtime;
GRANT EXECUTE ON FUNCTION core_identity_auth.auth_find_recovery_token(bytea) TO core_identity_auth_runtime;
GRANT EXECUTE ON FUNCTION core_tenancy.has_permission(uuid, uuid, text, uuid, uuid) TO core_tenancy_runtime;
GRANT EXECUTE ON FUNCTION core_tenancy.resolve_regional_context(uuid, uuid, uuid) TO core_tenancy_runtime;

COMMENT ON TABLE core_identity_auth.identity_credentials IS
    'Argon2id password verifier and account lockout state; plaintext passwords are never stored.';
COMMENT ON TABLE core_identity_auth.auth_sessions IS
    'Opaque refresh-token SHA-256 digests, single-session revocation and rotation/reuse evidence.';
COMMENT ON TABLE core_identity_auth.account_recovery_tokens IS
    'Single-use, expiring recovery token digests; delivery is performed by an approved out-of-band provider.';
COMMENT ON TABLE core_identity_auth.identity_mfa_totp IS
    'TOTP secret ciphertext only; encryption/decryption and key custody are application/KMS responsibilities.';
COMMENT ON TABLE core_identity_auth.auth_rate_limits IS
    'Pre-authentication throttling state keyed by a keyed digest; direct reads are denied.';
