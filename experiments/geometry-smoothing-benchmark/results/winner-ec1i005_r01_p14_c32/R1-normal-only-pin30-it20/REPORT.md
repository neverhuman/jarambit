# Input mesh smoothing report

Status: **PASS** (geometry checks only). Label: ec1i005_r01_p14_c32: best evolved design by canonical W_ULTRALIGHT_v1 (76.141571), not qualified

This is a geometry study. The smoothed mesh is NOT re-scored, re-assessed, certified or qualified. It is a triangle mesh, not B-rep CAD, and nothing in this report measures function. The synthetic sphere criteria do not apply to this input.

Tool: geometry-smoothing-benchmark 0.1.0

Command: `./target/release/geometry-smoothing-benchmark --input /home/ubuntu/jarambit_runs/smoothing/inputs/ec1i005_r01_p14_c32/ec1i005_r01_p14_c32.stl --out /home/ubuntu/jarambit_runs/smoothing/ec1i005_r01_p14_c32-R1-20261022T133904Z --label ec1i005_r01_p14_c32: best evolved design by canonical W_ULTRALIGHT_v1 (76.141571), not qualified --lambda 0.5 --mu -0.5 --max-displacement-mm 2 --normal-only --pin-feature-angle 30 --iterations 20`

## Input

- File: `/home/ubuntu/jarambit_runs/smoothing/inputs/ec1i005_r01_p14_c32/ec1i005_r01_p14_c32.stl`
- Bytes: 15514285
- SHA-256: `eca6868b8cd08ed65af2c816e210a96a4142675e29b1a0342b09c3bb8ed24eb7`
- Format: ascii

## Input audit (before smoothing)

| Measure | Value |
|---|---:|
| Vertices | 45736 |
| Faces | 91480 |
| Edges | 137220 |
| Euler characteristic | -4 |
| Open edges | 0 |
| Non-manifold edges | 0 |
| Inconsistent-winding edges | 0 |
| Degenerate faces | 0 |
| Duplicate faces | 0 |
| Unused vertices | 0 |
| Connected components | 1 |

## Smoothing

Method: `smooth_traced()`: 20 iterations of paired lambda 0.5 and mu -0.5 Laplacian passes; 20 completed. Normal-only: each pass keeps only the component of the umbrella vector along the area-weighted unit vertex normal, which is still Taubin lambda/mu filtering restricted to the normal direction. Total displacement is capped at 2 mm against the input. Boundary and feature-pinned vertices stay fixed. Feature pins: dihedral edges above 30 degrees pinned: 5749 feature edges, 5638 pinned vertices. An open boundary is rejected before smoothing.

## Measurements

| Measure | Before | After |
|---|---:|---:|
| Vertices | 45736 | 45736 |
| Faces | 91480 | 91480 |
| Edges | 137220 | 137220 |
| Euler characteristic | -4 | -4 |
| Signed volume, mm³ | 20928.240005 | 20913.202344 |
| Bounding box min, mm | -18.424400, -53.467800, -7.500000 | -18.424400, -53.467800, -7.511493 |
| Bounding box max, mm | 190.336700, 18.448200, 7.500000 | 190.336700, 18.448200, 7.511493 |

Volume change: -0.071853%. Bounding box change, min/max per axis (mm): (0.000000, 0.000000, -0.011493) / (0.000000, 0.000000, 0.011493).

Vertices moved: 31324 of 45736 (fraction 0.6849); 5222 moved more than 0.01 mm (fraction 0.1142). Max displacement 0.223630 mm (budget 2 mm). RMS displacement 0.015225 mm.

## Outputs

| File | Bytes | SHA-256 |
|---|---:|---|
| smoothed.stl | 25533458 | `2ec17cea923cba63ab04ddbb3597e8d6d34a996b440a4b72c4d83184c92f7e62` |
| displacement.csv | 4125471 | `16d2e2914aea75d0427051838bdbc2d20614154294a1259cbf26a867c2191ebc` |

## Limits

Smoothing moves vertices of the supplied triangle mesh. It does not reconstruct analytic surfaces, check continuous Hausdorff distance, or check feature preservation. No self-intersection test was run. The winner is not qualified by this report.
