import unittest

from validate_engineering_skills import (
    has_review_date,
    parse_agent_frontmatter,
    validate_copilot_agent_header,
)


class CopilotAgentFrontmatterTests(unittest.TestCase):
    def test_valid_copilot_coordinator_configuration(self):
        header = """name: Enterprise Architecture Governance
description: "Coordinate repository work."
target: github-copilot
tools: ['agent', 'read', 'search', 'edit', 'execute']
skills: ['enterprise-architecture-governance']
user-invocable: true
disable-model-invocation: false"""

        self.assertEqual(
            validate_copilot_agent_header(
                header,
                "Enterprise Architecture Governance",
                "enterprise-architecture-governance",
                True,
            ),
            [],
        )

    def test_local_target_is_rejected(self):
        header = self.coordinator_header().replace("github-copilot", "vscode")

        errors = validate_copilot_agent_header(
            header,
            "Enterprise Architecture Governance",
            "enterprise-architecture-governance",
            True,
        )

        self.assertIn("target must be github-copilot", errors)

    def test_local_only_frontmatter_fields_are_rejected(self):
        for field in ("agents", "handoffs", "argument-hint", "advancedOptions"):
            with self.subTest(field=field):
                header = f"{self.coordinator_header()}\n{field}: []"
                errors = validate_copilot_agent_header(
                    header,
                    "Enterprise Architecture Governance",
                    "enterprise-architecture-governance",
                    True,
                )
                self.assertTrue(
                    any(
                        f"unsupported Copilot frontmatter fields ['{field}']" == error
                        for error in errors
                    ),
                    errors,
                )

    def test_skill_mapping_and_tool_aliases_are_checked(self):
        header = self.coordinator_header().replace(
            "enterprise-architecture-governance", "unknown-skill"
        ).replace("'execute'", "'agent/runSubagent'")

        errors = validate_copilot_agent_header(
            header,
            "Enterprise Architecture Governance",
            "enterprise-architecture-governance",
            True,
        )

        self.assertIn(
            "unsupported Copilot tool aliases ['agent/runSubagent']",
            errors,
        )
        self.assertIn(
            "skills must reference exactly enterprise-architecture-governance",
            errors,
        )

    def test_specialist_is_hidden_but_model_invocable(self):
        header = """name: Rust Quality Engineering
description: "Assess Rust quality."
target: github-copilot
tools: ['read', 'search', 'execute']
skills: ['rust-quality-engineering']
user-invocable: false
disable-model-invocation: false"""

        self.assertEqual(
            validate_copilot_agent_header(
                header,
                "Rust Quality Engineering",
                "rust-quality-engineering",
                False,
            ),
            [],
        )

    def test_invalid_yaml_list_is_reported(self):
        header = self.coordinator_header().replace(
            "['agent', 'read', 'search', 'edit', 'execute']",
            "['agent', 'read'",
        )

        fields, errors = parse_agent_frontmatter(header)

        self.assertIn("invalid value for frontmatter key tools", errors)
        self.assertNotIn("tools", fields)

    def test_skill_review_date_accepts_valid_iso_dates(self):
        self.assertTrue(has_review_date("Revisada el 2026-11-11. Revisa las fuentes citadas."))

    def test_skill_review_date_rejects_invalid_or_missing_dates(self):
        for content in (
            "Revisada el 2026-02-30.",
            "Revisada el 2026/11/11.",
            "Revisada el 2026-11-11",
            "No review date",
        ):
            with self.subTest(content=content):
                self.assertFalse(has_review_date(content))

    @staticmethod
    def coordinator_header():
        return """name: Enterprise Architecture Governance
description: "Coordinate repository work."
target: github-copilot
tools: ['agent', 'read', 'search', 'edit', 'execute']
skills: ['enterprise-architecture-governance']
user-invocable: true
disable-model-invocation: false"""


if __name__ == "__main__":
    unittest.main()
