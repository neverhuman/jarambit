#!/usr/bin/env python3
"""Authorized Q7/native taper/fullers: finite 144-case five-mesh study."""
from concurrent.futures import ThreadPoolExecutor, as_completed
import csv
from datetime import datetime, timezone
import itertools
import json
import os
import run_phase5 as native
from prepare_phase5 import HERE, ROOT, digest
OUT=ROOT/'target/G16-06-RSV2-08/phase7'
ALLOWED={40,41,42,46,47,74,100,101,102,103}
ROWS=[];ERRORS=[];SELECTION=[];LAW_EFFECT=[]

def milestone(title,detail):
    stamp=datetime.now(timezone.utc).isoformat(timespec='seconds')
    for rel in ['.agent/CHANGELOG.md','.agent/handoffs/G16-06-RSV2-08.md']:
        with (ROOT/rel).open('a') as f:f.write(f'\n## {stamp} — Phase-7 {title}\n{detail}\n')

def publish(state):
    counts={k:sum(v.get(k) is True for v in ROWS) for k in ['geometry_legal','gci_admitted','fine_targets_pass','all_five_targets_pass','export_pass','side_a4_pass','scoped_joint_pass','assignment_scoped_joint_pass']}
    paths=[HERE/p for p in ['run_phase7.py','prepare_phase7.py','build_phase7.sh','phase7_adapter.rs','phase7_native.rs','phase7_geometry.py','phase5_sources/q7.csv','phase5_sources/q7_control_case.json']]
    paths += [OUT/p for p in ['main','native_adapter','source_provenance.json','geometry_controls.json','nominal_matrix.json']]
    for p in [HERE/'audit_phase7.py',OUT/'law_effect_comparison.json']:
        if p.exists():paths.append(p)
    r=dict(task='G16-06-RSV2-08-p7',state=state,cpus=[12,13],nice=os.getpriority(os.PRIO_PROCESS,0),cargo_build_jobs=2,solver='faer',max_concurrent_evaluations=2,
        areas_mm2=native.AREAS,thresholds=native.LIMITS,cert_sequence_floor=1.20,strict_sequence_floor=1.25,
        nominal_cases=144,canonical_cases=len(JOBS),completed_cases=len(ROWS),mesh_observations=sum(len(v['levels']) for v in ROWS),
        grid=dict(ring_wall=[6.2,6.4,6.6,6.8],fuller_depth=[0.,.45,.75,1.05],window_width=[3.,4.5,6.],window_start=[.05,.25,.45]),
        fixed=dict(taper_start=115.,tip_thickness=2.6,window_end=.85,window_fill=.65),ring_controls=CONTROLS['selected'],
        construction='Phase7 native distal taper + verbatim native fuller centreline/two-face subtraction on Q7 core, with Q7 grind protection and native web floor; common FEM/export law',
        native_reported_mass_note='p.mass_g is inherited native pheno field mass, not integrated Phase7 export mass; actual selected solid volumes are separately measured',
        results=ROWS,law_effect_controls=LAW_EFFECT,selection=SELECTION,errors=ERRORS,outcome_counts=counts,
        acceptance_scope_note='Assignment-scoped joint requires fine cert/strict numerical floors + native geometry/GCI/export/A4; stricter all-five comparison retained separately. No native gate is changed.',
        all_gate_qualification=False,unclosed=['independent 3D/cross-solver qualification','human-max twist','inherited ring collar/flat-stock constraint'],
        sha256={str(p.relative_to(ROOT)):digest(p) for p in paths})
    (HERE/'phase7_receipt.json').write_text(json.dumps(r,indent=2)+'\n')
    lines=['# G16-06-RSV2-08 Phase 7','',f'Status: {state}; {len(ROWS)}/144 cases; {r["mesh_observations"]} sweep mesh observations. Counts: {counts}.','',
        'The authorized section adapter applies native distal taper and the native fuller centreline/two-face parabolic groove directly to the Q7 core. Q7 edge/point guards and native web floor, brand/holder protection and bore collar remain. Fullers use Q7 grind protection rather than the native broad grind that the Q7 section replaces. FEM, certification and solid export share this section implementation. Production FEM operators, material, loads, masks, supports, native GCI, gates and all thresholds are unchanged.', '',
        'Grid: ring wall6.20/6.40/6.60/6.80 × fuller depth0/.45/.75/1.05 × window width3/4.5/6 × window start.05/.25/.45 =144 exact canonical inputs. Fixed taper115/tip2.6, window end.85/fill.65. Independent one-gene tip3.5 and taper130 controls establish activation. All CPs and spine offset42 remain byte-preserved; bounded native GRIP_DX/DROP ring controls are preflighted. Every geometry reject remains recorded and cannot qualify.', '',
        'Five unbiased meshes4/1/.25/.125/.0625 and unchanged native three-level GCI are evaluated for each native-legal case. Certification is min(.25,.125), sequence>=1.20; strict.0625 sequence>=1.25. Spine>=1, lateral>=358N, fatigue>=1, hunter>=1.377 remain unchanged. The all-five comparison is separately retained. Ten distinct leading configurations receive actual export and native A4. Unselected export/A4=null; no joint passage is inferred.', '',
        'Full RS-v2/manufacturing qualification remains false. GCI admission is an estimate, not asymptotic validation. No finite negative result establishes global infeasibility or an optimum. Native p.mass_g reflects its own field; selected Phase7 solid volumes are the actual constructed-section volume evidence.', '',
        '| Case | Legal | Cert spine | Strict spine | Cert seq | Strict seq | GCI | Fine floors | All-five floors | Export | A4 | Assignment joint |',
        '| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |']
    for v in ROWS:
        c,t=v.get('certification',{}),v.get('strict',{})
        values=[v['name'],v['geometry_legal'],c.get('hm_spine_sf'),t.get('hm_spine_sf'),c.get('seq_ratio'),t.get('seq_ratio'),v['gci_admitted'],v.get('fine_targets_pass'),v.get('all_five_targets_pass'),v['export_pass'],v['side_a4_pass'],v.get('assignment_scoped_joint_pass',False)]
        lines.append('| '+' | '.join(f'{x:.9f}' if isinstance(x,float) else str(x) for x in values)+' |')
    lines+=['',f'Effect controls: {[v["name"] for v in LAW_EFFECT]}. Export/A4 selection: {SELECTION}.', '',
        'Execution: `nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/run_phase7.py`. Effective niceness is recorded (host inherits nice10, so prefix reaches19). Audit is read-only, accepts CPUs12,13 or8,9 and creates no cache files. External archive destination is outside session writable roots; local delivery files await Captain relay. No PR/push/stacking/deletes.','']
    (HERE/'PHASE7.md').write_text('\n'.join(lines))

