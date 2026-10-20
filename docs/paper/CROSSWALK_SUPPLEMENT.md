# Source and Evidence Crosswalk

The [main paper](main.pdf) and [standalone technical supplement](supplement.pdf) use public integer identifiers. This navigation table connects principal examples to repository accessions. Full machine-readable identities and hashes are governed by [`data/public_ids.v1.json`](data/public_ids.v1.json), [`data/selection_receipts.v1.json`](data/selection_receipts.v1.json), [`data/score_profiles.v1.json`](data/score_profiles.v1.json), and [`publication.v2.json`](publication.v2.json).

## Identifier navigation

| Public Label | Internal Alias / Package ID | Cohort & Role | Primary Artifact & SHA-256 |
|---|---|---|---|
| `Run 1` | `w282_ultrarunner` | Cohort 1: 17-gen 4-island search (`1,984` births, `2,176` slot memberships) | `evidence/rs_v2/w282_ultrarunner/search/evidence.jevl` (`29d4602b1728e179606ac380df69652efea6e45c3daf67a6602cf56a6699b181`) |
| `Run 2`–`Run 5` | `evo_full_20261018_xbabe1_c01,c03..c05` (legacy staging `Run 37`–`Run 40`) | Cohort 3: 800-gen 4-island runs (`195,456` rows each) | `ac6ac8ae..`, `40763718..`, `b9e42e24..`, `1efcb3b2..` (`index.gen800_population.sha256.json`: `66c84ed4cdc943df885da4cdd72d25c0054195d9d8b9a01e1c35b6f3119c38cf`) |
| `Run 6`–`Run 9` | `evo_full_20261018_xbabe3_c01..c04` (legacy staging `Run 41`–`Run 44`) | Cohort 3: 800-gen 4-island runs (`195,456` rows each) | `75552d44..`, `e4425932..`, `a33071cd..`, `c3532f95..` |
| `Run 10`–`Run 11` | `evo_full_20261018_xbabe4_c01..c02` (legacy staging `Run 45`–`Run 46`) | Cohort 3: 800-gen 4-island runs (`195,456` rows each) | `6db20267..`, `86e2c256..` |
| `Run 12`–`Run 22` | `evidence/population_v2/runs/*.csv` | Earlier staged summary captures (`17,672` summary rows, max generation `708`); four overlap completed Runs 6–9 | `JR_EI_P1_T06_NATIVE_SUMMARY_ADMISSION_RECEIPT_20261019.json` |
| `Run 31`–`Run 36` | `w411c3, w411t3, w322, w501_ul, w409_ul, w501_xb1` | Cohort 2: 6 retained weighted reporting summaries (`15` rows) | `evidence/evolution_waves/2026-10-18_ultralight_v1/xbabe3/w411c3/fitness_curve.json` (`afe065170218f0f5f20942a9e737e7e91a300244f8e6a99fd3c1d617d16f1412`) |
| `Individual 1095` | `w282` birth ID `1095` (gen `9`; ancestors `174, 245, 299, 392, 413, 526`) | Cohort 1 highest native search fit (`1.15727`, `cert_pass_all = false`) | `evidence/rs_v2/w282_ultrarunner/search/all_evals.json` (`307370108fcede8a4399569582f6ab19d436603246531266afa1754f9ba65363`) |
| `Individual 2000` | `cert_pipeline_r01_p14_c02` (`db5d8e32...93c55e`) | Single assessed automatic export (`assessed_count = 1`, `qualified_count = 0`, `champion = null`) | STL `8e4987c422b2d9973b15e3fbbed64de809c3507abbb83cb6b1b6898753079022`; D1 `c2811ed8877904254ddc953dc888aedc3d7791b69c040dd6513fe0b763997bab` |
| `Individual 2001` | `w283_p14_c01` | Separately assessed controlled export (`W_CAD = -3.878704`, `G1 = 0.623903`, `75.382 g`, `11/16` legacy gates) | `evidence/rs_v2/w283_ultrarunner/w283_p14_c01_cad_eval.json` (`df555734d327afe077ca354764f8fea05e7e574bcaa1964beffb2653e911dffc`) |
| `Individual 2007` | `w283_p10_c02` | Separately assessed controlled export (`W_CAD = -3.964924`, `G1 = 0.621634`, `75.642 g`, `11/16` legacy gates) | `evidence/rs_v2/w283_ultrarunner/w283_p10_c02_cad_eval.json` (`6eb44dab69c307a5a90ae1a840fa6450d730718fce48542599999f0d5c2edf1d`) |
| `Individual 2008` | `w282_p14_c01` | Historical 16-gate controlled probe (`W_CAD = +1.7790`, `G1 = 0.878210`, `75.568 g`; fails grouped `RS-v2`) | `evidence/rs_v2/w282_ultrarunner/w282_p14_c01_cad_eval.json` |
| `Individual 2002` | `w411c3_r02_p10_c12` | Cohort 2 (`Run 31` gen `1–2`) highest retained weighted report (`F = 74.207299`, `101.139 g`, `13/26` flags) | `evidence/evolution_waves/2026-10-18_ultralight_v1/xbabe3/w411c3/fitness_curve.json` (`afe06517..`) |
| `Individual 2009` | `w411c3_r01_p10_c11` | Cohort 2 (`Run 31` gen `0`) weighted report (`F = 74.167488`, `101.108 g`) | `evidence/evolution_waves/2026-10-18_ultralight_v1/xbabe3/w411c3/fitness_curve.json` (`afe06517..`) |

