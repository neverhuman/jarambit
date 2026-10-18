"""Historical inventory regressions use opaque synthetic identity records."""

from contextlib import redirect_stderr, redirect_stdout
from copy import deepcopy
import hashlib
import io
import json
from pathlib import Path
import sys
import unittest
from unittest.mock import patch

import audit_phase20_stage2_identity_dedup as phase20
import verify_rsv2_07_receipts as verifier
import portable_audit as portable


class Round0InventoryTests(unittest.TestCase):
    def setUp(self):
        self.root, self.artifacts = Path("/relocated/checkout"), Path("/relocated/artifacts")
        self.source = b"opaque immutable source fixture"
        self.documents = {}
        for wave in phase20.WAVES:
            designs, errors = [], []
            for round_id in (0, 1):
                identities = sorted(verifier.REFERENCE_IDENTITIES) if round_id == 0 else [
                    hashlib.sha256(wave.encode()).hexdigest(), min(verifier.REFERENCE_IDENTITIES)]
                births, screens = [], []
                for slot, identity in enumerate(identities, 1):
                    label = f"{wave}_r{round_id + 1:02}_fixture_c{slot:02}"
                    births.append({"family": "fixture", "certificate_identity": identity, "row": identity + ",opaque"})
                    screens.append({"label": label, "family": "fixture", "certificate_identity": identity,
                                    "qualification_status": "pending independent top-k assessment"})
                    directory = f"out/{wave}/by-content/{identity}/attempt-{round_id + 1}{slot}"
                    if wave == "w320" and round_id == 1 and slot == 2:
                        errors.append({"label": label, "certificate_identity": identity, "status": "unverified",
                                       "reason": "synthetic failure; raw artifacts retained in " + directory})
                    else:
                        designs.append({"label": label, "family": "fixture", "certificate_identity": identity,
                                        "genome_csv": directory + "/" + label + "_genome.csv",
                                        "scorecard_md": directory + "/" + label + "_master_scorecard.md", "qualified": False})
                self.documents[f"out/{wave}/round_{round_id:02}_births.json"] = {
                    "round": round_id, "seed": 1, "source": "synthetic", "genome_header": "identity,marker", "genomes": births}
                self.documents[f"out/{wave}/round_{round_id:02}_screen.json"] = screens
            self.documents[f"out/{wave}/leaderboard.json"] = {
                "schema": "jarambit.auto_evolve.leaderboard.v2", "designs": designs,
                "assessed_count": len(designs), "qualified_count": 0, "champion": None, "unverified": errors}

    def payloads(self):
        files = {name: json.dumps(doc).encode() for name, doc in self.documents.items()}
        pins = {wave: {name: tuple(verifier.metadata(files[f"out/{wave}/{name}"]).values()) for name in phase20.ROOT_NAMES}
                for wave in phase20.WAVES}
        prior = {"artifacts": {phase20.ROOT_DIRECTORIES[wave] + "/" + name: verifier.metadata(files[f"out/{wave}/{name}"])
                               for wave in phase20.WAVES for name in phase20.ROOT_NAMES}}
        prior_bytes = json.dumps(prior).encode()
        return files, pins, prior_bytes

    def current(self, saved_files=None, saved_pins=None, prior_bytes=None, read=None):
        files, pins, prior = self.payloads()
        files = files if saved_files is None else saved_files
        pins = pins if saved_pins is None else saved_pins
        prior = prior if prior_bytes is None else prior_bytes
        def default_read(path):
            if path == self.root / phase20.RECEIPT:
                return prior
            name = str(path.relative_to(self.artifacts))
            if name not in files:
                raise FileNotFoundError(name)
            return files[name]
        with patch.object(Path, "read_bytes", new=default_read if read is None else read), \
                patch.object(phase20, "ROOT_PINS", pins), \
                patch.object(verifier, "SOURCE_PIN", tuple(verifier.metadata(self.source).values())), \
                patch.object(verifier, "PHASE20_PIN", tuple(verifier.metadata(prior).values())):
            return verifier.audit(self.root, self.artifacts, self.source)

    def test_inventory_counts_occurrences_without_claiming_novelty(self):
        rows = [{"family": "fixture", "certificate_identity": "a" * 64}] * 2 + [
            {"family": "other", "certificate_identity": "b" * 64}]
        self.assertEqual(verifier.inventory(rows, {"a" * 64}), {
            "entries": 3, "distinct_identities": 2, "reference_matching_entries": 2, "other_entries": 1,
            "families": {"fixture": {"entries": 2, "distinct_identities": 1, "reference_matching_entries": 2, "other_entries": 0},
                         "other": {"entries": 1, "distinct_identities": 1, "reference_matching_entries": 0, "other_entries": 1}}})
        self.assertEqual(verifier.inventory([], set())["entries"], 0)

    def test_counts_and_unknown_seed_provenance_are_explicit(self):
        result = self.current()
        self.assertEqual(result["status"], "partial_seed_input_provenance_missing")
        self.assertEqual(result["reference_inventory_totals"]["all_rounds_assessed"]["entries"], 23)
        self.assertEqual(result["reference_inventory_totals"]["round0_assessed"]["entries"], 12)
        self.assertEqual(result["unverified_events"], 1)
        for row in result["waves"].values():
            self.assertIsNone(row["seed_vs_mutated_counts"])
            self.assertEqual(row["qualified_count"], 0)
        self.assertNotIn("/relocated", json.dumps(result))
        self.assertEqual(self.current(), result)

    def test_later_round_repeated_identity_is_not_a_round0_event(self):
        row = self.current()["waves"]["w315"]["repeated_identity_reference_inventory"]
        self.assertEqual(row["round0_assessed"]["entries"], 2)
        self.assertEqual(row["later_round_assessed"]["entries"], 2)
        self.assertEqual(row["later_round_assessed"]["reference_matching_entries"], 1)

    def test_unverified_record_stays_out_of_assessed_totals(self):
        row = self.current()["waves"]["w320"]
        self.assertEqual(row["unverified_events"], 1)
        self.assertEqual(row["round0_unverified_events"], 0)
        self.assertEqual(row["repeated_identity_reference_inventory"]["all_rounds_assessed"]["entries"], 3)

    def test_root_bytes_and_sizes_are_pinned(self):
        files, pins, prior = self.payloads()
        name = "out/w315/round_00_births.json"
        for data in (files[name] + b" ", b"{}"):
            with self.subTest(data=data[:20]), self.assertRaisesRegex(ValueError, "SHA-256/size mismatch"):
                self.current({**files, name: data}, pins)
        with self.assertRaises(FileNotFoundError):
            self.current({name: data for name, data in files.items() if name != "out/w315/leaderboard.json"})

    def test_prior_receipt_binding_is_checked_even_when_wave_pin_is_refreshed(self):
        _, _, prior = self.payloads()
        self.documents["out/w315/round_00_births.json"]["source"] = "changed fixture"
        with self.assertRaisesRegex(ValueError, "Phase-20 root binding mismatch"):
            self.current(prior_bytes=prior)

    def test_source_and_receipt_pin_tampering_fail(self):
        for data, pin in ((self.source + b" ", tuple(verifier.metadata(self.source).values())),
                          (b"{}", verifier.PHASE20_PIN)):
            with self.subTest(pin=pin), self.assertRaisesRegex(ValueError, "SHA-256/size mismatch"):
                verifier.pinned(data, pin, "fixture")

    def test_boolean_counts_and_qualification_claims_fail_after_resigning(self):
        board = self.documents["out/w315/leaderboard.json"]
        board["assessed_count"] = True
        with self.assertRaisesRegex(ValueError, "Assessed event count mismatch"):
            self.current()
        board["assessed_count"] = 4
        board["qualified_count"] = 1
        with self.assertRaisesRegex(ValueError, "qualification count/champion"):
            self.current()

    def test_screen_identity_must_link_to_same_round_birth_slot(self):
        self.documents["out/w315/round_00_screen.json"][0]["certificate_identity"] = "0" * 64
        with self.assertRaisesRegex(ValueError, "Screen identity/birth slot mismatch"):
            self.current()

    def test_digest_syntax_is_required_for_rows_and_reference_set(self):
        for rows, references in (([{"family": "fixture", "certificate_identity": "bad"}], set()), ([], {"A" * 64})):
            with self.subTest(rows=rows), self.assertRaises(ValueError):
                verifier.inventory(rows, references)

    def test_strict_json_duplicate_keys_and_nonfinite_numbers_fail(self):
        files, pins, prior = self.payloads()
        name = "out/w315/round_00_screen.json"
        for data in (b'{"a":1,"a":2}', b'{"a":NaN}', b'{"a":1e999}'):
            altered = {**files, name: data}
            pins["w315"]["round_00_screen.json"] = tuple(verifier.metadata(data).values())
            changed_prior = json.loads(prior)
            changed_prior["artifacts"][phase20.ROOT_DIRECTORIES["w315"] + "/round_00_screen.json"] = verifier.metadata(data)
            with self.subTest(data=data), self.assertRaises(ValueError):
                self.current(altered, pins, json.dumps(changed_prior).encode())

    def test_wave_mutation_during_audit_fails(self):
        files, pins, prior = self.payloads()
        reads = {}
        def read(path):
            if path == self.root / phase20.RECEIPT:
                return prior
            reads[path] = reads.get(path, 0) + 1
            return files[str(path.relative_to(self.artifacts))] + (b" " if reads[path] > 1 else b"")
        with self.assertRaisesRegex(ValueError, "Wave input changed"):
            self.current(read=read)

    def test_relocated_cli_needs_no_git_is_readonly_and_rejects_changed_receipt(self):
        current = self.current()
        saved = deepcopy(current)
        blob = Path("/relocated/source-blob")
        args = ["verify_rsv2_07_receipts.py", "--repo-root", str(self.root), "--artifact-root", str(self.artifacts),
                "--source-blob", str(blob)]
        with patch.object(sys, "argv", args), \
                patch.object(Path, "read_bytes", new=lambda path: self.source if path == blob else json.dumps(saved).encode()), \
                patch.object(verifier, "audit", return_value=current) as audit, patch.object(Path, "write_text") as write, \
                patch.object(verifier.subprocess, "check_output", side_effect=AssertionError("Unexpected Git discovery")), \
                redirect_stdout(io.StringIO()) as output:
            verifier.main()
            audit.assert_called_once_with(self.root, self.artifacts, self.source)
            write.assert_not_called()
            self.assertIn("classification PARTIAL", output.getvalue())
            saved["qualified_count"] = 1
            with redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
                verifier.main()
            self.assertEqual(error.exception.code, 1)


