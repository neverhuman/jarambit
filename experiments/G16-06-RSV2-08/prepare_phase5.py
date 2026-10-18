#!/usr/bin/env python3
"""Freeze the assigned Q7 law and build lane-local observation adapters."""
from pathlib import Path
import hashlib
import json
import os
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
OUT = ROOT/'target/G16-06-RSV2-08/phase5'
SOURCE = Path('/home/ubuntu/jarambit_wt_diag/target/gate-feasibility-bound/unified_study_v5')
SNAP = HERE/'phase5_sources'

def digest(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

if __name__ == '__main__':
    assert os.sched_getaffinity(0) == {12,13}
    assert os.getpriority(os.PRIO_PROCESS,0) >= 10
    assert os.environ['CARGO_BUILD_JOBS'] == '2'
    assert not SNAP.exists(), 'preserve existing Phase-5 source snapshots'
    SNAP.mkdir(); OUT.mkdir(parents=True,exist_ok=True)
    provenance=json.loads((SOURCE/'source_provenance.json').read_text())
    for rel,want in provenance['production_sha256'].items():
        assert digest(ROOT/rel)==want,rel
    inputs={}
    for name in ['q7.csv','compound.rs','pocket.rs','prototype_pheno.rs','prototype_fem.rs',
                 'prototype_export.rs','diagnostic.rs','main.rs','source_provenance.json',
                 'inputs/q7_control_case.json','q7_control_fem.json']:
        p=SOURCE/name; dest=SNAP/p.name
        dest.write_bytes(p.read_bytes())
        inputs[str(p)]={'snapshot':str(dest.relative_to(ROOT)),'sha256':digest(dest)}
    native=(ROOT/'crates/crescent/src/fem.rs').read_text()
    observed=(SNAP/'prototype_fem.rs').read_text()
    hooks=['    crate::diagnostic::record(&t6, nod_sp.as_deref(), nod_rb.as_deref(), tp, sx0);\n',
           '    crate::diagnostic::record_sequence(&cen,&excl,&lat,&twist,p.heel.0,&te,&t6,&nod_pr,&ring6,tp,p);\n']
    for hook in hooks:
        assert observed.count(hook)==1
        observed=observed.replace(hook,'')
    assert observed==native,'imported FEM contains more than observation hooks'
    modules=[]
    for name in ['genome','pheno','fem','gates','aesthetic','certification','geom','c4_ref','c5_ref','c6_ref','step_export']:
        path=SNAP/f'prototype_{name}.rs' if name in ['pheno','fem'] else ROOT/f'crates/crescent/src/{name}.rs'
        modules.append(f'#[path="{path}"] pub mod {name};')
    (SNAP/'modules.rs').write_text('\n'.join(modules)+'\n')
    adapter=(HERE/'phase2.rs').read_text()
    adapter=adapter.replace('pub mod diagnostic;','pub mod diagnostic;\npub mod pocket;')
    adapter=adapter.replace('#[path = "../../../crates/crescent/src/gci.rs"]',f'#[path = "{ROOT}/crates/crescent/src/gci.rs"]')
    adapter=adapter.replace('"skyline"','"faer"').replace('G16-06-RSV2-08-p2','G16-06-RSV2-08-p5')
    adapter=adapter.replace('include_bytes!("../../../crates/crescent/src/gci.rs")',f'include_bytes!("{ROOT}/crates/crescent/src/gci.rs")')
    (SNAP/'native_adapter.rs').write_text(adapter)
    local=dict(task='G16-06-RSV2-08-p5',head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
               production_sha256=provenance['production_sha256'],inputs=inputs,
               local_source_sha256={str(p.relative_to(ROOT)):digest(p) for p in SNAP.iterdir()
                                    if p.name!='source_provenance.json'},
               execution=dict(cpus=[12,13],cargo_build_jobs=2,nice=os.getpriority(os.PRIO_PROCESS,0)),
               native_fem_only_two_observation_hooks=True)
    # The imported provenance is retained; the executable embeds this lane's provenance.
    (OUT/'source_provenance.json').write_text(json.dumps(local,indent=2)+'\n')
    for p in SNAP.iterdir():
        if p.suffix=='.rs':(OUT/p.name).write_bytes(p.read_bytes())
    # Relative module includes resolve in the target directory; explicit paths bind immutable snapshots.
    print(json.dumps(dict(source_snapshots=len(inputs),native_sources=len(local['production_sha256']),fem_identity='pass')))
