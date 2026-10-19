# JR-EI-P3-T02: 129-Gene Parameterization, Decoder, Mating/Mutation Operators, and 18-Evaluator / 16-Gate Technical Exposition

- **Task ID:** `JR-EI-P3-T02`
- **Canonical Ledger Title:** `Expand genome/decoder and actual mating/mutation explanation`
- **Admiralty Task ID:** `jarambit:JR-EI-P3-T02@r2:sha256:e2632df8b13129a114068944b72d61c87e132f4b9dcccc073924e3b892921981`
- **Governing Directives:** `SC-DEC-183K`, `SC-DEC-183L`, and Additive Erratum `SC-DEC-183M` (`/tmp/sc183m.md`)
- **Target Manuscript Sections:** `docs/paper/sections/03-genome.tex` (`JR_EI_P3_T02_03-genome.tex`) and `docs/paper/sections/04-evolution.tex` (`JR_EI_P3_T02_04-evolution.tex`)
- **Validation Status:** `implemented_unvalidated` (`completed_draft_pending_primary_merge_and_p8_ci`; `CI_HOLD_UNTIL_P8 = true`; **NO git push to `origin`** per `SC-DEC-183M` §1)

---

## 1. Mandatory `SC-DEC-183M` Erratum & Source-Binding Declarations

1. **No Branch Push Until `P8` (`SC-DEC-183M` §1):**
   - `.github/workflows/runner-policy.yml` has an unrestricted `push:` trigger. Pushing `paper/jr-ei-p3-p5-drafts-20261019` to `origin` would trigger remote CI and violate `CI_HOLD_UNTIL_P8`.
   - Verified state: `git -C /home/ubuntu/jarambit ls-remote --heads origin paper/jr-ei-p3-p5-drafts-20261019` is **EMPTY**. No remote push has been or will be executed prior to `P8`.

2. **Withdrawal of Invented `G1 <= 0.45` Threshold (`SC-DEC-183M` §2 & `A01` Erratum):**
   - `SC-DEC-183L` §2.1 parenthetically referred to `G1` as a `"dimensionless ratio <= 0.45"`. That parenthetical is **WITHDRAWN** per `SC-DEC-183M` §2.
   - As established in `docs/paper/JR_EI_A01_WEB_FALLBACK_CONTRADICTION_FIX_RECEIPT_ERRATUM_20261019.json` (`sha256: 806742003b951628d956a3359d778693f72b457249ab23a9d71f97044a7d9fc7`, `8,627` B), neither a `0.45 mm` physical thickness gate nor a `0.45` dimensionless fitness cutoff exists for `G1`.
   - In source (`crates/crescent/src/g1.rs`, `sha256: 88fd8b8322658643018757054f30b4227ca94b5b6796d3c749d203897a63e014`; `metrics/global_metric.py`, `sha256: 6550298e27c6918d349e771f086426614669c844a9e677a0df1bb0447110b9e9`), `G1` (`global_fitness_G1.G`) is a **dimensionless higher-is-better composite fitness scalar**, not a physical dimension or a `<= 0.45` gate.

3. **Withdrawal of Unbound `W283` (`0.6929...`) and `'W293 vs W283'` Numbers (`SC-DEC-183M` §3):**
   - The unbound numbers `g1=0.6929788378349956`, `g2=0.8165475938225951`, `g3=0.6594793678879538`, `total=0.7192900478962451` and `'W293 vs W283'` in `SC-DEC-183L` §2.1 lack source file/key/hash provenance in the repository and are **WITHDRAWN** per `SC-DEC-183M` §3.
   - Every numerical metric in this exposition is strictly bound to its canonical repository file path, JSON key or CSV column, and 64-hex SHA-256 digest.

---

## 2. The 129-Gene Parameterization and Decoder Contract (`crates/crescent/src/genome.rs`)

### 2.1 Exact Column Partition in `all_evals.csv` (`146` Total Columns = `17` Scalar Metadata + `128` `f64` Genes + `1` Categorical Material)

