# Static CLI main source-byte provenance audit

status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)
qualified_count: 0

Task: `G16-12-RSV2-04-p37`. Baseline: `0a090af`. Assignment outcome: PARTIAL because an assigned source is absent.

`crates/cli/src/main.rs` exists; `crates/cli/src/lib.rs` is absent from the checkout and baseline Git tree. This receipt preserves both assigned paths and records absence explicitly. It does not substitute another library file, create the missing source, or invent a digest/count. The companion JSON is `docs/audit/cli_main_source_provenance_audit.json`.

Only exact file SHA-256, byte size and UTF-8 split-line counts are in scope. Available source was treated as inert bytes; no Rust semantics, CLI behavior, optimization or physical logic were reviewed. No production source was imported, executed or modified; no Cargo commands, solvers or geometry were run.

## Assigned input inventory

|Source path|Availability|SHA-256|Bytes|Lines|
|---|---|---|---|---|
|crates/cli/src/main.rs|present|ba25342ea7332f9c3113c46e9ce6dbc6ca23b3a576e623f60b428e38a02908e5|24616|329|
|crates/cli/src/lib.rs|missing|(unavailable)|(unavailable)|(unavailable)|

SHA-256 hashes exact bytes without encoding, newline or whitespace normalization. Bytes is the byte length; Lines is `len(data.decode("utf-8").splitlines())`. A trailing newline adds no empty line. JSON preserves assigned order, source_count two, present_source_count one, and an explicit missing_sources list. The missing file has availability `missing` and null SHA-256/bytes/lines. Byte identity does not establish authorship, semantics or runtime validity.

## Catalog and historical scope

The index contains twenty-seven ordered Markdown reports: the unchanged original fifteen-report catalog, Phase26 through Phase36, then this partial receipt. Twelve companion JSON receipts are supplemental documents, not additional reports. Index records bind assigned paths, exact report SHA-256, byte/line counts, table count/schema digest and diagnostic flags. Every earlier report/JSON retains its bytes, task identity and scope, including Phase34's separate missing crescent bench evidence.

The verifier shares an absence-aware byte inventory helper for the two partial receipts, preserving Phase34's original task identity through its existing wrapper. It compares complete JSON inventories, rejecting changed available-source hashes/counts, fabricated missing-file metadata, changed ordered coverage/task/schema/scope/flags and an unrecorded file appearing at a missing path. Other file-access errors are not silently converted into absence. Current CLI-main missing paths are returned separately from historical FEM/bench missing paths.

Strict JSON, safe paths and literal/decoded host-prefix checks remain active alongside all earlier source/manifest inventories. Indexed documents and verifier text retain zero literal host-home prefixes; the detector is constructed dynamically. Synthetic controls cover both current paths, rebinding JSON hashes before metadata tampering, missing/duplicate/reordered source coverage, fabricated missing evidence, newly appearing files, all twenty-seven report identities and relocated document-only copies. Fixtures contain inert files and synthetic JSON only.

## Verification and limitations

Run `pytest -q` across both provenance suites, the standalone verifier and whitespace checks on CPUs 10,11 with `nice -n 10` and `CARGO_BUILD_JOBS=2`. Effective inherited nice is 19. Passing checks validate this partial receipt; they cannot supply a missing file's digest or complete the assigned pair. Assignment remains PARTIAL, diagnostic IN_PROGRESS/qualified_count zero. No design assessment or qualification is asserted.

Hashes cannot prove semantics or prevent coordinated replacement of inputs and trusted anchors. The index has no self-digest; exact output hashes, available source snapshots, absence evidence and command outcomes are sealed separately in the ignored local receipt. Earlier live-file seals refer to preserved original versions. Changes remain uncommitted.
