#!/usr/bin/env python3
"""Eight literal-parent/Q7 CP interpolations, fully evaluated without prefilters."""
from concurrent.futures import ThreadPoolExecutor, as_completed
import csv
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import struct
import subprocess
import threading
from run_phase2 import HERE, ROOT, environment, digest
from run_phase3 import REQUIREMENTS, verify_sources

OUT=ROOT/'target/G16-06-RSV2-08/phase4'
P2=OUT.parent/'phase2'
REPORT=Path('/home/ubuntu/jarambit-audits/2026-10-18-unified-prototype-v3-study/UNIFIED_PROTOTYPE_V3_REPORT.md')
TARGET={29:2.,30:1.73008,40:0.}
AREAS=[4.,1.,.25,.125,.0625]
LOCK=threading.Lock()

def now(): return datetime.now(timezone.utc).isoformat(timespec='seconds')

def milestone(title, detail):
    entry=f'\n## {now()} — Phase-4 {title}\n{detail}\n'
    for name in ['.agent/CHANGELOG.md','.agent/handoffs/G16-06-RSV2-08.md']:
        with (ROOT/name).open('a') as f:f.write(entry)

def event(name,stage):
    with LOCK:
        with (OUT/'progress.jsonl').open('a') as f:f.write(json.dumps(dict(time=now(),name=name,stage=stage))+'\n')
    print(name,stage,flush=True)

def invoke(binary,mode,path,genome,cert=None):
    env=environment();env['STUDY_AREAS']='4,1,.25,.125,.0625'
    # The entry process uses nice -n 10; child processes inherit its priority.
    command=['taskset','-c','12,13',str(binary),mode,str(path),str(genome)]
    if cert is not None:command.append(str(cert))
    with path.with_suffix('.log').open('w') as log:
        subprocess.run(command,cwd=ROOT,env=env,stdout=log,stderr=log,check=True)
    return json.loads(path.read_text())

def evaluate(job):
    name,parent,alpha,genome=job
    event(name,'gci')
    gp=OUT/f'{name}-gci.json';gci=invoke(P2/'study','gci',gp,genome)
    assert gci['genome_sha256']==digest(genome)
    assert gci['native_gci_source_sha256']==digest(ROOT/'crates/crescent/src/gci.rs')
    event(name,'export_and_all_five_meshes')
    fp=OUT/f'{name}-full.json';full=invoke(OUT.parent/'study','all',fp,genome)
    assert [r['area'] for r in full['levels']]==AREAS
    assert all(r['metrics'] is not None for r in full['levels'])
    verify_sources(full['provenance'])
    levels=full['levels'];fine=[l for l in levels if l['area']<=.25]
    keys=['hm_spine_sf','lat_rob','fat_sf','seq_ratio','sf_hunter','hm_twist_sf','hm_ringback_sf']
    cert={k:min(l['metrics'][k] for l in fine[:2]) for k in keys}
    strict={k:fine[2]['metrics'][k] for k in keys}
    event(name,'side_a4')
    ap=OUT/f'{name}-a4.json';side=invoke(P2/'study','a4',ap,genome,cert['seq_ratio']);a4=side['a4']
    mp=ap.with_suffix('.mesh.json');mesh=json.loads(mp.read_text())
    mesh_bytes=(b''.join(struct.pack('<dd',*q) for q in mesh['xy'])
                +b''.join(struct.pack('<QQQ',*t) for t in mesh['tris'])
                +b''.join(struct.pack('<d',t) for t in mesh['te']))
    glevels=gci['plate_gci']['levels_fine_to_coarse']
    assert hashlib.sha256(mesh_bytes).hexdigest()==glevels[0]['mesh_sha256']
    assert a4['certified_seq_ratio']==cert['seq_ratio']
    assert a4['same_mesh_ring_clamp_seq_ratio']==fine[0]['metrics']['seq_ratio']
    for gl in glevels:
        actual=next(l['metrics'] for l in levels if l['area']==gl['max_area_mm2'])
        assert actual['seq_ratio']==gl['seq_ratio'] and actual['sf_hunter']==gl['sf_hunter']
    m=full['measurement']
    export_ok=m['sharpness']['pass'] and m['point']['pass'] and full['solid']['census']==[0,0,1]
    legal=full['input_was_canonical'] and full['within_genome_domain'] and full['rs_v2'] and full['aes_pass'] and not full['rejects']
    a4_ok=a4['buckling_eigenvalue']>=1. and a4['supported_seq_ratio']>=cert['seq_ratio']
    limits={'hm_spine_sf':1.,'lat_rob':358.,'fat_sf':1.,'seq_ratio':REQUIREMENTS['constants']['seq_target'],'sf_hunter':1.377}
    fine_ok=all(l['metrics'][k]>=v for l in fine for k,v in limits.items())
    all_ok=all(l['metrics'][k]>=v for l in levels for k,v in limits.items())
    seq=[l['metrics']['seq_ratio'] for l in levels];diff=[b-a for a,b in zip(seq,seq[1:])]
    admitted=gci['plate_gci']['seq_ratio_gci']['admitted']
    row=dict(name=name,parent=parent,alpha=alpha,genome=str(genome.relative_to(ROOT)),genome_sha256=digest(genome),
             genes=full['genes'],config=full['config'],solver=full['solver'],geometry_legal=legal,rejects=full['rejects'],
             aesthetic_failures=full['aes_fails'],levels=levels,certification=cert,strict=strict,
             all_five_minimum={k:min(l['metrics'][k] for l in levels) for k in keys},
             GCI=gci['plate_gci'],gci_admitted=admitted,export_pass=export_ok,
             sharpness=m['sharpness'],point={k:v for k,v in m['point'].items() if k!='samples'},
             bore_edge_radius_mm=m['bore_edge_radius_mm'],solid=full['solid'],a4=a4,side_a4_pass=a4_ok,
             neck_section=full['neck_section'],mass_g=full['mass_g'],fine_target_pass=fine_ok,
             all_five_target_pass=all_ok,all_five_sequence_monotonic=all(d>0 for d in diff) or all(d<0 for d in diff),
             scoped_joint_pass=bool(legal and fine_ok and admitted and export_ok and a4_ok),all_gate_qualification=False,
             adapter_note='unchanged native Phase-2 GCI/A4 adapter reused; raw task label remains p2',
             sha256={str(p.relative_to(ROOT)):digest(p) for p in [genome,gp,fp,ap,mp]})
    (OUT/f'{name}-summary.json').write_text(json.dumps(row,indent=2)+'\n')
    event(name,'complete');return row

