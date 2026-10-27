//! `W_ERGOLIGHT_v1` (draft-d, RS-karambit-v3) — veto-free continuous multi-objective evaluator
//! for the G5 fresh evolutionary cohort (fixed blade, sheath-free, hollow-capable L-PBF Ti-6Al-4V).
//!
//! ```text
//! score_100 = 100 * E * sum_i w_i * s_i          (sum_i w_i = 1)
//! E         = prod_k e_k ** beta_k,  e_k in [0.3, 1]   (sum_k beta_k = 1)
//! ```
//!
//! Ergonomics `E` is a multiplicative factor rather than an additive budget: a design cannot buy
//! back comfort with grams, because every structural and mass point is scaled by `E`. Inside the
//! target ergonomic bands `e_k = 1`, so `E` never steers an already-comfortable design; mass then
//! carries the largest single weight (`0.37`, linear down to `25.0 g`). No term is a hard gate or
//! veto. Every safety-factor term uses `sfk(v)`: 94 % for reaching first-yield `SF = 1.0` plus a
//! 6 % exponential safety knee (`1 - exp(-max(0, v - 1) / 0.15)`) to reward margin against mesh
//! and manufacturing variance.
//!
//! Shares no name prefix with `W_ULTRALIGHT_v1`, which remains frozen for published v1/v2 papers.

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

pub const METRIC: &str = "W_ERGOLIGHT_v1";
pub const DRAFT: &str = "d";
pub const REQUIREMENT_SET: &str = "RS-karambit-v3";

/// Ordered `(key, weight)` table for `W_ERGOLIGHT_v1` draft-d (`sum = 1.000`).
/// Owner 2026-10-25 (draft d): `solid_tip_3d` 0.030 -> 0.070; the +0.040 is taken pro-rata (x0.870968, 4 dp)
/// from the 12 other structural terms, so mass, the Gate-S, build and form terms and every `BETAS` entry are unchanged.
/// The pilot's double-edged points reached a 3D solid-tip SF of only 0.11-0.51 under the 1.82 N*m tip pry.
/// Owner 2026-10-25 (~02:15Z): 300 N bearing on bore inside closest to handle (+/-60 deg about theta=0,
/// pushing +x) at SF >= 2 (600 N capacity), 30 N ring-back (+x on outer back) at SF >= 2 (60 N capacity).
/// Sheath/holder terms (`holder_and_compactness`) are omitted per owner directive (2026-10-25T00:09Z).
pub const WEIGHTS: &[(&str, f64)] = &[
    ("mass", 0.370),
    ("hiker_lateral_180n", 0.0305),
    ("hunter_lateral_260n", 0.0305),
    ("strict_hunter_260n", 0.0174),
    ("ring_bearing_300n_sf2", 0.0218),
    ("ring_back_30n_sf2", 0.0261),
    ("twist_body_1p82nm", 0.0218),
    ("twist_tip_1p82nm", 0.0218),
    ("solid_tip_3d", 0.070),
    ("side_a4_buckling", 0.0174),
    ("failure_sequence_1p2", 0.0305),
    ("plate_gci_3level", 0.020),
    ("human_max_twist_spine", 0.0174),
    ("goodman_fatigue_l7", 0.0174),
    ("gate_s_sharpness", 0.050),
    ("gate_s_point_profile", 0.050),
    ("hollow_grip_structure", 0.0174),
    ("cad_watertight", 0.040),
    ("karambit_ring_and_claw", 0.040),
    ("form_fairness", 0.040),
    ("am_build", 0.050),
];

/// Ordered `(key, beta)` table for the geometric-mean ergonomics multiplier `E` (`sum = 1.00`).
pub const BETAS: &[(&str, f64)] = &[
    ("ring_lip", 0.18),
    ("grip_section", 0.18),
    ("hand_edges", 0.18),
    ("finger_index", 0.15),
    ("grip_length", 0.08),
    ("guard", 0.08),
    ("grip_surface", 0.05),
    ("balance", 0.10),
];

/// Clamp to `[0.0, 1.0]`.
#[inline]
pub fn c01(v: f64) -> f64 {
    if v.is_nan() {
        0.0
    } else {
        v.clamp(0.0, 1.0)
    }
}

/// Plateau-band response: `1.0` inside `[lo, hi]`, linear ramp down to `floor` over `fall` outside.
#[inline]
pub fn band(v: f64, lo: f64, hi: f64, fall: f64, floor: f64) -> f64 {
    if !v.is_finite() {
        return floor;
    }
    if v < lo {
        floor.max(1.0 - (1.0 - floor) * (lo - v) / fall)
    } else if v > hi {
        floor.max(1.0 - (1.0 - floor) * (v - hi) / fall)
    } else {
        1.0
    }
}

/// Safety-factor response with a 6 % safety knee above `SF = 1.0`:
/// `0.94 * clamp01(v) + 0.06 * (1 - exp(-max(0, v - 1) / 0.15))`.
#[inline]
pub fn sfk(v: f64) -> f64 {
    if !v.is_finite() {
        return 0.0;
    }
    0.94 * c01(v) + 0.06 * (1.0 - (-(v - 1.0).max(0.0) / 0.15).exp())
}

#[inline]
fn round_n(v: f64, digits: i32) -> f64 {
    let p = 10_f64.powi(digits);
    (v * p).round() / p
}

fn ser_f64_inf<S: serde::Serializer>(v: &f64, s: S) -> Result<S::Ok, S::Error> {
    if v.is_infinite() && v.is_sign_positive() {
        s.serialize_str("inf")
    } else if v.is_infinite() && v.is_sign_negative() {
        s.serialize_str("-inf")
    } else {
        s.serialize_f64(*v)
    }
}

