//! Release tool (v3.0.0): true edge-normal cross-sections of both cutting
//! edges from the exact blade thickness law (`eval_blade_thickness_and_sdf2`, no voxels): every 2 mm of arc
//! from heel to tip, the full thickness t(s) at distance s along the inward edge normal, and from it the
//! apex thickness, the per-side bevel angle near the apex (s 0.1..0.5 mm) and mid-bevel (0.5..1.5 mm), the
//! steepest per-side slope anywhere in s 0..8 mm (a shoulder shows here), and the grind width (s where t
//! first reaches 90 % of its 8 mm value).
//!
//!   edgeprofile <root[,root]> <id,id,...> <out.jsonl>
use jarambit_g5::g5_genome::G5Genome;
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
    let mut f = fs::File::create(&a[3]).expect("out");
    for id in a[2].split(',') {
        let g = find(&a[1], id);
        let p = g.decode();
        let b = build_blade_profile_2d(&p);
        let n = b.xs.len();
        for (edge, ys, sign) in [("A_in", &b.y_in, 1.0), ("B_sp", &b.y_sp, -1.0)] {
            // arc length along this edge polyline
            let mut arc = vec![0.0; n];
            for i in 1..n {
                arc[i] = arc[i - 1] + (b.xs[i] - b.xs[i - 1]).hypot(ys[i] - ys[i - 1]);
            }
            let total = arc[n - 1];
            let mut next = 0.0;
            for i in 1..n - 1 {
                if arc[i] < next {
                    continue;
                }
                next += 2.0;
                let (tx, ty) = (b.xs[i + 1] - b.xs[i - 1], ys[i + 1] - ys[i - 1]);
                let l = tx.hypot(ty).max(1e-12);
                // inward normal: edge A (y_in, lower boundary) points +y; edge B (y_sp, upper) points -y
                let (nx, ny) = (-ty / l * sign, tx / l * sign);
                let (nx, ny) = if ny * sign < 0.0 { (-nx, -ny) } else { (nx, ny) };
                let t_at = |s: f64| eval_blade_thickness_and_sdf2(&p, &b, b.xs[i] + s * nx, ys[i] + s * ny).0;
                let ss: Vec<f64> = (0..=160).map(|k| 0.05 * k as f64).collect();
                let ts: Vec<f64> = ss.iter().map(|&s| t_at(s.max(1e-6))).collect();
                let ang = |s0: f64, s1: f64| ((t_at(s1) - t_at(s0)) / (2.0 * (s1 - s0))).atan().to_degrees();
                let mut steep = 0.0_f64;
                for k in 1..ts.len() {
                    steep = steep.max(((ts[k] - ts[k - 1]) / (2.0 * 0.05)).atan().to_degrees());
                }
                let t8 = ts[ts.len() - 1];
                let width = ss.iter().zip(&ts).find(|(_, &t)| t >= 0.9 * t8).map(|(&s, _)| s).unwrap_or(8.0);
                writeln!(f, "{}", serde_json::json!({
                    "design_id": id, "edge": edge, "arc_mm": arc[i], "from_tip_mm": total - arc[i],
                    "x": b.xs[i], "y": ys[i], "edge_slope_deg": ty.atan2(tx).to_degrees(),
                    "t_apex": ts[1], "t_0p5": t_at(0.5), "t_1": t_at(1.0), "t_2": t_at(2.0), "t_8": t8,
                    "ang_apex_deg": ang(0.1, 0.5), "ang_mid_deg": ang(0.5, 1.5), "steepest_deg": steep,
                    "grind_width_mm": width,
                    "normal": [nx, ny],
                    "t_profile_0p1": (0..=80).map(|k| ts[2 * k]).collect::<Vec<f64>>(),
                    "vertical_ang_deg": ((eval_blade_thickness_and_sdf2(&p, &b, b.xs[i], ys[i] + sign * 0.5).0
                        - eval_blade_thickness_and_sdf2(&p, &b, b.xs[i], ys[i] + sign * 0.1).0) / 0.8).atan().to_degrees(),
                })).unwrap();
            }
        }
    }
    eprintln!("wrote {}", a[3]);
}
