"""Phase-3 audit: exact saved compound CAD skin, skfem P2 and pinned DOLFINx."""
from pathlib import Path
import csv
import datetime
import hashlib
import json
import os
import resource
import shutil
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
PYTHON = Path('/home/ubuntu/.venvs/jarambit-sheath/bin/python')
IMAGE = 'ghcr.io/fenics/dolfinx/dolfinx@sha256:628f40dd62c7f37962ccadc5a745f23d52c56b2ac3ad9b9b6a86c567c07aa27b'


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


PRIMARY_ADDRESS_SPACE = int(os.environ.get('RSV2_07_PRIMARY_MAX_AS_GIB', '24')) * 1024**3


def limit_primary():
    resource.setrlimit(resource.RLIMIT_AS, (PRIMARY_ADDRESS_SPACE, PRIMARY_ADDRESS_SPACE))


def run(command, log, env, bounded=False):
    print('RUN', log, flush=True)
    start = time.time()
    with log.open('w') as stream:
        process = subprocess.run(command, cwd=ROOT, env=env, stdout=stream, stderr=subprocess.STDOUT,
                                 preexec_fn=limit_primary if bounded else None)
    return {'argv': command, 'exit_code': process.returncode, 'wall_seconds': time.time()-start,
            'log': str(log.relative_to(ROOT)), 'log_sha256': digest(log)}


