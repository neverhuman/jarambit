#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
export CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=line-tables-only
mkdir -p target/G16-06-RSV2-08/compiler-tmp
export TMPDIR="$PWD/target/G16-06-RSV2-08/compiler-tmp"
taskset -c 12,13 cargo build --release --locked -p jarambit-crescent
python3 experiments/G16-06-RSV2-08/generate.py
flags=()
for crate_name in jarambit_genotype jarambit_phenotype jarambit_physics jarambit_redteam jarambit_evidence serde serde_json; do
  matches=(target/release/deps/lib"$crate_name"-*.rlib)
  test "${#matches[@]}" -eq 1
  flags+=(--extern "$crate_name=${matches[0]}")
done
taskset -c 12,13 rustc --edition=2021 -O -C lto=thin target/G16-06-RSV2-08/generated/main.rs -L dependency=target/release/deps "${flags[@]}" -o target/G16-06-RSV2-08/study
