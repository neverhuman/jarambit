# G16-12-RSV2-04 Phase-14: existing bevel-angle × legal spine sweep

Task `G16-12-RSV2-04-p14`; `unified_study_v11`; execution commit `8904e33d26ada252c8e357ccd7ed18a2330e908b`; branch `audit/gate-feasibility-bound`. Worktree `.`. All native invocations use CPUs **10,11**, `CARGO_BUILD_JOBS=2`, faer, requested `nice -n 10`, inherited effective niceness **19**. No commit or push. Frozen assignment: `inputs/assignment.md`.

## Result

Executed all **16 assigned probes**: four exact legal spine families (`delta_in=0`, `delta_sp=0/.3/.6/.9 mm`) × existing compound bevel angles **18 / 19.5 / 21 / 22°**. All 16 received **4 / 1 / .25 / .125 / .0625 mm²** native FEM and independent full native solid export, including all measured failures. **80 grid + 20 source-control = 100 FEM observations**; **16/16** native geometry/aesthetic passes, **4/16** complete exported-solid passes. Measured physical FEM models: **16**.

Joint structural/export passes: **0** on .25/.125, **0** on .25/.125/.0625, **0** on all five. Strict global HM twist maximum is **0.521397645** at `bevel_sp0.9_a22`; this profile’s full export status is **False**. A strict-mesh maximum is not an all-mesh qualification.

Nominal bevel angle <=22° (and <=25°) is an input constraint, not proof of Gate S compliance. Actual exported facets determine the maximum half-angle and apex; the complete native point profile and bore radius are separately measured. Every failure below is retained, without changing point law, d0/d1 or certification thresholds to obtain a pass.

Native Gate S component pass counts: S1 apex **12/16**, S2 point profile **16/16**, S3 facet half-angle **4/16**.

All 19.5°, 21° and 22° probes fail S3; all four 22° probes also fail S1. Native S2 point profile and bore radius pass for every probe. The strongest strict result improves by only 0.511361% over its matching 18° source and still fails global HM twist and export requirements.

## Exact inherited construction and permitted change

Starting CSVs are the four exact Phase-12 legal `outward_in0_sp{0,.3,.6,.9}` families, frozen through Phase-13 inputs. They represent the same four physical models verified in Phase-13; retained inactive native taper genes use the source settings `t_tip=2.8 mm`, `taper_x0=144 mm`. Within each family the entire genome is unchanged; only existing environment/configuration knob `STUDY_ANGLE` varies. Relative to the original Arm-B parent, only native CP36/37/38 carry the assigned outward offsets; CP1/2/3 stay unchanged. No new planform, taper law or bevel formula is introduced.

Fixed physical construction: stock/core/full neck 6 mm; P1 off; point shape 3, apex parameter .175 mm, point slope .3 and directional radial/lateral factor 2; d0=.48 mm and d1=.65 mm. The 0..2 mm point-cap formula is unchanged, and actual axis thickness samples at every .05 mm from 0 through 2 mm match each source exactly. Original pocket floor 2.2 mm, x=126..156, spine/edge guards 4.5/5.5 mm and 1.5 mm cubic smoothstep (C1); native window S0/S1=.50/.85, W=3, fill=.65.

The existing bevel-angle knob acts on the inherited full bevel field, including the distal annulus; it is not confined to that annulus by a new spatial blend. The point cap remains the original minimum envelope. All six scientific harness files are byte-identical to Phase-13. Native FEM source recovers exactly after removing the same three observation-only hooks. Native material, loads, masks, mesher, element averaging, section factors, limits and native export/certification implementation remain unchanged. Added wrapper diagnostics only sample actual thickness at common coordinates.

## Controls and observation binding

Four independent source controls reproduce all five areas: **440 numeric comparisons**, maximum relative error **1.42305983952e-13**, exact physical mesh/thickness hashes. The four independently evaluated 18° grid probes reproduce another **440** comparisons, maximum relative error **9.45424633504e-13**, with exact native solid/point/facet/mass observations.

For every angle, native planform T6 geometry hashes match its corresponding source at every area; changed geometry+element-thickness hashes therefore bind actual physical thickness changes rather than remeshing. Stress peaks can still move because the solved field changes. `annulus_thickness.csv` records common physical locations inside the native material with 4<=r_tip<15 mm: axial stations 4.5/6/8/10/12/14 mm along the original compound point axis, lateral offsets -3..3 mm in 1 mm steps, filtered by the actual outline/holes and radius. These finite samples are not a whole-annulus thickness census. Sample coordinates/thicknesses and planform chord widths repeat exactly across geometry/FEM/export modes.

## Physical annulus changes and requested twist metrics

