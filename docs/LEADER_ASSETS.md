# Refreshing the leader's images (manual)

One command regenerates every image of the current leader that `README.md` and the G5 paper show:

```sh
python3 tools/refresh_leader_assets.py                       # leader = best Tier-2 (h 0.2 mm) design of the newest G5 run
python3 tools/refresh_leader_assets.py --design kv4i1406_g00009_528e4365   # another G5 design from that ranking
python3 tools/refresh_leader_assets.py --allow-legacy --evidence evidence/legacy_continuous_run_v6.1.0   # legacy v1 line only
python3 tools/refresh_leader_assets.py --plan-only           # print the resolved leader and STL only
```

What it does, in order (rerunning with the same inputs rewrites the same files):

1. **Leader**: current pipeline only by default. The newest `~/jarambit_runs/evo_g5*/cert/tier2_h020.jsonl` (schema `G5.*`),
   highest `score_100`, cross-checked against `evidence/g5/v<RECORD>/release.json`. Legacy `ec*` designs / `--evidence` are
   refused unless `--allow-legacy` (v6.1.1 rendered the legacy `ec2i089_r01_p10_c68` by mistake).
2. **Geometry**: the run's AM-build STL `<run>/prep/cad/out/ID_AMbuild_mm.stl` (internal lattice voids). The tool computes the
   void fraction 1 − V(AM-build)/V(envelope) and stops below 5 % (`--allow-solid` overrides). Nothing is re-solved or re-scored.
3. **Blender** (`ops/render/blender/leader_views.py`, Blender 4.5.3 Cycles on CPU, `nice 19`, 8 threads by default):
   `hero, xray, reverse, side, top, ring, grip, blade, tip, tip_edge, cut_full, cut_x, cut_b` (G5: open cuts so voids show; `xray` = mid-thickness cutaway) → `docs/assets/leader/VIEW.jpg`.
   Cameras are framed from the mesh, so any design works. Finishes are illustrative; section faces are the real mesh cut by a plane.
4. **Evolution plots**: G5 mode keeps the receipt-bound run plots in `docs/g5/v<RECORD>/img/` (evolution, fitness terms/budget, lineage).
   Legacy mode only: `ops/render/leader/aggregate_run.py` refreshes `docs/assets/legacy_run_v6.1.0/*.png`.
5. **Docs**: rewrites the leader-gallery block of `docs/README_LATEST_RUN.md`, writes `docs/g5/paper/generated/leader.tex`,
   runs `ops/ci/gen_readme.py` (the README hero strip and latest-run section pick the images up), and rebuilds
   `docs/g5/paper/g5-v<VERSION>.pdf` with latexmk (`--skip-paper` to skip).
6. **Receipt**: `docs/assets/leader/manifest.json` (design, STL hash, renderer hash, every image hash) and a printed list of
   changed files with SHA-256 before → after.

Then commit, open the docs PR and bump `docs/VERSION` as usual.

The G5 record images under `docs/g5/v6.0.0/img/` belong to the G5 receipt (hash-bound in `evidence/g5/v6.0.0/SHA256SUMS`) and
show the record design `kv4i1404_g00581_29ea3c73`; this tool does not overwrite them. Fast check: `--samples 8 --res 800x500`.
