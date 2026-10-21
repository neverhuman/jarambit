//! Materialize regression tables from records + historical sources.
//!
//! Output (default `datasets/regression/tables/`, git-ignored, rebuilt deterministically):
//! `<target>__<featureset>.csv` (features..., target LAST), `<name>.card.json` (provenance, drops,
//! target stats) and `index.json` / `index.md` (every table plus every skipped spec and why).
use super::csvio;
use super::json::Json;
use super::recipes::{self, Recipes, Source};
use super::records::{self, Frame};
use super::sources;
use super::stats;
use std::collections::HashSet;
use std::io::Write as _;
use std::path::{Path, PathBuf};

pub struct BuildOptions {
    pub root: PathBuf,
    pub recipes: PathBuf,
    pub out: PathBuf,
    /// Only build tables whose name contains this substring.
    pub only: Option<String>,
    /// Overrides `max_rows` from the recipes when set.
    pub max_rows: Option<usize>,
}

pub struct BuildSummary {
    pub built: usize,
    pub skipped: usize,
    /// `index.json` content (also written to disk).
    pub index: Json,
}

pub fn load_frames(root: &Path, r: &Recipes, log: &mut Vec<Json>) -> Result<Vec<Frame>, String> {
    let mut frames = Vec::new();
    for s in &r.sources {
        match s {
            Source::Records(dir) => {
                let p = root.join(dir);
                if !p.is_dir() {
                    log.push(Json::obj().with("source", "records").with("dir", dir.as_str()).with("status", "absent"));
                    continue;
                }
                let (mut f, rep) = records::load_records(&p)?;
                f.add_derived();
                log.push(
                    Json::obj()
                        .with("source", "records")
                        .with("dir", dir.as_str())
                        .with("shards", rep.shards)
                        .with("genomes", f.rows.len())
                        .with("base_rows", rep.base_rows)
                        .with("attachment_rows", rep.attachment_rows)
                        .with("duplicate_genomes", rep.duplicate_genomes)
                        .with("conflicting_cells", rep.conflicting_cells)
                        .with("orphan_attachments", rep.orphan_attachments)
                        .with("torn_tails", rep.torn_tails)
                        .with("bad_rows", rep.bad_rows)
                        .with("rows_without_genome", rep.rows_without_genome),
                );
                frames.push(f);
            }
            Source::Gen800 { dir, cap } => {
                let p = root.join(dir);
                let packages = sources::gen800_packages(&p);
                if packages.is_empty() {
                    return Err(format!("gen800 source {dir}: no packages (fail closed)"));
                }
                for pkg in packages {
                    let (mut f, rep) = sources::load_gen800_package(&pkg, *cap)?;
                    f.add_derived();
                    log.push(
                        Json::obj()
                            .with("source", "gen800")
                            .with("package", rep.package.as_str())
                            .with("rows_read", rep.rows_read)
                            .with("distinct_genomes", rep.distinct_genomes_seen)
                            .with("rows_kept", rep.rows_kept)
                            .with("cap", *cap)
                            .with("bad_rows", rep.bad_rows),
                    );
                    frames.push(f);
                }
            }
        }
    }
    Ok(frames)
}

/// Lineage group of a row for grouped CV: the DOE parent, else run + GA family, else the source
/// (one gen-800 run), else the genome itself. Rows in one group are held out together.
pub fn lineage_group(frame: &Frame, row: usize) -> String {
    let meta = |name: &str| -> &str {
        frame.meta_columns.iter().position(|c| c == name).map(|i| frame.meta[row][i].as_str()).unwrap_or("")
    };
    let parent = meta("meta.parent_hash");
    if !parent.is_empty() {
        return format!("parent:{parent}");
    }
    let (run, family) = (meta("meta.run_id"), meta("meta.family"));
    if !run.is_empty() && !family.is_empty() {
        return format!("run:{run}/{family}");
    }
    let source = meta("meta.source");
    if source.starts_with("gen800/") {
        return source.to_string();
    }
    format!("genome:{}", frame.keys[row])
}

fn family_present(frame: &Frame, row: usize, fam_cols: &[usize]) -> bool {
    fam_cols.iter().any(|&c| frame.rows[row][c].is_finite())
}

