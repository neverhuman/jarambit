#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/jarambit_wt_diag
taskset -pc 10,11 $$ >/dev/null
export CARGO_BUILD_JOBS=2
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"
export CARGO_PROFILE_DEV_DEBUG="${CARGO_PROFILE_DEV_DEBUG:-line-tables-only}"
study_dir="$PWD/target/gate-feasibility-bound/unified_study"
export TMPDIR="$study_dir/compiler-tmp"
mkdir -p "$TMPDIR"
flags=()
for crate_name in jarambit_genotype jarambit_phenotype jarambit_physics jarambit_redteam jarambit_evidence serde serde_json spade; do
 matches=(target/release/deps/lib"$crate_name"-*.rlib)
 test "${#matches[@]}" -eq 1
 flags+=(--extern "$crate_name=${matches[0]}")
done
rustc --edition=2021 -O -C lto=thin "$study_dir/main.rs" -L dependency=target/release/deps "${flags[@]}" -o "$study_dir/study"
