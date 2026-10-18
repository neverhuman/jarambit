#!/usr/bin/env python3
"""Static saved-log integrity, tracked artifact digests and script/test inventory."""

import argparse
import ast
import fnmatch
import hashlib
import json
import math
from pathlib import Path
import re
import stat
import subprocess

DIRECTORY = "docs/audits/rsv2_07_hm/"
RECEIPT = DIRECTORY + "phase26_round1_births_vs_elites_lineage_audit.json"
REPORT = DIRECTORY + "PHASE26_ROUND1_BIRTHS_VS_ELITES_LINEAGE_AUDIT.md"
WAVES = tuple(f"w{number}" for number in range(315, 321))
DIGEST_RECEIPT = DIRECTORY + "phase27_artifact_digest_index.json"
DIGEST_REPORT = DIRECTORY + "PHASE27_ARTIFACT_DIGEST_INDEX.md"
DIGEST_EXCLUSIONS = (DIGEST_REPORT, DIGEST_RECEIPT)
SCRIPT_RECEIPT = DIRECTORY + "phase28_script_test_inventory_audit.json"
SCRIPT_REPORT = DIRECTORY + "PHASE28_SCRIPT_TEST_INVENTORY_AUDIT.md"
SCRIPT_PATHS = ("scripts/audit_rsv2_07_hm_forensics.py", "scripts/test_audit_rsv2_07_hm_forensics.py")
CROSS_RECEIPT = DIRECTORY + "phase29_index_inventory_cross_check_audit.json"
CROSS_REPORT = DIRECTORY + "PHASE29_INDEX_INVENTORY_CROSS_CHECK_AUDIT.md"
CROSS_COMMIT = "cc1918287761ba7a16683b3b7bbfc8c06cce50b1"
SUMMARY_RECEIPT = DIRECTORY + "phase30_summary_manifest_audit.json"
SUMMARY_REPORT = DIRECTORY + "PHASE30_SUMMARY_MANIFEST_AUDIT.md"
SUMMARY_PATHS = tuple(sorted((REPORT, RECEIPT, DIGEST_REPORT, DIGEST_RECEIPT,
                              SCRIPT_REPORT, SCRIPT_RECEIPT, CROSS_REPORT, CROSS_RECEIPT)))
MANIFEST_CROSS_RECEIPT = DIRECTORY + "phase31_manifest_cross_check_audit.json"
MANIFEST_CROSS_REPORT = DIRECTORY + "PHASE31_MANIFEST_CROSS_CHECK_AUDIT.md"
CUMULATIVE_RECEIPT = DIRECTORY + "phase32_cumulative_receipt_table_audit.json"
CUMULATIVE_REPORT = DIRECTORY + "PHASE32_CUMULATIVE_RECEIPT_TABLE_AUDIT.md"
CUMULATIVE_PATHS = tuple(sorted((*SUMMARY_PATHS, SUMMARY_REPORT, SUMMARY_RECEIPT,
                                 MANIFEST_CROSS_REPORT, MANIFEST_CROSS_RECEIPT)))
TABLE_PARITY_RECEIPT = DIRECTORY + "phase33_table_parity_audit.json"
TABLE_PARITY_REPORT = DIRECTORY + "PHASE33_TABLE_PARITY_AUDIT.md"
# Absolute POSIX paths, Windows drive paths, UNC paths and file URIs.
HOST_PATH = re.compile(r"(?<![\w./:])/(?!/)[\w.~-]+|(?<!\w)[A-Za-z]:[\\/]|\\\\[^\\\s]+\\|file://")


def require(condition, message):
    if not condition:
        raise ValueError(message)


def object_pairs(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result, f"Duplicate JSON key: {key}")
        result[key] = value
    return result


def finite_float(value):
    parsed = float(value)
    require(math.isfinite(parsed), "Nonfinite JSON number")
    return parsed


def reject_constant(value):
    raise ValueError(f"Nonfinite JSON literal: {value}")


def strict_json(data):
    return json.loads(data, object_pairs_hook=object_pairs, parse_float=finite_float, parse_constant=reject_constant)


def parse_jsonl(data):
    records = []
    for number, line in enumerate(data.decode("utf-8").splitlines(), 1):
        require(bool(line.strip()), f"Blank JSONL record at line {number}")
        record = strict_json(line)
        require(isinstance(record, dict), f"Expected JSON object at line {number}")
        records.append(record)
    return records


def read_optional(path):
    try:
        return path.read_bytes()
    except FileNotFoundError:
        return None


def require_portable(text, name):
    require(HOST_PATH.search(text) is None, f"Literal host path in {name}")


def tracked_phase_paths(repo_root):
    output = subprocess.check_output(
        ["git", "-C", str(repo_root), "ls-files", "--cached", "-z", "--", DIRECTORY], stderr=subprocess.PIPE)
    names = set()
    for raw in output.split(b"\0"):
        if not raw:
            continue
        name = raw.decode("utf-8")
        path = Path(name)
        require(not path.is_absolute() and ".." not in path.parts and name.startswith(DIRECTORY),
                "Unsafe tracked artifact path")
        if (fnmatch.fnmatchcase(path.name, "PHASE*.md") or fnmatch.fnmatchcase(path.name, "phase*.json")) \
                and name not in DIGEST_EXCLUSIONS:
            require_portable(name, "tracked artifact name")
            names.add(name)
    return sorted(names)


def digest_audit(repo_root):
    repo_root = Path(repo_root)
    names = tracked_phase_paths(repo_root)
    require(bool(names), "No tracked phase artifacts found")
    sources, artifacts = {}, {}
    for name in names:
        path = repo_root / name
        require(stat.S_ISREG(path.lstat().st_mode), f"Expected regular artifact: {name}")
        require(all(not parent.is_symlink() for parent in path.parents if parent != repo_root),
                f"Symlinked artifact directory: {name}")
        data = path.read_bytes()
        require_portable(data.decode("utf-8"), name)
        if path.suffix == ".json":
            # Also detect host paths hidden by JSON string escapes.
            require_portable(json.dumps(strict_json(data), ensure_ascii=False), name)
        sources[name] = data
        artifacts[name] = {"size_bytes": len(data), "sha256": hashlib.sha256(data).hexdigest(),
                           "literal_host_path_count": 0}
    require(tracked_phase_paths(repo_root) == names, "Tracked artifact inventory changed during audit")
    for name, data in sources.items():
        path = repo_root / name
        require(stat.S_ISREG(path.lstat().st_mode) and path.read_bytes() == data,
                "Tracked artifact bytes or type changed during audit")
    return {"schema": "jarambit.rsv2_07_hm.phase27_artifact_digest_index.v1", "task": "RSV2-07-p27",
            "status": "pass", "scope": "tracked PHASE*.md and phase*.json below " + DIRECTORY,
            "digest_basis": "working-tree bytes of paths selected by git ls-files --cached",
            "excluded_self_outputs": list(DIGEST_EXCLUSIONS),
            "exclusion_reason": "The index and report exclude themselves to avoid recursive digests.",
            "artifact_count": len(artifacts), "total_size_bytes": sum(row["size_bytes"] for row in artifacts.values()),
            "literal_host_path_count": 0, "artifacts": artifacts}


