//! G5 3-Tier Multi-Fidelity Continuous Evaluation Cascade and 1024-Niche CVT-MAP-Elites Engine
//! (`T4.3`, `G3_GENOME_SPEC.md` v1.1 §4–§7).
//!
//! Strictly constraint- and fitness-driven (Owner directive 2026-10-25T01:13:42Z):
//! - **Zero manual design of individuals**: populations initialize exclusively from the quasi-random
//!   Sobol sequence (`sobol_unit_genome`) over $[0, 1]^{179}$.
//! - **Zero hard gates / vetoes**: every candidate receives a continuous `W_ERGOLIGHT_v1` (draft-d)
//!   score in `[0, 100]`.
//! - **3-Tier Multi-Fidelity Cascade**:
//!   - **Tier 0 (~2 ms)**: Analytic 1D/2D section integration (ring, hollow grip 250 N squeeze +
//!     bending + torsion, sandwich blade section + Gate S1/S2/S3, finger groove curvature, spinning
//!     balance) to rapidly weed out weak individuals.
//!   - **Tier 1 (~0.35 s)**: 3D SDF (`build_g5_sdf`) + `SandwichPlate` (`CST+DKT`) + `solve_ring_band_t6`
//!     (1.5 mm thickness-weighted distributed load band).
//!   - **Tier 2 (~1.5 s)**: Watertight Marching-Tetrahedra 3D mesh (`extract_watertight_mesh`) +
//!     topology census + fail-closed 3D `TET10` solid tip solver (`solve_g5_solid_tet10_strict`).

use crate::ergolight::{score_ergolight, ErgolightInputs, ErgolightScore};
use crate::g5_genome::{
    reproduce_iso_line_dd, sobol_unit_genome, G5Genome, G5Rng, FIXED_BORE_D_MM, FIXED_LIP_W_MM,
};
use crate::g5_sandwich::{
    solve_g5_solid_tet10_strict, solve_ring_band_t6, SandwichPlate, SandwichSection,
};
use crate::g5_sdf::{
    blade_bevel_window_weights, build_blade_profile_2d, build_g5_sdf, effective_blade_rho,
    eval_blade_thickness_and_sdf2, eval_grip_slice, extract_watertight_mesh, serration_gullets,
    solid_fraction, AmConnectivityStats, BladeProfile2D, VoxelGrid3D, GUARD_X_MM, RHO_TI64_G_MM3,
};
use jarambit_crescent::export;
use crate::material::{self, FatigueClass, Wall, AS_BUILT_WALL, GROUND_WALL, HOLLOW_BLADE_SKIN};
use jarambit_physics::fine::mesh_domain;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const M50_STATIONS_MM: [f64; 3] = [9.75, 30.0, 51.5];
/// Published v1 winner `ec3i042_r01_p10_c56` f075 spin inertia about its ring center `(cx, 0)`
/// (`tests/fixtures/f075_reference.json`, #6 / B6).
pub const F075_SPIN_I_REF_G_MM2: f64 = 572_965.368_840_274_9;

/// The frozen G5 decoder SHA-256: the genome schema, every decoder and evaluator source of this crate, the requirement
/// set RS-karambit-v5, the physics and crescent sources the evaluator calls, and this crate's `Cargo.lock`. Every
/// `G5EvalRecord` and `CvtMapElitesArchive` records it and refuses to mix evaluations if the decoder changes mid-run.
/// Computed once per process (audit F-H-1: G3 recomputed it for every evaluation).
pub fn g5_decoder_sha256() -> String {
    static HASH: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    HASH.get_or_init(g5_decoder_sha256_uncached).clone()
}

/// The decoder hash computed afresh (tests check it equals the cached value).
pub fn g5_decoder_sha256_uncached() -> String {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"G5.0\0");
    for src in [
        include_str!("g5_genome.rs"),
        include_str!("sobol_joe_kuo.rs"),
        include_str!("g5_sdf.rs"),
        include_str!("g5_sandwich.rs"),
        include_str!("ergolight.rs"),
        include_str!("g5_qd.rs"),
        include_str!("requirements.rs"),
        include_str!("material.rs"),
        include_str!("loads.rs"),
        include_str!("../../../requirements/RS-karambit-v5.toml"),
        include_str!("../../physics/src/fine.rs"),
        include_str!("../../physics/src/solid_tet10.rs"),
        include_str!("../../crescent/src/lib.rs"),
        include_str!("../../crescent/src/fem.rs"),
        include_str!("../../crescent/src/certification.rs"),
        include_str!("../../crescent/src/export.rs"),
        include_str!("../Cargo.lock"),
    ] {
        bytes.extend_from_slice(src.as_bytes());
        bytes.push(0);
    }
    crate::sha256_bytes(&bytes)
}

/// 8-dimensional behavior descriptor vector (`G3_GENOME_SPEC.md` v1.1 §4).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct G5Descriptors {
    pub hook_drop_mm: f64,
    pub inner_turn_deg: f64,
    pub silhouette_complexity: f64,
    pub grip_roundness_h_over_t: f64,
    pub groove_articulation_mm: f64,
    pub porosity_frac: f64,
    pub surface_energy: f64,
    pub mass_g: f64,
}

