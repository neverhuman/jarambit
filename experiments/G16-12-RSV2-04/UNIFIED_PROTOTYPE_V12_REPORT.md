# G16-12-RSV2-04 Phase-15: Q7 fine-angle and asymmetric spine boundary study

Task `G16-12-RSV2-04-p15`; `unified_study_v12`; execution commit `33870b5a92336193e3f72d09e9ce5cc77be3d440`; branch `audit/gate-feasibility-bound`; worktree `.` (repository root). CPUs **10,11**, `CARGO_BUILD_JOBS=2`, faer. Requested nice **10**; actual inherited nice **19**. Lowering to 10 with `os.setpriority` was denied (`PermissionError: EACCES`); no privilege bypass. All receipts record actual affinity and priority. No commit/push.

## Result

Executed all **12 assigned probes**: `dn37=.30/.45/.60 mm`, `dn36=.65*dn37`, `dn38=.80*dn37` × `EA_IN=EA_SP=18/18.3/18.6/18.9°`. Every probe has all **4 / 1 / .25 / .125 / .0625 mm²** native FEM meshes and independent full native solid export. **60 grid + 10 controls = 70 FEM observations**. Native geometry/aesthetic passes **12/12**; native full export passes **0/12**. Actual physical models **12**.

Native Gate S passes: S1 measured apex **0/12**, S2 point profile **12/12**, S3 measured half-angle<=25° **10/12**. The assignment’s separate half-angle<=26° comparison passes **12/12**; even using that comparison, complete export checks pass **0/12**. Native thresholds were not changed.

Joint structural/native-export passes remain **0** on .25/.125, **0** on .25/.125/.0625, **0** on all five. No exact continuous `EA*` is established by the four-angle grid; measured endpoint statuses and adjacent transition brackets are reported below.

All twelve measured half-angles satisfy the requested 26° comparison. No pass-to-fail transition is observed on the assigned [18.0,18.9] grid, so the requested exact EA* cannot be located there from these results. Native25° statuses and any sampled brackets are separate below.

Every exported solid fails measured S1 apex despite the nominal EDGE_TH=.192 input: the mapped physical apex parameter produces a larger measured facet-derived apex. This blocks complete export qualification even where angle, point and bore checks pass.

## Assignment/source distinctions

Native S3 is **25°**, defined by `crates/crescent/src/fem.rs::gate_s::HALF_ANGLE_MAX_DEG` and `requirements/RS-karambit-v2.toml::half_angle_max_deg`. The assignment calls 26° “S3”; this report preserves native 25° certification and separately labels 26° as the requested comparison. Phase-14 actually tested 18/19.5/21/22°, not 20°. Nominal EDGE_TH<=.20 is an input condition; measured exported apex thickness determines S1. No nominal value substitutes for native measurement.

Exact original Q7 CSV SHA-256: `0871168936d25b66a8bcd849193f63fd09f00185229f1cce8df71b0f43cd7e7a`, identical to archived v4/v6/v7 Q7. Original Q7 uses **no pocket**. This is not the later Arm-B parent’s pocketed field. Starting from that exact Q7, only these nine genes change: CP36,CP37,CP38,RING_WALL,T_TIP,TAPER_X0,EA_IN,EA_SP,EDGE_TH. Fixed assigned values are ring_wall=6.12 mm, t_tip=4.4 mm, taper_x0=148 mm, edge_t=.192 mm. CP increments are positive native outward-normal offsets with the exact requested .65/1/.80 factors; native canonicality confirms no clipping.

The retained compound construction overrides EA and EDGE_TH genes in its blade bevel law. To implement the requested physical study, the assigned genes are also mapped to the existing `STUDY_ANGLE=EA` and `STUDY_APEX=.192` knobs. This is an explicit interpretation of the assigned EA/EDGE_TH values as physical settings, using the existing knobs alongside their genome values. No new formula or spatial blend is introduced. Both gene and physical configuration are bound in every case/invocation/receipt. As in Phase-13, native t_tip/taper genes remain overridden by the retained full-core/P1-off blade law; they are nevertheless set exactly as assigned.