def render_digest(document):
    rows = [f"| {name} | {row['size_bytes']} | {row['sha256']} | {row['literal_host_path_count']} |"
            for name, row in document["artifacts"].items()]
    return "\n".join([
        "# Phase-27 artifact digest index", "",
        f"**PASS.** Indexed {document['artifact_count']} tracked phase artifacts, totaling {document['total_size_bytes']} bytes. Literal host paths: 0.", "",
        "The inventory includes tracked PHASE*.md and phase*.json files beneath docs/audits/rsv2_07_hm, sorted by repository-relative path. SHA-256 and sizes describe current working-tree bytes, including any tracked local edits. Untracked files and other filenames are outside scope. No conclusions about the contents of earlier audits are inferred from their digests.", "",
        "The Phase-27 report and JSON index explicitly exclude themselves, even after tracking, to avoid a recursive digest cycle. Missing tracked artifacts, nonregular files, symlinks, invalid UTF-8, malformed JSON and literal absolute host paths fail verification. JSON string escapes are decoded for the host-path check. Tracking membership and source bytes are checked again after scanning.", "",
        "| Tracked artifact | Bytes | SHA-256 | Literal host paths |",
        "| --- | ---: | --- | ---: |", *rows, "",
        "Persisted filenames are repository-relative. The verifier can run from another working directory using --repo-root to select a relocated Git checkout. Verification checks the complete JSON index and exact report text without writes; --write replaces only these two Phase-27 outputs.", "",
        "```sh", "python3 scripts/audit_rsv2_07_hm_forensics.py --phase 27",
        "python3 -m unittest discover -s scripts -p 'test_audit_rsv2_07_hm_forensics.py' -v", "git diff --check", "```", "",
        "Execution uses CPUs 14,15 at nice 10. This is a static artifact digest audit. Changes are left uncommitted.", "",
    ])


def run_digest(args):
    current = digest_audit(args.repo_root)
    report = render_digest(current)
    receipt = json.dumps(current, indent=2) + "\n"
    require_portable(report, "Phase-27 report")
    require_portable(receipt, "Phase-27 receipt")
    if args.write:
        (args.repo_root / DIRECTORY).mkdir(parents=True, exist_ok=True)
        (args.repo_root / DIGEST_RECEIPT).write_text(receipt, encoding="utf-8")
        (args.repo_root / DIGEST_REPORT).write_text(report, encoding="utf-8")
    else:
        saved = strict_json((args.repo_root / DIGEST_RECEIPT).read_bytes())
        require(json.dumps(saved, sort_keys=True) == json.dumps(current, sort_keys=True), "Phase-27 saved receipt mismatch")
        require((args.repo_root / DIGEST_REPORT).read_text(encoding="utf-8") == report, "Phase-27 report mismatch")
    print(f"LANE: RSV2-07-hm | RSV2-07-p27 | PASS | artifacts={current['artifact_count']} | literal_host_paths=0")
    return 0


def static_test_methods(data):
    classes = {}
    for node in ast.parse(data).body:
        if not isinstance(node, ast.ClassDef):
            continue
        is_test_case = any(
            isinstance(base, ast.Name) and base.id == "TestCase" or
            isinstance(base, ast.Attribute) and isinstance(base.value, ast.Name) and
            base.value.id == "unittest" and base.attr == "TestCase" for base in node.bases)
        if is_test_case:
            classes[node.name] = [method.name for method in node.body
                                  if isinstance(method, (ast.FunctionDef, ast.AsyncFunctionDef))
                                  and method.name.startswith("test")]
    return classes


def script_inventory(repo_root):
    repo_root = Path(repo_root)
    sources, scripts = {}, {}
    for name in SCRIPT_PATHS:
        path = repo_root / name
        require(stat.S_ISREG(path.lstat().st_mode), f"Expected regular script: {name}")
        data = path.read_bytes()
        classes = static_test_methods(data)
        sources[name] = data
        scripts[name] = {"size_bytes": len(data), "sha256": hashlib.sha256(data).hexdigest(),
                         "test_method_count": sum(map(len, classes.values())), "test_classes": classes}
    for name, data in sources.items():
        path = repo_root / name
        require(stat.S_ISREG(path.lstat().st_mode) and path.read_bytes() == data,
                "Script bytes or type changed during audit")
    return {"schema": "jarambit.rsv2_07_hm.phase28_script_test_inventory.v1", "task": "RSV2-07-p28",
            "status": "pass", "digest_basis": "exact working-tree bytes of the two named scripts",
            "test_count_rule": "direct test-prefixed sync or async method declarations in top-level classes explicitly based on unittest.TestCase or TestCase",
            "limitations": "static declarations only; no import, execution, inheritance, alias resolution or runtime discovery",
            "script_count": len(scripts), "total_size_bytes": sum(row["size_bytes"] for row in scripts.values()),
            "test_method_count": sum(row["test_method_count"] for row in scripts.values()), "scripts": scripts}


def render_scripts(document):
    rows = [f"| {name} | {row['size_bytes']} | {row['sha256']} | {row['test_method_count']} |"
            for name, row in document["scripts"].items()]
    return "\n".join([
        "# Phase-28 script and test inventory audit", "",
        f"**PASS.** Inventoried {document['script_count']} scripts, totaling {document['total_size_bytes']} bytes and {document['test_method_count']} statically declared test methods.", "",
        "| Repository-relative script | Bytes | SHA-256 | Test methods |",
        "| --- | ---: | --- | ---: |", *rows, "",
        "Sizes and SHA-256 describe exact working-tree bytes after the Phase-28 implementation and tests were written. Both sources are reread to detect changes during the audit. Missing files, nonregular files and invalid Python syntax fail. The output files are outside the input set, so no self-digest cycle exists.", "",
        "The AST parser counts directly declared synchronous and asynchronous methods whose names start with test, inside top-level classes explicitly based on unittest.TestCase or TestCase. Helpers, top-level functions, nested methods, strings and comments are excluded. Test class and method names are recorded in the JSON receipt. Sources are parsed without importing or executing them. Inherited methods, import aliases, dynamically generated tests and runtime discovery are outside this static count; these counts do not claim test execution or coverage.", "",
        "Persisted paths are repository-relative. --repo-root selects a relocated source tree without requiring Git. A normal Phase-28 run verifies the complete saved JSON inventory and exact report text without writes. --write replaces only the two Phase-28 output files.", "",
        "```sh", "python3 scripts/audit_rsv2_07_hm_forensics.py --phase 28",
        "python3 -m unittest discover -s scripts -p 'test_audit_rsv2_07_hm_forensics.py' -v", "git diff --check", "```", "",
        "Execution uses CPUs 14,15 at nice 10. This is a static script and test inventory audit. Changes are left uncommitted.", "",
    ])


