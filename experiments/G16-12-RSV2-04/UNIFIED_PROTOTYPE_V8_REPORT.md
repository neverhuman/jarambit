# G16-12-RSV2-04 Phase-11: literal CP interpolation and tip-annulus qualification

Task `G16-12-RSV2-04-p11`; execution commit `968fa148c290b9c1c01785c3d9ff1be629c7c6a9`, branch `audit/gate-feasibility-bound`. CPU affinity **10,11**, `CARGO_BUILD_JOBS=2`, native faer, requested `nice -n 10` with inherited effective niceness 19. No commit or push attempted. Assignment snapshot: `inputs/assignment.md`.

## Result

Executed all **25 independent CP interpolation geometry probes**. **10 profiles pass unchanged native domain/geometry/aesthetic predicates**, and 15 fail the native continuous-convex-spine aesthetic criterion. Every legal profile has all four requested areas **1.0 / 0.25 / 0.125 / 0.0625 mm²**, extra canonical area **4.0 mm²**, both exact-r=2 native GCI triplets and a matched complete native solid export. Total **54 native FEM observations**, including four independent source-control observations.

Joint structural/export passes on the 0.25/0.125 pair: **0**; on 0.25/0.125/0.0625: **0**; on all four requested meshes: **0**. Native export passes: **5/10**. This bounds the supplied interpolation experiment, not the continuous design domain; it is not an all-hard production certificate.

Best minimum-normalized-margin legal representative across the three fine meshes: `interp_a0.25_b0.5` (alpha=0.25, beta=0.5). Minima: sequence=1.115558658, spine=1.004806012, lateral=373.480167762 N, fatigue=1.043251509, ringback=4.925666843, HM body twist=1.062561913, global HM twist=0.425405411.

Its matched native export passes=False; measured half-angle=26.393518087 degrees against25. All five alpha=.25 profiles fail sharpness; all five alpha=0 profiles pass the complete native export checks. Apex, point, bore and positive-volume watertight census pass for all ten.

The numerical donor values do not describe the asserted large distal-tip change. CP28..31 are handle/heel points, while CP36..38 are identical between endpoints and CP35 changes by only about 0.0000673 mm. Original full-precision endpoints were honored; no substitute tip law was invented. The same source-label/coordinate mismatch is visible in the frozen live CSVs and native reference coordinates below.

## Source provenance and literal interpolation

Parent: Phase-10 native-domain `armB_s0.5_w3_x144_t2.8_s4.5_e5.5_d0.csv` and its exact `_case.json`. Donor: the full-precision historic neck candidate retained since Phase-4, independently matched against the current lane candidate and `phase3_candidates/placement_05.csv`. Its CP28..31 values exactly match the assignment’s rounded donor values. The live lane `phase5_sources/q7.csv` instead matches the parent’s eight CPs: the assignment reverses the Q7/placement_05 source labels. The numerical endpoints, source hashes and both live label bindings are frozen, so this ambiguity is not silently resolved by substituting another design.

| CP | Native reference x,y mm | Parent offset mm | Donor offset mm | Donor − parent mm |
| --- | --- | --- | --- | --- |
| 28 | [78.0, 13.0] | 0.212700402 | -0.547859518 | -0.760559919327297 |
| 29 | [86.0, 14.0] | 2.000000000 | -0.790113891 | -2.79011389127405 |
| 30 | [96.0, 11.8] | 1.730080382 | -2.769919618 | -4.5 |
| 31 | [111.0, 9.0] | 0.884531443 | -2.000780305 | -2.8853117479348 |
| 35 | [171.0, -18.4] | -1.261519917 | -1.261587171 | -6.7254023361496e-05 |
| 36 | [181.5, -29.8] | -0.133767371 | -0.133767371 | 0 |
| 37 | [188.3, -40.8] | -0.888966426 | -0.888966426 | 0 |
| 38 | [190.5, -48.0] | -0.891854636 | -0.891854636 | 0 |

