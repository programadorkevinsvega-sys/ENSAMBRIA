import json
import re
import subprocess
import unittest
from pathlib import Path


NATIVE = Path(__file__).resolve().parent


class NatsNativeObservabilityTests(unittest.TestCase):
    def read(self, name: str) -> str:
        return (NATIVE / name).read_text(encoding="utf-8")

    def test_shell_syntax(self) -> None:
        for script in ("install-native.sh", "prometheus-nats-exporter.sh.tmpl"):
            result = subprocess.run(
                ["bash", "-n", str(NATIVE / script)],
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(result.returncode, 0, f"{script}: {result.stderr}")

    def test_native_server_is_single_node_jetstream_with_bounded_resources(self) -> None:
        config = self.read("nats.conf.tmpl")
        self.assertIn("listen: NATS_LISTEN_ADDRESS:4222", config)
        self.assertIn("http: 127.0.0.1:8222", config)
        self.assertIn("store_dir: /var/lib/nats/jetstream", config)
        self.assertIn("max_mem: 512MB", config)
        self.assertIn("max_file: 65GB", config)
        self.assertIn("verify_and_map: true", config)
        self.assertNotIn("cluster {", config)
        self.assertNotIn("routes:", config)

    def test_services_are_hardened_and_limited(self) -> None:
        nats_service = self.read("nats-native.service")
        self.assertIn("MemoryMax=2G", nats_service)
        self.assertIn("NoNewPrivileges=true", nats_service)
        self.assertIn("ReadWritePaths=/var/lib/nats", nats_service)
        self.assertIn("ExecStartPre=/usr/local/bin/nats-server -t", nats_service)
        self.assertNotIn("Docker", nats_service)

        for filename, port, account in (
            ("node-exporter.service", "9100", "node-exporter"),
            ("process-exporter.service", "9256", "process-exporter"),
        ):
            service = self.read(filename)
            self.assertIn(f":{port}", service)
            self.assertIn(f"User={account}", service)
            self.assertIn("SupplementaryGroups=nats-tls", service)
            web_config_flag = "--web.config.file=" if filename.startswith("node") else "-web.config.file="
            self.assertIn(web_config_flag, service)
            self.assertIn("MemoryMax=", service)
            self.assertIn("NoNewPrivileges=true", service)

    def test_nats_exporter_collects_jetstream_consumers_over_tls(self) -> None:
        service = self.read("prometheus-nats-exporter.service.tmpl")
        wrapper = self.read("prometheus-nats-exporter.sh.tmpl")
        self.assertIn("EnvironmentFile=/etc/ensambria/credentials/nats/exporter-auth.env", service)
        self.assertIn("-varz", wrapper)
        self.assertIn("-connz", wrapper)
        self.assertIn("-subz", wrapper)
        self.assertIn("-jsz consumers", wrapper)
        self.assertIn("-healthz_js_enabled_only", wrapper)
        self.assertIn("-tlscert", wrapper)
        self.assertIn("-tlskey", wrapper)
        self.assertIn("-http_pass", wrapper)
        self.assertIn("bcrypt", wrapper)
        self.assertIn("MemoryMax=512M", service)
        self.assertIn("http://127.0.0.1:8222", wrapper)
        self.assertNotIn("-connz_detailed", wrapper)

    def test_prometheus_scrape_has_three_private_tls_targets_and_required_labels(self) -> None:
        scrape = self.read("prometheus-nats-scrape.yml.tmpl")
        jobs = ("ensambria-nats", "ensambria-nats-node", "ensambria-nats-processes")
        for job in jobs:
            self.assertIn(f"job_name: {job}", scrape)
        self.assertEqual(scrape.count("scheme: https"), 3)
        self.assertEqual(scrape.count("basic_auth:"), 3)
        self.assertEqual(scrape.count("insecure_skip_verify: false"), 3)
        for label in ("app: nats", "service:", "environment:", "host:"):
            self.assertIn(label, scrape)
        for password_file in (
            "NATS_PROMETHEUS_PASSWORD_FILE",
            "NATS_NODE_PROMETHEUS_PASSWORD_FILE",
            "NATS_PROCESS_PROMETHEUS_PASSWORD_FILE",
        ):
            self.assertIn(f"password_file: {password_file}", scrape)

    def test_host_dashboard_covers_required_node_exporter_metrics(self) -> None:
        dashboard = json.loads(self.read("grafana-nats-dashboard.json"))
        self.assertEqual(dashboard["title"], "ENSAMBRIA NATS JetStream + Host")
        expressions = " ".join(
            target["expr"]
            for panel in dashboard["panels"]
            for target in panel.get("targets", [])
        )
        for metric in (
            "node_cpu_seconds_total",
            "node_memory_MemTotal_bytes",
            "node_memory_MemAvailable_bytes",
            "node_memory_Cached_bytes",
            "node_memory_Buffers_bytes",
            "node_load1",
            "node_load5",
            "node_load15",
            "node_filesystem_size_bytes",
            "node_filesystem_avail_bytes",
            "node_disk_read_bytes_total",
            "node_disk_written_bytes_total",
            "node_network_receive_bytes_total",
            "node_network_transmit_bytes_total",
            "node_uname_info",
        ):
            self.assertIn(metric, expressions)

    def test_dashboard_covers_nats_process_and_consumer_signals(self) -> None:
        dashboard = json.loads(self.read("grafana-nats-dashboard.json"))
        titles = {panel["title"] for panel in dashboard["panels"]}
        for title in (
            "JetStream health",
            "NATS server CPU %",
            "NATS server memory",
            "Message throughput",
            "Connections and slow consumers",
            "JetStream storage (used / configured limit)",
            "JetStream memory store",
            "Per-stream storage",
            "Consumer sequence progress",
            "Top monitored processes by CPU",
            "Top monitored processes by resident memory",
            "HTTP request latency (p50 / p95 / p99)",
            "HTTP request rate",
            "HTTP 5xx error rate",
            "Application process CPU",
            "Application resident memory",
        ):
            self.assertIn(title, titles)
        expressions = " ".join(
            target["expr"]
            for panel in dashboard["panels"]
            for target in panel.get("targets", [])
        )
        for metric in (
            "jetstream_consumer_num_pending",
            "jetstream_consumer_num_ack_pending",
            "jetstream_consumer_num_redelivered",
            "jetstream_consumer_delivered_stream_seq",
            "jetstream_consumer_ack_floor_stream_seq",
            "jetstream_stream_total_bytes",
            "gnatsd_varz_cpu",
            "gnatsd_varz_mem",
            "namedprocess_namegroup_cpu_seconds_total",
            "namedprocess_namegroup_memory_bytes",
            "http_requests_total",
            "http_request_duration_seconds_bucket",
            "process_cpu_seconds_total",
            "process_resident_memory_bytes",
        ):
            self.assertIn(metric, expressions)
        self.assertIn("http_requests_total", dashboard["description"])
        self.assertIn("http_request_duration_seconds_bucket", dashboard["description"])

    def test_dashboard_panel_ids_and_grid_widths_are_valid(self) -> None:
        dashboard = json.loads(self.read("grafana-nats-dashboard.json"))
        panels = dashboard["panels"]
        panel_ids = [panel["id"] for panel in panels]
        self.assertEqual(len(panel_ids), len(set(panel_ids)))
        for panel in panels:
            grid = panel["gridPos"]
            self.assertGreater(grid["w"], 0)
            self.assertGreaterEqual(grid["x"], 0)
            self.assertLessEqual(grid["x"] + grid["w"], 24)

    def test_process_exporter_is_limited_to_named_groups(self) -> None:
        config = self.read("process-exporter.yml")
        service = self.read("process-exporter.service")
        for process in (
            "nats-server",
            "prometheus-nats",
            "node_exporter",
            "process-exporte",
        ):
            self.assertIn(process, config)
        self.assertIn("-children=false", service)
        self.assertIn("-threads=false", service)
        self.assertIn("-gather-smaps=false", service)
        self.assertNotIn("cmdline", config)

    def test_installer_pins_official_binaries_with_checksums(self) -> None:
        installer = self.read("install-native.sh")
        for version in ("v2.15.3", "v0.20.3", "v1.13.1", "v0.8.7"):
            self.assertIn(version, installer)
        self.assertEqual(installer.count("sha256sum --check --status"), 4)
        for variable in (
            "NATS_SHA256",
            "EXPORTER_SHA256",
            "NODE_EXPORTER_SHA256",
            "PROCESS_EXPORTER_SHA256",
        ):
            self.assertRegex(installer, re.compile(rf"{variable}=[a-f0-9]{{64}}"))

    def test_authorization_template_fails_closed_and_has_no_active_credentials(self) -> None:
        authorization = self.read("authorization.conf.example")
        installer = self.read("install-native.sh")
        self.assertIn('deny: [">"]', authorization)
        self.assertIn("verify_and_map: true", self.read("nats.conf.tmpl"))
        self.assertIn("NATS_AUTHORIZATION_FILE", installer)
        self.assertIn("password authentication is not accepted", installer)
        self.assertNotRegex(authorization, r"(?m)^\s*user:\s*[^#]")

    def test_all_exporter_passwords_require_external_bcrypt_credentials(self) -> None:
        installer = self.read("install-native.sh")
        examples = (
            "exporter-auth.env.example",
            "node-exporter-auth.env.example",
            "process-exporter-auth.env.example",
        )
        for filename in examples:
            auth_example = self.read(filename)
            self.assertIn("REPLACE_WITH_BCRYPT_HASH", auth_example)
        for auth_file in (
            "exporter-auth.env",
            "node-exporter-auth.env",
            "process-exporter-auth.env",
        ):
            self.assertIn(f"root-owned {auth_file}", installer)
        self.assertIn(r"^\$2a\$[0-9]{2}\$[./A-Za-z0-9]{53}$", installer)


if __name__ == "__main__":
    unittest.main()
