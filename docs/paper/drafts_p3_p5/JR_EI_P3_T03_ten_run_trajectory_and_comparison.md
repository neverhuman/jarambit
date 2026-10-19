# JR-EI-P3-T03: Profile-Specific Fitness, Numerical Methods, and 10-Run Generation-800 Trajectory & Cross-Run Statistical Comparison

- **Task ID:** `JR-EI-P3-T03`
- **Canonical Ledger Title:** `Write profile-specific fitness and numerical methods`
- **Admiralty Task ID:** `jarambit:JR-EI-P3-T03@r2:sha256:7dcec333e8fa4bbf1d2ee896195e51ae947d94ed1ae3c6eaa5dc1a8b04f55d55`
- **Governing Directives:** `SC-DEC-183K`, `SC-DEC-183L`, `SC-DEC-183M`, and `SC-DEC-183N` (`JR-R4` reader-facing integer labels in `.tex` drafts; technical path/hash crosswalk in JSON/Markdown/Supplement)
- **Target Manuscript Sections:** `docs/paper/sections/05-fitness.tex` (`JR_EI_P3_T03_05-fitness.tex`) and `docs/paper/sections/07-verification.tex` (`JR_EI_P3_T03_07-verification.tex`)
- **Validation Status:** `implemented_unvalidated` (`completed_draft_pending_primary_merge_and_p8_ci`; `CI_HOLD_UNTIL_P8 = true`; **NO git push to `origin`** per `SC-DEC-183M` §1 & `SC-DEC-183N` §3.3)

---

## 1. Profile-Specific Fitness Regimes and Display-Loss Transforms (`docs/paper/data/score_profiles.v1.json`)

Bound to `docs/paper/data/score_profiles.v1.json` (`sha256: 5d4403900023e9d602ae08c2ae7408e08699d4cd960e2fb660d5e5cc52d2e81d`, `7,679` B) and `docs/paper/JR_EI_P2_T02_SCORE_PROFILES_AND_DISPLAY_LOSS_RECEIPT_20261019.json` (`sha256: 5732e64f915481de9fab00bfe10a0a0114231c183dc0ad4e164e30f2c775843a`, `2,603` B), Jarambit enforces `cross_profile_splicing_allowed = false` across three distinct score regimes:

| Profile ID | Reader-Facing Cohort / Stage | Producer Commit & Source File | Native Direction & Domain | Display Loss Transform | Top Retained Individual / Run Metric |
|---|---|---|---|---|---|
| `historical_w282_native_qd_fit` | Cohort 1 (`Run 1`, Historical 17-Gen) | `82f450f7fb418e28c4b229031920dec5454435f3` (`crates/crescent/src/qd.rs`, `sha256: fe3d057eeec74c3278b46027265406ec0ee97dd65938ae49cb003e08ee5b3a5f`) | `higher_is_better`; conditional legality penalty (`< 0` when illegal) + mechanical/mass + archive novelty (`> 0` when feasible) | `L = -f` (order-preserving negation; retains unbounded negative legality penalty tail) | `Run 1`: `Individual 1095` at gen 9 (`f = 1.157265759829317` in `evidence.jevl`, `L = -1.157266`, `strict_certification.cert_pass_all = false`) |
| `gen800_retained_native_qd_fit` | Cohort 3 (`Run 2`–`Run 11`, 10 Retained 800-Gen Runs) | Base checkout `d108c9d7d8149ac230ca13fb642159985a8f2857` + local perf patch (`patched_qd_rs_sha256: e2ca28cc88469f2cc1c709d234150cbde2dc0eb1fb4b84eb778671ab5a7fc1bd`; 5 of 10 runs retain `proc_config.json`) | `higher_is_better`; per-run native `fit` column in `all_evals.csv` / `history_live.csv` (kept distinct from `historical_w282_native_qd_fit` until binary/seed/config provenance is uniform across all 10 runs) | `L = -f` | Cohort 3 (`Run 2`–`Run 11`): highest `max_fit_col6 = 1.1440` (`1.143970` in `all_evals.csv`, `Run 5`) |
| `W_ULTRALIGHT_v1` | Cohort 2 (Intermediate Multi-Stage Screening & Reporting Curves) | `009f5ac520fa3afe8ed8025230eef604dbb7f922` (`crates/paper-artifacts/src/validation.rs`) | `higher_is_better`; bounded `F in [0, 100]` across 4 priority groups (`30%` Mass+compactness, `25%` Form+comfort proxies, `15%` Exported geometry, `30%` Response+verification) | `L = 100 - F` (`L in [0, 100]`) | `Individual 2002` (`w411c3_r02_p10_c12`): `F = 74.207299`, `L = 25.792701` (`mass = 101.139 g` raster / `101.138 g` 3D solid; ranking only, not qualified) |
| `W_CAD_v2` | Stage-2 16-Gate CAD Evaluation | `e8cb049df7d03c7d3113556812cc140ca99a29c3` (`ops/ci/evaluate_cad_candidate.py`, `sha256: 18595cecc6336053d7304d34ab6d9ffbd4b0542bb9df504323b6bc8ea6887ef0`) | `higher_is_better`; feasible `[1.0, 2.0]` when `16/16` legacy gates pass; shortfall-penalized `< 0` when any gate fails | `L = -W_CAD` (retains negative shortfall penalty values without clipping) | Historical 16-gate screen leader (`w282_p14_c01`): `W_CAD = +1.7790` (`G1 = 0.878210`, fails RS-v2); Controlled probe `Individual 2001` (`w283_p14_c01`): `W_CAD = -3.878704` (`G1 = 0.6239029999999999`, `11/16` legacy gates true, `5/16` false, fails RS-v2) |

