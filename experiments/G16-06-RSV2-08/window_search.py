#!/usr/bin/env python3
"""Find a controlled blade section using existing native window genes."""
from concurrent.futures import ThreadPoolExecutor
import csv,json,os,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
OUT=ROOT/'target/G16-06-RSV2-08'
FOLDER=OUT/'window_search'
FOLDER.mkdir(exist_ok=True)
SEED=OUT/'ring_coordinates/r6.7_drop0.0_cp13-1.5_fine.canonical.csv'
def run(job, areas=".125", wall=6.7, drop=0.):
    width,fill=job
    name=f'window_w{width}_fill{fill}' + (f'_r{wall}_drop{drop}_refined' if areas!='.125' or wall!=6.7 or drop!=0. else '')
    rows=list(csv.reader(SEED.open()))
    for k,v in {42:wall,40:drop,99:1.,100:.05,101:.5,102:width,103:fill}.items():rows[1][k]=str(v)
    seed=FOLDER/(name+'.csv')
    with seed.open('w') as f:csv.writer(f,lineterminator='\n').writerows(rows)
    cfg=json.loads((HERE/'candidate_config.json').read_text());cfg.update(STUDY_BLADE_D1='0',STUDY_BLADE_CORE='6',STUDY_HEEL_RUNOUT='5',STUDY_WINDOW_CURRENT='1',STUDY_AREAS=areas)
    env={k:v for k,v in os.environ.items() if not k.startswith(('STUDY_','JARAMBIT_'))};env.update(cfg,JARAMBIT_SOLVER='faer',RAYON_NUM_THREADS='2',CARGO_BUILD_JOBS='2')
    path=FOLDER/(name+'.json')
    with path.with_suffix('.log').open('w') as f:subprocess.run(['taskset','-c','12,13',str(OUT/'study'),'fem',str(path),str(seed)],env=env,stdout=f,stderr=f,check=True)
    d=json.loads(path.read_text());legal=d['input_was_canonical'] and d['within_genome_domain'] and d['rs_v2'] and d['aes_pass'] and not d['rejects'] and 'window' in d['holes']
    result=dict(job=job,legal=legal,holes=d['holes'],hole_x_bounds=d['hole_x_bounds'],rejects=d['rejects'],aes_fails=d['aes_fails'],config=cfg,levels=d['levels'],metrics=d['levels'][0]['metrics'],receipt=str(path.relative_to(ROOT)))
    print(json.dumps(result),flush=True);return result
if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13}
    jobs=[(width,fill) for width in (6.,8.,10.) for fill in (.45,.6,.8)]
    with ThreadPoolExecutor(max_workers=2) as pool:results=list(pool.map(run,jobs))
    (OUT/'window_summary.json').write_text(json.dumps(results,indent=2)+'\n')
