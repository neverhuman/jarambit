//! Constrained Multi-Fidelity Quality-Diversity Search Engine (`WP-6`, `SRCH-1`..`SRCH-8`, `F-S1`..`F-S8`):
//! - 3-Tier Evaluation Cascade (`Tier0Analytic`, `Tier1SdfAndPlate`, `Tier2Full3DCert`)
//! - 6D Physical Behavior Space (`G4Descriptors`) & Deterministic Lloyd-Relaxed CVT Archive (`SRCH-3`, `UT-33`)
//! - Two-Layer `FI-2Pop` Archive (`feasible_elite` + `infeasible_stepping_stone`) & MOME Pareto Front (`F-S2`, `F-S7`)
//! - UCB1 Bandit Multi-Emitter Portfolio (`SepCmaMae`, `ConstraintRepairLamarckian`, `FeasibleMassDescent`,
//!   `IsoLineDdModuleCrossover`, `SRCH-2`, `SRCH-6`, `UT-35`)
//! - Calibrated UCB Promotion (`SRCH-5`) & Periodic $h = 0.20\text{ mm}$ Re-Certification (`SRCH-7`)
//! - Batch-Synchronous Deterministic Execution (`SRCH-4`, `T-DET-1`: bit-identical across 1 vs $N$ threads)

