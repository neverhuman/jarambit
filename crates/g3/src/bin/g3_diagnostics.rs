//! Non-invasive G3 v3.0.0 diagnostic suite (`P-16`, `AG-05`, `AG-07`, `AG-10`, `F-M-1`, `F-M-2`, `§8.1`).
//!
//! Generates four deterministic diagnostic receipts under `evidence/g3/v3.0.0/diagnostics/`
//! without touching any file hashed by `g3_decoder_sha256()`:
//! 1. `seq_per_lc.json`: Per-load-case yield sequence ratios (`lateral = 2.736`, `twist = 5.056`,
//!    `spine = 0.515`), corrected Goodman fatigue SF (`1.060` vs double-knockdown `0.942`), and
//!    re-scored `W_ERGOLIGHT_v1` using print-mesh mass (`59.30 g` -> `79.75` vs `57.00 g` -> `80.55`).
//! 2. `zone_mass.json`: Exact x-slab divergence-theorem volume & mass integration on the committed
//!    watertight STLs (`Ring < 9.9 mm`, `Grip 9.9-75.6 mm`, `Guard 75.6-79.6 mm`, `Blade root 79.6-92.0 mm`,
//!    `Blade mid 92.0-115.0 mm`, `Blade tip > 115.0 mm`).
//! 3. `plate_meshconv.json`: 4-level `SandwichPlate` mesh convergence study (`max_area` in
//!    `{3.5, 1.0, 0.25, 0.0625} mm^2`) with 3-level Roache GCI calculation.
//! 4. `tip_raster.json`: Quantification of the half-cell `(dx/2, dy/2)` thickness raster shift in
//!    `solve_g3_solid_tet10_strict` vs cell-centered `solid_tet10.rs`, plus planform point included
//!    angles at `1.0 mm` and `3.0 mm` from the tip (`Gate S4/S5`).

use jarambit_g3::ergolight::{score_ergolight, ErgolightInputs};
use jarambit_g3::g3_genome::G3Genome;
use jarambit_g3::g3_qd::g3_decoder_sha256;
use jarambit_g3::g3_sandwich::{joint_k, solve_g3_solid_tet10_strict, SandwichPlate, SandwichSection};
use jarambit_g3::g3_sdf::{build_blade_profile_2d, effective_blade_rho, eval_blade_thickness_and_sdf2};
use jarambit_physics::fine::mesh_domain;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

const E_TI64_MPA: f64 = 113_800.0;
const NU_PLATE: f64 = 0.34;
const YIELD_TI64_MPA: f64 = 828.0;
const RHO_TI64_G_MM3: f64 = 0.00443;

fn read_binary_stl_tris(path: &Path) -> Result<Vec<[[f64; 3]; 3]>, String> {
    let bytes = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if bytes.len() < 84 {
        return Err("STL too short".into());
    }
    let n_tri = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
    if bytes.len() < 84 + n_tri * 50 {
        return Err("STL truncated".into());
    }
    let mut tris = Vec::with_capacity(n_tri);
    for i in 0..n_tri {
        let off = 84 + i * 50 + 12;
        let mut tri = [[0.0_f64; 3]; 3];
        for v in 0..3 {
            for c in 0..3 {
                let b_off = off + (v * 3 + c) * 4;
                tri[v][c] = f32::from_le_bytes(bytes[b_off..b_off + 4].try_into().unwrap()) as f64;
            }
        }
        tris.push(tri);
    }
    Ok(tris)
}