fn de_f64_inf<'de, D: serde::Deserializer<'de>>(d: D) -> Result<f64, D::Error> {
    let val = Value::deserialize(d)?;
    if let Some(f) = val.as_f64() {
        Ok(f)
    } else if val.as_str() == Some("inf") {
        Ok(f64::INFINITY)
    } else if val.as_str() == Some("-inf") {
        Ok(f64::NEG_INFINITY)
    } else {
        Err(serde::de::Error::custom("expected f64 or \"inf\""))
    }
}

/// Complete strongly-typed measurement vector for `W_ERGOLIGHT_v1` (real solver keys, no silent defaults).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErgolightInputs {
    // Mass & structural capacities / safety factors
    pub mass_g: f64,
    pub lat_rob_cert_n: f64,
    pub strict_lat_n: f64,
    pub ring_bearing_cap_n: f64,
    pub ring_back_cap_n: f64,
    pub twist_body_sf: f64,
    pub twist_tip_sf: f64,
    pub twist_tipzone_strict_sf: f64,
    pub solid_tip_sf: f64,
    pub dolfinx_rel_err: f64,
    pub buckling_lambda: f64,
    pub seq_ratio: f64,
    pub gci_admitted: bool,
    pub hm_twist_sf: f64,
    pub hm_spine_sf: f64,
    pub fat_sf: f64,
    #[serde(serialize_with = "ser_f64_inf", deserialize_with = "de_f64_inf")]
    pub max_apex_mm: f64,
    #[serde(serialize_with = "ser_f64_inf", deserialize_with = "de_f64_inf")]
    pub max_half_angle_deg: f64,
    #[serde(serialize_with = "ser_f64_inf", deserialize_with = "de_f64_inf")]
    pub t_0p25_mm: f64,
    #[serde(serialize_with = "ser_f64_inf", deserialize_with = "de_f64_inf")]
    pub t_1p00_mm: f64,
    #[serde(serialize_with = "ser_f64_inf", deserialize_with = "de_f64_inf")]
    pub t_2p00_mm: f64,
    pub point_monotonic: bool,
    pub grip_sf_min: f64,
    pub watertight: bool,
    pub karambit_ok: bool,
    pub fairness: f64,
    pub am_build: f64,

    // Ergonomic & dynamic balance measurements
    pub lip_w: f64,
    pub bore_round: f64,
    pub crown: f64,
    pub junction_step_mm: f64,
    pub swell_perimeter_mm: f64,
    pub swell_h_over_t: f64,
    pub edge_r_min_mm: f64,
    pub section_corner_r_min_mm: f64,
    pub groove_r_min_mm: f64,
    pub crest_r_min_mm: f64,
    pub saddle_depth_mm: f64,
    pub choil_depth_mm: f64,
    pub mid_depth_mm: f64,
    pub station_err_mm: f64,
    #[serde(
        rename = "L_g_mm",
        alias = "l_g_mm",
        serialize_with = "ser_f64_inf",
        deserialize_with = "de_f64_inf"
    )]
    pub l_g_mm: f64,
    pub guard_h_mm: f64,
    pub tex_amp_mm: f64,
    pub tex_crest_r_mm: f64,
    pub com_to_index_mm: f64,
    #[serde(rename = "spin_I_g_mm2", alias = "spin_i_g_mm2")]
    pub spin_i_g_mm2: f64,
    #[serde(rename = "spin_I_ref_g_mm2", alias = "spin_i_ref_g_mm2")]
    pub spin_i_ref_g_mm2: f64,
}

/// Hand-zone ergonomic and structural measurements to combine with blade/plate receipts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandErgoMetrics {
    pub mass_g: f64,
    pub ring_bearing_cap_n: f64,
    pub ring_back_cap_n: f64,
    pub grip_sf_min: f64,
    pub watertight: bool,
    pub karambit_ok: bool,
    pub fairness: f64,
    pub am_build: f64,
    pub lip_w: f64,
    pub bore_round: f64,
    pub crown: f64,
    pub junction_step_mm: f64,
    pub swell_perimeter_mm: f64,
    pub swell_h_over_t: f64,
    pub edge_r_min_mm: f64,
    pub section_corner_r_min_mm: f64,
    pub groove_r_min_mm: f64,
    pub crest_r_min_mm: f64,
    pub saddle_depth_mm: f64,
    pub choil_depth_mm: f64,
    pub mid_depth_mm: f64,
    pub station_err_mm: f64,
    pub l_g_mm: f64,
    pub guard_h_mm: f64,
    pub tex_amp_mm: f64,
    pub tex_crest_r_mm: f64,
    pub com_to_index_mm: f64,
    pub spin_i_g_mm2: f64,
    pub spin_i_ref_g_mm2: f64,
}

