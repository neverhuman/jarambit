# G16-12-RSV2-04 Phase-12: bounded outward tip-control-point sweep

Task `G16-12-RSV2-04-p12`; execution commit `968fa148c290b9c1c01785c3d9ff1be629c7c6a9`; branch `audit/gate-feasibility-bound`; worktree `/home/ubuntu/jarambit_wt_diag`. All native evaluations run on CPUs **10,11**, `CARGO_BUILD_JOBS=2`, faer, requested `nice -n 10` (inherited effective niceness 19). No commit or push attempted. Assignment is frozen in `inputs/assignment.md`.

## Result

All **25 requested delta_in/delta_sp tuples** were screened with unchanged native gates and aesthetics. They produce **15 distinct clamped genomes**. **4 requested profiles pass native domain/geometry/aesthetic predicates**; 21 are rejected and retained. Every legal tuple receives all **4.0 / 1.0 / 0.25 / 0.125 / 0.0625 mm²** unbiased native FEM meshes and a matched complete native exported-solid measurement. Total **24 native FEM observations**, including four independent source-control observations; complete exports **4**, export passes **4**.

Joint structural/export passes: fine pair 0.25/0.125 **0**; three fine meshes 0.25/0.125/0.0625 **0**; all five meshes **0**. These are bounded results for the exact assigned discrete grid, not a continuous/global infeasibility claim or an all-hard production certificate.

Across the legal fine-mesh observations, global HM twist ranges **0.404390387..0.518744987**. The original global torsion stress peak lies in the native 4<=r_tip<15 mm annulus in **12/12** observations. No body-only or proxy torque metric substitutes for native global HM twist.

The largest legal spine increment (.9 mm) adds **1.367989529 mm** of material width at the fixed4 mm inward-axial station and raises strict global HM twist to **0.518744987** (**24.482672%**). This improvement is resolution-dependent: its area.125 global HM twist changes by **-1.004909%**, and joint margins remain failed.

## Exact construction, native clipping and metric names

Parent is the unchanged native-domain Phase-10/11 `armB_s0.5_w3_x144_t2.8_s4.5_e5.5_d0`: narrow window S0/S1=.50/.85, W=3, fill=.65; pocket floor 2.2 mm, x=126..156, spine/edge guards 4.5/5.5 mm; original 1.5 mm cubic smoothstep, bevel d0/d1=.48/.65 mm, near-edge 18° law, apex parameter 0.175 mm, directional point shape=3/lateral=2, 6 mm stock/core/full neck. All genes except CP1/2/3 and36/37/38 are bit-identical to the parent. Physical pocket/bevel/export/FEM law, loads, masks, material, averaging, mesher, native bounds and thresholds are unchanged.

For each of delta_in and delta_sp in `{0,.3,.6,.9,1.2}`, the appropriate CP gene is `clamp(parent_cp + delta, -cp_delta, cp_delta)`. Native `pheno::control_points` uses the signed reference contour orientation and neighboring-point tangent to apply a positive gene along the outward normal. Thus this is the assigned outward-normal operation, without a new planform law. Native `gates::check` and `aesthetic::evaluate` perform unchanged construction screening.

| CP | Native reference x,y mm | Parent offset mm | Native ±bound mm | Actual increment at requested 1.2 mm |
| --- | --- | --- | --- | --- |
| 1 | [183.1, -45.8] | 2.472034576 | 3.0 | 0.527965424 |
| 2 | [175.3, -38.2] | 3.000000000 | 3.0 | 0.000000000 |
| 3 | [166.0, -31.0] | 3.000000000 | 3.0 | 0.000000000 |
| 36 | [181.5, -29.8] | -0.133767371 | 2.5 | 1.200000000 |
| 37 | [188.3, -40.8] | -0.888966426 | 2.5 | 1.200000000 |
| 38 | [190.5, -48.0] | -0.891854636 | 2.5 | 1.200000000 |

