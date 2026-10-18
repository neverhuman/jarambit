#!/usr/bin/env python3
"""Compile the audit against verified lane-local native libraries."""
import hashlib,json,os,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent;OUT=ROOT/'target/G16-06-RSV2-08/phase15'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10 and os.environ['CARGO_BUILD_JOBS']=='2'
    assert not (OUT/'measure').exists();OUT.mkdir(exist_ok=True,parents=True)
    refs=json.loads((ROOT/'experiments/g16-06-rsv2-08/phase14_receipt.json').read_text())['source_provenance']
    for rel,want in refs.items():assert digest(ROOT/rel)==want,rel
    flags=[];libs={}
    for name in ['jarambit_crescent','jarambit_genotype','jarambit_physics','jarambit_evidence','serde_json']:
        matches=list((ROOT/'target/release/deps').glob('lib'+name+'-*.rlib'));assert len(matches)==1
        flags+=['--extern',name+'='+str(matches[0])];libs[str(matches[0].relative_to(ROOT))]=digest(matches[0])
    for p in [HERE/'phase15.rs',*list((HERE/'phase15_sources').iterdir()),ROOT/'fem_solid/tip_solid.py',ROOT/'fem_solid/export_surface.py']:
        refs[str(p.relative_to(ROOT))]=digest(p)
    (OUT/'source_provenance.json').write_text(json.dumps(refs,indent=2)+'\n')
    env=os.environ.copy();env.update(CARGO_INCREMENTAL='0',CARGO_PROFILE_DEV_DEBUG='line-tables-only',TMPDIR=str(OUT/'compiler-tmp'));(OUT/'compiler-tmp').mkdir(exist_ok=True)
    subprocess.run(['rustc','--edition=2021','-O','-C','lto=thin',str(HERE/'phase15.rs'),'-L','dependency='+str(ROOT/'target/release/deps'),*flags,'-o',str(OUT/'measure')],env=env,check=True)
    (OUT/'build_provenance.json').write_text(json.dumps(dict(libraries=libs,rustc=subprocess.check_output(['rustc','--version'],text=True).strip(),binary_sha256=digest(OUT/'measure')),indent=2)+'\n')
