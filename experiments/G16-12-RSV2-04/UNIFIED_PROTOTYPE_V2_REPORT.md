# Phase-5: spine-web-preserving unified prototype and heel-bevel evaluation

**Result: partial closure, no all-target candidate.** The full critical-source import with a narrow distal window restores HM spine >=1 on all three requested meshes, while lateral >=358 N and fatigue >=1 remain satisfied. Its actual native export passes sharpness/apex, point profile, bore radius and watertight census. Sequence remains below 1.25; the requested joint margin is not closed. A larger window passes sequence on all three meshes but loses lateral/fatigue/spine margins. This is bounded sample evidence, not a global infeasibility proof.

Worktree `/home/ubuntu/jarambit_wt_diag`, branch `audit/gate-feasibility-bound`, execution baseline `d046093432864ac3cc619dc40c177c452e1e967a`. All solver/export runs and compilation use CPUs 10,11 and CARGO_BUILD_JOBS=2, explicit faer, the existing lane target and retained native dependencies. No production source, load, mask, threshold or native element-thickness averaging was changed. No push or PR.

## Sources and controls

The assigned `critical_fem_3.csv`, source construction law/generator, original FEM/export receipts and invocations were snapshotted from lane-RSV2-07-hm. Ten Phase-4 harness/input/control files were frozen separately; their inherited neck/bevel construction comes from lane-G16-06-RSV2-08. Reproduction reads these stable snapshots, never mutable external lanes. `source_provenance.json` maps every source and digest.

Control parity: critical all four mesh areas, maximum comparable scalar relative error 1.39247839899e-13; neck area 0.25, 8.46174707071e-14. Exact critical angle-only exported-face control reproduces 23.931704291521 degrees against its retained full native export. This new angle-only control does not rerun native apex/point scans; the two reported prototype exports below do.

## Imported construction and bounded search

Every qualified candidate preserves the full exact source CP0..39 and SPINE_SHARP_OFFSET=42 mm. `full_cp.csv` imports them onto the neck witness genome. The `source_*` family imports the complete source genome (also preserving its -1.0075235746947224-degree drop, source ring/neck/edge parameters), then overrides only the documented window genes. The primary `source_distal_d048.csv` differs from the complete critical source only in WIN_N/S0/S1/W/FILL. Its companion `_case.json` records exact config and seed. A CSV requires this prototype construction law; the production phenotype alone does not reproduce it.

The unchanged neck source removes only the comet slot and retains the ring/cairn. N(q)=R(q)+(6-R(q))*S((x-ring_od/2)/3), B(q)=N(q)+(6-N(q))*S((x-heel_x)/5), with S clamped cubic smoothstep and R native raster thickness. Outside the bevel/point cap, the sampled neck is exactly 6 mm. Cutting-edge d is the exact minimum distance to native cut_in/cut_sp segments. For d<=d0, E=0.175+2*d*tan(18 deg); for d0<d<d1, blend E to B using S((d-d0)/(d1-d0)); otherwise use B. Apply the exact source POINT_SHAPE=3 directional cap at a=hypot(axial,2*lateral), and the inherited unchanged bore collar/floor.

Window screens vary native width 3/4/5/6/8/10 mm, fill 0.45/0.65/0.80 and legal span endpoints, plus the explicitly assigned belly placement coordinate. Belly offsets 0/1/2/3 mm were sampled toward the closest smooth inner-edge point before unchanged clearance sizing. The harness allows 0..4 mm. Diagnostic neck-fillet, ring-wall and drop probes preserve all imported CPs. No sampled ineligible input can qualify: 22-mm fillet requests clamp to 18 mm; 7-mm ring-wall cases fail grip/holder geometry; excessive belly movement fails the native ligament floor; some distal spans cause the native window generator to omit the hole. All are retained and labeled in fem_summary.csv.

The inherited source comment and Phase-4 prose incorrectly call the window rim 3.2 mm. The actual unchanged constant is WIN_RIM=1.5 mm, and the unchanged native hole/outline ligament gate is 3.0 mm. Clearance uses actual compound-bevel extent plus WIN_RIM; native geometry gates still run. No threshold was changed to accept a window.

Completed 80 unbiased native FEM evaluations in 65 receipts, 14 angle-only actual-solid screens, and 2 complete native export measurements. Full per-case genes, configs, CPU affinity, solver, metrics and peak coordinates are retained. `fem_summary.csv` contains every FEM observation and rejection. Six structurally distinct cases have all three assignment meshes; coarse area 2 is used only for the critical control.

## Three-mesh qualification

Values below are minima across area 0.25, 0.125 and 0.0625 mm². No mesh result or source-witness metric is pooled across candidates.

| Case | HM spine >=1 | Lateral >=358 N | Fatigue >=1 | Sequence >=1.25 |
| --- | --- | --- | --- | --- |
| source_distal_d048 | 1.025938124 | 370.667480404 | 1.035394759 | 0.721729333 |
| source_distal_qual | 1.024063789 | 370.667628631 | 1.035395173 | 0.655045539 |
| source_window_qual | 0.670047674 | 359.954221250 | 1.005469143 | 0.824638838 |
| neck_best_sequence_qual | 0.668519082 | 373.868732328 | 1.044336896 | 1.010751149 |
| balance_qual | 0.691023918 | 288.280353622 | 0.805260733 | 1.151505606 |
| sequence_qual | 0.500501996 | 274.420017627 | 0.766544310 | 1.379607087 |

