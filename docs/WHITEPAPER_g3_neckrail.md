# jarambit v0.1.0 — Design White Paper: SWITCHBACK g3_neckrail (engineering provisional)

**Jepson Taylor** — 2026-10-12. Release tag `v0.1.0-g3neckrail`. Status: **engineering provisional, research only — not a fabrication release.** No manufacturer contact; any fabrication needs explicit human approval. SWITCHBACK next steps are coupons/FA only; this release adds **no new FEM**.

## 1. Summary
`g3_neckrail` is a Ti-6Al-4V, 6.0 mm stock, double-edged, ring-retained hiking/utility karambit from the wave25 graft trial (g3 = wave24 c02 base + g3 neck + rail genes). It holds the best **solid tip SF × SPG = 6.340** at **84.59 g**, beating the prior provisional wave24 c02 (6.214 @ 85.62 g) and passing every wave25 HARD gate.

| Pareto role | design | mass g | SPG | solid tip SF | solid×SPG | status |
|---|---|---|---|---|---|---|
| **SPG / tip×SPG king** | g3_neckrail (w25 graft) | 84.59 | 4.158 | 1.525 | **6.340** | HARD PASS, provisional |
| mass king | a0_c02fit_ref (w26) | 81.37 | 3.656 | 1.526 | 5.579 | seq FAIL (soft), not promoted |
| prior provisional | wave24 c02 / g0_c02 | 85.62 | 4.069 | 1.527 | 6.214 | superseded |

## 2. Genotype (crescent C.4/C.6 spline genotype, 129 columns)
40 outline control points `cp00..cp39` plus feature genes. Key values for g3_neckrail:
- Stock/grind: `t_stock=6.0`, `t_tip=2.796`, `taper_x0=135.05`, `edge_angle_in=21.94°`, `edge_angle_sp=22.0°`, `edge_t=0.9`.
- Ring/neck: `ring_id=25.466`, `ring_wall=5.739`, `neck_fillet=17.76`, `neck_l=9.897`, `rail_w=3.177`, `rflute_d=1.642`, `rflute_w=0.639`, `rflute_arc=235.9°`.
- Lightening: `pocket_d=0.016`, `pocket_r=3.81`, `relief_d=0.065`, `lattice_d=0.204` (handle/blade off), `perf_d=4.84` (1 perforation, 20.5 mm²), `fuller_d=0.235`, windows 0.
- Serrations: large 1.353, small 0.642, fine 0.779 mm depth. Holder: `holder_wall=1.3`, `nub_engage=0.445`. Brand pad 6.07×12.09 mm.
- Material Ti-6Al-4V. Genome: `docs/figures/release_v0.1.0/g3_neckrail_genome.csv`.

## 3. Method
1. **Phenotype**: spline outline + ring hole → planform raster with per-element thickness (grind law below, pockets/grooves/perfs).
2. **D1 screen** (`crescent-d1`): plate FEM on the thickness raster with 3D stress concentration `kc_3d`, 828-joint floors, strict 0.0625 lateral, ring, pry, twist, aesthetics (Hausdorff/EMD vs C.6 anchor), holder envelope, blade-length floors.
3. **Solid tip submodel (gate of record)**: tetrahedral 3D FEM of the tip region (R_CUT 30 mm, PATCH 2.0 mm, 4 layers) under 1.82 N·m tip twist; SF from nodal-max, nodal-avg, element-mean peaks vs. joint allowable 730.2 MPa.
4. **Ranking**: solid tip SF (max) × SPG; mass ≤ 86 g HARD; seq ≥ 1.2 SOFT (reported only).

## 4. Gates (wave25 HARD) and g3 result
| gate | requirement | g3_neckrail |
|---|---|---|
| aesthetics | aes_pass, spine continuity, H ≤ 6 mm | PASS (H 1.82, EMD 0.0065) |
| blade length floor (all three) | ≥ 3.375 in | min 3.445 in (tip-heel 97.02, reach 87.49, arc 101.14/101.26 mm) |
| grip | 60–80 mm | 61.0 |
| ring bore (measured) | 25.4 ± 0.5, ≥ 25.4 measured | 25.46 |
| ring | ≥ 1500 N, SF ≥ 2 | 17464 N rob. (SF 11.6) |
| lateral (cert / strict) | ≥ 260 N | 351.7 N (SF 1.35) / 340.8 N |
| body twist | ≥ 1 | 4.14 |
| solid tip SF @ 1.82 N·m | ≥ 1.0 | **1.525 max / 1.610 avg / 1.762 elem** |
| holder | ≤ 201 × 101 mm | 200.9 × 100.4 |
| mass | ≤ 86 g | 84.59 g (blank 104.16 g) |
| seq (soft) | ≥ 1.2 | 0.959 (lateral-governed) — open |

## 5. FEM results (g3)
- Plate screen: σ/N·mm 0.292, peak at (187.7, −49.1) 4.1 mm from tip, t_local 2.97 mm, kc_3d 1.193 → σ 634 MPa, SF_raw 1.374 / SF_d1 1.152; plate÷solid −2.5 %.
- Solid: 92,448 tets, 422,955 DOF, 759 s; peaks nodal-avg 453.5 MPa, nodal-max 478.8 MPa, elem-mean 414.5 MPa at ~4.0 mm from tip.
- Stress profile (MPa, plate/solid) vs distance from tip: 4–5 mm 531/454; 5–6 356/332; 6–8 268/261; 8–10 204/197; 10–12 170/169; 12–15 154/152; 15–20 149/150; 20–25 124/118. Plate and solid agree within 5 % past 6 mm.
- Twist submodel: tip SF 1.152 (plate D1), tip-zone 1.36, body 4.14.

## 6. Wave26 findings (no promote)
Deterministic grafts of g3 neck/rail genes onto w25 c02_fit/c01_fit/c04/c06, a g3→c02_fit lightening line, and pocket-only controls (r1–r4):
- Every arm keeps solid tip SF ≈ 1.525 — **tip is untouched by the grafts**.
- ≤ 82 g arms top out at 5.579 (a0_c02fit_ref, 81.37 g); no arm reaches ≥ 6.0 at ≤ 82 g.
- Neck graft on c02_fit **lowers** lateral (SPG 3.602 vs 3.656); full rails restore SPG (3.814 @ 82.40 g).
- Pocket-only lightening can't reach 82 g at SPG ≥ 3.93; stock 5.8 on g3 gives 82.13 g, SPG 3.950 (short of the 3.934 @ ≤ 82 g needed for 6.0).
- All wave26 arms fail seq (soft). Pareto: (81.32, 5.493) … (83.07, 6.082). **Nothing beats g3.**

## 7. Tip-family ceiling
Across w24–w26 the solid tip SF sits at 1.520–1.527 for the whole c02 family. The peak is fixed by tip geometry 4 mm from the point (t_local ≈ 2.97 mm, grind law), not by neck/rail/pocket genes. So any further gain in tip×SPG has to come from SPG (mass/lateral), or from changing the tip itself (`t_tip`, taper, edge angle). That is what wave27 (tip mutate, in flight) is testing. Treat 6.34 as the ceiling of the current tip family.

## 8. Limitations
FEM is uncalibrated against coupons; SWITCHBACK coupon/FA work is pending. Seq < 1.2 is open. There is no B-rep STEP of g3 yet (§CAD in README): the grind law is analytic. Use is hiking/utility only.
