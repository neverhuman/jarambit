//! The G5.1 evaluation cascade: G5.0's tiers (`jarambit_g5::g5_qd`) on the G5.1 SDF (`sdf::build_g51_sdf`), with
//! the hand-zone block (`hand`) written into every record. Tier 0 is G5.0's analytic tier plus the block; Tiers 1
//! and 2 and the rank promotion are G5.0's code, copied because G5.0 builds its own grid inside them, with the G5.1
//! grid, decoder hash and block. Records keep G5.0's type (`G5EvalRecord`), so G5.0's archive, objective and row
//! format serve G5.1 unchanged; the decoder hash tells the two apart.

use crate::hand::{hand_zone_analytic, HandZone};
use crate::hash::g51_decoder_sha256;
use crate::sdf::build_g51_sdf;
use jarambit_g5::ergolight::{ErgolightInputs, ErgolightScore};
use jarambit_g5::g5_genome::{G5Genome, FIXED_BORE_D_MM};
use jarambit_g5::g5_qd::{
    evaluate_tier0_analytic, lloyd_cvt_centroids, measure_blade_edge_apex_and_angle, measure_point_planform,
    CvtMapElitesArchive, EvalTier, G5EvalRecord, MassGridAgreement,
};
use jarambit_g5::g5_sandwich::{solve_ring_band_t6, SandwichPlate, SandwichSection};
use jarambit_g5::g5_sdf::{
    build_blade_profile_2d, effective_blade_rho, eval_blade_thickness_and_sdf2, eval_grip_slice,
    extract_watertight_mesh, solid_fraction, AmConnectivityStats, BladeProfile2D, VoxelGrid3D, GUARD_X_MM,
};
use jarambit_g5::material::{self, FatigueClass, Wall, GROUND_WALL, HOLLOW_BLADE_SKIN};
use jarambit_crescent::export;
use jarambit_physics::fine::mesh_domain;
use std::collections::BTreeMap;

/// `W_ERGOLIGHT_v2` of a record's measurements (`objective::score_v2`, as G5.0's `score_record`).
pub fn score_record(
    inputs: &ErgolightInputs,
    loads: &jarambit_g5::loads::G5Loads,
    am: &AmConnectivityStats,
    blade: &BladeProfile2D,
) -> ErgolightScore {
    let point = measure_point_planform(blade);
    jarambit_g5::objective::score_v2(&jarambit_g5::objective::ObjectiveInputs { inputs, loads, point: &point, am, blade })
}

/// Tier 0: G5.0's analytic tier, then the hand-zone block, the G5.1 decoder hash and the score.
pub fn evaluate_tier0(genome: &G5Genome) -> (G5EvalRecord, HandZone) {
    let mut rec = evaluate_tier0_analytic(genome);
    let p = genome.decode();
    let blade = build_blade_profile_2d(&p);
    let hand = hand_zone_analytic(&p, &blade, &rec.loads);
    hand.apply(&mut rec.loads);
    rec.decoder_sha256 = g51_decoder_sha256();
    rec.score = score_record(&rec.inputs, &rec.loads, &rec.am_connectivity, &blade);
    (rec, hand)
}

/// The wall class and thickness that govern the blade's strength: the ground solid core, or the skin of a hollow blade.
fn blade_wall(p: &BTreeMap<&'static str, f64>, rho_blade_eff: f64) -> (f64, Wall) {
    if rho_blade_eff < 0.95 {
        (p["skin_t"], HOLLOW_BLADE_SKIN)
    } else {
        (p["t_core"], GROUND_WALL)
    }
}

/// Blade first-yield allowable at the service temperature (MPa).
fn blade_yield_allowable(p: &BTreeMap<&'static str, f64>, rho_blade_eff: f64) -> f64 {
    let (t, wall) = blade_wall(p, rho_blade_eff);
    material::yield_allowable(t, wall)
}

/// Fatigue safety factor of the blade under a 0 → `sigma_peak` cycle (R = 0).
fn blade_fatigue_sf(sigma_peak: f64, p: &BTreeMap<&'static str, f64>) -> f64 {
    let (t, wall) = blade_wall(p, blade_rho(p));
    let class: FatigueClass = material::fatigue_class(t, wall);
    material::fatigue_sf(0.5 * sigma_peak, 0.5 * sigma_peak, class)
}

