//! `W_ERGOLIGHT_v2`: the G5 objective, from RS-karambit-v5 `[objective]` (owner rules: continuous, no hard gates,
//! ergonomics never traded for grams).
//!
//! score = 100 · E^k · W · (w_mass · s_mass + Σ_i w_i · s_i)
//!
//! - **Structural terms.** One per load case, with v = SF / SF_req (`loads::G5Loads`, `[margins]`), scored by
//!   s(v) = 0.94 · v⁴ below 1 and 0.94 + 0.06 · (1 − e^(−(v − 1)/τ)) above. A failed SF earns little; G3's
//!   `sfk` gave 94 % credit at v → 1⁻ and 0.94·v below.
//! - **Worst margin.** μ = min_i v_i over the structural terms. It multiplies the score,
//!   W = min(1, s(μ)/0.94)^worst_margin_exponent, so comfort and mass cannot buy back a failed load case. The G5
//!   pilot (2026-10-27) found that with an additive 0.25 term, a design failing five cases scored within 12 % of one
//!   meeting all of them.
//! - **Mass.** s_mass = (ceiling − m)/(ceiling − floor), linear down to the floor.
//! - **Geometry terms**, measured on the decoded design:
//!   - edge: angle and apex flat in their bands;
//!   - point: the owner's blunt-point penalty and the tip thickness caps;
//!   - serration: depth and teeth on both edges;
//!   - AM build: sealed, undrainable volume;
//!   - form.
//! - **Ergonomics.** E = Π e_j^β_j over the RS-v5 betas, raised to `e_exponent` (5). By the owner rule, 1 % of E
//!   costs at least 10 g of mass credit (`requirements::tests`).

use crate::ergolight::{band, c01, ErgolightInputs, ErgolightScore};
use crate::g5_qd::PointPlanform;
use crate::g5_sdf::{AmConnectivityStats, BladeProfile2D};
use crate::loads::G5Loads;
use crate::requirements::rs_v5;

pub const METRIC: &str = "W_ERGOLIGHT_v2";
pub const DRAFT: &str = "rs-v5";

/// Structural credit of a margin v = SF / SF_req (`[objective]` below_one_*, knee_*). NaN or negative earns 0.
pub fn s_margin(v: f64) -> f64 {
    let o = &rs_v5().objective;
    if !v.is_finite() || v <= 0.0 {
        return if v == f64::INFINITY { o.below_one_scale + o.knee_bonus } else { 0.0 };
    }
    if v < 1.0 {
        o.below_one_scale * v.powf(o.below_one_exponent)
    } else {
        o.below_one_scale + o.knee_bonus * (1.0 - (-(v - 1.0) / o.knee_tau).exp())
    }
}

/// The worst-margin multiplier W = min(1, s(μ)/below_one_scale)^worst_margin_exponent: 1 when every load case meets its
/// requirement, μ⁴ below.
pub fn worst_margin_multiplier(mu: f64) -> f64 {
    let o = &rs_v5().objective;
    (s_margin(mu) / o.below_one_scale).min(1.0).powf(o.worst_margin_exponent)
}

/// One-sided upper band: 1 at or below `hi`, falling linearly to 0 at `hi + fall`.
fn at_most(v: f64, hi: f64, fall: f64) -> f64 {
    band(v, f64::NEG_INFINITY, hi, fall, 0.0)
}

/// Everything the objective reads that is not in `ErgolightInputs`.
pub struct ObjectiveInputs<'a> {
    pub inputs: &'a ErgolightInputs,
    pub loads: &'a G5Loads,
    pub point: &'a PointPlanform,
    pub am: &'a AmConnectivityStats,
    pub blade: &'a BladeProfile2D,
}

