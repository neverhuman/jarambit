# 3D Printing & CAD Upload Kit — Winner `hz4i013_g00995_d206e3c9` (release v4.0.1, cohort of record v3.0.0)

This folder contains the ready-to-upload **6-file 3D-printing and manufacturing bundle** (**Units: millimetres**, every file under 50 MiB) for **`hz4i013_g00995_d206e3c9`**, the winning individual of the v3.0.0 cohort of record ([`evidence/g3/v3.0.0/release.json`](../evidence/g3/v3.0.0/release.json)). It is populated automatically from [`evidence/g3/v3.0.0/winner_genome.csv`](../evidence/g3/v3.0.0/winner_genome.csv) by [`ops/g3/export_best_stl.sh`](../ops/g3/export_best_stl.sh) and [`ops/ci/gen_readme.py`](../ops/ci/gen_readme.py).

**Status & limits (computational design; not built, printed or physically tested):** this design is **not qualified for use as a knife**. Its 0.20 mm print-mesh mass is **59.3 g** (above the 50 g goal; `59.72 g` on the `0.45 mm` in-git mesh), and several structural safety factors are below target: solid-tip SF **0.64** (< 1), grip SF **0.73** (beam formula) / **0.74** (hex8 voxel FE spine, < 1), fatigue SF **0.94** (< 1), and blade-before-hand yield-sequence ratio **0.56** (< 1.2 target). Because the in-git print mesh (`hz4i013_g00995_d206e3c9_AMbuild_mm.stl` / `hz4i013_g00995_d206e3c9_AMbuild_mm.3mf`) is extracted on a **0.45 mm** voxel grid (the finest pitch under 50 MiB), it cannot represent the `0.05–0.12 mm` apex flat or the `20°` primary bevel at the cutting edge; cutting edges would be finished by grinding after printing to the `EDGE-3` angle (`17.5–22.5°` per side), or you can download the `0.2 mm` print mesh (`hz4i013_g00995_d206e3c9_AMbuild_h0.2_mm.stl`) and `0.05 mm` fine-blade mesh (`hz4i013_g00995_d206e3c9_envelope_fineblade_mm.stl`) from [release v3.0.0](https://github.com/neverhuman/jarambit/releases/tag/v3.0.0).

## 1. Ready-to-upload 6-file kit (for forms accepting up to 6 files, 100 MB each)

If your printing or manufacturing quote form asks for **"Drawings and related files (STL, OBJ, 3MF, STEP, PDF, images, DXF/DWG, Word, or a zip — up to 6 files, 100 MB each)"**, you can upload all **6 files** below (or just File #1 / File #2 if you only want a single 3D print file):

| # | File | Format | Size | What it is & why to upload it |
|---:|---|---|---:|---|
| **1** | **[`hz4i013_g00995_d206e3c9_AMbuild_mm.stl`](hz4i013_g00995_d206e3c9_AMbuild_mm.stl)** | **Binary STL (`mm`)** | **47.75 MiB** | **PRIMARY 3D PRINT FILE IN GIT (upload this first):** complete additive-manufacturing build at `0.45 mm` voxel pitch (finest under 50 MiB; `59.72 g`, 1,001,372 triangles) with outer skin, internal TPMS lattice, and powder-removal ports; cutting edges require post-print grinding. |
| **2** | **[`hz4i013_g00995_d206e3c9_AMbuild_mm.3mf`](hz4i013_g00995_d206e3c9_AMbuild_mm.3mf)** | **3MF (`unit="millimeter"`)** | **9.10 MiB** | **Modern 3D print package:** identical `0.45 mm` AM build mesh in compressed 3MF 1.2.3 format with `unit="millimeter"` and `Ti-6Al-4V (design assumption: yield 828 MPa)` material metadata embedded in the header. |
| **3** | **[`hz4i013_g00995_d206e3c9_envelope_mm.stl`](hz4i013_g00995_d206e3c9_envelope_mm.stl)** | **Binary STL (`mm`)** | **43.45 MiB** | **Solid outer envelope companion:** the unhollowed exterior surface at `0.3 mm` voxel pitch with all internal cavities and ports closed (119.4 g if printed solid; 911,132 triangles). |
| **4** | **[`hz4i013_g00995_d206e3c9_drawing_mm.dxf`](hz4i013_g00995_d206e3c9_drawing_mm.dxf)** | **AutoCAD R12 DXF (`mm`)** | **769 KiB** | **1:1 2D engineering drawing (`UNITS: MILLIMETERS`, `1 drawing unit = 1 mm`):** exact `z = 0` mid-plane outline, 24.9 mm ring bore at decoded `x = -5.97 mm`, `z`-extent thickness envelope (not a section), 30 mm blade cross-section, and overall dimension callouts. |
| **5** | **[`hz4i013_g00995_d206e3c9_spec_paper.pdf`](hz4i013_g00995_d206e3c9_spec_paper.pdf)** | **PDF** | **479 KiB** | **7-page engineering specification & FE paper (`g3-v4.0.1.pdf`):** complete geometry, bevel angles, CAD verification tables, and measured structural safety factors (including below-1 values: solid tip 0.64, grip 0.73 / 0.74, fatigue 0.94, yield-sequence 0.56 vs 1.2). |
| **6** | **[`hz4i013_g00995_d206e3c9_xray_cutaway.jpg`](hz4i013_g00995_d206e3c9_xray_cutaway.jpg)** | **JPG image** | **47 KiB** | **Lengthwise cutaway reference image:** visual guide for the print technician showing the outer skin, internal TPMS lattice, and powder-evacuation ports. |

*(Also included in this folder: [`hz4i013_g00995_d206e3c9_print_spec.json`](hz4i013_g00995_d206e3c9_print_spec.json), the machine-readable JSON receipt of dimensions, volumes, masses and SHA-256 digests.)*

## 2. Copy-paste order notes for the printing form

STL files store raw triangle coordinates without a unit header. When uploading to a printing bureau (Protolabs, PCBWay, Craftcloud, Shapeways, Xometry, JLC3DP, or a local L-PBF shop), select **Millimetres (`mm`)** and paste:

```text
Units: mm | Overall dimensions: 161.56 x 45.20 x 15.43 mm | Material: Ti-6Al-4V (design assumption: yield 828 MPa) | Depowder via neutral-axis drainage ports | Status: computational design; not built, printed or physically tested — below-target safety factors (solid-tip SF 0.64, grip SF 0.73 formula / 0.74 FE spine, fatigue SF 0.94, yield-sequence ratio 0.56 vs 1.2 target; mass 59.3 g vs 50 g goal); not qualified for use as a knife
```

| Specification | Value |
|---|---|
| Winning design ID | `hz4i013_g00995_d206e3c9` (genome `d206e3c9f4c3…`, decoder `a203278d80d2…`) |
| Units | `mm` (millimetres) |
| Overall bounding box ($L_x \times H_y \times T_z$) | `161.56 × 45.20 × 15.43 mm` |
| Material | Ti-6Al-4V (design assumption: yield 828 MPa, `0.00443 g/mm³`) |
| Mass (0.20 mm print mesh / 0.45 mm git mesh) | `59.3 g` / `59.72 g` (`13,481.5 mm³`) |
| Solid outer envelope mass (if unhollowed) | `119.4 g` (`26,953.1 mm³`) |
| Watertight manifold check (`AMbuild` / `envelope`) | `watertight = true` (`0` boundary edges, `0` non-manifold edges) |

## 3. High-resolution release assets (> 50 MiB)

| File | Triangles | Size | SHA-256 | Contents | Where |
|---|---:|---:|---|---|---|
| `hz4i013_g00995_d206e3c9_AMbuild_mm.stl` | 1,001,372 | 48 MB | `782fc9c3453a22ad…` | print file: skin + TPMS lattice + powder ports, 0.450 mm voxels (finest that fits 50 MiB) | [in `cad/`](hz4i013_g00995_d206e3c9_AMbuild_mm.stl) · [archive](../docs/g3/v3.0.0/cad/hz4i013_g00995_d206e3c9_AMbuild_mm.stl) |
| `hz4i013_g00995_d206e3c9_envelope_mm.stl` | 911,132 | 43 MB | `6c939b5fa82940e9…` | outer envelope, 0.300 mm voxels (finest that fits 50 MiB) | [in `cad/`](hz4i013_g00995_d206e3c9_envelope_mm.stl) · [archive](../docs/g3/v3.0.0/cad/hz4i013_g00995_d206e3c9_envelope_mm.stl) |
| `hz4i013_g00995_d206e3c9_AMbuild_h0.2_mm.stl` | 5,373,548 | 256 MB | `48bbd9db5a48139c…` | print file: skin + TPMS lattice + powder ports, 0.2 mm voxels | [release v3.0.0](https://github.com/neverhuman/jarambit/releases/tag/v3.0.0) |
| `hz4i013_g00995_d206e3c9_envelope_h0.2_mm.stl` | 2,063,960 | 98 MB | `0e0654b07d78ec33…` | outer envelope, 0.2 mm voxels | [release v3.0.0](https://github.com/neverhuman/jarambit/releases/tag/v3.0.0) |
| `hz4i013_g00995_d206e3c9_envelope_fineblade_mm.stl` | 3,843,106 | 183 MB | `680e0df5fdfccc03…` | outer envelope; blade (x >= 92 mm) re-meshed at 0.05 mm from the exact decoder law | [release v3.0.0](https://github.com/neverhuman/jarambit/releases/tag/v3.0.0) |

## 4. How to regenerate this folder automatically from the winner

```bash
./ops/g3/export_best_stl.sh
```

This reads [`evidence/g3/RECORD`](../evidence/g3/RECORD), decodes the winning genome row [`evidence/g3/v3.0.0/winner_genome.csv`](../evidence/g3/v3.0.0/winner_genome.csv), writes the watertight binary STL files and print spec to `docs/g3/v3.0.0/cad/`, generates the 6-file upload kit (`STL`, `3MF`, `DXF`, `PDF`, `JPG`) in `cad/`, and runs `python3 ops/ci/gen_readme.py` to update this guide and the root [`README.md`](../README.md).
