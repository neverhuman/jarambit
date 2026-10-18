#!/usr/bin/env python3
"""Read-only Phase7..11 master audit, saved-result synthesis and exact Pareto ranks."""
import sys
sys.dont_write_bytecode=True
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT))
sys.path.insert(0,str(ROOT/'experiments/G16-06-RSV2-08'))
import hashlib,json,math
import audit_phase7 as phase7
import audit_phase8 as phase8
from ops.ci.g16_06_rsv2_08 import audit_phase9 as phase9,audit_phase10 as phase10,audit_phase11 as phase11
STUDY=phase9.STUDY
require,digest,read,path,exists=phase9.require,phase9.digest,phase9.read,phase9.path,phase9.exists
line=phase10.line
TEST='ops/ci/tests/test_portable_phase7.py'
OBJECTIVES=('certified_seq_ratio','support_seq_margin','tip_bending_safety_factor')
TIP_DEFINITION=dict(source_phase=9,source='strict.native_hunter_sf',area_mm2=.0625,formula='strict.lat_rob_n / 260 N',
    scope='native tip-loaded lateral plate; native stress region, not an isolated annular or independent 3D safety factor')
PARETO_POLICY=dict(objectives=list(OBJECTIVES),directions=['maximize']*3,dominance='all >= and at least one >; exact unrounded finite values; no epsilon',
    layers='successively remove all nondominated candidates',within_front='descending objective tuple, then ascending candidate name',ties='retain every candidate; equal vectors do not dominate each other')
def receipt_rel(n):return f'experiments/{"G16-06-RSV2-08" if n<9 else "g16-06-rsv2-08"}/phase{n}_receipt.json'
def report_rel(n):return f'experiments/{"G16-06-RSV2-08" if n<9 else "g16-06-rsv2-08"}/PHASE{n}.md'
METADATA_FIELDS={9:[['sha256',TEST]],10:[['sha256',TEST],['sha256',receipt_rel(9)],['phase9_metadata_update','current_sha256']],
    11:[['sha256',TEST],['sha256',receipt_rel(9)],['sha256',receipt_rel(10)],['parent_metadata_updates','shared_test_sha256_after']]}
def get_field(doc,keys):
    for key in keys:doc=doc[key]
    return doc
def set_field(doc,keys,value):
    for key in keys[:-1]:doc=doc[key]
    doc[keys[-1]]=value
def document_hash(doc):return hashlib.sha256((json.dumps(doc,indent=2)+'\n').encode()).hexdigest()
def load_receipts(root):return {n:read(path(root,receipt_rel(n))) for n in range(7,12)}
def stable_audit_summary(result):
    return {k:result[k] for k in ['status','cases','mesh_observations','a4_observations','control_mesh_observations','replay_mesh_observations','outcome_counts','all_gate_qualification'] if k in result}
def chain_audits(root):
    results={}
    for n,module in [(7,phase7),(8,phase8),(9,phase9),(10,phase10),(11,phase11)]:
        result=module.audit(root);require(result['status']=='pass',f'Phase{n} audit failed');results[str(n)]=result
    return results

def dominates(a,b):return all(a[k]>=b[k] for k in OBJECTIVES) and any(a[k]>b[k] for k in OBJECTIVES)
def pareto_rank(rows):
    require(len({v['name'] for v in rows})==len(rows),'Duplicate Pareto candidate')
    require(all(type(v['objectives'][k]) in [int,float] and math.isfinite(v['objectives'][k]) for v in rows for k in OBJECTIVES),'Nonfinite Pareto objective')
    remaining=list(rows);ranking=[];front=0
    while remaining:
        front+=1
        members=[v for v in remaining if not any(dominates(other['objectives'],v['objectives']) for other in remaining)]
        members.sort(key=lambda v:tuple(-v['objectives'][k] for k in OBJECTIVES)+(v['name'],))
        for pos,v in enumerate(members,1):
            ranking.append(dict(name=v['name'],rank=len(ranking)+1,front=front,rank_in_front=pos,
                dominated_by=sorted(other['name'] for other in rows if dominates(other['objectives'],v['objectives']))))
        names={v['name'] for v in members};remaining=[v for v in remaining if v['name'] not in names]
    return ranking

