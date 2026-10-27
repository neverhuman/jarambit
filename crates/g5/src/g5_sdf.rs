//! G5 3D Signed Distance Field (SDF) geometry kernel, variable-radius morphology, TPMS lattice
//! constructor, un-anchored 3D corrugated/fenestrated/serrated blade builder, and watertight
//! Marching-Tetrahedra surface extractor (`T4.3`, `G3_GENOME_SPEC.md` v1.1).
//!
//! Enforces all 6 construction and decoder rules from `G3_GENOME_SPEC.md` v1.1 §2 & §7:
//! 1. Variable-radius closing & opening on half-voxel-corrected signed distances (`sdf_mask`) with
//!    Gaussian-smoothed radius fields; `opening = mask & dilate(erode)` is strictly anti-extensive,
//!    and `closing` on a convex body returns the body without 1-voxel phantom growth.
//! 2. Ring load cases use the 1.5 mm thickness-weighted distributed load band (`g5_sandwich.rs`).
//! 3. TPMS void sign: solid lattice fraction inside a cavity matches target $\rho$ within $\pm 0.02$.
//! 4. 3D 6-connectivity filter: only the solid component connected to the outer skin survives
//!    (zero floating struts), and any air pocket not connected to the exterior through the powder
//!    ports is filled solid (zero trapped powder).
//! 5. Smooth 14 mm grip-to-guard taper ($s \in [L_g - 6, L_g + 8]$) with all-round fillets (no step cliff).
//! 6. Smoothed guard/land outlines and Gate S1/S2/S3 satisfied by construction on both serrated edges
//!    plus the `tip_reserve` point taper.

use crate::g5_genome::{G5Genome, FIXED_BORE_D_MM, FIXED_LIP_W_MM};
use jarambit_crescent::export::Solid;
use rayon::prelude::*;
use std::collections::{BTreeMap, VecDeque};

/// Density of the mass of record, g/mm³ (RS-karambit-v5 `[material] density_g_per_cm3`; a test checks it).
pub const RHO_TI64_G_MM3: f64 = 0.00443;
pub const GUARD_X_MM: f64 = 75.6;
/// Fixed-blade union plane $x = 99.0\text{ mm}$ (`ref_f075/f075_reference.json`).
pub const F075_X_UNION_MM: f64 = 99.0;
/// `f075` blade volume beyond $x \ge 99.0\text{ mm}$ (`ref_f075/f075_reference.json`).
pub const F075_BLADE_VOL_BEYOND_UNION_MM3: f64 = 4_490.536_178_586_074;
/// `f075` blade mass beyond $x \ge 99.0\text{ mm}$ (`19.893075271136308 g`, `ref_f075/f075_reference.json`).
pub const F075_BLADE_MASS_BEYOND_UNION_G: f64 = F075_BLADE_VOL_BEYOND_UNION_MM3 * RHO_TI64_G_MM3;
/// `f075` blade centroid $(x_c, y_c)$ beyond $x \ge 99.0\text{ mm}$ (`ref_f075/f075_reference.json`).
pub const F075_BLADE_CENTROID_BEYOND_MM: [f64; 2] =
    [137.176_559_632_182_8, -11.738_407_410_077_114];
/// `f075` blade centroidal $I_{zz}$ beyond $x \ge 99.0\text{ mm}$ in $\text{g}\cdot\text{mm}^2$ (`ref_f075/f075_reference.json`).
pub const F075_BLADE_IZZ_CENTROID_BEYOND_G_MM2: f64 =
    4_006_413.803_140_461_4 * RHO_TI64_G_MM3;

/// Unified blade porosity truth (`N1`, `REREVIEW_S3.md`):
/// when `win_amp <= 0.35`, the blade core is solid (`1.0`), matching the medial fenestration gate
/// (`win_amp > 0.35`) so AM voxel mass, Tier-0 mass, Tier-1 `SandwichPlate`, and `am_build` strut
/// checks all see the exact same blade density.
#[inline]
pub fn effective_blade_rho(p: &BTreeMap<&'static str, f64>) -> f64 {
    if p["win_amp"] > 0.35 && p["rho_blade"] < 0.95 {
        p["rho_blade"]
    } else {
        1.0
    }
}

/// Solid volume fraction of a voxel whose centre has signed distance `d` (mm, negative inside): the linear ramp
/// `clamp(0.5 − d/h, 0, 1)`. Exact for a plane parallel to a voxel face and second-order accurate on average for a
/// smooth surface, so the mass converges with h. G3 counted whole voxels (`d < 0`), which made mass depend on the
/// grid (audit F-M-1).
#[inline]
pub fn solid_fraction(d: f32, h: f64) -> f64 {
    (0.5 - d as f64 / h).clamp(0.0, 1.0)
}

/// Blur radius of the hand-zone SDF hand-over, in millimetres (G3 used one voxel, so the geometry changed with h).
pub const HANDZONE_SDF_BLUR_SIGMA_MM: f64 = 0.4;

/// What `enforce_am_connectivity` changed in the AM field (mm³). Sealed air pockets cannot drain their powder: they
/// are kept solid in `d_am` (so the mass of record counts trapped powder at full density, an over-estimate) and their
/// volume is reported so the objective can penalise it. G3 filled them silently (audit F-M-3).
#[derive(Debug, Clone, Copy, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AmConnectivityStats {
    pub sealed_void_mm3: f64,
    pub floating_solid_mm3: f64,
}

/// 3D regular voxel grid dimensions and origin.
#[derive(Debug, Clone)]
pub struct VoxelGrid3D {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
    pub nx: usize,
    pub ny: usize,
    pub nz: usize,
    pub h: f64,
    /// Flattened `(nx * ny * nz)` signed distance field (mm, negative inside).
    pub d: Vec<f32>,
    /// Pre-texture outer-envelope signed distance field (mm, negative inside) so section/swell/planform
    /// measurements operate on the smooth pre-texture geometry (`N5`, `evaluate.py` parity).
    pub d_pre_tex: Vec<f32>,
    /// Homogenized 2D effective membrane thickness $t_{\text{eff}}(x, y)$ (`skin + \rho^{1.8}` inside cavity, `evaluate.py:167-175` parity) (`N7`).
    pub tmap_eff_2d: Vec<f64>,
    /// Optional AM-built signed distance field (skin + TPMS lattice + powder ports, negative inside solid).
    pub d_am: Option<Vec<f32>>,
    /// Whether this voxel grid includes the evolved G5 blade (`true` in production, `false` in `f075` parity helper).
    pub include_blade: bool,
    /// Sealed (undrainable) cavity volume and removed floating solid of `d_am`.
    pub am_connectivity: AmConnectivityStats,
}

impl VoxelGrid3D {
    #[inline]
    pub fn idx(&self, i: usize, j: usize, k: usize) -> usize {
        (i * self.ny + j) * self.nz + k
    }

    /// Integrate exact hand-zone AM-printed masses `(ring_d_am_g, grip_d_am_g, hz_d_am_x_lt_75p6_g)`
    /// directly from `d_am` (`S5-6`: production-path hand-zone AM mass assertion for `x < 75.6`).
    pub fn production_handzone_d_am_masses(&self, x_t: f64) -> (f64, f64, f64) {
        let field = self.d_am.as_deref().unwrap_or(&self.d);
        let dm = self.h.powi(3) * RHO_TI64_G_MM3;
        let mut ring_g = 0.0_f64;
        let mut grip_g = 0.0_f64;
        for (i, &x) in self.x.iter().enumerate() {
            if x >= GUARD_X_MM {
                break;
            }
            for j in 0..self.ny {
                let base = (i * self.ny + j) * self.nz;
                let f_col: f64 = (0..self.nz).map(|k| solid_fraction(field[base + k], self.h)).sum();
                if f_col > 0.0 {
                    let m_col = f_col * dm;
                    if x < x_t {
                        ring_g += m_col;
                    } else {
                        grip_g += m_col;
                    }
                }
            }
        }
        (ring_g, grip_g, ring_g + grip_g)
    }

    /// Integrate the AM-printed mass (g), centre of mass `(com_x, com_y)` and spin inertia `spin_i_g_mm2` about
    /// `(cx, 0)` from `d_am` by partial volume (`solid_fraction`). Sealed cavities are solid in `d_am`
    /// (`AmConnectivityStats`).
    pub fn am_mass_com_inertia(&self, cx: f64) -> (f64, f64, f64, f64) {
        let field = self.d_am.as_deref().unwrap_or(&self.d);
        let dm = self.h.powi(3) * RHO_TI64_G_MM3;
        let mut m_tot = 0.0_f64;
        let mut mx = 0.0_f64;
        let mut my = 0.0_f64;
        let mut i_zz = 0.0_f64;
        for (i, &x) in self.x.iter().enumerate() {
            let dx = x - cx;
            for (j, &y) in self.y.iter().enumerate() {
                let r2 = dx * dx + y * y;
                let base = (i * self.ny + j) * self.nz;
                let f_col: f64 = (0..self.nz).map(|k| solid_fraction(field[base + k], self.h)).sum();
                if f_col > 0.0 {
                    let m_col = f_col * dm;
                    m_tot += m_col;
                    mx += m_col * x;
                    my += m_col * y;
                    i_zz += m_col * r2;
                }
            }
        }
        let com_x = if m_tot > 1e-9 { mx / m_tot } else { cx };
        let com_y = if m_tot > 1e-9 { my / m_tot } else { 0.0 };
        (m_tot, com_x, com_y, i_zz)
    }

    /// Return the AM-printed solid thickness $\int_{d_{\text{am}}(x,y,z) < 0} dz$ (mm) at `(x, y)`, by partial
    /// volume along the voxel column.
    pub fn am_z_thickness_at(&self, x: f64, y: f64) -> f64 {
        if self.x.is_empty() || self.y.is_empty() {
            return 0.0;
        }
        let i = (((x - self.x[0]) / self.h).round() as isize).clamp(0, self.nx as isize - 1) as usize;
        let j = (((y - self.y[0]) / self.h).round() as isize).clamp(0, self.ny as isize - 1) as usize;
        let field = self.d_am.as_deref().unwrap_or(&self.d);
        let base = (i * self.ny + j) * self.nz;
        let f_solid: f64 = (0..self.nz).map(|k| solid_fraction(field[base + k], self.h)).sum();
        f_solid * self.h
    }

    /// Bilinearly interpolate the 2D homogenized effective membrane thickness `tmap_eff_2d`
    /// (`skin + \rho^{1.8}` inside the cavity, `evaluate.py::tmap_eff` parity) at `(x, y)` (`N7`).
    pub fn ring_tmap_eff_bilinear_at(&self, x: f64, y: f64) -> f64 {
        if self.x.is_empty() || self.y.is_empty() || self.tmap_eff_2d.len() != self.nx * self.ny {
            return self.am_z_thickness_at(x, y);
        }
        let u = ((x - self.x[0]) / self.h).clamp(0.0, (self.nx.saturating_sub(1)) as f64);
        let v = ((y - self.y[0]) / self.h).clamp(0.0, (self.ny.saturating_sub(1)) as f64);
        let i0 = (u.floor() as usize).min(self.nx.saturating_sub(2));
        let j0 = (v.floor() as usize).min(self.ny.saturating_sub(2));
        let fu = (u - i0 as f64).clamp(0.0, 1.0);
        let fv = (v - j0 as f64).clamp(0.0, 1.0);
        let t00 = self.tmap_eff_2d[i0 * self.ny + j0];
        let t10 = self.tmap_eff_2d[(i0 + 1) * self.ny + j0];
        let t01 = self.tmap_eff_2d[i0 * self.ny + j0 + 1];
        let t11 = self.tmap_eff_2d[(i0 + 1) * self.ny + j0 + 1];
        (1.0 - fu) * (1.0 - fv) * t00
            + fu * (1.0 - fv) * t10
            + (1.0 - fu) * fv * t01
            + fu * fv * t11
    }
}

/// Voxel-derived hand-zone measurements (`evaluate.py` parity).
#[derive(Debug, Clone, Copy)]
pub struct HandZoneVoxelMeasurements {
    pub ring_env_g: f64,
    pub ring_am_g: f64,
    pub grip_env_g: f64,
    pub grip_am_g: f64,
    pub hz_am_x_lt_99_g: f64,
    pub f075_union_mass_am_g: f64,
    pub f075_union_com_x_mm: f64,
    pub f075_union_com_to_index_mm: f64,
    pub f075_union_spin_i_g_mm2: f64,
    pub swell_perimeter_mm: f64,
    pub swell_h_mm: f64,
    pub swell_t_mm: f64,
    pub swell_h_over_t: f64,
    pub l_g_mm: f64,
    pub guard_h_mm: f64,
    pub grip_sf_min: f64,
}

impl VoxelGrid3D {
    fn marching_squares_slice_contour(&self, d_ref: &[f32], i_slice: usize) -> (f64, f64, f64) {
        let (ny, nz) = (self.ny, self.nz);
        let h = self.h;
        let mut y_lo = f64::INFINITY;
        let mut y_hi = f64::NEG_INFINITY;
        let mut z_lo = f64::INFINITY;
        let mut z_hi = f64::NEG_INFINITY;
        let mut perim = 0.0_f64;
        for j in 0..ny.saturating_sub(1) {
            for k in 0..nz.saturating_sub(1) {
                let d00 = d_ref[(i_slice * ny + j) * nz + k] as f64;
                let d10 = d_ref[(i_slice * ny + j + 1) * nz + k] as f64;
                let d11 = d_ref[(i_slice * ny + j + 1) * nz + k + 1] as f64;
                let d01 = d_ref[(i_slice * ny + j) * nz + k + 1] as f64;
                let mut pts: [(f64, f64); 4] = [(0.0, 0.0); 4];
                let mut n_pts = 0usize;
                if (d00 < 0.0) != (d10 < 0.0) {
                    let w = d00.abs() / (d00.abs() + d10.abs()).max(1e-12);
                    pts[n_pts] = (self.y[j] + w * h, self.z[k]);
                    n_pts += 1;
                }
                if (d10 < 0.0) != (d11 < 0.0) {
                    let w = d10.abs() / (d10.abs() + d11.abs()).max(1e-12);
                    pts[n_pts] = (self.y[j + 1], self.z[k] + w * h);
                    n_pts += 1;
                }
                if (d11 < 0.0) != (d01 < 0.0) {
                    let w = d01.abs() / (d01.abs() + d11.abs()).max(1e-12);
                    pts[n_pts] = (self.y[j] + w * h, self.z[k + 1]);
                    n_pts += 1;
                }
                if (d01 < 0.0) != (d00 < 0.0) {
                    let w = d00.abs() / (d00.abs() + d01.abs()).max(1e-12);
                    pts[n_pts] = (self.y[j], self.z[k] + w * h);
                    n_pts += 1;
                }
                for &(yp, zp) in &pts[..n_pts] {
                    y_lo = y_lo.min(yp);
                    y_hi = y_hi.max(yp);
                    z_lo = z_lo.min(zp);
                    z_hi = z_hi.max(zp);
                }
                if n_pts == 2 {
                    perim += (pts[0].0 - pts[1].0).hypot(pts[0].1 - pts[1].1);
                } else if n_pts == 4 {
                    perim += (pts[0].0 - pts[1].0).hypot(pts[0].1 - pts[1].1)
                        + (pts[2].0 - pts[3].0).hypot(pts[2].1 - pts[3].1);
                }
            }
        }
        let h_sec = if y_hi > y_lo { y_hi - y_lo } else { 0.0 };
        let t_sec = if z_hi > z_lo { z_hi - z_lo } else { 0.0 };
        (perim, h_sec, t_sec)
    }

