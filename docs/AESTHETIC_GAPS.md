# Crescent lane — aesthetic floor, novelty QD, constraint changes and open gaps

**Author:** Jepson Taylor · **Lane:** `crates/crescent` (branch `exec/crescent`) · **Date:** 2026-10-11
**Scope:** hiking / utility / runner karambit only. No combat framing. The pipeline does not contact manufacturers; any fabrication needs human approval.

## 1. Why a new lane
The jarambit genotype (20–23 genes: annular blade sector, straight grip rails, shoulders, V detent, one ring hole) produces
blocky "stick-figure" planforms. wave5 and the prior blocky Pareto are **not promoted**. The crescent lane starts from the
SWITCHBACK C.4 reference silhouette and only lets the search move *inside an aesthetic envelope* around it, while 3-D
features (thickness field, lattice, medial-axis relief, flutes) supply novelty.

## 2. Constraints switched OFF for this lane (the stick-figure genotype is not used)
| # | OFF in the crescent lane | Replaced by |
|---|---|---|
| 1 | Straight grip rails + annular-sector blade (`phenotype` decoder) | 40-point C.4 interpolating spline; each control point moves along its normal within a per-zone ±Δ |
| 2 | Genus-1 "exactly one hole = ring" | Lightening holes allowed **only** in C.4 zones (comet slot, eye, cairn); ligaments ≥ 3 mm, hole-to-hole ≥ 3 mm |
| 3 | Shoulder step + V detent holder interface | C.4 cairn-hole retention nub + friction land x ∈ [78, 100] + dx (land ≥ 40 mm², stack/insertion/draw forces gated) |
| 4 | Stock 2.5–4.5 mm | 5.3–6.0 mm finished from 6.35 mm plate (`JARAMBIT_T_MAX`, default 6.0), 0.1 mm quantum |
| 5 | `CURVE_RADIUS`, sweep ≥ 55°, blade-angle genes | Continuous-crescent shape tests (§3) |
| 6 | Horizontal reach ⟂ grip axis (jarambit convention) | C.4 convention: reach along the handle axis (same 3.375 in floor) |
| 7 | Holder envelope measured in the grip-axis frame | Best oriented box over −30…30° (**OWNER decision**, see §5) |
| 8 | Spine sharpening from the crest | Spine sharpening starts ≥ ~23.2 mm aft (gene `spine_sharp_offset`, seed 24) so the spine edge arc stays ≤ 4.0 in |
| 9 | Uniform-thickness plate FEM | Grind-aware per-element thickness FEM (stricter; bevels, relief and flutes are modelled) |
| 10 | Jarambit mesh bias 1.070 / 1.062 | Crescent-measured search bias 1.11 / 1.08 + certification at 0.25 & 0.125 mm² (§4) |

Unchanged and firm: blade ≥ 3.375 in (86.225 mm design floor) on tip-to-heel, reach and sharpened arc; each edge ≤ 4.0 in;
grip 60–80 mm; ring ID 25.4 ± 0.5 mm; double-edged and both edges serrated; titanium only; edge ≥ 14°/side; flat brand pad;
connected single solid; watertight STEP; floors **180 / 260 / 1500 N** on the robust certified FEM level.

## 3. New constraint set (crescent lane)
**HARD aesthetic floor vs C.4** (`crescent::aesthetic`):
- Hausdorff(exterior, C.4 exterior) ≤ 6 mm.
- Curvature-histogram EMD ≤ 0.06 (32 bins of log-signed κ along the arc-length-resampled outline).
- Longest straight run (R > 250 mm) ≤ C.4 + 8 mm → *no straight stem*.
- Minimum concave blend radius ≥ min(2 mm, 0.8 × C.4) → smooth ring→handle→belly→hawkbill blends; ring neck fillet ≥ 8 mm.
- Spine convex fraction (from cp30) and inside-edge concave fraction ≥ C.4 − 0.03 → continuous crescent, continuously concave hawkbill.
- Thumb seat depth ≥ 0.4 mm, guard-lobe prominence ≥ 5 mm, hook drop ≥ 25 mm.
- Grip texture present (machined texture or cord-wrap zone); brand pad on a protected flat.
- Score A = exp(−H/6)·exp(−EMD/0.05)·straight penalty (reported, multiplies fitness).

**HARD manufacturability gates** (`crescent::gates`): blade all-three ≥ 86.225 mm, each edge ≤ 101.6 mm; grip 60–80 mm; ring ID ±0.5;
double serrated; Ti-6Al-4V / Ti-3Al-2.5V; stock 5.3–6.0 mm @0.1 mm; edge ≥ 14°/side; no self-intersection; ligaments ≥ 3 mm
(ring wall ≥ 4.5 mm); hole spacing ≥ 3 mm; web floor max(1.5, 0.3 t) under every pocket; sheath pop-in (cairn nub engage,
holder wall, land ≥ 40 mm², insertion / draw windows); holder envelope 100 × 200 mm (oriented); brand pad clear; mass sanity;
STEP solid watertight (0 boundary / 0 non-manifold edges) on export.