| Probe | Fixed annulus samples | Thickened samples | Max Δthickness mm | Strict tip proxy SF | Strict global HM | Δstrict HM % vs18° | Fine-three minimum global HM | Fine-three joint |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bevel_sp0_a18 | 34 | 0 | 0.000000000 | 3.709271632 | 0.416720640 | 0.000000 | 0.416720640 | False |
| bevel_sp0_a19.5 | 34 | 2 | 0.028320821 | 3.714144956 | 0.417268137 | 0.131382 | 0.417268137 | False |
| bevel_sp0_a21 | 34 | 2 | 0.057171791 | 3.719076930 | 0.417822223 | 0.264346 | 0.417822223 | False |
| bevel_sp0_a22 | 34 | 2 | 0.076727673 | 3.722371312 | 0.418192333 | 0.353160 | 0.418192333 | False |
| bevel_sp0.3_a18 | 37 | 0 | 0.000000000 | 4.014168522 | 0.450974488 | 0.000000 | 0.404390387 | False |
| bevel_sp0.3_a19.5 | 37 | 5 | 0.017932107 | 4.018369035 | 0.451446398 | 0.104642 | 0.404787780 | False |
| bevel_sp0.3_a21 | 37 | 5 | 0.036199893 | 4.022896459 | 0.451955034 | 0.217428 | 0.405311865 | False |
| bevel_sp0.3_a22 | 37 | 5 | 0.048582238 | 4.025864125 | 0.452288439 | 0.291358 | 0.405684323 | False |
| bevel_sp0.6_a18 | 37 | 0 | 0.000000000 | 4.010460592 | 0.450557918 | 0.000000 | 0.433074064 | False |
| bevel_sp0.6_a19.5 | 37 | 4 | 0.018833356 | 4.010936217 | 0.450611353 | 0.011860 | 0.434679223 | False |
| bevel_sp0.6_a21 | 37 | 4 | 0.038019261 | 4.011452931 | 0.450669403 | 0.024744 | 0.436245574 | False |
| bevel_sp0.6_a22 | 37 | 4 | 0.051023930 | 4.012342068 | 0.450769294 | 0.046914 | 0.437142330 | False |
| bevel_sp0.9_a18 | 37 | 0 | 0.000000000 | 4.617400433 | 0.518744987 | 0.000000 | 0.431106525 | False |
| bevel_sp0.9_a19.5 | 37 | 4 | 0.022911865 | 4.626930249 | 0.519815621 | 0.206389 | 0.431466805 | False |
| bevel_sp0.9_a21 | 37 | 4 | 0.046252628 | 4.635838235 | 0.520816394 | 0.399311 | 0.431852250 | False |
| bevel_sp0.9_a22 | 37 | 4 | 0.062073559 | 4.641012002 | 0.521397645 | 0.511361 | 0.432110003 | False |

Native `twist_tipzone_sf` uses the original 1820 Nmm proxy in 4<=r_tip<15 mm; its strict value is at .0625 mm². Native global HM twist remains `twist_joint_nmm/16200`, equivalently `twist_sf*1820/16200`. Body uses the original r_tip>=15 mm selection; global excludes r_tip<4 mm. Body-only or proxy SF never replaces global HM. The diagnostic receipts retain original peak indices/locations, mean element thickness, stress and section correction factors.

## All native FEM observations

