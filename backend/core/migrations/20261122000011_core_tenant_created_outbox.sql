RESET ROLE;
SET LOCAL ROLE core_tenancy_schema_owner;

-- Durable CORE -> PLATFORM tenant-created event delivery (subphase 1.3).
-- The business mutation and this row must be inserted in the same CORE transaction.
CREATE TABLE core_tenancy.outbox_events (
    event_id uuid PRIMARY KEY,
    event_type text NOT NULL CHECK (event_type = 'core.tenant.created.v1'),
    schema_version integer NOT NULL CHECK (schema_version = 1),
    aggregate_type text NOT NULL CHECK (aggregate_type = 'tenant'),
    aggregate_id uuid NOT NULL,
    aggregate_version bigint NOT NULL CHECK (aggregate_version > 0),
    product_id uuid NOT NULL,
    product_config_version integer NOT NULL CHECK (product_config_version > 0),
    correlation_id uuid NOT NULL,
    payload jsonb NOT NULL CHECK (jsonb_typeof(payload) = 'object'),
    status text NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'leased', 'retry_wait', 'published', 'quarantined')),
    attempt_count integer NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),
    available_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    lease_owner text,
    lease_until timestamptz,
    published_at timestamptz,
    last_error_code text
        CHECK (last_error_code IS NULL OR last_error_code ~ '^[A-Z][A-Z0-9_]{2,79}$'),
    occurred_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    CONSTRAINT outbox_lease_shape CHECK (
        (status = 'leased' AND lease_owner IS NOT NULL AND lease_until IS NOT NULL)
        OR (status <> 'leased' AND lease_owner IS NULL AND lease_until IS NULL)
    ),
    CONSTRAINT outbox_published_shape CHECK (
        (status = 'published') = (published_at IS NOT NULL)
    ),
    CONSTRAINT outbox_error_shape CHECK (
        status NOT IN ('retry_wait', 'quarantined') OR last_error_code IS NOT NULL
    ),
    CONSTRAINT outbox_tenant_created_payload_shape CHECK (
        payload->>'tenantId' = aggregate_id::text
        AND payload->>'productId' = product_id::text
        AND payload->>'productConfigVersion' = product_config_version::text
    )
);

CREATE INDEX outbox_events_claim_idx
    ON core_tenancy.outbox_events (available_at, occurred_at, event_id)
    WHERE status IN ('pending', 'retry_wait');

CREATE INDEX outbox_events_expired_lease_idx
    ON core_tenancy.outbox_events (lease_until, occurred_at, event_id)
    WHERE status = 'leased';

ALTER TABLE core_tenancy.outbox_events ENABLE ROW LEVEL SECURITY;
ALTER TABLE core_tenancy.outbox_events FORCE ROW LEVEL SECURITY;
CREATE POLICY outbox_tenant_insert ON core_tenancy.outbox_events
    FOR INSERT TO core_tenancy_runtime
    WITH CHECK (
        aggregate_id = NULLIF(current_setting('app.tenant_id', true), '')::uuid
    );
CREATE POLICY outbox_relay_read ON core_tenancy.outbox_events
    FOR SELECT TO core_outbox_relay
    USING (true);
CREATE POLICY outbox_relay_update ON core_tenancy.outbox_events
    FOR UPDATE TO core_outbox_relay
    USING (true)
    WITH CHECK (true);

REVOKE ALL ON core_tenancy.outbox_events FROM PUBLIC;
GRANT USAGE ON SCHEMA core TO core_outbox_relay;
GRANT INSERT ON core_tenancy.outbox_events TO core_tenancy_runtime;
GRANT SELECT, UPDATE ON core_tenancy.outbox_events TO core_outbox_relay;

COMMENT ON TABLE core_tenancy.outbox_events IS
    'Durable CORE-owned facts awaiting confirmed JetStream publication; inserted atomically with the tenant aggregate.';

GRANT USAGE ON SCHEMA core_tenancy TO core_outbox_relay;
