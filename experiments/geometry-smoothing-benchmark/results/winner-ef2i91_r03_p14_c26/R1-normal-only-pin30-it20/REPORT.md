# Input mesh smoothing report

Status: **PASS** (geometry checks only). Label: ef2i91_r03_p14_c26: best evolved design by W_ULTRALIGHT_v1 (fitness 80.253391), not qualified

This is a geometry study. The smoothed mesh is NOT re-scored, re-assessed, certified or qualified. It is a triangle mesh, not B-rep CAD, and nothing in this report measures function. The synthetic sphere criteria do not apply to this input.

Tool: geometry-smoothing-benchmark 0.1.0

Command: `./target/ladder-bin-s1b --input /home/ubuntu/jarambit_runs/evo_full_20261018/xbabe2/islands/i91/by-content/e7ab9c8531255490be6a05c5dab8f629159ee9eae0580bda459a398a03747063/attempt-1792452614126747868/ef2i91_r03_p14_c26.stl --out /home/ubuntu/jarambit_runs/smoothing/ef2i91_r03_p14_c26-s1b-R1-20261021T231908Z --label ef2i91_r03_p14_c26: best evolved design by W_ULTRALIGHT_v1 (fitness 80.253391), not qualified --lambda 0.5 --mu -0.5 --max-displacement-mm 2 --normal-only --pin-feature-angle 30 --iterations 20`

## Input

- File: `/home/ubuntu/jarambit_runs/evo_full_20261018/xbabe2/islands/i91/by-content/e7ab9c8531255490be6a05c5dab8f629159ee9eae0580bda459a398a03747063/attempt-1792452614126747868/ef2i91_r03_p14_c26.stl`
- Bytes: 15435869
- SHA-256: `7741cf82418984b52e89e4a5f4f97ce49723e0d4513c2c9eada139b4bdeaeba6`
- Format: ascii

## Input audit (before smoothing)

| Measure | Value |
|---|---:|
| Vertices | 45468 |
| Faces | 90948 |
| Edges | 136422 |
| Euler characteristic | -6 |
| Open edges | 0 |
| Non-manifold edges | 0 |
| Inconsistent-winding edges | 0 |
| Degenerate faces | 0 |
| Duplicate faces | 0 |
| Unused vertices | 0 |
| Connected components | 1 |

## Smoothing

Method: `smooth_traced()`: 20 iterations of paired lambda 0.5 and mu -0.5 Laplacian passes; 20 completed. Normal-only: each pass keeps only the component of the umbrella vector along the area-weighted unit vertex normal, which is still Taubin lambda/mu filtering restricted to the normal direction. Total displacement is capped at 2 mm against the input. Boundary and feature-pinned vertices stay fixed. Feature pins: dihedral edges above 30 degrees pinned: 10090 feature edges, 8980 pinned vertices. An open boundary is rejected before smoothing.

## Measurements

| Measure | Before | After |
|---|---:|---:|
| Vertices | 45468 | 45468 |
| Faces | 90948 | 90948 |
| Edges | 136422 | 136422 |
| Euler characteristic | -6 | -6 |
| Signed volume, mm³ | 20358.296285 | 20340.753855 |
| Bounding box min, mm | -18.423700, -53.467800, -7.500000 | -18.423700, -53.467800, -7.509796 |
| Bounding box max, mm | 190.332900, 18.448600, 7.500000 | 190.332900, 18.448600, 7.509796 |

Volume change: -0.086168%. Bounding box change, min/max per axis (mm): (0.000000, 0.000000, -0.009796) / (0.000000, 0.000000, 0.009796).

Vertices moved: 35182 of 45468 (fraction 0.7738); 6978 moved more than 0.01 mm (fraction 0.1535). Max displacement 0.236983 mm (budget 2 mm). RMS displacement 0.017778 mm.

## Outputs

| File | Bytes | SHA-256 |
|---|---:|---|
| smoothed.stl | 26941039 | `895036be4f93beea3ce036946a9a21cbfcf4f262ec62b81433add776047d8888` |
| displacement.csv | 4331489 | `b5a3061c86c9f6637851140a7b7b944c3ee6e9e765913ce85824f2b69e20b233` |

## Limits

Smoothing moves vertices of the supplied triangle mesh. It does not reconstruct analytic surfaces, check continuous Hausdorff distance, or check feature preservation. No self-intersection test was run. The winner is not qualified by this report.
