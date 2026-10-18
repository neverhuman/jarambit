# G16-12-RSV2-04 Phase-13: native delayed-taper and tip-thickness sweep

Task `G16-12-RSV2-04-p13`, `unified_study_v10`; execution commit `968fa148c290b9c1c01785c3d9ff1be629c7c6a9`; branch `audit/gate-feasibility-bound`; worktree `.`. CPUs **10,11**, `CARGO_BUILD_JOBS=2`, faer solver, requested `nice -n 10`, inherited effective niceness **19**. Assignment is frozen in `inputs/assignment.md`. No commit or push.

## Result

Executed all **36 distinct native gene tuples**: four legal Phase-12 spine families (`delta_in=0`, `delta_sp=0/.3/.6/.9`) × `TAPER_X0=144/147/150 mm` × `T_TIP=3.8/4.2/4.5 mm`. Native domain/geometry/aesthetic passes **36/36**; rejections **0**. Every legal tuple independently receives all **4 / 1 / .25 / .125 / .0625 mm²** native FEM meshes and its own complete native solid export. **180 grid FEM observations + 20 source-control observations = 200**; full exports **36**, full export passes **36**.

**The native taper/tip sweep adds no measured physical thickness or strength change under the retained full-core construction.** The 36 gene tuples produce **4 physical FEM models**, one per spine family: at every assigned mesh area, each family’s nine probes have exactly the Phase-12 T6 geometry and element-thickness hashes. Complete native exported-solid summaries, sharpness/point measurements and samples, axis thickness samples and worst-face geometry also match the corresponding Phase-12 source exactly. Across **3960 numeric grid/source comparisons**, maximum relative difference is **1.17096155686e-12**, consistent with solver roundoff. All 36 probes were actually evaluated; physical aliases were not substituted for solves or exports.

Joint structural/export passes remain **0** on .25/.125, **0** on .25/.125/.0625, and **0** on all five. Strict global HM twist at .0625 remains about **0.416721 / 0.450974 / 0.450558 / 0.518745** for spine offsets 0/.3/.6/.9 respectively. The earlier +24.4827% strict result is the Phase-12 spine widening effect; it is not a new taper gain and does not establish mesh-independent improvement or qualification.

## Why these native genes are inactive here

The retained Phase-10/11/12 compound construction is unchanged byte for byte. `compound::base_thickness_at` reads the native raster field, but with `neck_full=true` replaces the neck by stock and recovers to fixed `blade_core=6 mm` beyond the heel. That recovery is complete before the requested distal taper locations. The point-cap core is `g.knot_t(...)`; native `Genome::knot_t` returns stock thickness when P1 is off, as it is here. Compound window clearance uses the retained bevel distance law rather than native distal-taper clearance. Thus changing native `taper_x0` and `t_tip` does not introduce a distal taper into this retained physical law. The hashes, full export observations and measured differences above verify this for every assigned probe and area.

The initial native thickness-field calculation references the taper genes, but RS-v2 recomputes the final raster through `export::thickness_at` before calculating `mass_g`. Accordingly, raster mass also matches the corresponding source exactly. The raster mass and native solid volume are separate discretized observations, both recorded; neither indicates a new taper effect in this grid. This task introduces no new taper law, planform, loads, stress masks, material, thresholds, or export construction.

## Fixed construction and input identity

Each probe starts from its exact frozen Phase-12 `outward_in0_sp{0,.3,.6,.9}` CSV. Within that family only native gene indices 46 (`t_tip`) and 47 (`taper_x0`) change. Relative to the original Arm-B parent, the only additional differences are the assigned positive offsets on CP36/37/38; CP1/2/3 and all other genes remain unchanged. All requested values are native canonical without clipping. Four source controls preserve `t_tip=2.8`, `taper_x0=144` and rerun all five areas.

Fixed: stock/core/full neck 6 mm, P1 off; one native window S0/S1=.50/.85, W=3, fill=.65; pocket floor 2.2 mm, x=126..156, spine/edge guards 4.5/5.5 mm and original 1.5 mm cubic smoothstep (C1); bevel angle 18°, apex parameter .175 mm, d0/d1=.48/.65 mm; point shape 3, radial directional factor 2. Export sampling grid .05 mm, maxarea .25 mm² and original native certification measurements remain fixed. Protected bands and the full-stock neck are checked independently in every FEM/export receipt.

The native global twist metric remains `twist_joint_nmm / 16200`, equivalently `twist_sf * 1820 / 16200`. Native `twist_tipzone_sf` is a proxy SF at 1820 Nmm in the original 4<=r_tip<15 mm annulus; strict means the .0625 mm² result. Body HM uses the original r_tip>=15 mm region. Global excludes r_tip<4 mm and is never replaced with body-only capacity. Actual peak indices, selected width/thickness and section factor remain in raw receipts and `fem_summary.csv`.

## RS-v2 raster mass and native solid volume

| Spine Δ mm | Source RS-v2 raster g | Probe RS-v2 raster min/max g | Compound exported volume mm³ |
| --- | --- | --- | --- |
| 0.0 | 136.239533307 | 136.239533307 / 136.239533307 | 30764.031033614 |
| 0.3 | 136.455935111 | 136.455935111 / 136.455935111 | 30814.819726742 |
| 0.6 | 136.697905388 | 136.697905388 / 136.697905388 | 30864.114706358 |
| 0.9 | 136.929843292 | 136.929843292 / 136.929843292 | 30920.866531777 |

## Source-control parity

