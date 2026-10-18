# Phase-8: distal flat-bottom structural fuse pocket on Q7

**Result: no joint Phase-8 witness on the supplied finite grid.** Every tested grid profile fails at least one structural target on a required mesh. Complete native export checks are measured on the best all-three-mesh representative in each arm. The report distinguishes unchanged local thickness from the measured global stress response. This is finite-grid evidence, not a global or continuous infeasibility proof.

In 1,106 retained pocketed FEM observations, 0 global lateral blade peaks lie on the flat floor. The floor is present in every profile, but the intended flat-floor fuse does not govern the measured response. The legal CP alternatives also increase worst hand stress rather than lowering it; requested ring widening fails native geometry/domain checks.

Worktree `.`, branch `audit/gate-feasibility-bound`, execution baseline `d046093432864ac3cc619dc40c177c452e1e967a`. Scientific commands use `nice -n 10 taskset -c 10,11`, CARGO_BUILD_JOBS=2, faer, CARGO_INCREMENTAL=0 and the existing lane target/dependencies. The host already ran at niceness 19, so children retain effective niceness 19; receipts record the effective value. No native load, mask, material, solver, four-sample thickness rule, gate or threshold was changed.

## Exact construction and reproduced controls

Parent Q7 is Phase-6 `split_hand_blade_literal_neckdrop_w3_s0.5_0.85_b0_qual.csv`, retained unchanged through Phase-7. Seventeen stable Phase-7 inputs/sources/control receipts are frozen in `inputs/`. Both arms retain the original narrow distal window, stock=6 mm, spine start=42 mm, bevel d0/d1=0.48/0.65 mm, directional POINT_SHAPE=3 and POINT_LATERAL=2, and the existing full-neck construction. All genes remain Q7 in the original arm. Only CP6/7/8 change in the selected reinforcement arm. The CSV requires the recorded `_case.json` prototype law; it is not a standalone production thickness genotype.

The detailed assignment enumerations govern the endpoints (118/122/126 to 150/156 mm); the title shorthand 120..154 is not silently substituted. Let S(u)=clamp(u,0,1)^2*(3-2*clamp(u,0,1)). ds is the minimum exact segment distance to the full smooth spine and the actual sharpened spine; de is the exact actual inner-edge segment distance. The only physical-law extension from Phase-7 makes its longitudinal endpoints configurable:

```text
w = S((x-x_start)/1.5) * S((x_end-x)/1.5)
    * S((ds-gs)/1.5) * S((de-ge)/1.5)
w = 0 when x <= x_start, x >= x_end, or the pocket is disabled
Tnew(q) = T_parent(q) - max(T_parent(q)-t_pocket, 0) * w
```

The law removes material symmetrically about the exporter midplane. It preserves the existing bevel/point geometry rather than filling it to 6 mm. Guard bands ds<=gs and de<=ge and the entire upstream/downstream material are mathematically unchanged within each arm; a fully blended original 6 mm interior reaches exactly the specified flat floor. Native FEM uses its unchanged four-sample element mean. The new flat-floor peak diagnostic requires both centroid mask weight=1 and element mean thickness equal to the requested floor. It is an observation, not a load or stress operator change.

| Control | Areas mm² | Comparable scalars | Maximum relative scalar error |
| --- | --- | --- | --- |
| q7_control | [0.25, 0.125, 0.0625] | 60 | 2.01847238043e-13 |
| phase7_control | [0.25, 0.125, 0.0625] | 60 | 5.97510588496e-14 |

Both controls reproduce before the new parameter sweep: disabled-pocket Q7 and the prior Phase-7 x=102..165 primary. Source Q7 native export reference is 24.32422429587 degrees / apex 0.195144636429 mm, rather than the older assignment shorthand 24.18 degrees. A source export pass is never substituted for a pocketed export measurement.

During execution, an external checkpoint advanced HEAD from d046093 to 348a649, adding only the prior Phase-4 archive and V1–V4 reports. This runner did not create that commit. Frozen production source digests remain unchanged, and the original execution baseline/binary inputs are retained. checkout_observation.json records the exact changed paths.

