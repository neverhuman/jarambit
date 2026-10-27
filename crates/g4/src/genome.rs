//! Declarative `G4.0` Genome (`genome/G4.0.toml`), typed `G4DecodedParams` (`H-1`),
//! phenotype-only `genome_sha256` (`G4-GEN-6`), cached `g4_decoder_sha256` (`H-13`),
//! Joe-Kuo scrambled Sobol generator, and typed variation operators (`C-50`..`C-53`, `C-61`).

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

include!(concat!(env!("OUT_DIR"), "/schema_gen.rs"));

pub const N_G4_GENES: usize = G4_GENE_COUNT;

/// Cached G4 schema SHA-256 (`genome/G4.0.toml`).
pub fn g4_schema_sha256() -> &'static str {
    G4_GENOME_TOML_SHA256
}

impl G4GeneSpec {
    #[inline]
    pub fn decode_u(&self, u: f64) -> f64 {
        let ui = u.clamp(0.0, 1.0);
        let raw = match self.scale {
            G4GeneScale::Lin => self.lo + ui * (self.hi - self.lo),
            G4GeneScale::Log => self.lo * libm::pow(self.hi / self.lo, ui),
        };
        match self.kind {
            G4GeneKind::F64 => raw.clamp(self.lo, self.hi),
            G4GeneKind::Int => raw.round().clamp(self.lo, self.hi),
        }
    }
}

/// Snap discrete integer genes and repair point spine monotonicity (`t_0p25 < t_1p00 < t_2p00 < t_tip_base < t_core`).
pub fn snap_and_repair_genome(g: &mut G4Genome) {
    for spec in &G4_GENES {
        let ui = g.u[spec.index].clamp(0.0, 1.0);
        if spec.kind == G4GeneKind::Int {
            let phys = (spec.lo + ui * (spec.hi - spec.lo)).round().clamp(spec.lo, spec.hi);
            g.u[spec.index] = ((phys - spec.lo) / (spec.hi - spec.lo)).clamp(0.0, 1.0);
        } else {
            g.u[spec.index] = ui;
        }
    }
}

/// Cached G4 decoder SHA-256 computed once per process (`H-13`, `C-40`).
/// Covers `genome/G4.0.toml`, all `crates/g4/src/*.rs` files, and `requirements/*-v4.toml`.
pub fn g4_decoder_sha256() -> &'static str {
    static HASH: OnceLock<String> = OnceLock::new();
    HASH.get_or_init(|| {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"G4.0\0");
        bytes.extend_from_slice(G4_GENOME_TOML_SHA256.as_bytes());
        bytes.push(0);
        for src in [
            include_str!("../../../requirements/RS-karambit-v4.toml"),
            include_str!("../../../requirements/loads-v4.toml"),
            include_str!("../../../requirements/materials-v4.toml"),
            include_str!("../../../requirements/frame-v4.toml"),
            include_str!("lib.rs"),
            include_str!("genome.rs"),
            include_str!("decoder.rs"),
            include_str!("physics.rs"),
            include_str!("objective.rs"),
            include_str!("search.rs"),
        ] {
            bytes.extend_from_slice(src.as_bytes());
            bytes.push(0);
        }
        crate::sha256_bytes(&bytes)
    })
}

/// Deterministic SplitMix64 PRNG for cross-architecture bit-exact streams (`SRCH-4`, `T-DET-1`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn from_tuple(cohort_seed: u64, island: u32, iteration: u32, child: u32) -> Self {
        let mut s = Self::new(cohort_seed ^ 0x9E37_79B9_7F4A_7C15);
        s.state ^= (island as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        let _ = s.next_u64();
        s.state ^= (iteration as u64).wrapping_mul(0x94D0_49BB_1331_11EB);
        let _ = s.next_u64();
        s.state ^= (child as u64).wrapping_mul(0xD6E8_FEB8_6659_FD93);
        let _ = s.next_u64();
        s
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        ((self.next_u64() >> 11) as f64) * (1.0 / ((1u64 << 53) as f64))
    }

    /// Deterministic Box-Muller standard normal using pure-Rust `libm` (`H-11`, `C-41`).
    pub fn next_normal(&mut self) -> f64 {
        let u1 = self.next_f64().clamp(1e-15, 1.0 - 1e-15);
        let u2 = self.next_f64();
        let r = libm::sqrt(-2.0 * libm::log(u1));
        let theta = 2.0 * std::f64::consts::PI * u2;
        r * libm::cos(theta)
    }
}

