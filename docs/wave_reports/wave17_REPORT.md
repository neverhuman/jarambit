# WAVE 17 | fitness -0.567 / SPG 3.802 N/g | 77.0 g | 5.9 mm stock  [wave17_c07]  OVERALL FAIL

OVERALL: 🔴 **FAIL** — NOT PROMOTED (this tool never promotes; human aesthetic sign-off by Jepson is required for any promotion).  
Generated 2026-10-11 16:36 MT (22:36Z) by `ops/wave_reports/make_wave_report.py`. Rules: CURRENT D1 (`D1-P1/2026-10-11: 828MPa-provisional JOINT(thin(t-0.1)^2 x hot sy100/sy20); lateral 260N (hiker 180 / hunter 260, aim 286); pry 260N (no floor); ring pull 1500N; chord torsion 1.82N*m tip-only (2mm patch, peak>=4mm, St-Venant 3D) + body(>15mm) + tip-submodel(4-15mm @0.0625); cert min(0.25,0.125mm2) no bias + strict 0.0625mm2`). Solver: `wave17 binary sha256:c6cc1be21698 on xbabe3 (exec/d1 P1 snapshot); run EXIT 0`.  
Source run: islands 8 x pop 48, gens 160, migrate every 6, threads 40, seed 1717, FEM search area 2 mm^2 (search bias lat/ring (1.11, 1.08), JARAMBIT_SEARCH_BIAS_* override), certification min(0.25, 0.125 mm^2) no bias, T_MAX 6.0, mass target Some(77.0) g (JARAMBIT_MASS_CAP; target only, every hard item unchanged), seeds 3 (from out/wave17_p1/wave14_c03_genome.csv,out/wave17_p1/wave13_c01_genome.csv + C.6), aesthetic anchor SWITCHBACK Rev C.6 (C.5 jimping R0.8/0.45 + C.6 two-sided ring-face groove 0.6/face, 1.6 mm rims), machining kick 0.5, min through-features 0 (JARAMBIT_MIN_THROUGH)  
Source run history last written 2026-10-11 16:30 MT (22:30Z).  
Headline design: **wave17_c07** (best D1 standing in the wave: fewest FAILs, then highest governing joint lateral, then lightest). Badge legend: 🟢 PASS · 🔴 FAIL · ⚪ NOT VERIFIED (missing data is never green).

## KPI dashboard (headline design)

| KPI | value | gate | status |
|---|---|---|---|
| Joint lateral (cert / strict 0.0625 / governing) | 292.6 / 281.1 / **281.1 N** | floor 260 (aim 286 → NOT MET) | 🟢 **PASS** |
| Hiker (cert joint) | 292.6 N (SF 1.626) | ≥180 | 🟢 **PASS** |
| Tip-only twist SF @1.82 N·m | 0.581 (cap 1057 N·mm, peak 4.1 mm from tip, 3D k NOT VERIFIED) | ≥1.0 | 🔴 **FAIL** |
| Body+tip twist SF @1.82 N·m | min(body 2.632, tip-sub strict 0.591) = **0.591** | ≥1.0 | 🔴 **FAIL** |
| Ring pull (joint) @1500 N | 16865 N (SF 11.24) | ≥1500 | 🟢 **PASS** |
| Hand/blade yield-onset ratio | 0.985 (lateral) | ≥1.2 | 🔴 **FAIL** |
| Aesthetic A / H (C.6 floor, automated) | A 0.546 / H 2.689 mm / EMD 0.0079 | C.6 floor | 🟢 **PASS** |
| Aesthetic sign-off (human) | — | Jepson | ⚪ **NOT VERIFIED** |
| Mass bare / carry | 76.97 g / NOT VERIFIED | band 74–77 · cap <80 | band 🟢 **PASS** · cap 🟢 **PASS** |
| Blade all-three (tip-heel / reach / arc) | 3.806 / 3.442 / 3.987 in | ≥3.375 in | 🟢 **PASS** |
| Hard gates / red-team | rejects [] | none | 🟢 **PASS** |
| Stock / joint t / fitness / SPG | 5.9 mm / 5.54 mm / -0.5669 / 3.8019 N/g | — | info |

## FEM visuals

*The FEM-metrics plate for this wave was removed in v3.0.2; it is in git history at tag v3.0.1.*

Panels: von Mises for lateral 260 N, chord torsion 1.82 N·m (tip-only + body peaks), ring pull 1500 N; SF envelope (worst case); body+tip twist SF; tip zoom (tip-only); thickness field; gate table. Map SF is σy(100 °C)/σvm at nominal thickness (no thin factor) and is for locating peaks; the gate SFs are the certified JOINT scalars.