## Reinforcement arm: native bounds, geometry and measured hand stress

Native ring_wall upper bound is 7 mm. Requested 7.2 is retained as a noncanonical probe (clamped by native loading) and is ineligible. Ring 6.8 and the intermediate 6.6 fallback are canonical but fail unchanged geometry. No native bound or predicate is widened. CP6 maximum is 3 mm; the bounded +0.6 arm saturates that coordinate at 3 while adding +0.6 to CP7/8. The exact gene changes are in arm_geometry.json and each arm CSV. CP6/7/8 lie in the actual blade-coordinate portion of the outline, so the assignment’s proposed hand reinforcement effect is measured rather than assumed.

| Probe | Canonical | Native geometry eligible | Rejection |
| --- | --- | --- | --- |
| ring66 | True | False | GripOutOfBand 60.0 |
| ring68 | True | False | GripOutOfBand 59.8; Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100 |
| ring72_requested | False | False | GripOutOfBand 59.6; Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100 |
| cp03 | True | True | none |
| cp06_bounded | True | True | none |

No-pocket lateral hand stresses (MPa/N):

| Body | 0.25 | 0.125 | 0.0625 | Worst |
| --- | --- | --- | --- | --- |
| Q7 | 1.532579612 | 1.522161879 | 1.520273987 | 1.532579612 |
| cp03 | 1.546040355 | 1.583451275 | 1.542302652 | 1.583451275 |
| cp06_bounded | 1.580774730 | 1.552079807 | 1.551806553 | 1.580774730 |

Selected second arm: `cp06_bounded`, by lowest worst hand stress over all three meshes among the legal reinforcement alternatives. Both alternatives increase the worst hand stress relative to Q7; neither approaches 1.42. This supersedes the preliminary first-mesh-only preference. The full paired sweep therefore tests the best measured legal alternative without claiming that it widens the fuse corridor.

## Complete supplied grid and hierarchical qualification

Each arm has 432 Cartesian profiles: x_start={118,122,126} × x_end={150,156} × gs={4.5,5.25,6} × ge={4,4.75,5.5} × t_pocket={1.6,1.8,2,2.1,2.2,2.3,2.4,2.6}. All 864 are evaluated at area 0.25 mm². Every first-mesh joint pass receives area 0.125, and every two-mesh joint pass receives strict area 0.0625. Additional top-margin and maximum-sequence representatives in each arm receive all three meshes even when a prior mesh fails. The selection reasons are retained in qualify_selection.json. A profile failing an earlier required mesh cannot become a three-mesh pass.

| Arm | Grid profiles | First-mesh passes | Second mesh evaluated | Two-mesh passes | Strict evaluated | Three-mesh passes |
| --- | --- | --- | --- | --- | --- | --- |
| q7 | 432 | 116 | 119 | 80 | 84 | 0 |
| cp06_bounded | 432 | 31 | 35 | 0 | 4 | 0 |

Completed 1118 unbiased native FEM observations in 1112 receipts, plus five initial geometry probes and 2 complete native export measurements. Control observations are included in those totals. Each arm’s node/triangle counts match its own fixed-planform no-pocket controls at each mesh. No meshing or stress-recovery setting varies with pocket parameters. Full results and actual peak diagnostics are in fem_summary.csv and raw matched receipts.

## Best complete three-mesh cases

Each row contains minima for one exact profile over the three required meshes. Profiles are ranked by the minimum of sequence/1.25, spine/1, lateral/358 and fatigue/1. No metric is borrowed from a different profile. The full list is in summary.json; the table retains the strongest eight and the best exported representative from each arm.

