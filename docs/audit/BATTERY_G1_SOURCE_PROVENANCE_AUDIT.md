# Static battery/g1 source-byte provenance audit

status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)
qualified_count: 0

Task: `G16-12-RSV2-04-p33`. Baseline: `c13bce4`. Assignment outcome: COMPLETE for static source digests.

Scope: SHA-256, byte size and UTF-8 split-line count only for `crates/crescent/src/battery.rs` and `crates/crescent/src/g1.rs`. Sources were handled as inert bytes, with no Rust semantic review, battery/g1 logic validation, production import/execution or source modification. No Cargo commands, geometry, optimization or physical evaluation were run. The companion JSON is `docs/audit/battery_g1_source_provenance_audit.json`.

## Exact fingerprints

|Source path|SHA-256|Bytes|Lines|
|---|---|---|---|
|crates/crescent/src/battery.rs|b84e1f2a5795eddb207142e548541e07d06d8d05a6f56f88a67b801a2afd341d|167490|4211|
|crates/crescent/src/g1.rs|650b09e2f63af45163f77925a93127a53902bbad775118f152aff19e6fdc877b|7784|245|

SHA-256 hashes exact bytes without encoding, newline or whitespace normalization. Bytes is the byte length; Lines is `len(data.decode("utf-8").splitlines())`. A trailing newline creates no extra empty line. JSON records retain the source order shown above. These fingerprints establish equality to recorded inputs, not authorship, correctness or runtime validity.

## Catalog and verification

The JSON index now covers twenty-three ordered Markdown reports: the unchanged original fifteen-report catalog, Phase26 through Phase32, then this receipt. The eight companion JSON receipts are supplemental documents, not additional reports. Index rows bind assigned paths, exact report hashes, byte/line counts, table count/schema digest and diagnostic flags. Earlier reports/JSON retain their exact bytes, task identities and scope limits.

The verifier reuses the source-byte inventory helper with this task and the two assigned paths. Full JSON comparison checks source digests, byte/line counts, exact ordered coverage, schema, task, scope and diagnostic flags. Earlier source and manifest inventories remain active. Strict JSON, safe paths, literal/decoded host-prefix checks, report metadata and relocation controls are retained. The host-prefix detector is constructed dynamically; indexed documents and verifier text contain zero literal host-home prefixes.

Existing corruption controls extend to both new sources and the new receipt: changed SHA/size/line/path metadata after rebinding the JSON digest, missing/duplicate/reordered source coverage, invalid flags, strict JSON, all twenty-three report identities and relocated document-only copies. Fixtures are inert file copies and synthetic data, with no production module execution. The verifier, audit-tool tests, historical Python discovery suite and whitespace checks run on CPUs 10,11 using `nice -n 10` and `CARGO_BUILD_JOBS=2`; inherited effective nice is 19.

Assignment completion concerns byte provenance only. Diagnostic IN_PROGRESS/qualified_count zero asserts no design assessment or qualification. Hashes cannot establish semantic correctness or prevent coordinated replacement of data and trusted anchors. The index has no self-digest; exact output hashes, original snapshots and command outcomes are sealed in the ignored local task receipt. Earlier live-file seals refer to preserved original versions. Changes remain uncommitted; LANE is emitted after checks.
