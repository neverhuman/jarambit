# G16-12-RSV2-04 Phase-10: literal distal-root grid and native uniform-ratio GCI

Execution commit: `32a29e8b1f9519e072b8e03e0fc8b5c45d7a3e2b`; branch `audit/gate-feasibility-bound`. Task `G16-12-RSV2-04-p10`; CPU affinity **10,11**, `CARGO_BUILD_JOBS=2`, faer. Requested `nice -n 10`; inherited effective niceness 19. Production requirements, loads/masks, operators, stock/material, geometry predicates and acceptance thresholds are unchanged. No commit, push or new PR was attempted, as explicitly instructed.

## Outcome

Executed all **720 literal assigned tuples** (144 windowless Arm A, 576 retained-window Arm B), with **766 unbiased native FEM observations in 742 receipts**, 8 complete screening-three-mesh profiles, and 4 top representatives with all five assigned ladder levels plus area 4.0 and matched complete native solid exports.

Numerical three-screen-mesh structural passes: **0**; matched structural/export passes: **0**; native-domain structural/export passes: **0**. This is a finite-grid result, not a continuous/global infeasibility proof or an all-hard production certificate.

Primary `armB_s0.52_w3_x144_t2.2_s4.5_e5.5_d0.1` has minimum sequence **1.216734841**, HM spine **0.725265009**, lateral **337.997947831 N**, fatigue **0.944138135** across areas 1.0 / 0.25 / 0.0625. Native-domain eligibility: **False**. Required limits: sequence>=1.20, spine>=1.00, lateral>=358 N and fatigue>=1.00.

The mathematically uniform √2 ladder still encounters native `nonuniform_refinement`: the unchanged Rust validator uses exact binary floating-point equality. The canonical 4/1/.25 and stride-2 1/.25/.0625 triplets have exactly representable r=2 and pass native sequence validation. Their remaining fit verdicts are reported as measured; sequence admission alone does not establish convergence or structural margins.

## Literal construction, bounds and inactive-gene interpretation

The assignment supplies native gene names and guard pairs but no new physical taper/relief law, pocket floor or pocket endpoints. This study keeps the exact best complete Phase-9 parent per arm: A floor 1.7 mm at x=120..158; B floor 2.2 mm at x=126..156. Only the explicitly named genes and guard pairs vary. Compound bevel/point cap, full-stock neck/core, 1.5 mm cubic-smoothstep pocket and native exported-solid measurement remain byte-identical to Phase-9. The inherited blend is C1 at joins. No hand-built alternative taper, fuller or P1 architecture was introduced.

| Arm | Explicit Cartesian sweep | Tuples |
| --- | --- | --- |
| A | WIN_N=0; TAPER_X0={126,132,138,144}; T_TIP={2.0,2.3,2.6,2.9}; FUL_D={0,.3,.6}; (gs,ge)={(3.5,3.5),(4,5),(4.5,5.5)} | 144 |
| B | WIN_N=1; S0={.05,.50,.52}; W={3,4.5}; TAPER_X0={126,132,138,144}; T_TIP={2.2,2.5,2.8,3}; (gs,ge)={(4,5),(4.5,5.5)}; POCKET_D={0,.1,.2} | 576 |

Native `T_TIP` minimum is 2.6 mm; therefore A t=2.0/2.3 and B t=2.2/2.5 are outside native bounds. Native S0 maximum is 0.50, so B S0=.52 remains noncanonical. The isolated main restores only these enumerated raw requested values after normal clamp; every other input must equal its native-clamped value. Receipts preserve `input_was_canonical`, `within_genome_domain`, `raw_tip_prototype` and `raw_window_prototype`; `.evaluated.csv` and `.canonical.csv` separately retain actual and clamped genes. All requested widths satisfy the original >=3 mm filter, which was restored. No production bounds/tool constraint changed. Arm A contains 72 canonical and 72 noncanonical tuples despite its assignment label; Arm B contains 192 canonical and 384 noncanonical tuples.

