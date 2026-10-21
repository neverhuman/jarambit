# Regression dataset factory — engineering specification

Status: implemented in `feat/regression-dataset-factory` (rebased on `main` @ `49dc71f`).
Owner surface: `jarambit dataset …`, `jarambit auto-evolve` recording flags, `regression-datasets` CI.

## 1. Goal

Every design the search evaluates is a labelled example: a genome, the CAD it produces, and the
outputs of one or more tests (Stage-1 plate FEM screen, Stage-2 independent solid/plate assessment,
and — for printed titanium parts — physical tests). The factory captures each evaluation **as it
happens**, stores it durably in git, and materialises as many regression tables as the data
supports, each with the **test output as the final column**. CI then runs CatBoost k-fold
cross-validation on every table and reports the out-of-fold Pearson *r* (with a 95 % CI), so the
team can see which test outputs are predictable from which design descriptions — i.e. where a
surrogate would find lift.

Non-goals: changing search, ranking, gates or certification. Recording is observational; the
survivor list, its order and every committed evidence byte are unchanged (§11).

## 2. Repository constraints honoured

| Constraint (source) | Consequence for this design |
|---|---|
| Rust for backend/CLI/CI/compute; no new Python (`AGENTS.md`, `PYTHON_EXCEPTIONS.md`) | Everything is Rust. CatBoost runs as its official standalone CLI binary, not the Python package. |
| `certification::identity` hashes `Cargo.lock` and the crescent/physics sources; the root `Cargo.lock` digest is audited (`#131`) | No new crate, no new dependency, no edit to any identity-hashed source. All code lives in `crates/cli/src/datasets/` and `crates/cli/src/auto_evolve.rs` (neither is hashed). `Cargo.lock` is byte-identical. |
| Evidence and certification inputs stay byte-stable | Records go to a new `datasets/regression/records/` tree. No existing evidence file is rewritten. |
| Fail closed | Missing CatBoost/zstd, malformed recipes, torn shards that block appends, CV errors (with `--fail-on-error`) all exit non-zero. |
| Self-hosted runner policy (`crates/runner-policy`) | The workflow uses `[self-hosted, linux, x64, xbabe, general]`, pinned action SHAs, `contents: read`, and a new bounded profile in `ops/ci/run_bounded.sh`. |

## 3. Architecture

```
            ┌──────────────── live GP (jarambit auto-evolve) ────────────────┐
births ───► │ Stage-1 screen (qd::evaluate, plate FEM 2.0 mm²)               │
            │   └─► featurize::record_from_eval ─► r00.s1.csv  (1 row/birth)  │
            │ Stage-2 assessment (score_cad.py: skfem P2 + DOLFINx)           │
            │   └─► recorder.append_s2 ─────────► r00.s2.<label>.csv           │
            │ end of round ─► git commit --only <run dir> [push + rebase]      │
            └─────────────────────────────────────────────────────────────────┘
rescreen / harvest-waves / add-physical / refeaturize ─► more shards (same schema)
                                   │
                     datasets/regression/records/**  (committed, append-only)
                                   │        + artifacts/…gen800… (committed archives, read via zstd)
                                   ▼
             jarambit dataset build  (recipes.conf)  ─► tables/<target>__<featureset>.csv
                                   ▼                    (+ .card.json, index.json, index.md)
             jarambit dataset cv (CatBoost CLI, k-fold) ─► cv/leaderboard.{json,md}, *.cv.json, *.oof.csv
                                   ▼
             CI: regression-datasets.yml ─► job summary (r, CI, R², lift) + artifact
```

## 4. Where it lives

### New files