/// Build one table in memory. Returns (columns, rows, keys, sources, drop notes) or a skip reason.
pub struct Table {
    pub name: String,
    pub target: String,
    pub featureset: String,
    pub features: Vec<String>,
    pub x: Vec<Vec<f64>>,
    pub y: Vec<f64>,
    pub keys: Vec<String>,
    /// Lineage group per row (`lineage_group`), written to `<name>.groups` for grouped CV.
    pub groups: Vec<String>,
    pub sources: Vec<(String, usize)>,
    pub dropped_constant: usize,
    pub dropped_missing: usize,
    pub dropped_leak_rule: Vec<String>,
    pub dropped_leak_guard: Vec<(String, f64)>,
    pub truncated_from: usize,
}

pub fn assemble(frames: &[Frame], r: &Recipes, target: &str, featureset: &str, max_rows: usize) -> Result<Table, String> {
    let fams = r.families(featureset).to_vec();
    let leak = r.leak_patterns(target);
    // feature candidates: union over contributing frames, in first-seen order
    let mut features: Vec<String> = Vec::new();
    let contributing: Vec<&Frame> = frames.iter().filter(|f| f.index.contains_key(target)).collect();
    let mut dropped_leak_rule: Vec<String> = Vec::new();
    for f in &contributing {
        for c in &f.columns {
            if c == target || !fams.iter().any(|fam| c.starts_with(fam.as_str())) || features.contains(c) {
                continue;
            }
            if leak.iter().any(|p| recipes::matches(p, c)) {
                if !dropped_leak_rule.contains(c) {
                    dropped_leak_rule.push(c.clone());
                }
                continue;
            }
            features.push(c.clone());
        }
    }
    let mut rows: Vec<(String, Vec<f64>, f64, usize, String)> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for (fi, f) in contributing.iter().enumerate() {
        let t = f.index[target];
        let fam_cols: Vec<Vec<usize>> =
            fams.iter().map(|fam| f.columns.iter().enumerate().filter(|(_, c)| c.starts_with(fam.as_str())).map(|(i, _)| i).collect()).collect();
        let feat_idx: Vec<Option<usize>> = features.iter().map(|c| f.index.get(c).copied()).collect();
        for row in 0..f.rows.len() {
            let y = f.rows[row][t];
            if !y.is_finite() || !fam_cols.iter().all(|fc| family_present(f, row, fc)) {
                continue;
            }
            if !seen.insert(f.keys[row].clone()) {
                continue;
            }
            let x: Vec<f64> = feat_idx.iter().map(|i| i.map(|i| f.rows[row][i]).unwrap_or(f64::NAN)).collect();
            rows.push((f.keys[row].clone(), x, y, fi, lineage_group(f, row)));
        }
    }
    rows.sort_by(|a, b| a.0.cmp(&b.0));
    let truncated_from = rows.len();
    if max_rows > 0 && rows.len() > max_rows {
        rows.truncate(max_rows);
    }
    let n = rows.len();
    if n < r.min_rows {
        return Err(format!("only {n} usable rows (< min_rows {})", r.min_rows));
    }
    let y: Vec<f64> = rows.iter().map(|r| r.2).collect();
    if y.iter().all(|v| *v == y[0]) {
        return Err("target is constant".into());
    }
    // drop all-missing / constant columns and target copies
    let mut keep: Vec<usize> = Vec::new();
    let (mut dropped_constant, mut dropped_missing) = (0, 0);
    let mut dropped_leak_guard: Vec<(String, f64)> = Vec::new();
    for (j, name) in features.iter().enumerate() {
        let col: Vec<f64> = rows.iter().map(|r| r.1[j]).collect();
        let finite: Vec<f64> = col.iter().cloned().filter(|v| v.is_finite()).collect();
        if finite.is_empty() {
            dropped_missing += 1;
            continue;
        }
        let has_missing = finite.len() < col.len();
        if !has_missing && finite.iter().all(|v| *v == finite[0]) {
            dropped_constant += 1;
            continue;
        }
        let (xs, ys): (Vec<f64>, Vec<f64>) = col.iter().zip(&y).filter(|(a, _)| a.is_finite()).map(|(a, b)| (*a, *b)).unzip();
        if xs.len() >= 10 {
            let rr = stats::pearson(&xs, &ys);
            if rr.is_finite() && rr.abs() >= r.leak_guard_r {
                dropped_leak_guard.push((name.clone(), rr));
                continue;
            }
        }
        keep.push(j);
    }
    if keep.is_empty() {
        return Err("no informative features".into());
    }
    let mut sources: Vec<(String, usize)> = Vec::new();
    for (fi, f) in contributing.iter().enumerate() {
        let c = rows.iter().filter(|r| r.3 == fi).count();
        if c > 0 {
            sources.push((f.source.clone(), c));
        }
    }
    Ok(Table {
        name: recipes::table_name(target, featureset),
        target: target.to_string(),
        featureset: featureset.to_string(),
        features: keep.iter().map(|&j| features[j].clone()).collect(),
        x: rows.iter().map(|r| keep.iter().map(|&j| r.1[j]).collect()).collect(),
        y,
        groups: rows.iter().map(|r| r.4.clone()).collect(),
        keys: rows.into_iter().map(|r| r.0).collect(),
        sources,
        dropped_constant,
        dropped_missing,
        dropped_leak_rule,
        dropped_leak_guard,
        truncated_from,
    })
}

