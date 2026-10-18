# Static crescent source-byte provenance audit

status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)
qualified_count: 0

Task: `G16-12-RSV2-04-p30`. Baseline: `6edc6e7`. Assignment outcome: COMPLETE for static source digests.

Scope: file identity only for `crates/crescent/src/lib.rs` and `crates/crescent/src/gates.rs`. Files were handled as inert bytes and decoded solely for UTF-8 line counting. No Rust semantics, gate logic, physical behavior or qualification criteria were reviewed. No production source was imported, executed or modified; no Cargo command, solver, optimization or geometry evaluation was run. The companion JSON is `docs/audit/crescent_source_provenance_audit.json`.

## Exact fingerprints

|Source path|SHA-256|Bytes|Lines|
|---|---|---|---|
|crates/crescent/src/lib.rs|e96dd9ee4e14e0e15a01e4dc342bdc0af3d480489708a36edbaa383145009828|937|31|
|crates/crescent/src/gates.rs|0e364aab4d5d16d43d4e2c95c2e393707edb35b614f35d4b9eaf11c4303f28d7|5204|68|

SHA-256 hashes exact bytes without encoding, newline or whitespace normalization. Bytes is their length; Lines is `len(data.decode("utf-8").splitlines())`. A trailing newline does not create an extra empty line. JSON source records retain the order shown above. These fingerprints establish equality relative to recorded inputs, not authorship, correctness or a runtime result.

## Catalog and verification

The index now contains twenty ordered Markdown reports: the unchanged original fifteen-report catalog, Phase26 through Phase29, then this receipt. The five companion JSON receipts are supplemental documents, not additional reports. Each report record binds assigned path, exact SHA-256, byte/line counts, table count/schema digest and diagnostic flags. All earlier reports and JSON retain their original bytes, task identities and scope limits.

The verifier reuses the source-byte inventory helper with the Phase30 task and two assigned source paths. It compares the entire JSON record to fresh file fingerprints, rejecting stale digest/size/line metadata, changed source coverage, task/schema/count discrepancies and invalid diagnostic flags. Existing source and manifest inventories remain active. Strict JSON, safe paths, literal/decoded host-prefix controls, report metadata and relocated document fixtures are retained. The detector is constructed dynamically; indexed documents and verifier text retain zero literal host-home prefixes.

Synthetic controls cover corruption of both new sources, changed digest/size/line/path metadata even after rebinding the receipt digest, missing/duplicate/reordered source coverage, invalid flags, strict JSON, all twenty report identities, supplemental documents and relocation without Git or build output. Fixtures are inert document/source copies only. Verifier, audit-tool tests, historical Python discovery and whitespace checks run on CPUs 10,11 with `nice -n 10` and `CARGO_BUILD_JOBS=2`; effective inherited nice is 19.

Assignment completion concerns this byte-provenance task only. Diagnostic IN_PROGRESS/qualified_count zero does not assert a design assessment or qualification. Hashes cannot establish semantic validity or prevent coordinated replacement of data and trusted anchors. The index has no self-digest; exact output hashes, original input snapshots and command outcomes are sealed in the ignored local task receipt. Earlier live-file seals refer to preserved original versions. Changes remain uncommitted; LANE is emitted after checks.
