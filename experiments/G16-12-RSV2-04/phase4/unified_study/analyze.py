from pathlib import Path
import csv,hashlib,json,math,subprocess
OUT=Path(__file__).resolve().parent;ROOT=OUT.parents[2]
load=lambda p:json.loads(p.read_text())
CASES=['combined_d065','combined_d055','combined_d048']
KEYS=['lat_rob','sf_hunter','fat_sf','seq_ratio','twist_body_sf','twist_tipzone_sf','hm_twist_sf','hm_spine_sf','hm_ringback_sf']
LIMITS={'lat_rob':358.,'sf_hunter':1.377,'fat_sf':1.,'seq_ratio':1.25,'twist_body_sf':1.,'twist_tipzone_sf':1.,'hm_twist_sf':1.,'hm_spine_sf':1.,'hm_ringback_sf':1.}
provenance=load(OUT/'source_provenance.json')
assert all(hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h for p,h in provenance['production_sha256'].items())
for row in provenance['inputs'].values():assert hashlib.sha256((OUT/row['snapshot']).read_bytes()).hexdigest()==row['sha256']
fem=(OUT/'prototype_fem.rs').read_text().replace('\n    crate::diagnostic::record(&t6, nod_sp.as_deref(), nod_rb.as_deref(), tp, sx0);\n    crate::diagnostic::record_sequence(&cen,&excl,&lat,&twist,p.heel.0,&te,&t6,&nod_pr,&ring6,tp);','')
assert fem==(ROOT/'crates/crescent/src/fem.rs').read_text()
hotlaw=(OUT/'inputs/hotspot/experiments/rsv2_07/compound.rs').read_text();law=(OUT/'compound.rs').read_text()
cap=lambda s:s[s.index('pub fn point_cap('):s.index('pub fn thickness_at(')]
assert cap(hotlaw)==cap(law)
section=lambda s:s[s.index('    let u=axis(p);let delta=sub(q,p.tip);'):s.index('\n    let end=if c.point_shape>0')]
assert section(hotlaw)==section(law)
base=list(csv.reader((OUT/'neck_control.csv').open()))[1];combined=list(csv.reader((OUT/'combined.csv').open()))[1]
assert [i for i in range(128) if float(base[i])!=float(combined[i])]==[7,8]
assert float(combined[7])==float(base[7])+.3 and float(combined[8])==float(base[8])+.2 and base[-1]==combined[-1]
controls=load(OUT/'control_verification.json')
assert all(x['all_reported_scalar_metrics_match'] and x['max_relative_error']<1e-10 for x in controls.values())
docs={name:load(OUT/(name+'_fem.json')) for name in CASES};summary={}
for name,d in docs.items():
 assert [l['area'] for l in d['levels']]==[2.,.25,.125,.0625]
 assert d['input_was_canonical'] and d['within_genome_domain'] and d['rs_v2'] and d['aes_pass'] and not d['rejects']
 assert d['cpu_affinity'].endswith('10-11') and d['cargo_build_jobs']=='2' and d['solver']=='faer'
 assert d['config']['neck_full'] and d['config']['neck_solid'] and d['config']['window_compound'] and d['config']['point_shape']==3 and d['config']['point_lateral']==2
 assert d['window_count']==1 and 'comet' not in d['holes'] and d['neck_section']['min_mm']==6. and d['neck_section']['max_mm']==6.
 assert all(n in ['ring','cairn'] or bounds[0]>=d['heel'][0] for n,bounds in zip(d['holes'],d['hole_x_bounds']))
 for l in d['levels']:
  assert l['unbiased'];m=l['metrics'];assert all(math.isfinite(m[k]) for k in KEYS)
  assert math.isclose(m['hm_twist_sf'],m['twist_sf']*1820/16200,rel_tol=1e-12)
  assert math.isclose(m['hm_spine_sf'],m['hm_spine_n']/3427,rel_tol=1e-12)
  assert math.isclose(m['fat_sf'],m['lat_rob']/357.996288224513,rel_tol=1e-12)
 cert={k:min(l['metrics'][k] for l in d['levels'] if l['area'] in [.25,.125]) for k in KEYS};strict=d['levels'][-1]['metrics']
 checks={k:cert[k]>=v and strict[k]>=v for k,v in LIMITS.items()}
 summary[name]={'d0_mm':d['config']['d0'],'d1_mm':d['config']['d1'],'certification_minimum':cert,'strict':{k:strict[k] for k in KEYS},'assignment_checks_certification_and_strict':checks,'all_assignment_targets_pass':all(checks.values()),'coarse':d['levels'][0]['metrics']}
