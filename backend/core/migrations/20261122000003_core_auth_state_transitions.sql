-- Atomic state transitions for authentication, recovery, throttling and MFA.
-- Every function is SECURITY DEFINER with a fixed search_path and narrowly granted.

CREATE OR REPLACE FUNCTION core.auth_record_login_failure(
    p_identity_id uuid,
    p_max_attempts integer,
    p_lock_seconds integer
)
RETURNS timestamptz
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
DECLARE
    v_locked_until timestamptz;
BEGIN
    IF p_max_attempts < 1 OR p_lock_seconds < 1 THEN
        RAISE EXCEPTION 'invalid authentication policy' USING ERRCODE = '22023';
    END IF;

    UPDATE core.identity_credentials
       SET failed_attempts = CASE
               WHEN locked_until IS NOT NULL AND locked_until <= clock_timestamp() THEN 1
               ELSE failed_attempts + 1
           END,
           locked_until = CASE
               WHEN (CASE
                   WHEN locked_until IS NOT NULL AND locked_until <= clock_timestamp() THEN 1
                   ELSE failed_attempts + 1
               END) >= p_max_attempts
               THEN clock_timestamp() + make_interval(secs => p_lock_seconds)
               ELSE locked_until
           END,
           updated_at = clock_timestamp()
     WHERE identity_id = p_identity_id
     RETURNING locked_until INTO v_locked_until;

    RETURN v_locked_until;
END
$$;

CREATE OR REPLACE FUNCTION core.auth_record_login_success(p_identity_id uuid)
RETURNS boolean
LANGUAGE sql
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
    UPDATE core.identity_credentials
       SET failed_attempts = 0,
           locked_until = NULL,
           last_authenticated_at = clock_timestamp(),
           updated_at = clock_timestamp()
     WHERE identity_id = p_identity_id
       AND (locked_until IS NULL OR locked_until <= clock_timestamp())
    RETURNING true
$$;

CREATE OR REPLACE FUNCTION core.auth_consume_rate_limit(
    p_subject_digest bytea,
    p_action text,
    p_window_seconds integer,
    p_max_attempts integer,
    p_block_seconds integer
)
RETURNS boolean
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
DECLARE
    v_attempts integer;
    v_blocked_until timestamptz;
    v_now timestamptz := clock_timestamp();
BEGIN
    IF octet_length(p_subject_digest) <> 32
       OR p_action NOT IN ('login', 'recovery', 'mfa')
       OR p_window_seconds < 1 OR p_max_attempts < 1 OR p_block_seconds < 1 THEN
        RAISE EXCEPTION 'invalid rate-limit input' USING ERRCODE = '22023';
    END IF;

    INSERT INTO core.auth_rate_limits (
        subject_digest, action, window_started_at, attempts, blocked_until, updated_at
    ) VALUES (
        p_subject_digest, p_action, v_now, 1, NULL, v_now
    )
    ON CONFLICT (subject_digest, action) DO UPDATE
       SET window_started_at = CASE
               WHEN core.auth_rate_limits.window_started_at
                    + make_interval(secs => p_window_seconds) <= v_now
               THEN v_now ELSE core.auth_rate_limits.window_started_at END,
           attempts = CASE
               WHEN core.auth_rate_limits.window_started_at
                    + make_interval(secs => p_window_seconds) <= v_now
               THEN 1 ELSE core.auth_rate_limits.attempts + 1 END,
           blocked_until = CASE
               WHEN core.auth_rate_limits.blocked_until > v_now
               THEN core.auth_rate_limits.blocked_until
               WHEN (CASE
                   WHEN core.auth_rate_limits.window_started_at
                        + make_interval(secs => p_window_seconds) <= v_now
                   THEN 1 ELSE core.auth_rate_limits.attempts + 1 END) > p_max_attempts
               THEN v_now + make_interval(secs => p_block_seconds)
               ELSE NULL END,
           updated_at = v_now
    RETURNING attempts, blocked_until INTO v_attempts, v_blocked_until;

    RETURN v_attempts <= p_max_attempts AND (v_blocked_until IS NULL OR v_blocked_until <= v_now);
END
$$;

-- Rotation is one transactionally serialized state transition. A reused old
-- token revokes every session for that identity and records the reuse signal.
CREATE OR REPLACE FUNCTION core.auth_rotate_refresh_token(
    p_old_token_hash bytea,
    p_new_session_id uuid,
    p_new_token_hash bytea,
    p_new_expires_at timestamptz
)
RETURNS TABLE(identity_id uuid, rotation_succeeded boolean, reuse_detected boolean)
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
DECLARE
    v_session core.auth_sessions%ROWTYPE;