fn blade_rho(p: &BTreeMap<&'static str, f64>) -> f64 {
    effective_blade_rho(p)
}

/// Build the AM field at h and at h/2 and compare the partial-volume masses. Finalists must agree within
/// `[mass] grid_agreement_max_frac` (2 %); a design that does not is reported, never ranked on the coarse mass.
pub fn mass_grid_agreement(genome: &G5Genome, h: f64) -> MassGridAgreement {
    let coarse = build_g51_sdf(genome, h, true);
    let cx = genome.decode()["cx"];
    let (m_h, ..) = coarse.am_mass_com_inertia(cx);
    let sealed_h = coarse.am_connectivity.sealed_void_mm3;
    drop(coarse);
    let fine = build_g51_sdf(genome, 0.5 * h, true);
    let (m_h2, ..) = fine.am_mass_com_inertia(cx);
    MassGridAgreement {
        h_mm: h,
        mass_h_g: m_h,
        mass_half_h_g: m_h2,
        rel_diff: (m_h - m_h2).abs() / m_h2.max(1e-9),
        sealed_void_h_mm3: sealed_h,
        sealed_void_half_h_mm3: fine.am_connectivity.sealed_void_mm3,
    }
}

/// Tier-1 evaluation: builds the full-knife 3D SDF (`build_g5_sdf`), integrates `mass_g`, `com_x`,
/// `com_y`, and `spin_i_g_mm2` strictly from the AM-printed voxels (`d_am`, after
/// `enforce_am_connectivity` fills any unported cavities solid, `B5`), measures `swell_perimeter_mm`,
/// `swell_h_over_t`, `l_g_mm`, `guard_h_mm`, and `grip_sf_min` from the 3D voxel contour (`I4`, `N8`),
/// and solves the `T6` ring submodel (`N7` bilinear homogenized thickness) and multi-RHS `SandwichPlate`
/// blade submodel (`B2`, `I6` residual gate) with fail-closed zero credit on solver failure (`B3`, `I2`).
pub fn evaluate_tier1_sdf_and_plate(genome: &G5Genome, h_vox: f64) -> (G5EvalRecord, HandZone) {
    let (rec, hand, _) = evaluate_tier1_with_grid(genome, h_vox);
    (rec, hand)
}

