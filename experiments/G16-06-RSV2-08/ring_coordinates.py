#!/usr/bin/env python3
"""Probe legal ring/holder coordinates before solving the native FEM."""
from concurrent.futures import ThreadPoolExecutor
import csv,json,os,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
OUT=ROOT/'target/G16-06-RSV2-08'
FOLDER=OUT/'ring_coordinates'
FOLDER.mkdir(exist_ok=True)

def evaluate(job, mode='geometry'):
    wall,drop,cp13=job
    name=f'r{wall}_drop{drop}_cp13{cp13}' + ('_fine' if mode=='fem' else '')
    rows=list(csv.reader((HERE/'candidate.csv').open()))
    for k,v in {42:wall,40:drop,13:cp13}.items():rows[1][k]=str(v)
    seed=FOLDER/(name+'.csv')
    with seed.open('w') as f:csv.writer(f,lineterminator='\n').writerows(rows)
    cfg=json.loads((HERE/'candidate_config.json').read_text());cfg.update(STUDY_BLADE_D1='0',STUDY_AREAS='.125')
    env={k:v for k,v in os.environ.items() if not k.startswith(('STUDY_','JARAMBIT_'))};env.update(cfg,JARAMBIT_SOLVER='faer',RAYON_NUM_THREADS='2',CARGO_BUILD_JOBS='2')
    path=FOLDER/(name+'.json')
    with path.with_suffix('.log').open('w') as f:subprocess.run(['taskset','-c','12,13',str(OUT/'study'),mode,str(path),str(seed)],env=env,stdout=f,stderr=f,check=True)
    d=json.loads(path.read_text());legal=d['input_was_canonical'] and d['within_genome_domain'] and d['rs_v2'] and d['aes_pass'] and not d['rejects']
    result=dict(job=job,legal=legal,rejects=d['rejects'],aes_fails=d['aes_fails'],source=str(seed.relative_to(ROOT)))
    if mode=='fem':result.update(metrics=d['levels'][0]['metrics'],diagnostic=d['levels'][0]['diagnostic_peaks'])
    print(json.dumps(result),flush=True);return result

if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13}
    jobs=[(wall,drop,cp13) for wall in (6.4,6.7,7.) for drop in (-1.,0.,.5,1.) for cp13 in (-1.5,-3.5)]
    with ThreadPoolExecutor(max_workers=2) as pool:geometry=list(pool.map(evaluate,jobs))
    (OUT/'ring_geometry_summary.json').write_text(json.dumps(geometry,indent=2)+'\n')
    legal=[tuple(d['job']) for d in geometry if d['legal']]
    with ThreadPoolExecutor(max_workers=2) as pool:fine=list(pool.map(lambda j:evaluate(j,'fem'),legal))
    (OUT/'ring_fine_summary.json').write_text(json.dumps(fine,indent=2)+'\n')
