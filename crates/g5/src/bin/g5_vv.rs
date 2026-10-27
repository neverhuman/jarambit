//! G5 verification and validation receipts: re-runs every numerical study behind the G5.0 decoder freeze on fixed
//! Sobol designs and writes one deterministic JSON per study (plus SHA256SUMS) into `<out_dir>`.
//!
//!   g5-vv <out_dir> [threads=4]
//!
//! Studies (README and paper numbers come from these receipts, never from hand-copied probe output):
//! - `grid.json`: partial-volume mass at h 0.4 and h 0.2 ([mass] grid_agreement_max_frac), with sealed volume.
//! - `plate_mesh.json`: Tier-1 whole-blade plate lateral / twist / pull SFs at 1.2, 0.6, 0.3 and 0.15 mm2.
//! - `tip_mesh.json`: solid-tip axial / side / twist SFs at 0.15 and 0.08 mm2, and the section-model values.
//! - `decoder_targets.json`: grip swell perimeter and H/T against their genes, point angles against the point gene,
//!   sealed volume, on 12 designs.
//! - `checks.json`: the sheet-TPMS density against the built sheet, the Ritz column against Euler, Lloyd CVT
//!   quantisation error against Halton, and the objective properties.
use jarambit_g5::g5_genome::sobol_unit_genome;
use jarambit_g5::g5_qd::{halton8, lloyd_cvt_centroids, mass_grid_agreement, measure_point_planform};
use jarambit_g5::g5_sdf::{build_blade_profile_2d, build_g5_sdf, tpms_val_and_grad};
use jarambit_g5::loads::{analytic_loads, plate_blade_cases_at, tip_buckling_load_n, tip_solid_cases_at};
use jarambit_g5::{material, objective};
use rayon::prelude::*;
use serde_json::{json, Value};
use std::{fs, path::Path};

const SEED: usize = 9;

fn design(i: usize) -> jarambit_g5::g5_genome::G5Genome {
    sobol_unit_genome(i + 1, SEED, "g5-vv")
}

