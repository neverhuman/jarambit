# Phase-9: windowless deep pocket and shifted-window prototype study

**Result: no tested profile closes the joint structural/export objective.** Every supplied grid tuple fails at least one structural target on a required mesh. The finite grids are fully screened hierarchically, with additional complete three-mesh representatives, five-mesh observations and native exports in both arms. This is finite-grid evidence, not a global infeasibility proof.

Execution baseline `f5fc9e54aad9cc348a04bbcc7d09f574b47adad6`, branch `audit/gate-feasibility-bound`, worktree `/home/ubuntu/jarambit_wt_diag`. Commands use `nice -n 10 taskset -c 10,11`, CARGO_BUILD_JOBS=2, faer, CARGO_INCREMENTAL=0 and the existing lane target/dependencies. Effective niceness is 19 because the host already ran at 19. No production source, native loads/masks, four-sample means, material, geometry predicate, acceptance threshold or GCI contract was edited.

## Source assumptions tested against actual geometry

The assignment attributes the Phase-8 strict peak (128.458205,-15.420706) mm to the proximal window endcap. The retained actual window is x=158.564348..177.880116 mm. A reproduced native solve measures the peak 30.789418841 mm from that window, 0.752610190 mm from the actual inner cutting edge/outline, and 18.922029271 mm from the sharpened spine. Its element mean thickness is 6 mm and pocket weight=0. This is an edge-adjacent peak; the assignment’s window-endcap attribution is contradicted by actual boundary distances. Removing/moving the distal window still executes literally as the requested experimental variable.

The previous pocket uses 1.5 mm cubic smoothstep runouts. Cubic smoothstep is C1 at its joins; its second derivative jumps there. The assignment’s C2 description does not describe that inherited law. This experiment preserves that law rather than adding an unrequested blend change.

Native WIN_S0 bounds are 0.05..0.50, WIN_W bounds are 3..10 mm, and the production window builder minimum nominal width is 3 mm. All requested Arm B S0 values and its 2.4/2.7 mm widths are outside that domain. To execute the explicit numerical tuples, the isolated harness restores only raw S0/W after standard clamping and permits their requested nominal width in the existing window-construction filter. This path is tagged raw_window_prototype=true, input_was_canonical=false and within_genome_domain=false. Production constants/bounds stay unchanged. The actual evaluated CSV and native-clamped CSV are both preserved. Native export measurement is a measurement of the resulting prototype solid; it does not grant genotype/tool eligibility.

## Parent, controls and exact two-arm construction

Q7 is the same literal-index source retained through Phases 6–8. Nineteen frozen Phase-8 source/input/control snapshots are in inputs/. All CPs and hand genes are fixed. Stock=6 mm, spine start=42 mm, compound bevel d0/d1=0.48/0.65 mm, directional POINT_SHAPE=3/POINT_LATERAL=2, full-neck construction and no neck slot are preserved. Arm A changes only WIN_N to zero. Arm B keeps WIN_N=1, S1=0.85, fill=0.65 and varies only the explicitly requested S0/W. No outline is hand drawn or repaired.

| Control | Required areas | Comparable scalars | Maximum relative error |
| --- | --- | --- | --- |
| q7_control | [0.25, 0.125, 0.0625] | 60 | 4.59878044514e-14 |
| phase8_control | [0.25, 0.125, 0.0625] | 60 | 6.18786081701e-14 |

Disabled-pocket Q7 and the exact Phase-8 primary both reproduce all three source meshes. A new no-pocket windowless control also has all three required areas. Native FEM matches production after removing only the two existing postcalculation observation calls. New diagnostics add actual boundary distances and optional SHA-256 of the actual T6 coordinates/connectivity for GCI; they do not change a solve.

Let S(u)=clamp(u,0,1)^2*(3-2*clamp(u,0,1)). ds is the minimum exact segment distance to the full smooth spine and actual sharpened spine; de is exact distance to the actual inner cutting edge. The retained symmetric thickness law is:

```text
w = S((x-x_start)/1.5) * S((x_end-x)/1.5)
    * S((ds-gs)/1.5) * S((de-ge)/1.5)
w = 0 for x <= x_start, x >= x_end, or a disabled pocket
Tnew = T_parent - max(T_parent-t_pocket, 0) * w
```

The law only removes material. It is identically the parent law throughout both protected bands and outside the longitudinal extent. Original full-stock samples in those bands stay exactly 6 mm. Existing thin edge/point material is preserved; it is not filled to the pocket floor. Actual flat-floor samples and native elements are verified separately from whether that floor governs global blade stress. Each CSV needs its matching case config to reproduce the prototype surface law.

