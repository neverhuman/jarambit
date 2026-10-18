#!/usr/bin/env python3
"""Check completed Phase-5 measurements against frozen inputs and native evidence."""
import csv
import hashlib
import json
import os
import struct
from prepare_phase5 import HERE, ROOT, OUT, SNAP, digest
from run_phase5 import AREAS, ALLOWED, verify_provenance

if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10
    path=HERE/'phase5_receipt.json';r=json.loads(path.read_text())
    assert r['state']=='complete' and not r['errors']
    assert r['cpus']==[12,13] and r['cargo_build_jobs']==2 and r['solver']=='faer'
    assert r['cert_sequence_floor']==1.20 and r['strict_sequence_floor']==1.25
    assert r['thresholds']=={'hm_spine_sf':1.,'lat_rob':358.,'fat_sf':1.,'sf_hunter':1.377}
    for rel,want in r['sha256'].items():assert digest(ROOT/rel)==want,rel
    prov=json.loads((OUT/'source_provenance.json').read_text())
    for group in ['production_sha256','local_source_sha256']:
        for rel,want in prov[group].items():assert digest(ROOT/rel)==want,rel
    for inp in prov['inputs'].values():assert digest(ROOT/inp['snapshot'])==inp['sha256']
    prior=json.loads((OUT/'attempt1_receipt.json').read_text())
    for original,snapshot in [('experiments/G16-06-RSV2-08/run_phase5.py','run_phase5-attempt1.py'),
                              ('target/G16-06-RSV2-08/phase5/assembly.json','attempt1-assembly.json')]:
        assert digest(OUT/snapshot)==prior['sha256'][original]
    assert len(prior['errors'])==24 and all(e['error']=='AssertionError()' for e in prior['errors'])
    for cached in r['cached_artifacts']:assert digest(ROOT/cached['path'])==cached['sha256']
    assert '2 passed; 0 failed' in (OUT/'gci-tests.log').read_text()
    assert '1 passed; 0 failed' in (OUT/'requirements-test.log').read_text()
    base=list(csv.reader((SNAP/'q7.csv').open()))
    names=set();observations=0;exported=0
    for v in r['results']:
        name=v['name'];assert name not in names;names.add(name)
        raw=list(csv.reader((ROOT/v['genome']).open()))
        assert len(raw[1])==129 and raw[0]==base[0] and raw[1][128]==base[1][128]
        changed=[i for i,(a,b) in enumerate(zip(base[1],raw[1])) if a!=b]
        assert set(changed)<=ALLOWED and changed==v['changed_indices']
        assert [float(x) for x in raw[1][:128]]==v['genes'] and v['genes'][64]==42.
        for rel,want in v['sha256'].items():assert digest(ROOT/rel)==want,rel
        geom=json.loads((OUT/f'{name}-geometry.json').read_text());verify_provenance(geom)
        assert v['observed_genes']==geom['genes']
        assert v['input_was_canonical']==geom['input_was_canonical']
        assert v['domain_projection_differences']==[i for i,(a,b) in enumerate(zip(v['genes'],geom['genes'])) if a!=b]
        assert v['geometry_legal']==bool(geom['input_was_canonical'] and geom['within_genome_domain'] and geom['rs_v2'] and geom['aes_pass'] and not geom['rejects'])
        if not v['geometry_legal']:
            assert not v['levels'] and v['gci_admitted'] is None and not v['scoped_joint_pass']
            continue
        assert v['window_count']>=1 and 'window' in v['holes']
        full=json.loads((OUT/f'{name}-fem.json').read_text());verify_provenance(full)
        assert full['solver']=='faer' and full['cpu_affinity'].split(':')[1].strip()=='12-13'
        assert full['cargo_build_jobs']=='2' and full['nice_priority']>=10
        assert full['genes']==v['genes'] and full['config']==v['config']
        assert full['levels']==v['levels'] and [l['area'] for l in v['levels']]==AREAS
        def numerical_pass(levels):
            return all(all(l['metrics'][k]>=floor for k,floor in r['thresholds'].items()) and
                       l['metrics']['seq_ratio']>=(1.25 if l['area']==.0625 else 1.20) for l in levels)
        assert v['all_five_targets_pass']==numerical_pass(v['levels'])
        assert v['fine_targets_pass']==numerical_pass(v['levels'][2:])
        for k in v['certification']:
            assert v['certification'][k]==min(l['metrics'][k] for l in v['levels'][2:4])
            assert v['strict'][k]==v['levels'][4]['metrics'][k]
        canonical=list(csv.reader((OUT/f'{name}-fem.canonical.csv').open()))[1]
        assert [float(x) for x in canonical[:128]]==v['genes'] and canonical[128]==raw[1][128]
        denominator=[l['metrics']['lat_rob']/l['metrics']['fat_sf'] for l in v['levels']]
        assert max(denominator)-min(denominator)<1e-10
        observations+=5
        gci=json.loads((OUT/f'{name}-gci.json').read_text());verify_provenance(gci)
        assert gci['plate_gci']==v['gci'] and gci['G16-15']==v['gci_admitted']
        for l in v['gci']['levels_fine_to_coarse']:
            m=next(z['metrics'] for z in v['levels'] if z['area']==l['max_area_mm2'])
            assert m['seq_ratio']==l['seq_ratio'] and m['sf_hunter']==l['sf_hunter']
        v['fatigue_identity_denominator']=denominator[0]
        v['known_other_gate_failures']={str(l['area']):{k:l['metrics'][k] for k in ['hm_twist_sf','hm_ringback_sf'] if l['metrics'][k]<1.} for l in v['levels']}
        if v['export_pass'] is not None:
            exported+=1
            export=json.loads((OUT/f'{name}-export.json').read_text());verify_provenance(export)
            assert export['genes']==v['genes'] and export['config']==v['config']
            side=json.loads((OUT/f'{name}-a4.json').read_text());verify_provenance(side)
            assert side['a4']==v['a4'] and side['G16-13']==v['side_a4_pass']
            mesh=json.loads((OUT/f'{name}-a4.mesh.json').read_text())
            data=(b''.join(struct.pack('<dd',*q) for q in mesh['xy'])+
                  b''.join(struct.pack('<QQQ',*t) for t in mesh['tris'])+
                  b''.join(struct.pack('<d',t) for t in mesh['te']))
            assert hashlib.sha256(data).hexdigest()==v['gci']['levels_fine_to_coarse'][0]['mesh_sha256']
            assert v['a4']['same_mesh_ring_clamp_seq_ratio']==v['levels'][2]['metrics']['seq_ratio']
            assert v['a4']['certified_seq_ratio']==v['certification']['seq_ratio']
        assert v['scoped_joint_pass']==bool(v['geometry_legal'] and v['all_five_targets_pass'] and v['gci_admitted'] and v['export_pass'] and v['side_a4_pass'])
        assert v['all_gate_qualification'] is False
    assert len(names)==r['completed_cases'] and observations==r['mesh_evaluations']
    assembly=json.loads((OUT/'assembly.json').read_text());assert names=={v['name'] for v in assembly}
    assert exported==len(r['selection'])
    audit=dict(status='pass',cases=len(names),legal_five_mesh_cases=observations//5,mesh_observations=observations,
               full_exports_and_a4=exported,native_requirement_tests_passed=1,native_gci_tests_passed=2,
               parent_and_canonical_identity=True,only_assigned_genes_changed=True,native_fem_identity=True,
               source_artifact_hashes=True,gci_a4_mesh_identity=True,matched_export_fem_inputs=True,
               fatigue_identity=True,thresholds_unchanged=True)
    ap=OUT/'integrity_audit.json';ap.write_text(json.dumps(audit,indent=2)+'\n')
    r['integrity_audit']=audit
    for p in [ap,HERE/'audit_phase5.py',OUT/'gci-tests.log',OUT/'requirements-test.log',
              OUT/'q7-baseline-full.json',OUT/'attempt1_receipt.json',OUT/'run_phase5-attempt1.py',
              OUT/'attempt1-assembly.json',OUT/'attempt1-run.log']:
        r['sha256'][str(p.relative_to(ROOT))]=digest(p)
    path.write_text(json.dumps(r,indent=2)+'\n')
    report=HERE/'PHASE5.md';marker='\n## Verified per-mesh measurements\n'
    lines=[marker, f'Integrity audit PASS: {audit}.', '',
           '| Case | Area mm² | Spine SF | Lateral N | Fatigue SF | Hunter SF | Sequence | Spine peak x,y |',
           '| --- | --- | --- | --- | --- | --- | --- | --- |']
    for v in r['results']:
        for l in v['levels']:
            m=l['metrics'];p=l['diagnostic_peaks']['spine']['at']
            lines.append(f'| {v["name"]} | {l["area"]} | {m["hm_spine_sf"]:.9f} | {m["lat_rob"]:.9f} | {m["fat_sf"]:.9f} | {m["sf_hunter"]:.9f} | {m["seq_ratio"]:.9f} | {p} |')
    lines+=['', '| Export/A4 case | GCI fit / p | Half-angle ° | Apex mm | Point | Bore mm | Census | A4 eigenvalue | Supported / cert seq |',
            '| --- | --- | --- | --- | --- | --- | --- | --- | --- |']
    for v in r['results']:
        if v['export_pass'] is None:continue
        g,s,a=v['gci']['seq_ratio_gci'],v['sharpness'],v['a4']
        lines.append(f'| {v["name"]} | {g["fit_status"]} / {g["p"]} | {s["max_half_angle_deg"]:.9f} | {s["max_apex_mm"]:.9f} | {v["point"]["pass"]} | {v["bore_edge_radius_mm"]:.9f} | {v["solid"]["census"]} | {a["buckling_eigenvalue"]:.9f} | {a["supported_seq_ratio"]:.9f} / {a["certified_seq_ratio"]:.9f} |')
    lines+=['','Geometry rejects (retained, never accepted):']
    for v in r['results']:
        if not v['geometry_legal']:lines.append(f'- {v["name"]}: {v["rejects"]}; {v["aesthetic_failures"]}')
    report.write_text(report.read_text().split(marker)[0]+'\n'.join(lines)+'\n')
    print(json.dumps(audit))
