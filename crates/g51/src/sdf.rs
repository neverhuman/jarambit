//! The G5.1 full-knife SDF builder: G5.0's production builder (`jarambit_g5::g5_sdf`, the `include_blade` path,
//! copied verbatim) with one change, in step 6. Surface texture stays off the ground bevels by construction: on each
//! sharpened edge, texture fades to zero across the bevel plus the RS-karambit-v5 finishing allowance, over a 1 mm
//! taper (`bevel_texture_weight`). Everywhere else the field is G5.0's, voxel for voxel (`tests`).

use jarambit_g5::g5_genome::{G5Genome, FIXED_BORE_D_MM, FIXED_LIP_W_MM};
use jarambit_g5::g5_sdf::{
    build_blade_profile_2d, blade_bevel_window_weights, edt_out, effective_blade_rho, enforce_am_connectivity,
    eval_blade_thickness_and_sdf2, eval_grip_slice, eval_guard_lobe_y, gaussian_filter_3d, morph_close_variable_2d_z,
    morph_open_variable_2d_z, sdf_mask, tpms_val_and_grad, AmConnectivityStats, BladeProfile2D, VoxelGrid3D,
    GUARD_X_MM, HANDZONE_SDF_BLUR_SIGMA_MM,
};
use rayon::prelude::*;
use std::collections::BTreeMap;

#[inline]
fn bump(t: f64) -> f64 {
    let t = t.abs().clamp(0.0, 1.0);
    let u = 1.0 - t * t;
    u * u
}

