#!/usr/bin/env python3
"""Bounded Q7 window/hand sweep with native five-mesh, GCI, export and A4 evidence."""
from concurrent.futures import ThreadPoolExecutor, as_completed
import csv
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import subprocess
from prepare_phase5 import HERE, ROOT, OUT, SNAP, digest

AREAS=[4.,1.,.25,.125,.0625]
LIMITS={'hm_spine_sf':1.,'lat_rob':358.,'fat_sf':1.,'sf_hunter':1.377}
ALLOWED={6,7,8,42,100,101,102,103}
ROWS=[]
ERRORS=[]
ASSEMBLY=[]
CACHED=[]

def milestone(title,detail):
    stamp=datetime.now(timezone.utc).isoformat(timespec='seconds')
    for rel in ['.agent/CHANGELOG.md','.agent/handoffs/G16-06-RSV2-08.md']:
        with (ROOT/rel).open('a') as f:f.write(f'\n## {stamp} — Phase-5 {title}\n{detail}\n')

def environment():
    env={k:v for k,v in os.environ.items() if not k.startswith(('STUDY_','JARAMBIT_'))}
    env.update(json.loads((SNAP/'q7_control_case.json').read_text())['config'])
    env.update(STUDY_AREAS='4,1,.25,.125,.0625',CARGO_BUILD_JOBS='2',RAYON_NUM_THREADS='2',
               CARGO_INCREMENTAL='0',CARGO_PROFILE_DEV_DEBUG='line-tables-only')
    assert env['JARAMBIT_SOLVER']=='faer'
    return env

def invoke(binary,mode,name,genome,cert=None):
    path=OUT/f'{name}-{mode}.json'
    if path.exists():
        assert (OUT/'attempt1_receipt.json').exists(), 'explicit retained-attempt recovery required'
        doc=json.loads(path.read_text());assert 'seconds' in doc, 'never reuse a partial artifact'
        CACHED.append(dict(path=str(path.relative_to(ROOT)),sha256=digest(path)))
        return path,doc
    command=[str(OUT/binary),mode,str(path),str(genome)]
    if cert is not None:command.append(str(cert))
    with path.with_suffix('.log').open('w') as log:
        subprocess.run(command,cwd=ROOT,env=environment(),stdout=log,stderr=log,check=True)
    return path,json.loads(path.read_text())

def verify_provenance(doc):
    assert doc['provenance']==json.loads((OUT/'source_provenance.json').read_text())
    for rel,want in doc['provenance']['production_sha256'].items():assert digest(ROOT/rel)==want,rel
    assert doc['config']['d0']==.48 and doc['config']['d1']==.65
    assert doc['config']['point_shape']==3 and doc['config']['point_lateral']==2
    assert doc['config']['pocket_t']==doc['config']['window_belly']==0

def base_row(name,genome,doc):
    verify_provenance(doc)
    raw=list(csv.reader(genome.open()))[1]
    requested=[float(v) for v in raw[:128]]
    if doc['input_was_canonical']:assert requested==doc['genes']
    assert raw[128]==BASE[1][128]
    changed=[i for i,(a,b) in enumerate(zip(BASE[1],raw)) if a!=b]
    assert set(changed)<=ALLOWED and float(raw[64])==42.
    return dict(name=name,genome=str(genome.relative_to(ROOT)),genome_sha256=digest(genome),
                changed_indices=changed,parameters={BASE[0][i]:float(raw[i]) for i in sorted(ALLOWED)},
                genes=requested,observed_genes=doc['genes'],input_was_canonical=doc['input_was_canonical'],
                domain_projection_differences=[i for i,(a,b) in enumerate(zip(requested,doc['genes'])) if a!=b],
                config=doc['config'],solver='faer',
                geometry_legal=bool(doc['input_was_canonical'] and doc['within_genome_domain'] and
                                    doc['rs_v2'] and doc['aes_pass'] and not doc['rejects']),
                rejects=doc['rejects'],aesthetic_failures=doc['aes_fails'],holes=doc['holes'],
                window_count=doc['window_count'],window_area_mm2=doc['window_area_mm2'],
                window_geometry=doc['window_geometry'],mass_g=doc['mass_g'],
                neck_section=doc['neck_section'],all_gate_qualification=False)

