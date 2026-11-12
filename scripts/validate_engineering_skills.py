#!/usr/bin/env python3
"""Validate static Agent Skills, Copilot profiles, and repository-governance structure."""

from __future__ import annotations

import ast
import re
import sys
from datetime import date
from pathlib import Path
from urllib.parse import unquote, urlsplit


ROOT = Path(__file__).resolve().parents[1]
SKILLS_ROOT = ROOT / ".github" / "skills"
AGENTS_FILE = ROOT / "AGENTS.md"
AGENT_PROFILES_ROOT = ROOT / ".github" / "agents"
GITHUB_RULE_FILE = ROOT / "docs" / "rules" / "ENSAMBRIA_github_repository_governance_and_engineering_rules.md"
GITHUB_WORKFLOW_FILE = ROOT / ".github" / "workflows" / "repository-quality.yml"
CODEOWNERS_FILE = ROOT / ".github" / "CODEOWNERS"
PULL_REQUEST_TEMPLATE_FILE = ROOT / ".github" / "pull_request_template.md"
REVIEW_DATE_PATTERN = re.compile(r"^Revisada el (\d{4}-\d{2}-\d{2})\.", re.MULTILINE)
REQUIRED_SECTIONS = (
    "## Objetivo",
    "## Alcance",
    "## Cuándo activarla",
    "## Referencias normativas obligatorias",
    "## Aplicación obligatoria en ENSAMBRIA",
    "## Procedimiento operativo",
    "## Restricciones",
    "## Validaciones obligatorias",
    "## Criterios de finalización",
    "## Conflictos y excepciones",
    "## Mantenimiento",
)
SKILL_SOURCES = {
    "enterprise-architecture-governance": (
        "docs/rules/ENSAMBRIA_architecture_doc_engineering_governance_standard.md",
        "docs/rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md",
        "ARCHITECTURE_RULES.md",
        "docs/requerimiento.md",
    ),
    "rust-enterprise-architecture": (
        "ARCHITECTURE_RULES.md",
        "docs/rules/ENSAMBRIA_reglas-estrictas-de-arquitectura-monolito-modular.md",
        "docs/rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md",
        "docs/requerimiento.md",
    ),
    "rust-quality-engineering": (
        "docs/rules/ENSAMBRIA_prompt_maestro_enterprise_quality_engineering.md",
        "docs/rules/ENSAMBRIA_k6_performance_load_stress_and_capacity_testing_rules.md",
        "docs/rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md",
        "ARCHITECTURE_RULES.md",
        "docs/rules/ENSAMBRIA_reglas-estrictas-de-arquitectura-monolito-modular.md",
        "docs/requerimiento.md",
    ),
    "k6-performance-engineering": (
        "docs/rules/ENSAMBRIA_k6_performance_load_stress_and_capacity_testing_rules.md",
        "docs/rules/ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md",
        "ARCHITECTURE_RULES.md",
        "docs/requerimiento.md",
    ),
    "distributed-messaging": (
        "docs/rules/ENSAMBRIA_Transactional_Outbox_Inbox.md",
        "ARCHITECTURE_RULES.md",
        "docs/requerimiento.md",
    ),
    "postgresql-engineering": (
        "docs/rules/ENSAMBRIA_Connection_Pooling_PostgreSQL.md",
        "ARCHITECTURE_RULES.md",
        "docs/requerimiento.md",
    ),
    "security-and-privacy": (
        "docs/rules/ENSAMBRIA_Geographic_Access_Policies.md",
        "docs/rules/ENSAMBRIA_Privacy_Legal_Compliance.md",
        "docs/rules/ENSAMBRIA_Privacy_Data_Lifecycle_Management.md",
        "ARCHITECTURE_RULES.md",
        "docs/requerimiento.md",
    ),
    "observability-engineering": (
        "docs/rules/ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md",
        "ARCHITECTURE_RULES.md",
        "docs/rules/ENSAMBRIA_Privacy_Legal_Compliance.md",
        "docs/rules/ENSAMBRIA_Privacy_Data_Lifecycle_Management.md",
    ),
    "web-quality-seo": (
        "docs/rules/ENSAMBRIA_frontend_security_accessibility_ux_ui_performance_observability_rules.md",
        "docs/rules/ENSAMBRIA_k6_performance_load_stress_and_capacity_testing_rules.md",
        "docs/rules/ENSAMBRIA_Website_Launch_SEO_Performance_Accessibility.md",
        "docs/rules/ENSAMBRIA_Open_Graph_SEO_Metadata.md",
        "ARCHITECTURE_RULES.md",
    ),
    "documentation-engineering": (
        "docs/rules/ENSAMBRIA_README_Documentation_Quality_Security_Standard.md",
        "docs/rules/ENSAMBRIA_architecture_doc_engineering_governance_standard.md",
        "docs/rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md",
        "ARCHITECTURE_RULES.md",
        "docs/requerimiento.md",
    ),
    "github-repository-governance": (
        "docs/rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md",
        "ARCHITECTURE_RULES.md",
        "docs/requerimiento.md",
    ),
}
AGENT_PROFILES = {
    "ENSAMBRIA-coordinator": "Enterprise Architecture Governance",
    "enterprise-architecture-governance": "Architecture Governance Specialist",
    "rust-enterprise-architecture": "Rust Enterprise Architecture",
    "rust-quality-engineering": "Rust Quality Engineering",
    "k6-performance-engineering": "k6 Performance Engineering",
    "distributed-messaging": "Distributed Messaging",
    "postgresql-engineering": "PostgreSQL Engineering",
    "security-and-privacy": "Security and Privacy",
    "observability-engineering": "Observability Engineering",
    "web-quality-seo": "Web Quality and SEO",
    "documentation-engineering": "Documentation Engineering",
    "github-repository-governance": "GitHub Repository Governance",
}
COPILOT_AGENT_FIELDS = {
    "name",
    "description",
    "target",
    "tools",
    "skills",
    "user-invocable",
    "disable-model-invocation",
}
COPILOT_TOOLS = {"agent", "edit", "execute", "read", "search", "todo", "web"}
AGENT_SKILLS = {
    "ENSAMBRIA-coordinator": "enterprise-architecture-governance",
    **{slug: slug for slug in SKILL_SOURCES},
}


