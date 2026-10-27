<!-- Frozen copy of the G3 genome draft cited by crates/g3/src/{g3_genome,g3_qd}.rs ("G3_GENOME_SPEC.md v1.1"), committed
     in v3.0.3 for provenance. Unchanged from the run copy (sha256 2f5a930f…). Its pointers to local tools (tools/hzm.py,
     ~/jarambit/plan.md) are historical; the implementation of record is crates/g3. -->

# G3 genome spec v1.1 (draft): Jarambit fresh run, fixed blade, hollow-capable L-PBF Ti-6Al-4V

Authors: Claude-HZ (H, S, I, M) and Helper (B, E, I extensions, 2026-10-25 00:08Z review). v1.1 merges the Helper review and the owner decisions of 2026-10-25: fixed blade, no sheath, finger-basis ring, compact grip, balance.

Status: the hand zone (H, S, I) is prototyped and running (`tools/hzm.py`, 98 genes). Modules B, E and M are specified here and are not yet implemented.

Companion files:
- `~/jarambit/plan.md`: scope, owner decisions, task board.
- `tools/ergolight.py`: W_ERGOLIGHT_v1.
- `tools/evaluate.py`: hand-zone metrics and descriptors.

## 0. Why a new genome (v1 limits measured in code)

- **v1 cannot express the owner's targets.** The ring band is fixed at 2.5 × wall, stock thickness is at most 6 mm, and the solid is a z-symmetric height field with no voids and no rounded outer edges. There are no groove genes.
- **v1 is anchored to one hand drawing.** The planform is C.4 ± 3 mm, with Hausdorff ≤ 6 mm and EMD ≤ 0.06 against C.5, and 14 genes sit on a bound.
- **v1 breaks heredity.** Hard-coded offspring overrides reset genes regardless of the parent.

## 1. Encoding

- **Fixed length.** A real vector in the unit cube [0,1]^n, n ≈ 180.
- **Header.** Every row carries `schema=G3.0`, `genome_sha256`, `decoder_commit`.
- **Strict loader.** It refuses any other schema, including v1's 90/117/128-length rows: no padding, no silent upgrade.
- **Gene record.** Each gene has `(name, module, lo, hi, map: lin|log, default)`. The physical value is lo + u·(hi − lo), or log-interpolated.
- **No hidden overrides.** Nothing in the search code may overwrite a gene. The only constraint mechanism is the decoder.

## 2. Construction rules (valid by construction, never by rejection)

| Rule | How the decoder guarantees it |
|---|---|
| Bore Ø 24.9 at the crown apex; lip 8 mm; crown 0.2–0.4; bore rounds 2.0–3.5 | Owner-fixed constants plus bounded genes. The bore is cut last by a rounded intersection |
| Hand-zone convex edges R ≥ 2.2 (none < 1.0) | Morphological opening with a radius field r_o(x) ≥ 2.2 in the hand zone. The ring outer spine is full-round (R = t/2 × 0.9) |
| Concave fillets (neck ≥ 4, grip→guard 5–10, lip→web) | Morphological closing with a radius field r_c(x) |
| Finger stations follow the hand | Groove centres = ring tangent + design.md pitches (19.5 / 21 / 22) × `hand_scale` + offsets (±2) |
| Hollow only where printable | Cavity = interior deeper than `skin_t`, opened by 0.5 mm (no voids < 1 mm), only where the zone density < 0.95. Edges, the tip, the grind and the bore stay solid automatically |
| Cutting edges (Gate S) | Apex thickness `edge_t` ≤ 0.20 and half-angle ≤ 25° are gene bounds. The point taper is built monotone, with S2 t(0.25/1/2 mm) inside bounds |
| Blade-length caps (RS-v1 `blade_limit_mm` 85.725, `sharpened_edge_max_in` 4.0) | The decoder scales the sharpened arc. Whether the cap applies per edge or in total is an **open owner question** |
| Seam to x ≥ 99 (prototype only) | In the prototype, the original blade is used verbatim. G3 proper builds the blade too |

