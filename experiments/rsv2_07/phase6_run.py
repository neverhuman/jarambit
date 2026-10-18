"""RSV2-07-p6 evidence acquisition; no production solver or gate edits."""
from pathlib import Path
import csv
import datetime
import importlib.util
import json
import os
import shutil
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor

import solid_tip_3d as base
import solid_tip_3d_p5 as p5

ROOT, HERE = base.ROOT, base.HERE
sha = base.digest


def write(p, obj):
    p.write_text(json.dumps(obj, indent=2, allow_nan=False)+'\n')


def main():
    os.sched_setaffinity(0,{14,15})
    os.setpriority(os.PRIO_PROCESS,0,10)
    base.PRIMARY_ADDRESS_SPACE = 64*1024**3
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
    out = ROOT/'target/rsv2-07/phase6'/stamp
    out.mkdir(parents=True,exist_ok=False)
    env = {k:v for k,v in os.environ.items() if not k.startswith(('STUDY_','JARAMBIT_')) and k not in
        ['COUNT_ONLY','DUMP_ONLY','DUMP_MESH','PATCH','PYTHONPATH','CARGO_TARGET_DIR']}
    env.update(CARGO_BUILD_JOBS='2',JARAMBIT_SOLVER='faer',RAYON_NUM_THREADS='2',
        OMP_NUM_THREADS='1',OPENBLAS_NUM_THREADS='1',MKL_NUM_THREADS='1',
        TMPDIR=str(ROOT/'target/rsv2-07/compiler-tmp'))
    env.setdefault('CARGO_INCREMENTAL','0')
    env.setdefault('CARGO_PROFILE_DEV_DEBUG','line-tables-only')
    prior_path=HERE/'results/phase5_witness_3d_validation.json'
    prior=json.loads(prior_path.read_text())
    for f,h in prior['provenance']['source_sha256'].items(): assert sha(ROOT/f)==h,f
    for c in prior['candidates'].values():
        for f,h in c['artifacts_sha256'].items():assert sha(ROOT/f)==h,f
    sources=[*ROOT.glob('crates/*/src/**/*.rs'),*ROOT.glob('crates/*/Cargo.toml'),
        *ROOT.glob('requirements/RS-karambit-*.toml'),*ROOT.glob('fem_solid/*.py'),
        ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'rust-toolchain.toml',ROOT/'docs/WORKPLAN.md',
        ROOT/'ops/ci/evaluate_cad_candidate.py',ROOT/'ops/ci/verify_genome_export.py',
        ROOT/'ops/ci/cad_battery.py',ROOT/'crates/cli/src/bench.rs',prior_path,
        HERE/'phase6_probe.rs',HERE/'phase6_cad_check.py',Path(__file__)]
    provenance={'commit':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),
        'source_sha256':{str(p.relative_to(ROOT)):sha(p) for p in sorted(set(sources))},
        'cpu_affinity':sorted(os.sched_getaffinity(0)),'nice':os.getpriority(os.PRIO_PROCESS,0),
        'environment':{k:env[k] for k in ['CARGO_BUILD_JOBS','JARAMBIT_SOLVER','RAYON_NUM_THREADS',
            'OMP_NUM_THREADS','OPENBLAS_NUM_THREADS','MKL_NUM_THREADS','CARGO_INCREMENTAL','CARGO_PROFILE_DEV_DEBUG']},
        'phase5_receipt_sha256':sha(prior_path)}
    write(out/'provenance.json',provenance)
    invocations=[]
    def invoke(cmd, path, environment=env):
        r=base.run(cmd,path,environment)
        invocations.append(r)
        write(out/'invocations.json',invocations)
        assert r['exit_code']==0,(path,r['exit_code'])
        return r
    invoke(['cargo','build','--release','--locked','-p','jarambit-crescent'],out/'build.log')
    flags=[]
    for name in ['jarambit_crescent','jarambit_genotype','jarambit_phenotype','jarambit_physics',
                 'jarambit_redteam','jarambit_evidence','serde','serde_json','spade']:
        libs=list((ROOT/'target/release/deps').glob(f'lib{name}-*.rlib'))
        assert len(libs)==1,(name,libs)
        flags+=['--extern',f'{name}={libs[0]}']
    cases={}
    for name in ['Q7','placement_05','Q7_window_p05']:
        folder=out/name;folder.mkdir();harness=folder/'harness';harness.mkdir()
        source=prior['candidates']['placement_05' if name=='placement_05' else 'Q7']
        old=ROOT/source['source_surface'];old=old.parent
        for f in ['compound.rs','prototype_pheno.rs','prototype_export.rs']:
            shutil.copy2(old/'harness'/f,harness/f)
        for f in ['candidate.csv','config.json']:
            shutil.copy2(old/'inputs'/f,folder/f)
        if name=='Q7_window_p05':
            q=list(csv.reader((folder/'candidate.csv').open()))
            placement=out/'placement_05/candidate.csv'
            pp=list(csv.reader(placement.open()))
            changes={}
            for k in ['win_s0','win_s1','win_w','win_fill','ring_wall']:
                idx=q[0].index(k);value=pp[1][pp[0].index(k)]
                changes[k]={'Q7':q[1][idx],'hybrid':value};q[1][idx]=value
            actual={k:float(v['hybrid']) for k,v in changes.items()}
            assert actual['win_s0']==.05 and actual['win_s1']==.525
            assert 4.5<=actual['win_w']<=6 and .65<=actual['win_fill']<=.70 and 6.4<=actual['ring_wall']<=6.8
            with (folder/'candidate.csv').open('w') as f:csv.writer(f,lineterminator='\n').writerows(q)
            write(folder/'hybrid_changes.json',changes)
        config_doc=json.loads((folder/'config.json').read_text());config=config_doc.get('config',config_doc)
        run_env={**env,**config,'JARAMBIT_SOLVER':'faer'}
        # Supported faer backend changes neither native loads nor unbiased mesh metrics.
        src=ROOT/'crates/crescent/src'
        p5.verify_pheno((src/'pheno.rs').read_text(),(harness/'prototype_pheno.rs').read_text(),name!='placement_05')
        generated=(harness/'prototype_export.rs').read_text();original=(src/'export.rs').read_text()
        assert generated[generated.index('\npub fn solid('):]==original[original.index('\npub fn solid('):]
        modules=['genome','pheno','fem','gates','aesthetic','certification','geom','c4_ref','c5_ref','c6_ref','step_export']
        (harness/'modules.rs').write_text('\n'.join(
            f'#[path="{harness}/prototype_pheno.rs"] pub mod pheno;' if n=='pheno'
            else f'#[path="{src/n}.rs"] pub mod {n};' for n in modules)+'\n')
        shutil.copy2(HERE/'phase6_probe.rs',harness/'phase6_probe.rs')
        binary=folder/'probe'
        invoke(['rustc','--edition=2021','-O','-C','lto=thin',str(harness/'phase6_probe.rs'),
            '-L',f'dependency={ROOT}/target/release/deps',*flags,'-o',str(binary)],folder/'build.log')
        cases[name]={'folder':folder,'seed':folder/'candidate.csv','env':run_env,'binary':binary,
            'binary_sha256':sha(binary),'generated_sha256':{str(p.relative_to(ROOT)):sha(p) for p in harness.iterdir()}}
    write(out/'cases.json',{n:{k:str(v) if isinstance(v,Path) else v for k,v in c.items() if k!='env'} for n,c in cases.items()})
    # Generalize only the Phase-5 witness-geometry validation selector for the new hybrid.
    adapter=out/'solid_adapter.py'
    adapter_text=(HERE/'solid_tip_3d_p5.py').read_text()
    assert adapter_text.count("name == 'Q7'")==1
    adapter.write_text(adapter_text.replace("name == 'Q7'","'STUDY_WINDOW_BELLY' in config"))
    spec=importlib.util.spec_from_file_location('phase6_solid_adapter',adapter)
    solid=importlib.util.module_from_spec(spec);spec.loader.exec_module(solid)
    versions=prior['provenance']['primary_runtime']
    def hybrid_solve():
        c=cases['Q7_window_p05']
        return solid.evaluate(('Q7_window_p05',c['seed'],c['folder']/'config.json',
            c['folder']/'harness/compound.rs',c['folder']/'harness/prototype_pheno.rs'),
            c['folder']/'solid',env,flags,versions)
    results={}
    with ThreadPoolExecutor(max_workers=1) as pool:
        future=pool.submit(hybrid_solve)
        for name,c in cases.items():
            folder=c['folder'];cad=folder/'cad';cad.mkdir();replay=folder/'replay';replay.mkdir()
            invoke([str(c['binary']),str(c['seed']),str(folder),name],folder/'plate.log',c['env'])
            for where in [cad,replay]:
                invoke([str(c['binary']),str(c['seed']),str(where),name,'export'],where/'export.log',c['env'])
            deterministic=all(sha(cad/f)==sha(replay/f) for f in ['candidate.step','candidate.stl','surface.json'])
            write(folder/'determinism.json',{'deterministic':deterministic,
                'sha256':{f:sha(cad/f) for f in ['candidate.step','candidate.stl','surface.json']}})
            assert deterministic
            invoke(['/home/ubuntu/.venvs/jarambit-cad/bin/python',str(HERE/'phase6_cad_check.py'),str(cad)],
                cad/'independent_check.log',c['env'])
            plate=json.loads((folder/'plate.json').read_text());export=json.loads((cad/'export.json').read_text())
            if name!='Q7_window_p05':
                assert sha(cad/'surface.json')==prior['candidates'][name]['source_surface_sha256']
                solid_result=prior['candidates'][name]
            else:solid_result=None
            results[name]={'plate':plate,'export':export,'cad_check':json.loads((cad/'cad_check.json').read_text()),
                'deterministic':deterministic,'solid':solid_result,'candidate_sha256':sha(c['seed']),
                'candidate_file':str(c['seed'].relative_to(ROOT))}
            write(out/'partial_results.json',results)
        results['Q7_window_p05']['solid']=future.result()
    for n,c in cases.items():
        assert sha(c['binary'])==c['binary_sha256']
        for f,h in c['generated_sha256'].items():assert sha(ROOT/f)==h,f
    for f,h in provenance['source_sha256'].items():assert sha(ROOT/f)==h,f
    write(out/'results.json',results)
    print('COMPLETE',out,flush=True)


if __name__=='__main__':main()
