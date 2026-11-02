# CI: `d1-gates` (.github/workflows/d1-gates.yml)

Runs on every pull request, on pushes to `main`, and on manual dispatch. The [workflow](../.github/workflows/d1-gates.yml) selects the local self-hosted `xbabe`, `general`, `jarambit-d1` pool and a 90-minute job ceiling; its bounded wrapper and step timeouts remain authoritative.

1. `cargo test --release --workspace`: every unit test in all crates. That includes the crescent D1 P0/P1 tests (St-Venant alpha, chord-torsion vs couple, joint factor, genome CSV round-trip, C.6 groove) and the owner-lock tests below.
2. `cargo test --release -p jarambit-crescent d1_lock`: the D1 OWNER-LOCK gate-definition tests (`crates/crescent/src/tests.rs`):
   - screening yield 828 MPa; floors hiker 180 / hunter 260 / ring 1500 N; JOINT thin+hot factor = product.
   - blade >= 3.375 in (+0.5 mm guard) on all three measures; each measure is rejected on its own when short.
   - C.6 is the default aesthetic anchor; a ring-face groove < 2 mm fails aesthetic.
   - twist floor 1.82 N*m: body AND tip submodel, each one gates alone; seq >= 1.2; mass is soft (not in promote_eligible since v0.28, Jepson 2026-10-12); full `promote_eligible` truth table.
3. D1 smoke: `jarambit crescent-d1` on the committed wave14_c03 genome, then `ops/ci/d1_smoke.py`. It requires:
   - the report header states 828 MPa, 1.82 N*m, 180/260/1500 N, blade 3.375 all three, C.6, body AND tip submodel.
   - wave14_c03 reproduces its reference verdict: C.6 PASS, gates ok, blade_min >= 3.375, PASS @828 joint (cert 280.2 / strict 269.8 N, +/- 5), twist body PASS, tip submodel < 1 so twist floor FAIL, and NOT promote-eligible.
   - every row's promote flag equals a recompute from the CSV with the locked thresholds.

CI does NOT run evolve or wave searches, and it does not certify new designs. Promotion still needs human aesthetic sign-off.
Loosening any D1 gate makes this workflow fail. Doing so is an explicit owner decision.


## Storage budget and retention proposal (2026-11-02)

**Status: native Cargo configuration defaults plus a retention proposal.**
Automatic mesh retention still needs a Rust implementation and runtime
qualification. Keep the existing scoring,
certification, promotion and evidence requirements intact.

The disk steward's full inventories found a recurring capacity problem on the
shared research hosts. On xbabe3, legacy continuous-evolution output occupied
about 165 GiB, including about 129 GiB of STEP/STL mesh copies in completed
islands. The installed keepalive synchronizes output to S3 but has no local mesh
retention cap; its fixed 20 GiB free-space admission floor does not reserve 10%
of a 915 GiB root filesystem. Two full scans subsequently observed about
0.303 GiB of evolution growth over 20.2 minutes. These are dated observations,
not a claim that a current build enforces storage limits.

Other writers also matter: xbabe1's remote-test target grew about 25.2 GiB over
20.3 minutes. An evolution retention cap therefore needs to work with host-level
build admission. It cannot by itself bound unrelated CI and research output.
Raw inventories, observation times, process checks and deletion receipts are
retained in the operator's `~/ops/disk_steward/` directory on xbabe2.

### Cargo development build defaults

The native Cargo [configuration template](../ops/maintenance/cargo-storage-defaults.toml)
sets `profile.dev.incremental = false` and `profile.dev.debug = "line-tables-only"`.
The test profile inherits development defaults unless explicitly configured.
Merge only missing keys into the user's `~/.cargo/config.toml`; retain existing
settings and source/registry configuration. Explicit command-line/environment
choices keep their normal precedence. The template leaves target-directory
selection and release profiles to their existing owners.

This addresses builds started through noninteractive SSH shells, where the
caller's usual environment defaults may be absent. On November 2, xbabe1's live
remote-test compiler processes showed `incremental=...` and `debuginfo=2`, while
the corresponding environment defaults and user Cargo config were absent.
Verify the effective compiler arguments with a small offline development/test
build before relying on the installation; do not infer them from a config file
or restart existing jobs to apply it.

