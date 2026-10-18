#!/usr/bin/env python3
"""Run non-solving thickness binding probes, preserve evidence, finalize local report."""
from concurrent.futures import ThreadPoolExecutor, as_completed
import json
import os
import subprocess
from prepare_phase5 import HERE, ROOT, digest
from run_phase7 import OUT, milestone
from run_phase5 import environment

def probe(row):
    out=OUT/f'{row["name"]}-field.json'
    if not out.exists():
        with out.with_suffix('.log').open('w') as log:
            subprocess.run([str(OUT/'field_probe'),str(ROOT/row['genome']),str(out)],env=environment(),stdout=log,stderr=log,check=True)
    doc=json.loads(out.read_text())
    assert doc['genome_sha256']==row['genome_sha256']
    assert doc['law_sha256']==digest(HERE/'phase7_adapter.rs')
    for f,g in zip(doc['levels'],row['gci']['levels_fine_to_coarse']):assert f['mesh_thickness_sha256']==g['mesh_sha256']
    return out

if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10
    assert os.environ['CARGO_BUILD_JOBS']=='2'
    receipt=HERE/'phase7_receipt.json';r=json.loads(receipt.read_text())
    assert r['state']=='complete' and not r['errors'] and r['completed_cases']==144
    with ThreadPoolExecutor(max_workers=2) as pool:
        futures=[pool.submit(probe,v) for v in r['results']+r['law_effect_controls']]
        for i,f in enumerate(as_completed(futures),1):
            p=f.result();r['sha256'][str(p.relative_to(ROOT))]=digest(p)
            print('field_binding',i,p.name,flush=True)
    test_env=environment();test_env['TMPDIR']=str(OUT/'compiler-tmp')
    for name,flt in [('gci-tests.log','gci::tests'),('requirements-test.log','rs_karambit_v2_matches_code')]:
        with (OUT/name).open('w') as log:
            subprocess.run(['cargo','test','--release','--locked','-p','jarambit-crescent','--lib',flt],env=test_env,stdout=log,stderr=log,check=True)
    for p in [HERE/'audit_phase7.py',HERE/'finish_phase7.py',HERE/'phase7_field.rs',HERE/'build_phase7_field.sh',OUT/'field_probe',OUT/'fuller_effects.json',OUT/'export_selection.json',OUT/'gci-tests.log',OUT/'requirements-test.log',OUT/'field_binding_batch.json',HERE/'probe_phase7_fields.py']:
        r['sha256'][str(p.relative_to(ROOT))]=digest(p)
    r['validation_scope']='Native GCI tests2 and requirements test1 plus full Phase7 data audit; full d1-gates CI pipeline not rerun locally. Captain must confirm required CI before push.'
    r['parent_context']=dict(phase6_commit='d13878d6c415ee188c77ca3dbd3483adb33f0a3a',phase6_report_sha256=digest(HERE/'PHASE6.md'),phase6_audit_commit='14a15469')
    r['delivery']=dict(captain_commit_required=True,external_archive_destination='/home/ubuntu/jarambit-audits/2026-10-18-g16-06-rsv2-08/',external_archive_status='outside writable roots; local verified delivery awaits Captain relay',push_or_PR_opened=False)
    receipt.write_text(json.dumps(r,indent=2)+'\n')
    command=[str(HERE/'audit_phase7.py')]
    env=environment();env['PYTHONDONTWRITEBYTECODE']='1'
    for pair,label in [('12,13','worker'),('8,9','auditor')]:
        result=subprocess.run(['taskset','-c',pair,*command],env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
        (OUT/f'audit-{label}.log').write_text(result.stdout+result.stderr)
        assert result.returncode==0,(pair,result.stderr)
        audit=json.loads(result.stdout)
    r['integrity_audit']=audit
    for p in [OUT/'audit-worker.log',OUT/'audit-auditor.log']:r['sha256'][str(p.relative_to(ROOT))]=digest(p)
    receipt.write_text(json.dumps(r,indent=2)+'\n')
    report=HERE/'PHASE7.md'
    lines=['','## Verified measurements','',f'Read-only audit PASS on CPU12,13 and8,9: {audit}.','',
        r['validation_scope'], '',
        f'Maximum certification/strict sequence: {max(v["certification"]["seq_ratio"] for v in r["results"]):.12f} / {max(v["strict"]["seq_ratio"] for v in r["results"]):.12f}. Both strength and sequence floors are jointly required; passing sequence alone is insufficient.', '',
        '| Case | Area mm² | Spine SF | Lateral N | Fatigue SF | Hunter SF | Sequence | Spine peak x,y |',
        '| --- | --- | --- | --- | --- | --- | --- | --- |']
    for row in r['results']:
        for l in row['levels']:
            m=l['metrics'];peak=l['diagnostic_peaks']['spine']['at']
            lines.append(f'| {row["name"]} | {l["area"]} | {m["hm_spine_sf"]:.9f} | {m["lat_rob"]:.9f} | {m["fat_sf"]:.9f} | {m["sf_hunter"]:.9f} | {m["seq_ratio"]:.9f} | {peak} |')
    lines+=['','| Export/A4 case | Export | GCI | Half-angle ° | Apex mm | Point | Bore mm | Census | A4 eigenvalue | Supported / cert seq |',
        '| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |']
    for row in r['results']:
        if row['export_pass'] is None:continue
        s,a=row['sharpness'],row['a4']
        lines.append(f'| {row["name"]} | {row["export_pass"]} | {row["gci_admitted"]} | {s["max_half_angle_deg"]:.9f} | {s["max_apex_mm"]:.9f} | {row["point"]["pass"]} | {row["bore_edge_radius_mm"]:.9f} | {row["solid"]["census"]} | {a["buckling_eigenvalue"]:.9f} | {a["supported_seq_ratio"]:.9f} / {a["certified_seq_ratio"]:.9f} |')
    report.write_text(report.read_text()+'\n'.join(lines)+'\n')
    milestone('milestone 5: read-only audit passed on both CPU pairs',f'- 144 canonical/geometry-legal cases,720 sweep observations +15 control observations. All36 fuller groups retain identical planform mesh hashes and four distinct thickness hashes on each GCI mesh; both taper genes independently active. All77 production hashes, native FEM two-hook identity, canonical/material/CP identity, exact thresholds, fatigue identities and GCI/A4 bindings verified. GCI tests2/2, requirements1/1 pass. Counts={r["outcome_counts"]}. Full qualification=false; final local archive/handoff follows. Audit creates no files and accepts12,13 or8,9.')
    print(json.dumps(audit),flush=True)
