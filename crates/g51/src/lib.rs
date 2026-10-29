//! Jarambit G5.1 (`jarambit-g51`): the decoder generation of v7.0.0.
//!
//! G5.1 is G5.0 (`crates/g5`, frozen and unchanged; a path dependency) with the changes the v6.0.0 paper left to a
//! later decoder: surface texture kept off the ground bevels by construction (`sdf`). The genome, RS-karambit-v5 and
//! W_ERGOLIGHT_v2 are G5.0's. Only what changes lives here; everything else is called from G5.0, so its decoder of
//! record stays verifiable on main (`tests::guards`).
//!
//! This file holds only the module list and the test modules; it is outside the decoder hash (`hash`), as G5.0's
//! `lib.rs` is outside G5.0's.

pub mod f64x;
pub mod fe;
pub mod hand;
pub mod hash;
pub mod namespace;
pub mod sdf;
pub mod tiers;

#[cfg(test)]
mod tests;