/// The structural margins v = SF / SF_req, in `[objective.weights]` order (names match the weight keys).
pub fn margins(l: &G5Loads) -> [(&'static str, f64); 16] {
    let m = &rs_v5().margins;
    let (y, u, r) = (m.sf_yield, m.sf_ultimate, m.sf_ring);
    let lat = l.lateral_limit_sf.min(l.grip_lateral_sf);
    let tw = l.twist_limit_sf.min(l.grip_twist_sf);
    let pull = l.pull_sf.min(l.grip_pull_sf);
    let fat = l.fatigue_sf.min(l.hand_fatigue_sf);
    let seq = rs_v5().sequence.ratio_min;
    [
        ("lateral_limit", lat / y),
        ("lateral_ultimate", l.lateral_ultimate_sf / u),
        ("twist_limit", tw / y),
        ("tip_axial", l.tip_axial_sf / y),
        ("tip_lateral", l.tip_lateral_sf / y),
        ("tip_twist", l.tip_twist_sf / y),
        ("grip_squeeze", l.grip_squeeze_sf / y),
        ("guard_push", l.guard_push_sf / y),
        ("ring_bearing", l.ring_bearing_sf / r),
        ("ring_back", l.ring_back_sf / r),
        ("ring_any_direction", l.ring_any_direction_sf / y),
        ("pull", pull / y),
        ("teeth", l.teeth_sf / y),
        ("fatigue", fat / rs_v5().fatigue.sf_fatigue),
        ("sequence", l.seq_min() / seq),
        ("buckling", l.buckling_lambda / rs_v5().buckling.lambda_min),
    ]
}

/// Geometry terms: (edge, point, serration, am_build, form).
pub fn geometry_terms(x: &ObjectiveInputs) -> [(&'static str, f64); 5] {
    let rs = rs_v5();
    let m = x.inputs;
    let ed = &rs.edge;
    let s_edge = (band(m.max_half_angle_deg, ed.per_side_angle_min_deg, ed.per_side_angle_max_deg, 3.0, 0.0)
        * band(m.max_apex_mm, ed.finished_apex_flat_min_mm, ed.finished_apex_flat_max_mm, 0.08, 0.0))
    .sqrt();
    let pt = &rs.point;
    let point_factors = [
        at_most(x.point.angle_1mm_deg, pt.angle_max_at_1mm_deg, 30.0),
        at_most(x.point.angle_3mm_deg, pt.angle_max_at_3mm_deg, 30.0),
        at_most(x.point.width_2mm, pt.width_max_at_2mm, 2.0),
        at_most(m.t_0p25_mm, pt.thickness_max_at_025mm, 0.3),
        at_most(m.t_1p00_mm, pt.thickness_max_at_1mm, 0.4),
        at_most(m.t_2p00_mm, pt.thickness_max_at_2mm, 0.5),
        if m.point_monotonic { 1.0 } else { 0.5 },
    ];
    let s_point = point_factors.iter().product::<f64>().powf(1.0 / point_factors.len() as f64);
    let sr = &rs.serration;
    let edge_serr = |depth: f64, teeth: usize| {
        band(depth, sr.depth_min_mm, sr.depth_max_mm, 0.4, 0.0)
            * (teeth as f64 / sr.full_depth_scallops_min as f64).min(1.0)
    };
    let s_serr = 0.5
        * (edge_serr(x.blade.serr_depth_in_mm, x.blade.serr_teeth_in)
            + edge_serr(x.blade.serr_depth_sp_mm, x.blade.serr_teeth_sp));
    // Closed voids are not allowed ([am] closed_voids_allowed = false): 300 mm3 sealed (1.3 g of powder) costs e^-1.
    // Unmeasured (Tier 0) earns nothing.
    let s_am = if x.am.measured { (-x.am.sealed_void_mm3.max(0.0) / 300.0).exp() } else { 0.0 };
    let s_form = 0.5 * (if m.karambit_ok { 1.0 } else { 0.5 }) + 0.5 * c01(m.fairness);
    [("edge", s_edge), ("point", s_point), ("serration", s_serr), ("am_build", s_am), ("form", s_form)]
}

/// Ergonomic factors e_j in [0.3, 1], in `[objective.betas]` order.
pub fn ergo_factors(m: &ErgolightInputs) -> [(&'static str, f64); 8] {
    let rs = rs_v5();
    let g = &rs.ergonomics.grip;
    let e_grip_section = (band(m.swell_perimeter_mm, g.perimeter_min_mm, g.perimeter_max_mm, g.perimeter_min_mm - g.perimeter_floor_mm, 0.3)
        * band(m.swell_h_over_t, g.height_over_thickness_min, g.height_over_thickness_max, 0.3, 0.3))
    .sqrt();
    let e_ring = ((m.lip_w / rs.ergonomics.ring.lip_w_mm).min(1.0)
        * band(m.bore_round, 2.5, 3.5, 1.0, 0.3)
        * band(m.crown, 0.2, 0.4, 0.3, 0.3)
        * band(m.junction_step_mm, 0.0, 0.3, 1.7, 0.3))
    .sqrt();
    let ed = &rs.ergonomics.edges;
    let e_hand_edges = (band(m.edge_r_min_mm, ed.radius_min_thick_section_mm, 99.0, ed.radius_min_thick_section_mm - ed.radius_min_mm, 0.3)
        * band(m.section_corner_r_min_mm, ed.radius_min_thick_section_mm, 99.0, 1.5, 0.3))
    .sqrt();
    // Contact pressure: the grip force times the normal-sum factor over the hand's contact patch, taken as the front
    // half of the swell perimeter along 60 % of the grip (a first-order hand model)
    let ld = &rs.loads.grip;
    let pr = &rs.ergonomics.pressure;
    let area = (0.6 * m.l_g_mm.max(1.0) * 0.5 * m.swell_perimeter_mm.max(1.0)).max(1.0);
    let kpa = |f: f64| f * ld.normal_sum_factor / area * 1000.0;
    let e_contact = (band(kpa(ld.working_typical_n), 0.0, pr.all_day_kpa_at_typical, 0.5 * pr.all_day_kpa_at_typical, 0.3)
        * band(kpa(ld.working_peak_n), 0.0, pr.discomfort_finger_kpa, 0.5 * pr.discomfort_finger_kpa, 0.3)
        * band(kpa(ld.p95_male_grip_n), 0.0, pr.pain_finger_kpa, 0.5 * pr.pain_finger_kpa, 0.3))
    .powf(1.0 / 3.0);
    let gd = &rs.ergonomics.guard;
    let e_guard = band(m.guard_h_mm, gd.height_target_mm, 99.0, gd.height_target_mm - gd.height_floor_mm, 0.3);
    let e_grip_length = band(m.l_g_mm, g.length_min_mm, 999.0, 5.0, 0.3);
    let bl = &rs.ergonomics.balance;
    let e_balance = (band(m.com_to_index_mm, 0.0, bl.com_to_index_max_mm, 12.0, 0.3)
        * band(m.spin_i_g_mm2 / m.spin_i_ref_g_mm2.max(1e-9), 0.0, bl.spin_inertia_ratio_max, 0.6, 0.3))
    .sqrt();
    let e_grip_surface =
        band(m.tex_amp_mm, 0.15, 0.25, 0.15, 0.5) * band(m.tex_crest_r_mm, 0.15, 99.0, 0.15, 0.5);
    let clamp_e = |v: f64| if v.is_nan() { 0.3 } else { v.clamp(0.3, 1.0) };
    [
        ("grip_section", clamp_e(e_grip_section)),
        ("ring", clamp_e(e_ring)),
        ("hand_edges", clamp_e(e_hand_edges)),
        ("contact_pressure", clamp_e(e_contact)),
        ("guard", clamp_e(e_guard)),
        ("grip_length", clamp_e(e_grip_length)),
        ("balance", clamp_e(e_balance)),
        ("grip_surface", clamp_e(e_grip_surface)),
    ]
}

/// Evaluate `W_ERGOLIGHT_v2`.
pub fn score_v2(x: &ObjectiveInputs) -> ErgolightScore {
    let rs = rs_v5();
    let o = &rs.objective;
    let betas = o.betas.entries();
    let e_vec = ergo_factors(x.inputs);
    let mut e_geo = 1.0_f64;
    let mut ergo_out = Vec::with_capacity(betas.len());
    for (&(k, beta), &(name, e)) in betas.iter().zip(e_vec.iter()) {
        debug_assert_eq!(k, name);
        e_geo *= e.powf(beta);
        ergo_out.push((k, beta, e));
    }
    let e_mult = e_geo.powf(o.e_exponent);

    let v = margins(x.loads);
    let mu = v.iter().map(|&(_, vi)| if vi.is_nan() { 0.0 } else { vi }).fold(f64::INFINITY, f64::min);
    let geo = geometry_terms(x);
    let s_of = |key: &str| -> f64 {
        v.iter()
            .find(|(k, _)| *k == key)
            .map(|&(_, vi)| s_margin(vi))
            .or_else(|| geo.iter().find(|(k, _)| *k == key).map(|&(_, s)| c01(s)))
            .unwrap_or(0.0)
    };
    let mass = &rs.mass;
    let s_mass = c01((mass.ceiling_g - x.inputs.mass_g) / (mass.ceiling_g - mass.floor_g));
    let w_worst = worst_margin_multiplier(mu);
    let k = e_mult * w_worst;
    // "worst_margin" carries the multiplier W itself (weight 0: it scales every term, it is not added)
    let mut terms_out = vec![
        ("mass", o.mass_weight, s_mass, 100.0 * k * o.mass_weight * s_mass),
        ("worst_margin", 0.0, w_worst, 0.0),
    ];
    let mut base = o.mass_weight * s_mass;
    for (key, w) in o.weights.entries() {
        let s = s_of(key);
        base += w * s;
        terms_out.push((key, w, s, 100.0 * k * w * s));
    }
    let raw_score_100 = 100.0 * k * base;
    ErgolightScore {
        metric: METRIC,
        draft: DRAFT,
        score_100: (raw_score_100 * 1e6).round() / 1e6,
        raw_score_100,
        ergo_multiplier: e_mult,
        base,
        terms: terms_out,
        ergo: ergo_out,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn margin_credit_is_steep_below_one_and_continuous_at_one() {
        assert!((s_margin(1.0) - 0.94).abs() < 1e-12);
        assert!((s_margin(1.0 - 1e-9) - 0.94).abs() < 1e-6);
        assert!((s_margin(0.8) - 0.94 * 0.8_f64.powi(4)).abs() < 1e-12);
        assert!(s_margin(0.64) < 0.16, "SF 0.64 of its requirement earns {}", s_margin(0.64));
        assert!(s_margin(3.0) < 1.0 && s_margin(3.0) > 0.99);
        assert_eq!(s_margin(f64::NAN), 0.0);
        assert_eq!(s_margin(-1.0), 0.0);
    }

    #[test]
    fn raising_a_failing_safety_factor_outscores_the_mass_it_costs() {
        // Plan rule: a single failing SF raised to its requirement is worth more than the mass it costs. A single
        // failing SF is the worst margin, so it lifts the multiplier W from W(v0) to 1. Even at a low base (0.6),
        // that gain is worth more than 10 g of mass credit (relative gain in score vs relative value of a gram).
        let o = &rs_v5().objective;
        let m = &rs_v5().mass;
        let base = 0.6;
        let per_gram_rel = o.mass_weight / (m.ceiling_g - m.floor_g) / base;
        for v0 in [0.5_f64, 0.7, 0.85, 0.95] {
            let rel_gain = 1.0 / worst_margin_multiplier(v0) - 1.0;
            assert!(rel_gain > 10.0 * per_gram_rel, "v {v0}: {rel_gain} vs 10 g = {}", 10.0 * per_gram_rel);
        }
        assert_eq!(worst_margin_multiplier(1.0), 1.0);
        assert_eq!(worst_margin_multiplier(2.0), 1.0);
    }

    #[test]
    fn tier0_scores_with_v2_and_every_weight_has_a_term() {
        let g = crate::g5_genome::sobol_unit_genome(4, 2, "objective");
        let rec = crate::g5_qd::evaluate_tier0_analytic(&g);
        let s = &rec.score;
        assert_eq!(s.metric, METRIC);
        assert!(s.raw_score_100.is_finite() && s.raw_score_100 > 0.0 && s.raw_score_100 < 100.0, "{s:?}");
        assert_eq!(s.terms.len(), 2 + rs_v5().objective.weights.entries().len());
        let wsum: f64 = s.terms.iter().map(|t| t.1).sum();
        assert!((wsum - 1.0).abs() < 1e-9);
        assert_eq!(s.ergo.len(), 8);
    }
}