/// Tier 1 with its G5.1 grid (G5.0's `evaluate_tier1_with_grid` on `build_g51_sdf`).
pub fn evaluate_tier1_with_grid(genome: &G5Genome, h_vox: f64) -> (G5EvalRecord, HandZone, VoxelGrid3D) {
    let (mut rec, hand) = evaluate_tier0(genome);
    let p = genome.decode();
    let grid = build_g51_sdf(genome, h_vox, true);

    // Envelope and AM volumes by partial volume, for the porosity descriptor
    let mut env_vox = 0.0_f64;
    let mut am_vox = 0.0_f64;
    let d_am = grid.d_am.as_deref().unwrap_or(&grid.d);
    for (&dv, &amv) in grid.d.iter().zip(d_am.iter()) {
        env_vox += solid_fraction(dv, grid.h);
        am_vox += solid_fraction(amv, grid.h);
    }
    rec.descriptors.porosity_frac = if env_vox > 0.0 {
        (1.0 - am_vox / env_vox).clamp(0.0, 0.85)
    } else {
        0.0
    };
    rec.am_connectivity = grid.am_connectivity;

    // Integrate full-knife AM mass, CoM, and spin inertia strictly from the 3D AM voxel grid (B5)
    let cx = p["cx"];
    let r_b = FIXED_BORE_D_MM * 0.5;
    let r_od = p["R_od"];
    let (am_mass_g, com_x, _com_y, spin_i_g_mm2) = grid.am_mass_com_inertia(cx);
    rec.inputs.mass_g = am_mass_g;
    rec.descriptors.mass_g = am_mass_g;
    let x_idx = p["x_t"] + p["centre_idx"] + p["off_idx"];
    rec.inputs.com_to_index_mm = (com_x - x_idx).abs();
    rec.inputs.spin_i_g_mm2 = spin_i_g_mm2;

    // Measure hand-zone swell section, contour metrics, and voxel-section `grip_sf_min` from the 3D SDF grid (I4, N8, N13)
    let hz_meas = grid.measure_handzone_masses_and_swell(&p);
    rec.inputs.swell_perimeter_mm = hz_meas.swell_perimeter_mm;
    rec.inputs.swell_h_over_t = hz_meas.swell_h_over_t;
    rec.inputs.l_g_mm = hz_meas.l_g_mm;
    rec.inputs.guard_h_mm = hz_meas.guard_h_mm;
    rec.inputs.grip_sf_min = hz_meas.grip_sf_min;
    rec.descriptors.grip_roundness_h_over_t = hz_meas.swell_h_over_t;

    // Re-measure Gate S1 (`max_apex_mm`), S3 (`max_half_angle_deg`), `fairness`, and `karambit_ok` on the built blade (`B2`, `S5-4`, `S5-M1`)
    let blade = build_blade_profile_2d(&p);
    let (apex_t1, ang_t1) = measure_blade_edge_apex_and_angle(&p, &blade);
    rec.inputs.max_apex_mm = apex_t1;
    rec.inputs.max_half_angle_deg = ang_t1;
    // Re-measure fairness on the 3D voxel `z = 0` mid-plane outline + smooth blade profile (`S5-M1`: 0.0 floor)
    let k_mid = grid
        .z
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.abs().total_cmp(&b.abs()))
        .map(|(k, _)| k)
        .unwrap_or(grid.nz / 2);
    let mut y_bot_3d = Vec::new();
    for i in 0..grid.nx {
        if grid.x[i] < p["x_t"] + 4.0 || grid.x[i] > GUARD_X_MM {
            continue;
        }
        for j in 0..grid.ny {
            if grid.d_pre_tex[(i * grid.ny + j) * grid.nz + k_mid] < 0.0 {
                y_bot_3d.push((grid.x[i], grid.y[j]));
                break;
            }
        }
    }
    let mut d2_3d = 0.0_f64;
    if y_bot_3d.len() >= 5 {
        for m in 2..(y_bot_3d.len() - 2) {
            let ds = (y_bot_3d[m + 2].0 - y_bot_3d[m - 2].0).max(0.5);
            let d2y = (y_bot_3d[m + 2].1 - 2.0 * y_bot_3d[m].1 + y_bot_3d[m - 2].1) / (ds * ds);
            d2_3d += d2y * d2y;
        }
        d2_3d = (d2_3d / (y_bot_3d.len() as f64)).sqrt();
    }
    rec.inputs.fairness = (rec.inputs.fairness - 0.05 * (d2_3d - 0.08).max(0.0)).clamp(0.0, 1.0);
    rec.inputs.karambit_ok = blade.tip_xy.1 <= -25.0 + 1e-6
        && (blade.x_tip - blade.x_heel) >= 60.0 - 1e-6
        && blade.reach_chord_mm <= 85.725 + 1e-6
        && blade.sharpened_arc_in_mm <= 101.6 + 1e-6
        && blade.sharpened_arc_sp_mm <= 101.6 + 1e-6
        && hz_meas.l_g_mm.is_finite();

    // Build ring + neck submodel mesh up to x = 46.5 with seat = (40.0, 46.0) (`evaluate.py` lines 238-240).
    // Fail-closed (B3): zero out Tier-0 analytic estimates before running the T6 solver.
    rec.inputs.ring_bearing_cap_n = 0.0;
    rec.inputs.ring_back_cap_n = 0.0;
    let x_clip = 46.5_f64;
    let th_join = p["junction_deg"].clamp(24.0, 45.0).to_radians();
    let mut outer = Vec::with_capacity(80);
    // 1. Rear ring arc from +th_join around pi to 2*pi - th_join (counter-clockwise)
    let n_arc = 44usize;
    for k in 0..=n_arc {
        let th = th_join
            + (k as f64 / n_arc as f64) * (2.0 * std::f64::consts::PI - 2.0 * th_join);
        let r_th = r_od + p["od_tear"] * 0.5 * (1.0 + th.cos()) + p["od_h2"] * (2.0 * th).cos();
        outer.push((cx + r_th * th.cos(), r_th * th.sin()));
    }
    // 2. Bottom neck & grip outline from x_join to x_clip (including neck_r smooth fillet, I2)
    let x_join = (cx + r_od * th_join.cos()).max(p["x_t"]);
    let y_join_bot = -r_od * th_join.sin();
    let y_join_top = r_od * th_join.sin();
    let neck_span = (0.8 * p["neck_r"]).clamp(2.5, 6.0);
    let n_neck = 18usize;
    for k in 1..=n_neck {
        let x = x_join + (k as f64 / n_neck as f64) * (x_clip - x_join);
        let gc = eval_grip_slice(&p, x);
        let w_fillet = (-(x - x_join) / neck_span).exp();
        let yb = gc.yb.min(y_join_bot * w_fillet + gc.yb * (1.0 - w_fillet));
        outer.push((x, yb));
    }
    // 3. Top grip & neck outline from x_clip back to x_join
    for k in (1..=n_neck).rev() {
        let x = x_join + (k as f64 / n_neck as f64) * (x_clip - x_join);
        let gc = eval_grip_slice(&p, x);
        let w_fillet = (-(x - x_join) / neck_span).exp();
        let yt = gc.yt.max(y_join_top * w_fillet + gc.yt * (1.0 - w_fillet));
        outer.push((x, yt));
    }
    let hole: Vec<(f64, f64)> = (0..48)
        .map(|k| {
            let th = (k as f64) * 2.0 * std::f64::consts::PI / 48.0;
            (cx + r_b * th.cos(), r_b * th.sin())
        })
        .collect();
    let holes = [hole];
    if let Some(mesh) = mesh_domain(&outer, &holes, 0.35).or_else(|| mesh_domain(&outer, &holes, 0.65)) {
        // N7: Use the homogenized 2D `skin + rho^1.8` thickness map interpolated bilinearly at quadrature points (`evaluate.py::tmap_eff` parity)
        let t_at = |x: f64, y: f64| -> f64 { grid.ring_tmap_eff_bilinear_at(x, y).max(0.3) };
        let t_elem: Vec<f64> = mesh
            .tris
            .iter()
            .map(|t| {
                let xm = (mesh.xy[t[0]].0 + mesh.xy[t[1]].0 + mesh.xy[t[2]].0) / 3.0;
                let ym = (mesh.xy[t[0]].1 + mesh.xy[t[1]].1 + mesh.xy[t[2]].1) / 3.0;
                t_at(xm, ym)
            })
            .collect();
        if let Some(rb_res) =
            solve_ring_band_t6(&mesh, &t_elem, &t_at, cx, r_b, r_od, p["web_t"], (40.0, 46.0))
        {
            rec.inputs.ring_bearing_cap_n = rb_res.bearing.capacity_n;
            rec.inputs.ring_back_cap_n = rb_res.back.capacity_n;
            let ring = &jarambit_g5::requirements::rs_v5().loads.ring;
            rec.loads.ring_bearing_sf = rb_res.bearing.capacity_n / ring.bearing_n;
            rec.loads.ring_back_sf = rb_res.back.capacity_n / ring.back_n;
            rec.loads.ring_any_direction_sf = rb_res.any_direction_min_capacity_n / ring.any_direction_n;
        }
    }

    // RS-karambit-v5 blade cases on the fine whole-blade plate (`loads::plate_blade_cases`). Fail closed: if the mesh
    // or a checked solve fails, the blade cases score zero (the analytic values can be more optimistic than the plate)
    if !jarambit_g5::loads::plate_blade_cases(&p, &blade, &mut rec.loads) {
        let l = &mut rec.loads;
        l.lateral_limit_sf = 0.0;
        l.lateral_ultimate_sf = 0.0;
        l.twist_limit_sf = 0.0;
        l.pull_sf = 0.0;
        l.fatigue_sf = 0.0;
        l.seq_lateral = 0.0;
        l.seq_twist = 0.0;
        l.seq_pull = 0.0;
    }

    // Solve SandwichPlate on the blade domain for all plate structural terms (`B2`, `B3`, `I6`, `N1`).
    // Fail-closed: zero out all Tier-0 analytic blade estimates before running the checked plate solver.
    rec.inputs.lat_rob_cert_n = 0.0;
    rec.inputs.strict_lat_n = 0.0;
    rec.inputs.twist_body_sf = 0.0;
    rec.inputs.twist_tip_sf = 0.0;
    rec.inputs.twist_tipzone_strict_sf = 0.0;
    rec.inputs.hm_twist_sf = 0.0;
    rec.inputs.hm_spine_sf = 0.0;
    rec.inputs.seq_ratio = 0.0;
    rec.inputs.fat_sf = 0.0;
    let mut b_outer = Vec::new();
    for i in (0..blade.xs.len()).step_by(4) {
        b_outer.push((blade.xs[i], blade.y_in[i]));
    }
    for i in (0..blade.xs.len()).step_by(4).rev() {
        b_outer.push((blade.xs[i], blade.y_sp[i]));
    }
    if let Some(b_mesh) = mesh_domain(&b_outer, &[], 3.5) {
        let skin_t = p["skin_t"];
        let rho_blade_eff = effective_blade_rho(&p);
        let blade_core_e = material::core_relative_stiffness(rho_blade_eff, p["lat_cell"], p["lat_mix"]);
        let secs: Vec<SandwichSection> = b_mesh
            .tris
            .iter()
            .map(|t| {
                let xm = (b_mesh.xy[t[0]].0 + b_mesh.xy[t[1]].0 + b_mesh.xy[t[2]].0) / 3.0;
                let ym = (b_mesh.xy[t[0]].1 + b_mesh.xy[t[1]].1 + b_mesh.xy[t[2]].1) / 3.0;
                let (th, _) = eval_blade_thickness_and_sdf2(&p, &blade, xm, ym);
                SandwichSection::from_skin_and_lattice(th.max(0.3), skin_t, blade_core_e, 0.0)
            })
            .collect();
        let s_plate = SandwichPlate::new(b_mesh.clone(), p["t_core"], material::e_mpa(), material::nu())
            .with_sections(secs);
        let n = s_plate.n();
        let mut fixed = vec![false; n * 5];
        let mut n_fix = 0usize;
        for (i, &(x, _)) in s_plate.mesh.xy.iter().enumerate() {
            if x <= blade.x_heel + 3.5 {
                n_fix += 1;
                for c in 0..5 {
                    fixed[i * 5 + c] = true;
                }
            }
        }
        if n_fix >= 3 {
            if let Some(sys) = s_plate.system(&fixed) {
                let allow_plate = blade_yield_allowable(&p, rho_blade_eff);

                // Load case 1: 1 N lateral tip load (`dof = 2` on tip patch `r <= 3.0 mm`)
                let mut f_lat = vec![0.0; n * 5];
                let mut patch_nodes = Vec::new();
                for (i, &(x, y)) in s_plate.mesh.xy.iter().enumerate() {
                    if (x - blade.tip_xy.0).hypot(y - blade.tip_xy.1) <= 3.0 {
                        patch_nodes.push(i);
                    }
                }
                if !patch_nodes.is_empty() {
                    let w_p = 1.0 / (patch_nodes.len() as f64);
                    for &nd in &patch_nodes {
                        f_lat[nd * 5 + 2] = w_p;
                    }
                    if let Some((u_lat, _)) = s_plate.solve_checked(&sys, &f_lat) {
                        let (_, sb) = s_plate.stresses(&u_lat);
                        let mut pk_sb_rob = 0.0_f64;
                        let mut pk_sb_strict = 0.0_f64;
                        // S5-1, S6-1: canonical 4.0 mm tip St-Venant exclusion with NO thickness filter and NO synthetic shear_warp
                        for (e, &s_val) in sb.iter().enumerate() {
                            let c = s_plate.centroid(e);
                            let d_tip = (c.0 - blade.tip_xy.0).hypot(c.1 - blade.tip_xy.1);
                            if d_tip >= 4.0 && c.0 >= blade.x_heel + 4.0 {
                                pk_sb_rob = pk_sb_rob.max(s_val);
                            }
                            if d_tip >= 4.0 && c.0 >= blade.x_heel + 3.5 {
                                pk_sb_strict = pk_sb_strict.max(s_val);
                            }
                        }
                        if pk_sb_rob > 1e-6 && pk_sb_rob.is_finite() {
                            let cap_plate = (allow_plate / pk_sb_rob).clamp(0.0, 1200.0);
                            let cap_strict = if pk_sb_strict > 1e-6 && pk_sb_strict.is_finite() {
                                (allow_plate / pk_sb_strict).clamp(0.0, cap_plate)
                            } else {
                                cap_plate
                            };
                            rec.inputs.lat_rob_cert_n = cap_plate;
                            rec.inputs.strict_lat_n = cap_strict;
                            if cap_plate > 1.0 {
                                rec.inputs.seq_ratio =
                                    ((rec.inputs.grip_sf_min * 260.0) / cap_plate).clamp(0.0, 2.5);
                                rec.inputs.fat_sf = blade_fatigue_sf(180.0 * pk_sb_rob, &p);
                            }
                        }
                    }
                }

                // Load case 2: 1820 N*mm tip twist couple (`f_twist`, opposing +/-z forces across inner/spine edges near tip)
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
                if !twist_in.is_empty() && !twist_sp.is_empty() {
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
                    if let Some((u_tw, _)) = s_plate.solve_checked(&sys, &f_twist) {
                        let (_, sb_tw) = s_plate.stresses(&u_tw);
                        let mut pk_body = 0.0_f64;
                        let mut pk_tip = 0.0_f64;
                        let mut pk_tip_strict = 0.0_f64;
                        for (e, &s_val) in sb_tw.iter().enumerate() {
                            let c = s_plate.centroid(e);
                            let d_tip = (c.0 - blade.tip_xy.0).hypot(c.1 - blade.tip_xy.1);
                            if d_tip > 16.0 && c.0 >= blade.x_heel + 5.0 {
                                pk_body = pk_body.max(s_val);
                            } else if (5.0..=16.0).contains(&d_tip) {
                                pk_tip = pk_tip.max(s_val);
                            }
                            if (4.0..=16.0).contains(&d_tip) {
                                pk_tip_strict = pk_tip_strict.max(s_val);
                            }
                        }
                        if pk_body > 1e-6 && pk_body.is_finite() {
                            let sf_body = (allow_plate / pk_body).clamp(0.0, 4.0);
                            rec.inputs.twist_body_sf = sf_body;
                            // `S2`: human-max twist = the same plate solve's body peak scaled from 1.82 to 16.2 N*m
                            rec.inputs.hm_twist_sf =
                                (allow_plate / pk_body * (1820.0 / 16_200.0)).clamp(0.0, 3.0);
                        }
                        if pk_tip > 1e-6 && pk_tip.is_finite() {
                            let sf_tip = (allow_plate / pk_tip).clamp(0.0, 3.0);
                            rec.inputs.twist_tip_sf = sf_tip;
                            let sf_strict = if pk_tip_strict > 1e-6 && pk_tip_strict.is_finite() {
                                (allow_plate / pk_tip_strict).clamp(0.0, sf_tip)
                            } else {
                                sf_tip
                            };
                            rec.inputs.twist_tipzone_strict_sf = sf_strict;
                        }
                    }
                }

                // Load case 3: 3427 N in-plane spine load (`dof = 1`, `-y` distributed along spine nodes)
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
                if !spine_nodes.is_empty() {
                    let f_per_node = -3427.0 / (spine_nodes.len() as f64);
                    for &nd in &spine_nodes {
                        f_spine[nd * 5 + 1] = f_per_node;
                    }
                    if let Some((u_sp, _)) = s_plate.solve_checked(&sys, &f_spine) {
                        let (sm_sp, _) = s_plate.stresses(&u_sp);
                        let mut pk_sm = 0.0_f64;
                        for (e, &s_val) in sm_sp.iter().enumerate() {
                            let c = s_plate.centroid(e);
                            let d_tip = (c.0 - blade.tip_xy.0).hypot(c.1 - blade.tip_xy.1);
                            if d_tip >= 6.0 && c.0 >= blade.x_heel + 6.0 {
                                pk_sm = pk_sm.max(s_val);
                            }
                        }
                        if pk_sm > 1e-6 && pk_sm.is_finite() {
                            rec.inputs.hm_spine_sf = (allow_plate / pk_sm).clamp(0.0, 3.0);
                        }
                    }
                }
            }
        }
    }

    rec.tier = EvalTier::Tier1SdfAndPlate;
    hand.apply(&mut rec.loads);
    rec.score = score_record(&rec.inputs, &rec.loads, &rec.am_connectivity, &blade);
    (rec, hand, grid)
}

