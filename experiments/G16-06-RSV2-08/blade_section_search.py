#!/usr/bin/env python3
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import json,os,subprocess,sys
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
OUT=ROOT/'target/G16-06-RSV2-08'
def run(d1):
    cfg=json.loads((HERE/'candidate_config.json').read_text())
    cfg.update(STUDY_BLADE_D1=str(d1),STUDY_AREAS='.125')
    env={k:v for k,v in os.environ.items() if not k.startswith(('STUDY_','JARAMBIT_'))}
    env.update(cfg,JARAMBIT_SOLVER='faer',RAYON_NUM_THREADS='2',CARGO_BUILD_JOBS='2')
    path=OUT/f'blade_d1_{d1}.json'
    with path.with_suffix('.log').open('w') as f:
        subprocess.run(['taskset','-c','12,13',str(OUT/'study'),'fem',str(path),str(HERE/'candidate.csv')],env=env,stdout=f,stderr=f,check=True)
    d=json.loads(path.read_text());m=d['levels'][0]['metrics']
    result=dict(d1=d1,legal=d['input_was_canonical'] and d['within_genome_domain'] and d['rs_v2'] and d['aes_pass'] and not d['rejects'],metrics=m,diagnostic=d['levels'][0]['diagnostic_peaks'])
    print(json.dumps(result),flush=True);return result
if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13}
    with ThreadPoolExecutor(max_workers=2) as pool:results=list(pool.map(run,[float(x) for x in sys.argv[1:]] or [1.15,1.2,1.25,1.3,1.4]))
    (OUT/('blade_section_'+('_'.join(sys.argv[1:]) or 'wide')+'_summary.json')).write_text(json.dumps(results,indent=2)+'\n')
