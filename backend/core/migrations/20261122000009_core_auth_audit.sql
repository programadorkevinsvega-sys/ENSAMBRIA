SET LOCAL ROLE core_identity_auth_schema_owner;

-- Security audit writes and safe pending-identity lookup for verification resend.
CREATE POLICY security_audit_events_definer_access ON core_identity_auth.security_audit_events
    USING (current_user = 'core_identity_auth_schema_owner')
    WITH CHECK (current_user = 'core_identity_auth_schema_owner');

CREATE OR REPLACE FUNCTION core_identity_auth.auth_lookup_pending_identity(p_email text)
RETURNS TABLE(identity_id uuid, email_display text)
LANGUAGE sql
STABLE
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
    SELECT i.id, i.email_display
      FROM core_identity_auth.identities i
     WHERE i.email_normalized = lower(btrim(p_email))
       AND i.status = 'pending_verification'
     LIMIT 1
$$;

CREATE OR REPLACE FUNCTION core_identity_auth.auth_record_security_event(
    p_event_id uuid,
    p_actor_id uuid,
    p_action text,
    p_outcome text,
    p_correlation_id uuid,
    p_target_type text,
    p_target_id uuid,
    p_metadata jsonb
)
RETURNS void
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog, core
AS $$
BEGIN
    IF p_action !~ '^auth\.[a-z0-9_.-]{1,110}$'
       OR p_outcome NOT IN ('success', 'denied', 'failure')
       OR p_correlation_id IS NULL
       OR jsonb_typeof(COALESCE(p_metadata, '{}'::jsonb)) <> 'object' THEN
        RAISE EXCEPTION 'invalid security audit event' USING ERRCODE = '22023';
    END IF;

    INSERT INTO core_identity_auth.security_audit_events (
        id, actor_id, tenant_id, action, target_type, target_id,
        outcome, correlation_id, metadata
    ) VALUES (
        p_event_id, p_actor_id, NULL, p_action, p_target_type, p_target_id,
        p_outcome, p_correlation_id, COALESCE(p_metadata, '{}'::jsonb)
    );
END
$$;

REVOKE ALL ON FUNCTION core_identity_auth.auth_lookup_pending_identity(text) FROM PUBLIC;
REVOKE ALL ON FUNCTION core_identity_auth.auth_record_security_event(uuid, uuid, text, text, uuid, text, uuid, jsonb) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION core_identity_auth.auth_lookup_pending_identity(text) TO core_identity_auth_runtime;
GRANT EXECUTE ON FUNCTION core_identity_auth.auth_record_security_event(uuid, uuid, text, text, uuid, text, uuid, jsonb) TO core_identity_auth_runtime;