## Complete grids and required-mesh coverage

Arm A (windowless_flat_fuse), 432 profiles: x_start={108,114,120}, x_end={152,158}, gs={2.5,3,3.5}, ge={2.5,3,3.5}, t={0.85,0.95,1.05,1.15,1.25,1.35,1.50,1.70} mm.

Arm B (distal_window_plus_pocket), 720 profiles: S0={0.52,0.54,0.56,0.58}, nominal width={2.4,2.7,3}, x_start={118,122,126}, x_end=156, gs={4,4.5}, ge={4.5,5.5}, t={1.6,1.8,2,2.2,2.4} mm. All twelve actual window geometries are measured before screening; no requested tuple is silently clamped into a duplicate parent window.

All 1,152 supplied tuples have area 0.25 mm². Every first-mesh numerical structural pass has area 0.125; every two-mesh pass has strict area 0.0625. Additional top-known-margin, maximum-sequence and any observed flat-floor-governing contrasts receive all three required meshes. A failure at an earlier required mesh already excludes a full three-mesh pass. Domain eligibility remains separate, so Arm B numerical passes cannot become native genotype passes. Selection reasons and exact receipts are retained in qualify_selection.json and qualification.json.

| Arm | Profiles | Coarse structural passes | Second mesh evaluated | Two-mesh passes | Strict evaluated | Three-mesh structural passes | Native-domain three-mesh passes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| A | 432 | 0 | 4 | 0 | 4 | 0 | 0 |
| B | 720 | 227 | 228 | 168 | 172 | 0 | 0 |

Completed 1,589 unbiased native FEM observations in 1,567 receipts, 176 complete required-three-mesh pocket profiles, 4 five-mesh qualifications and 4 complete native exports. Totals include nine control observations, all grid/representative observations and the five requested GCI levels. Identical body/mesh pairs retain the same node/triangle counts as pocket parameters vary. All observations and governing peak diagnostics are in fem_summary.csv.

## Best complete three-mesh representatives

Each row reports minima for one exact profile over areas 0.25/0.125/0.0625. Ranking uses the minimum of sequence/1.25, spine/1, lateral/358 and fatigue/1. Values are never pooled across designs. The full retained inventory is in summary.json.

| Case | Seq >=1.25 | HM spine >=1 | Lateral >=358 N | Fatigue >=1 | Structural pass | Native-domain eligible |
| --- | --- | --- | --- | --- | --- | --- |
| armB_s00.52_w3_x126_156_gs4.5_ge5.5_t2.2 | 1.250921371 | 1.007134993 | 337.997947831 | 0.944138135 | False | False |
| armB_s00.52_w2.7_x126_156_gs4.5_ge5.5_t2.2 | 1.250938012 | 1.007135054 | 337.793678516 | 0.943567544 | False | False |
| armB_s00.58_w2.7_x126_156_gs4.5_ge5.5_t2.2 | 1.251159882 | 1.007134514 | 337.788752059 | 0.943553783 | False | False |
| armB_s00.54_w2.4_x126_156_gs4.5_ge5.5_t2.2 | 1.251024009 | 1.007134741 | 337.777541748 | 0.943522469 | False | False |
| armB_s00.56_w2.4_x126_156_gs4.5_ge5.5_t2.2 | 1.251118102 | 1.007134600 | 337.771689464 | 0.943506122 | False | False |
| armB_s00.56_w3_x126_156_gs4.5_ge5.5_t2.2 | 1.251127056 | 1.007134594 | 337.768087495 | 0.943496060 | False | False |
| armB_s00.56_w2.7_x126_156_gs4.5_ge5.5_t2.2 | 1.251127469 | 1.007134591 | 337.767988157 | 0.943495783 | False | False |
| armB_s00.54_w2.7_x126_156_gs4.5_ge5.5_t2.2 | 1.251021351 | 1.007134735 | 337.714257716 | 0.943345696 | False | False |
| armA_x120_158_gs3.5_ge3.5_t1.7 | 1.410635821 | 0.891894304 | 236.111278767 | 0.659535550 | False | True |
| armA_x120_152_gs3.5_ge3.5_t1.7 | 1.394152134 | 0.891866108 | 235.518233362 | 0.657878981 | False | True |

Primary measured profile: `armB_s00.52_w3_x126_156_gs4.5_ge5.5_t2.2`. Its prototype thickness and actual window settings are recorded in its case JSON and evaluated CSV. A noncanonical prototype keeps that label even if its numerical metrics or native exported solid pass. Human-max twist and other legacy metrics remain in the raw receipts; no all-hard closure is implied.

