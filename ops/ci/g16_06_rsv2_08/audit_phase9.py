#!/usr/bin/env python3
"""Read-only Phase9 verifier: tracked evidence required; ignored raw files optional."""
import sys
sys.dont_write_bytecode=True
import csv,hashlib,itertools,json,math
from pathlib import Path,PurePosixPath
ROOT=Path(__file__).resolve().parents[3]
STUDY='experiments/g16-06-rsv2-08'
AREAS=[.25,.125,.0625]

def require(value,message):
    if not value:raise ValueError(message)
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def read(p):return json.loads(p.read_bytes())
def exists(p):
    try:return p.is_file()
    except FileNotFoundError:return False
def path(root,rel):
    p=PurePosixPath(rel);require(not p.is_absolute() and '..' not in p.parts,'Unsafe relative path')
    return root/rel

def check_case(root,row,parent):
    name=row['name'];genes=row['genes'];source=path(root,row['genome'])
    require(exists(source),f'Missing tracked candidate: {name}')
    require(digest(source)==row['genome_sha256'],f'Candidate SHA-256 mismatch: {name}')
    data=list(csv.reader(source.read_text().splitlines()));require(len(data[1])==129,'CSV width mismatch')
    require([float(v) for v in data[1][:128]]==genes and data[1][128]==parent['material'],'CSV/material mismatch')
    require(len(genes)==128 and all(genes[i]==v for i,v in enumerate(parent['genes']) if i not in [46,47,48,49]),'Unassigned parent gene changed')
    require(row['input_canonical'] and row['rs_v2'],'Noncanonical/native RS-v2 input')
    require(row['production_thickness_law'] and row['native_fem_unmodified'],'Production law/FEM replaced')
    require(row['solver']=='faer' and row['cargo_build_jobs']==2 and row['nice']>=10 and row['cpu_affinity'].split(':')[1].strip()=='12-13','Numerical execution limits changed')
    require(row['all_gate_qualification'] is False,'Unverified full qualification')
    require(len(row['sections'])==44 and [s['s_mm'] for s in row['sections']]==[4.+.25*k for k in range(44)],'Annular station grid changed')
    for section in row['sections']:
        require(section['radial_annulus_mm']==[4.,15.],'Radial band changed')
        es=section['estimates'];require([e['step_max_mm'] for e in es]==[.02,.01,.005],'Quadrature refinement missing')
        for e in es:
            require(all(isinstance(e[k],(float,int)) and math.isfinite(e[k]) and e[k]>0 for k in ['area_mm2','width_mm','I_out_of_plane_mm4','I_in_plane_mm4','Z_out_of_plane_mm3','Z_in_plane_mm3','max_thickness_mm','min_thickness_mm']),'Invalid section integral')
            require(e['samples']>0 and e['min_thickness_mm']<=e['max_thickness_mm'],'Invalid thickness sample range')
            require(math.isclose(e['Z_out_of_plane_mm3'],2.*e['I_out_of_plane_mm4']/e['max_thickness_mm'],rel_tol=1e-14,abs_tol=0.),'Section modulus identity mismatch')
    exact=row['exact_law_point_samples']
    require([v['s_mm'] for v in exact]==[.25,1.,2.] and [v['limit_mm'] for v in exact]==[.30,.60,1.],'Point sample/threshold grid changed')
    for v in exact:require(v['margin_mm']==v['limit_mm']-v['thickness_mm'],'Direct point margin mismatch')
    gate=row['gate_s'];m=gate['measurement'];sharp,point=m['sharpness'],m['point']
    require((gate['apex_limit_mm'],gate['half_angle_limit_deg'],gate['point_limits_mm'],gate['point_taper_min_mm'])==(.20,25.,[.30,.60,1.],5.),'Native Gate S bounds changed')
    if sharp:
        require(sharp['pass']==(sharp['complete'] and sharp['max_apex_mm']<=.20 and sharp['max_half_angle_deg']<=25.),'Native sharpness flag mismatch')
    if point:
        ts=[point[k] for k in ['t_0p25_mm','t_1p00_mm','t_2p00_mm']]
        require(point['monotonic']==all(b[1]>=a[1] for a,b in zip(point['samples'],point['samples'][1:])),'Point monotonicity mismatch')
        require(point['pass']==(all(v<=limit for v,limit in zip(ts,[.30,.60,1.])) and point['monotonic'] and point['taper_lower_bound_mm']>=5.),'Point gate flag mismatch')
        require(row['gate_s_margins']['point_mm']==[limit-v for v,limit in zip(ts,[.30,.60,1.])],'Native point margin mismatch')
        require(row['gate_s_margins']['point_taper_mm']==point['taper_lower_bound_mm']-5.,'Point taper margin mismatch')
    if sharp:
        require(row['gate_s_margins']['apex_mm']==.20-sharp['max_apex_mm'] and row['gate_s_margins']['half_angle_deg']==25.-sharp['max_half_angle_deg'],'Sharpness margin mismatch')
    require(row['gate_s_pass']==bool(sharp and point and sharp['pass'] and point['pass']),'Gate S aggregate mismatch')
    levels=row['levels'];require([l['area_mm2'] for l in levels]==AREAS,'FEM refinement grid changed')
    require(row['fem_complete']==all(l['result'] is not None for l in levels),'FEM completeness mismatch')
    if row['fem_complete']:
        denominators=[]
        for level in levels:
            result=level['result'];v=result['metrics']
            require(result['unbiased'] and result['area_mm2']==level['area_mm2'],'Biased/mismatched FEM observation')
            require(all(math.isfinite(v[k]) and v[k]>0 for k in ['lat_rob_n','lateral_sf_358','native_hiker_sf','native_hunter_sf','fatigue_sf']),'Invalid FEM metric')
            require(v['lateral_sf_358']==v['lat_rob_n']/358.,'Lateral safety factor denominator changed')
            require(math.isclose(v['native_hiker_sf'],v['lat_rob_n']/180.,rel_tol=1e-14) and math.isclose(v['native_hunter_sf'],v['lat_rob_n']/260.,rel_tol=1e-14),'Native lateral floors changed')
            denominators.append(v['lat_rob_n']/v['fatigue_sf'])
        require(max(denominators)-min(denominators)<1e-10,'Fatigue identity mismatch')
        for k in row['certification']:require(row['certification'][k]==min(l['result']['metrics'][k] for l in levels[:2]),'Certification minimum mismatch')
        require(row['strict']==levels[2]['result']['metrics'],'Strict observation mismatch')
    expected=row['fem_complete'] and all(l['result']['metrics']['lateral_sf_358']>=1 and l['result']['metrics']['fatigue_sf']>=1 for l in levels)
    require(row['lateral_fatigue_screen']==expected,'Lateral/fatigue screen flag mismatch')

