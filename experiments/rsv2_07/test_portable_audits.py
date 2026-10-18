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
import audit_phase13_screen_rounds as screen_audit
import audit_phase14_w319_receipts as w319_audit
import audit_phase15_gate_s_and_margins as phase15_audit
import audit_phase16_xbabe3_waves as xbabe3_audit
import audit_phase17_portable_schema as schema_audit

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


class ScreenRoundArtifactTests(unittest.TestCase):
    def setUp(self):
        self.docs = {}
        for round_id in range(3):
            births, screens = [], []
            for slot in range(1, 10):
                identity = hashlib.sha256(f"synthetic-{round_id}-{slot}".encode()).hexdigest()
                births.append({"family": "fixture", "certificate_identity": identity, "row": f"{round_id},{slot}"})
                screens.append({"family": "fixture", "certificate_identity": identity,
                                "label": f"w315_r{round_id + 1:02}_fixture_c{slot:02}",
                                "qualification_status": wave_audit.PENDING})
            self.docs[f"round_{round_id:02}_births.json"] = {
                "round": round_id, "seed": 1, "source": "synthetic", "genome_header": "a,b", "genomes": births}
            self.docs[f"round_{round_id:02}_screen.json"] = screens

    def payloads(self, docs=None):
        payloads = {name: json.dumps(doc, sort_keys=True).encode() for name, doc in (self.docs if docs is None else docs).items()}
        pins = {name: (hashlib.sha256(data).hexdigest(), len(data)) for name, data in payloads.items()}
        return payloads, pins

    def test_three_rounds_and_saved_slot_representation(self):
        result = screen_audit.audit_documents("w315", self.docs)
        self.assertEqual(result["unavailable_rounds"], [])
        for counts in result["rounds"].values():
            self.assertEqual((counts["births"], counts["screens"]), (9, 9))
            self.assertTrue(all(value == {"birth_present": True, "screen_present": True}
                                for value in counts["slots"].values()))
            self.assertEqual(counts["metric_field_presence"], dict.fromkeys(screen_audit.METRIC_KEYS, 0))

    def test_missing_round_is_unavailable_and_unscreened_slot_is_not_failure(self):
        self.docs.pop("round_02_births.json")
        self.docs.pop("round_02_screen.json")
        self.docs["round_01_screen.json"] = []
        result = screen_audit.audit_documents("w315", self.docs)
        self.assertEqual(result["unavailable_rounds"], [2])
        self.assertFalse(result["rounds"]["1"]["slots"]["c06"]["screen_present"])

    def test_field_presence_is_not_a_performance_statistic(self):
        self.docs["round_00_screen.json"][0]["lateral_sf"] = 123
        counts = screen_audit.audit_documents("w315", self.docs)["rounds"]["0"]
        self.assertEqual(counts["metric_field_presence"]["lateral_sf"], 1)
        self.assertNotIn("statistics", counts)

    def test_digest_tamper_and_invalid_pins_fail(self):
        payloads, pins = self.payloads()
        name = "round_02_screen.json"
        payloads[name] += b" "
        with self.assertRaisesRegex(ValueError, "SHA-256/size mismatch"):
            screen_audit.audit_payloads("w315", payloads, pins)
        payloads, pins = self.payloads()
        for pin in [("X" * 64, 1), ("a" * 64, True)]:
            pins[name] = pin
            with self.subTest(pin=pin), self.assertRaises(ValueError):
                screen_audit.audit_payloads("w315", payloads, pins)

    def test_unpinned_file_or_missing_hash_fails(self):
        payloads, pins = self.payloads()
        del pins["round_02_births.json"]
        with self.assertRaisesRegex(ValueError, "coverage mismatch"):
            screen_audit.audit_payloads("w315", payloads, pins)

    def test_exact_birth_slot_is_checked_even_when_identity_exists(self):
        row = self.docs["round_00_screen.json"][0]
        row["certificate_identity"] = self.docs["round_00_births.json"]["genomes"][1]["certificate_identity"]
        with self.assertRaisesRegex(ValueError, "identity/birth slot mismatch"):
            screen_audit.audit_documents("w315", self.docs)

    def test_wrong_wave_round_family_and_duplicate_labels_fail(self):
        for label in ["w316_r01_fixture_c01", "w315_r02_fixture_c01", "w315_r01_other_c01", "w315_r01_fixture_c00"]:
            changed = deepcopy(self.docs)
            changed["round_00_screen.json"][0]["label"] = label
            with self.subTest(label=label), self.assertRaises(ValueError):
                screen_audit.audit_documents("w315", changed)
        self.docs["round_00_screen.json"].append(deepcopy(self.docs["round_00_screen.json"][0]))
        with self.assertRaisesRegex(ValueError, "Duplicate screen"):
            screen_audit.audit_documents("w315", self.docs)

    def test_incomplete_gapped_rounds_and_conflicting_rows_fail(self):
        changed = deepcopy(self.docs)
        del changed["round_01_screen.json"]
        with self.assertRaisesRegex(ValueError, "Incomplete"):
            screen_audit.audit_documents("w315", changed)
        del changed["round_01_births.json"]
        with self.assertRaisesRegex(ValueError, "contiguous"):
            screen_audit.audit_documents("w315", changed)
        entry = self.docs["round_00_births.json"]["genomes"][0]
        self.docs["round_01_births.json"]["genomes"].append({**entry, "row": "different,1"})
        with self.assertRaisesRegex(ValueError, "Conflicting identity"):
            screen_audit.audit_documents("w315", self.docs)

    def test_round_type_csv_and_pending_status_fail_closed(self):
        for field, value in [("round", False), ("genome_header", "a,a"), ("genomes", {})]:
            changed = deepcopy(self.docs)
            changed["round_00_births.json"][field] = value
            with self.subTest(field=field), self.assertRaises(ValueError):
                screen_audit.audit_documents("w315", changed)
        self.docs["round_00_screen.json"][0]["qualification_status"] = "qualified"
        with self.assertRaisesRegex(ValueError, "remain pending"):
            screen_audit.audit_documents("w315", self.docs)

    def directory_fixture(self):
        # No original host paths or wave files are accessed by these tests.
        payloads, pins = {}, {}
        for wave in screen_audit.PINS:
            docs = deepcopy(self.docs)
            for name, rows in docs.items():
                if name.endswith("_screen.json"):
                    for row in rows:
                        row["label"] = row["label"].replace("w315_", wave + "_")
            payloads[wave], pins[wave] = self.payloads(docs)
        return payloads, pins

    def test_relocated_directories_optional_wave_and_missing_required_wave(self):
        payloads, pins = self.directory_fixture()
        output = io.StringIO()
        with patch.object(Path, "is_dir", new=lambda path: path.name != "w319"), \
                patch.object(Path, "is_file", return_value=True), \
                patch.object(Path, "read_bytes", new=lambda path: payloads[path.parent.name][path.name]), \
                patch.object(screen_audit, "PINS", pins), \
                patch.object(wave_audit, "default_wave_directory", side_effect=AssertionError("Unexpected Git access")), \
                patch.object(sys, "argv", ["audit_phase13_screen_rounds.py", "--out-root", "/synthetic/out"]), \
                redirect_stdout(output):
            screen_audit.main()
        self.assertIn("24 pinned file digests", output.getvalue())
        self.assertIn("absent optional waves=['w319']", output.getvalue())
        with patch.object(Path, "is_dir", return_value=False), self.assertRaisesRegex(ValueError, "Missing required"):
            screen_audit.audit_directories(Path("/synthetic/out"), pins)

    def test_inventory_and_byte_mutation_during_audit_fail(self):
        payloads, pins = self.directory_fixture()
        for mode in ("inventory", "bytes"):
            reads = {}

            def read(path):
                reads[path] = reads.get(path, 0) + 1
                return payloads[path.parent.name][path.name] + (b" " if mode == "bytes" and reads[path] > 1 else b"")

            calls = []

            def is_file(path):
                calls.append(path)
                return not (mode == "inventory" and len(calls) > 24 and path.name == "round_02_screen.json")

            with self.subTest(mode=mode), patch.object(Path, "is_dir", new=lambda path: path.name != "w319"), \
                    patch.object(Path, "is_file", new=is_file), patch.object(Path, "read_bytes", new=read), \
                    self.assertRaisesRegex(ValueError, "changed during audit"):
                screen_audit.audit_directories(Path("/synthetic/out"), pins)


