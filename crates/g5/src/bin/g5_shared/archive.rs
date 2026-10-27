//! Archive rows shared by `g5-continue` and `g5-release` (v6.0.0): read an archive directory, rebuild each record from
//! its row with a bit-exact re-score, write rows as `g5-evolve` does, and merge archives (the best record per niche,
//! inserted in order). The receipt recomputes every merge with this code. Errors are returned as messages; each bin
//! reports them its own way.
#![allow(dead_code)]
use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs;

use jarambit_g5::ergolight::{ErgolightInputs, ErgolightScore};
use jarambit_g5::g5_genome::G5Genome;
use jarambit_g5::g5_namespace::validate_g5_design_id;
use jarambit_g5::g5_qd::{g5_decoder_sha256, measure_point_planform, CvtMapElitesArchive, EvalTier, G5Descriptors, G5EvalRecord};
use jarambit_g5::g5_sdf::{build_blade_profile_2d, AmConnectivityStats};
use jarambit_g5::loads::G5Loads;
use jarambit_g5::objective::{score_v2, ObjectiveInputs};
use serde_json::{json, Value};

/// The W_ERGOLIGHT_v2 score of a record's measurements (the same call as `g5_qd::score_record`).
pub fn rescore(g: &G5Genome, inputs: &ErgolightInputs, loads: &G5Loads, am: &AmConnectivityStats) -> ErgolightScore {
    let blade = build_blade_profile_2d(&g.decode());
    let point = measure_point_planform(&blade);
    score_v2(&ObjectiveInputs { inputs, loads, point: &point, am, blade: &blade })
}

/// A record rebuilt from its archive row; refused unless it re-scores bit for bit under this binary's decoder.
pub fn record_from_row(id: &str, g: &G5Genome, row: &Value) -> Result<G5EvalRecord, String> {
    let take = |k: &str| row.get(k).cloned().ok_or_else(|| format!("{id}: row has no {k}"));
    let hash = g5_decoder_sha256();
    if row["decoder_sha256"] != json!(hash) {
        return Err(format!("{id}: row from decoder {}, this binary is {hash}", row["decoder_sha256"]));
    }
    if row["genome_sha256"] != json!(g.sha256()) || !id.ends_with(&g.sha256()[..8]) {
        return Err(format!("{id}: genome does not re-hash to its row and id"));
    }
    let de = |k: &str| -> Result<Value, String> { take(k) };
    let inputs: ErgolightInputs = serde_json::from_value(de("inputs")?).map_err(|e| format!("{id} inputs: {e}"))?;
    let loads: G5Loads = serde_json::from_value(de("loads")?).map_err(|e| format!("{id} loads: {e}"))?;
    let am: AmConnectivityStats = serde_json::from_value(de("am_connectivity")?).map_err(|e| format!("{id} am: {e}"))?;
    let descriptors: G5Descriptors = serde_json::from_value(de("descriptors")?).map_err(|e| format!("{id} descriptors: {e}"))?;
    let tier: EvalTier = serde_json::from_value(de("tier")?).map_err(|e| format!("{id} tier: {e}"))?;
    let score = rescore(g, &inputs, &loads, &am);
    let stored = row["raw_score_100"].as_f64().ok_or_else(|| format!("{id}: raw_score_100"))?;
    if score.raw_score_100.to_bits() != stored.to_bits() {
        return Err(format!("{id}: re-score {} != stored {stored}", score.raw_score_100));
    }
    Ok(G5EvalRecord {
        schema: de("schema")?.as_str().ok_or_else(|| format!("{id}: schema"))?.to_string(),
        decoder_sha256: hash,
        genome_sha256: g.sha256(),
        tier,
        inputs,
        score,
        descriptors,
        am_connectivity: am,
        loads,
    })
}

