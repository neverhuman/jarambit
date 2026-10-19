# Paper revision progress receipt

## 19 October 2026, 06:12 UTC — first recurring check

**Lane:** `codex_ieee_whitepaper`. **Branch:** `codex/paper-report-consistency`.

**Base:** `34925f99bd8bb7cd6e4b6e607c15ffe0795a2d32`, fetched from `origin/main` after PR #134 merged. GitHub confirms exact-head bot approval and successful required checks for that merged PR. Earlier reports describing #134 as open/draft remain historical review snapshots.

### PAPER-VIS-01 — reporting consistency

Implemented locally for red-team finding RT-01:

- Selected best score/label must match the final retained Hall of Fame.
- Matching selected/Stage-2 records must agree on content identities and profile.
- Objective membership, recorded weights, normalized values, and weighted points must reconcile within six-decimal rounding bounds.
- All 26 catalog-derived predicate IDs, typed flags, pass tally, aggregate flag, and failed-ID list must agree.
- The three reproduced adversarial mutations are rejected in both write and check modes before any output is overwritten. A permanent entry-point regression test checks this using scratch copies only.

These changes validate reporting records. They do not recompute physical results, equate screening and assessment, or change selection/qualification policy.

### PAPER-VIS-02 — explicit publication CI

Configured locally for RT-05 in the existing `version-check` workflow:

- Test the standalone `crates/paper-artifacts` workspace explicitly and check generated freshness.
- Build/check `docs/paper/main.tex` through its existing Makefile, with an explicit publication-runtime preflight.
- Upload the new PDF and build receipts as `ieee-paper-<checked-out commit SHA>` after successful build, separately from the older PDFs.

Workflow YAML parses locally. The added job steps have not run on GitHub yet. Availability of the new runtime capabilities on the publication runner remains to be established by that run; a missing dependency fails clearly rather than skipping the manuscript.

### Verification

- Standalone tests: **14 passed, zero failed**, under both local Rust 1.95 and the repository-pinned Rust 1.85.1.
- Generator freshness: passes; the eight outputs reconcile.
- `make -C docs/paper check`: passes; manuscript is **16 pages**, with no overfull-box/undefined-reference check failure.
- Numeric/figure outputs and generated atlas content are byte-identical to the base. Only publication source identities in the generated manifest change.
- Original evidence, requirements, scientific crates, solver/evaluation code, and root `Cargo.lock` remain unchanged. Root lock SHA-256: `a7d57f542f3233c99e86bd0973f79ea894ce007c982fddf9bd5a30414035e41b`.
- Pinned Rust 1.85.1 Clippy (`--all-targets -- -D warnings`) passes. Local strict Clippy under Rust 1.95 reported the pre-existing median parity expression as a newer lint; it was not rewritten to an API unavailable at the repository's minimum Rust version.
- Workflow YAML parsing and tracked whitespace checks pass.
- The rebuilt PDF has a different byte digest from the previous local build. Manuscript/figure sources are unchanged; no byte-identical PDF claim is made. The Makefile records the current output digest and toolchain.

Representative pinned command prefix:

```sh
rtk proxy env PATH="/Users/bentaylor/.rustup/toolchains/1.85.1-aarch64-apple-darwin/bin:$PATH" RUSTC="/Users/bentaylor/.rustup/toolchains/1.85.1-aarch64-apple-darwin/bin/rustc" CARGO_TARGET_DIR="$PWD/target/paper-artifacts-pinned" cargo test --locked --manifest-path crates/paper-artifacts/Cargo.toml
```

The paper Makefile and new CI configuration retain their own declared tool/target paths. No global compiler setting was changed.

### Integration and next work

PR #135, `fix: rank Stage 2 by canonical ultralight score`, is currently open with changes requested and a failed D1 check. It owns scientific selection/evaluation paths, which this lane leaves untouched. Under the master plan's one-integration-PR rule, this follow-up remains local rather than opening a competing PR. No push or merge is claimed.

The broad repository gate and fresh exact-head independent approval remain required before this branch is published/integrated. They were not rerun for this bounded local reporting packet. Its narrow tests are not a substitute for those checks.

Next: recheck the integration queue, then prepare the reviewed reporting fix for the required gate/integration. In parallel with queue monitoring, continue the already planned manuscript corrections for stage attribution, original historical-score provenance, the real search-versus-assessment result, and population status composition. The new Blender hero and per-assessment field visuals remain unfinished. The recurring check remains active.

## 19 October 2026, 06:51 UTC — second recurring check

**Lane/base:** same publication lane and `origin/main` base `34925f9`; this packet follows local commit `1826783`. No scientific or other lane-owned files were changed.