### Proposed policy

- Measure the filesystem containing each output root. Use the unrounded
  user-visible fraction `used / (used + available)`, with reserved blocks
  excluded from available space, rather than rounded `df` percentages.
- Begin archive/prune work at 85% use or a 48 GiB local mesh-cache high watermark.
  Aim for at most 32 GiB of eligible mesh copies and below 85% filesystem use.
  These are proposed starting limits, subject to review against per-island peaks.
- Admit a new island only when its projected peak output, all already-admitted
  islands' reserved output, and the host's 10% free-space reserve fit. Missing
  measurements or an exhausted budget defer new launches. Existing processes
  are not killed or signalled by storage maintenance.
- Keep one persistent build directory per lane. Set `CARGO_INCREMENTAL=0` and
  `CARGO_PROFILE_DEV_DEBUG=line-tables-only` unless already set by the caller.
  Do not introduce a global `CARGO_TARGET_DIR`. Build admission must reserve
  capacity before source extraction or compilation, including concurrent jobs.
- Run one maintenance writer per output root, at `nice -n 19` and idle I/O
  priority. A second invocation reports the existing run instead of duplicating it.

### Archive-before-prune contract

Limit automated pruning to explicitly identified, regenerable mesh derivatives
in completed, inactive islands. An `EXIT` marker alone does not establish
completion or disposability: reconcile the run manifest and current consumers,
and require every candidate to have been unwritten for more than six hours.

Retain every path in `leaders.pool`, all genomes and scorecards, canonical
release/certification inputs, evidence and custody stores, signed receipts,
root-owned entries, and every active run directory. Unknown contents remain
held for the owner. Neither a `target` directory name nor a Cargo marker makes
its contents disposable; source and evidence may exist at any depth.

For each eligible file, the implementation must:

1. Establish its canonical path, owner, device/inode, size, modification/change
   times and SHA-256 without following directory or file symlinks.
2. Archive under a content-addressed key in
   `s3://veox-jopedime-learning-use2/experiments/jarambit/`, using the operator's
   `veox-jepson-prod` AWS profile. Keep an existing conflicting object intact.
3. Record the exact object version and verify size plus an actual version-pinned
   GET/readback SHA-256. A successful sync or matching HEAD size is not the
   stronger readback proof required by this proposed implementation.
4. Recheck protected references, the whole island's live process context, and
   privileged `lsof`/`fuser` results. Missing privileges, unexpected diagnostics,
   a failed probe, changed file identity or new activity retain the local file.
5. Write and flush an intent receipt outside the run tree, revalidate the file,
   unlink only that verified file, and write and flush the completion receipt.
   Record host, path, logical/allocated size, reason, SHA-256, S3 key/version and
   the checks used. Keep the source/evidence recovery route independently
   verified before any Cargo-output reclamation.

Archive or verification failure preserves local data. It also prevents new work
from consuming the space that successful retention was expected to recover.
Retained protected data may itself exceed the budget; report that condition
rather than weakening protection or claiming the host is below its limit.

### Acceptance and rollout

A follow-up Rust implementation must prove both the successful path and refusal
cases: open or active runs; leader/proof references; root ownership; symlinks;
recent/changed files; missing privileges; S3 size, version or readback mismatch;
archive failure; a concurrent maintainer; and interruption between intent and
completion. Exercise overlapping build/island reservations and percentage
rounding at the 90% boundary. Interrupted receipts remain inspectable and are
reconciled before retry; they do not establish that deletion completed.

Start with a read-only plan against the real host layout, then a bounded explicit
batch of verified derivatives. Compare two inventories about 20 minutes apart,
including source extraction and build output. Retain the actual observation
intervals and distinguish gross allocated bytes removed from net filesystem
improvement, since hardlinks and other writers affect both.

Enable recurring pruning only after review, passing gates and a real restore
check. Keep the installed service's source/binary identity and configuration in
its receipts. This proposal does not authorize merging a PR, deleting a dirty
repository/worktree, altering certified evidence, or removing an active input.
