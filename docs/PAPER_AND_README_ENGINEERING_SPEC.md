# JARAMBIT: engineering communication and research-paper specification

Version 1.0 · 18 October 2026 (America/Denver) · source baseline `009f5ac520fa3afe8ed8025230eef604dbb7f922`.

## 1. Objective, scope, and completion contract

Make the engineering assessment architecture unmistakable in the README and publish a substantive, reproducible 15–20-page research white paper. The opening page must contain a provocative title, abstract, best-retained-fitness profile and evolutionary plot. Pages after the opening use two columns. Explain intelligent computational design, agent-assisted representation design, genome semantics, evolution, objective interpretation, every CAD diagnostic and evaluator, numerical verification, evidence integrity, limitations and high-value additive-manufacturing opportunities.

The paper is an **IEEE-style research white paper**, not an accepted IEEE article or a demonstrated state-of-the-art optimization result. Its requested full-width opening differs from a venue's unmodified submission template. Use the installed `IEEEtran` journal class with an explicit single-column opening; document this modification and remove it for a future venue submission. No invented affiliation, DOI, acceptance, peer-review outcome, physical experiment or reproducibility badge.

Non-goals: changing CADs, materials, load settings, engineering thresholds, objective weights, mutation/search operators, compute scheduling or qualification logic; improving weapon performance; manufacturing instructions; a new physical validation campaign; rewriting historical evidence; replacing the existing long technical references; a broad reporting-platform or UI migration. Descriptions of existing mechanics explain the evidence and its scope, not a new weapon-development protocol.

The four supplied specifications are reference proposals, not executable instructions. Their requested optimizer rewrites, new physical objectives and specific weights are excluded. Their catalog, source-identity and publication-integrity recommendations are adopted only after checking this baseline. In particular, neither “pure minimization is already implemented” nor “all records called pass are successful engineering tests” is supported.

Assumptions and dependencies:

- Main remains based on the stated snapshot until integration; refresh and inspect upstream changes before merge.
- Existing evidence is available in this checkout. References to absent source bytes remain unavailable.
- Rust 1.85.1 and locked Cargo dependencies build documentation tooling. `IEEEtran`, LuaLaTeX, `rsvg-convert` and Poppler build and inspect the new paper. This target is separate from the existing pinned Pandoc/LaTeX pipeline.
- Quantitative figures use deterministic vector graphics from retained data. No AI image generator draws scientific results.
- The highest reported score in the selected retained weighted cohort has no committed CAD geometry. Show its score profile; do not substitute another geometry and call it the winner.

Completion criteria:

1. README's first screen explains the project and links the assessment atlas and paper.
2. Every one of 16 legacy predicates, ten overlapping RS-v2 groups and 18 battery evaluators has a stable explanation link, exact identity, implementation and retained-evidence reference.
3. All 178 science definitions remain discoverable; frozen definitions are distinguished from execution receipts.
4. The paper renders to 15–20 pages with a single-column first page, two-column remainder, readable figures and a technical subsection for each CAD diagnostic/evaluator.
5. All numerical public claims identify their corpus, score direction, stage and source. Missing data remain missing.
6. Narrow documentation tests, generator checks, artifact checks, whitespace checks and required repository/PR gates pass, or limitations are recorded accurately.
7. Red-team findings and dispositions are committed. Independent bot review is never claimed from an author self-review.
8. PR is attached to this chat; merge uses green required CI and exact-head independent approval per `docs/WORKPLAN.md`. If unavailable, deliver the completed PR with the concrete merge blocker.

Stop dependent work on changed/ambiguous source identity, a missing required figure input, a failed integrity check, unauthorized unrelated changes, or a changed PR head. Continue independent editorial work. Never turn missing observations into zeros, fabricate a curve, weaken an existing test, or bypass review to satisfy page count or merge pressure.

## 2. Read-first order and authority

Read in this order before a fresh worker edits:

