"""Acquire one pending immutable case, or assemble completed Phase-6 artifacts."""
from pathlib import Path
import json
import os
import sys
import solid_tip_3d as base

ROOT,HERE=base.ROOT,base.HERE
sha=base.digest
def write(p,d):p.write_text(json.dumps(d,indent=2,allow_nan=False)+'\n')


def main():
    os.sched_setaffinity(0,{14,15});os.setpriority(os.PRIO_PROCESS,0,10)
    out=Path(sys.argv[1]).resolve()
    provenance=json.loads((out/'provenance.json').read_text())
    cases=json.loads((out/'cases.json').read_text())
    for f,h in provenance['source_sha256'].items():assert sha(ROOT/f)==h,f
    for c in cases.values():
        assert sha(Path(c['binary']))==c['binary_sha256']
        for f,h in c['generated_sha256'].items():assert sha(ROOT/f)==h,f
    if len(sys.argv)>2:
        name=sys.argv[2];c=cases[name];folder=out/name;binary=c['binary'];seed=c['seed']
        env={k:v for k,v in os.environ.items() if not k.startswith(('STUDY_','JARAMBIT_')) and k not in
            ['COUNT_ONLY','DUMP_ONLY','DUMP_MESH','PATCH','PYTHONPATH','CARGO_TARGET_DIR']}
        config=json.loads((folder/'config.json').read_text());config=config.get('config',config)
        env.update(provenance['environment'],**config)
        env.update(JARAMBIT_SOLVER='faer',TMPDIR=str(ROOT/'target/rsv2-07/compiler-tmp'))
        write(folder/'finish_provenance.json',{'source':str(Path(__file__).relative_to(ROOT)),
            'sha256':sha(Path(__file__)),'cpu_affinity':sorted(os.sched_getaffinity(0)),
            'nice':os.getpriority(os.PRIO_PROCESS,0),'source_and_binary_hashes_verified':True})
        invocations=[]
        def invoke(command,log):
            r=base.run(command,log,env);invocations.append(r)
            write(folder/'finish_invocations.json',invocations)
            assert r['exit_code']==0,(log,r['exit_code'])
        invoke([binary,seed,str(folder),name],folder/'plate.log')
        for sub in ['cad','replay']:
            dest=folder/sub;dest.mkdir()
            invoke([binary,seed,str(dest),name,'export'],dest/'export.log')
        cad=folder/'cad'
        deterministic=all(sha(cad/f)==sha(folder/'replay'/f) for f in ['candidate.step','candidate.stl','surface.json'])
        write(folder/'determinism.json',{'deterministic':deterministic,
            'sha256':{f:sha(cad/f) for f in ['candidate.step','candidate.stl','surface.json']}})
        assert deterministic
        invoke(['/home/ubuntu/.venvs/jarambit-cad/bin/python',str(HERE/'phase6_cad_check.py'),str(cad)],cad/'independent_check.log')
        for f,h in provenance['source_sha256'].items():assert sha(ROOT/f)==h,f
        print('COMPLETE CASE',name,flush=True)
        return
    prior=json.loads((HERE/'results/phase5_witness_3d_validation.json').read_text())
    results={}
    for name,c in cases.items():
        folder=out/name;cad=folder/'cad'
        solid=json.loads((folder/'solid/result.json').read_text()) if name=='Q7_window_p05' else prior['candidates'][name]
        assert sha(cad/'surface.json')==solid['source_surface_sha256']
        assert all(sha(cad/f)==sha(folder/'replay'/f) for f in ['candidate.step','candidate.stl','surface.json'])
        plate=json.loads((folder/'plate.json').read_text());assert len(plate['levels_coarse_to_fine'])==5
        assert 'gci' in plate and 'side_a4' in plate and 'certification' in plate
        results[name]={'plate':plate,'export':json.loads((cad/'export.json').read_text()),
            'cad_check':json.loads((cad/'cad_check.json').read_text()),'deterministic':True,
            'solid':solid,'candidate_sha256':sha(folder/'candidate.csv'),
            'candidate_file':str((folder/'candidate.csv').relative_to(ROOT))}
    write(out/'results.json',results)
    write(out/'finish_assembly.json',{'source':str(Path(__file__).relative_to(ROOT)),
        'sha256':sha(Path(__file__)),'all_scientific_source_and_binary_hashes_verified':True})
    print('ASSEMBLED',out,flush=True)


if __name__=='__main__':main()
