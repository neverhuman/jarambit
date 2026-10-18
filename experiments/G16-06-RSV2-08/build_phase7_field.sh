#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
export CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=line-tables-only
export TMPDIR="$PWD/target/G16-06-RSV2-08/phase7/compiler-tmp"
flags=()
for crate_name in jarambit_genotype jarambit_phenotype jarambit_physics jarambit_redteam jarambit_evidence serde serde_json spade; do
  matches=(target/release/deps/lib"$crate_name"-*.rlib)
  test "${#matches[@]}" -eq 1
  flags+=(--extern "$crate_name=${matches[0]}")
done
test ! -e target/G16-06-RSV2-08/phase7/field_probe
rustc --edition=2021 -O -C lto=thin experiments/G16-06-RSV2-08/phase7_field.rs -L dependency=target/release/deps "${flags[@]}" -o target/G16-06-RSV2-08/phase7/field_probe
