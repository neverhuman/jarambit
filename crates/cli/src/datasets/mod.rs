//! Regression dataset factory: micro-records written during evolution, deterministic tables
//! (final column = target), and CatBoost k-fold lift screening. See `datasets/regression/README.md`.
//!
//! The pure-std modules below are also compiled standalone by CI (`datasets/standalone.rs`), so the
//! CatBoost job needs no workspace build. `featurize` and `recorder` need the CAD kernel.
pub mod csvio;
pub mod cv;
pub mod engine;
pub mod gfeat;
pub mod gitsync;
pub mod harvest;
pub mod json;
pub mod materialize;
pub mod recipes;
pub mod records;
pub mod sources;
pub mod stats;

pub mod featurize;
pub mod recorder;

use jarambit_crescent::{certification, genome::Genome, pheno, qd};
use json::Json;
use records::{Row, HASH_COL};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// `jarambit dataset <command> ...`
pub fn run_dataset_cli(args: &[String]) -> i32 {
    let Some(cmd) = args.first() else {
        println!("{}", engine::USAGE);
        return 2;
    };
    if cmd == "-h" || cmd == "--help" {
        println!("{}", engine::USAGE);
        return 0;
    }
    if let Some(code) = engine::run_pure(cmd, &args[1..]) {
        return code;
    }
    let a = engine::Args(&args[1..]);
    let root = engine::repo_root(&a);
    let result = match cmd.as_str() {
        "featurize" => cmd_featurize(&a),
        "refeaturize" => cmd_refeaturize(&root, &a),
        "rescreen" => cmd_rescreen(&root, &a),
        "add-physical" => cmd_add_physical(&root, &a),
        other => Err(format!("unknown command {other:?}\n\n{}", engine::USAGE)),
    };
    match result {
        Ok(code) => code,
        Err(e) => {
            eprintln!("dataset {cmd}: {e}");
            2
        }
    }
}

fn resolve(root: &Path, p: &str) -> PathBuf {
    let p = PathBuf::from(p);
    if p.is_absolute() {
        p
    } else {
        root.join(p)
    }
}

/// Genomes from any CSV whose columns include the genome header (`cp00` .. `material`), e.g. a
/// `*_genome.csv`, `bench/fixed_1k/genomes.csv`, or a gen-800 `all_evals.csv`.
pub fn read_genomes(path: &Path) -> Result<Vec<Genome>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut lines = text.lines();
    let header = csvio::split(lines.next().ok_or("empty genome CSV")?);
    let start = header.iter().position(|h| h == "cp00").ok_or("genome CSV lacks a cp00 column")?;
    let mut out = Vec::new();
    for line in lines.filter(|l| !l.trim().is_empty()) {
        let cells = csvio::split(line);
        let fields: Vec<&str> = cells[start.min(cells.len())..].iter().map(String::as_str).collect();
        // exact stored genes: never re-clamp, the genome hash must match what was evaluated
        if let Some(g) = Genome::from_csv(&fields) {
            out.push(g);
        }
    }
    Ok(out)
}

fn cmd_featurize(a: &engine::Args) -> Result<i32, String> {
    let path = a.get("--genome").ok_or("--genome FILE required")?;
    for g in read_genomes(Path::new(&path))? {
        let t0 = std::time::Instant::now();
        let p = pheno::build(&g);
        let t_build = t0.elapsed().as_secs_f64() * 1e3;
        let aes = jarambit_crescent::aesthetic::evaluate(&g, &p);
        let rejects = jarambit_crescent::gates::check(&g, &p);
        let desc = qd::descriptor(&g, &p);
        let t_eval = t0.elapsed().as_secs_f64() * 1e3 - t_build;
        let mut row = Row::new();
        featurize::genome_cells(&g, &mut row);
        row.text("meta.feature_schema", featurize::FEATURE_SCHEMA);
        featurize::design_cells(&g, &p, &aes, &rejects, &desc, &mut row);
        let ms = t0.elapsed().as_secs_f64() * 1e3;
        eprintln!("featurize: phenotype {t_build:.1} ms, aesthetic+gates+descriptor {t_eval:.1} ms, design features {:.1} ms", ms - t_build - t_eval);
        let doc = Json::Obj(row.cells.iter().map(|(k, v)| (k.clone(), if k.starts_with("meta.") { Json::from(v.as_str()) } else { Json::from(csvio::num(v)) })).collect());
        println!("{}", doc.pretty());
        eprintln!("featurize: {} columns in {ms:.1} ms", row.cells.len());
    }
    Ok(0)
}