/// The (design id, genome, archive row) triples of one archive directory, in file order, genomes verified.
pub fn read_dir_rows(dir: &str) -> Result<Vec<(String, G5Genome, Value)>, String> {
    let csv = fs::read_to_string(format!("{dir}/genomes_g5.csv")).map_err(|e| format!("{dir}/genomes_g5.csv: {e}"))?;
    let mut genomes: HashMap<String, G5Genome> = HashMap::new();
    for line in csv.lines().skip(1).filter(|l| !l.trim().is_empty()) {
        let (id, row) = line.split_once(',').ok_or_else(|| format!("{dir}: bad genome row"))?;
        validate_g5_design_id(id).map_err(|e| format!("{id}: {e}"))?;
        let g = G5Genome::from_csv_row(row).map_err(|e| format!("{id}: {e}"))?;
        genomes.insert(id.to_string(), g);
    }
    let jsonl = fs::read_to_string(format!("{dir}/archive.jsonl")).map_err(|e| format!("{dir}/archive.jsonl: {e}"))?;
    let mut rows = Vec::new();
    for l in jsonl.lines().filter(|l| !l.trim().is_empty()) {
        let v: Value = serde_json::from_str(l).map_err(|e| format!("{dir}: {e}"))?;
        let id = v["design_id"].as_str().ok_or_else(|| format!("{dir}: row without design_id"))?.to_string();
        let g = genomes.remove(&id).ok_or_else(|| format!("{dir}: {id} has no genome row"))?;
        rows.push((id, g, v));
    }
    if !genomes.is_empty() {
        return Err(format!("{dir}: {} genome rows without an archive row", genomes.len()));
    }
    Ok(rows)
}

pub fn row_json(id: &str, niche: usize, g: &G5Genome, rec: &G5EvalRecord) -> Value {
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

/// The bytes of genomes_g5.csv and archive.jsonl for rows, as `g5-evolve` writes them.
pub fn rows_text(rows: &[(String, usize, &G5Genome, &G5EvalRecord)]) -> (String, String) {
    let mut csv = format!("design_id,{}\n", G5Genome::csv_header());
    let mut jsonl = String::new();
    for (id, niche, g, rec) in rows {
        let _ = writeln!(csv, "{id},{}", g.to_csv_row());
        let _ = writeln!(jsonl, "{}", row_json(id, *niche, g, rec));
    }
    (csv, jsonl)
}

/// A merge: the archive, the first design id seen for each genome, and the rows read per source directory.
pub struct Merged {
    pub archive: CvtMapElitesArchive,
    pub ids: HashMap<String, String>,
    pub rows_per_source: Vec<(String, usize)>,
}

impl Merged {
    pub fn rows_read(&self) -> usize {
        self.rows_per_source.iter().map(|s| s.1).sum()
    }

    /// The merged elites as named rows (niche order), ready for `rows_text`.
    pub fn named(&self) -> Vec<(String, usize, &G5Genome, &G5EvalRecord)> {
        self.archive.elites.iter().map(|(&nid, e)| (self.ids[&e.genome.sha256()].clone(), nid, &e.genome, &e.record)).collect()
    }
}

/// Merge archive directories in order: every row re-scored bit for bit, then inserted (the best record per niche wins;
/// on a tie the earlier one stays).
pub fn merge_dirs(dirs: &[&str]) -> Result<Merged, String> {
    let mut archive = CvtMapElitesArchive::new(1024);
    let mut ids: HashMap<String, String> = HashMap::new();
    let mut rows_per_source = Vec::new();
    for dir in dirs {
        let rows = read_dir_rows(dir)?;
        rows_per_source.push((dir.to_string(), rows.len()));
        for (id, g, row) in rows {
            let rec = record_from_row(&id, &g, &row)?;
            ids.entry(g.sha256()).or_insert(id);
            archive.insert(g, rec);
        }
    }
    archive.verify_integrity().map_err(|e| format!("archive integrity: {e}"))?;
    if archive.elites.is_empty() {
        return Err("empty merge".into());
    }
    Ok(Merged { archive, ids, rows_per_source })
}
