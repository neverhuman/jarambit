//! Release tool (v3.0.0): the exact blade thickness law of a design sampled on a
//! fine 2D grid, for meshing the blade far finer than the full-knife voxel export (a 0.2 mm voxel cannot carry a
//! 35-45 deg inclusive edge with a 0.05-0.12 mm apex; it aliases into teeth).
//!
//!   bladefield <root[,root]> <design_id> <out_prefix> [g=0.05] [x0=91.5]
//!
//! Writes <out_prefix>.json (grid origin, spacing, shape, design and decoder) and <out_prefix>.f32: two row-major
//! f32 arrays [ny][nx], thickness t(x, y) (mm; 0 outside the blade and inside windows) then the signed planform
//! distance d2(x, y) (mm, negative inside), exactly as `eval_blade_thickness_and_sdf2` returns them. For x >= 92 the
//! full-knife SDF is exactly max(d2, |z| - t/2), so a mesh of that field joins the voxel export without a seam.
use jarambit_g5::g5_genome::G5Genome;
use jarambit_g5::g5_qd::g5_decoder_sha256;
use jarambit_g5::g5_sdf::{build_blade_profile_2d, eval_blade_thickness_and_sdf2};
use std::{fs, io::Write, path::Path};

fn find(roots: &str, id: &str) -> G5Genome {
    for root in roots.split(',') {
        for isl in fs::read_dir(Path::new(root)).expect("root").flatten() {
            let Ok(text) = fs::read_to_string(isl.path().join("run/genomes_g5.csv")) else { continue };
            for line in text.lines().skip(1) {
                if let Some((d, rest)) = line.split_once(',') {
                    if d == id {
                        return G5Genome::from_csv_row(rest).expect("row");
                    }
                }
            }
        }
    }
    panic!("{id} not found")
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let g = find(&a[1], &a[2]);
    let gs: f64 = a.get(4).and_then(|v| v.parse().ok()).unwrap_or(0.05);
    let x0: f64 = a.get(5).and_then(|v| v.parse().ok()).unwrap_or(91.5);
    let p = g.decode();
    let b = build_blade_profile_2d(&p);
    let y_lo = b.y_in.iter().chain(&b.y_sp).cloned().fold(f64::INFINITY, f64::min) - 1.0;
    let y_hi = b.y_in.iter().chain(&b.y_sp).cloned().fold(f64::NEG_INFINITY, f64::max) + 1.0;
    let x1 = b.x_tip + 1.0;
    let (nx, ny) = (((x1 - x0) / gs).ceil() as usize + 1, ((y_hi - y_lo) / gs).ceil() as usize + 1);
    let mut t = vec![0f32; nx * ny];
    let mut d2 = vec![0f32; nx * ny];
    let mut t_max = 0.0_f64;
    for j in 0..ny {
        let y = y_lo + j as f64 * gs;
        for i in 0..nx {
            let x = x0 + i as f64 * gs;
            let (ti, di) = eval_blade_thickness_and_sdf2(&p, &b, x, y);
            t[j * nx + i] = ti as f32;
            d2[j * nx + i] = di as f32;
            t_max = t_max.max(ti);
        }
    }
    let mut f = fs::File::create(format!("{}.f32", a[3])).expect("out");
    for v in t.iter().chain(&d2) {
        f.write_all(&v.to_le_bytes()).unwrap();
    }
    let head = serde_json::json!({"design_id": a[2], "schema": g.schema, "decoder_sha256": g5_decoder_sha256(),
        "x0": x0, "y0": y_lo, "g": gs, "nx": nx, "ny": ny, "t_max": t_max, "x_tip": b.x_tip, "tip_xy": b.tip_xy,
        "arrays": ["t", "d2"], "dtype": "f32le", "layout": "[ny][nx]"});
    fs::write(format!("{}.json", a[3]), serde_json::to_string_pretty(&head).unwrap()).unwrap();
    eprintln!("wrote {}.f32 ({nx} x {ny}, t_max {t_max:.2} mm)", a[3]);
}
