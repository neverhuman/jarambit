//! Supply native plate context and the exact saved CAD upper facets to tip_solid.py.
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
    let (_f, fl) = fem::report_fields(&p, 0.25).expect("native plate reference");
    // A genuine compound-law raster supplies the legacy plate context. The solid
    // explicitly uses the saved skin heights, rather than this interpolation.
    let x0 = p.outline.iter().map(|q| q.0).fold(f64::INFINITY, f64::min).floor();
    let y0 = p.outline.iter().map(|q| q.1).fold(f64::INFINITY, f64::min).floor();
    let nx = (p.outline.iter().map(|q| q.0).fold(f64::NEG_INFINITY, f64::max) - x0).ceil() as usize + 1;
    let ny = (p.outline.iter().map(|q| q.1).fold(f64::NEG_INFINITY, f64::max) - y0).ceil() as usize + 1;
    let th: Vec<_> = (0..ny).flat_map(|j| (0..nx).map(move |i| (i, j))).map(|(i,j)| {
        let q = (x0 + i as f64 + 0.5, y0 + j as f64 + 0.5);
        if geom::point_in_poly(q, &p.outline) && !p.holes.iter().any(|h| geom::point_in_poly(q, h)) {
            Some(compound::thickness_at(&p,q))
        } else { None }
    }).collect();
    let doc = json!({"prototype_only": true, "candidate": args[1], "compound_config": compound::config(),
        "metrics": {"t_joint": p.t_joint}, "outline": p.outline, "holes": p.holes,
        "mesh": {"xy": fl.xy, "tris": fl.tris, "te": fl.te, "twist": fl.twist},
        "field": {"x0": x0, "y0": y0, "g": 1.0, "nx": nx, "ny": ny, "th": th},
        "native_tip": p.tip});
    let out = Path::new(&args[3]);
    fs::write(out.join("critical_fem_3_D1.json"), serde_json::to_vec(&doc).unwrap()).unwrap();
    fs::write(out.join("critical_fem_3_grind_law.json"), serde_json::to_vec(&json!({
        "prototype_only": true, "edges": [{"samples": [[p.tip.0, p.tip.1]]}],
        "purpose": "legacy tip-coordinate reader; actual geometry is export_surface"})).unwrap()).unwrap();
    eprintln!("saved native context; tip {:?}; joint {} mm", p.tip, p.t_joint);
}
