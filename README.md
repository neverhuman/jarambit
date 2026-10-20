# Jarambit — Engineered Intelligence: Making Physical Design Executable

**An executable geometric genome defines CAD shape, an island-based population search explores alternatives across generations, and multi-model numerical solvers evaluate mechanical response.**

Jarambit is a Rust-centered computational design research system connecting a 128-coordinate geometric genome and categorical material state to deterministic CAD decoding, four-island evolutionary search, and finite-element assessment models. Its retained archive records **1,954,560 complete genome-and-score evaluations** across ten 800-generation runs (`Run 2`–`Run 11`), alongside a reconstructable 17-generation historical run (`Run 1`, `1,984` births) that exposes exact parentage, island slot memberships, and ring migrations, plus `3,649` intermediate multi-stage screening proposals. The [test atlas](docs/tests/README.md) explains every named CAD diagnostic and evaluator, with implementation and retained-evidence links.

[Read the Engineered Intelligence white paper](docs/paper/README.md) · [Compiled PDF (`docs/paper/main.pdf`)](docs/paper/main.pdf) · [Reproducibility crosswalk & supplement (`docs/paper/CROSSWALK_SUPPLEMENT.md`)](docs/paper/CROSSWALK_SUPPLEMENT.md) · [Machine accession contract (`docs/paper/publication.v2.json`)](docs/paper/publication.v2.json) · [Explore all tests](docs/tests/README.md) · [Detailed engineering spec](docs/PAPER_AND_README_ENGINEERING_SPEC.md) · [Architecture](TECHNOLOGY_MAP.md)

![Illustrated assessed export Individual 2000 — exact 3D CAD surface with registered 5-map structural demand index](docs/paper/figures/fig1_hero_blender_v2.png)

*Illustrated assessed export **Individual 2000** (single assessed candidate in [`evidence/evolve_certifiable/automatic/leaderboard.json`](evidence/evolve_certifiable/automatic/leaderboard.json): `assessed_count = 1`, `qualified_count = 0`, `champion = null`; separate from the 23-subject historical battery; absent per-birth Stage-2 records across the `1,954,560` native search rows establish missing retained assessments). White-background exact-CAD 3D surface projection (`44,994` vertices, `90,000` triangles, `0` modifiers, proper `SO(3)` pose `det = +1.0`, dark `#0f172a` exterior silhouette and inner ring/slot hole contours; deterministic SVG/raster recipe `016dc907... -> 358713c7...` with raw Blender Cycles render `b9ca7246...` preserved in [`docs/paper/drafts_p3_p5/BLENDER_HERO_EXACT_CAD_RECEIPT_20261019.json`](docs/paper/drafts_p3_p5/BLENDER_HERO_EXACT_CAD_RECEIPT_20261019.json)) colored by the 5-map percentile composite demand index in `[0, 1]` over the retained `25,209`-triangle plate mesh.*

![All-record population scatter and upper-score detail across separate search regimes: Panel A1 (802x440 asinh(fit) scatter over [-215, 1.20]) and Panel A2 (802x260 linear detail over [-0.60, +1.16]) stacked across upper width, Panel B lower-left (Run 1: 1,984 scored, 0 unknown), and Panel C lower-right (Screening: 2,259 scored + 1,390 unknown band)](docs/paper/figures/population-latest.svg)

*All-record population scatter and bounded upper-score detail across separate retained search regimes (`1,954,560` native `Run 2`–`Run 11` + `1,984` historical `Run 1` + `2,259` screened = `1,958,803` numeric scored observations, plus `1,390` unscreened proposals in a separate band = `1,960,193` records, `0` excluded; cache [`docs/paper/generated/population/gen800_scatter_cache.v1.json`](docs/paper/generated/population/gen800_scatter_cache.v1.json), `sha256: 4eabe5a581dcb655b413857bfb3c28741070dc390b3ca71d744aa51f58e7a493`; full crosswalk in [`docs/paper/CROSSWALK_SUPPLEMENT.md`](docs/paper/CROSSWALK_SUPPLEMENT.md) and [`docs/paper/publication.v2.json`](docs/paper/publication.v2.json)). **Panels A1 and A2 (stacked across upper width)** plot Cohort 3 (`Run 2`–`Run 11`, **1,954,560 complete 146-column birth rows** across `801` generations `0..800`): **Panel A1** renders the full `asinh(fit)` mapping domain `[-215.0, +1.20]` on an `802×440` bin raster (`123,420` nonzero bins), and **Panel A2** renders the bounded linear detail `[-0.60, +1.16]` on an `802×260` bin raster (`727,077` in-window numeric scored points + `1,227,483` conserved in the bottom tail strip = `1,954,560` total conserved points, `98,043` nonzero bins). In Cohort 3, `754,236` rows have `legal = true` and `1,200,324` have `legal = false` (`legal` is a geometric/search legality label, not a partition by fitness sign: `610,051` `legal = true` rows have `fit < 0`, and the `-10.0 <= fit < -2.0` band is retained); the `802×440` and `802×260` rasters aggregate per-bin counts with logarithmic shading and per-bin majority legality color (`legal >= penalty`, hiding minority overlaps in shared bins). **Panel B (lower left)** plots Cohort 1 (`Run 1`, `1,984` scored births, `0` unknown: `1,455` FEM-complete, `529` not-evaluated, `446` search-legal); **Panel C (lower right)** plots Cohort 2 (`2,259` validated 1-based Stage-1 screen rows + `1,390` unscreened proposals in the explicit `Unknown` band = `3,649` proposals). Separate non-birth logs (`11` staged V2 summaries totaling `17,672` rows and `9` aborted runs totaling `216` rows) are disclosed in the figure footer as summary-only scope counts rather than plotted births.*