def synthesize(receipts):
    p9,p10,p11=[receipts[n] for n in [9,10,11]];a3={v['name']:v for v in p10['results']};a4={v['name']:v for v in p11['results']}
    require(len(p9['results'])==len(a3)==len(a4)==36 and {v['name'] for v in p9['results']}==set(a3)==set(a4),'Cross-phase candidate coverage mismatch')
    rows=[]
    for parent in sorted(p9['results'],key=lambda v:v['name']):
        name=parent['name'];gci=a3[name];side=a4[name];g=gci['plate_gci'];a=side['a4']
        require(a is not None,'Missing cross-phase A4 observation')
        for k in ['family','genome','genome_sha256','genes']:require(parent[k]==gci[k]==side[k],f'Cross-phase identity mismatch: {name}')
        levels=parent['levels'];require([v['area_mm2'] for v in levels]==[.25,.125,.0625],'Cross-phase certificate levels changed')
        cert=min(v['result']['metrics']['sequence_ratio'] for v in levels[:2]);strict=levels[2]['result']['metrics']
        require(cert==parent['certification']['sequence_ratio']==side['seq_cert_input']==a['certified_seq_ratio'],'Cross-phase certified sequence mismatch')
        require(side['margins']['sequence']==a['supported_seq_ratio']-cert==a['supported_vs_certified_seq_delta'],'Cross-phase support margin mismatch')
        mesh=levels[0]['result']['mesh_thickness_sha256']
        require(mesh==g['levels_fine_to_coarse'][0]['mesh_sha256']==side['mesh_sha256'],'Cross-phase fine mesh mismatch')
        require(strict==parent['strict'] and strict['native_hunter_sf']==strict['lat_rob_n']/260.,'Strict bending safety-factor binding mismatch')
        e=g['seq_ratio_gci']
        row=dict(name=name,family=parent['family'],genome=parent['genome'],genome_sha256=parent['genome_sha256'],
            parameters=dict(taper_x0_mm=parent['genes'][47],t_tip_mm=parent['genes'][46],ea_in_deg=parent['genes'][48],ea_sp_deg=parent['genes'][49]),
            side_a2=dict(source_receipt=receipt_rel(9),certified_seq_ratio=cert,strict_seq_ratio=strict['sequence_ratio'],
                certified_native_hunter_sf=min(v['result']['metrics']['native_hunter_sf'] for v in levels[:2]),strict_native_hunter_sf=strict['native_hunter_sf'],
                all_three_min_native_hunter_sf=min(v['result']['metrics']['native_hunter_sf'] for v in levels),strict_lat_rob_n=strict['lat_rob_n'],
                certified_lateral_sf_358=parent['certification']['lateral_sf_358'],strict_lateral_sf_358=strict['lateral_sf_358'],
                certified_fatigue_sf=parent['certification']['fatigue_sf'],strict_fatigue_sf=strict['fatigue_sf']),
            side_a3=dict(source_receipt=receipt_rel(10),sequence_valid=e['sequence_valid'],observable_valid=e['observable_valid'],admitted=e['admitted'],
                fit_status=e['fit_status'],reason=e['reason'],p=e['p'],gci_fine=e['gci_fine'],areas_mm2=[v['max_area_mm2'] for v in g['levels_fine_to_coarse']],
                sequence_values=[v['seq_ratio'] for v in g['levels_fine_to_coarse']],mesh_sha256=[v['mesh_sha256'] for v in g['levels_fine_to_coarse']]),
            side_a4=dict(source_receipt=receipt_rel(11),buckling_eigenvalue=a['buckling_eigenvalue'],buckling_margin=side['margins']['buckling'],
                supported_seq_ratio=a['supported_seq_ratio'],certified_seq_ratio=a['certified_seq_ratio'],support_seq_margin=side['margins']['sequence'],
                buckling_pass=side['buckling_pass'],support_sequence_pass=side['support_sequence_pass'],side_a4_pass=side['side_a4_pass'],mesh_sha256=side['mesh_sha256']),
            objectives=dict(certified_seq_ratio=cert,support_seq_margin=side['margins']['sequence'],tip_bending_safety_factor=strict['native_hunter_sf']),
            fine_mesh_sha256=mesh,gate_s_pass=parent['gate_s_pass'],lateral_fatigue_pass=parent['lateral_fatigue_screen'],all_gate_qualification=False)
        require(all(v['all_gate_qualification'] is False for v in [parent,gci,side]),'Cross-phase qualification claim changed')
        rows.append(row)
    return rows
def outcome_counts(rows):
    return dict(candidates=len(rows),sequence_valid=sum(v['side_a3']['sequence_valid'] for v in rows),observable_valid=sum(v['side_a3']['observable_valid'] for v in rows),
        gci_admitted=sum(v['side_a3']['admitted'] for v in rows),a4_buckling_pass=sum(v['side_a4']['buckling_pass'] for v in rows),
        a4_sequence_pass=sum(v['side_a4']['support_sequence_pass'] for v in rows),a4_joint_pass=sum(v['side_a4']['side_a4_pass'] for v in rows),
        strict_tip_bending_sf_ge_1=sum(v['objectives']['tip_bending_safety_factor']>=1. for v in rows),gate_s_pass=sum(v['gate_s_pass'] for v in rows),
        lateral_fatigue_pass=sum(v['lateral_fatigue_pass'] for v in rows),all_gate_qualification=sum(v['all_gate_qualification'] for v in rows))

