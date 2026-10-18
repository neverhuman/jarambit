# Phase-4 unified multi-lane prototype evaluation

**Result:** the requested witnesses can be combined into a canonical, native-planform-legal prototype with a solid 6.0 mm neck and one controlled blade window. Their passing strength results do not transfer as a complete set. All three combined variants retain lateral/fatigue/hunter targets on the two certification meshes and the strict mesh, but each misses sequence >=1.25 on one certification mesh, and every combined human-max twist/spine factor remains below 1. No combined candidate passes all assignment targets.

All Phase-4 sources, snapshots, invocation records, raw results, summaries and reproduction scripts are in `target/gate-feasibility-bound/unified_study/`. Worktree `/home/ubuntu/jarambit_wt_diag`, branch `audit/gate-feasibility-bound`, baseline commit `21425fe2948ab3941eedbe9710ebc3d29427faae`. Every computation uses CPUs 10,11 and `CARGO_BUILD_JOBS=2`; the existing lane target is reused. Production sources and requirements are unchanged.

## Imported witnesses and control reproduction

The neck/window input and config come from the assigned `lane-G16-06-RSV2-08/experiments/G16-06-RSV2-08` directory. The point-cap law and hotspot seed/strict receipt come from the assigned `lane-RSV2-07-hm` directory. Nineteen input/source/receipt files were copied once into `inputs/` and hashed; source lanes were not modified. Subsequent reproduction uses these stable snapshots.

The copied neck config explicitly selects skyline, while the retained candidate validation receipt and its stated witness numbers use faer. A partial skyline control was stopped before strict completion and its completed observations were preserved in `neck_skyline_partial.json`. The effective configs for all complete controls and combined evaluations explicitly select faer. `solver_import_note.json` and invocation receipts record the discrepancy and override; parity tolerances were not loosened.

| Local control | Areas mm² | Maximum relative difference from retained scalar metrics |
| --- | --- | --- |
| neck_control | 2, 0.25, 0.125, 0.0625 | 1.29177552328e-13 |
| hotspot_control | 0.0625 | 5.73541331848e-14 |

## Exact combined construction

The canonical `combined.csv` changes only CP7 from 1.1582812727496934 to 1.4582812727496934 (+0.3 mm) and CP8 from 0.65423624609483888 to 0.85423624609483895 (+0.2 mm) relative to the copied neck candidate. All other genes and material remain unchanged. Native clamping changes no gene. All combined variants pass the unchanged aesthetic and geometry predicates. This prototype additionally needs the supplied section law and prototype phenotype generator; the CSV alone does not reproduce it through the production phenotype.

The neck source removes its neck comet slot, keeps the ring/cairn and a blade-only window, and uses full 6.0 mm stock/knots. The controlled window keeps WIN_N=1, WIN_W=6.0 mm and WIN_FILL=0.65; actual window area is 175.129120106 mm² in all combined cases. The original 3.2 mm window rim is retained, with clearance computed from the actual compound-bevel extent. No hole is moved into the neck.

Let S(u) be cubic smoothstep after clamping u to [0,1], R(q) the native raster value and x the grip coordinate. The neck source defines N(q)=R(q)+(6-R(q))*S((x-ring_od/2)/3), then B(q)=N(q)+(6-N(q))*S((x-heel_x)/5). All sampled material beyond the ring transition and before the heel, outside the cutting-edge band, is exactly 6.0 mm; the analytic formula also preserves this full-stock neck scope. The inherited ring groove/collar remains.

The micro-bevel retains eta=0.175 mm and theta=18 degrees. With actual minimum cutting-edge segment distance d, L(d)=eta+2*d*tan(theta); E=L for d<=d0, E=L+(B-L)*S((d-d0)/(d1-d0)) through the transition, and E=B beyond d1. Three variants use d0/d1=0.65/1.10, 0.55/0.85 and 0.48/0.65 mm. No extra blade-band transition is enabled.

The imported directional point coordinate is a=hypot(axial,2*lateral), with the native tip bisector defining axial/lateral coordinates. Its POINT_SHAPE=3 envelope is reproduced byte for byte from the hotspot source:

```text
P(a) = eta + 0.30*a                                    0 <= a <= 2
       eta+0.60 + (0.94*K-(eta+0.60))*(a-2)/1.2         2 < a <= 3.2
       K*(0.94 + 0.005*(a-3.2))                        3.2 < a <= 5.2
       K*(0.95 + 0.05*(a-5.2)/6.8)                     5.2 < a <= 12
       +infinity                                       a > 12
```