/// Add CAD features to base rows that only carry genes (e.g. harvested evidence), as attachments.
fn cmd_refeaturize(root: &Path, a: &engine::Args) -> Result<i32, String> {
    let records_dir = resolve(root, &a.get("--records").unwrap_or_else(|| engine::DEFAULT_RECORDS.into()));
    let threads = a.num("--threads", jarambit_monte_carlo::max_threads())?.max(1);
    let (frame, _) = records::load_records(&records_dir)?;
    // genomes that only carry genes (no CAD features yet)
    let mut gene_only: Vec<(String, Genome)> = Vec::new();
    for r in 0..frame.rows.len() {
        if frame.get(r, "c.mass_g").is_finite() {
            continue;
        }
        let Some(genes) = frame.genes(r) else { continue };
        let fields: Vec<String> = genes[..genes.len() - 1]
            .iter()
            .map(|v| format!("{v}"))
            .chain(std::iter::once(if genes[genes.len() - 1] == 1.0 { "Ti-3Al-2.5V".to_string() } else { "Ti-6Al-4V".to_string() }))
            .collect();
        let refs: Vec<&str> = fields.iter().map(String::as_str).collect();
        if let Some(g) = Genome::from_csv(&refs) {
            gene_only.push((frame.keys[r].clone(), g));
        }
    }
    let th = threads.min(gene_only.len()).max(1);
    let gene_only_ref = &gene_only;
    let mut rows: Vec<(usize, Row)> = std::thread::scope(|s| {
        let hs: Vec<_> = (0..th)
            .map(|t| {
                s.spawn(move || {
                    let mut out = Vec::new();
                    for (i, (key, g)) in gene_only_ref.iter().enumerate().filter(|(i, _)| i % th == t) {
                        let (full, _) = featurize::record_from_genome(g);
                        if full.get(HASH_COL) != Some(key.as_str()) {
                            eprintln!("refeaturize: genome hash changed on reload for {key}; skipped");
                            continue;
                        }
                        let mut att = Row::new();
                        att.text(HASH_COL, key).text("meta.feature_schema", featurize::FEATURE_SCHEMA);
                        for (k, v) in &full.cells {
                            if ["c.", "a.", "gate.", "desc."].iter().any(|p| k.starts_with(p)) {
                                att.cells.push((k.clone(), v.clone()));
                            }
                        }
                        out.push((i, att));
                    }
                    out
                })
            })
            .collect();
        hs.into_iter().flat_map(|h| h.join().expect("refeaturize worker")).collect()
    });
    rows.sort_by_key(|(i, _)| *i);
    let rows: Vec<Row> = rows.into_iter().map(|(_, r)| r).collect();
    if rows.is_empty() {
        println!("refeaturize: every base row already has CAD features");
        return Ok(0);
    }
    let digest = stats::fnv1a(rows.iter().filter_map(|r| r.get(HASH_COL)).collect::<Vec<_>>().join(",").as_bytes());
    let out = records_dir.join("refeaturized").join(format!("{}-{digest:016x}.csv", featurize::FEATURE_SCHEMA.replace('.', "-")));
    records::write_shard(&out, &rows)?;
    println!("refeaturize: {} genomes -> {}", rows.len(), out.display());
    Ok(0)
}

/// Generated genomes enter the RS-karambit-v2 regime exactly like auto-evolve seeds do.
fn rs_v2_ready(mut g: Genome) -> Genome {
    if !g.rs_v2() {
        g.v[jarambit_crescent::genome::EDGE_TH] = 0.18;
        g.clamp();
    }
    g
}

