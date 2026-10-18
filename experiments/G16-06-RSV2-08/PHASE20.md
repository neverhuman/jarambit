# G16-06-RSV2-08 Phase 20 — Candidate CSV and genotype schema integrity

Task G16-06-RSV2-08-p21 audits all 411 tracked CSVs in the requested uppercase
`experiments/G16-06-RSV2-08/phase*_candidates/` directories, Phase 3 through 15,
at baseline `ee41703`. Every file's exact SHA-256, byte size, phase, numeric gene
column count, finite-value count, header width, data-row count and material are
recorded in `phase20_candidate_csv_schema_audit.json`.

Every CSV passes the same schema: one 129-column header, one 129-column data row,
128 finite numeric genes and a final `Ti-6Al-4V` material field. This verifies
52,608 finite gene values across the saved file inventory. All 411 headers match
the native crescent schema from `crates/crescent/src/genome.rs`: 40 control-point
columns, 88 named genes and one material column. The audit binds that source
file's exact digest and records the complete ordered header.

| Phase | Tracked CSV files in assigned directory |
| --- | ---: |
| 3 | 9 |
| 4 | 8 |
| 5 | 163 |
| 6 | 72 |
| 7 | 144 |
| 8 | 12 |
| 9 | 0 |
| 10 | 0 |
| 11 | 0 |
| 12 | 0 |
| 13 | 0 |
| 14 | 0 |
| 15 | 3 |

These are tracked file counts, not counts of distinct or engineering-qualified
designs. Phases 9–14 use a separate lowercase study tree; their candidate files
are outside the explicitly assigned uppercase glob. Zero here does not assert
that those phases had no candidates. Optional target evidence and other CSV
inputs outside the assigned candidate directories are also outside this audit.

The initial file list is taken from Git's tracked index, filtered to phases
3–15 and sorted. Its newline-delimited SHA-256 is recorded independently of the
per-file digests. An independent audit digest pin in the test module freezes the
complete inventory and all results. Portable verification reads those exact
paths without invoking Git, discovering untracked candidates or changing any
historical files. Omissions, duplicates and unsafe paths are rejected.

`test_phase20_candidate_csvs.py` includes a read-only generator/verifier and nine
regressions. They check native schema, all file digests, phase coverage and finite
values; reject NaN, infinities, float overflow, blank/nonnumeric genes, changed
headers, short/long rows, extra records, changed material, false qualification
and forged metadata. A relocated 413-file fixture (411 CSVs, native schema source
and audit JSON) verifies without Git metadata, target evidence, environment
settings, subprocesses or nonlocal reads. Missing files fail, and a line-ending
change fails the exact digest check even if parsed numeric values are unchanged.

`qualified_count = 0` and `full_qualification = false` remain explicit globally,
per phase and per file. Schema and finiteness establish neither canonical native
gene bounds nor geometry, FEM, export, manufacturing or full qualification.
No normalization, repairs, native clamping or design changes are performed.
All historical Phase 1–19 files remain unchanged. All three new Phase 20 files
contain zero literal legacy host prefixes.

Validation uses CPUs 12,13, `nice -n 10` (inherited effective nice 19),
CARGO_BUILD_JOBS=2 and bytecode writes disabled:

```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s experiments/G16-06-RSV2-08 -p 'test_*.py' -v
git diff --check
```

No gates, thresholds, production sources, scientific receipts or saved candidates
change. Changes remain uncommitted for captain review.

Validation result: all 79 discovered tests pass (70 prior and nine new).
The 411 audit paths exactly equal the filtered Git tracked inventory; every
CSV and the native schema source match committed baseline bytes. Whitespace
checks pass, and all three new deliverables contain zero literal host prefixes.
