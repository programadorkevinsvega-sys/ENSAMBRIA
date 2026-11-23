-- Durable Inbox and commercial onboarding projection for CORE tenant-created events.
-- The consumer inserts Inbox + onboarding in one PLATFORM transaction before ACKing JetStream.
CREATE TABLE platform.inbox_messages (
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
    ON platform.inbox_messages (received_at, event_id)
    WHERE status = 'received';

-- Both tables carry tenant data. The durable consumer sets this transaction-local
-- context from the validated event before touching either table.
ALTER TABLE platform.inbox_messages ENABLE ROW LEVEL SECURITY;
ALTER TABLE platform.inbox_messages FORCE ROW LEVEL SECURITY;
CREATE POLICY inbox_tenant_scope ON platform.inbox_messages
    USING (aggregate_id = NULLIF(current_setting('app.tenant_id', true), '')::uuid)
    WITH CHECK (aggregate_id = NULLIF(current_setting('app.tenant_id', true), '')::uuid);



CREATE TABLE platform.tenant_onboardings (
    tenant_id uuid NOT NULL,
    product_id uuid NOT NULL REFERENCES platform.products(id),
    product_config_version integer NOT NULL CHECK (product_config_version > 0),
    source_event_id uuid NOT NULL UNIQUE REFERENCES platform.inbox_messages(event_id),
    status text NOT NULL DEFAULT 'onboarding_pending'
        CHECK (status = 'onboarding_pending'),
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    PRIMARY KEY (tenant_id, product_id)
);

REVOKE ALL ON
    platform.inbox_messages,
    platform.tenant_onboardings
FROM PUBLIC;

GRANT USAGE ON SCHEMA platform TO platform_event_consumer;
GRANT SELECT, INSERT, UPDATE ON platform.inbox_messages TO platform_event_consumer;
GRANT SELECT, INSERT, UPDATE ON platform.tenant_onboardings TO platform_event_consumer;
GRANT SELECT ON
    platform.products,
    platform.product_configurations,
    platform.active_product_catalog
TO platform_event_consumer;

COMMENT ON TABLE platform.inbox_messages IS
    'Idempotency ledger for durable CORE events; applied only in the same transaction as the consumer effect.';
COMMENT ON TABLE platform.tenant_onboardings IS
    'PLATFORM-owned commercial reference created from CORE tenant facts; starts pending and does not activate a subscription.';
