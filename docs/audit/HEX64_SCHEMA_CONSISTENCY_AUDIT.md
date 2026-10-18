# Strict HEX64 schema consistency audit

status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)
qualified_count: 0

Task: `G16-12-RSV2-04-p40`. Baseline: `6e54c29`. Assignment outcome: PARTIAL; strict schema check FAIL and requested receipt-count premise unmet.

The baseline contains **15 JSON documents**, comprising **14 receipts plus the index**, not 29 JSON receipts. The number29 was the Markdown report count. This task adds report30 and one JSON receipt; the final audit includes **16 actual JSON documents** (15 receipts plus index). No files were invented to satisfy the requested count.

Strictly checking every exact `sha256` field and, additionally, every key ending `_sha256` finds **112 fields: 110 valid, 2 invalid**. Both invalid values are historical nulls recording absent source files. Null is not a lowercase64-character hexadecimal string and receives no exemption. All string digest values meet the strict format. Existing receipts are preserved unchanged; no missing-file hash is fabricated and no historical field is removed to manufacture a pass.

## Strict findings

|JSON document|JSON pointer|Observed value|Result|
|---|---|---|---|
|docs/audit/cli_main_source_provenance_audit.json|/source_files/1/sha256|null|FAIL: not a string|
|docs/audit/fem_bench_source_provenance_audit.json|/source_files/1/sha256|null|FAIL: not a string|

The CLI-main receipt records the absent `crates/cli/src/lib.rs`; the FEM/bench receipt records the absent `crates/crescent/src/bench.rs`. Their provenance meaning remains intact. This audit reports that their null digest fields violate the newly requested literal HEX64 rule; it does not reinterpret absence as a valid hash or alter the historical partial assignments.

## Complete actual JSON inventory

|JSON document|Exact sha256 fields|Suffix fields|Valid fields|Invalid fields|
|---|---|---|---|---|
|docs/audit/aesthetic_gci_source_provenance_audit.json|2|0|2|0|
|docs/audit/audit_provenance_index.json|45|30|75|0|
|docs/audit/battery_g1_source_provenance_audit.json|2|0|2|0|
|docs/audit/cargo_manifest_provenance_audit.json|3|0|3|0|
|docs/audit/cert_export_source_provenance_audit.json|2|0|2|0|
|docs/audit/cli_bench_source_provenance_audit.json|1|0|1|0|
|docs/audit/cli_main_source_provenance_audit.json|2|0|1|1|
|docs/audit/cli_source_provenance_audit.json|2|0|2|0|
|docs/audit/crescent_core_source_provenance_audit.json|2|0|2|0|
|docs/audit/crescent_gates_metrics_provenance_audit.json|2|0|2|0|
|docs/audit/crescent_source_provenance_audit.json|2|0|2|0|
|docs/audit/evolve_physics_source_provenance_audit.json|2|0|2|0|
|docs/audit/fem_bench_source_provenance_audit.json|2|0|1|1|
|docs/audit/genotype_phenotype_constant_contract_audit.json|2|0|2|0|
|docs/audit/hex64_schema_consistency_audit.json|0|0|0|0|
|docs/audit/workspace_manifest_provenance_audit.json|11|0|11|0|

Totals: exact `sha256` fields=82, suffix fields=30, valid=110, invalid=2. Suffix fields currently include the index's table-schema digests. Counts include the updated30-report index and this receipt. The new receipt stores counts, locations and classifications without introducing copied fields named sha256; its own zero-field row is checked. It contains no per-file content hashes, avoiding circular self/index hashing. The index separately binds its exact bytes.

## Rule and coverage

The walker recursively traverses every object and array in all actual `docs/audit/**/*.json` files. Recognized field names are exactly `sha256` or case-sensitive keys ending `_sha256`. Every recognized value must be a string fully matching `[0-9a-f]{64}`. Uppercase, wrong lengths, nonhex characters, whitespace, prefixes, Unicode lookalikes, numbers, booleans, objects, arrays and null fail. RFC6901 pointers escape tilde and slash in object keys. Duplicate-key and nonfinite JSON inputs fail strict parsing before field inspection.

The JSON companion is `docs/audit/hex64_schema_consistency_audit.json`. Its requested-input count remains29, while actual baseline counts are recorded separately. Count mismatch and strict format failure are independent findings. The receipt has diagnostic IN_PROGRESS/qualified_count zero and assignment PARTIAL. No claim that all29 requested receipts exist or that every field passes is made.

## Catalog, verifier and tests

The index now binds30 Markdown reports and15 supplemental JSON receipts. All29 previous reports and14 prior receipt JSON files remain unchanged. Earlier source/manifest provenance, partial scopes, fallback metadata and host-prefix controls continue to be checked. The verifier first checks document/source integrity, then reconstructs and compares the entire HEX64 inventory. On the unchanged dataset, it prints two invalid fields and a receipt-count mismatch and returns exit1. This is the expected strict rejection, not a passing schema verdict.

Tests cover valid digests and invalid types/lengths/case/characters, null without exemptions, recursive arrays/objects and escaped pointers, all actual files and the two exact findings, unexpected nested JSON files, false success metadata after rebinding its receipt hash, and command-line exit1. Existing tampering, path, relocation and historical missing-source controls remain. Pytest success demonstrates correct handling of the negative findings; it does not change the strict schema result.

Run `pytest -q` across both provenance suites, standalone verifier and whitespace checks on CPUs10,11 with `nice -n 10` and `CARGO_BUILD_JOBS=2`; effective inherited nice19. No production code, Rust semantics, Cargo commands, geometry or physical evaluations are executed. The index has no self-digest; exact outputs, originals and command evidence are sealed separately in the ignored local task receipt. Changes remain uncommitted.
