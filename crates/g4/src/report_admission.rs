//! Scientific report & release-receipt admission gate (`F01`–`F06`, `RI-01`–`RI-15`).
//!
//! Enforces that any release receipt or comparative engineering claim binds to a verified
//! [`ScientificIdentity`], separates numerical GCI execution metadata from convergence admission
//! (`plate_gci_computed` vs `plate_gci_admitted`), rejects vacuous empty gate maps (`F05`),
//! requires per-load-case failure-sequence reporting (`AG-05` / `F06`), and requires explicit
//! disclosure whenever the scored voxel mass differs from the watertight print-mesh mass (`AG-02`).

use jarambit_evidence::identity::{check_id, is_hash, scientific_hash, ContentRef, ScientificIdentity};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

/// Maximum admissible fine-grid Grid Convergence Index (5.0%) for a plate or solid mesh study
/// to be admitted as numerically converged (`Roache 1998`, `Celik et al. 2008`).
pub const MAX_ADMISSIBLE_GCI_3LEVEL: f64 = 0.05;

/// Minimum and maximum admissible observed Richardson convergence order `p`.
pub const MIN_ADMISSIBLE_GCI_ORDER_P: f64 = 0.8;
pub const MAX_ADMISSIBLE_GCI_ORDER_P: f64 = 3.5;

/// Explicit separation of GCI computation status from convergence admission (`F04`, `RI-04`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlateGciAdmission {
    /// Whether a 3-level mesh refinement study was actually executed (`false` in G3 v3.0.0 where 0.5 was a placeholder).
    pub plate_gci_computed: bool,
    /// Whether the 3-level study exhibited monotone convergence (as opposed to oscillatory sign change).
    pub monotone_converged: bool,
    /// Observed Richardson order `p` if monotone, otherwise `None`.
    pub observed_order_p: Option<f64>,
    /// Fine-grid GCI fraction (e.g. `0.024` for 2.4%) if computed, otherwise `None`.
    pub gci_fine_fraction: Option<f64>,
    /// True ONLY if `plate_gci_computed && monotone_converged` and `gci_fine_fraction <= 0.05`
    /// with `observed_order_p` in `[0.8, 3.5]`.
    pub plate_gci_admitted: bool,
}

impl PlateGciAdmission {
    /// Evaluate 3-level mesh quantities `(f_coarse, f_medium, f_fine)` with refinement ratio `r > 1.0`.
    pub fn evaluate_three_level(f_coarse: f64, f_medium: f64, f_fine: f64, r: f64) -> Result<Self, String> {
        if !f_coarse.is_finite() || !f_medium.is_finite() || !f_fine.is_finite() || !r.is_finite() || r <= 1.05 {
            return Err("invalid_gci_inputs".into());
        }
        let e21 = f_medium - f_fine;
        let e32 = f_coarse - f_medium;
        let monotone = (e21 * e32) > 0.0 && e21.abs() > 1e-12 && f_fine.abs() > 1e-12;
        if !monotone {
            let osc_band = (f_coarse.max(f_medium).max(f_fine) - f_coarse.min(f_medium).min(f_fine))
                / f_fine.abs().max(1e-9);
            return Ok(Self {
                plate_gci_computed: true,
                monotone_converged: false,
                observed_order_p: None,
                gci_fine_fraction: Some(1.25 * osc_band),
                plate_gci_admitted: false,
            });
        }
        let p = ((e32 / e21).abs().ln() / r.ln()).clamp(0.1, 8.0);
        let gci_fine = 1.25 * (e21 / f_fine).abs() / (r.powf(p) - 1.0).max(1e-9);
        let admitted = (MIN_ADMISSIBLE_GCI_ORDER_P..=MAX_ADMISSIBLE_GCI_ORDER_P).contains(&p)
            && gci_fine <= MAX_ADMISSIBLE_GCI_3LEVEL;
        Ok(Self {
            plate_gci_computed: true,
            monotone_converged: true,
            observed_order_p: Some(p),
            gci_fine_fraction: Some(gci_fine),
            plate_gci_admitted: admitted,
        })
    }

    /// Construct an uncomputed placeholder record (`F04`: never admitted).
    pub fn uncomputed_placeholder() -> Self {
        Self {
            plate_gci_computed: false,
            monotone_converged: false,
            observed_order_p: None,
            gci_fine_fraction: None,
            plate_gci_admitted: false,
        }
    }

