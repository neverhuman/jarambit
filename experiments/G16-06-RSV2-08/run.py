#!/usr/bin/env python3
"""Reproduce a section/coordinate study with unchanged native gate calculations."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import csv, json, os, subprocess, sys
ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
OUT = ROOT / 'target/G16-06-RSV2-08'
CONFIG = dict(STUDY_ANGLE='18', STUDY_APEX='.175', STUDY_D0='.65', STUDY_D1='1.1', STUDY_POINT_SHAPE='2', STUDY_RADIAL='1', STUDY_GRID='.1', STUDY_BOUNDARY_TOL='.001', STUDY_SMOOTH_HEEL='1')
def run(name, changes=None, config=None, mode='coarse'):
    rows = list(csv.reader((HERE / 'short_slot.csv').open()))
    for k, value in (changes or {}).items(): rows[1][k] = str(value)
    folder = OUT / 'runs'; folder.mkdir(exist_ok=True)
    seed = folder / (name + '.csv')
    with seed.open('w') as f: csv.writer(f, lineterminator='\n').writerows(rows)
    cfg = dict(CONFIG, **(config or {}))
    env = {k: v for k, v in os.environ.items() if not k.startswith(('STUDY_', 'JARAMBIT_'))}
    env.update(cfg, JARAMBIT_SOLVER='faer', RAYON_NUM_THREADS='2', CARGO_BUILD_JOBS='2')
    out = folder / (name + '.json')
    with (folder / (name + '.log')).open('w') as f:
        subprocess.run(['taskset', '-c', '12,13', str(OUT/'study'), mode, str(out), str(seed)], env=env, stdout=f, stderr=f, check=True)
    doc = json.loads(out.read_text())
    legal = doc['within_genome_domain'] and doc['rs_v2'] and doc['aes_pass'] and not doc['rejects']
    metrics = [level['metrics'] for level in doc.get('levels', [])]
    result = dict(name=name, legal_geometry=legal, config=cfg, levels=metrics, rejects=doc['rejects'], aes_fails=doc['aes_fails'], receipt=str(out.relative_to(ROOT)))
    print(json.dumps(result), flush=True)
    return result
if __name__ == '__main__':
    assert os.sched_getaffinity(0) == {12, 13}, os.sched_getaffinity(0)
    jobs = [('control', {}, {}), ('neck6_blade6', {}, {'STUDY_NECK_FULL':'1'}),
            ('neck6_blade5.8', {}, {'STUDY_NECK_FULL':'1','STUDY_BLADE_CORE':'5.8'}),
            ('neck6_blade5.6', {}, {'STUDY_NECK_FULL':'1','STUDY_BLADE_CORE':'5.6'}),
            ('neck6_blade5.4', {}, {'STUDY_NECK_FULL':'1','STUDY_BLADE_CORE':'5.4'})]
    with ThreadPoolExecutor(max_workers=2) as pool:
        results = list(pool.map(lambda j: run(*j), jobs))
    (OUT/'coarse_summary.json').write_text(json.dumps(results, indent=2)+'\n')
