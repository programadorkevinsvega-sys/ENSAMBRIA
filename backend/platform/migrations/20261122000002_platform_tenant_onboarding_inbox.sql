-- Durable Inbox and commercial onboarding projection for CORE tenant-created events.
-- The consumer inserts Inbox + onboarding in one PLATFORM transaction before ACKing JetStream.
CREATE TABLE platform_tenant_onboarding.inbox_messages (
    event_id uuid PRIMARY KEY,
    consumer_name text NOT NULL CHECK (consumer_name = 'platform.tenant-onboarding'),
    event_type text NOT NULL CHECK (event_type = 'core.tenant.created.v1'),
    schema_version integer NOT NULL CHECK (schema_version = 1),
    aggregate_id uuid NOT NULL,
    aggregate_version bigint NOT NULL CHECK (aggregate_version > 0),
    product_id uuid NOT NULL,
    product_config_version integer NOT NULL CHECK (product_config_version > 0),
    correlation_id uuid NOT NULL,
    payload_hash bytea NOT NULL CHECK (octet_length(payload_hash) = 32),
    status text NOT NULL DEFAULT 'received'
        CHECK (status IN ('received', 'applied', 'quarantined')),
    attempt_count integer NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),
    received_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    applied_at timestamptz,
    last_error_code text
        CHECK (last_error_code IS NULL OR last_error_code ~ '^[A-Z][A-Z0-9_]{2,79}$'),
    CONSTRAINT inbox_applied_shape CHECK (
        (status = 'applied') = (applied_at IS NOT NULL)
    ),
    CONSTRAINT inbox_error_shape CHECK (
        status <> 'quarantined' OR last_error_code IS NOT NULL
    )
);

CREATE INDEX inbox_messages_unapplied_idx
    ON platform_tenant_onboarding.inbox_messages (received_at, event_id)
    WHERE status = 'received';

-- Both tables carry tenant data. The durable consumer sets this transaction-local
-- context from the validated event before touching either table.
ALTER TABLE platform_tenant_onboarding.inbox_messages ENABLE ROW LEVEL SECURITY;
ALTER TABLE platform_tenant_onboarding.inbox_messages FORCE ROW LEVEL SECURITY;
CREATE POLICY inbox_tenant_scope ON platform_tenant_onboarding.inbox_messages
    USING (aggregate_id = NULLIF(current_setting('app.tenant_id', true), '')::uuid)
    WITH CHECK (aggregate_id = NULLIF(current_setting('app.tenant_id', true), '')::uuid);



CREATE TABLE platform_tenant_onboarding.tenant_onboardings (
    tenant_id uuid NOT NULL,
    product_id uuid NOT NULL,
    product_config_version integer NOT NULL CHECK (product_config_version > 0),
    source_event_id uuid NOT NULL UNIQUE REFERENCES platform_tenant_onboarding.inbox_messages(event_id),
    status text NOT NULL DEFAULT 'onboarding_pending'
        CHECK (status = 'onboarding_pending'),
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    PRIMARY KEY (tenant_id, product_id)
);

ALTER TABLE platform_tenant_onboarding.tenant_onboardings ENABLE ROW LEVEL SECURITY;
ALTER TABLE platform_tenant_onboarding.tenant_onboardings FORCE ROW LEVEL SECURITY;
CREATE POLICY tenant_onboardings_tenant_scope ON platform_tenant_onboarding.tenant_onboardings
    USING (tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::uuid)
    WITH CHECK (tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::uuid);

REVOKE ALL ON
    platform_tenant_onboarding.inbox_messages,
    platform_tenant_onboarding.tenant_onboardings
FROM PUBLIC;

GRANT USAGE ON SCHEMA platform_tenant_onboarding TO platform_tenant_onboarding_runtime;
GRANT SELECT, INSERT, UPDATE ON platform_tenant_onboarding.inbox_messages TO platform_tenant_onboarding_runtime;
GRANT SELECT, INSERT, UPDATE ON platform_tenant_onboarding.tenant_onboardings TO platform_tenant_onboarding_runtime;
GRANT SELECT ON
    platform.products,
    platform_product_catalog.product_configurations,
    platform_product_catalog.active_product_catalog
TO platform_tenant_onboarding_runtime;

COMMENT ON TABLE platform_tenant_onboarding.inbox_messages IS
    'Idempotency ledger for durable CORE events; applied only in the same transaction as the consumer effect.';
COMMENT ON TABLE platform_tenant_onboarding.tenant_onboardings IS
    'PLATFORM-owned commercial reference created from CORE tenant facts; starts pending and does not activate a subscription.';
