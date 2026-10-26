# WAVE 14 | fitness 2.702 / SPG 3.641 N/g | 77.0 g | 5.8 mm stock  [wave14_c03]  OVERALL FAIL

OVERALL: 🔴 **FAIL** — NOT PROMOTED (this tool never promotes; human aesthetic sign-off by Jepson is required for any promotion).  
Generated 2026-10-11 15:02 MT (21:02Z) by `ops/wave_reports/make_wave_report.py`. Rules: CURRENT D1 (`D1-P1/2026-10-11: 828MPa-provisional JOINT(thin(t-0.1)^2 x hot sy100/sy20); lateral 260N (hiker 180 / hunter 260, aim 286); pry 260N (no floor); ring pull 1500N; chord torsion 1.82N*m tip-only (2mm patch, peak>=4mm, St-Venant 3D) + body(>15mm) + tip-submodel(4-15mm @0.0625); cert min(0.25,0.125mm2) no bias + strict 0.0625mm2`). Solver: `exec/d1@f3f360d+dirty(P1 diff sha256:6f6faff933cb) target_d1 built 2026-10-11T20:53:39Z`.  
Source run: islands 8 x pop 48, gens 120, migrate every 5, threads 40, seed 1414, FEM search area 2 mm^2 (search bias lat/ring (1.29, 1.07), JARAMBIT_SEARCH_BIAS_* override), certification min(0.25, 0.125 mm^2) no bias, T_MAX 6.0, mass target Some(77.0) g (JARAMBIT_MASS_CAP; target only, every hard item unchanged), seeds 39 (from out/rescore_c6_repair/c6_pass_genomes.csv + C.6), aesthetic anchor SWITCHBACK Rev C.6 (C.5 jimping R0.8/0.45 + C.6 two-sided ring-face groove 0.6/face, 1.6 mm rims), machining kick 0.6, min through-features 3 (JARAMBIT_MIN_THROUGH)  
Source run history last written 2026-10-11 12:59 MT (18:59Z).  
Headline design: **wave14_c03** (best D1 standing in the wave: fewest FAILs, then highest governing joint lateral, then lightest). Badge legend: 🟢 PASS · 🔴 FAIL · ⚪ NOT VERIFIED (missing data is never green).

## KPI dashboard (headline design)

| KPI | value | gate | status |
|---|---|---|---|
| Joint lateral (cert / strict 0.0625 / governing) | 280.2 / 269.8 / **269.8 N** | floor 260 (aim 286 → NOT MET) | 🟢 **PASS** |
| Hiker (cert joint) | 280.2 N (SF 1.557) | ≥180 | 🟢 **PASS** |
| Tip-only twist SF @1.82 N·m | 0.533 (cap 970 N·mm, peak 4.2 mm from tip, 3D k 1.167) | ≥1.0 | 🔴 **FAIL** |
| Body+tip twist SF @1.82 N·m | min(body 2.777, tip-sub strict 0.581) = **0.581** | ≥1.0 | 🔴 **FAIL** |
| Ring pull (joint) @1500 N | 12914 N (SF 8.61) | ≥1500 | 🟢 **PASS** |
| Hand/blade yield-onset ratio | 0.837 (lateral) | ≥1.2 | 🔴 **FAIL** |
| Aesthetic A / H (C.6 floor, automated) | A 0.742 / H 1.465 mm / EMD 0.0027 | C.6 floor | 🟢 **PASS** |
| Aesthetic sign-off (human) | — | Jepson | ⚪ **NOT VERIFIED** |
| Mass bare / carry | 76.95 g / NOT VERIFIED | band 74–77 · cap <80 | band 🟢 **PASS** · cap 🟢 **PASS** |
| Blade all-three (tip-heel / reach / arc) | 3.807 / 3.444 / 3.987 in | ≥3.375 in | 🟢 **PASS** |
| Hard gates / red-team | rejects [] | none | 🟢 **PASS** |
| Stock / joint t / fitness / SPG | 5.8 mm / 5.80 mm / 2.7024 / 3.6413 N/g | — | info |

## FEM visuals

*The FEM-metrics plate for this wave was removed in v3.0.2; it is in git history at tag v3.0.1.*

Panels: von Mises for lateral 260 N, chord torsion 1.82 N·m (tip-only + body peaks), ring pull 1500 N; SF envelope (worst case); body+tip twist SF; tip zoom (tip-only); thickness field; gate table. Map SF is σy(100 °C)/σvm at nominal thickness (no thin factor) and is for locating peaks; the gate SFs are the certified JOINT scalars.

| case | peak σvm (MPa) | map SF | location (x, y mm) | region |
|---|---|---|---|---|
| lateral 260 N | 652 | 1.12 | (12.8, -2.5) | ring / ring-neck |
| ring pull 1500 N | 77 | 9.44 | (0.9, -12.8) | ring / ring-neck |
| twist tip-only 1.82 N*m | 1368 | 0.53 | (188.0, -48.9) | tip zone (2.9 mm from apex) |
| twist body >15 mm (field locator) | 217 | - | (180.6, -36.6) | blade body |
| worst SF (envelope, 100 C) | 1368 | 0.53 | (188.0, -48.9) | tip zone (2.9 mm from apex) |

## Break-point summary

