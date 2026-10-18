#!/usr/bin/env python3
"""Check native GCI and all five mesh directions; use sealed cert/strict scope."""
import csv
import itertools
import json
import os
import sys
from phase2_window_search import TRIALS, LIMITS, evaluate, legal, monotone, thresholds, report
from run_phase2 import HERE, OUT, run

if __name__ == '__main__':
    assert os.sched_getaffinity(0) == {12, 13}
    records = [json.loads(s) for s in (OUT / 'window_trials.jsonl').read_text().splitlines()]
    local = sys.argv[1:] == ['--local']
    coarse = [] if local else [r for r in records if r['stage'] == 'coarse']
    # Native window width also controls its rounded-end radii. No new shape law.
    base = list(csv.reader((HERE / 'candidate.csv').open()))
    jobs = ([(6., s0, s1) for s0, s1 in itertools.product((.05, .052, .055, .06), (.5, .502, .505, .51))]
            if local else [(w, .05, s1) for w, s1 in itertools.product((5., 5.5, 6.5, 7.), (.5, .525))])
    for i, (width, s0, s1) in enumerate(jobs):
        name = f'local_{i:02d}' if local else f'fillet_{i:02d}'
        rows = [list(r) for r in base]
        for k, v in {100:s0, 101:s1, 102:width}.items(): rows[1][k] = str(v)
        csv_path = TRIALS / f'{name}.csv'
        with csv_path.open('w') as f: csv.writer(f, lineterminator='\n').writerows(rows)
        d = evaluate(csv_path, name + '-coarse', '4,1')
        row = dict(name=name, stage='coarse', genes=dict(width=width, s0=s0, s1=s1, fill=.65),
                   legal=legal(d), targets_pass=legal(d) and thresholds(d['levels']), levels=d['levels'])
        report(row); coarse.append(row)
    checked = []
    successes = []
    for row in coarse:
        if not row['legal']: continue
        q4, q1 = [r['metrics']['seq_ratio'] for r in row['levels']]
        # A positive-order fit requires the fine increment to shrink. Reject only
        # sequences that cannot possibly reach the unchanged fine sequence floor.
        possible = (q4 < q1 and q1 + (q1-q4) > 1.25) or (q4 > q1 > 1.25)
        if not possible: continue
        name = row['name']
        csv_path = TRIALS / f'{name}.csv'
        saved = OUT / f'{name}-gci.json'
        d = json.loads(saved.read_text()) if saved.exists() else run('gci', csv_path, name)
        fit = d['plate_gci']['seq_ratio_gci']
        fine = d['plate_gci']['levels_fine_to_coarse'][0]
        checked.append(name)
        report(dict(name=name, stage='audit_scope_gci', admitted=fit['admitted'],
                    fit_status=fit['fit_status'], p=fit['p'], levels=d['plate_gci']['levels_fine_to_coarse']))
        if not (d['G16-15'] and fine['seq_ratio'] >= 1.25 and fine['sf_hunter'] >= 1.377): continue
        levels = list(row['levels'])
        for area in ('.25', '.125', '.0625'):
            d = evaluate(csv_path, name + '-audit-area' + area, area)
            levels += d['levels']
            cert_strict = [r for r in levels if r['area'] <= .25]
            ok = legal(d) and thresholds(cert_strict) and monotone(levels)
            report(dict(name=name, stage='audit_scope_refine', area=area,
                        cert_strict_targets_and_monotonic=ok, levels=levels))
            if not ok: break
        else:
            d = evaluate(csv_path, name + '-audit-export', '', mode='export')
            export_ok = legal(d) and d['measurement']['sharpness']['pass'] and d['measurement']['point']['pass'] and d['solid']['census'] == [0, 0, 1]
            cert = min(r['metrics']['seq_ratio'] for r in levels if r['area'] in (.25, .125))
            a4 = run('a4', csv_path, name, cert)
            success = dict(name=name, stage='audit_scope_final', export_pass=export_ok,
                           G16_13=a4['G16-13'], a4=a4['a4'], levels=levels,
                           all_five_meshes_meet_targets=thresholds(levels),
                           all_gate_qualification=False,
                           threshold_scope='unchanged audit: certification min(.25,.125), strict .0625; 4 and 1 are GCI observations')
            report(success)
            if export_ok and a4['G16-13']:
                successes.append(success)
                break
    summary = 'local_search_summary.json' if local else 'convergence_search_summary.json'
    (OUT / summary).write_text(json.dumps(dict(
        coarse_trials=len(coarse), gci_checked=checked, successes=successes,
        thresholds=LIMITS, all_gate_qualification=False), indent=2) + '\n')