Across all 10 completed generation-800 runs (`1,954,560` rows verified in `docs/paper/JR_EI_P1_T08_LOSSLESS_GEN800_POPULATION_ACCESSION_RECEIPT_20261019.json`, `sha256: 79ba4b6da9df8fda27dfa035f20187499cf032420a5b15e90ccfbaa06a1479a1`) and the historical `w282_ultrarunner` cohort (`1,984` births in `evidence/rs_v2/w282_ultrarunner/search/all_evals.json`, `sha256: 307370108fcede8a4399569582f6ab19d436603246531266afa1754f9ba65363`), each evaluated candidate record in `all_evals.csv` has a deterministic 146-column header (`header_sha256: 2f1469c4d391aca315ddc3204f903fec1e78e57359cf3cdcea2726004555cfb1`):

1. **Columns `0..=16` (`17` Scalar & Search-Metadata Columns):**
   `id`, `gen`, `fit`, `spg`, `mass_g`, `aes_pass`, `aes_score`, `hausdorff`, `emd`, `novelty`, `lat_rob`, `ring_rob`, `sf_hunter`, `legal`, `n_rejects`, `relief_frac`, `n_win`.
2. **Columns `17..=144` (`NG = 128` Numeric `f64` Gene Coordinates in `crates/crescent/src/genome.rs`, `sha256: 13d8ca4a57c6fb227319b0b12247396bc1d9bd2cb42db4fb2e466bf0ebff6bb6`):**
   - **Block A — Silhouette Normal Control Points (`genes[0..40]`, CSV columns `17..=56`, `N_CP = 40`):**
     `cp00` through `cp39`. Each gene encodes a normal displacement (in mm) relative to the C.4 reference outline (`C4_SIL`). Zone-dependent bounds `cp_delta(i)` restrict normal offsets by anatomical station:
     - Ring outer arc (`i in 0..5` or `i >= 36`): `±1.5 mm`
     - Tip approach (`i in 15..23`): `±2.0 mm`
     - Belly, spine, and grip arches (remaining indices): `±3.5 mm`
   - **Block B — Core Planform, Thickness, Bevel, Serration, Relief, Lattice, Fuller, Flute, Texture, Wrap, Pad, and Edge Genes (`genes[40..90]`, CSV columns `57..=106`, `NG_LEGACY = 90`):**
     - Planform & ring (`40..45`): `blade_drop_deg` (`[-8.0, 12.0]` deg), `grip_dx` (`[-3.0, 4.0]` mm), `ring_wall` (`[3.2, 6.5]` mm), `ring_id` (`[23.0, 25.0]` mm), `neck_fillet` (`[0.0, 5.0]` mm).
     - Stock & bevel (`45..50`): `t_stock` (`[4.5, 6.0]` mm), `t_tip` (`[2.2, 5.0]` mm), `taper_x0` (`[-35.0, 5.0]` mm), `edge_angle_in` (`[11.0, 22.0]` deg), `edge_angle_sp` (`[12.0, 24.0]` deg).
     - Comet groove & ergonomics (`50..61`): `comet_x0`, `comet_x1`, `comet_w0`, `comet_w1`, `comet_bow`, `comet_y`, `eye_scale`, `cairn_d`, `nub_engage`, `holder_wall`, `jimp_scale`.
     - Serrations & spine (`61..65`): `serr_large_d`, `serr_small_d`, `serr_fine_d` (`[0.0, 0.35]` mm), `spine_sharp_offset` (`[0.0, 45.0]` mm).
     - Handle relief pocket (`65..68`): `relief_d` (`[0.0, 2.2]` mm), `relief_w` (`[4.0, 14.0]` mm), `relief_q` (`[-6.0, 6.0]` mm).
     - Hex/dimple lattice (`68..74`): `lattice_d`, `lattice_pitch`, `lattice_frac`, `lattice_angle`, `lattice_handle`, `lattice_blade`.
     - Blade fuller & handle flute (`74..79`): `fuller_d`, `fuller_w`, `fuller_s0`, `fuller_s1`, `flute_d`.
     - Surface texture, cord wrap, grip pad, & cutting edge (`79..90`): `texture_mode`, `texture_d`, `texture_pitch`, `texture_angle`, `wrap_x0`, `wrap_len`, `pad_x`, `pad_y`, `pad_w`, `pad_l`, `edge_t` (`[0.25, 1.20]` mm).
   - **Block C — Wave 7–16 Triangular Isogrid, Pierced Windows, Ring Flute, and Micro-Perforation Genes (`genes[90..117]`, CSV columns `107..=133`, `NG_P0 = 117`):**
     - Isogrid (`90..99`, `108..110`): `iso_d` (`[0.0, 2.5]` mm), `iso_pitch` (`[3.0, 9.0]` mm), `iso_rib` (`[0.8, 2.5]` mm), `iso_angle` (`[-60.0, 60.0]` deg), `iso_rim` (`[1.8, 4.5]` mm), `iso_blade`, `iso_handle`, `iso_blade_extent`, `iso_grade`, `iso_handle_start`, `iso_land`.
     - Pierced windows (`99..105`): `win_n` (`[0.0, 8.0]`, integer-rounded by `Genome::clamp`), `win_s0`, `win_s1`, `win_w`, `win_fill`, `win_decay`.
     - Ring flute (`105..108`): `rflute_d`, `rflute_w`, `rflute_arc`.
     - Micro-perforations (`110..117`): `perf_d` (`[0.0, 1.5]` mm), `perf_gap` (`[0.8, 2.5]` mm), `perf_angle`, `perf_elong`, `perf_land`, `perf_handle`, `perf_handle_start`.
   - **Block D — Wave 17+ D1 P1 Continuous-Rail Layer (`genes[117..128]`, CSV columns `134..=144`, `NG = 128`):**
     `p1_on` (`{0.0, 1.0}`, thresholded at `>= 0.5` by `Genome::clamp`), `rail_w` (`[2.2, 5.5]` mm), `neck_l` (`[6.0, 24.0]` mm), `pocket_d` (`[0.0, 2.3]` mm), `pocket_r` (`[1.5, 5.0]` mm), `k_n` (`{3.0, 4.0, 5.0}`, rounded by `Genome::clamp`), and internal thickness spline knots `k0`..`k4` (`[1.5, 6.0]` mm).