![Actual historical population, with FEM-complete and not-evaluated records distinguished](docs/paper/generated/population.svg)

*Cohort 1 (`Run 1`, 17-generation historical run): 1,984 retained births (`1,455` FEM-complete, `529` not-evaluated, `446` search-legal) and `2,176` four-island slot memberships (`192` elite reappearances, `12` ring migrations). The retained historical top-ranked birth (**Individual 1095**, `fit = 1.15727`) failed its later strict assessment (`cert_pass_all = false`). [Results and interpretation](docs/paper/sections/08-results.tex) · [Inputs, producer identity and hashes](docs/paper/generated/manifest.json) · [Recorded parent links](docs/paper/generated/lineage.svg) · [Full crosswalk supplement](docs/paper/CROSSWALK_SUPPLEMENT.md).*

<details>
<summary>Three-cohort population census, separately assessed exports, byte totals, SDs, and provenance bindings</summary>

| Cohort / Layer | Retained Evidence Scope | Reconciled Metrics & Provenance |
|---|---|---|
| **Cohort 3: 10-Run 800-Gen Population (`Run 2`–`Run 11`)** | **1,954,560 complete 146-col birth rows** (`195,456` per run × `10` retained runs, `801` gens `0..800`) + **32,040 summary rows** (`200` `.csv.zst` segments, `494,843,141` B compressed from **`3,811,896,749` B** raw CSV, `100%` SHA-256 round-trip verified) | **754,236 search-legal births (38.59%)**, per-run mean `75,423.6 ± 2,936.55` (sample SD `ddof=1`; pop SD `2,785.86`); run-max native fit `1.1089`–`1.1440` (mean `1.1250 ± 0.01285` sample SD); plus 11 staged summaries (`17,672` rows, max gen `708`) and 9 aborted attempts (`216` rows). Absent per-birth Stage-2 records establish missing retained assessments |
| **Cohort 1: Historical 17-Gen Genealogy (`Run 1`)** | **1,984 births** (`1,455` FEM-complete, `529` not evaluated, `446` legal, `1,965` distinct gene vectors) + **2,176 island-slot memberships** (`192` elite reappearances, `12` ring migrations at G4/G8/G12) in `19` `JEV1` binary frames (`2,207` events) | Highest native birth **Individual 1095** (gen 9, `fit = 1.15727`, display loss `-1.15727`) failed its later strict assessment (`cert_pass_all = false`) |
| **Cohort 2: Multi-Stage Screening & Retained Weighted Reporting Profile** | **3,649 birth proposals** across `57` birth files in `24` run directories (`2,259` validated 1-based joined screen rows + `1,390` explicit unscreened births; `2,247` 0-based misjoins rejected) + **6 reporting curves** (`15` generation summaries, `Run 31`–`Run 36`) | Highest **retained weighted reporting profile** entry **Individual 2002** (`W_ULTRALIGHT_v1 = 74.207299`, display loss `25.792701`, `13/26` overlapping legacy flags met, not qualified; source crosswalk in [`docs/paper/CROSSWALK_SUPPLEMENT.md`](docs/paper/CROSSWALK_SUPPLEMENT.md)) |
| **Separately Assessed Controlled Exports (`Individual 2001` & `Individual 2007`)** | Separate controlled-probe CAD evaluations (`evidence/rs_v2/w283_ultrarunner/*_cad_eval.json`, not `Run 1` births) | **Individual 2001** (`sha256: df555734...1dffc`): `W_CAD = -3.878704`, `G1 = 0.6239029999999999`, mass **`75.382 g`** (`75.1992 g` 3D solid), **11 true / 5 false** legacy gates. **Individual 2007** (`sha256: 6eb44dab...edf1d`): `W_CAD = -3.964924`, `G1 = 0.6216340416666666`, mass **`75.642 g`** (`75.5049 g` 3D solid), **11 true / 5 false** legacy gates. Both fail 260 N lateral capacity, failure-sequence ordering, local tip torsion, quadratic solid tip safety factor, and grouped requirement gates |
| **Assessed Automatic Export (`Individual 2000`) & Historical Battery** | Single assessed candidate in `automatic/leaderboard.json` (`assessed_count = 1`, `qualified_count = 0`, `champion = null`); separate from the **23-subject × 18-evaluator historical battery** (`414` cells: `221 pass`, `81 fail`, `112 blocked`) | For **Individual 2000**, independent $P_2$ solid solvers agree within `0.04%–0.05%` (`0.669` vs `0.6693` max; `0.726` vs `0.7263` mean); 3-level plate GCI estimates refused (`p = -2.9003` / oscillatory). **No physical specimen or product qualification is claimed** |

</details>

## Scientific Assessment & Scoring Architecture

Each computational assessment answers a specific geometric, numerical-mechanics, or solver-verification question. Rather than treating overlapping software flags as independent physical experiments, Jarambit organizes them by **scientific test domain**, **scoring priority**, and **normalized contribution**:

