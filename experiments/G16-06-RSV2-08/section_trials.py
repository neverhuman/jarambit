#!/usr/bin/env python3
from concurrent.futures import ThreadPoolExecutor
import json,os,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
OUT=ROOT/'target/G16-06-RSV2-08'
SEED=OUT/'ring_coordinates/r6.7_drop0.0_cp13-1.5_fine.canonical.csv'
def run(job):
    core,runout=job
    cfg=json.loads((HERE/'candidate_config.json').read_text());cfg.update(STUDY_BLADE_D1='0',STUDY_BLADE_CORE=str(core),STUDY_HEEL_RUNOUT=str(runout),STUDY_AREAS='.25,.125,.0625')
    env={k:v for k,v in os.environ.items() if not k.startswith(('STUDY_','JARAMBIT_'))};env.update(cfg,JARAMBIT_SOLVER='faer',RAYON_NUM_THREADS='2',CARGO_BUILD_JOBS='2')
    path=OUT/f'section_core{core}_runout{runout}.json'
    with path.with_suffix('.log').open('w') as f:subprocess.run(['taskset','-c','12,13',str(OUT/'study'),'fem',str(path),str(SEED)],env=env,stdout=f,stderr=f,check=True)
    d=json.loads(path.read_text());metrics=[level['metrics'] for level in d['levels']]
    legal=d['input_was_canonical'] and d['within_genome_domain'] and d['rs_v2'] and d['aes_pass'] and not d['rejects']
    passed=legal and all(f['lat_rob']>=358. and f['sf_hunter']>=1.377 and f['fat_sf']>=1. and f['seq_ratio']>=1.25 for f in metrics)
    result=dict(job=job,task_pass=passed,legal=legal,config=cfg,levels=metrics,receipt=str(path.relative_to(ROOT)))
    print(json.dumps(result),flush=True);return result
if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13}
    with ThreadPoolExecutor(max_workers=2) as pool:results=list(pool.map(run,[(5.8,.25),(5.7,.5),(5.6,.25)]))
    (OUT/'section_trials_summary.json').write_text(json.dumps(results,indent=2)+'\n')
