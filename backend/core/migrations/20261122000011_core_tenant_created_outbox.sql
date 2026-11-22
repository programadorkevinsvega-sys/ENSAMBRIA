-- Durable CORE -> PLATFORM tenant-created event delivery (subphase 1.3).
-- The business mutation and this row must be inserted in the same CORE transaction.
CREATE TABLE core.outbox_events (
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
    )
);

CREATE INDEX outbox_events_claim_idx
    ON core.outbox_events (available_at, occurred_at, event_id)
    WHERE status IN ('pending', 'retry_wait');

CREATE INDEX outbox_events_expired_lease_idx
    ON core.outbox_events (lease_until, occurred_at, event_id)
    WHERE status = 'leased';

REVOKE ALL ON core.outbox_events FROM PUBLIC;
GRANT USAGE ON SCHEMA core TO core_outbox_relay;
GRANT INSERT ON core.outbox_events TO core_runtime;
GRANT SELECT, UPDATE ON core.outbox_events TO core_outbox_relay;

COMMENT ON TABLE core.outbox_events IS
    'Durable CORE-owned facts awaiting confirmed JetStream publication; inserted atomically with the tenant aggregate.';