## Reporting views and retained evidence

- **Export Identity & Topology Provenance Diagram (`Individual 2000`):** [`generated/export-identity.svg`](generated/export-identity.svg) / [`generated/export-identity.json`](generated/export-identity.json)
- **Separate retained composite reference and field inventory (`Individual 2000`):** [`drafts_p3_p5/JR_EI_P4_T01_T02_hero_white_field_composite.png`](drafts_p3_p5/JR_EI_P4_T01_T02_hero_white_field_composite.png) / [`drafts_p3_p5/D1_PRY_AND_FIELD_COVERAGE_INVENTORY_20261019.json`](drafts_p3_p5/D1_PRY_AND_FIELD_COVERAGE_INVENTORY_20261019.json)
- **Complete 23-Subject × 18-Evaluator Historical Battery Matrix (`414` cells: `221` pass, `81` fail, `112` blocked):** [`generated/battery-matrix.svg`](generated/battery-matrix.svg) / [`generated/battery-matrix.json`](generated/battery-matrix.json)
- **44-Entry Catalog-to-Observation-Family Crosswalk:** [`drafts_p3_p5/JR_EI_P3_T04_atlas_crosswalk_44.json`](drafts_p3_p5/JR_EI_P3_T04_atlas_crosswalk_44.json) and [`../tests/README.md`](../tests/README.md)


Short hash prefixes in the table are navigation aids. Use the original input contracts and archive indexes for full digests and producer revisions. The eleven earlier summary captures and ten completed native runs are overlapping accessions, not twenty-one independent runs. The [cohort inventory](supplement/cohort-inventory.tex) records the eighteen current campaign directories and separate aborted attempts.

Individual 2002's retained reporting vector and registry certificate identity do not establish availability of its CAD or later assessment. The admitted round directories retain neither its exported geometry nor complete round populations. Its original registry category and score-profile bytes remain unchanged. The historical scorer path bound to revision `009f5ac520fa3afe8ed8025230eef604dbb7f922` could not be recovered at that path; original normalization details remain unknown. The [score supplement](supplement/scores-and-composites.tex) reports stored terms and contributions without substituting the present scorer.

