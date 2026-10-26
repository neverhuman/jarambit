# WAVE 13 | fitness -1.984 / SPG 3.152 N/g | 74.4 g | 5.6 mm stock  [wave13_c01]  OVERALL FAIL

OVERALL: 🔴 **FAIL** — NOT PROMOTED (this tool never promotes; human aesthetic sign-off by Jepson is required for any promotion).  
Generated 2026-10-11 15:02 MT (21:02Z) by `ops/wave_reports/make_wave_report.py`. Rules: CURRENT D1 (`D1-P1/2026-10-11: 828MPa-provisional JOINT(thin(t-0.1)^2 x hot sy100/sy20); lateral 260N (hiker 180 / hunter 260, aim 286); pry 260N (no floor); ring pull 1500N; chord torsion 1.82N*m tip-only (2mm patch, peak>=4mm, St-Venant 3D) + body(>15mm) + tip-submodel(4-15mm @0.0625); cert min(0.25,0.125mm2) no bias + strict 0.0625mm2`). Solver: `exec/d1@f3f360d+dirty(P1 diff sha256:6f6faff933cb) target_d1 built 2026-10-11T20:53:39Z`.  
Source run: islands 8 x pop 48, gens 120, migrate every 5, threads 48, seed 1313, FEM search area 2 mm^2 (search bias lat/ring (1.11, 1.08), JARAMBIT_SEARCH_BIAS_* override), certification min(0.25, 0.125 mm^2) no bias, T_MAX 6.0, mass target Some(76.0) g (JARAMBIT_MASS_CAP; target only, every hard item unchanged), seeds 12 (from out/rescore_c6/c6_pass_genomes.csv + C.6), aesthetic anchor SWITCHBACK Rev C.6 (C.5 jimping R0.8/0.45 + C.6 two-sided ring-face groove 0.6/face, 1.6 mm rims), machining kick 0.6, min through-features 0 (JARAMBIT_MIN_THROUGH)  
Source run history last written 2026-10-11 12:50 MT (18:50Z).  
Headline design: **wave13_c01** (best D1 standing in the wave: fewest FAILs, then highest governing joint lateral, then lightest). Badge legend: 🟢 PASS · 🔴 FAIL · ⚪ NOT VERIFIED (missing data is never green).

## KPI dashboard (headline design)

| KPI | value | gate | status |
|---|---|---|---|
| Joint lateral (cert / strict 0.0625 / governing) | 234.4 / 248.6 / **234.4 N** | floor 260 (aim 286 → NOT MET) | 🔴 **FAIL** |
| Hiker (cert joint) | 234.4 N (SF 1.302) | ≥180 | 🟢 **PASS** |
| Tip-only twist SF @1.82 N·m | 0.448 (cap 816 N·mm, peak 4.1 mm from tip, 3D k 1.153) | ≥1.0 | 🔴 **FAIL** |
| Body+tip twist SF @1.82 N·m | min(body 2.337, tip-sub strict 0.461) = **0.461** | ≥1.0 | 🔴 **FAIL** |
| Ring pull (joint) @1500 N | 12520 N (SF 8.35) | ≥1500 | 🟢 **PASS** |
| Hand/blade yield-onset ratio | 0.838 (lateral) | ≥1.2 | 🔴 **FAIL** |
| Aesthetic A / H (C.6 floor, automated) | A 0.763 / H 1.276 mm / EMD 0.0029 | C.6 floor | 🟢 **PASS** |
| Aesthetic sign-off (human) | — | Jepson | ⚪ **NOT VERIFIED** |
| Mass bare / carry | 74.37 g / NOT VERIFIED | band 74–77 · cap <80 | band 🟢 **PASS** · cap 🟢 **PASS** |
| Blade all-three (tip-heel / reach / arc) | 3.815 / 3.438 / 3.989 in | ≥3.375 in | 🟢 **PASS** |
| Hard gates / red-team | rejects [] | none | 🟢 **PASS** |
| Stock / joint t / fitness / SPG | 5.6 mm / 5.60 mm / -1.9842 / 3.1519 N/g | — | info |

## FEM visuals

*The FEM-metrics plate for this wave was removed in v3.0.2; it is in git history at tag v3.0.1.*

Panels: von Mises for lateral 260 N, chord torsion 1.82 N·m (tip-only + body peaks), ring pull 1500 N; SF envelope (worst case); body+tip twist SF; tip zoom (tip-only); thickness field; gate table. Map SF is σy(100 °C)/σvm at nominal thickness (no thin factor) and is for locating peaks; the gate SFs are the certified JOINT scalars.

| case | peak σvm (MPa) | map SF | location (x, y mm) | region |
|---|---|---|---|---|
| lateral 260 N | 808 | 0.90 | (98.8, 2.6) | blade body |
| ring pull 1500 N | 80 | 9.06 | (-0.3, -12.8) | ring / ring-neck |
| twist tip-only 1.82 N*m | 1527 | 0.48 | (187.6, -49.1) | tip zone (3.1 mm from apex) |
| twist body >15 mm (field locator) | 257 | - | (180.8, -36.6) | blade body |
| worst SF (envelope, 100 C) | 1527 | 0.48 | (187.6, -49.1) | tip zone (3.1 mm from apex) |

## Break-point summary

