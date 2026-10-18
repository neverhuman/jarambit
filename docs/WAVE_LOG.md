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
| 298, 300–314 (`xbabe2`) | v1.0.0-retracted (post-#84 `./evolve.sh`) | 16 waves complete (`68` Stage-2 assessments, `0` qualified) | Coupled Stage-1 + Stage-2 (`scikit-fem` $P_2$ + `DOLFINx` v0.9.0 $P_2$ + OpenCascade B-rep); seed diversity (#87) + `plate.gci` binding (#91) | `0` hard PASS (`qualified=0`; best `W_CAD = -5.224322` on `w309_r02_p14_c35`, `-5.224336` on `w313_r02_p14_c26`, `-5.320623` on `w314_r01_p14_c56`; best `G1 = 0.4445` on `w314_r01_p10_c21`) | `evidence/evolution_waves/2026-10-17/` |
| 400c3–404c3 (`xbabe3`) | v1.0.0-retracted (post-#84 `evo3c`) | 5 waves complete (`17` Stage-2 assessments, `5` unverified, `0` qualified) | Secondary host 3-round automatic evolution (`w401c3`, `w402c3`, `w404c3` fully verified; `w400c3`, `w403c3` have unverified entries) | `0` hard PASS (`qualified=0`; best verified `W_CAD = -5.496513` on `w404c3_r03_p10_c15`) | `evidence/evolution_waves/2026-10-17/` |

Wave27.1-class micro experiments (docs/infra/re-plots, no new major arms) are OK on 0.27.x without a new tag. Next major (wave28 ↔ v0.28.0) only after a hard-PASS G1 > 0.9805 or ≤ 82 g @ tip×SPG ≥ 6 hard PASS.
