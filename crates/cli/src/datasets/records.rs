//! Micro-record shards: append-only CSV files under `datasets/regression/records/`.
//!
//! A shard is a CSV with one header line. Column prefixes carry meaning:
//! `meta.*` text (provenance), `g.*` exact genes, `c.*`/`a.*`/`gate.*`/`desc.*` CAD features,
//! `s1.*` Stage-1 screen outputs, `s2.*` Stage-2 assessment outputs, `h.*` historical
//! archive outputs, `phys.*` physical test results. A shard that carries the complete genome is a
//! *base* shard; one without genes is an *attachment* joined on `meta.genome_hash`.
//! Every row is written with a single `write_all` + flush, so a crash can only leave a torn final
//! line, which readers detect (no trailing newline) and drop.
use super::csvio;
use super::gfeat;
use std::collections::{BTreeMap, HashMap};
use std::io::Write as _;
use std::path::{Path, PathBuf};

pub const HASH_COL: &str = "meta.genome_hash";

/// One row under construction: ordered (column, formatted cell) pairs.
#[derive(Clone, Debug, Default)]
pub struct Row {
    pub cells: Vec<(String, String)>,
}

impl Row {
    pub fn new() -> Self {
        Row::default()
    }
    pub fn text(&mut self, col: &str, value: &str) -> &mut Self {
        self.put(col, value.to_string())
    }
    /// Feature / target number at 7 significant digits.
    pub fn num(&mut self, col: &str, value: f64) -> &mut Self {
        self.put(col, csvio::sig7(value))
    }
    /// Exact round-trip number (genes).
    pub fn exact(&mut self, col: &str, value: f64) -> &mut Self {
        self.put(col, csvio::exact(value))
    }
    fn put(&mut self, col: &str, cell: String) -> &mut Self {
        if let Some(slot) = self.cells.iter_mut().find(|(k, _)| k == col) {
            slot.1 = cell;
        } else {
            self.cells.push((col.to_string(), cell));
        }
        self
    }
    pub fn get(&self, col: &str) -> Option<&str> {
        self.cells.iter().find(|(k, _)| k == col).map(|(_, v)| v.as_str())
    }
    pub fn columns(&self) -> Vec<String> {
        self.cells.iter().map(|(k, _)| k.clone()).collect()
    }
    /// Genes in canonical order, or None if the row does not carry a complete genome.
    pub fn genes(&self) -> Option<Vec<f64>> {
        gfeat::gene_columns().iter().map(|c| self.get(c).map(csvio::num).filter(|x| x.is_finite())).collect()
    }
    /// Fill `meta.genome_hash` from the genes when present.
    pub fn stamp_hash(&mut self) -> &mut Self {
        if let Some(h) = self.genes().as_deref().and_then(gfeat::genome_hash) {
            self.text(HASH_COL, &h);
        }
        self
    }
}

/// Append-mode shard writer. Opening an existing shard requires an identical header.
pub struct ShardWriter {
    path: PathBuf,
    columns: Vec<String>,
    file: std::fs::File,
    pub rows_written: usize,
}

impl ShardWriter {
    pub fn open(path: &Path, columns: &[String]) -> Result<Self, String> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        }
        let header = csvio::join(columns);
        if path.exists() {
            let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
            if text.lines().next() != Some(header.as_str()) {
                return Err(format!("{}: existing shard header differs; refusing to append", path.display()));
            }
            if !text.is_empty() && !text.ends_with('\n') {
                return Err(format!("{}: torn final line; repair before appending", path.display()));
            }
        } else {
            std::fs::write(path, format!("{header}\n")).map_err(|e| format!("{}: {e}", path.display()))?;
        }
        let file = std::fs::OpenOptions::new().append(true).open(path).map_err(|e| format!("{}: {e}", path.display()))?;
        Ok(ShardWriter { path: path.to_path_buf(), columns: columns.to_vec(), file, rows_written: 0 })
    }
    /// Append one row (cells not in the header are an error, missing cells are empty).
    pub fn append(&mut self, row: &Row) -> Result<(), String> {
        let line = render(&self.columns, row)?;
        self.file
            .write_all(line.as_bytes())
            .and_then(|_| self.file.flush())
            .map_err(|e| format!("{}: {e}", self.path.display()))?;
        self.rows_written += 1;
        Ok(())
    }
}