---

## 2. Cohort 3: Complete 10-Run Generation-800 Population Census & Cross-Run Statistical Comparison (`Run 2`–`Run 11`)

Independently verified against:
- `docs/paper/JR_EI_P1_T08_LOSSLESS_GEN800_POPULATION_ACCESSION_RECEIPT_20261019.json` (`60,069` B, `sha256: 79ba4b6da9df8fda27dfa035f20187499cf032420a5b15e90ccfbaa06a1479a1`),
- `/home/ubuntu/jarambit/artifacts/versioned-evolution-archives-gen800-population-v1/index.gen800_population.sha256.json` (`176,429` B, `sha256: 66c84ed4cdc943df885da4cdd72d25c0054195d9d8b9a01e1c35b6f3119c38cf`),
- `/home/ubuntu/xbabe2_kicker/captains-20261017/GEN800_ARCHIVE_BYTE_CENSUS_20261019T1841Z.json` (`6,208` B, `sha256: f56cb191f1e8d4b1c235856d5a4ccbd77f84e648ccf07d843bb17ef2dc08fb56`), and
- `docs/paper/JR_EI_P1_T06_NATIVE_SUMMARY_ADMISSION_RECEIPT_20261019.json` (`99,050` B, `sha256: 04d01bf761c5c63e5210c19bc05471bc3a2b86d608e44167799fa5bf45fd8a63`).

### 2.1 Per-Run Population & Trajectory Table (`1,954,560` Total Evaluated Births Across `10` Retained 800-Generation Runs)

*(Canonical Run Registry Rule (`SC-DEC-183Q` §3.4 & `SC-DEC-183R` Step 1.2): `docs/paper/data/public_ids.v1.json` (`sha256: 4a7d8d25ab06b85b7f6586663407fb5abde3b1aa50c0277a104046866ba57c82`) is the sole canonical public run registry (`Run 1` for historical 17-gen + `Run 2`–`Run 11` for the ten retained 800-gen runs). The deprecated legacy numbering `Run 37`–`Run 46` from `latest_snapshots.json` is listed in column 2 strictly as a historical migration crosswalk and is non-canonical.)*

