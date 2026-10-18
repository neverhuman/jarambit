"""Verify historical report provenance and the static measurement-code audit."""

import hashlib
import json
import math
from pathlib import Path, PurePosixPath
import re
import shutil
import subprocess
import tempfile
import unittest
from unittest.mock import patch


REPORT_DIR = PurePosixPath("experiments/G16-12-RSV2-04")
INDEX_PATH = REPORT_DIR / "UNIFIED_PROTOTYPE_PROVENANCE_INDEX.md"
AUDIT_PATH = REPORT_DIR / "CERTIFICATION_MEASUREMENT_CODE_AUDIT.md"
AUDIT_SHA256 = "1409141a8d7c7174245b8ba9b8b901397d11225d7602f56497ccb5ee27230b3a"
AUDIT_BYTES = 15527  # Filled from the reviewed document, never refreshed by tests.
AUDIT_TABLE_COUNT = 6
AUDIT_SCHEMA_SHA256 = "42212ab2618b5be7937ca24f7b0662d57de18a7602d75acecb7a12124c8b26df"
SCHEMA_AUDIT_PATH = REPORT_DIR / "SYNTHETIC_MEASUREMENT_SCHEMA_AUDIT.md"
SCHEMA_AUDIT_SHA256 = "SCHEMA_AUDIT_FILE_DIGEST"
SCHEMA_AUDIT_BYTES = 0
SCHEMA_AUDIT_TABLE_COUNT = 0
SCHEMA_AUDIT_TABLE_SHA256 = "SCHEMA_AUDIT_TABLE_DIGEST"
JSON_HEADER = ["Artifact path", "Bytes", "SHA-256", "JSON shape SHA-256", "Host-prefix occurrences", "Root type"]
FORBIDDEN_HOME = "/home/ubuntu"
AUDIT_SOURCES = (
    "crates/crescent/src/certification.rs",
    "crates/crescent/src/export.rs",
    "crates/crescent/src/geom.rs",
    "crates/crescent/src/fem.rs",
    "crates/crescent/tests/cert_integrity.rs",
)
ROOT = Path(__file__).resolve().parents[2]
HEADER = ["Version", "Report path", "SHA-256", "Bytes", "Tables", "Table-schema SHA-256"]
DIGEST = re.compile(r"[0-9a-f]{64}")


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def cells(line):
    line = line.strip()
    if not (line.startswith("|") and line.endswith("|")):
        raise ValueError("table row must have outer pipes")
    return [cell.strip() for cell in re.split(r"(?<!\\)\|", line[1:-1])]


def separator(row):
    return bool(row) and all(re.fullmatch(r":?-{3,}:?", cell) for cell in row)


def table_schema(text):
    """Capture ordered header lines, header cells, and data-row counts.

    This checks the pipe-delimited table format used by these reports, ignoring
    fenced code blocks. It does not interpret table values or scientific claims.
    """
    lines = text.splitlines()
    tables = []
    i = 0
    fence = None
    while i < len(lines):
        stripped = lines[i].strip()
        if fence:
            if re.fullmatch(re.escape(fence[0]) + "{" + str(len(fence)) + r",}\s*", stripped):
                fence = None
            i += 1
            continue
        opening = re.match(r"(`{3,}|~{3,})", stripped)
        if opening:
            fence = opening[0]
            i += 1
            continue
        if not stripped.startswith("|"):
            i += 1
            continue
        header = cells(lines[i])
        if not all(header) or i + 1 >= len(lines):
            raise ValueError(f"missing table header/separator at line {i + 1}")
        rule = cells(lines[i + 1])
        if not separator(rule) or len(rule) != len(header):
            raise ValueError(f"invalid table separator at line {i + 2}")
        start = i
        i += 2
        count = 0
        while i < len(lines) and lines[i].strip().startswith("|"):
            if len(cells(lines[i])) != len(header):
                raise ValueError(f"ragged table row at line {i + 1}")
            count += 1
            i += 1
        tables.append({"header_line": start + 1, "headers": header, "rows": count})
    if fence:
        raise ValueError("unclosed fenced code block")
    if not tables:
        raise ValueError("no report tables")
    return tables