    pub fn check(&self) -> Result<(), String> {
        if !self.plate_gci_computed {
            if self.plate_gci_admitted || self.monotone_converged || self.observed_order_p.is_some() || self.gci_fine_fraction.is_some() {
                return Err("uncomputed_gci_cannot_be_admitted_or_carry_order".into());
            }
            return Ok(());
        }
        let gci = self.gci_fine_fraction.ok_or("computed_gci_missing_fine_fraction")?;
        if !gci.is_finite() || gci < 0.0 {
            return Err("invalid_gci_fine_fraction".into());
        }
        if self.plate_gci_admitted {
            if !self.monotone_converged {
                return Err("oscillatory_gci_cannot_be_admitted".into());
            }
            let p = self.observed_order_p.ok_or("admitted_gci_missing_order_p")?;
            if !(MIN_ADMISSIBLE_GCI_ORDER_P..=MAX_ADMISSIBLE_GCI_ORDER_P).contains(&p) {
                return Err("admitted_gci_order_p_out_of_bounds".into());
            }
            if gci > MAX_ADMISSIBLE_GCI_3LEVEL {
                return Err("admitted_gci_exceeds_5pct_threshold".into());
            }
        }
        Ok(())
    }
}

/// Per-load-case failure sequence record (`AG-05`, `F-M-1`, `F06`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PerCaseSequenceLedger {
    /// Lateral blade load (`260 N`): `SF_hand(lat) / SF_blade(lat)`.
    pub lateral_260n_seq_ratio: f64,
    /// Blade torsion (`1.82 N·m` or `16.2 N·m`): `SF_hand(twist) / SF_blade(twist)`.
    pub twist_seq_ratio: f64,
    /// Spine load (`3,427 N` baton or `500 N` push): `SF_hand(spine) / SF_blade(spine)`.
    pub spine_seq_ratio: f64,
    /// Governing blade-engaging sequence ratio (`min(lateral_260n_seq_ratio, twist_seq_ratio)`).
    pub governing_blade_seq_ratio: f64,
    /// Legacy mixed-load-case index (`min_LC SF_hand(LC) / SF_blade(lat_260n)`), retained for v3 continuity only.
    pub legacy_mixed_seq_index: f64,
}

impl PerCaseSequenceLedger {
    pub fn compute(
        sf_hand_lat: f64,
        sf_blade_lat: f64,
        sf_hand_twist: f64,
        sf_blade_twist: f64,
        sf_hand_spine: f64,
        sf_blade_spine: f64,
    ) -> Result<Self, String> {
        for v in [sf_hand_lat, sf_blade_lat, sf_hand_twist, sf_blade_twist, sf_hand_spine, sf_blade_spine] {
            if !v.is_finite() || v <= 0.0 {
                return Err("nonpositive_or_nonfinite_sf_in_sequence_ledger".into());
            }
        }
        let lateral_260n_seq_ratio = sf_hand_lat / sf_blade_lat;
        let twist_seq_ratio = sf_hand_twist / sf_blade_twist;
        let spine_seq_ratio = sf_hand_spine / sf_blade_spine;
        let governing_blade_seq_ratio = lateral_260n_seq_ratio.min(twist_seq_ratio);
        let grip_min = sf_hand_lat.min(sf_hand_twist).min(sf_hand_spine);
        let legacy_mixed_seq_index = grip_min / sf_blade_lat;
        Ok(Self {
            lateral_260n_seq_ratio,
            twist_seq_ratio,
            spine_seq_ratio,
            governing_blade_seq_ratio,
            legacy_mixed_seq_index,
        })
    }

    pub fn check(&self) -> Result<(), String> {
        for v in [
            self.lateral_260n_seq_ratio,
            self.twist_seq_ratio,
            self.spine_seq_ratio,
            self.governing_blade_seq_ratio,
            self.legacy_mixed_seq_index,
        ] {
            if !v.is_finite() || v <= 0.0 {
                return Err("invalid_sequence_ratio_scalar".into());
            }
        }
        let expected_gov = self.lateral_260n_seq_ratio.min(self.twist_seq_ratio);
        if (self.governing_blade_seq_ratio - expected_gov).abs() > 1e-6 {
            return Err("governing_blade_seq_ratio_mismatch".into());
        }
        Ok(())
    }
}

/// Fail-closed check on gate flag maps (`F05`, `RI-05`): an empty gate map is NEVER all-pass.
pub fn verify_nonempty_gate_flags(gate_flags: &BTreeMap<String, bool>, min_expected_gates: usize) -> Result<bool, String> {
    if gate_flags.is_empty() || gate_flags.len() < min_expected_gates {
        return Err(format!(
            "gate_flags_insufficient: got {}, need at least {}",
            gate_flags.len(),
            min_expected_gates
        ));
    }
    for k in gate_flags.keys() {
        check_id(k)?;
    }
    Ok(gate_flags.values().all(|&passed| passed))
}

