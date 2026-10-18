#!/usr/bin/env python3
"""Bind final shared-test metadata without changing Phase9 scientific results."""
import hashlib,json
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1]
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def scientific(r):return hashlib.sha256(json.dumps({k:v for k,v in r.items() if k!='sha256'},sort_keys=True,separators=(',',':')).encode()).hexdigest()
def finalize():
    p9_path=HERE/'phase9_receipt.json';p10_path=HERE/'phase10_receipt.json';p9=json.loads(p9_path.read_text());r=json.loads(p10_path.read_text())
    assert r['state']=='complete' and len(r['results'])==36 and not r['errors']
    assert digest(p9_path)==r['phase9_parent_receipt_sha256'] and scientific(p9)==r['phase9_scientific_sha256']
    rel='ops/ci/tests/test_portable_phase7.py';old=p9['sha256'][rel];new=digest(ROOT/rel)
    r['phase9_metadata_update']=dict(path=rel,original_sha256=old,current_sha256=new,reason='Phase10 adds tests to the shared requested unittest file; all Phase9 scientific fields unchanged')
    p9['sha256'][rel]=new;assert scientific(p9)==r['phase9_scientific_sha256']
    p9_path.write_text(json.dumps(p9,indent=2)+'\n')
    report=HERE/'PHASE10.md'
    report.write_text(report.read_text()+'\nShared test metadata: Phase9 receipt changes only the SHA256 entry for ops/ci/tests/test_portable_phase7.py, which now includes Phase10 tests. The original receipt hash is '+r['phase9_parent_receipt_sha256']+'. Restoring that single original digest reconstructs the original receipt byte for byte. Its scientific-data fingerprint and PHASE9.md hash are unchanged.\n')
    report.write_text(report.read_text()+'\nReference arithmetic retains the repository Unicode15 identity contract (Python3.12 here). Cross-language p/extrapolation/GCI numeric comparisons use relative1e-12/absolute1e-14 only to compare libm arithmetic; all validity/admission decisions, mesh identities, method constants and gate predicates compare exactly without an added tolerance.\n')
    for p in [p9_path,HERE/'PHASE9.md',report,HERE/'finalize_phase10.py',ROOT/rel,ROOT/'ops/ci/g16_06_rsv2_08/audit_phase10.py',ROOT/'target/g16-06-rsv2-08/phase10/build_provenance.json']:
        r['sha256'][str(p.relative_to(ROOT))]=digest(p)
    p10_path.write_text(json.dumps(r,indent=2)+'\n')
if __name__=='__main__':finalize()