3. **Column `145` (`1` Categorical Material Field):**
   `material` (`MaterialGrade::Ti6Al4V` serialized as `"Ti-6Al-4V"`, or `MaterialGrade::Ti3Al2_5V` serialized as `"Ti-3Al-2.5V"`).

### 2.2 Decoder Invariants and Coupling (`Genome::clamp` & `crates/crescent/src/pheno.rs`)

The 128 numeric coordinates and 1 material column do **not** represent 128 independent physical degrees of freedom:
- **Clamping & Ordering Coupling (`Genome::clamp`):**
  - Enforces box bounds `[lo, hi]` on all 128 genes.
  - Enforces `t_tip <= t_stock` (`genes[46] > genes[45] => genes[46] = genes[45]`).
  - Enforces `comet_x1 >= comet_x0 + 15.0` (`genes[51] < genes[50] + 15.0 => genes[51] = (genes[50] + 15.0).min(20.0)`).
  - Enforces `fuller_s1 >= fuller_s0 + 0.12` and `win_s1 >= win_s0 + 0.12`.
  - Discretizes `texture_mode` (`genes[79] = genes[79].round()`), `win_n` (`genes[99] = genes[99].round()`), `p1_on` (`genes[117] = if genes[117] >= 0.5 { 1.0 } else { 0.0 }`), and `k_n` (`genes[122] = genes[122].round().clamp(3.0, 5.0)`).
- **Conditional Layer Activation (`Genome::p1`):**
  When `p1_on >= 0.5`, the D1 P1 continuous-rail thickness law (`rail_w`, `neck_l`, `pocket_d`, `pocket_r`, `k_n`, `k0..k4`) governs the interior web/pocket thickness field; when `p1_on < 0.5`, those 11 coordinates remain stored in the CSV row (`129` genome columns preserved losslessly) while the legacy/P0 relief, lattice, fuller, isogrid, and perforation fields govern the thickness map.

---

## 3. Actual Implemented Mating (`crossover`) and Mutation (`mutate`) Operators

### 3.1 Two-Parent Crossover (`Genome::crossover` in `crates/crescent/src/genome.rs`)

Given two parent genomes `a` and `b` and PRNG state `r: &mut Rng`:
1. **Block Inheritance on the 40 Silhouette Control Points (`0..N_CP`):**
   With probability `0.5` (`if r.f64() < 0.5`), the child copies the entire 40-point silhouette displacement vector `genes[0..40]` from parent `b`; otherwise it retains `genes[0..40]` from parent `a`. This prevents jagged, non-smooth outline seams that would arise from independent per-control-point coin flips.
