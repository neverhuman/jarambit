# Static Cargo manifest byte-provenance audit

status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)
qualified_count: 0

Task: `G16-12-RSV2-04-p39`. Baseline: `4eced0b`. Assignment outcome: COMPLETE for the three assigned file fingerprints.

This receipt covers exact SHA-256, byte size and UTF-8 split-line count for `Cargo.toml`, `crates/crescent/Cargo.toml`, and `crates/cli/Cargo.toml`. All three assigned files exist. They were handled as inert bytes; no Cargo resolution, metadata/build/test commands, downloads, build scripts or production code were executed. No package compatibility, resolved dependency versions or physical behavior are assessed. The companion JSON is `docs/audit/cargo_manifest_provenance_audit.json`.

## Exact fingerprints

|Manifest path|SHA-256|Bytes|Lines|
|---|---|---|---|
|Cargo.toml|aa338525eac815da0ddf6557d150c0932f10c170985e9e7ff1cef1b4bcc50a1e|347|13|
|crates/crescent/Cargo.toml|07970e5be1e735f13fcc6a73bc47da51015405a82e9e26b301733875ccf68556|497|18|
|crates/cli/Cargo.toml|be4282631bd8747ad295b497fcb5fedde6088967488d7f1fd7aa1c011fbe8f74|590|18|

SHA-256 hashes exact bytes without encoding, newline or whitespace normalization. Bytes is the byte length; Lines is `len(data.decode("utf-8").splitlines())`. A trailing newline adds no empty line. JSON retains the root/crescent/CLI order above and source_count three. These hashes establish equality relative to recorded files, not publisher authenticity, dependency safety, semantic correctness or successful compilation.

## Catalog and historical scope

The index now covers twenty-nine ordered Markdown reports: the unchanged original fifteen-report catalog, Phase26 through Phase38, then this receipt. Fourteen companion JSON receipts are supplemental documents, not additional reports. Index rows bind assigned paths, exact report SHA-256, byte/line counts, table count/schema digest and diagnostic flags. Every earlier report/JSON retains its bytes, task identity and scope, including prior partial/missing-input evidence and Phase38's documented actual-file fallback.

Phase27's broader workspace metadata receipt remains unchanged and independently verified. This task adds a focused three-file byte inventory; it does not revise that earlier inventory or claim new dependency resolution. The verifier uses the existing source-byte inventory helper with the current task and three fixed manifest paths, comparing the complete JSON record to current file fingerprints. Stale digest/size/line metadata, source coverage/order, task/schema/scope and diagnostic flags are rejected.

Existing workspace-manifest corruption tests already cover every assigned input, including valid TOML changes, missing/extra files and metadata/inheritance changes. New receipt controls extend generic JSON digest/size/line/path tampering after receipt-hash rebinding, missing/duplicate/reordered coverage, strict JSON, all twenty-nine report identities and relocated document-only copies. TOML fixtures use valid comment changes; tests do not invoke Cargo. Earlier source, single/missing-source and fallback-selection checks remain active.

## Verification and limits

Run `pytest -q` against both provenance suites, the standalone verifier and whitespace checks on CPUs 10,11 with `nice -n 10` and `CARGO_BUILD_JOBS=2`. Effective inherited nice is 19. Fixtures are inert file copies and synthetic JSON/TOML only; no production code executes. Strict JSON, safe paths and literal/decoded host-prefix checks remain active. The detector is constructed dynamically; indexed documents and verifier text retain zero literal host-home prefixes.

Assignment completion concerns static byte provenance only; diagnostic IN_PROGRESS/qualified_count zero asserts no design assessment or qualification. Hashes cannot prove semantics or prevent coordinated input/anchor replacement. The index has no self-digest; exact output hashes, original snapshots and command outcomes are sealed separately in the ignored local task receipt. Prior live-file seals refer to preserved originals. Changes remain uncommitted.
