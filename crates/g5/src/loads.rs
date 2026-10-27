//! Load cases of RS-karambit-v5 (`[loads]`, `[fatigue]`, `[sequence]`, `[buckling]`).
//!
//! Every case is a limit load. Its safety factor is the allowable of the governing wall (`material`) over the peak
//! stress at that load. The objective compares each SF with its requirement: `[margins]` sf_yield 1.2 (sf_ultimate
//! 1.3 for the ultimate case, sf_ring 2 for the owner's ring loads).
//!
//! **Blade sections.** `blade_section` integrates the decoded thickness law across the planform, along the normal of
//! the smoothed midline, giving area, minor-axis bending (through the thickness), major-axis bending (in plane) and
//! thin-section torsion. A force is resolved, at each section, into axial force, bending through the thickness,
//! in-plane bending and torsion about the local blade axis. On a curved claw, a force normal to the blade 5 mm from
//! the point also twists every section behind it. The stresses are superposed at one point:
//! σ_eq = sqrt((|σ_axial| + |σ_bend| + |σ_inplane|)² + 3τ²). This is conservative.
//!
//! **Tiers.**
//! - Tier 0 (`analytic_loads`) evaluates every case on these sections. The hand side (grip, guard, ring) uses the G3
//!   closed-section grip model with the RS-v5 loads.
//! - Tier 1 (`plate_blade_cases`) replaces the blade lateral, twist and pull cases by a CST+DKT sandwich-plate solve
//!   of the whole blade, on a fine mesh, with smooth loads:
//!   - a patch force 5 mm behind the point;
//!   - a distributed couple over the working span;
//!   - an in-plane pull on the held 20 mm.
//! - The solid tip (Tier 2) follows in a later change.

use crate::g5_sdf::{
    effective_blade_rho, eval_blade_thickness_and_sdf2, eval_grip_slice, eval_guard_lobe_y, BladeProfile2D,
    GUARD_X_MM,
};
use crate::material::{self, Wall, AS_BUILT_WALL, GROUND_WALL, HOLLOW_BLADE_SKIN};
use crate::requirements::rs_v5;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

type Params = BTreeMap<&'static str, f64>;

/// Smoothed midline of the blade planform from the point back to the heel.
pub struct Midline {
    pts: Vec<(f64, f64)>,
    s: Vec<f64>,
}

impl Midline {
    pub fn of(b: &BladeProfile2D) -> Self {
        let n = b.xs.len();
        let pts: Vec<(f64, f64)> =
            (0..n).rev().map(|i| (b.xs[i], 0.5 * (b.y_in_smooth[i] + b.y_sp_smooth[i]))).collect();
        let mut s = vec![0.0];
        for k in 1..n {
            s.push(s[k - 1] + (pts[k].0 - pts[k - 1].0).hypot(pts[k].1 - pts[k - 1].1));
        }
        Self { pts, s }
    }

    /// Arc length from the point to the heel (mm).
    pub fn length(&self) -> f64 {
        *self.s.last().unwrap_or(&0.0)
    }

    /// Point on the midline at arc length `s` from the point, and the unit tangent there, pointing to the heel.
    pub fn at(&self, s: f64) -> ((f64, f64), (f64, f64)) {
        let s = s.clamp(0.0, self.length());
        let k = self.s.partition_point(|&v| v < s).clamp(1, self.s.len() - 1);
        let (a, b) = (self.pts[k - 1], self.pts[k]);
        let ds = (self.s[k] - self.s[k - 1]).max(1e-12);
        let f = ((s - self.s[k - 1]) / ds).clamp(0.0, 1.0);
        let t = ((b.0 - a.0) / ds, (b.1 - a.1) / ds);
        let tn = t.0.hypot(t.1).max(1e-12);
        ((a.0 + f * (b.0 - a.0), a.1 + f * (b.1 - a.1)), (t.0 / tn, t.1 / tn))
    }
}

/// Section of the blade normal to its midline.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BladeSection {
    pub s_mm: f64,
    pub centroid: (f64, f64),
    pub tangent: (f64, f64),
    pub width_mm: f64,
    pub t_max_mm: f64,
    pub area_mm2: f64,
    /// Bending through the thickness (moment vector in plane, across the blade).
    pub i_minor_mm4: f64,
    /// In-plane bending (moment vector normal to the blade).
    pub i_major_mm4: f64,
    pub c_major_mm: f64,
    /// Thin open-section torsion constant (1/3) ∫ t³ dη.
    pub j_mm4: f64,
    pub hollow: bool,
}

impl BladeSection {
    /// Equivalent stress (MPa) for a section force: axial `n` (N), bending through the thickness `m_b` and in plane
    /// `m_ip` (N·mm), torsion `t` (N·mm), superposed at one point.
    pub fn sigma_eq(&self, n: f64, m_b: f64, m_ip: f64, t: f64) -> f64 {
        let s = n.abs() / self.area_mm2.max(1e-9)
            + m_b.abs() * 0.5 * self.t_max_mm / self.i_minor_mm4.max(1e-12)
            + m_ip.abs() * self.c_major_mm / self.i_major_mm4.max(1e-12);
        let tau = t.abs() * self.t_max_mm / self.j_mm4.max(1e-12);
        (s * s + 3.0 * tau * tau).sqrt()
    }

    /// The governing wall of the section: ground solid blade, or the skin of a hollow blade.
    pub fn wall(&self, p: &Params) -> (f64, Wall) {
        if self.hollow {
            (p["skin_t"], HOLLOW_BLADE_SKIN)
        } else {
            (self.t_max_mm, GROUND_WALL)
        }
    }
}

/// True if `(x, y)` lies inside the blade's planform outline (between its two edges), windows included.
pub fn inside_outline(b: &BladeProfile2D, x: f64, y: f64) -> bool {
    let n = b.xs.len();
    if x < b.xs[0] || x > b.xs[n - 1] {
        return false;
    }
    let i = b.xs.partition_point(|&q| q < x).clamp(1, n - 1);
    let f = ((x - b.xs[i - 1]) / (b.xs[i] - b.xs[i - 1]).max(1e-12)).clamp(0.0, 1.0);
    let yi = b.y_in[i - 1] * (1.0 - f) + b.y_in[i] * f;
    let ys = b.y_sp[i - 1] * (1.0 - f) + b.y_sp[i] * f;
    y >= yi && y <= ys
}