2. **Independent Coordinate Inheritance on Structural & Feature Genes (`N_CP..NG`):**
   For each index `i in 40..128`, with probability `0.5` (`if r.f64() < 0.5`), `g.genes[i] = b.genes[i]`; otherwise `g.genes[i] = a.genes[i]`.
3. **Material Grade Inheritance:**
   With probability `0.5`, `g.mat = b.mat`; otherwise `g.mat = a.mat`.
4. **Post-Crossover Feasibility Clamp:**
   `g.clamp()` enforces all box, ordering, and discrete-gene invariants before evaluation or downstream mutation.

### 3.2 Six-Operator Structured Mutation (`Genome::mutate` in `crates/crescent/src/genome.rs`)

Given a genome `g`, PRNG `r`, and scale flag `fine: bool` (`scale = 0.04` when `fine == true`, `0.15` when `fine == false`):
- `Genome::mutate` draws `op = (r.f64() * 6.0) as usize` uniformly from `{0, 1, 2, 3, 4, 5}`:
  - **`op == 0` (Per-Gene Sparse Gaussian Perturbation):** Each coordinate `i in 0..128` is perturbed independently with probability `0.15` by `r.gauss() * (hi - lo) * scale`.
  - **`op == 1` (Smooth Silhouette Bump):** Selects center index `center = (r.f64() * 40.0) as usize`, width `width = 1.5 + r.f64() * 4.0`, and amplitude `amp = r.gauss() * 1.2 * (0.3 if fine else 1.0)`, applying a periodic Gaussian bump `amp * exp(-0.5 * (d / width)^2)` across all 40 normal control points `cp00..cp39`.
  - **`op == 2` (Thickness, Taper, & Relief Operator):** Perturbs `t_stock` (`45`), `t_tip` (`46`), `taper_x0` (`47`), `relief_d` (`65`), and `relief_w` (`66`). If `g.p1()` is active, also perturbs `rail_w` (`118`), `pocket_d` (`120`), and one randomly chosen knot `k0..k4` (`123..128`).
  - **`op == 3` (Dimple-Lattice Operator):** Toggles lattice off (`lattice_d = 0.0` with probability `0.15`) or perturbs `lattice_d`, `lattice_pitch`, `lattice_frac`, `lattice_angle`, `lattice_handle`, and `lattice_blade` (`68..74`).
  - **`op == 4` (Fuller, Flute, Texture, & Comet Groove Operator):** Perturbs `fuller_d`, `fuller_w`, `fuller_s0`, `fuller_s1`, `flute_d` (`74..79`), `texture_mode` (`79`, with probability `0.3`), `texture_d`, `texture_pitch`, `texture_angle` (`80..83`), and `comet_w0`, `comet_w1`, `comet_bow` (`52..55`).
  - **`op == 5` (Sub-op 0..4 Isogrid / Pierced-Window / Ring-Flute / Perforation / P1 Continuous-Rail Operator):**
    - When `g.p1()` is active and `r.f64() < 0.7`, executes P1 continuous-rail mutation on `rail_w`, `neck_l`, `pocket_d`, `pocket_r`, `k_n` (with probability `0.2`), and knots `k0..k4`.
    - Otherwise selects `sub = (r.f64() * 4.0) as usize`: `sub == 0` mutates triangular isogrid (`90..99`); `sub == 1` mutates pierced windows (`99..105`); `sub == 2` mutates ring flute + handle isogrid land (`105..110`); `sub == 3` mutates angled micro-perforation field (`110..117`).
- **Material Mutation:** When `!fine && r.f64() < 0.03`, toggles `g.mat` between `Ti6Al4V` and `Ti3Al2_5V`.
- **Final Clamp:** Invokes `g.clamp()`.

### 3.3 Recorded Multi-Hop Genealogy and Ancestor Reuse (`docs/paper/generated/population/inheritance.json`)

