#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
export CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=line-tables-only
study_dir="$PWD/target/G16-06-RSV2-08/phase6"
test ! -e "$study_dir/native_adapter" # Preserve any existing evaluated executable.
export TMPDIR="$study_dir/compiler-tmp"
mkdir -p "$TMPDIR"
flags=()
for crate_name in jarambit_genotype jarambit_phenotype jarambit_physics jarambit_redteam jarambit_evidence serde serde_json spade; do
  matches=(target/release/deps/lib"$crate_name"-*.rlib)
  test "${#matches[@]}" -eq 1
  flags+=(--extern "$crate_name=${matches[0]}")
done
# Invoke this script under nice -n 10 taskset -c 12,13.
taskset -c 12,13 rustc --edition=2021 -O -C lto=thin experiments/G16-06-RSV2-08/phase6_adapter.rs -L dependency=target/release/deps "${flags[@]}" -o "$study_dir/native_adapter"
