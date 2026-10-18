#!/usr/bin/env python3
"""Read-only Phase11 A4 evidence audit; raw target files are optional, never replayed."""
import sys
sys.dont_write_bytecode=True
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT))
import hashlib,json,math,struct
from ops.ci.g16_06_rsv2_08 import audit_phase10 as phase10
STUDY=phase10.STUDY
require,digest,read,path,exists=phase10.require,phase10.digest,phase10.read,phase10.path,phase10.exists
line=phase10.line

def verdict(eigenvalue,supported,certified):
    return eigenvalue>=1.,supported>=certified
def document_hash(r):return hashlib.sha256((json.dumps(r,indent=2)+'\n').encode()).hexdigest()
def check_parent_metadata(root,r,p9,p10):
    meta=r['parent_metadata_updates'];rel='ops/ci/tests/test_portable_phase7.py';p9rel=STUDY+'/phase9_receipt.json'
    current=digest(path(root,rel))
    require(meta['shared_test_path']==rel and meta['shared_test_sha256_after']==current,'Shared test metadata mismatch')
    require(p9['sha256'][rel]==p10['sha256'][rel]==p10['phase9_metadata_update']['current_sha256']==current,'Parent shared-test references disagree')
    require(p10['sha256'][p9rel]==digest(path(root,p9rel)),'Parent receipt reference mismatch')
    old9=json.loads(json.dumps(p9));old9['sha256'][rel]=meta['phase9_test_sha256_before']
    require(document_hash(old9)==r['phase9_parent_receipt_sha256'],'Phase9 changed beyond shared-test hash')
    old10=json.loads(json.dumps(p10));old10['sha256'][rel]=meta['phase10_test_sha256_before']
    old10['sha256'][p9rel]=meta['phase10_phase9_ref_before']
    old10['phase9_metadata_update']['current_sha256']=meta['phase10_shared_test_metadata_before']
    require(document_hash(old10)==r['phase10_parent_receipt_sha256'],'Phase10 changed beyond dependent hash metadata')

