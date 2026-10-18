"""Build and audit the existing compound candidate with native GCI and A4 routines."""
from pathlib import Path
import csv
import datetime
import hashlib
import json
import math
import os
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
SRC = ROOT / 'crates/crescent/src'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    os.sched_setaffinity(0, {14, 15})
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
    out = ROOT / 'target/rsv2-07/gci-a4' / stamp
    out.mkdir(parents=True, exist_ok=False)
    harness = out / 'harness'
    harness.mkdir()
    compiler_tmp = ROOT / 'target/rsv2-07/compiler-tmp'
    compiler_tmp.mkdir(exist_ok=True)
    env = {k: v for k, v in os.environ.items() if not k.startswith('STUDY_')}
    env.update(CARGO_BUILD_JOBS='2', JARAMBIT_SOLVER='faer', RAYON_NUM_THREADS='2',
               TMPDIR=str(compiler_tmp))
    env.setdefault('CARGO_INCREMENTAL', '0')
    env.setdefault('CARGO_PROFILE_DEV_DEBUG', 'line-tables-only')
    prior = json.loads((HERE / 'results/validation.json').read_text())
    selected = next(r for r in prior['validation'] if r['name'] == 'critical_fem_3')
    seed = HERE / 'results/critical_fem_3.csv'
    assert digest(seed) == selected['seed_sha256']
    config = {f'STUDY_{k.upper()}': str(int(v) if isinstance(v, bool) else v)
              for k, v in selected['config'].items()}
    env.update(config)
    source_paths = [*SRC.glob('*.rs'), *ROOT.glob('crates/*/src/**/*.rs'),
                    *ROOT.glob('crates/*/Cargo.toml'), ROOT / 'Cargo.toml', ROOT / 'Cargo.lock',
                    *ROOT.glob('requirements/RS-karambit-*.toml'),
                    HERE / 'compound.rs', HERE / 'gci_a4.rs', Path(__file__),
                    seed, HERE / 'results/validation.json']
    sources = {str(p.relative_to(ROOT)): digest(p) for p in sorted(set(source_paths))}
    commit = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
    provenance = {'commit': commit, 'source_sha256': sources, 'cpus': sorted(os.sched_getaffinity(0)),
                  'environment': {k: env[k] for k in ['CARGO_BUILD_JOBS', 'JARAMBIT_SOLVER',
                      'RAYON_NUM_THREADS', 'CARGO_INCREMENTAL', 'CARGO_PROFILE_DEV_DEBUG']},
                  'compound_environment': config}
    (out / 'provenance.json').write_text(json.dumps(provenance, indent=2) + '\n')
    with (out / 'build.log').open('w') as log:
        subprocess.run(['cargo', 'build', '--release', '--locked', '-p', 'jarambit-crescent'],
                       cwd=ROOT, env=env, stdout=log, stderr=log, check=True)
        # Same export wrapper as generate.py, isolated from all prior evidence.
        source = (SRC / 'export.rs').read_text()
        start = source.index('pub fn thickness_at(')
        end = source.index('\npub fn solid(', start)
        baseline = source[start:end].replace('pub fn thickness_at(', 'pub fn baseline_thickness_at(', 1)
        source = source[:start] + baseline + '\npub fn thickness_at(p: &Pheno, q: P2) -> f64 { crate::compound::thickness_at(p,q) }\n' + source[end:]
        source = source.replace('let outer=dp_closed(&p.outline,0.03);', 'let outer=dp_closed(&p.outline,crate::compound::config().boundary_tol);')
        source = source.replace('max_area.min(0.25)', 'max_area.min(crate::compound::config().export_area)')
        source = source.replace('let m=fine::mesh_domain(&outer,&holes,', 'let m=crate::compound::export_mesh(p,&outer,&holes,')
        (harness / 'prototype_export.rs').write_text(source)
        modules = ['genome', 'pheno', 'fem', 'gates', 'aesthetic', 'certification',
                   'geom', 'c4_ref', 'c5_ref', 'c6_ref', 'step_export']
        (harness / 'modules.rs').write_text('\n'.join(
            f'#[path="{SRC / name}.rs"] pub mod {name};' for name in modules) + '\n')
        for name in ['compound.rs', 'gci_a4.rs']:
            shutil.copy2(HERE / name, harness / name)
        flags = []
        for name in ['jarambit_crescent', 'jarambit_genotype', 'jarambit_phenotype',
                     'jarambit_physics', 'jarambit_redteam', 'jarambit_evidence',
                     'serde', 'serde_json', 'spade']:
            libs = list((ROOT / 'target/release/deps').glob(f'lib{name}-*.rlib'))
            assert len(libs) == 1, (name, libs)
            flags += ['--extern', f'{name}={libs[0]}']
        binary = out / 'gci-a4'
        subprocess.run(['rustc', '--edition=2021', '-O', '-C', 'lto=thin',
                        str(harness / 'gci_a4.rs'), '-L', f'dependency={ROOT}/target/release/deps',
                        *flags, '-o', str(binary)], cwd=ROOT, env=env, stdout=log, stderr=log, check=True)
    provenance['binary_sha256'] = digest(binary)
    provenance['generated_source_sha256'] = {str(p.relative_to(ROOT)): digest(p) for p in harness.iterdir()}
    command = [str(binary), str(seed.relative_to(ROOT)), str(out.relative_to(ROOT))]
    provenance['command'] = command
    (out / 'provenance.json').write_text(json.dumps(provenance, indent=2) + '\n')
    with (out / 'run.log').open('w') as log:
        subprocess.run(command, cwd=ROOT, env=env, stdout=log, stderr=log, check=True)
    raw = json.loads((out / 'raw.json').read_text())
    assert raw['compound_config'] == selected['config']
    seed_row = list(csv.reader(seed.open()))[1]
    assert raw['native_geometry_pass'] and raw['genes'] == [float(v) for v in seed_row[:-1]]
    assert digest(binary) == provenance['binary_sha256']
    assert all(digest(ROOT / p) == sha for p, sha in sources.items()), 'source drift'
    # Independent identity/replay checks, not another implementation of the estimators.
    for area, actual in [(0.25, raw['levels_fine_to_coarse'][0]['metrics']),
                         (0.125, raw['certification_reference']['area_0p125_metrics'])]:
        old = next(r for r in selected['levels'] if r['area'] == area)['metrics']
        for key in ['seq_ratio', 'hm_twist_sf', 'hm_spine_sf', 'hm_ringback_sf', 'fat_sf']:
            assert math.isclose(actual[key], old[key], rel_tol=1e-10, abs_tol=1e-12), (area, key)
    for row in raw['levels_fine_to_coarse']:
        assert digest(ROOT / row['mesh_file']) == row['mesh_sha256']
    a4 = raw['side_a4']
    assert math.isclose(a4['supported_seq_ratio'], a4['supported_blade_peak_mpa_per_n'] /
                        a4['supported_hand_peak_mpa_per_n'], rel_tol=1e-12)
    assert a4['buckling_critical_load_n'] == 260 * a4['buckling_eigenvalue']
    assert (a4['status'] == 'pass') == (a4['buckling_eigenvalue'] > 0 and
            a4['supported_seq_ratio'] >= a4['certified_seq_ratio'])
    primary = raw['gci']['seq_ratio']
    assessment = {
        'primary_observable': 'seq_ratio',
        'plate_gci_admitted': primary['admitted'],
        'discretization_under_3pct': primary['admitted'] and primary['gci_fine'] < 0.03,
        'side_a4_native_pass': a4['status'] == 'pass',
        'side_a4_native_pass_and_eigenvalue_ge_1': a4['status'] == 'pass' and a4['buckling_eigenvalue'] >= 1,
        'supported_vs_same_mesh_clamp_seq_delta': a4['supported_seq_ratio'] - raw['levels_fine_to_coarse'][0]['metrics']['seq_ratio'],
        'production_sequence_ratio_unchanged': True,
        'independent_3d_or_manufacturing_qualification': False,
    }
    result = {'schema': 'jarambit.rsv2_07.gci_a4_validation.v1', 'timestamp_utc': stamp,
              'all_gate_qualification': False, 'provenance': provenance, **raw,
              'assessment': assessment,
              'verification': {'source_and_binary_unchanged': True, 'prior_fine_mesh_replay': True,
                               'mesh_hashes_verified': True, 'a4_identities_verified': True},
              'artifacts_sha256': {str(p.relative_to(ROOT)): digest(p)
                                   for p in out.iterdir() if p.is_file() and p != binary}}
    destination = HERE / 'results/gci_a4_validation.json'
    destination.write_text(json.dumps(result, indent=2, allow_nan=False) + '\n')
    print(json.dumps({'validation': str(destination.relative_to(ROOT)), 'raw': str(out.relative_to(ROOT)),
                      'gci': raw['gci'], 'side_a4': a4}, indent=2))


if __name__ == '__main__':
    main()