The actual tip is at reference (188,-54) mm. CP28..31 at x=78/86/96/111 are near the handle/heel, not the distal tip. CP35 is at (171,-18.4); CP36..38 remain unchanged. The local native reference is authoritative for coordinates. Actual deformed curve samples at every CP index are retained in each geometry/FEM/export receipt.

Interpolation is `parent + weight*(donor-parent)`. Alpha applies only to CP28..31; beta only to CP35..38; both take `{0,.25,.5,.75,1}` independently. All other genes stay bit-identical to the selected Phase-10 parent. Parent pocket floor=2.2 mm, extent x=126..156 mm, spine/edge guards=4.5/5.5 mm, original 1.5 mm cubic smoothstep, bevel d0/d1=.48/.65 mm, 18° near-edge law, apex parameter=.175 mm, directional point shape=3/lateral=2, 6 mm stock/core/neck, and original narrow window S0/S1=.50/.85, W=3, fill=.65 are fixed. Physical FEM/exporter/pocket laws, materials, loads/masks, thresholds, mesher and native bounds are unchanged.

All 25 tuples are native-genome canonical. Native geometry/aesthetic screening precedes structural interpretation; rejected profiles are retained as failed construction probes, without a structural qualification claim. Every legal profile gets every requested mesh regardless of earlier mesh failures, to assess the proposed tip-annulus mechanism directly.

| Alpha | Betas tested | Canonical | Native geometry/aesthetic verdict |
| --- | --- | --- | --- |
| 0.0 | 0/.25/.5/.75/1 | True | PASS |
| 0.25 | 0/.25/.5/.75/1 | True | PASS |
| 0.5 | 0/.25/.5/.75/1 | True | spine not a continuous convex crescent (0.90 < C.4 0.94-0.03) |
| 0.75 | 0/.25/.5/.75/1 | True | spine not a continuous convex crescent (0.90 < C.4 0.94-0.03) |
| 1.0 | 0/.25/.5/.75/1 | True | spine not a continuous convex crescent (0.90 < C.4 0.94-0.03) |

## Metrics, source parity and native masks

Task comparisons: sequence>=1.20, HM spine>=1, lateral>=260 N, fatigue>=1, HM ringback>=1, HM body twist>=1 and global HM twist>=1. The Phase-10 lateral comparison 358 N remains available in raw metrics; lowering the lateral task comparison does not change the native fatigue law, which still requires roughly 357.9963 N for fatigue SF1. No target is made to pass by pooling different designs or meshes.

Native global HM twist is original `twist_joint_nmm / 16200`. Body and tip-annulus ratios are derived observations: `twist_body_sf * 1820 / 16200` and `twist_tipzone_sf * 1820 / 16200`. Native 4 mm tip exclusion, body r>=15 mm, annulus 4<=r<15 mm, original peak-element selection and native rectangular-section correction factors remain unchanged. A new third observation hook records these original selected elements, stress, mean thickness, radii and factors; it does not change the selected peak, force, mask, assembly or solve.

The exact Phase-10 parent reproduces all four required meshes across 80 comparable scalar observations, maximum relative error **9.92069998194e-14**, with exact geometry/thickness mesh hashes. Unchanged native metric identities and derived torque units are checked for every observation.

## Complete legal-profile results

The following minima use the original certification pair 0.25/0.125. “Joint” includes global HM twist and every listed structural comparison plus that same profile’s actual export.

