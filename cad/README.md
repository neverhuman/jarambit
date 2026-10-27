# 3D Printing & CAD Upload Kit — G4 Winner `g4v410_s042_a19374a8` (release v4.1.0)

This folder contains the ready-to-upload **6-file 3D-printing and manufacturing bundle** (**Units: millimetres**, every file under 50 MiB) for **`g4v410_s042_a19374a8`**, the winning individual of the **G4 (`v4.1.0`)** cohort of record ([`evidence/g4/v4.1.0/release.json`](../evidence/g4/v4.1.0/release.json)). It is populated automatically from [`evidence/g4/v4.1.0/winner_genome.csv`](../evidence/g4/v4.1.0/winner_genome.csv) by `g4-export`.

- **Status:** computational design; not built, printed or physically tested; not qualified for use as a knife.
> **Looking for the archived `v3.0.0` (`hz4i013_g00995_d206e3c9`) bundle?** All `v3.0.0` CAD files are preserved byte-for-byte in [`cad/v3.0.0/`](v3.0.0/).

**Engineering summary (`RS-karambit-v4`, `W_ERGOLIGHT_v2 = 94.49` / `W_ERGOLIGHT_v1 = 98.15`):**
- **Mass:** **`37.49 g`** finished (`38.15 g` with `+0.30 mm` bevel grind stock; `103.7 g` solid outer envelope; **`63.8%` hollowness**).
- **All 17 structural LCB safety factors meet targets (`V(x) = 0`, all `SF_LCB >= 1.00`):** lateral hiker `180 N` SF **`17.75`**, lateral hunter `260 N` SF **`12.29`**, body torsion `1.82 N·m` SF **`18.31`**, heavy twist `16.2 N·m` SF **`2.06`**, solid-tip bisector torsion `1.82 N·m` SF **`8.26`**, solid-tip lateral pry `60 N` SF **`47.93`**, spine load `3,427 N` (`HM-1`, first yield `828 MPa`) SF **`3.34`**, Goodman fatigue SF **`8.39`**, serration tooth SF **`7.50`**, and per-load-case blade-before-grip sequence ratio **`1.68`** (`>= 1.10`).
- **Point geometry & mesh convergence:** point planform angle at `3 mm` **`37.90°`** (`<= 55°` `Gate S4`), 3-level Roache plate `GCI_21` **`0.27%`** (`<= 5%`), and zero sealed internal voids (`Betti_2 = 0`, `r_esc = 0.50 mm`).

## 1. Ready-to-upload 6-file kit (for forms accepting up to 6 files, 100 MB each)

| # | File | Format | Size | What it is & why to upload it |
|---:|---|---|---:|---|
| **1** | **[`g4v410_s042_a19374a8_AMbuild_mm.stl`](g4v410_s042_a19374a8_AMbuild_mm.stl)** | **Binary STL (`mm`)** | **30.72 MiB** | **PRIMARY 3D PRINT FILE IN GIT (upload this first):** complete additive-manufacturing build (`37.49 g`, `644288` triangles) with flange-graded skin, perforated Warren web, Sheet-TPMS core, drained hollow ring, and powder-removal ports. |
| **2** | **[`g4v410_s042_a19374a8_AMbuild_mm.3mf`](g4v410_s042_a19374a8_AMbuild_mm.3mf)** | **3MF (`unit="millimeter"`)** | **5.90 MiB** | **Modern 3D print package:** identical AM build mesh in compressed 3MF 1.2.3 format with `unit="millimeter"` and `Ti-6Al-4V (L-PBF stress-relieved, yield 828 MPa)` metadata. |
| **3** | **[`g4v410_s042_a19374a8_envelope_mm.stl`](g4v410_s042_a19374a8_envelope_mm.stl)** | **Binary STL (`mm`)** | **24.34 MiB** | **Solid outer envelope companion:** unhollowed exterior surface with all internal cavities and ports closed (`103.7 g` if printed solid; `510548` triangles). |
| **4** | **[`g4v410_s042_a19374a8_drawing_mm.dxf`](g4v410_s042_a19374a8_drawing_mm.dxf)** | **AutoCAD R12 DXF (`mm`)** | **605 KiB** | **1:1 2D engineering drawing (`UNITS: MILLIMETERS`, `1 drawing unit = 1 mm`):** exact `z = 0` mid-plane outline, `24.9 mm` ring bore, `z`-extent thickness envelope, and `30 mm` blade cross-section. |
| **5** | **[`g4v410_s042_a19374a8_spec_paper.pdf`](g4v410_s042_a19374a8_spec_paper.pdf)** | **PDF** | **Spec Paper** | **Engineering specification & FE paper (`g4-v4.1.0.pdf`):** complete `G4.0` genome, `HM-1` hand mechanics, 3-level Roache GCI, and multi-seed QD ablation tables. |
| **6** | **[`g4v410_s042_a19374a8_xray_cutaway.jpg`](g4v410_s042_a19374a8_xray_cutaway.jpg)** | **JPG image** | **Cutaway** | **Lengthwise cutaway reference image:** visual guide rendered from `g4v410_s042_a19374a8` showing outer skin, internal lattice, and powder-evacuation ports. |

*(Also included: [`g4v410_s042_a19374a8_print_spec.json`](g4v410_s042_a19374a8_print_spec.json), the machine-readable JSON receipt of dimensions, volumes, masses, and SHA-256 digests.)*

## 2. Copy-paste order notes for the printing form

```text
Units: mm | Overall dimensions: 163.25 x 63.04 x 14.26 mm | Mass: 37.49 g (finished) / 38.15 g (with grind stock) | Material: Ti-6Al-4V (L-PBF stress-relieved, yield 828 MPa) | Status: computational design; not built, printed or physically tested; not qualified for use as a knife | Depowder via neutral-axis drainage ports | Post-build grind primary bevels to 19.5 deg per side (0.08 mm apex)
```
