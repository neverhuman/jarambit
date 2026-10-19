# docs/ — paper, versioning, logs, calibration

Start with the [IEEE-style research white paper](paper/README.md), [complete CAD test atlas](tests/README.md), [178-case science index](tests/science.md), and [detailed engineering specification](PAPER_AND_README_ENGINEERING_SPEC.md). Existing long technical references remain below.

| file | purpose |
|---|---|
| `WHITEPAPER.md` | living paper (author Jepson Taylor), header carries version + date |
| `ENGINEERING_GATES_AND_EVOLUTION.md` | comprehensive reference paper covering all 18 CAD battery evaluators, geometric/red-team/aesthetic/FEM/solid/RS-karambit-v2 gates, the 178-group `T00`–`T19` science suite, and multi-wave evolutionary dynamics (`w1` $\rightarrow$ `w273`) |
| `WHITEPAPER_g3_neckrail.md` | frozen v0.1.0 g3_neckrail paper (branch `release/v0.1.0`, tag `v0.1.0-g3neckrail`) — kept, not superseded |
| `GLOBAL_METRIC.md` | locked definition of G |
| `VERSION` | semver of the research state (`0.32.0`) |
| `RELEASES.md` | changelog per major wave / tag |
| `WAVE_LOG.md` | every wave and sub-experiment (Wave N.1 …) — no bump/tag for sub-experiments |
| `WAVE_FIGURE_TEMPLATE.md` | plate layout contract |
| `CALIBRATION.md`, `calibration/` | FEM calibration + convergence |
| `HOLDER_ENVELOPE.md` | holder/sheath envelope |
| `figures/whitepaper/` | best-G plates + 3D CAD renders used by the paper |
| `figures/engineering_gates/` | manifest-verified evolutionary trajectory, 18-evaluator CAD battery matrix, and 3D CAD/FEM gallery figures |

Feeds G: GLOBAL_METRIC.md is the authority; the paper Methods section quotes it.
