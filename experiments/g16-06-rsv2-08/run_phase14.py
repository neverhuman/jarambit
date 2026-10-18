#!/usr/bin/env python3
"""Run every assigned native seed offspring; retain every native failure."""
from concurrent.futures import ThreadPoolExecutor,as_completed
from datetime import datetime,timezone
from pathlib import Path
import csv,hashlib,itertools,json,os,subprocess
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];OUT=ROOT/'target/g16-06-rsv2-08/phase14'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def milestone(title,detail):
    for rel in ['.agent/CHANGELOG.md','.agent/handoffs/G16-06-RSV2-08.md']:
        with (ROOT/rel).open('a') as f:f.write('\n## '+datetime.now(timezone.utc).isoformat(timespec='seconds')+' — Phase-14 '+title+'\n'+detail+'\n')
def evaluate(job):
    name,dn,ea,path=job;output=OUT/f'{name}.json';assert not output.exists()
    with output.with_suffix('.log').open('w') as log:
        subprocess.run([str(OUT/'measure'),str(path.relative_to(ROOT)),str(output.relative_to(ROOT)),str((HERE/'phase14_candidates/fixed_seed.csv').relative_to(ROOT))],cwd=ROOT,env=ENV,stdout=log,stderr=log,check=True)
    d=json.loads(output.read_text());assert 'seconds' in d and d['source_provenance']==PROVENANCE
    for rel,want in PROVENANCE.items():assert digest(ROOT/rel)==want,rel
    assert digest(path)==d['genome_sha256']
    row={k:v for k,v in d.items() if k!='source_provenance'}
    row.update(dn37_mm=dn,ea_deg=ea,raw=str(output.relative_to(ROOT)),raw_sha256=digest(output))
    return row
if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.environ['CARGO_BUILD_JOBS']=='2' and os.getpriority(os.PRIO_PROCESS,0)>=10
    ENV={k:v for k,v in os.environ.items() if not k.startswith(('JARAMBIT_','STUDY_'))}
    ENV.update(JARAMBIT_SOLVER='faer',CARGO_BUILD_JOBS='2',CARGO_INCREMENTAL='0',CARGO_PROFILE_DEV_DEBUG='line-tables-only',RAYON_NUM_THREADS='2')
    PROVENANCE=json.loads((OUT/'source_provenance.json').read_text());inputs=HERE/'phase14_candidates';inputs.mkdir()
    subprocess.run([str(OUT/'measure'),'--seed',str(inputs.relative_to(ROOT))],cwd=ROOT,env=ENV,check=True)
    base=list(csv.reader((inputs/'fixed_seed.csv').read_text().splitlines()));jobs=[];rows=[];errors=[]
    for dn,ea in itertools.product([.3,.4,.5,.6],[18.,18.2,18.4,18.6]):
        name=f'dn{dn:.2f}_ea{ea:.1f}';data=[list(v) for v in base]
        for i,v in [(36,.65*dn),(37,dn),(38,.80*dn),(48,ea),(49,ea)]:data[1][i]=repr(v)
        path=inputs/f'{name}.csv'
        with path.open('w') as f:csv.writer(f,lineterminator='\n').writerows(data)
        jobs.append((name,dn,ea,path))
    milestone('milestone 1: native grid started','- Native seed/build passed. All16 assigned cells generated from canonical native c4_seed(6.0)+P1 fixed seed; offspring are never clamped. Proceeding with stated native4/1/.25mm² GCI definitions after clarification opportunity. Geometry/aesthetic failures will be retained. CPUs12,13/nice -n10/effective'+str(os.getpriority(os.PRIO_PROCESS,0))+'/jobs2; two native workers; no native source/gate/threshold changes.')
    with ThreadPoolExecutor(max_workers=2) as pool:
        futures={pool.submit(evaluate,j):j for j in jobs}
        for f in as_completed(futures):
            try:
                row=f.result();rows.append(row)
                milestone('case '+row['name'],f'- Geometry rejects={row["geometry_rejects"]}; aesthetic pass={row["aesthetic_pass"]}, failures={row["aesthetic_failures"]}; fine={row["levels"][-1]["result"]}; tip GCI={row.get("tip_gci",{}).get("fit_status")}, torsional GCI={row.get("torsional_gci",{}).get("fit_status")}; full qualification=false.')
            except Exception as e:errors.append(dict(name=futures[f][0],error=repr(e)))
            (OUT/'observations.json').write_text(json.dumps(dict(results=sorted(rows,key=lambda r:r['name']),errors=errors),indent=2)+'\n')
            print(len(rows),'of16; errors',errors,flush=True)
    assert len(rows)==16 and not errors,errors
    milestone('milestone 2: native observations complete','- All16 cases/48 native mesh observations complete and retained. Portable verifier, exact GCI reference replay, report and metadata-only lineage refresh follow. No qualification claim or gate/threshold changes. Leave uncommitted.')
