#!/bin/sh
# Reconstruct reporting derivatives from retained evidence; never runs a solver.
set -eu
paper_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_dir=$(CDPATH= cd -- "$paper_dir/../.." && pwd)
cd "$repo_dir"
: "${CARGO_INCREMENTAL:=0}"
: "${CARGO_PROFILE_DEV_DEBUG:=line-tables-only}"
: "${CARGO_BUILD_JOBS:=2}"
# Optimized reporting retains arithmetic and assertion checks.
export CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=true
export CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=true
export CARGO_INCREMENTAL CARGO_PROFILE_DEV_DEBUG CARGO_BUILD_JOBS
export CARGO_TARGET_DIR="$repo_dir/target"
case "${1:-build}" in
  build) exec make -C "$paper_dir" all ;;
  documents) exec make -C "$paper_dir" pdfs ;;
  check) exec make -C "$paper_dir" check ;;
  *) printf '%s\n' 'usage: docs/paper/reproduce.sh [build|documents|check]' >&2; exit 2 ;;
esac
