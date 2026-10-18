# Master CAD Scorecard — `w320_r01_p14_c27`

- **Verdict**: **FAIL (Gate Violation)**
- **Master Weighted CAD Score (`W_CAD_v2`)**: **`-5.258458`** (unpenalized quality `+1.136591`)
- **Locked Cross-Wave Global Fitness (`G1`)**: **`0.424383 (COMPLETE)`**
- **Mass**: **`91.483 g`** (2.5D raster) / **`91.506 g`** (3D watertight solid, `0.025%` delta)
- **Evaluation Mode**: `genome_and_3d_cad_full_battery`
- **CAD Battery Summary**: `14/18 pass`, `1` verified 3D solver run

## 1. Master Weighted Score Breakdown (`W_CAD_v2_ultrarunner_karambit`)

| Component | Weight | Normalized Subscore `[0, 1]` | Weighted Contribution |
|---|---:|---:|---:|
| Base Feasibility Floor (`16/16` gates) | — | `0.000000` | `-1.000000` |
| Ultralight Mass (`92 g -> 74 g`) | `70%` | `0.027729` | `+0.019410` |
| Karambit Claw & Ring Authenticity (Rev C.6) | `12%` | `0.409842` | `+0.049181` |
| Bone-Contact Tip & Slash Survival (`+8%` knee) | `10%` | `0.000000` | `+0.000000` |
| Hand-Safe Yield Sequence (`seq >= 1.20`, `1.28` knee) | `5%` | `1.000000` | `+0.050000` |
| Vest Sheath Carry & Buckling Stability | `3%` | `0.600000` | `+0.018000` |
| Gate Shortfall Penalty (`-4.0 * sum(shortfalls)`) | — | — | `-4.261231` |
| **Total Master Weighted Score (`W_CAD`)** | **`100%`** | — | **`-5.258458`** |

## 2. All 16 Hard Engineering, CAD & Archetype Gates

| # | Gate ID | Measured Value | Requirement / Floor | Status |
|---:|---|---|---|---|
| 1 | `cad_watertight_and_single_solid` | solids=1, boundary=0, nonmanifold=0, delta=0.02% | 1 solid, 0 open/non-manifold edges, <=1% mass delta | **PASS** |
| 2 | `karambit_ring_and_curved_claw` | ring=24.90 mm, wall=5.32 mm, groove=2.12 mm, hook=41.9 mm, blade=3.436 in | ring 25.4±0.5 mm, wall >=4.5 mm, groove >=2.0 mm, hook >=25 mm, blade >=3.375 in | **PASS** |
| 3 | `c6_aesthetic_floor` | aes_score=0.4070, pass=True | SWITCHBACK Rev C.6 conformance pass | **PASS** |
| 4 | `geometric_and_holder_gates` | blade_min=3.436 in, teeth=15/14 | 0 geometry/holder rejects | **PASS** |
| 5 | `cert_hiker_180n` | 177.1 N (SF=0.984) | >= 180.0 N (SF >= 1.00) | **FAIL** |
| 6 | `cert_hunter_260n` | 177.1 N (SF=0.681) | >= 260.0 N at min(0.25, 0.125 mm^2) | **FAIL** |
| 7 | `strict_hunter_260n` | 178.8 N (SF=0.6877) | >= 260.0 N at strict 0.0625 mm^2 | **FAIL** |
| 8 | `cert_ring_pull_1500n` | 27877.2 N (SF=18.585) | >= 1500.0 N (SF >= 1.00) | **PASS** |
| 9 | `twist_body_1p82nm` | SF=2.603 | >= 1.00 at 1.82 N*m (>15 mm from tip) | **PASS** |
| 10 | `twist_tip_only_1p82nm` | SF=0.472 | >= 1.00 at 1.82 N*m (>=4 mm from tip) | **FAIL** |
| 11 | `twist_tip_submodel_strict_1p82nm` | SF=0.544 | >= 1.00 at 1.82 N*m (4-15 mm strict) | **FAIL** |
| 12 | `solid_tip_3d_p2_1p82nm` | SF_nodal_max=0.7030 (avg=0.7296) | >= 1.00 (3D P2 TET10 solid submodel) | **FAIL** |
| 13 | `dolfinx_cross_check_2pct` | rel_err=0.0000% | <= 2.00% vs FEniCSx/dolfinx v0.9.0 | **PASS** |
| 14 | `failure_sequence_1p2` | seq_ratio=1.785 (a4=6.504) | >= 1.20 (hand yields after blade) | **PASS** |
| 15 | `plate_gci_valid_3level` | 3-level GCI valid | 3-level Richardson sequence & observable valid | **PASS** |
| 16 | `side_a4_buckling_and_support` | lambda_cr=5.042 | lambda_cr >= 1.00 & Winkler+bore pass | **PASS** |

