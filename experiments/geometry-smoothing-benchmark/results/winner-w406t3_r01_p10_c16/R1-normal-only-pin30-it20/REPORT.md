# Input mesh smoothing report

Status: **PASS** (geometry checks only). Label: re_w406t3_r01_p10_c16 smoothing ladder L4, not qualified

This is a geometry study. The smoothed mesh is NOT re-scored, re-assessed, certified or qualified. It is a triangle mesh, not B-rep CAD, and nothing in this report measures function. The synthetic sphere criteria do not apply to this input.

Tool: geometry-smoothing-benchmark 0.1.0

Command: `/home/ubuntu/jarambit/.agent/worktrees/ladder/experiments/geometry-smoothing-benchmark/target/release/geometry-smoothing-benchmark --input /home/ubuntu/jarambit_runs/reassess-20261022/w406t3_r01_p10_c16/re_w406t3_r01_p10_c16.stl --out /home/ubuntu/jarambit_runs/smoothing/re_w406t3_r01_p10_c16-ladder-20261022T160510Z/L4/smoothing --label re_w406t3_r01_p10_c16 smoothing ladder L4, not qualified --lambda 0.5 --mu -0.5 --max-displacement-mm 2 --normal-only --pin-feature-angle 30 --iterations 20`

## Input

- File: `/home/ubuntu/jarambit_runs/reassess-20261022/w406t3_r01_p10_c16/re_w406t3_r01_p10_c16.stl`
- Bytes: 15269827
- SHA-256: `e574212003bb4c32eb124cb2beda93d6e8ac13d4295b6b0ca46f6795fe5a3bb2`
- Format: ascii

## Input audit (before smoothing)

| Measure | Value |
|---|---:|
| Vertices | 44980 |
| Faces | 89972 |
| Edges | 134958 |
| Euler characteristic | -6 |
| Open edges | 0 |
| Non-manifold edges | 0 |
| Inconsistent-winding edges | 0 |
| Degenerate faces | 0 |
| Duplicate faces | 0 |
| Unused vertices | 0 |
| Connected components | 1 |

## Smoothing

Method: `smooth_traced()`: 20 iterations of paired lambda 0.5 and mu -0.5 Laplacian passes; 20 completed. Normal-only: each pass keeps only the component of the umbrella vector along the area-weighted unit vertex normal, which is still Taubin lambda/mu filtering restricted to the normal direction. Total displacement is capped at 2 mm against the input. Boundary and feature-pinned vertices stay fixed. Feature pins: dihedral edges above 30 degrees pinned: 9985 feature edges, 8802 pinned vertices. An open boundary is rejected before smoothing.

## Measurements

| Measure | Before | After |
|---|---:|---:|
| Vertices | 44980 | 44980 |
| Faces | 89972 | 89972 |
| Edges | 134958 | 134958 |
| Euler characteristic | -6 | -6 |
| Signed volume, mm³ | 18786.231426 | 18769.094073 |
| Bounding box min, mm | -17.631400, -53.467500, -6.501000 | -17.631400, -53.467500, -6.510397 |
| Bounding box max, mm | 191.777800, 17.649100, 6.501000 | 191.777800, 17.649100, 6.510397 |

Volume change: -0.091223%. Bounding box change, min/max per axis (mm): (0.000000, 0.000000, -0.009397) / (0.000000, 0.000000, 0.009397).

Vertices moved: 34762 of 44980 (fraction 0.7728); 6338 moved more than 0.01 mm (fraction 0.1409). Max displacement 0.213522 mm (budget 2 mm). RMS displacement 0.015619 mm.

## Outputs

| File | Bytes | SHA-256 |
|---|---:|---|
| smoothed.stl | 26724197 | `157d22c71890d49ae7eee0e755823dcd433c598b749d51e12d9efb1219f2c310` |
| displacement.csv | 4307079 | `9b8507584e51a99648137cf9df23e0984d852ff9363a92eb72aa86e5326bc1c3` |

## Limits

Smoothing moves vertices of the supplied triangle mesh. It does not reconstruct analytic surfaces, check continuous Hausdorff distance, or check feature preservation. No self-intersection test was run. The winner is not qualified by this report.