| Probe | Area mm² | Sequence | Spine SF | Lateral N | Fatigue SF | Ringback SF | Body HM | Global HM | Δglobal HM % vs18° | Peak thickness mm | Peak width mm |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bevel_sp0_a18 | 4.0 | 1.209514952 | 0.899795819 | 397.518752225 | 1.110399089 | 5.127088382 | 1.205617800 | 0.437186543 | 0.000000 | 5.701571679 | 7.950000000 |
| bevel_sp0_a18 | 1.0 | 1.215122538 | 0.725264676 | 398.596207537 | 1.113408772 | 5.055821842 | 1.185531763 | 0.416783980 | 0.000000 | 5.687454138 | 9.200000000 |
| bevel_sp0_a18 | 0.25 | 1.250619062 | 1.038357452 | 380.959221083 | 1.064142936 | 5.131770352 | 1.082833538 | 0.424203778 | 0.000000 | 5.673837610 | 8.850000000 |
| bevel_sp0_a18 | 0.125 | 1.258612740 | 1.063142596 | 381.130439334 | 1.064621204 | 4.992111565 | 1.021661607 | 0.435482731 | 0.000000 | 5.670980159 | 8.900000000 |
| bevel_sp0_a18 | 0.0625 | 1.422207709 | 1.007135462 | 337.700838841 | 0.943308213 | 5.064628196 | 1.310617549 | 0.416720640 | 0.000000 | 5.667106156 | 8.750000000 |
| bevel_sp0_a19.5 | 4.0 | 1.207404340 | 0.907619433 | 398.213643006 | 1.112340145 | 5.127088382 | 1.210594293 | 0.437240746 | 0.012398 | 5.701571679 | 7.950000000 |
| bevel_sp0_a19.5 | 1.0 | 1.212233087 | 0.733204049 | 399.546301503 | 1.116062693 | 5.055821842 | 1.189127620 | 0.417373267 | 0.141389 | 5.687454138 | 9.200000000 |
| bevel_sp0_a19.5 | 0.25 | 1.250767216 | 1.048587049 | 380.914116757 | 1.064016945 | 5.131770352 | 1.082942178 | 0.424683445 | 0.113075 | 5.673837610 | 8.850000000 |
| bevel_sp0_a19.5 | 0.125 | 1.257988982 | 1.062428249 | 381.319440327 | 1.065149145 | 4.992111565 | 1.021823300 | 0.436002087 | 0.119260 | 5.670980159 | 8.900000000 |
| bevel_sp0_a19.5 | 0.0625 | 1.420740702 | 1.006755298 | 338.049993206 | 0.944283514 | 5.064628196 | 1.310541334 | 0.417268137 | 0.131382 | 5.667106156 | 8.750000000 |
| bevel_sp0_a21 | 4.0 | 1.205224314 | 0.915434042 | 398.933939823 | 1.114352168 | 5.127088382 | 1.215724878 | 0.437295281 | 0.024872 | 5.701571679 | 7.950000000 |
| bevel_sp0_a21 | 1.0 | 1.209266193 | 0.741209411 | 400.526583427 | 1.118800939 | 5.055821842 | 1.192855824 | 0.417931928 | 0.275430 | 5.687454138 | 9.200000000 |
| bevel_sp0_a21 | 0.25 | 1.250914204 | 1.058853490 | 380.869347533 | 1.063891890 | 5.131770352 | 1.083053990 | 0.425173369 | 0.228567 | 5.673837610 | 8.850000000 |
| bevel_sp0_a21 | 0.125 | 1.257332251 | 1.061858016 | 381.518565484 | 1.065705366 | 4.992111565 | 1.021989524 | 0.436508032 | 0.235440 | 5.670980159 | 8.900000000 |
| bevel_sp0_a21 | 0.0625 | 1.419223993 | 1.006521232 | 338.411667061 | 0.945293787 | 5.064628196 | 1.310569110 | 0.417822223 | 0.264346 | 5.667106156 | 8.750000000 |
| bevel_sp0_a22 | 4.0 | 1.203730303 | 0.920644959 | 399.429075354 | 1.115735242 | 5.127088382 | 1.219236820 | 0.437331921 | 0.033253 | 5.701571679 | 7.950000000 |
| bevel_sp0_a22 | 1.0 | 1.207242622 | 0.746588830 | 401.197946763 | 1.120676275 | 5.055821842 | 1.195419226 | 0.418308603 | 0.365806 | 5.687454138 | 9.200000000 |
| bevel_sp0_a22 | 0.25 | 1.251011606 | 1.065726800 | 380.839695181 | 1.063809061 | 5.131770352 | 1.083130517 | 0.425489754 | 0.303151 | 5.673837610 | 8.850000000 |
| bevel_sp0_a22 | 0.125 | 1.256875033 | 1.061551928 | 381.657376342 | 1.066093110 | 4.992111565 | 1.022103051 | 0.436852769 | 0.314602 | 5.670980159 | 8.900000000 |
| bevel_sp0_a22 | 0.0625 | 1.418183304 | 1.006440134 | 338.660259831 | 0.945988188 | 5.064628196 | 1.310648119 | 0.418192333 | 0.353160 | 5.667106156 | 8.750000000 |
| bevel_sp0.3_a18 | 4.0 | 1.258506224 | 0.963204339 | 376.164235583 | 1.050748982 | 5.202661834 | 1.370809910 | 0.504700484 | 0.000000 | 5.667079418 | 8.050000000 |
| bevel_sp0.3_a18 | 1.0 | 1.143522880 | 0.889032764 | 379.261909918 | 1.059401794 | 5.128575149 | 1.401541746 | 0.426819695 | 0.000000 | 5.675300479 | 9.400000000 |
| bevel_sp0.3_a18 | 0.25 | 1.148828361 | 1.040153812 | 387.196177556 | 1.081564782 | 5.231792318 | 1.087742839 | 0.404390387 | 0.000000 | 5.665717869 | 9.150000000 |
| bevel_sp0.3_a18 | 0.125 | 1.110779141 | 1.007651964 | 396.935755704 | 1.108770590 | 5.001267803 | 1.316004715 | 0.435760742 | 0.000000 | 5.666921474 | 9.150000000 |
| bevel_sp0.3_a18 | 0.0625 | 1.279786041 | 0.970383886 | 372.128958319 | 1.039477141 | 4.944562955 | 1.099065741 | 0.450974488 | 0.000000 | 5.689069747 | 9.800000000 |
| bevel_sp0.3_a19.5 | 4.0 | 1.252441966 | 0.972899578 | 377.985595466 | 1.055836633 | 5.202661834 | 1.374187932 | 0.504817385 | 0.023163 | 5.667819064 | 8.050000000 |
| bevel_sp0.3_a19.5 | 1.0 | 1.144207239 | 0.898845089 | 379.035070865 | 1.058768159 | 5.128575149 | 1.402973547 | 0.427226824 | 0.095387 | 5.675300479 | 9.400000000 |
| bevel_sp0.3_a19.5 | 0.25 | 1.144478503 | 1.036538478 | 388.667789532 | 1.085675473 | 5.231792318 | 1.090382287 | 0.404787780 | 0.098270 | 5.665717869 | 9.150000000 |
| bevel_sp0.3_a19.5 | 0.125 | 1.109189789 | 1.015710353 | 397.504528070 | 1.110359356 | 5.001267803 | 1.318130366 | 0.436460431 | 0.160567 | 5.666921474 | 9.150000000 |
| bevel_sp0.3_a19.5 | 0.0625 | 1.277320710 | 0.967300049 | 372.847215619 | 1.041483468 | 4.944562955 | 1.099182044 | 0.451446398 | 0.104642 | 5.689069747 | 9.800000000 |
| bevel_sp0.3_a21 | 4.0 | 1.246278041 | 0.982680878 | 379.855069544 | 1.061058681 | 5.202661834 | 1.377691295 | 0.504935837 | 0.046632 | 5.668572556 | 8.050000000 |
| bevel_sp0.3_a21 | 1.0 | 1.144867027 | 0.908613252 | 378.816641356 | 1.058158014 | 5.128575149 | 1.404493198 | 0.427629557 | 0.189743 | 5.675300479 | 9.400000000 |
| bevel_sp0.3_a21 | 0.25 | 1.140046567 | 1.033361407 | 390.178742633 | 1.089896056 | 5.231792318 | 1.093103306 | 0.405311865 | 0.227868 | 5.665717869 | 9.150000000 |
| bevel_sp0.3_a21 | 0.125 | 1.107559840 | 1.023864763 | 398.089454593 | 1.111993246 | 5.001267803 | 1.320317533 | 0.437128636 | 0.313909 | 5.666921474 | 9.150000000 |
| bevel_sp0.3_a21 | 0.0625 | 1.274801864 | 0.964415658 | 373.583903591 | 1.043541276 | 4.944562955 | 1.099301542 | 0.451955034 | 0.217428 | 5.689069747 | 9.800000000 |
| bevel_sp0.3_a22 | 4.0 | 1.242108773 | 0.989256928 | 381.130097797 | 1.064620250 | 5.202661834 | 1.380100683 | 0.505015835 | 0.062483 | 5.669083291 | 8.050000000 |
| bevel_sp0.3_a22 | 1.0 | 1.145293115 | 0.915108334 | 378.675710685 | 1.057764349 | 5.128575149 | 1.405557669 | 0.427901554 | 0.253470 | 5.675300479 | 9.400000000 |
| bevel_sp0.3_a22 | 0.25 | 1.137042824 | 1.031464431 | 391.209475530 | 1.092775228 | 5.231792318 | 1.094965975 | 0.405684323 | 0.319972 | 5.665717869 | 9.150000000 |
| bevel_sp0.3_a22 | 0.125 | 1.106449103 | 1.029361164 | 398.489111802 | 1.113109618 | 5.001267803 | 1.321812508 | 0.437564785 | 0.413999 | 5.666921474 | 9.150000000 |
| bevel_sp0.3_a22 | 0.0625 | 1.273090346 | 0.962594253 | 374.086040880 | 1.044943909 | 4.944562955 | 1.099383315 | 0.452288439 | 0.291358 | 5.689069747 | 9.800000000 |
| bevel_sp0.6_a18 | 4.0 | 1.052025004 | 0.962212957 | 385.567116287 | 1.077014285 | 5.396308000 | 1.142258008 | 0.574982564 | 0.000000 | 5.717820975 | 9.900000000 |
| bevel_sp0.6_a18 | 1.0 | 1.252978311 | 1.093370672 | 354.614071029 | 0.990552368 | 5.208015933 | 1.585166827 | 0.333786973 | 0.000000 | 5.681876830 | 8.850000000 |
| bevel_sp0.6_a18 | 0.25 | 1.144896190 | 1.061876619 | 403.241481298 | 1.126384531 | 5.119626292 | 1.375656489 | 0.433074064 | 0.000000 | 5.666467315 | 9.550000000 |
| bevel_sp0.6_a18 | 0.125 | 1.188302691 | 1.055551282 | 388.853276191 | 1.086193597 | 5.011272996 | 1.438717861 | 0.440014729 | 0.000000 | 5.376946096 | 10.400000000 |
| bevel_sp0.6_a18 | 0.0625 | 1.194944019 | 1.057300697 | 388.743647245 | 1.085887368 | 4.924846522 | 1.366495697 | 0.450557918 | 0.000000 | 5.215243709 | 10.400000000 |
| bevel_sp0.6_a19.5 | 4.0 | 1.051299629 | 0.967794363 | 385.833152804 | 1.077757411 | 5.396308000 | 1.146661395 | 0.575309816 | 0.056915 | 5.717820975 | 9.900000000 |
| bevel_sp0.6_a19.5 | 1.0 | 1.251570385 | 1.098347295 | 355.012995002 | 0.991666692 | 5.208015933 | 1.589080750 | 0.334504886 | 0.215081 | 5.681876830 | 8.850000000 |
| bevel_sp0.6_a19.5 | 0.25 | 1.143101885 | 1.072946562 | 403.874427672 | 1.128152556 | 5.119626292 | 1.376432674 | 0.434679223 | 0.370643 | 5.666467315 | 9.550000000 |
| bevel_sp0.6_a19.5 | 0.125 | 1.186774572 | 1.075017355 | 389.354015899 | 1.087592326 | 5.011272996 | 1.125655739 | 0.440511465 | 0.112891 | 5.379466966 | 10.400000000 |
| bevel_sp0.6_a19.5 | 0.0625 | 1.192491868 | 1.077814516 | 389.542956581 | 1.088120099 | 4.924846522 | 1.368239718 | 0.450611353 | 0.011860 | 5.218688640 | 10.400000000 |
| bevel_sp0.6_a21 | 4.0 | 1.050559289 | 0.973357852 | 386.105059613 | 1.078516935 | 5.396308000 | 1.151211619 | 0.575651036 | 0.116259 | 5.717820975 | 9.900000000 |
| bevel_sp0.6_a21 | 1.0 | 1.250129187 | 1.103391926 | 355.422270473 | 0.992809932 | 5.208015933 | 1.593132239 | 0.335243626 | 0.436402 | 5.681876830 | 8.850000000 |
| bevel_sp0.6_a21 | 0.25 | 1.141271460 | 1.084028902 | 404.522197009 | 1.129961987 | 5.119626292 | 1.377301147 | 0.436245574 | 0.732325 | 5.666467315 | 9.550000000 |
| bevel_sp0.6_a21 | 0.125 | 1.185194721 | 1.083823631 | 389.873013141 | 1.089042054 | 5.011272996 | 1.125800768 | 0.440980355 | 0.219453 | 5.382035025 | 10.400000000 |
| bevel_sp0.6_a21 | 0.0625 | 1.189995854 | 1.088179136 | 390.360009723 | 1.090402394 | 4.924846522 | 1.370046767 | 0.450669403 | 0.024744 | 5.222198058 | 10.400000000 |
| bevel_sp0.6_a22 | 4.0 | 1.050056672 | 0.977062202 | 386.289871489 | 1.079033175 | 5.396308000 | 1.154331978 | 0.575886829 | 0.157268 | 5.717820975 | 9.900000000 |
| bevel_sp0.6_a22 | 1.0 | 1.249148370 | 1.106795185 | 355.701344054 | 0.993589475 | 5.208015933 | 1.595914657 | 0.335748627 | 0.587696 | 5.681876830 | 8.850000000 |
| bevel_sp0.6_a22 | 0.25 | 1.140029281 | 1.084938078 | 404.962955626 | 1.131193169 | 5.119626292 | 1.377933169 | 0.437142330 | 0.939393 | 5.666467315 | 9.550000000 |
| bevel_sp0.6_a22 | 0.125 | 1.184110733 | 1.084672158 | 390.229933147 | 1.090039048 | 5.011272996 | 1.125899974 | 0.441255089 | 0.281891 | 5.383775718 | 10.400000000 |
| bevel_sp0.6_a22 | 0.0625 | 1.188304740 | 1.088197352 | 390.915373910 | 1.091953707 | 4.924846522 | 1.371289124 | 0.450769294 | 0.046914 | 5.224576826 | 10.400000000 |
| bevel_sp0.9_a18 | 4.0 | 1.065379132 | 0.723929481 | 367.162046492 | 1.025602942 | 5.204306404 | 1.258750271 | 0.546709897 | 0.000000 | 5.703265984 | 9.650000000 |
| bevel_sp0.9_a18 | 1.0 | 1.156204061 | 0.958509182 | 380.524706649 | 1.062929196 | 5.090785760 | 1.249344366 | 0.468435953 | 0.000000 | 5.694510881 | 10.600000000 |
| bevel_sp0.9_a18 | 0.25 | 1.070193645 | 1.146651756 | 433.395372684 | 1.210614151 | 5.072760691 | 1.377772455 | 0.468287461 | 0.000000 | 5.666749666 | 10.100000000 |
| bevel_sp0.9_a18 | 0.125 | 1.217761572 | 1.014909975 | 360.778229190 | 1.007770865 | 5.039643999 | 1.453942694 | 0.431106525 | 0.000000 | 5.604681894 | 10.800000000 |
| bevel_sp0.9_a18 | 0.0625 | 1.302321675 | 0.995021961 | 363.067022522 | 1.014164209 | 5.001950526 | 1.441576150 | 0.518744987 | 0.000000 | 5.674065819 | 10.350000000 |
| bevel_sp0.9_a19.5 | 4.0 | 1.063733929 | 0.731186136 | 367.729918301 | 1.027189193 | 5.204306404 | 1.264617296 | 0.546858305 | 0.027146 | 5.703265984 | 9.650000000 |
| bevel_sp0.9_a19.5 | 1.0 | 1.154993360 | 0.966061650 | 380.923576166 | 1.064043368 | 5.090785760 | 1.255358746 | 0.468803676 | 0.078500 | 5.694510881 | 10.600000000 |
| bevel_sp0.9_a19.5 | 0.25 | 1.070127180 | 1.144686177 | 433.422337167 | 1.210689472 | 5.072760691 | 1.377564818 | 0.469226502 | 0.200527 | 5.666749666 | 10.100000000 |
| bevel_sp0.9_a19.5 | 0.125 | 1.215289506 | 1.012882817 | 361.512162792 | 1.009820981 | 5.039643999 | 1.451750861 | 0.431466805 | 0.083571 | 5.605817061 | 10.800000000 |
| bevel_sp0.9_a19.5 | 0.0625 | 1.300007808 | 0.993067706 | 363.713314699 | 1.015969513 | 5.001950526 | 1.442480010 | 0.519815621 | 0.206389 | 5.674065819 | 10.350000000 |
| bevel_sp0.9_a21 | 4.0 | 1.062052139 | 0.738495665 | 368.312240569 | 1.028815808 | 5.204306404 | 1.270665401 | 0.547015501 | 0.055899 | 5.703265984 | 9.650000000 |
| bevel_sp0.9_a21 | 1.0 | 1.153755190 | 0.973601493 | 381.332367244 | 1.065185254 | 5.090785760 | 1.261561514 | 0.469183265 | 0.159534 | 5.694510881 | 10.600000000 |
| bevel_sp0.9_a21 | 0.25 | 1.070058911 | 1.142919171 | 433.449975839 | 1.210766676 | 5.072760691 | 1.377369363 | 0.469893567 | 0.342974 | 5.666749666 | 10.100000000 |
| bevel_sp0.9_a21 | 0.125 | 1.212748381 | 1.011037442 | 362.269544625 | 1.011936594 | 5.039643999 | 1.449576872 | 0.431852250 | 0.172979 | 5.606973479 | 10.800000000 |
| bevel_sp0.9_a21 | 0.0625 | 1.297628828 | 0.991331410 | 364.380028337 | 1.017831861 | 5.001950526 | 1.443491215 | 0.520816394 | 0.399311 | 5.674065819 | 10.350000000 |
| bevel_sp0.9_a22 | 4.0 | 1.060908864 | 0.743403487 | 368.709150027 | 1.029924505 | 5.204306404 | 1.274804813 | 0.547125604 | 0.076038 | 5.703265984 | 9.650000000 |
| bevel_sp0.9_a22 | 1.0 | 1.152913221 | 0.978626976 | 381.610865830 | 1.065963191 | 5.090785760 | 1.265808435 | 0.469443413 | 0.215069 | 5.694510881 | 10.600000000 |
| bevel_sp0.9_a22 | 0.25 | 1.070012309 | 1.141844652 | 433.468818814 | 1.210819310 | 5.072760691 | 1.377246162 | 0.470279772 | 0.425446 | 5.666749666 | 10.100000000 |
| bevel_sp0.9_a22 | 0.125 | 1.211013442 | 1.009901176 | 362.788629582 | 1.013386567 | 5.039643999 | 1.448137692 | 0.432110003 | 0.232768 | 5.607757326 | 10.800000000 |
| bevel_sp0.9_a22 | 0.0625 | 1.296003968 | 0.990286358 | 364.836851677 | 1.019107917 | 5.001950526 | 1.444228541 | 0.521397645 | 0.511361 | 5.674065819 | 10.350000000 |