def run_scripts(args):
    require(args.artifact_root is None, "Phase-28 reads scripts in --repo-root; --artifact-root is not supported")
    current = script_inventory(args.repo_root)
    report = render_scripts(current)
    receipt = json.dumps(current, indent=2) + "\n"
    require_portable(report, "Phase-28 report")
    require_portable(receipt, "Phase-28 receipt")
    if args.write:
        (args.repo_root / DIRECTORY).mkdir(parents=True, exist_ok=True)
        (args.repo_root / SCRIPT_RECEIPT).write_text(receipt, encoding="utf-8")
        (args.repo_root / SCRIPT_REPORT).write_text(report, encoding="utf-8")
    else:
        saved = strict_json((args.repo_root / SCRIPT_RECEIPT).read_bytes())
        require(json.dumps(saved, sort_keys=True) == json.dumps(current, sort_keys=True), "Phase-28 saved receipt mismatch")
        require((args.repo_root / SCRIPT_REPORT).read_text(encoding="utf-8") == report, "Phase-28 report mismatch")
    print(f"LANE: RSV2-07-hm | RSV2-07-p28 | PASS | scripts={current['script_count']} | test_methods={current['test_method_count']}")
    return 0


def snapshot_bytes(repo_root, name):
    return subprocess.check_output(["git", "-C", str(repo_root), "show", f"{CROSS_COMMIT}:{name}"],
                                   stderr=subprocess.PIPE)


def cross_check(repo_root):
    repo_root = Path(repo_root)
    sources, documents, inputs = {}, {}, {}
    for name, schema, task, table, count in (
            (DIGEST_RECEIPT, "jarambit.rsv2_07_hm.phase27_artifact_digest_index.v1", "RSV2-07-p27", "artifacts", "artifact_count"),
            (SCRIPT_RECEIPT, "jarambit.rsv2_07_hm.phase28_script_test_inventory.v1", "RSV2-07-p28", "scripts", "script_count")):
        data = (repo_root / name).read_bytes()
        sources[name] = data
        document = strict_json(data)
        require(isinstance(document, dict) and document.get("schema") == schema and
                document.get("task") == task and document.get("status") == "pass", "Unexpected input receipt schema or status")
        rows = document.get(table)
        require(isinstance(rows, dict) and bool(rows), "Missing or empty input registry")
        require(type(document.get(count)) is int and document[count] == len(rows), "Registry count mismatch")
        for path, row in rows.items():
            require(isinstance(path, str) and not Path(path).is_absolute() and ".." not in Path(path).parts,
                    "Unsafe registry path")
            require_portable(path, "registry path")
            require(isinstance(row, dict) and type(row.get("size_bytes")) is int and row["size_bytes"] >= 0 and
                    isinstance(row.get("sha256"), str) and re.fullmatch(r"[0-9a-f]{64}", row["sha256"]) is not None,
                    "Invalid registry digest metadata")
        require(type(document.get("total_size_bytes")) is int and
                document["total_size_bytes"] == sum(row["size_bytes"] for row in rows.values()), "Registry byte total mismatch")
        require(data == snapshot_bytes(repo_root, name), "Input receipt differs from pinned snapshot")
        inputs[name] = {"size_bytes": len(data), "sha256": hashlib.sha256(data).hexdigest(), "snapshot_bytes_equal": True}
        documents[name] = rows
    left, right = documents[DIGEST_RECEIPT], documents[SCRIPT_RECEIPT]
    overlap = sorted(set(left) & set(right))
    require(not overlap, "Phase-27 and Phase-28 path sets overlap")
    require(set(right) == set(SCRIPT_PATHS), "Phase-28 script path set mismatch")
    require(all(name.startswith(DIRECTORY) and
                (fnmatch.fnmatchcase(Path(name).name, "PHASE*.md") or fnmatch.fnmatchcase(Path(name).name, "phase*.json"))
                for name in left), "Phase-27 artifact path outside scope")
    entries = {}
    for receipt, rows in documents.items():
        for name, row in sorted(rows.items()):
            historical = snapshot_bytes(repo_root, name)
            digest = hashlib.sha256(historical).hexdigest()
            require((len(historical), digest) == (row["size_bytes"], row["sha256"]), "Pinned snapshot digest parity mismatch")
            live = read_optional(repo_root / name)
            sources[name] = live
            live_digest = None if live is None else hashlib.sha256(live).hexdigest()
            live_size = None if live is None else len(live)
            entries[name] = {"source_receipt": receipt, "recorded_size_bytes": row["size_bytes"],
                             "recorded_sha256": row["sha256"], "snapshot_size_bytes": len(historical),
                             "snapshot_sha256": digest, "snapshot_digest_parity": True,
                             "working_tree_size_bytes": live_size, "working_tree_sha256": live_digest,
                             "working_tree_digest_parity": (live_size, live_digest) == (row["size_bytes"], row["sha256"])}
    for name, data in sources.items():
        require(read_optional(repo_root / name) == data, "Cross-check source bytes changed during audit")
    drift = sorted(name for name, row in entries.items() if not row["working_tree_digest_parity"])
    return {"schema": "jarambit.rsv2_07_hm.phase29_index_inventory_cross_check.v1", "task": "RSV2-07-p29",
            "status": "pass", "parity_scope": "saved registry entries versus pinned Phase-28 Git snapshot",
            "snapshot_commit": CROSS_COMMIT, "input_receipts": inputs,
            "phase27_paths": sorted(left), "phase28_paths": sorted(right), "overlap_paths": overlap,
            "disjoint_path_sets": True, "entry_count": len(entries), "exact_snapshot_digest_parity": True,
            "working_tree_exact_digest_parity": not drift, "working_tree_drift_paths": drift, "entries": entries}


