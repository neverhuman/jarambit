# G16-06-RSV2-08 Phase 27 — Static summary, live-file and receipt cross-check

Task G16-06-RSV2-08-p28 verifies the Phase 26 summary's recorded SHA-256 and byte
counts for all ten Phase 21–25 Markdown/JSON files against their live bytes.
`phase27_manifest_cross_check_audit.json` records expected/actual digests and
byte counts, explicit parity flags, and the exact summary receipt digest/size.

The cross-check also compares four direct receipt references with both the
summary records and live files: Phase 23's references to the Phase 21 and 22
indexes, and Phase 25's references to the Phase 23 and 24 receipts. Phase 24's
two script members are checked against their live source bytes; those script
paths are outside the summary's report/receipt scope and are explicitly marked
as such. Phase 23, 24 and 25 receipt files themselves are included among the ten
live summary checks.

Standalone and default verification first run the complete existing summary
and parent checks. They then compare each source/reference digest and size,
reject unsafe paths, require the four receipt references to exist in the summary,
and reject any disagreement with summary metadata. All earlier independent pins,
coverage, strict finite JSON, Markdown metrics, script AST counts and disjoint
member checks remain active.

Authorized script/test edits require metadata-only regeneration of Phase 24's
script metrics, Phase 25's dependent source/member digests and sizes, and Phase
26's dependent summary rows/totals. Prior receipts are preserved under
`target/G16-06-RSV2-08/phase28/prior_*.json`. Historical reports and Phase 21–23
receipts remain unchanged. Older reports retain their original run metrics.
No digest pin for this new receipt is embedded in an inventoried source script,
avoiding a circular hash dependency; verification recomputes exact current-byte
parity and retains the earlier independent pins.

Six new regressions cover complete parity, incorrect live digests/sizes, missing
or disagreeing summary references, forged parity/coverage, unsafe paths, missing
live files and static output scope. The combined relocated fixture now contains
43 unique files and verifies all stages without Git metadata, target evidence,
environment settings, writes, subprocesses or nonlocal reads.

```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_g16_06_rsv2_08_receipts.py --write-script-inventory --write-script-cross-check --write-summary --write-manifest-cross-check
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts/tests -p 'test_*.py' -v
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s experiments/G16-06-RSV2-08 -p 'test_*.py' -v
git diff --check
```

The write options regenerate dependent metadata in order; default verification
only reads saved files. This is a static cross-check only: no builds, solves,
package installation, designs or engineering assessment. Qualified count remains
zero, full qualification false and engineering qualification unassessed. No gate
or threshold changes. The new report and receipt contain zero literal host
paths. Baseline: `9499987`. Execution uses CPUs 12,13 with `nice -n 10`
(inherited effective nice 19) and CARGO_BUILD_JOBS=2. Leave changes uncommitted.

Validation result: combined verifier PASS; 47/47 verifier tests and 79/79
historical tests pass. All ten summary files, four receipt references and two
script members have exact SHA-256 and byte parity, with zero mismatches. The
summary still records 53,632 bytes and 1,167 lines; current scripts total 55,924
bytes with AST test counts 0 and 47. Structural comparisons prove only dependent
metadata fields changed in Phases 24–26. All Phase 21–26 reports and Phase 21–23
receipts are byte-identical to the committed baseline. Whitespace and new output
host-path scans pass; logs remain under `target/G16-06-RSV2-08/phase28/`.