class W320SchemaProvenanceTests(unittest.TestCase):
    def setUp(self):
        # Share only the opaque identifier/linkage fixture; no saved metrics are used.
        fixture = Round0InventoryTests()
        fixture.setUp()
        self.root, self.artifacts = fixture.root, fixture.artifacts
        self.documents = {Path(name).name: deepcopy(doc) for name, doc in fixture.documents.items()
                          if name.startswith("out/w320/")}
        for r in (0, 1):
            for row in self.documents[f"round_{r:02}_screen.json"]:
                row.update(dict.fromkeys(verifier.receipts.SCREEN_NUMBERS, 0.0))
        for row in self.documents["leaderboard.json"]["designs"]:
            row.update(dict.fromkeys(verifier.receipts.DESIGN_NUMBERS, 0.0))
            row.update(mass_g=12.375, w_cad=0.123456789, g1=0.234567891, solid_tip_sf=0.3456789,
                       rs_v2_all_hard=False, all_16_gates=False, battery_pass_count=7)
        lines = ["# Evolution certification attempts", "",
                 "| Design | Qualified | Mass g | W_CAD | G1 | Solid tip SF | RS-v2 all_hard | Battery |",
                 "|---|---|---:|---:|---:|---:|---|---|"]
        lines.extend(f"| `{row['label']}` | false | 12.375 | 0.123457 | 0.234568 | 0.3457 | false | 7/18 |"
                     for row in self.documents["leaderboard.json"]["designs"])
        self.markdown = "\n".join(lines) + "\n"
        self.prior = {"waves": {"w320": verifier.wave_inventory("w320", self.documents)}}

    def payloads(self):
        files = {name: json.dumps(doc).encode() for name, doc in self.documents.items()}
        files["leaderboard.md"] = self.markdown.encode()
        return files

    def pins(self, files):
        return {name: tuple(verifier.metadata(data).values()) for name, data in files.items()}

    def current(self, files=None, pins=None, prior=None, read=None):
        files = self.payloads() if files is None else files
        pins = self.pins(files) if pins is None else pins
        prior = json.dumps(self.prior).encode() if prior is None else prior
        def default_read(path):
            if path == self.root / verifier.RECEIPT:
                return prior
            name = str(path.relative_to(self.artifacts / "out/w320"))
            if name not in files:
                raise FileNotFoundError(name)
            return files[name]
        with patch.object(Path, "read_bytes", new=default_read if read is None else read), \
                patch.object(verifier, "PHASE21_PIN", tuple(verifier.metadata(prior).values())):
            return verifier.audit_phase22(self.root, self.artifacts, pins)

    def test_six_file_schema_linkage_and_transcription(self):
        result = self.current()
        self.assertEqual(result["status"], "pass_saved_schema_and_provenance")
        self.assertEqual(len(result["artifacts"]), 6)
        self.assertEqual(result["rounds"], {"0": {"births": 2, "pending_screens": 2}, "1": {"births": 2, "pending_screens": 2}})
        board = result["leaderboard"]
        self.assertEqual((board["assessed_count"], board["qualified_count"], board["unverified_count"]), (3, 0, 1))
        self.assertIsNone(board["champion"])
        self.assertEqual([row["saved_round"] for row in board["assessed"]], [0, 0, 1])
        self.assertEqual(result["markdown"]["assessed_rows"], 3)
        self.assertEqual(result["markdown"]["unverified_representation"], "not_listed_in_assessed_markdown_table")
        self.assertEqual(result["phase21_receipt"]["seed_lineage_status"], "partial_seed_input_provenance_missing")
        self.assertNotIn("/relocated", json.dumps(result))
        for name in result["artifacts"]:
            verifier.schema.portable_path(name)
        self.assertEqual(self.current(), result)

    def test_every_source_file_rejects_byte_tampering(self):
        files = self.payloads()
        pins = self.pins(files)
        for name in files:
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, "SHA-256/size mismatch"):
                verifier.audit_phase22_payloads({**files, name: files[name] + b" "}, pins)

    def test_missing_extra_files_and_invalid_pin_metadata_fail(self):
        files = self.payloads()
        pins = self.pins(files)
        for changed in ({name: data for name, data in files.items() if name != "leaderboard.md"}, {**files, "extra.json": b"{}"}):
            with self.subTest(names=list(changed)), self.assertRaisesRegex(ValueError, "Exactly six"):
                verifier.audit_phase22_payloads(changed, pins)
        for pin in (("A" * 64, 0), ("a" * 64, True), ("a" * 64, -1), ("bad", 0)):
            with self.subTest(pin=pin), self.assertRaises(ValueError):
                verifier.audit_phase22_payloads(files, {**pins, "leaderboard.md": pin})
        with self.assertRaises(FileNotFoundError):
            self.current({name: data for name, data in files.items() if name != "leaderboard.md"}, pins)

    def test_strict_json_duplicate_keys_nonfinite_and_overflow_fail(self):
        files = self.payloads()
        for data in (b'{"a":1,"a":2}', b'{"a":NaN}', b'{"a":Infinity}', b'{"a":1e999}'):
            changed = {**files, "round_00_screen.json": data}
            with self.subTest(data=data), self.assertRaises(ValueError):
                verifier.audit_phase22_payloads(changed, self.pins(changed))

    def test_screen_numeric_fields_require_finite_nonboolean_numbers(self):
        screen = self.documents["round_00_screen.json"][0]
        for value in (True, "0", None):
            screen["mass_g"] = value
            with self.subTest(value=value), self.assertRaisesRegex(ValueError, "Invalid numeric field"):
                files = self.payloads()
                verifier.audit_phase22_payloads(files, self.pins(files))
        del screen["mass_g"]
        files = self.payloads()
        with self.assertRaisesRegex(ValueError, "Missing schema fields"):
            verifier.audit_phase22_payloads(files, self.pins(files))

    def test_assessed_numeric_boolean_and_battery_schema_fields(self):
        design = self.documents["leaderboard.json"]["designs"][0]
        original = deepcopy(design)
        for key, value in (("mass_g", True), ("g1", "0"), ("rs_v2_all_hard", 0),
                           ("all_16_gates", "false"), ("battery_pass_count", True), ("battery_pass_count", 19)):
            design.update(original)
            design[key] = value
            files = self.payloads()
            with self.subTest(key=key, value=value), self.assertRaises(ValueError):
                verifier.audit_phase22_payloads(files, self.pins(files))

    def test_birth_provenance_seed_and_csv_shape_consistency(self):
        birth = self.documents["round_01_births.json"]
        original = deepcopy(birth)
        for key, value in (("seed", True), ("seed", -1), ("seed", 2 ** 64), ("seed", 2),
                           ("source", "different provenance"), ("genome_header", "a,a")):
            birth.update(deepcopy(original))
            birth[key] = value
            files = self.payloads()
            with self.subTest(key=key, value=value), self.assertRaises(ValueError):
                verifier.audit_phase22_payloads(files, self.pins(files))
        birth.update(deepcopy(original))
        birth["genomes"][0]["row"] = "ragged"
        files = self.payloads()
        with self.assertRaisesRegex(ValueError, "Birth CSV/header width mismatch"):
            verifier.audit_phase22_payloads(files, self.pins(files))

    def test_screen_same_round_slot_linkage_is_required(self):
        self.documents["round_00_screen.json"][0]["certificate_identity"] = "0" * 64
        files = self.payloads()
        with self.assertRaisesRegex(ValueError, "Screen identity/birth slot mismatch"):
            verifier.audit_phase22_payloads(files, self.pins(files))

    def test_qualified_count_champion_and_unverified_status_remain_explicit(self):
        board = self.documents["leaderboard.json"]
        original = deepcopy(board)
        for key, value in (("assessed_count", True), ("qualified_count", True), ("qualified_count", 1), ("champion", "fixture")):
            board.update(deepcopy(original))
            board[key] = value
            files = self.payloads()
            with self.subTest(key=key, value=value), self.assertRaises(ValueError):
                verifier.audit_phase22_payloads(files, self.pins(files))
        board.update(deepcopy(original))
        board["unverified"][0]["status"] = "qualified"
        files = self.payloads()
        with self.assertRaisesRegex(ValueError, "Unverified identity/screen mismatch"):
            verifier.audit_phase22_payloads(files, self.pins(files))

    def test_markdown_title_header_separator_and_row_order(self):
        files = self.payloads()
        lines = self.markdown.splitlines()
        variants = [self.markdown.replace("# Evolution certification attempts", "# Other", 1),
                    self.markdown.replace("| Mass g |", "| Other |", 1),
                    self.markdown.replace("|---|---|", "|bad|---|", 1),
                    "\n".join(lines[:-1]), "\n".join([*lines, lines[-1]]),
                    "\n".join(lines[:4] + [lines[5], lines[4], lines[6]])]
        for text in variants:
            changed = {**files, "leaderboard.md": text.encode()}
            with self.subTest(text=text[:25]), self.assertRaisesRegex(ValueError, "Markdown"):
                verifier.audit_phase22_payloads(changed, self.pins(changed))

    def test_all_displayed_saved_values_and_flags_must_match_json(self):
        files = self.payloads()
        # Each value below changes one independent table column.
        for old, new in (("| false | 12.375", "| true | 12.375"), ("| false | 7/18", "| true | 7/18"),
                         ("12.375", "12.376"), ("0.123457", "0.123456"), ("0.234568", "0.234567"),
                         ("0.3457", "0.3456"), ("7/18", "8/18"), ("7/18", "7/19")):
            changed = {**files, "leaderboard.md": self.markdown.replace(old, new, 1).encode()}
            with self.subTest(old=old), self.assertRaisesRegex(ValueError, "Markdown"):
                verifier.audit_phase22_payloads(changed, self.pins(changed))

    def test_unverified_attempt_cannot_appear_in_assessed_markdown_table(self):
        label = self.documents["leaderboard.json"]["unverified"][0]["label"]
        files = self.payloads()
        changed = {**files, "leaderboard.md": (self.markdown + f"| `{label}` | false | 0 | 0 | 0 | 0 | false | 0/18 |\n").encode()}
        with self.assertRaisesRegex(ValueError, "Markdown table/schema mismatch"):
            verifier.audit_phase22_payloads(changed, self.pins(changed))

    def test_phase21_inventory_binding_cannot_be_silently_changed(self):
        self.prior["waves"]["w320"]["qualified_count"] = 1
        with self.assertRaisesRegex(ValueError, "Phase-21 w320 inventory mismatch"):
            self.current()

    def test_wave_or_prior_receipt_mutation_during_audit_fails(self):
        files = self.payloads()
        prior = json.dumps(self.prior).encode()
        for target in ("wave", "prior"):
            reads = {}
            def read(path):
                reads[path] = reads.get(path, 0) + 1
                is_prior = path == self.root / verifier.RECEIPT
                data = prior if is_prior else files[str(path.relative_to(self.artifacts / "out/w320"))]
                mutate = reads[path] > 1 and ((target == "prior") == is_prior)
                return data + (b" " if mutate else b"")
            with self.subTest(target=target), self.assertRaisesRegex(ValueError, "changed during audit"):
                self.current(read=read)

    def test_relocated_phase22_cli_is_readonly_and_needs_no_git_or_source_blob(self):
        current = self.current()
        saved = deepcopy(current)
        args = ["verify_rsv2_07_receipts.py", "--phase", "22", "--repo-root", str(self.root), "--artifact-root", str(self.artifacts)]
        with patch.object(sys, "argv", args), patch.object(verifier, "audit_phase22", return_value=current) as audit, \
                patch.object(Path, "read_bytes", new=lambda path: json.dumps(saved).encode()), patch.object(Path, "write_text") as write, \
                patch.object(verifier.receipts, "default_wave_directory", side_effect=AssertionError("Unexpected Git discovery")), \
                patch.object(verifier.subprocess, "check_output", side_effect=AssertionError("Unexpected source access")), \
                redirect_stdout(io.StringIO()) as output:
            verifier.main()
            audit.assert_called_once_with(self.root, self.artifacts)
            write.assert_not_called()
            self.assertIn("Phase-22 schema/provenance: PASS", output.getvalue())
            saved["leaderboard"]["qualified_count"] = 1
            with redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
                verifier.main()
            self.assertEqual(error.exception.code, 1)

    def test_phase22_rejects_unrelated_source_blob_before_reading_inputs(self):
        args = ["verify_rsv2_07_receipts.py", "--phase", "22", "--source-blob", "unused"]
        with patch.object(sys, "argv", args), patch.object(Path, "read_bytes") as read, \
                patch.object(verifier.receipts, "default_wave_directory") as discover, \
                redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
            verifier.main()
        self.assertEqual(error.exception.code, 1)
        read.assert_not_called()
        discover.assert_not_called()


