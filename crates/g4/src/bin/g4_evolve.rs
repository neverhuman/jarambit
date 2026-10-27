//! `g4-evolve`: Executes the Multi-Seed Constrained Quality-Diversity Cohort & Ablation Suite
//! (`E0`, `E1`, `E2`, `E3`, `E5`), enforces the fail-closed `G4ReleaseAdmissionContract` (`RI-01`..`RI-12`),
//! and writes `evidence/g4/v4.1.0/{release.json,winner_genome.csv,evolution_summary.json,history/cohort_metrics.json}`.

use jarambit_g4::genome::{g4_decoder_sha256, g4_schema_sha256, G4_GENES};
use jarambit_g4::objective::{compare_g4_objectives, RsKarambitV4Spec};
use jarambit_g4::search::{
    run_g4_search, verify_record_admission_contract, G4BatchTelemetry, G4EvalRecord,
    G4ReleaseAdmissionContract, G4SearchAlgorithm, G4SearchConfig,
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedRunSummary {
    pub algorithm_id: String,
    pub seed: u64,
    pub total_evals: usize,
    pub feasible_niches: usize,
    pub infeasible_niches: usize,
    pub pareto_front_size: usize,
    pub feasible_qd_score: f64,
    pub best_feasible: bool,
    pub best_w_ergolight_v2: f64,
    pub best_w_ergolight_v1_legacy: f64,
    pub best_finished_mass_g: f64,
    pub best_printed_mass_g: f64,
    pub best_min_safety_margin: f64,
    pub winner_genome_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortGroupStats {
    pub algorithm_id: String,
    pub n_seeds: usize,
    pub median_w_ergolight_v2: f64,
    pub iqr_w_ergolight_v2: f64,
    pub median_finished_mass_g: f64,
    pub iqr_finished_mass_g: f64,
    pub median_feasible_niches: f64,
    pub median_feasible_qd_score: f64,
    pub vargha_delaney_a12_vs_e3: f64,
    pub runs: Vec<SeedRunSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct G4ReleaseBundle {
    pub version: String,
    pub schema_version: String,
    pub requirement_set: String,
    pub winner_id: String,
    pub winner_seed: u64,
    pub schema_sha256: String,
    pub decoder_sha256: String,
    pub genome_sha256: String,
    pub winner_record: G4EvalRecord,
    pub admission_contract: G4ReleaseAdmissionContract,
    pub winner_history: Vec<G4BatchTelemetry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct G4EvolutionSummary {
    pub version: String,
    pub winner_id: String,
    pub schema_sha256: String,
    pub decoder_sha256: String,
    pub cohorts: Vec<CohortGroupStats>,
}

fn median_and_iqr(vals: &[f64]) -> (f64, f64) {
    if vals.is_empty() {
        return (0.0, 0.0);
    }
    let mut s = vals.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = s.len();
    let med = if n % 2 == 1 {
        s[n / 2]
    } else {
        0.5 * (s[n / 2 - 1] + s[n / 2])
    };
    let q1 = s[n / 4];
    let q3 = s[(3 * n) / 4];
    (med, q3 - q1)
}

/// Compute Vargha-Delaney $\hat{A}_{12}$ effect size of `e3_scores` vs `other_scores`.
fn vargha_delaney_a12(e3_scores: &[f64], other_scores: &[f64]) -> f64 {
    if e3_scores.is_empty() || other_scores.is_empty() {
        return 0.5;
    }
    let mut wins = 0.0_f64;
    for &x in e3_scores {
        for &y in other_scores {
            if (x - y).abs() <= 1e-9 {
                wins += 0.5;
            } else if x > y {
                wins += 1.0;
            }
        }
    }
    wins / ((e3_scores.len() * other_scores.len()) as f64)
}

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let out_dir = root.join("evidence/g4/v4.1.0");
    let hist_dir = out_dir.join("history");
    fs::create_dir_all(&hist_dir).expect("create evidence/g4/v4.1.0/history");

    let spec = RsKarambitV4Spec::load_embedded().expect("load RS-karambit-v4.toml");

    // Define the 5 experimental arms (`§12.3`)
    let experiments: &[(&str, G4SearchAlgorithm, &[u64])] = &[
        ("E3_G4_Full", G4SearchAlgorithm::G4FullFi2PopCvt, &[42, 101, 202, 303, 404]),
        ("E5_G4_Core_No_T", G4SearchAlgorithm::G4CoreNoModuleT, &[42, 101, 202]),
        ("E2_Penalty_CMA", G4SearchAlgorithm::PenaltyCmaSearch, &[42, 101, 202]),
        ("E1_Legacy_W_ERGOLIGHT_v1", G4SearchAlgorithm::LegacyWErgolightV1Search, &[42, 101, 202]),
        ("E0_Random_Sobol", G4SearchAlgorithm::RandomSobolSearch, &[42, 101, 202]),
    ];

    let mut raw_groups: Vec<(String, Vec<SeedRunSummary>)> = Vec::new();
    let mut best_e3_run: Option<(u64, jarambit_g4::search::G4SearchRunResult)> = None;

    for &(alg_id, alg, seeds) in experiments {
        let seed_results: Vec<(u64, SeedRunSummary, jarambit_g4::search::G4SearchRunResult)> = seeds
            .par_iter()
            .map(|&seed| {
                let cfg = G4SearchConfig {
                    algorithm: alg,
                    seed,
                    k_cvt_cells: 64,
                    n_init_sobol: 64,
                    n_batches: 36,
                    batch_size: 24,
                    recert_interval_batches: 36,
                };
                let res = run_g4_search(&spec, &cfg);
                let b = &res.best_record;
                let total_evals = res.history.last().map(|h| h.total_evals).unwrap_or(0);
                let summary = SeedRunSummary {
                    algorithm_id: alg_id.to_string(),
                    seed,
                    total_evals,
                    feasible_niches: res.archive.feasible_count(),
                    infeasible_niches: res.archive.infeasible_count(),
                    pareto_front_size: res.archive.pareto_front_total(),
                    feasible_qd_score: res.archive.feasible_qd_score(),
                    best_feasible: b.objective.feasible,
                    best_w_ergolight_v2: b.objective.w_ergolight_v2,
                    best_w_ergolight_v1_legacy: b.objective.w_ergolight_v1_legacy,
                    best_finished_mass_g: b.mass.as_finished_total_mass_g,
                    best_printed_mass_g: b.mass.as_printed_total_mass_g,
                    best_min_safety_margin: b.objective.min_safety_margin,
                    winner_genome_sha256: b.genome.genome_sha256(),
                };
                (seed, summary, res)
            })
            .collect();

        let mut group_runs = Vec::with_capacity(seeds.len());
        for (seed, summary, res) in seed_results {
            group_runs.push(summary);
            if alg == G4SearchAlgorithm::G4FullFi2PopCvt {
                let take = match &best_e3_run {
                    None => true,
                    Some((_, prev)) => {
                        compare_g4_objectives(&res.best_record.objective, &prev.best_record.objective)
                            == Ordering::Greater
                    }
                };
                if take {
                    best_e3_run = Some((seed, res));
                }
            }
        }
        raw_groups.push((alg_id.to_string(), group_runs));
    }

    let e3_scores: Vec<f64> = raw_groups[0]
        .1
        .iter()
        .map(|r| r.best_w_ergolight_v2)
        .collect();

    let cohorts: Vec<CohortGroupStats> = raw_groups
        .into_iter()
        .map(|(alg_id, runs)| {
            let scores: Vec<f64> = runs.iter().map(|r| r.best_w_ergolight_v2).collect();
            let masses: Vec<f64> = runs.iter().map(|r| r.best_finished_mass_g).collect();
            let f_niches: Vec<f64> = runs.iter().map(|r| r.feasible_niches as f64).collect();
            let qd_scores: Vec<f64> = runs.iter().map(|r| r.feasible_qd_score).collect();
            let (med_s, iqr_s) = median_and_iqr(&scores);
            let (med_m, iqr_m) = median_and_iqr(&masses);
            let (med_n, _) = median_and_iqr(&f_niches);
            let (med_qd, _) = median_and_iqr(&qd_scores);
            let a12 = vargha_delaney_a12(&e3_scores, &scores);
            CohortGroupStats {
                algorithm_id: alg_id,
                n_seeds: runs.len(),
                median_w_ergolight_v2: med_s,
                iqr_w_ergolight_v2: iqr_s,
                median_finished_mass_g: med_m,
                iqr_finished_mass_g: iqr_m,
                median_feasible_niches: med_n,
                median_feasible_qd_score: med_qd,
                vargha_delaney_a12_vs_e3: a12,
                runs,
            }
        })
        .collect();

    let (win_seed, win_run) = best_e3_run.expect("E3 G4-Full must produce a verified run");
    let win_rec = win_run.best_record;
    let contract = verify_record_admission_contract(&win_rec, None)
        .expect("G4 winner MUST pass fail-closed G4ReleaseAdmissionContract (RI-01..RI-12)");
    let g_sha = win_rec.genome.genome_sha256();
    let winner_id = format!("g4v410_s{:03}_{}", win_seed, &g_sha[..8]);

    // Verify Sobol dispersion from G4.0.toml defaults (no hand-authored anchor)
    let mut diffs: Vec<f64> = G4_GENES
        .iter()
        .map(|s| {
            let u_def = ((s.default - s.lo) / (s.hi - s.lo)).clamp(0.0, 1.0);
            (win_rec.genome.u[s.index] - u_def).abs()
        })
        .collect();
    let within_005 = diffs.iter().filter(|&&d| d <= 0.05).count();
    diffs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median_du = 0.5 * (diffs[64] + diffs[65]);
    assert!(
        median_du >= 0.16 && within_005 <= 30,
        "Winner must exhibit genuine Sobol dispersion from G4.0.toml defaults (median |du|={:.4}, within_005={})",
        median_du,
        within_005
    );

    // Write `winner_genome.csv`
    let mut csv_lines = Vec::with_capacity(G4_GENES.len() + 1);
    csv_lines.push("index,module,name,kind,unit,lo,hi,u_norm,physical_value".to_string());
    for s in &G4_GENES {
        let u = win_rec.genome.u[s.index];
        let val = s.decode_u(u);
        csv_lines.push(format!(
            "{},{:?},{},{:?},{},{:.6},{:.6},{:.8},{:.8}",
            s.index, s.module, s.name, s.kind, s.unit, s.lo, s.hi, u, val
        ));
    }
    fs::write(out_dir.join("winner_genome.csv"), csv_lines.join("\n") + "\n").unwrap();
    fs::write(root.join("evidence/g4/RECORD"), "4.1.0\n").unwrap();

    // Write `release.json`
    let bundle = G4ReleaseBundle {
        version: "4.1.0".to_string(),
        schema_version: "G4.0".to_string(),
        requirement_set: "RS-karambit-v4".to_string(),
        winner_id: winner_id.clone(),
        winner_seed: win_seed,
        schema_sha256: g4_schema_sha256().to_string(),
        decoder_sha256: g4_decoder_sha256().to_string(),
        genome_sha256: g_sha,
        winner_record: win_rec.clone(),
        admission_contract: contract,
        winner_history: win_run.history,
    };
    fs::write(
        out_dir.join("release.json"),
        serde_json::to_string_pretty(&bundle).unwrap(),
    )
    .unwrap();

    // Write `evolution_summary.json` and `history/cohort_metrics.json`
    let summary = G4EvolutionSummary {
        version: "4.1.0".to_string(),
        winner_id: winner_id.clone(),
        schema_sha256: g4_schema_sha256().to_string(),
        decoder_sha256: g4_decoder_sha256().to_string(),
        cohorts,
    };
    let summary_json = serde_json::to_string_pretty(&summary).unwrap();
    fs::write(out_dir.join("evolution_summary.json"), &summary_json).unwrap();
    fs::write(hist_dir.join("cohort_metrics.json"), &summary_json).unwrap();

    println!(
        "g4-evolve complete: winner {} | feasible={} | W_ERGOLIGHT_v2={:.4} | W_ERGOLIGHT_v1={:.4} | as_finished_mass={:.3} g | as_printed_mass={:.3} g | min_margin={:+.4} | median_du={:.4} (within_005={}/130)",
        winner_id,
        win_rec.objective.feasible,
        win_rec.objective.w_ergolight_v2,
        win_rec.objective.w_ergolight_v1_legacy,
        win_rec.mass.as_finished_total_mass_g,
        win_rec.mass.as_printed_total_mass_g,
        win_rec.objective.min_safety_margin,
        median_du,
        within_005,
    );
}
