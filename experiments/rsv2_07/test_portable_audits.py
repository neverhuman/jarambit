"""Portability regressions; filesystem mocks never rename or remove saved evidence."""

from contextlib import redirect_stdout
from copy import deepcopy
import hashlib
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
import audit_phase11_wave_receipts as wave_audit
import audit_phase12_cross_wave as cross_wave_audit

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


def synthetic_wave_documents():
    docs, designs = {}, []
    for round_id in (0, 1):
        identity = hashlib.sha256(f"synthetic-{round_id}".encode()).hexdigest()
        label = f"synthetic_round_{round_id}"
        docs[f"round_{round_id:02}_births.json"] = {
            "round": round_id, "seed": 1, "source": "synthetic fixture", "genome_header": "a,b",
            "genomes": [{"row": f"{round_id},1", "family": "synthetic", "certificate_identity": identity}]}
        screen = {key: 0.0 for key in wave_audit.SCREEN_NUMBERS}
        screen.update(label=label, family="synthetic", certificate_identity=identity,
                      qualification_status=wave_audit.PENDING)
        docs[f"round_{round_id:02}_screen.json"] = [screen]
        design = {key: 0.0 for key in wave_audit.DESIGN_NUMBERS}
        design.update(label=label, family="synthetic", certificate_identity=identity, qualified=False,
                      all_16_gates=False, rs_v2_all_hard=False, battery_pass_count=0,
                      genome_csv="synthetic.csv", scorecard_md="synthetic.md")
        designs.append(design)
    docs["leaderboard.json"] = {"schema": "jarambit.auto_evolve.leaderboard.v2", "designs": designs,
                               "assessed_count": 2, "qualified_count": 0, "champion": None, "unverified": []}
    return docs


