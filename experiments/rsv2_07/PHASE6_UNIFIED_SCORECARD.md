# RSV2-07 Phase 6: unified plate, CAD and solid scorecard

All three requested candidates were evaluated on the full five-mesh plate study and the documented 16 engineering / 10 RS-v2 gate matrix. **No candidate passes all observed gates.** Gate observations apply to the explicitly bound compound prototypes; no formal native certificate or manufacturing promotion is issued.

| Candidate | Engineering | RS-v2 | Cert sequence ≥1.20 | Cert HM twist ≥1 | Primary / DOLFINx 3D max SF | W_CAD_v2 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Q7 | 15/16 | 6/10 | 0.972662 | 0.424207 | 2.824 / 2.8232 | -1.757794 |
| placement_05 | 16/16 | 7/10 | 1.231106 | 0.245689 | 2.346 / 2.3465 | 1.200089 |
| Q7_window_p05 | 15/16 | 6/10 | 1.060783 | 0.403634 | 2.111 / 2.1108 | -1.464056 |

W_CAD_v2 is computed by the unchanged evaluator helper on the 16 engineering flags. The 10 RS-v2 hard gates and CERT-02 sign-off remain separate qualification requirements.

## Candidate construction and evidence identity

Q7 and placement_05 use their exact Phase-5 canonical genomes and their distinct saved section/phenotype laws. Their Phase-5 primary and independent observations are reused only after checking all retained hashes and reproducing their production surface bytes exactly. The new **Q7_window_p05** retains Q7's section law (d0/d1=0.48/0.65 mm, point shape3, point lateral2) and copies placement_05's actual window values: win_s0=0.05, win_s1=0.525, win_w=6.0, win_fill=0.70, ring_wall=6.4 mm. These lie within every assigned range; ring_wall was already6.4 in Q7. Other genes are unchanged. This is one specified hybrid, without a claim of optimizing the supplied ranges. Its canonical CSV is [phase6_Q7_window_p05.csv](results/phase6_Q7_window_p05.csv).

Production export uses `export::solid(&p,1.0)` with native RS-v2 internal cap0.25 mm², DP0.03 mm and no study grid insertion. The generated export body is byte-identical to production from `pub fn solid` onward. The witness compound thickness and existing phenotype variants remain explicitly included in the source identity. STEP/STL/surface regenerations match byte for byte for each candidate. Native exported sharpness, full point profiles and bore radii are measured anew on these production facets; dense-study sharpness observations are not transferred.

## All 16 engineering gates

IDs and exact predicates follow [WORKPLAN.md](../../docs/WORKPLAN.md) §3B and the current evaluator; full predicate text is preserved in the JSON receipt. Certification uses native `fem::certify` (the conservative 0.25/0.125 mm² pair). Strict checks use 0.0625 mm². Coarse meshes are diagnostic and do not replace certification.

| ID | Gate | Q7 | placement_05 | Q7_window_p05 |
| --- | --- | --- | --- | --- |
| G16-01 | cad_watertight_and_single_solid | pass | pass | pass |
| G16-02 | geometric_and_holder_gates | pass | pass | pass |
| G16-03 | karambit_ring_and_curved_claw | pass | pass | pass |
| G16-04 | c6_aesthetic_floor | pass | pass | pass |
| G16-05 | cert_hiker_180n | pass | pass | pass |
| G16-06 | cert_hunter_260n | pass | pass | pass |
| G16-07 | strict_hunter_260n | pass | pass | pass |
| G16-08 | cert_ring_pull_1500n | pass | pass | pass |
| G16-09 | twist_body_1p82nm | pass | pass | pass |
| G16-10 | twist_tip_only_1p82nm | pass | pass | pass |
| G16-11 | twist_tip_submodel_strict_1p82nm | pass | pass | pass |
| G16-12 | solid_tip_3d_p2_1p82nm | pass | pass | pass |
| G16-13 | side_a4_buckling_and_support | pass | pass | pass |
| G16-14 | failure_sequence_1p2 | fail | pass | fail |
| G16-15 | plate_gci_valid_3level | pass | pass | pass |
| G16-16 | dolfinx_cross_check_2pct | pass | pass | pass |

