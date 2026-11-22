-- PLATFORM owns the commercial product catalogue and versioned product configuration.
-- CORE stores only opaque product IDs and validates them through the published contract.
CREATE TABLE platform.products (
    id uuid PRIMARY KEY,
    product_key text NOT NULL UNIQUE
        CHECK (product_key ~ '^[a-z][a-z0-9-]{2,63}$'),
    display_name text NOT NULL CHECK (length(btrim(display_name)) BETWEEN 1 AND 160),
    brand_name text NOT NULL CHECK (length(btrim(brand_name)) BETWEEN 1 AND 160),
    status text NOT NULL DEFAULT 'draft'
        CHECK (status IN ('draft', 'active', 'suspended', 'retired')),
    published_configuration_version integer,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    updated_at timestamptz NOT NULL DEFAULT clock_timestamp()
);

CREATE TABLE platform.product_configurations (
    product_id uuid NOT NULL REFERENCES platform.products(id),
    version integer NOT NULL CHECK (version > 0),
    schema_version text NOT NULL CHECK (length(schema_version) BETWEEN 1 AND 64),
    configuration jsonb NOT NULL CHECK (jsonb_typeof(configuration) = 'object'),
    status text NOT NULL CHECK (status IN ('draft', 'published', 'retired')),
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    published_at timestamptz,
    PRIMARY KEY (product_id, version),
    CHECK ((status = 'published') = (published_at IS NOT NULL))
);

ALTER TABLE platform.products
    ADD CONSTRAINT products_published_configuration_fk
    FOREIGN KEY (id, published_configuration_version)
    REFERENCES platform.product_configurations(product_id, version)
    DEFERRABLE INITIALLY DEFERRED;

CREATE TABLE platform.product_domains (
    id uuid PRIMARY KEY,
    product_id uuid NOT NULL REFERENCES platform.products(id),
    domain_name text NOT NULL UNIQUE
        CHECK (domain_name = lower(domain_name)
            AND length(domain_name) BETWEEN 4 AND 253
            AND domain_name !~ '[[:space:]/:]'),
    status text NOT NULL CHECK (status IN ('pending_verification', 'active', 'suspended', 'retired')),
    verified_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    UNIQUE (product_id, domain_name),
    CHECK ((status = 'active') = (verified_at IS NOT NULL))
);

CREATE TABLE platform.product_markets (
    product_id uuid NOT NULL REFERENCES platform.products(id),
    country_code char(2) NOT NULL CHECK (country_code ~ '^[A-Z]{2}$'),
    currency_code char(3) NOT NULL CHECK (currency_code ~ '^[A-Z]{3}$'),
    locale text NOT NULL CHECK (length(locale) BETWEEN 2 AND 35),
    status text NOT NULL CHECK (status IN ('enabled', 'disabled')),
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    PRIMARY KEY (product_id, country_code, locale)
);

CREATE TABLE platform.product_surfaces (
    product_id uuid NOT NULL REFERENCES platform.products(id),
    surface_code text NOT NULL CHECK (surface_code ~ '^[a-z][a-z0-9_.-]{1,63}$'),
    configuration_version integer NOT NULL,
    status text NOT NULL CHECK (status IN ('enabled', 'disabled')),
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    PRIMARY KEY (product_id, surface_code),
    FOREIGN KEY (product_id, configuration_version)
        REFERENCES platform.product_configurations(product_id, version)
);

CREATE TABLE platform.capability_catalog (
    capability_code text PRIMARY KEY
        CHECK (capability_code ~ '^[a-z][a-z0-9_.-]{1,119}$'),
    contract_version text NOT NULL CHECK (length(contract_version) BETWEEN 1 AND 64),
    implementation_status text NOT NULL
        CHECK (implementation_status IN ('available', 'planned', 'retired')),
    created_at timestamptz NOT NULL DEFAULT clock_timestamp()
);

CREATE TABLE platform.product_capabilities (
    product_id uuid NOT NULL REFERENCES platform.products(id),
    capability_code text NOT NULL REFERENCES platform.capability_catalog(capability_code),
    configuration_version integer NOT NULL,
    status text NOT NULL CHECK (status IN ('enabled', 'disabled')),
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    PRIMARY KEY (product_id, capability_code),
    FOREIGN KEY (product_id, configuration_version)
        REFERENCES platform.product_configurations(product_id, version)
);


ALTER TABLE platform.products
    ADD CONSTRAINT products_active_requires_published_configuration
    CHECK (status <> 'active' OR published_configuration_version IS NOT NULL);