def render_cross(document):
    rows = [f"| {name} | {row['recorded_size_bytes']} | {row['recorded_sha256']} | yes | {'yes' if row['working_tree_digest_parity'] else 'no'} |"
            for name, row in document["entries"].items()]
    return "\n".join([
        "# Phase-29 index and inventory cross-check audit", "",
        f"**PASS for the pinned snapshot.** Phase 27 lists {len(document['phase27_paths'])} paths and Phase 28 lists {len(document['phase28_paths'])} paths; their intersection is empty. All {document['entry_count']} recorded sizes and SHA-256 digests exactly match the pinned Git snapshot.", "",
        f"Snapshot commit: `{document['snapshot_commit']}`. Both input receipt files must also match this snapshot byte for byte. Registry schema, status, counts, byte totals, digest formats and relative path scope are checked. Duplicate keys, nonfinite JSON and overlapping path sets fail.", "",
        f"**Current working-tree parity: {'PASS' if document['working_tree_exact_digest_parity'] else 'DRIFT'}.** {len(document['working_tree_drift_paths'])} paths differ from the saved registry metadata. Extending the verifier and its tests for Phase 29 changes the script bytes recorded by Phase 28. Snapshot parity does not assert current script parity. The existing Phase-27 and Phase-28 reports and receipts are preserved.", "",
        "| Registered relative path | Recorded bytes | Recorded SHA-256 | Snapshot parity | Working-tree parity |",
        "| --- | ---: | --- | --- | --- |", *rows, "",
        "The JSON records both observed snapshot and current working-tree sizes and hashes. Inputs are reread to detect changes during the audit. This checks the saved registries; it does not rebuild Phase 27's historical file selection, rerun Phase 28's test count, execute source files or infer anything about earlier audit conclusions. Missing current entry files are recorded with null size and digest and fail working-tree parity; missing snapshot evidence fails the audit.", "",
        "Paths in the outputs are repository-relative. --repo-root selects a relocated Git checkout containing the pinned commit. Normal verification compares the complete saved JSON and exact report text without writes. --write replaces only the two Phase-29 outputs. Exit 0 means disjoint path sets and exact snapshot digest parity; current working-tree drift remains explicit in the output.", "",
        "```sh", "python3 scripts/audit_rsv2_07_hm_forensics.py --phase 29",
        "python3 -m unittest discover -s scripts -p 'test_audit_rsv2_07_hm_forensics.py' -v", "git diff --check", "```", "",
        "Execution uses CPUs 14,15 at nice 10. This is a static receipt cross-check. Changes are left uncommitted.", "",
    ])


def run_cross(args):
    require(args.artifact_root is None, "Phase-29 uses receipts in --repo-root; --artifact-root is not supported")
    current = cross_check(args.repo_root)
    report = render_cross(current)
    receipt = json.dumps(current, indent=2) + "\n"
    require_portable(report, "Phase-29 report")
    require_portable(receipt, "Phase-29 receipt")
    if args.write:
        (args.repo_root / DIRECTORY).mkdir(parents=True, exist_ok=True)
        (args.repo_root / CROSS_RECEIPT).write_text(receipt, encoding="utf-8")
        (args.repo_root / CROSS_REPORT).write_text(report, encoding="utf-8")
    else:
        saved = strict_json((args.repo_root / CROSS_RECEIPT).read_bytes())
        require(json.dumps(saved, sort_keys=True) == json.dumps(current, sort_keys=True), "Phase-29 saved receipt mismatch")
        require((args.repo_root / CROSS_REPORT).read_text(encoding="utf-8") == report, "Phase-29 report mismatch")
    print(f"LANE: RSV2-07-hm | RSV2-07-p29 | PASS | parity=pinned_snapshot | entries={current['entry_count']} | overlap=0 | working_tree_drift={len(current['working_tree_drift_paths'])}")
    return 0


def artifact_inventory(repo_root, names):
    repo_root = Path(repo_root)
    sources, artifacts = {}, {}
    for name in names:
        path = repo_root / name
        require(stat.S_ISREG(path.lstat().st_mode), f"Expected regular summary source: {name}")
        data = path.read_bytes()
        require_portable(data.decode("utf-8"), name)
        if path.suffix == ".json":
            require_portable(json.dumps(strict_json(data), ensure_ascii=False), name)
        sources[name] = data
        artifacts[name] = {"sha256": hashlib.sha256(data).hexdigest(), "size_bytes": len(data),
                           "line_count": data.count(b"\n") + int(bool(data) and not data.endswith(b"\n")),
                           "literal_host_path_count": 0}
    for name, data in sources.items():
        path = repo_root / name
        require(stat.S_ISREG(path.lstat().st_mode) and path.read_bytes() == data,
                "Summary source bytes or type changed during audit")
    return artifacts


def summary_manifest(repo_root):
    artifacts = artifact_inventory(repo_root, SUMMARY_PATHS)
    return {"schema": "jarambit.rsv2_07_hm.phase30_summary_manifest.v1", "task": "RSV2-07-p30",
            "status": "pass", "phases": [26, 27, 28, 29], "artifact_count": len(artifacts),
            "digest_basis": "exact working-tree bytes of the eight named Phase-26 through Phase-29 outputs",
            "line_count_rule": "LF-delimited physical lines including an unterminated final line; empty file has zero lines",
            "total_size_bytes": sum(row["size_bytes"] for row in artifacts.values()),
            "total_line_count": sum(row["line_count"] for row in artifacts.values()),
            "literal_host_path_count": 0, "artifacts": artifacts}


def render_summary(document):
    rows = [f"| {name} | {row['size_bytes']} | {row['line_count']} | {row['sha256']} | {row['literal_host_path_count']} |"
            for name, row in document["artifacts"].items()]
    return "\n".join([
        "# Phase-30 summary manifest audit", "",
        f"**PASS for the artifact inventory.** Indexed {document['artifact_count']} Phase-26 through Phase-29 Markdown and JSON files, totaling {document['total_size_bytes']} bytes and {document['total_line_count']} lines. Literal host paths: 0.", "",
        "| Repository-relative artifact | Bytes | Lines | SHA-256 | Literal host paths |",
        "| --- | ---: | ---: | --- | ---: |", *rows, "",
        "The input set is the fixed eight named report and receipt files from Phases 26 through 29. Hashes and sizes describe exact working-tree bytes. Physical lines are counted by LF delimiters, plus an unterminated final line; an empty file has zero lines. CRLF is one line ending, and Unicode line separators do not add physical lines. Phase-30 outputs, source scripts and other files are outside the input set.", "",
        "All inputs must be regular UTF-8 files. JSON is parsed strictly, rejecting duplicate keys and nonfinite numbers. Literal absolute host paths are rejected in raw text and decoded JSON strings. Source bytes and file types are checked again after scanning. These checks describe saved artifacts and do not rerun earlier audits or establish their conclusions; Phase 26's partial lineage result and Phase 29's snapshot-only parity remain as recorded.", "",
        "Persisted paths are repository-relative. --repo-root selects a relocated source tree without requiring Git. Normal verification checks the complete saved JSON manifest and exact report text without writes; --write replaces only the two Phase-30 outputs. The earlier reports and receipts are preserved.", "",
        "```sh", "python3 scripts/audit_rsv2_07_hm_forensics.py --phase 30",
        "python3 -m unittest discover -s scripts -p 'test_audit_rsv2_07_hm_forensics.py' -v", "git diff --check", "```", "",
        "Execution uses CPUs 14,15 at nice 10. This is a static summary manifest audit. Changes are left uncommitted.", "",
    ])