CP2/3 already equal their +3 mm bound, so they do not move further. CP1 increases by.3 mm at delta_in=.3, then saturates at +3 mm (actual maximum increment 0.527965423738 mm) for requested .6/.9/1.2. All three spine CPs have enough bound room for the full requested 1.2 mm. Requested values, unclipped gene requests, final genes and actual increments are separately recorded in `search_plan.json`; every tuple is native canonical. Identical clamped requests remain distinct independent native geometry observations, cataloged in `clamped_aliases.json`. All multiple-request saturation aliases fail native aesthetics, so they receive no qualifying FEM/export; the four legal profiles have distinct genomes. Physical-mesh hash comparisons apply when legal same-genome repeats exist.

The task’s `twist_tip_sf` is reported explicitly as native `twist_tipzone_sf` at each area (the original 1820 Nmm proxy torque). `twist_tipzone_sf_strict` is that same native field at maxarea 0.0625. Human-max annulus/body observations use `native twist_tipzone_sf or twist_body_sf *1820/16200`; native global `hm_twist_sf` remains `twist_joint_nmm/16200`. These quantities are not interchangeable, and no mask or torque floor was changed.

Fixed-station physical widths use exact intersections of the actual native outline/holes with lines perpendicular to the fixed ring-tip chord, at inward axial distances 4/6/8/10/12/15 mm. Those are axial station widths, not circular-radius widths. Native FEM stress-peak chord width, thickness and section factor are recorded separately because mesh-induced peak movement can change that selected section.

## Native geometry/aesthetic screen

| delta_in / delta_sp | 0.0 | 0.3 | 0.6 | 0.9 | 1.2 |
| --- | --- | --- | --- | --- | --- |
| 0.0 | PASS | PASS | PASS | PASS | FAIL |
| 0.3 | FAIL | FAIL | FAIL | FAIL | FAIL |
| 0.6 | FAIL | FAIL | FAIL | FAIL | FAIL |
| 0.9 | FAIL | FAIL | FAIL | FAIL | FAIL |
| 1.2 | FAIL | FAIL | FAIL | FAIL | FAIL |

- 4 probes: spine not a continuous convex crescent (0.88 < C.4 0.94-0.03)
- 5 probes: Holder envelope: best oriented box 200.2 x 100.0 > 200 x 100
- 5 probes: hawkbill edge not continuously concave (0.93 < C.4 1.00-0.03)
- 1 probes: spine not a continuous convex crescent (0.87 < C.4 0.94-0.03)
- 15 probes: hawkbill edge not continuously concave (0.90 < C.4 1.00-0.03)

All rejected probes retain native domain flags, aesthetic failures, gate rejects, actual window count, actual CP coordinates and geometric chord widths. Structural results are not inferred for a rejected construction. Every legal tuple receives every assigned mesh, regardless of earlier structural failures.

## Native control parity

Frozen Phase-11 parent reproduces areas1/.25/.125/.0625 across **88 scalar comparisons**, maximum relative error **8.52135914238e-14**, with exact T6 geometry/thickness hashes. Source control and new baseline tuple are separately retained. Six scientific harness files are byte-identical to Phase-11; native FEM source is recovered after removing the same three observation-only hooks.

## How much the legal outward offsets change the requested metrics

Per-mesh changes below compare to the independently evaluated delta_in=delta_sp=0 baseline on the identical mesh-area setting. A positive single-mesh change does not establish convergence or an all-mesh strength pass.

