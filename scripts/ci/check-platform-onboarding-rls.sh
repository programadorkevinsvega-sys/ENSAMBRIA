#!/usr/bin/env bash
set -euo pipefail
: "\${PLATFORM_VERIFY_DATABASE_URL:?PLATFORM_VERIFY_DATABASE_URL is required}"
: "\${PLATFORM_TENANT_ONBOARDING_DATABASE_URL:?PLATFORM_TENANT_ONBOARDING_DATABASE_URL is required}"
psql "$PLATFORM_VERIFY_DATABASE_URL" -v ON_ERROR_STOP=1 <<'SQL'
INSERT INTO platform_tenant_onboarding.inbox_messages
    (event_id, consumer_name, event_type, schema_version, aggregate_id, aggregate_version, product_id, product_config_version, correlation_id, payload_hash)
VALUES
    ('018f0f2a-7b3c-7abc-8def-0123456789e1', 'platform.tenant-onboarding', 'core.tenant.created.v1', 1, '018f0f2a-7b3c-7abc-8def-0123456789e3', 1, '018f0f2a-7b3c-7abc-8def-0123456789ea', 1, '018f0f2a-7b3c-7abc-8def-0123456789eb', decode(repeat('a', 64), 'hex')),
    ('018f0f2a-7b3c-7abc-8def-0123456789e2', 'platform.tenant-onboarding', 'core.tenant.created.v1', 1, '018f0f2a-7b3c-7abc-8def-0123456789e4', 1, '018f0f2a-7b3c-7abc-8def-0123456789ea', 1, '018f0f2a-7b3c-7abc-8def-0123456789ec', decode(repeat('b', 64), 'hex'))
ON CONFLICT (event_id) DO NOTHING;
INSERT INTO platform_tenant_onboarding.tenant_onboardings
    (tenant_id, product_id, product_config_version, source_event_id)
VALUES
    ('018f0f2a-7b3c-7abc-8def-0123456789e3', '018f0f2a-7b3c-7abc-8def-0123456789ea', 1, '018f0f2a-7b3c-7abc-8def-0123456789e1'),
    ('018f0f2a-7b3c-7abc-8def-0123456789e4', '018f0f2a-7b3c-7abc-8def-0123456789ea', 1, '018f0f2a-7b3c-7abc-8def-0123456789e2')
ON CONFLICT (tenant_id, product_id) DO NOTHING;
SQL
psql "$PLATFORM_TENANT_ONBOARDING_DATABASE_URL" -v ON_ERROR_STOP=1 <<'SQL'
BEGIN;
SET LOCAL ROLE platform_tenant_onboarding_runtime;
SELECT set_config('app.tenant_id', '018f0f2a-7b3c-7abc-8def-0123456789e3', true);
DO $onboarding_rls$
DECLARE visible_rows integer; affected_rows integer; spoof_rejected boolean := false;
BEGIN
    SELECT count(*) INTO visible_rows FROM platform_tenant_onboarding.inbox_messages;
    IF visible_rows <> 1 THEN RAISE EXCEPTION 'tenant A must see exactly one Inbox event, saw %', visible_rows; END IF;
    SELECT count(*) INTO visible_rows FROM platform_tenant_onboarding.tenant_onboardings;
    IF visible_rows <> 1 THEN RAISE EXCEPTION 'tenant A must see exactly one onboarding projection, saw %', visible_rows; END IF;
    SELECT count(*) INTO visible_rows FROM platform_tenant_onboarding.inbox_messages WHERE aggregate_id = '018f0f2a-7b3c-7abc-8def-0123456789e4';
    IF visible_rows <> 0 THEN RAISE EXCEPTION 'tenant A can read tenant B Inbox event'; END IF;
    SELECT count(*) INTO visible_rows FROM platform_tenant_onboarding.tenant_onboardings WHERE tenant_id = '018f0f2a-7b3c-7abc-8def-0123456789e4';
    IF visible_rows <> 0 THEN RAISE EXCEPTION 'tenant A can read tenant B onboarding projection'; END IF;
    UPDATE platform_tenant_onboarding.inbox_messages SET attempt_count = attempt_count + 1 WHERE event_id = '018f0f2a-7b3c-7abc-8def-0123456789e2';
    GET DIAGNOSTICS affected_rows = ROW_COUNT;
    IF affected_rows <> 0 THEN RAISE EXCEPTION 'tenant A modified tenant B Inbox event'; END IF;
    UPDATE platform_tenant_onboarding.tenant_onboardings SET status = 'onboarding_pending' WHERE tenant_id = '018f0f2a-7b3c-7abc-8def-0123456789e4';
    GET DIAGNOSTICS affected_rows = ROW_COUNT;
    IF affected_rows <> 0 THEN RAISE EXCEPTION 'tenant A modified tenant B onboarding projection'; END IF;
    BEGIN
        INSERT INTO platform_tenant_onboarding.inbox_messages
            (event_id, consumer_name, event_type, schema_version, aggregate_id, aggregate_version, product_id, product_config_version, correlation_id, payload_hash)
        VALUES ('018f0f2a-7b3c-7abc-8def-0123456789e5', 'platform.tenant-onboarding', 'core.tenant.created.v1', 1,
                '018f0f2a-7b3c-7abc-8def-0123456789e4', 2, '018f0f2a-7b3c-7abc-8def-0123456789ea', 1,
                '018f0f2a-7b3c-7abc-8def-0123456789ed', decode(repeat('c', 64), 'hex'));
    EXCEPTION WHEN insufficient_privilege THEN spoof_rejected := true;
    END;
    IF NOT spoof_rejected THEN RAISE EXCEPTION 'tenant A inserted an Inbox event for tenant B'; END IF;
    PERFORM set_config('app.tenant_id', '', true);
    SELECT count(*) INTO visible_rows FROM platform_tenant_onboarding.inbox_messages;
    IF visible_rows <> 0 THEN RAISE EXCEPTION 'missing tenant context must fail closed for Inbox'; END IF;
    SELECT count(*) INTO visible_rows FROM platform_tenant_onboarding.tenant_onboardings;
    IF visible_rows <> 0 THEN RAISE EXCEPTION 'missing tenant context must fail closed for onboarding'; END IF;
END
$onboarding_rls$;
ROLLBACK;
SQL
echo "PASS: PLATFORM Inbox and onboarding RLS block cross-tenant reads/writes and spoofed tenant inserts, and fail closed without tenant context."
