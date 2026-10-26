# WAVE 15 | fitness -1.286 / SPG 3.525 N/g | 71.6 g | 5.7 mm stock  [wave15_c01]  OVERALL FAIL

OVERALL: 🔴 **FAIL** — NOT PROMOTED (this tool never promotes; human aesthetic sign-off by Jepson is required for any promotion).  
Generated 2026-10-11 15:02 MT (21:02Z) by `ops/wave_reports/make_wave_report.py`. Rules: CURRENT D1 (`D1-P1/2026-10-11: 828MPa-provisional JOINT(thin(t-0.1)^2 x hot sy100/sy20); lateral 260N (hiker 180 / hunter 260, aim 286); pry 260N (no floor); ring pull 1500N; chord torsion 1.82N*m tip-only (2mm patch, peak>=4mm, St-Venant 3D) + body(>15mm) + tip-submodel(4-15mm @0.0625); cert min(0.25,0.125mm2) no bias + strict 0.0625mm2`). Solver: `exec/d1@f3f360d+dirty(P1 diff sha256:6f6faff933cb) target_d1 built 2026-10-11T20:53:39Z`.  
Source run: islands 8 x pop 48, gens 240, migrate every 6, threads 48, seed 1515, FEM search area 2 mm^2 (search bias lat/ring (1.11, 1.08), JARAMBIT_SEARCH_BIAS_* override), certification min(0.25, 0.125 mm^2) no bias, T_MAX 6.0, mass target Some(72.0) g (JARAMBIT_MASS_CAP; target only, every hard item unchanged), seeds 45 (from out/seeds_w15.csv + C.6), aesthetic anchor SWITCHBACK Rev C.6 (C.5 jimping R0.8/0.45 + C.6 two-sided ring-face groove 0.6/face, 1.6 mm rims), machining kick 0.7, min through-features 0 (JARAMBIT_MIN_THROUGH)  
Source run history last written 2026-10-11 14:32 MT (20:32Z).  
Headline design: **wave15_c01** (best D1 standing in the wave: fewest FAILs, then highest governing joint lateral, then lightest). Badge legend: 🟢 PASS · 🔴 FAIL · ⚪ NOT VERIFIED (missing data is never green).

## KPI dashboard (headline design)

| KPI | value | gate | status |
|---|---|---|---|
| Joint lateral (cert / strict 0.0625 / governing) | 252.6 / 243.9 / **243.9 N** | floor 260 (aim 286 → NOT MET) | 🔴 **FAIL** |
| Hiker (cert joint) | 252.6 N (SF 1.403) | ≥180 | 🟢 **PASS** |
| Tip-only twist SF @1.82 N·m | 0.469 (cap 854 N·mm, peak 4.1 mm from tip, 3D k 1.173) | ≥1.0 | 🔴 **FAIL** |
| Body+tip twist SF @1.82 N·m | min(body 1.926, tip-sub strict 0.494) = **0.494** | ≥1.0 | 🔴 **FAIL** |
| Ring pull (joint) @1500 N | 11885 N (SF 7.92) | ≥1500 | 🟢 **PASS** |
| Hand/blade yield-onset ratio | 0.893 (lateral) | ≥1.2 | 🔴 **FAIL** |
| Aesthetic A / H (C.6 floor, automated) | A 0.746 / H 1.325 mm / EMD 0.0036 | C.6 floor | 🟢 **PASS** |
| Aesthetic sign-off (human) | — | Jepson | ⚪ **NOT VERIFIED** |
| Mass bare / carry | 71.65 g / NOT VERIFIED | band 74–77 · cap <80 | band 🔴 **FAIL** · cap 🟢 **PASS** |
| Blade all-three (tip-heel / reach / arc) | 3.814 / 3.449 / 3.987 in | ≥3.375 in | 🟢 **PASS** |
| Hard gates / red-team | rejects [] | none | 🟢 **PASS** |
| Stock / joint t / fitness / SPG | 5.7 mm / 5.70 mm / -1.2857 / 3.5251 N/g | — | info |

## FEM visuals

