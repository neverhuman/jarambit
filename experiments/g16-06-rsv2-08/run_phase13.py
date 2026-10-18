#!/usr/bin/env python3
"""Deterministic 2x4x3 native coordinate/edge-thickness study; every case retained."""
from concurrent.futures import ThreadPoolExecutor,as_completed
from datetime import datetime,timezone
import csv,hashlib,itertools,json,os,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];OUT=ROOT/'target/g16-06-rsv2-08/phase13'
PARENT_NAMES=['local_01_x148_t4.4_ea21.8','local_01_x142_t4.4_ea21.8']
AREAS=[.25,.125,.0625];ROWS=[];ERRORS=[]
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def milestone(title,detail):
    for rel in ['.agent/CHANGELOG.md','.agent/handoffs/G16-06-RSV2-08.md']:
        with (ROOT/rel).open('a') as f:f.write('\n## '+datetime.now(timezone.utc).isoformat(timespec='seconds')+' — Phase-13 '+title+'\n'+detail+'\n')
def line(values):return '| '+' | '.join(f'{x:.12g}' if isinstance(x,float) else str(x) for x in values)+' |'
def aggregate(row):
    row['geometry_legal']=not row['geometry_rejects'];row['fem_complete']=all(v['result'] is not None for v in row['levels'])
    if not row['fem_complete']:row['reported_targets_pass']=False;return
    levels=[v['result']['metrics'] for v in row['levels']];row['certification']={k:min(v[k] for v in levels[:2]) for k in levels[0]};row['strict']=levels[2]
    c,s=row['certification'],row['strict']
    row['summary']=dict(lat_rob_cert=c['lat_rob_n'],strict_lat_rob=s['lat_rob_n'],fat_sf=c['fat_sf'],fat_sf_strict=s['fat_sf'],seq_ratio_cert=c['seq_ratio'],seq_ratio_strict=s['seq_ratio'],
        twist_tip_sf=c['twist_tip_sf'],twist_tip_sf_strict=s['twist_tip_sf'],twist_body_sf=c['twist_body_sf'],twist_tipzone_sf_strict=s['twist_tipzone_sf'],hm_twist_sf=c['hm_twist_sf'],hm_twist_sf_strict=s['hm_twist_sf'])
    row['target_flags']=dict(lateral=c['lat_rob_n']>=358. and s['lat_rob_n']>=358.,fatigue=c['fat_sf']>=1. and s['fat_sf']>=1.,
        sequence=c['seq_ratio']>=1.20 and s['seq_ratio']>=1.25,tip_loaded_twist=c['twist_tip_sf']>=1. and s['twist_tip_sf']>=1.,
        native_utility_twist=c['twist_body_sf']>=1. and s['twist_tipzone_sf']>=1.,human_max_twist=c['hm_twist_sf']>=1. and s['hm_twist_sf']>=1.)
    row['reported_targets_pass']=row['geometry_legal'] and row['aesthetic_pass'] and all(row['target_flags'].values())
def evaluate(job):
    name,parent,offset,edge,path=job;output=OUT/f'{name}.json';assert not output.exists()
    env={k:v for k,v in os.environ.items() if not k.startswith(('JARAMBIT_','STUDY_'))}
    env.update(JARAMBIT_SOLVER='faer',CARGO_BUILD_JOBS='2',CARGO_INCREMENTAL='0',CARGO_PROFILE_DEV_DEBUG='line-tables-only',RAYON_NUM_THREADS='2')
    with output.with_suffix('.log').open('w') as log:
        subprocess.run([str(OUT/'measure'),str(path.relative_to(ROOT)),str(output.relative_to(ROOT)),parent['genome']],cwd=ROOT,env=env,stdout=log,stderr=log,check=True)
    d=json.loads(output.read_text());assert 'seconds' in d and d['source_provenance']==PROVENANCE
    for rel,want in PROVENANCE.items():assert digest(ROOT/rel)==want,rel
    assert digest(path)==d['genome_sha256'] and d['parent_genome_sha256']==parent['genome_sha256']
    row={k:v for k,v in d.items() if k!='source_provenance'}
    row.update(parent=parent['name'],cp_offset_mm=offset,edge_th_mm=edge,raw=str(output.relative_to(ROOT)),raw_sha256=digest(output))
    aggregate(row);return row
