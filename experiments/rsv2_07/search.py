"""Deterministic bounded probes; coarse observations never certify a gate."""
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
import csv
import hashlib
import json
import math
import os
import random
import subprocess

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
BINARY = ROOT / 'target/rsv2-07/study'
OUT = ROOT / 'target/rsv2-07/v3'
os.sched_setaffinity(0, {14, 15})
ENV = {**os.environ, 'CARGO_BUILD_JOBS': '2', 'RAYON_NUM_THREADS': '1', 'JARAMBIT_SOLVER': 'faer'}
# No inherited study knobs may silently change a candidate's construction.
ENV = {k: v for k, v in ENV.items() if not k.startswith('STUDY_')}
BASE_CONFIG = {'ANGLE': 14, 'APEX': .08, 'D0': .65, 'D1': 1.1,
               'POINT_SHAPE': 2, 'RADIAL': 1, 'GRID': .1, 'BOUNDARY_TOL': .001}
ROWS = list(csv.reader((HERE / 'baseline.csv').open()))
BASE = [float(v) for v in ROWS[1][:-1]]
assert len(BASE) == 128


def write_seed(path, genes):
    with path.open('w') as stream:
        writer = csv.writer(stream, lineterminator='\n')
        writer.writerow(ROWS[0])
        writer.writerow([*genes, ROWS[1][-1]])


def run(name, genes, config, mode='coarse', area=None):
    folder = OUT / name
    folder.mkdir(parents=True, exist_ok=False)
    seed = folder / 'seed.csv'
    write_seed(seed, genes)
    config = {**BASE_CONFIG, **config}
    env = {**ENV, **{f'STUDY_{k}': str(v) for k, v in config.items()}}
    if area is not None:
        env['STUDY_AREA'] = str(area)
    command = [str(BINARY), mode, str(folder / 'result.json'), str(seed)]
    (folder / 'invocation.json').write_text(json.dumps({'command': command, 'config': config, 'mode': mode, 'area': area, 'cpus': sorted(os.sched_getaffinity(0)), 'CARGO_BUILD_JOBS': 2, 'executable_sha256': hashlib.sha256(BINARY.read_bytes()).hexdigest()}, indent=2) + '\n')
    with (folder / 'run.log').open('w') as log:
        subprocess.run(command, cwd=ROOT, env=env, stdout=log, stderr=log, check=True)
    doc = json.loads((folder / 'result.json').read_text())
    assert 'point_lateral' in doc['config'], 'stale harness binary'
    assert hashlib.sha256(BINARY.read_bytes()).hexdigest() == json.loads((folder / 'invocation.json').read_text())['executable_sha256'], 'binary changed during evaluation'
    legal = doc['within_genome_domain'] and doc['rs_v2'] and doc['aes_pass'] and not doc['rejects']
    metrics = doc.get('levels', [{}])[-1].get('metrics')
    row = {'name': name, 'legal_geometry': legal, 'config': config,
           'metrics': metrics, 'genes': doc['genes'], 'rejects': doc['rejects'], 'aes_fails': doc['aes_fails']}
    if 'measurement' in doc:
        row['sharpness'] = (doc['measurement']['sharpness'] or {}).get('pass', False)
        row['point'] = (doc['measurement']['point'] or {}).get('pass', False)
    return row


def score(row):
    f = row['metrics']
    return min(f['hm_twist_sf'], f['hm_spine_sf'], f['hm_ringback_sf']) if row['legal_geometry'] and f else -1


def main():
    # Stage 1: section law at the legal 6 mm stock endpoint; serrations remain present.
    strong = BASE.copy()
    strong[45] = 6
    strong[123:128] = [6] * 5
    strong[50:54] = [30, 58, 7, 3.5]
    cases = [('baseline', BASE, {})]
    for d0, d1 in [(.65, 1.1), (.5, .8), (.45, .6), (.42, .52)]:
        for angle, apex in [(14, .08), (18, .175), (22, .175)]:
            for shape in [2, 3]:
                config = {'ANGLE': angle, 'APEX': apex, 'D0': d0, 'D1': d1,
                          'POINT_SHAPE': shape, 'SMOOTH_HEEL': 1, 'GRID': .05}
                cases.append((f'section_{len(cases):03}', strong, config))
    rows = []
    def evaluate(case):
        row = run(*case)
        print(json.dumps({k: v for k, v in row.items() if k != 'genes'}), flush=True)
        return row
    with ThreadPoolExecutor(max_workers=2) as pool:
        rows += list(pool.map(evaluate, cases))
    (OUT / 'section_summary.json').write_text(json.dumps(rows, indent=2) + '\n')
    leader = max(rows, key=score)
    # Smooth planform bumps and spine-sharpening start probes stay in the native domain.
    variants = []
    for center in [1, 3, 6, 8, 30, 33, 36, 38]:
        for amplitude in [-2, 2]:
            genes = leader['genes'].copy()
            for k in range(40):
                cap = 0 if k in [0, 39] or 19 <= k <= 22 else (2 if k in [8, 28, 29] else 2.5 if 36 <= k <= 38 else 3.5 if 9 <= k <= 13 or 30 <= k <= 35 else 3)
                genes[k] = max(-cap, min(cap, genes[k] + amplitude * math.exp(-((k-center)/1.5)**2)))
            variants.append((f'planform_{len(variants):03}', genes, leader['config']))
    for off in [0, 10, 20, 30, 40, 45]:
        genes = leader['genes'].copy()
        genes[64] = off
        variants.append((f'planform_{len(variants):03}', genes, leader['config']))
    for depths in [(1, .4, .5), (1.5, .8, .9)]:
        genes = leader['genes'].copy()
        genes[61:64] = depths
        variants.append((f'planform_{len(variants):03}', genes, leader['config']))
    with ThreadPoolExecutor(max_workers=2) as pool:
        rows += list(pool.map(evaluate, variants))
    (OUT / 'search_summary.json').write_text(json.dumps(rows, indent=2) + '\n')
    by_name = {r['name']: r for r in rows}
    leaders = [max(rows, key=score),
               max((r for r in rows if r['legal_geometry']), key=lambda r:r['metrics']['hm_spine_sf']),
               by_name['section_016'], by_name['section_010'], by_name['baseline']]
    def export_candidate(item):
        i, candidate = item
        return run(f'export_{i}', candidate['genes'], candidate['config'], 'export')
    with ThreadPoolExecutor(max_workers=2) as pool:
        exports = list(pool.map(export_candidate, enumerate(leaders)))
    (OUT / 'export_summary.json').write_text(json.dumps(exports, indent=2) + '\n')


if __name__ == '__main__':
    main()