def run_summary(args):
    require(args.artifact_root is None, "Phase-30 uses saved outputs in --repo-root; --artifact-root is not supported")
    current = summary_manifest(args.repo_root)
    report = render_summary(current)
    receipt = json.dumps(current, indent=2) + "\n"
    require_portable(report, "Phase-30 report")
    require_portable(receipt, "Phase-30 receipt")
    if args.write:
        (args.repo_root / DIRECTORY).mkdir(parents=True, exist_ok=True)
        (args.repo_root / SUMMARY_RECEIPT).write_text(receipt, encoding="utf-8")
        (args.repo_root / SUMMARY_REPORT).write_text(report, encoding="utf-8")
    else:
        saved = strict_json((args.repo_root / SUMMARY_RECEIPT).read_bytes())
        require(json.dumps(saved, sort_keys=True) == json.dumps(current, sort_keys=True), "Phase-30 saved receipt mismatch")
        require((args.repo_root / SUMMARY_REPORT).read_text(encoding="utf-8") == report, "Phase-30 report mismatch")
    print(f"LANE: RSV2-07-hm | RSV2-07-p30 | PASS | artifacts={current['artifact_count']} | literal_host_paths=0")
    return 0


def digest_pair(row, prefix=""):
    require(isinstance(row, dict) and type(row.get(prefix + "size_bytes")) is int and row[prefix + "size_bytes"] >= 0 and
            isinstance(row.get(prefix + "sha256"), str) and re.fullmatch(r"[0-9a-f]{64}", row[prefix + "sha256"]) is not None,
            "Invalid receipt digest metadata")
    return {"size_bytes": row[prefix + "size_bytes"], "sha256": row[prefix + "sha256"]}


def manifest_cross_check(repo_root):
    repo_root = Path(repo_root)
    sources = {}
    for name in (SUMMARY_RECEIPT, *SUMMARY_PATHS):
        path = repo_root / name
        require(stat.S_ISREG(path.lstat().st_mode), f"Expected regular manifest source: {name}")
        sources[name] = path.read_bytes()
    saved = strict_json(sources[SUMMARY_RECEIPT])
    live = summary_manifest(repo_root)
    require(json.dumps(saved, sort_keys=True) == json.dumps(live, sort_keys=True), "Phase-30 manifest differs from live artifacts")
    documents = {}
    for name, schema, task in (
            (DIGEST_RECEIPT, "jarambit.rsv2_07_hm.phase27_artifact_digest_index.v1", "RSV2-07-p27"),
            (SCRIPT_RECEIPT, "jarambit.rsv2_07_hm.phase28_script_test_inventory.v1", "RSV2-07-p28"),
            (CROSS_RECEIPT, "jarambit.rsv2_07_hm.phase29_index_inventory_cross_check.v1", "RSV2-07-p29")):
        document = strict_json(sources[name])
        require(isinstance(document, dict) and document.get("schema") == schema and
                document.get("task") == task and document.get("status") == "pass", "Unexpected linked receipt schema or status")
        documents[name] = document
    left, right, cross = documents[DIGEST_RECEIPT], documents[SCRIPT_RECEIPT], documents[CROSS_RECEIPT]
    artifacts, scripts = left.get("artifacts"), right.get("scripts")
    require(isinstance(artifacts, dict) and bool(artifacts) and set(artifacts) <= set(SUMMARY_PATHS), "Phase-27 registry outside manifest")
    require(isinstance(scripts, dict) and set(scripts) == set(SCRIPT_PATHS), "Phase-28 script path set mismatch")
    require(type(left.get("artifact_count")) is int and left["artifact_count"] == len(artifacts) and
            type(right.get("script_count")) is int and right["script_count"] == len(scripts), "Linked registry count mismatch")
    entries, inputs = cross.get("entries"), cross.get("input_receipts")
    require(isinstance(entries, dict) and set(entries) == set(artifacts) | set(scripts) and
            isinstance(inputs, dict) and set(inputs) == {DIGEST_RECEIPT, SCRIPT_RECEIPT}, "Phase-29 registry path set mismatch")
    require(cross.get("phase27_paths") == sorted(artifacts) and cross.get("phase28_paths") == sorted(scripts) and
            cross.get("disjoint_path_sets") is True and cross.get("overlap_paths") == [] and
            cross.get("exact_snapshot_digest_parity") is True and type(cross.get("entry_count")) is int and
            cross["entry_count"] == len(entries), "Phase-29 registry invariants mismatch")
    checks = []
    def compare(name, relation, expected, observed):
        require(expected == observed, "Receipt digest parity mismatch: " + relation)
        checks.append({"path": name, "relation": relation, **expected, "match": True})
    for name, row in sorted(artifacts.items()):
        compare(name, "Phase-27 artifact versus Phase-30 manifest", digest_pair(saved["artifacts"][name]), digest_pair(row))
    for name, row in sorted(inputs.items()):
        require(isinstance(row, dict) and row.get("snapshot_bytes_equal") is True, "Phase-29 input snapshot flag mismatch")
        compare(name, "Phase-29 input receipt versus Phase-30 manifest", digest_pair(saved["artifacts"][name]), digest_pair(row))
    for name, row in sorted(entries.items()):
        source = DIGEST_RECEIPT if name in artifacts else SCRIPT_RECEIPT
        require(isinstance(row, dict) and row.get("source_receipt") == source and row.get("snapshot_digest_parity") is True,
                "Phase-29 entry provenance or snapshot flag mismatch")
        expected = digest_pair(saved["artifacts"][name] if name in artifacts else scripts[name])
        for prefix in ("recorded_", "snapshot_", "working_tree_") if name in artifacts else ("recorded_", "snapshot_"):
            compare(name, "Phase-29 " + prefix.rstrip("_") + " versus " + ("Phase-30 manifest" if name in artifacts else "Phase-28 saved script"),
                    expected, digest_pair(row, prefix))
    for name, data in sources.items():
        path = repo_root / name
        require(stat.S_ISREG(path.lstat().st_mode) and path.read_bytes() == data,
                "Manifest cross-check source bytes or type changed during audit")
    return {"schema": "jarambit.rsv2_07_hm.phase31_manifest_cross_check.v1", "task": "RSV2-07-p31", "status": "pass",
            "input_receipts": {name: {"sha256": hashlib.sha256(sources[name]).hexdigest(), "size_bytes": len(sources[name])}
                               for name in (SUMMARY_RECEIPT, DIGEST_RECEIPT, SCRIPT_RECEIPT, CROSS_RECEIPT)},
            "live_artifact_count": len(live["artifacts"]), "live_artifact_digest_parity": True,
            "total_size_bytes": live["total_size_bytes"], "total_line_count": live["total_line_count"],
            "literal_host_path_count": 0, "artifacts": live["artifacts"],
            "receipt_comparison_count": len(checks), "receipt_digest_parity": True, "receipt_comparisons": checks,
            "historical_script_paths": sorted(scripts), "current_script_bytes_checked": False}