Source audit: with `neck_full=1`, the inherited compound law substitutes the 6 mm blade core after the heel runout; distal `TAPER_X0/T_TIP` and native blade fuller raster changes do not implement a new taper in that law. `P1_ON=0` makes native `POCKET_D` inactive. The inherited compound-window clearance branch also uses the actual compound bevel extent, rather than the native tapered-stock clearance. Thus many gene tuples describe the same evaluated physical model. This is measured, not assumed: every receipt hashes actual T6 coordinates/connectivity and actual element thicknesses, and every tuple still has its own independent FEM solve.

Distinct coarse evaluated geometry/thickness hashes: Arm A **3**, Arm B **12**. `physical_equivalence.json` binds all independently evaluated aliases and verifies matching structural metrics. A new active physical law would require a separate assignment; no observed margin is attributed to an inactive parameter.

## Controls and complete hierarchical coverage

| Parent | Areas | Comparable scalar values | Max relative error |
| --- | --- | --- | --- |
| A | .25/.125/.0625 | 60 | 6.18786081701e-14 |
| B | .25/.125/.0625 | 60 | 6.96918593049e-14 |

Every tuple receives area 1.00. Every geometry-legal coarse numerical four-target pass receives area 0.25; every two-mesh pass receives area 0.0625. Top known-margin distinct physical fields, best native-domain representative and maximum-sequence contrasts per arm also receive all three screening meshes even when they fail earlier. Selection is deterministic and retained in `qualify_selection.json` / `export_selection.json`. Numerical screening is separate from domain eligibility. Reports never pool maxima from distinct designs or meshes.

| Arm | Tuples | Canonical | Geometry ineligible | Coarse pass | Area.25 observed | Two-mesh pass | Area.0625 observed | Three-mesh pass |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| A | 144 | 72 | 0 | 0 | 4 | 0 | 4 | 0 |
| B | 576 | 192 | 0 | 0 | 4 | 0 | 4 | 0 |

| Arm | Coarse sequence failures | Coarse spine failures | Coarse lateral failures | Coarse fatigue failures |
| --- | --- | --- | --- | --- |
| A | 0 | 144 | 96 | 96 |
| B | 48 | 576 | 96 | 96 |

## Complete representative three-mesh results

| Profile | Native domain | Min sequence | Min spine | Min lateral N | Min fatigue | Structural pass |
| --- | --- | --- | --- | --- | --- | --- |
| `armA_x138_t2.9_s4.5_e5.5_d0.3` | True | 1.252273422 | 0.692831302 | 327.574879322 | 0.915023116 | False |
| `armA_x144_t2.6_s4_e5_d0.6` | True | 1.331028513 | 0.687466484 | 298.109025901 | 0.832715410 | False |
| `armA_x144_t2.9_s3.5_e3.5_d0` | True | 1.410635821 | 0.675709555 | 236.111278767 | 0.659535550 | False |
| `armA_x144_t2.9_s3.5_e3.5_d0.6` | True | 1.410635821 | 0.675709555 | 236.111278767 | 0.659535550 | False |
| `armB_s0.05_w3_x144_t3_s4_e5_d0.2` | True | 1.326498572 | 0.546343956 | 290.706109037 | 0.812036657 | False |
| `armB_s0.5_w3_x144_t2.8_s4.5_e5.5_d0` | True | 1.215122538 | 0.725264676 | 337.700838841 | 0.943308213 | False |
| `armB_s0.5_w4.5_x144_t3_s4.5_e5.5_d0.1` | True | 1.214644331 | 0.725264088 | 337.820804302 | 0.943643315 | False |
| `armB_s0.52_w3_x144_t2.2_s4.5_e5.5_d0.1` | False | 1.216734841 | 0.725265009 | 337.997947831 | 0.944138135 | False |

