# Author red-team review

Review date: 18 October 2026 (America/Denver). Baseline: `009f5ac520fa3afe8ed8025230eef604dbb7f922`.

This is an author adversarial review, not independent peer review, a separate agent review, or the repository-required `neverhumanbot` approval. No new mechanics, CAD optimization, manufacturing or physical test was performed.

| Challenge | Evidence / attack | Disposition |
|---|---|---|
| Inflated test count | Try adding 16 predicates, ten groups, 414 cells and 356 slots into a single claim | Rejected. README, atlas and paper explain overlap and different denominators. |
| Stored pass mistaken for physical success | Inspect `solid.tip` and four repeated `build.*` columns | Fixed wording: completeness and shared capability remain distinct from favorable numerical findings. |
| Blocked converted to failed | Inspect `gate.rs_v2` 19/4 distribution | Preserve nineteen unfavorable and four blocked records. |
| Current execution inferred from old corpus | Inspect every historical freshness flag | All cells retain `fresh_this_lane: false`; generator refuses changed freshness semantics. |
| Wrong candidate silently read from manifest path | Substitute internal subject name in a subject record | Reader requires internal subject to match manifest filename and exact subject membership. |
| Duplicate/missing identities | Duplicate IDs or remove evaluator | Unique IDs and exact evaluator-set membership are required. |
| Score direction reversed to match request | Relabel old weighted values as loss | Rejected. Actual higher-is-better score and future requested convention are stated separately. |
| Common label hides different scorers | Compare native Stage 1, native Stage 2 and reporting implementation | Placeholder/constant/proxy differences are explicit; figure is labeled retained reporting profile. |
| Three points advertised as convergence | Fit long-run trend to weighted curve | Rejected. No extrapolation, scaling or optimum claim. Hall-of-Fame recurrence checked exactly. |
| Selected leaderboard called population | Infer weighted birth distribution from six assessed entries | Rejected. Round files unavailable; full-birth analysis uses separately labeled historical 1,984-record cohort. |
| Winner geometry substituted | Search retained weighted cohort for best candidate geometry | Missing. Opening render uses the exact assessed example; historical population/maxima are explicitly a different cohort. Weighted summary is retained in results with incomplete stage lineage disclosed. |
| Incompatible history joined | Connect historical `fit` and weighted score | Separate panels with explicit metric regimes; no cross-regime curve. |
| Outliers conceal population behavior | Initial linear full-range plot compressed central values | Fixed with labeled asinh vertical transform, original-value ticks, and every birth preserved. |
| Population artificially smoothed | Replace births with interpolated points | Rejected. Raw births plus exact generation medians/maxima; no imputation. |
| Metadata creates truth | Treat SHA or current source file as execution/validation | Rejected throughout. Identity, execution, finding and physical validation remain separate. |
| Agent novelty implied by prose | Describe superior agent-designed genomes as achieved | Rejected. Agents section is prospective research, with explicit independent evaluation needs. |
| Manufacturing tolerance overstated | Conclude that titanium printing guarantees unprecedented precision | Rejected. Geometric freedom is separate from measured dimensional accuracy and uncertainty. |
| Physical reliability implied | Treat modeled factor or aesthetic proxy as product qualification | Rejected. No qualified product, physical validation or manufacturing promotion claimed. |
| Authorship or venue fabricated | Add affiliation, DOI, acceptance, badge or autonomous author | Rejected. Human approval and independent scientific review pending; assistance disclosed. |
| Source projections silently drift | Change generator input after generation | Read-only freshness check fails; source and output hashes retained. |
| Renderer path escape | Use absolute/parent/symlink path | Parent/absolute paths and symlink components refused by publication path helper. |
| IEEE layout overclaimed | Call custom cover an unmodified venue template | Explicit IEEE-style white-paper distinction; venue conversion still needed. |
| Dense paper hides typesetting defects | Compile first draft and inspect all pages | Bibliography overflow repaired; custom first page and two-column body rendered and inspected. |
| Cell coefficients mislabeled as stress | Compare retained D1 arrays with the original producer and stress-recovery implementation | Envelope is scaled MPa; lateral/ring are MPa/N; torsion is MPa/(N mm). Independent full-range legends and source-bound interpretation. |
| Producer-forced zeros imply no stress | Compare centroid masks with stored zeros and coordinate-rounding bounds | Gray masks 75 envelope and 14,875 seated-ring cells. One positive ring coefficient resolves rounded boundary membership; ambiguous zeros are refused. Raw tip unit responses remain visible. |
| Coarse field substitutes for refined or solid result | Trace which mesh supplies retained arrays versus reported capacity minima | Figure explicitly uses the 0.25 mm² plate level. Capacity summaries combine two levels; strict/solid/fatigue/buckling fields are not invented. |
| Heatmap hides a high response | Clip the envelope at a material constant or smooth cell values | Rejected. All 100,836 triangle/value associations checked against saved arrays; full source maxima, no clipping or interpolation. |
| Valid convergence metadata implies a valid error estimate | Inspect the actual three-level assessed-example study | Metadata are valid, but recorded nonpositive order and oscillation refuse both estimates. Six observations plotted; null GCI/extrapolates stay null. |
| Two profiles imply a full-field independent comparison | Inspect original plate-cell and solid vertex-mean bin definitions | Sixteen bin maxima displayed with original intervals and distinct recovery/region semantics. Matched-solver scalar pairs are a separate panel; no spatial difference field is inferred. |
| Historical projection tag conceals implemented element order | Follow assessed in-plane calls beyond generic CST comments | Corrected methods to identify T6 in-plane cases and centroid field versus vertex-recovery capacity semantics; original metadata tags preserved. |

