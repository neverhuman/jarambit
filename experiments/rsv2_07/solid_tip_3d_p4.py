"""RSV2-07-p4: native export meshing, recorded compound law, two independent P2 solves."""
from pathlib import Path
import csv
import datetime
import json
import os
import shutil
import subprocess

import solid_tip_3d as base

ROOT, HERE, PYTHON, IMAGE = base.ROOT, base.HERE, base.PYTHON, base.IMAGE
digest = base.digest


def main():
    os.sched_setaffinity(0, {14, 15})
    os.setpriority(os.PRIO_PROCESS, 0, 10)
    base.PRIMARY_ADDRESS_SPACE = 64 * 1024**3
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
    out = ROOT/'target/rsv2-07/solid-tip-3d-p4'/stamp
    out.mkdir(parents=True, exist_ok=False)
    harness = out/'harness'
    harness.mkdir()
    for name in ['SOLID_TIP_3D_VERIFICATION.md', 'results/solid_tip_3d_validation.json']:
        shutil.copy2(HERE/name, out/('phase3_'+Path(name).name))
    prior = json.loads((out/'phase3_solid_tip_3d_validation.json').read_text())
    seed = HERE/'results/critical_fem_3.csv'
    assert digest(seed)==prior['candidate_sha256']
    gci = json.loads((HERE/'results/gci_a4_validation.json').read_text())
    config = gci['compound_config']
    prior_input = ROOT/next(p for p in prior['artifacts_sha256'] if p.endswith('/critical_fem_3_D1.json'))
    assert digest(prior_input)==prior['artifacts_sha256'][str(prior_input.relative_to(ROOT))]
    src = ROOT/'crates/crescent/src'
    modules = ['genome','pheno','fem','gates','aesthetic','certification','geom','c4_ref','c5_ref','c6_ref','step_export']
    (harness/'modules.rs').write_text('\n'.join(f'#[path="{src/name}.rs"] pub mod {name};' for name in modules)+'\n')
    original = (src/'export.rs').read_text()
    start = original.index('pub fn thickness_at(')
    end = original.index('\npub fn solid(',start)
    baseline = original[start:end].replace('pub fn thickness_at(', 'pub fn baseline_thickness_at(',1)
    generated = original[:start]+baseline+'\npub fn thickness_at(p: &Pheno, q: P2) -> f64 { crate::compound::thickness_at(p,q) }\n'+original[end:]
    assert generated[generated.index('\npub fn solid('):]==original[end:]
    (harness/'prototype_export.rs').write_text(generated)
    for name in ['compound.rs','production_surface.rs']:
        shutil.copy2(HERE/name,harness/name)
    env = {k:v for k,v in os.environ.items() if not k.startswith('STUDY_') and k not in
           ['COUNT_ONLY','DUMP_ONLY','DUMP_MESH','PATCH','PYTHONPATH','CARGO_TARGET_DIR']}
    env.update(CARGO_BUILD_JOBS='2',JARAMBIT_SOLVER='faer',RAYON_NUM_THREADS='2',
               OMP_NUM_THREADS='1',OPENBLAS_NUM_THREADS='1',MKL_NUM_THREADS='1',
               TMPDIR=str(ROOT/'target/rsv2-07/compiler-tmp'))
    env.setdefault('CARGO_INCREMENTAL','0')
    env.setdefault('CARGO_PROFILE_DEV_DEBUG','line-tables-only')
    env.update({f'STUDY_{k.upper()}':str(int(v) if isinstance(v,bool) else v) for k,v in config.items()})
    sources = [*ROOT.glob('crates/*/src/**/*.rs'),*ROOT.glob('crates/*/Cargo.toml'),
               ROOT/'Cargo.toml',ROOT/'Cargo.lock',*ROOT.glob('fem_solid/*.py'),
               ROOT/'assurance/numerical/observable_catalog.json',
               src.parent/'examples/export_step.rs',HERE/'compound.rs',HERE/'production_surface.rs',
               HERE/'solid_tip_3d.py',Path(__file__),seed,prior_input]
    provenance = {'commit':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),
        'source_and_input_sha256':{str(p.relative_to(ROOT)):digest(p) for p in sorted(set(sources))},
        'generated_source_sha256':{str(p.relative_to(ROOT)):digest(p) for p in harness.iterdir()},
        'compound_config':config,'cpu_affinity':sorted(os.sched_getaffinity(0)),
        'nice':os.getpriority(os.PRIO_PROCESS,0),'CARGO_BUILD_JOBS':2,
        'primary_address_space_bytes':base.PRIMARY_ADDRESS_SPACE,
        'environment':{k:env[k] for k in ['OMP_NUM_THREADS','OPENBLAS_NUM_THREADS','MKL_NUM_THREADS',
                       'JARAMBIT_SOLVER','RAYON_NUM_THREADS','CARGO_INCREMENTAL','CARGO_PROFILE_DEV_DEBUG']},
        'container_image':IMAGE,'container_image_id':subprocess.check_output(
            ['docker','image','inspect',IMAGE,'--format','{{.Id}}'],text=True).strip()}
    versions = json.loads(subprocess.check_output([str(PYTHON),'-c',
        'import json,sys,sysconfig,numpy,scipy,skfem,pyamg;print(json.dumps(dict(python=sys.version,numpy=numpy.__version__,scipy=scipy.__version__,skfem=skfem.__version__,pyamg=pyamg.__version__,site_packages=sysconfig.get_path("purelib"))))'],env=env,text=True))
    provenance['primary_runtime'] = versions
    invocations = []
    def invoke(command,name,environment=env,bounded=False):
        r=base.run(command,out/name,environment,bounded)
        invocations.append(r)
        (out/'invocations.json').write_text(json.dumps(invocations,indent=2)+'\n')
        return r
    (out/'provenance.json').write_text(json.dumps(provenance,indent=2)+'\n')
    assert invoke(['cargo','build','--release','--locked','-p','jarambit-crescent'],'cargo-build.log')['exit_code']==0
    flags=[]
    for name in ['jarambit_crescent','jarambit_genotype','jarambit_phenotype','jarambit_physics',
                 'jarambit_redteam','jarambit_evidence','serde','serde_json','spade']:
        libs=list((ROOT/'target/release/deps').glob(f'lib{name}-*.rlib'))
        assert len(libs)==1,(name,libs)
        flags+=['--extern',f'{name}={libs[0]}']
    binary=out/'production-surface'
    assert invoke(['rustc','--edition=2021','-O','-C','lto=thin',str(harness/'production_surface.rs'),
                   '-L',f'dependency={ROOT}/target/release/deps',*flags,'-o',str(binary)],'rustc.log')['exit_code']==0
    provenance['binary_sha256']=digest(binary)
    assert invoke([str(binary),str(seed),str(out)],'export.log')['exit_code']==0
    surface_path=out/'critical_fem_3_surface_1mm.json'
    surface=json.loads(surface_path.read_text())
    assert surface['genes']==[float(x) for x in list(csv.reader(seed.open()))[1][:-1]]
    assert surface['compound_config']==config
    context=json.loads(prior_input.read_text())
    assert context['compound_config']==config
    assert context['export_surface']['tip']==surface['tip']
    context['export_surface']=surface
    d1=out/'critical_fem_3_D1.json'
    d1.write_text(json.dumps(context,separators=(',',':'),allow_nan=False)+'\n')
    assert json.loads(d1.read_text())['export_surface']==surface
    shutil.copy2(prior_input.with_name('critical_fem_3_grind_law.json'),out/'critical_fem_3_grind_law.json')
    del context,surface
    mesh=out/'solid_mesh.npz'
    primary_path=out/'skfem_raw.json'
    command=[str(PYTHON),str(ROOT/'fem_solid/tip_solid.py'),str(d1),'30','4','1',str(primary_path)]
    assert invoke(command,'count.log',{**env,'COUNT_ONLY':'1'})['exit_code']==0
    count=json.loads(next(line[6:] for line in (out/'count.log').read_text().splitlines() if line.startswith('COUNT ')))
    provenance['native_mesh_counts']=count
    (out/'provenance.json').write_text(json.dumps(provenance,indent=2)+'\n')
    print('COUNTS',json.dumps(count),flush=True)
    assert count['n_tet']*900*16<base.PRIMARY_ADDRESS_SPACE,'native assembly exceeds execution budget'
    primary=invoke(command,'skfem.log',{**env,'DUMP_MESH':str(mesh)},bounded=True)
    sk=json.loads(primary_path.read_text()) if primary['exit_code']==0 else None
    assert mesh.exists(),'native primary did not produce the identical-mesh handoff'
    mesh_sha=digest(mesh)
    primary_sha=digest(primary_path) if sk else None
    site=Path(versions['site_packages'])
    container_out=out/'container-output'
    container_out.mkdir()
    launcher="import numpy,sys,os,json,runpy;sys.path.append('/deps');import scipy,dolfinx;from petsc4py import PETSc;json.dump(dict(dolfinx=dolfinx.__version__,numpy=numpy.__version__,scipy=scipy.__version__,petsc=PETSc.Sys.getVersion(),cpus=sorted(os.sched_getaffinity(0)),nice=os.getpriority(os.PRIO_PROCESS,0)),open('/results/runtime.json','w'));sys.argv=sys.argv[1:];runpy.run_path(sys.argv[0],run_name='__main__')"
    cross=invoke(['docker','run','--rm','--cpuset-cpus=14,15','--network=none',
        '-v',f'{ROOT}:/repo:ro','-v',f'{out}:/work:ro','-v',f'{container_out}:/results:rw',
        '-v',f'{site}/scipy:/deps/scipy:ro','-v',f'{site}/scipy.libs:/deps/scipy.libs:ro',
        '-e','OMP_NUM_THREADS=1','-e','OPENBLAS_NUM_THREADS=1','-e','MKL_NUM_THREADS=1',
        '-e','CARGO_BUILD_JOBS=2',
        '-e','PYTHONPATH=/usr/local/dolfinx-real/lib/python3.12/dist-packages:/usr/local/lib:/repo',
        '-e','CROSS_CHECK_SIDECAR=/results/sidecar.json',IMAGE,'nice','-n','10','python3','-c',launcher,
        '/repo/fem_solid/cross_check.py','/work/solid_mesh.npz','/work/skfem_raw.json' if sk else '-'],'dolfinx.log')
    assert digest(mesh)==mesh_sha
    if sk:assert digest(primary_path)==primary_sha
    runtime=json.loads((container_out/'runtime.json').read_text())
    assert runtime['dolfinx']=='0.9.0' and runtime['cpus']==[14,15] and runtime['nice']==10
    side=json.loads((container_out/'sidecar.json').read_text()) if (container_out/'sidecar.json').exists() else None
    dx=side['legacy_stdout'] if side else None
    if dx:(out/'dolfinx_raw.json').write_text(json.dumps(dx,indent=2)+'\n')
    for p,h in provenance['source_and_input_sha256'].items():assert digest(ROOT/p)==h,p
    for p,h in provenance['generated_source_sha256'].items():assert digest(ROOT/p)==h,p
    assert digest(binary)==provenance['binary_sha256']
    converged=bool(sk and sk['solver']['converged'] and sk['solver']['relative_residual']<=1e-10)
    errors={op:abs(dx[f'SF_nodal_{op}']/sk['solid'][f'SF_solid_nodal_{op}']-1) for op in ['max','avg']} if converged and dx else None
    within=bool(cross['exit_code']==0 and max(errors.values())<=0.02) if errors else None
    result={'schema':'jarambit.rsv2_07.solid_tip_3d_validation.v2','task':'RSV2-07-p4',
        'status':'verified' if converged and within else 'not_verified','timestamp_utc':stamp,
        'prototype_only':True,'all_gate_qualification':False,'candidate':str(seed.relative_to(ROOT)),
        'candidate_sha256':digest(seed),'source_surface':str(surface_path.relative_to(ROOT)),
        'source_surface_sha256':digest(surface_path),'mesh_sha256':mesh_sha,
        'parameters':{'export_call_max_area_mm2':1.0,'rs_v2_effective_area_cap_mm2':0.25,
                      'R_CUT_mm':30,'NL':4,'NSUB':1,'PATCH_mm':2,'T_Nmm':1820},
        'provenance':provenance,'invocations':invocations,'primary_converged':converged,'primary':sk,
        'solid_tip_sf_nodal_max':sk['solid']['SF_solid_nodal_max'] if sk else None,
        'solid_tip_sf_nodal_avg':sk['solid']['SF_solid_nodal_avg'] if sk else None,
        'dolfinx_cross_check':{'within_2pct':within,'numerical_solver_invoked':True,
                             'runtime':runtime,'raw':dx,'relative_errors_vs_primary':errors,'solver_sidecar':side},
        'surface_metadata':json.loads((out/'surface_metadata.json').read_text()),
        'phase3_archive':{'report':str((out/'phase3_SOLID_TIP_3D_VERIFICATION.md').relative_to(ROOT)),
                          'receipt':str((out/'phase3_solid_tip_3d_validation.json').relative_to(ROOT))},
        'verification':{'source_and_binary_unchanged':True,'surface_bound_exactly':True,
                        'identical_mesh_verified':True,'primary_result_unchanged_during_cross_check':True},
        'artifacts_sha256':{str(p.relative_to(ROOT)):digest(p) for p in out.rglob('*') if p.is_file() and p!=binary}}
    (HERE/'results/solid_tip_3d_validation.json').write_text(json.dumps(result,indent=2,allow_nan=False)+'\n')
    print('RESULT',json.dumps({k:result[k] for k in ['status','solid_tip_sf_nodal_max','solid_tip_sf_nodal_avg','primary_converged']},indent=2),flush=True)
    print('DOLFINX',json.dumps({'raw':dx,'relative_errors':errors,'within_2pct':within}),flush=True)


if __name__=='__main__':
    main()
