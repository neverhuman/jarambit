#!/usr/bin/env python3
"""Verify saved Phase-8 provenance and synthetic checks without requiring target/."""

import hashlib
import json
from pathlib import Path

from portable_audit import RAW, require, summary, verify_manifest

ROOT = Path(__file__).resolve().parents[2]
RECEIPT = ROOT / "experiments/rsv2_07/results/phase8_synthetic_gci_audit.json"


def verify():
    saved = json.loads(RECEIPT.read_text())
    require(saved["schema"] == "jarambit.rsv2_07.phase8_synthetic_gci.v1", "Invalid Phase-8 schema")
    require(saved["execution_status"] == "complete" and saved["all_checks_pass"] is True,
            "Incomplete synthetic receipt")
    require(saved["native_process_replay_bytes_equal"] is True, "Original native replay did not pass")
    records = verify_manifest(ROOT, saved["sha256"], saved["artifact_bytes"], saved["study_sha256"], saved["commit"])
    from phase8_synthetic_gci_check import bits, evaluate, manufactured_cases, run_probe, triplets
    cases = manufactured_cases()
    require(saved["sequence_count"] == len(cases) == len(saved["sequences"]) == 24, "Wrong sequence count")
    native_saved = [{key: row[key] for key in ("id", "estimate", "float_bits")}
                    for case in saved["sequences"] for row in case["triplets"]]
    payload = (json.dumps(triplets(cases), sort_keys=True) + "\n").encode()
    require([row["id"] for row in native_saved] == [row["id"] for row in triplets(cases)], "Incorrect triplet IDs")
    for row in native_saved:
        require(all(bits(row["estimate"][key]) == value for key, value in row["float_bits"].items()),
                f"JSON/IEEE-754 bit mismatch: {row['id']}")
    results, counts = evaluate(cases, native_saved)
    require((results, counts) == evaluate(cases, native_saved), "Nondeterministic synthetic reference")
    require(results == saved["sequences"] and counts == saved["status_counts"], "Saved synthetic results differ")
    require(saved["triplet_count"] == sum(counts.values()) == 39, "Wrong triplet count")
    raw = saved["raw_directory"]
    require(raw.startswith(RAW), "Invalid raw directory")
    input_name = f"{raw}/input.json"
    require(hashlib.sha256(payload).hexdigest() == saved["sha256"][input_name]
            and len(payload) == saved["artifact_bytes"][input_name], "Synthetic input binding mismatch")
    raw_status = {record["path"]: record["mode"] for record in records if record["path"].startswith(RAW)}
    for suffix in ("native_first.json", "native_second.json"):
        name = f"{raw}/{suffix}"
        if raw_status[name] == "on_disk":
            require(json.loads((ROOT / name).read_bytes()) == native_saved, "Saved native log differs from receipt")
    binary = f"{raw}/synthetic-gci-probe"
    replay = raw_status[binary] == "on_disk"
    if replay:
        first, second = run_probe(ROOT / binary, payload), run_probe(ROOT / binary, payload)
        require(first == second, "Native process replay differs")
        for suffix in ("native_first.json", "native_second.json"):
            name = f"{raw}/{suffix}"
            require(hashlib.sha256(first).hexdigest() == saved["sha256"][name]
                    and len(first) == saved["artifact_bytes"][name], "Native replay digest mismatch")
        require(json.loads(first) == native_saved, "Native replay differs from saved synthetic results")
    return records, replay


def main():
    if not __debug__:
        raise RuntimeError("Run this assertion-based checker without Python -O")
    records, replay = verify()
    print(f"Phase-8 provenance: PASS; {summary(records)}.")
    print("PASS: 24 synthetic sequences; 39 triplets; seven fit statuses; recorded Rust/Python bits agree.")
    print("Native replay: PASS (two process executions)." if replay else
          "Native replay: NOT RUN (saved binary absent); Python checks against recorded Rust results: PASS.")


if __name__ == "__main__":
    main()
