"""Generic saved-log integrity tests; no historical lineage schema is invented."""

from contextlib import redirect_stderr, redirect_stdout
from copy import deepcopy
import hashlib
import io
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

import audit_rsv2_07_hm_forensics as audit


class SavedLogForensicsTests(unittest.TestCase):
    def setUp(self):
        self.root = Path("/relocated/source")
        self.checkout = Path("/relocated/checkout")
        self.first = "out/w315/evolution_log.jsonl"

    def current(self, files=None):
        files = {} if files is None else files
        def read(path):
            name = str(path.relative_to(self.root))
            if name not in files:
                raise FileNotFoundError(name)
            return files[name]
        with patch.object(Path, "read_bytes", new=read):
            return audit.audit(self.root)

    def test_six_missing_logs_are_partial_with_null_not_zero_counts(self):
        result = self.current()
        self.assertEqual((result["missing_logs"], result["available_logs"]), (6, 0))
        self.assertEqual(result["status"], "partial_missing_source_logs")
        self.assertFalse(result["lineage_counts_established"])
        self.assertIsNone(result["round0_elites_total"])
        self.assertIsNone(result["round1_births_total"])
        for row in result["waves"].values():
            self.assertIsNone(row["round0_elites_count"])
            self.assertIsNone(row["round1_births_count"])
            self.assertIsNone(row["sha256"])
        self.assertNotIn(str(self.root), json.dumps(result))
        self.assertEqual(self.current(), result)

    def test_present_records_are_hashed_but_not_given_an_unreviewed_lineage_meaning(self):
        data = b'{"round":0,"elites":["opaque"]}\n{"round":1,"births":100}\n'
        result = self.current({self.first: data})
        row = result["waves"]["w315"]
        self.assertEqual((row["sha256"], row["size_bytes"]), (hashlib.sha256(data).hexdigest(), len(data)))
        self.assertEqual(row["jsonl_records"], 2)
        self.assertEqual(row["lineage_status"], "event_schema_not_reviewed")
        self.assertIsNone(row["round0_elites_count"])
        self.assertIsNone(row["round1_births_count"])

    def test_empty_file_has_no_records_but_does_not_establish_zero_lineage_counts(self):
        result = self.current({self.first: b""})
        row = result["waves"]["w315"]
        self.assertEqual(row["jsonl_records"], 0)
        self.assertEqual(row["availability"], "present")
        self.assertIsNone(row["round1_births_count"])

    def test_jsonl_rejects_duplicate_keys_nonfinite_blank_nonobject_and_invalid_utf8(self):
        bad = (b'{"x":1,"x":2}\n', b'{"x":{"k":1,"k":2}}\n', b'{"x":NaN}\n', b'{"x":Infinity}\n',
               b'{"x":1e999}\n', b'{}\n\n', b'[]\n', b'true\n', b'{broken}\n', b'\xff')
        for data in bad:
            with self.subTest(data=data), self.assertRaises(ValueError):
                self.current({self.first: data})
        self.assertEqual(audit.parse_jsonl(b'{}\r\n{"x":1}\r\n'), [{}, {"x": 1}])

    def test_changed_bytes_or_new_availability_during_audit_fail(self):
        for initial in (None, b'{}\n'):
            reads = {}
            def read(path):
                name = str(path.relative_to(self.root))
                reads[name] = reads.get(name, 0) + 1
                if name != self.first or initial is None and reads[name] == 1:
                    raise FileNotFoundError(name)
                return (b'{}\n' if initial is None else initial) + (b' ' if reads[name] > 1 else b'')
            with self.subTest(initial=initial), patch.object(Path, "read_bytes", new=read), \
                    self.assertRaisesRegex(ValueError, "changed during audit"):
                audit.audit(self.root)

    def test_read_errors_are_not_reported_as_missing(self):
        with patch.object(Path, "read_bytes", side_effect=PermissionError("fixture")), self.assertRaises(PermissionError):
            audit.audit(self.root)

    def test_relocated_readonly_cli_emits_partial_lane_and_checks_receipt_and_report_without_git(self):
        current = self.current()
        saved = deepcopy(current)
        report = audit.render(current)
        args = ["audit_rsv2_07_hm_forensics.py", "--repo-root", str(self.checkout), "--artifact-root", str(self.root)]
        def read(path):
            return json.dumps(saved).encode() if path == self.checkout / audit.RECEIPT else report.encode()
        with patch.object(sys, "argv", args), patch.object(audit, "audit", return_value=current), \
                patch.object(Path, "read_bytes", new=read), patch.object(Path, "read_text", new=lambda path: report), \
                patch.object(Path, "write_text") as write, \
                patch.object(audit.subprocess, "check_output", side_effect=AssertionError("Unexpected Git discovery")), \
                redirect_stdout(io.StringIO()) as output:
            self.assertEqual(audit.main(), 2)
            self.assertIn("LANE: RSV2-07-hm | RSV2-07-p26 | PARTIAL", output.getvalue())
            write.assert_not_called()
            saved["available_logs"] = False
            with redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
                audit.main()
            self.assertEqual(error.exception.code, 1)
            saved = deepcopy(current)
            report += "changed"
            with redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
                audit.main()
            self.assertEqual(error.exception.code, 1)

    def test_explicit_write_only_creates_the_two_requested_outputs_and_still_exits_partial(self):
        current = self.current()
        args = ["audit_rsv2_07_hm_forensics.py", "--repo-root", str(self.checkout), "--artifact-root", str(self.root), "--write"]
        writes = {}
        with patch.object(sys, "argv", args), patch.object(audit, "audit", return_value=current), \
                patch.object(Path, "mkdir"), patch.object(Path, "write_text", new=lambda path, value: writes.setdefault(path, value)), \
                redirect_stdout(io.StringIO()):
            self.assertEqual(audit.main(), 2)
        self.assertEqual(set(writes), {self.checkout / audit.RECEIPT, self.checkout / audit.REPORT})
        self.assertEqual(json.loads(writes[self.checkout / audit.RECEIPT]), current)

    def test_saved_outputs_remain_partial_and_have_no_literal_host_prefix(self):
        root = Path(__file__).resolve().parents[1]
        saved = audit.strict_json((root / audit.RECEIPT).read_bytes())
        self.assertFalse(saved["lineage_counts_established"])
        self.assertEqual((root / audit.REPORT).read_text(), audit.render(saved))
        for name in (audit.RECEIPT, audit.REPORT):
            self.assertNotIn(b'/home/' + b'ubuntu', (root / name).read_bytes())


