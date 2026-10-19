# Jarambit

**Intelligent CAD research: evolutionary geometry, multi-model numerical assessment, and traceable engineering evidence.**

Jarambit connects a 128-gene CAD representation to plate/membrane FEM, a quadratic 3D solid submodel, convergence studies, separate-solver comparisons, and scientific-assurance controls. The [test atlas](docs/tests/README.md) explains every named CAD diagnostic and evaluator, with implementation and retained-evidence links.

[Read the new IEEE-style white paper](docs/paper/README.md) · [Explore all tests](docs/tests/README.md) · [Detailed engineering spec](docs/PAPER_AND_README_ENGINEERING_SPEC.md) · [Architecture](TECHNOLOGY_MAP.md)

## Engineering assessments at a glance

| Layer | What is available | What it means |
|---|---|---|
| CAD diagnostics | **16 legacy predicates** | Geometry, modeled response, support/stability, convergence and solver consistency |
| Requirement groups | **10 overlapping RS-v2 groups** | Grouped comparisons that reuse some of the same observations |
| Historical CAD battery | **18 evaluators × 23 subjects = 414 retained cells** | Software, geometry, numerical findings and result-completeness records |
| Scientific assurance | **178 definitions across 20 T00–T19 families; 356 required control slots** | A frozen catalog; definitions do not prove execution |
| Physical validation | **Not established by this evidence** | No qualified product or manufacturing promotion is claimed |

