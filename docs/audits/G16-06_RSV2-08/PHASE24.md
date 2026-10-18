# G16-06-RSV2-08 Phase 24 — Static script and test inventory

Task G16-06-RSV2-08-p25 inventories the final exact bytes of
`scripts/verify_g16_06_rsv2_08_receipts.py` and
`scripts/tests/test_verify_g16_06_rsv2_08_receipts.py`. The receipt
`phase24_script_test_inventory_audit.json` records each file's SHA-256, byte size,
valid Python AST status and AST test-method declaration count.

AST counts include direct class-body synchronous and asynchronous function
declarations whose names start with `test_`, including decorated methods.
Standalone test-like functions and functions nested inside methods are excluded.
Parsing never imports or executes the inventoried source. These are declaration
counts; the actual unittest run is recorded separately as validation. For the
current test module, static declarations also match unittest discovery.

The default verifier now checks this inventory alongside the unchanged earlier
JSON, Markdown and cross-check audits. `--write-script-inventory` regenerates
only this new receipt after source edits are finalized. Exact source bytes are
hashed without normalization. A comment-only edit therefore fails verification
even if the parsed AST method count does not change. Syntax errors and missing
source files also fail verification.

The inventory recomputes both records and all metadata from the two fixed source
paths. Its own digest is not embedded inside either inventoried source: doing so
would create a circular hash dependency. All earlier independent receipt digest
pins and engineering checks remain unchanged. This new receipt establishes
source-byte parity, not an independently signed or immutable source attestation.

Six new regressions check exact metrics, unittest discovery parity, AST counting
semantics without execution, syntax failures, forged records and coverage,
missing/changed source files and static output scope. A three-file relocated
fixture verifies without Git, target evidence, environment settings, writes,
process launches or reads outside the fixture. The combined earlier fixture now
has 36 files and checks all four audit stages with the same read-only guards.

```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_g16_06_rsv2_08_receipts.py
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts/tests -p 'test_*.py' -v
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s experiments/G16-06-RSV2-08 -p 'test_*.py' -v
git diff --check
```

This is a static script/test inventory only. No builds, numerical solves, package
installations, design generation or engineering assessments run. Qualification
remains unassessed, qualified_count is zero and full_qualification is false.
Historical reports, inventories and measurements remain unchanged; no gate or
threshold changes. The new report and receipt contain zero literal host paths.
Baseline: `adacc5e`. Execution uses CPUs 12,13 with `nice -n 10` (inherited
effective nice 19) and CARGO_BUILD_JOBS=2. Leave changes uncommitted.

| File | Bytes | AST test methods | SHA-256 |
| --- | ---: | ---: | --- |
| `scripts/verify_g16_06_rsv2_08_receipts.py` | 15028 | 0 | `6c30d60c46597ec901ced555fb72f453d3097a13b9fb18e1d09bbe1b0e7a74ee` |
| `scripts/tests/test_verify_g16_06_rsv2_08_receipts.py` | 19954 | 29 | `6a4f9e3e8d4f264795d43e00bd703a83d8e9998764e41151df3bafcbfd22f725` |

Validation result: combined verifier PASS; 29/29 verifier tests and 79/79
historical tests pass. The two source files total 34,982 bytes. All three earlier
audit receipts are byte-identical to the committed baseline. Whitespace and
new report/receipt host-path scans pass. Logs remain under
`target/G16-06-RSV2-08/phase25/`.
