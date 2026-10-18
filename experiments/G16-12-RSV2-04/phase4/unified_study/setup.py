from pathlib import Path
import hashlib,json,csv
ROOT=Path('/home/ubuntu/jarambit_wt_diag');OUT=ROOT/'target/gate-feasibility-bound/unified_study'
if (OUT/'source_provenance.json').exists():
 previous=json.loads((OUT/'source_provenance.json').read_text())
 assert all(hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==digest for name,digest in previous['production_sha256'].items()),'Production inputs changed since this study snapshot'
NECK=Path('/home/ubuntu/jarambit/.agent/worktrees/lane-G16-06-RSV2-08')
HOT=Path('/home/ubuntu/jarambit/.agent/worktrees/lane-RSV2-07-hm')
inputs={
 'neck':(NECK,['experiments/G16-06-RSV2-08/'+n for n in ['candidate.csv','candidate_config.json','compound.rs','generate.py','main.rs','diagnostic.rs','verify_candidate.py']]+['target/G16-06-RSV2-08/'+n for n in ['candidate_validation.json','receipt.json','generated/source_provenance.json']]),
 'hotspot':(HOT,['experiments/rsv2_07/'+n for n in ['compound.rs','generate.py','main.rs','diagnostic.rs','hotspot.py']]+['target/rsv2-07/v3/hotspot_strict_summary.json']+['target/rsv2-07/v3/hotspot_strict_3/'+n for n in ['seed.csv','result.json','invocation.json']])}
manifest={}
for lane,(base,names) in inputs.items():
 for name in names:
  dest=OUT/'inputs'/lane/name;dest.parent.mkdir(parents=True,exist_ok=True)
  if dest.exists():data=dest.read_bytes()
  else:
   data=(base/name).read_bytes();dest.write_bytes(data)
  manifest[str(base/name)]={'snapshot':str(dest.relative_to(OUT)),'sha256':hashlib.sha256(data).hexdigest()}
(OUT/'input_provenance.json').write_text(json.dumps(manifest,indent=2)+'\n')
neck=OUT/'inputs/neck/experiments/G16-06-RSV2-08';hot=OUT/'inputs/hotspot/experiments/rsv2_07'
compound=(neck/'compound.rs').read_text();hotlaw=(hot/'compound.rs').read_text()
compound=compound.replace('pub heel_runout:f64}','pub heel_runout:f64,pub point_lateral:f64}')
compound=compound.replace('heel_runout:e("STUDY_HEEL_RUNOUT",5.)}', 'heel_runout:e("STUDY_HEEL_RUNOUT",5.),point_lateral:e("STUDY_POINT_LATERAL",1.)}')
a=hotlaw.index('    if c.point_shape==3 {');b=hotlaw.index('    } else if c.point_shape==2 {',a)
compound=compound.replace('    if c.point_shape==2 {',hotlaw[a:b]+'    } else if c.point_shape==2 {',1)
start='    let u=axis(p);let delta=sub(q,p.tip);';end='\n    let end=if c.point_shape>0'
a=compound.index(start);b=compound.index(end,a);c=hotlaw.index(start);d=hotlaw.index(end,c)
compound=compound[:a]+hotlaw[c:d]+compound[b:]
(OUT/'compound.rs').write_text(compound)
(OUT/'diagnostic.rs').write_bytes((neck/'diagnostic.rs').read_bytes())
main=(neck/'main.rs').read_text().replace('"prototype_only":true,','"prototype_only":true,"cpu_affinity":fs::read_to_string("/proc/self/status").unwrap().lines().find(|l|l.starts_with("Cpus_allowed_list:")).unwrap(),"cargo_build_jobs":std::env::var("CARGO_BUILD_JOBS").ok(),"window_count":p.n_win,"window_area_mm2":p.win_area,')
main=main.replace('    let mut doc=json!', '    assert_eq!(std::env::var("CARGO_BUILD_JOBS").as_deref(),Ok("2"));\n    let mut doc=json!',1)
(OUT/'main.rs').write_text(main)
# Generate the neck witness construction from local production sources using its exact generator.
gen=(neck/'generate.py').read_text();a=gen.index('src=(root/');b=gen.index("provenance={'commit'")
body=gen[a:b]
exec(compile(body,'snapshotted_neck_generate.py','exec'),{'root':ROOT,'out':OUT})
files=list((ROOT/'crates').rglob('*.rs'))+list((ROOT/'requirements').glob('*.toml'))+[ROOT/'Cargo.lock',ROOT/'Cargo.toml',ROOT/'rust-toolchain.toml']
provenance={'commit':'21425fe2948ab3941eedbe9710ebc3d29427faae','production_sha256':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in files},'inputs':manifest,'execution':{'cpus':[10,11],'CARGO_BUILD_JOBS':2}}
(OUT/'source_provenance.json').write_text(json.dumps(provenance,indent=2)+'\n')
rows=list(csv.reader((neck/'candidate.csv').open()));rr=[x[:] for x in rows]
rr[1][7]=str(float(rr[1][7])+.3);rr[1][8]=str(float(rr[1][8])+.2)
with (OUT/'combined.csv').open('w') as f:csv.writer(f,lineterminator='\n').writerows(rr)
(OUT/'neck_control.csv').write_bytes((neck/'candidate.csv').read_bytes())
(OUT/'hotspot_control.csv').write_bytes((OUT/'inputs/hotspot/target/rsv2-07/v3/hotspot_strict_3/seed.csv').read_bytes())
config=json.loads((neck/'candidate_config.json').read_text());config.update(STUDY_POINT_SHAPE='3',STUDY_POINT_LATERAL='2',JARAMBIT_SOLVER='faer')
(OUT/'combined_config.json').write_text(json.dumps(config,indent=2)+'\n')