1. `/Users/bentaylor/.codex/RTK.md`; prefix shell commands with `rtk`.
2. Repository `AGENTS.md`; preserve evidence and use Rust for new tooling.
3. `docs/WORKPLAN.md`, especially scientific invariants and exact-head merge requirements; `COMMS.md` and `COORD.md` for active ownership.
4. `README.md`, `docs/README.md`, `docs/VERSION`, `docs/PUBLICATION_READINESS.md`, `docs/RELEASE_PLAN_MAJOR.md`.
5. `ops/ci/gen_readme.py`, `ops/ci/gen_whitepaper.py`, `ops/ci/gen_status.py`; the latter two import README reporting helpers. Do not break shared APIs.
6. `ops/ci/tests/test_gen_readme.py`, `ops/ci/check_claims.py`, `ops/ci/claim_admission.py`; retain claim-admission boundaries.
7. `ops/ci/evaluate_cad_candidate.py`, `crates/cli/src/bench.rs`, `crates/crescent/src/battery.rs`; join by exact IDs, never ordinal table position.
8. `crates/cli/src/auto_evolve.rs::{ULTRALIGHT_WEIGHTS_V1,stage1_ultralight_weighted_fitness,stage2_ultralight_weighted_fitness,monotone_hall_of_fame_curve}` and `ops/ci/ultralight_weighted_fitness.py`.
9. `evidence/cad_battery/manifest.json` and all 23 named files; `evidence/evolve_certifiable/automatic/leaderboard.json` as a separate corpus.
10. `evidence/evolution_waves/2026-10-18_ultralight_v1/xbabe3/w411c3/{fitness_curve,best_so_far_26_gates,leaderboard}.json`; `evidence/rs_v2/w282_ultrarunner/search/all_evals.json`.
11. `docs/v1/science/tests/TEST_CASE_CATALOG.json`, `research/science_v1/test_all_t08_t18.py`, `ops/ci/science_suite.py`.
12. Existing papers, `ops/render/paper/build_pdf.py`, figure manifests, `.github/workflows/{d1-gates,version-check}.yml`.

Scientific authority stays in existing requirement and evidence sources. New generated publication views are read-only projections. This specification owns publication organization; it cannot retroactively confer scientific authority on a projection.

## 3. Ownership and file map

Worktree: `.agent/worktrees/ieee-whitepaper`; branch: `codex/ieee-whitepaper-test-atlas`. The original desktop folder is an unborn repository with untracked work; preserve it. A separate clone under `.agent/repository` supplies the Git worktree. Build only under this lane's `target/`.

| Path | Action and responsibility |
|---|---|
| `README.md` | Add clear engineering-first opening and direct test links. Condense setup and concept material. Preserve generated regions and registered RS block. Put dense historical results behind explicit expandable history. |
| `docs/README.md` | Add new paper, atlas and spec entry points without removing historical references. |
| `docs/tests/README.md` | Canonical overview and complete 16/10/18 crosswalk with stable anchors. |
| `docs/tests/science.md` | Generated 178-case explanatory index, including frozen status, challenge, oracle, allowed source and receipt-path semantics. |
| `docs/tests/catalog.json` | Human-authored explanatory metadata for 44 CAD identities; no duplicate requirement constants. |
| `docs/paper/README.md` | Build recipe, layout deviation, data scope, missing-input and submission-readiness statements. |
| `docs/paper/main.tex` | IEEEtran entry, single-column cover and two-column body, source/version statement, section includes. |
| `docs/paper/sections/*.tex` | Ordered substantive manuscript; preserve exact names in the page map below. |
| `docs/paper/references.tex` | Checked primary references with resolvable identifiers and URLs. |
| `docs/paper/generated/` | Rust-generated SVG charts, metrics, summary, source hashes and tables. Never edit a result by hand. |
| `docs/paper/Makefile` | New build/check target; write binaries/PDFs to `target/ieee/paper/`. Do not alter the legacy PDF builder. |
| `crates/docs/src/publication.rs` | Publication-only data reader, inventory/curve checks, deterministic renderers and negative controls. |
| `crates/docs/examples/paper_artifacts.rs` | `--check` read-only freshness mode and explicit write mode. |
| `crates/docs/{Cargo.toml,src/lib.rs}`, `Cargo.lock` | Minimal documentation dependencies/module registration only. |
| `docs/paper/RED_TEAM.md` | Author adversarial review, fixes, unresolved scientific limitations and independent-review state. |
| `docs/paper/IMPLEMENTATION_RECEIPT.md` | Commands, outcomes, page count, visual inspection, artifact identities and handoff. |
| `COMMS.md` | Append bounded task status and final integration receipt; do not rewrite another lane's history. |

