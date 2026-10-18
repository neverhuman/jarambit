#!/usr/bin/env python3
"""Two best parental ring profiles x two taper starts x three tip thicknesses."""
from concurrent.futures import ThreadPoolExecutor,as_completed
from datetime import datetime,timezone
import csv,itertools,json,os
import run_phase5 as native
from prepare_phase5 import HERE,ROOT,digest
OUT=ROOT/'target/G16-06-RSV2-08/phase8'
ALLOWED={40,41,42,46,47,74,100,101,102,103}
ROWS=[];CONTROLS=[];SELECTION=[];ERRORS=[]

def milestone(title,detail):
    for rel in ['.agent/CHANGELOG.md','.agent/handoffs/G16-06-RSV2-08.md']:
        with (ROOT/rel).open('a') as f:f.write(f'\n## {datetime.now(timezone.utc).isoformat(timespec="seconds")} — Phase-8 {title}\n{detail}\n')

def margin(v):return min(l['metrics'][k]/floor for l in v['levels'][2:] for k,floor in {**native.LIMITS,'seq_ratio':1.25 if l['area']==.0625 else 1.20}.items())

def publish(state):
    counts={k:sum(v.get(k) is True for v in ROWS) for k in ['geometry_legal','gci_admitted','fine_targets_pass','all_five_targets_pass','export_pass','side_a4_pass','scoped_joint_pass','assignment_scoped_joint_pass']}
    paths=[HERE/p for p in ['run_phase8.py','prepare_phase8.py','build_phase8.sh','phase7_adapter.rs','phase7_receipt.json','PHASE7.md']]+[OUT/p for p in ['main','native_adapter','source_provenance.json']]
    r=dict(task='G16-06-RSV2-08-p8',state=state,cpus=[12,13],nice=os.getpriority(os.PRIO_PROCESS,0),cargo_build_jobs=2,solver='faer',max_concurrent_evaluations=2,
      areas_mm2=native.AREAS,thresholds=native.LIMITS,cert_sequence_floor=1.20,strict_sequence_floor=1.25,
      nominal_cases=12,canonical_cases=len(JOBS),completed_cases=len(ROWS),mesh_observations=sum(len(v['levels']) for v in ROWS),
      grid=dict(ring_wall=[6.4,6.6],taper_start=[138.,144.],tip_thickness=[2.6,2.8,3.]),parents=PARENTS,
      selection_method='Best Phase7 fine-target minimum margin at each requested ring wall; exact-name tie break',
      results=ROWS,replay_controls=CONTROLS,selection=SELECTION,errors=ERRORS,outcome_counts=counts,all_gate_qualification=False,
      unclosed=['human-max twist','independent3D/cross-solver','inherited ring collar/flat-stock constraint'],
      construction='unchanged Phase7 section; only native TAPER_X0/T_TIP vary from the best two full Phase7 genomes',
      sha256={str(p.relative_to(ROOT)):digest(p) for p in paths})
    (HERE/'phase8_receipt.json').write_text(json.dumps(r,indent=2)+'\n')
    lines=['# G16-06-RSV2-08 Phase 8','',f'Status: {state}; {len(ROWS)}/12 cases; {r["mesh_observations"]} requested mesh observations. Counts: {counts}.','',
      'The best Phase7 profile at each ring wall6.4/6.6 is selected by its minimum fine-mesh target margin, with a fixed name tie break. Both have fuller0, window width3/start.45/end.85/fill.65. Twelve exact combinations vary taper start138/144 and tip2.6/2.8/3.0. All other parent genes, the Phase7 section law, native materials/operators/loads/masks/supports/gates and every threshold are unchanged. Two independent115/2.6 controls replay Phase7; no new design is constructed.', '',
      'Every accepted input receives unbiased areas4/1/.25/.125/.0625 and native three-level GCI. Certification=min(.25,.125), strict=.0625. Floors remain spine1, lateral358N, fatigue1, hunter1.377, sequence1.20 at certification and1.25 at strict. All-five comparisons remain separate; export/A4 null means unmeasured and cannot qualify. Full qualification=false. No global infeasibility/optimum or asymptotic validation is claimed.', '',
      '| Case | Cert spine | Strict spine | Cert lateral N | Strict lateral N | Cert fatigue | Strict fatigue | Cert seq | Strict seq | GCI | Fine pass | Export | A4 | Joint |',
      '| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |']
    for v in ROWS:
        c,s=v.get('certification',{}),v.get('strict',{})
        values=[v['name'],c.get('hm_spine_sf'),s.get('hm_spine_sf'),c.get('lat_rob'),s.get('lat_rob'),c.get('fat_sf'),s.get('fat_sf'),c.get('seq_ratio'),s.get('seq_ratio'),v['gci_admitted'],v.get('fine_targets_pass'),v['export_pass'],v['side_a4_pass'],v.get('assignment_scoped_joint_pass',False)]
        lines.append('| '+' | '.join(f'{x:.9f}' if isinstance(x,float) else str(x) for x in values)+' |')
    lines+=['',f'Replay controls: {[v["name"] for v in CONTROLS]}. Selected export/A4: {SELECTION}.','',
      'Execution: `nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/run_phase8.py`. Host inherits nice10, yielding effective19. Native sources differ from the Phase7 snapshot only in an unrelated CLI source, explicitly recorded in build provenance. Portable audit distinguishes tracked receipt verification from optional raw evidence; absent raw data is never described as replayed.','']
    (HERE/'PHASE8.md').write_text('\n'.join(lines))