All four source profiles reproduce all five Phase-12 areas: **440 numeric comparisons**, maximum relative difference **4.85427964018e-13**, exact mesh/thickness hashes and genes. Six scientific harness files are unchanged from Phase-12; stripping the same three observation hooks recovers production FEM source exactly. Only the diagnostic assertion label, module paths and frozen provenance change in the executable wrapper.

| Control | Source | Numeric comparisons | Max relative difference |
| --- | --- | --- | --- |
| control_sp0 | outward_in0_sp0 | 110 | 8.9731875797e-14 |
| control_sp0.3 | outward_in0_sp0.3 | 110 | 1.11321613878e-13 |
| control_sp0.6 | outward_in0_sp0.6 | 110 | 9.41901690616e-14 |
| control_sp0.9 | outward_in0_sp0.9 | 110 | 4.85427964018e-13 |

## Complete probe results

Each row summarizes its own five actual native solves and full native export. Source-relative differences are checked at identical mesh areas; no per-mesh pooling or favorable resolution selection. Full per-mesh values and original diagnostic peaks are in the digest-bound CSV and raw JSON receipts.

| Probe | Spine Δ mm | TAPER_X0 mm | T_TIP mm | Strict tip proxy SF | Strict global HM | Fine-three min global HM | Max scalar relative Δ vs source | Full export | Fine-three joint |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| taper_sp0_x144_t3.8 | 0.0 | 144.0 | 3.8 | 3.709271632 | 0.416720640 | 0.416720640 | 5.56e-14 | True | False |
| taper_sp0_x144_t4.2 | 0.0 | 144.0 | 4.2 | 3.709271632 | 0.416720640 | 0.416720640 | 8.97e-14 | True | False |
| taper_sp0_x144_t4.5 | 0.0 | 144.0 | 4.5 | 3.709271632 | 0.416720640 | 0.416720640 | 7.53e-14 | True | False |
| taper_sp0_x147_t3.8 | 0.0 | 147.0 | 3.8 | 3.709271632 | 0.416720640 | 0.416720640 | 8.97e-14 | True | False |
| taper_sp0_x147_t4.2 | 0.0 | 147.0 | 4.2 | 3.709271632 | 0.416720640 | 0.416720640 | 1.21e-13 | True | False |
| taper_sp0_x147_t4.5 | 0.0 | 147.0 | 4.5 | 3.709271632 | 0.416720640 | 0.416720640 | 4.47e-14 | True | False |
| taper_sp0_x150_t3.8 | 0.0 | 150.0 | 3.8 | 3.709271632 | 0.416720640 | 0.416720640 | 8.26e-14 | True | False |
| taper_sp0_x150_t4.2 | 0.0 | 150.0 | 4.2 | 3.709271632 | 0.416720640 | 0.416720640 | 8.26e-14 | True | False |
| taper_sp0_x150_t4.5 | 0.0 | 150.0 | 4.5 | 3.709271632 | 0.416720640 | 0.416720640 | 9.05e-14 | True | False |
| taper_sp0.3_x144_t3.8 | 0.3 | 144.0 | 3.8 | 4.014168522 | 0.450974488 | 0.404390387 | 8.8e-14 | True | False |
| taper_sp0.3_x144_t4.2 | 0.3 | 144.0 | 4.2 | 4.014168522 | 0.450974488 | 0.404390387 | 6.72e-14 | True | False |
| taper_sp0.3_x144_t4.5 | 0.3 | 144.0 | 4.5 | 4.014168522 | 0.450974488 | 0.404390387 | 1.11e-13 | True | False |
| taper_sp0.3_x147_t3.8 | 0.3 | 147.0 | 3.8 | 4.014168522 | 0.450974488 | 0.404390387 | 9.21e-14 | True | False |
| taper_sp0.3_x147_t4.2 | 0.3 | 147.0 | 4.2 | 4.014168522 | 0.450974488 | 0.404390387 | 8.1e-14 | True | False |
| taper_sp0.3_x147_t4.5 | 0.3 | 147.0 | 4.5 | 4.014168522 | 0.450974488 | 0.404390387 | 6.69e-14 | True | False |
| taper_sp0.3_x150_t3.8 | 0.3 | 150.0 | 3.8 | 4.014168522 | 0.450974488 | 0.404390387 | 9.78e-14 | True | False |
| taper_sp0.3_x150_t4.2 | 0.3 | 150.0 | 4.2 | 4.014168522 | 0.450974488 | 0.404390387 | 9.21e-14 | True | False |
| taper_sp0.3_x150_t4.5 | 0.3 | 150.0 | 4.5 | 4.014168522 | 0.450974488 | 0.404390387 | 4.77e-14 | True | False |
| taper_sp0.6_x144_t3.8 | 0.6 | 144.0 | 3.8 | 4.010460592 | 0.450557918 | 0.433074064 | 9.79e-14 | True | False |
| taper_sp0.6_x144_t4.2 | 0.6 | 144.0 | 4.2 | 4.010460592 | 0.450557918 | 0.433074064 | 1.42e-13 | True | False |
| taper_sp0.6_x144_t4.5 | 0.6 | 144.0 | 4.5 | 4.010460592 | 0.450557918 | 0.433074064 | 1.09e-13 | True | False |
| taper_sp0.6_x147_t3.8 | 0.6 | 147.0 | 3.8 | 4.010460592 | 0.450557918 | 0.433074064 | 1.09e-13 | True | False |
| taper_sp0.6_x147_t4.2 | 0.6 | 147.0 | 4.2 | 4.010460592 | 0.450557918 | 0.433074064 | 1.42e-13 | True | False |
| taper_sp0.6_x147_t4.5 | 0.6 | 147.0 | 4.5 | 4.010460592 | 0.450557918 | 0.433074064 | 1.23e-13 | True | False |
| taper_sp0.6_x150_t3.8 | 0.6 | 150.0 | 3.8 | 4.010460592 | 0.450557918 | 0.433074064 | 4.01e-14 | True | False |
| taper_sp0.6_x150_t4.2 | 0.6 | 150.0 | 4.2 | 4.010460592 | 0.450557918 | 0.433074064 | 1.95e-13 | True | False |
| taper_sp0.6_x150_t4.5 | 0.6 | 150.0 | 4.5 | 4.010460592 | 0.450557918 | 0.433074064 | 1.34e-13 | True | False |
| taper_sp0.9_x144_t3.8 | 0.9 | 144.0 | 3.8 | 4.617400433 | 0.518744987 | 0.431106525 | 9.45e-13 | True | False |
| taper_sp0.9_x144_t4.2 | 0.9 | 144.0 | 4.2 | 4.617400433 | 0.518744987 | 0.431106525 | 5.69e-13 | True | False |
| taper_sp0.9_x144_t4.5 | 0.9 | 144.0 | 4.5 | 4.617400433 | 0.518744987 | 0.431106525 | 5.69e-13 | True | False |
| taper_sp0.9_x147_t3.8 | 0.9 | 147.0 | 3.8 | 4.617400433 | 0.518744987 | 0.431106525 | 1.17e-12 | True | False |
| taper_sp0.9_x147_t4.2 | 0.9 | 147.0 | 4.2 | 4.617400433 | 0.518744987 | 0.431106525 | 8.16e-13 | True | False |
| taper_sp0.9_x147_t4.5 | 0.9 | 147.0 | 4.5 | 4.617400433 | 0.518744987 | 0.431106525 | 9.45e-13 | True | False |
| taper_sp0.9_x150_t3.8 | 0.9 | 150.0 | 3.8 | 4.617400433 | 0.518744987 | 0.431106525 | 8.5e-13 | True | False |
| taper_sp0.9_x150_t4.2 | 0.9 | 150.0 | 4.2 | 4.617400433 | 0.518744987 | 0.431106525 | 9.45e-13 | True | False |
| taper_sp0.9_x150_t4.5 | 0.9 | 150.0 | 4.5 | 4.617400433 | 0.518744987 | 0.431106525 | 9.45e-13 | True | False |