Full scalar observations, blade/hand peak positions, actual element thickness, pocket weight and distances to the actual cutting edge/window are in `fem_summary.csv` and raw receipts. The Phase-9 source attribution remains corrected: strict peak (128.458205,-15.420706) lies about 0.752610 mm from the actual cutting edge and 30.789419 mm from the window, not at a window endcap.

| Arm / area | FEM observations | Peak on flat floor | Peak weight zero |
| --- | --- | --- | --- |
| A / 1 | 146 | 0 | 146 |
| A / 4 | 2 | 0 | 2 |
| A / 0.5 | 2 | 0 | 2 |
| A / 0.25 | 6 | 0 | 6 |
| A / 0.125 | 2 | 0 | 2 |
| A / 0.0625 | 6 | 0 | 6 |
| B / 1 | 578 | 0 | 578 |
| B / 0.0625 | 6 | 0 | 6 |
| B / 0.25 | 6 | 0 | 6 |
| B / 4 | 2 | 0 | 2 |
| B / 0.5 | 2 | 0 | 2 |
| B / 0.125 | 2 | 0 | 2 |

## Five-level ladder, canonical triplet and unchanged native Rust GCI

Each selected representative has actual solves at requested areas **1,.5,.25,.125,.0625**, plus **4**. Repeated screening-level physical hashes match exactly; scalar metrics reproduce within the 1e-10 relative source-parity tolerance, with measured maximum error retained in verification.json. Mesh SHA-256 binds serialized actual T6 coordinates, triangle connectivity and corner-node count, rather than a receipt ID. The independent physical hash additionally binds per-element thickness. GCI level h is exactly the floating-point `sqrt(max_area)` of that actual mesh setting, ordered finest to coarsest. Observable identity, units, region and projection are fixed across each triplet. The compiled harness directly includes production `crates/crescent/src/gci.rs` and calls its unchanged `compute_gci`; no Python substitute, patched tolerance, rounded ratio or fabricated h is used.

| Fine-to-coarse area triple | h1 | h2 | h3 | h2/h1 | h3/h2 | Native sequence |
| --- | --- | --- | --- | --- | --- | --- |
| .25/.5/1 | 0.5 | 0.7071067811865476 | 1 | 1.4142135623730951 | 1.414213562373095 | nonuniform_refinement |
| .125/.25/.5 | 0.3535533905932738 | 0.5 | 0.7071067811865476 | 1.414213562373095 | 1.4142135623730951 | nonuniform_refinement |
| .0625/.125/.25 | 0.25 | 0.3535533905932738 | 0.5 | 1.4142135623730951 | 1.414213562373095 | nonuniform_refinement |
| .25/1/4 | 0.5 | 1 | 2 | 2 | 2 | valid |
| .0625/.25/1 | 0.25 | 0.5 | 1 | 2 | 2 | valid |

Native fit totals: `{"admitted": 2, "asymptotic_validation": "not_established", "exact_r2_checks": 48, "fit_status_counts": {"admissible_estimate": 2, "nonpositive_order": 17, "oscillatory": 29, "unsupported_sequence": 72}, "native_gci_checks": 120, "sqrt2_refusals": 72}`. All exact-r2 checks preserve `sequence_valid=true` and `observable_valid=true`; an arithmetic fit may still be oscillatory, nonpositive-order, unresolved or admitted. Native analytic controls independently verify a positive p=2 / extrapolated 5 estimate, an oscillatory refusal and the exact √2 equality refusal. All estimates retain native `asymptotic_validation=not_established`. No asymptotic/continuum/physical convergence claim follows from an admitted relative estimate.

Both admitted fits concern the legacy Arm-A HM twist metric, with extrapolated safety factor about 0.453, below 1.0. **None of the four required structural observables has an admitted exact-r2 fit.** The two arithmetic admissions supply no structural qualification.

### `armA_x138_t2.9_s4.5_e5.5_d0.3`

