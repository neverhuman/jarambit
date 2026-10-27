//! Strict loader for the G5 requirement set `requirements/RS-karambit-v5.toml`.
//!
//! Every table is a typed struct with `deny_unknown_fields` and no defaults, so a key the code does not read, a
//! missing key, or a renamed key is a parse error (audit F-R-1: G3 read only part of RS-karambit-v3). The file is
//! compiled in and hashed into the G5 decoder (`g5_qd::g5_decoder_sha256`), so a cohort can never mix requirement
//! sets. `validate` adds the cross-key rules; it fails closed.

use serde::Deserialize;
use std::sync::OnceLock;

pub const RS_V5_TEXT: &str = include_str!("../../../requirements/RS-karambit-v5.toml");

macro_rules! strict {
    ($(#[$m:meta])* $name:ident { $($f:ident : $t:ty),* $(,)? }) => {
        $(#[$m])*
        #[derive(Debug, Clone, PartialEq, Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct $name { $(pub $f: $t),* }
    };
}

strict!(RsV5 {
    id: String,
    version: u32,
    generation: String,
    metric: String,
    study: String,
    configuration: String,
    hard_gates: bool,
    margins: Margins,
    material: Material,
    loads: Loads,
    fatigue: FatigueDuty,
    sequence: Sequence,
    buckling: Buckling,
    edge: Edge,
    serration: Serration,
    point: Point,
    ergonomics: Ergonomics,
    am: Am,
    mass: Mass,
    objective: Objective,
});
strict!(Margins {
    sf_yield: f64,
    sf_ultimate: f64,
    sf_ring: f64
});
strict!(Material {
    name: String,
    yield_mpa: f64,
    ultimate_mpa: f64,
    elongation_min: f64,
    e_gpa: f64,
    nu: f64,
    g_gpa: f64,
    density_g_per_cm3: f64,
    density_min_g_per_cm3: f64,
    service_temp_c: f64,
    k_temp_yield: f64,
    k_temp_ultimate: f64,
    k_temp_modulus: f64,
    fracture_toughness_min_mpa_sqrt_m: f64,
    thin_wall: ThinWall,
    fatigue: MaterialFatigue,
    tpms: Tpms,
});
strict!(ThinWall {
    yield_exponent: f64,
    ultimate_exponent: f64,
    ts_up_skin_mm: f64,
    ts_down_skin_mm: f64,
    ts_ground_mm: f64,
    stiffness_uses_effective_thickness: bool,
});
strict!(MaterialFatigue {
    as_built_thick_mpa: f64,
    as_built_thin_mpa: f64,
    ground_mpa: f64,
    hip_ground_mpa: f64,
    thin_threshold_mm: f64,
    mean_stress_method: String,
    mean_stress_sensitivity: f64,
    notch_rule: String,
    cycles: f64,
});
strict!(Tpms {
    gyroid_stiffness_c: f64,
    gyroid_stiffness_n: f64,
    gyroid_strength_c: f64,
    gyroid_strength_n: f64,
    schwarz_d_stiffness_c: f64,
    schwarz_d_stiffness_n: f64,
    schwarz_d_strength_c: f64,
    schwarz_d_strength_n: f64,
    strut_tpms_allowed: bool,
    fatigue_ratio_compression: f64,
    fatigue_ratio_tension: f64,
});
strict!(Loads {
    lateral_bend: LateralBend,
    body_twist: BodyTwist,
    tip: TipLoads,
    grip: GripLoads,
    guard: GuardLoads,
    ring: RingLoads,
    pull: PullLoad,
    teeth: TeethLoads,
    drop: DropTest,
});
strict!(LateralBend {
    apply_mm_from_point: f64,
    limit_root_moment_nm: f64,
    ultimate_root_moment_nm: f64,
    physical_set_test_max_mm: f64,
});
strict!(BodyTwist {
    service_nm: f64,
    limit_nm: f64,
    friction_credit_max_nm: f64
});
strict!(TipLoads {
    axial_n: f64, axial_no_yield_from_mm: f64, lateral_n: f64, lateral_check_stations_mm: Vec<f64>, twist_nm: f64,
    twist_apply_mm_from_point: f64,
});
strict!(GripLoads {
    limit_squeeze_n: f64,
    normal_sum_factor: f64,
    working_typical_n: f64,
    working_peak_n: f64,
    p95_male_grip_n: f64,
});
strict!(GuardLoads { limit_push_n: f64 });
strict!(RingLoads {
    bearing_n: f64,
    bearing_arc_deg: f64,
    back_n: f64,
    any_direction_n: f64,
    working_n: f64
});
strict!(PullLoad {
    limit_n: f64,
    held_length_from_point_mm: f64
});
strict!(TeethLoads {
    snag_n: f64,
    pry_n: f64,
    pry_teeth_sharing: u32
});
strict!(DropTest {
    height_m: f64,
    orientations: u32,
    point_set_max_mm: f64,
    computed: bool
});
strict!(FatigueDuty {
    blade_root_moment_amplitude_nm: f64,
    twist_amplitude_nm: f64,
    cycles: f64,
    sf_fatigue: f64
});
strict!(Sequence {
    per_load_case: bool,
    ratio_min: f64
});
strict!(Buckling {
    load_case: String,
    lambda_min: f64
});
strict!(Edge {
    per_side_angle_min_deg: f64,
    per_side_angle_max_deg: f64,
    finished_apex_flat_min_mm: f64,
    finished_apex_flat_max_mm: f64,
    finishing_allowance_mm: f64,
    finishing_allowance_min_mm: f64,
    plunge_fraction_min: f64,
    plunge_fraction_max: f64,
    sharpened_arc_max_mm: f64,
    blade_reach_max_mm: f64,
    iso8442_initial_cut_min_mm: f64,
    iso8442_retention_plain_min_mm: f64,
    iso8442_retention_serrated_min_mm: f64,
});
strict!(Serration {
    pitch_min_mm: f64,
    pitch_max_mm: f64,
    depth_min_mm: f64,
    depth_max_mm: f64,
    depth_max_fraction_of_pitch: f64,
    gullet_radius_min_mm: f64,
    full_depth_scallops_min: u32,
    span_start_fraction_min: f64,
    span_start_fraction_max: f64,
    span_end_from_point_min_mm: f64,
    check_end_teeth: bool,
});
strict!(Point {
    angle_max_at_1mm_deg: f64,
    angle_max_at_3mm_deg: f64,
    width_max_at_2mm: f64,
    tip_radius_min_mm: f64,
    tip_radius_max_mm: f64,
    thickness_max_at_025mm: f64,
    thickness_max_at_1mm: f64,
    thickness_max_at_2mm: f64,
});
strict!(Ergonomics {
    grip: ErgoGrip,
    guard: ErgoGuard,
    edges: ErgoEdges,
    pressure: ErgoPressure,
    friction: ErgoFriction,
    ring: ErgoRing,
    balance: ErgoBalance,
});
strict!(ErgoGrip {
    perimeter_min_mm: f64,
    perimeter_max_mm: f64,
    perimeter_floor_mm: f64,
    height_over_thickness_min: f64,
    height_over_thickness_max: f64,
    length_min_mm: f64,
    groove_pitch_min_mm: f64,
    groove_pitch_max_mm: f64,
});
strict!(ErgoGuard {
    height_target_mm: f64,
    height_floor_mm: f64,
    fillet_min_mm: f64,
    fillet_max_mm: f64
});
strict!(ErgoEdges {
    radius_min_mm: f64,
    radius_min_thick_section_mm: f64,
    thick_section_min_mm: f64,
    thick_section_max_mm: f64,
    full_round_below_mm: f64,
    jimping_radius_min_mm: f64,
});
strict!(ErgoPressure {
    all_day_kpa_at_typical: f64,
    discomfort_finger_kpa: f64,
    discomfort_palm_kpa: f64,
    discomfort_thenar_kpa: f64,
    pain_finger_kpa: f64,
    pain_palm_kpa: f64,
    pain_thenar_kpa: f64,
});
strict!(ErgoFriction {
    mu_nominal: f64,
    mu_wet: f64,
    mu_credit_max: f64
});
strict!(ErgoRing {
    bore_d_mm: f64,
    lip_w_mm: f64,
    working_pressure_max_kpa: f64
});
strict!(ErgoBalance {
    com_to_index_max_mm: f64,
    spin_inertia_ratio_max: f64
});
strict!(Am {
    skin_min_mm: f64,
    skin_min_load_bearing_mm: f64,
    sheet_min_mm: f64,
    port_d_min_mm: f64,
    ports_per_cavity_min: u32,
    closed_voids_allowed: bool,
    overhang_min_deg: f64,
    horizontal_hole_max_mm: f64,
    ring_bore_machined: bool,
    ledge_max_mm: f64,
    supports_on_edges_allowed: bool,
    build_orientation_declared: bool,
});
strict!(Mass {
    ceiling_g: f64,
    floor_g: f64,
    goal_g: f64,
    grid_agreement_max_frac: f64
});
strict!(Objective {
    below_one_scale: f64,
    below_one_exponent: f64,
    knee_bonus: f64,
    knee_tau: f64,
    worst_margin_weight: f64,
    mass_weight: f64,
    e_exponent: f64,
    min_grams_per_percent_e: f64,
    weights: Weights,
    betas: Betas,
});
strict!(
    /// Weights of the structural and geometry terms (the 0.45 left after mass and the worst-margin term).
    Weights {
        lateral_limit: f64, lateral_ultimate: f64, twist_limit: f64, tip_axial: f64, tip_lateral: f64,
        tip_twist: f64, grip_squeeze: f64, guard_push: f64, ring_bearing: f64, ring_back: f64,
        ring_any_direction: f64, pull: f64, teeth: f64, fatigue: f64, sequence: f64, buckling: f64, edge: f64,
        point: f64, serration: f64, am_build: f64, form: f64,
    }
);
strict!(
    /// Geometric-mean exponents of the ergonomic multiplier E.
    Betas {
        grip_section: f64, ring: f64, hand_edges: f64, contact_pressure: f64, guard: f64, grip_length: f64,
        balance: f64, grip_surface: f64,
    }
);

impl Weights {
    pub fn entries(&self) -> [(&'static str, f64); 21] {
        [
            ("lateral_limit", self.lateral_limit),
            ("lateral_ultimate", self.lateral_ultimate),
            ("twist_limit", self.twist_limit),
            ("tip_axial", self.tip_axial),
            ("tip_lateral", self.tip_lateral),
            ("tip_twist", self.tip_twist),
            ("grip_squeeze", self.grip_squeeze),
            ("guard_push", self.guard_push),
            ("ring_bearing", self.ring_bearing),
            ("ring_back", self.ring_back),
            ("ring_any_direction", self.ring_any_direction),
            ("pull", self.pull),
            ("teeth", self.teeth),
            ("fatigue", self.fatigue),
            ("sequence", self.sequence),
            ("buckling", self.buckling),
            ("edge", self.edge),
            ("point", self.point),
            ("serration", self.serration),
            ("am_build", self.am_build),
            ("form", self.form),
        ]
    }
}

impl Betas {
    pub fn entries(&self) -> [(&'static str, f64); 8] {
        [
            ("grip_section", self.grip_section),
            ("ring", self.ring),
            ("hand_edges", self.hand_edges),
            ("contact_pressure", self.contact_pressure),
            ("guard", self.guard),
            ("grip_length", self.grip_length),
            ("balance", self.balance),
            ("grip_surface", self.grip_surface),
        ]
    }
}

impl Objective {
    /// Mass credit (grams) that a 1 % drop of E costs a design whose weighted terms sum to `base`:
    /// score = 100·E^k·Σw·s, so dS/S = k·dE/E, and one gram is worth 100·E^k·w_mass/(ceiling − floor).
    pub fn grams_per_percent_e(&self, base: f64, mass: &Mass) -> f64 {
        self.e_exponent * 0.01 * base * (mass.ceiling_g - mass.floor_g) / self.mass_weight
    }
}

/// Lowest weighted base at which the owner rule (1 % of E costs at least `min_grams_per_percent_e`) must hold. A
/// design scoring below this is failing most terms, so the rule is checked from here up.
pub const OWNER_RULE_BASE_MIN: f64 = 0.60;

fn ordered(name: &str, lo: f64, hi: f64) -> Result<(), String> {
    if lo.is_finite() && hi.is_finite() && lo <= hi {
        Ok(())
    } else {
        Err(format!(
            "RS-karambit-v5: {name} band [{lo}, {hi}] is not an ordered finite range"
        ))
    }
}

impl RsV5 {
    /// Cross-key rules. Fails closed on the first violation.
    pub fn validate(&self) -> Result<(), String> {
        let want = [
            ("id", self.id.as_str(), "RS-karambit-v5"),
            ("generation", self.generation.as_str(), "G5"),
            ("metric", self.metric.as_str(), "W_ERGOLIGHT_v2"),
            (
                "buckling.load_case",
                self.buckling.load_case.as_str(),
                "tip_axial",
            ),
            (
                "material.fatigue.mean_stress_method",
                self.material.fatigue.mean_stress_method.as_str(),
                "FKM",
            ),
        ];
        for (k, got, w) in want {
            if got != w {
                return Err(format!("RS-karambit-v5: {k} = '{got}', expected '{w}'"));
            }
        }
        if self.version != 5 {
            return Err(format!(
                "RS-karambit-v5: version = {}, expected 5",
                self.version
            ));
        }
        if self.hard_gates {
            return Err(
                "RS-karambit-v5: hard_gates must be false (owner rule: no hard gates in evolution)"
                    .into(),
            );
        }
        if !self.sequence.per_load_case {
            return Err("RS-karambit-v5: sequence.per_load_case must be true".into());
        }
        if self.loads.drop.computed {
            return Err(
                "RS-karambit-v5: loads.drop is a physical test; computed must be false".into(),
            );
        }
        let m = &self.margins;
        if !(m.sf_yield >= 1.0 && m.sf_ultimate >= m.sf_yield && m.sf_ring >= 1.0) {
            return Err(format!(
                "RS-karambit-v5: margins {m:?} are not sf_ultimate >= sf_yield >= 1"
            ));
        }
        let e = &self.edge;
        ordered(
            "edge.per_side_angle",
            e.per_side_angle_min_deg,
            e.per_side_angle_max_deg,
        )?;
        ordered(
            "edge.finished_apex_flat",
            e.finished_apex_flat_min_mm,
            e.finished_apex_flat_max_mm,
        )?;
        ordered(
            "edge.plunge_fraction",
            e.plunge_fraction_min,
            e.plunge_fraction_max,
        )?;
        if e.finishing_allowance_mm < e.finishing_allowance_min_mm {
            return Err("RS-karambit-v5: edge.finishing_allowance_mm is below its minimum".into());
        }
        let s = &self.serration;
        ordered("serration.pitch", s.pitch_min_mm, s.pitch_max_mm)?;
        ordered("serration.depth", s.depth_min_mm, s.depth_max_mm)?;
        ordered(
            "serration.span_start_fraction",
            s.span_start_fraction_min,
            s.span_start_fraction_max,
        )?;
        if s.depth_min_mm > s.depth_max_fraction_of_pitch * s.pitch_max_mm {
            return Err(
                "RS-karambit-v5: no serration depth satisfies depth <= fraction * pitch".into(),
            );
        }
        let p = &self.point;
        ordered("point.tip_radius", p.tip_radius_min_mm, p.tip_radius_max_mm)?;
        if p.angle_max_at_3mm_deg > p.angle_max_at_1mm_deg {
            return Err(
                "RS-karambit-v5: point angle limit at 3 mm exceeds the limit at 1 mm".into(),
            );
        }
        let g = &self.ergonomics.grip;
        ordered(
            "ergonomics.grip.perimeter",
            g.perimeter_min_mm,
            g.perimeter_max_mm,
        )?;
        ordered(
            "ergonomics.grip.height_over_thickness",
            g.height_over_thickness_min,
            g.height_over_thickness_max,
        )?;
        ordered(
            "ergonomics.grip.groove_pitch",
            g.groove_pitch_min_mm,
            g.groove_pitch_max_mm,
        )?;
        if g.perimeter_floor_mm > g.perimeter_min_mm {
            return Err(
                "RS-karambit-v5: ergonomics.grip.perimeter_floor_mm is above perimeter_min_mm"
                    .into(),
            );
        }
        let gd = &self.ergonomics.guard;
        ordered(
            "ergonomics.guard.fillet",
            gd.fillet_min_mm,
            gd.fillet_max_mm,
        )?;
        ordered(
            "ergonomics.guard.height",
            gd.height_floor_mm,
            gd.height_target_mm,
        )?;
        let ms = &self.mass;
        if !(ms.floor_g < ms.goal_g && ms.goal_g < ms.ceiling_g) {
            return Err(format!(
                "RS-karambit-v5: mass {ms:?} is not floor < goal < ceiling"
            ));
        }
        if self.material.tpms.strut_tpms_allowed {
            return Err("RS-karambit-v5: material.tpms.strut_tpms_allowed must be false".into());
        }
        let o = &self.objective;
        let wsum: f64 = o.weights.entries().iter().map(|(_, w)| w).sum::<f64>()
            + o.mass_weight
            + o.worst_margin_weight;
        if (wsum - 1.0).abs() > 1e-9 {
            return Err(format!(
                "RS-karambit-v5: objective weights sum to {wsum}, expected 1"
            ));
        }
        let bsum: f64 = o.betas.entries().iter().map(|(_, b)| b).sum();
        if (bsum - 1.0).abs() > 1e-9 {
            return Err(format!(
                "RS-karambit-v5: objective betas sum to {bsum}, expected 1"
            ));
        }
        if let Some((k, _)) = o
            .weights
            .entries()
            .iter()
            .chain(o.betas.entries().iter())
            .find(|(_, v)| !(*v > 0.0))
        {
            return Err(format!(
                "RS-karambit-v5: objective term '{k}' must have a positive weight"
            ));
        }
        let grams = o.grams_per_percent_e(OWNER_RULE_BASE_MIN, ms);
        if grams < o.min_grams_per_percent_e {
            return Err(format!(
                "RS-karambit-v5: 1 % of E costs {grams:.2} g at base {OWNER_RULE_BASE_MIN}, below the owner rule of {} g",
                o.min_grams_per_percent_e
            ));
        }
        Ok(())
    }
}

/// Parse and validate a requirement-set text. Unknown or missing keys are errors.
pub fn parse_rs_v5(text: &str) -> Result<RsV5, String> {
    let rs: RsV5 =
        toml::from_str(text).map_err(|e| format!("RS-karambit-v5 does not parse strictly: {e}"))?;
    rs.validate()?;
    Ok(rs)
}

/// The committed requirement set, parsed once. Panics (fails closed) if the committed file is invalid; the unit
/// tests below make that a build-time failure in CI.
pub fn rs_v5() -> &'static RsV5 {
    static RS: OnceLock<RsV5> = OnceLock::new();
    RS.get_or_init(|| parse_rs_v5(RS_V5_TEXT).unwrap_or_else(|e| panic!("{e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn committed_requirement_set_parses_strictly_and_validates() {
        let rs = rs_v5();
        assert_eq!(rs.id, "RS-karambit-v5");
        assert!(!rs.hard_gates);
        assert_eq!(rs.margins.sf_ring, 2.0);
        assert_eq!(rs.ergonomics.ring.bore_d_mm, 24.9);
        assert_eq!(rs.ergonomics.ring.lip_w_mm, 8.0);
        assert_eq!(rs.loads.ring.bearing_n, 300.0);
        assert_eq!(rs.loads.ring.back_n, 30.0);
        assert_eq!(rs.point.angle_max_at_1mm_deg, 75.0);
        assert_eq!(rs.point.angle_max_at_3mm_deg, 60.0);
        assert_eq!(
            (
                rs.edge.per_side_angle_min_deg,
                rs.edge.per_side_angle_max_deg
            ),
            (20.0, 25.0)
        );
        assert_eq!(rs.loads.tip.lateral_check_stations_mm, vec![1.0, 2.0]);
    }

    fn edited(old: &str, new: &str) -> String {
        assert_eq!(
            RS_V5_TEXT.matches(old).count(),
            1,
            "fixture line '{old}' must occur once"
        );
        RS_V5_TEXT.replacen(old, new, 1)
    }

    #[test]
    fn unknown_missing_and_mistyped_keys_fail_closed() {
        let unknown = edited("sf_ring = 2.0", "sf_ring = 2.0\nsf_spine = 1.0");
        assert!(parse_rs_v5(&unknown).unwrap_err().contains("unknown field"));
        let missing = edited("knee_tau = 0.15", "");
        assert!(parse_rs_v5(&missing).unwrap_err().contains("missing field"));
        let mistyped = edited("pry_teeth_sharing = 3", "pry_teeth_sharing = \"3\"");
        assert!(parse_rs_v5(&mistyped).is_err());
    }

    #[test]
    fn cross_key_rules_fail_closed() {
        assert!(
            parse_rs_v5(&edited("hard_gates = false", "hard_gates = true"))
                .unwrap_err()
                .contains("hard_gates")
        );
        assert!(parse_rs_v5(&edited("form = 0.0225", "form = 0.0325"))
            .unwrap_err()
            .contains("sum to"));
        assert!(
            parse_rs_v5(&edited("grip_surface = 0.05", "grip_surface = 0.06"))
                .unwrap_err()
                .contains("betas")
        );
        assert!(parse_rs_v5(&edited("e_exponent = 5.0", "e_exponent = 1.0"))
            .unwrap_err()
            .contains("owner rule"));
        assert!(parse_rs_v5(&edited("computed = false", "computed = true"))
            .unwrap_err()
            .contains("physical test"));
        assert!(parse_rs_v5(&edited("pitch_max_mm = 6.0", "pitch_max_mm = 3.0")).is_err());
    }

    #[test]
    fn owner_rule_one_percent_of_e_costs_at_least_ten_grams_over_the_scoring_range() {
        let rs = rs_v5();
        for i in 0..=40 {
            let base = OWNER_RULE_BASE_MIN + (1.0 - OWNER_RULE_BASE_MIN) * i as f64 / 40.0;
            let g = rs.objective.grams_per_percent_e(base, &rs.mass);
            assert!(
                g >= rs.objective.min_grams_per_percent_e,
                "base {base}: {g} g"
            );
        }
    }
}
