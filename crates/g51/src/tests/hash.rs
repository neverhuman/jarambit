use crate::hash::{g51_decoder_sha256, g51_decoder_sha256_uncached, hash_of, G51_HASHED_SOURCES};
use std::path::{Path, PathBuf};

fn src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// Every `.rs` file under `src/` except `lib.rs`, `tests/` and `bin/`, by path relative to `src/`.
fn decoder_sources_on_disk() -> Vec<String> {
    fn walk(dir: &Path, root: &Path, out: &mut Vec<String>) {
        for entry in std::fs::read_dir(dir).expect("read src") {
            let path = entry.expect("dir entry").path();
            let rel = path.strip_prefix(root).expect("under src").to_string_lossy().replace('\\', "/");
            if path.is_dir() {
                if rel != "tests" && rel != "bin" {
                    walk(&path, root, out);
                }
            } else if rel.ends_with(".rs") && rel != "lib.rs" {
                out.push(rel);
            }
        }
    }
    let mut out = Vec::new();
    walk(&src_dir(), &src_dir(), &mut out);
    out.sort();
    out
}

/// No decoder source is left out of the hash, and the hashed texts are those files.
#[test]
fn every_decoder_source_is_hashed() {
    let mut listed: Vec<String> = G51_HASHED_SOURCES.iter().map(|s| s.to_string()).collect();
    listed.sort();
    assert_eq!(decoder_sources_on_disk(), listed, "src/ and G51_HASHED_SOURCES differ");
    let texts: Vec<String> = G51_HASHED_SOURCES
        .iter()
        .map(|f| std::fs::read_to_string(src_dir().join(f)).expect("read source"))
        .collect();
    let refs: Vec<&str> = texts.iter().map(String::as_str).collect();
    let lock = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock")).expect("lock");
    let from_disk = hash_of(&jarambit_g5::g5_qd::g5_decoder_sha256_uncached(), &refs, &lock);
    assert_eq!(from_disk, g51_decoder_sha256_uncached());
}

#[test]
fn cached_hash_equals_a_fresh_recompute_and_differs_from_g5() {
    let h = g51_decoder_sha256();
    assert_eq!(h, g51_decoder_sha256_uncached());
    assert_eq!(h.len(), 64);
    assert_ne!(h, jarambit_g5::g5_qd::g5_decoder_sha256());
}

/// The lock file pins every dependency at G5.0's version: it is G5.0's lock plus this crate's own package entry.
#[test]
fn lock_file_is_g5s_plus_this_crate() {
    let ours = include_str!("../../Cargo.lock");
    let g5 = include_str!("../../../g5/Cargo.lock");
    let packages = |s: &str| -> Vec<String> {
        s.split("[[package]]").skip(1).map(|b| b.trim().to_string()).filter(|b| !b.starts_with("name = \"jarambit-g51\"")).collect()
    };
    assert_eq!(packages(ours), packages(g5));
}