| Path | Purpose |
|---|---|
| `crates/cli/src/datasets/mod.rs` | `jarambit dataset` dispatcher; CAD-dependent commands `featurize`, `refeaturize`, `rescreen`, `add-physical` |
| `crates/cli/src/datasets/featurize.rs` | design features (`c.`, `a.`, `gate.`, `desc.`) from the crescent CAD kernel; Stage-1 targets (`s1.`) from `Fem` |
| `crates/cli/src/datasets/recorder.rs` | live recorder used by auto-evolve: run manifest, per-round shards, Stage-2 rows, commit/push |
| `crates/cli/src/datasets/records.rs` | shard writer/reader, torn-tail handling, genome-keyed frame loader and attachment joins |
| `crates/cli/src/datasets/gfeat.rs` | canonical 129 genome columns, genome hash, genome-derived features (`gd.`) |
| `crates/cli/src/datasets/sources.rs` | read-only gen-800 archive source (zstd CLI, bottom-k genome-hash sampling) |
| `crates/cli/src/datasets/recipes.rs` | `recipes.conf` parser and target × feature-set expansion |
| `crates/cli/src/datasets/materialize.rs` | table assembly, leak guard, constant/missing drops, cards, index |
| `crates/cli/src/datasets/cv.rs` | CatBoost k-fold runner, OOF statistics, lift verdict, leaderboards |
| `crates/cli/src/datasets/stats.rs` | Pearson, Spearman, R², RMSE, MAE, Fisher-z CI, SplitMix64, FNV-1a |
| `crates/cli/src/datasets/harvest.rs` | converts retained `round_NN_births/screen.json` + `leaderboard.json` into shards |
| `crates/cli/src/datasets/gitsync.rs` | `git add/commit --only` of record paths; push with fetch + rebase retry |
| `crates/cli/src/datasets/json.rs`, `csvio.rs` | dependency-free JSON and CSV codecs |
| `crates/cli/src/datasets/engine.rs` | pure command layer (`build`, `cv`, `harvest-waves`, `commit`) shared with the standalone binary |
| `crates/cli/src/datasets/standalone.rs` | crate root that compiles the pure modules with plain `rustc` (CI; no workspace build) |
| `datasets/regression/recipes.conf` | which tables to build |
| `datasets/regression/README.md` | operator guide |
| `datasets/regression/records/harvest_waves/**` | 3,194 births / 92 Stage-2 rows harvested from `evidence/evolution_waves` |
| `datasets/regression/records/rescreen/**` | design-of-experiments Stage-1 screens around the w282/w283 seeds (full features + 41 targets) |
| `.github/workflows/regression-datasets.yml` | CI: build tables, CatBoost k-fold, publish r-values |
| `docs/REGRESSION_DATASETS.md` | this specification |

### Changed files

| Path | Change |
|---|---|
| `crates/cli/src/main.rs` | `mod datasets;` and the `jarambit dataset …` dispatch |
| `crates/cli/src/auto_evolve.rs` | recorder flags; `screen_candidate_stage1_recorded` (returns survivor + record); birth identities computed once; Stage-1 shard per round; Stage-2 record per assessment; commit/push at end of round; `design_value()` shared by leaderboard and recorder |
| `evolve.sh` | `JARAMBIT_DATASET_COMMIT=1` by default; documents `JARAMBIT_DATASET_PUSH` |
| `ops/ci/run_bounded.sh` | `datasets` profile (8 GiB, 4 CPUs) |
| `.gitignore` | derived `datasets/regression/tables/` and `cv/` |
| `TECHNOLOGY_MAP.md` | one row for the new lane |

## 5. Micro-record format

A **shard** is a CSV with one header line under `datasets/regression/records/<lane>/<run_id>/`.

* **Append-only, crash-safe.** Every row is written with one `write_all` + flush. A crash can only
  leave a torn last line (no trailing newline); readers drop it and report `torn_tails`; writers
  refuse to append after one. Per-round Stage-1 shards are written atomically (temp file + rename).
* **Run-unique directories.** `run_id = <prefix>-<UTC stamp>-<host>-s<seed>` so hosts never write
  the same path; rebases of record-only commits cannot conflict.
* **Base vs attachment.** A shard carrying all 129 genome columns is a *base* shard. A shard without
  genes (Stage-2, physical results keyed by hash, refeaturized CAD) is an *attachment* joined on
  `meta.genome_hash`; repeated attachment values (replicate physical specimens) are averaged.
* **Genome identity.** `meta.genome_hash` = FNV-1a-64 over the 129 gene values' IEEE bits (±0
  canonicalised) in canonical order. Stage-2 rows also carry `meta.certificate_identity` (the
  existing content-bound SHA-256), which `add-physical` can resolve.
* **Precision.** Genes are written with Rust's shortest round-trip representation, so
  `Genome::from_csv` reproduces the evaluated genome exactly. Features/targets use 7 significant
  digits. Non-finite values are empty cells (never zero).

### Provenance columns (`meta.`)

`lane, source, run_id, round, idx, family, label, certificate_identity, genome_hash, status
(screened | rejected_aesthetic | rejected_gate | fem_failed | not_rs_v2), feature_schema,
fem_nodes, fem_tris, seq_case` plus `run.json` per run (host, seed, git HEAD, argv, seeds, flags).

## 6. Features (everything a model may use)

All design features are computed from the genome and the CAD kernel only — **no FEM output is ever
a design feature**, so every design feature set is a legitimate predictor of every test output.