Joint comparison limits retain sequence>=1.20, spine>=1, lateral>=260 N, fatigue>=1, ringback>=1, body HM>=1 and global HM>=1, plus unchanged native geometry/aesthetic validity and a passing matched full native export. Native fatigue SF=1 corresponds to about 357.9963 N; 260 N alone does not establish fatigue pass. Fine-pair/fine-three/all-five minima are retained separately; no favorable mesh or metrics from different probes are pooled.

## Complete native exported-solid verification

Each profile independently runs native `certification::measure`: S1 apex <=.2 mm, S2 native point-profile pass, S3 facet half-angle <=25°, bore edge radius >=1.5 mm, census [0,0,1] and positive volume. Export grid=.05 mm, area=.25 mm². The native measurement’s embedded source label is preserved verbatim, with actual configuration recorded separately in each receipt.

| Probe | Half-angle ° | Apex mm | S1 apex | S2 point profile | S3 half-angle | Bore mm | Census | Volume mm³ | Full export | Failed checks |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bevel_sp0_a18 | 24.324224296 | 0.195144636 | True | True | True | 1.600000000 | [0, 0, 1] | 30764.031033614 | True | none |
| bevel_sp0_a19.5 | 26.227056719 | 0.196954932 | True | True | False | 1.600000000 | [0, 0, 1] | 30765.839019326 | False | sharpness |
| bevel_sp0_a21 | 28.103528631 | 0.198799116 | True | True | False | 1.600000000 | [0, 0, 1] | 30767.680020252 | False | sharpness |
| bevel_sp0_a22 | 29.339424020 | 0.200049148 | False | True | False | 1.600000000 | [0, 0, 1] | 30768.927277934 | False | sharpness |
| bevel_sp0.3_a18 | 24.211856857 | 0.195140515 | True | True | True | 1.600000000 | [0, 0, 1] | 30814.819726742 | True | none |
| bevel_sp0.3_a19.5 | 26.108357163 | 0.196950440 | True | True | False | 1.600000000 | [0, 0, 1] | 30816.630836155 | False | sharpness |
| bevel_sp0.3_a21 | 27.979094770 | 0.198794246 | True | True | False | 1.600000000 | [0, 0, 1] | 30818.474792939 | False | sharpness |
| bevel_sp0.3_a22 | 29.211502176 | 0.200044023 | False | True | False | 1.600000000 | [0, 0, 1] | 30819.724404896 | False | sharpness |
| bevel_sp0.6_a18 | 24.106336838 | 0.195141727 | True | True | True | 1.600000000 | [0, 0, 1] | 30864.114706358 | True | none |
| bevel_sp0.6_a19.5 | 25.996862022 | 0.196951761 | True | True | False | 1.600000000 | [0, 0, 1] | 30865.929877246 | False | sharpness |
| bevel_sp0.6_a21 | 27.862181940 | 0.198795678 | True | True | False | 1.600000000 | [0, 0, 1] | 30867.778054243 | False | sharpness |
| bevel_sp0.6_a22 | 29.091290090 | 0.200045530 | False | True | False | 1.600000000 | [0, 0, 1] | 30869.030385742 | False | sharpness |
| bevel_sp0.9_a18 | 23.800291946 | 0.195143062 | True | True | True | 1.600000000 | [0, 0, 1] | 30920.866531777 | True | none |
| bevel_sp0.9_a19.5 | 25.673329754 | 0.196953216 | True | True | False | 1.600000000 | [0, 0, 1] | 30922.685738180 | False | sharpness |
| bevel_sp0.9_a21 | 27.522757221 | 0.198797256 | True | True | False | 1.600000000 | [0, 0, 1] | 30924.538183252 | False | sharpness |
| bevel_sp0.9_a22 | 28.742166090 | 0.200047190 | False | True | False | 1.600000000 | [0, 0, 1] | 30925.793326142 | False | sharpness |