| case | peak σvm (MPa) | map SF | location (x, y mm) | region |
|---|---|---|---|---|
| lateral 260 N | 638 | 1.14 | (166.1, -24.0) | blade body |
| ring pull 1500 N | 59 | 12.38 | (1.6, -12.6) | ring / ring-neck |
| twist tip-only 1.82 N*m | 1190 | 0.61 | (187.6, -49.0) | tip zone (3.3 mm from apex) |
| twist body >15 mm (field locator) | 202 | - | (180.8, -36.5) | blade body |
| worst SF (envelope, 100 C) | 1190 | 0.61 | (187.6, -49.0) | tip zone (3.3 mm from apex) |

## Break-point summary

- **Failed (headline wave17_c07):** twist_tip_only_SF>=1 @1.82, twist_body+tip_SF>=1 @1.82, hand/blade_ratio>=1.2
- **Not verified (headline):** aesthetic_signoff (human)
- **Near misses (within 5 % above a floor):** wave17_c08 joint lat strict 265.000 vs 260
- **Gate failures across all 16 D1-screened designs:** twist_tip_only_SF>=1 @1.82 16/16, twist_body+tip_SF>=1 @1.82 16/16, hand/blade_ratio>=1.2 16/16, mass_band_74-77 9/16, joint_lateral_260 (cert AND strict) 2/16
- **Strength delta vs prior wave** (wave16_c01 (D1 rules, prior wave pack)):
  - Δmass_g: +5.315
  - Δjoint_lat_cert: +29.900
  - Δjoint_lat_strict: +34.600
  - Δjoint_lat_governing: +34.600
  - Δsf_hunter_cert: +0.116
  - Δtwist_tip_only_sf: +0.047
  - Δtwist_body_plus_tip_sf: +0.030
  - Δring_joint_N: +4110.600
  - Δhand_blade_ratio: +0.145
  - Δaes_A: -0.063

## Weight ledger (headline design)

| where | g removed |
|---|---|
| rails (P1 full-thickness rails/neck; nothing removed by design) | 0.00 |
| pockets (relief + lattice + isogrid + texture) | 0.07 |
| through-perfs/windows (vs unperforated blank) | 0.00 |
| ring (ring-face flute) | 0.00 |
| grind (fuller + flutes) | 0.00 |
| grind/taper/edge bevel residual (blank - finished - itemised machining) | 20.15 |
| **blank (5.9 mm plate outline) → finished** | 97.19 → **76.97 g** |

C.6 ring-face racetrack groove volume is not itemised in the plate JSON → NOT VERIFIED as a separate line (it sits inside the residual). Machined volumes are the phenotype raster ledger (`removed_mm3`: relief, lattice, fuller, flutes, texture, isogrid, ring flute).

Δ vs prior wave headline (g, + = more removed):  
rails NV, pockets -3.34, through-perfs/windows +0.00, ring -2.46, grind -0.37, grind/taper/edge bevel residual +4.72
  
against ΔSF: hunter(cert) +0.116, tip-only twist +0.047, body+tip twist +0.030, governing lat +34.6 N, Δmass +5.31 g

## All D1-screened designs in this wave

