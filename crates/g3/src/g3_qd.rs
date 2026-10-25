//! G3 3-Tier Multi-Fidelity Continuous Evaluation Cascade and 1024-Niche CVT-MAP-Elites Engine
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
//!   - **Tier 1 (~0.35 s)**: 3D SDF (`build_g3_sdf`) + `SandwichPlate` (`CST+DKT`) + `solve_ring_band_t6`
//!     (1.5 mm thickness-weighted distributed load band).
//!   - **Tier 2 (~1.5 s)**: Watertight Marching-Tetrahedra 3D mesh (`extract_watertight_mesh`) +
//!     topology census + fail-closed 3D `TET10` solid tip solver (`solve_g3_solid_tet10_strict`).

use crate::ergolight::{score_ergolight, ErgolightInputs, ErgolightScore};
use crate::g3_genome::{
    reproduce_iso_line_dd, sobol_unit_genome, G3Genome, G3Rng, FIXED_BORE_D_MM, FIXED_LIP_W_MM,
};
use crate::g3_sandwich::{
    joint_k, solve_g3_solid_tet10_strict, solve_ring_band_t6, SandwichPlate, SandwichSection,
    E_TI64_MPA, YIELD_TI64_MPA,
};
use crate::g3_sdf::{
    blade_bevel_window_weights, build_blade_profile_2d, build_g3_sdf, effective_blade_rho,
    eval_blade_thickness_and_sdf2, eval_grip_slice, extract_watertight_mesh, BladeProfile2D,
    VoxelGrid3D, GUARD_X_MM, RHO_TI64_G_MM3,
};
use jarambit_crescent::export;
use jarambit_physics::fine::{mesh_domain, NU};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const M50_STATIONS_MM: [f64; 3] = [9.75, 30.0, 51.5];
/// Published v1 winner `ec3i042_r01_p10_c56` f075 spin inertia about its ring center `(cx, 0)`
/// (`tests/fixtures/f075_reference.json`, #6 / B6).
pub const F075_SPIN_I_REF_G_MM2: f64 = 572_965.368_840_274_9;

/// Compute the frozen G3 decoder SHA-256 digest over the genome schema, all decoder/evaluator
/// sources, physics/crescent dependencies, and both workspace and standalone `Cargo.lock` files
/// (`I7`, `comms.md` `2026-10-25T01:14:45Z`).
/// Every `G3EvalRecord` and `CvtMapElitesArchive` records this hash and refuses to mix evaluations
/// if the decoder changes mid-run.
pub fn g3_decoder_sha256() -> String {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"G3.0\0");
    for src in [
        include_str!("g3_genome.rs"),
        include_str!("sobol_joe_kuo.rs"),
        include_str!("g3_sdf.rs"),
        include_str!("g3_sandwich.rs"),
        include_str!("ergolight.rs"),
        include_str!("g3_qd.rs"),
        include_str!("../../physics/src/fine.rs"),
        include_str!("../../physics/src/solid_tet10.rs"),
        include_str!("../../crescent/src/lib.rs"),
        include_str!("../../crescent/src/fem.rs"),
        include_str!("../../crescent/src/certification.rs"),
        include_str!("../../crescent/src/export.rs"),
        include_str!("../Cargo.lock"),
        include_str!("../../../Cargo.lock"),
    ] {
        bytes.extend_from_slice(src.as_bytes());
        bytes.push(0);
    }
    crate::sha256_bytes(&bytes)
}

/// 8-dimensional behavior descriptor vector (`G3_GENOME_SPEC.md` v1.1 §4).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct G3Descriptors {
    pub hook_drop_mm: f64,
    pub inner_turn_deg: f64,
    pub silhouette_complexity: f64,
    pub grip_roundness_h_over_t: f64,
    pub groove_articulation_mm: f64,
    pub porosity_frac: f64,
    pub surface_energy: f64,
    pub mass_g: f64,
}

