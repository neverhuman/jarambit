#!/usr/bin/env python3
"""Portable read-only verification of the Phase13 native coordinate/FEM grid."""
import sys
sys.dont_write_bytecode=True
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT))
import csv,itertools,json,math
from ops.ci.g16_06_rsv2_08 import audit_phase12 as master
STUDY=master.STUDY
require,digest,read,path,exists=master.require,master.digest,master.read,master.path,master.exists
TEST=master.TEST;line=master.line
PARENTS=['local_01_x148_t4.4_ea21.8','local_01_x142_t4.4_ea21.8'];OFFSETS=[0.,.3,.6,.9];EDGES=[.180,.190,.198];AREAS=[.25,.125,.0625]
THRESHOLDS=dict(lateral_n=358.,fat_sf=1.,seq_cert=1.20,seq_strict=1.25,twist_tip_sf=1.,twist_body_sf=1.,twist_tipzone_sf_strict=1.,hm_twist_sf=1.)
METADATA_FIELDS={**master.METADATA_FIELDS,12:[['sha256',TEST]]+[['sha256',master.receipt_rel(n)] for n in [9,10,11]]+
    [['phase_receipts_sha256',str(n)] for n in [9,10,11]]+
    [['parent_metadata_updates',str(n),i,'after'] for n,fields in master.METADATA_FIELDS.items() for i in range(len(fields))]}
def metadata_after(root,keys,parents):
    if keys[0]=='phase_receipts_sha256':return digest(path(root,master.receipt_rel(int(keys[1]))))
    if keys[0]=='parent_metadata_updates' and len(keys)==4:
        n=int(keys[1]);return master.get_field(parents[n],master.METADATA_FIELDS[n][keys[2]])
    return digest(path(root,TEST)) if keys[-1] in [TEST,'current_sha256','shared_test_sha256_after'] else digest(path(root,keys[-1]))
def check_history(root,r):
    parents={n:read(path(root,master.receipt_rel(n))) for n in [9,10,11,12]}
    for n,fields in METADATA_FIELDS.items():
        edits=r['parent_metadata_updates'][str(n)];require([e['keys'] for e in edits]==fields,'Unexpected historical metadata edits')
        original=json.loads(json.dumps(parents[n]))
        for e in edits:
            require(master.get_field(parents[n],e['keys'])==e['after']==metadata_after(root,e['keys'],parents),'Dependent historical hash mismatch')
            master.set_field(original,e['keys'],e['before'])
        require(master.document_hash(original)==r['parent_receipts_before_metadata_sha256'][str(n)],f'Phase{n} scientific receipt changed')
    return parents
def derive(row):
    if not row['fem_complete']:return None,None,None,None,False
    metrics=[v['result']['metrics'] for v in row['levels']]
    c={k:min(v[k] for v in metrics[:2]) for k in metrics[0]};s=metrics[2]
    summary=dict(lat_rob_cert=c['lat_rob_n'],strict_lat_rob=s['lat_rob_n'],fat_sf=c['fat_sf'],fat_sf_strict=s['fat_sf'],seq_ratio_cert=c['seq_ratio'],seq_ratio_strict=s['seq_ratio'],
        twist_tip_sf=c['twist_tip_sf'],twist_tip_sf_strict=s['twist_tip_sf'],twist_body_sf=c['twist_body_sf'],twist_tipzone_sf_strict=s['twist_tipzone_sf'],hm_twist_sf=c['hm_twist_sf'],hm_twist_sf_strict=s['hm_twist_sf'])
    flags=dict(lateral=c['lat_rob_n']>=358. and s['lat_rob_n']>=358.,fatigue=c['fat_sf']>=1. and s['fat_sf']>=1.,sequence=c['seq_ratio']>=1.20 and s['seq_ratio']>=1.25,
        tip_loaded_twist=c['twist_tip_sf']>=1. and s['twist_tip_sf']>=1.,native_utility_twist=c['twist_body_sf']>=1. and s['twist_tipzone_sf']>=1.,human_max_twist=c['hm_twist_sf']>=1. and s['hm_twist_sf']>=1.)
    return c,s,summary,flags,row['geometry_legal'] and row['aesthetic_pass'] and all(flags.values())

