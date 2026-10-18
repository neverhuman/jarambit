# Static CLI bench source-byte provenance audit

status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)
qualified_count: 0

Task: `G16-12-RSV2-04-p35`. Baseline: `aac39d8`. Assignment outcome: COMPLETE for the assigned CLI bench source digest.

This receipt records SHA-256, byte size and UTF-8 split-line count for `crates/cli/src/bench.rs` only. The file was treated as inert bytes; no Rust semantics, benchmark behavior, optimization or physical logic were reviewed. No production source was imported, executed or modified, and no Cargo command, solver or geometry evaluation was run. The companion JSON is `docs/audit/cli_bench_source_provenance_audit.json`.

## Exact fingerprint

|Source path|SHA-256|Bytes|Lines|
|---|---|---|---|
|crates/cli/src/bench.rs|191712f94e18ed8b49612edee84037ef4f9c4f533c3ebbb8342a7b866e7ffa71|18766|243|

SHA-256 hashes exact bytes without encoding, newline or whitespace normalization. Bytes is the byte length; Lines is `len(data.decode("utf-8").splitlines())`. A trailing newline adds no empty line. The JSON has exactly one source record. These fingerprints establish equality to recorded input, not authorship, correctness or runtime validity.

## Catalog and historical scope

The index now covers twenty-five ordered Markdown reports: the unchanged original fifteen-report catalog, Phase26 through Phase34, then this receipt. Ten companion JSON receipts are supplemental documents, not additional reports. Each report record binds assigned path, exact report SHA-256, bytes/lines, table count/schema digest and diagnostic flags. Previous report and JSON bytes/task identities remain unchanged.

Phase34 remains PARTIAL because its assigned `crates/crescent/src/bench.rs` is absent. This task explicitly assigns the existing CLI bench path, providing a separate completed receipt without rewriting the missing-input evidence. The verifier distinguishes current assignment completion from historical Phase34 partial status and continues validating that prior absence record. Diagnostic IN_PROGRESS/qualified_count zero does not assert a design assessment or qualification.

## Verification

The verifier uses the existing source-byte inventory helper with the Phase35 task and one assigned path. Complete JSON comparison checks the exact digest, size, line count, source coverage, task/schema/scope and diagnostic flags. Existing controls now include this file and receipt: altered source bytes, digest/size/line/path metadata after rebinding the receipt hash, missing/duplicate source records, invalid flags, strict JSON, all twenty-five report identities and relocation without Git/build output. One-source fixtures omit meaningless reorder tests; multi-source inventories retain their reorder checks.

Strict JSON parsing, safe paths, literal/decoded host-prefix checks and existing source/manifest provenance controls remain active. The detector is constructed dynamically; indexed documents and verifier text retain zero literal host-home prefixes. Hashes cannot prove semantic correctness or prevent coordinated source/anchor replacement. The index has no self-digest; exact output hashes, input snapshots and command outcomes are sealed separately in the ignored local receipt.

Verification uses `pytest -q` against the two provenance suites, plus the standalone verifier and whitespace checks, on CPUs 10,11 with `nice -n 10` and `CARGO_BUILD_JOBS=2`. Effective inherited nice is 19. Fixtures are inert file copies and synthetic JSON only. No production code is executed. Changes remain uncommitted.
