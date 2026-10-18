#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
export CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=line-tables-only
export TMPDIR="$PWD/target/g16-06-rsv2-08/phase9/compiler-tmp"
flags=()
for crate_name in jarambit_crescent jarambit_physics jarambit_evidence serde_json; do
  matches=(target/release/deps/lib"$crate_name"-*.rlib)
  test "${#matches[@]}" -eq 1
  flags+=(--extern "$crate_name=${matches[0]}")
done
test ! -e target/g16-06-rsv2-08/phase9/measure
rustc --edition=2021 -O -C lto=thin experiments/g16-06-rsv2-08/phase9.rs -L dependency=target/release/deps "${flags[@]}" -o target/g16-06-rsv2-08/phase9/measure