if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10 and os.environ['CARGO_BUILD_JOBS']=='2'
    native.OUT=OUT;native.BASE=list(csv.reader((HERE/'phase5_sources/q7.csv').open()));native.ALLOWED=ALLOWED
    p7=json.loads((HERE/'phase7_receipt.json').read_text());PARENTS=[];JOBS=[]
    inputs=HERE/'phase8_candidates';inputs.mkdir()
    for ring in [6.4,6.6]:
        parent=sorted([v for v in p7['results'] if v['genes'][42]==ring and v['geometry_legal']],key=lambda v:(-margin(v),v['name']))[0]
        PARENTS.append(dict(name=parent['name'],genome=parent['genome'],genome_sha256=parent['genome_sha256'],margin=margin(parent),genes=parent['genes']))
        for tx,tt in itertools.product([138.,144.],[2.6,2.8,3.]):
            name=f'r{ring:.2f}_x{tx:.0f}_t{tt:.2f}';data=list(csv.reader((ROOT/parent['genome']).open()))
            for i,v in [(46,tt),(47,tx)]:data[1][i]=str(v)
            path=inputs/f'{name}.csv'
            with path.open('w') as f:csv.writer(f,lineterminator='\n').writerows(data)
            JOBS.append((name,path))
    publish('replay_controls')
    for parent in PARENTS:
        row=native.evaluate((f'control_{parent["name"]}',ROOT/parent['genome']))
        original=next(v for v in p7['results'] if v['name']==parent['name'])
        errors=[abs(a['metrics'][k]-b['metrics'][k])/max(abs(b['metrics'][k]),1e-300) for a,b in zip(row['levels'],original['levels']) for k in a['metrics'] if isinstance(a['metrics'][k],(int,float)) and isinstance(b['metrics'].get(k),(int,float))]
        assert max(errors)<=1e-9,'parent replay exceeds existing owner comparison bound'
        row['phase7_max_relative_error']=max(errors);CONTROLS.append(row);publish('replay_controls')
    milestone('milestone 2: two parent replays passed; refinement started','- Best6.4/6.6 Phase7 genomes replayed at five meshes within existing1e-9 owner bound. Twelve requested138/144 x2.6/2.8/3.0 inputs start with unchanged Phase7 section, all genes except taper/tip preserved. No threshold changes.')
    with ThreadPoolExecutor(max_workers=2) as pool:
        futures={pool.submit(native.evaluate,j):j[0] for j in JOBS}
        for future in as_completed(futures):
            name=futures[future]
            try:
                row=future.result();row['assignment_scoped_joint_pass']=False;ROWS.append(row)
                milestone(f'case {name}',f'- Geometry={row["geometry_legal"]}, GCI={row["gci_admitted"]}, fine={row.get("fine_targets_pass")}; certification={row.get("certification")}; strict={row.get("strict")}. Full qualification=false.')
            except Exception as e:ERRORS.append(dict(name=name,error=repr(e)))
            publish('five_mesh_sweep');print('case',name,'complete',len(ROWS),'errors',ERRORS,flush=True)
    assert len(ROWS)==12 and not ERRORS,ERRORS
    for ring in [6.4,6.6]:
        v=sorted([v for v in ROWS if v['genes'][42]==ring and v['geometry_legal']],key=lambda v:(-v['fine_targets_pass'],-margin(v),-v['gci_admitted'],v['name']))[0]
        SELECTION.append(dict(name=v['name'],margin=margin(v),reason='best refined margin at this ring'))
    publish('export_a4')
    for selected in SELECTION:
        row=next(v for v in ROWS if v['name']==selected['name']);genome=ROOT/row['genome']
        ep,e=native.invoke('main','export',row['name'],genome);native.verify_provenance(e)
        assert e['genes']==row['genes'] and e['config']==row['config']
        m,s=e['measurement'],e['solid'];row.update(sharpness=m['sharpness'],point={k:v for k,v in m['point'].items() if k!='samples'},bore_edge_radius_mm=m['bore_edge_radius_mm'],solid=s,worst_edge_face=e['worst_edge_face'])
        row['export_pass']=bool(m['sharpness']['pass'] and m['point']['pass'] and m['bore_edge_radius_mm']>=1.5 and s['census']==[0,0,1] and s['volume_mm3']>0)
        ap,a=native.invoke('native_adapter','a4',row['name'],genome,row['certification']['seq_ratio']);native.verify_provenance(a)
        row['a4']=a['a4'];row['side_a4_pass']=a['G16-13']
        assert row['a4']['same_mesh_ring_clamp_seq_ratio']==row['levels'][2]['metrics']['seq_ratio']
        row['assignment_scoped_joint_pass']=bool(row['geometry_legal'] and row['fine_targets_pass'] and row['gci_admitted'] and row['export_pass'] and row['side_a4_pass'])
        row['scoped_joint_pass']=bool(row['geometry_legal'] and row['all_five_targets_pass'] and row['gci_admitted'] and row['export_pass'] and row['side_a4_pass'])
        for p in [ep,ap,ap.with_suffix('.mesh.json')]:row['sha256'][str(p.relative_to(ROOT))]=digest(p)
        publish('export_a4');milestone(f'export/A4 {row["name"]}',f'- Export={row["export_pass"]}; A4={row["side_a4_pass"]}; joint={row["assignment_scoped_joint_pass"]}. No threshold changes.')
    publish('complete');milestone('milestone 3: refinement measured','- Twelve cases/60 requested observations plus10 replay-control observations, native GCI14, selected export/A4 two. Final portable audits and tests follow; full qualification=false.')
