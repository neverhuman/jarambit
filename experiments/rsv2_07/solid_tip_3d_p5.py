"""RSV2-07-p5: replay both witness constructions through the Phase-4 3D pipeline."""
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
NECK = Path('/home/ubuntu/jarambit/.agent/worktrees/lane-G16-06-RSV2-08')
Q7 = Path('/home/ubuntu/jarambit_wt_diag/target/gate-feasibility-bound/unified_study_v3')
WITNESSES = [
    ('placement_05', Path('/home/ubuntu/jarambit-audits/2026-10-18-g16-06-rsv2-08/phase3_candidates/placement_05.csv'),
     NECK/'experiments/G16-06-RSV2-08/candidate_config.json',
     NECK/'experiments/G16-06-RSV2-08/compound.rs',
     NECK/'target/G16-06-RSV2-08/generated/prototype_pheno.rs'),
    ('Q7', Q7/'split_hand_blade_literal_neckdrop_w3_s0.5_0.85_b0_qual.csv',
     Q7/'split_hand_blade_literal_neckdrop_w3_s0.5_0.85_b0_qual_case.json',
     Q7/'compound.rs', Q7/'prototype_pheno.rs'),
]


def write(path, value):
    path.write_text(json.dumps(value, indent=2, allow_nan=False)+'\n')


def verify_pheno(original, witness, belly):
    """Require exactly the documented witness geometry variants, no other edits."""
    slot = 'holes.push(taper_slot(&comet_c, g.v[COMET_W0], g.v[COMET_W1])); hole_names.push("comet");'
    replacement = '''let comet=crate::compound::comet_slot(&comet_c, g.v[COMET_W0], g.v[COMET_W1]);
    if !crate::compound::config().neck_solid { holes.push(comet); hole_names.push("comet"); }
    else { assert!(comet.iter().all(|q|q.0<heel.0), "solid-neck variant only omits the neck slot"); }'''
    clear = '            (nn_in.nearest(q, 40.0) - (b - et) / (2.0 * tan_in)).min(nn_sp.nearest(q, 40.0) - (b - et) / (2.0 * tan_sp)) - WIN_RIM'
    new_clear = '''            if crate::compound::config().window_compound {
                // Use the actual compound-bevel extent; keep the same 3.2 mm window rim.
                nn_in.nearest(q,40.).min(nn_sp.nearest(q,40.)) - crate::compound::config().d1.max(crate::compound::config().blade_d1) - WIN_RIM
            } else {
                (nn_in.nearest(q, 40.0) - (b - et) / (2.0 * tan_in)).min(nn_sp.nearest(q, 40.0) - (b - et) / (2.0 * tan_sp)) - WIN_RIM
            }'''
    assert original.count(slot) == original.count(clear) == 1
    expected = original.replace(slot, replacement).replace(clear, new_clear)
    if belly:
        med = '        let med = medial_line(&c[..=i_heel], &c[cidx[29]..], g.v[WIN_S0], g.v[WIN_S1]);'
        shifted = '''        let mut med = medial_line(&c[..=i_heel], &c[cidx[29]..], g.v[WIN_S0], g.v[WIN_S1]);
        // Explicit Phase-5 construction coordinate: shift toward the local belly.
        // The unchanged window-clearance/rim and native ligament checks still apply.
        let shift=crate::compound::config().window_belly;
        if shift>0. { for q in &mut med {
            let target=c[..=i_heel].iter().copied().min_by(|a,b|dist(*a,*q).total_cmp(&dist(*b,*q))).unwrap();
            *q=add(*q,mul(norm(sub(target,*q)),shift));
        }}'''
        assert expected.count(med) == 1
        expected = expected.replace(med, shifted)
    assert expected == witness, 'undocumented witness phenotype changes'


