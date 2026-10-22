# All evolution runs, extended: the W_ULTRALIGHT_v1 runs plus the pre-W_ULTRALIGHT waves

This set holds everything in `stage*_all.tsv` / `islands.tsv` plus every pre-W_ULTRALIGHT evolution wave found
in the census. Everything is scored with the same canonical function: `compute_ultralight_weighted_fitness` from
the `evo-v2.2.0` lane, blob 9cfb4416. It was all collected read-only on 2026-10-22. Rows for the live runs are
the same 14:40Z snapshot as the base set. The added waves are static.

## Bottom line

- **The best evolved design under the current evaluator is not 76.14.** 106 distinct ok-class genomes from the
  2026-10-17/18 pre-W_ULTRALIGHT waves score above 76.141571.
- **The best is `w406t3_r01_p10_c16` at 80.299390.** It is p10, 83.2 g, solid_tip_sf 0.68, from the xbabe3 archive
  `jarambit_evo3c`. Your re-assessment with the v2.2.0 evaluator reproduced this value exactly, and it also
  reproduced `w401c3_r02_p10_c27` = 79.869405.
- **The best W_ULTRALIGHT-era design ranks #107.** That is `ec1i005_r01_p14_c32` at 76.141571.
- **The 1e93e6f3 regime is inflated, but its designs may still be strong.** Its scores (up to 92.29) are inflated.
  Even so, your re-assessment of `w290_r03_p14_c04` gives 79.166355. That is also above 76.14.

## Files

All paths are under `/tmp/claude-1000/-home-ubuntu-jarambit/c9e33658-c7d3-4b33-adfe-c826eb426c63/scratchpad/allruns/`.
The base files were not changed.

| file | rows (excl. header) | vs base |
|---|---:|---|
| `stage2_all_ext.tsv` | 1109 | 789 + 320 |
| `stage1_all_ext.tsv` | 52429 | 47169 + 5260; the added rows have an empty value |
| `islands_ext.tsv` | 295 | 225 + 70 |
| `SUMMARY_ext.md` | | this file |

How the columns differ from the base files:

- `stage2_all_ext.tsv` has the 13 base columns plus `evaluator_class` (column 14) and one more trailing column,
  `scoring_note` (column 15), which marks the rs=None rows:
  - `rs_present` (1065 rows).
  - `rs_missing_scored_with_None+null_metrics_dropped:plate_gci_fine,solid_tip_dolfinx_rel_err,solid_tip_sf_nodal_avg,solid_tip_sf_nodal_max`
    (44 rows: every scorecard of xbabe2 w291-w297, evaluator 4cd0f994e).
- These 44 scorecards have no `_rs.json`, and their `engineering_metrics` hold explicit `null` for the solid-tip
  fields. The canonical function raises `TypeError` on `float(None)`. So they were scored with `rs=None` after
  dropping the null keys, which means the function's defaults apply: solid tip 0.0 and rel_err 1.0.
- Every base row is unchanged in the ext files: `diff` shows all 789 / 47169 / 225 base rows, identical in the
  base columns.
- `scripts/census_ext.py` is the scorer variant that does the rs=None scoring; `scripts/assemble_ext.py` and
  `scripts/summarize_ext.py` build the ext files; `final/raw_xbabe2_ext_out.tsv` holds the re-read xbabe2 waves.

## What was added

| host | run | waves | Stage-2 | Stage-1 rows (empty value) |
|---|---|---|---:|---:|
| xbabe2 | `jarambit/out` (`~/jarambit/out`) | w284-w322, 39 waves | 183 | 3708 |
| xbabe3 | `archive:jarambit_evo3/out` | w284x3-w297x3, 14 waves | 56 | 0 |
| xbabe3 | `archive:jarambit_evo3c/out` | w400c3-w411t3, 16 waves | 80 | 1545 |
| xbabe2 | `repo-evidence:evolve_certifiable` | `automatic`, a 1-round certifiable-pipeline run | 1 | 7 |

- **xbabe2 `jarambit/out`:** the 183 Stage-2 are 139 with rs, including the 42 loose-in-wave-dir scorecards of
  w284-w290, plus 44 rs=None (w291-w297). The 3708 Stage-1 rows come from w298-w322; w284-w297 have no screens.
- **`archive:jarambit_evo3/out`:** all 56 have rs.
- **`repo-evidence:evolve_certifiable`:** this is a git-tracked run. It sits in every checkout, so it was taken
  once, from xbabe2 `~/jarambit`. Its evaluator is e8cb049, 79.806859. Drop it if a committed demo run should not
  count.

Not added, because they duplicate rows already in the set or are not evolution waves:

- `/tmp/jr_evo3_waves` on xbabe3: 56 scorecards, byte-identical to the `jarambit_evo3` archive (checked by sha256).
- Committed `evidence/evolution_plateau/2026-10-17` copies in every checkout: 11 scorecards, byte-identical to the
  originals in `~/jarambit/out` and `jarambit_evo3c`. The committed `evidence/evolution_waves/2026-10-17` screens
  are copies of the same waves.
- `~/jarambit-audits/*w313-w314-provenance` on xbabe2: 10 scorecards identical to `~/jarambit/out`, plus 3
  `auto_r01_p14_c01` test-publication fixtures.
- `jarambit_evo3c` `stage2_test`: a single 6438f07 test assessment, 79.863459, with no screens. Also the empty
  `seeds` dir there.
- Rescoring dirs (`~/jarambit/out/rescore_main_4cd0f994`, `~/jarambit_evo3_tools/rescore_4cd0f99`, `review83_w282`),
  crescent lanes, and test harness fixtures.

## evaluator_class

`inflated_1e93e6f3` covers 142 rows. Every other row (967) is `ok`.

| evaluator | rows | why it is in the class |
|---|---:|---|
| **1e93e6f31** | 98 | Prefix match: xbabe2 w284-w290 (42) and xbabe3 w284x3-w297x3 (56). |
| **4cd0f994e** | 44 | Assigned by regime: xbabe2 w291-w297. **Your call.** |

Why 4cd0f994e is assigned by regime:

- Its median stored mass_3d_g is 74.82 g, against 75.10 g for 1e93e6f3.
- Its solid_tip_sf is not stored (null in every scorecard), so the "tip ≈ 1.5" half of the test cannot be checked.
- Every other stored metric matches 1e93e6f3 and no later evaluator:

  | stored metric (median) | 4cd0f994e | 1e93e6f31 | every other evaluator |
  |---|---:|---:|---:|
  | twist_tip_sf | 1.12 | 1.14 | ≤ 0.56 |
  | lat_rob_cert_n | 263 | 262 | 123-192 |
  | ring_rob_cert_n | 13 993 | 14 250-14 388 | 20 600-28 000 |

- No other commit comes close: every other evaluator has median mass ≥ 82.9 g and median tip ≤ 0.71.
- 4cd0f994e (2026-10-16T20:21-06:00) and 1e93e6f3 (2026-10-16T22:56Z) are the two earliest evaluators in the set.

The rs file matters only for the 1e93e6f3 regime:

- For the 42 1e93e6f3 scorecards that have rs, `score(rs) − score(None)` is a constant **+5.318**. Their scorecards
  lack the embedded rs gates and export measurements that later scorecards carry.
- For later evaluators the difference is 0.000: 98 pre-W_ULTRALIGHT ok-class scorecards and 135 evo_continuous
  scorecards were checked.
