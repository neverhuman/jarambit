#!/usr/bin/env bash
# Automatically convert the G5 best candidate genome into 3D-printable Binary STL (Units: mm)
# files without modifying any hashed decoder source file.
#
# Usage:
#   ./ops/g5/export_best_stl.sh [winner_genome.csv] [out_dir] [h_mm] [max_mb]
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# The cohort of record (evidence/g5/RECORD) owns the winner's genome row and its CAD folder; fail closed without it.
RECORD="$(tr -d '[:space:]' < "${REPO_ROOT}/evidence/g5/RECORD")"
CSV_PATH="${1:-${REPO_ROOT}/evidence/g5/v${RECORD}/winner_genome.csv}"
OUT_DIR="${2:-${REPO_ROOT}/docs/g5/v${RECORD}/cad}"
H_MM="${3:-0.25}"
MAX_MB="${4:-50.0}"

DESIGN_ID="$(jq -r .design_id "${REPO_ROOT}/evidence/g5/v${RECORD}/release.json")"
if [[ -f "${OUT_DIR}/${DESIGN_ID}_AMbuild_mm.stl" && -f "${OUT_DIR}/${DESIGN_ID}_envelope_mm.stl" ]]; then
  echo "[export_best_stl] Preserving existing certified Binary STL files in ${OUT_DIR} for ${DESIGN_ID}..."
else
  echo "[export_best_stl] Exporting watertight Binary STL (Units: mm) from ${CSV_PATH} -> ${OUT_DIR} (h=${H_MM} mm, max=${MAX_MB} MiB)..."
  cargo run --release --manifest-path "${REPO_ROOT}/crates/g5/Cargo.toml" \
    --bin g5-export-stl -- "${CSV_PATH}" "${OUT_DIR}" --h "${H_MM}" --max-mb "${MAX_MB}"
fi

# Keep top-level cad/ synced with the cohort-of-record winner so https://github.com/neverhuman/jarambit/tree/main/cad
# always holds the current winner's 6-file 3D-printing & manufacturing upload kit (STL, 3MF, DXF, PDF, JPG) + README.md.
if [[ "${OUT_DIR}" == "${REPO_ROOT}/docs/g5/v${RECORD}/cad" ]]; then
  RAW_VERSION="$(tr -d '[:space:]' < "${REPO_ROOT}/docs/VERSION")"
  if [[ "${RAW_VERSION}" == 5.* ]]; then
    VERSION="${RAW_VERSION}"
  else
    VERSION="${RECORD}"
  fi
  TOP_CAD="${REPO_ROOT}/cad"
  mkdir -p "${TOP_CAD}"
  rm -f "${TOP_CAD}"/*.stl "${TOP_CAD}"/*.3mf "${TOP_CAD}"/*.dxf "${TOP_CAD}"/*.pdf "${TOP_CAD}"/*.jpg "${TOP_CAD}"/*_print_spec.json
  cargo run --release --manifest-path "${REPO_ROOT}/crates/g5/Cargo.toml" \
    --bin g5-export-stl -- --companion-from-dir "${CSV_PATH}" "${OUT_DIR}" "${TOP_CAD}" "${H_MM}"
  cp -f "${OUT_DIR}"/*.stl "${TOP_CAD}/"
  cp -f "${REPO_ROOT}/docs/g5/paper/g5-v${VERSION}.pdf" "${TOP_CAD}/${DESIGN_ID}_spec_paper.pdf"
  cp -f "${REPO_ROOT}/docs/g5/v${RECORD}/img/cut_full.jpg" "${TOP_CAD}/${DESIGN_ID}_xray_cutaway.jpg"
  python3 "${REPO_ROOT}/ops/ci/gen_readme.py"
  echo "[export_best_stl] Synced winner 6-file upload kit (STL, 3MF, DXF, PDF, JPG) and cad/README.md into ${TOP_CAD}/"
fi
