"""Probe spine-start/body combinations, then retain measured export and fine FEM."""
from concurrent.futures import ThreadPoolExecutor
import json
import math
from search import OUT, run, score


def main():
    rows = json.loads((OUT / 'search_summary.json').read_text())
    source = next(r for r in rows if r['name'] == 'section_024')
    laws = [
        {'ANGLE': 18, 'D0': .48, 'D1': .58, 'POINT_LATERAL': 1, 'GRID': .05},
        {'ANGLE': 18, 'D0': .5, 'D1': .6, 'POINT_LATERAL': 2, 'GRID': .05},
        {'ANGLE': 18, 'D0': .45, 'D1': .55, 'POINT_LATERAL': 2, 'GRID': .025},
        {'ANGLE': 22, 'D0': .48, 'D1': .58, 'POINT_LATERAL': 2, 'GRID': .025},
    ]
    cases = []
    for law, overrides in enumerate(laws):
        for body in [0, 2]:
            for off in [30, 34, 38, 40, 42, 44, 45]:
                genes = source['genes'].copy()
                genes[64] = off
                for k in range(28, 36):
                    cap = 2 if k in [28, 29] else 3.5
                    genes[k] = min(cap, genes[k] + body * math.exp(-((k-30)/1.5)**2))
                cases.append((f'follow_{law}_{body}_{off}', genes, {**source['config'], **overrides}))
    def evaluate(case):
        row = run(*case)
        print(json.dumps({k: v for k, v in row.items() if k != 'genes'}), flush=True)
        return row
    with ThreadPoolExecutor(max_workers=2) as pool:
        follow = list(pool.map(evaluate, cases))
    (OUT / 'follow_summary.json').write_text(json.dumps(follow, indent=2) + '\n')
    leaders = []
    for law in range(len(laws)):
        candidates = [r for r in follow if r['name'].startswith(f'follow_{law}_') and r['legal_geometry']]
        if candidates:
            leaders += [max(candidates, key=score), max(candidates, key=lambda r:r['metrics']['hm_spine_sf'])]
    leaders = list({r['name']: r for r in leaders}.values())
    def measure(item):
        i, r = item
        export = run(f'follow_export_{i}', r['genes'], r['config'], 'export')
        export['coarse_metrics'] = r['metrics']
        export['coarse_name'] = r['name']
        print(json.dumps({k: v for k, v in export.items() if k not in ['genes', 'coarse_metrics']}), flush=True)
        return export
    with ThreadPoolExecutor(max_workers=2) as pool:
        exports = list(pool.map(measure, enumerate(leaders)))
    (OUT / 'follow_export_summary.json').write_text(json.dumps(exports, indent=2) + '\n')
    eligible = [r for r in exports if r['legal_geometry'] and r['sharpness'] and r['point']]
    original_exports = json.loads((OUT / 'export_summary.json').read_text())
    for r in original_exports:
        if r['legal_geometry'] and r['sharpness'] and r['point']:
            matches = [candidate for candidate in rows if candidate['genes'] == r['genes'] and candidate['config'] == r['config']]
            assert matches
            r['coarse_metrics'] = matches[0]['metrics']
            eligible.append(r)
    finalists = []
    if eligible:
        finalists = [max(eligible, key=lambda r: min(r['coarse_metrics'][k] for k in ['hm_twist_sf', 'hm_spine_sf']))]
        finalists += [max(eligible, key=lambda r:r['coarse_metrics']['hm_spine_sf'])]
    finalists = list({r['name']: r for r in finalists}.values())
    # Original baseline uses the same executable and all four mesh areas as each finalist.
    baseline = next(r for r in rows if r['name'] == 'baseline')
    finalists.insert(0, baseline)
    def validate(item):
        i, r = item
        result = run(f'fine_{i}', r['genes'], r['config'], 'fem')
        result['selected_from'] = r['name']
        return result
    with ThreadPoolExecutor(max_workers=2) as pool:
        fine = list(pool.map(validate, enumerate(finalists)))
    (OUT / 'fine_summary.json').write_text(json.dumps(fine, indent=2) + '\n')


if __name__ == '__main__':
    main()