def publish(rows,errors,complete):
    rows=sorted(rows,key=lambda r:ORDER.index(r['name']))
    receipt=dict(task='G16-06-RSV2-08-p4',state='complete' if complete else 'running',expected_cases=8,
                 completed_cases=len(rows),errors=errors,cpus=[12,13],cargo_build_jobs=2,
                 nice_value=os.getpriority(os.PRIO_PROCESS,0),max_concurrent_evaluations=2,
                 targets_by_gene_index=TARGET,alphas=[.25,.5,.75,1.],areas_mm2=AREAS,
                 interpretation='two exact named window parents; interpolate only literal CP29/CP30/drop; CP06..08 and every other gene preserved',
                 assignment_discrepancy='local_04 has S1=.500, although assignment parenthetical says S1=.502..525; exact admissible-GCI parent retained',
                 section_law='sealed candidate_config.json and original lane construction law; Q7 external law/solver not imported',
                 thresholds={'hm_spine_sf':1.,'lat_rob':358.,'fat_sf':1.,'seq_ratio':REQUIREMENTS['constants']['seq_target'],'sf_hunter':1.377},
                 scope='certification min(.25,.125) plus strict .0625; all-five observations and minima reported independently',
                 results=rows,all_gate_qualification=False,
                 Q7_report=dict(source=str(REPORT),snapshot='target/G16-06-RSV2-08/phase4/inputs/Q7_REPORT.md',sha256=digest(OUT/'inputs/Q7_REPORT.md')),
                 sha256={str(p.relative_to(ROOT)):digest(p) for p in [HERE/'run_phase4.py',HERE/'candidate.csv',HERE/'candidate_config.json',
                     HERE/'phase3_candidates/placement_05.csv',HERE/'phase3_candidates/local_04.csv',OUT.parent/'study',P2/'study']})
    (HERE/'phase4_receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
    lines=['# G16-06-RSV2-08 Phase 4','',f'Status: {receipt["state"]}; {len(rows)}/8 complete. CPUs12,13; nice={receipt["nice_value"]}; CARGO_BUILD_JOBS=2; skyline plate backend. No physical source, threshold, load or support constant changed.','',
           'Eight cases use the two exact Phase-3 named parents, each at alpha=.25/.50/.75/1 toward CP29=2, CP30=1.73008 and blade_drop_deg=0. Both parent drops are already zero; only CP29/30 change numerically. CP06..08 and all other genes remain exactly parental. CP02..05/CP23..25 are already3. No spatial remapping of literal CP indices or Q7 section/solver import is performed.','',
           'Assignment discrepancy: local_04 S1 is **.500**, while the assignment parenthetical range starts at .502. The exact named admissible-GCI parent is preserved (.052/.500/fill.65); placement_05 uses .05/.525/fill.70. All windows remain width6. The external Q7 report uses a different section law and explicit faer; its reported HM spine margins are contextual and are not asserted to transfer to this study.','',
           '| Case | Alpha | Cert spine | Strict spine | Cert lat N | Strict lat N | Cert fatigue | Strict fatigue | Cert seq | Strict seq | GCI | Export | A4 | Joint |',
           '| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |']
    for r in rows:
        c,t=r['certification'],r['strict']
        lines.append(f'| {r["name"]} | {r["alpha"]} | {c["hm_spine_sf"]:.6f} | {t["hm_spine_sf"]:.6f} | {c["lat_rob"]:.6f} | {t["lat_rob"]:.6f} | {c["fat_sf"]:.6f} | {t["fat_sf"]:.6f} | {c["seq_ratio"]:.6f} | {t["seq_ratio"]:.6f} | {r["gci_admitted"]} | {r["export_pass"]} | {r["side_a4_pass"]} | {r["scoped_joint_pass"]} |')
    lines+=['','Joint scope requires geometry, HM spine>=1, lateral>=358N, fatigue>=1, sequence>=1.20 and hunter>=1.377 at certification/strict, native GCI admission, export and side-A4. Every one of the five mesh observations is retained; coarse failures are never waived or substituted for certification. Full RS-v2/independent-solid/manufacturing qualification remains false.','',
            'Raw inputs, full measurements, FEM diagnostics and meshes are retained under target/G16-06-RSV2-08/phase4/. No early numerical prefilter is applied. All five meshes, actual export and side-A4 are evaluated even after an earlier failure.','',
            'Reproduce using retained lane binaries:','```sh','nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/run_phase4.py','```','']
    (HERE/'PHASE4.md').write_text('\n'.join(lines))

if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10
    assert os.environ.get('CARGO_BUILD_JOBS')=='2'
    OUT.mkdir(parents=True,exist_ok=True);(OUT/'inputs').mkdir(exist_ok=True)
    assert not (OUT/'progress.jsonl').exists(),'preserve existing Phase-4 evidence before another run'
    (OUT/'inputs/Q7_REPORT.md').write_bytes(REPORT.read_bytes())
    sealed=json.loads((HERE/'candidate_receipt.json').read_text());prior=json.loads((P2/'baseline_receipt.json').read_text())
    for p in [HERE/'candidate.csv',HERE/'candidate_config.json',OUT.parent/'study']:
        assert digest(p)==sealed['sha256'][str(p.relative_to(ROOT))]
    assert digest(P2/'study')==prior['sha256'][str((P2/'study').relative_to(ROOT))]
    verify_sources(json.loads((P2/'sealed-gci.json').read_text())['provenance'])
    jobs=[];assembly=[]
    for parent in ['placement_05','local_04']:
        parent_path=HERE/'phase3_candidates'/f'{parent}.csv';base=list(csv.reader(parent_path.open()))
        for alpha in [.25,.5,.75,1.]:
            name=f'{parent}_a{alpha:.2f}';genome=OUT/'inputs'/f'{name}.csv';data=[list(r) for r in base]
            for i,target in TARGET.items():
                value=(1-alpha)*float(base[1][i])+alpha*target
                if value!=float(base[1][i]):data[1][i]=str(value)
            assert [i for i,(a,b) in enumerate(zip(base[1],data[1])) if a!=b]==[29,30]
            assert data[1][6:9]==base[1][6:9]
            with genome.open('w') as f:csv.writer(f,lineterminator='\n').writerows(data)
            jobs.append((name,parent,alpha,genome))
            assembly.append(dict(name=name,parent=str(parent_path.relative_to(ROOT)),alpha=alpha,
                changed_indices=[29,30],parent_hand_cp06_08=base[1][6:9],input_hand_cp06_08=data[1][6:9],
                target_values=TARGET,result_values={i:float(data[1][i]) for i in TARGET},genome_sha256=digest(genome)))
    (OUT/'assembly.json').write_text(json.dumps(assembly,indent=2)+'\n')
    ORDER=[j[0] for j in jobs];rows=[];errors=[];publish(rows,errors,False)
    milestone('milestone 1: eight-case literal-parent study started',
              '- Read Phase-4 Assignment and full95-line Q7 report. Exact named parents retained, including local_04 S1=.500 despite the assignment range starting.502. Interpolate CP29/30/drop only at alpha.25/.5/.75/1; drop already0, handCP06..08 byte-preserved, actual changed genes29/30 only. All other genes/config and native physics remain unchanged.\n- All8 cases receive all5 meshes4/1/.25/.125/.0625, actual export and side.a4, without numerical prefilters. Reference skyline; process started via nice -n10 taskset -c12,13, Cargo jobs2; children inherit nice priority and CPU affinity. Q7 report snapshotted/hashed, no Q7 faer/section law imported. No PR/push/stacking/deletes/gate relaxation.')
    with ThreadPoolExecutor(max_workers=2) as pool:
        futures={pool.submit(evaluate,j):j[0] for j in jobs}
        for future in as_completed(futures):
            name=futures[future]
            try:
                row=future.result();rows.append(row)
                milestone(f'case {name} complete',f'- All5 FEM meshes plus export/A4 evaluated. Cert={row["certification"]}; strict={row["strict"]}. GCI={row["gci_admitted"]}, export={row["export_pass"]}, A4={row["side_a4_pass"]}, joint={row["scoped_joint_pass"]}. Full qualification=false. Complete raw diagnostics retained.')
            except Exception as e:
                errors.append(dict(name=name,error=repr(e)));event(name,'ERROR '+repr(e))
            publish(rows,errors,False)
    assert len(rows)==8 and not errors,errors
    publish(rows,errors,True)
    milestone('milestone 10: all eight cases evaluated',f'- Coverage8/8, five requested meshes40 plus8 exports/8 side.a4, errors0. Scoped joint passes={[r["name"] for r in rows if r["scoped_joint_pass"]]}. PHASE4.md/phase4_receipt.json written. Thresholds/native source unchanged; full qualification=false.')
