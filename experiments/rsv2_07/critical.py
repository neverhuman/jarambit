"""Rank local offsets on all three qualification mesh areas before CAD replay."""
from concurrent.futures import ThreadPoolExecutor
import json
from search import OUT, run


def spine_margin(row):
    doc = json.loads((OUT / row['name'] / 'result.json').read_text())
    return min(l['metrics']['hm_spine_sf'] for l in doc['levels'] if l['area'] != 2)


def main():
    source = json.loads((OUT / 'endpoint_fine_summary.json').read_text())[0]
    offsets = [(.2, 0), (.2, .05), (.25, .1), (.3, .1), (.35, .15), (.4, .2)]
    def probe(item):
        i, (d7, d8) = item
        genes = source['genes'].copy()
        genes[7] += d7
        genes[8] += d8
        row = run(f'critical_fem_{i}', genes, source['config'], 'fem')
        row['min_spine_three_areas'] = spine_margin(row)
        print(json.dumps({k: v for k, v in row.items() if k != 'genes'}), flush=True)
        return row
    with ThreadPoolExecutor(max_workers=2) as pool:
        rows = list(pool.map(probe, enumerate(offsets)))
    (OUT / 'critical_fem_summary.json').write_text(json.dumps(rows, indent=2) + '\n')
    eligible = [r for r in rows if r['legal_geometry'] and r['min_spine_three_areas'] >= 1]
    eligible.sort(key=lambda r:r['min_spine_three_areas'], reverse=True)
    exports = []
    accepted = []
    for i, r in enumerate(eligible[:2]):
        export = run(f'critical_export_{i}', r['genes'], r['config'], 'export')
        export['fem_source'] = r['name']
        exports.append(export)
        (OUT / 'critical_export_summary.json').write_text(json.dumps(exports, indent=2) + '\n')
        if not (export['sharpness'] and export['point']):
            continue
        r['selected_from'] = export['name']
        accepted.append(r)
        break
    (OUT / 'critical_accepted_summary.json').write_text(json.dumps(accepted, indent=2) + '\n')


if __name__ == '__main__':
    main()
