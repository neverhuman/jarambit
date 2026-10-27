//! Release tool (v3.0.0): tip-FE mesh convergence (max area 0.4 / 0.15 production / 0.08 mm^2) of archived designs.
//!   g5-tipconv <root> <id,id,...>
use jarambit_g5::g5_genome::G5Genome;
use jarambit_g5::g5_sandwich::solve_g5_solid_tet10_strict;
use jarambit_g5::g5_sdf::{build_blade_profile_2d, eval_blade_thickness_and_sdf2};
use std::{fs, path::Path};
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let mut rows = std::collections::HashMap::new();
    for isl in fs::read_dir(Path::new(&a[1])).unwrap().flatten() {
        if let Ok(t) = fs::read_to_string(isl.path().join("run/genomes_g5.csv")) {
            for l in t.lines().skip(1) { let (d, r) = l.split_once(',').unwrap(); rows.insert(d.to_string(), r.to_string()); }
        }
    }
    for id in a[2].split(',') {
        let g = G5Genome::from_csv_row(&rows[id]).unwrap();
        let p = g.decode();
        let b = build_blade_profile_2d(&p);
        let mut outer = Vec::new();
        for i in 0..b.xs.len() { outer.push((b.xs[i], b.y_in[i])); }
        for i in (0..b.xs.len()).rev() { outer.push((b.xs[i], b.y_sp[i])); }
        let th = |x: f64, y: f64| { let (t, d2) = eval_blade_thickness_and_sdf2(&p, &b, x, y); (d2 <= 0.0 && t > 0.05).then_some(t) };
        let mut out = Vec::new();
        for ar in [0.4, 0.15, 0.08] {
            let m = jarambit_physics::fine::mesh_domain(&outer, &[], ar).unwrap();
            let r = solve_g5_solid_tet10_strict(m.xy, m.tris, b.tip_xy, p["t_core"], b.x_tip - 32.0, b.tip_xy.1 - 32.0, 0.5, 130, 130, &th, 0).map(|r| r.sf_nodal_max).unwrap_or(f64::NAN);
            out.push(r);
        }
        println!("{id}: sf 0.4={:.3} 0.15={:.3} 0.08={:.3} rel(0.4,0.15)={:.3} rel(0.15,0.08)={:.3}", out[0], out[1], out[2], (out[0]-out[1]).abs()/out[1], (out[1]-out[2]).abs()/out[2]);
    }
}