- **Failed (headline wave14_c03):** twist_tip_only_SF>=1 @1.82, twist_body+tip_SF>=1 @1.82, hand/blade_ratio>=1.2
- **Not verified (headline):** aesthetic_signoff (human)
- **Near misses (within 5 % above a floor):** wave14_c01 joint lat cert 269.200 vs 260; wave14_c01 joint lat strict 260.300 vs 260; wave14_c03 joint lat strict 269.800 vs 260
- **Gate failures across all 2 D1-screened designs:** twist_tip_only_SF>=1 @1.82 2/2, hand/blade_ratio>=1.2 2/2, twist_body+tip_SF>=1 @1.82 1/2
- **Strength delta vs prior wave** (wave13_c01 (D1 rules, prior wave pack)):
  - Δmass_g: +2.579
  - Δjoint_lat_cert: +45.800
  - Δjoint_lat_strict: +21.200
  - Δjoint_lat_governing: +35.400
  - Δsf_hunter_cert: +0.176
  - Δtwist_tip_only_sf: +0.085
  - Δtwist_body_plus_tip_sf: +0.120
  - Δring_joint_N: +393.000
  - Δhand_blade_ratio: -0.001
  - Δaes_A: -0.021

## Weight ledger (headline design)

| where | g removed |
|---|---|
| rails (P1 full-thickness rails/neck; nothing removed by design) | n/a (P0 layout, no rail genes) |
| pockets (relief + lattice + isogrid + texture) | 1.78 |
| through-perfs/windows (vs unperforated blank) | 0.73 |
| ring (ring-face flute) | 2.40 |
| grind (fuller + flutes) | 0.04 |
| grind/taper/edge bevel residual (blank - finished - itemised machining) | 16.57 |
| **blank (5.8 mm plate outline) → finished** | 97.74 → **76.95 g** |

C.6 ring-face racetrack groove volume is not itemised in the plate JSON → NOT VERIFIED as a separate line (it sits inside the residual). Machined volumes are the phenotype raster ledger (`removed_mm3`: relief, lattice, fuller, flutes, texture, isogrid, ring flute).

Δ vs prior wave headline (g, + = more removed):  
rails NV, pockets -2.45, through-perfs/windows -1.14, ring +0.34, grind -0.22, grind/taper/edge bevel residual +0.69
  
against ΔSF: hunter(cert) +0.176, tip-only twist +0.085, body+tip twist +0.120, governing lat +35.4 N, Δmass +2.58 g

## All D1-screened designs in this wave

| design | mass g | t | joint lat cert/strict N | tip-only twist | body+tip twist | ring N | hand/blade | band 74–77 | OVERALL |
|---|---|---|---|---|---|---|---|---|---|
| wave14_c01 | 76.38 | 5.8 | 269.2 / 260.3 | 0.56 | NOT VERIFIED | 14275 | 0.83 | PASS | 🔴 **FAIL** |
| wave14_c03 ★ | 76.95 | 5.8 | 280.2 / 269.8 | 0.53 | 0.58 | 12914 | 0.84 | PASS | 🔴 **FAIL** |

Pre-D1 certified designs in this wave **not** re-screened under D1 (all D1 gates ⚪ NOT VERIFIED): wave14_c02, wave14_c04, wave14_c05, wave14_c06, wave14_c07, wave14_c08, wave14_c09, wave14_c10, wave14_c11, wave14_c12.

## Paper data SAVE

- CSV: `wave14_paper_data.csv` (one row per D1-screened design) · JSON: `wave14_paper_data.json` (full pack: gates, peaks, ledger, deltas, provenance).
- genome_hash `sha256:45ded3edc9976f87` (4-decimal (pre-D1 CSV; see D1 P0 reproducibility caveat)) · geometry_hash `sha256:6422f291174b3c34` · solver_commit `exec/d1@f3f360d+dirty(P1 diff sha256:6f6faff933cb) target_d1 built 2026-10-11T20:53:39Z`
- mesh: search 2.0 mm² · cert min(0.25, 0.125) mm² · strict 0.0625 mm² · plate export 25976 tris / 14017 nodes (certified min(0.25, 0.125))
- capacities by mesh: search (pre-D1) 270.0 N · cert pre-D1 880/min 308.3 N · cert D1 joint 280.2 N · strict D1 joint 269.8 N · 0.25-only / 0.125-only NOT VERIFIED (not exported separately)
- joint thin+hot: t_joint 5.80 mm, factor lat_rob/lat20 = 0.8809
- final_edge: grind-law JSON: wave14_c03_grind_law.json; grind-law JSON exported; analytic B-rep grind STEP = STUB; faceted STEP != machining release → ⚪ NOT VERIFIED as a machining release
- novelty: 0.0807 (source-run archive (pre-D1 k-NN))
- unqualified regions: 4 mm tip-load patch excluded from peaks (St-Venant zone) - not qualified; clamped-cut tip submodel = STUB (tip-sub SF is a refined local check of the 4-15 mm annulus, not a cut model); ring peel = STUB (no ring-peel case); fatigue = STUB (static screening only); pry 260 N computed but has NO floor; 828 MPa yield provisional until lot MTR; flat Kirchhoff plate (DKT+T6) with per-element thickness; 3D only via St-Venant factor on twist
- fields NOT VERIFIED in at least one design row of this wave's CSV: aesthetic_signoff, cap_lat_0p125_only_N, cap_lat_0p25_only_N, genome_hash, genome_precision, geometry_hash, mass_carry_g, plate_mesh_nodes, plate_mesh_tris, pry20_N, source_json, source_mtime, t_joint_mm, twist_body_plus_tip_sf, twist_body_sf, twist_tipsub_sf_strict
- headline fields NOT VERIFIED: cap_lat_0p25_only_N, cap_lat_0p125_only_N, mass_carry_g, aesthetic_signoff
- timestamps: generated 2026-10-11 15:02 MT (21:02Z); headline source written 2026-10-11 14:57 MT (20:57Z)

## Gates context
Joint thin+hot 828 MPa; lateral ≥260 (aim 286); mass 74–77 g, cap <80; C.6 aesthetic; blade ≥3.375 in all three; hand/blade ≥1.2; BOTH twist modes @1.82 N·m. **No promotion.** Aesthetic sign-off still required.

