#!/usr/bin/env python3
"""Phase-6 native-domain matrix, legal ring/window/FUL sweep and top-ten qualification."""
from concurrent.futures import ThreadPoolExecutor, as_completed
import csv
from datetime import datetime, timezone
import itertools
import json
import os
from pathlib import Path
import run_phase5 as native
from prepare_phase5 import HERE, ROOT, digest

OUT=ROOT/'target/G16-06-RSV2-08/phase6'
P5=OUT.parent/'phase5'
ALLOWED={13,40,41,42,46,47,74,100,101,102,103}
WINDOWS=[(.5,.85,3.,.65),(.35,.85,3.,.65),(.2,.70,4.25,.65),
         (.05,.525,3.,.65),(.05,.525,4.25,.65),(.05,.525,5.5,.65)]
ROWS=[];ERRORS=[];SELECTION=[];LAW_EFFECT=[]

def milestone(title,detail):
    stamp=datetime.now(timezone.utc).isoformat(timespec='seconds')
    for rel in ['.agent/CHANGELOG.md','.agent/handoffs/G16-06-RSV2-08.md']:
        with (ROOT/rel).open('a') as f:f.write(f'\n## {stamp} — Phase-6 {title}\n{detail}\n')

def publish(state):
    counts={k:sum(v.get(k) is True for v in ROWS) for k in
            ['geometry_legal','gci_admitted','fine_targets_pass','all_five_targets_pass','export_pass','side_a4_pass','scoped_joint_pass']}
    r=dict(task='G16-06-RSV2-08-p6',state=state,cpus=[12,13],nice=os.getpriority(os.PRIO_PROCESS,0),cargo_build_jobs=2,
           solver='faer',max_concurrent_evaluations=2,nominal_cases=len(MATRIX),domain_rejected_cases=sum(not v['domain_legal'] for v in MATRIX),
           canonical_cases=len(JOBS),completed_cases=len(ROWS),mesh_observations=sum(len(v['levels']) for v in ROWS),
           thresholds=native.LIMITS,cert_sequence_floor=1.20,strict_sequence_floor=1.25,areas_mm2=native.AREAS,
           all_four_ring_levels_preflight_legal=True,ring_controls=CONTROLS['selected'],windows=WINDOWS,
           assignment_discrepancies=['TAPER_X0 native minimum115; requested100/106/112 excluded',
               'T_TIP native minimum2.6; requested1.80/2.05/2.30 excluded',
               'CP19..21 pinned0; legal GRIP_DX/DROP and bounded CP13 grip-end offset used; CP06..08 byte-preserved',
               'Q7 law preserved; taper/fullers may be inactive; matched mesh/metric equivalence audited'],
           construction='unchanged complete Q7 CSV/control law; conservative interpretation while optional clarification pending',
           adapter_note='Phase-5 native scientific modules/main reused byte-for-byte; p6 native geometry/GCI/A4 observation adapter; compiled provenance retains Phase-5 source identity',
           outcome_counts=counts,results=ROWS,errors=ERRORS,selection=SELECTION,law_effect_controls=LAW_EFFECT,all_gate_qualification=False,
           unclosed=['independent solid/cross-solver qualification','human-max twist','inherited ring collar/flat-stock constraint'],
           sha256={str(p.relative_to(ROOT)):digest(p) for p in [HERE/'run_phase6.py',HERE/'phase6_adapter.rs',
               HERE/'phase6_geometry.py',HERE/'phase6_geometry_refine.py',OUT/'main',OUT/'native_adapter',OUT/'source_provenance.json',
               OUT/'geometry_controls.json',OUT/'nominal_matrix.json',HERE/'phase5_sources/q7.csv',HERE/'phase5_sources/q7_control_case.json']})
    (HERE/'phase6_receipt.json').write_text(json.dumps(r,indent=2)+'\n')
    lines=['# G16-06-RSV2-08 Phase 6','',f'Status: {state}; {len(ROWS)}/{len(JOBS)} canonical cases complete; nominal grid {len(MATRIX)}; excluded by native domain {r["domain_rejected_cases"]}; mesh observations {r["mesh_observations"]}. Counts: {counts}.','',
           'All four ring levels are proven geometry/aesthetics legal before FEM. Q7 CP06..08 and pinned CP19..21 remain byte-identical. Native GRIP_DX/DROP and a small bounded CP13 grip-end offset restore grip/holder legality; exact native holder dimensions and grip lengths are in geometry_controls.json. Coarse failed searches are retained.', '',
           'Native domain conflict: requested taper starts100/106/112 are below115; tip thicknesses1.80/2.05/2.30 are below2.6. The full nominal Cartesian request is inventoried, without accepting clamped inputs. Only115/2.6 plus all three fuller depths and six stated window layouts enter the canonical sweep. No native bound is changed.', '',
           'The complete inherited Q7 section law is preserved (D0=.48,D1=.65,POINT_SHAPE3/POINT_LATERAL2,faer). It replaces native blade taper/fullers with its stock-core law. Matched native mesh/thickness/metric comparisons test whether the requested genes affect this construction; no revised physical law is silently introduced. This limits interpretation of the requested taper/fullers study.', '',
           'All geometry-legal canonical cases receive five meshes4/1/.25/.125/.0625 and native three-level4/1/.25 GCI. Certification sequence floor1.20 and strict1.25 are unchanged; spine1/lateral358N/fatigue1/hunter1.377 are retained on every mesh. Fine-only and all-five results are recorded separately. Native geometry, admitted GCI, actual export and A4 are mandatory for joint passage. Full RS-v2/manufacturing qualification remains false.', '',
           '| Case | Legal | Cert spine | Strict spine | Cert lat N | Strict lat N | Cert seq | Strict seq | GCI | Fine targets | All-five targets | Export | A4 | Joint |',
           '| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |']
    for v in ROWS:
        c,t=v.get('certification',{}),v.get('strict',{})
        values=[v['name'],v['geometry_legal'],c.get('hm_spine_sf'),t.get('hm_spine_sf'),c.get('lat_rob'),t.get('lat_rob'),c.get('seq_ratio'),t.get('seq_ratio'),v['gci_admitted'],v.get('fine_targets_pass'),v.get('all_five_targets_pass'),v['export_pass'],v['side_a4_pass'],v['scoped_joint_pass']]
        lines.append('| '+' | '.join(f'{x:.9f}' if isinstance(x,float) else str(x) for x in values)+' |')
    lines+=['',f'Top-ten selection: {SELECTION}. Ranking uses numerical target margins, native GCI admission and exact-input tie breaks. Equivalent three-level geometry/thickness results share a selection slot so ten distinct structural configurations receive export/A4; every fuller-genome observation remains retained.', '',
            'Unselected export/A4 values remain null and cannot qualify. This finite study does not establish global infeasibility or an optimum. GCI admission does not establish asymptotic validation. Native source/material/load/mask/support constants and every acceptance threshold remain unchanged.', '',
            'Original invocation:','```sh','nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/run_phase6.py','```','']
    (HERE/'PHASE6.md').write_text('\n'.join(lines))