All other original Q7 configuration remains fixed: 6 mm stock/core/full neck, P1 off; d0=.48/d1=.65 mm, point shape3, point slope .3, directional radial/lateral factor2; native narrow window, original handle/ring features; disabled pocket. Native FEM, loads, materials, masks, mesher, four-sample element-thickness rule and section factors remain unchanged. Six scientific harness files are byte-identical to Phase-14; production FEM recovers exactly after removing the same three observation-only hooks. Export grid=.05 mm and maxarea=.25 mm².

## Source controls

Original no-pocket Q7 reproduces three archived source areas across **60** numeric comparisons; Phase-14 parent reproduces five areas across **110** comparisons, including exact available physical mesh/thickness hashes. Combined maximum relative difference **1.69437351298e-13**. Both new controls retain all five requested areas, totaling ten control observations. Archived Q7 did not contain physical-thickness hashes; that absence is recorded rather than asserted as equality.

## Sampled angle boundary: native25° and requested26°

| dn37 mm | EA ° | Measured max half-angle ° | Native S3 <=25° | Requested <=26° |
| --- | --- | --- | --- | --- |
| 0.3 | 18.0 | 24.048380712 | True | True |
| 0.3 | 18.3 | 24.427771885 | True | True |
| 0.3 | 18.6 | 24.806199036 | True | True |
| 0.3 | 18.9 | 25.183653616 | False | True |
| 0.45 | 18.0 | 23.798559219 | True | True |
| 0.45 | 18.3 | 24.174988902 | True | True |
| 0.45 | 18.6 | 24.550505741 | True | True |
| 0.45 | 18.9 | 24.925101425 | True | True |
| 0.6 | 18.0 | 24.086798631 | True | True |
| 0.6 | 18.3 | 24.466642626 | True | True |
| 0.6 | 18.6 | 24.845514646 | True | True |
| 0.6 | 18.9 | 25.223406105 | False | True |

| dn37 mm | Threshold ° | Assigned sample passes (18/18.3/18.6/18.9) | Observed pass→fail bracket | Status | Exact EA* established |
| --- | --- | --- | --- | --- | --- |
| 0.3 | 25.0 | [True, True, True, False] | 18.600 pass → 18.900 fail | sampled_transition_bracket_only | False |
| 0.3 | 26.0 | [True, True, True, True] | none | all_assigned_samples_pass_no_transition_located | False |
| 0.45 | 25.0 | [True, True, True, True] | none | all_assigned_samples_pass_no_transition_located | False |
| 0.45 | 26.0 | [True, True, True, True] | none | all_assigned_samples_pass_no_transition_located | False |
| 0.6 | 25.0 | [True, True, True, False] | 18.600 pass → 18.900 fail | sampled_transition_bracket_only | False |
| 0.6 | 26.0 | [True, True, True, True] | none | all_assigned_samples_pass_no_transition_located | False |

A passing sample followed by a failing sample only locates an adjacent sampled bracket. When all four samples pass (or all fail), no transition is located within the assigned grid. Four discrete observations do not establish a unique exact root, continuous monotonicity, or compliance between sampled angles. No unobserved exact critical angle is invented, and no out-of-range or hand-built construction is introduced. `angle_boundary.json` preserves the measured endpoint data for both thresholds.

## Complete native exported-solid results

