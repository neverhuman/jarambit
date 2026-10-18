# Static crescent-core source-byte provenance audit

status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)
qualified_count: 0

Task: `G16-12-RSV2-04-p36`. Baseline: `393a8e7`. Assignment outcome: COMPLETE for static source digests.

Scope: exact SHA-256, byte size and UTF-8 split-line count for `crates/crescent/src/lib.rs` and `crates/crescent/src/genome.rs`. The files were treated as inert bytes. No Rust semantics, genome parameter behavior, optimization, physical logic or qualification criteria were reviewed. No production source was imported, executed or modified; no Cargo command, geometry or solver was run. The companion JSON is `docs/audit/crescent_core_source_provenance_audit.json`.

## Exact fingerprints

|Source path|SHA-256|Bytes|Lines|
|---|---|---|---|
|crates/crescent/src/lib.rs|e96dd9ee4e14e0e15a01e4dc342bdc0af3d480489708a36edbaa383145009828|937|31|
|crates/crescent/src/genome.rs|c71c10976edd3305db4c9beb7117139d126878169260b7913048fc9d20545e27|20686|252|

SHA-256 hashes exact bytes, with no encoding, newline or whitespace normalization. Bytes is their length; Lines is `len(data.decode("utf-8").splitlines())`. A trailing newline adds no empty line. The JSON preserves the two-source order shown above. These fingerprints establish byte identity relative to recorded inputs, not authorship, correctness or runtime validity. The library file was also fingerprinted in Phase30; this is a separate receipt for the explicitly assigned pair.

## Catalog and historical scope

The index now covers twenty-six ordered Markdown reports: the unchanged original fifteen-report catalog, Phase26 through Phase35, then this receipt. Eleven companion JSON receipts are supplemental documents, not additional reports. Index rows bind assigned paths, report SHA-256, byte/line counts, table count/schema digest and diagnostic flags. Previous report/JSON bytes, task identities and scope limits remain unchanged, including Phase34's PARTIAL/missing crescent bench evidence.

The verifier uses the existing source-byte inventory helper with this task and two fixed paths. Full JSON comparison validates source hashes, sizes, lines, task/schema/scope, exact ordered coverage and diagnostic flags. Existing controls extend to both sources and this receipt: source corruption, digest/size/line/path metadata after rebinding JSON SHA, missing/duplicate/reordered coverage, strict JSON, all twenty-six report identities and relocation without Git/build output. The earlier single-source CLI bench and missing-source controls remain active.

Strict JSON, safe paths, literal/decoded host-prefix checks and prior source/manifest provenance continue to be checked. The host-prefix detector is constructed dynamically; indexed documents and verifier text retain zero literal host-home prefixes. Hashes cannot establish semantic correctness or prevent coordinated replacement of inputs and trusted anchors. The index has no self-digest; exact output hashes, original input snapshots and command outcomes are sealed separately in the ignored task receipt.

## Verification scope

Run `pytest -q` against both provenance suites, plus the standalone verifier and whitespace checks, on CPUs 10,11 with `nice -n 10` and `CARGO_BUILD_JOBS=2`. Effective inherited nice is 19. Fixtures are inert file copies and synthetic JSON only; no production code executes. Actual outcomes are retained in the local receipt. Current assignment completion concerns byte provenance only; diagnostic IN_PROGRESS/qualified_count zero asserts no design assessment or qualification. Prior scopes stay unchanged and changes remain uncommitted.