| Area mm² | Sequence | HM spine | Lateral N | Fatigue | Blade peak x,y | Mean t | Pocket weight | Flat-floor peak |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0.25 | 1.250921371 | 1.038357494 | 380.867237384 | 1.063885995 | [129.01902128723077, -15.545498765624169] | 5.129238705 | 0.000000000 | 1.119093076 |
| 0.125 | 1.259814328 | 1.063142121 | 380.513775856 | 1.062898662 | [129.90111255980253, -15.828935275107796] | 5.936204993 | 0.000000000 | 0.998703791 |
| 0.0625 | 1.421907692 | 1.007134993 | 337.997947831 | 0.944138135 | [128.4582054785095, -15.420706329362323] | 6.000000000 | 0.000000000 | 0.991458513 |

## Does the flat floor govern?

A global lateral blade peak counts as lying on the flat floor only when its centroid pocket weight is at least 1-1e-12 and its native four-sample element mean equals t_pocket within 1e-10 mm. The flat-floor peak is also measured independently over all qualifying elements. Thickness protection and a flat floor do not force a governing location or preserve the original stress margin after stiffness redistribution.

| Arm / mesh | Observations | Has flat-floor elements | Global blade peak on flat floor | Global blade peak weight=0 |
| --- | --- | --- | --- | --- |
| A / 0.25 | 432 | 432 | 0 | 432 |
| A / 0.0625 | 4 | 4 | 0 | 4 |
| A / 0.125 | 4 | 4 | 0 | 4 |
| A / 0.6 | 2 | 2 | 0 | 2 |
| A / 0.36 | 2 | 2 | 0 | 2 |
| A / 0.22 | 2 | 2 | 0 | 2 |
| A / 0.14 | 2 | 2 | 0 | 2 |
| A / 0.08 | 2 | 2 | 0 | 2 |
| B / 0.25 | 720 | 720 | 0 | 720 |
| B / 0.0625 | 172 | 172 | 0 | 172 |
| B / 0.125 | 228 | 228 | 0 | 228 |
| B / 0.6 | 2 | 2 | 0 | 2 |
| B / 0.36 | 2 | 2 | 0 | 2 |
| B / 0.22 | 2 | 2 | 0 | 2 |
| B / 0.14 | 2 | 2 | 0 | 2 |
| B / 0.08 | 2 | 2 | 0 | 2 |

All reported lateral capacities are the native global value. lat_rob=730.175/sigma_global, where sigma_global may be in the hand or blade. Native sequence is the minimum across lateral, pry and chord-torsion. The blade/hand lateral ratio alone cannot replace that minimum, and a blade-only stress corridor cannot replace a global lateral-capacity check. Exact cutting-edge/window/outline distances are retained at every lateral peak.

## Five requested mesh levels and GCI

The top two fully evaluated representatives in each arm receive every requested max area: 0.60, 0.36, 0.22, 0.14 and 0.08 mm². Each has actual SHA-256 of T6 nodal coordinates, six-node connectivity and corner count, generated by the existing ContentRef hash utility. Those mesh-content identities, each raw receipt digest, the matching physical config and the unchanged observable definitions bind the observations; receipt hashes are not substituted for mesh hashes.

Native ordinary GCI accepts a uniform h ratio on each three-level sequence. With the native nominal convention h=sqrt(max_area), the five requested areas have unequal successive ratios. All three overlapping triples are therefore passed to the unchanged repository fem_solid.gci.compute_gci, and all return unsupported_sequence / nonuniform_refinement. The requested meshes were executed; ordinary native GCI admission remains unverified. No refinement ratio is rounded or replaced to manufacture admission.

For numerical diagnosis, gci_results.json also fits each overlapping nonuniform triple to f(h)=f_ext+C*h^p. Positive-order fits solve (f_coarse-f_middle)/(f_middle-f_fine)=(h_coarse^p-h_middle^p)/(h_middle^p-h_fine^p) with p bounded to (1e-7,40), then use Fs=1.25 for relative fine-grid error. Oscillatory, plateau, precision-limited and nonpositive-order sequences produce no such estimate. These estimates are explicitly diagnostic, with no native admission, asymptotic validation or physical-validation claim. Four target metrics plus blade/hand stresses are retained over all five levels.


Five-mesh profile `armA_x120_158_gs3.5_ge3.5_t1.7` (native-domain eligible=True):