def render_manifest_cross(document):
    rows = [f"| {name} | {row['size_bytes']} | {row['sha256']} | yes |"
            for name, row in document["artifacts"].items()]
    return "\n".join([
        "# Phase-31 manifest cross-check audit", "",
        f"**PASS.** All {document['live_artifact_count']} Phase-26 through Phase-29 artifacts match the Phase-30 manifest's SHA-256 and byte sizes. All {document['receipt_comparison_count']} linked receipt metadata comparisons agree.", "",
        "| Manifest artifact | Bytes | SHA-256 | Live parity |", "| --- | ---: | --- | --- |", *rows, "",
        "The Phase-30 manifest is recomputed from the fixed eight live artifacts and compared in full, including line counts, total sizes and host-path checks. Phase-27 artifact metadata is compared with the corresponding Phase-30 entries. Phase-29 pins for its Phase-27 and Phase-28 input receipts are compared with Phase-30's hashes of those same receipt files. Its recorded, snapshot and saved working-tree fields for manifest artifacts must also agree with Phase 30.", "",
        "Phase-28 script paths are outside Phase 30's artifact set. Their historical sizes and hashes are compared with Phase-29's recorded and snapshot fields. Current scripts and Phase-29's saved script working-tree observations are outside this check; extending the scripts does not refresh earlier receipts. The JSON lists every metadata comparison and pins the four input receipts. This audit verifies saved metadata consistency, without fetching Git snapshots or reassessing earlier audit conclusions.", "",
        "Inputs are parsed strictly and reread to detect changes during the audit. Registry schemas, source references, path sets, counts, snapshot flags, digest formats and integer sizes are checked. Missing files, symlinks, inconsistent metadata, duplicate keys, nonfinite JSON and host paths in the Phase-30 input set fail. Paths in these outputs are repository-relative. --repo-root supports a relocated source tree without Git.", "",
        "Normal verification compares the complete saved Phase-31 JSON and exact report text without writes. --write replaces only the two Phase-31 outputs. Earlier reports and receipts are preserved.", "",
        "```sh", "python3 scripts/audit_rsv2_07_hm_forensics.py --phase 31",
        "python3 -m unittest discover -s scripts -p 'test_audit_rsv2_07_hm_forensics.py' -v", "git diff --check", "```", "",
        "Execution uses CPUs 14,15 at nice 10. This is a static manifest and receipt cross-check. Changes are left uncommitted.", "",
    ])


def run_manifest_cross(args):
    require(args.artifact_root is None, "Phase-31 uses saved outputs in --repo-root; --artifact-root is not supported")
    current = manifest_cross_check(args.repo_root)
    report = render_manifest_cross(current)
    receipt = json.dumps(current, indent=2) + "\n"
    require_portable(report, "Phase-31 report")
    require_portable(receipt, "Phase-31 receipt")
    if args.write:
        (args.repo_root / DIRECTORY).mkdir(parents=True, exist_ok=True)
        (args.repo_root / MANIFEST_CROSS_RECEIPT).write_text(receipt, encoding="utf-8")
        (args.repo_root / MANIFEST_CROSS_REPORT).write_text(report, encoding="utf-8")
    else:
        saved = strict_json((args.repo_root / MANIFEST_CROSS_RECEIPT).read_bytes())
        require(json.dumps(saved, sort_keys=True) == json.dumps(current, sort_keys=True), "Phase-31 saved receipt mismatch")
        require((args.repo_root / MANIFEST_CROSS_REPORT).read_text(encoding="utf-8") == report, "Phase-31 report mismatch")
    print(f"LANE: RSV2-07-hm | RSV2-07-p31 | PASS | live_artifacts={current['live_artifact_count']} | receipt_comparisons={current['receipt_comparison_count']}")
    return 0


def cumulative_receipt_table(repo_root):
    artifacts = artifact_inventory(repo_root, CUMULATIVE_PATHS)
    return {"schema": "jarambit.rsv2_07_hm.phase32_cumulative_receipt_table.v1", "task": "RSV2-07-p32",
            "status": "pass", "phases": [26, 27, 28, 29, 30, 31], "artifact_count": len(artifacts),
            "digest_basis": "exact working-tree bytes of the twelve named Phase-26 through Phase-31 outputs",
            "line_count_rule": "LF-delimited physical lines including an unterminated final line; empty file has zero lines",
            "total_size_bytes": sum(row["size_bytes"] for row in artifacts.values()),
            "total_line_count": sum(row["line_count"] for row in artifacts.values()),
            "literal_host_path_count": 0, "artifacts": artifacts}


def render_cumulative(document):
    rows = [f"| {name} | {row['size_bytes']} | {row['line_count']} | {row['sha256']} | {row['literal_host_path_count']} |"
            for name, row in document["artifacts"].items()]
    return "\n".join([
        "# Phase-32 cumulative receipt table audit", "",
        f"**PASS for the artifact inventory.** Indexed {document['artifact_count']} Phase-26 through Phase-31 Markdown and JSON files, totaling {document['total_size_bytes']} bytes and {document['total_line_count']} lines. Literal host paths: 0.", "",
        "| Repository-relative artifact | Bytes | Lines | SHA-256 | Literal host paths |",
        "| --- | ---: | ---: | --- | ---: |", *rows, "",
        "The input set is the fixed twelve named report and receipt files from Phases 26 through 31, sorted by relative path. Hashes and sizes describe exact working-tree bytes. Lines are counted by LF delimiters, plus an unterminated final line; an empty file has zero lines. CRLF is one line ending, and Unicode line separators do not add physical lines. Phase-32 outputs, source scripts and other files are outside this cumulative table.", "",
        "All inputs must be regular UTF-8 files. JSON is parsed strictly, rejecting duplicate keys and nonfinite numbers. Literal absolute host paths are rejected in raw text and decoded JSON strings. Source bytes and file types are checked again after scanning. This is an inventory of saved artifacts; earlier audit conclusions are preserved, including Phase 26's partial lineage result and Phase 29's snapshot-only parity. Earlier audits are not rerun by this command.", "",
        "Persisted paths are repository-relative. --repo-root selects a relocated source tree without Git. Normal verification compares the complete saved JSON table and exact report text without writes; --write replaces only the two Phase-32 outputs. Earlier reports and receipts are preserved.", "",
        "```sh", "python3 scripts/audit_rsv2_07_hm_forensics.py --phase 32",
        "python3 -m unittest discover -s scripts -p 'test_audit_rsv2_07_hm_forensics.py' -v", "git diff --check", "```", "",
        "Execution uses CPUs 14,15 at nice 10. This is a static cumulative receipt table audit. Changes are left uncommitted.", "",
    ])