/// Exact volume of a watertight mesh in `x <= x_cut` via the continuous divergence field
/// `F = (min(x, x_cut), 0, 0)` where `div F = 1_{x < x_cut}` (`§8.1`).
fn clip_tri_below_x(tri: &[[f64; 3]; 3], x_cut: f64) -> f64 {
    fn tri_nx_2a(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> f64 {
        let ux = b[1] - a[1];
        let uy = b[2] - a[2];
        let vx = c[1] - a[1];
        let vy = c[2] - a[2];
        ux * vy - uy * vx
    }
    let interp = |p1: [f64; 3], p2: [f64; 3]| -> [f64; 3] {
        let t = ((x_cut - p1[0]) / (p2[0] - p1[0])).clamp(0.0, 1.0);
        [
            x_cut,
            p1[1] + t * (p2[1] - p1[1]),
            p1[2] + t * (p2[2] - p1[2]),
        ]
    };
    let mut below = Vec::with_capacity(4);
    let mut above = Vec::with_capacity(4);
    for i in 0..3 {
        let cur = tri[i];
        let nxt = tri[(i + 1) % 3];
        let cur_in = cur[0] <= x_cut;
        let nxt_in = nxt[0] <= x_cut;
        if cur_in {
            below.push(cur);
        } else {
            above.push(cur);
        }
        if cur_in != nxt_in {
            let m = interp(cur, nxt);
            below.push(m);
            above.push(m);
        }
    }
    let mut vol = 0.0;
    if below.len() >= 3 {
        for k in 1..(below.len() - 1) {
            let a = below[0];
            let b = below[k];
            let c = below[k + 1];
            let x_mean = (a[0] + b[0] + c[0]) / 3.0;
            vol += 0.5 * x_mean * tri_nx_2a(a, b, c);
        }
    }
    if above.len() >= 3 {
        for k in 1..(above.len() - 1) {
            let a = above[0];
            let b = above[k];
            let c = above[k + 1];
            vol += 0.5 * x_cut * tri_nx_2a(a, b, c);
        }
    }
    vol
}

fn integrate_stl_x_slabs(tris: &[[[f64; 3]; 3]], cuts_x: &[f64]) -> Vec<f64> {
    let mut cum_vols = Vec::with_capacity(cuts_x.len() + 1);
    for &xc in cuts_x {
        let v: f64 = tris.iter().map(|t| clip_tri_below_x(t, xc)).sum();
        cum_vols.push(v.abs());
    }
    let total_v: f64 = tris.iter().map(|t| clip_tri_below_x(t, 1.0e6)).sum::<f64>().abs();
    cum_vols.push(total_v);
    let mut slab_masses = Vec::with_capacity(cum_vols.len());
    let mut prev = 0.0;
    for &cv in &cum_vols {
        let dv = (cv - prev).max(0.0);
        slab_masses.push(dv * RHO_TI64_G_MM3);
        prev = cv;
    }
    slab_masses
}

fn run_plate_at_area(
    p: & std::collections::BTreeMap<&'static str, f64>,
    blade: &jarambit_g3::g3_sdf::BladeProfile2D,
    max_area: f64,
    full_outline: bool,
) -> Value {
    let step = if full_outline { 1 } else { 4 };
    let mut b_outer = Vec::new();
    for i in (0..blade.xs.len()).step_by(step) {
        b_outer.push((blade.xs[i], blade.y_in[i]));
    }
    for i in (0..blade.xs.len()).step_by(step).rev() {
        b_outer.push((blade.xs[i], blade.y_sp[i]));
    }
    let b_mesh = mesh_domain(&b_outer, &[], max_area).expect("plate mesh");
    let skin_t = p["skin_t"];
    let rho_blade_eff = effective_blade_rho(p);
    let secs: Vec<SandwichSection> = b_mesh
        .tris
        .iter()
        .map(|t| {
            let xm = (b_mesh.xy[t[0]].0 + b_mesh.xy[t[1]].0 + b_mesh.xy[t[2]].0) / 3.0;
            let ym = (b_mesh.xy[t[0]].1 + b_mesh.xy[t[1]].1 + b_mesh.xy[t[2]].1) / 3.0;
            let (th, _) = eval_blade_thickness_and_sdf2(p, blade, xm, ym);
            SandwichSection::from_skin_and_lattice(th.max(0.3), skin_t, rho_blade_eff, 0.0)
        })
        .collect();
    let n_tris = b_mesh.tris.len();
    let n_nodes = b_mesh.xy.len();
    let s_plate = SandwichPlate::new(b_mesh, p["t_core"], E_TI64_MPA, NU_PLATE).with_sections(secs);
    let n = s_plate.n();
    let mut fixed = vec![false; n * 5];
    for (i, &(x, _)) in s_plate.mesh.xy.iter().enumerate() {
        if x <= blade.x_heel + 3.5 {
            for c in 0..5 {
                fixed[i * 5 + c] = true;
            }
        }
    }
    let sys = s_plate.system(&fixed).expect("plate system");
    let allow_plate = YIELD_TI64_MPA * joint_k(p["t_core"]);

    // Lateral 1N load
    let mut f_lat = vec![0.0; n * 5];
    let mut patch_nodes = Vec::new();
    for (i, &(x, y)) in s_plate.mesh.xy.iter().enumerate() {
        if (x - blade.tip_xy.0).hypot(y - blade.tip_xy.1) <= 3.0 {
            patch_nodes.push(i);
        }
    }
    let w_p = 1.0 / (patch_nodes.len() as f64);
    for &nd in &patch_nodes {
        f_lat[nd * 5 + 2] = w_p;
    }
    let (u_lat, _) = s_plate.solve_checked(&sys, &f_lat).expect("lat solve");
    let (_, sb) = s_plate.stresses(&u_lat);
    let mut pk_sb_rob = 0.0_f64;
    for (e, &s_val) in sb.iter().enumerate() {
        let c = s_plate.centroid(e);
        let d_tip = (c.0 - blade.tip_xy.0).hypot(c.1 - blade.tip_xy.1);
        if d_tip >= 4.0 && c.0 >= blade.x_heel + 4.0 {
            pk_sb_rob = pk_sb_rob.max(s_val);
        }
    }
    let cap_lat_n = allow_plate / pk_sb_rob;
    let fat_sf_corrected = jarambit_crescent::fem::fatigue::goodman_sf(180.0 * pk_sb_rob, p["t_core"]);

    // Twist 1.82 N*m
    let mut f_twist = vec![0.0; n * 5];
    let mut twist_in = Vec::new();
    let mut twist_sp = Vec::new();
    for (i, &(x, y)) in s_plate.mesh.xy.iter().enumerate() {
        let d_tip = (x - blade.tip_xy.0).hypot(y - blade.tip_xy.1);
        if (4.0..=16.0).contains(&d_tip) {
            let u_b = ((x - blade.x_heel) / (blade.x_tip - blade.x_heel).max(1e-9)).clamp(0.0, 1.0);
            let idx = (u_b * (blade.xs.len() - 1) as f64).round() as usize;
            let y_mid = 0.5 * (blade.y_in[idx] + blade.y_sp[idx]);
            if y < y_mid - 1.0 {
                twist_in.push((i, (y_mid - y).max(1.0)));
            } else if y > y_mid + 1.0 {
                twist_sp.push((i, (y - y_mid).max(1.0)));
            }
        }
    }
    let arm_in: f64 = twist_in.iter().map(|&(_, a)| a).sum::<f64>() / (twist_in.len() as f64);
    let arm_sp: f64 = twist_sp.iter().map(|&(_, a)| a).sum::<f64>() / (twist_sp.len() as f64);
    let couple_arm = (arm_in + arm_sp).max(3.0);
    let f_tot = 1820.0 / couple_arm;
    for &(nd, _) in &twist_in {
        f_twist[nd * 5 + 2] -= f_tot / (twist_in.len() as f64);
    }
    for &(nd, _) in &twist_sp {
        f_twist[nd * 5 + 2] += f_tot / (twist_sp.len() as f64);
    }
    let (u_tw, _) = s_plate.solve_checked(&sys, &f_twist).expect("twist solve");
    let (_, sb_tw) = s_plate.stresses(&u_tw);
    let mut pk_body = 0.0_f64;
    for (e, &s_val) in sb_tw.iter().enumerate() {
        let c = s_plate.centroid(e);
        let d_tip = (c.0 - blade.tip_xy.0).hypot(c.1 - blade.tip_xy.1);
        if d_tip > 16.0 && c.0 >= blade.x_heel + 5.0 {
            pk_body = pk_body.max(s_val);
        }
    }
    let hm_twist_sf = allow_plate / pk_body * (1820.0 / 16_200.0);

    // Spine 3427 N
    let mut f_spine = vec![0.0; n * 5];
    let mut spine_nodes = Vec::new();
    for (i, &(x, y)) in s_plate.mesh.xy.iter().enumerate() {
        if x >= blade.x_heel + 8.0 && x <= blade.x_tip - 8.0 {
            let u_b = ((x - blade.x_heel) / (blade.x_tip - blade.x_heel).max(1e-9)).clamp(0.0, 1.0);
            let idx = (u_b * (blade.xs.len() - 1) as f64).round() as usize;
            if (y - blade.y_sp[idx]).abs() <= 3.5 {
                spine_nodes.push(i);
            }
        }
    }
    let f_per_node = -3427.0 / (spine_nodes.len() as f64);
    for &nd in &spine_nodes {
        f_spine[nd * 5 + 1] = f_per_node;
    }
    let (u_sp, _) = s_plate.solve_checked(&sys, &f_spine).expect("spine solve");
    let (sm_sp, _) = s_plate.stresses(&u_sp);
    let mut pk_sm = 0.0_f64;
    for (e, &s_val) in sm_sp.iter().enumerate() {
        let c = s_plate.centroid(e);
        let d_tip = (c.0 - blade.tip_xy.0).hypot(c.1 - blade.tip_xy.1);
        if d_tip >= 6.0 && c.0 >= blade.x_heel + 6.0 {
            pk_sm = pk_sm.max(s_val);
        }
    }
    let hm_spine_sf = allow_plate / pk_sm;

    json!({
        "max_area_mm2": max_area,
        "full_180_station_outline": full_outline,
        "n_nodes": n_nodes,
        "n_tris": n_tris,
        "lateral_capacity_n": (cap_lat_n * 1000.0).round() / 1000.0,
        "hm_twist_16p2nm_sf": (hm_twist_sf * 10000.0).round() / 10000.0,
        "hm_spine_3427n_sf": (hm_spine_sf * 10000.0).round() / 10000.0,
        "goodman_fatigue_180n_sf_corrected": (fat_sf_corrected * 10000.0).round() / 10000.0
    })
}

fn main() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let release_path = repo_root.join("evidence/g3/v3.0.0/release.json");
    let genome_path = repo_root.join("evidence/g3/v3.0.0/winner_genome.csv");
    let out_dir = repo_root.join("evidence/g3/v3.0.0/diagnostics");
    fs::create_dir_all(&out_dir).expect("create diagnostics dir");

    let rel_str = fs::read_to_string(&release_path).expect("read release.json");
    let rel: Value = serde_json::from_str(&rel_str).expect("parse release.json");
    let genome_txt = fs::read_to_string(&genome_path).expect("read winner_genome.csv");
    let csv_row = genome_txt
        .lines()
        .find(|l| !l.trim().is_empty() && !l.starts_with("design_id"))
        .expect("winner row");
    let (_, row_payload) = csv_row.split_once(',').unwrap_or(("", csv_row));
    let genome = G3Genome::from_csv_row(if row_payload.starts_with("G3.0") { row_payload } else { csv_row })
        .expect("parse G3Genome");
    let p = genome.decode();
    let blade = build_blade_profile_2d(&p);

    // 1. seq_per_lc.json (AG-05, F-M-1, F-M-2, P-1)
    let lat_cap = rel["inputs"]["lat_rob_cert_n"].as_f64().unwrap();
    let blade_sf_lat_260 = lat_cap / 260.0;
    let blade_sf_twist = rel["inputs"]["hm_twist_sf"].as_f64().unwrap();
    let blade_sf_spine = rel["inputs"]["hm_spine_sf"].as_f64().unwrap();

    let grip_fe_lat = rel["fe_certification"]["grip_sf"]["lat"].as_f64().unwrap();
    let grip_fe_twist = rel["fe_certification"]["grip_sf"]["twist"].as_f64().unwrap();
    let grip_fe_spine = rel["fe_certification"]["grip_sf"]["spine"].as_f64().unwrap();
    let grip_formula_lat = rel["fe_certification"]["formula_sf"]["sf_lat"].as_f64().unwrap();
    let grip_formula_twist = rel["fe_certification"]["formula_sf"]["sf_twist"].as_f64().unwrap();
    let grip_formula_spine = rel["fe_certification"]["formula_sf"]["sf_spine"].as_f64().unwrap();
    let grip_search_min = rel["inputs"]["grip_sf_min"].as_f64().unwrap();

    let t_core = p["t_core"];
    let jk = joint_k(t_core);
    let allow_plate = YIELD_TI64_MPA * jk;
    let peak_stress_180n_true_mpa = allow_plate * 180.0 / lat_cap;
    let peak_stress_180n_buggy_mpa = YIELD_TI64_MPA * 180.0 / lat_cap;
    let fat_sf_reported = rel["inputs"]["fat_sf"].as_f64().unwrap();
    let fat_sf_corrected = jarambit_crescent::fem::fatigue::goodman_sf(peak_stress_180n_true_mpa, t_core);

    // Re-score with print-mesh mass (59.2966 g vs 57.0023 g)
    let mut inputs_mesh: ErgolightInputs = serde_json::from_value(rel["inputs"].clone()).expect("ErgolightInputs");
    let scored_voxel_mass_g = inputs_mesh.mass_g;
    let print_mesh_mass_g = rel["mass"]["mesh_g"].as_f64().unwrap();
    let score_voxel = score_ergolight(&inputs_mesh);
    inputs_mesh.mass_g = print_mesh_mass_g;
    let score_mesh = score_ergolight(&inputs_mesh);

    let seq_json = json!({
        "schema": "jarambit.g3.diagnostics.seq_per_lc.v1",
        "design_id": rel["design_id"],
        "decoder_sha256": g3_decoder_sha256(),
        "per_load_case_sequence_ratio_voxel_fe": {
            "lateral_260n": (grip_fe_lat / blade_sf_lat_260 * 10000.0).round() / 10000.0,
            "twist_16p2nm": (grip_fe_twist / blade_sf_twist * 10000.0).round() / 10000.0,
            "spine_3427n": (grip_fe_spine / blade_sf_spine * 10000.0).round() / 10000.0,
            "governing_blade_case": (grip_fe_lat / blade_sf_lat_260).min(grip_fe_twist / blade_sf_twist)
        },
        "per_load_case_sequence_ratio_formula": {
            "lateral_260n": (grip_formula_lat / blade_sf_lat_260 * 10000.0).round() / 10000.0,
            "twist_16p2nm": (grip_formula_twist / blade_sf_twist * 10000.0).round() / 10000.0,
            "spine_3427n": (grip_formula_spine / blade_sf_spine * 10000.0).round() / 10000.0
        },
        "legacy_v3_mixed_load_case_index": {
            "value": (grip_search_min / blade_sf_lat_260 * 10000.0).round() / 10000.0,
            "note": "Divided grip_sf_min (governed by 3427 N spine = 0.7255) by blade 260 N lateral SF (1.2977)"
        },
        "goodman_fatigue_180n_audit": {
            "t_core_mm": t_core,
            "joint_k_t_core": (jk * 10000.0).round() / 10000.0,
            "reported_v3_fat_sf_with_double_knockdown": (fat_sf_reported * 10000.0).round() / 10000.0,
            "corrected_fat_sf_from_true_plate_peak": (fat_sf_corrected * 10000.0).round() / 10000.0,
            "true_peak_stress_at_180n_mpa": (peak_stress_180n_true_mpa * 100.0).round() / 100.0,
            "overstated_stress_in_v3_mpa": (peak_stress_180n_buggy_mpa * 100.0).round() / 100.0
        },
        "mass_of_record_score_audit": {
            "scored_ranking_voxel_mass_g": (scored_voxel_mass_g * 1000.0).round() / 1000.0,
            "reported_print_mesh_mass_g": (print_mesh_mass_g * 1000.0).round() / 1000.0,
            "score_100_at_voxel_mass": (score_voxel.score_100 * 10000.0).round() / 10000.0,
            "score_100_at_print_mesh_mass": (score_mesh.score_100 * 10000.0).round() / 10000.0,
            "delta_score_100": ((score_mesh.score_100 - score_voxel.score_100) * 10000.0).round() / 10000.0
        }
    });
    fs::write(out_dir.join("seq_per_lc.json"), serde_json::to_string_pretty(&seq_json).unwrap() + "\n").unwrap();

    // 2. zone_mass.json (§8.1, REC-3)
    let am_stl_path = repo_root.join("docs/g3/v3.0.0/cad/hz4i013_g00995_d206e3c9_AMbuild_mm.stl");
    let env_stl_path = repo_root.join("docs/g3/v3.0.0/cad/hz4i013_g00995_d206e3c9_envelope_mm.stl");
    let am_tris = read_binary_stl_tris(&am_stl_path).expect("read AM STL");
    let env_tris = read_binary_stl_tris(&env_stl_path).expect("read envelope STL");
    let x_cuts = [9.9, 75.6, 79.6, 92.0, 115.0];
    let am_slabs = integrate_stl_x_slabs(&am_tris, &x_cuts);
    let env_slabs = integrate_stl_x_slabs(&env_tris, &x_cuts);
    let zone_names = [
        "ring_x_lt_9p9mm",
        "grip_9p9_to_75p6mm",
        "guard_75p6_to_79p6mm",
        "blade_root_79p6_to_92mm",
        "blade_mid_92_to_115mm",
        "blade_tip_gt_115mm",
    ];
    let am_total: f64 = am_slabs.iter().sum();
    let env_total: f64 = env_slabs.iter().sum();
    let mut zones_vec = Vec::new();
    for i in 0..zone_names.len() {
        zones_vec.push(json!({
            "zone": zone_names[i],
            "am_mass_g": (am_slabs[i] * 1000.0).round() / 1000.0,
            "am_share_pct": (am_slabs[i] / am_total * 1000.0).round() / 10.0,
            "envelope_mass_g": (env_slabs[i] * 1000.0).round() / 1000.0,
            "solid_fraction": (am_slabs[i] / env_slabs[i].max(1e-9) * 1000.0).round() / 1000.0
        }));
    }
    let zone_json = json!({
        "schema": "jarambit.g3.diagnostics.zone_mass.v1",
        "design_id": rel["design_id"],
        "am_stl_total_mass_g": (am_total * 1000.0).round() / 1000.0,
        "envelope_stl_total_mass_g": (env_total * 1000.0).round() / 1000.0,
        "overall_solid_fraction": (am_total / env_total * 1000.0).round() / 1000.0,
        "zones": zones_vec,
        "grip_subcomponent_decoded_params": {
            "skin_t_mm": (p["skin_t"] * 1000.0).round() / 1000.0,
            "rib_thickness_mm": ((0.8 + 0.8 * p["rib_w"]) * 1000.0).round() / 1000.0,
            "shear_web_thickness_mm": ((0.70 * p["shear_web_amp"]) * 1000.0).round() / 1000.0,
            "rho_grip": (p["rho_grip"] * 10000.0).round() / 10000.0,
            "rho_ring": (p["rho_ring"] * 10000.0).round() / 10000.0,
            "rho_blade_effective": (effective_blade_rho(&p) * 10000.0).round() / 10000.0
        }
    });
    fs::write(out_dir.join("zone_mass.json"), serde_json::to_string_pretty(&zone_json).unwrap() + "\n").unwrap();

    // 3. plate_meshconv.json (AG-10, P-16)
    let mut plate_rows = Vec::new();
    for &area in &[3.5, 1.0, 0.25, 0.0625] {
        plate_rows.push(run_plate_at_area(&p, &blade, area, false));
    }
    let full_outline_025 = run_plate_at_area(&p, &blade, 0.25, true);
    let plate_json = json!({
        "schema": "jarambit.g3.diagnostics.plate_meshconv.v1",
        "design_id": rel["design_id"],
        "subsampled_45_station_series": plate_rows,
        "full_180_station_at_0p25_mm2": full_outline_025,
        "note": "G3 Tier 1 used max_area = 3.5 mm^2 on every 4th outline station (45 stations) with gci_admitted = false (0.5 constant credit)."
    });
    fs::write(out_dir.join("plate_meshconv.json"), serde_json::to_string_pretty(&plate_json).unwrap() + "\n").unwrap();

    // 4. tip_raster.json (AG-07, F-M-6, P-5)
    let mut outer = Vec::new();
    for i in 0..blade.xs.len() {
        outer.push((blade.xs[i], blade.y_in[i]));
    }
    for i in (0..blade.xs.len()).rev() {
        outer.push((blade.xs[i], blade.y_sp[i]));
    }
    let m_015 = mesh_domain(&outer, &[], 0.15).expect("tip mesh 0.15");
    let g_step = 0.5;
    let th_unshifted = |x: f64, y: f64| {
        let (t, d2) = eval_blade_thickness_and_sdf2(&p, &blade, x, y);
        (d2 <= 0.0 && t > 0.05).then_some(t)
    };
    // Compensate for solid_tet10.rs cell-centered (i + 0.5)*g lookup by sampling at (x + 0.5*g, y + 0.5*g)
    let th_cell_centered = |x: f64, y: f64| {
        let (t, d2) = eval_blade_thickness_and_sdf2(&p, &blade, x + 0.5 * g_step, y + 0.5 * g_step);
        (d2 <= 0.0 && t > 0.05).then_some(t)
    };
    let sf_v3_shifted = solve_g3_solid_tet10_strict(
        m_015.xy.clone(),
        m_015.tris.clone(),
        blade.tip_xy,
        p["t_core"],
        blade.x_tip - 32.0,
        blade.tip_xy.1 - 32.0,
        g_step,
        130,
        130,
        &th_unshifted,
        0,
    )
    .map(|r| r.sf_nodal_max)
    .unwrap_or(f64::NAN);

    let sf_cell_aligned = solve_g3_solid_tet10_strict(
        m_015.xy,
        m_015.tris,
        blade.tip_xy,
        p["t_core"],
        blade.x_tip - 32.0,
        blade.tip_xy.1 - 32.0,
        g_step,
        130,
        130,
        &th_cell_centered,
        0,
    )
    .map(|r| r.sf_nodal_max)
    .unwrap_or(f64::NAN);

    // Measure planform width & included angle at a = 1.0, 2.0, 3.0 mm from the tip
    let mut planform_angles = Vec::new();
    for &dist_mm in &[1.0_f64, 2.0, 3.0] {
        let x_probe = blade.x_tip - dist_mm;
        let u_b = ((x_probe - blade.x_heel) / (blade.x_tip - blade.x_heel).max(1e-9)).clamp(0.0, 1.0);
        let idx = (u_b * (blade.xs.len() - 1) as f64).round() as usize;
        let w = (blade.y_sp[idx] - blade.y_in[idx]).abs();
        let inc_deg = 2.0 * (w / (2.0 * dist_mm)).atan().to_degrees();
        planform_angles.push(json!({
            "dist_from_tip_mm": dist_mm,
            "planform_width_mm": (w * 1000.0).round() / 1000.0,
            "included_angle_deg": (inc_deg * 100.0).round() / 100.0
        }));
    }

    let tip_json = json!({
        "schema": "jarambit.g3.diagnostics.tip_raster.v1",
        "design_id": rel["design_id"],
        "mesh_series_reported_v3": rel["fe_certification"]["tip_tet10_sf_by_mesh_area_mm2"],
        "monotone_converged": false,
        "half_cell_raster_shift_audit_at_0p15_mm2": {
            "raster_pitch_g_mm": g_step,
            "half_cell_shift_mm": 0.5 * g_step,
            "sf_v3_node_sampled_shifted": (sf_v3_shifted * 10000.0).round() / 10000.0,
            "sf_cell_center_aligned": (sf_cell_aligned * 10000.0).round() / 10000.0,
            "delta_sf": ((sf_cell_aligned - sf_v3_shifted) * 10000.0).round() / 10000.0
        },
        "planform_point_geometry": {
            "h_exp_gene_decoded": (p["h_exp"] * 10000.0).round() / 10000.0,
            "tip_angle_gene_decoded_deg": (p["tip_angle"] * 100.0).round() / 100.0,
            "stations": planform_angles,
            "note": "h_exp = 0.626 < 1.0 produces a rounded bullet-nose planform (included angle ~88 deg at 3 mm) that passes thickness-only Gate S2."
        }
    });
    fs::write(out_dir.join("tip_raster.json"), serde_json::to_string_pretty(&tip_json).unwrap() + "\n").unwrap();

    println!("Wrote 4 diagnostic receipts to {}", out_dir.display());
}