def measurements(row,doc):
    levels=doc['levels'];assert [v['area'] for v in levels]==AREAS
    assert all(v['unbiased'] and v['metrics'] is not None for v in levels)
    keys=list(LIMITS)+['seq_ratio','hm_twist_sf','hm_ringback_sf']
    row.update(levels=levels,certification={k:min(v['metrics'][k] for v in levels[2:4]) for k in keys},
               strict={k:levels[4]['metrics'][k] for k in keys},
               all_five_minimum={k:min(v['metrics'][k] for v in levels) for k in keys})
    def passed(group):
        return all(all(l['metrics'][k]>=floor for k,floor in LIMITS.items()) and
                   l['metrics']['seq_ratio']>=(1.25 if l['area']==.0625 else 1.20) for l in group)
    row['fine_targets_pass']=passed(levels[2:])
    row['all_five_targets_pass']=passed(levels)
    row['fine_score']=min(l['metrics'][k]/floor for l in levels[2:] for k,floor in
                          {**LIMITS,'seq_ratio':1.25}.items())
    row['all_five_score']=min(l['metrics'][k]/floor for l in levels for k,floor in
                              {**LIMITS,'seq_ratio':1.25}.items())
    seq=[v['metrics']['seq_ratio'] for v in levels];diff=[b-a for a,b in zip(seq,seq[1:])]
    row['all_five_sequence_monotonic']=all(d>0 for d in diff) or all(d<0 for d in diff)

def evaluate(job):
    name,genome=job
    gp,geom=invoke('main','geometry',name,genome)
    row=base_row(name,genome,geom);paths=[genome,gp]
    if row['geometry_legal']:
        assert row['window_count']>=1 and 'window' in row['holes']
        fp,full=invoke('main','fem',name,genome);verify_provenance(full)
        measurements(row,full)
        cp,gci=invoke('native_adapter','gci',name,genome)
        verify_provenance(gci);assert gci['genome_sha256']==row['genome_sha256']
        assert gci['native_gci_source_sha256']==digest(ROOT/'crates/crescent/src/gci.rs')
        for gl in gci['plate_gci']['levels_fine_to_coarse']:
            m=next(l['metrics'] for l in row['levels'] if l['area']==gl['max_area_mm2'])
            assert m['seq_ratio']==gl['seq_ratio'] and m['sf_hunter']==gl['sf_hunter']
        row.update(gci=gci['plate_gci'],gci_admitted=gci['G16-15'],coverage='five_meshes_and_native_gci')
        paths += [fp,cp]
    else:
        row.update(coverage='native_geometry_rejected',levels=[],gci_admitted=None)
    row.update(export_pass=None,side_a4_pass=None,scoped_joint_pass=False,
               sha256={str(p.relative_to(ROOT)):digest(p) for p in paths})
    (OUT/f'{name}-summary.json').write_text(json.dumps(row,indent=2)+'\n')
    return row

def create(name,changes,reason):
    data=[list(v) for v in BASE]
    for i,v in changes.items():
        assert i in ALLOWED
        if float(data[1][i])!=v:data[1][i]=str(v)
    assert .05<=float(data[1][100])<=.5 and .5<=float(data[1][101])<=.92
    assert 3.<=float(data[1][102])<=10. and .45<=float(data[1][103])<=.8
    assert float(data[1][101])>=float(data[1][100])+.2
    key=tuple(float(v) for v in data[1][:128])
    if key in SEEN:return None
    SEEN.add(key)
    path=INPUTS/f'{name}.csv'
    if path.exists():assert list(csv.reader(path.open()))==data,'existing input must match exactly'
    else:
        with path.open('w') as f:csv.writer(f,lineterminator='\n').writerows(data)
    ASSEMBLY.append(dict(name=name,changes=changes,reason=reason,sha256=digest(path)))
    (OUT/'assembly.json').write_text(json.dumps(ASSEMBLY,indent=2)+'\n')
    return name,path

