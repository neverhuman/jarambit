#!/usr/bin/env python3
"""Portable read-only checks for the assigned native Phase14 study."""
import sys
sys.dont_write_bytecode=True
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT))
import csv,itertools,json,math
from fem_solid.gci import Level,MeshRef,Observable,compute_gci
from ops.ci.g16_06_rsv2_08 import audit_phase13 as previous
master=previous.master;STUDY=previous.STUDY;TEST=previous.TEST
require,digest,read,path,exists,line=previous.require,previous.digest,previous.read,previous.path,previous.exists,previous.line
DNS=[.3,.4,.5,.6];ANGLES=[18.,18.2,18.4,18.6];AREAS=[4.,1.,.25]
FIXED={42:6.20,46:4.4,47:148.,89:.192,117:1.,120:.25}
METRICS=dict(F_lat='Fem.lat_rob',F_ax='Fem.hm_tip_inplane_n: conservative envelope of chord-axial and transverse in-plane tip loads',SF_fat='Fem.fat_sf: native Goodman 180N cyclic lateral',tip_gci='F_lat: tip-loaded whole-plate lateral capacity outside native 4mm tip exclusion',torsional_gci='Fem.twist_joint_nmm: global tip-loaded chord torsion with native section correction')
THRESHOLDS=previous.THRESHOLDS
METADATA_FIELDS={**previous.METADATA_FIELDS,13:[['sha256',TEST]]+[['sha256',master.receipt_rel(n)] for n in range(9,13)]+
    [['parent_metadata_updates',str(n),i,'after'] for n,fields in previous.METADATA_FIELDS.items() for i in range(len(fields))]}
def metadata_after(root,keys,parents):
    if keys[0]=='parent_metadata_updates' and len(keys)==4:return master.get_field(parents[int(keys[1])],previous.METADATA_FIELDS[int(keys[1])][keys[2]])
    return previous.metadata_after(root,keys,parents)
def check_history(root,r):
    parents={n:read(path(root,master.receipt_rel(n))) for n in range(9,14)}
    require(set(r['parent_metadata_updates'])==set(map(str,range(9,14))),'Historical receipt coverage mismatch')
    for n,fields in METADATA_FIELDS.items():
        edits=r['parent_metadata_updates'][str(n)];require([e['keys'] for e in edits]==fields,'Unexpected historical metadata edits')
        original=json.loads(json.dumps(parents[n]))
        for e in edits:
            require(master.get_field(parents[n],e['keys'])==e['after']==metadata_after(root,e['keys'],parents),'Dependent historical hash mismatch')
            master.set_field(original,e['keys'],e['before'])
        require(master.document_hash(original)==r['parent_receipts_before_metadata_sha256'][str(n)],f'Phase{n} scientific receipt changed')

def seed_check(root,r):
    s=r['seed'];require(s['api']=='Genome::c4_seed(6.0)','Native seed API changed')
    for key in ['original','fixed']:
        p=path(root,s[key+'_genome']);require(exists(p) and digest(p)==s[key+'_sha256'],'Native seed bytes changed')
        data=list(csv.reader(p.read_text().splitlines()))
        require(len(data)==2 and len(data[1])==129 and [float(v) for v in data[1][:128]]==s[key+'_genes'] and data[1][128]==s['material'],'Seed CSV/material mismatch')
    original=s['original_genes'];expected=list(original)
    require(original[:40]==[0.]*40 and original[45]==6. and s['material']=='Ti-6Al-4V','Native C4 stock/material changed')
    for i,v in FIXED.items():expected[i]=v
    before=expected.copy()
    for i in [90,95,96,109,99,105,110,114,115,68,72,73,65,78,79]:expected[i]=0.
    expected[74]=min(expected[74],.6);expected[121]=max(expected[121],3.)
    require(s['before_p1_clamp_genes']==before and s['fixed_genes']==expected,'Unassigned native seed/P1 canonicalization change')
    prov=read(path(root,s['provenance']));require(prov=={k:s[k] for k in ['api','original_genes','before_p1_clamp_genes','fixed_genes','material','fixed_control_points']},'Native seed provenance mismatch')
    return expected

def flags(row):
    if not row['fem_complete']:return dict.fromkeys(['lateral','fatigue','sequence','tip_loaded_twist','native_utility_twist','human_max_twist'],False)
    m=[l['result']['metrics'] for l in row['levels']]
    return dict(lateral=all(v['F_lat']>=358. for v in m),fatigue=all(v['SF_fat']>=1. for v in m),sequence=all(v['seq_ratio']>=1.25 for v in m),
        tip_loaded_twist=all(v['twist_sf']>=1. for v in m),native_utility_twist=all(v['twist_body_sf']>=1. and v['twist_tipzone_sf']>=1. for v in m),human_max_twist=all(v['hm_twist_sf']>=1. for v in m))
def observable(key):
    metric,units,projection=('F_lat','N','plate_dkt') if key=='tip_gci' else ('twist_joint_nmm','Nmm','plate_dkt_with_native_section_correction')
    return dict(id='phase14_'+metric,units=units,region='whole_plate_outside_native_4mm_tip_exclusion',projection=projection,metric=metric)