| Case | Sequence >=1.25 | HM spine >=1 | Lateral >=358 N | Fatigue >=1 | All structural |
| --- | --- | --- | --- | --- | --- |
| fuse_q7_x126_156_s4.5_e5.5_t2.2 | 1.250619062 | 1.007135462 | 337.700838841 | 0.943308213 | False |
| fuse_q7_x126_150_s4.5_e5.5_t2.2 | 1.252314587 | 1.007134972 | 337.443124293 | 0.942588332 | False |
| fuse_q7_x126_156_s5.25_e4.75_t2.4 | 1.252378398 | 1.007214836 | 336.321353776 | 0.939454863 | False |
| fuse_q7_x122_156_s4.5_e5.5_t2.2 | 1.251026708 | 1.005448666 | 336.299095104 | 0.939392687 | False |
| fuse_q7_x122_150_s4.5_e5.5_t2.2 | 1.252647011 | 1.005447996 | 336.084897868 | 0.938794364 | False |
| fuse_q7_x126_150_s5.25_e4.75_t2.4 | 1.255909973 | 1.007215958 | 335.712473002 | 0.937754061 | False |
| fuse_q7_x126_156_s4.5_e5.5_t2.1 | 1.260264094 | 1.007142445 | 335.047599437 | 0.935896853 | False |
| fuse_q7_x122_156_s5.25_e4.75_t2.3 | 1.253441047 | 1.005622366 | 334.987766031 | 0.935729719 | False |
| fuse_cp06_bounded_x118_156_s5.25_e4.75_t1.8 | 1.291598952 | 1.055977279 | 322.374867725 | 0.900497794 | False |

Primary `fuse_q7_x126_156_s4.5_e5.5_t2.2`: x=126..156 mm, gs=4.5, ge=5.5, t_pocket=2.2 mm; all blends remain 1.5 mm.

| Area mm² | Sequence | HM spine | Lateral N | Fatigue | Spine peak x,y mm |
| --- | --- | --- | --- | --- | --- |
| 0.25 | 1.250619062 | 1.038357452 | 380.959221083 | 1.064142936 | [100.51466426636259, -12.322975014499171] |
| 0.125 | 1.258612740 | 1.063142596 | 381.130439334 | 1.064621204 | [107.83060263944975, -12.004190305608311] |
| 0.0625 | 1.422207709 | 1.007135462 | 337.700838841 | 0.943308213 | [107.85295710791556, -12.002552918124557] |

Human-max twist remains recorded separately in raw receipts and summary minima; it is not a Phase-8 closure claim. Preserving local flange thickness does not guarantee identical recovered stress after changing neighboring stiffness. The tables above use actual recomputed native stress factors.

## Fuse corridor and actual governing material

For native global lateral peak sigma_global, lat_rob=730.175/sigma_global. The reported lateral blade/hand ratio is sigma_blade/sigma_hand; the native sequence metric is the minimum across lateral, pry and chord-torsion cases. Therefore the approximate blade corridor [1.25*hand_stress, 730.175/358] is useful only with the actual hand stress and confirmation that the lateral case and blade peak govern. Upper bound is 2.039595 MPa/N; [1.92,2.02] is the requested interior diagnostic target, not an acceptance-threshold change. All load cases and the actual global native metrics remain authoritative.

Primary lateral peaks below distinguish full flat-floor elements from the global blade peak. A peak at pocket weight=0 or mean thickness different from t_pocket does not establish a flat-floor structural fuse. Native four-sample averaging near a bevel can retain mesh-dependent mean thickness even though the pocket interior has a true flat floor.

| Area | Hand stress | Corridor lower | Blade stress | Blade peak x,y | Element mean t | Pocket weight | Flat-floor peak stress | Flat-floor elements |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0.25 | 1.532580034 | 1.915725043 | 1.916673805 | [129.01902128723077, -15.545498765624169] | 5.129238705 | 0.000000000 | 1.119212440 | 802 |
| 0.125 | 1.522162220 | 1.902702776 | 1.915812763 | [129.90111255980253, -15.828935275107796] | 5.936204993 | 0.000000000 | 0.997266841 | 1630 |
| 0.0625 | 1.520307822 | 1.900384777 | 2.162193504 | [128.4582054785095, -15.420706329362323] | 6.000000000 | 0.000000000 | 0.992679240 | 3311 |