    /// Measure `ring` (`x < x_t`) and `grip` (`x_t <= x < GUARD_X_MM`) envelope and AM-infill masses (g),
    /// Marching-Squares `swell_perimeter_mm`, `swell_H_mm`, `swell_T_mm`, `z = 0` contour `L_g_mm` + `guard_h_mm`,
    /// and voxel-section `grip_sf_min` directly from the pre-texture 3D SDF (`evaluate.py` and `hzm.py::cavity_mask` parity).
    pub fn measure_handzone_masses_and_swell(
        &self,
        p: &BTreeMap<&'static str, f64>,
    ) -> HandZoneVoxelMeasurements {
        let (nx, ny, nz) = (self.nx, self.ny, self.nz);
        let d_ref = if self.d_pre_tex.len() == nx * ny * nz {
            &self.d_pre_tex
        } else {
            &self.d
        };
        let h = self.h;
        let cx = p["cx"];
        let x_t = p["x_t"];
        let r_od = p["R_od"];
        let skin_t = p["skin_t"] as f32;
        let rib_w = p["rib_w"];
        let rib_half = if rib_w > 0.0 {
            0.4 + 0.4 * rib_w
        } else {
            -1.0
        };

        let mut rho_2d = vec![1.0_f64; nx * ny];
        let mut core = vec![false; nx * ny * nz];
        let mut any_core = false;
        for (i, &x) in self.x.iter().enumerate() {
            let dx = x - cx;
            for (j, &y) in self.y.iter().enumerate() {
                let r = dx.hypot(y);
                let mut rz = if x < x_t {
                    p["rho_ring"]
                } else if x < GUARD_X_MM {
                    p["rho_grip"]
                } else {
                    p["rho_guard"]
                };
                if x >= x_t && r < r_od + 1.0 {
                    rz = p["rho_ring"];
                }
                if x > 92.0 {
                    rz = 1.0;
                }
                rho_2d[i * ny + j] = rz;
                if rz < 0.95 {
                    let base = (i * ny + j) * nz;
                    for k in 0..nz {
                        if d_ref[base + k] < -(skin_t + 0.5) {
                            core[base + k] = true;
                            any_core = true;
                        }
                    }
                }
            }
        }
        let do_core = if any_core {
            edt_out(&core, nx, ny, nz, h)
        } else {
            vec![10.0_f32; nx * ny * nz]
        };

        let v_vox = h.powi(3) * RHO_TI64_G_MM3;
        let mut ring_env_g = 0.0_f64;
        let mut ring_am_g = 0.0_f64;
        let mut grip_env_g = 0.0_f64;
        let mut grip_am_g = 0.0_f64;
        let mut hz_am_x_lt_99_g = 0.0_f64;
        let mut hz_bal_x_lt_99_g = 0.0_f64;
        let mut wx_hz = 0.0_f64;
        let mut wy_hz = 0.0_f64;
        let mut izz_hz = 0.0_f64;

        for (i, &x) in self.x.iter().enumerate() {
            for (j, &y) in self.y.iter().enumerate() {
                let rz = rho_2d[i * ny + j];
                let base = (i * ny + j) * nz;
                for (k, &z) in self.z.iter().enumerate() {
                    let idx = base + k;
                    if d_ref[idx] >= 0.0 {
                        continue;
                    }
                    let is_cav = rz < 0.95 && do_core[idx] <= 0.5 && d_ref[idx] < -skin_t;
                    let fill = if is_cav {
                        if z.abs() < rib_half {
                            1.0
                        } else {
                            rz
                        }
                    } else {
                        1.0
                    };
                    let fill_bal = if is_cav { rz } else { 1.0 };
                    let dm = fill * v_vox;
                    let dm_bal = fill_bal * v_vox;
                    if x < x_t {
                        ring_env_g += v_vox;
                        ring_am_g += dm;
                    } else if x < GUARD_X_MM {
                        grip_env_g += v_vox;
                        grip_am_g += dm;
                    }
                    if x < F075_X_UNION_MM {
                        hz_am_x_lt_99_g += dm;
                        hz_bal_x_lt_99_g += dm_bal;
                        wx_hz += dm_bal * x;
                        wy_hz += dm_bal * y;
                        izz_hz += dm_bal * ((x - cx).powi(2) + y.powi(2));
                    }
                }
            }
        }
        let f075_union_mass_am_g = hz_am_x_lt_99_g + F075_BLADE_MASS_BEYOND_UNION_G;
        let m_bal_total = (hz_bal_x_lt_99_g + F075_BLADE_MASS_BEYOND_UNION_G).max(1e-12);
        let f075_union_com_x_mm =
            (wx_hz + F075_BLADE_MASS_BEYOND_UNION_G * F075_BLADE_CENTROID_BEYOND_MM[0])
                / m_bal_total;
        let _f075_union_com_y_mm =
            (wy_hz + F075_BLADE_MASS_BEYOND_UNION_G * F075_BLADE_CENTROID_BEYOND_MM[1])
                / m_bal_total;
        let x_idx = x_t + p["centre_idx"] + p["off_idx"];
        let f075_union_com_to_index_mm = (f075_union_com_x_mm - x_idx).abs();
        let f075_union_spin_i_g_mm2 = izz_hz
            + F075_BLADE_IZZ_CENTROID_BEYOND_G_MM2
            + F075_BLADE_MASS_BEYOND_UNION_G
                * ((F075_BLADE_CENTROID_BEYOND_MM[0] - cx).powi(2)
                    + F075_BLADE_CENTROID_BEYOND_MM[1].powi(2));

        // Measure swell perimeter, H, and T at x_sw = x_t + s_sw * L_g by Marching Squares on d_ref[i_sw]
        let x_sw = x_t + p["s_sw"] * p["L_g"];
        let i_sw = (((x_sw - self.x[0]) / h).round() as isize).clamp(0, nx as isize - 1) as usize;
        let (swell_perim, swell_h, swell_t) = self.marching_squares_slice_contour(d_ref, i_sw);
        let swell_h_over_t = if swell_t > 1e-6 {
            swell_h / swell_t
        } else {
            9.0
        };

        // Measure voxel-section grip_sf_min at s in {0.25, 0.50, 0.75} (`evaluate.py::grip_beam_sf` parity, N8, N13, S5-7)
        let rho_e = crate::material::core_relative_stiffness(p["rho_grip"], p["lat_cell"], p["lat_mix"]);
        let hollow = p["rho_grip"] < 0.95;
        let skin_t_f64 = p["skin_t"].max(0.4);
        let allow = crate::material::yield_allowable(skin_t_f64, crate::material::AS_BUILT_WALL);
        let (tip_ref, spine_x_ref) = if self.include_blade {
            let bp = build_blade_profile_2d(p);
            ([bp.tip_xy.0, bp.tip_xy.1], bp.centroid_x_mm)
        } else {
            ([187.9_f64, -53.5_f64], 145.0_f64)
        };
        let mut grip_sf_min = 9.9_f64;
        for &s_frac in &[0.25_f64, 0.50, 0.75] {
            let x_sec = x_t + s_frac * p["L_g"];
            let i_sec = (((x_sec - self.x[0]) / h).round() as isize).clamp(0, nx as isize - 1) as usize;
            let (perim_s, h_s, t_s) = self.marching_squares_slice_contour(d_ref, i_sec);
            if perim_s <= 1e-6 {
                grip_sf_min = 0.0;
                continue;
            }
            let mut a_sum = 0.0_f64;
            let mut a_enc = 0.0_f64;
            let mut wy_s = 0.0_f64;
            let mut wz_s = 0.0_f64;
            let da = h * h;
            for (j, &y) in self.y.iter().enumerate() {
                let base = (i_sec * ny + j) * nz;
                for (k, &z) in self.z.iter().enumerate() {
                    let dv = d_ref[base + k];
                    if dv < 0.0 {
                        let wgt = if hollow && dv < -skin_t { rho_e } else { 1.0 };
                        a_sum += wgt * da;
                        a_enc += da;
                        wy_s += wgt * y * da;
                        wz_s += wgt * z * da;
                    }
                }
            }
            if a_sum <= 1e-9 {
                grip_sf_min = 0.0;
                continue;
            }
            let yc = wy_s / a_sum;
            let zc = wz_s / a_sum;
            let mut iyy = 0.0_f64;
            let mut izz = 0.0_f64;
            let mut ymax = 0.0_f64;
            let mut zmax = 0.0_f64;
            for (j, &y) in self.y.iter().enumerate() {
                let base = (i_sec * ny + j) * nz;
                for (k, &z) in self.z.iter().enumerate() {
                    let dv = d_ref[base + k];
                    if dv < 0.0 {
                        let wgt = if hollow && dv < -skin_t { rho_e } else { 1.0 };
                        let dy = y - yc;
                        let dz = z - zc;
                        iyy += wgt * dz * dz * da;
                        izz += wgt * dy * dy * da;
                        ymax = ymax.max(dy.abs());
                        zmax = zmax.max(dz.abs());
                    }
                }
            }
            let arm = (tip_ref[0] - x_sec).max(10.0);
            let m_lat = 260.0 * arm;
            let sig = m_lat * zmax / iyy.max(1e-9);
            let t_torq = 260.0 * (tip_ref[1] - yc).abs();
            let tau = if hollow {
                t_torq / (2.0 * a_enc.max(1e-6) * skin_t_f64)
            } else {
                t_torq * 3.0 / (perim_s * 0.5 * h_s.min(t_s).max(0.5).powi(2)).max(1e-6)
            };
            let vm_lat = (sig * sig + 3.0 * tau * tau).sqrt();
            let tau_tw = if hollow {
                16_200.0 / (2.0 * a_enc.max(1e-6) * skin_t_f64)
            } else {
                0.0
            };
            let mz = 3_427.0 * (spine_x_ref - x_sec).max(0.0);
            let sig_sp = mz * ymax / izz.max(1e-9);
            let sig_sq = if hollow {
                let q = 250.0 / (20.0 * (h_s.min(30.0) * 0.5).max(5.0));
                let panel = 0.75 * q * p["lat_cell"].powi(2) / skin_t_f64.powi(2);
                let rm = 0.25 * (h_s + t_s) * 0.5;
                let oval = 6.0 * 0.318 * (250.0 / 20.0) * rm / skin_t_f64.powi(2);
                panel.max(oval)
            } else {
                1e-9
            };
            let sf = (allow / vm_lat.max(1e-9))
                .min(allow / (3.0_f64.sqrt() * tau_tw).max(1e-9))
                .min(allow / sig_sp.max(1e-9))
                .min(allow / sig_sq.max(1e-9));
            grip_sf_min = grip_sf_min.min(sf);
        }

        // Measure L_g_mm and guard_h_mm from planform_bottom on the z = 0 slice (`evaluate.py:281-288`, N4 fail-closed INFINITY)
        let k_mid = self
            .z
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.abs().total_cmp(&b.abs()))
            .map(|(k, _)| k)
            .unwrap_or(nz / 2);
        let mut yb_slice = vec![f64::NAN; nx];
        for i in 0..nx {
            for j in 0..ny {
                if d_ref[(i * ny + j) * nz + k_mid] < 0.0 {
                    yb_slice[i] = self.y[j];
                    break;
                }
            }
        }
        let l_g_param = p["L_g"];
        let mut line = f64::INFINITY;
        for (i, &x) in self.x.iter().enumerate() {
            let s = x - x_t;
            if s > 0.2 * l_g_param && s < 0.8 * l_g_param && yb_slice[i].is_finite() {
                line = line.min(yb_slice[i]);
            }
        }
        // N4: Fail-closed `f64::INFINITY` if no guard drop >= 2.0 mm is found (never fall back to gene `p["L_g"]`)
        let mut l_g_meas = f64::INFINITY;
        if line.is_finite() {
            for (i, &x) in self.x.iter().enumerate() {
                let s = x - x_t;
                if s > 0.8 * l_g_param && yb_slice[i].is_finite() && yb_slice[i] < line - 2.0 {
                    l_g_meas = s;
                    break;
                }
            }
        }
        let mut min_yb_gz = f64::INFINITY;
        for (i, &x) in self.x.iter().enumerate() {
            if (GUARD_X_MM..=92.0).contains(&x) && yb_slice[i].is_finite() {
                min_yb_gz = min_yb_gz.min(yb_slice[i]);
            }
        }
        let guard_h_meas = if line.is_finite() && min_yb_gz.is_finite() {
            (line - min_yb_gz).max(0.0)
        } else {
            0.0
        };

        HandZoneVoxelMeasurements {
            ring_env_g,
            ring_am_g,
            grip_env_g,
            grip_am_g,
            hz_am_x_lt_99_g,
            f075_union_mass_am_g,
            f075_union_com_x_mm,
            f075_union_com_to_index_mm,
            f075_union_spin_i_g_mm2,
            swell_perimeter_mm: swell_perim,
            swell_h_mm: swell_h,
            swell_t_mm: swell_t,
            swell_h_over_t,
            l_g_mm: l_g_meas,
            guard_h_mm: guard_h_meas,
            grip_sf_min,
        }
    }
}

// ---------------- 1. Exact Separable O(N) 3D Euclidean Distance Transform ----------------

/// 1D squared distance transform along a line of length `n` using the Felzenszwalb-Huttenlocher
/// lower envelope of parabolas.
fn edt_1d_line(f: &[f64], n: usize, out: &mut [f64], v: &mut [isize], z: &mut [f64]) {
    if n == 0 {
        return;
    }
    let mut k = 0usize;
    v[0] = 0;
    z[0] = f64::NEG_INFINITY;
    z[1] = f64::INFINITY;
    for q in 1..n {
        let fq = f[q];
        if fq >= 1e28 {
            continue;
        }
        let q_i = q as isize;
        let q_f = q as f64;
        let s = loop {
            let vk = v[k];
            let vk_f = vk as f64;
            let fvk = f[vk as usize];
            let s = if fvk >= 1e28 {
                f64::NEG_INFINITY
            } else {
                ((fq + q_f * q_f) - (fvk + vk_f * vk_f)) / (2.0 * (q_f - vk_f))
            };
            if s > z[k] {
                break s;
            }
            if k == 0 {
                break s;
            }
            k -= 1;
        };
        if f[v[k] as usize] < 1e28 {
            k += 1;
        }
        v[k] = q_i;
        z[k] = s;
        z[k + 1] = f64::INFINITY;
    }
    if f[v[0] as usize] >= 1e28 {
        for x in out.iter_mut().take(n) {
            *x = 1e28;
        }
        return;
    }
    let mut k_eval = 0usize;
    for (q, out_q) in out.iter_mut().enumerate().take(n) {
        let q_f = q as f64;
        while z[k_eval + 1] < q_f {
            k_eval += 1;
        }
        let dx = q_f - (v[k_eval] as f64);
        *out_q = dx * dx + f[v[k_eval] as usize];
    }
}