| Profile | Area mm² | Tip proxy SF | Δproxy % | HM annulus | HM global | ΔHM global % | Native selected width mm |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `outward_in0_sp0` | 4.0 | 3.891440656 | 0.000000 | 0.437186543 | 0.437186543 | 0.000000 | 7.950000000 |
| `outward_in0_sp0` | 1.0 | 3.709835429 | 0.000000 | 0.416783980 | 0.416783980 | 0.000000 | 9.200000000 |
| `outward_in0_sp0` | 0.25 | 3.775879781 | 0.000000 | 0.424203778 | 0.424203778 | 0.000000 | 8.850000000 |
| `outward_in0_sp0` | 0.125 | 3.876274861 | 0.000000 | 0.435482731 | 0.435482731 | 0.000000 | 8.900000000 |
| `outward_in0_sp0` | 0.0625 | 3.709271632 | 0.000000 | 0.416720640 | 0.416720640 | 0.000000 | 8.750000000 |
| `outward_in0_sp0.3` | 4.0 | 4.492388925 | 15.442822 | 0.504700484 | 0.504700484 | 15.442822 | 8.050000000 |
| `outward_in0_sp0.3` | 1.0 | 3.799164316 | 2.407894 | 0.426819695 | 0.426819695 | 2.407894 | 9.400000000 |
| `outward_in0_sp0.3` | 0.25 | 3.599518829 | -4.670725 | 0.404390387 | 0.404390387 | -4.670725 | 9.150000000 |
| `outward_in0_sp0.3` | 0.125 | 3.878749459 | 0.063840 | 0.435760742 | 0.435760742 | 0.063840 | 9.150000000 |
| `outward_in0_sp0.3` | 0.0625 | 4.014168522 | 8.219859 | 0.450974488 | 0.450974488 | 8.219859 | 9.800000000 |
| `outward_in0_sp0.6` | 4.0 | 5.117976671 | 31.518816 | 0.574982564 | 0.574982564 | 31.518816 | 9.900000000 |
| `outward_in0_sp0.6` | 1.0 | 2.971070857 | -19.913675 | 0.333786973 | 0.333786973 | -19.913675 | 8.850000000 |
| `outward_in0_sp0.6` | 0.25 | 3.854835074 | 2.091044 | 0.433074064 | 0.433074064 | 2.091044 | 9.550000000 |
| `outward_in0_sp0.6` | 0.125 | 3.916614621 | 1.040684 | 0.440014729 | 0.440014729 | 1.040684 | 10.400000000 |
| `outward_in0_sp0.6` | 0.0625 | 4.010460592 | 8.119895 | 0.450557918 | 0.450557918 | 8.119895 | 10.400000000 |
| `outward_in0_sp0.9` | 4.0 | 4.866318864 | 25.051859 | 0.546709897 | 0.546709897 | 25.051859 | 9.650000000 |
| `outward_in0_sp0.9` | 1.0 | 4.169594747 | 12.392984 | 0.468435953 | 0.468435953 | 12.392984 | 10.600000000 |
| `outward_in0_sp0.9` | 0.25 | 4.168273008 | 10.392101 | 0.468287461 | 0.468287461 | 10.392101 | 10.100000000 |
| `outward_in0_sp0.9` | 0.125 | 3.837321813 | -1.004909 | 0.431106525 | 0.431106525 | -1.004909 | 10.800000000 |
| `outward_in0_sp0.9` | 0.0625 | 4.617400433 | 24.482672 | 0.518744987 | 0.518744987 | 24.482672 | 10.350000000 |

| Profile | Strict tip proxy SF (.0625) | Δstrict proxy % | Strict global HM | Min global HM on fine three | Min body HM on fine three | Export |
| --- | --- | --- | --- | --- | --- | --- |
| `outward_in0_sp0` | 3.709271632 | 0.000000 | 0.416720640 | 0.416720640 | 1.021661607 | True |
| `outward_in0_sp0.3` | 4.014168522 | 8.219859 | 0.450974488 | 0.404390387 | 1.087742839 | True |
| `outward_in0_sp0.6` | 4.010460592 | 8.119895 | 0.450557918 | 0.433074064 | 1.366495697 | True |
| `outward_in0_sp0.9` | 4.617400433 | 24.482672 | 0.518744987 | 0.431106525 | 1.377772455 | True |

## Measured physical chord widening

| Profile | Width at axial 4 mm | Width at axial 6 mm | Width at axial 8 mm | Width at axial 10 mm | Width at axial 12 mm | Width at axial 15 mm |
| --- | --- | --- | --- | --- | --- | --- |
| `outward_in0_sp0` | 9.957684586 | 11.192962211 | 12.374948965 | 13.417405515 | 14.255913422 | 12.978853178 |
| `outward_in0_sp0.3` | 10.414826477 | 11.646475191 | 12.826227169 | 13.853847387 | 14.656944242 | 13.053562328 |
| `outward_in0_sp0.6` | 10.870379626 | 12.102179208 | 13.279090935 | 14.288401899 | 15.051683623 | 13.252817957 |
| `outward_in0_sp0.9` | 11.325674115 | 12.560385465 | 13.733182411 | 14.720220455 | 15.439535227 | 13.356611852 |

