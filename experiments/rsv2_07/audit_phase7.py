#!/usr/bin/env python3
"""Verify Phase-6 saved file provenance; does not perform a GCI audit."""

import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import platform

from portable_audit import digest, require, study_manifest, summary, verify_manifest

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "experiments/rsv2_07/results/phase6_unified_scorecard.json"
SOURCE_SHA256 = "5d301a3b33afdd8470441c0094f79e47915f8f9e1161af035b90ac37223f7dd7"
REPORT = ROOT / "experiments/rsv2_07/PHASE7_SAVED_TRIPLET_GCI_AUDIT.md"
RECEIPT = ROOT / "experiments/rsv2_07/results/phase7_saved_triplet_gci_audit.json"


def verify():
    source_bytes = SOURCE.read_bytes()
    require(hashlib.sha256(source_bytes).hexdigest() == SOURCE_SHA256,
            "Phase-6 receipt differs from the pinned saved receipt")
    source = json.loads(source_bytes)
    saved = json.loads(RECEIPT.read_text())
    require(saved["schema"] == "jarambit.rsv2_07.phase7_file_integrity.v1", "Invalid Phase-7 schema")
    require(saved["assignment_status"] == "partial_hash_verification_only"
            and saved["gci_analysis_status"] == "not_performed", "Unexpected Phase-7 scope")
    groups = {
        "source": source["provenance"]["source_sha256"],
        "raw_artifact": source["artifacts_sha256"],
        "report": {source["report"]["path"]: source["report"]["sha256"]},
        "postprocessor": {"experiments/rsv2_07/phase6_scorecard.py": source["postprocessor_sha256"]},
    }
    expected = {(group, name): sha for group, entries in groups.items() for name, sha in entries.items()}
    recorded = {(entry["group"], entry["path"]): entry["expected_sha256"] for entry in saved["checks"]}
    require(recorded == expected and len(recorded) == len(saved["checks"]), "Phase-6/7 manifest mismatch")
    require(saved["file_integrity_pass"] is True and all(
        entry["matches"] is True and entry["read_error"] is None
        and entry["actual_sha256"] == entry["expected_sha256"] for entry in saved["checks"]),
        "Invalid original file-integrity observations")
    require(saved["counts"] == {group: len(entries) for group, entries in groups.items()}, "Wrong snapshot counts")
    require(saved["source_receipt"] == {"path": str(SOURCE.relative_to(ROOT)), "sha256": SOURCE_SHA256},
            "Incorrect source receipt binding")
    hashes = {name: sha for (_, name), sha in expected.items()}
    require(digest(Path(__file__)) == saved["verifier_sha256"], "Phase-7 verifier SHA-256 mismatch")
    require(saved["report"]["path"] == str(REPORT.relative_to(ROOT)), "Incorrect report binding")
    require(digest(REPORT) == saved["report"]["sha256"], "Phase-7 report SHA-256 mismatch")
    records = verify_manifest(ROOT, hashes, saved["artifact_bytes"], saved["study_sha256"],
                              source["provenance"]["commit"])
    return saved, records


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--write", action="store_true", help="Refresh execution metadata after successful verification")
    args = parser.parse_args()
    saved, records = verify()
    if args.write:
        saved["timestamp_utc"] = datetime.now(timezone.utc).isoformat()
        saved["runtime"] = {"cpu_affinity": sorted(os.sched_getaffinity(0)),
                            "nice": os.getpriority(os.PRIO_PROCESS, 0), "python": platform.python_version(),
                            "CARGO_BUILD_JOBS": os.environ.get("CARGO_BUILD_JOBS")}
        saved["portable_verification"] = records
        saved["study_sha256"] = study_manifest(ROOT)
        RECEIPT.write_text(json.dumps(saved, indent=2) + "\n")
    print(f"File provenance: PASS; {len(saved['checks'])} snapshot entries; {summary(records)}.")
    print("Phase-7 status: partial; GCI/Richardson analysis not performed.")


if __name__ == "__main__":
    main()
