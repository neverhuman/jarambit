# Jarambit Gate Feasibility Verdict (`docs/FEASIBILITY_VERDICT_J3.md`)

**Authors:** `captain_jarambit` (`2740538.captain_jarambit`) & `1242467.codex_jarambit_gate_diag`
**Authority:**
1. `/home/ubuntu/xbabe2_kicker/captains-20261017/gb_jarambit_release_1120.txt` (`J3. FEASIBILITY VERDICT`)
2. `/home/ubuntu/xbabe2_kicker/captains-20261017/JEPSON_RULING_20261018T1736Z_jarambit_release_a.md` (`Option (a)` + `11:36 AM MDT Weighted-Objectives Addendum`)

---

## 1. Plain Verdict (`J3`)

> **Verdict:** **"Not jointly reachable in the current 128-gene parameterization family without changing a frozen threshold/bound or replacing hard pass/fail vetoes with weighted objectives."**

1. **Individual Reachability (`26 / 26` Gates Individually Reachable in Legal Genotype Bounds):**
   Every single one of the 16 `W_CAD_v2` engineering gates (`G16-01`..`G16-16`) and 10 `RS-karambit-v2` hard-gate groups (`RSV2-01`..`RSV2-10`) is individually reachable within the frozen legal genotype bounds (`crates/crescent/src/gates.rs`, stock thickness $T \in [5.3, 6.0]\text{ mm}$), as demonstrated by committed `main` wave records (`w298`–`w318`, `w400c3`–`w408t3`) and bounded structural witness studies (`placement_05`, `local_01`, and `Q7` in PRs #106, #107, #108, #112, and #113).
2. **Joint Infeasibility Under Hard Conjunction (`0` All-26 Qualified Candidates Across 32 Completed Waves / 142 Stage-2 Assessments):**
   No candidate in the 128-gene family satisfies all 26 hard gates simultaneously (`qualified_count = 0`, `champion = null`). The hard conjunction is blocked by two coupled geometric-structural trade-offs under the $T \le 6.0\text{ mm}$ stock ceiling and C.6 Hausdorff envelope ($H \le 6.0\text{ mm}$).
3. **Resolution Implemented Under Jepson Ruling `2026-10-18T17:36Z` Addendum:**
   Per Jepson's `11:36 AM MDT Oct 18` ruling (`JEPSON_RULING_20261018T1736Z_jarambit_release_a.md`), the 26 hard pass/fail vetoes in selection are replaced by the **`W_ULTRALIGHT_v1` global weighted fitness** (`[0.0, 100.0]`) tailored for an **Ultralight Running Knife**, while every raw per-gate metric, its legacy threshold, and its signed deficit/margin remain logged on every candidate.

---

## 2. Best Value Reached in Legal Genotype Bounds for the 12 Deficit Gates (`J3`)

| Gate Task ID(s) | Gate Key & Metric | Legacy Hard Threshold | Wave Baseline (`w298`–`w318`, `w401c3`–`w408t3`) | Best Value Reached in Legal Bounds (`main` Committed Evidence) | Individual Reachability |
|---|---|---:|---:|---|---|
| `G16-05` | `cert_hiker_180n` (`lat_rob_cert_n`) | $\ge 180.0\text{ N}$ | $170.2\text{--}179.4\text{ N}$ | **`312.4 N`** (`SF = 1.736`, `placement_05`, PR #107 `a4d8953147`, `PHASE7.md` SHA-256 `18c3ef60...`) | **PASS** (Individually reachable) |
| `G16-06`, `RSV2-02` | `cert_hunter_260n` / `floors_828_joint` (`lat_rob_cert_n`) | $\ge 260.0\text{ N}$ | $170.2\text{--}179.4\text{ N}$ | **`312.4 N`** (`SF = 1.202`, `placement_05`, PR #107 `a4d8953147`) | **PASS** (Individually reachable) |
| `G16-07`, `RSV2-05` | `strict_hunter_260n` / `strict_hunter_lateral` (`strict_lat_rob_0p0625_n`) | $\ge 260.0\text{ N}$ | $180.38\text{ N}$ (`w313_r02_p14_c26`); $189.49\text{ N}$ (`SF = 0.7288`, `w408c3_r03_p10_c30`) | **`284.6 N`** (`SF = 1.095`, `placement_05`, PR #107 `a4d8953147`) | **PASS** (Individually reachable) |
| `G16-10` | `twist_tip_only_1p82nm` (`twist_tip_sf` at $1.82\text{ N}\cdot\text{m}$) | $\ge 1.000$ | $0.53\text{--}0.58$ | **`1.112`** (`Q7` distal-reinforced witness, PR #106 `90f77f3f04` / PR #108 `657a1bc892`) | **PASS** (When $T_{\text{tip}} \ge 4.0\text{ mm}$) |
| `G16-11`, `RSV2-03` | `twist_tip_submodel_strict_1p82nm` / `twist_floor_body_and_tipsub_strict` (`twist_tipzone_sf_strict`) | $\ge 1.000$ | $0.53\text{--}0.58$ | **`1.084`** (`Q7` distal-reinforced witness, PR #106 `90f77f3f04`) | **PASS** (When $T_{\text{tip}} \ge 4.0\text{ mm}$) |
| `G16-12`, `RSV2-04` | `solid_tip_3d_p2_1p82nm` / `solid_tip_sf` (`solid_tip_sf_nodal_max` 3D $P_2$ TET10) | $\ge 1.000$ | $0.7163$ (`w309_r02_p14_c35`); $0.6849$ (`w406c3_r03_p10_c15`) | **`2.346`** (`DOLFINx = 2.3465`, `rel_err = 0.0213%`, `placement_05`) & **`2.824`** (`DOLFINx = 2.8232`, `rel_err = 0.0283%`, `Q7`, PR #106 `90f77f3f04`) | **PASS** (When $T_{\text{tip}} \ge 4.0\text{ mm}$) |
| `RSV2-07` | `human_max` (`hm_twist_sf` at $16.2\text{ N}\cdot\text{m}$, `hm_spine_sf` at $3427\text{ N}$, `hm_ringback_sf` at $3000\text{ N}$, `ring_edge_r_mm`) | $\text{SF} \ge 1.000$, $r_{\text{edge}} \ge 1.50\text{ mm}$ | `hm_twist_sf = 0.06..0.11`, `hm_spine_sf = 0.22..0.43`, $r_{\text{edge}} = 1.125\text{ mm}$ | **`hm_twist_sf = 1.148`**, **`hm_spine_sf = 1.045`**, **`hm_ringback_sf = 1.653`**, **`r_edge = 1.50 mm`** (`Q7`, PR #106 `90f77f3f04`, `PHASE6_UNIFIED_SCORECARD.md` SHA-256 `a425b66d...`) | **PASS** (Individually reachable at $T = 6.0\text{ mm}$, $w_{\text{ring}} \ge 6.0\text{ mm}$) |
| `RSV2-08` | `fatigue` (`l7_goodman_sf`, $0\dots 180\text{ N}$ cyclic lateral, $10^7$ cycles) | $\ge 1.000$ | $0.46\text{--}0.55$ ($0.5092$ on `w406t3_r02_p10_c23`) | **`1.1149`** (`placement_05`) & **`1.0231`** (`local_01`, PR #107 `a4d8953147`, `PHASE7.md` SHA-256 `18c3ef60...`) | **PASS** (Individually reachable) |

---

## 3. Conflicting Gate Pairs Preventing Joint All-26 Hard-Gate Qualification (`J3`)

### Conflict Pair A: `RSV2-10` (`gate_s_tip_point_brep`) + `RSV2-09` (`gate_s_sharp_law`) vs. `G16-10..12` / `RSV2-03..04` ($1.82\text{ N}\cdot\text{m}$ 4–15 mm Tip Torsion) & `G16-07` / `RSV2-05` ($260\text{ N}$ Strict Hunter Lateral)
- **Mechanism:**
  - `RSV2-10` enforces exported STL bisector thickness at $0.25, 1.0, 2.0\text{ mm}$ aft of the tip point to be $\le 0.30, 0.60, 1.00\text{ mm}$ with strict monotone non-decreasing thickness over $0\text{--}10\text{ mm}$ and $95\%$-of-core taper length $\ge 5.0\text{ mm}$.
  - `RSV2-09` enforces cutting-edge apex thickness $\le 0.20\text{ mm}$ and local bevel half-angle $\le 25.0^\circ$ over the first $0.4\text{ mm}$ normal to both inside and spine edges.
  - Under the exported double-bevel wedge cross-section (`crates/crescent/src/export.rs`), bounding the point thickness to $\le 1.00\text{ mm}$ at $2.0\text{ mm}$ aft and the bevel half-angle to $\le 25^\circ$ restricts the torsional St. Venant polar constant $J(r)$ and section modulus $Z(r)$ across the $4 \le r_{\text{tip}} < 15\text{ mm}$ tip-submodel ROI.
  - Consequently, genomes that pass `RSV2-09` and `RSV2-10` on the exported STL (`w309`–`w321`) top out at `twist_tipzone_sf_strict` $\approx 0.58$ and `solid_tip_sf_nodal_max` $\approx 0.72$, whereas genomes that thicken the distal core ($T_{\text{tip}} \ge 4.0\text{ mm}$, e.g. `placement_05` and `Q7`) pass 3D solid-tip torsion (`SF = 2.35..2.82`) and lateral (`284..312 N`) but exceed the `RSV2-10` $1.00\text{ mm}$ thickness ceiling at $2.0\text{ mm}$ aft or the `RSV2-09` $25.0^\circ$ 3D facet bevel angle.

### Conflict Pair B: `RSV2-07` (`human_max` $16.2\text{ N}\cdot\text{m}$ Twist / $3427\text{ N}$ Spine) vs. `G16-14` / `RSV2-06` (`seq_ratio_cert >= 1.20` Blade-Before-Hand Yield Sequence), `G16-04` (`H <= 6.0 mm`), & Ultralight Mass
- **Mechanism:**
  - `RSV2-07` applies $16.2\text{ N}\cdot\text{m}$ chord torque ($8.90\times$ the $1.82\text{ N}\cdot\text{m}$ certification torque) and $3427\text{ N}$ spine load, requiring maximum stock thickness ($T = 6.0\text{ mm}$) and broad blade chord sections (`Q7` mass $> 110\text{ g}$).
  - Increasing blade chord stiffness to survive $16.2\text{ N}\cdot\text{m}$ raises the blade first-yield load $C_{\text{blade}}$ relative to the finger-ring first-yield load $C_{\text{hand}}$, driving the blade-before-hand safety ratio $\text{seq}_{\text{cert}} = C_{\text{hand}} / C_{\text{blade}}$ below the $1.20$ floor (`G16-14` / `RSV2-06`) unless the ring wall is expanded beyond the C.6 Hausdorff boundary ($H \le 6.0\text{ mm}$, `G16-04`).
  - Furthermore, a $> 110\text{ g}$ heavy-stock prybar directly contradicts the primary mission profile of an **Ultralight Running Knife**.
