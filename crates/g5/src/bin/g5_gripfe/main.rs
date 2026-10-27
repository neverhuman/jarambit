//! Release tool (v3.0.0): grip and human-max twist certification. Hex8 voxel FE on the
//! design's own AM voxel field (`build_g5_sdf(.., with_am = true).d_am`: skin, lattice, rib, gallery and
//! windows as built), against the closed-section formulas of `eval_hollow_grip_sf`.
//! `voxel_fe/` is a verbatim copy of `platform/voxel-fe/src` at 53e75771 (implant lane), `crate::` -> `super::`.
//!
//!   gripfe control <h> <out.jsonl> [threads] [kinds]    prismatic bars against Saint-Venant / Bredt
//!   gripfe run <root[,root]> <id,id,...> <h> <out.jsonl> [threads]
//!
//! Domains (clamped at the low-x face, wrench on the high-x face about its area centroid; statistics only
//! in the inner window, away from both faces):
//!   grip: x in [x_t + 0.05 L_g, x_t + 0.95 L_g], stats [x_t + 0.20 L_g, x_t + 0.80 L_g]
//!   root: x in [x_t + 0.60 L_g, x_heel + 25],    stats [GUARD_X - 10, x_heel + 15]
//! Cases (the loads of `eval_hollow_grip_sf`, translated from their points of action to the load face):
//!   lat: 260 N along z at the tip; twist: 16.2 N m about x; spine: 3427 N along y at the blade centroid x.
//! Stresses are reported as von Mises volume quantiles (q99, q99.9) and max, never as a single-element claim.
mod voxel_fe;

use jarambit_g5::g5_genome::G5Genome;
use jarambit_g5::g5_sandwich::{SandwichSection, YIELD_TI64_MPA};
use jarambit_g5::g5_sdf::{build_blade_profile_2d, build_g5_sdf, eval_grip_slice, GUARD_X_MM};
use std::collections::BTreeMap;
use std::{fs, io::Write, path::Path, time::Instant};
use voxel_fe::*;

const TI: Material = Material { youngs_mpa: 113_800.0, poisson: 0.342 };

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

/// Clamp the low-x node plane, load the high-x faces; the reference is the loaded area centroid.
fn solve(model: &Model, threads: usize) -> (UnitSolution, P3, f64) {
    let g = &model.grid;
    let x_lo = g.origin[0];
    let loaded = faces_on_plane(model, 0, g.n[0], true, &[]);
    let (mut sy, mut sz) = (0.0, 0.0);
    for f in &loaded {
        let c = g.element_center(f.element);
        sy += c[1];
        sz += c[2];
    }
    let nf = loaded.len().max(1) as f64;
    let reference = [g.origin[0] + g.h * g.n[0] as f64, sy / nf, sz / nf];
    let problem = Problem::new(g, |q| q[0] <= x_lo + 0.25 * g.h, loaded, reference);
    let opts = Options { relative_tolerance: 1e-8, max_iterations: 20_000, threads, keep_displacements: false };
    let t = Instant::now();
    let sol = solve_unit_wrenches(model, &problem, &opts).expect("voxel FE solve");
    (sol, reference, t.elapsed().as_secs_f64())
}

fn stats(sol: &UnitSolution, w: [f64; 6], sel: impl Fn(P3) -> bool) -> serde_json::Value {
    match sol.case(w, sel, &[]) {
        Some(c) => serde_json::json!({"q99": c.q99_von_mises_mpa, "q999": c.q999_von_mises_mpa,
                                      "max": c.max_von_mises_mpa, "mean": c.mean_von_mises_mpa, "n": c.elements}),
        None => serde_json::Value::Null,
    }
}

