#!/usr/bin/env python3
"""Audit native-domain exclusions, ring legality, five-mesh evidence and export/A4."""
import csv
import hashlib
import json
import os
import struct
from collections import defaultdict
from prepare_phase5 import HERE, ROOT, digest
import run_phase5 as native
from run_phase6 import OUT, P5, ALLOWED

if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10
    assert os.environ['CARGO_BUILD_JOBS']=='2'
    receipt=HERE/'phase6_receipt.json';r=json.loads(receipt.read_text())
    assert r['state']=='complete' and not r['errors']
    assert r['cpus']==[12,13] and r['cargo_build_jobs']==2 and r['nice']>=10 and r['solver']=='faer'
    assert r['thresholds']=={'hm_spine_sf':1.,'lat_rob':358.,'fat_sf':1.,'sf_hunter':1.377}
    assert r['cert_sequence_floor']==1.20 and r['strict_sequence_floor']==1.25
    for rel,want in r['sha256'].items():assert digest(ROOT/rel)==want,rel
    assert digest(OUT/'main')==digest(P5/'main')
    assert (OUT/'source_provenance.json').read_bytes()==(P5/'source_provenance.json').read_bytes()
    prov=json.loads((OUT/'source_provenance.json').read_text())
    for group in ['production_sha256','local_source_sha256']:
        for rel,want in prov[group].items():assert digest(ROOT/rel)==want,rel
    for value in prov['inputs'].values():assert digest(ROOT/value['snapshot'])==value['sha256']
    fem=(HERE/'phase5_sources/prototype_fem.rs').read_text()
    for hook in ['    crate::diagnostic::record(&t6, nod_sp.as_deref(), nod_rb.as_deref(), tp, sx0);\n',
                 '    crate::diagnostic::record_sequence(&cen,&excl,&lat,&twist,p.heel.0,&te,&t6,&nod_pr,&ring6,tp,p);\n']:
        assert fem.count(hook)==1;fem=fem.replace(hook,'')
    assert fem==(ROOT/'crates/crescent/src/fem.rs').read_text()
    assert '2 passed; 0 failed' in (OUT/'gci-tests.log').read_text()
    assert '1 passed; 0 failed' in (OUT/'requirements-test.log').read_text()
    base=list(csv.reader((HERE/'phase5_sources/q7.csv').open()))
    controls=json.loads((OUT/'geometry_controls.json').read_text());bounds=controls['bounds']
    assert controls['state']=='complete' and controls['all_four_ring_levels_legal']
    assert [v['ring_wall'] for v in controls['selected']]==[6.4,6.65,6.85,7.]
    for control in controls['selected']:
        assert control['legal'] and digest(ROOT/control['genome'])==control['genome_sha256']
        assert digest(ROOT/control['native_output'])==control['native_output_sha256']
        doc=json.loads((ROOT/control['native_output']).read_text());native.verify_provenance(doc)
        assert not doc['rejects'] and doc['aes_pass'] and doc['input_was_canonical']
        assert doc['inside_concave']>=.97 and doc['holder_fit'][0] and 60<=doc['grip_len_mm']<=80
        assert doc['genes'][6:9]==[float(x) for x in base[1][6:9]] and doc['genes'][19:22]==[0.,0.,0.]
        assert doc['bounds']==bounds
    matrix=json.loads((OUT/'nominal_matrix.json').read_text())
    assert len(matrix)==r['nominal_cases']==1152
    for item in matrix:
        violations=[dict(index=int(i),name=base[0][int(i)],requested=v,bounds=bounds[int(i)]) for i,v in item['changes'].items() if not bounds[int(i)][0]<=v<=bounds[int(i)][1]]
        assert violations==item['violations'] and item['domain_legal']==(not violations)
    assert sum(not v['domain_legal'] for v in matrix)==r['domain_rejected_cases']==1080
    assert {v['name'] for v in matrix if v['domain_legal']}=={v['name'] for v in r['results']}
    assert r['canonical_cases']==r['completed_cases']==72
    observations=0;legal=0;exported=0;groups=defaultdict(list)
    for row in r['results']+r['law_effect_controls']:
        name=row['name'];path=ROOT/row['genome'];data=list(csv.reader(path.open()))
        assert data[0]==base[0] and len(data[1])==129 and data[1][128]==base[1][128]
        assert data[1][6:9]==base[1][6:9] and data[1][19:22]==base[1][19:22] and data[1][64]==base[1][64]
        changed=[i for i,(a,b) in enumerate(zip(base[1],data[1])) if a!=b]
        assert set(changed)<=ALLOWED and changed==row['changed_indices']
        assert [float(v) for v in data[1][:128]]==row['genes']
        # Native clamp has an existing RS-v2 EDGE_TH exception to the generic
        # legacy table. All varied genes obey the native table; the complete
        # input must independently pass native canonicalization and rs_v2.
        assert all(bounds[i][0]<=row['genes'][i]<=bounds[i][1] for i in ALLOWED)
        exceptions=[i for i,(value,(lo,hi)) in enumerate(zip(row['genes'],bounds)) if not lo<=value<=hi]
        assert exceptions==[89] and row['genes'][89]==float(base[1][89])==.175
        for rel,want in row['sha256'].items():assert digest(ROOT/rel)==want,rel
        geometry=json.loads((OUT/f'{name}-geometry.json').read_text());native.verify_provenance(geometry)
        assert geometry['input_was_canonical'] and geometry['within_genome_domain'] and geometry['rs_v2']
        assert row['input_was_canonical'] and not row['domain_projection_differences'] and row['genes']==row['observed_genes']
        assert row['geometry_legal']==bool(geometry['input_was_canonical'] and geometry['within_genome_domain'] and geometry['rs_v2'] and geometry['aes_pass'] and not geometry['rejects'])
        if not row['geometry_legal']:
            assert not row['levels'] and not row['scoped_joint_pass'];continue
        full=json.loads((OUT/f'{name}-fem.json').read_text());native.verify_provenance(full)
        assert full['solver']=='faer' and full['cpu_affinity'].split(':')[1].strip()=='12-13'
        assert full['nice_priority']>=10 and full['cargo_build_jobs']=='2'
        assert full['genes']==row['genes'] and full['config']==row['config']
        assert full['levels']==row['levels'] and [v['area'] for v in row['levels']]==[4.,1.,.25,.125,.0625]
        canonical=list(csv.reader((OUT/f'{name}-fem.canonical.csv').open()))[1]
        assert [float(x) for x in canonical[:128]]==row['genes'] and canonical[128]==data[1][128]
        def passed(levels):
            return all(all(l['metrics'][k]>=floor for k,floor in r['thresholds'].items()) and
                       l['metrics']['seq_ratio']>=(1.25 if l['area']==.0625 else 1.20) for l in levels)
        assert row['fine_targets_pass']==passed(row['levels'][2:])
        assert row['all_five_targets_pass']==passed(row['levels'])
        for k in row['certification']:
            assert row['certification'][k]==min(l['metrics'][k] for l in row['levels'][2:4])
            assert row['strict'][k]==row['levels'][4]['metrics'][k]
        denominator=[l['metrics']['lat_rob']/l['metrics']['fat_sf'] for l in row['levels']]
        assert max(denominator)-min(denominator)<1e-10
        gci=json.loads((OUT/f'{name}-gci.json').read_text());native.verify_provenance(gci)
        assert gci['task']=='G16-06-RSV2-08-p6' and gci['plate_solver']=='faer'
        assert gci['plate_gci']==row['gci'] and gci['G16-15']==row['gci_admitted']
        assert gci['native_gci_source_sha256']==digest(ROOT/'crates/crescent/src/gci.rs')
        for l in row['gci']['levels_fine_to_coarse']:
            m=next(v['metrics'] for v in row['levels'] if v['area']==l['max_area_mm2'])
            assert m['seq_ratio']==l['seq_ratio'] and m['sf_hunter']==l['sf_hunter']
        if not name.startswith('law_'):
            legal+=1;observations+=5
            groups[(row['genes'][42],row['genes'][100],row['genes'][101],row['genes'][102],row['genes'][103])].append(row)
        if row['export_pass'] is not None:
            exported+=1
            export=json.loads((OUT/f'{name}-export.json').read_text());native.verify_provenance(export)
            assert export['genes']==row['genes'] and export['config']==row['config']
            m=export['measurement'];s=export['solid']
            assert row['export_pass']==bool(m['sharpness']['pass'] and m['point']['pass'] and m['bore_edge_radius_mm']>=1.5 and s['census']==[0,0,1] and s['volume_mm3']>0)
            side=json.loads((OUT/f'{name}-a4.json').read_text());native.verify_provenance(side)
            assert side['a4']==row['a4'] and side['G16-13']==row['side_a4_pass']
            mesh=json.loads((OUT/f'{name}-a4.mesh.json').read_text())
            binary=(b''.join(struct.pack('<dd',*q) for q in mesh['xy'])+b''.join(struct.pack('<QQQ',*t) for t in mesh['tris'])+b''.join(struct.pack('<d',v) for v in mesh['te']))
            assert hashlib.sha256(binary).hexdigest()==row['gci']['levels_fine_to_coarse'][0]['mesh_sha256']
            assert row['a4']['same_mesh_ring_clamp_seq_ratio']==row['levels'][2]['metrics']['seq_ratio']
            assert row['a4']['certified_seq_ratio']==row['certification']['seq_ratio']
        assert row['scoped_joint_pass']==bool(row['geometry_legal'] and row['all_five_targets_pass'] and row['gci_admitted'] and row['export_pass'] and row['side_a4_pass'])
        row['assignment_scoped_joint_pass']=bool(row['geometry_legal'] and row['fine_targets_pass'] and row['gci_admitted'] and row['export_pass'] and row['side_a4_pass'])
        assert row['all_gate_qualification'] is False
    assert observations==r['mesh_observations'] and exported==len(r['selection'])==10
    assert len({tuple(v['three_level_mesh_hashes']) for v in r['selection']})==10
    fuller_effects=[]
    for key,rows in groups.items():
        shas={tuple(l['mesh_sha256'] for l in row['gci']['levels_fine_to_coarse']) for row in rows}
        relative=[]
        for row in rows[1:]:
            for a,b in zip(rows[0]['levels'],row['levels']):
                for k,want in a['metrics'].items():
                    if isinstance(want,(int,float)) and isinstance(b['metrics'].get(k),(int,float)):
                        relative.append(abs(want-b['metrics'][k])/max(abs(want),1e-300))
        fuller_effects.append(dict(ring_window=key,cases=len(rows),native_GCI_mesh_thickness_identical=len(shas)==1,
                                  max_relative_five_mesh_scalar_error=max(relative,default=0.)))
    comparison=json.loads((OUT/'law_effect_comparison.json').read_text())
    a,b=r['law_effect_controls'];assert len(r['law_effect_controls'])==2
    assert [(v['genes'][46],v['genes'][47],v['genes'][74]) for v in [a,b]]==[(2.6,115.,0.),(3.5,130.,.9)]
    assert [l['mesh_sha256'] for l in a['gci']['levels_fine_to_coarse']]==[l['mesh_sha256'] for l in b['gci']['levels_fine_to_coarse']]
    errors=[]
    for left,right in zip(a['levels'],b['levels']):
        for k,want in left['metrics'].items():
            if isinstance(want,(int,float)) and isinstance(right['metrics'].get(k),(int,float)):
                errors.append(abs(right['metrics'][k]-want)/max(abs(want),1e-300))
    assert max(errors)==comparison['max_relative_scalar_error'] and max(errors)<=1e-9
    assert len(groups)==24 and all(len(v)==3 for v in groups.values())
    r['construction_effects']=dict(fuller_groups=24,
        groups_with_identical_native_GCI_mesh_thickness=sum(v['native_GCI_mesh_thickness_identical'] for v in fuller_effects),
        max_relative_fuller_five_mesh_scalar_error=max(v['max_relative_five_mesh_scalar_error'] for v in fuller_effects),
        matched_inbound_taper_tip_fuller_control=comparison)
    assignment_passes=sum(v.get('assignment_scoped_joint_pass',False) for v in r['results'])
    r['outcome_counts']['assignment_scoped_joint_pass']=assignment_passes
    r['acceptance_scope_note']='Phase-6 assignment cert/strict+GCI/export/A4 is audited separately from the retained stricter all-five numerical comparison; neither changes a native gate'
    audit=dict(status='pass',nominal_cells=1152,native_domain_rejected=1080,canonical_cases=72,
               preflight_legal_ring_levels=4,legal_five_mesh_cases=legal,requested_mesh_observations=observations,
               law_effect_control_mesh_observations=10,selected_actual_exports_and_a4=10,
               native_requirement_tests_passed=1,native_gci_tests_passed=2,source_hashes=True,
               parent_canonical_and_material_identity=True,hand_and_pinned_CPs_preserved=True,
               native_FEM_only_two_observation_hooks=True,export_FEM_input_identity=True,GCI_A4_mesh_identity=True,
               unrounded_native_thresholds_verified=True,preexisting_native_RSv2_apex_canonicalization_verified=True,
               assignment_scoped_joint_passes=assignment_passes,
               fuller_effect_groups=fuller_effects,law_effect_comparison=comparison)
    ap=OUT/'integrity_audit.json';ap.write_text(json.dumps(audit,indent=2)+'\n');r['integrity_audit']=audit
    for p in [ap,HERE/'audit_phase6.py',HERE/'build_phase6.sh',OUT/'gci-tests.log',OUT/'requirements-test.log',OUT/'law_effect_comparison.json',
              OUT/'audit_repair.json',OUT/'audit_phase6-attempt1.py',OUT/'audit-attempt1.log']:
        r['sha256'][str(p.relative_to(ROOT))]=digest(p)
    receipt.write_text(json.dumps(r,indent=2)+'\n')
    report=HERE/'PHASE6.md';marker='\n## Verified measurements\n'
    lines=[marker,f'Audit PASS: canonical cases72; legal five-mesh cases{legal}; requested mesh observations{observations}; law-effect control observations10; selected export/A4 cases10. Native GCI tests2/2 and requirements consistency1/1 pass. All source/input/mesh/threshold checks pass.',
           '',f'Phase-6 assignment scope (certification/strict numerical floors + geometry/native GCI/export/A4) passes: {assignment_passes}. The original runner also retains the stricter all-five numerical comparison as a separate diagnostic. Native gates and unrounded thresholds are unchanged in either view.',
           '',f'Matched law-effect control: {comparison}.', '',f'Fuller equivalence groups: {fuller_effects}.','',
           '| Case | Area mm² | Spine SF | Lateral N | Fatigue SF | Hunter SF | Sequence | Spine peak x,y |',
           '| --- | --- | --- | --- | --- | --- | --- | --- |']
    for row in r['results']:
        for l in row['levels']:
            m=l['metrics'];peak=l['diagnostic_peaks']['spine']['at']
            lines.append(f'| {row["name"]} | {l["area"]} | {m["hm_spine_sf"]:.9f} | {m["lat_rob"]:.9f} | {m["fat_sf"]:.9f} | {m["sf_hunter"]:.9f} | {m["seq_ratio"]:.9f} | {peak} |')
    lines+=['','| Export/A4 case | GCI fit / p | Half-angle ° | Apex mm | Point | Bore mm | Census | A4 eigenvalue | Supported / cert seq |',
            '| --- | --- | --- | --- | --- | --- | --- | --- | --- |']
    for row in r['results']:
        if row['export_pass'] is None:continue
        g,s,a=row['gci']['seq_ratio_gci'],row['sharpness'],row['a4']
        lines.append(f'| {row["name"]} | {g["fit_status"]} / {g["p"]} | {s["max_half_angle_deg"]:.9f} | {s["max_apex_mm"]:.9f} | {row["point"]["pass"]} | {row["bore_edge_radius_mm"]:.9f} | {row["solid"]["census"]} | {a["buckling_eigenvalue"]:.9f} | {a["supported_seq_ratio"]:.9f} / {a["certified_seq_ratio"]:.9f} |')
    report.write_text(report.read_text().split(marker)[0]+'\n'.join(lines)+'\n')
    print(json.dumps({k:v for k,v in audit.items() if k!='fuller_effect_groups'}))
