-- One-time verification/recovery issuance and MFA challenge lifecycle.
CREATE TABLE core.auth_mfa_challenges (
    id uuid PRIMARY KEY,
    identity_id uuid NOT NULL REFERENCES core.identities(id) ON DELETE CASCADE,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    expires_at timestamptz NOT NULL,
    consumed_at timestamptz,
    UNIQUE (identity_id, id),
    CHECK (expires_at > created_at),
    CHECK (consumed_at IS NULL OR consumed_at >= created_at)
);
CREATE TABLE core.auth_mfa_challenge_lookup (
    token_hash bytea PRIMARY KEY CHECK (octet_length(token_hash) = 32),
    challenge_id uuid NOT NULL UNIQUE,
    identity_id uuid NOT NULL,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (identity_id, challenge_id)
        REFERENCES core.auth_mfa_challenges(identity_id, id)
);
ALTER TABLE core.auth_mfa_challenges ENABLE ROW LEVEL SECURITY;
ALTER TABLE core.auth_mfa_challenges FORCE ROW LEVEL SECURITY;
CREATE POLICY auth_mfa_challenges_definer_only ON core.auth_mfa_challenges
    USING (current_user = 'core_schema_owner')
    WITH CHECK (current_user = 'core_schema_owner');

CREATE OR REPLACE FUNCTION core.auth_issue_recovery_token(
    p_identity_id uuid, p_token_id uuid, p_token_hash bytea, p_expires_at timestamptz
)
RETURNS boolean
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
BEGIN
    IF octet_length(p_token_hash) <> 32 OR p_expires_at <= clock_timestamp()
       OR p_expires_at > clock_timestamp() + interval '2 hours' THEN
        RETURN false;
    END IF;
    IF NOT EXISTS (
        SELECT 1 FROM core.identities
         WHERE id = p_identity_id AND status = 'active' AND email_verified_at IS NOT NULL
    ) THEN
        RETURN false;
    END IF;
    UPDATE core.account_recovery_tokens
       SET consumed_at = COALESCE(consumed_at, clock_timestamp())
     WHERE identity_id = p_identity_id AND consumed_at IS NULL;
    INSERT INTO core.account_recovery_tokens(id, identity_id, expires_at)
    VALUES (p_token_id, p_identity_id, p_expires_at);
    INSERT INTO core.account_recovery_token_lookup(token_hash, recovery_id, identity_id)
    VALUES (p_token_hash, p_token_id, p_identity_id);
    RETURN true;
END
$$;

CREATE OR REPLACE FUNCTION core.auth_issue_verification_token(
    p_identity_id uuid, p_token_id uuid, p_token_hash bytea, p_expires_at timestamptz
)
RETURNS boolean
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
BEGIN
    IF octet_length(p_token_hash) <> 32 OR p_expires_at <= clock_timestamp()
       OR p_expires_at > clock_timestamp() + interval '48 hours' THEN
        RETURN false;
    END IF;
    IF NOT EXISTS (
        SELECT 1 FROM core.identities
         WHERE id = p_identity_id AND status = 'pending_verification'
    ) THEN
        RETURN false;
    END IF;
    UPDATE core.identity_verification_tokens
       SET consumed_at = COALESCE(consumed_at, clock_timestamp())
     WHERE identity_id = p_identity_id AND consumed_at IS NULL;
    INSERT INTO core.identity_verification_tokens(id, identity_id, expires_at)
    VALUES (p_token_id, p_identity_id, p_expires_at);
    INSERT INTO core.identity_verification_token_lookup(token_hash, verification_id, identity_id)
    VALUES (p_token_hash, p_token_id, p_identity_id);
    RETURN true;
END
$$;

CREATE OR REPLACE FUNCTION core.auth_consume_verification_token(p_token_hash bytea)
RETURNS uuid
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
DECLARE v_identity_id uuid; v_token_id uuid;
BEGIN
    IF octet_length(p_token_hash) <> 32 THEN RETURN NULL; END IF;
    SELECT t.identity_id, t.id INTO v_identity_id, v_token_id
      FROM core.identity_verification_tokens t
      JOIN core.identity_verification_token_lookup l
        ON l.identity_id = t.identity_id AND l.verification_id = t.id
     WHERE l.token_hash = p_token_hash
       AND t.consumed_at IS NULL AND t.expires_at > clock_timestamp()
     FOR UPDATE OF t;
    IF v_identity_id IS NULL THEN RETURN NULL; END IF;
    UPDATE core.identity_verification_tokens
       SET consumed_at = clock_timestamp()
     WHERE id = v_token_id AND consumed_at IS NULL;
    IF NOT FOUND THEN RETURN NULL; END IF;
    UPDATE core.identities
       SET status = 'active',
           email_verified_at = clock_timestamp(),
           updated_at = clock_timestamp()
     WHERE id = v_identity_id AND status = 'pending_verification';
    IF NOT FOUND THEN RETURN NULL; END IF;
    RETURN v_identity_id;
END
$$;

CREATE OR REPLACE FUNCTION core.auth_mfa_is_enabled(p_identity_id uuid)
RETURNS boolean
LANGUAGE sql
STABLE
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
    SELECT EXISTS (
        SELECT 1 FROM core.identity_mfa_totp
         WHERE identity_id = p_identity_id AND confirmed_at IS NOT NULL
    )
$$;