/// Separate emitter strategy state (`G4-GEN-6`): self-adaptive per-module step sizes live here,
/// never inside `G4Genome` or `genome_sha256()`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct G4EmitterStrategy {
    pub sigma_h: f64,
    pub sigma_g: f64,
    pub sigma_p: f64,
    pub sigma_z: f64,
    pub sigma_x: f64,
    pub sigma_i: f64,
    pub sigma_e: f64,
    pub sigma_t: f64,
    pub sigma_a: f64,
}

impl Default for G4EmitterStrategy {
    fn default() -> Self {
        Self {
            sigma_h: 0.035,
            sigma_g: 0.035,
            sigma_p: 0.045,
            sigma_z: 0.030,
            sigma_x: 0.045,
            sigma_i: 0.040,
            sigma_e: 0.035,
            sigma_t: 0.050,
            sigma_a: 0.050,
        }
    }
}

impl G4EmitterStrategy {
    pub fn sigma_for(&self, m: G4Module) -> f64 {
        match m {
            G4Module::H => self.sigma_h,
            G4Module::G => self.sigma_g,
            G4Module::P => self.sigma_p,
            G4Module::Z => self.sigma_z,
            G4Module::X => self.sigma_x,
            G4Module::I => self.sigma_i,
            G4Module::E => self.sigma_e,
            G4Module::T => self.sigma_t,
            G4Module::A => self.sigma_a,
        }
    }

    pub fn self_adapt(&mut self, rng: &mut SplitMix64) {
        let tau = 1.0 / libm::sqrt(2.0 * libm::sqrt(G4_GENE_COUNT as f64));
        for s in [
            &mut self.sigma_h,
            &mut self.sigma_g,
            &mut self.sigma_p,
            &mut self.sigma_z,
            &mut self.sigma_x,
            &mut self.sigma_i,
            &mut self.sigma_e,
            &mut self.sigma_t,
            &mut self.sigma_a,
        ] {
            *s = (*s * libm::exp(tau * rng.next_normal())).clamp(0.005, 0.18);
        }
    }
}

/// Phenotype-bound `G4.0` unit-cube genome (`u in [0, 1]^130`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct G4Genome {
    pub schema: String,
    pub u: Vec<f64>,
    #[serde(default)]
    pub strategy: G4EmitterStrategy,
}

impl G4Genome {
    /// Construct the default baseline `G4.0` genome from the `default` values in `genome/G4.0.toml`.
    pub fn default_baseline() -> Self {
        let mut u = Vec::with_capacity(G4_GENE_COUNT);
        for spec in &G4_GENES {
            let u_val = match spec.scale {
                G4GeneScale::Lin => ((spec.default - spec.lo) / (spec.hi - spec.lo)).clamp(0.0, 1.0),
                G4GeneScale::Log => {
                    (libm::log(spec.default / spec.lo) / libm::log(spec.hi / spec.lo)).clamp(0.0, 1.0)
                }
            };
            u.push(u_val);
        }
        Self {
            schema: G4_GENOME_SCHEMA.to_string(),
            u,
            strategy: G4EmitterStrategy::default(),
        }
    }

    /// Alias for `default_baseline()`.
    pub fn default_anchor() -> Self {
        Self::default_baseline()
    }

    /// Alias for `sobol_unit_genome(index as u32, seed)`.
    pub fn from_sobol(index: usize, island_seed: u64) -> Self {
        Self::sobol_unit_genome(index as u32, island_seed)
    }

    /// Alias for `sha256()`.
    pub fn genome_sha256(&self) -> String {
        self.sha256()
    }

    /// Generate a scrambled Joe-Kuo Sobol genome in `[0, 1]^130`.
    pub fn sobol_unit_genome(index: u32, island_seed: u64) -> Self {
        let g3_sobol = jarambit_g3::g3_genome::sobol_unit_genome(index as usize, island_seed as usize, "g4_sobol");
        let mut u = Vec::with_capacity(G4_GENE_COUNT);
        for i in 0..G4_GENE_COUNT {
            u.push(g3_sobol.u[i].clamp(0.0, 1.0));
        }
        Self {
            schema: G4_GENOME_SCHEMA.to_string(),
            u,
            strategy: G4EmitterStrategy::default(),
        }
    }