def schema_digest(schema):
    data = json.dumps(schema, ensure_ascii=False, sort_keys=True, separators=(",", ":"))
    return sha256(data.encode("utf-8"))


def portable_path(root, value):
    path = PurePosixPath(value)
    if (not value or path.is_absolute() or "\\" in value or ":" in value
            or any(part in ("", ".", "..") for part in value.split("/"))
            or any(char.isspace() for char in value) or "%" in value or "#" in value):
        raise ValueError(f"non-portable report path: {value!r}")
    resolved = root.joinpath(*path.parts).resolve()
    if not resolved.is_relative_to(root.resolve()):
        raise ValueError("report path escapes root through symlink")
    return resolved


def verify(root):
    """Verify all nine reports relative to root; no Git, target/, or network."""
    root = Path(root)
    lines = (root / INDEX_PATH).read_text(encoding="utf-8").splitlines()
    rows = [cells(line) for line in lines if line.strip().startswith("|")]
    if len(rows) != 11 or rows[0] != HEADER or len(rows[1]) != len(HEADER) or not separator(rows[1]):
        raise ValueError("invalid index table schema or coverage")
    for version, row in zip(range(6, 15), rows[2:]):
        if len(row) != len(HEADER):
            raise ValueError("invalid index row width")
        label, relative, digest, size, count, table_digest = row
        path = portable_path(root, relative)
        expected = str(REPORT_DIR / f"UNIFIED_PROTOTYPE_V{version}_REPORT.md")
        if label != f"V{version}" or relative != expected:
            raise ValueError("unexpected report path/version, order, duplicate, or missing report")
        if not DIGEST.fullmatch(digest) or not DIGEST.fullmatch(table_digest):
            raise ValueError("invalid SHA-256 syntax")
        if not re.fullmatch(r"[1-9][0-9]*", size) or not re.fullmatch(r"[1-9][0-9]*", count):
            raise ValueError("invalid byte/table count")
        data = path.read_bytes()
        if sha256(data) != digest:
            raise ValueError(f"SHA-256 mismatch: {relative}")
        if len(data) != int(size):
            raise ValueError(f"byte count mismatch: {relative}")
        schema = table_schema(data.decode("utf-8"))
        if len(schema) != int(count) or schema_digest(schema) != table_digest:
            raise ValueError(f"table schema mismatch: {relative}")
    return len(rows) - 2


def verify_audit(root):
    """Check the fixed audit document and the exact source bytes it describes."""
    root = Path(root)
    data = portable_path(root, str(AUDIT_PATH)).read_bytes()
    if sha256(data) != AUDIT_SHA256:
        raise ValueError("audit SHA-256 mismatch")
    if len(data) != AUDIT_BYTES:
        raise ValueError("audit byte count mismatch")
    text = data.decode("utf-8")
    schema = table_schema(text)
    if len(schema) != AUDIT_TABLE_COUNT or schema_digest(schema) != AUDIT_SCHEMA_SHA256:
        raise ValueError("audit table schema mismatch")
    lines = text.splitlines()
    start = lines.index("|Source path|SHA-256|") + 2
    sources = []
    for line in lines[start:]:
        if not line.startswith("|"):
            break
        relative, digest = cells(line)
        path = portable_path(root, relative)
        if relative not in AUDIT_SOURCES or relative in sources or not DIGEST.fullmatch(digest):
            raise ValueError("invalid audit source provenance row")
        if sha256(path.read_bytes()) != digest:
            raise ValueError(f"audit source SHA-256 mismatch: {relative}")
        sources.append(relative)
    if set(sources) != set(AUDIT_SOURCES):
        raise ValueError("incomplete audit source provenance")
    return len(sources)


def json_shape(value):
    """Observed JSON type shape; no interpretation of measurement values."""
    if value is None:
        return "null"
    if isinstance(value, bool):
        return "boolean"
    if isinstance(value, int):
        return "integer"
    if isinstance(value, float):
        if not math.isfinite(value):
            raise ValueError("nonfinite JSON number")
        return "number"
    if isinstance(value, str):
        return "string"
    if isinstance(value, list):
        shapes = {json.dumps(json_shape(item), sort_keys=True, ensure_ascii=False, separators=(",", ":"))
                  for item in value}
        return {"array": [json.loads(item) for item in sorted(shapes)]}
    if isinstance(value, dict):
        return {"object": {key: json_shape(item) for key, item in sorted(value.items())}}
    raise ValueError("unsupported JSON type")


