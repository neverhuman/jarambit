# WAVE 16 | fitness 2.232 / SPG 3.666 N/g | 71.7 g | 5.8 mm stock  [wave16_c01]  OVERALL FAIL

OVERALL: 🔴 **FAIL** — NOT PROMOTED (this tool never promotes; human aesthetic sign-off by Jepson is required for any promotion).  
Generated 2026-10-11 15:02 MT (21:02Z) by `ops/wave_reports/make_wave_report.py`. Rules: CURRENT D1 (`D1-P1/2026-10-11: 828MPa-provisional JOINT(thin(t-0.1)^2 x hot sy100/sy20); lateral 260N (hiker 180 / hunter 260, aim 286); pry 260N (no floor); ring pull 1500N; chord torsion 1.82N*m tip-only (2mm patch, peak>=4mm, St-Venant 3D) + body(>15mm) + tip-submodel(4-15mm @0.0625); cert min(0.25,0.125mm2) no bias + strict 0.0625mm2`). Solver: `exec/d1@f3f360d+dirty(P1 diff sha256:6f6faff933cb) target_d1 built 2026-10-11T20:53:39Z`.  
Source run: islands 8 x pop 48, gens 240, migrate every 6, threads 40, seed 1616, FEM search area 2 mm^2 (search bias lat/ring (1.29, 1.07), JARAMBIT_SEARCH_BIAS_* override), certification min(0.25, 0.125 mm^2) no bias, T_MAX 6.0, mass target Some(72.0) g (JARAMBIT_MASS_CAP; target only, every hard item unchanged), seeds 57 (from out/seeds_w16.csv + C.6), aesthetic anchor SWITCHBACK Rev C.6 (C.5 jimping R0.8/0.45 + C.6 two-sided ring-face groove 0.6/face, 1.6 mm rims), machining kick 0.7, min through-features 4 (JARAMBIT_MIN_THROUGH)  
Source run history last written 2026-10-11 14:32 MT (20:32Z).  
Headline design: **wave16_c01** (best D1 standing in the wave: fewest FAILs, then highest governing joint lateral, then lightest). Badge legend: 🟢 PASS · 🔴 FAIL · ⚪ NOT VERIFIED (missing data is never green).

## KPI dashboard (headline design)

| KPI | value | gate | status |
|---|---|---|---|
| Joint lateral (cert / strict 0.0625 / governing) | 262.7 / 246.5 / **246.5 N** | floor 260 (aim 286 → NOT MET) | 🔴 **FAIL** |
| Hiker (cert joint) | 262.7 N (SF 1.459) | ≥180 | 🟢 **PASS** |
| Tip-only twist SF @1.82 N·m | 0.534 (cap 972 N·mm, peak 4.1 mm from tip, 3D k 1.175) | ≥1.0 | 🔴 **FAIL** |
| Body+tip twist SF @1.82 N·m | min(body 2.659, tip-sub strict 0.561) = **0.561** | ≥1.0 | 🔴 **FAIL** |
| Ring pull (joint) @1500 N | 12754 N (SF 8.50) | ≥1500 | 🟢 **PASS** |
| Hand/blade yield-onset ratio | 0.840 (lateral) | ≥1.2 | 🔴 **FAIL** |
| Aesthetic A / H (C.6 floor, automated) | A 0.609 / H 2.250 mm / EMD 0.0061 | C.6 floor | 🟢 **PASS** |
| Aesthetic sign-off (human) | — | Jepson | ⚪ **NOT VERIFIED** |
| Mass bare / carry | 71.66 g / NOT VERIFIED | band 74–77 · cap <80 | band 🔴 **FAIL** · cap 🟢 **PASS** |
| Blade all-three (tip-heel / reach / arc) | 3.817 / 3.421 / 3.996 in | ≥3.375 in | 🟢 **PASS** |
| Hard gates / red-team | rejects [] | none | 🟢 **PASS** |
| Stock / joint t / fitness / SPG | 5.8 mm / 5.80 mm / 2.2319 / 3.6661 N/g | — | info |

## FEM visuals

*The FEM-metrics plate for this wave was removed in v3.0.2; it is in git history at tag v3.0.1.*

Panels: von Mises for lateral 260 N, chord torsion 1.82 N·m (tip-only + body peaks), ring pull 1500 N; SF envelope (worst case); body+tip twist SF; tip zoom (tip-only); thickness field; gate table. Map SF is σy(100 °C)/σvm at nominal thickness (no thin factor) and is for locating peaks; the gate SFs are the certified JOINT scalars.