/// Tier-2 evaluation for top elites: reuses the Tier-1 `VoxelGrid3D` (`N12`), extracts the watertight
/// 3D mesh (`extract_watertight_mesh`), checks exact manifold topology (`census == (0, 0, 1)`), and
/// runs the fail-closed 3D `TET10` tip submodel (`solve_g5_solid_tet10_strict`).
/// Fail-closed (`B2`, `B3`): `solid_tip_sf` starts at `0.0` and is only populated if the strict
/// Cholesky solve (`true_rel_res <= 1e-10`) succeeds; `dolfinx_rel_err` remains `1.0` unless an
/// independent DOLFINx cross-check is provided (never substituting the linear-solver residual).
pub fn evaluate_tier2_elites(genome: &G5Genome, h_vox: f64) -> (G5EvalRecord, HandZone) {
    let (rec1, hand, grid) = evaluate_tier1_with_grid(genome, h_vox);
    evaluate_tier2_from_tier1(genome, rec1, hand, &grid)
}

/// Tier 2 from a Tier-1 record and its grid (G5.0's `evaluate_tier2_from_tier1`).
pub fn evaluate_tier2_from_tier1(
    genome: &G5Genome,
    mut rec: G5EvalRecord,
    hand: HandZone,
    grid: &VoxelGrid3D,
) -> (G5EvalRecord, HandZone) {
    let p = genome.decode();
    let solid = extract_watertight_mesh(grid, true);
    let (boundary, non_manifold, dmax) = export::census(&solid.tris);
    rec.inputs.watertight = !solid.tris.is_empty() && boundary == 0 && non_manifold == 0 && dmax == 1;
    rec.inputs.solid_tip_sf = 0.0;
    rec.inputs.dolfinx_rel_err = 1.0;

    // Run fail-closed 3D TET10 tip submodel on the blade surface law (P6: full xs outline containing exact tip_xy,
    // max triangle area `TIP_FE_MAX_AREA_MM2`). r2: on the sharp EDGE-spec points 0.4 mm^2 was 3-19 % off a 0.15 mm^2
    // mesh, while 0.15 and 0.08 mm^2 agree within 0.7-8.2 % on the 8 edge-pilot top designs, so 0.15 is the production mesh.
    let blade = build_blade_profile_2d(&p);
    let mut b_outer = Vec::with_capacity(blade.xs.len() * 2);
    for i in 0..blade.xs.len() {
        b_outer.push((blade.xs[i], blade.y_in[i]));
    }
    for i in (0..blade.xs.len()).rev() {
        b_outer.push((blade.xs[i], blade.y_sp[i]));
    }
    // RS-karambit-v5 tip cases on the TET10 solid (`loads::tip_solid_cases`). Fail closed: a finalist's tip is
    // judged on the solid, so a failed mesh or solve scores the tip cases zero. (G3's own TET10 solve, a 1.82 N m
    // twist with G3 material whose result W_ERGOLIGHT_v2 does not read, no longer runs.)
    match jarambit_g5::loads::tip_solid_cases(&p, &blade) {
        Some(ts) => {
            rec.loads.tip_axial_sf = ts.axial_sf;
            rec.loads.tip_lateral_sf = ts.lateral_sf;
            rec.loads.tip_twist_sf = ts.twist_sf;
            rec.loads.tip_model = jarambit_g5::loads::TipModel::Solid;
        }
        None => {
            rec.loads.tip_axial_sf = 0.0;
            rec.loads.tip_lateral_sf = 0.0;
            rec.loads.tip_twist_sf = 0.0;
            rec.loads.tip_model = jarambit_g5::loads::TipModel::Solid;
        }
    }

    rec.tier = EvalTier::Tier2Full3D;
    hand.apply(&mut rec.loads);
    rec.score = score_record(&rec.inputs, &rec.loads, &rec.am_connectivity, &build_blade_profile_2d(&p));
    (rec, hand)
}