/// Run the Stage-1 FEM screen on genomes (file, mutation neighbours of a file, or random in-envelope)
/// and record every result.
fn cmd_rescreen(root: &Path, a: &engine::Args) -> Result<i32, String> {
    let area = a.num("--area", 2.0f64)?;
    let threads = a.num("--threads", jarambit_monte_carlo::max_threads())?.max(1);
    let seed = a.num("--seed", 1u64)?;
    let mut genomes = match a.get("--genomes") {
        Some(p) => read_genomes(&resolve(root, &p))?,
        None => Vec::new(),
    };
    let n_random = a.num("--random", 0usize)?;
    let n_neighbors = a.num("--neighbors", 0usize)?;
    let sigma = a.num("--sigma", 0.05f64)?;
    let mut rng = jarambit_genotype::Rng(seed);
    // design-of-experiments around the supplied genomes (same mutation operators as the GA)
    // lineage for grouped CV: file genomes are their own parents, neighbours carry their parent's hash
    let mut lineage: Vec<String> = genomes.iter().map(|g| featurize::genome_hash(g).unwrap_or_else(String::new)).collect();
    let parents = genomes.clone();
    for parent in &parents {
        let parent_hash = featurize::genome_hash(parent).unwrap_or_else(String::new);
        for _ in 0..n_neighbors {
            genomes.push(rs_v2_ready(parent.mutate(&mut rng, sigma)));
            lineage.push(parent_hash.clone());
        }
    }
    for _ in 0..n_random {
        genomes.push(rs_v2_ready(Genome::random_in_envelope(&mut rng)));
        lineage.push(String::new());
    }
    if genomes.is_empty() {
        return Err("--genomes FILE and/or --random N required".into());
    }
    let tag = a.get("--tag").unwrap_or_else(|| "rescreen".into());
    let run_id = format!("{tag}-{}-{}-s{seed}", recorder::utc_stamp(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)), recorder::host_name());
    let out_dir = resolve(root, &a.get("--out").unwrap_or_else(|| engine::DEFAULT_RECORDS.into())).join("rescreen").join(&run_id);
    let chunk = a.num("--chunk", 256usize)?.max(1);
    let mut written = 0;
    for (ci, part) in genomes.chunks(chunk).enumerate() {
        let th = threads.min(part.len()).max(1);
        let run_ref = run_id.as_str();
        let lineage_ref = &lineage[ci * chunk..(ci * chunk + part.len()).min(lineage.len())];
        let mut rows: Vec<(usize, Row)> = std::thread::scope(|s| {
            let hs: Vec<_> = (0..th)
                .map(|t| {
                    s.spawn(move || {
                        part.iter()
                            .enumerate()
                            .filter(|(i, _)| i % th == t)
                            .map(|(i, g)| {
                                let r = qd::evaluate(g, i, 0, area);
                                let p = pheno::build(g);
                                let mut row = featurize::record_from_eval(g, &p, &r);
                                row.text("meta.lane", "rescreen").text("meta.source", "rescreen").text("meta.run_id", run_ref);
                                row.text("meta.idx", &(ci * chunk + i).to_string()).text("meta.fem_area_mm2", &area.to_string());
                                row.text("meta.certificate_identity", &certification::identity(g));
                                if let Some(parent) = lineage_ref.get(i).filter(|p| !p.is_empty()) {
                                    row.text("meta.parent_hash", parent);
                                }
                                (i, row)
                            })
                            .collect::<Vec<_>>()
                    })
                })
                .collect();
            hs.into_iter().flat_map(|h| h.join().expect("rescreen worker")).collect()
        });
        rows.sort_by_key(|(i, _)| *i);
        let rows: Vec<Row> = rows.into_iter().map(|(_, r)| r).collect();
        written += records::write_shard(&out_dir.join(format!("c{ci:04}.s1.csv")), &rows)?;
        eprintln!("rescreen: {written}/{} genomes", genomes.len());
    }
    // Committed manifests name the genome file relative to the repository (never a machine path)
    // and list the seed genome hashes, so the run is reproducible from the records themselves.
    let genomes_file = a.get("--genomes").map(|p| {
        let full = resolve(root, &p);
        match full.strip_prefix(root) {
            Ok(rel) => rel.to_string_lossy().to_string(),
            Err(_) => full.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or(p),
        }
    });
    let seed_hashes: Vec<Json> = lineage[..parents.len()].iter().map(Json::from).collect();
    let manifest = Json::obj()
        .with("schema", "jarambit.regression.run.v1")
        .with("run_id", run_id.as_str())
        .with("lane", "rescreen")
        .with("fem_area_mm2", area)
        .with("seed", seed)
        .with("random", n_random)
        .with("neighbors", n_neighbors)
        .with("sigma", sigma)
        .with("genomes_file", genomes_file)
        .with("seed_genome_hashes", seed_hashes)
        .with("git_head", gitsync::head(root))
        .with("feature_schema", featurize::FEATURE_SCHEMA);
    std::fs::write(out_dir.join("run.json"), manifest.pretty()).map_err(|e| e.to_string())?;
    println!("rescreen: {written} records -> {}", out_dir.display());
    if a.flag("--commit") {
        let rel = out_dir.strip_prefix(root).unwrap_or(&out_dir).to_string_lossy().to_string();
        if let Some(sha) = gitsync::commit_paths(root, &[rel], &format!("data(regression): {run_id} (+{written} s1 records)"))? {
            println!("rescreen: committed {sha}");
        }
    }
    Ok(0)
}

