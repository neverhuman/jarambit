#!/usr/bin/env bash
# ==============================================================================
# evolve.sh — Turnkey Automated Evolution Across ALL Engineering Gates
# ==============================================================================
# Evolves genome-defined CAD and automatically assesses the top-k with the standard
# skfem P2 + DOLFINx pipeline on one saved fine mesh. Qualification is earned only
# when the unchanged 16 engineering gates and RS-karambit-v2 all_hard both pass.
# Failed and unverified attempts remain visible in leaderboard.json and raw files.
# --top-k-certify [N] (alias --certify-top N; default 2) controls costly assessments.
#
# Usage:
#   ./evolve.sh                                  # Default: 2 rounds, 12 candidates/elite/round -> out/auto_evolve
#   ./evolve.sh 4 24 out/my_wave                 # Positional: <rounds> <candidates_per_round> <out_dir>
#   ./evolve.sh --rounds 5 --out-dir out/wave_x  # Named flags forwarded to `jarambit auto-evolve`
#   JARAMBIT_DATASET_PUSH=origin:data/regression-records ./evolve.sh   # also push records each round
# ==============================================================================
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT_DIR"

export JARAMBIT_P1_SFMAP="${JARAMBIT_P1_SFMAP:-$ROOT_DIR/evidence/d1_p1/w249_sfmap.txt}"
export JARAMBIT_FEM="${JARAMBIT_FEM:-1}"
# Regression micro-records (datasets/regression/records/auto_evolve/<run>/): every birth's design
# features + Stage-1 screen outputs, every Stage-2 assessment. Committed after each round (record
# paths only). Set JARAMBIT_DATASET_PUSH=origin:<branch> to also push; JARAMBIT_DATASET=0 disables.
export JARAMBIT_DATASET_COMMIT="${JARAMBIT_DATASET_COMMIT:-1}"

echo "[evolve.sh] Building release binary (target/release/jarambit)..."
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"
export CARGO_PROFILE_DEV_DEBUG="${CARGO_PROFILE_DEV_DEBUG:-line-tables-only}"
cargo build --release --locked -p jarambit -p jarambit-crescent --bin jarambit \
    --example export_step --example plate_gci_probe --example g3_side_report --quiet

if [[ $# -gt 0 && "${1:-}" != -* ]]; then
    ROUNDS="${1:-2}"
    CANDS="${2:-12}"
    OUT_DIR="${3:-$ROOT_DIR/out/auto_evolve}"
    shift $(( $# >= 3 ? 3 : $# ))
    exec "$ROOT_DIR/target/release/jarambit" auto-evolve \
        --rounds "$ROUNDS" \
        --candidates-per-round "$CANDS" \
        --out-dir "$OUT_DIR" \
        "$@"
else
    exec "$ROOT_DIR/target/release/jarambit" auto-evolve "$@"
fi
