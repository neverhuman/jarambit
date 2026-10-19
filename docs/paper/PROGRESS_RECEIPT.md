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