/// Integrate the section at arc length `s` from the point, between the two edges of the planform; fenestration
/// windows contribute no material. `None` if the section has no material.
pub fn blade_section(p: &Params, b: &BladeProfile2D, ml: &Midline, s: f64) -> Option<BladeSection> {
    let (m, tau) = ml.at(s);
    let nu = (-tau.1, tau.0);
    let rho_b = effective_blade_rho(p);
    let rho_e = material::core_relative_stiffness(rho_b, p["lat_cell"], p["lat_mix"]);
    let skin = p["skin_t"];
    let core_x_max = b.x_tip - p["tip_reserve"].max(6.0) - 4.0;
    let step = 0.05_f64;
    let n_side = 600usize; // +-30 mm
    // The chord between the two edges through the midline point; windows (t = 0) inside it carry nothing
    let inside = |eta: f64| inside_outline(b, m.0 + eta * nu.0, m.1 + eta * nu.1);
    let thick = |eta: f64| -> f64 {
        let (x, y) = (m.0 + eta * nu.0, m.1 + eta * nu.1);
        let (t, d2) = eval_blade_thickness_and_sdf2(p, b, x, y);
        if d2 <= 0.0 && t > 0.0 {
            t
        } else {
            0.0
        }
    };
    if !inside(0.0) {
        return None;
    }
    let mut lo = 0usize;
    while lo < n_side && inside(-((lo + 1) as f64) * step) {
        lo += 1;
    }
    let mut hi = 0usize;
    while hi < n_side && inside(((hi + 1) as f64) * step) {
        hi += 1;
    }
    let (eta0, eta1) = (-(lo as f64) * step, (hi as f64) * step);
    let width = eta1 - eta0 + step;
    let mut samples = Vec::with_capacity(lo + hi + 1);
    let mut any_hollow = false;
    for k in 0..=(lo + hi) {
        let eta = eta0 + k as f64 * step;
        let t = thick(eta);
        let (x, _) = (m.0 + eta * nu.0, m.1 + eta * nu.1);
        let d_edge = (eta - eta0).min(eta1 - eta);
        let hollow_here = rho_b < 0.95 && d_edge > 4.2 && x <= core_x_max && t > 2.0 * skin + 1.0;
        any_hollow |= hollow_here;
        let hc = if hollow_here { t - 2.0 * skin } else { 0.0 };
        // effective t (area), t³ (bending) and t³ (torsion, open section) with the core at relative stiffness rho_e
        let t_a = t - (1.0 - rho_e) * hc;
        let t3 = t.powi(3) - (1.0 - rho_e) * hc.powi(3);
        samples.push((eta, t, t_a, t3));
    }
    let area: f64 = samples.iter().map(|s| s.2 * step).sum();
    if area <= 1e-9 {
        return None;
    }
    let eta_c = samples.iter().map(|s| s.0 * s.2 * step).sum::<f64>() / area;
    let i_minor: f64 = samples.iter().map(|s| s.3 / 12.0 * step).sum();
    let i_major: f64 = samples.iter().map(|s| s.2 * (s.0 - eta_c).powi(2) * step).sum();
    let j: f64 = samples.iter().map(|s| s.3 / 3.0 * step).sum();
    let t_max = samples.iter().map(|s| s.1).fold(0.0, f64::max);
    Some(BladeSection {
        s_mm: s,
        centroid: (m.0 + eta_c * nu.0, m.1 + eta_c * nu.1),
        tangent: tau,
        width_mm: width,
        t_max_mm: t_max,
        area_mm2: area,
        i_minor_mm4: i_minor,
        i_major_mm4: i_major,
        c_major_mm: (eta_c - eta0).max(eta1 - eta_c),
        j_mm4: j,
        hollow: any_hollow,
    })
}

/// Section forces at section `sec` from a force `f = (fx, fy, fz)` (N) applied at in-plane point `q`: axial force,
/// bending through the thickness, in-plane bending and torsion about the local blade axis.
pub fn section_forces(sec: &BladeSection, q: (f64, f64), f: (f64, f64, f64)) -> (f64, f64, f64, f64) {
    let r = (q.0 - sec.centroid.0, q.1 - sec.centroid.1);
    let tau = sec.tangent;
    let n = -(f.0 * tau.0 + f.1 * tau.1); // tension positive when q is pulled away from the heel
    let m_ip = r.0 * f.1 - r.1 * f.0;
    // out-of-plane force fz at q: moment r x (0, 0, fz) = (r_y fz, -r_x fz, 0)
    let m = (r.1 * f.2, -r.0 * f.2);
    let torsion = m.0 * tau.0 + m.1 * tau.1;
    let bend = (m.0 * m.0 + m.1 * m.1 - torsion * torsion).max(0.0).sqrt();
    (n, bend, m_ip, torsion)
}

