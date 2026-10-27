//! Continue the G5 evolution from earlier archives (v6.0.0; frozen decoder G5.0). Hash-neutral: only public library
//! APIs are used, and no decoder-hashed file changes.
//!
//!   g5-continue run    --out DIR [--pool DIR] [--init N] --evals N --seed S --host H --island I --commit C
//!                      [--h 0.4] [--tier1-top 0.4] [--tier2-top 0] [--log-every 25] [--stl]
//!   g5-continue reeval --out DIR --genomes CSV[,CSV...] [--h 0.4] [--threads 4] [--shard k/n]
//!   g5-continue merge  --out DIR --from DIR[,DIR...]
//!   g5-continue screen --out DIR --pool DIR --n 200 --seed S [--h 0.4] [--threads 2]
//!
//! `run` is one island. Without `--pool` it is exactly `g5-evolve` (Sobol start of `--init`, then the same loop);
//! with `--pool` it starts from that archive (its records inserted as they are, after a bit-exact re-score) and
//! evolves `--evals` more. `reeval` evaluates genomes at Tier 1 on this platform (the epoch-0 pool from the v5
//! elites); `merge` keeps the best record per niche over several archives (the pool of the next epoch). Every record
//! read from disk is rebuilt from its row and re-scored with W_ERGOLIGHT_v2; it must reproduce the row's score bit
//! for bit and carry this binary's decoder hash, and every genome must re-hash to its design id (fail closed).
//! Outputs are the `g5-evolve` files (archive.jsonl, genomes_g5.csv, history.jsonl) plus launch.json or pool.json;
//! no STL unless `--stl` (the v5 islands wrote 330 MB each). `screen` measures the Tier-1 pre-screen (the GPU spike's
//! question): for `--n` offspring of the pool it records the Tier-0 and Tier-1 scores and times, and how well the Tier-0
//! rank (the screen the search uses) finds the children that Tier 1 ranks best.
use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs;

use jarambit_g5::g5_genome::{sobol_unit_genome, G5Genome, G5Rng};
use jarambit_g5::g5_namespace::{format_g5_design_id, validate_g5_design_id, G5CohortConfig};
use jarambit_g5::g5_qd::{
    evaluate_tier1_sdf_and_plate, g5_decoder_sha256, splitmix64, CvtMapElitesArchive, EvalTier, G5EvalRecord,
    G5HistoryPoint, RankPromotion,
};
use jarambit_g5::g5_sdf::extract_watertight_mesh_fail_closed;
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

#[path = "g5_shared/archive.rs"]
mod archive;

fn record_from_row(id: &str, g: &G5Genome, row: &Value) -> G5EvalRecord {
    archive::record_from_row(id, g, row).unwrap_or_else(|e| fail(e))
}

fn read_dir_rows(dir: &str) -> Vec<(String, G5Genome, Value)> {
    archive::read_dir_rows(dir).unwrap_or_else(|e| fail(e))
}

/// Write rows as `g5-evolve` does (genomes_g5.csv + archive.jsonl).
fn write_rows(dir: &str, rows: &[(String, usize, &G5Genome, &G5EvalRecord)]) {
    let (csv, jsonl) = archive::rows_text(rows);
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
    let from = a.req("--from");
    let dirs: Vec<&str> = from.split(',').collect();
    let m = archive::merge_dirs(&dirs).unwrap_or_else(|e| fail(e));
    write_rows(&out, &m.named());
    let sources: Vec<Value> = m
        .rows_per_source
        .iter()
        .map(|(dir, n)| json!({"dir": dir, "rows": n, "archive.jsonl": sha_file(&format!("{dir}/archive.jsonl"))}))
        .collect();
    let best = m.archive.best_elite().unwrap_or_else(|| fail("empty merge"));
    let pool = json!({"tool": "g5-continue merge", "decoder_sha256": g5_decoder_sha256(), "sources": sources,
        "rows_read": m.rows_read(), "niches": m.archive.elites.len(), "qd_score": m.archive.qd_score(),
        "best_design_id": m.ids[&best.genome.sha256()], "best_raw_score_100": best.record.score.raw_score_100});
    fs::write(format!("{out}/pool.json"), serde_json::to_string_pretty(&pool).unwrap() + "\n").unwrap_or_else(|e| fail(e));
    println!("g5-continue merge: {} rows -> {} niches, best {:.4} -> {out}", m.rows_read(), m.archive.elites.len(),
             best.record.score.raw_score_100);
}