def fail(message: str, failures: list[str]) -> None:
    failures.append(message)


def has_review_date(content: str) -> bool:
    match = REVIEW_DATE_PATTERN.search(content)
    if not match:
        return False
    try:
        date.fromisoformat(match.group(1))
    except ValueError:
        return False
    return True


def parse_agent_frontmatter(header: str) -> tuple[dict[str, object], list[str]]:
    fields: dict[str, object] = {}
    errors = []
    for line_number, line in enumerate(header.splitlines(), start=1):
        if not line.strip():
            continue
        match = re.fullmatch(r"([A-Za-z][A-Za-z0-9-]*):\s*(.*?)\s*", line)
        if not match:
            errors.append(f"invalid frontmatter entry on line {line_number}")
            continue
        key, raw_value = match.groups()
        if key in fields:
            errors.append(f"duplicate frontmatter key {key}")
            continue
        if raw_value in {"true", "false"}:
            value: object = raw_value == "true"
        elif raw_value.startswith(("[", "'", '"')):
            try:
                value = ast.literal_eval(raw_value)
            except (SyntaxError, ValueError):
                errors.append(f"invalid value for frontmatter key {key}")
                continue
        else:
            value = raw_value
        fields[key] = value
    return fields, errors


def validate_copilot_agent_header(
    header: str,
    expected_name: str,
    expected_skill: str,
    is_coordinator: bool,
) -> list[str]:
    fields, errors = parse_agent_frontmatter(header)
    unsupported_fields = set(fields) - COPILOT_AGENT_FIELDS
    if unsupported_fields:
        errors.append(f"unsupported Copilot frontmatter fields {sorted(unsupported_fields)}")

    required_fields = COPILOT_AGENT_FIELDS
    missing_fields = required_fields - set(fields)
    if missing_fields:
        errors.append(f"missing Copilot frontmatter fields {sorted(missing_fields)}")

    if fields.get("name") != expected_name:
        errors.append(f"name must be {expected_name}")
    if not isinstance(fields.get("description"), str) or not fields["description"].strip():
        errors.append("description must be a non-empty string")
    if fields.get("target") != "github-copilot":
        errors.append("target must be github-copilot")

    tools = fields.get("tools")
    if not isinstance(tools, list) or not tools or not all(
        isinstance(tool, str) for tool in tools
    ):
        errors.append("tools must be a non-empty list of strings")
    else:
        invalid_tools = set(tools) - COPILOT_TOOLS
        if invalid_tools:
            errors.append(f"unsupported Copilot tool aliases {sorted(invalid_tools)}")
        if is_coordinator and "agent" not in tools:
            errors.append("coordinator must declare Copilot's agent capability alias")
        if (
            not is_coordinator
            and expected_skill in {"rust-quality-engineering", "security-and-privacy"}
            and "edit" in tools
        ):
            errors.append("independent review profiles must not include edit")

    skills = fields.get("skills")
    if skills != [expected_skill]:
        errors.append(f"skills must reference exactly {expected_skill}")

    expected_user_invocable = is_coordinator
    if fields.get("user-invocable") is not expected_user_invocable:
        errors.append(f"user-invocable must be {str(expected_user_invocable).lower()}")
    if fields.get("disable-model-invocation") is not False:
        errors.append("disable-model-invocation must be false so Copilot can invoke the profile")
    return errors