G16-01 includes the existing unit/single-solid/watertight/mass check, deterministic regeneration and independent OpenCascade `BRepCheck_Analyzer`/solid count. G16-15 preserves the evaluator's exact `sequence_valid && observable_valid` rule. The separate battery `plate.gci` admission result is reported below; identity validity alone does not establish convergence.

CAD verdicts use the repository's normal STEP import, including standard shape processing, followed by the full analyzer and solid count. An optional Q7 import with an empty shape-processing sequence returned `valid=false`, `solids=1`; its input hashes and tolerance settings were unchanged. The matrix uses the completed standard-import results. The optional result is retained as an additional diagnostic. The [OCCT processing implementation](https://github.com/Open-Cascade-SAS/OCCT/blob/V7_8_1/src/XSAlgo/XSAlgo_AlgoContainer.cxx) documents this import-processing branch.

The written STL also receives an independent edge census and signed-volume calculation. The native STL writer rounds coordinates to 4 decimal places, so its volume difference from the unrounded solid is recorded; the unchanged 1% mass limit is checked against the raster for both the native CAD volume and the serialized STL. Full-precision surface JSON supplies the 3D solve.

| Candidate | Raster / CAD mass g | Relative mass difference ≤1% | Deterministic | OCC valid / solids | STEP/mesh bore match |
| --- | ---: | ---: | --- | --- | --- |
| Q7 | 139.550171 / 139.495800 | 0.039000% | True | True / 1 | True |
| placement_05 | 134.572599 / 134.556900 | 0.011700% | True | True / 1 | True |
| Q7_window_p05 | 136.179494 / 136.168100 | 0.008400% | True | True / 1 | True |

## All 10 RS-karambit-v2 gates

Predicates follow native `bench.rs::rs_v2_eval` and WORKPLAN §3C. The solid-tip row is a diagnostic observation supported by candidate/law/surface/mesh hashes and independent raw solves. Production certificate binding and owner promotion are not asserted for this isolated compound harness. The 1000 N tip in-plane result remains record-only, as native code requires.

| ID | Gate | Q7 | placement_05 | Q7_window_p05 |
| --- | --- | --- | --- | --- |
| RSV2-01 | aesthetic_and_geometric | pass | pass | pass |
| RSV2-02 | floors_828_joint | pass | pass | pass |
| RSV2-03 | twist_floor_body_and_tipsub_strict | pass | pass | pass |
| RSV2-04 | solid_tip_sf | pass | pass | pass |
| RSV2-05 | strict_hunter_lateral | pass | pass | pass |
| RSV2-06 | seq_hard | fail | pass | fail |
| RSV2-07 | human_max | fail | fail | fail |
| RSV2-08 | fatigue | pass | pass | pass |
| RSV2-09 | gate_s_sharp_law | fail | fail | fail |
| RSV2-10 | gate_s_tip_point_brep | fail | fail | fail |

Production-facet sharpness and point observations (limits: apex≤0.20 mm, half-angle≤25°, t0.25/t1/t2≤0.30/0.60/1.00 mm, monotonic=true and taper≥5 mm):

| Candidate | Maximum apex mm | Maximum half-angle deg | Point t0.25 / t1 / t2 mm | Monotonic / taper lower bound mm |
| --- | ---: | ---: | ---: | --- |
| Q7 | 0.194227 | 85.932530 | 0.176476 / 0.441639 / 1.440056 | True / 5.195944 |
| placement_05 | 0.194492 | 78.692535 | 0.176381 / 0.441369 / 1.108372 | True / 10.000000 |
| Q7_window_p05 | 0.194227 | 84.985894 | 0.176476 / 0.443781 / 1.429187 | True / 5.156299 |

