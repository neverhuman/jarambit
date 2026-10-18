"""Read-only receipt checks shared by the Phase7 and Phase8 command-line audits.

Missing ignored build evidence is reported, never described as recomputed evidence.
Present raw artifacts must match their recorded hashes. Acceptance floors are fixed.
"""
import csv
import hashlib
import itertools
import json
import math
from collections import defaultdict
from pathlib import Path,PurePosixPath

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
AREAS=[4.,1.,.25,.125,.0625]
LIMITS={'hm_spine_sf':1.,'lat_rob':358.,'fat_sf':1.,'sf_hunter':1.377}
ALLOWED={40,41,42,46,47,74,100,101,102,103}
BOUNDS={40:(-4.,4.),41:(-.4,10.),42:(4.5,7.),46:(2.6,4.5),47:(115.,150.),74:(0.,1.2),100:(.05,.5),101:(.5,.92),102:(3.,10.),103:(.45,.8)}

def require(condition,message):
    if not condition:raise ValueError(message)

def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def read(path):return json.loads(path.read_bytes())
def is_file(path):
    try:return path.is_file()
    except FileNotFoundError:return False
def table_value(value):return f'{value:.9f}' if isinstance(value,float) else str(value)
def passed(levels):return all(all(l['metrics'][k]>=floor for k,floor in LIMITS.items()) and l['metrics']['seq_ratio']>=(1.25 if l['area']==.0625 else 1.20) for l in levels)
def margin(row):return min(l['metrics'][k]/floor for l in row['levels'][2:] for k,floor in {**LIMITS,'seq_ratio':1.25 if l['area']==.0625 else 1.20}.items())

def safe_path(root,relative):
    p=PurePosixPath(relative)
    require(not p.is_absolute() and '..' not in p.parts,'Artifact path must stay inside checkout')
    return root/relative

def raw_path(root,relative):
    p=safe_path(root,relative)
    if is_file(p):return p
    # The original runner used capitals; also support the assignment's lower-case spelling.
    alt=safe_path(root,relative.replace('target/G16-06-RSV2-08/','target/g16-06-rsv2-08/',1))
    return alt if is_file(alt) else None

def check_raw(root,receipt):
    references=dict(receipt['sha256'])
    for row in receipt['results']+receipt.get('law_effect_controls',receipt.get('replay_controls',[])):
        for rel,want in row['sha256'].items():
            require(rel not in references or references[rel]==want,'Conflicting artifact digest')
            references[rel]=want
    for c in receipt.get('ring_controls',[]):
        references[c['genome']]=c['genome_sha256'];references[c['native_output']]=c['native_output_sha256']
    checked=missing=0
    for rel,want in references.items():
        require(len(want)==64 and all(c in '0123456789abcdef' for c in want),'Malformed SHA-256')
        if not rel.startswith('target/'):continue
        path=raw_path(root,rel)
        if path is None:missing+=1;continue
        require(digest(path)==want,f'SHA-256 mismatch: {rel}');checked+=1
    return dict(checked=checked,unavailable=missing,native_replay='not_run')

