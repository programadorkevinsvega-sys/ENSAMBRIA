#!/usr/bin/env bash
set -euo pipefail
: "\${PLATFORM_VERIFY_DATABASE_URL:?PLATFORM_VERIFY_DATABASE_URL is required}"
: "\${CORE_TEST_DATABASE_URL:?CORE_TEST_DATABASE_URL is required}"
psql "$PLATFORM_VERIFY_DATABASE_URL" -v ON_ERROR_STOP=1 <<'SQL'
INSERT INTO core_identity_auth.identities
    (id, email_normalized, email_display, status)
VALUES
    ('018f0f2a-7b3c-7abc-8def-0123456789d1', 'rls-actor-a@example.test', 'rls-actor-a@example.test', 'pending_verification'),
    ('018f0f2a-7b3c-7abc-8def-0123456789d2', 'rls-actor-b@example.test', 'rls-actor-b@example.test', 'pending_verification')
ON CONFLICT (id) DO NOTHING;
INSERT INTO core_identity_auth.security_audit_events
    (id, actor_id, tenant_id, action, outcome, correlation_id, metadata)
VALUES
    ('018f0f2a-7b3c-7abc-8def-0123456789d3', '018f0f2a-7b3c-7abc-8def-0123456789d1', NULL, 'auth.rls_probe', 'success', '018f0f2a-7b3c-7abc-8def-0123456789d5', '{}'::jsonb),
    ('018f0f2a-7b3c-7abc-8def-0123456789d4', '018f0f2a-7b3c-7abc-8def-0123456789d2', NULL, 'auth.rls_probe', 'success', '018f0f2a-7b3c-7abc-8def-0123456789d6', '{}'::jsonb)
ON CONFLICT (id) DO NOTHING;
SQL
psql "$CORE_TEST_DATABASE_URL" -v ON_ERROR_STOP=1 <<'SQL'
BEGIN;
SELECT set_config('app.actor_id', '018f0f2a-7b3c-7abc-8def-0123456789d1', true);
DO $actor_rls$
DECLARE visible_rows integer; affected_rows integer; spoof_rejected boolean := false;
BEGIN
    SELECT count(*) INTO visible_rows FROM core_identity_auth.identities;
    IF visible_rows <> 1 THEN RAISE EXCEPTION 'actor A must see exactly one identity, saw %', visible_rows; END IF;
    SELECT count(*) INTO visible_rows FROM core_identity_auth.identities WHERE id = '018f0f2a-7b3c-7abc-8def-0123456789d2';
    IF visible_rows <> 0 THEN RAISE EXCEPTION 'actor A can read actor B identity'; END IF;
    UPDATE core_identity_auth.identities SET display_name = 'tampered' WHERE id = '018f0f2a-7b3c-7abc-8def-0123456789d2';
    GET DIAGNOSTICS affected_rows = ROW_COUNT;
    IF affected_rows <> 0 THEN RAISE EXCEPTION 'actor A modified actor B identity'; END IF;
    SELECT count(*) INTO visible_rows FROM core_identity_auth.security_audit_events;
    IF visible_rows <> 1 THEN RAISE EXCEPTION 'actor A must see exactly one actor-scoped audit event, saw %', visible_rows; END IF;
    SELECT count(*) INTO visible_rows FROM core_identity_auth.security_audit_events WHERE actor_id = '018f0f2a-7b3c-7abc-8def-0123456789d2';
    IF visible_rows <> 0 THEN RAISE EXCEPTION 'actor A can read actor B audit events'; END IF;
    BEGIN
        INSERT INTO core_identity_auth.identities (id, email_normalized, email_display, status)
        VALUES ('018f0f2a-7b3c-7abc-8def-0123456789d7', 'rls-spoofed@example.test', 'rls-spoofed@example.test', 'pending_verification');
    EXCEPTION WHEN insufficient_privilege THEN spoof_rejected := true;
    END;
    IF NOT spoof_rejected THEN RAISE EXCEPTION 'actor A inserted an identity for another actor'; END IF;
    spoof_rejected := false;
    BEGIN
        INSERT INTO core_identity_auth.security_audit_events
            (id, actor_id, tenant_id, action, outcome, correlation_id, metadata)
        VALUES ('018f0f2a-7b3c-7abc-8def-0123456789d8', '018f0f2a-7b3c-7abc-8def-0123456789d2', NULL,
                'auth.rls_spoof', 'success', '018f0f2a-7b3c-7abc-8def-0123456789d9', '{}'::jsonb);
    EXCEPTION WHEN insufficient_privilege THEN spoof_rejected := true;
    END;
    IF NOT spoof_rejected THEN RAISE EXCEPTION 'actor A forged an audit event for actor B'; END IF;
    PERFORM set_config('app.actor_id', '', true);
    SELECT count(*) INTO visible_rows FROM core_identity_auth.identities;
    IF visible_rows <> 0 THEN RAISE EXCEPTION 'missing actor context must fail closed for identities'; END IF;
    SELECT count(*) INTO visible_rows FROM core_identity_auth.security_audit_events;
    IF visible_rows <> 0 THEN RAISE EXCEPTION 'missing actor context must fail closed for audit events'; END IF;
END
$actor_rls$;
ROLLBACK;
SQL
echo "PASS: CORE identity and security-audit actor RLS blocks cross-actor reads, writes and spoofing, and fails closed without actor context."