def markdown_targets(path: Path) -> list[Path]:
    content = path.read_text(encoding="utf-8")
    targets = []
    for raw_target in re.findall(r"\[[^\]]+\]\(([^)]+)\)", content):
        parsed = urlsplit(raw_target.strip())
        if parsed.scheme or parsed.netloc:
            continue
        target = unquote(parsed.path)
        if target:
            targets.append((path.parent / target).resolve())
    return targets


def validate_skill(slug: str, failures: list[str]) -> None:
    skill_file = SKILLS_ROOT / slug / "SKILL.md"
    if not skill_file.is_file():
        fail(f"Missing skill file: {skill_file.relative_to(ROOT)}", failures)
        return

    content = skill_file.read_text(encoding="utf-8")
    frontmatter = re.match(r"\A---\n(.*?)\n---\n", content, re.DOTALL)
    if not frontmatter:
        fail(f"{skill_file.relative_to(ROOT)}: invalid or missing YAML frontmatter", failures)
        return

    header = frontmatter.group(1)
    if len(header.splitlines()) != 2:
        fail(
            f"{skill_file.relative_to(ROOT)}: frontmatter must contain only name and description",
            failures,
        )
    name = re.search(r"^name:\s*([a-z0-9-]+)\s*$", header, re.MULTILINE)
    description = re.search(r'^description:\s*"([^"\n]+)"\s*$', header, re.MULTILINE)
    if not name or name.group(1) != slug:
        fail(f"{skill_file.relative_to(ROOT)}: frontmatter name must be {slug}", failures)
    if not description:
        fail(f"{skill_file.relative_to(ROOT)}: frontmatter description is missing or invalid", failures)

    for heading in REQUIRED_SECTIONS:
        if heading not in content:
            fail(f"{skill_file.relative_to(ROOT)}: missing section {heading}", failures)
    if not has_review_date(content):
        fail(f"{skill_file.relative_to(ROOT)}: missing review date", failures)

    application_heading = "## Aplicación obligatoria en ENSAMBRIA"
    if application_heading in content:
        application = content.split(application_heading, 1)[1].split("\n## ", 1)[0]
        required_application = (
            "fase",
            "ADR",
            "REQUERIDO",
            "IMPLEMENTADO",
            "VALIDADO",
            "APROBADO",
            "BLOQUEADO",
            "evidencia",
        )
        for phrase in required_application:
            if phrase not in application:
                fail(
                    f"{skill_file.relative_to(ROOT)}: ENSAMBRIA application procedure "
                    f"is missing {phrase}",
                    failures,
                )

    available_targets = {target.relative_to(ROOT).as_posix() for target in markdown_targets(skill_file)}
    for source in ("AGENTS.md", "ARCHITECTURE_RULES.md"):
        if source not in available_targets:
            fail(
                f"{skill_file.relative_to(ROOT)}: application procedure must link {source}",
                failures,
            )
    for source in SKILL_SOURCES[slug]:
        if source not in available_targets:
            fail(f"{skill_file.relative_to(ROOT)}: missing normative link to {source}", failures)

    if slug == "github-repository-governance":
        required_governance = (
            "ENSAMBRIA_github_repository_governance_and_engineering_rules.md",
            "§8.1",
            "auto-revisión documentada",
            "no la presentes como aprobación independiente",
            "No agregues gates placebo",
            "validaciones locales",
            "remotos",
        )
        for phrase in required_governance:
            if phrase not in content:
                fail(
                    f"{skill_file.relative_to(ROOT)}: GitHub governance procedure "
                    f"is missing {phrase}",
                    failures,
                )