| Profile | Seq | Spine | Lat N | Fatigue | Ringback | HM body | HM global | Export | Joint |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `interp_a0_b0` | 1.250619062 | 1.038357452 | 380.959221083 | 1.064142936 | 4.992111565 | 1.021661607 | 0.424203778 | True | False |
| `interp_a0_b0.25` | 1.173105411 | 0.949859609 | 383.429717635 | 1.071043836 | 4.982136730 | 1.080552394 | 0.424260227 | True | False |
| `interp_a0_b0.5` | 1.237671693 | 0.978056917 | 360.787701919 | 1.007797326 | 5.054483453 | 1.016639102 | 0.424202327 | True | False |
| `interp_a0_b0.75` | 1.027688473 | 1.005973884 | 413.793095178 | 1.155858619 | 5.060374222 | 1.111526881 | 0.424202197 | True | False |
| `interp_a0_b1` | 1.062218224 | 1.088356635 | 400.872839179 | 1.119768144 | 5.042253738 | 1.304033293 | 0.424205450 | True | False |
| `interp_a0.25_b0` | 1.291683821 | 0.864360241 | 342.924539338 | 0.957899706 | 5.047973265 | 1.115633007 | 0.417126147 | False | False |
| `interp_a0.25_b0.25` | 1.174771747 | 0.967722794 | 347.050578042 | 0.969425074 | 5.093310806 | 1.121465617 | 0.417216946 | False | False |
| `interp_a0.25_b0.5` | 1.161179701 | 1.004806012 | 373.480167762 | 1.043251509 | 5.064659652 | 1.334006309 | 0.427974545 | False | False |
| `interp_a0.25_b0.75` | 1.349036293 | 1.003697052 | 327.747364939 | 0.915504925 | 5.042363502 | 1.283523616 | 0.417125398 | False | False |
| `interp_a0.25_b1` | 1.245138114 | 0.966436353 | 373.394309463 | 1.043011678 | 5.062000588 | 1.322251048 | 0.427975124 | False | False |

Three-fine-mesh minima include 0.0625. Coarse 1.0 results remain independently retained and do not replace the original fine-mesh pair.

| Profile | Seq | Spine | Lat N | Fatigue | Ringback | HM body | HM global | Joint |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `interp_a0_b0` | 1.250619062 | 1.007135462 | 337.700838841 | 0.943308213 | 4.992111565 | 1.021661607 | 0.416720640 | False |
| `interp_a0_b0.25` | 1.146827387 | 0.949859609 | 383.429717635 | 1.071043836 | 4.982136730 | 1.057889303 | 0.423851575 | False |
| `interp_a0_b0.5` | 1.237671693 | 0.978056917 | 328.014807284 | 0.916251978 | 4.938048051 | 1.016639102 | 0.416947457 | False |
| `interp_a0_b0.75` | 1.027688473 | 0.999120923 | 370.466018317 | 1.034832009 | 5.010739331 | 1.111526881 | 0.415558802 | False |
| `interp_a0_b1` | 1.062218224 | 1.046573607 | 400.872839179 | 1.119768144 | 4.987875299 | 1.085030509 | 0.419828729 | False |
| `interp_a0.25_b0` | 1.287467026 | 0.864360241 | 342.924539338 | 0.957899706 | 5.041335248 | 1.115633007 | 0.417126147 | False |
| `interp_a0.25_b0.25` | 1.174771747 | 0.963453221 | 347.050578042 | 0.969425074 | 4.938490847 | 1.083067719 | 0.375370185 | False |
| `interp_a0.25_b0.5` | 1.115558658 | 1.004806012 | 373.480167762 | 1.043251509 | 4.925666843 | 1.062561913 | 0.425405411 | False |
| `interp_a0.25_b0.75` | 1.225281150 | 1.000282923 | 327.747364939 | 0.915504925 | 5.025492564 | 1.081709560 | 0.417125398 | False |
| `interp_a0.25_b1` | 1.158874709 | 0.966436353 | 373.394309463 | 1.043011678 | 5.001464069 | 1.082097095 | 0.423405827 | False |

Global lateral blade peak lies on the actual pocket floor in **0/50** grid/qualification observations. Actual pocket floor elements exist and protected bands remain unchanged; a governing floor or mesh-independent structural fuse is not inferred.

## Tip-annulus mechanism and mesh sensitivity

The native global torsion stress peak lies in the 4<=r_tip<15 mm annulus in **30/30** fine-mesh observations. Global HM twist ranges **0.375370185..0.453960334**, below 1.0. The native walk width at the selected stress-peak section ranges 8.650000000..9.500000000 mm; these are peak-dependent section measurements, not a fixed radial-station width or a presumed physical widening.

