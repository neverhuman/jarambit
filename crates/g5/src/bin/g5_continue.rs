//! Continue the G5 evolution from earlier archives (v6.0.0; frozen decoder G5.0). Hash-neutral: only public library
//! APIs are used, and no decoder-hashed file changes.
//!
//!   g5-continue run    --out DIR [--pool DIR] [--init N] --evals N --seed S --host H --island I --commit C
//!                      [--h 0.4] [--tier1-top 0.4] [--tier2-top 0] [--log-every 25] [--stl]
//!   g5-continue reeval --out DIR --genomes CSV[,CSV...] [--h 0.4] [--threads 4] [--shard k/n]
//!   g5-continue merge  --out DIR --from DIR[,DIR...]
//!
//! `run` is one island. Without `--pool` it is exactly `g5-evolve` (Sobol start of `--init`, then the same loop);
//! with `--pool` it starts from that archive (its records inserted as they are, after a bit-exact re-score) and
//! evolves `--evals` more. `reeval` evaluates genomes at Tier 1 on this platform (the epoch-0 pool from the v5
//! elites); `merge` keeps the best record per niche over several archives (the pool of the next epoch). Every record
//! read from disk is rebuilt from its row and re-scored with W_ERGOLIGHT_v2; it must reproduce the row's score bit
//! for bit and carry this binary's decoder hash, and every genome must re-hash to its design id (fail closed).
//! Outputs are the `g5-evolve` files (archive.jsonl, genomes_g5.csv, history.jsonl) plus launch.json or pool.json;
//! no STL unless `--stl` (the v5 islands wrote 330 MB each).
use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs;

use jarambit_g5::ergolight::ErgolightInputs;
use jarambit_g5::g5_genome::{sobol_unit_genome, G5Genome, G5Rng};
use jarambit_g5::g5_namespace::{format_g5_design_id, validate_g5_design_id, G5CohortConfig};
use jarambit_g5::g5_qd::{
    evaluate_tier1_sdf_and_plate, g5_decoder_sha256, measure_point_planform, splitmix64, CvtMapElitesArchive,
    EvalTier, G5Descriptors, G5EvalRecord, G5HistoryPoint, RankPromotion,
};
use jarambit_g5::g5_sdf::{build_blade_profile_2d, extract_watertight_mesh_fail_closed, AmConnectivityStats};
use jarambit_g5::loads::G5Loads;
use jarambit_g5::objective::{score_v2, ObjectiveInputs};
use jarambit_g5::sha256_bytes;
use serde_json::{json, Value};

fn fail(msg: impl std::fmt::Display) -> ! {
    eprintln!("g5-continue error: {msg}");
    std::process::exit(1)
}

struct Args(Vec<String>);

impl Args {
    fn get(&self, k: &str) -> Option<&str> {
        self.0.iter().position(|x| x == k).map(|i| self.0.get(i + 1).map(String::as_str).unwrap_or_else(|| fail(format!("missing value for {k}"))))
    }
    fn req(&self, k: &str) -> String {
        self.get(k).unwrap_or_else(|| fail(format!("{k} is required"))).to_string()
    }
    fn n(&self, k: &str, d: usize) -> usize {
        self.get(k).map_or(d, |v| v.parse().unwrap_or_else(|e| fail(format!("{k} '{v}': {e}"))))
    }
    fn f(&self, k: &str, d: f64) -> f64 {
        let v = self.get(k).map_or(d, |v| v.parse().unwrap_or_else(|e| fail(format!("{k} '{v}': {e}"))));
        if !v.is_finite() {
            fail(format!("{k} must be finite"));
        }
        v
    }
    fn flag(&self, k: &str) -> bool {
        self.0.iter().any(|x| x == k)
    }
}

/// Output roots follow the G5 namespace rule (an `evo_g5` or `evo_g5_*` root, never `evo_g5_dev` once frozen).
fn check_root(dir: &str) {
    let mut cfg = G5CohortConfig::default();
    cfg.run_root = dir.to_string();
    cfg.validate().unwrap_or_else(|e| fail(format!("{dir}: {e}")));
    fs::create_dir_all(dir).unwrap_or_else(|e| fail(format!("{dir}: {e}")));
}

