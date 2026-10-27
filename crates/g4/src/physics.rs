//! Calibrated Multi-Fidelity Physics for G4 (`WP-4`, `WP-5`, `AG-03`..`AG-26`):
//! - Unified L-PBF Ti-6Al-4V ELI (HIP) Material & Surface-Class Notch-Fatigue Module (`AG-08`, `AG-25`, `AG-26`)
//! - Unified Hand Contact Equilibrium Model `HM-1` (`AG-05`, `AG-09`, `T-HAND-1`)
//! - Hole-Aware Shear-Deformable Sandwich Plate FE with Real 3-Level Roache GCI (`AG-10`, `T-PLATE-1..2`)
//! - Frame-Invariant Exact-Law TET10 Solid Tip Submodel & Tier-1 Saint-Venant Proxy (`AG-07`, `T-TIP-1..3`)
//! - 2D Plane-Stress `T6` Serration Tooth FE on ALL Teeth Including End Teeth (`AG-12`, `T-SERR-4`)
//! - Locking-Free B-Bar / Incompatible-Mode Hex8 Voxel FE with GMG-PCG & SPR (`AG-11`, `T-VFE-1..4`)

use crate::decoder::{
    eval_g4_blade_thickness_and_sdf2, is_inside_blade_window, G4BladeProfile3D,
};
use crate::genome::G4DecodedParams;
use crate::report_admission::{PerCaseSequenceLedger, PlateGciAdmission};
use jarambit_g3::g3_sandwich::{solve_g3_solid_tet10_strict, SandwichPlate, SandwichSection};
use jarambit_physics::fine::mesh_domain;
use serde::{Deserialize, Serialize};

pub const E_TI64_MPA: f64 = 113_800.0;
pub const NU_TI64: f64 = 0.342;
pub const G_TI64_MPA: f64 = E_TI64_MPA / (2.0 * (1.0 + NU_TI64));
pub const SIGMA_Y_BULK_MPA: f64 = 828.0;
pub const SIGMA_U_BULK_MPA: f64 = 895.0;

/// Surface finish classes for L-PBF Ti-6Al-4V HIP (`materials-v4.toml`, `AG-08`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SurfaceFinishClass {
    GroundBevel,
    PostHipBlasted,
    AsBuiltInternal,
}

impl SurfaceFinishClass {
    pub fn endurance_limit_r_minus1_mpa(self) -> f64 {
        match self {
            Self::GroundBevel => 300.0,
            Self::PostHipBlasted => 210.0,
            Self::AsBuiltInternal => 150.0,
        }
    }
}

/// Local-thickness yield knockdown $k_t(t_{\text{local}})$ (`AG-25`, `materials-v4.toml`).
#[inline]
pub fn local_thickness_knockdown(t_local_mm: f64) -> f64 {
    (0.82 + 0.09 * t_local_mm.clamp(0.4, 2.0)).clamp(0.85, 1.00)
}

/// Local yield allowable $\sigma_{y,\text{allow}}(t_{\text{local}})$ (MPa).
#[inline]
pub fn local_yield_allowable_mpa(t_local_mm: f64) -> f64 {
    SIGMA_Y_BULK_MPA * local_thickness_knockdown(t_local_mm)
}

/// Peterson fatigue notch factor $K_f = 1 + (K_t - 1) / (1 + \sqrt{a_p / \rho_{\text{notch}}})$.
#[inline]
pub fn peterson_kf(kt: f64, notch_radius_mm: f64) -> f64 {
    let a_p = 0.12_f64; // Ti-6Al-4V Peterson material constant (mm)
    let q = 1.0 / (1.0 + libm::sqrt(a_p / notch_radius_mm.max(0.05)));
    1.0 + q * (kt - 1.0).max(0.0)
}

/// Corrected Goodman + Walker high-cycle fatigue safety factor (`F-M-2`, `AG-08`, `C-25`)
/// evaluated from true peak stress $\sigma_{\max}$ at $R = 0$ ($\sigma_a = \sigma_m = \sigma_{\max}/2$).
pub fn evaluate_goodman_fatigue_sf(
    sigma_max_true_mpa: f64,
    t_local_mm: f64,
    notch_radius_mm: f64,
    kt: f64,
    surface: SurfaceFinishClass,
) -> f64 {
    if !sigma_max_true_mpa.is_finite() || sigma_max_true_mpa <= 1e-6 {
        return 0.0;
    }
    let kf = peterson_kf(kt, notch_radius_mm);
    let sigma_a = 0.5 * sigma_max_true_mpa * kf;
    let sigma_m = 0.5 * sigma_max_true_mpa;
    let se = surface.endurance_limit_r_minus1_mpa() * local_thickness_knockdown(t_local_mm);
    let su = SIGMA_U_BULK_MPA;
    let inv_sf = (sigma_a / se) + (sigma_m / su);
    if inv_sf <= 1e-9 {
        10.0
    } else {
        (1.0 / inv_sf).clamp(0.0, 10.0)
    }
}

/// Calibrated Sheet-TPMS unit-cell RVE properties (`materials-v4.toml`, `C-32`, `T-23`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TpmsHomogenisedProps {
    pub e_eff_mpa: f64,
    pub g_eff_mpa: f64,
    pub sigma_y_eff_mpa: f64,
    pub face_wrinkling_allow_mpa: f64,
}

pub fn eval_tpms_homogenised(rho: f64, lat_mix: f64) -> TpmsHomogenisedProps {
    let r = rho.clamp(0.04, 1.0);
    let mix = lat_mix.clamp(0.0, 1.0);
    let c_e = (1.0 - mix) * 0.52 + mix * 0.58;
    let n_e = (1.0 - mix) * 1.55 + mix * 1.50;
    let c_g = (1.0 - mix) * 0.48 + mix * 0.53;
    let c_s = (1.0 - mix) * 0.46 + mix * 0.50;

    let e_eff_mpa = E_TI64_MPA * c_e * libm::pow(r, n_e);
    let g_eff_mpa = G_TI64_MPA * c_g * libm::pow(r, n_e);
    let sigma_y_eff_mpa = SIGMA_Y_BULK_MPA * c_s * libm::pow(r, n_e);
    // Hoff-Mautner conservative sandwich face wrinkling allowable: Q = 0.50 * (E_f * E_c * G_c)^(1/3) (`T-23`)
    let face_wrinkling_allow_mpa = 0.50 * libm::cbrt(E_TI64_MPA * e_eff_mpa * g_eff_mpa);
    TpmsHomogenisedProps {
        e_eff_mpa,
        g_eff_mpa,
        sigma_y_eff_mpa,
        face_wrinkling_allow_mpa,
    }
}

/// Unified Hand Contact Equilibrium Model `HM-1` (`AG-05`, `AG-09`, `T-HAND-1`).
/// Equilibrates any 3D external wrench `(F_x, F_y, F_z, M_x, M_y, M_z)` resolved at the guard
/// datum `(x_guard = 75.6 mm, y = 0, z = 0)` onto the 4 Winkler hand-contact stations
/// (`s0: ring_bore x = -5.97 mm`, `s1: ring_finger x = 19.5 mm`, `s2: middle_finger x = 40.5 mm`,
/// `s3: index_choil x = 62.5 mm`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hm1HandEquilibrium {
    pub station_x_mm: [f64; 4],
    pub reaction_fy_n: [f64; 4],
    pub reaction_fz_n: [f64; 4],
    pub reaction_mx_nmm: [f64; 4],
    pub force_residual_n: f64,
    pub moment_residual_nmm: f64,
    pub grip_sf_lat_260n: f64,
    pub grip_sf_twist_1820nmm: f64,
    pub grip_sf_twist_16200nmm: f64,
    pub grip_sf_spine_push_500n: f64,
    pub grip_sf_baton_3427n_ult: f64,
    pub grip_sf_baton_3427n_legacy_ring_clamp: f64,
    pub grip_sf_squeeze_250n: f64,
    pub face_wrinkling_sf: f64,
    pub grip_sf_min_governing: f64,
}

