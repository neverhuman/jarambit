//! `G5.0` 139-gene modular genome schema (derived from G3's 179-gene `G3_GENOME_SPEC.md` v1.1) for the
//! fixed-blade, sheath-free, hollow-capable L-PBF Ti-6Al-4V Jarambit under RS-karambit-v5.
//!
//! Design principles (Owner directive 2026-10-25T01:13:42Z):
//! - **Evolutionary search drives all 3D geometry**: no agent-hand-crafted individuals, no hidden
//!   offspring gene overrides, and no v1 parent-pool seeds.
//! - **Unit hypercube encoding**: `u in [0, 1]^139`, mapped linearly or log-uniformly to physical
//!   parameters by `decode()`. Sobol starts use the first 139 Joe-Kuo dimensions.
//! - **Strict CSV/JSON loader**: requires `schema=G5.0` and a verified `genome_sha256` over exact
//!   IEEE-754 little-endian bits; refuses v1 90/117/128-gene rows.
//! - **Variation**: reflective `Iso+Line-DD` mutation (`child = a + sigma_m * N(0, I) + 0.25 * N(0, 1) * (b - a)`),
//!   module-level crossover (`p = 0.15`), and self-adaptive log-normal mutation of `sigma_B..sigma_I` (Module M).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const G5_SCHEMA: &str = "G5.0";
pub const G5_NUM_GENES: usize = 139;
pub const FIXED_BORE_D_MM: f64 = 24.9;
pub const FIXED_LIP_W_MM: f64 = 8.0;
pub const M50_PITCHES_MM: [f64; 3] = [19.5, 21.0, 22.0];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum G5Module {
    /// Hand zone (ring, grip, guard)
    H,
    /// Surface morphogen (knurl, Turing, Voronoi, ridge, 32-weight CPPN, zone scopes)
    S,
    /// Interior 3D architecture (skin, TPMS gyroid/Schwarz-D, anisotropic cell, density grading, shear webs)
    I,
    /// Blade planform, 3D dorsal/medial ridge, fuller corrugation, and 5-gene level-set fenestration
    B,
    /// Double-edged, double-serrated cutting bevels with `tip_reserve` (Gate S1/S2/S3 by construction)
    E,
    /// Self-adaptive per-module mutation step sizes `sigma_B..sigma_I`
    M,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeneMap {
    Linear,
    Log,
}

#[derive(Debug, Clone, Copy)]
pub struct GeneSpec {
    pub name: &'static str,
    pub module: G5Module,
    pub lo: f64,
    pub hi: f64,
    pub default_phys: f64,
    pub map: GeneMap,
}

impl GeneSpec {
    #[inline]
    pub fn phys_from_unit(&self, u: f64) -> f64 {
        let u = u.clamp(0.0, 1.0);
        match self.map {
            GeneMap::Linear => self.lo + u * (self.hi - self.lo),
            GeneMap::Log => {
                let l0 = self.lo.max(1e-9).ln();
                let l1 = self.hi.max(1e-9).ln();
                (l0 + u * (l1 - l0)).exp()
            }
        }
    }

    #[inline]
    pub fn unit_from_phys(&self, p: f64) -> f64 {
        let p = p.clamp(self.lo, self.hi);
        match self.map {
            GeneMap::Linear => ((p - self.lo) / (self.hi - self.lo).max(1e-12)).clamp(0.0, 1.0),
            GeneMap::Log => {
                let l0 = self.lo.max(1e-9).ln();
                let l1 = self.hi.max(1e-9).ln();
                ((p.max(1e-9).ln() - l0) / (l1 - l0).max(1e-12)).clamp(0.0, 1.0)
            }
        }
    }
}

macro_rules! lin_gene {
    ($name:expr, $mod:ident, $lo:expr, $hi:expr, $def:expr) => {
        GeneSpec {
            name: $name,
            module: G5Module::$mod,
            lo: $lo,
            hi: $hi,
            default_phys: $def,
            map: GeneMap::Linear,
        }
    };
}

macro_rules! log_gene {
    ($name:expr, $mod:ident, $lo:expr, $hi:expr, $def:expr) => {
        GeneSpec {
            name: $name,
            module: G5Module::$mod,
            lo: $lo,
            hi: $hi,
            default_phys: $def,
            map: GeneMap::Log,
        }
    };
}

/// Complete 139-gene table for `G5.0`: the G3 table without its 37 texture genes, the two retired edge-stop genes,
/// the five cusped-serration genes and `exo_open` (G3's port-radius gene), plus five genes in requirement units
/// (grip perimeter and H/T, port diameter, point angle, point length).
pub static G5_GENES: [GeneSpec; G5_NUM_GENES] = [
    // ---- Module H: Ring (11 genes, 0..11) ----
    lin_gene!("ring_od", H, 29.5, 35.0, 30.6),
    lin_gene!("crown", H, 0.2, 0.4, 0.3),
    lin_gene!("lip_d", H, 1.5, 2.8, 2.0),
    lin_gene!("taper_deg", H, 30.0, 75.0, 45.0),
    // v3 (2026-10-25): web_t >= 2.7 mm so the ring outer spine keeps R >= 1.0 mm with the 0.75 * t/2 round
    lin_gene!("web_t", H, 2.7, 4.0, 3.0),
    lin_gene!("thumbpad", H, 0.0, 3.5, 0.0),
    lin_gene!("od_tear", H, -1.0, 2.0, 0.8),
    lin_gene!("od_h2", H, -0.6, 0.6, 0.0),
    lin_gene!("junction_deg", H, 30.0, 50.0, 40.0),
    lin_gene!("neck_r", H, 4.0, 12.0, 8.0),
    lin_gene!("bore_round", H, 2.0, 3.5, 2.8),
    // ---- Module H: Grip (29 genes, 11..40) ----
    lin_gene!("hand_scale", H, 0.90, 1.10, 1.0),
    lin_gene!("dL", H, 4.0, 14.0, 9.0), // G5.0: grip length >= 69 mm with 19-21 mm finger pitches
    lin_gene!("T0", H, 8.0, 12.0, 10.0),
    lin_gene!("T_sw", H, 13.0, 17.0, 15.0),
    lin_gene!("T_g", H, 11.0, 15.0, 13.0),
    lin_gene!("s_sw", H, 0.40, 0.60, 0.50),
    lin_gene!("yt0", H, 10.5, 14.5, 12.5),
    lin_gene!("yt_sw", H, 12.0, 16.0, 14.0),
    lin_gene!("s_ps", H, 0.35, 0.50, 0.42),
    lin_gene!("yt_land", H, 11.0, 14.5, 13.0),
    lin_gene!("yb0", H, -12.0, -9.0, -10.5),
    lin_gene!("yb_g", H, -13.5, -10.5, -12.0),
    lin_gene!("belly", H, -1.5, 1.0, 0.0),
    // G5.0: the swell section is scaled to this perimeter and height/thickness (RS-karambit-v5 [ergonomics.grip])
    lin_gene!("grip_perim", H, 78.0, 104.0, 92.0),
    lin_gene!("grip_hot", H, 1.40, 1.90, 1.65),
    lin_gene!("d_ring", H, 1.0, 3.0, 2.0),
    lin_gene!("w_ring", H, 7.0, 11.0, 9.0),
    lin_gene!("d_mid", H, 0.0, 3.0, 2.1),
    lin_gene!("w_mid", H, 7.0, 12.0, 10.0),
    lin_gene!("d_idx", H, 2.5, 5.0, 4.2),
    lin_gene!("w_idx", H, 8.0, 14.0, 12.0),
    lin_gene!("off_ring", H, -2.0, 2.0, 0.0),
    lin_gene!("off_mid", H, -2.0, 2.0, 0.0),
    lin_gene!("off_idx", H, -2.0, 2.0, 0.0),
    lin_gene!("n0", H, 2.2, 3.2, 2.5),
    lin_gene!("n1", H, 2.2, 3.2, 2.5),
    lin_gene!("skew", H, -0.20, 0.20, 0.0),
    lin_gene!("edge_r", H, 3.0, 4.5, 3.5), // G5.0: >= 3 mm on 6.5-25 mm sections [ergonomics.edges]
    lin_gene!("ramp_h", H, 0.0, 3.0, 1.5),
    // ---- Module H: Guard & spine (6 genes, 40..46) ----
    lin_gene!("h_g", H, 10.0, 18.0, 15.0), // G5.0: guard target 15 mm, floor 10 mm
    lin_gene!("t_guard", H, 5.5, 10.0, 6.5),
    lin_gene!("guard_r", H, 5.0, 11.0, 7.0),
    lin_gene!("jimp_n", H, 0.0, 8.0, 6.0),
    lin_gene!("jimp_p", H, 1.8, 2.8, 2.5),
    lin_gene!("jimp_d", H, 0.4, 0.7, 0.6),
    // ---- Module S: Surface texture: knurl and ridge, zero-mean (5 genes, 46..51) ----
    lin_gene!("tex_amp", S, 0.0, 0.25, 0.18),
    lin_gene!("tex_pitch", S, 1.2, 2.6, 1.6),
    lin_gene!("tex_angle", S, -45.0, 45.0, 0.0),
    lin_gene!("w_knurl", S, 0.0, 1.0, 1.0),
    lin_gene!("w_ridge", S, 0.0, 1.0, 0.0),
    // ---- Module I: Interior base (7 genes, 51..58) ----
    lin_gene!("skin_t", I, 0.8, 1.2, 1.0),
    lin_gene!("lat_mix", I, 0.0, 1.0, 0.0),
    lin_gene!("lat_cell", I, 5.0, 12.0, 8.0), // G5.0: sheet TPMS >= 0.8 mm needs larger cells for low density
    lin_gene!("rho_grip", I, 0.06, 1.0, 0.10),
    lin_gene!("rho_guard", I, 0.06, 1.0, 1.0),
    lin_gene!("rho_ring", I, 0.06, 1.0, 1.0),
    lin_gene!("rib_w", I, 0.0, 1.0, 0.0),
    // ---- Module H v2 (T9): grip sweep starts this far inside the ring tangent with a rounded end cap (gene 58) ----
    lin_gene!("neck_in", H, 3.0, 8.0, 5.0),
    // ---- Module S extensions (3 genes, 59..62) ----
    lin_gene!("scope_grip", S, 0.0, 1.0, 1.0),
    lin_gene!("scope_ring", S, 0.0, 1.0, 0.15),
    lin_gene!("scope_blade", S, 0.0, 1.0, 0.25),
    // ---- Module I extensions (8 genes, 62..70) ----
    lin_gene!("rho_blade", I, 0.06, 1.0, 0.35),
    lin_gene!("lat_grade", I, 0.0, 1.0, 0.20),
    lin_gene!("port_n", I, 2.0, 4.0, 2.0),
    lin_gene!("port_d", I, 3.0, 4.0, 3.2), // G5.0: powder port and drain-hole diameter >= [am] port_d_min_mm
    lin_gene!("lat_aniso_x", I, 0.70, 1.60, 1.15),
    lin_gene!("rho_grad_wall", I, 0.0, 0.50, 0.20),
    lin_gene!("shear_web_amp", I, 0.0, 1.0, 0.30),
    // ---- Module B: Blade planform, 3D ridge, fuller corrugation, fenestration (44 genes, 70..114) ----
    lin_gene!("blade_len", B, 60.0, 85.725, 85.725),
    lin_gene!("sweep_deg", B, -12.0, 12.0, -3.5),
    lin_gene!("hook_drop", B, 25.0, 55.0, 41.5),
    lin_gene!("blade_belly", B, -6.0, 6.0, 0.0),
    lin_gene!("spine_arch", B, -4.0, 8.0, 2.0),
    lin_gene!("tip_angle", B, 18.0, 40.0, 26.0),
    // G5.0 point module: a rounded wedge of this included planform angle over the last point_len mm (its tip
    // radius is the construction constant `g5_sdf::POINT_TIP_RADIUS_MM`)
    lin_gene!("point_angle", B, 30.0, 80.0, 55.0),
    lin_gene!("point_len", B, 3.0, 12.0, 6.0),
    lin_gene!("h_root", B, 18.0, 30.0, 25.5),
    lin_gene!("h_exp", B, 0.60, 1.60, 1.05),
    // Inner-edge cosine basis offsets a_k in +-(6/k) mm
    lin_gene!("edge_in_c1", B, -6.0, 6.0, 0.0),
    lin_gene!("edge_in_c2", B, -3.0, 3.0, 0.0),
    lin_gene!("edge_in_c3", B, -2.0, 2.0, 0.0),
    lin_gene!("edge_in_c4", B, -1.5, 1.5, 0.0),
    lin_gene!("edge_in_c5", B, -1.2, 1.2, 0.0),
    lin_gene!("edge_in_c6", B, -1.0, 1.0, 0.0),
    lin_gene!("edge_in_c7", B, -0.857, 0.857, 0.0),
    lin_gene!("edge_in_c8", B, -0.75, 0.75, 0.0),
    // Spine-edge cosine basis offsets b_k in +-(6/k) mm
    lin_gene!("edge_sp_c1", B, -6.0, 6.0, 0.0),
    lin_gene!("edge_sp_c2", B, -3.0, 3.0, 0.0),
    lin_gene!("edge_sp_c3", B, -2.0, 2.0, 0.0),
    lin_gene!("edge_sp_c4", B, -1.5, 1.5, 0.0),
    lin_gene!("edge_sp_c5", B, -1.2, 1.2, 0.0),
    lin_gene!("edge_sp_c6", B, -1.0, 1.0, 0.0),
    lin_gene!("edge_sp_c7", B, -0.857, 0.857, 0.0),
    lin_gene!("edge_sp_c8", B, -0.75, 0.75, 0.0),
    // Thickness & 3D cross-section ridge + fuller corrugation
    lin_gene!("t_core", B, 4.0, 7.0, 5.8),
    lin_gene!("t_tip", B, 1.5, 5.0, 2.4), // G5.0: a stout point can carry the tip loads
    lin_gene!("taper_x0", B, 0.35, 0.80, 0.55),
    lin_gene!("fuller_depth", B, 0.0, 1.8, 0.9),
    lin_gene!("fuller_width", B, 3.0, 10.0, 6.0),
    lin_gene!("fuller_s0", B, 0.05, 0.30, 0.12),
    lin_gene!("fuller_s1", B, 0.45, 0.85, 0.70),
    lin_gene!("ridge_amp", B, 0.0, 1.8, 0.6),
    lin_gene!("ridge_w", B, 2.5, 7.0, 4.5),
    lin_gene!("ridge_s0", B, 0.05, 0.25, 0.10),
    lin_gene!("ridge_s1", B, 0.55, 0.92, 0.80),
    lin_gene!("fuller_corrug_amp", B, 0.0, 1.2, 0.3),
    // 5-gene level-set medial fenestration field
    lin_gene!("win_amp", B, 0.0, 1.0, 0.25),
    lin_gene!("win_pitch", B, 6.0, 18.0, 10.0),
    lin_gene!("win_s0", B, 0.08, 0.35, 0.15),
    lin_gene!("win_s1", B, 0.45, 0.85, 0.68),
    lin_gene!("win_angle", B, -40.0, 40.0, 15.0),
    // ---- Module E: Double-edged, double-serrated bevels + tip_reserve (22 genes, 114..136) ----
    lin_gene!("tip_reserve", E, 4.0, 12.0, 8.0), // G5.0: the tip may thicken sooner (tip twist)
    lin_gene!("t_0p25_target", E, 0.18, 0.28, 0.24),
    lin_gene!("t_1p00_target", E, 0.38, 0.56, 0.48),
    lin_gene!("t_2p00_target", E, 0.65, 0.95, 0.82),
    // Inner edge `in` (13)
    lin_gene!("s0_in", E, 0.05, 0.12, 0.06),
    lin_gene!("angle_in", E, 20.0, 25.0, 22.0), // G5.0: per-side angle [edge]
    lin_gene!("edge_t_in", E, 0.05, 0.12, 0.08),
    lin_gene!("grind_curv_in", E, -0.35, 0.45, 0.0),
    lin_gene!("ser_s0_in", E, 0.16, 0.24, 0.20), // SERR-3: starts past the 5-12 % plunge
    lin_gene!("ser_s1_in", E, 0.62, 0.82, 0.70),
    lin_gene!("ser_pitch_in", E, 4.0, 6.0, 5.0), // G5.0: [serration] pitch
    lin_gene!("ser_depth_in", E, 0.8, 1.5, 1.0), // G5.0: [serration] depth
    lin_gene!("ser_duty_in", E, 0.45, 0.85, 0.65),
    // Spine edge `sp` (12)
    lin_gene!("s0_sp", E, 0.05, 0.12, 0.06),
    lin_gene!("angle_sp", E, 20.0, 25.0, 22.0), // G5.0: per-side angle [edge]
    lin_gene!("edge_t_sp", E, 0.05, 0.12, 0.08),
    lin_gene!("grind_curv_sp", E, -0.35, 0.45, 0.0),
    lin_gene!("ser_s0_sp", E, 0.16, 0.24, 0.20),
    lin_gene!("ser_s1_sp", E, 0.62, 0.80, 0.68),
    lin_gene!("ser_pitch_sp", E, 4.0, 6.0, 5.0), // G5.0: [serration] pitch
    lin_gene!("ser_depth_sp", E, 0.8, 1.5, 1.0), // G5.0: [serration] depth
    lin_gene!("ser_duty_sp", E, 0.45, 0.85, 0.65),
    // ---- Module M: Self-adaptive per-module mutation step sizes (5 genes, 136..139) ----
    log_gene!("sigma_B", M, 0.01, 0.20, 0.05),
    log_gene!("sigma_E", M, 0.01, 0.20, 0.04),
    log_gene!("sigma_H", M, 0.01, 0.20, 0.05),
    log_gene!("sigma_S", M, 0.01, 0.20, 0.06),
    log_gene!("sigma_I", M, 0.01, 0.20, 0.05),
];

#[derive(Deserialize)]
struct RawG5Genome {
    schema: String,
    decoder_commit: String,
    u: Vec<f64>,
}

impl TryFrom<RawG5Genome> for G5Genome {
    type Error = String;
    fn try_from(raw: RawG5Genome) -> Result<Self, Self::Error> {
        let g = G5Genome {
            schema: raw.schema,
            decoder_commit: raw.decoder_commit,
            u: raw.u,
        };
        g.validate()?;
        Ok(g)
    }
}

/// Unit-hypercube genome `u in [0, 1]^139` with strict `G5.0` provenance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "RawG5Genome")]
pub struct G5Genome {
    pub schema: String,
    pub decoder_commit: String,
    pub u: Vec<f64>,
}

impl G5Genome {
    /// Fail-closed validation of schema, decoder_commit, gene count (`G5_NUM_GENES`), and unit-cube `[0, 1]` bounds.
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != G5_SCHEMA {
            return Err(format!(
                "strict G5.0 validator refused schema '{}' (expected '{G5_SCHEMA}')",
                self.schema
            ));
        }
        if self.decoder_commit.trim().is_empty() {
            return Err("strict G5.0 validator refused empty decoder_commit".to_string());
        }
        if self.u.len() != G5_NUM_GENES {
            return Err(format!(
                "strict G5.0 validator refused u of length {} (expected {G5_NUM_GENES})",
                self.u.len()
            ));
        }
        for (i, &v) in self.u.iter().enumerate() {
            if !v.is_finite() || !(0.0..=1.0).contains(&v) {
                return Err(format!(
                    "gene {} ({}) value {v} outside unit cube [0, 1]",
                    i, G5_GENES[i].name
                ));
            }
        }
        Ok(())
    }

    /// Reference benchmark genome (derived strictly from `default_phys` bounds for unit tests and
    /// baseline comparison; evolutionary runs initialize exclusively from `sobol_unit_genome`).
    pub fn benchmark_reference(decoder_commit: &str) -> Self {
        let u = G5_GENES
            .iter()
            .map(|g| g.unit_from_phys(g.default_phys))
            .collect();
        Self {
            schema: G5_SCHEMA.to_string(),
            decoder_commit: decoder_commit.to_string(),
            u,
        }
    }

    /// Exact IEEE-754 little-endian SHA-256 content hash of `(schema, decoder_commit, u)`.
    pub fn sha256(&self) -> String {
        let mut bytes = Vec::with_capacity(32 + self.u.len() * 8);
        bytes.extend_from_slice(self.schema.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(self.decoder_commit.as_bytes());
        bytes.push(0);
        for &v in &self.u {
            bytes.extend_from_slice(&v.to_bits().to_le_bytes());
        }
        crate::sha256_bytes(&bytes)
    }

    /// Decode unit coordinates `u in [0, 1]^139` into physical parameters and derived hand-zone datums.
    /// Fails closed (panics) if `self.validate()` fails (`I11`, `N9`).
    pub fn decode(&self) -> BTreeMap<&'static str, f64> {
        self.validate()
            .expect("G5Genome::decode requires a valid G5.0 genome");
        let mut p = BTreeMap::new();
        for (spec, &u_val) in G5_GENES.iter().zip(self.u.iter()) {
            p.insert(spec.name, spec.phys_from_unit(u_val));
        }
        // Fixed owner constants
        p.insert("bore_d", FIXED_BORE_D_MM);
        p.insert("lip_w", FIXED_LIP_W_MM);
        p.insert("sharpened_edge_cap_mm", 4.0 * 25.4);
        p.insert("blade_reach_max_mm", 85.725);
        let hs = p["hand_scale"];
        // G5.0: finger (groove) pitches within RS-karambit-v5 [ergonomics.grip] groove_pitch_min/max_mm
        let gp = &crate::requirements::rs_v5().ergonomics.grip;
        let pitch = |m50: f64| (m50 * hs).clamp(gp.groove_pitch_min_mm, gp.groove_pitch_max_mm);
        let p0 = pitch(M50_PITCHES_MM[0]);
        let p1 = pitch(M50_PITCHES_MM[1]);
        let p2 = pitch(M50_PITCHES_MM[2]);
        let l_g = p0 + p1 + p2 + p["dL"];
        let r_od = p["ring_od"] * 0.5;
        let r_b = FIXED_BORE_D_MM * 0.5;
        let guard_x = 75.6_f64;
        let x_t = guard_x - l_g;
        let cx = x_t - r_od;
        p.insert("pitch_ring", p0);
        p.insert("pitch_mid", p1);
        p.insert("pitch_idx", p2);
        p.insert("centre_ring", 0.5 * p0);
        p.insert("centre_mid", p0 + 0.5 * p1);
        p.insert("centre_idx", p0 + p1 + 0.5 * p2);
        p.insert("L_g", l_g);
        p.insert("R_od", r_od);
        p.insert("R_b", r_b);
        p.insert("x_t", x_t);
        p.insert("cx", cx);
        p
    }

    pub fn csv_header() -> String {
        let mut cols = vec![
            "schema".to_string(),
            "genome_sha256".to_string(),
            "decoder_commit".to_string(),
        ];
        for g in &G5_GENES {
            cols.push(g.name.to_string());
        }
        cols.join(",")
    }

    pub fn to_csv_row(&self) -> String {
        let mut fields = vec![
            self.schema.clone(),
            self.sha256(),
            self.decoder_commit.clone(),
        ];
        for &v in &self.u {
            // Rust shortest exact IEEE-754 f64 representation guarantees bit-exact parse() roundtrip
            fields.push(format!("{v}"));
        }
        fields.join(",")
    }

    /// Strict loader (`G3_GENOME_SPEC.md` §1): refuses any schema other than `G5.0` (including
    /// v1's 90/117/128-gene rows), refuses out-of-range coordinates, and verifies `genome_sha256`.
    pub fn from_csv_row(row: &str) -> Result<Self, String> {
        let cols: Vec<&str> = row.trim().split(',').collect();
        if cols.len() != 3 + G5_NUM_GENES {
            return Err(format!(
                "strict G5.0 loader refused row of length {} (expected {} columns = 3 header + {} genes; v1 90/117/128 rows are rejected)",
                cols.len(),
                3 + G5_NUM_GENES,
                G5_NUM_GENES
            ));
        }
        let schema = cols[0].trim();
        let expected_sha = cols[1].trim();
        let decoder_commit = cols[2].trim().to_string();
        if decoder_commit.is_empty() {
            return Err("strict G5.0 loader refused row with empty decoder_commit".to_string());
        }
        let mut u = Vec::with_capacity(G5_NUM_GENES);
        for (i, s) in cols[3..].iter().enumerate() {
            let v: f64 = s
                .trim()
                .parse()
                .map_err(|e| format!("invalid float at gene {} ({}): {e}", i, G5_GENES[i].name))?;
            u.push(v);
        }
        let genome = Self {
            schema: schema.to_string(),
            decoder_commit,
            u,
        };
        genome.validate()?;
        let actual_sha = genome.sha256();
        if actual_sha != expected_sha {
            return Err(format!(
                "G5.0 genome_sha256 mismatch: row claims {expected_sha}, computed {actual_sha}"
            ));
        }
        Ok(genome)
    }

    /// Strict loader (`I5`) that additionally verifies `decoder_commit == expected_decoder_commit`.
    pub fn from_csv_row_checked(row: &str, expected_decoder_commit: &str) -> Result<Self, String> {
        let genome = Self::from_csv_row(row)?;
        if genome.decoder_commit != expected_decoder_commit {
            return Err(format!(
                "G5.0 decoder_commit mismatch: row claims '{}', expected '{}'",
                genome.decoder_commit, expected_decoder_commit
            ));
        }
        Ok(genome)
    }
}