class WaveReceiptAuditTests(unittest.TestCase):
    def setUp(self):
        self.docs = synthetic_wave_documents()

    def payloads(self):
        payloads = {name: json.dumps(doc, sort_keys=True).encode() for name, doc in self.docs.items()}
        expected = {name: (hashlib.sha256(data).hexdigest(), len(data)) for name, data in payloads.items()}
        return payloads, expected

    def test_synthetic_wave_accounting(self):
        payloads, expected = self.payloads()
        result = wave_audit.audit_payloads(payloads, expected)
        self.assertEqual((result["births"], result["screens"], result["assessed"]), (2, 2, 2))
        self.assertEqual((result["qualified"], result["unverified"], result["champion"]), (0, 0, None))

    def test_portable_wave_directory_with_in_memory_files(self):
        payloads, expected = self.payloads()
        directory = Path("/synthetic-checkout/saved-wave")

        def read(path):
            self.assertEqual(path.parent, directory)
            return payloads[path.name]

        with patch.object(Path, "read_bytes", new=read):
            self.assertEqual(wave_audit.audit_wave_directory(directory, expected)["assessed"], 2)

    def test_wave_byte_tampering_and_missing_files_fail(self):
        payloads, expected = self.payloads()
        payloads["leaderboard.json"] += b" "
        with self.assertRaisesRegex(ValueError, "SHA-256/size mismatch"):
            wave_audit.audit_payloads(payloads, expected)
        del payloads["leaderboard.json"]
        with self.assertRaisesRegex(ValueError, "Exactly five"):
            wave_audit.audit_payloads(payloads, expected)

    def test_wave_manifest_digest_and_size_formats(self):
        payloads, expected = self.payloads()
        name = "leaderboard.json"
        for sha, size in [("A" * 64, 1), ("a" * 63, 1), ("a" * 64, -1), ("a" * 64, True)]:
            changed = {**expected, name: (sha, size)}
            with self.subTest(sha=sha, size=size), self.assertRaises(ValueError):
                wave_audit.audit_payloads(payloads, changed)

    def test_missing_wave_qualification_fields_fail(self):
        del self.docs["leaderboard.json"]["designs"][0]["qualified"]
        with self.assertRaisesRegex(ValueError, "Missing schema fields"):
            wave_audit.audit_documents(self.docs)

    def test_wave_qualification_and_unverified_fail_closed(self):
        for key, value in [("qualified_count", 1), ("qualified_count", False), ("assessed_count", True),
                           ("champion", {}), ("unverified", ["missing evidence"]), ("unverified", 0)]:
            changed = deepcopy(self.docs)
            changed["leaderboard.json"][key] = value
            with self.subTest(key=key, value=value), self.assertRaises(ValueError):
                wave_audit.audit_documents(changed)
        for value in (True, 0, "false"):
            changed = deepcopy(self.docs)
            changed["leaderboard.json"]["designs"][0]["qualified"] = value
            with self.subTest(value=value), self.assertRaisesRegex(ValueError, "boolean false"):
                wave_audit.audit_documents(changed)
        changed = deepcopy(self.docs)
        changed["leaderboard.json"]["designs"][0].update(all_16_gates=True, rs_v2_all_hard=True)
        with self.assertRaisesRegex(ValueError, "Contradictory"):
            wave_audit.audit_documents(changed)

    def test_stage1_pending_is_not_a_qualification(self):
        screen = self.docs["round_00_screen.json"][0]
        screen["qualification_status"] = "qualified"
        with self.assertRaisesRegex(ValueError, "remain pending"):
            wave_audit.audit_documents(self.docs)
        screen["qualification_status"] = wave_audit.PENDING
        screen["qualified"] = True
        with self.assertRaisesRegex(ValueError, "boolean false"):
            wave_audit.audit_documents(self.docs)

    def test_cross_stage_identity_and_label_mismatches_fail(self):
        changed = deepcopy(self.docs)
        changed["round_00_screen.json"][0]["certificate_identity"] = "a" * 64
        with self.assertRaisesRegex(ValueError, "same-round births"):
            wave_audit.audit_documents(changed)
        changed = deepcopy(self.docs)
        changed["leaderboard.json"]["designs"][0]["certificate_identity"] = "a" * 64
        with self.assertRaisesRegex(ValueError, "identity/screen mismatch"):
            wave_audit.audit_documents(changed)
        changed = deepcopy(self.docs)
        changed["round_01_screen.json"][0]["label"] = changed["round_00_screen.json"][0]["label"]
        with self.assertRaisesRegex(ValueError, "Duplicate screen label"):
            wave_audit.audit_documents(changed)

    def test_conflicting_birth_rows_and_csv_width_fail(self):
        entry = self.docs["round_00_births.json"]["genomes"][0]
        self.docs["round_01_births.json"]["genomes"].append({**entry, "family": "another", "row": "changed,1"})
        with self.assertRaisesRegex(ValueError, "Conflicting rows"):
            wave_audit.audit_documents(self.docs)
        self.docs["round_01_births.json"]["genomes"].pop()
        entry["row"] = "one-column"
        with self.assertRaisesRegex(ValueError, "CSV/header width mismatch"):
            wave_audit.audit_documents(self.docs)

    def test_mutating_wave_snapshot_fails(self):
        payloads, expected = self.payloads()
        calls = {}

        def read(path):
            calls[path.name] = calls.get(path.name, 0) + 1
            return payloads[path.name] + (b" " if calls[path.name] > 1 else b"")

        with patch.object(Path, "read_bytes", new=read):
            with self.assertRaisesRegex(ValueError, "changed during audit"):
                wave_audit.audit_wave_directory(Path("/synthetic/wave"), expected)