pub fn write_table(dir: &Path, t: &Table) -> Result<Json, String> {
    let csv = dir.join(format!("{}.csv", t.name));
    let file = std::fs::File::create(&csv).map_err(|e| format!("{}: {e}", csv.display()))?;
    let mut w = std::io::BufWriter::new(file);
    let mut header: Vec<String> = t.features.clone();
    header.push(t.target.clone());
    let io = |e: std::io::Error| format!("{}: {e}", csv.display());
    writeln!(w, "{}", csvio::join(&header)).map_err(io)?;
    for (x, y) in t.x.iter().zip(&t.y) {
        let mut line: Vec<String> = x.iter().map(|v| csvio::sig7(*v)).collect();
        line.push(csvio::sig7(*y));
        writeln!(w, "{}", line.join(",")).map_err(io)?;
    }
    w.flush().map_err(io)?;
    let groups_path = dir.join(format!("{}.groups", t.name));
    std::fs::write(&groups_path, t.groups.join("\n") + "\n").map_err(|e| format!("{}: {e}", groups_path.display()))?;
    let n_groups = t.groups.iter().collect::<std::collections::BTreeSet<_>>().len();
    let n = t.y.len() as f64;
    let mean = t.y.iter().sum::<f64>() / n;
    let sd = (t.y.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1.0).max(1.0)).sqrt();
    let digest = stats::fnv1a(t.keys.join("\n").as_bytes());
    let mut fam_counts: Vec<(String, usize)> = Vec::new();
    for f in &t.features {
        let fam = f.split('.').next().unwrap_or("").to_string() + ".";
        match fam_counts.iter_mut().find(|(k, _)| *k == fam) {
            Some(e) => e.1 += 1,
            None => fam_counts.push((fam, 1)),
        }
    }
    let card = Json::obj()
        .with("schema", "jarambit.regression.table.v1")
        .with("name", t.name.as_str())
        .with("file", format!("{}.csv", t.name))
        .with("target", t.target.as_str())
        .with("target_column", "last")
        .with("featureset", t.featureset.as_str())
        .with("n_rows", t.y.len())
        .with("n_features", t.features.len())
        .with("n_groups", n_groups)
        .with("groups_file", format!("{}.groups", t.name))
        .with("feature_families", Json::Obj(fam_counts.into_iter().map(|(k, v)| (k, Json::from(v))).collect()))
        .with("sources", Json::Obj(t.sources.iter().map(|(k, v)| (k.clone(), Json::from(*v))).collect()))
        .with("row_order", "ascending meta.genome_hash (a deterministic pseudo-random order)")
        .with("row_digest_fnv1a64", format!("{digest:016x}"))
        .with("rows_before_max_rows", t.truncated_from)
        .with(
            "target_stats",
            Json::obj()
                .with("mean", mean)
                .with("sd", sd)
                .with("min", t.y.iter().cloned().fold(f64::INFINITY, f64::min))
                .with("max", t.y.iter().cloned().fold(f64::NEG_INFINITY, f64::max)),
        )
        .with(
            "dropped",
            Json::obj()
                .with("constant", t.dropped_constant)
                .with("all_missing", t.dropped_missing)
                .with("leak_rule", Json::Arr(t.dropped_leak_rule.iter().map(Json::from).collect()))
                .with(
                    "leak_guard",
                    Json::Arr(t.dropped_leak_guard.iter().map(|(c, r)| Json::obj().with("feature", c.as_str()).with("r", *r)).collect()),
                ),
        );
    let card_path = dir.join(format!("{}.card.json", t.name));
    std::fs::write(&card_path, card.pretty()).map_err(|e| format!("{}: {e}", card_path.display()))?;
    Ok(card)
}