exports={name:load(OUT/(name+'_export.json')) for name in CASES+['combined_d065_h025']}
export_summary={}
for name,d in exports.items():
 assert d['input_was_canonical'] and d['cpu_affinity'].endswith('10-11') and d['cargo_build_jobs']=='2' and d['solver']=='faer'
 m=d['measurement'];s=m['sharpness'];p=m['point'];assert s is not None and p is not None and m['bore_edge_radius_mm'] is not None
 export_summary[name]={'grid_mm':d['config']['grid'],'sharpness_pass':s['pass'],'point_profile_pass':p['pass'],'bore_radius_pass':m['bore_edge_radius_mm']>=1.5,'watertight_positive_volume_pass':d['solid']['census']==[0,0,1] and d['solid']['volume_mm3']>0,'all_export_checks_pass':s['pass'] and p['pass'] and m['bore_edge_radius_mm']>=1.5 and d['solid']['census']==[0,0,1] and d['solid']['volume_mm3']>0,'measurement':m,'solid':d['solid'],'worst_edge_face':d['worst_edge_face']}
# Refining only the exporter grid leaves every physical construction parameter and input gene unchanged.
fine=exports['combined_d065_h025'];coarse=exports['combined_d065'];assert fine['genes']==coarse['genes']
assert {k:v for k,v in fine['config'].items() if k!='grid'}=={k:v for k,v in coarse['config'].items() if k!='grid'}
checks={'all_19_snapshotted_inputs_verified':len(provenance['inputs'])==19,'all_77_production_source_requirement_build_digests_unchanged':len(provenance['production_sha256'])==77,'native_fem_diff_only_postcalculation_observation_hooks':True,'directional_point_cap_and_coordinate_law_match_hotspot_source':True,'combined_csv_changes_only_CP7_plus_0p3_CP8_plus_0p2':True,'both_witness_controls_reproduce':controls,'three_combined_cases_have_all_four_meshes':True,'all_combined_inputs_canonical_and_native_geometry_legal':True,'solid_6mm_neck_and_one_blade_window_verified':True,'all_combined_runs_cpus_10_11_jobs2_faer_unbiased':True,'native_metric_identities_pass':True,'all_four_export_measurements_complete':True,'fine_export_differs_only_by_grid':True,'any_combined_case_passes_all_assignment_targets':any(x['all_assignment_targets_pass'] for x in summary.values()),'tracked_git_status_clean':not subprocess.check_output(['git','status','--porcelain'],cwd=ROOT,text=True).strip()}
(OUT/'verification.json').write_text(json.dumps(checks,indent=2)+'\n')
(OUT/'summary.json').write_text(json.dumps({'limits':LIMITS,'qualification_scope':'minimum of unbiased area .25/.125 plus separate strict .0625; area 2 retained as coarse observation','fem':summary,'exports':export_summary,'source_controls':controls,'not_a_production_certificate':True},indent=2)+'\n')
lines=['# Phase-4 unified multi-lane prototype evaluation','',
'**Result:** the requested witnesses can be combined into a canonical, native-planform-legal prototype with a solid 6.0 mm neck and one controlled blade window. Their passing strength results do not transfer as a complete set. All three combined variants retain lateral/fatigue/hunter targets on the two certification meshes and the strict mesh, but each misses sequence >=1.25 on one certification mesh, and every combined human-max twist/spine factor remains below 1. No combined candidate passes all assignment targets.','',
'All Phase-4 sources, snapshots, invocation records, raw results, summaries and reproduction scripts are in `target/gate-feasibility-bound/unified_study/`. Worktree `/home/ubuntu/jarambit_wt_diag`, branch `audit/gate-feasibility-bound`, baseline commit `21425fe2948ab3941eedbe9710ebc3d29427faae`. Every computation uses CPUs 10,11 and `CARGO_BUILD_JOBS=2`; the existing lane target is reused. Production sources and requirements are unchanged.','',
'## Imported witnesses and control reproduction','',
'The neck/window input and config come from the assigned `lane-G16-06-RSV2-08/experiments/G16-06-RSV2-08` directory. The point-cap law and hotspot seed/strict receipt come from the assigned `lane-RSV2-07-hm` directory. Nineteen input/source/receipt files were copied once into `inputs/` and hashed; source lanes were not modified. Subsequent reproduction uses these stable snapshots.','',
'The copied neck config explicitly selects skyline, while the retained candidate validation receipt and its stated witness numbers use faer. A partial skyline control was stopped before strict completion and its completed observations were preserved in `neck_skyline_partial.json`. The effective configs for all complete controls and combined evaluations explicitly select faer. `solver_import_note.json` and invocation receipts record the discrepancy and override; parity tolerances were not loosened.','',
'| Local control | Areas mm² | Maximum relative difference from retained scalar metrics |','| --- | --- | --- |',f"| neck_control | 2, 0.25, 0.125, 0.0625 | {controls['neck_control']['max_relative_error']:.12g} |",f"| hotspot_control | 0.0625 | {controls['hotspot_control']['max_relative_error']:.12g} |",'',
'## Exact combined construction','',
f'The canonical `combined.csv` changes only CP7 from {float(base[7]):.17g} to {float(combined[7]):.17g} (+0.3 mm) and CP8 from {float(base[8]):.17g} to {float(combined[8]):.17g} (+0.2 mm) relative to the copied neck candidate. All other genes and material remain unchanged. Native clamping changes no gene. All combined variants pass the unchanged aesthetic and geometry predicates. This prototype additionally needs the supplied section law and prototype phenotype generator; the CSV alone does not reproduce it through the production phenotype.','',
'The neck source removes its neck comet slot, keeps the ring/cairn and a blade-only window, and uses full 6.0 mm stock/knots. The controlled window keeps WIN_N=1, WIN_W=6.0 mm and WIN_FILL=0.65; actual window area is 175.129120106 mm² in all combined cases. The original 3.2 mm window rim is retained, with clearance computed from the actual compound-bevel extent. No hole is moved into the neck.','',
'Let S(u) be cubic smoothstep after clamping u to [0,1], R(q) the native raster value and x the grip coordinate. The neck source defines N(q)=R(q)+(6-R(q))*S((x-ring_od/2)/3), then B(q)=N(q)+(6-N(q))*S((x-heel_x)/5). All sampled material beyond the ring transition and before the heel, outside the cutting-edge band, is exactly 6.0 mm; the analytic formula also preserves this full-stock neck scope. The inherited ring groove/collar remains.','',
'The micro-bevel retains eta=0.175 mm and theta=18 degrees. With actual minimum cutting-edge segment distance d, L(d)=eta+2*d*tan(theta); E=L for d<=d0, E=L+(B-L)*S((d-d0)/(d1-d0)) through the transition, and E=B beyond d1. Three variants use d0/d1=0.65/1.10, 0.55/0.85 and 0.48/0.65 mm. No extra blade-band transition is enabled.','',
'The imported directional point coordinate is a=hypot(axial,2*lateral), with the native tip bisector defining axial/lateral coordinates. Its POINT_SHAPE=3 envelope is reproduced byte for byte from the hotspot source:',
'','```text','P(a) = eta + 0.30*a                                    0 <= a <= 2',
'       eta+0.60 + (0.94*K-(eta+0.60))*(a-2)/1.2         2 < a <= 3.2',
'       K*(0.94 + 0.005*(a-3.2))                        3.2 < a <= 5.2',
'       K*(0.95 + 0.05*(a-5.2)/6.8)                     5.2 < a <= 12',
'       +infinity                                       a > 12','```','',
'K=6 mm for the combined inputs. The final law applies min(B,E,P), retains the native bore collar, and floors thickness at 0.05 mm. Raster construction, its 0.25 mm spacing and f32/nearest-cell behavior, and subsequent evaluations of the shared FEM/export thickness law retain the imported ordering. They are not replaced by an ideal continuum stock field. The native FEM mesh, four-sample element-thickness averaging, load patches, 4 mm exclusion, T6 nodal peaks, joint factors, St-Venant correction, fatigue model and numerical thresholds are unchanged.','',
'## All four native FEM levels','',
'All runs are unbiased with faer. Area=2 mm² is retained as a coarse observation. The source witness protocol compares min(0.25,0.125 mm²) and a separate 0.0625 mm² strict check; no mesh level is selected because it passes. Mesh values need not vary monotonically. The requested stronger margins are lateral >=358 N, hunter SF >=1.377 and sequence >=1.25; native sequence remains >=1.20. Fatigue and the human-max/tip/body safety-factor targets remain >=1.','',
'| Case (d0/d1 mm) | Area mm² | Lateral N | Hunter SF | Fatigue SF | Sequence | Body twist SF | Tip-zone SF | HM twist SF | HM spine SF | HM ring-back SF |','| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |']
for name,d in docs.items():
 for l in d['levels']:lines.append('| '+name+' | '+str(l['area'])+' | '+' | '.join(f"{l['metrics'][k]:.9g}" for k in KEYS)+' |')