| Area mm² | Sequence | Spine | Lateral N | Fatigue | Nodes | Triangles |
| --- | --- | --- | --- | --- | --- | --- |
| 4.0 | 1.322852863 | 0.857507798 | 363.460545665 | 1.015263447 | 1917 | 3193 |
| 1.0 | 1.252273422 | 0.692831302 | 386.771094454 | 1.080377387 | 4528 | 8299 |
| 0.5 | 1.351935744 | 0.821928132 | 324.767003834 | 0.907179807 | 9080 | 16861 |
| 0.25 | 1.284547153 | 1.038693567 | 370.897116174 | 1.036036206 | 16424 | 31393 |
| 0.125 | 1.346928522 | 1.057818658 | 355.891469063 | 0.994120556 | 32189 | 62275 |
| 0.0625 | 1.283096031 | 1.002078249 | 327.574879322 | 0.915023116 | 62891 | 122930 |

| Metric | Triplet | Status / reason | p | Extrapolated | Fine GCI (fraction) | Admitted |
| --- | --- | --- | --- | --- | --- | --- |
| seq_ratio | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| seq_ratio | stride2 | oscillatory / oscillatory_sequence | — | — | — | False |
| hm_spine_sf | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| hm_spine_sf | stride2 | oscillatory / oscillatory_sequence | — | — | — | False |
| lat_rob | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| lat_rob | stride2 | nonpositive_order / nonpositive_observed_order | -1.448443999 | — | — | False |
| fat_sf | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| fat_sf | stride2 | nonpositive_order / nonpositive_observed_order | -1.448443999 | — | — | False |
| sf_hunter | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| sf_hunter | stride2 | nonpositive_order / nonpositive_observed_order | -1.448443999 | — | — | False |
| hm_twist_sf | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| hm_twist_sf | stride2 | admissible_estimate / checked_relative_estimate | 2.502283488 | 0.453137144 | 0.011978017 | True |

### `armA_x144_t2.6_s4_e5_d0.6`

| Area mm² | Sequence | Spine | Lateral N | Fatigue | Nodes | Triangles |
| --- | --- | --- | --- | --- | --- | --- |
| 4.0 | 1.466820935 | 0.850690566 | 327.786992961 | 0.915615619 | 1917 | 3193 |
| 1.0 | 1.369513359 | 0.687466484 | 353.660766750 | 0.987889479 | 4528 | 8299 |
| 0.5 | 1.505390849 | 0.813408369 | 291.661206677 | 0.814704555 | 9080 | 16861 |
| 0.25 | 1.408856808 | 1.038753162 | 338.171213097 | 0.944622121 | 16424 | 31393 |
| 0.125 | 1.466287868 | 1.057129197 | 321.022196853 | 0.896719344 | 32189 | 62275 |
| 0.0625 | 1.331028513 | 1.001421235 | 298.109025901 | 0.832715410 | 62891 | 122930 |

| Metric | Triplet | Status / reason | p | Extrapolated | Fine GCI (fraction) | Admitted |
| --- | --- | --- | --- | --- | --- | --- |
| seq_ratio | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| seq_ratio | stride2 | oscillatory / oscillatory_sequence | — | — | — | False |
| hm_spine_sf | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| hm_spine_sf | stride2 | oscillatory / oscillatory_sequence | — | — | — | False |
| lat_rob | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| lat_rob | stride2 | nonpositive_order / nonpositive_observed_order | -1.370945615 | — | — | False |
| fat_sf | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| fat_sf | stride2 | nonpositive_order / nonpositive_observed_order | -1.370945615 | — | — | False |
| sf_hunter | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| sf_hunter | stride2 | nonpositive_order / nonpositive_observed_order | -1.370945615 | — | — | False |
| hm_twist_sf | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| hm_twist_sf | stride2 | admissible_estimate / checked_relative_estimate | 2.502283109 | 0.453137003 | 0.011978014 | True |