#[derive(Debug, Clone)]
pub struct RankPromotion {
    pub tier1_top_frac: f64,
    pub tier2_top_frac: f64,
    t0: std::collections::VecDeque<f64>,
    t1: std::collections::VecDeque<f64>,
}

impl RankPromotion {
    pub const WINDOW: usize = 256;
    pub const WARMUP: usize = 16;

    pub fn new(tier1_top_frac: f64, tier2_top_frac: f64) -> Self {
        Self { tier1_top_frac, tier2_top_frac, t0: Default::default(), t1: Default::default() }
    }

    fn decide(window: &mut std::collections::VecDeque<f64>, frac: f64, score: f64) -> bool {
        let promote = if frac <= 0.0 {
            false
        } else if window.len() < Self::WARMUP || frac >= 1.0 {
            true
        } else {
            let mut v: Vec<f64> = window.iter().copied().collect();
            v.sort_by(f64::total_cmp);
            let k = (((1.0 - frac) * v.len() as f64).floor() as usize).min(v.len() - 1);
            score >= v[k]
        };
        window.push_back(score);
        if window.len() > Self::WINDOW {
            window.pop_front();
        }
        promote
    }

    /// Evaluate one design through the tiers its ranks earn.
    pub fn evaluate(&mut self, genome: &G5Genome, h_vox: f64) -> (G5EvalRecord, HandZone) {
        let (r0, hand0) = evaluate_tier0(genome);
        if !Self::decide(&mut self.t0, self.tier1_top_frac, r0.score.raw_score_100) {
            return (r0, hand0);
        }
        let (r1, hand1, grid) = evaluate_tier1_with_grid(genome, h_vox);
        if !Self::decide(&mut self.t1, self.tier2_top_frac, r1.score.raw_score_100) {
            return (r1, hand1);
        }
        evaluate_tier2_from_tier1(genome, r1, hand1, &grid)
    }
}