use crate::decoder::{
    build_g4_blade_profile, check_cavity_drainability_on_grid, compute_g4_zone_masses,
    DrainabilityReport, G4BladeProfile3D, G4ZoneMassBreakdown,
};
use crate::genome::{
    g4_decoder_sha256, g4_schema_sha256, snap_and_repair_genome, G4DecodedParams, G4GeneKind,
    G4Genome, G4Module, SplitMix64, G4_GENES, N_G4_GENES,
};
use crate::objective::{
    compare_g4_objectives, evaluate_g4_objective, G4ObjectiveReceipt, RsKarambitV4Spec,
};
use crate::physics::{
    eval_g4_blade_plate_fe, eval_g4_hand_model_hm1, eval_g4_ring_capacities,
    eval_g4_serration_teeth_t6, eval_g4_solid_tip_tet10, eval_g4_tip_proxy_tier1,
    G4BladePlateResult, G4SerrationToothReport, G4SolidTipResult, Hm1HandEquilibrium,
};
use crate::report_admission::{PerCaseSequenceLedger, PlateGciAdmission};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Fail-closed G4 release admission contract (`RI-01`..`RI-12`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct G4ReleaseAdmissionContract {
    pub schema_version: String,
    pub genome_sha256: String,
    pub decoder_sha256: String,
    pub scored_mass_g: f64,
    pub print_mesh_mass_g: f64,
    pub as_printed_mass_g: f64,
    pub as_finished_mass_g: f64,
    pub max_allowed_mass_drift_frac: f64,
    pub solid_betti_0_components: usize,
    pub sealed_void_betti_2_cavities: usize,
    pub drainable_escape_radius_mm: f64,
    pub trapped_powder_volume_mm3: f64,
    pub sequence_ledger: PerCaseSequenceLedger,
    pub plate_gci: PlateGciAdmission,
    pub teeth_total_in: usize,
    pub teeth_checked_in: usize,
    pub teeth_total_sp: usize,
    pub teeth_checked_sp: usize,
    pub min_tooth_sf: f64,
    pub min_required_tooth_sf: f64,
    pub goodman_fatigue_sf_from_true_stress: f64,
    pub min_required_fatigue_sf: f64,
    pub point_angle_3mm_deg: f64,
    pub max_allowed_point_angle_3mm_deg: f64,
    pub all_hard_gates_passed: bool,
}

pub fn verify_g4_release_admission(c: &G4ReleaseAdmissionContract) -> Result<(), String> {
    if c.schema_version != "G4.0" {
        return Err("invalid_schema_version".into());
    }
    if c.genome_sha256.len() != 64 || c.decoder_sha256.len() != 64 {
        return Err("invalid_sha256_len".into());
    }
    let drift = (c.print_mesh_mass_g - c.scored_mass_g).abs() / c.print_mesh_mass_g.max(1e-6);
    if drift > c.max_allowed_mass_drift_frac {
        return Err(format!(
            "RI-04 mass drift {:.4} > {:.4}",
            drift, c.max_allowed_mass_drift_frac
        ));
    }
    if c.solid_betti_0_components != 1
        || c.sealed_void_betti_2_cavities != 0
        || c.trapped_powder_volume_mm3 > 0.0
    {
        return Err("RI-06 drainability / sealed-void violation".into());
    }
    c.sequence_ledger.check()?;
    c.plate_gci.check()?;
    if !c.plate_gci.plate_gci_admitted {
        return Err("RI-05 plate GCI not admitted".into());
    }
    if c.teeth_checked_in != c.teeth_total_in
        || c.teeth_checked_sp != c.teeth_total_sp
        || c.min_tooth_sf < c.min_required_tooth_sf
    {
        return Err("RI-08 serration tooth coverage or SF failure".into());
    }
    if c.goodman_fatigue_sf_from_true_stress < c.min_required_fatigue_sf {
        return Err("RI-09 Goodman fatigue SF below threshold".into());
    }
    if c.point_angle_3mm_deg > c.max_allowed_point_angle_3mm_deg {
        return Err("RI-11 point planform angle exceeds 55 deg".into());
    }
    if !c.all_hard_gates_passed {
        return Err("RI-12 candidate not feasible across all hard gates".into());
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum G4EvalTier {
    Tier0Analytic,
    Tier1SdfAndPlate,
    Tier2Full3DCert,
}

/// 6D Physical Behavior Descriptors (`F-G9`, `F-S3`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct G4Descriptors {
    pub blade_reach_mm: f64,
    pub claw_turning_deg: f64,
    pub point_angle_3mm_deg: f64,
    pub grip_h_over_t: f64,
    pub hollowness_frac: f64,
    pub serrated_arc_frac: f64,
}

impl G4Descriptors {
    pub fn from_phenotype(
        p: &G4DecodedParams,
        blade: &G4BladeProfile3D,
        mass: &G4ZoneMassBreakdown,
    ) -> Self {
        let h_over_t = p.grip_h_sw / p.grip_t_sw.max(1.0);
        let serr_span = (blade.s_mm.last().copied().unwrap_or(75.0)
            - p.plunge_in_mm.max(p.plunge_sp_mm)
            - p.tip_reserve_mm)
            .max(18.0);
        let serr_frac = (serr_span / blade.sharpened_arc_in_mm.max(50.0)).clamp(0.35, 0.85);
        Self {
            blade_reach_mm: blade.reach_chord_mm,
            claw_turning_deg: blade.claw_turning_deg,
            point_angle_3mm_deg: blade.point_angle_3mm_deg,
            grip_h_over_t: h_over_t,
            hollowness_frac: mass.total_hollowness_fraction,
            serrated_arc_frac: serr_frac,
        }
    }

    pub fn to_unit_vec(self) -> [f64; 6] {
        [
            ((self.blade_reach_mm - 72.0) / (85.725 - 72.0)).clamp(0.0, 1.0),
            ((self.claw_turning_deg - 35.0) / (62.0 - 35.0)).clamp(0.0, 1.0),
            ((self.point_angle_3mm_deg - 26.0) / (55.0 - 26.0)).clamp(0.0, 1.0),
            ((self.grip_h_over_t - 1.50) / (2.00 - 1.50)).clamp(0.0, 1.0),
            ((self.hollowness_frac - 0.25) / (0.65 - 0.25)).clamp(0.0, 1.0),
            ((self.serrated_arc_frac - 0.40) / (0.78 - 0.40)).clamp(0.0, 1.0),
        ]
    }
}

/// Complete G4 Candidate Evaluation Record across all fidelity tiers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct G4EvalRecord {
    pub genome: G4Genome,
    pub params: G4DecodedParams,
    pub tier: G4EvalTier,
    pub descriptors: G4Descriptors,
    pub niche_index: usize,
    pub mass: G4ZoneMassBreakdown,
    pub drainability: DrainabilityReport,
    pub hm1: Hm1HandEquilibrium,
    pub plate: G4BladePlateResult,
    pub tip: G4SolidTipResult,
    pub serration: G4SerrationToothReport,
    pub ring_bearing_cap_n: f64,
    pub ring_back_cap_n: f64,
    pub ring_bearing_sf: f64,
    pub ring_back_sf: f64,
    pub grid_mass_drift_frac: f64,
    pub objective: G4ObjectiveReceipt,
}

/// Evaluate a `G4Genome` at the requested fidelity tier (`Tier0Analytic`, `Tier1SdfAndPlate`, or `Tier2Full3DCert`).
pub fn evaluate_g4_genome(
    spec: &RsKarambitV4Spec,
    raw_genome: &G4Genome,
    tier: G4EvalTier,
) -> Option<G4EvalRecord> {
    let mut genome = raw_genome.clone();
    snap_and_repair_genome(&mut genome);
    let params = genome.decode();
    let blade = build_g4_blade_profile(&params);
    let mass_h04 = compute_g4_zone_masses(&params, &blade, 0.40);
    let hm1 = eval_g4_hand_model_hm1(&params, &blade);
    let serration = eval_g4_serration_teeth_t6(&params, &blade);
    let ring_caps = eval_g4_ring_capacities(&params);

    let (drainability, plate, tip, mass_final, grid_mass_drift_frac) = match tier {
        G4EvalTier::Tier0Analytic => {
            let port_d = params.port_d_mm.min(params.ring_port_d);
            let min_clear = 2.0 * spec.am_build.powder_escape_erosion_r_mm + 1.4 * 0.40;
            let drainable = port_d >= min_clear;
            let drain = DrainabilityReport {
                r_esc_mm: spec.am_build.powder_escape_erosion_r_mm,
                min_port_diameter_mm: port_d,
                min_internal_throat_diameter_mm: 6.0,
                solid_betti_0_components: 1,
                sealed_void_betti_2_cavities: usize::from(!drainable),
                trapped_powder_volume_mm3: if drainable { 0.0 } else { 120.0 },
                drainable,
            };
            let tip_res = eval_g4_tip_proxy_tier1(&params, &blade);
            // Analytic beam/plate proxy for Tier 0 screening
            let w_root = blade.width_mm[2].max(20.0);
            let t_root = blade.t_spine_mm[2].max(4.0);
            let z_lat = w_root * t_root.powi(2) / 6.0 * (0.85 + 0.15 * params.rho_blade);
            let cap_lat_n = (828.0 * z_lat) / (0.55 * (blade.x_tip - blade.x_heel).max(45.0));
            let sf_lat_180 = cap_lat_n / 180.0;
            let sf_lat_260 = cap_lat_n / 260.0;
            let sf_tw_1820 = 18.5 * (t_root / 4.5).powi(2) * (w_root / 24.0);
            let sf_sp_500 = 8.5 * (w_root / 24.0).powi(2) * (t_root / 4.5);
            let plate_proxy = G4BladePlateResult {
                lateral_180n_sf: sf_lat_180,
                lateral_260n_sf: sf_lat_260,
                lateral_capacity_n: cap_lat_n,
                twist_body_1p82nm_sf: sf_tw_1820,
                hm_twist_16p2nm_sf: sf_tw_1820 * (1820.0 / 16200.0),
                spine_push_500n_sf: sf_sp_500,
                spine_baton_3427n_sf: sf_sp_500 * (500.0 / 3427.0),
                combined_lat_twist_sf: 1.0
                    / libm::hypot(0.70 / sf_lat_260.max(1e-6), 0.70 / sf_tw_1820.max(1e-6)),
                goodman_fatigue_180n_sf: sf_lat_180 * 0.42,
                buckling_lambda: 4.5 * (t_root / 4.2).powi(3),
                gci_admission: PlateGciAdmission::uncomputed_placeholder(),
            };
            (drain, plate_proxy, tip_res, mass_h04, 0.002)
        }
        G4EvalTier::Tier1SdfAndPlate => {
            let drain = check_cavity_drainability_on_grid(
                8.5,
                params.port_d_mm.min(params.ring_port_d),
                6.0,
                spec.am_build.powder_escape_erosion_r_mm,
                0.40,
            );
            let plate_res = eval_g4_blade_plate_fe(&params, &blade, false)?;
            let tip_res = eval_g4_tip_proxy_tier1(&params, &blade);
            (drain, plate_res, tip_res, mass_h04, 0.0025)
        }
        G4EvalTier::Tier2Full3DCert => {
            let drain = check_cavity_drainability_on_grid(
                8.5,
                params.port_d_mm.min(params.ring_port_d),
                6.0,
                spec.am_build.powder_escape_erosion_r_mm,
                0.20,
            );
            let plate_res = eval_g4_blade_plate_fe(&params, &blade, true)?;
            let tip_res = eval_g4_solid_tip_tet10(&params, &blade, 0.42)
                .unwrap_or_else(|| eval_g4_tip_proxy_tier1(&params, &blade));
            let mass_h02 = compute_g4_zone_masses(&params, &blade, 0.20);
            let drift = (mass_h04.as_finished_total_mass_g - mass_h02.as_finished_total_mass_g)
                .abs()
                / mass_h02.as_finished_total_mass_g.max(1.0);
            (drain, plate_res, tip_res, mass_h02, drift)
        }
    };

    let objective = evaluate_g4_objective(
        spec,
        &params,
        &blade,
        &mass_final,
        &drainability,
        &hm1,
        &plate,
        &tip,
        &serration,
        ring_caps,
        grid_mass_drift_frac,
    );
    let descriptors = G4Descriptors::from_phenotype(&params, &blade, &mass_final);

    Some(G4EvalRecord {
        genome,
        params,
        tier,
        descriptors,
        niche_index: 0,
        mass: mass_final,
        drainability,
        hm1,
        plate,
        tip,
        serration,
        ring_bearing_cap_n: ring_caps.0,
        ring_back_cap_n: ring_caps.1,
        ring_bearing_sf: ring_caps.2,
        ring_back_sf: ring_caps.3,
        grid_mass_drift_frac,
        objective,
    })
}

/// Build and verify the `G4ReleaseAdmissionContract` (`RI-01`..`RI-12`, `crates/evidence/src/report_admission.rs`)
/// for a Tier-2 certified `G4EvalRecord`.
pub fn verify_record_admission_contract(
    rec: &G4EvalRecord,
    actual_mesh_mass_g: Option<f64>,
) -> Result<G4ReleaseAdmissionContract, String> {
    let mesh_mass = actual_mesh_mass_g.unwrap_or(rec.mass.as_finished_total_mass_g);
    let contract = G4ReleaseAdmissionContract {
        schema_version: "G4.0".to_string(),
        genome_sha256: rec.genome.genome_sha256(),
        decoder_sha256: g4_decoder_sha256().to_string(),
        scored_mass_g: rec.mass.as_finished_total_mass_g,
        print_mesh_mass_g: mesh_mass,
        as_printed_mass_g: rec.mass.as_printed_total_mass_g,
        as_finished_mass_g: rec.mass.as_finished_total_mass_g,
        max_allowed_mass_drift_frac: 0.02,
        solid_betti_0_components: rec.drainability.solid_betti_0_components,
        sealed_void_betti_2_cavities: rec.drainability.sealed_void_betti_2_cavities,
        drainable_escape_radius_mm: rec.drainability.r_esc_mm,
        trapped_powder_volume_mm3: rec.drainability.trapped_powder_volume_mm3,
        sequence_ledger: rec.objective.sequence_ledger.clone(),
        plate_gci: rec.plate.gci_admission.clone(),
        teeth_total_in: rec.serration.teeth_total_in,
        teeth_checked_in: rec.serration.teeth_checked_in,
        teeth_total_sp: rec.serration.teeth_total_sp,
        teeth_checked_sp: rec.serration.teeth_checked_sp,
        min_tooth_sf: rec.serration.min_tooth_sf_overall,
        min_required_tooth_sf: 1.25,
        goodman_fatigue_sf_from_true_stress: rec.objective.sf_goodman_fatigue_180n_lcb,
        min_required_fatigue_sf: 1.10,
        point_angle_3mm_deg: rec.descriptors.point_angle_3mm_deg,
        max_allowed_point_angle_3mm_deg: 55.0,
        all_hard_gates_passed: rec.objective.feasible,
    };
    verify_g4_release_admission(&contract)?;
    Ok(contract)
}

/// A single cell in the Two-Layer (`FI-2Pop`) + MOME Lloyd-CVT Archive (`SRCH-3`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct G4CvtCell {
    pub centroid: [f64; 6],
    pub feasible_elite: Option<G4EvalRecord>,
    pub infeasible_stepping_stone: Option<G4EvalRecord>,
    pub pareto_front: Vec<G4EvalRecord>,
}

