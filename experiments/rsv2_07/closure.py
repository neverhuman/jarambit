"""Resolve the measured near-threshold spine deficit without changing any gate."""
from concurrent.futures import ThreadPoolExecutor
import json
from search import OUT, run


def main():
    source = json.loads((OUT / 'endpoint_fine_summary.json').read_text())[0]
    cases = [(18.5, .175), (18.7, .175), (18, .180), (18.5, .180), (18.7, .180)]
    def probe(item):
        i, (angle, apex) = item
        row = run(f'closure_strict_{i}', source['genes'],
                  {**source['config'], 'ANGLE': angle, 'APEX': apex}, 'fem', .0625)
        print(json.dumps({k: v for k, v in row.items() if k != 'genes'}), flush=True)
        return row
    with ThreadPoolExecutor(max_workers=2) as pool:
        rows = list(pool.map(probe, enumerate(cases)))
    (OUT / 'closure_strict_summary.json').write_text(json.dumps(rows, indent=2) + '\n')
    # Start with the moderate angle/apex adjustment; preserve the larger strict observations
    # as search evidence rather than promoting them without their own measured export.
    order = [3, 0, 2, 4, 1]
    exports = []
    fine = []
    for i in order:
        r = rows[i]
        if not r['legal_geometry'] or r['metrics']['hm_spine_sf'] < 1:
            continue
        export = run(f'closure_export_{i}', r['genes'], r['config'], 'export')
        exports.append(export)
        (OUT / 'closure_export_summary.json').write_text(json.dumps(exports, indent=2) + '\n')
        if not (export['sharpness'] and export['point']):
            continue
        row = run(f'closure_fine_{i}', r['genes'], r['config'], 'fem')
        row['selected_from'] = export['name']
        fine.append(row)
        break
    (OUT / 'closure_fine_summary.json').write_text(json.dumps(fine, indent=2) + '\n')


if __name__ == '__main__':
    main()
