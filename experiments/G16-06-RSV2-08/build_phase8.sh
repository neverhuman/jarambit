#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
export CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=line-tables-only
study_dir="$PWD/target/G16-06-RSV2-08/phase8"
mkdir -p "$study_dir/compiler-tmp"
export TMPDIR="$study_dir/compiler-tmp"
python3 experiments/G16-06-RSV2-08/prepare_phase8.py
flags=()
for crate_name in jarambit_genotype jarambit_phenotype jarambit_physics jarambit_redteam jarambit_evidence serde serde_json spade; do
  matches=(target/release/deps/lib"$crate_name"-*.rlib)
  test "${#matches[@]}" -eq 1
  flags+=(--extern "$crate_name=${matches[0]}")
done
for adapter in main native_adapter; do
  test ! -e "$study_dir/$adapter"
  taskset -c 12,13 rustc --edition=2021 -O -C lto=thin "$study_dir/$adapter.rs" -L dependency=target/release/deps "${flags[@]}" -o "$study_dir/$adapter"
done
