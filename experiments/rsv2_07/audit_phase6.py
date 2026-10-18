#!/usr/bin/env python3
"""Read-only receipt schema, file integrity and report transcription checks."""

import json
import math
from pathlib import Path
import re

from portable_audit import RAW, check_file, require, summary

ROOT = Path(__file__).resolve().parents[2]
HERE = "experiments/rsv2_07/"
MANIFEST = HERE + "results/phase10_portable_receipt_manifest.json"
REPORT = HERE + "PHASE10_PORTABLE_RECEIPT_AUDIT.md"
PHASE6_REPORT = HERE + "PHASE6_UNIFIED_SCORECARD.md"
SPECS = {
    "phase6_unified_scorecard.json": (
        "jarambit.rsv2_07.phase6_unified_scorecard.v1",
        {"schema": str, "task": str, "candidates": dict, "gate_definitions": dict,
         "provenance": dict, "report": dict, "artifacts_sha256": dict, "verification": dict},
    ),
    "solid_tip_3d_validation.json": (
        "jarambit.rsv2_07.solid_tip_3d_validation.v2",
        {"schema": str, "task": str, "status": str, "candidate": str,
         "provenance": dict, "primary_converged": bool, "primary": dict,
         "dolfinx_cross_check": dict, "artifacts_sha256": dict, "verification": dict},
    ),
    "validation.json": (
        "legacy.validation.v1 (structural)",
        {"cert_integrity_tests": str, "prototype_only": bool, "all_gate_qualification": bool,
         "verified_invocations": int, "coarse_evaluations": int, "source_provenance": dict,
         "runtime_provenance": dict, "workspace_drift": dict, "study_support_sha256": dict,
         "external_inputs_sha256": dict, "validation": list, "cad_exports": list},
    ),
}


def load_json(text):
    def pairs(items):
        result = {}
        for key, value in items:
            require(key not in result, f"Duplicate JSON key: {key}")
            result[key] = value
        return result

    def invalid(value):
        raise ValueError(f"Non-finite JSON number: {value}")

    def finite_float(value):
        number = float(value)
        require(math.isfinite(number), f"Non-finite JSON number: {value}")
        return number

    return json.loads(text, object_pairs_hook=pairs, parse_constant=invalid, parse_float=finite_float)


def check_digest_fields(value, path="root"):
    count = 0
    if isinstance(value, dict):
        for key, item in value.items():
            location = f"{path}/{key}"
            if key == "production_surface_matches_3d_source_sha256":
                # Historical verification flag, not a digest despite its suffix.
                require(type(item) is bool, f"Expected verification flag: {location}")
            elif key == "sha256" or key.endswith("_sha256"):
                entries = item.values() if isinstance(item, dict) else [item]
                for sha in entries:
                    require(isinstance(sha, str) and re.fullmatch(r"[0-9a-f]{64}", sha) is not None,
                            f"Invalid SHA-256: {location}")
                    count += 1
            count += check_digest_fields(item, location)
    elif isinstance(value, list):
        count += sum(check_digest_fields(item, f"{path}/{index}") for index, item in enumerate(value))
    return count


def check_schema(name, doc):
    require(isinstance(doc, dict), f"Expected JSON object: {name}")
    schema, fields = SPECS[name]
    for field, kind in fields.items():
        require(field in doc and type(doc[field]) is kind, f"Missing/invalid {name}:{field}")
    if name != "validation.json":
        require(doc["schema"] == schema, f"Unexpected schema: {name}")
    else:
        require("schema" not in doc, "Legacy validation receipt unexpectedly declares a schema")
        for key in ("verified_invocations", "coarse_evaluations"):
            require(doc[key] >= 0, f"Negative metadata count: {key}")
    for flag in ("prototype_only", "all_gate_qualification"):
        require(type(doc.get(flag)) is bool, f"Missing/invalid scope flag: {name}:{flag}")
    if name == "phase6_unified_scorecard.json":
        families = {"engineering_gates": [f"G16-{i:02}" for i in range(1, 17)],
                    "rs_v2_gates": [f"RSV2-{i:02}" for i in range(1, 11)]}
        require(set(doc["gate_definitions"]) == set(sum(families.values(), [])), "Invalid gate definition IDs")
        require(bool(doc["candidates"]), "Missing candidate records")
        for candidate in doc["candidates"].values():
            require(isinstance(candidate, dict), "Invalid candidate record")
            for family, ids in families.items():
                require(set(candidate.get(family, {})) == set(ids), f"Invalid {family} IDs")
                for gate in candidate[family].values():
                    require(isinstance(gate, dict) and isinstance(gate.get("key"), str)
                            and gate.get("status") in ("pass", "fail"), "Invalid stored gate record")
            for field in ("engineering_pass_count", "rs_v2_pass_count"):
                require(type(candidate.get(field)) is int and candidate[field] >= 0, f"Invalid {field}")
    return check_digest_fields(doc, name)


