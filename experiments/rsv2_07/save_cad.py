"""Retain the validated construction's full-precision facets and viewing STL."""
from pathlib import Path
import hashlib
import json
import math
import os
import shutil
import subprocess
from search import ROOT, HERE, OUT, ENV


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    provenance = json.loads((OUT.parent / 'source_provenance.json').read_text())
    assert all(digest(ROOT/path) == sha for path, sha in provenance['source_sha256'].items())
    validation = json.loads((HERE / 'results/validation.json').read_text())['validation']
    selected = [max(validation, key=lambda r:min(r['strict_0p0625']['hm_twist_sf'], r['cert_min_0p25_0p125']['hm_twist_sf'])),
                max(validation, key=lambda r:min(r['strict_0p0625']['hm_spine_sf'], r['cert_min_0p25_0p125']['hm_spine_sf']))]
    selected = list({r['name']: r for r in selected}.values())
    harness = OUT.parent / 'harness'
    shutil.copy2(HERE / 'cad.rs', harness / 'cad.rs')
    flags = []
    for name in ['jarambit_genotype', 'jarambit_phenotype', 'jarambit_physics', 'jarambit_redteam',
                 'jarambit_evidence', 'serde', 'serde_json', 'spade']:
        paths = list((ROOT / 'target/release/deps').glob(f'lib{name}-*.rlib'))
        assert len(paths) == 1
        flags += ['--extern', f'{name}={paths[0]}']
    binary = OUT.parent / 'save-cad'
    env = {**ENV, 'TMPDIR': str(OUT.parent / 'compiler-tmp')}
    subprocess.run(['rustc', '--edition=2021', '-O', '-C', 'lto=thin', str(harness / 'cad.rs'),
                    '-L', f'dependency={ROOT}/target/release/deps', *flags, '-o', str(binary)],
                   env=env, cwd=ROOT, check=True)
    manifest = []
    for r in selected:
        folder = OUT.parent / 'cad' / r['name']
        assert not folder.exists()
        config = {f'STUDY_{k.upper()}': str(int(v) if isinstance(v, bool) else v)
                  for k, v in r['config'].items()}
        subprocess.run([str(binary), str(HERE / 'results' / f'{r["name"]}.csv'), str(folder)],
                       env={**env, **config}, cwd=ROOT, check=True)
        metadata = json.loads((folder / 'metadata.json').read_text())
        assert metadata['config'] == r['config']
        for key in ['vertices', 'triangles', 'census']:
            assert metadata[key] == r['solid'][key]
        assert math.isclose(metadata['volume_mm3'], r['solid']['volume_mm3'], rel_tol=1e-12)
        manifest.append({'candidate': r['name'], 'config': r['config'], 'metadata': metadata,
                         'binary_sha256': digest(binary), 'source_sha256': digest(HERE / 'cad.rs'),
                         'cpus': sorted(os.sched_getaffinity(0)), 'CARGO_BUILD_JOBS': 2,
                         'files': {str(p.relative_to(ROOT)): digest(p) for p in folder.iterdir()}})
    (HERE / 'results/cad_manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
    print(json.dumps(manifest, indent=2))


if __name__ == '__main__':
    main()