lines+=['','Protocol minima and separate strict results:','','| Case | Protocol | Lateral N | Fatigue SF | Sequence | HM twist SF | HM spine SF |','| --- | --- | --- | --- | --- | --- | --- |']
for name,s in summary.items():
 for label,key in [('min(0.25,0.125)','certification_minimum'),('strict 0.0625','strict')]:lines.append('| '+name+' | '+label+' | '+' | '.join(f"{s[key][k]:.9g}" for k in ['lat_rob','fat_sf','seq_ratio','hm_twist_sf','hm_spine_sf'])+' |')
lines+=['',
'The strongest two-mesh human-max result among these three is d0/d1=0.48/0.65: twist 0.386575096 and spine 0.565685833; its strict values are 0.429888472 and 0.511348390. This does not establish a global optimum. The default neck-witness bevel 0.65/1.10 has two-mesh twist/spine 0.281926575 / 0.524061980, with strict 0.353856653 / 0.466910823. None inherits the hotspot witness spine SF of 1.041424487.','',
'All three fine/strict lateral and fatigue checks pass, but assignment sequence minima are 1.236935124, 1.222186639 and 1.221363356. These miss 1.25 while satisfying the native 1.20 threshold. All three area=2 observations fail the lateral 358 N and fatigue 1 targets. All combined body/tip-zone and ring-back native safety factors are retained in the full table; no human-max failure is hidden by the passing lower-torque twist floor.','',
'## Governing peaks explain the lost transfer','',
'The strict hotspot control spine peak is (157.028067,-9.861161) mm at unit-load stress 0.208057822 MPa/N. All three combined strict spine peaks move to (113.951102,-12.509824) mm; unit-load stresses are 0.464064012, 0.446888785 and 0.423735586 MPa/N in descending bevel-width order. The local hotspot reinforcement does not eliminate the governing stress in the combined geometry. Full hand/blade sequence peaks, unit stresses, thicknesses and twist section parameters are retained in each raw receipt. This is a native-model observation, not an independent 3D qualification.','',
'## Native actual-export measurements','',
'The exact native certification routines measure the exported facets. Requirement comparisons remain apex <=0.20 mm, every relevant face half-angle <=25 degrees within the first 0.4 mm edge strip, point thickness <=0.30/0.60/1.00 mm at 0.25/1/2 mm, native point-profile monotonicity and taper length >=5 mm, bore radius >=1.5 mm and positive-volume watertight census [0,0,1]. No sampled replacement of a triangle check or added tolerance is used.','',
'| Export | Strip grid mm | Faces | Max apex mm | Max face half-angle degrees | Sharpness | Point profile | Bore radius mm | Census |','| --- | --- | --- | --- | --- | --- | --- | --- | --- |']
for name,d in exports.items():
 m=d['measurement'];s=m['sharpness'];lines.append(f"| {name} | {d['config']['grid']} | {d['solid']['triangles']} | {s['max_apex_mm']:.9g} | {s['max_half_angle_deg']:.9g} | {s['pass']} | {m['point']['pass']} | {m['bore_edge_radius_mm']:.9g} | {d['solid']['census']} |")