### Concrete progress

- Added the read-only `crates/paper-artifacts/src/historical.rs` reader. It admits the original producer revision (`82f450f7fb41`) and recorded `qd.rs` digest, derives the FEM/legality census, and joins the highest stored score to its later assessment by both ID and generation. Unknown states, untyped flags, duplicate IDs, and conflicting assessment identities are refused. Missing assessment stays unavailable in the reader; the manuscript's specific failed-assessment assertion is admitted only when supported.
- Bound the historical population, provenance and search summary to the original wave manifest and pinned that manifest's identity. A changed cohort requires editorial readmission rather than silently retaining this paper's fixed claims.
- Revised the actual population figure and README preview: all **1,984** records remain, with **1,455** blue FEM-complete points and **529** amber not-evaluated crosses. Seventeen stacked bars disclose evaluation-status composition. The summary additionally retains FEM-complete medians and legality counts (**446** legal records).
- Rewrote the abstract and historical results around the retained finding: highest stored score **1.15727**, ID **1095**, generation **9**, has `cert_pass_all=false`. The original producer says GA passers did not beat the baseline; the named exported examples came from separate controlled probes. Historical producer comparisons retain their original, pre-correction scope.
- Labeled weighted reporting stage lineage incomplete in figures, machine-readable provenance and prose. The summary producer can substitute assessment scores for screening scores. The three retained summaries therefore do not establish convergence under one fixed objective.
- Attributed historical fitness to its original legality-dependent, contextual-novelty producer. Changed the unsupported human-comparison title to **When CAD Evolves**. Original evidence and stored scores were preserved.

This advances RT-02/03/07/08 and the title portion of RT-11. It does not close the broader review: the exact source-bound Blender cover, representative field plate, complete subsection visuals, fuller methods/results narrative, independent reproduction and release integration remain outstanding. The interim weighted graphic is still on page one and is explicitly identified as unfinished in the paper build README. No render or new physics run is claimed.

### Verification and artifacts

- Pinned Rust **1.85.1**: **18 tests passed**, Clippy `--all-targets -- -D warnings` passed. Formatting and tracked whitespace checks passed.
- Generator write followed by freshness check passed: eight outputs, 414 retained battery cells, 178 assurance definitions. Original README generator check passed.
- A separate read-only Python cross-check independently calculated the raw census, all 17 generation reductions, FEM-complete medians and best-assessment join, and counted exactly 1,455 point marks plus 529 cross marks in the SVG. This is internal author verification, not independent scientific reproduction.
- `make -C docs/paper check` passed: **16 pages**, no checked overfull boxes or undefined references/citations. Rendered all pages and inspected the contact sheet, cover and revised population/results pages. QA: `target/ieee/paper/qa-20261019/`.
- PDF: `target/ieee/paper/main.pdf`, SHA-256 `53627940961d6052bcdd9ea553133b1f32494ede09e3f79606ec43c1fd78590c`. Source/output/tool receipts accompany it. Machine check receipt: `target/paper-progress-2026-10-19T0651Z.json`.
- Diff against `34925f9` confirms no original evidence, requirements, scientific code, CAD, scoring/solver definitions or root lock changes. Root lock digest remains `a7d57f542f3233c99e86bd0973f79ea894ce007c982fddf9bd5a30414035e41b`.

### Integration and next action

Fresh GitHub inspection finds PR **#135** still open, now at **`f7dee598ae011c58957b7e3f77a7974687f49676`**. Its version check succeeds and D1 is in progress. Review decision remains changes requested; the latest bot diagnosis concerns evaluator-source identity changes on the preceding head and is not approval of the new head. Earlier receipts describing a failed D1 apply to that preceding revision. This lane preserves the single integration queue and makes no push/merge claim. The full required gate and exact-head approval remain prerequisites for this publication follow-up.

Next bounded packet: admit the exact assessed-example STL and retained plate field semantics, add isolated render outputs, then produce the early Blender cover/representative numerical plate specified by P1/P2. Continue to disclose different hero/population cohorts and unavailable full fields. Recheck the integration queue before publishing these local commits. The 30-minute check remains active.

## 19 October 2026, 07:26 UTC — third recurring check

**Progress:** completed the source-bound Blender cover portion of P1/P2 and RT-04. The cover now places the exact assessed export directly below the abstract, beside all 1,984 historical births and independently reconstructed generation/cumulative maxima. Index terms follow the figure. The heterogeneous weighted summary moved into the results, completing its cover demotion under RT-02. Different geometry/population cohorts and the unqualified assessed-example status remain explicit. The manuscript is **17 pages**.

