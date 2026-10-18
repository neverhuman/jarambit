# G16-06-RSV2-08 Phase 30 — Receipt top-level JSON keysets

Task G16-06-RSV2-08-p31 audits all 15 Phase 15–29 JSON receipts. The output
`phase30_json_schema_keyset_audit.json` records each receipt's phase, relative
path, exact byte size, SHA-256, sorted parsed top-level keys, key count and
canonical keyset SHA-256. The fixed receipt list extends the cumulative
Phase 15–27 list with the Phase 28 and 29 receipts.

The keyset digest hashes the UTF-8 bytes of the sorted key list serialized as
compact JSON with ASCII escaping. Key order and values do not affect that
digest; the separate raw-file digest still detects any byte change. Shared
keys, the union of keys and groups of matching keysets are recorded. Parsing
rejects non-object roots, nonfinite constants and float overflow. This audits
parsed top-level keys only; nested keys, types, duplicate-key detection and
schema or semantic equivalence are outside its scope.

Default verification preserves the complete existing receipt verification
chain and recomputes the new audit from live files. Six new tests cover exact
coverage and aggregates, key-order independence, nested-key exclusion, invalid
roots and nonfinite numbers, forged metadata, missing files and byte-only
changes. The combined relocated fixture contains 46 files and verifies without
Git metadata, target evidence, subprocesses, writes or nonlocal reads.

Updating the verifier and tests requires dependent metadata refreshes of
Phases 24–29. Exact prior copies are preserved under
`target/G16-06-RSV2-08/phase31/prior_*.json`. Earlier independent pins remain
active. Historical reports and Phase 15–23 receipts are unchanged. Phase 29
AST counts reflect the new check and tests; declaration counts do not prove
runtime coverage. No self-referential source digest pin is introduced.

```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_g16_06_rsv2_08_receipts.py --write-script-inventory --write-script-cross-check --write-summary --write-manifest-cross-check --write-cumulative-index --write-assertion-inventory --write-json-keysets
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts/tests -p 'test_*.py' -v
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s experiments/G16-06-RSV2-08 -p 'test_*.py' -v
git diff --check
```

This is a static inventory only. No build, solve, design, package installation
or engineering qualification is performed. Qualification remains unassessed,
qualified count zero and full qualification false. No gate or threshold changes.
Baseline is `e2a24ea`. Execution uses CPUs 12,13, `nice -n 10` (inherited
effective nice 19) and CARGO_BUILD_JOBS=2. Changes remain uncommitted.
`COMMS.md` is not edited. New output literal host-path counts are zero.

Validation result: combined verifier PASS; 65/65 verifier tests and 79/79
historical tests pass. All 15 receipts have distinct keysets; their only shared
key is `task`, and the union contains 85 keys. The updated AST inventory records
24 check functions, 11 verification entry points, 37 explicit raises, zero
assert statements and 65 test methods across ten classes; source bytes total
78,308. Structural comparisons prove metadata-only refreshes for Phases 24–28
and exact recomputation of the Phase 29 source inventory. All six prior copies
match baseline bytes. Earlier receipts and reports remain unchanged. Whitespace
and host-path scans pass; COMMS bytes and SHA-256 are unchanged. Test logs and
preservation proof remain under `target/G16-06-RSV2-08/phase31/`.