/// Spearman rank correlation (average ranks for ties).
fn spearman(x: &[f64], y: &[f64]) -> f64 {
    let rank = |v: &[f64]| -> Vec<f64> {
        let mut idx: Vec<usize> = (0..v.len()).collect();
        idx.sort_by(|&a, &b| v[a].total_cmp(&v[b]));
        let mut r = vec![0.0; v.len()];
        let mut i = 0;
        while i < idx.len() {
            let mut j = i;
            while j + 1 < idx.len() && v[idx[j + 1]] == v[idx[i]] {
                j += 1;
            }
            for &k in &idx[i..=j] {
                r[k] = 0.5 * (i + j) as f64;
            }
            i = j + 1;
        }
        r
    };
    let (rx, ry) = (rank(x), rank(y));
    let n = x.len() as f64;
    let (mx, my) = (rx.iter().sum::<f64>() / n, ry.iter().sum::<f64>() / n);
    let cov: f64 = rx.iter().zip(&ry).map(|(a, b)| (a - mx) * (b - my)).sum();
    let vx: f64 = rx.iter().map(|a| (a - mx).powi(2)).sum();
    let vy: f64 = ry.iter().map(|b| (b - my).powi(2)).sum();
    cov / (vx * vy).sqrt()
}

fn screen(a: &Args) {
    use jarambit_g5::g5_qd::evaluate_tier0_analytic;
    use rayon::prelude::*;
    use std::time::Instant;
    let out = a.req("--out");
    check_root(&out);
    let (n, h) = (a.n("--n", 200), a.f("--h", 0.4));
    let seed: u64 = a.get("--seed").map_or(1, |v| v.parse().unwrap_or_else(|e| fail(format!("--seed: {e}"))));
    rayon::ThreadPoolBuilder::new().num_threads(a.n("--threads", 2)).build_global().ok();
    let dir = a.req("--pool");
    let mut archive = CvtMapElitesArchive::new(1024);
    for (id, g, row) in read_dir_rows(&dir) {
        let rec = record_from_row(&id, &g, &row);
        archive.insert(g, rec);
    }
    let mut rng = G5Rng(splitmix64(seed).max(1));
    let children: Vec<G5Genome> = (0..n).filter_map(|_| archive.sample_offspring(&mut rng).map(|c| c.0)).collect();
    let rows: Vec<(f64, f64, f64, f64)> = children
        .par_iter()
        .map(|g| {
            let t = Instant::now();
            let s0 = evaluate_tier0_analytic(g).score.raw_score_100;
            let d0 = t.elapsed().as_secs_f64();
            let t = Instant::now();
            let s1 = evaluate_tier1_sdf_and_plate(g, h).score.raw_score_100;
            (s0, d0, s1, t.elapsed().as_secs_f64())
        })
        .collect();
    let mut jsonl = String::new();
    for (k, r) in rows.iter().enumerate() {
        let _ = writeln!(jsonl, "{}", json!({"child": k, "tier0_score": r.0, "tier0_s": r.1, "tier1_score": r.2, "tier1_s": r.3}));
    }
    fs::write(format!("{out}/screen.jsonl"), jsonl).unwrap_or_else(|e| fail(e));
    let (s0, s1): (Vec<f64>, Vec<f64>) = rows.iter().filter(|r| r.0.is_finite() && r.2.is_finite()).map(|r| (r.0, r.2)).unzip();
    let m = s0.len();
    // the screen the search uses: promote the top 40 % by Tier 0; how many of the Tier-1 top 10 % does it keep?
    let top_by = |v: &[f64], frac: f64| -> Vec<usize> {
        let mut idx: Vec<usize> = (0..v.len()).collect();
        idx.sort_by(|&a, &b| v[b].total_cmp(&v[a]));
        idx.truncate(((frac * v.len() as f64).ceil() as usize).max(1));
        idx
    };
    let promoted = top_by(&s0, 0.4);
    let best1 = top_by(&s1, 0.1);
    let kept = best1.iter().filter(|i| promoted.contains(i)).count();
    let mean = |f: fn(&(f64, f64, f64, f64)) -> f64| rows.iter().map(f).sum::<f64>() / rows.len() as f64;
    let (t0, t1) = (mean(|r| r.1), mean(|r| r.3));
    let summary = json!({"tool": "g5-continue screen", "pool": dir, "n": rows.len(), "finite": m, "h": h, "seed": seed,
        "spearman_tier0_tier1": spearman(&s0, &s1), "tier1_top10_kept_by_tier0_top40": kept, "tier1_top10": best1.len(),
        "mean_tier0_s": t0, "mean_tier1_s": t1, "island_s_per_eval_at_tier1_top_0.4": t0 + 0.4 * t1,
        "share_of_island_time_in_tier1": 0.4 * t1 / (t0 + 0.4 * t1)});
    fs::write(format!("{out}/screen.json"), serde_json::to_string_pretty(&summary).unwrap() + "\n").unwrap_or_else(|e| fail(e));
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
}

fn main() {
    let a = Args(std::env::args().collect());
    match a.0.get(1).map(String::as_str) {
        Some("run") => run(&a),
        Some("reeval") => reeval(&a),
        Some("merge") => merge(&a),
        Some("screen") => screen(&a),
        _ => fail("usage: g5-continue run|reeval|merge|screen ... (see the module documentation)"),
    }
}