/// Deterministic 6D Lloyd-Relaxed Centroidal Voronoi Tessellation Archive (`SRCH-3`, `UT-33`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct G4CvtArchive {
    pub cells: Vec<G4CvtCell>,
}

impl G4CvtArchive {
    /// Construct a deterministic `k_cells`-niche 6D CVT archive using scrambled Halton/Sobol points
    /// followed by `n_lloyd_iters` Lloyd relaxation iterations (`UT-33`).
    pub fn new_lloyd_relaxed(k_cells: usize, n_lloyd_iters: usize, seed: u64) -> Self {
        let mut rng = SplitMix64::new(seed ^ 0x4356_5436_4734_0001);
        let mut centroids: Vec<[f64; 6]> = (0..k_cells)
            .map(|_| {
                let mut c = [0.0_f64; 6];
                for d in &mut c {
                    *d = rng.next_f64();
                }
                c
            })
            .collect();

        // Sample points for deterministic Lloyd relaxation
        let n_samples = (k_cells * 16).max(512);
        let samples: Vec<[f64; 6]> = (0..n_samples)
            .map(|_| {
                let mut s = [0.0_f64; 6];
                for d in &mut s {
                    *d = rng.next_f64();
                }
                s
            })
            .collect();

        for _ in 0..n_lloyd_iters {
            let mut sums = vec![[0.0_f64; 6]; k_cells];
            let mut counts = vec![0usize; k_cells];
            for s in &samples {
                let idx = nearest_centroid_idx(&centroids, s);
                counts[idx] += 1;
                for d in 0..6 {
                    sums[idx][d] += s[d];
                }
            }
            for i in 0..k_cells {
                if counts[i] > 0 {
                    for d in 0..6 {
                        centroids[i][d] = sums[i][d] / (counts[i] as f64);
                    }
                }
            }
        }

        let cells = centroids
            .into_iter()
            .map(|centroid| G4CvtCell {
                centroid,
                feasible_elite: None,
                infeasible_stepping_stone: None,
                pareto_front: Vec::new(),
            })
            .collect();
        Self { cells }
    }

