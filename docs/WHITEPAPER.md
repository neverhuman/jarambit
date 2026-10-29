# jarambit v7.0.0 — Continuous island evolution result

**Version 7.0.0 — 2026-10-29**

## Abstract

We report the latest retained continuous island evolution run on xbabe2, comprising 103 islands (i001–i103), 54,814 Stage 1 candidates, and 1,413 Stage 2 assessments. The highest-scoring candidate was `ec2i089_r01_p10_c68`, with W_ULTRALIGHT_v1 fitness 80.851214, modeled mass 83.409 g, and 3D mass estimate 83.3333 g. It passed 13 of 26 legacy hard gates and failed 13. It is not certified. The five leading scores span only 0.01 points (80.84–80.85), a plateau. This paper describes the run, score selection, observed results, limitations, and reproducibility evidence.

## Methods

The continuous system ran multiple seeded evolutionary islands. Each island generated candidate genomes over successive rounds; Stage 1 screened candidates and Stage 2 assessed selected candidates using the configured engineering scorecard. The asynchronous run manifest records island seeds, binary identity, candidate counts, assessment counts, completion state and best-so-far metrics. Selection used W_ULTRALIGHT_v1 as a weighted score; the hard-gate verdicts were retained separately and did not prevent selection. The run source binary was built from the v6.0.0 worktree. The full run ended 2026-10-29 08:52 EDT.

## Results

The winning retained record is `ec2i089_r01_p10_c68` from island i089. Fitness is 80.851214/100. The recorded masses are 83.409 g and 83.3333 g (3D). There were 103 islands, 54,814 Stage 1 candidates and 1,413 Stage 2 assessments. Nearby top-five scores fall in 80.84–80.85, consistent with a plateau.

### Gate results

The following table reproduces the winning candidate’s 26 recorded legacy hard gate outcomes. Raw values and thresholds come from `islands/i089/best_so_far_26_gates.json`.

| Gate | Result | Observed | Requirement |
|---|---|---:|---:|
| CAD watertight and single solid | PASS | True | True |
| Geometric and holder gates | PASS | True | True |
| Karambit ring and curved claw | PASS | True | True |
| C6 aesthetic floor | PASS | True | True |
| Certified hiker 180 N | PASS | 201.0 | 180.0 |
| Certified hunter 260 N | FAIL | 201.0 | 260.0 |
| Strict hunter 260 N | FAIL | 202.0 | 260.0 |
| Certified ring pull 1500 N | PASS | 26297.0 | 1500.0 |
| Body twist at 1.82 N·m | PASS | 2.416 | 1.0 |
| Tip only twist at 1.82 N·m | FAIL | 0.52 | 1.0 |
| Strict tip submodel twist | FAIL | 0.572 | 1.0 |
| Solid tip 3D P2 | FAIL | 0.7054 | 1.0 |
| Side A4 buckling and support | PASS | 3.9531 | 1.0 |
| Failure sequence 1.2 | PASS | 1.2 | 1.2 |
| Three level plate GCI | PASS | True | True |
| Dolfinx cross check ≤2% | PASS | 0.000567 | 0.02 |
| Aesthetic and geometric | PASS | pass | pass |
| Joint floors (828) | FAIL | 201.0 | 260.0 |
| Body and strict tip twist floor | FAIL | 0.572 | 1.0 |
| Solid tip safety factor | FAIL | 0.7054 | 1.0 |
| Strict hunter lateral | FAIL | 202.0 | 260.0 |
| Hard sequence | PASS | 1.2 | 1.2 |
| Human maximum | FAIL | {"hm_spine_sf": 0.35, "hm_twist_sf": 0.1, "ring_edge_r_mm": 1.125} | {"hm_spine_sf": 1.0, "hm_twist_sf": 1.0, "ring_edge_r_mm": 1.5} |
| Fatigue (Goodman L7) | FAIL | 0.5613 | 1.0 |
| Sharpness law | FAIL | {"apex_max_mm": 0.25, "half_angle_max_deg": 28.0} | {"apex_max_mm": 0.2, "half_angle_max_deg": 25.0} |
| Tip point B-rep profile | FAIL | {"monotone": false, "t_0p25_mm": 0.45, "t_1p0_mm": 0.85, "t_2p0_mm": 1.4} | {"monotone": true, "t_0p25_mm": 0.3, "t_1p0_mm": 0.6, "t_2p0_mm": 1.0} |

The candidate failed hunter load thresholds (201–202 N against 260 N), tip twist (0.52–0.572 against 1.0), solid-tip safety factor (0.7054 against 1.0), human maximum criteria, fatigue (0.5613 against 1.0), sharpness law, and tip point profile. Some failures are represented by both legacy and supplemental checks.

## Limitations and next steps

This is a search result, not a certification or independent qualification. The candidate does not meet all hard gates. Scores reflect the declared model and objectives; they do not establish real-world safety, manufacturing suitability, or performance outside the assessed cases. Next work should prioritize the hunter load, tip torsion and solid-tip margin, human maximum, fatigue, edge-radius law, and monotone tip profile, then rerun independent validation on a revised candidate.

## Reproducibility and data availability

The retained source package is `/home/ubuntu/jarambit_runs/evo_continuous/xbabe2`; key records are `MANIFEST.json`, `aggregate.log`, `leaders/fitness.json`, and `islands/i089/best_so_far_26_gates.json`. The run manifest records the binary SHA-256, git revision, island seeds and run metrics. The source binary derives from the v6.0.0 worktree. The aggregate run plots and i089 fitness curve are included under `docs/assets/v7/`. The retained package did not include a render of the winning candidate.