def evaluate(spec, out, common_env, flags, versions):
    name, source_seed, source_config, source_law, source_pheno = spec
    out.mkdir()
    harness = out/'harness'
    harness.mkdir()
    inputs = out/'inputs'
    inputs.mkdir()
    external = {str(p): digest(p) for p in spec[1:]}
    for p, dest in zip(spec[1:], ['candidate.csv', 'config.json', 'compound.rs', 'prototype_pheno.rs']):
        shutil.copy2(p, inputs/dest)
    seed = inputs/'candidate.csv'
    cfg_doc = json.loads((inputs/'config.json').read_text())
    config = cfg_doc.get('config', cfg_doc)
    env = {**common_env, **config}
    # Preserve the witness's plate solver selection; both 3D solvers are unchanged.
    src = ROOT/'crates/crescent/src'
    pheno = (inputs/'prototype_pheno.rs').read_text()
    verify_pheno((src/'pheno.rs').read_text(), pheno, name == 'Q7')
    (harness/'prototype_pheno.rs').write_text(pheno)
    modules = ['genome','pheno','fem','gates','aesthetic','certification','geom','c4_ref','c5_ref','c6_ref','step_export']
    (harness/'modules.rs').write_text('\n'.join(
        f'#[path="{harness}/prototype_pheno.rs"] pub mod pheno;' if m == 'pheno'
        else f'#[path="{src/m}.rs"] pub mod {m};' for m in modules)+'\n')
    original = (src/'export.rs').read_text()
    a = original.index('pub fn thickness_at(')
    b = original.index('\npub fn solid(', a)
    generated = original[:a]+original[a:b].replace('pub fn thickness_at(', 'pub fn baseline_thickness_at(', 1)
    generated += '\npub fn thickness_at(p: &Pheno, q: P2) -> f64 { crate::compound::thickness_at(p,q) }\n'+original[b:]
    assert generated[generated.index('\npub fn solid('):] == original[b:]
    (harness/'prototype_export.rs').write_text(generated)
    shutil.copy2(inputs/'compound.rs', harness/'compound.rs')
    for f in ['production_surface.rs', 'solid_tip_input.rs']:
        shutil.copy2(HERE/f, harness/f)
    provenance = {'external_inputs_sha256':external, 'environment':{k:env[k] for k in config},
        'cpu_affinity':sorted(os.sched_getaffinity(0)), 'nice':os.getpriority(os.PRIO_PROCESS,0),
        'generated_source_sha256':{str(p.relative_to(ROOT)):digest(p) for p in harness.iterdir()},
        'production_export_body_byte_identical':True, 'witness_pheno_only_documented_variants':True,
        'primary_address_space_bytes':base.PRIMARY_ADDRESS_SPACE}
    invocations = []
    def invoke(command, log, environment=env, bounded=False):
        r = base.run(command, out/log, environment, bounded)
        invocations.append(r)
        write(out/'invocations.json', invocations)
        return r
    binaries = {}
    for source, executable in [('production_surface.rs','production-surface'), ('solid_tip_input.rs','solid-tip-input')]:
        binary = out/executable
        assert invoke(['rustc','--edition=2021','-O','-C','lto=thin',str(harness/source),
            '-L',f'dependency={ROOT}/target/release/deps',*flags,'-o',str(binary)],executable+'-build.log')['exit_code'] == 0
        binaries[str(binary.relative_to(ROOT))] = digest(binary)
    provenance['binary_sha256'] = binaries
    write(out/'provenance.json', provenance)
    assert invoke([str(out/'production-surface'),str(seed),str(out)],'export.log')['exit_code'] == 0
    assert invoke([str(out/'solid-tip-input'),str(seed),'-',str(out)],'plate-context.log')['exit_code'] == 0
    surface_path = out/'critical_fem_3_surface_1mm.json'
    surface = json.loads(surface_path.read_text())
    genes = [float(x) for x in list(csv.reader(seed.open()))[1][:-1]]
    assert surface['genes'] == genes
    d1 = out/'critical_fem_3_D1.json'
    context = json.loads(d1.read_text())
    assert context['compound_config'] == surface['compound_config']
    assert context['native_tip'] == surface['tip']
    context['export_surface'] = surface
    d1.write_text(json.dumps(context,separators=(',',':'),allow_nan=False)+'\n')
    assert json.loads(d1.read_text())['export_surface'] == surface
    effective = surface['compound_config']
    # Check every explicit section-law setting against the actual Rust serialization.
    aliases = {'window_current':'window_compound'}
    for key, value in config.items():
        if key.startswith('STUDY_') and key != 'STUDY_AREAS':
            field = aliases.get(key[6:].lower(), key[6:].lower())
            assert float(effective[field]) == float(value), (key, effective.get(field), value)
    del context, surface
    mesh, primary_path = out/'solid_mesh.npz', out/'skfem_raw.json'
    command = [str(PYTHON),str(ROOT/'fem_solid/tip_solid.py'),str(d1),'30','4','1',str(primary_path)]
    assert invoke(command,'count.log',{**env,'COUNT_ONLY':'1'})['exit_code'] == 0
    count = json.loads(next(l[6:] for l in (out/'count.log').read_text().splitlines() if l.startswith('COUNT ')))
    provenance['native_mesh_counts'] = count
    write(out/'provenance.json', provenance)
    print(name, 'COUNTS', json.dumps(count), flush=True)
    assert count['n_tet']*900*16 < base.PRIMARY_ADDRESS_SPACE
    primary = invoke(command,'skfem.log',{**env,'DUMP_MESH':str(mesh)},True)
    sk = json.loads(primary_path.read_text()) if primary['exit_code'] == 0 else None
    assert mesh.exists(), 'primary did not dump the identical-mesh handoff'
    mesh_sha, primary_sha = digest(mesh), digest(primary_path) if sk else None
    site = Path(versions['site_packages'])
    container_out = out/'container-output'
    container_out.mkdir()
    launcher = "import numpy,sys,os,json,runpy;sys.path.append('/deps');import scipy,dolfinx;from petsc4py import PETSc;json.dump(dict(dolfinx=dolfinx.__version__,numpy=numpy.__version__,scipy=scipy.__version__,petsc=PETSc.Sys.getVersion(),cpus=sorted(os.sched_getaffinity(0)),nice=os.getpriority(os.PRIO_PROCESS,0)),open('/results/runtime.json','w'));sys.argv=sys.argv[1:];runpy.run_path(sys.argv[0],run_name='__main__')"
    cross = invoke(['docker','run','--rm','--cpuset-cpus=14,15','--network=none',
        '-v',f'{ROOT}:/repo:ro','-v',f'{out}:/work:ro','-v',f'{container_out}:/results:rw',
        '-v',f'{site}/scipy:/deps/scipy:ro','-v',f'{site}/scipy.libs:/deps/scipy.libs:ro',
        '-e','OMP_NUM_THREADS=1','-e','OPENBLAS_NUM_THREADS=1','-e','MKL_NUM_THREADS=1',
        '-e','CARGO_BUILD_JOBS=2',
        '-e','PYTHONPATH=/usr/local/dolfinx-real/lib/python3.12/dist-packages:/usr/local/lib:/repo',
        '-e','CROSS_CHECK_SIDECAR=/results/sidecar.json',IMAGE,'nice','-n','10','python3','-c',launcher,
        '/repo/fem_solid/cross_check.py','/work/solid_mesh.npz','/work/skfem_raw.json' if sk else '-'],'dolfinx.log')
    assert digest(mesh) == mesh_sha
    if sk: assert digest(primary_path) == primary_sha
    runtime = json.loads((container_out/'runtime.json').read_text())
    assert runtime['dolfinx'] == '0.9.0' and runtime['cpus'] == [14,15] and runtime['nice'] == 10
    side_path = container_out/'sidecar.json'
    side = json.loads(side_path.read_text()) if side_path.exists() else None
    dx = side['legacy_stdout'] if side else None
    if dx:
        write(out/'dolfinx_raw.json', dx)
        assert dx['n_dof'] == count['n_dof']
        assert side['inputs']['/work/solid_mesh.npz'] == mesh_sha
        if sk: assert side['inputs']['/work/skfem_raw.json'] == primary_sha
    for p, h in {**external, **provenance['generated_source_sha256'], **binaries}.items():
        assert digest(ROOT/p) == h, p
    converged = bool(sk and sk['solver']['converged'] and sk['solver']['relative_residual'] <= 1e-10)
    errors = {op:abs(dx[f'SF_nodal_{op}']/sk['solid'][f'SF_solid_nodal_{op}']-1)
              for op in ['max','avg']} if converged and dx else None
    within = bool(cross['exit_code'] == 0 and max(errors.values()) <= .02) if errors else None
    result = {'candidate':name,'status':'verified' if converged and within else 'not_verified',
        'source_candidate':str(source_seed),'candidate_sha256':digest(seed),'genes':genes,
        'requested_config':config,'effective_compound_config':effective,'provenance':provenance,
        'source_surface':str(surface_path.relative_to(ROOT)),'source_surface_sha256':digest(surface_path),
        'surface_metadata':json.loads((out/'surface_metadata.json').read_text()),
        'primary_converged':converged,'primary':sk,
        'SF_solid_nodal_max':sk['solid']['SF_solid_nodal_max'] if sk else None,
        'SF_solid_nodal_avg':sk['solid']['SF_solid_nodal_avg'] if sk else None,
        'mesh_sha256':mesh_sha,'dolfinx_cross_check':{'within_2pct':within,
            'relative_errors_vs_primary':errors,'runtime':runtime,'raw':dx,'solver_sidecar':side},
        'invocations':invocations,'verification':{'source_and_binary_unchanged':True,
            'surface_bound_exactly':True,'identical_mesh_verified':bool(side),
            'primary_result_unchanged_during_cross_check':True},
        'artifacts_sha256':{str(p.relative_to(ROOT)):digest(p) for p in out.rglob('*') if p.is_file()}}
    write(out/'result.json', result)
    print(name, 'RESULT', json.dumps({'converged':converged,'errors':errors,'within_2pct':within,
          'max':result['SF_solid_nodal_max'],'avg':result['SF_solid_nodal_avg']}), flush=True)
    return result


