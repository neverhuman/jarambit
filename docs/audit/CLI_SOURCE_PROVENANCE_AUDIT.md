# Static CLI source-byte provenance audit

status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)
qualified_count: 0

Task: `G16-12-RSV2-04-p29`. Baseline: `605c36d`. Assignment outcome: COMPLETE for the static CLI source-digest audit.

This receipt records SHA-256, byte size and line count for `crates/cli/src/main.rs` and `crates/cli/src/auto_evolve.rs` only. Files are treated as inert bytes and decoded solely for UTF-8 split-line counting. No Rust semantics, CLI behavior, candidate optimization, physical logic or qualification criteria were inspected or validated. No production source was imported, executed or modified, and no Cargo command, solver or geometry evaluation was run.

The companion JSON is `docs/audit/cli_source_provenance_audit.json`. Assignment completion concerns byte provenance only; diagnostic IN_PROGRESS and qualified_count zero do not assert that a candidate was assessed or qualified.

## Exact source fingerprints

|Source path|SHA-256|Bytes|Lines|
|---|---|---|---|
|crates/cli/src/main.rs|ba25342ea7332f9c3113c46e9ce6dbc6ca23b3a576e623f60b428e38a02908e5|24616|329|
|crates/cli/src/auto_evolve.rs|c91e2c4744631fa1a982544bacc362804f78185dcec3215d9becdd6d2d6c1d96|65243|1560|

SHA-256 hashes exact bytes, with no encoding, newline or whitespace normalization. Bytes is the byte length. Lines is `len(data.decode("utf-8").splitlines())`; a trailing newline does not add an empty line. The two JSON source records retain the order shown above. These fingerprints detect differences relative to the recorded inputs; they do not establish authorship, correctness or successful runtime behavior.

## Nineteen-report provenance catalog

The JSON index now has nineteen ordered Markdown reports: the unchanged original fifteen-report catalog, Phase26, Phase27, Phase28, then this receipt. The four companion JSON receipts are supplemental documents, not additional reports. Each report record binds its exact digest, byte/line counts, table count/schema digest, assigned path and diagnostic flags. Earlier reports and JSON retain their exact bytes, task identities and scope limits.

The verifier reuses the source-byte inventory helper with explicit task/source inputs for this receipt, while retaining the historical Phase28 task and source defaults. It compares the entire JSON record against the current two-file inventory. Changed hashes, byte/line counts, missing/duplicate/reordered source coverage, task, schema, source count and diagnostic flags are rejected. Source files are never imported or executed.

Strict JSON, safe repository-relative paths, literal and decoded host-prefix checks, report metadata and historical source/manifest provenance checks remain active. Indexed report text, the index and verifier source contain no literal host-home prefix; the detector is constructed dynamically. The index has no self-digest. Exact output digests and input snapshots are sealed in the ignored local receipt; prior live-path index/verifier seals remain anchors to their preserved original versions.

## Verification scope and limits

Synthetic corruption controls cover both new CLI sources, all nineteen report identities, the four supplemental documents, changed digest/size/line metadata even after rebinding the receipt hash, source-coverage changes, diagnostic flags, strict JSON and relocation without Git or build output. Earlier manifest and source tests remain in place. Fixtures consist only of inert document/source copies and synthetic data.

The verifier, audit-tool tests, historical Python discovery suite and whitespace checks run on CPUs 10,11 with `nice -n 10` and `CARGO_BUILD_JOBS=2`. Effective inherited nice is 19 on this host. Test results and before/after byte equality are retained separately in the task receipt. Hash-based integrity cannot prevent coordinated replacement of sources and trusted anchors or establish semantic/physical validity. Changes remain uncommitted; LANE is emitted after checks.
