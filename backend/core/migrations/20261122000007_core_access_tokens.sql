-- Short-lived opaque access tokens and complete session lifecycle.
ALTER TABLE core.auth_sessions
    ADD COLUMN access_token_hash bytea UNIQUE
        CHECK (access_token_hash IS NULL OR octet_length(access_token_hash) = 32),
    ADD COLUMN access_expires_at timestamptz,
    ADD CONSTRAINT auth_access_expiry_before_refresh CHECK (
        access_expires_at IS NULL
        OR (access_expires_at > created_at AND access_expires_at < expires_at)
    );

CREATE OR REPLACE FUNCTION core.auth_create_session(
    p_session_id uuid,
    p_identity_id uuid,
    p_access_token_hash bytea,
    p_access_expires_at timestamptz,
    p_refresh_token_hash bytea,
    p_refresh_expires_at timestamptz
)
RETURNS boolean
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
BEGIN
    IF octet_length(p_access_token_hash) <> 32
       OR octet_length(p_refresh_token_hash) <> 32
       OR p_access_expires_at <= clock_timestamp()
       OR p_refresh_expires_at <= p_access_expires_at THEN
        RETURN false;
    END IF;

    INSERT INTO core.auth_sessions (
        id, identity_id, access_token_hash, access_expires_at,
        refresh_token_hash, expires_at
    )
    SELECT p_session_id, i.id, p_access_token_hash, p_access_expires_at,
           p_refresh_token_hash, p_refresh_expires_at
      FROM core.identities i
     WHERE i.id = p_identity_id
       AND i.status = 'active'
       AND i.email_verified_at IS NOT NULL;

    RETURN FOUND;
END
$$;

CREATE OR REPLACE FUNCTION core.auth_find_access_session(p_access_token_hash bytea)
RETURNS TABLE (
    session_id uuid,
    identity_id uuid,
    access_expires_at timestamptz,
    refresh_expires_at timestamptz,
    revoked_at timestamptz,
    identity_status text
)
LANGUAGE sql
STABLE
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
    SELECT s.id, s.identity_id, s.access_expires_at, s.expires_at,
           s.revoked_at, i.status
      FROM core.auth_sessions s
      JOIN core.identities i ON i.id = s.identity_id
     WHERE s.access_token_hash = p_access_token_hash
       AND octet_length(p_access_token_hash) = 32
     LIMIT 1
$$;

DROP FUNCTION core.auth_rotate_refresh_token(bytea, uuid, bytea, timestamptz);

CREATE FUNCTION core.auth_rotate_refresh_token(
    p_old_refresh_hash bytea,
    p_new_session_id uuid,
    p_new_access_hash bytea,
    p_new_access_expires_at timestamptz,
    p_new_refresh_hash bytea,
    p_new_refresh_expires_at timestamptz
)
RETURNS TABLE(identity_id uuid, rotation_succeeded boolean, reuse_detected boolean)
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
DECLARE
    v_session core.auth_sessions%ROWTYPE;
BEGIN
    IF octet_length(p_old_refresh_hash) <> 32
       OR octet_length(p_new_access_hash) <> 32
       OR octet_length(p_new_refresh_hash) <> 32
       OR p_new_access_expires_at <= clock_timestamp()
       OR p_new_refresh_expires_at <= p_new_access_expires_at THEN
        RETURN QUERY SELECT NULL::uuid, false, false;
        RETURN;
    END IF;

    SELECT * INTO v_session
      FROM core.auth_sessions
     WHERE refresh_token_hash = p_old_refresh_hash
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

    IF v_session.expires_at <= clock_timestamp()
       OR p_new_refresh_expires_at <= clock_timestamp() THEN
        RETURN QUERY SELECT v_session.identity_id, false, false;
        RETURN;
    END IF;

    INSERT INTO core.auth_sessions (
        id, identity_id, access_token_hash, access_expires_at,
        refresh_token_hash, expires_at, ip_prefix, user_agent_digest
    ) VALUES (
        p_new_session_id, v_session.identity_id, p_new_access_hash,
        p_new_access_expires_at, p_new_refresh_hash,
        p_new_refresh_expires_at, v_session.ip_prefix, v_session.user_agent_digest
    );

    UPDATE core.auth_sessions
       SET revoked_at = clock_timestamp(),
           replaced_by_session_id = p_new_session_id,
           last_used_at = clock_timestamp()
     WHERE id = v_session.id;

    RETURN QUERY SELECT v_session.identity_id, true, false;
END
$$;

REVOKE ALL ON FUNCTION core.auth_create_session(uuid, uuid, bytea, timestamptz, bytea, timestamptz) FROM PUBLIC;
REVOKE ALL ON FUNCTION core.auth_find_access_session(bytea) FROM PUBLIC;
REVOKE ALL ON FUNCTION core.auth_rotate_refresh_token(bytea, uuid, bytea, timestamptz, bytea, timestamptz) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION core.auth_create_session(uuid, uuid, bytea, timestamptz, bytea, timestamptz) TO core_runtime;
GRANT EXECUTE ON FUNCTION core.auth_find_access_session(bytea) TO core_runtime;
GRANT EXECUTE ON FUNCTION core.auth_rotate_refresh_token(bytea, uuid, bytea, timestamptz, bytea, timestamptz) TO core_runtime;

COMMENT ON COLUMN core.auth_sessions.access_token_hash IS
    'SHA-256 digest of a random, short-lived opaque access token; raw bearer material is never persisted.';
