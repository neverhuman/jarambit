#!/usr/bin/env python3
"""Sample reference skyline / faer parity; this does not certify the fine meshes."""
import hashlib,json,os,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
OUT=ROOT/'target/G16-06-RSV2-08'
if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13}
    config=json.loads((HERE/'candidate_config.json').read_text());config['STUDY_AREAS']='2'
    env={k:v for k,v in os.environ.items() if not k.startswith(('STUDY_','JARAMBIT_'))};env.update(config,JARAMBIT_SOLVER='faer',RAYON_NUM_THREADS='2',CARGO_BUILD_JOBS='2')
    path=OUT/'candidate_faer_parity.json'
    with (OUT/'candidate_faer_parity.log').open('w') as log:
        subprocess.run(['taskset','-c','12,13',str(OUT/'study'),'fem',str(path),str(HERE/'candidate.csv')],cwd=ROOT,env=env,stdout=log,stderr=log,check=True)
    primary=json.loads((OUT/'candidate_validation.json').read_text())
    reference=json.loads(path.read_text())
    assert primary['genes']==reference['genes'] and primary['solver']=='skyline' and reference['solver']=='faer'
    a=next(level['metrics'] for level in primary['levels'] if level['area']==2.)
    b=reference['levels'][0]['metrics']
    names=['lat_rob','fat_sf','sf_hunter','seq_ratio','twist_sf','twist_body_sf','twist_tipzone_sf','hm_twist_sf','hm_spine_sf','hm_ringback_sf']
    comparison={key:dict(skyline=a[key],faer=b[key],relative_difference=abs(a[key]-b[key])/max(abs(a[key]),abs(b[key]),1e-300)) for key in names}
    doc=dict(area_mm2=2.,scope='one unbiased coarse faer versus reference skyline sample; native owner bound 1e-9, not fine-mesh or 3D qualification',tolerance=1e-9,metrics=comparison)
    doc['pass']=all(row['relative_difference']<=doc['tolerance'] for row in comparison.values())
    doc['reference_sha256']=hashlib.sha256(path.read_bytes()).hexdigest()
    (OUT/'backend_parity.json').write_text(json.dumps(doc,indent=2)+'\n')
    print(json.dumps(doc,indent=2),flush=True)
    assert doc['pass']
