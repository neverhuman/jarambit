#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
export CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=line-tables-only
export TMPDIR="$PWD/target/g16-06-rsv2-08/phase13/compiler-tmp"
mkdir -p "$TMPDIR"
test ! -e target/g16-06-rsv2-08/phase13/measure
python3 - <<'PY'
import hashlib,json
from pathlib import Path
p=Path('target/g16-06-rsv2-08/phase13/source_provenance.json');assert not p.exists()
refs=json.loads(Path('experiments/g16-06-rsv2-08/phase11_receipt.json').read_text())['source_provenance']
for rel,want in refs.items():assert hashlib.sha256(Path(rel).read_bytes()).hexdigest()==want,rel
rel='experiments/g16-06-rsv2-08/phase13.rs';refs[rel]=hashlib.sha256(Path(rel).read_bytes()).hexdigest()
p.write_text(json.dumps(refs,indent=2,sort_keys=True)+'\n')
PY
flags=()
for crate_name in jarambit_crescent jarambit_physics jarambit_evidence serde_json; do
  matches=(target/release/deps/lib"$crate_name"-*.rlib)
  test "${#matches[@]}" -eq 1
  flags+=(--extern "$crate_name=${matches[0]}")
done
rustc --edition=2021 -O -C lto=thin experiments/g16-06-rsv2-08/phase13.rs -L dependency=target/release/deps "${flags[@]}" -o target/g16-06-rsv2-08/phase13/measure
