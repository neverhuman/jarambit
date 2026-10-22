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
