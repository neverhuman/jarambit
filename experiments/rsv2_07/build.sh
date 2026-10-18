#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
export CARGO_BUILD_JOBS=2
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"
export CARGO_PROFILE_DEV_DEBUG="${CARGO_PROFILE_DEV_DEBUG:-line-tables-only}"
mkdir -p target/rsv2-07/compiler-tmp
export TMPDIR="$PWD/target/rsv2-07/compiler-tmp"
taskset -pc 14,15 $$ >/dev/null
cargo build --release --locked -p jarambit-crescent
python3 experiments/rsv2_07/generate.py
flags=()
for crate_name in jarambit_genotype jarambit_phenotype jarambit_physics jarambit_redteam jarambit_evidence serde serde_json spade; do
  matches=(target/release/deps/lib"$crate_name"-*.rlib)
  test "${#matches[@]}" -eq 1
  flags+=(--extern "$crate_name=${matches[0]}")
done
rustc --edition=2021 -O -C lto=thin target/rsv2-07/harness/main.rs \
  -L dependency=target/release/deps "${flags[@]}" -o target/rsv2-07/study