**Render:** `docs/paper/figure-inputs.v1.json` freezes six original input identities. The selected `figures/v2/hero_dark.png` is a completed 3,600 × 2,800, 256-sample Cycles CPU render (four threads, seed 42, denoising, AgX). SHA-256: `29e8f64665e961c8129a663fdbbb2af884961f7c68aeb21fe731c272f81d6698`. Its scene/render receipts record camera, lights, color configuration and identical before/after imported geometry digests: **44,994 vertices and 90,000 faces**. No geometry modifier, smoothing, remeshing or deformation was applied. Three earlier 48-sample lighting proofs informed the choice. Illustrative material appearance and original export faceting are disclosed.

**Compatibility correction:** archived figure receipts hash `render_paper.py` and `paper_scenes.py`. Extending those files would invalidate the archive. Their bytes were restored and the profile instead extends the existing approved, previously unconsumed `rerender.py`/`candidate_scenes.py` bindings. Legacy final/preview admission remains intact; publication uses a separately labeled, frozen assessed-example profile with isolated output roots. Two intermediate high-resolution attempts were canceled during compatibility corrections; only the final completed source-bound run is published. The new manifest also satisfies the existing figure verifier. No new Python file was added. The checksum-verified official Blender 4.5.3 DMG remains in lane-local `target/tools`; its temporary read-only volume was ejected after rendering.

**Checks:** pinned Rust 1.85.1 **19 tests passed**, strict Clippy passed; existing publication-preparation tests **6 passed**. Generator write/freshness checks pass with **nine outputs**. README generation check, workflow YAML parse and whitespace checks pass. Figure verifier: **52 manifests, zero errors**; media verifier: **12 manifests, zero errors**. Original archived images/manifests/render sources, evidence, requirements, scientific implementations, CAD and root lock are unchanged by this publication packet. Separate raw-record verification confirms 1,455 FEM point marks, 529 unevaluated cross marks, and both 17-generation maximum curves.

**PDF QA:** Makefile checks pass; all 17 pages rendered and contact-sheet/cover/changed-figure layout inspected under `target/ieee/paper/qa-cover-20261019`. Current PDF SHA-256: `8bfedb81cf7c07c9d8d92b411afced9fcec1f35671171bf0875c586e3a05347a`. Source/output/tool receipts remain beside the PDF. These are internal author checks, not independent scientific reproduction or exact-head bot approval.

**Integration changed:** PR #135 received `neverhumanbot` approval on exact head `f7dee598ae011c58957b7e3f77a7974687f49676` at 07:05:32 UTC, with successful required checks, and merged at 07:06:09 UTC as **`72e25a3604700a13b7fcf4af0523224eb23db6aa`**. Fresh fetch confirms that main. Its only changed paths are the CLI Stage-2 implementation and its integration test. README/manuscript now identify this subsequent scorer change without retroactively rescoring retained cohorts. Queue hold is lifted; the publication branch still needs rebasing, its full required local gate, its own PR/CI and exact-head approval before integration. No publication push or merge is claimed.

**Next:** preserve this packet in a local commit, rebase onto verified main, then prepare the required gate/integration while continuing the representative numerical plate. Field-source audit verified that `crates/crescent/src/fem.rs` and `crates/cli/src/crescent.rs` match the assessed record's kernel revision `e8cb049df7d0`. Retained D1 arrays contain 13,503 nodes and 25,209 triangles; the envelope peaks at 1,388.6695 and contains 75 stored zeros. Case arrays have different unit-response and masking semantics; zeros also reflect serialized rounding. Resolve the exact display mask and full-range color mapping before releasing a heatmap. Actual subsection field visuals and missing full fields remain unfinished. Also harmonize the legacy README release drawer's “356 controls” wording with the atlas's more precise “356 required paired-control slots.” The recurring check remains active.

**07:28 UTC integration handoff:** the three local publication commits rebased cleanly onto `72e25a3604700a13b7fcf4af0523224eb23db6aa`. The publication diff against that main contains no protected scientific/evidence/archived-render changes. Post-rebase checks again pass: 19 standalone tests, nine fresh projections, 17-page PDF. The rebuilt PDF's final digest is `a2ac579ad7f7db84be62a6cddb2d84b7a39f5f63829df31ecf71f1621ffc8595`; identical source content does not imply byte-identical PDF builds. Final page renders are retained alongside the initial QA. Required-gate workspace phase is handed to a bounded Rust 1.85.1/two-build-job run, with log/result under `target/paper-required-gate-20261019.{log,json}`; inspect that result and process state before reporting completion. Subsequent Clippy/D1/CAD/workflow checks and independent exact-head review remain separate requirements.

## 19 October 2026, 07:51 UTC — actual retained plate visualization