if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10
    assert os.environ['CARGO_BUILD_JOBS']=='2'
    CONTROLS=json.loads((OUT/'geometry_controls.json').read_text())
    assert CONTROLS['state']=='complete' and CONTROLS['all_four_ring_levels_legal']
    assert [v['ring_wall'] for v in CONTROLS['selected']]==[6.4,6.65,6.85,7.]
    assert all(v['legal'] for v in CONTROLS['selected'])
    prior=json.loads((HERE/'phase5_receipt.json').read_text())
    for p in [P5/'main',P5/'source_provenance.json',HERE/'phase5_sources/q7.csv',HERE/'phase5_sources/q7_control_case.json']:
        assert digest(p)==prior['sha256'][str(p.relative_to(ROOT))]
    assert not (OUT/'main').exists();(OUT/'main').write_bytes((P5/'main').read_bytes());(OUT/'main').chmod(0o755)
    (OUT/'source_provenance.json').write_bytes((P5/'source_provenance.json').read_bytes())
    base=list(csv.reader((HERE/'phase5_sources/q7.csv').open()))
    native.OUT=OUT;native.BASE=base;native.ALLOWED=ALLOWED
    inputs=HERE/'phase6_candidates';assert not inputs.exists();inputs.mkdir()
    MATRIX=[];JOBS=[];bounds=CONTROLS['bounds']
    for control,tx,tt,fd,wi in itertools.product(CONTROLS['selected'],[100.,106.,112.,115.],[1.80,2.05,2.30,2.60],[0.,.45,.9],range(len(WINDOWS))):
        name=f'r{control["ring_wall"]:.2f}_x{tx:.0f}_t{tt:.2f}_f{fd:.2f}_w{wi}'
        changes={13:control['cp13'],40:control['drop'],41:control['grip_dx'],42:control['ring_wall'],46:tt,47:tx,74:fd,**dict(zip([100,101,102,103],WINDOWS[wi]))}
        violations=[dict(index=i,name=base[0][i],requested=v,bounds=bounds[i]) for i,v in changes.items() if not bounds[i][0]<=v<=bounds[i][1]]
        row=dict(name=name,ring_control=control['name'],changes=changes,domain_legal=not violations,violations=violations)
        if not violations:
            data=[list(v) for v in base]
            for i,v in changes.items():
                if v!=float(data[1][i]):data[1][i]=str(v)
            assert data[1][6:9]==base[1][6:9] and data[1][19:22]==base[1][19:22]
            path=inputs/f'{name}.csv'
            with path.open('w') as f:csv.writer(f,lineterminator='\n').writerows(data)
            row.update(genome=str(path.relative_to(ROOT)),genome_sha256=digest(path));JOBS.append((name,path))
        MATRIX.append(row)
    (OUT/'nominal_matrix.json').write_text(json.dumps(MATRIX,indent=2)+'\n');publish('law_effect_control')
    controls_dir=OUT/'law_effect_controls';controls_dir.mkdir()
    for name,changes in [('law_baseline',{}),('law_native_variant',{46:3.5,47:130.,74:.9})]:
        data=[list(v) for v in base]
        for i,v in changes.items():data[1][i]=str(v)
        path=controls_dir/f'{name}.csv'
        with path.open('w') as f:csv.writer(f,lineterminator='\n').writerows(data)
        LAW_EFFECT.append(native.evaluate((name,path)));publish('law_effect_control')
    assert all(v['geometry_legal'] for v in LAW_EFFECT)
    assert [v['mesh_sha256'] for v in LAW_EFFECT[0]['gci']['levels_fine_to_coarse']]==[v['mesh_sha256'] for v in LAW_EFFECT[1]['gci']['levels_fine_to_coarse']]
    comparison=[]
    for a,b in zip(LAW_EFFECT[0]['levels'],LAW_EFFECT[1]['levels']):
        for k,want in a['metrics'].items():
            if isinstance(want,(float,int)) and isinstance(b['metrics'].get(k),(float,int)):
                comparison.append(abs(b['metrics'][k]-want)/max(abs(want),1e-300))
    assert max(comparison)<=1e-9, 'record changed construction; do not silently declare inactive genes'
    (OUT/'law_effect_comparison.json').write_text(json.dumps(dict(max_relative_scalar_error=max(comparison),owner_comparison_bound=1e-9,
         native_GCI_mesh_thickness_hashes_identical=True,all_five_native_metrics_match=True,
         control_genes=dict(t_tip=[2.6,3.5],taper_x0=[115.,130.],fuller_depth=[0.,.9]),
         interpretation='in-bound taper/fullers do not change the inherited Q7 FEM law for this matched control; not an active distal-taper qualification'),indent=2)+'\n')
    publish('five_mesh_sweep')
    milestone('milestone 3: all ring levels legal; canonical sweep started',f'- Preflight controls4/4 legal before FEM. Nominal grid={len(MATRIX)}, native-domain excluded={len(MATRIX)-len(JOBS)}, canonical remainder={len(JOBS)}. No clamping/bound changes; hand CP06..08 and pinned CPs preserved. Five meshes/native GCI on every accepted case; Q7 law unchanged. Ten distinct top structural results will receive actual export/A4.')
    with ThreadPoolExecutor(max_workers=2) as pool:
        futures={pool.submit(native.evaluate,j):j[0] for j in JOBS}
        for f in as_completed(futures):
            name=futures[f]
            try:
                row=f.result();ROWS.append(row)
                milestone(f'case {name}',f'- Geometry={row["geometry_legal"]}, GCI={row["gci_admitted"]}, fine={row.get("fine_targets_pass")}, all-five={row.get("all_five_targets_pass")}; cert={row.get("certification")}; strict={row.get("strict")}. Full qualification=false.')
            except Exception as e:ERRORS.append(dict(name=name,error=repr(e)))
            publish('five_mesh_sweep');print('case',name,'completed',len(ROWS),'errors',ERRORS,flush=True)
    assert len(ROWS)==len(JOBS) and not ERRORS,ERRORS
    def margin(v):
        return min(l['metrics'][k]/floor for l in v['levels'][2:] for k,floor in
                   {**native.LIMITS,'seq_ratio':1.25 if l['area']==.0625 else 1.20}.items())
    ranked=sorted([v for v in ROWS if v['geometry_legal']],key=lambda v:(-v['fine_targets_pass'],-margin(v),-v['gci_admitted'],v['name']))
    seen=set()
    for v in ranked:
        key=tuple(l['mesh_sha256'] for l in v['gci']['levels_fine_to_coarse'])
        if key in seen:continue
        seen.add(key);SELECTION.append(dict(name=v['name'],rank=len(SELECTION)+1,margin=margin(v),three_level_mesh_hashes=key))
        if len(SELECTION)==10:break
    assert len(SELECTION)==10
    (OUT/'export_selection.json').write_text(json.dumps(SELECTION,indent=2)+'\n');publish('export_a4')
    for selected in SELECTION:
        row=next(v for v in ROWS if v['name']==selected['name']);genome=ROOT/row['genome']
        ep,export=native.invoke('main','export',row['name'],genome);native.verify_provenance(export)
        assert export['genes']==row['genes'] and export['config']==row['config']
        m=export['measurement'];s=export['solid'];row.update(sharpness=m['sharpness'],point={k:v for k,v in m['point'].items() if k!='samples'},bore_edge_radius_mm=m['bore_edge_radius_mm'],solid=s,worst_edge_face=export['worst_edge_face'])
        row['export_pass']=bool(m['sharpness']['pass'] and m['point']['pass'] and m['bore_edge_radius_mm']>=1.5 and s['census']==[0,0,1] and s['volume_mm3']>0)
        ap,side=native.invoke('native_adapter','a4',row['name'],genome,row['certification']['seq_ratio']);native.verify_provenance(side)
        row['a4']=side['a4'];row['side_a4_pass']=side['G16-13']
        assert row['a4']['same_mesh_ring_clamp_seq_ratio']==row['levels'][2]['metrics']['seq_ratio']
        row['scoped_joint_pass']=bool(row['geometry_legal'] and row['all_five_targets_pass'] and row['gci_admitted'] and row['export_pass'] and row['side_a4_pass'])
        row['coverage']='five_meshes_native_gci_actual_export_a4'
        for p in [ep,ap,ap.with_suffix('.mesh.json')]:row['sha256'][str(p.relative_to(ROOT))]=digest(p)
        publish('export_a4');milestone(f'export/A4 {row["name"]}',f'- Export={row["export_pass"]}; A4={row["side_a4_pass"]}; joint={row["scoped_joint_pass"]}. Full qualification=false; no threshold changes.');print('export_a4',row['name'],flush=True)
    publish('complete');milestone('finite study evaluated',f'- Canonical cases={len(ROWS)}, requested five-mesh observations={sum(len(v["levels"]) for v in ROWS)}, selected exports/A4=10. Joint passes={[v["name"] for v in ROWS if v["scoped_joint_pass"]]}; final independent audit follows. Full qualification=false.')