| Profile | Area | HM global | HM body | HM annulus | Global r_tip mm | Native walk width mm | Mean thickness mm | Section kc |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `interp_a0_b0` | 0.25 | 0.424203778 | 1.082833538 | 0.424203778 | 4.263701591 | 8.850000000 | 5.673837610 | 1.430571400 |
| `interp_a0_b0` | 0.0625 | 0.416720640 | 1.310617549 | 0.416720640 | 4.050217977 | 8.750000000 | 5.667106156 | 1.433829829 |
| `interp_a0_b0.25` | 0.25 | 0.424260227 | 1.318085196 | 0.424260227 | 4.263565013 | 8.850000000 | 5.673833824 | 1.430571187 |
| `interp_a0_b0.25` | 0.0625 | 0.423851575 | 1.057889303 | 0.423851575 | 4.197013411 | 9.500000000 | 3.693495524 | 1.286537375 |
| `interp_a0_b0.5` | 0.25 | 0.424202327 | 1.061026603 | 0.424202327 | 4.263701525 | 8.850000000 | 5.673837607 | 1.430571400 |
| `interp_a0_b0.5` | 0.0625 | 0.416947457 | 1.039946170 | 0.416947457 | 4.050215773 | 8.750000000 | 5.667106098 | 1.433829826 |
| `interp_a0_b0.75` | 0.25 | 0.424202197 | 1.111526881 | 0.424202197 | 4.263701492 | 8.850000000 | 5.673837606 | 1.430571400 |
| `interp_a0_b0.75` | 0.0625 | 0.415558802 | 1.324601898 | 0.415558802 | 4.029509625 | 8.900000000 | 5.669096573 | 1.428509686 |
| `interp_a0_b1` | 0.25 | 0.424205450 | 1.304033293 | 0.424205450 | 4.263701459 | 8.850000000 | 5.673837605 | 1.430571400 |
| `interp_a0_b1` | 0.0625 | 0.419828729 | 1.085030509 | 0.419828729 | 4.029509568 | 8.900000000 | 5.669096571 | 1.428509685 |
| `interp_a0.25_b0` | 0.25 | 0.453956682 | 1.350119826 | 0.453956682 | 4.020313576 | 9.000000000 | 5.671863081 | 1.425120826 |
| `interp_a0.25_b0` | 0.0625 | 0.429191222 | 1.265470725 | 0.429191222 | 4.056613214 | 8.800000000 | 5.668126307 | 1.432060661 |
| `interp_a0.25_b0.25` | 0.25 | 0.432372249 | 1.121465617 | 0.432372249 | 4.019848830 | 8.650000000 | 5.665456272 | 1.437439406 |
| `interp_a0.25_b0.25` | 0.0625 | 0.375370185 | 1.083067719 | 0.375370185 | 4.009999917 | 9.400000000 | 5.659031202 | 1.410826337 |
| `interp_a0.25_b0.5` | 0.25 | 0.448064458 | 1.334006309 | 0.448064458 | 4.316771693 | 8.950000000 | 5.677645835 | 1.427208487 |
| `interp_a0.25_b0.5` | 0.0625 | 0.425405411 | 1.062561913 | 0.425405411 | 4.011371894 | 9.050000000 | 5.675106928 | 1.423551844 |
| `interp_a0.25_b0.75` | 0.25 | 0.432418328 | 1.484590792 | 0.432418328 | 4.019848879 | 8.650000000 | 5.665456274 | 1.437439407 |
| `interp_a0.25_b0.75` | 0.0625 | 0.429223145 | 1.081709560 | 0.429223145 | 4.056613166 | 8.800000000 | 5.668126306 | 1.432060661 |
| `interp_a0.25_b1` | 0.25 | 0.453960334 | 1.457614909 | 0.453960334 | 4.020313563 | 9.000000000 | 5.671863079 | 1.425120825 |
| `interp_a0.25_b1` | 0.0625 | 0.423405827 | 1.082097095 | 0.423405827 | 4.000999607 | 9.150000000 | 5.677648900 | 1.420236807 |