refined=exports['combined_d065_h025'];m=refined['measurement']
lines+=['',
'The 0.05 mm exports all fail sharpness at about 28.543457 degrees. Their common worst face is near (100.67,-12.76) mm at r_tip about 96.2 mm, with vertex cutting-edge distances 0 / 0.008967 / 0.015625 mm. It lies entirely within the linear micro-bevel, so widening d0 alone does not resolve that faceted-angle failure. The actual worst triangle is retained, rather than inferring sharpness from the 18 degree input angle.','',
f"The primary 0.65/1.10 construction was also fully measured using the hotspot source grid of 0.025 mm. Its measured maximum half-angle is {m['sharpness']['max_half_angle_deg']:.12g} degrees and native sharpness verdict is {m['sharpness']['pass']}; point-profile verdict is {m['point']['pass']}. All physical law parameters and input genes are identical between these two primary export runs; only the exporter grid changes. FEM was not repeated because the exporter grid is not an input to its native mesher or thickness law. This comparison does not establish continuum convergence.",'',
'## Verification, scope and reproduction','',
'`verification.json` records all control, schema, native-clamp, construction, metric-identity and unchanged-source checks. Seventy-seven production source/requirement/build-file digests and all 19 snapshot digests are verified. Removing the two postcalculation diagnostic hooks reproduces the production FEM source exactly. `input_provenance.json` identifies every external source and its stable snapshot; `run_provenance.json` records compiler and dependency identities.','',
'Run `taskset -c 10,11 env CARGO_BUILD_JOBS=2 bash target/gate-feasibility-bound/unified_study/reproduce.sh` to rebuild locally and rerun both witness controls, all three combined four-mesh cases and all four completed native export measurements. It uses retained local input snapshots and the existing lane release dependencies. `SHA256SUMS` seals the report and artifacts; the report SHA-256 is recorded in the handoff and changelog.','',
'This is a numerical prototype study. Independent 3D P2 solid/cross-solver, GCI adjudication and support/buckling batteries remain unassessed. The inherited ring collar extends beyond nominal flat stock and has no process qualification here. No production certificate, RS-v2 all-hard pass, global infeasibility proof, threshold relaxation, source promotion or push follows from these measurements. Tracked git status is clean.','',
'The next structural investigation should address the combined spine peak near x=114 mm while preserving the blade/hand sequence balance, and retain actual-export sharpness checks for every construction change. The separate source witnesses are useful controls, but their passing values cannot be pooled into one qualified candidate.']
(OUT.parent/'UNIFIED_PROTOTYPE_REPORT.md').write_text('\n'.join(lines)+'\n')
print(json.dumps({'verification':checks,'assignment_results':summary,'export_passes':{n:r['all_export_checks_pass'] for n,r in export_summary.items()}},indent=2))
