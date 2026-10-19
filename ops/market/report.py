#!/usr/bin/env python3
"""Generate a readable research report and the scoped main README insertion."""
import csv
import json
import pathlib
from build import DATA, ROOT, PALETTE


def main():
    rows=json.loads((DATA/'census.json').read_text())
    s=json.loads((DATA/'summary.json').read_text())
    bench=json.loads((DATA/'benchmark.json').read_text())
    bm=bench['primary']['mass_g']
    lines=[
        '# Karambit market census and SWITCHBACK weight comparison',
        '',
        'Research date: **October 13, 2026 (America/Denver)**. Minimum published blade length: **3.375 in = 85.725 mm**, inclusive.',
        '',
        f'**SWITCHBACK C.10a RC4 is {bm:.3f} g by CAD calculation.** That is nominally lighter than **{s["benchmark_lighter_than_count"]}/{s["plotted_entries"]} ({s["benchmark_lighter_than_fraction"]:.1%})** comparable catalog entries and **{s["benchmark_below_median_fraction"]:.1%} below** their **{s["market_median_g"]:.2f} g median**. These are catalog comparisons, not measured product tests. The TALIM titanium listing is lighter at **51.03 g**. The FGX polymer listing is approximately **65.2 g**, close enough that published rounding prevents a demonstrated physical weight advantage.',
        '',
        '![Market blade-length and weight comparison](figures/market_landscape.png)',
        '',
        'The gold star identifies C.10a. Moving left reduces weight; moving up increases listed blade length. Colors identify blade material families. Diamonds identify broader styles, including straight ring designs and ringless hawkbills. The histogram above uses the same weight axis and common bins. Product names preserve source identity; the project comparison concerns utility knives and catalog specifications.',
        '',
        '![Transparent material histogram](figures/weight_histogram.png)',
        '',
        'Histogram bins are **25 g**, shared across groups, with **alpha 0.42**. Heights are counts of catalog entries in each bin. Overlapping colors do not form a stacked total. Each color includes only entries with a verified current published knife-weight field or explicit knife-only mass. This is not sales share.',
        '',
        '## Coverage and what “credible” means',
        '',
        f'The eight original CSVs contain **{s["input_rows"]} rows**. All are preserved byte-for-byte, hashed, and assigned exactly one canonical disposition. One additional qualifying nonmetal model was discovered during the current category review. The resulting **{s["canonical_entries"]} canonical entries** contain **{s["qualified_entries"]} current-source-confirmed qualifying listings**, **{s["plotted_entries"]} comparable weights**, and **{s["qualified_missing_comparable_weight"]} qualifying entries without comparable verified weights. The remaining entries require source/identity/length resolution; they are not silently counted as verified.',
        '',
        'Credibility here means an identifiable manufacturer, established retailer, or named custom dealer with a product page whose current published blade-length field was read. It does not certify alloy chemistry, heat treatment, utility performance, product authenticity, stock availability or seller measurements. Independent marketplace claims remain candidates. Archived and custom-dealer pages are retained as historical examples, with availability kept separate from qualification. A page existing does not establish that the knife can currently be purchased.',
        '',
        '**This is a broad source-audited census, not a proof that every global model has been found.** The candidate queue, merchant fetch failures, short-blade exclusions, and missing weights remain in the checked-in evidence. The supplied files overlap heavily; multiple CSV mentions are not independent confirmations. The current Blade HQ category was reviewed for additional leads, including the FGX and separate rubber trainer; a full 250-item category/stock audit has not been completed.',
        '',
        '| Disposition | Entries |', '|---|---:|',
    ]
    lines.extend(f'| {status.replace("_"," ")} | {n} |' for status,n in s['qualification_counts'].items())
    lines += ['', '## Blade materials', '',
              'Blade substrate controls grouping. Titanium coatings, titanium clips and titanium handles do not turn a steel blade into a titanium blade. PM stainless steels such as M390 and CPM 154 remain in stainless steel; CPM 3V, K340 and Z-Wear are grouped as PM tool steels. Damascus is a declared pattern/composite category with unspecified chemistry, not a single standardized grade.', '',
              '| Material family | Qualifying entries | Comparable weights |', '|---|---:|---:|']
    for family in PALETTE:
        lines.append(f'| {family} | {s["material_counts"].get(family,0)} | {s["material_weight_counts"].get(family,0)} |')
    lines += ['', 'The exact-grade distribution below preserves individual grades; it avoids a long, unreadable multicolor histogram legend.', '',
              '![Every comparable weight by exact blade grade](figures/weight_by_grade.png)', '',
              '## Important reconciliations', '',
              '- **TALIM titanium:** Northern Knives explicitly lists a 3.5 in titanium blade, 1.8 oz live knife, and 3.4 oz with sheath. The product heading/HTML title disagree TALIM/TUSK; SKU and URL remain attached. It belongs to the broader ringless-style group and prevents a “lightest overall” claim.',
              '- **HIGHTRON Cyroclaw:** manufacturer and Blade HQ list 3.94 in, with a 2.44 in edge. Arizona Custom Knives repeats that description but its own product-details table lists 2.75 in. Both color variants are merged; the unresolved length remains outside the qualifying comparison.',
              '- **Schwartz Spartan:** KnifeCenter’s 4.4 oz expressly includes the sheath. A distinct custom-dealer example lists 3.7 oz with unclear scope. Neither a sheath mass nor an estimated 103 g is treated as verified bare-knife mass.',
              '- **TOPS TAC-TOPS:** current manufacturer table lists a 7.13 in blade, 5.75 in cutting edge, 10 oz knife and 14.5 oz with sheath. Older 6.125 in catalog claims remain visible.',
              '- **Boker Wildcat XL:** manufacturer 3.43 in / 5.78 oz takes precedence over Blade HQ 3.375 in / 5 oz. Both lengths qualify. Weight disagreement is retained.',
              '- **Boker Straight:** sources disagree on 7Cr17MoV versus 440A; the consolidated grade remains unspecified. It is a broader straight-ring comparator.',
              '- **Black Legion set:** three color variants and an alternate MTech label are collapsed. The 3.5 in karambit component qualifies, but a three-knife/set weight is never used for a single knife.',
              '- **M48 Liberator/Falcon:** UC3334 appears under conflicting model names and 4.63/5.25 in lengths. It remains conditional rather than counting the same product twice as verified.',
              '- **Emerson Super:** Blade HQ/Oso Grande list 3.375/3.4 in; Knivesandtools lists 8 cm (3.15 in). The unresolved measurement/SKU boundary stays conditional. A 3–3.49 in storefront filter range is not treated as an exact blade measurement.',
              '- **Maxace Hela:** published 6.17/7.30 oz specifications conflict across M20A/B/C/E listings. Length qualifies; weight stays unresolved.',
              '- **Rogue marketplace prototype:** structured length is 100 mm, correcting the input 4.75 in claim. Titanium/Kevlar-carbon-fiber and 3.75 oz remain single-seller claims in the candidate queue.',
              '- **Knife Country USA:** the catalog weight block sits with shipping/package dimensions. These values are retained in observations and excluded from knife-weight comparisons.',
              '', '## Sensitivity and claim limits', '',
              '| Comparable subset | Entries | Median g | Nominally heavier than C.10a |', '|---|---:|---:|---:|']
    for name,sub in s['sensitivity'].items():lines.append(f'| {name.replace("_"," ")} | {sub["n"]} | {sub["median_g"]:.2f} | {sub["nominally_heavier_than_benchmark"]} |')
    bounds=s['missing_weight_bounds']
    lines += ['',
              f'Including all **{s["qualified_entries"]} qualifying entries**, the missing-weight extremes put the nominal share heavier than C.10a between **{bounds["minimum_fraction_lighter_than_benchmark"]:.1%} and {bounds["maximum_fraction_lighter_than_benchmark"]:.1%}**. These are bookkeeping bounds, not statistical confidence intervals. The headline percentage uses only the **{s["plotted_entries"]}** comparable weights.',
              '',
              'C.10a is pinned to repository base `7042f199ad1a9c9e924b7aa68d514e8ea5123567`, the accepted C.10a RC4 mass/outline records and the STEP SHA in `benchmark.json`. Its 96.653 mm tip-to-heel distance is used on the chart; 87.003 mm projection and 101.180 mm edge arc remain recorded separately. Commercial blade-length fields do not establish these same three dimensions. Listed blade length is not assumed to equal cutting-edge length.',
              '',
              'The historical design of record, `g3_tip+00_t60_tt3`, is 84.80 g. C.10a is a release candidate, not a manufactured product or a promoted design of record. A later C.10c RC5 PR appeared during research; it is not substituted for accepted main evidence. No market mass comparison establishes strength, sharpness, safety, fitness, edge life or manufacturing readiness.',
              '', '## Files and reproduction', '',
              '- [Qualifying entries organized by material](qualified_by_material.csv)',
              '- [Complete normalized census](census.csv) and [JSON](census.json)',
              '- [Exact-grade summary](material_summary.csv)',
              '- [Every lighter or nearby listing up to 80 g, with full URLs](nearby_weight_links.csv)',
              '- [Candidate and conflict queue](review_queue.csv)',
              '- [Screened short blades/trainers](screened_exclusions.csv)',
              '- [Original input manifest](input_manifest.json), [all observations](observations.json), [frozen source facts](source_facts.json), [web-reader factual evidence](web_evidence.json), [retrieval outcomes](retrieval_ledger.json), and [curated reconciliations](curation.json)',
              '- [Full-size SVG](figures/market_landscape.svg), [PDF](figures/market_landscape.pdf), and [histogram PDF](figures/weight_histogram.pdf)',
              '', 'From the repository root:', '', '```sh',
              'python3 ops/market/build.py --no-plots', 'python3 ops/market/validate.py',
              'python3 ops/market/report.py', '# Optional: regenerate PNG/SVG/PDF with matplotlib and numpy available.',
              'python3 ops/market/build.py', '```', '',
              'The ordinary build is offline and reads frozen committed facts. It does not require scratch, web access, a CAD engine, FEM runs or a shared environment upgrade. `normalize.py` can regenerate original observations from the preserved inputs. `harvest.py` is an optional future public-source refresh into private scratch; do not use its output to replace curation automatically. The original page bodies stay outside Git. Rechecking requires inspecting identities, dimensions, weight scopes and candidate status before freezing new facts.',
              '', '## Qualifying catalog, organized by material', '',
              'A dash means no comparable verified knife weight. Every model links to its selected product source. Material grade is a published claim, not an independent assay.', '',
              '| Blade family / grade | Model | Listed blade in | Comparable knife g | Weight basis |',
              '|---|---|---:|---:|---|']
    for r in sorted([r for r in rows if r['qualified']],key=lambda r:(r['material_family'],r['blade_grade'],r['model'])):
        mass=f'{r["weight_g"]:.2f}' if r['plot_eligible'] else '—'
        model=r['model'].replace('|','/')
        lines.append(f'| {r["material_family"]} / {r["blade_grade"]} | [{model}]({r["source_url"]}) | {r["blade_length_in"]:.3f} | {mass} | {r["weight_basis"].replace("_"," ")} |')
    (DATA/'README.md').write_text('\n'.join(lines)+'\n')
    pw = [r['weight_g'] for r in rows if r['plot_eligible']]
    med_g = s['market_median_g']
    def cmp_stats(mass_g):
        lt = sum(1 for w in pw if mass_g < w)
        return lt, lt / len(pw), (med_g - mass_g) / med_g
    w411 = json.loads((ROOT / 'evidence/evolution_waves/2026-10-18_ultralight_v1/xbabe3/w411c3/best_so_far_26_gates.json').read_text())['best_stage2_26_gate_design']
    w282_doc = json.loads((ROOT / 'evidence/rs_v2/w282_ultrarunner/w282_p14_c01_cad_eval.json').read_text())
    w282 = w282_doc['engineering_metrics']
    w282_lbl = w282_doc['subject']
    g3 = json.loads((ROOT / 'evidence/rs_v2/g3_tip+00_t60_tt3.json').read_text())
    w411_m = w411['mass_g']
    w411_m3d = w411['mass_3d_g']
    w411_ul = w411['ultralight_weighted_fitness']
    w282_m = w282['mass_g']
    w282_m3d = w282['mass_3d_g']
    g3_m = g3['mass_g']
    w282_lt, w282_lt_f, w282_med_f = cmp_stats(w282_m)
    dor_lt, dor_lt_f, dor_med_f = cmp_stats(g3_m)
    w411_lt, w411_lt_f, w411_med_f = cmp_stats(w411_m)
    snippet=f'''<!-- MARKET:CENSUS:BEGIN -->
## Market weight comparison

The source-audited [October 13, 2026 census](docs/market/2026-10-13/README.md) consolidates eight supplied CSVs plus an additional qualifying polymer model. **{s['qualified_entries']} identifiable entries meet the published 3.375 in blade minimum; {s['plotted_entries']} have comparable verified published knife weights.** Color-only duplicates are consolidated; genuine material variants and distinct custom examples remain separate. Missing weights and unresolved candidates stay visible in the evidence tables.

**Committed genome-pipeline Stage-2 designs: `{w282_lbl}` at {w282_m:.3f} g ({w282_m3d:.3f} g 3D solid, historical 16-gate screen record) and `{w411['label']}` at {w411_m:.3f} g ({w411_m3d:.3f} g 3D solid, `W_ULTRALIGHT_v1 = {w411_ul:.6f}`, committed ultralight-v1 Stage-2 leader).** `{w282_lbl}` is nominally lighter than {w282_lt} of {s['plotted_entries']} plotted entries ({w282_lt_f:.1%}) and {w282_med_f:.1%} below their {med_g:.2f} g median; `{g3['label']}` ({g3_m:.2f} g, lighter than {dor_lt} of {s['plotted_entries']} or {dor_lt_f:.1%}, {dor_med_f:.1%} below median) remains the historical design of record; and `{w411['label']}` is lighter than {w411_lt} of {s['plotted_entries']} plotted entries ({w411_lt_f:.1%}), {w411_med_f:.1%} below median. TALIM titanium lists 51.03 g, so this does not support a lightest-overall claim. None is qualified: `{w282_lbl}` passes the 16 legacy gates but fails RS-karambit-v2 (fatigue, Gate-S sharpness and human-max fail; tip-point unverified); 0 candidates pass RS-karambit-v2 all-hard (champion = null). Stage-2 `W_ULTRALIGHT_v1` is for ranking only, not qualification. All are unmanufactured and unpromoted. Listed commercial lengths use different measurement conventions, and material/weight comparisons establish no performance equivalence.

[Catalog by material](docs/market/2026-10-13/qualified_by_material.csv) · [Complete normalized census](docs/market/2026-10-13/census.csv) · [Exact-grade summary](docs/market/2026-10-13/material_summary.csv) · [Missing/conditional candidate queue](docs/market/2026-10-13/review_queue.csv)
<!-- MARKET:CENSUS:END -->'''
    readme=ROOT/'README.md'
    if readme.exists():text=readme.read_text()
    else:text=(ROOT/'.study/market/base_README.md').read_text()
    if '<!-- MARKET:CENSUS:BEGIN -->' in text:
        import re
        text=re.sub(r'<!-- MARKET:CENSUS:BEGIN -->.*?<!-- MARKET:CENSUS:END -->',lambda _:snippet,text,flags=re.S)
    else:text=text.replace('<!-- GEN:RELEASE:BEGIN -->',snippet+'\n\n<!-- GEN:RELEASE:BEGIN -->',1)
    readme.write_text(text)
    print('Research report and scoped README comparison generated')


if __name__=='__main__':main()