## All five meshes, grouped only after exact physical identity checks

Representative is the first independently evaluated probe per family. The remaining eight each have their own receipts and match its physical hashes at every area. This grouping describes verified aliases, not reduced execution.

| Spine Δ mm | Area mm² | Sequence | Spine SF | Lateral N | Fatigue SF | Ringback SF | Body HM twist | Global HM twist | Tip proxy SF |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0.0 | 4.0 | 1.209514952 | 0.899795819 | 397.518752225 | 1.110399089 | 5.127088382 | 1.205617800 | 0.437186543 | 3.891440656 |
| 0.0 | 1.0 | 1.215122538 | 0.725264676 | 398.596207537 | 1.113408772 | 5.055821842 | 1.185531763 | 0.416783980 | 3.709835429 |
| 0.0 | 0.25 | 1.250619062 | 1.038357452 | 380.959221083 | 1.064142936 | 5.131770352 | 1.082833538 | 0.424203778 | 3.775879781 |
| 0.0 | 0.125 | 1.258612740 | 1.063142596 | 381.130439334 | 1.064621204 | 4.992111565 | 1.021661607 | 0.435482731 | 3.876274861 |
| 0.0 | 0.0625 | 1.422207709 | 1.007135462 | 337.700838841 | 0.943308213 | 5.064628196 | 1.310617549 | 0.416720640 | 3.709271632 |
| 0.3 | 4.0 | 1.258506224 | 0.963204339 | 376.164235583 | 1.050748982 | 5.202661834 | 1.370809910 | 0.504700484 | 4.492388925 |
| 0.3 | 1.0 | 1.143522880 | 0.889032764 | 379.261909918 | 1.059401794 | 5.128575149 | 1.401541746 | 0.426819695 | 3.799164316 |
| 0.3 | 0.25 | 1.148828361 | 1.040153812 | 387.196177556 | 1.081564782 | 5.231792318 | 1.087742839 | 0.404390387 | 3.599518829 |
| 0.3 | 0.125 | 1.110779141 | 1.007651964 | 396.935755704 | 1.108770590 | 5.001267803 | 1.316004715 | 0.435760742 | 3.878749459 |
| 0.3 | 0.0625 | 1.279786041 | 0.970383886 | 372.128958319 | 1.039477141 | 4.944562955 | 1.099065741 | 0.450974488 | 4.014168522 |
| 0.6 | 4.0 | 1.052025004 | 0.962212957 | 385.567116287 | 1.077014285 | 5.396308000 | 1.142258008 | 0.574982564 | 5.117976671 |
| 0.6 | 1.0 | 1.252978311 | 1.093370672 | 354.614071029 | 0.990552368 | 5.208015933 | 1.585166827 | 0.333786973 | 2.971070857 |
| 0.6 | 0.25 | 1.144896190 | 1.061876619 | 403.241481298 | 1.126384531 | 5.119626292 | 1.375656489 | 0.433074064 | 3.854835074 |
| 0.6 | 0.125 | 1.188302691 | 1.055551282 | 388.853276191 | 1.086193597 | 5.011272996 | 1.438717861 | 0.440014729 | 3.916614621 |
| 0.6 | 0.0625 | 1.194944019 | 1.057300697 | 388.743647245 | 1.085887368 | 4.924846522 | 1.366495697 | 0.450557918 | 4.010460592 |
| 0.9 | 4.0 | 1.065379132 | 0.723929481 | 367.162046492 | 1.025602942 | 5.204306404 | 1.258750271 | 0.546709897 | 4.866318864 |
| 0.9 | 1.0 | 1.156204061 | 0.958509182 | 380.524706649 | 1.062929196 | 5.090785760 | 1.249344366 | 0.468435953 | 4.169594747 |
| 0.9 | 0.25 | 1.070193645 | 1.146651756 | 433.395372684 | 1.210614151 | 5.072760691 | 1.377772455 | 0.468287461 | 4.168273008 |
| 0.9 | 0.125 | 1.217761572 | 1.014909975 | 360.778229190 | 1.007770865 | 5.039643999 | 1.453942694 | 0.431106525 | 3.837321813 |
| 0.9 | 0.0625 | 1.302321675 | 0.995021961 | 363.067022522 | 1.014164209 | 5.001950526 | 1.441576150 | 0.518744987 | 4.617400433 |

