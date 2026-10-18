#!/usr/bin/env python3
"""Portable Phase10 native-GCI evidence audit; never reruns a native solve."""
import sys
sys.dont_write_bytecode=True
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT))
import hashlib,json,math
from fem_solid.gci import Level,MeshRef,Observable,compute_gci
from ops.ci.g16_06_rsv2_08 import audit_phase9 as phase9
STUDY=phase9.STUDY
require,digest,read,path,exists=phase9.require,phase9.digest,phase9.read,phase9.path,phase9.exists
AREAS=[.25,1.,4.]

def scientific_hash(r):
    return hashlib.sha256(json.dumps({k:v for k,v in r.items() if k!='sha256'},sort_keys=True,separators=(',',':')).encode()).hexdigest()
def line(values):return '| '+' | '.join(f'{x:.12g}' if isinstance(x,float) else str(x) for x in values)+' |'
def check_estimate(doc,key,metric,observable):
    raw=doc['levels_fine_to_coarse'];levels=[]
    for l in raw:
        levels.append(Level(MeshRef(f'{doc["subject"]}_area_{l["max_area_mm2"]:g}','rev1',l['mesh_sha256']),l['h_mm'],Observable(**observable)))
    expected=compute_gci(*[l[metric] for l in raw],2.,1.25,levels=levels).to_dict();actual=doc[key]
    require(actual['levels']==json.loads(json.dumps(expected['levels'])),'GCI mesh/observable binding mismatch')
    for k in ['fit_status','reason','monotone','sequence_valid','observable_valid','method_id','asymptotic_validation','relative_error_available','admitted']:
        require(actual[k]==expected[k],f'GCI {k} mismatch')
    # Libm arithmetic comparison only; no gate or admission predicate gets a tolerance.
    for k in ['p','f_ext','gci_fine']:
        a,b=actual[k],expected[k]
        require((a is None and b is None) or (a is not None and b is not None and math.isfinite(a) and math.isclose(a,b,rel_tol=1e-12,abs_tol=1e-14)),f'GCI {k} arithmetic mismatch')

