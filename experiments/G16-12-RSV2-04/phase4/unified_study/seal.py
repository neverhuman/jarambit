from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,os,subprocess
OUT=Path(__file__).resolve().parent;ROOT=OUT.parents[2]
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
assert os.sched_getaffinity(0)=={10,11} and os.environ['CARGO_BUILD_JOBS']=='2'
deps={}
for name in ['jarambit_genotype','jarambit_phenotype','jarambit_physics','jarambit_redteam','jarambit_evidence','serde','serde_json','spade']:
 paths=list((ROOT/'target/release/deps').glob('lib'+name+'-*.rlib'));assert len(paths)==1;deps[str(paths[0].relative_to(ROOT))]=sha(paths[0])
prior=json.loads((OUT.parent/'compound_study/run_provenance.json').read_text())['dependencies_sha256'];assert deps==prior
prov={'sealed_utc':datetime.now(timezone.utc).isoformat(),'commit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'branch':subprocess.check_output(['git','branch','--show-current'],cwd=ROOT,text=True).strip(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),'cpu_affinity':sorted(os.sched_getaffinity(0)),'CARGO_BUILD_JOBS':2,'solver':'faer','dependency_rlib_sha256':deps,'dependency_digests_match_retained_phase2':True,'study_binary_sha256':sha(OUT/'study'),'completed_combined_fem_evaluations':12,'completed_witness_control_evaluations':5,'completed_export_measurements':4,'additional_partial_skyline_observations':len(json.loads((OUT/'neck_skyline_partial.json').read_text())['levels']),'source_snapshots_used_for_reproduction':True,'report_sha256':sha(OUT.parent/'UNIFIED_PROTOTYPE_REPORT.md'),'reproduction':'taskset -c 10,11 env CARGO_BUILD_JOBS=2 bash target/gate-feasibility-bound/unified_study/reproduce.sh','prior_report_sha256':{n:sha(OUT.parent/n) for n in ['REPORT.md','COMPOUND_BEVEL_REPORT.md','WIDE_TIP_REPORT.md']}}
(OUT/'run_provenance.json').write_text(json.dumps(prov,indent=2)+'\n')
(OUT/'REPORT_SHA256').write_text(prov['report_sha256']+'  ../UNIFIED_PROTOTYPE_REPORT.md\n')
files=[OUT.parent/'UNIFIED_PROTOTYPE_REPORT.md']+[p for p in sorted(OUT.rglob('*')) if p.is_file() and p.name!='SHA256SUMS' and '__pycache__' not in p.parts and 'compiler-tmp' not in p.parts]
(OUT/'SHA256SUMS').write_text(''.join(f'{sha(p)}  {os.path.relpath(p,OUT)}\n' for p in files))
print('sealed',len(files),'artifacts; report SHA-256',prov['report_sha256'])
