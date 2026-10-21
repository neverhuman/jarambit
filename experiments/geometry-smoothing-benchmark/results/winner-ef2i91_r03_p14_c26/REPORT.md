# Input mesh smoothing report

Status: **FAIL** (geometry checks only). Label: ef2i91_r03_p14_c26: best evolved design by W_ULTRALIGHT_v1 (fitness 80.253391), not qualified

This is a geometry study. The smoothed mesh is NOT re-scored, re-assessed, certified or qualified. It is a triangle mesh, not B-rep CAD, and nothing in this report measures function. The synthetic sphere criteria do not apply to this input.

Tool: geometry-smoothing-benchmark 0.1.0

Command: `./target/release/geometry-smoothing-benchmark --input /home/ubuntu/jarambit_runs/evo_full_20261018/xbabe2/islands/i91/by-content/e7ab9c8531255490be6a05c5dab8f629159ee9eae0580bda459a398a03747063/attempt-1792452614126747868/ef2i91_r03_p14_c26.stl --out /home/ubuntu/jarambit_runs/smoothing/ef2i91_r03_p14_c26-20261021T231156Z --label ef2i91_r03_p14_c26: best evolved design by W_ULTRALIGHT_v1 (fitness 80.253391), not qualified`

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

Method: `smooth_traced()`, the synthetic benchmark arithmetic, unchanged: 20 iterations of paired lambda 0.5 and mu -0.5 Laplacian passes; 0 completed. Total displacement is capped at 2 mm against the input. Boundary and pinned vertices stay fixed. Pins: none. Sharp-feature pinning is not implemented, and an open boundary is rejected before smoothing.

## Measurements

| Measure | Before | After |
|---|---:|---:|
| Vertices | 45468 | 45468 |
| Faces | 90948 | 90948 |
| Edges | 136422 | 136422 |
| Euler characteristic | -6 | -6 |
| Signed volume, mm³ | 20358.296285 | 20292.245835 |
| Bounding box min, mm | -18.423700, -53.467800, -7.500000 | -18.410365, -53.393791, -7.514265 |
| Bounding box max, mm | 190.332900, 18.448600, 7.500000 | 190.312221, 18.433175, 7.514265 |

Volume change: -0.324440%. Bounding box change, min/max per axis (mm): (0.013335, 0.074009, -0.014265) / (-0.020679, -0.015425, 0.014265).

Vertices moved: 45468 of 45468. Max displacement 0.987748 mm (budget 2 mm). RMS displacement 0.189426 mm.

## Outputs

| File | Bytes | SHA-256 |
|---|---:|---|
| failed-state.stl | 28695680 | `ccb765cdca264477ce48c5acd1dc8b8ccf508ccc81417e07a29fe788eaff20f0` |
| failed-state-displacement.csv | 4771319 | `0e08e3a3eac8a395bf4d1982b2758296feedb1a26e3071908d6e289fd4ffe5a3` |

## Reasons

- smoothing stopped in iteration 1 of 20: smoothing reversed a face relative to the input
- 2050 faces reversed relative to input

## Limits

Smoothing moves vertices of the supplied triangle mesh. It does not reconstruct analytic surfaces, check continuous Hausdorff distance, or check feature preservation. No self-intersection test was run. The winner is not qualified by this report.
