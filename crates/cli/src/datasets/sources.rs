//! Read-only historical sources that are already committed and need no copying into `records/`.
//!
//! `gen800`: the lossless gen-800 population archives
//! (`artifacts/versioned-evolution-archives-gen800-population-v1/<package>/segments/*.csv.zst`).
//! Each package is decompressed with the `zstd` CLI (no new crates), its 129 genome columns map to
//! `g.*`, and every scalar archive column maps to `h.<name>`. Rows are de-duplicated by genome and
//! sampled deterministically by keeping the `cap` smallest genome hashes ("bottom-k"), which is
//! order-independent and reproducible.
use super::csvio;
use super::gfeat;
use super::records::{Frame, HASH_COL};
use std::collections::BTreeMap;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Locate the zstd CLI: `$JARAMBIT_ZSTD`, else `zstd` on PATH.
pub fn zstd_bin() -> Result<PathBuf, String> {
    if let Ok(p) = std::env::var("JARAMBIT_ZSTD") {
        let p = PathBuf::from(p);
        return if p.is_file() { Ok(p) } else { Err(format!("JARAMBIT_ZSTD={} is not a file", p.display())) };
    }
    let path = std::env::var("PATH").unwrap_or_default();
    for dir in std::env::split_paths(&path) {
        let c = dir.join("zstd");
        if c.is_file() {
            return Ok(c);
        }
    }
    Err("zstd CLI unavailable (install zstd or set JARAMBIT_ZSTD); historical archive sources cannot be read".into())
}

/// Package directories (those holding `population_manifest.v1.json`) under an archive root.
pub fn gen800_packages(root: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = std::fs::read_dir(root)
        .map(|it| it.flatten().map(|e| e.path()).filter(|p| p.join("population_manifest.v1.json").is_file()).collect())
        .unwrap_or_default();
    out.sort();
    out
}

#[derive(Debug, Default)]
pub struct Gen800Report {
    pub package: String,
    pub rows_read: usize,
    pub distinct_genomes_seen: usize,
    pub rows_kept: usize,
    pub bad_rows: usize,
}

