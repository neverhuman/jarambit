//! The hand-zone load cases G5.1 scores differently from G5.0: grip squeeze, guard push and the serration teeth.
//!
//! Every evaluation records, per case, G5.0's analytic safety factor beside the one G5.1 scores, and which model
//! produced it. At Tiers 0 and 1 the block is a function of the genome alone, so an archive row's block is recomputed
//! and checked bit for bit when the row is read back.

use jarambit_g5::g5_qd::serration_tooth_strength;
use jarambit_g5::g5_sdf::BladeProfile2D;
use jarambit_g5::loads::{guard_push_sf, G5Loads};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

type Params = BTreeMap<&'static str, f64>;

/// Which model produced the safety factors a record scores.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandModel {
    /// G5.0's analytic models, unchanged: development builds only, before the calibration exists. A frozen G5.1
    /// refuses it.
    G50Analytic,
}

/// One case: G5.0's analytic SF and the SF G5.1 scores.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HandCase {
    #[serde(with = "crate::f64x")]
    pub g50: f64,
    #[serde(with = "crate::f64x")]
    pub scored: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HandZone {
    pub model: HandModel,
    pub grip_squeeze: HandCase,
    pub guard_push: HandCase,
    pub teeth: HandCase,
}

impl HandZone {
    /// Write the scored SFs into a record's load cases.
    pub fn apply(&self, loads: &mut G5Loads) {
        loads.grip_squeeze_sf = self.grip_squeeze.scored;
        loads.guard_push_sf = self.guard_push.scored;
        loads.teeth_sf = self.teeth.scored;
    }
}

/// G5.0's three analytic SFs, exactly as its Tier 0 computes them (`analytic_loads`, `guard_push_sf`, the minimum of
/// `serration_tooth_strength` over both edges).
pub fn g50_analytic(p: &Params, blade: &BladeProfile2D, loads_g50: &G5Loads) -> (f64, f64, f64) {
    let teeth = [true, false].iter().map(|&inner| serration_tooth_strength(p, blade, inner).0).fold(f64::INFINITY, f64::min);
    debug_assert_eq!(guard_push_sf(p).to_bits(), loads_g50.guard_push_sf.to_bits());
    (loads_g50.grip_squeeze_sf, loads_g50.guard_push_sf, teeth)
}

/// The Tier-0/1 hand-zone block of a design.
pub fn hand_zone_analytic(p: &Params, blade: &BladeProfile2D, loads_g50: &G5Loads) -> HandZone {
    let (grip, guard, teeth) = g50_analytic(p, blade, loads_g50);
    let case = |g50: f64| HandCase { g50, scored: g50 };
    HandZone { model: HandModel::G50Analytic, grip_squeeze: case(grip), guard_push: case(guard), teeth: case(teeth) }
}

/// One serration tooth of the census: its gullets and crest (indices into the blade profile), height, and the
/// notched-root SFs of the two load cases (`serration_tooth_strength`'s loop, copied, which keeps only the minimum).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Tooth {
    pub inner: bool,
    pub gullet_a: usize,
    pub gullet_b: usize,
    pub crest: usize,
    pub height_mm: f64,
    /// Root length between the gullets (mm), gullet radius (mm) and notch factor
    pub root_mm: f64,
    pub gullet_r_mm: f64,
    pub kt: f64,
    #[serde(with = "crate::f64x")]
    pub snag_sf: f64,
    #[serde(with = "crate::f64x")]
    pub pry_sf: f64,
    /// G5.0's tooth SF, `allow / max(sigma_snag, sigma_pry)`
    #[serde(with = "crate::f64x")]
    pub sf: f64,
}

/// Every tooth on one edge, as `jarambit_g5::g5_qd::serration_tooth_strength` walks them (its minimum of `sf` over
/// the census is that function's SF, bit for bit; `tests::hand`). Empty when the edge is not serrated.
pub fn teeth_census(p: &Params, blade: &BladeProfile2D, inner: bool) -> Vec<Tooth> {
    use jarambit_g5::g5_qd::{tooth_pry_per_tooth_n, tooth_snag_n};
    use jarambit_g5::g5_sdf::{eval_blade_thickness_and_sdf2, serration_gullets};
    use jarambit_g5::material::{self, GROUND_WALL};
    let (edge, smooth, sign) = if inner { (&blade.y_in, &blade.y_in_smooth, 1.0) } else { (&blade.y_sp, &blade.y_sp_smooth, -1.0) };
    let xs = &blade.xs;
    let n = xs.len();
    let (depth, gullets) = serration_gullets(edge, smooth);
    let mut out = Vec::new();
    if depth < 0.2 {
        return out;
    }
    let allow = material::yield_allowable(1.0, GROUND_WALL);
    let at = |x: f64, y: f64| eval_blade_thickness_and_sdf2(p, blade, x, y);
    let interp = |v: &[f64], x: f64| -> f64 {
        let i = xs.partition_point(|&q| q < x).clamp(1, n - 1);
        let f = ((x - xs[i - 1]) / (xs[i] - xs[i - 1]).max(1e-12)).clamp(0.0, 1.0);
        v[i - 1] * (1.0 - f) + v[i] * f
    };
    let mut stations = gullets.clone();
    if jarambit_g5::requirements::rs_v5().serration.check_end_teeth && gullets.len() >= 2 {
        let span = (gullets[gullets.len() - 1] - gullets[0]) / (gullets.len() - 1);
        stations.insert(0, gullets[0].saturating_sub(span));
        stations.push((gullets[gullets.len() - 1] + span).min(n - 1));
    }
    let (snag_n, pry_n) = (tooth_snag_n(), tooth_pry_per_tooth_n());
    for w in stations.windows(2) {
        let (ga, gb) = (w[0], w[1]);
        let (pa, pb) = ((xs[ga], edge[ga]), (xs[gb], edge[gb]));
        let b = (pb.0 - pa.0).hypot(pb.1 - pa.1);
        let (ux, uy) = ((pb.0 - pa.0) / b.max(1e-9), (pb.1 - pa.1) / b.max(1e-9));
        let (mut h_t, mut crest) = (0.0_f64, ga);
        for k in ga..=gb {
            let d = ((xs[k] - pa.0) * -uy + (edge[k] - pa.1) * ux).abs();
            if d > h_t {
                crest = k;
            }
            h_t = h_t.max(d);
        }
        if b < 0.3 || h_t < 0.2 {
            continue;
        }
        let (nx, ny) = (-uy * sign, ux * sign);
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
        let tooth = |snag_sf: f64, pry_sf: f64, sf: f64, gullet_r_mm: f64, kt: f64| Tooth {
            inner,
            gullet_a: ga,
            gullet_b: gb,
            crest,
            height_mm: h_t,
            root_mm: b,
            gullet_r_mm,
            kt,
            snag_sf,
            pry_sf,
            sf,
        };
        if a0 <= 1e-9 || i_op <= 1e-12 {
            out.push(tooth(0.0, 0.0, 0.0, f64::NAN, f64::NAN));
            continue;
        }
        let s_bar = a1 / a0;
        let i_ip = (a2 - a0 * s_bar * s_bar).max(1e-12);
        let c = s_bar.max(b - s_bar);
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
        out.push(tooth(
            allow / sig_snag.max(1e-9),
            allow / sig_pry.max(1e-9),
            allow / sig_snag.max(sig_pry).max(1e-9),
            rho,
            kt,
        ));
    }
    out
}
