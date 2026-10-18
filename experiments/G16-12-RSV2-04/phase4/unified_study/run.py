from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import json,os,subprocess,sys,time
OUT=Path(__file__).resolve().parent
ENV={k:v for k,v in os.environ.items() if not k.startswith(('STUDY_','JARAMBIT_'))}
ENV.update(CARGO_BUILD_JOBS='2',JARAMBIT_SOLVER='faer',RAYON_NUM_THREADS='2')
assert os.sched_getaffinity(0)=={10,11}
neck=json.loads((OUT/'inputs/neck/experiments/G16-06-RSV2-08/candidate_config.json').read_text())
hot_summary=json.loads((OUT/'inputs/hotspot/target/rsv2-07/v3/hotspot_strict_summary.json').read_text())
hot=next(r for r in hot_summary if r['name']=='hotspot_strict_3')
combined=json.loads((OUT/'combined_config.json').read_text())
CASES={
 'neck_control':('neck_control.csv',neck),
 'hotspot_control':('hotspot_control.csv',{**{'STUDY_'+k:str(v) for k,v in hot['config'].items()},'STUDY_AREAS':'.0625'}),
 'combined_d065_h025':('combined.csv',{**combined,'STUDY_D0':'.65','STUDY_D1':'1.1','STUDY_GRID':'.025'}),
 'combined_d065':('combined.csv',{**combined,'STUDY_D0':'.65','STUDY_D1':'1.1'}),
 'combined_d055':('combined.csv',{**combined,'STUDY_D0':'.55','STUDY_D1':'.85'}),
 'combined_d048':('combined.csv',{**combined,'STUDY_D0':'.48','STUDY_D1':'.65'})}
def run(name,mode):
 seed,original_config=CASES[name];config={**original_config,'JARAMBIT_SOLVER':'faer'};out=OUT/(name+'_'+mode+'.json');cmd=[str(OUT/'study'),mode,str(out),str(OUT/seed)]
 invocation={'command':cmd,'config':config,'cpu_affinity':sorted(os.sched_getaffinity(0)),'CARGO_BUILD_JOBS':2,'solver':'faer'}
 (OUT/(name+'_'+mode+'_invocation.json')).write_text(json.dumps(invocation,indent=2)+'\n')
 with out.with_suffix('.log').open('w') as log:subprocess.run(cmd,env={**ENV,**config},stdout=log,stderr=log,check=True)
 d=json.loads(out.read_text());print(name,mode,[(l['area'],{k:l['metrics'][k] for k in ['lat_rob','fat_sf','seq_ratio','hm_spine_sf','hm_twist_sf']}) for l in d.get('levels',[])],flush=True)
 return d
if __name__=='__main__':
 mode=sys.argv[1];names=sys.argv[2:]
 with ThreadPoolExecutor(max_workers=2) as pool:list(pool.map(lambda n:run(n,mode),names))
