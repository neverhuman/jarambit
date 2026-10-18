#!/usr/bin/env python3
"""Bind the authorized native taper/fullers to Q7 without editing production code."""
import json
import os
from prepare_phase5 import HERE, ROOT, SNAP, digest
OUT=ROOT/'target/G16-06-RSV2-08/phase7'

if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10
    assert os.environ['CARGO_BUILD_JOBS']=='2'
    OUT.mkdir(parents=True,exist_ok=True)
    old=json.loads((OUT.parent/'phase5/source_provenance.json').read_text())
    for rel,want in old['production_sha256'].items():assert digest(ROOT/rel)==want,rel
    section=(SNAP/'compound.rs').read_text()
    section=section.replace('let core=if c.core>0.', '''// Native pheno::thickness_field distal taper, applied to the Q7 core.
    let maxx=p.outline.iter().map(|q|q.0).fold(f64::NEG_INFINITY,f64::max);
    let tx0=g.v[TAPER_X0]+p.dx;
    let taper=(g.v[T]-g.v[T_TIP])*((q.0-tx0)/(maxx-tx0)).clamp(0.,1.);
    let core=if c.core>0.''')
    section=section.replace('let blade_band=', 'let blade_band=')
    section=section.replace('neck+(c.blade_core-neck)*smooth', 'neck+(c.blade_core-taper-neck)*smooth')
    section=section.replace('let core=if c.core>0. {c.core}else{g.knot_t(q.0.min(g.v[TAPER_X0]+p.dx),g.v[TAPER_X0]+p.dx)};',
        'let core=(if c.core>0. {c.core}else{g.knot_t(q.0.min(g.v[TAPER_X0]+p.dx),g.v[TAPER_X0]+p.dx)})-taper;')
    section=section.replace('let edge=if d>=d1 {base}else{', '''// Native two-sided parabolic fuller, protected from the Q7 grind,
    // holder land and brand pad, and subject to the native web floor.
    let pad=p.pad;
    let protected=(q.0>=pad[0].0-1. && q.0<=pad[1].0+1. && q.1>=pad[0].1-1. && q.1<=pad[2].1+1.)
        || (q.0>=78.+p.dx && q.0<=100.+p.dx);
    let removal=if !protected && d>=d1 && q.0>p.heel.0-5. {native_fuller(p,q)}else{0.};
    let base=(base-removal).max((0.3*g.v[T]).max(1.5).min(base));
    let edge=if d>=d1 {base}else{''')
    native=(ROOT/'crates/crescent/src/pheno.rs').read_text()
    block=native.split('    // fuller centreline: midpoints between inside edge and spine over [s0, s1] of the inside edge\n')[1].split('    let (rd, rw, rq)')[0]
    # Centreline construction is copied verbatim from native pheno, not redesigned.
    fuller='''
/// Native fuller centreline and two-face parabolic subtraction. One phenotype per process.
pub fn native_fuller(p:&Pheno,q:P2)->f64 {
    static F:OnceLock<Option<Nn>>=OnceLock::new();
    let g=&p.genome;
    let nn=F.get_or_init(|| {
'''+block+'''        nn_ful
    });
    let (fd,fw)=(g.v[FUL_D],g.v[FUL_W]);
    if let Some(nf)=nn {let d=nf.nearest(q,fw/2.);if d<fw/2. {return 2.*fd*(1.-(2.*d/fw).powi(2));}}
    0.
}
'''
    section+=fuller
    adapter=HERE/'phase7_adapter.rs'
    if adapter.exists():assert adapter.read_text()==section,'preserve adapter source'
    else:adapter.write_text(section)
    for name in ['modules.rs','prototype_export.rs','diagnostic.rs','pocket.rs']:
        (OUT/name).write_bytes((SNAP/name).read_bytes())
    main=(SNAP/'main.rs').read_text().replace('pub mod compound;',f'#[path="{adapter}"] pub mod compound;')
    (OUT/'main.rs').write_text(main)
    native_adapter=(HERE/'phase6_adapter.rs').read_text().replace(str(SNAP/'compound.rs'),str(adapter)).replace('G16-06-RSV2-08-p6','G16-06-RSV2-08-p7').replace(str(OUT.parent/'phase5/source_provenance.json'),str(OUT/'source_provenance.json'))
    (HERE/'phase7_native.rs').write_text(native_adapter)
    (OUT/'native_adapter.rs').write_text(native_adapter)
    paths=[adapter,HERE/'phase7_native.rs',HERE/'prepare_phase7.py',OUT/'main.rs',OUT/'modules.rs',OUT/'prototype_export.rs',OUT/'diagnostic.rs',OUT/'pocket.rs']
    prov=dict(task='G16-06-RSV2-08-p7',production_sha256=old['production_sha256'],
        inherited_phase5_source_sha256=old['local_source_sha256'],
        local_source_sha256={str(p.relative_to(ROOT)):digest(p) for p in paths},
        native_fuller_centreline_source=block,native_fem_only_two_observation_hooks=True,
        section_change='native distal taper and native two-face fuller subtraction on Q7 core; Q7 grind protection; native web floor; common FEM/export law')
    (OUT/'source_provenance.json').write_text(json.dumps(prov,indent=2)+'\n')
    print('Phase7 source prepared; native production sources unchanged:',len(old['production_sha256']))