- So the 44 rs=None rows of 4cd0f994e (≈ 81.2) are probably about 5 points low relative to the other
  inflated-class rows, on top of their unmeasured tip counting as 0.

## Evaluator commits in the extended Stage-2 set

| evaluator commit | W_ULTRALIGHT_v1 in evaluator | n | median stored mass_3d_g | median stored solid_tip_sf | best canonical | evaluator_class | runs |
|---|---|---:|---:|---:|---:|---|---|
| 1e93e6f31 | no | 98 | 75.097 | 1.508 | 92.290346 | inflated_1e93e6f3 | xbabe2:jarambit/out, xbabe3:archive:jarambit_evo3/out |
| 4cd0f994e | no | 44 | 74.818 | null | 81.207040 | inflated_1e93e6f3 | xbabe2:jarambit/out |
| e8cb049df | no | 1 | 83.039 | 0.669 | 79.806859 | ok | xbabe2:repo-evidence:evolve_certifiable |
| 6438f0778 | no | 8 | 83.067 | 0.669 | 79.869405 | ok | xbabe3:archive:jarambit_evo3c/out |
| ae86f188b | no | 6 | 83.029 | 0.653 | 79.800674 | ok | xbabe2:jarambit/out |
| 9d6416005 | no | 10 | 83.147 | 0.664 | 79.984072 | ok | xbabe2:jarambit/out, xbabe3:archive:jarambit_evo3c/out |
| 7cb952ef2 | no | 15 | 82.994 | 0.662 | 79.930876 | ok | xbabe2:jarambit/out, xbabe3:archive:jarambit_evo3c/out |
| edc0f4057 | no | 8 | 89.748 | 0.674 | 78.843615 | ok | xbabe2:jarambit/out |
| 805faaa45 | no | 12 | 91.468 | 0.685 | 78.890167 | ok | xbabe2:jarambit/out |
| 253944829 | no | 4 | 92.184 | 0.661 | 77.935104 | ok | xbabe2:jarambit/out |
| a8a7de426 | no | 4 | 92.169 | 0.684 | 78.179691 | ok | xbabe2:jarambit/out |
| 0c8d1692a | no | 4 | 92.181 | 0.709 | 78.210578 | ok | xbabe2:jarambit/out |
| dae022254 | no | 4 | 91.439 | 0.707 | 78.924834 | ok | xbabe2:jarambit/out |
| 19276875b | no | 8 | 89.512 | 0.707 | 78.946511 | ok | xbabe2:jarambit/out |
| 1eee4d128 | no | 20 | 86.475 | 0.682 | 80.299390 | ok | xbabe2:jarambit/out, xbabe3:archive:jarambit_evo3c/out |
| 21425fe29 | no | 3 | 87.631 | 0.709 | 78.954831 | ok | xbabe2:jarambit/out |
| d04609343 | no | 11 | 98.517 | 0.221 | 80.299390 | ok | xbabe3:archive:jarambit_evo3c/out |
| f1b2671f3 | no | 8 | 85.427 | 0.680 | 80.299390 | ok | xbabe2:jarambit/out, xbabe3:archive:jarambit_evo3c/out |
| 0a6aa9466 | no | 13 | 98.529 | 0.237 | 78.954831 | ok | xbabe2:jarambit/out, xbabe3:archive:jarambit_evo3c/out |
| b38642b37 | no | 3 | 91.506 | 0.703 | 78.954831 | ok | xbabe2:jarambit/out |
| 90f77f3f0 | no | 10 | 93.499 | 0.438 | 80.299390 | ok | xbabe2:jarambit/out, xbabe3:archive:jarambit_evo3c/out |
| 8e7950d4c | no | 2 | 101.363 | 0.468 | 78.035623 | ok | xbabe2:jarambit/out |
| dd41756fb | no | 3 | 91.506 | 0.703 | 78.954831 | ok | xbabe2:jarambit/out |
| 9443b11a7 | no | 13 | 102.866 | 0.358 | 74.207299 | ok | xbabe2:jarambit/out, xbabe3:archive:jarambit_evo3c/out |
| 015039790 | no | 4 | 113.432 | 0.376 | 71.104919 | ok | xbabe2:jarambit/out |
| b7ef2de48 | no | 3 | 113.862 | 0.351 | 71.010933 | ok | xbabe2:jarambit/out |
| 513e215e2 | no | 3 | 108.419 | 0.364 | 72.324730 | ok | xbabe3:archive:jarambit_evo3c/out |
| b64f15603 | no | 1 | 112.935 | 0.351 | 71.010933 | ok | xbabe2:jarambit/out |
| 9f04b4717 | no | 1 | 113.908 | 0.371 | 71.097880 | ok | xbabe1:jarambit/out |
| 9967a93a9 | yes | 26 | 110.560 | 0.367 | 72.398819 | ok | xbabe1:jarambit/out, xbabe3:archive:jarambit_evo3c/out |
| eb5835d73 | yes | 127 | 90.814 | 0.275 | 75.983539 | ok | xbabe2:jarambit/out, xbabe3:archive:jarambit_evo3u/out, xbabe3:evo_full_20261018 |
| 97d6855c9 | yes | 1 | 93.656 | 0.212 | 74.319278 | ok | xbabe2:jarambit/out |
| 3367c907b | yes | 1 | 94.376 | 0.234 | 72.786611 | ok | xbabe2:jarambit/out |
| 6c3d64e3d | yes | 2 | 94.015 | 0.223 | 74.319381 | ok | xbabe2:jarambit/out |
| 009f5ac52 | yes | 392 | 100.748 | 0.293 | 75.463948 | ok | xbabe1:jarambit/out, xbabe2:evo_full_20261018, xbabe2:jarambit/out, xbabe3:evo_full_20261018 |
| 49dc71f88 | yes | 221 | 90.175 | 0.283 | 76.141571 | ok | xbabe1:evo_continuous, xbabe1:evo_continuous:_superseded_dup_seed_20261022, xbabe2:evo_continuous, xbabe3:evo_continuous |
| e07606a8e | yes | 15 | 91.814 | 0.326 | 76.127401 | ok | xbabe1:evo_continuous, xbabe2:evo_continuous |

## Per wave: added pre-W_ULTRALIGHT waves and the 2026-10-18/19 W_ULTRALIGHT_v1 waves