K=6 mm for the combined inputs. The final law applies min(B,E,P), retains the native bore collar, and floors thickness at 0.05 mm. Raster construction, its 0.25 mm spacing and f32/nearest-cell behavior, and subsequent evaluations of the shared FEM/export thickness law retain the imported ordering. They are not replaced by an ideal continuum stock field. The native FEM mesh, four-sample element-thickness averaging, load patches, 4 mm exclusion, T6 nodal peaks, joint factors, St-Venant correction, fatigue model and numerical thresholds are unchanged.

## All four native FEM levels

All runs are unbiased with faer. Area=2 mm² is retained as a coarse observation. The source witness protocol compares min(0.25,0.125 mm²) and a separate 0.0625 mm² strict check; no mesh level is selected because it passes. Mesh values need not vary monotonically. The requested stronger margins are lateral >=358 N, hunter SF >=1.377 and sequence >=1.25; native sequence remains >=1.20. Fatigue and the human-max/tip/body safety-factor targets remain >=1.

| Case (d0/d1 mm) | Area mm² | Lateral N | Hunter SF | Fatigue SF | Sequence | Body twist SF | Tip-zone SF | HM twist SF | HM spine SF | HM ring-back SF |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| combined_d065 | 2.0 | 343.950624 | 1.32288701 | 0.960765894 | 1.26446783 | 11.576413 | 2.78072125 | 0.312402017 | 0.497357135 | 6.75348202 |
| combined_d065 | 0.25 | 397.034709 | 1.52705657 | 1.109047 | 1.23693512 | 13.3747732 | 2.50945633 | 0.281926575 | 0.531948009 | 6.81909128 |
| combined_d065 | 0.125 | 386.954445 | 1.48828633 | 1.08088954 | 1.28724044 | 13.913087 | 3.10613749 | 0.348961125 | 0.52406198 | 6.57302839 |
| combined_d065 | 0.0625 | 385.35232 | 1.48212431 | 1.07641429 | 1.33326069 | 11.9231425 | 3.14971306 | 0.353856653 | 0.466910823 | 6.44236702 |
| combined_d055 | 2.0 | 321.466262 | 1.2364087 | 0.897959762 | 1.19563078 | 14.9761085 | 3.30883574 | 0.371733398 | 0.494823728 | 6.75348202 |
| combined_d055 | 0.25 | 401.826303 | 1.54548578 | 1.12243148 | 1.22218664 | 10.9980812 | 3.10223666 | 0.348522884 | 0.559945095 | 6.81909128 |
| combined_d055 | 0.125 | 365.303325 | 1.40501279 | 1.02041093 | 1.2825286 | 10.8242001 | 3.4651716 | 0.389297057 | 0.545131641 | 6.57302839 |
| combined_d055 | 0.0625 | 406.977039 | 1.5652963 | 1.13681916 | 1.26242112 | 9.84481464 | 3.49888147 | 0.393084214 | 0.484855555 | 6.44236702 |
| combined_d048 | 2.0 | 292.958696 | 1.12676421 | 0.818328864 | 1.28120161 | 11.3186248 | 3.88237187 | 0.436167704 | 0.454136282 | 6.75348202 |
| combined_d048 | 0.25 | 382.434402 | 1.47090155 | 1.0682636 | 1.28416108 | 11.5383604 | 3.44094316 | 0.386575096 | 0.565685833 | 6.81909128 |
| combined_d048 | 0.125 | 407.827126 | 1.56856587 | 1.13919373 | 1.22136336 | 14.2142252 | 3.63708388 | 0.408610658 | 0.571658066 | 6.57302839 |
| combined_d048 | 0.0625 | 360.706651 | 1.38733327 | 1.00757093 | 1.42436278 | 13.1113557 | 3.8264798 | 0.429888472 | 0.51134839 | 6.44236702 |

Protocol minima and separate strict results:

| Case | Protocol | Lateral N | Fatigue SF | Sequence | HM twist SF | HM spine SF |
| --- | --- | --- | --- | --- | --- | --- |
| combined_d065 | min(0.25,0.125) | 386.954445 | 1.08088954 | 1.23693512 | 0.281926575 | 0.52406198 |
| combined_d065 | strict 0.0625 | 385.35232 | 1.07641429 | 1.33326069 | 0.353856653 | 0.466910823 |
| combined_d055 | min(0.25,0.125) | 365.303325 | 1.02041093 | 1.22218664 | 0.348522884 | 0.545131641 |
| combined_d055 | strict 0.0625 | 406.977039 | 1.13681916 | 1.26242112 | 0.393084214 | 0.484855555 |
| combined_d048 | min(0.25,0.125) | 382.434402 | 1.0682636 | 1.22136336 | 0.386575096 | 0.565685833 |
| combined_d048 | strict 0.0625 | 360.706651 | 1.00757093 | 1.42436278 | 0.429888472 | 0.51134839 |