/// Release receipt scientific binding (`F01`–`F06`, `RI-01`–`RI-12`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseReceiptEnvelope {
    pub schema_version: u32,
    pub release_version: String,
    pub design_id: String,
    pub scientific_identity: ScientificIdentity,
    pub archive_manifest_ref: ContentRef,
    /// Mass used inside the search/ranking comparator (g).
    pub scored_mass_g: f64,
    /// Watertight print-mesh mass at certification fidelity (g).
    pub print_mesh_mass_g: f64,
    /// Explicit relative discrepancy `|print_mesh_mass_g - scored_mass_g| / print_mesh_mass_g`.
    pub mass_discrepancy_fraction: f64,
    pub gci_admission: PlateGciAdmission,
    pub sequence_ledger: PerCaseSequenceLedger,
    pub hard_gate_flags: BTreeMap<String, bool>,
    pub all_hard_gates_pass: bool,
}

impl ReleaseReceiptEnvelope {
    /// Validate all research-integrity invariants (`RI-01`..`RI-15`) fail-closed.
    pub fn check_admission(&self, min_expected_gates: usize, max_mass_discrepancy_for_feasible: f64) -> Result<String, String> {
        if self.schema_version != 1 {
            return Err("unsupported_release_receipt_schema".into());
        }
        check_id(&self.release_version)?;
        check_id(&self.design_id)?;
        self.scientific_identity.check()?;
        self.archive_manifest_ref.check()?;
        if !is_hash(&self.archive_manifest_ref.sha256) {
            return Err("invalid_archive_manifest_hash".into());
        }
        if !self.scored_mass_g.is_finite() || self.scored_mass_g <= 0.0
            || !self.print_mesh_mass_g.is_finite() || self.print_mesh_mass_g <= 0.0
            || !self.mass_discrepancy_fraction.is_finite() || self.mass_discrepancy_fraction < 0.0
        {
            return Err("invalid_mass_fields".into());
        }
        let expected_disc = (self.print_mesh_mass_g - self.scored_mass_g).abs() / self.print_mesh_mass_g;
        if (self.mass_discrepancy_fraction - expected_disc).abs() > 1e-5 {
            return Err("mass_discrepancy_fraction_mismatch".into());
        }
        self.gci_admission.check()?;
        self.sequence_ledger.check()?;
        let computed_all_pass = verify_nonempty_gate_flags(&self.hard_gate_flags, min_expected_gates)?;
        if self.all_hard_gates_pass != computed_all_pass {
            return Err("all_hard_gates_pass_contradicts_gate_flags".into());
        }
        if self.all_hard_gates_pass {
            if !self.gci_admission.plate_gci_admitted {
                return Err("feasible_release_requires_admitted_plate_gci".into());
            }
            if self.mass_discrepancy_fraction > max_mass_discrepancy_for_feasible {
                return Err("feasible_release_exceeds_mass_discrepancy_limit".into());
            }
        }
        scientific_hash(&Value::from(serde_json::to_value(self).map_err(|e| e.to_string())?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jarambit_evidence::identity::{MaterialState, SubjectRef};

    fn sample_identity() -> ScientificIdentity {
        let h = |ch: char| std::iter::repeat(ch).take(64).collect::<String>();
        ScientificIdentity {
            schema: 2,
            subject: SubjectRef::Genome {
                representation: ContentRef { id: "genome.G4.0".into(), revision: "4.0.0".into(), sha256: h('1') },
                bits: vec!["3fe0000000000000".into(), "3fd0000000000000".into()],
            },
            requirements: vec![ContentRef { id: "RS-karambit-v4".into(), revision: "4.0.0".into(), sha256: h('2') }],
            source: ContentRef { id: "source.tree".into(), revision: "4.0.0".into(), sha256: h('3') },
            build: ContentRef { id: "build.profile".into(), revision: "4.0.0".into(), sha256: h('4') },
            cargo_lock: ContentRef { id: "cargo.lock".into(), revision: "4.0.0".into(), sha256: h('5') },
            config: ContentRef { id: "search.config".into(), revision: "4.0.0".into(), sha256: h('6') },
            evaluator: ContentRef { id: "g4.evaluator".into(), revision: "4.0.0".into(), sha256: h('7') },
            scenario: ContentRef { id: "loads.v4".into(), revision: "4.0.0".into(), sha256: h('8') },
            objective: ContentRef { id: "W_ERGOLIGHT_v2".into(), revision: "4.0.0".into(), sha256: h('9') },
            descriptor: ContentRef { id: "cvt.6d.v4".into(), revision: "4.0.0".into(), sha256: h('a') },
            material: MaterialState::Identified {
                passport: ContentRef { id: "mat.ti64.v4".into(), revision: "4.0.0".into(), sha256: h('b') },
            },
            dependencies: vec![],
        }
    }

    #[test]
    fn ri04_uncomputed_or_oscillatory_gci_never_admitted() {
        let uncomputed = PlateGciAdmission::uncomputed_placeholder();
        assert!(uncomputed.check().is_ok());
        assert!(!uncomputed.plate_gci_admitted);

        let mut forged = uncomputed.clone();
        forged.plate_gci_admitted = true;
        assert!(forged.check().is_err());

        let osc = PlateGciAdmission::evaluate_three_level(0.585, 0.638, 0.611, 1.41421356).unwrap();
        assert!(osc.plate_gci_computed);
        assert!(!osc.monotone_converged);
        assert!(!osc.plate_gci_admitted);

        let mono = PlateGciAdmission::evaluate_three_level(345.0, 339.0, 337.0, 2.0_f64.sqrt()).unwrap();
        assert!(mono.plate_gci_computed);
        assert!(mono.monotone_converged);
        assert!(mono.plate_gci_admitted);
        assert!(mono.gci_fine_fraction.unwrap() <= MAX_ADMISSIBLE_GCI_3LEVEL);
    }

    #[test]
    fn ri05_empty_gate_flags_rejected_fail_closed() {
        let empty = BTreeMap::new();
        assert!(verify_nonempty_gate_flags(&empty, 1).is_err());

        let mut flags = BTreeMap::new();
        flags.insert("solid_tip_sf".into(), true);
        flags.insert("grip_sf_min".into(), false);
        assert_eq!(verify_nonempty_gate_flags(&flags, 2).unwrap(), false);
    }

    #[test]
    fn ri06_per_load_case_sequence_ledger_separates_lateral_from_mixed_spine() {
        let ledger = PerCaseSequenceLedger::compute(3.55, 337.4 / 260.0, 5.50, 1.087, 0.726, 1.438).unwrap();
        assert!((ledger.lateral_260n_seq_ratio - 2.7356).abs() < 1e-3);
        assert!((ledger.legacy_mixed_seq_index - 0.5595).abs() < 1e-3);
        assert!(ledger.governing_blade_seq_ratio > 2.5);
        assert!(ledger.check().is_ok());
    }

    #[test]
    fn ri01_to_ri12_release_receipt_envelope_admission() {
        let gci = PlateGciAdmission::evaluate_three_level(345.0, 339.0, 337.0, 2.0_f64.sqrt()).unwrap();
        let seq = PerCaseSequenceLedger::compute(2.40, 1.50, 2.80, 1.60, 1.85, 1.40).unwrap();
        let mut gates = BTreeMap::new();
        gates.insert("solid_tip_sf".into(), true);
        gates.insert("grip_sf_min".into(), true);
        gates.insert("seq_ratio".into(), true);
        gates.insert("fat_sf".into(), true);
        gates.insert("drainability".into(), true);

        let scored_mass: f64 = 47.80;
        let mesh_mass: f64 = 48.00;
        let disc = (mesh_mass - scored_mass).abs() / mesh_mass;
        let receipt = ReleaseReceiptEnvelope {
            schema_version: 1,
            release_version: "4.0.0".into(),
            design_id: "g4i001_n0042_a1b2c3d4".into(),
            scientific_identity: sample_identity(),
            archive_manifest_ref: ContentRef {
                id: "archive.manifest.v4".into(),
                revision: "4.0.0".into(),
                sha256: std::iter::repeat('c').take(64).collect(),
            },
            scored_mass_g: scored_mass,
            print_mesh_mass_g: mesh_mass,
            mass_discrepancy_fraction: disc,
            gci_admission: gci,
            sequence_ledger: seq,
            hard_gate_flags: gates,
            all_hard_gates_pass: true,
        };
        let digest = receipt.check_admission(5, 0.02).expect("valid receipt admitted");
        assert!(is_hash(&digest));

        let mut bad_gci = receipt.clone();
        bad_gci.gci_admission = PlateGciAdmission::uncomputed_placeholder();
        assert!(bad_gci.check_admission(5, 0.02).is_err());

        let mut bad_mass = receipt;
        bad_mass.scored_mass_g = 57.0;
        bad_mass.print_mesh_mass_g = 59.3;
        bad_mass.mass_discrepancy_fraction = (59.3 - 57.0) / 59.3;
        assert!(bad_mass.check_admission(5, 0.02).is_err());
    }
}
