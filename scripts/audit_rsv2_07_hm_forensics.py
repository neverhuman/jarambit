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
    parser.add_argument("--phase", choices=("26", "27", "28"), default="26")
    parser.add_argument("--artifact-root", type=Path)
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    try:
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