Joint comparison thresholds retained from prior phases: sequence>=1.20, spine>=1, lateral>=260 N, fatigue>=1, ringback>=1, body HM twist>=1 and global HM twist>=1, plus matched full native export. Fatigue SF=1 requires about 357.9963 N under the unchanged native law, so lateral 260 N alone is insufficient. Native global HM twist remains below 1 at every requested mesh. These are results for this discrete grid under the retained construction, not a global infeasibility statement or production all-hard certificate.

## Native solid exports

Every probe independently runs complete native `certification::measure`; original Gate S facet half-angle/apex, point profile/monotonicity, bore radius, solid census and positive volume checks are retained. The native source label inside `measurement` is preserved verbatim; actual export configuration is bound separately in each receipt. Per-family observations are exact aliases of the frozen Phase-12 export.

| Probe | Half-angle ° | Apex mm | Point | Bore mm | Census | Volume mm³ | Full pass |
| --- | --- | --- | --- | --- | --- | --- | --- |
| taper_sp0_x144_t3.8 | 24.324224296 | 0.195144636 | True | 1.600000000 | [0, 0, 1] | 30764.031033614 | True |
| taper_sp0_x144_t4.2 | 24.324224296 | 0.195144636 | True | 1.600000000 | [0, 0, 1] | 30764.031033614 | True |
| taper_sp0_x144_t4.5 | 24.324224296 | 0.195144636 | True | 1.600000000 | [0, 0, 1] | 30764.031033614 | True |
| taper_sp0_x147_t3.8 | 24.324224296 | 0.195144636 | True | 1.600000000 | [0, 0, 1] | 30764.031033614 | True |
| taper_sp0_x147_t4.2 | 24.324224296 | 0.195144636 | True | 1.600000000 | [0, 0, 1] | 30764.031033614 | True |
| taper_sp0_x147_t4.5 | 24.324224296 | 0.195144636 | True | 1.600000000 | [0, 0, 1] | 30764.031033614 | True |
| taper_sp0_x150_t3.8 | 24.324224296 | 0.195144636 | True | 1.600000000 | [0, 0, 1] | 30764.031033614 | True |
| taper_sp0_x150_t4.2 | 24.324224296 | 0.195144636 | True | 1.600000000 | [0, 0, 1] | 30764.031033614 | True |
| taper_sp0_x150_t4.5 | 24.324224296 | 0.195144636 | True | 1.600000000 | [0, 0, 1] | 30764.031033614 | True |
| taper_sp0.3_x144_t3.8 | 24.211856857 | 0.195140515 | True | 1.600000000 | [0, 0, 1] | 30814.819726742 | True |
| taper_sp0.3_x144_t4.2 | 24.211856857 | 0.195140515 | True | 1.600000000 | [0, 0, 1] | 30814.819726742 | True |
| taper_sp0.3_x144_t4.5 | 24.211856857 | 0.195140515 | True | 1.600000000 | [0, 0, 1] | 30814.819726742 | True |
| taper_sp0.3_x147_t3.8 | 24.211856857 | 0.195140515 | True | 1.600000000 | [0, 0, 1] | 30814.819726742 | True |
| taper_sp0.3_x147_t4.2 | 24.211856857 | 0.195140515 | True | 1.600000000 | [0, 0, 1] | 30814.819726742 | True |
| taper_sp0.3_x147_t4.5 | 24.211856857 | 0.195140515 | True | 1.600000000 | [0, 0, 1] | 30814.819726742 | True |
| taper_sp0.3_x150_t3.8 | 24.211856857 | 0.195140515 | True | 1.600000000 | [0, 0, 1] | 30814.819726742 | True |
| taper_sp0.3_x150_t4.2 | 24.211856857 | 0.195140515 | True | 1.600000000 | [0, 0, 1] | 30814.819726742 | True |
| taper_sp0.3_x150_t4.5 | 24.211856857 | 0.195140515 | True | 1.600000000 | [0, 0, 1] | 30814.819726742 | True |
| taper_sp0.6_x144_t3.8 | 24.106336838 | 0.195141727 | True | 1.600000000 | [0, 0, 1] | 30864.114706358 | True |
| taper_sp0.6_x144_t4.2 | 24.106336838 | 0.195141727 | True | 1.600000000 | [0, 0, 1] | 30864.114706358 | True |
| taper_sp0.6_x144_t4.5 | 24.106336838 | 0.195141727 | True | 1.600000000 | [0, 0, 1] | 30864.114706358 | True |
| taper_sp0.6_x147_t3.8 | 24.106336838 | 0.195141727 | True | 1.600000000 | [0, 0, 1] | 30864.114706358 | True |
| taper_sp0.6_x147_t4.2 | 24.106336838 | 0.195141727 | True | 1.600000000 | [0, 0, 1] | 30864.114706358 | True |
| taper_sp0.6_x147_t4.5 | 24.106336838 | 0.195141727 | True | 1.600000000 | [0, 0, 1] | 30864.114706358 | True |
| taper_sp0.6_x150_t3.8 | 24.106336838 | 0.195141727 | True | 1.600000000 | [0, 0, 1] | 30864.114706358 | True |
| taper_sp0.6_x150_t4.2 | 24.106336838 | 0.195141727 | True | 1.600000000 | [0, 0, 1] | 30864.114706358 | True |
| taper_sp0.6_x150_t4.5 | 24.106336838 | 0.195141727 | True | 1.600000000 | [0, 0, 1] | 30864.114706358 | True |
| taper_sp0.9_x144_t3.8 | 23.800291946 | 0.195143062 | True | 1.600000000 | [0, 0, 1] | 30920.866531777 | True |
| taper_sp0.9_x144_t4.2 | 23.800291946 | 0.195143062 | True | 1.600000000 | [0, 0, 1] | 30920.866531777 | True |
| taper_sp0.9_x144_t4.5 | 23.800291946 | 0.195143062 | True | 1.600000000 | [0, 0, 1] | 30920.866531777 | True |
| taper_sp0.9_x147_t3.8 | 23.800291946 | 0.195143062 | True | 1.600000000 | [0, 0, 1] | 30920.866531777 | True |
| taper_sp0.9_x147_t4.2 | 23.800291946 | 0.195143062 | True | 1.600000000 | [0, 0, 1] | 30920.866531777 | True |
| taper_sp0.9_x147_t4.5 | 23.800291946 | 0.195143062 | True | 1.600000000 | [0, 0, 1] | 30920.866531777 | True |
| taper_sp0.9_x150_t3.8 | 23.800291946 | 0.195143062 | True | 1.600000000 | [0, 0, 1] | 30920.866531777 | True |
| taper_sp0.9_x150_t4.2 | 23.800291946 | 0.195143062 | True | 1.600000000 | [0, 0, 1] | 30920.866531777 | True |
| taper_sp0.9_x150_t4.5 | 23.800291946 | 0.195143062 | True | 1.600000000 | [0, 0, 1] | 30920.866531777 | True |