def strict_json(data):
    def reject_constant(value):
        raise ValueError(f"nonstandard JSON constant: {value}")

    def unique(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise ValueError(f"duplicate JSON key: {key}")
            result[key] = value
        return result

    value = json.loads(data, parse_constant=reject_constant, object_pairs_hook=unique)
    json_shape(value)  # Reject number overflow to infinity as well as explicit constants.
    return value


def home_occurrences(value):
    if isinstance(value, str):
        return value.count(FORBIDDEN_HOME)
    if isinstance(value, list):
        return sum(home_occurrences(item) for item in value)
    if isinstance(value, dict):
        return sum(home_occurrences(key) + home_occurrences(item) for key, item in value.items())
    return 0


def tracked_json_paths(root):
    root = Path(root)
    if (root / ".git").exists():
        paths = subprocess.check_output(
            ["git", "-C", str(root), "ls-files", "-z", "--", str(REPORT_DIR)], text=True
        ).split("\0")
        return {path for path in paths if path.endswith(".json")}
    return {path.relative_to(root).as_posix() for path in (root / REPORT_DIR).rglob("*.json")}


def verify_schema_audit(root):
    """Verify diagnostic receipt and every indexed JSON; return portability findings."""
    root = Path(root)
    data = portable_path(root, str(SCHEMA_AUDIT_PATH)).read_bytes()
    if FORBIDDEN_HOME.encode() in data:
        raise ValueError("host-home prefix in schema audit document")
    if sha256(data) != SCHEMA_AUDIT_SHA256 or len(data) != SCHEMA_AUDIT_BYTES:
        raise ValueError("schema audit SHA-256/bytes mismatch")
    text = data.decode("utf-8")
    if ("status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)" not in text.splitlines()
            or "qualified_count: 0" not in text.splitlines()):
        raise ValueError("invalid diagnostic qualification status")
    schema = table_schema(text)
    if len(schema) != SCHEMA_AUDIT_TABLE_COUNT or schema_digest(schema) != SCHEMA_AUDIT_TABLE_SHA256:
        raise ValueError("schema audit table mismatch")
    lines = text.splitlines()
    start = lines.index("|" + "|".join(JSON_HEADER) + "|") + 2
    archive = REPORT_DIR / "phase4"
    archive_hashes = {str(archive / name): digest for digest, name in
                      (line.split("  ", 1) for line in (root / archive / "SHA256SUMS").read_text().splitlines())}
    records = {}
    for line in lines[start:]:
        if not line.startswith("|"):
            break
        relative, size, digest, shape_digest, count, kind = cells(line)
        if relative in records or not DIGEST.fullmatch(digest) or not DIGEST.fullmatch(shape_digest):
            raise ValueError("invalid/duplicate JSON inventory row")
        raw = portable_path(root, relative).read_bytes()
        if sha256(raw) != digest or len(raw) != int(size):
            raise ValueError(f"JSON SHA-256/bytes mismatch: {relative}")
        value = strict_json(raw)
        if schema_digest(json_shape(value)) != shape_digest:
            raise ValueError(f"JSON shape mismatch: {relative}")
        if archive_hashes.get(relative) != digest:
            raise ValueError(f"archive SHA-256 mismatch: {relative}")
        hits = home_occurrences(value)
        root_type = next(iter(json_shape(value))) if isinstance(value, (dict, list)) else json_shape(value)
        if hits != int(count) or root_type != kind:
            raise ValueError(f"JSON diagnostic mismatch: {relative}")
        records[relative] = hits
    if set(records) != tracked_json_paths(root):
        raise ValueError("JSON inventory does not cover every tracked file")
    return records


class ReportProvenanceTests(unittest.TestCase):
    def test_reports_match_index(self):
        self.assertEqual(verify(ROOT), 9)

    def snapshot(self):
        # Only small document fixtures; no builds or geometry outputs.
        tmp = tempfile.TemporaryDirectory()
        self.addCleanup(tmp.cleanup)
        root = Path(tmp.name)
        (root / REPORT_DIR).mkdir(parents=True)
        paths = ([INDEX_PATH, AUDIT_PATH]
                 + [REPORT_DIR / f"UNIFIED_PROTOTYPE_V{v}_REPORT.md" for v in range(6, 15)]
                 + [PurePosixPath(path) for path in AUDIT_SOURCES])
        for path in paths:
            (root / path).parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / path, root / path)
        return root

    def alter_index(self, root, column, value):
        path = root / INDEX_PATH
        lines = path.read_text(encoding="utf-8").splitlines()
        i = next(i for i, line in enumerate(lines) if line.startswith("|V6|"))
        row = cells(lines[i])
        row[column] = value
        lines[i] = "|" + "|".join(row) + "|"
        path.write_text("\n".join(lines) + "\n", encoding="utf-8")

    def test_relocated_documents_without_git_or_target(self):
        root = self.snapshot()
        self.assertFalse((root / ".git").exists())
        self.assertFalse((root / "target").exists())
        self.assertEqual(verify(root), 9)
        self.assertEqual(verify_audit(root), 5)

    def test_audit_document_and_source_provenance(self):
        self.assertEqual(verify_audit(ROOT), 5)

    def test_audit_document_tampering_is_rejected(self):
        root = self.snapshot()
        path = root / AUDIT_PATH
        path.write_bytes(path.read_bytes() + b"\nchanged\n")
        with self.assertRaisesRegex(ValueError, "audit SHA-256 mismatch"):
            verify_audit(root)

    def test_audit_source_tampering_is_rejected(self):
        root = self.snapshot()
        path = root / AUDIT_SOURCES[0]
        path.write_bytes(path.read_bytes() + b"\n// changed\n")
        with self.assertRaisesRegex(ValueError, "audit source SHA-256 mismatch"):
            verify_audit(root)

    def test_audit_schema_is_checked_independently_of_file_digest(self):
        root = self.snapshot()
        path = root / AUDIT_PATH
        data = path.read_bytes().replace(b"|Source path|SHA-256|", b"|Source location|SHA-256|")
        path.write_bytes(data)
        # A refreshed file digest must not conceal a stale table-schema anchor.
        with patch.dict(globals(), {"AUDIT_SHA256": sha256(data), "AUDIT_BYTES": len(data)}):
            with self.assertRaisesRegex(ValueError, "audit table schema mismatch"):
                verify_audit(root)

    def test_modified_report_is_rejected(self):
        root = self.snapshot()
        path = root / REPORT_DIR / "UNIFIED_PROTOTYPE_V6_REPORT.md"
        path.write_bytes(path.read_bytes() + b"\nchanged\n")
        with self.assertRaisesRegex(ValueError, "SHA-256 mismatch"):
            verify(root)

    def test_index_digest_counts_and_schema_are_checked(self):
        for column, value, message in [(2, "f" * 64, "SHA-256 mismatch"),
                                      (2, "NOT-A-HASH", "SHA-256 syntax"),
                                      (3, "1", "byte count mismatch"),
                                      (4, "1", "table schema mismatch"),
                                      (5, "0" * 64, "table schema mismatch")]:
            with self.subTest(column=column, value=value):
                root = self.snapshot()
                self.alter_index(root, column, value)
                with self.assertRaisesRegex(ValueError, message):
                    verify(root)

    def test_missing_duplicate_and_reordered_entries_are_rejected(self):
        for mode in ["missing", "duplicate", "reordered"]:
            with self.subTest(mode=mode):
                root = self.snapshot()
                path = root / INDEX_PATH
                lines = path.read_text(encoding="utf-8").splitlines()
                i = next(i for i, line in enumerate(lines) if line.startswith("|V6|"))
                if mode == "missing":
                    del lines[i]
                elif mode == "duplicate":
                    lines[i + 1] = lines[i]
                else:
                    lines[i], lines[i + 1] = lines[i + 1], lines[i]
                path.write_text("\n".join(lines) + "\n", encoding="utf-8")
                with self.assertRaises(ValueError):
                    verify(root)

    def test_unsafe_paths_are_rejected(self):
        for value in ["/tmp/report.md", "../report.md", "a/../report.md", "a//b",
                      "./report.md", "C:/report.md", "a\\b", "https://host/report",
                      "a%2fb", "a#b", "a b"]:
            with self.subTest(value=value):
                root = self.snapshot()
                self.alter_index(root, 1, value)
                with self.assertRaisesRegex(ValueError, "non-portable"):
                    verify(root)

    def test_symlink_escape_is_rejected(self):
        root = self.snapshot()
        (root / "escape").symlink_to(root.parent, target_is_directory=True)
        with self.assertRaisesRegex(ValueError, "symlink"):
            portable_path(root, "escape/outside.md")

    def test_malformed_table_is_rejected_independently_of_hash(self):
        for text in ["|a|b|\n|---|---|\n|one|\n", "|a|\n|--|\n|one|\n"]:
            with self.subTest(text=text), self.assertRaises(ValueError):
                table_schema(text)

    def test_fenced_examples_are_ignored_and_escaped_pipes_retained(self):
        text = "```text\n|not a table|\n```\n|name|value|\n|---|---:|\n|a\\|b|1|\n"
        self.assertEqual(table_schema(text), [{"header_line": 4, "headers": ["name", "value"], "rows": 1}])

    def schema_snapshot(self):
        root = self.snapshot()
        paths = [SCHEMA_AUDIT_PATH, REPORT_DIR / "phase4/SHA256SUMS"]
        paths += [PurePosixPath(path) for path in tracked_json_paths(ROOT)]
        for path in paths:
            (root / path).parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / path, root / path)
        return root

    def test_schema_audit_and_all_tracked_json_digests(self):
        self.assertEqual(len(verify_schema_audit(ROOT)), 37)

    def test_tracked_json_have_zero_forbidden_home_paths(self):
        findings = {path: count for path, count in verify_schema_audit(ROOT).items() if count}
        self.assertFalse(findings, f"Non-portable historical JSON files ({len(findings)}): {findings}")

    def test_schema_audit_is_portable_without_git(self):
        self.assertEqual(len(verify_schema_audit(self.schema_snapshot())), 37)

    def test_schema_audit_document_tampering(self):
        root = self.schema_snapshot()
        path = root / SCHEMA_AUDIT_PATH
        path.write_bytes(path.read_bytes() + b"\nchanged\n")
        with self.assertRaisesRegex(ValueError, "schema audit SHA-256"):
            verify_schema_audit(root)

    def test_archived_json_tampering(self):
        root = self.schema_snapshot()
        relative = sorted(tracked_json_paths(root))[0]
        path = root / relative
        path.write_bytes(path.read_bytes() + b"\n")
        with self.assertRaisesRegex(ValueError, "JSON SHA-256"):
            verify_schema_audit(root)

    def test_synthetic_json_rejects_invalid_or_ambiguous_values(self):
        self.assertEqual(strict_json(b'{"schema":"synthetic.v1","samples":[0,1,null]}')["samples"], [0, 1, None])
        for data in [b'{"x":1,"x":2}', b'{"x":NaN}', b'{"x":Infinity}', b'{"x":1e999}', b'{"x":}']:
            with self.subTest(data=data), self.assertRaises(ValueError):
                strict_json(data)

    def test_synthetic_json_shape_distinguishes_boolean_and_number(self):
        self.assertNotEqual(schema_digest(json_shape({"x": True})), schema_digest(json_shape({"x": 1})))
        self.assertEqual(json_shape([]), {"array": []})

    def test_synthetic_json_decoded_paths_are_checked(self):
        raw = br'{"path":"\u002fhome\u002fubuntu/file"}'
        self.assertNotIn(FORBIDDEN_HOME.encode(), raw)
        self.assertEqual(home_occurrences(strict_json(raw)), 1)


if __name__ == "__main__":
    unittest.main()