| Canonical Public Label (`public_ids.v1.json`) | Deprecated Legacy Crosswalk (`latest_snapshots.json`) | Internal Package ID (Supplement Only) | Evaluated Birth Rows (`all_evals.csv`) | Generations (`min..max` / count) | Search-Legal Births (`legal == true`) | Summary Rows (`history_live.csv`) | Max Summary `best_fit` (`col 6` / `all_evals.csv`) | Min Loss `L = -max_fit` | `NaN` Tokens in Summary | `proc_config.json` Present | Raw CSV Bytes | Compressed zstd Bytes (`20` segs) | Raw CSV SHA-256 | Run Manifest SHA-256 |
|---|---:|---|---:|---|---:|---:|---:|---:|---:|---|---:|---:|---|---|
| **Run 2** | `37` | `evo_full_20261018_xbabe1_c01` | `195,456` | `0..800` (`801`) | `74,874` (`38.31%`) | `3,204` | `1.1327` (`1.132693`) | `-1.1327` | `1` | `false` | `383,105,475` | `49,310,376` | `ac6ac8aee11a6e744c69ffcc7fa20d87ae6e78aa0ef2efa2ce5570744b5ac58a` | `9ab21efa5ccb372ebb8cb82deb49d516700b3bbbc6d92656526ea988298d3c63` |
| **Run 3** | `38` | `evo_full_20261018_xbabe1_c03` | `195,456` | `0..800` (`801`) | `77,011` (`39.40%`) | `3,204` | `1.1089` (`1.108914`) | `-1.1089` | `5` | `true` (`seed=2026104203`) | `385,322,474` | `49,791,636` | `4076371886530cfa749eda6cffa8d88caf3be5bd76450c33ce577cecd6fd1fe0` | `fb1b35a5a6c6b3e1d31894522fa33b89f3d6ba021dbe76f93b9ea3dca2a8d355` |
| **Run 4** | `39` | `evo_full_20261018_xbabe1_c04` | `195,456` | `0..800` (`801`) | `77,396` (`39.60%`) | `3,204` | `1.1380` (`1.137975`) | `-1.1380` | `2` | `true` (`seed=2026104204`) | `377,177,096` | `48,902,646` | `b9e42e24177a7c318080b5d8965921945b5b657fc879e606bcbe19be215951d6` | `ef4680b5196548188a9c125ad198549e45b022280691b51aada9f54a9b32b786` |
| **Run 5** | `40` | `evo_full_20261018_xbabe1_c05` | `195,456` | `0..800` (`801`) | `77,625` (`39.71%`) | `3,204` | `1.1440` (`1.143970`) | `-1.1440` | `4` | `true` (`seed=2026104205`) | `383,117,362` | `50,038,585` | `1efcb3b228e8d682bf8cd8f886bd06f0baa01c2f64ef50b283ea35af022f82c6` | `630f754b1499bac3561fa91cd7b3cf49cbff76a6f2810c45f229fe9a3797ec57` |
| **Run 6** | `41` | `evo_full_20261018_xbabe3_c01` | `195,456` | `0..800` (`801`) | `70,278` (`35.96%`) | `3,204` | `1.1156` (`1.115555`) | `-1.1156` | `2` | `false` | `384,808,219` | `49,666,459` | `75552d44073ecba1864ad42cfdd7cfdadfb675cd4eace72783e99cd97f6b1176` | `632d3c660e09f4bb73f2665c724b6fc92ce46ec498e55e31973a07936c61bbd6` |
| **Run 7** | `42` | `evo_full_20261018_xbabe3_c02` | `195,456` | `0..800` (`801`) | `74,777` (`38.26%`) | `3,204` | `1.1228` (`1.122813`) | `-1.1228` | `6` | `false` | `381,765,717` | `49,990,583` | `e442593276e62784800abed28210d2f4774e8143069948c6ef8dd055e56e6989` | `bec92d4b8c19930c63b0d5b133e8449c69585742ab5974ab55277c02a1ce3f45` |
| **Run 8** | `43` | `evo_full_20261018_xbabe3_c03` | `195,456` | `0..800` (`801`) | `71,550` (`36.61%`) | `3,204` | `1.1108` (`1.110829`) | `-1.1108` | `2` | `true` (`seed=2026105303`) | `377,698,296` | `48,637,667` | `a33071cd6ce3d90c7196e14ee45592d556f58c4e2f3548ef2bf45604a964eb76` | `16d52c921f0321a1d6c051f0394a483f9545692dce939261b71496781284d1b1` |
| **Run 9** | `44` | `evo_full_20261018_xbabe3_c04` | `195,456` | `0..800` (`801`) | `76,284` (`39.03%`) | `3,204` | `1.1408` (`1.140844`) | `-1.1408` | `4` | `true` (`seed=2026105304`) | `380,387,911` | `50,148,309` | `c3532f95975c217b468fa18034f366d03a10b501331edb32405f8752ca691307` | `8a45a3bc587358941a5af747452b396f63e42029a5030d718cd55443774e53da` |
| **Run 10** | `45` | `evo_full_20261018_xbabe4_c01` | `195,456` | `0..800` (`801`) | `74,309` (`38.02%`) | `3,204` | `1.1186` (`1.118636`) | `-1.1186` | `7` | `false` | `377,583,811` | `49,150,203` | `6db20267b159bf795db54a9c9ff520ae5d0aa3351693c95eed266277cb4ad81c` | `2ae800591ee37890eff5b823ee7b65f936662296198f64fb43fb0ae2c8712996` |
| **Run 11** | `46` | `evo_full_20261018_xbabe4_c02` | `195,456` | `0..800` (`801`) | `80,132` (`41.00%`) | `3,204` | `1.1177` (`1.117713`) | `-1.1177` | `3` | `false` | `380,930,388` | `49,206,677` | `86e2c256f218e87ef18d7cb14df92724e298d3611bdbb8859b246106ec0e0db0` | `44eac1169c1371e0a716a8727e2e8dea873c1c4f3c6537065022d5ec726bb22c` |
| **Total / Range** | `10` retained runs | `200` zstd segments + `10` manifests | **`1,954,560`** | **`0..800`** (`801` each) | **`754,236`** (`38.5885%`) | **`32,040`** | **`1.1089`–`1.1440`** | **`-1.1440` to `-1.1089`** | **`36`** | `5` with / `5` without | **`3,811,896,749`** | **`494,843,141`** (`7.7032x`) | Index: `66c84ed4cdc943df885da4cdd72d25c0054195d9d8b9a01e1c35b6f3119c38cf` | Receipt: `79ba4b6da9df8fda27dfa035f20187499cf032420a5b15e90ccfbaa06a1479a1` |

