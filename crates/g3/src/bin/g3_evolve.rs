use std::fmt::Write as _;

use jarambit_g3::g3_genome::G3Genome;
use jarambit_g3::g3_namespace::{G3CohortConfig, format_g3_design_id, validate_g3_design_id};
use jarambit_g3::g3_qd::{g3_decoder_sha256, run_g3_cvt_map_elites_seeded, splitmix64};
use jarambit_g3::g3_sdf::extract_watertight_mesh_fail_closed;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let sarg = |k: &str, d: &str| {
        if let Some(i) = a.iter().position(|x| x == k) {
            let Some(v) = a.get(i + 1) else {
                eprintln!("g3-evolve error: missing value for flag {k}");
                std::process::exit(2);
            };
            v.clone()
        } else {
            d.to_string()
        }
    };
    let narg = |k: &str, d: usize| {
        if let Some(i) = a.iter().position(|x| x == k) {
            let Some(v) = a.get(i + 1) else {
                eprintln!("g3-evolve error: missing value for flag {k}");
                std::process::exit(2);
            };
            v.parse::<usize>().unwrap_or_else(|e| {
                eprintln!("g3-evolve error: invalid usize for {k} ('{v}'): {e}");
                std::process::exit(2);
            })
        } else {
            d
        }
    };
    let u64arg_opt = |k: &str| -> Option<u64> {
        if let Some(i) = a.iter().position(|x| x == k) {
            let Some(v) = a.get(i + 1) else {
                eprintln!("g3-evolve error: missing value for flag {k}");
                std::process::exit(2);
            };
            Some(v.parse::<u64>().unwrap_or_else(|e| {
                eprintln!("g3-evolve error: invalid u64 for {k} ('{v}'): {e}");
                std::process::exit(2);
            }))
        } else {
            None
        }
    };
    let farg = |k: &str, d: f64| {
        if let Some(i) = a.iter().position(|x| x == k) {
            let Some(v) = a.get(i + 1) else {
                eprintln!("g3-evolve error: missing value for flag {k}");
                std::process::exit(2);
            };
            let parsed = v.parse::<f64>().unwrap_or_else(|e| {
                eprintln!("g3-evolve error: invalid f64 for {k} ('{v}'): {e}");
                std::process::exit(2);
            });
            if !parsed.is_finite() {
                eprintln!("g3-evolve error: non-finite f64 for {k} ('{v}')");
                std::process::exit(2);
            }
            parsed
        } else {
            d
        }
    };

    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/ubuntu".to_string());
    let default_out = format!("{home}/jarambit_runs/evo_g3/run_local");
    let out_dir = sarg("--out", &default_out);
    let n_init = narg("--init", 64);
    let n_evals = narg("--evals", 512);
    let host_num = narg("--host", 2) as u32;
    let island = narg("--island", 1) as u32;
    let host_island_key = splitmix64(((host_num as u64) << 32) | (island as u64));
    let raw_seed = u64arg_opt("--seed").unwrap_or(0);
    let seed = splitmix64(raw_seed ^ host_island_key).max(1);
    let t1_gate = farg("--tier1-gate", 62.0);
    let t2_gate = farg("--tier2-gate", 72.0);
    let h_vox = farg("--h", 0.5);
    let default_commit = &g3_decoder_sha256()[..12];
    let commit = sarg("--commit", default_commit);

    let mut cfg = G3CohortConfig::default();
    cfg.run_root = out_dir.clone();
    if let Err(e) = cfg.validate() {
        eprintln!("g3-evolve config error: {e}");
        std::process::exit(2);
    }
    if commit.len() < 7 || !commit.chars().all(|c| c.is_ascii_hexdigit()) {
        eprintln!("g3-evolve error: --commit must be a 7..40 char hex git commit SHA, got '{commit}'");
        std::process::exit(2);
    }

    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        eprintln!("g3-evolve error creating {out_dir}: {e}");
        std::process::exit(1);
    }

    // --seed-genomes <genomes_g3.csv>: continue from an earlier archive. Rows are `design_id,` + the strict G3.0 row
    // (genome sha256 verified against the row's own commit); genes are re-stamped and re-evaluated, and the source,
    // its sha256 and each seed's former genome sha256 are recorded in `seeds.json`.
    let seed_path = sarg("--seed-genomes", "");
    let mut seeds = Vec::new();
    if !seed_path.is_empty() {
        let text = std::fs::read_to_string(&seed_path).unwrap_or_else(|e| {
            eprintln!("g3-evolve error reading --seed-genomes {seed_path}: {e}");
            std::process::exit(2);
        });
        for (k, line) in text.lines().enumerate().skip(1).filter(|(_, l)| !l.trim().is_empty()) {
            let row = line.split_once(',').map(|(_, r)| r).unwrap_or("");
            match G3Genome::from_csv_row(row) {
                Ok(g) => seeds.push(g),
                Err(e) => {
                    eprintln!("g3-evolve error: --seed-genomes line {}: {e}", k + 1);
                    std::process::exit(2);
                }
            }
        }
        if seeds.is_empty() {
            eprintln!("g3-evolve error: --seed-genomes {seed_path} holds no genomes");
            std::process::exit(2);
        }
        let seeds_json = serde_json::json!({
            "source": seed_path,
            "source_sha256": jarambit_g3::sha256_bytes(text.as_bytes()),
            "n_seeds": seeds.len(),
            "former_genome_sha256": seeds.iter().map(|g| g.sha256()).collect::<Vec<_>>(),
            "former_decoder_commit": seeds.iter().map(|g| g.decoder_commit.clone()).collect::<Vec<_>>(),
            "re_evaluated_under_commit": commit,
        });
        if let Err(e) = std::fs::write(format!("{out_dir}/seeds.json"), serde_json::to_string_pretty(&seeds_json).unwrap()) {
            eprintln!("g3-evolve error writing seeds.json: {e}");
            std::process::exit(1);
        }
    }

    let arch = run_g3_cvt_map_elites_seeded(&seeds, n_init, n_evals, seed, t1_gate, t2_gate, h_vox, &commit);
    if arch.elites.is_empty() {
        eprintln!("g3-evolve error: archive produced zero elites");
        std::process::exit(1);
    }
    if let Err(e) = arch.verify_integrity() {
        eprintln!("g3-evolve archive integrity error: {e}");
        std::process::exit(1);
    }

    let mut csv = format!("design_id,{}\n", G3Genome::csv_header());
    let mut jsonl = String::new();
    for (&niche, e) in &arch.elites {
        let design_id = format_g3_design_id(host_num, island, niche as u64, &e.genome.sha256());
        if let Err(err) = validate_g3_design_id(&design_id) {
            eprintln!("g3-evolve design_id validation error: {err}");
            std::process::exit(2);
        }
        let _ = writeln!(csv, "{design_id},{}", e.genome.to_csv_row());
        let score_json = e.record.score.to_json();
        let row_json = serde_json::json!({
            "design_id": design_id,
            "niche_id": niche,
            "schema": e.record.schema,
            "genome_sha256": e.record.genome_sha256,
            "decoder_commit": e.genome.decoder_commit,
            "decoder_sha256": e.record.decoder_sha256,
            "tier": e.record.tier,
            "tier_provenance": e.record.tier.input_provenance(),
            "score_100": e.record.score.score_100,
            "raw_score_100": e.record.score.raw_score_100,
            "base": e.record.score.base,
            "ergo_multiplier": e.record.score.ergo_multiplier,
            "descriptors": e.record.descriptors,
            "inputs": e.record.inputs,
            "terms": score_json["terms"],
            "ergo": score_json["ergo"],
        });
        let _ = writeln!(jsonl, "{row_json}");
    }
    if let Err(e) = std::fs::write(format!("{out_dir}/genomes_g3.csv"), csv) {
        eprintln!("g3-evolve error writing genomes_g3.csv: {e}");
        std::process::exit(1);
    }
    if let Err(e) = std::fs::write(format!("{out_dir}/archive.jsonl"), jsonl) {
        eprintln!("g3-evolve error writing archive.jsonl: {e}");
        std::process::exit(1);
    }
    if let Err(e) = jarambit_g3::g3_qd::CvtMapElitesArchive::load_and_verify_persisted_dir(&out_dir, &commit) {
        eprintln!("g3-evolve persisted archive verification error: {e}");
        std::process::exit(1);
    }

    let Some(best) = arch.best_elite() else {
        eprintln!("g3-evolve error: no best elite found");
        std::process::exit(1);
    };
    let best_id = format_g3_design_id(host_num, island, best.niche_id as u64, &best.genome.sha256());
    validate_g3_design_id(&best_id).unwrap();
    let best_json = serde_json::to_string_pretty(best).unwrap();
    std::fs::write(format!("{out_dir}/{best_id}_elite.json"), best_json).unwrap();
    let Some(solid) = extract_watertight_mesh_fail_closed(&best.genome, h_vox, true) else {
        eprintln!("g3-evolve error: best elite mesh failed watertight census");
        std::process::exit(1);
    };
    let (boundary, non_manifold, dmax) = jarambit_crescent::export::census(&solid.tris);
    if boundary != 0 || non_manifold != 0 || dmax != 1 {
        eprintln!(
            "g3-evolve error: best elite mesh is non-watertight (census=({boundary}, {non_manifold}, {dmax}))"
        );
        std::process::exit(1);
    }
    std::fs::write(
        format!("{out_dir}/{best_id}.stl"),
        jarambit_crescent::export::stl(&solid, &best_id),
    )
    .unwrap();
    println!(
        "g3-evolve done: best_id={best_id} niches={} qd_score={:.2} best_score={:.2} mass={:.2}g E={:.3} raw_seed={raw_seed} effective_seed={seed} decoder_sha256={}",
        arch.elites.len(),
        arch.qd_score(),
        best.record.score.raw_score_100,
        best.record.inputs.mass_g,
        best.record.score.ergo_multiplier,
        arch.decoder_sha256
    );
}
