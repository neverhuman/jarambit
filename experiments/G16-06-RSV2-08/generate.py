from pathlib import Path
import hashlib,json
root=Path(__file__).resolve().parents[2]
out=root/'target/G16-06-RSV2-08/generated'
out.mkdir(parents=True,exist_ok=True)
src=(root/'crates/crescent/src/export.rs').read_text()
start=src.index('pub fn thickness_at(')
end=src.index('\npub fn solid(',start)
original=src[start:end].replace('pub fn thickness_at(','pub fn baseline_thickness_at(',1)
replacement=original+'\npub fn thickness_at(p: &Pheno, q: P2) -> f64 { crate::compound::thickness_at(p,q) }\n'
src=src[:start]+replacement+src[end:]
src=src.replace('let outer=dp_closed(&p.outline,0.03);','let outer=dp_closed(&p.outline,crate::compound::config().boundary_tol);')
src=src.replace('max_area.min(0.25)','max_area.min(crate::compound::config().export_area)')
src=src.replace('let m=fine::mesh_domain(&outer,&holes,','let m=crate::compound::export_mesh(p,&outer,&holes,')
(out/'prototype_export.rs').write_text(src)
files=['genome','pheno','fem','gates','aesthetic','certification','geom','c4_ref','c5_ref','c6_ref','step_export']
fem_src=(root/'crates/crescent/src/fem.rs').read_text()
marker='    let hm_rb = nod_rb.as_ref().map(|v| cap(pk6(v, true))).unwrap_or(0.0);'
assert fem_src.count(marker)==1
fem_src=fem_src.replace(marker,marker+'\n    crate::diagnostic::record(&t6, nod_sp.as_deref(), nod_rb.as_deref(), tp, sx0);\n    crate::diagnostic::record_sequence(&cen,&excl,&lat,&twist,p.heel.0,&te,&t6,&nod_pr,&ring6,tp);')
(out/'prototype_fem.rs').write_text(fem_src)
pheno_src=(root/'crates/crescent/src/pheno.rs').read_text()
slot='taper_slot(&comet_c, g.v[COMET_W0], g.v[COMET_W1])'
assert pheno_src.count(slot)==1
pheno_src=pheno_src.replace(slot,'crate::compound::comet_slot(&comet_c, g.v[COMET_W0], g.v[COMET_W1])')
slot_push='holes.push(crate::compound::comet_slot(&comet_c, g.v[COMET_W0], g.v[COMET_W1])); hole_names.push("comet");'
assert pheno_src.count(slot_push)==1
pheno_src=pheno_src.replace(slot_push,'''let comet=crate::compound::comet_slot(&comet_c, g.v[COMET_W0], g.v[COMET_W1]);
    if !crate::compound::config().neck_solid { holes.push(comet); hole_names.push("comet"); }
    else { assert!(comet.iter().all(|q|q.0<heel.0), "solid-neck variant only omits the neck slot"); }''')
old_clear='            (nn_in.nearest(q, 40.0) - (b - et) / (2.0 * tan_in)).min(nn_sp.nearest(q, 40.0) - (b - et) / (2.0 * tan_sp)) - WIN_RIM'
assert pheno_src.count(old_clear)==1
new_clear='            if crate::compound::config().window_compound {\n                // Use the actual compound-bevel extent; keep the same 3.2 mm window rim.\n                nn_in.nearest(q,40.).min(nn_sp.nearest(q,40.)) - crate::compound::config().d1.max(crate::compound::config().blade_d1) - WIN_RIM\n            } else {\n                (nn_in.nearest(q, 40.0) - (b - et) / (2.0 * tan_in)).min(nn_sp.nearest(q, 40.0) - (b - et) / (2.0 * tan_sp)) - WIN_RIM\n            }'
pheno_src=pheno_src.replace(old_clear,new_clear)
(out/'prototype_pheno.rs').write_text(pheno_src)
modules='\n'.join(f'#[path="{out}/prototype_fem.rs"] pub mod fem;' if f=='fem' else f'#[path="{out}/prototype_pheno.rs"] pub mod pheno;' if f=='pheno' else f'#[path="{root}/crates/crescent/src/{f}.rs"] pub mod {f};' for f in files)
(out/'modules.rs').write_text(modules+'\n')
provenance={'commit':'21425fe2948ab3941eedbe9710ebc3d29427faae','source_sha256':{str(root/'crates/crescent/src'/f'{f}.rs'):hashlib.sha256((root/'crates/crescent/src'/f'{f}.rs').read_bytes()).hexdigest() for f in files+['export']},'requirements_sha256':{str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in (root/'requirements/RS-karambit-v1.toml',root/'requirements/RS-karambit-v2.toml')}}
provenance['prototype_source_sha256']={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in Path(__file__).resolve().parent.iterdir() if p.is_file() and (p.suffix in {'.rs','.py','.sh','.csv'} or p.name in {'input_provenance.json','candidate_config.json'})}
provenance['audit_inputs']=json.loads((Path(__file__).resolve().parent/'input_provenance.json').read_text())
dependency_files=[root/'Cargo.lock',root/'Cargo.toml',root/'rust-toolchain.toml']
for crate in ['physics','genotype','phenotype','redteam','evidence']:
    dependency_files.extend((root/'crates'/crate/'src').rglob('*.rs'))
provenance['dependency_source_sha256']={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in dependency_files}
provenance['execution']={'cpus':[12,13],'CARGO_BUILD_JOBS':2,'CARGO_INCREMENTAL':0,'CARGO_PROFILE_DEV_DEBUG':'line-tables-only'}
(out/'source_provenance.json').write_text(json.dumps(provenance,indent=2)+'\n')

# Keep prototype source files tracked; place all generated sources and binaries in the lane target.
for name in ['main.rs','compound.rs','diagnostic.rs']:
    (out/name).write_text((Path(__file__).resolve().parent/name).read_text())