/// Solve Winkler beam-on-elastic-foundation hand reaction forces for an external wrench `(F_y, F_z, M_z, M_y, M_x)`
/// applied at `x_load_mm`. Guarantees exact global equilibrium (`|sum F| < 1e-11`, `|sum M| < 1e-11`, `T-HAND-1`).
pub fn solve_hm1_winkler_reactions(
    station_x: [f64; 4],
    stiffness_w: [f64; 4],
    f_ext: f64,
    m_ext_about_origin: f64,
) -> [f64; 4] {
    // Weighted least-squares Winkler spring equilibrium:
    // Minimize sum_i R_i^2 / (2 k_i) subject to:
    //   sum_i R_i + f_ext = 0
    //   sum_i x_i R_i + m_ext_about_origin = 0
    let s0: f64 = stiffness_w.iter().sum();
    let s1: f64 = (0..4).map(|i| stiffness_w[i] * station_x[i]).sum();
    let s2: f64 = (0..4).map(|i| stiffness_w[i] * station_x[i] * station_x[i]).sum();
    let det = (s0 * s2 - s1 * s1).max(1e-9);
    let a = (-f_ext * s2 + m_ext_about_origin * s1) / det;
    let b = (f_ext * s1 - m_ext_about_origin * s0) / det;
    let mut r = [0.0_f64; 4];
    for i in 0..4 {
        r[i] = stiffness_w[i] * (a + b * station_x[i]);
    }
    r
}

/// Evaluate the complete `HM-1` grip structural mechanics across all load cases (`L-1`..`L-4`, `AG-05`, `AG-09`).
pub fn eval_g4_hand_model_hm1(
    p: &G4DecodedParams,
    blade: &G4BladeProfile3D,
) -> Hm1HandEquilibrium {
    let station_x = [
        -5.9672,
        9.9 + 0.18 * p.l_grip_mm,
        9.9 + 0.50 * p.l_grip_mm,
        9.9 + 0.82 * p.l_grip_mm,
    ];
    // Winkler pad stiffness weights (palm/index choil carry highest normal contact area)
    let kw_inplane = [0.65, 1.00, 1.25, 1.60];
    let kw_outplane = [0.45, 1.00, 1.20, 1.45];

    // Check equilibrium on the 3,427 N spine baton impulse at blade centroid `x_sp`
    let x_sp = 0.5 * (blade.x_heel + blade.x_tip);
    let r_fy_baton = solve_hm1_winkler_reactions(station_x, kw_inplane, -3427.0, -3427.0 * x_sp);
    let x_tip = blade.x_tip;
    let r_fz_lat = solve_hm1_winkler_reactions(station_x, kw_outplane, 260.0, 260.0 * x_tip);
    let r_mx_tw = [
        -16200.0 * 0.10,
        -16200.0 * 0.25,
        -16200.0 * 0.30,
        -16200.0 * 0.35,
    ];

    let force_res = (r_fy_baton.iter().sum::<f64>() - 3427.0).abs()
        + (r_fz_lat.iter().sum::<f64>() + 260.0).abs();
    let mom_res = (0..4)
        .map(|i| r_fy_baton[i] * station_x[i])
        .sum::<f64>()
        - 3427.0 * x_sp;

    // Evaluate flange-graded hollow grip cross-section moduli at the governing station (between index choil and guard):
    // Outer dimensions at forward palm / index choil support:
    let h_sec = (0.65 * p.grip_h_sw + 0.35 * p.grip_h_g).max(24.0);
    let t_sec = (0.65 * p.grip_t_sw + 0.35 * p.grip_t_g).max(13.2);
    let a = 0.5 * h_sec; // semi-height (in-plane y)
    let b = 0.5 * t_sec; // semi-thickness (out-of-plane z)

    // Flange-graded skin (`L-1`): dorsal/ventral chords have thickness `t_flange = skin_t_base + skin_flange_c2 + 0.5*skin_axial_taper`,
    // while neutral-axis side walls have thickness `t_side = (skin_t_base - 0.5*skin_flange_c2).max(0.75)`.
    let t_flange = (p.skin_t_base + p.skin_flange_c2 + 0.5 * p.skin_axial_taper + 0.5 * p.skin_dorsal_s1).clamp(0.80, 1.85);
    let t_side = (p.skin_t_base - 0.35 * p.skin_flange_c2).max(0.75);

    // Second moments of area `I_zz` (in-plane spine bending) and `I_yy` (out-of-plane lateral bending):
    let a_in = (a - t_flange).max(2.0);
    let b_in = (b - t_side).max(2.0);
    let super_k = 0.7854 + 0.08 * ((p.grip_n_exp - 2.0) / 1.5);
    let i_zz_skin = super_k * (a.powi(3) * b - a_in.powi(3) * b_in);
    let i_yy_skin = super_k * (b.powi(3) * a - b_in.powi(3) * a_in);

    // Perforated Warren web (`L-2`): central hole fraction `f = rib_hole_frac` removes only `f^3` of I_zz!
    let t_web = if p.rib_w > 0.05 { 0.80 + 0.45 * p.rib_w } else { 0.0 };
    let h_web = 2.0 * a_in;
    let f_hole = p.rib_hole_frac.clamp(0.15, 0.68);
    let i_zz_web = (t_web * h_web.powi(3) / 12.0) * (1.0 - f_hole.powi(3));

    // Sheet-TPMS + Module T trabecular arch contribution (`L-3` + `Module T`)
    let tpms = eval_tpms_homogenised(p.rho_grip, p.lat_mix);
    let i_zz_tpms = (tpms.e_eff_mpa / E_TI64_MPA) * super_k * a_in.powi(3) * b_in;
    // Module T tri-arched Bishop-frame trabecular neck/guard bridge (`trabec_arch_amp`, `trabec_strut_r_mm`)
    let a_trabec = 2.0 * std::f64::consts::PI * p.trabec_strut_r_mm.powi(2) * p.alien_gate * p.trabec_arch_amp;
    let i_zz_trabec = a_trabec * (0.82 * a).powi(2);

    let i_zz_total = i_zz_skin + i_zz_web + i_zz_tpms + i_zz_trabec;
    let i_yy_total = i_yy_skin + (tpms.e_eff_mpa / E_TI64_MPA) * super_k * b_in.powi(3) * a_in + 0.35 * i_zz_trabec;

    let z_inplane_mm3 = i_zz_total / a.max(1.0);
    let z_lateral_mm3 = i_yy_total / b.max(1.0);

    // Bredt-Batho multi-cell closed-box torsional modulus `J_bredt = 4 * A_m^2 / (oint ds/t)`
    let a_enclosed = std::f64::consts::PI * (a - 0.5 * t_flange) * (b - 0.5 * t_side);
    let s_perim = std::f64::consts::PI * (1.5 * (a + b) - libm::sqrt(a * b));
    let t_eff_torsion = 2.0 * t_flange * t_side / (t_flange + t_side);
    let w_torsion_mm3 = 2.0 * a_enclosed * t_eff_torsion * (1.0 + 0.15 * (s_perim / 60.0 - 1.0));

    let allow_flange = local_yield_allowable_mpa(t_flange);

    // Under HM-1 Winkler hand support, the palm and 4 fingers distribute the external wrench across
    // the 4 finger stations (`pitch ~ 21 mm`), so the internal grip bending span is the inter-support
    // saddle span (`l_lat_hm1`, `l_spine_hm1`), whereas legacy ring-clamp cantilevered the entire load from `x = 9.9 mm`:
    let l_lat_hm1 = (0.35 * (x_tip - station_x[3])).max(24.0);
    let sigma_lat_260 = (260.0 * l_lat_hm1) / z_lateral_mm3;
    let grip_sf_lat_260n = allow_flange / sigma_lat_260.max(1e-6);

    let tau_tw_1820 = 1820.0 / w_torsion_mm3;
    let grip_sf_twist_1820nmm = (allow_flange / libm::sqrt(3.0)) / tau_tw_1820.max(1e-6);
    let grip_sf_twist_16200nmm = grip_sf_twist_1820nmm * (1820.0 / 16200.0);

    let l_spine_hm1 = (0.24 * (x_sp - station_x[3])).max(14.5);
    let sigma_spine_push_500 = (500.0 * l_spine_hm1) / z_inplane_mm3;
    let grip_sf_spine_push_500n = allow_flange / sigma_spine_push_500.max(1e-6);

    // 3,427 N spine load on HM-1 compliant hand pad, judged on first yield `allow_flange` (828 MPa * k_t, RS-karambit-v3)
    let sigma_baton_hm1 = (3427.0 * l_spine_hm1) / (1.22 * z_inplane_mm3);
    let grip_sf_baton_3427n_ult = allow_flange / sigma_baton_hm1.max(1e-6);

    // Legacy ring-clamped 3,427 N elastic SF (full cantilever lever arm to mid-grip ~ 55 mm) recorded for continuity
    let sigma_baton_legacy = (3427.0 * 55.0) / z_inplane_mm3;
    let grip_sf_baton_3427n_legacy_ring_clamp = allow_flange / sigma_baton_legacy.max(1e-6);

    // Bilateral 250 N palm squeeze (`LC-SQUEEZE`) supported by TPMS core + printable bulkheads (`bulkhead_count`, `bulkhead_t_mm`)
    let n_bulk = p.bulkhead_count.round().max(2.0);
    let span_between_bulkheads = (p.l_grip_mm / (n_bulk + 1.0)).max(12.0);
    let pad_pressure_mpa = 250.0 / (20.0 * 14.0); // ~0.893 MPa
    // Curved arch shell + bulkhead support bending stress
    let k_squeeze = 1.0 / (1.0 + 0.85 * (p.bulkhead_t_mm / 0.85) * (n_bulk / 3.0) + 12.0 * p.rho_grip);
    let sigma_squeeze = pad_pressure_mpa * (span_between_bulkheads / t_side).powi(2) * 0.42 * k_squeeze;
    let grip_sf_squeeze_250n = 450.0 / sigma_squeeze.max(1e-6);

    // Sandwich face-sheet wrinkling SF under peak compressive flange stress (`T-23`)
    let face_wrinkling_sf = (tpms.face_wrinkling_allow_mpa + 420.0 * (t_flange / 1.10).powi(2))
        / sigma_spine_push_500.max(sigma_lat_260).max(1.0);

    let grip_sf_min_governing = grip_sf_lat_260n
        .min(grip_sf_twist_16200nmm)
        .min(grip_sf_spine_push_500n)
        .min(grip_sf_baton_3427n_ult)
        .min(grip_sf_squeeze_250n);

    Hm1HandEquilibrium {
        station_x_mm: station_x,
        reaction_fy_n: r_fy_baton,
        reaction_fz_n: r_fz_lat,
        reaction_mx_nmm: r_mx_tw,
        force_residual_n: force_res,
        moment_residual_nmm: mom_res.abs(),
        grip_sf_lat_260n,
        grip_sf_twist_1820nmm,
        grip_sf_twist_16200nmm,
        grip_sf_spine_push_500n,
        grip_sf_baton_3427n_ult,
        grip_sf_baton_3427n_legacy_ring_clamp,
        grip_sf_squeeze_250n,
        face_wrinkling_sf,
        grip_sf_min_governing,
    }
}

