# Phase-7: neutral-axis interior web fuller on Q7

**Result: the requested stiffness decoupling is observed, but no sampled joint closure.** The interior fuller raises blade/hand sequence and preserves the original thickness exactly in the protected spine/edge bands. Section quadrature shows substantially greater reduction of integral(t^3) than of in-plane integral(t*(y-y0)^2). Native FEM nevertheless retains a compliance/strength tradeoff: all 24 first-mesh joint passes fail sequence on the second mesh, and strict strength fails in the retained near misses. A larger-removal contrast passes sequence on all three meshes but loses lateral/fatigue/spine. Both measured pocketed exports pass every requested export check.

Worktree `/home/ubuntu/jarambit_wt_diag`, branch `audit/gate-feasibility-bound`, baseline `d046093432864ac3cc619dc40c177c452e1e967a`. All scientific execution/compilation uses CPUs 10,11, CARGO_BUILD_JOBS=2, explicit faer, CARGO_INCREMENTAL=0 and the existing lane target/dependencies. No production source, native load, mask, element-thickness averaging or acceptance threshold was changed. The result is a bounded numerical experiment, not a global infeasibility proof.

## Parent, exact pocket construction and controls

Parent Q7 is the exact Phase-6 literal-index `split_hand_blade_literal_neckdrop_w3_s0.5_0.85_b0_qual.csv`. Fifteen stable parent source/input/control files are frozen in `inputs/`. Every Q7 gene is fixed except WIN_N=0/1; all CPs, hand parameters, stock=6 mm, spine start=42 mm, bevel=0.48/0.65 mm, directional POINT_SHAPE=3/POINT_LATERAL=2 and distal-window placement remain unchanged. P1 is not enabled. The fuller is the explicitly assigned prototype surface law recorded in each `_case.json`; the CSV alone does not reproduce it in production.

The assignment quotes 24.18 degrees for Q7. Its exact retained native source export measured 24.32422429587 degrees, apex 0.195144636429 mm, point pass, bore 1.60 mm and census [0,0,1]. That source pass is preserved as a control reference; it is not substituted for a pocketed measurement.

Disabled-pocket Q7 reproduces all three source FEM meshes to maximum comparable scalar relative error 2.14728681812e-13 over 60 observations. Disabled-pocket with/without-window controls both have areas 0.25/0.125/0.0625 mm². Baseline law recovery is checked byte for byte after removing only the pocket wrapper/config additions.

Let S(u)=clamp(u,0,1)^2*(3-2*clamp(u,0,1)). Let ds be the minimum of exact segment distance to the full smooth spine (curve CP30..tip) and the actual sharpened-spine segments; de is exact distance to the actual inner cutting-edge segments. Fixed extent is x=102..165 mm and blend length b=1.5 mm. With spine guard gs and edge guard ge:

```text
w = S((x-102)/1.5) * S((165-x)/1.5)
    * S((ds-gs)/1.5) * S((de-ge)/1.5)
w = 0 outside 102 < x < 165, or when the pocket is disabled
Tnew(q) = T_Q7(q) - max(T_Q7(q)-t_pocket, 0) * w
```

The native exporter applies this full thickness symmetrically about its midplane. The law never adds material and never reduces thickness below the requested floor in the modified web. It is identically the parent law where ds<=gs, de<=ge, or outside the longitudinal extent. Existing full-stock flange samples remain 6 mm. Existing bevel/point-cap material is preserved, rather than filled back to 6 mm. Four unchanged source files implement phenotype, FEM, exporter and diagnostic hooks; production certification is included directly.

## Search coverage and hierarchical mesh checks

All 72 grid tuples were evaluated at area 0.25 mm²: t_pocket={2.6,3.2,3.8,4.4} mm, gs={5.5,6.25,7.0} mm, ge={5.0,5.75,6.5} mm, WIN_N={0,1}. Sixteen endpoint tests preceded the remaining 56 cases. Every input is canonical and passes the unchanged native planform/aesthetic predicates.

