# Static Stage-1/Stage-2 pipeline and hard-gate separation audit

status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)
qualified_count: 0

Task: `G16-12-RSV2-04-p24`. Reviewed baseline: `fbc1596`.

This is a static software-control-flow and provenance receipt. No candidate optimization, physical evaluation, geometry generation, weapon-design comparison, or production source change was performed. The status describes this diagnostic receipt; it is not a certification verdict for any candidate. File fingerprints bind the inspected implementation, not runtime solver success.

## Source provenance

The two requested primary sources are `crates/cli/src/auto_evolve.rs` and `crates/crescent/src/certification.rs`. Supporting sources below establish transitive evaluator dispatch and the distinction between plate area and exported-surface area. Exact bytes, UTF-8 split-line counts and SHA-256 are checked by the document verifier without importing or executing audited production code.

|Source path|SHA-256|Bytes|Lines|
|---|---|---|---|
|crates/cli/src/auto_evolve.rs|c91e2c4744631fa1a982544bacc362804f78185dcec3215d9becdd6d2d6c1d96|65243|1560|
|crates/crescent/src/certification.rs|eeefc8b1ecbf21df85aefb967e351ada4f0070f8b481959afef419379215c4ac|14755|364|
|crates/crescent/src/qd.rs|4166fc38a8879b869b378247b3fd527ca82854796369765f6f3c680ca0a02311|27783|364|
|ops/ci/score_cad.py|8d2691c087d63c1f66b4de06b6fa95a7bc577968b26aa06c7bd0a35ca04c9901|55629|1115|
|ops/ci/evaluate_cad_candidate.py|f056b5bb4a02f1df94316359bc08a8a4f4ec59ea5c5619dd5ab3e258a489a2e6|39033|788|
|crates/crescent/examples/export_step.rs|1cb6d62b3c15692944659199f9de57446440b3e6c1994adcf8813e69d17a1ff8|1332|19|
|crates/crescent/src/fem.rs|9c62d764e8df8d02513b22fa8f2ce76da5bee4c612e81e9f2f58d43ad9f7fdca|36052|500|
|crates/crescent/src/export.rs|575eef5f12aebc81a345ef2fe7f52a51ab73e6f7ed837eb1c5e556f2e81bec06|10067|172|
|crates/cli/src/crescent.rs|c8f2e1664263e3bb885ca33e27f42dab0fbe0f180116ec890ee4dcd83bdeab22|57901|561|

## Stage boundaries and verdict ownership

|Boundary|Observed implementation|Qualification consequence|
|---|---|---|
|Stage-1 screening|`screen_candidate_stage1` calls `qd::evaluate(g, idx, round, 2.0)`; `qd::evaluate` calls unbiased `fem::run` when its preceding checks permit a FEM result|Coarse 2.0 mm² plate observations are provisional only|
|Provisional record|`ScreenSurvivor` holds genome, display identity, mass and screening observations/deficit; it has no qualification flag|No Stage-1 metric can issue a qualification verdict in this implementation|
|Screen ordering|Ascending `screen_deficit`, then mass; screening JSON explicitly says `pending independent top-k assessment`|A low deficit, low mass or selected rank is not a hard-gate pass|
|Stage-2 selection|Inline loops in `run_auto_evolve_cli` choose up to `certify_top`, preferring family diversity, then filling distinct genomes|Selection schedules assessment; it does not assert qualification|
|Independent assessment|`certify_candidate_stage2` writes canonical genome input in a fresh content-identity attempt directory and invokes `ops/ci/score_cad.py`|Stage-2 measurements are read from the new scorecard, not copied from the screen|
|Assessment storage|Successful parsing creates `AssessedDesign` and appends it to `assessed_all`, even when gate booleans are false|An available assessment is not necessarily qualified|
|Final qualification|Final qualified list and row flag use `all_16_gates && rs_v2_all_hard` from assessed records|Only both Stage-2 scorecard flags being true can produce the final qualification flag|
|Assessment errors|Failed process, missing required data or identity mismatch yields an error retained as `unverified`|An error is not admitted as a qualified assessed record|

`select_stage2_candidates` and `round_parent_identities` do not exist in this checkout. The actual selection implementation is inline, and parent retention is implemented by `replacement_elites`; no missing helper was invented or added. The selection variable `already_certified` checks membership in `assessed_all`, which includes failed gate assessments, rather than only qualified records. It also avoids duplicate genomes within the selected batch.

`replacement_elites` starts with distinct genomes from assessed records whose two qualification flags are true, then can retain provisional survivors with family diversity and distinct-genome filling. Previous parents are retained if the replacement set is empty. A provisional genome becoming a parent does not change its qualification status. Recorded genome identities in round artifacts are provenance, not pass flags.

## Transitive Stage-2 certification path

