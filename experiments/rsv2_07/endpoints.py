"""Probe the remaining native blade-root envelope before accepting a spine deficit."""
from concurrent.futures import ThreadPoolExecutor
import json
import math
from search import OUT, run, score


def main():
    initial = json.loads((OUT / 'search_summary.json').read_text())
    source = next(r for r in initial if r['name'] == 'section_024')
    config = {**source['config'], 'ANGLE': 18, 'D0': .45, 'D1': .55,
              'POINT_LATERAL': 2, 'GRID': .025}
    cases = []
    for body in [3, 4.5, 6.5]:
        for off in [40, 42, 45]:
            genes = source['genes'].copy()
            genes[64] = off
            for k in range(28, 36):
                cap = 2 if k in [28, 29] else 3.5
                genes[k] = min(cap, genes[k] + body * math.exp(-((k-30)/1.5)**2))
            cases.append((f'endpoint_{body}_{off}', genes, config))
    def evaluate(case):
        row = run(*case)
        print(json.dumps({k: v for k, v in row.items() if k != 'genes'}), flush=True)
        return row
    with ThreadPoolExecutor(max_workers=2) as pool:
        rows = list(pool.map(evaluate, cases))
    (OUT / 'endpoint_summary.json').write_text(json.dumps(rows, indent=2) + '\n')
    legal = [r for r in rows if r['legal_geometry']]
    fine = []
    if legal:
        best = max(legal, key=lambda r:r['metrics']['hm_spine_sf'])
        export = run('endpoint_export_0', best['genes'], best['config'], 'export')
        export['coarse_name'] = best['name']
        (OUT / 'endpoint_export_summary.json').write_text(json.dumps([export], indent=2) + '\n')
        if export['sharpness'] and export['point']:
            row = run('endpoint_fine_0', best['genes'], best['config'], 'fem')
            row['selected_from'] = export['name']
            fine.append(row)
    (OUT / 'endpoint_fine_summary.json').write_text(json.dumps(fine, indent=2) + '\n')


if __name__ == '__main__':
    main()
