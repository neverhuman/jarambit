#!/usr/bin/env python3
"""Measure the exact Phase9 grid through the unmodified native GCI API."""
from concurrent.futures import ThreadPoolExecutor,as_completed
from datetime import datetime,timezone
import hashlib,json,os,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];OUT=ROOT/'target/g16-06-rsv2-08/phase10'
ROWS=[];ERRORS=[]
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def science(r):return hashlib.sha256(json.dumps({k:v for k,v in r.items() if k!='sha256'},sort_keys=True,separators=(',',':')).encode()).hexdigest()
def milestone(title,detail):
    for rel in ['.agent/CHANGELOG.md','.agent/handoffs/G16-06-RSV2-08.md']:
        with (ROOT/rel).open('a') as f:f.write('\n## '+datetime.now(timezone.utc).isoformat(timespec='seconds')+' — Phase-10 '+title+'\n'+detail+'\n')
def line(values):return '| '+' | '.join(f'{x:.12g}' if isinstance(x,float) else str(x) for x in values)+' |'
def evaluate(parent):
    name=parent['name'];output=OUT/f'{name}.json';assert not output.exists()
    env={k:v for k,v in os.environ.items() if not k.startswith(('JARAMBIT_','STUDY_'))}
    env.update(JARAMBIT_SOLVER='faer',CARGO_BUILD_JOBS='2',CARGO_INCREMENTAL='0',CARGO_PROFILE_DEV_DEBUG='line-tables-only',RAYON_NUM_THREADS='2')
    with output.with_suffix('.log').open('w') as log:
        subprocess.run([str(OUT/'measure'),parent['genome'],str(output),name],cwd=ROOT,env=env,stdout=log,stderr=log,check=True)
    d=json.loads(output.read_text());assert d['genome_sha256']==parent['genome_sha256'] and d['genes']==parent['genes']
    assert d['source_provenance']==PROVENANCE
    for rel,want in PROVENANCE.items():assert digest(ROOT/rel)==want,rel
    row={k:v for k,v in d.items() if k not in ['source_provenance','seconds']}
    row.update(family=parent['family'],raw=str(output.relative_to(ROOT)),raw_sha256=digest(output),seconds=d['seconds'])
    g=d['plate_gci'];assert g is not None,'Native GCI solve failed'
    fine=g['levels_fine_to_coarse'][0];p9=parent['levels'][0]['result']
    relative=lambda a,b:abs(a-b)/max(abs(b),1e-300)
    row['phase9_fine_binding']=dict(mesh_sha256_matches=fine['mesh_sha256']==p9['mesh_thickness_sha256'],
        sequence_relative_error=relative(fine['seq_ratio'],p9['metrics']['sequence_ratio']),hunter_relative_error=relative(fine['sf_hunter'],p9['metrics']['native_hunter_sf']))
    binding=row['phase9_fine_binding'];binding['within_owner_bar']=binding['mesh_sha256_matches'] and binding['sequence_relative_error']<=1e-9 and binding['hunter_relative_error']<=1e-9
    row['plate_gci_valid_3level']=g['seq_ratio_gci']['sequence_valid'] and g['seq_ratio_gci']['observable_valid']
    row['seq_ratio_gci_admitted']=g['seq_ratio_gci']['admitted']
    return row
