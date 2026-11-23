#!/usr/bin/env python3
"""Validate a complete ENSAMBRIA staging release archive before host mutation."""
from __future__ import annotations

import hashlib
import re
import sys
import tarfile
from pathlib import PurePosixPath

BINARIES = {"core", "core-migrate", "core-email-worker", "business", "business-migrate"}
CORE_SQL = re.compile(r"migrations/[0-9]{14}_[a-z0-9_]+\\.sql\\Z")
BUSINESS_SQL = re.compile(r"business-migrations/(?:[a-z0-9_-]+/)?[0-9]{14}_[a-z0-9_]+\\.sql\\Z")
ALLOWED_DIRS = {"migrations", "business-migrations", "business-migrations/bootstrap"}
CHECKSUM_LINE = re.compile(r"([0-9a-f]{64})  ([A-Za-z0-9_./-]+)\\Z")


class PackageValidationError(ValueError):
    pass


def validate(path: str) -> int:
    try:
        archive = tarfile.open(path, mode="r:gz")
    except (OSError, tarfile.TarError) as error:
        raise PackageValidationError(f"cannot open deployment archive: {error}") from error

    with archive:
        files: dict[str, tarfile.TarInfo] = {}
        directories: set[str] = set()
        for member in archive.getmembers():
            name = member.name.rstrip("/")
            pure = PurePosixPath(name)
            if not name or pure.is_absolute() or ".." in pure.parts or "." in pure.parts:
                raise PackageValidationError(f"unsafe archive path: {member.name!r}")
            if member.issym() or member.islnk() or not (member.isfile() or member.isdir()):
                raise PackageValidationError(f"non-regular archive entry: {member.name!r}")
            if member.isdir():
                if name not in ALLOWED_DIRS:
                    raise PackageValidationError(f"unexpected directory: {name}")
                directories.add(name)
                continue
            if name in files:
                raise PackageValidationError(f"duplicate archive entry: {name}")
            if name in BINARIES:
                if member.size <= 0 or not (member.mode & 0o111):
                    raise PackageValidationError(f"binary is empty or not executable: {name}")
            elif name == "SHA256SUMS":
                if member.size <= 0:
                    raise PackageValidationError("SHA256SUMS is empty")
            elif CORE_SQL.fullmatch(name) or BUSINESS_SQL.fullmatch(name):
                if member.size <= 0:
                    raise PackageValidationError(f"empty migration file: {name}")
            else:
                raise PackageValidationError(f"unexpected archive file: {name}")
            files[name] = member

        missing = BINARIES - files.keys()
        if missing:
            raise PackageValidationError("missing required binaries: " + ", ".join(sorted(missing)))
        core_migrations = {name for name in files if CORE_SQL.fullmatch(name)}
        business_migrations = {name for name in files if BUSINESS_SQL.fullmatch(name)}
        if not core_migrations or not business_migrations:
            raise PackageValidationError("both CORE and BUSINESS SQL migrations are required")
        if "SHA256SUMS" not in files:
            raise PackageValidationError("SHA256SUMS is required")

        checksum_stream = archive.extractfile(files["SHA256SUMS"])
        if checksum_stream is None:
            raise PackageValidationError("cannot read SHA256SUMS")
        manifest = checksum_stream.read().decode("ascii")
        expected: dict[str, str] = {}
        for line in manifest.splitlines():
            match = CHECKSUM_LINE.fullmatch(line)
            if not match:
                raise PackageValidationError("invalid SHA256SUMS entry")
            digest, name = match.groups()
            if name in expected or name == "SHA256SUMS":
                raise PackageValidationError(f"duplicate or invalid checksum entry: {name}")
            expected[name] = digest

        required_hashes = set(files) - {"SHA256SUMS"}
        if set(expected) != required_hashes:
            raise PackageValidationError("SHA256SUMS does not cover exactly every release file")
        for name, digest in expected.items():
            stream = archive.extractfile(files[name])
            if stream is None:
                raise PackageValidationError(f"cannot read archive file: {name}")
            actual = hashlib.sha256(stream.read()).hexdigest()
            if actual != digest:
                raise PackageValidationError(f"checksum mismatch: {name}")

        return len(core_migrations) + len(business_migrations)


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: validate_staging_package.py ARCHIVE", file=sys.stderr)
        return 2
    try:
        count = validate(sys.argv[1])
    except (PackageValidationError, OSError, UnicodeError, tarfile.TarError) as error:
        print(f"deployment package rejected: {error}", file=sys.stderr)
        return 1
    print(f"deployment package validated: 5 executable binaries, {count} SQL migrations, checksums verified")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