## 3. Modules and genes

### H: hand zone (prototype implemented; 47 genes after the 2026-10-25 owner update)

| Part | Genes |
|---|---|
| Ring | `ring_od` 29.5–35, `crown` 0.2–0.4, `lip_d` 1.5–2.8, `taper_deg` 30–75 (T-like ↔ conical comfort taper), `web_t` 1.8–4.0, `thumbpad` 0–3.5, `od_tear` −1…2 (radial root bridge, parabolic per design.md), `od_h2` ±0.6, `junction_deg` 30–50, `neck_r` 4–12, `bore_round` 2.0–3.5 |
| Grip | `hand_scale` 0.9–1.1, `dL` −1…5, T0/T_sw/T_g (8–12 / 13–17 / 11–15), `s_sw`, yt0/yt_sw/yt_land, `s_ps`, yb0/yb_g, `belly`, groove depth/width/offset × 3 (ring 1–3, mid 0–3, index choil 2.5–5), n0/n1 2.2–3.2, `skew`, `edge_r` 2.2–3.0, `ramp_h` 0–3 (thumb ramp) |
| Guard | `h_g` 10–15, `t_guard` 5.9–8, `guard_r` 5–10, jimping `n` 0–8 / `p` 1.8–2.8 / `d` 0.4–0.7 (sinusoid, crest R = p²/(2π²d) ≥ 0.5) |

### S: surface morphogen (implemented; 43 genes)

- **Relief:** `tex_amp` 0–0.25 mm, `tex_pitch` 1.2–2.6, `tex_angle` ±45.
- **Family blend:** weights `w_knurl`, `w_turing`, `w_voronoi`, `w_ridge`, `w_cppn`. The relief is a **blend**, not a choice, so hybrids exist.
- **Turing tiles:** `turing_mix` (two Gray–Scott tiles).
- **Seed:** `seed` (Voronoi jitter).
- **CPPN:** 32 weights for a fixed-topology 3→6→2→1 network with sin / gauss / abs / tanh units.
- **G3 adds:** `scope_grip`, `scope_ring`, `scope_blade` (where the relief applies; 0 = none). Blade-face relief doubles as alien lightening pockets, with depth bounded by the Gate S grind margin.

### I: interior (implemented for the hand zone; G3 adds the blade)

- **Shared:** `skin_t` 0.8–1.2, `lat_mix` (gyroid ↔ Schwarz-D blend of level sets), `lat_cell` 4–7, `rib_w` 0–1 (mid-plane tendon web).
- **Per-zone density** (1.0 = solid): `rho_grip`, `rho_guard`, `rho_ring`, and **G3 adds `rho_blade`** (spine cavity).
- **G3 adds:** `lat_grade` (amplitude of a CPPN channel modulating local density, within 0.06–1), and `port_n` 2–4.
- **Printability:** network struts ≈ 0.603 · cell · √ρ ≥ 0.8 mm, scored in `am_build`.

### B: blade planform (to implement; replaces v1's cp offsets and C.4 anchoring)

**Global morph genes**

| Gene | Range | Note |
|---|---|---|
| `blade_len` | 60–86 mm | Sharpened-arc reach; the decoder scales it to the cap |
| `sweep_deg` | −12…12 | v1 drop was ±4 |
| `hook_drop` | 25–55 mm | v1 winner 41.9 |
| `belly` | — | Inner-edge curvature amplitude |
| `spine_arch` | — | — |
| `tip_angle` | 18–40° | Included angle at the point |
| `h_root` | 18–30 | Blade height at the heel |
| `h_exp` | 0.6–1.6 | Height taper exponent |

**Edge shape:** cosine-basis offsets per edge, a_k ∈ ±(6/k) mm for k = 1…8. Low frequencies get large σ and high frequencies small σ (multi-scale locality).