def summary_line(row,rank):
    a2,a3,a4=row['side_a2'],row['side_a3'],row['side_a4']
    return line([rank['rank'],rank['front'],row['name'],a2['certified_seq_ratio'],a4['support_seq_margin'],row['objectives']['tip_bending_safety_factor'],
        a3['sequence_valid'],a3['observable_valid'],a3['admitted'],a4['buckling_eigenvalue'],a4['side_a4_pass']])
def check_metadata_history(root,r,receipts):
    current_test=digest(path(root,TEST))
    for n,fields in METADATA_FIELDS.items():
        edits=r['parent_metadata_updates'][str(n)];require([e['keys'] for e in edits]==fields,'Unexpected historical metadata edits')
        original=json.loads(json.dumps(receipts[n]))
        for e in edits:
            keys=e['keys'];after=current_test if keys[-1] in [TEST,'current_sha256','shared_test_sha256_after'] else digest(path(root,keys[-1]))
            require(get_field(receipts[n],keys)==e['after']==after,'Dependent metadata hash mismatch')
            set_field(original,keys,e['before'])
        require(document_hash(original)==r['phase_receipts_before_metadata_sha256'][str(n)],f'Phase{n} scientific receipt changed')

def audit_document(root,r,text):
    require(r['task']=='G16-06-RSV2-08-p12' and r['state']=='complete','Incomplete Phase12 synthesis')
    require(r['read_only_master'] is True and r['native_solves_run']==0 and r['production_sources_changed'] is False and r['all_gate_qualification'] is False,'Read-only/source/qualification claim changed')
    require(r['tip_bending_definition']==TIP_DEFINITION and r['pareto_policy']==PARETO_POLICY,'Objective definition/Pareto policy changed')
    require(r['execution']['cpus']==[12,13] and r['execution']['nice']>=10 and r['execution']['cargo_build_jobs']==2,'Authoring execution limits changed')
    receipts=load_receipts(root);check_metadata_history(root,r,receipts)
    require(set(r['phase_receipts_sha256'])=={str(n) for n in range(7,12)},'Phase chain coverage changed')
    for n in range(7,12):
        require(digest(path(root,receipt_rel(n)))==r['phase_receipts_sha256'][str(n)],f'Phase{n} receipt SHA mismatch')
        require(digest(path(root,report_rel(n)))==r['phase_reports_sha256'][str(n)],f'Phase{n} report changed')
    expected=synthesize(receipts)
    require(r['results']==expected,'Cross-phase synthesis mismatch')
    ranking=pareto_rank(expected)
    require(r['pareto_ranking']==ranking,'Pareto ranking mismatch')
    require(r['frontier']==[v['name'] for v in ranking if v['front']==1],'Pareto frontier mismatch')
    require(r['outcome_counts']==outcome_counts(expected),'Synthesis outcome count mismatch')
    require(r['candidate_count']==36 and r['qualified_frontier']==[],'Candidate count/qualification changed')
    by_name={v['name']:v for v in expected};lines=[summary_line(by_name[v['name']],v) for v in ranking]
    actual=[l for l in text.splitlines() if l.startswith('| ') and len(l.split('|'))==13 and l.split('|')[3].strip() in by_name]
    require(len(actual)==36 and actual==lines,'Report synthesis/Pareto table mismatch')
    require('full qualification=false' in text.lower() and 'lat_rob' in text and '260' in text,'Scope/objective limitation missing')

def audit(root=ROOT):
    root=Path(root);r=read(root/STUDY/'phase12_receipt.json');text=(root/STUDY/'PHASE12.md').read_text()
    phases=chain_audits(root);audit_document(root,r,text)
    require(r['phase_audit_summaries']=={n:stable_audit_summary(v) for n,v in phases.items()},'Saved phase-audit summary mismatch')
    checked=missing=0
    for rel,want in r['sha256'].items():
        p=path(root,rel);require(len(want)==64 and all(c in '0123456789abcdef' for c in want),'Malformed artifact hash')
        if rel.startswith('target/') and not exists(p):missing+=1;continue
        require(digest(p)==want,f'Artifact SHA-256 mismatch: {rel}');checked+=1
    return dict(status='pass',phases={n:stable_audit_summary(v) for n,v in phases.items()},candidates=36,pareto_frontier_count=len(r['frontier']),
        pareto_fronts=max(v['front'] for v in r['pareto_ranking']),outcome_counts=r['outcome_counts'],artifacts_checked=checked,optional_raw_unavailable=missing,
        native_replay='not_run',all_gate_qualification=False)

if __name__=='__main__':print(json.dumps(audit(),sort_keys=True))