def validate_agents(failures: list[str]) -> None:
    if not AGENTS_FILE.is_file():
        fail("Missing root AGENTS.md", failures)
        return
    content = AGENTS_FILE.read_text(encoding="utf-8")
    governance_heading = "## Gobernanza de instrucciones para agentes"
    if governance_heading not in content:
        fail("AGENTS.md is missing agent governance and precedence", failures)
    else:
        governance = content.split(governance_heading, 1)[1]
        required_governance = (
            "### Precedencia",
            "1. Instrucciones explícitas del propietario",
            "2. ADR ratificados",
            "3. [ARCHITECTURE_RULES.md]",
            "4. Reglas técnicas obligatorias",
            "5. Requisitos y criterios de aceptación",
            "6. Instrucciones específicas de la tarea",
            "7. Procedimientos operativos de las Skills",
            "CONSULTAR → INTERPRETAR → PLANIFICAR → IMPLEMENTAR → PROBAR → VALIDAR → DOCUMENTAR → REPORTAR",
            "REQUERIDO, DISEÑADO, IMPLEMENTADO, VALIDADO, APROBADO y BLOQUEADO",
            "rechaza o devuelve entregables",
        )
        for phrase in required_governance:
            if phrase not in governance:
                fail(f"AGENTS.md agent governance is missing: {phrase}", failures)
        precedence_positions = [governance.find(phrase) for phrase in required_governance[1:8]]
        if any(position < 0 for position in precedence_positions) or precedence_positions != sorted(
            precedence_positions
        ):
            fail("AGENTS.md instruction precedence is missing or out of order", failures)

    for phrase in (
        "sin reemplazarla por una política genérica",
        "§8.1",
        "no equivale a aprobación independiente",
    ):
        if phrase not in content:
            fail(f"AGENTS.md GitHub governance is missing: {phrase}", failures)

    heading = "## Engineering Skills — Uso obligatorio"
    if heading not in content:
        fail("AGENTS.md is missing the Engineering Skills section", failures)
        return
    section = content.split(heading, 1)[1]
    required_phrases = (
        ".github/skills/<nombre>/SKILL.md",
        "no como estándar universal",
        "python3 scripts/validate_engineering_skills.py",
        "varias Skills",
        "`target: github-copilot`",
        "`task`",
        "`skills`",
        "python3 -m unittest discover -s scripts -p 'test_*.py'",
        "Respeta cualquier `AGENTS.md` más específico aplicable.",
        "nunca importes automáticamente como universales",
    )
    for phrase in required_phrases:
        if phrase not in section:
            fail(f"AGENTS.md Skills section is missing: {phrase}", failures)
    for slug in SKILL_SOURCES:
        if f"`{slug}`" not in section:
            fail(f"AGENTS.md Skills registry is missing {slug}", failures)

    targets = markdown_targets(AGENTS_FILE)
    for slug in AGENT_PROFILES:
        profile_name = (
            "coordinator.agent.md" if slug == "ENSAMBRIA-coordinator" else f"{slug}.agent.md"
        )
        profile_path = (AGENT_PROFILES_ROOT / profile_name).resolve()
        if profile_path not in targets:
            fail(f"AGENTS.md does not register custom agent {slug}", failures)
    for target in targets:
        if not target.exists():
            fail(f"AGENTS.md has a broken local link: {target}", failures)


