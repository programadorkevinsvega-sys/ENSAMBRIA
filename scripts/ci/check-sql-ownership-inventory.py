#!/usr/bin/env python3
"""Compare schema-qualified SQL DDL with the canonical module ownership inventory.

This intentionally uses only Python's standard library: the inventory is a small,
strict YAML subset whose module sections are simple scalar lists.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
INVENTORY = ROOT / "architecture/data-ownership.yaml"
SECTIONS = ("tables", "views", "functions")
PATTERNS = {
    "tables": re.compile(
        r"\bCREATE\s+TABLE\s+(?:IF\s+NOT\s+EXISTS\s+)?([a-z_][\w$]*\.[a-z_][\w$]*)",
        re.IGNORECASE,
    ),
    "views": re.compile(
        r"\bCREATE\s+(?:OR\s+REPLACE\s+)?(?:MATERIALIZED\s+)?VIEW\s+([a-z_][\w$]*\.[a-z_][\w$]*)",
        re.IGNORECASE,
    ),
    "functions": re.compile(
        r"\bCREATE\s+(?:OR\s+REPLACE\s+)?FUNCTION\s+([a-z_][\w$]*\.[a-z_][\w$]*)",
        re.IGNORECASE,
    ),
}


def parse_inventory(text: str) -> dict[str, dict[str, set[str]]]:
    modules: dict[str, dict[str, set[str]]] = {}
    blocks = re.split(r"(?m)^  - id: ", text)[1:]
    for block in blocks:
        module_id = block.splitlines()[0].strip()
        schema_match = re.search(r"(?m)^    schema: ([a-z_][a-z0-9_]*)$", block)
        if not schema_match:
            raise ValueError(f"{module_id}: missing schema")
        schema = schema_match.group(1)
        modules[schema] = {section: set() for section in SECTIONS}
        lines = block.splitlines()
        active: str | None = None
        for line in lines:
            section_match = re.match(r"^    (tables|views|functions):\s*$", line)
            if section_match:
                active = section_match.group(1)
                continue
            if re.match(r"^    [a-z_][a-z0-9_]*:", line):
                active = None
            item_match = re.match(r"^      - ([a-z_][a-z0-9_]*)$", line)
            if active and item_match:
                modules[schema][active].add(item_match.group(1))
    return modules


def main() -> int:
    inventory = parse_inventory(INVENTORY.read_text(encoding="utf-8"))
    discovered = {
        schema: {section: set() for section in SECTIONS}
        for schema in inventory
    }
    errors: list[str] = []

    migration_files = sorted((ROOT / "backend").glob("*/migrations/**/*.sql"))
    migration_files += sorted((ROOT / "backend").glob("*/migrations/*.sql"))
    for path in sorted(set(migration_files)):
        sql = path.read_text(encoding="utf-8")
        for section, pattern in PATTERNS.items():
            for qualified_name in pattern.findall(sql):
                schema, object_name = qualified_name.split(".", 1)
                if schema not in discovered:
                    errors.append(
                        f"{path.relative_to(ROOT)}: {section[:-1]} {qualified_name} "
                        "belongs to a schema absent from data-ownership.yaml"
                    )
                    continue
                discovered[schema][section].add(object_name)

    for schema, sections in discovered.items():
        declared = inventory[schema]
        for section in SECTIONS:
            missing = sorted(sections[section] - declared[section])
            stale = sorted(declared[section] - sections[section])
            if missing:
                errors.append(
                    f"{schema}.{section}: SQL objects missing from inventory: {', '.join(missing)}"
                )
            if stale:
                errors.append(
                    f"{schema}.{section}: inventory entries not found in migrations: {', '.join(stale)}"
                )

    if errors:
        for error in errors:
            print(f"sql-ownership-inventory: {error}", file=sys.stderr)
        return 1
    print(
        "SQL ownership inventory matches all schema-qualified tables, views and "
        "functions declared by backend migrations."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