Primary input: complete critical source genome, WIN_N=1, WIN_S0=0.5, WIN_S1=0.8, WIN_W=3 mm, WIN_FILL=0.65, belly offset=0; d0/d1=0.48/0.65 mm; actual export grid 0.05 mm. Its window occupies x=157.902223..175.524334 mm, area 57.783650 mm². It removes no material in the x=110..125 mm concern region; sampled minimum distance to smooth spine is 5.700413 mm. Native planform/aesthetic/ligament checks and canonical roundtrip pass.

| Area mm² | HM spine | Lateral N | Fatigue | Sequence | Spine peak x,y mm |
| --- | --- | --- | --- | --- | --- |
| 0.25 | 1.087412474 | 404.380495505 | 1.129566168 | 0.845945840 | [147.96930010173884, -4.020606561170063] |
| 0.125 | 1.120508478 | 370.667480404 | 1.035394759 | 0.721729333 | [107.82623571495415, -11.897197860062489] |
| 0.0625 | 1.025938124 | 384.989692127 | 1.075401351 | 0.808484779 | [150.2228631945004, -5.45997536201623] |

The sequence failure remains lateral, governed by the ring-root/hand peak around x=17 mm. Full source CP import changes that region as well as the blade. Window relocation preserves the requested spine web but does not inherit the Phase-4 neck witness sequence ratio. The source itself has sequence <1.25. The larger-window `sequence_qual` is intentionally retained as a competing failure, not a fallback passing input. `diagnostic_peaks.sequence_cases` records each hand/blade numerator, denominator, location and local thickness. Human-max twist also remains below 1 and is explicitly retained in summary.json; this Phase-5 result is not an all-hard qualification.

## Actual exported triangles and native measurement

Both source functions already use exact cutting-edge segment distance; no separate heel-specific distance correction exists to copy. The repaired facet result comes from importing the complete CP/spine geometry and retaining sufficient linear-bevel width for the actual triangles inspected by the native 0.4-mm band. In the partial-import full-window case, d0/d1=0.45/0.55 with h=0.05 produces a 78.104897-degree transition triangle whose edge distances range 0.399692..0.467454 mm: it intersects the inspection band although two vertices lie outside. Changing drop or the native edge-angle genes alone does not fix that case.

The same full-CP construction at d0/d1=0.48/0.65, 0.55/0.85 and 0.65/1.10 yields 24.183533 degrees on actual faces at h=0.05. Keeping 0.45/0.55 and refining the grid to the source h=0.025 gives 24.171053 degrees for the partial-source geometry, while the complete source geometry reproduces 23.931704 degrees. These are separate measured solids. No continuum-convergence claim is made.

| Case | Half-angle deg <=25 | Apex mm <=0.20 | Point pass | Bore radius mm >=1.5 | Census | All export checks |
| --- | --- | --- | --- | --- | --- | --- |
| source_distal_d048 | 23.843282600750 | 0.195143274343 | True | 1.57499999999993 | [0, 0, 1] | True |
| bevel_d0.48 | 24.183533328086 | 0.195141860758 | True | 1.599999999999966 | [0, 0, 1] | True |

`worst_edge_face` retains the measured worst triangle, per-vertex exact edge distance and tip distance. The full native `certification::measure` computes apex, half-angle, point profile and bore radius directly; angle-only screening never substitutes for it in the table above. Both full exported solids have positive volume. The primary FEM and export receipts have identical genes and section config.

## Verification, reproduction and next step

Verified 77 production source/requirement/build-file digests and 18 stable source snapshots. FEM differs from production only by the two existing postcalculation observation hooks; removing them reproduces production source exactly. Native certification is included directly. Prototype exporter is byte-identical to Phase-4. Metric identities, CPU affinity, jobs, source parity, canonical/native-geometry checks, full CP/start import, unchanged dependency digests and matching primary FEM/export input are recorded in verification.json/run_provenance.json.

Run `taskset -c 10,11 env CARGO_BUILD_JOBS=2 bash target/gate-feasibility-bound/unified_study_v2/reproduce.sh` in this lane. It rebuilds using the same retained native dependencies, reexecutes the controls/screens/qualification/native exports from frozen seeds and regenerates the report/seal. It does not edit production code, push or open a PR. SHA256SUMS includes this report and every stable study artifact; REPORT_SHA256 identifies the report.

Assignment evaluation is complete; joint target closure remains failed because sequence cannot be retained in the tested spine-preserving construction. The next milestone needs captain direction on the ring-root/hand sequence balance while retaining the full imported spine geometry, native thresholds and the verified actual-export bevel. There is no authorization here to change native masks or loads. Independent 3D/cross-solver/GCI/support qualification and manufacturing process qualification remain unassessed. Phase-4 evidence stays sealed and unchanged.