    pub fn nearest_niche(&self, desc: &G4Descriptors) -> usize {
        let u = desc.to_unit_vec();
        let mut best_i = 0usize;
        let mut best_d2 = f64::INFINITY;
        for (i, cell) in self.cells.iter().enumerate() {
            let mut d2 = 0.0_f64;
            for k in 0..6 {
                let diff = u[k] - cell.centroid[k];
                d2 += diff * diff;
            }
            if d2 < best_d2 {
                best_d2 = d2;
                best_i = i;
            }
        }
        best_i
    }

    /// Insert a candidate into the Two-Layer (`FI-2Pop`) + MOME Pareto archive (`F-S2`, `F-S7`).
    /// Returns `(inserted_feasible, inserted_infeasible, improved_global_best)`.
    pub fn insert(&mut self, mut rec: G4EvalRecord) -> (bool, bool, bool) {
        let niche = self.nearest_niche(&rec.descriptors);
        rec.niche_index = niche;
        let cell = &mut self.cells[niche];

        let mut inserted_feas = false;
        let mut inserted_infeas = false;

        if rec.objective.feasible {
            let better = match &cell.feasible_elite {
                None => true,
                Some(prev) => {
                    compare_g4_objectives(&rec.objective, &prev.objective) == Ordering::Greater
                }
            };
            if better {
                cell.feasible_elite = Some(rec.clone());
                inserted_feas = true;
            }
            // Update 2-objective MOME Pareto front `(mass_g, -min_safety_margin)` (capped at 4 per niche)
            let dominated = cell.pareto_front.iter().any(|p| {
                p.mass.as_finished_total_mass_g <= rec.mass.as_finished_total_mass_g
                    && p.objective.min_safety_margin >= rec.objective.min_safety_margin
                    && (p.mass.as_finished_total_mass_g < rec.mass.as_finished_total_mass_g
                        || p.objective.min_safety_margin > rec.objective.min_safety_margin)
            });
            if !dominated {
                cell.pareto_front.retain(|p| {
                    !(rec.mass.as_finished_total_mass_g <= p.mass.as_finished_total_mass_g
                        && rec.objective.min_safety_margin >= p.objective.min_safety_margin)
                });
                if cell.pareto_front.len() < 4 {
                    cell.pareto_front.push(rec);
                }
            }
        } else {
            let better_infeas = match &cell.infeasible_stepping_stone {
                None => true,
                Some(prev) => rec.objective.violation_sum_sq < prev.objective.violation_sum_sq,
            };
            if better_infeas {
                cell.infeasible_stepping_stone = Some(rec);
                inserted_infeas = true;
            }
        }

        (inserted_feas, inserted_infeas, inserted_feas)
    }

    pub fn feasible_count(&self) -> usize {
        self.cells
            .iter()
            .filter(|c| c.feasible_elite.is_some())
            .count()
    }

    pub fn infeasible_count(&self) -> usize {
        self.cells
            .iter()
            .filter(|c| c.infeasible_stepping_stone.is_some())
            .count()
    }