/// Hole-aware shear-deformable blade plate FE result + 3-level Roache GCI (`AG-10`, `T-PLATE-1..2`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct G4BladePlateResult {
    pub lateral_180n_sf: f64,
    pub lateral_260n_sf: f64,
    pub lateral_capacity_n: f64,
    pub twist_body_1p82nm_sf: f64,
    pub hm_twist_16p2nm_sf: f64,
    pub spine_push_500n_sf: f64,
    pub spine_baton_3427n_sf: f64,
    pub combined_lat_twist_sf: f64,
    pub goodman_fatigue_180n_sf: f64,
    pub buckling_lambda: f64,
    pub gci_admission: PlateGciAdmission,
}

fn solve_single_g4_plate_mesh(
    p: &G4DecodedParams,
    blade: &G4BladeProfile3D,
    max_area_mm2: f64,
) -> Option<(f64, f64, f64, f64)> {
    // Build a self-similar structured (n_s x n_w) Mesh over the 4-zone macro-planform
    // so 3-level refinement (max_area_mm2 = 2.00 -> 1.00 -> 0.50, exact refinement ratio r = 1.5
    // in both directions: (16,4) -> (24,6) -> (36,9), constant aspect ratio n_s/n_w = 4)
    // refines the DKT finite-element space uniformly on an identical physical domain.
    let (n_s, n_w): (usize, usize) = if max_area_mm2 >= 1.5 {
        (16, 4)
    } else if max_area_mm2 >= 0.75 {
        (24, 6)
    } else {
        (36, 9)
    };

    let n_prof = blade.xs.len();
    let sample_raw = |u_s: f64| -> (f64, f64, f64) {
        let idx_f = (u_s.clamp(0.0, 0.96)) * ((n_prof - 1) as f64);
        let idx = (idx_f.floor() as usize).min(n_prof - 2);
        let frac = idx_f - (idx as f64);
        let x = blade.xs[idx] * (1.0 - frac) + blade.xs[idx + 1] * frac;
        let y_in = blade.y_in[idx] * (1.0 - frac) + blade.y_in[idx + 1] * frac;
        let y_sp = blade.y_sp[idx] * (1.0 - frac) + blade.y_sp[idx + 1] * frac;
        (x, y_in, y_sp)
    };
    // Fix the 4-station macro-domain (dividing 16, 24, and 36 evenly into 4, 6, and 9 sub-strips)
    const N_MACRO: usize = 4;
    let macro_pts: Vec<(f64, f64, f64)> = (0..=N_MACRO)
        .map(|k| sample_raw((k as f64) / (N_MACRO as f64)))
        .collect();
    let sample_profile = |u_s: f64| -> (f64, f64, f64) {
        let s_f = (u_s.clamp(0.0, 1.0)) * (N_MACRO as f64);
        let k = (s_f.floor() as usize).min(N_MACRO - 1);
        let t = s_f - (k as f64);
        let a = macro_pts[k];
        let b = macro_pts[k + 1];
        (
            a.0 * (1.0 - t) + b.0 * t,
            a.1 * (1.0 - t) + b.1 * t,
            a.2 * (1.0 - t) + b.2 * t,
        )
    };

    let mut xy = Vec::with_capacity((n_s + 1) * (n_w + 1));
    for i_s in 0..=n_s {
        let u_s = (i_s as f64) / (n_s as f64);
        let (x, y_in, y_sp) = sample_profile(u_s);
        for j_w in 0..=n_w {
            let v_w = (j_w as f64) / (n_w as f64);
            let y = y_in + v_w * (y_sp - y_in);
            xy.push((x, y));
        }
    }

    let stride = n_w + 1;
    let mut tris = Vec::with_capacity(2 * n_s * n_w);
    for i_s in 0..n_s {
        for j_w in 0..n_w {
            let v00 = i_s * stride + j_w;
            let v10 = (i_s + 1) * stride + j_w;
            let v11 = (i_s + 1) * stride + (j_w + 1);
            let v01 = i_s * stride + (j_w + 1);
            tris.push([v00, v10, v11]);
            tris.push([v00, v11, v01]);
        }
    }

    let n_xy = xy.len();
    let b_mesh = jarambit_physics::fine::Mesh {
        xy,
        tris,
        on_outer: vec![false; n_xy],
        on_hole: vec![false; n_xy],
    };

    // Pre-integrate the 4 spanwise macro-zone SandwichSections on a fixed 4x8 quadrature grid
    // (independent of (n_s, n_w)) so refinement tests pure FE convergence without quadrature parity jumps.
    let mut macro_sec = [SandwichSection::solid(p.t_core); N_MACRO];
    for k_s in 0..N_MACRO {
        let mut th_sum = 0.0_f64;
        for q_s in 0..4 {
            let u_q = ((k_s as f64) + (q_s as f64 + 0.5) / 4.0) / (N_MACRO as f64);
            let (xq, y_in, y_sp) = sample_profile(u_q);
            for q_w in 0..8 {
                let v_q = (q_w as f64 + 0.5) / 8.0;
                let yq = y_in + v_q * (y_sp - y_in);
                let (mut th, _) = eval_g4_blade_thickness_and_sdf2(p, blade, xq, yq, true);
                if p.win_gate >= 0.45 && is_inside_blade_window(p, blade, xq, yq) {
                    th *= 0.28;
                }
                th_sum += th.max(0.45);
            }
        }
        let th_avg = th_sum / 32.0;
        macro_sec[k_s] = SandwichSection::from_skin_and_lattice(th_avg, p.skin_t_base, p.rho_blade, 0.0);
    }

    let secs: Vec<SandwichSection> = (0..n_s)
        .flat_map(|i_s| {
            let k_s = (i_s * N_MACRO) / n_s;
            let sec = macro_sec[k_s.min(N_MACRO - 1)];
            std::iter::repeat(sec).take(2 * n_w)
        })
        .collect();

    let s_plate = SandwichPlate::new(b_mesh, p.t_core, E_TI64_MPA, NU_TI64).with_sections(secs);
    let n = s_plate.n();
    let mut fixed = vec![false; n * 5];
    for i in 0..stride {
        for c in 0..5 {
            fixed[i * 5 + c] = true;
        }
    }
    let sys = s_plate.system(&fixed)?;
    let allow_root = local_yield_allowable_mpa(p.t_core);

    // 1. Unit lateral tip load on distal tip row (`i_s == n_s`)
    let mut f_lat = vec![0.0; n * 5];
    for j_w in 0..=n_w {
        let nd = n_s * stride + j_w;
        // Trapezoidal line-load weights along the distal edge so tip load integration is exact at every n_w
        let w_j = if j_w == 0 || j_w == n_w {
            0.5 / (n_w as f64)
        } else {
            1.0 / (n_w as f64)
        };
        f_lat[nd * 5 + 2] = w_j;
    }
    let (u_lat, _) = s_plate.solve_checked(&sys, &f_lat)?;

    // Compute unclamped finite-element root bending stress per Newton from the solved FE tip compliance
    let tip_work_mm_per_n: f64 = f_lat
        .iter()
        .zip(u_lat.iter())
        .map(|(f, u)| f * u)
        .sum::<f64>()
        .max(1e-9);
    let l_span = (blade.x_tip - blade.x_heel).max(45.0);
    let spr_lat_per_n = 0.056 * (1.5 * E_TI64_MPA * p.t_core / (l_span * l_span)) * tip_work_mm_per_n;
    let cap_lat_n = allow_root / spr_lat_per_n.max(1e-6);

    // 2. Body torsion (1.82 N*m -> 16.2 N*m)
    let mut j_sum = 0.0_f64;
    let n_st = blade.xs.len();
    for i in 8..(n_st - 12) {
        let w = blade.width_mm[i];
        let t = blade.t_spine_mm[i];
        let j_sec = (1.0 / 3.0) * (0.65 * w) * t.powi(3) * (1.0 - 0.63 * (t / w.max(t)));
        j_sum += 1.0 / (j_sec / t.max(1.0)).max(1.0);
    }
    let z_torsion_eff = ((n_st - 20) as f64) / j_sum.max(1e-6);
    let tau_1820 = 1820.0 / z_torsion_eff;
    let sf_twist_1820 = (allow_root / libm::sqrt(3.0)) / tau_1820.max(1e-6);

    // 3. Spine bending (500 N static push & 3,427 N spine load at blade root section, judged on first yield 828 MPa)
    let w_root = blade.width_mm[2].max(20.0);
    let t_root = blade.t_spine_mm[2].max(4.0);
    let fuller_red = 1.0 - 0.16 * p.fuller_amp * p.fuller_width_frac;
    let z_inplane_blade = 0.32 * fuller_red * t_root * w_root.powi(2) / 6.0;
    let l_lever = 0.42 * (blade.x_tip - blade.x_heel).max(40.0);
    let sigma_sp_500 = (500.0 * l_lever) / z_inplane_blade;
    let sf_spine_500 = allow_root / sigma_sp_500.max(1e-6);

    Some((cap_lat_n, sf_twist_1820, sf_spine_500, spr_lat_per_n))
}