impl G5Descriptors {
    /// Normalize the 8 measured descriptors into $[0, 1]^8$ for CVT-MAP-Elites niche assignment.
    pub fn to_unit8(&self) -> [f64; 8] {
        let norm = |v: f64, lo: f64, hi: f64| ((v - lo) / (hi - lo).max(1e-9)).clamp(0.0, 1.0);
        [
            norm(self.hook_drop_mm, 25.0, 55.0),
            norm(self.inner_turn_deg, 30.0, 95.0),
            norm(self.silhouette_complexity, 4.5, 9.5),
            norm(self.grip_roundness_h_over_t, 1.3, 2.2),
            norm(self.groove_articulation_mm, 2.5, 11.0),
            norm(self.porosity_frac, 0.0, 0.70),
            norm(self.surface_energy, 0.0, 0.25),
            norm(self.mass_g, 25.0, 105.0),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EvalTier {
    Tier0Analytic,
    Tier1SdfAndPlate,
    Tier2Full3D,
}

impl EvalTier {
    /// Explicit per-input measurement vs surrogate provenance for `archive.jsonl` (`S8-2`),
    /// so downstream consumers never quote Tier-0 analytic surrogate credit as a 3D/FE measurement.
    pub fn input_provenance(self) -> serde_json::Value {
        match self {
            EvalTier::Tier0Analytic => serde_json::json!({
                "tier": "Tier0Analytic",
                "summary": "analytic_surrogate_only",
                "measured": [
                    "groove_r_min_mm", "crest_r_min_mm", "saddle_depth_mm",
                    "mid_depth_mm", "choil_depth_mm", "station_err_mm"
                ],
                "design_law": ["edge_r_min_mm", "section_corner_r_min_mm"],
                "surrogate": [
                    "buckling_lambda", "mass_g", "lat_rob_cert_n", "strict_lat_n",
                    "ring_bearing_cap_n", "ring_back_cap_n",
                    "twist_body_sf", "twist_tip_sf", "twist_tipzone_strict_sf",
                    "hm_twist_sf", "hm_spine_sf", "fat_sf", "grip_sf_min",
                    "swell_perimeter_mm", "swell_h_over_t", "L_g_mm", "guard_h_mm",
                    "com_to_index_mm", "spin_I_g_mm2", "am_build", "fairness",
                    "max_apex_mm", "max_half_angle_deg", "t_0p25_mm", "t_1p00_mm", "t_2p00_mm"
                ],
                "unmeasured_zero_credit": ["solid_tip_sf"]
            }),
            EvalTier::Tier1SdfAndPlate => serde_json::json!({
                "tier": "Tier1SdfAndPlate",
                "summary": "voxel_3d_sdf_and_plate_fe",
                "measured": [
                    "mass_g", "lat_rob_cert_n", "strict_lat_n",
                    "ring_bearing_cap_n", "ring_back_cap_n",
                    "twist_body_sf", "twist_tip_sf", "twist_tipzone_strict_sf",
                    "hm_twist_sf", "hm_spine_sf", "fat_sf", "grip_sf_min",
                    "watertight", "am_build", "swell_perimeter_mm", "swell_h_over_t",
                    "L_g_mm", "guard_h_mm", "com_to_index_mm", "spin_I_g_mm2",
                    "groove_r_min_mm", "crest_r_min_mm", "saddle_depth_mm",
                    "mid_depth_mm", "choil_depth_mm", "station_err_mm"
                ],
                "design_law": ["edge_r_min_mm", "section_corner_r_min_mm"],
                "surrogate": [
                    "buckling_lambda", "max_apex_mm", "max_half_angle_deg", "t_0p25_mm", "t_1p00_mm", "t_2p00_mm", "fairness"
                ],
                "unmeasured_zero_credit": ["solid_tip_sf"]
            }),
            EvalTier::Tier2Full3D => serde_json::json!({
                "tier": "Tier2Full3D",
                "summary": "voxel_3d_sdf_plate_fe_watertight_stl_and_solid_tet10",
                "measured": [
                    "mass_g", "lat_rob_cert_n", "strict_lat_n",
                    "ring_bearing_cap_n", "ring_back_cap_n",
                    "twist_body_sf", "twist_tip_sf", "twist_tipzone_strict_sf",
                    "solid_tip_sf", "hm_twist_sf", "hm_spine_sf", "fat_sf", "grip_sf_min",
                    "watertight", "am_build", "swell_perimeter_mm", "swell_h_over_t",
                    "L_g_mm", "guard_h_mm", "com_to_index_mm", "spin_I_g_mm2",
                    "groove_r_min_mm", "crest_r_min_mm", "saddle_depth_mm",
                    "mid_depth_mm", "choil_depth_mm", "station_err_mm"
                ],
                "design_law": ["edge_r_min_mm", "section_corner_r_min_mm"],
                "surrogate": [
                    "buckling_lambda", "max_apex_mm", "max_half_angle_deg", "t_0p25_mm", "t_1p00_mm", "t_2p00_mm", "fairness"
                ],
                "unmeasured_zero_credit": []
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct G5EvalRecord {
    pub schema: String,
    pub decoder_sha256: String,
    pub genome_sha256: String,
    pub tier: EvalTier,
    pub inputs: ErgolightInputs,
    pub score: ErgolightScore,
    pub descriptors: G5Descriptors,
    /// Sealed (undrainable) cavity volume and removed floating solid measured on `d_am` (zero at Tier 0).
    pub am_connectivity: AmConnectivityStats,
    /// RS-karambit-v5 load cases (`loads`): analytic at Tier 0, blade cases by plate FE from Tier 1.
    pub loads: crate::loads::G5Loads,
}

/// Exact perimeter of an asymmetric superellipse section with top semi-height `a_top`, bottom
/// semi-height `a_bot`, half-thickness `b`, and exponent `n`.
pub fn superellipse_perim(a_top: f64, a_bot: f64, b: f64, n: f64) -> f64 {
    crate::g5_sdf::superellipse_perimeter(a_top, a_bot, b, n)
}

/// Minimum curvature radius of a superellipse quarter-arc $(y/a)^n + (z/b)^n = 1$ (`evaluate.py::superellipse_rmin` parity).
pub fn superellipse_rmin(a: f64, b: f64, n: f64) -> f64 {
    let k = 2000usize;
    let t0 = 1e-4_f64;
    let t1 = std::f64::consts::FRAC_PI_2 - 1e-4;
    let dt = (t1 - t0) / ((k - 1) as f64);
    let p = 2.0 / n;
    let mut ys = Vec::with_capacity(k);
    let mut zs = Vec::with_capacity(k);
    for i in 0..k {
        let t = t0 + (i as f64) * dt;
        let (s, c) = t.sin_cos();
        ys.push(a * c.powf(p));
        zs.push(b * s.powf(p));
    }
    let np_grad = |v: &[f64]| -> Vec<f64> {
        let mut g = vec![0.0; k];
        g[0] = (v[1] - v[0]) / dt;
        for i in 1..(k - 1) {
            g[i] = (v[i + 1] - v[i - 1]) / (2.0 * dt);
        }
        g[k - 1] = (v[k - 1] - v[k - 2]) / dt;
        g
    };
    let dy = np_grad(&ys);
    let dz = np_grad(&zs);
    let ddy = np_grad(&dy);
    let ddz = np_grad(&dz);
    let mut max_kap = 1e-12_f64;
    for i in 20..(k - 20) {
        let kap = (dy[i] * ddz[i] - dz[i] * ddy[i]).abs()
            / (dy[i] * dy[i] + dz[i] * dz[i]).powf(1.5).max(1e-18);
        if kap > max_kap {
            max_kap = kap;
        }
    }
    1.0 / max_kap
}

/// Compute finger-side groove curvature, crest curvature, and station errors (`evaluate.py::finger_metrics` parity).
pub fn compute_finger_metrics(p: &BTreeMap<&'static str, f64>) -> (f64, f64, f64, f64, f64, f64) {
    let l_g = p["L_g"];
    let ds = 0.05_f64;
    let n = ((l_g + 1e-9) / ds).floor() as usize;
    let mut ss = Vec::with_capacity(n + 1);
    let mut ys = Vec::with_capacity(n + 1);
    for i in 0..=n {
        let s = (i as f64) * ds;
        let u01 = (s / l_g.max(1e-9)).clamp(0.0, 1.0);
        let base = p["yb0"]
            + (p["yb_g"] - p["yb0"]) * u01
            + p["belly"] * (std::f64::consts::PI * u01).sin();
        let bump = |t: f64| {
            let tc = t.abs().clamp(0.0, 1.0);
            (1.0 - tc * tc).powi(2)
        };
        let g_ring = p["d_ring"] * bump((s - (p["centre_ring"] + p["off_ring"])) / p["w_ring"]);
        let g_mid = p["d_mid"] * bump((s - (p["centre_mid"] + p["off_mid"])) / p["w_mid"]);
        let g_idx = p["d_idx"] * bump((s - (p["centre_idx"] + p["off_idx"])) / p["w_idx"]);
        ss.push(s);
        ys.push(base + g_ring + g_mid + g_idx);
    }
    let m = ss.len();
    let np_grad = |v: &[f64]| -> Vec<f64> {
        let mut g = vec![0.0; m];
        g[0] = (v[1] - v[0]) / ds;
        for i in 1..(m - 1) {
            g[i] = (v[i + 1] - v[i - 1]) / (2.0 * ds);
        }
        g[m - 1] = (v[m - 1] - v[m - 2]) / ds;
        g
    };
    let d1 = np_grad(&ys);
    let d2 = np_grad(&d1);
    let mut max_groove_kap = 0.0_f64;
    let mut max_crest_kap = 0.0_f64;
    for i in 0..m {
        let s = ss[i];
        if s <= 3.0 || s >= l_g - 3.0 {
            continue;
        }
        let kap = d2[i] / (1.0 + d1[i] * d1[i]).powf(1.5);
        if kap < 0.0 {
            max_groove_kap = max_groove_kap.max(-kap);
        } else if kap > 0.0 {
            max_crest_kap = max_crest_kap.max(kap);
        }
    }
    let groove_r = if max_groove_kap > 1e-6 {
        1.0 / max_groove_kap
    } else {
        999.0
    };
    let crest_r = if max_crest_kap > 1e-6 {
        1.0 / max_crest_kap
    } else {
        999.0
    };

    let specs = [
        ("centre_ring", "off_ring", "w_ring", "d_ring"),
        ("centre_mid", "off_mid", "w_mid", "d_mid"),
        ("centre_idx", "off_idx", "w_idx", "d_idx"),
    ];
    let mut depths = [0.0_f64; 3];
    let mut centres = [0.0_f64; 3];
    for (k, &(ck, ok, wk, dk)) in specs.iter().enumerate() {
        let c = p[ck] + p[ok];
        let w = p[wk];
        let mut ymin = f64::INFINITY;
        let mut ymax = f64::NEG_INFINITY;
        let mut smax = c;
        for (&s, &y) in ss.iter().zip(ys.iter()) {
            if s > c - w - 3.0 && s < c + w + 3.0 {
                if y < ymin {
                    ymin = y;
                }
                if y > ymax {
                    ymax = y;
                    smax = s;
                }
            }
        }
        depths[k] = if ymax > ymin { ymax - ymin } else { 0.0 };
        centres[k] = if p[dk] > 0.05 { smax } else { c };
    }
    let mut err = 0.0_f64;
    for k in 0..3 {
        if k != 1 || p["d_mid"] > 0.05 {
            err = err.max((centres[k] - M50_STATIONS_MM[k]).abs());
        }
    }
    (
        groove_r,
        crest_r,
        depths[0],
        depths[1],
        depths[2],
        err,
    )
}

/// Evaluate hollow-grip safety factor at $s \in \{0.25, 0.50, 0.75\} L_g$ under:
/// 1. 260 N lateral tip load + torsion
/// 2. 16.2 N·m human-max twist
/// 3. 3427 N human-max spine load (`blade_centroid_x - x` lever, `N13`, `S5-7`)
/// 4. 250 N bilateral grip squeeze over 20 mm pad (`design.md` `CHK-FEA-01`, `evaluate.py::grip_beam_sf`, `N8`).
pub fn eval_hollow_grip_sf(
    p: &BTreeMap<&'static str, f64>,
    tip_xy: (f64, f64),
    blade_centroid_x: f64,
) -> f64 {
    let rho_g = p["rho_grip"];
    let rho_e = material::core_relative_stiffness(rho_g, p["lat_cell"], p["lat_mix"]);
    let hollow = rho_g < 0.95;
    let skin_t = p["skin_t"];
    let allow = material::yield_allowable(skin_t, AS_BUILT_WALL);
    let mut worst_sf = 9.9_f64;

    for &s_frac in &[0.25, 0.50, 0.75] {
        let x = p["x_t"] + s_frac * p["L_g"];
        let gc = eval_grip_slice(p, x);
        let h_sec = (gc.yt - gc.yb).max(8.0);
        let t_sec = gc.t_sec.max(4.0);
        let a_enc = std::f64::consts::PI * 0.25 * h_sec * t_sec;
        let perim = std::f64::consts::PI
            * (3.0 * 0.5 * (h_sec + t_sec)
                - ((1.5 * h_sec + 0.5 * t_sec) * (0.5 * h_sec + 1.5 * t_sec)).sqrt());
        let sec_sand = SandwichSection::from_skin_and_lattice(
            t_sec,
            skin_t,
            rho_e,
            if p["rib_w"] > 0.0 {
                0.4 + 0.4 * p["rib_w"]
            } else {
                0.0
            },
        );
        let i_yy = (0.78 * h_sec) * sec_sand.t3_bend / 12.0;
        let h_inner = (h_sec - 2.0 * skin_t).max(0.0);
        let t_inner = (t_sec - 2.0 * skin_t).max(0.0);
        let t_rib = if hollow && p["rib_w"] > 0.0 {
            (0.8 + 0.8 * p["rib_w"]).min(t_inner)
        } else {
            0.0
        };
        let i_zz = (std::f64::consts::PI / 64.0)
            * ((t_sec * h_sec.powi(3) - t_inner * h_inner.powi(3))
                + rho_e * (t_inner - t_rib).max(0.0) * h_inner.powi(3))
            + (t_rib * h_inner.powi(3)) / 12.0;
        // N8: Do not credit `shear_web_amp` in torsion or bilateral squeeze (`evaluate.py::grip_beam_sf` parity)
        let box_tors_stiff = if hollow {
            1.0 + 0.25 * p["rib_w"]
        } else {
            1.0
        };

        let arm = (tip_xy.0 - x).max(10.0);
        let m_lat = 260.0 * arm;
        let sig_lat = m_lat * (0.5 * t_sec) / i_yy.max(1.0);
        let t_mom = 260.0 * (tip_xy.1 - gc.ym).abs();
        let tau_lat = if hollow {
            t_mom / (2.0 * a_enc * skin_t * box_tors_stiff).max(1.0)
        } else {
            3.0 * t_mom / (perim * 0.5 * t_sec.min(h_sec).powi(2)).max(1.0)
        };
        let vm_lat = (sig_lat * sig_lat + 3.0 * tau_lat * tau_lat).sqrt();

        let tau_tw = if hollow {
            16_200.0 / (2.0 * a_enc * skin_t * box_tors_stiff).max(1.0)
        } else {
            3.0 * 16_200.0 / (perim * 0.5 * t_sec.min(h_sec).powi(2)).max(1.0)
        };
        let mz_sp = 3427.0 * (blade_centroid_x - x).max(0.0);
        let sig_sp = mz_sp * (0.5 * h_sec) / i_zz.max(1.0);

        let sig_sq = if hollow {
            let q = 250.0 / (20.0 * (0.5 * h_sec.min(30.0)).max(5.0));
            let panel = 0.75 * q * p["lat_cell"].powi(2) / (skin_t * skin_t);
            let r_m = 0.125 * (h_sec + t_sec);
            let oval = 6.0 * 0.318 * (250.0 / 20.0) * r_m / (skin_t * skin_t);
            panel.max(oval)
        } else {
            1e-9
        };

        let sf = (allow / vm_lat.max(1e-6))
            .min(allow / (3.0_f64.sqrt() * tau_tw).max(1e-6))
            .min(allow / sig_sp.max(1e-6))
            .min(allow / sig_sq.max(1e-6));
        worst_sf = worst_sf.min(sf);
    }
    worst_sf
}

/// Measure Gate S1 (`max_apex_mm`) and Gate S3 (`max_half_angle_deg`) strictly at steady full-window
/// stations (`win == 1.0`, excluding the 3 mm entry ramp and unsharpened spine stations, `S5-4`, `S6-2`),
/// taking the worst (`max`) station over each active edge and failing closed to
/// `(f64::INFINITY, f64::INFINITY)` (zero sharpness credit) if any active edge has no qualifying station.
/// `EDGE-2`/`EDGE-5`: each station is sampled along the true inward edge normal, and the half-angle is the
/// steeper of the apex band (0 to 0.40 mm) and the rest of the primary bevel (0.40 to 2.0 mm), so a shoulder
/// behind the apex band is measured instead of being invisible to Gate S3.
pub fn measure_blade_edge_apex_and_angle(
    p: &BTreeMap<&'static str, f64>,
    blade: &BladeProfile2D,
) -> (f64, f64) {
    // (apex thickness, half-angle) of the section through edge point `(x, y)` along unit inward normal `n`
    let section = |x: f64, y: f64, n: (f64, f64)| -> Option<(f64, f64)> {
        let at = |s: f64| eval_blade_thickness_and_sdf2(p, blade, x + s * n.0, y + s * n.1);
        let ((t0, d0), (t40, d40), (t200, d200)) = (at(1e-6), at(0.40), at(2.0));
        if d0 > 0.0 || d40 > 0.0 || d200 > 0.0 || t0 <= 0.0 || t40 <= t0 {
            return None;
        }
        let ang_apex = ((t40 - t0) / (2.0 * 0.40)).atan().to_degrees();
        let ang_bevel = ((t200 - t40) / (2.0 * 1.60)).atan().to_degrees();
        Some((t0, ang_apex.max(ang_bevel)))
    };
    let mut max_apex_in = 0.0_f64;
    let mut max_ang_in = 0.0_f64;
    let mut has_in = false;
    let mut max_apex_sp = 0.0_f64;
    let mut max_ang_sp = 0.0_f64;
    let mut has_sp = false;
    for i in 2..blade.xs.len().saturating_sub(2) {
        let x = blade.xs[i];
        let s_arc_in = blade.cum_arc_in[i];
        let s_arc_sp = blade.cum_arc_sp[i];
        let y_mid = 0.5 * (blade.y_in[i] + blade.y_sp[i]);
        let a_tip = ((x - blade.tip_xy.0) * blade.tip_inward_dir.0
            + (y_mid - blade.tip_xy.1) * blade.tip_inward_dir.1)
            .max(0.0)
            .min((blade.x_tip - x).hypot(y_mid - blade.tip_xy.1));
        // a station counts only when the grind is already steady one serration pitch + 2.5 mm of arc behind it:
        // the window is read at the probe's x, and a 2 mm probe along a skewed normal on a steep tooth flank can sit
        // that far back along the arc, inside the 3 mm entry ramp; the window rises monotonically to the tip
        // (`EDGE-4`), so this one look-back suffices
        let ds = 2.5 + p["ser_pitch_in"].max(p["ser_pitch_sp"]).clamp(0.0, 8.0);
        let (win_in, win_sp) =
            blade_bevel_window_weights(p, blade.arc_in_mm, blade.arc_sp_mm, s_arc_in - ds, s_arc_sp - ds, a_tip);
        let tip_res = p["tip_reserve"].max(6.0);
        let tx = blade.xs[i + 1] - blade.xs[i - 1];
        let unit = |vx: f64, vy: f64| {
            let l = vx.hypot(vy).max(1e-12);
            (vx / l, vy / l)
        };
        if (win_in - 1.0).abs() <= 1e-6 && a_tip > tip_res {
            // inner edge `y_in` is the lower boundary: inward normal (-ty, tx) points to +y
            let n = unit(-(blade.y_in[i + 1] - blade.y_in[i - 1]), tx);
            if let Some((t0, ang)) = section(x, blade.y_in[i], n) {
                max_apex_in = max_apex_in.max(t0);
                max_ang_in = max_ang_in.max(ang);
                has_in = true;
            }
        }
        if (win_sp - 1.0).abs() <= 1e-6 && a_tip > tip_res {
            // spine edge `y_sp` is the upper boundary: inward normal (ty, -tx) points to -y
            let n = unit(blade.y_sp[i + 1] - blade.y_sp[i - 1], -tx);
            if let Some((t0, ang)) = section(x, blade.y_sp[i], n) {
                max_apex_sp = max_apex_sp.max(t0);
                max_ang_sp = max_ang_sp.max(ang);
                has_sp = true;
            }
        }
    }
    let req_in = blade.sharpened_arc_in_mm > 1e-6;
    let req_sp = blade.sharpened_arc_sp_mm > 1e-6;
    if (req_in && !has_in) || (req_sp && !has_sp) || (!has_in && !has_sp) {
        return (f64::INFINITY, f64::INFINITY);
    }
    let mut apex_out = 0.0_f64;
    let mut ang_out = 0.0_f64;
    if has_in {
        apex_out = apex_out.max(max_apex_in);
        ang_out = ang_out.max(max_ang_in);
    }
    if has_sp {
        apex_out = apex_out.max(max_apex_sp);
        ang_out = ang_out.max(max_ang_sp);
    }
    (apex_out, ang_out)
}

/// Serration tooth strength of one edge as built (`SERR-4`, owner: "the most formidable, but we don't want to break
/// serrated teeth"). Each tooth spans two neighbouring gullet bottoms (`serration_gullets`; micro-serration ripples
/// on a gullet floor are floor texture, not load-bearing teeth); its
/// root section is the segment between them, with the thickness of the exact blade law sampled along it. Two load
/// cases on the tooth tip, at the tooth height above the root:
/// - snag: `tooth_snag_n()` along the edge (in-plane bending of the root), times the gullet notch factor
///   `Kt = 1 + 2 sqrt(d / rho)` (Inglis) with rho the gullet radius fitted through the cut at +-0.6 mm;
/// - pry: `tooth_pry_per_tooth_n()` across the blade (out-of-plane bending of the root, thickness t(s)).
/// Returns (min SF over teeth and cases, teeth counted, mean flank angle atan(2 d / w) in degrees; w = gullet width).
pub fn serration_tooth_strength(
    p: &BTreeMap<&'static str, f64>,
    blade: &BladeProfile2D,
    inner: bool,
) -> (f64, usize, f64) {
    let (edge, smooth, sign) = if inner {
        (&blade.y_in, &blade.y_in_smooth, 1.0)
    } else {
        (&blade.y_sp, &blade.y_sp_smooth, -1.0)
    };
    let xs = &blade.xs;
    let n = xs.len();
    let (depth, gullets) = serration_gullets(edge, smooth);
    if depth < 0.2 {
        return (f64::INFINITY, 0, 0.0);
    }
    // Serrations are ground after printing (owner, 2026-10-27), so the teeth carry the bulk service allowable
    let allow = material::yield_allowable(1.0, GROUND_WALL);
    let at = |x: f64, y: f64| eval_blade_thickness_and_sdf2(p, blade, x, y);
    let interp = |v: &[f64], x: f64| -> f64 {
        let i = xs.partition_point(|&q| q < x).clamp(1, n - 1);
        let f = ((x - xs[i - 1]) / (xs[i] - xs[i - 1]).max(1e-12)).clamp(0.0, 1.0);
        v[i - 1] * (1.0 - f) + v[i] * f
    };
    let (mut sf_min, mut flank_sum) = (f64::INFINITY, 0.0_f64);
    let mut teeth = 0usize;
    // RS-karambit-v5 [serration] check_end_teeth: the outermost teeth stand between the first (last) gullet and a
    // virtual end one mean gullet spacing beyond it, so they are checked too (G3 skipped them)
    let mut stations = gullets.clone();
    if rs_v5_check_end_teeth() && gullets.len() >= 2 {
        let span = (gullets[gullets.len() - 1] - gullets[0]) / (gullets.len() - 1);
        stations.insert(0, gullets[0].saturating_sub(span));
        stations.push((gullets[gullets.len() - 1] + span).min(n - 1));
    }
    let (snag_n, pry_n) = (tooth_snag_n(), tooth_pry_per_tooth_n());
    for w in stations.windows(2) {
        let (ga, gb) = (w[0], w[1]);
        let (pa, pb) = ((xs[ga], edge[ga]), (xs[gb], edge[gb]));
        let b = (pb.0 - pa.0).hypot(pb.1 - pa.1);
        // tooth height: the furthest point of the outline between the gullets, measured off the root line
        let (ux, uy) = ((pb.0 - pa.0) / b.max(1e-9), (pb.1 - pa.1) / b.max(1e-9));
        let mut h_t = 0.0_f64;
        for k in ga..=gb {
            h_t = h_t.max(((xs[k] - pa.0) * -uy + (edge[k] - pa.1) * ux).abs());
        }
        if b < 0.3 || h_t < 0.2 {
            continue;
        }
        // root section: thickness along the root line, 0.05 mm inside the blade
        let (nx, ny) = (-uy * sign, ux * sign); // inward: +y side of the inner edge, -y side of the spine edge
        let m = 40usize;
        let (mut a0, mut a1, mut a2, mut i_op, mut t_max) = (0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64);
        let ds = b / m as f64;
        for j in 0..m {
            let sj = (j as f64 + 0.5) * ds;
            let (x, y) = (pa.0 + ux * sj + 0.05 * nx, pa.1 + uy * sj + 0.05 * ny);
            let (t, d2) = at(x, y);
            let t = if d2 <= 0.0 { t.max(0.0) } else { 0.0 };
            a0 += t * ds;
            a1 += t * sj * ds;
            a2 += t * sj * sj * ds;
            i_op += t.powi(3) / 12.0 * ds;
            t_max = t_max.max(t);
        }
        if a0 <= 1e-9 || i_op <= 1e-12 {
            sf_min = 0.0;
            continue;
        }
        let s_bar = a1 / a0;
        let i_ip = (a2 - a0 * s_bar * s_bar).max(1e-12);
        let c = s_bar.max(b - s_bar);
        // gullet radius: circle through the cut at -0.6, 0, +0.6 mm around each gullet bottom
        let rho_at = |g: usize| -> f64 {
            let (x1, x2, x3) = (xs[g] - 0.6, xs[g], xs[g] + 0.6);
            let (y1, y2, y3) = (interp(edge, x1), edge[g], interp(edge, x3));
            let (a, bb, cc) = ((x2 - x1).hypot(y2 - y1), (x3 - x2).hypot(y3 - y2), (x3 - x1).hypot(y3 - y1));
            let area2 = ((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)).abs();
            if area2 < 1e-12 { 1e3 } else { a * bb * cc / (2.0 * area2) }
        };
        let is_gullet = |g: usize| gullets.contains(&g);
        let rho_g = |g: usize| if is_gullet(g) { rho_at(g) } else { 1e3 };
        let rho = rho_g(ga).min(rho_g(gb)).max(0.02);
        let kt = 1.0 + 2.0 * (h_t / rho).sqrt();
        let sig_snag = kt * snag_n * h_t * c / i_ip;
        let sig_pry = pry_n * h_t * (0.5 * t_max) / i_op;
        sf_min = sf_min.min(allow / sig_snag.max(sig_pry).max(1e-9));
        flank_sum += (2.0 * h_t / (0.5 * b).max(1e-6)).atan().to_degrees();
        teeth += 1;
    }
    (sf_min, teeth, if teeth > 0 { flank_sum / teeth as f64 } else { 0.0 })
}

/// Serration tooth loads (RS-karambit-v5 [loads.teeth]): a snag along the edge on one tooth.
pub fn tooth_snag_n() -> f64 {
    crate::requirements::rs_v5().loads.teeth.snag_n
}

/// Lateral pry per tooth: [loads.teeth] pry_n shared over pry_teeth_sharing teeth.
pub fn tooth_pry_per_tooth_n() -> f64 {
    let t = &crate::requirements::rs_v5().loads.teeth;
    t.pry_n / t.pry_teeth_sharing as f64
}

fn rs_v5_check_end_teeth() -> bool {
    crate::requirements::rs_v5().serration.check_end_teeth
}

/// Max triangle area (mm^2) of the Tier-2 solid-tip TET10 planform mesh (converged on sharp points; see P6).
pub const TIP_FE_MAX_AREA_MM2: f64 = 0.15;

/// Fast Tier-0 analytic/1D-section evaluation (~2 ms per individual).
/// Computes continuous `ErgolightInputs`, `ErgolightScore`, and `G5Descriptors` directly from the
/// decoded geometry laws so weak individuals can be weeded out before 3D voxelization.
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

pub fn evaluate_tier0_analytic(genome: &G5Genome) -> G5EvalRecord {
    let p = genome.decode();
    let blade = build_blade_profile_2d(&p);
    let (groove_r, crest_r, saddle_d, mid_d, choil_d, station_err) = compute_finger_metrics(&p);
    let grip_sf_min = eval_hollow_grip_sf(&p, blade.tip_xy, blade.centroid_x_mm);

    // Integrate zone masses, CoM, and spin inertia about the ring axis (cx, 0)
    let cx = p["cx"];
    let r_b = FIXED_BORE_D_MM * 0.5;
    let r_od = p["R_od"];
    let skin_t = p["skin_t"];

    // 1. Ring volume & inertia (unported ring cavities are filled solid by enforce_am_connectivity,
    // so ring_cav_frac = 0.0 for fitness honesty per comms.md 2026-10-25T01:16:17Z).
    // Include od_tear and od_h2 harmonic radial area contributions (B5).
    let ring_wall = (r_od - r_b).max(2.0);
    let r_od_eff_sq = (r_od * r_od
        + r_od * p["od_tear"]
        + 0.375 * p["od_tear"].powi(2)
        + 0.5 * p["od_h2"].powi(2))
    .max((r_b + 1.5).powi(2));
    let ring_env_vol = std::f64::consts::PI
        * (r_od_eff_sq - r_b * r_b)
        * (0.45 * FIXED_LIP_W_MM + 0.55 * p["web_t"] + 0.25 * p["thumbpad"]);
    let ring_cav_frac = 0.0_f64;
    let ring_am_vol = ring_env_vol;
    let m_ring = ring_am_vol * RHO_TI64_G_MM3;
    let i_ring = 0.5 * m_ring * (r_od_eff_sq + r_b * r_b);

    // 2. Grip + guard volume & inertia up to blade.x_heel (no overlap with blade loop, B5)
    let n_grip = 32usize;
    let x_grip_end = blade.x_heel.max(p["x_t"] + p["L_g"]);
    let dx_g = (x_grip_end - p["x_t"]).max(1.0) / (n_grip as f64);
    let mut env_vol = ring_env_vol;
    let mut cav_vol = ring_env_vol * ring_cav_frac;
    let mut m_total = m_ring;
    let mut mx_total = m_ring * cx;
    let mut _my_total = 0.0_f64;
    let mut i_spin = i_ring;

    for i in 0..n_grip {
        let x = p["x_t"] + (i as f64 + 0.5) * dx_g;
        let gc = eval_grip_slice(&p, x);
        let h_sec = (gc.yt - gc.yb).max(6.0);
        let t_sec = gc.t_sec.max(3.0);
        let a_sec = 0.80 * h_sec * t_sec;
        let dv_env = a_sec * dx_g;
        // Only the ported grip cavity (x < GUARD_X_MM) drains powder; unported guard pockets beyond
        // the port count as solid unless connected to the ported grip cavity
        let rho_z = if x < GUARD_X_MM {
            p["rho_grip"]
        } else if p["rho_grip"] < 0.95 {
            p["rho_guard"]
        } else {
            1.0
        };
        let h_cav = (h_sec - 2.0 * skin_t).max(0.0);
        let t_cav = (t_sec - 2.0 * skin_t).max(0.0);
        let a_cav = if rho_z < 0.95 && t_sec > 2.0 * skin_t + 1.0 {
            0.80 * h_cav * t_cav
        } else {
            0.0
        };
        // Account for solid mid-plane rib (`rib_w`) and +/-45 deg shear webs (`shear_web_amp`) mass inside the cavity (I8)
        let a_rib = if a_cav > 0.0 && p["rib_w"] > 0.0 {
            ((0.8 + 0.8 * p["rib_w"]) * h_cav).min(a_cav)
        } else {
            0.0
        };
        let f_web = (0.18 * p["shear_web_amp"]).clamp(0.0, 0.35);
        let a_rem_cav = (a_cav - a_rib).max(0.0);
        let a_eff_void = a_rem_cav * (1.0 - f_web) * (1.0 - rho_z);
        let dv_cav = a_eff_void * dx_g;
        let dv_am = (dv_env - dv_cav).max(0.0);
        let dm = dv_am * RHO_TI64_G_MM3;
        env_vol += dv_env;
        cav_vol += dv_cav;
        m_total += dm;
        mx_total += dm * x;
        _my_total += dm * gc.ym;
        i_spin += dm * ((x - cx).powi(2) + gc.ym.powi(2) + (h_sec * h_sec) / 12.0);
    }

    // 3. Blade volume, inertia, and sandwich structural capacities (`N1`: unified `effective_blade_rho(&p)`)
    let rho_blade_eff = effective_blade_rho(&p);
    let blade_core_e = material::core_relative_stiffness(rho_blade_eff, p["lat_cell"], p["lat_mix"]);
    let n_bx = 40usize;
    let n_by = 16usize;
    let dx_b = (blade.x_tip - blade.x_heel) / (n_bx as f64);
    let mut min_blade_lat_n = f64::INFINITY;
    let mut min_twist_body_sf = f64::INFINITY;
    let mut min_twist_body_sf_raw = f64::INFINITY; // unclamped, for the 16.2 N*m human-max twist (`S2`)
    let mut min_twist_tip_sf = f64::INFINITY;
    let mut min_hm_spine_sf = f64::INFINITY;
    let (mut ltb_sum, mut ltb_n) = (0.0_f64, 0usize); // sum of sqrt(E I_y G J) over blade stations (`S3`)
    let mut blade_area_2d = 0.0_f64;
    let mut m_blade_total = 0.0_f64;

    for i in 0..n_bx {
        let u = (i as f64 + 0.5) / (n_bx as f64);
        let x = blade.x_heel + u * (blade.x_tip - blade.x_heel);
        let idx = (u * (blade.xs.len() - 1) as f64).round() as usize;
        let yi = blade.y_in[idx];
        let ys = blade.y_sp[idx];
        let h_b = (ys - yi).max(1.0);
        blade_area_2d += h_b * dx_b;
        let dy_b = h_b / (n_by as f64);
        let mut col_env = 0.0_f64;
        let mut col_cav = 0.0_f64;
        let mut col_i_yy = 0.0_f64;
        let mut col_j_open = 0.0_f64;
        let mut t_max_slice = 0.2_f64;
        for j in 0..n_by {
            let y = yi + (j as f64 + 0.5) * dy_b;
            let (t_xy, d2) = eval_blade_thickness_and_sdf2(&p, &blade, x, y);
            if d2 <= 0.0 && t_xy > 0.0 {
                t_max_slice = t_max_slice.max(t_xy);
                let dv = t_xy * dx_b * dy_b;
                col_env += dv;
                let hc = if rho_blade_eff < 0.95 && t_xy > 2.0 * skin_t + 1.0 {
                    (t_xy - 2.0 * skin_t).max(0.0)
                } else {
                    0.0
                };
                col_cav += hc * dx_b * dy_b;
                let sec = SandwichSection::from_skin_and_lattice(t_xy, skin_t, blade_core_e, 0.0);
                col_i_yy += (sec.t3_bend / 12.0) * dy_b;
                col_j_open += (sec.t3_bend / 3.0) * dy_b; // open thin-section J, a lower bound for hollow sections
            }
        }
        let dv_am = (col_env - col_cav) + col_cav * rho_blade_eff;
        let dm = dv_am * RHO_TI64_G_MM3;
        let y_mid = 0.5 * (yi + ys);
        env_vol += col_env;
        cav_vol += col_cav * (1.0 - rho_blade_eff);
        m_blade_total += dm;
        m_total += dm;
        mx_total += dm * x;
        _my_total += dm * y_mid;
        i_spin += dm * ((x - cx).powi(2) + y_mid.powi(2));

        if col_i_yy > 0.0 && col_j_open > 0.0 {
            ltb_sum += (material::e_mpa() * col_i_yy * material::g_mpa() * col_j_open).sqrt();
            ltb_n += 1;
        }
        // Structural section checks along the blade using true Euclidean distance to tip_xy (`B3`, `S5-1`: 4.0 mm exclusion)
        let a_tip = (x - blade.tip_xy.0).hypot(y_mid - blade.tip_xy.1);
        if a_tip >= 4.0 {
            let allow = blade_yield_allowable(&p, rho_blade_eff);
            let z_eff = col_i_yy / (0.5 * t_max_slice).max(0.1);
            let cap_lat_n = allow * z_eff / a_tip.max(4.0);
            min_blade_lat_n = min_blade_lat_n.min(cap_lat_n);

            // Saint-Venant / Bredt torsion of the blade section at 1.82 N*m (1820 N*mm)
            let j_tors = 0.31 * h_b * t_max_slice.powi(3);
            let tau_1820 = 1820.0 * t_max_slice / j_tors.max(0.1);
            let sf_tw_raw = allow / (3.0_f64.sqrt() * tau_1820).max(1e-6);
            let sf_tw = sf_tw_raw.min(4.0);
            if a_tip > 15.0 {
                min_twist_body_sf = min_twist_body_sf.min(sf_tw);
                min_twist_body_sf_raw = min_twist_body_sf_raw.min(sf_tw_raw);
            } else {
                min_twist_tip_sf = min_twist_tip_sf.min(sf_tw);
            }

            // In-plane spine load (3427 N distributed)
            let z_inplane = t_max_slice * h_b * h_b / 6.0;
            let m_spine = 3427.0 * 0.25 * a_tip;
            let sf_sp = allow * z_inplane / m_spine.max(1.0);
            min_hm_spine_sf = min_hm_spine_sf.min(sf_sp);
        }
    }

    let lat_rob_n = if min_blade_lat_n.is_finite() {
        min_blade_lat_n.clamp(0.0, 1200.0)
    } else {
        0.0
    };
    let strict_lat_n = lat_rob_n;
    // Failure sequence ratio: hand/grip lateral capacity divided by blade lateral capacity
    let seq_ratio = if lat_rob_n > 1.0 {
        ((grip_sf_min * 260.0) / lat_rob_n).clamp(0.0, 2.5)
    } else {
        0.0
    };

    // Analytic ring bearing (+x on handle-side bore) and ring-back (+x on rear OD) calibrated against 1.5 mm band T6
    let ring_allow = material::yield_allowable(p["web_t"], AS_BUILT_WALL);
    let ring_bearing_cap_n =
        ring_allow * (1.55 * ring_wall * (0.45 * FIXED_LIP_W_MM + 0.55 * p["web_t"]))
            / (1.0 + 0.12 * (r_b / ring_wall));
    let rear_wall = (ring_wall + p["od_tear"].max(-0.8) + p["od_h2"]).max(1.8);
    let ring_back_cap_n =
        ring_allow * (1.05 * rear_wall * (0.40 * FIXED_LIP_W_MM + 0.60 * p["web_t"]))
            / (1.0 + 0.18 * (r_od / rear_wall));

    // Swell cross-section at s_sw * L_g
    let gc_sw = eval_grip_slice(&p, p["x_t"] + p["s_sw"] * p["L_g"]);
    let swell_h = (gc_sw.yt - gc_sw.yb).max(10.0);
    let swell_t = gc_sw.t_sec.max(6.0);
    let swell_perim = superellipse_perim(
        gc_sw.yt - gc_sw.ym,
        gc_sw.ym - gc_sw.yb,
        0.5 * swell_t,
        gc_sw.n_exp,
    );
    let swell_h_over_t = swell_h / swell_t;

    // Minimum superellipse corner radius along the grip
    let mut r_corner_min = 99.0_f64;
    for k in 1..=9 {
        let s_f = (k as f64) * 0.1;
        let gc = eval_grip_slice(&p, p["x_t"] + s_f * p["L_g"]);
        let b = 0.5 * gc.t_sec;
        for a in [gc.yt - gc.ym, gc.ym - gc.yb] {
            r_corner_min = r_corner_min.min(superellipse_rmin(a, b, gc.n_exp));
        }
    }

    // AM buildability score (`evaluate.py` lines 256-260, plus `rho_blade_eff` check per `N1`)
    let mut min_strut = 9.9_f64;
    let mut any_hollow = false;
    for &rv in &[p["rho_grip"], p["rho_guard"], p["rho_ring"], rho_blade_eff] {
        if rv < 0.95 {
            any_hollow = true;
            min_strut = min_strut.min(p["lat_cell"] * 0.603 * rv.sqrt());
        }
    }
    let ports_ok = if p["rho_grip"] < 0.95 || !any_hollow {
        1.0
    } else {
        0.5
    };
    let am_build = ((if skin_t >= 0.8 { 1.0 } else { 0.0 })
        + ports_ok
        + (min_strut / 0.8).min(1.0))
        / 3.0;

    let com_x = mx_total / m_total.max(1e-6);
    let x_idx = p["x_t"] + p["centre_idx"] + p["off_idx"];
    let com_to_index_mm = (com_x - x_idx).abs();

    // Gate S measurements on the constructed blade (`B3`, `S5-4`: measured strictly at `win == 1.0`
    // stations with zero gene fallback, plus inward tip bisector point probes).
    let (max_apex_mm, max_half_angle_deg) = measure_blade_edge_apex_and_angle(&p, &blade);
    let sample_tip_t = |dist_mm: f64| -> Option<f64> {
        let px = blade.tip_xy.0 + dist_mm * blade.tip_inward_dir.0;
        let py = blade.tip_xy.1 + dist_mm * blade.tip_inward_dir.1;
        let (t_val, d2) = eval_blade_thickness_and_sdf2(&p, &blade, px, py);
        if d2 <= 0.0 && t_val.is_finite() && t_val > 0.0 {
            Some(t_val)
        } else {
            None
        }
    };
    let (t_0p25_mm, t_1p00_mm, t_2p00_mm, point_monotonic) = match (
        sample_tip_t(0.25),
        sample_tip_t(1.00),
        sample_tip_t(2.00),
    ) {
        (Some(t025), Some(t100), Some(t200)) => {
            (t025, t100, t200, t025 <= t100 && t100 <= t200)
        }
        _ => (f64::INFINITY, f64::INFINITY, f64::INFINITY, false),
    };

    // 180 N lateral duty (R = 0) on the blade: peak stress = allowable * 180 / capacity, by the surface-class model
    // with FKM mean stress (G3 divided the stress by its joint factor and then applied Marin factors on top)
    let fat_sf = if lat_rob_n > 1.0 {
        blade_fatigue_sf(blade_yield_allowable(&p, blade_rho(&p)) * 180.0 / lat_rob_n, &p)
    } else {
        0.0
    };

    // Unmeasured solver-only fields (`solid_tip_sf`, `dolfinx_rel_err`, `buckling_lambda`, `gci_admitted`, `watertight`)
    // are set to zero credit (`B2`, `S5-8` compliance: never fabricate solver verification metrics).
    let twist_body_sf = if min_twist_body_sf.is_finite() {
        min_twist_body_sf.clamp(0.0, 4.0)
    } else {
        0.0
    };
    let twist_tip_sf = if min_twist_tip_sf.is_finite() {
        min_twist_tip_sf.clamp(0.0, 3.0)
    } else {
        0.0
    };
    let hm_spine_sf = if min_hm_spine_sf.is_finite() {
        min_hm_spine_sf.clamp(0.0, 3.0)
    } else {
        0.0
    };
    // Measure profile fairness from the second-difference curvature energy of the baseline smooth
    // blade edges (`B2`, `S5-M1`: does not penalize owner-mandated serrations, and has a 0.0 floor).
    let mut d2_energy = 0.0_f64;
    for i in 1..(blade.xs.len() - 1) {
        let ds = (blade.xs[i + 1] - blade.xs[i - 1]).max(0.5);
        let d2_in = (blade.y_in_smooth[i + 1] - 2.0 * blade.y_in_smooth[i] + blade.y_in_smooth[i - 1])
            / (ds * ds);
        let d2_sp = (blade.y_sp_smooth[i + 1] - 2.0 * blade.y_sp_smooth[i] + blade.y_sp_smooth[i - 1])
            / (ds * ds);
        d2_energy += 0.5 * (d2_in * d2_in + d2_sp * d2_sp);
    }
    let rms_d2 = (d2_energy / (blade.xs.len().max(1) as f64)).sqrt();
    let fairness_t0 = (1.0 - 0.15 * (rms_d2 - 0.05).max(0.0)).clamp(0.0, 1.0);

    let inputs = ErgolightInputs {
        mass_g: m_total,
        lat_rob_cert_n: lat_rob_n,
        strict_lat_n,
        ring_bearing_cap_n,
        ring_back_cap_n,
        twist_body_sf,
        twist_tip_sf,
        twist_tipzone_strict_sf: twist_tip_sf,
        solid_tip_sf: 0.0,
        dolfinx_rel_err: 1.0,
        // `S3`: lateral-torsional buckling of the blade as a cantilever from the heel under the 3427 N spine load
        // spread along its length (Timoshenko: q_cr L = 12.85 sqrt(E I_y G J) / L^2, station-averaged section)
        buckling_lambda: if ltb_n > 0 {
            let l_b = (blade.x_tip - blade.x_heel).max(10.0);
            (12.85 * (ltb_sum / ltb_n as f64) / (l_b * l_b) / 3427.0).clamp(0.0, 50.0)
        } else {
            0.0
        },
        seq_ratio,
        gci_admitted: false,
        // `S2`: the 16.2 N*m human-max twist loads the blade body (root side), not the tip zone; scale the unclamped
        // 1.82 N*m body SF linearly (the old `twist_tip_sf * 1820/16200` read the tip zone through a 4.0 clamp: <= 0.337)
        hm_twist_sf: if min_twist_body_sf_raw.is_finite() {
            (min_twist_body_sf_raw * (1820.0 / 16_200.0)).clamp(0.0, 3.0)
        } else {
            0.0
        },
        hm_spine_sf,
        fat_sf,
        max_apex_mm,
        max_half_angle_deg,
        t_0p25_mm,
        t_1p00_mm,
        t_2p00_mm,
        point_monotonic,
        grip_sf_min,
        watertight: false,
        karambit_ok: blade.tip_xy.1 <= -25.0 + 1e-6
            && (blade.x_tip - blade.x_heel) >= 60.0 - 1e-6
            && blade.reach_chord_mm <= 85.725 + 1e-6
            && blade.sharpened_arc_in_mm <= 101.6 + 1e-6
            && blade.sharpened_arc_sp_mm <= 101.6 + 1e-6,
        fairness: fairness_t0,
        am_build,
        lip_w: FIXED_LIP_W_MM,
        bore_round: p["bore_round"],
        crown: p["crown"],
        junction_step_mm: if p["neck_r"] >= 4.0 { 0.0 } else { 0.6 },
        swell_perimeter_mm: swell_perim,
        swell_h_over_t,
        edge_r_min_mm: p["edge_r"].min((0.4 * p["t_guard"]).min(3.0).max(2.2)),
        section_corner_r_min_mm: r_corner_min,
        groove_r_min_mm: groove_r,
        crest_r_min_mm: crest_r,
        saddle_depth_mm: saddle_d,
        choil_depth_mm: choil_d,
        mid_depth_mm: mid_d,
        station_err_mm: station_err,
        l_g_mm: p["L_g"],
        guard_h_mm: p["h_g"],
        tex_amp_mm: p["tex_amp"],
        tex_crest_r_mm: p["tex_pitch"].powi(2)
            / (4.0 * std::f64::consts::PI.powi(2) * p["tex_amp"].max(1e-6)),
        com_to_index_mm,
        spin_i_g_mm2: i_spin,
        spin_i_ref_g_mm2: F075_SPIN_I_REF_G_MM2,
    };

    let score = score_ergolight(&inputs);
    let outline_len = 2.0 * (p["L_g"] + blade.x_tip - blade.x_heel) + 2.0 * std::f64::consts::PI * r_od;
    let total_area = (p["L_g"] * swell_h + blade_area_2d + std::f64::consts::PI * r_od * r_od).max(100.0);
    let inner_turn_deg = (p["hook_drop"] / (blade.x_tip - blade.x_heel).max(1.0))
        .atan()
        .to_degrees()
        * 1.8
        + p["sweep_deg"].abs();
    let coverage = (p["scope_grip"] + 0.3 * p["scope_ring"] + 0.5 * p["scope_blade"]) / 1.8;
    let descriptors = G5Descriptors {
        hook_drop_mm: p["hook_drop"],
        inner_turn_deg,
        silhouette_complexity: outline_len / total_area.sqrt(),
        grip_roundness_h_over_t: swell_h_over_t,
        groove_articulation_mm: saddle_d + mid_d + choil_d,
        porosity_frac: (cav_vol / env_vol.max(1.0)).clamp(0.0, 0.85),
        surface_energy: p["tex_amp"] * coverage,
        mass_g: m_total,
    };

    let _ = m_blade_total;
    let mut loads = crate::loads::analytic_loads(&p, &blade, ring_bearing_cap_n, ring_back_cap_n);
    loads.teeth_sf = [true, false]
        .iter()
        .map(|&inner| serration_tooth_strength(&p, &blade, inner).0)
        .fold(f64::INFINITY, f64::min);
    G5EvalRecord {
        schema: genome.schema.clone(),
        decoder_sha256: g5_decoder_sha256(),
        genome_sha256: genome.sha256(),
        tier: EvalTier::Tier0Analytic,
        inputs,
        score,
        descriptors,
        am_connectivity: AmConnectivityStats::default(),
        loads,
    }
}

/// Planform point of the blade (RS-karambit-v5 `[point]`): the included angle between the two edges seen from the
/// tip at 1 mm and 3 mm, and the width across the edges at 2 mm. Each edge point is the point of that edge's outline
/// at the given straight-line distance from the tip.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PointPlanform {
    pub angle_1mm_deg: f64,
    pub angle_3mm_deg: f64,
    pub width_2mm: f64,
}

pub fn measure_point_planform(b: &BladeProfile2D) -> PointPlanform {
    let tip = b.tip_xy;
    let n = b.xs.len();
    let at = |ys: &[f64], s: f64| -> (f64, f64) {
        let mut prev = (b.xs[n - 1], ys[n - 1]);
        let mut d_prev = (prev.0 - tip.0).hypot(prev.1 - tip.1);
        for k in (0..n - 1).rev() {
            let cur = (b.xs[k], ys[k]);
            let d_cur = (cur.0 - tip.0).hypot(cur.1 - tip.1);
            if d_cur >= s {
                let f = ((s - d_prev) / (d_cur - d_prev).max(1e-12)).clamp(0.0, 1.0);
                return (prev.0 + f * (cur.0 - prev.0), prev.1 + f * (cur.1 - prev.1));
            }
            prev = cur;
            d_prev = d_cur;
        }
        prev
    };
    let angle = |s: f64| -> f64 {
        let (a, c) = (at(&b.y_in, s), at(&b.y_sp, s));
        let (va, vc) = ((a.0 - tip.0, a.1 - tip.1), (c.0 - tip.0, c.1 - tip.1));
        let cosv = (va.0 * vc.0 + va.1 * vc.1) / (va.0.hypot(va.1) * vc.0.hypot(vc.1)).max(1e-12);
        cosv.clamp(-1.0, 1.0).acos().to_degrees()
    };
    let (a2, c2) = (at(&b.y_in, 2.0), at(&b.y_sp, 2.0));
    PointPlanform {
        angle_1mm_deg: angle(1.0),
        angle_3mm_deg: angle(3.0),
        width_2mm: (a2.0 - c2.0).hypot(a2.1 - c2.1),
    }
}

/// Mass of one design on two grids, h and h/2, for the RS-karambit-v5 `[mass] grid_agreement_max_frac` check.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct MassGridAgreement {
    pub h_mm: f64,
    pub mass_h_g: f64,
    pub mass_half_h_g: f64,
    /// |m(h) − m(h/2)| / m(h/2).
    pub rel_diff: f64,
    pub sealed_void_h_mm3: f64,
    pub sealed_void_half_h_mm3: f64,
}

impl MassGridAgreement {
    /// True when the two masses agree within the requirement set's tolerance.
    pub fn within_requirement(&self) -> bool {
        self.rel_diff <= crate::requirements::rs_v5().mass.grid_agreement_max_frac
    }
}

/// Build the AM field at h and at h/2 and compare the partial-volume masses. Finalists must agree within
/// `[mass] grid_agreement_max_frac` (2 %); a design that does not is reported, never ranked on the coarse mass.
pub fn mass_grid_agreement(genome: &G5Genome, h: f64) -> MassGridAgreement {
    let coarse = build_g5_sdf(genome, h, true);
    let cx = genome.decode()["cx"];
    let (m_h, ..) = coarse.am_mass_com_inertia(cx);
    let sealed_h = coarse.am_connectivity.sealed_void_mm3;
    drop(coarse);
    let fine = build_g5_sdf(genome, 0.5 * h, true);
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
pub fn evaluate_tier1_sdf_and_plate(genome: &G5Genome, h_vox: f64) -> G5EvalRecord {
    evaluate_tier1_with_grid(genome, h_vox).0
}

fn evaluate_tier1_with_grid(genome: &G5Genome, h_vox: f64) -> (G5EvalRecord, VoxelGrid3D) {
    let mut rec = evaluate_tier0_analytic(genome);
    let p = genome.decode();
    let grid = build_g5_sdf(genome, h_vox, true);

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
            let ring = &crate::requirements::rs_v5().loads.ring;
            rec.loads.ring_bearing_sf = rb_res.bearing.capacity_n / ring.bearing_n;
            rec.loads.ring_back_sf = rb_res.back.capacity_n / ring.back_n;
            rec.loads.ring_any_direction_sf = rb_res.any_direction_min_capacity_n / ring.any_direction_n;
        }
    }

    // RS-karambit-v5 blade cases on the fine whole-blade plate (`loads::plate_blade_cases`). If the mesh or a checked
    // solve fails, the Tier-0 analytic values stay and the record keeps `blade_model = "analytic"`.
    let _ = crate::loads::plate_blade_cases(&p, &blade, &mut rec.loads);

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
    rec.score = score_ergolight(&rec.inputs);
    (rec, grid)
}

/// 2.5D extruded `TET10` ring bearing, pull, and ring-back check (`comms.md` 02:25:54Z) with 1.5 mm band load,
/// grip clamped ~20 mm from the ring tangent (`seat = (x_t + 20.0, x_t + 26.0)`), and strict
/// Cholesky relative residual gate `true_rel_res <= 1e-10`.
pub fn evaluate_ring_2p5d_extruded_tet10(
    genome: &G5Genome,
    h_vox: f64,
) -> Option<crate::g5_sandwich::Ring3dTet10Summary> {
    let p = genome.decode();
    let grid = build_g5_sdf(genome, h_vox, true);
    let cx = p["cx"];
    let r_b = FIXED_BORE_D_MM * 0.5;
    let r_od = p["R_od"];
    let seat_x0 = p["x_t"] + 20.0;
    let seat_x1 = p["x_t"] + 26.0;
    let x_clip = seat_x1 + 0.5;
    let th_join = p["junction_deg"].clamp(24.0, 45.0).to_radians();
    let mut outer = Vec::with_capacity(64);
    let n_arc = 36usize;
    for k in 0..=n_arc {
        let th = th_join
            + (k as f64 / n_arc as f64) * (2.0 * std::f64::consts::PI - 2.0 * th_join);
        let r_th = r_od + p["od_tear"] * 0.5 * (1.0 + th.cos()) + p["od_h2"] * (2.0 * th).cos();
        outer.push((cx + r_th * th.cos(), r_th * th.sin()));
    }
    let x_join = (cx + r_od * th_join.cos()).max(p["x_t"]);
    let n_neck = 14usize;
    for k in 1..=n_neck {
        let x = x_join + (k as f64 / n_neck as f64) * (x_clip - x_join);
        let gc = eval_grip_slice(&p, x);
        outer.push((x, gc.yb));
    }
    for k in (1..=n_neck).rev() {
        let x = x_join + (k as f64 / n_neck as f64) * (x_clip - x_join);
        let gc = eval_grip_slice(&p, x);
        outer.push((x, gc.yt));
    }
    let hole: Vec<(f64, f64)> = (0..40)
        .map(|k| {
            let th = (k as f64) * 2.0 * std::f64::consts::PI / 40.0;
            (cx + r_b * th.cos(), r_b * th.sin())
        })
        .collect();
    let mesh = mesh_domain(&outer, &[hole], 0.9)?;
    let t_at = |x: f64, y: f64| -> f64 { grid.ring_tmap_eff_bilinear_at(x, y).max(0.3) };
    crate::g5_sandwich::solve_ring_2p5d_extruded_tet10(
        &mesh,
        &t_at,
        cx,
        r_b,
        r_od,
        p["web_t"],
        (seat_x0, seat_x1),
        2,
    )
}

/// Tier-2 evaluation for top elites: reuses the Tier-1 `VoxelGrid3D` (`N12`), extracts the watertight
/// 3D mesh (`extract_watertight_mesh`), checks exact manifold topology (`census == (0, 0, 1)`), and
/// runs the fail-closed 3D `TET10` tip submodel (`solve_g5_solid_tet10_strict`).
/// Fail-closed (`B2`, `B3`): `solid_tip_sf` starts at `0.0` and is only populated if the strict
/// Cholesky solve (`true_rel_res <= 1e-10`) succeeds; `dolfinx_rel_err` remains `1.0` unless an
/// independent DOLFINx cross-check is provided (never substituting the linear-solver residual).
pub fn evaluate_tier2_elites(genome: &G5Genome, h_vox: f64) -> G5EvalRecord {
    let (rec1, grid) = evaluate_tier1_with_grid(genome, h_vox);
    evaluate_tier2_from_tier1(genome, rec1, &grid)
}

fn evaluate_tier2_from_tier1(
    genome: &G5Genome,
    mut rec: G5EvalRecord,
    grid: &VoxelGrid3D,
) -> G5EvalRecord {
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
    if let Some(b_mesh) = mesh_domain(&b_outer, &[], TIP_FE_MAX_AREA_MM2) {
        let x0 = blade.x_tip - 32.0;
        let y0 = blade.tip_xy.1 - 32.0;
        let g = 0.5;
        let nx = 130usize;
        let ny = 130usize;
        let th_fn = |x: f64, y: f64| -> Option<f64> {
            let (t, d2) = eval_blade_thickness_and_sdf2(&p, &blade, x, y);
            if d2 <= 0.0 && t > 0.05 {
                Some(t)
            } else {
                None
            }
        };
        if let Some(tip_res) = solve_g5_solid_tet10_strict(
            b_mesh.xy,
            b_mesh.tris,
            blade.tip_xy,
            p["t_core"],
            x0,
            y0,
            g,
            nx,
            ny,
            &th_fn,
            0,
        ) {
            rec.inputs.solid_tip_sf = tip_res.sf_nodal_max;
        }
    }

    rec.tier = EvalTier::Tier2Full3D;
    rec.score = score_ergolight(&rec.inputs);
    rec
}

/// Evaluate a candidate through the 3-tier multi-fidelity cascade:
/// - Always runs `Tier 0` (~2 ms).
/// - Promotes to `Tier 1` if `score_100 >= tier1_gate_score`.
/// - Promotes to `Tier 2` if `score_100 >= tier2_gate_score` (reusing the Tier-1 `VoxelGrid3D`, `N12`).
pub fn evaluate_g5_cascade(
    genome: &G5Genome,
    tier1_gate_score: f64,
    tier2_gate_score: f64,
    h_vox: f64,
) -> G5EvalRecord {
    let r0 = evaluate_tier0_analytic(genome);
    if r0.score.score_100 < tier1_gate_score {
        return r0;
    }
    let (r1, grid) = evaluate_tier1_with_grid(genome, h_vox);
    if r1.score.score_100 < tier2_gate_score {
        return r1;
    }
    evaluate_tier2_from_tier1(genome, r1, &grid)
}

// ---------------- 1024-Niche CVT-MAP-Elites Archive ----------------

#[derive(Debug, Clone, Serialize)]
pub struct NicheElite {
    pub niche_id: usize,
    pub genome: G5Genome,
    pub record: G5EvalRecord,
    pub curiosity: f64,
}

#[derive(Debug, Clone)]
pub struct CvtMapElitesArchive {
    pub schema: String,
    pub decoder_sha256: String,
    pub centroids: Vec<[f64; 8]>,
    pub elites: BTreeMap<usize, NicheElite>,
}

impl CvtMapElitesArchive {
    /// Construct a deterministic `n_niches` (default 1024) CVT-MAP-Elites archive in $[0, 1]^8$.
    pub fn new(n_niches: usize) -> Self {
        const BASES: [usize; 8] = [2, 3, 5, 7, 11, 13, 17, 19];
        let mut centroids = Vec::with_capacity(n_niches);
        for idx in 0..n_niches {
            let mut c = [0.0_f64; 8];
            for (d, &b) in BASES.iter().enumerate() {
                let mut n = idx + 1;
                let mut f = 1.0 / (b as f64);
                let mut v = 0.0;
                while n > 0 {
                    v += (n % b) as f64 * f;
                    n /= b;
                    f /= b as f64;
                }
                c[d] = v;
            }
            centroids.push(c);
        }
        Self {
            schema: "G5.0".to_string(),
            decoder_sha256: g5_decoder_sha256(),
            centroids,
            elites: BTreeMap::new(),
        }
    }

    /// Find the nearest CVT centroid index for an 8D descriptor vector.
    pub fn niche_of(&self, desc: &G5Descriptors) -> usize {
        let u = desc.to_unit8();
        let mut best_id = 0usize;
        let mut best_d2 = f64::INFINITY;
        for (i, c) in self.centroids.iter().enumerate() {
            let mut d2 = 0.0_f64;
            for k in 0..8 {
                let df = u[k] - c[k];
                d2 += df * df;
            }
            if d2 < best_d2 {
                best_d2 = d2;
                best_id = i;
            }
        }
        best_id
    }

    /// Insert a candidate into its CVT niche if the niche is empty or the candidate improves
    /// `(record.tier, record.score.raw_score_100)` (`I9`, `M9`). A lower-tier evaluation (e.g.
    /// Tier 0) can NEVER displace a higher-tier elite in the same niche. Refuses to mix evaluations
    /// if `schema` or `decoder_sha256` drifted mid-run (`comms.md` `2026-10-25T01:14:45Z`).
    pub fn insert(&mut self, genome: G5Genome, record: G5EvalRecord) -> bool {
        assert_eq!(
            record.schema, self.schema,
            "refusing to insert record with mismatched schema {} into {} archive",
            record.schema, self.schema
        );
        assert_eq!(
            record.decoder_sha256, self.decoder_sha256,
            "refusing to insert record with drifted decoder_sha256 {} into frozen archive {}",
            record.decoder_sha256, self.decoder_sha256
        );
        if !record.score.raw_score_100.is_finite() {
            return false;
        }
        let nid = self.niche_of(&record.descriptors);
        let improves = match self.elites.get(&nid) {
            None => true,
            Some(existing) => record
                .tier
                .cmp(&existing.record.tier)
                .then_with(|| {
                    record
                        .score
                        .raw_score_100
                        .total_cmp(&existing.record.score.raw_score_100)
                })
                .is_gt(),
        };
        if improves {
            self.elites.insert(
                nid,
                NicheElite {
                    niche_id: nid,
                    genome,
                    record,
                    curiosity: 1.0,
                },
            );
        }
        improves
    }

    /// Select an offspring using Claude-HZ's hybrid emitter strategy (`comms.md` `00:37:19Z`)
    /// without ever overwriting any parent gene (`I3`: self-adaptive step sizes in `174..179`
    /// evolve naturally under `reproduce_iso_line_dd`):
    /// - 40 % Improvement Emitter: draw parents from top-16 elites by `(tier, W_ERGOLIGHT_v1)`.
    /// - 60 % Curiosity-Weighted MAP-Elites Emitter: `Iso+Line-DD` + module crossover (`p = 0.15`).
    pub fn sample_offspring(&mut self, rng: &mut G5Rng) -> Option<(G5Genome, usize)> {
        if self.elites.is_empty() {
            return None;
        }
        let mut all_nids: Vec<usize> = self.elites.keys().copied().collect();
        all_nids.sort_by(|&a, &b| {
            let ea = &self.elites[&a].record;
            let eb = &self.elites[&b].record;
            eb.tier
                .cmp(&ea.tier)
                .then_with(|| eb.score.raw_score_100.total_cmp(&ea.score.raw_score_100))
        });

        if rng.next_f64() < 0.40 {
            let top_k = all_nids.len().min(16);
            let nid_a = all_nids[(rng.next_u64() as usize) % top_k];
            let nid_b = all_nids[(rng.next_u64() as usize) % top_k];
            let pa = &self.elites[&nid_a].genome;
            let pb = &self.elites[&nid_b].genome;
            let child = reproduce_iso_line_dd(pa, pb, rng);
            Some((child, nid_a))
        } else {
            // Curiosity-weighted tournament of size 3
            let pick_curious = |rng: &mut G5Rng, nids: &[usize], elites: &BTreeMap<usize, NicheElite>| -> usize {
                let mut best = nids[(rng.next_u64() as usize) % nids.len()];
                for _ in 0..2 {
                    let cand = nids[(rng.next_u64() as usize) % nids.len()];
                    if elites[&cand].curiosity > elites[&best].curiosity {
                        best = cand;
                    }
                }
                best
            };
            let nid_a = pick_curious(rng, &all_nids, &self.elites);
            let nid_b = pick_curious(rng, &all_nids, &self.elites);
            let pa = &self.elites[&nid_a].genome;
            let pb = &self.elites[&nid_b].genome;
            let child = reproduce_iso_line_dd(pa, pb, rng);
            Some((child, nid_a))
        }
    }

    pub fn update_curiosity(&mut self, parent_niche: usize, child_improved: bool) {
        if let Some(e) = self.elites.get_mut(&parent_niche) {
            if child_improved {
                e.curiosity += 1.0;
            } else {
                e.curiosity = (e.curiosity - 0.15).max(0.1);
            }
        }
    }

    pub fn best_elite(&self) -> Option<&NicheElite> {
        self.elites.values().max_by(|a, b| {
            a.record
                .tier
                .cmp(&b.record.tier)
                .then_with(|| {
                    a.record
                        .score
                        .raw_score_100
                        .total_cmp(&b.record.score.raw_score_100)
                })
        })
    }

    pub fn qd_score(&self) -> f64 {
        self.elites
            .values()
            .map(|e| e.record.score.raw_score_100)
            .sum()
    }

    /// Self-check all archived elites (`I7`): verifies `schema`, frozen `decoder_sha256`,
    /// genome validity, `genome.sha256() == record.genome_sha256`, and CVT niche assignment.
    pub fn verify_integrity(&self) -> Result<(), String> {
        if self.schema != "G5.0" {
            return Err(format!("archive schema mismatch: '{}'", self.schema));
        }
        let expected_dec = g5_decoder_sha256();
        if self.decoder_sha256 != expected_dec {
            return Err(format!(
                "archive decoder_sha256 mismatch: '{}' vs current '{expected_dec}'",
                self.decoder_sha256
            ));
        }
        for (&nid, elite) in &self.elites {
            if elite.niche_id != nid {
                return Err(format!(
                    "archive key {nid} != elite.niche_id {}",
                    elite.niche_id
                ));
            }
            elite.genome.validate()?;
            let g_sha = elite.genome.sha256();
            if elite.record.genome_sha256 != g_sha {
                return Err(format!(
                    "niche {nid} genome_sha256 mismatch: record='{}', genome='{g_sha}'",
                    elite.record.genome_sha256
                ));
            }
            if elite.record.schema != self.schema {
                return Err(format!(
                    "niche {nid} record schema mismatch: '{}'",
                    elite.record.schema
                ));
            }
            if elite.record.decoder_sha256 != self.decoder_sha256 {
                return Err(format!(
                    "niche {nid} record decoder_sha256 mismatch: '{}'",
                    elite.record.decoder_sha256
                ));
            }
            let computed_nid = self.niche_of(&elite.record.descriptors);
            if computed_nid != nid {
                return Err(format!(
                    "niche {nid} CVT assignment mismatch: descriptors map to niche {computed_nid}"
                ));
            }
        }
        Ok(())
    }

    /// Load and verify a persisted `{genomes_g5.csv, archive.jsonl}` run directory (`I5`, `I7`, `S8-2`, `S8-3`):
    /// - Parses every CSV row via `G5Genome::from_csv_row_checked(row, expected_commit)` (`I5`),
    /// - Verifies 1-to-1 join with `archive.jsonl` on `design_id` and `genome_sha256`,
    /// - Verifies `decoder_sha256 == g5_decoder_sha256()` (`I7`),
    /// - Verifies CVT `niche_id == niche_of(&descriptors)`,
    /// - Re-scores every row's `inputs` via `score_ergolight_map` and checks `score_100`, `base`, and `E` (`S8-2`).
    pub fn load_and_verify_persisted_dir(
        dir: &str,
        expected_commit: &str,
    ) -> Result<usize, String> {
        let csv_raw = std::fs::read_to_string(format!("{dir}/genomes_g5.csv"))
            .map_err(|e| format!("read genomes_g5.csv: {e}"))?;
        let jsonl_raw = std::fs::read_to_string(format!("{dir}/archive.jsonl"))
            .map_err(|e| format!("read archive.jsonl: {e}"))?;
        let mut csv_lines = csv_raw.lines();
        let _hdr = csv_lines
            .next()
            .ok_or_else(|| "empty genomes_g5.csv".to_string())?;
        let csv_rows: Vec<&str> = csv_lines.filter(|l| !l.trim().is_empty()).collect();
        let jsonl_rows: Vec<&str> = jsonl_raw
            .lines()
            .filter(|l| !l.trim().is_empty())
            .collect();
        if csv_rows.is_empty() || csv_rows.len() != jsonl_rows.len() {
            return Err(format!(
                "persisted row count mismatch: csv={} vs jsonl={}",
                csv_rows.len(),
                jsonl_rows.len()
            ));
        }
        let expected_dec = g5_decoder_sha256();
        let ref_arch = Self::new(1024);
        for (idx, (&c_line, &j_line)) in csv_rows.iter().zip(jsonl_rows.iter()).enumerate() {
            let (design_id, genome_part) = c_line
                .split_once(',')
                .ok_or_else(|| format!("malformed csv row {idx}"))?;
            crate::g5_namespace::validate_g5_design_id(design_id)?;
            let genome = G5Genome::from_csv_row_checked(genome_part, expected_commit)?;
            let doc: serde_json::Value = serde_json::from_str(j_line)
                .map_err(|e| format!("parse archive.jsonl row {idx}: {e}"))?;
            if doc["design_id"].as_str() != Some(design_id) {
                return Err(format!("row {idx} design_id mismatch"));
            }
            if doc["schema"].as_str() != Some("G5.0") {
                return Err(format!("row {idx} schema mismatch"));
            }
            if doc["decoder_commit"].as_str() != Some(expected_commit) {
                return Err(format!("row {idx} decoder_commit mismatch"));
            }
            if doc["decoder_sha256"].as_str() != Some(expected_dec.as_str()) {
                return Err(format!(
                    "row {idx} decoder_sha256 mismatch (persisted archive from a different decoder cannot be mixed)"
                ));
            }
            let g_sha = genome.sha256();
            if doc["genome_sha256"].as_str() != Some(g_sha.as_str()) {
                return Err(format!("row {idx} genome_sha256 mismatch"));
            }
            let nid = doc["niche_id"]
                .as_u64()
                .ok_or_else(|| format!("row {idx} missing niche_id"))?
                as usize;
            let desc: G5Descriptors = serde_json::from_value(doc["descriptors"].clone())
                .map_err(|e| format!("row {idx} invalid descriptors: {e}"))?;
            if ref_arch.niche_of(&desc) != nid {
                return Err(format!("row {idx} CVT niche mismatch"));
            }
            if !doc["tier_provenance"].is_object() {
                return Err(format!("row {idx} missing tier_provenance object"));
            }
            let inputs_map = doc["inputs"]
                .as_object()
                .ok_or_else(|| format!("row {idx} inputs is not a JSON object"))?;
            let rescored = crate::ergolight::score_ergolight_map(inputs_map)
                .map_err(|e| format!("row {idx} score_ergolight_map failed: {e}"))?;
            let s_row = doc["score_100"]
                .as_f64()
                .ok_or_else(|| format!("row {idx} missing score_100"))?;
            let s_re = rescored["score_100"]
                .as_f64()
                .ok_or_else(|| format!("row {idx} rescored missing score_100"))?;
            if (s_row - s_re).abs() > 1e-9 {
                return Err(format!(
                    "row {idx} re-score mismatch: stored {s_row} vs rescored {s_re}"
                ));
            }
        }
        Ok(csv_rows.len())
    }
}

/// SplitMix64 bijective mixer for stream seed derivation without overflow (`S5-M6`).
#[inline]
pub fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Run a pure Sobol-seeded CVT-MAP-Elites evolutionary search under `W_ERGOLIGHT_v1` (draft-d)
/// with the 3-tier cascade (zero manual design of individuals).
pub fn run_g5_cvt_map_elites(
    n_init_sobol: usize,
    n_evals: usize,
    seed: u64,
    tier1_gate_score: f64,
    tier2_gate_score: f64,
    h_vox: f64,
    decoder_commit: &str,
) -> CvtMapElitesArchive {
    run_g5_cvt_map_elites_logged(
        n_init_sobol,
        n_evals,
        seed,
        tier1_gate_score,
        tier2_gate_score,
        h_vox,
        decoder_commit,
        0,
        &mut Vec::new(),
    )
}

/// One snapshot of a run's progress, for the evolution figures and the lineage renders of a release.
/// Logging reads the archive only; it draws no random numbers, so a logged run is identical to an unlogged one.
#[derive(Debug, Clone, Serialize)]
pub struct G5HistoryPoint {
    pub evals: usize,
    pub niches: usize,
    pub n_tier2: usize,
    pub qd_score: f64,
    pub best_tier: EvalTier,
    pub best_raw_score_100: f64,
    pub best_score_100: f64,
    pub best_mass_g: f64,
    pub best_niche: usize,
    pub best_genome_sha256: String,
    /// The best genome's strict CSV row, at power-of-two evaluation counts and at the end of the run.
    pub best_genome_row: Option<String>,
}

fn history_point(archive: &CvtMapElitesArchive, evals: usize, with_row: bool) -> Option<G5HistoryPoint> {
    let best = archive.best_elite()?;
    Some(G5HistoryPoint {
        evals,
        niches: archive.elites.len(),
        n_tier2: archive.elites.values().filter(|e| e.record.tier == EvalTier::Tier2Full3D).count(),
        qd_score: archive.qd_score(),
        best_tier: best.record.tier,
        best_raw_score_100: best.record.score.raw_score_100,
        best_score_100: best.record.score.score_100,
        best_mass_g: best.record.inputs.mass_g,
        best_niche: best.niche_id,
        best_genome_sha256: best.genome.sha256(),
        best_genome_row: with_row.then(|| best.genome.to_csv_row()),
    })
}

/// `run_g5_cvt_map_elites` that also appends a `G5HistoryPoint` to `history` every `log_every` evaluations
/// (`0` disables logging), after the Sobol initialisation, and after the last evaluation.
#[allow(clippy::too_many_arguments)]
pub fn run_g5_cvt_map_elites_logged(
    n_init_sobol: usize,
    n_evals: usize,
    seed: u64,
    tier1_gate_score: f64,
    tier2_gate_score: f64,
    h_vox: f64,
    decoder_commit: &str,
    log_every: usize,
    history: &mut Vec<G5HistoryPoint>,
) -> CvtMapElitesArchive {
    let mut archive = CvtMapElitesArchive::new(1024);
    let mut rng = G5Rng(seed.max(1));
    let mixed_seed = splitmix64(seed);
    let scramble_seed = (mixed_seed as usize).max(1);
    let mut log = |archive: &CvtMapElitesArchive, evals: usize, last: bool| {
        if log_every > 0 && (last || evals % log_every == 0 || evals.is_power_of_two()) {
            if let Some(h) = history_point(archive, evals, last || evals.is_power_of_two()) {
                history.push(h);
            }
        }
    };

    let init_count = n_init_sobol.min(n_evals).max(1);
    for i in 0..init_count {
        let g = sobol_unit_genome(i, scramble_seed, decoder_commit);
        let rec = evaluate_g5_cascade(&g, tier1_gate_score, tier2_gate_score, h_vox);
        archive.insert(g, rec);
    }
    let mut evals = init_count;
    log(&archive, evals, evals == n_evals);

    for _ in init_count..n_evals {
        let Some((child, parent_nid)) = archive.sample_offspring(&mut rng) else {
            break;
        };
        let rec = evaluate_g5_cascade(&child, tier1_gate_score, tier2_gate_score, h_vox);
        let improved = archive.insert(child, rec);
        archive.update_curiosity(parent_nid, improved);
        evals += 1;
        log(&archive, evals, evals == n_evals);
    }
    if evals < n_evals {
        log(&archive, evals, true); // the archive stopped offering parents: still close the history
    }
    archive
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sobol_population_validity_heritability_and_cvt_qd_growth() {
        // 1. Validity on 100 Sobol genomes (0 decode failures, finite scores in (0, 100])
        for i in 0..100 {
            let g = sobol_unit_genome(i, 0, "22bb8696");
            let rec = evaluate_tier0_analytic(&g);
            assert!(
                rec.score.score_100.is_finite() && rec.score.score_100 > 10.0,
                "Sobol genome {i} produced invalid score {}",
                rec.score.score_100
            );
            assert!(rec.inputs.max_apex_mm <= 0.19 + 1e-9);
            // RS-karambit-v5 [edge] 20-25 deg per side, plus the hollow/convex grind modulation beyond 0.4 mm
            assert!(rec.inputs.max_half_angle_deg <= 26.0 + 1e-9);
            assert!(rec.inputs.point_monotonic);
        }

        // 2. Pure Sobol-seeded CVT-MAP-Elites evolution improves best score and QD score
        let arch_init = run_g5_cvt_map_elites(24, 24, 7, 999.0, 999.0, 1.0, "22bb8696");
        let arch_evol = run_g5_cvt_map_elites(24, 160, 7, 999.0, 999.0, 1.0, "22bb8696");
        assert!(
            arch_evol.elites.len() > arch_init.elites.len(),
            "niches must grow: {} -> {}",
            arch_init.elites.len(),
            arch_evol.elites.len()
        );
        assert!(
            arch_evol.qd_score() > arch_init.qd_score(),
            "QD score must grow: {} -> {}",
            arch_init.qd_score(),
            arch_evol.qd_score()
        );
        let best_init = arch_init.best_elite().unwrap().record.score.raw_score_100;
        let best_evol = arch_evol.best_elite().unwrap().record.score.raw_score_100;
        assert!(
            best_evol >= best_init,
            "best W_ERGOLIGHT_v1 must not regress: {best_init} -> {best_evol}"
        );
    }

    #[test]
    fn tier1_and_tier2_cascade_execute_cleanly_on_evolved_elite() {
        let arch = run_g5_cvt_map_elites(16, 48, 19, 999.0, 999.0, 1.2, "22bb8696");
        let elite = arch.best_elite().expect("best elite");
        let r1 = evaluate_tier1_sdf_and_plate(&elite.genome, 1.2);
        assert_eq!(r1.tier, EvalTier::Tier1SdfAndPlate);
        assert!(r1.score.score_100 > 20.0);
        assert!(r1.inputs.ring_bearing_cap_n > 200.0);
        assert!(r1.inputs.ring_back_cap_n > 100.0);
    }

    #[test]
    fn r2_both_edges_serrated_measured_twist_and_buckling() {
        // SERR-1..3: every design the decoder can express carries real serrations on both edges
        let mut hm = Vec::new();
        for i in 0..60 {
            let g = sobol_unit_genome(i, 0, "22bb8696");
            let p = g.decode();
            let b = build_blade_profile_2d(&p);
            assert!(b.serr_depth_in_mm >= 0.6 && b.serr_depth_sp_mm >= 0.6,
                "Sobol {i}: serration depth in {:.2} / sp {:.2} mm", b.serr_depth_in_mm, b.serr_depth_sp_mm);
            assert!(b.serr_teeth_in >= 3 && b.serr_teeth_sp >= 3,
                "Sobol {i}: teeth in {} / sp {}", b.serr_teeth_in, b.serr_teeth_sp);
            // Tooth strength is measured on every edge; under RS-karambit-v5 it is a scored term ([loads.teeth]),
            // not a decoder guarantee as G3's SERR-4 was
            for inner in [true, false] {
                let (sf, teeth, _) = serration_tooth_strength(&p, &b, inner);
                assert!(teeth >= 2 && sf.is_finite() && sf > 0.0, "Sobol {i}: tooth SF {sf:.2} over {teeth} teeth (inner {inner})");
            }
            let r0 = evaluate_tier0_analytic(&g);
            // S3: buckling is measured (lateral-torsional, analytic on the exact blade law), never zero credit
            assert!(r0.inputs.buckling_lambda > 0.0 && r0.inputs.buckling_lambda.is_finite(), "Sobol {i}: buckling");
            hm.push(r0.inputs.hm_twist_sf);
        }
        // S2: the human-max twist is read on the blade body, not capped at 3 * 1820/16200 = 0.337 any more
        assert!(hm.iter().filter(|&&v| v > 0.34).count() > hm.len() / 2, "hm_twist still capped: {hm:?}");

        // Evolution history: logging draws no random numbers, so the archive is identical, and best-so-far never drops
        let plain = run_g5_cvt_map_elites(16, 64, 3, 999.0, 999.0, 1.0, "22bb8696");
        let mut hist = Vec::new();
        let logged = run_g5_cvt_map_elites_logged(16, 64, 3, 999.0, 999.0, 1.0, "22bb8696", 8, &mut hist);
        assert_eq!(plain.elites.len(), logged.elites.len());
        for (k, e) in &plain.elites {
            let f = &logged.elites[k];
            assert_eq!((e.genome.sha256(), e.record.score.raw_score_100.to_bits()),
                       (f.genome.sha256(), f.record.score.raw_score_100.to_bits()));
        }
        assert_eq!(hist.first().map(|h| h.evals), Some(16));
        assert_eq!(hist.last().map(|h| h.evals), Some(64));
        assert!(hist.last().unwrap().best_genome_row.is_some());
        for w in hist.windows(2) {
            assert!((w[1].best_tier, w[1].best_raw_score_100) >= (w[0].best_tier, w[0].best_raw_score_100) || w[1].best_tier > w[0].best_tier);
            assert!(w[1].evals > w[0].evals && w[1].niches >= w[0].niches);
        }
    }

    #[test]
    fn edge_spec_one_ideal_angle_full_height_to_the_tip_on_both_edges() {
        // EDGE-1..5 (owner 2026-10-26): every steady Gate-S3 station of both edges measures one primary
        // half-angle in the RS-karambit-v5 [20, 25] deg band (plus the hollow/convex modulation beyond 0.4 mm), normal to
        // the edge, over the apex band and the rest of the bevel; both grinds stay on all the way to the tip.
        for i in 0..60 {
            let g = sobol_unit_genome(i, 0, "22bb8696");
            let p = g.decode();
            let blade = build_blade_profile_2d(&p);
            let (apex, ang) = measure_blade_edge_apex_and_angle(&p, &blade);
            assert!(apex.is_finite() && apex <= 0.19 + 1e-9, "Sobol {i}: apex {apex}");
            assert!((19.5..=26.0).contains(&ang), "Sobol {i}: worst half-angle {ang:.2} deg outside the band");
            let n = blade.xs.len();
            for (s_in, s_sp) in [(blade.arc_in_mm - 0.5, blade.arc_sp_mm - 0.5), (blade.arc_in_mm, blade.arc_sp_mm)] {
                let (w_in, w_sp) =
                    blade_bevel_window_weights(&p, blade.arc_in_mm, blade.arc_sp_mm, s_in, s_sp, 10.0);
                assert_eq!((w_in, w_sp), (1.0, 1.0), "Sobol {i}: a grind stops before the tip");
            }
            // the section 10 mm behind the tip is ground on both sides: thin at the edge, wedge-shaped inward
            let k = (0..n).rev().find(|&k| blade.x_tip - blade.xs[k] >= 10.0).unwrap();
            for (y_edge, sign) in [(blade.y_in[k], 1.0), (blade.y_sp[k], -1.0)] {
                let t_at = |s: f64| eval_blade_thickness_and_sdf2(&p, &blade, blade.xs[k], y_edge + sign * s).0;
                assert!(t_at(0.05) <= 0.30, "Sobol {i}: edge 10 mm from the tip is unground ({:.2} mm)", t_at(0.05));
                assert!(t_at(1.0) > t_at(0.3), "Sobol {i}: no wedge 10 mm from the tip");
            }
        }
    }

    #[test]
    fn b2_b3_i8_i9_fail_closed_zero_credit_no_dead_genes_and_tier_ordering() {
        use crate::g5_genome::G5_GENES;

        let base = G5Genome::benchmark_reference("22bb8696");
        let r0 = evaluate_tier0_analytic(&base);
        // B2: unmeasured solver-only fields in Tier 0 must receive zero credit
        assert_eq!(r0.inputs.solid_tip_sf, 0.0);
        assert_eq!(r0.inputs.dolfinx_rel_err, 1.0);
        assert!(!r0.inputs.gci_admitted);
        assert!(!r0.inputs.watertight);
        let term_s = |name: &str| -> f64 {
            r0.score
                .terms
                .iter()
                .find(|&&(k, _, _, _)| k == name)
                .map(|&(_, _, s, _)| s)
                .unwrap()
        };
        assert_eq!(term_s("solid_tip_3d"), 0.0);
        assert_eq!(term_s("plate_gci_3level"), 0.5);
        assert_eq!(term_s("cad_watertight"), 0.0);

        // I8: rib_w and shear_web_amp must affect mass and structural safety factor
        let idx_rib = G5_GENES.iter().position(|g| g.name == "rib_w").unwrap();
        let idx_web = G5_GENES
            .iter()
            .position(|g| g.name == "shear_web_amp")
            .unwrap();
        let mut g_no_rib = base.clone();
        g_no_rib.u[idx_rib] = 0.0;
        g_no_rib.u[idx_web] = 0.0;
        let mut g_full_rib = base.clone();
        g_full_rib.u[idx_rib] = 1.0;
        g_full_rib.u[idx_web] = 1.0;
        let r_no_rib = evaluate_tier0_analytic(&g_no_rib);
        let r_full_rib = evaluate_tier0_analytic(&g_full_rib);
        assert!(
            r_full_rib.inputs.mass_g > r_no_rib.inputs.mass_g,
            "rib_w + shear_web_amp must increase mass: {} vs {}",
            r_full_rib.inputs.mass_g,
            r_no_rib.inputs.mass_g
        );
        assert!(
            r_full_rib.inputs.grip_sf_min > r_no_rib.inputs.grip_sf_min,
            "rib_w + shear_web_amp must increase grip_sf_min: {} vs {}",
            r_full_rib.inputs.grip_sf_min,
            r_no_rib.inputs.grip_sf_min
        );

        // I9: a Tier-0 record must never displace a Tier-1 elite in the same niche even if Tier-0 raw_score_100 is higher
        let mut arch = CvtMapElitesArchive::new(1024);
        let mut r_tier1 = r0.clone();
        r_tier1.tier = EvalTier::Tier1SdfAndPlate;
        r_tier1.score.raw_score_100 = 60.0;
        assert!(arch.insert(base.clone(), r_tier1));
        let mut r_tier0_optimistic = r0.clone();
        r_tier0_optimistic.tier = EvalTier::Tier0Analytic;
        r_tier0_optimistic.score.raw_score_100 = 85.0;
        assert!(
            !arch.insert(base.clone(), r_tier0_optimistic),
            "Tier-0 optimistic score must not displace Tier-1 elite"
        );
        let nid = arch.niche_of(&r0.descriptors);
        assert_eq!(arch.elites[&nid].record.tier, EvalTier::Tier1SdfAndPlate);
    }
}
