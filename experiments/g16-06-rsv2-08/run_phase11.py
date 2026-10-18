#!/usr/bin/env python3
"""Production side.a4 for every unchanged Phase9/10 candidate; failures retained."""
from concurrent.futures import ThreadPoolExecutor,as_completed
from datetime import datetime,timezone
import hashlib,json,os,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];OUT=ROOT/'target/g16-06-rsv2-08/phase11'
ROWS=[];ERRORS=[]
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def scientific(r):return hashlib.sha256(json.dumps({k:v for k,v in r.items() if k!='sha256'},sort_keys=True,separators=(',',':')).encode()).hexdigest()
def milestone(title,detail):
    for rel in ['.agent/CHANGELOG.md','.agent/handoffs/G16-06-RSV2-08.md']:
        with (ROOT/rel).open('a') as f:f.write('\n## '+datetime.now(timezone.utc).isoformat(timespec='seconds')+' — Phase-11 '+title+'\n'+detail+'\n')
def line(values):return '| '+' | '.join(f'{x:.12g}' if isinstance(x,float) else str(x) for x in values)+' |'
def evaluate(parent):
    name=parent['name'];output=OUT/f'{name}.json';assert not output.exists();cert=parent['certification']['sequence_ratio']
    env={k:v for k,v in os.environ.items() if not k.startswith(('JARAMBIT_','STUDY_'))}
    env.update(JARAMBIT_SOLVER='faer',CARGO_BUILD_JOBS='2',CARGO_INCREMENTAL='0',CARGO_PROFILE_DEV_DEBUG='line-tables-only',RAYON_NUM_THREADS='2')
    with output.with_suffix('.log').open('w') as log:
        subprocess.run([str(OUT/'measure'),parent['genome'],str(output.relative_to(ROOT)),name,repr(cert)],cwd=ROOT,env=env,stdout=log,stderr=log,check=True)
    d=json.loads(output.read_text());assert d['genome_sha256']==parent['genome_sha256'] and d['genes']==parent['genes'] and d['seq_cert_input']==cert
    assert d['source_provenance']==PROVENANCE
    for rel,want in PROVENANCE.items():assert digest(ROOT/rel)==want,rel
    assert digest(ROOT/d['mesh_json'])==d['mesh_json_sha256']
    row={k:v for k,v in d.items() if k not in ['source_provenance','seconds']}
    row.update(family=parent['family'],raw=str(output.relative_to(ROOT)),raw_sha256=digest(output),seconds=d['seconds'])
    old=parent['levels'][0]['result'];p10=GCI[name]['plate_gci']['levels_fine_to_coarse'][0]
    relative=abs(d['same_mesh_ring_clamp_seq_ratio']-old['metrics']['sequence_ratio'])/max(abs(old['metrics']['sequence_ratio']),1e-300)
    row['parent_mesh_binding']=dict(phase9_hash_matches=d['mesh_sha256']==old['mesh_thickness_sha256'],phase10_hash_matches=d['mesh_sha256']==p10['mesh_sha256'],ring_clamp_sequence_relative_error=relative)
    binding=row['parent_mesh_binding'];binding['within_owner_bar']=binding['phase9_hash_matches'] and binding['phase10_hash_matches'] and relative<=1e-9
    a=d['a4'];row['a4_complete']=a is not None
    row['buckling_pass']=bool(a and a['buckling_eigenvalue']>=1.)
    row['support_sequence_pass']=bool(a and a['supported_seq_ratio']>=cert)
    row['side_a4_pass']=row['buckling_pass'] and row['support_sequence_pass']
    row['margins']=dict(buckling=a['buckling_eigenvalue']-1. if a else None,sequence=a['supported_seq_ratio']-cert if a else None)
    return row
