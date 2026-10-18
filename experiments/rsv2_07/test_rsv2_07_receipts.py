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


if __name__ == "__main__":
    unittest.main(verbosity=2)
