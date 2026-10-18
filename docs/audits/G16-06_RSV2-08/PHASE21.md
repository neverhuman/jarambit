# G16-06-RSV2-08 Phase 21 — Static measurement JSON inventory

Task G16-06-RSV2-08-p22 inventories every tracked repository file matching
`phase*_measurements/*.json` or `phase*.json` at baseline `2374e55`: 29 files,
18,547,344 bytes. This includes 25 files from the uppercase/lowercase
G16-06-RSV2-08 study and four phase receipts under `experiments/rsv2_07/results`.
Patterns match basenames and immediate measurement-directory names anywhere in
the repository. The two patterns are combined without double-counting.

Every file parses as strict JSON and has zero nonfinite numeric values. The
inventory records each repository-relative path, exact byte size, SHA-256,
validity, integer/float value counts and nonfinite numeric count. Parsing rejects
NaN, positive/negative Infinity and numeric literals whose float conversion
overflows, including nested values. Strings that mention those terms remain
ordinary valid JSON strings. Boolean values are not counted as integers.

`scripts/verify_g16_06_rsv2_08_receipts.py` and its test directory did not exist
in this checkout; both are created for this assignment. The verifier checks the
frozen inventory digest, sorted tracked-path-list digest, exact coverage, file
sizes, file hashes, strict parsing and finite numeric values. Its default command
only reads saved files and launches no processes. The optional `--write` uses
read-only Git discovery at the pinned baseline to regenerate the same snapshot.
New untracked audit outputs are excluded from that historical tracked snapshot.

```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_g16_06_rsv2_08_receipts.py
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts/tests -p 'test_*.py' -v
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s experiments/G16-06-RSV2-08 -p 'test_*.py' -v
git diff --check
```

Ten focused regressions cover complete coverage, exact sizes/hashes, valid and
malformed JSON, nested NaN/Infinity, float overflow, metadata forgery, inventory
pinning, unsafe paths and a relocated 30-file fixture without Git or target
evidence. Missing files, byte changes and injected nonfinite values are rejected.
The fixture guards reads outside its root, writes and subprocess launches.

This is a static inventory audit only. No solver, numerical package installation,
build, design generation, measurement or engineering assessment is performed.
`engineering_qualification_assessed: false`, `qualified_count: 0` and
`full_qualification: false` are explicit. No historical source, receipt,
measurement, gate or threshold changes. All new files contain zero literal legacy
host prefixes. Execution is pinned to CPUs 12,13 with `nice -n 10` (inherited
effective nice 19) and CARGO_BUILD_JOBS=2. Changes remain uncommitted.

Validation result: verifier PASS for all 29 files; 10/10 focused tests and
79/79 historical tests pass. Every inventory path matches the tracked baseline
selection; all 29 files match committed baseline bytes and hashes. Whitespace
checks and scans of all four new files for literal legacy host prefixes pass.
Logs are retained under `target/G16-06-RSV2-08/phase22/`.
