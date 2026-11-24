SET LOCAL ROLE core_identity_auth_schema_owner;

-- Preserve the previous CORE release contract while the new access-token
-- implementation is deployed. The old release calls the four-argument
-- function; the current release uses the six-argument overload. Remove this
-- compatibility overload only in a later contract release after rollback
-- to the previous binary is no longer supported.
CREATE OR REPLACE FUNCTION core_identity_auth.auth_rotate_refresh_token(
    p_old_token_hash bytea,
    p_new_session_id uuid,
    p_new_token_hash bytea,
    p_new_expires_at timestamptz
)
RETURNS TABLE(identity_id uuid, rotation_succeeded boolean, reuse_detected boolean)
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog, core_identity_auth
AS $$
DECLARE
    v_session core_identity_auth.auth_sessions%ROWTYPE;
BEGIN
    IF octet_length(p_old_token_hash) <> 32
       OR octet_length(p_new_token_hash) <> 32
       OR p_new_expires_at <= clock_timestamp() THEN
        RETURN QUERY SELECT NULL::uuid, false, false;
        RETURN;
    END IF;

    SELECT * INTO v_session
      FROM core_identity_auth.auth_sessions
     WHERE refresh_token_hash = p_old_token_hash
     FOR UPDATE;

    IF NOT FOUND THEN
        RETURN QUERY SELECT NULL::uuid, false, false;
        RETURN;
    END IF;

    IF v_session.revoked_at IS NOT NULL OR v_session.replaced_by_session_id IS NOT NULL THEN
        UPDATE core_identity_auth.auth_sessions
           SET reuse_detected_at = COALESCE(reuse_detected_at, clock_timestamp())
         WHERE id = v_session.id;

        UPDATE core_identity_auth.auth_sessions
           SET revoked_at = COALESCE(revoked_at, clock_timestamp())
         WHERE auth_sessions.identity_id = v_session.identity_id;

        RETURN QUERY SELECT v_session.identity_id, false, true;
        RETURN;
    END IF;

    IF v_session.expires_at <= clock_timestamp() THEN
        RETURN QUERY SELECT v_session.identity_id, false, false;
        RETURN;
    END IF;

    INSERT INTO core_identity_auth.auth_sessions (
        id, identity_id, refresh_token_hash, expires_at, ip_prefix
    ) VALUES (
        p_new_session_id, v_session.identity_id, p_new_token_hash,
        p_new_expires_at, v_session.ip_prefix
    );

    UPDATE core_identity_auth.auth_sessions
       SET revoked_at = clock_timestamp(),
           replaced_by_session_id = p_new_session_id,
           last_used_at = clock_timestamp()
     WHERE id = v_session.id;

    RETURN QUERY SELECT v_session.identity_id, true, false;
END
$$;

REVOKE ALL ON FUNCTION core_identity_auth.auth_rotate_refresh_token(bytea, uuid, bytea, timestamptz) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION core_identity_auth.auth_rotate_refresh_token(bytea, uuid, bytea, timestamptz) TO core_identity_auth_runtime;