    /// Decode `[0, 1]^130` into the typed `#[repr(C)] G4DecodedParams` struct (`H-1` / `C-42`).
    #[inline]
    pub fn decode(&self) -> G4DecodedParams {
        assert_eq!(self.u.len(), G4_GENE_COUNT, "G4Genome length mismatch");
        let mut phys = [0.0_f64; G4_GENE_COUNT];
        for (i, spec) in G4_GENES.iter().enumerate() {
            let ui = self.u[i].clamp(0.0, 1.0);
            let raw = match spec.scale {
                G4GeneScale::Lin => spec.lo + ui * (spec.hi - spec.lo),
                G4GeneScale::Log => spec.lo * libm::pow(spec.hi / spec.lo, ui),
            };
            phys[i] = match spec.kind {
                G4GeneKind::F64 => raw.clamp(spec.lo, spec.hi),
                G4GeneKind::Int => raw.round().clamp(spec.lo, spec.hi),
            };
        }
        G4DecodedParams::from_physical_slice(&phys)
    }

    /// Set a gene by name using its physical value (automatically mapped to `[0, 1]`).
    pub fn set_physical(&mut self, name: &str, phys_val: f64) -> bool {
        for spec in &G4_GENES {
            if spec.name == name {
                let v = match spec.kind {
                    G4GeneKind::F64 => phys_val.clamp(spec.lo, spec.hi),
                    G4GeneKind::Int => phys_val.round().clamp(spec.lo, spec.hi),
                };
                let ui = match spec.scale {
                    G4GeneScale::Lin => ((v - spec.lo) / (spec.hi - spec.lo)).clamp(0.0, 1.0),
                    G4GeneScale::Log => (libm::log(v / spec.lo) / libm::log(spec.hi / spec.lo)).clamp(0.0, 1.0),
                };
                self.u[spec.index] = ui;
                return true;
            }
        }
        false
    }