| Probe | dn36/dn37/dn38 mm | EA ° | Measured apex mm | S1 | S2 | Max half-angle ° | Native S3 | Requested26 angle | Bore mm | Census | Volume mm³ | Native full export |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fine_dn0.3_a18 | 0.195000/0.300000/0.240000 | 18.0 | 0.212130984 | False | True | 24.048380712 | True | True | 1.530000000 | [0, 0, 1] | 30983.999881385 | False |
| fine_dn0.3_a18.3 | 0.195000/0.300000/0.240000 | 18.3 | 0.212490252 | False | True | 24.427771885 | True | True | 1.530000000 | [0, 0, 1] | 30984.359587392 | False |
| fine_dn0.3_a18.6 | 0.195000/0.300000/0.240000 | 18.6 | 0.212850766 | False | True | 24.806199036 | True | True | 1.530000000 | [0, 0, 1] | 30984.720527830 | False |
| fine_dn0.3_a18.9 | 0.195000/0.300000/0.240000 | 18.9 | 0.213212553 | False | True | 25.183653616 | False | True | 1.530000000 | [0, 0, 1] | 30985.082683671 | False |
| fine_dn0.45_a18 | 0.292500/0.450000/0.360000 | 18.0 | 0.212144795 | False | True | 23.798559219 | True | True | 1.530000000 | [0, 0, 1] | 31010.472392335 | False |
| fine_dn0.45_a18.3 | 0.292500/0.450000/0.360000 | 18.3 | 0.212504310 | False | True | 24.174988902 | True | True | 1.530000000 | [0, 0, 1] | 31010.832316444 | False |
| fine_dn0.45_a18.6 | 0.292500/0.450000/0.360000 | 18.6 | 0.212865071 | False | True | 24.550505741 | True | True | 1.530000000 | [0, 0, 1] | 31011.193435748 | False |
| fine_dn0.45_a18.9 | 0.292500/0.450000/0.360000 | 18.9 | 0.213227107 | False | True | 24.925101425 | True | True | 1.530000000 | [0, 0, 1] | 31011.555786137 | False |
| fine_dn0.6_a18 | 0.390000/0.600000/0.480000 | 18.0 | 0.212144355 | False | True | 24.086798631 | True | True | 1.530000000 | [0, 0, 1] | 31025.566808069 | False |
| fine_dn0.6_a18.3 | 0.390000/0.600000/0.480000 | 18.3 | 0.212503862 | False | True | 24.466642626 | True | True | 1.530000000 | [0, 0, 1] | 31025.926912127 | False |
| fine_dn0.6_a18.6 | 0.390000/0.600000/0.480000 | 18.6 | 0.212864615 | False | True | 24.845514646 | True | True | 1.530000000 | [0, 0, 1] | 31026.288198541 | False |
| fine_dn0.6_a18.9 | 0.390000/0.600000/0.480000 | 18.9 | 0.213226643 | False | True | 25.223406105 | False | True | 1.530000000 | [0, 0, 1] | 31026.650722822 | False |

Each probe independently runs full native `certification::measure`, including S1 apex<=.2 mm, S2 native point thickness/monotonicity/taper profile, S3 facets<=25°, bore radius>=1.5 mm, census[0,0,1] and positive solid volume. Original native flags and measurement source labels are retained. Requested26 angle pass is not a native full export pass.

## Structural results at every native mesh