CREATE FUNCTION platform.prevent_published_configuration_mutation()
RETURNS trigger
LANGUAGE plpgsql
SET search_path = pg_catalog, platform
AS $
BEGIN
    IF OLD.status = 'published'
       AND (
           NEW.product_id IS DISTINCT FROM OLD.product_id
           OR NEW.version IS DISTINCT FROM OLD.version
           OR NEW.schema_version IS DISTINCT FROM OLD.schema_version
           OR NEW.configuration IS DISTINCT FROM OLD.configuration
           OR NEW.published_at IS DISTINCT FROM OLD.published_at
       ) THEN
        RAISE EXCEPTION 'published product configurations are immutable'
            USING ERRCODE = '55000';
    END IF;
    RETURN NEW;
END
$;

CREATE TRIGGER product_configuration_immutable
BEFORE UPDATE ON platform.product_configurations
FOR EACH ROW EXECUTE FUNCTION platform.prevent_published_configuration_mutation();

CREATE FUNCTION platform.validate_product_surface_configuration()
RETURNS trigger
LANGUAGE plpgsql
SET search_path = pg_catalog, platform
AS $
BEGIN
    IF NEW.status = 'enabled' AND NOT EXISTS (
        SELECT 1 FROM platform.product_configurations c
         WHERE c.product_id = NEW.product_id
           AND c.version = NEW.configuration_version
           AND c.status = 'published'
    ) THEN
        RAISE EXCEPTION 'enabled product surfaces require a published configuration'
            USING ERRCODE = '23514';
    END IF;
    RETURN NEW;
END
$;

CREATE TRIGGER product_surface_published_configuration
BEFORE INSERT OR UPDATE ON platform.product_surfaces
FOR EACH ROW EXECUTE FUNCTION platform.validate_product_surface_configuration();

CREATE FUNCTION platform.validate_product_capability()
RETURNS trigger
LANGUAGE plpgsql
SET search_path = pg_catalog, platform
AS $
DECLARE v_implementation_status text;
BEGIN
    SELECT implementation_status INTO v_implementation_status
      FROM platform.capability_catalog
     WHERE capability_code = NEW.capability_code;
    IF NEW.status = 'enabled' AND v_implementation_status <> 'available' THEN
        RAISE EXCEPTION 'only available capabilities may be enabled'
            USING ERRCODE = '23514';
    END IF;
    IF NEW.status = 'enabled' AND NOT EXISTS (
        SELECT 1 FROM platform.product_configurations c
         WHERE c.product_id = NEW.product_id
           AND c.version = NEW.configuration_version
           AND c.status = 'published'
    ) THEN
        RAISE EXCEPTION 'enabled product capabilities require a published configuration'
            USING ERRCODE = '23514';
    END IF;
    RETURN NEW;
END
$;

CREATE TRIGGER product_capability_available_only
BEFORE INSERT OR UPDATE ON platform.product_capabilities
FOR EACH ROW EXECUTE FUNCTION platform.validate_product_capability();

CREATE INDEX product_catalog_status_idx ON platform.products(status, product_key);
CREATE INDEX product_domains_active_idx ON platform.product_domains(domain_name)
    WHERE status = 'active';

CREATE VIEW platform.active_product_catalog AS
SELECT
    p.id AS product_id,
    p.product_key,
    p.display_name,
    p.brand_name,
    p.published_configuration_version AS configuration_version,
    c.schema_version,
    c.configuration
FROM platform.products p
JOIN platform.product_configurations c
  ON c.product_id = p.id
 AND c.version = p.published_configuration_version
 AND c.status = 'published'
WHERE p.status = 'active';

REVOKE ALL ON ALL TABLES IN SCHEMA platform FROM PUBLIC;
REVOKE ALL ON ALL SEQUENCES IN SCHEMA platform FROM PUBLIC;
GRANT SELECT ON
    platform.products,
    platform.product_configurations,
    platform.product_domains,
    platform.product_markets,
    platform.product_surfaces,
    platform.capability_catalog,
    platform.product_capabilities,
    platform.active_product_catalog
TO platform_runtime;

COMMENT ON TABLE platform.products IS
    'PLATFORM-owned canonical SaaS product catalogue; product identity is not tenant authorization.';
COMMENT ON TABLE platform.product_configurations IS
    'Immutable published configuration versions; consumers pin and validate the exact version.';
COMMENT ON TABLE platform.product_domains IS
    'Verified product routing domains; a domain lookup never authenticates an actor or tenant.';
COMMENT ON TABLE platform.capability_catalog IS
    'Canonical capability allowlist; planned or retired capabilities cannot be activated.';
COMMENT ON VIEW platform.active_product_catalog IS
    'Read-only published catalogue surface consumed by CORE through a versioned contract.';
