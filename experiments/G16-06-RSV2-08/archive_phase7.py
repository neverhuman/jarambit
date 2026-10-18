#!/usr/bin/env python3
"""Prepare verified lane-local handoff and archives; external delivery needs Captain."""
from datetime import datetime, timezone
import fcntl
import json
import os
import subprocess
import tarfile
from pathlib import Path
from prepare_phase5 import HERE, ROOT, digest
from run_phase7 import OUT, milestone

if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10
    r=json.loads((HERE/'phase7_receipt.json').read_text())
    assert r['state']=='complete' and r['integrity_audit']['status']=='pass'
    proof=OUT.parent/'phase7-readonly-proof.log'
    r['read_only_audit_execution_proof']=json.loads(proof.read_text())
    r['sha256'][str(proof.relative_to(ROOT))]=digest(proof)
    counts=r['outcome_counts'];head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
    delivery=OUT.parent/'phase7-delivery';delivery.mkdir()
    status=f'''# G16-06-RSV2-08 — Phase-7 evaluated; Captain checkpoint/delivery pending

Authorized native taper/fullers are active on the Q7 profile. All144 requested ring/full/window inputs are native canonical and geometry/aesthetic legal, with720 five-mesh observations plus15 independent taper-control observations. All36 fuller groups have identical planform meshes and four distinct plate thickness hashes at each GCI level; each taper gene independently changes thickness. All CPs/spine offset/material preserved. Production FEM, loads/masks/supports, native bounds/gates and thresholds unchanged. CPUs12,13, nice -n10 prefix (effective19), CARGO_BUILD_JOBS=2, faer.

Outcome counts: {counts}. Certification/strict sequence maxima: {max(v['certification']['seq_ratio'] for v in r['results']):.12f} / {max(v['strict']['seq_ratio'] for v in r['results']):.12f}. Numerical floors are jointly required; sequence alone cannot qualify. Ten leading distinct configurations receive actual export/certification and native A4. Full RS-v2/manufacturing qualification remains false: human-max twist, independent3D/cross-solver and inherited collar/flat-stock constraints remain unclosed. No global infeasibility or optimum claim.

Read-only audit PASS directly on both CPU12,13 and8,9;77 native source/requirements digests, unchanged FEM except two observation hooks, exact canonical/material/CP identities, native quadrature/GCI/A4 mesh bindings, fatigue identities and unrounded thresholds verified. Native GCI tests2/2 and requirements consistency1/1 pass. See PHASE7.md, phase7_receipt.json, audit_phase7.py and Phase7 helpers/candidates. Complete raw evidence is under target/G16-06-RSV2-08/phase7/; source/raw archives, delivery pair and manifest are in its parent.

Captain must confirm required CI before pushing; the full d1-gates workflow was not rerun locally. Captain must commit and push this worktree, then relay the verified PHASE7.md/phase7_receipt.json pair to /home/ubuntu/jarambit-audits/2026-10-18-g16-06-rsv2-08/ (outside this session's writable roots). Snapshot HEAD {head}; no Phase7 commit, PR or push is claimed. Lane COMMS HANDOFF appended under shared flock; shared-main COMMS requires Captain relay. No stacking/deletes/extra designs.

Required commit trailers:
```
Agent-Session: 1574779.jarambit_lane_lat_fat
Agent-Model: gpt-6.1-sol high
Agent-Task: G16-06-RSV2-08-p7
```
'''
    (HERE/'STATUS.md').write_text(status)
    failures=[v for v in r['results'] if v['export_pass'] is False]
    assert len(failures)==3 and all(v['sharpness']['pass'] is False and v['point']['pass'] for v in failures)
    report=HERE/'PHASE7.md';text=report.read_text()
    summary='144/144 cases are canonical and geometry-legal. Fuller activation is verified in36/36 matched groups, and both taper genes are independently active. Numerical passes0/144; native GCI admissions12/144; selected actual export passes7/10; A4 passes10/10; joint passes0. All gates and thresholds remain unchanged; full qualification=false.'
    text=text.replace('# G16-06-RSV2-08 Phase 7\n', '# G16-06-RSV2-08 Phase 7\n\n'+summary+'\n',1)
    text+='\nSelected export failures (unchanged sharpness gate): '+', '.join(v['name']+' half-angle='+str(v['sharpness']['max_half_angle_deg'])+'deg' for v in failures)+'. Point checks pass for these cases. No mesh/profile/threshold workaround was applied.\n'
    text+='\nDirect read-only audit on8,9 also passed with CARGO_BUILD_JOBS and PYTHONDONTWRITEBYTECODE unset; all2232 experiment/raw files retained identical sizes/mtimes.\n'
    report.write_text(text)
    stamp=datetime.now(timezone.utc).isoformat(timespec='seconds')
    handoff=f'''\n## {stamp} HANDOFF G16-06-RSV2-08-p7\n- Phase7 evaluated:144/144 legal cases,720 sweep+15 control mesh observations. All36 fuller groups have four distinct thickness hashes on identical native GCI meshes; both taper genes individually active. Counts={counts}. Native gates/thresholds unchanged; full qualification=false.\n- PHASE7.md/phase7_receipt.json/audit_phase7.py ready; audit read-only and PASS on12,13 and8,9; native tests3/3 PASS. Full raw/source archives and verified local delivery pair under target/G16-06-RSV2-08/. External archive destination outside writable roots; Captain relay required. Captain commit/push requested; shared Git metadata is read-only in this session. HEAD {head}; no Phase7 commit/PR/push claimed. Required trailers Agent-Session1574779.jarambit_lane_lat_fat; Agent-Model gpt-6.1-sol high; Agent-Task G16-06-RSV2-08-p7. No stacking/deletes. Await Captain milestone.\n'''
    with open('/home/ubuntu/jarambit/.agent/COMMS.lock','rb') as lock:
        fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
        with (ROOT/'COMMS.md').open('a') as f:f.write(handoff)
        fcntl.flock(lock,fcntl.LOCK_UN)
    milestone('milestone 6: Captain-ready results and flocked COMMS handoff',f'- Counts={counts}. Read-only audit both CPU pairs/native tests pass; all144/720+15 observations retained. No gate/threshold changes; full qualification=false. Captain commit/push and external-archive relay required; lane HANDOFF appended under shared flock. Local delivery/archive verification follows. No PR/push/stacking/deletes.')
    r['sha256'][str((HERE/'archive_phase7.py').relative_to(ROOT))]=digest(HERE/'archive_phase7.py')
    (HERE/'phase7_receipt.json').write_text(json.dumps(r,indent=2)+'\n')
    for name in ['PHASE7.md','phase7_receipt.json']:(delivery/name).write_bytes((HERE/name).read_bytes())
    files=sorted(p for p in OUT.rglob('*') if p.is_file())
    files+=sorted(p for p in OUT.parent.glob('phase7-*.log') if p.name!='phase7-archive.log')
    sources=sorted(p for p in HERE.rglob('*') if p.is_file() and '__pycache__' not in p.parts)
    manifest={str(p.relative_to(ROOT)):digest(p) for p in files+sources+[ROOT/'COMMS.md']}
    archives={}
    for name,paths in [('phase7-sources.tar.gz',sources+[ROOT/'COMMS.md']),('phase7-raw.tar.gz',files),('phase7-delivery.tar.gz',list(delivery.iterdir()))]:
        archive=OUT.parent/name;assert not archive.exists()
        with tarfile.open(archive,'w:gz') as tar:
            for p in paths:tar.add(p,arcname=str(p.relative_to(ROOT)),recursive=False)
        with tarfile.open(archive,'r:gz') as tar:
            for p in paths:
                content=tar.extractfile(str(p.relative_to(ROOT))).read()
                assert content==p.read_bytes(),p
        archives[str(archive.relative_to(ROOT))]=digest(archive)
    for rel,want in manifest.items():assert digest(ROOT/rel)==want,rel
    for name in ['PHASE7.md','phase7_receipt.json']:assert (delivery/name).read_bytes()==(HERE/name).read_bytes()
    manifest_path=OUT.parent/'phase7-artifact-manifest.json'
    assert not manifest_path.exists()
    manifest_path.write_text(json.dumps(dict(files=manifest,archives=archives,delivery={str(p.relative_to(ROOT)):digest(p) for p in delivery.iterdir()}),indent=2)+'\n')
    milestone('milestone 7: local archives and delivery pair verified',f'- Verified{len(manifest)} source/raw hashes plus every archived file; PHASE7.md/receipt delivery bytes match canonical pair. Archives={archives}; manifest SHA256={digest(manifest_path)}; receipt SHA256={digest(HERE/"phase7_receipt.json")}. External /home/ubuntu/jarambit-audits/2026-10-18-g16-06-rsv2-08/ remains outside writable roots; Captain relay required. Captain-ready worktree, no agent commit/PR/push claimed; no stacking/deletes. Await next milestone.')
    print(json.dumps(dict(verified_files=len(manifest),archives=archives,manifest_sha256=digest(manifest_path),receipt_sha256=digest(HERE/'phase7_receipt.json'))),flush=True)