Across all retained sweep observations, the following counts test whether the requested flat floor actually governs. Finer-mesh rows are the hierarchical/representative subset, not a complete fine-mesh grid.

| Body / mesh | Observations | Has flat-floor elements | Blade peak on flat floor | Blade peak weight=0 | Blade stress in [1.92,2.02] |
| --- | --- | --- | --- | --- | --- |
| cp06_bounded / 0.25 | 432 | 432 | 0 | 237 | 50 |
| cp06_bounded / 0.125 | 35 | 35 | 0 | 35 | 0 |
| cp06_bounded / 0.0625 | 4 | 4 | 0 | 4 | 0 |
| q7 / 0.25 | 432 | 432 | 0 | 432 | 100 |
| q7 / 0.125 | 119 | 119 | 0 | 119 | 64 |
| q7 / 0.0625 | 84 | 84 | 0 | 84 | 0 |

These observations test the proposed mechanism directly. Flat-bottom thickness alone does not force that region to govern: the preserved thicker edge/spine material can carry larger recovered stress. The report does not infer mesh independence from the flat floor, and it does not hold global spine margin fixed merely because material before x_start is unchanged.

## Complete native exported-solid measurements

Each export uses exactly the corresponding FEM genes and physical-law config. Production certification::measure is included directly and computes complete apex/half-angle/point/bore measurements. Census and positive volume come from the actual native solid.

| Case | Half-angle deg <=25 | Apex mm <=0.20 | Point pass | Bore mm >=1.5 | Census | Volume mm³ | Export pass | Joint pass |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fuse_q7_x126_156_s4.5_e5.5_t2.2 | 24.324224295870 | 0.195144636429 | True | 1.600000000 | [0, 0, 1] | 30764.031034 | True | False |
| fuse_cp06_bounded_x118_156_s5.25_e4.75_t1.8 | 25.914071314065 | 0.195143726079 | True | 1.600000000 | [0, 0, 1] | 30453.172434 | False | False |

Full measured triangle vertices, edge distances, point-profile samples and actual solid census remain in each export receipt. Export passes and structural passes are always combined only for the same complete profile.

## Verification, reproduction and captain handoff

Verified 77 production source/requirement/build-file digests and 17 frozen input snapshots. Phenotype/exporter are byte-identical to Phase-7; reverting only the endpoint variables and flat-sample audit recovers its pocket law exactly. Removing the two postcalculation observation calls recovers production FEM exactly. Native certification is included directly. Source controls, canonical inputs/native geometry, fixed permitted genes, all enumerated parameter bounds, analytic zero upstream mask plus sampled protected-band/outside-extent deltas, protected original 6 mm stock, unchanged-body mesh counts, native metric identities, all-three-mesh coverage policy, effective niceness, executable hashes and matched FEM/export inputs are checked. Eight native dependency digests and all original Phase-4/5/6/7 manifests are verified at seal time.

Run `nice -n 10 taskset -c 10,11 env CARGO_BUILD_JOBS=2 bash target/gate-feasibility-bound/unified_study_v5/reproduce.sh`. This reuses the lane target and retained dependencies, rebuilds the isolated harness from frozen inputs, reproduces controls, executes the complete paired 864-profile grid and hierarchical mesh checks, completes representative three-mesh profiles, measures both arm exports, analyzes, reports and reseals. SHA256SUMS covers the report and stable artifacts; REPORT_SHA256 identifies the report.

Phase-8 evaluation is complete. Await captain_jarambit for the next scientific milestone. The finite grid and the measured peak diagnostics are the evidence for that decision; no global infeasibility or all-hard production certificate is claimed. This Phase-8 runner made no production edits, commit, push, PR, stacking, branch/worktree change or deletion. Shared Git metadata and COMMS are read-only in this session, so the local queued handoff is not posted externally.
