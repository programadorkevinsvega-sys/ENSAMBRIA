#!/usr/bin/env python3
"""Reconcile SQL migration DDL, including policy DDL expanded from PL/pgSQL loops.

The inventory is deliberately strict: every discovered object must be declared,
and every declared object must be created by a migration. Dynamic CREATE POLICY
statements emitted through EXECUTE format() are expanded from their FOREACH
ARRAY literals instead of being silently exempted.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path
from typing import Iterable

ROOT = Path(__file__).resolve().parents[2]
INVENTORY = ROOT / "architecture/data-ownership.yaml"
SECTIONS = ("tables", "views", "sequences", "functions", "triggers", "policies")
PATTERNS = {
    "tables": re.compile(
        r"\bCREATE\s+TABLE\s+(?:IF\s+NOT\s+EXISTS\s+)?([a-z_][\w$]*\.[a-z_][\w$]*)",
        re.IGNORECASE,
    ),
    "views": re.compile(
        r"\bCREATE\s+(?:OR\s+REPLACE\s+)?(?:MATERIALIZED\s+)?VIEW\s+([a-z_][\w$]*\.[a-z_][\w$]*)",
        re.IGNORECASE,
    ),
    "sequences": re.compile(
        r"\bCREATE\s+SEQUENCE\s+(?:IF\s+NOT\s+EXISTS\s+)?([a-z_][\w$]*\.[a-z_][\w$]*)",
        re.IGNORECASE,
    ),
    "functions": re.compile(
        r"\bCREATE\s+(?:OR\s+REPLACE\s+)?FUNCTION\s+([a-z_][\w$]*\.[a-z_][\w$]*)",
        re.IGNORECASE,
    ),
    "triggers": re.compile(
        r"\bCREATE\s+TRIGGER\s+([a-z_][\w$]*)[\s\S]*?\bON\s+([a-z_][\w$]*)\.([a-z_][\w$]*)",
        re.IGNORECASE,
    ),
    "policies": re.compile(
        r"\bCREATE\s+POLICY\s+([a-z_][\w$]*)\s+ON\s+([a-z_][\w$]*)\.([a-z_][\w$]*)",
        re.IGNORECASE,
    ),
}
FOREACH_RE = re.compile(
    r"\bFOREACH\s+([a-z_][\w$]*)\s+IN\s+ARRAY\s+ARRAY\s*\[(.*?)\]\s*LOOP(.*?)END\s+LOOP\s*;",
    re.IGNORECASE | re.DOTALL,
)
EXECUTE_FORMAT_RE = re.compile(
    r"\bEXECUTE\s+format\s*\(\s*'((?:''|[^'])*)'\s*,\s*(.*?)\)\s*;",
    re.IGNORECASE | re.DOTALL,
)
DYNAMIC_POLICY_RE = re.compile(
    r"\bCREATE\s+POLICY\s+(%I|[a-z_][\w$]*)\s+ON\s+"
    r"([a-z_][\w$]*)\.(%I|[a-z_][\w$]*)",
    re.IGNORECASE,
)


def strip_comments(sql: str) -> str:
    """Remove ordinary SQL comments without touching quoted dynamic SQL."""
    sql = re.sub(r"/\*.*?\*/", "", sql, flags=re.DOTALL)
    return re.sub(r"(?m)--[^\n]*$", "", sql)


def parse_inventory(text: str) -> dict[str, dict[str, set[str]]]:
    modules: dict[str, dict[str, set[str]]] = {}
    blocks = re.split(r"(?m)^  - id: ", text)[1:]
    if not blocks:
        raise ValueError("data-ownership.yaml contains no modules")
    for block in blocks:
        module_id = block.splitlines()[0].strip()
        schema_match = re.search(r"(?m)^    schema: ([a-z_][a-z0-9_]*)$", block)
        if not schema_match:
            raise ValueError(f"{module_id}: missing schema")
        schema = schema_match.group(1)
        if schema in modules:
            raise ValueError(f"duplicate schema ownership declaration: {schema}")
        modules[schema] = {section: set() for section in SECTIONS}
        active: str | None = None
        for line in block.splitlines():
            section_match = re.match(
                r"^    (tables|views|sequences|functions|triggers|policies):\s*$",
                line,
            )
            if section_match:
                active = section_match.group(1)
                continue
            if re.match(r"^    [a-z_][a-z0-9_]*:", line):
                active = None
            item_match = re.match(r"^      - ([a-z_][a-z0-9_.]*)$", line)
            if active and item_match:
                item = item_match.group(1)
                if item in modules[schema][active]:
                    raise ValueError(f"{schema}.{active}: duplicate inventory item {item}")
                modules[schema][active].add(item)
    return modules


def _split_sql_arguments(arguments: str) -> list[str]:
    parts: list[str] = []
    start = 0
    depth = 0
    in_quote = False
    index = 0
    while index < len(arguments):
        char = arguments[index]
        if char == "'":
            if in_quote and index + 1 < len(arguments) and arguments[index + 1] == "'":
                index += 2
                continue
            in_quote = not in_quote
        elif not in_quote:
            if char == "(":
                depth += 1
            elif char == ")":
                depth -= 1
            elif char == "," and depth == 0:
                parts.append(arguments[start:index].strip())
                start = index + 1
        index += 1
    tail = arguments[start:].strip()
    if tail:
        parts.append(tail)
    return parts


def _foreach_values(literal_array: str) -> list[str]:
    return [
        value.replace("''", "'")
        for value in re.findall(r"'((?:''|[^'])*)'", literal_array)
    ]


def _dynamic_policies(sql: str) -> set[tuple[str, str]]:
    found: set[tuple[str, str]] = set()
    for loop in FOREACH_RE.finditer(sql):
        variable, array_literal, body = loop.groups()
        values = _foreach_values(array_literal)
        for execution in EXECUTE_FORMAT_RE.finditer(body):
            raw_template, raw_arguments = execution.groups()
            template = raw_template.replace("''", "'")
            ddl = DYNAMIC_POLICY_RE.search(template)
            if not ddl:
                continue
            policy_token, schema, table_token = ddl.groups()
            arguments = _split_sql_arguments(raw_arguments)
            table_argument_index = 1 if policy_token == "%I" else 0
            if table_token != "%I" or len(arguments) <= table_argument_index:
                continue
            table_expression = arguments[table_argument_index]
            if re.fullmatch(rf"{re.escape(variable)}", table_expression, re.IGNORECASE) is None:
                continue

            policy_suffix = ""
            if policy_token == "%I":
                if not arguments:
                    continue
                policy_expression = arguments[0]
                suffix_match = re.fullmatch(
                    rf"{re.escape(variable)}\s*\|\|\s*'((?:''|[^'])*)'",
                    policy_expression,
                    re.IGNORECASE,
                )
                if suffix_match:
                    policy_suffix = suffix_match.group(1).replace("''", "'")
                elif re.fullmatch(rf"{re.escape(variable)}", policy_expression, re.IGNORECASE):
                    policy_suffix = ""
                else:
                    continue
            else:
                policy_suffix = policy_token

            for value in values:
                table_name = value
                policy_name = f"{value}{policy_suffix}" if policy_token == "%I" else policy_suffix
                found.add((schema.lower(), f"{table_name}.{policy_name}"))
    return found


def discover_sql_objects(sql: str) -> dict[str, dict[str, set[str]]]:
    cleaned = strip_comments(sql)
    found: dict[str, dict[str, set[str]]] = {}

    def add(schema: str, section: str, name: str) -> None:
        found.setdefault(schema.lower(), {item: set() for item in SECTIONS})
        found[schema.lower()][section].add(name)

    for section, pattern in PATTERNS.items():
        for match in pattern.findall(cleaned):
            if section in ("triggers", "policies"):
                object_name, schema, table_name = match
                name = object_name if section == "triggers" else f"{table_name}.{object_name}"
                add(schema, section, name)
            else:
                schema, object_name = match.split(".", 1)
                add(schema, section, object_name)

    for schema, policy in _dynamic_policies(cleaned):
        add(schema, "policies", policy)
    return found


def validate_inventory(
    inventory_text: str,
    sources: Iterable[tuple[str, str]],
) -> list[str]:
    inventory = parse_inventory(inventory_text)
    discovered = {schema: {section: set() for section in SECTIONS} for schema in inventory}
    errors: list[str] = []

    for source_name, sql in sources:
        for schema, sections in discover_sql_objects(sql).items():
            if schema not in discovered:
                names = sorted(name for values in sections.values() for name in values)
                errors.append(
                    f"{source_name}: objects {', '.join(names)} belong to schema "
                    f"{schema}, absent from data-ownership.yaml"
                )
                continue
            for section in SECTIONS:
                discovered[schema][section].update(sections[section])

    for schema, sections in discovered.items():
        for section in SECTIONS:
            missing = sorted(sections[section] - inventory[schema][section])
            stale = sorted(inventory[schema][section] - sections[section])
            if missing:
                errors.append(
                    f"{schema}.{section}: SQL objects missing from inventory: {', '.join(missing)}"
                )
            if stale:
                errors.append(
                    f"{schema}.{section}: inventory entries not found in migrations: {', '.join(stale)}"
                )
    return errors


def main() -> int:
    try:
        inventory_text = INVENTORY.read_text(encoding="utf-8")
        migration_files = sorted((ROOT / "backend").glob("*/migrations/**/*.sql"))
        migration_files += sorted((ROOT / "backend").glob("*/migrations/*.sql"))
        migration_files = sorted(set(migration_files))
        if not migration_files:
            raise ValueError("no backend SQL migration files were found")
        sources = [
            (str(path.relative_to(ROOT)), path.read_text(encoding="utf-8"))
            for path in migration_files
        ]
        errors = validate_inventory(inventory_text, sources)
    except (OSError, ValueError) as error:
        print(f"sql-ownership-inventory: {error}", file=sys.stderr)
        return 1

    if errors:
        for error in errors:
            print(f"sql-ownership-inventory: {error}", file=sys.stderr)
        return 1
    print(
        "SQL ownership inventory exactly matches schema-qualified tables, views, "
        "sequences, functions, triggers, and static/dynamically generated policies."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