def publish(state):
    rows=sorted(ROWS,key=lambda v:v['name']);counts={k:sum(v.get(k) is True for v in rows) for k in ['input_canonical','geometry_legal','aesthetic_pass','fem_complete','reported_targets_pass']}
    counts.update({k:sum(v.get('target_flags',{}).get(k) is True for v in rows) for k in ['lateral','fatigue','sequence','tip_loaded_twist','native_utility_twist','human_max_twist']})
    r=dict(task='G16-06-RSV2-08-p13',state=state,errors=ERRORS,cpus=[12,13],nice=os.getpriority(os.PRIO_PROCESS,0),cargo_build_jobs=2,solver='faer',max_concurrent_evaluations=2,
        expected_cases=24,completed_cases=len(rows),mesh_observations=sum(len(v['levels']) for v in rows),areas_mm2=AREAS,
        grid=dict(parents=PARENT_NAMES,cp_offsets_mm=[0.,.3,.6,.9],edge_th_mm=[.180,.190,.198],cp_indices=[36,37,38],edge_index=89),
        cp_offset_rule='add the same requested offset to each parent CP36/37/38 gene; positive is native outward normal',
        thresholds=dict(lateral_n=358.,fat_sf=1.,seq_cert=1.20,seq_strict=1.25,twist_tip_sf=1.,twist_body_sf=1.,twist_tipzone_sf_strict=1.,hm_twist_sf=1.),
        parents=PARENTS,results=rows,outcome_counts=counts,production_sources_changed=False,all_gate_qualification=False,
        unverified=['new-offspring actual-export Gate S','new-offspring plate GCI','new-offspring side_a4','independent3D/remaining qualification gates'],
        source_provenance=PROVENANCE,parent_receipts_before_metadata_sha256=INITIAL['receipts'],
        parent_reports_sha256={str((HERE/f'PHASE{n}.md').relative_to(ROOT)):digest(HERE/f'PHASE{n}.md') for n in [9,10,11,12]},
        sha256={str(p.relative_to(ROOT)):digest(p) for p in [HERE/'phase13.rs',HERE/'build_phase13.sh',HERE/'run_phase13.py',OUT/'measure',OUT/'source_provenance.json',OUT/'ASSIGNMENT.md',OUT/'initial-state.json']})
    lines=['# G16-06-RSV2-08 Phase 13','',f'Status: {state}; {len(rows)}/24 cases; {r["mesh_observations"]}/72 mesh observations. Counts: {counts}.','',
        'The two assignment-named Phase9 parents local_01_x148_t4.4_ea21.8 and local_01_x142_t4.4_ea21.8 each receive CP36/37/38 additive offsets0/.3/.6/.9mm and EDGE_TH.180/.190/.198mm. These40 leading genes are scalar offsets along the production C4 outward normals, not Cartesian coordinate pairs. Only genes36/37/38/89 change; all other128-gene values and material remain parent-identical. Exact native clamp equality/RS-v2 and native geometry/aesthetic results are retained for every case. Genotype legality does not establish actual-export Gate S.','',
        'Production phenotype, export::thickness_at and unbiased fem::report_fields at.25/.125/.0625mm² are unchanged. Certification is the metric-wise minimum of.25/.125; strict is.0625. twist_tip_sf means native Fem.twist_sf: tip-loaded global chord torsion with its native3D section correction. twist_tipzone_sf_strict is the separate native4–15mm annular torsion result on the strict mesh; hm_twist_sf is the native16.2Nm human-maximum ratio. No metric is substituted for another.','',
        'All prior thresholds remain: lateral358N at certification/strict, Goodman fatigue>=1, sequence>=1.20 certification/>=1.25 strict, recorded tip-loaded twist>=1, native utility twist=body certification>=1 AND strict tipzone>=1, human-max twist>=1. All requested metrics are shown for each case and all three native observations are retained, including failures. No native source/load/support/region/bias/gate/threshold edits. Explicit faer, CPUs12,13, nice -n10 prefix/effective19, Cargo jobs2/two workers.','',
        'This is a bounded native-coordinate sensitivity study. Actual-export Gate S, GCI, A4 and independent3D are unverified for these new offspring; parent verdicts are not inherited. Full qualification=false. No staging, commit or push.','',
        '| Candidate | Cert lateral N | Strict lateral N | Cert fatigue | Strict fatigue | Cert sequence | Strict sequence | Cert tip twist SF | Strict tipzone SF | Cert human twist SF | Strict human twist SF | Reported targets |',
        '| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |']
    for row in rows:
        s=row.get('summary',{});lines.append(line([row['name']]+[s.get(k) for k in ['lat_rob_cert','strict_lat_rob','fat_sf','fat_sf_strict','seq_ratio_cert','seq_ratio_strict','twist_tip_sf','twist_tipzone_sf_strict','hm_twist_sf','hm_twist_sf_strict']]+[row['reported_targets_pass']]))
    lines+=['','| Candidate | Area mm² | Lateral N | Fatigue SF | Sequence | Tip-loaded twist SF | Tipzone SF | Human-max twist SF | Mesh/thickness SHA256 |','| --- | --- | --- | --- | --- | --- | --- | --- | --- |']
    for row in rows:
        for level in row['levels']:
            result=level['result'] or {};m=result.get('metrics',{});lines.append(line([row['name'],level['area_mm2']]+[m.get(k) for k in ['lat_rob_n','fat_sf','seq_ratio','twist_tip_sf','twist_tipzone_sf','hm_twist_sf']]+[result.get('mesh_thickness_sha256')]))
    lines+=['','Portable verification checks the exact Cartesian grid, tracked candidate/parent bytes, assigned/unassigned genes, native execution/source identities, native CP displacements, all three observations, certification minima, strict values and unchanged exact target comparisons. It performs no native solve and needs no target/.git/Cargo/CPU setting; present raw evidence must match hashes.','']
    (HERE/'PHASE13.md').write_text('\n'.join(lines));(HERE/'phase13_receipt.json').write_text(json.dumps(r,indent=2)+'\n')