| Probe | Area mm² | Sequence | Spine SF | Lateral N | Fatigue SF | Ringback SF | Body HM twist | Global HM twist | Tip proxy SF |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fine_dn0.3_a18 | 4.0 | 0.924252122 | 0.737873218 | 454.713414278 | 1.270162371 | 4.915501023 | 1.068800587 | 0.374510287 | 3.333553108 |
| fine_dn0.3_a18 | 1.0 | 1.131942998 | 0.918301713 | 394.104603960 | 1.100862263 | 4.946521079 | 1.105016543 | 0.327445091 | 2.914621143 |
| fine_dn0.3_a18 | 0.25 | 0.846530644 | 1.073092229 | 459.862314770 | 1.284544924 | 4.832936986 | 1.117594055 | 0.429396866 | 3.822103974 |
| fine_dn0.3_a18 | 0.125 | 0.897572695 | 1.042372335 | 459.544239576 | 1.283656436 | 4.762721359 | 1.112898014 | 0.439861574 | 3.915251377 |
| fine_dn0.3_a18 | 0.0625 | 0.849312831 | 1.087686429 | 474.920948503 | 1.326608583 | 4.569585079 | 1.132909800 | 0.440542362 | 3.921311130 |
| fine_dn0.3_a18.3 | 4.0 | 0.924656206 | 0.739304758 | 454.713412439 | 1.270162366 | 4.915501023 | 1.068976360 | 0.374572533 | 3.334107161 |
| fine_dn0.3_a18.3 | 1.0 | 1.132069374 | 0.920164249 | 394.060600861 | 1.100739348 | 4.946521079 | 1.105108703 | 0.327605913 | 2.916052630 |
| fine_dn0.3_a18.3 | 0.25 | 0.846548264 | 1.073423122 | 459.862322199 | 1.284544945 | 4.832936986 | 1.117478533 | 0.429449440 | 3.822571942 |
| fine_dn0.3_a18.3 | 0.125 | 0.897543598 | 1.043826638 | 459.587768558 | 1.283778027 | 4.762721359 | 1.112926993 | 0.440029801 | 3.916748775 |
| fine_dn0.3_a18.3 | 0.0625 | 0.849372631 | 1.087915334 | 474.921021739 | 1.326608787 | 4.569585079 | 1.132929962 | 0.440711047 | 3.922812613 |
| fine_dn0.3_a18.6 | 4.0 | 0.925058886 | 0.740738074 | 454.713411602 | 1.270162364 | 4.915501023 | 1.069152844 | 0.374635042 | 3.334663558 |
| fine_dn0.3_a18.6 | 1.0 | 1.132192242 | 0.922023242 | 394.017843070 | 1.100619912 | 4.946521079 | 1.105201280 | 0.327766782 | 2.917484541 |
| fine_dn0.3_a18.6 | 0.25 | 0.846565239 | 1.073757813 | 459.862315802 | 1.284544927 | 4.832936986 | 1.117363919 | 0.429503801 | 3.823055807 |
| fine_dn0.3_a18.6 | 0.125 | 0.897513785 | 1.045278642 | 459.632007053 | 1.283901599 | 4.762721359 | 1.112956118 | 0.440195088 | 3.918220017 |
| fine_dn0.3_a18.6 | 0.0625 | 0.849432440 | 1.088148473 | 474.921045498 | 1.326608854 | 4.569585079 | 1.132950238 | 0.440880489 | 3.924320832 |
| fine_dn0.3_a18.9 | 4.0 | 0.925460166 | 0.742173252 | 454.713409582 | 1.270162358 | 4.915501023 | 1.069330053 | 0.374697819 | 3.335222345 |
| fine_dn0.3_a18.9 | 1.0 | 1.132311584 | 0.923878818 | 393.976320193 | 1.100503925 | 4.946521079 | 1.105294278 | 0.327928222 | 2.918921539 |
| fine_dn0.3_a18.9 | 0.25 | 0.846581553 | 1.074096264 | 459.862340881 | 1.284544997 | 4.832936986 | 1.117250225 | 0.429559952 | 3.823555617 |
| fine_dn0.3_a18.9 | 0.125 | 0.897483354 | 1.046728512 | 459.676893855 | 1.284026983 | 4.762721359 | 1.112985396 | 0.440356434 | 3.919656172 |
| fine_dn0.3_a18.9 | 0.0625 | 0.849491941 | 1.088385776 | 474.920817323 | 1.326608216 | 4.569585079 | 1.132970648 | 0.441048282 | 3.925814381 |
| fine_dn0.45_a18 | 4.0 | 0.920531330 | 1.055362788 | 445.654220551 | 1.244857098 | 4.666316864 | 1.266398979 | 0.373418601 | 3.323835903 |
| fine_dn0.45_a18 | 1.0 | 0.838529125 | 0.837854230 | 438.102009564 | 1.223761318 | 4.770501236 | 1.271620443 | 0.338826157 | 3.015925135 |
| fine_dn0.45_a18 | 0.25 | 0.845454955 | 1.152304958 | 465.613088300 | 1.300608704 | 4.734092366 | 0.950190718 | 0.447151934 | 3.980143589 |
| fine_dn0.45_a18 | 0.125 | 0.885703212 | 1.091298995 | 461.498636451 | 1.289115702 | 4.579289491 | 1.175399768 | 0.462981746 | 4.121046307 |
| fine_dn0.45_a18 | 0.0625 | 0.924796221 | 0.988177974 | 449.372505051 | 1.255243476 | 4.605519666 | 1.121252923 | 0.476638363 | 4.242605212 |
| fine_dn0.45_a18.3 | 4.0 | 0.919974003 | 1.056953167 | 445.654229738 | 1.244857124 | 4.666316864 | 1.267411354 | 0.373459322 | 3.324198358 |
| fine_dn0.45_a18.3 | 1.0 | 0.838956626 | 0.839403969 | 438.102007569 | 1.223761313 | 4.770501236 | 1.272029562 | 0.338996141 | 3.017438178 |
| fine_dn0.45_a18.3 | 0.25 | 0.845318012 | 1.152347262 | 465.613077363 | 1.300608673 | 4.734092366 | 0.950491421 | 0.447279222 | 3.981276591 |
| fine_dn0.45_a18.3 | 0.125 | 0.885534331 | 1.091763760 | 461.498604836 | 1.289115614 | 4.579289491 | 1.175428582 | 0.463054720 | 4.121695858 |
| fine_dn0.45_a18.3 | 0.0625 | 0.924580912 | 0.988691159 | 449.372605760 | 1.255243757 | 4.605519666 | 1.121273744 | 0.476810232 | 4.244135033 |
| fine_dn0.45_a18.6 | 4.0 | 0.919415259 | 1.058543017 | 445.654228313 | 1.244857120 | 4.666316864 | 1.268430225 | 0.373500441 | 3.324564363 |
| fine_dn0.45_a18.6 | 1.0 | 0.839383500 | 0.840955578 | 438.102009160 | 1.223761317 | 4.770501236 | 1.272442410 | 0.339166680 | 3.018956162 |
| fine_dn0.45_a18.6 | 0.25 | 0.845180604 | 1.152393253 | 465.613110817 | 1.300608766 | 4.734092366 | 0.950794417 | 0.447407977 | 3.982422656 |
| fine_dn0.45_a18.6 | 0.125 | 0.885364524 | 1.092227293 | 461.498561776 | 1.289115494 | 4.579289491 | 1.175457540 | 0.463130177 | 4.122367511 |
| fine_dn0.45_a18.6 | 0.0625 | 0.924363622 | 0.989208252 | 449.372474621 | 1.255243391 | 4.605519666 | 1.121294672 | 0.476981371 | 4.245658359 |
| fine_dn0.45_a18.9 | 4.0 | 0.918855072 | 1.060132450 | 445.654231036 | 1.244857128 | 4.666316864 | 1.269455687 | 0.373541964 | 3.324933968 |
| fine_dn0.45_a18.9 | 1.0 | 0.839809755 | 0.842509151 | 438.102015468 | 1.223761335 | 4.770501236 | 1.272859038 | 0.339337788 | 3.020479208 |
| fine_dn0.45_a18.9 | 0.25 | 0.845042682 | 1.152442885 | 465.613083515 | 1.300608690 | 4.734092366 | 0.951099741 | 0.447538199 | 3.983581774 |
| fine_dn0.45_a18.9 | 0.125 | 0.885193771 | 1.092689639 | 461.498631731 | 1.289115689 | 4.579289491 | 1.175486644 | 0.463208113 | 4.123061228 |
| fine_dn0.45_a18.9 | 0.0625 | 0.924144651 | 0.989729212 | 449.372282850 | 1.255242855 | 4.605519666 | 1.121315718 | 0.477153892 | 4.247193980 |
| fine_dn0.6_a18 | 4.0 | 1.094366083 | 0.736585804 | 425.900131029 | 1.189677505 | 4.747494685 | 1.083818014 | 0.536024809 | 4.771209842 |
| fine_dn0.6_a18 | 1.0 | 0.946949441 | 0.817442822 | 471.270278057 | 1.316411073 | 4.846442799 | 1.167720269 | 0.425198026 | 3.784729686 |
| fine_dn0.6_a18 | 0.25 | 0.886079320 | 0.991149580 | 454.565925707 | 1.269750388 | 4.764239804 | 1.159429696 | 0.436143295 | 3.882154601 |
| fine_dn0.6_a18 | 0.125 | 0.837718368 | 1.069701120 | 445.156548788 | 1.243466939 | 4.745125742 | 1.233318724 | 0.446480270 | 3.974165041 |
| fine_dn0.6_a18 | 0.0625 | 0.823503605 | 1.014227789 | 465.505617873 | 1.300308504 | 4.631394615 | 1.173252416 | 0.421440345 | 3.751282189 |
| fine_dn0.6_a18.3 | 4.0 | 1.093615569 | 0.738020418 | 426.192415674 | 1.190493951 | 4.747494685 | 1.083996789 | 0.536077892 | 4.771682338 |
| fine_dn0.6_a18.3 | 1.0 | 0.947290465 | 0.819275289 | 471.270297047 | 1.316411126 | 4.846442799 | 1.167793897 | 0.425368698 | 3.786248848 |
| fine_dn0.6_a18.3 | 0.25 | 0.885954184 | 0.992629548 | 454.565922640 | 1.269750379 | 4.764239804 | 1.159647116 | 0.436257236 | 3.883168805 |
| fine_dn0.6_a18.3 | 0.125 | 0.837429935 | 1.069143007 | 445.188380827 | 1.243555857 | 4.745125742 | 1.233388327 | 0.446353727 | 3.973038670 |
| fine_dn0.6_a18.3 | 0.0625 | 0.823245204 | 1.014847097 | 465.505739362 | 1.300308843 | 4.631394615 | 1.173273999 | 0.421530063 | 3.752080781 |
| fine_dn0.6_a18.6 | 4.0 | 1.092863039 | 0.739456811 | 426.485879749 | 1.191313692 | 4.747494685 | 1.084176306 | 0.536131079 | 4.772155754 |
| fine_dn0.6_a18.6 | 1.0 | 0.947630723 | 0.821110218 | 471.270292742 | 1.316411114 | 4.846442799 | 1.167867861 | 0.425539267 | 3.787767098 |
| fine_dn0.6_a18.6 | 0.25 | 0.885828461 | 0.994110609 | 454.565929029 | 1.269750397 | 4.764239804 | 1.159866214 | 0.436364638 | 3.884124795 |
| fine_dn0.6_a18.6 | 0.125 | 0.837140558 | 1.068592424 | 445.221078414 | 1.243647191 | 4.745125742 | 1.233462975 | 0.446223664 | 3.971880962 |
| fine_dn0.6_a18.6 | 0.0625 | 0.822986057 | 1.015470359 | 465.505972342 | 1.300309494 | 4.631394615 | 1.173295669 | 0.421630593 | 3.752975612 |
| fine_dn0.6_a18.9 | 4.0 | 1.092108459 | 0.740895068 | 426.780565049 | 1.192136844 | 4.747494685 | 1.084356580 | 0.536184372 | 4.772630126 |
| fine_dn0.6_a18.9 | 1.0 | 0.947970237 | 0.822947715 | 471.270291416 | 1.316411111 | 4.846442799 | 1.167942168 | 0.425709745 | 3.789284542 |
| fine_dn0.6_a18.9 | 0.25 | 0.885702098 | 0.995592865 | 454.565919963 | 1.269750371 | 4.764239804 | 1.160087008 | 0.436471895 | 3.885079509 |
| fine_dn0.6_a18.9 | 0.125 | 0.836850177 | 1.068049221 | 445.254585092 | 1.243740787 | 4.745125742 | 1.233542687 | 0.446094508 | 3.970731333 |
| fine_dn0.6_a18.9 | 0.0625 | 0.822725929 | 1.016097535 | 465.505640882 | 1.300308568 | 4.631394615 | 1.173317465 | 0.421731317 | 3.753872161 |