BEGIN
    IF octet_length(p_old_token_hash) <> 32
       OR octet_length(p_new_token_hash) <> 32
       OR p_new_expires_at <= clock_timestamp() THEN
        RETURN QUERY SELECT NULL::uuid, false, false;
        RETURN;
    END IF;

    SELECT * INTO v_session
      FROM core.auth_sessions
     WHERE refresh_token_hash = p_old_token_hash
     FOR UPDATE;

    IF NOT FOUND THEN
        RETURN QUERY SELECT NULL::uuid, false, false;
        RETURN;
    END IF;

    IF v_session.revoked_at IS NOT NULL OR v_session.replaced_by_session_id IS NOT NULL THEN
        UPDATE core.auth_sessions
           SET reuse_detected_at = COALESCE(reuse_detected_at, clock_timestamp())
         WHERE id = v_session.id;

        UPDATE core.auth_sessions
           SET revoked_at = COALESCE(revoked_at, clock_timestamp())
         WHERE auth_sessions.identity_id = v_session.identity_id;

        RETURN QUERY SELECT v_session.identity_id, false, true;
        RETURN;
    END IF;

    IF v_session.expires_at <= clock_timestamp() THEN
        RETURN QUERY SELECT v_session.identity_id, false, false;
        RETURN;
    END IF;

    INSERT INTO core.auth_sessions (
        id, identity_id, refresh_token_hash, expires_at, ip_prefix
    ) VALUES (
        p_new_session_id, v_session.identity_id, p_new_token_hash,
        p_new_expires_at, v_session.ip_prefix
    );

    UPDATE core.auth_sessions
       SET revoked_at = clock_timestamp(),
           replaced_by_session_id = p_new_session_id,
           last_used_at = clock_timestamp()
     WHERE id = v_session.id;

    RETURN QUERY SELECT v_session.identity_id, true, false;
END
$$;

-- Recovery completion consumes the token, replaces the Argon2id verifier,
-- clears lockout state and revokes every extant session atomically.
CREATE OR REPLACE FUNCTION core.auth_complete_recovery(
    p_token_hash bytea,
    p_new_password_hash text
)
RETURNS uuid
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
DECLARE
    v_identity_id uuid;
BEGIN
    IF octet_length(p_token_hash) <> 32 OR p_new_password_hash NOT LIKE '$argon2id$%' THEN
        RETURN NULL;
    END IF;

    SELECT identity_id INTO v_identity_id
      FROM core.account_recovery_tokens
     WHERE token_hash = p_token_hash
       AND consumed_at IS NULL
       AND expires_at > clock_timestamp()
     FOR UPDATE;

    IF v_identity_id IS NULL THEN
        RETURN NULL;
    END IF;

    UPDATE core.account_recovery_tokens
       SET consumed_at = clock_timestamp()
     WHERE token_hash = p_token_hash AND consumed_at IS NULL;

    IF NOT FOUND THEN
        RETURN NULL;
    END IF;

    UPDATE core.identity_credentials
       SET password_hash = p_new_password_hash,
           password_changed_at = clock_timestamp(),
           failed_attempts = 0,
           locked_until = NULL,
           updated_at = clock_timestamp()
     WHERE identity_id = v_identity_id;

    UPDATE core.auth_sessions
       SET revoked_at = COALESCE(revoked_at, clock_timestamp())
     WHERE identity_id = v_identity_id;

    RETURN v_identity_id;
END
$$;

-- TOTP verification must supply a strictly increasing accepted time-step.
CREATE OR REPLACE FUNCTION core.auth_accept_totp_step(p_identity_id uuid, p_step bigint)
RETURNS boolean
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
BEGIN
    UPDATE core.identity_mfa_totp
       SET last_accepted_step = p_step,
           updated_at = clock_timestamp()
     WHERE identity_id = p_identity_id
       AND confirmed_at IS NOT NULL
       AND (last_accepted_step IS NULL OR p_step > last_accepted_step);
    RETURN FOUND;
END
$$;

REVOKE ALL ON FUNCTION core.auth_record_login_failure(uuid, integer, integer) FROM PUBLIC;
REVOKE ALL ON FUNCTION core.auth_record_login_success(uuid) FROM PUBLIC;
REVOKE ALL ON FUNCTION core.auth_consume_rate_limit(bytea, text, integer, integer, integer) FROM PUBLIC;
REVOKE ALL ON FUNCTION core.auth_rotate_refresh_token(bytea, uuid, bytea, timestamptz) FROM PUBLIC;
REVOKE ALL ON FUNCTION core.auth_complete_recovery(bytea, text) FROM PUBLIC;
REVOKE ALL ON FUNCTION core.auth_accept_totp_step(uuid, bigint) FROM PUBLIC;

GRANT EXECUTE ON FUNCTION core.auth_record_login_failure(uuid, integer, integer) TO core_runtime;
GRANT EXECUTE ON FUNCTION core.auth_record_login_success(uuid) TO core_runtime;
GRANT EXECUTE ON FUNCTION core.auth_consume_rate_limit(bytea, text, integer, integer, integer) TO core_runtime;
GRANT EXECUTE ON FUNCTION core.auth_rotate_refresh_token(bytea, uuid, bytea, timestamptz) TO core_runtime;
GRANT EXECUTE ON FUNCTION core.auth_complete_recovery(bytea, text) TO core_runtime;
GRANT EXECUTE ON FUNCTION core.auth_accept_totp_step(uuid, bigint) TO core_runtime;
