#!/usr/bin/env python3
"""Deterministic production-law 36-case sweep and two wave-parameter controls."""
from concurrent.futures import ThreadPoolExecutor,as_completed
from datetime import datetime,timezone
import csv,hashlib,itertools,json,os,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
OUT=ROOT/'target/g16-06-rsv2-08/phase9'
AREAS=[.25,.125,.0625]
ROWS=[];CONTROLS=[];ERRORS=[]
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def milestone(title,detail):
    for rel in ['.agent/CHANGELOG.md','.agent/handoffs/G16-06-RSV2-08.md']:
        with (ROOT/rel).open('a') as f:f.write(f'\n## {datetime.now(timezone.utc).isoformat(timespec="seconds")} — Phase-9 {title}\n{detail}\n')
def environment():
    env={k:v for k,v in os.environ.items() if not k.startswith(('JARAMBIT_','STUDY_'))}
    env.update(JARAMBIT_SOLVER='faer',CARGO_BUILD_JOBS='2',CARGO_INCREMENTAL='0',CARGO_PROFILE_DEV_DEBUG='line-tables-only',RAYON_NUM_THREADS='2')
    return env

def evaluate(job):
    name,family,path,kind=job;output=OUT/f'{name}.json';assert not output.exists()
    with output.with_suffix('.log').open('w') as log:subprocess.run([str(OUT/'measure'),str(path),str(output)],env=environment(),cwd=ROOT,stdout=log,stderr=log,check=True)
    d=json.loads(output.read_text());assert 'seconds' in d and d['genome_sha256']==digest(path)
    assert d['source_provenance']==json.loads((OUT/'source_provenance.json').read_text())
    for rel,want in d['source_provenance'].items():assert digest(ROOT/rel)==want,rel
    row=dict(name=name,family=family,kind=kind,genome=str(path.relative_to(ROOT)),genome_sha256=digest(path),
        genes=d['genes'],input_canonical=d['input_canonical'],rs_v2=d['rs_v2'],geometry_rejects=d['geometry_rejects'],aesthetic_pass=d['aesthetic_pass'],aesthetic_failures=d['aesthetic_failures'],
        tip=d['tip'],axis=d['axis'],section_normal=d['section_normal'],sections=d['sections'],exact_law_point_samples=d['exact_law_point_samples'],gate_s=d['gate_s'],levels=d['levels'],
        certificate_identity=d['certificate_identity'],production_thickness_law=d['production_thickness_law'],native_fem_unmodified=d['native_fem_unmodified'],solver=d['solver'],cpu_affinity=d['cpu_affinity'],nice=d['nice'],cargo_build_jobs=d['cargo_build_jobs'],
        raw=str(output.relative_to(ROOT)),raw_sha256=digest(output),all_gate_qualification=False)
    assert [l['area_mm2'] for l in row['levels']]==AREAS
    row['fem_complete']=all(l['result'] is not None for l in row['levels'])
    sharp=d['gate_s']['measurement']['sharpness'];point=d['gate_s']['measurement']['point']
    row['gate_s_pass']=bool(sharp and point and sharp['pass'] and point['pass'])
    row['gate_s_margins']=dict(apex_mm=.20-sharp['max_apex_mm'] if sharp else None,half_angle_deg=25.-sharp['max_half_angle_deg'] if sharp else None,
        point_mm=[limit-point[key] for limit,key in zip([.30,.60,1.],['t_0p25_mm','t_1p00_mm','t_2p00_mm'])] if point else None,
        point_taper_mm=point['taper_lower_bound_mm']-5. if point else None)
    if row['fem_complete']:
        keys=row['levels'][0]['result']['metrics']
        row['certification']={k:min(l['result']['metrics'][k] for l in row['levels'][:2]) for k in keys}
        row['strict']=row['levels'][2]['result']['metrics']
        row['lateral_fatigue_screen']=all(l['result']['metrics']['lateral_sf_358']>=1. and l['result']['metrics']['fatigue_sf']>=1. for l in row['levels'])
    else:row['lateral_fatigue_screen']=False
    return row

