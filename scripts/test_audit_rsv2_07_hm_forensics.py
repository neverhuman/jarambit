"""Static saved-log, artifact digest and script/test inventory regressions."""

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


class ScriptTestInventoryTests(unittest.TestCase):
    def setUp(self):
        scratch = Path(__file__).resolve().parents[1] / "target/rsv2-07/phase28/test-fixtures"
        scratch.mkdir(parents=True, exist_ok=True)
        temporary = tempfile.TemporaryDirectory(prefix="inventory-", dir=scratch)
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.files = {
            audit.SCRIPT_PATHS[0]: b'raise RuntimeError("Source must never be executed")\n',
            audit.SCRIPT_PATHS[1]: 'import unittest\n# Unicode byte lengths: α\nclass Example(unittest.TestCase):\n    def test_one(self): pass\n    def helper(self): pass\n'.encode(),
        }
        for name, data in self.files.items():
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)

    def cli(self, *extra):
        with patch.object(sys, "argv", ["forensics", "--phase", "28", "--repo-root", str(self.root), *extra]), \
                redirect_stdout(io.StringIO()) as output:
            result = audit.main()
        return result, output.getvalue()

    def test_ast_count_distinguishes_methods_helpers_nested_code_and_strings(self):
        source = b'''
# def test_comment(self): pass
text = "def test_string(self): pass"
def test_top_level(): pass
class Helper:
    def test_not_a_testcase(self): pass
class Actual(unittest.TestCase):
    def test_sync(self):
        def test_nested(): pass
    async def test_async(self): pass
    def helper(self): pass
class Imported(TestCase):
    def test_imported(self): pass
class Inherited(Actual):
    def test_indirect(self): pass
def factory():
    class Nested(unittest.TestCase):
        def test_uncreated(self): pass
'''
        self.assertEqual(audit.static_test_methods(source), {
            "Actual": ["test_sync", "test_async"], "Imported": ["test_imported"]})

    def test_exact_hash_sizes_and_counts_without_executing_sources(self):
        current = audit.script_inventory(self.root)
        self.assertEqual((current["script_count"], current["test_method_count"]), (2, 1))
        self.assertEqual(current["total_size_bytes"], sum(map(len, self.files.values())))
        self.assertEqual(list(current["scripts"]), list(audit.SCRIPT_PATHS))
        for name, data in self.files.items():
            self.assertEqual(current["scripts"][name]["sha256"], hashlib.sha256(data).hexdigest())
            self.assertEqual(current["scripts"][name]["size_bytes"], len(data))
        self.assertEqual(current["scripts"][audit.SCRIPT_PATHS[0]]["test_method_count"], 0)
        self.assertEqual(current["scripts"][audit.SCRIPT_PATHS[1]]["test_classes"], {"Example": ["test_one"]})
        self.assertEqual(current, audit.script_inventory(self.root))
        self.assertNotIn(str(self.root), json.dumps(current))

    def test_missing_nonregular_or_invalid_python_sources_fail(self):
        path = self.root / audit.SCRIPT_PATHS[0]
        path.write_bytes(b"def broken(\n")
        with self.assertRaises(SyntaxError):
            audit.script_inventory(self.root)
        with redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
            self.cli()
        self.assertEqual(error.exception.code, 1)
        path.unlink()
        with self.assertRaises(FileNotFoundError):
            audit.script_inventory(self.root)
        path.symlink_to(audit.SCRIPT_PATHS[1].split("/")[-1])
        with self.assertRaisesRegex(ValueError, "regular script"):
            audit.script_inventory(self.root)

    def test_source_mutation_during_inventory_fails(self):
        original_read = Path.read_bytes
        changed = False
        def read(path):
            nonlocal changed
            data = original_read(path)
            if path == self.root / audit.SCRIPT_PATHS[0] and not changed:
                changed = True
                path.write_bytes(data + b"# changed\n")
            return data
        with patch.object(Path, "read_bytes", new=read), self.assertRaisesRegex(ValueError, "changed during audit"):
            audit.script_inventory(self.root)

    def test_write_only_creates_requested_outputs_and_readonly_needs_no_git(self):
        with patch.object(audit.subprocess, "check_output", side_effect=AssertionError("Unexpected Git command")):
            result, output = self.cli("--write")
            self.assertEqual(result, 0)
            self.assertIn("LANE: RSV2-07-hm | RSV2-07-p28 | PASS | scripts=2 | test_methods=1", output)
            expected = set(self.files) | {audit.SCRIPT_RECEIPT, audit.SCRIPT_REPORT}
            self.assertEqual({path.relative_to(self.root).as_posix() for path in self.root.rglob("*") if path.is_file()}, expected)
            before = {name: (self.root / name).read_bytes() for name in expected}
            with patch.object(Path, "write_text", side_effect=AssertionError("Readonly audit wrote")):
                self.assertEqual(self.cli()[0], 0)
            for name, data in before.items():
                self.assertEqual((self.root / name).read_bytes(), data)
            for name in (audit.SCRIPT_RECEIPT, audit.SCRIPT_REPORT):
                audit.require_portable((self.root / name).read_text(), name)

    def test_receipt_type_digest_and_count_tampering_fail(self):
        self.cli("--write")
        original = (self.root / audit.SCRIPT_RECEIPT).read_bytes()
        for field, value in (("sha256", "0" * 64), ("size_bytes", 1), ("test_method_count", False)):
            saved = json.loads(original)
            saved["scripts"][audit.SCRIPT_PATHS[0]][field] = value
            (self.root / audit.SCRIPT_RECEIPT).write_text(json.dumps(saved))
            with self.subTest(field=field), redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
                self.cli()
            self.assertEqual(error.exception.code, 1)

    def test_report_tampering_and_source_edits_invalidate_saved_inventory(self):
        self.cli("--write")
        report = self.root / audit.SCRIPT_REPORT
        original = report.read_bytes()
        report.write_bytes(original + b"changed\n")
        with redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
            self.cli()
        self.assertEqual(error.exception.code, 1)
        report.write_bytes(original)
        source = self.root / audit.SCRIPT_PATHS[0]
        source.write_bytes(source.read_bytes() + b"# local edit\n")
        with redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
            self.cli()
        self.assertEqual(error.exception.code, 1)

    def test_artifact_root_is_rejected_for_script_inventory(self):
        with redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
            self.cli("--artifact-root", str(self.root))
        self.assertEqual(error.exception.code, 1)

    def test_actual_static_declarations_match_current_unittest_discovery(self):
        root = Path(__file__).resolve().parents[1]
        current = audit.script_inventory(root)
        self.assertEqual(current["scripts"][audit.SCRIPT_PATHS[0]]["test_method_count"], 0)
        discovered = unittest.defaultTestLoader.loadTestsFromModule(sys.modules[__name__]).countTestCases()
        self.assertEqual(current["scripts"][audit.SCRIPT_PATHS[1]]["test_method_count"], discovered)