| case | peak σvm (MPa) | map SF | location (x, y mm) | region |
|---|---|---|---|---|
| lateral 260 N | 711 | 1.03 | (12.8, -2.3) | ring / ring-neck |
| ring pull 1500 N | 80 | 9.09 | (0.3, -12.8) | ring / ring-neck |
| twist tip-only 1.82 N*m | 1366 | 0.53 | (187.5, -50.2) | tip zone (2.7 mm from apex) |
| twist body >15 mm (field locator) | 219 | - | (180.4, -37.9) | blade body |
| worst SF (envelope, 100 C) | 1366 | 0.53 | (187.5, -50.2) | tip zone (2.7 mm from apex) |

## Break-point summary

- **Failed (headline wave16_c01):** joint_lateral_260 (cert AND strict), twist_tip_only_SF>=1 @1.82, twist_body+tip_SF>=1 @1.82, hand/blade_ratio>=1.2, mass_band_74-77
- **Not verified (headline):** aesthetic_signoff (human)
- **Near misses (within 5 % above a floor):** wave16_c01 joint lat cert 262.700 vs 260; wave16_c02 joint lat cert 262.700 vs 260; wave16_c03 joint lat cert 262.700 vs 260; wave16_c04 joint lat cert 262.700 vs 260; wave16_c05 joint lat cert 262.700 vs 260; wave16_c06 joint lat cert 262.700 vs 260; wave16_c07 joint lat cert 262.700 vs 260; wave16_c08 joint lat cert 262.700 vs 260; wave16_c09 joint lat cert 262.700 vs 260; wave16_c10 joint lat cert 262.700 vs 260; wave16_c11 joint lat cert 262.700 vs 260; wave16_c12 joint lat cert 262.700 vs 260; wave16_c13 joint lat cert 262.700 vs 260; wave16_c14 joint lat cert 262.700 vs 260; wave16_c15 joint lat cert 262.700 vs 260; wave16_c16 joint lat cert 262.700 vs 260
- **Gate failures across all 16 D1-screened designs:** joint_lateral_260 (cert AND strict) 16/16, twist_tip_only_SF>=1 @1.82 16/16, twist_body+tip_SF>=1 @1.82 16/16, hand/blade_ratio>=1.2 16/16, mass_band_74-77 16/16
- **Strength delta vs prior wave** (wave15_c01 (D1 rules, prior wave pack)):
  - Δmass_g: +0.008
  - Δjoint_lat_cert: +10.100
  - Δjoint_lat_strict: +2.600
  - Δjoint_lat_governing: +2.600
  - Δsf_hunter_cert: +0.039
  - Δtwist_tip_only_sf: +0.065
  - Δtwist_body_plus_tip_sf: +0.067
  - Δring_joint_N: +869.700
  - Δhand_blade_ratio: -0.053
  - Δaes_A: -0.137

## Weight ledger (headline design)

| where | g removed |
|---|---|
| rails (P1 full-thickness rails/neck; nothing removed by design) | n/a (P0 layout, no rail genes) |
| pockets (relief + lattice + isogrid + texture) | 3.40 |
| through-perfs/windows (vs unperforated blank) | 0.00 |
| ring (ring-face flute) | 2.46 |
| grind (fuller + flutes) | 0.37 |
| grind/taper/edge bevel residual (blank - finished - itemised machining) | 15.44 |
| **blank (5.8 mm plate outline) → finished** | 93.33 → **71.66 g** |

C.6 ring-face racetrack groove volume is not itemised in the plate JSON → NOT VERIFIED as a separate line (it sits inside the residual). Machined volumes are the phenotype raster ledger (`removed_mm3`: relief, lattice, fuller, flutes, texture, isogrid, ring flute).

Δ vs prior wave headline (g, + = more removed):  
rails NV, pockets -0.92, through-perfs/windows +0.00, ring +0.46, grind +0.14, grind/taper/edge bevel residual -1.16
  
against ΔSF: hunter(cert) +0.039, tip-only twist +0.065, body+tip twist +0.067, governing lat +2.6 N, Δmass +0.01 g

## All D1-screened designs in this wave