def publish(state):
    counts={k:sum(v[k] is True for v in ROWS) for k in ['input_canonical','aesthetic_pass','fem_complete','gate_s_pass','lateral_fatigue_screen']}
    r=dict(task='G16-06-RSV2-08-p9',state=state,cpus=[12,13],nice=os.getpriority(os.PRIO_PROCESS,0),cargo_build_jobs=2,solver='faer',max_concurrent_evaluations=2,
      grid=dict(families=['placement_05','local_01'],taper_x0_mm=[136.,142.,148.],t_tip_mm=[3.6,4.,4.4],ea_deg=[20.,21.8]),
      expected_cases=36,completed_cases=len(ROWS),mesh_observations=sum(len(v['levels']) for v in ROWS),areas_mm2=AREAS,
      section_method=dict(stations_mm=[4.+.25*k for k in range(44)],radial_interval_mm=[4.,15.],radial_upper_exclusive=True,
        orientation='normal to native Gate S tip bisector',quadrature_steps_mm=[.02,.01,.005],
        law='unmodified production crates/crescent/src/export.rs::thickness_at',
        interpretation='exact production-function queries; numerical midpoint integration split at polygon/native raster boundaries; convergence estimates retained, not analytical exactness'),
      thresholds=dict(lateral_n=358.,fatigue_sf=1.,sequence_cert=1.20,sequence_strict=1.25,apex_mm=.20,half_angle_deg=25.,point_mm=[.30,.60,1.],point_taper_min_mm=5.),
      parents=PARENTS,results=sorted(ROWS,key=lambda v:v['name']),early_taper_controls=sorted(CONTROLS,key=lambda v:v['name']),errors=ERRORS,outcome_counts=counts,
      all_gate_qualification=False,production_sources_changed=False,
      limitations=['finite diagnostic grid, not an optimum/infeasibility proof','Gate S/lateral/fatigue do not establish full RS-v2 qualification','unrelated geometry/human-max/independent3D gates remain unwaived','parent family genomes are tracked Phase3 inputs; controls use task wave parameters, not a claimed replay of unsupplied w318 winners'],
      source_provenance=json.loads((OUT/'source_provenance.json').read_text()),
      sha256={str(p.relative_to(ROOT)):digest(p) for p in [HERE/'phase9.rs',HERE/'run_phase9.py',HERE/'build_phase9.sh',OUT/'measure',OUT/'source_provenance.json',OUT/'ASSIGNMENT.md']})
    (HERE/'phase9_receipt.json').write_text(json.dumps(r,indent=2)+'\n')
    lines=['# G16-06-RSV2-08 Phase 9','',f'Status: {state}; {len(ROWS)}/36 requested cases; {r["mesh_observations"]} requested mesh observations. Counts: {counts}.','',
      'The two complete tracked Phase3 placement_05/local_01 genomes are varied only at TAPER_X0, T_TIP, EA_IN and EA_SP. This is the production crescent library, exporter, phenotype and FEM, without Q7/custom section hooks. Both early-taper controls use116/2.62/18 from the assignment; an unsupplied w318 genome is not claimed replayed. Every requested case is measured, including geometry, sharpness, point or FEM failures; no clamped input is accepted.', '',
      'Sections use the native Gate S inward tip bisector and its in-plane normal, with exact polygon/hole intersections and radial clipping to4<=r_tip<15mm. At44 axial stations4..14.75mm, midpoint integration evaluates production export::thickness_at directly at maximum steps.02/.01/.005mm, split at polygon/native raster cell boundaries. Areas and section properties are numerical integrals of exact production queries, not nominal b*t or raster-base substitutes. All resolutions are retained. I_out=integral(t³/12 dv), Z_out=2I_out/t_max; I_in=integral((v-v_centroid)²t dv), Z_in=I_in/max edge distance. No quadrature tolerance is an acceptance-gate tolerance.', '',
      'Gate S uses unchanged native actual-export certification: apex<=.20mm, half-angle<=25deg, point t(.25/1/2)<=.30/.60/1mm, monotonic and taper>=5mm. Direct production-law point queries are reported separately from native triangle-intersection measurements. Lateral safety factors include native hunter/hiker factors and explicit lat_rob/358; fatigue is the native Goodman SF. Certification=min(.25,.125), strict=.0625. Sequence1.20/strict1.25 and every other prior gate remain unwaived. Full qualification=false.', '',
      '| Case | Cert lateral N | Strict lateral N | Cert fatigue SF | Strict fatigue SF | Gate S | Lateral/fatigue | Min area mm² | Min Z_out mm³ |',
      '| --- | --- | --- | --- | --- | --- | --- | --- | --- |']
    for v in sorted(ROWS,key=lambda v:v['name']):
        c,s=v.get('certification',{}),v.get('strict',{})
        vals=[v['name'],c.get('lat_rob_n'),s.get('lat_rob_n'),c.get('fatigue_sf'),s.get('fatigue_sf'),v['gate_s_pass'],v['lateral_fatigue_screen'],min(t['estimates'][-1]['area_mm2'] for t in v['sections']),min(t['estimates'][-1]['Z_out_of_plane_mm3'] for t in v['sections'])]
        lines.append('| '+' | '.join(f'{x:.9f}' if isinstance(x,float) else str(x) for x in vals)+' |')
    lines+=['',f'Early controls completed: {[v["name"] for v in CONTROLS]}. Raw outputs and failed attempts remain under target/g16-06-rsv2-08/phase9/. Native solver=faer, CPUs12,13, nice -n10 prefix (effective19 inherited), Cargo jobs2. No native source, gate, mask, support, load or threshold changed.','']
    (HERE/'PHASE9.md').write_text('\n'.join(lines))

