#!/usr/bin/env python3
"""Verify the requested structural gates and exported geometry; never promote a design."""
import hashlib
import json
import os
from pathlib import Path
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
OUT = ROOT / 'target/G16-06-RSV2-08'
CONFIG = json.loads((HERE / 'candidate_config.json').read_text())

def matches(path, digest):
    path=Path(path)
    if not path.is_absolute(): path=ROOT/path
    return hashlib.sha256(path.read_bytes()).hexdigest()==digest

def summarize(doc):
    limits = {'lat_rob': 358.0, 'fat_sf': 1.0, 'sf_hunter': 1.377, 'seq_ratio': 1.25}
    cert = [row for row in doc['levels'] if row['area'] in (.25, .125)]
    strict = [row for row in doc['levels'] if row['area'] == .0625]
    assert len(cert) == 2 and len(strict) == 1, 'all certification and strict levels must run'
    legal = doc['within_genome_domain'] and doc['input_was_canonical'] and doc['rs_v2'] and doc['aes_pass'] and not doc['rejects']
    minimum = {key: min(row['metrics'][key] for row in cert) for key in limits}
    strict_metrics = {key: strict[0]['metrics'][key] for key in limits}
    coarse=next(row['metrics'] for row in doc['levels'] if row['area']==2.)
    coarse_observation={key:coarse[key] for key in limits}
    neck = doc['neck_section']
    neck_ok = neck['samples'] > 0 and neck['min_mm'] == 6.0 and neck['max_mm'] == 6.0 and 'comet' not in doc['holes'] and all(name in ['ring', 'cairn'] or bounds[0] >= doc['heel'][0] for name,bounds in zip(doc['holes'],doc['hole_x_bounds']))
    measurement = doc['measurement']
    export_ok = measurement['sharpness']['pass'] and measurement['point']['pass'] and doc['solid']['census'] == [0, 0, 1]
    task_pass = legal and neck_ok and all(minimum[key] >= limit and strict_metrics[key] >= limit for key, limit in limits.items())
    identities_ok = all(abs(row['metrics']['fat_sf'] - row['metrics']['lat_rob'] / 357.996288224513) < 1e-10 for row in doc['levels'])
    unchanged = all(matches(path,digest) for group in ['source_sha256', 'requirements_sha256', 'dependency_source_sha256'] for path, digest in doc['provenance'][group].items())
    prototype_unchanged=all(matches(path,digest) for path,digest in doc['provenance']['prototype_source_sha256'].items())
    known_failures = {str(row['area']): {key: row['metrics'][key] for key in ['hm_twist_sf', 'hm_spine_sf', 'hm_ringback_sf'] if row['metrics'][key] < 1.0} for row in cert + strict}
    return dict(qualification_solver=doc["solver"], task_pass=task_pass, export_pass=export_ok, geometry_legal=legal, full_neck_verified=neck_ok,
                certification_minimum=minimum, strict=strict_metrics, thresholds=limits, coarse_search_observation=coarse_observation, coarse_meets_task_thresholds=all(coarse[key]>=limit for key,limit in limits.items()), qualification_scope='audit protocol: minimum of 0.25/0.125 mm2, plus separate strict 0.0625 mm2 check; area=2 is a search observation',
                fatigue_identity=identities_ok, native_sources_and_requirements_unchanged=unchanged, prototype_sources_unchanged=prototype_unchanged,
                known_other_gate_failures=known_failures, all_gate_qualification=False,
                unassessed=['independent 3D P2 solid and cross-solver battery', 'plate GCI adjudication', 'side.a4 support and buckling battery'],
                construction_scope='prototype compound section and solid neck; native CSV alone does not reproduce it',
                manufacturing_limitation='Inherited ring collar exceeds flat plate stock; no machining release or process qualification.')

if __name__ == '__main__':
    assert os.sched_getaffinity(0) == {12, 13}, 'run with taskset -c 12,13'
    env = {key: value for key, value in os.environ.items() if not key.startswith(('STUDY_', 'JARAMBIT_'))}
    assert CONFIG['JARAMBIT_SOLVER']=='skyline', 'use the native reference backend for qualification'
    env.update(CONFIG, RAYON_NUM_THREADS='2', CARGO_BUILD_JOBS='2')
    result_path = OUT / 'candidate_validation.json'
    with (OUT / 'candidate_validation.log').open('w') as log:
        subprocess.run(['taskset', '-c', '12,13', str(OUT / 'study'), 'all', str(result_path), str(HERE / 'candidate.csv')],
                       cwd=ROOT, env=env, stdout=log, stderr=log, check=True)
    doc = json.loads(result_path.read_text())
    receipt = summarize(doc)
    receipt['sha256'] = {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest()
                         for path in [HERE/'candidate.csv', HERE/'candidate_config.json', OUT/'study', result_path]}
    (OUT / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
    print(json.dumps(receipt, indent=2), flush=True)
    assert receipt['qualification_solver']=='skyline'
    assert receipt['task_pass'], 'requested task gates fail'
    assert receipt['export_pass'], 'export geometry gates fail'
    assert receipt['fatigue_identity'] and receipt['native_sources_and_requirements_unchanged'] and receipt['prototype_sources_unchanged']