**HARD floors:** robust lateral ≥ 180 N (hiker) and ≥ 260 N (hunter), robust ring pull ≥ 1500 N; robust = min(20 °C at −0.1 mm stock, 100 °C yield).

**SOFT novelty:** k = 5 nearest-neighbour distance in a 12-D descriptor (relief / lattice / fuller / flute volume, texture,
lightening fraction, four silhouette-zone offsets, thickness CV, blade drop) computed **among aesthetic passers only**.
MAP-Elites archive cell = (3-D relief fraction bin {0.5, 1, 1.5, 2, 3, 4.5, 6.5 %}, Hausdorff bin of 1 mm).

**Fitness** = SPG × A × (1 + novelty) for designs passing every hard item; graded negatives otherwise (aesthetic fail < gate fail < floor fail).
Elites are seeded from C.4; mutation operators: control-point walk, thickness-field, lattice, medial-axis relief, feature toggles.

## 4. FEM calibration for the crescent (measured, not assumed)
`jarambit crescent-conv` (out/crescent_conv/convergence.md): 25 distinct aesthetic passers, 5 mesh levels (2 → 0.125 mm²).
Search level (2 mm²) over the 0.125 mm² reference: lateral max **1.105**, ring max **1.073**; 0.25 / 0.125: lateral ≤ 1.030, ring ≤ 1.046.
The jarambit bias 1.070 was therefore **unconservative** for ground crescent tips (first uncertified smoke: a design scored
282.7 N robust at 2 mm² but 230 N at 0.75 mm²). Fix: search divides by 1.11 / 1.08, and every plated/promoted design is
**certified** = min over the 0.25 and 0.125 mm² solutions with no bias. The first uncertified smoke
(`out/crescent_smoke_v0_uncertified`) is superseded and not reported.

## 5. Open gaps / owner decisions
1. **Holder envelope (OWNER):** C.4 itself is 209.5 × 73 mm in the grip-axis frame and 200.5 × 100.2 mm at its best orientation
   (−8.5°) — 0.5 mm over the frozen 100 × 200 mm. The lane evaluates the oriented box; winners sit at 198.8 × 99.2 mm @ −9°.
   Either the sheath is drawn on that diagonal or the envelope is re-confirmed.
2. **Twist proxy SF < 1 (advisory):** 1.26 N·m tip twist has no floor; at 100 °C the top design's twist SF is 0.93 at the
   ground tip (5 mm from the point). A twist floor or tip-thickness rule is an owner call.
3. **Mass:** the 5.3–6.0 mm stock default puts finished mass at ~85–88 g (C.4 at 3.175 mm is 48.3 g). Lightness comes from
   SPG; a thinner-stock lane would need the floors re-met with more section height.
4. **Novelty is still small:** first smoke novelty 0.05–0.16, Hausdorff 1–4 mm, 3-D relief ≤ 4.5 % of blank. Longer runs and
   archive-parent pressure are needed to reach genuinely "alien" relief inside the floor.
5. **Grip texture:** current winners pick cord-wrap mode (0 mm³ machined); machined texture is in the genome but not yet selected.
6. **C.5 jimping** adoption is a separate item and not blocking (the jimping-scale gene stays near C.4).
7. Linear-elastic first-yield only; no fatigue, fracture, residual stress from grinding/waterjet, or sheath shock.

## 7. Rev C.6 adoption (2026-10-11, owner Jepson) — ring-face racetrack groove
* **Anchor:** SWITCHBACK Rev **C.6** = C.5 (outline, R0.8 / 0.45 mm jimping, holes, flutes, stock) + a **two-sided ring-face
  racetrack groove**: annulus concentric with the finger-ring bore, r = r_bore + 1.6 … r_OD − 1.6 mm (C.6 nominal 14.3–17.4 mm),
  **0.6 mm deep per face** (web t − 1.2 mm), R0.5 floor fillet (B-rep only). Source: `jarambit_ref/hiking-karambit/revC6_outline.json`,
  `switchback_revC6_{blank_taper,ground,flatground_noserrbevel}.step` (ground STEP 46.93 g Gr5, valid).
* **Implementation:** `crates/crescent/src/c6_ref.rs` (re-exports C.5 + groove constants); `pheno.rs` cuts the groove into the
  0.25 mm thickness raster on both faces wherever the solid covers the annulus (not on grind cells), so FEM, mass and the
  watertight STEP/STL all carry it. The genome is unchanged (the groove follows the ring-wall genes); `JARAMBIT_ANCHOR=C5`
  rebuilds the C.5 phenotype for A/B studies.