/// The W_ERGOLIGHT_v2 score of a record's measurements (the same call as `g5_qd::score_record`).
fn rescore(g: &G5Genome, inputs: &ErgolightInputs, loads: &G5Loads, am: &AmConnectivityStats) -> jarambit_g5::ergolight::ErgolightScore {
    let blade = build_blade_profile_2d(&g.decode());
    let point = measure_point_planform(&blade);
    score_v2(&ObjectiveInputs { inputs, loads, point: &point, am, blade: &blade })
}

/// A record rebuilt from its archive row; fails unless it re-scores bit for bit under this binary's decoder.
fn record_from_row(id: &str, g: &G5Genome, row: &Value) -> G5EvalRecord {
    let take = |k: &str| row.get(k).cloned().unwrap_or_else(|| fail(format!("{id}: row has no {k}")));
    let hash = g5_decoder_sha256();
    if row["decoder_sha256"] != json!(hash) {
        fail(format!("{id}: row from decoder {}, this binary is {hash}", row["decoder_sha256"]));
    }
    if row["genome_sha256"] != json!(g.sha256()) || !id.ends_with(&g.sha256()[..8]) {
        fail(format!("{id}: genome does not re-hash to its row and id"));
    }
    let inputs: ErgolightInputs = serde_json::from_value(take("inputs")).unwrap_or_else(|e| fail(format!("{id} inputs: {e}")));
    let loads: G5Loads = serde_json::from_value(take("loads")).unwrap_or_else(|e| fail(format!("{id} loads: {e}")));
    let am: AmConnectivityStats = serde_json::from_value(take("am_connectivity")).unwrap_or_else(|e| fail(format!("{id} am: {e}")));
    let descriptors: G5Descriptors = serde_json::from_value(take("descriptors")).unwrap_or_else(|e| fail(format!("{id} descriptors: {e}")));
    let tier: EvalTier = serde_json::from_value(take("tier")).unwrap_or_else(|e| fail(format!("{id} tier: {e}")));
    let score = rescore(g, &inputs, &loads, &am);
    let stored = row["raw_score_100"].as_f64().unwrap_or_else(|| fail(format!("{id}: raw_score_100")));
    if score.raw_score_100.to_bits() != stored.to_bits() {
        fail(format!("{id}: re-score {} != stored {stored}", score.raw_score_100));
    }
    G5EvalRecord {
        schema: take("schema").as_str().unwrap_or_else(|| fail(format!("{id}: schema"))).to_string(),
        decoder_sha256: hash,
        genome_sha256: g.sha256(),
        tier,
        inputs,
        score,
        descriptors,
        am_connectivity: am,
        loads,
    }
}

/// The (design id, genome, archive row) triples of one archive directory, in file order, genomes verified.
fn read_dir_rows(dir: &str) -> Vec<(String, G5Genome, Value)> {
    let csv = fs::read_to_string(format!("{dir}/genomes_g5.csv")).unwrap_or_else(|e| fail(format!("{dir}/genomes_g5.csv: {e}")));
    let mut genomes: HashMap<String, G5Genome> = HashMap::new();
    for line in csv.lines().skip(1).filter(|l| !l.trim().is_empty()) {
        let (id, row) = line.split_once(',').unwrap_or_else(|| fail(format!("{dir}: bad genome row")));
        validate_g5_design_id(id).unwrap_or_else(|e| fail(format!("{id}: {e}")));
        let g = G5Genome::from_csv_row(row).unwrap_or_else(|e| fail(format!("{id}: {e}")));
        genomes.insert(id.to_string(), g);
    }
    let jsonl = fs::read_to_string(format!("{dir}/archive.jsonl")).unwrap_or_else(|e| fail(format!("{dir}/archive.jsonl: {e}")));
    let rows: Vec<(String, G5Genome, Value)> = jsonl
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let v: Value = serde_json::from_str(l).unwrap_or_else(|e| fail(format!("{dir}: {e}")));
            let id = v["design_id"].as_str().unwrap_or_else(|| fail(format!("{dir}: row without design_id"))).to_string();
            let g = genomes.remove(&id).unwrap_or_else(|| fail(format!("{dir}: {id} has no genome row")));
            (id, g, v)
        })
        .collect();
    if !genomes.is_empty() {
        fail(format!("{dir}: {} genome rows without an archive row", genomes.len()));
    }
    rows
}

