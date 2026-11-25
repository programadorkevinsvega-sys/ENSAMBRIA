#!/usr/bin/env python3
"""Render canonical YAML policies as SQL VALUES tuples for psql."""
from __future__ import annotations

import importlib.util
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
VALIDATOR_PATH = ROOT / "scripts/ci/check-sql-ownership-inventory.py"
INVENTORY_PATH = ROOT / "architecture/data-ownership.yaml"


def load_validator():
    spec = importlib.util.spec_from_file_location("sql_ownership_validator", VALIDATOR_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load SQL ownership validator: {VALIDATOR_PATH}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def render_policy_values(inventory_text: str) -> str:
    inventory = load_validator().parse_inventory(inventory_text)
    rows: list[str] = []
    for schema, sections in sorted(inventory.items()):
        for item in sorted(sections["policies"]):
            table_name, policy_name = item.split(".", 1)
            rows.append(f"('{schema}', '{table_name}', '{policy_name}')")
    if not rows:
        raise ValueError("canonical ownership inventory contains no RLS policies")
    return ",\n        ".join(rows)


def main() -> int:
    try:
        print(render_policy_values(INVENTORY_PATH.read_text(encoding="utf-8")))
    except (OSError, RuntimeError, ValueError) as error:
        print(f"postgres-policy-inventory: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
