# G16-06-RSV2-08 Phase 28 — Cumulative Phase 15–27 receipt index

Task G16-06-RSV2-08-p29 indexes exactly 13 JSON receipts, one for each Phase 15
through 27, with actual repository-relative paths, exact SHA-256 and byte size.
The six Phase 15–20 receipts retain their existing `experiments/G16-06-RSV2-08`
paths; the seven Phase 21–27 receipts reside in `docs/audits/G16-06_RSV2-08`.
The output is `phase28_cumulative_receipt_index_audit.json`. Raw measurement
JSONs and Markdown reports are outside this cumulative receipt-only inventory.

Every receipt passes strict JSON parsing and retains its explicit false full
qualification status. Phase 15 names that flag `all_gate_qualification`; later
receipts use `full_qualification` and zero `qualified_count`. Those historical
schema differences are handled explicitly. The new index records zero qualified
count and false full qualification per receipt and globally; engineering
qualification remains unassessed by this static task.

The verifier checks the complete earlier manifest cross-check chain before
generating or verifying the cumulative index. Coverage is a fixed, ordered set
of Phase 15–27 receipt identities. Omission, duplication, wrong phases, forged
paths, digests, sizes or qualification claims fail. Supplied index paths do not
select files to read. Exact-byte verification also rejects harmless formatting
changes that leave parsed JSON values unchanged. Default verification now
includes this cumulative index.

Updating the two scripts requires metadata-only regeneration of their Phase 24
inventory, Phase 25 dependent cross-check, Phase 26 summary and Phase 27 live
manifest cross-check. Original receipts are preserved under
`target/G16-06-RSV2-08/phase29/prior_*.json`. Only source-dependent hashes, sizes,
AST counts and corresponding totals are refreshed. Historical reports and the
Phase 15–23 receipts remain unchanged. Earlier independent digest pins remain
active; no circular pin for the new index is embedded in an inventoried script.

Six new regressions check all 13 receipts, coverage and metadata forgery, source
qualification flags, missing receipts, formatting-only byte changes and static
output scope. They also check that verification leaves COMMS bytes unchanged.
The combined relocated read-only fixture now has 44 unique files and verifies
all stages without Git metadata, target evidence, environment settings, writes,
subprocesses or nonlocal reads.

```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_g16_06_rsv2_08_receipts.py --write-script-inventory --write-script-cross-check --write-summary --write-manifest-cross-check --write-cumulative-index
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts/tests -p 'test_*.py' -v
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s experiments/G16-06-RSV2-08 -p 'test_*.py' -v
git diff --check
```

Write options regenerate dependent metadata in order; default verification only
reads files. No builds, solves, package installation, design generation or
engineering assessment runs. No gate or threshold changes. New report/index
host-path counts are zero. Baseline: `57bd850`. Execution uses CPUs 12,13 with
`nice -n 10` (inherited effective nice 19) and CARGO_BUILD_JOBS=2. Changes remain
uncommitted. `COMMS.md` is not edited, as explicitly requested.

Validation result: combined verifier PASS; 53/53 verifier tests and 79/79
historical tests pass. All 13 indexed receipts have exact SHA-256 and byte
parity, totaling 488,084 bytes. Current scripts total 62,528 bytes with
AST test counts 0 and 53. Structural comparisons prove metadata-only refreshes
in Phases 24–27; original receipts match the committed baseline. Phase 15–23
receipts and all earlier reports remain unchanged. Whitespace and new output
host-path scans pass. COMMS bytes and SHA-256 are unchanged. Logs remain under
`target/G16-06-RSV2-08/phase29/`.