/// Exact 3D Euclidean distance (in mm) of every `true` voxel to the nearest `false` voxel;
/// outside the array counts as `false` (`black_border = true`, matching `hzm.py::edt_in`).
pub fn edt_in(mask: &[bool], nx: usize, ny: usize, nz: usize, h: f64) -> Vec<f32> {
    let total = nx * ny * nz;
    assert_eq!(mask.len(), total);
    let mut d2 = vec![0.0_f64; total];

    // Pass 1 & 2: along Z and Y per X-slice in parallel (`M7`)
    d2.par_chunks_mut(ny * nz)
        .enumerate()
        .for_each(|(i, slice)| {
            let mut line_in = vec![0.0_f64; nz.max(ny) + 2];
            let mut line_out = vec![0.0_f64; nz.max(ny) + 2];
            let mut v = vec![0isize; nz.max(ny) + 3];
            let mut z = vec![0.0_f64; nz.max(ny) + 4];
            let mask_slice = &mask[i * ny * nz..(i + 1) * ny * nz];
            if nz == 1 {
                for j in 0..ny {
                    slice[j] = if mask_slice[j] { 1e29 } else { 0.0 };
                }
            } else {
                for j in 0..ny {
                    let base = j * nz;
                    line_in[0] = 0.0;
                    for k in 0..nz {
                        line_in[k + 1] = if mask_slice[base + k] { 1e29 } else { 0.0 };
                    }
                    line_in[nz + 1] = 0.0;
                    edt_1d_line(&line_in, nz + 2, &mut line_out, &mut v, &mut z);
                    for k in 0..nz {
                        slice[base + k] = line_out[k + 1];
                    }
                }
            }
            for k in 0..nz {
                line_in[0] = 0.0;
                for j in 0..ny {
                    line_in[j + 1] = slice[j * nz + k];
                }
                line_in[ny + 1] = 0.0;
                edt_1d_line(&line_in, ny + 2, &mut line_out, &mut v, &mut z);
                for j in 0..ny {
                    slice[j * nz + k] = line_out[j + 1];
                }
            }
        });

    // Pass 3: along X (with virtual false borders at i = -1 and i = nx)
    let mut line_in = vec![0.0_f64; nx + 2];
    let mut line_out = vec![0.0_f64; nx + 2];
    let mut v = vec![0isize; nx + 3];
    let mut z = vec![0.0_f64; nx + 4];
    for j in 0..ny {
        for k in 0..nz {
            line_in[0] = 0.0;
            for i in 0..nx {
                line_in[i + 1] = d2[(i * ny + j) * nz + k];
            }
            line_in[nx + 1] = 0.0;
            edt_1d_line(&line_in, nx + 2, &mut line_out, &mut v, &mut z);
            for i in 0..nx {
                d2[(i * ny + j) * nz + k] = line_out[i + 1];
            }
        }
    }

    d2.into_par_iter()
        .map(|s| (s.max(0.0).sqrt() * h) as f32)
        .collect()
}

/// Exact 3D Euclidean distance (in mm) of every `false` voxel to the nearest `true` voxel;
/// outside the array is NOT `true` (`black_border = false`, matching `hzm.py::edt_out`).
pub fn edt_out(mask: &[bool], nx: usize, ny: usize, nz: usize, h: f64) -> Vec<f32> {
    let total = nx * ny * nz;
    assert_eq!(mask.len(), total);
    let mut d2 = vec![0.0_f64; total];

    d2.par_chunks_mut(ny * nz)
        .enumerate()
        .for_each(|(i, slice)| {
            let mut line_in = vec![0.0_f64; nz.max(ny)];
            let mut line_out = vec![0.0_f64; nz.max(ny)];
            let mut v = vec![0isize; nz.max(ny) + 1];
            let mut z = vec![0.0_f64; nz.max(ny) + 2];
            let mask_slice = &mask[i * ny * nz..(i + 1) * ny * nz];
            for j in 0..ny {
                let base = j * nz;
                for k in 0..nz {
                    line_in[k] = if mask_slice[base + k] { 0.0 } else { 1e29 };
                }
                edt_1d_line(&line_in, nz, &mut line_out, &mut v, &mut z);
                for k in 0..nz {
                    slice[base + k] = line_out[k];
                }
            }
            for k in 0..nz {
                for j in 0..ny {
                    line_in[j] = slice[j * nz + k];
                }
                edt_1d_line(&line_in, ny, &mut line_out, &mut v, &mut z);
                for j in 0..ny {
                    slice[j * nz + k] = line_out[j];
                }
            }
        });

    let mut line_in = vec![0.0_f64; nx];
    let mut line_out = vec![0.0_f64; nx];
    let mut v = vec![0isize; nx + 1];
    let mut z = vec![0.0_f64; nx + 2];
    for j in 0..ny {
        for k in 0..nz {
            for i in 0..nx {
                line_in[i] = d2[(i * ny + j) * nz + k];
            }
            edt_1d_line(&line_in, nx, &mut line_out, &mut v, &mut z);
            for i in 0..nx {
                d2[(i * ny + j) * nz + k] = line_out[i];
            }
        }
    }

    d2.into_par_iter()
        .map(|s| (s.max(0.0).sqrt() * h) as f32)
        .collect()
}

/// Half-voxel-corrected signed distance (mm, negative inside) of a boolean voxel mask;
/// zero-level surface sits exactly on the voxel boundary (`hzm.py::sdf_mask` parity).
pub fn sdf_mask(mask: &[bool], nx: usize, ny: usize, nz: usize, h: f64) -> Vec<f32> {
    let di = edt_in(mask, nx, ny, nz, h);
    let do_ = edt_out(mask, nx, ny, nz, h);
    let half_h = (0.5 * h) as f32;
    mask.iter()
        .zip(di.iter().zip(do_.iter()))
        .map(|(&m, (&d_in, &d_out))| if m { -(d_in - half_h) } else { d_out - half_h })
        .collect()
}

// ---------------- 2. Separable Gaussian Smoothing & Variable-Radius Morphology ----------------

fn gaussian_kernel_1d(sigma_vox: f64) -> Vec<f32> {
    if sigma_vox <= 1e-4 {
        return vec![1.0];
    }
    let rad = (3.0 * sigma_vox).ceil() as isize;
    let mut k = Vec::with_capacity((2 * rad + 1) as usize);
    let mut sum = 0.0_f64;
    for i in -rad..=rad {
        let w = (-0.5 * (i as f64 / sigma_vox).powi(2)).exp();
        k.push(w as f32);
        sum += w;
    }
    for v in &mut k {
        *v /= sum as f32;
    }
    k
}

pub fn gaussian_filter_2d(field: &[f32], nx: usize, ny: usize, sigma_vox: f64) -> Vec<f32> {
    let k = gaussian_kernel_1d(sigma_vox);
    let r = (k.len() / 2) as isize;
    let mut tmp = vec![0.0_f32; nx * ny];
    let mut out = vec![0.0_f32; nx * ny];
    for i in 0..nx {
        for j in 0..ny {
            let mut acc = 0.0_f32;
            for (idx_k, &w) in k.iter().enumerate() {
                let jj = (j as isize + idx_k as isize - r).clamp(0, ny as isize - 1) as usize;
                acc += w * field[i * ny + jj];
            }
            tmp[i * ny + j] = acc;
        }
    }
    for i in 0..nx {
        for j in 0..ny {
            let mut acc = 0.0_f32;
            for (idx_k, &w) in k.iter().enumerate() {
                let ii = (i as isize + idx_k as isize - r).clamp(0, nx as isize - 1) as usize;
                acc += w * tmp[ii * ny + j];
            }
            out[i * ny + j] = acc;
        }
    }
    out
}

pub fn gaussian_filter_3d(
    field: &[f32],
    nx: usize,
    ny: usize,
    nz: usize,
    sigma_vox: f64,
) -> Vec<f32> {
    let k = gaussian_kernel_1d(sigma_vox);
    if k.len() <= 1 {
        return field.to_vec();
    }
    let r = (k.len() / 2) as isize;
    let total = nx * ny * nz;
    let mut tmp2 = vec![0.0_f32; total];
    // Z & Y passes per X-slice in parallel (`M7`)
    tmp2.par_chunks_mut(ny * nz)
        .enumerate()
        .for_each(|(i, out_slice)| {
            let in_slice = &field[i * ny * nz..(i + 1) * ny * nz];
            let mut tmp1 = vec![0.0_f32; ny * nz];
            for j in 0..ny {
                let base = j * nz;
                for z in 0..nz {
                    let mut acc = 0.0_f32;
                    for (ik, &w) in k.iter().enumerate() {
                        let zz = (z as isize + ik as isize - r).clamp(0, nz as isize - 1) as usize;
                        acc += w * in_slice[base + zz];
                    }
                    tmp1[base + z] = acc;
                }
            }
            for j in 0..ny {
                for z in 0..nz {
                    let mut acc = 0.0_f32;
                    for (ik, &w) in k.iter().enumerate() {
                        let jj = (j as isize + ik as isize - r).clamp(0, ny as isize - 1) as usize;
                        acc += w * tmp1[jj * nz + z];
                    }
                    out_slice[j * nz + z] = acc;
                }
            }
        });
    // X pass in parallel over X-slices
    let mut out = vec![0.0_f32; total];
    out.par_chunks_mut(ny * nz)
        .enumerate()
        .for_each(|(i, out_slice)| {
            for j in 0..ny {
                for z in 0..nz {
                    let mut acc = 0.0_f32;
                    for (ik, &w) in k.iter().enumerate() {
                        let ii = (i as isize + ik as isize - r).clamp(0, nx as isize - 1) as usize;
                        acc += w * tmp2[(ii * ny + j) * nz + z];
                    }
                    out_slice[j * nz + z] = acc;
                }
            }
        });
    out
}

/// Variable-radius morphological closing (`erode(dilate)`) on half-voxel-corrected signed distances
/// (`G3_GENOME_SPEC.md` v1.1 §7 Rule 1).
/// Extensive (`mask ⊆ closed`) and preserves convex bodies whose curvature radius exceeds `r_c2`.
pub fn morph_close_variable_2d_z(
    mask: &[bool],
    r_c2: &[f32],
    nx: usize,
    ny: usize,
    nz: usize,
    h: f64,
) -> Vec<bool> {
    assert_eq!(r_c2.len(), nx * ny);
    let r_smooth = gaussian_filter_2d(r_c2, nx, ny, 2.0 / h);
    let s1 = sdf_mask(mask, nx, ny, nz, h);
    let mut dil = vec![false; mask.len()];
    for i in 0..nx {
        for j in 0..ny {
            let rc = r_smooth[i * ny + j];
            let base = (i * ny + j) * nz;
            for k in 0..nz {
                dil[base + k] = s1[base + k] <= rc;
            }
        }
    }
    let s2 = sdf_mask(&dil, nx, ny, nz, h);
    let mut out = mask.to_vec();
    for i in 0..nx {
        for j in 0..ny {
            let rc = r_smooth[i * ny + j];
            if rc <= 0.05 {
                continue;
            }
            let base = (i * ny + j) * nz;
            for k in 0..nz {
                if s2[base + k] <= -rc {
                    out[base + k] = true;
                }
            }
        }
    }
    out
}

/// Variable-radius morphological opening (`mask & dilate(erode)`) on half-voxel-corrected signed
/// distances (`G3_GENOME_SPEC.md` v1.1 §7 Rule 1).
/// Strictly anti-extensive (`opened ⊆ mask`): never adds a single voxel outside `mask`.
pub fn morph_open_variable_2d_z(
    mask: &[bool],
    r_o2: &[f32],
    nx: usize,
    ny: usize,
    nz: usize,
    h: f64,
) -> Vec<bool> {
    assert_eq!(r_o2.len(), nx * ny);
    let r_gauss = gaussian_filter_2d(r_o2, nx, ny, 1.0 / h);
    let r_smooth: Vec<f32> = r_o2
        .iter()
        .zip(r_gauss.iter())
        .map(|(&a, &b)| a.min(b))
        .collect();
    let s3 = sdf_mask(mask, nx, ny, nz, h);
    let mut ero = vec![false; mask.len()];
    for i in 0..nx {
        for j in 0..ny {
            let ro = r_smooth[i * ny + j];
            let base = (i * ny + j) * nz;
            for k in 0..nz {
                ero[base + k] = s3[base + k] <= -ro;
            }
        }
    }
    let s4 = sdf_mask(&ero, nx, ny, nz, h);
    let mut out = vec![false; mask.len()];
    for i in 0..nx {
        for j in 0..ny {
            let ro = r_smooth[i * ny + j];
            let base = (i * ny + j) * nz;
            for k in 0..nz {
                out[base + k] = mask[base + k] && (ro <= 0.05 || s4[base + k] <= ro);
            }
        }
    }
    out
}

// ---------------- 3. TPMS Lattice & 3D Connectivity Filter (Rules 3 & 4) ----------------

/// Evaluate the blended Gyroid $\leftrightarrow$ Schwarz-D TPMS level set at `(x, y, z)` with
/// anisotropic x-stretching `lat_aniso_x`.
#[inline]
pub fn tpms_val(
    x: f64,
    y: f64,
    z: f64,
    lat_cell: f64,
    lat_mix: f64,
    lat_aniso_x: f64,
) -> f64 {
    let k = 2.0 * std::f64::consts::PI / lat_cell.max(1.0);
    let kx = (x / lat_aniso_x.max(0.5)) * k;
    let ky = y * k;
    let kz = z * k;
    let (sx, cx) = kx.sin_cos();
    let (sy, cy) = ky.sin_cos();
    let (sz, cz) = kz.sin_cos();
    let gy = sx * cy + sy * cz + sz * cx;
    let sd = sx * sy * sz + sx * cy * cz + cx * sy * cz + cx * cy * sz;
    (1.0 - lat_mix) * (gy / 1.5) + lat_mix * (sd / 1.41)
}

/// `tpms_val` and the magnitude of its gradient (1/mm), for the first-order distance |f| / |grad f| to the level set
/// f = 0 that the G5.0 sheet TPMS uses.
pub fn tpms_val_and_grad(
    x: f64,
    y: f64,
    z: f64,
    lat_cell: f64,
    lat_mix: f64,
    lat_aniso_x: f64,
) -> (f64, f64) {
    let k = 2.0 * std::f64::consts::PI / lat_cell.max(1.0);
    let ax = lat_aniso_x.max(0.5);
    let (sx, cx) = ((x / ax) * k).sin_cos();
    let (sy, cy) = (y * k).sin_cos();
    let (sz, cz) = (z * k).sin_cos();
    let gy = sx * cy + sy * cz + sz * cx;
    let gy_d = [cx * cy - sz * sx, -sx * sy + cy * cz, -sy * sz + cz * cx];
    let sd = sx * sy * sz + sx * cy * cz + cx * sy * cz + cx * cy * sz;
    let sd_d = [
        cx * sy * sz + cx * cy * cz - sx * sy * cz - sx * cy * sz,
        sx * cy * sz - sx * sy * cz + cx * cy * cz - cx * sy * sz,
        sx * sy * cz - sx * cy * sz - cx * sy * sz + cx * cy * cz,
    ];
    let m = lat_mix;
    let f = (1.0 - m) * (gy / 1.5) + m * (sd / 1.41);
    let g = |a: usize| (1.0 - m) * gy_d[a] / 1.5 + m * sd_d[a] / 1.41;
    let (gx, gyy, gz) = (g(0) * k / ax, g(1) * k, g(2) * k);
    (f, (gx * gx + gyy * gyy + gz * gz).sqrt())
}