## Physical identity SHA-256 by family and area

| Spine family source | Area mm² | T6 geometry and actual element-thickness SHA-256 |
| --- | --- | --- |
| outward_in0_sp0 | 4.0 | 0637ab884c1ed9cf873f5fdb744fb2ccaaf7d53a06cb74e70acc94ff50cf660c |
| outward_in0_sp0 | 1.0 | a2f533938afc1171a4a6e57f905074302573ef287994dcd7c3fddbfc147d00fa |
| outward_in0_sp0 | 0.25 | 1c8240410f32564431c986084ad190b83fa982ea22e403074c844aa3c3ec5772 |
| outward_in0_sp0 | 0.125 | 682839f47cdf45c342d3b6d321bc9455a07304e3f650ac0365161a918da6fd35 |
| outward_in0_sp0 | 0.0625 | e66e5a7a19514c6629ce016dcd7090d9a1af1c3d7fc91f8c203dda2645aa254f |
| outward_in0_sp0.3 | 4.0 | 3e24100376c6237794d085576977e10a7b7197fdfcda33aeb8dc31f39417f765 |
| outward_in0_sp0.3 | 1.0 | c2f64bb7107c4d886981d7ee3aab57c74178e3981b9a5eafd92d47f91e63e713 |
| outward_in0_sp0.3 | 0.25 | 66b16ce69e9fa546e4cf311a436d657383f6d0d06f218e1830d18b8fa316521e |
| outward_in0_sp0.3 | 0.125 | d869ea0b2073511706b5c9acbe556c4dda494401c33f2e5c5c289f235e5b3894 |
| outward_in0_sp0.3 | 0.0625 | 306cbb3bf7ae0883b37f37d3ef6d03a81cf9174fcb448d0e1e92d449e58f52ca |
| outward_in0_sp0.6 | 4.0 | 3c7134d88016b0e7d94c68ad5d56a33f1605208bc23821460c4a8a306b6b7e51 |
| outward_in0_sp0.6 | 1.0 | 3d9a4b3fe724df46e719cd2359dd75ad2c7835e8fcc160a1f284e9b493ba3bef |
| outward_in0_sp0.6 | 0.25 | 13364a15e0765024b28e79e3840a4f7c3b6c1f028abda02397625d20d6de54de |
| outward_in0_sp0.6 | 0.125 | 48f15c85f35d5e16546d8a7222defa08da2416e8aacee508b4c34837a60cf960 |
| outward_in0_sp0.6 | 0.0625 | 9fc3f9844576b68672fe2f17d7783047dd089cd0e3e53e04e1fd242b71d9133a |
| outward_in0_sp0.9 | 4.0 | 7a8ea5ae5b3fa960b846643eff510936bb88e6e98734547561d2939a7c9aceb3 |
| outward_in0_sp0.9 | 1.0 | 8a782f32680bcef81e65a7d654224d322561a75f2915a0b3213493eb1438f46b |
| outward_in0_sp0.9 | 0.25 | a13144d0619f67c564e544696e3da0959bcfdc6ed5199868e2f640850779fe62 |
| outward_in0_sp0.9 | 0.125 | b598cbd7899508e105e43c5a814452eb30fe9b9fa3c50a25f679c56af0ce75d5 |
| outward_in0_sp0.9 | 0.0625 | 1304141f997db40f3877fd11dd6823b7efc56db4a649d42ac2ba00a7785c7b29 |