| host | run | wave | first scorecard (UTC) | evaluator commit(s) | Stage-1 rows (with value) | Stage-2 (rs missing) | best ok-class | best inflated-class |
|---|---|---|---|---|---:|---:|---:|---:|
| xbabe3 | archive:jarambit_evo3/out | w284x3 | 2026-10-16T23:33:01Z | 1e93e6f31×4 | 0 (0) | 4 |  | 92.191760 |
| xbabe2 | jarambit/out | w284 | 2026-10-16T23:43:34Z | 1e93e6f31×6 | 0 (0) | 6 |  | 92.207807 |
| xbabe3 | archive:jarambit_evo3/out | w285x3 | 2026-10-16T23:47:17Z | 1e93e6f31×4 | 0 (0) | 4 |  | 92.198619 |
| xbabe3 | archive:jarambit_evo3/out | w286x3 | 2026-10-17T00:00:00Z | 1e93e6f31×4 | 0 (0) | 4 |  | 92.200922 |
| xbabe2 | jarambit/out | w285 | 2026-10-17T00:10:43Z | 1e93e6f31×6 | 0 (0) | 6 |  | 92.215831 |
| xbabe3 | archive:jarambit_evo3/out | w287x3 | 2026-10-17T00:13:06Z | 1e93e6f31×4 | 0 (0) | 4 |  | 92.206668 |
| xbabe3 | archive:jarambit_evo3/out | w288x3 | 2026-10-17T00:25:22Z | 1e93e6f31×4 | 0 (0) | 4 |  | 92.210085 |
| xbabe2 | jarambit/out | w286 | 2026-10-17T00:34:43Z | 1e93e6f31×6 | 0 (0) | 6 |  | 92.219893 |
| xbabe3 | archive:jarambit_evo3/out | w289x3 | 2026-10-17T00:37:12Z | 1e93e6f31×4 | 0 (0) | 4 |  | 92.212388 |
| xbabe3 | archive:jarambit_evo3/out | w290x3 | 2026-10-17T00:49:17Z | 1e93e6f31×4 | 0 (0) | 4 |  | 92.214692 |
| xbabe2 | jarambit/out | w287 | 2026-10-17T00:58:16Z | 1e93e6f31×6 | 0 (0) | 6 |  | 92.214692 |
| xbabe3 | archive:jarambit_evo3/out | w291x3 | 2026-10-17T01:01:08Z | 1e93e6f31×4 | 0 (0) | 4 |  | 92.215831 |
| xbabe3 | archive:jarambit_evo3/out | w292x3 | 2026-10-17T01:13:04Z | 1e93e6f31×4 | 0 (0) | 4 |  | 92.211249 |
| xbabe2 | jarambit/out | w288 | 2026-10-17T01:22:39Z | 1e93e6f31×6 | 0 (0) | 6 |  | 92.260555 |
| xbabe3 | archive:jarambit_evo3/out | w293x3 | 2026-10-17T01:24:42Z | 1e93e6f31×4 | 0 (0) | 4 |  | 92.212388 |
| xbabe3 | archive:jarambit_evo3/out | w294x3 | 2026-10-17T01:36:57Z | 1e93e6f31×4 | 0 (0) | 4 |  | 92.216969 |
| xbabe3 | archive:jarambit_evo3/out | w295x3 | 2026-10-17T01:49:39Z | 1e93e6f31×4 | 0 (0) | 4 |  | 92.216969 |
| xbabe2 | jarambit/out | w289 | 2026-10-17T01:51:06Z | 1e93e6f31×6 | 0 (0) | 6 |  | 92.281184 |
| xbabe3 | archive:jarambit_evo3/out | w296x3 | 2026-10-17T02:03:53Z | 1e93e6f31×4 | 0 (0) | 4 |  | 92.218108 |
| xbabe2 | jarambit/out | w290 | 2026-10-17T02:10:39Z | 1e93e6f31×6 | 0 (0) | 6 |  | 92.290346 |
| xbabe3 | archive:jarambit_evo3/out | w297x3 | 2026-10-17T02:20:38Z | 1e93e6f31×4 | 0 (0) | 4 |  | 92.222715 |
| xbabe2 | jarambit/out | w291 | 2026-10-17T02:48:35Z | 4cd0f994e×8 | 0 (0) | 8 (8) |  | 81.203411 |
| xbabe2 | jarambit/out | w292 | 2026-10-17T03:31:37Z | 4cd0f994e×6 | 0 (0) | 6 (6) |  | 81.207040 |
| xbabe2 | jarambit/out | w293 | 2026-10-17T03:52:54Z | 4cd0f994e×6 | 0 (0) | 6 (6) |  | 81.207040 |
| xbabe2 | jarambit/out | w294 | 2026-10-17T04:14:14Z | 4cd0f994e×6 | 0 (0) | 6 (6) |  | 81.207040 |
| xbabe2 | jarambit/out | w295 | 2026-10-17T04:36:29Z | 4cd0f994e×6 | 0 (0) | 6 (6) |  | 81.207040 |
| xbabe3 | archive:jarambit_evo3c/out | w400c3 |  |  | 92 (0) | 0 |  |  |
| xbabe2 | jarambit/out | w296 | 2026-10-17T04:58:20Z | 4cd0f994e×6 | 0 (0) | 6 (6) |  | 81.207040 |
| xbabe2 | jarambit/out | w297 | 2026-10-17T05:19:16Z | 4cd0f994e×6 | 0 (0) | 6 (6) |  | 81.205896 |
| xbabe2 | repo-evidence:evolve_certifiable | automatic | 2026-10-17T05:34:53Z | e8cb049df×1 | 7 (0) | 1 | 79.806859 |  |
| xbabe3 | archive:jarambit_evo3c/out | w401c3 | 2026-10-17T05:48:31Z | 6438f0778×4 | 92 (0) | 4 | 79.869405 |  |
| xbabe2 | jarambit/out | w298 | 2026-10-17T05:52:01Z | ae86f188b×6 | 179 (0) | 6 | 79.800674 |  |
| xbabe2 | jarambit/out | w299 |  |  | 64 (0) | 0 |  |  |
| xbabe3 | archive:jarambit_evo3c/out | w402c3 | 2026-10-17T07:54:19Z | 6438f0778×4 | 90 (0) | 4 | 79.847065 |  |
| xbabe2 | jarambit/out | w300 | 2026-10-17T08:04:14Z | 9d6416005×4 | 156 (0) | 4 | 79.984072 |  |
| xbabe2 | jarambit/out | w301 | 2026-10-17T09:25:14Z | 7cb952ef2×3, 9d6416005×1 | 178 (0) | 4 | 79.627949 |  |
| xbabe3 | archive:jarambit_evo3c/out | w403c3 | 2026-10-17T10:17:21Z | 9d6416005×5 | 116 (0) | 5 | 79.868766 |  |
| xbabe2 | jarambit/out | w302 | 2026-10-17T10:38:39Z | edc0f4057×4 | 173 (0) | 4 | 78.590852 |  |
| xbabe2 | jarambit/out | w303 | 2026-10-17T12:26:02Z | edc0f4057×4 | 176 (0) | 4 | 78.843615 |  |
| xbabe2 | jarambit/out | w304 | 2026-10-17T13:42:24Z | 805faaa45×4 | 171 (0) | 4 | 78.890167 |  |
| xbabe2 | jarambit/out | w305 | 2026-10-17T14:52:48Z | 805faaa45×4 | 175 (0) | 4 | 78.852711 |  |
| xbabe2 | jarambit/out | w306 | 2026-10-17T15:58:52Z | 805faaa45×4 | 177 (0) | 4 | 78.020067 |  |
| xbabe3 | archive:jarambit_evo3c/out | w404c3 | 2026-10-17T16:36:16Z | 7cb952ef2×6 | 124 (0) | 6 | 79.930876 |  |
| xbabe2 | jarambit/out | w307 | 2026-10-17T17:16:35Z | 253944829×4 | 176 (0) | 4 | 77.935104 |  |
| xbabe2 | jarambit/out | w308 | 2026-10-17T18:35:02Z | a8a7de426×4 | 177 (0) | 4 | 78.179691 |  |
| xbabe2 | jarambit/out | w309 | 2026-10-17T19:41:39Z | 0c8d1692a×4 | 178 (0) | 4 | 78.210578 |  |
| xbabe2 | jarambit/out | w310 | 2026-10-17T20:57:19Z | dae022254×4 | 176 (0) | 4 | 78.924834 |  |
| xbabe2 | jarambit/out | w311 | 2026-10-17T22:10:34Z | 19276875b×4 | 177 (0) | 4 | 78.940593 |  |
| xbabe3 | archive:jarambit_evo3c/out | w405c3 | 2026-10-17T22:15:03Z | 7cb952ef2×6 | 129 (0) | 6 | 79.918893 |  |
| xbabe2 | jarambit/out | w312 | 2026-10-17T23:32:05Z | 19276875b×4 | 180 (0) | 4 | 78.946511 |  |
| xbabe2 | jarambit/out | w313 | 2026-10-18T00:48:50Z | 1eee4d128×4 | 178 (0) | 4 | 78.949960 |  |
| xbabe2 | jarambit/out | w314 | 2026-10-18T02:02:51Z | 1eee4d128×4 | 175 (0) | 4 | 78.959202 |  |
| xbabe3 | archive:jarambit_evo3c/out | w406c3 | 2026-10-18T02:26:25Z | 1eee4d128×6 | 127 (0) | 6 | 79.776493 |  |
| xbabe3 | archive:jarambit_evo3c/out | w406t3 | 2026-10-18T02:26:32Z | 1eee4d128×6 | 128 (0) | 6 | 80.299390 |  |
| xbabe2 | jarambit/out | w315 | 2026-10-18T03:25:13Z | 21425fe29×3, f1b2671f3×1 | 176 (0) | 4 | 78.954831 |  |
| xbabe3 | archive:jarambit_evo3c/out | w407c3 | 2026-10-18T05:24:56Z | d04609343×5 | 52 (0) | 5 | 80.299390 |  |
| xbabe3 | archive:jarambit_evo3c/out | w407t3 | 2026-10-18T05:25:09Z | d04609343×6 | 52 (0) | 6 | 80.299390 |  |
| xbabe2 | jarambit/out | w316 | 2026-10-18T07:04:40Z | 0a6aa9466×1, f1b2671f3×3 | 34 (0) | 4 | 78.954831 |  |
| xbabe3 | archive:jarambit_evo3c/out | w408c3 | 2026-10-18T07:50:00Z | 0a6aa9466×4, f1b2671f3×2 | 72 (0) | 6 | 80.299390 |  |
| xbabe3 | archive:jarambit_evo3c/out | w408t3 | 2026-10-18T07:50:22Z | 0a6aa9466×4, f1b2671f3×2 | 72 (0) | 6 | 80.299390 |  |
| xbabe2 | jarambit/out | w317 | 2026-10-18T08:29:22Z | 0a6aa9466×4 | 33 (0) | 4 | 78.954831 |  |
| xbabe2 | jarambit/out | w318 | 2026-10-18T09:49:19Z | 90f77f3f0×1, b38642b37×3 | 90 (0) | 4 | 78.954831 |  |
| xbabe3 | archive:jarambit_evo3c/out | w410t3 | 2026-10-18T10:39:01Z | 90f77f3f0×3 | 94 (0) | 3 | 80.299390 |  |
| xbabe3 | archive:jarambit_evo3c/out | w410c3 | 2026-10-18T10:39:05Z | 90f77f3f0×5 | 94 (0) | 5 | 80.299390 |  |
| xbabe2 | jarambit/out | w319 | 2026-10-18T11:18:29Z | 8e7950d4c×2, 90f77f3f0×1, dd41756fb×1 | 104 (0) | 4 | 78.954831 |  |
| xbabe2 | jarambit/out | w320 | 2026-10-18T12:51:49Z | 9443b11a7×1, dd41756fb×2 | 105 (0) | 3 | 78.954831 |  |
| xbabe3 | archive:jarambit_evo3c/out | w411t3 | 2026-10-18T14:35:14Z | 9443b11a7×6 | 105 (0) | 6 | 74.167488 |  |
| xbabe3 | archive:jarambit_evo3c/out | w411c3 | 2026-10-18T14:36:49Z | 9443b11a7×6 | 106 (0) | 6 | 74.207299 |  |
| xbabe2 | jarambit/out | w321 | 2026-10-18T14:54:06Z | 015039790×4 | 129 (0) | 4 | 71.104919 |  |
| xbabe2 | jarambit/out | w322 | 2026-10-18T16:50:51Z | b64f15603×1, b7ef2de48×3 | 171 (0) | 4 | 71.010933 |  |
| xbabe3 | archive:jarambit_evo3c/out | w412c3 | 2026-10-18T18:05:50Z | 513e215e2×2, 9967a93a9×4 | 143 (0) | 6 | 72.398819 |  |
| xbabe3 | archive:jarambit_evo3c/out | w412t3 | 2026-10-18T18:11:00Z | 513e215e2×1, 9967a93a9×11 | 143 (0) | 12 | 72.398819 |  |
| xbabe1 | jarambit/out | w501_ul |  |  | 104 (104) | 0 |  |  |
| xbabe3 | archive:jarambit_evo3c/out | w409_ul |  |  | 115 (115) | 0 |  |  |
| xbabe2 | jarambit/out | w323 |  |  | 135 (135) | 0 |  |  |
| xbabe1 | jarambit/out | w501_xb1 | 2026-10-18T18:28:47Z | 9967a93a9×11, 9f04b4717×1 | 270 (0) | 12 | 71.104919 |  |
| xbabe2 | jarambit/out | w324 | 2026-10-18T19:51:17Z | 3367c907b×1, 97d6855c9×1, eb5835d73×2 | 147 (147) | 4 | 74.319278 |  |
| xbabe2 | jarambit/out | w325 | 2026-10-18T23:23:28Z | 009f5ac52×1, 6c3d64e3d×2 | 216 (216) | 3 | 74.333984 |  |
| xbabe1 | jarambit/out | w502_xb1 | 2026-10-19T00:34:20Z | 009f5ac52×13 | 289 (289) | 13 | 74.358394 |  |
| xbabe3 | archive:jarambit_evo3u/out | w413c3 | 2026-10-19T04:27:56Z | eb5835d73×6 | 164 (164) | 6 | 74.050404 |  |
| xbabe3 | archive:jarambit_evo3u/out | w413t3 | 2026-10-19T04:34:53Z | eb5835d73×11 | 164 (164) | 11 | 74.050404 |  |