| Profile | Δwidth at axial 4 mm | Δwidth at axial 6 mm | Δwidth at axial 8 mm | Δwidth at axial 10 mm | Δwidth at axial 12 mm | Δwidth at axial 15 mm |
| --- | --- | --- | --- | --- | --- | --- |
| `outward_in0_sp0` | 0.000000000 | 0.000000000 | 0.000000000 | 0.000000000 | 0.000000000 | 0.000000000 |
| `outward_in0_sp0.3` | 0.457141891 | 0.453512981 | 0.451278205 | 0.436441872 | 0.401030819 | 0.074709150 |
| `outward_in0_sp0.6` | 0.912695040 | 0.909216998 | 0.904141970 | 0.870996385 | 0.795770201 | 0.273964778 |
| `outward_in0_sp0.9` | 1.367989529 | 1.367423255 | 1.358233446 | 1.302814940 | 1.183621805 | 0.377758674 |

These geometry-only widths are identical across a profile’s geometry/FEM/export modes and are independent of the native mesher. Actual remeshed stress peaks may move even for small geometric changes; no favorable resolution or peak is selected to replace another profile’s failing result.

## Matched full native Gate S and point-profile exports

Every legal profile runs complete native `certification::measure` with exactly its own evaluated genes and physical law. Actual exported facets determine sharpness/apex; full native point, bore, census and positive volume are retained. Original export sampling h=.05 mm and maxarea=.25 mm² are fixed. Passing values from different profiles are never pooled.

| Profile | Half-angle deg | Apex mm | Gate S | Point | Bore mm | Census | Volume mm³ | Full export |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `outward_in0_sp0` | 24.324224296 | 0.195144636 | True | True | 1.600000000 | [0, 0, 1] | 30764.031033614 | True |
| `outward_in0_sp0.3` | 24.211856857 | 0.195140515 | True | True | 1.600000000 | [0, 0, 1] | 30814.819726742 | True |
| `outward_in0_sp0.6` | 24.106336838 | 0.195141727 | True | True | 1.600000000 | [0, 0, 1] | 30864.114706358 | True |
| `outward_in0_sp0.9` | 23.800291946 | 0.195143062 | True | True | 1.600000000 | [0, 0, 1] | 30920.866531777 | True |

## Joint margins and mechanism

For continuity with Phase-11, joint comparisons use sequence>=1.20, spine>=1, lateral>=260 N, fatigue>=1, ringback>=1, HM body twist>=1 and global HM twist>=1. The native fatigue law is unchanged and corresponds to lateral capacity about 357.9963 N for fatigue 1; a 260 N comparison alone cannot establish fatigue pass.

| Profile | Fine-three seq | Spine | Lat N | Fatigue | Ringback | HM body | HM global | Fine-three joint | All-five joint |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `outward_in0_sp0` | 1.250619062 | 1.007135462 | 337.700838841 | 0.943308213 | 4.992111565 | 1.021661607 | 0.416720640 | False | False |
| `outward_in0_sp0.3` | 1.110779141 | 0.970383886 | 372.128958319 | 1.039477141 | 4.944562955 | 1.087742839 | 0.404390387 | False | False |
| `outward_in0_sp0.6` | 1.144896190 | 1.055551282 | 388.743647245 | 1.085887368 | 4.924846522 | 1.366495697 | 0.433074064 | False | False |
| `outward_in0_sp0.9` | 1.070193645 | 0.995021961 | 360.778229190 | 1.007770865 | 5.001950526 | 1.377772455 | 0.431106525 | False | False |

Best minimum-normalized-margin fine-three representative: `outward_in0_sp0.6`. This is a measured comparison, not a passing design claim. Every native global/body/annulus peak position, radius, unit stress, original mean element thickness and correction factor is retained in the raw five-area receipt and `fem_summary.csv`. Global lateral blade peak lies on the actual pocket floor in 0/20 observations; protected bands and full-stock neck remain unchanged.

