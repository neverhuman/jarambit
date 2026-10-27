//! Standalone Jarambit G4 (`v4.0.0`) Engine (`crates/g4`):
//! - `genome`: Declarative 130-gene `G4.0` genome (`genome/G4.0.toml`), `#[repr(C)] G4DecodedParams`
//!   (zero string-keyed `BTreeMap` lookups in hot loops), phenotype-only `genome_sha256` (`G4-GEN-6`),
//!   compile-time `g4_decoder_sha256` cached via `OnceLock` (`H-13`), and scrambled Joe-Kuo Sobol init.
//! - `decoder`: Curvilinear 3D Bernstein spine + Bishop frame (`Module P`, `Module Z`), decoupled point
//!   planform angle (`Gate S4/S5`), $C^2$ quartic smooth booleans (`smin_c2`), flange-graded grip skin
//!   (`L-1`), perforated/arched web (`L-2`), drained hollow ring (`L-4`), local tip-bisector crown (`L-8`),
//!   Module `T` trabecular/CPPN overlay inside keep-outs, zero-mean Module `A` texture (`L-7`), sparse
//!   $8^3$ narrow-band bricks (`H-2`), sub-voxel smoothed-Heaviside volume & zone mass (`AG-02`),
//!   erosion drainability (`AG-01`), and dual `as_printed` / `as_finished` geometry (`AG-13`).
//! - `physics`: Unified `HM-1` Winkler hand contact equilibrium (`AG-09`), hole-aware shear-deformable
//!   sandwich plate with 3-level Roache GCI (`AG-10`), exact-law TET10 solid tip (`LC-TIP-1/2`, `AG-07`),
//!   2D `T6` serration tooth FE on all teeth (`AG-12`), locking-free B-bar hex8 voxel FE with Galerkin
//!   GMG-PCG & SPR stress recovery (`AG-11`), and unified L-PBF Ti-6Al-4V HIP materials & fatigue (`AG-08`).
//! - `objective`: `RS-karambit-v4.toml` loader (`UT-31`), uncertainty-bounded safety factors (`LCB`),
//!   Feasibility-First Lexicographic Comparator (`W_ERGOLIGHT_v2`, `SRCH-1`), and legacy `W_ERGOLIGHT_v1`.
//! - `search`: Constrained Two-Layer (`FI-2Pop`) Lloyd-CVT Archive over 6 physical descriptors,
//!   multi-emitter UCB1 bandit (`Sep-CMA-MAE`, subspace constraint-repair / Lamarckian lattice grading,
//!   feasible mass-descent, `Iso+LineDD`), calibrated UCB promotion (`SRCH-5`), periodic $h = 0.20\text{ mm}$
//!   re-certification (`SRCH-7`), and `libm` cross-architecture bit-identical execution (`T-DET-1`).

pub mod decoder;
pub mod genome;
pub mod objective;
pub mod physics;
pub mod report_admission;
pub mod search;

pub use jarambit_g3::sha256_bytes;
