#!/usr/bin/env python3
"""Evaluate the sealed prototype without changing native gates or solver laws."""
import hashlib
import json
import os
from pathlib import Path
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
OUT = ROOT / 'target/G16-06-RSV2-08/phase2'

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def environment():
    env = {k: v for k, v in os.environ.items() if not k.startswith(('STUDY_', 'JARAMBIT_'))}
    env.update(json.loads((HERE / 'candidate_config.json').read_text()))
    env.update(CARGO_BUILD_JOBS='2', RAYON_NUM_THREADS='2', CARGO_INCREMENTAL='0',
               CARGO_PROFILE_DEV_DEBUG='line-tables-only')
    return env

def run(mode, genome, name, cert=None):
    path = OUT / f'{name}-{mode}.json'
    command = ['taskset', '-c', '12,13', str(OUT / 'study'), mode, str(path), str(genome)]
    if cert is not None:
        command.append(str(cert))
    with path.with_suffix('.log').open('w') as log:
        subprocess.run(command, cwd=ROOT, env=environment(), stdout=log, stderr=log, check=True)
    return json.loads(path.read_text())

if __name__ == '__main__':
    assert os.sched_getaffinity(0) == {12, 13}, 'run with taskset -c 12,13'
    OUT.mkdir(parents=True, exist_ok=True)
    receipt = json.loads((HERE / 'candidate_receipt.json').read_text())
    for name in ('candidate.csv', 'candidate_config.json'):
        relative = str((HERE / name).relative_to(ROOT))
        assert digest(ROOT / relative) == receipt['sha256'][relative], f'sealed input changed: {relative}'
    gci = run('gci', HERE / 'candidate.csv', 'sealed')
    for group in ('source_sha256', 'requirements_sha256', 'dependency_source_sha256', 'prototype_source_sha256'):
        for name, expected in gci['provenance'][group].items():
            path = Path(name)
            if not path.is_absolute(): path = ROOT / path
            assert digest(path) == expected, f'compiled source changed: {name}'
    fine = gci['plate_gci']['levels_fine_to_coarse'][0]
    assert fine['seq_ratio'] == receipt['certification_minimum']['seq_ratio']
    assert fine['sf_hunter'] == receipt['certification_minimum']['sf_hunter']
    print('sealed GCI:', json.dumps(gci['plate_gci']['seq_ratio_gci']), flush=True)
    a4 = run('a4', HERE / 'candidate.csv', 'sealed', receipt['certification_minimum']['seq_ratio'])
    print('sealed side.a4:', json.dumps(a4['a4']), flush=True)
    paths = [HERE / 'candidate.csv', HERE / 'candidate_config.json', HERE / 'candidate_receipt.json',
             HERE / 'phase2.rs', HERE / 'run_phase2.py', HERE / 'build_phase2.sh', OUT / 'study',
             OUT / 'sealed-gci.json', OUT / 'sealed-a4.json', OUT / 'sealed-a4.mesh.json',
             ROOT / 'crates/crescent/src/gci.rs', ROOT / 'crates/physics/src/side_a4.rs']
    summary = dict(task='G16-06-RSV2-08-p2', cpus=[12, 13], cargo_build_jobs=2,
                   sealed_G16_15=gci['G16-15'], sealed_G16_13=a4['G16-13'],
                   plate_gci=gci['plate_gci'], a4=a4['a4'], all_gate_qualification=False,
                   sha256={str(p.relative_to(ROOT)): digest(p) for p in paths})
    (OUT / 'baseline_receipt.json').write_text(json.dumps(summary, indent=2) + '\n')
