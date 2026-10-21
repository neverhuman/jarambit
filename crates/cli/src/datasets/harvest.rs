//! Convert retained auto-evolve outputs into record shards.
//!
//! Inputs (any directory tree): `round_NN_births.json` (genome rows + certificate identity),
//! `round_NN_screen.json` (Stage-1 screen scalars) and `leaderboard.json` (Stage-2 assessments).
//! Every birth becomes a base row (`g.*` + whatever screen scalars exist); every assessed design
//! becomes an `s2.*` attachment keyed by the genome hash of its birth. The same column names are
//! produced by the live recorder, so harvested and live rows pool into the same tables.
use super::csvio;
use super::gfeat;
use super::json::{self, Json};
use super::records::{self, Row, HASH_COL};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Stage-1 screen scalars as they appear in `round_NN_screen.json` -> record columns.
pub const SCREEN_KEYS: [(&str, &str); 6] = [
    ("mass_g", "s1.mass_g"),
    ("seq_ratio", "s1.seq_ratio"),
    ("fat_sf", "s1.fat_sf"),
    ("ring_bearing_w_mm", "s1.ring_bearing_w_mm"),
    ("screen_deficit", "s1.screen_deficit"),
    ("ultralight_weighted_fitness", "s1.ultralight_weighted_fitness"),
];

/// Leaderboard design keys that are provenance, not measurements.
const DESIGN_SKIP: [&str; 6] = ["label", "family", "certificate_identity", "genome_csv", "scorecard_md", "qualified"];

/// Stage-2 attachment row from one leaderboard design (or the live equivalent).
pub fn s2_row(design: &Json, genome_hash: &str) -> Row {
    let mut r = Row::new();
    r.text(HASH_COL, genome_hash);
    if let Some(id) = design.get("certificate_identity").and_then(Json::as_str) {
        r.text("meta.certificate_identity", id);
    }
    if let Some(l) = design.get("label").and_then(Json::as_str) {
        r.text("meta.label", l);
    }
    if let Some(items) = design.as_obj() {
        for (k, v) in items {
            if DESIGN_SKIP.contains(&k.as_str()) {
                continue;
            }
            if let Some(x) = v.as_f64() {
                r.num(&format!("s2.{k}"), x);
            }
        }
    }
    r
}

/// Base row from a genome CSV header + row (exact genes) with provenance.
pub fn genome_row(header: &[String], row: &[String]) -> Option<Row> {
    let mut r = Row::new();
    for (h, v) in header.iter().zip(row) {
        let col = gfeat::record_column_for_gene_header(h)?;
        if col == gfeat::MATERIAL_COL {
            r.exact(&col, gfeat::material_flag(v));
        } else {
            r.exact(&col, csvio::num(v));
        }
    }
    r.stamp_hash();
    r.get(HASH_COL)?;
    Some(r)
}

#[derive(Default, Debug)]
pub struct HarvestReport {
    pub runs: usize,
    pub births: usize,
    pub screened: usize,
    pub assessed: usize,
    pub unmatched_assessments: usize,
    pub shards: Vec<PathBuf>,
}

fn find_run_dirs(root: &Path, out: &mut Vec<PathBuf>) {
    let Ok(it) = std::fs::read_dir(root) else { return };
    let mut has_births = false;
    let mut subdirs = Vec::new();
    for e in it.flatten() {
        let p = e.path();
        let n = e.file_name().to_string_lossy().to_string();
        if p.is_dir() {
            if n != "by-content" {
                subdirs.push(p);
            }
        } else if n.starts_with("round_") && n.ends_with("_births.json") {
            has_births = true;
        }
    }
    if has_births {
        out.push(root.to_path_buf());
    }
    subdirs.sort();
    for d in subdirs {
        find_run_dirs(&d, out);
    }
}

fn read_json(p: &Path) -> Result<Json, String> {
    json::parse(&std::fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))?).map_err(|e| format!("{}: {e}", p.display()))
}