def check_rows(root,r,phase):
    require(r['state']=='complete' and not r['errors'],'Study incomplete or has execution errors')
    require(r['all_gate_qualification'] is False,'Full qualification must remain false')
    require(r['cpus']==[12,13] and r['nice']>=10 and r['cargo_build_jobs']==2 and r['solver']=='faer','Numerical execution limits changed')
    require(r['thresholds']==LIMITS and r['cert_sequence_floor']==1.20 and r['strict_sequence_floor']==1.25,'Acceptance thresholds changed')
    require(r['areas_mm2']==AREAS,'Five required areas changed')
    base=list(csv.reader((root/'experiments/G16-06-RSV2-08/phase5_sources/q7.csv').read_text().splitlines()))
    controls=r.get('law_effect_controls',r.get('replay_controls',[]))
    counts={k:0 for k in r['outcome_counts']};groups=defaultdict(list);observations=0
    names=[v['name'] for v in r['results']]
    require(len(names)==len(set(names)),'Duplicate result name')
    for row in r['results']+controls:
        name=row['name'];genes=row['genes'];levels=row['levels']
        require(len(genes)==128 and all(math.isfinite(v) for v in genes),'Invalid gene vector')
        require(genes[:40]==[float(x) for x in base[1][:40]] and genes[64]==42.,f'Parent CP/spine changed: {name}')
        require(all(lo<=genes[i]<=hi for i,(lo,hi) in BOUNDS.items()),f'Native varied-gene bounds: {name}')
        require(genes[89]==.175,'Native RS-v2 apex exception changed')
        require(all(genes[i]==float(base[1][i]) for i in range(128) if i not in ALLOWED),'Unassigned gene changed')
        require(genes[101]>=genes[100]+.2 and genes[46]<=genes[45]-.5,'Native coupled bounds changed')
        c=row['config'];require(c['d0']==.48 and c['d1']==.65 and c['point_shape']==3 and c['point_lateral']==2 and c['pocket_t']==c['window_belly']==0,'Q7 section configuration changed')
        require(row['input_was_canonical'] and row['genes']==row['observed_genes'] and not row['domain_projection_differences'],'Clamped input accepted')
        require(row['geometry_legal'] and not row['rejects'] and not row['aesthetic_failures'],f'Geometry not legal: {name}')
        relative=row['genome'];path=raw_path(root,relative) if relative.startswith('target/') else safe_path(root,relative)
        if path is not None:
            require(is_file(path),f'Missing tracked genome: {relative}')
            require(digest(path)==row['genome_sha256'],f'Genome SHA-256 mismatch: {name}')
            data=list(csv.reader(path.read_text().splitlines()))
            require(data[0]==base[0] and len(data[1])==129 and data[1][128]==base[1][128],'CSV/material identity changed')
            require([float(v) for v in data[1][:128]]==genes,'CSV genes do not match result')
            changed=[i for i,(a,b) in enumerate(zip(base[1],data[1])) if a!=b]
            require(changed==row['changed_indices'] and set(changed)<=ALLOWED,'Unexpected changed genes')
        require([v['area'] for v in levels]==AREAS and all(v['unbiased'] and v['metrics'] for v in levels),f'Missing mesh observation: {name}')
        for l in levels:
            require(all(isinstance(l['metrics'][k],(int,float)) and math.isfinite(l['metrics'][k]) for k in [*LIMITS,'seq_ratio']),'Nonfinite metric')
        require(row['fine_targets_pass']==passed(levels[2:]) and row['all_five_targets_pass']==passed(levels),'Numerical pass flag mismatch')
        for k in row['certification']:
            require(row['certification'][k]==min(v['metrics'][k] for v in levels[2:4]),'Certification minimum mismatch')
            require(row['strict'][k]==levels[4]['metrics'][k],'Strict metric mismatch')
            require(row['all_five_minimum'][k]==min(v['metrics'][k] for v in levels),'All-five minimum mismatch')
        denominator=[v['metrics']['lat_rob']/v['metrics']['fat_sf'] for v in levels]
        require(max(denominator)-min(denominator)<1e-10,'Fatigue identity mismatch')
        gci=row['gci'];gl=gci['levels_fine_to_coarse']
        require([l['max_area_mm2'] for l in gl]==[.25,1.,4.] and gci['refinement_ratio_r']==2.,'GCI levels changed')
        require(gci['genome_sha256']==row['genome_sha256'],'GCI genome binding mismatch')
        require(row['gci_admitted']==gci['seq_ratio_gci']['admitted'],'GCI admission flag mismatch')
        for i,l in enumerate(gl):
            m=next(v['metrics'] for v in levels if v['area']==l['max_area_mm2'])
            require(m['seq_ratio']==l['seq_ratio'] and m['sf_hunter']==l['sf_hunter'],'GCI observable mismatch')
            for observable in ['seq_ratio_gci','sf_hunter_gci']:
                require(gci[observable]['levels'][i]['mesh']['sha256']==l['mesh_sha256'],'GCI mesh binding mismatch')
        if row['export_pass'] is not None:
            s,a=row['solid'],row['a4']
            expected=bool(row['sharpness']['pass'] and row['point']['pass'] and row['bore_edge_radius_mm']>=1.5 and s['census']==[0,0,1] and s['volume_mm3']>0)
            require(row['export_pass']==expected,'Export pass flag mismatch')
            require(row['side_a4_pass']==(a['buckling_eigenvalue']>=1. and a['supported_seq_ratio']>=a['certified_seq_ratio']),'A4 pass flag mismatch')
            require(a['same_mesh_ring_clamp_seq_ratio']==levels[2]['metrics']['seq_ratio'] and a['certified_seq_ratio']==row['certification']['seq_ratio'],'A4 certified observation mismatch')
            require(a['genome_sha256']==row['genome_sha256'],'A4 genome binding mismatch')
        require(row['scoped_joint_pass']==bool(row['geometry_legal'] and row['all_five_targets_pass'] and row['gci_admitted'] and row['export_pass'] and row['side_a4_pass']),'All-five joint mismatch')
        require(row['all_gate_qualification'] is False,'Row falsely fully qualified')
        if row in r['results']:
            require(row['assignment_scoped_joint_pass']==bool(row['geometry_legal'] and row['fine_targets_pass'] and row['gci_admitted'] and row['export_pass'] and row['side_a4_pass']),'Assignment joint mismatch')
            for k in counts:counts[k]+=row.get(k) is True
            groups[tuple(genes[i] for i in [42,100,101,102,103])].append(row);observations+=5
    require(counts==r['outcome_counts'],'Outcome counts mismatch')
    require(observations==r['mesh_observations'],'Mesh observation count mismatch')
    require(len(r['results'])==r['nominal_cases']==r['canonical_cases']==r['completed_cases'],'Case count mismatch')
    require({v['name'] for v in r['results'] if v['export_pass'] is not None}=={v['name'] for v in r['selection']},'Export selection coverage mismatch')
    return groups

def check_report(r,text,phase):
    # Verify every published five-mesh numeric row; rounding here is display-only.
    expected=set()
    for row in r['results']:
        for l in row['levels']:
            m=l['metrics'];peak=l['diagnostic_peaks']['spine']['at']
            expected.add(f'| {row["name"]} | {l["area"]} | {m["hm_spine_sf"]:.9f} | {m["lat_rob"]:.9f} | {m["fat_sf"]:.9f} | {m["sf_hunter"]:.9f} | {m["seq_ratio"]:.9f} | {peak} |')
    numeric=[line for line in text.splitlines() if line.startswith('| ') and len(line.split('|'))==10 and line.split('|')[2].strip() in {str(a) for a in AREAS}]
    require(len(numeric)==len(expected)==r['mesh_observations'] and set(numeric)==expected,'Report mesh table mismatch')
    require('full qualification=false' in text.lower() or 'full rs-v2/manufacturing qualification remains false' in text.lower(),'Report full-qualification limitation missing')
