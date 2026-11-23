CREATE TABLE business.establishments (
    id uuid PRIMARY KEY,
    tenant_id uuid NOT NULL,
    organization_id uuid NOT NULL,
    branch_id uuid,
    display_name text NOT NULL CHECK (length(btrim(display_name)) BETWEEN 1 AND 200),
    status text NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'inactive')),
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    updated_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    UNIQUE (tenant_id, id)
);
CREATE INDEX establishments_tenant_created_idx
    ON business.establishments (tenant_id, created_at DESC, id DESC);
ALTER TABLE business.establishments ENABLE ROW LEVEL SECURITY;
ALTER TABLE business.establishments FORCE ROW LEVEL SECURITY;
CREATE POLICY establishments_tenant_scope ON business.establishments
    USING (tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::uuid)
    WITH CHECK (tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::uuid);
GRANT USAGE ON SCHEMA business TO business_runtime;
GRANT SELECT, INSERT, UPDATE ON business.establishments TO business_runtime;