| Family | Count | Content |
|---|---:|---|
| `g.` | 129 | 40 silhouette control-point offsets `cp00…cp39`, the 88 named genes (blade drop, grip, ring wall/ID, neck fillet, stock, taper, edge angles, comet/eye/cairn holes, serrations, relief, lattice, fuller, flute, texture, wrap, brand pad, edge thickness, isogrid, pierced-fuller windows, ring flute, perforation lattice, P1 rails/neck/pockets/knots) and the material flag |
| `gd.` | 62 | per-zone normalised silhouette means/abs-means (tip-inside, belly, heel, spine rear/mid/tip), silhouette energy, curvature roughness, total variation, extrema and their positions, Fourier harmonics 1–6 of the closed offset field, stock − tip, ring OD and annulus, wall × stock, edge-angle mean/difference, comet/fuller/window spans, effective architecture switches (P1, isogrid, windows, perforation, lattice, relief, texture, fuller, flutes) and their count, removal densities, knot count/min/max/mean/range |
| `c.` | 234 | mass, blank mass, removed fraction and per-mechanism removed volume (relief, lattice, fuller, comet flute, texture, isogrid/pocket, ring flute); stock and joint thickness; blade tip-to-heel / horizontal reach / inside arc / spine arc / sharpened arc / hook drop; tip & heel coordinates, tip radius & angle; grip length; ring OD/ID/wall; serration teeth & ripple; outline and macro perimeters, bounding box, convex-hull area and solidity; per-hole area, count and ligament to the outline (ring, comet, eye, cairn, window, perforation), minimum hole-outline and hole-hole ligaments; thickness-field mean/sd/CV/quantiles and full-stock / thin fractions; centre of mass, planform centroid, mass moments (about COM and ring centre, principal values and angle, radius of gyration); **12 cross-sections** along the part (area, out-of-plane second moment Σw·t³/12, in-plane second moment, width, number of material runs) and the minimum blade-side out-of-plane stiffness; **12 polar sectors** about the ring (area, mass fraction, mean thickness, radial extent); **planform distance transform**: largest inscribed radius, mean half-width, narrowest ridge half-width overall / within 8–40 mm of the tip / in the body / at the ring, and min(2·half-width·t³) along the ridge (a ligament bending-stiffness proxy) |
| `a.` | 44 | C.6 aesthetic metrics (Hausdorff, EMD, straight run, min concave radius, spine convexity, inside concavity, seat depth, lobe prominence, hook drop, score, pass, fail count) and the 32-bin macro-silhouette curvature histogram |
| `gate.` | 20 | reject count/pass, rejects by kind, holder fit (fits, length, width, angle), ring-bearing width and band, bore roll radius and bearing length, RS-v2 edge regime |
| `desc.` | 16 | the QD novelty descriptor |

Cost (release build, one core): phenotype 2.1 s and aesthetic + gates 0.8 s are already paid by the
Stage-1 screen; the additional design features cost ≈ 0.2 s per birth.

## 7. Targets (the test outputs)

| Family | Source | Targets |
|---|---|---|
| `s1.` | Stage-1 plate FEM screen (`qd::evaluate`, 2.0 mm²) | `lat20, pry20, twist20_nmm, ring20, lat_rob, ring_rob, sf_hiker, sf_hunter, sf_ring, couple20_nmm, twist_kc, twist_b_mm, twist_t_mm, twist_joint_nmm, twist_sf, seq_ratio, twist_tipdist, twist_body_joint_nmm, twist_hand_joint_nmm, twist_body_kc, twist_body_sf, twist_tipzone_kc, twist_tipzone_joint_nmm, twist_tipzone_sf, hm_twist_nmm, hm_tip_inplane_n, hm_spine_n, hm_ringback_n, hm_twist_sf, hm_tip_inplane_sf, hm_spine_sf, hm_ringback_sf, fat_sf`, peak locations `lat_at_{x,y}`, `ring_at_{x,y}`, `twist_at_{x,y}`, `spg`, `screen_deficit`, `ultralight_weighted_fitness` |
| `s2.` | Stage-2 assessment (`score_cad.py`, skfem P2 + DOLFINx) | leaderboard scalars (`mass_g, mass_3d_g, w_cad, g1, ultralight_weighted_fitness, solid_tip_sf, strict_hunter_sf, seq_ratio, fat_sf, ring_bearing_w_mm, buckling_eigenvalue_a4`) plus every numeric leaf of the scorecard's `engineering_metrics` (`s2.em.*`), `weighted_cad_score` (`s2.wc.*`), `export_measurements` (`s2.x.*`) and the RS-v2 `human_max` block (`s2.hm.*`) |
| `h.` | gen-800 archives | `lat_rob, ring_rob, sf_hunter, spg, mass_g, aes_score, hausdorff, emd, relief_frac, fit` |
| `phys.` | `add-physical` | any numeric column of a lab CSV (load at yield, failure load, deflection, …) |
| `c./a.` surrogates | CAD kernel | `c.mass_g, c.blade_tip_to_heel, c.blade_sharpened_arc, c.grip_len, c.ridge_min_halfwidth_mm, c.sec_min_ioop_blade, c.th_p05, a.score, a.hausdorff` from genome only |

## 8. Producers

### 8.1 Live GP hook (`auto_evolve.rs`)

1. `RecorderConfig::from_args` parses `--no-dataset`, `--dataset-dir`, `--dataset-commit`,
   `--dataset-push REMOTE:BRANCH` (env: `JARAMBIT_DATASET=0`, `JARAMBIT_DATASET_COMMIT=1`,
   `JARAMBIT_DATASET_PUSH`). `Recorder::start` writes `run.json`.