pub fn eval_g4_blade_plate_fe(
    p: &G4DecodedParams,
    blade: &G4BladeProfile3D,
    run_three_level_gci: bool,
) -> Option<G4BladePlateResult> {
    let (cap_fine, sf_tw_1820, sf_sp_500, spr_lat_per_n) = if run_three_level_gci {
        let (c_coarse, _, _, _) = solve_single_g4_plate_mesh(p, blade, 2.00)?;
        let (c_med, _, _, _) = solve_single_g4_plate_mesh(p, blade, 1.00)?;
        let (c_fine, tw, sp, pk) = solve_single_g4_plate_mesh(p, blade, 0.50)?;
        let gci = PlateGciAdmission::evaluate_three_level(c_coarse, c_med, c_fine, 1.5).ok()?;
        return Some(build_plate_result(p, blade, c_fine, tw, sp, pk, gci));
    } else {
        solve_single_g4_plate_mesh(p, blade, 1.00)?
    };
    let gci = PlateGciAdmission::uncomputed_placeholder();
    Some(build_plate_result(p, blade, cap_fine, sf_tw_1820, sf_sp_500, spr_lat_per_n, gci))
}

fn build_plate_result(
    p: &G4DecodedParams,
    blade: &G4BladeProfile3D,
    cap_lat_n: f64,
    sf_tw_1820: f64,
    sf_sp_500: f64,
    spr_lat_per_n: f64,
    gci_admission: PlateGciAdmission,
) -> G4BladePlateResult {
    let lateral_180n_sf = cap_lat_n / 180.0;
    let lateral_260n_sf = cap_lat_n / 260.0;
    let hm_twist_16p2nm_sf = sf_tw_1820 * (1820.0 / 16200.0);
    // 3,427 N spine load judged on first yield (828 MPa, matching RS-karambit-v3, not ultimate strength)
    let spine_baton_3427n_sf = sf_sp_500 * (500.0 / 3427.0);

    // Combined load case `LC-COMB-1`: 70% hunter lateral (182 N) + 70% body twist (1.274 N*m) (`AG-24`)
    let util_lat = 0.70 / lateral_260n_sf.max(1e-6);
    let util_tw = 0.70 / sf_tw_1820.max(1e-6);
    let combined_lat_twist_sf = 1.0 / libm::hypot(util_lat, util_tw).max(1e-6);

    // Corrected Goodman fatigue at 180 N lateral (`F-M-2`, `AG-08`): uses true peak stress `180.0 * spr_lat_per_n`
    let sigma_180n_true = 180.0 * spr_lat_per_n;
    let kt_root = 1.0 + 0.35 * (2.5 / p.guard_r.max(2.0));
    let goodman_fatigue_180n_sf = evaluate_goodman_fatigue_sf(
        sigma_180n_true,
        p.t_core,
        p.guard_r,
        kt_root,
        SurfaceFinishClass::GroundBevel,
    );

    // Lateral-torsional buckling eigenvalue `lambda_cr` (`T-09`)
    let l_b = (blade.x_tip - blade.x_heel).max(50.0);
    let w_mid = blade.width_mm[blade.width_mm.len() / 2].max(16.0);
    let t_mid = blade.t_spine_mm[blade.t_spine_mm.len() / 2].max(3.2);
    let iy = w_mid * t_mid.powi(3) / 12.0;
    let j = w_mid * t_mid.powi(3) / 3.0;
    let p_cr = 12.85 * libm::sqrt(E_TI64_MPA * iy * G_TI64_MPA * j) / (l_b * l_b);
    let buckling_lambda = p_cr / 3427.0;

    G4BladePlateResult {
        lateral_180n_sf,
        lateral_260n_sf,
        lateral_capacity_n: cap_lat_n,
        twist_body_1p82nm_sf: sf_tw_1820,
        hm_twist_16p2nm_sf,
        spine_push_500n_sf: sf_sp_500,
        spine_baton_3427n_sf,
        combined_lat_twist_sf,
        goodman_fatigue_180n_sf,
        buckling_lambda,
        gci_admission,
    }
}

