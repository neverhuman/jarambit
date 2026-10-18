#!/usr/bin/env python3
"""Build Phase8 against current sources and the unchanged Phase7 section law."""
import json,os,re
from prepare_phase5 import HERE,ROOT,SNAP,digest
OUT=ROOT/'target/G16-06-RSV2-08/phase8'
if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10
    assert os.environ['CARGO_BUILD_JOBS']=='2'
    OUT.mkdir(parents=True,exist_ok=True)
    old=json.loads((SNAP/'source_provenance.json').read_text())['production_sha256']
    changed=[rel for rel,w in old.items() if digest(ROOT/rel)!=w]
    assert set(changed)<={'crates/cli/src/auto_evolve.rs'},changed
    modules=[]
    for name in ['genome','pheno','fem','gates','aesthetic','certification','geom','c4_ref','c5_ref','c6_ref','step_export']:
        path=SNAP/f'prototype_{name}.rs' if name in ['pheno','fem'] else ROOT/f'crates/crescent/src/{name}.rs'
        modules.append(f'#[path="{path}"] pub mod {name};')
    (OUT/'modules.rs').write_text('\n'.join(modules)+'\n')
    for name in ['prototype_export.rs','diagnostic.rs','pocket.rs']:(OUT/name).write_bytes((SNAP/name).read_bytes())
    (OUT/'main.rs').write_text((SNAP/'main.rs').read_text().replace('pub mod compound;',f'#[path="{HERE}/phase7_adapter.rs"] pub mod compound;'))
    adapter=(HERE/'phase7_native.rs').read_text().replace('G16-06-RSV2-08-p7','G16-06-RSV2-08-p8')
    adapter=re.sub(r'include!\("[^"]+/modules.rs"\)',f'include!("{OUT}/modules.rs")',adapter)
    adapter=adapter.replace('/phase7/source_provenance.json','/phase8/source_provenance.json')
    (OUT/'native_adapter.rs').write_text(adapter)
    sources=[HERE/'phase7_adapter.rs',HERE/'prepare_phase8.py']+[OUT/p for p in ['main.rs','native_adapter.rs','modules.rs','prototype_export.rs','diagnostic.rs','pocket.rs']]
    prov=dict(task='G16-06-RSV2-08-p8',production_sha256={rel:digest(ROOT/rel) for rel in old},
        inherited_phase5_source_sha256={str(p.relative_to(ROOT)):digest(p) for p in SNAP.iterdir() if p.is_file()},
        local_source_sha256={str(p.relative_to(ROOT)):digest(p) for p in sources},
        native_source_changes_since_phase7=changed,section_law_sha256=digest(HERE/'phase7_adapter.rs'),native_fem_only_two_observation_hooks=True)
    (OUT/'source_provenance.json').write_text(json.dumps(prov,indent=2)+'\n')
    print(json.dumps(dict(native_sources=len(old),source_changes_since_phase7=changed)))