def publish(state):
    rows=sorted(ROWS,key=lambda v:v['name'])
    counts={k:sum(v[k] is True for v in rows) for k in ['plate_gci_valid_3level','seq_ratio_gci_admitted']}
    counts.update(sequence_valid=sum(v['plate_gci']['seq_ratio_gci']['sequence_valid'] is True for v in rows),
        observable_valid=sum(v['plate_gci']['seq_ratio_gci']['observable_valid'] is True for v in rows),phase9_fine_binding=sum(v['phase9_fine_binding']['within_owner_bar'] for v in rows))
    r=dict(task='G16-06-RSV2-08-p10',state=state,cpus=[12,13],nice=os.getpriority(os.PRIO_PROCESS,0),cargo_build_jobs=2,solver='faer',max_concurrent_evaluations=2,
        expected_cases=36,completed_cases=len(rows),mesh_observations=3*len(rows),areas_coarse_to_fine_mm2=[4.,1.,.25],areas_fine_to_coarse_mm2=[.25,1.,4.],
        refinement_ratio_r=2.,gci_safety_factor=1.25,method_id='relative_gci.v2',owner_replay_bar=1e-9,
        phase9_parent_receipt_sha256=P9_SHA,phase9_scientific_sha256=science(P9),phase9_report_sha256=digest(HERE/'PHASE9.md'),
        results=rows,errors=ERRORS,outcome_counts=counts,source_provenance=PROVENANCE,production_sources_changed=False,all_gate_qualification=False,
        sha256={str(p.relative_to(ROOT)):digest(p) for p in [HERE/'phase10.rs',HERE/'build_phase10.sh',HERE/'run_phase10.py',OUT/'measure',OUT/'source_provenance.json',OUT/'ASSIGNMENT.md']})
    lines=['# G16-06-RSV2-08 Phase 10','',f'Status: {state}; {len(rows)}/36 cases; {r["mesh_observations"]}/108 native plate observations. Counts: {counts}.','',
        'All36 unchanged Phase9 delayed-taper placement_05/local_01 profiles run through production jarambit_crescent::gci::run_plate_gci_3level. The API evaluates areas.25/1/4mm² fine to coarse (the requested4/1/.25mm²), h=.5/1/2mm, r=2, GCI safety factor1.25 and method relative_gci.v2. Canonical genomes, production export thickness, native FEM loads/supports/regions and thresholds are unchanged. Solver=faer, CPUs12,13, nice -n10 prefix (effective19), Cargo jobs2, two workers.','',
        'sequence_valid and observable_valid validate the bound three-level sequence and observable. Admitted additionally requires a monotone positive-order finite relative-error estimate. These flags are reported separately; checked metadata alone does not imply an admitted fit or asymptotic validity. The production battery plate_gci_valid_3level predicate is sequence_valid AND observable_valid; it is preserved and shown beside the stronger seq_ratio_gci.admitted predicate. No gate is replaced or relaxed.','',
        'The independently rerun.25mm² mesh hash must match Phase9 exactly. Sequence and hunter observables are compared against that Phase9 observation using the existing1e-9 replay bar. No Phase9 numerical results, report, candidates, section law or acceptance thresholds change. Full qualification=false; Phase9 Gate S and lateral/fatigue failures remain failures.','',
        '| Case | sequence_valid | observable_valid | admitted | Fit status | Reason | p | GCI fine | Phase9 binding |',
        '| --- | --- | --- | --- | --- | --- | --- | --- | --- |']
    for row in rows:
        e=row['plate_gci']['seq_ratio_gci'];lines.append(line([row['name']]+[e[k] for k in ['sequence_valid','observable_valid','admitted','fit_status','reason','p','gci_fine']]+[row['phase9_fine_binding']['within_owner_bar']]))
    lines+=['','Secondary hunter GCI is retained in the receipt; native sequence and hunter observations and exact mesh hashes follow.','',
        '| Case | Area mm² | h mm | Nodes | Elements | Sequence ratio | Hunter SF | Mesh/thickness SHA256 |',
        '| --- | --- | --- | --- | --- | --- | --- | --- |']
    for row in rows:
        for l in row['plate_gci']['levels_fine_to_coarse']:lines.append(line([row['name']]+[l[k] for k in ['max_area_mm2','h_mm','n_node','n_elem','seq_ratio','sf_hunter','mesh_sha256']]))
    lines+=['','The portable verifier checks tracked data and recomputes GCI arithmetic with the repository standard-library Python reference; it does not replay native solves and requires no target directory, Git metadata, Cargo settings or CPU mask. Present raw evidence is hash-checked. Asymptotic validation remains not_established; GCI admission is not full product qualification.','']
    (HERE/'PHASE10.md').write_text('\n'.join(lines));(HERE/'phase10_receipt.json').write_text(json.dumps(r,indent=2)+'\n')
if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.environ['CARGO_BUILD_JOBS']=='2' and os.getpriority(os.PRIO_PROCESS,0)>=10
    P9=json.loads((HERE/'phase9_receipt.json').read_text());P9_SHA=digest(HERE/'phase9_receipt.json');assert len(P9['results'])==36 and P9['state']=='complete'
    PROVENANCE=json.loads((OUT/'source_provenance.json').read_text());publish('running')
    milestone('milestone 1: native GCI study started','- Phase10 assignment read; clean captain-committed Phase9 baseline54006d25. Exact36 Phase9 profiles; production native GCI API at4/1/.25mm², faer, CPUs12,13/nice prefix/jobs2. All gates unchanged; Phase9 fine hash/observable replay required. Portable audit/tests follow.')
    with ThreadPoolExecutor(max_workers=2) as pool:
        futures={pool.submit(evaluate,p):p for p in P9['results']}
        for f in as_completed(futures):
            parent=futures[f]
            try:
                row=f.result();ROWS.append(row);e=row['plate_gci']['seq_ratio_gci']
                milestone('case '+row['name'],f'- sequence_valid={e["sequence_valid"]}; observable_valid={e["observable_valid"]}; admitted={e["admitted"]}; status={e["fit_status"]}; Phase9 binding={row["phase9_fine_binding"]}; full qualification=false.')
            except Exception as e:ERRORS.append(dict(name=parent['name'],error=repr(e)))
            publish('running');print(parent['name'],len(ROWS),'errors',ERRORS,flush=True)
    assert len(ROWS)==36 and not ERRORS,ERRORS
    publish('complete');milestone('milestone 2: native observations complete','- All36 profiles/108 native plate observations retained; portable arithmetic/metadata/report audits and requested tests follow. Gates and thresholds unchanged; full qualification=false.')