| Assessment Family & Priority Share (`W_ULTRALIGHT_v1`) | Scientific Observable & Physical / Numerical Criterion | Retained `Individual 2002` Contribution (`74.21 / 100` pts) | Retained `Individual 2000` Assessed Outcome |
|---|---|---|---|
| **1. Mass & Structural Compactness (`30.0 pts` / `30%` priority)** | 2.5D raster & 3D watertight solid mass (`g`), structural efficiency index `G1`, and target ultralight envelope (`<= 80 g` soft reference) | **`19.72 / 30.00 pts`** (`65.7%` normalized; `101.14 g` solid mass) | `124.21 g` raster / `124.20 g` 3D solid (`G1 = 0.7204`) |
| **2. Archetype Form, Holder & Ergonomic Proxies (`25.0 pts` / `25%` priority)** | Watertight single-solid topology, retention-ring bore clearance, curved-claw silhouette, and Fourier/Hu reference-shape similarity (`C5`/`C6`) | **`21.84 / 25.00 pts`** (`87.4%` normalized; `4/4` form gates met) | `1` solid, `1` shell, `4/4` form & reference-shape gates met |
| **3. Exported B-Rep Edge & Tip Geometry (`15.0 pts` / `15%` priority)** | Exported STEP/STL sharpness law along cutting edge and `0–10 mm` monotonic distal tip-point profile | **`7.65 / 15.00 pts`** (`51.0%` normalized; tip-point met, edge law failed) | Tip-point profile met; exported edge sharpness law failed |
| **4. Multi-Load FEM Response & Solver Verification (`30.0 pts` / `30%` priority)** | `180 N` / `260 N` lateral bend, `1,500 N` ring traction, `1.82 N·m` body/tip torsion, `P2` 3D solid tip submodel, regional yield sequence (`>= 1.20`), elastic stability, 3-level plate GCI, and dual-solver (`scikit-fem` vs `DOLFINx`) cross-check | **`25.00 / 30.00 pts`** (`83.3%` normalized across screening proxies; `13/26` overlapping legacy flags met, not qualified) | `10/16` legacy gates & `2/10` grouped checks met; `P2` solvers agree within `0.05%` (`SF = 0.669` vs `0.6693`); 3-level GCI refused (`p = -2.9003`); **not qualified** |

<details>
<summary>All 16 CAD & Finite-Element Diagnostic Tests (Scientific Names, Criteria, Weights, and Atlas Links)</summary>

