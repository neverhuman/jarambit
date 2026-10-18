"""Generic saved-log integrity tests; no historical lineage schema is invented."""

from contextlib import redirect_stderr, redirect_stdout
from copy import deepcopy
import hashlib
import io
import json
from pathlib import Path
import sys
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


if __name__ == "__main__":
    unittest.main(verbosity=2)