| Area mm² | Sequence | HM spine | Lateral N | Fatigue | Nodes | Triangles |
| --- | --- | --- | --- | --- | --- | --- |
| 0.6 | 1.480442865 | 0.881647776 | 280.255993297 | 0.782846087 | 7864 | 14450 |
| 0.36 | 1.735684685 | 0.847212718 | 247.982459106 | 0.692695615 | 11932 | 22503 |
| 0.22 | 1.517011082 | 1.039348885 | 266.615145701 | 0.744742765 | 19403 | 36870 |
| 0.14 | 1.500948397 | 1.047293445 | 240.085018520 | 0.670635497 | 28983 | 55894 |
| 0.08 | 1.463039380 | 0.957774176 | 270.771250742 | 0.756352118 | 49827 | 96919 |

| Lateral triple, finest first | Native status | Generalized diagnostic | Positive p | Relative GCI fine |
| --- | --- | --- | --- | --- |
| [0.22, 0.36, 0.6] | unsupported_sequence/nonuniform_refinement | oscillatory | unavailable | unavailable |
| [0.14, 0.22, 0.36] | unsupported_sequence/nonuniform_refinement | oscillatory | unavailable | unavailable |
| [0.08, 0.14, 0.22] | unsupported_sequence/nonuniform_refinement | oscillatory | unavailable | unavailable |

Five-mesh profile `armA_x120_152_gs3.5_ge3.5_t1.7` (native-domain eligible=True):

| Area mm² | Sequence | HM spine | Lateral N | Fatigue | Nodes | Triangles |
| --- | --- | --- | --- | --- | --- | --- |
| 0.6 | 1.478546618 | 0.881647856 | 280.615453532 | 0.783850176 | 7864 | 14450 |
| 0.36 | 1.741006179 | 0.937557852 | 247.224531882 | 0.690578478 | 11932 | 22503 |
| 0.22 | 1.513422298 | 1.039298117 | 266.477203581 | 0.744357448 | 19403 | 36870 |
| 0.14 | 1.497252733 | 1.047252176 | 239.812557671 | 0.669874425 | 28983 | 55894 |
| 0.08 | 1.459845832 | 0.958820198 | 271.151820575 | 0.757415173 | 49827 | 96919 |

| Lateral triple, finest first | Native status | Generalized diagnostic | Positive p | Relative GCI fine |
| --- | --- | --- | --- | --- |
| [0.22, 0.36, 0.6] | unsupported_sequence/nonuniform_refinement | oscillatory | unavailable | unavailable |
| [0.14, 0.22, 0.36] | unsupported_sequence/nonuniform_refinement | oscillatory | unavailable | unavailable |
| [0.08, 0.14, 0.22] | unsupported_sequence/nonuniform_refinement | oscillatory | unavailable | unavailable |

Five-mesh profile `armB_s00.52_w3_x126_156_gs4.5_ge5.5_t2.2` (native-domain eligible=False):

| Area mm² | Sequence | HM spine | Lateral N | Fatigue | Nodes | Triangles |
| --- | --- | --- | --- | --- | --- | --- |
| 0.6 | 1.045326538 | 0.881547133 | 396.328311751 | 1.107073802 | 7938 | 14500 |
| 0.36 | 1.119288959 | 1.130691310 | 384.547621584 | 1.074166505 | 11974 | 22469 |
| 0.22 | 1.161028051 | 1.042679480 | 374.254030317 | 1.045413158 | 19461 | 36822 |
| 0.14 | 1.183085688 | 1.086957157 | 398.442908826 | 1.112980559 | 28861 | 55476 |
| 0.08 | 1.212834546 | 1.030971828 | 381.033007403 | 1.064349045 | 49471 | 95986 |

| Lateral triple, finest first | Native status | Generalized diagnostic | Positive p | Relative GCI fine |
| --- | --- | --- | --- | --- |
| [0.22, 0.36, 0.6] | unsupported_sequence/nonuniform_refinement | diagnostic_positive_order_fit | 0.392022420 | 0.339245435 |
| [0.14, 0.22, 0.36] | unsupported_sequence/nonuniform_refinement | oscillatory | unavailable | unavailable |
| [0.08, 0.14, 0.22] | unsupported_sequence/nonuniform_refinement | oscillatory | unavailable | unavailable |

Five-mesh profile `armB_s00.52_w2.7_x126_156_gs4.5_ge5.5_t2.2` (native-domain eligible=False):

| Area mm² | Sequence | HM spine | Lateral N | Fatigue | Nodes | Triangles |
| --- | --- | --- | --- | --- | --- | --- |
| 0.6 | 1.045327514 | 0.881547131 | 396.328001712 | 1.107072936 | 7956 | 14529 |
| 0.36 | 1.119519643 | 1.130690936 | 384.468300981 | 1.073944936 | 11996 | 22510 |
| 0.22 | 1.162930262 | 1.042679471 | 373.978920013 | 1.044644686 | 19485 | 36867 |
| 0.14 | 1.182971953 | 1.086957132 | 398.445998051 | 1.112989188 | 28898 | 55545 |
| 0.08 | 1.212655053 | 1.030961886 | 381.116738035 | 1.064582932 | 49527 | 96092 |

