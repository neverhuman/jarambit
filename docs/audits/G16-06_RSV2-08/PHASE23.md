# G16-06-RSV2-08 Phase 23 — Static inventory cross-check

Task G16-06-RSV2-08-p24 cross-checks the unchanged Phase 21 JSON inventory and
Phase 22 Markdown report index. Their frozen snapshots contain 29 JSON members
and one Markdown member: 30 distinct paths, 18,550,633 member bytes, zero path
overlap and exact SHA-256 parity for every member. Both source-index files are
also bound to their independently pinned expected digests and exact byte sizes.

`phase23_inventory_cross_check_audit.json` records both source-index digests and
all 30 member paths with expected/actual SHA-256, byte sizes and explicit parity
flags. Duplicate paths within either index and shared paths between indexes are
rejected. Overlap is a path-set test: different digest values do not excuse an
overlapping path. Digest parity means each recorded member digest equals the
digest of its current exact bytes; different JSON and Markdown files are not
expected to share a digest.

The earlier snapshots are preserved at their original baselines. This audit does
not resnapshot newly tracked audit outputs or enlarge either historical index.
`scripts/verify_g16_06_rsv2_08_receipts.py` retains the previous JSON and Markdown
checks and now verifies the cross-check by default. Standalone cross-check
verification also enforces both source-index pins, their original coverage,
strict JSON/finite-number policy, Markdown metrics and literal-host-path checks.
The new cross-check receipt has its own independent digest pin.

Six new regressions check exact member/source parity, overlapping paths despite
different digests, duplicate entries, forged parity/counts, all three receipt
pins and static output scope. The existing relocated combined-verifier fixture
now contains 33 files, including the cross-check. It passes without Git metadata,
target evidence, environment settings, subprocesses, writes or reads outside the
fixture. Existing missing/corrupt/host-path rejection checks remain active.

```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_g16_06_rsv2_08_receipts.py
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts/tests -p 'test_*.py' -v
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s experiments/G16-06-RSV2-08 -p 'test_*.py' -v
git diff --check
```

`--write-cross-check` regenerates only this new receipt from the two unchanged
source indexes. Default verification reads files and launches no subprocesses.
This is a static index cross-check only: no builds, solves, package installation,
design generation or engineering assessment. Historical indexes, reports and
measurement bytes remain unchanged. Engineering qualification is unassessed,
qualified_count is zero and full_qualification is false. No gates or thresholds
change. The new report and receipt contain zero literal host paths.

Baseline: `a2be63b`. A preexisting COMMS edit is preserved before appending the
LANE handoff. Execution uses CPUs 12,13, `nice -n 10` (inherited effective nice 19)
and CARGO_BUILD_JOBS=2. Changes remain uncommitted.

Validation result: combined verifier PASS; 23/23 verifier tests and 79/79
historical tests pass. Both source indexes and all 30 members match committed
baseline bytes. Zero overlaps and zero digest mismatches; 18,550,633 member
bytes verified. Whitespace and new report/receipt host-path scans pass. Logs
remain under `target/G16-06-RSV2-08/phase24/`.
