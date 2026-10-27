//! Material model of RS-karambit-v5 `[material]`: stress-relieved L-PBF Ti-6Al-4V at the 80 °C service temperature.
//!
//! Every allowable in G5 comes from here, so no evaluator carries its own yield, modulus, Poisson ratio or knock-down
//! (G3 used 828 MPa wrought plate, E 113.8 GPa, ν 0.34 and a (t − 0.1)/t · 0.912 joint factor).
//!
//! A wall's strength follows the thin-wall law k_t = (1 − (Ts_a + Ts_b)/t)^n, where each of its two faces contributes
//! the rough-layer depth Ts of its surface class (up-skin, down-skin or ground) [xie2026, murchio2024]. The law is
//! fitted to stress on the nominal section, so G5 analyses the nominal geometry and applies k_t once, to the
//! allowable (`[material.thin_wall] stiffness_uses_effective_thickness = false`).

use crate::requirements::rs_v5;
use serde::{Deserialize, Serialize};

/// Surface class of one face of a wall.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Face {
    /// Vertical or up-facing as-built face.
    UpSkin,
    /// As-built face facing down at less than the overhang angle (the rougher class).
    DownSkin,
    /// Ground or machined face (finishing allowance removed).
    Ground,
}

/// The two faces of a wall.
pub type Wall = (Face, Face);

/// An as-built wall of a part printed flat: one up-skin and one down-skin face. Every as-built wall is assumed to
/// have one down-skin face until the decoder declares the build orientation per face.
pub const AS_BUILT_WALL: Wall = (Face::UpSkin, Face::DownSkin);
/// A wall ground on both faces (the finished blade).
pub const GROUND_WALL: Wall = (Face::Ground, Face::Ground);
/// The skin of a hollow blade: ground outside, as-built (assumed down-skin) on the cavity side.
pub const HOLLOW_BLADE_SKIN: Wall = (Face::Ground, Face::DownSkin);

/// Fatigue class of a surface (RS-karambit-v5 `[material.fatigue]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FatigueClass {
    AsBuiltThick,
    AsBuiltThin,
    Ground,
}

/// Rough-layer depth of a face (mm).
pub fn ts_mm(face: Face) -> f64 {
    let tw = &rs_v5().material.thin_wall;
    match face {
        Face::UpSkin => tw.ts_up_skin_mm,
        Face::DownSkin => tw.ts_down_skin_mm,
        Face::Ground => tw.ts_ground_mm,
    }
}

/// Young's modulus at the service temperature (MPa).
pub fn e_mpa() -> f64 {
    let m = &rs_v5().material;
    m.e_gpa * 1000.0 * m.k_temp_modulus
}

/// Shear modulus at the service temperature (MPa).
pub fn g_mpa() -> f64 {
    let m = &rs_v5().material;
    m.g_gpa * 1000.0 * m.k_temp_modulus
}

/// Poisson ratio.
pub fn nu() -> f64 {
    rs_v5().material.nu
}

/// Bulk yield strength at the service temperature (MPa).
pub fn yield_mpa() -> f64 {
    let m = &rs_v5().material;
    m.yield_mpa * m.k_temp_yield
}

/// Bulk ultimate strength at the service temperature (MPa).
pub fn ultimate_mpa() -> f64 {
    let m = &rs_v5().material;
    m.ultimate_mpa * m.k_temp_ultimate
}

/// Density of the mass of record (g/mm³).
pub fn density_g_mm3() -> f64 {
    rs_v5().material.density_g_per_cm3 * 1e-3
}

/// Thin-wall strength factor k_t = (1 − (Ts_a + Ts_b)/t)^n of a wall of nominal thickness `t_mm`, clamped to [0, 1].
pub fn thin_wall_factor(t_mm: f64, wall: Wall, exponent: f64) -> f64 {
    let ts = ts_mm(wall.0) + ts_mm(wall.1);
    let r = 1.0 - ts / t_mm.max(1e-6);
    r.clamp(0.0, 1.0).powf(exponent)
}

/// First-yield allowable (MPa) of a wall of nominal thickness `t_mm` at the service temperature.
pub fn yield_allowable(t_mm: f64, wall: Wall) -> f64 {
    yield_mpa() * thin_wall_factor(t_mm, wall, rs_v5().material.thin_wall.yield_exponent)
}

/// Ultimate allowable (MPa) of a wall of nominal thickness `t_mm` at the service temperature.
pub fn ultimate_allowable(t_mm: f64, wall: Wall) -> f64 {
    ultimate_mpa() * thin_wall_factor(t_mm, wall, rs_v5().material.thin_wall.ultimate_exponent)
}

