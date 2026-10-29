//! The hand-zone load cases G5.1 scores differently from G5.0: grip squeeze, guard push and the serration teeth.
//!
//! Every evaluation records, per case, G5.0's analytic safety factor beside the one G5.1 scores, and which model
//! produced it. At Tiers 0 and 1 the block is a function of the genome alone, so an archive row's block is recomputed
//! and checked bit for bit when the row is read back.

use jarambit_g5::g5_qd::serration_tooth_strength;
use jarambit_g5::g5_sdf::BladeProfile2D;
use jarambit_g5::loads::{guard_push_sf, G5Loads};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

type Params = BTreeMap<&'static str, f64>;

/// Which model produced the safety factors a record scores.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandModel {
    /// G5.0's analytic models, unchanged: development builds only, before the calibration exists. A frozen G5.1
    /// refuses it.
    G50Analytic,
}

/// One case: G5.0's analytic SF and the SF G5.1 scores.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HandCase {
    #[serde(with = "crate::f64x")]
    pub g50: f64,
    #[serde(with = "crate::f64x")]
    pub scored: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HandZone {
    pub model: HandModel,
    pub grip_squeeze: HandCase,
    pub guard_push: HandCase,
    pub teeth: HandCase,
}

impl HandZone {
    /// Write the scored SFs into a record's load cases.
    pub fn apply(&self, loads: &mut G5Loads) {
        loads.grip_squeeze_sf = self.grip_squeeze.scored;
        loads.guard_push_sf = self.guard_push.scored;
        loads.teeth_sf = self.teeth.scored;
    }
}

/// G5.0's three analytic SFs, exactly as its Tier 0 computes them (`analytic_loads`, `guard_push_sf`, the minimum of
/// `serration_tooth_strength` over both edges).
pub fn g50_analytic(p: &Params, blade: &BladeProfile2D, loads_g50: &G5Loads) -> (f64, f64, f64) {
    let teeth = [true, false].iter().map(|&inner| serration_tooth_strength(p, blade, inner).0).fold(f64::INFINITY, f64::min);
    debug_assert_eq!(guard_push_sf(p).to_bits(), loads_g50.guard_push_sf.to_bits());
    (loads_g50.grip_squeeze_sf, loads_g50.guard_push_sf, teeth)
}

/// The Tier-0/1 hand-zone block of a design.
pub fn hand_zone_analytic(p: &Params, blade: &BladeProfile2D, loads_g50: &G5Loads) -> HandZone {
    let (grip, guard, teeth) = g50_analytic(p, blade, loads_g50);
    let case = |g50: f64| HandCase { g50, scored: g50 };
    HandZone { model: HandModel::G50Analytic, grip_squeeze: case(grip), guard_push: case(guard), teeth: case(teeth) }
}