/// An empty G5.1 archive: G5.0's 1024-niche CVT archive (same centroids, insert and emitters), keyed to the G5.1
/// decoder hash, so it refuses G5.0 records exactly as G5.0's refuses records of another decoder. The schema is the
/// genome's (`G5.0`).
pub fn new_archive() -> CvtMapElitesArchive {
    CvtMapElitesArchive {
        schema: jarambit_g5::g5_genome::G5_SCHEMA.to_string(),
        decoder_sha256: g51_decoder_sha256(),
        centroids: lloyd_cvt_centroids(1024),
        elites: BTreeMap::new(),
    }
}

/// G5.0's `verify_integrity` against the G5.1 decoder hash: niche keys, genome validity and hashes, record schema
/// and decoder, and the CVT assignment of every elite.
pub fn verify_archive(a: &CvtMapElitesArchive) -> Result<(), String> {
    if a.schema != jarambit_g5::g5_genome::G5_SCHEMA {
        return Err(format!("archive schema mismatch: '{}'", a.schema));
    }
    let dec = g51_decoder_sha256();
    if a.decoder_sha256 != dec {
        return Err(format!("archive decoder_sha256 '{}' is not G5.1's '{dec}'", a.decoder_sha256));
    }
    for (&nid, e) in &a.elites {
        if e.niche_id != nid {
            return Err(format!("archive key {nid} != elite.niche_id {}", e.niche_id));
        }
        e.genome.validate()?;
        if e.record.genome_sha256 != e.genome.sha256() {
            return Err(format!("niche {nid}: record genome_sha256 does not match its genome"));
        }
        if e.record.schema != a.schema || e.record.decoder_sha256 != dec {
            return Err(format!("niche {nid}: record schema or decoder differs from the archive"));
        }
        if a.niche_of(&e.record.descriptors) != nid {
            return Err(format!("niche {nid}: descriptors map to niche {}", a.niche_of(&e.record.descriptors)));
        }
    }
    Ok(())
}