def validate_custom_agent_profiles(failures: list[str]) -> None:
    expected_files = {
        "coordinator.agent.md" if slug == "ENSAMBRIA-coordinator" else f"{slug}.agent.md"
        for slug in AGENT_PROFILES
    }
    actual_files = {path.name for path in AGENT_PROFILES_ROOT.glob("*.agent.md")}
    if actual_files != expected_files:
        fail(
            f"Custom agent profile mismatch; missing={sorted(expected_files - actual_files)}, "
            f"unexpected={sorted(actual_files - expected_files)}",
            failures,
        )

    coordinator_path = AGENT_PROFILES_ROOT / "coordinator.agent.md"
    coordinator_content = ""
    coordinator_targets: set[Path] = set()
    for slug, expected_name in AGENT_PROFILES.items():
        profile_name = (
            "coordinator.agent.md" if slug == "ENSAMBRIA-coordinator" else f"{slug}.agent.md"
        )
        profile = AGENT_PROFILES_ROOT / profile_name
        if not profile.is_file():
            continue
        content = profile.read_text(encoding="utf-8")
        if slug == "ENSAMBRIA-coordinator":
            coordinator_content = content
        frontmatter = re.match(r"\A---\n(.*?)\n---\n", content, re.DOTALL)
        if not frontmatter:
            fail(f"{profile.relative_to(ROOT)}: invalid or missing YAML frontmatter", failures)
            continue

        header = frontmatter.group(1)
        is_coordinator = slug == "ENSAMBRIA-coordinator"
        for error in validate_copilot_agent_header(
            header,
            expected_name,
            AGENT_SKILLS[slug],
            is_coordinator,
        ):
            fail(
                f"{profile.relative_to(ROOT)}: {error}",
                failures,
            )

        if slug == "ENSAMBRIA-coordinator":
            architecture_gate = "## Normative order and architecture gate"
            if architecture_gate not in content:
                fail(f"{profile.relative_to(ROOT)}: missing normative architecture gate", failures)
            else:
                gate = content.split(architecture_gate, 1)[1].split("\n## ", 1)[0]
                for phrase in (
                    "original GitHub governance rule is authoritative",
                    "§8.1",
                    "not independent approval",
                    "Do not replace it with a generic policy",
                ):
                    if phrase not in gate:
                        fail(
                            f"{profile.relative_to(ROOT)}: normative gate is missing {phrase}",
                            failures,
                        )
            delegation_heading = "### Mandatory delegation context"
            if delegation_heading not in content:
                fail(f"{profile.relative_to(ROOT)}: missing mandatory delegation context", failures)
            else:
                delegation = content.split(delegation_heading, 1)[1]
                required_context = (
                    "Objective and bounded deliverable:",
                    "Phase and authorized scope",
                    "Applicable rules and exact file/section references:",
                    "Related ADRs/contracts and their status",
                    "Domain/data owner and allowed boundaries:",
                    "Security, privacy, repository, and operational constraints:",
                    "Dependencies and known blockers:",
                    "Required scenarios, tests, and CI/local checks:",
                    "Acceptance criteria:",
                    "Required evidence and report format:",
                    "Files in scope and files explicitly out of scope:",
                    "does not infer approval",
                    "all relevant `docs/rules/` sources",
                )
                for phrase in required_context:
                    if phrase not in delegation:
                        fail(
                            f"{profile.relative_to(ROOT)}: delegation contract is missing {phrase}",
                            failures,
                        )
                for phrase in (
                    "Use Copilot's native `task` capability to delegate",
                    "do not use Local's `agents` allowlist",
                    "static validator cannot prove runtime discovery",
                ):
                    if phrase not in content:
                        fail(
                            f"{profile.relative_to(ROOT)}: Copilot delegation guidance "
                            f"is missing {phrase}",
                            failures,
                        )
        else:
            architecture_gate = "## ENSAMBRIA Architecture Gate"
            if architecture_gate not in content:
                fail(f"{profile.relative_to(ROOT)}: missing ENSAMBRIA architecture gate", failures)
            else:
                gate = content.split(architecture_gate, 1)[1]
                required_gate = (
                    "AGENTS.md",
                    "ARCHITECTURE_RULES.md",
                    "associated Skill",
                    "proposed ADR",
                    "ask the coordinator",
                    "ownership",
                    "tests/checks and results",
                    "BLOQUEADO",
                )
                for phrase in required_gate:
                    if phrase not in gate:
                        fail(
                            f"{profile.relative_to(ROOT)}: architecture gate is missing {phrase}",
                            failures,
                        )
                if slug == "github-repository-governance":
                    for phrase in (
                        "original GitHub governance rule",
                        "§8.1 single-maintainer self-review",
                        "never describe it as independent approval",
                    ):
                        if phrase not in content:
                            fail(
                                f"{profile.relative_to(ROOT)}: GitHub governance profile "
                                f"is missing {phrase}",
                                failures,
                            )

        targets = markdown_targets(profile)
        if slug == "ENSAMBRIA-coordinator":
            coordinator_targets.update(targets)
        for target in targets:
            if not target.exists():
                fail(f"{profile.relative_to(ROOT)} has a broken local link: {target}", failures)
        if slug == "ENSAMBRIA-coordinator":
            for source in sorted({source for sources in SKILL_SOURCES.values() for source in sources}):
                if source.startswith("docs/rules/"):
                    expected_target = (ROOT / source).resolve()
                    if expected_target not in targets:
                        fail(
                            f"{profile.relative_to(ROOT)}: coordinator does not map "
                            f"normative rule {source}",
                            failures,
                        )
        else:
            skill_file = ROOT / ".github" / "skills" / slug / "SKILL.md"
            expected_targets = {(ROOT / source).resolve() for source in SKILL_SOURCES[slug]}
            if skill_file.resolve() not in targets:
                fail(
                    f"{profile.relative_to(ROOT)}: missing associated Skill "
                    f".github/skills/{slug}/SKILL.md",
                    failures,
                )
            for expected_target in expected_targets:
                if expected_target not in targets:
                    fail(
                        f"{profile.relative_to(ROOT)}: missing normative reference "
                        f"{expected_target.relative_to(ROOT)}",
                        failures,
                    )
    for rule in sorted((ROOT / "docs" / "rules").glob("*.md")):
        if rule.resolve() not in coordinator_targets:
            fail(
                f"{coordinator_path.relative_to(ROOT)}: rule lacks explicit agent mapping: "
                f"{rule.relative_to(ROOT)}",
                failures,
            )
    for slug in SKILL_SOURCES:
        skill_path = (SKILLS_ROOT / slug / "SKILL.md").resolve()
        if skill_path not in coordinator_targets:
            fail(
                f"{coordinator_path.relative_to(ROOT)}: missing Skill-to-agent mapping "
                f"for {slug}",
                failures,
            )

    if not coordinator_content:
        fail("Missing ENSAMBRIA coordinator profile", failures)