class W319ReceiptAuditTests(unittest.TestCase):
    def setUp(self):
        self.docs = synthetic_wave_documents()
        self.cards, designs = {}, []
        for round_id in (0, 1):
            births = self.docs[f"round_{round_id:02}_births.json"]["genomes"]
            screens = self.docs[f"round_{round_id:02}_screen.json"]
            birth_template, screen_template = deepcopy(births[0]), deepcopy(screens[0])
            births.clear()
            screens.clear()
            for slot in (1, 2):
                identity = hashlib.sha256(f"synthetic-w319-{round_id}-{slot}".encode()).hexdigest()
                label = f"w319_r{round_id + 1:02}_synthetic_c{slot:02}"
                key = f"by-content/{identity}/attempt-{round_id * 2 + slot}/{label}_master_scorecard.md"
                births.append({**birth_template, "certificate_identity": identity, "row": f"{round_id},{slot}"})
                screens.append({**screen_template, "certificate_identity": identity, "label": label})
                design = {**self.docs["leaderboard.json"]["designs"][round_id], "label": label,
                          "certificate_identity": identity, "scorecard_md": "out/w319/" + key}
                designs.append(design)
                self.cards[key] = f"# Master CAD Scorecard — `{label}`\n\n- **Verdict**: **FAIL (Gate Violation)**\n"
        self.docs["leaderboard.json"].update(designs=designs, assessed_count=4)
        self.markdown = "# Evolution certification attempts\n\n| " + " | ".join(w319_audit.BOARD_HEADER) + " |\n"
        self.markdown += "|---|---|---:|---:|---:|---:|---|---|\n"
        self.markdown += "".join(f"| `{d['label']}` | false | 0 | 0 | 0 | 0 | false | 0/18 |\n" for d in designs)

    def payloads(self):
        files = {name: json.dumps(doc, sort_keys=True).encode() for name, doc in self.docs.items()}
        files["leaderboard.md"] = self.markdown.encode()
        files.update({name: text.encode() for name, text in self.cards.items()})
        pins = {name: (hashlib.sha256(data).hexdigest(), len(data)) for name, data in files.items()}
        return files, pins

    def test_synthetic_receipt_accounting_and_four_card_bindings(self):
        result = w319_audit.audit_payloads(*self.payloads())
        self.assertEqual((result["births"], result["screens"], result["assessed"], result["verified_files"]), (4, 4, 4, 10))
        self.assertEqual((result["qualified"], result["champion"], result["unverified"]), (0, None, 0))
        self.assertEqual(set(result["scorecards"].values()), set(self.cards))

    def test_every_pinned_file_rejects_byte_tampering(self):
        payloads, pins = self.payloads()
        for name in payloads:
            changed = {**payloads, name: payloads[name] + b" "}
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, "SHA-256/size mismatch"):
                w319_audit.audit_payloads(changed, pins)

    def test_missing_extra_files_and_invalid_pins_fail(self):
        payloads, pins = self.payloads()
        for name in ("leaderboard.json", next(iter(self.cards))):
            changed = dict(payloads)
            del changed[name]
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, "Exactly ten"):
                w319_audit.audit_payloads(changed, pins)
        with self.assertRaisesRegex(ValueError, "Exactly ten"):
            w319_audit.audit_payloads({**payloads, "extra.md": b""}, pins)
        for sha, size in [("A" * 64, 0), ("a" * 64, True), ("a" * 64, -1)]:
            changed = {**pins, "leaderboard.md": (sha, size)}
            with self.subTest(sha=sha, size=size), self.assertRaises(ValueError):
                w319_audit.audit_payloads(payloads, changed)

    def test_birth_slot_and_assessed_screen_linkage_fail(self):
        screen = self.docs["round_00_screen.json"][0]
        screen["certificate_identity"] = self.docs["round_00_births.json"]["genomes"][1]["certificate_identity"]
        self.docs["leaderboard.json"]["designs"][0]["certificate_identity"] = screen["certificate_identity"]
        with self.assertRaisesRegex(ValueError, "birth slot mismatch"):
            w319_audit.audit_payloads(*self.payloads())
        self.docs["leaderboard.json"]["designs"][0]["certificate_identity"] = "a" * 64
        with self.assertRaisesRegex(ValueError, "identity/screen mismatch"):
            w319_audit.audit_payloads(*self.payloads())

    def test_qualification_count_champion_and_unverified_fail_closed(self):
        board = self.docs["leaderboard.json"]
        for key, value in [("qualified_count", 1), ("qualified_count", False), ("assessed_count", True),
                           ("champion", {"label": "synthetic"}), ("unverified", ["synthetic-error"])]:
            original = board[key]
            board[key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                w319_audit.audit_payloads(*self.payloads())
            board[key] = original
        board["designs"][0]["qualified"] = True
        with self.assertRaisesRegex(ValueError, "boolean false"):
            w319_audit.audit_payloads(*self.payloads())

    def test_markdown_false_claims_missing_rows_and_wrong_labels_fail(self):
        original = self.markdown
        variants = [original.replace("| false |", "| true |", 1),
                    original.replace("w319_r01_synthetic_c01", "unknown", 1),
                    "\n".join(original.splitlines()[:-1])]
        for text in variants:
            self.markdown = text
            with self.subTest(text=text), self.assertRaisesRegex(ValueError, "Leaderboard Markdown"):
                w319_audit.audit_payloads(*self.payloads())

    def test_scorecard_title_and_stored_verdict_mismatch_fail(self):
        key = next(iter(self.cards))
        original = self.cards[key]
        for text in [original.replace("w319_r01_synthetic_c01", "another-label"),
                     original.replace("FAIL (Gate Violation)", "PASS"),
                     original + "- **Verdict**: **FAIL (Gate Violation)**\n"]:
            self.cards[key] = text
            with self.subTest(text=text), self.assertRaisesRegex(ValueError, "Scorecard"):
                w319_audit.audit_payloads(*self.payloads())

    def test_scorecard_path_identity_attempt_label_and_traversal_fail(self):
        design = self.docs["leaderboard.json"]["designs"][0]
        original = design["scorecard_md"]
        variants = [original.replace(design["certificate_identity"], "a" * 64),
                    original.replace("attempt-1", "attempt-x"), original.replace("_c01_master", "_c02_master"),
                    "../" + original, original.replace("out/", "out/./", 1), original.replace("out/", "out//", 1)]
        for reference in variants:
            design["scorecard_md"] = reference
            with self.subTest(reference=reference), self.assertRaisesRegex(ValueError, "[Ss]corecard"):
                w319_audit.audit_payloads(*self.payloads())

    def test_absolute_original_references_need_no_original_files(self):
        for design in self.docs["leaderboard.json"]["designs"]:
            design["scorecard_md"] = "/original/checkout/" + design["scorecard_md"]
        self.assertEqual(w319_audit.audit_payloads(*self.payloads())["verified_files"], 10)

    def test_unreferenced_or_missing_attempt_is_rejected(self):
        key = next(iter(self.cards))
        self.cards[key.replace("attempt-1", "attempt-99")] = self.cards.pop(key)
        with self.assertRaisesRegex(ValueError, "Missing or reused assessed scorecard"):
            w319_audit.audit_payloads(*self.payloads())

    def test_strict_json_duplicate_keys_fail_after_valid_digest(self):
        payloads, pins = self.payloads()
        data = b'{"schema":"one","schema":"two"}'
        payloads["leaderboard.json"] = data
        pins["leaderboard.json"] = (hashlib.sha256(data).hexdigest(), len(data))
        with self.assertRaisesRegex(ValueError, "Duplicate"):
            w319_audit.audit_payloads(payloads, pins)

    def test_relocated_cli_reads_only_ten_pinned_files_without_git(self):
        payloads, pins = self.payloads()
        directory = Path("/synthetic/relocated/w319")
        reads, output = [], io.StringIO()

        def read(path):
            key = str(path.relative_to(directory))
            reads.append(key)
            return payloads[key]

        with patch.object(Path, "read_bytes", new=read), patch.object(w319_audit, "PINS", pins), \
                patch.object(wave_audit, "default_wave_directory", side_effect=AssertionError("Unexpected Git access")), \
                patch.object(sys, "argv", ["audit_phase14_w319_receipts.py", "--wave-dir", str(directory)]), \
                redirect_stdout(output):
            w319_audit.main()
        self.assertEqual(len(reads), 20)
        self.assertEqual(set(reads), set(payloads))
        self.assertIn("10 pinned file digests", output.getvalue())
        self.assertIn("qualified_count=0; champion=null", output.getvalue())

    def test_mutating_snapshot_and_unsafe_pin_paths_fail(self):
        payloads, pins = self.payloads()
        directory, reads = Path("/synthetic/w319"), {}

        def read(path):
            key = str(path.relative_to(directory))
            reads[key] = reads.get(key, 0) + 1
            return payloads[key] + (b" " if reads[key] > 1 else b"")

        with patch.object(Path, "read_bytes", new=read), self.assertRaisesRegex(ValueError, "changed during audit"):
            w319_audit.audit_directory(directory, pins)
        with patch.object(Path, "read_bytes") as read_mock, self.assertRaisesRegex(ValueError, "Invalid pinned artifact path"):
            w319_audit.audit_directory(directory, {"../outside": ("a" * 64, 0)})
        read_mock.assert_not_called()


class Phase15GateSMarginAuditTests(unittest.TestCase):
    """Artifact/layout subset only; no real Gate S or mechanical-margin evaluation."""

    def setUp(self):
        self.board = {"schema": "jarambit.auto_evolve.leaderboard.v2", "assessed_count": 4,
                      "qualified_count": 0, "champion": None, "unverified": [], "designs": []}
        self.cards = {}
        for slot in range(4):
            label = f"w319_fixture_{slot}"
            identity = hashlib.sha256(label.encode()).hexdigest()
            key = f"by-content/{identity}/attempt-{slot}/{label}_master_scorecard.md"
            self.board["designs"].append({"label": label, "family": "fixture", "certificate_identity": identity,
                                          "scorecard_md": "out/w319/" + key, "qualified": False})
            tables = []
            for index, (header, count) in enumerate(phase15_audit.TABLE_LAYOUT):
                table = ["| " + " | ".join(header) + " |", "|" + "|".join("---" for _ in header) + "|"]
                for row in range(1, count + 1):
                    if index == 0:
                        cells = [f"component_{row}", "opaque", "opaque", "opaque"]
                    elif index == 1:
                        cells = [str(row), f"`synthetic_check_{row}`", "opaque observation", "opaque requirement", "**FAIL**"]
                    else:
                        cells = [f"`synthetic_evaluator_{row}`", "fixture", "fixture", "**`FAIL`**", "fixture"]
                    table.append("| " + " | ".join(cells) + " |")
                tables.append("\n".join(table))
            self.cards[key] = f"# Master CAD Scorecard — `{label}`\n\n- **Verdict**: **FAIL (Gate Violation)**\n\n" + "\n\n".join(tables) + "\n"

    def payloads(self):
        files = {"leaderboard.json": json.dumps(self.board, sort_keys=True).encode()}
        files.update({key: text.encode() for key, text in self.cards.items()})
        pins = {key: (hashlib.sha256(data).hexdigest(), len(data)) for key, data in files.items()}
        return files, pins

    def test_synthetic_layouts_and_reference_accounting(self):
        files, pins = self.payloads()
        result = phase15_audit.audit_payloads("w319", files, pins)
        self.assertEqual(result["verified_files"], 5)
        self.assertEqual(set(result["scorecards"].values()), set(self.cards))
        label = self.board["designs"][0]["label"]
        self.assertEqual(phase15_audit.check_layout(next(iter(self.cards.values())), label), [8, 16, 18])
        self.assertNotIn("margins", result)
        self.assertNotIn("gate_s_pass_rate", result)

    def test_all_pinned_files_reject_byte_tampering(self):
        files, pins = self.payloads()
        for key in files:
            with self.subTest(key=key), self.assertRaisesRegex(ValueError, "SHA-256/size mismatch"):
                phase15_audit.audit_payloads("w319", {**files, key: files[key] + b" "}, pins)

    def test_missing_required_synthetic_row_or_whole_table_fails(self):
        key = next(iter(self.cards))
        original = self.cards[key]
        row = next(line for line in original.splitlines() if "`synthetic_check_1`" in line)
        for changed in (original.replace(row + "\n", ""), original[:original.index("| # |")]):
            self.cards[key] = changed
            with self.subTest(changed=changed), self.assertRaisesRegex(ValueError, "[Tt]able"):
                phase15_audit.audit_payloads("w319", *self.payloads())

    def test_table_header_separator_column_and_ordinal_mismatches_fail(self):
        key = next(iter(self.cards))
        original = self.cards[key]
        variants = [original.replace("| Gate ID |", "| Wrong ID |"),
                    original.replace("|---|", "|bad|", 1),
                    original.replace("| 1 | `synthetic_check_1` |", "| 1 | extra | `synthetic_check_1` |"),
                    original.replace("| 1 | `synthetic_check_1` |", "| 0 | `synthetic_check_1` |")]
        for changed in variants:
            self.cards[key] = changed
            with self.subTest(changed=changed), self.assertRaises(ValueError):
                phase15_audit.audit_payloads("w319", *self.payloads())

    def test_duplicate_gate_and_evaluator_identifiers_fail(self):
        key = next(iter(self.cards))
        original = self.cards[key]
        for target in ("synthetic_check", "synthetic_evaluator"):
            self.cards[key] = original.replace(f"`{target}_2`", f"`{target}_1`")
            with self.subTest(target=target), self.assertRaisesRegex(ValueError, "Duplicate table identifiers"):
                phase15_audit.audit_payloads("w319", *self.payloads())

    def test_contradictory_or_duplicate_stored_overall_verdict_fails(self):
        key = next(iter(self.cards))
        original = self.cards[key]
        for changed in [original.replace("FAIL (Gate Violation)", "PASS"),
                        original + "\n- **Verdict**: **FAIL (Gate Violation)**\n"]:
            self.cards[key] = changed
            with self.subTest(changed=changed), self.assertRaisesRegex(ValueError, "Stored overall verdict"):
                phase15_audit.audit_payloads("w319", *self.payloads())

    def test_missing_extra_files_and_malformed_pins_fail(self):
        files, pins = self.payloads()
        changed = dict(files)
        del changed[next(iter(self.cards))]
        with self.assertRaisesRegex(ValueError, "Exactly five"):
            phase15_audit.audit_payloads("w319", changed, pins)
        with self.assertRaisesRegex(ValueError, "Exactly five"):
            phase15_audit.audit_payloads("w319", {**files, "extra": b""}, pins)
        for pin in [("Z" * 64, 0), ("a" * 64, True), ("a" * 64, -1)]:
            with self.subTest(pin=pin), self.assertRaises(ValueError):
                phase15_audit.audit_payloads("w319", files, {**pins, "leaderboard.json": pin})

    def test_stored_qualification_claims_and_counts_fail_closed(self):
        for key, value in [("qualified_count", False), ("qualified_count", 1), ("assessed_count", True),
                           ("assessed_count", 3), ("champion", "fixture"), ("unverified", ["fixture"])]:
            original = self.board[key]
            self.board[key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                phase15_audit.audit_payloads("w319", *self.payloads())
            self.board[key] = original
        self.board["designs"][0]["qualified"] = True
        with self.assertRaisesRegex(ValueError, "boolean false"):
            phase15_audit.audit_payloads("w319", *self.payloads())

    def test_path_identity_title_and_unreferenced_attempt_mismatches_fail(self):
        design = self.board["designs"][0]
        original = design["scorecard_md"]
        for reference in ["../" + original, original.replace("out/w319/", "out/w318/"),
                          original.replace(design["certificate_identity"], "a" * 64),
                          original.replace("attempt-0", "attempt-99")]:
            design["scorecard_md"] = reference
            with self.subTest(reference=reference), self.assertRaises(ValueError):
                phase15_audit.audit_payloads("w319", *self.payloads())
        design["scorecard_md"] = original
        key = next(iter(self.cards))
        self.cards[key] = self.cards[key].replace("# Master CAD Scorecard", "# Wrong Title", 1)
        with self.assertRaisesRegex(ValueError, "title/label"):
            phase15_audit.audit_payloads("w319", *self.payloads())

    def test_relocated_cli_without_git_or_original_host_files(self):
        files, pins = self.payloads()
        directory, reads, output = Path("/portable/fixture"), [], io.StringIO()

        def read(path):
            key = str(path.relative_to(directory))
            reads.append(key)
            return files[key]

        with patch.object(Path, "read_bytes", new=read), patch.object(phase15_audit, "PINS", {"w319": pins}), \
                patch.object(wave_audit, "default_wave_directory", side_effect=AssertionError("Unexpected Git access")), \
                patch.object(sys, "argv", ["audit_phase15_gate_s_and_margins.py", "--wave-dir", str(directory)]), \
                redirect_stdout(output):
            phase15_audit.main()
        self.assertEqual(len(reads), 10)
        self.assertEqual(set(reads), set(files))
        self.assertIn("artifact subset: PASS; 5 files", output.getvalue())
        self.assertIn("NOT ASSESSED", output.getvalue())

    def test_mutating_snapshot_and_unsafe_pin_paths_fail(self):
        files, pins = self.payloads()
        directory, reads = Path("/portable/fixture"), {}

        def read(path):
            key = str(path.relative_to(directory))
            reads[key] = reads.get(key, 0) + 1
            return files[key] + (b" " if reads[key] > 1 else b"")

        with patch.object(Path, "read_bytes", new=read), self.assertRaisesRegex(ValueError, "changed during audit"):
            phase15_audit.audit_directories({"w319": directory}, {"w319": pins})
        with patch.object(Path, "read_bytes") as read_mock, self.assertRaisesRegex(ValueError, "Invalid pinned artifact path"):
            phase15_audit.audit_directories({"w319": directory}, {"w319": {"../unsafe": ("a" * 64, 0)}})
        read_mock.assert_not_called()


class Phase16Xbabe3WaveAuditTests(unittest.TestCase):
    def setUp(self):
        self.waves = {}
        for wave in xbabe3_audit.WAVES:
            documents, designs, errors = {}, [], []
            for round_id in range(3):
                births, screens = [], []
                for slot in (1, 2):
                    identity = hashlib.sha256(f"{wave}-{round_id}-{slot}".encode()).hexdigest()
                    label = f"{wave}_r{round_id + 1:02}_fixture_c{slot:02}"
                    births.append({"family": "fixture", "certificate_identity": identity, "row": f"{round_id},{slot}"})
                    screens.append({**dict.fromkeys(wave_audit.SCREEN_NUMBERS, 0.0), "family": "fixture",
                                    "certificate_identity": identity, "label": label, "qualification_status": wave_audit.PENDING})
                    if wave == "w407c3" and round_id == 1 and slot == 2:
                        errors.append({"label": label, "certificate_identity": identity, "status": "unverified", "reason": "synthetic error"})
                    else:
                        designs.append({**dict.fromkeys(wave_audit.DESIGN_NUMBERS, 0.0), "label": label, "family": "fixture",
                                        "certificate_identity": identity, "qualified": False, "all_16_gates": False,
                                        "rs_v2_all_hard": False, "battery_pass_count": 0, "genome_csv": "fixture.csv", "scorecard_md": "fixture.md"})
                documents[f"round_{round_id:02}_births.json"] = {"round": round_id, "seed": 1, "source": "synthetic",
                    "genome_header": "a,b", "genomes": births}
                documents[f"round_{round_id:02}_screen.json"] = screens
            documents["leaderboard.json"] = {"schema": "jarambit.auto_evolve.leaderboard.v2", "designs": designs,
                "assessed_count": len(designs), "qualified_count": 0, "champion": None, "unverified": errors}
            documents["leaderboard.md"] = "# Evolution certification attempts\n\n| " + " | ".join(w319_audit.BOARD_HEADER) + " |\n"
            documents["leaderboard.md"] += "|" + "|".join(["---"] * 8) + "|\n"
            documents["leaderboard.md"] += "".join(f"| `{d['label']}` | false | opaque | opaque | opaque | opaque | false | opaque |\n" for d in designs)
            result = f"# {wave} RESULT synthetic fixture\n\nSource `leaderboard.json` (schema jarambit.auto_evolve.leaderboard.v2): assessed {len(designs)}, qualified 0, unverified {len(errors)}\n\n"
            result += "| " + " | ".join(xbabe3_audit.RESULT_HEADER) + " |\n|" + "|".join(["---"] * 13) + "|\n"
            result += "".join("| " + " | ".join([d["label"], "False"] + ["opaque"] * 8 + ["False", "False", "opaque"]) + " |\n" for d in designs)
            result += "\n" + "\n".join(f"- unverified: {e['label']}: {e['reason']}" for e in errors)
            result += f"\n\n**Summary: assessed {len(designs)}, qualified 0; fixture metadata**\n"
            documents["RESULT.md"] = result
            self.waves[wave] = documents

    def payloads(self):
        files = {wave: {name: (json.dumps(doc, sort_keys=True).encode() if name.endswith(".json") else doc.encode())
                        for name, doc in documents.items()} for wave, documents in self.waves.items()}
        pins = {wave: {name: (hashlib.sha256(data).hexdigest(), len(data)) for name, data in wave_files.items()}
                for wave, wave_files in files.items()}
        return files, pins

    def test_41_assessments_and_one_separate_unverified_attempt(self):
        result = xbabe3_audit.audit_payloads(*self.payloads())
        self.assertEqual(result["totals"], {"births": 42, "screens": 42, "assessed": 41, "qualified": 0, "unverified": 1})
        self.assertEqual(result["verified_files"], 63)
        self.assertEqual(result["standalone"], ["w405c3"])
        self.assertEqual(len(result["pairs"]), 3)
        self.assertEqual(result["waves"]["w407c3"]["assessed"], 5)
        self.assertNotIn("structural_comparison", result)

    def test_all_three_rounds_are_accounted_for(self):
        counts = xbabe3_audit.audit_documents("w406c3", self.waves["w406c3"])
        self.assertEqual(counts["rounds"], {str(r): {"births": 2, "screens": 2} for r in range(3)})
        del self.waves["w406c3"]["round_02_screen.json"]
        with self.assertRaisesRegex(ValueError, "Exactly nine"):
            xbabe3_audit.audit_documents("w406c3", self.waves["w406c3"])

    def test_each_of_63_artifacts_rejects_byte_tampering(self):
        files, pins = self.payloads()
        for wave, wave_files in files.items():
            for name, original in list(wave_files.items()):
                wave_files[name] = original + b" "
                with self.subTest(wave=wave, name=name), self.assertRaisesRegex(ValueError, "SHA-256/size mismatch"):
                    xbabe3_audit.audit_payloads(files, pins)
                wave_files[name] = original

    def test_missing_wave_file_and_extra_round_fail(self):
        files, pins = self.payloads()
        changed = dict(files)
        del changed["w408t3"]
        with self.assertRaisesRegex(ValueError, "Exactly seven"):
            xbabe3_audit.audit_payloads(changed, pins)
        files["w408t3"].pop("RESULT.md")
        with self.assertRaisesRegex(ValueError, "Exactly nine"):
            xbabe3_audit.audit_payloads(files, pins)
        files, pins = self.payloads()
        files["w408t3"]["round_03_screen.json"] = b"[]"
        with self.assertRaisesRegex(ValueError, "Exactly nine"):
            xbabe3_audit.audit_payloads(files, pins)

    def test_malformed_digest_and_size_pins_fail(self):
        files, pins = self.payloads()
        for pin in [("A" * 64, 0), ("a" * 64, True), ("a" * 64, -1)]:
            pins["w408t3"]["RESULT.md"] = pin
            with self.subTest(pin=pin), self.assertRaises(ValueError):
                xbabe3_audit.audit_payloads(files, pins)

    def test_screen_must_match_exact_birth_position(self):
        docs = self.waves["w406c3"]
        docs["round_00_screen.json"][0]["certificate_identity"] = docs["round_00_births.json"]["genomes"][1]["certificate_identity"]
        with self.assertRaisesRegex(ValueError, "birth slot mismatch"):
            xbabe3_audit.audit_payloads(*self.payloads())

    def test_assessed_and_screen_identity_must_match(self):
        self.waves["w406c3"]["leaderboard.json"]["designs"][0]["certificate_identity"] = "a" * 64
        with self.assertRaisesRegex(ValueError, "identity/screen mismatch"):
            xbabe3_audit.audit_payloads(*self.payloads())

    def test_zero_qualification_champion_and_boolean_counts_fail_closed(self):
        board = self.waves["w406c3"]["leaderboard.json"]
        for key, value in [("qualified_count", False), ("qualified_count", 1), ("champion", {}), ("assessed_count", True), ("assessed_count", 5)]:
            original = board[key]
            board[key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                xbabe3_audit.audit_payloads(*self.payloads())
            board[key] = original
        board["designs"][0]["qualified"] = True
        with self.assertRaisesRegex(ValueError, "boolean false"):
            xbabe3_audit.audit_payloads(*self.payloads())

    def test_unverified_attempt_cannot_be_lost_or_mislinked(self):
        board = self.waves["w407c3"]["leaderboard.json"]
        error = board["unverified"][0]
        board["unverified"] = []
        with self.assertRaisesRegex(ValueError, "Unverified attempt count"):
            xbabe3_audit.audit_payloads(*self.payloads())
        board["unverified"] = [error]
        for key, value in [("certificate_identity", "a" * 64), ("status", "qualified"), ("reason", "")]:
            original = error[key]
            error[key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                xbabe3_audit.audit_payloads(*self.payloads())
            error[key] = original

    def test_both_markdown_tables_reject_flags_labels_and_missing_rows(self):
        docs = self.waves["w406c3"]
        for name in ("leaderboard.md", "RESULT.md"):
            original = docs[name]
            label = docs["leaderboard.json"]["designs"][0]["label"]
            first_row = next(line for line in original.splitlines() if label in line and line.startswith("|"))
            for changed in [original.replace(label, "wrong_label", 1), original.replace(first_row + "\n", ""),
                            original.replace("| false |", "| true |", 1) if name == "leaderboard.md" else original.replace("| False |", "| True |", 1)]:
                docs[name] = changed
                with self.subTest(name=name, changed=changed), self.assertRaisesRegex(ValueError, "[Mm]arkdown"):
                    xbabe3_audit.audit_payloads(*self.payloads())
            docs[name] = original

    def test_result_source_summary_and_error_reporting_must_agree(self):
        docs = self.waves["w407c3"]
        original = docs["RESULT.md"]
        variants = [original.replace("assessed 5, qualified 0, unverified 1", "assessed 6, qualified 0, unverified 0"),
                    original.replace("**Summary: assessed 5", "**Summary: assessed 6"),
                    original.replace("- unverified:", "- hidden attempt:")]
        for changed in variants:
            docs["RESULT.md"] = changed
            with self.subTest(changed=changed), self.assertRaisesRegex(ValueError, "RESULT"):
                xbabe3_audit.audit_payloads(*self.payloads())

    def test_malformed_round_csv_and_duplicate_screen_label_fail(self):
        docs = self.waves["w406t3"]
        docs["round_00_births.json"]["genomes"][0]["row"] = "single-column"
        with self.assertRaisesRegex(ValueError, "CSV/header width"):
            xbabe3_audit.audit_payloads(*self.payloads())
        docs["round_00_births.json"]["genomes"][0]["row"] = "0,1"
        docs["round_00_screen.json"].append(deepcopy(docs["round_00_screen.json"][0]))
        with self.assertRaisesRegex(ValueError, "Duplicate screen"):
            xbabe3_audit.audit_payloads(*self.payloads())

    def test_relocated_cli_without_git_or_host_archive(self):
        files, pins = self.payloads()
        root, reads, output = Path("/portable/xbabe3/out"), [], io.StringIO()

        def read(path):
            wave, name = path.relative_to(root).parts
            reads.append((wave, name))
            return files[wave][name]

        def glob(path, pattern):
            return [path / name for name in xbabe3_audit.ROUND_NAMES if name.endswith(pattern.removeprefix("round_*"))]

        with patch.object(Path, "glob", new=glob), patch.object(Path, "read_bytes", new=read), \
                patch.object(xbabe3_audit, "PINS", pins), \
                patch.object(wave_audit, "default_wave_directory", side_effect=AssertionError("Unexpected Git discovery")), \
                patch.object(sys, "argv", ["audit_phase16_xbabe3_waves.py", "--out-root", str(root)]), redirect_stdout(output):
            xbabe3_audit.main()
        self.assertEqual(len(reads), 126)
        self.assertEqual(len(set(reads)), 63)
        self.assertIn("41 assessed rows; 1 separate unverified attempt", output.getvalue())
        self.assertIn("paired structural metrics NOT ASSESSED", output.getvalue())

    def test_inventory_and_bytes_changed_during_audit_fail(self):
        files, pins = self.payloads()
        root = Path("/portable/xbabe3/out")
        for mode in ("inventory", "bytes"):
            reads, globs = {}, []

            def read(path):
                wave, name = path.relative_to(root).parts
                reads[path] = reads.get(path, 0) + 1
                return files[wave][name] + (b" " if mode == "bytes" and reads[path] > 1 else b"")

            def glob(path, pattern):
                globs.append(path)
                names = [path / name for name in xbabe3_audit.ROUND_NAMES if name.endswith(pattern.removeprefix("round_*"))]
                if mode == "inventory" and len(globs) > 14:
                    names.append(path / "round_03_births.json")
                return names

            with self.subTest(mode=mode), patch.object(Path, "glob", new=glob), patch.object(Path, "read_bytes", new=read), \
                    self.assertRaisesRegex(ValueError, "changed during audit"):
                xbabe3_audit.audit_root(root, pins)

    def test_duplicate_or_nonfinite_json_fails_with_matching_digest(self):
        files, pins = self.payloads()
        for data in (b'{"schema":1,"schema":2}', b'{"bad":NaN}', b'{"bad":1e999}'):
            files["w405c3"]["leaderboard.json"] = data
            pins["w405c3"]["leaderboard.json"] = (hashlib.sha256(data).hexdigest(), len(data))
            with self.subTest(data=data), self.assertRaises(ValueError):
                xbabe3_audit.audit_payloads(files, pins)


class Phase17PortableSchemaAuditTests(unittest.TestCase):
    def setUp(self):
        self.root = Path("/portable/checkout")
        self.artifacts = Path("/portable/saved")
        self.data = b"synthetic bytes"
        self.sha = hashlib.sha256(self.data).hexdigest()
        self.study = schema_audit.HERE + "fixture.json"
        self.raw = "target/rsv2-07/fixture.bin"
        self.manifest = {"schema": "jarambit.rsv2_07.portable_receipt_manifest.v1", "task": "RSV2-07-p10",
                         "files": {self.study: self.sha}, "raw_artifacts": {self.raw: {"sha256": self.sha, "size_bytes": len(self.data)}},
                         "digest_reference_counts": {"fixture.json": 1}}
        self.sources = {phase: {"w319": {"fixture.bin": (self.sha, len(self.data))}} for phase in range(11, 17)}
        self.sources[11] = {"fixture.bin": (self.sha, len(self.data))}
        self.sources[12] = {"w315": {"fixture.bin": (self.sha, len(self.data))}}
        self.sources[14] = {"fixture.bin": (self.sha, len(self.data))}
        self.disk = {self.root / self.study: self.data, self.root / self.raw: self.data}
        for module, _ in schema_audit.MODULES.values():
            self.disk[self.root / schema_audit.HERE / (module + ".py")] = b"# synthetic registry module\n"
        for phase, registry in self.sources.items():
            for name, _, _ in schema_audit.pin_entries(phase, registry):
                self.disk[self.artifacts / name] = self.data

    def read(self, path):
        if path == self.root / schema_audit.MANIFEST:
            return json.dumps(self.manifest).encode()
        if path not in self.disk:
            raise FileNotFoundError(str(path))
        return self.disk[path]

    def current(self):
        with patch.object(Path, "read_bytes", new=lambda path: self.read(path)):
            return schema_audit.audit(self.root, self.artifacts, self.sources)

    def test_seven_profiles_deduplicate_matching_references(self):
        document = self.current()
        self.assertEqual([p["phase"] for p in document["phase_profiles"]], list(range(10, 17)))
        self.assertEqual(document["verification"]["references"], 8)
        self.assertEqual(document["verification"]["unique_paths"], 6)
        self.assertEqual(document["verification"]["on_disk"], 6)
        self.assertEqual(document["verification"]["manifest_only"], 0)
        self.assertEqual(len(document["source_sha256"]), 7)
        schema_audit.check_receipt(document, document)

    def test_portable_paths_reject_host_paths_and_noncanonical_forms(self):
        for name in ("", ".", "/home/fixture", "../fixture", "a/../b", "a//b", "a/./b", "C:/fixture", "a\\b", "a\nb", True):
            with self.subTest(name=name), self.assertRaises(ValueError):
                schema_audit.portable_path(name)
        self.assertEqual(schema_audit.portable_path("evidence/a-b/file.json"), "evidence/a-b/file.json")

    def test_manifest_identity_fields_and_count_types_fail_closed(self):
        for key, value in [("schema", "wrong"), ("task", "wrong"), ("files", {}), ("raw_artifacts", []),
                           ("digest_reference_counts", {"fixture.json": True})]:
            changed = deepcopy(self.manifest)
            changed[key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                schema_audit.manifest_entries(changed)
        changed = deepcopy(self.manifest)
        changed["extra"] = 1
        with self.assertRaises(ValueError):
            schema_audit.manifest_entries(changed)

    def test_pin_shape_digest_and_size_types_fail(self):
        for value in [(self.sha,), (self.sha, None), (self.sha, True), (self.sha, -1), "bad"]:
            with self.subTest(value=value), self.assertRaises(ValueError):
                schema_audit.pin_entries(11, {"fixture": value})
        for sha, size in [("A" * 64, 0), ("g" * 64, 0), (self.sha, True), (self.sha, -1)]:
            with self.subTest(sha=sha, size=size), self.assertRaises(ValueError):
                schema_audit.digest_metadata(sha, size)
        with self.assertRaisesRegex(ValueError, "Invalid wave key"):
            schema_audit.pin_entries(12, {"../wave": {"fixture": (self.sha, 0)}})

    def test_duplicate_and_conflicting_registry_entries_fail(self):
        groups = {phase: [("same.bin", self.sha, len(self.data))] for phase in range(10, 17)}
        for entries in [[("same.bin", self.sha, len(self.data))] * 2,
                        [("same.bin", "a" * 64, len(self.data))], [("same.bin", self.sha, 999)]]:
            groups[16] = entries
            with self.subTest(entries=entries), self.assertRaises(ValueError):
                schema_audit.merge_entries(groups)
        groups.pop(16)
        with self.assertRaisesRegex(ValueError, "Exactly phases"):
            schema_audit.merge_entries(groups)

    def test_missing_raw_and_wave_bytes_are_manifest_only(self):
        self.disk.pop(self.root / self.raw)
        for name in list(self.disk):
            if name.is_relative_to(self.artifacts):
                self.disk.pop(name)
        result = self.current()["verification"]
        self.assertEqual((result["on_disk"], result["manifest_only"]), (1, 5))

    def test_missing_study_input_never_becomes_manifest_only(self):
        self.disk.pop(self.root / self.study)
        self.disk[self.artifacts / self.study] = self.data
        with self.assertRaisesRegex(ValueError, "Missing study input"):
            self.current()

    def test_tampered_copy_is_not_masked_by_matching_supplement(self):
        self.disk[self.root / self.raw] = b"tampered"
        self.disk[self.artifacts / self.raw] = self.data
        with self.assertRaisesRegex(ValueError, "SHA-256 mismatch"):
            self.current()

    def test_size_mismatch_and_malformed_raw_metadata_fail(self):
        self.manifest["raw_artifacts"][self.raw]["size_bytes"] = len(self.data) + 1
        with self.assertRaisesRegex(ValueError, "Byte size mismatch"):
            self.current()
        self.manifest["raw_artifacts"][self.raw]["size_bytes"] = True
        with self.assertRaisesRegex(ValueError, "Invalid raw artifact record"):
            self.current()

    def test_saved_receipt_detects_changed_registry_and_boolean_sizes(self):
        current = self.current()
        changed = deepcopy(current)
        changed["source_sha256"][schema_audit.MANIFEST] = "a" * 64
        with self.assertRaisesRegex(ValueError, "source_sha256"):
            schema_audit.check_receipt(changed, current)
        changed = deepcopy(current)
        changed["registry"][0]["size_bytes"] = True
        with self.assertRaisesRegex(ValueError, "registry"):
            schema_audit.check_receipt(changed, current)

    def test_saved_observations_reject_false_counts_and_modes(self):
        current = self.current()
        for key, value in [("on_disk", True), ("unique_paths", 999), ("manifest_only", -1)]:
            changed = deepcopy(current)
            changed["verification"][key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                schema_audit.check_receipt(changed, current)
        changed = deepcopy(current)
        changed["verification"]["observations"][0]["mode"] = "manifest_only"
        with self.assertRaisesRegex(ValueError, "observation"):
            schema_audit.check_receipt(changed, current)

    def test_historical_observations_can_differ_from_clean_checkout(self):
        saved = self.current()
        self.disk.pop(self.root / self.raw)
        current = self.current()
        self.assertNotEqual(saved["verification"]["on_disk"], current["verification"]["on_disk"])
        schema_audit.check_receipt(saved, current)

    def test_relocated_read_only_cli_uses_no_git_or_host_paths(self):
        saved = self.current()
        self.disk[self.root / schema_audit.RECEIPT] = json.dumps(saved).encode()
        output = io.StringIO()
        args = ["audit_phase17_portable_schema.py", "--repo-root", str(self.root), "--artifact-root", str(self.artifacts)]
        with patch.object(Path, "read_bytes", new=lambda path: self.read(path)), patch.object(Path, "write_text") as write, \
                patch.object(schema_audit, "registry_sources", return_value=self.sources), \
                patch.object(portable, "git", side_effect=AssertionError("Unexpected Git access")), \
                patch.object(sys, "argv", args), redirect_stdout(output):
            schema_audit.main()
        write.assert_not_called()
        self.assertIn("7 phases; 8 references; 6 unique paths; 6 on-disk", output.getvalue())
        self.assertNotIn("/portable", json.dumps(saved))

    def test_duplicate_nonfinite_json_and_input_read_only_behavior(self):
        before = deepcopy(self.disk)
        self.current()
        self.assertEqual(before, self.disk)
        for data in (b'{"schema":1,"schema":2}', b'{"count":NaN}', b'{"count":1e999}'):
            with self.subTest(data=data), self.assertRaises(ValueError):
                receipt_audit.load_json(data)


if __name__ == "__main__":
    unittest.main(verbosity=2)