| Lateral triple, finest first | Native status | Generalized diagnostic | Positive p | Relative GCI fine |
| --- | --- | --- | --- | --- |
| [0.22, 0.36, 0.6] | unsupported_sequence/nonuniform_refinement | diagnostic_positive_order_fit | 0.343579633 | 0.397126743 |
| [0.14, 0.22, 0.36] | unsupported_sequence/nonuniform_refinement | oscillatory | unavailable | unavailable |
| [0.08, 0.14, 0.22] | unsupported_sequence/nonuniform_refinement | oscillatory | unavailable | unavailable |

## Complete native exported-solid measurements

Each measurement directly invokes production certification::measure on the exact evaluated genes and section config used for that profile’s FEM. The actual native solid supplies positive volume and census. Prototype Arm B remains outside native genotype/tool bounds irrespective of its solid measurements.

| Case | Half-angle <=25 deg | Apex <=0.20 mm | Point | Bore >=1.5 mm | Census | Export pass | Three-mesh structural + export | Native-domain eligible |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| armA_x120_158_gs3.5_ge3.5_t1.7 | 24.324224295870 | 0.195144636429 | True | 1.600000000 | [0, 0, 1] | True | False | True |
| armA_x120_152_gs3.5_ge3.5_t1.7 | 24.324224295870 | 0.195144636429 | True | 1.600000000 | [0, 0, 1] | True | False | True |
| armB_s00.52_w3_x126_156_gs4.5_ge5.5_t2.2 | 24.324224295870 | 0.195144636429 | True | 1.600000000 | [0, 0, 1] | True | False | False |
| armB_s00.52_w2.7_x126_156_gs4.5_ge5.5_t2.2 | 24.324224295870 | 0.195144636429 | True | 1.600000000 | [0, 0, 1] | True | False | False |

Native geometry/window probes, actual evaluated/native-clamped CSVs, full point-profile samples, worst exported triangles/edge distances, actual volumes, every FEM peak and raw boundary distances are retained. Passing values from different designs, meshes or prototype-eligibility categories are never pooled.

## Verification, reproduction and handoff

Verified 80 production source/requirement/build/GCI-file digests and 19 frozen source/input snapshots. Compound law, pocket, exporter and FEM are byte-identical to Phase-8; removing the explicitly tagged raw-window width-filter branch recovers its phenotype source. Removing two existing postcalculation hooks recovers production FEM. Checked source parity, actual boundary attribution, raw-domain flags, evaluated/canonical CSVs, fixed non-window genes, parameter enumerations, native geometry, protected thickness, actual floor samples/elements, repeated-body node/triangle counts, native metric identities, full hierarchical required-mesh coverage, actual mesh-content hashes and exact FEM/export law/genes. Shell/Python workflow syntax checks pass. Eight dependency hashes and the original Phase-4/5/6/7/8 seals are checked at seal time.

Run `nice -n 10 taskset -c 10,11 env CARGO_BUILD_JOBS=2 bash target/gate-feasibility-bound/unified_study_v6/reproduce.sh`. This rebuilds the isolated harness from frozen inputs, reproduces source/windowless controls, constructs all twelve explicit prototype windows, executes the complete 1,152-profile coarse grids and hierarchical finer checks, completes representative three-mesh profiles, runs all five requested GCI levels for the top two per arm, measures all four native exports, preserves native GCI refusal, analyzes, reports and reseals. It reuses this lane target and retained dependencies.

Primary report: experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V6_REPORT.md. An identical convenience copy is in target/gate-feasibility-bound/UNIFIED_PROTOTYPE_V6_REPORT.md. unified_study_v6/SHA256SUMS seals both copies and all stable study artifacts; REPORT_SHA256 identifies the report.

Phase-9 evaluation is complete. Await captain_jarambit for another scientific milestone. The actual source peak is edge-adjacent, not at the distal window endcap. Arm B’s requested domain/tool violations and the five-area native GCI incompatibility remain explicit limitations. No production edits, push, PR, stacking, branch/worktree change or deletion by this runner. Shared Git/COMMS metadata remain read-only in this session; prepared local checkpoint/handoff files do not imply a commit or external post. No global infeasibility proof, asymptotic verification or all-hard certificate is claimed.