def validate_repository_governance(failures: list[str]) -> None:
    if not GITHUB_RULE_FILE.is_file():
        fail("Missing original GitHub governance rule", failures)
        return

    github_rule = GITHUB_RULE_FILE.read_text(encoding="utf-8")
    required_rule_text = (
        "# 7. Política de Pull Requests",
        "## 8.1 Repositorio con una sola persona mantenedora",
        "no equivale a aprobación independiente",
        "# 9. GitHub Actions e integración continua",
        "# 12. GitHub Actions: permisos y cadena de suministro",
        "# 14. CODEOWNERS y responsabilidad técnica",
        "# 19. Definition of Done para Pull Requests",
    )
    for phrase in required_rule_text:
        if phrase not in github_rule:
            fail(f"Original GitHub governance rule is missing: {phrase}", failures)

    if not GITHUB_WORKFLOW_FILE.is_file():
        fail("Missing repository-quality GitHub Actions workflow", failures)
    else:
        workflow = GITHUB_WORKFLOW_FILE.read_text(encoding="utf-8")
        required_workflow_text = (
            "pull_request:",
            "push:",
            "branches:\n      - main",
            "permissions:\n  contents: read",
            "cancel-in-progress: true",
            "timeout-minutes:",
            "python3 -m unittest discover -s scripts -p 'test_*.py'",
            "python3 scripts/validate_engineering_skills.py",
            "repository-quality:",
        )
        for phrase in required_workflow_text:
            if phrase not in workflow:
                fail(f"Repository-quality workflow is missing: {phrase}", failures)
        action_references = re.findall(r"^\s+uses:\s+([^\s]+)\s*$", workflow, re.MULTILINE)
        for action in action_references:
            if not re.fullmatch(r"[^@]+@[0-9a-f]{40}", action):
                fail(
                    f"GitHub Action must be pinned to a full commit SHA: {action}",
                    failures,
                )

    for path, label in (
        (CODEOWNERS_FILE, "CODEOWNERS"),
        (PULL_REQUEST_TEMPLATE_FILE, "pull request template"),
    ):
        if not path.is_file():
            fail(f"Missing {label}: {path.relative_to(ROOT)}", failures)

    if CODEOWNERS_FILE.is_file():
        codeowners = CODEOWNERS_FILE.read_text(encoding="utf-8")
        if not re.search(r"^\*?\s+@[A-Za-z0-9-]+(?:/[A-Za-z0-9-]+)?\s*$", codeowners, re.MULTILINE):
            fail("CODEOWNERS must identify the current repository owner", failures)

    if PULL_REQUEST_TEMPLATE_FILE.is_file():
        template = PULL_REQUEST_TEMPLATE_FILE.read_text(encoding="utf-8")
        for heading in ("## Alcance", "## Criterios de aceptación", "## Riesgos", "## Validación"):
            if heading not in template:
                fail(f"Pull request template is missing required heading: {heading}", failures)


