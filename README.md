# Jarambit 6.0.0: an evolved titanium karambit for laser powder-bed fusion

[![release](https://img.shields.io/badge/release-v6.0.0-2f56d6)](https://github.com/neverhuman/jarambit/releases/tag/v6.0.0) [![paper](https://img.shields.io/badge/paper-G5%20v6.0.0%20PDF-2f56d6)](docs/g5/paper/g5-v6.0.0.pdf) [![3D print CAD](https://img.shields.io/badge/3D%20print%20CAD-cad%2F%20(STL%20%7C%203MF%20%7C%20DXF%20%7C%20PDF)-2f56d6)](cad/) ![design](https://img.shields.io/badge/best%20individual-kv4i1404__g00581__29ea3c73-555) ![score](https://img.shields.io/badge/W__ERGOLIGHT__v2-74.40-555) ![mass](https://img.shields.io/badge/mass-88.4%20g-555) ![status](https://img.shields.io/badge/status-computational%2C%20not%20built-b4570f)

Every computed number and picture on this page describes one design, **`kv4i1404_g00581_29ea3c73`**, the best individual of the G5 v6.0.0 cohort of record. They are generated from [`evidence/g5/v6.0.0/release.json`](evidence/g5/v6.0.0/release.json), its verification studies and the sourced requirement set [`requirements/RS-karambit-v5.toml`](requirements/RS-karambit-v5.toml) ([provenance](requirements/PROVENANCE-v5.md)). Status: computational design; not built, printed or physically tested.

**Paper:** [`docs/g5/paper/g5-v6.0.0.pdf`](docs/g5/paper/g5-v6.0.0.pdf), generated from the same receipt. **3D printing:** the upload kit in [`cad/`](cad/).

<img src="docs/g5/v6.0.0/img/hero.jpg" alt="The best individual kv4i1404_g00581_29ea3c73" width="100%">

## The best individual

Titanium (Ti-6Al-4V, L-PBF, stress-relieved): a 75.9 mm grip of 99.4 mm perimeter with a 24.9 mm finger ring, a hollow core of sheet-TPMS lattice with powder ports, and a double-edged claw blade with serrations on both edges (6 and 5 teeth). It meets every RS-karambit-v5 load case: the lowest margin is tip axial, 1.094 (the minimum over the plate meshes 0.6 / 0.3 / 0.15 mm²).

| Measure | Value | Requirement (RS-karambit-v5) | Reading |
|---|---|---|---|
| W_ERGOLIGHT_v2, Tier 2 at h 0.2 mm (rank 1 of 64) | **74.40** | higher is better | hand multiplier E 0.922 (raised to the 5th power), worst-margin multiplier W 1.000 |
| Mass of record (partial volume at h 0.2 mm, × 0.00443 g/mm³) | **88.4 g** | ceiling 130 g; 50 g goal (a frontier, not a requirement) | meets target; watertight print mesh 90.3 g |
| Grip length / perimeter / height-to-thickness | 75.9 mm / 99.4 mm / 1.63 | ≥ 69 mm / 84–100 mm / 1.5–1.8 | meets target |
| Guard height / centre of mass to the index finger | 15.8 mm / 7.3 mm | ≥ 15 mm / ≤ 8 mm | meets target / meets target |
| Point: included angle at 1 mm / at 3 mm; width at 2 mm | 65.5° / 60.4°; 2.05 mm | ≤ 75° / ≤ 60°; ≤ 3 mm | meets target / **above the limit**; meets target (a continuous penalty, not a gate) |
| Edges A (inner) / B (spine): per-side grind angle | 22.3° / 23.5° | 20–25° | largest measured half-angle 23.5°, apex at most 0.113 mm thick |
| Serrations A / B: depth, teeth | 1.09 mm, 6 / 1.30 mm, 5 | 0.8–1.5 mm; gullet radius ≥ 0.5 mm by construction | weakest tooth SF 2.38 / 3.22 |
| Hand comfort factors (1 = within every limit; E is their weighted product) | contact pressure 0.90 | NASA/MIL hand limits (contact pressure, edge radii, grip section) | the other factors are 1.00 |
| Powder: sealed cavity / thin crevices (printed solid) | 45 mm³ / 200 mm³ | no closed voids; drainage passages ≥ 1 mm | sealed volume is penalised by the score; 4 ports of 3.9 mm (≥ 3 mm) |

Every RS-karambit-v5 load case, as a safety factor (SF) against its requirement. The margin is v = SF / requirement; the smallest margin sets the worst-margin multiplier W once it falls below 1. SF is on the production plate mesh (0.3 mm²), as scored; the margin of record is the minimum over the plate meshes 0.6 / 0.3 / 0.15 mm² (cases not solved on the plate are mesh-independent):

| Load case | SF | Requirement | Margin v (score mesh) | Margin v of record (min over meshes) |
|---|---:|---:|---:|---:|
| Lateral bending, limit: 12 N·m root moment, load 5 mm from the point (blade plate, grip) | 2.01 | ≥ 1.2 | 1.678 | 1.636 |
| Lateral bending, ultimate: 18 N·m (blade plate) | 1.47 | ≥ 1.3 | 1.132 | 1.104 |
| Body twist, limit: 16.2 N·m (blade plate, grip) | 1.90 | ≥ 1.2 | 1.583 | 1.583 |
| Tip, axial: 150 N along the tip bisector (TET10 solid) | 1.31 | ≥ 1.2 | 1.094 | 1.094 |
| Tip, side load: 10 N at the point (TET10 solid) | 1.86 | ≥ 1.2 | 1.550 | 1.550 |
| Tip, twist: 0.3 N·m, 3 mm from the point (TET10 solid) | 2.50 | ≥ 1.2 | 2.083 | 2.083 |
| Grip squeeze: 800 N | 2.32 | ≥ 1.2 | 1.933 | 1.933 |
| Guard push: 300 N | 26.39 | ≥ 1.2 | 21.993 | 21.993 |
| Ring bearing: 300 N on the handle side of the bore (T6 ring) | 65.39 | ≥ 2 | 32.694 | 32.694 |
| Ring back pull: 30 N (T6 ring) | 68.46 | ≥ 2 | 34.232 | 34.232 |
| Ring, any in-plane direction: 300 N (T6 ring sweep) | 3.49 | ≥ 1.2 | 2.910 | 2.910 |
| Axial pull: 600 N, held over the 20 mm nearest the point | 1.83 | ≥ 1.2 | 1.523 | 1.399 |
| Serration teeth: 180 N snag, 60 N pry shared by 3 teeth (every tooth) | 2.38 | ≥ 1.2 | 1.983 | 1.983 |
| Fatigue: 1,000,000 cycles, FKM mean stress (blade and hand zone) | 1.17 | ≥ 1 | 1.171 | 1.171 |
| Yield sequence: the blade yields before the hand zone, per load case (lateral / twist / pull) | 1.91 | ≥ 1.2 | 1.592 | 1.592 |
| Tip buckling under the 150 N axial load (Rayleigh–Ritz eigenvalue λ) | 11.94 | ≥ 1.5 | 7.958 | 7.958 |

<table><tr><td width="50%"><img src="docs/g5/v6.0.0/img/side.jpg" alt="Plan view"><br>Plan view.</td><td width="50%"><img src="docs/g5/v6.0.0/img/top.jpg" alt="Edge-on view"><br>Edge-on: ring, grip and blade thickness.</td></tr><tr><td><img src="docs/g5/v6.0.0/img/blade.jpg" alt="The blade"><br>The blade, reach 72.2 mm.</td><td><img src="docs/g5/v6.0.0/img/tip.jpg" alt="The point"><br>The point: 66° at 1 mm.</td></tr><tr><td><img src="docs/g5/v6.0.0/img/edge_a.jpg" alt="Inner edge"><br>Inner edge (A), 6 teeth, as decoded (before grinding).</td><td><img src="docs/g5/v6.0.0/img/edge_b.jpg" alt="Spine edge"><br>Spine edge (B), 5 teeth, as decoded (before grinding).</td></tr><tr><td><img src="docs/g5/v6.0.0/img/grip.jpg" alt="Grip"><br>Grip, 99.4 mm perimeter.</td><td><img src="docs/g5/v6.0.0/img/ring.jpg" alt="Ring"><br>Ring, 24.9 mm bore.</td></tr><tr><td><img src="docs/g5/v6.0.0/img/xray.jpg" alt="X-ray of the internal cavities"><br>X-ray of the hollow core.</td><td><img src="docs/g5/v6.0.0/img/cut_full.jpg" alt="Lengthwise cut"><br>Lengthwise cut: skin, lattice and ports.</td></tr><tr><td><img src="docs/g5/v6.0.0/img/cut_x.jpg" alt="Section across the grip"><br>Section across the grip.</td><td><img src="docs/g5/v6.0.0/img/cut_b.jpg" alt="Section across the blade"><br>Section across the blade.</td></tr><tr><td><img src="docs/g5/v6.0.0/img/tip_edge.jpg" alt="Edge at the point"><br>The edge at the point, as decoded (before grinding).</td><td></td></tr></table>

Edge sections at four stations per edge, true scale, from the exact decoder law (dashed: 20° per side, the smallest angle the requirement allows):

<picture><source media="(prefers-color-scheme: dark)" srcset="docs/g5/v6.0.0/img/edge_sections_dark.png"><img src="docs/g5/v6.0.0/img/edge_sections_light.png" alt="Blade cross-sections of both edges"></picture>

## Where the score comes from

The score is 100 · E · W · base: the weighted credit of 22 terms (base, at most 1), times the hand multiplier E and the worst-margin multiplier W. Of its 100 points the winner earns **74.4**: structural safety 45.3 (the 16 load-case margins), mass 11.0, cutting geometry 11.0 (edge, point and serrations) and build and form 7.1. It loses 6.3 to the multipliers (E = 0.922; the lowest hand factor is contact pressure, e = 0.897; W = 1.000), and 19.3 are unmet credit: terms short of full credit (base = 0.807).

<img src="docs/g5/v6.0.0/img/fitness_budget.svg" alt="The 100 points of the score: earned by group, lost to the multipliers, unmet credit" width="100%">

<img src="docs/g5/v6.0.0/img/fitness_terms.svg" alt="Points earned by each score term against the most it could earn" width="100%">

## FEM renderings of every load case

Each load case of `kv4i1404_g00581_29ea3c73` re-solved by `g5-fem-fields` on the ranking grid (h 0.2 mm), with the field the scored solvers discard: von Mises stress, the governing point and the deformed shape. The 28 scored values these solves produce match the receipt bit for bit.

<table>
<tr><td width="50%"><img src="docs/g5/v6.0.0/img/fem/lateral_limit.svg" alt="Lateral bending, limit load" width="100%"><br>Lateral bending, limit load: SF 2.01</td><td width="50%"><img src="docs/g5/v6.0.0/img/fem/lateral_ultimate.svg" alt="Lateral bending, ultimate load" width="100%"><br>Lateral bending, ultimate load: SF 1.47</td></tr>
<tr><td width="50%"><img src="docs/g5/v6.0.0/img/fem/twist_limit.svg" alt="Body twist, limit load" width="100%"><br>Body twist, limit load: SF 1.90</td><td width="50%"><img src="docs/g5/v6.0.0/img/fem/pull.svg" alt="Pull" width="100%"><br>Pull: SF 1.83</td></tr>
<tr><td width="50%"><img src="docs/g5/v6.0.0/img/fem/fatigue.svg" alt="Fatigue, fully reversed duty" width="100%"><br>Fatigue, fully reversed duty: SF 1.17</td><td width="50%"><img src="docs/g5/v6.0.0/img/fem/tip_axial.svg" alt="Point, axial" width="100%"><br>Point, axial: SF 1.31</td></tr>
<tr><td width="50%"><img src="docs/g5/v6.0.0/img/fem/tip_lateral.svg" alt="Point, side load" width="100%"><br>Point, side load: SF 1.86</td><td width="50%"><img src="docs/g5/v6.0.0/img/fem/tip_twist.svg" alt="Point, twist" width="100%"><br>Point, twist: SF 2.50</td></tr>
<tr><td width="50%"><img src="docs/g5/v6.0.0/img/fem/ring_bearing.svg" alt="Ring, bearing toward the handle" width="100%"><br>Ring, bearing toward the handle: SF 65.39</td><td width="50%"><img src="docs/g5/v6.0.0/img/fem/ring_back.svg" alt="Ring, back" width="100%"><br>Ring, back: SF 68.46</td></tr>
<tr><td width="50%"><img src="docs/g5/v6.0.0/img/fem/ring_any_direction.svg" alt="Ring, any in-plane direction" width="100%"><br>Ring, any in-plane direction: SF 3.49</td><td width="50%"><img src="docs/g5/v6.0.0/img/fem/sequence.svg" alt="Blade before hand (failure sequence)" width="100%"><br>Blade before hand (failure sequence): SF 1.91</td></tr>
<tr><td width="50%"><img src="docs/g5/v6.0.0/img/fem/buckling.svg" alt="Buckling of the point" width="100%"><br>Buckling of the point: SF 11.94</td><td width="50%"><img src="docs/g5/v6.0.0/img/fem/grip_squeeze.svg" alt="Grip squeeze" width="100%"><br>Grip squeeze: hex8 FE SF 23.78 (score model 2.32)</td></tr>
<tr><td width="50%"><img src="docs/g5/v6.0.0/img/fem/guard_push.svg" alt="Guard push" width="100%"><br>Guard push: hex8 FE SF 8.85 (score model 26.39)</td><td width="50%"><img src="docs/g5/v6.0.0/img/fem/teeth.svg" alt="Teeth" width="100%"><br>Teeth: hex8 FE SF 2.72 (score model 2.38)</td></tr>
</table>

The score models grip squeeze, guard push and teeth analytically. The hex8 voxel FE of the same design is an independent check, not used by the score:

| Case | Score model SF | Hex8 FE SF |
|---|---:|---:|
| Grip squeeze | 2.32 | 23.78 |
| Guard push | 26.39 | 8.85 |
| Teeth | 2.38 | 2.72 |

## How evolution produced it

v6.0.0 continues the v5.0.0 cohort with the same frozen decoder (G5.0, `c63629d4b310`) and objective, on one platform of record (linux/amd64: static x86_64 binaries, native on Linux and in a Rosetta container on macOS; Tier 2 at h 0.2 mm). The 1,508 final elites of v5.0.0 were re-evaluated on it and merged to the best record per niche: 131 niches, best 63.4 on the search grid (Tier 1, h 0.4 mm).

From that pool, 247 warm-started islands ran 20 rounds on 2 hosts (xbabe3: 7, xbabe4: 13), 296,400 evaluations in all. Whenever a host finished a round, the hub merged its islands into the latest pool and the host's next round started from that pool; every merge is recorded with its sources and recomputed by the receipt. Beside them, 4 fresh-start control islands ran 4,800 evaluations each from scrambled Sobol samples, with no pool and no migration.

**The lineage's best rose from 63.4 to 75.5; the best control island reached 59.6.**

<img src="docs/g5/v6.0.0/img/evolution.svg" alt="Best score found against evaluations: the pool lineage and the fresh-start control" width="100%">

The best design of the pool at points of the lineage, same camera and light (scores on the search grid):

<table><tr><td align="center"><img src="docs/g5/v6.0.0/img/lineage_epoch00_pool.jpg" alt="best of epoch00/pool" width="100%"><br><code>epoch00/pool</code><br>63.4, 104 g<br><b>warm start</b></td><td align="center"><img src="docs/g5/v6.0.0/img/lineage_p04.jpg" alt="best of pools/p04" width="100%"><br><code>pools/p04</code><br>67.8, 96 g</td><td align="center"><img src="docs/g5/v6.0.0/img/lineage_p10.jpg" alt="best of pools/p10" width="100%"><br><code>pools/p10</code><br>71.9, 96 g</td><td align="center"><img src="docs/g5/v6.0.0/img/lineage_p14.jpg" alt="best of pools/p14" width="100%"><br><code>pools/p14</code><br>72.8, 91 g</td><td align="center"><img src="docs/g5/v6.0.0/img/lineage_p20.jpg" alt="best of pools/p20" width="100%"><br><code>pools/p20</code><br>75.5, 89 g<br><b>final pool</b></td></tr></table>

| Host | Round | Islands × evaluations | Started from (best) | Merged into (best) |
|---|---:|---:|---|---|
| xbabe3 | 1 | 12 × 1,200 | `epoch00/pool` (63.36) | `pools/p02` (66.12) |
| xbabe3 | 2 | 12 × 1,200 | `pools/p02` (66.12) | `pools/p05` (68.72) |
| xbabe3 | 3 | 12 × 1,200 | `pools/p05` (68.72) | `pools/p08` (69.96) |
| xbabe3 | 4 | 12 × 1,200 | `pools/p08` (69.96) | `pools/p11` (71.94) |
| xbabe3 | 5 | 12 × 1,200 | `pools/p11` (71.94) | `pools/p14` (72.81) |
| xbabe3 | 6 | 12 × 1,200 | `pools/p14` (72.81) | `pools/p17` (74.57) |
| xbabe3 | 7 | 13 × 1,200 | `pools/p17` (74.57) | `pools/p20` (75.54) |
| xbabe4 | 1 | 12 × 1,200 | `epoch00/pool` (63.36) | `pools/p01` (65.56) |
| xbabe4 | 2 | 12 × 1,200 | `pools/p01` (65.56) | `pools/p03` (66.85) |
| xbabe4 | 3 | 12 × 1,200 | `pools/p03` (66.85) | `pools/p04` (67.81) |
| xbabe4 | 4 | 12 × 1,200 | `pools/p04` (67.81) | `pools/p06` (68.79) |
| xbabe4 | 5 | 12 × 1,200 | `pools/p06` (68.79) | `pools/p07` (69.85) |
| xbabe4 | 6 | 12 × 1,200 | `pools/p07` (69.85) | `pools/p09` (71.58) |
| xbabe4 | 7 | 12 × 1,200 | `pools/p09` (71.58) | `pools/p10` (71.94) |
| xbabe4 | 8 | 12 × 1,200 | `pools/p10` (71.94) | `pools/p12` (72.23) |
| xbabe4 | 10 | 12 × 1,200 | `pools/p12` (72.23) | `pools/p13` (72.81) |
| xbabe4 | 11 | 12 × 1,200 | `pools/p13` (72.81) | `pools/p15` (73.83) |
| xbabe4 | 12 | 14 × 1,200 | `pools/p15` (73.83) | `pools/p16` (74.57) |
| xbabe4 | 13 | 14 × 1,200 | `pools/p16` (74.57) | `pools/p18` (75.15) |
| xbabe4 | 14 | 14 × 1,200 | `pools/p18` (75.15) | `pools/p19` (75.54) |

Round numbers skip 9, which the control islands use (islands 9xx).

**Why the search leader is not the winner.** The best design on the search grid, `kv4i1406_g00009_528e4365` (75.5), ranks 2 once re-scored at Tier 2, h 0.2 mm. Every finalist is judged at Tier 2 (solid tip at 0.08 mm²); the top 16 also on five grids (partial-volume mass in grams at h = 0.4 / 0.3 / 0.25 / 0.2 / 0.15 mm; the h 0.2 and h 0.15 mm masses must agree within 2 %):

| Rank | Design | Score | E | W | Mass (g) | Mass at the five grids (g) | h 0.2 vs 0.15 |
|---:|---|---:|---:|---:|---:|---|---:|
| 1 | `kv4i1404_g00581_29ea3c73` | 74.40 | 0.922 | 1.000 | 88.4 | 88.6 / 89.5 / 89.2 / 88.4 / 88.6 | 0.17 % |
| 2 | `kv4i1406_g00009_528e4365` | 74.32 | 0.927 | 1.000 | 88.3 | 88.6 / 89.0 / 88.5 / 88.3 / 88.3 | 0.05 % |
| 3 | `kv4i1408_g00735_e6796cbf` | 74.18 | 0.928 | 1.000 | 88.9 | 89.1 / 89.5 / 89.1 / 88.9 / 88.9 | 0.00 % |
| 4 | `kv4i1405_g00335_c923c290` | 74.06 | 0.928 | 1.000 | 88.4 | 88.8 / 89.0 / 88.4 / 88.4 / 88.1 | 0.31 % |
| 5 | `kv4i1414_g00435_0e2e651b` | 74.04 | 0.931 | 1.000 | 89.5 | 89.8 / 90.1 / 90.2 / 89.5 / 89.5 | 0.01 % |
| 6 | `kv4i1410_g00279_290be921` | 74.04 | 0.926 | 1.000 | 89.0 | 89.3 / 89.7 / 89.2 / 89.0 / 88.9 | 0.10 % |
| 7 | `kv4i1401_g00149_412d5153` | 74.03 | 0.926 | 1.000 | 88.6 | 88.9 / 89.4 / 88.7 / 88.6 / 88.5 | 0.06 % |
| 8 | `kv4i1409_g00336_4a44463a` | 74.02 | 0.931 | 1.000 | 89.0 | 89.3 / 89.8 / 89.1 / 89.0 / 88.9 | 0.13 % |
| 9 | `kv4i1401_g00281_b7d5a90f` | 73.99 | 0.928 | 1.000 | 89.1 | 89.6 / 89.9 / 89.7 / 89.1 / 89.1 | 0.08 % |
| 10 | `kv4i1409_g00504_749c79d8` | 73.80 | 0.930 | 1.000 | 88.4 | 88.7 / 89.2 / 88.5 / 88.4 / 88.3 | 0.14 % |
| 11 | `kv4i1414_g00999_42619e63` | 73.62 | 0.924 | 1.000 | 90.4 | 90.9 / 91.1 / 90.5 / 90.4 / 90.3 | 0.07 % |
| 12 | `kv4i1410_g00280_5831a5e3` | 73.62 | 0.929 | 1.000 | 90.0 | 90.3 / 90.8 / 90.1 / 90.0 / 89.9 | 0.10 % |
| 13 | `kv4i1405_g00205_a7f055b6` | 72.90 | 0.924 | 1.000 | 90.3 | 90.3 / 90.8 / 90.3 / 90.3 / 90.2 | 0.15 % |
| 14 | `kv4i1411_g00490_71efc13e` | 72.83 | 0.931 | 1.000 | 90.0 | 90.3 / 90.7 / 90.0 / 90.0 / 89.8 | 0.30 % |
| 15 | `kv4i1310_g00526_5127c223` | 71.78 | 0.904 | 1.000 | 90.5 | 90.9 / 91.5 / 90.7 / 90.5 / 91.0 | 0.51 % |
| 16 | `kv4i1414_g00960_db1fd3ec` | 71.53 | 0.911 | 0.977 | 88.6 | 88.9 / 89.3 / 88.7 / 88.6 / 88.5 | 0.05 % |


## Verification and limits

- **One platform of record.** linux/amd64: static x86_64 binaries, native on Linux and in a Rosetta container on macOS; Tier 2 at h 0.2 mm. Every row of every archive re-scores bit for bit; every pool of the lineage is recomputed from its recorded sources byte for byte; re-evaluating all 282 elites of the final pool reproduces every Tier-1 row (172); and `epoch01/xbabe4/island101` re-run on xbabe2 reproduces its history bit for bit (17 entries over 200 evaluations).
- **Fleet sizing** ([`fleet/sizing.json`](evidence/g5/v6.0.0/fleet/sizing.json)). The continuation ran one island per core complex on the Threadripper hosts, following an early pilot. A controlled measurement afterwards (12 Tier-1 evaluations: 5.6 s each at one per core complex vs 7.7 s at four) shows that one island per core would have given 2.89× the throughput. The search is deterministic either way, so this cost evaluations, not correctness; glibc malloc tunables cut minor page faults 2.5× with identical outputs.
- **GPUs: renders, not search.** A profile of the Tier-1 evaluation ([`spike/spike.json`](evidence/g5/v6.0.0/spike/spike.json)) puts 39 % of it in voxel kernels and 20 % in the plate solve. Records must stay exact CPU arithmetic, so a GPU could only screen children before Tier 1. A perfect screen costing nothing would give at most 3.1× more evaluations per hour, and the Tier-0 screen already keeps 17 of the 20 best children (rank correlation 0.90). A real predictor costs time and misses designs, so the 3× bar for adopting it is out of reach and the search stays on the CPUs.
- **Plate mesh sensitivity** ([`vv/plate_mesh.json`](evidence/g5/v5.0.0/vv/plate_mesh.json)). Across plate meshes of 1.2 / 0.6 / 0.3 / 0.15 mm² the lateral safety factor of the study designs varies by up to 20 % (not monotone) and the twist safety factor by up to 6 %; the search and ranking use 0.3 mm². So the winner was chosen on the minimum over 0.6 / 0.3 / 0.15 mm² ([`plate_meshes.jsonl`](evidence/g5/v6.0.0/plate_meshes.jsonl), solved on the platform of record, whose 0.3 mm² rows reproduce the ranking bit for bit): in rank order, the first design meeting every load case on every mesh. It is the top-ranked design; its lowest margin over the meshes is tip axial 1.094.
- **Numerical studies** ([`evidence/g5/v5.0.0/vv/`](evidence/g5/v5.0.0/vv/)): grid agreement of mass, blade-plate and solid-tip mesh studies, the grip perimeter within 0.12 % of its gene, Rayleigh–Ritz against Euler within 5e-09, and Lloyd niches with 12.5 % less quantisation error than Halton.
- **Texture on the blade.** The frozen G5.0 decoder lets the evolved surface texture reach the blade, where its depth is the texture depth times the blade scope (0.22 mm × 0.07 = 0.014 mm here), so the blade and its bevels are nearly smooth. The edge figures and measurements use the exact bevel law, which grinding after printing restores: the requirement's finishing allowance (0.5 mm per ground face) exceeds the texture depth. Keeping texture off the bevels by construction is a decoder change for a later generation.
- **Status: computational design; not built, printed or physically tested.** Nothing here is a qualification for use. The safety factors are computed from models with the limits stated in the paper; the material values are specification minima, not measured coupons.
- **Evidence gaps** listed in [`requirements/PROVENANCE-v5.md`](requirements/PROVENANCE-v5.md) need physical tests before any build: tooth loads, tip load shares, edge retention, thin-wall roughness, lattice homogenisation, the temperature derate and the drop test.

## 3D printing and CAD (`cad/`, millimetres)

The upload kit in [`cad/`](cad/) holds six files for `kv4i1404_g00581_29ea3c73`, each under 50 MiB, generated from the winner's genome row ([`evidence/g5/v6.0.0/winner_genome.csv`](evidence/g5/v6.0.0/winner_genome.csv)). Order notes are in [`cad/README.md`](cad/README.md).

| File | What it is |
|---|---|
| [`cad/kv4i1404_g00581_29ea3c73_AMbuild_mm.stl`](cad/kv4i1404_g00581_29ea3c73_AMbuild_mm.stl) | print file: skin, sheet-TPMS lattice and powder ports, 0.525 mm voxels, 1,030,568 triangles |
| [`cad/kv4i1404_g00581_29ea3c73_AMbuild_mm.3mf`](cad/kv4i1404_g00581_29ea3c73_AMbuild_mm.3mf) | the same print mesh as 3MF, with millimetre units |
| [`cad/kv4i1404_g00581_29ea3c73_envelope_mm.stl`](cad/kv4i1404_g00581_29ea3c73_envelope_mm.stl) | outer envelope, 0.325 mm voxels |
| [`cad/kv4i1404_g00581_29ea3c73_drawing_mm.dxf`](cad/kv4i1404_g00581_29ea3c73_drawing_mm.dxf) | 1:1 plan drawing (DXF, mm) |
| [`cad/kv4i1404_g00581_29ea3c73_spec_paper.pdf`](cad/kv4i1404_g00581_29ea3c73_spec_paper.pdf) | the G5 paper |
| [`cad/kv4i1404_g00581_29ea3c73_xray_cutaway.jpg`](cad/kv4i1404_g00581_29ea3c73_xray_cutaway.jpg) | cutaway of the hollow core |

Finer meshes and the full evaluation records are release assets of [v6.0.0](https://github.com/neverhuman/jarambit/releases/tag/v6.0.0), each with its SHA-256 in the receipt:

| Asset | Size | SHA-256 |
|---|---:|---|
| `g5-v6.0.0-archives.tar.gz` | 167.2 MiB | `fe1651beaa485639…` |
| `g5-v6.0.0-eval_h040.jsonl` | 1.3 MiB | `64027768be0aea9d…` |
| `g5-v6.0.0-kv4i1404_g00581_29ea3c73_AMbuild_h0.2_mm.stl` | 361.3 MiB | `7054688acce5fd7e…` |
| `g5-v6.0.0-kv4i1404_g00581_29ea3c73_envelope_h0.2_mm.stl` | 116.9 MiB | `26987a58000432d0…` |

## Reproduce

```bash
# static x86_64 binaries, as on the platform of record (RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --locked --target x86_64-unknown-linux-gnu (Rust 1.85.1))
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --locked --target x86_64-unknown-linux-gnu --manifest-path crates/g5/Cargo.toml --bins
B=crates/g5/target/x86_64-unknown-linux-gnu/release
# one island: the arguments of any evidence/g5/v6.0.0/launch/<round>_<host>_<island>.json
$B/g5-continue run --out ~/jarambit_runs/evo_g5_v6/<round>/<host>/<island> --pool <pool> --evals <evals> --seed <raw_seed> \
  --host <host> --island <island> --commit bf3e37fa --h 0.4 --tier1-top 0.4 --tier2-top 0 --log-every 25
# a pool of the lineage: the merge of its recorded sources, in order (evidence/g5/v6.0.0/pools/*.json)
$B/g5-continue merge --out <pool> --from <previous pool>,<island>,<island>,...
$B/g5-pilot eval <final_root> eval_h040.jsonl 0.4 4 && $B/g5-pilot tier2 <final_root> eval_h040.jsonl 64 tier2_h020.jsonl 0.2 4
$B/g5-release receipt-v6 . <run_root> <cert_dir>
```

Every island's launch record and history, every pool's sources and the final pool are in [`evidence/g5/v6.0.0/`](evidence/g5/v6.0.0/); the islands' archives are a release asset.

## Repository

- [`crates/g5/`](crates/g5/README.md): the G5 genome (G5.0), decoder, materials, load models, objective, search and release tools.
- [`crates/g51/`](crates/g51/README.md): decoder G5.1 of the coming v7.0.0, in development (no results; not frozen): surface texture kept off the ground bevels, and the hand-zone cases scored on the solid under the pre-registered [calibration protocol](docs/g5/v7.0.0/calibration_protocol.md).
- [`requirements/`](requirements/RS-karambit-v5.toml): RS-karambit-v5 and its [provenance](requirements/PROVENANCE-v5.md).
- [`evidence/g5/v6.0.0/`](evidence/g5/v6.0.0/): the receipt, the winner's genome, every island's history, the ranking, the grid check and `vv/`.
- [`docs/g5/`](docs/g5/): the paper, the images and the CAD archive. [`docs/RELEASES.md`](docs/RELEASES.md): every release.
- Earlier generations, kept as their own record: G3 ([`docs/g3/`](docs/g3/), [`crates/g3/`](crates/g3/README.md)), the withdrawn G4 prototype ([`crates/g4/WITHDRAWN.md`](crates/g4/WITHDRAWN.md)) and the v1/v2 study ([`docs/paper/`](docs/paper/), [`docs/WHITEPAPER.md`](docs/WHITEPAPER.md)). The v1 command-line pipeline (`cargo run -p jarambit`) is still in the repository; its plate level is opt-in with `JARAMBIT_FEM=1`.