## Residual limitations

- Retrospective single-repository evidence does not establish a new state-of-the-art optimization result.
- No controlled agent-representation experiment, independent reconstruction study or manufacturing-tolerance experiment is supplied.
- Weighted reporting placeholders and native/reporting score differences remain scientific-source limitations; this publication change does not repair or tune them.
- Historical cell semantics and source revisions remain heterogeneous. The explanatory projection preserves them rather than reissuing scientific evidence.
- Publication tooling checks retained data and layout. It does not verify every original solver execution or guarantee physical correctness.
- Independent exact-head bot approval and required PR CI remain merge conditions. This author review cannot satisfy that approval.
- The PDF now embeds lossless high-resolution PNG projections while retaining the original source SVGs. This removes the duplicate `/Group` keys from the former SVG-PDF import path and avoids dense triangle-edge seams. Current measured page counts, strict pypdf parsing and final page rendering are recorded in the dated progress receipt. This is a local parser/layout check, not formal PDF/A validation or cross-toolchain byte-reproducibility proof.

## Local broad-gate finding

The 19 October 2026 Rust 1.85.1 release workspace run on `08ed64b1dd31` again failed `tests::d1_knife_fixture_reports_torsion_and_couple`; an isolated exact-test rerun reproduced the assertion at `crates/crescent/src/tests.rs:206`. This scientific source, its mechanics dependencies and the root lock match main `72e25a360470` byte-for-byte. This task preserves the test and requirement definitions and does not claim the workspace gate passed. Later local gate phases remain incomplete. Required PR CI and independent review must resolve merge readiness.

## A4 eigen-operator contract: open P1

