#!/usr/bin/env python3
"""One non-solving field probe alongside one serial export/A4 evaluation."""
import json
import os
from finish_phase7 import probe
from prepare_phase5 import HERE, ROOT, digest
from run_phase7 import OUT, milestone

if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10
    assert os.environ['CARGO_BUILD_JOBS']=='2'
    r=json.loads((HERE/'phase7_receipt.json').read_text())
    assert r['completed_cases']==144 and r['state'] in ['export_a4','complete']
    records={}
    for i,row in enumerate(r['results']+r['law_effect_controls'],1):
        path=probe(row);records[str(path.relative_to(ROOT))]=digest(path)
        print('field_binding',i,row['name'],flush=True)
    out=OUT/'field_binding_batch.json';assert not out.exists()
    out.write_text(json.dumps(dict(cpus=[12,13],nice=os.getpriority(os.PRIO_PROCESS,0),cargo_build_jobs=2,
        max_concurrent_field_probes=1,concurrency_note='one probe alongside one serial export/A4; total at most2 evaluations',
        field_probe_sha256=digest(OUT/'field_probe'),records=records),indent=2)+'\n')
    milestone('native quadrature field probes complete','- All147 sweep/control inputs re-meshed without solving; split planform/thickness hashes bind exactly to actual native GCI hashes. One probe ran alongside one serial export/A4; CPU12,13/nice/jobs limits preserved. Final read-only audit will independently check fuller and taper effects from these bindings.')