**The engineering depth is in the models, checks, and evidence links—not a combined test count.** All historical battery cells say `fresh_this_lane: false`. Stored outcomes are 221 pass, 81 fail, 112 blocked; four shared software columns account for 92 passes. `solid.tip` passes mean complete records, not favorable strength findings. [Read status meanings and every evaluator](docs/tests/README.md#battery).

[See all 414 stored cells](docs/paper/generated/battery-matrix.svg) · [Cell-level sources and selectors](docs/paper/generated/battery-matrix.json). The matrix preserves blocked records, original subject roles and the two different retained source-revision fields. It is a categorical evidence map, not a stress heatmap or a count of independent physical tests.

[Assessment visual coverage](docs/tests/visuals.md) lists every subsection's image, visual kind and source receipt, including missing geometry/export views and unavailable spatial fields. A scalar plot or status map does not supply a missing FEM heatmap.

![Actual historical population, with FEM-complete and not-evaluated records distinguished](docs/paper/generated/population.svg)

*Actual historical w282 data: 1,984 retained births, including 1,455 FEM-complete and 529 not-evaluated records; 446 are legal under the recorded search. The plot distinguishes those states and preserves all stored fitness values. Fitness includes legality penalties and contextual novelty. The highest search score did not pass its later strict assessment; the producer's named exported examples came from separate controlled probes. These historical findings do not establish current qualification. [Results and interpretation](docs/paper/sections/08-results.tex) · [Inputs, producer identity and hashes](docs/paper/generated/manifest.json).*

[See the highest stored score's recorded parent links](docs/paper/generated/lineage.svg) · [Birth and assessment selectors](docs/paper/generated/lineage.json). The two-hop view preserves reused parent IDs and its cutoff; it does not bind those births to the separate cover export.

## A link to every CAD assessment

Each entry explains the question, observation family, implementation, related software controls, retained evidence, and interpretation limits. These inventories overlap; they are not independent physical experiments.

<details>
<summary>All 16 legacy CAD diagnostics</summary>

| Exact identity | Explanation |
|---|---|
| `cad_watertight_and_single_solid` | [G16-01: Export topology and identity](docs/tests/README.md#g16-cad_watertight_and_single_solid) |
| `geometric_and_holder_gates` | [G16-02: Geometry and holder compatibility](docs/tests/README.md#g16-geometric_and_holder_gates) |
| `karambit_ring_and_curved_claw` | [G16-03: Shape and archetype](docs/tests/README.md#g16-karambit_ring_and_curved_claw) |
| `c6_aesthetic_floor` | [G16-04: Reference-shape comparison](docs/tests/README.md#g16-c6_aesthetic_floor) |
| `cert_hiker_180n` | [G16-05: Lateral-response comparison](docs/tests/README.md#g16-cert_hiker_180n) |
| `cert_hunter_260n` | [G16-06: Additional lateral comparison](docs/tests/README.md#g16-cert_hunter_260n) |
| `strict_hunter_260n` | [G16-07: Refined lateral observation](docs/tests/README.md#g16-strict_hunter_260n) |
| `cert_ring_pull_1500n` | [G16-08: Ring-traction response](docs/tests/README.md#g16-cert_ring_pull_1500n) |
| `twist_body_1p82nm` | [G16-09: Body-region torsion](docs/tests/README.md#g16-twist_body_1p82nm) |
| `twist_tip_only_1p82nm` | [G16-10: Local plate torsion](docs/tests/README.md#g16-twist_tip_only_1p82nm) |
| `twist_tip_submodel_strict_1p82nm` | [G16-11: Refined local torsion](docs/tests/README.md#g16-twist_tip_submodel_strict_1p82nm) |
| `solid_tip_3d_p2_1p82nm` | [G16-12: Local quadratic solid result](docs/tests/README.md#g16-solid_tip_3d_p2_1p82nm) |
| `side_a4_buckling_and_support` | [G16-13: Support and stability](docs/tests/README.md#g16-side_a4_buckling_and_support) |
| `failure_sequence_1p2` | [G16-14: Regional yield-order proxy](docs/tests/README.md#g16-failure_sequence_1p2) |
| `plate_gci_valid_3level` | [G16-15: Mesh and observable consistency](docs/tests/README.md#g16-plate_gci_valid_3level) |
| `dolfinx_cross_check_2pct` | [G16-16: Separate-solver consistency](docs/tests/README.md#g16-dolfinx_cross_check_2pct) |

</details>

<details>
<summary>All ten RS-v2 grouped comparisons</summary>

| Exact identity | Explanation |
|---|---|
| `aesthetic_and_geometric` | [RSV2-01: Shape and geometry](docs/tests/README.md#rsv2-aesthetic_and_geometric) |
| `floors_828_joint` | [RSV2-02: Combined numerical response](docs/tests/README.md#rsv2-floors_828_joint) |
| `twist_floor_body_and_tipsub_strict` | [RSV2-03: Body and local torsion](docs/tests/README.md#rsv2-twist_floor_body_and_tipsub_strict) |
| `solid_tip_sf` | [RSV2-04: Local solid response](docs/tests/README.md#rsv2-solid_tip_sf) |
| `strict_hunter_lateral` | [RSV2-05: Refined lateral response](docs/tests/README.md#rsv2-strict_hunter_lateral) |
| `seq_hard` | [RSV2-06: Regional yield order](docs/tests/README.md#rsv2-seq_hard) |
| `human_max` | [RSV2-07: Additional response and geometry](docs/tests/README.md#rsv2-human_max) |
| `fatigue` | [RSV2-08: Fatigue-model assessment](docs/tests/README.md#rsv2-fatigue) |
| `gate_s_sharp_law` | [RSV2-09: Exported edge measurements](docs/tests/README.md#rsv2-gate_s_sharp_law) |
| `gate_s_tip_point_brep` | [RSV2-10: Exported point-profile measurements](docs/tests/README.md#rsv2-gate_s_tip_point_brep) |

</details>

<details>
<summary>All 18 CAD battery evaluators</summary>

| Exact identity | Explanation |
|---|---|
| `build.analytic` | [Analytic software controls](docs/tests/README.md#battery-build.analytic) |
| `build.gci_contracts` | [Convergence arithmetic controls](docs/tests/README.md#battery-build.gci_contracts) |
| `build.claims` | [Claim-handling controls](docs/tests/README.md#battery-build.claims) |
| `build.dolfinx` | [Separate-solver capability](docs/tests/README.md#battery-build.dolfinx) |
| `geom.unit` | [Export units](docs/tests/README.md#battery-geom.unit) |
| `geom.one_solid` | [Solid topology](docs/tests/README.md#battery-geom.one_solid) |
| `geom.mass` | [Mass observation](docs/tests/README.md#battery-geom.mass) |
| `geom.pin` | [Geometry identity](docs/tests/README.md#battery-geom.pin) |
| `plate.loads` | [Plate response](docs/tests/README.md#battery-plate.loads) |
| `plate.seq` | [Sequence proxy](docs/tests/README.md#battery-plate.seq) |
| `plate.l7` | [Fatigue screen](docs/tests/README.md#battery-plate.l7) |
| `plate.gci` | [Candidate mesh study](docs/tests/README.md#battery-plate.gci) |
| `side.a1` | [Legacy side-report record A1](docs/tests/README.md#battery-side.a1) |
| `side.a2` | [Legacy side-report record A2](docs/tests/README.md#battery-side.a2) |
| `side.a3` | [Legacy side-report record A3](docs/tests/README.md#battery-side.a3) |
| `side.a4` | [Support and stability report](docs/tests/README.md#battery-side.a4) |
| `solid.tip` | [Solid-result completeness](docs/tests/README.md#battery-solid.tip) |
| `gate.rs_v2` | [Grouped requirement finding](docs/tests/README.md#battery-gate.rs_v2) |

</details>

[All 178 science definitions, challenges and oracles](docs/tests/science.md) · [Historical battery manifest](evidence/cad_battery/manifest.json) · [Separate automatic proof](evidence/evolve_certifiable/automatic/leaderboard.json)

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

Start by reading the retained evidence and test explanations. Build the evidence studio to inspect existing records:

```sh
./target/release/jarambit export-web-data --out-dir web/public/data
npm --prefix web ci
npm --prefix web test
npm --prefix web run build
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