impl G3Descriptors {
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
                    "mid_depth_mm", "choil_depth_mm", "station_err_mm",
                    "edge_r_min_mm", "section_corner_r_min_mm"
                ],
                "surrogate": [
                    "mass_g", "lat_rob_cert_n", "strict_lat_n",
                    "ring_bearing_cap_n", "ring_back_cap_n",
                    "twist_body_sf", "twist_tip_sf", "twist_tipzone_strict_sf",
                    "hm_twist_sf", "hm_spine_sf", "fat_sf", "grip_sf_min",
                    "swell_perimeter_mm", "swell_h_over_t", "L_g_mm", "guard_h_mm",
                    "com_to_index_mm", "spin_I_g_mm2", "am_build", "fairness",
                    "max_apex_mm", "max_half_angle_deg", "t_0p25_mm", "t_1p00_mm", "t_2p00_mm"
                ],
                "unmeasured_zero_credit": ["solid_tip_sf", "buckling_lambda"]
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
                    "mid_depth_mm", "choil_depth_mm", "station_err_mm",
                    "edge_r_min_mm", "section_corner_r_min_mm"
                ],
                "surrogate": [
                    "max_apex_mm", "max_half_angle_deg", "t_0p25_mm", "t_1p00_mm", "t_2p00_mm", "fairness"
                ],
                "unmeasured_zero_credit": ["solid_tip_sf", "buckling_lambda"]
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
                    "mid_depth_mm", "choil_depth_mm", "station_err_mm",
                    "edge_r_min_mm", "section_corner_r_min_mm"
                ],
                "surrogate": [
                    "max_apex_mm", "max_half_angle_deg", "t_0p25_mm", "t_1p00_mm", "t_2p00_mm", "fairness"
                ],
                "unmeasured_zero_credit": ["buckling_lambda"]
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct G3EvalRecord {
    pub schema: String,
    pub decoder_sha256: String,
    pub genome_sha256: String,
    pub tier: EvalTier,
    pub inputs: ErgolightInputs,
    pub score: ErgolightScore,
    pub descriptors: G3Descriptors,
}

