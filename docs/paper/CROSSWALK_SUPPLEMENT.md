# Technical Supplement: Canonical Identifier Crosswalk, Provenance Digests, and 414-Cell Battery Ledger

This technical supplement binds every integer-only public run and individual identifier in [`docs/paper/main.pdf`](main.pdf) to its internal repository alias, producer revision, file path, and SHA-256 digest, governed by [`data/public_ids.v1.json`](data/public_ids.v1.json), [`data/selection_receipts.v1.json`](data/selection_receipts.v1.json), [`data/score_profiles.v1.json`](data/score_profiles.v1.json), and [`publication.v2.json`](publication.v2.json).

## 1. Canonical Crosswalk: Public Integer Identifiers to Repository Artifacts

| Public Label | Internal Alias / Package ID | Cohort & Role | Primary Artifact & SHA-256 |
|---|---|---|---|
| `Run 1` | `w282_ultrarunner` | Cohort 1: 17-gen 4-island search (`1,984` births, `2,176` slot memberships) | `evidence/rs_v2/w282_ultrarunner/search/evidence.jevl` (`29d4602b1728e179606ac380df69652efea6e45c3daf67a6602cf56a6699b181`) |
| `Run 2`–`Run 5` | `evo_full_20261018_xbabe1_c01,c03..c05` (legacy staging `Run 37`–`Run 40`) | Cohort 3: 800-gen 4-island runs (`195,456` rows each) | `ac6ac8ae..`, `40763718..`, `b9e42e24..`, `1efcb3b2..` (`index.gen800_population.sha256.json`: `66c84ed4cdc943df885da4cdd72d25c0054195d9d8b9a01e1c35b6f3119c38cf`) |
| `Run 6`–`Run 9` | `evo_full_20261018_xbabe3_c01..c04` (legacy staging `Run 41`–`Run 44`) | Cohort 3: 800-gen 4-island runs (`195,456` rows each) | `75552d44..`, `e4425932..`, `a33071cd..`, `c3532f95..` |
| `Run 10`–`Run 11` | `evo_full_20261018_xbabe4_c01..c02` (legacy staging `Run 45`–`Run 46`) | Cohort 3: 800-gen 4-island runs (`195,456` rows each) | `6db20267..`, `86e2c256..` |
| `Run 12`–`Run 22` | `evidence/population_v2/runs/*.csv` | Staged native summary cohort (`17,672` rows, max generation `708`) | `JR_EI_P1_T06_NATIVE_SUMMARY_ADMISSION_RECEIPT_20261019.json` |
| `Run 31`–`Run 36` | `w411c3, w411t3, w322, w501_ul, w409_ul, w501_xb1` | Cohort 2: 6 retained weighted reporting summaries (`15` rows) | `evidence/evolution_waves/2026-10-18_ultralight_v1/xbabe3/w411c3/fitness_curve.json` (`afe065170218f0f5f20942a9e737e7e91a300244f8e6a99fd3c1d617d16f1412`) |
| `Individual 1095` | `w282` birth ID `1095` (gen `9`; ancestors `174, 245, 299, 392, 413, 526`) | Cohort 1 highest native search fit (`1.15727`, `cert_pass_all = false`) | `evidence/rs_v2/w282_ultrarunner/search/all_evals.json` (`307370108fcede8a4399569582f6ab19d436603246531266afa1754f9ba65363`) |
| `Individual 2000` | `cert_pipeline_r01_p14_c02` (`db5d8e32...93c55e`) | Single assessed automatic export (`assessed_count = 1`, `qualified_count = 0`, `champion = null`) | STL `8e4987c422b2d9973b15e3fbbed64de809c3507abbb83cb6b1b6898753079022`; D1 `c2811ed8877904254ddc953dc888aedc3d7791b69c040dd6513fe0b763997bab` |
| `Individual 2001` | `w283_p14_c01` | Separately assessed controlled export (`W_CAD = -3.878704`, `G1 = 0.623903`, `75.382 g`, `11/16` legacy gates) | `evidence/rs_v2/w283_ultrarunner/w283_p14_c01_cad_eval.json` (`df555734d327afe077ca354764f8fea05e7e574bcaa1964beffb2653e911dffc`) |
| `Individual 2007` | `w283_p10_c02` | Separately assessed controlled export (`W_CAD = -3.964924`, `G1 = 0.621634`, `75.642 g`, `11/16` legacy gates) | `evidence/rs_v2/w283_ultrarunner/w283_p10_c02_cad_eval.json` (`6eb44dab69c307a5a90ae1a840fa6450d730718fce48542599999f0d5c2edf1d`) |
| `Individual 2008` | `w282_p14_c01` | Historical 16-gate controlled probe (`W_CAD = +1.7790`, `G1 = 0.878210`, `75.568 g`; fails grouped `RS-v2`) | `evidence/rs_v2/w282_ultrarunner/w282_p14_c01_cad_eval.json` |
| `Individual 2002` | `w411c3_r02_p10_c12` | Cohort 2 (`Run 31` gen `1–2`) highest retained weighted report (`F = 74.207299`, `101.139 g`, `13/26` flags) | `evidence/evolution_waves/2026-10-18_ultralight_v1/xbabe3/w411c3/fitness_curve.json` (`afe06517..`) |
| `Individual 2009` | `w411c3_r01_p10_c11` | Cohort 2 (`Run 31` gen `0`) weighted report (`F = 74.167488`, `101.108 g`) | `evidence/evolution_waves/2026-10-18_ultralight_v1/xbabe3/w411c3/fitness_curve.json` (`afe06517..`) |

## 2. Supplementary Visual & Provenance Artifacts

- **Export Identity & Topology Provenance Diagram (`Individual 2000`):** [`generated/export-identity.svg`](generated/export-identity.svg) / [`generated/export-identity.json`](generated/export-identity.json)
- **Unlit 5-Map Plate Reference Composite & Contributor Support (`Individual 2000`):** [`drafts_p3_p5/JR_EI_P4_T01_T02_hero_white_field_composite.png`](drafts_p3_p5/JR_EI_P4_T01_T02_hero_white_field_composite.png) / [`drafts_p3_p5/D1_PRY_AND_FIELD_COVERAGE_INVENTORY_20261019.json`](drafts_p3_p5/D1_PRY_AND_FIELD_COVERAGE_INVENTORY_20261019.json)
- **Complete 23-Subject × 18-Evaluator Historical Battery Matrix (`414` cells: `221` pass, `81` fail, `112` blocked):** [`generated/battery-matrix.svg`](generated/battery-matrix.svg) / [`generated/battery-matrix.json`](generated/battery-matrix.json)
- **44-Entry Catalog-to-Observation-Family Crosswalk:** [`drafts_p3_p5/JR_EI_P3_T04_atlas_crosswalk_44.json`](drafts_p3_p5/JR_EI_P3_T04_atlas_crosswalk_44.json) and [`../tests/README.md`](../tests/README.md)