| design | mass g | t | joint lat cert/strict N | tip-only twist | body+tip twist | ring N | hand/blade | band 74–77 | OVERALL |
|---|---|---|---|---|---|---|---|---|---|
| wave17_c01 | 77.00 | 5.9 | 295.3 / 280.0 | 0.58 | 0.59 | 16866 | 0.99 | PASS | 🔴 **FAIL** |
| wave17_c02 | 77.00 | 5.9 | 295.3 / 280.0 | 0.58 | 0.59 | 16866 | 0.99 | PASS | 🔴 **FAIL** |
| wave17_c03 | 77.00 | 5.9 | 295.3 / 280.0 | 0.58 | 0.59 | 16866 | 0.99 | PASS | 🔴 **FAIL** |
| wave17_c04 | 77.00 | 5.9 | 295.3 / 280.0 | 0.58 | 0.59 | 16866 | 0.99 | PASS | 🔴 **FAIL** |
| wave17_c05 | 77.00 | 5.9 | 295.3 / 280.0 | 0.58 | 0.59 | 16866 | 0.99 | PASS | 🔴 **FAIL** |
| wave17_c06 | 76.99 | 5.9 | 295.3 / 280.1 | 0.58 | 0.59 | 16866 | 0.99 | PASS | 🔴 **FAIL** |
| wave17_c07 ★ | 76.97 | 5.9 | 292.6 / 281.1 | 0.58 | 0.59 | 16865 | 0.98 | PASS | 🔴 **FAIL** |
| wave17_c08 | 77.02 | 5.9 | 295.6 / 265.0 | 0.58 | 0.59 | 16879 | 0.99 | FAIL | 🔴 **FAIL** |
| wave17_c09 | 77.01 | 5.9 | 292.7 / 281.2 | 0.58 | 0.59 | 16891 | 0.99 | FAIL | 🔴 **FAIL** |
| wave17_c10 | 77.01 | 5.9 | 292.7 / 281.2 | 0.58 | 0.59 | 16891 | 0.99 | FAIL | 🔴 **FAIL** |
| wave17_c11 | 77.01 | 5.9 | 292.7 / 281.2 | 0.58 | 0.59 | 16891 | 0.99 | FAIL | 🔴 **FAIL** |
| wave17_c12 | 77.01 | 5.9 | 292.7 / 281.2 | 0.58 | 0.59 | 16891 | 0.99 | FAIL | 🔴 **FAIL** |
| wave17_c13 | 77.01 | 5.9 | 292.7 / 281.2 | 0.58 | 0.59 | 16891 | 0.99 | FAIL | 🔴 **FAIL** |
| wave17_c14 | 77.01 | 5.9 | 292.7 / 281.2 | 0.58 | 0.59 | 16891 | 0.99 | FAIL | 🔴 **FAIL** |
| wave17_c15 | 77.03 | 5.9 | 283.7 / 254.5 | 0.58 | 0.59 | 16855 | 0.95 | FAIL | 🔴 **FAIL** |
| wave17_c16 | 77.03 | 5.9 | 283.7 / 254.5 | 0.58 | 0.59 | 16855 | 0.95 | FAIL | 🔴 **FAIL** |
## Paper data SAVE

- CSV: `wave17_paper_data.csv` (one row per D1-screened design) · JSON: `wave17_paper_data.json` (full pack: gates, peaks, ledger, deltas, provenance).
- genome_hash `sha256:948ac505bb8c3750` (full-precision) · geometry_hash `sha256:2c05847f41cad131` · solver_commit `wave17 binary sha256:c6cc1be21698 on xbabe3 (exec/d1 P1 snapshot); run EXIT 0`
- mesh: search 2.0 mm² · cert min(0.25, 0.125) mm² · strict 0.0625 mm² · plate export 24789 tris / 13305 nodes (certified min(0.25, 0.125))
- capacities by mesh: search (pre-D1) 262.1 N · cert pre-D1 880/min 292.6 N · cert D1 joint 292.6 N · strict D1 joint 281.1 N · 0.25-only / 0.125-only NOT VERIFIED (not exported separately)
- joint thin+hot: t_joint 5.54 mm, factor lat_rob/lat20 = 0.8792
- final_edge: NOT VERIFIED (no grind-law JSON found); grind-law JSON exported; analytic B-rep grind STEP = STUB; faceted STEP != machining release → ⚪ NOT VERIFIED as a machining release
- novelty: 0.0011 (source-run archive (pre-D1 k-NN))
- unqualified regions: 4 mm tip-load patch excluded from peaks (St-Venant zone) - not qualified; clamped-cut tip submodel = STUB (tip-sub SF is a refined local check of the 4-15 mm annulus, not a cut model); ring peel = STUB (no ring-peel case); fatigue = STUB (static screening only); pry 260 N computed but has NO floor; 828 MPa yield provisional until lot MTR; flat Kirchhoff plate (DKT+T6) with per-element thickness; 3D only via St-Venant factor on twist
- fields NOT VERIFIED in at least one design row of this wave's CSV: aesthetic_signoff, cap_lat_0p125_only_N, cap_lat_0p25_only_N, mass_carry_g, twist_kc_3d
- headline fields NOT VERIFIED: cap_lat_0p25_only_N, cap_lat_0p125_only_N, twist_kc_3d, mass_carry_g, aesthetic_signoff
- timestamps: generated 2026-10-11 16:36 MT (22:36Z); headline source written 2026-10-11 16:33 MT (22:33Z)

## Gates context
Joint thin+hot 828 MPa; lateral ≥260 (aim 286); mass 74–77 g, cap <80; C.6 aesthetic; blade ≥3.375 in all three; hand/blade ≥1.2; BOTH twist modes @1.82 N·m. **No promotion.** Aesthetic sign-off still required.

