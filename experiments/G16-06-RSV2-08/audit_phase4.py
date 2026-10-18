#!/usr/bin/env python3
"""Audit the eight completed five-mesh interpolation cases and expose exact inputs."""
import csv
import json
import os
from run_phase4 import HERE, ROOT, OUT, TARGET, AREAS, verify_sources, digest

if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10
    rp=HERE/'phase4_receipt.json';r=json.loads(rp.read_text())
    assert r['state']=='complete' and r['completed_cases']==8 and not r['errors']
    assert r['cpus']==[12,13] and r['nice_value']>=10 and r['cargo_build_jobs']==2
    for rel,want in r['sha256'].items():assert digest(ROOT/rel)==want,rel
    assert '1 passed; 0 failed' in (OUT/'requirements-test.log').read_text()
    prior=json.loads((HERE/'phase3_receipt.json').read_text())
    parents={v['name']:v for v in prior['results'] if v['name'] in ['placement_05','local_04']}
    review=HERE/'phase4_candidates';review.mkdir(exist_ok=True)
    checks=[]
    for row in r['results']:
        name=row['name'];parent=row['parent'];alpha=row['alpha']
        p=HERE/'phase3_candidates'/f'{parent}.csv';assert digest(p)==parents[parent]['genome_sha256']
        base=list(csv.reader(p.open()))[1]
        raw=list(csv.reader((ROOT/row['genome']).open()))[1]
        canonical=list(csv.reader((OUT/f'{name}-full.canonical.csv').open()))[1]
        assert len(base)==len(raw)==len(canonical)==129
        assert [float(v) for v in raw[:128]]==[float(v) for v in canonical[:128]]==row['genes']
        assert raw[128]==base[128]==canonical[128]
        assert [i for i,(a,b) in enumerate(zip(base,raw)) if a!=b]==[29,30]
        assert raw[6:9]==base[6:9]
        for i,t in TARGET.items():assert float(raw[i])==(1-alpha)*float(base[i])+alpha*t
        full=json.loads((OUT/f'{name}-full.json').read_text())
        verify_sources(full['provenance'])
        assert full['levels']==row['levels'] and [v['area'] for v in row['levels']]==AREAS
        assert full['solver']=='skyline'
        row['actual_holes']=dict(names=full['holes'],x_bounds=full['hole_x_bounds'])
        row['window_present']='window' in full['holes']
        row['worst_edge_face']=full['worst_edge_face']
        for rel,want in row['sha256'].items():assert digest(ROOT/rel)==want,rel
        # A single unchanged material/section law must give the same fatigue identity at every mesh.
        denominators=[l['metrics']['lat_rob']/l['metrics']['fat_sf'] for l in row['levels']]
        assert max(denominators)-min(denominators)<1e-10
        row['fatigue_identity_denominator']=denominators[0]
        row['known_other_gate_failures']={str(l['area']):{k:l['metrics'][k] for k in
            ['hm_twist_sf','hm_spine_sf','hm_ringback_sf'] if l['metrics'][k]<1.} for l in row['levels']}
        row['mass_soft_80g_exceeded']=row['mass_g']>80.
        row['parent_certification']={k:parents[parent]['certification'][k] for k in row['certification']}
        row['parent_strict']={k:parents[parent]['strict'][k] for k in row['strict']}
        destination=review/f'{name}.csv';destination.write_bytes((ROOT/row['genome']).read_bytes())
        row['reviewable_genome']=str(destination.relative_to(ROOT));row['sha256'][str(destination.relative_to(ROOT))]=digest(destination)
        checks.append(dict(name=name,only_CP29_CP30_changed=True,hand_CP06_CP08_byte_preserved=True,
                           target_interpolation_exact=True,canonical_numeric_and_material_identity=True,
                           all_five_meshes=True,native_source_and_artifact_hashes_match=True,fatigue_identity=True))
    events=[json.loads(s) for s in (OUT/'progress.jsonl').read_text().splitlines()]
    assert sorted(e['name'] for e in events if e['stage']=='complete')==sorted(v['name'] for v in r['results'])
    audit=dict(cases=8,requested_FEM_mesh_evaluations=40,export_evaluations=8,side_a4_evaluations=8,
               GCI_evaluations=8,requirements_consistency_test=dict(passed=1,failed=0),checks=checks)
    ap=OUT/'integrity_audit.json';ap.write_text(json.dumps(audit,indent=2)+'\n')
    r['final_integrity_audit']=audit
    for p in [ap,HERE/'audit_phase4.py',HERE/'phase3_receipt.json',OUT/'assembly.json',OUT/'inputs/Q7_REPORT.md']:
        r['sha256'][str(p.relative_to(ROOT))]=digest(p)
    rp.write_text(json.dumps(r,indent=2)+'\n')
    report=HERE/'PHASE4.md';marker='\n## Full Phase-4 measurements\n'
    lines=[marker,'All 40 requested mesh observations and all eight export/support evaluations completed. Exact assigned interpolations, preserved hand/other genes, canonical inputs, source/material identity, raw hashes and fatigue identities pass. The unchanged native requirements consistency test passes (1/1). `phase4_candidates/` contains the eight exact evaluated CSVs.','',
           '| Parent reference (Phase 3) | Cert HM spine | Strict HM spine | Cert lat N | Strict lat N | Cert seq | Strict seq |',
           '| --- | --- | --- | --- | --- | --- | --- |']
    for name,v in parents.items():
        c,t=v['certification'],v['strict']
        lines.append(f'| {name} | {c["hm_spine_sf"]:.9f} | {t["hm_spine_sf"]:.9f} | {c["lat_rob"]:.9f} | {t["lat_rob"]:.9f} | {c["seq_ratio"]:.9f} | {t["seq_ratio"]:.9f} |')
    lines+=['','| Case | Area mm² | HM spine SF | Lateral N | Fatigue SF | Sequence | Spine peak x,y |','| --- | --- | --- | --- | --- | --- | --- |']
    for v in r['results']:
        for l in v['levels']:
            m=l['metrics'];peak=l['diagnostic_peaks']['spine']['at']
            lines.append(f'| {v["name"]} | {l["area"]} | {m["hm_spine_sf"]:.9f} | {m["lat_rob"]:.9f} | {m["fat_sf"]:.9f} | {m["seq_ratio"]:.9f} | {peak} |')
    lines+=['','| Case | GCI fit / p | Max half-angle ° | Max apex mm | Point | Census | A4 eigenvalue | A4 seq / cert |','| --- | --- | --- | --- | --- | --- | --- | --- |']
    for v in r['results']:
        g=v['GCI']['seq_ratio_gci'];s=v['sharpness'];a=v['a4']
        lines.append(f'| {v["name"]} | {g["fit_status"]} / {g["p"]} | {s["max_half_angle_deg"]:.9f} | {s["max_apex_mm"]:.9f} | {v["point"]["pass"]} | {v["solid"]["census"]} | {a["buckling_eigenvalue"]:.9f} | {a["supported_seq_ratio"]:.9f} / {a["certified_seq_ratio"]:.9f} |')
    passed=[v['name'] for v in r['results'] if v['scoped_joint_pass']]
    r['outcome_counts']={k:sum(bool(v[k]) for v in r['results']) for k in
                         ['geometry_legal','gci_admitted','export_pass','side_a4_pass','fine_target_pass',
                          'all_five_target_pass','all_five_sequence_monotonic','scoped_joint_pass']}
    rp.write_text(json.dumps(r,indent=2)+'\n')
    lines+=['','| Case | Geometry legal | Native geometry failures | Cert HM twist SF | Strict HM twist SF |',
            '| --- | --- | --- | --- | --- |']
    for v in r['results']:
        reasons='; '.join(v['rejects']+v['aesthetic_failures'])
        lines.append(f'| {v["name"]} | {v["geometry_legal"]} | {reasons} | {v["certification"]["hm_twist_sf"]:.9f} | {v["strict"]["hm_twist_sf"]:.9f} |')
    lines+=['',f'Pass counts among eight cases: {r["outcome_counts"]}.']
    lines+=['',f'Scoped joint passes: {passed}. This finite eight-case study does not establish global infeasibility or an optimum. Full RS-v2/manufacturing qualification remains false. Human-max twist, independent 3D/cross-solver checks and the inherited collar/flat-stock inconsistency are not waived.','',
            'Native GCI admission is a checked three-level estimate, with asymptotic validation still not established. Every coarse failure and all-five sequence monotonicity are retained separately. The raw native GCI/A4 adapter retains its Phase-2 label; exact Phase-4 inputs, meshes and content hashes bind each evaluation. These are law+config+genome prototype results, not production CSV-only exporter qualification.','',
            'Audit after a completed run:','```sh','nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/audit_phase4.py','```','']
    report.write_text(report.read_text().split(marker)[0]+'\n'.join(lines))
    print(json.dumps(dict(audit='pass',cases=8,requested_meshes=40,joint_passes=passed)))