## Integrity and reproducibility

`verification.json` passes: **80** production digests, **37** frozen inputs, exact permitted gene changes, native canonicality/domain/gates, all legal five-area and matched complete exports, inherited scientific source identity, native stress masks/peaks/metric/torque identities, protected bands/full-stock neck, per-family physical hashes and complete source export observation identity. All **112** invocations bind the same executable, CPUs 10,11, jobs=2, faer and effective niceness 19. Workflow syntax checks pass. Dependency digests and all nine earlier seals are checked at final sealing.

| Artifact | SHA-256 |
| --- | --- |
| study | 591fe851d99b8f5c86dae6c5fff1736df29dad316fbb20fff780c210d2f33b8a |
| source_provenance.json | e2dac51aa9b991ab2fcde296a52297f4c0aa415f792066f74e2e583375ee42d7 |
| verification.json | 15c3c6fb9c6ca7bf12fe0f5f6cb3a7cd5e5a96710b850b1feb386e7a7af5690b |
| control_verification.json | a3e0a30e19bd62cb4d7449e14aaaba68a6ba5f4bed1e77a11729a7e634aa0d9b |
| physical_aliases.json | 74f369f054e50715f8417d5f9f6febe1b5b87862de692bbb8f25af9460721db4 |
| fem_summary.csv | 6277e55aa4e183762ab91b20e8a1902b61c18bca9566537cbe89584e0a824324 |
| summary.json | 4f7b7f6226da6a26388c3812a7befaa385e4f786436c11e6ab433fab13d8fed9 |
| inputs/assignment.md | b577b9152ac21232df8ad9b25475df28694d39e6d9deaf1681d333c4f5ba0214 |

