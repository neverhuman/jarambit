#!/usr/bin/env python3
"""Read-only Phase7 audit. Direct invocation is supported on CPUs12,13 or8,9."""
import sys
sys.dont_write_bytecode=True
import csv
import hashlib
import itertools
import json
import os
import struct
from collections import defaultdict
from prepare_phase5 import HERE, ROOT, digest
import run_phase5 as native
from run_phase7 import OUT, ALLOWED, configure

def read(path):return json.loads(path.read_text())

def audit():
    assert os.sched_getaffinity(0) in [{12,13},{8,9}], 'audit requires assigned worker or auditor CPU pair'
    configure();r=read(HERE/'phase7_receipt.json')
    assert r['state']=='complete' and not r['errors'] and r['all_gate_qualification'] is False
    assert r['parent_context']['phase6_report_sha256']==digest(HERE/'PHASE6.md')=='57a86ad4859f61e4ebfef8b18d7297908da40c3b53f69ab6f46bf8436d09ce55'
    assert r['cpus']==[12,13] and r['nice']>=10 and r['cargo_build_jobs']==2 and r['solver']=='faer'
    assert r['thresholds']=={'hm_spine_sf':1.,'lat_rob':358.,'fat_sf':1.,'sf_hunter':1.377}
    assert r['areas_mm2']==[4.,1.,.25,.125,.0625] and r['cert_sequence_floor']==1.20 and r['strict_sequence_floor']==1.25
    for rel,want in r['sha256'].items():assert digest(ROOT/rel)==want,rel
    batch=read(OUT/'field_binding_batch.json')
    assert batch['cpus']==[12,13] and batch['nice']>=10 and batch['cargo_build_jobs']==2
    assert batch['max_concurrent_field_probes']==1 and len(batch['records'])==147
    assert batch['field_probe_sha256']==digest(OUT/'field_probe')
    for rel,want in batch['records'].items():assert digest(ROOT/rel)==want,rel
    prov=read(OUT/'source_provenance.json')
    for group in ['production_sha256','inherited_phase5_source_sha256','local_source_sha256']:
        for rel,want in prov[group].items():assert digest(ROOT/rel)==want,rel
    assert len(prov['production_sha256'])==77
    pheno=(ROOT/'crates/crescent/src/pheno.rs').read_text()
    block=pheno.split('    // fuller centreline: midpoints between inside edge and spine over [s0, s1] of the inside edge\n')[1].split('    let (rd, rw, rq)')[0]
    assert block==prov['native_fuller_centreline_source'] and block in (HERE/'phase7_adapter.rs').read_text()
    fem=(HERE/'phase5_sources/prototype_fem.rs').read_text()
    for hook in ['    crate::diagnostic::record(&t6, nod_sp.as_deref(), nod_rb.as_deref(), tp, sx0);\n','    crate::diagnostic::record_sequence(&cen,&excl,&lat,&twist,p.heel.0,&te,&t6,&nod_pr,&ring6,tp,p);\n']:
        assert fem.count(hook)==1;fem=fem.replace(hook,'')
    assert fem==(ROOT/'crates/crescent/src/fem.rs').read_text()
    assert '2 passed; 0 failed' in (OUT/'gci-tests.log').read_text()
    assert '1 passed; 0 failed' in (OUT/'requirements-test.log').read_text()
    base=list(csv.reader((HERE/'phase5_sources/q7.csv').open()))
    controls=read(OUT/'geometry_controls.json');bounds=controls['bounds']
    assert controls['state']=='complete' and controls['all_four_ring_levels_legal']
    assert [v['ring_wall'] for v in controls['selected']]==[6.2,6.4,6.6,6.8]
    for c in controls['selected']:
        assert c['legal'] and digest(ROOT/c['genome'])==c['genome_sha256']
        assert digest(ROOT/c['native_output'])==c['native_output_sha256']
        d=read(ROOT/c['native_output']);native.verify_provenance(d)
        assert d['input_was_canonical'] and d['rs_v2'] and d['aes_pass'] and not d['rejects']
        assert 60<=d['grip_len_mm']<=80 and d['holder_fit'][0] and d['holder_fit'][1]<=200 and d['holder_fit'][2]<=100
        assert d['inside_concave']>=.97 and d['genes'][:40]==[float(x) for x in base[1][:40]]
    matrix=read(OUT/'nominal_matrix.json')
    assert len(matrix)==r['nominal_cases']==r['canonical_cases']==r['completed_cases']==144
    expected=set(itertools.product([6.2,6.4,6.6,6.8],[0.,.45,.75,1.05],[3.,4.5,6.],[.05,.25,.45]))
    assert {(v['changes']['42'],v['changes']['74'],v['changes']['102'],v['changes']['100']) for v in matrix}==expected
    assert {v['name'] for v in matrix}=={v['name'] for v in r['results']}
    assert {tuple(v['genes'][i] for i in [42,74,102,100]) for v in r['results']}==expected
    groups=defaultdict(list);observations=0;exported=0;counts=defaultdict(int)
    for row in r['results']+r['law_effect_controls']:
        name=row['name'];data=list(csv.reader((ROOT/row['genome']).open()))
        assert data[0]==base[0] and len(data[1])==129 and data[1][128]==base[1][128]
        assert data[1][:40]==base[1][:40] and data[1][64]==base[1][64]
        changed=[i for i,(a,b) in enumerate(zip(base[1],data[1])) if a!=b]
        assert set(changed)<=ALLOWED and changed==row['changed_indices']
        assert [float(x) for x in data[1][:128]]==row['genes']
        assert all(bounds[i][0]<=row['genes'][i]<=bounds[i][1] for i in ALLOWED)
        exceptions=[i for i,(v,(lo,hi)) in enumerate(zip(row['genes'],bounds)) if not lo<=v<=hi]
        assert exceptions==[89] and row['genes'][89]==float(base[1][89])==.175 # Existing native RS-v2 apex exception.
        for rel,want in row['sha256'].items():assert digest(ROOT/rel)==want,rel
        geometry=read(OUT/f'{name}-geometry.json');native.verify_provenance(geometry)
        assert geometry['input_was_canonical'] and geometry['within_genome_domain'] and geometry['rs_v2']
        assert row['input_was_canonical'] and not row['domain_projection_differences'] and row['genes']==row['observed_genes']
        assert row['geometry_legal']==bool(geometry['input_was_canonical'] and geometry['within_genome_domain'] and geometry['rs_v2'] and geometry['aes_pass'] and not geometry['rejects'])
        assert row['geometry_legal'],'every prescribed cell must be retained; unexpected rejects need explicit review'
        full=read(OUT/f'{name}-fem.json');native.verify_provenance(full)
        assert full['solver']=='faer' and full['cpu_affinity'].split(':')[1].strip()=='12-13'
        assert full['nice_priority']>=10 and full['cargo_build_jobs']=='2'
        assert full['genes']==row['genes'] and full['config']==row['config'] and full['levels']==row['levels']
        assert [v['area'] for v in row['levels']]==[4.,1.,.25,.125,.0625]
        canonical=list(csv.reader((OUT/f'{name}-fem.canonical.csv').open()))[1]
        assert [float(x) for x in canonical[:128]]==row['genes'] and canonical[128]==data[1][128]
        def passed(levels):return all(all(l['metrics'][k]>=floor for k,floor in r['thresholds'].items()) and l['metrics']['seq_ratio']>=(1.25 if l['area']==.0625 else 1.20) for l in levels)
        assert row['fine_targets_pass']==passed(row['levels'][2:]) and row['all_five_targets_pass']==passed(row['levels'])
        for k in row['certification']:
            assert row['certification'][k]==min(l['metrics'][k] for l in row['levels'][2:4])
            assert row['strict'][k]==row['levels'][4]['metrics'][k]
        denominator=[l['metrics']['lat_rob']/l['metrics']['fat_sf'] for l in row['levels']]
        assert max(denominator)-min(denominator)<1e-10
        gci=read(OUT/f'{name}-gci.json');native.verify_provenance(gci)
        assert gci['task']=='G16-06-RSV2-08-p7' and gci['plate_solver']=='faer'
        assert gci['cpu_affinity'].split(':')[1].strip()=='12-13' and gci['cargo_build_jobs']=='2'
        assert gci['plate_gci']==row['gci'] and gci['G16-15']==row['gci_admitted']
        assert gci['native_gci_source_sha256']==digest(ROOT/'crates/crescent/src/gci.rs')
        for l in row['gci']['levels_fine_to_coarse']:
            m=next(v['metrics'] for v in row['levels'] if v['area']==l['max_area_mm2'])
            assert m['seq_ratio']==l['seq_ratio'] and m['sf_hunter']==l['sf_hunter']
        field=read(OUT/f'{name}-field.json')
        assert field['native_fem_quadrature'] and field['law_sha256']==digest(HERE/'phase7_adapter.rs')
        assert field['genome_sha256']==row['genome_sha256']
        for l,g in zip(field['levels'],row['gci']['levels_fine_to_coarse']):
            assert l['area']==g['max_area_mm2'] and l['mesh_thickness_sha256']==g['mesh_sha256']
            assert l['n_node']==g['n_node'] and l['n_elem']==g['n_elem']
        if not name.startswith('law_'):
            groups[tuple(row['genes'][i] for i in [42,100,101,102,103])].append((row,field));observations+=5
            assert row['genes'][46]==2.6 and row['genes'][47]==115. and row['genes'][101]==.85 and row['genes'][103]==.65
        if row['export_pass'] is not None:
            exported+=1;export=read(OUT/f'{name}-export.json');native.verify_provenance(export)
            assert export['genes']==row['genes'] and export['config']==row['config']
            m,s=export['measurement'],export['solid']
            assert row['export_pass']==bool(m['sharpness']['pass'] and m['point']['pass'] and m['bore_edge_radius_mm']>=1.5 and s['census']==[0,0,1] and s['volume_mm3']>0)
            side=read(OUT/f'{name}-a4.json');native.verify_provenance(side)
            assert side['cpu_affinity'].split(':')[1].strip()=='12-13' and side['cargo_build_jobs']=='2'
            assert export['cpu_affinity'].split(':')[1].strip()=='12-13' and export['nice_priority']>=10
            assert side['a4']==row['a4'] and side['G16-13']==row['side_a4_pass']
            mesh=read(OUT/f'{name}-a4.mesh.json')
            binary=b''.join(struct.pack('<dd',*q) for q in mesh['xy'])+b''.join(struct.pack('<QQQ',*t) for t in mesh['tris'])+b''.join(struct.pack('<d',v) for v in mesh['te'])
            assert hashlib.sha256(binary).hexdigest()==row['gci']['levels_fine_to_coarse'][0]['mesh_sha256']
            assert digest(OUT/f'{name}-a4.mesh.json')==row['a4']['mesh_sha256']
            assert row['a4']['genome_sha256']==row['genome_sha256']
            assert row['a4']['same_mesh_ring_clamp_seq_ratio']==row['levels'][2]['metrics']['seq_ratio']
            assert row['a4']['certified_seq_ratio']==row['certification']['seq_ratio']
        assert row['scoped_joint_pass']==bool(row['geometry_legal'] and row['all_five_targets_pass'] and row['gci_admitted'] and row['export_pass'] and row['side_a4_pass'])
        if not name.startswith('law_'):
            assert row['assignment_scoped_joint_pass']==bool(row['geometry_legal'] and row['fine_targets_pass'] and row['gci_admitted'] and row['export_pass'] and row['side_a4_pass'])
            for k in r['outcome_counts']:counts[k]+=row.get(k) is True
        assert row['all_gate_qualification'] is False
    assert observations==r['mesh_observations']==720 and exported==len(r['selection'])==10
    assert dict(counts)==r['outcome_counts']
    assert len({tuple(v['three_level_mesh_hashes']) for v in r['selection']})==10
    def margin(v):return min(l['metrics'][k]/floor for l in v['levels'][2:] for k,floor in {**native.LIMITS,'seq_ratio':1.25 if l['area']==.0625 else 1.20}.items())
    ranked=sorted(r['results'],key=lambda v:(-v['fine_targets_pass'],-margin(v),-v['gci_admitted'],v['name']))
    picked=[];seen=set()
    for row in ranked:
        key=tuple(l['mesh_sha256'] for l in row['gci']['levels_fine_to_coarse'])
        if key in seen:continue
        seen.add(key);picked.append(dict(name=row['name'],rank=len(picked)+1,margin=margin(row),three_level_mesh_hashes=list(key)))
        if len(picked)==10:break
    assert picked==r['selection']
    assert len(groups)==36
    for key,values in groups.items():
        assert sorted(v['genes'][74] for v,f in values)==[0.,.45,.75,1.05]
        for i in range(3):
            assert len({f['levels'][i]['xy_triangles_sha256'] for v,f in values})==1
            assert len({f['levels'][i]['thickness_sha256'] for v,f in values})==4
    controls=r['law_effect_controls'];assert len(controls)==3
    a,b,c=controls
    assert [(v['genes'][46],v['genes'][47],v['genes'][74]) for v in controls]==[(2.6,115.,.75),(3.5,115.,.75),(2.6,130.,.75)]
    assert [i for i,(x,y) in enumerate(zip(a['genes'],b['genes'])) if x!=y]==[46]
    assert [i for i,(x,y) in enumerate(zip(a['genes'],c['genes'])) if x!=y]==[47]
    for i in range(3):
        fields=[read(OUT/f'{v["name"]}-field.json')['levels'][i] for v in controls]
        assert len({f['xy_triangles_sha256'] for f in fields})==1 and len({f['thickness_sha256'] for f in fields})==3
    return dict(status='pass',cases=144,sweep_mesh_observations=720,taper_control_mesh_observations=15,
        fuller_groups_with_four_distinct_thickness_hashes_on_each_GCI_mesh=36,individually_active_taper_genes=[46,47],
        selected_export_A4_cases=10,outcome_counts=r['outcome_counts'],production_hashes=77,
        native_FEM_only_two_observation_hooks=True,native_quadrature_and_GCI_A4_mesh_binding=True,
        exact_thresholds_verified=True,all_CPs_byte_preserved=True,read_only_audit=True,allowed_audit_cpu_pairs=[[12,13],[8,9]],all_gate_qualification=False)

if __name__=='__main__':print(json.dumps(audit(),sort_keys=True))