/// Reflective boundary map into `[0.0, 1.0]` (`G3_GENOME_SPEC.md` §4: never piles genes onto 0 or 1 bounds).
#[inline]
pub fn reflect_unit(mut x: f64) -> f64 {
    if !x.is_finite() {
        return 0.5;
    }
    x = x.rem_euclid(2.0);
    if x > 1.0 {
        2.0 - x
    } else {
        x
    }
}

/// Lightweight deterministic SplitMix64 PRNG + Box-Muller normal sampler for reproducible variation.
#[derive(Debug, Clone)]
pub struct G5Rng(pub u64);

impl G5Rng {
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        ((self.next_u64() >> 11) as f64) * (1.0 / ((1u64 << 53) as f64))
    }

    #[inline]
    pub fn normal(&mut self) -> f64 {
        let u1 = self.next_f64().clamp(1e-15, 1.0 - 1e-15);
        let u2 = self.next_f64();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
}

/// Generate a scrambled Joe-Kuo Sobol genome in $[0, 1]^{139}$ (the first `G5_NUM_GENES` of the 179 tabulated
/// `new-joe-kuo-6.21201` dimensions; any leading subset of a Sobol sequence is itself a Sobol sequence) with
/// Matousek Linear Matrix Scrambling (LMS) + digital shift seeded by `seed_offset` (zero manual design bias, zero
/// v1 or G3 seeds).
pub fn sobol_unit_genome(index: usize, seed_offset: usize, decoder_commit: &str) -> G5Genome {
    let seed = (seed_offset as u64).wrapping_add(20_261_025);
    let tbl = crate::sobol_joe_kuo::Sobol179Table::scrambled(seed);
    let u = tbl.point(index)[..G5_NUM_GENES].to_vec();
    G5Genome {
        schema: G5_SCHEMA.to_string(),
        decoder_commit: decoder_commit.to_string(),
        u,
    }
}