fn control(h: f64, out: &str, threads: usize, kinds: &str) {
    // 27 x 18 mm section, 60 mm long: solid, closed tube (wall 1.2), tube slit along one side.
    let (a, b, l, t) = (27.0, 18.0, 60.0, 1.2);
    let n = [(l / h).round() as usize, (a / h).round() as usize, (b / h).round() as usize];
    let grid = Grid { n, h, origin: [0.0, -a / 2.0, -b / 2.0] };
    let mut f = fs::File::create(out).expect("out");
    for kind in kinds.split(',') {
        let phase = (0..grid.elements())
            .map(|e| {
                let c = grid.element_center(e);
                let wall = c[1].abs() > a / 2.0 - t || c[2].abs() > b / 2.0 - t;
                let slit = kind == "slit" && c[2] > 0.0 && c[1].abs() < 1.0;
                // elliptical tube: outer semi-axes (a/2, b/2), inner (a/2 - t, b/2 - t); "ewin" adds one window in
                // each z face, 20 mm long (x 20..40) and 8 mm wide (|y| < 4), like the grip's exo windows
                let e_out = (c[1] / (a / 2.0)).powi(2) + (c[2] / (b / 2.0)).powi(2) <= 1.0;
                let e_in = (c[1] / (a / 2.0 - t)).powi(2) + (c[2] / (b / 2.0 - t)).powi(2) < 1.0;
                let win = kind == "ewin" && (20.0..40.0).contains(&c[0]) && c[1].abs() < 4.0;
                u8::from(match kind {
                    "solid" => true,
                    "etube" => e_out && !e_in,
                    "ewin" => e_out && !e_in && !win,
                    _ => wall && !slit,
                })
            })
            .collect();
        let model = Model { grid: grid.clone(), phase, materials: vec![TI] };
        let (sol, _, secs) = solve(&model, threads);
        let mid = |q: P3| (15.0..45.0).contains(&q[0]);
        // Saint-Venant a/b = 1.5: tau_max = T / (0.231 a b^2); Bredt: T / (2 A_m t); open: 3 T / (sum s t^2)
        let am = (a - t) * (b - t);
        let am_e = std::f64::consts::PI * (a / 2.0 - t / 2.0) * (b / 2.0 - t / 2.0);
        let theory = match kind {
            "solid" => 16_200.0 / (0.231 * a * b * b),
            "tube" => 16_200.0 / (2.0 * am * t),
            "etube" | "ewin" => 16_200.0 / (2.0 * am_e * t),
            _ => 3.0 * 16_200.0 / ((2.0 * (a + b) - 2.0 * t - 2.0) * t * t),
        };
        let j = serde_json::json!({"control": kind, "h": h, "secs": secs,
            "theory_tau_mpa": theory, "theory_vm_mpa": theory * 3f64.sqrt(),
            "twist": stats(&sol, [0., 0., 0., 16_200., 0., 0.], mid),
            "residual": sol.convergence.iter().map(|c| c.relative_residual).collect::<Vec<_>>()});
        writeln!(f, "{j}").unwrap();
        eprintln!("{j}");
    }
}

/// Per-case closed-section formulas of `eval_hollow_grip_sf` (copied; worst over the 3 stations).
fn formula_cases(p: &BTreeMap<&'static str, f64>, tip_xy: (f64, f64), xc: f64) -> serde_json::Value {
    let is_g5_1 = p.get("g5_schema_ver").copied().unwrap_or(0.0) >= 1.0;
    let flange = if is_g5_1 { p.get("flange_amp_grip").copied().unwrap_or(0.0) } else { 0.0 };
    let rho_g = p["rho_grip"];
    let rho_e = rho_g.powf(1.8);
    let hollow = rho_g < 0.95;
    let skin_t = p["skin_t"];
    let skin_tq = skin_t + 0.40 * flange;
    let allow = YIELD_TI64_MPA * 0.912 * (skin_tq - 0.1) / skin_tq.max(0.2);
    let (mut lat, mut tw, mut sp) = (0.0_f64, 0.0_f64, 0.0_f64);
    for s in [0.25, 0.50, 0.75] {
        let x = p["x_t"] + s * p["L_g"];
        let gc = eval_grip_slice(p, x);
        let h_sec = (gc.yt - gc.yb).max(8.0);
        let t_sec = gc.t_sec.max(4.0);
        let a_enc = std::f64::consts::PI * 0.25 * h_sec * t_sec;
        let perim = std::f64::consts::PI
            * (3.0 * 0.5 * (h_sec + t_sec) - ((1.5 * h_sec + 0.5 * t_sec) * (0.5 * h_sec + 1.5 * t_sec)).sqrt());
        let sec = SandwichSection::from_skin_and_lattice(t_sec, skin_tq, if hollow { rho_g } else { 1.0 },
            if p["rib_w"] > 0.0 { 0.4 + 0.4 * p["rib_w"] } else { 0.0 });
        let i_yy = (0.78 * h_sec) * sec.t3_bend / 12.0;
        let h_in = (h_sec - 2.0 * (skin_t + 0.75 * flange)).max(0.0);
        let t_in = (t_sec - 2.0 * skin_t).max(0.0);
        let t_rib = if hollow && p["rib_w"] > 0.0 { (0.8 + 0.8 * p["rib_w"]).min(t_in) } else { 0.0 };
        let i_zz = (std::f64::consts::PI / 64.0)
            * ((t_sec * h_sec.powi(3) - t_in * h_in.powi(3)) + rho_e * (t_in - t_rib).max(0.0) * h_in.powi(3))
            + (t_rib * h_in.powi(3)) / 12.0;
        let box_k = if hollow { 1.0 + 0.25 * p["rib_w"] } else { 1.0 };
        let arm = (tip_xy.0 - x).max(10.0);
        let sig_lat = 260.0 * arm * (0.5 * t_sec) / i_yy.max(1.0);
        let t_mom = 260.0 * (tip_xy.1 - gc.ym).abs();
        let tors = |t: f64| if hollow { t / (2.0 * a_enc * skin_tq * box_k).max(1.0) }
                            else { 3.0 * t / (perim * 0.5 * t_sec.min(h_sec).powi(2)).max(1.0) };
        let tau_lat = tors(t_mom);
        lat = lat.max((sig_lat * sig_lat + 3.0 * tau_lat * tau_lat).sqrt());
        tw = tw.max(3f64.sqrt() * tors(16_200.0));
        sp = sp.max(3427.0 * (xc - x).max(0.0) * (0.5 * h_sec) / i_zz.max(1.0));
    }
    serde_json::json!({"allow_mpa": allow, "lat_vm": lat, "twist_vm": tw, "spine_vm": sp,
                       "sf_lat": allow / lat, "sf_twist": allow / tw, "sf_spine": allow / sp})
}