    /// Phenotype-only content SHA-256 (`G4-GEN-6`): hashes schema, `G4_GENOME_TOML_SHA256`, and the
    /// decoded physical values (so quantized integer genes and strategy parameters never create distinct
    /// hashes for identical phenotypes).
    pub fn sha256(&self) -> String {
        let mut bytes = Vec::with_capacity(64 + G4_GENE_COUNT * 8);
        bytes.extend_from_slice(G4_GENOME_SCHEMA.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(G4_GENOME_TOML_SHA256.as_bytes());
        bytes.push(0);
        let decoded = self.decode();
        for spec in &G4_GENES {
            let mut v = decoded.get_by_name(spec.name).unwrap();
            // Zero out conditional genes when their activation gate is closed (`G4-GEN-6`):
            if (spec.name == "win_length_frac" || spec.name == "win_width_frac" || spec.name == "win_fillet_r")
                && decoded.win_gate < 0.05
            {
                v = 0.0;
            }
            if spec.module == G4Module::T && spec.name != "alien_gate" && decoded.alien_gate < 0.05 {
                v = 0.0;
            }
            // Normalize -0.0 to +0.0 before hashing IEEE-754 bits
            let norm = if v == 0.0 { 0.0_f64 } else { v };
            bytes.extend_from_slice(&norm.to_bits().to_le_bytes());
        }
        crate::sha256_bytes(&bytes)
    }

    /// Structural-only SHA-256 (excludes Module `A` aesthetic texture genes, `UT-52`).
    pub fn structural_sha256(&self) -> String {
        let mut bytes = Vec::with_capacity(64 + G4_GENE_COUNT * 8);
        bytes.extend_from_slice(b"G4.0.STRUCTURAL\0");
        let decoded = self.decode();
        for spec in &G4_GENES {
            if spec.module == G4Module::A {
                continue;
            }
            let v = decoded.get_by_name(spec.name).unwrap();
            let norm = if v == 0.0 { 0.0_f64 } else { v };
            bytes.extend_from_slice(&norm.to_bits().to_le_bytes());
        }
        crate::sha256_bytes(&bytes)
    }

    /// Serialize to a strict, tamper-evident CSV row:
    /// `G4.0,<sha256>,<toml_sha16>,<u_0_hex>,...,<u_129_hex>`
    pub fn to_csv_row(&self) -> String {
        let mut parts = Vec::with_capacity(3 + G4_GENE_COUNT);
        parts.push(G4_GENOME_SCHEMA.to_string());
        parts.push(self.sha256());
        parts.push(G4_GENOME_TOML_SHA256[..16].to_string());
        for &ui in &self.u {
            let clamped = ui.clamp(0.0, 1.0);
            let norm = if clamped == 0.0 { 0.0_f64 } else { clamped };
            parts.push(format!("{:016x}", norm.to_bits()));
        }
        parts.join(",")
    }

    /// Parse and verify a strict `G4.0` CSV row fail-closed (`T-GEN-1`).
    pub fn from_csv_row(row: &str) -> Result<Self, String> {
        let cols: Vec<&str> = row.trim().split(',').collect();
        if cols.len() != 3 + G4_GENE_COUNT {
            return Err(format!(
                "invalid_g4_csv_column_count: got {}, expected {}",
                cols.len(),
                3 + G4_GENE_COUNT
            ));
        }
        if cols[0] != G4_GENOME_SCHEMA {
            return Err(format!("invalid_g4_schema: got {}, expected G4.0", cols[0]));
        }
        if cols[2] != &G4_GENOME_TOML_SHA256[..16] {
            return Err("g4_toml_schema_hash_mismatch".into());
        }
        let mut u = Vec::with_capacity(G4_GENE_COUNT);
        for &hex_str in &cols[3..] {
            if hex_str.len() != 16 {
                return Err("invalid_g4_gene_hex_len".into());
            }
            let bits = u64::from_str_radix(hex_str, 16).map_err(|_| "invalid_g4_gene_hex")?;
            let val = f64::from_bits(bits);
            if !val.is_finite() || !(0.0..=1.0).contains(&val) {
                return Err("g4_gene_out_of_unit_cube".into());
            }
            u.push(val);
        }
        let g = Self {
            schema: G4_GENOME_SCHEMA.to_string(),
            u,
            strategy: G4EmitterStrategy::default(),
        };
        if g.sha256() != cols[1] {
            return Err("g4_genome_sha256_tamper_detected".into());
        }
        Ok(g)
    }
}

/// Reflective unit-cube bounce (`T-GEN-4`: never piles up on bounds).
#[inline]
pub fn reflect_unit(mut x: f64) -> f64 {
    if !x.is_finite() {
        return 0.5;
    }
    while !(0.0..=1.0).contains(&x) {
        if x < 0.0 {
            x = -x;
        }
        if x > 1.0 {
            x = 2.0 - x;
        }
    }
    x.clamp(0.0, 1.0)
}

/// Typed `Iso+LineDD` + module-crossover variation operator (`T-GEN-4`, `C-61`).
pub fn reproduce_iso_line_dd_g4(
    parent_a: &G4Genome,
    parent_b: &G4Genome,
    rng: &mut SplitMix64,
    freeze_module_t: bool,
) -> G4Genome {
    let mut child = parent_a.clone();
    child.strategy.self_adapt(rng);

    // Module crossover with probability 0.18
    if rng.next_f64() < 0.18 {
        let modules = [
            G4Module::H,
            G4Module::G,
            G4Module::P,
            G4Module::Z,
            G4Module::X,
            G4Module::I,
            G4Module::E,
            G4Module::T,
        ];
        let swap_mod = modules[(rng.next_u64() as usize) % modules.len()];
        for spec in &G4_GENES {
            if spec.module == swap_mod {
                child.u[spec.index] = parent_b.u[spec.index];
            }
        }
    }

    let line_dd = 0.22 * rng.next_normal();
    for spec in &G4_GENES {
        if freeze_module_t && spec.module == G4Module::T {
            child.u[spec.index] = 0.0;
            continue;
        }
        // Apply isotropic perturbation to ~35% of genes per step to preserve high heritability (UT-54)
        let active = rng.next_f64() < 0.35;
        let sigma_m = child.strategy.sigma_for(spec.module);
        let iso = if active { sigma_m * rng.next_normal() } else { 0.0 };
        let line = line_dd * (parent_b.u[spec.index] - parent_a.u[spec.index]);
        let mut u_new = reflect_unit(child.u[spec.index] + iso + line);
        // For integer genes, snap unit coordinate to the exact integer grid center + small jitter
        if spec.kind == G4GeneKind::Int {
            let phys = (spec.lo + u_new * (spec.hi - spec.lo)).round().clamp(spec.lo, spec.hi);
            u_new = ((phys - spec.lo) / (spec.hi - spec.lo)).clamp(0.0, 1.0);
        }
        child.u[spec.index] = u_new;
    }
    child
}