pub fn build(opts: &BuildOptions) -> Result<BuildSummary, String> {
    let text = std::fs::read_to_string(&opts.recipes).map_err(|e| format!("{}: {e}", opts.recipes.display()))?;
    let r = Recipes::parse(&text)?;
    let mut log = Vec::new();
    let frames = load_frames(&opts.root, &r, &mut log)?;
    std::fs::create_dir_all(&opts.out).map_err(|e| format!("{}: {e}", opts.out.display()))?;
    // the output directory is owned by this command: clear previous tables
    if let Ok(entries) = std::fs::read_dir(&opts.out) {
        for e in entries.flatten() {
            let n = e.file_name().to_string_lossy().to_string();
            if n.ends_with(".csv") || n.ends_with(".card.json") || n.ends_with(".groups") || n == "index.json" || n == "index.md" {
                let _ = std::fs::remove_file(e.path());
            }
        }
    }
    let mut columns: Vec<String> = Vec::new();
    for f in &frames {
        for c in &f.columns {
            if !columns.contains(c) {
                columns.push(c.clone());
            }
        }
    }
    let max_rows = opts.max_rows.unwrap_or(r.max_rows);
    let mut tables = Vec::new();
    let mut skipped = Vec::new();
    for (target, fs) in r.expand(&columns) {
        let name = recipes::table_name(&target, &fs);
        if opts.only.as_deref().is_some_and(|o| !name.contains(o)) {
            continue;
        }
        match assemble(&frames, &r, &target, &fs, max_rows) {
            Ok(t) => {
                let card = write_table(&opts.out, &t)?;
                eprintln!("dataset build: {} rows={} features={}", t.name, t.y.len(), t.features.len());
                tables.push(card);
            }
            Err(reason) => skipped.push(Json::obj().with("name", name.as_str()).with("reason", reason)),
        }
    }
    let recipe_digest = stats::fnv1a(text.as_bytes());
    let index = Json::obj()
        .with("schema", "jarambit.regression.index.v1")
        .with("recipes", opts.recipes.display().to_string())
        .with("recipes_fnv1a64", format!("{recipe_digest:016x}"))
        .with("sources", Json::Arr(log))
        .with("built", tables.len())
        .with("skipped_count", skipped.len())
        .with(
            "tables",
            Json::Arr(
                tables
                    .iter()
                    .map(|c| {
                        Json::obj()
                            .with("name", c.get("name").cloned().unwrap_or(Json::Null))
                            .with("target", c.get("target").cloned().unwrap_or(Json::Null))
                            .with("featureset", c.get("featureset").cloned().unwrap_or(Json::Null))
                            .with("n_rows", c.get("n_rows").cloned().unwrap_or(Json::Null))
                            .with("n_features", c.get("n_features").cloned().unwrap_or(Json::Null))
                            .with("n_groups", c.get("n_groups").cloned().unwrap_or(Json::Null))
                            .with("row_digest_fnv1a64", c.get("row_digest_fnv1a64").cloned().unwrap_or(Json::Null))
                    })
                    .collect(),
            ),
        )
        .with("skipped", Json::Arr(skipped.clone()));
    std::fs::write(opts.out.join("index.json"), index.pretty()).map_err(|e| e.to_string())?;
    let mut md = String::from("# Regression tables\n\nGenerated by `jarambit dataset build`. The final column of every CSV is the target.\n\n| table | rows | features | lineage groups |\n|---|---:|---:|---:|\n");
    for c in &tables {
        md.push_str(&format!(
            "| `{}` | {} | {} | {} |\n",
            c.get("name").and_then(Json::as_str).unwrap_or(""),
            c.get("n_rows").and_then(Json::as_f64).unwrap_or(0.0),
            c.get("n_features").and_then(Json::as_f64).unwrap_or(0.0),
            c.get("n_groups").and_then(Json::as_f64).unwrap_or(0.0)
        ));
    }
    if !skipped.is_empty() {
        md.push_str(&format!("\n{} specs skipped (see index.json).\n", skipped.len()));
    }
    std::fs::write(opts.out.join("index.md"), md).map_err(|e| e.to_string())?;
    Ok(BuildSummary { built: tables.len(), skipped: skipped.len(), index })
}

