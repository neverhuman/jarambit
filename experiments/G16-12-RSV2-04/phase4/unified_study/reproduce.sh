#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/jarambit_wt_diag
taskset -pc 10,11 $$ >/dev/null
export CARGO_BUILD_JOBS=2
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"
export CARGO_PROFILE_DEV_DEBUG="${CARGO_PROFILE_DEV_DEBUG:-line-tables-only}"
study_dir="$PWD/target/gate-feasibility-bound/unified_study"
python3 "$study_dir/setup.py"
bash "$study_dir/build.sh" > "$study_dir/build.log" 2>&1
python3 "$study_dir/run.py" fem neck_control hotspot_control > "$study_dir/controls_repro.log" 2>&1
python3 "$study_dir/verify_controls.py"
python3 "$study_dir/run.py" fem combined_d065 combined_d055 combined_d048 > "$study_dir/combined_fem_runner.log" 2>&1
python3 "$study_dir/run.py" export combined_d065 combined_d055 combined_d048 > "$study_dir/combined_export_runner.log" 2>&1
python3 "$study_dir/run.py" export combined_d065_h025 > "$study_dir/refined_export_runner.log" 2>&1
python3 "$study_dir/analyze.py" > "$study_dir/analysis.log"
python3 "$study_dir/seal.py"
