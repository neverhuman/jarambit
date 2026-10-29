<!-- leader-gallery:begin (tools/refresh_leader_assets.py) -->

## G5 leader renders: `kv4i1404_g00581_29ea3c73` (current pipeline)

Blender renders of the current G5 pipeline's leader `kv4i1404_g00581_29ea3c73` (W_ERGOLIGHT_v2 74.40 on the h 0.2 mm ranking grid, mass 88.4 g, lattice porosity 0.58, internal void fraction of the AM-build mesh 0.53) from its AM-build STL (1,030,568 triangles). Regenerate with `python3 tools/refresh_leader_assets.py` ([how](docs/LEADER_ASSETS.md)). Finishes are illustrative; cuts are open so the printed internal lattice shows.

<img src="docs/assets/leader/xray.jpg" alt="X-ray cutaway of kv4i1404_g00581_29ea3c73" width="100%">

<img src="docs/assets/leader/hero.jpg" alt="Leader kv4i1404_g00581_29ea3c73" width="100%">

<table><tr><td width="50%"><img src="docs/assets/leader/reverse.jpg" alt="Reverse oblique"><br>Reverse oblique.</td><td width="50%"><img src="docs/assets/leader/side.jpg" alt="Plan view"><br>Plan view.</td></tr><tr><td width="50%"><img src="docs/assets/leader/top.jpg" alt="Edge-on"><br>Edge-on.</td><td width="50%"><img src="docs/assets/leader/ring.jpg" alt="Ring"><br>Ring.</td></tr><tr><td width="50%"><img src="docs/assets/leader/grip.jpg" alt="Grip and pockets"><br>Grip and pockets.</td><td width="50%"><img src="docs/assets/leader/blade.jpg" alt="Blade"><br>Blade.</td></tr><tr><td width="50%"><img src="docs/assets/leader/tip.jpg" alt="The point"><br>The point.</td><td width="50%"><img src="docs/assets/leader/tip_edge.jpg" alt="Edge at the point"><br>Edge at the point.</td></tr><tr><td width="50%"><img src="docs/assets/leader/cut_full.jpg" alt="Mid-plane cut (z = 0)"><br>Mid-plane cut (z = 0).</td><td width="50%"><img src="docs/assets/leader/cut_x.jpg" alt="Section across the grip"><br>Section across the grip.</td></tr><tr><td width="50%"><img src="docs/assets/leader/cut_b.jpg" alt="Section across the blade"><br>Section across the blade.</td></tr></table>

<!-- leader-gallery:end -->

## Legacy-pipeline run (v6.1.0 addendum — older, solid, non-lattice design line; not G5, not the cohort of record)

The most recent legacy continuous island run (legacy v1 W_ULTRALIGHT_v1 pipeline, 103 islands on xbabe2, stopped 2026-10-29) found `ec2i089_r01_p10_c68`: fitness **80.851214**, mass **83.409 g** (3D 83.3333 g), **13 of 26** legacy hard gates met. It misses the hunter 260 N load (201–202 N), tip twist and solid-tip safety factors (0.52–0.71 vs 1.0), fatigue (0.56), human maximum, sharpness law and tip profile, so it is a research result, not a qualified design. Full gate table and plots: [docs/LATEST_RUN.md](docs/LATEST_RUN.md); evidence: [`evidence/legacy_continuous_run_v6.1.0/`](evidence/legacy_continuous_run_v6.1.0/).

![Island convergence of the latest run](docs/assets/legacy_run_v6.1.0/convergence_islands.png)

![Stage 1 mass vs fitness](docs/assets/legacy_run_v6.1.0/pareto_stage1_mass_vs_fitness.png)