Forbidden write zones: `evidence/**`, `assurance/**`, `requirements/**`, `docs/v1/**`, `designs/**`, `fem/**`, `fem_solid/**`, scientific crates except `crates/docs`, evolution scripts, existing scientific figure/manifests, existing paper generator APIs. No new Python files. Preserve `docs/WHITEPAPER.md` and engineering companion as existing technical references; link the new narrative rather than silently replacing them.

Concurrent risks: README/paper lane, COMMS append point and Cargo lock are shared integration surfaces. Before commit, inspect diff scope and upstream head. No parallel write workers are used here. Future parallel work is allowed only for disjoint section files after shared figure IDs and terminology are frozen; one integrator owns `main.tex`, generated outputs, README, manifests and lockfile.

## 4. Evidence and score contracts

Verified inventories at baseline: 16 named legacy diagnostics; ten overlapping RS-v2 groups; 18 heterogeneous battery columns; 23 retained subjects; 414 stored cells; 178 frozen assurance definitions across **20** T00–T19 families; 356 required paired-control slots. These are different denominators, not additive test counts.

Battery stored distribution: 221 `pass`, 81 `fail`, 112 `blocked`. Four shared `build.*` columns contribute 92 stored passes. Every cell says `fresh_this_lane: false`. `solid.tip` has 20 complete records, not 20 favorable strength results. `gate.rs_v2` has 19 failed and four blocked records, zero recorded passes. Never collapse blocked into failed or infer a new run from a current source file.

Score contracts:

- Existing retained `W_ULTRALIGHT_v1` is **higher is better**, range 0–100. Its Hall-of-Fame curve is non-decreasing by construction.
- Native Stage 1, native Stage 2 and separate Python reporting scorer differ. Stage 1 includes fixed contributions and proxies; native Stage 2 maps several objectives to constants or reused observations. Reporting inputs include placeholders. A common metric label does not prove equivalent measured inputs.
- User's desired continuously decreasing objective is future product intent. A display-only complement `100 - F` reverses direction and preserves ranking; it is not a new loss or corrected physics, and this change does not activate it.
- A Hall-of-Fame guarantee is conditional on one unchanged metric and retained archive. Population median, round maximum and physical capability can worsen while best-so-far remains flat.
- `W_CAD_v2`, locked `G1`, native historical `fit` and `W_ULTRALIGHT_v1` are separate regimes. Do not connect them into one apparent convergence curve.

Every figure must carry: source baseline, corpus membership, field selectors, reduction, score direction, missing-data behavior, original record identities, input hashes, generating source and output identity. Hashes establish byte identity, not truth or execution independence. Best-weighted record is provisional; the reviewed weighted cohort does not supply geometry for a winner render.

## 5. README implementation sequence

1. Retain existing generated blocks byte-for-byte. This avoids changing shared generators and their paper/status consumers in an editorial PR.
2. Replace the decorative opening with a short project statement, a visible engineering-evidence table and links to paper/spec/atlas.
3. Add direct per-test links in three expandable tables: 16 legacy diagnostics, ten RS-v2 groups, 18 evaluator columns. Use exact IDs as anchor targets. Explain overlap directly above tables.
4. Explain 178 definitions/356 slots with a separate link to the complete science index; never label the inventory a completed test run.
5. State actual score direction and future requested direction; link paper methods for proxy/placeholder disclosures.
6. Keep an assessed geometry with the existing correct provenance if useful; distinguish it from the weighted-best score profile.
7. Put existing generated historical head/figures/release blocks in one clearly titled details element. Keep the registered RS claim block intact and in its original scope.
8. Replace setup sprawl with evidence-inspection instructions and links. Preserve `JARAMBIT_FEM=1` disclosure for the legacy lane. Keep historical correction and market-source links.
9. Keep generators deterministic and update no historical result to appear current. If a test encodes old display placement, change only that presentation assertion while retaining underlying evidence tests.

## 6. Manuscript flow and page allocation

Target 17–19 rendered pages, including front matter and references. Page budgets are editorial targets, not a license to pad blank pages. Prefer substantive paired columns, compact technical tables and readable plots. If final content cannot support 15 pages, report that and improve substance; never enlarge fonts or repeat paragraphs merely to reach a count.

