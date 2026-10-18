#!/usr/bin/env python3
"""Static saved-log integrity for Phase-26; do not infer unavailable lineage counts."""

import argparse
import hashlib
import json
import math
from pathlib import Path
import subprocess

DIRECTORY = "docs/audits/rsv2_07_hm/"
RECEIPT = DIRECTORY + "phase26_round1_births_vs_elites_lineage_audit.json"
REPORT = DIRECTORY + "PHASE26_ROUND1_BIRTHS_VS_ELITES_LINEAGE_AUDIT.md"
WAVES = tuple(f"w{number}" for number in range(315, 321))


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
    parser.add_argument("--artifact-root", type=Path)
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    try:
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
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        parser.exit(1, f"Phase-26 saved-log integrity: FAIL: {error}\n")
    print(f"LANE: RSV2-07-hm | RSV2-07-p26 | PARTIAL | missing_logs={current['missing_logs']}/6 | lineage_counts=unestablished")
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
