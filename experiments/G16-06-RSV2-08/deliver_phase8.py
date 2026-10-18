#!/usr/bin/env python3
"""Prepare verified delivery, attempt the authorized commit, record sandbox limits."""
from datetime import datetime,timezone
import fcntl,json,os,subprocess,tarfile
from pathlib import Path
from prepare_phase5 import HERE,ROOT,digest
from run_phase8 import OUT,milestone

if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10 and os.environ['CARGO_BUILD_JOBS']=='2'
    branch=subprocess.check_output(['git','branch','--show-current'],text=True).strip()
    assert branch=='lane/G16-06-RSV2-08',branch
    rp=HERE/'phase8_receipt.json';r=json.loads(rp.read_text());assert r['state']=='complete' and all(v['exit_code']==0 for v in r['validation'].values())
    message=OUT/'commit-message.txt';message.write_text('''exp(g16-06-rsv2-08): portable Phase7 audits and Phase8 taper refinement

Verify the 144-case Phase7 capsule without target artifacts. Retain strict checks
for available raw evidence and regression tests for portability and corruption.
Measure twelve five-mesh taper/tip refinements with two parent replays; retain
unchanged gates and the bounded negative joint qualification result.

Agent-Session: 1574779.jarambit_lane_lat_fat
Agent-Model: gpt-6.1-sol high
Agent-Task: G16-06-RSV2-08-p8
''')
    r['sha256'][str((HERE/'deliver_phase8.py').relative_to(ROOT))]=digest(HERE/'deliver_phase8.py')
    r['sha256'][str((OUT/'ASSIGNMENT.md').relative_to(ROOT))]=digest(OUT/'ASSIGNMENT.md')
    r['requested_delivery'].update(external_copy='blocked by session writable roots; exact local delivery prepared',commit='outcome recorded in target/G16-06-RSV2-08/phase8/delivery_status.json')
    rp.write_text(json.dumps(r,indent=2)+'\n')
    status=f'''# G16-06-RSV2-08 — Phase8 measured and portable validation complete

Phase7 audits are portable across relocated clean data/checkouts with no target directory, Git metadata, native executables, Cargo environment or CPU-mask dependency. All144 Phase7 cases/720 observations and original report/receipt hashes are verified. Available raw artifacts remain strictly checked; absent raw evidence is explicitly not replayed. Phase7 scientific data and all acceptance thresholds remain unchanged.

Phase8 measures12 exact refinements of the best6.4/6.6 Phase7 parent genomes: taper138/144 x tip2.6/2.8/3.0. All12 canonical/geometry legal,60 requested mesh observations plus10 parent replay observations, native GCI6/12, numerical/joint passes0, selected export1/2 and A4=2/2. Lateral/fatigue cert/strict floors pass; spine and certification sequence fail for all12. Full qualification=false; no global infeasibility/optimum claim. Source, loads, masks, supports and gates unchanged; the inherited unrelated CLI revision is documented. CPU12,13/nice -n10 prefix (effective19)/Cargo jobs2 maintained.

Both portable audit commands and unittest discovery exit0; all10 tests pass. All three commands also pass from a181-file relocated tracked-only capsule without target or .git. Initial missing-file mock failures and sources are retained. See PHASE8.md/phase8_receipt.json and target/G16-06-RSV2-08/phase8/.

External reports destination /home/ubuntu/jarambit-audits/2026-10-18-g16-06-rsv2-08/ is outside writable roots; verified local report/receipt delivery is prepared for Captain relay. Authorized commit outcome is recorded in target/G16-06-RSV2-08/phase8/delivery_status.json and the ignored lane handoff. Exact Agent-* trailers are in commit-message.txt. No PR/push/stacking/deletes. Required CI must be confirmed before any later push.
'''
    (HERE/'STATUS.md').write_text(status)
    handoff=f'''\n## {datetime.now(timezone.utc).isoformat(timespec='seconds')} HANDOFF G16-06-RSV2-08-p8\n- Portable Phase7 verifies144/720 with unchanged archival hashes; missing target is explicit, present raw corruption fatal. All10 unit tests + requested audit7/audit8 commands exit0, also in181-file relocated target-free/Git-free fixture. Phase8:12 legal/60+10 mesh observations/GCI6/12/export1/2/A4=2/2/joint0. All gates/thresholds unchanged; full qualification=false.\n- Requested commit attempt follows on lane/G16-06-RSV2-08 using exact Agent-Session1574779.jarambit_lane_lat_fat / Agent-Model gpt-6.1-sol high / Agent-Task G16-06-RSV2-08-p8. Outcome in target/G16-06-RSV2-08/phase8/delivery_status.json and ignored handoff. Four report/receipt files and source/raw archives prepared locally; external archive path outside writable roots requires Captain relay. No PR/push/stacking/deletes; await next milestone.\n'''
    with open('/home/ubuntu/jarambit/.agent/COMMS.lock','rb') as lock:
        fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
        with (ROOT/'COMMS.md').open('a') as f:f.write(handoff)
        fcntl.flock(lock,fcntl.LOCK_UN)
    add=subprocess.run(['git','add','--','experiments/G16-06-RSV2-08','COMMS.md'],text=True,capture_output=True)
    commit=None
    if add.returncode==0:commit=subprocess.run(['git','commit','-F',str(message)],text=True,capture_output=True)
    committed=commit is not None and commit.returncode==0
    delivery_status=dict(branch=branch,staging_exit_code=add.returncode,staging_output=add.stdout+add.stderr,
      commit_exit_code=commit.returncode if commit else None,commit_output=commit.stdout+commit.stderr if commit else None,
      committed=committed,head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),
      external_copy='not permitted outside session writable roots; Captain relay required',push_or_PR=False)
    (OUT/'delivery_status.json').write_text(json.dumps(delivery_status,indent=2)+'\n')
    if not committed:
        assert add.returncode!=0 or commit.returncode!=0
        r['requested_delivery']['commit']='blocked; '+(add.stderr if add.returncode else commit.stderr).strip()
        rp.write_text(json.dumps(r,indent=2)+'\n')
        (HERE/'STATUS.md').write_text(status+'\nCommit attempt did not succeed: '+r['requested_delivery']['commit']+'\n')
    milestone('milestone 5: flocked COMMS handoff and requested commit attempt',f'- Commit succeeded={committed}; staging exit={add.returncode}; HEAD={delivery_status["head"]}; output={delivery_status["staging_output"].strip()}. Exact requested Agent-* trailers used/prepared. External archive destination outside writable roots; local exact report/receipt delivery follows. All three requested commands/10 tests and relocated capsule proof pass; no native gate/threshold changes. No PR/push/stacking/deletes.')
    directory=OUT.parent/'phase8-delivery';directory.mkdir()
    for name in ['PHASE7.md','phase7_receipt.json','PHASE8.md','phase8_receipt.json']:(directory/name).write_bytes((HERE/name).read_bytes())
    source_files=sorted(p for p in HERE.rglob('*') if p.is_file() and '__pycache__' not in p.parts)+[ROOT/'COMMS.md']
    raw_files=sorted(p for p in OUT.rglob('*') if p.is_file() and '__pycache__' not in p.parts)
    raw_files+=sorted(p for p in OUT.parent.glob('phase8-*.log') if p.name!='phase8-delivery.log')
    files={str(p.relative_to(ROOT)):digest(p) for p in source_files+raw_files}
    archives={}
    for name,paths in [('phase8-sources.tar.gz',source_files),('phase8-raw.tar.gz',raw_files),('phase8-delivery.tar.gz',list(directory.iterdir()))]:
        archive=OUT.parent/name;assert not archive.exists()
        with tarfile.open(archive,'w:gz') as tar:
            for p in paths:tar.add(p,arcname=str(p.relative_to(ROOT)),recursive=False)
        with tarfile.open(archive,'r:gz') as tar:
            for p in paths:assert tar.extractfile(str(p.relative_to(ROOT))).read()==p.read_bytes(),p
        archives[str(archive.relative_to(ROOT))]=digest(archive)
    manifest=OUT.parent/'phase8-artifact-manifest.json';assert not manifest.exists()
    manifest.write_text(json.dumps(dict(files=files,archives=archives,delivery={str(p.relative_to(ROOT)):digest(p) for p in directory.iterdir()}),indent=2)+'\n')
    for rel,want in files.items():assert digest(ROOT/rel)==want,rel
    for p in directory.iterdir():assert p.read_bytes()==(HERE/p.name).read_bytes()
    milestone('milestone 6: local delivery and archives verified',f'- Verified{len(files)} source/raw files and all3 archives; exact unchanged Phase7 and final Phase8 report/receipt delivery pair ready. Manifest SHA256={digest(manifest)}; archives={archives}. Commit succeeded={committed}; external copy blocked by writable roots, Captain relay/checkpoint required if not committed. Receipt SHA256={digest(rp)}. All results retained, no gate/threshold relaxation; no PR/push/deletes. Final post-archive verification follows.')
    print(json.dumps(dict(delivery_status=delivery_status,verified_files=len(files),archives=archives,manifest_sha256=digest(manifest),receipt_sha256=digest(rp)),sort_keys=True),flush=True)
