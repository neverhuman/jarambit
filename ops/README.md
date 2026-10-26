# ops/

Scripts that check, generate and render the repository. Rust tools live in [`../crates/`](../crates/); Python here is
allowed only as listed in [`../PYTHON_EXCEPTIONS.md`](../PYTHON_EXCEPTIONS.md).

| path | what it does | run by |
|---|---|---|
| [`g3/export_best_stl.sh`](g3/export_best_stl.sh) | exports the G3 winner's print and envelope STL files and the `cad/` upload kit | by hand, at a release |
| [`ci/gen_readme.py`](ci/gen_readme.py), [`ci/gen_status.py`](ci/gen_status.py), [`ci/gen_whitepaper.py`](ci/gen_whitepaper.py) | generate `README.md`, `cad/README.md`, `STATUS.md` and the white-paper blocks; `--check` fails on drift | `version-check` workflow |
| [`ci/check_claims.py`](ci/check_claims.py), [`ci/claim_admission.py`](ci/claim_admission.py), [`ci/rs_verdicts.py`](ci/rs_verdicts.py) | the claim checker for the public documents | `version-check` workflow |
| [`ci/verify_figures.py`](ci/verify_figures.py), [`ci/verify_media.py`](ci/verify_media.py), [`ci/figures_required.txt`](ci/figures_required.txt) | check every figure and media file against its manifest | `version-check` workflow |
| [`ci/score_cad.py`](ci/score_cad.py), [`ci/cad_battery.py`](ci/cad_battery.py), [`ci/evaluate_cad_candidate.py`](ci/evaluate_cad_candidate.py), [`ci/verify_genome_export.py`](ci/verify_genome_export.py), [`ci/side_a4.py`](ci/side_a4.py) | the v1 CAD battery (these files are part of the v1 certification identity; do not move them) | `d1-gates`, `version-check` |
| [`ci/tests/`](ci/tests/) | unit tests for the scripts above and for committed evidence | `version-check` workflow |
| [`ci/runners/`](ci/runners/), [`ci/RUNNERS.md`](ci/RUNNERS.md) | self-hosted runner images, registration and routing rules | by hand |
| [`ci/run_bounded.sh`](ci/run_bounded.sh), [`ci/bounded_process.py`](ci/bounded_process.py), [`ci/select_rust_toolchain.py`](ci/select_rust_toolchain.py), [`ci/ensure_cargo_cache.py`](ci/ensure_cargo_cache.py) | resource-bounded CI steps and toolchain selection | `d1-gates`, `version-check`, `regression-datasets`, `synthetic-geometry-benchmark` |
| [`render/paper/`](render/paper/), [`render/patent/`](render/patent/) | white-paper PDFs and patent figures | `version-check` workflow |
| [`render/blender/`](render/blender/) | Blender scenes for the paper figures | by hand |
| [`render/marketing/`](render/marketing/), [`render/pro/`](render/pro/) | the v1/v2 marketing and hero render pipelines | `render-marketing-smoke` workflow |
| [`render/brand/`](render/brand/) | brand artwork and the signature mark | by hand |
| [`render/whitepaper_hero_figs.py`](render/whitepaper_hero_figs.py), [`render/engineering_gates_paper_figs.py`](render/engineering_gates_paper_figs.py), [`render/components.py`](render/components.py), [`render/turntable.py`](render/turntable.py) | figure generators for the white paper and the gate reference | by hand |
| [`science_tests/native_cad/`](science_tests/native_cad/) | native CAD validation used by the CAD battery | by hand |
| [`wave_figures/`](wave_figures/), [`wave_reports/`](wave_reports/) | v1 per-wave FEM plates and reports | by hand |
| [`multihost/`](multihost/), [`multihost_launch.sh`](multihost_launch.sh), [`yield_guard.sh`](yield_guard.sh) | the v1 multi-host wave launcher and its load guard | by hand |
| [`market/`](market/), [`bench/`](bench/), [`calibration/`](calibration/), [`d1/`](d1/) | the market census, the fixed benchmark, T1/T2 calibration and the D1 safety-factor map | by hand |