#[inline]
fn smoothstep(e0: f64, e1: f64, x: f64) -> f64 {
    let t = ((x - e0) / (e1 - e0).max(1e-9)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

#[inline]
fn rint(a: f64, b: f64, r: f64) -> f64 {
    let ar = a + r;
    let br = b + r;
    ar.max(0.0).hypot(br.max(0.0)) + ar.max(br).min(0.0) - r
}

/// Exact Fritsch-Carlson monotone cubic Hermite interpolation (`scipy.interpolate.PchipInterpolator` parity).
fn interp_pchip(xs: &[f64], ys: &[f64], x: f64) -> f64 {
    let n = xs.len();
    debug_assert_eq!(n, ys.len());
    if n == 0 {
        return 0.0;
    }
    if n == 1 || x <= xs[0] {
        return ys[0];
    }
    if x >= xs[n - 1] {
        return ys[n - 1];
    }
    if n == 2 {
        let t = (x - xs[0]) / (xs[1] - xs[0]).max(1e-12);
        return ys[0] + t * (ys[1] - ys[0]);
    }
    let mut h = [0.0_f64; 8];
    let mut m = [0.0_f64; 8];
    for k in 0..(n - 1) {
        h[k] = (xs[k + 1] - xs[k]).max(1e-12);
        m[k] = (ys[k + 1] - ys[k]) / h[k];
    }
    let edge_case = |h0: f64, h1: f64, m0: f64, m1: f64| -> f64 {
        let mut d = ((2.0 * h0 + h1) * m0 - h0 * m1) / (h0 + h1);
        if d * m0 <= 0.0 {
            d = 0.0;
        } else if m0 * m1 <= 0.0 && d.abs() > 3.0 * m0.abs() {
            d = 3.0 * m0;
        }
        d
    };
    let mut d = [0.0_f64; 8];
    d[0] = edge_case(h[0], h[1], m[0], m[1]);
    d[n - 1] = edge_case(h[n - 2], h[n - 3], m[n - 2], m[n - 3]);
    for k in 1..(n - 1) {
        if m[k - 1] * m[k] <= 0.0 {
            d[k] = 0.0;
        } else {
            let w1 = 2.0 * h[k] + h[k - 1];
            let w2 = h[k] + 2.0 * h[k - 1];
            d[k] = (w1 + w2) / (w1 / m[k - 1] + w2 / m[k]);
        }
    }
    let mut k = 0usize;
    while k + 1 < n && x > xs[k + 1] {
        k += 1;
    }
    let hk = h[k];
    let t = ((x - xs[k]) / hk).clamp(0.0, 1.0);
    let omt = 1.0 - t;
    let h00 = (1.0 + 2.0 * t) * omt * omt;
    let h10 = t * omt * omt;
    let h01 = t * t * (3.0 - 2.0 * t);
    let h11 = t * t * (t - 1.0);
    h00 * ys[k] + h10 * hk * d[k] + h01 * ys[k + 1] + h11 * hk * d[k + 1]
}

/// Distance (mm) from `(x, y)` to the edge polyline `(xs, ys)`, searching the segments within 12 mm of sample
/// `i0` in x (`EDGE-2`: bevels are ground normal to the edge, not along y). Returns 12.0 when no segment is that
/// close, which is past every primary grind (`(t_core + 1.8) / (2 tan 17.5 deg) < 12`).
fn edge_normal_distance(xs: &[f64], ys: &[f64], i0: usize, x: f64, y: f64) -> f64 {
    let dx = ((xs[xs.len() - 1] - xs[0]) / (xs.len() - 1) as f64).abs().max(1e-6);
    let w = (12.0 / dx).ceil() as usize + 1;
    let lo = i0.saturating_sub(w);
    let hi = (i0 + w + 1).min(xs.len() - 1);
    let mut best = 12.0_f64;
    for k in lo..hi {
        let (ax, ay, bx, by) = (xs[k], ys[k], xs[k + 1], ys[k + 1]);
        let (ex, ey) = (bx - ax, by - ay);
        let l2 = (ex * ex + ey * ey).max(1e-18);
        let t = (((x - ax) * ex + (y - ay) * ey) / l2).clamp(0.0, 1.0);
        best = best.min((x - ax - t * ex).hypot(y - ay - t * ey));
    }
    best
}

/// Evaluate the G5 multi-modal surface texture displacement `tx in [-1, 1]`
/// using `w_knurl` and `w_ridge` (G5.0 retires the Turing, Voronoi and 32-weight CPPN patterns).
#[inline]
fn eval_surface_texture(p: &BTreeMap<&'static str, f64>, sx: f64, sy: f64, _l_g: f64) -> f64 {
    let pitch = p["tex_pitch"].max(1.2);
    let ang = p["tex_angle"].to_radians();
    let (sa, ca) = ang.sin_cos();
    let u = (sx * ca + sy * sa) / pitch;
    let v = (-sx * sa + sy * ca) / pitch;
    // Zero-mean patterns, so the texture moves surface material without biasing the mass: the knurl's mean
    // 2 (2/pi)^2 - 1 is subtracted, and the ridge is a cosine
    let knurl_mean = 8.0 / (std::f64::consts::PI * std::f64::consts::PI) - 1.0;
    let knurl = (std::f64::consts::PI * (u + v)).sin().abs()
        * (std::f64::consts::PI * (u - v)).sin().abs()
        * 2.0
        - 1.0
        - knurl_mean;
    let ridge = (2.0 * std::f64::consts::PI * u).cos();
    let wk = p["w_knurl"].max(0.0);
    let wr = p["w_ridge"].max(0.0);
    let w_tot = (wk + wr).max(1e-6);
    ((wk * knurl + wr * ridge) / w_tot).clamp(-1.0, 1.0)
}

/// Width (mm) over which the texture weight rises from 0 to 1 beyond a bevel and its finishing allowance, so the
/// mask leaves no texture step (a step would be a notch of its own).
pub const BEVEL_TEXTURE_TAPER_MM: f64 = 1.0;

/// The blade thickness law of G5.0 (`eval_blade_thickness_and_sdf2`, copied) at one planform point, with the bevel
/// quantities it otherwise discards. `[0]` is the inner edge, `[1]` the spine edge.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BevelDetail {
    /// G5.0's thickness and signed planform distance (`tests` check both bit for bit)
    pub t_final: f64,
    pub d2: f64,
    /// Medial thickness the bevels grind down to
    pub t_medial: f64,
    /// Full-grind thickness of each bevel before its window blend, and the sharpening-window weight
    pub t_bevel_raw: [f64; 2],
    pub window: [f64; 2],
    /// Thickness gained per mm along the edge normal (`2 tan(angle) curv`)
    pub slope: [f64; 2],
    /// Signed in-plane distance inside each edge (negative outside the planform)
    pub d_edge: [f64; 2],
}

/// `BevelDetail` at `(x, y)`, or `None` outside the blade's x range (where G5.0 returns `(0.0, 20.0)`).
pub fn blade_bevel_detail(
    p: &BTreeMap<&'static str, f64>,
    prof: &BladeProfile2D,
    x: f64,
    y: f64,
) -> Option<BevelDetail> {
    if x < prof.x_heel || x > prof.x_tip {
        return None;
    }
    let u = ((x - prof.x_heel) / (prof.x_tip - prof.x_heel).max(1e-9)).clamp(0.0, 1.0);
    let idx_f = u * ((prof.xs.len() - 1) as f64);
    let i0 = (idx_f.floor() as usize).min(prof.xs.len() - 2);
    let frac = idx_f - (i0 as f64);
    let yi = prof.y_in[i0] * (1.0 - frac) + prof.y_in[i0 + 1] * frac;
    let ys = prof.y_sp[i0] * (1.0 - frac) + prof.y_sp[i0 + 1] * frac;
    let d_in = y - yi;
    let d_sp = ys - y;
    let d2 = -(d_in.min(d_sp));

    let dx_tip = x - prof.tip_xy.0;
    let dy_tip = y - prof.tip_xy.1;
    let proj_bisector = (dx_tip * prof.tip_inward_dir.0 + dy_tip * prof.tip_inward_dir.1).max(0.0);
    let a_tip = proj_bisector.min((prof.x_tip - x).hypot(dy_tip));
    let tip_reserve = p["tip_reserve"].max(6.0);
    let t_core = p["t_core"];
    let t_tip = p["t_tip"].min(t_core);
    let taper_u = ((u - p["taper_x0"]) / (1.0 - p["taper_x0"]).max(0.1)).clamp(0.0, 1.0);
    let t_body = t_core - (t_core - t_tip) * taper_u;
    let t_spine_cap = if a_tip <= tip_reserve {
        interp_pchip(
            &[0.0, 0.25, 1.0, 2.0, tip_reserve],
            &[
                p["edge_t_in"].min(0.16),
                p["t_0p25_target"].min(0.28),
                p["t_1p00_target"].min(0.56),
                p["t_2p00_target"].min(0.95),
                t_body.max(1.2),
            ],
            a_tip,
        )
    } else {
        t_body
    };
    let h_span = (ys - yi).max(1.0);
    let y_mid = 0.5 * (yi + ys);
    let dy_mid = (y - y_mid).abs();
    let fuller_win = smoothstep(p["fuller_s0"], p["fuller_s0"] + 0.06, u)
        * (1.0 - smoothstep(p["fuller_s1"] - 0.06, p["fuller_s1"], u))
        * smoothstep(tip_reserve, tip_reserve + 4.0, a_tip);
    let fuller_cut = fuller_win
        * p["fuller_depth"]
        * bump(dy_mid / (0.5 * p["fuller_width"].min(0.45 * h_span)).max(1.0))
        * (1.0 + 0.35 * p["fuller_corrug_amp"] * (2.0 * std::f64::consts::PI * (x - prof.x_heel) / 9.0).cos());
    let ridge_win = smoothstep(p["ridge_s0"], p["ridge_s0"] + 0.06, u)
        * (1.0 - smoothstep(p["ridge_s1"] - 0.06, p["ridge_s1"], u))
        * smoothstep(tip_reserve, tip_reserve + 4.0, a_tip);
    let ridge_add = ridge_win * p["ridge_amp"] * bump((y - (y_mid + 0.18 * h_span)) / p["ridge_w"].max(1.5));
    let t_medial = (t_spine_cap - fuller_cut + ridge_add).clamp(0.16, t_spine_cap.max(t_core + 1.8));

    let s_arc_in = prof.cum_arc_in[i0] * (1.0 - frac) + prof.cum_arc_in[i0 + 1] * frac;
    let s_arc_sp = prof.cum_arc_sp[i0] * (1.0 - frac) + prof.cum_arc_sp[i0 + 1] * frac;
    let dn_in = edge_normal_distance(&prof.xs, &prof.y_in, i0, x, y);
    let dn_sp = edge_normal_distance(&prof.xs, &prof.y_sp, i0, x, y);
    let (win_in, win_sp) = blade_bevel_window_weights(p, prof.arc_in_mm, prof.arc_sp_mm, s_arc_in, s_arc_sp, a_tip);
    let ed = &jarambit_g5::requirements::rs_v5().edge;
    let tan_in = p["angle_in"].clamp(ed.per_side_angle_min_deg, ed.per_side_angle_max_deg).to_radians().tan();
    let tan_sp = p["angle_sp"].clamp(ed.per_side_angle_min_deg, ed.per_side_angle_max_deg).to_radians().tan();
    let curv_in = 1.0 + 0.18 * p["grind_curv_in"] * ((dn_in - 0.4).max(0.0) / 6.0).min(1.0);
    let curv_sp = 1.0 + 0.18 * p["grind_curv_sp"] * ((dn_sp - 0.4).max(0.0) / 6.0).min(1.0);
    let t_bevel_in_raw = p["edge_t_in"].min(0.19) + 2.0 * dn_in * tan_in * curv_in;
    let t_bevel_sp_raw = p["edge_t_sp"].min(0.19) + 2.0 * dn_sp * tan_sp * curv_sp;
    let t_bevel_in = win_in * t_bevel_in_raw + (1.0 - win_in) * t_medial;
    let t_bevel_sp = win_sp * t_bevel_sp_raw + (1.0 - win_sp) * t_medial;
    let t_final = if d2 > 0.0 { 0.0 } else { t_medial.min(t_bevel_in).min(t_bevel_sp).min(t_spine_cap) };
    Some(BevelDetail {
        t_final,
        d2,
        t_medial,
        t_bevel_raw: [t_bevel_in_raw, t_bevel_sp_raw],
        window: [win_in, win_sp],
        slope: [2.0 * tan_in * curv_in, 2.0 * tan_sp * curv_sp],
        d_edge: [d_in, d_sp],
    })
}

/// Texture weight in [0, 1] at planform point `(x, y)` of the blade zone: 0 on a sharpened edge's bevel and its
/// finishing allowance (RS-karambit-v5 `[edge] finishing_allowance_mm`, in plane past the bevel's inner boundary),
/// rising to 1 over `BEVEL_TEXTURE_TAPER_MM`; exactly 1 away from every bevel. Just outside the planform the nearer
/// edge decides, because those voxels shape the edge surface too.
pub fn bevel_texture_weight(p: &BTreeMap<&'static str, f64>, prof: &BladeProfile2D, x: f64, y: f64) -> f64 {
    let Some(b) = blade_bevel_detail(p, prof, x, y) else {
        return 1.0;
    };
    if b.d2 > 0.0 {
        let e = if b.d_edge[0] < b.d_edge[1] { 0 } else { 1 };
        return if b.window[e] > 0.0 { 0.0 } else { 1.0 };
    }
    let allowance = jarambit_g5::requirements::rs_v5().edge.finishing_allowance_mm;
    let mut w = 1.0;
    for e in 0..2 {
        if b.window[e] <= 0.0 {
            continue;
        }
        let past = ((b.t_bevel_raw[e] - b.t_medial) / b.slope[e].max(1e-9)).max(0.0);
        w *= smoothstep(allowance, allowance + BEVEL_TEXTURE_TAPER_MM, past);
    }
    w
}

/// Build the complete G5.1 3D SDF (`VoxelGrid3D`) for a `G5Genome` at voxel spacing `h` (mm),
/// voxelizing the entire one-piece knife (ring + grip + guard lobe + curved claw blade).
pub fn build_g51_sdf(genome: &G5Genome, h: f64, with_am: bool) -> VoxelGrid3D {
    build_g51_sdf_from_params_with_origin_shift(&genome.decode(), h, with_am, 0.0)
}

/// G5.0's `build_g5_sdf_from_params_with_origin_shift` on its production path (`include_blade == true`; the
/// hand-zone parity path of the `f075` tests is not part of G5.1), with the bevel-masked texture of step 6.
pub fn build_g51_sdf_from_params_with_origin_shift(
    p: &BTreeMap<&'static str, f64>,
    h: f64,
    with_am: bool,
    origin_shift_mm: f64,
) -> VoxelGrid3D {
    let include_blade = true;
    let cx = p["cx"];
    let r_b = FIXED_BORE_D_MM * 0.5;
    let r_od = p["R_od"];
    let blade_prof = build_blade_profile_2d(p);

    let margin = p["neck_r"].max(p["guard_r"]).max(3.2) + 1.2;
    let x_lo = cx - r_od - 1.0 - p["od_tear"].max(0.0) - p["od_h2"].abs() - margin + origin_shift_mm;
    // N2: On the production path (`include_blade == true`), stop the hand-zone sub-grid at x = 93.5 mm
    // (blending the guard lobe into the evolved G5 blade over x in [x_heel, 90.0]) so no `f075` seam
    // slab fin ever protrudes beyond x > 94.0. On the hand-zone parity path (`include_blade == false`),
    // run to `X_GRID_END_MM = 101.6` with the `f075` seam slab and land perforation (`N3c`).
    let x_hz_end = if include_blade { 93.5_f64 } else { 101.6_f64 };
    let x_hi = if include_blade {
        blade_prof.x_tip + 3.5
    } else {
        x_hz_end
    };
    let min_blade_y = if include_blade {
        blade_prof
            .y_in
            .iter()
            .copied()
            .fold(0.0_f64, f64::min)
            - 3.0
    } else {
        0.0
    };
    let max_blade_y = if include_blade {
        blade_prof
            .y_sp
            .iter()
            .copied()
            .fold(0.0_f64, f64::max)
            + 3.0
    } else {
        0.0
    };
    let y_min = ((-r_od - p["od_h2"].abs())
        .min(p["yb_g"] - p["h_g"])
        .min(p["yb0"])
        - 0.5
        - margin)
        .min(min_blade_y)
        + origin_shift_mm;
    let y_max = ((r_od + p["od_h2"].abs() + p["od_tear"].max(0.0))
        .max(p["yt_sw"])
        .max(p["yt0"])
        + 0.5
        + margin)
        .max(max_blade_y);
    let z_max = p["T_sw"]
        .max(p["T0"])
        .max(p["T_g"])
        .max(p["t_core"] + p["ridge_amp"])
        .max(p["web_t"] + p["thumbpad"])
        .max(FIXED_LIP_W_MM)
        * 0.5
        + 0.5
        + margin;

    let make_axis = |lo: f64, hi: f64| -> Vec<f64> {
        let n = (((hi - lo) / h).floor() as usize) + 1;
        (0..n).map(|i| lo + (i as f64) * h).collect()
    };
    let xs = make_axis(x_lo, x_hi);
    let ys = make_axis(y_min, y_max);
    let zs = make_axis(-z_max + origin_shift_mm, z_max);
    let (nx, ny, nz) = (xs.len(), ys.len(), zs.len());
    let nx_hz = xs
        .iter()
        .position(|&x| x > x_hz_end)
        .unwrap_or(nx)
        .max(1);
    let total_hz = nx_hz * ny * nz;
    let total = nx * ny * nz;
    let cl = 3.0_f32;
    let mut da_hz = vec![cl; total_hz];

    let tj = p["junction_deg"].to_radians();
    let cot = 1.0 / p["taper_deg"].to_radians().tan();
    let (sa, _ca) = p["taper_deg"].to_radians().sin_cos();
    let r_head = r_b + p["lip_d"];

    let mut r_grid = vec![0.0_f64; nx * ny];
    let mut th_grid = vec![0.0_f64; nx * ny];
    let mut wt_grid = vec![0.0_f64; nx * ny];
    let mut hw_grid = vec![0.0_f64; nx * ny];

    // 1. Ring primitive (within 0..nx_hz)
    for (i, &x) in xs.iter().enumerate() {
        let dx = x - cx;
        for (j, &y) in ys.iter().enumerate() {
            let r = dx.hypot(y);
            let th = y.atan2(dx);
            let ramp = if th.abs() < tj {
                (std::f64::consts::PI * th / (2.0 * tj)).cos().powi(2)
            } else {
                0.0
            };
            let hw = 0.5 * FIXED_LIP_W_MM + 0.5 * (p["T0"] - FIXED_LIP_W_MM) * ramp;
            let tp_ang = th - 90.0_f64.to_radians();
            let tp = if tp_ang.abs() < 50.0_f64.to_radians() {
                p["thumbpad"]
                    * (std::f64::consts::PI * tp_ang / 100.0_f64.to_radians())
                        .cos()
                        .powi(2)
            } else {
                0.0
            };
            let wt = p["web_t"] + tp + (p["T0"] - p["web_t"]) * ramp;
            let r_o = (r_od + p["od_tear"] * 0.5 * (1.0 + th.cos()) + p["od_h2"] * (2.0 * th).cos())
                .max(r_head + 0.8);
            r_grid[i * ny + j] = r;
            th_grid[i * ny + j] = th;
            wt_grid[i * ny + j] = wt;
            hw_grid[i * ny + j] = hw;

            if i < nx_hz && r < r_o + 3.0 && r >= r_b - 6.0 {
                let base = (i * ny + j) * nz;
                for (k, &z) in zs.iter().enumerate() {
                    let az = z.abs();
                    let head = (az - hw)
                        .max((r - r_head - (hw - az) * cot) * sa)
                        .max((r_b - 3.0) - r)
                        .max(r - r_o);
                    let web = (az - 0.5 * wt).max(r - r_o).max((r_b - 3.0) - r);
                    da_hz[base + k] = (head.min(web) as f32).clamp(-cl, cl);
                }
            }
        }
    }

    // 2. Grip sweep primitive + guard transition (`Rule 6`) within 0..nx_hz
    let s_min_grip = -p.get("neck_in").copied().unwrap_or(5.0);
    for i in 0..nx_hz {
        let x = xs[i];
        let gc = eval_grip_slice(p, x);
        if gc.s > s_min_grip && gc.s <= p["L_g"] + 8.0 {
            let a_t = (gc.yt - gc.ym).max(0.3);
            let a_b = (gc.ym - gc.yb).max(0.3);
            let b = 0.5 * gc.t_sec;
            let n_i = gc.n_exp;
            for (j, &y) in ys.iter().enumerate() {
                let aa = if y >= gc.ym { a_t } else { a_b };
                let u = (y - gc.ym).abs() / aa;
                if u >= 1.6 {
                    continue;
                }
                let base = (i * ny + j) * nz;
                for (k, &z) in zs.iter().enumerate() {
                    let v = z.abs() / b;
                    let f_val = u.powf(n_i) + v.powf(n_i);
                    let gy = n_i * u.max(1e-6).powf(n_i - 1.0) / aa;
                    let gz = n_i * v.max(1e-6).powf(n_i - 1.0) / b;
                    let dg = ((f_val - 1.0) / gy.hypot(gz).max(1e-6)) as f32;
                    da_hz[base + k] = da_hz[base + k].min(dg.clamp(-cl, cl));
                }
            }
        }
        if include_blade {
            // Production path (`N2` + `S5-5`): blend the guard lobe outline AND thickness smoothly
            // into the evolved G5 blade's local profile `[yi_b, ys_b]` and local thickness `t_b(x, y)`
            // over `x in [x_heel, 90.0]`, continuing with the exact blade section to `nx_hz` (`x = 93.5`)
            // so there is zero plunge-line step at `x = 90.0` even when bevels or fullers reach `x < 90.0`.
            if x >= GUARD_X_MM - 1.0 {
                let (yb_guard, yt_guard) = eval_guard_lobe_y(p, x);
                let w_blade = smoothstep(blade_prof.x_heel, 90.0, x);
                let u_b = ((x.max(blade_prof.x_heel) - blade_prof.x_heel)
                    / (blade_prof.x_tip - blade_prof.x_heel).max(1e-9))
                .clamp(0.0, 1.0);
                let idx_f = u_b * ((blade_prof.xs.len() - 1) as f64);
                let i0 = (idx_f.floor() as usize).min(blade_prof.xs.len() - 2);
                let frac = idx_f - (i0 as f64);
                let yi_b = blade_prof.y_in[i0] * (1.0 - frac) + blade_prof.y_in[i0 + 1] * frac;
                let ys_b = blade_prof.y_sp[i0] * (1.0 - frac) + blade_prof.y_sp[i0 + 1] * frac;
                let yb_eff = (1.0 - w_blade) * yb_guard + w_blade * yi_b;
                let yt_eff = (1.0 - w_blade) * yt_guard + w_blade * ys_b;
                for (j, &y) in ys.iter().enumerate() {
                    let d2_slab = (yb_eff - y).max(y - yt_eff);
                    if d2_slab > 3.0 {
                        continue;
                    }
                    let y_b_probe = y.clamp(yi_b + 1e-3, ys_b - 1e-3);
                    let (t_b_local, _) = eval_blade_thickness_and_sdf2(
                        p,
                        &blade_prof,
                        x.max(blade_prof.x_heel),
                        y_b_probe,
                    );
                    let base = (i * ny + j) * nz;
                    let t_eff = (1.0 - w_blade) * p["t_guard"] + w_blade * t_b_local.max(0.16);
                    if t_eff >= 0.8 {
                        let half_t = 0.5 * t_eff;
                        for (k, &z) in zs.iter().enumerate() {
                            let s_guard = d2_slab.max(z.abs() - half_t) as f32;
                            da_hz[base + k] = da_hz[base + k].min(s_guard.clamp(-cl, cl));
                        }
                    }
                }
            }
        }
    }

    let mut mask_hz: Vec<bool> = da_hz.iter().map(|&v| v < 0.0).collect();

    // 3. Variable-radius closing & opening on the hand-zone sub-grid (0..nx_hz) (`Rule 1`)
    let mut r_c2 = vec![0.0_f32; nx_hz * ny];
    let mut r_o2 = vec![0.0_f32; nx_hz * ny];
    let neck_ang_max = (p["junction_deg"] + 30.0).to_radians();
    for i in 0..nx_hz {
        let x = xs[i];
        let seam = if include_blade {
            1.0 - smoothstep(87.0, 92.0, x)
        } else {
            1.0 - smoothstep(94.0, 97.5, x)
        };
        let gg = (-((x - (GUARD_X_MM + 3.0)) / 7.0).powi(2)).exp();
        for (j, &y) in ys.iter().enumerate() {
            let r = r_grid[i * ny + j];
            let th = th_grid[i * ny + j];
            let wt = wt_grid[i * ny + j];
            let is_neck =
                r < r_od + 6.0 && r > r_b + p["lip_d"] && th.abs() < neck_ang_max;
            let rc_base = if is_neck {
                p["neck_r"]
            } else if r < r_od + 1.0 {
                1.2
            } else {
                1.5
            };
            r_c2[i * ny + j] = (rc_base.max(p["guard_r"] * gg) * seam) as f32;

            let mut ro_base = if r < r_od + 1.0 {
                0.75 * wt * 0.5
            } else if x > GUARD_X_MM + 1.0 {
                // RS-karambit-v5 [ergonomics.edges]: >= 3 mm on sections 6.5-25 mm thick, >= 1.5 mm below; the
                // opening radius stays under half the guard thickness so the guard survives the opening
                let ed = &jarambit_g5::requirements::rs_v5().ergonomics.edges;
                let t_g = p["t_guard"];
                let r_req = if t_g >= ed.thick_section_min_mm {
                    ed.radius_min_thick_section_mm
                } else {
                    ed.radius_min_mm
                };
                r_req.max(0.4 * t_g).min(0.49 * t_g)
            } else {
                p["edge_r"]
            };
            if !include_blade && x > 82.0 && y.abs() < 7.0 {
                ro_base = 1.0; // `hzm.py::hole_proximity`
            }
            // `EDGE-1`: the guard edge-rounding opening fades out with the guard-to-blade blend (full at the heel,
            // none from x = 90), so a 2.2-3.0 mm opening never erases the thin full-height bevel at the plunge
            let blade_fade = if include_blade { 1.0 - smoothstep(GUARD_X_MM + 4.0, 90.0, x) } else { 1.0 };
            r_o2[i * ny + j] = (ro_base * seam * blade_fade) as f32;
        }
    }

    mask_hz = morph_close_variable_2d_z(&mask_hz, &r_c2, nx_hz, ny, nz, h);
    mask_hz = morph_open_variable_2d_z(&mask_hz, &r_o2, nx_hz, ny, nz, h);

    // 4. Smooth hand-over between analytic field `da_hz` and morphological `de_hz`
    let de_hz = sdf_mask(&mask_hz, nx_hz, ny, nz, h);
    let mod_field: Vec<f32> = mask_hz
        .iter()
        .zip(da_hz.iter())
        .map(|(&m, &a)| if m != (a < 0.0) { 1.0 } else { 0.0 })
        .collect();
    let mod_blur = gaussian_filter_3d(&mod_field, nx_hz, ny, nz, 0.6 / h);
    let mut d_hz: Vec<f32> = da_hz
        .iter()
        .zip(de_hz.iter().zip(mod_blur.iter()))
        .map(|(&a, (&e, &mb))| {
            let wa = (1.0 - 3.0 * mb).clamp(0.0, 1.0);
            wa * a + (1.0 - wa) * e
        })
        .collect();
    d_hz = gaussian_filter_3d(&d_hz, nx_hz, ny, nz, HANDZONE_SDF_BLUR_SIGMA_MM / h);

    // Assemble full-knife 3D SDF `d` (hand zone + full 3D blade smoothly blended across `x in [86.0, 92.0]`, `S5-5`)
    let mut d = vec![cl; total];
    for i in 0..nx_hz {
        let slice_len = ny * nz;
        d[i * slice_len..(i + 1) * slice_len]
            .copy_from_slice(&d_hz[i * slice_len..(i + 1) * slice_len]);
    }
    if include_blade {
        d.par_chunks_mut(ny * nz)
            .enumerate()
            .for_each(|(i, slice)| {
                let x = xs[i];
                if x < 86.0 || x > blade_prof.x_tip {
                    return;
                }
                let w_seam = smoothstep(86.0, 92.0, x) as f32;
                let w_union = smoothstep(86.0, 87.5, x) as f32;
                for (j, &y) in ys.iter().enumerate() {
                    let (t_b, d2_b) = eval_blade_thickness_and_sdf2(p, &blade_prof, x, y);
                    let base = j * nz;
                    let half_t = 0.5 * t_b;
                    for (k, &z) in zs.iter().enumerate() {
                        let db = if t_b <= 0.0 {
                            d2_b.max(0.5) as f32
                        } else {
                            d2_b.max(z.abs() - half_t) as f32
                        }
                        .clamp(-cl, cl);
                        if x < 92.0 {
                            // `EDGE-1`: union with the blade, phased in over x 86-87.5, so the hand-zone morphology
                            // can never erase the thin full-height bevel through the seam (the linear blend alone left
                            // 3 mm voids) and the union itself adds no step where it starts
                            let blend = (1.0 - w_seam) * slice[base + k] + w_seam * db;
                            slice[base + k] = blend.min(db + (1.0 - w_union) * cl);
                        } else {
                            slice[base + k] = db;
                        }
                    }
                }
            });
    }

    // 5. Crowned bore cut by rounded intersection (`Rule 1` / Spec §2) and boundary cap BEFORE texture (`N5`)
    let crown = p["crown"];
    let bore_round = p["bore_round"];
    for i in 0..nx_hz {
        for j in 0..ny {
            let r = r_grid[i * ny + j];
            if r >= r_od + 3.0 {
                continue;
            }
            let hw = hw_grid[i * ny + j].max(1.0);
            let base = (i * ny + j) * nz;
            for (k, &z) in zs.iter().enumerate() {
                let rb_z = r_b + crown * (z / hw).powi(2);
                let bmat = rb_z - r;
                d[base + k] = rint(d[base + k] as f64, bmat, bore_round) as f32;
            }
        }
    }
    for i in 0..nx {
        for j in 0..ny {
            for k in 0..nz {
                if i == 0 || i + 1 == nx || j == 0 || j + 1 == ny || k == 0 || k + 1 == nz {
                    let idx = (i * ny + j) * nz + k;
                    d[idx] = d[idx].max(h as f32);
                }
            }
        }
    }

    // Save smooth pre-texture SDF `d_pre_tex` (`N5`, `hzm.build(tex=False)` parity)
    let d_pre_tex = d.clone();

    // Compute cavity mask on `d_pre_tex` and homogenized 2D `tmap_eff_2d` (`skin + rho^1.8`, `N7`, `evaluate.py::tmap_eff` parity)
    let skin_t = p["skin_t"] as f32;
    let rho_blade_eff = effective_blade_rho(p);
    let mut core = vec![false; total];
    let mut any_core = false;
    for i in 0..nx {
        let x = xs[i];
        if !include_blade && x > 92.0 {
            continue;
        }
        if include_blade && x > blade_prof.x_tip - p["tip_reserve"].max(6.0) - 4.0 {
            continue;
        }
        let (yi_b, ys_b) = if include_blade && x >= blade_prof.x_heel {
            let u_b = ((x - blade_prof.x_heel) / (blade_prof.x_tip - blade_prof.x_heel).max(1e-9))
                .clamp(0.0, 1.0);
            let idx_f = u_b * ((blade_prof.xs.len() - 1) as f64);
            let i0 = (idx_f.floor() as usize).min(blade_prof.xs.len() - 2);
            let frac = idx_f - (i0 as f64);
            (
                blade_prof.y_in[i0] * (1.0 - frac) + blade_prof.y_in[i0 + 1] * frac,
                blade_prof.y_sp[i0] * (1.0 - frac) + blade_prof.y_sp[i0 + 1] * frac,
            )
        } else {
            (-1e9, 1e9)
        };
        for j in 0..ny {
            let y = ys[j];
            if include_blade && x >= blade_prof.x_heel && (y - yi_b < 4.2 || ys_b - y < 4.2) {
                continue;
            }
            let r = r_grid[i * ny + j];
            let rho_z = if x < p["x_t"] || r < r_od + 1.0 {
                p["rho_ring"]
            } else if x < GUARD_X_MM {
                p["rho_grip"]
            } else if x < 92.0 {
                p["rho_guard"]
            } else {
                rho_blade_eff
            };
            if rho_z >= 0.95 {
                continue;
            }
            let base = (i * ny + j) * nz;
            for k in 0..nz {
                if d_pre_tex[base + k] < -(skin_t + 0.5) {
                    core[base + k] = true;
                    any_core = true;
                }
            }
        }
    }
    let do_core = if any_core {
        edt_out(&core, nx, ny, nz, h)
    } else {
        vec![10.0_f32; total]
    };

    let mut tmap_eff_2d = vec![0.0_f64; nx * ny];
    for i in 0..nx {
        let x = xs[i];
        for j in 0..ny {
            let r = r_grid[i * ny + j];
            let rho_z = if x < p["x_t"] || r < r_od + 1.0 {
                p["rho_ring"]
            } else if x < GUARD_X_MM {
                p["rho_grip"]
            } else if x < 92.0 {
                p["rho_guard"]
            } else if include_blade {
                rho_blade_eff
            } else {
                1.0
            };
            let rho_stiff = jarambit_g5::material::core_relative_stiffness(rho_z, p["lat_cell"], p["lat_mix"]);
            let base = (i * ny + j) * nz;
            let mut w_col = 0.0_f64;
            for k in 0..nz {
                let idx = base + k;
                if d_pre_tex[idx] < 0.0 {
                    let is_cav = rho_z < 0.95 && do_core[idx] <= 0.5 && d_pre_tex[idx] < -skin_t;
                    w_col += if is_cav { rho_stiff } else { 1.0 };
                }
            }
            tmap_eff_2d[i * ny + j] = w_col * h;
        }
    }

    // 6. Surface texture (`tex_amp`, `w_knurl`, `w_ridge`, `scope_grip/ring/blade`) and dorsal jimping (`jimp_n/p/d`)
    let tex_amp = p["tex_amp"];
    let jimp_n = p["jimp_n"].round() as i32;
    if tex_amp > 0.0 || jimp_n >= 1 {
        let l_g = p["L_g"];
        let xj1 = (GUARD_X_MM + 2.0).min(99.0);
        let pj = p["jimp_p"].max(1.5);
        let xj0 = xj1 - (jimp_n as f64) * pj;
        for (i, &x) in xs.iter().enumerate() {
            let sx = x - p["x_t"];
            let gc = eval_grip_slice(p, x);
            let tt_half = (0.5 * gc.t_sec).max(1.0);
            let a_t = (gc.yt - gc.ym).max(0.3);
            let a_b = (gc.ym - gc.yb).max(0.3);
            let win_grip = smoothstep(2.0, 6.0, sx) * (1.0 - smoothstep(l_g - 8.0, l_g - 3.0, sx));
            let scope_w = if x < p["x_t"] {
                p["scope_ring"]
            } else if x < GUARD_X_MM {
                p["scope_grip"] * win_grip
            } else {
                p["scope_blade"] * smoothstep(blade_prof.x_heel + 3.0, blade_prof.x_heel + 8.0, x)
                    * (1.0 - smoothstep(blade_prof.x_tip - 14.0, blade_prof.x_tip - 8.0, x))
            };
            for (j, &y) in ys.iter().enumerate() {
                let aa = if y >= gc.ym { a_t } else { a_b };
                let base = (i * ny + j) * nz;
                // G5.1: no texture on a ground bevel or its finishing allowance (1 off them, exactly G5.0)
                let w_bevel = if x >= GUARD_X_MM && tex_amp > 0.0 && scope_w > 0.01 {
                    bevel_texture_weight(p, &blade_prof, x, y)
                } else {
                    1.0
                };
                for (k, &z) in zs.iter().enumerate() {
                    let dv = d[base + k];
                    if dv.abs() >= 1.2 {
                        continue;
                    }
                    // Keep bore interior untouched by texture
                    if i < nx_hz && r_grid[i * ny + j] < r_b + p["bore_round"] + 0.8 {
                        continue;
                    }
                    let psi = (z / tt_half).atan2((y - gc.ym) / aa);
                    let cos_psi = psi.cos();
                    let land = smoothstep(0.55, 0.80, cos_psi);
                    let top = smoothstep(0.86, 0.96, cos_psi);
                    let mut disp = 0.0_f64;
                    if tex_amp > 0.0 && scope_w > 0.01 {
                        let tx = eval_surface_texture(p, sx, y - gc.ym, l_g);
                        disp += tex_amp * tx * scope_w * w_bevel * (1.0 - land);
                    }
                    if jimp_n >= 1 && x >= xj0 && x <= xj1 && top > 0.01 {
                        let prof_j =
                            0.5 * (1.0 - (2.0 * std::f64::consts::PI * (x - xj0) / pj).cos());
                        let fade_j =
                            smoothstep(xj0, xj0 + pj, x) * (1.0 - smoothstep(xj1 - pj, xj1, x));
                        disp += p["jimp_d"] * prof_j * fade_j * top;
                    }
                    if x >= GUARD_X_MM && dv >= 0.0 {
                        // Never allow blade surface texture to push material outside the 2D blade profile
                        disp = disp.max(0.0);
                    }
                    d[base + k] += disp as f32;
                }
            }
        }
        for i in 0..nx {
            for j in 0..ny {
                for k in 0..nz {
                    if i == 0 || i + 1 == nx || j == 0 || j + 1 == ny || k == 0 || k + 1 == nz {
                        let idx = (i * ny + j) * nz + k;
                        d[idx] = d[idx].max(h as f32);
                    }
                }
            }
        }
    }

    // 7. Optional AM interior architecture (`hzm.py::cavity_mask` + `am_build` parity + Rule 4 connectivity)
    let mut am_connectivity = AmConnectivityStats::default();
    let d_am = if with_am {
        let lat_cell = p["lat_cell"];
        let lat_mix = p["lat_mix"];
        let lat_aniso_x = p["lat_aniso_x"];
        let lat_grade = p["lat_grade"];
        let rho_grad_wall = p["rho_grad_wall"];
        let shear_web_amp = p["shear_web_amp"];
        let rib_half = if p["rib_w"] > 0.0 {
            (0.4 + 0.4 * p["rib_w"]) as f32
        } else {
            -1.0
        };
        // G5.0 sheet TPMS ([material.tpms] strut_tpms_allowed = false): solid within half a sheet thickness of the
        // level set f = 0, measured by the first-order distance |f| / |grad f|. The thickness gives each zone its
        // target density and is floored at [am] sheet_min_mm (`material::tpms_sheet_thickness`).
        let t_sheet = |rho: f64| jarambit_g5::material::tpms_sheet_thickness(rho, lat_cell, lat_mix);
        let (t_ring, t_grip, t_guard, t_blade) = (
            t_sheet(p["rho_ring"]),
            t_sheet(p["rho_grip"]),
            t_sheet(p["rho_guard"]),
            t_sheet(rho_blade_eff),
        );
        // Ports and drain holes ([am] port_d_min_mm, ports_per_cavity_min): `port_d` wide, cut on both sides of
        // the mid-plane rib (which splits a cavity into two halves) so no port opens onto the rib
        let port_r = 0.5 * p["port_d"];
        let z_offs: Vec<f64> = if rib_half > 0.0 {
            let zo = rib_half as f64 + port_r + 0.3;
            vec![-zo, zo]
        } else {
            vec![0.0]
        };
        let web_st = [p["centre_ring"], p["centre_mid"], p["centre_idx"]];

        let mut am = d.clone();
        if any_core {
            for i in 0..nx {
                let x = xs[i];
                let (yi_b, ys_b) = if include_blade && x >= blade_prof.x_heel {
                    let u_b = ((x - blade_prof.x_heel)
                        / (blade_prof.x_tip - blade_prof.x_heel).max(1e-9))
                    .clamp(0.0, 1.0);
                    let idx_f = u_b * ((blade_prof.xs.len() - 1) as f64);
                    let i0 = (idx_f.floor() as usize).min(blade_prof.xs.len() - 2);
                    let frac = idx_f - (i0 as f64);
                    (
                        blade_prof.y_in[i0] * (1.0 - frac) + blade_prof.y_in[i0 + 1] * frac,
                        blade_prof.y_sp[i0] * (1.0 - frac) + blade_prof.y_sp[i0 + 1] * frac,
                    )
                } else {
                    (-1e9, 1e9)
                };
                let y_web_axis = eval_grip_slice(p, x).ym;
                for j in 0..ny {
                    let y = ys[j];
                    if include_blade && x >= blade_prof.x_heel && (y - yi_b < 3.6 || ys_b - y < 3.6)
                    {
                        continue;
                    }
                    let r = r_grid[i * ny + j];
                    let (rho_z, t_lvl) = if x < p["x_t"] || r < r_od + 1.0 {
                        (p["rho_ring"], t_ring)
                    } else if x < GUARD_X_MM {
                        (p["rho_grip"], t_grip)
                    } else if x < 92.0 {
                        (p["rho_guard"], t_guard)
                    } else {
                        (rho_blade_eff, t_blade)
                    };
                    if rho_z >= 0.95 {
                        continue;
                    }
                    let base = (i * ny + j) * nz;
                    for (k, &z) in zs.iter().enumerate() {
                        let idx = base + k;
                        let dv = d[idx];
                        if do_core[idx] <= 0.5 && dv < -skin_t {
                            let (f, gnorm) = tpms_val_and_grad(x, y, z, lat_cell, lat_mix, lat_aniso_x);
                            // Wall grading (`lat_grade`, `rho_grad_wall`) thickens the sheet near the skin
                            let depth_past_skin = (-dv - skin_t).max(0.0) as f64;
                            let wall_boost = rho_grad_wall * (-depth_past_skin / 2.0).exp()
                                + 0.15 * lat_grade * (1.0 - (z.abs() / 6.0).min(1.0));
                            let half_t = 0.5 * t_lvl * (1.0 + wall_boost);
                            let mut li = (f.abs() / gnorm.max(1e-6) - half_t) as f32;
                            if rib_half > 0.0 {
                                li = li.min((z.abs() as f32) - rib_half);
                            }
                            // Internal transverse shear webs, each with drain holes on the grip axis (one per
                            // rib side), so the web compartments form one drainable cavity
                            if shear_web_amp > 0.05 {
                                let web_half = 0.35 * shear_web_amp;
                                let s_grip = x - p["x_t"];
                                let hole = z_offs
                                    .iter()
                                    .map(|&zo| port_r - (y - y_web_axis).hypot(z - zo))
                                    .fold(f64::NEG_INFINITY, f64::max);
                                for &st in &web_st {
                                    let dw = ((s_grip - st).abs() - web_half).max(hole);
                                    li = li.min(dw as f32);
                                }
                                // A sheet TPMS splits the cavity into two interleaved labyrinths: the hole runs on
                                // as a tunnel through the sheet for half a cell each side, so both reach it
                                if web_st.iter().any(|&st| (s_grip - st).abs() < 0.5 * lat_cell) {
                                    li = li.max(hole as f32);
                                }
                            }
                            let void_d = li.min(-(dv + skin_t));
                            am[idx] = dv.max(void_d);
                        }
                    }
                }
            }
        }
        // Ports run one lattice cell into the cavity, cutting through the sheet, so both TPMS labyrinths drain
        let carve_depth = skin_t + (lat_cell as f32).max(1.5);
        // Grip ports along -y through the grip underside, between the webs (`port_n` stations in 2..=4)
        if p["rho_grip"] < 0.95 {
            let n_ports = (p["port_n"].round() as usize).clamp(2, 4);
            let cand = [
                0.5 * (web_st[0] + web_st[1]),
                GUARD_X_MM - 4.0 - p["x_t"],
                0.5 * web_st[0],
                0.5 * (web_st[1] + web_st[2]),
            ];
            for &ps in cand.iter().take(n_ports) {
                let px = p["x_t"] + ps;
                for (i, &x) in xs.iter().enumerate() {
                    if x >= GUARD_X_MM - 1.5 || (x - px).abs() > port_r + 1.0 {
                        continue;
                    }
                    for (j, &y) in ys.iter().enumerate() {
                        if y >= 0.0 {
                            continue;
                        }
                        let base = (i * ny + j) * nz;
                        for (k, &z) in zs.iter().enumerate() {
                            if d[base + k] > -carve_depth {
                                for &zo in &z_offs {
                                    let cyl = ((x - px).hypot(z - zo) - port_r) as f32;
                                    am[base + k] = am[base + k].max(-cyl);
                                }
                            }
                        }
                    }
                }
            }
        }
        // Guard ports along -y through the guard underside
        if p["rho_guard"] < 0.95 {
            let px = GUARD_X_MM + 3.0;
            for (i, &x) in xs.iter().enumerate() {
                if (x - px).abs() > port_r + 1.0 {
                    continue;
                }
                for (j, &y) in ys.iter().enumerate() {
                    if y >= 0.0 {
                        continue;
                    }
                    let base = (i * ny + j) * nz;
                    for (k, &z) in zs.iter().enumerate() {
                        if d[base + k] > -carve_depth {
                            for &zo in &z_offs {
                                let cyl = ((x - px).hypot(z - zo) - port_r) as f32;
                                am[base + k] = am[base + k].max(-cyl);
                            }
                        }
                    }
                }
            }
        }
        // Ring ports: two radial ports through the rear of the ring outer wall (150 and 210 degrees from +x, away
        // from the grip and clear of the bore)
        if p["rho_ring"] < 0.95 {
            let cx = p["cx"];
            let r_b = p["R_b"];
            for th_deg in [150.0_f64, 210.0] {
                let (sn, cs) = th_deg.to_radians().sin_cos();
                for (i, &x) in xs.iter().enumerate() {
                    for (j, &y) in ys.iter().enumerate() {
                        let (vx, vy) = (x - cx, y);
                        let along = vx * cs + vy * sn;
                        if along < r_b + 1.0 {
                            continue;
                        }
                        let perp = (vx - along * cs).hypot(vy - along * sn);
                        if perp > port_r + 1.0 {
                            continue;
                        }
                        let base = (i * ny + j) * nz;
                        for (k, &z) in zs.iter().enumerate() {
                            if d[base + k] > -carve_depth {
                                for &zo in &z_offs {
                                    let cyl = (perp.hypot(z - zo) - port_r) as f32;
                                    am[base + k] = am[base + k].max(-cyl);
                                }
                            }
                        }
                    }
                }
            }
        }
        // Blade vents along z through both faces at two medial stations (30 % and 55 % of the blade), when the
        // blade is hollow; each station sits inside the cavity and >= 3.5 mm + port radius from both edges
        if include_blade && rho_blade_eff < 0.95 {
            let k_mid = zs
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.abs().total_cmp(&b.abs()))
                .map(|(k, _)| k)
                .unwrap_or(nz / 2);
            let profile_at = |x: f64| {
                let u_b = ((x - blade_prof.x_heel) / (blade_prof.x_tip - blade_prof.x_heel).max(1e-9))
                    .clamp(0.0, 1.0);
                let idx_f = u_b * ((blade_prof.xs.len() - 1) as f64);
                let i0 = (idx_f.floor() as usize).min(blade_prof.xs.len() - 2);
                let frac = idx_f - (i0 as f64);
                (
                    blade_prof.y_in[i0] * (1.0 - frac) + blade_prof.y_in[i0 + 1] * frac,
                    blade_prof.y_sp[i0] * (1.0 - frac) + blade_prof.y_sp[i0 + 1] * frac,
                )
            };
            for frac_target in [0.30_f64, 0.55] {
                let x_target = blade_prof.x_heel + frac_target * (blade_prof.x_tip - blade_prof.x_heel);
                let mut best_station: Option<(f64, f64)> = None;
                let mut best_dist = f64::INFINITY;
                for (i, &x) in xs.iter().enumerate() {
                    if x < 92.5 || x > blade_prof.x_tip - p["tip_reserve"].max(6.0) - 6.0 {
                        continue;
                    }
                    let (yi_b, ys_b) = profile_at(x);
                    let py = 0.5 * (yi_b + ys_b);
                    if py - yi_b < 3.5 + port_r + 0.4 || ys_b - py < 3.5 + port_r + 0.4 {
                        continue;
                    }
                    let j_mid = (((py - ys[0]) / h).round() as isize).clamp(0, ny as isize - 1) as usize;
                    if core[(i * ny + j_mid) * nz + k_mid] {
                        let dist = (x - x_target).abs();
                        if dist < best_dist {
                            best_dist = dist;
                            best_station = Some((x, py));
                        }
                    }
                }
                let Some((px, py)) = best_station else { continue };
                for (i, &x) in xs.iter().enumerate() {
                    if (x - px).abs() > port_r + 1.0 {
                        continue;
                    }
                    let (yi_b, ys_b) = profile_at(x);
                    for (j, &y) in ys.iter().enumerate() {
                        // Never cut into the 3.5 mm edge bevel bands (`S5-2`, `P4`)
                        if y - yi_b < 3.6 || ys_b - y < 3.6 {
                            continue;
                        }
                        let r_xy = (x - px).hypot(y - py);
                        if r_xy > port_r + 0.5 {
                            continue;
                        }
                        let cyl_z = (r_xy - port_r) as f32;
                        let base = (i * ny + j) * nz;
                        for k in 0..nz {
                            if d[base + k] > -carve_depth {
                                am[base + k] = am[base + k].max(-cyl_z);
                            }
                        }
                    }
                }
            }
        }
        am_connectivity = enforce_am_connectivity(
            &mut am,
            Some(&d),
            nx,
            ny,
            nz,
            h as f32,
            jarambit_g5::requirements::rs_v5().am.drain_channel_min_mm,
        );
        if include_blade {
            // Preserve the solid 3.5 mm edge bevel bands and tip apex 100% unmodified in `d_am` (`S5-2`, `P4`)
            for (i, &x) in xs.iter().enumerate() {
                if x < blade_prof.x_heel || x > blade_prof.x_tip {
                    continue;
                }
                let u_b = ((x - blade_prof.x_heel)
                    / (blade_prof.x_tip - blade_prof.x_heel).max(1e-9))
                .clamp(0.0, 1.0);
                let idx_f = u_b * ((blade_prof.xs.len() - 1) as f64);
                let i0 = (idx_f.floor() as usize).min(blade_prof.xs.len() - 2);
                let frac = idx_f - (i0 as f64);
                let yi_b = blade_prof.y_in[i0] * (1.0 - frac) + blade_prof.y_in[i0 + 1] * frac;
                let ys_b = blade_prof.y_sp[i0] * (1.0 - frac) + blade_prof.y_sp[i0 + 1] * frac;
                for (j, &y) in ys.iter().enumerate() {
                    if y - yi_b < 3.55 || ys_b - y < 3.55 {
                        let base = (i * ny + j) * nz;
                        for k in 0..nz {
                            am[base + k] = d[base + k];
                        }
                    }
                }
            }
        }
        Some(am)
    } else {
        None
    };

    VoxelGrid3D {
        x: xs,
        y: ys,
        z: zs,
        nx,
        ny,
        nz,
        h,
        d,
        d_pre_tex,
        tmap_eff_2d,
        d_am,
        include_blade,
        am_connectivity,
    }
}
