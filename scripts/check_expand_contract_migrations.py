#!/usr/bin/env python3
"""Reject destructive PostgreSQL DDL in migrations while old releases may run."""
from __future__ import annotations

import pathlib
import re
import sys

FORBIDDEN = (
    (re.compile(r"\\bDROP\\s+(?:TABLE|COLUMN|TYPE|SCHEMA|INDEX|CONSTRAINT)\\b", re.I), "DROP requires a later contract migration"),
    (re.compile(r"\\bRENAME\\s+(?:COLUMN|TO)\\b", re.I), "RENAME is not backward-compatible with the previous release"),
    (re.compile(r"\\bALTER\\s+(?:TABLE\\s+)?[\\w.\"]+\\s+ALTER\\s+COLUMN\\s+[\\w\"]+\\s+(?:TYPE|SET\\s+NOT\\s+NULL)\\b", re.I), "column type/nullability changes require an expand/contract rollout"),
    (re.compile(r"\\bTRUNCATE\\b", re.I), "TRUNCATE is destructive and is not allowed in an automatic deployment"),
)


def strip_comments(sql: str) -> str:
    sql = re.sub(r"/\\*.*?\\*/", " ", sql, flags=re.S)
    return re.sub(r"--[^\\n]*", " ", sql)


def main() -> int:
    roots = [pathlib.Path(value) for value in sys.argv[1:]]
    failures = []
    for root in roots:
        if not root.is_dir():
            continue
        for path in sorted(root.rglob("*.sql")):
            source = strip_comments(path.read_text(encoding="utf-8"))
            for pattern, reason in FORBIDDEN:
                match = pattern.search(source)
                if match:
                    line = source.count("\\n", 0, match.start()) + 1
                    failures.append(f"{path}:{line}: {reason}")
    if failures:
        print("\\n".join(failures), file=sys.stderr)
        return 1
    print("migration compatibility policy passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