- **Every-record main population view:** [SVG](figures/population-latest.svg), [source, identities, extrema and point-operation receipt](generated/population/point-rendering.v1.json). All numeric records receive point operations; overlap can hide individual marks. Unscreened proposals remain nonnumeric.
- **Additional score context:** [axis definitions](generated/population/score-axis.v1.json), [quantile detail](generated/population-detail.svg), paired native runs [2–3](generated/native-runs-2-3.svg), [4–5](generated/native-runs-4-5.svg), [6–7](generated/native-runs-6-7.svg), [8–9](generated/native-runs-8-9.svg), [10–11](generated/native-runs-10-11.svg), and [source-bound leaders/ties](generated/population/retained-leaders.v1.json). Detail and run views reuse the original observations; the full-range view retains every numeric record.
- **Display support and inventory:** [complete row-level table](generated/test-field-roster.v1.tsv), [per-cell display/support receipt](generated/display-composite.v1.json), [support SVG](generated/hero-support.svg) and [separate contrast preview](generated/hero-contrast-preview.svg). Contrast tail percentages are original source-area fractions, not fractions of cell count. The selected main hero uses the full [0,1] index and mesh off.
- **Corrected physical-unit crop view:** [SVG](drafts_p3_p5/JR_EI_P4_T04_fem_saliency_crops_and_locators.svg), [PNG](drafts_p3_p5/JR_EI_P4_T04_fem_saliency_crops_and_locators.png), [derived semantic ledger](figure-semantics.v1.json), [runtime projection](generated/crop-semantics.v1.json), [claim/output binding](claim-evidence.v1.json). Panel C is tip pry in MPa/N. Panel B's rectangular window contains 64.5174% of the whole retained source-active torsion integral before picture-in-picture overlays; this is not unobscured visible-ink coverage. Its raw maximum 34.8589 exceeds the displayed 0.6611 range and is disclosed by an over-range legend.
- **Cover:** [exact CAD SVG](drafts_p3_p5/JR_EI_P4_T01_blender_exact_cad_hero_white.svg), [shared paper/README PNG](figures/fig1_hero_blender_v2.png). Its dimensionless index averages available, pooled area-weighted response ranks from four retained channels, without a post-mean min/max stretch. The [full test/channel roster](data/test-field-roster.v1.json) accounts for unavailable, scalar-only and incompatible observations; its 44 catalog predicates plus 8 channel metadata rows are not 52 independent tests. The [display contract](data/display-composite-contract.v1.json) defines per-cell contributors and source-area coverage with four eligible channels as denominator. This replaces the [predecessor hero recipe](https://github.com/neverhuman/jarambit/blob/510713a8b0a9ba0ae6b4740c8ab81f1e3e3bc5b6/docs/paper/figure-semantics.v1.json) while preserving its original evidence. The index is distinct from physical-unit fields and Individual 2002's weighted score.
- **Evidence paths:** [authored architecture SVG](figures/evidence-architecture.svg), with [source-bound diagram manifest](generated/evidence-architecture.svg.manifest.json); search counts and assessed exports retain distinct denominators and no inferred join.
- **Separate ancestry routes:** [two-hop lineage contract](lineage-inputs.v1.json) and [three-hop inheritance view](figures/inheritance.svg) with [edge ledger](generated/population/inheritance.json) and [supplementary raw-coordinate details](figures/inheritance-details.svg). The additive semantic ledger maps these routes without changing the original two-hop contract.

The original historical crop/composite receipts remain as historical evidence. Active crop output admission is superseded additively by `claim-evidence.v1.json`; old output hashes are not receipts for the corrected artwork. PNG/PDF inspection and validation outcomes require the matching publication receipt.

## Reproduction and version identities

[Build instructions and prerequisites](README.md) and [the reporting entry point](reproduce.sh) reconstruct reporting derivatives without a new search, CAD export or mechanical solve. Both PDFs are actual standalone builds; a TeX include fragment is source rather than a supplement download.

Keep these identities distinct:

| Identity | Meaning |
|---|---|
| Original producer | Revision and source bytes that define an archived observation; bound by original contracts |
| Evidence accession | Retained files and archive index; prior publication baseline `102cdbef52b3710ce360ac34475c96ede0e33970` |
| Reporting source | Current Rust reporter, semantic ledgers, TeX drivers and build recipe |
| Built outputs | Exact SVG/PNG/PDF bytes and toolchain/input manifests from that source |
| Delivery accession | Commit/archive containing the matching manuscript, outputs and final validation receipt |

This remains a research draft with author approval pending. The final delivery accession and independent approval are not established by a local build. The build writes its actual TeX/font/input closure and output digests under `target/ieee/paper/`; the reporting manifest separately binds original producer inputs. Final validation and immutable delivery links are recorded after the source is frozen and checked.