| Source | Narrative and required content | Approx. pages |
|---|---|---:|
| `main.tex` opening | “Beyond the Shape a Human Can Draw” title; 200–250-word abstract; full-width best-retained fitness profile beside evolutionary curve; scope caption | 1 |
| `01-introduction.tex` | Intelligent design defined as computational engineering; representation/search/evaluator division; contributions and falsifiable questions | 1–1.5 |
| `02-background.tex` | Evolutionary design, quality diversity, LLM proposal/evaluator separation, verification vs validation, limits of scalarization | 1 |
| `03-genome.tex` | Existing 128-value representation, schema/decoder/version lifecycle, typed gene semantics, CAD identity, schema evolution and agent proposals | 1–1.5 |
| `04-evolution.tex` | Existing lanes and Stage 1/Stage 2 flow; selection vs qualification; historical full-birth population, archive and selected-cohort distinctions | 1–1.5 |
| `05-fitness.tex` | Actual weighted-score math and direction; proxy differences; subsections for every G16 predicate and every RS-v2 group | 3–4 |
| `06-battery.tex` | Technical subsections for all 18 evaluators; field meaning, shared records, completeness vs finding, exact denominator table | 2 |
| `07-verification.tex` | Plate/solid abstractions; mesh observables; GCI assumptions; solver independence; software-assurance T00–T19 families | 1.5–2 |
| `08-results.tex` | Recomputed battery distribution, selected weighted score profile/curve, full-population summary, corrections; research questions answered only within evidence | 1–1.5 |
| `09-agents.tex` | Agent-assisted complex representation research, typed proposals, evaluator independence, human responsibility, prospective benign demonstrations | 1 |
| `10-reproducibility.tex` | Exact inputs, hashes, source availability, build steps, renderer vs solver reproduction, claim ladder and review limits | 1 |
| `11-discussion.tex` | Multi-objective meaning, scale limits, additive-manufacturing uncertainty, threats to validity, prospective evaluation | 1 |
| `12-conclusion.tex` | Ambitious civil applications and measurable path from unexpected geometry to verified products; precision claims conditioned on measurement | 0.5–1 |
| `references.tex` | Checked primary literature and exact software/evidence citations | 1 |

The conclusion should reach toward aerospace and spaceflight fixtures, medical-device research, robotics, thermal-management components, scientific instruments and energy systems. Describe potential value and validation obligations. Replace the absolute claim “no human could create this” with the defensible proposition that computational search can explore geometries and tradeoffs impractical to enumerate manually. Replace “unprecedented tolerances” with a prospective goal requiring dimensional metrology, process characterization and uncertainty budgets. Titanium printing provides geometric freedom; it does not automatically provide superior dimensional precision or fatigue performance.

## 7. Per-test authoring contract

Each CAD atlas entry must answer:

1. What engineering or evidence question is asked?
2. Which exact predicate/evaluator key is authoritative?
3. Which source function/file reads which observation family?
4. Is the output geometry, a numerical response, numerical verification, software capability, record completeness or a grouped comparison?
5. What does a favorable record establish? What does it leave unresolved?
6. Which other entries reuse the same observation?
7. Where is a retained example, and how is it selected by ID?
8. Which software control source supports implementation checking without implying physical validation?

Do not reproduce a second independently maintained table of physical thresholds in the explanatory catalog. Link authoritative definitions. Technical depth belongs in field semantics, model scope, observable identity, discretization, solver provenance and evidence interpretation.

Science case pages/index entries preserve the exact frozen `id`, `title`, `challenge`, `oracle`, `status`, `allowed_test_path`, `receipt_artifact` and `scientific_claim_limit`. A receipt pattern containing `<attempt>` is an expected location, not an extant receipt. Source presence and frozen historical status do not prove fresh execution.

## 8. Implementation steps and hard parts