| Scientific Test Name | Physical / Numerical Criterion & Units | Scoring Role & Normalized Priority | Atlas & Machine Crosswalk |
|---|---|---|---|
| **Watertight Single-Solid B-Rep & Mesh Topology** | Single connected solid (`solids = 1`, `shells = 1`), zero non-manifold edges, valid STEP/STL export | Hard prerequisite (`W_CAD` & `25%` form/geometry share) | [G16-01 (`cad_watertight_and_single_solid`)](docs/tests/README.md#g16-cad_watertight_and_single_solid) |
| **Retention-Ring Bore & Holder Envelope Compatibility** | Inner finger-ring diameter, bounding envelope, and fixture clearance within mechanical holder limits (`mm`) | Hard geometric gate (`25%` form/holder share) | [G16-02 (`geometric_and_holder_gates`)](docs/tests/README.md#g16-geometric_and_holder_gates) |
| **Curved-Claw & Finger-Ring Archetype Continuity** | Curvature sign, claw tip reach, and ring-to-handle bridge continuity | Hard shape gate (`25%` form/ergonomics share) | [G16-03 (`karambit_ring_and_curved_claw`)](docs/tests/README.md#g16-karambit_ring_and_curved_claw) |
| **Reference Silhouette & Contour Similarity Floor** | Normalized shape similarity score (`>= 0.70`) against reference geometric descriptors | Soft/floor aesthetic proxy (`25%` form share) | [G16-04 (`c6_aesthetic_floor`)](docs/tests/README.md#g16-c6_aesthetic_floor) |
| **Standard Lateral Cantilever Bending (`180 N`)** | Plate FEM von Mises safety factor under `180 N` transverse load (`SF >= 1.0`) | Structural response gate (`30%` response share) | [G16-05 (`cert_hiker_180n`)](docs/tests/README.md#g16-cert_hiker_180n) |
| **Heavy Lateral Cantilever Bending (`260 N`)** | Two-level plate FEM lateral capacity (`>= 260 N`, `SF >= 1.0`) | Primary lateral strength gate (`30%` response share) | [G16-06 (`cert_hunter_260n`)](docs/tests/README.md#g16-cert_hunter_260n) |
| **Refined-Mesh Lateral Bending (`260 N`, `0.0625 mm²`)** | High-resolution `0.0625 mm²` plate refinement check under `260 N` lateral load (`SF >= 1.0`) | Strict discretization gate (`30%` response share) | [G16-07 (`strict_hunter_260n`)](docs/tests/README.md#g16-strict_hunter_260n) |
| **In-Plane Finger-Ring Traction (`1,500 N`)** | In-plane membrane FEM stress capacity under `1,500 N` ring pull (`SF >= 1.0`) | Primary tensile/ring gate (`30%` response share) | [G16-08 (`cert_ring_pull_1500n`)](docs/tests/README.md#g16-cert_ring_pull_1500n) |
| **Global Blade-Body Torsion (`1.82 N·m`)** | Plate twisting response across primary blade body (`SF >= 1.0` at `1.82 N·m`) | Structural torsion gate (`30%` response share) | [G16-09 (`twist_body_1p82nm`)](docs/tests/README.md#g16-twist_body_1p82nm) |
| **Distal Tip-Zone Plate Torsion (`1.82 N·m`)** | Coarse/medium local tip-region plate torsion check (`SF >= 1.0` at `1.82 N·m`) | Local tip torsion screen (`30%` response share) | [G16-10 (`twist_tip_only_1p82nm`)](docs/tests/README.md#g16-twist_tip_only_1p82nm) |
| **Refined Tip-Submodel Plate Torsion (`4–15 mm`, `1.82 N·m`)** | Refined `4–15 mm` distal submodel torsion safety factor (`SF >= 1.10`) | Strict local torsion gate (`30%` response share) | [G16-11 (`twist_tip_submodel_strict_1p82nm`)](docs/tests/README.md#g16-twist_tip_submodel_strict_1p82nm) |
| **Quadratic 3D Tetrahedral Solid Tip Submodel (`P2`, `1.82 N·m`)** | 10-node `P2` tetrahedral solid elasticity solve (`91,008` tets, `416,583` DOF; `SF >= 1.25`) | 3D solid stress verification (`30%` response share) | [G16-12 (`solid_tip_3d_p2_1p82nm`)](docs/tests/README.md#g16-solid_tip_3d_p2_1p82nm) |
| **Elastic Support Stiffness & Linear Buckling Eigenvalue** | Winkler-supported lateral sequence and linear buckling eigenvalue (`lambda_crit >= 2.0`) | Stability & boundary-support gate (`30%` response share) | [G16-13 (`side_a4_buckling_and_support`)](docs/tests/README.md#g16-side_a4_buckling_and_support) |
| **Regional Yield-Order Progression Ratio** | Ratio of handle/ring safety factor to blade/tip safety factor (`seq_ratio >= 1.20`) | Fail-safe progression gate (`30%` response share) | [G16-14 (`failure_sequence_1p2`)](docs/tests/README.md#g16-failure_sequence_1p2) |
| **Three-Level Mesh-Refinement Convergence Study (`GCI`)** | Roache 3-level Grid Convergence Index (`4.0`, `1.0`, `0.25 mm²`) monotone positive order check | Numerical discretization verification (`30%` response share) | [G16-15 (`plate_gci_valid_3level`)](docs/tests/README.md#g16-plate_gci_valid_3level) |
| **Independent Dual-Solver Cross-Check (`<= 2%`)** | Relative agreement between `scikit-fem` and `DOLFINx` `P2` solid solves on identical mesh (`<= 2.0%`) | Independent solver verification (`30%` response share) | [G16-16 (`dolfinx_cross_check_2pct`)](docs/tests/README.md#g16-dolfinx_cross_check_2pct) |

</details>

<details>
<summary>All 10 Grouped Requirement Comparisons (Scientific Names, Scope, and Atlas Links)</summary>

| Scientific Comparison Group | Combined Physical / Numerical Requirement | Priority & Qualification Role | Atlas & Machine Crosswalk |
|---|---|---|---|
| **Archetype Geometry, Topology & Silhouette Group** | Joint satisfaction of watertight solid, holder envelope, ring/claw continuity, and shape floor | Hard geometric admissibility (`25%` form share) | [RSV2-01 (`aesthetic_and_geometric`)](docs/tests/README.md#rsv2-aesthetic_and_geometric) |
| **Combined Primary Load-Case Safety Floors** | Joint satisfaction of `180 N` lateral, `260 N` lateral, and `1,500 N` ring-pull floors | Primary multi-load floor (`30%` response share) | [RSV2-02 (`floors_828_joint`)](docs/tests/README.md#rsv2-floors_828_joint) |
| **Global Body & Refined Tip-Submodel Torsion Group** | Joint satisfaction of global body torsion and strict `4–15 mm` tip-submodel torsion | Combined torsion floor (`30%` response share) | [RSV2-03 (`twist_floor_body_and_tipsub_strict`)](docs/tests/README.md#rsv2-twist_floor_body_and_tipsub_strict) |
| **3D Quadratic Solid Tip Safety Factor Group** | `P2` 3D solid tip safety factor (`SF >= 1.25`) with independent solver verification | 3D local solid requirement (`30%` response share) | [RSV2-04 (`solid_tip_sf`)](docs/tests/README.md#rsv2-solid_tip_sf) |
| **Refined `0.0625 mm²` Lateral Capacity Group** | Strict fine-mesh lateral bending capacity (`>= 260 N`, `SF >= 1.0`) | High-resolution lateral requirement (`30%` response share) | [RSV2-05 (`strict_hunter_lateral`)](docs/tests/README.md#rsv2-strict_hunter_lateral) |
| **Hard Regional Yield-Sequence Ordering Group** | Strict regional yield-sequence ratio (`seq_ratio >= 1.20`) | Fail-safe yield-order requirement (`30%` response share) | [RSV2-06 (`seq_hard`)](docs/tests/README.md#rsv2-seq_hard) |
| **Human-Maximum Peak Load Envelope Group** | Peak ergonomic/human-maximum lateral, pry, and extraction load factors (`SF >= 1.0`) | Peak overload screen (`30%` response share) | [RSV2-07 (`human_max`)](docs/tests/README.md#rsv2-human_max) |
| **Cyclic High-Cycle Fatigue Endurance Screen** | Goodman/Soderberg endurance safety factor (`fat_sf >= 1.15`) under cyclic loading | Cyclic durability screen (`30%` response share) | [RSV2-08 (`fatigue`)](docs/tests/README.md#rsv2-fatigue) |
| **Exported B-Rep Cutting-Edge Sharpness Law** | Continuous bevel-angle and edge-radius bounds sampled directly on exported STEP/STL | Exported edge geometry (`15%` geometry share) | [RSV2-09 (`gate_s_sharp_law`)](docs/tests/README.md#rsv2-gate_s_sharp_law) |
| **Exported `0–10 mm` Distal Tip-Point Profile** | Monotonic cross-sectional area and thickness taper over the distal `0–10 mm` B-Rep tip | Exported tip geometry (`15%` geometry share) | [RSV2-10 (`gate_s_tip_point_brep`)](docs/tests/README.md#rsv2-gate_s_tip_point_brep) |

</details>

<details>
<summary>All 18 Historical Battery Evaluators (`23 Subjects × 18 Evaluators = 414 Cells`)</summary>

| Scientific Evaluator Name | Verification Domain & Retained Battery Tally (`23` Subjects) | Atlas & Machine Crosswalk |
|---|---|---|
| **Analytic Beam & Plate Benchmark Controls** | Closed-form analytic software verification controls (`23 pass`) | [`build.analytic`](docs/tests/README.md#battery-build.analytic) |
| **Grid Convergence Index (`GCI`) Arithmetic Controls** | Roache order-of-accuracy and refusal arithmetic verification (`23 pass`) | [`build.gci_contracts`](docs/tests/README.md#battery-build.gci_contracts) |
| **Fail-Closed Claim & Certificate Guard Controls** | Automated rejection of unsupported or retracted qualification claims (`23 pass`) | [`build.claims`](docs/tests/README.md#battery-build.claims) |
| **Independent `DOLFINx` Solver Environment Capability** | Availability and version pin of independent `DOLFINx` v0.9.0 solver (`23 pass`) | [`build.dolfinx`](docs/tests/README.md#battery-build.dolfinx) |
| **Export Coordinate Units & Dimensional Scale** | Millimeter unit consistency across genome, STEP, and STL (`22 pass`, `1 blocked`) | [`geom.unit`](docs/tests/README.md#battery-geom.unit) |
| **Single Connected Watertight Solid Topology** | Connected-component and manifold shell check (`22 pass`, `1 blocked`) | [`geom.one_solid`](docs/tests/README.md#battery-geom.one_solid) |
| **2.5D Raster vs. 3D Solid Mass Reconciliation** | Mass conservation check between plate quadrature and 3D tetrahedral volume (`22 pass`, `1 blocked`) | [`geom.mass`](docs/tests/README.md#battery-geom.mass) |
| **Content-Addressed Geometry Hash Identity** | SHA-256 digest binding across genome, STEP, STL, and mesh artifacts (`22 pass`, `1 blocked`) | [`geom.pin`](docs/tests/README.md#battery-geom.pin) |
| **Primary Multi-Case Plate Bending & Traction Loads** | Combined plate FEM lateral (`180 N`, `260 N`) and ring (`1,500 N`) evaluation (`14 pass`, `8 fail`, `1 blocked`) | [`plate.loads`](docs/tests/README.md#battery-plate.loads) |
| **Regional Yield-Order Progression Proxy** | Handle/ring vs. blade/tip safety-factor ratio (`13 pass`, `9 fail`, `1 blocked`) | [`plate.seq`](docs/tests/README.md#battery-plate.seq) |
| **Cyclic Fatigue Endurance Screen (`L7`)** | Model-based cyclic stress endurance ratio (`0 pass`, `22 fail`, `1 blocked`) | [`plate.l7`](docs/tests/README.md#battery-plate.l7) |
| **Candidate Plate Mesh-Refinement Study** | Candidate-specific 3-level mesh convergence status (`1 pass`, `21 fail`, `1 blocked`) | [`plate.gci`](docs/tests/README.md#battery-plate.gci) |
| **Historical Side-Report Record A1 (In-Plane Variant)** | Legacy auxiliary in-plane screening record completeness (`0 pass`, `0 fail`, `23 blocked`) | [`side.a1`](docs/tests/README.md#battery-side.a1) |
| **Historical Side-Report Record A2 (Torsion Variant)** | Legacy auxiliary torsion screening record completeness (`0 pass`, `0 fail`, `23 blocked`) | [`side.a2`](docs/tests/README.md#battery-side.a2) |
| **Historical Side-Report Record A3 (Submodel Variant)** | Legacy auxiliary submodel record completeness (`0 pass`, `0 fail`, `23 blocked`) | [`side.a3`](docs/tests/README.md#battery-side.a3) |
| **Winkler Support Stiffness & Buckling Report (`A4`)** | Supported lateral sequence and linear buckling eigenvalue record (`1 pass`, `0 fail`, `22 blocked`) | [`side.a4`](docs/tests/README.md#battery-side.a4) |
| **3D Quadratic Solid Tip Record Completeness** | Presence and schema completeness of `P2` 3D solid tip solve (`12 pass`, `0 fail`, `11 blocked`) | [`solid.tip`](docs/tests/README.md#battery-solid.tip) |
| **Grouped Requirement Suite (`RS-v2`) Verdict** | All-hard joint requirement verdict across all 10 grouped checks (`0 pass`, `21 fail`, `2 blocked`) | [`gate.rs_v2`](docs/tests/README.md#battery-gate.rs_v2) |

</details>

[All 178 science definitions, challenges and oracles](docs/tests/science.md) · [Reproducibility crosswalk & supplement](docs/paper/CROSSWALK_SUPPLEMENT.md) · [Historical battery manifest](evidence/cad_battery/manifest.json) · [Separate automatic proof](evidence/evolve_certifiable/automatic/leaderboard.json)

## What it is

Rust CAD and evaluation core, with a Vite + TypeScript + React evidence studio. The current automatic lane ranks candidates using weighted fitness and sends a selected subset to independent assessment. Other lanes retain their own algorithms and score conventions. [Genome and methodology](docs/paper/sections/03-genome.tex) · [Existing operational guide](docs/EVOLUTION_CERTIFICATION.md).

**Score direction:** retained `W_ULTRALIGHT_v1` is higher-is-better. At the manuscript's source baseline, native Stage 1, native Stage 2 and the reporting scorer used different proxies/constants. [PR #135](https://github.com/neverhuman/jarambit/pull/135) subsequently changed Stage-2 ranking to the canonical score; the retained cohorts keep their original score history and incomplete lineage. The desired future lower-is-better convention remains a separate product direction. [Full interpretation](docs/paper/sections/05-fitness.tex).

## Retained history and corrections

The following generated panels preserve their original model/requirement scope. Historical screen records do not establish a current qualified design. Certification claims from v1.0.0 remain withdrawn; the separate automatic proof records `champion=null`.

<details>
<summary>Historical generated records, correction table, figures and release details</summary>

<!-- GEN:HEAD:BEGIN -->
[![version](https://img.shields.io/badge/version-v2.0.0-ff6f00?style=for-the-badge)](https://github.com/neverhuman/jarambit/releases/tag/v2.0.0)
[![historical record](https://img.shields.io/badge/historical%20record-w282__p14__c01-2e7d32?style=for-the-badge)](docs/ENGINEERING_GATES_AND_EVOLUTION.md)
[![mass](https://img.shields.io/badge/mass-75.568%20g%20%2875.323%20g%203D%29-00897b?style=for-the-badge)](docs/WHITEPAPER.md)
[![G1](https://img.shields.io/badge/G1-0.878210-7b1fa2?style=for-the-badge)](docs/GLOBAL_METRIC.md)
[![W_CAD](https://img.shields.io/badge/W__CAD-%2B1.7790%20%2816%2F16%20gates%29-1565c0?style=for-the-badge)](docs/ENGINEERING_GATES_AND_EVOLUTION.md)
[![tip×SPG](https://img.shields.io/badge/tip%C3%97SPG-4.886-0277bd?style=for-the-badge)](docs/WHITEPAPER.md)
[![status](https://img.shields.io/badge/status-engineering%20provisional%20%C2%B7%20not%20promoted-c62828?style=for-the-badge)](docs/PROVISIONAL.md)

**Historical 16-gate screen record `w282_p14_c01` — 75.568 g (75.323 g 3D solid) · G1 0.878210 (`COMPLETE`) · W_CAD +1.7790 (16/16 engineering gates pass) · strict lateral 262.6 N (SF 1.010) · strict twist SF 1.125 · cert seq 1.201 · 3D P2 solid-tip SF 1.4093 × SPG 3.467 = 4.886 · Ti-6Al-4V 5.9 mm** *(historical Wave-27 reference `g3_tip+00_t60_tt3`: 84.80 g, G1 0.9804, seq 0.959 < 1.20)*

**v1.0.0 certification claims withdrawn (#75–#81).** Neither Wave 283 candidate passes the fixed pipeline. The exported geometry and all requirement thresholds are unchanged. Native-only solid solves are unverified; the results below use actual scikit-fem P2 and DOLFINx solves on the same mesh.

| candidate | raster / solid g | strict lateral N | cert seq | solid-tip SF | G1 | W_CAD | RS-v2 |
|---|---:|---:|---:|---:|---:|---:|---|
| `w283_p14_c01` | 75.382 / 75.199 | 176.720 | 0.7436 | 0.9577 | 0.623888 | -3.878704 | FAIL |
| `w283_p10_c02` | 75.642 / 75.505 | 176.763 | 0.7179 | 0.9578 | 0.621632 | -3.964924 | FAIL |

Source: `evidence/rs_v2/w283_ultrarunner/*_{cad_eval,rs,solid_tip}.json`; raw solver outputs and exact meshes are retained in `evidence/cert_integrity/recertified/`. The retracted certificates are preserved under `evidence/cert_integrity/v1.0.0-retracted/`. Lateral strength, failure sequence, human-maximum loads, fatigue, exported edge sharpness and full 0–10 mm point monotonicity remain blockers; the bore has no verified required fillet. Historical 16-gate screen results below do not establish an RS-v2 all-hard pass or manufacturing promotion. G1 above uses the full-precision RS inputs in each battery record; CAD-eval JSON retains its rounded D1 inputs.
<!-- GEN:HEAD:END -->

<!-- RS:BEGIN docs/wave27_diversity/plates/wave27_D1_index.json g3_tip+00_t60_tt3_D1 -->
RS-karambit-v1: screen PASS · promotion FAIL (seq 0.959 < 1.2) · soft: mass 84.80 g over 80 g target (G penalty) · human aesthetic sign-off pending · not promoted
<!-- RS:END -->

<!-- GEN:FIGS:BEGIN -->
📄 [White paper](docs/WHITEPAPER.md) · 📐 [Engineering gates & evolution paper](docs/ENGINEERING_GATES_AND_EVOLUTION.md) · 📋 [Work plan](docs/WORKPLAN.md) · 📊 [Live status](STATUS.md) · ⚖️ [Patent draft (not filed)](docs/PATENT_DRAFT.md) · 📦 [Release v2.0.0](https://github.com/neverhuman/jarambit/releases/tag/v2.0.0) · 🗺️ [Technology map](TECHNOLOGY_MAP.md)

<img src="docs/figures/whitepaper/fig_readme_mass_vs_wave.png" width="96%" alt="Historical pre-#82 model: selected-candidate mass vs wave w20 through w282 from committed evidence JSON, with historical screen record w282_p14_c01 at 75.568 g (75.323 g 3D) labelled"/>

*Historical (pre-#82 model) selected-candidate mass vs. wave (`w20` through `w282`, generated by `ops/render/whitepaper_hero_figs.py` and verified by `ops/ci/verify_figures.py`): historical 16-gate screen record **`w282_p14_c01` at `75.568 g` (`75.323 g` 3D solid), `G1 = 0.878210`, `W_CAD = +1.7790`**.*

<img src="docs/figures/whitepaper/fig_web_studio.png" width="96%" alt="Corrected Wave 283 certificate scorecard"/>

*Interactive 3D CAD & Gate Scorecard Web Studio (`web/` — Vite + TypeScript + React + Three.js, exported by `./target/release/jarambit export-web-data --out-dir web/public/data`): 3D STL/STEP inspection, measured 16-gate `W_CAD_v2` breakdown, failed `RS-karambit-v2` scorecard, and 23-subject × 18-evaluator CAD battery matrix. Wave 283 is rejected.*
<!-- GEN:FIGS:END -->

> **SWITCHBACK Rev C.9.3 (rc3, current RC within the archived C.9 study)** — [`designs/switchback/C9.3/`](designs/switchback/C9.3/README.md): B-rep STEP, drawings and FEM plates from the hiking-karambit study; all C.9-study gates V1–V8 pass with ≥ 5 % margin (governor V3 spine SF 1.597, 66.773 g). C.9.3 is a release candidate, not the design of record: the gates are the C.9 study set, not an RS-karambit-v2 verdict from this repo, and nothing is machined or tested.
>
> SWITCHBACK Rev C.9.2 (rc2, previous RC within the archived C.9 study) — [`designs/switchback/C9.2/`](designs/switchback/C9.2/README.md): release candidate, not the design of record; superseded by C.9.3 (its V6 lateral-pry margin was 1.9 %, below the 5 % promotion rule).

<!-- GEN:RELEASE:BEGIN -->
## Release v2.0.0 & Documentation

- **Historical 16-gate screen record (`w282_p14_c01`):** `75.568 g` 2.5D raster / `75.323 g` 3D watertight solid, locked `G1 = 0.878210` (`COMPLETE`), `W_CAD = +1.7790` (`16/16` engineering gates pass: strict 0.0625 mm² lateral `262.6 N` / `SF = 1.010`, strict 4–15 mm tip-submodel twist `SF = 1.125`, certified failure sequence `seq_ratio = 1.201 >= 1.20`, 3D P2 tetrahedral solid-tip `SF = 1.4093` with `0.021%` `dolfinx` v0.9.0 vs. `skfem` agreement, and `side.a4` Winkler+bore supported `seq = 4.042`).
- **Historical 16-gate `W_CAD_v2` mass progression (`w274` → `w282`):** `w274_s2905_c01` (`84.979 g` / `85.021 g` 3D, `W_CAD = +1.4634`) → `w275_s2909_c01` (`81.774 g` / `81.804 g` 3D, `W_CAD = +1.5589`) → `w276_s2913_c01` (`79.995 g` / `80.050 g` 3D, `W_CAD = +1.6381`) → `w277_s2919_c01` (`79.509 g` / `79.656 g` 3D, `W_CAD = +1.6604`) → `w278_s2922_c01` (`78.580 g` / `78.597 g` 3D, `W_CAD = +1.7021`) → `w279_s2928_c01` (`77.535 g` / `77.414 g` 3D, `W_CAD = +1.7212`) → `w280_s2931_c01` (`76.965 g` / `76.834 g` 3D, `W_CAD = +1.7432`) → `w281_s2932_c01` (`75.911 g` / `75.695 g` 3D, `W_CAD = +1.7916`) → `w282_p10_c02` (`75.836 g` / `75.616 g` 3D, `W_CAD = +1.7978`) → `w282_p14_c01` (`75.568 g` / `75.323 g` 3D, `W_CAD = +1.7790`).
- **Battery total across 23 subjects × 18 evaluators (414 cells):** `221 pass`, `81 fail`, `112 blocked`.
- **Scientific assurance catalog (`T00`–`T19`):** **178 definitions with 356 required paired-control slots** in the committed catalog; run the software controls via `python3 -B ops/ci/science_suite.py`. Catalog entries do not establish execution.
- **Full technical papers, tables, and evidence:**
  - [White paper (`docs/WHITEPAPER.md`)](docs/WHITEPAPER.md) — page-1 summary, `w20`–`w282` mass curve, market comparison, `w282_p14_c01` 16-gate margin breakdown, and T1→T2 calibration.
  - [Engineering gates & evolution paper (`docs/ENGINEERING_GATES_AND_EVOLUTION.md`)](docs/ENGINEERING_GATES_AND_EVOLUTION.md) — complete 23-subject × 18-evaluator CAD battery matrix, Margin-vs-Mass Pareto table, `RS-karambit-v2` gate tables, and 3D CAD/FEM gallery.
  - [Master work plan (`docs/WORKPLAN.md`)](docs/WORKPLAN.md) · [Live status (`STATUS.md`)](STATUS.md) · [Wave log (`docs/WAVE_LOG.md`)](docs/WAVE_LOG.md) · [Release notes (`docs/RELEASES.md`)](docs/RELEASES.md) · [Global metric `G1` (`docs/GLOBAL_METRIC.md`)](docs/GLOBAL_METRIC.md) · [Provisional record (`docs/PROVISIONAL.md`)](docs/PROVISIONAL.md) · [Technology map (`TECHNOLOGY_MAP.md`)](TECHNOLOGY_MAP.md).
<!-- GEN:RELEASE:END -->

</details>

## Install

Build the Rust workspace with the toolchain pinned in `rust-toolchain.toml`. The evidence studio uses Node/npm; independent solver and CAD/render bindings require the runtimes listed in [PYTHON_EXCEPTIONS.md](PYTHON_EXCEPTIONS.md). [Stack migration policy](PY_TO_RUST_PLAN.md).

```sh
cargo build --release --locked
```

## Quick start

**Read-only retained-evidence projection (no live CAD/FEM or `score_cad_native` recomputation):**

```sh
cargo run --release --locked --manifest-path crates/paper-artifacts/Cargo.toml --example paper_artifacts -- --check
cargo run --release --locked --manifest-path crates/paper-artifacts/Cargo.toml --example publication_v2_audit
npm --prefix web ci
npm --prefix web test
npm --prefix web run build
```

**Live Web Studio regeneration (invokes live `score_cad_native` / STL export; distinct from read-only retained-evidence projection):**

```sh
./target/release/jarambit export-web-data --out-dir web/public/data
```

A native screen is not an independent certification. The older Monte Carlo lane uses a beam surrogate by default; plate FEM is opt-in with `JARAMBIT_FEM=1`. See the [assessment guide](docs/EVOLUTION_CERTIFICATION.md) for runtime scope and the [paper build](docs/paper/README.md#format-and-build) for reproducible documentation artifacts.

## Research and project documentation

[New white paper](docs/paper/README.md) · [Engineering spec](docs/PAPER_AND_README_ENGINEERING_SPEC.md) · [Living technical report](docs/WHITEPAPER.md) · [Detailed engineering reference](docs/ENGINEERING_GATES_AND_EVOLUTION.md) · [Work plan](docs/WORKPLAN.md) · [Status](STATUS.md) · [Release notes](docs/RELEASES.md) · [Technology map](TECHNOLOGY_MAP.md)

The hand-built C.10d concept film and its cross-revision field/weight disclosures remain in [the pre-cleanup README at the audited snapshot](https://github.com/neverhuman/jarambit/blob/009f5ac520fa3afe8ed8025230eef604dbb7f922/README.md#concept-render-hand-built-design-not-pipeline-output). It is separate from evolved, assessed evidence.

<details>
<summary>Historical market comparison and original source caveats</summary>

<!-- MARKET:CENSUS:BEGIN -->
## Market weight comparison

The source-audited [October 13, 2026 census](docs/market/2026-10-13/README.md) consolidates eight supplied CSVs plus an additional qualifying polymer model. **119 identifiable entries meet the published 3.375 in blade minimum; 77 have comparable verified published knife weights.** Color-only duplicates are consolidated; genuine material variants and distinct custom examples remain separate. Missing weights and unresolved candidates stay visible in the evidence tables.

**Committed genome-pipeline Stage-2 designs: `w282_p14_c01` at 75.568 g (75.323 g 3D solid, historical 16-gate screen record) and `w411c3_r02_p10_c12` at 101.139 g (101.138 g 3D solid, `W_ULTRALIGHT_v1 = 74.207299`, committed ultralight-v1 Stage-2 leader).** `w282_p14_c01` is nominally lighter than 72 of 77 plotted entries (93.5%) and 52.6% below their 159.32 g median; `g3_tip+00_t60_tt3` (84.80 g, lighter than 71 of 77 or 92.2%, 46.8% below median) remains the historical design of record; and `w411c3_r02_p10_c12` is lighter than 66 of 77 plotted entries (85.7%), 36.5% below median. TALIM titanium lists 51.03 g, so this does not support a lightest-overall claim. None is qualified: `w282_p14_c01` passes the 16 legacy gates but fails RS-karambit-v2 (fatigue, Gate-S sharpness and human-max fail; tip-point unverified); 0 candidates pass RS-karambit-v2 all-hard (champion = null). Stage-2 `W_ULTRALIGHT_v1` is for ranking only, not qualification. All are unmanufactured and unpromoted. Listed commercial lengths use different measurement conventions, and material/weight comparisons establish no performance equivalence.

[Catalog by material](docs/market/2026-10-13/qualified_by_material.csv) · [Complete normalized census](docs/market/2026-10-13/census.csv) · [Exact-grade summary](docs/market/2026-10-13/material_summary.csv) · [Missing/conditional candidate queue](docs/market/2026-10-13/review_queue.csv)
<!-- MARKET:CENSUS:END -->

</details>
