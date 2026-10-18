#!/usr/bin/env python3
"""Portable, read-only audit of the immutable tracked Phase7 study.

No Cargo artifacts, native executable, Git metadata or CPU mask is required.
Available ignored raw artifacts receive additional checks; absence is explicit.
"""
import sys
sys.dont_write_bytecode=True
import itertools,json,struct,hashlib
from collections import defaultdict
from pathlib import Path
from portable_study_audit import ROOT,read,digest,require,check_rows,check_report,check_raw,raw_path,margin
RECEIPT_SHA256='71e751199151a99c61058625d55ad2358485e69e308b101d383b16b7ec8cee4a'
REPORT_SHA256='18c3ef60e9edf901174300a800e98128e2ce4fefa3398669d148bfa1864c828c'

def audit_document(root,r,text):
    groups=check_rows(root,r,7);check_report(r,text,7)
    require(r['task']=='G16-06-RSV2-08-p7' and len(r['results'])==144 and r['mesh_observations']==720,'Phase7 scope mismatch')
    expected=set(itertools.product([6.2,6.4,6.6,6.8],[0.,.45,.75,1.05],[3.,4.5,6.],[.05,.25,.45]))
    require({tuple(v['genes'][i] for i in [42,74,102,100]) for v in r['results']}==expected,'Phase7 Cartesian grid mismatch')
    for v in r['results']:
        require((v['genes'][46],v['genes'][47],v['genes'][101],v['genes'][103])==(2.6,115.,.85,.65),'Fixed Phase7 genes changed')
    require(len(groups)==36,'Fuller group count mismatch')
    for rows in groups.values():
        require(sorted(v['genes'][74] for v in rows)==[0.,.45,.75,1.05],'Missing fuller level')
        for i in range(3):require(len({v['gci']['levels_fine_to_coarse'][i]['mesh_sha256'] for v in rows})==4,'Masked fuller in GCI hash')
    a,b,c=r['law_effect_controls']
    require([(v['genes'][46],v['genes'][47],v['genes'][74]) for v in [a,b,c]]==[(2.6,115.,.75),(3.5,115.,.75),(2.6,130.,.75)],'Taper controls changed')
    for variant,index in [(b,46),(c,47)]:require([i for i,(x,y) in enumerate(zip(a['genes'],variant['genes'])) if x!=y]==[index],'Taper control confounded')
    for i in range(3):require(len({v['gci']['levels_fine_to_coarse'][i]['mesh_sha256'] for v in [a,b,c]})==3,'Masked taper control')
    require(len(r['selection'])==10,'Export coverage mismatch')
    ranked=sorted(r['results'],key=lambda v:(-v['fine_targets_pass'],-margin(v),-v['gci_admitted'],v['name']))
    picked=[];seen=set()
    for row in ranked:
        key=tuple(l['mesh_sha256'] for l in row['gci']['levels_fine_to_coarse'])
        if key in seen:continue
        seen.add(key);picked.append(dict(name=row['name'],rank=len(picked)+1,margin=margin(row),three_level_mesh_hashes=list(key)))
        if len(picked)==10:break
    require(picked==r['selection'],'Export ranking mismatch')
    for control in r['ring_controls']:
        require(control['legal'] and not control['rejects'] and not control['aes_fails'],'Ring control rejected')
        require(60<=control['grip_len_mm']<=80 and control['holder_fit'][0] and control['holder_fit'][1]<=200 and control['holder_fit'][2]<=100,'Ring grip/holder threshold failed')
    return groups

def raw_details(root,r,groups):
    field_groups=defaultdict(list);fields=0;fem=0;a4_meshes=0
    for row in r['results']+r['law_effect_controls']:
        prefix=f'target/G16-06-RSV2-08/phase7/{row["name"]}'
        full=raw_path(root,prefix+'-fem.json')
        if full:
            d=read(full);require(d['levels']==row['levels'] and d['genes']==row['genes'] and d['config']==row['config'],'Raw FEM differs from receipt')
            require(d['cpu_affinity'].split(':')[1].strip()=='12-13' and d['nice_priority']>=10 and d['cargo_build_jobs']=='2','Raw execution limits changed');fem+=1
        field=raw_path(root,prefix+'-field.json')
        if field:
            f=read(field);require(f['genome_sha256']==row['genome_sha256'] and f['native_fem_quadrature'],'Raw field genome mismatch')
            for l,g in zip(f['levels'],row['gci']['levels_fine_to_coarse']):require(l['area']==g['max_area_mm2'] and l['mesh_thickness_sha256']==g['mesh_sha256'],'Raw quadrature/GCI binding mismatch')
            field_groups[tuple(row['genes'][i] for i in [42,100,101,102,103])].append((row,f));fields+=1
        mesh=raw_path(root,prefix+'-a4.mesh.json')
        if mesh:
            d=read(mesh);binary=b''.join(struct.pack('<dd',*q) for q in d['xy'])+b''.join(struct.pack('<QQQ',*t) for t in d['tris'])+b''.join(struct.pack('<d',t) for t in d['te'])
            require(hashlib.sha256(binary).hexdigest()==row['gci']['levels_fine_to_coarse'][0]['mesh_sha256'],'Raw A4/GCI mesh mismatch');a4_meshes+=1
    verified=0
    for values in field_groups.values():
        values=[(row,f) for row,f in values if row['name'] in {v['name'] for v in r['results']}]
        if len(values)!=4:continue
        for i in range(3):
            require(len({f['levels'][i]['xy_triangles_sha256'] for row,f in values})==1,'Fuller planform changed')
            require(len({f['levels'][i]['thickness_sha256'] for row,f in values})==4,'Fuller thickness hashes coincide')
        verified+=1
    return dict(fem_documents=fem,field_documents=fields,split_hash_groups=verified,a4_meshes=a4_meshes)

def audit(root=ROOT):
    root=Path(root);here=root/'experiments/G16-06-RSV2-08';receipt=here/'phase7_receipt.json';report=here/'PHASE7.md'
    require(digest(receipt)==RECEIPT_SHA256,'Immutable Phase7 receipt SHA-256 mismatch')
    require(digest(report)==REPORT_SHA256,'Immutable Phase7 report SHA-256 mismatch')
    r=read(receipt);text=report.read_text();groups=audit_document(root,r,text)
    # This task intentionally replaces the historical checker, not its scientific data.
    for rel,want in r['sha256'].items():
        if rel.startswith('target/') or rel=='experiments/G16-06-RSV2-08/audit_phase7.py':continue
        require(digest(root/rel)==want,f'Tracked Phase7 input SHA-256 mismatch: {rel}')
    raw=check_raw(root,r);raw.update(raw_details(root,r,groups))
    return dict(status='pass',cases=144,mesh_observations=720,control_mesh_observations=15,
        outcome_counts=r['outcome_counts'],tracked_receipt_and_report_verified=True,
        raw_evidence=raw,verification_scope='tracked results plus available raw evidence; native solves are not replayed',
        all_gate_qualification=False)

if __name__=='__main__':print(json.dumps(audit(),sort_keys=True))