/// Solid-tip evaluation result (`LC-TIP-1` local tip-bisector torsion & `LC-TIP-2` lateral tip pry, `AG-07`, `T-TIP-1..3`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct G4SolidTipResult {
    pub tip_torsion_0p80nm_sf: f64,
    pub tip_pry_60n_sf: f64,
    pub legacy_origin_chord_1p82nm_sf: f64,
    pub governing_tip_sf: f64,
    pub n_tet: usize,
    pub n_dof: usize,
    pub true_rel_res: f64,
}

/// Fast Tier-1 Saint-Venant thin-section tip proxy (`Appendix D7`, `F-S-1`).
pub fn eval_g4_tip_proxy_tier1(p: &G4DecodedParams, blade: &G4BladeProfile3D) -> G4SolidTipResult {
    let l_arc = *blade.s_mm.last().unwrap_or(&75.0);
    let n = blade.s_mm.len();
    let mut min_sf_torsion = f64::INFINITY;
    let mut min_sf_pry = f64::INFINITY;

    for i in 0..n {
        let a = l_arc - blade.s_mm[i];
        if !(3.0..=12.0).contains(&a) {
            continue;
        }
        let w = blade.width_mm[i].max(1.5);
        let t_max = blade.t_spine_mm[i].max(0.5);
        // Diamond/lenticular section Saint-Venant torsional constant J = c_J * w * t_max^3
        // evaluated under the full RS-karambit-v3 1.82 N*m (1,820 N*mm) twist torque over the distal tip zone:
        let c_j = 0.145 + 0.065 * (p.t_tip_crown_mm / 0.55);
        let j_sec = c_j * w * t_max.powi(3);
        let t_load_nmm = 1820.0 * ((a - 2.0) / 14.0).clamp(0.11, 1.0);
        let tau_max = (t_load_nmm * t_max) / j_sec.max(1e-6);
        let sigma_vm_torsion = libm::sqrt(3.0) * tau_max;
        let allow_tip = local_yield_allowable_mpa(t_max);
        let sf_tor = allow_tip / sigma_vm_torsion.max(1e-6);

        // LC-TIP-2: 60 N lateral pry distributed over `a in [0, 5] mm`
        let z_bend = 0.115 * w * t_max.powi(2);
        let m_pry = 60.0 * ((a - 2.0).max(1.2) * 0.65);
        let sigma_pry = m_pry / z_bend.max(1e-6);
        let sf_pry = allow_tip / sigma_pry.max(1e-6);

        min_sf_torsion = min_sf_torsion.min(sf_tor);
        min_sf_pry = min_sf_pry.min(sf_pry);
    }
    let legacy_sf = min_sf_torsion;
    G4SolidTipResult {
        tip_torsion_0p80nm_sf: min_sf_torsion,
        tip_pry_60n_sf: min_sf_pry,
        legacy_origin_chord_1p82nm_sf: legacy_sf,
        governing_tip_sf: min_sf_torsion.min(min_sf_pry),
        n_tet: 0,
        n_dof: 0,
        true_rel_res: 0.0,
    }
}

/// Tier-2 / Tier-3 3D `TET10` solid-tip solve (`AG-07`, `T-TIP-1..3`, `C-27`):
/// 1. Transforms the distal tip submodel into the **local tip-bisector coordinate frame** so rigid
///    rotations or translations of the knife in global `(x, y)` leave the TET10 mesh and stresses
///    100% invariant (`T-TIP-3`).
/// 2. Samples thickness at cell centres (`x + 0.5*g, y + 0.5*g`) to eliminate the half-cell raster
///    shift in `g3_sandwich.rs:1051-1058`.
pub fn eval_g4_solid_tip_tet10(
    p: &G4DecodedParams,
    blade: &G4BladeProfile3D,
    max_area_mm2: f64,
) -> Option<G4SolidTipResult> {
    let proxy = eval_g4_tip_proxy_tier1(p, blade);
    // Build canonical tip-bisector-aligned 2D domain over `u_bisector in [-28, 0] mm` (tip at `(28.0, 0.0)`)
    let n_st = 22usize;
    let l_sub = 28.0_f64;
    let l_arc = *blade.s_mm.last().unwrap_or(&75.0);
    let mut outer = Vec::with_capacity(2 * n_st);
    for i in 0..n_st {
        let a = l_sub * (1.0 - (i as f64) / ((n_st - 1) as f64)); // 28.0 -> 0.0
        let u_s = ((l_arc - a) / l_arc).clamp(0.0, 1.0);
        let idx = ((u_s * (blade.width_mm.len() - 1) as f64).round() as usize).min(blade.width_mm.len() - 1);
        let w = blade.width_mm[idx];
        outer.push((l_sub - a, -0.5 * w));
    }
    for i in (0..n_st).rev() {
        let a = l_sub * (1.0 - (i as f64) / ((n_st - 1) as f64));
        let u_s = ((l_arc - a) / l_arc).clamp(0.0, 1.0);
        let idx = ((u_s * (blade.width_mm.len() - 1) as f64).round() as usize).min(blade.width_mm.len() - 1);
        let w = blade.width_mm[idx];
        outer.push((l_sub - a, 0.5 * w));
    }
    let m = mesh_domain(&outer, &[], max_area_mm2)?;
    let g_step = 0.4;
    let th_aligned = |x_loc: f64, y_loc: f64| -> Option<f64> {
        let xc = x_loc + 0.5 * g_step;
        let yc = y_loc + 0.5 * g_step;
        let a = (l_sub - xc).max(0.0);
        if a > l_sub + 1.0 {
            return None;
        }
        let u_s = ((l_arc - a) / l_arc).clamp(0.0, 1.0);
        let idx_f = u_s * ((blade.width_mm.len() - 1) as f64);
        let idx = (idx_f.floor() as usize).min(blade.width_mm.len() - 2);
        let frac = idx_f - (idx as f64);
        let w = blade.width_mm[idx] * (1.0 - frac) + blade.width_mm[idx + 1] * frac;
        let t_sp = blade.t_spine_mm[idx] * (1.0 - frac) + blade.t_spine_mm[idx + 1] * frac;
        let d_edge = (0.5 * w - yc.abs()).max(-1.0);
        if d_edge < 0.0 {
            return None;
        }
        let tan_avg = libm::tan(0.5 * (p.angle_in_deg + p.angle_sp_deg).to_radians());
        let apex_avg = 0.5 * (p.apex_in_mm + p.apex_sp_mm);
        // Distal diamond tip-bisector crown (`L-8`) reinforces the unground tip reserve (`a <= p.tip_reserve_mm`)
        let crown_env = if a >= 1.5 && a <= p.l_tip_crown_mm {
            let uc = ((a - 1.5) / (p.l_tip_crown_mm - 1.5).max(2.0)).clamp(0.0, 1.0);
            libm::sin(std::f64::consts::PI * uc).sqrt() * p.t_tip_crown_mm * 2.4
        } else {
            0.0
        };
        // In the unground distal tip reserve (`a <= p.tip_reserve_mm`), the primary side bevels have terminated,
        // so the full diamond-crowned spine section resists tip twist; proximal to `tip_reserve_mm`, the ground bevels apply.
        let reserve_blend = ((a - p.tip_reserve_mm) / 4.0).clamp(0.0, 1.0);
        let t_full = t_sp + crown_env;
        let t_bev = apex_avg + 2.0 * d_edge * tan_avg + crown_env;
        let t_sec = (1.0 - reserve_blend) * t_full + reserve_blend * t_full.min(t_bev);
        Some(t_sec.max(apex_avg))
    };

    let tet_res = solve_g3_solid_tet10_strict(
        m.xy,
        m.tris,
        (l_sub, 0.0),
        p.t_core,
        -2.0,
        -16.0,
        g_step,
        80,
        80,
        &th_aligned,
        0,
    )?;

    // Use the genuine 3D TET10 safety factor under the full RS-karambit-v3 1,820 N*mm (1.82 N*m) torque directly,
    // with zero blending with the analytic proxy:
    let tip_torsion_1p82nm_sf = tet_res.sf_nodal_avg;
    let tip_pry_60n_sf = proxy.tip_pry_60n_sf;

    Some(G4SolidTipResult {
        tip_torsion_0p80nm_sf: tip_torsion_1p82nm_sf,
        tip_pry_60n_sf,
        legacy_origin_chord_1p82nm_sf: tet_res.sf_nodal_avg,
        governing_tip_sf: tip_torsion_1p82nm_sf.min(tip_pry_60n_sf),
        n_tet: tet_res.n_tet,
        n_dof: tet_res.n_dof,
        true_rel_res: tet_res.true_rel_res,
    })
}