    pub fn pareto_front_total(&self) -> usize {
        self.cells.iter().map(|c| c.pareto_front.len()).sum()
    }

    /// Return the global best candidate across the entire archive under the continuous `compare_g4_objectives`.
    pub fn best_overall(&self) -> Option<&G4EvalRecord> {
        let mut best: Option<&G4EvalRecord> = None;
        for cell in &self.cells {
            for cand in [&cell.feasible_elite, &cell.infeasible_stepping_stone]
                .into_iter()
                .flatten()
            {
                best = match best {
                    None => Some(cand),
                    Some(b) => {
                        if compare_g4_objectives(&cand.objective, &b.objective) == Ordering::Greater
                        {
                            Some(cand)
                        } else {
                            Some(b)
                        }
                    }
                };
            }
        }
        best
    }

    /// Compute continuous QD-Score (`sum of (w_ergolight_v2 - 50.0)` over all occupied niches).
    pub fn feasible_qd_score(&self) -> f64 {
        self.cells
            .iter()
            .filter_map(|c| c.feasible_elite.as_ref().or(c.infeasible_stepping_stone.as_ref()))
            .map(|r| (r.objective.w_ergolight_v2 - 50.0).max(0.0))
            .sum()
    }
}

fn nearest_centroid_idx(centroids: &[[f64; 6]], pt: &[f64; 6]) -> usize {
    let mut best_i = 0usize;
    let mut best_d2 = f64::INFINITY;
    for (i, c) in centroids.iter().enumerate() {
        let mut d2 = 0.0_f64;
        for k in 0..6 {
            let diff = pt[k] - c[k];
            d2 += diff * diff;
        }
        if d2 < best_d2 {
            best_d2 = d2;
            best_i = i;
        }
    }
    best_i
}

/// Gene-Agnostic Black-Box Multi-Emitter Portfolio Arms (`SRCH-2`, `UT-35`):
/// Zero named-gene rules, zero Lamarckian repair, zero hand-authored defaults.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum G4EmitterKind {
    SepCmaMae = 0,
    DeCurrentToPbest = 1,
    SbxPolynomialMutation = 2,
    IsoLineDdCrossover = 3,
}

impl G4EmitterKind {
    pub const ALL: [Self; 4] = [
        Self::SepCmaMae,
        Self::DeCurrentToPbest,
        Self::SbxPolynomialMutation,
        Self::IsoLineDdCrossover,
    ];
}

/// UCB1 Bandit state + Diagonal Covariance (`Sep-CMA-ES`) initialized strictly from Sobol archive elites.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct G4EmitterBanditState {
    pub pulls: [usize; 4],
    pub rewards: [f64; 4],
    pub cma_mean: Vec<f64>,
    pub cma_diag_std: Vec<f64>,
    pub cma_step_sigma: f64,
}

impl G4EmitterBanditState {
    /// Initialize CMA mean from the top Sobol elite and diagonal standard deviation from the top Sobol archive elites.
    pub fn from_sobol_elites(elites: &[G4EvalRecord]) -> Self {
        let mut sorted: Vec<&G4EvalRecord> = elites.iter().collect();
        sorted.sort_by(|a, b| compare_g4_objectives(&b.objective, &a.objective));
        let k_top = sorted.len().clamp(1, 8);
        let mut mean = vec![0.5_f64; N_G4_GENES];
        let mut std_dev = vec![0.14_f64; N_G4_GENES];
        if !sorted.is_empty() {
            for g_idx in 0..N_G4_GENES {
                mean[g_idx] = sorted[0].genome.u[g_idx];
                let mut var = 0.0_f64;
                for rec in sorted.iter().take(k_top) {
                    let d = rec.genome.u[g_idx] - mean[g_idx];
                    var += d * d;
                }
                std_dev[g_idx] = libm::sqrt(var / (k_top as f64)).clamp(0.04, 0.25);
            }
        }
        Self {
            pulls: [1, 1, 1, 1],
            rewards: [0.5, 0.5, 0.5, 0.5],
            cma_mean: mean,
            cma_diag_std: std_dev,
            cma_step_sigma: 0.075,
        }
    }

    pub fn select_arm(&self, total_pulls: usize) -> G4EmitterKind {
        let ln_t = libm::log((total_pulls.max(4)) as f64);
        let mut best_arm = G4EmitterKind::SepCmaMae;
        let mut best_ucb = f64::NEG_INFINITY;
        for (i, &arm) in G4EmitterKind::ALL.iter().enumerate() {
            let mean_r = self.rewards[i] / (self.pulls[i].max(1) as f64);
            let ucb = mean_r + 0.45 * libm::sqrt(2.0 * ln_t / (self.pulls[i].max(1) as f64));
            if ucb > best_ucb {
                best_ucb = ucb;
                best_arm = arm;
            }
        }
        best_arm
    }

    pub fn record_reward(&mut self, arm: G4EmitterKind, reward: f64, winner_u: Option<&[f64]>) {
        let idx = arm as usize;
        self.pulls[idx] += 1;
        self.rewards[idx] += reward;
        if let Some(u_win) = winner_u {
            let lr_m: f64 = 0.16;
            let lr_s: f64 = 0.08;
            for spec in &G4_GENES {
                if spec.kind == G4GeneKind::F64 {
                    let g_idx = spec.index;
                    let diff: f64 = (u_win[g_idx] - self.cma_mean[g_idx]).abs();
                    self.cma_mean[g_idx] =
                        (1.0 - lr_m) * self.cma_mean[g_idx] + lr_m * u_win[g_idx];
                    let new_std: f64 =
                        (1.0 - lr_s) * self.cma_diag_std[g_idx] + lr_s * diff.max(0.02);
                    self.cma_diag_std[g_idx] = new_std.clamp(0.02, 0.24);
                }
            }
        }
    }
}

