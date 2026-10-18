#![allow(dead_code, unused_imports)]
include!("modules.rs");
#[path="prototype_export.rs"] pub mod export;
pub mod compound;
pub mod diagnostic;
use std::{fs, path::Path};
use serde::Serialize;

#[derive(Serialize)]
struct Mesh<'a> {
    prototype_only: bool,
    units: &'static str,
    config: &'a compound::Config,
    genes: &'a [f64],
    verts: &'a [(f64, f64, f64)],
    tris: &'a [[usize; 3]],
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let text = fs::read_to_string(&args[1]).unwrap();
    let g = text.lines().find_map(|l| genome::Genome::from_csv(&l.split(',').collect::<Vec<_>>())).unwrap();
    let mut canonical = g.clone(); canonical.clamp();
    assert_eq!(g, canonical);
    let p = pheno::build(&g);
    let s = export::solid(&p, 1.).expect("solid export");
    assert_eq!(export::census(&s.tris), (0, 0, 1));
    let out = Path::new(&args[2]); fs::create_dir_all(out).unwrap();
    fs::write(out.join("prototype.stl"), export::stl(&s, "RSV2_07_PROTOTYPE")).unwrap();
    let mesh = Mesh {prototype_only: true, units: "mm", config: compound::config(), genes: &g.v, verts: &s.verts, tris: &s.tris};
    serde_json::to_writer(fs::File::create(out.join("prototype.mesh.json")).unwrap(), &mesh).unwrap();
    let metadata = serde_json::json!({"prototype_only": true, "units": "mm", "config": compound::config(),
        "vertices": s.verts.len(), "triangles": s.tris.len(), "census": export::census(&s.tris),
        "volume_mm3": export::volume(&s)});
    fs::write(out.join("metadata.json"), serde_json::to_string_pretty(&metadata).unwrap()).unwrap();
}