fn render(columns: &[String], row: &Row) -> Result<String, String> {
    let index: HashMap<&str, usize> = columns.iter().enumerate().map(|(i, c)| (c.as_str(), i)).collect();
    let mut cells = vec![String::new(); columns.len()];
    for (k, v) in &row.cells {
        let i = index.get(k.as_str()).ok_or_else(|| format!("column {k:?} not in shard header"))?;
        cells[*i] = v.clone();
    }
    Ok(csvio::join(&cells) + "\n")
}

/// Column order for a set of rows: meta first, then genes in canonical order, then everything else
/// in first-seen order.
pub fn columns_for(rows: &[Row]) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    for r in rows {
        for (k, _) in &r.cells {
            if !seen.contains(k) {
                seen.push(k.clone());
            }
        }
    }
    let genes = gfeat::gene_columns();
    let mut out: Vec<String> = seen.iter().filter(|c| c.starts_with("meta.")).cloned().collect();
    out.extend(genes.iter().filter(|g| seen.contains(g)).cloned());
    out.extend(seen.iter().filter(|c| !c.starts_with("meta.") && !genes.contains(c)).cloned());
    out
}

/// Write a whole shard atomically (temp file + rename). Used for per-round Stage-1 shards.
pub fn write_shard(path: &Path, rows: &[Row]) -> Result<usize, String> {
    let columns = columns_for(rows);
    let mut text = csvio::join(&columns) + "\n";
    for r in rows {
        text.push_str(&render(&columns, r)?);
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    let tmp = path.with_extension("csv.tmp");
    std::fs::write(&tmp, text).map_err(|e| format!("{}: {e}", tmp.display()))?;
    std::fs::rename(&tmp, path).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(rows.len())
}

/// Publish a whole shard only if `path` does not exist yet; never replaces an existing file.
/// The live recorder uses this for per-round Stage-1 shards; `write_shard` keeps its rewrite semantics.
pub fn write_shard_create_new(path: &Path, rows: &[Row]) -> Result<usize, String> {
    let columns = columns_for(rows);
    let mut text = csvio::join(&columns) + "\n";
    for r in rows {
        text.push_str(&render(&columns, r)?);
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    let tmp = path.with_extension("csv.create-new.tmp");
    // `create_new` on the private temp name: if it already exists we did not create it, so we must not remove it.
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&tmp)
        .map_err(|e| format!("{}: {e}", tmp.display()))?;
    if let Err(e) = file.write_all(text.as_bytes()) {
        drop(file);
        let _ = std::fs::remove_file(&tmp);
        return Err(format!("{}: {e}", tmp.display()));
    }
    drop(file);
    // A hard link publishes the complete file and refuses an existing destination.
    // No fallback to rename: that would silently replace another run's shard.
    if let Err(e) = std::fs::hard_link(&tmp, path) {
        let _ = std::fs::remove_file(&tmp);
        return Err(format!("{}: create-only publication failed: {e}", path.display()));
    }
    // Publication succeeded; a failure to unlink our private name must not turn it into an error.
    if let Err(e) = std::fs::remove_file(&tmp) {
        eprintln!("dataset shard: published {}; temporary link retained at {}: {e}", path.display(), tmp.display());
    }
    Ok(rows.len())
}

/// Parsed shard: header + rows of raw cells.
pub struct Shard {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub torn_tail: bool,
    pub bad_rows: usize,
}

pub fn read_shard(path: &Path) -> Result<Shard, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let torn_tail = !text.is_empty() && !text.ends_with('\n');
    let mut lines: Vec<&str> = text.split('\n').collect();
    // split leaves a trailing "" for a complete file, or the torn fragment otherwise
    lines.pop();
    let mut it = lines.into_iter();
    let header = it.next().ok_or_else(|| format!("{}: empty shard", path.display()))?;
    let columns = csvio::split(header.trim_end_matches('\r'));
    let mut rows = Vec::new();
    let mut bad_rows = 0;
    for line in it {
        let line = line.trim_end_matches('\r');
        if line.is_empty() {
            continue;
        }
        let cells = csvio::split(line);
        if cells.len() == columns.len() {
            rows.push(cells);
        } else {
            bad_rows += 1;
        }
    }
    Ok(Shard { columns, rows, torn_tail, bad_rows })
}