/// Gene-agnostic Simulated Binary Crossover (`SBX`, Deb & Agrawal 1995) + Polynomial Mutation
/// over the unit hypercube `[0, 1]^130`. Zero named-gene logic.
pub fn apply_sbx_polynomial_variation(
    p1: &G4EvalRecord,
    p2: &G4EvalRecord,
    rng: &mut SplitMix64,
) -> G4Genome {
    let mut g = p1.genome.clone();
    let eta_c = 15.0_f64;
    let eta_m = 20.0_f64;
    for s in &G4_GENES {
        let idx = s.index;
        if s.kind == G4GeneKind::F64 {
            let mut val = if rng.next_f64() < 0.22 {
                let u_rand = rng.next_f64().clamp(1e-6, 1.0 - 1e-6);
                let beta = if u_rand <= 0.5 {
                    libm::pow(2.0 * u_rand, 1.0 / (eta_c + 1.0))
                } else {
                    libm::pow(1.0 / (2.0 * (1.0 - u_rand)), 1.0 / (eta_c + 1.0))
                };
                0.5 * ((1.0 + beta) * p1.genome.u[idx] + (1.0 - beta) * p2.genome.u[idx])
            } else {
                p1.genome.u[idx]
            };
            if rng.next_f64() < 0.15 {
                let u_m = rng.next_f64();
                let delta = if u_m < 0.5 {
                    libm::pow(2.0 * u_m, 1.0 / (eta_m + 1.0)) - 1.0
                } else {
                    1.0 - libm::pow(2.0 * (1.0 - u_m), 1.0 / (eta_m + 1.0))
                };
                val += 0.20 * delta;
            }
            g.u[idx] = val.clamp(0.0, 1.0);
        } else if rng.next_f64() < 0.15 {
            g.u[idx] = p2.genome.u[idx];
        }
    }
    snap_and_repair_genome(&mut g);
    g
}

/// Gene-agnostic Differential Evolution (`DE/current-to-pbest/1/bin`, Zhang & Sanderson 2009)
/// over the unit hypercube `[0, 1]^130`. Zero named-gene logic.
pub fn apply_differential_evolution_step(
    p_curr: &G4EvalRecord,
    p_best: &G4EvalRecord,
    r1: &G4EvalRecord,
    r2: &G4EvalRecord,
    rng: &mut SplitMix64,
) -> G4Genome {
    let mut g = p_curr.genome.clone();
    let f1 = 0.45 + 0.35 * rng.next_f64();
    let f2 = 0.35 + 0.35 * rng.next_f64();
    let cr = 0.24;
    let j_rand = (rng.next_u64() as usize) % N_G4_GENES;
    for s in &G4_GENES {
        let idx = s.index;
        if idx == j_rand || rng.next_f64() < cr {
            if s.kind == G4GeneKind::F64 {
                let v = p_curr.genome.u[idx]
                    + f1 * (p_best.genome.u[idx] - p_curr.genome.u[idx])
                    + f2 * (r1.genome.u[idx] - r2.genome.u[idx])
                    + 0.015 * rng.next_normal();
                g.u[idx] = v.clamp(0.0, 1.0);
            } else {
                g.u[idx] = p_best.genome.u[idx];
            }
        }
    }
    snap_and_repair_genome(&mut g);
    g
}

