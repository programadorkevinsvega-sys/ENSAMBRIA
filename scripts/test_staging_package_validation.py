import hashlib
import io
import tarfile
import tempfile
import unittest
from pathlib import Path

from validate_staging_package import PackageValidationError, validate


BINARIES = ("core", "core-migrate", "core-email-worker", "business", "business-migrate")
MIGRATIONS = ("migrations/20261122000001_core_test.sql", "business-migrations/20261122000001_business_test.sql")


def write_package(path: Path, *, omit=(), non_executable=()):
    files = {name: (b"binary-content", 0o750) for name in BINARIES}
    files.update({name: (b"SELECT 1;\\n", 0o640) for name in MIGRATIONS})
    for name in non_executable:
        files[name] = (files[name][0], 0o640)
    for name in omit:
        files.pop(name, None)
    manifest = "".join(
        f"{hashlib.sha256(data).hexdigest()}  {name}\\n"
        for name, (data, _) in sorted(files.items())
    ).encode("ascii")
    files["SHA256SUMS"] = (manifest, 0o640)
    with tarfile.open(path, "w:gz") as archive:
        for name, (data, mode) in files.items():
            info = tarfile.TarInfo(name)
            info.size = len(data)
            info.mode = mode
            archive.addfile(info, io.BytesIO(data))


class StagingPackageValidationTests(unittest.TestCase):
    def test_complete_package_is_accepted(self):
        with tempfile.TemporaryDirectory() as directory:
            archive = Path(directory) / "release.tar.gz"
            write_package(archive)
            self.assertEqual(validate(str(archive)), 2)

    def test_missing_business_binary_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            archive = Path(directory) / "release.tar.gz"
            write_package(archive, omit=("business",))
            with self.assertRaisesRegex(PackageValidationError, "business"):
                validate(str(archive))

    def test_non_executable_binary_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            archive = Path(directory) / "release.tar.gz"
            write_package(archive, non_executable=("business",))
            with self.assertRaisesRegex(PackageValidationError, "not executable"):
                validate(str(archive))

    def test_tampered_manifest_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            archive = Path(directory) / "release.tar.gz"
            write_package(archive)
            with tarfile.open(archive, "r:gz") as source:
                members = source.getmembers()
                payloads = {member.name: source.extractfile(member).read() for member in members}
            payloads["business"] = b"tampered"
            with tarfile.open(archive, "w:gz") as output:
                for name, data in payloads.items():
                    info = tarfile.TarInfo(name)
                    info.size = len(data)
                    info.mode = 0o750 if name in BINARIES else 0o640
                    output.addfile(info, io.BytesIO(data))
            with self.assertRaisesRegex(PackageValidationError, "checksum mismatch"):
                validate(str(archive))


if __name__ == "__main__":
    unittest.main()