class CrossWaveReceiptAuditTests(unittest.TestCase):
    def setUp(self):
        self.waves = {wave: synthetic_wave_documents() for wave in cross_wave_audit.PINS}

    def payloads(self):
        payloads = {wave: {name: json.dumps(doc, sort_keys=True).encode() for name, doc in docs.items()}
                    for wave, docs in self.waves.items()}
        pins = {wave: {name: (hashlib.sha256(data).hexdigest(), len(data)) for name, data in files.items()}
                for wave, files in payloads.items()}
        return payloads, pins

    def test_assessment_events_and_unique_identities_are_separate(self):
        result = cross_wave_audit.audit_documents(self.waves)
        self.assertEqual(result["totals"], {"births": 8, "screens": 8, "assessed": 8, "qualified": 0, "unverified": 0})
        self.assertEqual(result["unique_assessed_identities"], 2)
        self.assertEqual(result["birth_identities_in_multiple_waves"], 2)
        self.assertTrue(all(len(waves) == 4 for waves in result["assessed_identity_waves"].values()))

    def test_stage_linkage_is_checked_in_every_wave(self):
        self.waves["w317"]["round_00_screen.json"][0]["certificate_identity"] = "a" * 64
        with self.assertRaisesRegex(ValueError, "same-round births"):
            cross_wave_audit.audit_documents(self.waves)

    def test_same_identity_cannot_bind_different_cross_wave_rows_or_headers(self):
        for field in ("row", "header"):
            changed = deepcopy(self.waves)
            if field == "row":
                changed["w316"]["round_00_births.json"]["genomes"][0]["row"] = "changed,1"
            else:
                for round_id in (0, 1):
                    changed["w316"][f"round_{round_id:02}_births.json"]["genome_header"] = "different,b"
            wave_audit.audit_documents(changed["w316"])
            with self.subTest(field=field), self.assertRaisesRegex(ValueError, "Cross-wave identity binding mismatch"):
                cross_wave_audit.audit_documents(changed)

    def test_equal_rows_may_have_different_source_bound_identities(self):
        identity = hashlib.sha256(b"different synthetic source context").hexdigest()
        docs = self.waves["w316"]
        docs["round_00_births.json"]["genomes"][0]["certificate_identity"] = identity
        docs["round_00_screen.json"][0]["certificate_identity"] = identity
        docs["leaderboard.json"]["designs"][0]["certificate_identity"] = identity
        result = cross_wave_audit.audit_documents(self.waves)
        self.assertEqual(result["unique_assessed_identities"], 3)
        self.assertEqual(result["totals"]["assessed"], 8)

    def test_inconsistent_cross_wave_counts_and_qualification_fail(self):
        for key, value in [("assessed_count", 1), ("assessed_count", True), ("qualified_count", 1),
                           ("qualified_count", False), ("champion", {}), ("unverified", ["missing evidence"])]:
            changed = deepcopy(self.waves)
            changed["w318"]["leaderboard.json"][key] = value
            with self.subTest(key=key, value=value), self.assertRaises(ValueError):
                cross_wave_audit.audit_documents(changed)
        self.waves["w317"]["leaderboard.json"]["designs"][0]["qualified"] = True
        with self.assertRaisesRegex(ValueError, "boolean false"):
            cross_wave_audit.audit_documents(self.waves)

    def test_missing_wave_and_file_fail_closed(self):
        changed = dict(self.waves)
        del changed["w316"]
        with self.assertRaisesRegex(ValueError, "Exactly waves"):
            cross_wave_audit.audit_documents(changed)
        payloads, pins = self.payloads()
        del payloads["w315"]["leaderboard.json"]
        with self.assertRaisesRegex(ValueError, "Exactly five"):
            cross_wave_audit.audit_payloads(payloads, pins)

    def test_cross_wave_digest_tampering_fails(self):
        payloads, pins = self.payloads()
        payloads["w317"]["leaderboard.json"] += b" "
        with self.assertRaisesRegex(ValueError, "w317: SHA-256/size mismatch"):
            cross_wave_audit.audit_payloads(payloads, pins)
        payloads, pins = self.payloads()
        pins["w315"]["leaderboard.json"] = ("A" * 64, len(payloads["w315"]["leaderboard.json"]))
        with self.assertRaisesRegex(ValueError, "Invalid SHA-256"):
            cross_wave_audit.audit_payloads(payloads, pins)

    def test_portable_cross_wave_directories(self):
        payloads, pins = self.payloads()
        directories = {wave: Path("/synthetic") / wave for wave in self.waves}
        calls = []

        def read(path):
            calls.append(path)
            return payloads[path.parent.name][path.name]

        with patch.object(Path, "read_bytes", new=read):
            result = cross_wave_audit.audit_directories(directories, pins)
        self.assertEqual(result["totals"]["assessed"], 8)
        self.assertEqual(len(calls), 40)

    def test_cross_wave_mutation_during_audit_fails(self):
        payloads, pins = self.payloads()
        calls = {}

        def read(path):
            calls[path] = calls.get(path, 0) + 1
            return payloads[path.parent.name][path.name] + (b" " if calls[path] > 1 else b"")

        with patch.object(Path, "read_bytes", new=read):
            with self.assertRaisesRegex(ValueError, "Cross-wave artifacts changed"):
                cross_wave_audit.audit_directories({wave: Path("/synthetic") / wave for wave in self.waves}, pins)

    def test_explicit_cli_paths_need_no_git_or_host_wave_data(self):
        payloads, pins = self.payloads()
        output = io.StringIO()

        def read(path):
            return payloads[path.parent.name][path.name]

        args = ["audit_phase12_cross_wave.py", "--archive-root", "/synthetic/archive",
                "--live-wave-dir", "/synthetic/live/w318"]
        with patch.object(Path, "read_bytes", new=read), patch.object(cross_wave_audit, "PINS", pins), \
                patch.object(wave_audit, "default_wave_directory", side_effect=AssertionError("Unexpected Git access")), \
                patch.object(sys, "argv", args), redirect_stdout(output):
            cross_wave_audit.main()
        self.assertIn("20 file digests; 8 assessment events; 2 distinct assessed identities", output.getvalue())


if __name__ == "__main__":
    unittest.main(verbosity=2)