## Full SHA-256 digests, verification and reproduction

Verification status **passed**: 80 unchanged production digests; 17 frozen assignment/source/control/input digests; native outward-normal sign and original CP bounds; exact requested/actual clamping and unchanged other genes; all native canonical flags/rejections; every legal five-mesh and matched full export; native original masks/peaks/metric/torque identities; protected bands/neck; actual mesh/thickness hashes; exact same-genome mesh hashes across saturation aliases; per-mesh baseline differences and geometric chord repeatability. All 34 invocations use the same executable and CPUs 10,11 / jobs=2 / faer / effective niceness 19. Workflow Python/shell syntax checks pass. Native dependency digests and all prior seals are reverified at final sealing.

| Artifact | Full SHA-256 |
| --- | --- |
| study executable | 0041acb118af65a70d6c3a63f774e9732e1c2490910c32febba4dee9f4f70dd7 |
| source_provenance.json | 0c89591a40538ab07ded7c8e35ded1607dfc98f746fe0570b8074bae947886ce |
| verification.json | b6dca963b2377e865ddfb24fd3c021bccb27abbd17dc5c0369ec305f3308f7dc |
| parent.csv | b8336e4bb08bf7aabb0abf4b2e474daf8ee8027ba20b7861ac745cf722567f24 |
| assignment snapshot | e70f5d77aec24195801a710f935cfea4f9992acead7fb4616a8450a503f4a010 |

| Legal profile FEM receipt | Full SHA-256 | Export receipt full SHA-256 |
| --- | --- | --- |
| outward_in0_sp0 | 8f3ae639160ffa01d3faed3854ac9e9a89b892b74bb77be158f2645d930c0222 | 87a2d6ca2adf71dfc15694202510c2c2efef6b3087239b54bf204a875e2549d9 |
| outward_in0_sp0.3 | dd87940e2ca1e12745d899260fdeba08c26ff583262288f1a1df14ee9718b27d | 7b6be793d1918af13e093add2e970437d13f1e46a18d8f63d3cceaeda9680cb1 |
| outward_in0_sp0.6 | 41b2bd6cc5ac9a1d5438e627845462f712f6425dc94d7394d8dd090b89e538c8 | bb939a3febcef9840d62c389f1ee0d94e954b01e68d2d15ebea38041fcc49465 |
| outward_in0_sp0.9 | 805c93f4e6078b333ddd2af9242af7bf2ce65389966f4cc6e76d4ee3a71ee660 | 0424527bcc6a9bc9e2e2be90b76bca588c1975c17e73313a309d5df3dee40dc1 |

`SHA256SUMS` binds every stable artifact, both report copies, executable, frozen inputs, scripts, raw receipts/logs, actual mesh-content hashes, geometric width observations, full input saturation/alias catalog, `fem_summary.csv`, `summary.json`, `verification.json` and `run_provenance.json`. `REPORT_SHA256` records the final report’s full digest; the same digest and final artifact count are in both agent trackers, avoiding a self-referential report hash. Production-file/input/dependency full digests are retained in the provenance catalogs. `verify_seal.py` independently verifies every manifest entry.

Primary report: `experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V9_REPORT.md`; byte-identical local mirror: `target/gate-feasibility-bound/UNIFIED_PROTOTYPE_V9_REPORT.md`. Raw evidence and scripts: `target/gate-feasibility-bound/unified_study_v9/`. Phase-11 V8 report and every prior artifact remain untouched. No external mirror was requested for this task.

```bash
nice -n 10 taskset -c 10,11 env CARGO_BUILD_JOBS=2 JARAMBIT_SOLVER=faer \
  bash target/gate-feasibility-bound/unified_study_v9/reproduce.sh
```

No production edit, git add/commit/push, PR, stacking, branch/worktree change or deletion. Both tracking files are updated at milestones. Await captain_jarambit before further scientific work; the captain will commit from a writable shell.
