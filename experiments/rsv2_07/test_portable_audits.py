"""Portability regressions; filesystem mocks never rename or remove saved evidence."""

from contextlib import redirect_stdout
from copy import deepcopy
import io
import json
from pathlib import Path
import runpy
import sys
from types import SimpleNamespace
import unittest
from unittest.mock import patch

import portable_audit as portable
import audit_phase6 as receipt_audit

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
RAW_ROOT = ROOT / portable.RAW
REAL_STAT = Path.stat
REAL_DIGEST = portable.digest
PHASE6 = json.loads((HERE / "results/phase6_unified_scorecard.json").read_text())


def absent_raw(path, *args, **kwargs):
    if path.is_relative_to(RAW_ROOT):
        raise FileNotFoundError(str(path))
    return REAL_STAT(path, *args, **kwargs)


class PortabilityTests(unittest.TestCase):
    def commands(self):
        output = io.StringIO()
        with redirect_stdout(output):
            for script in ("audit_phase7.py", "audit_phase8.py"):
                with patch.object(sys, "argv", [script]):
                    runpy.run_path(str(HERE / script), run_name="__main__")
        return output.getvalue()

    def test_default_commands_with_available_evidence(self):
        output = self.commands()
        p7_disk = sum((ROOT / name).is_file() for name in PHASE6["artifacts_sha256"])
        p8 = json.loads((HERE / "results/phase8_synthetic_gci_audit.json").read_text())
        p8_names = [name for name in p8["sha256"] if name.startswith(portable.RAW)]
        p8_disk = sum((ROOT / name).is_file() for name in p8_names)
        self.assertIn(f"{p7_disk} on-disk, {164 - p7_disk} manifest-only", output)
        self.assertIn(f"{p8_disk} on-disk, {9 - p8_disk} manifest-only", output)
        binary = ROOT / p8["raw_directory"] / "synthetic-gci-probe"
        self.assertIn("Native replay: PASS" if binary.is_file() else "Native replay: NOT RUN", output)

    def test_default_commands_without_target(self):
        with patch.object(Path, "stat", new=absent_raw):
            output = self.commands()
        self.assertIn("0 on-disk, 164 manifest-only", output)
        self.assertIn("0 on-disk, 9 manifest-only", output)
        self.assertIn("Native replay: NOT RUN", output)
        print("Simulated clean checkout:\n" + output.rstrip())

    def test_newer_sources_use_git_history_without_target(self):
        newer = {ROOT / "crates/cli/src/auto_evolve.rs", ROOT / "crates/crescent/src/gci.rs",
                 ROOT / "docs/WORKPLAN.md"}

        def changed_digest(path):
            return "0" * 64 if path in newer else REAL_DIGEST(path)

        with patch.object(Path, "stat", new=absent_raw), patch.object(portable, "digest", side_effect=changed_digest):
            output = self.commands()
        self.assertIn("3 historical snapshots", output)
        self.assertIn("1 historical snapshots", output)
        self.assertIn("0 on-disk, 164 manifest-only", output)
        print("Simulated clean checkout with newer source files:\n" + output.rstrip())

    def test_changed_target_is_not_downgraded_to_manifest_only(self):
        name, expected = next(iter(PHASE6["artifacts_sha256"].items()))
        with patch.object(Path, "stat", return_value=SimpleNamespace(st_size=0)), \
                patch.object(portable, "digest", return_value="0" * 64):
            with self.assertRaisesRegex(ValueError, "SHA-256 mismatch"):
                portable.check_file(ROOT, name, expected, 0)

    def test_missing_study_file_never_uses_history(self):
        name = "experiments/rsv2_07/audit_phase7.py"
        with patch.object(Path, "stat", side_effect=FileNotFoundError), patch.object(portable, "historical_blob") as history:
            with self.assertRaisesRegex(ValueError, "Missing tracked study file"):
                portable.check_file(ROOT, name, "a" * 64)
            history.assert_not_called()

    def test_changed_study_file_never_uses_history(self):
        with patch.object(portable, "historical_blob") as history:
            with self.assertRaisesRegex(ValueError, "SHA-256 mismatch"):
                portable.check_file(ROOT, "experiments/rsv2_07/audit_phase7.py", "0" * 64)
            history.assert_not_called()

    def test_manifest_only_requires_real_digest_and_size(self):
        name = "target/rsv2-07/nonexistent-test-fixture"
        for sha, size in [("bad", 0), ("z" * 64, 0), ("a" * 64, -1),
                          ("a" * 64, None), ("a" * 64, True), ("a" * 64, 1.5)]:
            with self.subTest(sha=sha, size=size), self.assertRaises(ValueError):
                portable.check_file(ROOT, name, sha, size)
        result = portable.check_file(ROOT, name, "a" * 64, 0)
        self.assertEqual(result["mode"], "manifest_only")

    def test_present_raw_size_mismatch_fails(self):
        name, expected = next(iter(PHASE6["artifacts_sha256"].items()))
        with patch.object(Path, "stat", return_value=SimpleNamespace(st_size=3)), \
                patch.object(portable, "digest", return_value=expected):
            with self.assertRaisesRegex(ValueError, "Byte size mismatch"):
                portable.check_file(ROOT, name, expected, 4)

    def test_unknown_historical_hash_fails(self):
        with self.assertRaisesRegex(ValueError, "not found in Git history"):
            portable.check_file(ROOT, "crates/nonexistent-portability-fixture.rs", "0" * 64)


