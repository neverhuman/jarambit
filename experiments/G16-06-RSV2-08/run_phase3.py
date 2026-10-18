#!/usr/bin/env python3
"""Fully evaluate the nine assigned candidates; no numerical prefilter."""
from concurrent.futures import ThreadPoolExecutor, as_completed
import csv
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import struct
import subprocess
import threading
import tomllib
from run_phase2 import HERE, ROOT, environment, digest

OUT = ROOT / 'target/G16-06-RSV2-08/phase3'
P2 = OUT.parent / 'phase2'
EVENT_LOCK = threading.Lock()
REQUIREMENTS = tomllib.loads((ROOT/'requirements/RS-karambit-v2.toml').read_text())
SEQ_FLOOR = REQUIREMENTS['constants']['seq_target']
assert SEQ_FLOOR == 1.20

def now():
    return datetime.now(timezone.utc).isoformat(timespec='seconds')

def milestone(title, detail):
    entry = f'\n## {now()} — Phase-3 {title}\n{detail}\n'
    for name in ('.agent/CHANGELOG.md', '.agent/handoffs/G16-06-RSV2-08.md'):
        with (ROOT/name).open('a') as f: f.write(entry)

def event(name, stage):
    with EVENT_LOCK:
        with (OUT/'progress.jsonl').open('a') as f:
            f.write(json.dumps(dict(time=now(), name=name, stage=stage))+'\n')
    print(name, stage, flush=True)

def invoke(binary, mode, path, genome, cert=None):
    env = environment()
    env['STUDY_AREAS'] = '.25,.125,.0625'
    command = ['taskset', '-c', '12,13', str(binary), mode, str(path), str(genome)]
    if cert is not None: command.append(str(cert))
    with path.with_suffix('.log').open('w') as log:
        subprocess.run(command, cwd=ROOT, env=env, stdout=log, stderr=log, check=True)
    return json.loads(path.read_text())

def verify_sources(provenance):
    for group in ('source_sha256', 'requirements_sha256', 'dependency_source_sha256', 'prototype_source_sha256'):
        for name, expected in provenance[group].items():
            path = Path(name)
            if not path.is_absolute(): path = ROOT/path
            assert digest(path) == expected, (group, name)

def evaluate(job):
    name, genome, old_gci = job
    event(name, 'started')
    if old_gci is None:
        event(name, 'gci')
        gci_path = OUT/f'{name}-gci.json'
        gci = invoke(P2/'study', 'gci', gci_path, genome)
    else:
        gci_path = old_gci
        gci = json.loads(gci_path.read_text())
    assert gci['genome_sha256'] == digest(genome)
    assert gci['native_gci_source_sha256'] == digest(ROOT/'crates/crescent/src/gci.rs')
    event(name, 'export_and_all_three_fem_levels')
    full_path = OUT/f'{name}-full.json'
    full = invoke(OUT.parent/'study', 'all', full_path, genome)
    assert [r['area'] for r in full['levels']] == [.25, .125, .0625]
    assert all(r['metrics'] is not None for r in full['levels'])
    verify_sources(full['provenance'])
    levels = full['levels']
    cert = {k:min(r['metrics'][k] for r in levels[:2]) for k in levels[0]['metrics'] if isinstance(levels[0]['metrics'][k], (int, float))}
    strict = levels[2]['metrics']
    event(name, 'side_a4')
    a4_path = OUT/f'{name}-a4.json'
    side = invoke(P2/'study', 'a4', a4_path, genome, cert['seq_ratio'])
    a4 = side['a4']
    mesh_path = a4_path.with_suffix('.mesh.json')
    mesh = json.loads(mesh_path.read_text())
    mesh_bytes = (b''.join(struct.pack('<dd', *q) for q in mesh['xy'])
                  + b''.join(struct.pack('<QQQ', *t) for t in mesh['tris'])
                  + b''.join(struct.pack('<d', t) for t in mesh['te']))
    fine_gci = gci['plate_gci']['levels_fine_to_coarse'][0]
    assert hashlib.sha256(mesh_bytes).hexdigest() == fine_gci['mesh_sha256']
    assert fine_gci['seq_ratio'] == levels[0]['metrics']['seq_ratio'] == a4['same_mesh_ring_clamp_seq_ratio']
    assert fine_gci['sf_hunter'] == levels[0]['metrics']['sf_hunter']
    assert a4['certified_seq_ratio'] == cert['seq_ratio']
    measurement = full['measurement']
    geometry_ok = (full['input_was_canonical'] and full['within_genome_domain'] and full['rs_v2'] and full['aes_pass'] and not full['rejects'])
    export_ok = measurement['sharpness']['pass'] and measurement['point']['pass'] and full['solid']['census'] == [0, 0, 1]
    admission = gci['plate_gci']['seq_ratio_gci']['admitted']
    sequence_ok = cert['seq_ratio'] >= SEQ_FLOOR and strict['seq_ratio'] >= SEQ_FLOOR
    strength_ok = all(cert[k] >= v and strict[k] >= v for k,v in {'lat_rob':358., 'fat_sf':1., 'sf_hunter':1.377}.items())
    supported_ok = a4['buckling_eigenvalue'] >= 1. and a4['supported_seq_ratio'] >= cert['seq_ratio']
    sequence_five = [r['seq_ratio'] for r in reversed(gci['plate_gci']['levels_fine_to_coarse'])] + [r['metrics']['seq_ratio'] for r in levels[1:]]
    differences = [b-a for a,b in zip(sequence_five, sequence_five[1:])]
    monotonic = all(d>0 for d in differences) or all(d<0 for d in differences)
    result = dict(name=name, genome=str(genome.relative_to(ROOT)), genome_sha256=digest(genome),
                  canonical_roundtrip_values=True, geometry_legal=geometry_ok, rejects=full['rejects'],
                  config=full['config'], solver=full['solver'], levels=levels, certification=cert,
                  strict=strict, sequence_floor=SEQ_FLOOR, repository_sequence_pass=sequence_ok,
                  prior_internal_125_comparison=cert['seq_ratio']>=1.25 and strict['seq_ratio']>=1.25,
                  prior_strength_targets_pass=strength_ok, gci=gci['plate_gci'],
                  all_five_sequence=sequence_five, all_five_sequence_monotonic=monotonic,
                  export_pass=export_ok, sharpness=measurement['sharpness'],
                  point={k:v for k,v in measurement['point'].items() if k!='samples'},
                  solid=full['solid'], neck_section=full['neck_section'], mass_g=full['mass_g'],
                  a4=a4, G16_13=supported_ok, G16_15_admitted=admission,
                  phase3_scoped_pass=bool(geometry_ok and admission and sequence_ok and strength_ok and export_ok and supported_ok),
                  all_gate_qualification=False,
                  adapter_note='unchanged Phase-2 native adapter reused; its raw task label remains p2',
                  sha256={str(p.relative_to(ROOT)):digest(p) for p in [genome,gci_path,full_path,a4_path,mesh_path]})
    (OUT/f'{name}-summary.json').write_text(json.dumps(result,indent=2)+'\n')
    event(name, 'complete')
    return result

