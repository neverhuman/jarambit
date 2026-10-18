# Phase-7 saved-file integrity audit — partial assignment

Only the file-integrity portion was executed. The requested triplet GCI / Richardson analysis was not performed. This report establishes no new structural-performance or convergence result.

File-byte checks: **PASS**; 246 manifest entries (80 source, 164 raw artifact, 1 report, 1 postprocessor).

Pinned Phase-6 receipt SHA-256: `5d301a3b33afdd8470441c0094f79e47915f8f9e1161af035b90ac37223f7dd7`. Matches establish byte identity against this saved manifest; they do not establish scientific correctness, authenticity or complete directory coverage.

Executed 2026-10-18T08:52:15.151731+00:00 with CPU affinity `[14, 15]`, nice `10`, Python `3.12.3`, CARGO_BUILD_JOBS=`2`. No new geometry, FEM solve or build was run.

Re-run the read-only checks from the checkout:

```sh
taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/audit_phase7.py
```

CPU pair `8,9` can be substituted when available. Add `--write` to regenerate this partial report and receipt. The verifier returns success only for file integrity; that exit code does not mean the Phase-7 GCI assignment is complete.

Per-file expected/actual SHA-256 digests and read errors are recorded in `results/phase7_saved_triplet_gci_audit.json`.
