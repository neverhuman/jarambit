# WAVE LOG — jarambit (author Jepson Taylor)

Every wave and sub-experiment. Major waves map to versions (wave N ↔ 0.N.0); sub-experiments are `Wave N.k` and do not bump or tag.

| wave | version | status | key change | best G (hard PASS) | where |
|---|---|---|---|---|---|
| 1–12 | 0.1–0.12 (retro) | done | NSGA-II islands, MC, plate FEM, crescent lane (6), C.6 anchor (11–13) | — | `docs/figures/wave_heatmaps/` |
| 13–17 | 0.13–0.17 (retro) | done | D1 rules (828 MPa joint, strict 0.0625, seq, twist both modes); CI d1-gates | — | `docs/wave_reports/` |
| 18–19 | 0.18–0.19 (retro) | done | controlled RSC trial; outline lane | — | exec/wave18-controlled, exec-w19o |
| 20 | 0.20 (retro) | done | W20 lock: SOLID tip gate, seq soft, holder +1 mm | 0.848 (c11) | exec-w20d `docs/wave20_diversity` |
| 21 | 0.21 (retro) | done | P1 rail OFF arm | 0.808 (c08) | exec-w21d |
| 22 | 0.22 (retro) | done | diversity | 0.885 (c03) | exec-w22d |
| 23 | 0.23 (retro) | done | ring fix arm | 0.925 (c02) | exec-w23d |
| 24 | 0.24 (retro) | done | diversity | 0.966 (c02) | exec-w24d |
| 25 | 0.25 (retro) | done | novelty arms + graft | 0.952 (c01) | exec-w25d |
| 26 | 0.26 (retro) | D1 screen r4 done; solids/plates via `jarambit_w26_resume` | g3 pocket/line arms, grind law | pending (solid table: ctl_g3_ref SF 1.525) | exec-w26d |
| 27 | **0.27.0 (tag v0.27.0) — MAJOR** | done | tip outline × t_stock × t_tip arms (+ b/c/d/e) | **0.9805 (g3_tip+00_t60_tt3, engineering provisional, not promoted)** | exec-w27d `docs/wave27_diversity` |
| 27.1 | (none) | repo reorg + G1 + versioning | docs/infra only | — | exec/repo-reorg |
| 298, 300–317 (`xbabe2`) | v1.0.0-retracted (post-#84 `./evolve.sh`) | 19 waves complete (`78` Stage-2 assessments, `0` unverified, `0` qualified) | Coupled Stage-1 + Stage-2 (`scikit-fem` $P_2$ + `DOLFINx` v0.9.0 $P_2$ + OpenCascade B-rep); seed diversity (#87) + `plate.gci` binding (#91) | `0` hard PASS (`qualified=0`; best `W_CAD = -5.183108` on `w309_r02_p10_c09` / `w310_r01_p10_c28`, `-5.196612` and best `solid_tip_SF = 0.7163` on `w309_r02_p14_c35`, `-5.224336` on `w313_r02_p14_c26`, `-5.320623` on `w314_r01_p14_c56`, `-5.258458` on `w315_r01_p14_c50` / `w316_r01_p14_c27` / `w317_r01_p14_c27` (`solid_tip_SF = 0.7091` on `w315_r01_p10_c03` / `w316_r01_p10_c01` / `w317_r01_p10_c01`); best `G1 = 0.4689` on `w300_r01_p10_c68`, `0.4445` on `w313_r01_p10_c09` / `w314_r01_p10_c21`, `0.4444` on `w315_r01_p10_c03` / `w316_r01_p10_c01` / `w317_r01_p10_c01`) | `evidence/evolution_waves/2026-10-17/` |
| 400c3–407t3 (`xbabe3`) | v1.0.0-retracted (post-#84 `evo3c`) | 10 waves complete (`48` Stage-2 assessments, `6` unverified, `0` qualified) | Secondary-host 3-round evolution; paired `w406c3`/`w406t3` (`1eee4d1`) and `w407c3`/`w407t3` (`d046093`) tip-band A/B (control bias 0, treatment bias 1) | `0` hard PASS (`qualified=0`; across `w406c3`–`w407t3` all boards report `14/18` battery passes; `w406t3_r01_p10_c16` / `w407t3_r01_p10_c29`: `W_CAD = -5.252210`, `G1 = 0.479637`, `solid_tip_SF = 0.6823`; `w406t3_r02_p10_c23`: `strict_hunter_sf = 0.7058`, `fat_sf = 0.5092`) | `evidence/evolution_waves/2026-10-17/xbabe3/out/` |

The newly committed wave records in PR #103 (`b38642b37e6231cda97b07d73cdadec420e23aba`) and PR #104 are `w315`, `w316`, and `w317` on `xbabe2` (`4` assessed candidates each, `0` qualified, `0` unverified; adding `12` assessments to the prior `66` for `78` total across `19` `xbabe2` waves, with `8` waves `w310`–`w317` spanning `32` post-`w309` assessments) and `w405c3`, `w406c3`, `w406t3`, `w407c3`, and `w407t3` on `xbabe3` (`6` verified assessments each on `w405c3`, `w406c3`, `w406t3`, and `w407t3` plus `5` unverified on `w407c3`; adding `29` assessments to the prior `19` for `48` total across `10` `xbabe3` waves, `29` waves / `126` assessments overall). The `w406c3`/`w406t3` pair (`1eee4d1`) and `w407c3`/`w407t3` pair (`d046093`) test `JARAMBIT_TIPBAND_BIAS=0` vs `1` on matched seeds, and `w317` (`0a6aa94`, `--seed 3170`) evaluates the witness-guided search arms on `xbabe2`. All listed battery values are recorded screen scores, not all-gate qualifications (`w406t3_r01_p10_c16` / `w407t3_r01_p10_c29`: `W_CAD = -5.252210`, `G1 = 0.479637`, `solid_tip_sf = 0.6823`; `w406t3_r02_p10_c23`: `strict_hunter_sf = 0.7058`, `fat_sf = 0.5092`; `w317_r01_p14_c27`: `W_CAD = -5.258458`, `solid_tip_sf = 0.7030`; `w317_r01_p10_c01`: `solid_tip_sf = 0.7091`).

Wave27.1-class micro experiments (docs/infra/re-plots, no new major arms) are OK on 0.27.x without a new tag. Next major (wave28 ↔ v0.28.0) only after a hard-PASS G1 > 0.9805 or ≤ 82 g @ tip×SPG ≥ 6 hard PASS.
