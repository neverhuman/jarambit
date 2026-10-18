# Static CI evaluator and solid-solver file-integrity/schema receipt

status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)
qualified_count: 0

Task: `G16-12-RSV2-04-p25`. Baseline: `94343b5`.

Assignment outcome: PARTIAL. This receipt covers exact file fingerprints and generic software data-integrity observations only. The requested weapon-specific physical qualification logic, loading, and boundary-condition audit was not performed. No solver configuration, physical pass criteria or design validation is established by this document. Its assigned filename does not imply the full requested engineering audit is complete.

No production module was imported or executed, and no candidate optimization, geometry generation, solver run or weapon-design comparison was performed. The solver source was handled as inert bytes for hashing, size and UTF-8 line counting; its physical implementation was not assessed. Selected evaluator source text was inspected only for generic artifact binding, serialization and failure handling. Both production files remain unchanged.

## Exact file provenance

SHA-256 hashes exact bytes. Bytes is the file length; Lines is `len(data.decode("utf-8").splitlines())`. These are fingerprints of this checkout, not claims of authorship or of a successful runtime result.

|Source path|SHA-256|Bytes|Lines|
|---|---|---|---|
|ops/ci/evaluate_cad_candidate.py|f056b5bb4a02f1df94316359bc08a8a4f4ec59ea5c5619dd5ab3e258a489a2e6|39033|788|
|fem_solid/tip_solid.py|fdf1cfcd51c78a74519f306587699fd2aebca8468cb718222a935ee0d19f43cb|13500|191|

## Generic data and artifact checks

|Inspected software surface|Static observation|Limit of this receipt|
|---|---|---|
|`_sha256`|Computes a file-content SHA-256 digest|A digest detects byte differences relative to an anchor, not authenticity|
|`solid_evidence_matches`|Compares document content identities, checks expected provenance tags and mesh-digest shape/equality, and rejects malformed numeric observations including booleans and nonfinite values in the checked pairs|This is a focused reuse predicate, not a complete JSON schema validator; physical agreement and qualification criteria are unassessed|
|`validate_solid_artifacts`|Rehashes referenced files, reparses raw JSON, requires an explicit boolean success marker, and compares recorded observations against the parsed raw records|File/metadata consistency does not prove the physical validity of an observation|
|Handled artifact failures|The artifact helper returns false for the listed file-access, missing-key, type and value exceptions|This does not establish that every malformed input throughout the evaluator fails in the same manner|
|Output serialization|The evaluator writes JSON records with provenance identifiers and artifact digests; several write sites request sorted keys and indentation|Stable formatting at individual sites does not establish a universal canonical encoding or strict schema|
|CLI completion|The CLI calls the evaluator, prints a summary and returns zero after normal completion|Process success alone is not a qualification verdict|

These observations record existing source behavior without adding import-based tests against the production evaluator. No measured values, physical thresholds or operating conditions are reconstructed here. Existing JSON parsing sites use ordinary `json.loads`; this receipt does not claim they enforce a universal strict schema or share every restriction of the separate provenance test suite.

## Diagnostic provenance verification

The new fifteenth index entry is `CI_AUDIT`. Its metadata records the diagnostic receipt as IN_PROGRESS with qualified_count zero; neither the index nor this metadata certifies a candidate. All fourteen earlier reports remain byte-identical to the baseline. The index binds every report's exact SHA-256, UTF-8 line count, byte count, table count and ordered table-schema digest. Report paths and all indexed Markdown plus the verifier are checked by the existing portable-path policy; the forbidden host-prefix detector is constructed dynamically, with no literal prefix embedded in these files.

The verifier adds two inert source fingerprints using the existing shared source-check helper. Relocated fixtures copy only documents and inert source files; they need no original checkout path, network, build output or solver execution. Negative controls corrupt each new source independently and alter each size/count column, expecting rejection. Existing report-digest and host-prefix injection tests extend automatically to the fifteenth report. These tests concern data provenance only; they do not test the physical solver or its certification decisions.

The historical JSON catalog and its archive seal are preserved. Exact legacy files retain their existing cataloged host references; no archive normalization or numerical rewriting is performed. Earlier receipts remain historical byte anchors, while original pre-update index/verifier bytes are frozen in the local task receipt.

Requested verification is Python unittest discovery on CPUs 10,11 with `nice -n 10` and `CARGO_BUILD_JOBS=2`, followed by `git diff --check`. The inherited effective nice level is 19 on this host. Actual test outcomes, before/after source equality and output digests are retained separately in the local verification receipt. Successful provenance checks do not complete the omitted weapon-specific engineering audit.
