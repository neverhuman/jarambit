# Static FEM/bench source-byte provenance audit

status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)
qualified_count: 0

Task: `G16-12-RSV2-04-p34`. Baseline: `8312673`. Assignment outcome: PARTIAL because an assigned source is missing.

The assigned `crates/crescent/src/fem.rs` exists; `crates/crescent/src/bench.rs` is absent from both this checkout and the baseline Git tree. `crates/cli/src/bench.rs` exists, but it was not substituted for the assigned path or digested by this task. Clarification was requested; this receipt preserves the original path. Only the available FEM file has SHA-256, byte size and UTF-8 split-line metadata. No hash or counts are invented for a missing file.

Sources were treated as inert bytes, with no Rust semantic review, FEM/benchmark logic validation, production execution/import or source modification. No Cargo command, solver, optimization or physical evaluation was run. The companion JSON is `docs/audit/fem_bench_source_provenance_audit.json`.

## Assigned input inventory

|Source path|Availability|SHA-256|Bytes|Lines|
|---|---|---|---|---|
|crates/crescent/src/fem.rs|present|9c62d764e8df8d02513b22fa8f2ce76da5bee4c612e81e9f2f58d43ad9f7fdca|36052|500|
|crates/crescent/src/bench.rs|missing|(unavailable)|(unavailable)|(unavailable)|

SHA-256 hashes exact bytes without encoding, newline or whitespace normalization. Bytes is the byte length; Lines is `len(data.decode("utf-8").splitlines())`. A trailing newline adds no empty line. JSON records retain assigned order and distinguish requested source_count two from present_source_count one. The missing record has availability `missing` and JSON null for SHA-256, bytes and lines. Its absence is an observation, not a successful file audit or proof that no differently located benchmark exists.

## Catalog and verification

The index now contains twenty-four ordered Markdown reports: the unchanged original fifteen-report catalog, Phase26 through Phase33, then this partial receipt. Nine companion JSON receipts are supplemental documents, not additional reports. Index records bind paths, report hashes, byte/line counts, table count/schema digest and diagnostic flags. Earlier reports and JSON retain their bytes, task identities and scope limits. Indexing this receipt does not complete the missing-source portion of the assignment.

The verifier reconstructs available file fingerprints and explicit absence records for the two assigned paths, then compares the complete JSON receipt. A new file appearing at the previously absent path, changed FEM bytes, invented missing-file hashes/counts, changed metadata/coverage/schema/task/flags all fail against this receipt. File access failures other than a missing path are not silently converted into absence. Safe path resolution and symlink-escape rejection still apply.

Existing strict JSON, host-prefix, report, source and manifest checks remain active. New missing-input controls reject fabricated availability/hash metadata after rebinding the JSON receipt digest and reject a synthetic file appearing at the missing path. Existing digest controls extend to both assigned paths, all twenty-four reports, supplemental documents and relocated copies without Git/build output. Fixtures consist only of inert files and synthetic data; no production modules execute.

Verification runs on CPUs 10,11 with `nice -n 10` and `CARGO_BUILD_JOBS=2`: verifier, audit-tool tests, historical Python discovery and whitespace checks. Effective inherited nice is 19. Passing provenance checks validate the partial receipt, not the missing file. Assignment remains PARTIAL, diagnostic IN_PROGRESS/qualified_count zero. Exact output hashes, source snapshots and command outcomes are sealed in the ignored task receipt; hashes do not establish semantics or prevent coordinated anchor replacement. The index has no self-digest. Changes remain uncommitted; LANE reports PARTIAL and the missing assigned path.
