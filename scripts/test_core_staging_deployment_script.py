import subprocess
import unittest
from pathlib import Path


class CoreStagingDeploymentScriptTests(unittest.TestCase):
    def setUp(self):
        repository_root = Path(__file__).resolve().parents[1]
        self.script_path = repository_root / "backend/core/deployment/deploy-staging.sh"
        self.script_source = self.script_path.read_text(encoding="utf-8")

    def test_bash_syntax(self):
        result = subprocess.run(
            ["bash", "-n", str(self.script_path)],
            capture_output=True,
            check=False,
            text=True,
        )

        self.assertEqual(result.returncode, 0, result.stderr)

    def test_all_core_units_share_env_and_isolate_database_credentials(self):
        deployment = Path(__file__).resolve().parents[1] / "backend/core/deployment"
        api = (deployment / "ensambria-core.service").read_text(encoding="utf-8")
        migrator = (deployment / "ensambria-core-migrate.service").read_text(encoding="utf-8")
        worker = (deployment / "ensambria-core-email-worker.service").read_text(encoding="utf-8")

        self.assertIn("ensambria-core-email-worker.service", self.script_source)
        self.assertIn("core-email-worker", self.script_source)
        for service in (api, migrator, worker):
            self.assertIn("EnvironmentFile=/etc/apps/ensambria/core/.env", service)

        self.assertIn("UnsetEnvironment=MIGRATIONS_DATABASE_URL", api)
        self.assertIn("CORE_EMAIL_WORKER_DATABASE_URL", api)
        self.assertIn("CORE_SMTP_PASSWORD", api)

        self.assertIn("UnsetEnvironment=DATABASE_URL", migrator)
        self.assertIn("CORE_EMAIL_WORKER_DATABASE_URL", migrator)
        self.assertIn("CORE_SMTP_PASSWORD", migrator)

        self.assertIn("UnsetEnvironment=DATABASE_URL MIGRATIONS_DATABASE_URL", worker)
        self.assertIn("CORE_EMAIL_WORKER_DATABASE_URL", self.script_source)
        self.assertNotIn("email-worker.env", self.script_source)
        self.assertNotIn('systemctl enable "$email_worker_service_name"', self.script_source)

    def test_business_health_uses_configured_non_conflicting_listener(self):
        self.assertIn('BUSINESS_HTTP_LISTEN_ADDRESS', self.script_source)
        self.assertIn('business_health_url', self.script_source)
        self.assertIn('report_business_migration_failure', self.script_source)
        self.assertNotIn('127.0.0.1:8080/health', self.script_source)
        self.assertIn('127.0.0.1:${business_address##*:}', self.script_source)

    def test_embedded_python_syntax(self):
        start_marker = "| python3 -c '\n"
        end_marker = "\n'; then"
        start = self.script_source.index(start_marker) + len(start_marker)
        end = self.script_source.index(end_marker, start)
        python_source = self.script_source[start:end]

        try:
            compile(python_source, str(self.script_path), "exec")
        except SyntaxError as error:
            self.fail(f"embedded migration diagnostic Python is invalid: {error}")


if __name__ == "__main__":
    unittest.main()