def run_cumulative(args):
    require(args.artifact_root is None, "Phase-32 uses saved outputs in --repo-root; --artifact-root is not supported")
    current = cumulative_receipt_table(args.repo_root)
    report = render_cumulative(current)
    receipt = json.dumps(current, indent=2) + "\n"
    require_portable(report, "Phase-32 report")
    require_portable(receipt, "Phase-32 receipt")
    if args.write:
        (args.repo_root / DIRECTORY).mkdir(parents=True, exist_ok=True)
        (args.repo_root / CUMULATIVE_RECEIPT).write_text(receipt, encoding="utf-8")
        (args.repo_root / CUMULATIVE_REPORT).write_text(report, encoding="utf-8")
    else:
        saved = strict_json((args.repo_root / CUMULATIVE_RECEIPT).read_bytes())
        require(json.dumps(saved, sort_keys=True) == json.dumps(current, sort_keys=True), "Phase-32 saved receipt mismatch")
        require((args.repo_root / CUMULATIVE_REPORT).read_text(encoding="utf-8") == report, "Phase-32 report mismatch")
    print(f"LANE: RSV2-07-hm | RSV2-07-p32 | PASS | artifacts={current['artifact_count']} | literal_host_paths=0")
    return 0


def table_parity(repo_root):
    repo_root = Path(repo_root)
    sources = {}
    for name in (CUMULATIVE_RECEIPT, *CUMULATIVE_PATHS):
        path = repo_root / name
        require(stat.S_ISREG(path.lstat().st_mode), f"Expected regular table parity source: {name}")
        sources[name] = path.read_bytes()
    documents = {}
    for name, recompute in ((CUMULATIVE_RECEIPT, cumulative_receipt_table),
                            (SUMMARY_RECEIPT, summary_manifest), (MANIFEST_CROSS_RECEIPT, manifest_cross_check)):
        document = strict_json(sources[name])
        require(json.dumps(document, sort_keys=True) == json.dumps(recompute(repo_root), sort_keys=True),
                "Saved receipt differs from recomputed metadata: " + name)
        documents[name] = document
    table, summary, cross = (documents[name] for name in (CUMULATIVE_RECEIPT, SUMMARY_RECEIPT, MANIFEST_CROSS_RECEIPT))
    checks, excluded = [], []
    def compare(name, relation, row):
        require(name in table["artifacts"], "Linked artifact outside Phase-32 table")
        expected, observed = digest_pair(table["artifacts"][name]), digest_pair(row)
        require(expected == observed, "Phase-32 linked receipt digest parity mismatch")
        checks.append({"path": name, "relation": relation, **expected, "match": True})
    for relation, rows in (("Phase-30 artifacts", summary["artifacts"]),
                           ("Phase-31 artifacts", cross["artifacts"]),
                           ("Phase-31 input receipts", cross["input_receipts"])):
        for name, row in sorted(rows.items()):
            compare(name, relation, row)
    for row in cross["receipt_comparisons"]:
        if row["path"] in table["artifacts"]:
            compare(row["path"], "Phase-31 comparison: " + row["relation"], row)
        else:
            require(row["path"] in SCRIPT_PATHS, "Unexpected Phase-31 comparison outside artifact table")
            excluded.append({"path": row["path"], "relation": row["relation"]})
    for name, data in sources.items():
        path = repo_root / name
        require(stat.S_ISREG(path.lstat().st_mode) and path.read_bytes() == data,
                "Table parity source bytes or type changed during audit")
    return {"schema": "jarambit.rsv2_07_hm.phase33_table_parity.v1", "task": "RSV2-07-p33", "status": "pass",
            "input_receipts": {name: {"sha256": hashlib.sha256(sources[name]).hexdigest(), "size_bytes": len(sources[name])}
                               for name in (CUMULATIVE_RECEIPT, SUMMARY_RECEIPT, MANIFEST_CROSS_RECEIPT)},
            "live_artifact_count": table["artifact_count"], "live_artifact_digest_parity": True,
            "artifacts": table["artifacts"], "total_size_bytes": table["total_size_bytes"],
            "total_line_count": table["total_line_count"], "literal_host_path_count": 0,
            "receipt_comparison_count": len(checks), "receipt_digest_parity": True, "receipt_comparisons": checks,
            "excluded_historical_script_comparisons": excluded, "current_script_bytes_checked": False}


def render_table_parity(document):
    rows = [f"| {name} | {row['size_bytes']} | {row['sha256']} | yes |"
            for name, row in document["artifacts"].items()]
    return "\n".join([
        "# Phase-33 table parity audit", "",
        f"**PASS.** All {document['live_artifact_count']} Phase-26 through Phase-31 artifacts match the Phase-32 table's SHA-256 and byte sizes. All {document['receipt_comparison_count']} shared metadata comparisons with Phase-30 and Phase-31 receipts agree.", "",
        "| Table artifact | Bytes | SHA-256 | Live parity |", "| --- | ---: | --- | --- |", *rows, "",
        "Phase 32 is recomputed from its fixed twelve live files and compared in full, including line counts, totals and host-path checks. The Phase-30 manifest and Phase-31 receipt are also recomputed and checked in full using their existing static verifiers. Shared metadata comparisons cover eight Phase-30 artifact rows, eight Phase-31 artifact rows, four Phase-31 input receipt pins and ten Phase-31 comparison rows whose paths occur in the table.", "",
        "Four Phase-31 comparison rows describe two historical scripts outside the Phase-32 file set. Their saved consistency is checked by the Phase-31 verifier; they are listed separately and are not counted as table comparisons. Current script bytes are outside this audit. No earlier receipt or report is refreshed, and earlier conclusions remain as recorded.", "",
        "The JSON pins all three input receipts and lists every table comparison. Inputs are parsed strictly and reread to detect changes during scanning. Missing files, symlinks, inconsistent metadata, duplicate keys, nonfinite JSON and host paths in the artifact set fail. Paths in the outputs are repository-relative. --repo-root supports a relocated source tree without Git or source execution.", "",
        "Normal verification compares the complete saved Phase-33 JSON and exact report text without writes. --write replaces only the two Phase-33 outputs.", "",
        "```sh", "python3 scripts/audit_rsv2_07_hm_forensics.py --phase 33",
        "python3 -m unittest discover -s scripts -p 'test_audit_rsv2_07_hm_forensics.py' -v", "git diff --check", "```", "",
        "Execution uses CPUs 14,15 at nice 10. This is a static table parity audit. Changes are left uncommitted.", "",
    ])


