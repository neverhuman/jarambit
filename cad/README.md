# 3D Printing & CAD Upload Kit — Winner `hz4i013_g00995_d206e3c9` (v3.0.1)

This folder contains the ready-to-upload **6-file 3D-printing and manufacturing bundle** (**Units: millimetres**, every file under 50 MB) for **`hz4i013_g00995_d206e3c9`**, the winning individual of the v3.0.0 cohort of record ([`evidence/g3/v3.0.0/release.json`](../evidence/g3/v3.0.0/release.json)). It is populated automatically from [`evidence/g3/v3.0.0/winner_genome.csv`](../evidence/g3/v3.0.0/winner_genome.csv) by [`ops/g3/export_best_stl.sh`](../ops/g3/export_best_stl.sh) and [`ops/ci/gen_readme.py`](../ops/ci/gen_readme.py).

## 1. Ready-to-upload 6-file kit (for forms accepting up to 6 files, 100 MB each)

If your printing or manufacturing quote form asks for **"Drawings and related files (STL, OBJ, 3MF, STEP, PDF, images, DXF/DWG, Word, or a zip — up to 6 files, 100 MB each)"**, you can upload all **6 files** below (or just File #1 / File #2 if you only want a single 3D print file):

| # | File | Format | Size | What it is & why to upload it |
|---:|---|---|---:|---|
| **1** | **[`hz4i013_g00995_d206e3c9_AMbuild_mm.stl`](hz4i013_g00995_d206e3c9_AMbuild_mm.stl)** | **Binary STL (`mm`)** | **47.75 MiB** | **PRIMARY 3D PRINT FILE (upload this first):** complete additive-manufacturing build with the outer skin, internal TPMS lattice, and powder-removal drainage ports (1,001,372 triangles). |
| **2** | **[`hz4i013_g00995_d206e3c9_AMbuild_mm.3mf`](hz4i013_g00995_d206e3c9_AMbuild_mm.3mf)** | **3MF (`unit="millimeter"`)** | **9.10 MiB** | **Modern 3D print package:** identical AM build mesh in compressed 3MF 1.2.3 format with `unit="millimeter"` and `Ti-6Al-4V (Grade 5 / ELI LPBF)` material metadata embedded in the header. |
| **3** | **[`hz4i013_g00995_d206e3c9_envelope_mm.stl`](hz4i013_g00995_d206e3c9_envelope_mm.stl)** | **Binary STL (`mm`)** | **43.45 MiB** | **Solid outer envelope companion:** the unhollowed exterior surface with all internal cavities and ports closed (119.4 g if printed solid; 911,132 triangles). |
| **4** | **[`hz4i013_g00995_d206e3c9_drawing_mm.dxf`](hz4i013_g00995_d206e3c9_drawing_mm.dxf)** | **AutoCAD DXF (`mm`)** | **125 KiB** | **1:1 2D engineering drawing (`$INSUNITS = 4` mm):** planform silhouette, 24.9 mm ring bore, edge-on thickness profile, 30 mm blade cross-section, and overall dimension callouts. |
| **5** | **[`hz4i013_g00995_d206e3c9_spec_paper.pdf`](hz4i013_g00995_d206e3c9_spec_paper.pdf)** | **PDF** | **474 KiB** | **7-page engineering specification & FE paper:** complete geometry, bevel angles, structural safety factors, and CAD verification tables (`g3-v3.0.1.pdf`). |
| **6** | **[`hz4i013_g00995_d206e3c9_xray_cutaway.jpg`](hz4i013_g00995_d206e3c9_xray_cutaway.jpg)** | **JPG image** | **47 KiB** | **Lengthwise cutaway reference image:** visual guide for the print technician showing the outer skin, internal TPMS lattice, and powder-evacuation ports. |

*(Also included in this folder: [`hz4i013_g00995_d206e3c9_print_spec.json`](hz4i013_g00995_d206e3c9_print_spec.json), the machine-readable JSON receipt of dimensions, volumes, masses and SHA-256 digests.)*

## 2. Copy-paste order notes for the printing form

STL files store raw triangle coordinates without a unit header. When uploading to a printing bureau (Protolabs, PCBWay, Craftcloud, Shapeways, Xometry, JLC3DP, or a local L-PBF shop), select **Millimetres (`mm`)** and paste:

```text
Units: mm | Overall dimensions: 161.56 x 45.20 x 15.43 mm | Material: Ti-6Al-4V LPBF | Depowder via neutral-axis drainage ports
```

| Specification | Value |
|---|---|
| Winning design ID | `hz4i013_g00995_d206e3c9` (genome `d206e3c9f4c3…`, decoder `a203278d80d2…`) |
| Units | `mm` (millimetres) |
| Overall bounding box ($L_x \times H_y \times T_z$) | `161.56 × 45.20 × 15.43 mm` |
| Material | Ti-6Al-4V (Grade 5 / ELI LPBF) (`0.00443 g/mm³`) |
| Mass (0.20 mm print mesh / 0.45 mm git mesh) | `59.3 g` / `59.72 g` (`13,481.5 mm³`) |
| Solid outer envelope mass (if unhollowed) | `119.4 g` (`26,953.1 mm³`) |
| Watertight manifold check (`AMbuild` / `envelope`) | `watertight = true` (`0` boundary edges, `0` non-manifold edges) |

## 3. High-resolution release assets (> 50 MB)

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
