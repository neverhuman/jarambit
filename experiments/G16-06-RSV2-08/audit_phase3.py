#!/usr/bin/env python3
"""Audit completed raw Phase-3 artifacts and add reviewable measurements/inputs."""
import csv
import json
import os
from pathlib import Path
from run_phase3 import HERE, ROOT, OUT, REQUIREMENTS, verify_sources
from run_phase2 import digest

if __name__ == '__main__':
    assert os.sched_getaffinity(0) == {12,13}
    path=HERE/'phase3_receipt.json'
    receipt=json.loads(path.read_text())
    assert receipt['state']=='complete' and receipt['completed_candidates']==9 and not receipt['errors']
    names=['placement_05','local_01','local_04']+[f'interp_s1_{s:.4f}' for s in (.5003,.5006,.5009,.5012,.5015,.5018)]
    assert [r['name'] for r in receipt['results']]==names
    for name,want in receipt['sha256'].items(): assert digest(ROOT/name)==want,name
    assert '1 passed; 0 failed' in (OUT/'requirements-test.log').read_text()
    base=list(csv.reader((HERE/'candidate.csv').open()))[1]
    reviewable=HERE/'phase3_candidates';reviewable.mkdir(exist_ok=True)
    checks=[]
    for row in receipt['results']:
        name=row['name'];full=json.loads((OUT/f'{name}-full.json').read_text())
        verify_sources(full['provenance'])
        assert [l['area'] for l in full['levels']]==[.25,.125,.0625]
        assert full['levels']==row['levels'] and full['solver']=='skyline'
        for relative,want in row['sha256'].items(): assert digest(ROOT/relative)==want,relative
        genome=ROOT/row['genome'];raw=list(csv.reader(genome.open()))[1]
        canonical=list(csv.reader((OUT/f'{name}-full.canonical.csv').open()))[1]
        assert len(raw)==len(canonical)==129
        assert [float(v) for v in raw[:128]]==[float(v) for v in canonical[:128]]
        assert raw[128]==canonical[128]
        if name.startswith('interp_'):
            assert [i for i,(a,b) in enumerate(zip(base,raw)) if a!=b]==[101]
            assert float(raw[101])==float(name.rsplit('_',1)[1])
        for level in row['levels']:
            m=level['metrics']
            assert abs(m['fat_sf']-m['lat_rob']/357.996288224513)<1e-10
        destination=reviewable/f'{name}.csv';destination.write_bytes(genome.read_bytes())
        row['reviewable_genome']=str(destination.relative_to(ROOT))
        row['sha256'][str(destination.relative_to(ROOT))]=digest(destination)
        checks.append(dict(name=name, all_three_meshes=True, canonical_numeric_identity=True,
                           material_identity=True, fatigue_identity=True,
                           source_and_raw_hashes_match=True))
    events=[json.loads(s) for s in (OUT/'progress.jsonl').read_text().splitlines()]
    assert sorted(e['name'] for e in events if e['stage']=='complete')==sorted(names)
    audit=dict(candidates=9, mesh_evaluations=27, export_evaluations=9, side_a4_evaluations=9,
               new_interpolation_gci_evaluations=6, reused_gci_fits_with_identity_checks=3,
               repository_requirements_consistency_test=dict(passed=1,failed=0), checks=checks)
    audit_path=OUT/'integrity_audit.json';audit_path.write_text(json.dumps(audit,indent=2)+'\n')
    receipt['final_integrity_audit']=audit
    receipt['sha256'][str(audit_path.relative_to(ROOT))]=digest(audit_path)
    receipt['sha256'][str((HERE/'audit_phase3.py').relative_to(ROOT))]=digest(HERE/'audit_phase3.py')
    path.write_text(json.dumps(receipt,indent=2)+'\n')
    report=HERE/'PHASE3.md'
    marker='\n## Full Phase-3 measurements\n'
    text=report.read_text().split(marker)[0]
    lines=[marker, 'All 27 plate evaluations, nine exports and nine side-A4 solves completed. Canonical numeric/material identity, the fatigue identity and source/raw hashes pass for every candidate. The unchanged native requirements consistency test passes (1/1). Reviewable input CSVs are in `phase3_candidates/`.', '',
           '| Candidate | Area mm² | Lateral N | Fatigue SF | Hunter SF | Sequence |',
           '| --- | --- | --- | --- | --- | --- |']
    for r in receipt['results']:
        for level in r['levels']:
            m=level['metrics']
            lines.append(f'| {r["name"]} | {level["area"]} | {m["lat_rob"]:.9f} | {m["fat_sf"]:.9f} | {m["sf_hunter"]:.9f} | {m["seq_ratio"]:.9f} |')
    lines += ['', '| Candidate | GCI fit / p | Max apex mm | Max half-angle ° | S2 point | Census | A4 eigenvalue | A4 seq / cert seq |',
              '| --- | --- | --- | --- | --- | --- | --- | --- |']
    for r in receipt['results']:
        fit=r['gci']['seq_ratio_gci'];a=r['a4'];s=r['sharpness']
        lines.append(f'| {r["name"]} | {fit["fit_status"]} / {fit["p"]} | {s["max_apex_mm"]:.9f} | {s["max_half_angle_deg"]:.9f} | {r["point"]["pass"]} | {r["solid"]["census"]} | {a["buckling_eigenvalue"]:.9f} | {a["supported_seq_ratio"]:.9f} / {a["certified_seq_ratio"]:.9f} |')
    passed=[r['name'] for r in receipt['results'] if r['phase3_scoped_pass']]
    lines += ['', f'Scoped passing candidates: {passed}. Full RS-v2/manufacturing qualification remains **false**. The native GCI/side-A4 executable is reused unchanged from Phase 2; its raw task label remains p2, with Phase-3 inputs and exact mesh/content hashes bound in this receipt.', '',
              'GCI admission is a checked three-level fit; native `asymptotic_validation` remains `not_established`. Five-level sequence monotonicity is reported separately and no rejected mesh, export or support result is waived.', '',
              'Audit after a completed run:', '```sh',
              'taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/audit_phase3.py', '```', '']
    report.write_text(text+'\n'.join(lines))
    print(json.dumps(dict(audit='pass', candidates=9, mesh_evaluations=27, scoped_passes=passed)))
