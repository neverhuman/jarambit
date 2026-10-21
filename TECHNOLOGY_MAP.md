# TECHNOLOGY MAP — jarambit (v0.27.0-dev)

One table: technology role → where the code is → README. Crates keep their names (renaming breaks cargo/CI/live worktrees); top-level role dirs carry READMEs that point into `crates/`.

| role dir | crate(s) / code | what it owns | README |
|---|---|---|---|
| `genotype/` | `crates/genotype`, `crates/redteam`, `crates/crescent` (genome, gates, c4/c5/c6 refs) | encoding, validators, ring ≥ 25.4, blade ≥ 3.375 in ×3, connected solid, no stick-figure priors | `genotype/README.md`, `crates/genotype/README.md`, `crates/redteam/README.md` |
| `phenotype/` | `crates/phenotype`, `crates/crescent` (pheno, geom, export) | STEP/STL CAD, watertight, grinds, tip outline, stock | `phenotype/README.md`, `crates/phenotype/README.md` |
| `fem/` | `crates/physics`, `crates/crescent/src/fem.rs` | plate screen, solid tip submodel, lateral/ring/twist(1.82 N·m)/seq, mesh rules, 828 MPa | `fem/README.md`, `crates/physics/README.md` |
| `monte_carlo/` | `crates/monte_carlo` | tolerance cloud, −10…100 °C, heterogeneous media, p05 | `monte_carlo/README.md`, `crates/monte_carlo/README.md` |
| `evolve/` | `crates/evolve`, `crates/crescent/src/qd.rs` | NSGA-II islands, fitness, novelty, diversity/QD | `evolve/README.md`, `crates/evolve/README.md` |
| `metrics/` | `metrics/global_metric.py`, `crates/crescent/src/aesthetic.rs`, `ops/wave_figures/` | GLOBAL metric G, wave plates, WAVE_FIGURE_TEMPLATE | `metrics/README.md` |
| `docs/` | `docs/`, `crates/docs` | WHITEPAPER, VERSION, RELEASES, WAVE_LOG, GLOBAL_METRIC, calibration, holder | `docs/README.md`, `crates/docs/README.md` |
| `cli/` | `crates/cli` (binary `jarambit`) | entry points: evolve, crescent-evolve, crescent-d1, calib, fem-smoke | `cli/README.md`, `crates/cli/README.md` |
| `ops/` | `ops/ci`, `ops/wave_figures`, `ops/wave_reports`, launch scripts | CI smoke + version check, plate/report packers, multihost launch | `ops/README.md` |
| `datasets/regression/` | `crates/cli/src/datasets` (`jarambit dataset ...`; dependency-free core also builds with plain `rustc`) | regression micro-records written during evolution, deterministic tables (final column = target), CatBoost k-fold lift CI | `datasets/regression/README.md`, `docs/REGRESSION_DATASETS.md` |
| (lane) | `crates/crescent` | crescent/C.6 aesthetic lane: spans genotype+phenotype+fem+evolve | `crates/crescent/README.md` |

## Data flow
`genotype` (validate/reject) → `phenotype` (outline, STEP, mass) → `monte_carlo` × `fem` (capacities, SF, SPG, seq) → `evolve` (select) → `cli` writes `out/<run>` → `ops/wave_figures` plates → `metrics` G → `docs/WHITEPAPER.md` + `docs/WAVE_LOG.md`.

Hard gates: ring ID 25.4 ± 0.5 mm (≥ 25.4 nominal) · blade ≥ 3.375 in (85.725 mm) on tip-to-heel, horizontal reach and sharpened arc · sharpened edge ≤ 4.0 in · grip 60–80 mm · single connected solid · Ti only · 0.1 mm stock · no stick-figure priors (crescent C.6 anchor); FEM 828 MPa / 1.82 N·m / 1500 N / hiker 180 / hunter 260 / solid tip SF ≥ 1; seq ≥ 1.2 soft.
