# Jarambit 3.0.1: an evolved, 3D-printable titanium karambit

[![release](https://img.shields.io/badge/release-v3.0.1-2f56d6)](https://github.com/neverhuman/jarambit/releases/tag/v3.0.1) [![paper](https://img.shields.io/badge/paper-G3%20v3.0.1%20PDF-2f56d6)](docs/g3/paper/g3-v3.0.1.pdf) [![3D print CAD](https://img.shields.io/badge/3D%20print%20CAD-cad%2F%20(STL%20%7C%203MF%20%7C%20DXF%20%7C%20PDF)-2f56d6)](cad/) ![design](https://img.shields.io/badge/best%20individual-hz4i013__g00995__d206e3c9-555) ![score](https://img.shields.io/badge/W__ERGOLIGHT__v1-80.55-555) ![mass](https://img.shields.io/badge/mass-59.3%20g-555) ![status](https://img.shields.io/badge/status-computational%2C%20not%20built-b4570f)

Every measurement and picture on this page describes one design, **`hz4i013_g00995_d206e3c9`**, the best individual of the v3.0.0 cohort of record, and is generated from [`evidence/g3/v3.0.0/release.json`](evidence/g3/v3.0.0/release.json); the rule values (bounds, loads, targets) are the decoder's own. Status: computational design; not built, printed or physically tested.

**3D printing & CAD upload kit ([`cad/`](cad/)):** ready-to-upload 6-file bundle (each under 50 MB) — primary print mesh [`cad/hz4i013_g00995_d206e3c9_AMbuild_mm.stl`](cad/hz4i013_g00995_d206e3c9_AMbuild_mm.stl) (Binary STL, mm), unit-embedded [`cad/hz4i013_g00995_d206e3c9_AMbuild_mm.3mf`](cad/hz4i013_g00995_d206e3c9_AMbuild_mm.3mf) (3MF, mm), outer envelope [`cad/hz4i013_g00995_d206e3c9_envelope_mm.stl`](cad/hz4i013_g00995_d206e3c9_envelope_mm.stl), 1:1 2D drawing [`cad/hz4i013_g00995_d206e3c9_drawing_mm.dxf`](cad/hz4i013_g00995_d206e3c9_drawing_mm.dxf) (DXF, mm), engineering spec [`cad/hz4i013_g00995_d206e3c9_spec_paper.pdf`](cad/hz4i013_g00995_d206e3c9_spec_paper.pdf) (PDF), and internal cutaway [`cad/hz4i013_g00995_d206e3c9_xray_cutaway.jpg`](cad/hz4i013_g00995_d206e3c9_xray_cutaway.jpg) (JPG). See [`cad/README.md`](cad/README.md) for copy-paste order notes.

**Paper:** [`docs/g3/paper/g3-v3.0.1.pdf`](docs/g3/paper/g3-v3.0.1.pdf), generated from the same receipt, sets out the design space, objective, search, measurement chain and every limit.

<img src="docs/g3/v3.0.0/img/hero.jpg" alt="The best individual hz4i013_g00995_d206e3c9: anodized grip and ring, double-edged serrated claw blade" width="100%">

## The best individual

Ti-6Al-4V, laser powder-bed fusion: a hollow grip and blade core filled with a printed lattice, a 64.9 mm grip with a 24.9 mm finger ring, and a double-edged, double-serrated claw blade.

| Measure | Value | Target | Reading |
|---|---|---|---|
| W_ERGOLIGHT_v1 (draft d), Tier 2, Linux x86_64, h 0.2 mm | 80.55 | higher is better | ergonomic multiplier E 0.988 |
| Mass of the print file (watertight 0.2 mm mesh × 0.00443 g/mm³) | 59.3 g | under 50 g | **above the limit** |
| Edge A (inner): per-side angle, apex / bevel | 20.5–20.8° / 19.5–20.8° | 17.5–22.5° | ground from 57.0 to 3.0 mm from the point |
| Edge B (spine): per-side angle, apex / bevel | 20.6–21.6° / 18.7–21.7° | 17.5–22.5° | ground from 65.1 to 3.1 mm from the point |
| Steepest slope within 2 mm of either edge | 21.9° | no shoulder | one grind per edge |
| Serrations, edge A / B: depth, scallops | 1.63 mm, 3 / 1.41 mm, 5 | both edges serrated | mean flank 39° / 36° |
| Weakest tooth between scallops, edge A / B (2 / 4 checked; 180 N snag, 60 N pry, beam formula) | SF 3.47 / 2.83 | ≥ 1 | meets target |
| Solid tip, 1.82 N·m pry (TET10) | SF 0.64 | ≥ 1 | **below target** |
| Lateral load at the tip (plate FE) | 337 N | ≥ 260 N | meets target |
| Grip, worst case (beam formula) / hex8 voxel FE, spine 3427 N | SF 0.73 / 0.74 | ≥ 1 | **below target** |
| Human-max twist 16.2 N·m (plate FE, blade body) / hex8 voxel FE, root | SF 1.09 / 1.69 | ≥ 1 | meets target |
| Spine load 3427 N (plate FE) | SF 1.44 | ≥ 1 | meets target |
| Ring, 300 N bearing / 30 N back at SF 2 | 8677 / 1686 N | ≥ 600 / 60 N | meets target |
| Fatigue / blade-before-hand yield sequence | 0.94 / 0.56 | ≥ 1 / ≥ 1.2 | **below target** |
| Lateral-torsional buckling of the blade (analytic) | λ 11.2 | ≥ 1 | meets target |

<table><tr><td width="50%"><img src="docs/g3/v3.0.0/img/cut_b.jpg" alt="Section across the blade 30 mm from the point"><br>Section across the blade, 30 mm from the point: one grind per edge, hollow lattice core.</td><td width="50%"><img src="docs/g3/v3.0.0/img/tip.jpg" alt="The point"><br>The point; the grinds end 3.0 mm (A) and 3.1 mm (B) from it.</td></tr><tr><td><img src="docs/g3/v3.0.0/img/edge_b.jpg" alt="Spine edge with serrations"><br>Spine edge (B): one bevel, serration scallops.</td><td><img src="docs/g3/v3.0.0/img/edge_a.jpg" alt="Inner edge"><br>Inner edge (A).</td></tr><tr><td><img src="docs/g3/v3.0.0/img/xray.jpg" alt="X-ray of the internal cavities"><br>X-ray: internal cavities in amber.</td><td><img src="docs/g3/v3.0.0/img/cut_full.jpg" alt="Lengthwise cut through the grip"><br>Lengthwise cut: skin, lattice and powder ports.</td></tr><tr><td><img src="docs/g3/v3.0.0/img/grip.jpg" alt="Grip"><br>Grip.</td><td><img src="docs/g3/v3.0.0/img/ring.jpg" alt="Ring"><br>Ring, 24.9 mm bore, 8 mm lip.</td></tr><tr><td><img src="docs/g3/v3.0.0/img/blade.jpg" alt="The blade"><br>The blade: both serrated edges and the lightening slots.</td><td><img src="docs/g3/v3.0.0/img/cut_x.jpg" alt="Section across the grip"><br>Section across the grip: skin and lattice core.</td></tr><tr><td><img src="docs/g3/v3.0.0/img/tip_edge.jpg" alt="Edge at the point"><br>The edge at the point.</td><td><img src="docs/g3/v3.0.0/img/top.jpg" alt="Edge-on view"><br>Edge-on: ring, grip and blade thickness.</td></tr><tr><td colspan="2"><img src="docs/g3/v3.0.0/img/side.jpg" alt="Plan view" width="100%"><br>Plan view.</td></tr></table>

Edge sections at four stations per edge, true scale, measured normal to the edge from the exact decoder law (dashed: a 20° per-side grind):

<picture><source media="(prefers-color-scheme: dark)" srcset="docs/g3/v3.0.0/img/edge_sections_dark.png"><img src="docs/g3/v3.0.0/img/edge_sections_light.png" alt="Blade cross-sections of both edges"></picture>

## How evolution produced it

24 independent islands ran 2,500 evaluations each (60,000 in all) on xbabe4 (arm64 macOS), from scrambled Sobol only (64 per island); no seeded or hand-made individuals. Each island is a CVT-MAP-Elites archive of 1,024 behaviour niches under W_ERGOLIGHT_v1 (draft d), with a three-tier cascade: analytic Tier 0, voxel-SDF plus plate-FE Tier 1, and Tier 2 with a watertight mesh and a solid-tip TET10 model (gates 62 and 71, h 0.4 mm). The run kept 3,177 elites, 68 of them at Tier 2. The winner's island (island13) started from a best measured score of 56.54 after its Sobol initialisation.

<picture><source media="(prefers-color-scheme: dark)" srcset="docs/g3/v3.0.0/img/evolution_best_dark.png"><img src="docs/g3/v3.0.0/img/evolution_best_light.png" alt="Best measured score so far, every island"></picture>

<picture><source media="(prefers-color-scheme: dark)" srcset="docs/g3/v3.0.0/img/evolution_archive_dark.png"><img src="docs/g3/v3.0.0/img/evolution_archive_light.png" alt="Archive growth, every island"></picture>

The best design of island13 at each logged checkpoint, same camera and light:

<table><tr><td align="center"><img src="docs/g3/v3.0.0/img/lineage_00064.jpg" alt="evaluation 64" width="100%"><br>eval 64: 56.54, 72.1 g</td><td align="center"><img src="docs/g3/v3.0.0/img/lineage_00128.jpg" alt="evaluation 128" width="100%"><br>eval 128: 59.01, 65.3 g</td><td align="center"><img src="docs/g3/v3.0.0/img/lineage_00256.jpg" alt="evaluation 256" width="100%"><br>eval 256: 62.15, 68.0 g</td><td align="center"><img src="docs/g3/v3.0.0/img/lineage_00512.jpg" alt="evaluation 512" width="100%"><br>eval 512: 66.34, 59.9 g</td><td align="center"><img src="docs/g3/v3.0.0/img/lineage_01024.jpg" alt="evaluation 1024" width="100%"><br>eval 1,024: 70.94, 62.4 g</td><td align="center"><img src="docs/g3/v3.0.0/img/lineage_02048.jpg" alt="evaluation 2048" width="100%"><br>eval 2,048: 80.77, 56.0 g<br><b>winner of record</b></td><td align="center"><img src="docs/g3/v3.0.0/img/lineage_02500.jpg" alt="evaluation 2500" width="100%"><br>eval 2,500: 81.70, 51.3 g<br><b>rejected by the grid check</b></td></tr></table>

Scores and masses under each frame are the search's own (h 0.4 mm).

**Choosing the winner on the print grid.** The search measures every design on a 0.4 mm voxel grid, and that grid can undercount mass. The search's own leader, `hz4i013_g00840_2a44149e` (Tier 2 score 81.53 at h 0.4 on Linux; the search host logged it a little higher), weighs 51.3 g at h 0.4 but 75.2 g at h 0.2 and 74.6 g at h 0.15, and its Tier-1 score falls from 75.06 to 66.65. So the ranking and the mass of record are measured at h 0.2 mm, the grid of the print file (the voxel FE below runs at h 0.4 mm): every elite ordered by its record from the run, the top 32 re-evaluated at Tier 2 at h 0.2 on the search host, then the top 8 again on Linux x86_64, the platform of record (the winner scored 80.55 on the search host).

Grid check: mass in grams at each voxel pitch h (mm). Tier 1 (voxel SDF + plate) of the top 8 of the h 0.4 Linux Tier-2 ranking; h 0.2 and h 0.15 agree within 3.3 % for every design.

| Design | h 0.4 | h 0.3 | h 0.25 | h 0.2 | h 0.15 |
|---|---:|---:|---:|---:|---:|
| `hz4i013_g00840_2a44149e` | 51.3 | 55.6 | 54.9 | 75.2 | 74.6 |
| `hz4i013_g00995_d206e3c9` | 56.0 | 60.0 | 59.0 | 57.0 | 57.5 |
| `hz4i002_g00995_6f4643f1` | 58.8 | 59.6 | 58.9 | 58.7 | 58.5 |
| `hz4i013_g00345_be45c4bc` | 55.3 | 59.7 | 56.4 | 56.7 | 57.1 |
| `hz4i002_g00479_76f5e0ac` | 58.0 | 61.7 | 58.2 | 59.5 | 58.9 |
| `hz4i013_g00555_5cd1f106` | 50.6 | 54.1 | 54.4 | 52.1 | 53.1 |
| `hz4i005_g00345_c7922cf1` | 56.8 | 61.0 | 57.6 | 58.5 | 59.7 |
| `hz4i002_g00785_6b2ff4aa` | 59.1 | 62.0 | 58.8 | 61.8 | 59.8 |

Final ranking:

| Rank | Design | Score (Linux Tier 2, h 0.2) | Mass at h 0.2 |
|---:|---|---:|---:|
| 1 | `hz4i013_g00995_d206e3c9` | 80.55 | 57.0 g |
| 2 | `hz4i002_g00995_6f4643f1` | 80.44 | 58.7 g |
| 3 | `hz4i013_g00345_be45c4bc` | 79.51 | 56.7 g |
| 4 | `hz4i002_g00479_76f5e0ac` | 79.39 | 59.5 g |
| 5 | `hz4i013_g00555_5cd1f106` | 79.27 | 52.1 g |
| 6 | `hz4i004_g00501_cb3c9e4e` | 79.24 | 57.3 g |
| 7 | `hz4i002_g00905_b274d4d9` | 78.94 | 59.6 g |
| 8 | `hz4i007_g00005_98a9344b` | 78.77 | 53.7 g |

## How the design is constrained and measured

- **Edges (EDGE-1..5).** One primary grind per edge at 17.5–22.5° per side, measured normal to the actual serrated edge, at full height and unbroken to the point; only a short plunge by the guard is unsharpened.
- **Serrations (SERR-1..4).** Both edges serrated, at least 3 full-depth scallops each, pitch 5–9 mm, depth at most a quarter of the pitch, and no serration within 18 mm of the point. Every tooth between two scallops is checked under a single-tooth snag and pry (a beam formula on the exact section law), on this design and on 60 Sobol designs in the decoder's tests; the check is not a term of the score.
- **Evolution designs the knife.** Populations start from scrambled Sobol samples only. Nobody edits an individual; the rules above are constraints on what the decoder can express.
- **FE certification of this design (hex8 voxel FE on the as-built voxels, h 0.4 mm, von Mises q99.9 over the zone).** Grip zone: lateral 3.55, twist 5.50, spine 0.74. Root: lateral 2.08, twist 1.69, spine 1.29. Loads: lateral 260 N at the tip; human-max twist 16.2 N m; spine 3427 N at the blade centroid.
- **Tip mesh convergence.** Solid-tip SF 0.585 / 0.638 / 0.611 at 0.4 / 0.15 (production) / 0.08 mm² planform meshes.
- **Mass.** 59.3 g from the watertight 0.2 mm print mesh; the ranking counted 57.0 g on its 0.2 mm voxel grid (the score above uses this mass) and the search 56.0 g on its 0.4 mm grid.

## Limits of this result

- It weighs 59.3 g, above the 50 g goal.
- Its solid-tip SF is 0.64, below the 1 target.
- Its grip SF (formula) is 0.73, below the 1 target.
- Its grip spine SF (FE) is 0.74, below the 1 target.
- Its fatigue SF is 0.94, below the 1 target.
- Its yield-sequence ratio is 0.56, below the 1.2 target.
- It is a computational result: nothing has been printed, heat-treated, sharpened or tested.
- The archive was selected on the 0.4 mm search grid, which undercounts the mass of some designs (see the grid check); only the final ranking is measured at 0.2 mm, so designs the coarse grid scored low were not re-checked.
- The cohort is 2,500 evaluations per island, a shortened run for this release.

## Download the CAD & 3D-printing upload kit (`cad/`, millimetres)

The top-level [`cad/`](cad/) folder contains a ready-to-upload **6-file manufacturing bundle** (every file under 50 MB, designed for quote forms accepting up to 6 files of 100 MB each in `STL`, `3MF`, `DXF`, `PDF`, or `JPG`), generated automatically from the winning individual:

| # | File in `cad/` | Format | Purpose on a 3D-printing / manufacturing quote form |
|---:|---|---|---|
| 1 | [`cad/hz4i013_g00995_d206e3c9_AMbuild_mm.stl`](cad/hz4i013_g00995_d206e3c9_AMbuild_mm.stl) | Binary STL (`mm`) | **Primary 3D print file** (outer skin + internal TPMS lattice + powder-removal ports) |
| 2 | [`cad/hz4i013_g00995_d206e3c9_AMbuild_mm.3mf`](cad/hz4i013_g00995_d206e3c9_AMbuild_mm.3mf) | 3MF (`unit="millimeter"`) | **Modern 3D print package** with `millimeter` units and `Ti-6Al-4V` material embedded |
| 3 | [`cad/hz4i013_g00995_d206e3c9_envelope_mm.stl`](cad/hz4i013_g00995_d206e3c9_envelope_mm.stl) | Binary STL (`mm`) | **Solid outer envelope companion** (all internal cavities and ports closed) |
| 4 | [`cad/hz4i013_g00995_d206e3c9_drawing_mm.dxf`](cad/hz4i013_g00995_d206e3c9_drawing_mm.dxf) | AutoCAD DXF (`mm`) | **1:1 2D engineering drawing** (`$INSUNITS = 4` mm: planform, ring bore, thickness & 30 mm blade section) |
| 5 | [`cad/hz4i013_g00995_d206e3c9_spec_paper.pdf`](cad/hz4i013_g00995_d206e3c9_spec_paper.pdf) | PDF | **7-page engineering specification & FE verification paper** (`g3-v3.0.1.pdf`) |
| 6 | [`cad/hz4i013_g00995_d206e3c9_xray_cutaway.jpg`](cad/hz4i013_g00995_d206e3c9_xray_cutaway.jpg) | JPG image | **Lengthwise cutaway render** showing the internal TPMS lattice and powder-evacuation ports |

Mesh assets and high-resolution release files:

| File | Triangles | Size | SHA-256 | Contents | Where |
|---|---:|---:|---|---|---|
| `hz4i013_g00995_d206e3c9_AMbuild_mm.stl` | 1,001,372 | 48 MB | `782fc9c3453a22ad…` | print file: skin + TPMS lattice + powder ports, 0.450 mm voxels (finest that fits 50 MiB) | [`cad/hz4i013_g00995_d206e3c9_AMbuild_mm.stl`](cad/hz4i013_g00995_d206e3c9_AMbuild_mm.stl) · [archive](docs/g3/v3.0.0/cad/hz4i013_g00995_d206e3c9_AMbuild_mm.stl) |
| `hz4i013_g00995_d206e3c9_envelope_mm.stl` | 911,132 | 43 MB | `6c939b5fa82940e9…` | outer envelope, 0.300 mm voxels (finest that fits 50 MiB) | [`cad/hz4i013_g00995_d206e3c9_envelope_mm.stl`](cad/hz4i013_g00995_d206e3c9_envelope_mm.stl) · [archive](docs/g3/v3.0.0/cad/hz4i013_g00995_d206e3c9_envelope_mm.stl) |
| `hz4i013_g00995_d206e3c9_AMbuild_h0.2_mm.stl` | 5,373,548 | 256 MB | `48bbd9db5a48139c…` | print file: skin + TPMS lattice + powder ports, 0.2 mm voxels | [release v3.0.0](https://github.com/neverhuman/jarambit/releases/tag/v3.0.0) |
| `hz4i013_g00995_d206e3c9_envelope_h0.2_mm.stl` | 2,063,960 | 98 MB | `0e0654b07d78ec33…` | outer envelope, 0.2 mm voxels | [release v3.0.0](https://github.com/neverhuman/jarambit/releases/tag/v3.0.0) |
| `hz4i013_g00995_d206e3c9_envelope_fineblade_mm.stl` | 3,843,106 | 183 MB | `680e0df5fdfccc03…` | outer envelope; blade (x >= 92 mm) re-meshed at 0.05 mm from the exact decoder law | [release v3.0.0](https://github.com/neverhuman/jarambit/releases/tag/v3.0.0) |

Units are millimetres. The print file is the AM build: the outer skin, the internal TPMS lattice and the powder-removal ports. The files in git are written by [`ops/g3/export_best_stl.sh`](ops/g3/export_best_stl.sh) from the committed genome row, each at the finest voxel pitch that keeps it under 50 MiB; [`cad/hz4i013_g00995_d206e3c9_print_spec.json`](cad/hz4i013_g00995_d206e3c9_print_spec.json) ([archive](docs/g3/v3.0.0/cad/hz4i013_g00995_d206e3c9_print_spec.json)) lists their dimensions, volume, mass, pitch and watertightness.

## Reproduce

```bash
git checkout 80e669e5b558   # decoder a203278d80d2
cargo build --release --manifest-path crates/g3/Cargo.toml --bin g3-evolve --example pilot_tools --example gripfe
crates/g3/target/release/g3-evolve --out ~/jarambit_runs/evo_g3/<run>/island01/run --init 64 --evals 2500 \
  --seed 4001 --host 4 --island 1 --commit 80e669e5b558 --h 0.4 --tier1-gate 62 --tier2-gate 71 --log-every 50
crates/g3/target/release/examples/pilot_tools tier2 <root> <rank.jsonl> 32 tier2_top32.jsonl 0.2 4   # ranking of record
crates/g3/target/release/examples/pilot_tools hconv <root> <id,id,...> 0.4,0.3,0.25,0.2,0.15 grid.jsonl 4   # grid check
crates/g3/target/release/examples/pilot_tools export <root> hz4i013_g00995_d206e3c9 <out_dir> 0.2   # watertight envelope + AM build
crates/g3/target/release/examples/gripfe run <root> hz4i013_g00995_d206e3c9 0.4 fe.jsonl 4             # hex8 voxel FE certification
```

The winner's genome row is [`evidence/g3/v3.0.0/winner_genome.csv`](evidence/g3/v3.0.0/winner_genome.csv); every island's logged history is in [`evidence/g3/v3.0.0/history/`](evidence/g3/v3.0.0/history/).

The v1 command-line pipeline (`cargo run -p jarambit`) is still in the repository; its plate level is opt-in with `JARAMBIT_FEM=1`.

## Repository

- [`cad/`](cad/): 6-file 3D-printing & manufacturing upload kit (`STL`, `3MF`, `DXF`, `PDF`, `JPG`, millimetres) and order notes for `hz4i013_g00995_d206e3c9`, generated automatically from the winner.
- [`docs/g3/paper/`](docs/g3/paper/): the G3 paper; `make -C docs/g3/paper` rebuilds it from the receipt.
- [`docs/g3/RELEASE_3.0.0.md`](docs/g3/RELEASE_3.0.0.md): what the v3.0.0 release changed and why; later patches are in `docs/RELEASES.md`.
- [`docs/g3/RENDERS.md`](docs/g3/RENDERS.md): how every image of the design was made.
- [`crates/g3/README.md`](crates/g3/README.md): the G3 genome, decoder, objective and their known limitations.
- [`docs/RELEASES.md`](docs/RELEASES.md): every release. [`docs/WHITEPAPER.md`](docs/WHITEPAPER.md) and [`docs/paper/`](docs/paper/) (Paper B): the earlier v1 study, kept as its own record.
