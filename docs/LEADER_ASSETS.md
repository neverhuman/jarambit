# Refreshing the leader's images (manual)

One command regenerates every image of the current leader that `README.md` and the G5 paper show:

```sh
python3 tools/refresh_leader_assets.py                       # leader = max fitness in evidence/legacy_continuous_run_v6.1.0/leaders/fitness.json
python3 tools/refresh_leader_assets.py --design ec2i089_r01_p10_c68
python3 tools/refresh_leader_assets.py --plan-only           # print the resolved leader and STL only
```

What it does, in order (rerunning with the same inputs rewrites the same files):

1. **Leader**: `--design ID`, otherwise the best entry of `<evidence>/leaders/fitness.json` (`--evidence` to point at another run).
2. **Geometry**: uses `<evidence>/leaders/ID.stl`; if absent, finds the run's own exported `ID.stl` under `--runs-root`
   (default `~/jarambit_runs/evo_continuous`) and commits a binary-STL copy there. Nothing is re-exported, re-solved or re-scored.
3. **Blender** (`ops/render/blender/leader_views.py`, Blender 4.5.3 Cycles on CPU, `nice 19`, 8 threads by default):
   `hero, reverse, side, top, ring, grip, blade, tip, tip_edge, cut_full, cut_x, cut_b` → `docs/assets/leader/VIEW.jpg`.
   Cameras are framed from the mesh, so any design works. Finishes are illustrative; section faces are the real mesh cut by a plane.
4. **Evolution plots**: `ops/render/leader/aggregate_run.py` over `--run-root` (default `~/jarambit_runs/evo_continuous/xbabe2`)
   refreshes `docs/assets/legacy_run_v6.1.0/*.png` in place (skipped if the run root is absent; `--skip-plots`).
5. **Docs**: rewrites the leader-gallery block of `docs/README_LATEST_RUN.md`, writes `docs/g5/paper/generated/leader.tex`,
   runs `ops/ci/gen_readme.py` (the README hero strip and latest-run section pick the images up), and rebuilds
   `docs/g5/paper/g5-v<VERSION>.pdf` with latexmk (`--skip-paper` to skip).
6. **Receipt**: `docs/assets/leader/manifest.json` (design, STL hash, renderer hash, every image hash) and a printed list of
   changed files with SHA-256 before → after.

Then commit, open the docs PR and bump `docs/VERSION` as usual.

The G5 record images under `docs/g5/v6.0.0/img/` belong to the G5 receipt (hash-bound in `evidence/g5/v6.0.0/SHA256SUMS`) and
show the record design `kv4i1404_g00581_29ea3c73`; this tool does not overwrite them. Fast check: `--samples 8 --res 800x500`.