def main():
    os.sched_setaffinity(0,{14,15})
    os.setpriority(os.PRIO_PROCESS,0,10)
    base.PRIMARY_ADDRESS_SPACE = 64*1024**3
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
    out = ROOT/'target/rsv2-07/solid-tip-3d-p5'/stamp
    out.mkdir(parents=True,exist_ok=False)
    env = {k:v for k,v in os.environ.items() if not k.startswith(('STUDY_','JARAMBIT_')) and k not in
        ['COUNT_ONLY','DUMP_ONLY','DUMP_MESH','PATCH','PYTHONPATH','CARGO_TARGET_DIR']}
    env.update(CARGO_BUILD_JOBS='2',RAYON_NUM_THREADS='2',OMP_NUM_THREADS='1',
        OPENBLAS_NUM_THREADS='1',MKL_NUM_THREADS='1',TMPDIR=str(ROOT/'target/rsv2-07/compiler-tmp'))
    env.setdefault('CARGO_INCREMENTAL','0')
    env.setdefault('CARGO_PROFILE_DEV_DEBUG','line-tables-only')
    sources = [*ROOT.glob('crates/*/src/**/*.rs'),*ROOT.glob('crates/*/Cargo.toml'),
        ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'rust-toolchain.toml',*ROOT.glob('fem_solid/*.py'),
        ROOT/'assurance/numerical/observable_catalog.json',HERE/'production_surface.rs',
        HERE/'solid_tip_input.rs',HERE/'solid_tip_3d.py',Path(__file__)]
    versions = json.loads(subprocess.check_output([str(PYTHON),'-c',
        'import json,sys,sysconfig,numpy,scipy,skfem,pyamg;print(json.dumps(dict(python=sys.version,numpy=numpy.__version__,scipy=scipy.__version__,skfem=skfem.__version__,pyamg=pyamg.__version__,site_packages=sysconfig.get_path("purelib"))))'],env=env,text=True))
    result = {'schema':'jarambit.rsv2_07.phase5_witness_3d_validation.v1','task':'RSV2-07-p5',
        'timestamp_utc':stamp,'status':'running','prototype_only':True,'all_gate_qualification':False,
        'provenance':{'commit':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),
            'source_sha256':{str(p.relative_to(ROOT)):digest(p) for p in sorted(set(sources))},
            'cpu_affinity':[14,15],'nice':10,'CARGO_BUILD_JOBS':2,'environment':{
                k:env[k] for k in ['CARGO_INCREMENTAL','CARGO_PROFILE_DEV_DEBUG','RAYON_NUM_THREADS',
                                  'OMP_NUM_THREADS','OPENBLAS_NUM_THREADS','MKL_NUM_THREADS']},
            'primary_runtime':versions,'container_image':IMAGE,
            'container_image_id':subprocess.check_output(['docker','image','inspect',IMAGE,
                '--format','{{.Id}}'],text=True).strip()},
        'parameters':{'export_call_max_area_mm2':1.,'rs_v2_effective_area_cap_mm2':.25,
            'production_outer_dp_tolerance_mm':.03,'uses_study_grid_points':False,
            'R_CUT_mm':30,'NL':4,'NSUB':1,'PATCH_mm':2,'T_Nmm':1820},'candidates':{}}
    write(out/'provenance.json', result['provenance'])
    build = base.run(['cargo','build','--release','--locked','-p','jarambit-crescent'],out/'cargo-build.log',env)
    assert build['exit_code'] == 0
    result['build'] = build
    flags = []
    for name in ['jarambit_crescent','jarambit_genotype','jarambit_phenotype','jarambit_physics',
                 'jarambit_redteam','jarambit_evidence','serde','serde_json','spade']:
        libs = list((ROOT/'target/release/deps').glob(f'lib{name}-*.rlib'))
        assert len(libs) == 1, (name,libs)
        flags += ['--extern',f'{name}={libs[0]}']
    for spec in WITNESSES:
        result['candidates'][spec[0]] = evaluate(spec,out/spec[0],env,flags,versions)
        write(HERE/'results/phase5_witness_3d_validation.json', result)
    for p,h in result['provenance']['source_sha256'].items(): assert digest(ROOT/p) == h,p
    result['status'] = 'verified' if all(c['status'] == 'verified' for c in result['candidates'].values()) else 'not_verified'
    result['verification'] = {'all_source_hashes_unchanged':True}
    write(HERE/'results/phase5_witness_3d_validation.json', result)
    print('COMPLETE',result['status'],str(out),flush=True)


if __name__ == '__main__':
    main()
