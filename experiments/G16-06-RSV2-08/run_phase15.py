#!/usr/bin/env python3
"""Run three unmodified production offspring, native plate and exported-skin P2 FEM."""
from concurrent.futures import ThreadPoolExecutor,as_completed
from datetime import datetime,timezone
from pathlib import Path
import json,os,subprocess
ROOT=Path(__file__).resolve().parents[2];OUT=ROOT/'target/G16-06-RSV2-08/phase15'
def milestone(title,detail):
    for rel in ['.agent/CHANGELOG.md','.agent/handoffs/G16-06-RSV2-08.md']:
        with (ROOT/rel).open('a') as f:f.write('\n## '+datetime.now(timezone.utc).isoformat(timespec='seconds')+' — Phase-15 '+title+'\n'+detail+'\n')
def run(slot):
    name=f'q7_slot{slot}_seed1'
    with (OUT/(name+'_plate.log')).open('w') as log:subprocess.run([str(OUT/'measure'),str(slot)],cwd=ROOT,env=ENV,stdout=log,stderr=log,check=True)
    plate=json.loads((OUT/(name+'_plate.json')).read_text())
    milestone('native plate '+name,'- Native geometry='+str(plate['geometry_rejects'])+'; aesthetics='+str(plate['aesthetic_pass'])+'; all3 native observations retained. Solid P2 standard RC30/NL4/NSUB1/PATCH2 follows; no metric substitution.')
    with (OUT/(name+'_solid.log')).open('w') as log:
        subprocess.run([PYTHON,'fem_solid/tip_solid.py',str((OUT/(name+'_D1.json')).relative_to(ROOT)),'30','4','1',str((OUT/(name+'_solid.json')).relative_to(ROOT))],cwd=ROOT,env=ENV,stdout=log,stderr=log,check=True)
    solid=json.loads((OUT/(name+'_solid.json')).read_text())
    milestone('solid P2 '+name,'- Standard exported-skin P2 solve converged='+str(solid['solver']['converged'])+'; nodal-max SF='+str(solid['solid']['SF_solid_nodal_max'])+'; native plate retained; actual-export Gate S/independent DOLFINx remain unverified, full qualification=false.')
    return name
if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.environ['CARGO_BUILD_JOBS']=='2' and os.getpriority(os.PRIO_PROCESS,0)>=10
    import sys
    PYTHON=sys.argv[1];ENV={k:v for k,v in os.environ.items() if not k.startswith(('JARAMBIT_','STUDY_')) and k not in ['PYTHONPATH','PATCH','COUNT_ONLY','DUMP_ONLY','DUMP_MESH']}
    ENV.update(JARAMBIT_SOLVER='faer',CARGO_BUILD_JOBS='2',CARGO_INCREMENTAL='0',CARGO_PROFILE_DEV_DEBUG='line-tables-only',RAYON_NUM_THREADS='2',OMP_NUM_THREADS='1',OPENBLAS_NUM_THREADS='1',MKL_NUM_THREADS='1')
    versions=subprocess.check_output([PYTHON,'-c','import json,numpy,scipy,skfem,pyamg;print(json.dumps(dict(numpy=numpy.__version__,scipy=scipy.__version__,skfem=skfem.__version__,pyamg=pyamg.__version__)))'],env=ENV,text=True)
    (OUT/'runtime.json').write_text(json.dumps(dict(versions=json.loads(versions),cpus=[12,13],nice=os.getpriority(os.PRIO_PROCESS,0),cargo_build_jobs=2,max_workers=2),indent=2)+'\n')
    subprocess.run([str(OUT/'measure'),'--generate'],cwd=ROOT,env=ENV,check=True)
    milestone('milestone 1: native generated slots started','- Production generator frozen and compiled unchanged; RNG1/c4_seed6mm/round1/12-child batch, slots8/10/11 exactly. Certification .25/.125 and strict .0625mm²; actual exported top facets at native1mm², standard P2 RC30/NL4/NSUB1/PATCH2. CPU12,13, nice -n10/effective'+str(os.getpriority(os.PRIO_PROCESS,0))+'/jobs2, two workers. No hand-built geometry or source/gate/threshold changes.')
    with ThreadPoolExecutor(max_workers=2) as pool:
        for f in as_completed([pool.submit(run,slot) for slot in [8,10,11]]):print('complete',f.result(),flush=True)
    milestone('milestone 2: all native/solid observations complete','- Three production offspring,9 native mesh observations and3 standard exported-skin P2 solves complete. Exact4-predicate audit, portable receipt and tests follow; no qualification claim or changed threshold.')