## Five-mesh plate, native certification and A4

All capacities use the unchanged native loads, material, four-sample element thickness, exclusions and thin/hot knockdown. Native faer uses two Rayon threads on CPUs 14,15. The placement_05 Phase-5 context used its configured skyline backend; Phase6 uses the supported faer backend for all plate studies, without changing geometry or the reused solid surface.

Q7's three existing fine-mesh witness records replay within 1e-10 relative tolerance. The earlier placement_05 witness used skyline: its checked metrics differ from the new faer values by at most 2.459e-6 relative (0.0002459%); they do not meet a 1e-10 equality check. Both source records and every difference are retained. The scorecard uses the newly solved faer results and the unchanged physical gate thresholds.

### Q7

| area mm² | nodes / elements | lateral N | seq | HM twist | HM spine | HM ring-back | fatigue | body twist / tip zone |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 4 | 2034 / 3371 | 480.804945 | 0.994365 | 0.437189 | 0.904029 | 5.127088 | 1.343044 | 10.977806 / 3.891467 |
| 1 | 4567 / 8309 | 484.343340 | 0.924590 | 0.416786 | 0.728460 | 5.055822 | 1.352928 | 13.446901 / 3.709856 |
| 0.25 | 16421 / 31257 | 474.635232 | 1.003792 | 0.424207 | 1.038249 | 5.131770 | 1.325810 | 9.837244 / 3.775911 |
| 0.125 | 31965 / 61641 | 479.695734 | 0.972662 | 0.435487 | 1.063025 | 4.992112 | 1.339946 | 9.293154 / 3.876317 |
| 0.0625 | 62294 / 121486 | 396.501964 | 1.211322 | 0.416724 | 1.007022 | 5.064628 | 1.107559 | 9.457251 / 3.709305 |

Native certify: lateral=474.635231835 N, ring=23368.980726536 N, sequence=0.972661735, body/tip SF=9.293153574/3.775911274, HM twist/spine/ring-back=0.424207316/1.038248515/4.992111565, fatigue=1.325810483.

A4 status=pass, buckling eigenvalue=10.366098143, critical load=2695.185517281 N, supported sequence=2.942390034. This support model does not replace the production ring-clamp sequence.

### placement_05

| area mm² | nodes / elements | lateral N | seq | HM twist | HM spine | HM ring-back | fatigue | body twist / tip zone |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 4 | 2088 / 3443 | 356.018457 | 1.393803 | 0.250815 | 0.469205 | 6.737271 | 0.994475 | 10.454276 / 2.232530 |
| 1 | 4545 / 8212 | 407.275288 | 1.254653 | 0.253513 | 0.438300 | 6.610823 | 1.137652 | 13.892741 / 2.256547 |
| 0.25 | 16115 / 30571 | 400.086539 | 1.231106 | 0.245689 | 0.482583 | 6.660085 | 1.117572 | 11.233061 / 2.186901 |
| 0.125 | 31404 / 60400 | 399.115772 | 1.310469 | 0.261355 | 0.461002 | 6.500083 | 1.114860 | 12.429100 / 2.326347 |
| 0.0625 | 61197 / 119109 | 386.840695 | 1.325634 | 0.301237 | 0.466848 | 6.343549 | 1.080572 | 11.336213 / 2.681338 |

Native certify: lateral=399.115771513 N, ring=32335.517514732 N, sequence=1.231105512, body/tip SF=11.233061393/2.186900932, HM twist/spine/ring-back=0.245688870/0.461001599/6.500083239, fatigue=1.114860083.

A4 status=pass, buckling eigenvalue=8.424593980, critical load=2190.394434789 N, supported sequence=2.965487206. This support model does not replace the production ring-clamp sequence.

### Q7_window_p05

