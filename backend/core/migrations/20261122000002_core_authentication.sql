SET LOCAL ROLE core_identity_auth_schema_owner;

-- Pre-authentication one-time tokens. Password verifiers, sessions and MFA
-- credentials are owned by the consolidated authorization migration.
CREATE TABLE core_identity_auth.account_recovery_tokens (
    id uuid PRIMARY KEY,
    identity_id uuid NOT NULL REFERENCES core_identity_auth.identities(id) ON DELETE CASCADE,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    expires_at timestamptz NOT NULL,
    consumed_at timestamptz,
    UNIQUE (identity_id, id),
    CHECK (expires_at > created_at),
    CHECK (consumed_at IS NULL OR consumed_at >= created_at)
);
CREATE TABLE core_identity_auth.account_recovery_token_lookup (
    token_hash bytea PRIMARY KEY CHECK (octet_length(token_hash) = 32),
    recovery_id uuid NOT NULL UNIQUE,
    identity_id uuid NOT NULL,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (identity_id, recovery_id)
        REFERENCES core_identity_auth.account_recovery_tokens(identity_id, id)
);
CREATE INDEX account_recovery_expiry_idx ON core_identity_auth.account_recovery_tokens(expires_at)
    WHERE consumed_at IS NULL;

CREATE TABLE core_identity_auth.identity_verification_tokens (
    id uuid PRIMARY KEY,
    identity_id uuid NOT NULL REFERENCES core_identity_auth.identities(id) ON DELETE CASCADE,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    expires_at timestamptz NOT NULL,
    consumed_at timestamptz,
    UNIQUE (identity_id, id),
    CHECK (expires_at > created_at),
    CHECK (consumed_at IS NULL OR consumed_at >= created_at)
);
CREATE TABLE core_identity_auth.identity_verification_token_lookup (
    token_hash bytea PRIMARY KEY CHECK (octet_length(token_hash) = 32),
    verification_id uuid NOT NULL UNIQUE,
    identity_id uuid NOT NULL,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (identity_id, verification_id)
        REFERENCES core_identity_auth.identity_verification_tokens(identity_id, id)
);
CREATE INDEX identity_verification_expiry_idx ON core_identity_auth.identity_verification_tokens(expires_at)
    WHERE consumed_at IS NULL;

DO $$
DECLARE relation_name text;
BEGIN
    FOREACH relation_name IN ARRAY ARRAY['account_recovery_tokens', 'identity_verification_tokens']
    LOOP
        EXECUTE format('ALTER TABLE core_identity_auth.%I ENABLE ROW LEVEL SECURITY', relation_name);
        EXECUTE format('ALTER TABLE core_identity_auth.%I FORCE ROW LEVEL SECURITY', relation_name);
        EXECUTE format(
            'CREATE POLICY %I ON core_identity_auth.%I USING (identity_id = NULLIF(current_setting(''app.actor_id'', true), '''')::uuid) WITH CHECK (identity_id = NULLIF(current_setting(''app.actor_id'', true), '''')::uuid)',
            relation_name || '_actor_scope', relation_name
        );
    END LOOP;
END
$$;

COMMENT ON TABLE core_identity_auth.account_recovery_tokens IS
    'Single-use, expiring recovery token state; only a SHA-256 digest is indexed.';
COMMENT ON TABLE core_identity_auth.identity_verification_tokens IS
    'Single-use email verification token state; only a SHA-256 digest is indexed.';