/// Generate an unscrambled Joe-Kuo Sobol genome (the first `G5_NUM_GENES` dimensions of
/// `scipy.stats.qmc.Sobol(d=179, scramble=False)`).
pub fn sobol_unit_genome_unscrambled(index: usize, decoder_commit: &str) -> G5Genome {
    let tbl = crate::sobol_joe_kuo::Sobol179Table::unscrambled();
    let u = tbl.point(index)[..G5_NUM_GENES].to_vec();
    G5Genome {
        schema: G5_SCHEMA.to_string(),
        decoder_commit: decoder_commit.to_string(),
        u,
    }
}

/// Produce an offspring genome via self-adaptive `Iso+Line-DD` mutation and optional module-level
/// crossover (`p = 0.15`), using reflective bounds in `[0, 1]^139` (`G3_GENOME_SPEC.md` §4).
/// Never applies any manual gene overrides.
pub fn reproduce_iso_line_dd(
    parent_a: &G5Genome,
    parent_b: &G5Genome,
    rng: &mut G5Rng,
) -> G5Genome {
    assert_eq!(parent_a.u.len(), G5_NUM_GENES);
    assert_eq!(parent_b.u.len(), G5_NUM_GENES);
    let mut child_u = parent_a.u.clone();

    // 1. With probability 0.15, swap an entire functional module from parent_b (module-aware crossover)
    if rng.next_f64() < 0.15 {
        let mods = [
            G5Module::H,
            G5Module::S,
            G5Module::I,
            G5Module::B,
            G5Module::E,
        ];
        let chosen = mods[(rng.next_u64() as usize) % mods.len()];
        for (i, spec) in G5_GENES.iter().enumerate() {
            if spec.module == chosen {
                child_u[i] = parent_b.u[i];
            }
        }
    }

    // 2. Self-adapt Module M step sizes (the last 5 genes) via log-normal mutation with reflective bounds
    let m_start = G5_NUM_GENES - 5;
    let tau = 1.0 / (2.0 * (G5_NUM_GENES as f64).sqrt()).sqrt();
    for i in m_start..G5_NUM_GENES {
        let spec = &G5_GENES[i];
        let step_u = (tau * rng.normal()) / (spec.hi / spec.lo).ln();
        child_u[i] = reflect_unit(child_u[i] + step_u);
    }

    let sigma_b = G5_GENES[m_start].phys_from_unit(child_u[m_start]);
    let sigma_e = G5_GENES[m_start + 1].phys_from_unit(child_u[m_start + 1]);
    let sigma_h = G5_GENES[m_start + 2].phys_from_unit(child_u[m_start + 2]);
    let sigma_s = G5_GENES[m_start + 3].phys_from_unit(child_u[m_start + 3]);
    let sigma_i = G5_GENES[m_start + 4].phys_from_unit(child_u[m_start + 4]);

    // 3. Iso + Line-DD variation with reflective bounds in [0, 1]. The isotropic step is sparse: each gene of a
    //    module moves with probability min(1, 3 / n_module), about three genes per module (G3 moved all 179, so a
    //    child was far from its parent in every coordinate). The line step follows the parent-parent direction.
    let n_of = |m: G5Module| G5_GENES[..m_start].iter().filter(|g| g.module == m).count().max(1);
    let line_zeta = 0.25 * rng.normal();
    for i in 0..m_start {
        let module = G5_GENES[i].module;
        let sigma_m = match module {
            G5Module::B => sigma_b,
            G5Module::E => sigma_e,
            G5Module::H => sigma_h,
            G5Module::S => sigma_s,
            G5Module::I => sigma_i,
            G5Module::M => 0.0,
        };
        let p_mut = (3.0 / n_of(module) as f64).min(1.0);
        let iso = if rng.next_f64() < p_mut { sigma_m * rng.normal() } else { 0.0 };
        let step = iso + line_zeta * (parent_b.u[i] - parent_a.u[i]);
        child_u[i] = reflect_unit(child_u[i] + step);
    }

    G5Genome {
        schema: G5_SCHEMA.to_string(),
        decoder_commit: parent_a.decoder_commit.clone(),
        u: child_u,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn g5_gene_table_has_139_unique_genes_without_texture_cppn_or_retired_genes() {
        assert_eq!(G5_GENES.len(), G5_NUM_GENES);
        let mut names: Vec<&str> = G5_GENES.iter().map(|g| g.name).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), G5_NUM_GENES, "gene names must be unique");
        for gone in ["cppn_00", "cppn_31", "w_turing", "w_voronoi", "w_cppn", "turing_mix", "seed", "s1_in", "s1_sp",
            "w_scallop_in", "w_tooth_in", "w_micro_in", "w_scallop_sp", "w_tooth_sp", "exo_open", "tip_radius"]
        {
            assert!(!names.contains(&gone), "{gone} is retired in G5.0");
        }
        for new in ["grip_perim", "grip_hot", "port_d", "point_angle", "point_len"] {
            assert!(names.contains(&new), "{new} is a G5.0 gene");
        }
        assert_eq!(G5_GENES[0].name, "ring_od");
        assert_eq!(G5_GENES[G5_NUM_GENES - 5].name, "sigma_B");
        assert_eq!(G5_GENES[G5_NUM_GENES - 1].name, "sigma_I");
        // Gene bounds that encode RS-karambit-v5 requirements
        let rs = crate::requirements::rs_v5();
        let bounds = |n: &str| {
            let g = G5_GENES.iter().find(|g| g.name == n).unwrap();
            (g.lo, g.hi)
        };
        assert_eq!(bounds("angle_in"), (rs.edge.per_side_angle_min_deg, rs.edge.per_side_angle_max_deg));
        assert_eq!(bounds("angle_sp"), (rs.edge.per_side_angle_min_deg, rs.edge.per_side_angle_max_deg));
        assert_eq!(bounds("ser_pitch_in"), (rs.serration.pitch_min_mm, rs.serration.pitch_max_mm));
        assert_eq!(bounds("ser_depth_sp"), (rs.serration.depth_min_mm, rs.serration.depth_max_mm));
        assert_eq!(bounds("port_d").0, rs.am.port_d_min_mm);
        assert!(bounds("grip_perim").0 <= rs.ergonomics.grip.perimeter_floor_mm);
        assert!(bounds("grip_perim").1 >= rs.ergonomics.grip.perimeter_max_mm);
    }

    #[test]
    fn strict_csv_and_serde_loader_roundtrips_exact_bits_and_rejects_invalid_or_v1_rows() {
        let g = sobol_unit_genome(42, 0, "22bb8696");
        let row = g.to_csv_row();
        let parsed = G5Genome::from_csv_row(&row).expect("valid G5.0 row");
        assert_eq!(g, parsed);
        assert_eq!(g.sha256(), parsed.sha256());

        // Changing decoder_commit must change genome_sha256
        let mut g_diff_commit = g.clone();
        g_diff_commit.decoder_commit = "different_commit".to_string();
        assert_ne!(g.sha256(), g_diff_commit.sha256());

        // Sub-display-precision bit perturbation must fail sha256 verification
        let mut tampered_cols: Vec<String> = row.split(',').map(String::from).collect();
        let v3: f64 = tampered_cols[3].parse().unwrap();
        tampered_cols[3] = format!("{}", f64::from_bits(v3.to_bits() ^ 1));
        assert!(
            G5Genome::from_csv_row(&tampered_cols.join(",")).is_err(),
            "tampered bit must fail strict sha256 check"
        );

        // Reject v1 90/117/128-length rows
        let v1_row = vec!["0.5"; 128].join(",");
        assert!(G5Genome::from_csv_row(&v1_row).is_err());

        // Serde JSON deserialization must also enforce G5.0 schema, length, and [0, 1] bounds (I11)
        let json_ok = serde_json::to_string(&g).unwrap();
        let g_from_json: G5Genome = serde_json::from_str(&json_ok).expect("valid JSON");
        assert_eq!(g_from_json.schema, g.schema);
        assert_eq!(g_from_json.decoder_commit, g.decoder_commit);
        assert_eq!(g_from_json.u.len(), g.u.len());
        for (&a, &b) in g.u.iter().zip(g_from_json.u.iter()) {
            assert!((a - b).abs() <= f64::EPSILON);
        }
        let bad_schema_json = json_ok.replace("\"G5.0\"", "\"v1\"");
        assert!(serde_json::from_str::<G5Genome>(&bad_schema_json).is_err());
        let bad_len_json =
            "{\"schema\":\"G5.0\",\"decoder_commit\":\"22bb8696\",\"u\":[0.5,0.5]}";
        assert!(serde_json::from_str::<G5Genome>(bad_len_json).is_err());
    }

    #[test]
    fn sobol_joe_kuo_179d_unscrambled_and_scrambled_uniformity_and_correlation() {
        // 1. Unscrambled Joe-Kuo Sobol exact bit-for-bit parity check (03:26:30Z B4 request)
        let unscr = crate::sobol_joe_kuo::Sobol179Table::unscrambled();
        let u0 = unscr.point(0);
        assert!(u0.iter().all(|&x| x.to_bits() == 0.0_f64.to_bits()));
        let u1 = unscr.point(1);
        assert!(u1.iter().all(|&x| x.to_bits() == 0.5_f64.to_bits()));
        let expected_vdc_8: [f64; 8] = [0.0, 0.5, 0.75, 0.25, 0.375, 0.875, 0.625, 0.125];
        for (idx, &exp0) in expected_vdc_8.iter().enumerate() {
            let pt = unscr.point(idx);
            assert_eq!(pt[0].to_bits(), exp0.to_bits(), "van der Corput dim 0 row {idx}");
        }
        let u1023 = unscr.point(1023);
        let exp_1023_first8 = [
            0.0009765625_f64,
            0.7529296875,
            0.6123046875,
            0.1455078125,
            0.1865234375,
            0.4384765625,
            0.1396484375,
            0.6181640625,
        ];
        for (j, &exp) in exp_1023_first8.iter().enumerate() {
            assert_eq!(u1023[j].to_bits(), exp.to_bits(), "row_1023 dim {j}");
        }
        let raw = include_str!("../../g3/tests/fixtures/sobol_test_vector.json");
        let doc: serde_json::Value = serde_json::from_str(raw).expect("parse sobol_test_vector.json");
        let tv = &doc["test_vector"];
        let first_8 = tv["first_8"].as_array().expect("first_8");
        for (r, row_val) in first_8.iter().enumerate() {
            let pt = unscr.point(r);
            let row = row_val.as_array().unwrap();
            assert_eq!(row.len(), 179);
            for (j, v) in row.iter().enumerate() {
                let exp = v.as_f64().unwrap();
                assert_eq!(pt[j].to_bits(), exp.to_bits(), "first_8[{r}][{j}] bit mismatch");
            }
        }
        let row_1023 = tv["row_1023"].as_array().expect("row_1023");
        assert_eq!(row_1023.len(), 179);
        for (j, v) in row_1023.iter().enumerate() {
            let exp = v.as_f64().unwrap();
            assert_eq!(u1023[j].to_bits(), exp.to_bits(), "row_1023[{j}] bit mismatch");
        }
        // S8-6: Also verify the SHA-256 of the full 1024 x 179 little-endian f64 payload
        // against sobol_unscrambled_179d_1024.npy (1,466,368 bytes = 183,296 f64 values).
        let mut npy_payload = Vec::with_capacity(1024 * G5_NUM_GENES * 8);
        for r in 0..1024 {
            let pt = unscr.point(r);
            for &v in &pt {
                npy_payload.extend_from_slice(&v.to_le_bytes());
            }
        }
        assert_eq!(
            crate::sha256_bytes(&npy_payload),
            "a8d371847ba33a99818aa20ccbf06c69359953828d17f96fb0dee2e34eb1f126",
            "full 1024x179 unscrambled Sobol payload must match sobol_unscrambled_179d_1024.npy bit-for-bit"
        );

        // 2. Scrambled Sobol uniformity on 64 and 1024 points and pairwise correlation check (B4)
        let tbl = crate::sobol_joe_kuo::Sobol179Table::scrambled(20_261_025);
        let pts: Vec<Vec<f64>> = (0..1024).map(|i| tbl.point(i)).collect();

        for j in 0..G5_NUM_GENES {
            let max_64 = (0..64).map(|i| pts[i][j]).fold(0.0_f64, f64::max);
            let min_64 = (0..64).map(|i| pts[i][j]).fold(1.0_f64, f64::min);
            assert!(
                max_64 > 0.85,
                "gene {j} ({}) 64-pt max {max_64} <= 0.85",
                G5_GENES[j].name
            );
            assert!(
                min_64 < 0.15,
                "gene {j} ({}) 64-pt min {min_64} >= 0.15",
                G5_GENES[j].name
            );
            let mean_1024: f64 = pts.iter().map(|r| r[j]).sum::<f64>() / 1024.0;
            assert!(
                (mean_1024 - 0.5).abs() <= 0.02,
                "gene {j} ({}) 1024-pt mean {mean_1024} outside 0.5 +- 0.02",
                G5_GENES[j].name
            );
        }

        let means: Vec<f64> = (0..G5_NUM_GENES)
            .map(|j| pts.iter().map(|r| r[j]).sum::<f64>() / 1024.0)
            .collect();
        let norms: Vec<f64> = (0..G5_NUM_GENES)
            .map(|j| {
                pts.iter()
                    .map(|r| (r[j] - means[j]).powi(2))
                    .sum::<f64>()
                    .sqrt()
            })
            .collect();
        let mut corrs = Vec::with_capacity(G5_NUM_GENES * (G5_NUM_GENES - 1) / 2);
        for a in 0..G5_NUM_GENES {
            for b in (a + 1)..G5_NUM_GENES {
                let mut cov = 0.0_f64;
                for r in &pts {
                    cov += (r[a] - means[a]) * (r[b] - means[b]);
                }
                corrs.push((cov / (norms[a] * norms[b]).max(1e-15)).abs());
            }
        }
        corrs.sort_by(|x, y| x.total_cmp(y));
        let median_corr = corrs[corrs.len() / 2];
        let p99_corr = corrs[(corrs.len() * 99) / 100];
        assert!(
            median_corr <= 1.5 * 0.021030635849726236,
            "median pairwise |corr| = {median_corr} exceeds 1.5x iid reference"
        );
        assert!(
            p99_corr <= 1.5 * 0.07975472116448021,
            "p99 pairwise |corr| = {p99_corr} exceeds 1.5x iid reference"
        );
    }

    #[test]
    fn reflective_iso_line_dd_never_clips_or_escapes_unit_cube() {
        let a = sobol_unit_genome(1, 0, "22bb8696");
        let b = sobol_unit_genome(2, 0, "22bb8696");
        let mut rng = G5Rng(12345);
        for _ in 0..200 {
            let c = reproduce_iso_line_dd(&a, &b, &mut rng);
            assert_eq!(c.u.len(), G5_NUM_GENES);
            for &v in &c.u {
                assert!(v.is_finite() && (0.0..=1.0).contains(&v));
            }
            // Reflective bounds mean genes do not pile up on exact 0.0 or 1.0
            let on_boundary = c.u.iter().filter(|&&v| v == 0.0 || v == 1.0).count();
            assert_eq!(on_boundary, 0);
        }
    }
}