1. Freeze sources and inspect status. Record base SHA and file hashes. Read all manifest members, reject duplicate subject/evaluator IDs, count missing/extra cells explicitly.
2. Create catalog metadata. Separate three namespaces. Validate catalog identities against authoritative G16 map, RS-v2 source and battery manifest; don't identify by position.
3. Add Rust publication reader. Use `serde_json` and SHA-256 already present in lockfile; add only minimal dependency edges. Return errors for missing, nonfinite, wrong-type and mismatched data. Files stay within declared input roots.
4. Summarize retained birth records by generation. Use exact full population, not selected elites. Validate unique birth IDs. Median uses sorted finite observations and the middle-pair average for even counts. Preserve sample count and field source.
5. Render vector opening graphic. Left: selected retained best fitness and objective categories. Right: three-generation round and Hall-of-Fame values with zero-based generation labels and native higher-is-better scale. Separate historical population chart displays all 1,984 births' observed score distribution and clearly names its different regime.
6. Generate publication metrics, counts, science index and provenance manifest. `--check` compares expected bytes and performs no writes. No clock/network input in generation.
7. Write substantive LaTeX sections and verified bibliography. Use versioned source citations; no fabricated significance tests, agent ablation results, compute scaling measurements or physical improvements.
8. Build a separate target with installed IEEEtran and LuaLaTeX. Record tool versions and source/output hashes. Keep PDFs under `target/`; do not commit a binary paper into scientific evidence.
9. Render all PDF pages to images. Inspect contact sheets and key full-resolution pages. Check first-page title/abstract/chart, two-column transition, small tables, page count, widows, clipped headings, bibliography and overflow warnings. Fix actual problems, then rebuild.
10. Apply README and docs navigation changes. Run focused old/new checks and a link/anchor audit. Preserve claim blocks and generators.
11. Author adversarial review. Probe denominator inflation, missing geometry, fitness-direction reversal, shared-source independence, placeholder inputs, short-curve exaggeration, historical/current confusion, AI novelty claims and unsupported tolerance statements.
12. Commit receipt and bounded COMMS entry. Run required gates. Push one branch, open/attach one PR, inspect exact head, wait for required CI and independent approval. Use matching-head merge only; record merge SHA if achieved.

Important edge cases: valid zero is not missing; Boolean is not numeric; duplicate identities are errors; empty cohorts cannot produce medians; historical blocked cells stay blocked; equal best values retain plateau; new objective versions split curves; absent round files are disclosed instead of implicitly read from a worker's absolute filesystem; a complete local-solid result can be unfavorable; selected six designs cannot establish whole-population behavior; original source commit and publication baseline are different identities.

## 9. Validation, receipts, and handoff

Commands below run from the lane worktree; prefix each with `rtk`:

```sh
cargo test --locked -p jarambit-docs
cargo run --locked -p jarambit-docs --example paper_artifacts -- --check
python3 -B ops/ci/gen_readme.py --check
python3 -B ops/ci/gen_whitepaper.py --check
python3 -B ops/ci/gen_status.py --check
python3 -B -m unittest ops.ci.tests.test_gen_readme -v
make -C docs/paper
git diff --check
```

Required remote gates remain `version-check` and `d1-gates`; use workflow definitions for complete command lists and pinned remote toolchains. Run the workspace gate before push where supported; distinguish any local dependency/toolchain failure from a scientific result. Existing publication target requires pinned Pandoc 3.1.3 and LuaHBTeX 1.17.0; the new local white-paper target does not claim to reproduce those old PDFs.

Expected artifacts: complete atlas and science index; generated charts/metrics/manifest; source manuscript; `target/ieee/paper/main.pdf`; page renderings; build/source hashes; implementation and red-team receipts; PR URL/head. A freshness error means source/projection drift, not a reason to weaken a check. A paper overflow means repair layout/content. A numerical evidence mismatch stops associated claims. CI failure must be diagnosed, not labeled “docs only” and bypassed.

Progress updates: concise findings and next uncertainty at least once per minute during active work. `COMMS.md` receives bounded status/receipt entries, not a transcript. Final handoff reports what changed, what was tested, actual page count, unresolved scientific claims, branch/PR/head, independent-review state and merge state. Never report this author review as independent red-teaming or call a draft tier-1 accepted research.

## 10. Deferred recommendations

After this publication change, consider a separate reporting-only Rust projection shared by README/status/paper/studio, replacing imports from `gen_readme.py` with an explicit snapshot contract. First inventory exact generated-block/admission rules and preserve their refusal behavior. Add provenance-aware UI labels for capability, completeness, model finding and physical validation. Retain original result bytes; expose corrected/withdrawn records rather than overwriting history.

For future neutral research, evaluate reporting reconstruction and agent-authored schema proposals on benign reference designs with preregistered metrics, blinded review, fixed budgets and independent evaluators. Report failures, invalid proposals, unavailable inputs and uncertainty. This is a recommendation for scholarly evidence, not a claim those experiments have happened.
