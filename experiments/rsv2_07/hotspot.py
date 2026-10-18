"""Follow the retained strict spine hotspot with small native inner-edge offsets."""
from concurrent.futures import ThreadPoolExecutor
import json
from search import OUT, run


def main():
    source = json.loads((OUT / 'endpoint_fine_summary.json').read_text())[0]
    changes = [(.2, 0), (.5, 0), (0, .2), (.3, .2), (.7, .4)]
    def probe(item):
        i, (d7, d8) = item
        genes = source['genes'].copy()
        genes[7] += d7
        genes[8] += d8
        row = run(f'hotspot_strict_{i}', genes, source['config'], 'fem', .0625)
        print(json.dumps({k: v for k, v in row.items() if k != 'genes'}), flush=True)
        return row
    with ThreadPoolExecutor(max_workers=2) as pool:
        rows = list(pool.map(probe, enumerate(changes)))
    (OUT / 'hotspot_strict_summary.json').write_text(json.dumps(rows, indent=2) + '\n')
    eligible = [r for r in rows if r['legal_geometry'] and r['metrics']['hm_spine_sf'] >= 1]
    eligible.sort(key=lambda r:r['metrics']['hm_spine_sf'], reverse=True)
    exports = []
    fine = []
    for i, r in enumerate(eligible[:2]):
        export = run(f'hotspot_export_{i}', r['genes'], r['config'], 'export')
        export['strict_source'] = r['name']
        exports.append(export)
        (OUT / 'hotspot_export_summary.json').write_text(json.dumps(exports, indent=2) + '\n')
        if not (export['sharpness'] and export['point']):
            continue
        row = run(f'hotspot_fine_{i}', r['genes'], r['config'], 'fem')
        row['selected_from'] = export['name']
        fine.append(row)
        break
    (OUT / 'hotspot_fine_summary.json').write_text(json.dumps(fine, indent=2) + '\n')


if __name__ == '__main__':
    main()
