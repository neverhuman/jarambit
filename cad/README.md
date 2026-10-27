# 3D Printing & CAD Upload Kit — Certified G4 Winner `g4v400_s042_62763aa2` (release v4.0.0)

This folder contains the ready-to-upload **6-file 3D-printing and manufacturing bundle** (**Units: millimetres**, every file under 50 MiB) for **`g4v400_s042_62763aa2`**, the winning individual of the **G4 (`v4.0.0`)** cohort of record ([`evidence/g4/v4.0.0/release.json`](../evidence/g4/v4.0.0/release.json)). It is populated automatically from [`evidence/g4/v4.0.0/winner_genome.csv`](../evidence/g4/v4.0.0/winner_genome.csv) by `g4-export` and [`ops/ci/gen_readme.py`](../ops/ci/gen_readme.py).

- **Status:** computational design; not built, printed or physically tested; not qualified for use as a knife.
> **Looking for the archived `v3.0.0` (`hz4i013_g00995_d206e3c9`) bundle?** All `v3.0.0` CAD files are preserved byte-for-byte in [`cad/v3.0.0/`](v3.0.0/).

**Engineering summary (`RS-karambit-v4`, `W_ERGOLIGHT_v2 = 92.45` / `W_ERGOLIGHT_v1 = 94.90`):**
- **Mass:** **`35.42 g`** as-finished (`35.68 g` as-printed with `+0.30 mm` bevel grind allowance; `108.3 g` solid outer envelope; **`67.3%` hollowness**).
- **All 17 structural LCB safety factors meet targets (`V(x) = 0`, all `SF_LCB >= 1.00`):** lateral hiker `180 N` SF **`4.62`**, lateral hunter `260 N` SF **`3.20`**, body torsion `1.82 N·m` SF **`14.74`**, heavy twist `16.2 N·m` SF **`1.66`**, solid-tip bisector torsion `0.80 N·m` SF **`3.29`**, solid-tip lateral pry `60 N` SF **`32.21`**, spine baton `3,427 N` (`HM-1`) SF **`4.27`**, Goodman fatigue SF **`2.31`**, serration tooth SF **`12.07`**, and per-load-case blade-before-grip sequence ratio **`2.17`** (`>= 1.20`).
- **Point geometry & mesh convergence:** point planform angle at `3 mm` **`41.38°`** (`<= 55°` `Gate S4`), 3-level Roache plate `GCI_21` **`0.37%`** (`<= 5%`), and zero sealed internal voids (`Betti_2 = 0`, `r_esc = 0.50 mm`).

## 1. Ready-to-upload 6-file kit (for forms accepting up to 6 files, 100 MB each)

| # | File | Format | Size | What it is & why to upload it |
|---:|---|---|---:|---|
| **1** | **[`g4v400_s042_62763aa2_AMbuild_mm.stl`](g4v400_s042_62763aa2_AMbuild_mm.stl)** | **Binary STL (`mm`)** | **42.36 MiB** | **PRIMARY 3D PRINT FILE IN GIT (upload this first):** complete additive-manufacturing build (`35.42 g`, `888416` triangles) with flange-graded skin, perforated Warren web, Sheet-TPMS core, drained hollow ring, and powder-removal ports. |
| **2** | **[`g4v400_s042_62763aa2_AMbuild_mm.3mf`](g4v400_s042_62763aa2_AMbuild_mm.3mf)** | **3MF (`unit="millimeter"`)** | **8.12 MiB** | **Modern 3D print package:** identical AM build mesh in compressed 3MF 1.2.3 format with `unit="millimeter"` and `Ti-6Al-4V ELI HIP (yield 828 MPa)` metadata. |
| **3** | **[`g4v400_s042_62763aa2_envelope_mm.stl`](g4v400_s042_62763aa2_envelope_mm.stl)** | **Binary STL (`mm`)** | **42.38 MiB** | **Solid outer envelope companion:** unhollowed exterior surface with all internal cavities and ports closed (`108.3 g` if printed solid; `888808` triangles). |
| **4** | **[`g4v400_s042_62763aa2_drawing_mm.dxf`](g4v400_s042_62763aa2_drawing_mm.dxf)** | **AutoCAD R12 DXF (`mm`)** | **738 KiB** | **1:1 2D engineering drawing (`UNITS: MILLIMETERS`, `1 drawing unit = 1 mm`):** exact `z = 0` mid-plane outline, `24.9 mm` ring bore, `z`-extent thickness envelope, and `30 mm` blade cross-section. |
| **5** | **[`g4v400_s042_62763aa2_spec_paper.pdf`](g4v400_s042_62763aa2_spec_paper.pdf)** | **PDF** | **138 KiB** | **Engineering specification & FE paper (`g4-v4.0.0.pdf`):** complete `G4.0` genome, `HM-1` hand mechanics, 3-level Roache GCI, and multi-seed QD ablation tables. |
| **6** | **[`g4v400_s042_62763aa2_xray_cutaway.jpg`](g4v400_s042_62763aa2_xray_cutaway.jpg)** | **JPG image** | **47 KiB** | **Lengthwise cutaway reference image:** visual guide showing outer skin, internal lattice, and powder-evacuation ports. |

*(Also included: [`g4v400_s042_62763aa2_print_spec.json`](g4v400_s042_62763aa2_print_spec.json), the machine-readable JSON receipt of dimensions, volumes, masses, and SHA-256 digests.)*

## 2. Copy-paste order notes for the printing form

```text
Units: mm | Overall dimensions: 172.77 x 50.06 x 9.48 mm | Mass: 35.42 g (finished) / 35.68 g (as-printed) | Material: Ti-6Al-4V ELI HIP (yield 828 MPa) | Status: computational design; not built, printed or physically tested; not qualified for use as a knife | Depowder via neutral-axis drainage ports | Post-HIP grind primary bevels to 20.2 deg per side (0.08 mm apex)
```