| area mm² | nodes / elements | lateral N | seq | HM twist | HM spine | HM ring-back | fatigue | body twist / tip zone |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 4 | 2012 / 3302 | 350.480155 | 1.371846 | 0.354928 | 0.547811 | 5.127088 | 0.979005 | 9.601690 / 3.159245 |
| 1 | 4484 / 8114 | 386.672151 | 1.252594 | 0.392248 | 0.635175 | 5.055822 | 1.080101 | 13.012598 / 3.491435 |
| 0.25 | 16064 / 30488 | 449.135064 | 1.060783 | 0.428769 | 0.656553 | 5.131770 | 1.254580 | 15.870103 / 3.816517 |
| 0.125 | 31344 / 60304 | 425.704640 | 1.126040 | 0.403634 | 0.799369 | 4.992112 | 1.189131 | 11.647899 / 3.592787 |
| 0.0625 | 60839 / 118440 | 429.078166 | 1.119322 | 0.438662 | 0.813076 | 5.064628 | 1.198555 | 13.914890 / 3.904575 |

Native certify: lateral=425.704639618 N, ring=23368.980899747 N, sequence=1.060783104, body/tip SF=11.647898844/3.592786661, HM twist/spine/ring-back=0.403634057/0.656552629/4.992111573, fatigue=1.189131434.

A4 status=pass, buckling eigenvalue=10.735712524, critical load=2791.285256197 N, supported sequence=2.749568614. This support model does not replace the production ring-clamp sequence.


## Native GCI and independent 3D checks

The six observables below call actual `jarambit_crescent::gci::compute_gci` with ordered areas0.25/1/4 mm², r=2 and Fs=1.25, bound to each solved mesh hash. Unadmitted fits retain null extrapolation/error/order fields where native code supplies null; no oscillatory or nonpositive fit is converted into an error bound.

Native `asymptotic_validation` remains `not_established`, including admitted fits. In particular, placement_05 has canonical sequence GCI0.4870% at0.25 mm², while its0.125/0.0625 mm² sequence values are1.310469/1.325634 versus1.231106 at0.25 mm². The canonical admission result is retained without extending its error bound to the whole five-mesh sequence.

| Candidate | Observable | Native fit status | admitted | relative GCI |
| --- | --- | --- | --- | ---: |
| Q7 | fat_sf | oscillatory | false | null |
| Q7 | hm_ringback_sf | oscillatory | false | null |
| Q7 | hm_spine_sf | oscillatory | false | null |
| Q7 | hm_twist_sf | oscillatory | false | null |
| Q7 | seq_ratio | oscillatory | false | null |
| Q7 | sf_hunter | oscillatory | false | null |
| placement_05 | fat_sf | oscillatory | false | null |
| placement_05 | hm_ringback_sf | oscillatory | false | null |
| placement_05 | hm_spine_sf | oscillatory | false | null |
| placement_05 | hm_twist_sf | oscillatory | false | null |
| placement_05 | seq_ratio | admissible_estimate | true | 0.004870121403132769 |
| placement_05 | sf_hunter | oscillatory | false | null |
| Q7_window_p05 | fat_sf | nonpositive_order | false | null |
| Q7_window_p05 | hm_ringback_sf | oscillatory | false | null |
| Q7_window_p05 | hm_spine_sf | admissible_estimate | true | 0.01318594392039272 |
| Q7_window_p05 | hm_twist_sf | admissible_estimate | true | 4.870129297086229 |
| Q7_window_p05 | seq_ratio | nonpositive_order | false | null |
| Q7_window_p05 | sf_hunter | nonpositive_order | false | null |

The solid protocol is unchanged `tip_solid.py 30 4 1`, PATCH2 mm and torque1820 N·mm. Primary convergence requires the independently recomputed relative residual ≤1e-10. DOLFINx v0.9.0 uses the identical saved mesh, cuts, forces, material and allowance; both nodal-max and nodal-average SFs must differ by ≤2%. The receipt includes both native reported SFs, primary residuals, both relative errors, solver settings, runtime versions and sidecars. Q7_window_p05 receives a new primary solve and new independent solve; no witness solid result is substituted for it.