def publish(state):
    rows=sorted(ROWS,key=lambda v:v['name']);counts={k:sum(v[k] is True for v in rows) for k in ['a4_complete','buckling_pass','support_sequence_pass','side_a4_pass']}
    counts['parent_mesh_binding']=sum(v['parent_mesh_binding']['within_owner_bar'] for v in rows)
    r=dict(task='G16-06-RSV2-08-p11',state=state,cpus=[12,13],nice=os.getpriority(os.PRIO_PROCESS,0),cargo_build_jobs=2,solver='faer',max_concurrent_evaluations=2,
        expected_cases=36,completed_cases=len(rows),a4_observations=sum(v['a4_complete'] for v in rows),mesh_area_mm2=.25,certificate_areas_mm2=[.25,.125],
        thresholds=dict(buckling_eigenvalue=1.,sequence='supported_seq_ratio >= Phase9 min(sequence_ratio at .25/.125)',owner_replay_bar=1e-9),
        native_support_constants=dict(E_MPA=113800.,NU=.34,bore_radial=50000.,bore_tangential=5000.,bore_flange=100000.,grip_foundation=200.),
        phase9_parent_receipt_sha256=P9_SHA,phase10_parent_receipt_sha256=P10_SHA,phase9_scientific_sha256=scientific(P9),
        parent_reports_sha256={str((HERE/f'PHASE{n}.md').relative_to(ROOT)):digest(HERE/f'PHASE{n}.md') for n in [9,10]},
        results=rows,errors=ERRORS,outcome_counts=counts,source_provenance=PROVENANCE,production_sources_changed=False,all_gate_qualification=False,
        sha256={str(p.relative_to(ROOT)):digest(p) for p in [HERE/'phase11.rs',HERE/'build_phase11.sh',HERE/'run_phase11.py',OUT/'measure',OUT/'source_provenance.json',OUT/'ASSIGNMENT.md']})
    lines=['# G16-06-RSV2-08 Phase 11','',f'Status: {state}; {len(rows)}/36 cases. Counts: {counts}.','',
        'All36 unchanged Phase9/10 placement_05/local_01 delayed-taper profiles use the native production side_a4 elastic-foundation solver through battery::run_side_a4_native_for_genome, on the full-precision in-memory production FEM .25mm² mesh. Exact binary xy/f64LE, triangle/u64LE and element-thickness/f64LE hashes bind to both Phase9 and Phase10; the complete unrounded mesh JSON is retained as optional raw evidence. The native battery d1_sha256 zero sentinel is preserved and is not claimed as a D1 certificate; separate mesh JSON and binary hashes provide the binding.','',
        'seq_cert is the unchanged Phase9 certificate minimum over.25/.125mm², not a substituted floor or the lone.25 result. The requested buckling predicate is eigenvalue>=1.0; support consistency is seq_a4>=seq_cert, evaluated independently and jointly without a tolerance. Native solver status is retained separately: it checks eigenvalue>0 and support consistency, so status alone is not the requested buckling verdict. Buckling critical load is260N times eigenvalue.','',
        'The unmodified native5-DOF CST/DKT solver uses radial/tangential/flange bore springs50000/5000/100000, grip Winkler foundation200, E=113800MPa and nu=.34. Native tip loading,15mm stress-region exclusion,32-step deterministic K-orthogonal Lanczos and regularization are unchanged. Report_fields remains the ring-clamp plate model; A4 support results do not replace its certified sequence. Explicit faer, CPUs12,13, nice -n10 prefix/effective19, Cargo jobs2, two workers.','',
        'Phase9 Gate S/lateral/fatigue failures and Phase10 oscillatory/unadmitted GCI remain unchanged. This A4 study does not establish all-gate qualification. Full qualification=false. No native source, design, support, load, gate or threshold changed.','',
        '| Case | Eigenvalue | Buckling margin | seq_a4 | seq_cert | Sequence margin | Buckling pass | Sequence pass | A4 pass | Native status |',
        '| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |']
    for row in rows:
        a=row['a4'] or {};lines.append(line([row['name'],a.get('buckling_eigenvalue'),row['margins']['buckling'],a.get('supported_seq_ratio'),row['seq_cert_input'],row['margins']['sequence'],row['buckling_pass'],row['support_sequence_pass'],row['side_a4_pass'],a.get('status')]))
    lines+=['','| Case | Nodes | Elements | Bore nodes | Grip nodes | Parent binding | Mesh/thickness SHA256 |','| --- | --- | --- | --- | --- | --- | --- |']
    for row in rows:
        a=row['a4'] or {};lines.append(line([row['name'],a.get('n_node'),a.get('n_elem'),a.get('bore_support',{}).get('n_bore_nodes'),a.get('grip_foundation_stiffness',{}).get('n_grip_nodes'),row['parent_mesh_binding']['within_owner_bar'],row['mesh_sha256']]))
    lines+=['','The portable audit checks tracked results, input identity, certificate minima, both exact gate comparisons, margins, support constants, parent mesh bindings and published tables. Present raw outputs/mesh files must match hashes; missing target files are explicitly optional. It performs no native solve and needs no target directory, Git metadata, worker CPU affinity or Cargo settings.','']
    (HERE/'PHASE11.md').write_text('\n'.join(lines));(HERE/'phase11_receipt.json').write_text(json.dumps(r,indent=2)+'\n')
if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.environ['CARGO_BUILD_JOBS']=='2' and os.getpriority(os.PRIO_PROCESS,0)>=10
    P9=json.loads((HERE/'phase9_receipt.json').read_text());P10=json.loads((HERE/'phase10_receipt.json').read_text());P9_SHA=digest(HERE/'phase9_receipt.json');P10_SHA=digest(HERE/'phase10_receipt.json')
    assert P9['state']==P10['state']=='complete' and len(P9['results'])==len(P10['results'])==36
    GCI={v['name']:v for v in P10['results']};assert set(GCI)=={v['name'] for v in P9['results']}
    PROVENANCE=json.loads((OUT/'source_provenance.json').read_text());publish('running')
    milestone('milestone 1: production side.a4 study started','- Native build complete; all36 unchanged Phase9/10 profiles, exact production .25 mesh, seq_cert=min(.25,.125) from Phase9. Buckling>=1 and support sequence>=certificate checked independently; no native status substitution. CPUs12,13/nice prefix/jobs2, no native edits/gate changes. Do not commit or push.')
    with ThreadPoolExecutor(max_workers=2) as pool:
        futures={pool.submit(evaluate,p):p for p in P9['results']}
        for f in as_completed(futures):
            parent=futures[f]
            try:
                row=f.result();ROWS.append(row)
                milestone('case '+row['name'],f'- A4 complete={row["a4_complete"]}; eigenvalue={row["a4"]["buckling_eigenvalue"] if row["a4"] else None}; seq_a4={row["a4"]["supported_seq_ratio"] if row["a4"] else None}; seq_cert={row["seq_cert_input"]}; buckling={row["buckling_pass"]}; sequence={row["support_sequence_pass"]}; joint={row["side_a4_pass"]}; parent binding={row["parent_mesh_binding"]}; full qualification=false.')
            except Exception as e:ERRORS.append(dict(name=parent['name'],error=repr(e)))
            publish('running');print(parent['name'],len(ROWS),'errors',ERRORS,flush=True)
    assert len(ROWS)==36 and not ERRORS,ERRORS
    publish('complete');milestone('milestone 2: all native A4 observations retained','- All36 requested cases retained. Portable audits, shared-test metadata refresh and requested unit tests follow. No native/gate/threshold changes; no commit or push.')
