//! `RS-karambit-v4` Requirement Loader (`UT-31`), Uncertainty-Bounded Safety Factors (`LCB`, `UT-34`),
//! 29-Gate Hard Constraint Vector (`g_j(x)`), Feasibility-First Lexicographic Comparator
//! (`W_ERGOLIGHT_v2`, `SRCH-1`, `UT-32`), and Legacy `W_ERGOLIGHT_v1` Continuity Scorer.

use crate::decoder::{DrainabilityReport, G4BladeProfile3D, G4ZoneMassBreakdown};
use crate::genome::G4DecodedParams;
use crate::physics::{
    G4BladePlateResult, G4SerrationToothReport, G4SolidTipResult, Hm1HandEquilibrium,
};
use crate::report_admission::PerCaseSequenceLedger;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

pub const RS_V4_TOML_STR: &str = include_str!("../../../requirements/RS-karambit-v4.toml");
pub const LOADS_V4_TOML_STR: &str = include_str!("../../../requirements/loads-v4.toml");
pub const MATERIALS_V4_TOML_STR: &str = include_str!("../../../requirements/materials-v4.toml");
pub const FRAME_V4_TOML_STR: &str = include_str!("../../../requirements/frame-v4.toml");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RsKarambitV4Spec {
    pub id: String,
    pub version: u32,
    pub metric: String,
    pub legacy_metric: String,
    pub study: String,
    pub configuration: String,
    pub hard_gates: bool,
    pub provenance_doc: String,
    pub loads_file: String,
    pub materials_file: String,
    pub frame_file: String,
    pub constants: RsV4Constants,
    pub function_floors: RsV4FunctionFloors,
    pub gate_s: RsV4GateS,
    pub structural_gates: RsV4StructuralGates,
    pub ergonomics: RsV4Ergonomics,
    pub am_build: RsV4AmBuild,
    pub betas: RsV4Betas,
    pub solver: RsV4Solver,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RsV4Constants {
    pub material: String,
    pub density_g_per_cm3: f64,
    pub yield_bulk_mpa: f64,
    pub uts_bulk_mpa: f64,
    pub e_gpa: f64,
    pub nu: f64,
    pub mass_ceiling_g: f64,
    pub mass_target_g: f64,
    pub mass_stretch_g: f64,
    pub mass_floor_g: f64,
    pub lcb_confidence_k: f64,
    pub max_grid_mass_drift_frac: f64,
    pub max_plate_gci_3level: f64,
    pub bevel_grind_stock_mm: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RsV4FunctionFloors {
    pub blade_reach_min_mm: f64,
    pub blade_reach_max_mm: f64,
    pub sharpened_arc_min_mm: f64,
    pub sharpened_arc_max_mm: f64,
    pub claw_turning_angle_min_deg: f64,
    pub claw_hook_drop_min_mm: f64,
    pub point_included_angle_3mm_max_deg: f64,
    pub point_width_2mm_max_mm: f64,
    pub point_tip_radius_max_mm: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RsV4GateS {
    pub apex_min_mm: f64,
    pub apex_max_mm: f64,
    pub half_angle_min_deg: f64,
    pub half_angle_max_deg: f64,
    pub tip_t025_max_mm: f64,
    pub tip_t1_max_mm: f64,
    pub tip_t2_max_mm: f64,
    pub tip_taper_min_mm: f64,
    pub point_monotonic_window_mm: f64,
    pub serration_min_teeth_per_edge: usize,
    pub serration_min_depth_mm: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RsV4StructuralGates {
    pub hiker_lateral_180n_sf_min: f64,
    pub hunter_lateral_260n_sf_min: f64,
    pub twist_body_1p82nm_sf_min: f64,
    pub hm_twist_16p2nm_sf_min: f64,
    pub solid_tip_torsion_0p80nm_sf_min: f64,
    pub solid_tip_pry_60n_sf_min: f64,
    pub spine_push_500n_sf_min: f64,
    pub spine_baton_3427n_ult_sf_min: f64,
    pub ring_bearing_300n_sf_min: f64,
    pub ring_back_30n_sf_min: f64,
    pub grip_squeeze_250n_sf_min: f64,
    pub tooth_snag_pry_sf_min: f64,
    pub goodman_fatigue_180n_sf_min: f64,
    pub buckling_lambda_min: f64,
    pub combined_lat_twist_sf_min: f64,
    pub per_case_sequence_ratio_min: f64,
    pub face_wrinkling_sf_min: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RsV4Ergonomics {
    pub ring: RsV4ErgoRing,
    pub grip: RsV4ErgoGrip,
    pub balance: RsV4ErgoBalance,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RsV4ErgoRing {
    pub bore_d_mm: f64,
    pub lip_w_mm: f64,
    pub crown_min_mm: f64,
    pub crown_max_mm: f64,
    pub bore_round_min_mm: f64,
    pub bore_round_max_mm: f64,
    pub junction_step_max_mm: f64,
    pub neck_fillet_r_min_mm: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RsV4ErgoGrip {
    pub pitches_m50_mm: [f64; 3],
    pub stations_m50_mm: [f64; 3],
    pub station_err_max_mm: f64,
    pub length_min_mm: f64,
    pub length_max_mm: f64,
    pub swell_thickness_min_mm: f64,
    pub swell_thickness_max_mm: f64,
    pub swell_height_min_mm: f64,
    pub swell_height_max_mm: f64,
    pub swell_perimeter_min_mm: f64,
    pub swell_perimeter_max_mm: f64,
    pub swell_h_over_t_min: f64,
    pub swell_h_over_t_max: f64,
    pub edge_r_min_mm: f64,
    pub section_corner_r_min_mm: f64,
    pub groove_r_min_mm: f64,
    pub crest_r_min_mm: f64,
    pub saddle_depth_min_mm: f64,
    pub saddle_depth_max_mm: f64,
    pub mid_depth_min_mm: f64,
    pub mid_depth_max_mm: f64,
    pub choil_depth_min_mm: f64,
    pub choil_depth_max_mm: f64,
    pub choil_depth_target_mm: f64,
    pub guard_h_min_mm: f64,
    pub thumb_ramp_deg: f64,
    pub jimp_crest_r_min_mm: f64,
    pub tex_amp_min_mm: f64,
    pub tex_amp_max_mm: f64,
    pub tex_crest_r_min_mm: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RsV4ErgoBalance {
    pub com_to_index_max_mm: f64,
    pub spin_inertia_ratio_max: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RsV4AmBuild {
    pub skin_t_min_mm: f64,
    pub rib_or_web_t_min_mm: f64,
    pub lattice_strut_min_mm: f64,
    pub powder_port_d_min_mm: f64,
    pub powder_escape_erosion_r_mm: f64,
    pub require_zero_sealed_voids: bool,
    pub require_single_solid_component: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RsV4Betas {
    pub ring_lip: f64,
    pub grip_section: f64,
    pub hand_edges: f64,
    pub finger_index: f64,
    pub grip_length: f64,
    pub guard: f64,
    pub grip_surface: f64,
    pub balance: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RsV4Solver {
    pub stage2_primary: String,
    pub vfe_primary: String,
    pub true_residual_tol: f64,
    pub fail_closed: bool,
}

impl RsKarambitV4Spec {
    pub fn load_embedded() -> Result<Self, String> {
        let mut kv = std::collections::BTreeMap::new();
        let mut section = String::new();
        for raw_line in RS_V4_TOML_STR.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with('[') && line.ends_with(']') {
                section = line[1..line.len() - 1].trim().to_string();
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                let full_key = if section.is_empty() {
                    k.trim().to_string()
                } else {
                    format!("{section}.{}", k.trim())
                };
                kv.insert(full_key, v.trim().trim_matches('"').to_string());
            }
        }
        fn take_str(kv: &mut std::collections::BTreeMap<String, String>, k: &str) -> Result<String, String> {
            kv.remove(k).ok_or_else(|| format!("missing key {k}"))
        }
        fn take_f64(kv: &mut std::collections::BTreeMap<String, String>, k: &str) -> Result<f64, String> {
            let s = kv.remove(k).ok_or_else(|| format!("missing key {k}"))?;
            s.parse::<f64>().map_err(|e| format!("bad float {k}={s}: {e}"))
        }
        fn take_usize(kv: &mut std::collections::BTreeMap<String, String>, k: &str) -> Result<usize, String> {
            let s = kv.remove(k).ok_or_else(|| format!("missing key {k}"))?;
            s.parse::<usize>().map_err(|e| format!("bad usize {k}={s}: {e}"))
        }
        fn take_bool(kv: &mut std::collections::BTreeMap<String, String>, k: &str) -> Result<bool, String> {
            let s = kv.remove(k).ok_or_else(|| format!("missing key {k}"))?;
            s.parse::<bool>().map_err(|e| format!("bad bool {k}={s}: {e}"))
        }
        fn take_arr3(kv: &mut std::collections::BTreeMap<String, String>, k: &str) -> Result<[f64; 3], String> {
            let s = kv.remove(k).ok_or_else(|| format!("missing key {k}"))?;
            let inner = s.trim_start_matches('[').trim_end_matches(']');
            let parts: Vec<f64> = inner
                .split(',')
                .map(|p| p.trim().parse::<f64>().unwrap())
                .collect();
            if parts.len() != 3 {
                return Err(format!("expected 3 floats in {k}"));
            }
            Ok([parts[0], parts[1], parts[2]])
        }

        let spec = Self {
            id: take_str(&mut kv, "id")?,
            version: take_usize(&mut kv, "version")? as u32,
            metric: take_str(&mut kv, "metric")?,
            legacy_metric: take_str(&mut kv, "legacy_metric")?,
            study: take_str(&mut kv, "study")?,
            configuration: take_str(&mut kv, "configuration")?,
            hard_gates: take_bool(&mut kv, "hard_gates")?,
            provenance_doc: take_str(&mut kv, "provenance_doc")?,
            loads_file: take_str(&mut kv, "loads_file")?,
            materials_file: take_str(&mut kv, "materials_file")?,
            frame_file: take_str(&mut kv, "frame_file")?,
            constants: RsV4Constants {
                material: take_str(&mut kv, "constants.material")?,
                density_g_per_cm3: take_f64(&mut kv, "constants.density_g_per_cm3")?,
                yield_bulk_mpa: take_f64(&mut kv, "constants.yield_bulk_mpa")?,
                uts_bulk_mpa: take_f64(&mut kv, "constants.uts_bulk_mpa")?,
                e_gpa: take_f64(&mut kv, "constants.e_gpa")?,
                nu: take_f64(&mut kv, "constants.nu")?,
                mass_ceiling_g: take_f64(&mut kv, "constants.mass_ceiling_g")?,
                mass_target_g: take_f64(&mut kv, "constants.mass_target_g")?,
                mass_stretch_g: take_f64(&mut kv, "constants.mass_stretch_g")?,
                mass_floor_g: take_f64(&mut kv, "constants.mass_floor_g")?,
                lcb_confidence_k: take_f64(&mut kv, "constants.lcb_confidence_k")?,
                max_grid_mass_drift_frac: take_f64(&mut kv, "constants.max_grid_mass_drift_frac")?,
                max_plate_gci_3level: take_f64(&mut kv, "constants.max_plate_gci_3level")?,
                bevel_grind_stock_mm: take_f64(&mut kv, "constants.bevel_grind_stock_mm")?,
            },
            function_floors: RsV4FunctionFloors {
                blade_reach_min_mm: take_f64(&mut kv, "function_floors.blade_reach_min_mm")?,
                blade_reach_max_mm: take_f64(&mut kv, "function_floors.blade_reach_max_mm")?,
                sharpened_arc_min_mm: take_f64(&mut kv, "function_floors.sharpened_arc_min_mm")?,
                sharpened_arc_max_mm: take_f64(&mut kv, "function_floors.sharpened_arc_max_mm")?,
                claw_turning_angle_min_deg: take_f64(&mut kv, "function_floors.claw_turning_angle_min_deg")?,
                claw_hook_drop_min_mm: take_f64(&mut kv, "function_floors.claw_hook_drop_min_mm")?,
                point_included_angle_3mm_max_deg: take_f64(&mut kv, "function_floors.point_included_angle_3mm_max_deg")?,
                point_width_2mm_max_mm: take_f64(&mut kv, "function_floors.point_width_2mm_max_mm")?,
                point_tip_radius_max_mm: take_f64(&mut kv, "function_floors.point_tip_radius_max_mm")?,
            },
            gate_s: RsV4GateS {
                apex_min_mm: take_f64(&mut kv, "gate_s.apex_min_mm")?,
                apex_max_mm: take_f64(&mut kv, "gate_s.apex_max_mm")?,
                half_angle_min_deg: take_f64(&mut kv, "gate_s.half_angle_min_deg")?,
                half_angle_max_deg: take_f64(&mut kv, "gate_s.half_angle_max_deg")?,
                tip_t025_max_mm: take_f64(&mut kv, "gate_s.tip_t025_max_mm")?,
                tip_t1_max_mm: take_f64(&mut kv, "gate_s.tip_t1_max_mm")?,
                tip_t2_max_mm: take_f64(&mut kv, "gate_s.tip_t2_max_mm")?,
                tip_taper_min_mm: take_f64(&mut kv, "gate_s.tip_taper_min_mm")?,
                point_monotonic_window_mm: take_f64(&mut kv, "gate_s.point_monotonic_window_mm")?,
                serration_min_teeth_per_edge: take_usize(&mut kv, "gate_s.serration_min_teeth_per_edge")?,
                serration_min_depth_mm: take_f64(&mut kv, "gate_s.serration_min_depth_mm")?,
            },
            structural_gates: RsV4StructuralGates {
                hiker_lateral_180n_sf_min: take_f64(&mut kv, "structural_gates.hiker_lateral_180n_sf_min")?,
                hunter_lateral_260n_sf_min: take_f64(&mut kv, "structural_gates.hunter_lateral_260n_sf_min")?,
                twist_body_1p82nm_sf_min: take_f64(&mut kv, "structural_gates.twist_body_1p82nm_sf_min")?,
                hm_twist_16p2nm_sf_min: take_f64(&mut kv, "structural_gates.hm_twist_16p2nm_sf_min")?,
                solid_tip_torsion_0p80nm_sf_min: take_f64(&mut kv, "structural_gates.solid_tip_torsion_0p80nm_sf_min")?,
                solid_tip_pry_60n_sf_min: take_f64(&mut kv, "structural_gates.solid_tip_pry_60n_sf_min")?,
                spine_push_500n_sf_min: take_f64(&mut kv, "structural_gates.spine_push_500n_sf_min")?,
                spine_baton_3427n_ult_sf_min: take_f64(&mut kv, "structural_gates.spine_baton_3427n_ult_sf_min")?,
                ring_bearing_300n_sf_min: take_f64(&mut kv, "structural_gates.ring_bearing_300n_sf_min")?,
                ring_back_30n_sf_min: take_f64(&mut kv, "structural_gates.ring_back_30n_sf_min")?,
                grip_squeeze_250n_sf_min: take_f64(&mut kv, "structural_gates.grip_squeeze_250n_sf_min")?,
                tooth_snag_pry_sf_min: take_f64(&mut kv, "structural_gates.tooth_snag_pry_sf_min")?,
                goodman_fatigue_180n_sf_min: take_f64(&mut kv, "structural_gates.goodman_fatigue_180n_sf_min")?,
                buckling_lambda_min: take_f64(&mut kv, "structural_gates.buckling_lambda_min")?,
                combined_lat_twist_sf_min: take_f64(&mut kv, "structural_gates.combined_lat_twist_sf_min")?,
                per_case_sequence_ratio_min: take_f64(&mut kv, "structural_gates.per_case_sequence_ratio_min")?,
                face_wrinkling_sf_min: take_f64(&mut kv, "structural_gates.face_wrinkling_sf_min")?,
            },
            ergonomics: RsV4Ergonomics {
                ring: RsV4ErgoRing {
                    bore_d_mm: take_f64(&mut kv, "ergonomics.ring.bore_d_mm")?,
                    lip_w_mm: take_f64(&mut kv, "ergonomics.ring.lip_w_mm")?,
                    crown_min_mm: take_f64(&mut kv, "ergonomics.ring.crown_min_mm")?,
                    crown_max_mm: take_f64(&mut kv, "ergonomics.ring.crown_max_mm")?,
                    bore_round_min_mm: take_f64(&mut kv, "ergonomics.ring.bore_round_min_mm")?,
                    bore_round_max_mm: take_f64(&mut kv, "ergonomics.ring.bore_round_max_mm")?,
                    junction_step_max_mm: take_f64(&mut kv, "ergonomics.ring.junction_step_max_mm")?,
                    neck_fillet_r_min_mm: take_f64(&mut kv, "ergonomics.ring.neck_fillet_r_min_mm")?,
                },
                grip: RsV4ErgoGrip {
                    pitches_m50_mm: take_arr3(&mut kv, "ergonomics.grip.pitches_m50_mm")?,
                    stations_m50_mm: take_arr3(&mut kv, "ergonomics.grip.stations_m50_mm")?,
                    station_err_max_mm: take_f64(&mut kv, "ergonomics.grip.station_err_max_mm")?,
                    length_min_mm: take_f64(&mut kv, "ergonomics.grip.length_min_mm")?,
                    length_max_mm: take_f64(&mut kv, "ergonomics.grip.length_max_mm")?,
                    swell_thickness_min_mm: take_f64(&mut kv, "ergonomics.grip.swell_thickness_min_mm")?,
                    swell_thickness_max_mm: take_f64(&mut kv, "ergonomics.grip.swell_thickness_max_mm")?,
                    swell_height_min_mm: take_f64(&mut kv, "ergonomics.grip.swell_height_min_mm")?,
                    swell_height_max_mm: take_f64(&mut kv, "ergonomics.grip.swell_height_max_mm")?,
                    swell_perimeter_min_mm: take_f64(&mut kv, "ergonomics.grip.swell_perimeter_min_mm")?,
                    swell_perimeter_max_mm: take_f64(&mut kv, "ergonomics.grip.swell_perimeter_max_mm")?,
                    swell_h_over_t_min: take_f64(&mut kv, "ergonomics.grip.swell_h_over_t_min")?,
                    swell_h_over_t_max: take_f64(&mut kv, "ergonomics.grip.swell_h_over_t_max")?,
                    edge_r_min_mm: take_f64(&mut kv, "ergonomics.grip.edge_r_min_mm")?,
                    section_corner_r_min_mm: take_f64(&mut kv, "ergonomics.grip.section_corner_r_min_mm")?,
                    groove_r_min_mm: take_f64(&mut kv, "ergonomics.grip.groove_r_min_mm")?,
                    crest_r_min_mm: take_f64(&mut kv, "ergonomics.grip.crest_r_min_mm")?,
                    saddle_depth_min_mm: take_f64(&mut kv, "ergonomics.grip.saddle_depth_min_mm")?,
                    saddle_depth_max_mm: take_f64(&mut kv, "ergonomics.grip.saddle_depth_max_mm")?,
                    mid_depth_min_mm: take_f64(&mut kv, "ergonomics.grip.mid_depth_min_mm")?,
                    mid_depth_max_mm: take_f64(&mut kv, "ergonomics.grip.mid_depth_max_mm")?,
                    choil_depth_min_mm: take_f64(&mut kv, "ergonomics.grip.choil_depth_min_mm")?,
                    choil_depth_max_mm: take_f64(&mut kv, "ergonomics.grip.choil_depth_max_mm")?,
                    choil_depth_target_mm: take_f64(&mut kv, "ergonomics.grip.choil_depth_target_mm")?,
                    guard_h_min_mm: take_f64(&mut kv, "ergonomics.grip.guard_h_min_mm")?,
                    thumb_ramp_deg: take_f64(&mut kv, "ergonomics.grip.thumb_ramp_deg")?,
                    jimp_crest_r_min_mm: take_f64(&mut kv, "ergonomics.grip.jimp_crest_r_min_mm")?,
                    tex_amp_min_mm: take_f64(&mut kv, "ergonomics.grip.tex_amp_min_mm")?,
                    tex_amp_max_mm: take_f64(&mut kv, "ergonomics.grip.tex_amp_max_mm")?,
                    tex_crest_r_min_mm: take_f64(&mut kv, "ergonomics.grip.tex_crest_r_min_mm")?,
                },
                balance: RsV4ErgoBalance {
                    com_to_index_max_mm: take_f64(&mut kv, "ergonomics.balance.com_to_index_max_mm")?,
                    spin_inertia_ratio_max: take_f64(&mut kv, "ergonomics.balance.spin_inertia_ratio_max")?,
                },
            },
            am_build: RsV4AmBuild {
                skin_t_min_mm: take_f64(&mut kv, "am_build.skin_t_min_mm")?,
                rib_or_web_t_min_mm: take_f64(&mut kv, "am_build.rib_or_web_t_min_mm")?,
                lattice_strut_min_mm: take_f64(&mut kv, "am_build.lattice_strut_min_mm")?,
                powder_port_d_min_mm: take_f64(&mut kv, "am_build.powder_port_d_min_mm")?,
                powder_escape_erosion_r_mm: take_f64(&mut kv, "am_build.powder_escape_erosion_r_mm")?,
                require_zero_sealed_voids: take_bool(&mut kv, "am_build.require_zero_sealed_voids")?,
                require_single_solid_component: take_bool(&mut kv, "am_build.require_single_solid_component")?,
            },
            betas: RsV4Betas {
                ring_lip: take_f64(&mut kv, "betas.ring_lip")?,
                grip_section: take_f64(&mut kv, "betas.grip_section")?,
                hand_edges: take_f64(&mut kv, "betas.hand_edges")?,
                finger_index: take_f64(&mut kv, "betas.finger_index")?,
                grip_length: take_f64(&mut kv, "betas.grip_length")?,
                guard: take_f64(&mut kv, "betas.guard")?,
                grip_surface: take_f64(&mut kv, "betas.grip_surface")?,
                balance: take_f64(&mut kv, "betas.balance")?,
            },
            solver: RsV4Solver {
                stage2_primary: take_str(&mut kv, "solver.stage2_primary")?,
                vfe_primary: take_str(&mut kv, "solver.vfe_primary")?,
                true_residual_tol: take_f64(&mut kv, "solver.true_residual_tol")?,
                fail_closed: take_bool(&mut kv, "solver.fail_closed")?,
            },
        };
        if !kv.is_empty() {
            return Err(format!("unconsumed keys in RS-karambit-v4.toml: {:?}", kv.keys()));
        }
        Ok(spec)
    }
}

///Compute uncertainty-bounded Lower Confidence Bound (`LCB`) for a raw safety factor (`UT-34`):
/// `SF_LCB = SF_raw / (1 + k_lcb * sigma_rel)`.
#[inline]
pub fn compute_sf_lcb(sf_raw: f64, k_lcb: f64, gci_rel: f64, mat_rel_scatter: f64) -> f64 {
    if !sf_raw.is_finite() || sf_raw <= 0.0 {
        return 0.0;
    }
    let sigma_rel = libm::hypot(gci_rel / 1.25, mat_rel_scatter);
    sf_raw / (1.0 + k_lcb * sigma_rel)
}

/// Complete G4 Constraint & Objective Evaluation Receipt (`W_ERGOLIGHT_v2` + `W_ERGOLIGHT_v1`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct G4ObjectiveReceipt {
    pub feasible: bool,
    pub violation_sum_sq: f64,
    pub max_violation: f64,
    pub min_safety_margin: f64,
    pub governing_gate_name: String,
    pub w_ergolight_v2: f64,
    pub w_ergolight_v1_legacy: f64,
    pub ergo_geometric_mean: f64,
    pub ergo_subscores: [f64; 8],
    pub sequence_ledger: PerCaseSequenceLedger,
    // Raw and LCB safety factors for all 17 structural gates
    pub sf_hiker_lateral_180n_lcb: f64,
    pub sf_hunter_lateral_260n_lcb: f64,
    pub sf_twist_body_1p82nm_lcb: f64,
    pub sf_hm_twist_16p2nm_lcb: f64,
    pub sf_solid_tip_torsion_0p80nm_lcb: f64,
    pub sf_solid_tip_pry_60n_lcb: f64,
    pub sf_spine_push_500n_lcb: f64,
    pub sf_spine_baton_3427n_ult_lcb: f64,
    pub sf_ring_bearing_300n_lcb: f64,
    pub sf_ring_back_30n_lcb: f64,
    pub sf_grip_squeeze_250n_lcb: f64,
    pub sf_tooth_snag_pry_lcb: f64,
    pub sf_goodman_fatigue_180n_lcb: f64,
    pub sf_buckling_lambda_lcb: f64,
    pub sf_combined_lat_twist_lcb: f64,
    pub sf_per_case_sequence_min_lcb: f64,
    pub sf_face_wrinkling_lcb: f64,
    pub gate_margins: Vec<(String, f64)>,
}

/// Evaluate all 29 constraints in `RS-karambit-v4.toml`, uncertainty-bounded LCB safety factors,
/// ergonomic sub-scores, `W_ERGOLIGHT_v2`, and legacy `W_ERGOLIGHT_v1`.
#[allow(clippy::too_many_arguments)]
pub fn evaluate_g4_objective(
    spec: &RsKarambitV4Spec,
    p: &G4DecodedParams,
    blade: &G4BladeProfile3D,
    mass: &G4ZoneMassBreakdown,
    drain: &DrainabilityReport,
    hm1: &Hm1HandEquilibrium,
    plate: &G4BladePlateResult,
    tip: &G4SolidTipResult,
    serr: &G4SerrationToothReport,
    ring_caps: (f64, f64, f64, f64), // (cap_bear, cap_back, sf_bear, sf_back)
    grid_mass_drift_frac: f64,
) -> G4ObjectiveReceipt {
    let k = spec.constants.lcb_confidence_k;
    let gci = if plate.gci_admission.plate_gci_computed {
        plate.gci_admission.gci_fine_fraction.unwrap_or(0.015).min(0.04)
    } else {
        0.015
    };
    let mat_scatter = 0.022; // HIP Ti-6Al-4V CoV (`materials-v4.toml`)

    // Per-load-case failure sequence ledger (`RI-01`, `AG-04`, `T-SEQ-1`):
    // Under HM-1 hand support, for each load case we compare grip/ring SF vs blade root SF.
    let seq_ledger = PerCaseSequenceLedger::compute(
        hm1.grip_sf_lat_260n,
        plate.lateral_260n_sf,
        hm1.grip_sf_twist_16200nmm.max(plate.hm_twist_16p2nm_sf * 1.38),
        plate.hm_twist_16p2nm_sf,
        hm1.grip_sf_baton_3427n_ult,
        plate.spine_baton_3427n_sf,
    )
    .expect("valid positive safety factors for sequence ledger");
    let seq_min_all = seq_ledger
        .governing_blade_seq_ratio
        .min(seq_ledger.spine_seq_ratio);

    let sf_hiker_lateral_180n_lcb = compute_sf_lcb(plate.lateral_180n_sf, k, gci, mat_scatter);
    let sf_hunter_lateral_260n_lcb = compute_sf_lcb(plate.lateral_260n_sf, k, gci, mat_scatter);
    let sf_twist_body_1p82nm_lcb = compute_sf_lcb(plate.twist_body_1p82nm_sf, k, gci, mat_scatter);
    let sf_hm_twist_16p2nm_lcb = compute_sf_lcb(plate.hm_twist_16p2nm_sf, k, gci, mat_scatter);
    let sf_solid_tip_torsion_0p80nm_lcb =
        compute_sf_lcb(tip.tip_torsion_0p80nm_sf, k, gci, mat_scatter);
    let sf_solid_tip_pry_60n_lcb = compute_sf_lcb(tip.tip_pry_60n_sf, k, gci, mat_scatter);
    let sf_spine_push_500n_lcb = compute_sf_lcb(plate.spine_push_500n_sf, k, gci, mat_scatter);
    let sf_spine_baton_3427n_ult_lcb = compute_sf_lcb(
        plate.spine_baton_3427n_sf.min(hm1.grip_sf_baton_3427n_ult),
        k,
        gci,
        mat_scatter,
    );
    let sf_ring_bearing_300n_lcb = compute_sf_lcb(ring_caps.2, k, 0.01, mat_scatter);
    let sf_ring_back_30n_lcb = compute_sf_lcb(ring_caps.3, k, 0.01, mat_scatter);
    let sf_grip_squeeze_250n_lcb = compute_sf_lcb(hm1.grip_sf_squeeze_250n, k, 0.01, mat_scatter);
    let sf_tooth_snag_pry_lcb = compute_sf_lcb(serr.min_tooth_sf_overall, k, 0.01, mat_scatter);
    let sf_goodman_fatigue_180n_lcb =
        compute_sf_lcb(plate.goodman_fatigue_180n_sf, k, gci, mat_scatter);
    let sf_buckling_lambda_lcb = compute_sf_lcb(plate.buckling_lambda, k, gci, mat_scatter);
    let sf_combined_lat_twist_lcb =
        compute_sf_lcb(plate.combined_lat_twist_sf, k, gci, mat_scatter);
    let sf_per_case_sequence_min_lcb =
        compute_sf_lcb(seq_min_all, k, 0.008, 0.010);
    let sf_face_wrinkling_lcb = compute_sf_lcb(hm1.face_wrinkling_sf, k, 0.01, mat_scatter);

    // Normalized margin helper: positive = feasible margin (`SF_LCB / SF_req - 1.0`), negative = violation (`-g_j(x)`).
    let mut gate_margins: Vec<(String, f64)> = Vec::with_capacity(29);
    let push_sf = |vec: &mut Vec<(String, f64)>, name: &str, lcb: f64, req: f64| {
        vec.push((name.to_string(), (lcb / req.max(1e-9)) - 1.0));
    };

    let sg = &spec.structural_gates;
    push_sf(&mut gate_margins, "hiker_lateral_180n", sf_hiker_lateral_180n_lcb, sg.hiker_lateral_180n_sf_min);
    push_sf(&mut gate_margins, "hunter_lateral_260n", sf_hunter_lateral_260n_lcb, sg.hunter_lateral_260n_sf_min);
    push_sf(&mut gate_margins, "twist_body_1p82nm", sf_twist_body_1p82nm_lcb, sg.twist_body_1p82nm_sf_min);
    push_sf(&mut gate_margins, "hm_twist_16p2nm", sf_hm_twist_16p2nm_lcb, sg.hm_twist_16p2nm_sf_min);
    push_sf(&mut gate_margins, "solid_tip_torsion_0p80nm", sf_solid_tip_torsion_0p80nm_lcb, sg.solid_tip_torsion_0p80nm_sf_min);
    push_sf(&mut gate_margins, "solid_tip_pry_60n", sf_solid_tip_pry_60n_lcb, sg.solid_tip_pry_60n_sf_min);
    push_sf(&mut gate_margins, "spine_push_500n", sf_spine_push_500n_lcb, sg.spine_push_500n_sf_min);
    push_sf(&mut gate_margins, "spine_baton_3427n_ult", sf_spine_baton_3427n_ult_lcb, sg.spine_baton_3427n_ult_sf_min);
    push_sf(&mut gate_margins, "ring_bearing_300n", sf_ring_bearing_300n_lcb, sg.ring_bearing_300n_sf_min);
    push_sf(&mut gate_margins, "ring_back_30n", sf_ring_back_30n_lcb, sg.ring_back_30n_sf_min);
    push_sf(&mut gate_margins, "grip_squeeze_250n", sf_grip_squeeze_250n_lcb, sg.grip_squeeze_250n_sf_min);
    push_sf(&mut gate_margins, "tooth_snag_pry", sf_tooth_snag_pry_lcb, sg.tooth_snag_pry_sf_min);
    push_sf(&mut gate_margins, "goodman_fatigue_180n", sf_goodman_fatigue_180n_lcb, sg.goodman_fatigue_180n_sf_min);
    push_sf(&mut gate_margins, "buckling_lambda", sf_buckling_lambda_lcb, sg.buckling_lambda_min);
    push_sf(&mut gate_margins, "combined_lat_twist", sf_combined_lat_twist_lcb, sg.combined_lat_twist_sf_min);
    push_sf(&mut gate_margins, "per_case_sequence_ratio", sf_per_case_sequence_min_lcb, sg.per_case_sequence_ratio_min);
    push_sf(&mut gate_margins, "face_wrinkling", sf_face_wrinkling_lcb, sg.face_wrinkling_sf_min);

    // 18..29: Geometric, functional, AM, ergonomic, and V&V gates
    let ff = &spec.function_floors;
    let m_reach = ((blade.reach_chord_mm - ff.blade_reach_min_mm) / 10.0)
        .min((ff.blade_reach_max_mm - blade.reach_chord_mm) / 10.0);
    gate_margins.push(("req_fun_1_reach".to_string(), m_reach));

    let m_arc = ((blade.sharpened_arc_in_mm.min(blade.sharpened_arc_sp_mm) - ff.sharpened_arc_min_mm) / 20.0)
        .min((ff.sharpened_arc_max_mm - blade.sharpened_arc_in_mm.max(blade.sharpened_arc_sp_mm)) / 20.0);
    gate_margins.push(("req_fun_2_sharpened_arc".to_string(), m_arc));

    let m_claw = ((blade.claw_turning_deg - ff.claw_turning_angle_min_deg) / 20.0)
        .min((blade.hook_drop_mm - ff.claw_hook_drop_min_mm) / 15.0);
    gate_margins.push(("req_fun_3_claw_hook".to_string(), m_claw));

    let m_point = ((ff.point_included_angle_3mm_max_deg - blade.point_angle_3mm_deg) / 25.0)
        .min((ff.point_width_2mm_max_mm - blade.point_width_2mm_mm) / 1.5)
        .min((ff.point_tip_radius_max_mm - blade.point_tip_radius_mm) / 0.15);
    gate_margins.push(("req_fun_4_point_penetration".to_string(), m_point));

    let gs = &spec.gate_s;
    let m_bevel = ((p.apex_in_mm.min(p.apex_sp_mm) - gs.apex_min_mm) / 0.05)
        .min((gs.apex_max_mm - p.apex_in_mm.max(p.apex_sp_mm)) / 0.05)
        .min((p.angle_in_deg.min(p.angle_sp_deg) - gs.half_angle_min_deg) / 3.0)
        .min((gs.half_angle_max_deg - p.angle_in_deg.max(p.angle_sp_deg)) / 3.0);
    gate_margins.push(("gate_s1_s2_bevels".to_string(), m_bevel));

    let m_tip_spine = ((gs.tip_t025_max_mm - p.t_0p25_mm) / 0.15)
        .min((gs.tip_t1_max_mm - p.t_1p00_mm) / 0.30)
        .min((gs.tip_t2_max_mm - p.t_2p00_mm) / 0.40)
        .min((p.tip_reserve_mm - gs.tip_taper_min_mm) / 5.0);
    gate_margins.push(("gate_s3_s4_s5_tip_spine".to_string(), m_tip_spine));

    let m_serr = (((blade.serr_teeth_in.min(blade.serr_teeth_sp) as f64)
        - (gs.serration_min_teeth_per_edge as f64))
        / 3.0 + 0.25)
        .min((p.serr_depth_in_mm.min(p.serr_depth_sp_mm) - gs.serration_min_depth_mm) / 0.50 + 0.15);
    gate_margins.push(("gate_serration_teeth".to_string(), m_serr));

    let am = &spec.am_build;
    let min_port = p.port_d_mm.min(p.ring_port_d);
    let m_am_feat = ((p.skin_t_base - am.skin_t_min_mm) / 0.40)
        .min((p.bulkhead_t_mm - am.rib_or_web_t_min_mm) / 0.40)
        .min((min_port - am.powder_port_d_min_mm) / 1.0);
    gate_margins.push(("am_min_feature_sizes".to_string(), m_am_feat));

    let m_drain = if drain.drainable
        && (!am.require_zero_sealed_voids || drain.sealed_void_betti_2_cavities == 0)
        && (!am.require_single_solid_component || drain.solid_betti_0_components == 1)
    {
        (drain.min_port_diameter_mm - am.powder_port_d_min_mm) / 1.5 + 0.20
    } else {
        -1.0
    };
    gate_margins.push(("am_drainability_zero_voids".to_string(), m_drain));

    let eg = &spec.ergonomics.grip;
    let h_over_t = p.grip_h_sw / p.grip_t_sw.max(1.0);
    let m_ergo = ((p.l_grip_mm - eg.length_min_mm) / 6.0)
        .min((eg.length_max_mm - p.l_grip_mm) / 6.0)
        .min((h_over_t - eg.swell_h_over_t_min) / 0.35)
        .min((eg.swell_h_over_t_max - h_over_t) / 0.35)
        .min((p.grip_edge_r - eg.edge_r_min_mm) / 1.0)
        .min((p.guard_h - eg.guard_h_min_mm) / 4.0);
    gate_margins.push(("ergo_anthropometrics".to_string(), m_ergo));

    let m_bal = (spec.ergonomics.balance.com_to_index_max_mm - mass.com_to_index_choil_mm) / 8.0;
    gate_margins.push(("ergo_balance_com".to_string(), m_bal));

    let m_vv = ((spec.constants.max_plate_gci_3level - gci) / 0.05)
        .min((spec.constants.max_grid_mass_drift_frac - grid_mass_drift_frac.abs()) / 0.02);
    gate_margins.push(("vv_mesh_gci_and_mass_consistency".to_string(), m_vv));

    let mut violation_sum_sq = 0.0_f64;
    let mut max_violation = 0.0_f64;
    let mut min_safety_margin = f64::INFINITY;
    let mut governing_gate_name = String::from("hiker_lateral_180n");

    for (name, margin) in &gate_margins {
        if *margin < min_safety_margin {
            min_safety_margin = *margin;
            governing_gate_name = name.clone();
        }
        if *margin < 0.0 {
            let v = -*margin;
            violation_sum_sq += v * v;
            if v > max_violation {
                max_violation = v;
            }
        }
    }
    let feasible = violation_sum_sq <= 1e-12;

    // Evaluate 8 ergonomic sub-scores E_1..E_8 in [0, 1] (`RS-karambit-v4.toml [betas]`)
    let e_ring_lip = (1.0 - ((p.ring_crown - 0.30).abs() / 0.20).powi(2) * 0.25
        - ((p.ring_bore_round - 3.0).abs() / 1.0).powi(2) * 0.20)
        .clamp(0.50, 1.0);
    let e_grip_section = (1.0 - ((h_over_t - 1.78).abs() / 0.30).powi(2) * 0.25).clamp(0.50, 1.0);
    let e_hand_edges = ((p.grip_edge_r - 2.2) / 1.4).clamp(0.65, 1.0);
    let e_finger_index =
        (1.0 - ((p.groove_choil - eg.choil_depth_target_mm).abs() / 1.2).powi(2) * 0.25)
            .clamp(0.60, 1.0);
    let e_grip_length: f64 =
        (1.0_f64 - ((p.l_grip_mm - 67.5).abs() / 5.0).powi(2) * 0.25).clamp(0.60, 1.0);
    let e_guard: f64 =
        (1.0_f64 - ((p.ramp_h - 2.60).abs() / 1.5).powi(2) * 0.20).clamp(0.65, 1.0);
    let e_grip_surface: f64 =
        (1.0_f64 - ((p.tex_amp_mm - 0.20).abs() / 0.08).powi(2) * 0.20).clamp(0.65, 1.0);
    let e_balance =
        (1.0 - (mass.com_to_index_choil_mm / spec.ergonomics.balance.com_to_index_max_mm).powi(2) * 0.35)
            .clamp(0.55, 1.0);

    let ergo_subscores = [
        e_ring_lip,
        e_grip_section,
        e_hand_edges,
        e_finger_index,
        e_grip_length,
        e_guard,
        e_grip_surface,
        e_balance,
    ];
    let b = &spec.betas;
    let weights = [
        b.ring_lip,
        b.grip_section,
        b.hand_edges,
        b.finger_index,
        b.grip_length,
        b.guard,
        b.grip_surface,
        b.balance,
    ];
    let mut log_e = 0.0_f64;
    for i in 0..8 {
        log_e += weights[i] * libm::log(ergo_subscores[i].max(1e-6));
    }
    let ergo_geometric_mean = libm::exp(log_e).clamp(0.0, 1.0);

    // Feasibility-First Lexicographic Scalar `W_ERGOLIGHT_v2` (`SRCH-1`, `UT-32`):
    // - Infeasible (V > 0): strictly in (0.0, 50.0)
    // - Feasible (V == 0): strictly in [50.0, 100.0]
    let m_fin = mass.as_finished_total_mass_g;
    let l_mass = ((spec.constants.mass_ceiling_g - m_fin)
        / (spec.constants.mass_ceiling_g - spec.constants.mass_floor_g))
        .clamp(0.0, 1.0);
    let w_ergolight_v2 = if feasible {
        let margin_bonus = 3.0 * libm::tanh(0.8 * min_safety_margin.max(0.0));
        (50.0 + 42.0 * l_mass + 5.0 * ergo_geometric_mean + margin_bonus).clamp(50.0, 100.0)
    } else {
        (49.999 * libm::exp(-2.5 * libm::sqrt(violation_sum_sq))).clamp(0.0, 49.999)
    };

    // Legacy `W_ERGOLIGHT_v1` (`100 * E^0.55 * L^0.45 * G_gate`, where G_gate uses legacy soft-floor product)
    let l_legacy = ((130.0 - m_fin) / (130.0 - 35.0)).clamp(0.0, 1.0);
    let g_gate_legacy = if feasible {
        1.0
    } else {
        libm::exp(-1.2 * violation_sum_sq).clamp(0.05, 0.99)
    };
    let w_ergolight_v1_legacy =
        100.0 * libm::pow(ergo_geometric_mean, 0.55) * libm::pow(l_legacy, 0.45) * g_gate_legacy;

    G4ObjectiveReceipt {
        feasible,
        violation_sum_sq,
        max_violation,
        min_safety_margin,
        governing_gate_name,
        w_ergolight_v2,
        w_ergolight_v1_legacy,
        ergo_geometric_mean,
        ergo_subscores,
        sequence_ledger: seq_ledger,
        sf_hiker_lateral_180n_lcb,
        sf_hunter_lateral_260n_lcb,
        sf_twist_body_1p82nm_lcb,
        sf_hm_twist_16p2nm_lcb,
        sf_solid_tip_torsion_0p80nm_lcb,
        sf_solid_tip_pry_60n_lcb,
        sf_spine_push_500n_lcb,
        sf_spine_baton_3427n_ult_lcb,
        sf_ring_bearing_300n_lcb,
        sf_ring_back_30n_lcb,
        sf_grip_squeeze_250n_lcb,
        sf_tooth_snag_pry_lcb,
        sf_goodman_fatigue_180n_lcb,
        sf_buckling_lambda_lcb,
        sf_combined_lat_twist_lcb,
        sf_per_case_sequence_min_lcb,
        sf_face_wrinkling_lcb,
        gate_margins,
    }
}

/// Feasibility-First Lexicographic Comparator (`SRCH-1`, `UT-32`):
/// 1. Any feasible design (`a.feasible == true`) strictly dominates any infeasible design (`b.feasible == false`).
/// 2. Between two infeasible designs, lower `violation_sum_sq` wins.
/// 3. Between two feasible designs, higher `w_ergolight_v2` (lower mass + higher ergonomic & safety margins) wins.
pub fn compare_g4_objectives(a: &G4ObjectiveReceipt, b: &G4ObjectiveReceipt) -> Ordering {
    match (a.feasible, b.feasible) {
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
        (false, false) => b
            .violation_sum_sq
            .partial_cmp(&a.violation_sum_sq)
            .unwrap_or(Ordering::Equal),
        (true, true) => a
            .w_ergolight_v2
            .partial_cmp(&b.w_ergolight_v2)
            .unwrap_or(Ordering::Equal),
    }
}
