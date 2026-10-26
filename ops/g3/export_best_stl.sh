#!/usr/bin/env bash
# Automatically convert the G3 best candidate genome into 3D-printable Binary STL (Units: mm)
# files without modifying any hashed decoder source file.
#
# Usage:
#   ./ops/g3/export_best_stl.sh [winner_genome.csv] [out_dir] [h_mm] [max_mb]
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VERSION="$(tr -d '[:space:]' < "${REPO_ROOT}/docs/VERSION" 2>/dev/null || echo "3.0.0")"
CSV_PATH="${1:-${REPO_ROOT}/evidence/g3/v${VERSION}/winner_genome.csv}"
OUT_DIR="${2:-${REPO_ROOT}/docs/g3/v${VERSION}/cad}"
H_MM="${3:-0.25}"
MAX_MB="${4:-50.0}"

echo "[export_best_stl] Exporting watertight Binary STL (Units: mm) from ${CSV_PATH} -> ${OUT_DIR} (h=${H_MM} mm, max=${MAX_MB} MiB)..."
cargo run --release --manifest-path "${REPO_ROOT}/crates/g3/Cargo.toml" \
  --example export_print_stl -- "${CSV_PATH}" "${OUT_DIR}" --h "${H_MM}" --max-mb "${MAX_MB}"