The planform changes are remeshed by the unchanged native mesher. Even the tiny CP35 perturbation can select a substantially different unstructured mesh and stress maximum, so a single favorable result is not evidence of physical widening or convergence. Every profile retains all measured meshes, actual geometry/thickness hashes, native torsion peak positions and original mean-thickness/section-factor observations. The finite interpolation study cannot turn an index/source-label mismatch into a validated tip redesign.

## Both exact-r=2 native Rust GCI triplets

For every legal profile and each of nine fixed observables, directly included unchanged production `crates/crescent/src/gci.rs::compute_gci` receives actual T6 mesh-content hashes and actual metric values. Canonical finest-to-coarsest areas .25/1/4 have h=.5/1/2; strict areas .0625/.25/1 have h=.25/.5/1. Both have exactly representable r=2, so every sequence/observable check must pass before its independent arithmetic fit verdict. No native validation tolerance or refinement value is altered.

Native totals: `{"admitted": 18, "asymptotic_validation": "not_established", "fit_status_counts": {"admissible_estimate": 18, "nonpositive_order": 34, "oscillatory": 128}, "native_checks": 180, "sequence_valid": 180}`. The analytic p=2/extrapolated 5 control passes independently of the FEM data. Admitted relative error estimates retain `asymptotic_validation=not_established`; oscillatory/nonpositive-order/unresolved results remain refusals. All 180 actual-profile estimates are in `gci_results.json` and `gci_summary.csv`; no favorable observable from another profile is substituted.

- `seq_ratio`: admissible_estimate=5, nonpositive_order=5, oscillatory=10
- `hm_spine_sf`: admissible_estimate=5, nonpositive_order=3, oscillatory=12
- `lat_rob`: admissible_estimate=1, nonpositive_order=7, oscillatory=12
- `fat_sf`: admissible_estimate=1, nonpositive_order=7, oscillatory=12
- `hm_ringback_sf`: oscillatory=20
- `hm_twist_sf`: oscillatory=20
- `hm_twist_body_sf`: admissible_estimate=5, nonpositive_order=5, oscillatory=10
- `hm_twist_tipzone_sf`: oscillatory=20

Primary `interp_a0.25_b0.5` native estimates:

| Metric | Triplet | Fit status | p | Extrapolated | Fine GCI fraction | Admitted |
| --- | --- | --- | --- | --- | --- | --- |
| seq_ratio | canonical | nonpositive_order | -6.666853733 | — | — | False |
| seq_ratio | strict | oscillatory | — | — | — | False |
| hm_spine_sf | canonical | nonpositive_order | -3.040159649 | — | — | False |
| hm_spine_sf | strict | oscillatory | — | — | — | False |
| lat_rob | canonical | nonpositive_order | -1.537002358 | — | — | False |
| lat_rob | strict | oscillatory | — | — | — | False |
| fat_sf | canonical | nonpositive_order | -1.537002358 | — | — | False |
| fat_sf | strict | oscillatory | — | — | — | False |
| hm_ringback_sf | canonical | oscillatory | — | — | — | False |
| hm_ringback_sf | strict | oscillatory | — | — | — | False |
| hm_twist_sf | canonical | oscillatory | — | — | — | False |
| hm_twist_sf | strict | oscillatory | — | — | — | False |
| hm_twist_body_sf | canonical | nonpositive_order | -1.525910040 | — | — | False |
| hm_twist_body_sf | strict | oscillatory | — | — | — | False |
| hm_twist_tipzone_sf | canonical | oscillatory | — | — | — | False |
| hm_twist_tipzone_sf | strict | oscillatory | — | — | — | False |
| sf_hunter | canonical | nonpositive_order | -1.537002358 | — | — | False |
| sf_hunter | strict | oscillatory | — | — | — | False |

## Matched full native solid exports

Every legal profile gets complete native `certification::measure`, identical evaluated genome and compound/pocket law to its FEM receipt. Original actual exported-facet sharpness, apex, point, bore and positive-volume/watertight census contracts are retained. Export sampling h=.05 mm / maxarea=.25 mm² is inherited unchanged.