def configure():
    native.OUT=OUT
    native.BASE=list(csv.reader((HERE/'phase5_sources/q7.csv').open()))
    native.ALLOWED=ALLOWED

def create(path,changes):
    data=[list(v) for v in native.BASE]
    for i,v in changes.items():
        assert i in ALLOWED and CONTROLS['bounds'][i][0]<=v<=CONTROLS['bounds'][i][1]
        if v!=float(data[1][i]):data[1][i]=str(v)
    assert data[1][:40]==native.BASE[1][:40]
    with path.open('w') as f:csv.writer(f,lineterminator='\n').writerows(data)
    return path

if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10
    assert os.environ['CARGO_BUILD_JOBS']=='2'
    configure();CONTROLS=json.loads((OUT/'geometry_controls.json').read_text())
    assert CONTROLS['state']=='complete' and CONTROLS['all_four_ring_levels_legal']
    inputs=HERE/'phase7_candidates';inputs.mkdir();MATRIX=[];JOBS=[]
    for control,fd,w,s0 in itertools.product(CONTROLS['selected'],[0.,.45,.75,1.05],[3.,4.5,6.],[.05,.25,.45]):
        name=f'r{control["ring_wall"]:.2f}_f{fd:.2f}_w{w:.1f}_s{s0:.2f}'
        changes={40:control['drop'],41:control['grip_dx'],42:control['ring_wall'],46:2.6,47:115.,74:fd,100:s0,101:.85,102:w,103:.65}
        path=create(inputs/f'{name}.csv',changes)
        MATRIX.append(dict(name=name,changes=changes,domain_legal=True,genome=str(path.relative_to(ROOT)),genome_sha256=digest(path)))
        JOBS.append((name,path))
    assert len(JOBS)==144
    (OUT/'nominal_matrix.json').write_text(json.dumps(MATRIX,indent=2)+'\n')
    publish('law_effect_controls')
    control_dir=OUT/'law_effect_controls';control_dir.mkdir()
    for name,changes in [('law_baseline',{74:.75}),('law_tip_only',{74:.75,46:3.5}),('law_taper_only',{74:.75,47:130.})]:
        path=create(control_dir/f'{name}.csv',changes)
        LAW_EFFECT.append(native.evaluate((name,path)));publish('law_effect_controls')
        print('law control',name,flush=True)
    hashes=[tuple(l['mesh_sha256'] for l in row['gci']['levels_fine_to_coarse']) for row in LAW_EFFECT]
    assert all(v['geometry_legal'] for v in LAW_EFFECT) and len(set(hashes))==3,'both taper genes must affect actual native plate mesh/thickness'
    (OUT/'law_effect_comparison.json').write_text(json.dumps(dict(tip_only_changes=[46],taper_only_changes=[47],native_GCI_mesh_thickness_hashes=hashes,each_taper_gene_active=True),indent=2)+'\n')
    publish('five_mesh_sweep')
    milestone('milestone 2: legal ring controls and active taper controls; sweep started',f'- Four rings preflight legal with all CPs preserved; 144 exact canonical inputs. Three matched five-mesh/native-GCI controls independently prove T_TIP and TAPER_X0 alter mesh/thickness hashes. Five requested meshes and native GCI will run for all legal inputs. All native thresholds unchanged.')
    with ThreadPoolExecutor(max_workers=2) as pool:
        futures={pool.submit(native.evaluate,j):j[0] for j in JOBS}
        for f in as_completed(futures):
            name=futures[f]
            try:
                row=f.result();row['assignment_scoped_joint_pass']=False;ROWS.append(row)
                milestone(f'case {name}',f'- Geometry={row["geometry_legal"]}; GCI={row["gci_admitted"]}; fine={row.get("fine_targets_pass")}; all-five={row.get("all_five_targets_pass")}; certification={row.get("certification")}; strict={row.get("strict")}. Full qualification=false.')
            except Exception as e:ERRORS.append(dict(name=name,error=repr(e)))
            publish('five_mesh_sweep');print('case',name,'completed',len(ROWS),'errors',ERRORS,flush=True)
    assert len(ROWS)==144 and not ERRORS,ERRORS
    groups={}
    for row in ROWS:
        if row['geometry_legal']:
            key=tuple(row['genes'][i] for i in [42,100,101,102,103])
            groups.setdefault(key,[]).append(row)
    effects=[]
    for key,rows in groups.items():
        unique=len({tuple(l['mesh_sha256'] for l in v['gci']['levels_fine_to_coarse']) for v in rows})
        effects.append(dict(ring_window=key,fuller_depths=sorted(v['genes'][74] for v in rows),distinct_mesh_thickness_hash_sets=unique))
        assert len(rows)==unique==4,'never claim unmasked fuller if actual plate hashes coincide'
    assert len(groups)==36
    (OUT/'fuller_effects.json').write_text(json.dumps(effects,indent=2)+'\n')
    def margin(v):return min(l['metrics'][k]/floor for l in v['levels'][2:] for k,floor in {**native.LIMITS,'seq_ratio':1.25 if l['area']==.0625 else 1.20}.items())
    ranked=sorted([v for v in ROWS if v['geometry_legal']],key=lambda v:(-v['fine_targets_pass'],-margin(v),-v['gci_admitted'],v['name']))
    seen=set()
    for v in ranked:
        key=tuple(l['mesh_sha256'] for l in v['gci']['levels_fine_to_coarse'])
        if key in seen:continue
        seen.add(key);SELECTION.append(dict(name=v['name'],rank=len(SELECTION)+1,margin=margin(v),three_level_mesh_hashes=key))
        if len(SELECTION)==10:break
    assert len(SELECTION)==10
    (OUT/'export_selection.json').write_text(json.dumps(SELECTION,indent=2)+'\n');publish('export_a4')
    milestone('milestone 3: all cases measured; actual export/A4 started',f'- 144 cases retained, mesh observations={sum(len(v["levels"]) for v in ROWS)}; 36/36 matched fuller groups have four distinct native GCI mesh/thickness hash sets. Ten distinct target-margin leaders selected for actual export/native A4; no threshold changes.')
    for selected in SELECTION:
        row=next(v for v in ROWS if v['name']==selected['name']);genome=ROOT/row['genome']
        ep,export=native.invoke('main','export',row['name'],genome);native.verify_provenance(export)
        assert export['genes']==row['genes'] and export['config']==row['config']
        m,s=export['measurement'],export['solid']
        row.update(sharpness=m['sharpness'],point={k:v for k,v in m['point'].items() if k!='samples'},bore_edge_radius_mm=m['bore_edge_radius_mm'],solid=s,worst_edge_face=export['worst_edge_face'])
        row['export_pass']=bool(m['sharpness']['pass'] and m['point']['pass'] and m['bore_edge_radius_mm']>=1.5 and s['census']==[0,0,1] and s['volume_mm3']>0)
        ap,side=native.invoke('native_adapter','a4',row['name'],genome,row['certification']['seq_ratio']);native.verify_provenance(side)
        row['a4']=side['a4'];row['side_a4_pass']=side['G16-13']
        assert row['a4']['same_mesh_ring_clamp_seq_ratio']==row['levels'][2]['metrics']['seq_ratio']
        row['scoped_joint_pass']=bool(row['geometry_legal'] and row['all_five_targets_pass'] and row['gci_admitted'] and row['export_pass'] and row['side_a4_pass'])
        row['assignment_scoped_joint_pass']=bool(row['geometry_legal'] and row['fine_targets_pass'] and row['gci_admitted'] and row['export_pass'] and row['side_a4_pass'])
        row['coverage']='five_meshes_native_gci_actual_export_a4'
        for p in [ep,ap,ap.with_suffix('.mesh.json')]:row['sha256'][str(p.relative_to(ROOT))]=digest(p)
        publish('export_a4');milestone(f'export/A4 {row["name"]}',f'- Export={row["export_pass"]}; A4={row["side_a4_pass"]}; assignment joint={row["assignment_scoped_joint_pass"]}; all-five joint={row["scoped_joint_pass"]}. Full qualification=false.');print('export_a4',row['name'],flush=True)
    publish('complete');milestone('milestone 4: finite study evaluated',f'- Cases144; five-mesh observations={sum(len(v["levels"]) for v in ROWS)} plus15 matched-control observations; export/A4 cases10. Assignment joint passes={[v["name"] for v in ROWS if v["assignment_scoped_joint_pass"]]}. Final read-only audit follows; full qualification=false.')