def audit_document(root,r,text):
    require(r['task']=='G16-06-RSV2-08-p11' and r['state']=='complete' and not r['errors'],'Incomplete Phase11 study')
    require(r['expected_cases']==r['completed_cases']==len(r['results'])==36,'Phase11 case count mismatch')
    require(r['mesh_area_mm2']==.25 and r['certificate_areas_mm2']==[.25,.125],'A4/certificate mesh areas changed')
    require(r['thresholds']==dict(buckling_eigenvalue=1.,sequence='supported_seq_ratio >= Phase9 min(sequence_ratio at .25/.125)',owner_replay_bar=1e-9),'A4 acceptance thresholds changed')
    require(r['native_support_constants']==dict(E_MPA=113800.,NU=.34,bore_radial=50000.,bore_tangential=5000.,bore_flange=100000.,grip_foundation=200.),'Native support constants changed')
    require(r['cpus']==[12,13] and r['nice']>=10 and r['cargo_build_jobs']==2 and r['solver']=='faer' and r['max_concurrent_evaluations']==2,'Execution limits changed')
    require(r['production_sources_changed'] is False and r['all_gate_qualification'] is False,'Source/qualification claim changed')
    p9=read(Path(root)/STUDY/'phase9_receipt.json');p10=read(Path(root)/STUDY/'phase10_receipt.json')
    check_parent_metadata(root,r,p9,p10)
    require(phase10.scientific_hash(p9)==r['phase9_scientific_sha256'],'Phase9 scientific data changed')
    for rel,want in r['parent_reports_sha256'].items():require(digest(path(root,rel))==want,'Prior report changed')
    require(set(r['parent_reports_sha256'])=={STUDY+'/PHASE9.md',STUDY+'/PHASE10.md'},'Parent report coverage changed')
    parents={v['name']:v for v in p9['results']};gci={v['name']:v for v in p10['results']}
    require(len({v['name'] for v in r['results']})==36 and {v['name'] for v in r['results']}==set(parents)==set(gci),'Phase9/10 profile coverage mismatch')
    summary=[];meshes=[]
    for row in r['results']:
        parent=parents[row['name']]
        for k in ['name','family','genome','genome_sha256','genes']:require(row[k]==parent[k],'Parent candidate changed')
        require(exists(path(root,row['genome'])),'Missing tracked Phase11 genome')
        require(digest(path(root,row['genome']))==row['genome_sha256'],'Phase11 genome SHA mismatch')
        require(row['input_canonical'] and row['rs_v2'] and row['native_fem_unmodified'],'Noncanonical/custom native input')
        require(row['production_api']=='jarambit_crescent::battery::run_side_a4_native_for_genome','Native A4 API changed')
        require(row['mesh_area_mm2']==.25 and row['solver']=='faer' and row['cpu_affinity'].split(':')[1].strip()=='12-13' and row['nice']>=10 and row['cargo_build_jobs']==2,'Case execution limits changed')
        require(row['all_gate_qualification'] is False,'Case falsely qualified')
        cert=min(v['result']['metrics']['sequence_ratio'] for v in parent['levels'][:2])
        require(cert==parent['certification']['sequence_ratio']==row['seq_cert_input'],'Certificate sequence minimum changed')
        old=parent['levels'][0]['result'];fine=gci[row['name']]['plate_gci']['levels_fine_to_coarse'][0]
        relative=abs(row['same_mesh_ring_clamp_seq_ratio']-old['metrics']['sequence_ratio'])/max(abs(old['metrics']['sequence_ratio']),1e-300)
        binding=dict(phase9_hash_matches=row['mesh_sha256']==old['mesh_thickness_sha256'],phase10_hash_matches=row['mesh_sha256']==fine['mesh_sha256'],ring_clamp_sequence_relative_error=relative)
        binding['within_owner_bar']=binding['phase9_hash_matches'] and binding['phase10_hash_matches'] and relative<=1e-9
        require(row['parent_mesh_binding']==binding,'Parent mesh/replay binding mismatch')
        for k in ['mesh_sha256','mesh_json_sha256']:require(len(row[k])==64 and all(c in '0123456789abcdef' for c in row[k]),'Malformed mesh hash')
        a=row['a4'];require(row['a4_complete']==(a is not None),'A4 completeness mismatch')
        if a:
            require(a['schema']=='jarambit.side_report.a4.v1' and a['subject']==row['name'] and a['genome']==row['genome'] and a['genome_sha256']==row['genome_sha256'],'A4 input binding mismatch')
            require(a['d1_sha256']=='0'*64,'Native D1 sentinel changed')
            require(a['mesh_max_area_mm2']==.25 and a['n_node']==old['metrics']['nodes'] and a['n_elem']==old['metrics']['tris'],'A4 mesh dimensions mismatch')
            require(a['certified_seq_ratio']==cert,'A4 certificate reference changed')
            require(a['partial_solver_edit'] is False and a['changes_seq_ratio'] is False and a['certified_model']=='run_b ring-hole clamp in all 5 DOFs','Certified/native solver model changed')
            require(a['eigensolver']=='jarambit_physics::side_a4::lanczos_k_orth','Native eigensolver changed')
            bore=a['bore_support'];grip=a['grip_foundation_stiffness']
            require(bore==dict(model='radial_pin_bearing_and_flange',k_radial_n_per_mm=50000.,k_tangential_n_per_mm=5000.,k_flange_n_per_mm=100000.,n_bore_nodes=bore['n_bore_nodes']) and bore['n_bore_nodes']>0,'Bore support model changed')
            require(grip==dict(model='winkler_grip_pad_foundation',k_foundation_n_per_mm3=200.,n_grip_nodes=grip['n_grip_nodes']) and grip['n_grip_nodes']>0,'Grip foundation model changed')
            for k in ['buckling_eigenvalue','buckling_critical_load_n','supported_seq_ratio','supported_blade_peak_mpa_per_n','supported_hand_peak_mpa_per_n']:
                require(type(a[k]) in [int,float] and math.isfinite(a[k]) and a[k]>0,'Invalid native A4 observation')
            require(math.isclose(a['buckling_critical_load_n'],260.*a['buckling_eigenvalue'],rel_tol=1e-14,abs_tol=0.),'Native critical-load identity mismatch')
            require(math.isclose(a['supported_seq_ratio'],a['supported_blade_peak_mpa_per_n']/max(a['supported_hand_peak_mpa_per_n'],1e-12),rel_tol=1e-14,abs_tol=0.),'Supported sequence identity mismatch')
            require(a['supported_vs_certified_seq_delta']==a['supported_seq_ratio']-cert,'Native sequence delta mismatch')
            require(a['status']==('pass' if a['buckling_eigenvalue']>0 and a['supported_seq_ratio']>=cert else 'fail'),'Native status mismatch')
        buckling,sequence=verdict(a['buckling_eigenvalue'],a['supported_seq_ratio'],cert) if a else (False,False)
        require(row['buckling_pass'] is buckling and row['support_sequence_pass'] is sequence and row['side_a4_pass'] is (buckling and sequence),'A4 gate verdict mismatch')
        margins=dict(buckling=a['buckling_eigenvalue']-1. if a else None,sequence=a['supported_seq_ratio']-cert if a else None)
        require(row['margins']==margins,'A4 margin mismatch')
        av=a or {}
        summary.append(line([row['name'],av.get('buckling_eigenvalue'),margins['buckling'],av.get('supported_seq_ratio'),cert,margins['sequence'],buckling,sequence,buckling and sequence,av.get('status')]))
        meshes.append(line([row['name'],av.get('n_node'),av.get('n_elem'),av.get('bore_support',{}).get('n_bore_nodes'),av.get('grip_foundation_stiffness',{}).get('n_grip_nodes'),binding['within_owner_bar'],row['mesh_sha256']]))
    counts={k:sum(v[k] is True for v in r['results']) for k in ['a4_complete','buckling_pass','support_sequence_pass','side_a4_pass']};counts['parent_mesh_binding']=sum(v['parent_mesh_binding']['within_owner_bar'] for v in r['results'])
    require(r['outcome_counts']==counts and r['a4_observations']==counts['a4_complete'],'A4 outcome count mismatch')
    actual_summary=[l for l in text.splitlines() if l.startswith('| ') and len(l.split('|'))==12 and l.split('|')[1].strip() in parents]
    actual_meshes=[l for l in text.splitlines() if l.startswith('| ') and len(l.split('|'))==9 and l.split('|')[1].strip() in parents]
    require(len(actual_summary)==36 and set(actual_summary)==set(summary),'Report A4 table mismatch')
    require(len(actual_meshes)==36 and set(actual_meshes)==set(meshes),'Report A4 mesh table mismatch')
    require('full qualification=false' in text.lower(),'Qualification limitation missing')