def check_case(root,row,parent):
    require(row['parent']==parent['name'] and row['parent_genome']==parent['genome'] and row['parent_genome_sha256']==parent['genome_sha256'],'Assigned parent identity mismatch')
    offset,edge=row['cp_offset_mm'],row['edge_th_mm'];require(offset in OFFSETS and edge in EDGES,'Off-grid coordinate')
    expected=list(parent['genes'])
    for i in [36,37,38]:expected[i]+=offset
    expected[89]=edge
    require(row['genes']==expected,'Assigned/unassigned gene mismatch')
    require(all(-2.5<=row['genes'][i]<=2.5 for i in [36,37,38]) and .05<=row['genes'][89]<=.20,'Native CP/RS-v2 apex bounds changed')
    source=path(root,row['genome']);require(exists(source),'Missing tracked Phase13 candidate')
    require(digest(source)==row['genome_sha256'],'Candidate SHA-256 mismatch')
    data=list(csv.reader(source.read_text().splitlines()));base=list(csv.reader(path(root,parent['genome']).read_text().splitlines()))
    require(data[0]==base[0] and len(data[1])==129 and [float(v) for v in data[1][:128]]==row['genes'] and data[1][128]==parent['material'],'CSV/header/material mismatch')
    require(row['input_canonical'] and row['rs_v2'] and row['production_thickness_law'] and row['native_fem_unmodified'],'Noncanonical/custom production input')
    require(row['solver']=='faer' and row['nice']>=10 and row['cargo_build_jobs']==2 and row['cpu_affinity'].split(':')[1].strip()=='12-13','Case execution limits changed')
    require(row['all_gate_qualification'] is False,'Unsupported full qualification claim')
    require(len(row['control_points'])==len(row['parent_control_points'])==40,'Native CP proof missing')
    for i,(a,b) in enumerate(zip(row['control_points'],row['parent_control_points'])):
        require(all(math.isfinite(v) for v in a+b),'Nonfinite control point')
        if i in [36,37,38]:require(math.isclose(math.dist(a,b),offset,rel_tol=1e-12,abs_tol=5e-13),'Native outward CP displacement mismatch')
        else:require(a==b,'Unassigned physical CP changed')
    require(row['geometry_legal']==(not row['geometry_rejects']),'Geometry verdict mismatch')
    require(row['aesthetic_pass']==(not row['aesthetic_failures']),'Aesthetic verdict mismatch')
    levels=row['levels'];require([v['area_mm2'] for v in levels]==AREAS,'Three-mesh grid changed')
    require(row['fem_complete']==all(v['result'] is not None for v in levels),'FEM completeness mismatch')
    denominators=[]
    for level in levels:
        result=level['result']
        if result is None:continue
        require(result['unbiased'] is True and result['area_mm2']==level['area_mm2'],'Biased/mismatched observation')
        m=result['metrics'];require(all(type(v) in [float,int] and math.isfinite(v) and v>0 for v in m.values()),'Invalid native metric')
        require(m['lateral_sf_358']==m['lat_rob_n']/358. and m['native_hunter_sf']==m['lat_rob_n']/260.,'Lateral SF denominator changed')
        require(math.isclose(m['twist_tip_sf'],m['twist_joint_nmm']/1820.,rel_tol=1e-14,abs_tol=0.) and math.isclose(m['hm_twist_sf'],m['twist_joint_nmm']/16200.,rel_tol=1e-14,abs_tol=0.),'Native torsion/human-max identity mismatch')
        denominators.append(m['lat_rob_n']/m['fat_sf'])
        require(len(result['mesh_thickness_sha256'])==64 and all(c in '0123456789abcdef' for c in result['mesh_thickness_sha256']),'Malformed native mesh hash')
    if denominators:require(max(denominators)-min(denominators)<1e-10,'Native fatigue identity mismatch')
    c,s,summary,flags,passed=derive(row)
    if row['fem_complete']:
        require(row['certification']==c and row['strict']==s,'Certification/strict aggregation mismatch')
        require(row['summary']==summary,'Requested metric mapping mismatch')
        require(row['target_flags']==flags,'Unchanged target verdict mismatch')
    require(row['reported_targets_pass'] is passed,'Reported target conjunction mismatch')

