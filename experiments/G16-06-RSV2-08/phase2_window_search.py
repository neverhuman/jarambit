#!/usr/bin/env python3
"""Deterministic native window-gene trials; retain every failure and sealed input."""
import csv
import itertools
import json
import os
from pathlib import Path
import subprocess
from run_phase2 import HERE, ROOT, OUT, environment, run

TRIALS = OUT / 'window_trials'
LIMITS = {'lat_rob': 358.0, 'fat_sf': 1.0, 'sf_hunter': 1.377, 'seq_ratio': 1.25}

def monotone(rows):
    vals = [r['metrics']['seq_ratio'] for r in rows]
    differences = [b-a for a, b in zip(vals, vals[1:])]
    return all(d > 0 for d in differences) or all(d < 0 for d in differences)

def thresholds(rows):
    return all(r['metrics'] and all(r['metrics'][k] >= v for k, v in LIMITS.items()) for r in rows)

def evaluate(csv_path, name, areas, mode='fem'):
    path = TRIALS / f'{name}.json'
    env = environment()
    env['STUDY_AREAS'] = areas
    with path.with_suffix('.log').open('w') as log:
        subprocess.run(['taskset', '-c', '12,13', str(OUT.parent / 'study'), mode,
                        str(path), str(csv_path)], cwd=ROOT, env=env, stdout=log, stderr=log, check=True)
    return json.loads(path.read_text())

def legal(d):
    return (d['input_was_canonical'] and d['within_genome_domain'] and d['rs_v2']
            and d['aes_pass'] and not d['rejects'] and 'window' in d['holes'])

def report(row):
    with (OUT / 'window_trials.jsonl').open('a') as f:
        f.write(json.dumps(row) + '\n')
    print(json.dumps(row), flush=True)

if __name__ == '__main__':
    assert os.sched_getaffinity(0) == {12, 13}
    TRIALS.mkdir(parents=True, exist_ok=True)
    base = list(csv.reader((HERE / 'candidate.csv').open()))
    survivors = []
    # Only native placement/fill genes change; native width/end-rounding law is retained.
    for i, (s0, s1, fill) in enumerate(itertools.product((.05, .075, .1, .125), (.5, .525, .55), (.6, .65, .7))):
        name = f'placement_{i:02d}'
        rows = [list(r) for r in base]
        for k, v in {100:s0, 101:s1, 103:fill}.items(): rows[1][k] = str(v)
        csv_path = TRIALS / f'{name}.csv'
        with csv_path.open('w') as f: csv.writer(f, lineterminator='\n').writerows(rows)
        d = evaluate(csv_path, name + '-coarse', '4,1')
        ok = legal(d) and thresholds(d['levels'])
        report(dict(name=name, stage='coarse', genes=dict(s0=s0, s1=s1, fill=fill),
                    legal=legal(d), targets_pass=ok, levels=d['levels']))
        if ok: survivors.append((name, csv_path, d['levels']))
    admitted = []
    for name, csv_path, coarse in survivors:
        d = run('gci', csv_path, name)
        fit = d['plate_gci']['seq_ratio_gci']
        report(dict(name=name, stage='gci', admitted=fit['admitted'], fit_status=fit['fit_status'],
                    levels=d['plate_gci']['levels_fine_to_coarse']))
        if d['G16-15']: admitted.append((name, csv_path, coarse))
    successes = []
    for name, csv_path, coarse in admitted:
        levels = list(coarse)
        # Reject each failed level before spending the finest-mesh solve.
        for area in ('.25', '.125', '.0625'):
            d = evaluate(csv_path, name + '-area' + area, area)
            levels += d['levels']
            ok = legal(d) and thresholds(levels) and monotone(levels)
            report(dict(name=name, stage='refine', area=area, targets_and_monotonic=ok, levels=levels))
            if not ok: break
        else:
            d = evaluate(csv_path, name + '-export', '', mode='export')
            export_ok = legal(d) and d['measurement']['sharpness']['pass'] and d['measurement']['point']['pass'] and d['solid']['census'] == [0, 0, 1]
            cert = min(r['metrics']['seq_ratio'] for r in levels if r['area'] in (.25, .125))
            a4 = run('a4', csv_path, name, cert)
            report(dict(name=name, stage='final', export_pass=export_ok, G16_13=a4['G16-13'],
                        a4=a4['a4'], levels=levels, all_gate_qualification=False))
            if export_ok and a4['G16-13']: successes.append(name)
    (OUT / 'window_search_summary.json').write_text(json.dumps(dict(
        trials=36, coarse_survivors=[n for n,_,_ in survivors], gci_admitted=[n for n,_,_ in admitted],
        successes=successes, thresholds=LIMITS, all_gate_qualification=False), indent=2) + '\n')