fn row_json(id: &str, niche: usize, g: &G5Genome, rec: &G5EvalRecord) -> Value {
    let score_json = rec.score.to_json();
    json!({
        "design_id": id, "niche_id": niche, "schema": rec.schema, "genome_sha256": rec.genome_sha256,
        "decoder_commit": g.decoder_commit, "decoder_sha256": rec.decoder_sha256, "tier": rec.tier,
        "tier_provenance": rec.tier.input_provenance(), "score_100": rec.score.score_100,
        "raw_score_100": rec.score.raw_score_100, "base": rec.score.base, "ergo_multiplier": rec.score.ergo_multiplier,
        "descriptors": rec.descriptors, "inputs": rec.inputs, "loads": rec.loads, "am_connectivity": rec.am_connectivity,
        "metric": rec.score.metric, "terms": score_json["terms"], "ergo": score_json["ergo"],
    })
}

/// Write rows as `g5-evolve` does (genomes_g5.csv + archive.jsonl).
fn write_rows(dir: &str, rows: &[(String, usize, &G5Genome, &G5EvalRecord)]) {
    let mut csv = format!("design_id,{}\n", G5Genome::csv_header());
    let mut jsonl = String::new();
    for (id, niche, g, rec) in rows {
        let _ = writeln!(csv, "{id},{}", g.to_csv_row());
        let _ = writeln!(jsonl, "{}", row_json(id, *niche, g, rec));
    }
    fs::write(format!("{dir}/genomes_g5.csv"), csv).unwrap_or_else(|e| fail(format!("{dir}: {e}")));
    fs::write(format!("{dir}/archive.jsonl"), jsonl).unwrap_or_else(|e| fail(format!("{dir}: {e}")));
}

fn history_point(archive: &CvtMapElitesArchive, evals: usize, with_row: bool) -> Option<G5HistoryPoint> {
    let best = archive.best_elite()?;
    Some(G5HistoryPoint {
        evals,
        niches: archive.elites.len(),
        n_tier2: archive.elites.values().filter(|e| e.record.tier == EvalTier::Tier2Full3D).count(),
        qd_score: archive.qd_score(),
        best_tier: best.record.tier,
        best_raw_score_100: best.record.score.raw_score_100,
        best_score_100: best.record.score.score_100,
        best_mass_g: best.record.inputs.mass_g,
        best_niche: best.niche_id,
        best_genome_sha256: best.genome.sha256(),
        best_genome_row: with_row.then(|| best.genome.to_csv_row()),
    })
}

fn sha_file(path: &str) -> String {
    sha256_bytes(&fs::read(path).unwrap_or_else(|e| fail(format!("{path}: {e}"))))
}