def run_table_parity(args):
    require(args.artifact_root is None, "Phase-33 uses saved outputs in --repo-root; --artifact-root is not supported")
    current = table_parity(args.repo_root)
    report = render_table_parity(current)
    receipt = json.dumps(current, indent=2) + "\n"
    require_portable(report, "Phase-33 report")
    require_portable(receipt, "Phase-33 receipt")
    if args.write:
        (args.repo_root / DIRECTORY).mkdir(parents=True, exist_ok=True)
        (args.repo_root / TABLE_PARITY_RECEIPT).write_text(receipt, encoding="utf-8")
        (args.repo_root / TABLE_PARITY_REPORT).write_text(report, encoding="utf-8")
    else:
        saved = strict_json((args.repo_root / TABLE_PARITY_RECEIPT).read_bytes())
        require(json.dumps(saved, sort_keys=True) == json.dumps(current, sort_keys=True), "Phase-33 saved receipt mismatch")
        require((args.repo_root / TABLE_PARITY_REPORT).read_text(encoding="utf-8") == report, "Phase-33 report mismatch")
    print(f"LANE: RSV2-07-hm | RSV2-07-p33 | PASS | live_artifacts={current['live_artifact_count']} | receipt_comparisons={current['receipt_comparison_count']}")
    return 0


def audit(artifact_root):
    artifact_root = Path(artifact_root)
    sources, waves = {}, {}
    for wave in WAVES:
        name = f"out/{wave}/evolution_log.jsonl"
        data = read_optional(artifact_root / name)
        sources[name] = data
        records = None if data is None else parse_jsonl(data)
        waves[wave] = {"path": name, "availability": "missing" if data is None else "present",
                       "sha256": None if data is None else hashlib.sha256(data).hexdigest(),
                       "size_bytes": None if data is None else len(data),
                       "jsonl_records": None if records is None else len(records),
                       "round0_elites_count": None, "round1_births_count": None,
                       "lineage_status": "missing_source_log" if data is None else "event_schema_not_reviewed"}
    for name, data in sources.items():
        require(read_optional(artifact_root / name) == data, "Saved-log availability or bytes changed during audit")
    missing = sum(row["availability"] == "missing" for row in waves.values())
    return {"schema": "jarambit.rsv2_07_hm.phase26_lineage_availability.v1", "task": "RSV2-07-p26",
            "status": "partial_missing_source_logs" if missing else "partial_unreviewed_event_schema",
            "requested_scope": "static_round1_births_vs_round0_elites_counts",
            "completed_scope": "source_availability_and_strict_jsonl_integrity_only",
            "lineage_counts_established": False, "missing_logs": missing, "available_logs": len(waves) - missing,
            "waves": waves, "round0_elites_total": None, "round1_births_total": None,
            "required_evidence": "saved evolution logs and reviewed producer event schema identifying rounds, births and elites",
            "limitations": ["missing and unclassified counts are null, never inferred zero",
                            "round birth/screen JSON snapshots are not substituted for the requested logs",
                            "no lineage identifiers, metrics, new candidates or simulations are generated"]}


def render(document):
    rows = [f"| {wave} | {row['path']} | {row['availability']} | {row['sha256'] or 'unavailable'} | {row['size_bytes'] if row['size_bytes'] is not None else 'null'} | null | null |"
            for wave, row in document["waves"].items()]
    return "\n".join([
        "# Phase-26 Round-1 births versus Round-0 elites lineage audit", "",
        f"**PARTIAL — lineage counts are not established.** {document['missing_logs']} of six requested evolution_log.jsonl files are absent; {document['available_logs']} are available. Counts remain null because saved source records and a reviewed event schema are required. This report does not claim a successful lineage count audit.", "",
        "| Wave | Requested relative log | Availability | SHA-256 | Bytes | Round-0 elites | Round-1 births |",
        "| --- | --- | --- | --- | ---: | ---: | ---: |", *rows, "",
        "The initial inspection found all six logs absent in both the selected worktree and common checkout, with no matching evolution logs in the inspected saved wave archives. The requested forensics script and its tests were also absent; this task adds a minimal standalone verifier and tests rather than claiming to extend an existing implementation.", "",
        "The verifier checks the selected source root, records exact SHA-256 and byte size for any present logs, and parses JSONL object records strictly. Duplicate keys, nonfinite numbers (including overflow), blank records and non-object records fail. Empty files have zero JSONL records but do not establish zero elites or births. Present records remain unclassified until their producer schema and round/event meaning are reviewed. No speculative event profile is treated as historical evidence.", "",
        "Source bytes and availability are checked again after scanning. A normal run verifies the complete saved receipt and exact report text without source writes; --write explicitly writes these two audit outputs. --artifact-root accepts a relocated saved repository without Git discovery; the default uses the common Git repository. --repo-root selects the report checkout. Persisted paths are repository-relative. Exit status 2 and a PARTIAL LANE line mean lineage evidence is still unavailable; malformed or mismatching evidence exits 1.", "",
        "The request is limited to static lineage counts. Other birth/screen snapshots are not used to infer the absent log's elite events. No source events, lineage relationships, engineering calculations, geometry changes or simulations are generated. Completing the count portion requires the six saved logs and their event schema.", "",
        "```sh", "python3 scripts/audit_rsv2_07_hm_forensics.py",
        "python3 -m unittest discover -s scripts -p 'test_audit_rsv2_07_hm_forensics.py' -v", "git diff --check", "```", "",
        "Execution uses CPUs14,15 at inherited nice10. Changes are left uncommitted.", "",
    ])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--phase", choices=("26", "27", "28", "29", "30", "31", "32", "33"), default="26")
    parser.add_argument("--artifact-root", type=Path)
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    try:
        if args.phase == "33":
            return run_table_parity(args)
        if args.phase == "32":
            return run_cumulative(args)
        if args.phase == "31":
            return run_manifest_cross(args)
        if args.phase == "30":
            return run_summary(args)
        if args.phase == "29":
            return run_cross(args)
        if args.phase == "28":
            return run_scripts(args)
        if args.phase == "27":
            require(args.artifact_root is None, "Phase-27 uses tracked files in --repo-root; --artifact-root is not supported")
            return run_digest(args)
        artifact_root = args.artifact_root
        if artifact_root is None:
            common = subprocess.check_output(["git", "-C", str(args.repo_root), "rev-parse", "--path-format=absolute", "--git-common-dir"],
                                             text=True, stderr=subprocess.PIPE).strip()
            artifact_root = Path(common).parent
        current = audit(artifact_root)
        if args.write:
            (args.repo_root / DIRECTORY).mkdir(parents=True, exist_ok=True)
            (args.repo_root / RECEIPT).write_text(json.dumps(current, indent=2) + "\n")
            (args.repo_root / REPORT).write_text(render(current))
        else:
            saved = strict_json((args.repo_root / RECEIPT).read_bytes())
            require(json.dumps(saved, sort_keys=True) == json.dumps(current, sort_keys=True), "Phase-26 saved receipt mismatch")
            require((args.repo_root / REPORT).read_text() == render(current), "Phase-26 report mismatch")
    except (OSError, ValueError, SyntaxError, subprocess.CalledProcessError) as error:
        parser.exit(1, f"Phase-{args.phase} static integrity: FAIL: {error}\n")
    print(f"LANE: RSV2-07-hm | RSV2-07-p26 | PARTIAL | missing_logs={current['missing_logs']}/6 | lineage_counts=unestablished")
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