fn crop(grid: &jarambit_g5::g5_sdf::VoxelGrid3D, x0: f64, x1: f64) -> Model {
    let d = grid.d_am.as_deref().unwrap_or(&grid.d);
    let (nx, ny, nz) = (grid.nx, grid.ny, grid.nz);
    let i0 = grid.x.iter().position(|&x| x >= x0).unwrap();
    let i1 = grid.x.iter().rposition(|&x| x <= x1).unwrap();
    let inside = |i: usize, j: usize, k: usize| d[(i * ny + j) * nz + k] < 0.0;
    let (mut j0, mut j1, mut k0, mut k1) = (ny, 0, nz, 0);
    for i in i0..=i1 {
        for j in 0..ny {
            for k in 0..nz {
                if inside(i, j, k) {
                    (j0, j1, k0, k1) = (j0.min(j), j1.max(j), k0.min(k), k1.max(k));
                }
            }
        }
    }
    let n = [i1 - i0 + 1, j1 - j0 + 1, k1 - k0 + 1];
    let h = grid.h;
    let g = Grid { n, h, origin: [grid.x[i0] - 0.5 * h, grid.y[j0] - 0.5 * h, grid.z[k0] - 0.5 * h] };
    let phase = (0..g.elements())
        .map(|e| {
            let [a, b, c] = g.element_index(e);
            u8::from(inside(i0 + a, j0 + b, k0 + c))
        })
        .collect();
    Model { grid: g, phase, materials: vec![TI] }
}

fn run(roots: &str, ids: &str, h: f64, out: &str, threads: usize) {
    let mut f = fs::File::create(out).expect("out");
    for id in ids.split(',') {
        let g = find(roots, id);
        let p = g.decode();
        let blade = build_blade_profile_2d(&p);
        let (tx, ty) = blade.tip_xy;
        let xc = blade.centroid_x_mm;
        let grid = build_g5_sdf(&g, h, true);
        let (xt, lg) = (p["x_t"], p["L_g"]);
        let skin = p["skin_t"];
        let allow = YIELD_TI64_MPA * 0.912 * (skin - 0.1) / skin.max(0.2);
        let mut rec = serde_json::json!({"design_id": id, "h": h, "schema": g.schema, "skin_t": skin,
            "allow_fe_mpa": allow, "formula": formula_cases(&p, blade.tip_xy, xc)});
        for (name, x0, x1, s0, s1) in [
            ("grip", xt + 0.05 * lg, xt + 0.95 * lg, xt + 0.20 * lg, xt + 0.80 * lg),
            ("root", xt + 0.60 * lg, blade.x_heel + 25.0, GUARD_X_MM - 10.0, blade.x_heel + 15.0),
        ] {
            let model = crop(&grid, x0, x1);
            let solid = model.phase.iter().filter(|&&q| q != 0).count();
            let (sol, r, secs) = solve(&model, threads);
            let sel = |q: P3| (s0..s1).contains(&q[0]);
            let lat = [0., 0., 260., 260. * (ty - r[1]), -260. * (tx - r[0]), 0.];
            let spine = [0., 3427., 0., 0., 0., 3427. * (xc - r[0])];
            let twist = [0., 0., 0., 16_200., 0., 0.];
            rec[name] = serde_json::json!({
                "x": [x0, x1], "stats_x": [s0, s1], "elements": solid, "secs": secs,
                "excluded_elements": sol.exclusion.unsupported_elements, "components": sol.exclusion.components,
                "residual": sol.convergence.iter().map(|c| c.relative_residual).collect::<Vec<_>>(),
                "lat": stats(&sol, lat, sel), "twist": stats(&sol, twist, sel), "spine": stats(&sol, spine, sel),
                "torsional_stiffness_nmm_per_rad": 1.0 / sol.flexibility[3][3],
            });
            eprintln!("{id} {name}: {} el, {secs:.1}s", solid);
        }
        writeln!(f, "{rec}").unwrap();
        f.flush().unwrap();
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a[1].as_str() {
        "control" => control(a[2].parse().unwrap(), &a[3], a.get(4).and_then(|v| v.parse().ok()).unwrap_or(4),
                             a.get(5).map(String::as_str).unwrap_or("solid,tube,slit,etube,ewin")),
        "run" => run(&a[2], &a[3], a[4].parse().unwrap(), &a[5], a.get(6).and_then(|v| v.parse().ok()).unwrap_or(4)),
        _ => eprintln!("usage: gripfe control <h> <out> [threads] | run <roots> <ids> <h> <out> [threads]"),
    }
}
