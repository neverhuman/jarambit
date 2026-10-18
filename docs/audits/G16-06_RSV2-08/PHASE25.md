# G16-06-RSV2-08 Phase 25 — Static Phase 23/24 cross-check

Task G16-06-RSV2-08-p26 cross-checks
`phase23_inventory_cross_check_audit.json` and
`phase24_script_test_inventory_audit.json`. Phase 23 supplies 30 measurement/report
member paths; Phase 24 supplies two script/test paths. Their sets are disjoint:
32 unique member paths, zero overlap and exact recorded/current SHA-256 parity.

`phase25_script_and_inventory_cross_check_audit.json` records the exact digests
and byte sizes of both source receipts and every member. Each member has explicit
expected/actual digests and a parity flag. Duplicate entries within either source
and overlapping paths across sources fail, even when their digests differ.
The comparison uses Phase 23's member set, rather than its source-index paths.

The verifier first validates the unchanged independently pinned Phase 23 receipt
and its complete parent checks. It separately verifies Phase 24 against the exact
current script bytes and AST method counts. It then hashes both source receipts
and rechecks all 32 members. Default verification includes this new cross-check;
standalone verification applies the same parent validations.

Updating the two scripts required a metadata-only refresh of Phase 24's script
sizes, SHA-256 values and AST method counts, plus their corresponding totals.
Its task, scope, file identities and all other metadata remain unchanged. The
original committed Phase 24 receipt is preserved under
`target/G16-06-RSV2-08/phase26/prior_phase24_script_test_inventory_audit.json`.
Phase 23 and earlier audit receipts, reports and measurement files are unchanged.
The older Phase 24 report remains a historical record of its original run.

Neither the refreshed Phase 24 receipt nor this new cross-check embeds a digest
pin in an inventoried script: that would create a circular source-hash dependency.
Both are recomputed from exact current source bytes; all earlier independent pins
remain active. This verifies source-byte parity rather than immutable or signed
attestation.

Six new regressions cover all source/member digests, overlapping paths despite
different digests, duplicates, forged parity/counts, stale/missing/changed source
receipts and static output scope. The combined read-only fixture now contains
37 files and verifies all five stages without Git metadata, target evidence,
environment settings, writes, subprocesses or nonlocal reads.

```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_g16_06_rsv2_08_receipts.py --write-script-inventory --write-script-cross-check
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts/tests -p 'test_*.py' -v
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s experiments/G16-06-RSV2-08 -p 'test_*.py' -v
git diff --check
```

The two write options refresh Phase 24 metadata before generating Phase 25;
default verification only reads files. This is a static cross-check only: no
builds, solves, packages, design generation or engineering assessment. Qualified
count remains zero, full qualification false and engineering qualification
unassessed. No gate or threshold changes. New report/receipt host-path counts
are zero. Baseline: `8e73079`. Execution uses CPUs 12,13 with `nice -n 10`
(inherited effective nice 19) and CARGO_BUILD_JOBS=2. Leave changes uncommitted.

Validation result: combined verifier PASS; 35/35 verifier tests and 79/79
historical tests pass. All 32 member digests match exact current bytes, totaling
18,593,018 bytes. The refreshed scripts total 42,385 bytes with AST test counts
0 and 35. A structural comparison confirms Phase 24 changed only script size,
digest and AST-count fields and their totals. Its prior receipt SHA-256 is
`bc9be92f16017130d449173db6d787853d8e45c5fdd36896ba4e87b1a8dc2207`. Phase 23 and earlier
receipt bytes are unchanged. Whitespace and new report/receipt host-path scans
pass; logs remain under `target/G16-06-RSV2-08/phase26/`.