### `armB_s0.52_w3_x144_t2.2_s4.5_e5.5_d0.1`

| Area mm² | Sequence | Spine | Lateral N | Fatigue | Nodes | Triangles |
| --- | --- | --- | --- | --- | --- | --- |
| 4.0 | 1.208910493 | 0.899787772 | 397.717514047 | 1.110954295 | 2003 | 3313 |
| 1.0 | 1.216734841 | 0.725265009 | 398.068003726 | 1.111933327 | 4561 | 8297 |
| 0.5 | 1.093440828 | 0.870642500 | 401.543567541 | 1.121641706 | 9171 | 16933 |
| 0.25 | 1.250921371 | 1.038357494 | 380.867237384 | 1.063885995 | 16406 | 31238 |
| 0.125 | 1.259814328 | 1.063142121 | 380.513775856 | 1.062898662 | 32020 | 61764 |
| 0.0625 | 1.421907692 | 1.007134993 | 337.997947831 | 0.944138135 | 62394 | 121671 |

| Metric | Triplet | Status / reason | p | Extrapolated | Fine GCI (fraction) | Admitted |
| --- | --- | --- | --- | --- | --- | --- |
| seq_ratio | canonical | nonpositive_order / nonpositive_observed_order | -2.127385521 | — | — | False |
| seq_ratio | stride2 | nonpositive_order / nonpositive_observed_order | -2.322381009 | — | — | False |
| hm_spine_sf | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| hm_spine_sf | stride2 | oscillatory / oscillatory_sequence | — | — | — | False |
| lat_rob | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| lat_rob | stride2 | nonpositive_order / nonpositive_observed_order | -1.317471666 | — | — | False |
| fat_sf | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| fat_sf | stride2 | nonpositive_order / nonpositive_observed_order | -1.317471666 | — | — | False |
| sf_hunter | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| sf_hunter | stride2 | nonpositive_order / nonpositive_observed_order | -1.317471666 | — | — | False |
| hm_twist_sf | canonical | nonpositive_order / nonpositive_observed_order | -0.337004568 | — | — | False |
| hm_twist_sf | stride2 | oscillatory / oscillatory_sequence | — | — | — | False |

### `armB_s0.5_w3_x144_t2.8_s4.5_e5.5_d0`

| Area mm² | Sequence | Spine | Lateral N | Fatigue | Nodes | Triangles |
| --- | --- | --- | --- | --- | --- | --- |
| 4.0 | 1.209514952 | 0.899795819 | 397.518752225 | 1.110399089 | 2034 | 3371 |
| 1.0 | 1.215122538 | 0.725264676 | 398.596207537 | 1.113408772 | 4567 | 8309 |
| 0.5 | 1.093464615 | 0.870644330 | 401.534944481 | 1.121617619 | 9178 | 16944 |
| 0.25 | 1.250619062 | 1.038357452 | 380.959221083 | 1.064142936 | 16421 | 31257 |
| 0.125 | 1.258612740 | 1.063142596 | 381.130439334 | 1.064621204 | 31965 | 61641 |
| 0.0625 | 1.422207709 | 1.007135462 | 337.700838841 | 0.943308213 | 62294 | 121486 |

| Metric | Triplet | Status / reason | p | Extrapolated | Fine GCI (fraction) | Admitted |
| --- | --- | --- | --- | --- | --- | --- |
| seq_ratio | canonical | nonpositive_order / nonpositive_observed_order | -2.662226022 | — | — | False |
| seq_ratio | stride2 | nonpositive_order / nonpositive_observed_order | -2.273204443 | — | — | False |
| hm_spine_sf | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| hm_spine_sf | stride2 | oscillatory / oscillatory_sequence | — | — | — | False |
| lat_rob | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| lat_rob | stride2 | nonpositive_order / nonpositive_observed_order | -1.294375637 | — | — | False |
| fat_sf | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| fat_sf | stride2 | nonpositive_order / nonpositive_observed_order | -1.294375637 | — | — | False |
| sf_hunter | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| sf_hunter | stride2 | nonpositive_order / nonpositive_observed_order | -1.294375637 | — | — | False |
| hm_twist_sf | canonical | oscillatory / oscillatory_sequence | — | — | — | False |
| hm_twist_sf | stride2 | oscillatory / oscillatory_sequence | — | — | — | False |