/// Computed `W_ERGOLIGHT_v1` breakdown.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErgolightScore {
    pub metric: &'static str,
    pub draft: &'static str,
    pub score_100: f64,
    pub raw_score_100: f64,
    #[serde(rename = "E")]
    pub ergo_multiplier: f64,
    pub base: f64,
    pub terms: Vec<(&'static str, f64, f64, f64)>, // (key, weight, s_i, pts)
    pub ergo: Vec<(&'static str, f64, f64)>,       // (key, beta, e_k)
}

impl ErgolightScore {
    /// Convert to the exact JSON structure returned by `tools/ergolight.py::score(m)`.
    pub fn to_json(&self) -> Value {
        let mut terms_map = Map::new();
        for &(k, w, s, pts) in &self.terms {
            terms_map.insert(k.to_string(), json!({ "w": w, "s": s, "pts": pts }));
        }
        let mut ergo_map = Map::new();
        for &(k, beta, e) in &self.ergo {
            ergo_map.insert(k.to_string(), json!({ "beta": beta, "e": e }));
        }
        json!({
            "metric": self.metric,
            "draft": self.draft,
            "score_100": self.score_100,
            "E": self.ergo_multiplier,
            "base": self.base,
            "terms": terms_map,
            "ergo": ergo_map,
        })
    }
}

/// Compute normalized structural/build subscores `s_i` (`ergolight.py::terms_from(m)` parity; draft d changes weights only).
pub fn terms_from(m: &ErgolightInputs) -> Vec<(&'static str, f64)> {
    let s_mass = c01((130.0 - m.mass_g) / 105.0);
    let s_hiker = sfk(m.lat_rob_cert_n / 180.0);
    let s_hunter = sfk(m.lat_rob_cert_n / 260.0);
    let s_strict = sfk(m.strict_lat_n / 260.0);
    // Owner 2026-10-25 (~02:15Z): 300 N bearing on bore inside closest to handle (SF 2 => 600 N), 30 N ring-back (SF 2 => 60 N)
    let s_ring_bearing = sfk(m.ring_bearing_cap_n / 600.0);
    let s_ring_back = sfk(m.ring_back_cap_n / 60.0);
    let s_tw_body = sfk(m.twist_body_sf);
    let s_tw_tip = sfk(0.5 * m.twist_tip_sf + 0.5 * m.twist_tipzone_strict_sf);
    let dx_bonus = if m.dolfinx_rel_err.is_finite() && m.dolfinx_rel_err <= 0.02 {
        1.0
    } else {
        0.0
    };
    let s_solid_tip = sfk(m.solid_tip_sf) * (0.8 + 0.2 * dx_bonus);
    let s_buckling = sfk(m.buckling_lambda);
    let s_seq = sfk(m.seq_ratio / 1.2);
    let s_gci = if m.gci_admitted { 1.0 } else { 0.5 };
    let s_hm = 0.5 * sfk(m.hm_twist_sf) + 0.5 * sfk(m.hm_spine_sf);
    let s_fat = sfk(m.fat_sf);
    let s_sharp = c01(
        0.5 * (0.20 / m.max_apex_mm.max(1e-9)).min(1.0)
            + 0.5 * (25.0 / m.max_half_angle_deg.max(1e-9)).min(1.0),
    );
    let s_point = c01(
        0.25 * ((0.30 / m.t_0p25_mm.max(1e-9)).min(1.0)
            + (0.60 / m.t_1p00_mm.max(1e-9)).min(1.0)
            + (1.00 / m.t_2p00_mm.max(1e-9)).min(1.0)
            + if m.point_monotonic { 1.0 } else { 0.0 }),
    );
    let s_grip = sfk(m.grip_sf_min);
    let s_wt = if m.watertight { 1.0 } else { 0.0 };
    let s_kar = if m.karambit_ok { 1.0 } else { 0.5 };
    let s_fair = c01(m.fairness);
    let s_am = c01(m.am_build);

    vec![
        ("mass", s_mass),
        ("hiker_lateral_180n", s_hiker),
        ("hunter_lateral_260n", s_hunter),
        ("strict_hunter_260n", s_strict),
        ("ring_bearing_300n_sf2", s_ring_bearing),
        ("ring_back_30n_sf2", s_ring_back),
        ("twist_body_1p82nm", s_tw_body),
        ("twist_tip_1p82nm", s_tw_tip),
        ("solid_tip_3d", s_solid_tip),
        ("side_a4_buckling", s_buckling),
        ("failure_sequence_1p2", s_seq),
        ("plate_gci_3level", s_gci),
        ("human_max_twist_spine", s_hm),
        ("goodman_fatigue_l7", s_fat),
        ("gate_s_sharpness", s_sharp),
        ("gate_s_point_profile", s_point),
        ("hollow_grip_structure", s_grip),
        ("cad_watertight", s_wt),
        ("karambit_ring_and_claw", s_kar),
        ("form_fairness", s_fair),
        ("am_build", s_am),
    ]
}

/// Compute ergonomic sub-factors `e_k in [0.3, 1.0]` (`ergolight.py::ergo_from(m)` parity).
pub fn ergo_from(m: &ErgolightInputs) -> Vec<(&'static str, f64)> {
    let e_ring_lip = ((m.lip_w / 8.0).min(1.0)
        * band(m.bore_round, 2.5, 3.5, 1.0, 0.3)
        * band(m.crown, 0.2, 0.4, 0.3, 0.3)
        * band(m.junction_step_mm, 0.0, 0.3, 1.7, 0.3))
    .sqrt();

    let e_grip_section = (band(m.swell_perimeter_mm, 65.0, 75.0, 10.0, 0.3)
        * band(m.swell_h_over_t, 1.5, 2.0, 0.4, 0.3))
    .sqrt();

    let e_hand_edges = (band(m.edge_r_min_mm, 2.2, 99.0, 1.2, 0.3)
        * band(m.section_corner_r_min_mm, 4.0, 99.0, 3.0, 0.3))
    .sqrt();

    let e_finger_index = (band(m.groove_r_min_mm, 10.0, 999.0, 6.0, 0.3)
        * band(m.crest_r_min_mm, 5.0, 999.0, 3.0, 0.3)
        * band(m.saddle_depth_mm, 1.5, 2.5, 1.5, 0.3)
        * band(m.choil_depth_mm, 3.5, 4.5, 1.5, 0.3)
        * band(m.mid_depth_mm, 1.5, 2.5, 1.5, 0.3)
        * band(m.station_err_mm, 0.0, 2.5, 4.0, 0.3))
    .powf(1.0 / 6.0);

    let e_grip_length = band(m.l_g_mm, 64.0, 72.0, 6.0, 0.3);
    let e_guard = band(m.guard_h_mm, 12.0, 99.0, 6.0, 0.3);
    let e_grip_surface = band(m.tex_amp_mm, 0.15, 0.25, 0.15, 0.5)
        * band(m.tex_crest_r_mm, 0.15, 99.0, 0.15, 0.5);
    let e_balance = (band(m.com_to_index_mm, 0.0, 8.0, 12.0, 0.3)
        * band(
            m.spin_i_g_mm2 / m.spin_i_ref_g_mm2.max(1e-9),
            0.0,
            0.85,
            0.6,
            0.3,
        ))
    .sqrt();

    let clamp_e = |v: f64| {
        if v.is_nan() {
            0.3
        } else {
            v.clamp(0.3, 1.0)
        }
    };

    vec![
        ("ring_lip", clamp_e(e_ring_lip)),
        ("grip_section", clamp_e(e_grip_section)),
        ("hand_edges", clamp_e(e_hand_edges)),
        ("finger_index", clamp_e(e_finger_index)),
        ("grip_length", clamp_e(e_grip_length)),
        ("guard", clamp_e(e_guard)),
        ("grip_surface", clamp_e(e_grip_surface)),
        ("balance", clamp_e(e_balance)),
    ]
}

/// Evaluate `W_ERGOLIGHT_v1` (draft-d) from strongly typed inputs.
pub fn score_ergolight(m: &ErgolightInputs) -> ErgolightScore {
    let s_vec = terms_from(m);
    let e_vec = ergo_from(m);

    let mut e_mult = 1.0_f64;
    let mut ergo_out = Vec::with_capacity(BETAS.len());
    for (&(k, beta), &(ek_name, ek_val)) in BETAS.iter().zip(e_vec.iter()) {
        debug_assert_eq!(k, ek_name);
        e_mult *= ek_val.powf(beta);
        ergo_out.push((k, beta, ek_val));
    }

    let mut base = 0.0_f64;
    let mut terms_out = Vec::with_capacity(WEIGHTS.len());
    for (&(k, w), &(sk_name, sk_val)) in WEIGHTS.iter().zip(s_vec.iter()) {
        debug_assert_eq!(k, sk_name);
        base += w * sk_val;
        terms_out.push((k, w, sk_val, 100.0 * e_mult * w * sk_val));
    }

    let raw_score_100 = 100.0 * e_mult * base;
    ErgolightScore {
        metric: METRIC,
        draft: DRAFT,
        score_100: round_n(raw_score_100, 6),
        raw_score_100,
        ergo_multiplier: e_mult,
        base,
        terms: terms_out,
        ergo: ergo_out,
    }
}

/// Parse a JSON measurement dictionary (matching `tools/ergolight.py`'s `m` dict) and compute `W_ERGOLIGHT_v1`.
/// Fails closed if any required solver or ergonomic key is missing or non-finite.
pub fn score_ergolight_map(m: &Map<String, Value>) -> Result<Value, String> {
    let req_f64 = |k: &str| -> Result<f64, String> {
        let v = m
            .get(k)
            .and_then(|v| v.as_f64())
            .ok_or_else(|| format!("missing or non-numeric required key '{k}'"))?;
        if !v.is_finite() {
            return Err(format!("non-finite value for required key '{k}'"));
        }
        Ok(v)
    };
    let req_f64_or_inf_sentinel = |k: &str| -> Result<f64, String> {
        let val = m
            .get(k)
            .ok_or_else(|| format!("missing required key '{k}'"))?;
        if val.as_str() == Some("inf") {
            return Ok(f64::INFINITY);
        }
        let v = val
            .as_f64()
            .ok_or_else(|| format!("non-numeric required key '{k}'"))?;
        if !v.is_finite() {
            return Err(format!("non-finite value for required key '{k}'"));
        }
        Ok(v)
    };
    let req_bool = |k: &str| -> Result<bool, String> {
        m.get(k)
            .and_then(|v| v.as_bool())
            .ok_or_else(|| format!("missing or non-boolean required key '{k}'"))
    };

    let inputs = ErgolightInputs {
        mass_g: req_f64("mass_g")?,
        lat_rob_cert_n: req_f64("lat_rob_cert_n")?,
        strict_lat_n: req_f64("strict_lat_n")?,
        ring_bearing_cap_n: req_f64("ring_bearing_cap_n")?,
        ring_back_cap_n: req_f64("ring_back_cap_n")?,
        twist_body_sf: req_f64("twist_body_sf")?,
        twist_tip_sf: req_f64("twist_tip_sf")?,
        twist_tipzone_strict_sf: req_f64("twist_tipzone_strict_sf")?,
        solid_tip_sf: req_f64("solid_tip_sf")?,
        dolfinx_rel_err: req_f64("dolfinx_rel_err")?,
        buckling_lambda: req_f64("buckling_lambda")?,
        seq_ratio: req_f64("seq_ratio")?,
        gci_admitted: req_bool("gci_admitted")?,
        hm_twist_sf: req_f64("hm_twist_sf")?,
        hm_spine_sf: req_f64("hm_spine_sf")?,
        fat_sf: req_f64("fat_sf")?,
        max_apex_mm: req_f64_or_inf_sentinel("max_apex_mm")?,
        max_half_angle_deg: req_f64_or_inf_sentinel("max_half_angle_deg")?,
        t_0p25_mm: req_f64_or_inf_sentinel("t_0p25_mm")?,
        t_1p00_mm: req_f64_or_inf_sentinel("t_1p00_mm")?,
        t_2p00_mm: req_f64_or_inf_sentinel("t_2p00_mm")?,
        point_monotonic: req_bool("point_monotonic")?,
        grip_sf_min: req_f64("grip_sf_min")?,
        watertight: req_bool("watertight")?,
        karambit_ok: req_bool("karambit_ok")?,
        fairness: req_f64("fairness")?,
        am_build: req_f64("am_build")?,
        lip_w: req_f64("lip_w")?,
        bore_round: req_f64("bore_round")?,
        crown: req_f64("crown")?,
        junction_step_mm: req_f64("junction_step_mm")?,
        swell_perimeter_mm: req_f64("swell_perimeter_mm")?,
        swell_h_over_t: req_f64("swell_h_over_t")?,
        edge_r_min_mm: req_f64("edge_r_min_mm")?,
        section_corner_r_min_mm: req_f64("section_corner_r_min_mm")?,
        groove_r_min_mm: req_f64("groove_r_min_mm")?,
        crest_r_min_mm: req_f64("crest_r_min_mm")?,
        saddle_depth_mm: req_f64("saddle_depth_mm")?,
        choil_depth_mm: req_f64("choil_depth_mm")?,
        mid_depth_mm: req_f64("mid_depth_mm")?,
        station_err_mm: req_f64("station_err_mm")?,
        l_g_mm: req_f64_or_inf_sentinel("L_g_mm")?,
        guard_h_mm: req_f64("guard_h_mm")?,
        tex_amp_mm: req_f64("tex_amp_mm")?,
        tex_crest_r_mm: req_f64("tex_crest_r_mm")?,
        com_to_index_mm: req_f64("com_to_index_mm")?,
        spin_i_g_mm2: req_f64("spin_I_g_mm2")?,
        spin_i_ref_g_mm2: req_f64("spin_I_ref_g_mm2")?,
    };
    Ok(score_ergolight(&inputs).to_json())
}

/// Frozen draft-c weights (draft b used the same table), so the frozen draft-b parity fixtures keep testing the
/// subscores after draft d re-weighted `solid_tip_3d`.
#[cfg(test)]
const WEIGHTS_DRAFT_C: &[(&str, f64)] = &[
    ("mass", 0.370),
    ("hiker_lateral_180n", 0.035),
    ("hunter_lateral_260n", 0.035),
    ("strict_hunter_260n", 0.020),
    ("ring_bearing_300n_sf2", 0.025),
    ("ring_back_30n_sf2", 0.030),
    ("twist_body_1p82nm", 0.025),
    ("twist_tip_1p82nm", 0.025),
    ("solid_tip_3d", 0.030),
    ("side_a4_buckling", 0.020),
    ("failure_sequence_1p2", 0.035),
    ("plate_gci_3level", 0.020),
    ("human_max_twist_spine", 0.020),
    ("goodman_fatigue_l7", 0.020),
    ("gate_s_sharpness", 0.050),
    ("gate_s_point_profile", 0.050),
    ("hollow_grip_structure", 0.020),
    ("cad_watertight", 0.040),
    ("karambit_ring_and_claw", 0.040),
    ("form_fairness", 0.040),
    ("am_build", 0.050),
];

/// Test-only evaluator for `W_ERGOLIGHT_v1` draft-b (`ring_pull_300n_sf2` and `ring_back_300n_sf2`, both 600 N cap)
/// against frozen `tools_frozen/` (`run6` and `parity/g3hz_parity_v1.json`).
#[cfg(test)]
pub fn score_ergolight_map_draft_b(m: &Map<String, Value>) -> Result<Value, String> {
    let mut m_c = m.clone();
    let ring_pull_cap_n = m
        .get("ring_pull_cap_n")
        .and_then(|v| v.as_f64())
        .ok_or("missing ring_pull_cap_n")?;
    let ring_back_cap_n = m
        .get("ring_back_cap_n")
        .and_then(|v| v.as_f64())
        .ok_or("missing ring_back_cap_n")?;
    m_c.insert("ring_bearing_cap_n".to_string(), json!(ring_pull_cap_n));
    let mut doc = score_ergolight_map(&m_c)?;
    let e_mult = doc["E"].as_f64().unwrap_or(1.0);
    let s_pull = sfk(ring_pull_cap_n / 600.0);
    let s_back_b = sfk(ring_back_cap_n / 600.0);
    let s_back_c = sfk(ring_back_cap_n / 60.0);
    let mut base_c = 0.0;
    for &(k, w) in WEIGHTS_DRAFT_C {
        let s_k = doc["terms"][k]["s"].as_f64().ok_or_else(|| format!("missing term {k}"))?;
        base_c += w * s_k;
    }
    let base_b = base_c - 0.030 * s_back_c + 0.030 * s_back_b;
    let raw_b = 100.0 * e_mult * base_b;
    doc["draft"] = json!("b");
    doc["base"] = json!(base_b);
    doc["score_100"] = json!(round_n(raw_b, 6));
    if let Some(terms) = doc.get_mut("terms").and_then(|t| t.as_object_mut()) {
        terms.remove("ring_bearing_300n_sf2");
        terms.remove("ring_back_30n_sf2");
        terms.insert(
            "ring_pull_300n_sf2".to_string(),
            json!({ "w": 0.025, "s": s_pull, "pts": 100.0 * e_mult * 0.025 * s_pull }),
        );
        terms.insert(
            "ring_back_300n_sf2".to_string(),
            json!({ "w": 0.030, "s": s_back_b, "pts": 100.0 * e_mult * 0.030 * s_back_b }),
        );
    }
    Ok(doc)
}

/// Extract the real blade/plate solver observations from `_rs.json` and `_master_scorecard.json`
/// without any silent fallback bugs (fixing the 4 v1 scorer bugs noted in `plan.md` §1) and
/// combine with `HandErgoMetrics`.
pub fn inputs_from_v2_receipts(
    rs_doc: &Value,
    scorecard_doc: &Value,
    hand: &HandErgoMetrics,
) -> Result<ErgolightInputs, String> {
    let em = scorecard_doc
        .get("engineering_metrics")
        .ok_or("missing engineering_metrics in scorecard")?;
    let ex = rs_doc
        .get("export_measurements")
        .ok_or("missing export_measurements in rs_doc")?;
    let sharp = ex
        .get("sharpness")
        .ok_or("missing export_measurements.sharpness in rs_doc")?;
    let point = ex
        .get("point")
        .ok_or("missing export_measurements.point in rs_doc")?;
    let gates = scorecard_doc
        .get("weighted_cad_score")
        .and_then(|w| w.get("gates"))
        .ok_or("missing weighted_cad_score.gates in scorecard")?;

    let req_f64 = |obj: &Value, k: &str, ctx: &str| -> Result<f64, String> {
        let v = obj
            .get(k)
            .and_then(|v| v.as_f64())
            .ok_or_else(|| format!("missing or null key '{k}' in {ctx}"))?;
        if !v.is_finite() {
            return Err(format!("non-finite key '{k}' in {ctx}"));
        }
        Ok(v)
    };
    let req_bool = |obj: &Value, k: &str, ctx: &str| -> Result<bool, String> {
        obj.get(k)
            .and_then(|v| v.as_bool())
            .ok_or_else(|| format!("missing or non-bool key '{k}' in {ctx}"))
    };

    Ok(ErgolightInputs {
        mass_g: hand.mass_g,
        lat_rob_cert_n: req_f64(em, "lat_rob_cert_n", "engineering_metrics")?,
        strict_lat_n: req_f64(em, "strict_lat_rob_0p0625_n", "engineering_metrics")?,
        ring_bearing_cap_n: hand.ring_bearing_cap_n,
        ring_back_cap_n: hand.ring_back_cap_n,
        twist_body_sf: req_f64(em, "twist_body_sf", "engineering_metrics")?,
        twist_tip_sf: req_f64(em, "twist_tipzone_sf_cert", "engineering_metrics")?,
        twist_tipzone_strict_sf: req_f64(em, "twist_tipzone_sf_strict", "engineering_metrics")?,
        solid_tip_sf: req_f64(em, "solid_tip_sf_nodal_max", "engineering_metrics")?,
        dolfinx_rel_err: req_f64(em, "solid_tip_dolfinx_rel_err", "engineering_metrics")?,
        buckling_lambda: req_f64(em, "buckling_eigenvalue_a4", "engineering_metrics")?,
        seq_ratio: req_f64(em, "seq_ratio_cert", "engineering_metrics")?,
        gci_admitted: req_bool(gates, "plate_gci_valid_3level", "weighted_cad_score.gates")?,
        hm_twist_sf: req_f64(rs_doc, "hm_twist_sf", "rs_doc")?,
        hm_spine_sf: req_f64(rs_doc, "hm_spine_sf", "rs_doc")?,
        fat_sf: req_f64(em, "fat_sf_l7", "engineering_metrics")?,
        max_apex_mm: req_f64(sharp, "max_apex_mm", "export_measurements.sharpness")?,
        max_half_angle_deg: req_f64(
            sharp,
            "max_half_angle_deg",
            "export_measurements.sharpness",
        )?,
        t_0p25_mm: req_f64(point, "t_0p25_mm", "export_measurements.point")?,
        t_1p00_mm: req_f64(point, "t_1p00_mm", "export_measurements.point")?,
        t_2p00_mm: req_f64(point, "t_2p00_mm", "export_measurements.point")?,
        point_monotonic: req_bool(point, "monotonic", "export_measurements.point")?,
        grip_sf_min: hand.grip_sf_min,
        watertight: hand.watertight,
        karambit_ok: hand.karambit_ok,
        fairness: hand.fairness,
        am_build: hand.am_build,
        lip_w: hand.lip_w,
        bore_round: hand.bore_round,
        crown: hand.crown,
        junction_step_mm: hand.junction_step_mm,
        swell_perimeter_mm: hand.swell_perimeter_mm,
        swell_h_over_t: hand.swell_h_over_t,
        edge_r_min_mm: hand.edge_r_min_mm,
        section_corner_r_min_mm: hand.section_corner_r_min_mm,
        groove_r_min_mm: hand.groove_r_min_mm,
        crest_r_min_mm: hand.crest_r_min_mm,
        saddle_depth_mm: hand.saddle_depth_mm,
        choil_depth_mm: hand.choil_depth_mm,
        mid_depth_mm: hand.mid_depth_mm,
        station_err_mm: hand.station_err_mm,
        l_g_mm: hand.l_g_mm,
        guard_h_mm: hand.guard_h_mm,
        tex_amp_mm: hand.tex_amp_mm,
        tex_crest_r_mm: hand.tex_crest_r_mm,
        com_to_index_mm: hand.com_to_index_mm,
        spin_i_g_mm2: hand.spin_i_g_mm2,
        spin_i_ref_g_mm2: hand.spin_i_ref_g_mm2,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weights_and_betas_sum_to_unity_without_sheath_term() {
        let w_sum: f64 = WEIGHTS.iter().map(|(_, w)| *w).sum();
        let b_sum: f64 = BETAS.iter().map(|(_, b)| *b).sum();
        assert!((w_sum - 1.0).abs() < 1e-12, "WEIGHTS sum = {w_sum}");
        assert!((b_sum - 1.0).abs() < 1e-12, "BETAS sum = {b_sum}");
        assert!(
            !WEIGHTS.iter().any(|(k, _)| k.contains("holder") || k.contains("sheath")),
            "sheath/holder terms must be absent per owner directive"
        );
        assert_eq!(WEIGHTS[0], ("mass", 0.370));
    }

    #[test]
    fn mass_term_is_strictly_linear_down_to_25g_and_sfk_has_6pct_knee() {
        // Check sfk knee behavior
        assert!((sfk(0.0) - 0.0).abs() < 1e-12);
        assert!((sfk(0.5) - 0.47).abs() < 1e-12);
        assert!((sfk(1.0) - 0.94).abs() < 1e-12);
        let sf_1p15 = 0.94 + 0.06 * (1.0 - (-1.0_f64).exp());
        assert!((sfk(1.15) - sf_1p15).abs() < 1e-12);
        assert!(sfk(1.30) > sfk(1.15) && sfk(1.30) < 1.0);

        // Check mass term does not saturate at 45 g (keeps rewarding weight savings down to 25 g)
        let mut m45 = sample_default_inputs();
        m45.mass_g = 45.0;
        let mut m30 = m45.clone();
        m30.mass_g = 30.0;
        let s45 = score_ergolight(&m45);
        let s30 = score_ergolight(&m30);
        assert!(
            s30.raw_score_100 > s45.raw_score_100 + 5.0,
            "30 g design ({}) must score >5 pts above 45 g design ({})",
            s30.raw_score_100,
            s45.raw_score_100
        );
    }

    #[test]
    fn rs_karambit_v3_toml_matches_ergolight_weights_betas_and_constants() {
        let toml_text = include_str!("../../../requirements/RS-karambit-v3.toml");
        let mut section = String::new();
        let mut kv: std::collections::BTreeMap<(String, String), String> =
            std::collections::BTreeMap::new();
        for raw_line in toml_text.lines() {
            let line = raw_line.split('#').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }
            if line.starts_with('[') && line.ends_with(']') {
                section = line[1..line.len() - 1].trim().to_string();
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                kv.insert((section.clone(), k.trim().to_string()), v.trim().to_string());
            }
        }
        let get_str = |sec: &str, key: &str| -> String {
            kv.get(&(sec.to_string(), key.to_string()))
                .unwrap_or_else(|| panic!("missing [{sec}].{key}"))
                .trim_matches('"')
                .to_string()
        };
        let get_f64 = |sec: &str, key: &str| -> f64 {
            kv.get(&(sec.to_string(), key.to_string()))
                .unwrap_or_else(|| panic!("missing [{sec}].{key}"))
                .parse::<f64>()
                .unwrap_or_else(|e| panic!("invalid f64 at [{sec}].{key}: {e}"))
        };
        assert_eq!(get_str("", "id"), "RS-karambit-v3");
        assert_eq!(get_str("", "metric"), "W_ERGOLIGHT_v1");
        assert_eq!(get_str("", "draft"), "d");
        assert!((get_f64("constants", "nu") - 0.34).abs() < 1e-12);
        assert!((get_f64("loads", "ring_bearing_working_n") - 300.0).abs() < 1e-12);
        assert!((get_f64("loads", "ring_back_working_n") - 30.0).abs() < 1e-12);
        assert!((get_f64("constants", "blade_reach_max_mm") - 85.725).abs() < 1e-12);
        assert!((get_f64("constants", "sharpened_edge_cap_in") - 4.0).abs() < 1e-12);
        for &(k, w) in WEIGHTS {
            let val = get_f64("weights", k);
            assert!(
                (val - w).abs() < 1e-12,
                "RS-karambit-v3.toml weight [{k}] = {val} != {w}"
            );
        }
        for &(k, b) in BETAS {
            let val = get_f64("betas", k);
            assert!(
                (val - b).abs() < 1e-12,
                "RS-karambit-v3.toml beta [{k}] = {val} != {b}"
            );
        }

        // Also exercise `inputs_from_v2_receipts` (`B2`)
        let rs_doc = json!({
            "hm_twist_sf": 2.0,
            "hm_spine_sf": 2.0,
            "export_measurements": {
                "sharpness": { "max_apex_mm": 0.145, "max_half_angle_deg": 20.0 },
                "point": { "t_0p25_mm": 0.22, "t_1p00_mm": 0.48, "t_2p00_mm": 0.85, "monotonic": true }
            }
        });
        let sc_doc = json!({
            "engineering_metrics": {
                "lat_rob_cert_n": 310.0,
                "strict_lat_rob_0p0625_n": 305.0,
                "twist_body_sf": 3.5,
                "twist_tipzone_sf_cert": 2.8,
                "twist_tipzone_sf_strict": 2.7,
                "solid_tip_sf_nodal_max": 1.65,
                "solid_tip_dolfinx_rel_err": 0.015,
                "buckling_eigenvalue_a4": 8.5,
                "seq_ratio_cert": 1.45,
                "fat_sf_l7": 1.5
            },
            "weighted_cad_score": {
                "gates": { "plate_gci_valid_3level": true }
            }
        });
        let d = sample_default_inputs();
        let hand = HandErgoMetrics {
            mass_g: d.mass_g,
            ring_bearing_cap_n: d.ring_bearing_cap_n,
            ring_back_cap_n: d.ring_back_cap_n,
            grip_sf_min: d.grip_sf_min,
            watertight: d.watertight,
            karambit_ok: d.karambit_ok,
            fairness: d.fairness,
            am_build: d.am_build,
            lip_w: d.lip_w,
            bore_round: d.bore_round,
            crown: d.crown,
            junction_step_mm: d.junction_step_mm,
            swell_perimeter_mm: d.swell_perimeter_mm,
            swell_h_over_t: d.swell_h_over_t,
            edge_r_min_mm: d.edge_r_min_mm,
            section_corner_r_min_mm: d.section_corner_r_min_mm,
            groove_r_min_mm: d.groove_r_min_mm,
            crest_r_min_mm: d.crest_r_min_mm,
            saddle_depth_mm: d.saddle_depth_mm,
            choil_depth_mm: d.choil_depth_mm,
            mid_depth_mm: d.mid_depth_mm,
            station_err_mm: d.station_err_mm,
            l_g_mm: d.l_g_mm,
            guard_h_mm: d.guard_h_mm,
            tex_amp_mm: d.tex_amp_mm,
            tex_crest_r_mm: d.tex_crest_r_mm,
            com_to_index_mm: d.com_to_index_mm,
            spin_i_g_mm2: d.spin_i_g_mm2,
            spin_i_ref_g_mm2: d.spin_i_ref_g_mm2,
        };
        let combined = inputs_from_v2_receipts(&rs_doc, &sc_doc, &hand).expect("valid receipts");
        assert!((combined.solid_tip_sf - 1.65).abs() < 1e-12);
        assert!((combined.dolfinx_rel_err - 0.015).abs() < 1e-12);
        assert!(combined.gci_admitted);
    }

    #[test]
    fn parity_with_python_ergolight_draft_d_and_real_receipt_keys() {
        let m = sample_default_inputs();
        let sc = score_ergolight(&m);
        // Inside all target ergonomic bands, every e_k == 1.0 so E == 1.0
        assert!((sc.ergo_multiplier - 1.0).abs() < 1e-12, "E = {}", sc.ergo_multiplier);

        // Verify JSON map roundtrip parity without hitting serde_json::json! recursion limit
        let mut map_obj = Map::new();
        for (k, v) in [
            ("mass_g", m.mass_g),
            ("lat_rob_cert_n", m.lat_rob_cert_n),
            ("strict_lat_n", m.strict_lat_n),
            ("ring_bearing_cap_n", m.ring_bearing_cap_n),
            ("ring_back_cap_n", m.ring_back_cap_n),
            ("twist_body_sf", m.twist_body_sf),
            ("twist_tip_sf", m.twist_tip_sf),
            ("twist_tipzone_strict_sf", m.twist_tipzone_strict_sf),
            ("solid_tip_sf", m.solid_tip_sf),
            ("dolfinx_rel_err", m.dolfinx_rel_err),
            ("buckling_lambda", m.buckling_lambda),
            ("seq_ratio", m.seq_ratio),
            ("hm_twist_sf", m.hm_twist_sf),
            ("hm_spine_sf", m.hm_spine_sf),
            ("fat_sf", m.fat_sf),
            ("max_apex_mm", m.max_apex_mm),
            ("max_half_angle_deg", m.max_half_angle_deg),
            ("t_0p25_mm", m.t_0p25_mm),
            ("t_1p00_mm", m.t_1p00_mm),
            ("t_2p00_mm", m.t_2p00_mm),
            ("grip_sf_min", m.grip_sf_min),
            ("fairness", m.fairness),
            ("am_build", m.am_build),
            ("lip_w", m.lip_w),
            ("bore_round", m.bore_round),
            ("crown", m.crown),
            ("junction_step_mm", m.junction_step_mm),
            ("swell_perimeter_mm", m.swell_perimeter_mm),
            ("swell_h_over_t", m.swell_h_over_t),
            ("edge_r_min_mm", m.edge_r_min_mm),
            ("section_corner_r_min_mm", m.section_corner_r_min_mm),
            ("groove_r_min_mm", m.groove_r_min_mm),
            ("crest_r_min_mm", m.crest_r_min_mm),
            ("saddle_depth_mm", m.saddle_depth_mm),
            ("choil_depth_mm", m.choil_depth_mm),
            ("mid_depth_mm", m.mid_depth_mm),
            ("station_err_mm", m.station_err_mm),
            ("L_g_mm", m.l_g_mm),
            ("guard_h_mm", m.guard_h_mm),
            ("tex_amp_mm", m.tex_amp_mm),
            ("tex_crest_r_mm", m.tex_crest_r_mm),
            ("com_to_index_mm", m.com_to_index_mm),
            ("spin_I_g_mm2", m.spin_i_g_mm2),
            ("spin_I_ref_g_mm2", m.spin_i_ref_g_mm2),
        ] {
            map_obj.insert(k.to_string(), json!(v));
        }
        for (k, b) in [
            ("gci_admitted", m.gci_admitted),
            ("point_monotonic", m.point_monotonic),
            ("watertight", m.watertight),
            ("karambit_ok", m.karambit_ok),
        ] {
            map_obj.insert(k.to_string(), json!(b));
        }
        let doc = score_ergolight_map(&map_obj).unwrap();
        assert_eq!(doc["metric"].as_str(), Some("W_ERGOLIGHT_v1"));
        assert_eq!(doc["draft"].as_str(), Some("d"));
        assert_eq!(doc["score_100"].as_f64(), Some(sc.score_100));

        // Verify fail-closed rejection on missing solver key or legacy ring_pull_cap_n key
        let mut incomplete = map_obj.clone();
        incomplete.remove("hm_twist_sf");
        assert!(score_ergolight_map(&incomplete).is_err());
        let mut legacy_ring = map_obj.clone();
        legacy_ring.remove("ring_bearing_cap_n");
        legacy_ring.insert("ring_pull_cap_n".to_string(), json!(989.0));
        assert!(score_ergolight_map(&legacy_ring).is_err());
    }

    fn sample_default_inputs() -> ErgolightInputs {
        // Real blade observations from ec3i042_r01_p10_c56 f075 + default G5 hand-zone candidate
        ErgolightInputs {
            mass_g: 62.6,
            lat_rob_cert_n: 201.1,
            strict_lat_n: 199.9,
            ring_bearing_cap_n: 989.0,
            ring_back_cap_n: 637.0,
            twist_body_sf: 2.509,
            twist_tip_sf: 0.507,
            twist_tipzone_strict_sf: 0.556,
            solid_tip_sf: 0.645,
            dolfinx_rel_err: 0.0,
            buckling_lambda: 8.67899747353465,
            seq_ratio: 1.34,
            gci_admitted: true,
            hm_twist_sf: 0.05693914447958685,
            hm_spine_sf: 0.27392339260684456,
            fat_sf: 0.5617739041132452,
            max_apex_mm: 0.21474588867172784,
            max_half_angle_deg: 26.131685264717643,
            t_0p25_mm: 0.3262673769758777,
            t_1p00_mm: 0.7290022191903188,
            t_2p00_mm: 1.1695931342210566,
            point_monotonic: false,
            grip_sf_min: 1.85,
            watertight: true,
            karambit_ok: true,
            fairness: 1.0,
            am_build: 1.0,
            lip_w: 8.0,
            bore_round: 2.8,
            crown: 0.3,
            junction_step_mm: 0.0,
            swell_perimeter_mm: 70.0,
            swell_h_over_t: 1.75,
            edge_r_min_mm: 2.5,
            section_corner_r_min_mm: 4.5,
            groove_r_min_mm: 12.0,
            crest_r_min_mm: 6.0,
            saddle_depth_mm: 2.0,
            choil_depth_mm: 4.2,
            mid_depth_mm: 2.1,
            station_err_mm: 0.5,
            l_g_mm: 65.5,
            guard_h_mm: 13.5,
            tex_amp_mm: 0.18,
            tex_crest_r_mm: 0.36,
            com_to_index_mm: 3.5,
            spin_i_g_mm2: 240_000.0,
            spin_i_ref_g_mm2: 320_000.0,
        }
    }
}