2. Births: certificate identities are computed once and reused for `round_NN_births.json` and records.
3. Stage-1 workers call `screen_candidate_stage1_recorded(…, record)`, which returns the unchanged
   `Option<ScreenSurvivor>` plus the birth's record (built from the phenotype and `qd::Rec` the
   screen already computed). Survivors are collected **in the original worker order**, so the stable
   fitness sort and every downstream selection are identical to the unrecorded run.
4. After `round_NN_screen.json`: rows get provenance and the Stage-1 `W_ULTRALIGHT_v1` value from the
   ranked JSON, then `r{round}.s1.csv` is written atomically.
5. Stage 2: every verified assessment appends `r{round}.s2.<label>.csv` from the same
   `design_value()` object the leaderboard uses, enriched with the scorecard/RS numeric leaves.
6. End of round: `git commit --only -- <run dir>`; optional push.

Recording is on by default, so anything that launches `auto-evolve` as a test or process control
must pass `--no-dataset` / `JARAMBIT_DATASET=0` or send records to a scratch `--dataset-dir`;
otherwise control runs (for example with a synthetic Stage-2 evaluator) would land in the committed
records. `crates/cli/tests/evolve_pipeline.rs` does both, and its fail-closed controls assert that
every birth is recorded but only a receipt-verified Stage-2 result becomes an `s2` row.

### 8.2 Other producers

* `jarambit dataset rescreen --genomes F [--neighbors N --sigma S] [--random N]` — parallel Stage-1
  screens of genome files, GA-operator neighbourhoods or space-filling random designs (RS-v2 regime
  applied like auto-evolve seeds). This decouples dataset growth from the GA's selection bias.
* `jarambit dataset harvest-waves` — converts retained auto-evolve outputs (births + screens +
  leaderboards) into shards; `refeaturize` later adds CAD features as attachments.
* `jarambit dataset add-physical --csv results.csv --campaign NAME` — lab rows identified by
  `genome_csv`, `genome_hash` or `certificate_identity`; numeric columns become `phys.*`.
* The committed gen-800 archives are read in place (no copy) as a build source.

## 9. Getting records into git automatically

* **What is committed:** only `datasets/regression/records/**` (the irreproducible evaluations). Tables
  and CV outputs are derived, deterministic and git-ignored.
* **How:** `gitsync::commit_paths` runs `git add -- <run dir>` then
  `git commit --no-verify --only -- <run dir>`, so unrelated staged or unstaged work in the checkout is
  never swept into a data commit. Index-lock contention is retried with backoff.
* **Push (optional):** `git push <remote> HEAD:refs/heads/<branch>`; on rejection the recorder fetches and
  rebases onto the remote branch **only if the checkout has no other tracked modifications**, then
  retries (3 attempts). Because each run owns its directory, rebases are conflict-free.
* **Recommended topology:** GP hosts run from a dedicated data worktree
  (`.agent/worktrees/datasets`, branch `data/regression-records`) with
  `JARAMBIT_DATASET_PUSH=origin:data/regression-records`; CI runs on pushes to `data/**`; a periodic
  PR merges the data branch into `main`. Developers' feature branches never receive data commits.
* **Size budget:** a full live row is ≈ 4.7 kB of CSV; 10⁴ births ≈ 47 MB before git's zlib packing.
  If records outgrow plain git, the same paths can move to Git LFS without code changes.
* **Hooks:** data commits use `--no-verify` (they contain no code). Code commits keep the jankurai
  pre-commit gate.

## 10. Materialisation (`jarambit dataset build`)

`recipes.conf` grammar: `version`, `min_rows`, `max_rows`, `leak_guard_r`, `source records <dir>`,
`source gen800 <dir> cap=N`, `featureset <name> = <family.> …`, `targets <pattern> … : <featureset> …`,
`skip <pattern> …`, `leak <target> : <feature pattern> …` (patterns are exact names or `prefix*`).

Feature sets: `g` (genome), `c` (CAD only), `gc` (full design), `gcs` (design + Stage-1 screen:
multi-fidelity), `s` (screen only: screen → assessment transfer).

For every (target, feature set):

1. Rows = all frames containing the target, with a finite target **and** at least one value in every
   family of the feature set (so `gc` tables only use rows that really have CAD features).
2. De-duplicate by genome hash (records before archives), sort by genome hash.
3. Truncate to `max_rows` (hash order = deterministic random sample).
4. Drop all-missing and constant columns; drop recipe leak patterns; drop any feature with
   |r| ≥ `leak_guard_r` against the target (exact copies); never include the target itself.
5. Skip with a recorded reason if `n < min_rows`, the target is constant, or nothing informative is left.
6. Write `<target>__<featureset>.csv` (features…, **target last**), `.card.json` (sources, row digest,
   target statistics, every dropped column and why), `.groups` (one lineage key per row: the DOE
   parent `meta.parent_hash`, else GA `run_id/family`, else the gen-800 run, else the genome) and
   `index.json/md`.

Rebuilding from the same inputs is byte-identical (tested).

