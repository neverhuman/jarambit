# Jarambit 5.0.0: an evolved titanium karambit for laser powder-bed fusion

[![release](https://img.shields.io/badge/release-v5.0.0-2f56d6)](https://github.com/neverhuman/jarambit/releases/tag/v5.0.0) [![paper](https://img.shields.io/badge/paper-G5%20v5.0.0%20PDF-2f56d6)](docs/g5/paper/g5-v5.0.0.pdf) [![3D print CAD](https://img.shields.io/badge/3D%20print%20CAD-cad%2F%20(STL%20%7C%203MF%20%7C%20DXF%20%7C%20PDF)-2f56d6)](cad/) ![design](https://img.shields.io/badge/best%20individual-kv4i012__g00009__dd6ffea5-555) ![score](https://img.shields.io/badge/W__ERGOLIGHT__v2-59.34-555) ![mass](https://img.shields.io/badge/mass-83.3%20g-555) ![status](https://img.shields.io/badge/status-computational%2C%20not%20built-b4570f)

Every computed number and picture on this page describes one design, **`kv4i012_g00009_dd6ffea5`**, the best individual of the G5 v5.0.0 cohort of record. They are generated from [`evidence/g5/v5.0.0/release.json`](evidence/g5/v5.0.0/release.json), its verification studies and the sourced requirement set [`requirements/RS-karambit-v5.toml`](requirements/RS-karambit-v5.toml) ([provenance](requirements/PROVENANCE-v5.md)). Status: computational design; not built, printed or physically tested.

**Paper:** [`docs/g5/paper/g5-v5.0.0.pdf`](docs/g5/paper/g5-v5.0.0.pdf), generated from the same receipt. **3D printing:** the upload kit in [`cad/`](cad/).

<img src="docs/g5/v5.0.0/img/hero.jpg" alt="The best individual kv4i012_g00009_dd6ffea5" width="100%">

## The best individual

Titanium (Ti-6Al-4V, L-PBF, stress-relieved): a 69.1 mm grip of 84.1 mm perimeter with a 24.9 mm finger ring, a hollow core of sheet-TPMS lattice with powder ports, and a double-edged claw blade with serrations on both edges (7 and 5 teeth). It meets every RS-karambit-v5 load case: the lowest margin is fatigue, 1.005 (the minimum over the plate meshes 0.6 / 0.3 / 0.15 mm²).

| Measure | Value | Requirement (RS-karambit-v5) | Reading |
|---|---|---|---|
| W_ERGOLIGHT_v2, Tier 2 at h 0.2 mm (rank 1 of 32) | **59.34** | higher is better | hand multiplier E 0.740 (raised to the 5th power), worst-margin multiplier W 1.000 |
| Mass of record (partial volume at h 0.2 mm, × 0.00443 g/mm³) | **83.3 g** | ceiling 130 g; 50 g goal (a frontier, not a requirement) | meets target; watertight print mesh 84.5 g |
| Grip length / perimeter / height-to-thickness | 69.1 mm / 84.1 mm / 1.66 | ≥ 69 mm / 84–100 mm / 1.5–1.8 | meets target |
| Guard height / centre of mass to the index finger | 16.0 mm / 7.4 mm | ≥ 15 mm / ≤ 8 mm | meets target / meets target |
| Point: included angle at 1 mm / at 3 mm; width at 2 mm | 69.2° / 64.0°; 2.16 mm | ≤ 75° / ≤ 60°; ≤ 3 mm | meets target / **above the limit**; meets target (a continuous penalty, not a gate) |
| Edges A (inner) / B (spine): per-side grind angle | 21.5° / 23.1° | 20–25° | largest measured half-angle 23.1°, apex at most 0.088 mm thick |
| Serrations A / B: depth, teeth | 1.23 mm, 7 / 1.38 mm, 5 | 0.8–1.5 mm; gullet radius ≥ 0.5 mm by construction | weakest tooth SF 1.71 / 2.71 |
| Hand comfort factors (1 = within every limit; E is their weighted product) | contact pressure 0.67 | NASA/MIL hand limits (contact pressure, edge radii, grip section) | the other factors are 1.00 |
| Powder: sealed cavity / thin crevices (printed solid) | 176 mm³ / 306 mm³ | no closed voids; drainage passages ≥ 1 mm | sealed volume is penalised by the score; 4 ports of 3.8 mm (≥ 3 mm) |

Every RS-karambit-v5 load case, as a safety factor (SF) against its requirement. The margin is v = SF / requirement; the smallest margin sets the worst-margin multiplier W once it falls below 1. SF is on the production plate mesh (0.3 mm²), as scored; the margin of record is the minimum over the plate meshes 0.6 / 0.3 / 0.15 mm² (cases not solved on the plate are mesh-independent):

| Load case | SF | Requirement | Margin v (score mesh) | Margin v of record (min over meshes) |
|---|---:|---:|---:|---:|
| Lateral bending, limit: 12 N·m root moment, load 5 mm from the point (blade plate, grip) | 2.39 | ≥ 1.2 | 1.990 | 1.849 |
| Lateral bending, ultimate: 18 N·m (blade plate) | 1.70 | ≥ 1.3 | 1.308 | 1.216 |
| Body twist, limit: 16.2 N·m (blade plate, grip) | 1.93 | ≥ 1.2 | 1.612 | 1.581 |
| Tip, axial: 150 N along the tip bisector (TET10 solid) | 1.28 | ≥ 1.2 | 1.070 | 1.070 |
| Tip, side load: 10 N at the point (TET10 solid) | 1.96 | ≥ 1.2 | 1.637 | 1.637 |
| Tip, twist: 0.3 N·m, 3 mm from the point (TET10 solid) | 2.78 | ≥ 1.2 | 2.315 | 2.315 |
| Grip squeeze: 800 N | 3.15 | ≥ 1.2 | 2.623 | 2.623 |
| Guard push: 300 N | 28.18 | ≥ 1.2 | 23.487 | 23.487 |
| Ring bearing: 300 N on the handle side of the bore (T6 ring) | 32.51 | ≥ 2 | 16.255 | 16.255 |
| Ring back pull: 30 N (T6 ring) | 73.76 | ≥ 2 | 36.880 | 36.880 |
| Ring, any in-plane direction: 300 N (T6 ring sweep) | 3.85 | ≥ 1.2 | 3.211 | 3.211 |
| Axial pull: 600 N, held over the 20 mm nearest the point | 2.34 | ≥ 1.2 | 1.953 | 1.849 |
| Serration teeth: 180 N snag, 60 N pry shared by 3 teeth (every tooth) | 1.71 | ≥ 1.2 | 1.428 | 1.428 |
| Fatigue: 1,000,000 cycles, FKM mean stress (blade and hand zone) | 1.03 | ≥ 1 | 1.032 | 1.005 |
| Yield sequence: the blade yields before the hand zone, per load case (lateral / twist / pull) | 1.64 | ≥ 1.2 | 1.370 | 1.370 |
| Tip buckling under the 150 N axial load (Rayleigh–Ritz eigenvalue λ) | 14.32 | ≥ 1.5 | 9.548 | 9.548 |

<table><tr><td width="50%"><img src="docs/g5/v5.0.0/img/side.jpg" alt="Plan view"><br>Plan view.</td><td width="50%"><img src="docs/g5/v5.0.0/img/top.jpg" alt="Edge-on view"><br>Edge-on: ring, grip and blade thickness.</td></tr><tr><td><img src="docs/g5/v5.0.0/img/blade.jpg" alt="The blade"><br>The blade, reach 81.5 mm.</td><td><img src="docs/g5/v5.0.0/img/tip.jpg" alt="The point"><br>The point: 69° at 1 mm.</td></tr><tr><td><img src="docs/g5/v5.0.0/img/edge_a.jpg" alt="Inner edge"><br>Inner edge (A), 7 teeth, as decoded (before grinding).</td><td><img src="docs/g5/v5.0.0/img/edge_b.jpg" alt="Spine edge"><br>Spine edge (B), 5 teeth, as decoded (before grinding).</td></tr><tr><td><img src="docs/g5/v5.0.0/img/grip.jpg" alt="Grip"><br>Grip, 84.1 mm perimeter.</td><td><img src="docs/g5/v5.0.0/img/ring.jpg" alt="Ring"><br>Ring, 24.9 mm bore.</td></tr><tr><td><img src="docs/g5/v5.0.0/img/xray.jpg" alt="X-ray of the internal cavities"><br>X-ray of the hollow core.</td><td><img src="docs/g5/v5.0.0/img/cut_full.jpg" alt="Lengthwise cut"><br>Lengthwise cut: skin, lattice and ports.</td></tr><tr><td><img src="docs/g5/v5.0.0/img/cut_x.jpg" alt="Section across the grip"><br>Section across the grip.</td><td><img src="docs/g5/v5.0.0/img/cut_b.jpg" alt="Section across the blade"><br>Section across the blade.</td></tr><tr><td><img src="docs/g5/v5.0.0/img/tip_edge.jpg" alt="Edge at the point"><br>The edge at the point, as decoded (before grinding).</td><td></td></tr></table>

Edge sections at four stations per edge, true scale, from the exact decoder law (dashed: 20° per side, the smallest angle the requirement allows):

<picture><source media="(prefers-color-scheme: dark)" srcset="docs/g5/v5.0.0/img/edge_sections_dark.png"><img src="docs/g5/v5.0.0/img/edge_sections_light.png" alt="Blade cross-sections of both edges"></picture>

## How evolution produced it

16 independent islands each ran 3,000 evaluations (48,000 in all) of a MAP-Elites search over 1,024 Lloyd-relaxed niches, starting only from scrambled Sobol samples (64 per island; no seeded or hand-made individuals). Alongside them, 2 control islands drew 3,000 Sobol samples each with the same decoder, objective and promotion, and no evolution.

**All 16 of 16 evolving islands beat the best random-search island** (42.9). The island median rose from 16.2 after the Sobol start to 54.0 (range 47.3–63.4). Evolution found the winner in island12 at 2,425 evaluations.

<img src="docs/g5/v5.0.0/img/evolution.svg" alt="Best score found against evaluations, every island of the cohort of record" width="100%">

The best design of island12 at checkpoints of its run, same camera and light (scores on the h 0.4 mm search grid):

<table><tr><td align="center"><img src="docs/g5/v5.0.0/img/lineage_0064.jpg" alt="island12 at 64 evaluations" width="100%"><br>64 evaluations<br>16.7, 105 g</td><td align="center"><img src="docs/g5/v5.0.0/img/lineage_0256.jpg" alt="island12 at 256 evaluations" width="100%"><br>256 evaluations<br>40.0, 139 g</td><td align="center"><img src="docs/g5/v5.0.0/img/lineage_1024.jpg" alt="island12 at 1024 evaluations" width="100%"><br>1,024 evaluations<br>52.2, 102 g</td><td align="center"><img src="docs/g5/v5.0.0/img/lineage_2048.jpg" alt="island12 at 2048 evaluations" width="100%"><br>2,048 evaluations<br>58.6, 87 g</td><td align="center"><img src="docs/g5/v5.0.0/img/lineage_2425.jpg" alt="island12 at 2425 evaluations" width="100%"><br>2,425 evaluations<br>59.5, 83 g<br><b>winner of record</b></td><td align="center"><img src="docs/g5/v5.0.0/img/lineage_3000.jpg" alt="island12 at 3000 evaluations" width="100%"><br>3,000 evaluations<br>63.4, 104 g<br><b>search-grid leader</b></td></tr></table>

**Why the search leader is not the winner.** The best design on the search grid, `kv4i012_g00204_19a0b6f0` (63.4), ranks 23 once re-scored at Tier 2. On the TET10 solid its tip carries the 150 N axial case at SF 1.15, below the 1.2 required, where the section model of the search had given 1.38. Every finalist is judged on the solid.

Tier-2 ranking (h 0.2 mm, solid tip at 0.08 mm²) and the grid check of the top 8 (partial-volume mass in grams at h = 0.4 / 0.3 / 0.25 / 0.2 / 0.15 mm; the h 0.2 and h 0.15 mm masses must agree within 2 %):

| Rank | Design | Island | Score | E | W | Mass (g) | Mass at the five grids (g) | h 0.2 vs 0.15 |
|---:|---|---|---:|---:|---:|---:|---|---:|
| 1 | `kv4i012_g00009_dd6ffea5` | island12 | 59.34 | 0.740 | 1.000 | 83.3 | 83.3 / 83.3 / 83.2 / 83.3 / 83.0 | 0.33 % |
| 2 | `kv4i012_g00239_68594d05` | island12 | 58.52 | 0.739 | 0.989 | 81.9 | 81.9 / 82.4 / 82.2 / 81.9 / 81.9 | 0.03 % |
| 3 | `kv4i012_g00449_317fdf8f` | island12 | 58.48 | 0.732 | 1.000 | 84.2 | 84.0 / 84.9 / 84.8 / 84.2 / 84.4 | 0.26 % |
| 4 | `kv4i012_g01004_24f201c1` | island12 | 58.20 | 0.771 | 1.000 | 100.7 | 100.7 / 100.9 / 101.0 / 100.7 / 100.8 | 0.08 % |
| 5 | `kv4i007_g00335_5e62ba21` | island07 | 57.52 | 0.794 | 1.000 | 103.2 | 103.2 / 104.3 / 103.8 / 103.2 / 102.9 | 0.24 % |
| 6 | `kv4i016_g00279_3d756d1e` | island16 | 57.00 | 0.806 | 1.000 | 110.2 | 109.9 / 111.1 / 110.5 / 110.2 / 110.2 | 0.00 % |
| 7 | `kv4i004_g00009_7eed028c` | island04 | 56.80 | 0.793 | 1.000 | 103.4 | 103.4 / 103.8 / 103.1 / 103.4 / 103.3 | 0.09 % |
| 8 | `kv4i016_g00281_a663ab0e` | island16 | 56.79 | 0.780 | 1.000 | 116.1 | 116.4 / 116.6 / 115.9 / 116.1 / 116.4 | 0.21 % |

## Verification and limits

- **Reproducible on the platform of record.** Re-evaluating all 1,508 final elites reproduces every Tier-1 archive row bit for bit (706 rows), and island01 re-run from the launch record reproduces its history bit for bit (8 entries over 200 evaluations, arm64 macOS (xbabe4), the platform of record).
- **Not bit-identical across architectures.** On Linux x86_64 the platform maths library differs: the top 8 Tier-2 scores agree within 1.24 % (bit-identical: no), the winner ranks 1 there as well, and island01 keeps the same best design through all 200 evaluations compared.
- **Plate mesh sensitivity** ([`vv/plate_mesh.json`](evidence/g5/v5.0.0/vv/plate_mesh.json)). Across plate meshes of 1.2 / 0.6 / 0.3 / 0.15 mm² the lateral safety factor of the study designs varies by up to 20 % (not monotone) and the twist safety factor by up to 6 %; the search and ranking use 0.3 mm². So the winner was chosen on the minimum over 0.6 / 0.3 / 0.15 mm² ([`plate_meshes.jsonl`](evidence/g5/v5.0.0/plate_meshes.jsonl), solved on the platform of record, whose 0.3 mm² rows reproduce the ranking bit for bit): in rank order, the first design meeting every load case on every mesh. It is the top-ranked design; its lowest margin over the meshes is fatigue 1.005.
- **Numerical studies** ([`evidence/g5/v5.0.0/vv/`](evidence/g5/v5.0.0/vv/)): grid agreement of mass, blade-plate and solid-tip mesh studies, the grip perimeter within 0.12 % of its gene, Rayleigh–Ritz against Euler within 5e-09, and Lloyd niches with 12.5 % less quantisation error than Halton.
- **Texture on the blade.** The frozen G5.0 decoder lets the evolved surface texture reach the blade (here texture depth 0.20 mm, blade scope 0.99), bevels included, and the renders and print meshes show it. The edge figures and measurements use the exact bevel law, which grinding after printing restores: the requirement's finishing allowance (0.5 mm per ground face) exceeds the texture depth. Keeping texture off the bevels is a decoder change for a later generation.
- **Status: computational design; not built, printed or physically tested.** Nothing here is a qualification for use. The safety factors are computed from models with the limits stated in the paper; the material values are specification minima, not measured coupons.
- **Evidence gaps** listed in [`requirements/PROVENANCE-v5.md`](requirements/PROVENANCE-v5.md) need physical tests before any build: tooth loads, tip load shares, edge retention, thin-wall roughness, lattice homogenisation, the temperature derate and the drop test.

## 3D printing and CAD (`cad/`, millimetres)

The upload kit in [`cad/`](cad/) holds six files for `kv4i012_g00009_dd6ffea5`, each under 50 MiB, generated from the winner's genome row ([`evidence/g5/v5.0.0/winner_genome.csv`](evidence/g5/v5.0.0/winner_genome.csv)). Order notes are in [`cad/README.md`](cad/README.md).

| File | What it is |
|---|---|
| [`cad/kv4i012_g00009_dd6ffea5_AMbuild_mm.stl`](cad/kv4i012_g00009_dd6ffea5_AMbuild_mm.stl) | print file: skin, sheet-TPMS lattice and powder ports, 0.5 mm voxels, 947,160 triangles |
| [`cad/kv4i012_g00009_dd6ffea5_AMbuild_mm.3mf`](cad/kv4i012_g00009_dd6ffea5_AMbuild_mm.3mf) | the same print mesh as 3MF, with millimetre units |
| [`cad/kv4i012_g00009_dd6ffea5_envelope_mm.stl`](cad/kv4i012_g00009_dd6ffea5_envelope_mm.stl) | outer envelope, 0.325 mm voxels |
| [`cad/kv4i012_g00009_dd6ffea5_drawing_mm.dxf`](cad/kv4i012_g00009_dd6ffea5_drawing_mm.dxf) | 1:1 plan drawing (DXF, mm) |
| [`cad/kv4i012_g00009_dd6ffea5_spec_paper.pdf`](cad/kv4i012_g00009_dd6ffea5_spec_paper.pdf) | the G5 paper |
| [`cad/kv4i012_g00009_dd6ffea5_xray_cutaway.jpg`](cad/kv4i012_g00009_dd6ffea5_xray_cutaway.jpg) | cutaway of the hollow core |

Finer meshes and the full evaluation records are release assets of [v5.0.0](https://github.com/neverhuman/jarambit/releases/tag/v5.0.0), each with its SHA-256 in the receipt:

| Asset | Size | SHA-256 |
|---|---:|---|
| `g5-v5.0.0-archives.tar.gz` | 2.3 MiB | `5f86e3f118ba02c7…` |
| `g5-v5.0.0-baseline_eval_h040.jsonl` | 1.1 MiB | `63fdba928159619a…` |
| `g5-v5.0.0-eval_h040.jsonl` | 6.9 MiB | `66055f05d8b03a2f…` |
| `g5-v5.0.0-kv4i012_g00009_dd6ffea5_AMbuild_h0.2_mm.stl` | 289.9 MiB | `3247e7918014b7e3…` |
| `g5-v5.0.0-kv4i012_g00009_dd6ffea5_envelope_h0.2_mm.stl` | 113.1 MiB | `250875000e163f74…` |

## Reproduce

```bash
cargo build --release --locked --manifest-path crates/g5/Cargo.toml --bins
# one island of the cohort of record (decoder c63629d4b310, freeze label bf3e37fa); seed = host × 1000 + island
crates/g5/target/release/g5-evolve --out ~/jarambit_runs/evo_g5/<run>/island01/run --init 64 --evals 3000 \
  --seed 4001 --host 4 --island 1 --commit bf3e37fa --h 0.4 --tier1-top 0.4 --tier2-top 0 --log-every 25
crates/g5/target/release/g5-pilot eval <cohort_root> eval_h040.jsonl 0.4 4
crates/g5/target/release/g5-pilot tier2 <cohort_root> eval_h040.jsonl 32 tier2_h020.jsonl 0.2 4
crates/g5/target/release/g5-release receipt . <cohort_root> <control_root> <cert_dir> 32 8
crates/g5/target/release/g5-vv evidence/g5/v5.0.0/vv 4
```

Every island's genomes and history are in [`evidence/g5/v5.0.0/`](evidence/g5/v5.0.0/), so any elite can be re-evaluated.

## Repository

- [`crates/g5/`](crates/g5/README.md): the G5 genome (G5.0), decoder, materials, load models, objective, search and release tools.
- [`requirements/`](requirements/RS-karambit-v5.toml): RS-karambit-v5 and its [provenance](requirements/PROVENANCE-v5.md).
- [`evidence/g5/v5.0.0/`](evidence/g5/v5.0.0/): the receipt, the winner's genome, every island's history, the ranking, the grid check and `vv/`.
- [`docs/g5/`](docs/g5/): the paper, the images and the CAD archive. [`docs/RELEASES.md`](docs/RELEASES.md): every release.
- Earlier generations, kept as their own record: G3 ([`docs/g3/`](docs/g3/), [`crates/g3/`](crates/g3/README.md)), the withdrawn G4 prototype ([`crates/g4/WITHDRAWN.md`](crates/g4/WITHDRAWN.md)) and the v1/v2 study ([`docs/paper/`](docs/paper/), [`docs/WHITEPAPER.md`](docs/WHITEPAPER.md)). The v1 command-line pipeline (`cargo run -p jarambit`) is still in the repository; its plate level is opt-in with `JARAMBIT_FEM=1`.
