DO $ownership$
DECLARE
    actual_owner text;
BEGIN
    SELECT pg_get_userbyid(nspowner)
      INTO actual_owner
      FROM pg_namespace
     WHERE nspname = 'business_establishment_catalog';
    IF actual_owner IS DISTINCT FROM 'business_establishment_catalog_schema_owner' THEN
        RAISE EXCEPTION 'BUSINESS schema must be provisioned by the module owner; actual owner: %', actual_owner;
    END IF;
END
$ownership$;