class IndexInventoryCrossCheckTests(unittest.TestCase):
    def setUp(self):
        self.root = Path("/relocated/checkout")
        self.artifact = audit.DIRECTORY + "PHASE01_NOTE.md"
        self.snapshot = {self.artifact: b"# Opaque saved audit\n",
                         audit.SCRIPT_PATHS[0]: b"opaque saved verifier bytes\n",
                         audit.SCRIPT_PATHS[1]: b"opaque saved test bytes\n"}
        self.live = dict(self.snapshot)
        for name, schema, task, table, count, paths in (
                (audit.DIGEST_RECEIPT, "jarambit.rsv2_07_hm.phase27_artifact_digest_index.v1", "RSV2-07-p27", "artifacts", "artifact_count", [self.artifact]),
                (audit.SCRIPT_RECEIPT, "jarambit.rsv2_07_hm.phase28_script_test_inventory.v1", "RSV2-07-p28", "scripts", "script_count", audit.SCRIPT_PATHS)):
            rows = {path: {"size_bytes": len(self.snapshot[path]), "sha256": hashlib.sha256(self.snapshot[path]).hexdigest()}
                    for path in paths}
            self.save(name, {"schema": schema, "task": task, "status": "pass", count: len(rows),
                             "total_size_bytes": sum(row["size_bytes"] for row in rows.values()), table: rows})

    def save(self, name, document):
        self.live[name] = self.snapshot[name] = json.dumps(document).encode()

    def read(self, path):
        name = path.relative_to(self.root).as_posix()
        if name not in self.live:
            raise FileNotFoundError(name)
        return self.live[name]

    def current(self):
        with patch.object(Path, "read_bytes", new=lambda path: self.read(path)), \
                patch.object(audit, "snapshot_bytes", side_effect=lambda root, name: self.snapshot[name]):
            return audit.cross_check(self.root)

    def test_disjoint_registries_have_exact_snapshot_and_live_byte_parity(self):
        result = self.current()
        self.assertEqual((result["entry_count"], result["overlap_paths"]), (3, []))
        self.assertTrue(result["disjoint_path_sets"])
        self.assertTrue(result["exact_snapshot_digest_parity"])
        self.assertTrue(result["working_tree_exact_digest_parity"])
        self.assertEqual(set(result["phase27_paths"]) & set(result["phase28_paths"]), set())
        for name, row in result["entries"].items():
            self.assertEqual(row["recorded_sha256"], hashlib.sha256(self.snapshot[name]).hexdigest())
            self.assertEqual(row["recorded_size_bytes"], len(self.snapshot[name]))
            self.assertEqual(row["recorded_sha256"], row["snapshot_sha256"])
        self.assertEqual(result, self.current())
        self.assertNotIn(str(self.root), json.dumps(result))

    def test_edited_or_missing_live_scripts_do_not_claim_live_parity(self):
        self.live[audit.SCRIPT_PATHS[0]] += b"edited\n"
        del self.live[audit.SCRIPT_PATHS[1]]
        result = self.current()
        self.assertEqual(result["status"], "pass")
        self.assertTrue(result["exact_snapshot_digest_parity"])
        self.assertFalse(result["working_tree_exact_digest_parity"])
        self.assertEqual(result["working_tree_drift_paths"], list(audit.SCRIPT_PATHS))
        row = result["entries"][audit.SCRIPT_PATHS[1]]
        self.assertIsNone(row["working_tree_size_bytes"])
        self.assertIsNone(row["working_tree_sha256"])
        self.assertIn("**Current working-tree parity: DRIFT.**", audit.render_cross(result))

    def test_overlapping_path_sets_fail_even_with_equal_digests(self):
        document = json.loads(self.live[audit.DIGEST_RECEIPT])
        path = audit.SCRIPT_PATHS[0]
        document["artifacts"] = {path: {"sha256": hashlib.sha256(self.snapshot[path]).hexdigest(), "size_bytes": len(self.snapshot[path])}}
        document["total_size_bytes"] = len(self.snapshot[path])
        self.save(audit.DIGEST_RECEIPT, document)
        with self.assertRaisesRegex(ValueError, "path sets overlap"):
            self.current()

    def test_snapshot_size_and_digest_mismatches_fail(self):
        for field, value in (("size_bytes", 1), ("sha256", "0" * 64)):
            self.setUp()
            document = json.loads(self.live[audit.DIGEST_RECEIPT])
            document["artifacts"][self.artifact][field] = value
            document["total_size_bytes"] = sum(row["size_bytes"] for row in document["artifacts"].values())
            self.save(audit.DIGEST_RECEIPT, document)
            with self.subTest(field=field), self.assertRaisesRegex(ValueError, "snapshot digest parity mismatch"):
                self.current()

    def test_receipt_schema_types_totals_and_digest_formats_fail_closed(self):
        changes = (("schema", "unknown"), ("status", "partial"), ("artifact_count", True), ("total_size_bytes", False),
                   ("artifacts", {}), ("artifacts", {self.artifact: {"size_bytes": False, "sha256": "0" * 64}}),
                   ("artifacts", {self.artifact: {"size_bytes": 0, "sha256": "INVALID"}}))
        original = json.loads(self.live[audit.DIGEST_RECEIPT])
        for field, value in changes:
            document = deepcopy(original)
            document[field] = value
            self.save(audit.DIGEST_RECEIPT, document)
            with self.subTest(field=field, value=value), self.assertRaises(ValueError):
                self.current()

    def test_malformed_json_or_receipt_bytes_differing_from_snapshot_fail(self):
        original = self.live[audit.DIGEST_RECEIPT]
        for data in (b'{"x":1,"x":2}', b'{"x":NaN}', b"{broken}", original + b"\n"):
            self.live[audit.DIGEST_RECEIPT] = data
            with self.subTest(data=data[:20]), self.assertRaises(ValueError):
                self.current()

    def test_source_mutation_during_cross_check_fails(self):
        changed = False
        def read(path):
            nonlocal changed
            data = self.read(path)
            if path == self.root / self.artifact and not changed:
                changed = True
                self.live[self.artifact] += b"changed\n"
            return data
        with patch.object(Path, "read_bytes", new=read), \
                patch.object(audit, "snapshot_bytes", side_effect=lambda root, name: self.snapshot[name]), \
                self.assertRaisesRegex(ValueError, "changed during audit"):
            audit.cross_check(self.root)

    def test_cli_writes_only_phase29_outputs_and_checks_readonly_receipt_and_report(self):
        original = deepcopy(self.live)
        def write(path, value, **kwargs):
            self.live[path.relative_to(self.root).as_posix()] = value.encode()
        def text(path, **kwargs):
            return self.read(path).decode()
        args = ["forensics", "--phase", "29", "--repo-root", str(self.root)]
        with patch.object(Path, "read_bytes", new=lambda path: self.read(path)), patch.object(Path, "read_text", new=text), \
                patch.object(Path, "write_text", new=write), patch.object(Path, "mkdir"), \
                patch.object(audit, "snapshot_bytes", side_effect=lambda root, name: self.snapshot[name]), \
                redirect_stdout(io.StringIO()) as output:
            with patch.object(sys, "argv", args + ["--write"]):
                self.assertEqual(audit.main(), 0)
            self.assertEqual(set(self.live) - set(original), {audit.CROSS_RECEIPT, audit.CROSS_REPORT})
            for name, data in original.items():
                self.assertEqual(self.live[name], data)
            with patch.object(sys, "argv", args), patch.object(Path, "write_text", side_effect=AssertionError("Readonly audit wrote")):
                self.assertEqual(audit.main(), 0)
                for name in (audit.CROSS_RECEIPT, audit.CROSS_REPORT):
                    saved = self.live[name]
                    self.live[name] += b"changed"
                    with redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
                        audit.main()
                    self.assertEqual(error.exception.code, 1)
                    self.live[name] = saved
            self.assertIn("parity=pinned_snapshot", output.getvalue())