def audit_document(root,r,text):
    require(r['task']=='G16-06-RSV2-08-p10' and r['state']=='complete' and not r['errors'],'Incomplete Phase10 study')
    require(r['expected_cases']==r['completed_cases']==len(r['results'])==36 and r['mesh_observations']==108,'Phase10 observation count mismatch')
    require(r['areas_coarse_to_fine_mm2']==[4.,1.,.25] and r['areas_fine_to_coarse_mm2']==AREAS,'Three-level mesh areas changed')
    require((r['refinement_ratio_r'],r['gci_safety_factor'],r['method_id'],r['owner_replay_bar'])==(2.,1.25,'relative_gci.v2',1e-9),'GCI method/thresholds changed')
    require(r['cpus']==[12,13] and r['cargo_build_jobs']==2 and r['max_concurrent_evaluations']==2 and r['nice']>=10 and r['solver']=='faer','Execution limits changed')
    require(r['production_sources_changed'] is False and r['all_gate_qualification'] is False,'Source/qualification claim changed')
    p9=read(Path(root)/STUDY/'phase9_receipt.json');parents={p['name']:p for p in p9['results']}
    require(scientific_hash(p9)==r['phase9_scientific_sha256'],'Phase9 scientific data changed')
    meta=r['phase9_metadata_update'];rel='ops/ci/tests/test_portable_phase7.py'
    require(meta['path']==rel and meta['current_sha256']==p9['sha256'][rel]==digest(path(root,rel)),'Shared test metadata mismatch')
    original=json.loads(json.dumps(p9));original['sha256'][rel]=meta['original_sha256']
    require(hashlib.sha256((json.dumps(original,indent=2)+'\n').encode()).hexdigest()==r['phase9_parent_receipt_sha256'],'Phase9 receipt changed beyond shared-test digest')
    require(digest(Path(root)/STUDY/'PHASE9.md')==r['phase9_report_sha256'],'Phase9 report changed')
    require(len({v['name'] for v in r['results']})==36 and {v['name'] for v in r['results']}==set(parents),'Phase9 profile coverage mismatch')
    expected_summary=[];expected_mesh=[]
    for row in r['results']:
        parent=parents[row['name']]
        for k in ['name','family','genome','genome_sha256','genes']:require(row[k]==parent[k],'Phase9 candidate changed')
        require(exists(path(root,row['genome'])),'Missing tracked Phase10 genome')
        require(digest(path(root,row['genome']))==row['genome_sha256'],'Phase10 genome hash mismatch')
        require(row['input_canonical'] and row['rs_v2'] and row['native_fem_unmodified'],'Noncanonical/custom native input')
        require(row['production_api']=='jarambit_crescent::gci::run_plate_gci_3level','Native API changed')
        require(row['cpu_affinity'].split(':')[1].strip()=='12-13' and row['nice']>=10 and row['cargo_build_jobs']==2 and row['solver']=='faer','Case execution limits changed')
        require(row['all_gate_qualification'] is False,'Case falsely qualified')
        g=row['plate_gci'];require(g is not None,'Missing native GCI observation')
        require(g['schema']=='jarambit.plate_gci.v1' and g['subject']==row['name'] and g['genome']==row['genome'] and g['genome_sha256']==row['genome_sha256'],'GCI genome binding mismatch')
        require(g['primary_observable']=='seq_ratio' and g['refinement_ratio_r']==2.,'GCI primary observable/refinement changed')
        raw=g['levels_fine_to_coarse'];require([v['max_area_mm2'] for v in raw]==AREAS and [v['h_mm'] for v in raw]==[.5,1.,2.],'GCI mesh level order changed')
        for v in raw:
            require(all(type(v[k]) in [float,int] and math.isfinite(v[k]) and v[k]>0 for k in ['seq_ratio','sf_hunter','lat20_n','n_node','n_elem']),'Invalid native mesh observation')
            require(len(v['mesh_sha256'])==64 and all(c in '0123456789abcdef' for c in v['mesh_sha256']),'Invalid mesh content hash')
        check_estimate(g,'seq_ratio_gci','seq_ratio',dict(id='plate_seq_ratio',metric='seq_ratio',region='blade_vs_ring_neck',projection='plate_cst_dkt',units='ratio'))
        check_estimate(g,'sf_hunter_gci','sf_hunter',dict(id='plate_sf_hunter',metric='sf_hunter',region='blade_outside_4mm',projection='plate_cst_dkt',units='ratio'))
        e=g['seq_ratio_gci']
        require(row['plate_gci_valid_3level']==(e['sequence_valid'] and e['observable_valid']),'Native battery validity flag mismatch')
        require(row['seq_ratio_gci_admitted']==e['admitted'],'Admission summary mismatch')
        fine=raw[0];old=parent['levels'][0]['result'];relative=lambda a,b:abs(a-b)/max(abs(b),1e-300)
        binding=dict(mesh_sha256_matches=fine['mesh_sha256']==old['mesh_thickness_sha256'],sequence_relative_error=relative(fine['seq_ratio'],old['metrics']['sequence_ratio']),hunter_relative_error=relative(fine['sf_hunter'],old['metrics']['native_hunter_sf']))
        binding['within_owner_bar']=binding['mesh_sha256_matches'] and binding['sequence_relative_error']<=1e-9 and binding['hunter_relative_error']<=1e-9
        require(row['phase9_fine_binding']==binding,'Phase9 replay binding mismatch')
        expected_summary.append(line([row['name']]+[e[k] for k in ['sequence_valid','observable_valid','admitted','fit_status','reason','p','gci_fine']]+[binding['within_owner_bar']]))
        for v in raw:expected_mesh.append(line([row['name']]+[v[k] for k in ['max_area_mm2','h_mm','n_node','n_elem','seq_ratio','sf_hunter','mesh_sha256']]))
    counts={k:sum(v[k] is True for v in r['results']) for k in ['plate_gci_valid_3level','seq_ratio_gci_admitted']}
    counts.update(sequence_valid=sum(v['plate_gci']['seq_ratio_gci']['sequence_valid'] is True for v in r['results']),observable_valid=sum(v['plate_gci']['seq_ratio_gci']['observable_valid'] is True for v in r['results']),phase9_fine_binding=sum(v['phase9_fine_binding']['within_owner_bar'] for v in r['results']))
    require(r['outcome_counts']==counts,'Outcome count mismatch')
    actual_summary=[l for l in text.splitlines() if l.startswith('| ') and len(l.split('|'))==11 and l.split('|')[1].strip() in parents]
    actual_mesh=[l for l in text.splitlines() if l.startswith('| ') and len(l.split('|'))==10 and l.split('|')[1].strip() in parents]
    require(len(actual_summary)==36 and set(actual_summary)==set(expected_summary),'Report GCI table mismatch')
    require(len(actual_mesh)==108 and set(actual_mesh)==set(expected_mesh),'Report mesh table mismatch')
    require('full qualification=false' in text.lower() and 'not_established' in text,'Qualification/asymptotic limitation missing')

def audit(root=ROOT):
    root=Path(root);r=read(root/STUDY/'phase10_receipt.json');report=(root/STUDY/'PHASE10.md').read_text()
    phase9.audit(root);audit_document(root,r,report)
    refs=dict(r['sha256']);refs.update(r['source_provenance'])
    for row in r['results']:refs[row['raw']]=row['raw_sha256']
    checked=missing=0
    for rel,want in refs.items():
        p=path(root,rel)
        require(len(want)==64 and all(c in '0123456789abcdef' for c in want),'Malformed artifact hash')
        if rel.startswith('target/') and not exists(p):missing+=1;continue
        require(digest(p)==want,f'Artifact SHA-256 mismatch: {rel}');checked+=1
    for row in r['results']:
        p=path(root,row['raw'])
        if exists(p):
            raw=read(p);require(raw['source_provenance']==r['source_provenance'],'Native source provenance mismatch')
            require(all(row[k]==v for k,v in raw.items() if k in row),'Published/native raw mismatch')
    return dict(status='pass',cases=36,mesh_observations=108,outcome_counts=r['outcome_counts'],artifacts_checked=checked,optional_raw_unavailable=missing,native_replay='not_run',all_gate_qualification=False)

if __name__=='__main__':print(json.dumps(audit(),sort_keys=True))
