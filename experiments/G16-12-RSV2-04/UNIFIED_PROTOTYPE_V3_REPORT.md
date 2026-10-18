# Phase-6: split hand/blade control-point synthesis

**Result: bounded negative joint-closure result.** The literal-index narrow-window hybrid retains HM spine >=1, lateral >=358 N and fatigue >=1 on all three required meshes, and passes every measured native export check. Its sequence minimum remains below 1.25. Neither the literal nor the coordinate-based interpretation supplies an all-target candidate. No threshold, load or mask was changed. These samples do not prove global infeasibility.

All execution and compilation used CPUs 10,11, CARGO_BUILD_JOBS=2, explicit faer, CARGO_INCREMENTAL=0 and the existing lane target/dependencies. Worktree `/home/ubuntu/jarambit_wt_diag`, branch `audit/gate-feasibility-bound`, baseline `d046093432864ac3cc619dc40c177c452e1e967a`. Production sources, requirements, native FEM operators and native actual-export certification remain unchanged.

## Assignment interpretation and frozen parents

The assignment lists CP0..5 and CP20..25 as hand points at x<95 mm. The repository C.4 reference starts at the tip: CP0=(188,-54), CP5=(141.5,-18.6), CP18=(16,-11), CP23=(18,12.8). The index list therefore conflicts with its physical description. Both interpretations were executed and kept separate:

- Literal: preserve CP0..5/CP20..25 from the neck witness; import CP6..19/CP26..39 from critical_fem_3.
- Spatial: preserve CP10..29 from the neck witness; import CP0..9/CP30..39 from critical_fem_3, matching reference x>=95 mm.

Both import SPINE_SHARP_OFFSET=42 mm and retain the neck witness RING_WALL=6.4 mm, neck fillet=8 mm, NECK_L and all other nonblade/nonwindow genes. There is no NECK_W gene. Existing RAIL_W is preserved; P1 stays off, so its rail/pocket controls are inactive. No surrogate neck-width law was added. All scientific harness files are byte-identical to Phase-5. `hybrid_assembly.json` contains exact CP coordinates and parent-index mappings.

The frozen CSVs exactly match the two named source-lane CSVs when checked during this execution: neck SHA-256 `52b795486b3c7f72ed9c9e06f57888ef166afdc22434118a0a821004ccc016ed`; critical SHA-256 `070da10ab3c66dced8592aabf4a06ec2c68708a3f92df43e64685bbe6019da15`. Eighteen stable Phase-5 source/input/control files were copied into `inputs/`. The 538-artifact Phase-5 seal remains unchanged.

Control reproduction: Phase-5 primary on 0.25/0.125/0.0625 mm², maximum comparable scalar relative error 6.47117865486e-13; neck witness area 0.25, 1.73365956712e-14. The actual-solid angle-only Phase-5 control exactly reproduces 23.84328260075 degrees. The assignment combined two prior export values: 24.183533 degrees / bore 1.60 mm belonged to the separate Phase-5 bevel_d0.48 case; source_distal_d048 measured 23.843283 degrees / bore 1.575 mm. Phase-6 results always use matched-input receipts.

## Bounded construction and search

The unchanged Phase-5 section law retains the full sampled 6 mm neck, omits the comet slot, keeps ring/cairn, uses POINT_SHAPE=3 / POINT_LATERAL=2 and the exact segment distance to cut_in/cut_sp. The linear bevel is E=0.175+2*d*tan(18 deg) through d0=0.48 mm; cubic smoothstep blends it to the unchanged base by d1=0.65 mm. The native bore collar/floor and exact directional point cap are retained. Export outline tolerance is 0.001 mm and edge grid is 0.05 mm. Native FEM mesh construction, four-sample element thickness, material, fixed nodes, distributed 3427 N spine load and all thresholds are unchanged.

The four initial families combine literal/spatial CP assignment with either parent blade-drop setting: 0 or -1.0075235746947224 degrees. Four solid controls establish the effect of a window; they cannot qualify as a windowed input. Eight initial family observations, 32 endpoint-window screens and 36 local refinements sample widths 3/3.5/4.25/5.5 mm, S0=0.35/0.4/0.425/0.45/0.5, S1=0.75/0.8/0.85 and belly offsets 0/1/1.5/2 mm. WIN_FILL stays 0.65. Requested S0=0.55 exceeds the native maximum 0.5; all accepted tests respect 0.5. A final 12-case probe interpolates only blade drop between the two source settings (25/50/75%), preserving the exact CP split and all hand parameters. This probe does not improve joint closure.