## Per host and run (extended set)

| host | run | islands/waves | Stage-1 rows | Stage-2 | ok-class | inflated-class | best ok-class | best inflated-class |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| xbabe1 | evo_continuous | 6 | 2175 | 50 | 50 | 0 | 76.141571 |  |
| xbabe1 | evo_continuous:_superseded_dup_seed_20261022 | 3 | 263 | 4 | 4 | 0 | 75.448059 |  |
| xbabe1 | jarambit/out | 3 | 663 | 25 | 25 | 0 | 74.358394 |  |
| xbabe2 | evo_continuous | 17 | 5104 | 129 | 129 | 0 | 76.127401 |  |
| xbabe2 | evo_full_20261018 | 96 | 21054 | 326 | 326 | 0 | 75.463948 |  |
| xbabe2 | evo_full_20261018:_aborted_islands_v200_unpatched_0124Z | 24 | 0 | 0 | 0 | 0 |  |  |
| xbabe2 | jarambit/out | 42 | 4206 | 190 | 104 | 86 | 79.984072 | 92.290346 |
| xbabe2 | repo-evidence:evolve_certifiable | 1 | 7 | 1 | 1 | 0 | 79.806859 |  |
| xbabe3 | archive:jarambit_evo3/out | 14 | 0 | 56 | 0 | 56 |  | 92.222715 |
| xbabe3 | archive:jarambit_evo3c/out | 19 | 1946 | 98 | 98 | 0 | 80.299390 |  |
| xbabe3 | archive:jarambit_evo3u/out | 2 | 328 | 17 | 17 | 0 | 74.050404 |  |
| xbabe3 | evo_continuous | 16 | 7247 | 53 | 53 | 0 | 75.644245 |  |
| xbabe3 | evo_continuous:_superseded_dup_seed_20261022 | 8 | 553 | 0 | 0 | 0 |  |  |
| xbabe3 | evo_full_20261018 | 24 | 7199 | 160 | 160 | 0 | 75.983539 |  |
| xbabe3 | evo_full_20261018:_aborted_islands_v200_unpatched_0124Z | 16 | 0 | 0 | 0 | 0 |  |  |
| xbabe4 | evo_full_20261018 | 4 | 1684 | 0 | 0 | 0 |  |  |
| **all** | | 295 | 52429 | 1109 | 967 | 142 | 80.299390 | 92.290346 |