**Found 19 October 2026 during scalar-visual admission.** The [original producer](../../ops/ci/side_a4.py) constructs a `LinearOperator` applying `K_support^-1 G_mat`, passes it to `scipy.sparse.linalg.eigsh` without a generalized mass argument or explicit inner-product transform, and reciprocates its selected eigenvalue. The [official API contract](https://docs.scipy.org/doc/scipy/reference/generated/scipy.sparse.linalg.eigsh.html), inspected 19 October, requires the supplied operator to be real symmetric or complex Hermitian. The archived A4 report does not record the SciPy version, operator-symmetry evidence, eigenvector or eigenpair residual.

This is a **verification gap**, not a demonstrated numerical error in the saved value 7.0350618718604325. Even symmetric positive matrices K and G do not generally make K^-1 G Euclidean-symmetric: its transpose is G K^-1. A neutral two-by-two algebra counterexample was recorded under `target/paper-a4-eigen-contract-probe.json`. It contains no CAD model and does not test the archived A4 eigenvalue. An appropriate generalized/self-adjoint formulation could be justified, but that justification is not established by the inspected call or report.

**Disposition:** preserve the original scalar and favorable legacy flag; label them as archived observations with this open audit. Do not call the A4 eigensolve numerically verified or fabricate a buckling mode. The manuscript and figure now say so. Resolution requires the numerical owner's independent assessment of the realized operator/inner product and retained eigenpair residual or an appropriately scoped new verification receipt. No scientific implementation, test, requirement or original evidence is modified by this publication task.

The A4 ratio comparison also changes the modeled case and regional masks: supported lateral-only versus the certification minimum across cases. The publication therefore reports both values but does not attribute their difference solely to compliant supports.

## Related-work and editorial follow-up

The 19 October revision adds primary references for learned CAD-operation sequences, text-conditioned CAD generation, earlier evolutionary hardware and DOLFINx's software architecture. The [source-review register](REFERENCE_REVIEW.md) records exact identities, claim limits and actual review depth. This addresses part of RT-12, not a complete full-text literature audit. The comparison table distinguishes evaluation domains; none of the cited studies becomes a shared benchmark or evidence of agent advantage in Jarambit. The version-specific 2023 DOLFINx preprint is separated from the archived 0.9.0 execution environment.

RT-11's author-facing framing is reduced in the introduction and discussion. The wording now uses *intelligent computational design*, and future experiments are presented as research questions rather than submission advice. The numerical observations and their local limitations remain unchanged. Bibliographic numbering follows first citation; unused venue-guidance bibliography material is removed while the public build README retains the venue link. Independent review, remaining visual coverage, public artifact delivery and the A4 audit stay open.

## Historical battery visualization and provenance

The source-bound matrix now displays every one of the 414 stored cells, retaining categorical P/F/B/I labels rather than depicting them as stress or physical reliability. All 18 battery explanations link to their exact-ID column and a cell-level source/selector ledger. Original record kinds, nullable solver/geometry/evidence fields and typed false freshness flags remain intact. The four build columns repeat 92 pass references, while `solid.tip` passes retain their record-completeness meaning.

The manifest's source revision is not uniform across member records: 21 subjects cite `6d18ef8cbe7b995890451a8ac7b93b75c928d150`, and two w283 subjects cite `4e54019dd016ddde71b416460b96e0a9b08a95de`. The matrix labels both rather than silently replacing member fields with the manifest tag. These are retained provenance fields, not independent proof of an execution or a homogeneous model regime. Seven explicit null evidence-path fields stay null; no missing receipt is invented. The figure contract freezes all source bytes and refuses count/membership drift.

Current cross-references and explicit missing entries are enumerated in the generated visual coverage ledger. The combined response group links to its shared retained field family and the article identifies its separate original flag. These figures advance RT-04 but do not supply missing FEM spatial fields or complete the entire visual brief. The 414 cells, symbols, colors, source selectors and raster samples were checked independently of the Rust rendering code; this is internal author QA, not independent scientific reconstruction or required bot approval.

## Recorded lineage: bounded reconstruction

The w282 lineage figure retains five birth records and five original parent-list edges around ID 1095, the highest stored historical score. It preserves reuse of ID 413 and explicitly lists the two omitted parent links at the two-hop cutoff. Both omitted references have retained source births; the view does not depict full ancestry, infer inherited geometry or bind those rows to the separate cover export. Node placement has no time/fitness axis meaning. Search legality and completed-FEM labels coexist with the focal row's later failed strict assessment.

The reader refuses changed focal membership, duplicate/untyped parent IDs, missing in-window parents and contradictory generation order. An unavailable reference outside the displayed window stays an unavailable reference; a neutral control confirms that no source node is invented. Source pointers, seven fields per displayed node, all five parent-list positions, both omitted links and the original assessment join were separately checked against the saved JSON. Author checks do not establish causality, original-solver replay or independent scientific reproduction.

Editorial review also found stale wording that called the studied scorer implementation “current” after the subsequent Stage-2 correction. It now states the studied baseline and preserves the retained cohorts' original score history. Repeated generic reproduction guidance was shortened to keep the actual lineage figure within the 20-page artifact.

## Visual coverage: links versus spatial evidence

The generated [coverage table](../tests/visuals.md) and [typed ledger](../tests/visuals.v1.json) enumerate all 44 catalog identities. Current link and missing-entry counts are computed by the generator. Six entries reuse the retained four-panel plate-field asset; other image kinds include saved scalar/profile figures, topology-record diagrams, support schematics and the historical categorical matrix. These counts establish neither unique experiments nor complete heatmaps or main-text figure proximity. Full strict-plate, solved-solid, buckling-mode and fatigue fields remain `archive_required` in the admitted inputs.

The generator uses one shared panel mapping for the atlas and coverage ledger. It checks each link within its own subsection, catalog membership, actual matrix column identity, figure/manifest hashes, projection schema and assessed subject/certificate binding. It refuses fresh-execution claims for the archived scalar/status assets and a full-solid-field claim in the profile receipt. Rejection controls exercise a single removed subsection link, duplicate/unknown identity, missing asset, altered SVG bytes, wrong subject/certificate and false freshness/spatial availability. These are internal publication checks; they do not create new numerical evidence, and RT-04 remains incomplete.

## Original export-record semantics

The source-bound G16-01 diagram preserves twelve observations and five content-bound artifacts for the separate assessed example. Its original evaluator at `e8cb049df7d03c7d3113556812cc140ca99a29c3` has SHA-256 `e5478dba651bfaf327c44746b2388e786a8609916fecafdd51c836ecf4c81945`, verified by reading that Git object. It counts raw STEP text occurrences and parses the exporter stdout edge census. The later evaluator differs: its STEP parser strips comments/strings, restricts counts to the data section and has additional unit/regeneration checks. The archived fields must not be reinterpreted as results from those newer checks.

In particular, `cad.facets=55698` is the stored STEP face-token count; the cover scene's 90,000 imported triangles count a different representation. This difference is not proof of a geometry mismatch. The original OpenCascade result reports one valid solid; its Boolean is preserved, not rerun. The genome, STEP and STL digests establish retained content associations; dashed diagram links do not claim current deterministic regeneration. The original favorable composite CAD flag coexists with failed overall qualification. No shell/B-rep equivalence, fresh CAD validation, FEM field, human fit or manufacturing precision is inferred.

The topology diagram closes one catalog visual gap, with its own visual kind, rather than replacing the unavailable mechanics fields. Internal controls retain explicitly unfavorable flags and reject wrong subject/content bindings, untyped counts or altered render-mesh metadata. A complete current export/physical validation remains outside this publication projection.

## Concurrent README PR #137: source-binding follow-up

**Read-only spot check, 19 October 2026, exact head `d810c79b4ac00a3189ab6e5e55bbc52f9126e18b`; not an approval or complete PR review.** [PR #137](https://github.com/neverhuman/jarambit/pull/137) changes only `README.md` and `ops/market/report.py`. Preserve that lane's ownership and do not overwrite its generated market block during publication integration.

**P1 — New leader claims lack committed assessment bindings in the inspected tree.** [Report lines 93–104](https://github.com/neverhuman/jarambit/blob/d810c79b4ac00a3189ab6e5e55bbc52f9126e18b/ops/market/report.py#L93) hard-code the newly named `ef2i01_r02_p14_c110`, `ef2i04_r02_p14_c109` and `ef3i09_r01_p10_c41` observations and the overall/lightest Stage-2 leader wording. The PR adds no evidence files or source selectors for those records; no matching retained assessment was found in the inspected committed evidence corpus. In contrast, the w411 row is read from a retained JSON. This is an evidence-binding gap, not proof that the new numbers are false. A publisher cannot reconstruct their candidate identities, source/stage/scorer revisions or the membership behind the leader claim from this patch. Before integrating those claims, retain the original assessment/cohort receipts with hashes and exact selectors, or label their scope and unavailable bindings explicitly. Do not join them to the paper's frozen cohorts merely by a score name.

**P2 — The w411 comparison inherits another candidate's statistics.** The template reads the w411 label, mass and score from its saved record, but places it under percentages/counts computed by `cs(101.172)` for the other candidate. A later source-record change can therefore update the displayed w411 observation while its comparison remains tied to the unrelated constant. This is a dependency defect visible by inspection, not a claim that today's rounded percentages differ. Each printed comparison needs its own bound source observation and reconciliation control, or separate presentation with narrower wording.

The exact remote report source is retained locally at `target/paper-pr137-market-report-d810c79b.txt`, SHA-256 `e9cb77249542ed77ed928664e86c674f3c1c3f28e542d0e79edcaf1db7143f66`. No PR code was executed, no market generator or concurrent README was edited, and no live observation was invented or imported into the manuscript. This finding is recorded for integration review; it does not replace the required exact-head bot review.

The fresh 11:20 UTC check finds `neverhumanbot` CHANGES_REQUESTED on this same head, submitted at 11:16:33. The reviewer independently raises the source-binding gap and additionally requests explicit not-qualified disclosure and correction of the hand-built figure headline. Its reported live check of one xbabe3 value does not supply a committed reconstruction input here; this task did not inspect that live run. Required approval remains outstanding.

## Six retained weighted-summary files: bounded expansion

The main results now show all fifteen generation rows from six frozen reporting-summary files, rather than only the selected w411c3 trajectory with a repeated historical inset. The selected component profile remains in the supplement. Each run stays in its own facet; its source-directory label is not proof of an independent experimental replicate. None of the fifteen named round files is present next to these admitted summaries, so no birth distribution, missing generation or uncertainty band is reconstructed.

Thirty score markers retain their original row/field selectors: fifteen stored archive values and fifteen round bests, including coincident values. The one-row w501_xb1 file has exactly one observation pair and no connecting line. Four of the five multi-row files retain a constant archive over their saved rows; a single row is not counted as evidence of a plateau. The shared 70–75 display is explicitly truncated, and the common metric label does not establish comparable stage/scorer inputs. Revision and point-stage fields remain not established; recursive source inspection found no commit/revision fields in these admitted files.

The reader checks all six member identities/hashes, the fifteen-row total, generation order and the original Hall-of-Fame recurrence, plus final score/label association. It rejects omitted/duplicate runs, an asserted homogeneous regime, invented scorer revision and a display that would clip a saved value. All thirty marker values, selectors, coordinates and raster color samples were separately reconciled to source. These are internal reporting checks, not independent reconstruction of the original search, qualification or a compute-scaling experiment. RT-02's underlying stage-lineage gap remains open. The generated visual-coverage ledger records the current assessment-visual gaps; adding a reporting-summary plot does not close a missing assessment visual.

## PR #137 revised head: preliminary disposition

At 11:49 UTC, read-only inspection of new head `d78ab8baecd9f2606e5868040a9f558eae00a07b` finds the unbound `ef` leader literals removed. The market template now reads existing w282, w411 and g3 records, computes the w411 comparison using its own loaded source mass, adds explicit qualification disclosure, and removes the market block's leading hand-built image/histogram links. These changes address the earlier P1 source-binding and P2 cross-candidate dependency findings by inspection; those findings must not be repeated as unchanged defects of this new head.

The revised report source is retained at `target/paper-pr137-market-report-d78ab8ba.txt`, SHA-256 `b21e16fa3473e5fb8c9d1b1a99129fac38bbc466835b3bb10a0bafb513e0bba5`. It was parsed and inspected, not executed. This is a preliminary source/diff disposition, not complete validation of the revised README or exact-head approval. The PR remains open, with new-head CI queued/running and no new-head approval observed in this check. Its work and ownership remain separate from this publication packet.


## Integration recheck, 19 October 2026, 12:31 UTC

PR #137 is now **merged**, rather than still awaiting approval. The required D1 job `118041878688` succeeded at 12:19:14 UTC; `neverhumanbot` approved exact head `d78ab8baecd9f2606e5868040a9f558eae00a07b` at 12:20:48, and the PR merged at 12:21:59 as `1ea4d803207d066d2cdc36254173d8a3905848b4`. The previously recorded source-binding, qualification-disclosure and cross-candidate dependency findings are resolved in that reviewed revision. Earlier open/changes-requested statements above are dated snapshots. The reviewer leaves a non-blocking issue in the unlinked historical market figures, which still headline the hand-built reference; those figures were not regenerated by this publication task.

The publication branch has rebased onto that main without conflict. The approved market block is retained byte-for-byte, as are the root README outside that block, COMMS history and the two untracked planning inputs. Scientific/evidence/CAD/scorer paths match upstream main. The publication's historical source baseline and cohort identities remain unchanged; a newer integration base does not retrospectively change their producer revisions. A fresh full local gate is running; no final gate success, new publication approval or merge is inferred from #137's checks.