Positive belly offset follows the closest smooth inner-edge point before the inherited window clearance calculation. Actual WIN_RIM remains 1.5 mm and the native hole/outline ligament gate remains 3.0 mm. Some offset=2 windows fail that gate or are resized by native clearance; rejected cases stay in the evidence and never qualify. No acceptance condition was relaxed.

Completed 113 unbiased native FEM evaluations in 97 receipts, 4 angle-only actual-solid screens, and 2 complete native export measurements. All inputs, rejected predicates, invocations, per-case metrics and peak coordinates are retained. `fem_summary.csv` contains every FEM observation. No area=2 mesh substitutes for any assignment mesh.

## Three-mesh qualification

Selection uses the minimum normalized first-mesh margin over sequence/1.25, spine/1, lateral/358 and fatigue/1, with literal, spatial, sequence-passing and parent-drop refinements retained separately. Seven cases were checked at all three areas. A literal narrow-window case was retained alongside its centered counterpart because their first-mesh scores tie within 1e-6. Qualification repeats every selected area-0.25 observation and additionally solves area 0.125 and strict 0.0625. Selection reasons are in `qualify_selection.json`.

| ID | CP split | Drop deg | WIN_W mm | S0..S1 | Belly mm |
| --- | --- | --- | --- | --- | --- |
| Q1 | spatial | -1.007523575 | 3.0 | 0.5..0.85 | 0.0 |
| Q2 | literal | 0.000000000 | 4.25 | 0.425..0.8 | 0.0 |
| Q3 | spatial | 0.000000000 | 5.5 | 0.35..0.75 | 0.0 |
| Q4 | spatial | 0.000000000 | 3.0 | 0.35..0.75 | 2.0 |
| Q5 | spatial | -1.007523575 | 3.0 | 0.45..0.75 | 1.0 |
| Q6 | spatial | -0.251880894 | 3.0 | 0.5..0.85 | 0.0 |
| Q7 | literal | 0.000000000 | 3.0 | 0.5..0.85 | 0.0 |

The following values are minima across all three required meshes, with assignment thresholds shown in the headers. Minima are never pooled across inputs.

| ID | Sequence >=1.25 | HM spine >=1 | Lateral >=358 N | Fatigue >=1 |
| --- | --- | --- | --- | --- |
| Q1 | 0.847189868 | 0.989560175 | 425.747473774 | 1.189251084 |
| Q2 | 0.973089140 | 0.747181833 | 396.804033236 | 1.108402646 |
| Q3 | 1.138881188 | 0.614813663 | 362.109002502 | 1.011488148 |
| Q4 | 0.930110762 | 0.875168879 | 367.058603051 | 1.025313991 |
| Q5 | 0.846750710 | 0.989561242 | 425.748161327 | 1.189253004 |
| Q6 | 0.910518812 | 0.960911709 | 425.224081744 | 1.187789080 |
| Q7 | 0.972661735 | 1.007021689 | 396.501963522 | 1.107558867 |

Case mapping:

- Q1: `split_hand_blade_spatial_criticaldrop_w3_s0.5_0.85_b0_qual.csv` (best normalized first-mesh joint margin).
- Q2: `split_hand_blade_literal_neckdrop_center_qual.csv` (best literal-index joint margin).
- Q3: `split_hand_blade_spatial_neckdrop_w5.5_s0.35_0.75_b0_qual.csv` (best spine while first-mesh sequence/lateral/fatigue pass).
- Q4: `split_hand_blade_spatial_neckdrop_w3_s0.35_0.75_b2_qual.csv` (strongest neck-drop spatial tradeoff).
- Q5: `split_hand_blade_refine_spatial_criticaldrop_w3_s0.45_0.75_b1_qual.csv` (strongest refined spatial tradeoff).
- Q6: `split_hand_blade_spatial_dropmix0.25_w3_s0.5_0.85_b0_qual.csv` (strongest interpolated-drop tradeoff).
- Q7: `split_hand_blade_literal_neckdrop_w3_s0.5_0.85_b0_qual.csv` (literal narrow-window spine witness; first-mesh joint-score tie within 1e-6).