def record_resource_failure(out):
    """Publish an explicit incomplete audit; missing numerical results stay null."""
    provenance = json.loads((out/'provenance.json').read_text())
    invocations = json.loads((out/'invocations.json').read_text())
    primary = next(r for r in invocations if r['log'].endswith('/skfem.log'))
    assert primary['exit_code'] != 0
    log = (out/'skfem.log').read_text()
    guard = json.loads((out/'primary_guard_stop.json').read_text()) if (out/'primary_guard_stop.json').exists() else None
    assert 'MemoryError' in log or guard is not None, 'unexpected failure requires separate adjudication'
    own = str(Path(__file__).relative_to(ROOT))
    # The recorded attempt used the pre-guard driver. Only its orchestration is
    # revised; solver, adapter, CAD, candidate and configuration must be identical.
    changed = {p:{'before':h,'after':digest(ROOT/p)} for p,h in provenance['source_and_input_sha256'].items()
               if digest(ROOT/p)!=h}
    assert set(changed).issubset({own}), changed
    provenance['orchestration_revision_after_attempt'] = changed
    provenance['recording_driver_sha256'] = digest(__file__)
    assert digest(out/'solid-tip-input') == provenance['adapter_binary_sha256']
    for p,h in provenance['generated_source_sha256'].items(): assert digest(ROOT/p)==h,p
    runtime_log = out/'dolfinx_runtime.log'
    runtime_command = ['docker','run','--rm','--cpuset-cpus=14,15','--network=none',
        '-v',f'{ROOT}:/repo:ro',
        '-v',f'{provenance["primary_runtime"]["site_packages"]}/scipy:/deps/scipy:ro',
        '-v',f'{provenance["primary_runtime"]["site_packages"]}/scipy.libs:/deps/scipy.libs:ro',
        IMAGE,'python3','-c',
        "import numpy,sys,os,json;sys.path.extend(['/deps','/repo/fem_solid']);import scipy,dolfinx,solve_dolfinx;from petsc4py import PETSc;print(json.dumps(dict(dolfinx=dolfinx.__version__,numpy=numpy.__version__,scipy=scipy.__version__,petsc=PETSc.Sys.getVersion(),reference_model=solve_dolfinx.MODEL_ID,cpus=sorted(os.sched_getaffinity(0)))))"]
    runtime = run(runtime_command,runtime_log,{**os.environ,'CARGO_BUILD_JOBS':'2'})
    assert runtime['exit_code']==0
    runtime_versions = json.loads(runtime_log.read_text())
    assert runtime_versions['dolfinx']=='0.9.0' and runtime_versions['cpus']==[14,15]
    invocations.append(runtime)
    counts = provenance['mesh_counts']
    mem = {line.split(':')[0]:int(line.split()[1])*1024 for line in Path('/proc/meminfo').read_text().splitlines()
           if line.startswith(('MemTotal:','MemAvailable:','SwapTotal:','SwapFree:'))}
    minimum = counts['n_tet'] * 30 * 30 * (8+4+4)
    limit_path = out/'primary_resource_limit.json'
    if not limit_path.exists():
        limit_path.write_text(json.dumps({'rlimit_as_bytes':PRIMARY_ADDRESS_SPACE,'inherited_by_primary':True},indent=2)+'\n')
    library = Path(provenance['primary_runtime']['site_packages'])/'skfem/assembly/form/bilinear_form.py'
    resources = {'native_counts':counts,'p2_vector_local_dofs':30,
                 'native_coo_data_and_indices_bytes':minimum,'host_at_recording_bytes':mem,
                 'primary_rlimit_as_bytes':PRIMARY_ADDRESS_SPACE,
                 'assembly_implementation':str(library),'assembly_implementation_sha256':digest(library)}
    seed = HERE/'results/critical_fem_3.csv'
    cad = ROOT/'target/rsv2-07/cad/critical_fem_3/prototype.mesh.json'
    result = {'schema':'jarambit.rsv2_07.solid_tip_3d_validation.v1',
        'status':'incomplete_resource_limit','timestamp_utc':out.name,
        'prototype_only':True,'all_gate_qualification':False,'candidate':str(seed.relative_to(ROOT)),
        'candidate_sha256':digest(seed),'source_surface':str(cad.relative_to(ROOT)),
        'source_surface_sha256':digest(cad),'parameters':{'R_CUT_mm':30,'NL':4,'NSUB':1,'PATCH_mm':2,'T_Nmm':1820},
        'provenance':provenance,'invocations':invocations,'resources':resources,
        'primary_converged':False,
        'primary_status':('not_run_resource_guard' if primary.get('invoked') is False else
                          'stopped_resource_guard_before_solve' if guard else 'memory_error_before_solve'),
        'primary_solver_invoked':primary.get('invoked',True),'resource_guard':guard,
        'solid_tip_sf_nodal_max':None,'solid_tip_sf_nodal_avg':None,
        'primary_raw_result_exists':(out/'skfem_raw.json').exists(),
        'shared_tet_mesh_exists':(out/'solid_mesh.npz').exists(),
        'dolfinx_cross_check':{'within_2pct':None,'status':'not_run_missing_primary_result_and_shared_mesh',
            'numerical_solver_invoked':False,'runtime_verified':True,'runtime_versions':runtime_versions,
            'container_image':IMAGE,'container_image_id':provenance['container_image_id'],
            'reason':'primary did not reach DUMP_MESH; no validated identical mesh or primary SF exists'},
        'verification':{'scientific_sources_and_inputs_unchanged':True,'adapter_binary_and_sources_unchanged':True,
                        'native_count_completed':True,'cpus':[14,15],'CARGO_BUILD_JOBS':2},
        'surface_validation':json.loads((out/'surface_validation.json').read_text()) if (out/'surface_validation.json').exists() else None,
        'artifacts_sha256':{str(p.relative_to(ROOT)):digest(p) for p in out.rglob('*') if p.is_file() and p.name!='solid-tip-input'}}
    controls = out/'software-controls.log'
    if controls.exists():
        assert 'Ran 2 tests' in controls.read_text() and '\nOK\n' in controls.read_text()
        result['verification']['software_controls'] = {'passed':2,'failed':0,'physical_validation':False,
            'tests':['test_refinement_preserves_the_actual_thin_cad_skin','test_nonconverged_primary_never_certifies'],
            'log':str(controls.relative_to(ROOT)),'log_sha256':digest(controls)}
    assert not result['primary_raw_result_exists'] and not result['shared_tet_mesh_exists']
    (HERE/'results/solid_tip_3d_validation.json').write_text(json.dumps(result,indent=2,allow_nan=False)+'\n')
    print('RECORDED incomplete_resource_limit',str(out),flush=True)


