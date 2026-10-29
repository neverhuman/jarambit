//! The G5.1 decoder hash: `G5.1`, the whole G5.0 decoder hash (every file G5.0 hashes, RS-karambit-v5 included),
//! every G5.1 decoder source and this crate's `Cargo.lock`. Every G5.1 record and archive carries it, and archives
//! refuse rows of another decoder. `lib.rs` (the module list), `src/tests/` and `src/bin/` are outside it, as in G5.0;
//! a test checks that no other source is left out (`tests::hash`).

/// Decoder sources in hash order, by path relative to `src/` (the test compares this list with the directory).
pub const G51_HASHED_SOURCES: [&str; 6] = ["f64x.rs", "hand.rs", "hash.rs", "namespace.rs", "sdf.rs", "tiers.rs"];

fn hashed_texts() -> [&'static str; 6] {
    [
        include_str!("f64x.rs"),
        include_str!("hand.rs"),
        include_str!("hash.rs"),
        include_str!("namespace.rs"),
        include_str!("sdf.rs"),
        include_str!("tiers.rs"),
    ]
}

/// The G5.1 decoder hash, computed once per process.
pub fn g51_decoder_sha256() -> String {
    static HASH: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    HASH.get_or_init(g51_decoder_sha256_uncached).clone()
}

/// The hash computed afresh.
pub fn g51_decoder_sha256_uncached() -> String {
    hash_of(&jarambit_g5::g5_qd::g5_decoder_sha256_uncached(), &hashed_texts(), include_str!("../Cargo.lock"))
}

/// `sha256("G5.1\0" + g5_hash + "\0" + each source + "\0" + lock + "\0")`.
pub fn hash_of(g5_hash: &str, sources: &[&str], lock: &str) -> String {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"G5.1\0");
    bytes.extend_from_slice(g5_hash.as_bytes());
    bytes.push(0);
    for src in sources {
        bytes.extend_from_slice(src.as_bytes());
        bytes.push(0);
    }
    bytes.extend_from_slice(lock.as_bytes());
    bytes.push(0);
    jarambit_g5::sha256_bytes(&bytes)
}
