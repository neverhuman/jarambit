"""Run the existing CAD predicates on STEP import with zero shape-repair operations."""
from pathlib import Path
import hashlib
import json
import os
import runpy
import shutil
import sys
import time

ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
os.sched_setaffinity(0,{14,15});os.setpriority(os.PRIO_PROCESS,0,10)
folder=Path(sys.argv[1]).resolve()
source=folder/'cad';out=folder/'cad_as_written';out.mkdir()
inputs={}
for name in ['candidate.step','candidate.stl','surface.json','export.json']:
    shutil.copy2(source/name,out/name)
    inputs[name]=sha(source/name);assert sha(out/name)==inputs[name]
from OCP.Interface import Interface_Static
from OCP.STEPControl import STEPControl_Reader
import OCP

_initialize=STEPControl_Reader()
keys=['read.step.resource.name','read.step.sequence','read.precision.mode',
      'read.precision.val','read.maxprecision.mode','read.maxprecision.val']
before={k:Interface_Static.CVal_s(k) for k in keys}
# OCCT's supported resource mechanism: load the explicit sequence from the
# Interface_Static map. Whitespace tokenizes to zero operators. The upstream
# ProcessShape implementation returns the original shape for this empty list.
sequence='Phase6AsWritten'
assert Interface_Static.Init_s('phase6',sequence+'.exec.op','t',' ')
assert Interface_Static.SetCVal_s('read.step.resource.name','')
assert Interface_Static.SetCVal_s('read.step.sequence',sequence)
assert Interface_Static.CVal_s(sequence+'.exec.op')==' '
configured={k:Interface_Static.CVal_s(k) for k in keys}
assert all(configured[k]==before[k] for k in keys if k not in keys[:2])
record={'source_inputs_sha256':inputs,'reader_parameters_before':before,
    'reader_parameters_used':configured,'shape_processing_sequence':sequence,
    'shape_processing_operators':[],'tolerances_unchanged':True,'OCP':OCP.__version__,
    'cpu_affinity':sorted(os.sched_getaffinity(0)),'nice':os.getpriority(os.PRIO_PROCESS,0),
    'CARGO_BUILD_JOBS':os.environ['CARGO_BUILD_JOBS'],
    'runner_sha256':sha(Path(__file__)),'unchanged_check_script_sha256':sha(HERE/'phase6_cad_check.py'),
    'upstream_configuration_source':'https://github.com/Open-Cascade-SAS/OCCT/blob/V7_8_1/src/XSAlgo/XSAlgo_AlgoContainer.cxx',
    'upstream_empty_sequence_source':'https://github.com/Open-Cascade-SAS/OCCT/blob/V7_8_1/src/ShapeProcess/ShapeProcess.cxx'}
(out/'reader_configuration.json').write_text(json.dumps(record,indent=2)+'\n')
sys.argv=[str(HERE/'phase6_cad_check.py'),str(out)]
start=time.time()
runpy.run_path(sys.argv[0],run_name='__main__')
assert {k:Interface_Static.CVal_s(k) for k in keys}==configured
for name,h in inputs.items():assert sha(source/name)==sha(out/name)==h
record['wall_seconds']=time.time()-start
record['result_sha256']=sha(out/'cad_check.json')
record['predicate_script_unchanged']=True
(out/'reader_configuration.json').write_text(json.dumps(record,indent=2)+'\n')
print('AS_WRITTEN_CHECK_COMPLETE',out,flush=True)