Exactly 24 tuples pass sequence>=1.25, spine>=1, lateral>=358 N and fatigue>=1 together on the first mesh. Every one was then checked at area 0.125, and every one fails sequence>=1.25 there. Five of those cases additionally have strict area 0.0625; the maximum-removal contrast has all three areas as well. Other coarse failures cannot be all-three-mesh passes and were not silently promoted. `qualification.json` lists the exact matched-input receipts/levels for all 24 second-mesh screens.

Completed 111 unbiased native FEM observations in 103 receipts, including six disabled-pocket controls, plus 2 complete native export measurements. Node/triangle counts match the fixed-planform control for each window family/mesh. Mesher inputs, operators, four-sample element thickness and loads are unchanged; thickness law is the experimental variable. `fem_summary.csv` contains every observation.

## Complete three-mesh profiles

These are minima across all three required areas. They never combine metrics from different profiles. The primary is the largest minimum normalized structural margin among the five fully evaluated first-pass candidates; it is a representative near miss, not a qualified design.

| ID | Case | Sequence >=1.25 | HM spine >=1 | Lateral >=358 N | Fatigue >=1 |
| --- | --- | --- | --- | --- | --- |
| P1 | web_t2.6_s7_e6.5_w1 | 1.180924270 | 0.958834439 | 332.109797344 | 0.927690617 |
| P2 | web_t3.2_s5.5_e6.5_w0 | 1.196597777 | 0.966224045 | 328.086586608 | 0.916452481 |
| P3 | web_t3.2_s5.5_e6.5_w1 | 1.196207364 | 0.966224710 | 328.046310254 | 0.916339976 |
| P4 | web_t3.2_s6.25_e5.75_w0 | 1.191229913 | 0.956947698 | 330.466301061 | 0.923099797 |
| P5 | web_t3.2_s6.25_e5.75_w1 | 1.190862574 | 0.956948641 | 330.421317100 | 0.922974142 |
| C1 | web_t2.6_s5.5_e5_w1_contrast | 1.349981092 | 0.920616674 | 302.403968525 | 0.844712581 |

Primary construction: t_pocket=2.6 mm, spine guard=7 mm, edge guard=6.5 mm, original distal window retained, x=102..165 mm, all runouts 1.5 mm. Its only gene difference from Q7 is none; the changed thickness is entirely specified by the pocket config.

| Area mm² | Sequence | HM spine | Lateral N | Fatigue | Spine peak x,y mm |
| --- | --- | --- | --- | --- | --- |
| 0.25 | 1.303026641 | 1.030453617 | 365.632275970 | 1.021329796 | [100.51466426636259, -12.322975014499171] |
| 0.125 | 1.180924270 | 1.012133758 | 406.199714578 | 1.134647838 | [107.83060263944975, -12.004190305608311] |
| 0.0625 | 1.445027975 | 0.958834439 | 332.109797344 | 0.927690617 | [107.85295710791556, -12.002552918124557] |

The first-mesh primary pass does not survive the stronger checks: sequence is 1.180924270 at area 0.125, while strict lateral=332.109797 N, fatigue=0.927690617 and spine=0.958834439 fail. Strict spine peak remains near (107.85296,-12.00255) mm. Protected local material is unchanged, but its recovered stress increases as the surrounding web is removed. Maintaining the flange thickness therefore does not guarantee its prior stress margin.

Contrast C1 uses the maximum-removal assigned profile (t=2.6, gs=5.5, ge=5 mm, window retained). Its all-three-mesh sequence minimum is 1.349981092, but lateral minimum=302.403969 N, fatigue=0.844712581 and spine=0.920616674 fail. This directly demonstrates an all-mesh sequence pass at the expense of strength without borrowing results from another construction. Human-max twist remains below 1 and is retained in raw metrics; no all-hard qualification follows.

## Protected-band and stiffness diagnostics

All active profiles have nonzero sampled material removal. Every receipt has zero thickness delta in both protected bands and outside the fixed extent, and minimum protected original full-stock thickness=6.0 mm. For the primary, 5,506 material samples change, minimum modified thickness is exactly 2.6 mm, 13,869 protected full-stock samples remain 6 mm, and the raster quadrature estimates 837.404865 mm³ removed. The analytic zero-mask conditions protect the entire bands; sample audits check their implementation.