class ArtifactDigestIndexTests(unittest.TestCase):
    def setUp(self):
        scratch = Path(__file__).resolve().parents[1] / "target/rsv2-07/phase27/test-fixtures"
        scratch.mkdir(parents=True, exist_ok=True)
        temporary = tempfile.TemporaryDirectory(prefix="digest-", dir=scratch)
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.git("init", "-q")
        self.markdown = audit.DIRECTORY + "PHASE01_NOTE.md"
        self.receipt = audit.DIRECTORY + "phase01_note.json"
        self.files = {self.markdown: "# Saved audit\n\nOpaque UTF-8: α.\n".encode(),
                      self.receipt: b'{"status":"partial","count":null}\n'}
        for name, data in self.files.items():
            self.write(name, data)
        self.git("add", "--", *self.files)

    def git(self, *arguments):
        return audit.subprocess.check_output(["git", "-C", str(self.root), *arguments], stderr=audit.subprocess.PIPE)

    def write(self, name, data):
        path = self.root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)

    def cli(self, *extra):
        with patch.object(sys, "argv", ["forensics", "--phase", "27", "--repo-root", str(self.root), *extra]), \
                redirect_stdout(io.StringIO()) as output:
            result = audit.main()
        return result, output.getvalue()

    def test_sorted_exact_byte_digests_and_deterministic_totals(self):
        current = audit.digest_audit(self.root)
        self.assertEqual(list(current["artifacts"]), sorted(self.files))
        self.assertEqual(current["artifact_count"], 2)
        self.assertEqual(current["total_size_bytes"], sum(map(len, self.files.values())))
        for name, data in self.files.items():
            self.assertEqual(current["artifacts"][name], {
                "size_bytes": len(data), "sha256": hashlib.sha256(data).hexdigest(), "literal_host_path_count": 0})
        self.assertEqual(current, audit.digest_audit(self.root))
        self.assertNotIn(str(self.root), json.dumps(current))

    def test_only_matching_tracked_files_in_directory_are_included(self):
        baseline = audit.digest_audit(self.root)
        self.write(audit.DIRECTORY + "phase99_untracked.json", b"invalid JSON")
        extras = [audit.DIRECTORY + "notes.md", "docs/PHASE99_OUTSIDE.md", audit.DIRECTORY + "phase99_wrong.md"]
        for name in extras:
            self.write(name, b"outside scope")
        self.git("add", "--", *extras)
        self.assertEqual(audit.digest_audit(self.root), baseline)
        nested = audit.DIRECTORY + "archive/PHASE02_NOTE.md"
        self.write(nested, b"nested tracked audit\n")
        self.git("add", "--", nested)
        self.assertIn(nested, audit.digest_audit(self.root)["artifacts"])

    def test_self_outputs_are_explicitly_excluded_even_when_tracked(self):
        baseline = audit.digest_audit(self.root)
        for name in audit.DIGEST_EXCLUSIONS:
            self.write(name, b"self output placeholder")
        self.git("add", "--", *audit.DIGEST_EXCLUSIONS)
        self.assertEqual(audit.digest_audit(self.root), baseline)
        self.assertEqual(baseline["excluded_self_outputs"], list(audit.DIGEST_EXCLUSIONS))

    def test_tracked_local_edits_use_working_tree_bytes(self):
        before = audit.digest_audit(self.root)
        updated = self.files[self.markdown] + b"Local edit\n"
        self.write(self.markdown, updated)
        after = audit.digest_audit(self.root)
        self.assertNotEqual(before, after)
        self.assertEqual(after["artifacts"][self.markdown]["sha256"], hashlib.sha256(updated).hexdigest())

    def test_missing_tracked_file_and_empty_inventory_fail(self):
        (self.root / self.markdown).unlink()
        with self.assertRaises(FileNotFoundError):
            audit.digest_audit(self.root)
        self.git("rm", "--cached", "--", *self.files)
        with self.assertRaisesRegex(ValueError, "No tracked phase artifacts"):
            audit.digest_audit(self.root)

    def test_absolute_host_paths_and_json_escaped_host_paths_fail(self):
        paths = ("/home/example/file", "/Users/example/file", "/root/file", "/tmp/file", "/data/file",
                 r"C:\Users\example\file", r"\\server\share\file", "file:///saved/file")
        for text in paths:
            with self.subTest(text=text):
                self.write(self.markdown, ("saved path: " + text).encode())
                with self.assertRaisesRegex(ValueError, "Literal host path"):
                    audit.digest_audit(self.root)
        self.write(self.markdown, self.files[self.markdown])
        self.write(self.receipt, b'{"source":"\\u002fhome\\u002fexample\\u002ffile"}')
        with self.assertRaisesRegex(ValueError, "Literal host path"):
            audit.digest_audit(self.root)

    def test_relative_paths_and_web_links_are_portable(self):
        self.write(self.markdown, b"docs/audits/note.md out/w315/file.json https://example.org/path\n")
        self.assertEqual(audit.digest_audit(self.root)["literal_host_path_count"], 0)

    def test_invalid_utf8_and_non_strict_json_fail(self):
        for data in (b"\xff", b'{"x":1,"x":2}', b'{"x":NaN}', b'{"x":1e999}', b"{broken}"):
            with self.subTest(data=data):
                self.write(self.receipt, data)
                with self.assertRaises(ValueError):
                    audit.digest_audit(self.root)

    def test_symlinked_artifact_is_not_followed(self):
        (self.root / self.markdown).unlink()
        (self.root / self.markdown).symlink_to(Path(self.receipt).name)
        with self.assertRaisesRegex(ValueError, "regular artifact"):
            audit.digest_audit(self.root)

    def test_byte_or_membership_mutation_during_scan_fails(self):
        original_read = Path.read_bytes
        changed = False
        def read(path):
            nonlocal changed
            data = original_read(path)
            if path == self.root / self.markdown and not changed:
                changed = True
                path.write_bytes(data + b"changed\n")
            return data
        with patch.object(Path, "read_bytes", new=read), self.assertRaisesRegex(ValueError, "changed during audit"):
            audit.digest_audit(self.root)
        original_inventory = audit.tracked_phase_paths
        with patch.object(audit, "tracked_phase_paths", side_effect=[original_inventory(self.root), []]), \
                self.assertRaisesRegex(ValueError, "inventory changed"):
            audit.digest_audit(self.root)

    def test_cli_writes_only_outputs_then_readonly_verifies_from_other_cwd(self):
        existing = {path.relative_to(self.root) for path in self.root.rglob("*") if path.is_file()}
        result, output = self.cli("--write")
        self.assertEqual(result, 0)
        self.assertIn("LANE: RSV2-07-hm | RSV2-07-p27 | PASS", output)
        added = {path.relative_to(self.root) for path in self.root.rglob("*") if path.is_file()} - existing
        self.assertEqual(added, {Path(name) for name in audit.DIGEST_EXCLUSIONS})
        before = {name: (self.root / name).read_bytes() for name in audit.DIGEST_EXCLUSIONS}
        with patch.object(Path, "write_text", side_effect=AssertionError("Readonly verifier wrote")):
            self.assertEqual(self.cli()[0], 0)
        for name, data in before.items():
            self.assertEqual((self.root / name).read_bytes(), data)
            audit.require_portable(data.decode(), name)

    def test_saved_receipt_digest_type_and_report_tampering_fail(self):
        self.cli("--write")
        original = (self.root / audit.DIGEST_RECEIPT).read_bytes()
        for field, value in (("sha256", "0" * 64), ("literal_host_path_count", False), ("size_bytes", 1)):
            saved = json.loads(original)
            saved["artifacts"][self.markdown][field] = value
            self.write(audit.DIGEST_RECEIPT, json.dumps(saved).encode())
            with self.subTest(field=field), redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
                self.cli()
            self.assertEqual(error.exception.code, 1)
        self.write(audit.DIGEST_RECEIPT, original)
        self.write(audit.DIGEST_REPORT, b"changed report\n")
        with redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
            self.cli()
        self.assertEqual(error.exception.code, 1)


if __name__ == "__main__":
    unittest.main(verbosity=2)