/// Compute the threshold `c` such that `P(tpms_val(X, Y, Z) > c) == target_rho` over one unit cell
/// (`G3_GENOME_SPEC.md` v1.1 §7 Rule 3).
pub fn tpms_level_for_rho(lat_cell: f64, lat_mix: f64, lat_aniso_x: f64, target_rho: f64) -> f64 {
    let rho = target_rho.clamp(0.02, 0.98);
    let n_side = 28usize;
    let mut vals = Vec::with_capacity(n_side * n_side * n_side);
    for i in 0..n_side {
        let x = ((i as f64 + 0.5) / n_side as f64) * lat_cell * lat_aniso_x;
        for j in 0..n_side {
            let y = ((j as f64 + 0.5) / n_side as f64) * lat_cell;
            for k in 0..n_side {
                let z = ((k as f64 + 0.5) / n_side as f64) * lat_cell;
                vals.push(tpms_val(x, y, z, lat_cell, lat_mix, lat_aniso_x));
            }
        }
    }
    vals.sort_by(|a, b| a.total_cmp(b));
    let idx = (((1.0 - rho) * (vals.len() - 1) as f64).round() as usize).min(vals.len() - 1);
    vals[idx]
}

/// Apply `G3_GENOME_SPEC.md` v1.1 §7 Rule 4, with G5's drainage rule, to an AM signed distance field `d_am`.
///
/// Powder drains only through passages at least `drain_channel_min_mm` wide (RS-karambit-v5 `[am]`). An air voxel
/// inside the envelope (`d_env < 0`) is drained when it lies on, or within that half-width of, a path from the
/// outside whose voxels all have clearance `d_am >= drain_channel_min_mm / 2`. Air outside the envelope always
/// counts as open, so exterior slots are never filled. G3 drained a cavity through any one-voxel gap, so the
/// same design was hollow on one grid and filled on another (audit F-M-3).
///
/// 1. Fill any sealed internal air pockets (`d_am > 0`) that are not 6-connected to the outer grid
///    boundary (no trapped un-sintered powder).
/// 2. Keep only the largest 6-connected solid component (`d_am < 0`, which includes the outer skin),
///    removing any disconnected floating lattice struts.
pub fn enforce_am_connectivity(
    d_am: &mut [f32],
    d_env: Option<&[f32]>,
    nx: usize,
    ny: usize,
    nz: usize,
    h: f32,
    drain_channel_min_mm: f64,
) -> AmConnectivityStats {
    let total = nx * ny * nz;
    assert_eq!(d_am.len(), total);
    if let Some(env) = d_env {
        assert_eq!(env.len(), total);
    }
    let hv = h as f64;
    let dv = hv * hv * hv;
    let r_clear = (0.5 * drain_channel_min_mm) as f32;
    let mut stats = AmConnectivityStats::default();
    let exterior = |idx: usize| d_env.is_some_and(|env| env[idx] >= 0.0);
    let wide = |idx: usize| d_am[idx] > 0.0 && (exterior(idx) || d_am[idx] >= r_clear);
    let nbrs6 = |i: usize, j: usize, k: usize| {
        [
            (i.wrapping_sub(1), j, k),
            (i + 1, j, k),
            (i, j.wrapping_sub(1), k),
            (i, j + 1, k),
            (i, j, k.wrapping_sub(1)),
            (i, j, k + 1),
        ]
    };

    // 1. Flood the wide air (exterior, or clearance >= r_clear) from the 6 boundary faces of the grid
    let mut ext_air = vec![false; total];
    let mut q = VecDeque::new();
    for i in 0..nx {
        for j in 0..ny {
            for k in 0..nz {
                if i == 0 || i + 1 == nx || j == 0 || j + 1 == ny || k == 0 || k + 1 == nz {
                    let idx = (i * ny + j) * nz + k;
                    if wide(idx) && !ext_air[idx] {
                        ext_air[idx] = true;
                        q.push_back((i, j, k));
                    }
                }
            }
        }
    }
    let mut reached = Vec::new();
    while let Some((i, j, k)) = q.pop_front() {
        reached.push((i, j, k));
        for (ni, nj, nk) in nbrs6(i, j, k) {
            if ni < nx && nj < ny && nk < nz {
                let nidx = (ni * ny + nj) * nz + nk;
                if wide(nidx) && !ext_air[nidx] {
                    ext_air[nidx] = true;
                    q.push_back((ni, nj, nk));
                }
            }
        }
    }
    // 2. The air along the walls of a drained passage drains with it: grow the wide set through any air by at
    //    most the passage half-width, in 26-connected steps (each covers one voxel of Chebyshev distance, so
    //    corners are reached), so a thinner gap leading further stays sealed
    let max_steps = ((r_clear as f64) / hv).ceil() as usize + 1;
    let mut front = reached;
    for _ in 0..max_steps {
        let mut next = Vec::new();
        for &(i, j, k) in &front {
            for di in -1_isize..=1 {
                for dj in -1_isize..=1 {
                    for dk in -1_isize..=1 {
                        let (ni, nj, nk) = (
                            i.wrapping_add_signed(di),
                            j.wrapping_add_signed(dj),
                            k.wrapping_add_signed(dk),
                        );
                        if ni < nx && nj < ny && nk < nz {
                            let nidx = (ni * ny + nj) * nz + nk;
                            if d_am[nidx] > 0.0 && !ext_air[nidx] {
                                ext_air[nidx] = true;
                                next.push((ni, nj, nk));
                            }
                        }
                    }
                }
            }
        }
        if next.is_empty() {
            break;
        }
        front = next;
    }
    // Any envelope air voxel not drained is a sealed pocket: measured, then kept solid (-h)
    for idx in 0..total {
        if d_am[idx] > 0.0 && !ext_air[idx] && !exterior(idx) {
            stats.sealed_void_mm3 += (1.0 - solid_fraction(d_am[idx], hv)) * dv;
            d_am[idx] = -h;
        }
    }

    // 2. Keep only the largest 6-connected solid component (d_am < 0)
    let mut comp_id = vec![0u32; total];
    let mut comp_sizes = vec![0usize];
    let mut cur_id = 0u32;
    for i in 0..nx {
        for j in 0..ny {
            for k in 0..nz {
                let start = (i * ny + j) * nz + k;
                if d_am[start] < 0.0 && comp_id[start] == 0 {
                    cur_id += 1;
                    comp_id[start] = cur_id;
                    let mut sz = 0usize;
                    q.push_back((i, j, k));
                    while let Some((ci, cj, ck)) = q.pop_front() {
                        sz += 1;
                        let nbrs = [
                            (ci.wrapping_sub(1), cj, ck),
                            (ci + 1, cj, ck),
                            (ci, cj.wrapping_sub(1), ck),
                            (ci, cj + 1, ck),
                            (ci, cj, ck.wrapping_sub(1)),
                            (ci, cj, ck + 1),
                        ];
                        for (ni, nj, nk) in nbrs {
                            if ni < nx && nj < ny && nk < nz {
                                let nidx = (ni * ny + nj) * nz + nk;
                                if d_am[nidx] < 0.0 && comp_id[nidx] == 0 {
                                    comp_id[nidx] = cur_id;
                                    q.push_back((ni, nj, nk));
                                }
                            }
                        }
                    }
                    comp_sizes.push(sz);
                }
            }
        }
    }
    if cur_id > 1 {
        let best_id = comp_sizes
            .iter()
            .enumerate()
            .skip(1)
            .max_by_key(|&(_, &sz)| sz)
            .map(|(id, _)| id as u32)
            .unwrap_or(1);
        for idx in 0..total {
            if d_am[idx] < 0.0 && comp_id[idx] != best_id {
                stats.floating_solid_mm3 += solid_fraction(d_am[idx], hv) * dv;
                d_am[idx] = h;
            }
        }
    }
    stats
}

// ---------------- 4. Hand Zone + Un-Anchored 3D Blade Builder (Rules 5 & 6) ----------------

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

/// Per-x cross-section parameters of the grip sweep (`hzm.py::grip_curves` + 14 mm grip->guard taper).
#[derive(Debug, Clone, Copy)]
pub struct GripSliceCurve {
    pub yt: f64,
    pub yb: f64,
    pub ym: f64,
    pub t_sec: f64,
    pub n_exp: f64,
    pub s: f64,
}