def main():
    os.sched_setaffinity(0, {14, 15})
    if len(sys.argv)==3 and sys.argv[1]=='--record-existing':
        record_resource_failure(Path(sys.argv[2]).resolve())
        return
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
    out = ROOT / 'target/rsv2-07/solid-tip-3d' / stamp
    out.mkdir(parents=True, exist_ok=False)
    harness = out / 'harness'
    harness.mkdir()
    cad = ROOT / 'target/rsv2-07/cad/critical_fem_3/prototype.mesh.json'
    seed = HERE / 'results/critical_fem_3.csv'
    old = json.loads((HERE / 'results/gci_a4_validation.json').read_text())
    cad_manifest = json.loads((HERE / 'results/cad_manifest.json').read_text())[0]
    assert digest(cad) == cad_manifest['files'][str(cad.relative_to(ROOT))]
    assert digest(seed) == old['candidate_sha256']
    previous_harness = Path(old['provenance']['command'][0]).parent / 'harness'
    for name in ['modules.rs', 'prototype_export.rs', 'compound.rs']:
        source = previous_harness / name
        assert digest(ROOT / source) == old['provenance']['generated_source_sha256'][str(source.relative_to(ROOT))]
        shutil.copy2(ROOT / source, harness / name)
    shutil.copy2(HERE / 'solid_tip_input.rs', harness / 'solid_tip_input.rs')
    env = {k:v for k,v in os.environ.items() if not k.startswith('STUDY_') and
           k not in ['COUNT_ONLY', 'DUMP_ONLY', 'DUMP_MESH', 'PATCH', 'PYTHONPATH', 'CARGO_TARGET_DIR']}
    env.update(CARGO_BUILD_JOBS='2', JARAMBIT_SOLVER='faer', RAYON_NUM_THREADS='2',
               OMP_NUM_THREADS='1', OPENBLAS_NUM_THREADS='1', MKL_NUM_THREADS='1',
               TMPDIR=str(ROOT / 'target/rsv2-07/compiler-tmp'))
    env.setdefault('CARGO_INCREMENTAL', '0')
    env.setdefault('CARGO_PROFILE_DEV_DEBUG', 'line-tables-only')
    env.update({f'STUDY_{k.upper()}': str(int(v) if isinstance(v,bool) else v)
                for k,v in old['compound_config'].items()})
    source_paths = [*ROOT.glob('crates/*/src/**/*.rs'), *ROOT.glob('crates/*/Cargo.toml'),
                    ROOT/'Cargo.toml', ROOT/'Cargo.lock', *ROOT.glob('fem_solid/*.py'),
                    ROOT/'assurance/numerical/observable_catalog.json',
                    HERE/'solid_tip_input.rs', Path(__file__), cad, seed,
                    HERE/'results/validation.json', HERE/'results/cad_manifest.json']
    sources = {str(p.relative_to(ROOT)):digest(p) for p in sorted(set(source_paths))}
    image_id = subprocess.check_output(['docker','image','inspect',IMAGE,'--format','{{.Id}}'],text=True).strip()
    versions = json.loads(subprocess.check_output([str(PYTHON),'-c',
        'import json,sys,sysconfig,numpy,scipy,skfem,pyamg; print(json.dumps(dict(python=sys.version,numpy=numpy.__version__,scipy=scipy.__version__,skfem=skfem.__version__,pyamg=pyamg.__version__,site_packages=sysconfig.get_path("purelib"))))'],text=True,env=env))
    provenance = {'commit':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),
                  'source_and_input_sha256':sources, 'cpus':sorted(os.sched_getaffinity(0)),
                  'CARGO_BUILD_JOBS':2, 'environment':{k:env[k] for k in ['JARAMBIT_SOLVER','RAYON_NUM_THREADS',
                       'OMP_NUM_THREADS','OPENBLAS_NUM_THREADS','MKL_NUM_THREADS','CARGO_INCREMENTAL','CARGO_PROFILE_DEV_DEBUG']},
                  'primary_runtime':versions, 'container_image':IMAGE, 'container_image_id':image_id,
                  'compound_config':old['compound_config']}
    invocations = []
    def invoke(command, name, environment=env, bounded=False):
        record = run(command,out/name,environment,bounded)
        invocations.append(record)
        (out/'invocations.json').write_text(json.dumps(invocations,indent=2)+'\n')
        return record
    (out/'provenance.json').write_text(json.dumps(provenance,indent=2)+'\n')
    build = invoke(['cargo','build','--release','--locked','-p','jarambit-crescent'],'cargo-build.log')
    assert build['exit_code'] == 0
    flags = []
    for name in ['jarambit_crescent','jarambit_genotype','jarambit_phenotype','jarambit_physics',
                 'jarambit_redteam','jarambit_evidence','serde','serde_json','spade']:
        libs = list((ROOT/'target/release/deps').glob(f'lib{name}-*.rlib'))
        assert len(libs)==1,(name,libs)
        flags += ['--extern',f'{name}={libs[0]}']
    binary = out/'solid-tip-input'
    compile_result = invoke(['rustc','--edition=2021','-O','-C','lto=thin',str(harness/'solid_tip_input.rs'),
                             '-L',f'dependency={ROOT}/target/release/deps',*flags,'-o',str(binary)],'rustc.log')
    assert compile_result['exit_code']==0
    provenance['adapter_binary_sha256'] = digest(binary)
    provenance['generated_source_sha256'] = {str(p.relative_to(ROOT)):digest(p) for p in harness.iterdir()}
    adapter = invoke([str(binary),str(seed),str(cad),str(out)],'adapter.log')
    assert adapter['exit_code']==0
    d1 = out/'critical_fem_3_D1.json'
    # Python's correctly rounded JSON reader preserves the original saved float
    # coordinates. Do not round-trip the CAD through serde_json's default reader.
    cad_doc = json.loads(cad.read_text())
    assert cad_doc['genes'] == [float(v) for v in list(csv.reader(seed.open()))[1][:-1]]
    assert cad_doc['config'] == old['compound_config']
    verts = cad_doc['verts']
    upper = [t for t in cad_doc['tris'] if all(verts[i][2] > 0 for i in t)]
    used = sorted({i for t in upper for i in t})
    remap = {old:new for new,old in enumerate(used)}
    context = json.loads(d1.read_text())
    context['export_surface'] = {'schema':'jarambit.export_surface.v1','tip':context.pop('native_tip'),
        'verts':[verts[i] for i in used],'tris':[[remap[i] for i in t] for t in upper],
        'source':str(cad.relative_to(ROOT)),
        'selection':'all three original vertex z coordinates strictly positive; no simplification'}
    d1.write_text(json.dumps(context,separators=(',',':'),allow_nan=False)+'\n')
    provenance['surface_selection'] = {'upper_facets':len(upper),'upper_vertices':len(used),
                                      'coordinate_values_preserved':True}
    del context, cad_doc, verts, upper, used, remap
    mesh = out/'solid_mesh.npz'
    primary_path = out/'skfem_raw.json'
    command = [str(PYTHON),str(ROOT/'fem_solid/tip_solid.py'),str(d1),'30','4','1',str(primary_path)]
    count = invoke(command,'count.log',{**env,'COUNT_ONLY':'1'})
    assert count['exit_code']==0
    counts = json.loads(next(l.removeprefix('COUNT ') for l in (out/'count.log').read_text().splitlines() if l.startswith('COUNT ')))
    provenance['mesh_counts'] = counts
    (out/'provenance.json').write_text(json.dumps(provenance,indent=2)+'\n')
    print('COUNTS',json.dumps(counts),flush=True)
    minimum_coo_bytes = counts['n_tet']*30*30*16
    if minimum_coo_bytes > PRIMARY_ADDRESS_SPACE:
        (out/'primary_guard_stop.json').write_text(json.dumps({
            'reason':'native COO data/indices alone exceed configured address-space budget',
            'signal':None,'coo_bytes':minimum_coo_bytes,
            'primary_address_space_limit_bytes':PRIMARY_ADDRESS_SPACE},indent=2)+'\n')
        (out/'skfem.log').write_text('Native count completed; numerical solve not run: native COO arrays exceed configured address-space budget.\n')
        invocations.append({'argv':command,'exit_code':None,'invoked':False,'status':'not_run_resource_guard',
                            'log':str((out/'skfem.log').relative_to(ROOT)),'log_sha256':digest(out/'skfem.log')})
        (out/'invocations.json').write_text(json.dumps(invocations,indent=2)+'\n')
        record_resource_failure(out)
        return
    primary = invoke(command,'skfem.log',{**env,'DUMP_MESH':str(mesh)},bounded=True)
    if primary['exit_code'] != 0 and not mesh.exists():
        record_resource_failure(out)
        return
    mesh_hash = digest(mesh)
    primary_doc = json.loads(primary_path.read_text()) if primary['exit_code']==0 else None
    site = Path(versions['site_packages'])
    container_out = out/'container-output'
    container_out.mkdir()
    launcher = "import numpy,runpy,sys;sys.path.append('/deps');sys.argv=sys.argv[1:];runpy.run_path(sys.argv[0],run_name='__main__')"
    container_command = ['docker','run','--rm','--cpuset-cpus=14,15','--network=none',
        '-v',f'{ROOT}:/repo:ro','-v',f'{out}:/work:ro','-v',f'{container_out}:/results:rw',
        '-v',f'{site}/scipy:/deps/scipy:ro','-v',f'{site}/scipy.libs:/deps/scipy.libs:ro',
        '-e','OMP_NUM_THREADS=1','-e','OPENBLAS_NUM_THREADS=1','-e','MKL_NUM_THREADS=1',
        '-e','PYTHONPATH=/usr/local/dolfinx-real/lib/python3.12/dist-packages:/usr/local/lib:/repo',
        '-e','CROSS_CHECK_SIDECAR=/results/sidecar.json',IMAGE,'python3','-c',launcher,
        '/repo/fem_solid/cross_check.py','/work/solid_mesh.npz', '/work/skfem_raw.json' if primary_doc else '-']
    cross = invoke(container_command,'dolfinx.log')
    assert digest(mesh)==mesh_hash
    dx = None
    if (container_out/'sidecar.json').exists():
        side = json.loads((container_out/'sidecar.json').read_text())
        dx = side['legacy_stdout']
        (out/'dolfinx_raw.json').write_text(json.dumps(dx,indent=2)+'\n')
    assert all(digest(ROOT/p)==h for p,h in sources.items()),'source/input drift'
    assert digest(binary)==provenance['adapter_binary_sha256']
    primary_converged = bool(primary_doc and primary_doc['solver']['converged'] and
                             primary_doc['solver']['relative_residual']<=1e-10)
    agreement = None
    if primary_converged and dx:
        sk=primary_doc['solid']
        errors={op:abs(dx[f'SF_nodal_{op}']/sk[f'SF_solid_nodal_{op}']-1) for op in ['max','avg']}
        agreement={'relative_error_nodal_max':errors['max'],'relative_error_nodal_avg':errors['avg'],
                   'within_2pct':cross['exit_code']==0 and max(errors.values())<=0.02}
    result={'schema':'jarambit.rsv2_07.solid_tip_3d_validation.v1','timestamp_utc':stamp,
            'prototype_only':True,'all_gate_qualification':False,'candidate':str(seed.relative_to(ROOT)),
            'candidate_sha256':digest(seed),'source_surface':str(cad.relative_to(ROOT)),
            'source_surface_sha256':digest(cad),'parameters':{'R_CUT_mm':30,'NL':4,'NSUB':1,'PATCH_mm':2,'T_Nmm':1820},
            'provenance':provenance,'invocations':invocations,'mesh_sha256':mesh_hash,
            'primary_converged':primary_converged,
            'solid_tip_sf_nodal_max':primary_doc['solid']['SF_solid_nodal_max'] if primary_doc else None,
            'solid_tip_sf_nodal_avg':primary_doc['solid']['SF_solid_nodal_avg'] if primary_doc else None,
            'primary':primary_doc,'dolfinx_cross_check':{'within_2pct':None,'raw':dx,**(agreement or {})},
            'artifacts_sha256':{str(p.relative_to(ROOT)):digest(p) for p in out.rglob('*') if p.is_file() and p!=binary}}
    (HERE/'results/solid_tip_3d_validation.json').write_text(json.dumps(result,indent=2,allow_nan=False)+'\n')
    print('COMPLETE',json.dumps({k:result[k] for k in ['solid_tip_sf_nodal_max','solid_tip_sf_nodal_avg','primary_converged','dolfinx_cross_check']}),flush=True)


if __name__=='__main__':
    main()
