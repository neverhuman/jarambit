#!/usr/bin/env python3
"""Portable read-only Phase8 receipt/report audit; raw evidence is optional."""
import sys
sys.dont_write_bytecode=True
import itertools,json
from pathlib import Path
from portable_study_audit import ROOT,read,digest,require,check_rows,check_report,check_raw,margin
from audit_phase7 import RECEIPT_SHA256

def audit(root=ROOT):
    root=Path(root);here=root/'experiments/G16-06-RSV2-08';r=read(here/'phase8_receipt.json');text=(here/'PHASE8.md').read_text()
    require(digest(here/'phase7_receipt.json')==RECEIPT_SHA256,'Phase7 parent receipt changed')
    p7=read(here/'phase7_receipt.json');check_rows(root,r,8);check_report(r,text,8)
    require(r['task']=='G16-06-RSV2-08-p8' and r['completed_cases']==12 and r['mesh_observations']==60,'Phase8 scope mismatch')
    require({tuple(v['genes'][i] for i in [42,47,46]) for v in r['results']}==set(itertools.product([6.4,6.6],[138.,144.],[2.6,2.8,3.])),'Refinement Cartesian grid mismatch')
    require(len(r['parents'])==len(r['replay_controls'])==len(r['selection'])==2,'Parent/replay/export coverage mismatch')
    for parent in r['parents']:
        ring=parent['genes'][42]
        expected=sorted([v for v in p7['results'] if v['genes'][42]==ring and v['geometry_legal']],key=lambda v:(-margin(v),v['name']))[0]
        require(parent['name']==expected['name'] and parent['genes']==expected['genes'] and parent['margin']==margin(expected),'Parent is not best Phase7 margin')
        control=next(v for v in r['replay_controls'] if v['genes'][42]==ring)
        require(control['genes']==expected['genes'],'Replay genes changed')
        errors=[abs(a['metrics'][k]-b['metrics'][k])/max(abs(b['metrics'][k]),1e-300) for a,b in zip(control['levels'],expected['levels']) for k in a['metrics'] if isinstance(a['metrics'][k],(int,float)) and isinstance(b['metrics'].get(k),(int,float))]
        require(max(errors)==control['phase7_max_relative_error'] and max(errors)<=1e-9,'Parent replay mismatch')
        cases=[v for v in r['results'] if v['genes'][42]==ring]
        for case in cases:require(all(a==b for i,(a,b) in enumerate(zip(case['genes'],expected['genes'])) if i not in [46,47]),'Refinement changed another parent gene')
        best=sorted(cases,key=lambda v:(-v['fine_targets_pass'],-margin(v),-v['gci_admitted'],v['name']))[0]
        picked=next(v for v in r['selection'] if v['name']==best['name'])
        require(picked['margin']==margin(best),'Export selection margin changed')
    for rel,want in r['sha256'].items():
        if rel.startswith('target/') or rel=='experiments/G16-06-RSV2-08/audit_phase8.py':continue
        require(digest(root/rel)==want,f'Tracked input SHA-256 mismatch: {rel}')
    return dict(status='pass',cases=12,mesh_observations=60,replay_mesh_observations=10,
        outcome_counts=r['outcome_counts'],raw_evidence=check_raw(root,r),
        verification_scope='tracked receipt/report plus available raw hashes; native solves not replayed',all_gate_qualification=False)

if __name__=='__main__':print(json.dumps(audit(),sort_keys=True))