/// Harvest every auto-evolve run directory below `evidence` into `<out>/<run>/r00.s1.csv` etc.
/// `repo_root` is used only to name runs relative to the repository.
pub fn harvest_waves(evidence: &Path, repo_root: &Path, out: &Path) -> Result<HarvestReport, String> {
    let mut runs = Vec::new();
    find_run_dirs(evidence, &mut runs);
    let mut rep = HarvestReport::default();
    for run in runs {
        rep.runs += 1;
        let rel = run.strip_prefix(repo_root).unwrap_or(&run).to_string_lossy().replace(['/', '\\'], "__");
        let run_id = rel.trim_start_matches("evidence__").to_string();
        let mut id_to_hash: HashMap<String, String> = HashMap::new();
        let mut rounds: Vec<PathBuf> = std::fs::read_dir(&run)
            .map_err(|e| e.to_string())?
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().ends_with("_births.json")))
            .collect();
        rounds.sort();
        for births_path in rounds {
            let stem = births_path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(String::new);
            let round = stem.trim_start_matches("round_").trim_end_matches("_births.json").to_string();
            let births = read_json(&births_path)?;
            let header: Vec<String> = births.get("genome_header").and_then(Json::as_str).map(csvio::split).ok_or("births: genome_header missing")?;
            let screen_path = run.join(format!("round_{round}_screen.json"));
            let mut screen: HashMap<String, Json> = HashMap::new();
            if screen_path.is_file() {
                for s in read_json(&screen_path)?.as_arr().unwrap_or(&[]) {
                    if let Some(id) = s.get("certificate_identity").and_then(Json::as_str) {
                        screen.insert(id.to_string(), s.clone());
                    }
                }
            }
            let mut rows = Vec::new();
            for b in births.get("genomes").and_then(Json::as_arr).unwrap_or(&[]) {
                let Some(row) = b.get("row").and_then(Json::as_str) else { continue };
                let Some(mut r) = genome_row(&header, &csvio::split(row)) else { continue };
                rep.births += 1;
                let id = b.get("certificate_identity").and_then(Json::as_str).unwrap_or("").to_string();
                r.text("meta.lane", "auto_evolve").text("meta.source", "harvest_waves").text("meta.run_id", &run_id);
                r.text("meta.round", &round).text("meta.certificate_identity", &id);
                if let Some(f) = b.get("family").and_then(Json::as_str) {
                    r.text("meta.family", f);
                }
                if let Some(s) = screen.get(&id) {
                    rep.screened += 1;
                    r.text("meta.status", "screened");
                    if let Some(l) = s.get("label").and_then(Json::as_str) {
                        r.text("meta.label", l);
                    }
                    for (k, col) in SCREEN_KEYS {
                        if let Some(x) = s.get(k).and_then(Json::as_f64) {
                            r.num(col, x);
                        }
                    }
                } else {
                    r.text("meta.status", "not_screened");
                }
                if let Some(h) = r.get(HASH_COL) {
                    id_to_hash.insert(id.clone(), h.to_string());
                }
                rows.push(r);
            }
            if !rows.is_empty() {
                let p = out.join(&run_id).join(format!("r{round}.s1.csv"));
                records::write_shard(&p, &rows)?;
                rep.shards.push(p);
            }
        }
        let lb = run.join("leaderboard.json");
        if lb.is_file() {
            let doc = read_json(&lb)?;
            let mut rows = Vec::new();
            for d in doc.get("designs").and_then(Json::as_arr).unwrap_or(&[]) {
                let id = d.get("certificate_identity").and_then(Json::as_str).unwrap_or("");
                match id_to_hash.get(id) {
                    Some(h) => {
                        rep.assessed += 1;
                        rows.push(s2_row(d, h));
                    }
                    None => rep.unmatched_assessments += 1,
                }
            }
            if !rows.is_empty() {
                let p = out.join(&run_id).join("leaderboard.s2.csv");
                records::write_shard(&p, &rows)?;
                rep.shards.push(p);
            }
        }
    }
    Ok(rep)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn harvests_births_screen_and_leaderboard_into_joinable_shards() {
        let root = std::env::temp_dir().join(format!("jarambit-harvest-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let run = root.join("evidence/waves/host/w1");
        std::fs::create_dir_all(&run).unwrap();
        let header: Vec<String> = gfeat::gene_columns()
            .iter()
            .map(|c| if c == gfeat::MATERIAL_COL { "material".to_string() } else { c.trim_start_matches("g.").to_string() })
            .collect();
        let mk = |k: f64| -> String {
            header.iter().enumerate().map(|(i, h)| if h == "material" { "Ti-6Al-4V".to_string() } else { format!("{}", k + i as f64) }).collect::<Vec<_>>().join(",")
        };
        let births = Json::obj().with("genome_header", header.join(",")).with(
            "genomes",
            Json::Arr(vec![
                Json::obj().with("family", "p10").with("row", mk(1.0)).with("certificate_identity", "id-a"),
                Json::obj().with("family", "p14").with("row", mk(2.0)).with("certificate_identity", "id-b"),
            ]),
        );
        std::fs::write(run.join("round_00_births.json"), births.pretty()).unwrap();
        let screen = Json::Arr(vec![Json::obj().with("certificate_identity", "id-a").with("label", "w1_r01_p10_c01").with("mass_g", 90.5).with("seq_ratio", 1.1)]);
        std::fs::write(run.join("round_00_screen.json"), screen.pretty()).unwrap();
        let lb = Json::obj().with(
            "designs",
            Json::Arr(vec![
                Json::obj().with("certificate_identity", "id-a").with("label", "w1_r01_p10_c01").with("solid_tip_sf", 0.9).with("all_16_gates", false),
                Json::obj().with("certificate_identity", "id-zzz").with("solid_tip_sf", 2.0),
            ]),
        );
        std::fs::write(run.join("leaderboard.json"), lb.pretty()).unwrap();
        let out = root.join("records/harvest_waves");
        let rep = harvest_waves(&root.join("evidence"), &root, &out).unwrap();
        assert_eq!((rep.runs, rep.births, rep.screened, rep.assessed, rep.unmatched_assessments), (1, 2, 1, 1, 1));
        let (f, lr) = records::load_records(&root.join("records")).unwrap();
        assert_eq!(f.rows.len(), 2);
        assert_eq!(lr.orphan_attachments, 0);
        let a = (0..2).find(|&r| f.get(r, "s1.mass_g") == 90.5).unwrap();
        assert_eq!(f.get(a, "s2.solid_tip_sf"), 0.9);
        assert_eq!(f.get(a, "s2.all_16_gates"), 0.0);
        assert!(f.get(1 - a, "s1.mass_g").is_nan());
    }
}
