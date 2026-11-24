DO $ownership$
DECLARE
    identity_owner text;
    tenancy_owner text;
BEGIN
    SELECT pg_get_userbyid(nspowner)
      INTO identity_owner
      FROM pg_namespace
     WHERE nspname = 'core_identity_auth';
    SELECT pg_get_userbyid(nspowner)
      INTO tenancy_owner
      FROM pg_namespace
     WHERE nspname = 'core_tenancy';

    IF identity_owner IS DISTINCT FROM 'core_identity_auth_schema_owner' THEN
        RAISE EXCEPTION 'CORE identity schema must be pre-provisioned by its owner';
    END IF;
    IF tenancy_owner IS DISTINCT FROM 'core_tenancy_schema_owner' THEN
        RAISE EXCEPTION 'CORE tenancy schema must be pre-provisioned by its owner';
    END IF;
END
$ownership$;