### 2.2 Cross-Run Statistical Summary Across the Ten Retained Runs (`Run 2`–`Run 11`)

- **Per-Run Evaluation Count:** Each of the 10 retained runs evaluated `195,456` births (`64` initial seeds per island × `4` islands = `256` at generation `0`, plus `244` evaluated births per generation × `800` generations = `195,200`, totaling `195,456` records with local IDs `0..195455`, `0` duplicate IDs per run, `0` malformed widths). Because only 5 of the 10 runs retain `proc_config.json` with explicit PRNG seeds, we refer to the ensemble strictly as **ten retained runs** rather than ten statistically independent runs.
- **Search-Legal Counts (`legal == true`, exact row source: `all_evals.csv` column `legal` across `[74874, 77011, 77396, 77625, 70278, 74777, 71550, 76284, 74309, 80132]`):**
  - Range: `70,278 / 195,456` (`35.9559%`, `Run 6`) to `80,132 / 195,456` (`40.9975%`, `Run 11`).
  - Sum: `754,236` (`38.5885%` of `1,954,560`); Mean: `75,423.6`.
  - **Sample standard deviation (`ddof = 1`):** **`2,936.5539592446717`** (`1.5024%` of `195,456`).
  - **Population standard deviation (`ddof = 0`):** **`2,785.859694959529`** (`1.4253%` of `195,456`).
- **Maximum Recorded Summary Fitness (`max_fit_col6` in `history_live.csv`, 4-decimal list `[1.1327, 1.1089, 1.1380, 1.1440, 1.1156, 1.1228, 1.1108, 1.1408, 1.1186, 1.1177]` vs. 6-decimal `all_evals.csv` list `[1.132693, 1.108914, 1.137975, 1.143970, 1.115555, 1.122813, 1.110829, 1.140844, 1.118636, 1.117713]`):**
  - 4-decimal `history_live.csv` (`col 6`): range `1.1089` (`Run 3`) to `1.1440` (`Run 5`), mean `1.12499`, median `1.12070`, **sample SD (`ddof = 1`) = `0.012854869375713873`**, **population SD (`ddof = 0`) = `0.012195199875360786`**.
  - 6-decimal `all_evals.csv` (`fit` column): range `1.108914` (`Run 3`) to `1.143970` (`Run 5`), mean `1.1249942`, median `1.1207245`, **sample SD (`ddof = 1`) = `0.012847802534112782`**, **population SD (`ddof = 0`) = `0.012188495680763863`**.
