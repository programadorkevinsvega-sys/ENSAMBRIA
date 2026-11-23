import subprocess
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).with_name("check_expand_contract_migrations.py")


class ExpandContractMigrationPolicyTests(unittest.TestCase):
    def run_policy(self, sql):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "migration.sql").write_text(sql, encoding="utf-8")
            return subprocess.run(
                ["python3", str(SCRIPT), str(root)],
                capture_output=True,
                text=True,
                check=False,
            )

    def test_additive_migration_is_allowed(self):
        result = self.run_policy(
            "ALTER TABLE business.establishments ADD COLUMN IF NOT EXISTS display_name text;"
        )
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_drop_column_is_rejected(self):
        result = self.run_policy("ALTER TABLE business.establishments DROP COLUMN name;")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("DROP requires a later contract migration", result.stderr)

    def test_drop_function_is_rejected_for_rollback_compatibility(self):
        result = self.run_policy("DROP FUNCTION core.old_contract(bytea);")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("DROP requires a later contract migration", result.stderr)

    def test_destructive_keyword_in_comment_is_ignored(self):
        result = self.run_policy("-- DROP TABLE old_table;\nSELECT 1;")
        self.assertEqual(result.returncode, 0, result.stderr)


if __name__ == "__main__":
    unittest.main()