*The FEM-metrics plate for this wave was removed in v3.0.2; it is in git history at tag v3.0.1.*

Panels: von Mises for lateral 260 N, chord torsion 1.82 N·m (tip-only + body peaks), ring pull 1500 N; SF envelope (worst case); body+tip twist SF; tip zoom (tip-only); thickness field; gate table. Map SF is σy(100 °C)/σvm at nominal thickness (no thin factor) and is for locating peaks; the gate SFs are the certified JOINT scalars.

| case | peak σvm (MPa) | map SF | location (x, y mm) | region |
|---|---|---|---|---|
| lateral 260 N | 721 | 1.01 | (12.5, -3.7) | ring / ring-neck |
| ring pull 1500 N | 84 | 8.63 | (0.9, -12.8) | ring / ring-neck |
| twist tip-only 1.82 N*m | 1537 | 0.47 | (188.6, -48.8) | tip zone (2.1 mm from apex) |
| twist body >15 mm (field locator) | 292 | - | (181.5, -35.8) | blade body |
| worst SF (envelope, 100 C) | 1537 | 0.47 | (188.6, -48.8) | tip zone (2.1 mm from apex) |

## Break-point summary

- **Failed (headline wave15_c01):** joint_lateral_260 (cert AND strict), twist_tip_only_SF>=1 @1.82, twist_body+tip_SF>=1 @1.82, hand/blade_ratio>=1.2, mass_band_74-77
- **Not verified (headline):** aesthetic_signoff (human)
- **Near misses (within 5 % above a floor):** none
- **Gate failures across all 9 D1-screened designs:** joint_lateral_260 (cert AND strict) 9/9, twist_tip_only_SF>=1 @1.82 9/9, twist_body+tip_SF>=1 @1.82 9/9, hand/blade_ratio>=1.2 9/9, mass_band_74-77 9/9
- **Strength delta vs prior wave** (wave14_c03 (D1 rules, prior wave pack)):
  - Δmass_g: -5.301
  - Δjoint_lat_cert: -27.600
  - Δjoint_lat_strict: -25.900
  - Δjoint_lat_governing: -25.900
  - Δsf_hunter_cert: -0.107
  - Δtwist_tip_only_sf: -0.064
  - Δtwist_body_plus_tip_sf: -0.087
  - Δring_joint_N: -1028.800
  - Δhand_blade_ratio: +0.056
  - Δaes_A: +0.004

## Weight ledger (headline design)

| where | g removed |
|---|---|
| rails (P1 full-thickness rails/neck; nothing removed by design) | n/a (P0 layout, no rail genes) |
| pockets (relief + lattice + isogrid + texture) | 4.32 |
| through-perfs/windows (vs unperforated blank) | 0.00 |
| ring (ring-face flute) | 1.99 |
| grind (fuller + flutes) | 0.23 |
| grind/taper/edge bevel residual (blank - finished - itemised machining) | 16.59 |
| **blank (5.7 mm plate outline) → finished** | 94.79 → **71.65 g** |

C.6 ring-face racetrack groove volume is not itemised in the plate JSON → NOT VERIFIED as a separate line (it sits inside the residual). Machined volumes are the phenotype raster ledger (`removed_mm3`: relief, lattice, fuller, flutes, texture, isogrid, ring flute).

Δ vs prior wave headline (g, + = more removed):  
rails NV, pockets +2.54, through-perfs/windows -0.73, ring -0.40, grind +0.19, grind/taper/edge bevel residual +0.02
  
against ΔSF: hunter(cert) -0.107, tip-only twist -0.064, body+tip twist -0.087, governing lat -25.9 N, Δmass -5.30 g

## All D1-screened designs in this wave