def main() -> int:
    failures: list[str] = []
    expected = set(SKILL_SOURCES)
    actual = {path.parent.name for path in SKILLS_ROOT.glob("*/SKILL.md")}
    if actual != expected:
        fail(
            f"Skill registry mismatch; missing={sorted(expected - actual)}, "
            f"unexpected={sorted(actual - expected)}",
            failures,
        )

    all_targets: set[Path] = set()
    for slug in sorted(expected):
        validate_skill(slug, failures)
        skill_file = SKILLS_ROOT / slug / "SKILL.md"
        if skill_file.is_file():
            all_targets.update(markdown_targets(skill_file))
    for target in sorted(all_targets):
        if not target.exists():
            fail(f"Broken local Skill link: {target}", failures)

    validate_agents(failures)
    validate_custom_agent_profiles(failures)
    validate_repository_governance(failures)
    if failures:
        print("\n".join(f"ERROR: {failure}" for failure in failures), file=sys.stderr)
        return 1
    print(
        f"Copilot configuration checks passed: {len(expected)} Engineering Skills, "
        f"{len(AGENT_PROFILES)} schema-checked agent profiles, and the AGENTS.md registry."
    )
    print(
        "Scope limitation: static checks do not prove runtime discovery or invocation of agents, "
        "Skill loading, or successful delegation by the active harness; they also do not verify "
        "architectural or ADR semantic consistency, Phase 0 acceptance, or legal approval."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