def audit_document(root,r,text):
    require(r['task']=='G16-06-RSV2-08-p13' and r['state']=='complete' and not r['errors'],'Incomplete Phase13 study')
    require(r['expected_cases']==r['completed_cases']==len(r['results'])==24 and r['mesh_observations']==72,'Phase13 observation count mismatch')
    require(r['areas_mm2']==AREAS and r['thresholds']==THRESHOLDS,'Mesh areas/acceptance thresholds changed')
    require(r['grid']==dict(parents=PARENTS,cp_offsets_mm=OFFSETS,edge_th_mm=EDGES,cp_indices=[36,37,38],edge_index=89),'Assignment grid changed')
    require(r['cp_offset_rule']=='add the same requested offset to each parent CP36/37/38 gene; positive is native outward normal','Outward offset interpretation changed')
    require(r['cpus']==[12,13] and r['nice']>=10 and r['cargo_build_jobs']==2 and r['solver']=='faer' and r['max_concurrent_evaluations']==2,'Numerical execution limits changed')
    require(r['production_sources_changed'] is False and r['all_gate_qualification'] is False and len(r['unverified'])==4,'Qualification/source claim changed')
    historical=check_history(root,r);p9={v['name']:v for v in historical[9]['results']};parents={v['name']:v for v in r['parents']}
    require(len(r['parents'])==2 and set(parents)==set(PARENTS),'Parent coverage mismatch')
    for name,p in parents.items():
        old=p9[name]
        require(all(p[k]==old[k] for k in ['name','genome','genome_sha256','genes']),'Source parent changed')
        require(digest(path(root,p['genome']))==p['genome_sha256'],'Parent SHA-256 mismatch')
        base=list(csv.reader(path(root,p['genome']).read_text().splitlines()))
        require(p['material']==base[1][128] and p['genes']==[float(v) for v in base[1][:128]],'Parent material/genes changed')
    require(len({v['name'] for v in r['results']})==24 and {(v['parent'],v['cp_offset_mm'],v['edge_th_mm']) for v in r['results']}==set(itertools.product(PARENTS,OFFSETS,EDGES)),'Cartesian grid coverage mismatch')
    summary_lines=[];mesh_lines=[]
    for row in r['results']:
        check_case(root,row,parents[row['parent']]);s=row.get('summary',{})
        summary_lines.append(line([row['name']]+[s.get(k) for k in ['lat_rob_cert','strict_lat_rob','fat_sf','fat_sf_strict','seq_ratio_cert','seq_ratio_strict','twist_tip_sf','twist_tipzone_sf_strict','hm_twist_sf','hm_twist_sf_strict']]+[row['reported_targets_pass']]))
        for l in row['levels']:
            result=l['result'] or {};m=result.get('metrics',{})
            mesh_lines.append(line([row['name'],l['area_mm2']]+[m.get(k) for k in ['lat_rob_n','fat_sf','seq_ratio','twist_tip_sf','twist_tipzone_sf','hm_twist_sf']]+[result.get('mesh_thickness_sha256')]))
    counts={k:sum(v.get(k) is True for v in r['results']) for k in ['input_canonical','geometry_legal','aesthetic_pass','fem_complete','reported_targets_pass']}
    counts.update({k:sum(v.get('target_flags',{}).get(k) is True for v in r['results']) for k in ['lateral','fatigue','sequence','tip_loaded_twist','native_utility_twist','human_max_twist']})
    require(r['outcome_counts']==counts,'Outcome count mismatch')
    for rel,want in r['parent_reports_sha256'].items():require(digest(path(root,rel))==want,'Prior scientific report changed')
    require(set(r['parent_reports_sha256'])=={STUDY+f'/PHASE{n}.md' for n in [9,10,11,12]},'Prior report coverage changed')
    names={v['name'] for v in r['results']}
    actual_summary=[l for l in text.splitlines() if l.startswith('| ') and len(l.split('|'))==14 and l.split('|')[1].strip() in names]
    actual_mesh=[l for l in text.splitlines() if l.startswith('| ') and len(l.split('|'))==11 and l.split('|')[1].strip() in names]
    require(len(actual_summary)==24 and actual_summary==summary_lines,'Report certification table mismatch')
    require(len(actual_mesh)==72 and actual_mesh==mesh_lines,'Report native mesh table mismatch')
    require('full qualification=false' in text.lower() and 'Fem.twist_sf' in text,'Scope/native metric definition missing')

def audit(root=ROOT):
    root=Path(root);r=read(root/STUDY/'phase13_receipt.json');text=(root/STUDY/'PHASE13.md').read_text()
    master.audit(root);audit_document(root,r,text)
    refs=dict(r['sha256']);refs.update(r['source_provenance'])
    for row in r['results']:refs[row['raw']]=row['raw_sha256']
    checked=missing=0
    for rel,want in refs.items():
        p=path(root,rel);require(len(want)==64 and all(c in '0123456789abcdef' for c in want),'Malformed artifact digest')
        if rel.startswith('target/') and not exists(p):missing+=1;continue
        require(digest(p)==want,f'Artifact SHA-256 mismatch: {rel}');checked+=1
    for row in r['results']:
        p=path(root,row['raw'])
        if exists(p):
            raw=read(p);require(raw['source_provenance']==r['source_provenance'],'Native source provenance mismatch')
            require(all(row[k]==v for k,v in raw.items() if k in row),'Published/native measurement mismatch')
    return dict(status='pass',cases=24,mesh_observations=72,outcome_counts=r['outcome_counts'],artifacts_checked=checked,optional_raw_unavailable=missing,native_replay='not_run',all_gate_qualification=False)

if __name__=='__main__':print(json.dumps(audit(),sort_keys=True))