def audit_document(root,r,text):
    require(r['task']=='G16-06-RSV2-08-p9' and r['state']=='complete' and not r['errors'],'Incomplete Phase9 study')
    require(r['expected_cases']==r['completed_cases']==len(r['results'])==36 and r['mesh_observations']==108,'Phase9 observation count mismatch')
    require(r['areas_mm2']==AREAS,'Mesh areas changed')
    require(r['thresholds']==dict(lateral_n=358.,fatigue_sf=1.,sequence_cert=1.20,sequence_strict=1.25,apex_mm=.20,half_angle_deg=25.,point_mm=[.30,.60,1.],point_taper_min_mm=5.),'Acceptance thresholds changed')
    require(r['all_gate_qualification'] is False and r['production_sources_changed'] is False,'Source/qualification claim changed')
    require(r['cpus']==[12,13] and r['cargo_build_jobs']==2 and r['max_concurrent_evaluations']==2 and r['nice']>=10 and r['solver']=='faer','Study execution limits changed')
    expected=set(itertools.product(['placement_05','local_01'],[136.,142.,148.],[3.6,4.,4.4],[20.,21.8]))
    require({(v['family'],v['genes'][47],v['genes'][46],v['genes'][48]) for v in r['results']}==expected,'Cartesian grid mismatch')
    require(all(v['genes'][48]==v['genes'][49] for v in r['results']),'Edge angles differ')
    require(len({v['name'] for v in r['results']})==36,'Duplicate grid case')
    parents={v['family']:v for v in r['parents']};require(set(parents)=={'placement_05','local_01'},'Family parents changed')
    for p in parents.values():
        source=path(root,p['genome']);require(digest(source)==p['sha256'],'Parent hash mismatch')
        data=list(csv.reader(source.read_text().splitlines()))[1]
        require([float(v) for v in data[:128]]==p['genes'] and data[128]==p['material'],'Parent genes/material mismatch')
    require(len(r['early_taper_controls'])==2 and {v['family'] for v in r['early_taper_controls']}==set(parents),'Early control coverage mismatch')
    for v in r['early_taper_controls']:require((v['genes'][47],v['genes'][46],v['genes'][48],v['genes'][49])==(116.,2.62,18.,18.),'Early control genes changed')
    for v in r['results']+r['early_taper_controls']:check_case(root,v,parents[v['family']])
    require(r['outcome_counts']=={k:sum(v[k] is True for v in r['results']) for k in ['input_canonical','aesthetic_pass','fem_complete','gate_s_pass','lateral_fatigue_screen']},'Outcome count mismatch')
    # Every published fine cross-section is bound to its unrounded tracked observation.
    expected_lines=set()
    for row in r['results']+r['early_taper_controls']:
        for s in row['sections']:
            e=s['estimates'][-1]
            expected_lines.add('| '+' | '.join([row['name'],str(s['s_mm'])]+[f'{e[k]:.9f}' for k in ['area_mm2','Z_out_of_plane_mm3','Z_in_plane_mm3','min_thickness_mm','max_thickness_mm']])+' |')
    actual=[l for l in text.splitlines() if l.startswith('| ') and len(l.split('|'))==9 and l.split('|')[2].strip() in {str(4.+.25*k) for k in range(44)}]
    require(len(actual)==len(expected_lines)==1672 and set(actual)==expected_lines,'Report section table mismatch')
    def line(values):return '| '+' | '.join(f'{x:.9f}' if isinstance(x,float) else str(x) for x in values)+' |'
    for row in r['results']+r['early_taper_controls']:
        g=row['gate_s_margins'];p=row['gate_s']['measurement']['point']
        def triple(vs):return ', '.join(f'{x:.9f}' for x in vs) if vs is not None else 'unavailable'
        require(line([row['name'],g['apex_mm'],g['half_angle_deg'],triple(g['point_mm']),g['point_taper_mm'],p['monotonic'] if p else None,row['gate_s_pass'],
            triple([v['thickness_mm'] for v in row['exact_law_point_samples']]),triple([v['margin_mm'] for v in row['exact_law_point_samples']])]) in text,'Report Gate S table mismatch')
        for level in row['levels']:
            m=level['result']['metrics'] if level['result'] else {}
            require(line([row['name'],level['area_mm2']]+[m.get(k) for k in ['lat_rob_n','lateral_sf_358','native_hiker_sf','native_hunter_sf','fatigue_sf','sequence_ratio']]) in text,'Report FEM table mismatch')
        control=next(v for v in r['early_taper_controls'] if v['family']==row['family'])
        for k in ['area_mm2','Z_out_of_plane_mm3','Z_in_plane_mm3']:
            change=max(abs(s['estimates'][-1][k]-s['estimates'][-2][k])/abs(s['estimates'][-1][k]) for s in row['sections'])
            ratios=[s['estimates'][-1][k]/b['estimates'][-1][k] for s,b in zip(row['sections'],control['sections'])]
            require(r['section_convergence'][row['name']][k]==change and r['matched_control_ratios'][row['name']][k]==[min(ratios),max(ratios)],'Section convergence/control comparison mismatch')
    require('full qualification=false' in text.lower(),'Qualification limit missing')