def raw_mesh_hash(mesh):
    data=b''.join(struct.pack('<dd',*xy) for xy in mesh['xy'])+b''.join(struct.pack('<QQQ',*t) for t in mesh['tris'])+b''.join(struct.pack('<d',t) for t in mesh['te'])
    return hashlib.sha256(data).hexdigest()
def audit(root=ROOT):
    root=Path(root);r=read(root/STUDY/'phase11_receipt.json');report=(root/STUDY/'PHASE11.md').read_text()
    phase10.audit(root);audit_document(root,r,report)
    refs=dict(r['sha256']);refs.update(r['source_provenance'])
    for row in r['results']:refs.update({row['raw']:row['raw_sha256'],row['mesh_json']:row['mesh_json_sha256']})
    checked=missing=raw_meshes=0
    for rel,want in refs.items():
        p=path(root,rel);require(len(want)==64 and all(c in '0123456789abcdef' for c in want),'Malformed artifact hash')
        if rel.startswith('target/') and not exists(p):missing+=1;continue
        require(digest(p)==want,f'Artifact SHA-256 mismatch: {rel}');checked+=1
    for row in r['results']:
        p=path(root,row['raw'])
        if exists(p):
            raw=read(p);require(raw['source_provenance']==r['source_provenance'],'Native source provenance mismatch')
            require(all(row[k]==v for k,v in raw.items() if k in row),'Published/native A4 mismatch')
        p=path(root,row['mesh_json'])
        if exists(p):
            mesh=read(p);require(raw_mesh_hash(mesh)==row['mesh_sha256'],'Raw mesh binary SHA mismatch')
            if row['a4']:require(len(mesh['xy'])==row['a4']['n_node'] and len(mesh['tris'])==len(mesh['te'])==row['a4']['n_elem'],'Raw mesh dimensions mismatch')
            raw_meshes+=1
    return dict(status='pass',cases=36,a4_observations=r['a4_observations'],outcome_counts=r['outcome_counts'],artifacts_checked=checked,optional_raw_unavailable=missing,raw_meshes_checked=raw_meshes,native_replay='not_run',all_gate_qualification=False)

if __name__=='__main__':print(json.dumps(audit(),sort_keys=True))
