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
