#!/usr/bin/env python3
"""Portable four-predicate margin audit; evidence is never replaced by a proxy."""
import sys
sys.dont_write_bytecode=True
from pathlib import Path,PurePosixPath
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
import ast,csv,hashlib,json,math
FLOORS=dict(strict_lat=260.,twist_tip_sf=1.,twist_tipzone_sf_strict=1.,solid_tip_sf_nodal_max=1.)
SLOTS=[8,10,11];AREAS=[.25,.125,.0625]
RECEIPT='experiments/G16-06-RSV2-08/phase15_gate_s_compliant_joint_audit.json'
STUDY='experiments/G16-06-RSV2-08'
def require(ok,message):
    if not ok:raise ValueError(message)
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def load(p):return json.loads(p.read_text())
def relative(root,rel):
    p=PurePosixPath(rel);require(not p.is_absolute() and '..' not in p.parts and str(p)==rel and '\\' not in rel,'Nonportable evidence path');return Path(root)/rel
def finite(v):return type(v) in [int,float] and math.isfinite(v)
def joint_margin(values):
    require(set(values)==set(FLOORS),'Four-gate metric coverage changed')
    gates={}
    for k,floor in FLOORS.items():
        v=values[k];require(v is None or finite(v),'Nonfinite/non-numeric gate metric')
        gates[k]=dict(value=v,floor=floor,margin=None if v is None else v/floor-1.,passed=None if v is None else v>=floor)
    known=[v['margin'] for v in gates.values() if v['margin'] is not None]
    missing=[k for k,v in gates.items() if v['value'] is None]
    passed=False if any(v['passed'] is False for v in gates.values()) else (None if missing else True)
    return dict(gates=gates,missing_metrics=missing,joint_margin=None if missing else min(known),observed_min_margin=min(known) if known else None,passed=passed,status='unverified' if passed is None else ('pass' if passed else 'fail'))
def derive(plate,solid):
    levels=plate['levels'];require([l['area_mm2'] for l in levels]==AREAS,'Certification/strict meshes changed')
    for level in levels:
        for k in ['lat_rob','fat_sf','twist_tip_sf','twist_tipzone_sf','seq_ratio','twist_body_sf']:
            require(finite(level[k]) and level[k]>0,'Invalid native observation')
    c={k:min(v[k] for v in levels[:2]) for k in ['lat_rob','fat_sf','twist_tip_sf','twist_tipzone_sf','seq_ratio','twist_body_sf']}
    observed=None
    if solid is not None and solid['solver']['converged'] is True:
        require(solid['T_Nmm']==1820. and {k:solid['solid'][k] for k in ['R_CUT','NL','NSUB','PATCH_mm']}==dict(R_CUT=30.,NL=4,NSUB=1,PATCH_mm=2.),'Solid load/refinement changed')
        require(finite(solid['solver']['relative_residual']) and 0.<=solid['solver']['relative_residual']<=1e-10,'Invalid solid residual')
        observed=solid['solid']['SF_solid_nodal_max']
    metrics=dict(strict_lat=levels[2]['lat_rob'],twist_tip_sf=c['twist_tip_sf'],twist_tipzone_sf_strict=levels[2]['twist_tipzone_sf'],solid_tip_sf_nodal_max=observed)
    return c,metrics,joint_margin(metrics)
def bounds(genes,baseline):
    return dict(bevel=all(18.<=genes[i]<=18.6 for i in [48,49]),apex=.185<=genes[89]<=.195,spine_shift=all(genes[i]-baseline[i]<=.60 for i in [36,37,38]),tip=4.<=genes[46]<=4.45,taper=142.<=genes[47]<=149.)
def table_line(row):
    vals=[row['slot'],row['name'],row['geometry_legal'],row['aesthetic_pass']]+[row['metrics'][k] for k in FLOORS]+[row['joint']['joint_margin'],row['joint']['status']]
    return '| '+' | '.join(f'{v:.12g}' if isinstance(v,float) else str(v) for v in vals)+' |'