class SummaryManifestTests(unittest.TestCase):
    def setUp(self):
        scratch = Path(__file__).resolve().parents[1] / "target/rsv2-07/phase30/test-fixtures"
        scratch.mkdir(parents=True, exist_ok=True)
        temporary = tempfile.TemporaryDirectory(prefix="summary-", dir=scratch)
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.files = {name: b'{"status":"saved"}\n' if name.endswith(".json") else "# Saved α\r\nSecond line\nFinal line".encode()
                      for name in audit.SUMMARY_PATHS}
        for name, data in self.files.items():
            self.write(name, data)

    def write(self, name, data):
        path = self.root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)

    def cli(self, *extra):
        with patch.object(sys, "argv", ["forensics", "--phase", "30", "--repo-root", str(self.root), *extra]), \
                redirect_stdout(io.StringIO()) as output:
            result = audit.main()
        return result, output.getvalue()

    def test_exact_eight_file_hash_sizes_lines_and_deterministic_totals(self):
        current = audit.summary_manifest(self.root)
        self.assertEqual((current["phases"], current["artifact_count"]), ([26, 27, 28, 29], 8))
        self.assertEqual(list(current["artifacts"]), sorted(self.files))
        self.assertEqual(current["total_size_bytes"], sum(map(len, self.files.values())))
        self.assertEqual(current["total_line_count"], 16)
        for name, data in self.files.items():
            self.assertEqual(current["artifacts"][name], {"sha256": hashlib.sha256(data).hexdigest(),
                "size_bytes": len(data), "line_count": 1 if name.endswith(".json") else 3, "literal_host_path_count": 0})
        self.assertEqual(current, audit.summary_manifest(self.root))
        self.assertNotIn(str(self.root), json.dumps(current))

    def test_line_count_handles_empty_unterminated_crlf_and_unicode_separators(self):
        cases = ((b"", 0), (b"one", 1), (b"one\n", 1), (b"one\n\n", 2),
                 (b"a\r\nb\r\n", 2), ("a\u2028b".encode(), 1), (b"a\rb", 1))
        for data, expected in cases:
            self.write(audit.REPORT, data)
            with self.subTest(data=data):
                self.assertEqual(audit.summary_manifest(self.root)["artifacts"][audit.REPORT]["line_count"], expected)

    def test_other_phases_scripts_and_own_outputs_are_outside_fixed_scope(self):
        before = audit.summary_manifest(self.root)
        for name in (audit.DIRECTORY + "PHASE25_OTHER.md", audit.SUMMARY_REPORT, audit.SUMMARY_RECEIPT, audit.SCRIPT_PATHS[0]):
            self.write(name, b"outside scope, deliberately not JSON")
        self.assertEqual(audit.summary_manifest(self.root), before)

    def test_invalid_utf8_and_non_strict_json_fail(self):
        for data in (b"\xff", b'{"x":1,"x":2}', b'{"x":NaN}', b'{"x":1e999}', b"{broken}"):
            self.write(audit.RECEIPT, data)
            with self.subTest(data=data), self.assertRaises(ValueError):
                audit.summary_manifest(self.root)

    def test_literal_host_paths_and_escaped_json_host_paths_fail(self):
        for data in (b"saved path: /home/example/file", b"saved path: /data/file", b"saved path: C:\\Users\\example\\file"):
            self.write(audit.REPORT, data)
            with self.subTest(data=data), self.assertRaisesRegex(ValueError, "Literal host path"):
                audit.summary_manifest(self.root)
        self.write(audit.REPORT, self.files[audit.REPORT])
        self.write(audit.RECEIPT, b'{"source":"\\u002fhome\\u002fexample\\u002ffile"}')
        with self.assertRaisesRegex(ValueError, "Literal host path"):
            audit.summary_manifest(self.root)

    def test_missing_or_symlinked_source_fails(self):
        path = self.root / audit.REPORT
        path.unlink()
        with self.assertRaises(FileNotFoundError):
            audit.summary_manifest(self.root)
        path.symlink_to(Path(audit.RECEIPT).name)
        with self.assertRaisesRegex(ValueError, "regular summary source"):
            audit.summary_manifest(self.root)

    def test_source_change_during_scan_fails(self):
        original_read = Path.read_bytes
        changed = False
        def read(path):
            nonlocal changed
            data = original_read(path)
            if path == self.root / audit.REPORT and not changed:
                changed = True
                path.write_bytes(data + b"changed\n")
            return data
        with patch.object(Path, "read_bytes", new=read), self.assertRaisesRegex(ValueError, "changed during audit"):
            audit.summary_manifest(self.root)

    def test_cli_writes_only_phase30_outputs_and_readonly_rejects_tampering(self):
        with patch.object(audit.subprocess, "check_output", side_effect=AssertionError("Unexpected Git command")):
            result, output = self.cli("--write")
            self.assertEqual(result, 0)
            self.assertIn("LANE: RSV2-07-hm | RSV2-07-p30 | PASS | artifacts=8 | literal_host_paths=0", output)
            expected = set(self.files) | {audit.SUMMARY_REPORT, audit.SUMMARY_RECEIPT}
            self.assertEqual({path.relative_to(self.root).as_posix() for path in self.root.rglob("*") if path.is_file()}, expected)
            for name, data in self.files.items():
                self.assertEqual((self.root / name).read_bytes(), data)
            with patch.object(Path, "write_text", side_effect=AssertionError("Readonly audit wrote")):
                self.assertEqual(self.cli()[0], 0)
            receipt = self.root / audit.SUMMARY_RECEIPT
            original = receipt.read_bytes()
            for field, value in (("sha256", "0" * 64), ("size_bytes", 0), ("line_count", 0), ("literal_host_path_count", False)):
                saved = json.loads(original)
                saved["artifacts"][audit.REPORT][field] = value
                receipt.write_text(json.dumps(saved))
                with self.subTest(field=field), redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
                    self.cli()
                self.assertEqual(error.exception.code, 1)
            receipt.write_bytes(original)
            self.write(audit.SUMMARY_REPORT, b"altered report\n")
            with redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
                self.cli()
            self.assertEqual(error.exception.code, 1)

    def test_artifact_root_is_rejected_for_summary_manifest(self):
        with redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
            self.cli("--artifact-root", str(self.root))
        self.assertEqual(error.exception.code, 1)


if __name__ == "__main__":
    unittest.main(verbosity=2)
