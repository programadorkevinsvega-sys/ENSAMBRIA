-- Authentication records and opaque lookup indexes for CORE.
-- Raw passwords and bearer tokens are never persisted.

CREATE TABLE core.identity_login_lookup (
    email_hmac bytea PRIMARY KEY CHECK (octet_length(email_hmac) = 32),
    identity_id uuid NOT NULL UNIQUE REFERENCES core.identities(id),
    created_at timestamptz NOT NULL DEFAULT clock_timestamp()
);

CREATE TABLE core.password_credentials (
    identity_id uuid PRIMARY KEY REFERENCES core.identities(id),
    algorithm text NOT NULL CHECK (algorithm = 'pbkdf2-hmac-sha256'),
    iterations integer NOT NULL CHECK (iterations = 600000),
    salt bytea NOT NULL CHECK (octet_length(salt) = 16),
    password_hash bytea NOT NULL CHECK (octet_length(password_hash) = 32),
    password_changed_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    updated_at timestamptz NOT NULL DEFAULT clock_timestamp()
);

CREATE TABLE core.sessions (
    id uuid PRIMARY KEY,
    identity_id uuid NOT NULL REFERENCES core.identities(id),
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    expires_at timestamptz NOT NULL,
    last_seen_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    revoked_at timestamptz,
    UNIQUE (identity_id, id),
    CHECK (expires_at > created_at),
    CHECK (revoked_at IS NULL OR revoked_at >= created_at)
);

CREATE INDEX sessions_identity_expiry_idx
    ON core.sessions (identity_id, expires_at DESC)
    WHERE revoked_at IS NULL;

-- Token hashes are high-entropy opaque lookup keys, not reusable bearer credentials.
CREATE TABLE core.session_token_lookup (
    token_hash bytea PRIMARY KEY CHECK (octet_length(token_hash) = 32),
    session_id uuid NOT NULL UNIQUE,
    identity_id uuid NOT NULL,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (identity_id, session_id)
        REFERENCES core.sessions(identity_id, id)
);

CREATE TABLE core.account_recovery_tokens (
    id uuid PRIMARY KEY,
    identity_id uuid NOT NULL REFERENCES core.identities(id),
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    expires_at timestamptz NOT NULL,
    consumed_at timestamptz,
    UNIQUE (identity_id, id),
    CHECK (expires_at > created_at),
    CHECK (consumed_at IS NULL OR consumed_at >= created_at)
);

CREATE TABLE core.account_recovery_token_lookup (
    token_hash bytea PRIMARY KEY CHECK (octet_length(token_hash) = 32),
    recovery_id uuid NOT NULL UNIQUE,
    identity_id uuid NOT NULL,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (identity_id, recovery_id)
        REFERENCES core.account_recovery_tokens(identity_id, id)
);

CREATE TABLE core.identity_verification_tokens (
    id uuid PRIMARY KEY,
    identity_id uuid NOT NULL REFERENCES core.identities(id),
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    expires_at timestamptz NOT NULL,
    consumed_at timestamptz,
    UNIQUE (identity_id, id),
    CHECK (expires_at > created_at),
    CHECK (consumed_at IS NULL OR consumed_at >= created_at)
);

CREATE TABLE core.identity_verification_token_lookup (
    token_hash bytea PRIMARY KEY CHECK (octet_length(token_hash) = 32),
    verification_id uuid NOT NULL UNIQUE,
    identity_id uuid NOT NULL,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (identity_id, verification_id)
        REFERENCES core.identity_verification_tokens(identity_id, id)
);

-- MFA secrets must be encrypted by an approved key-management adapter before persistence.
CREATE TABLE core.mfa_credentials (
    identity_id uuid PRIMARY KEY REFERENCES core.identities(id),
    method text NOT NULL CHECK (method IN ('totp')),
    secret_ciphertext bytea NOT NULL CHECK (octet_length(secret_ciphertext) > 0),
    encryption_key_id text NOT NULL CHECK (length(encryption_key_id) BETWEEN 1 AND 120),
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    verified_at timestamptz,
    disabled_at timestamptz,
    CHECK (disabled_at IS NULL OR disabled_at >= created_at)
);

DO $$
DECLARE
    table_name text;
BEGIN
    FOREACH table_name IN ARRAY ARRAY[
        'password_credentials',
        'sessions',
        'account_recovery_tokens',
        'identity_verification_tokens',
        'mfa_credentials'
    ]
    LOOP
        EXECUTE format('ALTER TABLE core.%I ENABLE ROW LEVEL SECURITY', table_name);
        EXECUTE format('ALTER TABLE core.%I FORCE ROW LEVEL SECURITY', table_name);
        EXECUTE format(
            'CREATE POLICY %I ON core.%I USING (identity_id = NULLIF(current_setting(''app.actor_id'', true), '''')::uuid) WITH CHECK (identity_id = NULLIF(current_setting(''app.actor_id'', true), '''')::uuid)',
            table_name || '_actor_scope',
            table_name
        );
    END LOOP;
END
$$;

GRANT SELECT, INSERT, UPDATE ON
    core.password_credentials,
    core.sessions,
    core.account_recovery_tokens,
    core.identity_verification_tokens,
    core.mfa_credentials
TO core_runtime;

GRANT SELECT, INSERT, DELETE ON
    core.identity_login_lookup,
    core.session_token_lookup,
    core.account_recovery_token_lookup,
    core.identity_verification_token_lookup
TO core_runtime;

COMMENT ON TABLE core.identity_login_lookup IS
    'HMAC-only email lookup index; the HMAC key is held by CORE, never by PostgreSQL.';
COMMENT ON TABLE core.session_token_lookup IS
    'SHA-256 lookup index for random 256-bit opaque session tokens; the raw token is never stored.';
COMMENT ON TABLE core.mfa_credentials IS
    'Only encrypted MFA secrets may be stored; plaintext secrets are forbidden.';