**Thickness:** `t_core` 4.0–7.0 (AM, not plate stock; holder-stack scoring stays soft), `t_tip` 1.5–4.5, `taper_x0`.

**Face features, all continuous with no switches:**
- fuller: depth, width, s0, s1;
- six window slots: size 0 = absent, plus s and elongation;
- isogrid amplitude.

### E: edges, double-edged and double-serrated (to implement; two copies, `in` and `sp`)

Each edge `k ∈ {in, sp}` gets:
- `s0_k` / `s1_k`: sharpened arc; an empty band means unsharpened;
- `angle_k`: half-angle 14–25;
- `edge_t_k`: 0.05–0.20;
- `grind_k`: flat ↔ convex ↔ hollow, as an exponent;
- serration band: `ser_s0_k`, `ser_s1_k`;
- `ser_pitch_k` 2–8, `ser_depth_k` 0.5–2.5, `ser_duty_k` 0.3–0.8;
- pattern blend weights `w_scallop_k`, `w_tooth_k`, `w_micro_k`, plus `ser_phase_k`.

Both edges are symmetric double-bevel (z-symmetric). Gate S is scored per edge, and a serration root must keep the edge's apex rule.

### M: meta (to implement; 5 genes)

`sigma_B`, `sigma_E`, `sigma_H`, `sigma_S`, `sigma_I` are log-uniform 0.01–0.2. They are self-adaptive through log-normal mutation of σ before the child is drawn. σ is inherited, so lineages learn their own step sizes.

## 4. Variation and selection

- **Mutation:** iso + line-DD, child = a + σ_m·N(0, I) + 0.25·N(0,1)·(b − a), with **reflective** bounds.
- **Crossover:** with probability 0.15, a whole module from another elite (module-aware).
- **Archive:** CVT-MAP-Elites with 1024 centroids over 8 measured descriptors. Ranking inside a niche is by W_ERGOLIGHT_v1. Parent choice is curiosity-weighted (a child improving its niche credits the parent).

| # | Descriptor | Measured from |
|---|---|---|
| 1 | Hook drop | Measured |
| 2 | Inner-edge turning angle | Measured |
| 3 | Silhouette complexity | Outline length / √area |
| 4 | Grip roundness | H/T at the swell |
| 5 | Groove articulation | Σ depths |
| 6 | Porosity | Cavity / envelope volume |
| 7 | Surface energy | Amplitude × coverage |
| 8 | Mass | — |

- **Initialisation:** Sobol over the unit cube, with no v1 seeds. The v1 winner and the spec-reference design are scored as **benchmarks** only and never enter parent pools.
- **Islands:** each keeps its own archive. Migration is elites-only, between G3 pools, through the G3 S3 prefix.

## 5. Evaluation fidelity (G3)

| Stage | What it runs | Time (target) |
|---|---|---|
| 1 (screen) | 0.4 mm voxel geometry; ring membrane FE submodel; hollow-grip beam check; blade plate FEM coarse with **sandwich rigidities (A_eff, D_eff)**; descriptors | about 5–10 s |
| 2 (certify top-k) | 0.1 mm geometry; full canonical-definition ring cases; fine plate FEM; 3D solid tip (faster solver per `hpc/HPC_REPORT.md`); battery | a few minutes after the HPC changes |
| Finals | Faceted STEP + `step_check`, 3D ring FE, exports | — |

## 6. Evolvability acceptance (run on the decoder before any fleet launch)

| Test | Target |
|---|---|
| Validity on 1000 Sobol genomes (watertight single body, construction rules met) | ≥ 99 % |
| Locality: voxel IoU parent–child vs σ | Monotone; ≥ 0.9 at σ = 0.02 |
| Heritability: corr(parent, child score) at σ = 0.06 | ≥ 0.6 |
| Niche stability | ≥ 50 % of children stay within one niche of the parent |
| Coverage and QD-score | Still growing at 3000 evaluations (no early collapse) |

