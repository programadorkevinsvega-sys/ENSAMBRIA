-- PLATFORM owns the versioned commercial offer and subscription contract.
-- No price, currency, tax, trial, billing interval or payment behavior is seeded.
CREATE TABLE platform_product_catalog.commercial_offers (
    id uuid PRIMARY KEY,
    product_id uuid NOT NULL REFERENCES platform_product_catalog.products(id),
    offer_key text NOT NULL CHECK (offer_key ~ '^[a-z][a-z0-9-]{2,63}$'),
    display_name text NOT NULL CHECK (length(btrim(display_name)) BETWEEN 1 AND 160),
    status text NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'published', 'retired')),
    created_by uuid NOT NULL,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    updated_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    UNIQUE (product_id, offer_key),
    UNIQUE (id, product_id)
);
CREATE TABLE platform_product_catalog.commercial_offer_versions (
    offer_id uuid NOT NULL REFERENCES platform_product_catalog.commercial_offers(id),
    version integer NOT NULL CHECK (version > 0),
    entitlements jsonb NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(entitlements) = 'object'),
    commercial_terms jsonb NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(commercial_terms) = 'object'),
    status text NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'published', 'retired')),
    created_by uuid NOT NULL,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    published_at timestamptz,
    PRIMARY KEY (offer_id, version),
    CHECK ((status = 'published') = (published_at IS NOT NULL))
);
ALTER TABLE platform_product_catalog.commercial_offers ADD COLUMN published_version integer,
    ADD CONSTRAINT commercial_offers_published_version_fk
    FOREIGN KEY (id, published_version)
    REFERENCES platform_product_catalog.commercial_offer_versions(offer_id, version)
    DEFERRABLE INITIALLY DEFERRED;

CREATE FUNCTION platform_product_catalog.prevent_published_offer_version_mutation()
RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, platform AS $$
BEGIN
    IF OLD.published_at IS NOT NULL AND (
        NEW.offer_id IS DISTINCT FROM OLD.offer_id OR NEW.version IS DISTINCT FROM OLD.version
        OR NEW.entitlements IS DISTINCT FROM OLD.entitlements
        OR NEW.commercial_terms IS DISTINCT FROM OLD.commercial_terms
        OR NEW.published_at IS DISTINCT FROM OLD.published_at
    ) THEN
        RAISE EXCEPTION 'published commercial offer versions are immutable' USING ERRCODE = '55000';
    END IF;
    IF OLD.published_at IS NOT NULL AND NEW.status = 'retired'
       AND EXISTS (SELECT 1 FROM platform_product_catalog.commercial_offers o
                   WHERE o.id = OLD.offer_id AND o.published_version = OLD.version) THEN
        RAISE EXCEPTION 'current published offer version cannot be retired' USING ERRCODE = '55000';
    END IF;
    RETURN NEW;
END $$;
CREATE TRIGGER commercial_offer_version_immutable
BEFORE UPDATE ON platform_product_catalog.commercial_offer_versions
FOR EACH ROW EXECUTE FUNCTION platform_product_catalog.prevent_published_offer_version_mutation();

CREATE FUNCTION platform_product_catalog.validate_published_offer()
RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, platform AS $$
BEGIN
    IF NEW.status = 'published' AND (
        NEW.published_version IS NULL OR NOT EXISTS (
            SELECT 1 FROM platform_product_catalog.commercial_offer_versions v
            WHERE v.offer_id = NEW.id AND v.version = NEW.published_version
              AND v.status = 'published' AND v.published_at IS NOT NULL
        )
    ) THEN
        RAISE EXCEPTION 'published offer requires an immutable published version' USING ERRCODE = '23514';
    END IF;
    RETURN NEW;
END $$;
CREATE TRIGGER commercial_offer_publication_required
BEFORE INSERT OR UPDATE OF status, published_version ON platform_product_catalog.commercial_offers
FOR EACH ROW EXECUTE FUNCTION platform_product_catalog.validate_published_offer();

CREATE TABLE platform_product_catalog.saas_subscriptions (
    id uuid PRIMARY KEY,
    tenant_id uuid NOT NULL,
    product_id uuid NOT NULL REFERENCES platform_product_catalog.products(id),
    offer_id uuid NOT NULL,
    offer_version integer NOT NULL,
    entitlements_snapshot jsonb NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(entitlements_snapshot) = 'object'),
    status text NOT NULL CHECK (status IN ('active', 'suspended', 'cancelled', 'expired')),
    starts_at timestamptz NOT NULL,
    ends_at timestamptz,
    suspended_at timestamptz,
    cancelled_at timestamptz,
    version bigint NOT NULL DEFAULT 1 CHECK (version > 0),
    created_by uuid NOT NULL,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    updated_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (offer_id, offer_version) REFERENCES platform_product_catalog.commercial_offer_versions(offer_id, version),
    FOREIGN KEY (offer_id, product_id) REFERENCES platform_product_catalog.commercial_offers(id, product_id),
    UNIQUE (id, tenant_id),
    CHECK (ends_at IS NULL OR ends_at > starts_at),
    CHECK ((status = 'suspended') = (suspended_at IS NOT NULL)),
    CHECK ((status = 'cancelled') = (cancelled_at IS NOT NULL))
);
CREATE UNIQUE INDEX saas_subscriptions_one_live_contract
    ON platform_product_catalog.saas_subscriptions (tenant_id, product_id)
    WHERE status IN ('active', 'suspended');