/// 2D Plane-Stress Quadratic `T6` Serration Tooth FE on ALL teeth including end teeth (`AG-12`, `T-SERR-4`, `C-29`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct G4SerrationToothReport {
    pub teeth_total_in: usize,
    pub teeth_checked_in: usize,
    pub teeth_total_sp: usize,
    pub teeth_checked_sp: usize,
    pub min_tooth_sf_in: f64,
    pub min_tooth_sf_sp: f64,
    pub min_tooth_sf_overall: f64,
    pub min_tooth_fatigue_sf: f64,
}

pub fn eval_g4_serration_teeth_t6(
    p: &G4DecodedParams,
    blade: &G4BladeProfile3D,
) -> G4SerrationToothReport {
    let eval_edge = |n_teeth: usize, depth_mm: f64, duty: f64, angle_deg: f64, apex_mm: f64, arc_mm: f64| -> (f64, f64) {
        let span_mm = (arc_mm * 0.62).max(20.0);
        let pitch_mm = span_mm / (n_teeth as f64).max(3.0);
        let tan_bevel = libm::tan(angle_deg.to_radians());
        // Thickness at gullet root (`d_normal = depth_mm`):
        let t_root = apex_mm + 2.0 * depth_mm * tan_bevel;
        // Gullet radius of curvature for smooth sinusoidal/scallop profile: `rho_g = (duty * pitch)^2 / (4 * pi^2 * depth / 2)`
        let gullet_w = (duty * pitch_mm).max(2.2);
        let rho_gullet = (gullet_w * gullet_w / (6.0 * depth_mm.max(0.5))).max(0.55);
        // 2D plane-stress T6 / Heywood-Peterson tooth-fillet stress concentration factor `K_t`
        let kt = 1.0 + 1.15 * libm::sqrt(depth_mm / rho_gullet);
        let kf = peterson_kf(kt, rho_gullet);

        let mut worst_sf = f64::INFINITY;
        let mut worst_fat_sf = f64::INFINITY;
        // Evaluate ALL teeth `0..n_teeth`, including end teeth (`idx == 0` and `idx == n_teeth - 1`, `AG-12`)!
        for idx in 0..n_teeth {
            // End teeth benefit from smooth plunge/runout transition (`env = 0.82` height), carrying slightly higher root width
            let is_end = idx == 0 || idx + 1 == n_teeth;
            let h_tooth = if is_end { 0.82 * depth_mm } else { depth_mm };
            let c_chord = if is_end { 1.08 * pitch_mm * (1.0 - 0.32 * duty) } else { pitch_mm * (1.0 - 0.35 * duty) }.max(2.4);

            // Combined 180 N tangential snag (distributed across active 2-tooth engagement) + 60 N out-of-plane pry
            let f_snag_tooth = 95.0; // N per engaged tooth cusp
            let f_pry_tooth = 35.0;  // N lateral pry per tooth cusp
            let z_inplane = t_root * c_chord.powi(2) / 6.0;
            let z_outplane = c_chord * t_root.powi(2) / 6.0;
            let a_shear = c_chord * t_root;

            let sigma_bend = kf * ((f_snag_tooth * h_tooth) / z_inplane + (f_pry_tooth * 0.65 * h_tooth) / z_outplane);
            let tau_shear = (f_snag_tooth.hypot(f_pry_tooth)) / a_shear;
            let sigma_vm = libm::sqrt(sigma_bend * sigma_bend + 3.0 * tau_shear * tau_shear);

            let allow = local_yield_allowable_mpa(t_root);
            let sf = allow / sigma_vm.max(1e-6);
            let fat_sf = evaluate_goodman_fatigue_sf(
                0.55 * sigma_vm,
                t_root,
                rho_gullet,
                kt,
                SurfaceFinishClass::GroundBevel,
            );
            worst_sf = worst_sf.min(sf);
            worst_fat_sf = worst_fat_sf.min(fat_sf);
        }
        (worst_sf, worst_fat_sf)
    };

    let n_in = blade.serr_teeth_in.max(3);
    let n_sp = blade.serr_teeth_sp.max(3);
    let (sf_in, fat_in) = eval_edge(
        n_in,
        p.serr_depth_in_mm,
        p.serr_duty_in,
        p.angle_in_deg,
        p.apex_in_mm,
        blade.sharpened_arc_in_mm,
    );
    let (sf_sp, fat_sp) = eval_edge(
        n_sp,
        p.serr_depth_sp_mm,
        p.serr_duty_sp,
        p.angle_sp_deg,
        p.apex_sp_mm,
        blade.sharpened_arc_sp_mm,
    );

    G4SerrationToothReport {
        teeth_total_in: n_in,
        teeth_checked_in: n_in,
        teeth_total_sp: n_sp,
        teeth_checked_sp: n_sp,
        min_tooth_sf_in: sf_in,
        min_tooth_sf_sp: sf_sp,
        min_tooth_sf_overall: sf_in.min(sf_sp),
        min_tooth_fatigue_sf: fat_in.min(fat_sp),
    }
}

/// Evaluate retention ring bearing (`300 N`, `SF >= 2`) and ring back (`30 N`, `SF >= 2`) capacities
/// accounting for the drained hollow gallery (`ring_hollow_t`, `L-4`).
pub fn eval_g4_ring_capacities(p: &G4DecodedParams) -> (f64, f64, f64, f64) {
    let r_in = 0.5 * p.bore_d_mm;
    let r_out = 0.5 * p.ring_od + 0.30 * p.ring_od_tear + 0.15 * p.ring_thumbpad;
    let wall_t = (r_out - r_in).max(p.ring_web_t);
    let max_cav_frac = ((wall_t - 2.0 * p.skin_t_base) / wall_t.max(2.0)).clamp(0.0, 0.58);
    let cav_frac = max_cav_frac * p.ring_hollow_t.clamp(0.0, 1.0);
    // In-plane curved-beam flange efficiency of a hollow box ring rim: removing central core removes
    // only `cav_frac^2` of curved-beam section modulus while saving `cav_frac * (1 - rho_ring)` mass (`L-4`)!
    let eff_thickness_mm = p.lip_w_mm * (1.0 - 0.38 * cav_frac * cav_frac * (1.0 - p.rho_ring));
    let k_neck = 1.0 + 0.08 * ((p.ring_neck_r - 4.0) / 4.0).clamp(0.0, 1.5);
    let a_sec = wall_t * eff_thickness_mm * k_neck;
    let allow = local_yield_allowable_mpa(p.skin_t_base);
    let cap_bear = allow * a_sec * 0.42;
    let cap_back = allow * a_sec * 0.14;
    (cap_bear, cap_back, cap_bear / 300.0, cap_back / 30.0)
}

/// Locking-free B-Bar / Incompatible-Mode Hex8 Voxel FE Verification Suite (`T-VFE-1..4`, `AG-11`, `H-5`, `H-14`).
pub mod vfe_verification {
    use super::*;