The prototype run `qd/run1` measures these for module H.


## 7. v1.1 changes (Helper review + owner decisions, 2026-10-25)

**Owner decisions:**
- Fixed blade; **no sheath in this run.** The holder and `carry_envelope` terms are removed, `t_guard` is 5.5–10, and there is no cairn retention hole.
- Ring: **finger basis, 300 N at SF ≥ 2** for pull and ring-back.
- **Compact grip:** perimeter 65–75, H/T 1.5–2.0.
- Spinning is a use case: balance is in E (CoM within 8 mm of the index choil, spin inertia ≤ 0.85 × f075).

**Module B (Helper):**
- A 3D structural cross-section field: `ridge_amp` 0–1.8 mm, `ridge_w` 2.5–7 mm, `ridge_s0`, `ridge_s1`, `ridge_offset`, plus `fuller_corrug_amp`. Material goes off the neutral plane (D ∝ ∫z²dz).
- The 6 window slots are **replaced** by a 5-gene level-set fenestration field: `win_amp` 0–1, `win_pitch` 6–18, `win_s0` 0.08–0.35, `win_s1` 0.45–0.85, `win_angle` ±40°. It is modulated by the CPPN/Voronoi field inside the medial fuller band, kept ≥ 2.0 mm from the grind line with ligaments ≥ 3.0 mm by construction. This removes slot permutation symmetry.

**Module E (Helper):**
- The bevel SDF is measured from the **serrated** contour, so every gullet and tooth keeps apex ≤ 0.19 mm and half-angle ≤ 24.5° (Gate S1/S3 by construction).
- `tip_reserve` 6–12 mm fades serrations before the point and enforces the S2 taper: t(0.25) ≤ 0.28, t(1) ≤ 0.56, t(2) ≤ 0.95.
- `grind_curv_k` ranges −0.35…+0.45, with the first 0.4 mm locked to `angle_k`.

**Module I (Helper):**
- `lat_aniso_x` 0.7–1.6 (TPMS stretched along the principal stress axis);
- `rho_grad_wall` 0–0.5 (cortical-to-cancellous grading near skins and junctions);
- `shear_web_amp` 0–1 (±45° bulkheads for the 250 N squeeze);
- `exo_open` fenestrations, masked to zero inside the hand-contact SDF band.

**Decoder rules learned in the prototype (unit-test them in Rust):**
1. **Variable-radius closing and opening** run on half-voxel-corrected signed distances with Gaussian-smoothed radius fields. Opening = mask ∧ dilate(erode). The neck radius applies only in the junction window. Tests: "closing a convex body returns the body" and "opening ⊆ body".
2. **Ring load cases** on rounded edges use a 1.5 mm thickness-weighted load band, not boundary tractions.
3. **TPMS void sign:** the lattice fraction in a cavity ≈ ρ (± 0.02).
4. **Connectivity:** only the solid component connected to the skin survives, so there are no loose struts.
5. **No grip "cliff":** the grip tapers into the guard plate over 14 mm (from L − 6 to L + 8), with fillets all round.
6. **Smoothed outlines:** the guard/land outline is smoothed (spine σ 1.5 mm, lobe 0.6 mm), so published jimping notches and pocket rims don't leak into the new geometry.

**Evolvability, measured on the hand-zone prototype (runs 2–4, about 7000 evaluations including 160 fully random Sobol genomes):**
- **0 decode failures** (100 % valid, watertight) with the 98-gene H/S/I genome.
- **381 niches** filled.
- The frontier rose from 68.2 (spec-reference design) to 74.4 W_ERGOLIGHT_v1 in 6000 evaluations, with mass falling from 71 g to 65.7 g. The comfort factor E of the leaders is about 0.98.
- On the M3 Ultra (16 workers): about 7.7 evaluations/s at 0.4 mm, including the ring FE submodel.