def publish(state):
    counts={k:sum(v.get(k) is True for v in ROWS) for k in
            ['geometry_legal','gci_admitted','fine_targets_pass','all_five_targets_pass','export_pass','side_a4_pass','scoped_joint_pass']}
    receipt=dict(task='G16-06-RSV2-08-p5',state=state,cpus=[12,13],nice=os.getpriority(os.PRIO_PROCESS,0),
                 cargo_build_jobs=2,solver='faer',max_concurrent_evaluations=2,thresholds=LIMITS,
                 cert_sequence_floor=1.20,strict_sequence_floor=1.25,areas_mm2=AREAS,
                 sweep='96 initial combinations; 48 bounded smaller-step hand profiles at legal ring wall6.3; up to24 local window refinements',
                 assignment_domain_discrepancy='Native CP08 maximum2.0; original all-3 requests are retained as domain rejects, never silently accepted after clamping',
                 prior_attempt=dict(receipt='target/G16-06-RSV2-08/phase5/attempt1_receipt.json',
                                    runner='target/G16-06-RSV2-08/phase5/run_phase5-attempt1.py',
                                    errors='24 identity-guard assertions on requested CP08=3 exceeding native maximum2; recovered as explicit domain rejections'),
                 cached_artifacts=CACHED,
                 acceptance='all five meshes must retain strength floors and sequence>=1.20 (strict>=1.25); geometry/GCI/export/A4 mandatory',
                 completed_cases=len(ROWS),mesh_evaluations=sum(len(v['levels']) for v in ROWS),
                 outcome_counts=counts,results=ROWS,errors=ERRORS,baseline_reproduction=BASELINE,
                 selection=SELECTION,all_gate_qualification=False,
                 unclosed=['independent 3D/cross-solver qualification','human-max twist','inherited ring collar/flat-stock constraint'],
                 sha256={str(p.relative_to(ROOT)):digest(p) for p in
                         [HERE/'run_phase5.py',HERE/'prepare_phase5.py',HERE/'build_phase5.sh',OUT/'main',OUT/'native_adapter',
                          OUT/'source_provenance.json',OUT/'assembly.json',SNAP/'q7.csv',SNAP/'q7_control_case.json']})
    (HERE/'phase5_receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
    lines=['# G16-06-RSV2-08 Phase 5','',f'Status: {state}; {len(ROWS)} trials recorded; {receipt["mesh_evaluations"]} five-mesh observations. Counts: {counts}.',
           '', 'The complete assigned Q7 CSV and section implementation are snapshotted in phase5_sources/. Only window S0/S1/width/fill, ring_wall and CP06..08 vary. Spine offset42 and every other gene remain parental. Native material, FEM operators/loads/masks, geometry gates, certification, GCI and support constants are unchanged. CPUs12,13, requested nice -n10 prefix (inherited effective nice recorded in receipt), Cargo jobs2, explicit faer.',
           '', 'All accepted geometry trials receive areas4/1/.25/.125/.0625 plus native three-level4/1/.25 GCI. Geometry rejects are recorded and cannot qualify. Full export/A4 selection retains every all-five numerical pass and representative fine-mesh/GCI tradeoffs. Unselected screens have export/A4=null and do not count as joint passes.',
           '', 'Native CP08 is bounded by2.0 despite the assignment saying up to3. Initial all-3 inputs triggered24 identity-guard assertions; the first runner/receipt/log are retained. Those requests are now explicitly classified as domain rejections, with requested and observed genes kept separate. No clamped input is accepted. Complete matching initial native artifacts are reused by hash; additional smaller-step hand probes obey every native bound.',
           '', 'Acceptance requires spine>=1, lateral>=358N, fatigue>=1 and hunter>=1.377 on every mesh; sequence>=1.20 on4/1/.25/.125 and>=1.25 on strict.0625. Actual native exported sharpness/point/bore/census and native A4 must pass. Admission is an estimate, with asymptotic validation not established. No gate or threshold is relaxed.',
           '', '| Case | Legal | Cert spine | Strict spine | Cert lat | Strict lat | Cert seq | Strict seq | GCI | All-five targets | Export | A4 | Joint |',
           '| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |']
    for v in ROWS:
        c,t=v.get('certification',{}),v.get('strict',{})
        values=[v['name'],v['geometry_legal'],c.get('hm_spine_sf'),t.get('hm_spine_sf'),c.get('lat_rob'),t.get('lat_rob'),c.get('seq_ratio'),t.get('seq_ratio'),v.get('gci_admitted'),v.get('all_five_targets_pass'),v['export_pass'],v['side_a4_pass'],v['scoped_joint_pass']]
        lines.append('| '+' | '.join(f'{x:.9f}' if isinstance(x,float) else str(x) for x in values)+' |')
    lines+=['',f'Baseline reproduction: {BASELINE}.', '',f'Export/A4 selection: {SELECTION}.',
            '', 'This bounded parameter study does not establish global infeasibility or an optimum. Full RS-v2/manufacturing qualification remains false. Native human-max twist and independent solid/cross-solver/manufacturing constraints are not waived. Raw measurements, canonical inputs, peak coordinates, mesh hashes and rejected geometry remain in target/G16-06-RSV2-08/phase5/.',
            '', 'Original invocation:', '```sh','nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/run_phase5.py','```','']
    (HERE/'PHASE5.md').write_text('\n'.join(lines))

def batch(jobs,label):
    with ThreadPoolExecutor(max_workers=2) as pool:
        futures={pool.submit(evaluate,j):j[0] for j in jobs}
        for f in as_completed(futures):
            name=futures[f]
            try:
                row=f.result();ROWS.append(row)
                milestone(f'{label} {name}',f'- Coverage={row["coverage"]}; geometry={row["geometry_legal"]}; GCI={row.get("gci_admitted")}; all-five targets={row.get("all_five_targets_pass")}; cert={row.get("certification")}; strict={row.get("strict")}. Full qualification=false.')
            except Exception as e:ERRORS.append(dict(name=name,error=repr(e)))
            publish(label);print(label,name,len(ROWS),'errors',len(ERRORS),flush=True)

if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10
    assert os.environ['CARGO_BUILD_JOBS']=='2'
    INPUTS=HERE/'phase5_candidates';assert (OUT/'attempt1_receipt.json').exists();INPUTS.mkdir(exist_ok=True)
    BASE=list(csv.reader((SNAP/'q7.csv').open()));SEEN=set();SELECTION=[]
    baseline=json.loads((OUT/'q7-baseline-full.json').read_text());verify_provenance(baseline)
    old=json.loads((SNAP/'q7_control_fem.json').read_text())
    errors=[]
    for previous in old['levels']:
        actual=next(l for l in baseline['levels'] if l['area']==previous['area'])
        for k,want in previous['metrics'].items():
            if isinstance(want,(int,float)) and isinstance(actual['metrics'].get(k),(int,float)):
                errors.append(abs(actual['metrics'][k]-want)/max(abs(want),1e-300))
    assert max(errors)<=1e-9,'Q7 baseline must reproduce within existing owner bound'
    assert baseline['solver']=='faer' and baseline['aes_pass'] and not baseline['rejects']
    BASELINE=dict(max_relative_scalar_error=max(errors),owner_bound=1e-9,pass_=True)
    baseline_job=create('q7_baseline',{},'exact assigned Q7 control')
    # Use fresh five-mesh/GCI baseline alongside the independent initial full export control.
    windows=[(.5,.85,3.,.65),(.5,.85,4.5,.65),(.5,.85,6.,.70),(.35,.85,3.,.8),
             (.35,.85,4.5,.70),(.2,.7,4.5,.65),(.05,.525,4.5,.70),(.052,.5,6.,.65)]
    jobs=[baseline_job]
    for wi,w in enumerate(windows):
        for ring in [6.3,6.8,7.]:
            for hand in range(4):
                changes=dict(zip([100,101,102,103],w));changes[42]=ring
                for i in range(6,6+hand):changes[i]=3.
                job=create(f'w{wi:02d}_r{ring:.1f}_h{hand}',changes,'initial Cartesian window/ring/staged-hand sweep')
                if job:jobs.append(job)
    publish('initial_sweep');batch(jobs,'initial_sweep')
    assert not ERRORS,ERRORS
    gentle=[]
    for wi,w in enumerate(windows):
        for profile in range(6):
            changes=dict(zip([100,101,102,103],w));changes[42]=6.3
            if profile<3:
                alpha=[.0625,.125,.25][profile]
                for i,target in [(6,3.),(7,3.),(8,2.)]:changes[i]=(1-alpha)*float(BASE[1][i])+alpha*target
            else:
                i=6+profile-3;changes[i]=float(BASE[1][i])+.1
            job=create(f'gentle_w{wi:02d}_p{profile}',changes,'bounded smaller-step hand reinforcement; larger ring walls rejected by native grip/envelope')
            if job:gentle.append(job)
    batch(gentle,'bounded_hand_sweep');assert not ERRORS,ERRORS
    seeds=sorted([r for r in ROWS if r['geometry_legal'] and r['genes'][42] in [6.3,6.8,7.]],key=lambda r:(r['fine_targets_pass'],r['fine_score'],r['gci_admitted']),reverse=True)[:3]
    refinements=[]
    for si,seed in enumerate(seeds):
        original={i:seed['genes'][i] for i in ALLOWED}
        for i,step in [(100,.01),(101,.01),(102,.3),(103,.05)]:
            for sign in [-1,1]:
                changes=dict(original);changes[i]+=sign*step
                if not (.05<=changes[100]<=.5 and .5<=changes[101]<=.92 and changes[101]>=changes[100]+.2 and 3.<=changes[102]<=10. and .45<=changes[103]<=.8):continue
                job=create(f'local{si}_g{i}_{"plus" if sign>0 else "minus"}',changes,f'one-coordinate refinement of {seed["name"]}')
                if job:refinements.append(job)
    batch(refinements,'local_sweep');assert not ERRORS,ERRORS
    legal=[v for v in ROWS if v['geometry_legal']]
    picked={v['name']:'all-five numerical pass' for v in legal if v['all_five_targets_pass']}
    picked['q7_baseline']='exact Q7 control'
    for key,count,predicate in [('fine_score',4,lambda v:True),('all_five_score',3,lambda v:True),
                                ('fine_score',3,lambda v:v['gci_admitted'])]:
        for v in sorted([v for v in legal if predicate(v)],key=lambda v:v[key],reverse=True)[:count]:
            picked.setdefault(v['name'],f'top {key} / admitted filter={key=="fine_score" and count==3}')
    SELECTION=[dict(name=n,reason=why) for n,why in picked.items()]
    (OUT/'export_selection.json').write_text(json.dumps(SELECTION,indent=2)+'\n');publish('export_a4')
    for selected in SELECTION:
        row=next(v for v in ROWS if v['name']==selected['name']);genome=ROOT/row['genome']
        ep,export=invoke('main','export',row['name'],genome);verify_provenance(export)
        assert export['genes']==row['genes'] and export['config']==row['config']
        m=export['measurement'];solid=export['solid']
        row.update(sharpness=m['sharpness'],point={k:v for k,v in m['point'].items() if k!='samples'},
                   bore_edge_radius_mm=m['bore_edge_radius_mm'],solid=solid,worst_edge_face=export['worst_edge_face'])
        row['export_pass']=bool(m['sharpness']['pass'] and m['point']['pass'] and m['bore_edge_radius_mm']>=1.5 and
                                solid['census']==[0,0,1] and solid['volume_mm3']>0)
        ap,side=invoke('native_adapter','a4',row['name'],genome,row['certification']['seq_ratio'])
        verify_provenance(side);a4=side['a4'];row['a4']=a4
        assert a4['same_mesh_ring_clamp_seq_ratio']==row['levels'][2]['metrics']['seq_ratio']
        assert a4['genome_sha256']==row['genome_sha256']
        row['side_a4_pass']=side['G16-13']
        row['scoped_joint_pass']=bool(row['geometry_legal'] and row['all_five_targets_pass'] and row['gci_admitted'] and row['export_pass'] and row['side_a4_pass'])
        row['coverage']='five_meshes_native_gci_actual_export_a4'
        for p in [ep,ap,ap.with_suffix('.mesh.json')]:row['sha256'][str(p.relative_to(ROOT))]=digest(p)
        publish('export_a4');milestone(f'export/A4 {row["name"]}',f'- Export={row["export_pass"]}; A4={row["side_a4_pass"]}; joint={row["scoped_joint_pass"]}. Native thresholds and source unchanged; full qualification=false.')
        print('export_a4',row['name'],row['scoped_joint_pass'],flush=True)
    publish('complete')
    milestone('all assigned sweep stages complete',f'- Trials={len(ROWS)}; legal five-mesh cases={len(legal)}; requested mesh observations={sum(len(v["levels"]) for v in ROWS)}; full export/A4={len(SELECTION)}. Joint passes={[v["name"] for v in ROWS if v["scoped_joint_pass"]]}. All rejected/failed/unselected evidence retained; full qualification=false. Final audit and archive follow.')