| Profile | Half-angle deg | Apex mm | Point | Bore mm | Census | Volume mm³ | Pass |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `interp_a0_b0` | 24.324224296 | 0.195144636 | True | 1.600000000 | [0, 0, 1] | 30764.031033614 | True |
| `interp_a0_b0.25` | 24.324741751 | 0.195144777 | True | 1.600000000 | [0, 0, 1] | 30758.879157155 | True |
| `interp_a0_b0.5` | 24.325259194 | 0.195142971 | True | 1.600000000 | [0, 0, 1] | 30766.596859480 | True |
| `interp_a0_b0.75` | 24.325776626 | 0.195142814 | True | 1.600000000 | [0, 0, 1] | 30769.554632225 | True |
| `interp_a0_b1` | 24.326294047 | 0.195142656 | True | 1.600000000 | [0, 0, 1] | 30767.677117419 | True |
| `interp_a0.25_b0` | 26.398394070 | 0.195131701 | True | 1.600000000 | [0, 0, 1] | 30559.223979586 | False |
| `interp_a0.25_b0.25` | 26.395955675 | 0.195139024 | True | 1.600000000 | [0, 0, 1] | 30554.699936260 | False |
| `interp_a0.25_b0.5` | 26.393518087 | 0.195138893 | True | 1.600000000 | [0, 0, 1] | 30554.542463014 | False |
| `interp_a0.25_b0.75` | 26.391081306 | 0.195132065 | True | 1.600000000 | [0, 0, 1] | 30555.010374944 | False |
| `interp_a0.25_b1` | 52.953296389 | 0.195138631 | True | 1.600000000 | [0, 0, 1] | 30550.526423283 | False |

## Verification, artifacts and handoff

`verification.json` status **passed**: 80 unchanged production-file digests; 24 frozen assignment/source/control/input digests; exact parent bevel/pocket/phenotype/exporter sources; native FEM recovered after removing three observation calls; exact eight-CP interpolation and other genes fixed; all evaluated inputs native-canonical; all legal profiles fully evaluated/exported; native global/body/annulus masks and peak indices, metric identities and torque units; protected bands/full-stock neck; actual T6 hashes bound to GCI observables/values; exact-r2 native checks; parent parity. All 47 invocations bind the same executable, CPUs 10,11 / jobs=2 / faer / effective niceness 19. Workflow Python/shell syntax checks pass. Eight dependency hashes and all prior evidence seals are reverified at final sealing.

Primary report: `experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V8_REPORT.md`; byte-identical target mirror: `target/gate-feasibility-bound/UNIFIED_PROTOTYPE_V8_REPORT.md`. All stable artifacts, executable, scripts, raw geometry/FEM/export receipts, source/control provenance, interpolation plan, diagnostics, `fem_summary.csv`, native GCI results, `verification.json`, `summary.json`, `run_provenance.json`, `REPORT_SHA256`, `SHA256SUMS` and standalone `verify_seal.py` are in `target/gate-feasibility-bound/unified_study_v8/`. Report hash and final artifact count appear in the seal sidecars and both agent tracking files to avoid a self-referential report digest.

Requested external audit mirror could not be written: [Errno 30] Read-only file system: '/home/ubuntu/jarambit-audits/2026-10-18-gate-feasibility-bound/UNIFIED_PROTOTYPE_V8_REPORT.md'. The final report is preserved in the primary worktree path and identical target mirror; the exact delivery failure and intended destination are in `mirror_status.json`. The captain can copy the sealed primary report from this worktree.

Reproduce using the existing lane target:

```bash
nice -n 10 taskset -c 10,11 env CARGO_BUILD_JOBS=2 JARAMBIT_SOLVER=faer \
  bash target/gate-feasibility-bound/unified_study_v8/reproduce.sh
```

No production edits, commit, push, PR, branch/worktree change or deletion. Both agent trackers are updated at each milestone. Await captain_jarambit before any further scientific design; the explicit no-commit/no-push instruction is honored.
