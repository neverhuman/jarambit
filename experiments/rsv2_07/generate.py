"""Build a diagnostic-only compound-section harness from this checkout's source."""
from pathlib import Path
import hashlib
import json
import subprocess

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
OUT = ROOT / 'target/rsv2-07/harness'
OUT.mkdir(parents=True, exist_ok=True)
SRC = ROOT / 'crates/crescent/src'

source = (SRC / 'export.rs').read_text()
start = source.index('pub fn thickness_at(')
end = source.index('\npub fn solid(', start)
original = source[start:end].replace('pub fn thickness_at(', 'pub fn baseline_thickness_at(', 1)
source = source[:start] + original + '\npub fn thickness_at(p: &Pheno, q: P2) -> f64 { crate::compound::thickness_at(p,q) }\n' + source[end:]
source = source.replace('let outer=dp_closed(&p.outline,0.03);', 'let outer=dp_closed(&p.outline,crate::compound::config().boundary_tol);')
source = source.replace('max_area.min(0.25)', 'max_area.min(crate::compound::config().export_area)')
source = source.replace('let m=fine::mesh_domain(&outer,&holes,', 'let m=crate::compound::export_mesh(p,&outer,&holes,')
(OUT / 'prototype_export.rs').write_text(source)

source = (SRC / 'fem.rs').read_text()
marker = '    let hm_rb = nod_rb.as_ref().map(|v| cap(pk6(v, true))).unwrap_or(0.0);'
assert source.count(marker) == 1
source = source.replace(marker, marker + '\n    crate::diagnostic::record(&t6, nod_sp.as_deref(), nod_rb.as_deref(), tp, sx0);')
(OUT / 'prototype_fem.rs').write_text(source)
modules = ['genome', 'pheno', 'fem', 'gates', 'aesthetic', 'certification', 'geom', 'c4_ref', 'c5_ref', 'c6_ref', 'step_export']
paths = {name: OUT / 'prototype_fem.rs' if name == 'fem' else SRC / f'{name}.rs' for name in modules}
(OUT / 'modules.rs').write_text('\n'.join(f'#[path="{path}"] pub mod {name};' for name, path in paths.items()) + '\n')
for name in ['main.rs', 'compound.rs', 'diagnostic.rs']:
    (OUT / name).write_bytes((HERE / name).read_bytes())
tracked = [SRC / f'{name}.rs' for name in modules + ['export']]
tracked += sorted((ROOT / 'requirements').glob('RS-karambit-*.toml'))
tracked += sorted(HERE.glob('*.rs'))
provenance = {
    'commit': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
    'source_sha256': {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in tracked},
    'prototype_only': True,
}
(OUT.parent / 'source_provenance.json').write_text(json.dumps(provenance, indent=2) + '\n')
