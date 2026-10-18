#!/usr/bin/env python3
"""Verify saved Phase-6 file hashes only; does not perform a GCI audit."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import sys
from datetime import datetime, timezone


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "experiments/rsv2_07/results/phase6_unified_scorecard.json"
SOURCE_SHA256 = "5d301a3b33afdd8470441c0094f79e47915f8f9e1161af035b90ac37223f7dd7"
REPORT = ROOT / "experiments/rsv2_07/PHASE7_SAVED_TRIPLET_GCI_AUDIT.md"
RECEIPT = ROOT / "experiments/rsv2_07/results/phase7_saved_triplet_gci_audit.json"


def digest(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def verify():
    source_bytes = SOURCE.read_bytes()
    actual_source_sha256 = hashlib.sha256(source_bytes).hexdigest()
    if actual_source_sha256 != SOURCE_SHA256:
        raise ValueError("Phase-6 receipt differs from the pinned saved receipt")
    saved = json.loads(source_bytes)
    groups = {
        "source": saved["provenance"]["source_sha256"],
        "raw_artifact": saved["artifacts_sha256"],
        "report": {saved["report"]["path"]: saved["report"]["sha256"]},
        "postprocessor": {
            "experiments/rsv2_07/phase6_scorecard.py": saved["postprocessor_sha256"]
        },
    }
    checks = []
    for group, entries in groups.items():
        if not entries:
            raise ValueError(f"Empty hash manifest: {group}")
        for name, expected in sorted(entries.items()):
            path = Path(name)
            if not path.is_absolute():
                path = ROOT / path
            try:
                actual = digest(path)
                error = None
            except OSError as exc:
                actual = None
                error = str(exc)
            checks.append({
                "group": group, "path": name, "expected_sha256": expected,
                "actual_sha256": actual, "matches": actual == expected,
                "read_error": error,
            })
    return {
        "schema": "jarambit.rsv2_07.phase7_file_integrity.v1",
        "task": "RSV2-07-p7",
        "assignment_status": "partial_hash_verification_only",
        "file_integrity_pass": all(check["matches"] for check in checks),
        "gci_analysis_status": "not_performed",
        "scope": "File-byte integrity against the saved Phase-6 manifest only.",
        "limitations": [
            "No GCI or Richardson extrapolation calculations were performed.",
            "No structural-performance or convergence conclusions are established.",
            "Hash matches establish byte identity, not scientific correctness or authenticity.",
            "Only manifest-listed files are covered; this is not a directory census.",
        ],
        "new_geometries": 0,
        "new_simulations": 0,
        "timestamp_utc": datetime.now(timezone.utc).isoformat(),
        "runtime": {
            "cpu_affinity": sorted(os.sched_getaffinity(0)),
            "nice": os.getpriority(os.PRIO_PROCESS, 0),
            "python": platform.python_version(),
            "CARGO_BUILD_JOBS": os.environ.get("CARGO_BUILD_JOBS"),
        },
        "source_receipt": {
            "path": str(SOURCE.relative_to(ROOT)),
            "sha256": actual_source_sha256,
        },
        "verifier_sha256": digest(Path(__file__).resolve()),
        "counts": {group: len(entries) for group, entries in groups.items()},
        "checks": checks,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--write", action="store_true", help="Write partial audit report and JSON")
    args = parser.parse_args()
    result = verify()
    failed = [item for item in result["checks"] if not item["matches"]]
    status = "PASS" if not failed else "FAIL"
    if args.write:
        counts = result["counts"]
        report = (
            "# Phase-7 saved-file integrity audit — partial assignment\n\n"
            "Only the file-integrity portion was executed. The requested triplet GCI / "
            "Richardson analysis was not performed. This report establishes no new "
            "structural-performance or convergence result.\n\n"
            f"File-byte checks: **{status}**; {len(result['checks'])} manifest entries "
            f"({counts['source']} source, {counts['raw_artifact']} raw artifact, "
            f"{counts['report']} report, {counts['postprocessor']} postprocessor).\n\n"
            f"Pinned Phase-6 receipt SHA-256: `{SOURCE_SHA256}`. "
            "Matches establish byte identity against this saved manifest; they do not "
            "establish scientific correctness, authenticity or complete directory coverage.\n\n"
            f"Executed {result['timestamp_utc']} with CPU affinity "
            f"`{result['runtime']['cpu_affinity']}`, nice "
            f"`{result['runtime']['nice']}`, Python `{result['runtime']['python']}`, "
            f"CARGO_BUILD_JOBS=`{result['runtime']['CARGO_BUILD_JOBS']}`. "
            "No new geometry, FEM solve or build was run.\n\n"
            "Re-run the read-only checks from the checkout:\n\n"
            "```sh\n"
            "taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/audit_phase7.py\n"
            "```\n\n"
            "CPU pair `8,9` can be substituted when available. Add `--write` to regenerate "
            "this partial report and receipt. The verifier returns success only for file "
            "integrity; that exit code does not mean the Phase-7 GCI assignment is complete.\n\n"
            "Per-file expected/actual SHA-256 digests and read errors are recorded in "
            "`results/phase7_saved_triplet_gci_audit.json`.\n"
        )
        REPORT.write_text(report)
        result["report"] = {"path": str(REPORT.relative_to(ROOT)), "sha256": digest(REPORT)}
        RECEIPT.write_text(json.dumps(result, indent=2) + "\n")
    print(f"File integrity: {status}; {len(result['checks'])} checked, {len(failed)} failed.")
    print("Phase-7 status: partial; GCI/Richardson analysis not performed.")
    for item in failed:
        print(json.dumps(item))
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