## Top 15 ok-class Stage-2 genomes overall (distinct genome sha256, best assessment of each)

| # | score | label | host | run | evaluator | mass_3d_g | solid_tip_sf | genome sha12 | assessments of this genome |
|---:|---:|---|---|---|---|---:|---:|---|---:|
| 1 | 80.299390 | w406t3_r01_p10_c16 | xbabe3 | archive:jarambit_evo3c/out | 1eee4d128 | 83.2231 | 0.6823 | 83877fdbb08a | 7 |
| 2 | 80.179111 | w406t3_r02_p10_c23 | xbabe3 | archive:jarambit_evo3c/out | 1eee4d128 | 83.2154 | 0.6233 | 441447033132 | 1 |
| 3 | 79.984072 | w300_r01_p10_c68 | xbabe2 | jarambit/out | 9d6416005 | 83.2698 | 0.6596 | 2b32d32271d9 | 1 |
| 4 | 79.971998 | w406t3_r01_p14_c35 | xbabe3 | archive:jarambit_evo3c/out | 1eee4d128 | 83.6128 | 0.6363 | 10df9e82c24b | 1 |
| 5 | 79.930876 | w404c3_r03_p10_c15 | xbabe3 | archive:jarambit_evo3c/out | 7cb952ef2 | 82.8793 | 0.6777 | ace0e40a9c50 | 1 |
| 6 | 79.918893 | w405c3_r01_p10_c21 | xbabe3 | archive:jarambit_evo3c/out | 7cb952ef2 | 82.9256 | 0.6777 | 32bf4fae97ec | 1 |
| 7 | 79.913655 | w404c3_r02_p10_c21 | xbabe3 | archive:jarambit_evo3c/out | 7cb952ef2 | 82.8746 | 0.6756 | 132ca4d8c703 | 1 |
| 8 | 79.898664 | w405c3_r02_p10_c15 | xbabe3 | archive:jarambit_evo3c/out | 7cb952ef2 | 82.9301 | 0.6795 | 3ac27f49e2c0 | 1 |
| 9 | 79.869405 | w401c3_r02_p10_c27 | xbabe3 | archive:jarambit_evo3c/out | 6438f0778 | 82.8525 | 0.6716 | a61941ea266e | 1 |
| 10 | 79.869336 | w404c3_r01_p10_c26 | xbabe3 | archive:jarambit_evo3c/out | 7cb952ef2 | 82.8445 | 0.6708 | c4679d16c8cc | 1 |
| 11 | 79.868766 | w401c3_r02_p10_c15 | xbabe3 | archive:jarambit_evo3c/out | 6438f0778 | 82.8467 | 0.6708 | 50f7141bbfbb | 2 |
| 12 | 79.847065 | w402c3_r02_p10_c03 | xbabe3 | archive:jarambit_evo3c/out | 6438f0778 | 83.2297 | 0.6546 | 9a34a6ec14bb | 1 |
| 13 | 79.846886 | w401c3_r01_p10_c14 | xbabe3 | archive:jarambit_evo3c/out | 6438f0778 | 82.8414 | 0.6693 | 0247ee94769a | 1 |
| 14 | 79.837049 | w402c3_r01_p10_c45 | xbabe3 | archive:jarambit_evo3c/out | 6438f0778 | 83.2185 | 0.6525 | deab478dd702 | 1 |
| 15 | 79.835781 | w402c3_r02_p10_c07 | xbabe3 | archive:jarambit_evo3c/out | 6438f0778 | 83.2234 | 0.6525 | 74bb43054495 | 1 |

## Top 5 inflated-class genomes (for reference only)

| # | score | label | host | run | evaluator | mass_3d_g | solid_tip_sf | genome sha12 | assessments of this genome |
|---:|---:|---|---|---|---|---:|---:|---|---:|
| 1 | 92.290346 | w290_r03_p14_c04 | xbabe2 | jarambit/out | 1e93e6f31 | 74.7873 | 1.509 | 4329ea676799 | 1 |
| 2 | 92.289207 | w290_r02_p14_c08 | xbabe2 | jarambit/out | 1e93e6f31 | 74.7917 | 1.509 | 94befff03689 | 1 |
| 3 | 92.281184 | w289_r01_p14_c12 | xbabe2 | jarambit/out | 1e93e6f31 | 74.8227 | 1.509 | 764075cd1128 | 1 |
| 4 | 92.276602 | w290_r02_p14_c10 | xbabe2 | jarambit/out | 1e93e6f31 | 74.8404 | 1.509 | 018025848158 | 1 |
| 5 | 92.276602 | w290_r03_p14_c08 | xbabe2 | jarambit/out | 1e93e6f31 | 74.8404 | 1.509 | 909848b50c2d | 1 |

## Notes

- **The top genome was re-assessed 7 times and always scored the same.** `w406t3_r01_p10_c16` (genome `83877fdbb08a`)
  was carried as an elite into w407c3/t3, w408c3/t3 and w410c3/t3. There it was re-assessed under evaluators
  d046093, f1b2671 and 90f77f3, and it scored 80.299390 every time. Under 1eee4d1 it also scored 80.299390. This
  agrees with your finding that the 2026-10-17/18 evaluators reproduce under v2.2.0.
- **Scores drop by about 4 points from 2026-10-18 14:30Z on.** That starts with w321 and w411c3/t3, at about
  71-74. Those waves' designs are heavier (≈ 101-114 g, tip ≈ 0.35). They are not a re-scoring of the 83 g p10
  designs.
- **The added waves have no Stage-1 values.** Their screens predate `ultralight_weighted_fitness`, so they cannot
  be plotted on the Stage-1 axis. The w284-w297 waves (both hosts) have no screens at all.
- **Labels repeat across hosts.** Wave labels such as `w284_r01_...` (xbabe2) and `w284x3_r01_...` (xbabe3) do not
  collide. Still, use `(host, run, island, label)` as the key.
- **Remaining 1e93e6f3 scores need a re-run to be usable.** The 98 1e93e6f3 rows and 44 4cd0f994e rows are
  `inflated_1e93e6f3`. If they are wanted on the plot, they need a re-assessment like the one for
  `w290_r03_p14_c04` (79.166355). Their canonical values here (81.2-92.3) should not be plotted as is.

---

# All evolution runs on canonical W_ULTRALIGHT_v1: data census

