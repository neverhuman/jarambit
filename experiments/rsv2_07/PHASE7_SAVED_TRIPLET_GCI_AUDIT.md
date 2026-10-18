# Phase-7 saved-file integrity audit — partial assignment

Only the file-integrity portion was executed. The requested triplet GCI / Richardson analysis was not performed. This report establishes no new structural-performance or convergence result.

File-byte checks: **PASS**; 246 manifest entries (80 source, 164 raw artifact, 1 report, 1 postprocessor).

Pinned Phase-6 receipt SHA-256: `5d301a3b33afdd8470441c0094f79e47915f8f9e1161af035b90ac37223f7dd7`. Matches establish byte identity against this saved manifest; they do not establish scientific correctness, authenticity or complete directory coverage.

Executed 2026-10-18T08:52:15.151731+00:00 with CPU affinity `[14, 15]`, nice `10`, Python `3.12.3`, CARGO_BUILD_JOBS=`2`. No new geometry, FEM solve or build was run.

Re-run the read-only checks from the checkout:

```sh
taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/audit_phase7.py
```

CPU pair `8,9` can be substituted when available. Add `--write` to refresh receipt execution metadata after successful verification. The verifier returns success only for file integrity; that exit code does not mean the Phase-7 GCI assignment is complete.

Per-file expected/actual SHA-256 digests and read errors are recorded in `results/phase7_saved_triplet_gci_audit.json`.

## Phase-9 verifier portability

Default verification is read-only. Present `target/rsv2-07/**` files must match both recorded SHA-256 and byte size. Missing raw files are reported separately as **manifest-only**: their 64-hex SHA-256 and non-negative integer byte size are validated, without claiming to have verified absent bytes.

Non-study source files, including `crates/**` and `docs/WORKPLAN.md`, may match either the working tree or a historical Git blob. Historical matches require hashing the actual blob bytes with SHA-256, starting with the recorded snapshot commit and then searching commits affecting that path. A missing historical blob is a failure; an object ID alone is not evidence for a SHA-256 match.

Every tracked `experiments/rsv2_07/**` file is covered by the strict study manifest and must be present and unchanged. The two controlling Phase-7/8 JSON receipts are required and structurally checked; they cannot hash themselves or each other without a cycle. Phase-7 additionally binds its source receipt, complete snapshot manifest, report and verifier. This is provenance validation, not a scientific GCI result or an authenticity guarantee.

The standalone portability regressions run with `taskset -c 14,15 python3 experiments/rsv2_07/test_portable_audits.py`. They simulate absent raw files and newer source bytes without moving evidence or modifying another checkout. They also require changed/missing study files, corrupt present raw files, malformed raw metadata and unknown historical snapshots to fail.

Validation on CPUs14,15 / nice10: both default commands pass with local raw files (164 Phase-7 and 9 Phase-8 on-disk records). All nine portability regressions pass, including a simulated clean checkout with all raw files missing, plus newer source bytes requiring three Phase-7 and one Phase-8 historical matches. Original acquisition hash observations and synthetic numeric results are unchanged. The paper worktree initially had a different tracked Phase-5 receipt; it was synchronized during this session, without changes to that checkout by this worker. Study-file mismatches remain failures and cannot be treated as historical-source evidence.
