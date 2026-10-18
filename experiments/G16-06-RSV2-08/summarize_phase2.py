#!/usr/bin/env python3
"""Publish compact Phase-2 observations without claiming full qualification."""
import hashlib
import json
import struct
from pathlib import Path
from run_phase2 import HERE, ROOT, OUT, digest

if __name__ == '__main__':
    base = json.loads((OUT / 'baseline_receipt.json').read_text())
    search = json.loads((OUT / 'convergence_search_summary.json').read_text())
    local_path = OUT / 'local_search_summary.json'
    if local_path.exists():
        search['successes'] += json.loads(local_path.read_text())['successes']
    rows = [json.loads(line) for line in (OUT / 'window_trials.jsonl').read_text().splitlines()]
    gci = base['plate_gci']['seq_ratio_gci']
    a4 = base['a4']
    mesh = json.loads((OUT/'sealed-a4.mesh.json').read_text())
    mesh_bytes = (b''.join(struct.pack('<dd', *q) for q in mesh['xy'])
                  + b''.join(struct.pack('<QQQ', *t) for t in mesh['tris'])
                  + b''.join(struct.pack('<d', t) for t in mesh['te']))
    fine = base['plate_gci']['levels_fine_to_coarse'][0]
    assert hashlib.sha256(mesh_bytes).hexdigest() == fine['mesh_sha256']
    assert fine['seq_ratio'] == a4['same_mesh_ring_clamp_seq_ratio'] == a4['certified_seq_ratio']
    assert base['sealed_G16_13'] == (a4['buckling_eigenvalue'] >= 1 and a4['supported_seq_ratio'] >= a4['certified_seq_ratio'])
    assert '2 passed; 0 failed' in (OUT/'gci-tests.log').read_text()
    coarse = [r for r in rows if r['stage'] == 'coarse']
    fits = {r['name']: r for r in rows if r['stage'] in ('gci', 'audit_scope_gci')}
    refinements = [r for r in rows if r['stage'] in ('refine', 'audit_scope_refine')]
    native = json.loads((OUT / 'sealed-gci.json').read_text())['provenance']
    verified = {}
    for group in ('source_sha256', 'requirements_sha256', 'dependency_source_sha256', 'prototype_source_sha256'):
        for name, expected in native[group].items():
            path = Path(name)
            if not path.is_absolute(): path = ROOT / path
            assert digest(path) == expected, (group, name)
        verified[group] = len(native[group])
    for name in ('candidate.csv', 'candidate_config.json'):
        relative = str((HERE / name).relative_to(ROOT))
        assert digest(HERE / name) == base['sha256'][relative]
    result = dict(task='G16-06-RSV2-08-p2', cpus=[12, 13], cargo_build_jobs=2,
                  sealed_G16_15=False, sealed_G16_13=base['sealed_G16_13'],
                  sealed_plate_gci=gci, sealed_side_a4=a4,
                  native_G16_15_metadata_valid=gci['sequence_valid'] and gci['observable_valid'],
                  assignment_G16_15_requires_admitted=True,
                  native_sources_and_sealed_inputs_unchanged=True, source_hash_checks=verified,
                  window_trials=len(coarse), geometry_legal=sum(r['legal'] for r in coarse),
                  trials=[dict(name=r['name'], genes=r['genes'], legal=r['legal'],
                               coarse_targets_pass=r['targets_pass'],
                               coarse_sequence=[v['metrics']['seq_ratio'] for v in r['levels']],
                               gci_fit=fits.get(r['name'], {}).get('fit_status', 'not_run'),
                               gci_admitted=fits.get(r['name'], {}).get('admitted', False)) for r in coarse],
                  refinement_observations=refinements, selected_variants=search['successes'],
                  combined_adjustment_found=bool(search['successes']),
                  scope='GCI 4/1/.25 plus monotonicity through .125/.0625; unchanged certification min(.25,.125) and strict .0625 task thresholds',
                  thresholds=search['thresholds'], all_gate_qualification=False,
                  native_gci_tests=dict(passed=2, failed=0),
                  unresolved=['sealed plate GCI is oscillatory', 'human-max twist/spine',
                              'flat-stock/collar manufacturing inconsistency',
                              'independent solid/cross-solver qualification',
                              'coarse search failures and fine-backend faer parity failure retained'],
                  commit_status='pending; shared Git metadata was read-only at previous delivery')
    result['sha256'] = {str(p.relative_to(ROOT)): digest(p) for p in [
        HERE/'candidate.csv', HERE/'candidate_config.json', HERE/'candidate_receipt.json',
        HERE/'phase2.rs', HERE/'build_phase2.sh', HERE/'run_phase2.py',
        HERE/'phase2_window_search.py', HERE/'phase2_convergence_search.py', HERE/'summarize_phase2.py',
        ROOT/'crates/crescent/src/gci.rs', ROOT/'crates/physics/src/side_a4.rs',
        OUT/'study', OUT/'sealed-gci.json', OUT/'sealed-a4.json', OUT/'sealed-a4.mesh.json',
        OUT/'window_trials.jsonl', OUT/'window_search_summary.json', OUT/'convergence_search_summary.json']}
    if local_path.exists(): result['sha256'][str(local_path.relative_to(ROOT))] = digest(local_path)
    for selected in search['successes']:
        name = selected['name']
        candidate = OUT/'window_trials'/f'{name}.csv'
        destination = HERE/'phase2_candidate.csv'
        destination.write_bytes(candidate.read_bytes())
        result['sha256'][str(destination.relative_to(ROOT))] = digest(destination)
        result['selected_variant_gci'] = json.loads((OUT/f'{name}-gci.json').read_text())['plate_gci']
    (HERE/'phase2_receipt.json').write_text(json.dumps(result, indent=2) + '\n')
    lines = [
        '# G16-06-RSV2-08 Phase 2 — measured prototype results', '',
        f'Sealed witness: side-A4 **PASS**, plate GCI **FAIL** (`{gci["fit_status"]}`, admitted=false). Full qualification remains false. No native source, requirement, load, support constant or threshold changed.', '',
        '| Sealed evaluation | Result |', '| --- | --- |',
        f'| Sequence at 4 / 1 / .25 mm² | ' + ' / '.join(str(r['seq_ratio']) for r in reversed(base['plate_gci']['levels_fine_to_coarse'])) + ' |',
        f'| Side-A4 buckling eigenvalue | {a4["buckling_eigenvalue"]} (floor 1) |',
        f'| Side-A4 supported sequence | {a4["supported_seq_ratio"]} |',
        f'| Certified sequence | {a4["certified_seq_ratio"]} |', '',
        'The native GCI sequence/observable metadata flags are valid, but the numerical fit is oscillatory and `seq_ratio_gci.admitted=false`. This assignment requires admission; metadata alone is not reported as convergence success. The .25 mesh hash and ring-clamp metrics match exactly between native GCI, native side-A4 input and the sealed witness.', '',
        f'Executed {len(coarse)} deterministic native window-gene trials: 36 broad placement/fill, 8 width/rounded-end-radius, and {sum(r["name"].startswith("local_") for r in coarse)} local placement probes. {sum(r["legal"] for r in coarse)} pass the unchanged geometry checks. '
        + ('A combined adjustment was found; see the receipt for its exact scope and mesh results.' if search['successes'] else 'No tested adjustment passed the combined convergence, certification/strict and export/support checks. This finite study does not establish infeasibility.'), '',
        'The first screen required all task targets at both coarse levels. The follow-up uses the sealed audit scope: targets at certification min(.25,.125) and strict .0625, with 4/1 as GCI observations; all five sequence values must be monotonic. Every coarse failure remains a failure in the receipt. Native GCI determines admission and positive order; failed fits and fine meshes are retained.', '',
        'The 7 mm width trial (fillet_06) admits a 3-level fit and passes .25 targets (lateral 360.391068616949 N, fatigue 1.0066894000614173, sequence 1.3680770766716677). At .125 it fails lateral/fatigue (322.012021 N) and reverses sequence to 1.624254496. It is rejected before strict/export/support work; passing a 3-level fit does not establish the required five-level behavior.', '',
        '| Trial | Window genes | Sequence 4 → 1 mm² | GCI fit |', '| --- | --- | --- | --- |']
    for r in result['trials']:
        lines.append(f'| {r["name"]} | {json.dumps(r["genes"], sort_keys=True)} | {r["coarse_sequence"][0]:.8f} → {r["coarse_sequence"][1]:.8f} | {r["gci_fit"]} |')
    lines += ['', 'Native GCI tests: 2 passed, 0 failed. Python and shell syntax and source/receipt integrity checks pass. All numerical work used CPUs 12,13 and CARGO_BUILD_JOBS=2. The previous 38-test native library result is retained; it was not rerun for this adapter-only phase.', '',
              'Review `phase2_receipt.json`. Full raw GCI, side-A4 mesh, per-trial CSV/JSON/logs, refinement and export evidence remain under `target/G16-06-RSV2-08/phase2/`. The sealed genome/config/receipt are unchanged; adjusted variants are not substituted into the production exporter or declared manufacturing-qualified.', '',
              'Reproduce in the lane (preserve prior logs before another sweep):', '```sh',
              'taskset -c 12,13 bash experiments/G16-06-RSV2-08/build_phase2.sh',
              'taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/run_phase2.py',
              'taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/phase2_window_search.py',
              'taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/phase2_convergence_search.py',
              'taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/phase2_convergence_search.py --local',
              'taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/summarize_phase2.py', '```', '']
    (HERE/'PHASE2.md').write_text('\n'.join(lines))
    print(json.dumps(dict(sealed_G16_15=False, sealed_G16_13=result['sealed_G16_13'], trials=len(coarse), selected_variants=len(search['successes']))))