The strongest two-mesh human-max result among these three is d0/d1=0.48/0.65: twist 0.386575096 and spine 0.565685833; its strict values are 0.429888472 and 0.511348390. This does not establish a global optimum. The default neck-witness bevel 0.65/1.10 has two-mesh twist/spine 0.281926575 / 0.524061980, with strict 0.353856653 / 0.466910823. None inherits the hotspot witness spine SF of 1.041424487.

All three fine/strict lateral and fatigue checks pass, but assignment sequence minima are 1.236935124, 1.222186639 and 1.221363356. These miss 1.25 while satisfying the native 1.20 threshold. All three area=2 observations fail the lateral 358 N and fatigue 1 targets. All combined body/tip-zone and ring-back native safety factors are retained in the full table; no human-max failure is hidden by the passing lower-torque twist floor.

## Governing peaks explain the lost transfer

The strict hotspot control spine peak is (157.028067,-9.861161) mm at unit-load stress 0.208057822 MPa/N. All three combined strict spine peaks move to (113.951102,-12.509824) mm; unit-load stresses are 0.464064012, 0.446888785 and 0.423735586 MPa/N in descending bevel-width order. The local hotspot reinforcement does not eliminate the governing stress in the combined geometry. Full hand/blade sequence peaks, unit stresses, thicknesses and twist section parameters are retained in each raw receipt. This is a native-model observation, not an independent 3D qualification.

## Native actual-export measurements

The exact native certification routines measure the exported facets. Requirement comparisons remain apex <=0.20 mm, every relevant face half-angle <=25 degrees within the first 0.4 mm edge strip, point thickness <=0.30/0.60/1.00 mm at 0.25/1/2 mm, native point-profile monotonicity and taper length >=5 mm, bore radius >=1.5 mm and positive-volume watertight census [0,0,1]. No sampled replacement of a triangle check or added tolerance is used.

| Export | Strip grid mm | Faces | Max apex mm | Max face half-angle degrees | Sharpness | Point profile | Bore radius mm | Census |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| combined_d065 | 0.05 | 613016 | 0.195138946 | 28.5434571 | False | True | 1.6 | [0, 0, 1] |
| combined_d055 | 0.05 | 533700 | 0.195138946 | 28.5434571 | False | True | 1.6 | [0, 0, 1] |
| combined_d048 | 0.05 | 470476 | 0.195138946 | 28.5434571 | False | True | 1.6 | [0, 0, 1] |
| combined_d065_h025 | 0.025 | 2009692 | 0.195142197 | 28.5434571 | False | True | 1.6 | [0, 0, 1] |

The 0.05 mm exports all fail sharpness at about 28.543457 degrees. Their common worst face is near (100.67,-12.76) mm at r_tip about 96.2 mm, with vertex cutting-edge distances 0 / 0.008967 / 0.015625 mm. It lies entirely within the linear micro-bevel, so widening d0 alone does not resolve that faceted-angle failure. The actual worst triangle is retained, rather than inferring sharpness from the 18 degree input angle.

The primary 0.65/1.10 construction was also fully measured using the hotspot source grid of 0.025 mm. Its measured maximum half-angle is 28.5434571059 degrees and native sharpness verdict is False; point-profile verdict is True. All physical law parameters and input genes are identical between these two primary export runs; only the exporter grid changes. FEM was not repeated because the exporter grid is not an input to its native mesher or thickness law. This comparison does not establish continuum convergence.

## Verification, scope and reproduction

`verification.json` records all control, schema, native-clamp, construction, metric-identity and unchanged-source checks. Seventy-seven production source/requirement/build-file digests and all 19 snapshot digests are verified. Removing the two postcalculation diagnostic hooks reproduces the production FEM source exactly. `input_provenance.json` identifies every external source and its stable snapshot; `run_provenance.json` records compiler and dependency identities.

Run `taskset -c 10,11 env CARGO_BUILD_JOBS=2 bash target/gate-feasibility-bound/unified_study/reproduce.sh` to rebuild locally and rerun both witness controls, all three combined four-mesh cases and all four completed native export measurements. It uses retained local input snapshots and the existing lane release dependencies. `SHA256SUMS` seals the report and artifacts; the report SHA-256 is recorded in the handoff and changelog.

This is a numerical prototype study. Independent 3D P2 solid/cross-solver, GCI adjudication and support/buckling batteries remain unassessed. The inherited ring collar extends beyond nominal flat stock and has no process qualification here. No production certificate, RS-v2 all-hard pass, global infeasibility proof, threshold relaxation, source promotion or push follows from these measurements. Tracked git status is clean.

The next structural investigation should address the combined spine peak near x=114 mm while preserving the blade/hand sequence balance, and retain actual-export sharpness checks for every construction change. The separate source witnesses are useful controls, but their passing values cannot be pooled into one qualified candidate.
