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

    def test_email_worker_is_packaged_and_installed_without_auto_enabling(self):
        self.assertIn("ensambria-core-email-worker.service", self.script_source)
        self.assertIn("core-email-worker", self.script_source)
        self.assertIn("email-worker.env", self.script_source)
        self.assertNotIn('systemctl enable "$email_worker_service_name"', self.script_source)

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