CREATE OR REPLACE FUNCTION core.auth_issue_mfa_challenge(
    p_challenge_id uuid, p_identity_id uuid, p_token_hash bytea, p_expires_at timestamptz
)
RETURNS boolean
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
BEGIN
    IF octet_length(p_token_hash) <> 32
       OR p_expires_at <= clock_timestamp()
       OR p_expires_at > clock_timestamp() + interval '10 minutes'
       OR NOT core.auth_mfa_is_enabled(p_identity_id) THEN
        RETURN false;
    END IF;
    INSERT INTO core.auth_mfa_challenges(id, identity_id, expires_at)
    VALUES (p_challenge_id, p_identity_id, p_expires_at);
    INSERT INTO core.auth_mfa_challenge_lookup(token_hash, challenge_id, identity_id)
    VALUES (p_token_hash, p_challenge_id, p_identity_id);
    RETURN true;
END
$$;

CREATE OR REPLACE FUNCTION core.auth_find_mfa_challenge(p_token_hash bytea)
RETURNS TABLE(challenge_id uuid, identity_id uuid, expires_at timestamptz, consumed_at timestamptz)
LANGUAGE sql
STABLE
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
    SELECT c.id, c.identity_id, c.expires_at, c.consumed_at
      FROM core.auth_mfa_challenges c
      JOIN core.auth_mfa_challenge_lookup l
        ON l.identity_id = c.identity_id AND l.challenge_id = c.id
     WHERE l.token_hash = p_token_hash AND octet_length(p_token_hash) = 32
     LIMIT 1
$$;

CREATE OR REPLACE FUNCTION core.auth_consume_mfa_challenge(p_token_hash bytea)
RETURNS uuid
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
DECLARE v_identity_id uuid; v_challenge_id uuid;
BEGIN
    SELECT c.identity_id, c.id INTO v_identity_id, v_challenge_id
      FROM core.auth_mfa_challenges c
      JOIN core.auth_mfa_challenge_lookup l
        ON l.identity_id = c.identity_id AND l.challenge_id = c.id
     WHERE l.token_hash = p_token_hash
       AND c.consumed_at IS NULL AND c.expires_at > clock_timestamp()
     FOR UPDATE OF c;
    IF v_identity_id IS NULL THEN RETURN NULL; END IF;
    UPDATE core.auth_mfa_challenges
       SET consumed_at = clock_timestamp()
     WHERE id = v_challenge_id AND consumed_at IS NULL;
    IF NOT FOUND THEN RETURN NULL; END IF;
    RETURN v_identity_id;
END
$$;

CREATE OR REPLACE FUNCTION core.auth_consume_mfa_recovery_code(
    p_identity_id uuid, p_code_hash bytea
)
RETURNS boolean
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
BEGIN
    IF octet_length(p_code_hash) <> 32 THEN RETURN false; END IF;
    UPDATE core.identity_mfa_recovery_codes
       SET consumed_at = clock_timestamp()
     WHERE identity_id = p_identity_id
       AND code_hash = p_code_hash
       AND consumed_at IS NULL;
    RETURN FOUND;
END
$$;

CREATE OR REPLACE FUNCTION core.auth_confirm_totp_step(p_identity_id uuid, p_step bigint)
RETURNS boolean
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
BEGIN
    UPDATE core.identity_mfa_totp
       SET confirmed_at = clock_timestamp(),
           last_accepted_step = p_step,
           updated_at = clock_timestamp()
     WHERE identity_id = p_identity_id
       AND confirmed_at IS NULL
       AND (last_accepted_step IS NULL OR p_step > last_accepted_step);
    RETURN FOUND;
END
$$;

REVOKE ALL ON FUNCTION core.auth_issue_recovery_token(uuid, uuid, bytea, timestamptz) FROM PUBLIC;
REVOKE ALL ON FUNCTION core.auth_issue_verification_token(uuid, uuid, bytea, timestamptz) FROM PUBLIC;
REVOKE ALL ON FUNCTION core.auth_consume_verification_token(bytea) FROM PUBLIC;
REVOKE ALL ON FUNCTION core.auth_mfa_is_enabled(uuid) FROM PUBLIC;
REVOKE ALL ON FUNCTION core.auth_issue_mfa_challenge(uuid, uuid, bytea, timestamptz) FROM PUBLIC;
REVOKE ALL ON FUNCTION core.auth_find_mfa_challenge(bytea) FROM PUBLIC;
REVOKE ALL ON FUNCTION core.auth_consume_mfa_challenge(bytea) FROM PUBLIC;
REVOKE ALL ON FUNCTION core.auth_consume_mfa_recovery_code(uuid, bytea) FROM PUBLIC;
REVOKE ALL ON FUNCTION core.auth_confirm_totp_step(uuid, bigint) FROM PUBLIC;

GRANT EXECUTE ON FUNCTION core.auth_issue_recovery_token(uuid, uuid, bytea, timestamptz) TO core_runtime;
GRANT EXECUTE ON FUNCTION core.auth_issue_verification_token(uuid, uuid, bytea, timestamptz) TO core_runtime;
GRANT EXECUTE ON FUNCTION core.auth_consume_verification_token(bytea) TO core_runtime;
GRANT EXECUTE ON FUNCTION core.auth_mfa_is_enabled(uuid) TO core_runtime;
GRANT EXECUTE ON FUNCTION core.auth_issue_mfa_challenge(uuid, uuid, bytea, timestamptz) TO core_runtime;
GRANT EXECUTE ON FUNCTION core.auth_find_mfa_challenge(bytea) TO core_runtime;
GRANT EXECUTE ON FUNCTION core.auth_consume_mfa_challenge(bytea) TO core_runtime;
GRANT EXECUTE ON FUNCTION core.auth_consume_mfa_recovery_code(uuid, bytea) TO core_runtime;
GRANT EXECUTE ON FUNCTION core.auth_confirm_totp_step(uuid, bigint) TO core_runtime;
