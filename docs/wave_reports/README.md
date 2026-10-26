# WAVE REPORTS (per research wave) — jarambit, SWITCHBACK-class titanium hiking/utility karambit

Owner directive (Jepson Taylor, 2026-10-11): every experiment wave gets one report pack, same layout every wave.

## Pack (per wave N, this directory)
| file | content |
|---|---|
| `wave{N}_REPORT.md` | title `WAVE N | fitness/SPG | mass g | stock mm`, KPI dashboard, FEM visuals + peak table, break-point summary, weight ledger, all D1-screened designs, paper-data summary, gates context |
| `wave{N}_FEM_metrics.png` | 300 dpi (6000×3960 px): von Mises lateral 260 N / chord torsion 1.82 N·m / ring pull 1500 N, envelope SF (worst case), body+tip twist SF, tip zoom (tip-only), thickness field, gate table, peak table |
| `wave{N}_paper_data.csv` | one row per D1-screened design: genome_hash, geometry_hash, solver_commit, load_case_version, mesh sizes, capacities by mesh, joint thin+hot, tip-only vs body+tip twist, ring, hand/blade, mass bare+carry, aesthetic, novelty, unqualified regions, final_edge, timestamps |
| `wave{N}_paper_data.json` | full pack: per-design gates, peaks, ledger, deltas vs prior wave, provenance, list of NOT VERIFIED fields |

Badges are three-state: **PASS / FAIL / NOT VERIFIED**. A missing value is NOT VERIFIED and is never green. Human aesthetic sign-off is always NOT VERIFIED unless `signoff/<design>.signoff` exists and starts with `APPROVED` (written by Jepson only). OVERALL = FAIL if any gate fails, else NOT VERIFIED if anything is unverified, else PASS. **The tool never promotes.**

## Gates (CURRENT D1)
Joint thin+hot 828 MPa (provisional); joint lateral ≥ 260 N at certification min(0.25, 0.125 mm²) **and** strict 0.0625 mm² (aim 286, reported); hiker 180; ring pull 1500; chord torsion 1.82 N·m **both** modes — tip-only (default hard) and body+tip = min(body >15 mm, tip-submodel strict); hand/blade yield-onset ratio ≥ 1.2; mass band 74–77 g, cap < 80 g; blade ≥ 3.375 in on tip-heel / reach / sharpened arc; C.6 aesthetic floor (automated) + human sign-off.

## Generators
- `ops/wave_reports/make_wave_report.py` — offline Python (plotting/reporting only).
- `ops/wave_reports/post_wave_hook.sh N RUN_DIR [SOLVER]` — **evolve post-hook** for wave17+. Append after the evolve command in a run script (`...; ops/wave_reports/post_wave_hook.sh 18 out/wave18_x`), with `WAVE_REPORT_COMMIT=1` to commit+push.
- `ops/wave_reports/watch_remote_wave.sh N HOST REMOTE_LOG REMOTE_DIR` — xbabe2 watcher for runs on another host: waits for the `EXIT` line the screen wrapper appends, rsyncs the run (no binaries/STL/STEP), runs the hook, commits+pushes. Read-only on the remote run.

## Wave index
| wave | headline | OVERALL | report |
|---|---|---|---|
| 13 | wave13_c01 | FAIL | [wave13_REPORT.md](wave13_REPORT.md) |
| 14 | wave14_c03 | FAIL | [wave14_REPORT.md](wave14_REPORT.md) |
| 15 | wave15_c01 | FAIL | [wave15_REPORT.md](wave15_REPORT.md) |
| 16 | wave16_c01 | FAIL | [wave16_REPORT.md](wave16_REPORT.md) |
| 17 | pending (watcher: out/WATCH_wave17.txt) | NOT VERIFIED (run in progress) | — |