- **Compression & Custody Boundary:** Raw CSV bytes total `3,811,896,749` B (`3.81 GB`) and compress losslessly into `494,843,141` B (`494.84 MB`) across `200` `.csv.zst` segments (overall compression ratio `7.70324:1`; maximum single segment `2,627,104` B `< 8 MiB`). As required by `SC-DEC-183M` §4 and `SC-DEC-183N` §1, this staged population archive is `implemented_unvalidated`, distinct from versioned Git checkout inclusion and final `P8` test execution.

---

## 3. Separation of Verified Top-Candidate & Probe Metrics Across Cohorts

To prevent any conflation across cohorts, score profiles, or evaluation stages:
1. **Cohort 1 (`Run 1`, Historical 17-Gen, `1,984` Births):**
   - Highest birth score: `Individual 1095` (generation `9`, `fit = 1.157265759829317` in `evidence.jevl`, rounded `1.15727` in `all_evals.json`). Admitted to `strict_certification` in `search_summary.json`, where `cert_pass_all = false`, `seq_ok = false`, `strict_ok = true`.
   - Historical 16-gate screen leader: `w282_p14_c01` (`75.568 g` raster / `75.323 g` 3D solid, `G1 = 0.878210`, `W_CAD = +1.7790` with `16/16` legacy engineering gates passing, but fails `RS-karambit-v2` on fatigue, Gate-S sharpness, and human-max).
2. **Cohort 2 (Intermediate Multi-Stage Screening, `3,649` Births, & Controlled Probes):**
   - Highest `W_ULTRALIGHT_v1` reporting curve candidate: `Individual 2002` (`w411c3_r02_p10_c12`, `W_ULTRALIGHT_v1 = 74.207299`, display loss `L = 25.792701`, `101.139 g` raster / `101.138 g` 3D solid; ranking score only, not qualified).
   - Controlled probes in `evidence/rs_v2/w283_ultrarunner/`:
     - `Individual 2001` (`w283_p14_c01_cad_eval.json`, `7,442` B, `sha256: df555734d327afe077ca354764f8fea05e7e574bcaa1964beffb2653e911dffc`): `W_CAD = -3.878704`, `global_fitness_G1.G = 0.6239029999999999` (battery `G = 0.623888308291947`), `all_engineering_gates_pass = false` (**`11/16` legacy weighted gates `true`, `5/16` `false`**: `cert_hunter_260n`, `failure_sequence_1p2`, `solid_tip_3d_p2_1p82nm`, `strict_hunter_260n`, `twist_tip_only_1p82nm`; `RS-karambit-v2 = FAIL`).
     - `w283_p10_c02` (`w283_p10_c02_cad_eval.json`, `7,492` B, `sha256: 6eb44dab69c307a5a90ae1a840fa6450d730718fce48542599999f0d5c2edf1d`): `W_CAD = -3.964924`, `global_fitness_G1.G = 0.6216340416666666` (battery `G = 0.6216318603776162`), `all_engineering_gates_pass = false` (**`11/16` legacy weighted gates `true`, `5/16` `false`**: `cert_hunter_260n`, `failure_sequence_1p2`, `solid_tip_3d_p2_1p82nm`, `strict_hunter_260n`, `twist_tip_only_1p82nm`; `RS-karambit-v2 = FAIL`).
3. **Illustrated Assessed Subject (`Individual 2000`, `cert_pipeline_r01_p14_c02`):**
   - Selected editorially (`selection_authority = "human"`) from `evidence/evolve_certifiable/manifest.json` (`1` assessed automatic candidate, `0` qualified in the 23-subject assessed automatic CAD set, `champion = null`; the `1,954,560` native search births were not individually Stage-2 CAD-assessed) because its complete STEP, STL, D1 plate fields (`25,209` triangles across 7 retained arrays including `pry`), 3-level GCI study, and matched-mesh `skfem` vs `DOLFINx 0.9.0` quadratic solid tip submodel (`91,008` tetrahedra) are retained and hash-bound.