class ReceiptManifestTests(unittest.TestCase):
    def setUp(self):
        self.root = Path("/relocated/checkout")
        self.files = {
            verifier.HERE + "PHASE15_FIXTURE.md": b"# Saved fixture\n",
            verifier.HERE + "phase17_fixture.json": b'{"schema":"synthetic","qualified_count":0}',
            verifier.HERE + "results/phase7_saved_triplet_gci_audit.json": b'{"schema":"synthetic"}',
            verifier.HERE + "results/phase8_synthetic_gci_audit.json": b'{"schema":"synthetic"}',
            verifier.HERE + "results/cad_manifest.json": b"[]",
            verifier.HERE + "GCI_A4_VERIFICATION.md": b"# Legacy alias\n",
            verifier.HERE + "results/gci_a4_validation.json": b'{"schema":"synthetic"}',
            verifier.HERE + "phase22_fixture.json": b'{"schema":"synthetic","nested":[{"qualified_count":0}]}',
        }

    def current(self, files=None, read=None):
        files = self.files if files is None else files
        def default_read(path):
            name = str(path.relative_to(self.root))
            if name not in files:
                raise FileNotFoundError(name)
            return files[name]
        with patch.object(Path, "rglob", new=lambda path, pattern: [self.root / name for name in files]), \
                patch.object(Path, "is_file", return_value=True), \
                patch.object(Path, "read_bytes", new=default_read if read is None else read):
            return verifier.audit_phase23(self.root)

    def test_catalogue_binds_all_current_files_and_documents_coverage_gaps(self):
        result = self.current()
        self.assertEqual(result["totals"]["files"], 8)
        self.assertEqual(set(result["artifacts"]), set(self.files))
        for name, row in result["artifacts"].items():
            self.assertEqual((row["sha256"], row["size_bytes"]), (hashlib.sha256(self.files[name]).hexdigest(), len(self.files[name])))
        self.assertEqual(result["phases"]["1"]["coverage"], "report_and_json")
        self.assertEqual(result["phases"]["2"]["coverage"], "no_current_dedicated_files")
        self.assertEqual(result["phases"]["15"]["coverage"], "report_only")
        self.assertEqual(result["totals"]["explicit_zero_count_fields"], 2)
        self.assertEqual(result["artifacts"][verifier.HERE + "results/cad_manifest.json"]["phase"], None)
        self.assertNotIn(str(self.root), json.dumps(result))
        self.assertEqual(self.current(), result)

    def test_missing_qualification_fields_remain_null_and_nested_fields_are_explicit(self):
        result = self.current()
        missing = result["artifacts"][verifier.HERE + "GCI_A4_VERIFICATION.md"]
        self.assertIsNone(missing["qualified_count"])
        explicit = result["artifacts"][verifier.HERE + "phase22_fixture.json"]
        self.assertEqual(explicit["explicit_qualified_count_fields"], [{"field_location": ["nested", 0, "qualified_count"], "value": 0}])
        for invalid in (True, 1, -1, "0", None):
            with self.subTest(invalid=invalid), self.assertRaisesRegex(ValueError, "integer zero"):
                verifier.qualified_count_fields({"nested": [{"qualified_count": invalid}]})

    def test_all_json_documents_are_strict_even_without_a_named_receipt_schema(self):
        name = verifier.HERE + "results/cad_manifest.json"
        for data in (b'{"a":1,"a":2}', b'{"a":NaN}', b'{"a":1e999}'):
            with self.subTest(data=data), self.assertRaises(ValueError):
                self.current({**self.files, name: data})

    def test_literal_host_prefix_policy_covers_only_requested_historical_range(self):
        name = verifier.HERE + "PHASE15_FIXTURE.md"
        with self.assertRaisesRegex(ValueError, "Literal host prefix"):
            self.current({**self.files, name: self.files[name] + verifier.HOST_PREFIX})
        older = verifier.HERE + "GCI_A4_VERIFICATION.md"
        self.assertIn(older, self.current({**self.files, older: self.files[older] + verifier.HOST_PREFIX})["artifacts"])

    def test_unsafe_paths_and_legacy_aliases_are_handled_explicitly(self):
        self.assertEqual(verifier.receipt_phase(verifier.HERE + "SOLID_TIP_3D_VERIFICATION.md"), 4)
        self.assertEqual(verifier.receipt_phase(verifier.HERE + "results/gci_a4_validation.json"), 1)
        for name in ("../escape.json", "/absolute/phase15.json", verifier.HERE + "../escape.json", "other/phase15.json"):
            with self.subTest(name=name), self.assertRaises(ValueError):
                verifier.receipt_phase(name)

    def test_control_json_metadata_does_not_change_the_report_or_create_self_hashes(self):
        current = self.current()
        changed = deepcopy(current)
        for name in verifier.INDIRECT_REPORT_RECORDS:
            changed["artifacts"][name].update(sha256="0" * 64, size_bytes=123456)
        self.assertEqual(verifier.report_phase23(current), verifier.report_phase23(changed))
        self.assertIn(verifier.PHASE23_RECEIPT, portable.CONTROL_RECEIPTS)
        self.assertNotIn(verifier.PHASE23_RECEIPT, current["artifacts"])
        self.assertNotIn(verifier.PHASE23_REPORT, current["artifacts"])
        self.assertEqual(current["hash_dependencies"]["phase23_self_hash"], "not_claimed")

    def test_new_missing_and_changed_sources_change_the_saved_catalogue(self):
        saved = self.current()
        name = verifier.HERE + "PHASE15_FIXTURE.md"
        variants = [{**self.files, name: self.files[name] + b" "},
                    {key: data for key, data in self.files.items() if key != name},
                    {**self.files, verifier.HERE + "PHASE20_ADDITIONAL.md": b"# Additional saved report\n"}]
        for files in variants:
            with self.subTest(paths=list(files)):
                self.assertNotEqual(saved, self.current(files))

    def test_source_bytes_or_inventory_mutation_fails_closed(self):
        reads = {}
        def read(path):
            reads[path] = reads.get(path, 0) + 1
            return self.files[str(path.relative_to(self.root))] + (b" " if reads[path] > 1 else b"")
        with self.assertRaisesRegex(ValueError, "Receipt bytes changed"):
            self.current(read=read)
        first = (set(self.files), {name for name in self.files if (verifier.receipt_phase(name) or 0) >= 15}
                 | {verifier.PHASE23_RECEIPT, verifier.PHASE23_REPORT})
        second = (first[0] | {verifier.HERE + "PHASE20_EXTRA.md"}, first[1])
        with patch.object(verifier, "receipt_manifest_paths", side_effect=[first, second]), \
                self.assertRaisesRegex(ValueError, "Receipt inventory changed"):
            self.current()

    def test_relocated_phase23_cli_is_readonly_and_checks_its_exact_report_without_git(self):
        current = self.current()
        saved = deepcopy(current)
        output_files = {verifier.PHASE23_REPORT: verifier.report_phase23(current).encode()}
        args = ["verify_rsv2_07_receipts.py", "--phase", "23", "--repo-root", str(self.root)]
        def read(path):
            name = str(path.relative_to(self.root))
            return json.dumps(saved).encode() if name == verifier.PHASE23_RECEIPT else output_files[name]
        with patch.object(sys, "argv", args), patch.object(verifier, "audit_phase23", return_value=current), \
                patch.object(Path, "read_bytes", new=read), patch.object(Path, "write_text") as write, \
                patch.object(verifier.receipts, "default_wave_directory", side_effect=AssertionError("Unexpected Git discovery")), \
                redirect_stdout(io.StringIO()) as output:
            verifier.main()
            self.assertIn("Phase-23 receipt manifest: PASS", output.getvalue())
            write.assert_not_called()
            output_files[verifier.PHASE23_REPORT] += b"changed"
            with redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
                verifier.main()
            self.assertEqual(error.exception.code, 1)
            output_files[verifier.PHASE23_REPORT] = verifier.report_phase23(current).encode()
            saved["qualified_count"] = 1
            with redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
                verifier.main()
            self.assertEqual(error.exception.code, 1)

    def test_zero_literal_host_prefix_across_all_phase15_through_phase23_outputs(self):
        root = Path(__file__).resolve().parents[2]
        _, paths = verifier.receipt_manifest_paths(root)
        self.assertTrue(all((root / name).is_file() for name in paths))
        verifier.zero_literal_host_prefix({name: (root / name).read_bytes() for name in paths})
        current = verifier.audit_phase23(root)
        saved = verifier.load_json((root / verifier.PHASE23_RECEIPT).read_bytes())
        self.assertEqual(current, saved)
        self.assertEqual(sorted(paths), saved["host_prefix_audit"]["paths"])
        self.assertEqual((root / verifier.PHASE23_REPORT).read_text(), verifier.report_phase23(current))