## 3. 18-Evaluator CAD Battery Matrix (`jarambit.cad_battery.v1`)

| Evaluator ID | Kind | Solver | Status | Evidence / Detail |
|---|---|---|---|---|
| `build.analytic` | `software_contract` | `analytic` | **`PASS`** | `crates/physics/tests/invariance.rs` |
| `build.gci_contracts` | `software_contract` | `gci` | **`PASS`** | `fem_solid/tests/test_gci.py` |
| `build.claims` | `software_contract` | `contract` | **`PASS`** | `ops/ci/check_claims.py` |
| `build.dolfinx` | `software_contract` | `solid_dolfinx` | **`PASS`** | `evidence/cad_battery/dolfinx_cross_check.json` |
| `geom.unit` | `gate` | `contract` | **`PASS`** | `/home/ubuntu/jarambit/out/w320/by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792327910595534301/w320_r01_p14_c27.step` |
| `geom.one_solid` | `gate` | `contract` | **`PASS`** | `/home/ubuntu/jarambit/out/w320/by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792327910595534301/w320_r01_p14_c27.step` |
| `geom.mass` | `derived` | `plate_cst_dkt` | **`PASS`** | `/home/ubuntu/jarambit/out/w320/by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792327910595534301/w320_r01_p14_c27.step` |
| `geom.pin` | `software_contract` | `contract` | **`PASS`** | `/home/ubuntu/jarambit/out/w320/by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792327910595534301/w320_r01_p14_c27.step` |
| `plate.loads` | `derived` | `plate_cst_dkt` | **`FAIL`** | `/home/ubuntu/jarambit/out/w320/by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792327910595534301/w320_r01_p14_c27_rs.json` |
| `plate.seq` | `gate` | `plate_cst_dkt` | **`PASS`** | `/home/ubuntu/jarambit/out/w320/by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792327910595534301/w320_r01_p14_c27_rs.json` |
| `plate.l7` | `derived` | `plate_cst_dkt` | **`FAIL`** | `/home/ubuntu/jarambit/out/w320/by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792327910595534301/w320_r01_p14_c27_rs.json` |
| `plate.gci` | `derived` | `plate_cst_dkt` | **`FAIL`** | `/home/ubuntu/jarambit/out/w320/by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792327910595534301/w320_r01_p14_c27_plate_gci.json` |
| `side.a1` | `derived` | `plate_cst_dkt` | **`PASS`** | `/home/ubuntu/jarambit/out/w320/by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792327910595534301/w320_r01_p14_c27_a1a3.json` |
| `side.a2` | `derived` | `plate_cst_dkt` | **`PASS`** | `/home/ubuntu/jarambit/out/w320/by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792327910595534301/w320_r01_p14_c27_a1a3.json` |
| `side.a3` | `derived` | `plate_cst_dkt` | **`PASS`** | `/home/ubuntu/jarambit/out/w320/by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792327910595534301/w320_r01_p14_c27_a1a3.json` |
| `side.a4` | `derived` | `plate_cst_dkt` | **`PASS`** | `/home/ubuntu/jarambit/out/w320/by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792327910595534301/w320_r01_p14_c27_a4.json` |
| `solid.tip` | `solver_run` | `solid_skfem_p2` | **`PASS`** | `out/w320/by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792327910595534301/w320_r01_p14_c27_solid_tip.json` |
| `gate.rs_v2` | `gate` | `contract` | **`FAIL`** | `/home/ubuntu/jarambit/out/w320/by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792327910595534301/w320_r01_p14_c27_rs.json` |