#[cfg(test)]
mod tests {
    use super::super::gfeat;
    use super::super::records::{Row, HASH_COL};
    use super::*;

    fn row(seed: u64) -> Row {
        let mut r = Row::new();
        let mut s = stats::SplitMix(seed);
        for c in gfeat::gene_columns() {
            let v = if c == gfeat::MATERIAL_COL { 0.0 } else { (s.next_u64() % 1000) as f64 / 100.0 };
            r.exact(&c, v);
        }
        r.stamp_hash();
        assert!(r.get(HASH_COL).is_some());
        r
    }

    #[test]
    fn builds_tables_with_target_last_and_guards_leaks() {
        let root = std::env::temp_dir().join(format!("jarambit-materialize-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let rec = root.join("records/run/r00.s1.csv");
        let mut rows = Vec::new();
        for i in 0..80u64 {
            let mut r = row(i + 1);
            let g0 = csvio::num(r.get("g.cp01").unwrap());
            r.num("c.mass_g", 70.0 + g0).num("c.blank_g", 2.0 * (70.0 + g0)).num("c.noise", (i % 7) as f64).num("c.const", 1.0);
            r.num("s1.lat_rob", 200.0 + 10.0 * g0 + (i % 3) as f64);
            rows.push(r);
        }
        records::write_shard(&rec, &rows).unwrap();
        let recipes = root.join("recipes.conf");
        std::fs::write(
            &recipes,
            "source records records\nmin_rows 20\nfeatureset g = g. gd.\nfeatureset c = c.\ntargets s1.* : g c\ntargets c.mass_g : g\nleak c.mass_g : c.blank_g\n",
        )
        .unwrap();
        let out = root.join("tables");
        let s = build(&BuildOptions { root: root.clone(), recipes, out: out.clone(), only: None, max_rows: None }).unwrap();
        assert_eq!(s.built, 3, "{}", s.index.pretty());
        let text = std::fs::read_to_string(out.join("s1.lat_rob__c.csv")).unwrap();
        let header: Vec<&str> = text.lines().next().unwrap().split(',').collect();
        assert_eq!(*header.last().unwrap(), "s1.lat_rob");
        assert!(!header.contains(&"c.const"), "constant column dropped");
        // c.mass_g tracks the target only up to noise (|r| < guard), so it stays a feature
        assert!(header.contains(&"c.noise") && header.contains(&"c.mass_g"));
        assert_eq!(text.lines().count(), 81);
        let card = super::super::json::parse(&std::fs::read_to_string(out.join("c.mass_g__g.card.json")).unwrap()).unwrap();
        // g.cp01 is an exact copy of the target (mass = 70 + cp01): the leak guard removes it
        let guard = card.get("dropped").unwrap().get("leak_guard").unwrap().as_arr().unwrap();
        assert!(guard.iter().any(|g| g.get("feature").and_then(Json::as_str) == Some("g.cp01")));
        assert!(out.join("index.json").is_file());
        let groups = std::fs::read_to_string(out.join("s1.lat_rob__c.groups")).unwrap();
        assert_eq!(groups.lines().count(), 80, "one lineage group per row");
        assert!(groups.lines().all(|g| g.starts_with("genome:")), "no run/family/parent meta -> each genome its own group");
        // rebuild is byte-identical
        let first = std::fs::read(out.join("s1.lat_rob__g.csv")).unwrap();
        build(&BuildOptions { root: root.clone(), recipes: root.join("recipes.conf"), out: out.clone(), only: None, max_rows: None }).unwrap();
        assert_eq!(first, std::fs::read(out.join("s1.lat_rob__g.csv")).unwrap());
        // max_rows truncates in hash order
        build(&BuildOptions { root: root.clone(), recipes: root.join("recipes.conf"), out: out.clone(), only: Some("s1.lat_rob__g".into()), max_rows: Some(25) }).unwrap();
        assert_eq!(std::fs::read_to_string(out.join("s1.lat_rob__g.csv")).unwrap().lines().count(), 26);
        assert!(!out.join("c.mass_g__g.csv").exists(), "only filter applied");
    }
}