def publish(results, errors, complete):
    results = sorted(results, key=lambda r: ORDER.index(r['name']))
    receipt = dict(task='G16-06-RSV2-08-p3', state='complete' if complete else 'running',
                   cpus=[12,13], cargo_build_jobs=2, max_concurrent_evaluations=2,
                   expected_candidates=9, completed_candidates=len(results), errors=errors,
                   thresholds=dict(repository_sequence=SEQ_FLOOR, fatigue=1., prior_lateral_target_n=358.,
                                   prior_hunter_target=1.377, prior_internal_sequence_comparison=1.25,
                                   sharpness=REQUIREMENTS['gate_s']),
                   scope='certification min(.25,.125) plus separate strict .0625; no prefilter or early numerical rejection',
                   results=results, all_gate_qualification=False,
                   unassessed=['independent 3D P2 solid and cross-solver qualification'],
                   manufacturing_limitation='inherited 16 mm ring collar exceeds 6.35 mm flat stock',
                   source_integrity='sealed genome/config and native/requirement/prototype source hashes checked',
                   sha256={str(p.relative_to(ROOT)):digest(p) for p in [HERE/'candidate.csv',HERE/'candidate_config.json',
                       HERE/'run_phase3.py',OUT.parent/'study',P2/'study',ROOT/'requirements/RS-karambit-v2.toml']})
    (HERE/'phase3_receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
    lines = ['# G16-06-RSV2-08 Phase 3', '',
             f'Status: {receipt["state"]}; {len(results)}/9 candidates fully evaluated. All numerical work uses CPUs 12,13; CARGO_BUILD_JOBS=2. No native source, gate, load, support constant or threshold changed.', '',
             'The assignment uses the frozen repository sequence floor **1.20**. The earlier internal 1.25 prefilter is absent; 1.25 is reported only as a comparison. All three FEM levels, actual exported-triangle sharpness/point/census and side-A4 are evaluated for every candidate, including failures. New interpolation candidates also receive native 4/1/.25 GCI; the three prior fits are reused with exact input/source/mesh identity checks.', '',
             '| Candidate | Cert lat N | Cert fatigue | Cert seq | Strict lat N | Strict fatigue | Strict seq | GCI admitted | Export | A4 | Scoped pass |',
             '| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |']
    for r in results:
        c,t=r['certification'],r['strict']
        lines.append(f'| {r["name"]} | {c["lat_rob"]:.6f} | {c["fat_sf"]:.6f} | {c["seq_ratio"]:.6f} | {t["lat_rob"]:.6f} | {t["fat_sf"]:.6f} | {t["seq_ratio"]:.6f} | {r["G16_15_admitted"]} | {r["export_pass"]} | {r["G16_13"]} | {r["phase3_scoped_pass"]} |')
    lines += ['', 'Scoped pass requires geometry, admitted GCI, sequence >=1.20 and prior strength targets (lateral >=358 N, fatigue >=1, hunter >=1.377) at certification/strict, export and side-A4 (eigenvalue >=1 and supported sequence >=certified). Five-level monotonicity and the old internal 1.25 comparison are reported separately in the receipt. Full RS-v2/manufacturing qualification remains false; human-max failures, soft mass objective and independent 3D checks are retained separately.', '',
              'Exact per-mesh values, GCI fits, exported measurements, support constants, mesh/input hashes and failures are in phase3_receipt.json. Full raw outputs/logs/canonical CSVs/side-A4 meshes remain under target/G16-06-RSV2-08/phase3/. The sealed candidate/config and Phase-2 evidence are preserved.', '',
              'Reproduce with retained lane binaries:', '```sh',
              'taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/run_phase3.py', '```', '']
    (HERE/'PHASE3.md').write_text('\n'.join(lines))

if __name__ == '__main__':
    assert os.sched_getaffinity(0) == {12,13}
    OUT.mkdir(parents=True,exist_ok=True)
    (OUT/'candidates').mkdir(exist_ok=True)
    assert not (OUT/'progress.jsonl').exists(), 'preserve prior Phase-3 evidence before rerunning'
    sealed = json.loads((HERE/'candidate_receipt.json').read_text())
    for name in ('candidate.csv','candidate_config.json'):
        p=HERE/name; assert digest(p)==sealed['sha256'][str(p.relative_to(ROOT))]
    assert digest(OUT.parent/'study') == sealed['sha256'][str((OUT.parent/'study').relative_to(ROOT))]
    prior=json.loads((P2/'baseline_receipt.json').read_text())
    assert digest(P2/'study') == prior['sha256'][str((P2/'study').relative_to(ROOT))]
    verify_sources(json.loads((P2/'sealed-gci.json').read_text())['provenance'])
    jobs=[]
    for name in ('placement_05','local_01','local_04'):
        source=P2/'window_trials'/f'{name}.csv'; genome=OUT/'candidates'/f'{name}.csv'
        genome.write_bytes(source.read_bytes()); jobs.append((name,genome,P2/f'{name}-gci.json'))
    base=list(csv.reader((HERE/'candidate.csv').open()))
    for s1 in (.5003,.5006,.5009,.5012,.5015,.5018):
        name=f'interp_s1_{s1:.4f}'; genome=OUT/'candidates'/f'{name}.csv'
        rows=[list(r) for r in base];rows[1][101]=str(s1)
        with genome.open('w') as f:csv.writer(f,lineterminator='\n').writerows(rows)
        jobs.append((name,genome,None))
    ORDER=[job[0] for job in jobs]
    results=[];errors=[];publish(results,errors,False)
    milestone('milestone 1: complete nine-candidate evaluation started',
              '- Read Phase-3 Assignment in full. All9 assigned candidates will receive .25/.125/.0625 FEM, actual export and side.a4, without the former1.25 prefilter. Repository sequence floor1.20 is read directly from unchanged RS-v2 requirements; prior358N/fatigue1/hunter1.377 targets and1.25 comparison are reported separately.\n- Reusing unchanged native/prototype binaries, skyline for all plate solves. Six new interpolations alter only native WIN_S1; Phase-2 GCI/inputs are preserved. Two concurrent evaluations share the CPU12,13 affinity; CARGO_BUILD_JOBS=2. No PR/push/stacking/deletes or gate changes.')
    with ThreadPoolExecutor(max_workers=2) as pool:
        futures={pool.submit(evaluate,j):j[0] for j in jobs}
        for future in as_completed(futures):
            name=futures[future]
            try:
                row=future.result();results.append(row)
                c,t=row['certification'],row['strict']
                milestone(f'candidate {name} complete',
                          f'- All3 FEM meshes + export + side.a4 evaluated. Cert(lat/fat/seq)={c["lat_rob"]}/{c["fat_sf"]}/{c["seq_ratio"]}; strict={t["lat_rob"]}/{t["fat_sf"]}/{t["seq_ratio"]}. GCI admitted={row["G16_15_admitted"]}; export={row["export_pass"]}; A4={row["G16_13"]}; scoped pass={row["phase3_scoped_pass"]}. Full qualification=false. Raw artifacts and complete measurements retained.')
            except Exception as e:
                errors.append(dict(name=name,error=repr(e)))
                event(name,'ERROR '+repr(e))
            publish(results,errors,False)
    assert len(results)==9 and not errors, errors
    publish(results,errors,True)
    milestone('milestone 11: all nine candidates evaluated',
              f'- Full .25/.125/.0625 FEM/export/side.a4 coverage9/9; errors0. Scoped passing candidates: {[r["name"] for r in results if r["phase3_scoped_pass"]]}. Report PHASE3.md and phase3_receipt.json. Native/source/input/mesh identities verified, thresholds unchanged; full qualification remains false. No PR/push/stacking/deletes.')