The coarse screening function does not run the following certification chain. Stage-2's genome-backed `score_cad.py` path delegates to `evaluate_cad_candidate.evaluate_candidate`; the default invocation retains solid evaluation. These are statically observed call sites, not evidence that this task executed any subprocess or obtained a passing result.

|Requested component|Source evidence and dispatch|Separation and limits|
|---|---|---|
|0.25/0.125 mm² plate|`fem.rs` defines `CERT_AREAS = [0.25, 0.125]`; `fem::certify` calls both unbiased plate runs. The evaluator launches `crescent-d1` with `D1_PLATES=1` and `JARAMBIT_CERT_STRICT=1`|The CLI certification path also requests strict unbiased 0.0625 mm² plate work; Stage-1's 2.0 mm² result does not substitute for these runs|
|0.25 mm² exported solid|`export_step.rs` calls `export::solid` with 1.0, while `export.rs` caps the RS-v2 maximum surface area with `max_area.min(0.25)`; certification measurement also uses its export-area constant of 1.0 through that cap|0.25 is an effective maximum surface-area cap, not a claim that every triangle has exactly that area or that the caller passes 0.25|
|P2 TET10 tip and independent comparison|The evaluator dispatches the primary solid-tip solver and an independent DOLFINx comparison using a shared, hashed mesh archive, preserving primary raw-output binding|P2/independent 3D evidence is separate from the 2D plate screen; plate area and tetrahedral refinement are different quantities|
|`plate_gci_probe`|The evaluator invokes the example and retains its artifact; the source identifies the ordinary 4, 1, 0.25 area sequence|A probe invocation is part of the assessment path, not a GCI pass inferred by this static audit|
|`g3_side_report`|The evaluator invokes the example and side evaluation, retaining the resulting JSON|Side results belong to independent assessment rather than provisional screen observations|

`score_cad.py` accepts a genome-backed input and defaults to running solid evaluation; Stage-2 supplies neither the standalone bypass nor the no-solid option. The standalone score path explicitly disables promotion and the RS-v2 hard-gate flag. A completed score process can return exit code zero with failing gate booleans; subprocess success must not be confused with qualification.

## Binding, validation and trust limits

`certify_candidate_stage2` first requires successful subprocess completion and a readable master scorecard. It compares the scorecard's `certificate_identity` against `certification::identity(g)`. The identity incorporates material, exact genome floating-point bit patterns, requirement revision, optional factor mapping and numerical implementation text. Display labels do not replace this binding. `bound` is an identity-string comparison; it alone does not independently remeasure geometry or verify every referenced artifact.

Required scorecard numeric fields must be finite JSON numbers, and `battery_pass` must be an integer. Missing, malformed or nonfinite required data fails assessment. Each qualification boolean passes only when it is exactly JSON `true`; absent or nonboolean values become false. `AssessedDesign` receives independent scorecard measurements; the screen contributes only the genome and display label/family. The CLI trusts the bound scorecard's two gate booleans rather than recomputing every physical predicate locally.

In `certification.rs`, `measure` requires the expected topology census and positive volume according to its literal comparisons. The independent-comparison helper recognizes supported solver labels, checks mesh-identity strings and required run identifiers, requires finite positive primary/secondary values and matching stored secondary observations, and recomputes the two-percent comparisons. It does not accept a stored `within_2pct` flag as proof. Invalid comparison evidence yields no established comparison; valid out-of-tolerance evidence yields a failed comparison. This helper does not by itself execute solvers or validate the actual contents of external mesh files.

The final report derives qualification solely from the conjunction of the two assessed gate flags. No screening deficit, screening mass, provisional rank, parent selection or Stage-1 observation participates in that predicate. The command can still finish successfully with zero qualified records when assessments complete without errors. This audit establishes that source-level separation and records its trust boundary; it does not certify an actual candidate or independently prove the scorecard's physical claims.

## Verification scope

The provenance index now has fourteen ordered diagnostic entries: V5 through V14, three previous audits and this audit. The verifier checks each report's exact digest, line/byte counts, table schema, diagnostic status and zero qualification count in index metadata. It additionally checks these nine source fingerprints and their byte/line metadata, with corruption controls and relocation into document-only copies. All indexed Markdown, the index and verifier source must contain zero literal host-home prefixes. The detector is assembled dynamically.

Historical report bytes and the archived JSON catalog remain unchanged. Portable JSON remains distinct from the exact legacy archive policy; no historical host paths are rewritten. The required command is Python unittest discovery under CPUs 10,11 and requested nice increment 10 with `CARGO_BUILD_JOBS=2`, followed by `git diff --check`. The process inherits effective nice 19 on this host; no priority escalation is attempted. Runtime outcomes and exact output digests are retained in the local task receipt after verification.
