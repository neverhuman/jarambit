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
| Winner geometry substituted | Search retained weighted cohort for best candidate geometry | Missing. Opening figure displays actual score profile, with absence disclosed. |
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

## Residual limitations

- Retrospective single-repository evidence does not establish a new state-of-the-art optimization result.
- No controlled agent-representation experiment, independent reconstruction study or manufacturing-tolerance experiment is supplied.
- Weighted reporting placeholders and native/reporting score differences remain scientific-source limitations; this publication change does not repair or tune them.
- Historical cell semantics and source revisions remain heterogeneous. The explanatory projection preserves them rather than reissuing scientific evidence.
- Publication tooling checks retained data and layout. It does not verify every original solver execution or guarantee physical correctness.
- Independent exact-head bot approval and required PR CI remain merge conditions. This author review cannot satisfy that approval.

## Local broad-gate finding

The release workspace run encountered the unchanged D1 fixture regression `tests::d1_knife_fixture_reports_torsion_and_couple`; an isolated exact-test rerun reproduced its existing assertion failure. Scientific source and dependency versions match baseline. This task preserves the test and requirement definitions and does not claim the workspace gate passed. Required PR CI and independent review must resolve merge readiness.