## 11. CatBoost k-fold CV (`jarambit dataset cv`)

* Binary: official CatBoost CLI (`catboost-linux-x86_64-1.2.10`, sha256 pinned in CI) via `--catboost`
  or `JARAMBIT_CATBOOST`. Last column → `Label`; text columns (if any) → `Categ`.
* Folds: SplitMix64 permutation seeded by `seed ⊕ FNV(table name)`, k = 5 by default.
* Per fold: `fit --loss-function RMSE --use-best-model false` (the held-out fold never steers
  training), 500 iterations, depth 6, lr 0.05; predictions via `--eval-file`; feature importance via
  `PredictionValuesChange` averaged over folds.
* Metrics on pooled out-of-fold predictions: Pearson *r*, Fisher-z 95 % CI, Spearman ρ, R², RMSE,
  MAE, RMSE of the fold-mean baseline, lift % = 100·(1 − RMSE/RMSE_baseline). Optional
  `--permutations P` refits on permuted targets for an empirical *p*.
* **Two fold schemes.** Random k-fold measures interpolation between designs already seen.
  Lineage-grouped k-fold (from `<table>.groups`; groups dealt largest-first to the lightest fold;
  k = min(5, groups); needs ≥ 3 groups) holds every DOE parent / GA run+family / gen-800 run out
  whole, measuring prediction for an unseen design family. Both r values are always reported.
* **Verdict.** `lift`: r > 0, CI lower bound > 0 **and** RMSE < baseline. `weak`: r > 0 otherwise.
  `none`: r ≤ 0 or undefined. The headline verdict uses the grouped scores whenever the table has
  ≥ 3 lineage groups, otherwise the random ones (`verdict_basis`). A unit test shows the difference:
  a per-lineage constant target with a lineage-fingerprint feature is `lift` under random folds and
  not under grouped folds.
* Outputs: `cv/<table>.cv.json`, `cv/<table>.oof.csv` (random and grouped OOF predictions and folds),
  `cv/leaderboard.{json,md}` (lift first, then the basis r).
* Parallelism: `--jobs` tables at once, `--threads` split among them; per-fit `--timeout`.

## 12. CI (`.github/workflows/regression-datasets.yml`)

Triggers: PRs/pushes touching `datasets/regression/**`, the dataset code or the workflow; pushes to
`main` and `data/**`; nightly; manual dispatch with `folds`, `max_rows`, `iterations`, `only`,
`permutations`. Steps (bounded `datasets` profile): compile + test the engine with plain `rustc 1.85.1
-D warnings` (seconds, no workspace build) → install/verify CatBoost → require `zstd` → `build` →
`cv --fail-on-error --summary $GITHUB_STEP_SUMMARY` → upload leaderboard, per-table results, OOF
predictions, cards and index. The job summary is the r-value/lift leaderboard. The full-workspace
unit tests of `featurize`/`recorder` run in the existing `d1-gates` `cargo test --workspace` job.

## 13. Initial results

Run on this branch over all 160 tables (recipes above, ≤ 10,000 rows per table, CatBoost 1.2.10,
500 iterations, random + lineage-grouped 5-fold): 70 minutes on a 2-core VM, 0 errors. The 29 tables
that changed when test-control records were removed from the seed data (all `s2.*`, nine `c.*`/`a.*`)
were re-run on the corrected tables; the other 131 tables are byte-identical.
**145 of 160 tables show lift on lineage-grouped folds** (107 strong with R² ≥ 0.5, 20 moderate,
18 marginal); random folds alone would have called 152.

- **The cheap screen predicts the expensive assessment.** Five Stage-1 screen scalars predict Stage-2
  solid-tip SF, W_CAD and G1 at grouped r ≥ 0.98 (n = 66 assessed designs, 34 run/family groups).
  Small n: a strong lead for a pre-certification surrogate, not yet a validated one.
- **Strength generalises across independent GA runs.** In the gen-800 archives (10 runs, 10,000 rows)
  the held-out-run r is 0.98 for ring pull and 0.83 for lateral strength.
- **Around the w282/w283 seeds, Ti grade and edge thickness dominate** (top features in 52 and 29 of
  the small Stage-1 tables) because GA mutation flips the titanium grade; the neighbourhoods are narrow.
- **Not yet predictable:** torsion peak geometry (`twist_at_y`, `twist_t_mm`, `twist_body_kc`) and
  several CAD surrogates collapse once lineages are held out (`c.blade_tip_to_heel` 0.85 → 0.22).
  These need more diverse designs (`rescreen --random`) or local features at the peak.
- **Redundant targets:** several Stage-1 safety factors are fixed-load rescalings of one capacity and
  share an r (`twist_sf`, `hm_twist_sf`, `twist_tipzone_sf`); adding them to `skip` halves their CV cost.

Best feature set per target (70 targets):

