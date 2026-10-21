use crate::{hash, Record, Result};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
static NEXT: AtomicU64 = AtomicU64::new(0);
/// Atomic, no-clobber publish. A concurrent writer with identical bytes is idempotent.
/// A different payload at the same identity is an error, never last-writer-wins.
pub fn put(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path.parent().ok_or("path has no parent")?;
    std::fs::create_dir_all(parent)?;
    if path.exists() {
        return if std::fs::read(path)? == bytes {
            Ok(())
        } else {
            Err(format!("immutable conflict: {}", path.display()).into())
        };
    }
    let tmp = parent.join(format!(
        ".tmp-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let mut f = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&tmp)?;
    let result = (|| -> Result<()> {
        f.write_all(bytes)?;
        f.sync_all()?;
        match std::fs::hard_link(&tmp, path) {
            Ok(()) => (),
            Err(e)
                if e.kind() == std::io::ErrorKind::AlreadyExists
                    && std::fs::read(path)? == bytes => {}
            Err(e) => return Err(e.into()),
        }
        std::fs::File::open(parent)?.sync_all()?;
        Ok(())
    })();
    let _ = std::fs::remove_file(tmp);
    result
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    pub schema: u32,
    pub rows: usize,
    pub sha256: String,
    pub segment: String,
}
/// One completed batch = one compressed micro-shard + receipt. No shared CSV append.
pub fn write_batch(root: &Path, records: &[Record]) -> Result<PathBuf> {
    if records.is_empty() {
        return Err("empty batch".into());
    }
    let mut raw = Vec::new();
    for record in records {
        record.validate()?;
        serde_json::to_writer(&mut raw, record)?;
        raw.push(b'\n');
    }
    if raw.len() > 64 * 1024 * 1024 || records.len() > 100_000 {
        return Err("batch exceeds the reader budget".into());
    }
    let compressed = zstd::bulk::compress(&raw, 3)?;
    let digest = hash(&compressed);
    let segment = format!("{digest}.jsonl.zst");
    put(&root.join("segments").join(&segment), &compressed)?;
    let receipt = Receipt {
        schema: 1,
        rows: records.len(),
        sha256: digest.clone(),
        segment,
    };
    let receipt_path = root.join("receipts").join(format!("{digest}.json"));
    put(&receipt_path, &serde_json::to_vec_pretty(&receipt)?)?;
    Ok(receipt_path)
}
pub fn read_batch(receipt: &Path) -> Result<Vec<Record>> {
    let r: Receipt = serde_json::from_slice(&std::fs::read(receipt)?)?;
    if r.schema != 1
        || r.sha256.len() != 64
        || !r.sha256.bytes().all(|c| c.is_ascii_hexdigit())
        || r.segment != format!("{}.jsonl.zst", r.sha256)
    {
        return Err("invalid receipt".into());
    }
    let root = receipt
        .parent()
        .and_then(Path::parent)
        .ok_or("receipt root missing")?;
    let b = std::fs::read(root.join("segments").join(&r.segment))?;
    if hash(&b) != r.sha256 {
        return Err("segment checksum mismatch".into());
    }
    // Bound decompression before JSON parsing (64 MiB per batch, 100k observations).
    let mut decoder = zstd::stream::read::Decoder::new(b.as_slice())?;
    let mut raw = Vec::new();
    use std::io::Read;
    decoder
        .by_ref()
        .take(64 * 1024 * 1024 + 1)
        .read_to_end(&mut raw)?;
    if raw.len() > 64 * 1024 * 1024 || !raw.ends_with(b"\n") || r.rows > 100_000 {
        return Err("oversize or incomplete shard".into());
    }
    let mut records = Vec::new();
    for line in raw.split(|x| *x == b'\n').filter(|x| !x.is_empty()) {
        let record: Record = serde_json::from_slice(line)?;
        record.validate()?;
        records.push(record);
    }
    if records.len() != r.rows {
        return Err("receipt row count mismatch".into());
    }
    Ok(records)
}
pub fn receipts(root: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    fn walk(p: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
        for entry in std::fs::read_dir(p)? {
            let e = entry?;
            let t = e.file_type()?;
            if t.is_symlink() {
                return Err("symlinks are not accepted in dataset input".into());
            }
            if t.is_dir() {
                walk(&e.path(), out)?;
            } else if p.file_name().is_some_and(|n| n == "receipts")
                && e.path().extension().is_some_and(|n| n == "json")
            {
                out.push(e.path());
            }
        }
        Ok(())
    }
    walk(root, &mut out)?;
    out.sort();
    Ok(out)
}
