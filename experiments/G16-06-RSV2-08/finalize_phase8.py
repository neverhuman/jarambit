#!/usr/bin/env python3
"""Finish tracked evidence and portable validation; preserve failed attempts."""
import json,os,subprocess,sys
from pathlib import Path
from prepare_phase5 import HERE,ROOT,digest
from run_phase8 import OUT,milestone

if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10 and os.environ['CARGO_BUILD_JOBS']=='2'
    receipt=HERE/'phase8_receipt.json';r=json.loads(receipt.read_text())
    assert r['state']=='complete' and not r['errors'] and r['completed_cases']==12
    report=HERE/'PHASE8.md';text=report.read_text()
    lines=['','## Five-mesh observations','',
      '| Case | Area mm² | Spine SF | Lateral N | Fatigue SF | Hunter SF | Sequence | Spine peak x,y |',
      '| --- | --- | --- | --- | --- | --- | --- | --- |']
    for row in r['results']:
        for l in row['levels']:
            m=l['metrics'];peak=l['diagnostic_peaks']['spine']['at']
            lines.append(f'| {row["name"]} | {l["area"]} | {m["hm_spine_sf"]:.9f} | {m["lat_rob"]:.9f} | {m["fat_sf"]:.9f} | {m["sf_hunter"]:.9f} | {m["seq_ratio"]:.9f} | {peak} |')
    lines+=['','Portable Phase7 verification preserves the immutable original report/receipt hashes and checks all144 cases/720 observations. Missing ignored target evidence is reported separately from tracked-result verification; present raw corruption fails. No native solve is claimed by a read-only audit. Audit execution has no CPU-mask or Cargo environment dependency.','',
      'Validation retains the first failed portability-test attempt under target/ with its scripts/log. The missing-file exception classification was fixed; no numerical bound, acceptance threshold or measured result changed.','']
    summary=f'All12 refinement inputs are canonical/geometry-legal; native GCI admissions={r["outcome_counts"]["gci_admitted"]}; fine numerical and joint passes=0. All12 meet certification/strict lateral358N and fatigue1 floors, but every case misses certification sequence1.20 and spine1. Maximum certification/strict spine={max(v["certification"]["hm_spine_sf"] for v in r["results"]):.12f}/{max(v["strict"]["hm_spine_sf"] for v in r["results"]):.12f}; maximum certification/strict sequence={max(v["certification"]["seq_ratio"] for v in r["results"]):.12f}/{max(v["strict"]["seq_ratio"] for v in r["results"]):.12f}. Selected export passes={r["outcome_counts"]["export_pass"]}/2; A4 passes={r["outcome_counts"]["side_a4_pass"]}/2. Full qualification=false; all thresholds unchanged.'
    text=text.replace('# G16-06-RSV2-08 Phase 8\n','# G16-06-RSV2-08 Phase 8\n\n'+summary+'\n',1)
    report.write_text(text+'\n'.join(lines))
    r['sha256'].update({str(p.relative_to(ROOT)):digest(p) for p in [HERE/'audit_phase7.py',HERE/'audit_phase8.py',HERE/'portable_study_audit.py',HERE/'test_portable_phase7.py',HERE/'finalize_phase8.py',report]})
    r['phase7_portability']=dict(receipt_sha256=digest(HERE/'phase7_receipt.json'),report_sha256=digest(HERE/'PHASE7.md'),
        no_target_required=True,cpu_independent_read_only_audit=True,present_raw_corruption_fatal=True,
        historical_phase7_checker_hash_superseded=True,scientific_receipt_unchanged=True)
    r['requested_delivery']=dict(external_path='/home/ubuntu/jarambit-audits/2026-10-18-g16-06-rsv2-08/',external_copy='pending; outside writable roots',commit='pending final attempt',push_or_PR=False)
    receipt.write_text(json.dumps(r,indent=2)+'\n')
    commands=[('audit7',[sys.executable,str(HERE/'audit_phase7.py')]),('audit8',[sys.executable,str(HERE/'audit_phase8.py')]),
      ('unittest',[sys.executable,'-m','unittest','discover','-s',str(HERE),'-p','test_*.py','-v'])]
    env=dict(os.environ,PYTHONDONTWRITEBYTECODE='1')
    validation={}
    for label,cmd in commands:
        log=OUT/f'validation-{label}.log';assert not log.exists()
        result=subprocess.run(cmd,cwd=ROOT,env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
        log.write_text(result.stdout);assert result.returncode==0,(label,result.stdout)
        validation[label]=dict(exit_code=result.returncode,log=str(log.relative_to(ROOT)),sha256=digest(log))
        print(label,'PASS',flush=True)
    # Relocate only the tracked audit/data capsule, without creating a Git working copy.
    fixture=OUT/'portable_capsule';fixture.mkdir()
    p7=json.loads((HERE/'phase7_receipt.json').read_text())
    paths={rel for rel in p7['sha256'] if not rel.startswith('target/')}
    paths|={v['genome'] for v in p7['results']}
    paths|={rel for rel in r['sha256'] if not rel.startswith('target/')}
    paths|={v['genome'] for v in r['results']}
    paths|={str((HERE/name).relative_to(ROOT)) for name in ['phase7_receipt.json','PHASE7.md','audit_phase7.py','portable_study_audit.py','phase8_receipt.json','PHASE8.md','audit_phase8.py','test_portable_phase7.py']}
    for rel in sorted(paths):
        dest=fixture/rel;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes((ROOT/rel).read_bytes())
    relocated={};transcript=[];fixture_here=fixture/'experiments/G16-06-RSV2-08'
    for label,cmd in [('audit7',[sys.executable,str(fixture_here/'audit_phase7.py')]),('audit8',[sys.executable,str(fixture_here/'audit_phase8.py')]),('unittest',[sys.executable,'-m','unittest','discover','-s',str(fixture_here),'-p','test_*.py','-v'])]:
        result=subprocess.run(cmd,cwd=ROOT,env=env,text=True,capture_output=True)
        transcript.append(label+'\n'+result.stdout+result.stderr)
        assert result.returncode==0,(label,result.stdout,result.stderr)
        relocated[label]=json.loads(result.stdout) if label!='unittest' else dict(exit_code=0)
    (OUT/'validation-relocated.log').write_text('\n'.join(transcript))
    assert relocated['audit7']['cases']==144 and relocated['audit7']['mesh_observations']==720 and relocated['audit7']['raw_evidence']['checked']==0
    assert relocated['audit8']['cases']==12 and relocated['audit8']['mesh_observations']==60 and relocated['audit8']['raw_evidence']['checked']==0
    validation['relocated_capsule']=dict(exit_code=0,files=len(paths),target_absent=not(fixture/'target').exists(),results=relocated)
    r['validation']=validation
    receipt.write_text(json.dumps(r,indent=2)+'\n')
    milestone('milestone 4: portable audits and unit tests passed',f'- Exact requested audit7/audit8/unittest commands exit0. Ten portability/integrity tests pass, including absent-target, CPU-independent read-only operation, raw corruption, missing tracked input, changed report/mesh/floor/flag. Actual relocated tracked-only capsule has no target directory and verifies144 cases/720 observations. P8 counts={r["outcome_counts"]}; all gates unchanged. Final delivery/commit attempt follows.')
    print(json.dumps(validation,sort_keys=True),flush=True)