| design | mass g | t | joint lat cert/strict N | tip-only twist | body+tip twist | ring N | hand/blade | band 74–77 | OVERALL |
|---|---|---|---|---|---|---|---|---|---|
| wave15_c01 ★ | 71.65 | 5.7 | 252.6 / 243.9 | 0.47 | 0.49 | 11885 | 0.89 | FAIL | 🔴 **FAIL** |
| wave15_c02 | 71.82 | 5.7 | 228.5 / 242.0 | 0.47 | 0.50 | 12422 | 0.97 | FAIL | 🔴 **FAIL** |
| wave15_c03 | 71.51 | 5.6 | 241.6 / 233.9 | 0.48 | 0.48 | 12500 | 0.90 | FAIL | 🔴 **FAIL** |
| wave15_c04 | 71.12 | 5.6 | 237.5 / 226.0 | 0.47 | 0.47 | 11905 | 0.83 | FAIL | 🔴 **FAIL** |
| wave15_c05 | 71.96 | 5.6 | 245.6 / 240.6 | 0.50 | 0.48 | 12481 | 0.80 | FAIL | 🔴 **FAIL** |
| wave15_c06 | 71.93 | 5.6 | 241.3 / 236.5 | 0.51 | 0.52 | 12079 | 0.88 | FAIL | 🔴 **FAIL** |
| wave15_c07 | 71.62 | 5.6 | 252.6 / 222.8 | 0.50 | 0.51 | 11971 | 0.95 | FAIL | 🔴 **FAIL** |
| wave15_c08 | 71.99 | 5.6 | 243.2 / 239.4 | 0.45 | 0.50 | 12229 | 0.81 | FAIL | 🔴 **FAIL** |
| wave15_c09 | 70.25 | 5.6 | 237.5 / 227.0 | 0.48 | 0.48 | 11661 | 0.81 | FAIL | 🔴 **FAIL** |
## Paper data SAVE

- CSV: `wave15_paper_data.csv` (one row per D1-screened design) · JSON: `wave15_paper_data.json` (full pack: gates, peaks, ledger, deltas, provenance).
- genome_hash `sha256:98ed13f7fc3c3409` (4-decimal (pre-D1 CSV; see D1 P0 reproducibility caveat)) · geometry_hash `sha256:948ec142dafb8c1b` · solver_commit `exec/d1@f3f360d+dirty(P1 diff sha256:6f6faff933cb) target_d1 built 2026-10-11T20:53:39Z`
- mesh: search 2.0 mm² · cert min(0.25, 0.125) mm² · strict 0.0625 mm² · plate export 24920 tris / 13369 nodes (certified min(0.25, 0.125))
- capacities by mesh: search (pre-D1) 275.4 N · cert pre-D1 880/min 278.1 N · cert D1 joint 252.6 N · strict D1 joint 243.9 N · 0.25-only / 0.125-only NOT VERIFIED (not exported separately)
- joint thin+hot: t_joint 5.70 mm, factor lat_rob/lat20 = 0.8804
- final_edge: grind-law JSON: wave15_c01_grind_law.json; grind-law JSON exported; analytic B-rep grind STEP = STUB; faceted STEP != machining release → ⚪ NOT VERIFIED as a machining release
- novelty: 0.0973 (source-run archive (pre-D1 k-NN))
- unqualified regions: 4 mm tip-load patch excluded from peaks (St-Venant zone) - not qualified; clamped-cut tip submodel = STUB (tip-sub SF is a refined local check of the 4-15 mm annulus, not a cut model); ring peel = STUB (no ring-peel case); fatigue = STUB (static screening only); pry 260 N computed but has NO floor; 828 MPa yield provisional until lot MTR; flat Kirchhoff plate (DKT+T6) with per-element thickness; 3D only via St-Venant factor on twist
- fields NOT VERIFIED in at least one design row of this wave's CSV: aesthetic_signoff, cap_lat_0p125_only_N, cap_lat_0p25_only_N, mass_carry_g
- headline fields NOT VERIFIED: cap_lat_0p25_only_N, cap_lat_0p125_only_N, mass_carry_g, aesthetic_signoff
- timestamps: generated 2026-10-11 15:02 MT (21:02Z); headline source written 2026-10-11 14:56 MT (20:56Z)

## Gates context
Joint thin+hot 828 MPa; lateral ≥260 (aim 286); mass 74–77 g, cap <80; C.6 aesthetic; blade ≥3.375 in all three; hand/blade ≥1.2; BOTH twist modes @1.82 N·m. **No promotion.** Aesthetic sign-off still required.

