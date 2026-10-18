# Static crescent gates/metrics source-byte provenance audit

status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)
qualified_count: 0

Task: `G16-12-RSV2-04-p38`. Baseline: `f9b908e`. Assignment outcome: COMPLETE for the explicitly permitted actual-file provenance audit.

Requested paths were `crates/crescent/src/gates.rs` and `crates/crescent/src/metrics.rs`, with permission to use actual files in that directory. `gates.rs` exists; `metrics.rs` is absent in the checkout and baseline Git tree. This audit selects the existing `crates/crescent/src/record.rs` as the actual-file fallback under that allowance. No semantic equivalence between record.rs and metrics.rs is asserted or assessed, and no digest is attributed to the absent metrics.rs.

The two actual files below were handled as inert bytes for exact SHA-256, byte size and UTF-8 line counting only. No gate logic, metric definitions, Rust semantics, optimization, physical behavior or qualification criteria were reviewed. No production source was imported, executed or modified; no Cargo command, geometry or solver was run. The companion JSON is `docs/audit/crescent_gates_metrics_provenance_audit.json`.

## Actual audited files

|Source path|SHA-256|Bytes|Lines|
|---|---|---|---|
|crates/crescent/src/gates.rs|0e364aab4d5d16d43d4e2c95c2e393707edb35b614f35d4b9eaf11c4303f28d7|5204|68|
|crates/crescent/src/record.rs|5cf69d38d9d78766e045ee66427b143e063c5e9e25b6b4adb777582826ecbccb|7903|111|

SHA-256 hashes exact bytes without encoding, newline or whitespace normalization. Bytes is their length; Lines is `len(data.decode("utf-8").splitlines())`. A trailing newline adds no empty line. The two-source JSON order matches the table. Fingerprints establish byte identity relative to recorded inputs, not authorship, semantics or runtime validity.

The JSON also records original requested paths, `requested_metrics_path_exists: false`, the actual fallback path, `semantic_equivalence_assessed: false`, and the selection basis. The verifier reconstructs this selection metadata as well as actual file fingerprints. If metrics.rs appears later, the historical choice requires review rather than silently remaining current. A file being selected here establishes no relationship between its contents and the missing module.

## Catalog and historical scope

The JSON index covers twenty-eight ordered Markdown reports: the unchanged original fifteen-report catalog, Phase26 through Phase37, then this receipt. Thirteen companion JSON receipts are supplemental documents, not additional reports. Index records bind paths, exact report SHA-256, byte/line counts, table count/schema digest and diagnostic flags. Every earlier report/JSON keeps its bytes, task identity and scope, including Phase34 and Phase37's separate partial/missing-input evidence. Current completion uses the explicit actual-file allowance and does not rewrite either earlier task.

The verifier uses the existing source-byte inventory helper with two fixed actual paths plus the explicit selection record. Full JSON comparison rejects changed digests/counts, ordered coverage, task/schema/scope/flags or fallback metadata. Existing source, manifest, missing-source, strict JSON, safe path and literal/decoded host-prefix checks remain active. The host-prefix detector is constructed dynamically; indexed documents and verifier text retain zero literal host-home prefixes.

## Verification scope

Synthetic controls extend to both actual files, this receipt, metadata after rebinding JSON SHA, missing/duplicate/reordered source coverage, strict JSON, all twenty-eight report identities and relocated document-only copies. Additional selection controls reject altered fallback paths, a false semantic-equivalence claim, fabricated requested-path availability, and a synthetic metrics.rs appearing after this receipt. Fixtures are inert file copies and synthetic data only.

Run `pytest -q` against both provenance suites, the standalone verifier and whitespace checks on CPUs 10,11 with `nice -n 10` and `CARGO_BUILD_JOBS=2`. Effective inherited nice is 19. Passing provenance checks establish no engineering verdict. Diagnostic IN_PROGRESS/qualified_count zero asserts no design assessment or qualification. Hashes cannot prevent coordinated input/anchor replacement or prove semantics; the index has no self-digest. Exact output hashes, input snapshots, absence evidence and command outcomes are sealed separately in the ignored local receipt. Changes remain uncommitted.