/// Exact perimeter of an asymmetric superellipse section with top semi-height `a_top`, bottom
/// semi-height `a_bot`, half-thickness `b`, and exponent `n`.
pub fn superellipse_perim(a_top: f64, a_bot: f64, b: f64, n: f64) -> f64 {
    let b = b.max(0.5);
    let n = n.clamp(1.8, 4.0);
    let p = 2.0 / n;
    let quad_arc = |a: f64| -> f64 {
        let a = a.max(0.5);
        let steps = 64usize;
        let mut len = 0.0_f64;
        let mut prev = (a, 0.0_f64);
        for i in 1..=steps {
            let t = (i as f64 / steps as f64) * std::f64::consts::FRAC_PI_2;
            let (s, c) = t.sin_cos();
            let cur = (a * c.powf(p), b * s.powf(p));
            len += (cur.0 - prev.0).hypot(cur.1 - prev.1);
            prev = cur;
        }
        len
    };
    2.0 * (quad_arc(a_top) + quad_arc(a_bot))
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
    let rho_e = rho_g.powf(1.8);
    let hollow = rho_g < 0.95;
    let skin_t = p["skin_t"];
    let allow = YIELD_TI64_MPA * 0.912 * (skin_t - 0.1) / skin_t.max(0.2);
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
            if hollow { rho_g } else { 1.0 },
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
/// stations (`win == 1.0`, excluding the 3 mm entry/exit transition ramps and unsharpened spine stations,
/// `S5-4`, `S6-2`), taking the worst (`max`) station over each active edge and failing closed to
/// `(f64::INFINITY, f64::INFINITY)` (zero sharpness credit) if any active edge has no qualifying station.
pub fn measure_blade_edge_apex_and_angle(
    p: &BTreeMap<&'static str, f64>,
    blade: &BladeProfile2D,
) -> (f64, f64) {
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
        let (win_in, win_sp) = blade_bevel_window_weights(
            p,
            blade.arc_in_mm,
            blade.arc_sp_mm,
            s_arc_in,
            s_arc_sp,
            a_tip,
        );
        let tip_res = p["tip_reserve"].max(6.0);
        if (win_in - 1.0).abs() <= 1e-6 && a_tip > tip_res {
            let (t_in_apex, d2_in) =
                eval_blade_thickness_and_sdf2(p, blade, x, blade.y_in[i] + 1e-12);
            let (t_in_40, d2_in_40) =
                eval_blade_thickness_and_sdf2(p, blade, x, blade.y_in[i] + 0.40);
            if d2_in <= 0.0 && d2_in_40 <= 0.0 && t_in_apex > 0.0 && t_in_40 > t_in_apex {
                max_apex_in = max_apex_in.max(t_in_apex);
                let ang = ((t_in_40 - t_in_apex) / (2.0 * 0.40)).atan().to_degrees();
                max_ang_in = max_ang_in.max(ang);
                has_in = true;
            }
        }
        if (win_sp - 1.0).abs() <= 1e-6 && a_tip > tip_res {
            let (t_sp_apex, d2_sp) =
                eval_blade_thickness_and_sdf2(p, blade, x, blade.y_sp[i] - 1e-12);
            let (t_sp_40, d2_sp_40) =
                eval_blade_thickness_and_sdf2(p, blade, x, blade.y_sp[i] - 0.40);
            if d2_sp <= 0.0 && d2_sp_40 <= 0.0 && t_sp_apex > 0.0 && t_sp_40 > t_sp_apex {
                max_apex_sp = max_apex_sp.max(t_sp_apex);
                let ang = ((t_sp_40 - t_sp_apex) / (2.0 * 0.40)).atan().to_degrees();
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

/// Fast Tier-0 analytic/1D-section evaluation (~2 ms per individual).
/// Computes continuous `ErgolightInputs`, `ErgolightScore`, and `G3Descriptors` directly from the
/// decoded geometry laws so weak individuals can be weeded out before 3D voxelization.
pub fn evaluate_tier0_analytic(genome: &G3Genome) -> G3EvalRecord {
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
    let n_bx = 40usize;
    let n_by = 16usize;
    let dx_b = (blade.x_tip - blade.x_heel) / (n_bx as f64);
    let mut min_blade_lat_n = f64::INFINITY;
    let mut min_twist_body_sf = f64::INFINITY;
    let mut min_twist_tip_sf = f64::INFINITY;
    let mut min_hm_spine_sf = f64::INFINITY;
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
                let sec = SandwichSection::from_skin_and_lattice(t_xy, skin_t, rho_blade_eff, 0.0);
                col_i_yy += (sec.t3_bend / 12.0) * dy_b;
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

        // Structural section checks along the blade using true Euclidean distance to tip_xy (`B3`, `S5-1`: 4.0 mm exclusion)
        let a_tip = (x - blade.tip_xy.0).hypot(y_mid - blade.tip_xy.1);
        if a_tip >= 4.0 {
            let jk = joint_k(p["t_core"]);
            let allow = YIELD_TI64_MPA * jk;
            let z_eff = col_i_yy / (0.5 * t_max_slice).max(0.1);
            let cap_lat_n = allow * z_eff / a_tip.max(4.0);
            min_blade_lat_n = min_blade_lat_n.min(cap_lat_n);

            // Saint-Venant / Bredt torsion of the blade section at 1.82 N*m (1820 N*mm)
            let j_tors = 0.31 * h_b * t_max_slice.powi(3);
            let tau_1820 = 1820.0 * t_max_slice / j_tors.max(0.1);
            let sf_tw = (allow / (3.0_f64.sqrt() * tau_1820).max(1e-6)).min(4.0);
            if a_tip > 15.0 {
                min_twist_body_sf = min_twist_body_sf.min(sf_tw);
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
    let jk_ring = joint_k(p["web_t"]);
    let ring_bearing_cap_n =
        YIELD_TI64_MPA * jk_ring * (1.55 * ring_wall * (0.45 * FIXED_LIP_W_MM + 0.55 * p["web_t"]))
            / (1.0 + 0.12 * (r_b / ring_wall));
    let rear_wall = (ring_wall + p["od_tear"].max(-0.8) + p["od_h2"]).max(1.8);
    let ring_back_cap_n =
        YIELD_TI64_MPA * jk_ring * (1.05 * rear_wall * (0.40 * FIXED_LIP_W_MM + 0.60 * p["web_t"]))
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

    let fat_sf = if lat_rob_n > 1.0 {
        jarambit_crescent::fem::fatigue::goodman_sf(
            YIELD_TI64_MPA * 180.0 / lat_rob_n,
            p["t_core"],
        )
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
        buckling_lambda: 0.0,
        seq_ratio,
        gci_admitted: false,
        hm_twist_sf: (twist_tip_sf * (1820.0 / 16_200.0)).clamp(0.0, 2.0),
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
    let descriptors = G3Descriptors {
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
    G3EvalRecord {
        schema: genome.schema.clone(),
        decoder_sha256: g3_decoder_sha256(),
        genome_sha256: genome.sha256(),
        tier: EvalTier::Tier0Analytic,
        inputs,
        score,
        descriptors,
    }
}

/// Tier-1 evaluation: builds the full-knife 3D SDF (`build_g3_sdf`), integrates `mass_g`, `com_x`,
/// `com_y`, and `spin_i_g_mm2` strictly from the AM-printed voxels (`d_am`, after
/// `enforce_am_connectivity` fills any unported cavities solid, `B5`), measures `swell_perimeter_mm`,
/// `swell_h_over_t`, `l_g_mm`, `guard_h_mm`, and `grip_sf_min` from the 3D voxel contour (`I4`, `N8`),
/// and solves the `T6` ring submodel (`N7` bilinear homogenized thickness) and multi-RHS `SandwichPlate`
/// blade submodel (`B2`, `I6` residual gate) with fail-closed zero credit on solver failure (`B3`, `I2`).
pub fn evaluate_tier1_sdf_and_plate(genome: &G3Genome, h_vox: f64) -> G3EvalRecord {
    evaluate_tier1_with_grid(genome, h_vox).0
}

fn evaluate_tier1_with_grid(genome: &G3Genome, h_vox: f64) -> (G3EvalRecord, VoxelGrid3D) {
    let mut rec = evaluate_tier0_analytic(genome);
    let p = genome.decode();
    let grid = build_g3_sdf(genome, h_vox, true);

    // Measure voxel envelope & AM porosity across the full-knife 3D SDF grid
    let mut env_vox = 0usize;
    let mut am_vox = 0usize;
    let d_am = grid.d_am.as_deref().unwrap_or(&grid.d);
    for (&dv, &amv) in grid.d.iter().zip(d_am.iter()) {
        if dv < 0.0 {
            env_vox += 1;
        }
        if amv < 0.0 {
            am_vox += 1;
        }
    }
    rec.descriptors.porosity_frac = if env_vox > 0 {
        (1.0 - (am_vox as f64) / (env_vox as f64)).clamp(0.0, 0.85)
    } else {
        0.0
    };

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
        }
    }

    // Solve SandwichPlate on the blade domain for all plate structural terms (`B2`, `B3`, `I6`, `N1`).
    // Fail-closed: zero out all Tier-0 analytic blade estimates before running the checked plate solver.
    rec.inputs.lat_rob_cert_n = 0.0;
    rec.inputs.strict_lat_n = 0.0;
    rec.inputs.twist_body_sf = 0.0;
    rec.inputs.twist_tip_sf = 0.0;
    rec.inputs.twist_tipzone_strict_sf = 0.0;
    rec.inputs.buckling_lambda = 0.0;
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
        let secs: Vec<SandwichSection> = b_mesh
            .tris
            .iter()
            .map(|t| {
                let xm = (b_mesh.xy[t[0]].0 + b_mesh.xy[t[1]].0 + b_mesh.xy[t[2]].0) / 3.0;
                let ym = (b_mesh.xy[t[0]].1 + b_mesh.xy[t[1]].1 + b_mesh.xy[t[2]].1) / 3.0;
                let (th, _) = eval_blade_thickness_and_sdf2(&p, &blade, xm, ym);
                SandwichSection::from_skin_and_lattice(th.max(0.3), skin_t, rho_blade_eff, 0.0)
            })
            .collect();
        let s_plate = SandwichPlate::new(b_mesh.clone(), p["t_core"], E_TI64_MPA, NU)
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
                let allow_plate = YIELD_TI64_MPA * joint_k(p["t_core"]);

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
                                rec.inputs.fat_sf = jarambit_crescent::fem::fatigue::goodman_sf(
                                    YIELD_TI64_MPA * 180.0 / cap_plate,
                                    p["t_core"],
                                );
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
                            // B2 remainder: exact 1.82 N*m -> 16.2 N*m physical scaling without * 3.2 inflation
                            rec.inputs.hm_twist_sf =
                                (sf_strict * (1820.0 / 16_200.0)).clamp(0.0, 2.0);
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
    genome: &G3Genome,
    h_vox: f64,
) -> Option<crate::g3_sandwich::Ring3dTet10Summary> {
    let p = genome.decode();
    let grid = build_g3_sdf(genome, h_vox, true);
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
    crate::g3_sandwich::solve_ring_2p5d_extruded_tet10(
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
/// runs the fail-closed 3D `TET10` tip submodel (`solve_g3_solid_tet10_strict`).
/// Fail-closed (`B2`, `B3`): `solid_tip_sf` starts at `0.0` and is only populated if the strict
/// Cholesky solve (`true_rel_res <= 1e-10`) succeeds; `dolfinx_rel_err` remains `1.0` unless an
/// independent DOLFINx cross-check is provided (never substituting the linear-solver residual).
pub fn evaluate_tier2_elites(genome: &G3Genome, h_vox: f64) -> G3EvalRecord {
    let (rec1, grid) = evaluate_tier1_with_grid(genome, h_vox);
    evaluate_tier2_from_tier1(genome, rec1, &grid)
}

fn evaluate_tier2_from_tier1(
    genome: &G3Genome,
    mut rec: G3EvalRecord,
    grid: &VoxelGrid3D,
) -> G3EvalRecord {
    let p = genome.decode();
    let solid = extract_watertight_mesh(grid, true);
    let (boundary, non_manifold, dmax) = export::census(&solid.tris);
    rec.inputs.watertight = !solid.tris.is_empty() && boundary == 0 && non_manifold == 0 && dmax == 1;
    rec.inputs.solid_tip_sf = 0.0;
    rec.inputs.dolfinx_rel_err = 1.0;

    // Run fail-closed 3D TET10 tip submodel on the blade surface law (P6: full xs outline containing
    // exact tip_xy + max triangle area 0.4 mm^2 so the 2 mm tip patch has ~30 vertices and converges <= 10%).
    let blade = build_blade_profile_2d(&p);
    let mut b_outer = Vec::with_capacity(blade.xs.len() * 2);
    for i in 0..blade.xs.len() {
        b_outer.push((blade.xs[i], blade.y_in[i]));
    }
    for i in (0..blade.xs.len()).rev() {
        b_outer.push((blade.xs[i], blade.y_sp[i]));
    }
    if let Some(b_mesh) = mesh_domain(&b_outer, &[], 0.4) {
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
        if let Some(tip_res) = solve_g3_solid_tet10_strict(
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
pub fn evaluate_g3_cascade(
    genome: &G3Genome,
    tier1_gate_score: f64,
    tier2_gate_score: f64,
    h_vox: f64,
) -> G3EvalRecord {
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
    pub genome: G3Genome,
    pub record: G3EvalRecord,
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
            schema: "G3.0".to_string(),
            decoder_sha256: g3_decoder_sha256(),
            centroids,
            elites: BTreeMap::new(),
        }
    }

    /// Find the nearest CVT centroid index for an 8D descriptor vector.
    pub fn niche_of(&self, desc: &G3Descriptors) -> usize {
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
    pub fn insert(&mut self, genome: G3Genome, record: G3EvalRecord) -> bool {
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
    pub fn sample_offspring(&mut self, rng: &mut G3Rng) -> Option<(G3Genome, usize)> {
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
            let pick_curious = |rng: &mut G3Rng, nids: &[usize], elites: &BTreeMap<usize, NicheElite>| -> usize {
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
        if self.schema != "G3.0" {
            return Err(format!("archive schema mismatch: '{}'", self.schema));
        }
        let expected_dec = g3_decoder_sha256();
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

    /// Load and verify a persisted `{genomes_g3.csv, archive.jsonl}` run directory (`I5`, `I7`, `S8-2`, `S8-3`):
    /// - Parses every CSV row via `G3Genome::from_csv_row_checked(row, expected_commit)` (`I5`),
    /// - Verifies 1-to-1 join with `archive.jsonl` on `design_id` and `genome_sha256`,
    /// - Verifies `decoder_sha256 == g3_decoder_sha256()` (`I7`),
    /// - Verifies CVT `niche_id == niche_of(&descriptors)`,
    /// - Re-scores every row's `inputs` via `score_ergolight_map` and checks `score_100`, `base`, and `E` (`S8-2`).
    pub fn load_and_verify_persisted_dir(
        dir: &str,
        expected_commit: &str,
    ) -> Result<usize, String> {
        let csv_raw = std::fs::read_to_string(format!("{dir}/genomes_g3.csv"))
            .map_err(|e| format!("read genomes_g3.csv: {e}"))?;
        let jsonl_raw = std::fs::read_to_string(format!("{dir}/archive.jsonl"))
            .map_err(|e| format!("read archive.jsonl: {e}"))?;
        let mut csv_lines = csv_raw.lines();
        let _hdr = csv_lines
            .next()
            .ok_or_else(|| "empty genomes_g3.csv".to_string())?;
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
        let expected_dec = g3_decoder_sha256();
        let ref_arch = Self::new(1024);
        for (idx, (&c_line, &j_line)) in csv_rows.iter().zip(jsonl_rows.iter()).enumerate() {
            let (design_id, genome_part) = c_line
                .split_once(',')
                .ok_or_else(|| format!("malformed csv row {idx}"))?;
            crate::g3_namespace::validate_g3_design_id(design_id)?;
            let genome = G3Genome::from_csv_row_checked(genome_part, expected_commit)?;
            let doc: serde_json::Value = serde_json::from_str(j_line)
                .map_err(|e| format!("parse archive.jsonl row {idx}: {e}"))?;
            if doc["design_id"].as_str() != Some(design_id) {
                return Err(format!("row {idx} design_id mismatch"));
            }
            if doc["schema"].as_str() != Some("G3.0") {
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
            let desc: G3Descriptors = serde_json::from_value(doc["descriptors"].clone())
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
pub fn run_g3_cvt_map_elites(
    n_init_sobol: usize,
    n_evals: usize,
    seed: u64,
    tier1_gate_score: f64,
    tier2_gate_score: f64,
    h_vox: f64,
    decoder_commit: &str,
) -> CvtMapElitesArchive {
    let mut archive = CvtMapElitesArchive::new(1024);
    let mut rng = G3Rng(seed.max(1));
    let mixed_seed = splitmix64(seed);
    let scramble_seed = (mixed_seed as usize).max(1);

    let init_count = n_init_sobol.min(n_evals).max(1);
    for i in 0..init_count {
        let g = sobol_unit_genome(i, scramble_seed, decoder_commit);
        let rec = evaluate_g3_cascade(&g, tier1_gate_score, tier2_gate_score, h_vox);
        archive.insert(g, rec);
    }

    for _ in init_count..n_evals {
        let Some((child, parent_nid)) = archive.sample_offspring(&mut rng) else {
            break;
        };
        let rec = evaluate_g3_cascade(&child, tier1_gate_score, tier2_gate_score, h_vox);
        let improved = archive.insert(child, rec);
        archive.update_curiosity(parent_nid, improved);
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
            assert!(rec.inputs.max_half_angle_deg <= 24.5 + 1e-9);
            assert!(rec.inputs.point_monotonic);
        }

        // 2. Pure Sobol-seeded CVT-MAP-Elites evolution improves best score and QD score
        let arch_init = run_g3_cvt_map_elites(24, 24, 7, 999.0, 999.0, 1.0, "22bb8696");
        let arch_evol = run_g3_cvt_map_elites(24, 160, 7, 999.0, 999.0, 1.0, "22bb8696");
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
        let arch = run_g3_cvt_map_elites(16, 48, 19, 999.0, 999.0, 1.2, "22bb8696");
        let elite = arch.best_elite().expect("best elite");
        let r1 = evaluate_tier1_sdf_and_plate(&elite.genome, 1.2);
        assert_eq!(r1.tier, EvalTier::Tier1SdfAndPlate);
        assert!(r1.score.score_100 > 20.0);
        assert!(r1.inputs.ring_bearing_cap_n > 200.0);
        assert!(r1.inputs.ring_back_cap_n > 100.0);
    }

    #[test]
    fn b2_b3_i8_i9_fail_closed_zero_credit_no_dead_genes_and_tier_ordering() {
        use crate::g3_genome::G3_GENES;

        let base = G3Genome::benchmark_reference("22bb8696");
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
        let idx_rib = G3_GENES.iter().position(|g| g.name == "rib_w").unwrap();
        let idx_web = G3_GENES
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
