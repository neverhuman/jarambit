#!/usr/bin/env python3
"""Refresh only dependent shared-test hashes; preserve every earlier scientific result."""
import hashlib,json
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1]
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def finalize():
    paths={n:HERE/f'phase{n}_receipt.json' for n in [9,10,11]};p9,p10,r=[json.loads(paths[n].read_text()) for n in [9,10,11]]
    assert r['state']=='complete' and len(r['results'])==36 and not r['errors']
    assert digest(paths[9])==r['phase9_parent_receipt_sha256'] and digest(paths[10])==r['phase10_parent_receipt_sha256']
    rel='ops/ci/tests/test_portable_phase7.py';p9rel=str(paths[9].relative_to(ROOT));new=digest(ROOT/rel)
    r['parent_metadata_updates']=dict(shared_test_path=rel,shared_test_sha256_after=new,phase9_test_sha256_before=p9['sha256'][rel],
        phase10_test_sha256_before=p10['sha256'][rel],phase10_phase9_ref_before=p10['sha256'][p9rel],phase10_shared_test_metadata_before=p10['phase9_metadata_update']['current_sha256'])
    p9['sha256'][rel]=new;paths[9].write_text(json.dumps(p9,indent=2)+'\n')
    p10['sha256'][rel]=new;p10['sha256'][p9rel]=digest(paths[9]);p10['phase9_metadata_update']['current_sha256']=new
    paths[10].write_text(json.dumps(p10,indent=2)+'\n')
    report=HERE/'PHASE11.md'
    complete=[v for v in r['results'] if v['a4']]
    extra=['','Final native observations: '+str(r['outcome_counts'])+'.']
    if complete:
        extra+=['Eigenvalue range: '+str(min(v['a4']['buckling_eigenvalue'] for v in complete))+'..'+str(max(v['a4']['buckling_eigenvalue'] for v in complete))+'. Support sequence margin range: '+str(min(v['margins']['sequence'] for v in complete))+'..'+str(max(v['margins']['sequence'] for v in complete))+'.']
    extra+=['','Earlier receipt updates change only dependent hash metadata: the shared test-file reference in Phase9, and the test/Phase9/current-test references in Phase10. Restoring the recorded old references reconstructs both original receipts byte for byte; every prior measurement, gate decision and report is unchanged. The verifier requires those reconstructions.',
        '','Arithmetic identity checks use relative1e-14 solely for derived load/peak ratios; neither eigenvalue>=1 nor seq_a4>=seq_cert receives any comparison tolerance. Native solves are not replayed by the portable audit.',
        '','Validation: `python3 ops/ci/g16_06_rsv2_08/audit_phase11.py` and `python3 -m unittest ops/ci/tests/test_portable_phase7.py -v`. The assignment explicitly forbids committing or pushing; neither is attempted.','']
    report.write_text(report.read_text()+'\n'.join(extra))
    for p in [paths[9],paths[10],report,HERE/'finalize_phase11.py',ROOT/rel,ROOT/'ops/ci/g16_06_rsv2_08/audit_phase11.py',ROOT/'target/g16-06-rsv2-08/phase11/build_provenance.json']:
        r['sha256'][str(p.relative_to(ROOT))]=digest(p)
    paths[11].write_text(json.dumps(r,indent=2)+'\n')
if __name__=='__main__':finalize()