/// Fatigue class of a wall: ground on both faces, or as-built thick (≥ `thin_threshold_mm`, no down-skin face), or
/// as-built thin. HIP is not part of the G5 configuration (stress-relieved), so `hip_ground_mpa` never applies.
pub fn fatigue_class(t_mm: f64, wall: Wall) -> FatigueClass {
    let f = &rs_v5().material.fatigue;
    if wall == GROUND_WALL {
        FatigueClass::Ground
    } else if t_mm >= f.thin_threshold_mm && wall.0 != Face::DownSkin && wall.1 != Face::DownSkin {
        FatigueClass::AsBuiltThick
    } else {
        FatigueClass::AsBuiltThin
    }
}

/// Fully reversed fatigue strength amplitude at `[material.fatigue] cycles` (MPa) of a class, at room temperature.
pub fn fatigue_strength_mpa(class: FatigueClass) -> f64 {
    let f = &rs_v5().material.fatigue;
    match class {
        FatigueClass::AsBuiltThick => f.as_built_thick_mpa,
        FatigueClass::AsBuiltThin => f.as_built_thin_mpa,
        FatigueClass::Ground => f.ground_mpa,
    }
}

/// FKM equivalent fully reversed amplitude σ_a + M·σ_m (MPa) of a stress cycle with amplitude `sigma_a` and mean
/// `sigma_m`; compressive means get no credit.
pub fn fkm_equivalent_amplitude(sigma_a: f64, sigma_m: f64) -> f64 {
    sigma_a.abs() + rs_v5().material.fatigue.mean_stress_sensitivity * sigma_m.max(0.0)
}

/// Fatigue safety factor of a stress cycle on a surface of the given class. On as-built faces the notch factor is
/// the elastic Kt (`notch_rule = Kf_equals_Kt_on_as_built`); pass the notch stress, already multiplied by Kt.
pub fn fatigue_sf(sigma_a: f64, sigma_m: f64, class: FatigueClass) -> f64 {
    fatigue_strength_mpa(class) / fkm_equivalent_amplitude(sigma_a, sigma_m).max(1e-9)
}

/// Surface area per unit volume of a sheet TPMS, times the cell size (gyroid 3.09, Schwarz-D 3.84), blended by
/// `lat_mix` as the decoder blends the two level sets.
pub fn tpms_area_density_times_cell(lat_mix: f64) -> f64 {
    let m = lat_mix.clamp(0.0, 1.0);
    (1.0 - m) * 3.0936 + m * 3.8377
}

/// Sheet thickness (mm) that gives relative density `rho` in cells of size `cell_mm` (rho = S t / cell), floored at
/// `[am] sheet_min_mm`.
pub fn tpms_sheet_thickness(rho: f64, cell_mm: f64, lat_mix: f64) -> f64 {
    let t = rho.max(0.0) * cell_mm / tpms_area_density_times_cell(lat_mix);
    t.max(rs_v5().am.sheet_min_mm)
}

/// Relative density of a sheet of thickness `t_mm` in cells of size `cell_mm`, capped at 1.
pub fn tpms_density_of_sheet(t_mm: f64, cell_mm: f64, lat_mix: f64) -> f64 {
    (tpms_area_density_times_cell(lat_mix) * t_mm / cell_mm.max(1e-9)).min(1.0)
}

/// Homogenised relative stiffness E*/Es of a sheet TPMS core of relative density `rho` ([material.tpms]: gyroid
/// 0.29 ρ^1.6, Schwarz-D 0.23 ρ^1.2, blended by `lat_mix`; lower bounds until calibrated by FE homogenisation).
pub fn tpms_relative_stiffness(rho: f64, lat_mix: f64) -> f64 {
    let t = &rs_v5().material.tpms;
    let m = lat_mix.clamp(0.0, 1.0);
    let rho = rho.clamp(0.0, 1.0);
    ((1.0 - m) * t.gyroid_stiffness_c * rho.powf(t.gyroid_stiffness_n)
        + m * t.schwarz_d_stiffness_c * rho.powf(t.schwarz_d_stiffness_n))
    .min(1.0)
}

/// Relative stiffness of a zone's core from its density gene: solid (1) at `rho >= 0.95`, otherwise the sheet TPMS
/// law at the density the decoder actually builds (the sheet floor can raise it above the gene).
pub fn core_relative_stiffness(rho_gene: f64, cell_mm: f64, lat_mix: f64) -> f64 {
    if rho_gene >= 0.95 {
        return 1.0;
    }
    let t = tpms_sheet_thickness(rho_gene, cell_mm, lat_mix);
    tpms_relative_stiffness(tpms_density_of_sheet(t, cell_mm, lat_mix), lat_mix)
}

