//! Production export meshing with the recorded study's compound thickness law.
#![allow(dead_code, unused_imports)]
include!("modules.rs");
#[path = "prototype_export.rs"]
pub mod export;
pub mod compound;
use serde_json::json;
use std::{env, fs, path::Path};

fn main() {
    let args: Vec<_> = env::args().collect();
    let text = fs::read_to_string(&args[1]).unwrap();
    let g = text.lines().find_map(|l| genome::Genome::from_csv(&l.split(',').collect::<Vec<_>>())).unwrap();
    let mut canonical = g.clone();
    canonical.clamp();
    assert_eq!(g.v, canonical.v);
    let p = pheno::build(&g);
    let s = export::solid(&p, 1.0).expect("solid");
    assert_eq!(export::census(&s.tris), (0, 0, 1));
    let mut surface = export::surface_json(&s);
    surface["tip"] = json!(p.tip);
    surface["rs_v2"] = json!(p.rs_v2);
    surface["prototype_only"] = json!(true);
    surface["compound_config"] = json!(compound::config());
    surface["genes"] = json!(g.v);
    surface["export_call_max_area_mm2"] = json!(1.0);
    let out = Path::new(&args[2]);
    fs::write(out.join("critical_fem_3_surface_1mm.json"), serde_json::to_vec(&surface).unwrap()).unwrap();
    let metadata = json!({"export_call": "export::solid(&p, 1.0)",
        "production_rs_v2_area_cap_mm2": 0.25, "production_outer_dp_tolerance_mm": 0.03,
        "uses_study_grid_points": false, "compound_config": compound::config(),
        "volume_mm3": export::volume(&s), "census": export::census(&s.tris),
        "solid_vertices": s.verts.len(), "solid_triangles": s.tris.len(),
        "upper_triangles": surface["tris"].as_array().unwrap().len()});
    fs::write(out.join("surface_metadata.json"), serde_json::to_vec_pretty(&metadata).unwrap()).unwrap();
    eprintln!("{metadata}");
}