Primary reporting case Q7 is `split_hand_blade_literal_neckdrop_w3_s0.5_0.85_b0_qual.csv`, selected by the largest minimum normalized structural margin over all three meshes. It follows the assignment's literal CP index list, uses the neck-source blade drop, WIN_N=1, WIN_W=3 mm, S0/S1=0.5/0.85, fill=0.65, belly=0, and the unchanged 0.48/0.65 mm bevel. It has one 66.327147562 mm² window. Measured window geometry: `[{"spine_ligament_110_125_mm": null, "spine_ligament_mm": 5.333134602067238, "xmax": 177.88011600165694, "xmin": 158.56434752890576}]`.

| Area mm² | Sequence | HM spine | Lateral N | Fatigue | Sequence case | Spine peak x,y mm |
| --- | --- | --- | --- | --- | --- | --- |
| 0.25 | 1.003791887 | 1.038248515 | 474.635231835 | 1.325810483 | lateral | [100.51466426636259, -12.322975014499171] |
| 0.125 | 0.972661735 | 1.063024520 | 479.695733997 | 1.339946111 | lateral | [107.83060263944975, -12.004190305608311] |
| 0.0625 | 1.211321681 | 1.007021689 | 396.501963522 | 1.107558867 | lateral | [107.85295710791556, -12.002552918124557] |

The literal narrow-window input passes spine/lateral/fatigue across all three meshes but fails sequence on every mesh. The geometric split with critical drop gives a first-mesh spine pass but strict spine=0.9895602; its nearby refined witness is also below 1 on strict. The spatial wide-window case passes sequence only at area 0.25 (1.3124048); its finer values are 1.1388812/1.1507114 and spine is below 1 throughout. All seven qualifications fail sequence>=1.25 across the required levels.

Restoring parent hand control points improves some observations but does not transfer the source witness margins as a complete set. The blade window also changes the mesh and stress distribution; first-mesh successes are not evidence of convergence. Raw `diagnostic_peaks.sequence_cases` records each hand/blade stress numerator, denominator, location and local thickness. The three-mesh test supplies the requested numerical observations; independent GCI/convergence adjudication was not performed. Human-max twist remains below 1 in the retained metrics and is not a claimed closure.

## Complete native export measurements

Two complete production `certification::measure` evaluations use the same genes and section configuration as their corresponding three-mesh FEM receipts. The actual solid supplies sharpness/apex, point profile, bore radius, volume and watertight census. Angle-only screens are kept separate and do not replace these measurements.

| ID | Half-angle deg <=25 | Apex mm <=0.20 | Point pass | Bore mm >=1.5 | Census | All export checks |
| --- | --- | --- | --- | --- | --- | --- |
| Q7 | 24.324224295870 | 0.195144636429 | True | 1.600000000 | [0, 0, 1] | True |
| Q4 | 24.099508750940 | 0.195133307205 | True | 1.600000000 | [0, 0, 1] | True |

Both solids have positive volume. Their `worst_edge_face` records actual vertices, edge distances and tip distances; full point-profile samples and geometry census are retained in the raw export JSON. The Phase-5 actual-export repair is retained for these inputs, but sequence remains unresolved.

## Verification, reproduction and handoff

Verified 77 production source/requirement/build-file digests and 18 stable source/input snapshots. Compound law, phenotype, FEM, exporter, diagnostics and main harness are byte-identical to Phase-5; only module paths, immutable provenance and generated inputs differ. Removing the two existing observation hooks reproduces production FEM source exactly. Native certification is included directly. CP parent ownership, preserved hand genes, canonical inputs, native geometry, actual window bounds, metric identities, source parity, repeated first-mesh values and matched FEM/export genes/configs are checked in `verification.json`. `run_provenance.json` supplies compiler, dependency/executable digests, CPU affinity and prior-seal checks.

Run `taskset -c 10,11 env CARGO_BUILD_JOBS=2 bash target/gate-feasibility-bound/unified_study_v3/reproduce.sh` to rebuild and rerun the entire study using stable local snapshots and the existing lane dependencies. The report and stable artifacts are sealed by SHA256SUMS; REPORT_SHA256 supplies the report digest. No production edits, pushes, PRs, worktree/branch changes or deletions were made. Full repository before-push gates were not invoked because there is no push. Shared Git metadata and shared COMMS remain outside the writable worktree; no commit or external handoff is claimed.

Phase-6 evaluation is complete, with joint target closure still failed. Await captain_jarambit direction on the hand/blade compliance tradeoff and the literal-versus-coordinate CP interpretation before a new scientific milestone. Retain native masks, loads, thresholds and all sealed Phase-4/5/6 evidence. Independent 3D/cross-solver/support/manufacturing qualification remains unassessed; this report is not an all-hard certificate or a global infeasibility proof.
