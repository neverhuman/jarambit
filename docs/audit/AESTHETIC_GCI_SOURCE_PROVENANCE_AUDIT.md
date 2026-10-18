# Static aesthetic/gci source-byte provenance audit

status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)
qualified_count: 0

Task: `G16-12-RSV2-04-p32`. Baseline: `325d109`. Assignment outcome: COMPLETE for static source digests.

Scope: SHA-256, byte size and UTF-8 split-line count only for `crates/crescent/src/aesthetic.rs` and `crates/crescent/src/gci.rs`. Sources were handled as inert bytes, with no Rust semantic review, aesthetic/gci logic validation, production import/execution or source modification. No Cargo commands, geometry, optimization or physical evaluation were run. The companion JSON is `docs/audit/aesthetic_gci_source_provenance_audit.json`.

## Exact fingerprints

|Source path|SHA-256|Bytes|Lines|
|---|---|---|---|
|crates/crescent/src/aesthetic.rs|30404b5e898613fd36b6ea534d501a57ddf1898567ee545807bd2206c7b5282a|7507|121|
|crates/crescent/src/gci.rs|ba3333de3828db2f9602d5fbb4078d03ac7bb2f380a85f90910ef8cc5f6f97a9|16364|505|

SHA-256 hashes exact bytes without encoding, newline or whitespace normalization. Bytes is the byte length; Lines is `len(data.decode("utf-8").splitlines())`. A trailing newline creates no extra empty line. JSON records retain the source order shown above. These fingerprints establish equality to recorded inputs, not authorship, correctness or runtime validity.

## Catalog and verification

The JSON index now covers twenty-two ordered Markdown reports: the unchanged original fifteen-report catalog, Phase26 through Phase31, then this receipt. The seven companion JSON receipts are supplemental documents, not additional reports. Index rows bind assigned paths, exact report hashes, byte/line counts, table count/schema digest and diagnostic flags. Earlier reports/JSON retain their exact bytes, task identities and scope limits.

The verifier reuses the source-byte inventory helper with this task and the two assigned paths. Full JSON comparison checks source digests, byte/line counts, exact ordered coverage, schema, task, scope and diagnostic flags. Earlier source and manifest inventories remain active. Strict JSON, safe paths, literal/decoded host-prefix checks, report metadata and relocation controls are retained. The host-prefix detector is constructed dynamically; indexed documents and verifier text contain zero literal host-home prefixes.

Existing corruption controls extend to both new sources and the new receipt: changed SHA/size/line/path metadata after rebinding the JSON digest, missing/duplicate/reordered source coverage, invalid flags, strict JSON, all twenty-two report identities and relocated document-only copies. Fixtures are inert file copies and synthetic data, with no production module execution. The verifier, audit-tool tests, historical Python discovery suite and whitespace checks run on CPUs 10,11 using `nice -n 10` and `CARGO_BUILD_JOBS=2`; inherited effective nice is 19.

Assignment completion concerns byte provenance only. Diagnostic IN_PROGRESS/qualified_count zero asserts no design assessment or qualification. Hashes cannot establish semantic correctness or prevent coordinated replacement of data and trusted anchors. The index has no self-digest; exact output hashes, original snapshots and command outcomes are sealed in the ignored local task receipt. Earlier live-file seals refer to preserved original versions. Changes remain uncommitted; LANE is emitted after checks.
