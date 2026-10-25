# Paper source map (one file per section)

Both papers (A: `main.tex`, B: `main-smoothed.tex`, which sets `\JRSmoothedVariant` and inputs `main.tex`) and the
supplement (`supplement-main.tex`) are assembled from one file per section, so several people or agents can edit at
the same time without touching the same file. The order below is the reading order.

**Editing protocol for parallel work**
- Claim the file(s) before editing (in `~/jarambit/comms.md` or the PR thread) and release them when done; one writer per file.
- Numbers come from generated macros (`generated/*.tex`, `allruns/allruns-meta.tex`, `ladder/ladder-meta.tex`); never type a result.
- Labels are global across both papers; cross-document references use `S-` (main → supplement) and `M-` (supplement → main).
- `make -C docs/paper check pdf-smoothed` must pass: both papers 15–20 pages, no overfull boxes or undefined references.
- Style lives in `ei-ieee-skin.tex`; change it only with a measured page count for both papers.

## Main papers (A and B)

| # | File | Section | Labels |
|---|---|---|---|
| 0 | `sections/00-frontmatter.tex` | (title page, cover, key numbers) | `fig:winner-cover` |
| 0a | `sections/00-abstract.tex` | (abstract; input inside the page-1 layout) |  |
| 1 | `sections/01-introduction.tex` | Introduction | `sec:introduction`, `subsec:research-question-and-results`, `subsec:contributions` |
| 2 | `sections/02-background.tex` | Background and Related Research | `sec:background`, `tab:related` |
| 3 | `sections/study-design.tex` | Study Design and Retained Evidence | `sec:study-design`, `tab:study-coverage`, `fig:evidence-architecture` |
| 4 | `sections/03-genome.tex` | Representation and Candidate Identity | `sec:genome`, `tab:genome-blocks` |
| 5 | `sections/observed-morphology.tex` | Observed Morphology and Feature Provenance | `sec:observed-morphology`, `fig:cover`, `tab:winner-requirements` |
| 6 | `sections/pocket-smoothing.tex` | Surface-Tension Pocket Smoothing *(Paper B only)* | `sec:pocket-smoothing`, `eq:pocket-flow`, `fig:pocket-bw`, `fig:pocket-fields`, `tab:pocket-score` |
| 7 | `sections/04-evolution.tex` | Recorded Search and Inheritance | `sec:evolution`, `fig:lineage` |
| 8 | `sections/05-fitness.tex` | Score Definitions and Comparability | `sec:fitness`, `tab:score-dictionary`, `fig:score-decomposition`, `fig:plate-fields` |
| 9 | `sections/08-results.tex` | Findings from the Retained Records | `sec:results`, `fig:native-dynamics`, `fig:all-runs`, `tab:candidate-evidence` |
| 10 | `sections/06-battery.tex` | What the Numerical Observations Represent | `sec:battery`, `tab:observation-families`, `subsec:family-lateral`, `subsec:family-ring`, `subsec:family-torsion`, `subsec:family-solid` … |
| 11 | `sections/07-verification.tex` | Numerical Agreement and Unresolved Error | `sec:verification`, `fig:verification` |
| 12 | `sections/10-reproducibility.tex` | Reconstruction and Artifact Access | `sec:reproducibility`, `tab:reproduction-levels` |
| 13 | `sections/09-agents.tex` | Prospective Representation Research | `sec:agents` |
| 14 | `sections/11-discussion.tex` | Discussion | `sec:discussion` |
| 15 | `sections/12-conclusion.tex` | Conclusion | `sec:conclusion`, `tab:prospective` |
| 16 | `sections/13-data-availability.tex` | Data and Code Availability |  |
| 17 | `sections/14-acknowledgment.tex` | Acknowledgment and Assistance Disclosure |  |

## Supplement

| # | File | Section | Labels |
|---|---|---|---|
| 0 | `supplement/representation-search.tex` | Historical Representation and Search Details | `supp:representation-search`, `supp:inheritance-details` |
| 1 | `supplement/scores-and-composites.tex` | Scores, Candidate Roles and Display Composites | `sec:score-composites-details`, `tab:weighted-vector-details` |
| 2 | `supplement/display-and-score-views.tex` | Display Support and Additional Score Views | `sec:display-score-views`, `tab:display-roster-families`, `fig:hero-quantitative`, `fig:display-support`, `fig:display-contrast` … |
| 3 | `supplement/numerical-methods.tex` | Numerical Methods, Source Bindings and Observation Limits | `sec:numerical-methods-details`, `tab:solid-implementation` |
| 4 | `supplement/cohort-inventory.tex` | Cohort Inventory, Overlap and Descriptive Summaries | `sec:cohort-inventory-details`, `tab:cohort3-runs` |
| 5 | `supplement/related-work-notes.tex` | Related-Work Interpretation Notes | `supp:related-work-notes` |
| 6 | `supplement/moved-from-main.tex` | Material Moved from the Main Paper | `sec:moved-from-main`, `sec:moved-display`, `fig:population-latest`, `fig:assessment`, `fig:objective-priorities` … |
| 7 | `supplement/historical-battery.tex` | Complete Historical 23-Subject, 18-Evaluator CAD Battery | `sup:battery-matrix`, `fig:battery-matrix` |
| 8 | `supplement/catalog-crosswalk.tex` | Complete 44-Entry Catalog Crosswalk (\code{G16..RSV2}) | `sup:crosswalk-44` |
| 9 | `supplement/missing-visuals.tex` | Missing Subsection Visuals and Unavailable Spatial Fields | `sup:missing-disclosures` |