In `docs/paper/generated/population/inheritance.json` (`sha256: 337abc826a4032256db7700ba68abdee8aee51ab09817853ee6741a27d99f3df`, `20,249` B), the focal 3-hop ancestry subgraph around `Individual 1095` (Run 1 highest stored birth fitness `fit = 1.15727` at generation 9) demonstrates actual recorded operator execution and coordinate-level differences across 7 nodes:
- `Individual 174` (gen 1, `fit = 1.14979`, `archive_crossover_mutate`, parents `[57, 61]`, `csv_gene_serialization_sha256: ffdbc6ca0e45c32c8ddc8ce80c8343d1ba69000e8c4b6621f2308eaa1c8a4d7c`)
- `Individual 245` (gen 2, `fit = 1.15388`, `archive_mutate` from parent `[174]`: `24` changed `f64` genes, `104` identical `f64` genes, `l2_gene_distance = 0.604263`, `max_abs_coordinate_delta = 0.361541`)
- `Individual 299` (gen 2, `fit = 1.14855`, `archive_mutate` from parent `[174]`: `1` changed `f64` gene `t_stock`, `127` identical `f64` genes, `l2_gene_distance = 0.032993`)
- `Individual 392` (gen 3, `fit = 1.12512`, `mutate` from parent `[299]`: `6` changed `f64` genes, `122` identical `f64` genes, `l2_gene_distance = 4.948287`)
- `Individual 413` (gen 3, `fit = 1.15296`, `archive_mutate` from parent `[245]`: `5` changed `f64` genes, `123` identical `f64` genes, `l2_gene_distance = 0.903516`)
- `Individual 526` (gen 4, `fit = 1.15563`, `crossover_mutate` from parents `[413, 392]`: vs `413` `30` changed `f64` genes, `l2_gene_distance = 13.719885`; vs `392` `22` changed `f64` genes, `l2_gene_distance = 7.826658`)
- `Individual 1095` (gen 9, `fit = 1.15727`, `crossover_mutate` from parents `[413, 526]`: vs `413` `30` changed `f64` genes, `l2_gene_distance = 13.720172`; vs `526` `5` changed `f64` genes, `l2_gene_distance = 0.946269`). Notably, `Individual 413` is reused both as parent of `526` and as direct parent of `1095` alongside `526`, while `Individual 174` is a common ancestor along both the `174 -> 245 -> 413` and `174 -> 299 -> 392 -> 526` branches. Despite having the highest stored birth fitness in Run 1, `Individual 1095` recorded `cert_pass_all = false` (`seq_ok = false`, `strict_ok = true`) in `evidence/rs_v2/w282_ultrarunner/search_summary.json#/strict_certification` (`sha256: 0ca6e9dc716b525a15e1f4d6285161efcf42a5be7aa3eb8fd42c8aa5b43cf11c`).

---

## 4. Exact Formulas and Source Bindings for `G1`, `W_CAD_v2`, and the 18-Evaluator Battery

### 4.1 Global Fitness `G1` (`crates/crescent/src/g1.rs` & `metrics/global_metric.py`)

In `crates/crescent/src/g1.rs` (`sha256: 88fd8b8322658643018757054f30b4227ca94b5b6796d3c749d203897a63e014`) and `metrics/global_metric.py` (`sha256: 6550298e27c6918d349e771f086426614669c844a9e677a0df1bb0447110b9e9`), the dimensionless composite score `G1` (`higher_is_better`) is defined on non-vetoed candidates by:

$$G_1 = 0.35\,\min(\mathrm{SF}_{\text{solid\_tip}},\,2.0) + 0.35\,\frac{\mathrm{SPG}}{4.0} + 0.20\,s_{\text{aes}} - 0.05\,\frac{\max(0,\,m_g - 77.0)}{10.0} - 0.05\,\frac{\max(0,\,1.2 - r_{\text{seq}})}{1.2}$$

where:
- $\mathrm{SF}_{\text{solid\_tip}}$ is the 3D quadratic ($P_2$) tetrahedral tip safety factor under `1.82 N·m` torsion (`cap = 2.0`),
- $\mathrm{SPG} = \mathrm{SF}_{\text{hunter}} / (m_g / 100)$ is strength-per-gram under `260 N` lateral load (`scale = 4.0`),
- $s_{\text{aes}} \in [0, 1]$ is the C.6 silhouette aesthetic similarity score,
- $m_g$ is mass in grams above the `77.0 g` soft reference (`10.0 g` scale), and
- $r_{\text{seq}}$ is the failure-sequence ratio below `1.2`.

### 4.2 Stage-2 `W_CAD_v2_ultrarunner_karambit` (`ops/ci/evaluate_cad_candidate.py`)