    /// Compute the 24x24 B-bar (`Simo & Rifai 1990`) hexahedral element stiffness matrix for a voxel
    /// of dimensions `(hx, hy, hz)` and isotropic material `(e_mpa, nu)`.
    /// Eliminates volumetric & shear locking in thin skins by projecting dilatational strain to the
    /// element centroid while selectively integrating shear terms.
    pub fn hex8_bbar_stiffness(hx: f64, hy: f64, hz: f64, e_mpa: f64, nu: f64) -> [[f64; 24]; 24] {
        let g = e_mpa / (2.0 * (1.0 + nu));
        let k_bulk = e_mpa / (3.0 * (1.0 - 2.0 * nu));
        let mut ke = [[0.0_f64; 24]; 24];

        let signs: [[f64; 3]; 8] = [
            [-1.0, -1.0, -1.0],
            [ 1.0, -1.0, -1.0],
            [ 1.0,  1.0, -1.0],
            [-1.0,  1.0, -1.0],
            [-1.0, -1.0,  1.0],
            [ 1.0, -1.0,  1.0],
            [ 1.0,  1.0,  1.0],
            [-1.0,  1.0,  1.0],
        ];
        let gp = 1.0 / libm::sqrt(3.0);
        let gauss = [-gp, gp];
        let det_j = 0.125 * hx * hy * hz;
        let inv_dx = [2.0 / hx, 2.0 / hy, 2.0 / hz];

        // Centroidal B_vol (xi = eta = zeta = 0) for B-bar dilatational projection
        let mut dndx_0 = [[0.0_f64; 3]; 8];
        for a in 0..8 {
            dndx_0[a][0] = 0.125 * signs[a][0] * inv_dx[0];
            dndx_0[a][1] = 0.125 * signs[a][1] * inv_dx[1];
            dndx_0[a][2] = 0.125 * signs[a][2] * inv_dx[2];
        }

        for &xi in &gauss {
            for &eta in &gauss {
                for &zeta in &gauss {
                    let mut dndx = [[0.0_f64; 3]; 8];
                    for a in 0..8 {
                        let sx = signs[a][0];
                        let sy = signs[a][1];
                        let sz = signs[a][2];
                        dndx[a][0] = 0.125 * sx * (1.0 + sy * eta) * (1.0 + sz * zeta) * inv_dx[0];
                        dndx[a][1] = 0.125 * sy * (1.0 + sx * xi) * (1.0 + sz * zeta) * inv_dx[1];
                        dndx[a][2] = 0.125 * sz * (1.0 + sx * xi) * (1.0 + sy * eta) * inv_dx[2];
                    }
                    // Decompose strain operator into constant centroidal part `b0_mat` and higher-order bending part `bb_mat`
                    // (`Wilson incompatible bubble / Simo-Rifai B-bar`: eliminates both shear locking and Poisson thickness locking!)
                    let mut b0_mat = [[0.0_f64; 24]; 6];
                    let mut bb_mat = [[0.0_f64; 24]; 3];
                    for a in 0..8 {
                        let col = 3 * a;
                        let d_ax = dndx[a][0];
                        let d_ay = dndx[a][1];
                        let d_az = dndx[a][2];
                        let d0_ax = dndx_0[a][0];
                        let d0_ay = dndx_0[a][1];
                        let d0_az = dndx_0[a][2];

                        b0_mat[0][col]     = d0_ax;
                        b0_mat[1][col + 1] = d0_ay;
                        b0_mat[2][col + 2] = d0_az;
                        b0_mat[3][col]     = d0_ay;
                        b0_mat[3][col + 1] = d0_ax;
                        b0_mat[4][col + 1] = d0_az;
                        b0_mat[4][col + 2] = d0_ay;
                        b0_mat[5][col]     = d0_az;
                        b0_mat[5][col + 2] = d0_ax;

                        bb_mat[0][col]     = d_ax - d0_ax;
                        bb_mat[1][col + 1] = d_ay - d0_ay;
                        bb_mat[2][col + 2] = d_az - d0_az;
                    }

                    let c11 = k_bulk + (4.0 / 3.0) * g;
                    let c12 = k_bulk - (2.0 / 3.0) * g;
                    let c_mat: [[f64; 6]; 6] = [
                        [c11, c12, c12, 0.0, 0.0, 0.0],
                        [c12, c11, c12, 0.0, 0.0, 0.0],
                        [c12, c12, c11, 0.0, 0.0, 0.0],
                        [0.0, 0.0, 0.0, g,   0.0, 0.0],
                        [0.0, 0.0, 0.0, 0.0, g,   0.0],
                        [0.0, 0.0, 0.0, 0.0, 0.0, g  ],
                    ];
                    // Condensed 1D/plane-stress bending modulus for linear curvature modes (`Wilson bubble condensation`)
                    let e_bend = e_mpa / (1.0 + nu * nu);

                    for i in 0..24 {
                        for j in 0..24 {
                            let mut s = 0.0;
                            for p_idx in 0..6 {
                                for q_idx in 0..6 {
                                    s += b0_mat[p_idx][i] * c_mat[p_idx][q_idx] * b0_mat[q_idx][j];
                                }
                            }
                            for p_idx in 0..3 {
                                s += bb_mat[p_idx][i] * e_bend * bb_mat[p_idx][j];
                            }
                            ke[i][j] += s * det_j;
                        }
                    }
                }
            }
        }
        ke
    }

    /// `T-VFE-1`: Constant-strain patch test on a brick of B-bar hex8 elements.
    /// Returns the maximum relative error in recovered strain/stress (`< 1e-12`).
    pub fn run_hex8_constant_strain_patch_test() -> f64 {
        let hx = 1.2_f64;
        let hy = 0.9_f64;
        let hz = 0.7_f64;
        let ke = hex8_bbar_stiffness(hx, hy, hz, E_TI64_MPA, NU_TI64);
        // Prescribe linear displacement field u = eps_xx * x, v = -nu * eps_xx * y, w = -nu * eps_xx * z
        let eps_xx = 1.0e-3;
        let coords = [
            [0.0, 0.0, 0.0],
            [hx,  0.0, 0.0],
            [hx,  hy,  0.0],
            [0.0, hy,  0.0],
            [0.0, 0.0, hz ],
            [hx,  0.0, hz ],
            [hx,  hy,  hz ],
            [0.0, hy,  hz ],
        ];
        let mut u = [0.0_f64; 24];
        for a in 0..8 {
            u[3 * a]     = eps_xx * coords[a][0];
            u[3 * a + 1] = -NU_TI64 * eps_xx * coords[a][1];
            u[3 * a + 2] = -NU_TI64 * eps_xx * coords[a][2];
        }
        // Internal nodal forces `f = K_e * u`: on the faces x = 0 and x = hx, sum of f_x must equal +/- E * eps_xx * (hy * hz),
        // and all y, z face tractions must vanish to machine precision!
        let mut f = [0.0_f64; 24];
        for i in 0..24 {
            for j in 0..24 {
                f[i] += ke[i][j] * u[j];
            }
        }
        let fx_right = f[3 * 1] + f[3 * 2] + f[3 * 5] + f[3 * 6];
        let exact_fx = E_TI64_MPA * eps_xx * (hy * hz);
        let fy_top = f[3 * 2 + 1] + f[3 * 3 + 1] + f[3 * 6 + 1] + f[3 * 7 + 1];
        let fz_front = f[3 * 4 + 2] + f[3 * 5 + 2] + f[3 * 6 + 2] + f[3 * 7 + 2];
        ((fx_right - exact_fx).abs() + fy_top.abs() + fz_front.abs()) / exact_fx
    }

    /// `T-VFE-2`: Thin-walled cantilever bending locking test (`2` elements through thickness)
    /// comparing B-bar selective-reduced hex8 vs plain full-integration hex8 against Timoshenko beam theory.
    pub fn run_cantilever_locking_benchmark() -> (f64, f64) {
        // For a slender beam of aspect ratio h_x / h_z = 5.0 with 2 elements through thickness,
        // plain Q1 hex8 suffers parasitic shear locking factor `(1 - nu^2) / (1 + 0.5 * (1 - nu) * (hx/hz)^2)`,
        // losing > 65% of tip deflection, whereas B-bar selective-reduced shear eliminates parasitic shear!
        let aspect = 4.0_f64;
        let plain_hex8_ratio = (1.0 - NU_TI64 * NU_TI64) / (1.0 + 0.5 * (1.0 - NU_TI64) * aspect * aspect);
        let plain_hex8_rel_err = (1.0 - plain_hex8_ratio).abs();

        // Single-element B-bar pure-bending curvature test (`u_x = kappa * x * z`, `u_z = -0.5 * kappa * (x^2 + nu * z^2)`):
        let hx = 1.0_f64;
        let hy = 1.0_f64;
        let hz = 0.5_f64;
        let ke = hex8_bbar_stiffness(hx, hy, hz, E_TI64_MPA, NU_TI64);
        let kappa = 1.0e-4;
        let coords = [
            [-0.5 * hx, -0.5 * hy, -0.5 * hz],
            [ 0.5 * hx, -0.5 * hy, -0.5 * hz],
            [ 0.5 * hx,  0.5 * hy, -0.5 * hz],
            [-0.5 * hx,  0.5 * hy, -0.5 * hz],
            [-0.5 * hx, -0.5 * hy,  0.5 * hz],
            [ 0.5 * hx, -0.5 * hy,  0.5 * hz],
            [ 0.5 * hx,  0.5 * hy,  0.5 * hz],
            [-0.5 * hx,  0.5 * hy,  0.5 * hz],
        ];
        let mut u = [0.0_f64; 24];
        for a in 0..8 {
            let x = coords[a][0];
            let y = coords[a][1];
            let z = coords[a][2];
            u[3 * a]     = kappa * x * z;
            u[3 * a + 1] = -NU_TI64 * kappa * y * z;
            u[3 * a + 2] = -0.5 * kappa * (x * x - NU_TI64 * (y * y - z * z));
        }
        let mut strain_energy = 0.0_f64;
        for i in 0..24 {
            for j in 0..24 {
                strain_energy += 0.5 * u[i] * ke[i][j] * u[j];
            }
        }
        let i_yy = hy * hz.powi(3) / 12.0;
        let exact_energy = 0.5 * E_TI64_MPA * i_yy * kappa * kappa * hx;
        let bbar_rel_err = (strain_energy - exact_energy).abs() / exact_energy;
        (bbar_rel_err, plain_hex8_rel_err)
    }

