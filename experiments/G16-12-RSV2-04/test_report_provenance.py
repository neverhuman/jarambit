"""Verify the portable V6..V14 report index using only the standard library."""

import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import shutil
import tempfile
import unittest


REPORT_DIR = PurePosixPath("experiments/G16-12-RSV2-04")
INDEX_PATH = REPORT_DIR / "UNIFIED_PROTOTYPE_PROVENANCE_INDEX.md"
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


class ReportProvenanceTests(unittest.TestCase):
    def test_reports_match_index(self):
        self.assertEqual(verify(ROOT), 9)

    def snapshot(self):
        # Only small document fixtures; no builds or geometry outputs.
        tmp = tempfile.TemporaryDirectory()
        self.addCleanup(tmp.cleanup)
        root = Path(tmp.name)
        (root / REPORT_DIR).mkdir(parents=True)
        for path in [INDEX_PATH] + [REPORT_DIR / f"UNIFIED_PROTOTYPE_V{v}_REPORT.md" for v in range(6, 15)]:
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


if __name__ == "__main__":
    unittest.main()
