# G16-06-RSV2-08 Phase 29 — Verifier assertion and test declaration inventory

Task G16-06-RSV2-08-p30 audits the check functions in
`scripts/verify_g16_06_rsv2_08_receipts.py` and the declared tests in
`scripts/tests/test_verify_g16_06_rsv2_08_receipts.py`. The output is
`phase29_verifier_assertion_inventory_audit.json`.

The AST inventory selects every function with explicit `raise` or `assert`
statements in its own lexical body, plus the `verify` and `verify_*` entry points
and dispatchers. Nested functions and class methods have qualified names and
separate records. Their bodies, and lambda bodies, are excluded from the parent
function's statement counts, so guard ownership is not double-counted. Each row
records source line range, raise/assert/if counts and verification-entrypoint
status. Helpers without these guards or entrypoint names are outside this
defined check-function set.

Both exact source digests and byte sizes are recorded. The test inventory lists
class names, direct `test_*` method names and class/total counts. Decorated,
synchronous and asynchronous declarations follow the existing AST counting
definition. Standalone and nested helper functions are excluded. Source parsing
does not import or execute inventoried code. Function selection and declaration
counts do not establish runtime coverage, reachability, engineering qualification
or a count of every possible Python exception.

Default verification first preserves the complete earlier receipt-check chain,
then recomputes this assertion inventory from the current source bytes. Its own
verification entry point is included. Updating scripts requires metadata-only
refreshes of Phases 24–28; their prior receipts are preserved under
`target/G16-06-RSV2-08/phase30/prior_*.json`. Historical reports and Phase 15–23
receipts remain unchanged. All earlier independent pins stay active; this new
receipt has no self-referential pin embedded in an inventoried source.

Six new regressions check live function/source inventories and unittest discovery
parity, nested lexical guard ownership, zero-guard verification dispatchers,
invalid syntax, forged guard/test/source records, missing source and static
output scope. The combined relocated fixture has 45 unique files and verifies
all stages without Git metadata, target evidence, environment settings, writes,
subprocesses or nonlocal reads. Verification also checks unchanged COMMS bytes.

```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_g16_06_rsv2_08_receipts.py --write-script-inventory --write-script-cross-check --write-summary --write-manifest-cross-check --write-cumulative-index --write-assertion-inventory
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts/tests -p 'test_*.py' -v
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s experiments/G16-06-RSV2-08 -p 'test_*.py' -v
git diff --check
```

Write options regenerate dependent metadata in order; default verification only
reads saved files. This is a static inventory only: no builds, solves, package
installation, designs or engineering assessments. Qualified count is zero, full
qualification false and engineering qualification unassessed. No gate or threshold
changes. New report/receipt host-path counts are zero. Baseline: `c8b592d`.
Execution uses CPUs 12,13 with `nice -n 10` (inherited effective nice 19) and
CARGO_BUILD_JOBS=2. Changes remain uncommitted. `COMMS.md` is not edited.

Validation result: combined verifier PASS; 59/59 verifier tests and 79/79
historical tests pass. The AST inventory records 22 check functions, including
10 verification entry points, with 35 explicit raise statements and zero Python
assert statements. It records 59 test methods across nine classes. Both source
files total 71,100 bytes. Structural comparisons prove metadata-only refreshes
in Phases 24–28; prior receipts match committed baseline bytes. Older reports
and Phase 15–23 receipts remain unchanged. Whitespace and new output host-path
scans pass. COMMS bytes/SHA-256 are unchanged. Logs remain under
`target/G16-06-RSV2-08/phase30/`.