class ReceiptAuditTests(unittest.TestCase):
    def setUp(self):
        self.docs = {name: receipt_audit.load_json((HERE / "results" / name).read_text())
                     for name in receipt_audit.SPECS}

    def test_receipt_schemas(self):
        for name, doc in self.docs.items():
            with self.subTest(name=name):
                self.assertGreater(receipt_audit.check_schema(name, doc), 0)

    def test_missing_required_keys(self):
        for name, doc in self.docs.items():
            changed = deepcopy(doc)
            del changed[next(iter(receipt_audit.SPECS[name][1]))]
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, "Missing/invalid"):
                receipt_audit.check_schema(name, changed)

    def test_invalid_digest_formats(self):
        for value in ("A" * 64, "a" * 63, "g" * 64, "", None, True, 42):
            with self.subTest(value=value), self.assertRaisesRegex(ValueError, "Invalid SHA-256"):
                receipt_audit.check_digest_fields({"candidate_sha256": value})

    def test_boolean_verification_flag_is_not_a_digest(self):
        self.assertEqual(receipt_audit.check_digest_fields(
            {"production_surface_matches_3d_source_sha256": True}), 0)

    def test_duplicate_keys_and_nonfinite_numbers_fail(self):
        for text in ('{"a":1,"a":2}', '{"x":NaN}', '{"x":Infinity}', '{"x":1e999}'):
            with self.subTest(text=text), self.assertRaises(ValueError):
                receipt_audit.load_json(text)

    def test_phase6_table_tampering_fails(self):
        text = (ROOT / receipt_audit.PHASE6_REPORT).read_text()
        changed = text.replace("| Q7 | 15/16 |", "| Q7 | 0/16 |", 1)
        self.assertNotEqual(text, changed)
        with self.assertRaisesRegex(ValueError, "overview table/JSON mismatch"):
            receipt_audit.check_phase6_tables(self.docs["phase6_unified_scorecard.json"], changed)
        line = next(line for line in text.splitlines() if line.startswith("| G16-01 |"))
        changed = text.replace(line, line.replace(" pass ", " fail ", 1), 1)
        with self.assertRaisesRegex(ValueError, "engineering_gates table/JSON mismatch"):
            receipt_audit.check_phase6_tables(self.docs["phase6_unified_scorecard.json"], changed)

    def test_inventory_table_tampering_fails(self):
        manifest = receipt_audit.load_json((ROOT / receipt_audit.MANIFEST).read_text())
        text = (ROOT / receipt_audit.REPORT).read_text()
        sha = manifest["files"][receipt_audit.HERE + "results/phase6_unified_scorecard.json"]
        changed = text.replace(sha, "0" * 64, 1)
        self.assertNotEqual(text, changed)
        with self.assertRaisesRegex(ValueError, "inventory table/JSON mismatch"):
            receipt_audit.check_inventory(self.docs, manifest["files"], changed)

    def test_default_command_is_read_only(self):
        paths = [ROOT / receipt_audit.MANIFEST, ROOT / receipt_audit.REPORT]
        paths += [HERE / "results" / name for name in self.docs]
        before = [REAL_DIGEST(path) for path in paths]
        with redirect_stdout(io.StringIO()), patch.object(sys, "argv", ["audit_phase6.py"]):
            runpy.run_path(str(HERE / "audit_phase6.py"), run_name="__main__")
        self.assertEqual(before, [REAL_DIGEST(path) for path in paths])

    def test_default_command_without_target(self):
        output = io.StringIO()
        with patch.object(Path, "stat", new=absent_raw), redirect_stdout(output), \
                patch.object(sys, "argv", ["audit_phase6.py"]):
            runpy.run_path(str(HERE / "audit_phase6.py"), run_name="__main__")
        self.assertIn("0 on-disk, 196 manifest-only", output.getvalue())

    def test_changed_receipt_file_fails(self):
        target = HERE / "results/phase6_unified_scorecard.json"

        def tampered(path):
            return "0" * 64 if path == target else REAL_DIGEST(path)

        with patch.object(portable, "digest", side_effect=tampered):
            with self.assertRaisesRegex(ValueError, "SHA-256 mismatch"):
                receipt_audit.verify()

    def test_incomplete_raw_manifest_fails(self):
        original = receipt_audit.load_json

        def incomplete(text):
            value = original(text)
            if value.get("schema") == "jarambit.rsv2_07.portable_receipt_manifest.v1":
                value["raw_artifacts"].pop(next(iter(value["raw_artifacts"])))
            return value

        with patch.object(receipt_audit, "load_json", side_effect=incomplete):
            with self.assertRaisesRegex(ValueError, "Raw manifest coverage mismatch"):
                receipt_audit.verify()


if __name__ == "__main__":
    unittest.main(verbosity=2)
