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

Visual cross-references now cover 36 of 44 atlas entries. The remaining eight are four G16 geometry/reference entries, RS-v2 geometry/appearance, the combined response group, and two exported edge/point measurement entries. This categorical figure advances RT-04 but does not supply missing FEM spatial fields or complete the entire visual brief. The 414 cells, symbols, colors, source selectors and raster samples were checked independently of the Rust rendering code; this is internal author QA, not independent scientific reconstruction or required bot approval.