| Probe | Input CSV SHA-256 | FEM receipt SHA-256 | Export receipt SHA-256 |
| --- | --- | --- | --- |
| taper_sp0_x144_t3.8 | 76ecddcedb3b1f6bf1273be90dff6f7b455fbd2101ffd5a0749f239b4a69351b | 73daff4b40f703c2bb50dbacc4d08bd2d2b90d38322638075111c91912cb7b3f | e1af7c9944046e8e73ae3d595bb4403016626edbb66e8a6b0f24eb656b195b4e |
| taper_sp0_x144_t4.2 | b07c18e40b4eb29ee1e3f798e5830d12d6336ba9765415ddb4d830d8ec2747a7 | 2b103e69f6fc0d9ed32c18093506782b252a2638725975af1577b780dc78f500 | 54145a1f83d0b5a3dfaf28d1dfcb18e1a06a11ade80a8cabe4f94d7bd9064243 |
| taper_sp0_x144_t4.5 | bd275686745664b2c576e4c476647ad45e10d692e5f53ee609580d73173fae2d | 0ff202c9306c05a4a08adf6ea7d169aa5819864d3e77c06834faa469c5370df4 | 881ddc1144d571d44cdf24628b658ddcb895ea60a3781b19c6d544ffc09d3434 |
| taper_sp0_x147_t3.8 | ae193e6ce09ad092df2e6dce3b251892967f22065179f28399425f450b9b616c | b39c5862a007f700baafd1bdf874f5ad399622b08d9560fb7d0b4be56c838c88 | b4991b64ad08bade8ed4bf63d2f351245af1dbfe5e5c7a66029a44fd2d8e782b |
| taper_sp0_x147_t4.2 | 062535e03f96958431dcf58bfacdc594504a2aa27f117bcf01e5dd24fd0ac091 | 4079f4b1688631a42848b14bc5cc5818c42be60378d7c856aea15bca0689360f | ffc7c766567baa9da119d226aa1239269583c4af77b6ad4317e7cb142ff53fdf |
| taper_sp0_x147_t4.5 | c8948e803161504dfd91e4ad2c547fbac5920ef442dbe98465c56a0463ec19cc | d8875161da7814c8aa71410ad431ae89f5085da9cba45b585e5da2851e5d5edc | d81aadf5a2c0da770ba6d3ea8ab6bd039293b112f41bb888adcee84b4371d84e |
| taper_sp0_x150_t3.8 | e80cb46c3fbf1647fa720a7bba9f5b9343b30d4e115e4b8b63d3cbfb41d84cf5 | d784c4967c582d6c344aed733de2624e38af9ab65eacab5fa9ad2d8add61fbe0 | 08bb40f4e75468c6c7e06cbbce8412bed0339f71b749ae74ca0a826a6ca1bb6a |
| taper_sp0_x150_t4.2 | c7539ad206764d02be63c4efdd755cd2620feb68a8d49d39172e6a390f039aaa | 6c192530bc88f9389ce90a3f24629b4f24067d4e61c5b2d34d3e8b3b0beaf58d | 416128e5621f46a34a32fa1dbfc50c16ce53b09286afb02bc352a7a4d8d74212 |
| taper_sp0_x150_t4.5 | d2cfe6936714cf12b53f984ec49b39c5e14614f4a2256adbd2b2f4fde6398319 | 1121853395af19ffa99997b02f136f07edfb95f2b698011e2bc8e776c6c364ac | 27cfa65bd3f25b3e283dd5e2227ad361a9f72cbf365980387fa8502e255f6fba |
| taper_sp0.3_x144_t3.8 | 82a116a99bd6815e3fa4bd5378dd13c7046d9b939ea9e458d064cbb2573dd53f | dfab9391735c3f1470eceb39def7a6780452a6ecbdac512e77b921a79eecc2ce | 1d983338ad82e8061a01f4fefd7fffece27b3a12c6318ac6ce8e6e0ec0c2f7cc |
| taper_sp0.3_x144_t4.2 | 9cb1df74d5abbaed8a935fe841002e9040305af4887779516d9f2241497a5542 | 5a3e20cea53162fc653771096823ce2ea631fa3d5c466192762a45733273d229 | b78985f87474a2551087f9275742468f1c8b1fba8a65ace486def649583c32b8 |
| taper_sp0.3_x144_t4.5 | ca5695e49ca80699a9b7254913f80de9357b60753ea125e3cfd1ffce456b49cc | 4e031127aa353d09fc3435cbe257375ca8ae6df468df9ca57ac3a82a5086dc4d | df5f1289cdcfa51b450dbaac487c705a8a65127fb36684946391ba1432ea6865 |
| taper_sp0.3_x147_t3.8 | b2c4f500ddf52d67fa220464d1ebf1c5c5adbb19f0b6248f03167654f7c47e31 | 042ce2e32ba266cae94c952ad638e4da563da4a1fd7babfce9b55a970b1f7758 | 9ee746e7d0699147d164ffca5cbbf83596aa667fcd277d42da9f36f64bb580cf |
| taper_sp0.3_x147_t4.2 | d8f1b1ec112a628de63822939b0d930966fef3607a30dc916a6d73344fb8e782 | 18f91e861ddc30533c9b3412e2266ce216d7ad6cc406727be0dffbd7a796d10a | 301db233a11b837028ad08d6aa1462557003fc94a59c6135e161a78b610a9c3d |
| taper_sp0.3_x147_t4.5 | ee5512411f4c4f928436c500bfd1b042f0845a4a320704a7cfac79a68ddb0c1e | 80cec23d16438b880316b060fb3f91360bd5cc60c36e60fbe7f6dbcd4601bf35 | 029ae718d5345525ff8bb33366d7b9e83cda11e4ea913695e59d5f6375a7816d |
| taper_sp0.3_x150_t3.8 | d030f7c81838b207c8afd1cbd51a1a8607cb2b646a1b5fd9dd448fb4e32806cf | 7593a1053ed04fac31da9fae7499f9350aa38faa5a284d66f288c56fed55ef74 | 48ab0cdca5f58f814bd4eb1b0c0bef0a5a0ea89706effc51c2cb4e5d2fd1ca71 |
| taper_sp0.3_x150_t4.2 | acb417190ccf285624844e1978163d7a41787d8bb9547da190bac09ece288764 | 08d8421a6a64eb0d7ea07ac7971d88c3eb01421fa76223450dd40d5bbdd20989 | 61efd0bc9a7679504d9c47c2a9b80bcd4ea6f0e4ba2ea88e9450dbd6170efa0a |
| taper_sp0.3_x150_t4.5 | a081211e1289f294991b6355f4fc8d1dd92c9028aa4ba55e4d6420e6d25bfa50 | 4ff0cd20334cba3425a2e0dd996052f29a4df8744c6349263ea55d48b0c86b01 | 6299c609dc417b21e56c1d9d9861b15da3856008aba0fc9cde4fd2bf686d954e |
| taper_sp0.6_x144_t3.8 | dc65c12ae9ddec4b1e78ec46cc5b6bcc5d45f28dc7eb598a8dd3df6116c8ed59 | 2264d3ad1db4ddb8c3fc6333735f6b64a51e4d5ee22bec7f69f571cc1d060d03 | fa71441d2f122fdc818f89eb9154ff6f767c3327e1b62c0790f0063bc83ad9b1 |
| taper_sp0.6_x144_t4.2 | 1da7d184fab0eb5ea97d8a5d564ecff8325b14e8d1eea5490a756e52a9b4838f | c152ecf2e156f3895aaaaeb28cbe768941f63445861ce6bba36a61607247de37 | 76d4e0ff493053258b52361687e713b8b9c3afbaa54e2f954c89f949e6a32d24 |
| taper_sp0.6_x144_t4.5 | 57c9b74993a6b7b1a2a6477c078500895a4f46eb159e08549975d0fd420bcb29 | 79f1521aeeaec10b89f7ef2c8948135d4e355d06dd85e4de93b111a9ee7d2aa9 | 2ae1aeadc3860012d47429fbb31a9ea389d96ca343d3c0ca3cc8205e502b9376 |
| taper_sp0.6_x147_t3.8 | d9900da87ffb2fa3362b56381444f45a5ab9d8fdb528de28272b6b2560c58bd5 | bee01f4954815a2554c490c880e05f0cdc1f509bfed7928fabc2fa75a5fdae8a | abbe6e52a0f8d98adf2ef69a2d2c673b849e446e533efbc97093142bd3a3476a |
| taper_sp0.6_x147_t4.2 | 15ce0ed0b3c434f109595c91a503b8577c00df531f490d47dc9d8a28b4a8ea67 | 1b93bc50b76b8d00e6c4f8cac878d385b77095d7e30b2f00e06f508577b271d4 | f0d4645087a109d82e02d558167d9713a279d1ed69ea714ad7f3c60970110d59 |
| taper_sp0.6_x147_t4.5 | 8c4db784222826792a5ff4ffb385740734053ffe510a97642b3820fdf85f5d38 | ddf13723bb2e67b279bb0f02e958809ffb382da601b53cbb35a61ebb7d7423ce | 74346df6ab0c7da050ad9900a3fd486164e8ba8c645a5c80f43762b9317dc4fa |
| taper_sp0.6_x150_t3.8 | 9fdfedc13cd84cf4ae6fc75cab0a2e827e37f38551255e007c986c2df40d54a9 | 8bdabf102f91f02343bfe0651c6134ad214ac3840a4266f631b212f8c807383a | 18d1d5279086981234692add6f975891d3831d4641c15ce5070fc4861a05d1c5 |
| taper_sp0.6_x150_t4.2 | 164c3321dc192fbe24fad33a5ffc27e5b7a032a10e9183857fed3ef9f5998461 | 856fa5d84ba7c4afd54d62a6f9d1c221a4ec82d9384484cb9dc593adc45d1f2a | 54a5b7352ea38a5cacab2819729ba8e1dcc6a8dcdc682c7df796314644ee9565 |
| taper_sp0.6_x150_t4.5 | 8dec26eae7901a040fdb8fba2dfca29e720f88664129189b5af5b0a6c24fcfb2 | bd5af635a451dba8acfaa9f0cded6f0c84469a3ca73bdd15c3905a55d1024f71 | 1419717901d3f093d2f7ef944666a8dcee623807bd3525d36dde2b997f2952b6 |
| taper_sp0.9_x144_t3.8 | 97f39cc1c208498733aa2235efb672ef522daa3a21d8c2b0e2c2fea6bf35e086 | f9869e7c3d797c7dae968cab8bd8c4c0a7b1afab71f94b9fb85b259a6839d89b | d95121392f6991d4a9d7531ae25b79243b02b0c983aeef71a2b7d1ed0cd8893f |
| taper_sp0.9_x144_t4.2 | 3002db4b2fd602cfc1db2a60c3e7a5480fa324e49440bb70db7d002efc2163f1 | 1f2c20555648aa64f926efc199d56e0395fa4685fcc09aa2f8c57bad4e18fbb8 | e9356585a95b80b3237efeed7663bb3e91f714a1558fac0ac9bb3865e24c0c4b |
| taper_sp0.9_x144_t4.5 | f34b3418ef26e76fa1a798e7c4cf832c77193114fe44805eebc305c26f1d7c9e | f1bcb6f4b849f5e37eac95a59972fb7b825dd1d7abd9fb67644055437f1d6fee | de44b39d722429f053e6eca8b4a227c08b8da65b2c7c322624b3abed29359f5d |
| taper_sp0.9_x147_t3.8 | bdba5e2bbe7e8680e22264d3332a92cc8d2ea14c3cb553b66da3187a0d67e284 | e07d22a6d8c8fcf72afcba98341f437cc389fe3f01f03369954493bb7d804d6a | 289c771dff89b89f347041e356eac821b0d51d277a1675ab2975cb04d9a5d7d6 |
| taper_sp0.9_x147_t4.2 | ea5c24c23b52452f84446b4afe873fe7b551934bf440442f72b9cd5a40c44ccf | ab2d72065eb748a6bc4cbc740c04a7cf54d555b54c7c5a97caae15d46c2f6d9f | 61ef72e12babbbd82eedf9144e547beb8aba288eb886a78f6d28dcfd2abf141e |
| taper_sp0.9_x147_t4.5 | d21daddc763d9c32c24900d5e682167f703acc356007f84456dafb2d71f58836 | bd86aae9659a2e0362b7d0b08563887ebb255bdd89eb8b1bc5864264497672aa | 32062b0d69a3b31f1fd04392500503f3f9a7554f675e66498c9c2182e4875309 |
| taper_sp0.9_x150_t3.8 | 24b4d5d15c3f5939b44f8fe72daae98ac1f5ac65d8f8512322d3ef2d9850be63 | 8d4fb1eaa078c847d10ab23a0945481ce5450115f6317c48097f0d63cea55009 | a28c61b3a814622da2bb791b12eba08db445a75ae8f67fd64569deff29728f0f |
| taper_sp0.9_x150_t4.2 | 5774648cd1553b13ca48194932a2a80564244b24eb57dc74fe43be79158a0af3 | 57b10a91bd6196fadca217c5698e57d05653ab5b1ebfd3ea189a9fae84c047e3 | d42555c38e5ad06eea6943cd17d28e999b96221b6962861b70beda7126fc1446 |
| taper_sp0.9_x150_t4.5 | d17b1619971fefdb376181596d9f8c40a4b97080efe36c3d86c3a0e3f34c3789 | bcf31571eda31adc4d9761a4e19f67c0de0814606884f863e3a476d4d6a48ab4 | b51f4aeedb4add309d1f9ff8b52f90eb250b98caed803b8dbcdf8805cb99b56e |

Raw evidence/scripts: `target/gate-feasibility-bound/unified_study_v10/`. Primary report: `experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V10_REPORT.md`; byte-identical mirror: `target/gate-feasibility-bound/UNIFIED_PROTOTYPE_V10_REPORT.md`. `SHA256SUMS` binds every stable artifact and both report copies. `REPORT_SHA256` and both trackers hold the final report digest, avoiding a self-referential hash. Full production/input/native dependency digests are in provenance catalogs; `verify_seal.py` independently verifies the manifest. All earlier reports/evidence are preserved.

```bash
nice -n 10 taskset -c 10,11 env CARGO_BUILD_JOBS=2 JARAMBIT_SOLVER=faer \
  bash target/gate-feasibility-bound/unified_study_v10/reproduce.sh
```

No production edit, git add/commit/push, PR, stacking, branch/worktree change or deletion. Both trackers are updated at milestones. Await captain_jarambit for the next assignment.