def check_gci(row,key):
    spec=observable(key);levels=list(reversed(row['levels']))
    bound=[Level(MeshRef(f'{row["name"]}_area_{l["area_mm2"]:g}','rev1',l['result']['mesh_thickness_sha256']),math.sqrt(l['area_mm2']),Observable(**spec)) for l in levels]
    expected=compute_gci(*[l['result']['metrics'][spec['metric']] for l in levels],2.,1.25,levels=bound).to_dict();actual=row[key]
    require(set(actual)==set(expected),'GCI field coverage mismatch')
    for k,v in expected.items():
        if k in ['p','f_ext','gci_fine']:
            a=actual[k];require((a is None and v is None) or (a is not None and v is not None and math.isfinite(a) and math.isclose(a,v,rel_tol=1e-12,abs_tol=1e-14)),f'GCI {k} arithmetic mismatch')
        else:require(actual[k]==json.loads(json.dumps(v)),f'GCI {k} mismatch')
def check_case(root,row,seed):
    dn,ea=row['dn37_mm'],row['ea_deg'];require(dn in DNS and ea in ANGLES,'Off-grid candidate')
    require(row['name']==f'dn{dn:.2f}_ea{ea:.1f}','Candidate name/grid mismatch')
    expected=list(seed)
    for i,v in [(36,.65*dn),(37,dn),(38,.80*dn),(48,ea),(49,ea)]:expected[i]=v
    require(row['genes']==expected,'Assigned/unassigned gene mismatch')
    p=path(root,row['genome']);require(exists(p),'Missing tracked Phase14 candidate');require(digest(p)==row['genome_sha256'],'Candidate SHA-256 mismatch')
    data=list(csv.reader(p.read_text().splitlines()));base=list(csv.reader(path(root,STUDY+'/phase14_candidates/fixed_seed.csv').read_text().splitlines()))
    require(data[0]==base[0] and len(data[1])==129 and [float(v) for v in data[1][:128]]==expected and data[1][128]==base[1][128],'Candidate CSV/material mismatch')
    require(row['input_canonical'] and row['rs_v2'] and row['native_fem_unmodified'] and row['production_thickness_law'],'Noncanonical/custom production input')
    require(row['solver']=='faer' and row['cargo_build_jobs']==2 and row['nice']>=10 and row['cpu_affinity'].split(':')[1].strip()=='12-13','Case execution limits changed')
    require(row['geometry_legal']==(not row['geometry_rejects']) and row['aesthetic_pass']==(not row['aesthetic_failures']),'Native gate verdict mismatch')
    require(row['native_gate_s_law_pass']==(.05<=expected[89]<=.20 and expected[48]<=25. and expected[49]<=25.),'Native Gate S law predicate changed')
    require(row['all_gate_qualification'] is False,'Unsupported qualification claim')
    cp,basecp=row['control_points'],row['parent_control_points'];require(len(cp)==len(basecp)==40,'Native control point proof missing')
    for i,(a,b) in enumerate(zip(cp,basecp)):
        require(all(math.isfinite(v) for v in a+b),'Nonfinite native control point')
        if i in [36,37,38]:require(math.isclose(math.dist(a,b),{36:.65*dn,37:dn,38:.80*dn}[i],rel_tol=1e-12,abs_tol=5e-13),'Native CP displacement mismatch')
        else:require(a==b,'Unassigned physical CP changed')
    require([l['area_mm2'] for l in row['levels']]==AREAS,'Three-mesh grid changed')
    require(row['fem_complete']==all(l['result'] is not None for l in row['levels']),'FEM completeness mismatch')
    denominators=[]
    for l in row['levels']:
        v=l['result']
        if v is None:continue
        require(v['area_mm2']==l['area_mm2'] and v['unbiased'] is True,'Biased/mismatched observation')
        m=v['metrics'];require(all(type(x) in [int,float] and math.isfinite(x) and x>0 for x in m.values()),'Invalid native metric')
        require(m['native_hunter_sf']==m['F_lat']/260. and m['hm_tip_inplane_sf']==m['F_ax']/1000.,'Native lateral/axial metric identity mismatch')
        require(math.isclose(m['twist_sf'],m['twist_joint_nmm']/1820.,rel_tol=1e-14) and math.isclose(m['hm_twist_sf'],m['twist_joint_nmm']/16200.,rel_tol=1e-14),'Native torsional identity mismatch')
        denominators.append(m['F_lat']/m['SF_fat']);h=v['mesh_thickness_sha256'];require(len(h)==64 and all(c in '0123456789abcdef' for c in h),'Invalid mesh hash')
    if denominators:require(max(denominators)-min(denominators)<1e-10,'Native fatigue identity mismatch')
    if row['fem_complete']:
        for key in ['tip_gci','torsional_gci']:check_gci(row,key)
    require(row['observed_target_flags']==flags(row),'Unchanged target verdict mismatch')
    require(row['reported_targets_pass']==(row['geometry_legal'] and row['aesthetic_pass'] and all(flags(row).values())),'Target conjunction mismatch')