* **New HARD aesthetic item:** the C.6 groove must be present with radial width ≥ 2.0 mm (Ø2 mm cutter) — i.e. ring wall
  ≥ 5.2 mm with the fixed 1.6 mm rims. Hausdorff / EMD / continuity are still measured against the C.5 = C.6 exterior.
* **Consequence (measured, out/rescore_c6):** 29 of 42 prior certified/legal crescents had thinned the ring wall below 5.2 mm
  to save mass and now FAIL the C.6 look (they are not re-promoted). 11 of 42 pass C.6 aesthetic + all gates + certified
  floors + < 80 g; the best (wave8_c01 under C.6) gets lighter **and** keeps its certified lateral (75.55 g, SPG 4.03 N/g).
* **Patentable callout (document only; NO prior-art search performed):** *dual-face ring contrast-anodize channels on a
  crescent karambit* — a machined, recessed annular channel on BOTH faces of the finger ring, concentric with the bore and
  bounded by full-thickness rims at the bore and at the ring OD, sized so a second (contrast) anodize colour or a masked
  bead-blast sits below the worn ring faces, combined with a continuous one-spline crescent planform. Functional side:
  the rims keep the bore/OD contact surfaces at full stock while the channel removes mass where the certified ring-pull
  SF is ~9 (low-stress zone). See WHITEPAPER IX-C.
* **Open:** the R0.5 floor fillet is not in the raster (slightly conservative on mass, slightly unconservative on the
  groove-floor Kt in the ring-pull case — ring SF ≥ 8.8 so not governing). Contrast-anodize process masking is an owner
  / finisher item, not modelled.

## 6. Reproduce
```
cargo build --release --bin jarambit
JARAMBIT_MAX_THREADS=48 nice -n 19 ./target/release/jarambit crescent-smoke --out out/crescent_smoke --pop 64 --gens 40 --threads 48 --seed 7 --t 5.7 --area 2.0
./target/release/jarambit crescent-conv --evals out/crescent_smoke_v0_uncertified/all_evals.csv --n 24 --out out/crescent_conv
python3 ops/wave_figures/make_crescent_plate.py out/crescent_smoke out/mh1/heatmaps
```
Committed copies: `docs/crescent/` (report, convergence, archive, history, genome, plate index) and
`docs/figures/wave_heatmaps/wave6_crescent_*` (aesthetic passers only; removed in v3.0.2, in git history at tag v3.0.1).

* **C.6 status after waves 13–14 (2026-10-11):** C.6 silhouette floor (Hausdorff vs C.6 exterior + groove ≥ 2 mm) is a hard
  gate; novelty is scored only inside that envelope. Lightest C.6-certified design wave13_c01 74.37 g (A 0.763, novelty 0.173).
  Remaining gap: through-lattice designs (wave 14) trade aesthetic score (A 0.69–0.76) for SPG; owner review of perforation
  visual language is open.

## 8. D1 review gaps (2026-10-11): status

| item | status | where |
|---|---|---|
| chord torsion vs bending couple | FIXED + unit tests (strip benchmark, St-Venant 3D) | crates/crescent/src/fem.rs, tests.rs |
| 1.82 N*m twist floor | measured; tip-patch fixture FAILS all designs (SF 0.45-0.56, peak on the ground tip); body >15 mm from tip SF 2.9-3.6. OWNER decides the fixture | docs/d1/D1_P0_STATUS.md |
| yield 828 MPa provisional | DONE | crates/physics/src/lib.rs, CALIBRATION §7 |
| joint thin+hot | DONE (product) | fem::joint_k |
| analytic grind export | grind-law JSON REAL (`export::grind_law_json`: ruled surface C(s)+u(N,tan theta), land line); analytic STEP B-rep is a STUB, ETA ~1 working day. **Faceted STEP is not a machining release.** | crates/crescent/src/export.rs |
| failure sequence (hand onset >= 1.2 x blade onset) | proxy REAL (`Fem::seq_ratio`); all evolved designs FAIL (0.83-0.97, lateral) | fem.rs |
| tip exclusion submodel | STUB (the peak sits at the 4 mm exclusion edge) | — |
| ring peel | STUB | — |
| fatigue | STUB | — |
| pry | real T6 capacity, no floor; feeds the seq proxy | fem.rs |
| continuous-rail architecture / thickness field | PLAN only (D1_P0_STATUS answer d) | — |
| holder orientation | OPEN | — |
| promotion | BLOCKED until Jepson's aesthetic sign-off | — |