CREATE INDEX saas_subscriptions_expiry_idx
    ON platform_product_catalog.saas_subscriptions (ends_at, id)
    WHERE status = 'active' AND ends_at IS NOT NULL;

CREATE TABLE platform_product_catalog.saas_subscription_audit (
    audit_id uuid PRIMARY KEY,
    subscription_id uuid NOT NULL,
    tenant_id uuid NOT NULL,
    actor_id uuid NOT NULL,
    action text NOT NULL CHECK (action IN ('created', 'suspended', 'reactivated', 'renewed', 'cancelled', 'expired')),
    previous_status text,
    new_status text NOT NULL,
    previous_version bigint,
    new_version bigint NOT NULL,
    reason_code text,
    occurred_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (subscription_id, tenant_id) REFERENCES platform_product_catalog.saas_subscriptions(id, tenant_id)
);
CREATE TABLE platform_product_catalog.saas_subscription_commands (
    tenant_id uuid NOT NULL,
    idempotency_key text NOT NULL CHECK (length(idempotency_key) BETWEEN 16 AND 128),
    request_hash bytea NOT NULL CHECK (octet_length(request_hash) = 32),
    subscription_id uuid,
    response jsonb,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    PRIMARY KEY (tenant_id, idempotency_key),
    FOREIGN KEY (subscription_id, tenant_id) REFERENCES platform_product_catalog.saas_subscriptions(id, tenant_id)
);
CREATE FUNCTION platform_product_catalog.prevent_published_offer_deletion()
RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, platform AS $$
BEGIN
    IF OLD.published_at IS NOT NULL THEN
        RAISE EXCEPTION 'published offer versions cannot be deleted' USING ERRCODE = '55000';
    END IF;
    RETURN OLD;
END $$;
CREATE TRIGGER commercial_offer_version_no_delete
BEFORE DELETE ON platform_product_catalog.commercial_offer_versions
FOR EACH ROW EXECUTE FUNCTION platform_product_catalog.prevent_published_offer_deletion();

CREATE FUNCTION platform_product_catalog.protect_published_offer_identity()
RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, platform AS $$
BEGIN
    IF OLD.status <> 'draft' AND (
        NEW.id IS DISTINCT FROM OLD.id OR NEW.product_id IS DISTINCT FROM OLD.product_id
        OR NEW.offer_key IS DISTINCT FROM OLD.offer_key OR NEW.display_name IS DISTINCT FROM OLD.display_name
        OR NEW.created_by IS DISTINCT FROM OLD.created_by
    ) THEN
        RAISE EXCEPTION 'published offer identity is immutable' USING ERRCODE = '55000';
    END IF;
    IF OLD.status = 'published' AND NEW.status NOT IN ('published','retired') THEN
        RAISE EXCEPTION 'published offers can only remain published or be retired' USING ERRCODE = '55000';
    END IF;
    IF OLD.status = 'retired' AND NEW.status <> 'retired' THEN
        RAISE EXCEPTION 'retired offers cannot be reactivated' USING ERRCODE = '55000';
    END IF;
    RETURN NEW;
END $$;
CREATE TRIGGER commercial_offer_identity_immutable
BEFORE UPDATE ON platform_product_catalog.commercial_offers
FOR EACH ROW EXECUTE FUNCTION platform_product_catalog.protect_published_offer_identity();

CREATE FUNCTION platform_product_catalog.validate_subscription_transition()
RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, platform AS $$
BEGIN
    IF NEW.tenant_id IS DISTINCT FROM OLD.tenant_id OR NEW.product_id IS DISTINCT FROM OLD.product_id
       OR NEW.offer_id IS DISTINCT FROM OLD.offer_id OR NEW.offer_version IS DISTINCT FROM OLD.offer_version
       OR NEW.created_by IS DISTINCT FROM OLD.created_by OR NEW.starts_at IS DISTINCT FROM OLD.starts_at THEN
        RAISE EXCEPTION 'subscription contract identity is immutable' USING ERRCODE = '55000';
    END IF;
    IF NEW.version <> OLD.version + 1 THEN
        RAISE EXCEPTION 'subscription version must increase exactly once' USING ERRCODE = '40001';
    END IF;
    IF NOT (
        (OLD.status = 'active' AND NEW.status IN ('active','suspended','cancelled','expired'))
        OR (OLD.status = 'suspended' AND NEW.status IN ('active','cancelled','expired'))
    ) THEN
        RAISE EXCEPTION 'invalid subscription lifecycle transition' USING ERRCODE = '23514';
    END IF;
    RETURN NEW;