**Bounded work completed:** added a source-bound four-panel FEM plate for the cover's assessed example, `cert_pipeline_r01_p14_c02`. This is a reporting projection of saved arrays, not a new solve, optimization or qualification. `field-inputs.v1.json` binds the original D1 SHA-256 `c2811ed8877904254ddc953dc888aedc3d7791b69c040dd6513fe0b763997bab`, original kernel revision and the four interpretation sources. The Rust reader checks identity, source membership/hashes, mesh/value associations, units and reconstructible masks before writing.

- Displays 13,503 retained nodes / 25,209 triangles: scaled envelope in MPa; lateral and ring unit responses in MPa/N; torsion unit response in MPa/(N mm). Each legend spans zero to its own unmodified maximum. No clipping, interpolation, smoothing, deformation or new physical assumption.
- Gray identifies 75 producer-zeroed envelope cells and 14,875 producer-zeroed ring cells. The tip mask's nearest threshold margin is about 0.01757 mm, above the conservative coordinate-rounding bound. Ring cell 11825 remains active because its positive saved coefficient resolves rounded boundary membership. An ambiguous zero would be refused. Raw lateral/torsion tip responses remain visible with the original capacity exclusion outlined.
- Manuscript methods distinguish DKT surface corner maxima from T6 centroid recovery, serialized rounding, the displayed 0.25 mm² level and capacity minima across two mesh levels. G16 subsection references point to actual shared fields. No strict, solved solid, fatigue or buckling field is invented. These missing-field visuals remain open.
- SVG and projection/figure manifests are committed outputs. The PDF uses an identical 3,600-pixel raster projection to remove vector triangle-edge seams exposed in visual QA. CI artifact configuration now includes the field contract, projection receipts, source SVG and displayed PNG. Remote execution of this new publication workflow remains pending.
- Corrected the generated README release drawer from “356 controls” to “356 required paired-control slots,” without claiming catalog execution.

**Checks:** pinned Rust 1.85.1: **22 tests passed**, strict Clippy passed; publication/README suites **11 tests passed**. Twelve projections are fresh. Figure verifier **53 manifests / zero errors**; media verifier **12 / zero errors**. README generation, workflow YAML, formatting and whitespace checks pass. A separate source-to-display audit verifies all **100,836 polygons**, **36 interior raster samples** and **20 legend anchors**. Original evidence, scientific code, CAD, objective policy, thresholds, archived assets/render sources and root lock remain unchanged; root lock SHA-256 remains `a7d57f542f3233c99e86bd0973f79ea894ce007c982fddf9bd5a30414035e41b`.

**PDF:** **18 pages**, no overfull boxes or unresolved references/citations. Rendered all 18 pages and inspected the contact sheet and methods/figure pages. Final raster substitution changes only page 8's pixels. PDF SHA-256: `cc3ad58bc777eda1fd5c405416d07eb52b2e39fb2c0adb038d25ff920908ffb4`. Cell PNG: `761efdf523cf68b5e56fec30bab54bfbaa89f346f6d8fba4b95528a19ec4986d`. Output and QA: `target/ieee/paper/{main.pdf,plate-visual-qa.json,qa-plate-final-*.png}`. pypdf still warns about duplicate `/Group` keys in three older embedded SVG-PDF objects; Poppler renders them. Structural cleanup/toolchain portability remains a separate follow-up, and `qpdf` is unavailable locally.

**Gate and integration:** the release workspace phase on `08ed64b1dd310ca34296077ac903546e4f8c5fbe` ended **failed / 101 at 07:43:54 UTC**. CLI 64 tests plus two integration tests passed; crescent finished 37 passed / one failed, at the unchanged `tests::d1_knife_fixture_reports_torsion_and_couple` assertion (`crates/crescent/src/tests.rs:206`). An isolated exact-test rerun reproduced it. Scientific source/dependencies match main `72e25a360470` byte-for-byte; no threshold or fixture was weakened. Later broad-gate phases are incomplete. Fresh GitHub inspection at 07:49 finds **PR #136** open at `787592b12ec8d17cca7c65b4c7fc5f9557bc4640`, version check green, D1 running, no reviews. Preserve its ownership and the single integration queue. This publication packet remains local, with no push, new PR or merge claim.

**Next bounded action:** finish source-backed per-assessment scalar/outcome visuals where full fields are absent, keeping those distinct from heatmaps; address the remaining PDF structure issue; continue the methods/results and contribution review. Recheck the active integration queue and local gate blocker before preparing the publication PR. Exact-head independent `neverhumanbot` approval, green required CI and writer-role matched-head rebase merge remain required. The 30-minute check stays active; the overall publication work is not complete.