/// All `*.csv` files below `root`, sorted (hidden entries and `*.tmp` skipped).
pub fn list_shards(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for e in entries.flatten() {
            let p = e.path();
            let name = e.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                continue;
            }
            if p.is_dir() {
                stack.push(p);
            } else if name.ends_with(".csv") {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

/// Wide numeric frame (NaN = missing) plus text meta columns. One row per distinct genome.
#[derive(Default)]
pub struct Frame {
    pub source: String,
    pub columns: Vec<String>,
    pub index: HashMap<String, usize>,
    pub rows: Vec<Vec<f64>>,
    pub keys: Vec<String>,
    pub meta_columns: Vec<String>,
    pub meta: Vec<Vec<String>>,
}

impl Frame {
    pub fn new(source: &str) -> Self {
        Frame { source: source.to_string(), ..Default::default() }
    }
    pub fn col(&mut self, name: &str) -> usize {
        if let Some(i) = self.index.get(name) {
            return *i;
        }
        let i = self.columns.len();
        self.columns.push(name.to_string());
        self.index.insert(name.to_string(), i);
        for r in &mut self.rows {
            r.push(f64::NAN);
        }
        i
    }
    pub fn meta_col(&mut self, name: &str) -> usize {
        if let Some(i) = self.meta_columns.iter().position(|c| c == name) {
            return i;
        }
        self.meta_columns.push(name.to_string());
        for r in &mut self.meta {
            r.push(String::new());
        }
        self.meta_columns.len() - 1
    }
    pub fn get(&self, row: usize, col: &str) -> f64 {
        self.index.get(col).map(|&i| self.rows[row][i]).unwrap_or(f64::NAN)
    }
    pub fn genes(&self, row: usize) -> Option<Vec<f64>> {
        gfeat::gene_columns().iter().map(|c| Some(self.get(row, c)).filter(|x| x.is_finite())).collect()
    }
    /// Append the genome-derived `gd.*` features for every row with a complete genome.
    pub fn add_derived(&mut self) {
        let names = gfeat::derived_names();
        let cols: Vec<usize> = names.iter().map(|n| self.col(n)).collect();
        for r in 0..self.rows.len() {
            if let Some(g) = self.genes(r) {
                for ((_, v), &c) in gfeat::derived(&g).iter().zip(&cols) {
                    self.rows[r][c] = *v;
                }
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct LoadReport {
    pub shards: usize,
    pub base_rows: usize,
    pub attachment_rows: usize,
    pub duplicate_genomes: usize,
    pub conflicting_cells: usize,
    pub orphan_attachments: usize,
    pub torn_tails: usize,
    pub bad_rows: usize,
    pub rows_without_genome: usize,
}

/// Load every shard under `root` into one frame keyed by genome hash.
pub fn load_records(root: &Path) -> Result<(Frame, LoadReport), String> {
    let mut rep = LoadReport::default();
    let mut frame = Frame::new("records");
    let mut by_key: HashMap<String, usize> = HashMap::new();
    // (genome hash, column) -> (sum, count) for attachment values
    let mut attach: BTreeMap<(String, String), (f64, usize)> = BTreeMap::new();
    let genes = gfeat::gene_columns();
    for path in list_shards(root) {
        let shard = read_shard(&path)?;
        rep.shards += 1;
        rep.torn_tails += shard.torn_tail as usize;
        rep.bad_rows += shard.bad_rows;
        let pos: HashMap<&str, usize> = shard.columns.iter().enumerate().map(|(i, c)| (c.as_str(), i)).collect();
        let has_genome = genes.iter().all(|g| pos.contains_key(g.as_str()));
        for cells in &shard.rows {
            if has_genome {
                let gv: Option<Vec<f64>> = genes.iter().map(|g| Some(csvio::num(&cells[pos[g.as_str()]])).filter(|x| x.is_finite())).collect();
                let Some(key) = gv.as_deref().and_then(gfeat::genome_hash) else {
                    rep.rows_without_genome += 1;
                    continue;
                };
                rep.base_rows += 1;
                let row = match by_key.get(&key) {
                    Some(&r) => {
                        rep.duplicate_genomes += 1;
                        r
                    }
                    None => {
                        let r = frame.rows.len();
                        frame.rows.push(vec![f64::NAN; frame.columns.len()]);
                        frame.meta.push(vec![String::new(); frame.meta_columns.len()]);
                        frame.keys.push(key.clone());
                        by_key.insert(key.clone(), r);
                        r
                    }
                };
                for (c, name) in shard.columns.iter().enumerate() {
                    let cell = &cells[c];
                    if name.starts_with("meta.") {
                        let m = frame.meta_col(name);
                        if frame.meta[row][m].is_empty() {
                            frame.meta[row][m] = cell.clone();
                        }
                        continue;
                    }
                    let v = csvio::num(cell);
                    if !v.is_finite() {
                        continue;
                    }
                    let i = frame.col(name);
                    let slot = &mut frame.rows[row][i];
                    if slot.is_nan() {
                        *slot = v;
                    } else if *slot != v {
                        rep.conflicting_cells += 1;
                    }
                }
            } else {
                let Some(&h) = pos.get(HASH_COL) else {
                    rep.rows_without_genome += 1;
                    continue;
                };
                let key = cells[h].trim().to_string();
                if key.is_empty() {
                    rep.rows_without_genome += 1;
                    continue;
                }
                rep.attachment_rows += 1;
                for (c, name) in shard.columns.iter().enumerate() {
                    if name.starts_with("meta.") {
                        continue;
                    }
                    let v = csvio::num(&cells[c]);
                    if v.is_finite() {
                        let e = attach.entry((key.clone(), name.clone())).or_insert((0.0, 0));
                        e.0 += v;
                        e.1 += 1;
                    }
                }
            }
        }
    }
    let mut orphans: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for ((key, col), (sum, n)) in attach {
        match by_key.get(&key) {
            Some(&r) => {
                let i = frame.col(&col);
                frame.rows[r][i] = sum / n as f64;
            }
            None => {
                orphans.insert(key);
            }
        }
    }
    rep.orphan_attachments = orphans.len();
    let h = frame.meta_col(HASH_COL);
    for r in 0..frame.rows.len() {
        frame.meta[r][h] = frame.keys[r].clone();
    }
    Ok((frame, rep))
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn genome_row(seed: f64) -> Row {
        let mut r = Row::new();
        for (i, c) in gfeat::gene_columns().iter().enumerate() {
            r.exact(c, if c == gfeat::MATERIAL_COL { 0.0 } else { seed + i as f64 * 0.01 });
        }
        r.stamp_hash();
        r
    }

    fn tmpdir(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("jarambit-datasets-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn shard_writer_appends_and_reader_drops_torn_tail() {
        let d = tmpdir("torn");
        let p = d.join("a.csv");
        let cols = vec!["meta.x".to_string(), "s1.v".to_string()];
        let mut w = ShardWriter::open(&p, &cols).unwrap();
        let mut r = Row::new();
        r.text("meta.x", "a,b").num("s1.v", 1.5);
        w.append(&r).unwrap();
        drop(w);
        // reopen with the same header appends
        let mut w = ShardWriter::open(&p, &cols).unwrap();
        w.append(Row::new().num("s1.v", 2.0)).unwrap();
        assert!(w.append(Row::new().num("s1.other", 1.0)).is_err());
        drop(w);
        assert!(ShardWriter::open(&p, &["meta.y".to_string()]).is_err());
        let mut f = std::fs::OpenOptions::new().append(true).open(&p).unwrap();
        f.write_all(b"torn,3").unwrap();
        drop(f);
        let s = read_shard(&p).unwrap();
        assert!(s.torn_tail);
        assert_eq!(s.rows.len(), 2);
        assert_eq!(s.rows[0][0], "a,b");
        assert!(ShardWriter::open(&p, &cols).is_err(), "refuses to append after a torn tail");
    }

    #[test]
    fn records_dedupe_by_genome_and_join_attachments() {
        let d = tmpdir("join");
        let mut a = genome_row(1.0);
        a.text("meta.run_id", "r1").num("c.mass_g", 80.0).num("s1.lat_rob", 250.0);
        let mut b = genome_row(2.0);
        b.text("meta.run_id", "r1").num("c.mass_g", 81.0);
        let mut a2 = genome_row(1.0);
        a2.text("meta.run_id", "r2").num("s1.seq_ratio", 1.3);
        write_shard(&d.join("run1/r00.s1.csv"), &[a.clone(), b.clone()]).unwrap();
        write_shard(&d.join("run2/r00.s1.csv"), &[a2]).unwrap();
        let ha = a.get(HASH_COL).unwrap().to_string();
        let mut s2 = Row::new();
        s2.text(HASH_COL, &ha).num("s2.solid_tip_sf", 1.0);
        let mut s2b = Row::new();
        s2b.text(HASH_COL, &ha).num("s2.solid_tip_sf", 2.0);
        let mut orphan = Row::new();
        orphan.text(HASH_COL, "ffffffffffffffff").num("s2.solid_tip_sf", 9.0);
        write_shard(&d.join("run1/r00.s2.csv"), &[s2, s2b, orphan]).unwrap();
        let (f, rep) = load_records(&d).unwrap();
        assert_eq!(f.rows.len(), 2);
        assert_eq!(rep.duplicate_genomes, 1);
        assert_eq!(rep.orphan_attachments, 1);
        let ra = f.keys.iter().position(|k| *k == ha).unwrap();
        assert_eq!(f.get(ra, "s1.lat_rob"), 250.0);
        assert_eq!(f.get(ra, "s1.seq_ratio"), 1.3);
        assert_eq!(f.get(ra, "s2.solid_tip_sf"), 1.5, "replicates average");
        let rb = 1 - ra;
        assert!(f.get(rb, "s2.solid_tip_sf").is_nan());
        let mut f = f;
        f.add_derived();
        assert!(f.get(ra, "gd.ring_od").is_finite());
    }

    #[test]
    fn write_shard_create_new_refuses_to_replace() {
        let d = tmpdir("create-new");
        let path = d.join("r00.s1.csv");
        let a = genome_row(1.0);
        assert_eq!(write_shard_create_new(&path, std::slice::from_ref(&a)).unwrap(), 1);
        let first = std::fs::read(&path).unwrap();
        write_shard(&d.join("reference.csv"), std::slice::from_ref(&a)).unwrap();
        assert_eq!(first, std::fs::read(d.join("reference.csv")).unwrap(), "same CSV as write_shard");
        assert!(!d.join("r00.s1.csv.create-new.tmp").exists(), "private temp name removed after publication");
        let b = genome_row(2.0);
        assert!(write_shard_create_new(&path, &[b]).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), first, "first shard bytes unchanged");
        assert!(!d.join("r00.s1.csv.create-new.tmp").exists(), "temp removed after refused publication");
        let _ = std::fs::remove_dir_all(&d);
    }
}
