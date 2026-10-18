"""Bind the observed Phase-6 results to all documented gate predicates."""
from pathlib import Path
import datetime
import hashlib
import json
import math
import re
import sys

ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
sys.path.insert(0,str(ROOT))
from ops.ci.evaluate_cad_candidate import compute_weighted_cad_score


def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def write(p,d):p.write_text(json.dumps(d,indent=2,allow_nan=False)+'\n')


def inspect_stl(path):
    vertices={};points=[];tri=[];edges={};directed={};volumes=[];nface=0
    for line in path.read_text().splitlines():
        fields=line.split()
        if not fields or fields[0]!='vertex':continue
        point=tuple(float(x) for x in fields[1:])
        if point not in vertices:vertices[point]=len(vertices)
        tri.append(vertices[point]);points.append(point)
        if len(tri)!=3:continue
        for a,b in zip(tri,tri[1:]+tri[:1]):
            key=(min(a,b),max(a,b));edges[key]=edges.get(key,0)+1
            directed[a,b]=directed.get((a,b),0)+1
        a,b,c=points
        volumes.append((a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]))/6)
        tri=[];points=[];nface+=1
    assert not tri
    census=[sum(v==1 for v in edges.values()),sum(v>2 for v in edges.values()),max(directed.values())]
    return {'stl_sha256':sha(path),'vertices':len(vertices),'triangles':nface,
            'census':census,'signed_volume_mm3':math.fsum(volumes),'watertight':census==[0,0,1]}