/// Load one gen800 package as a frame. `cap == 0` keeps every distinct genome.
pub fn load_gen800_package(dir: &Path, cap: usize) -> Result<(Frame, Gen800Report), String> {
    let package = dir.file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(String::new);
    let mut segments: Vec<PathBuf> = std::fs::read_dir(dir.join("segments"))
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.to_string_lossy().ends_with(".csv.zst"))
        .collect();
    segments.sort();
    if segments.is_empty() {
        return Err(format!("{}: no segments", dir.display()));
    }
    let zstd = zstd_bin()?;
    let mut child = Command::new(&zstd)
        .arg("-dcq")
        .args(&segments)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("{}: {e}", zstd.display()))?;
    let stdout = child.stdout.take().ok_or("zstd stdout unavailable")?;
    let reader = std::io::BufReader::with_capacity(1 << 20, stdout);
    let mut lines = reader.lines();
    let header_line = lines.next().ok_or_else(|| format!("{package}: empty archive"))?.map_err(|e| e.to_string())?;
    let header = csvio::split(&header_line);

    // column plan: genes -> canonical gene slot; scalars -> h.<name>; id/gen -> meta
    let gene_cols = gfeat::gene_columns();
    let mut gene_slot: Vec<Option<usize>> = Vec::with_capacity(header.len());
    let mut h_cols: Vec<(usize, String)> = Vec::new();
    let (mut id_col, mut gen_col) = (None, None);
    for (i, name) in header.iter().enumerate() {
        let rc = gfeat::record_column_for_gene_header(name);
        gene_slot.push(rc.as_ref().and_then(|c| gene_cols.iter().position(|g| g == c)));
        if rc.is_none() {
            match name.as_str() {
                "id" => id_col = Some(i),
                "gen" => gen_col = Some(i),
                _ => h_cols.push((i, format!("h.{name}"))),
            }
        }
    }
    let present = gene_slot.iter().flatten().count();
    if present != gene_cols.len() {
        let _ = child.kill();
        return Err(format!("{package}: archive header carries {present}/{} genome columns", gene_cols.len()));
    }

    let mut rep = Gen800Report { package: package.clone(), ..Default::default() };
    // bottom-k by 64-bit genome hash; value = (genes, h values, id, gen)
    type Kept = (Vec<f64>, Vec<f64>, String, String);
    let mut kept: BTreeMap<u64, Kept> = BTreeMap::new();
    let mut seen_hashes: std::collections::HashSet<u64> = std::collections::HashSet::new();
    for line in lines {
        let line = line.map_err(|e| format!("{package}: {e}"))?;
        if line.is_empty() {
            continue;
        }
        rep.rows_read += 1;
        let cells = csvio::split(&line);
        if cells.len() != header.len() {
            rep.bad_rows += 1;
            continue;
        }
        let mut genes = vec![f64::NAN; gene_cols.len()];
        for (i, slot) in gene_slot.iter().enumerate() {
            if let Some(s) = slot {
                genes[*s] = if *s == gene_cols.len() - 1 { gfeat::material_flag(&cells[i]) } else { csvio::num(&cells[i]) };
            }
        }
        let Some(hex) = gfeat::genome_hash(&genes) else {
            rep.bad_rows += 1;
            continue;
        };
        let h = u64::from_str_radix(&hex, 16).unwrap_or(0);
        if !seen_hashes.insert(h) {
            continue; // same genome already considered (deterministic evaluation)
        }
        if cap > 0 && kept.len() >= cap {
            let worst = *kept.keys().next_back().unwrap_or(&u64::MAX);
            if h >= worst {
                continue;
            }
            kept.remove(&worst);
        }
        let hv: Vec<f64> = h_cols.iter().map(|(i, _)| csvio::num(&cells[*i])).collect();
        let id = id_col.map(|i| cells[i].clone()).unwrap_or_else(String::new);
        let gen = gen_col.map(|i| cells[i].clone()).unwrap_or_else(String::new);
        kept.insert(h, (genes, hv, id, gen));
    }
    let status = child.wait().map_err(|e| e.to_string())?;
    if !status.success() {
        let mut err = String::new();
        if let Some(mut s) = child.stderr.take() {
            let _ = std::io::Read::read_to_string(&mut s, &mut err);
        }
        return Err(format!("{package}: zstd exited {status}: {}", err.trim()));
    }
    rep.distinct_genomes_seen = seen_hashes.len();
    rep.rows_kept = kept.len();

    let mut frame = Frame::new(&format!("gen800:{package}"));
    let gene_idx: Vec<usize> = gene_cols.iter().map(|c| frame.col(c)).collect();
    let h_idx: Vec<usize> = h_cols.iter().map(|(_, c)| frame.col(c)).collect();
    let m_hash = frame.meta_col(HASH_COL);
    let m_src = frame.meta_col("meta.source");
    let m_id = frame.meta_col("meta.archive_id");
    let m_gen = frame.meta_col("meta.gen");
    let width = frame.columns.len();
    let mwidth = frame.meta_columns.len();
    for (h, (genes, hv, id, gen)) in kept {
        let mut row = vec![f64::NAN; width];
        for (v, &i) in genes.iter().zip(&gene_idx) {
            row[i] = *v;
        }
        for (v, &i) in hv.iter().zip(&h_idx) {
            row[i] = *v;
        }
        let mut meta = vec![String::new(); mwidth];
        meta[m_hash] = format!("{h:016x}");
        meta[m_src] = format!("gen800/{package}");
        meta[m_id] = id;
        meta[m_gen] = gen;
        frame.rows.push(row);
        frame.keys.push(format!("{h:016x}"));
        frame.meta.push(meta);
    }
    Ok((frame, rep))
}