| design | mass g | t | joint lat cert/strict N | tip-only twist | body+tip twist | ring N | hand/blade | band 74–77 | OVERALL |
|---|---|---|---|---|---|---|---|---|---|
| wave16_c01 ★ | 71.66 | 5.8 | 262.7 / 246.5 | 0.53 | 0.56 | 12754 | 0.84 | FAIL | 🔴 **FAIL** |
| wave16_c02 | 71.66 | 5.8 | 262.7 / 246.5 | 0.53 | 0.56 | 12754 | 0.84 | FAIL | 🔴 **FAIL** |
| wave16_c03 | 71.66 | 5.8 | 262.7 / 246.5 | 0.53 | 0.56 | 12754 | 0.84 | FAIL | 🔴 **FAIL** |
| wave16_c04 | 71.66 | 5.8 | 262.7 / 246.5 | 0.53 | 0.56 | 12754 | 0.84 | FAIL | 🔴 **FAIL** |
| wave16_c05 | 71.66 | 5.8 | 262.7 / 246.5 | 0.53 | 0.56 | 12754 | 0.84 | FAIL | 🔴 **FAIL** |
| wave16_c06 | 71.66 | 5.8 | 262.7 / 246.5 | 0.53 | 0.56 | 12754 | 0.84 | FAIL | 🔴 **FAIL** |
| wave16_c07 | 71.70 | 5.8 | 262.7 / 246.5 | 0.53 | 0.56 | 12754 | 0.85 | FAIL | 🔴 **FAIL** |
| wave16_c08 | 71.70 | 5.8 | 262.7 / 246.5 | 0.53 | 0.56 | 12754 | 0.84 | FAIL | 🔴 **FAIL** |
| wave16_c09 | 71.70 | 5.8 | 262.7 / 246.5 | 0.53 | 0.56 | 12754 | 0.84 | FAIL | 🔴 **FAIL** |
| wave16_c10 | 71.70 | 5.8 | 262.7 / 246.5 | 0.53 | 0.56 | 12754 | 0.83 | FAIL | 🔴 **FAIL** |
| wave16_c11 | 71.70 | 5.8 | 262.7 / 246.5 | 0.53 | 0.56 | 12754 | 0.85 | FAIL | 🔴 **FAIL** |
| wave16_c12 | 71.70 | 5.8 | 262.7 / 246.5 | 0.53 | 0.56 | 12754 | 0.84 | FAIL | 🔴 **FAIL** |
| wave16_c13 | 71.71 | 5.8 | 262.7 / 246.5 | 0.53 | 0.56 | 12754 | 0.84 | FAIL | 🔴 **FAIL** |
| wave16_c14 | 71.72 | 5.8 | 262.7 / 246.5 | 0.53 | 0.56 | 12754 | 0.83 | FAIL | 🔴 **FAIL** |
| wave16_c15 | 71.72 | 5.8 | 262.7 / 246.5 | 0.53 | 0.56 | 12754 | 0.86 | FAIL | 🔴 **FAIL** |
| wave16_c16 | 71.73 | 5.8 | 262.7 / 246.5 | 0.53 | 0.56 | 12754 | 0.83 | FAIL | 🔴 **FAIL** |
## Paper data SAVE

- CSV: `wave16_paper_data.csv` (one row per D1-screened design) · JSON: `wave16_paper_data.json` (full pack: gates, peaks, ledger, deltas, provenance).
- genome_hash `sha256:0bc0040cdd8f2e95` (4-decimal (pre-D1 CSV; see D1 P0 reproducibility caveat)) · geometry_hash `sha256:683f70313353fcd0` · solver_commit `exec/d1@f3f360d+dirty(P1 diff sha256:6f6faff933cb) target_d1 built 2026-10-11T20:53:39Z`
- mesh: search 2.0 mm² · cert min(0.25, 0.125) mm² · strict 0.0625 mm² · plate export 24172 tris / 12989 nodes (certified min(0.25, 0.125))
- capacities by mesh: search (pre-D1) 271.6 N · cert pre-D1 880/min 289.1 N · cert D1 joint 262.7 N · strict D1 joint 246.5 N · 0.25-only / 0.125-only NOT VERIFIED (not exported separately)
- joint thin+hot: t_joint 5.80 mm, factor lat_rob/lat20 = 0.8807
- final_edge: grind-law JSON: wave16_c01_grind_law.json; grind-law JSON exported; analytic B-rep grind STEP = STUB; faceted STEP != machining release → ⚪ NOT VERIFIED as a machining release
- novelty: 0.0006 (source-run archive (pre-D1 k-NN))
- unqualified regions: 4 mm tip-load patch excluded from peaks (St-Venant zone) - not qualified; clamped-cut tip submodel = STUB (tip-sub SF is a refined local check of the 4-15 mm annulus, not a cut model); ring peel = STUB (no ring-peel case); fatigue = STUB (static screening only); pry 260 N computed but has NO floor; 828 MPa yield provisional until lot MTR; flat Kirchhoff plate (DKT+T6) with per-element thickness; 3D only via St-Venant factor on twist
- fields NOT VERIFIED in at least one design row of this wave's CSV: aesthetic_signoff, cap_lat_0p125_only_N, cap_lat_0p25_only_N, mass_carry_g
- headline fields NOT VERIFIED: cap_lat_0p25_only_N, cap_lat_0p125_only_N, mass_carry_g, aesthetic_signoff
- timestamps: generated 2026-10-11 15:02 MT (21:02Z); headline source written 2026-10-11 14:56 MT (20:56Z)

## Gates context
Joint thin+hot 828 MPa; lateral ≥260 (aim 286); mass 74–77 g, cap <80; C.6 aesthetic; blade ≥3.375 in all three; hand/blade ≥1.2; BOTH twist modes @1.82 N·m. **No promotion.** Aesthetic sign-off still required.