| Candidate | Primary max / avg SF | DOLFINx max / avg SF | Error vs primary max / avg | Primary relative residual | P2 DOFs |
| --- | ---: | ---: | ---: | ---: | ---: |
| Q7 | 2.824 / 3.787 | 2.8232 / 3.7876 | 0.02832861% / 0.01584368% | 9.10159e-11 | 402813 |
| placement_05 | 2.346 / 3.127 | 2.3465 / 3.1273 | 0.02131287% / 0.00959386% | 9.59727e-11 | 406809 |
| Q7_window_p05 | 2.111 / 2.502 | 2.1108 / 2.5026 | 0.00947418% / 0.02398082% | 9.54709e-11 | 401139 |

## CERT-02 bore dimensions and sign-off

The assignment requires measured bearing≥12.0 mm and edge radius≥1.5 mm. Native RS-v2 currently uses bearing≥10.0 mm; both the native human-max rule and the additional stricter12 mm check are recorded. Bearing is measured from independently fitted exported meridians and checked against native bearing evaluation. Native radius fits, independent mesh circle fits and analytic STEP roll measurements are recorded separately.

| Candidate | Measured bearing mm | Native measured edge radius mm | ≥12 / ≥1.5 numeric check | Owner construction/aesthetic sign-off |
| --- | ---: | ---: | --- | --- |
| Q7 | 12.800000000 | 1.600000000 | pass | unverified |
| placement_05 | 12.800000000 | 1.600000000 | pass | unverified |
| Q7_window_p05 | 12.800000000 | 1.600000000 | pass | unverified |

Numeric compliance does not supply CERT-02 owner construction approval or final aesthetic sign-off. Those remain unverified; no approval is inferred from this assignment.

## Verification and reproduction

Raw evidence: `target/rsv2-07/phase6/20261018T072928Z/`. All builds/solves are pinned to CPUs 14,15 at nice 10 with CARGO_BUILD_JOBS=2; OMP/OpenBLAS/MKL threads=1. The hybrid primary has a 64 GiB address-space bound, and the independent container has explicit `--cpuset-cpus=14,15` and nice 10. This lane's target directory is reused. Production solvers, gate thresholds, requirements and prior reports are unchanged.

After the hybrid primary/independent pair completed, a recorded scheduling resumption replaced the initial Python launcher to overlap independent CAD work within the same CPU pair. The unfinished Q7 CAD process ended without a result following that replacement; its exit status was not captured. That incomplete attempt is excluded; the exact unchanged Q7 CAD check was restarted in its own execution session. No completed plate or 3D result was lost, and no primary/independent numerical solver was interrupted. Original attempt logs/stacks, the scheduling correction, retry completion and final CAD results are retained. No exit code is invented for the interrupted CAD attempt. Scientific inputs, implementations and thresholds are unchanged.

The receipt [phase6_unified_scorecard.json](results/phase6_unified_scorecard.json) contains complete five-mesh values, exact predicate text, gate verdicts, native GCI/A4 results, export measurements, CAD checks, primary/independent records, source/input/binary hashes and all raw artifact hashes. Primary/independent input identities and errors, native certification minima, A4 algebraic identities, native mesh hashes and unchanged sources are checked. The separate retained mesh checker verifies positive tets, pure torque and disjoint load/clamp sets. This is a diagnostic scorecard, without a solid mesh-convergence study or a claim of full formal certification.

From a normal-priority shell at the repository root:

```sh
nice -n 10 taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/phase6_run.py
python3 experiments/rsv2_07/phase6_scorecard.py target/rsv2-07/phase6/<timestamp>
```

The first command retains raw results; the second binds the matrix/report. New raw runs refuse timestamp-directory reuse. The saved Phase-5 source and raw artifacts must remain available for exact evidence reuse.
