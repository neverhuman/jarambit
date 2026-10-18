#!/usr/bin/env python3
"""Bind the completed grid and refresh only dependent shared-test metadata."""
import sys
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1]
sys.path.insert(0,str(ROOT))
import json,shutil
from ops.ci.g16_06_rsv2_08 import audit_phase13 as audit
OUT=ROOT/'target/g16-06-rsv2-08/phase13'
def legality_notes(rows):
    rejected=[v for v in rows if not v['aesthetic_pass']]
    lines=['','Legality of the reported extrema: '+str(len(rejected))+' cases fail the native aesthetic gate; every0.9mm CP-offset case is rejected for insufficient continuous convex-spine curvature. Their larger numerical capacities remain rejected observations. The receipt retains each native rejection reason.']
    eligible=[v for v in rows if v['geometry_legal'] and v['aesthetic_pass'] and v['fem_complete']]
    if eligible:
        best=max(eligible,key=lambda v:(v['summary']['strict_lat_rob'],v['name']))
        lines.append(f'Among the {len(eligible)} geometry/aesthetic-admitted cases, the largest observed strict lateral result is {best["summary"]["strict_lat_rob"]:.15g}N at `{best["name"]}`. It still fails the unchanged358N target and does not establish qualification.')
    return lines+['']
def finalize():
    receipt=HERE/'phase13_receipt.json';r=json.loads(receipt.read_text());assert r['state']=='complete' and len(r['results'])==24 and not r['errors']
    parents={n:audit.read(ROOT/audit.master.receipt_rel(n)) for n in [9,10,11,12]};olddir=OUT/'parent_receipts_before';olddir.mkdir()
    for n in parents:
        p=ROOT/audit.master.receipt_rel(n);assert audit.digest(p)==r['parent_receipts_before_metadata_sha256'][str(n)]
        shutil.copy2(p,olddir/f'phase{n}_receipt.json')
    history={}
    for n,fields in audit.METADATA_FIELDS.items():
        history[str(n)]=[]
        for keys in fields:
            before=audit.master.get_field(parents[n],keys);after=audit.metadata_after(ROOT,keys,parents)
            audit.master.set_field(parents[n],keys,after);history[str(n)].append(dict(keys=keys,before=before,after=after))
        (ROOT/audit.master.receipt_rel(n)).write_text(json.dumps(parents[n],indent=2)+'\n')
    r['parent_metadata_updates']=history
    report=HERE/'PHASE13.md';complete=[v for v in r['results'] if v['fem_complete']]
    lines=['','Final observed outcome counts: '+str(r['outcome_counts'])+'.','']
    for k in ['lat_rob_cert','strict_lat_rob','fat_sf','seq_ratio_cert','twist_tip_sf','twist_tipzone_sf_strict','hm_twist_sf']:
        if complete:
            best=max(complete,key=lambda v:(v['summary'][k],v['name']))
            lines.append(f'{k}: {min(v["summary"][k] for v in complete):.15g}..{max(v["summary"][k] for v in complete):.15g}; largest observed at `{best["name"]}`.')
    lines+=legality_notes(r['results'])
    lines+=['','The requested shared unittest extension changes only dependent hash metadata in Phase9..12. Restoring the recorded old references reconstructs every original receipt byte for byte; original measurements, Pareto ranking, decisions and reports remain unchanged. No historical checker or gate is weakened.',
        '','Native CP movement is checked to floating-point arithmetic precision (relative1e-12/absolute5e-13mm); torsional capacity identities use relative1e-14. These checks do not add a tolerance to any engineering gate. Raw files are optional when absent and mandatory integrity checks when present.',
        '','Validation: `nice -n 10 taskset -c 12,13 python3 ops/ci/g16_06_rsv2_08/audit_phase13.py`; `nice -n 10 taskset -c 12,13 python3 -m unittest ops/ci/tests/test_portable_phase7.py -v`; `git diff --check`. No staging, commit or push.','']
    report.write_text(report.read_text()+'\n'.join(lines))
    refs=[report,Path(__file__).resolve(),ROOT/audit.TEST,ROOT/'ops/ci/g16_06_rsv2_08/audit_phase13.py',OUT/'build_provenance.json']
    for n in parents:refs.extend([ROOT/audit.master.receipt_rel(n),olddir/f'phase{n}_receipt.json'])
    for p in refs:r['sha256'][str(p.relative_to(ROOT))]=audit.digest(p)
    receipt.write_text(json.dumps(r,indent=2)+'\n')
if __name__=='__main__':finalize()