def recorded_source_drift(root,here,r):
    """Tracked files changed after the study: {path: current digest}, each verified.

    The receipt stays byte-identical. A drifted file is accepted only at the digest the record
    names, and only while its study-time bytes are kept as a history copy matching the receipt."""
    p=here/'source_drift.v1.json'
    if not exists(p):return {}
    d=read(p)
    require(d.get('schema')=='jarambit.phase9_source_drift.v1' and d.get('study_receipt_sha256')==digest(here/'phase9_receipt.json'),'Source drift record does not bind this receipt')
    out={}
    for f in d['files']:
        rel=f['path'];want=r['source_provenance'].get(rel) or r['sha256'].get(rel)
        require(not rel.startswith('target/') and want==f['study_sha256'],f'Source drift record differs from receipt: {rel}')
        require(digest(path(root,f['history_copy']))==f['study_sha256'],f'Source history copy mismatch: {rel}')
        out[rel]=f['current_sha256']
    return out

def audit(root=ROOT):
    root=Path(root);here=root/STUDY;r=read(here/'phase9_receipt.json');text=(here/'PHASE9.md').read_text();audit_document(root,r,text)
    checked=missing=0
    refs=dict(r['sha256']);refs.update(r['source_provenance'])
    for row in r['results']+r['early_taper_controls']:refs[row['raw']]=row['raw_sha256']
    drift=recorded_source_drift(root,here,r);drifted=[]
    for rel,want in refs.items():
        require(len(want)==64 and all(c in '0123456789abcdef' for c in want),'Malformed digest')
        p=path(root,rel)
        if rel.startswith('target/') and not exists(p):missing+=1;continue
        got=digest(p)
        if got!=want and got==drift.get(rel):drifted.append(rel);checked+=1;continue
        require(got==want,f'Artifact SHA-256 mismatch: {rel}');checked+=1
    for row in r['results']+r['early_taper_controls']:
        p=path(root,row['raw'])
        if exists(p):
            raw=read(p)
            require(raw['task']==r['task'] and raw['source_provenance']==r['source_provenance'],'Raw production provenance mismatch')
            require(all(raw[k]==v for k,v in row.items() if k in raw),'Published observation/raw mismatch')
    return dict(status='pass',cases=36,mesh_observations=108,control_mesh_observations=6,section_stations=1672,
        outcome_counts=r['outcome_counts'],artifacts_checked=checked,optional_raw_unavailable=missing,native_replay='not_run',all_gate_qualification=False,
        recorded_source_drift=sorted(drifted))

if __name__=='__main__':print(json.dumps(audit(),sort_keys=True))
