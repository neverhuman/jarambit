#!/usr/bin/env python3
"""Check native GCI using 24 manufactured sequences, without FEM or design data."""

import argparse
from collections import Counter
from dataclasses import replace
from datetime import datetime, timezone
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import struct
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
HERE = ROOT / "experiments/rsv2_07"
RECEIPT = HERE / "results/phase8_synthetic_gci_audit.json"
REPORT = HERE / "PHASE8_SYNTHETIC_GCI_AUDIT.md"
sys.path.insert(0, str(ROOT))
from fem_solid.gci import Level, MeshRef, Observable, compute_gci  # noqa: E402


def digest(path):
    with Path(path).open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def bits(value):
    return None if value is None else struct.pack(">d", value).hex()


def manufactured_cases():
    cases = []
    for r in (2.0, math.sqrt(2.0)):
        for index, p in enumerate((0.5, 1.0, 1.5, 2.0, 2.5, 3.0)):
            c = 0.125 * (-1 if index % 2 else 1)
            h = [0.25 * r**i for i in range(5)]
            cases.append({
                "id": f"polynomial_r{r.hex()}_p{p}", "r": r, "fs": 1.25,
                "values": [2.0 + c * x**p for x in h], "h": h,
                "p_exact": p, "f0_exact": 2.0, "c": c,
                "level_mode": "valid" if r == 2.0 else "none",
                "expected": "admissible_estimate",
            })
    ulp = math.ulp(1.0)
    controls = [
        ("oscillatory_three", [2., 3., 2.], 2., "oscillatory", "none"),
        ("oscillatory_five", [2., 1., 2., 1., 2.], math.sqrt(2.), "oscillatory", "none"),
        ("constant", [2.] * 5, 2., "unresolved", "none"),
        ("fine_plateau", [2., 2., 3.], 2., "unresolved", "none"),
        ("coarse_plateau", [2., 3., 3.], 2., "unresolved", "none"),
        ("negative_order", [2., 3., 3.5, 3.75, 3.875], 2., "nonpositive_order", "none"),
        ("zero_order", [2., 3., 4.], 2., "nonpositive_order", "none"),
        ("ratio_one", [2., 3., 5.], 1., "invalid_input", "none"),
        ("ratio_below_one", [2., 3., 5.], 0.5, "invalid_input", "none"),
        ("precision_floor", [1., 1. + ulp, 1. + 4. * ulp], 2., "precision_limited", "none"),
        ("zero_reference", [0., 1., 3.], 2., "unresolved", "none"),
        ("duplicate_synthetic_content", [2., 3., 5.], 2., "unsupported_sequence", "duplicate_content"),
    ]
    for name, values, r, expected, mode in controls:
        cases.append({"id": name, "values": values, "r": r, "fs": 1.25,
                      "level_mode": mode, "expected": expected})
    assert len(cases) == 24
    return cases


def triplets(cases):
    result = []
    for case in cases:
        for start in ([0, 2] if len(case["values"]) == 5 else [0]):
            result.append({
                "id": f"{case['id']}/slice{start}",
                "values_bits": [bits(x) for x in case["values"][start:start + 3]],
                "r_bits": bits(case["r"]), "fs_bits": bits(case["fs"]),
                "level_mode": case["level_mode"],
            })
    return result


def python_estimate(case, start):
    levels = None
    if case["level_mode"] != "none":
        obs = Observable("synthetic_scalar", "dimensionless", "synthetic", "algebra", "scalar")
        r = case["r"]
        levels = [Level(MeshRef(f"synthetic-{i}", "synthetic-v1", f"{i + 1:064x}"), h, obs)
                  for i, h in enumerate((1., r, r * r))]
        if case["level_mode"] == "duplicate_content":
            levels[1] = replace(levels[1], mesh=replace(levels[1].mesh, sha256=levels[0].mesh.sha256))
    return compute_gci(*case["values"][start:start + 3], case["r"], case["fs"], levels=levels)