The following primary section diagnostics use dy=0.05 mm quadrature over the actual material, excluding holes. Each in-plane I uses its own thickness-weighted neutral axis. Integral(t^3) is a stiffness proxy, not a substitute for the native plate solve.

| x mm | In-plane I / parent I | Integral(t³) / parent | Parent neutral y mm | Pocket neutral y mm |
| --- | --- | --- | --- | --- |
| 104.0 | 0.971209185 | 0.657892662 | -0.166804635 | -0.013033339 |
| 110.0 | 0.977463869 | 0.686147681 | -0.845794098 | -0.706714035 |
| 120.0 | 0.984061904 | 0.720914064 | -2.940760918 | -2.827118191 |
| 130.0 | 0.988964169 | 0.755969301 | -5.915288237 | -5.816000368 |
| 140.0 | 0.996549707 | 0.856557292 | -10.166367492 | -10.101414489 |
| 150.0 | 0.999899659 | 0.970365830 | -15.725498838 | -15.721540780 |
| 160.0 | 1.000000000 | 1.000000000 | -22.211857061 | -22.211857061 |
| 164.0 | 1.000000000 | 1.000000000 | -24.923394243 | -24.923394243 |

At x=110 the in-plane proxy retains 97.75% while the out-of-plane proxy retains 68.61%; at x=130 they retain 98.90%/75.60%. This confirms the intended preferential stiffness reduction. It does not establish the required stress-factor/sequence combination. Where the blade cannot accommodate the guards, the smooth distance mask naturally vanishes; for this primary, x=160/164 sections are unchanged even though they are inside the allowed extent.

## Actual exported-solid measurements

Both measurements use the exact genes and physical config of their corresponding FEM profiles. Full native certification computes apex, inspected triangle half-angle, point profile and bore radius. The actual exported solid supplies positive volume and watertight census. No angle-only estimate substitutes for this table.

| ID | Half-angle deg <=25 | Apex mm <=0.20 | Point pass | Bore mm >=1.5 | Census | All export checks |
| --- | --- | --- | --- | --- | --- | --- |
| P1 | 24.324224295870 | 0.195144636429 | True | 1.600000000 | [0, 0, 1] | True |
| P4 | 24.324224295870 | 0.195144636429 | True | 1.600000000 | [0, 0, 1] | True |

Both solids retain the source edge/point/bore behavior while introducing the interior fuller. Their worst triangle vertices, exact edge distances, point-profile samples, actual volume and mesh census are retained in the raw export receipts. Export passes do not repair the structural failures.

## Verification, reproduction and handoff

Verified 77 production source/requirement/build-file digests and 15 stable source snapshots. Four scientific source files remain byte-identical to Phase-6. The original compound law is recovered exactly by removing the added pocket wrapper/config; FEM matches production after removing the two pre-existing postcalculation observation hooks. Verified fixed Q7 genes, canonical/native geometry predicates, parameter bounds, guard/extent retention, 6 mm protected stock, native metric identities, control parity, unchanged mesh inputs/counts and matched FEM/export law/genes. Eight dependency digests and the original Phase-4/5/6 seals are checked at seal time.

Run `taskset -c 10,11 env CARGO_BUILD_JOBS=2 bash target/gate-feasibility-bound/unified_study_v4/reproduce.sh`. It rebuilds from stable source inputs, reruns controls and the entire 72-point grid, checks every first-mesh pass at 0.125, completes strict/contrast observations, and performs both native exports. It reuses this lane target and dependencies. SHA256SUMS seals the report and all stable study artifacts; REPORT_SHA256 identifies the report.

Phase-7 evaluation is complete; joint target closure remains failed on the tested grid. Next captain-directed work should address the heel stress redistribution/longitudinal pocket placement while retaining the protected bands and actual-export repair. The present study keeps the assigned x=102..165 extent and 1.5 mm blend; it does not invent a new pocket location or change native masks/loads. Await captain_jarambit before a new scientific milestone. No production edits, commit, push, PR, branch/worktree changes, stacking or deletion. Shared Git/COMMS metadata remain read-only. Independent 3D/cross-solver/GCI/support/manufacturing qualification remains unassessed; no global infeasibility proof or all-hard certificate is claimed.
