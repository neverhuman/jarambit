# Regression datasets

Every evaluation the search performs is also a labelled example. This directory turns those
evaluations into tabular regression datasets — **features describe the design, the final column
is a test output** — and CI screens every table with CatBoost k-fold cross-validation to show
which ones carry predictive lift. Full design: [`docs/REGRESSION_DATASETS.md`](../../docs/REGRESSION_DATASETS.md).

```
datasets/regression/
  recipes.conf            which tables to build (sources, feature sets, targets, leak rules)
  records/                COMMITTED micro-records (append-only CSV shards, source of truth)
    auto_evolve/<run>/    written live by `jarambit auto-evolve` (one row per birth / assessment)
    rescreen/<run>/       Stage-1 FEM screens of genome files, mutation neighbours or random designs
    harvest_waves/<run>/  converted from retained evidence/evolution_waves outputs
    physical/<campaign>/  titanium part and coupon test results
    refeaturized/         CAD features added later to rows that only carried genes
  tables/                 DERIVED, git-ignored: <target>__<featureset>.csv (+ .card.json, index)
  cv/                     DERIVED, git-ignored: CatBoost CV results and leaderboard.{json,md}
```

## Quick start

```sh
cargo build --release --locked -p jarambit
./target/release/jarambit dataset build                       # records + gen-800 archives -> tables/
./target/release/jarambit dataset cv --catboost /path/to/catboost   # k-fold CV -> cv/leaderboard.md
```

`cv` needs the official CatBoost CLI binary (`catboost-linux-x86_64-1.2.10` from the CatBoost GitHub
release; CI pins its sha256) via `--catboost` or `JARAMBIT_CATBOOST`. The gen-800 source needs the
`zstd` CLI (or `JARAMBIT_ZSTD`). Without the full workspace, the same engine builds with plain rustc:

```sh
rustc --edition 2021 -O crates/cli/src/datasets/standalone.rs -o jarambit-datasets
./jarambit-datasets build && ./jarambit-datasets cv
```

## Producing records

| Producer | Command | Rows |
|---|---|---|
| Live GP | `./evolve.sh` (records on, commit after each round) or `jarambit auto-evolve --dataset-commit [--dataset-push origin:data/regression-records]` | every birth: genes + CAD features + all Stage-1 screen outputs; every Stage-2 assessment |
| Design of experiments | `jarambit dataset rescreen --genomes seeds.csv --neighbors 50 --sigma 0.04 --threads 48 --commit` | Stage-1 screens of mutation neighbours (or `--random N`) |
| Retained evidence | `jarambit dataset harvest-waves` then `jarambit dataset refeaturize` | births + screen scalars + Stage-2 leaderboards |
| Physical tests | `jarambit dataset add-physical --csv results.csv --campaign 2026-11-coupons` | `phys.*` targets joined to the design |

Disable live recording with `--no-dataset` or `JARAMBIT_DATASET=0`. Recording never changes search,
ranking or certification.

## Column families

| prefix | meaning | role |
|---|---|---|
| `meta.` | provenance (run, round, label, certificate identity, genome hash, status) | never a feature |
| `g.` | the 128 genes + material flag, exact round-trip values | feature |
| `gd.` | genome-derived: silhouette zones, Fourier terms, roughness, architecture switches, knots | feature (computed at build time) |
| `c.` | CAD: blade lengths, ring, mass, thickness statistics, mass moments, 12 sections, 12 polar sectors, hole ligaments, hull, distance-transform ligaments | feature |
| `a.` | C.6 aesthetic metrics + 32-bin curvature histogram | feature |
| `gate.` | hard-gate outcomes, holder fit, ring-bearing geometry | feature |
| `desc.` | 16-D QD descriptor | feature |
| `s1.` | Stage-1 FEM screen outputs (lateral, ring, twist body/tip, human-max, fatigue, peak locations ...) | target (feature only in `gcs`/`s` sets) |
| `s2.` | Stage-2 independent assessment outputs | target |
| `h.` | gen-800 archive outputs | target |
| `phys.` | physical test results | target |

Tables are named `<target>__<featureset>` (`s1.lat_rob__gc.csv`). The **last column is always the
target**; rows are ordered by genome hash, so the same inputs always produce byte-identical tables.
Each table has a `<name>.groups` sidecar (one lineage key per row: DOE parent, GA run + family, or
gen-800 run). `cv` reports random k-fold r and lineage-grouped k-fold r; the lift verdict uses the
grouped one whenever a table has ≥ 3 lineage groups.