fn write(dir: &Path, name: &str, v: &Value) {
    let text = serde_json::to_string_pretty(v).unwrap() + "\n";
    fs::write(dir.join(name), text).unwrap();
    eprintln!("wrote {name}");
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let out = Path::new(a.get(1).expect("usage: g5-vv <out_dir> [threads]"));
    let threads: usize = a.get(2).and_then(|v| v.parse().ok()).unwrap_or(4);
    rayon::ThreadPoolBuilder::new().num_threads(threads).build_global().ok();
    fs::create_dir_all(out).unwrap();
    let rs = jarambit_g5::requirements::rs_v5();

    // Grid agreement
    let grid: Vec<Value> = (0..6usize)
        .into_par_iter()
        .map(|i| {
            let a = mass_grid_agreement(&design(i), 0.4);
            json!({"design": i, "mass_h04_g": a.mass_h_g, "mass_h02_g": a.mass_half_h_g, "rel_diff": a.rel_diff,
                   "sealed_h04_mm3": a.sealed_void_h_mm3, "sealed_h02_mm3": a.sealed_void_half_h_mm3,
                   "within_requirement": a.within_requirement()})
        })
        .collect();
    let max_rel = grid.iter().map(|r| r["rel_diff"].as_f64().unwrap()).fold(0.0, f64::max);
    write(out, "grid.json", &json!({"study": "partial-volume mass, h 0.4 vs h 0.2", "seed_offset": SEED,
        "requirement_max_frac": rs.mass.grid_agreement_max_frac, "max_rel_diff": max_rel, "rows": grid}));

    // Plate mesh convergence
    let areas = [1.2, 0.6, 0.3, 0.15];
    let plate: Vec<Value> = (0..4usize)
        .into_par_iter()
        .map(|i| {
            let g = design(i);
            let p = g.decode();
            let b = build_blade_profile_2d(&p);
            let base = analytic_loads(&p, &b, 900.0, 120.0);
            let per: Vec<Value> = areas
                .iter()
                .map(|&ar| {
                    let mut l = base;
                    let ok = plate_blade_cases_at(&p, &b, &mut l, ar);
                    json!({"max_area_mm2": ar, "solved": ok, "lateral_sf": l.lateral_limit_sf, "twist_sf": l.twist_limit_sf,
                           "pull_sf": l.pull_sf, "fatigue_sf": l.fatigue_sf})
                })
                .collect();
            json!({"design": i, "sections": {"lateral_sf": base.lateral_limit_sf,
                   "twist_sf": base.twist_limit_sf, "pull_sf": base.pull_sf}, "plate": per})
        })
        .collect();
    write(out, "plate_mesh.json", &json!({"study": "Tier-1 whole-blade plate mesh convergence", "seed_offset": SEED,
        "production_max_area_mm2": jarambit_g5::loads::PLATE_MAX_AREA_MM2, "rows": plate}));

    // Solid-tip mesh convergence
    let tip: Vec<Value> = (0..3usize)
        .into_par_iter()
        .map(|i| {
            let g = design(i);
            let p = g.decode();
            let b = build_blade_profile_2d(&p);
            let sec = analytic_loads(&p, &b, 900.0, 120.0);
            let per: Vec<Value> = [0.15, 0.08]
                .iter()
                .map(|&ar| match tip_solid_cases_at(&p, &b, ar) {
                    Some(t) => json!({"max_area_mm2": ar, "axial_sf": t.axial_sf, "lateral_sf": t.lateral_sf,
                                      "twist_sf": t.twist_sf, "n_dof": t.n_dof}),
                    None => json!({"max_area_mm2": ar, "failed": true}),
                })
                .collect();
            json!({"design": i, "point_angle_deg": p["point_angle"], "sections": {"axial_sf": sec.tip_axial_sf,
                   "lateral_sf": sec.tip_lateral_sf, "twist_sf": sec.tip_twist_sf}, "solid": per})
        })
        .collect();
    write(out, "tip_mesh.json", &json!({"study": "solid-tip TET10 mesh convergence and section model", "seed_offset": SEED,
        "certification_max_area_mm2": jarambit_g5::loads::TIP_FE_MAX_AREA_MM2, "rows": tip}));

    // Decoder construction targets
    let targets: Vec<Value> = (0..12usize)
        .into_par_iter()
        .map(|i| {
            let g = design(i);
            let p = g.decode();
            let grid = build_g5_sdf(&g, 0.4, true);
            let hz = grid.measure_handzone_masses_and_swell(&p);
            let pt = measure_point_planform(&build_blade_profile_2d(&p));
            json!({"design": i, "perimeter_mm": hz.swell_perimeter_mm, "perimeter_gene_mm": p["grip_perim"],
                   "h_over_t": hz.swell_h_over_t, "h_over_t_gene": p["grip_hot"], "grip_length_mm": hz.l_g_mm,
                   "point_angle_1mm_deg": pt.angle_1mm_deg, "point_angle_3mm_deg": pt.angle_3mm_deg,
                   "point_width_2mm": pt.width_2mm, "point_angle_gene_deg": p["point_angle"],
                   "sealed_mm3": grid.am_connectivity.sealed_void_mm3})
        })
        .collect();
    let perim_err = targets.iter().map(|r| (r["perimeter_mm"].as_f64().unwrap() / r["perimeter_gene_mm"].as_f64().unwrap() - 1.0).abs()).fold(0.0, f64::max);
    write(out, "decoder_targets.json", &json!({"study": "G5.0 decoder construction targets at h 0.4", "seed_offset": SEED,
        "max_perimeter_rel_err": perim_err, "rows": targets}));

    // Checks
    let mut sheet = Vec::new();
    for mix in [0.0, 1.0] {
        for rho in [0.25, 0.40] {
            let cell = 8.0;
            let t = material::tpms_sheet_thickness(rho, cell, mix);
            let n = 48usize;
            let mut c = 0usize;
            for i in 0..n {
                for j in 0..n {
                    for k in 0..n {
                        let s = |q: usize| (q as f64 + 0.5) / n as f64 * cell;
                        let (f, gn) = tpms_val_and_grad(s(i), s(j), s(k), cell, mix, 1.0);
                        if f.abs() / gn.max(1e-6) < 0.5 * t {
                            c += 1;
                        }
                    }
                }
            }
            sheet.push(json!({"lat_mix": mix, "target_rho": rho, "sheet_mm": t,
                              "built_rho": material::tpms_density_of_sheet(t, cell, mix),
                              "measured_rho": c as f64 / (n * n * n) as f64}));
        }
    }
    let (ei, l) = (2.0e6_f64, 25.0_f64);
    let ritz = tip_buckling_load_n(&|_| ei, 0.0, l);
    let euler = std::f64::consts::PI.powi(2) * ei / (4.0 * l * l);
    let q_err = |c: &[[f64; 8]]| {
        let tbl = jarambit_g5::sobol_joe_kuo::Sobol179Table::scrambled(99);
        (0..4000).map(|i| { let p = tbl.point(i + 1); c.iter().map(|cc| (0..8).map(|d| (p[d] - cc[d]).powi(2)).sum::<f64>()).fold(f64::INFINITY, f64::min) }).sum::<f64>() / 4000.0
    };
    let o = &rs.objective;
    let m = &rs.mass;
    let wm: Vec<Value> = [0.5, 0.7, 0.85, 0.95, 1.0].iter().map(|&v| json!([v, objective::worst_margin_multiplier(v)])).collect();
    write(out, "checks.json", &json!({
        "sheet_tpms": sheet,
        "ritz_column": {"ei_n_mm2": ei, "length_mm": l, "ritz_n": ritz, "euler_n": euler, "rel_err": ritz / euler - 1.0},
        "cvt_1024": {"halton_quantisation_error": q_err(&halton8(1024)), "lloyd_quantisation_error": q_err(&lloyd_cvt_centroids(1024))},
        "objective": {
            "s_margin_at_0_64": objective::s_margin(0.64),
            "worst_margin_multiplier": wm,
            "grams_per_percent_e_at_base_0_6": o.grams_per_percent_e(0.6, m),
            "min_grams_per_percent_e": o.min_grams_per_percent_e
        }
    }));

    // SHA256SUMS
    let mut names: Vec<String> = fs::read_dir(out).unwrap().flatten().map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.ends_with(".json")).collect();
    names.sort();
    let sums: String = names.iter().map(|n| format!("{}  {n}\n", jarambit_g5::sha256_bytes(&fs::read(out.join(n)).unwrap()))).collect();
    fs::write(out.join("SHA256SUMS"), sums).unwrap();
    eprintln!("wrote SHA256SUMS ({} files)", names.len());
}