Passing exports: `bevel_sp0_a18`, `bevel_sp0.3_a18`, `bevel_sp0.6_a18`, `bevel_sp0.9_a18`.

Failing exports: `bevel_sp0_a19.5`, `bevel_sp0_a21`, `bevel_sp0_a22`, `bevel_sp0.3_a19.5`, `bevel_sp0.3_a21`, `bevel_sp0.3_a22`, `bevel_sp0.6_a19.5`, `bevel_sp0.6_a21`, `bevel_sp0.6_a22`, `bevel_sp0.9_a19.5`, `bevel_sp0.9_a21`, `bevel_sp0.9_a22`.

## Evidence, SHA-256 digests and reproduction

Phase-13 now has supplemental `unified_study_v10/receipt_manifest.json`, enumerating and verifying the original 721 sealed artifacts and report digest `9183bbecbf3898c16f37a647866b922cce5070db7775224a99182ac42127fb15`. Its original `SHA256SUMS` is preserved unchanged; the new supplemental receipt is frozen and sealed as a Phase-14 input.

Verification passes **80 production digests**, **39 frozen inputs**, complete 16-probe Cartesian coverage, unchanged genomes per source family, only assigned angle changes, fixed point/d0/d1, all native five-area solves and full exports including failures, original masks/peaks/torque identities, protected bands/full-stock neck, exact source planform hashes, physical thickness hashes and fixed-coordinate observations. All **52** invocations bind one executable and CPUs10,11/jobs2/faer/effective nice19. Workflow syntax checks pass. Eight native dependency hashes and all ten prior seals are verified at final sealing.