- **Failed (headline wave13_c01):** joint_lateral_260 (cert AND strict), twist_tip_only_SF>=1 @1.82, twist_body+tip_SF>=1 @1.82, hand/blade_ratio>=1.2
- **Not verified (headline):** aesthetic_signoff (human)
- **Near misses (within 5 % above a floor):** none
- **Gate failures across all 2 D1-screened designs:** joint_lateral_260 (cert AND strict) 2/2, twist_tip_only_SF>=1 @1.82 2/2, hand/blade_ratio>=1.2 2/2, twist_body+tip_SF>=1 @1.82 1/2
- **Strength delta vs prior wave** (prior wave best-fit wave12_c01 — PRE-D1 RULES ONLY (880 MPa, min-factor); D1 delta NOT VERIFIED (prior wave never D1-screened)):
  - Δmass_g: -4.981
  - ΔpreD1_lat_rob: -25.300
  - ΔpreD1_sf_hunter: -0.097
  - ΔpreD1_ring_rob: -418.300

## Weight ledger (headline design)

| where | g removed |
|---|---|
| rails (P1 full-thickness rails/neck; nothing removed by design) | n/a (P0 layout, no rail genes) |
| pockets (relief + lattice + isogrid + texture) | 4.23 |
| through-perfs/windows (vs unperforated blank) | 1.87 |
| ring (ring-face flute) | 2.06 |
| grind (fuller + flutes) | 0.26 |
| grind/taper/edge bevel residual (blank - finished - itemised machining) | 15.88 |
| **blank (5.6 mm plate outline) → finished** | 96.81 → **74.37 g** |

C.6 ring-face racetrack groove volume is not itemised in the plate JSON → NOT VERIFIED as a separate line (it sits inside the residual). Machined volumes are the phenotype raster ledger (`removed_mm3`: relief, lattice, fuller, flutes, texture, isogrid, ring flute).

Δ machined mass vs prior wave best (PRE-D1 records, g, + = more removed): relief +0.09, lattice -0.08, fuller -0.21, flutes -0.08, texture +0.84, isogrid +1.70, ring flute -0.76, perfs +1.32  
against pre-D1 ΔSF hunter -0.097, Δlat_rob -25.3 N, Δmass -4.98 g (D1-rule SF delta NOT VERIFIED).

## All D1-screened designs in this wave

| design | mass g | t | joint lat cert/strict N | tip-only twist | body+tip twist | ring N | hand/blade | band 74–77 | OVERALL |
|---|---|---|---|---|---|---|---|---|---|
| wave13_c01 ★ | 74.37 | 5.6 | 234.4 / 248.6 | 0.45 | 0.46 | 12520 | 0.84 | PASS | 🔴 **FAIL** |
| wave13_c07 | 74.03 | 5.5 | 237.4 / 214.5 | 0.48 | NOT VERIFIED | 12585 | 0.92 | PASS | 🔴 **FAIL** |

Pre-D1 certified designs in this wave **not** re-screened under D1 (all D1 gates ⚪ NOT VERIFIED): wave13_c02, wave13_c03, wave13_c04, wave13_c05, wave13_c06, wave13_c08, wave13_c09, wave13_c10, wave13_c11, wave13_c12.

## Paper data SAVE

- CSV: `wave13_paper_data.csv` (one row per D1-screened design) · JSON: `wave13_paper_data.json` (full pack: gates, peaks, ledger, deltas, provenance).
- genome_hash `sha256:42ce7e8f717775d5` (4-decimal (pre-D1 CSV; see D1 P0 reproducibility caveat)) · geometry_hash `sha256:ed91d8a2430c1942` · solver_commit `exec/d1@f3f360d+dirty(P1 diff sha256:6f6faff933cb) target_d1 built 2026-10-11T20:53:39Z`
- mesh: search 2.0 mm² · cert min(0.25, 0.125) mm² · strict 0.0625 mm² · plate export 26297 tris / 14126 nodes (certified min(0.25, 0.125))
- capacities by mesh: search (pre-D1) 277.8 N · cert pre-D1 880/min 258.3 N · cert D1 joint 234.4 N · strict D1 joint 248.6 N · 0.25-only / 0.125-only NOT VERIFIED (not exported separately)
- joint thin+hot: t_joint 5.60 mm, factor lat_rob/lat20 = 0.8795
- final_edge: grind-law JSON: wave13_c01_grind_law.json; grind-law JSON exported; analytic B-rep grind STEP = STUB; faceted STEP != machining release → ⚪ NOT VERIFIED as a machining release
- novelty: 0.1726 (source-run archive (pre-D1 k-NN))
- unqualified regions: 4 mm tip-load patch excluded from peaks (St-Venant zone) - not qualified; clamped-cut tip submodel = STUB (tip-sub SF is a refined local check of the 4-15 mm annulus, not a cut model); ring peel = STUB (no ring-peel case); fatigue = STUB (static screening only); pry 260 N computed but has NO floor; 828 MPa yield provisional until lot MTR; flat Kirchhoff plate (DKT+T6) with per-element thickness; 3D only via St-Venant factor on twist
- fields NOT VERIFIED in at least one design row of this wave's CSV: aesthetic_signoff, cap_lat_0p125_only_N, cap_lat_0p25_only_N, genome_hash, genome_precision, geometry_hash, mass_carry_g, plate_mesh_nodes, plate_mesh_tris, pry20_N, source_json, source_mtime, t_joint_mm, twist_body_plus_tip_sf, twist_body_sf, twist_tipsub_sf_strict
- headline fields NOT VERIFIED: cap_lat_0p25_only_N, cap_lat_0p125_only_N, mass_carry_g, aesthetic_signoff
- timestamps: generated 2026-10-11 15:02 MT (21:02Z); headline source written 2026-10-11 14:57 MT (20:57Z)

## Gates context
Joint thin+hot 828 MPa; lateral ≥260 (aim 286); mass 74–77 g, cap <80; C.6 aesthetic; blade ≥3.375 in all three; hand/blade ≥1.2; BOTH twist modes @1.82 N·m. **No promotion.** Aesthetic sign-off still required.