## Matched complete native solid exports

Each selected profile runs complete native `certification::measure` with exactly the same genes, pocket/compound laws and native domain flags as its FEM observations. Original actual exported-facet inspection and apex/point/bore/census contracts are unchanged; export sampling is inherited h=0.05 mm, local surface maxarea=0.25 mm². Measurements apply only to that matched prototype; export pass cannot confer native genotype eligibility.

| Profile | Half-angle deg | Apex mm | Point | Bore mm | Census | Volume mm³ | Export pass | Native domain |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `armA_x138_t2.9_s4.5_e5.5_d0.3` | 24.324224296 | 0.195144636 | True | 1.600000000 | [0, 0, 1] | 30776.616625943 | True | True |
| `armA_x144_t2.6_s4_e5_d0.6` | 24.324224296 | 0.195144636 | True | 1.600000000 | [0, 0, 1] | 30600.160309792 | True | True |
| `armB_s0.52_w3_x144_t2.2_s4.5_e5.5_d0.1` | 24.324224296 | 0.195144636 | True | 1.600000000 | [0, 0, 1] | 30785.474144375 | True | False |
| `armB_s0.5_w3_x144_t2.8_s4.5_e5.5_d0` | 24.324224296 | 0.195144636 | True | 1.600000000 | [0, 0, 1] | 30764.031033614 | True | True |

## Verification, artifacts and disposition

`verification.json` status is **passed**: 80 production-file digests, 22 frozen Phase-9 input digests, exact compound/pocket/export/FEM source preservation, native FEM recovery after removing two observation hooks, original window minimum restored, unchanged direct native GCI/certification, literal input enumerations and domain flags, actual/evaluated CSV binding, exact protected-band/outside retention, 6 mm neck, metric identities, hierarchical coverage, physical alias/repeat checks, actual T6 mesh hashes, all GCI observable/value bindings and matched exports. All 748 executable invocations bind the same binary and CPUs 10,11 / jobs=2 / faer / effective niceness 19. Workflow Python/shell syntax checks pass. Prior seals and native dependency hashes are reverified at final sealing.

Primary output: `experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V7_REPORT.md`; byte-identical mirror: `target/gate-feasibility-bound/UNIFIED_PROTOTYPE_V7_REPORT.md`. Complete scripts, source provenance, search/selection records, raw receipts and logs, control parity, `physical_equivalence.json`, `fem_summary.csv`, `gci_requests.json`, native `gci_native.json`, `gci_results.json`, `summary.json`, `verification.json`, `run_provenance.json`, `REPORT_SHA256` and `SHA256SUMS` reside in `target/gate-feasibility-bound/unified_study_v7/`. The manifest binds both reports, the executable, all stable inputs and evidence. The report digest and verified artifact count are in the seal sidecars and both agent tracking files, avoiding a self-referential report hash.

Reproduce on the existing lane target:

```bash
nice -n 10 taskset -c 10,11 env CARGO_BUILD_JOBS=2 \
  bash target/gate-feasibility-bound/unified_study_v7/reproduce.sh
```

Phase-10 evaluation is complete. Await captain_jarambit before another scientific milestone. No production code edit, branch/worktree change, deletion, commit, push or PR. Both agent tracking files are updated at milestones. The explicit no-commit/no-push instruction is honored; no checkpoint command will be attempted. Any subsequent active taper/relief architecture or native GCI tolerance change requires a separate assignment.