def evaluate(cases, native):
    results = []
    native = {entry["id"]: entry for entry in native}
    for case in cases:
        differences = [b - a for a, b in zip(case["values"], case["values"][1:])]
        signs = [0 if d == 0 else (1 if d > 0 else -1) for d in differences]
        item = {**case, "difference_signs": signs,
                "strictly_monotone": len(set(signs)) == 1 and 0 not in signs,
                "adjacent_sign_change": any(a * b < 0 for a, b in zip(signs, signs[1:])),
                "triplets": []}
        for start in ([0, 2] if len(case["values"]) == 5 else [0]):
            key = f"{case['id']}/slice{start}"
            row = native[key]
            py = python_estimate(case, start)
            expected_bits = {key: bits(getattr(py, key)) for key in ("p", "f_ext", "gci_fine")}
            assert row["float_bits"] == expected_bits, (key, "Rust/Python bit mismatch", row, expected_bits)
            py_fields = json.loads(json.dumps(py.to_dict()))
            native_fields = dict(row["estimate"])
            for name in expected_bits:
                del py_fields[name]
                del native_fields[name]
            assert native_fields == py_fields, (key, "Rust/Python metadata mismatch")
            assert py.fit_status.value == case["expected"], (key, py.fit_status)
            assert py.monotone == (case["expected"] in {"admissible_estimate", "nonpositive_order"}), key
            assert py.relative_error_available == (case["expected"] == "admissible_estimate"), key
            assert py.admitted == (case["expected"] == "admissible_estimate" and case["level_mode"] == "valid"), key
            assert py.asymptotic_validation == "not_established"
            checks = {"rust_python_float_bits_equal": True, "metadata_equal": True,
                      "expected_status": True, "expected_monotonicity_and_availability": True}
            if "p_exact" in case:
                p, f0, c, h = case["p_exact"], case["f0_exact"], case["c"], case["h"][start]
                true_error = abs(case["values"][start] - f0) / abs(case["values"][start])
                exact_gci = case["fs"] * abs(c * h**p) / abs(case["values"][start])
                assert math.isclose(py.p, p, rel_tol=1e-11, abs_tol=1e-11), key
                assert math.isclose(py.f_ext, f0, rel_tol=1e-11, abs_tol=1e-11), key
                assert math.isclose(py.gci_fine, exact_gci, rel_tol=1e-11, abs_tol=1e-13), key
                assert true_error <= py.gci_fine + 1e-13, key
                checks.update({"order_recovered": True, "extrapolate_recovered": True,
                               "known_error_bounded": True, "analytic_gci_agrees": True})
                row = {**row, "known_relative_error": true_error, "analytic_gci": exact_gci}
            item["triplets"].append({**row, "slice_start": start, "checks": checks})
        results.append(item)
    statuses = Counter(t["estimate"]["fit_status"] for c in results for t in c["triplets"])
    assert set(statuses) == {"admissible_estimate", "invalid_input", "unresolved",
                             "precision_limited", "oscillatory", "nonpositive_order", "unsupported_sequence"}
    return results, dict(sorted(statuses.items()))


def run_probe(binary, payload):
    return subprocess.run([str(binary)], input=payload, stdout=subprocess.PIPE, check=True).stdout


def build(raw, env):
    with (raw / "cargo.jsonl").open("w") as out, (raw / "cargo.log").open("w") as err:
        subprocess.run(["cargo", "build", "-p", "jarambit-crescent", "--lib", "--message-format=json"],
                       cwd=ROOT, env=env, stdout=out, stderr=err, check=True)
    artifacts = {}
    for line in (raw / "cargo.jsonl").read_text().splitlines():
        event = json.loads(line)
        if event.get("reason") == "compiler-artifact":
            for path in event["filenames"]:
                if path.endswith(".rlib"):
                    artifacts[event["target"]["name"]] = path
    native = raw / "synthetic-gci-probe"
    deps = Path(artifacts["serde_json"]).parent
    command = ["rustc", "--edition=2021", str(HERE / "phase8_synthetic_gci_probe.rs"),
               "-L", f"dependency={deps}", "--extern", f"jarambit_crescent={artifacts['jarambit_crescent']}",
               "--extern", f"serde_json={artifacts['serde_json']}", "-o", str(native)]
    with (raw / "rustc.log").open("w") as err:
        subprocess.run(command, cwd=ROOT, env=env, stderr=err, check=True)
    return native, command