| target | best feature set | n | lineage groups | r random | r grouped [95 % CI] | R² grouped | verdict |
|---|---|---:|---:|---:|---|---:|---|
| `c.th_p05` | `g` | 196 | 6 | 0.999 | 0.996 [0.995, 0.997] | 0.982 | lift (strong) |
| `s1.ring_at_x` | `c` | 155 | 7 | 0.988 | 0.995 [0.993, 0.996] | 0.953 | lift (strong) |
| `s2.w_cad` | `s` | 66 | 34 | 0.993 | 0.990 [0.984, 0.994] | 0.978 | lift (strong) |
| `s2.mass_g` | `g` | 66 | 34 | 0.991 | 0.990 [0.983, 0.994] | 0.977 | lift (strong) |
| `s2.mass_3d_g` | `g` | 66 | 34 | 0.993 | 0.990 [0.983, 0.994] | 0.977 | lift (strong) |
| `s2.solid_tip_sf` | `s` | 66 | 34 | 0.996 | 0.990 [0.983, 0.994] | 0.978 | lift (strong) |
| `s2.g1` | `s` | 66 | 34 | 0.990 | 0.987 [0.979, 0.992] | 0.973 | lift (strong) |
| `s2.ring_bearing_w_mm` | `g` | 66 | 34 | 0.991 | 0.987 [0.979, 0.992] | 0.969 | lift (strong) |
| `s1.ring_at_y` | `gc` | 155 | 7 | 0.971 | 0.986 [0.980, 0.990] | 0.932 | lift (strong) |
| `s1.sf_ring` | `g` | 155 | 7 | 0.984 | 0.986 [0.980, 0.989] | 0.958 | lift (strong) |
| `s1.ring_rob` | `g` | 155 | 7 | 0.988 | 0.986 [0.980, 0.989] | 0.958 | lift (strong) |
| `s1.ring20` | `g` | 155 | 7 | 0.708 | 0.985 [0.980, 0.989] | 0.957 | lift (strong) |
| `h.mass_g` | `g` | 10000 | 10 | 0.988 | 0.983 [0.983, 0.984] | 0.964 | lift (strong) |
| `h.relief_frac` | `g` | 10000 | 10 | 0.982 | 0.979 [0.978, 0.979] | 0.955 | lift (strong) |
| `h.ring_rob` | `g` | 10000 | 10 | 0.983 | 0.977 [0.976, 0.978] | 0.954 | lift (strong) |
| `h.hausdorff` | `g` | 10000 | 10 | 0.983 | 0.972 [0.971, 0.973] | 0.945 | lift (strong) |
| `s2.strict_hunter_sf` | `s` | 66 | 34 | 0.972 | 0.966 [0.944, 0.979] | 0.931 | lift (strong) |
| `s2.fat_sf` | `s` | 66 | 34 | 0.962 | 0.948 [0.916, 0.968] | 0.896 | lift (strong) |
| `s2.seq_ratio` | `s` | 66 | 34 | 0.961 | 0.946 [0.913, 0.967] | 0.886 | lift (strong) |
| `s1.screen_deficit` | `g` | 1409 | 46 | 0.968 | 0.933 [0.925, 0.939] | 0.857 | lift (strong) |
| `s1.seq_ratio` | `g` | 1556 | 48 | 0.941 | 0.910 [0.901, 0.918] | 0.825 | lift (strong) |
| `s1.fat_sf` | `g` | 1556 | 48 | 0.947 | 0.909 [0.900, 0.917] | 0.819 | lift (strong) |
| `s1.hm_ringback_sf` | `g` | 155 | 7 | 0.744 | 0.899 [0.863, 0.925] | 0.790 | lift (strong) |
| `s1.hm_ringback_n` | `g` | 155 | 7 | 0.994 | 0.898 [0.863, 0.925] | 0.789 | lift (strong) |
| `s1.hm_tip_inplane_sf` | `g` | 155 | 7 | 0.990 | 0.877 [0.835, 0.909] | 0.735 | lift (strong) |
| `s1.hm_tip_inplane_n` | `g` | 155 | 7 | 0.610 | 0.877 [0.835, 0.909] | 0.735 | lift (strong) |
| `s1.spg` | `g` | 155 | 7 | 0.689 | 0.872 [0.829, 0.906] | 0.743 | lift (strong) |
| `s1.sf_hunter` | `g` | 155 | 7 | 0.732 | 0.872 [0.828, 0.905] | 0.742 | lift (strong) |
| `s1.sf_hiker` | `g` | 155 | 7 | 0.986 | 0.872 [0.828, 0.905] | 0.742 | lift (strong) |
| `s1.lat_rob` | `g` | 155 | 7 | 0.985 | 0.871 [0.827, 0.905] | 0.742 | lift (strong) |
| `s1.hm_twist_sf` | `g` | 155 | 7 | 0.917 | 0.871 [0.827, 0.905] | 0.732 | lift (strong) |
| `s1.twist_sf` | `g` | 155 | 7 | 0.927 | 0.871 [0.827, 0.905] | 0.732 | lift (strong) |
| `s1.twist_tipzone_sf` | `g` | 155 | 7 | 0.917 | 0.871 [0.827, 0.905] | 0.732 | lift (strong) |
| `s1.hm_twist_nmm` | `g` | 155 | 7 | 0.930 | 0.871 [0.827, 0.905] | 0.732 | lift (strong) |
| `s1.twist_joint_nmm` | `g` | 155 | 7 | 0.931 | 0.871 [0.827, 0.905] | 0.732 | lift (strong) |
| `s1.twist_tipzone_joint_nmm` | `g` | 155 | 7 | 0.937 | 0.871 [0.827, 0.905] | 0.732 | lift (strong) |
| `s1.twist20_nmm` | `g` | 155 | 7 | 0.692 | 0.869 [0.824, 0.903] | 0.731 | lift (strong) |
| `s1.lat20` | `g` | 155 | 7 | 0.985 | 0.869 [0.824, 0.903] | 0.736 | lift (strong) |
| `h.sf_hunter` | `g` | 10000 | 10 | 0.851 | 0.834 [0.828, 0.840] | 0.692 | lift (strong) |
| `h.lat_rob` | `g` | 10000 | 10 | 0.851 | 0.834 [0.828, 0.840] | 0.691 | lift (strong) |
| `s1.pry20` | `g` | 155 | 7 | 0.989 | 0.833 [0.777, 0.875] | 0.664 | lift (strong) |
| `s1.couple20_nmm` | `g` | 155 | 7 | 0.901 | 0.827 [0.770, 0.871] | 0.667 | lift (strong) |
| `s1.twist_hand_joint_nmm` | `gc` | 155 | 7 | 0.906 | 0.804 [0.740, 0.853] | 0.578 | lift (strong) |
| `s1.twist_body_joint_nmm` | `g` | 155 | 7 | 0.935 | 0.785 [0.717, 0.839] | 0.595 | lift (strong) |
| `h.emd` | `g` | 10000 | 10 | 0.914 | 0.772 [0.764, 0.779] | 0.580 | lift (strong) |
| `s1.twist_body_sf` | `g` | 155 | 7 | 0.940 | 0.768 [0.695, 0.826] | 0.563 | lift (strong) |
| `h.spg` | `g` | 10000 | 10 | 0.805 | 0.757 [0.749, 0.766] | 0.549 | lift (strong) |
| `c.grip_len` | `g` | 196 | 6 | 0.987 | 0.749 [0.680, 0.805] | 0.363 | lift (moderate) |
| `c.mass_g` | `g` | 196 | 6 | 0.980 | 0.731 [0.658, 0.790] | 0.430 | lift (moderate) |
| `h.fit` | `g` | 10000 | 10 | 0.801 | 0.728 [0.719, 0.738] | 0.496 | lift (moderate) |
| `s1.hm_spine_n` | `g` | 155 | 7 | 0.986 | 0.719 [0.634, 0.788] | 0.480 | lift (moderate) |
| `s1.hm_spine_sf` | `g` | 155 | 7 | 0.817 | 0.719 [0.634, 0.788] | 0.480 | lift (moderate) |
| `c.ridge_min_halfwidth_mm` | `g` | 196 | 6 | 0.743 | 0.675 [0.591, 0.745] | 0.452 | lift (moderate) |
| `h.aes_score` | `g` | 10000 | 10 | 0.862 | 0.649 [0.638, 0.660] | 0.375 | lift (moderate) |
| `c.blade_sharpened_arc` | `g` | 196 | 6 | 0.816 | 0.638 [0.547, 0.714] | 0.384 | lift (moderate) |
| `c.sec_min_ioop_blade` | `g` | 196 | 6 | 0.848 | 0.601 [0.503, 0.683] | 0.360 | lift (moderate) |
| `s2.buckling_eigenvalue_a4` | `g` | 66 | 34 | 0.650 | 0.556 [0.363, 0.703] | 0.278 | lift (moderate) |
| `a.score` | `g` | 196 | 6 | 0.877 | 0.539 [0.431, 0.631] | 0.175 | lift (marginal) |
| `s1.twist_at_x` | `gc` | 155 | 7 | 0.484 | 0.515 [0.389, 0.622] | 0.259 | lift (moderate) |
| `s1.lat_at_y` | `gc` | 155 | 7 | 0.943 | 0.431 [0.293, 0.551] | 0.166 | lift (marginal) |
| `s1.lat_at_x` | `gc` | 155 | 7 | 0.782 | 0.380 [0.236, 0.507] | 0.136 | lift (marginal) |
| `a.hausdorff` | `g` | 196 | 6 | 0.968 | 0.340 [0.210, 0.458] | 0.079 | lift (marginal) |
| `s1.twist_kc` | `g` | 155 | 7 | 0.356 | 0.318 [0.168, 0.453] | 0.092 | lift (marginal) |
| `s1.twist_tipzone_kc` | `g` | 155 | 7 | 0.448 | 0.318 [0.168, 0.453] | 0.092 | lift (marginal) |
| `s1.twist_b_mm` | `g` | 155 | 7 | 0.363 | 0.245 [0.091, 0.387] | -0.020 | lift (marginal) |
| `c.blade_tip_to_heel` | `g` | 196 | 6 | 0.849 | 0.216 [0.078, 0.346] | 0.024 | lift (marginal) |
| `s1.twist_tipdist` | `g` | 155 | 7 | 0.293 | 0.203 [0.046, 0.349] | -0.025 | lift (marginal) |
| `s1.twist_body_kc` | `gc` | 155 | 7 | 0.798 | 0.116 [-0.043, 0.269] | -0.023 | weak |
| `s1.twist_t_mm` | `g` | 155 | 7 | 0.189 | 0.098 [-0.061, 0.251] | -0.071 | weak |
| `s1.twist_at_y` | `g` | 155 | 7 | 0.114 | 0.001 [-0.156, 0.159] | -0.109 | weak |