def audit_document(root,r,text):
    require(r['task']=='G16-06-RSV2-08-p15' and r['state']=='complete','Incomplete Phase15 audit')
    require(r['floors']==FLOORS and r['slots']==SLOTS and r['areas_mm2']==AREAS,'Audit policy/thresholds changed')
    require(r['sample']==dict(parent='Genome::c4_seed(6.0)',rng_seed=1,round=1,batch_size=12),'Production sample definition changed')
    require(r['phase14_model_status']=='Referenced four-gate model absent from this lane; exact assignment predicates applied','Unsupported Phase14 model lineage')
    require(r['runtime']['cpus']==[12,13] and r['runtime']['nice']>=10 and r['runtime']['cargo_build_jobs']==2 and r['runtime']['max_workers']==2,'Execution limits changed')
    require(r['native_sources_changed'] is False and r['all_gate_qualification'] is False,'Unsupported source/qualification claim')
    require([row['slot'] for row in r['results']]==SLOTS and len({row['name'] for row in r['results']})==3,'Slot coverage mismatch')
    source=relative(root,STUDY+'/phase15_sources/auto_evolve.rs').read_text();extracted=relative(root,STUDY+'/phase15_sources/offspring_extracted.rs').read_text()
    for piece in [source[source.index('const Q7_CPS:'):source.index('\n];',source.index('const Q7_CPS:'))+3],source[source.index('pub fn q7_offspring_arm_label'):source.index('\nfn repo_root')].split('fn repo_root')[0],source[source.index('fn spawn_offspring_batch('):source.index('\n/// Stage 1:',source.index('fn spawn_offspring_batch('))]]:
        require(piece in extracted,'Production generator extraction changed')
    cp_source=source[source.index('const Q7_CPS:'):source.index('\n];',source.index('const Q7_CPS:'))+3]
    q7=ast.literal_eval(cp_source.split('=',1)[1].strip().removesuffix(';'))
    for row in r['results']:
        require(row['q7_baseline_cp']==q7,'Q7 baseline CP binding mismatch')
        p=relative(root,row['genome']);require(digest(p)==row['genome_sha256'],'Candidate hash mismatch')
        data=list(csv.reader(p.read_text().splitlines()));require(len(data)==2 and len(data[1])==129 and [float(v) for v in data[1][:128]]==row['genes'] and data[1][128]=='Ti-6Al-4V','Candidate genes/material mismatch')
        require(row['arm']=={8:'q7_c8_delayed_taper',10:'q7_c10_spine_widened',11:'q7_c11_spine_widened_p1'}[row['slot']],'Production arm label changed')
        require(row['parameter_bounds']==bounds(row['genes'],row['q7_baseline_cp']) and all(row['parameter_bounds'].values()),'Assigned Gate S neighborhood changed')
        require(row['dn36_38_including_jitter']==[row['genes'][i]-row['q7_baseline_cp'][i] for i in [36,37,38]],'Native spine shift mapping changed')
        plate=load(relative(root,row['plate']));solid=load(relative(root,row['solid']))
        require(plate['slot']==row['slot'] and plate['genes']==row['genes'] and plate['genome_sha256']==row['genome_sha256'],'Native candidate binding mismatch')
        require(plate['input_canonical'] and plate['rs_v2'] and plate['unbiased'] and plate['native_sources_unmodified'] and plate['solver']=='faer','Noncanonical/modified native observation')
        require(plate['cpus']==[12,13] and plate['nice']>=10 and plate['cargo_build_jobs']==2,'Native worker limits changed')
        require(row['geometry_legal']==(not plate['geometry_rejects']) and row['aesthetic_pass']==plate['aesthetic_pass']==(not plate['aesthetic_failures']),'Native geometry/aesthetic verdict changed')
        c,m,j=derive(plate,solid)
        require(row['certification']==c and row['metrics']==m,'Native metric mapping mismatch')
        require(row['joint']==j,'Four-gate margin/verdict mismatch')
        require(row['exported_gate_s']=='unverified' and row['independent_dolfinx']=='unverified' and row['all_gate_qualification'] is False,'Unsupported exported/independent qualification claim')
        require(row['solid_input']==solid['design'],'Solid input binding mismatch')
    expected_counts=dict(candidates=3,plate_observations=9,solid_observations=3,geometry=sum(v['geometry_legal'] for v in r['results']),aesthetics=sum(v['aesthetic_pass'] for v in r['results']),four_gate_pass=sum(v['joint']['passed'] is True for v in r['results']))
    require(r['outcome_counts']==expected_counts,'Outcome count mismatch')
    actual=[l for l in text.splitlines() if l.startswith('| ') and len(l.split('|'))>3 and l.split('|')[1].strip() in ['8','10','11']]
    require(actual==[table_line(row) for row in r['results']],'Report four-gate table mismatch')
    require('Full qualification=false' in text and 'Phase14 model' in text and 'rounded' in text and 'bounded sample' in text,'Audit limitations missing')
def audit(root=ROOT):
    root=Path(root);r=load(root/RECEIPT);audit_document(root,r,(root/STUDY/'PHASE15.md').read_text())
    refs={**r['sha256'],**r['source_provenance']};checked=missing=0
    for rel,want in refs.items():
        p=relative(root,rel)
        if rel.startswith('target/') and not p.is_file():missing+=1;continue
        require(digest(p)==want,'Artifact SHA-256 mismatch: '+rel);checked+=1
    return dict(status='pass',cases=3,observations=12,artifacts_checked=checked,optional_raw_unavailable=missing,outcome_counts=r['outcome_counts'],all_gate_qualification=False)
if __name__=='__main__':print(json.dumps(audit(),sort_keys=True))