def main():
    out=Path(sys.argv[1]).resolve()
    raw=json.loads((out/'results.json').read_text())
    provenance=json.loads((out/'provenance.json').read_text())
    for f,h in provenance['source_sha256'].items():assert sha(ROOT/f)==h,f
    definitions={}
    for line in (ROOT/'docs/WORKPLAN.md').read_text().splitlines():
        m=re.match(r'\| `(G16-\d\d|RSV2-\d\d)` \| `([^`]+)` \|',line)
        if m:
            cells=[s.strip() for s in line.strip().strip('|').split('|')]
            definitions[m[1]]={'key':m[2],'documented_predicate':cells[4],
                'source':'docs/WORKPLAN.md sections 3B/3C'}
    assert len(definitions)==26
    result={'schema':'jarambit.rsv2_07.phase6_unified_scorecard.v1','task':'RSV2-07-p6',
        'timestamp_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
        'prototype_only':True,'all_gate_qualification':False,'provenance':provenance,
        'gate_definitions':definitions,'candidates':{},'raw_directory':str(out.relative_to(ROOT)),
        'interpretation':{'matrix':'documented engineering predicates on explicitly bound compound-prototype evidence',
            'certification_pair_mm2':[.25,.125],'strict_mesh_mm2':.0625,
            'canonical_gci_areas_fine_to_coarse_mm2':[.25,1.,4.],
            'G16_15':'sequence_valid && observable_valid, as current evaluator; battery admission is reported separately',
            'native_bearing_minimum_mm':10.,'phase6_bearing_minimum_mm':12.,
            'CERT_02':'numeric bore checks do not supply owner construction/aesthetic sign-off',
            'formal_native_certificate_package':'not issued for the compound prototype harness'}}
    if (out/'resume_provenance.json').exists():
        result['acquisition_resumption']=json.loads((out/'resume_provenance.json').read_text())
        resume=result['acquisition_resumption']
        resume_source=ROOT/resume['source']
        if not resume_source.exists():
            resume_source=out/'interrupted_phase6_resume.py'
            resume['archived_source']=str(resume_source.relative_to(ROOT))
        assert sha(resume_source)==resume['sha256']
        assert resume['cpu_affinity']==[14,15] and resume['nice']==10
        resume['scheduling_record']=json.loads((out/'scheduling_resumption.json').read_text())
    names=['Q7','placement_05','Q7_window_p05']
    summary_rows=[];eng_rows=[];rs_rows=[];plate_sections=[];gci_rows=[];cert_rows=[]
    export_rows=[];solid_rows=[];cad_rows=[]
    for name in names:
        r=raw[name];p=r['plate'];export=r['export'];check=r['cad_check'];solid=r['solid'];c=p['certification']
        folder=out/name
        assert p['candidate_sha256']==r['candidate_sha256']==sha(folder/'candidate.csv')
        assert p['genes']==export['genes']==solid['genes']
        assert p['compound_config']==export['compound_config']==solid['effective_compound_config']
        assert sha(folder/'cad/surface.json')==solid['source_surface_sha256']
        levels=p['levels_coarse_to_fine'];assert [l['area_mm2'] for l in levels]==[4.,1.,.25,.125,.0625]
        for l in levels:assert sha(folder/f"mesh_{l['area_mm2']:g}.json")==l['mesh_sha256']
        for key in ['lat_rob','ring_rob','twist_sf','twist_body_sf','twist_tipzone_sf','seq_ratio',
                    'hm_twist_sf','hm_spine_sf','hm_ringback_sf','fat_sf']:
            assert math.isclose(c[key],min(levels[2]['metrics'][key],levels[3]['metrics'][key]),rel_tol=1e-12),key
        a4=p['side_a4'];assert a4['certified_seq_ratio']==c['seq_ratio']
        assert a4['mesh_sha256']==levels[2]['mesh_sha256']
        assert a4['buckling_critical_load_n']==260*a4['buckling_eigenvalue']
        assert math.isclose(a4['supported_seq_ratio'],a4['supported_blade_peak_mpa_per_n']/a4['supported_hand_peak_mpa_per_n'],rel_tol=1e-12)
        assert (a4['status']=='pass')==(a4['buckling_eigenvalue']>0 and a4['supported_seq_ratio']>=c['seq_ratio'])
        for f,h in solid['artifacts_sha256'].items():assert sha(ROOT/f)==h,f
        dx=solid['dolfinx_cross_check'];sk=solid['primary'];sf=solid['SF_solid_nodal_max']
        errors={op:abs(dx['raw']['SF_nodal_'+op]/sk['solid']['SF_solid_nodal_'+op]-1) for op in ['max','avg']}
        assert errors==dx['relative_errors_vs_primary']
        cross=solid['primary_converged'] and sk['solver']['relative_residual']<=1e-10 and max(errors.values())<=.02
        assert cross==dx['within_2pct']
        errors_vs_dx={op:abs(sk['solid']['SF_solid_nodal_'+op]-dx['raw']['SF_nodal_'+op])/dx['raw']['SF_nodal_'+op] for op in ['max','avg']}
        dx['relative_errors_vs_dolfinx']=errors_vs_dx
        native_cross=solid['primary_converged'] and sk['solver']['relative_residual']<=1e-10 and max(errors_vs_dx.values())<=.02
        assert native_cross==cross, 'denominator conventions differ at the decision boundary'
        evaluator_sf=dx['raw']['SF_nodal_max']
        assert dx['solver_sidecar']['inputs']['/work/solid_mesh.npz']==solid['mesh_sha256']
        assert dx['runtime']['dolfinx']=='0.9.0' and dx['runtime']['cpus']==[14,15] and dx['runtime']['nice']==10
        geom=p['geometry'];aes=p['aes'];strict=levels[-1]['metrics'];measure=export['measurements']
        bearing=check['measured_bearing_width_mm'];radius=measure['bore_edge_radius_mm']
        assert abs(bearing-export['native_bearing_width_and_band_mm'][0])<1e-7
        assert radius is not None and abs(radius-check['minimum_mesh_roll_radius_mm'])<1e-7
        archetype=(abs(geom['ring_id_mm']-25.4)<=.5+1e-6 and geom['ring_wall_mm']>=4.5-1e-6
            and geom['groove_w_mm']>=2-1e-6 and aes['hook_drop_mm']>=25-1e-6
            and min(geom[k] for k in ['tip_to_heel_mm','horizontal_reach_mm','sharpened_arc_mm'])>=85.725-1e-6
            and geom['teeth_in']>=3 and geom['teeth_sp']>=3 and aes['pass'])
        fit=p['gci']['seq_ratio']
        cad=check['cad'];step=check['step_check']
        stl=inspect_stl(folder/'cad/candidate.stl')
        stl['native_coordinate_decimal_places']=4
        stl['native_unrounded_volume_mm3']=export['solid']['volume_mm3']
        stl['relative_volume_difference_from_unrounded_export']=abs(stl['signed_volume_mm3']/export['solid']['volume_mm3']-1)
        stl['relative_mass_difference_from_raster']=abs(stl['signed_volume_mm3']*.00443-p['mass_raster_g'])/p['mass_raster_g']
        write(folder/'cad/stl_census.json',stl)
        assert stl['triangles']==export['solid']['triangles']
        assert stl['signed_volume_mm3']>0
        flags=[cad['cad_valid'] and r['deterministic'] and step['valid'] and step['solids']==1
            and stl['watertight'] and stl['relative_mass_difference_from_raster']<=.01,
            not p['rejects'],archetype,aes['pass'],c['lat_rob']>=180,c['lat_rob']>=260,
            strict['lat_rob']>=260,c['ring_rob']>=1500,c['twist_body_sf']>=1,c['twist_sf']>=1,
            strict['twist_tipzone_sf']>=1,evaluator_sf>=1,a4['status']=='pass' and a4['buckling_eigenvalue']>=1,
            c['seq_ratio']>=1.2,fit['sequence_valid'] and fit['observable_valid'],native_cross]
        flags=list(map(bool,flags))
        eng={f'G16-{i+1:02}':{'key':definitions[f'G16-{i+1:02}']['key'],
                'status':'pass' if v else 'fail'} for i,v in enumerate(flags)}
        human_structural=all(c[k]>=1 for k in ['hm_twist_sf','hm_spine_sf','hm_ringback_sf'])
        native_human=human_structural and bearing>=10 and radius>=1.5
        phase6_bore=(bearing>=12 and radius>=1.5 and step['bore_geometry_matches_mesh']
            and step['bore_edge_radius_mm'] is not None and step['bore_edge_radius_mm']>=1.5)
        rs_flags=[aes['pass'] and not p['rejects'],c['floors_ok'],c['twist_body_sf']>=1 and strict['twist_tipzone_sf']>=1,
            evaluator_sf>=1 and native_cross,strict['lat_rob']>=260,c['seq_ratio']>=1.2,native_human,c['fat_sf']>=1,
            measure['sharpness']['pass'] if measure['sharpness'] else None,
            measure['point']['pass'] if measure['point'] else None]
        rs={f'RSV2-{i+1:02}':{'key':definitions[f'RSV2-{i+1:02}']['key'],
            'status':'unverified' if v is None else 'pass' if v else 'fail'} for i,v in enumerate(rs_flags)}
        rs['RSV2-07']['phase6_stricter_12mm_status']='pass' if human_structural and phase6_bore else 'fail'
        rs['RSV2-04']['formal_native_certificate_binding_status']='unverified'
        gate_map={eng[k]['key']:flags[i] for i,k in enumerate(eng)}
        shortfalls={'cad':0. if flags[0] else 1.,'karambit_archetype':0. if archetype else 1.,
            'aesthetic':0. if aes['pass'] else max(.5-aes['score'],.1),'geometry_gates':float(len(p['rejects'])),
            'hunter_strict':max(0.,(260-strict['lat_rob'])/260),'ring_pull':max(0.,(1500-c['ring_rob'])/1500),
            'twist_body':max(0.,1-c['twist_body_sf']),'twist_tip_strict':max(0.,1-strict['twist_tipzone_sf']),
            'solid_tip_3d':max(0.,1-evaluator_sf),'seq_ratio':max(0.,(1.2-c['seq_ratio'])/1.2)}
        score=compute_weighted_cad_score({'gates':gate_map,'shortfalls':shortfalls},
            {'mass_g':p['mass_raster_g'],'strict_lat_rob_0p0625_n':strict['lat_rob'],'solid_tip_sf_nodal_max':evaluator_sf,
             'twist_tipzone_sf_strict':strict['twist_tipzone_sf'],'seq_ratio':c['seq_ratio'],'fat_sf':c['fat_sf'],
             'aes_score':aes['score'],'buckling_eigenvalue':a4['buckling_eigenvalue'],
             'hook_drop_mm':aes['hook_drop_mm'],'inside_concave':aes['inside_concave'],'spine_convex':aes['spine_convex']})
        case={'candidate_sha256':r['candidate_sha256'],'candidate_file':r['candidate_file'],
            'genes':p['genes'],'compound_config':p['compound_config'],'engineering_gates':eng,'rs_v2_gates':rs,
            'engineering_pass_count':sum(flags),'rs_v2_pass_count':sum(v['status']=='pass' for v in rs.values()),
            'all_16_engineering_pass':all(flags),'all_10_rs_v2_pass':all(v['status']=='pass' for v in rs.values()),
            'formal_qualification':False,'weighted_cad_score':score,'levels_coarse_to_fine':levels,
            'certification':c,'side_a4':a4,'gci':p['gci'],'export':export,'cad_check':check,'serialized_stl_check':stl,
            'solid_3d':solid,'solid_evidence_origin':'new Phase-6 solve' if name=='Q7_window_p05' else 'Phase-5 exact hash-bound reuse',
            'CERT_02':{'measured_bearing_width_mm':bearing,'required_bearing_width_mm':12.,
                'native_required_bearing_width_mm':10.,'measured_native_edge_radius_mm':radius,
                'minimum_mesh_fit_radius_mm':check['minimum_mesh_roll_radius_mm'],
                'step_edge_radius_mm':step['bore_edge_radius_mm'],'required_edge_radius_mm':1.5,
                'numeric_status':'pass' if phase6_bore else 'fail',
                'owner_construction_and_final_aesthetic_signoff':'unverified'},
            'battery_plate_gci':{'admitted':fit['admitted'],'fit_status':fit['fit_status'],
                'relative_error_under_3pct':bool(fit['admitted'] and fit['gci_fine']<.03)},
            'additional_refinement_observation':{
                'sequence_relative_change_0p125_vs_0p25':abs(levels[3]['metrics']['seq_ratio']/levels[2]['metrics']['seq_ratio']-1),
                'sequence_relative_change_0p0625_vs_0p25':abs(levels[4]['metrics']['seq_ratio']/levels[2]['metrics']['seq_ratio']-1),
                'native_asymptotic_validation':fit['asymptotic_validation']},
            'verification':{'canonical_genes_and_section_law_match_all_observations':True,
                'production_surface_matches_3d_source_sha256':True,'all_plate_mesh_hashes_verified':True,
                'native_certification_pair_minima_verified':True,'a4_identities_verified':True,
                'primary_independent_errors_recomputed':True,'raw_artifacts_sha256_verified':True}}
        if name=='Q7_window_p05':case['hybrid_changes']=json.loads((folder/'hybrid_changes.json').read_text())
        if (folder/'existing_witness_plate_replay.json').exists():
            case['existing_witness_plate_replay']=json.loads((folder/'existing_witness_plate_replay.json').read_text())
        optional=folder/'cad_as_written/cad_check.json'
        if optional.exists():
            configuration=json.loads(optional.with_name('reader_configuration.json').read_text())
            assert sha(optional)==configuration['result_sha256']
            assert configuration['tolerances_unchanged']
            for f,h in configuration['source_inputs_sha256'].items():
                assert sha(folder/'cad'/f)==sha(optional.parent/f)==h
            case['additional_as_written_cad_diagnostic']={
                'result':json.loads(optional.read_text()),'configuration':configuration,
                'used_for_gate_matrix':False}
        result['candidates'][name]=case
        summary_rows.append(f"| {name} | {sum(flags)}/16 | {case['rs_v2_pass_count']}/10 | {c['seq_ratio']:.6f} | {c['hm_twist_sf']:.6f} | {sf:.3f} / {dx['raw']['SF_nodal_max']:.4f} | {score['W_CAD']:.6f} |")
        sharp=measure['sharpness'];point=measure['point']
        export_rows.append(f"| {name} | {sharp['max_apex_mm']:.6f} | {sharp['max_half_angle_deg']:.6f} | {point['t_0p25_mm']:.6f} / {point['t_1p00_mm']:.6f} / {point['t_2p00_mm']:.6f} | {point['monotonic']} / {point['taper_lower_bound_mm']:.6f} |")
        solid_rows.append(f"| {name} | {sf:.3f} / {solid['SF_solid_nodal_avg']:.3f} | {dx['raw']['SF_nodal_max']:.4f} / {dx['raw']['SF_nodal_avg']:.4f} | {100*errors['max']:.8f}% / {100*errors['avg']:.8f}% | {sk['solver']['relative_residual']:.6g} | {sk['solid']['n_dof']} |")
        cad_rows.append(f"| {name} | {p['mass_raster_g']:.6f} / {cad['mass_3d_g']:.6f} | {100*cad['rel_mass_diff']:.6f}% | {r['deterministic']} | {step['valid']} / {step['solids']} | {step['bore_geometry_matches_mesh']} |")
        cert_rows.append(f"| {name} | {bearing:.9f} | {radius:.9f} | {case['CERT_02']['numeric_status']} | unverified |")
        for key,value in p['gci'].items():gci_rows.append(f"| {name} | {key} | {value['fit_status']} | {str(value['admitted']).lower()} | {json.dumps(value['gci_fine'])} |")
        table=['| area mm² | nodes / elements | lateral N | seq | HM twist | HM spine | HM ring-back | fatigue | body twist / tip zone |',
               '| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |']
        for l in levels:
            m=l['metrics'];table.append(f"| {l['area_mm2']:g} | {m['nodes']} / {m['tris']} | {m['lat_rob']:.6f} | {m['seq_ratio']:.6f} | {m['hm_twist_sf']:.6f} | {m['hm_spine_sf']:.6f} | {m['hm_ringback_sf']:.6f} | {m['fat_sf']:.6f} | {m['twist_body_sf']:.6f} / {m['twist_tipzone_sf']:.6f} |")
        plate_sections.append(f"### {name}\n\n"+'\n'.join(table)+f"\n\nNative certify: lateral={c['lat_rob']:.9f} N, ring={c['ring_rob']:.9f} N, sequence={c['seq_ratio']:.9f}, body/tip SF={c['twist_body_sf']:.9f}/{c['twist_sf']:.9f}, HM twist/spine/ring-back={c['hm_twist_sf']:.9f}/{c['hm_spine_sf']:.9f}/{c['hm_ringback_sf']:.9f}, fatigue={c['fat_sf']:.9f}.\n\nA4 status={a4['status']}, buckling eigenvalue={a4['buckling_eigenvalue']:.9f}, critical load={a4['buckling_critical_load_n']:.9f} N, supported sequence={a4['supported_seq_ratio']:.9f}. This support model does not replace the production ring-clamp sequence.\n")
    for prefix,destination in [('G16-',eng_rows),('RSV2-',rs_rows)]:
        for key,definition in definitions.items():
            if key.startswith(prefix):
                group='engineering_gates' if prefix=='G16-' else 'rs_v2_gates'
                destination.append(f"| {key} | {definition['key']} | "+' | '.join(result['candidates'][n][group][key]['status'] for n in names)+' |')
    observed_all=[n for n,c in result['candidates'].items() if c['all_16_engineering_pass'] and c['all_10_rs_v2_pass']]
    result['observed_all_gate_candidates']=observed_all
    conclusion='No candidate passes all observed gates.' if not observed_all else 'All observed gates pass for: '+', '.join(observed_all)+'.'
    report=f'''# RSV2-07 Phase 6: unified plate, CAD and solid scorecard

All three requested candidates were evaluated on the full five-mesh plate study and the documented 16 engineering / 10 RS-v2 gate matrix. **{conclusion}** Gate observations apply to the explicitly bound compound prototypes; no formal native certificate or manufacturing promotion is issued.

| Candidate | Engineering | RS-v2 | Cert sequence ≥1.20 | Cert HM twist ≥1 | Primary / DOLFINx 3D max SF | W_CAD_v2 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
{chr(10).join(summary_rows)}

W_CAD_v2 is computed by the unchanged evaluator helper on the 16 engineering flags. The 10 RS-v2 hard gates and CERT-02 sign-off remain separate qualification requirements.

## Candidate construction and evidence identity

Q7 and placement_05 use their exact Phase-5 canonical genomes and their distinct saved section/phenotype laws. Their Phase-5 primary and independent observations are reused only after checking all retained hashes and reproducing their production surface bytes exactly. The new **Q7_window_p05** retains Q7's section law (d0/d1=0.48/0.65 mm, point shape3, point lateral2) and copies placement_05's actual window values: win_s0=0.05, win_s1=0.525, win_w=6.0, win_fill=0.70, ring_wall=6.4 mm. These lie within every assigned range; ring_wall was already6.4 in Q7. Other genes are unchanged. This is one specified hybrid, without a claim of optimizing the supplied ranges. Its canonical CSV is [phase6_Q7_window_p05.csv](results/phase6_Q7_window_p05.csv).

Production export uses `export::solid(&p,1.0)` with native RS-v2 internal cap0.25 mm², DP0.03 mm and no study grid insertion. The generated export body is byte-identical to production from `pub fn solid` onward. The witness compound thickness and existing phenotype variants remain explicitly included in the source identity. STEP/STL/surface regenerations match byte for byte for each candidate. Native exported sharpness, full point profiles and bore radii are measured anew on these production facets; dense-study sharpness observations are not transferred.

## All 16 engineering gates

IDs and exact predicates follow [WORKPLAN.md](../../docs/WORKPLAN.md) §3B and the current evaluator; full predicate text is preserved in the JSON receipt. Certification uses native `fem::certify` (the conservative 0.25/0.125 mm² pair). Strict checks use0.0625 mm². Coarse meshes are diagnostic and do not replace certification.

| ID | Gate | Q7 | placement_05 | Q7_window_p05 |
| --- | --- | --- | --- | --- |
{chr(10).join(eng_rows)}

G16-01 includes the existing unit/single-solid/watertight/mass check, deterministic regeneration and independent OpenCascade `BRepCheck_Analyzer`/solid count. G16-15 preserves the evaluator's exact `sequence_valid && observable_valid` rule. The separate battery `plate.gci` admission result is reported below; identity validity alone does not establish convergence.

CAD verdicts use the repository's normal STEP import, including standard shape processing, followed by the full analyzer and solid count. An optional Q7 import with an empty shape-processing sequence returned `valid=false`, `solids=1`; its input hashes and tolerance settings were unchanged. The matrix uses the completed standard-import results. The optional result is retained as an additional diagnostic. The [OCCT processing implementation](https://github.com/Open-Cascade-SAS/OCCT/blob/V7_8_1/src/XSAlgo/XSAlgo_AlgoContainer.cxx) documents this import-processing branch.

The written STL also receives an independent edge census and signed-volume calculation. The native STL writer rounds coordinates to4 decimal places, so its volume difference from the unrounded solid is recorded; the unchanged1% mass limit is checked against the raster for both the native CAD volume and the serialized STL. Full-precision surface JSON supplies the 3D solve.

| Candidate | Raster / CAD mass g | Relative mass difference ≤1% | Deterministic | OCC valid / solids | STEP/mesh bore match |
| --- | ---: | ---: | --- | --- | --- |
{chr(10).join(cad_rows)}

## All 10 RS-karambit-v2 gates

Predicates follow native `bench.rs::rs_v2_eval` and WORKPLAN §3C. The solid-tip row is a diagnostic observation supported by candidate/law/surface/mesh hashes and independent raw solves. Production certificate binding and owner promotion are not asserted for this isolated compound harness. The1000 N tip in-plane result remains record-only, as native code requires.

| ID | Gate | Q7 | placement_05 | Q7_window_p05 |
| --- | --- | --- | --- | --- |
{chr(10).join(rs_rows)}

Production-facet sharpness and point observations (limits: apex≤0.20 mm, half-angle≤25°, t0.25/t1/t2≤0.30/0.60/1.00 mm, monotonic=true and taper≥5 mm):

| Candidate | Maximum apex mm | Maximum half-angle deg | Point t0.25 / t1 / t2 mm | Monotonic / taper lower bound mm |
| --- | ---: | ---: | ---: | --- |
{chr(10).join(export_rows)}

## Five-mesh plate, native certification and A4

All capacities use the unchanged native loads, material, four-sample element thickness, exclusions and thin/hot knockdown. Native faer uses two Rayon threads on CPUs14,15. The placement_05 Phase-5 context used its configured skyline backend; Phase6 uses the supported faer backend for all plate studies, without changing geometry or the reused solid surface.

Q7's three existing fine-mesh witness records replay within1e-10 relative tolerance. The earlier placement_05 witness used skyline: its checked metrics differ from the new faer values by at most2.459e-6 relative (0.0002459%); they do not meet a1e-10 equality check. Both source records and every difference are retained. The scorecard uses the newly solved faer results and the unchanged physical gate thresholds.

{chr(10).join(plate_sections)}

## Native GCI and independent 3D checks

The six observables below call actual `jarambit_crescent::gci::compute_gci` with ordered areas0.25/1/4 mm², r=2 and Fs=1.25, bound to each solved mesh hash. Unadmitted fits retain null extrapolation/error/order fields where native code supplies null; no oscillatory or nonpositive fit is converted into an error bound.

Native `asymptotic_validation` remains `not_established`, including admitted fits. In particular, placement_05 has canonical sequence GCI0.4870% at0.25 mm², while its0.125/0.0625 mm² sequence values are1.310469/1.325634 versus1.231106 at0.25 mm². The canonical admission result is retained without extending its error bound to the whole five-mesh sequence.

| Candidate | Observable | Native fit status | admitted | relative GCI |
| --- | --- | --- | --- | ---: |
{chr(10).join(gci_rows)}

The solid protocol is unchanged `tip_solid.py 30 4 1`, PATCH2 mm and torque1820 N·mm. Primary convergence requires the independently recomputed relative residual ≤1e-10. DOLFINx v0.9.0 uses the identical saved mesh, cuts, forces, material and allowance; both nodal-max and nodal-average SFs must differ by ≤2%. The receipt includes both native reported SFs, primary residuals, both relative errors, solver settings, runtime versions and sidecars. Q7_window_p05 receives a new primary solve and new independent solve; no witness solid result is substituted for it.

| Candidate | Primary max / avg SF | DOLFINx max / avg SF | Error vs primary max / avg | Primary relative residual | P2 DOFs |
| --- | ---: | ---: | ---: | ---: | ---: |
{chr(10).join(solid_rows)}

## CERT-02 bore dimensions and sign-off

The assignment requires measured bearing≥12.0 mm and edge radius≥1.5 mm. Native RS-v2 currently uses bearing≥10.0 mm; both the native human-max rule and the additional stricter12 mm check are recorded. Bearing is measured from independently fitted exported meridians and checked against native bearing evaluation. Native radius fits, independent mesh circle fits and analytic STEP roll measurements are recorded separately.

| Candidate | Measured bearing mm | Native measured edge radius mm | ≥12 / ≥1.5 numeric check | Owner construction/aesthetic sign-off |
| --- | ---: | ---: | --- | --- |
{chr(10).join(cert_rows)}

Numeric compliance does not supply CERT-02 owner construction approval or final aesthetic sign-off. Those remain unverified; no approval is inferred from this assignment.

## Verification and reproduction

Raw evidence: `{out.relative_to(ROOT)}/`. All builds/solves are pinned to CPUs14,15 at nice10 with CARGO_BUILD_JOBS=2; OMP/OpenBLAS/MKL threads=1. The hybrid primary has a64 GiB address-space bound, and the independent container has explicit `--cpuset-cpus=14,15` and nice10. This lane's target directory is reused. Production solvers, gate thresholds, requirements and prior reports are unchanged.

After the hybrid primary/independent pair completed, a recorded scheduling resumption replaced the initial Python launcher to overlap independent CAD work within the same CPU pair. The unfinished Q7 CAD process ended without a result following that replacement; its exit status was not captured. That incomplete attempt is excluded; the exact unchanged Q7 CAD check was restarted in its own execution session. No completed plate or 3D result was lost, and no primary/independent numerical solver was interrupted. Original attempt logs/stacks, the scheduling correction, retry completion and final CAD results are retained. No exit code is invented for the interrupted CAD attempt. Scientific inputs, implementations and thresholds are unchanged.

The receipt [phase6_unified_scorecard.json](results/phase6_unified_scorecard.json) contains complete five-mesh values, exact predicate text, gate verdicts, native GCI/A4 results, export measurements, CAD checks, primary/independent records, source/input/binary hashes and all raw artifact hashes. Primary/independent input identities and errors, native certification minima, A4 algebraic identities, native mesh hashes and unchanged sources are checked. The separate retained mesh checker verifies positive tets, pure torque and disjoint load/clamp sets. This is a diagnostic scorecard, without a solid mesh-convergence study or a claim of full formal certification.

From a normal-priority shell at the repository root:

```sh
nice -n 10 taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/phase6_run.py
python3 experiments/rsv2_07/phase6_scorecard.py target/rsv2-07/phase6/<timestamp>
```

The first command retains raw results; the second binds the matrix/report. New raw runs refuse timestamp-directory reuse. The saved Phase-5 source and raw artifacts must remain available for exact evidence reuse.
'''
    for before,after in [('CPUs14,15','CPUs 14,15'),('nice10','nice 10'),('the16','the 16'),
        ('The10','The 10'),('The1000','The 1000'),('uses0.0625','uses 0.0625'),
        ('use0.0625','use 0.0625'),('within1e-10','within 1e-10'),('most2.459','most 2.459'),
        ('a1e-10','a 1e-10'),('unchanged1%','unchanged 1%'),('to4 decimal','to 4 decimal'),
        ('a64 GiB','a 64 GiB')]:report=report.replace(before,after)
    report_path=HERE/'PHASE6_UNIFIED_SCORECARD.md';report_path.write_text(report)
    result['report']={'path':str(report_path.relative_to(ROOT)),'sha256':sha(report_path)}
    result['postprocessor_sha256']=sha(Path(__file__))
    result['execution_status']='complete'
    result['verification']={'all_scientific_source_hashes_unchanged':True,
        'all_26_gate_definitions_present':True,'no_thresholds_relaxed':True,
        'all_internal_identity_and_algebra_checks_passed':True,
        'prior_witness_replay_1e_10_all_cases':False}
    result['artifacts_sha256']={str(p.relative_to(ROOT)):sha(p) for p in out.rglob('*') if p.is_file()}
    write(HERE/'results/phase6_unified_scorecard.json',result)
    print('\n'.join(summary_rows))


if __name__=='__main__':main()