Joint comparisons retain sequence>=1.20, spine>=1, lateral>=260 N, fatigue>=1, ringback>=1, body HM twist>=1 and native global HM twist>=1, plus native geometry/aesthetics and matched full export. Fatigue1 implies about357.9963 N under unchanged native law; 260 N alone is insufficient. All finite-grid failures are retained. Native tipzone proxy uses1820 Nmm in4<=r_tip<15 mm; global HM remains twist_joint_nmm/16200. Original body r_tip>=15 and global r_tip>=4 masks are unchanged; body-only capacity does not replace global. Actual peak indices, stress, mean thickness, width and section correction factors remain in raw receipts and `fem_summary.csv`.

## Integrity, receipt manifests and reproduction

Verification passes **80** production digests, **24** frozen inputs, exact nine-gene construction and physical knob mapping, complete12-probe Cartesian coverage, native canonicality, all five meshes/full exports including failures, exact source controls, original native masks/peaks/metric/torque identities, protected full-stock neck, same-family planform mesh hashes across angles and fixed-coordinate annulus observations. All **38** invocations bind one binary, CPUs10,11/jobs2/faer/**actual nice19**. Exact nice10 was denied and is not claimed.

Phase-14 `unified_study_v11/receipt_manifest.json` is already covered by its unchanged386-artifact SHA256SUMS. Both are frozen Phase-15 inputs, independently verified and sealed again in this study’s catalog. Phase-15 produces both SHA256SUMS and receipt_manifest.json; the root receipt catalogs every other stable artifact, and SHA256SUMS also seals that receipt, avoiding circular self digests. All eleven prior seals and eight native dependencies are reverified at final sealing.

| Artifact | Full SHA-256 |
| --- | --- |
| study | 68c89c5d16ac9e2b3a1fffd7f3c9595a9b14357b2a7893418ad40d41c798ed51 |
| source_provenance.json | 2782f50b83b60a1c2113c7380198212d2de1b96cb31caf0989f267fbde3e87ed |
| execution_constraints.json | 7f58a9bf5b217431390fb15974a8dfc86d491a595bfcc6ea90153a82484737cc |
| verification.json | 6bb95e18af4ae0e3a211faa0053fb5dbc17764f01c94193a79e9d312195af9dc |
| control_verification.json | 6c8b920aa5537a11c29a92032b74155d1c7eeb5ebd14bdad0e3282f5620bdf50 |
| angle_boundary.json | 7dfd6a3a88a7e3249cd890eb401ba6524e82d86b56deacea1e36d6e85a3673de |
| physical_models.json | 514f32666ddf5769ea0236d89e881c34ecb222956cb0824f6d20abb817ea0ebe |
| fem_summary.csv | 9a32894dd34f6293e6ce8df0c4681c49d5ff07ef646c85a1acc8f63cd93ee3b2 |
| summary.json | b7d1f9a4f093e2675c9f1df2090bdcb75d92c32ba12186d2b2d52366b1059629 |
| inputs/assignment.md | 9176a63285b0c965bc31b14da449cf0eccb5ddc16e95884d5ba9d6731de2182e |
| inputs/receipt_manifest.json | 73e55a747fd9ded60e024d3a047d91aa0f87005526c4d2aa8c24cff99dd1e877 |
| inputs/SHA256SUMS | 36a54d92768fe27e1840fbfa1629cfaaa0eadccb1a93e20a75204e77c3d2476a |

| Probe | Input CSV SHA-256 | FEM receipt SHA-256 | Full export receipt SHA-256 |
| --- | --- | --- | --- |
| fine_dn0.3_a18 | 9a3375505cc66555f4bf10766595b216782402182a10f172046c5f7e7bc3b065 | dad72285a4c5abee97de1ae012bf5a0787075ebcc1251f3f3db923efce2b9d3d | 5bf60a410cd052f2aabd787ea17e102b82325e38cb9349c1fa705e3bbdce0ccc |
| fine_dn0.3_a18.3 | f7d80595748b5d36a247409437f9be959082d5151dd6c1fa626b9b61119e0c65 | 272f1f6fc0bf6bf040aab16f69992a284acf542497769dfbfb044235a2a03659 | 32bc6fc2e1206a0499a94eb964678d3de0f4f562634d0116871301b3c44301b3 |
| fine_dn0.3_a18.6 | 80a23f60ca6871d6e0e074ddf4a5a7a11fe71ad7494e253c912d4c58e8cade94 | 745de56ccff99dd745e8d0b603fb2c24c7eed85eebc1960f5e4cda8022798993 | 2479d1572db85cc71f28dbb0bed6598c38d71459a213c3db64d769c268b53f8b |
| fine_dn0.3_a18.9 | 3c1ce03aceff00f69b139c8433ea1103bcadff5e0cab0eaca9db19a8c75d4fc9 | 98a326fbbb0bc50de8ef73f709696851ed6ee2473bafa3048593bc544703e6d9 | 8e43f1b69ddc60716852118eb606df7e9824a67edbd565b30030621e59fb3db9 |
| fine_dn0.45_a18 | 9c3a8961befc8f32fd91d7c8e14c838406761ad738a347b72c46b651ff16d62d | 122274348908dd1b0873c19591473cd48d2623fb690b5fb43e89100154a19693 | 16cfde48a39f69f380470e5fbbaf7dbb43b5edf3ae85166b14f71eec51c46d20 |
| fine_dn0.45_a18.3 | 95e27ee1245a60164141d3de816d80cf0e42bfa542deb1fd13864f0b644629c6 | 5c6d6bf33428a5b8d0783af3843908e284dfbb138df6ec40e7f4e04cc7d78e55 | 1c14bc87c8ba4d9a0774d61b4d817997b7dde3071215bfb05a73a53c9661f77e |
| fine_dn0.45_a18.6 | 87066f0eba07d1d39710a86c36a05097699ee4e55647a22191ec0796cdada70f | ec70c6684ec4d784dad820464f6237bb2f3fbb5774caf40cd8e52aaed416d7e5 | 1671ba4a4db5677823fae352b89d48e047ac05ae024d50076940248a78f442bd |
| fine_dn0.45_a18.9 | 1c6f1eea5794d2e9d25e13208716a8d41a1d3cb6012fbfa9ebf71aadba80d6b3 | 8e58ffdc7a5daddee68f0f7887847de76587d2bd4833ba49c38096a1ea647daa | 14dfe64a0570d85f38cacbd529713a6b9f1b5b7d44e8becbb1ea83f48618f751 |
| fine_dn0.6_a18 | bfc664328a4d829f96eb6ed24e4bf2d636b964c45343bf24e0f346129a2a6945 | f55b3cba362e633035cf35f0202f68e14db6f44c6aef35ea702b2aba19f92878 | 09c3470d9c84eda0fadb25e36d885f1688bd5053f03b9ff3a71fd19e03826ee4 |
| fine_dn0.6_a18.3 | 19261e18ef13f5b8f7cdc3d6b98862542e95d333b2a63cc6dc2179f8643ebb49 | 4bf24e1628a8e196d4209690cb6d238544a1bcce32de47fe321a1e2a14e2c36e | 2a75f22ccf77f4b0b3009b7904c78b0068db0a04fe82886d8d5be11e0936d038 |
| fine_dn0.6_a18.6 | 065facc42c07d84675972605602b004cbbc13daf8d03456da55e465e098647a9 | 1e2837afaf0e0f0dbdc5556b0c9beb23db4916394aa3485e8fb019d4ce864c5e | a84e1506bac433f169369e479ba4e09237ea0c08d7699a2f3fba037755ad5e0b |
| fine_dn0.6_a18.9 | 63fa646076120424533637385e70af4f6013c9f3a1b996f42d2d0bfe02e18f63 | 2dc5b8573354eeb371c5a02e92700cdae924e0afc1dbc30ac2c9ab008b28901a | 950b7817442cd4157d72b839c256d24616d109b38a066f5021f764cf91181a2a |

Primary report: `experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V12_REPORT.md`; byte-identical mirror: `target/gate-feasibility-bound/UNIFIED_PROTOTYPE_V12_REPORT.md`; raw evidence/scripts: `target/gate-feasibility-bound/unified_study_v12/`. Full report digest is held by REPORT_SHA256, run provenance, both manifests and both agent trackers. `verify_seal.py` checks every digest and the complete receipt catalog. Workflow syntax and `git diff --check` pass; report whitespace is also checked directly.

```bash
nice -n 10 taskset -c 10,11 env CARGO_BUILD_JOBS=2 JARAMBIT_SOLVER=faer \
  bash target/gate-feasibility-bound/unified_study_v12/reproduce.sh
```

The command requests +10 niceness adjustment but this runner already inherits19, so actual19 remains recorded. No production edit, git add/commit/push, PR, stacking, worktree/branch change or deletion. Report left uncommitted for captain. This finite grid establishes neither an exact continuum threshold nor an all-hard certificate. Both trackers updated at milestones; await captain_jarambit.
