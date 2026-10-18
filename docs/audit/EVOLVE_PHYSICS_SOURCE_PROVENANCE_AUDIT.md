# Static evolve/physics source-byte provenance audit

status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)
qualified_count: 0

Task: `G16-12-RSV2-04-p28`. Baseline: `e9380c3`. Assignment outcome: COMPLETE for the source-digest audit.

This receipt records file identity only for `crates/evolve/src/lib.rs` and `crates/physics/src/lib.rs`. The files are handled as inert bytes for SHA-256 and byte counting, then decoded as UTF-8 for line counting. No Rust semantics, algorithms, optimization logic, physical behavior or qualification criteria were inspected or validated. No production source was imported, executed or modified; no Cargo commands, geometry generation, solvers or candidate comparisons were run.

The companion JSON is `docs/audit/evolve_physics_source_provenance_audit.json`. The completed assignment status describes this byte-provenance task. Diagnostic IN_PROGRESS and qualified_count zero do not assert that any design was assessed or qualified.

## Exact fingerprints

|Source path|SHA-256|Bytes|Lines|
|---|---|---|---|
|crates/evolve/src/lib.rs|ef0ab493adbf054b76a2f7209753ba66bab4b0e8556a3aab0a744518efc17981|100734|1810|
|crates/physics/src/lib.rs|a56733ace01514f9e6366bd71ed630719e5a582f0552a99bb4820327ec1f0a3e|7526|134|

SHA-256 hashes exact file bytes, without newline, encoding or whitespace normalization. Bytes is their length. Lines is `len(data.decode("utf-8").splitlines())`; a trailing newline does not create an extra empty line. The JSON records use the same definition and fixed source order shown above. These fingerprints establish equality relative to the recorded inputs, not authorship, source correctness or physical validity.

## Catalog and verifier

The JSON provenance index now contains eighteen ordered Markdown reports: the unchanged original fifteen-report catalog, Phase26, Phase27, then this receipt. The Phase26, Phase27 and Phase28 JSON receipts are supplemental documents, not additional reports. Index records bind exact report bytes, lines, SHA-256, table count/schema digest and diagnostic flags. The source JSON is separately digest-bound. Earlier reports and JSON remain unchanged, including the Phase26 partial/unassessed contract scope and Phase27 manifest task identity.

`tools/verify_gate_feasibility_audit.py` reconstructs the two source-file fingerprints as data and compares the complete JSON record with the observed inventory. Exact ordered source coverage, byte/line counts, digests, task, schema, scope, source count and diagnostic flags are checked. No production module is loaded. Existing manifest and historical-document verification continues independently. The new JSON uses strict parsing and rejects duplicate keys, nonfinite numbers, unsafe path substitutions and literal or decoded host-home prefixes. All indexed documents, the index and verifier source retain zero literal host-home prefixes; the detector is constructed dynamically.

The verifier detects stale or tampered source bytes and metadata relative to these anchors. It cannot establish correctness or prevent coordinated replacement of both sources and trusted records. The index has no self-digest; exact output digests and original input snapshots are sealed separately in the ignored local task receipt. Prior live-file seals remain historical anchors to preserved original index/verifier versions.

## Verification scope

Synthetic fixture checks cover corruption of either source file, every indexed report and supplemental receipt, changed SHA/size/line metadata even after rebinding the JSON digest, missing/duplicate/reordered source coverage, invalid diagnostic flags, strict JSON controls, safe paths and relocation without Git or build output. Fixtures are document and inert source copies only. Existing manifest checks and the historical Python suite are retained.

Run the verifier, audit-tool tests, historical Python discovery suite and whitespace checks on CPUs 10,11 with `nice -n 10` and `CARGO_BUILD_JOBS=2`. The host inherits effective nice 19. Actual command outcomes, source equality checks and exact output fingerprints are retained in the local task receipt. Changes are left uncommitted; LANE is emitted after checks. No source-digest result is interpreted as an engineering or performance verdict.
