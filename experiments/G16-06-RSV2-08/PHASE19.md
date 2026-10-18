# G16-06-RSV2-08 Phase 19 — Recorded solver and environment cross-verification

Task G16-06-RSV2-08-p20 audits the nine Phase 7–15 aggregate measurement
receipts, all six tracked Phase 15 measurement JSONs, the three Phase 16–18
summary receipts and Cargo.lock: 19 exact SHA-256-bound inputs. Historical
Phase 1–18 files remain unchanged. `qualified_count = 0` and
`full_qualification = false` remain explicit throughout.

Recorded values are consistent, with incomplete historical version evidence.
The audit sets `all_phase_versions_fully_pinned: false`. Missing records are
never filled using the current machine, another phase or package imports.

| Evidence | Recorded result | Scope and limits |
| --- | --- | --- |
| Native plate solver | faer | Every explicit plate solver field in Phase 7–15 aggregate receipts and all three Phase 15 plate JSONs |
| faer version | 0.24.4 | Identical Cargo.lock hash bound by Phases 9, 10, 11, 13, 14 and 15; Phase 7/8 have no recorded lockfile binding |
| scikit-fem | 12.0.2 | Phase 15 runtime records the distribution under `skfem` |
| scipy | 1.18.1 | Phase 15 runtime only |
| numpy | 2.5.4 | Phase 15 runtime only |
| meshio | unknown | No version recorded in the audited measurement receipts |
| Primary solid solver | skfem_p2_amg_cg | All three saved Phase 15 solid JSONs, converged with residual at most unchanged 1e-10 |
| R_CUT / NL / NSUB / PATCH | 30 mm / 4 / 1 / 2 mm | All three saved primary solid measurements |
| Chord torque | 1820 N mm | Exact `T_Nmm` field in each saved primary solid measurement |

Python versions are recorded in Phase 15's shared runtime metadata; they are
not independently embedded in each solid JSON. The audit binds those measurement
files to their exact hashes in the same Phase 15 receipt. It does not claim
independent per-process version attestation. The recorded extra pyamg version
5.3.0 is retained in the exact version observation.

Phase 12 is a saved-data master with zero new native solves. Phases 16–18 are
provenance and inventory summaries, without new measured runtime environments.
Their explicit solver/version fields are therefore not inferred from parents.
Phases 7–14 have no recorded Python package versions in these aggregate receipts;
the P2 settings above apply to Phase 15's saved primary solid measurements,
not to earlier native plate, export-census, A4 or GCI studies. All nested exact
`solver` and `versions` keypaths in Phase 7–15 aggregate receipts are inventoried.
Performance metrics or joint capacity values are not mistaken for applied loads.
The 12 nested Phase 7/8 side-A4 solver descriptions separately record native
faer sparse Cholesky with K-orthogonal Lanczos; their full strings are checked
under that distinct solver role rather than confused with plate-only metadata.

The audit covers published aggregate/embedded observations and all separately
tracked Phase 15 plate/solid measurement JSONs. Optional ignored target files are
outside this portable cross-verification; no native solve or external package
installation is performed. A numerical package missing from historical metadata
is an evidence gap, not proof of a version mismatch or a verified pin.

`test_phase19_solver_env.py` provides a read-only generator/verifier and nine
regressions, including independent audit digest pinning, complete phase coverage,
unknown-version handling, solver drift, exact P2 settings, unchanged residual
threshold, false pinning/qualification claims and literal host-prefix scans.
A relocated 20-file fixture verifies using only the standard library, without
Git, target evidence, environment settings, subprocesses or nonlocal file reads.
All three new Phase 19 deliverables contain zero literal legacy host prefixes.

Validation uses CPUs 12,13, `nice -n 10` (inherited effective nice 19),
CARGO_BUILD_JOBS=2 and bytecode writes disabled:

```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s experiments/G16-06-RSV2-08 -p 'test_*.py' -v
git diff --check
```

Baseline: `9672d3e`. No gates, thresholds, production sources, candidate designs
or historical scientific receipts change. Strict historical content-portability
and the missing Phase 1 report remain unchanged limitations of earlier studies.
Changes remain uncommitted for captain review.

Validation result: all 70 discovered tests pass (61 prior and nine new).
All 19 audit inputs match committed baseline bytes; all 33 earlier manifest
files retain their exact hashes. The three new files contain zero literal host
prefixes and pass whitespace checks. Recorded-value consistency passes; complete
cross-phase version pinning remains unverified because of the stated gaps.