END $$;
CREATE TRIGGER saas_subscription_transition_guard
BEFORE UPDATE ON platform_product_catalog.saas_subscriptions
FOR EACH ROW EXECUTE FUNCTION platform_product_catalog.validate_subscription_transition();

CREATE TABLE platform_product_catalog.saas_subscription_outbox (
    event_id uuid PRIMARY KEY,
    tenant_id uuid NOT NULL,
    subscription_id uuid NOT NULL,
    aggregate_version bigint NOT NULL CHECK (aggregate_version > 0),
    event_type text NOT NULL CHECK (event_type IN ('platform.saas-subscription.created.v1', 'platform.saas-subscription.changed.v1')),
    schema_version integer NOT NULL DEFAULT 1 CHECK (schema_version = 1),
    payload jsonb NOT NULL CHECK (jsonb_typeof(payload) = 'object'),
    occurred_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    published_at timestamptz,
    attempts integer NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    last_error_code text,
    UNIQUE (subscription_id, aggregate_version),
    FOREIGN KEY (subscription_id, tenant_id) REFERENCES platform_product_catalog.saas_subscriptions(id, tenant_id)
);
CREATE INDEX saas_subscription_outbox_pending_idx
    ON platform_product_catalog.saas_subscription_outbox (occurred_at, event_id) WHERE published_at IS NULL;

DO $$
DECLARE table_name text;
BEGIN
    FOREACH table_name IN ARRAY ARRAY[
        'saas_subscriptions', 'saas_subscription_audit',
        'saas_subscription_commands', 'saas_subscription_outbox'
    ] LOOP
        EXECUTE format('ALTER TABLE platform_product_catalog.%I ENABLE ROW LEVEL SECURITY', table_name);
        EXECUTE format('ALTER TABLE platform_product_catalog.%I FORCE ROW LEVEL SECURITY', table_name);
        EXECUTE format(
            'CREATE POLICY tenant_scope ON platform_product_catalog.%I USING (tenant_id = NULLIF(current_setting(''app.tenant_id'', true), '''')::uuid) WITH CHECK (tenant_id = NULLIF(current_setting(''app.tenant_id'', true), '''')::uuid)',
            table_name
        );
    END LOOP;
END $$;
ALTER TABLE platform_product_catalog.commercial_offers ENABLE ROW LEVEL SECURITY;
CREATE POLICY commercial_offers_read_published ON platform_product_catalog.commercial_offers FOR SELECT USING (status = 'published');
ALTER TABLE platform_product_catalog.commercial_offer_versions ENABLE ROW LEVEL SECURITY;
CREATE POLICY commercial_offer_versions_read_published ON platform_product_catalog.commercial_offer_versions FOR SELECT USING (status = 'published');

REVOKE ALL ON platform_product_catalog.commercial_offers, platform_product_catalog.commercial_offer_versions,
    platform_product_catalog.saas_subscriptions, platform_product_catalog.saas_subscription_audit,
    platform_product_catalog.saas_subscription_commands, platform_product_catalog.saas_subscription_outbox FROM PUBLIC;
GRANT SELECT ON platform_product_catalog.commercial_offers, platform_product_catalog.commercial_offer_versions TO platform_product_catalog_runtime;
GRANT SELECT, INSERT, UPDATE ON platform_product_catalog.saas_subscriptions, platform_product_catalog.saas_subscription_audit,
    platform_product_catalog.saas_subscription_commands, platform_product_catalog.saas_subscription_outbox TO platform_product_catalog_runtime;

COMMENT ON TABLE platform_product_catalog.commercial_offers IS 'PLATFORM-owned SaaS offer catalogue; publication never assigns a tenant.';
COMMENT ON TABLE platform_product_catalog.commercial_offer_versions IS 'Immutable published offer snapshots; CORE evaluates entitlements.';
COMMENT ON TABLE platform_product_catalog.saas_subscriptions IS 'Explicit tenant/product contracts; tenant onboarding does not create subscriptions.';
COMMENT ON TABLE platform_product_catalog.saas_subscription_outbox IS 'Transactional contract events; payment and billing effects are out of scope.';
