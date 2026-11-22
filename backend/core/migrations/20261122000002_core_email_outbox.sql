-- Durable transactional email outbox. Delivery is performed by a separately run CORE worker.
CREATE TABLE core.email_outbox (
    id uuid PRIMARY KEY,
    recipient text NOT NULL CHECK (length(recipient) BETWEEN 3 AND 320),
    subject text NOT NULL CHECK (length(subject) BETWEEN 1 AND 998),
    text_body text NOT NULL CHECK (length(text_body) BETWEEN 1 AND 100000),
    status text NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'processing', 'sent', 'dead')),
    attempts integer NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    available_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    locked_until timestamptz,
    last_error text,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    sent_at timestamptz,
    CONSTRAINT email_outbox_processing_lease CHECK (
        (status = 'processing' AND locked_until IS NOT NULL)
        OR (status <> 'processing' AND locked_until IS NULL)
    )
);
CREATE INDEX email_outbox_pending_idx
    ON core.email_outbox (available_at, created_at)
    WHERE status IN ('pending', 'processing');
GRANT SELECT, INSERT, UPDATE ON core.email_outbox TO core_runtime;
COMMENT ON TABLE core.email_outbox IS
    'Durable transactional email queue. Payloads contain one-time links; access is restricted to CORE runtime workers.';