| Artifact | Full SHA-256 |
| --- | --- |
| study | bb6e93c3f978969b7d799a4a3b1e407c2d30c95d2a626933ed32b3b545a62a3d |
| source_provenance.json | 0265265760e93278dc8f2c403936b9f1b046fa9907e85de4f9f96326991cc06f |
| verification.json | 6b1d5244707dc0eae6e6685ca152068e095a62e84038870a7737d40b23024677 |
| control_verification.json | 971dda42f458a2b112e141e390c493be83ab86f37d0112a5b877328360702d61 |
| source_comparisons.json | 3ef41c48211b546a059f8645b9ea78339adb219ce29e315a92958d65a8b68ca5 |
| annulus_thickness.csv | 4c7a583aabcda097840064ca9a6529149c23f0419f7902eb3beb96ee060af9c9 |
| fem_summary.csv | 256c3ebeac9e96da942641d7a9f4353db3720b995293e6b14b8b598f6b4f21a0 |
| summary.json | 653fcc05b1fbd37bcd0b89a5f6c965d918c9ac37da10b7fe1c3711fb8593b6a1 |
| inputs/assignment.md | 57dfe030a69228cf1bcd4c6a5688d876d56b58c7fa7a5d954c3f159074c3a5db |
| inputs/receipt_manifest.json | 8b65216e981a037f848b631b13633c4707ec4faf156950d86da57177dd1d2182 |
| inputs/SHA256SUMS | af83d7c4f14d5bd1364fe74f2a5004e44131e0399938884817bdb53fd2e16ed9 |

