#![allow(dead_code,unused_imports)]
include!("/home/ubuntu/jarambit/.agent/worktrees/lane-G16-06-RSV2-08/experiments/G16-06-RSV2-08/phase5_sources/modules.rs");
#[path="/home/ubuntu/jarambit/.agent/worktrees/lane-G16-06-RSV2-08/experiments/G16-06-RSV2-08/phase5_sources/prototype_export.rs"] pub mod export;
#[path="/home/ubuntu/jarambit/.agent/worktrees/lane-G16-06-RSV2-08/experiments/G16-06-RSV2-08/phase7_adapter.rs"] pub mod compound;
#[path="/home/ubuntu/jarambit/.agent/worktrees/lane-G16-06-RSV2-08/experiments/G16-06-RSV2-08/phase5_sources/diagnostic.rs"] pub mod diagnostic;
#[path="/home/ubuntu/jarambit/.agent/worktrees/lane-G16-06-RSV2-08/experiments/G16-06-RSV2-08/phase5_sources/pocket.rs"] pub mod pocket;
use geom::*;
use serde_json::json;
use jarambit_physics::fine;
use std::fs;
fn hash(b:&[u8])->String {jarambit_evidence::identity::ContentRef::from_bytes("phase7","rev1",b).sha256}
fn main(){
    let args:Vec<_>=std::env::args().collect();
    let csv=fs::read(&args[1]).unwrap();let text=std::str::from_utf8(&csv).unwrap();
    let g=text.lines().find_map(|l|genome::Genome::from_csv(&l.split(',').collect::<Vec<_>>())).unwrap();
    let mut canonical=g.clone();canonical.clamp();assert_eq!(g.v,canonical.v);
    let p=pheno::build(&g);let outer=dp_closed(&p.outline,0.03);
    let holes:Vec<Vec<P2>>=p.holes.iter().map(|h|dp_closed(h,0.02)).collect();
    let mut levels=Vec::new();
    for area in [0.25,1.,4.] {
        let mesh=fine::mesh_domain(&outer,&holes,area).unwrap();
        // Identical native FEM four-point element-thickness quadrature, no solve.
        let te: Vec<f64> = mesh.tris.iter().map(|t| {
            let (a, b, c) = (mesh.xy[t[0]], mesh.xy[t[1]], mesh.xy[t[2]]);
            let cen = ((a.0 + b.0 + c.0) / 3.0, (a.1 + b.1 + c.1) / 3.0);
            let pts = [cen, ((a.0 + cen.0) / 2.0, (a.1 + cen.1) / 2.0), ((b.0 + cen.0) / 2.0, (b.1 + cen.1) / 2.0), ((c.0 + cen.0) / 2.0, (c.1 + cen.1) / 2.0)];
            pts.iter().map(|&q| crate::export::thickness_at(&p, q)).sum::<f64>() / 4.0
        }).collect();
        let mut xytr=Vec::new();
        for &(x,y) in &mesh.xy {xytr.extend(x.to_le_bytes());xytr.extend(y.to_le_bytes());}
        for t in &mesh.tris {for &i in t {xytr.extend((i as u64).to_le_bytes());}}
        let mut tb=Vec::new();for &t in &te {tb.extend(t.to_le_bytes());}
        let mut all=xytr.clone();all.extend(&tb);
        levels.push(json!({"area":area,"xy_triangles_sha256":hash(&xytr),"thickness_sha256":hash(&tb),"mesh_thickness_sha256":hash(&all),"n_node":mesh.xy.len(),"n_elem":mesh.tris.len(),"min_mm":te.iter().copied().fold(f64::INFINITY,f64::min),"max_mm":te.iter().copied().fold(f64::NEG_INFINITY,f64::max)}));
    }
    fs::write(&args[2],serde_json::to_string_pretty(&json!({"genome_sha256":hash(&csv),"levels":levels,"law_sha256":hash(include_bytes!("/home/ubuntu/jarambit/.agent/worktrees/lane-G16-06-RSV2-08/experiments/G16-06-RSV2-08/phase7_adapter.rs")),"native_fem_quadrature":true})).unwrap()).unwrap();
}
