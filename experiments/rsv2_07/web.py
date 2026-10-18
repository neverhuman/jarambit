"""Test broader, gradual compound transitions after the narrow export failed."""
from concurrent.futures import ThreadPoolExecutor
import json
from search import OUT, run


def margin(row):
    d = json.loads((OUT / row['name'] / 'result.json').read_text())
    return min(l['metrics']['hm_spine_sf'] for l in d['levels'] if l['area'] != 2)


def main():
    # Select the exact CP7-only geometry by name; completion order in the thread pool is irrelevant.
    source = next(r for r in json.loads((OUT / 'critical_fem_summary.json').read_text()) if r['name'] == 'critical_fem_0')
    laws = [(.55, .9), (.65, 1.1), (.8, 1.3)]
    def probe(item):
        i, (d0, d1) = item
        row = run(f'web_fem_{i}', source['genes'], {**source['config'], 'D0': d0, 'D1': d1, 'GRID': .05}, 'fem')
        row['min_spine_three_areas'] = margin(row)
        print(json.dumps({k: v for k, v in row.items() if k != 'genes'}), flush=True)
        return row
    with ThreadPoolExecutor(max_workers=2) as pool:
        rows = list(pool.map(probe, enumerate(laws)))
    (OUT / 'web_fem_summary.json').write_text(json.dumps(rows, indent=2) + '\n')
    eligible = sorted((r for r in rows if r['legal_geometry'] and r['min_spine_three_areas'] >= 1),
                      key=lambda r:r['min_spine_three_areas'], reverse=True)
    exports = []
    accepted = []
    for i, r in enumerate(eligible[:2]):
        export = run(f'web_export_{i}', r['genes'], r['config'], 'export')
        export['fem_source'] = r['name']
        exports.append(export)
        (OUT / 'web_export_summary.json').write_text(json.dumps(exports, indent=2) + '\n')
        if export['sharpness'] and export['point']:
            r['selected_from'] = export['name']
            accepted.append(r)
            break
    (OUT / 'web_accepted_summary.json').write_text(json.dumps(accepted, indent=2) + '\n')


if __name__ == '__main__':
    main()