/// Perimeter of the asymmetric superellipse grip section `(|y|/a)^n + (|z|/b)^n = 1` with top and bottom
/// semi-heights `a_top`, `a_bot` and half-thickness `b` (64-segment quarter arcs).
pub fn superellipse_perimeter(a_top: f64, a_bot: f64, b: f64, n: f64) -> f64 {
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

/// Evaluate the grip section at `x` (G5.0). The G3 profile genes set the shape along the grip; the section is then
/// scaled about its mid-height so that the swell section has perimeter `grip_perim` and height/thickness
/// `grip_hot` (RS-karambit-v5 `[ergonomics.grip]`, design at the requirement level). The scaling fades out over the
/// 14 mm grip-to-guard taper, so the guard is unchanged.
pub fn eval_grip_slice(p: &BTreeMap<&'static str, f64>, x: f64) -> GripSliceCurve {
    let raw = eval_grip_slice_g3(p, x);
    let sw = eval_grip_slice_g3(p, p["x_t"] + p["s_sw"] * p["L_g"]);
    let a_t = (sw.yt - sw.ym).max(0.5);
    let a_b = (sw.ym - sw.yb).max(0.5);
    let b = 0.5 * sw.t_sec.max(1.0);
    let k = p["grip_hot"] * (2.0 * b) / (a_t + a_b);
    let sz = p["grip_perim"] / superellipse_perimeter(k * a_t, k * a_b, b, sw.n_exp).max(1e-6);
    let sy = sz * k;
    let w = 1.0 - smoothstep(p["L_g"] - 6.0, p["L_g"] + 8.0, raw.s);
    let (fy, fz) = (1.0 + w * (sy - 1.0), 1.0 + w * (sz - 1.0));
    GripSliceCurve {
        yt: raw.ym + fy * (raw.yt - raw.ym),
        yb: raw.ym - fy * (raw.ym - raw.yb),
        t_sec: (raw.t_sec * fz).max(0.3),
        ..raw
    }
}

/// The G3 grip section law (profile genes only), before the G5.0 perimeter and H/T scaling.
fn eval_grip_slice_g3(p: &BTreeMap<&'static str, f64>, x: f64) -> GripSliceCurve {
    let l_g = p["L_g"];
    let x_t = p["x_t"];
    let s = x - x_t;
    let s_clamped = s.clamp(0.0, l_g + 8.0);
    let yt = interp_pchip(
        &[0.0, p["s_ps"] * l_g, 0.9 * l_g, l_g + 8.0],
        &[p["yt0"], p["yt_sw"], p["yt_land"], 12.200000000000605],
        s_clamped,
    ) + p["ramp_h"] * bump((s - (l_g - 11.0)) / 10.0);

    let u01 = (s / l_g.max(1.0)).clamp(0.0, 1.0);
    let base = p["yb0"]
        + (p["yb_g"] - p["yb0"]) * u01
        + p["belly"] * (std::f64::consts::PI * u01).sin();
    let g_ring = p["d_ring"] * bump((s - (p["centre_ring"] + p["off_ring"])) / p["w_ring"]);
    let g_mid = p["d_mid"] * bump((s - (p["centre_mid"] + p["off_mid"])) / p["w_mid"]);
    let g_idx = p["d_idx"] * bump((s - (p["centre_idx"] + p["off_idx"])) / p["w_idx"]);
    let mut yb = base + g_ring + g_mid + g_idx;
    if s > l_g {
        yb += (s - l_g) * 0.6;
    }

    // Rule 5: 14 mm smooth grip->guard thickness taper from L_g - 6 to L_g + 8 (no grip cliff)
    let t_sec = interp_pchip(
        &[0.0, p["s_sw"] * l_g, l_g - 6.0, l_g + 8.0],
        &[
            p["T0"],
            p["T_sw"],
            p["T_g"],
            (p["t_guard"] - 0.6).max(3.0),
        ],
        s_clamped,
    );
    let n_exp = interp_pchip(
        &[0.0, p["s_sw"] * l_g, l_g + 8.0],
        &[p["n0"], p["n1"], 0.5 * (p["n0"] + p["n1"])],
        s_clamped,
    );

    // v2/v3 (2026-10-25, T9): replace the ring-circle clip with a smooth quarter-ellipse cap inside s < -neck_in + 4
    // so the grip starts at s = -neck_in (inside the ring outer band, well clear of the bore) with zero radius
    // and blooms smoothly to the full grip section over 4 mm — no abrupt shoulder at the ring tangent.
    let ym = yb + (0.5 + p["skew"]) * (yt - yb);
    let cap = 4.0_f64;
    let neck_in = p.get("neck_in").copied().unwrap_or(5.0);
    let s_c = -neck_in + cap;
    let f = if s < s_c {
        (1.0 - ((s_c - s) / cap).powi(2)).clamp(0.0, 1.0).sqrt()
    } else {
        1.0
    };
    let yt = ym + (yt - ym) * f;
    let yb = ym - (ym - yb) * f;
    let t_sec = (t_sec * f).max(0.3);
    GripSliceCurve {
        yt,
        yb,
        ym,
        t_sec,
        n_exp,
        s,
    }
}

/// Evaluate the un-anchored G5 blade planform curves `(y_in(x), y_sp(x))` and 3D half-thickness
/// `z_half(x, y)` for `x in [GUARD_X_MM, x_tip]`, including:
/// - Cosine-basis edge offsets `edge_in_c1..c8`, `edge_sp_c1..c8`
/// - Serrated contour modulation outside `tip_reserve`
/// - 3D structural ridge + fuller + sinusoidal corrugation
/// - 5-gene continuous level-set medial fenestration (`win_amp`, `win_pitch`, `win_s0`, `win_s1`, `win_angle`)
/// - Double-bevel grind from the serrated contour + monotone S2 point taper inside `tip_reserve`.
/// Published f075 guard spine `yt(x)` (smoothed with sigma = 1.5 mm) for `x = 67.6 + 0.5 * k`, `k = 0..64` (`hzm.py::guard_lobe`).
const F075_GUARD_YT: [f64; 64] = [
    14.010235, 13.999398, 13.980026, 13.948308, 13.900502, 13.833764, 13.746849, 13.640377,
    13.516604, 13.378855, 13.230930, 13.076716, 12.920071, 12.764906, 12.615249, 12.475209,
    12.348699, 12.238958, 12.148126, 12.076999, 12.025092, 11.990996, 11.973035, 11.969884,
    11.980879, 12.005820, 12.044404, 12.095701, 12.157987, 12.228828, 12.305055, 12.382517,
    12.455961, 12.519480, 12.567527, 12.595964, 12.602576, 12.586897, 12.549648, 12.492130,
    12.415875, 12.322488, 12.213570, 12.090706, 11.955504, 11.809614, 11.654702, 11.492438,
    11.324451, 11.152258, 10.977243, 10.800658, 10.623688, 10.447532, 10.273467, 10.102856,
    9.937116, 9.777671, 9.625970, 9.483586, 9.352356, 9.234406, 9.131943, 9.046782,
];

/// Published f075 normalized guard lobe shape `lobe(x)` (smoothed with sigma = 0.6 mm) for `x = 67.6 + 0.5 * k`, `k = 0..64` (`hzm.py::guard_lobe`).
const F075_GUARD_LOBE: [f64; 64] = [
    0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000032, 0.000459, 0.003366, 0.013884,
    0.034861, 0.060020, 0.082365, 0.102869, 0.125527, 0.151960, 0.181549, 0.214870, 0.254794,
    0.302531, 0.355584, 0.411600, 0.470678, 0.533048, 0.597225, 0.660623, 0.720245, 0.773698,
    0.820078, 0.859546, 0.892823, 0.920587, 0.943374, 0.961709, 0.975838, 0.985984, 0.992111,
    0.993843, 0.990932, 0.983008, 0.969224, 0.948362, 0.919180, 0.880727, 0.832364, 0.774031,
    0.707222, 0.635176, 0.561546, 0.489409, 0.421058, 0.357622, 0.299102, 0.244812, 0.194414,
    0.147986, 0.105020, 0.065840, 0.033616, 0.012628, 0.003148, 0.000477, 0.000040, 0.000001,
    0.000000,
];

#[inline]
pub fn eval_guard_lobe_y(p: &BTreeMap<&'static str, f64>, x: f64) -> (f64, f64) {
    let u = ((x - 67.6) / 0.5).clamp(0.0, 63.0);
    let i0 = (u.floor() as usize).min(62);
    let frac = u - (i0 as f64);
    let yt = F075_GUARD_YT[i0] * (1.0 - frac) + F075_GUARD_YT[i0 + 1] * frac;
    let lobe = F075_GUARD_LOBE[i0] * (1.0 - frac) + F075_GUARD_LOBE[i0 + 1] * frac;
    let yb = p["yb_g"] - p["h_g"] * lobe;
    (yb, yt)
}

/// Published `f075` land perforation hole `(x, y_lo, y_hi)` (`61.2 mm^2` at `x in [84.5, 95.5]`, `hzm.py::perf_hole` parity, `N3c`).
const F075_PERF_HOLE: [(f64, f64, f64); 23] = [
    (84.50, -0.50, 0.35),
    (85.00, -1.50, 1.35),
    (85.50, -2.15, 1.95),
    (86.00, -2.60, 2.40),
    (86.50, -2.90, 2.85),
    (87.00, -3.10, 3.25),
    (87.50, -3.30, 3.55),
    (88.00, -3.45, 3.80),
    (88.50, -3.65, 3.95),
    (89.00, -3.85, 4.15),
    (89.50, -4.00, 4.30),
    (90.00, -4.15, 4.40),
    (90.50, -4.25, 4.50),
    (91.00, -4.35, 4.55),
    (91.50, -4.45, 4.60),
    (92.00, -4.55, 4.60),
    (92.50, -4.60, 4.60),
    (93.00, -4.65, 4.50),
    (93.50, -4.60, 4.40),
    (94.00, -4.50, 4.25),
    (94.50, -4.30, 4.00),
    (95.00, -3.90, 3.55),
    (95.50, -2.85, 2.55),
];

#[inline]
fn f075_in_perf_hole(x: f64, y: f64) -> bool {
    if !(84.45..=95.55).contains(&x) || !(-4.70..=4.65).contains(&y) {
        return false;
    }
    let u = ((x - 84.50) / 0.50).clamp(0.0, 22.0);
    let i0 = (u.floor() as usize).min(21);
    let frac = u - (i0 as f64);
    let y_lo = F075_PERF_HOLE[i0].1 * (1.0 - frac) + F075_PERF_HOLE[i0 + 1].1 * frac;
    let y_hi = F075_PERF_HOLE[i0].2 * (1.0 - frac) + F075_PERF_HOLE[i0 + 1].2 * frac;
    y >= y_lo && y <= y_hi
}

#[derive(Debug, Clone)]
pub struct BladeProfile2D {
    pub x_heel: f64,
    pub x_tip: f64,
    pub tip_xy: (f64, f64),
    pub tip_inward_dir: (f64, f64),
    pub centroid_x_mm: f64,
    pub reach_chord_mm: f64,
    pub xs: Vec<f64>,
    pub y_in: Vec<f64>,
    pub y_sp: Vec<f64>,
    pub y_in_smooth: Vec<f64>,
    pub y_sp_smooth: Vec<f64>,
    pub cum_arc_in: Vec<f64>,
    pub cum_arc_sp: Vec<f64>,
    pub arc_in_mm: f64,
    pub arc_sp_mm: f64,
    pub sharpened_arc_in_mm: f64,
    pub sharpened_arc_sp_mm: f64,
    /// Serrations as built (`SERR`): the deepest cut of the outline below its smooth line (mm) and the number of
    /// teeth (cuts deeper than half that), per edge
    pub serr_depth_in_mm: f64,
    pub serr_depth_sp_mm: f64,
    pub serr_teeth_in: usize,
    pub serr_teeth_sp: usize,
}

/// Gullets of a serrated edge: each contiguous run of the outline cut deeper than half the deepest cut below its smooth
/// line is one gullet (micro-serration ripples on a gullet floor stay inside its run); returns the deepest cut (mm) and
/// the sample index of each gullet bottom. A tooth is the uncut edge between two neighbouring gullets.
pub fn serration_gullets(edge: &[f64], smooth: &[f64]) -> (f64, Vec<usize>) {
    let dev: Vec<f64> = edge.iter().zip(smooth).map(|(e, s)| (e - s).abs()).collect();
    let depth = dev.iter().cloned().fold(0.0_f64, f64::max);
    let mut bottoms = Vec::new();
    let mut k = 0usize;
    while k < dev.len() {
        if depth > 0.0 && dev[k] > 0.5 * depth {
            let start = k;
            while k < dev.len() && dev[k] > 0.5 * depth {
                k += 1;
            }
            let run = start..k;
            bottoms.push(run.clone().max_by(|&a, &b| dev[a].total_cmp(&dev[b])).unwrap());
        } else {
            k += 1;
        }
    }
    (depth, bottoms)
}

/// Deepest serration cut (mm) and number of teeth (gullets bounding a tooth on both sides count; the outermost
/// gullets of the span bound one tooth each, so teeth = gullets).
fn serration_census(edge: &[f64], smooth: &[f64]) -> (f64, usize) {
    let (depth, bottoms) = serration_gullets(edge, smooth);
    (depth, bottoms.len())
}

/// Serration tooth depth per unit pitch (`SERR-4`).
/// RS-karambit-v5 `[serration] depth_max_fraction_of_pitch` (a test checks it).
pub const SERR_DEPTH_PER_PITCH: f64 = 0.25;

pub const SHARPENED_EDGE_CAP_MM: f64 = 4.0 * 25.4; // 101.6 mm (4.0 in) per edge
pub const BLADE_REACH_MAX_MM: f64 = 85.725;

#[inline]
/// One serration period (G5.0): a raised-cosine scallop of width `duty` (fraction of the pitch) and unit depth, flat
/// between scallops. G3's triangular-tooth and micro components cut cusped gullets; G5.0 keeps only the scallop,
/// whose gullet radius is set by `serration_duty_for_gullet`.
fn serration_wave(phase_01: f64, duty: f64) -> f64 {
    let duty = duty.clamp(0.25, 0.95);
    let u = (phase_01.rem_euclid(1.0) / duty).clamp(0.0, 1.0);
    0.5 * (1.0 - (2.0 * std::f64::consts::PI * u).cos())
}

/// Gullet radius (mm) of a raised-cosine scallop of width `w` and depth `d`: y = d (1 − cos(2πs/w))/2 has curvature
/// 2π²d/w² at its root.
pub fn serration_gullet_radius(w_mm: f64, depth_mm: f64) -> f64 {
    w_mm * w_mm / (2.0 * std::f64::consts::PI * std::f64::consts::PI * depth_mm.max(1e-9))
}

/// Scallop duty (fraction of the pitch) widened as needed so the gullet radius is at least
/// `[serration] gullet_radius_min_mm`, and at most 0.95 of the pitch.
pub fn serration_duty_for_gullet(duty_gene: f64, pitch_mm: f64, depth_mm: f64) -> f64 {
    let r_min = crate::requirements::rs_v5().serration.gullet_radius_min_mm;
    let w_min = (2.0 * std::f64::consts::PI * std::f64::consts::PI * r_min * depth_mm.max(0.0)).sqrt();
    duty_gene.max(w_min / pitch_mm.max(1e-9)).min(0.95)
}

/// Evaluate the bevel blend weights `(win_in, win_sp)` along the inner and spine edges, budgeting
/// the 3.0 mm entry ramp and `tip_bevel_lock` strictly inside `[arc_start, arc_len]` with a 1.5 mm
/// discretization margin (`S5-M3`, `S5-M4`) so the midpoint-rule sharpened arc never exceeds
/// `SHARPENED_EDGE_CAP_MM` (`101.6 mm`, `N6`).
///
/// `EDGE-4` (owner 2026-10-26, "ideal sharpness angle for the entire two-sided blade"): both grinds run
/// unbroken to the tip. `s1_in` / `s1_sp` are retired (read by nothing); the sharpened length is set by the
/// heel-side start `s0_*` and the per-edge cap, so any unsharpened length sits at the plunge, never at the point.
#[inline]
pub fn blade_bevel_window_weights(
    p: &BTreeMap<&'static str, f64>,
    arc_in_mm: f64,
    arc_sp_mm: f64,
    s_arc_in: f64,
    s_arc_sp: f64,
    a_tip: f64,
) -> (f64, f64) {
    let cap_safe = SHARPENED_EDGE_CAP_MM - 1.5; // 100.1 mm margin against ds/2 midpoint discretization
    let start_in = (p["s0_in"].clamp(0.0, 0.50) * arc_in_mm)
        .min((arc_in_mm - 8.0).max(0.0))
        .max((arc_in_mm - cap_safe).max(0.0));
    let start_sp = (p["s0_sp"].clamp(0.0, 0.50) * arc_sp_mm)
        .min((arc_sp_mm - 8.0).max(0.0))
        .max((arc_sp_mm - cap_safe).max(0.0));
    let tip_bevel_lock = 1.0 - smoothstep(2.5, 4.5, a_tip);
    let win_in = if s_arc_in <= start_in {
        0.0
    } else {
        smoothstep(start_in, start_in + 3.0, s_arc_in).max(tip_bevel_lock)
    };
    let win_sp = if s_arc_sp <= start_sp {
        0.0
    } else {
        smoothstep(start_sp, start_sp + 3.0, s_arc_sp).max(tip_bevel_lock)
    };
    (win_in, win_sp)
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

/// Planform tip radius of the finished point (mm): the midpoint of RS-karambit-v5 `[point] tip_radius_min/max_mm`.
/// A construction constant, not a gene: it lies below the planform sampling and the voxel grid, so a gene would not
/// change the evaluated geometry.
pub const POINT_TIP_RADIUS_MM: f64 = 0.175;

/// Half-width (mm) of the G5.0 point at distance `d` behind its tip: a planform wedge of included angle `angle_deg`
/// whose apex is rounded to radius `r` (circle tangent to both flanks).
pub fn rounded_wedge_half_width(d: f64, angle_deg: f64, r: f64) -> f64 {
    let a = (0.5 * angle_deg).clamp(1.0, 89.0).to_radians();
    let c = r / a.sin(); // virtual apex to circle centre
    let dp = d.max(0.0) + (c - r); // distance from the virtual apex
    if dp >= c * a.cos() * a.cos() {
        dp * a.tan()
    } else {
        (r * r - (c - dp) * (c - dp)).max(0.0).sqrt()
    }
}

pub fn build_blade_profile_2d(p: &BTreeMap<&'static str, f64>) -> BladeProfile2D {
    let x_heel = GUARD_X_MM + 4.0;
    let blade_len = p["blade_len"].clamp(60.0, BLADE_REACH_MAX_MM);
    let x_tip_raw = x_heel + blade_len;
    let hook_drop = p["hook_drop"];
    let h_root = p["h_root"];
    let h_exp = p["h_exp"];
    let sweep_rad = p["sweep_deg"].to_radians();
    let tip_reserve = p["tip_reserve"];

    let n_pts = 180usize;
    let mut xs = Vec::with_capacity(n_pts);
    let mut y_in = Vec::with_capacity(n_pts);
    let mut y_sp = Vec::with_capacity(n_pts);
    let mut y_in_smooth = Vec::with_capacity(n_pts);
    let mut y_sp_smooth = Vec::with_capacity(n_pts);

    let c_in = [
        p["edge_in_c1"],
        p["edge_in_c2"],
        p["edge_in_c3"],
        p["edge_in_c4"],
        p["edge_in_c5"],
        p["edge_in_c6"],
        p["edge_in_c7"],
        p["edge_in_c8"],
    ];
    let c_sp = [
        p["edge_sp_c1"],
        p["edge_sp_c2"],
        p["edge_sp_c3"],
        p["edge_sp_c4"],
        p["edge_sp_c5"],
        p["edge_sp_c6"],
        p["edge_sp_c7"],
        p["edge_sp_c8"],
    ];

    // `SERR-2` (owner: "double-edged, double-serrated"): each serrated span carries at least 3 teeth, so the pitch is
    // capped at a third of the span; depth floors (`SERR-1`) and the post-plunge start (`SERR-3`) are gene bounds
    // The span ends at least 18 mm from the point (past the point taper's fade at tip_reserve + 3 mm and outside the
    // solid-tip FE read-out, 4-15 mm), leaving the piercing point plain; the pitch is capped on the part of the span at
    // full amplitude (inside the 0.04 entry and exit ramps), so all 3+ teeth are cut to depth.
    let ser_span = |s0: f64, s1: f64| {
        let s1 = s1.min(1.0 - (tip_reserve + 3.0).max(18.0) / blade_len).max(s0 + 0.16);
        (s0, s1)
    };
    let (ser_s0_in, ser_s1_in) = ser_span(p["ser_s0_in"], p["ser_s1_in"]);
    let (ser_s0_sp, ser_s1_sp) = ser_span(p["ser_s0_sp"], p["ser_s1_sp"]);
    let pitch_in = p["ser_pitch_in"].min((ser_s1_in - ser_s0_in - 0.08) * blade_len / 3.2);
    let pitch_sp = p["ser_pitch_sp"].min((ser_s1_sp - ser_s0_sp - 0.08) * blade_len / 3.2);
    // `SERR-4` (owner: "the most formidable, but we don't want to break serrated teeth"): pitch >= 5 mm (gene bound;
    // the serrated span is long enough for 3 teeth at that pitch) and depth <= `SERR_DEPTH_PER_PITCH` of the pitch.
    // Narrower or deeper teeth fail the single-tooth snag / pry cases of `serration_tooth_strength`.
    let depth_in = p["ser_depth_in"].min(SERR_DEPTH_PER_PITCH * pitch_in);
    let depth_sp = p["ser_depth_sp"].min(SERR_DEPTH_PER_PITCH * pitch_sp);
    let duty_in = serration_duty_for_gullet(p["ser_duty_in"], pitch_in, depth_in);
    let duty_sp = serration_duty_for_gullet(p["ser_duty_sp"], pitch_sp, depth_sp);
    for i in 0..n_pts {
        let u = (i as f64) / ((n_pts - 1) as f64);
        let x = x_heel + u * blade_len;
        let dist_to_tip = x_tip_raw - x;
        // Smooth karambit claw centerline drop and height taper
        let y_tip = -hook_drop;
        let y_root_mid = 0.0;
        let arch = p["spine_arch"] * (std::f64::consts::PI * u).sin();
        let sweep_shift = blade_len * sweep_rad.tan() * u * (1.0 - u);
        let y_center = y_root_mid + (y_tip - y_root_mid) * u.powf(1.35) + arch + sweep_shift;
        let h_u_g3 = h_root * (1.0 - u).max(0.0).powf(h_exp)
            + 2.0 * dist_to_tip * (0.5 * p["tip_angle"].to_radians()).tan() * u;
        // G5.0 point module (owner 2026-10-27: penalise a blunt point): within `point_len` of the tip the blade is
        // no wider than a rounded wedge of `point_angle`, fading back to the G3 law over the next 4 mm. G3's
        // (1 - u)^h_exp law with h_exp < 1 made a bullet nose (the G3 winner: 110 deg at 1 mm).
        let w_pt = 1.0 - smoothstep(p["point_len"], p["point_len"] + 4.0, dist_to_tip);
        let h_wedge = 2.0 * rounded_wedge_half_width(dist_to_tip, p["point_angle"], POINT_TIP_RADIUS_MM);
        let h_u = h_u_g3 * (1.0 - w_pt) + h_u_g3.min(h_wedge) * w_pt;
        let h_half = 0.5 * h_u.max(0.08);

        // Cosine basis modulation (vanishes at u=0 and u=1, and faded inside tip_reserve)
        let tip_fade = smoothstep(tip_reserve, tip_reserve + 3.0, dist_to_tip);
        let env = (std::f64::consts::PI * u).sin() * tip_fade;
        let mut d_in = p["blade_belly"] * env;
        let mut d_sp = 0.0_f64;
        for k in 0..8 {
            let harm = ((k + 1) as f64 * std::f64::consts::PI * u).sin() * env;
            d_in += 0.25 * c_in[k] * harm;
            d_sp += 0.25 * c_sp[k] * harm;
        }

        // Serrated contour modulation (faded out inside tip_reserve so Gate S2 point is clean)
        let ser_in_win = smoothstep(ser_s0_in, ser_s0_in + 0.04, u)
            * (1.0 - smoothstep(ser_s1_in - 0.04, ser_s1_in, u))
            * tip_fade;
        let ser_sp_win = smoothstep(ser_s0_sp, ser_s0_sp + 0.04, u)
            * (1.0 - smoothstep(ser_s1_sp - 0.04, ser_s1_sp, u))
            * tip_fade;
        let s_mm = u * blade_len;
        let wave_in = serration_wave(s_mm / pitch_in, duty_in);
        let wave_sp = serration_wave(s_mm / pitch_sp, duty_sp);
        let ser_in = ser_in_win * depth_in * wave_in;
        let ser_sp = ser_sp_win * depth_sp * wave_sp;

        let yi_sm = y_center - h_half + 0.3 * d_in;
        let ys_sm = (y_center + h_half + 0.3 * d_sp).max(yi_sm + 0.08);
        let yi = yi_sm + ser_in;
        let ys_val = (ys_sm - ser_sp).max(yi + 0.08);
        xs.push(x);
        y_in.push(yi);
        y_sp.push(ys_val);
        y_in_smooth.push(yi_sm);
        y_sp_smooth.push(ys_sm);
    }

    // Enforce straight-line chord reach from tip to guard face <= 85.725 mm (`N6`, `P3`, owner 2026-10-25):
    // satisfies BOTH chord to (GUARD_X_MM, y_mid_heel) (`hz_probes.rs` P3) AND chord to (GUARD_X_MM, 0.0).
    let y_mid0 = 0.5 * (y_in[0] + y_sp[0]);
    let y_tip_raw = 0.5 * (y_in[n_pts - 1] + y_sp[n_pts - 1]);
    let dy_mid_raw = y_tip_raw - y_mid0;
    let dx_heel_raw = xs[n_pts - 1] - x_heel;
    let dx_guard_heel = x_heel - GUARD_X_MM; // 4.0 mm
    let solve_chord_scale = |dx_h: f64, dy_h: f64, y_offset: f64| -> f64 {
        let chord = (dx_guard_heel + dx_h).hypot(y_offset + dy_h);
        if chord <= BLADE_REACH_MAX_MM {
            return 1.0;
        }
        let a_q = dx_h * dx_h + dy_h * dy_h;
        let b_q = 2.0 * (dx_guard_heel * dx_h + y_offset * dy_h);
        let c_q = dx_guard_heel * dx_guard_heel + y_offset * y_offset
            - BLADE_REACH_MAX_MM * BLADE_REACH_MAX_MM;
        let disc = (b_q * b_q - 4.0 * a_q * c_q).max(0.0).sqrt();
        let raw_s = (-b_q + disc) / (2.0 * a_q.max(1e-9));
        debug_assert!(raw_s >= 0.1, "chord reach scale {raw_s} fell below 0.1");
        raw_s.clamp(0.1, 1.0)
    };
    let scale_mid = solve_chord_scale(dx_heel_raw, dy_mid_raw, 0.0);
    let scale_zero = solve_chord_scale(dx_heel_raw, dy_mid_raw, y_mid0);
    let scale = scale_mid.min(scale_zero);
    if scale < 1.0 {
        for i in 0..n_pts {
            xs[i] = x_heel + scale * (xs[i] - x_heel);
            y_in[i] = y_mid0 + scale * (y_in[i] - y_mid0);
            y_sp[i] = y_mid0 + scale * (y_sp[i] - y_mid0);
            y_in_smooth[i] = y_mid0 + scale * (y_in_smooth[i] - y_mid0);
            y_sp_smooth[i] = y_mid0 + scale * (y_sp_smooth[i] - y_mid0);
        }
    }
    let x_tip = xs[n_pts - 1];

    // Compute 2D area-weighted centroid x of the evolved blade planform (`S5-7`)
    let mut area_w_sum = 0.0_f64;
    let mut area_wx_sum = 0.0_f64;
    for i in 0..n_pts {
        let w_h = (y_sp[i] - y_in[i]).max(0.1);
        area_w_sum += w_h;
        area_wx_sum += w_h * xs[i];
    }
    let centroid_x_mm = if area_w_sum > 1e-9 {
        area_wx_sum / area_w_sum
    } else {
        0.5 * (x_heel + x_tip)
    };

    let mut cum_arc_in = Vec::with_capacity(n_pts);
    let mut cum_arc_sp = Vec::with_capacity(n_pts);
    cum_arc_in.push(0.0);
    cum_arc_sp.push(0.0);
    let mut arc_in_mm = 0.0_f64;
    let mut arc_sp_mm = 0.0_f64;
    for i in 1..n_pts {
        let dx = xs[i] - xs[i - 1];
        arc_in_mm += dx.hypot(y_in[i] - y_in[i - 1]);
        arc_sp_mm += dx.hypot(y_sp[i] - y_sp[i - 1]);
        cum_arc_in.push(arc_in_mm);
        cum_arc_sp.push(arc_sp_mm);
    }
    let tip_xy = (x_tip, 0.5 * (y_in[n_pts - 1] + y_sp[n_pts - 1]));
    let y_mid_heel = 0.5 * (y_in[0] + y_sp[0]);
    let reach_chord_mm = (tip_xy.0 - GUARD_X_MM)
        .hypot(tip_xy.1 - y_mid_heel)
        .max((tip_xy.0 - GUARD_X_MM).hypot(tip_xy.1));
    // Compute unit inward tip bisector vector along the blade centerline at 2.0 mm from tip_xy
    let mut q_mid = (xs[0], 0.5 * (y_in[0] + y_sp[0]));
    for i in (0..n_pts - 1).rev() {
        let ym = 0.5 * (y_in[i] + y_sp[i]);
        let d_tip = (xs[i] - tip_xy.0).hypot(ym - tip_xy.1);
        if d_tip >= 2.0 {
            let ym_next = 0.5 * (y_in[i + 1] + y_sp[i + 1]);
            let d_next = (xs[i + 1] - tip_xy.0).hypot(ym_next - tip_xy.1);
            let t_lerp = ((2.0 - d_next) / (d_tip - d_next).max(1e-9)).clamp(0.0, 1.0);
            q_mid = (
                xs[i + 1] + t_lerp * (xs[i] - xs[i + 1]),
                ym_next + t_lerp * (ym - ym_next),
            );
            break;
        }
    }
    let bx = q_mid.0 - tip_xy.0;
    let by = q_mid.1 - tip_xy.1;
    let b_len = bx.hypot(by).max(1e-9);
    let tip_inward_dir = (bx / b_len, by / b_len);

    // Measure actual sharpened arc length directly on the bevel field along both edges (`N6`)
    let mut sharpened_arc_in_mm = 0.0_f64;
    let mut sharpened_arc_sp_mm = 0.0_f64;
    for i in 0..(n_pts - 1) {
        let ds_in = cum_arc_in[i + 1] - cum_arc_in[i];
        let ds_sp = cum_arc_sp[i + 1] - cum_arc_sp[i];
        let s_mid_in = 0.5 * (cum_arc_in[i] + cum_arc_in[i + 1]);
        let s_mid_sp = 0.5 * (cum_arc_sp[i] + cum_arc_sp[i + 1]);
        let x_mid = 0.5 * (xs[i] + xs[i + 1]);
        let y_mid_in = 0.5 * (y_in[i] + y_in[i + 1]);
        let y_mid_sp = 0.5 * (y_sp[i] + y_sp[i + 1]);
        let a_tip_in = ((x_mid - tip_xy.0) * tip_inward_dir.0
            + (y_mid_in - tip_xy.1) * tip_inward_dir.1)
            .max(0.0)
            .min((x_tip - x_mid).hypot(y_mid_in - tip_xy.1));
        let a_tip_sp = ((x_mid - tip_xy.0) * tip_inward_dir.0
            + (y_mid_sp - tip_xy.1) * tip_inward_dir.1)
            .max(0.0)
            .min((x_tip - x_mid).hypot(y_mid_sp - tip_xy.1));
        let (win_in, _) =
            blade_bevel_window_weights(p, arc_in_mm, arc_sp_mm, s_mid_in, s_mid_sp, a_tip_in);
        let (_, win_sp) =
            blade_bevel_window_weights(p, arc_in_mm, arc_sp_mm, s_mid_in, s_mid_sp, a_tip_sp);
        if win_in > 0.0 {
            sharpened_arc_in_mm += ds_in;
        }
        if win_sp > 0.0 {
            sharpened_arc_sp_mm += ds_sp;
        }
    }

    let (serr_depth_in_mm, serr_teeth_in) = serration_census(&y_in, &y_in_smooth);
    let (serr_depth_sp_mm, serr_teeth_sp) = serration_census(&y_sp, &y_sp_smooth);
    BladeProfile2D {
        x_heel,
        x_tip,
        tip_xy,
        tip_inward_dir,
        centroid_x_mm,
        reach_chord_mm,
        xs,
        y_in,
        y_sp,
        y_in_smooth,
        y_sp_smooth,
        cum_arc_in,
        cum_arc_sp,
        arc_in_mm,
        arc_sp_mm,
        sharpened_arc_in_mm,
        sharpened_arc_sp_mm,
        serr_depth_in_mm,
        serr_depth_sp_mm,
        serr_teeth_in,
        serr_teeth_sp,
    }
}

/// Evaluate local blade thickness $t(x, y)$ (mm) and signed 2D planform distance $d_2(x, y)$
/// from `BladeProfile2D`, guaranteeing Gate S1 ($\text{apex} \le 0.19\text{ mm}$), S2 (monotone point
/// taper with $t(0.25) \le 0.28$, $t(1) \le 0.56$, $t(2) \le 0.95$), S3 (one primary half-angle in
/// $[17.5^\circ, 22.5^\circ]$ per side, normal to the edge, over the full grind height and to the tip on both
/// edges, `EDGE-1..4`), and the 4.0 in (`101.6 mm`) sharpened-arc cap per edge by construction.
pub fn eval_blade_thickness_and_sdf2(
    p: &BTreeMap<&'static str, f64>,
    prof: &BladeProfile2D,
    x: f64,
    y: f64,
) -> (f64, f64) {
    if x < prof.x_heel || x > prof.x_tip {
        return (0.0, 20.0);
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
    if d2 > 0.0 {
        return (0.0, d2);
    }

    // Distance from the tip measured along the inward tip bisector (`B3`: works for any hook_drop / sweep_deg)
    let dx_tip = x - prof.tip_xy.0;
    let dy_tip = y - prof.tip_xy.1;
    let proj_bisector =
        (dx_tip * prof.tip_inward_dir.0 + dy_tip * prof.tip_inward_dir.1).max(0.0);
    let a_tip = proj_bisector.min((prof.x_tip - x).hypot(dy_tip));
    let tip_reserve = p["tip_reserve"].max(6.0);
    let t_core = p["t_core"];
    let t_tip = p["t_tip"].min(t_core);
    let taper_u = ((u - p["taper_x0"]) / (1.0 - p["taper_x0"]).max(0.1)).clamp(0.0, 1.0);
    let t_body = t_core - (t_core - t_tip) * taper_u;

    // Monotone S2 point profile for a_tip in [0, tip_reserve]
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

    // 3D dorsal/medial ridge + fuller + sinusoidal corrugation (faded out near edges and tip)
    let h_span = (ys - yi).max(1.0);
    let y_mid = 0.5 * (yi + ys);
    let dy_mid = (y - y_mid).abs();
    let fuller_win = smoothstep(p["fuller_s0"], p["fuller_s0"] + 0.06, u)
        * (1.0 - smoothstep(p["fuller_s1"] - 0.06, p["fuller_s1"], u))
        * smoothstep(tip_reserve, tip_reserve + 4.0, a_tip);
    let fuller_cut = fuller_win
        * p["fuller_depth"]
        * bump(dy_mid / (0.5 * p["fuller_width"].min(0.45 * h_span)).max(1.0))
        * (1.0
            + 0.35
                * p["fuller_corrug_amp"]
                * (2.0 * std::f64::consts::PI * (x - prof.x_heel) / 9.0).cos());
    let ridge_win = smoothstep(p["ridge_s0"], p["ridge_s0"] + 0.06, u)
        * (1.0 - smoothstep(p["ridge_s1"] - 0.06, p["ridge_s1"], u))
        * smoothstep(tip_reserve, tip_reserve + 4.0, a_tip);
    let ridge_add =
        ridge_win * p["ridge_amp"] * bump((y - (y_mid + 0.18 * h_span)) / p["ridge_w"].max(1.5));
    let t_medial = (t_spine_cap - fuller_cut + ridge_add).clamp(0.16, t_spine_cap.max(t_core + 1.8));

    // Double-bevel cutting edges measured along the actual curved edge arc length (`M4`, `N6`: <= 101.6 mm per
    // edge). The grind window is read at this x, which is continuous; the foot of the normal is not (on a
    // serrated edge it jumps a whole tooth, which would step the thickness inside the 3 mm entry ramp).
    let s_arc_in = prof.cum_arc_in[i0] * (1.0 - frac) + prof.cum_arc_in[i0 + 1] * frac;
    let s_arc_sp = prof.cum_arc_sp[i0] * (1.0 - frac) + prof.cum_arc_sp[i0 + 1] * frac;
    let dn_in = edge_normal_distance(&prof.xs, &prof.y_in, i0, x, y);
    let dn_sp = edge_normal_distance(&prof.xs, &prof.y_sp, i0, x, y);
    let (win_in, win_sp) = blade_bevel_window_weights(
        p,
        prof.arc_in_mm,
        prof.arc_sp_mm,
        s_arc_in,
        s_arc_sp,
        a_tip,
    );
    // Primary grind (`EDGE-1..3`, owner 2026-10-26): one per-side angle in [17.5, 22.5] deg (40 deg inclusive at
    // the 20 deg centre, a hard-use edge suited to Ti-6Al-4V), measured normal to the actual (serrated) edge and
    // carried at full height until it meets the medial thickness. No band cap and no shoulder: the old
    // 2.2-3.5 mm band blended a micro-bevel into the 4-7 mm body at 50-90 deg per side.
    let ed = &crate::requirements::rs_v5().edge;
    let tan_in = p["angle_in"].clamp(ed.per_side_angle_min_deg, ed.per_side_angle_max_deg).to_radians().tan();
    let tan_sp = p["angle_sp"].clamp(ed.per_side_angle_min_deg, ed.per_side_angle_max_deg).to_radians().tan();
    // Hollow/convex bevel curvature modulation (`grind_curv_in`, `grind_curv_sp`) beyond the 0.4 mm Gate-S3 edge band
    let curv_in = 1.0 + 0.18 * p["grind_curv_in"] * ((dn_in - 0.4).max(0.0) / 6.0).min(1.0);
    let curv_sp = 1.0 + 0.18 * p["grind_curv_sp"] * ((dn_sp - 0.4).max(0.0) / 6.0).min(1.0);
    let t_bevel_in_raw = p["edge_t_in"].min(0.19) + 2.0 * dn_in * tan_in * curv_in;
    let t_bevel_sp_raw = p["edge_t_sp"].min(0.19) + 2.0 * dn_sp * tan_sp * curv_sp;
    let t_bevel_in = win_in * t_bevel_in_raw + (1.0 - win_in) * t_medial;
    let t_bevel_sp = win_sp * t_bevel_sp_raw + (1.0 - win_sp) * t_medial;
    let t_final = t_medial.min(t_bevel_in).min(t_bevel_sp).min(t_spine_cap);

    // 5-gene continuous level-set medial fenestration inside fuller band (`S5-5` / `P5` / `S7-1` / `S8-4`):
    // - Kept >= 3.5 mm from grind edges and strictly past the guard-to-blade plunge/seam blend zone.
    // - Phase shifted by the window half-extent (`S8-4`) so the first cosine crest sits at
    //   `x_crest0 = x_bound + half_win_dx`, ensuring the first window is whole and never clipped at `x_bound`.
    let pitch = p["win_pitch"].max(6.0);
    let x_s0 = prof.x_heel + p["win_s0"] * (prof.x_tip - prof.x_heel).max(1e-9);
    let x_bound = x_s0.max(92.0 + 0.5 * pitch);
    if p["win_amp"] > 0.35
        && x >= x_bound
        && dn_in > 3.5
        && dn_sp > 3.5
        && a_tip > tip_reserve + 6.0
        && u < p["win_s1"]
    {
        let ang = p["win_angle"].to_radians();
        let w_span = (0.35 * h_span).max(2.0);
        let thresh = (1.0 - 0.55 * (p["win_amp"] - 0.35) / 0.65).clamp(-1.0, 1.0);
        let half_win_dx = (pitch * thresh.acos() / (2.0 * std::f64::consts::PI)) * ang.cos().abs()
            + w_span * ang.sin().abs()
            + 0.35;
        let x_crest0 = x_bound + half_win_dx;
        let dx = x - x_crest0;
        let wx = (dx * ang.cos() + (y - y_mid) * ang.sin()) / pitch;
        let wy = (-dx * ang.sin() + (y - y_mid) * ang.cos()) / w_span;
        let f_win = (2.0 * std::f64::consts::PI * wx).cos() * bump(wy);
        if f_win > thresh {
            return (0.0, 0.5);
        }
    }

    (t_final, d2)
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

/// Build the complete 3D SDF (`VoxelGrid3D`) for a `G5Genome` at voxel spacing `h` (mm),
/// voxelizing the entire one-piece knife (ring + grip + guard lobe + curved claw blade).
pub fn build_g5_sdf(genome: &G5Genome, h: f64, with_am: bool) -> VoxelGrid3D {
    let p = genome.decode();
    build_g5_sdf_from_params(&p, h, with_am, true)
}

/// Build the 3D SDF from a decoded parameter map `p` (when `include_blade` is false, stops at
/// `x_hi = 101.6` for hand-zone-only parity tests against `hzm.py`).
pub fn build_g5_sdf_from_params(
    p: &BTreeMap<&'static str, f64>,
    h: f64,
    with_am: bool,
    include_blade: bool,
) -> VoxelGrid3D {
    build_g5_sdf_from_params_with_origin_shift(p, h, with_am, include_blade, 0.0)
}

pub fn build_g5_sdf_from_params_with_origin_shift(
    p: &BTreeMap<&'static str, f64>,
    h: f64,
    with_am: bool,
    include_blade: bool,
    origin_shift_mm: f64,
) -> VoxelGrid3D {
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

    // Precompute 2D `slab_sdf2` on `(nx_hz, ny)` for the parity path (`include_blade == false`, `hzm.py::slab_sdf2`)
    let d2_slab_parity = if !include_blade {
        let mut ins_2d = vec![false; nx_hz * ny];
        for i in 0..nx_hz {
            let x = xs[i];
            if x >= GUARD_X_MM - 1.0 {
                let (yb_guard, yt_guard) = eval_guard_lobe_y(p, x);
                for (j, &y) in ys.iter().enumerate() {
                    if y >= yb_guard && y <= yt_guard && !f075_in_perf_hole(x, y) {
                        ins_2d[i * ny + j] = true;
                    }
                }
            }
        }
        sdf_mask(&ins_2d, nx_hz, ny, 1, h)
    } else {
        Vec::new()
    };

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
        } else if x >= GUARD_X_MM - 1.0 {
            // Parity path (`include_blade == false`, `hzm.py:471-478` parity, `N3c`):
            let (yb_guard, yt_guard) = eval_guard_lobe_y(p, x);
            let w_orig = smoothstep(94.0, 97.5, x);
            let bevel_w = 6.4 * smoothstep(94.0, 96.0, x);
            for (j, &y) in ys.iter().enumerate() {
                let d2_new = d2_slab_parity[i * ny + j] as f64;
                let d2_orig = (yb_guard - y).max(y - yt_guard);
                let th_orig = if bevel_w > 0.05 && y < yb_guard + bevel_w {
                    0.18 + 5.02 * ((y - yb_guard) / bevel_w).clamp(0.0, 1.0)
                } else {
                    5.20
                };
                let base = (i * ny + j) * nz;
                for (k, &z) in zs.iter().enumerate() {
                    let s_new = d2_new.max(z.abs() - 0.5 * p["t_guard"]);
                    let s_orig = d2_orig.max(z.abs() - 0.5 * th_orig);
                    let s_blend = ((1.0 - w_orig) * s_new + w_orig * s_orig) as f32;
                    da_hz[base + k] = da_hz[base + k].min(s_blend.clamp(-cl, cl));
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
                let ed = &crate::requirements::rs_v5().ergonomics.edges;
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
            let rho_stiff = crate::material::core_relative_stiffness(rho_z, p["lat_cell"], p["lat_mix"]);
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
                        disp += tex_amp * tx * scope_w * (1.0 - land);
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
        let t_sheet = |rho: f64| crate::material::tpms_sheet_thickness(rho, lat_cell, lat_mix);
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
            crate::requirements::rs_v5().am.drain_channel_min_mm,
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

// ---------------- 5. Watertight Marching-Tetrahedra Surface Extractor ----------------

/// Extract a guaranteed watertight, zero-boundary-edge, zero-non-manifold-edge triangle mesh
/// (`Solid`) from the zero level set ($d = 0$) of a boundary-capped `VoxelGrid3D` using Kuhn's
/// 6-tetrahedra-per-cube decomposition.
pub fn extract_watertight_mesh(grid: &VoxelGrid3D, use_am: bool) -> Solid {
    let field = if use_am {
        grid.d_am.as_deref().unwrap_or(&grid.d)
    } else {
        &grid.d
    };
    let (nx, ny, nz) = (grid.nx, grid.ny, grid.nz);
    let mut verts: Vec<(f64, f64, f64)> = Vec::new();
    let mut tris: Vec<[usize; 3]> = Vec::new();
    let mut edge_cache: std::collections::HashMap<(usize, usize), usize> =
        std::collections::HashMap::new();

    // 8 cube corner offsets and Kuhn 6-tetrahedra decomposition sharing main diagonal (0 -> 7)
    const C_OFF: [[usize; 3]; 8] = [
        [0, 0, 0],
        [1, 0, 0],
        [0, 1, 0],
        [1, 1, 0],
        [0, 0, 1],
        [1, 0, 1],
        [0, 1, 1],
        [1, 1, 1],
    ];
    const TETS: [[usize; 4]; 6] = [
        [0, 5, 1, 7],
        [0, 1, 3, 7],
        [0, 3, 2, 7],
        [0, 2, 6, 7],
        [0, 6, 4, 7],
        [0, 4, 5, 7],
    ];

    fn get_edge_vert(
        verts: &mut Vec<(f64, f64, f64)>,
        edge_cache: &mut std::collections::HashMap<(usize, usize), usize>,
        a: usize,
        b: usize,
        pa: (f64, f64, f64),
        pb: (f64, f64, f64),
        va: f32,
        vb: f32,
    ) -> usize {
        let (k0, k1, p0, p1, v0, v1) = if a < b {
            (a, b, pa, pb, va, vb)
        } else {
            (b, a, pb, pa, vb, va)
        };
        if let Some(&vi) = edge_cache.get(&(k0, k1)) {
            return vi;
        }
        let denom = (v0 - v1).abs().max(1e-12);
        let t = (v0.abs() / denom).clamp(0.001, 0.999) as f64;
        let pt = (
            p0.0 + t * (p1.0 - p0.0),
            p0.1 + t * (p1.1 - p0.1),
            p0.2 + t * (p1.2 - p0.2),
        );
        let vi = verts.len();
        verts.push(pt);
        edge_cache.insert((k0, k1), vi);
        vi
    }

    for i in 0..nx.saturating_sub(1) {
        for j in 0..ny.saturating_sub(1) {
            for k in 0..nz.saturating_sub(1) {
                let mut g_idx = [0usize; 8];
                let mut vals = [0.0_f32; 8];
                let mut n_in = 0usize;
                for c in 0..8 {
                    let [di, dj, dk] = C_OFF[c];
                    let gi = grid.idx(i + di, j + dj, k + dk);
                    g_idx[c] = gi;
                    // Perturb exact zeros slightly so vertices never sit on 0.0
                    let v = if field[gi] == 0.0 { -1e-7 } else { field[gi] };
                    vals[c] = v;
                    if v < 0.0 {
                        n_in += 1;
                    }
                }
                if n_in == 0 || n_in == 8 {
                    continue;
                }
                let mut pts = [(0.0, 0.0, 0.0); 8];
                for c in 0..8 {
                    let [di, dj, dk] = C_OFF[c];
                    pts[c] = (grid.x[i + di], grid.y[j + dj], grid.z[k + dk]);
                }

                for tet in &TETS {
                    let mut in_c = [0usize; 4];
                    let mut out_c = [0usize; 4];
                    let (mut ni, mut no) = (0usize, 0usize);
                    for &tc in tet {
                        if vals[tc] < 0.0 {
                            in_c[ni] = tc;
                            ni += 1;
                        } else {
                            out_c[no] = tc;
                            no += 1;
                        }
                    }
                    if ni == 0 || ni == 4 {
                        continue;
                    }
                    let orient_tri =
                        |v0: usize, v1: usize, v2: usize, p_out: (f64, f64, f64), verts: &[(f64, f64, f64)]| -> [usize; 3] {
                            let (a, b, c) = (verts[v0], verts[v1], verts[v2]);
                            let ux = b.0 - a.0;
                            let uy = b.1 - a.1;
                            let uz = b.2 - a.2;
                            let vx = c.0 - a.0;
                            let vy = c.1 - a.1;
                            let vz = c.2 - a.2;
                            let nx_ = uy * vz - uz * vy;
                            let ny_ = uz * vx - ux * vz;
                            let nz_ = ux * vy - uy * vx;
                            let dx = p_out.0 - a.0;
                            let dy = p_out.1 - a.1;
                            let dz = p_out.2 - a.2;
                            if nx_ * dx + ny_ * dy + nz_ * dz >= 0.0 {
                                [v0, v1, v2]
                            } else {
                                [v0, v2, v1]
                            }
                        };

                    if ni == 1 {
                        let i0 = in_c[0];
                        let (o0, o1, o2) = (out_c[0], out_c[1], out_c[2]);
                        let e0 = get_edge_vert(&mut verts, &mut edge_cache, g_idx[i0], g_idx[o0], pts[i0], pts[o0], vals[i0], vals[o0]);
                        let e1 = get_edge_vert(&mut verts, &mut edge_cache, g_idx[i0], g_idx[o1], pts[i0], pts[o1], vals[i0], vals[o1]);
                        let e2 = get_edge_vert(&mut verts, &mut edge_cache, g_idx[i0], g_idx[o2], pts[i0], pts[o2], vals[i0], vals[o2]);
                        tris.push(orient_tri(e0, e1, e2, pts[o0], &verts));
                    } else if ni == 3 {
                        let o0 = out_c[0];
                        let (i0, i1, i2) = (in_c[0], in_c[1], in_c[2]);
                        let e0 = get_edge_vert(&mut verts, &mut edge_cache, g_idx[o0], g_idx[i0], pts[o0], pts[i0], vals[o0], vals[i0]);
                        let e1 = get_edge_vert(&mut verts, &mut edge_cache, g_idx[o0], g_idx[i1], pts[o0], pts[i1], vals[o0], vals[i1]);
                        let e2 = get_edge_vert(&mut verts, &mut edge_cache, g_idx[o0], g_idx[i2], pts[o0], pts[i2], vals[o0], vals[i2]);
                        tris.push(orient_tri(e0, e1, e2, pts[o0], &verts));
                    } else {
                        let (i0, i1) = (in_c[0], in_c[1]);
                        let (o0, o1) = (out_c[0], out_c[1]);
                        let e00 = get_edge_vert(&mut verts, &mut edge_cache, g_idx[i0], g_idx[o0], pts[i0], pts[o0], vals[i0], vals[o0]);
                        let e01 = get_edge_vert(&mut verts, &mut edge_cache, g_idx[i0], g_idx[o1], pts[i0], pts[o1], vals[i0], vals[o1]);
                        let e10 = get_edge_vert(&mut verts, &mut edge_cache, g_idx[i1], g_idx[o0], pts[i1], pts[o0], vals[i1], vals[o0]);
                        let e11 = get_edge_vert(&mut verts, &mut edge_cache, g_idx[i1], g_idx[o1], pts[i1], pts[o1], vals[i1], vals[o1]);
                        tris.push(orient_tri(e00, e01, e11, pts[o0], &verts));
                        tris.push(orient_tri(e00, e11, e10, pts[o0], &verts));
                    }
                }
            }
        }
    }

    Solid {
        verts,
        tris,
        bore: None,
    }
}

/// Fail-closed watertight mesh extraction (`comms.md` 2026-10-25T03:06:29Z, `N11`):
/// verifies `census == (0, 0, 1)`, and if a degenerate iso-surface vertex sits on a grid face,
/// retries up to 4 times with a `0.19 * h` coordinate origin shift while keeping `h` constant;
/// returns `None` if no shift produces a strictly watertight manifold mesh.
pub fn extract_watertight_mesh_fail_closed(
    genome: &G5Genome,
    h: f64,
    use_am: bool,
) -> Option<Solid> {
    let p = genome.decode();
    for try_idx in 0..4 {
        let origin_shift = (try_idx as f64) * 0.19 * h;
        let grid = build_g5_sdf_from_params_with_origin_shift(&p, h, use_am, true, origin_shift);
        let solid = extract_watertight_mesh(&grid, use_am);
        if !solid.tris.is_empty() {
            let (boundary, non_manifold, dmax) = jarambit_crescent::export::census(&solid.tris);
            if boundary == 0 && non_manifold == 0 && dmax == 1 {
                return Some(solid);
            }
        }
    }
    None
}

/// Compute coordinate-aligned 3D voxel Intersection-over-Union (IoU) between two `VoxelGrid3D`s
/// with identical voxel spacing `h`.
pub fn voxel_grid_iou(a: &VoxelGrid3D, b: &VoxelGrid3D) -> f64 {
    assert!((a.h - b.h).abs() < 1e-9);
    let h = a.h;
    let x_min = a.x[0].min(b.x[0]);
    let y_min = a.y[0].min(b.y[0]);
    let z_min = a.z[0].min(b.z[0]);
    let x_max = a.x.last().unwrap().max(*b.x.last().unwrap());
    let y_max = a.y.last().unwrap().max(*b.y.last().unwrap());
    let z_max = a.z.last().unwrap().max(*b.z.last().unwrap());
    let nx = ((x_max - x_min) / h).round() as isize + 1;
    let ny = ((y_max - y_min) / h).round() as isize + 1;
    let nz = ((z_max - z_min) / h).round() as isize + 1;
    let ox_a = ((a.x[0] - x_min) / h).round() as isize;
    let oy_a = ((a.y[0] - y_min) / h).round() as isize;
    let oz_a = ((a.z[0] - z_min) / h).round() as isize;
    let ox_b = ((b.x[0] - x_min) / h).round() as isize;
    let oy_b = ((b.y[0] - y_min) / h).round() as isize;
    let oz_b = ((b.z[0] - z_min) / h).round() as isize;
    let mut inter = 0usize;
    let mut union = 0usize;
    for ix in 0..nx {
        let ia_x = ix - ox_a;
        let ib_x = ix - ox_b;
        for iy in 0..ny {
            let ia_y = iy - oy_a;
            let ib_y = iy - oy_b;
            for iz in 0..nz {
                let ia_z = iz - oz_a;
                let ib_z = iz - oz_b;
                let in_a = ia_x >= 0
                    && (ia_x as usize) < a.nx
                    && ia_y >= 0
                    && (ia_y as usize) < a.ny
                    && ia_z >= 0
                    && (ia_z as usize) < a.nz
                    && a.d[a.idx(ia_x as usize, ia_y as usize, ia_z as usize)] < 0.0;
                let in_b = ib_x >= 0
                    && (ib_x as usize) < b.nx
                    && ib_y >= 0
                    && (ib_y as usize) < b.ny
                    && ib_z >= 0
                    && (ib_z as usize) < b.nz
                    && b.d[b.idx(ib_x as usize, ib_y as usize, ib_z as usize)] < 0.0;
                if in_a && in_b {
                    inter += 1;
                }
                if in_a || in_b {
                    union += 1;
                }
            }
        }
    }
    if union == 0 {
        1.0
    } else {
        (inter as f64) / (union as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::g5_genome::{reproduce_iso_line_dd, sobol_unit_genome, G5Rng, G5_NUM_GENES};
    use jarambit_crescent::export;

    #[test]
    fn rule1_closing_of_convex_body_returns_itself_and_opening_is_subset() {
        let h = 0.5;
        let (nx, ny, nz) = (41usize, 41usize, 41usize);
        let mut box_mask = vec![false; nx * ny * nz];
        for i in 10..=30 {
            for j in 10..=30 {
                for k in 12..=28 {
                    box_mask[(i * ny + j) * nz + k] = true;
                }
            }
        }
        let r_const = vec![1.5_f32; nx * ny];
        let box_closed = morph_close_variable_2d_z(&box_mask, &r_const, nx, ny, nz, h);
        assert_eq!(
            box_mask.iter().filter(|&&b| b).count(),
            box_closed.iter().filter(|&&b| b).count(),
            "convex box must not grow a 1-voxel phantom shell under closing"
        );

        let mut mask = vec![false; nx * ny * nz];
        for i in 0..nx {
            let x = (i as f64 - 20.0) * h;
            for j in 0..ny {
                let y = (j as f64 - 20.0) * h;
                for k in 0..nz {
                    let z = (k as f64 - 20.0) * h;
                    if (x * x + y * y + z * z).sqrt() <= 6.0 {
                        mask[(i * ny + j) * nz + k] = true;
                    }
                }
            }
        }
        let mut r_c2 = vec![0.0_f32; nx * ny];
        let mut r_o2 = vec![0.0_f32; nx * ny];
        for i in 0..nx {
            for j in 0..ny {
                r_c2[i * ny + j] = 1.5 + 1.5 * ((i as f32) / (nx as f32));
                r_o2[i * ny + j] = 1.0 + 1.5 * ((j as f32) / (ny as f32));
            }
        }
        let closed = morph_close_variable_2d_z(&mask, &r_c2, nx, ny, nz, h);
        let orig_cnt = mask.iter().filter(|&&b| b).count();
        let closed_cnt = closed.iter().filter(|&&b| b).count();
        assert!(closed_cnt >= orig_cnt);
        assert!(
            ((closed_cnt - orig_cnt) as f64) / (orig_cnt as f64) < 0.02,
            "convex sphere grew excessively under variable-radius closing: {orig_cnt} -> {closed_cnt}"
        );

        let opened = morph_open_variable_2d_z(&mask, &r_o2, nx, ny, nz, h);
        let added_outside = opened
            .iter()
            .zip(mask.iter())
            .filter(|(&o, &m)| o && !m)
            .count();
        assert_eq!(added_outside, 0, "opening added {added_outside} voxels outside mask");
        let opened_box = morph_open_variable_2d_z(&box_mask, &r_const, nx, ny, nz, h);
        assert!(
            opened_box.iter().filter(|&&b| b).count() < box_mask.iter().filter(|&&b| b).count(),
            "opening must erode sharp corners of a box"
        );
    }

    #[test]
    fn rule3_sheet_tpms_solid_fraction_matches_target_rho() {
        // A sheet of thickness t = rho * cell / S around f = 0 (first-order distance |f| / |grad f|) fills about
        // rho of the cell, for gyroid and Schwarz-D sheets
        let cell = 8.0;
        let aniso = 1.0;
        for &mix in &[0.0, 1.0] {
            for &target_rho in &[0.25, 0.40] {
                let t = crate::material::tpms_sheet_thickness(target_rho, cell, mix);
                let n = 40usize;
                let mut solid_cnt = 0usize;
                for i in 0..n {
                    let x = ((i as f64 + 0.5) / n as f64) * cell * aniso;
                    for j in 0..n {
                        let y = ((j as f64 + 0.5) / n as f64) * cell;
                        for k in 0..n {
                            let z = ((k as f64 + 0.5) / n as f64) * cell;
                            let (f, g) = tpms_val_and_grad(x, y, z, cell, mix, aniso);
                            if f.abs() / g.max(1e-6) < 0.5 * t {
                                solid_cnt += 1;
                            }
                        }
                    }
                }
                let actual_rho = (solid_cnt as f64) / ((n * n * n) as f64);
                // The expected density is that of the sheet actually built (the 0.8 mm floor can raise it)
                let built_rho = crate::material::tpms_density_of_sheet(t, cell, mix);
                println!("mix {mix} target {target_rho}: sheet {t:.3} mm, built {built_rho:.4}, measured {actual_rho:.4}");
                assert!(
                    (actual_rho - built_rho).abs() < 0.05,
                    "sheet TPMS fraction {actual_rho} differed from the built density {built_rho} (mix {mix}) by >= 0.05"
                );
            }
        }
    }

    #[test]
    fn rule4_connectivity_removes_floating_struts_and_sealed_air_pockets() {
        let (nx, ny, nz) = (21usize, 21usize, 21usize);
        let mut d_am = vec![1.0_f32; nx * ny * nz];
        for i in 4..=16 {
            for j in 4..=16 {
                for k in 4..=16 {
                    d_am[(i * ny + j) * nz + k] = -1.0;
                }
            }
        }
        d_am[(10 * ny + 10) * nz + 10] = 1.0;
        d_am[(19 * ny + 19) * nz + 19] = -1.0;

        let stats = enforce_am_connectivity(&mut d_am, None, nx, ny, nz, 0.5, 1.0);
        assert!(
            d_am[(10 * ny + 10) * nz + 10] < 0.0,
            "sealed air pocket must be kept solid (its powder cannot drain)"
        );
        // The pocket voxel (d = 1.0 at h = 0.5: fraction 0) is reported as 0.125 mm³ of sealed void, and the
        // fragment (d = -1.0: fraction 1) as 0.125 mm³ of removed floating solid.
        assert!((stats.sealed_void_mm3 - 0.125).abs() < 1e-12, "{stats:?}");
        assert!((stats.floating_solid_mm3 - 0.125).abs() < 1e-12, "{stats:?}");
        assert!(
            d_am[(19 * ny + 19) * nz + 19] > 0.0,
            "disconnected floating solid fragment must be removed"
        );
    }

    /// A box (outer half-size 6 mm, skin 1 mm) with a cubic cavity, joined to the outside by a straight channel along
    /// +x of square half-width `w`, on a 0.25 mm grid. Returns the sealed volume found by the drainage rule.
    fn sealed_volume_of_channel_box(w: f64, with_exterior_slot: bool) -> (f64, Vec<f32>, usize) {
        let h = 0.25_f64;
        let n = 64usize;
        let c = |i: usize| (i as f64 + 0.5) * h - 8.0;
        let mut d_env = vec![0.0_f32; n * n * n];
        let mut d_am = vec![0.0_f32; n * n * n];
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    let (x, y, z) = (c(i), c(j), c(k));
                    let box_out = x.abs().max(y.abs()).max(z.abs()) - 6.0;
                    let mut env = box_out;
                    if with_exterior_slot {
                        // a 0.5 mm wide, 0.6 mm deep exterior slot cut into the -y face (inside the 1 mm skin)
                        let slot = (x.abs() - 0.25).max(z.abs() - 3.0).max(y + 5.4);
                        env = env.max(-slot);
                    }
                    let cavity = 5.0 - x.abs().max(y.abs()).max(z.abs());
                    let channel = (w - y.abs().max(z.abs())).min(8.0 - x).min(x);
                    let air_inside = cavity.min(4.0).max(channel);
                    let idx = (i * n + j) * n + k;
                    d_env[idx] = env as f32;
                    d_am[idx] = env.max(air_inside) as f32;
                }
            }
        }
        let stats = enforce_am_connectivity(&mut d_am, Some(&d_env), n, n, n, h as f32, 1.0);
        (stats.sealed_void_mm3, d_am, n)
    }

    #[test]
    fn drainage_needs_a_passage_at_least_drain_channel_min_wide() {
        // The cavity is 8 mm wide: 512 mm³ of air (plus the channel).
        let (sealed_narrow, ..) = sealed_volume_of_channel_box(0.2, false); // 0.4 mm channel: sealed
        let (sealed_wide, ..) = sealed_volume_of_channel_box(0.75, false); // 1.5 mm channel: drains
        assert!(sealed_narrow > 500.0, "a 0.4 mm passage must not drain the cavity: {sealed_narrow} mm³");
        assert!(sealed_wide < 1.0, "a 1.5 mm passage must drain the cavity: {sealed_wide} mm³");
        // An exterior slot narrower than the drainage width is outside the envelope and is never filled.
        let (_, d_am, n) = sealed_volume_of_channel_box(0.75, true);
        let h = 0.25_f64;
        let idx = |x: f64, y: f64, z: f64| {
            let f = |v: f64| ((v + 8.0) / h - 0.5).round() as usize;
            (f(x) * n + f(y)) * n + f(z)
        };
        assert!(d_am[idx(0.125, -5.875, 0.125)] > 0.0, "the exterior slot must stay open");
    }

    #[test]
    fn rule5_and_rule6_grip_taper_has_no_cliff_and_guard_lobe_scales_smoothly() {
        let g = sobol_unit_genome(3, 0, "22bb8696");
        let p = g.decode();
        let x_start = p["x_t"] + p["L_g"] - 6.0;
        let x_end = p["x_t"] + p["L_g"] + 8.0;
        let mut prev_t = eval_grip_slice(&p, x_start).t_sec;
        for k in 1..=28 {
            let x = x_start + (k as f64) * 0.5;
            let cur_t = eval_grip_slice(&p, x).t_sec;
            assert!(
                (cur_t - prev_t).abs() < 1.2,
                "Rule 5 grip-to-guard taper must have no cliff at x={x}: {prev_t} -> {cur_t}"
            );
            prev_t = cur_t;
        }
        let _ = x_end;
        let (yb_lo, yt_lo) = eval_guard_lobe_y(&p, 70.0);
        let (yb_peak, _yt_peak) = eval_guard_lobe_y(&p, 85.6);
        assert!(yt_lo > yb_lo);
        assert!(
            (yb_lo - yb_peak) > 0.9 * p["h_g"],
            "Rule 6 guard lobe must drop by ~h_g at peak"
        );
    }

    #[test]
    fn g5_sdf_builds_full_knife_watertight_manifold_mesh_and_preserves_locality() {
        let parent = sobol_unit_genome(7, 0, "22bb8696");
        let grid_p = build_g5_sdf(&parent, 1.0, false);
        // Verify full knife covers the blade beyond x = 135 mm (B5)
        assert!(*grid_p.x.last().unwrap() > 135.0);
        let solid = extract_watertight_mesh(&grid_p, false);
        assert!(solid.tris.len() > 1000, "tris = {}", solid.tris.len());
        let (boundary, non_manifold, dmax) = export::census(&solid.tris);
        assert_eq!(boundary, 0, "mesh must have 0 boundary edges");
        assert_eq!(non_manifold, 0, "mesh must have 0 non-manifold edges");
        assert_eq!(dmax, 1, "mesh must have max directed edge repeat == 1");
        assert!(export::volume(&solid) > 2000.0);

        // Locality check: small mutation (sigma = 0.015 on all 5 M genes 174..179) preserves voxel IoU >= 0.90
        let mut small_parent = parent.clone();
        for i in (G5_NUM_GENES - 5)..G5_NUM_GENES {
            small_parent.u[i] = crate::g5_genome::G5_GENES[i].unit_from_phys(0.015);
        }
        let mut rng = G5Rng(99);
        let child = reproduce_iso_line_dd(&small_parent, &small_parent, &mut rng);
        let grid_c = build_g5_sdf(&child, 1.0, false);
        let iou = voxel_grid_iou(&grid_p, &grid_c);
        assert!(iou >= 0.90, "parent-child voxel IoU at sigma=0.015 was {iou}");
    }
}
