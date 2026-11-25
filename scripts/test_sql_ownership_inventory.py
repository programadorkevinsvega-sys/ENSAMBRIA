import importlib.util
import sys
import unittest
from pathlib import Path

validator_path = Path(__file__).parent / "ci" / "check-sql-ownership-inventory.py"
spec = importlib.util.spec_from_file_location("check_sql_ownership_inventory", validator_path)
if spec is None or spec.loader is None:
    raise RuntimeError(f"cannot load SQL ownership validator: {validator_path}")
validator = importlib.util.module_from_spec(spec)
spec.loader.exec_module(validator)
discover_sql_objects = validator.discover_sql_objects
validate_inventory = validator.validate_inventory
sys.path.insert(0, str(Path(__file__).parent / "ci"))
from render_postgres_policy_values import render_policy_values


class SqlOwnershipInventoryTests(unittest.TestCase):
    def test_discovers_sequence_table_function_trigger_and_policy(self):
        sql = """
        CREATE SEQUENCE core_tenancy.event_seq;
        CREATE TABLE core_tenancy.events (id bigint);
        CREATE FUNCTION core_tenancy.next_event() RETURNS bigint
          LANGUAGE sql AS 'SELECT 1';
        CREATE TRIGGER events_guard BEFORE INSERT ON core_tenancy.events
          FOR EACH ROW EXECUTE FUNCTION core_tenancy.next_event();
        CREATE POLICY events_scope ON core_tenancy.events USING (true);
        """
        found = discover_sql_objects(sql)["core_tenancy"]
        self.assertEqual(found["sequences"], {"event_seq"})
        self.assertEqual(found["tables"], {"events"})
        self.assertEqual(found["functions"], {"next_event"})
        self.assertEqual(found["triggers"], {"events_guard"})
        self.assertEqual(found["policies"], {"events.events_scope"})

    def test_expands_dynamic_policy_names_and_tables_from_foreach(self):
        sql = """
        DO $$
        DECLARE relation_name text;
        BEGIN
          FOREACH relation_name IN ARRAY ARRAY['accounts', 'sessions'] LOOP
            EXECUTE format(
              'CREATE POLICY %I ON app_data.%I USING (tenant_id = current_setting(''app.tenant_id''))',
              relation_name || '_tenant_scope', relation_name
            );
          END LOOP;
        END $$;
        """
        policies = discover_sql_objects(sql)["app_data"]["policies"]
        self.assertEqual(
            policies,
            {"accounts.accounts_tenant_scope", "sessions.sessions_tenant_scope"},
        )

    def test_expands_dynamic_policy_with_literal_name(self):
        sql = """
        DO $$
        DECLARE table_name text;
        BEGIN
          FOREACH table_name IN ARRAY ARRAY['orders', 'invoices'] LOOP
            EXECUTE format(
              'CREATE POLICY tenant_scope ON app_data.%I USING (tenant_id = current_setting(''app.tenant_id''))',
              table_name
            );
          END LOOP;
        END $$;
        """
        policies = discover_sql_objects(sql)["app_data"]["policies"]
        self.assertEqual(policies, {"orders.tenant_scope", "invoices.tenant_scope"})

    def test_missing_dynamic_policy_is_a_hard_inventory_error(self):
        inventory = """version: 1
modules:
  - id: app.data
    schema: app_data
    policies:
      - accounts.accounts_tenant_scope
    tables:
      - accounts
"""
        sql = """
        CREATE TABLE app_data.accounts (id uuid);
        DO $$
        DECLARE relation_name text;
        BEGIN
          FOREACH relation_name IN ARRAY ARRAY['accounts', 'sessions'] LOOP
            EXECUTE format(
              'CREATE POLICY %I ON app_data.%I USING (true)',
              relation_name || '_tenant_scope', relation_name
            );
          END LOOP;
        END $$;
        """
        errors = validate_inventory(inventory, [("migration.sql", sql)])
        self.assertTrue(
            any("sessions.sessions_tenant_scope" in error and "missing from inventory" in error
                for error in errors),
            errors,
        )

    def test_catalog_policy_values_are_rendered_from_the_canonical_inventory(self):
        inventory = """version: 1
modules:
  - id: app.data
    schema: app_data
    policies:
      - accounts.accounts_tenant_scope
      - sessions.sessions_tenant_scope
    tables:
      - accounts
      - sessions
"""
        self.assertEqual(
            render_policy_values(inventory),
            "('app_data', 'accounts', 'accounts_tenant_scope'),\n        "
            "('app_data', 'sessions', 'sessions_tenant_scope')",
        )

    def test_missing_sequence_is_a_hard_inventory_error(self):
        inventory = """version: 1
modules:
  - id: app.data
    schema: app_data
    sequences:
    tables:
      - accounts
"""
        sql = """
        CREATE TABLE app_data.accounts (id bigint);
        CREATE SEQUENCE app_data.account_id_seq;
        """
        errors = validate_inventory(inventory, [("migration.sql", sql)])
        self.assertTrue(
            any("account_id_seq" in error and "missing from inventory" in error
                for error in errors),
            errors,
        )


if __name__ == "__main__":
    unittest.main()