/// Import physical test results. The CSV identifies each specimen's design by one of
/// `genome_csv` (path to a genome CSV), `genome_hash`, or `certificate_identity`; every other
/// numeric column becomes `phys.<column>`. Text columns are kept as `meta.<column>`.
fn cmd_add_physical(root: &Path, a: &engine::Args) -> Result<i32, String> {
    let csv = resolve(root, &a.get("--csv").ok_or("--csv FILE required")?);
    let campaign = a.get("--campaign").ok_or("--campaign NAME required")?;
    let records_dir = resolve(root, &a.get("--records").unwrap_or_else(|| engine::DEFAULT_RECORDS.into()));
    let shard = records::read_shard(&csv)?;
    let col = |n: &str| shard.columns.iter().position(|c| c == n);
    let (gcsv, ghash, gid) = (col("genome_csv"), col("genome_hash"), col("certificate_identity"));
    if gcsv.is_none() && ghash.is_none() && gid.is_none() {
        return Err("physical CSV needs a genome_csv, genome_hash or certificate_identity column".into());
    }
    let mut id_to_hash: HashMap<String, String> = HashMap::new();
    if gid.is_some() && records_dir.is_dir() {
        let (frame, _) = records::load_records(&records_dir)?;
        if let Some(m) = frame.meta_columns.iter().position(|c| c == "meta.certificate_identity") {
            for (r, meta) in frame.meta.iter().enumerate() {
                if !meta[m].is_empty() {
                    id_to_hash.insert(meta[m].clone(), frame.keys[r].clone());
                }
            }
        }
    }
    let mut out_rows = Vec::new();
    let mut unresolved = 0;
    for cells in &shard.rows {
        let mut row = Row::new();
        if let Some(i) = gcsv.filter(|&i| !cells[i].trim().is_empty()) {
            let gs = read_genomes(&resolve(root, cells[i].trim()))?;
            let g = gs.first().ok_or_else(|| format!("{}: no genome", cells[i]))?;
            row = featurize::record_from_genome(g).0;
            row.text("meta.certificate_identity", &certification::identity(g));
        } else if let Some(h) = ghash.map(|i| cells[i].trim().to_string()).filter(|h| !h.is_empty()) {
            row.text(HASH_COL, &h);
        } else if let Some(h) = gid.and_then(|i| id_to_hash.get(cells[i].trim())) {
            row.text(HASH_COL, h);
        } else {
            unresolved += 1;
            continue;
        }
        row.text("meta.lane", "physical").text("meta.campaign", &campaign);
        for (j, name) in shard.columns.iter().enumerate() {
            if Some(j) == gcsv || Some(j) == ghash || Some(j) == gid {
                continue;
            }
            let safe: String = name.chars().map(|c| if c.is_ascii_alphanumeric() || c == '_' { c.to_ascii_lowercase() } else { '_' }).collect();
            if csvio::is_text(&cells[j]) {
                row.text(&format!("meta.{safe}"), &cells[j]);
            } else {
                row.num(&format!("phys.{safe}"), csvio::num(&cells[j]));
            }
        }
        out_rows.push(row);
    }
    if out_rows.is_empty() {
        return Err(format!("no rows resolved to a design ({unresolved} unresolved)"));
    }
    let stem = csv.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| "results".into());
    let dir = records_dir.join("physical").join(&campaign);
    // rows with a genome are base rows; rows keyed only by hash are attachments (separate shards)
    let (base, attach): (Vec<Row>, Vec<Row>) = out_rows.into_iter().partition(|r| r.genes().is_some());
    let total = base.len() + attach.len();
    if !base.is_empty() {
        records::write_shard(&dir.join(format!("{stem}.csv")), &base)?;
    }
    if !attach.is_empty() {
        records::write_shard(&dir.join(format!("{stem}.attach.csv")), &attach)?;
    }
    println!("add-physical: {total} specimens ({unresolved} unresolved) -> {}", dir.display());
    Ok(if unresolved > 0 { 1 } else { 0 })
}
