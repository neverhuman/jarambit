"""Verify retained observations and write a compact, reviewable task report."""
from pathlib import Path
import csv
import hashlib
import json
import math
import shutil
import subprocess
from search import ROOT, HERE, OUT, BINARY


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    provenance = json.loads((OUT.parent / 'source_provenance.json').read_text())
    assert all(digest(ROOT / p) == sha for p, sha in provenance['source_sha256'].items())
    binary_sha = digest(BINARY)
    runtime = json.loads((OUT.parent / 'runtime_provenance.json').read_text())
    assert not runtime['production_diff']
    drift = {path: {'build_sha256': sha, 'current_sha256': digest(ROOT/path)}
             for path, sha in runtime['tracked_source_sha256'].items() if digest(ROOT/path) != sha}
    # The coordinator rebased this checkout during the run. This CLI search module is
    # neither linked nor included by the standalone harness; every compiled source above
    # still must match. Preserve the original snapshot and bind the observed change to Git.
    assert set(drift) <= {'crates/cli/src/auto_evolve.rs'}, drift
    for path in drift:
        committed = subprocess.check_output(['git', 'show', f'HEAD:{path}'], cwd=ROOT)
        assert hashlib.sha256(committed).hexdigest() == drift[path]['current_sha256']
    workspace_drift = {'build_commit': provenance['commit'],
                       'current_commit': subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
                       'changed_tracked_sources': drift, 'compiled_sources_unchanged': True,
                       'note': 'External commit/rebase during evaluation; CLI search module is outside the standalone compiled dependency graph.'}
    (OUT.parent/'workspace_drift.json').write_text(json.dumps(workspace_drift,indent=2)+'\n')
    invocations = list(OUT.glob('*/invocation.json'))
    for path in invocations:
        invocation = json.loads(path.read_text())
        assert invocation['cpus'] == [14, 15]
        assert invocation['CARGO_BUILD_JOBS'] == 2
        assert invocation['executable_sha256'] == binary_sha
        assert (path.parent / 'result.json').exists(), path
        doc = json.loads((path.parent / 'result.json').read_text())
        c = doc['config']
        assert 14 <= c['angle'] <= 22 and .05 <= c['apex'] <= .20
        assert c['core'] == 0 and c['d0'] >= .4 and c['d1'] > c['d0']
        assert len(doc['genes']) == 128 and 'point_lateral' in c
        if c['point_shape'] == 3:
            probes = dict(doc['point_cap_probe'])
            assert math.isclose(probes[3.2], .94*doc['t_joint'], rel_tol=1e-12)
            assert math.isclose(probes[5.2], .95*doc['t_joint'], rel_tol=1e-12)
        for level in doc.get('levels', []):
            assert level['unbiased']
            f = level['metrics']
            assert f and all(math.isfinite(v) for v in f.values() if isinstance(v, (float, int)))
            for actual, expected in [
                (f['hm_twist_sf'], f['twist_sf'] * 1820 / 16200),
                (f['hm_spine_sf'], f['hm_spine_n'] / 3427),
                (f['hm_ringback_sf'], f['hm_ringback_n'] / 3000),
                (f['fat_sf'], f['lat_rob'] / 357.996288224513),
            ]:
                assert math.isclose(actual, expected, rel_tol=1e-10, abs_tol=1e-12), path
    initial = json.loads((OUT / 'search_summary.json').read_text())
    follow = json.loads((OUT / 'follow_summary.json').read_text())
    fine = json.loads((OUT / 'fine_summary.json').read_text())
    follow += json.loads((OUT / 'endpoint_summary.json').read_text())
    fine += json.loads((OUT / 'endpoint_fine_summary.json').read_text())
    fine += json.loads((OUT / 'closure_fine_summary.json').read_text())
    fine += json.loads((OUT / 'hotspot_fine_summary.json').read_text())
    fine += json.loads((OUT / 'critical_accepted_summary.json').read_text())
    fine += json.loads((OUT / 'web_accepted_summary.json').read_text())
    unique_fine = []
    seen = set()
    for row in fine:
        key = json.dumps([row['genes'], row['config']], sort_keys=True)
        if key not in seen:
            seen.add(key)
            unique_fine.append(row)
    fine = unique_fine
    table = []
    saved = HERE / 'results'
    saved.mkdir(exist_ok=True)
    for row in fine:
        folder = OUT / row['name']
        doc = json.loads((folder / 'result.json').read_text())
        assert row['legal_geometry']
        levels = {l['area']: l for l in doc['levels']}
        assert set(levels) == {2, .25, .125, .0625}
        if row['selected_from'] == 'baseline':
            export_folder = OUT / 'export_4'
        else:
            export_folder = OUT / row['selected_from']
        export = json.loads((export_folder / 'result.json').read_text())
        assert export['genes'] == doc['genes'] and export['config'] == doc['config']
        m = export['measurement']
        assert m['sharpness']['pass'] and m['point']['pass']
        assert export['solid']['census'] == [0, 0, 1]
        cert = {k: min(levels[a]['metrics'][k] for a in [.25, .125])
                for k in ['hm_twist_sf', 'hm_spine_sf', 'hm_ringback_sf', 'twist_body_sf',
                          'twist_tipzone_sf', 'lat_rob', 'fat_sf', 'seq_ratio']}
        point = {k: v for k, v in m['point'].items() if k != 'samples'}
        entry = {'name': row['name'], 'selected_from': row['selected_from'], 'config': doc['config'],
                 'legal_geometry': True, 'cert_min_0p25_0p125': cert,
                 'strict_0p0625': levels[.0625]['metrics'],
                 'levels': doc['levels'], 'sharpness': m['sharpness'], 'point': point,
                 'native_plate_hm_pass_all_three': {k: min(cert[k], levels[.0625]['metrics'][k]) >= 1 for k in ['hm_twist_sf', 'hm_spine_sf', 'hm_ringback_sf']},
                 'solid': export['solid'], 'raw_fem': str((folder / 'result.json').relative_to(ROOT)),
                 'raw_export': str((export_folder / 'result.json').relative_to(ROOT))}
        shutil.copy2(folder / 'seed.csv', saved / f'{row["name"]}.csv')
        entry['seed_sha256'] = digest(saved / f'{row["name"]}.csv')
        table.append(entry)
    # Reproduce the supplied strict baseline using the final, immutable executable.
    baseline = table[0]['strict_0p0625']
    for key, value in {'hm_twist_sf': .2840261088979059, 'hm_spine_sf': .5808230600216392,
                       'twist_body_sf': 13.592325931672965, 'twist_tipzone_sf': 2.5281444857945474}.items():
        assert math.isclose(baseline[key], value, rel_tol=1e-7), (key, baseline[key], value)
    tests = (OUT.parent / 'cert-integrity-faer-tests.log').read_text()
    assert 'test result: ok. 14 passed; 0 failed;' in tests
    summary = {'cert_integrity_tests': '14 passed; faer; CPUs 14,15; two build jobs and test threads',
               'prototype_only': True, 'all_gate_qualification': False,
               'executable_sha256': binary_sha, 'verified_invocations': len(invocations),
               'coarse_evaluations': len(initial) + len(follow),
               'targeted_four_level_probes': len(json.loads((OUT / 'critical_fem_summary.json').read_text())) + len(json.loads((OUT / 'web_fem_summary.json').read_text())),
               'closure_strict_probes': len(json.loads((OUT / 'closure_strict_summary.json').read_text())) + len(json.loads((OUT / 'hotspot_strict_summary.json').read_text())),
               'legal_coarse_evaluations': sum(r['legal_geometry'] for r in initial + follow),
               'source_provenance': provenance, 'runtime_provenance': runtime, 'workspace_drift': workspace_drift,
               'study_support_sha256': {p.name: digest(p) for p in HERE.iterdir() if p.suffix in ['.py', '.rs', '.sh', '.csv']},
               'external_inputs_sha256': {str(p): digest(p) for p in [
                   Path('/home/ubuntu/jarambit-audits/2026-10-18-gate-feasibility-bound/REPORT.md'),
                   Path('/home/ubuntu/jarambit_wt_diag/target/gate-feasibility-bound/compound_study/run_four.log')]},
               'validation': table}
    summary['best_native_plate_cases'] = {}
    for metric in ['hm_twist_sf', 'hm_spine_sf', 'hm_ringback_sf']:
        best = max(table, key=lambda r:min(r['cert_min_0p25_0p125'][metric], r['strict_0p0625'][metric]))
        value = min(best['cert_min_0p25_0p125'][metric], best['strict_0p0625'][metric])
        summary['best_native_plate_cases'][metric] = {'candidate': best['name'], 'minimum_three_fine_areas': value, 'pass': value >= 1}
    cad_manifest = saved / 'cad_manifest.json'
    if cad_manifest.exists():
        summary['cad_exports'] = json.loads(cad_manifest.read_text())
    (saved / 'validation.json').write_text(json.dumps(summary, indent=2) + '\n')
    leader = next(r for r in table if r['name'] == summary['best_native_plate_cases']['hm_spine_sf']['candidate'])
    lc, ls = leader['cert_min_0p25_0p125'], leader['strict_0p0625']
    lines = [
        '# RSV2-07 compound-section and planform study', '',
        f'The accepted `{leader["name"]}` prototype reaches native-plate spine SF {lc["hm_spine_sf"]:.6f} at certification and {ls["hm_spine_sf"]:.6f} on the strict mesh, while passing exported sharpness and point profile. '
        f'Its human-max twist SF is {lc["hm_twist_sf"]:.6f} / {ls["hm_twist_sf"]:.6f} (certification / strict), still below 1.0. '
        'Spine closes on the three specified fine meshes; human-max twist closure remains open.', '',
        'This is a reproducible construction prototype, not an all-gate qualification. '
        'This study does not change production geometry, gate evaluators, numerical exclusions, loads or thresholds. '
        'A seed must be used with its recorded compound law; loading its CSV in the production CLI does not apply that law.', '',
        f'Validated {len(invocations)} invocations with one executable SHA-256 `{binary_sha}`, CPUs 14,15 and CARGO_BUILD_JOBS=2. '
        f'The deterministic search retained {len(initial)+len(follow)} complete coarse evaluations, '
        f'{sum(r["legal_geometry"] for r in initial+follow)} passing the native geometric predicate. '
        'A finite search supplies observations, not a global optimum or infeasibility theorem. '
        'The later targeted probes add ten strict-only and nine four-level comparisons; the full invocation count includes validation replays.', '',
        'The construction keeps a straight bevel to distance d0 and blends into the stock web by d1. '
        'Point shape 3 reaches 94% of core thickness at 3.2 mm and 95% at 5.2 mm; the elliptical variant permits thicker cheeks away from the measured bisector. '
        'Native spine-start and bounded blade-root control-point offsets redistribute the spine load. '
        'No stock or genome bound is increased.', '',
        'The native predicate means clamp-domain + rs_v2 + aesthetics + empty gates::check; it does not establish structural legality. '
        'Sharpness and point profile below were measured on exported triangles by the unchanged certification code. '
        'The same compound thickness function feeds the export and every FEM solve. '
        'The original field mass is not the prototype mass; export volume is retained instead.', '',
        '| Candidate | Cert HM twist | Strict HM twist | Cert HM spine | Strict HM spine | Strict ring-back | Strict tip-zone | Sharp / point |',
        '| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |',
    ]
    for r in table:
        c = r['cert_min_0p25_0p125']; s = r['strict_0p0625']
        lines.append(f'| {r["name"]} ({r["selected_from"]}) | {c["hm_twist_sf"]:.6f} | {s["hm_twist_sf"]:.6f} | {c["hm_spine_sf"]:.6f} | {s["hm_spine_sf"]:.6f} | {s["hm_ringback_sf"]:.6f} | {s["twist_tipzone_sf"]:.6f} | pass / pass |')
    lines += ['', 'Certification columns are min(0.25,0.125 mm²), and strict columns are the separate 0.0625 mm² solve. '
              'All target SF thresholds remain 1.0. Body twist must be divided by 8.9010989011 to compare it to the human-max torque. '
              'The reported human-max twist uses the complete eligible region, including the governing 4–15 mm tip annulus.', '',
              '| Candidate | Max apex mm | Max half-angle deg | Point t0.25 / t1 / t2 mm | Taper lower bound mm | Strict lateral N | Strict fatigue | Strict sequence |',
              '| --- | ---: | ---: | --- | ---: | ---: | ---: | ---: |']
    for r in table:
        a = r['sharpness']; p = r['point']; s = r['strict_0p0625']
        lines.append(f'| {r["name"]} | {a["max_apex_mm"]:.6f} | {a["max_half_angle_deg"]:.6f} | {p["t_0p25_mm"]:.6f} / {p["t_1p00_mm"]:.6f} / {p["t_2p00_mm"]:.6f} | {p["taper_lower_bound_mm"]:.6f} | {s["lat_rob"]:.6f} | {s["fat_sf"]:.6f} | {s["seq_ratio"]:.6f} |')
    lines += ['', '| Candidate | Strict body / HM requirement | Twist peak radius mm | Peak width / thickness mm | Required/current twist capacity | Required/current spine capacity | Spine hotspot x / y mm |',
              '| --- | ---: | ---: | --- | ---: | ---: | --- |']
    for r in table:
        f = r['strict_0p0625']
        level = next(l for l in r['levels'] if l['area'] == .0625)
        xy = level['diagnostic_peaks']['spine']['at']
        lines.append(f'| {r["name"]} | {f["twist_body_sf"]*1820/16200:.6f} | {f["twist_tipdist"]:.6f} | {f["twist_b_mm"]:.6f} / {f["twist_t_mm"]:.6f} | {1/f["hm_twist_sf"]:.6f} | {1/f["hm_spine_sf"]:.6f} | {xy[0]:.6f} / {xy[1]:.6f} |')
    lines += ['', 'The reciprocal SF columns compare required/current capacity; values above 1 quantify observed deficits. They do not predict a local thickness change that would provide those factors. '
              'Changing a section redistributes stress, and the retained peak locations show which region governs each solve.', '']
    lines += ['', 'The unchanged native certification-integrity suite passed 14/14 tests with faer, CPUs 14,15 and two build jobs/test threads. '
              'Per-invocation verification checks the executable, source digests, CPU affinity, mesh areas, native baseline reproduction, and human-max/fatigue identities.', '',
              '![Exported point profiles and face-angle measurements](results/export_measurements.png)', '',
              'No independent P2 solid, DOLFINx cross-check, GCI adjudication, support/buckling battery or manufacturing qualification is supplied by this study. '
              'The existing bore collar exceeds the flat stock thickness; this unresolved construction issue is inherited from the audited model. '
              'Passing these exported edge measurements does not resolve that issue or establish 16/16 + 10/10.', '',
              'Raw results, per-level peak locations, source provenance and invocation hashes remain in `target/rsv2-07/v3/`. '
              'The root-level exploratory attempts and `superseded-build/` are excluded because their harness versions overlapped. '
              'Only the hashed v3 run is used above. Compact receipts and canonical 128-gene seeds are in `results/validation.json` and `results/*.csv`.', '',
              'Reproduce from the repository root (use a fresh or archived v3 evidence destination):', '',
              '```sh', 'bash experiments/rsv2_07/build.sh',
              'taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/search.py',
              'taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/refine.py',
              'taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/endpoints.py',
              'taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/closure.py',
              'taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/hotspot.py',
              'taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/critical.py',
              'taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/web.py',
              'taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/plot.py',
              'taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/report.py',
              'taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/save_cad.py',
              'taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/report.py', '```', '']
    if drift:
        lines += ['During the run another process committed and rebased this checkout. '
                  'The only recorded tracked-source drift is `crates/cli/src/auto_evolve.rs`, which is outside the standalone harness dependency graph. '
                  'All compiled study sources, requirements and the evaluated executable remain identical to the build snapshot. '
                  '`target/rsv2-07/workspace_drift.json` preserves both commit identities and old/new digests; the original provenance is retained.', '']
    if cad_manifest.exists():
        lines += ['The retained full-precision facets are in `prototype.mesh.json`; the STL is a viewing copy. '
                  'Each CAD replay matches the measured vertex/triangle census and volume; hashes and construction settings are in `results/cad_manifest.json`.', '']
        for cad in summary['cad_exports']:
            for path in cad['files']:
                if path.endswith('.stl') or path.endswith('.mesh.json'):
                    lines.append(f'- [{cad["candidate"]} {Path(path).name}](../../{path})')
        lines.append('')
    (HERE / 'REPORT.md').write_text('\n'.join(lines))
    print(json.dumps({'verified_invocations': len(invocations), 'coarse_evaluations': len(initial)+len(follow),
                      'finalists': [r['selected_from'] for r in table]}, indent=2))


if __name__ == '__main__':
    main()