| Probe | Input CSV SHA-256 | FEM receipt SHA-256 | Export receipt SHA-256 |
| --- | --- | --- | --- |
| bevel_sp0_a18 | f49db5dc08437d235abe6c1a62f3a79df4b50e4d711161d4a7bd59a7ccc56280 | 68db8f34480cdab28b9ff92d10b5cb06b45aa455010f77fff140da43694b525d | a8ca7697489051a47401fd2abfcd9cbf94c71787ba482f0d991630ad9c87b157 |
| bevel_sp0_a19.5 | f49db5dc08437d235abe6c1a62f3a79df4b50e4d711161d4a7bd59a7ccc56280 | f41810786b6807305734d2b1bfa3034b5ac7aeebd71abbe8764b0b9abcde5e62 | f296bb8f6bfcdb6365c8d0b7c3a440e884aaa999257639b2be17191720eb6b6d |
| bevel_sp0_a21 | f49db5dc08437d235abe6c1a62f3a79df4b50e4d711161d4a7bd59a7ccc56280 | 63ca18ebabc6f4748a2a5168add3c6972c15152943661679bb80b96944f6ef32 | 98b02da9a3a3d057d54a45ad3090fb96b4f025ff0aa1e141d00d6e1798441f18 |
| bevel_sp0_a22 | f49db5dc08437d235abe6c1a62f3a79df4b50e4d711161d4a7bd59a7ccc56280 | 4cbfcfa1342cb6af58c020a195fb18926754ccaff6fe0df11e96e49bf92369fe | 2f9ef2164b3f296cb2ec907070c85ea7d9cc01caf41bdf1aed89ab66d05a1c46 |
| bevel_sp0.3_a18 | 662f3d5d51bbfeb200eff6d443c62776032baffb17aa521869ff08ecf994d8ed | b231aad7d1f02bb29cf10ad146fb1b388d496ac1cf200b42b23ddd78fe0fda0d | 08334dbc0f2b3b939768e2f8993284e513d33fed20f9fa33c341d63d76db118c |
| bevel_sp0.3_a19.5 | 662f3d5d51bbfeb200eff6d443c62776032baffb17aa521869ff08ecf994d8ed | b404e354ab7c97bfdd87ce26389de0ba6a822905758759cc39daf479512269ad | d4807ff51c8ba8154d8c90068e9126c41536bc6478d625d46243b25afd7b4fe2 |
| bevel_sp0.3_a21 | 662f3d5d51bbfeb200eff6d443c62776032baffb17aa521869ff08ecf994d8ed | f234ce58880cc5ccd2b18ef1f2d0f2cf68fd5d8c095181ebd57f5ab1f7688e46 | 1563460e738ad26ac3c44bba395ddd1cbeedc35d297e36d793db274bb34899ea |
| bevel_sp0.3_a22 | 662f3d5d51bbfeb200eff6d443c62776032baffb17aa521869ff08ecf994d8ed | baf4bd31cd1697d2616746fe513afb1618e017d27fead590cfcc0d92a5ba76c9 | d87e65e1a194725d81c4f6b681d636d43b61f75cf82ea705de986c050c2f46a7 |
| bevel_sp0.6_a18 | 9ec95b50c57f31eb3d402e44ed07f83d484007bb0d36cb74ee184c2fdf479a2e | f8b00d0ce793245e4b51b730ff389c55734e17909d709a8b110e97ad14ef75b3 | aeed7e0c1f1e718d6025f10e69e3462a16eb555a212e3422771692d439f40021 |
| bevel_sp0.6_a19.5 | 9ec95b50c57f31eb3d402e44ed07f83d484007bb0d36cb74ee184c2fdf479a2e | 447255672aa0fb529757c9cd9cd1b6b0aff55bb821e1a4fd26991c381e51661f | ab847702d761a4ef2550edb4d2279c077e580e690ab8bcc94b0657dc97360aff |
| bevel_sp0.6_a21 | 9ec95b50c57f31eb3d402e44ed07f83d484007bb0d36cb74ee184c2fdf479a2e | 4a2f13ed9a83188ca5c12f1dd7ec8dc0af9490caedb8ef44a22fd6b57f2a1708 | 6cfd9a13e16671d247c0d6642dfe10c78e3010261394d3cd7053bb965e864446 |
| bevel_sp0.6_a22 | 9ec95b50c57f31eb3d402e44ed07f83d484007bb0d36cb74ee184c2fdf479a2e | aff30fc83ac142a8ba6d4a22402915dd85be95910248650283007ab27ffb2aa4 | ba084f15f7c742a18e7a4a51b8b91449ef3c769a4712543141dba21c8d18ebe9 |
| bevel_sp0.9_a18 | 2c7f15793cc0716e0b7e97683209ea0c0077100ef18dc3d8bce89d6da9d9f89e | d46cbd5d111df482e6a9e3657c7dddd97fdfe1c383840759dcee2929d729b0d0 | 7faf0afb0842ee1a72b2034d798421a855abc1be3dab155fdae463261bced510 |
| bevel_sp0.9_a19.5 | 2c7f15793cc0716e0b7e97683209ea0c0077100ef18dc3d8bce89d6da9d9f89e | 2855d9655a43aed64bf85d5b525462bceb10a12baa4cc05fb8a76f81749b0c72 | 2549aaf39c21a665342af8e9aa8b6973e44b7cbeea7f640d57c206787b240a98 |
| bevel_sp0.9_a21 | 2c7f15793cc0716e0b7e97683209ea0c0077100ef18dc3d8bce89d6da9d9f89e | 0545728cffba15d0ff9c26c82e8a78b4209783099061a8e07b8d787f3a5881fa | 745922c63e7dccbdfb5f143e455f4d63bf82c0be3fcc34b81118066585dc58c3 |
| bevel_sp0.9_a22 | 2c7f15793cc0716e0b7e97683209ea0c0077100ef18dc3d8bce89d6da9d9f89e | fa05232cb2b8b07f69263b5b948561e0bc8856ea87e94b235454c2ad50c6c5a6 | 9a09fd853605037f9c3ede003bc625dea6c9e56652f7fc3a25b0f48ca854d538 |

Primary report: `experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V11_REPORT.md`; byte-identical target mirror: `target/gate-feasibility-bound/UNIFIED_PROTOTYPE_V11_REPORT.md`; raw evidence/scripts: `target/gate-feasibility-bound/unified_study_v11/`. Both `SHA256SUMS` and `receipt_manifest.json` are produced. The receipt catalogs stable artifacts except itself (whose digest is covered by SHA256SUMS) and states the final sealed artifact count; this avoids a circular self hash. `REPORT_SHA256`, provenance and both trackers hold the report digest. The independent verifier checks both the seal and receipt catalog.

```bash
nice -n 10 taskset -c 10,11 env CARGO_BUILD_JOBS=2 JARAMBIT_SOLVER=faer \
  bash target/gate-feasibility-bound/unified_study_v11/reproduce.sh
```

This completes the assigned finite experiment; it is not a global infeasibility, convergence, or all-hard certificate. No production edit, git add/commit/push, PR, stacking, branch/worktree change or deletion. Both trackers are updated at milestones; await captain_jarambit for further work.