def main():
    if not __debug__:
        raise RuntimeError("Run this assertion-based checker without Python -O")
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--verify", action="store_true", help="Verify saved hashes and repeat the synthetic checks")
    args = parser.parse_args()
    if args.verify:
        from audit_phase8 import main as verify_saved
        verify_saved()
        return
    cases = manufactured_cases()
    payload = (json.dumps(triplets(cases), sort_keys=True) + "\n").encode()
    raw = ROOT / "target/rsv2-07/phase8" / datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%S%fZ")
    raw.mkdir(parents=True, exist_ok=False)
    env = os.environ.copy()
    env.setdefault("CARGO_BUILD_JOBS", "2")
    env.setdefault("CARGO_INCREMENTAL", "0")
    env.setdefault("CARGO_PROFILE_DEV_DEBUG", "line-tables-only")
    native, command = build(raw, env)
    (raw / "build_command.json").write_text(json.dumps(command, indent=2) + "\n")
    (raw / "input.json").write_bytes(payload)
    first, second = run_probe(native, payload), run_probe(native, payload)
    assert first == second, "Native output changed between independent process invocations"
    results, counts = evaluate(cases, json.loads(first))
    (raw / "native_first.json").write_bytes(first)
    (raw / "native_second.json").write_bytes(second)
    (raw / "toolchain.txt").write_text(subprocess.check_output(["rustc", "-Vv"], text=True))
    receipt = {
        "schema": "jarambit.rsv2_07.phase8_synthetic_gci.v1", "task": "RSV2-07-p8",
        "execution_status": "complete", "all_checks_pass": True,
        "timestamp_utc": datetime.now(timezone.utc).isoformat(),
        "commit": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
        "runtime": {"cpu_affinity": sorted(os.sched_getaffinity(0)),
                    "nice": os.getpriority(os.PRIO_PROCESS, 0), "python": platform.python_version(),
                    "build_environment": {k: env[k] for k in ("CARGO_BUILD_JOBS", "CARGO_INCREMENTAL", "CARGO_PROFILE_DEV_DEBUG")}},
        "raw_directory": str(raw.relative_to(ROOT)), "sequence_count": len(cases),
        "triplet_count": sum(counts.values()), "status_counts": counts,
        "native_process_replay_bytes_equal": True, "sequences": results,
    }
    rows = "\n".join(f"| {name} | {count} |" for name, count in counts.items())
    REPORT.write_text(
        "# Phase-8 synthetic GCI arithmetic audit\n\n"
        f"PASS: 24 synthetic sequences, {sum(counts.values())} triplets, all seven native fit statuses. "
        "Twelve five-number sequences use f(h)=2+c*h^p with p in "
        "{0.5,1,1.5,2,2.5,3}, r=2 or sqrt(2), and alternating signs of c. "
        "Twelve controls cover oscillation, constants, pair plateaus, negative/zero order, "
        "invalid ratios, precision floor, zero reference and duplicate synthetic identities.\n\n"
        "| Native status | Triplets |\n|---|---:|\n" + rows + "\n\n"
        "Inputs are ordered finest to coarsest; five-number cases use slices [0:3] and [2:5]. "
        "Each triplet checks expected status, monotonicity, error availability and admission. "
        "Inputs cross the Rust boundary as IEEE-754 bit strings. Native output p, f_ext and "
        "gci_fine match the existing Python reference bit for bit for every triplet. "
        "Two independent executions of the same native binary produce identical output bytes.\n\n"
        "For every polynomial triplet, recovered order and extrapolate agree with the "
        "manufactured p and f0 within 1e-11 relative/absolute tolerance; relative GCI agrees "
        "with Fs*abs(c*h^p)/abs(f1) within 1e-11 relative or 1e-13 absolute tolerance. "
        "Fs=1.25 bounds the known relative error for these test functions. "
        "This check is not a universal error-bound proof.\n\n"
        "The assignment's unresolved_difference maps to native unresolved (with a specific reason); "
        "invalid_grid_ratio maps to invalid_input / invalid_ratio_or_safety_factor. "
        "Coverage here means all seven FitStatus variants, not exhaustive coverage of every "
        "error reason or source-code branch. Synthetic level identities are test fixtures. "
        "Every result retains asymptotic_validation=not_established.\n\n"
        "These checks use no saved design arrays, geometry or FEM solves and establish no "
        "physical convergence or design qualification. Bit equality is established for the "
        "recorded toolchain/platform, not guaranteed across other libm or compiler versions.\n\n"
        f"Execution affinity: {receipt['runtime']['cpu_affinity']}; nice: {receipt['runtime']['nice']}; "
        f"build jobs: {env['CARGO_BUILD_JOBS']}. Existing lane target/ reused.\n\n"
        "Reproduce acquisition: `taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 "
        "experiments/rsv2_07/phase8_synthetic_gci_check.py`. Verify saved hashes and repeat "
        "native checks: `taskset -c 14,15 python3 experiments/rsv2_07/audit_phase8.py`. "
        "The verifier imposes no CPU affinity restriction; pair 8,9 can be substituted.\n\n"
        f"Raw evidence: `{raw.relative_to(ROOT)}`. Per-sequence values, sign patterns, native "
        "results, IEEE-754 bits and SHA-256 provenance are in "
        "`results/phase8_synthetic_gci_audit.json`.\n"
    )
    sources = [ROOT / name for name in (
        "Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "crates/crescent/Cargo.toml",
        "crates/crescent/src/gci.rs", "crates/evidence/src/identity.rs", "fem_solid/gci.py",
        "assurance/testing/contracts.py", "experiments/rsv2_07/phase8_synthetic_gci_probe.rs",
        "experiments/rsv2_07/phase8_synthetic_gci_check.py", "experiments/rsv2_07/audit_phase8.py")]
    receipt["sha256"] = {str(path.relative_to(ROOT)): digest(path)
                         for path in sources + [REPORT] + sorted(raw.iterdir())}
    from portable_audit import RAW, study_manifest
    receipt["artifact_bytes"] = {name: (ROOT / name).stat().st_size
                                 for name in receipt["sha256"] if name.startswith(RAW)}
    receipt["study_sha256"] = study_manifest(ROOT)
    RECEIPT.write_text(json.dumps(receipt, indent=2, allow_nan=False) + "\n")
    print(f"PASS: {len(cases)} synthetic sequences; {sum(counts.values())} triplets; seven fit statuses.")
    print("Rust/Python floating-point bits and independent native-process replay agree.")


if __name__ == "__main__":
    main()