/// Exact JSON for f64 values that may be non-finite: numbers, or "inf" / "-inf" / "nan" (JSON has no non-finite
/// numbers, and an archive row must re-score bit for bit).
mod f64x {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &f64, s: S) -> Result<S::Ok, S::Error> {
        if v.is_finite() {
            s.serialize_f64(*v)
        } else if v.is_nan() {
            s.serialize_str("nan")
        } else if *v > 0.0 {
            s.serialize_str("inf")
        } else {
            s.serialize_str("-inf")
        }
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Repr {
        N(f64),
        S(String),
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<f64, D::Error> {
        match Repr::deserialize(d)? {
            Repr::N(v) => Ok(v),
            Repr::S(t) => match t.as_str() {
                "inf" => Ok(f64::INFINITY),
                "-inf" => Ok(f64::NEG_INFINITY),
                "nan" => Ok(f64::NAN),
                other => Err(serde::de::Error::custom(format!("not an f64: {other}"))),
            },
        }
    }
}

/// `f64x` for a pair.
mod f64x2 {
    use serde::{Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &(f64, f64), s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeTuple;
        struct W(f64);
        impl serde::Serialize for W {
            fn serialize<S2: Serializer>(&self, s: S2) -> Result<S2::Ok, S2::Error> {
                super::f64x::serialize(&self.0, s)
            }
        }
        let mut t = s.serialize_tuple(2)?;
        t.serialize_element(&W(v.0))?;
        t.serialize_element(&W(v.1))?;
        t.end()
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<(f64, f64), D::Error> {
        #[derive(serde::Deserialize)]
        struct W(#[serde(with = "super::f64x")] f64);
        let (a, b): (W, W) = serde::Deserialize::deserialize(d)?;
        Ok((a.0, b.0))
    }
}

/// Model of the blade lateral, twist and pull cases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BladeModel {
    /// Integrated sections (Tier 0)
    #[default]
    Analytic,
    /// Whole-blade sandwich plate (Tier 1)
    Plate,
}

/// Every RS-karambit-v5 safety factor of one design, with the governing quantities.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct G5Loads {
    pub blade_model: BladeModel,
    #[serde(with = "f64x")]
    pub lateral_force_n: f64,
    #[serde(with = "f64x")]
    pub lateral_arm_mm: f64,
    #[serde(with = "f64x")]
    pub lateral_limit_sf: f64,
    #[serde(with = "f64x")]
    pub lateral_ultimate_sf: f64,
    #[serde(with = "f64x")]
    pub twist_limit_sf: f64,
    #[serde(with = "f64x")]
    pub tip_axial_sf: f64,
    #[serde(with = "f64x")]
    pub tip_lateral_sf: f64,
    #[serde(with = "f64x")]
    pub tip_twist_sf: f64,
    #[serde(with = "f64x")]
    pub pull_sf: f64,
    #[serde(with = "f64x")]
    pub buckling_lambda: f64,
    #[serde(with = "f64x")]
    pub teeth_sf: f64,
    #[serde(with = "f64x")]
    pub fatigue_sf: f64,
    #[serde(with = "f64x")]
    pub grip_lateral_sf: f64,
    #[serde(with = "f64x")]
    pub grip_twist_sf: f64,
    #[serde(with = "f64x")]
    pub grip_pull_sf: f64,
    #[serde(with = "f64x")]
    pub grip_squeeze_sf: f64,
    #[serde(with = "f64x")]
    pub guard_push_sf: f64,
    #[serde(with = "f64x")]
    pub ring_bearing_sf: f64,
    #[serde(with = "f64x")]
    pub ring_back_sf: f64,
    #[serde(with = "f64x")]
    pub ring_any_direction_sf: f64,
    #[serde(with = "f64x")]
    pub hand_fatigue_sf: f64,
    #[serde(with = "f64x")]
    pub seq_lateral: f64,
    #[serde(with = "f64x")]
    pub seq_twist: f64,
    #[serde(with = "f64x")]
    pub seq_pull: f64,
    /// Where the Tier-1 lateral and twist peaks sit: (midline arc from the point, offset across the blade), mm
    #[serde(with = "f64x2")]
    pub lateral_peak_at: (f64, f64),
    #[serde(with = "f64x2")]
    pub twist_peak_at: (f64, f64),
}

impl G5Loads {
    /// Worst blade-before-hand ratio over the three load cases that load both.
    pub fn seq_min(&self) -> f64 {
        self.seq_lateral.min(self.seq_twist).min(self.seq_pull)
    }
}

/// Sections every `ds` mm from `s0` to the heel.
fn sections(p: &Params, b: &BladeProfile2D, ml: &Midline, s0: f64, s1: f64, ds: f64) -> Vec<BladeSection> {
    let mut out = Vec::new();
    let mut s = s0;
    while s <= s1.min(ml.length() - 0.5) {
        if let Some(sec) = blade_section(p, b, ml, s) {
            out.push(sec);
        }
        s += ds;
    }
    out
}

fn allow_y(p: &Params, sec: &BladeSection) -> f64 {
    let (t, w) = sec.wall(p);
    material::yield_allowable(t, w)
}

fn allow_u(p: &Params, sec: &BladeSection) -> f64 {
    let (t, w) = sec.wall(p);
    material::ultimate_allowable(t, w)
}

/// Worst SF over sections of a section-force function.
fn worst_sf(
    p: &Params,
    secs: &[BladeSection],
    forces: &dyn Fn(&BladeSection) -> (f64, f64, f64, f64),
    ultimate: bool,
) -> f64 {
    secs.iter()
        .map(|sec| {
            let (n, mb, mip, t) = forces(sec);
            let a = if ultimate { allow_u(p, sec) } else { allow_y(p, sec) };
            a / sec.sigma_eq(n, mb, mip, t).max(1e-9)
        })
        .fold(f64::INFINITY, f64::min)
}

/// Lowest elastic buckling load (N) of the point as a tapered column, clamped `len` mm behind the point and loaded
/// axially at 0.5 mm, from the minor-axis stiffness E·I(s) of the decoded sections. Rayleigh-Ritz with the basis
/// (1 − ξ)^(k+2), k = 0..6, which satisfies the clamp.
pub fn tip_buckling_load_n(ei: &dyn Fn(f64) -> f64, s0: f64, len: f64) -> f64 {
    const NB: usize = 7;
    let nq = 400usize;
    let l = len - s0;
    let mut k = [[0.0_f64; NB]; NB];
    let mut g = [[0.0_f64; NB]; NB];
    for q in 0..nq {
        let xi = (q as f64 + 0.5) / nq as f64;
        let s = s0 + xi * l;
        let w = l / nq as f64;
        let e = ei(s);
        let mut d1 = [0.0; NB];
        let mut d2 = [0.0; NB];
        for (i, (a, c)) in d1.iter_mut().zip(d2.iter_mut()).enumerate() {
            let pw = (i + 2) as f64;
            *a = -pw * (1.0 - xi).powf(pw - 1.0) / l;
            *c = pw * (pw - 1.0) * (1.0 - xi).powf(pw - 2.0) / (l * l);
        }
        for i in 0..NB {
            for jj in 0..NB {
                k[i][jj] += e * d2[i] * d2[jj] * w;
                g[i][jj] += d1[i] * d1[jj] * w;
            }
        }
    }
    smallest_generalized_eigenvalue(&k, &g)
}

/// Smallest λ of K a = λ G a for symmetric K and SPD G (Cholesky reduction, then Jacobi).
fn smallest_generalized_eigenvalue<const N: usize>(k: &[[f64; N]; N], g: &[[f64; N]; N]) -> f64 {
    let mut lch = [[0.0_f64; N]; N];
    for i in 0..N {
        for j in 0..=i {
            let mut sum = g[i][j];
            for kk in 0..j {
                sum -= lch[i][kk] * lch[j][kk];
            }
            if i == j {
                if sum <= 0.0 {
                    return f64::NAN;
                }
                lch[i][i] = sum.sqrt();
            } else {
                lch[i][j] = sum / lch[j][j];
            }
        }
    }
    // C = L^-1 K L^-T
    let mut y = [[0.0_f64; N]; N]; // Y = L^-1 K
    for c in 0..N {
        for i in 0..N {
            let mut sum = k[i][c];
            for kk in 0..i {
                sum -= lch[i][kk] * y[kk][c];
            }
            y[i][c] = sum / lch[i][i];
        }
    }
    let mut a = [[0.0_f64; N]; N]; // A = Y L^-T  (solve A L^T = Y row by row)
    for r in 0..N {
        for i in 0..N {
            let mut sum = y[r][i];
            for kk in 0..i {
                sum -= a[r][kk] * lch[i][kk];
            }
            a[r][i] = sum / lch[i][i];
        }
    }
    for _sweep in 0..100 {
        let mut off = 0.0;
        for i in 0..N {
            for j in (i + 1)..N {
                off += a[i][j] * a[i][j];
            }
        }
        if off < 1e-24 {
            break;
        }
        for pp in 0..N {
            for qq in (pp + 1)..N {
                if a[pp][qq].abs() < 1e-300 {
                    continue;
                }
                let theta = (a[qq][qq] - a[pp][pp]) / (2.0 * a[pp][qq]);
                let t = theta.signum() / (theta.abs() + (theta * theta + 1.0).sqrt());
                let t = if theta == 0.0 { 1.0 } else { t };
                let c = 1.0 / (t * t + 1.0).sqrt();
                let s = t * c;
                for r in 0..N {
                    let (arp, arq) = (a[r][pp], a[r][qq]);
                    a[r][pp] = c * arp - s * arq;
                    a[r][qq] = s * arp + c * arq;
                }
                for r in 0..N {
                    let (apr, aqr) = (a[pp][r], a[qq][r]);
                    a[pp][r] = c * apr - s * aqr;
                    a[qq][r] = s * apr + c * aqr;
                }
            }
        }
    }
    (0..N).map(|i| a[i][i]).fold(f64::INFINITY, f64::min)
}

/// Closed-section properties of the grip at `x` (the G3 model): enclosed area, skin, bending about the two axes.
struct GripSection {
    h: f64,
    t: f64,
    ym: f64,
    a_enc: f64,
    perim: f64,
    i_lat: f64,
    i_inplane: f64,
    area: f64,
    hollow: bool,
}

fn grip_section(p: &Params, x: f64) -> GripSection {
    let rho_g = p["rho_grip"];
    let rho_e = material::core_relative_stiffness(rho_g, p["lat_cell"], p["lat_mix"]);
    let hollow = rho_g < 0.95;
    let skin_t = p["skin_t"];
    let gc = eval_grip_slice(p, x);
    let h = (gc.yt - gc.yb).max(8.0);
    let t = gc.t_sec.max(4.0);
    let a_enc = std::f64::consts::PI * 0.25 * h * t;
    let perim =
        std::f64::consts::PI * (3.0 * 0.5 * (h + t) - ((1.5 * h + 0.5 * t) * (0.5 * h + 1.5 * t)).sqrt());
    let (hi, ti) = if hollow { ((h - 2.0 * skin_t).max(0.0), (t - 2.0 * skin_t).max(0.0)) } else { (0.0, 0.0) };
    let ell_i = |a: f64, b: f64| std::f64::consts::PI / 64.0 * a * b.powi(3); // ellipse, axes a (along), b (across)
    let i_lat = ell_i(h, t) - (1.0 - rho_e) * ell_i(hi, ti);
    let i_inplane = ell_i(t, h) - (1.0 - rho_e) * ell_i(ti, hi);
    let area = a_enc - (1.0 - rho_e) * std::f64::consts::PI * 0.25 * hi * ti;
    GripSection { h, t, ym: gc.ym, a_enc, perim, i_lat, i_inplane, area, hollow }
}

impl GripSection {
    fn tau(&self, torque: f64, skin_t: f64) -> f64 {
        if self.hollow {
            torque.abs() / (2.0 * self.a_enc * skin_t).max(1.0)
        } else {
            3.0 * torque.abs() / (self.perim * 0.5 * self.t.min(self.h).powi(2)).max(1.0)
        }
    }
}

/// Tier-0 evaluation of every load case.
pub fn analytic_loads(p: &Params, b: &BladeProfile2D, ring_bearing_cap_n: f64, ring_back_cap_n: f64) -> G5Loads {
    let rs = rs_v5();
    let ld = &rs.loads;
    let ml = Midline::of(b);
    let len = ml.length();
    let secs = sections(p, b, &ml, 0.5, len, 1.0);
    let at_or_after = |s0: f64| -> Vec<BladeSection> { secs.iter().copied().filter(|s| s.s_mm >= s0).collect() };
    let heel = ml.at(len).0;

    // Lateral: a force normal to the blade 5 mm behind the point, sized to the limit root moment
    let (q5, _) = ml.at(ld.lateral_bend.apply_mm_from_point);
    let arm = (q5.0 - heel.0).hypot(q5.1 - heel.1).max(1.0);
    let f_lat = ld.lateral_bend.limit_root_moment_nm * 1000.0 / arm;
    let f_lat_u = ld.lateral_bend.ultimate_root_moment_nm * 1000.0 / arm;
    let blade_lat = at_or_after(ld.lateral_bend.apply_mm_from_point + 3.0);
    let lateral_limit_sf = worst_sf(p, &blade_lat, &|s| section_forces(s, q5, (0.0, 0.0, f_lat)), false);
    let lateral_ultimate_sf = worst_sf(p, &blade_lat, &|s| section_forces(s, q5, (0.0, 0.0, f_lat_u)), true);

    // Body twist about the blade axis, applied over the working span 25-75 % of the blade (from the point): full
    // torque from 75 % to the heel, ramping to zero at 25 %
    let t_lim = ld.body_twist.limit_nm * 1000.0;
    let twist_at = |s: f64| t_lim * ((s / len - 0.25) / 0.5).clamp(0.0, 1.0);
    let twist_limit_sf = worst_sf(p, &at_or_after(0.25 * len), &|s| (0.0, 0.0, 0.0, twist_at(s.s_mm)), false);

    // The point
    let (tip, tau_tip) = ml.at(0.0);
    let f_ax = ld.tip.axial_n;
    let tip_axial_sf = worst_sf(
        p,
        &secs.iter().copied().filter(|s| s.s_mm >= ld.tip.axial_no_yield_from_mm && s.s_mm <= 10.0).collect::<Vec<_>>(),
        &|s| section_forces(s, tip, (f_ax * tau_tip.0, f_ax * tau_tip.1, 0.0)),
        false,
    );
    let tip_lat_secs: Vec<BladeSection> =
        ld.tip.lateral_check_stations_mm.iter().filter_map(|&s| blade_section(p, b, &ml, s)).collect();
    let tip_lateral_sf = worst_sf(p, &tip_lat_secs, &|s| section_forces(s, tip, (0.0, 0.0, ld.tip.lateral_n)), false);
    let t_tip = ld.tip.twist_nm * 1000.0;
    let s_tw = ld.tip.twist_apply_mm_from_point;
    let tip_twist_sf = worst_sf(
        p,
        &secs.iter().copied().filter(|s| s.s_mm >= s_tw && s.s_mm <= s_tw + 12.0).collect::<Vec<_>>(),
        &|_| (0.0, 0.0, 0.0, t_tip),
        false,
    );

    // Pull: the blade held over its last 20 mm, the hand pulling along the grip axis (-x); the held region pushes +x
    let (qh, _) = ml.at(0.5 * ld.pull.held_length_from_point_mm);
    let pull_sf = worst_sf(
        p,
        &at_or_after(ld.pull.held_length_from_point_mm),
        &|s| section_forces(s, qh, (ld.pull.limit_n, 0.0, 0.0)),
        false,
    );

    // Buckling of the point under the tip axial load
    let e_mod = material::e_mpa();
    let ei = |s: f64| {
        blade_section(p, b, &ml, s.max(0.25))
            .map(|sec| e_mod * sec.i_minor_mm4.min(sec.i_major_mm4))
            .unwrap_or(1e-6)
    };
    let p_cr = tip_buckling_load_n(&ei, 0.5, 30.0_f64.min(len - 1.0));
    let buckling_lambda = p_cr / ld.tip.axial_n;

    // Fatigue: fully reversed root moment and twist duties ([fatigue]), on the blade's surface class
    let fat = &rs.fatigue;
    // The outer faces of the blade are ground; in a hollow blade the cavity faces stay as-built, at
    // (t - 2 skin) / t of the outer-fibre stress
    let fatigue_sf = blade_lat
        .iter()
        .map(|sec| {
            let k_b = fat.blade_root_moment_amplitude_nm / ld.lateral_bend.limit_root_moment_nm;
            let (n, mb, mip, tq) = section_forces(sec, q5, (0.0, 0.0, f_lat * k_b));
            let s_bend = sec.sigma_eq(n, mb, mip, tq);
            let s_tw = sec.sigma_eq(0.0, 0.0, 0.0, twist_at(sec.s_mm) * fat.twist_amplitude_nm / ld.body_twist.limit_nm);
            let s_a = s_bend.max(s_tw);
            let outer = material::fatigue_sf(s_a, 0.0, material::FatigueClass::Ground);
            let inner = if sec.hollow {
                let k_in = (1.0 - 2.0 * p["skin_t"] / sec.t_max_mm.max(1e-6)).max(0.0);
                let (t, w) = sec.wall(p);
                material::fatigue_sf(s_a * k_in, 0.0, material::fatigue_class(t, w))
            } else {
                f64::INFINITY
            };
            outer.min(inner) / fat.sf_fatigue
        })
        .fold(f64::INFINITY, f64::min);

    // Hand side: grip stations 25 / 50 / 75 % of the grip, as-built walls
    let skin = p["skin_t"];
    let a_grip = material::yield_allowable(skin, AS_BUILT_WALL);
    let mut grip_lateral_sf = f64::INFINITY;
    let mut grip_twist_sf = f64::INFINITY;
    let mut grip_pull_sf = f64::INFINITY;
    let mut grip_squeeze_sf = f64::INFINITY;
    let mut hand_fatigue_sf = f64::INFINITY;
    let fat_hand = material::fatigue_strength_mpa(material::fatigue_class(skin, AS_BUILT_WALL));
    for s_frac in [0.25, 0.50, 0.75] {
        let x = p["x_t"] + s_frac * p["L_g"];
        let g = grip_section(p, x);
        // lateral: lever from the load point to the station along the grip axis (+x), torsion from the offset
        let (rx, ry) = (q5.0 - x, q5.1 - g.ym);
        let s_lat = f_lat * rx.abs() * 0.5 * g.t / g.i_lat.max(1.0);
        let tau_lat = g.tau(f_lat * ry.abs(), skin);
        let vm_lat = (s_lat * s_lat + 3.0 * tau_lat * tau_lat).sqrt();
        grip_lateral_sf = grip_lateral_sf.min(a_grip / vm_lat.max(1e-9));
        grip_twist_sf = grip_twist_sf.min(a_grip / (3.0_f64.sqrt() * g.tau(t_lim, skin)).max(1e-9));
        // pull: axial plus in-plane bending from the held point's offset from the grip axis
        let s_pull = ld.pull.limit_n / g.area.max(1.0)
            + ld.pull.limit_n * (qh.1 - g.ym).abs() * 0.5 * g.h / g.i_inplane.max(1.0);
        grip_pull_sf = grip_pull_sf.min(a_grip / s_pull.max(1e-9));
        // squeeze: the limit squeeze's normal sum spread over four 20 mm finger pads (G3 panel/ovalisation model)
        if g.hollow {
            let f_pad = ld.grip.normal_sum_factor * ld.grip.limit_squeeze_n / 4.0;
            let q = f_pad / (20.0 * (0.5 * g.h.min(30.0)).max(5.0));
            let panel = 0.75 * q * p["lat_cell"].powi(2) / (skin * skin);
            let r_m = 0.125 * (g.h + g.t);
            let oval = 6.0 * 0.318 * (f_pad / 20.0) * r_m / (skin * skin);
            grip_squeeze_sf = grip_squeeze_sf.min(a_grip / panel.max(oval).max(1e-9));
        }
        let k_b = fat.blade_root_moment_amplitude_nm / ld.lateral_bend.limit_root_moment_nm;
        let k_t = fat.twist_amplitude_nm / ld.body_twist.limit_nm;
        let s_fat = (vm_lat * k_b).max(3.0_f64.sqrt() * g.tau(t_lim * k_t, skin));
        hand_fatigue_sf = hand_fatigue_sf.min(fat_hand / s_fat.max(1e-9) / fat.sf_fatigue);
    }

    // Guard: the hand sliding forward onto the guard lobe, bending its root
    let guard_push_sf = guard_push_sf(p);

    let ring = &ld.ring;
    let ring_bearing_sf = ring_bearing_cap_n / ring.bearing_n;
    let ring_back_sf = ring_back_cap_n / ring.back_n;
    // Tier 0: the bore bearing toward the handle stands in for the any-direction case (Tier 1 sweeps it)
    let ring_any_direction_sf = ring_bearing_cap_n / ring.any_direction_n;

    let hand_lat = grip_lateral_sf;
    let hand_twist = grip_twist_sf;
    let hand_pull = grip_pull_sf.min(ring_any_direction_sf);
    G5Loads {
        blade_model: BladeModel::Analytic,
        lateral_force_n: f_lat,
        lateral_arm_mm: arm,
        lateral_limit_sf,
        lateral_ultimate_sf,
        twist_limit_sf,
        tip_axial_sf,
        tip_lateral_sf,
        tip_twist_sf,
        pull_sf,
        buckling_lambda,
        teeth_sf: f64::NAN,
        fatigue_sf,
        grip_lateral_sf,
        grip_twist_sf,
        grip_pull_sf,
        grip_squeeze_sf,
        guard_push_sf,
        ring_bearing_sf,
        ring_back_sf,
        ring_any_direction_sf,
        hand_fatigue_sf,
        seq_lateral: hand_lat / lateral_limit_sf.max(1e-9),
        seq_twist: hand_twist / twist_limit_sf.max(1e-9),
        seq_pull: hand_pull / pull_sf.max(1e-9),
        lateral_peak_at: (f64::NAN, f64::NAN),
        twist_peak_at: (f64::NAN, f64::NAN),
    }
}

/// Fenestration windows of the blade as closed polygons: the zero contour of the decoder's window margin
/// (`eval_blade_thickness_sdf2_window`), by marching squares on a 0.2 mm raster. The Tier-1 plate mesh conforms to
/// them; with windows classified element by element, the window edges became mesh-dependent staircases.
pub fn window_holes(p: &Params, b: &BladeProfile2D) -> Vec<Vec<(f64, f64)>> {
    use std::collections::HashMap;
    if p["win_amp"] <= 0.35 {
        return Vec::new();
    }
    let h = 0.2_f64;
    let (x0, x1) = (b.x_heel, b.x_tip);
    let y0 = b.y_in.iter().copied().fold(f64::INFINITY, f64::min) - 1.0;
    let y1 = b.y_sp.iter().copied().fold(f64::NEG_INFINITY, f64::max) + 1.0;
    let (nx, ny) = (((x1 - x0) / h) as usize + 2, ((y1 - y0) / h) as usize + 2);
    let mut phi = vec![-1.0_f64; nx * ny];
    let mut any = false;
    for i in 0..nx {
        for j in 0..ny {
            let (x, y) = (x0 + i as f64 * h, y0 + j as f64 * h);
            let m = crate::g5_sdf::eval_blade_thickness_sdf2_window(p, b, x, y).2;
            if m.is_finite() {
                phi[i * ny + j] = m.clamp(-1.0, 1.0);
                any |= m > 0.0;
            }
        }
    }
    if !any {
        return Vec::new();
    }
    let v = |i: usize, j: usize| phi[i * ny + j];
    // Edge ids: horizontal (i, j)-(i+1, j) -> 2 (i ny + j); vertical (i, j)-(i, j+1) -> 2 (i ny + j) + 1
    let pt_on = |e: usize| -> (f64, f64) {
        let (k, vert) = (e / 2, e % 2 == 1);
        let (i, j) = (k / ny, k % ny);
        let (a, c, (xa, ya), (xc, yc)) = if vert {
            (v(i, j), v(i, j + 1), (i, j), (i, j + 1))
        } else {
            (v(i, j), v(i + 1, j), (i, j), (i + 1, j))
        };
        let f = (a / (a - c)).clamp(0.0, 1.0);
        let (xa, ya, xc, yc) = (xa as f64, ya as f64, xc as f64, yc as f64);
        (x0 + h * (xa + f * (xc - xa)), y0 + h * (ya + f * (yc - ya)))
    };
    let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..nx - 1 {
        for j in 0..ny - 1 {
            let c = [v(i, j), v(i + 1, j), v(i + 1, j + 1), v(i, j + 1)];
            // cell edges: bottom, right, top, left
            let e = [2 * (i * ny + j), 2 * ((i + 1) * ny + j) + 1, 2 * (i * ny + j + 1), 2 * (i * ny + j) + 1];
            let mut cross: Vec<usize> = Vec::new();
            for k in 0..4 {
                let (a, d) = (c[k], c[(k + 1) % 4]);
                if (a > 0.0) != (d > 0.0) {
                    cross.push(e[k]);
                }
            }
            let mut link = |a: usize, d: usize| {
                adj.entry(a).or_default().push(d);
                adj.entry(d).or_default().push(a);
            };
            match cross.len() {
                2 => link(cross[0], cross[1]),
                4 => {
                    // saddle: decide by the cell-centre value
                    let centre = 0.25 * c.iter().sum::<f64>();
                    if (centre > 0.0) == (c[0] > 0.0) {
                        link(cross[0], cross[3]);
                        link(cross[1], cross[2]);
                    } else {
                        link(cross[0], cross[1]);
                        link(cross[2], cross[3]);
                    }
                }
                _ => {}
            }
        }
    }
    let mut seen = std::collections::HashSet::new();
    let mut loops = Vec::new();
    let mut keys: Vec<usize> = adj.keys().copied().collect();
    keys.sort_unstable();
    for start in keys {
        if seen.contains(&start) {
            continue;
        }
        let mut lp = vec![start];
        seen.insert(start);
        let (mut prev, mut cur) = (usize::MAX, start);
        loop {
            let next = adj[&cur].iter().copied().find(|&n| n != prev && !seen.contains(&n));
            match next {
                Some(n) => {
                    seen.insert(n);
                    lp.push(n);
                    prev = cur;
                    cur = n;
                }
                None => break,
            }
        }
        if lp.len() < 8 {
            continue;
        }
        let poly: Vec<(f64, f64)> = lp.iter().step_by(2).map(|&e| pt_on(e)).collect();
        let area = 0.5
            * poly
                .iter()
                .zip(poly.iter().cycle().skip(1))
                .map(|(a, c)| a.0 * c.1 - c.0 * a.1)
                .sum::<f64>()
                .abs();
        if area >= 1.0 {
            loops.push(poly);
        }
    }
    loops
}

/// Element area of the Tier-1 blade plate (mm²). G3 used every fourth outline station and 3.5 mm², which did not
/// converge (audit `plate_meshconv.json`: 337 / 191 / 254 / 103 N).
pub const PLATE_MAX_AREA_MM2: f64 = 0.3;

/// Tier-1 blade cases on a CST+DKT sandwich plate of the whole blade (full serrated outline), clamped at the heel.
/// Replaces the analytic lateral, ultimate, twist, pull and fatigue values in `loads` and recomputes the sequence
/// ratios. Returns false, leaving `loads` unchanged, if the mesh or a checked solve fails.
pub fn plate_blade_cases(p: &Params, b: &BladeProfile2D, loads: &mut G5Loads) -> bool {
    plate_blade_cases_at(p, b, loads, PLATE_MAX_AREA_MM2)
}

/// `plate_blade_cases` on a mesh of the given maximum element area (mesh-convergence studies).
pub fn plate_blade_cases_at(p: &Params, b: &BladeProfile2D, loads: &mut G5Loads, max_area_mm2: f64) -> bool {
    use crate::g5_sandwich::{SandwichPlate, SandwichSection};
    let rs = rs_v5();
    let ld = &rs.loads;
    let n_st = b.xs.len();
    let mut outer: Vec<(f64, f64)> = (0..n_st).map(|i| (b.xs[i], b.y_in[i])).collect();
    outer.extend((0..n_st).rev().map(|i| (b.xs[i], b.y_sp[i])));
    let holes = window_holes(p, b);
    let Some(mesh) = jarambit_physics::fine::mesh_domain(&outer, &holes, max_area_mm2) else {
        return false;
    };
    let ml = Midline::of(b);
    let len = ml.length();
    let rho_b = effective_blade_rho(p);
    let core_e = material::core_relative_stiffness(rho_b, p["lat_cell"], p["lat_mix"]);
    let skin = p["skin_t"];
    let core_x_max = b.x_tip - p["tip_reserve"].max(6.0) - 4.0;
    // Midline samples every 0.25 mm and section half-widths every 1 mm, computed once
    let samples: Vec<(f64, (f64, f64), (f64, f64))> = (0..=((len / 0.25) as usize))
        .map(|k| {
            let s = k as f64 * 0.25;
            let (m, t) = ml.at(s);
            (s, m, t)
        })
        .collect();
    let half_w: Vec<f64> = (0..=(len as usize))
        .map(|k| blade_section(p, b, &ml, k as f64).map(|sec| 0.5 * sec.width_mm).unwrap_or(0.0))
        .collect();
    let half_width_at = |s: f64| half_w[(s.round() as usize).min(half_w.len() - 1)];
    // midline arc position and signed offset across the blade of a point
    let locate = |x: f64, y: f64| -> (f64, f64) {
        let mut best = (f64::INFINITY, 0.0, 0.0);
        for &(s, m, t) in &samples {
            let d2 = (x - m.0).powi(2) + (y - m.1).powi(2);
            if d2 < best.0 {
                best = (d2, s, (x - m.0) * -t.1 + (y - m.1) * t.0);
            }
        }
        (best.1, best.2)
    };
    let tris = mesh.tris.clone();
    let xy = mesh.xy.clone();
    let mut elem = Vec::with_capacity(tris.len());
    let mut secs = Vec::with_capacity(tris.len());
    for t in &tris {
        let xm = (xy[t[0]].0 + xy[t[1]].0 + xy[t[2]].0) / 3.0;
        let ym = (xy[t[0]].1 + xy[t[1]].1 + xy[t[2]].1) / 3.0;
        let (th, d2) = eval_blade_thickness_and_sdf2(p, b, xm, ym);
        let (s_e, eta) = locate(xm, ym);
        // a fenestration window: a hole (near-zero stiffness), excluded from the stress read-out
        let window = (d2 > 0.0 || th <= 0.0) && inside_outline(b, xm, ym);
        let hollow =
            rho_b < 0.95 && half_width_at(s_e) - eta.abs() > 4.2 && xm <= core_x_max && th > 2.0 * skin + 1.0;
        secs.push(if window {
            SandwichSection::solid(0.05)
        } else {
            SandwichSection::from_skin_and_lattice(th.max(0.3), skin, if hollow { core_e } else { 1.0 }, 0.0)
        });
        let (tw, wall) = if hollow { (skin, HOLLOW_BLADE_SKIN) } else { (th.max(0.3), GROUND_WALL) };
        let (a_y, a_u) = if window {
            (f64::INFINITY, f64::INFINITY)
        } else {
            (material::yield_allowable(tw, wall), material::ultimate_allowable(tw, wall))
        };
        elem.push((xm, ym, s_e, hollow, th, a_y, a_u));
    }
    let plate = SandwichPlate::new(mesh, p["t_core"], material::e_mpa(), material::nu()).with_sections(secs);
    let n = plate.n();
    let mut fixed = vec![false; n * 5];
    let mut n_fix = 0usize;
    for (i, &(x, _)) in plate.mesh.xy.iter().enumerate() {
        if x <= b.x_heel + 3.5 {
            n_fix += 1;
            for c in 0..5 {
                fixed[i * 5 + c] = true;
            }
        }
    }
    if n_fix < 3 {
        return false;
    }
    let Some(sys) = plate.system(&fixed) else {
        return false;
    };
    let node_pos: Vec<(f64, f64)> = plate.mesh.xy.iter().map(|&(x, y)| locate(x, y)).collect();

    // 1. Lateral: the limit force normal to the blade on a 2 mm patch around the point 5 mm from the tip
    let (q5, _) = ml.at(ld.lateral_bend.apply_mm_from_point);
    let heel = ml.at(len).0;
    let f_lat = ld.lateral_bend.limit_root_moment_nm * 1000.0 / (q5.0 - heel.0).hypot(q5.1 - heel.1).max(1.0);
    let patch: Vec<usize> =
        (0..n).filter(|&i| (plate.mesh.xy[i].0 - q5.0).hypot(plate.mesh.xy[i].1 - q5.1) <= 2.0).collect();
    // 2. Body twist: a couple linear across the blade over the 25-75 % span, unit torque about the midline, on the
    //    blade body (3.5 mm clear of each edge, so no concentrated force lands on the thin bevels)
    let span: Vec<usize> = (0..n)
        .filter(|&i| {
            let (s_i, eta_i) = node_pos[i];
            s_i >= 0.25 * len && s_i <= 0.75 * len && !fixed[i * 5] && eta_i.abs() <= half_width_at(s_i) - 3.5
        })
        .collect();
    let eta2: f64 = span.iter().map(|&i| node_pos[i].1.powi(2)).sum();
    // 3. Pull: the held 20 mm pushed +x (the hand pulls the knife -x)
    let held: Vec<usize> = (0..n).filter(|&i| node_pos[i].0 <= ld.pull.held_length_from_point_mm).collect();
    if patch.is_empty() || span.is_empty() || eta2 <= 1e-9 || held.is_empty() {
        return false;
    }
    let mut f1 = vec![0.0; n * 5];
    for &i in &patch {
        f1[i * 5 + 2] = f_lat / patch.len() as f64;
    }
    let t_lim = ld.body_twist.limit_nm * 1000.0;
    let mut f2 = vec![0.0; n * 5];
    for &i in &span {
        f2[i * 5 + 2] = t_lim * node_pos[i].1 / eta2;
    }
    let mut f3 = vec![0.0; n * 5];
    for &i in &held {
        f3[i * 5] = ld.pull.limit_n / held.len() as f64;
    }
    let solve = |f: &[f64]| -> Option<Vec<f64>> {
        let (u, _) = plate.solve_checked(&sys, f)?;
        let (sm, sb) = plate.stresses(&u);
        Some(sm.iter().zip(sb.iter()).map(|(a, c)| a + c).collect())
    };
    let (Some(s1), Some(s2), Some(s3)) = (solve(&f1), solve(&f2), solve(&f3)) else {
        return false;
    };
    let clamp_ok = |e: usize| elem[e].0 >= b.x_heel + 4.0;
    let worst_at = |sig: &[f64], keep: &dyn Fn(usize) -> bool, ult: bool, k: f64| -> (f64, usize) {
        (0..sig.len())
            .filter(|&e| clamp_ok(e) && keep(e))
            .map(|e| ((if ult { elem[e].6 } else { elem[e].5 }) / (k * sig[e]).max(1e-9), e))
            .fold((f64::INFINITY, 0), |a, c| if c.0 < a.0 { c } else { a })
    };
    let worst = |sig: &[f64], keep: &dyn Fn(usize) -> bool, ult: bool, k: f64| worst_at(sig, keep, ult, k).0;
    let away_from_patch = |e: usize| (elem[e].0 - q5.0).hypot(elem[e].1 - q5.1) >= 4.0;
    let k_u = ld.lateral_bend.ultimate_root_moment_nm / ld.lateral_bend.limit_root_moment_nm;
    let (lat_sf, e_lat) = worst_at(&s1, &away_from_patch, false, 1.0);
    loads.lateral_limit_sf = lat_sf;
    loads.lateral_peak_at = locate(elem[e_lat].0, elem[e_lat].1);
    loads.lateral_ultimate_sf = worst(&s1, &away_from_patch, true, k_u);
    let (tw_sf, e_tw) = worst_at(&s2, &|_| true, false, 1.0);
    loads.twist_limit_sf = tw_sf;
    loads.twist_peak_at = locate(elem[e_tw].0, elem[e_tw].1);
    loads.pull_sf = worst(&s3, &|e| elem[e].2 >= ld.pull.held_length_from_point_mm + 2.0, false, 1.0);
    // Fatigue duties on the plate: ground outer faces, as-built cavity faces of a hollow blade
    let fat = &rs.fatigue;
    let k_b = fat.blade_root_moment_amplitude_nm / ld.lateral_bend.limit_root_moment_nm;
    let k_t = fat.twist_amplitude_nm / ld.body_twist.limit_nm;
    let mut fat_sf = f64::INFINITY;
    for e in 0..s1.len() {
        if !clamp_ok(e) || !away_from_patch(e) || !elem[e].5.is_finite() {
            continue;
        }
        let s_a = (k_b * s1[e]).max(k_t * s2[e]);
        let mut sf = material::fatigue_sf(s_a, 0.0, material::FatigueClass::Ground);
        if elem[e].3 {
            let k_in = (1.0 - 2.0 * skin / elem[e].4.max(1e-6)).max(0.0);
            let class = material::fatigue_class(skin, HOLLOW_BLADE_SKIN);
            sf = sf.min(material::fatigue_sf(s_a * k_in, 0.0, class));
        }
        fat_sf = fat_sf.min(sf / fat.sf_fatigue);
    }
    loads.fatigue_sf = fat_sf;
    loads.blade_model = BladeModel::Plate;
    loads.seq_lateral = loads.grip_lateral_sf / loads.lateral_limit_sf.max(1e-9);
    loads.seq_twist = loads.grip_twist_sf / loads.twist_limit_sf.max(1e-9);
    loads.seq_pull = loads.grip_pull_sf.min(loads.ring_any_direction_sf) / loads.pull_sf.max(1e-9);
    true
}

/// Guard push ([loads.guard]): the lobe below the grip as a cantilever from the grip underside, loaded in +x at 60 %
/// of its height; the root section is the lobe's x-extent where it is at least 40 % deep, `t_guard` thick.
pub fn guard_push_sf(p: &Params) -> f64 {
    let h_g = p["h_g"];
    let t_g = p["t_guard"];
    let yb_ref = eval_grip_slice(p, GUARD_X_MM - 6.0).yb;
    let mut x_lo = f64::INFINITY;
    let mut x_hi = f64::NEG_INFINITY;
    let mut x = 67.6;
    while x <= 99.0 {
        let (yb, _) = eval_guard_lobe_y(p, x);
        if yb_ref - yb >= 0.4 * h_g {
            x_lo = x_lo.min(x);
            x_hi = x_hi.max(x);
        }
        x += 0.25;
    }
    let depth = if x_hi > x_lo { x_hi - x_lo } else { 0.5 * h_g };
    let rho = p["rho_guard"];
    let skin = p["skin_t"];
    let rho_e = material::core_relative_stiffness(rho, p["lat_cell"], p["lat_mix"]);
    let (ti, di) = if rho < 0.95 { ((t_g - 2.0 * skin).max(0.0), (depth - 2.0 * skin).max(0.0)) } else { (0.0, 0.0) };
    let i = (t_g * depth.powi(3) - (1.0 - rho_e) * ti * di.powi(3)) / 12.0;
    let m = rs_v5().loads.guard.limit_push_n * 0.6 * h_g;
    let sigma = m * 0.5 * depth / i.max(1e-6);
    material::yield_allowable(if rho < 0.95 { skin } else { t_g }, AS_BUILT_WALL) / sigma.max(1e-9)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ritz_column_reproduces_the_euler_cantilever() {
        // Uniform cantilever: P_cr = pi^2 EI / (4 L^2)
        let (ei, l) = (2.0e6_f64, 25.0_f64);
        let p = tip_buckling_load_n(&|_| ei, 0.0, l);
        let exact = std::f64::consts::PI.powi(2) * ei / (4.0 * l * l);
        assert!((p / exact - 1.0).abs() < 1e-3, "{p} vs {exact}");
    }

    #[test]
    fn section_forces_split_a_lateral_force_into_bending_and_torsion() {
        let sec = BladeSection {
            s_mm: 10.0,
            centroid: (0.0, 0.0),
            tangent: (1.0, 0.0),
            width_mm: 10.0,
            t_max_mm: 2.0,
            area_mm2: 15.0,
            i_minor_mm4: 5.0,
            i_major_mm4: 100.0,
            c_major_mm: 5.0,
            j_mm4: 20.0,
            hollow: false,
        };
        // A force normal to the blade 10 mm back along the axis: pure bending 10 N x 10 mm
        let (n, mb, mip, t) = section_forces(&sec, (-10.0, 0.0), (0.0, 0.0, 10.0));
        assert!(n.abs() < 1e-12 && mip.abs() < 1e-12 && t.abs() < 1e-12 && (mb - 100.0).abs() < 1e-9);
        // Offset 3 mm across the blade: adds torsion 30 N mm
        let (_, mb2, _, t2) = section_forces(&sec, (-10.0, 3.0), (0.0, 0.0, 10.0));
        assert!((mb2 - 100.0).abs() < 1e-9 && (t2.abs() - 30.0).abs() < 1e-9);
        // An in-plane pull along +x at the axis is pure tension here (q pulled away from the heel side at +x)
        let (n3, _, mip3, _) = section_forces(&sec, (-10.0, 0.0), (-50.0, 0.0, 0.0));
        assert!((n3 - 50.0).abs() < 1e-9 && mip3.abs() < 1e-12);
    }

    #[test]
    fn plate_cases_converge_and_windows_are_mesh_holes() {
        // A design with fenestration windows (sobol 3) and one without (sobol 11)
        for i in [3usize, 11] {
            let g = crate::g5_genome::sobol_unit_genome(i, 1, "conv");
            let p = g.decode();
            let b = crate::g5_sdf::build_blade_profile_2d(&p);
            let base = analytic_loads(&p, &b, 900.0, 120.0);
            let (mut a, mut c) = (base, base);
            assert!(plate_blade_cases_at(&p, &b, &mut a, PLATE_MAX_AREA_MM2));
            assert!(plate_blade_cases_at(&p, &b, &mut c, 0.5 * PLATE_MAX_AREA_MM2));
            for (name, x, y) in [("lateral", a.lateral_limit_sf, c.lateral_limit_sf), ("twist", a.twist_limit_sf, c.twist_limit_sf)] {
                assert!((x / y - 1.0).abs() < 0.10, "sobol {i} {name}: {x} at {} mm2 vs {y} at half", PLATE_MAX_AREA_MM2);
            }
            let holes = window_holes(&p, &b);
            assert_eq!(holes.is_empty(), p["win_amp"] <= 0.35 || holes.is_empty());
            for h in &holes {
                assert!(h.iter().all(|&(x, y)| inside_outline(&b, x, y)), "sobol {i}: a window hole leaves the outline");
            }
        }
    }

    #[test]
    fn analytic_loads_are_finite_and_scale_with_the_requirement() {
        let g = crate::g5_genome::sobol_unit_genome(3, 1, "loads");
        let p = g.decode();
        let b = crate::g5_sdf::build_blade_profile_2d(&p);
        let l = analytic_loads(&p, &b, 900.0, 120.0);
        for (name, v) in [
            ("lateral", l.lateral_limit_sf),
            ("ultimate", l.lateral_ultimate_sf),
            ("twist", l.twist_limit_sf),
            ("tip axial", l.tip_axial_sf),
            ("tip lateral", l.tip_lateral_sf),
            ("tip twist", l.tip_twist_sf),
            ("pull", l.pull_sf),
            ("buckling", l.buckling_lambda),
            ("fatigue", l.fatigue_sf),
            ("grip lateral", l.grip_lateral_sf),
            ("grip twist", l.grip_twist_sf),
            ("guard", l.guard_push_sf),
        ] {
            assert!(v.is_finite() && v > 0.0, "{name}: {v}");
        }
        // The lateral force is the root moment over its arm
        assert!((l.lateral_force_n * l.lateral_arm_mm - 12_000.0).abs() < 1e-6);
        // Ring loads use the owner's values
        assert!((l.ring_bearing_sf - 3.0).abs() < 1e-12 && (l.ring_back_sf - 4.0).abs() < 1e-12);
        println!("{l:?}");
        // JSON round trip is exact, non-finite values included
        let mut lnf = l;
        lnf.teeth_sf = f64::INFINITY;
        let back: G5Loads = serde_json::from_str(&serde_json::to_string(&lnf).unwrap()).unwrap();
        assert_eq!(serde_json::to_string(&back).unwrap(), serde_json::to_string(&lnf).unwrap());
        assert!(back.teeth_sf.is_infinite() && back.lateral_peak_at.0.is_nan());
        // Tier 1: the plate replaces the blade cases and keeps them finite
        let mut l1 = l;
        let t0 = std::time::Instant::now();
        assert!(plate_blade_cases(&p, &b, &mut l1), "plate solve failed");
        println!("plate {:.2} s: {l1:?}", t0.elapsed().as_secs_f64());
        assert_eq!(l1.blade_model, BladeModel::Plate);
        for v in [l1.lateral_limit_sf, l1.lateral_ultimate_sf, l1.twist_limit_sf, l1.pull_sf, l1.fatigue_sf] {
            assert!(v.is_finite() && v > 0.0);
        }
    }
}