## 14. Verification

Built and tested on the pinned Rust 1.85.1 toolchain against `main` @ `49dc71f`; every producer was run on real data.

| Check | Result |
|---|---|
| `cargo build --release --locked -p jarambit` | clean; `Cargo.lock` unchanged |
| `cargo test --release -p jarambit` (incl. every existing `auto_evolve` test + `tests/evolve_pipeline.rs`) | 93 + 2 passed, 0 failed; the integration controls write no records into the repository |
| Dataset unit tests (codecs, statistics vs hand values, deterministic random and grouped folds, lineage-memorisation control, torn tails, genome-keyed joins, replicate averaging, recipes, leak guard, byte-identical rebuilds, `git commit --only` isolation, harvest joins, gene-list parity with `jarambit_crescent::genome`, stable feature columns, exact gene round-trip, real CatBoost CV finds lift on a learnable table and none on noise) | 27 passed in the crate; the 22 pure ones also pass in the standalone `rustc -D warnings` build |
| CI clippy gate `cargo clippy -p jarambit -- -A clippy::all -D clippy::disallowed_methods` | passes |
| `crates/runner-policy` on the new workflow | passes |
| Live `auto-evolve` round with `--dataset-commit` | 16 births recorded (8 screened, 8 gate-rejected); values match `round_00_screen.json`; the auto-commit held only the run directory while other edits stayed uncommitted |
| `rescreen` design of experiments (4 seeds x 45 neighbours, sigma 0.04) | 184 records, 498 columns each |
| `harvest-waves` over `evidence/evolution_waves` | 19 runs -> 3,194 births (1,873 screened) + 92 Stage-2 rows, 0 unmatched |
| `refeaturize` (threaded) | adds CAD features as attachments; deterministic; a second run is a no-op |
| `add-physical` | genome-file, hash and certificate-identity rows resolve; replicate specimens average; unknown designs reported (exit 1) |
| `build` over records + all 10 gen-800 archives | 160 tables, 25 specs skipped with reasons, 58 s |