fn run(a: &Args) {
    let out = a.req("--out");
    check_root(&out);
    let (host, island) = (a.n("--host", 2) as u32, a.n("--island", 1) as u32);
    let raw_seed: u64 = a.get("--seed").map_or(0, |v| v.parse().unwrap_or_else(|e| fail(format!("--seed: {e}"))));
    let seed = splitmix64(raw_seed ^ splitmix64(((host as u64) << 32) | island as u64)).max(1);
    let (n_init, n_evals) = (a.n("--init", 0), a.n("--evals", 512));
    let (t1, t2, h) = (a.f("--tier1-top", 0.4), a.f("--tier2-top", 0.0), a.f("--h", 0.4));
    let log_every = a.n("--log-every", 25);
    let commit = a.req("--commit");
    if commit.len() < 7 || !commit.chars().all(|c| c.is_ascii_hexdigit()) {
        fail("--commit must be a 7..40 char hex label");
    }
    let pool = a.get("--pool").map(str::to_string);
    if pool.is_none() && n_init == 0 {
        fail("a run needs --pool or --init > 0");
    }

    // The `run_g5_cvt_map_elites_logged` loop, with an optional pooled start
    let mut archive = CvtMapElitesArchive::new(1024);
    let mut promotion = RankPromotion::new(t1, t2);
    let mut rng = G5Rng(seed.max(1));
    let scramble_seed = (splitmix64(seed) as usize).max(1);
    let mut history: Vec<G5HistoryPoint> = Vec::new();
    let mut log = |archive: &CvtMapElitesArchive, evals: usize, last: bool| {
        if log_every > 0 && (last || evals % log_every == 0 || evals.is_power_of_two()) {
            if let Some(p) = history_point(archive, evals, last || evals.is_power_of_two()) {
                // progress for the supervisors (stderr only; the outputs do not change)
                eprintln!("g5-continue progress: evals={evals} niches={} best={:.4}", p.niches, p.best_raw_score_100);
                history.push(p);
            }
        }
    };
    let mut pool_ids: HashMap<String, String> = HashMap::new();
    let mut pool_sha = Value::Null;
    if let Some(dir) = &pool {
        let rows = read_dir_rows(dir);
        for (id, g, row) in rows {
            let rec = record_from_row(&id, &g, &row);
            pool_ids.insert(g.sha256(), id);
            archive.insert(g, rec);
        }
        pool_sha = json!({"archive.jsonl": sha_file(&format!("{dir}/archive.jsonl")),
                          "genomes_g5.csv": sha_file(&format!("{dir}/genomes_g5.csv"))});
        log(&archive, 0, false);
    }
    let init_count = n_init.min(n_evals);
    for i in 0..init_count {
        let g = sobol_unit_genome(i, scramble_seed, &commit);
        let rec = promotion.evaluate(&g, h);
        archive.insert(g, rec);
    }
    let mut evals = init_count;
    if init_count > 0 {
        log(&archive, evals, evals == n_evals);
    }
    for _ in init_count..n_evals {
        let Some((child, parent_nid)) = archive.sample_offspring(&mut rng) else { break };
        let rec = promotion.evaluate(&child, h);
        let improved = archive.insert(child, rec);
        archive.update_curiosity(parent_nid, improved);
        evals += 1;
        log(&archive, evals, evals == n_evals);
    }
    if evals < n_evals || n_evals == 0 {
        log(&archive, evals, true);
    }
    archive.verify_integrity().unwrap_or_else(|e| fail(format!("archive integrity: {e}")));

    // Outputs: pooled elites keep their design id; new elites are named for this host and island
    let named: Vec<(String, usize, &G5Genome, &G5EvalRecord)> = archive
        .elites
        .iter()
        .map(|(&nid, e)| {
            let id = pool_ids.get(&e.genome.sha256()).cloned().unwrap_or_else(|| format_g5_design_id(host, island, nid as u64, &e.genome.sha256()));
            (id, nid, &e.genome, &e.record)
        })
        .collect();
    write_rows(&out, &named);
    let hist: String = history.iter().map(|p| serde_json::to_string(p).unwrap() + "\n").collect();
    fs::write(format!("{out}/history.jsonl"), hist).unwrap_or_else(|e| fail(e));
    let launch = json!({"tool": "g5-continue run", "decoder_sha256": g5_decoder_sha256(),
        "binary_sha256": std::env::current_exe().ok().and_then(|p| fs::read(p).ok()).map(|b| sha256_bytes(&b)),
        "host": host, "island": island, "raw_seed": raw_seed, "effective_seed": seed, "init": n_init, "evals": n_evals,
        "h": h, "tier1_top": t1, "tier2_top": t2, "log_every": log_every, "commit": commit, "pool": pool, "pool_sha256": pool_sha});
    fs::write(format!("{out}/launch.json"), serde_json::to_string_pretty(&launch).unwrap() + "\n").unwrap_or_else(|e| fail(e));
    let best = archive.best_elite().unwrap_or_else(|| fail("empty archive"));
    if a.flag("--stl") {
        let solid = extract_watertight_mesh_fail_closed(&best.genome, h, true).unwrap_or_else(|| fail("best elite mesh not watertight"));
        fs::write(format!("{out}/{}.stl", best.genome.sha256()), jarambit_crescent::export::stl(&solid, "best")).unwrap_or_else(|e| fail(e));
    }
    println!(
        "g5-continue done: niches={} qd_score={:.2} best_score={:.4} mass={:.2}g evals={evals} pool={} raw_seed={raw_seed} decoder_sha256={}",
        archive.elites.len(),
        archive.qd_score(),
        best.record.score.raw_score_100,
        best.record.inputs.mass_g,
        pool.as_deref().unwrap_or("none"),
        g5_decoder_sha256()
    );
}