/// Homogenised relative strength σ*/σys of a sheet TPMS core ([material.tpms]: gyroid 0.55 ρ^1.5, Schwarz-D
/// 0.42 ρ^1.14).
pub fn tpms_relative_strength(rho: f64, lat_mix: f64) -> f64 {
    let t = &rs_v5().material.tpms;
    let m = lat_mix.clamp(0.0, 1.0);
    let rho = rho.clamp(0.0, 1.0);
    ((1.0 - m) * t.gyroid_strength_c * rho.powf(t.gyroid_strength_n)
        + m * t.schwarz_d_strength_c * rho.powf(t.schwarz_d_strength_n))
    .min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sheet_tpms_thickness_density_and_laws() {
        // 0.8 mm floor: a 10 % gyroid in 5 mm cells would be 0.16 mm thick, so it is thickened (and denser)
        assert_eq!(tpms_sheet_thickness(0.10, 5.0, 0.0), 0.8);
        assert!((tpms_density_of_sheet(0.8, 5.0, 0.0) - 0.494976).abs() < 1e-6);
        let t = tpms_sheet_thickness(0.4, 8.0, 0.0);
        assert!((tpms_density_of_sheet(t, 8.0, 0.0) - 0.4).abs() < 1e-12);
        assert!((tpms_relative_stiffness(0.3, 0.0) - 0.29 * 0.3_f64.powf(1.6)).abs() < 1e-12);
        assert!((tpms_relative_strength(0.3, 1.0) - 0.42 * 0.3_f64.powf(1.14)).abs() < 1e-12);
    }

    #[test]
    fn service_values_follow_rs_karambit_v5() {
        assert!((e_mpa() - 106_700.0).abs() < 1e-9);
        assert!((g_mpa() - 41_419.0).abs() < 1e-9);
        assert_eq!(nu(), 0.31);
        assert!((yield_mpa() - 750.75).abs() < 1e-9);
        assert!((ultimate_mpa() - 823.4).abs() < 1e-9);
        assert!((density_g_mm3() - 0.00443).abs() < 1e-15);
        assert!((crate::g5_sdf::RHO_TI64_G_MM3 - density_g_mm3()).abs() < 1e-15);
    }

    #[test]
    fn thin_wall_law_counts_each_face_once() {
        // Ground blade: no knock-down. A 0.8 mm as-built wall loses 0.13 mm of rough layer: (1 - 0.13/0.8)^1.8.
        assert_eq!(yield_allowable(0.8, GROUND_WALL), yield_mpa());
        let k = (1.0_f64 - 0.13 / 0.8).powf(1.8);
        assert!((yield_allowable(0.8, AS_BUILT_WALL) - yield_mpa() * k).abs() < 1e-9);
        assert!(yield_allowable(1.5, AS_BUILT_WALL) > yield_allowable(0.8, AS_BUILT_WALL));
        assert_eq!(thin_wall_factor(0.1, AS_BUILT_WALL, 1.8), 0.0);
        // The ultimate exponent is steeper.
        assert!(thin_wall_factor(0.8, AS_BUILT_WALL, 2.1) < thin_wall_factor(0.8, AS_BUILT_WALL, 1.8));
    }

    #[test]
    fn fatigue_classes_and_fkm_mean_stress() {
        assert_eq!(fatigue_class(1.0, GROUND_WALL), FatigueClass::Ground);
        assert_eq!(fatigue_class(2.0, AS_BUILT_WALL), FatigueClass::AsBuiltThin); // has a down-skin face
        assert_eq!(fatigue_class(2.0, (Face::UpSkin, Face::UpSkin)), FatigueClass::AsBuiltThick);
        assert_eq!(fatigue_class(1.0, (Face::UpSkin, Face::UpSkin)), FatigueClass::AsBuiltThin);
        assert_eq!(fatigue_strength_mpa(FatigueClass::Ground), 200.0);
        assert!((fkm_equivalent_amplitude(50.0, 100.0) - 110.0).abs() < 1e-12);
        assert_eq!(fkm_equivalent_amplitude(50.0, -100.0), 50.0);
        assert!((fatigue_sf(100.0, 0.0, FatigueClass::Ground) - 2.0).abs() < 1e-12);
    }
}