In `ops/ci/evaluate_cad_candidate.py` (`sha256: 18595cecc6336053d7304d34ab6d9ffbd4b0542bb9df504323b6bc8ea6887ef0`), `W_CAD_v2` evaluates 16 legacy engineering gates (`G16-01`..`G16-16`) with soft weights $w_{\text{mass}}=0.70$, $w_{\text{karambit}}=0.12$, $w_{\text{survival}}=0.10$, $w_{\text{seq}}=0.05$, $w_{\text{carry}}=0.03$:
- If `all_engineering_gates_pass == true`:
  $$W_{\text{CAD}} = 1.0 + 0.70\,s_{\text{mass}} + 0.12\,s_{\text{karambit}} + 0.10\,s_{\text{survival}} + 0.05\,s_{\text{seq}} + 0.03\,s_{\text{carry}} \in [1.0,\,2.0]$$
- If `all_engineering_gates_pass == false`:
  $$W_{\text{CAD}} = -\left(1.0 + 4.0\sum_{k \in \text{shortfalls}} \Delta_k\right) + 0.10\,s_{\text{mass}} < 0$$

### 4.3 Canonical Source-Bound `W283` Controlled-Probe Metrics (`evidence/rs_v2/w283_ultrarunner/`)

| Candidate Alias | Public ID | Source File & SHA-256 | `W_CAD` (`W_CAD_v2_ultrarunner_karambit.W_CAD`) | `G1` (`global_fitness_G1.G`) | Battery `G1` (`gate.rs_v2`) | `all_engineering_gates_pass` | Retracted v1.0.0 Preimage (`W_CAD` / `G1`) |
|---|---:|---|---:|---:|---:|---|---|
| `w283_p14_c01` | `Individual 2001` | `evidence/rs_v2/w283_ultrarunner/w283_p14_c01_cad_eval.json` (`sha256: df5557342f59a0e60b703d533638ec731317672f9a5330a32200a3f9262bfdc6`) | `-3.878704` | `0.6239029999999999` | `0.623888308291947` | `false` (`8/16` gates pass) | `1.795575` / `0.913854625` |
| `w283_p10_c02` | *(controlled probe)* | `evidence/rs_v2/w283_ultrarunner/w283_p10_c02_cad_eval.json` (`sha256: 6eb44dab282bc8c6d42dc2194284e1bb2f999d0460b1091f1085c452abbcdfcf`) | `-3.964924` | `0.6216340416666666` | `0.6216318603776162` | `false` (`8/16` gates pass) | `1.816593` / `0.9167193749999998` |

### 4.4 The 18 Historical CAD Battery Evaluators (`evidence/cad_battery/manifest.json`)

Bound to `docs/paper/battery-inputs.v1.json` (`sha256: ab70614f0df08986b2db655cfee101a0d6a498d89681533f6722d97131637de2`) and `evidence/cad_battery/manifest.json` (`sha256: 75e29e9a871b241d464616c4968f5c7990a99923d409e338c2b9358e076853c8`), the 18 evaluators across 23 subjects (`414` cells: `221` `PASS`, `81` `FAIL`, `112` `BLOCKED`; `fresh_this_lane: false` on all 414 cells) partition into:
1. **Shared Software Controls (`4` evaluators × `23` subjects = `92` shared `PASS` cells):** `build.analytic`, `build.gci_contracts`, `build.claims`, `build.dolfinx`.
2. **Exported Geometry & Identity (`4` evaluators):** `geom.unit` (`23/0/0`), `geom.one_solid` (`23/0/0`), `geom.mass` (`18/5/0`), `geom.pin` (`16/5/2`).
3. **2.5D Plate Mechanics (`4` evaluators):** `plate.loads` (`18/5/0`), `plate.seq` (`13/10/0`), `plate.l7` (`4/7/12`), `plate.gci` (`5/0/18`).
4. **Historical Side-Report Audits (`4` evaluators):** `side.a1` (`1/4/18`), `side.a2` (`5/0/18`), `side.a3` (`5/0/18`), `side.a4` (`5/0/18`).
5. **Local 3D Solid Completeness & Grouped RS-v2 Verdict (`2` evaluators):** `solid.tip` (`12/0/11`, result completeness only, not strength pass), `gate.rs_v2` (`1/22/0`, where only the synthetic geometric control `ctrl_thick_ti64_10mm` passes and all `22` evolved/reference knife candidates fail).
