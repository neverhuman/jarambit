# Input mesh smoothing report

Status: **PASS** (geometry checks only). Label: ef3i17_r05_p14_c06: best evolved design by canonical W_ULTRALIGHT_v1 (75.983539), not qualified

This is a geometry study. The smoothed mesh is NOT re-scored, re-assessed, certified or qualified. It is a triangle mesh, not B-rep CAD, and nothing in this report measures function. The synthetic sphere criteria do not apply to this input.

Tool: geometry-smoothing-benchmark 0.1.0

Command: `./target/release/geometry-smoothing-benchmark --input /home/ubuntu/jarambit_runs/smoothing/inputs/ef3i17_r05_p14_c06/ef3i17_r05_p14_c06.stl --out /home/ubuntu/jarambit_runs/smoothing/ef3i17_r05_p14_c06-R1-20261022T082935Z --label ef3i17_r05_p14_c06: best evolved design by canonical W_ULTRALIGHT_v1 (75.983539), not qualified --lambda 0.5 --mu -0.5 --max-displacement-mm 2 --normal-only --pin-feature-angle 30 --iterations 20`

## Input

- File: `/home/ubuntu/jarambit_runs/smoothing/inputs/ef3i17_r05_p14_c06/ef3i17_r05_p14_c06.stl`
- Bytes: 15501102
- SHA-256: `c31d4dbb7ca5380a5f55baa4ca841834fecbc0a7bd845787e088cdc15c633fda`
- Format: ascii

## Input audit (before smoothing)

| Measure | Value |
|---|---:|
| Vertices | 45696 |
| Faces | 91400 |
| Edges | 137100 |
| Euler characteristic | -4 |
| Open edges | 0 |
| Non-manifold edges | 0 |
| Inconsistent-winding edges | 0 |
| Degenerate faces | 0 |
| Duplicate faces | 0 |
| Unused vertices | 0 |
| Connected components | 1 |

## Smoothing

Method: `smooth_traced()`: 20 iterations of paired lambda 0.5 and mu -0.5 Laplacian passes; 20 completed. Normal-only: each pass keeps only the component of the umbrella vector along the area-weighted unit vertex normal, which is still Taubin lambda/mu filtering restricted to the normal direction. Total displacement is capped at 2 mm against the input. Boundary and feature-pinned vertices stay fixed. Feature pins: dihedral edges above 30 degrees pinned: 5706 feature edges, 5668 pinned vertices. An open boundary is rejected before smoothing.

## Measurements

| Measure | Before | After |
|---|---:|---:|
| Vertices | 45696 | 45696 |
| Faces | 91400 | 91400 |
| Edges | 137100 | 137100 |
| Euler characteristic | -4 | -4 |
| Signed volume, mm³ | 20725.333111 | 20710.666188 |
| Bounding box min, mm | -18.424400, -53.467800, -7.500000 | -18.424400, -53.467800, -7.511936 |
| Bounding box max, mm | 190.297500, 18.448500, 7.500000 | 190.297500, 18.448500, 7.511936 |

Volume change: -0.070768%. Bounding box change, min/max per axis (mm): (0.000000, 0.000000, -0.011936) / (0.000000, 0.000000, 0.011936).

Vertices moved: 31180 of 45696 (fraction 0.6823); 5038 moved more than 0.01 mm (fraction 0.1103). Max displacement 0.228999 mm (budget 2 mm). RMS displacement 0.015335 mm.

## Outputs

| File | Bytes | SHA-256 |
|---|---:|---|
| smoothed.stl | 26224184 | `e4ee6a7768b3e27886db1ef2b6d0ef853e62a54656dcb571912627751d532d68` |
| displacement.csv | 4236060 | `5faff0f99bd9fa375b95681e97c83ed0154cc3185a69ef6908d3718302f7a9da` |

## Limits

Smoothing moves vertices of the supplied triangle mesh. It does not reconstruct analytic surfaces, check continuous Hausdorff distance, or check feature preservation. No self-intersection test was run. The winner is not qualified by this report.