if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.environ['CARGO_BUILD_JOBS']=='2' and os.getpriority(os.PRIO_PROCESS,0)>=10
    INITIAL=json.loads((OUT/'initial-state.json').read_text());PROVENANCE=json.loads((OUT/'source_provenance.json').read_text())
    p9=json.loads((HERE/'phase9_receipt.json').read_text());PARENTS=[];JOBS=[];inputs=HERE/'phase13_candidates';inputs.mkdir()
    for name in PARENT_NAMES:
        p=next(v for v in p9['results'] if v['name']==name);base=list(csv.reader((ROOT/p['genome']).read_text().splitlines()))
        PARENTS.append(dict(name=name,genome=p['genome'],genome_sha256=p['genome_sha256'],genes=p['genes'],material=base[1][128]))
        for offset,edge in itertools.product([0.,.3,.6,.9],[.180,.190,.198]):
            label=f'{name}_cp{offset:.1f}_et{edge:.3f}';data=[list(v) for v in base]
            for i in [36,37,38]:data[1][i]=str(float(base[1][i])+offset)
            data[1][89]=f'{edge:.3f}';path=inputs/f'{label}.csv'
            with path.open('w') as f:csv.writer(f,lineterminator='\n').writerows(data)
            JOBS.append((label,p,offset,edge,path))
    publish('running');milestone('milestone 1: production grid started','- Native build passed;24 canonical parent/CP36..38/EDGE_TH cells generated mechanically from assigned parents. Every cell evaluated at.25/.125/.0625, including legal/geometry/aesthetic/FEM failures. No hand-built geometry or gate/threshold changes; CPUs12,13/nice/jobs2; do not commit/push.')
    with ThreadPoolExecutor(max_workers=2) as pool:
        futures={pool.submit(evaluate,j):j for j in JOBS}
        for f in as_completed(futures):
            job=futures[f]
            try:
                row=f.result();ROWS.append(row);milestone('case '+row['name'],f'- Geometry={row["geometry_legal"]}; aesthetics={row["aesthetic_pass"]}; FEM={row["fem_complete"]}; summary={row.get("summary")}; flags={row.get("target_flags")}; full qualification=false.')
            except Exception as e:ERRORS.append(dict(name=job[0],error=repr(e)))
            publish('running');print(job[0],len(ROWS),'errors',ERRORS,flush=True)
    assert len(ROWS)==24 and not ERRORS,ERRORS
    publish('complete');milestone('milestone 2: native grid observations complete','- All24 candidates/72 native mesh observations retained. Portable integrity, metadata-only lineage refresh and requested tests follow. No source/gate/threshold changes; no staging/commit/push.')