if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10 and os.environ['CARGO_BUILD_JOBS']=='2'
    inputs=HERE/'phase9_candidates';inputs.mkdir();PARENTS=[];JOBS=[];controls=[]
    for family in ['placement_05','local_01']:
        source=ROOT/f'experiments/G16-06-RSV2-08/phase3_candidates/{family}.csv';base=list(csv.reader(source.open()))
        PARENTS.append(dict(family=family,genome=str(source.relative_to(ROOT)),sha256=digest(source),genes=[float(v) for v in base[1][:128]],material=base[1][128]))
        for tx,tt,ea in itertools.product([136.,142.,148.],[3.6,4.,4.4],[20.,21.8]):
            name=f'{family}_x{tx:.0f}_t{tt:.1f}_ea{ea:.1f}';data=[list(v) for v in base]
            for i,v in [(46,tt),(47,tx),(48,ea),(49,ea)]:data[1][i]=str(v)
            path=inputs/f'{name}.csv'
            with path.open('w') as f:csv.writer(f,lineterminator='\n').writerows(data)
            JOBS.append((name,family,path,'grid'))
        name=f'{family}_early116_t2.62_ea18';data=[list(v) for v in base]
        for i,v in [(46,2.62),(47,116.),(48,18.),(49,18.)]:data[1][i]=str(v)
        path=inputs/f'{name}.csv'
        with path.open('w') as f:csv.writer(f,lineterminator='\n').writerows(data)
        controls.append((name,family,path,'early_control'))
    publish('running')
    milestone('milestone 1: production-law study started','- Assignment read;36 exact family/taper/tip/angle cells +2 early116/2.62/18 controls. Direct unmodified production library/FEM/export, explicit faer; all requested cases evaluated including failures.44 annular sections at3 integration resolutions, direct law and native actual-export Gate S point/sharpness, three unbiased FEM meshes. Canonical genes/material/CPs preserved except46..49. Initial Rust syntax build failure/source retained and corrected; no production source edits or threshold changes. CPUs12,13/nice prefix/Cargo jobs2.')
    with ThreadPoolExecutor(max_workers=2) as pool:
        futures={pool.submit(evaluate,j):j for j in controls+JOBS}
        for f in as_completed(futures):
            job=futures[f]
            try:
                row=f.result();(CONTROLS if job[3]=='early_control' else ROWS).append(row)
                milestone(f'case {job[0]}',f'- Native geometry rejects={row["geometry_rejects"]}; aesthetic={row["aesthetic_pass"]}; Gate S={row["gate_s_pass"]}; FEM complete={row["fem_complete"]}; lateral/fatigue={row["lateral_fatigue_screen"]}. Full qualification=false.')
            except Exception as e:ERRORS.append(dict(name=job[0],error=repr(e)))
            publish('running');print(job[0],'grid',len(ROWS),'controls',len(CONTROLS),'errors',ERRORS,flush=True)
    assert len(ROWS)==36 and len(CONTROLS)==2 and not ERRORS,ERRORS
    publish('complete');milestone('milestone 2: all requested measurements retained','- Grid36 cases/108 FEM observations,2 early controls/6 FEM observations;44 sections/case at3 quadrature levels and native actual-export Gate S measured for all38. Portable audit/unit/report detail and commit attempt follow. Full qualification=false; no threshold changes.')