def summary_line(row):
    fine=row['levels'][-1]['result'];m={} if fine is None else fine['metrics']
    return line([row['name'],row['geometry_legal'],row['aesthetic_pass']]+[m.get(k) for k in ['F_lat','F_ax','SF_fat']]+[row.get(k,{}).get(v) for k in ['tip_gci','torsional_gci'] for v in ['fit_status','gci_fine']]+[row['reported_targets_pass']])
def mesh_lines(row):
    return [line([row['name'],level,l['area_mm2']]+[(l['result'] or {}).get('metrics',{}).get(k) for k in ['F_lat','F_ax','SF_fat','twist_joint_nmm','seq_ratio']]+[(l['result'] or {}).get('mesh_thickness_sha256')]) for level,l in zip(['coarse','medium','fine'],row['levels'])]
def gate_line(row):return line([row['name'],json.dumps(row['geometry_rejects']),json.dumps(row['aesthetic_failures'])])
def counts(rows):
    c={k:sum(v[k] is True for v in rows) for k in ['input_canonical','geometry_legal','aesthetic_pass','fem_complete','native_gate_s_law_pass','reported_targets_pass']}
    c.update({k:sum(v['observed_target_flags'][k] for v in rows) for k in ['lateral','fatigue','sequence','tip_loaded_twist','native_utility_twist','human_max_twist']})
    c['geometry_and_aesthetic_pass']=sum(v['geometry_legal'] and v['aesthetic_pass'] for v in rows)
    c.update({k+'_admitted':sum(v.get(k,{}).get('admitted') is True for v in rows) for k in ['tip_gci','torsional_gci']});return c

def audit_document(root,r,text):
    require(r['task']=='G16-06-RSV2-08-p14' and r['state']=='complete' and not r['errors'],'Incomplete Phase14 study')
    require(r['expected_cases']==r['completed_cases']==len(r['results'])==16 and r['mesh_observations']==48,'Phase14 observation count mismatch')
    require(r['grid']==dict(dn37_mm=DNS,ea_deg=ANGLES,dn36_factor=.65,dn38_factor=.80,fixed_genes={str(k):v for k,v in FIXED.items()}),'Assignment grid changed')
    require(r['areas_coarse_to_fine_mm2']==AREAS and r['refinement_ratio_r']==2. and r['gci_safety_factor']==1.25 and r['method_id']=='relative_gci.v2','Native GCI method/mesh changed')
    require(r['metric_definitions']==METRICS,'Requested native metric definitions changed')
    require(r['thresholds']==THRESHOLDS and r['observed_sequence_floor']==1.25,'Acceptance thresholds changed')
    require(r['cpus']==[12,13] and r['nice']>=10 and r['cargo_build_jobs']==2 and r['solver']=='faer' and r['max_concurrent_evaluations']==2,'Execution limits changed')
    require(r['production_sources_changed'] is False and r['all_gate_qualification'] is False and r['unverified']==['actual-export 3D Gate S','strict .0625mm2/certification .125mm2','A4 and remaining independent 3D qualification gates'],'Qualification/source claim changed')
    check_history(root,r);seed=seed_check(root,r)
    require(len({v['name'] for v in r['results']})==16 and {(v['dn37_mm'],v['ea_deg']) for v in r['results']}==set(itertools.product(DNS,ANGLES)),'Cartesian grid coverage mismatch')
    for row in r['results']:
        check_case(root,row,seed);require(row['parent_control_points']==r['seed']['fixed_control_points'],'Native seed CP binding mismatch')
    require(r['outcome_counts']==counts(r['results']),'Outcome count mismatch')
    for rel,want in r['parent_reports_sha256'].items():require(digest(path(root,rel))==want,'Historical report changed')
    require(set(r['parent_reports_sha256'])=={STUDY+f'/PHASE{n}.md' for n in range(9,14)},'Historical report coverage changed')
    names={v['name'] for v in r['results']};actual=[l for l in text.splitlines() if l.startswith('| ') and l.split('|')[1].strip() in names]
    expected=[summary_line(v) for v in r['results']]+[l for v in r['results'] for l in mesh_lines(v)]+[gate_line(v) for v in r['results']]
    require(actual==expected,'Report native table mismatch')
    require('full qualification=false' in text.lower() and 'not_established' in text and 'conservative envelope' in text,'Scope/native metric limitation missing')
def audit(root=ROOT):
    root=Path(root);r=read(root/STUDY/'phase14_receipt.json');text=(root/STUDY/'PHASE14.md').read_text()
    previous.audit(root);audit_document(root,r,text)
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
    return dict(status='pass',cases=16,mesh_observations=48,outcome_counts=r['outcome_counts'],artifacts_checked=checked,optional_raw_unavailable=missing,native_replay='not_run',all_gate_qualification=False)
if __name__=='__main__':print(json.dumps(audit(),sort_keys=True))
