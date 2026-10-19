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

## Residual limitations

- Retrospective single-repository evidence does not establish a new state-of-the-art optimization result.
- No controlled agent-representation experiment, independent reconstruction study or manufacturing-tolerance experiment is supplied.
- Weighted reporting placeholders and native/reporting score differences remain scientific-source limitations; this publication change does not repair or tune them.
- Historical cell semantics and source revisions remain heterogeneous. The explanatory projection preserves them rather than reissuing scientific evidence.
- Publication tooling checks retained data and layout. It does not verify every original solver execution or guarantee physical correctness.
- Independent exact-head bot approval and required PR CI remain merge conditions. This author review cannot satisfy that approval.
- The local PDF renders in Poppler, but pypdf reports repeated `/Group` dictionary keys at embedded SVG-PDF objects. PDF structural cleanup/toolchain portability remain publication follow-ups; `qpdf` is unavailable locally. The new heatmap uses a verified 3,600-pixel raster projection to avoid PDF triangle-edge seams; the source SVG and cell associations remain available.

## Local broad-gate finding

The 19 October 2026 Rust 1.85.1 release workspace run on `08ed64b1dd31` again failed `tests::d1_knife_fixture_reports_torsion_and_couple`; an isolated exact-test rerun reproduced the assertion at `crates/crescent/src/tests.rs:206`. This scientific source, its mechanics dependencies and the root lock match main `72e25a360470` byte-for-byte. This task preserves the test and requirement definitions and does not claim the workspace gate passed. Later local gate phases remain incomplete. Required PR CI and independent review must resolve merge readiness.