fn reeval(a: &Args) {
    use rayon::prelude::*;
    let out = a.req("--out");
    check_root(&out);
    let h = a.f("--h", 0.4);
    rayon::ThreadPoolBuilder::new().num_threads(a.n("--threads", 4)).build_global().ok();
    let (k, n) = a.get("--shard").map_or((0, 1), |s| {
        let (k, n) = s.split_once('/').unwrap_or_else(|| fail("--shard k/n"));
        (k.parse().unwrap_or_else(|_| fail("--shard k")), n.parse().unwrap_or_else(|_| fail("--shard n")))
    });
    let mut genomes: Vec<(String, G5Genome)> = Vec::new();
    for csv in a.req("--genomes").split(',') {
        let text = fs::read_to_string(csv).unwrap_or_else(|e| fail(format!("{csv}: {e}")));
        for line in text.lines().skip(1).filter(|l| !l.trim().is_empty()) {
            let (id, row) = line.split_once(',').unwrap_or_else(|| fail(format!("{csv}: bad row")));
            validate_g5_design_id(id).unwrap_or_else(|e| fail(format!("{id}: {e}")));
            let g = G5Genome::from_csv_row(row).unwrap_or_else(|e| fail(format!("{id}: {e}")));
            if !id.ends_with(&g.sha256()[..8]) {
                fail(format!("{id}: genome does not re-hash to its id"));
            }
            genomes.push((id.to_string(), g));
        }
    }
    let mine: Vec<&(String, G5Genome)> = genomes.iter().enumerate().filter(|(i, _)| i % n == k).map(|(_, x)| x).collect();
    let recs: Vec<G5EvalRecord> = mine.par_iter().map(|(_, g)| evaluate_tier1_sdf_and_plate(g, h)).collect();
    let probe = CvtMapElitesArchive::new(1024);
    let rows: Vec<(String, usize, &G5Genome, &G5EvalRecord)> =
        mine.iter().zip(recs.iter()).map(|((id, g), r)| (id.clone(), probe.niche_of(&r.descriptors), g, r)).collect();
    write_rows(&out, &rows);
    eprintln!("g5-continue reeval: {} of {} genomes (shard {k}/{n}) at h {h} -> {out}", rows.len(), genomes.len());
}

fn merge(a: &Args) {
    let out = a.req("--out");
    check_root(&out);
    let mut archive = CvtMapElitesArchive::new(1024);
    let mut ids: HashMap<String, String> = HashMap::new();
    let mut sources = Vec::new();
    let mut n_rows = 0;
    for dir in a.req("--from").split(',') {
        let rows = read_dir_rows(dir);
        sources.push(json!({"dir": dir, "rows": rows.len(), "archive.jsonl": sha_file(&format!("{dir}/archive.jsonl"))}));
        for (id, g, row) in rows {
            let rec = record_from_row(&id, &g, &row);
            ids.entry(g.sha256()).or_insert(id);
            archive.insert(g, rec);
            n_rows += 1;
        }
    }
    archive.verify_integrity().unwrap_or_else(|e| fail(format!("archive integrity: {e}")));
    let named: Vec<(String, usize, &G5Genome, &G5EvalRecord)> =
        archive.elites.iter().map(|(&nid, e)| (ids[&e.genome.sha256()].clone(), nid, &e.genome, &e.record)).collect();
    write_rows(&out, &named);
    let best = archive.best_elite().unwrap_or_else(|| fail("empty merge"));
    let pool = json!({"tool": "g5-continue merge", "decoder_sha256": g5_decoder_sha256(), "sources": sources,
        "rows_read": n_rows, "niches": archive.elites.len(), "qd_score": archive.qd_score(),
        "best_design_id": ids[&best.genome.sha256()], "best_raw_score_100": best.record.score.raw_score_100});
    fs::write(format!("{out}/pool.json"), serde_json::to_string_pretty(&pool).unwrap() + "\n").unwrap_or_else(|e| fail(e));
    println!("g5-continue merge: {n_rows} rows -> {} niches, best {:.4} -> {out}", archive.elites.len(), best.record.score.raw_score_100);
}

fn main() {
    let a = Args(std::env::args().collect());
    match a.0.get(1).map(String::as_str) {
        Some("run") => run(&a),
        Some("reeval") => reeval(&a),
        Some("merge") => merge(&a),
        _ => fail("usage: g5-continue run|reeval|merge ... (see the module documentation)"),
    }
}