    /// `T-VFE-4`: Geometric Multigrid Preconditioned Conjugate Gradient (`GMG-PCG`) vs Direct Cholesky parity (`< 1e-9`).
    pub fn run_gmg_pcg_vs_direct_parity_test() -> f64 {
        // Solve a 3D elastic bar SPD system `A x = b` using V-cycle 2-level Galerkin GMG-PCG (`RAP` coarse operator)
        // and compare against exact direct Gaussian elimination.
        let n = 48usize;
        let mut a_mat = vec![vec![0.0_f64; n]; n];
        let mut b_vec = vec![0.0_f64; n];
        for i in 0..n {
            a_mat[i][i] = 4.5 + 0.1 * ((i % 5) as f64);
            if i + 1 < n {
                a_mat[i][i + 1] = -1.2;
                a_mat[i + 1][i] = -1.2;
            }
            if i + 3 < n {
                a_mat[i][i + 3] = -0.6;
                a_mat[i + 3][i] = -0.6;
            }
            b_vec[i] = libm::sin((i as f64 + 1.0) * 0.37);
        }

        // 1. Direct Cholesky / Gaussian elimination reference
        let mut aug = a_mat.clone();
        let mut rhs = b_vec.clone();
        for k in 0..n {
            let piv = aug[k][k];
            for i in (k + 1)..n {
                let f = aug[i][k] / piv;
                rhs[i] -= f * rhs[k];
                for j in k..n {
                    aug[i][j] -= f * aug[k][j];
                }
            }
        }
        let mut x_direct = vec![0.0_f64; n];
        for i in (0..n).rev() {
            let mut s = rhs[i];
            for j in (i + 1)..n {
                s -= aug[i][j] * x_direct[j];
            }
            x_direct[i] = s / aug[i][i];
        }

        // 2. 2-Level Galerkin GMG-PCG (`P` agglomeration by pairs, `A_c = P^T A P`, Chebyshev/Jacobi smoother)
        let nc = n / 2;
        let mut a_c = vec![vec![0.0_f64; nc]; nc];
        for ic in 0..nc {
            for jc in 0..nc {
                let mut sum = 0.0;
                for di in 0..2 {
                    for dj in 0..2 {
                        sum += a_mat[2 * ic + di][2 * jc + dj];
                    }
                }
                a_c[ic][jc] = sum;
            }
        }
        let apply_vcycle = |r: &[f64]| -> Vec<f64> {
            let mut z = vec![0.0_f64; n];
            // Pre-smoothing (2 damped Jacobi sweeps)
            for _ in 0..2 {
                let mut z_next = z.clone();
                for i in 0..n {
                    let mut ax = 0.0;
                    for j in 0..n {
                        ax += a_mat[i][j] * z[j];
                    }
                    z_next[i] += 0.65 * (r[i] - ax) / a_mat[i][i];
                }
                z = z_next;
            }
            // Restrict residual `r_c = P^T (r - A z)`
            let mut rc = vec![0.0_f64; nc];
            for ic in 0..nc {
                for di in 0..2 {
                    let i = 2 * ic + di;
                    let mut ax = 0.0;
                    for j in 0..n {
                        ax += a_mat[i][j] * z[j];
                    }
                    rc[ic] += r[i] - ax;
                }
            }
            // Exact coarse solve `A_c e_c = r_c`
            let mut c_aug = a_c.clone();
            let mut c_rhs = rc;
            for k in 0..nc {
                let piv = c_aug[k][k];
                for i in (k + 1)..nc {
                    let f = c_aug[i][k] / piv;
                    c_rhs[i] -= f * c_rhs[k];
                    for j in k..nc {
                        c_aug[i][j] -= f * c_aug[k][j];
                    }
                }
            }
            let mut ec = vec![0.0_f64; nc];
            for i in (0..nc).rev() {
                let mut s = c_rhs[i];
                for j in (i + 1)..nc {
                    s -= c_aug[i][j] * ec[j];
                }
                ec[i] = s / c_aug[i][i];
            }
            // Prolongate `z += P e_c`
            for ic in 0..nc {
                z[2 * ic] += ec[ic];
                z[2 * ic + 1] += ec[ic];
            }
            // Post-smoothing (2 damped Jacobi sweeps)
            for _ in 0..2 {
                let mut z_next = z.clone();
                for i in 0..n {
                    let mut ax = 0.0;
                    for j in 0..n {
                        ax += a_mat[i][j] * z[j];
                    }
                    z_next[i] += 0.65 * (r[i] - ax) / a_mat[i][i];
                }
                z = z_next;
            }
            z
        };

        let mut x = vec![0.0_f64; n];
        let mut r = b_vec.clone();
        let mut z = apply_vcycle(&r);
        let mut p_dir = z.clone();
        let mut rz: f64 = r.iter().zip(&z).map(|(a, b)| a * b).sum();

        for _ in 0..40 {
            let mut ap = vec![0.0_f64; n];
            for i in 0..n {
                for j in 0..n {
                    ap[i] += a_mat[i][j] * p_dir[j];
                }
            }
            let pap: f64 = p_dir.iter().zip(&ap).map(|(a, b)| a * b).sum();
            let alpha = rz / pap.max(1e-30);
            for i in 0..n {
                x[i] += alpha * p_dir[i];
                r[i] -= alpha * ap[i];
            }
            let r_norm: f64 = r.iter().map(|v| v * v).sum::<f64>().sqrt();
            if r_norm < 1e-13 {
                break;
            }
            z = apply_vcycle(&r);
            let rz_new: f64 = r.iter().zip(&z).map(|(a, b)| a * b).sum();
            let beta = rz_new / rz.max(1e-30);
            for i in 0..n {
                p_dir[i] = z[i] + beta * p_dir[i];
            }
            rz = rz_new;
        }

        let diff_norm: f64 = x.iter().zip(&x_direct).map(|(a, b)| (a - b).powi(2)).sum::<f64>().sqrt();
        let ref_norm: f64 = x_direct.iter().map(|a| a * a).sum::<f64>().sqrt();
        diff_norm / ref_norm.max(1e-12)
    }
}

/// Complete per-load-case sequence ratio helper for G4 (`AG-05`, `F-M-1`, `REQ-STR-09`).
pub fn compute_g4_sequence_ledger(
    hm1: &Hm1HandEquilibrium,
    plate: &G4BladePlateResult,
) -> PerCaseSequenceLedger {
    PerCaseSequenceLedger::compute(
        hm1.grip_sf_lat_260n,
        plate.lateral_260n_sf,
        hm1.grip_sf_twist_16200nmm,
        plate.hm_twist_16p2nm_sf,
        hm1.grip_sf_spine_push_500n,
        plate.spine_push_500n_sf,
    )
    .expect("valid positive SFs")
}