class SavedScreenOrderingTests(unittest.TestCase):
    def setUp(self):
        fixture = Round0InventoryTests()
        fixture.setUp()
        self.documents = fixture.documents
        self.root, self.artifacts = fixture.root, fixture.artifacts
        for name, document in self.documents.items():
            if name.endswith("_screen.json"):
                for index, row in enumerate(document):
                    row.update(screen_deficit=float(index), mass_g=float(10 - index))

    def payloads(self):
        return {wave: {name: json.dumps(self.documents[f"out/{wave}/{name}"]).encode() for name in phase20.ROOT_NAMES}
                for wave in phase20.WAVES}

    def pins(self, payloads):
        return {wave: {name: tuple(verifier.metadata(data).values()) for name, data in files.items()}
                for wave, files in payloads.items()}

    def current(self, read=None):
        payloads = self.payloads()
        def default_read(path):
            parts = path.relative_to(self.artifacts).parts
            return payloads[parts[1]][parts[2]]
        with patch.object(Path, "read_bytes", new=default_read if read is None else read):
            return verifier.audit_phase24(self.artifacts, self.pins(payloads))

    def test_lexicographic_secondary_order_ties_and_readonly_reproducibility(self):
        rows = [{"screen_deficit": a, "mass_g": b} for a, b in ((0, 9), (1, 1), (1, 1), (1, 2))]
        before = json.dumps(rows)
        result = verifier.screen_tuple_order(rows)
        self.assertEqual(result, {"rows": 4, "adjacent_pairs": 3, "descending_adjacent_pairs": 0,
                                  "equal_adjacent_keys": 1, "monotonic_non_decreasing": True})
        self.assertEqual(verifier.screen_tuple_order(rows), result)
        self.assertEqual(json.dumps(rows), before)
        zeros = [{"screen_deficit": -0.0, "mass_g": 0.0}, {"screen_deficit": 0.0, "mass_g": -0.0}]
        before = json.dumps(zeros)
        self.assertEqual(verifier.screen_tuple_order(zeros)["equal_adjacent_keys"], 1)
        self.assertEqual(json.dumps(zeros), before)

    def test_primary_and_secondary_descending_pairs_are_recorded_without_repair(self):
        rows = [{"screen_deficit": a, "mass_g": b} for a, b in ((1, 1), (0, 9), (0, 8))]
        before = deepcopy(rows)
        result = verifier.screen_tuple_order(rows)
        self.assertFalse(result["monotonic_non_decreasing"])
        self.assertEqual(result["descending_adjacent_pairs"], 2)
        self.assertEqual(rows, before)
        for rows in ([], [{"screen_deficit": 0, "mass_g": 0}]):
            self.assertEqual(verifier.screen_tuple_order(rows)["adjacent_pairs"], 0)
            self.assertTrue(verifier.screen_tuple_order(rows)["monotonic_non_decreasing"])

    def test_ordering_schema_rejects_nonfinite_boolean_missing_and_nonnumeric_keys(self):
        variants = [None, {}, [None], [{}], [{"screen_deficit": 0}]]
        variants.extend([{"screen_deficit": value, "mass_g": 0}] for value in (True, "0", None, float("nan"), float("inf")))
        variants.extend([{"screen_deficit": 0, "mass_g": value}] for value in (False, "0", float("-inf")))
        for rows in variants:
            with self.subTest(rows=rows), self.assertRaises(ValueError):
                verifier.screen_tuple_order(rows)

    def test_six_wave_receipt_counts_and_stored_qualification_invariants(self):
        result = self.current()
        self.assertTrue(result["ordering_pass"])
        self.assertEqual(len(result["artifacts"]), 30)
        self.assertEqual((result["totals"]["screen_arrays"], result["totals"]["rows"], result["totals"]["adjacent_pairs"]), (12, 24, 12))
        self.assertEqual((result["totals"]["assessed_count"], result["totals"]["unverified_count"]), (23, 1))
        for row in result["waves"].values():
            self.assertEqual(row["qualified_count"], 0)
            self.assertIsNone(row["champion"])
        self.assertNotIn(str(self.artifacts), json.dumps(result))
        self.assertEqual(self.current(), result)

    def test_all_thirty_pinned_files_reject_tampering(self):
        payloads = self.payloads()
        pins = self.pins(payloads)
        for wave, files in payloads.items():
            for name, data in list(files.items()):
                files[name] = data + b" "
                with self.subTest(wave=wave, name=name), self.assertRaisesRegex(ValueError, "SHA-256/size mismatch"):
                    verifier.audit_phase24_payloads(payloads, pins)
                files[name] = data

    def test_missing_wave_or_file_and_invalid_pin_size_fail(self):
        payloads = self.payloads()
        pins = self.pins(payloads)
        with self.assertRaisesRegex(ValueError, "Exactly six"):
            verifier.audit_phase24_payloads({wave: files for wave, files in payloads.items() if wave != "w315"}, pins)
        del payloads["w315"]["round_00_screen.json"]
        with self.assertRaisesRegex(ValueError, "Exactly five"):
            verifier.audit_phase24_payloads(payloads, pins)
        payloads = self.payloads()
        pins["w315"]["leaderboard.json"] = ("a" * 64, True)
        with self.assertRaisesRegex(ValueError, "Invalid pinned byte size"):
            verifier.audit_phase24_payloads(payloads, pins)

    def test_strict_json_rejects_duplicate_keys_and_nonfinite_literals_after_resigning(self):
        payloads = self.payloads()
        for data in (b'{"a":0,"a":0}', b'{"a":NaN}', b'{"a":1e999}'):
            payloads["w315"]["round_00_screen.json"] = data
            with self.subTest(data=data), self.assertRaises(ValueError):
                verifier.audit_phase24_payloads(payloads, self.pins(payloads))

    def test_stored_qualification_or_status_cannot_be_recast_as_passed(self):
        board = self.documents["out/w315/leaderboard.json"]
        original = deepcopy(board)
        for key, value in (("qualified_count", True), ("qualified_count", 1), ("champion", "fixture")):
            board.update(deepcopy(original))
            board[key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                self.current()

    def test_mutation_of_source_bytes_during_audit_fails(self):
        payloads, reads = self.payloads(), {}
        def read(path):
            reads[path] = reads.get(path, 0) + 1
            _, wave, name = path.relative_to(self.artifacts).parts
            return payloads[wave][name] + (b" " if reads[path] > 1 else b"")
        with self.assertRaisesRegex(ValueError, "Saved wave bytes changed"):
            self.current(read=read)

    def test_relocated_cli_checks_saved_receipt_and_report_and_fails_for_descending_order(self):
        current = self.current()
        self.documents["out/w315/round_00_screen.json"].reverse()
        failure = self.current()
        saved = deepcopy(current)
        args = ["verify_rsv2_07_receipts.py", "--phase", "24", "--repo-root", str(self.root), "--artifact-root", str(self.artifacts)]
        def read(path):
            return json.dumps(saved).encode() if path == self.root / verifier.PHASE24_RECEIPT else verifier.report_phase24(current).encode()
        with patch.object(sys, "argv", args), patch.object(verifier, "audit_phase24", side_effect=lambda root: current), \
                patch.object(Path, "read_bytes", new=read), patch.object(Path, "write_text") as write, \
                patch.object(verifier.receipts, "default_wave_directory", side_effect=AssertionError("Unexpected Git discovery")), \
                redirect_stdout(io.StringIO()) as output:
            verifier.main()
            self.assertIn("Phase-24 saved screen ordering: PASS", output.getvalue())
            write.assert_not_called()
            saved["qualified_count"] = 1
            with redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
                verifier.main()
            self.assertEqual(error.exception.code, 1)
            current = failure
            saved = deepcopy(current)
            self.assertFalse(current["ordering_pass"])
            self.assertIn("**FAIL", verifier.report_phase24(current))
            with redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
                verifier.main()
            self.assertEqual(error.exception.code, 1)

    def test_saved_phase24_outputs_have_no_host_prefix_and_render_exactly(self):
        root = Path(__file__).resolve().parents[2]
        paths = (verifier.PHASE24_RECEIPT, verifier.PHASE24_REPORT)
        verifier.zero_literal_host_prefix({name: (root / name).read_bytes() for name in paths})
        saved = verifier.load_json((root / verifier.PHASE24_RECEIPT).read_bytes())
        self.assertEqual((root / verifier.PHASE24_REPORT).read_text(), verifier.report_phase24(saved))
        self.assertEqual(saved["qualified_count"], 0)


if __name__ == "__main__":
    unittest.main(verbosity=2)