Collected 2026-10-22, read-only, from xbabe1-xbabe4. The live trees were read in one pass at
**14:40:42Z-14:40:45Z**, so islands that are still running will keep adding rows after that. The archived
tarballs on xbabe3 are static.

## What's in this folder

| file | rows (excl. header) | content |
|---|---:|---|
| `stage2_all.tsv` | 789 | One row per `*_master_scorecard.json` with a sibling `*_rs.json`. Score = `compute_ultralight_weighted_fitness(scorecard, rs)['global_weighted_fitness']` from the `evo-v2.2.0` lane on each host (blob 9cfb4416 checked on xbabe1-3). There are two extra trailing columns: `evaluator_commit` (the scorecard's `source_commit`) and `evaluator_has_w_ultralight_v1`. |
| `stage1_all.tsv` | 47169 | One row per screened candidate in `round_XX_screen.json`. The value is the row's `ultralight_weighted_fitness`, the only W_ULTRALIGHT-type key found in any screen. It is empty for 556 rows from runs whose screens predate the field. |
| `islands.tsv` | 225 | One row per island or wave directory, from `island.json` and `EXIT`. When there is no EXIT file, `exit_code` shows a `STOPPED_*`/`EXIT_*` marker file. Evo_full islands keep their time in `added_batch_utc`, which is used as `added_utc`. Two extra columns: `label_prefix`, `island_dir`. |
| `runs_found.txt` | | Every run tree found on every host, extracted or not, with counts and a description. |
| `scripts/`, `final/` | | `scripts/census.py` (directory trees) and `scripts/tarcensus.py` (tarballs, streamed) were piped over ssh as `python3 -I -`. `scripts/assemble.py` and `scripts/summarize.py` build the outputs. `final/raw_*.tsv` holds the per-host raw records, `final/records.json` the warnings, schemas and excluded runs. `work/` has the discovery scans. |

**Column conventions**
- `round` is the NN parsed from the label (`_rNN_<family>_cNN`). This also covers `_rNN_control_cNN`. The label round is always the screen-file index + 1: `round_00_screen.json` holds the `_r01_` candidates. This holds for all 47169 Stage-1 rows.
- `run` is the top directory under `~/jarambit_runs` (`evo_full_20261018`, `evo_continuous`). Non-standard subtrees get a suffix: `evo_continuous:_superseded_dup_seed_20261022` and `evo_full_20261018:_aborted_islands_v200_unpatched_0124Z`. The 2026-10-18/19 waves are `jarambit/out`, which means `~/jarambit/out` on xbabe1 and xbabe2. The waves read from the xbabe3 hygiene tarballs are `archive:jarambit_evo3c/out` and `archive:jarambit_evo3u/out`, and their `scorecard_path` and `screen_file` are `<tarball>::<member>`.
- Labels are not globally unique. The superseded xbabe1 islands reuse `ec1i001..3`, and the w412/w413 c3/t3 pairs share some genomes. Use `(host, run, island, label)` as the key.

**Inclusion rule.** A run is extracted when either of these holds:
- its Stage-1 screens carry a numeric `ultralight_weighted_fitness`;
- at least one of its Stage-2 scorecards was produced by an evaluator commit that descends from `9967a93` ("land W_ULTRALIGHT_v1 global weighted fitness", 2026-10-18T18:08Z). That is the commit that added `ops/ci/ultralight_weighted_fitness.py`.

Every included run is listed below. Each pre-W_ULTRALIGHT tree is listed in `runs_found.txt` with its counts and is not extracted.

Four Stage-2 rows inside included runs come from evaluators slightly older than 9967a93 and are flagged `evaluator_has_w_ultralight_v1=no`:

| label | evaluator | score |
|---|---|---:|
| w501_xb1_r01_p10_c80 | 9f04b47 | 71.097880 |
| w412c3_r01_p10_c19 | 513e215 | 72.324730 |
| w412c3_r01_p14_c33 | 513e215 | 72.186389 |
| w412t3_r01_p10_c19 | 513e215 | 72.324730 |

Leaderboard-stored scores were never used. **xbabe4 has no Stage-2 data.** Its 4 `evo_full_20261018` islands each ended with "Completed assessments: 0; unverified: 15", so only their 1684 Stage-1 rows are included. Its checkouts' scorer is blob dba5ff35, not 9cfb4416, but no scoring or copying was needed, so `xbabe4-copies/` does not exist.

## Counts per host and run

| host | run | islands | islands with Stage-1 | Stage-1 rows | islands with Stage-2 | Stage-2 assessments | distinct Stage-2 genomes | best Stage-2 (canonical) | best Stage-1 |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| xbabe1 | evo_continuous | 6 | 6 | 2175 | 6 | 50 | 50 | 76.141571 | 72.074248 |
| xbabe1 | evo_continuous:_superseded_dup_seed_20261022 | 3 | 3 | 263 | 3 | 4 | 4 | 75.448059 | 71.391426 |
| xbabe1 | jarambit/out | 3 | 3 | 663 | 2 | 25 | 25 | 74.358394 | 74.018130 |
| xbabe2 | evo_continuous | 17 | 16 | 5104 | 16 | 129 | 129 | 76.127401 | 71.813888 |
| xbabe2 | evo_full_20261018 | 96 | 86 | 21054 | 40 | 326 | 326 | 75.463948 | 74.130657 |
| xbabe2 | evo_full_20261018:_aborted_islands_v200_unpatched_0124Z | 24 | 0 | 0 | 0 | 0 | 0 |  |  |
| xbabe2 | jarambit/out | 3 | 3 | 498 | 2 | 7 | 7 | 74.333984 | 73.955144 |
| xbabe3 | archive:jarambit_evo3c/out | 3 | 3 | 401 | 2 | 18 | 12 | 72.398819 | 74.018130 |
| xbabe3 | archive:jarambit_evo3u/out | 2 | 2 | 328 | 2 | 17 | 11 | 74.050404 | 74.130657 |
| xbabe3 | evo_continuous | 16 | 16 | 7247 | 8 | 53 | 53 | 75.644245 | 71.462968 |
| xbabe3 | evo_continuous:_superseded_dup_seed_20261022 | 8 | 8 | 553 | 0 | 0 | 0 |  | 71.400159 |
| xbabe3 | evo_full_20261018 | 24 | 24 | 7199 | 21 | 160 | 160 | 75.983539 | 74.130657 |
| xbabe3 | evo_full_20261018:_aborted_islands_v200_unpatched_0124Z | 16 | 0 | 0 | 0 | 0 | 0 |  |  |
| xbabe4 | evo_full_20261018 | 4 | 4 | 1684 | 0 | 0 | 0 |  | 74.130657 |
| **all** | | 225 | | 47169 | | 789 | 776 | | |

Stage-2 per host: {'xbabe1': 79, 'xbabe2': 462, 'xbabe3': 248}
Stage-1 per host: {'xbabe1': 3101, 'xbabe2': 26656, 'xbabe3': 15728, 'xbabe4': 1684}

## Top 10 Stage-2 assessments (canonical W_ULTRALIGHT_v1)

| # | score | label | host | run | island | mass_3d_g | solid_tip_sf | scorecard mtime (UTC) | genome sha256 (12) |
|---:|---:|---|---|---|---|---:|---:|---|---|
| 1 | 76.141571 | ec1i005_r01_p14_c32 | xbabe1 | evo_continuous | i005 | 92.7121 | 0.3506 | 2026-10-22T12:38:24Z | f3aa930ea85b |
| 2 | 76.127401 | ec2i013_r01_p14_c56 | xbabe2 | evo_continuous | i013 | 91.8385 | 0.3444 | 2026-10-22T13:26:56Z | 9521b917855f |
| 3 | 76.126586 | ec2i011_r01_p14_c39 | xbabe2 | evo_continuous | i011 | 91.8139 | 0.3577 | 2026-10-22T13:46:25Z | 509120eb13bd |
| 4 | 76.113836 | ec2i009_r01_p14_c32 | xbabe2 | evo_continuous | i009 | 91.4069 | 0.3155 | 2026-10-22T12:07:31Z | 2fe4ce2d3dd9 |
| 5 | 76.028919 | ec2i011_r01_p14_c31 | xbabe2 | evo_continuous | i011 | 91.8041 | 0.3264 | 2026-10-22T14:27:00Z | 4672a1c73427 |
| 6 | 76.018185 | ec1i005_r01_p14_c31 | xbabe1 | evo_continuous | i005 | 92.5848 | 0.3421 | 2026-10-22T13:04:24Z | 177398ae8db2 |
| 7 | 75.985880 | ec2i013_r01_p14_c53 | xbabe2 | evo_continuous | i013 | 91.8377 | 0.3118 | 2026-10-22T14:09:12Z | e39e21a2be10 |
| 8 | 75.983539 | ef3i17_r05_p14_c06 | xbabe3 | evo_full_20261018 | i17 | 91.8133 | 0.3118 | 2026-10-20T13:06:20Z | 0c2e9810b1dc |
| 9 | 75.983539 | ec1i004_r01_p14_c68 | xbabe1 | evo_continuous | i004 | 91.8133 | 0.3118 | 2026-10-22T12:16:40Z | fa9e0ce133d4 |
| 10 | 75.983539 | ec1i004_r01_p14_c52 | xbabe1 | evo_continuous | i004 | 91.8133 | 0.3118 | 2026-10-22T12:43:01Z | 70d51741f7a1 |

## Top 10 distinct genomes (best assessment of each)

| # | score | label | host | run | n assessments of this genome |
|---:|---:|---|---|---|---:|
| 1 | 76.141571 | ec1i005_r01_p14_c32 | xbabe1 | evo_continuous | 1 |
| 2 | 76.127401 | ec2i013_r01_p14_c56 | xbabe2 | evo_continuous | 1 |
| 3 | 76.126586 | ec2i011_r01_p14_c39 | xbabe2 | evo_continuous | 1 |
| 4 | 76.113836 | ec2i009_r01_p14_c32 | xbabe2 | evo_continuous | 1 |
| 5 | 76.028919 | ec2i011_r01_p14_c31 | xbabe2 | evo_continuous | 1 |
| 6 | 76.018185 | ec1i005_r01_p14_c31 | xbabe1 | evo_continuous | 1 |
| 7 | 75.985880 | ec2i013_r01_p14_c53 | xbabe2 | evo_continuous | 1 |
| 8 | 75.983539 | ef3i17_r05_p14_c06 | xbabe3 | evo_full_20261018 | 1 |
| 9 | 75.983539 | ec1i004_r01_p14_c68 | xbabe1 | evo_continuous | 1 |
| 10 | 75.983539 | ec1i004_r01_p14_c52 | xbabe1 | evo_continuous | 1 |

## Cumulative-best timeline (Stage-2, by scorecard mtime)

| scorecard mtime (UTC) | score | label | host | run |
|---|---:|---|---|---|
| 2026-10-18T18:05:50Z | 72.324730 | w412c3_r01_p10_c19 | xbabe3 | archive:jarambit_evo3c/out |
| 2026-10-18T19:13:51Z | 72.389556 | w412c3_r02_p10_c39 | xbabe3 | archive:jarambit_evo3c/out |
| 2026-10-18T19:51:17Z | 72.745635 | w324_r01_p14_c43 | xbabe2 | jarambit/out |
| 2026-10-18T21:27:53Z | 74.319278 | w324_r02_p14_c06 | xbabe2 | jarambit/out |
| 2026-10-18T23:23:28Z | 74.319381 | w325_r01_p14_c03 | xbabe2 | jarambit/out |
| 2026-10-19T00:50:38Z | 74.358394 | w502_xb1_r01_p14_c15 | xbabe1 | jarambit/out |
| 2026-10-19T03:24:57Z | 74.470186 | ef3i09_r01_p10_c41 | xbabe3 | evo_full_20261018 |
| 2026-10-19T06:04:33Z | 75.252409 | ef2i01_r02_p14_c110 | xbabe2 | evo_full_20261018 |
| 2026-10-19T21:55:04Z | 75.258414 | ef2i91_r01_p14_c93 | xbabe2 | evo_full_20261018 |
| 2026-10-19T23:08:48Z | 75.327297 | ef2i95_r01_p14_c81 | xbabe2 | evo_full_20261018 |
| 2026-10-19T23:39:43Z | 75.395548 | ef2i95_r02_p14_c13 | xbabe2 | evo_full_20261018 |
| 2026-10-20T01:31:34Z | 75.416653 | ef2i95_r03_p14_c27 | xbabe2 | evo_full_20261018 |
| 2026-10-20T02:58:41Z | 75.455992 | ef2i95_r04_p14_c27 | xbabe2 | evo_full_20261018 |
| 2026-10-20T04:24:51Z | 75.463948 | ef2i95_r05_p14_c27 | xbabe2 | evo_full_20261018 |
| 2026-10-20T09:55:23Z | 75.882190 | ef3i17_r04_p14_c102 | xbabe3 | evo_full_20261018 |
| 2026-10-20T12:20:24Z | 75.905040 | ef3i17_r05_p14_c13 | xbabe3 | evo_full_20261018 |
| 2026-10-20T13:06:20Z | 75.983539 | ef3i17_r05_p14_c06 | xbabe3 | evo_full_20261018 |
| 2026-10-22T12:07:31Z | 76.113836 | ec2i009_r01_p14_c32 | xbabe2 | evo_continuous |
| 2026-10-22T12:38:24Z | 76.141571 | ec1i005_r01_p14_c32 | xbabe1 | evo_continuous |

The first point in the timeline (`w412c3_r01_p10_c19`, 18:05:50Z) is one of the 4 pre-9967a93 scorecards. The first point from an evaluator that has W_ULTRALIGHT_v1 is `w412c3_r02_p10_c39` at 72.389556 (19:13:51Z). Scorecard file mtime stands in for "time assessed". The evo_full_20261018 mtimes fall on 2026-10-19 (309), 2026-10-20 (135) and 2026-10-21 (42), which fits the campaign.

## Sanity check against the 2026-10-22T13:38Z scan

- **Reproduced exactly.** In the canon_scan scope (`~/jarambit_runs/evo_*/<host>/islands/*/by-content/`, xbabe1-3) with scorecard mtime ≤ 13:38Z, this census has **xbabe1 48, xbabe2 445, xbabe3 203 = 696**, the same as the earlier scan, and the same best design, `ec1i005_r01_p14_c32` = **76.141571**.
- **Now, same scope:** xbabe1 50, xbabe2 455, xbabe3 213 = **718** (the runs are live).
- **Outside that scope but included:** 71 assessments:
  - evo_continuous superseded islands on xbabe1: 4
  - `jarambit/out` on xbabe1: 25
  - `jarambit/out` on xbabe2: 7
  - xbabe3 archive `jarambit_evo3c`: 18
  - xbabe3 archive `jarambit_evo3u`: 17
- **Total Stage-2:** 789 (776 distinct genome files).
- **xbabe4:** 0 Stage-2 assessments, so nothing on xbabe4 beats or matches 76.141571. Its best Stage-1 is 74.130657.
- **Nothing in the extracted W_ULTRALIGHT_v1 runs beats 76.141571.** It is still the best canonical score of the 789. The best entries since 13:38Z are `ec2i011_r01_p14_c39` at 76.126586 (13:46:25Z) and `ec2i011_r01_p14_c31` at 76.028919 (14:27:00Z). The newest scorecards when the snapshot was taken were `ec3i012_r04_p14_c26` (14:38:41Z) and `ec1i004_r02_p14_c101` (14:38:05Z).

## Parsing problems

- **Extracted data has no problems.** There are 0 scorer errors, 0 missing `_rs.json`, 0 missing `_genome.csv`, and 0 unreadable screens. Every scorecard's `genome_sha256` field matches the sha256 of its sibling `_genome.csv`. Every Stage-2 label has a Stage-1 row in the same island.
- **86 warnings, all in excluded pre-W_ULTRALIGHT waves on xbabe2 `~/jarambit/out`:**
  - w291-w297: 44 scorecards without `_rs.json`.
  - w284-w290: 42 scorecards sitting loose in the wave dir instead of under `by-content/`.
- **Screen schema.** Every screen is a JSON list of objects with `label`, `mass_g`, `fat_sf`, `seq_ratio`, `screen_deficit`, `ring_bearing_w_mm`, `certificate_identity`, `family` and `qualification_status`, plus `ultralight_weighted_fitness` once that field exists. v2.2.0 islands also add `fem_screen_status`, `legacy_c6_aesthetic_pass` and `legacy_holder_and_geometry_pass`. 525 such rows are on xbabe2 and 100 on xbabe1, all `measured`. No other fitness key exists, and the old evo-main binary and v2.2.0 use the same value key.
- **The Stage-1 value does not depend on the binary.** I matched 13 identical designs (same mass_g, fat_sf, seq_ratio) between xbabe2 evo_full (009f5ac) and evo_continuous (49dc71f). Their Stage-1 values agree within 1e-6.

## Surprises and concerns

1. **Pre-W_ULTRALIGHT runs score higher on the canonical function, but they are not comparable.** They are excluded from the TSVs and flagged here.
   - Running the canonical scorer on Stage-2 scorecard+rs pairs from runs before 9967a93 gives values well above 76.14:
     - 2026-10-16/17 waves w284-w290 (xbabe2 `~/jarambit/out`) and w284x3-w297x3 (xbabe3 archive): up to **92.29**, with mass ≈ 75 g and solid_tip_sf ≈ 1.5.
     - 2026-10-17/18 waves w298-w320 (xbabe2) and w401c3-w410t3 (xbabe3 archive): **77.9-80.3**. The best is `w406t3_r01_p10_c16` at 80.299390 (83.2 g, tip 0.68). The repo's committed `evidence/evolution_plateau/2026-10-17` copies give 79.87 (`w401c3_r02_p10_c27`).
   - These scorecards share the current schema (`jarambit.master_cad_scorecard.v1`, `genome_and_3d_cad_full_battery`), but every one of them comes from an evaluator commit older than 9967a93 (1e93e6f … b64f156).
   - The level appears to be evaluator-driven. On 2026-10-16 the battery reported solid_tip_sf ≈ 1.5 at 75 g. From the w321/w411c3 waves on (2026-10-18 ≈14:30Z), the same function gives ≈ 71-74, and every later run stays at 71-76.
   - Comparing `w401c3_r02_p10_c27` (79.87) with `ec1i005_r01_p14_c32` (76.14), the difference comes from the measured metrics, mostly `solid_tip_sf_nodal_max` (0.67 vs 0.35), twist tip SF (0.53 vs 0.29) and mass (82.9 vs 92.7 g).
   - I treat these runs as not comparable. That is an inference: it was not verified by re-running a genome.
   - No genome from those runs was ever Stage-2 re-assessed under a post-9967a93 evaluator. That includes the `evo_full_20261018` seed `w298_r03_p14_c01` (same sha256 d6653791…). The claim "76.14 is the best design ever evolved" therefore rests on that incomparability, not on a measured head-to-head.
   - Re-assessing `w406t3_r01_p10_c16`, `w401c3_r02_p10_c27` and `w298_r03_p14_c01` with the v2.2.x evaluator would settle it.
2. **Stage-1 and Stage-2 values are on different absolute scales.** For the 759 designs that have both, the canonical Stage-2 score is higher than the Stage-1 screen by +1.80 to +4.54 (mean +3.36). The best Stage-1 anywhere is 74.130657, reached by 50 different labels across evo_full and evo3u; it looks like a seed-derived design. A figure that puts both stages on one axis needs a caption that says this.
3. **Duplicate-seed islands.** `_superseded_dup_seed_20261022` on xbabe1 (3 islands) and xbabe3 (8) reran xbabe2's RNG seeds and parents for about 1-1.6 h before being killed (exit 143) at 00:20Z. One xbabe1 Stage-2 genome is byte-identical to xbabe2's (`ec1i002_r01_p14_c18` = `ec2i002_r01_p14_c18`, 75.215225). They are included under their own `run` value so they can be dropped.
4. **Equal scores for different genome files.** For example, `ef3i17_r05_p14_c06`, `ec1i004_r01_p14_c52` and `ec1i004_r01_p14_c68` all score 75.983539 with mass 91.8133 g. Their genome files differ in a few genes that apparently do not move the scored metrics; these look like migrants and their near-clones.
5. **Some islands produced nothing.** On xbabe2, evo_full islands i61-i70 never screened (i70 has `STOPPED_MEMORY`). `evo_continuous/xbabe2/islands/_failed_i001_path_bug` reports "certification tool build failed". The 24 + 16 `_aborted_islands_v200_unpatched_0124Z` dirs hold births only. All of these are in `islands.tsv` with zero counts.
   - `exit_code` values across the 225 rows:

     | exit_code | islands | meaning |
     |---|---:|---|
     | 0 | 12 | |
     | 1 | 38 | |
     | 130 | 8 | signal |
     | 137 | 4 | signal |
     | 143 | 21 | signal |
     | `shed_memory` | 5 | memory shedding on xbabe2/3 evo_full |
     | `STOPPED_MEMORY` | 27 | marker file, no EXIT |
     | empty | 110 | still running, or ended without EXIT/marker; wave dirs and archived waves have no EXIT |

   - Exit 0 versus 1 does not track qualification: no island qualified, and both codes occur with unverified candidates.
6. **Some raw outputs exist only in archives.** The raw 2026-10-18/19 xbabe3 waves (w409_ul, w412c3/t3, w413c3/t3) survive only inside the hygiene tarballs. They were read by streaming (`zstd -dc | tarfile`), and nothing was unpacked. The removed lane `~/jarambit_wt_agy_ul` on xbabe2 kept only `dirty.patch`, so any runs it held would exist only on S3, which was not scanned.
7. **Search coverage.** The search was exhaustive on xbabe1, xbabe3 and xbabe4. On xbabe2, every jarambit tree and the jarambit lanes in the hygiene archive were searched completely, but I stopped the whole-disk `find` over the multi-TB non-jarambit dataset trees after about 50 min. S3 was not scanned. Details are in `runs_found.txt`.
