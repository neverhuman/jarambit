"""Portability regressions; filesystem mocks never rename or remove saved evidence."""

from contextlib import redirect_stdout
import io
import json
from pathlib import Path
import runpy
import sys
import unittest
from unittest.mock import patch

import portable_audit as portable

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

    def test_default_commands_with_saved_raw(self):
        output = self.commands()
        self.assertIn("164 on-disk, 0 manifest-only", output)
        self.assertIn("9 on-disk, 0 manifest-only", output)
        self.assertIn("Native replay: PASS", output)

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
        with patch.object(portable, "digest", return_value="0" * 64):
            with self.assertRaisesRegex(ValueError, "SHA-256 mismatch"):
                portable.check_file(ROOT, name, expected, (ROOT / name).stat().st_size)

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
        with self.assertRaisesRegex(ValueError, "Byte size mismatch"):
            portable.check_file(ROOT, name, expected, (ROOT / name).stat().st_size + 1)

    def test_unknown_historical_hash_fails(self):
        with self.assertRaisesRegex(ValueError, "not found in Git history"):
            portable.check_file(ROOT, "crates/nonexistent-portability-fixture.rs", "0" * 64)


if __name__ == "__main__":
    unittest.main(verbosity=2)