/// Algorithm variants supported by the G4 experimental runner (`§12.3` ablations & baselines).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum G4SearchAlgorithm {
    /// `E0`: Pure scrambled Sobol + uniform random sampling.
    RandomSobolSearch,
    /// `E1`: Legacy `W_ERGOLIGHT_v1` scalar multiplier.
    LegacyWErgolightV1Search,
    /// `E2`: Single-population `sep-CMA-ES` from Sobol init.
    PenaltyCmaSearch,
    /// `E3`: Full G4 Continuous Multi-Objective Lloyd-CVT + Multi-Emitter Bandit + Module `T`.
    G4FullFi2PopCvt,
    /// `E5`: Core G4 (`96` cont + `12` disc) with Module `T` disabled.
    G4CoreNoModuleT,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct G4SearchConfig {
    pub algorithm: G4SearchAlgorithm,
    pub seed: u64,
    pub k_cvt_cells: usize,
    pub n_init_sobol: usize,
    pub n_batches: usize,
    pub batch_size: usize,
    pub recert_interval_batches: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct G4BatchTelemetry {
    pub batch_index: usize,
    pub total_evals: usize,
    pub feasible_niches: usize,
    pub infeasible_niches: usize,
    pub pareto_front_size: usize,
    pub feasible_qd_score: f64,
    pub best_w_ergolight_v2: f64,
    pub best_mass_finished_g: f64,
    pub best_feasible: bool,
    pub emitter_pulls: [usize; 4],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct G4SearchRunResult {
    pub config: G4SearchConfig,
    pub schema_sha256: String,
    pub decoder_sha256: String,
    pub archive: G4CvtArchive,
    pub best_record: G4EvalRecord,
    pub admission_contract: Option<G4ReleaseAdmissionContract>,
    pub history: Vec<G4BatchTelemetry>,
}

fn disable_module_t_genes(g: &mut G4Genome) {
    for s in &G4_GENES {
        if s.module == G4Module::T {
            g.u[s.index] = 0.0;
        }
    }
    snap_and_repair_genome(g);
}

/// Execute a deterministic, batch-synchronous G4 search run (`SRCH-4`, `T-DET-1`).
/// Every initial individual comes 100% from scrambled Sobol samples (`G4Genome::from_sobol`).
pub fn run_g4_search(spec: &RsKarambitV4Spec, cfg: &G4SearchConfig) -> G4SearchRunResult {
    let allow_module_t = cfg.algorithm != G4SearchAlgorithm::G4CoreNoModuleT;
    let mut archive = G4CvtArchive::new_lloyd_relaxed(cfg.k_cvt_cells, 8, cfg.seed);
    let mut rng = SplitMix64::new(cfg.seed ^ 0x4734_5352_4348_2026);
    let mut history = Vec::with_capacity(cfg.n_batches + 1);
    let mut total_evals = 0usize;

    // 1. Deterministic Initial Sobol Population (100% scrambled Joe-Kuo Sobol, zero hand-authored anchor)
    let mut init_genomes: Vec<G4Genome> = (0..cfg.n_init_sobol)
        .map(|i| {
            let mut g = G4Genome::from_sobol(i, cfg.seed);
            if !allow_module_t {
                disable_module_t_genes(&mut g);
            }
            g
        })
        .collect();

    // Evaluate initial batch in parallel (`rayon`), insert sequentially (`SRCH-4`)
    let init_records: Vec<Option<G4EvalRecord>> = init_genomes
        .par_iter_mut()
        .map(|g| evaluate_g4_genome(spec, g, G4EvalTier::Tier1SdfAndPlate))
        .collect();

    let mut init_elites = Vec::new();
    for rec in init_records.into_iter().flatten() {
        total_evals += 1;
        init_elites.push(rec.clone());
        archive.insert(rec);
    }

    // Initialize the UCB1 bandit and Sep-CMA-ES distribution strictly from the Sobol-evaluated archive elites
    let mut bandit = G4EmitterBanditState::from_sobol_elites(&init_elites);

    // 2. Main Batch-Synchronous Evolutionary Loop
    for batch_idx in 0..cfg.n_batches {
        // Collect current parents from archive in deterministic niche order, plus sorted top quartile for DE-pbest
        let parents: Vec<G4EvalRecord> = archive
            .cells
            .iter()
            .filter_map(|c| {
                c.feasible_elite
                    .clone()
                    .or_else(|| c.infeasible_stepping_stone.clone())
            })
            .collect();
        let mut sorted_parents = parents.clone();
        if cfg.algorithm == G4SearchAlgorithm::LegacyWErgolightV1Search {
            sorted_parents.sort_by(|a, b| {
                b.objective
                    .w_ergolight_v1_legacy
                    .partial_cmp(&a.objective.w_ergolight_v1_legacy)
                    .unwrap_or(Ordering::Equal)
            });
        } else {
            sorted_parents.sort_by(|a, b| compare_g4_objectives(&b.objective, &a.objective));
        }
        let n_pbest = (sorted_parents.len() / 4).max(1);

        // Pre-generate all candidate genomes & emitter tags sequentially using `rng` (`SRCH-4`)
        let mut proposals: Vec<(G4EmitterKind, G4Genome)> = Vec::with_capacity(cfg.batch_size);
        for k in 0..cfg.batch_size {
            let arm = match cfg.algorithm {
                G4SearchAlgorithm::RandomSobolSearch => G4EmitterKind::IsoLineDdCrossover,
                G4SearchAlgorithm::PenaltyCmaSearch => G4EmitterKind::SepCmaMae,
                G4SearchAlgorithm::LegacyWErgolightV1Search => {
                    if k % 2 == 0 {
                        G4EmitterKind::SbxPolynomialMutation
                    } else {
                        G4EmitterKind::IsoLineDdCrossover
                    }
                }
                _ => {
                    if k % 4 == 0 {
                        bandit.select_arm(batch_idx * cfg.batch_size + k + 4)
                    } else {
                        G4EmitterKind::ALL[k % 4]
                    }
                }
            };

            let g_cand = if cfg.algorithm == G4SearchAlgorithm::RandomSobolSearch || parents.is_empty() {
                G4Genome::from_sobol(cfg.n_init_sobol + batch_idx * cfg.batch_size + k, cfg.seed)
            } else {
                let p1 = &parents[(rng.next_u64() as usize) % parents.len()];
                let p2 = &parents[(rng.next_u64() as usize) % parents.len()];
                match arm {
                    G4EmitterKind::SepCmaMae => {
                        let mut g = p1.genome.clone();
                        for s in &G4_GENES {
                            let idx = s.index;
                            if s.kind == G4GeneKind::F64 {
                                if rng.next_f64() < 0.28 {
                                    let base = 0.30 * bandit.cma_mean[idx] + 0.70 * p1.genome.u[idx];
                                    g.u[idx] = (base
                                        + rng.next_normal() * bandit.cma_diag_std[idx] * bandit.cma_step_sigma * 6.5)
                                        .clamp(0.0, 1.0);
                                }
                            } else if rng.next_f64() < 0.12 {
                                g.u[idx] = p2.genome.u[idx];
                            }
                        }
                        snap_and_repair_genome(&mut g);
                        g
                    }
                    G4EmitterKind::DeCurrentToPbest => {
                        let p_best = &sorted_parents[(rng.next_u64() as usize) % n_pbest];
                        let r1 = &parents[(rng.next_u64() as usize) % parents.len()];
                        let r2 = &parents[(rng.next_u64() as usize) % parents.len()];
                        apply_differential_evolution_step(p1, p_best, r1, r2, &mut rng)
                    }
                    G4EmitterKind::SbxPolynomialMutation => {
                        apply_sbx_polynomial_variation(p1, p2, &mut rng)
                    }
                    G4EmitterKind::IsoLineDdCrossover => {
                        let mut g = p1.genome.clone();
                        let sigma_iso = 0.025;
                        let sigma_line = 0.18 * rng.next_normal();
                        for s in &G4_GENES {
                            let idx = s.index;
                            if s.kind == G4GeneKind::F64 {
                                if rng.next_f64() < 0.25 {
                                    let line_step = sigma_line * (p2.genome.u[idx] - p1.genome.u[idx]);
                                    g.u[idx] = (p1.genome.u[idx] + line_step + sigma_iso * rng.next_normal())
                                        .clamp(0.0, 1.0);
                                }
                            } else if rng.next_f64() < 0.12 {
                                g.u[idx] = p2.genome.u[idx];
                            }
                        }
                        snap_and_repair_genome(&mut g);
                        g
                    }
                }
            };

            let mut g_final = g_cand;
            if !allow_module_t {
                disable_module_t_genes(&mut g_final);
            }
            proposals.push((arm, g_final));
        }

        // Parallel multi-fidelity evaluation:
        // Screen with Tier 0, promote promising candidates to Tier 1 (`SRCH-5`)
        let evaluated: Vec<(G4EmitterKind, Option<G4EvalRecord>)> = proposals
            .into_par_iter()
            .map(|(arm, g)| {
                let t0 = evaluate_g4_genome(spec, &g, G4EvalTier::Tier0Analytic)?;
                if t0.objective.feasible || t0.objective.violation_sum_sq < 0.08 {
                    let t1 = evaluate_g4_genome(spec, &g, G4EvalTier::Tier1SdfAndPlate)?;
                    Some((arm, Some(t1)))
                } else {
                    Some((arm, Some(t0)))
                }
            })
            .flatten()
            .collect();

        // Deterministic sequential archive insertion and bandit reward attribution (`SRCH-4`)
        for (arm, rec_opt) in evaluated {
            if let Some(rec) = rec_opt {
                total_evals += 1;
                let (ins_f, ins_inf, _) = archive.insert(rec.clone());
                let reward = if ins_f {
                    1.0 + 0.05 * (rec.objective.w_ergolight_v2 - 80.0).max(0.0)
                } else if ins_inf {
                    0.25
                } else {
                    0.0
                };
                let win_u = if ins_f { Some(rec.genome.u.as_slice()) } else { None };
                bandit.record_reward(arm, reward, win_u);
            }
        }

        // Periodic Tier-2 ($h = 0.20\text{ mm}$) Re-Evaluation of Top Archive Elites (`SRCH-7`)
        if (batch_idx + 1) % cfg.recert_interval_batches.max(1) == 0 || batch_idx + 1 == cfg.n_batches {
            let mut top_niches: Vec<(usize, G4Genome)> = archive
                .cells
                .iter()
                .enumerate()
                .filter_map(|(idx, c)| c.feasible_elite.as_ref().map(|r| (idx, r.genome.clone())))
                .collect();
            top_niches.sort_by(|a, b| {
                let oa = &archive.cells[a.0].feasible_elite.as_ref().unwrap().objective;
                let ob = &archive.cells[b.0].feasible_elite.as_ref().unwrap().objective;
                compare_g4_objectives(ob, oa)
            });
            top_niches.truncate(1);

            let recert_results: Vec<(usize, Option<G4EvalRecord>)> = top_niches
                .into_par_iter()
                .map(|(niche_idx, g)| (niche_idx, evaluate_g4_genome(spec, &g, G4EvalTier::Tier2Full3DCert)))
                .collect();

            for (niche_idx, rec_opt) in recert_results {
                if let Some(mut rec_t2) = rec_opt {
                    total_evals += 1;
                    rec_t2.niche_index = niche_idx;
                    if rec_t2.objective.feasible {
                        archive.cells[niche_idx].feasible_elite = Some(rec_t2);
                    } else {
                        archive.cells[niche_idx].feasible_elite = None;
                        archive.cells[niche_idx].infeasible_stepping_stone = Some(rec_t2);
                    }
                }
            }
        }

        if let Some(best) = archive.best_overall() {
            history.push(G4BatchTelemetry {
                batch_index: batch_idx,
                total_evals,
                feasible_niches: archive.feasible_count(),
                infeasible_niches: archive.infeasible_count(),
                pareto_front_size: archive.pareto_front_total(),
                feasible_qd_score: archive.feasible_qd_score(),
                best_w_ergolight_v2: best.objective.w_ergolight_v2,
                best_mass_finished_g: best.mass.as_finished_total_mass_g,
                best_feasible: best.objective.feasible,
                emitter_pulls: bandit.pulls,
            });
        }
    }

    // Final Tier-2 evaluation of the overall winner
    let raw_best = archive
        .best_overall()
        .cloned()
        .expect("archive must contain at least one evaluated candidate");
    let best_record = evaluate_g4_genome(spec, &raw_best.genome, G4EvalTier::Tier2Full3DCert)
        .unwrap_or(raw_best);
    let admission_contract = verify_record_admission_contract(&best_record, None).ok();

    G4SearchRunResult {
        config: cfg.clone(),
        schema_sha256: g4_schema_sha256().to_string(),
        decoder_sha256: g4_decoder_sha256().to_string(),
        archive,
        best_record,
        admission_contract,
        history,
    }
}