Not run here: the Stage-2 Python solver stack (skfem/DOLFINx) and the jankurai auditor are not installed on the authoring machine; both run in CI. In CI, jankurai 1.7.5 scores this branch 48 with 66 findings, the same as `main` @ `49dc71f` (its first run flagged a local variable named `todo`, since renamed).

## 15. Runbook

```sh
cargo build --release --locked -p jarambit
./evolve.sh --rounds 4                                   # records + per-round commits
JARAMBIT_DATASET_PUSH=origin:data/regression-records ./evolve.sh --rounds 4
./target/release/jarambit dataset rescreen --genomes seeds.csv --neighbors 100 --threads 48 --commit
./target/release/jarambit dataset harvest-waves && ./target/release/jarambit dataset refeaturize --threads 48
./target/release/jarambit dataset add-physical --csv lab/2026-11.csv --campaign 2026-11-coupons
./target/release/jarambit dataset build && ./target/release/jarambit dataset cv --catboost ~/bin/catboost
```

## 16. Risks and next steps

1. **Fidelity drift.** Stage-1 code and env flags (e.g. `JARAMBIT_TIPBAND_BIAS`) change screen values
   for the same genome (100 conflicting cells across retained waves). Records keep the first value
   and the build reports conflicts; next step: stamp a screen-model digest per row and filter by it.
2. **Leakage through lineage.** Siblings share most genes; random k-fold is optimistic. Grouped
   k-fold now ships and drives the verdict; next, record a parent id for every GA birth so lineage
   groups get finer than run + family.
3. **Storage growth.** Move records to a data branch or LFS once they exceed a few hundred MB.
4. **B-rep export features** (`certification::measure`: apex, half-angle, point profile, bore edge
   radius) are expensive; add them as an opt-in attachment for Stage-2 candidates.
5. **Surrogate in the loop.** Once a table shows stable lift (e.g. `s1.lat_rob__gc`), its CatBoost
   model can pre-rank births before the FEM screen (never replacing any gate).