def tables(text):
    result, rows = [], []
    for line in text.splitlines() + [""]:
        if line.strip().startswith("|") and line.strip().endswith("|"):
            rows.append([cell.strip().strip("`") for cell in line.strip()[1:-1].split("|")])
        elif rows:
            require(len(rows) >= 2 and all(re.fullmatch(r":?-+:?", cell) for cell in rows[1]),
                    "Malformed Markdown table")
            require(all(len(row) == len(rows[0]) for row in rows), "Inconsistent Markdown table width")
            result.append((rows[0], rows[2:]))
            rows = []
    return result


def check_phase6_tables(doc, text):
    parsed = tables(text)
    overview = [rows for header, rows in parsed if header[:3] == ["Candidate", "Engineering", "RS-v2"]]
    expected = [[name, f"{candidate['engineering_pass_count']}/16", f"{candidate['rs_v2_pass_count']}/10"]
                for name, candidate in doc["candidates"].items()]
    require(len(overview) == 1 and [row[:3] for row in overview[0]] == expected,
            "Phase-6 overview table/JSON mismatch")
    header = ["ID", "Gate", *doc["candidates"]]
    grids = [rows for columns, rows in parsed if columns == header]
    require(len(grids) == 2, "Missing Phase-6 gate tables")
    for family, rows in zip(("engineering_gates", "rs_v2_gates"), grids):
        first = next(iter(doc["candidates"].values()))[family]
        expected = [[gate_id, gate["key"], *[c[family][gate_id]["status"] for c in doc["candidates"].values()]]
                    for gate_id, gate in first.items()]
        require(rows == expected, f"Phase-6 {family} table/JSON mismatch")


def check_inventory(docs, hashes, text):
    expected = [[name, SPECS[name][0], hashes[HERE + "results/" + name],
                 str(len(doc.get("artifacts_sha256", {})))] for name, doc in docs.items()]
    rows = [rows for header, rows in tables(text)
            if header == ["Receipt", "Schema", "SHA-256", "Raw artifact entries"]]
    require(rows == [expected], "Phase-10 inventory table/JSON mismatch")


def verify():
    manifest = load_json((ROOT / MANIFEST).read_text())
    require(manifest["schema"] == "jarambit.rsv2_07.portable_receipt_manifest.v1", "Invalid integrity manifest")
    required_files = {HERE + "results/" + name for name in SPECS} | {
        REPORT, PHASE6_REPORT, HERE + "audit_phase6.py", HERE + "test_portable_audits.py", HERE + "portable_audit.py"}
    require(set(manifest["files"]) == required_files, "Incomplete tracked input manifest")
    records = [check_file(ROOT, name, sha) for name, sha in manifest["files"].items()]
    docs = {name: load_json((ROOT / HERE / "results" / name).read_text()) for name in SPECS}
    counts = {name: check_schema(name, doc) for name, doc in docs.items()}
    require(counts == manifest["digest_reference_counts"], "Digest reference count mismatch")
    digest_count = sum(counts.values())
    raw_hashes = {}
    for doc in docs.values():
        for name, sha in doc.get("artifacts_sha256", {}).items():
            require(name.startswith(RAW), f"Unexpected raw artifact location: {name}")
            require(name not in raw_hashes or raw_hashes[name] == sha, f"Conflicting artifact digests: {name}")
            raw_hashes[name] = sha
    require(set(raw_hashes) == set(manifest["raw_artifacts"]), "Raw manifest coverage mismatch")
    for name, sha in raw_hashes.items():
        record = manifest["raw_artifacts"][name]
        require(record["sha256"] == sha, f"Receipt/raw manifest digest mismatch: {name}")
        records.append(check_file(ROOT, name, sha, record["size_bytes"]))
    phase6 = docs["phase6_unified_scorecard.json"]
    require(phase6["report"] == {"path": PHASE6_REPORT, "sha256": manifest["files"][PHASE6_REPORT]},
            "Phase-6 report binding mismatch")
    check_phase6_tables(phase6, (ROOT / PHASE6_REPORT).read_text())
    check_inventory(docs, manifest["files"], (ROOT / REPORT).read_text())
    return records, digest_count


def main():
    records, count = verify()
    print(f"Receipt integrity/schema: PASS; 3 receipts; {count} SHA-256 references; {summary(records)}.")
    print("Report transcription: PASS (overview counts, 26 stored gate rows, receipt inventory).")
    print("No simulations or physical qualification checks performed.")


if __name__ == "__main__":
    main()
