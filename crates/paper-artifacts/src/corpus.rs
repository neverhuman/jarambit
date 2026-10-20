//! Retained-evidence Rust readers and receipt metadata validators (`JR-EI-P1-T04`..`T08`,
//! `JR-EI-P2-T03`..`T05`, and `SC-DEC-183N` `JR-R1` / `JR-R3`).
//!
//! Functions prefixed `validate_*` are **receipt metadata validators only**.
//! Functions prefixed `read_*` / `verify_and_decode_*` are **real byte-reading
//! retained-evidence decoders** that open the underlying `JEV1` binary stream,
//! JSONL birth/screen files, CSVs, run manifests, and `.csv.zst` compressed
//! population chunks.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub const HISTORICAL_JEVL_SHA256: &str =
    "29d4602b1728e179606ac380df69652efea6e45c3daf67a6602cf56a6699b181";
pub const HISTORICAL_JEVL_FIRST16_HEX: &str = "4a45563198010000a34f027366a90fd9";
pub const HISTORICAL_ALL_EVALS_JSON_SHA256: &str =
    "307370108fcede8a4399569582f6ab19d436603246531266afa1754f9ba65363";
pub const HISTORICAL_ALL_EVALS_CSV_SHA256: &str =
    "a28f6d2c8a676661280981733a1e33c80136aba157c7a9d2f5a5c3db444a223c";
pub const HISTORICAL_HISTORY_CSV_SHA256: &str =
    "9fde87c601b0a7fab3bf4b3f66ca3505df19917cb45e7c7bc7994f3e8f59333f";
pub const HISTORICAL_QD_ARCHIVE_CSV_SHA256: &str =
    "9d4ce161375faceb7a13c03c021b2fed50a47db96cb9f514552416f41898f28c";
pub const HISTORICAL_SUMMARY_JSON_SHA256: &str =
    "e4e6cf51da8269833709ef8501ba3b06934cfb5baecc038628f384675b63dd49";

pub const V1_FIRST_OBSERVED_SHA256: &str =
    "081f34e5659ce518ce45ef6ed731840609b221aa3c2f6b1bfbc24322fceacf98";
pub const V1_SECOND_OBSERVED_SHA256: &str =
    "62b67373d45f5a1174561591e6ba49508f3ec1d3bce0bed6e2adad2f04807eae";
pub const V1_CURRENT_REMOTE_SHA256: &str =
    "010a91d7e4865cd661d14b56c7c1bf50feccfcc8080c5e65c3040628881bf311";
pub const V2_FROZEN_HANDOFF_SHA256: &str =
    "b1f884622aaffc9929069424685ed0aaf8b950470b254141336c193f1ec2529e";
pub const V2_ERRATUM_SHA256: &str =
    "a3cefc5ce7f841aa6e7b1edfd8503c988dfa1e4e04754b22ea7babc4bcc1ff8a";
pub const T07_BASE_INDEX_SHA256: &str =
    "dd69650f66a0c6cf29cbfcfc80e1302529717ba7e31a9af6b0b825d13970c49b";
pub const GEN800_POPULATION_INDEX_SHA256: &str =
    "66c84ed4cdc943df885da4cdd72d25c0054195d9d8b9a01e1c35b6f3119c38cf";

pub const HARD_SEGMENT_CEILING_BYTES: u64 = 8_388_608;
pub const HARD_LOGICAL_SEGMENT_CEILING_BYTES: u64 = 67_108_864;
pub const MAX_JEV1_FRAME_LOGICAL_BYTES: u64 = 16_777_216;
pub const GEN800_EXPECTED_COLUMNS: usize = 146;
pub const GEN800_EXPECTED_GENE_MATERIAL_COLUMNS: usize = 129;

fn sha256_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn hex_bytes(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &x in bytes {
        h ^= x as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn sha256_hex(v: &Value, field: &str) -> Result<String> {
    let s = v
        .as_str()
        .ok_or_else(|| format!("corpus: missing string for {field}"))?;
    if s.len() != 64 || !s.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(format!("corpus: invalid sha256 hex in {field}: {s}").into());
    }
    Ok(s.to_string())
}

/// Resolve a strictly relative path under `root`, refusing absolute paths,
/// parent traversal (`..`), curdir (`.`), backslashes, symlink components,
/// and canonical root containment escapes (`SC-DEC-183P` item 5).
pub fn resolve_safe_relpath(root: &Path, rel: &str) -> Result<PathBuf> {
    if rel.is_empty() || rel.contains('\\') {
        return Err(format!("corpus: unsafe or empty relative path: {rel}").into());
    }
    let p = Path::new(rel);
    if p.is_absolute() {
        return Err(format!("corpus: absolute path refused: {rel}").into());
    }
    let mut cur = root.to_path_buf();
    for c in p.components() {
        if !matches!(c, Component::Normal(_)) {
            return Err(format!("corpus: non-normal path component refused: {rel}").into());
        }
        cur.push(c);
        if let Ok(meta) = fs::symlink_metadata(&cur) {
            if meta.file_type().is_symlink() {
                return Err(format!("corpus: symlink component refused in path: {rel}").into());
            }
        }
    }
    if let (Ok(canon_root), Ok(canon_cur)) = (root.canonicalize(), cur.canonicalize()) {
        if !canon_cur.starts_with(&canon_root) {
            return Err(format!("corpus: canonical containment escape refused: {rel}").into());
        }
    }
    Ok(cur)
}

/// Preflight file metadata (`fs::symlink_metadata`) and enforce a hard byte ceiling
/// (`Read::take(max_bytes + 1)`) before allocating or reading (`SC-DEC-183P` item 5).
pub fn read_bounded_file(path: &Path, max_bytes: u64) -> Result<Vec<u8>> {
    let meta = fs::symlink_metadata(path)
        .map_err(|e| format!("corpus: missing or unreadable file {}: {e}", path.display()))?;
    if meta.file_type().is_symlink() || !meta.file_type().is_file() {
        return Err(format!(
            "corpus: non-regular or symlink file refused: {}",
            path.display()
        )
        .into());
    }
    let len = meta.len();
    if len == 0 || len > max_bytes {
        return Err(format!(
            "corpus: file {} byte length {len} violates ceiling (1..={max_bytes})",
            path.display()
        )
        .into());
    }
    let mut file = fs::File::open(path)?;
    let mut buf = Vec::with_capacity(len as usize);
    (&mut file)
        .take(max_bytes.saturating_add(1))
        .read_to_end(&mut buf)?;
    if buf.len() as u64 != len || buf.len() as u64 > max_bytes {
        return Err(format!(
            "corpus: bounded read size mismatch or ceiling exceeded for {}",
            path.display()
        )
        .into());
    }
    Ok(buf)
}

/// Resolve a repo-relative path (such as `artifacts/versioned-evolution-archives-gen800-population-v1/...`)
/// strictly inside `repo_root` without escaping to parent directories (`SC-DEC-183X` §3.1).
pub fn resolve_archive_path(repo_root: &Path, rel: &str) -> Result<PathBuf> {
    let direct = resolve_safe_relpath(repo_root, rel)?;
    if !direct.exists() {
        return Err(format!(
            "corpus: required archive path missing under repo_root: {}",
            direct.display()
        )
        .into());
    }
    Ok(direct)
}

/// Parse binary `JEV1` frames from `evidence/rs_v2/w282_ultrarunner/search/evidence.jevl`,
/// verifying magic `b"JEV1"`, u32 LE compressed length, u64 LE FNV-1a checksum of the compressed
/// zstd frame, bounded pure-Rust `ruzstd` decompression, JSONL event parsing, and exact EOF consumption.
pub fn parse_jev1_frames(bytes: &[u8]) -> Result<Vec<Value>> {
    let mut offset = 0usize;
    let mut frames = Vec::new();
    while offset < bytes.len() {
        if offset + 16 > bytes.len() {
            return Err("corpus: truncated JEV1 frame header".into());
        }
        if &bytes[offset..offset + 4] != b"JEV1" {
            return Err("corpus: invalid JEV1 frame magic".into());
        }
        let compressed_len =
            u32::from_le_bytes(bytes[offset + 4..offset + 8].try_into().unwrap()) as usize;
        if compressed_len == 0 || (compressed_len as u64) > HARD_SEGMENT_CEILING_BYTES {
            return Err("corpus: JEV1 compressed frame length outside ceiling".into());
        }
        let expected_fnv1a = u64::from_le_bytes(bytes[offset + 8..offset + 16].try_into().unwrap());
        let payload_start = offset + 16;
        let payload_end = payload_start + compressed_len;
        if payload_end > bytes.len() {
            return Err("corpus: truncated JEV1 compressed zstd frame".into());
        }
        let zstd_slice = &bytes[payload_start..payload_end];
        if fnv1a_64(zstd_slice) != expected_fnv1a {
            return Err("corpus: JEV1 frame FNV-1a checksum mismatch".into());
        }
        let decoder = ruzstd::decoding::StreamingDecoder::new(zstd_slice)?;
        let mut raw = Vec::new();
        decoder
            .take(MAX_JEV1_FRAME_LOGICAL_BYTES.saturating_add(1))
            .read_to_end(&mut raw)?;
        if (raw.len() as u64) > MAX_JEV1_FRAME_LOGICAL_BYTES {
            return Err("corpus: JEV1 frame logical size exceeded bound".into());
        }
        let mut events = Vec::new();
        for line in raw.split(|c| *c == b'\n').filter(|l| !l.is_empty()) {
            let ev: Value = serde_json::from_slice(line)?;
            events.push(ev);
        }
        frames.push(json!({
            "frame_index": frames.len(),
            "compressed_bytes": compressed_len,
            "fnv1a_u64": expected_fnv1a,
            "events": events,
        }));
        offset = payload_end;
    }
    if offset != bytes.len() {
        return Err("corpus: trailing bytes after JEV1 frame stream".into());
    }
    Ok(frames)
}

/// Real byte-reading historical cohort reader (`JR-R1` / `JR-EI-P2-T03` / `SC-DEC-183P`).
/// Opens and reconciles `evidence.jevl` (`JEV1` binary stream), `all_evals.json`,
/// `all_evals.csv` (`146` columns = `17` scalar + `128` numeric genes + `material`),
/// `history.csv`, `qd_archive.csv`, and `search_summary.json`.
pub fn read_and_reconcile_historical_w282(repo_root: &Path) -> Result<Value> {
    let jevl_path = resolve_safe_relpath(
        repo_root,
        "evidence/rs_v2/w282_ultrarunner/search/evidence.jevl",
    )?;
    let pop_path = resolve_safe_relpath(
        repo_root,
        "evidence/rs_v2/w282_ultrarunner/search/all_evals.json",
    )?;
    let csv_path = resolve_safe_relpath(
        repo_root,
        "evidence/rs_v2/w282_ultrarunner/search/all_evals.csv",
    )?;
    let hist_csv_path = resolve_safe_relpath(
        repo_root,
        "evidence/rs_v2/w282_ultrarunner/search/history.csv",
    )?;
    let qd_csv_path = resolve_safe_relpath(
        repo_root,
        "evidence/rs_v2/w282_ultrarunner/search/qd_archive.csv",
    )?;
    let sum_path = resolve_safe_relpath(
        repo_root,
        "evidence/rs_v2/w282_ultrarunner/search_summary.json",
    )?;

    let jevl_bytes = read_bounded_file(&jevl_path, HARD_SEGMENT_CEILING_BYTES)?;
    if sha256_bytes(&jevl_bytes) != HISTORICAL_JEVL_SHA256
        || jevl_bytes.len() != 64_162
        || hex_bytes(&jevl_bytes[..16]) != HISTORICAL_JEVL_FIRST16_HEX
    {
        return Err(
            "corpus: historical evidence.jevl SHA-256, header, or byte length mismatch".into(),
        );
    }
    let frames = parse_jev1_frames(&jevl_bytes)?;
    if frames.len() != 19 {
        return Err(format!("corpus: expected 19 JEV1 frames, found {}", frames.len()).into());
    }

    let mut total_jevl_events = 0u64;
    let mut run_start_events = 0u64;
    let mut run_end_events = 0u64;
    let mut births_by_id = BTreeMap::new();
    let mut fem_evaluated_births = 0u64;
    let mut not_evaluated_births = 0u64;
    let mut legal_births = 0u64;
    let mut slot_occurrences = BTreeMap::<(u64, u64, u64), u64>::new();
    let mut elite_reappearances = 0u64;
    let mut migration_events = 0u64;
    let mut migration_copied_ids = 0u64;
    let mut migration_generations = BTreeSet::new();
    let mut gen_end_events = 0u64;

    for frame in &frames {
        let evs = frame["events"]
            .as_array()
            .ok_or("corpus: missing JEV1 frame events array")?;
        for ev in evs {
            total_jevl_events += 1;
            match ev["ev"].as_str().ok_or("corpus: missing JEV1 ev tag")? {
                "run_start" => run_start_events += 1,
                "run_end" => run_end_events += 1,
                "eval" => {
                    let id = ev["id"].as_u64().ok_or("corpus: missing eval id")?;
                    let gen = ev["gen"].as_u64().ok_or("corpus: missing eval gen")?;
                    let island = ev["island"].as_u64().ok_or("corpus: missing eval island")?;
                    let raw_status = ev["fem_status"]
                        .as_str()
                        .ok_or("corpus: missing eval fem_status")?;
                    let adapted_status =
                        crate::population::adapt_historical_jevl_fem_status(raw_status)?;
                    match adapted_status {
                        "complete" => fem_evaluated_births += 1,
                        "not_evaluated" => not_evaluated_births += 1,
                        _ => unreachable!(),
                    }
                    if ev["legal"].as_bool() == Some(true) {
                        legal_births += 1;
                    }
                    *slot_occurrences.entry((gen, island, id)).or_default() += 1;
                    if births_by_id.insert(id, ev.clone()).is_some() {
                        return Err(format!("corpus: duplicate historical eval id {id}").into());
                    }
                }
                "elite" => {
                    let id = ev["id"].as_u64().ok_or("corpus: missing elite id")?;
                    let gen = ev["gen"].as_u64().ok_or("corpus: missing elite gen")?;
                    let island = ev["island"]
                        .as_u64()
                        .ok_or("corpus: missing elite island")?;
                    elite_reappearances += 1;
                    *slot_occurrences.entry((gen, island, id)).or_default() += 1;
                }
                "migration" => {
                    migration_events += 1;
                    let g = ev["gen"].as_u64().ok_or("corpus: missing migration gen")?;
                    migration_generations.insert(g);
                    let ids = ev["ids"]
                        .as_array()
                        .ok_or("corpus: missing migration ids")?;
                    migration_copied_ids += ids.len() as u64;
                }
                "gen_end" => {
                    gen_end_events += 1;
                }
                other => return Err(format!("corpus: unexpected JEV1 event {other}").into()),
            }
        }
    }

    let member_occurrences: u64 = slot_occurrences.values().sum();
    let duplicate_slot_keys = slot_occurrences.values().filter(|&&c| c > 1).count() as u64;

    let pop_bytes = read_bounded_file(&pop_path, HARD_SEGMENT_CEILING_BYTES)?;
    if sha256_bytes(&pop_bytes) != HISTORICAL_ALL_EVALS_JSON_SHA256 {
        return Err("corpus: historical all_evals.json SHA-256 mismatch".into());
    }
    let all_evals_json: Vec<Value> = serde_json::from_slice(&pop_bytes)?;
    if all_evals_json.len() != 1984 {
        return Err("corpus: historical all_evals.json length mismatch".into());
    }

    let csv_bytes = read_bounded_file(&csv_path, HARD_SEGMENT_CEILING_BYTES)?;
    if sha256_bytes(&csv_bytes) != HISTORICAL_ALL_EVALS_CSV_SHA256 {
        return Err("corpus: historical all_evals.csv SHA-256 mismatch".into());
    }
    let csv_str = std::str::from_utf8(&csv_bytes)?;
    let mut csv_lines = csv_str.lines();
    let header = csv_lines
        .next()
        .ok_or("corpus: empty historical all_evals.csv")?;
    let header_cols: Vec<&str> = header.split(',').collect();
    if header_cols.len() != 146 || header_cols[17] != "cp00" || header_cols[145] != "material" {
        return Err(format!(
            "corpus: expected 146 columns (17 scalar + 128 genes + material) in historical all_evals.csv, found {}",
            header_cols.len()
        )
        .into());
    }
    let mut csv_rows = 0u64;
    let mut distinct_gene_vectors = BTreeSet::new();
    for line in csv_lines {
        if line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() != 146 {
            return Err("corpus: malformed row width in historical all_evals.csv".into());
        }
        let id: u64 = cols[0].parse()?;
        let gen: u64 = cols[1].parse()?;
        let birth = births_by_id
            .get(&id)
            .ok_or_else(|| format!("corpus: CSV id {id} missing from JEV1 births"))?;
        if birth["gen"].as_u64() != Some(gen) {
            return Err(format!("corpus: CSV gen mismatch for birth id {id}").into());
        }
        let gene_vec = cols[17..].join(",");
        distinct_gene_vectors.insert(gene_vec);
        csv_rows += 1;
    }

    let hist_csv_bytes = read_bounded_file(&hist_csv_path, HARD_SEGMENT_CEILING_BYTES)?;
    if sha256_bytes(&hist_csv_bytes) != HISTORICAL_HISTORY_CSV_SHA256 {
        return Err("corpus: historical history.csv SHA-256 mismatch".into());
    }
    let hist_rows = std::str::from_utf8(&hist_csv_bytes)?
        .lines()
        .skip(1)
        .filter(|l| !l.trim().is_empty())
        .count();

    let qd_csv_bytes = read_bounded_file(&qd_csv_path, HARD_SEGMENT_CEILING_BYTES)?;
    if sha256_bytes(&qd_csv_bytes) != HISTORICAL_QD_ARCHIVE_CSV_SHA256 {
        return Err("corpus: historical qd_archive.csv SHA-256 mismatch".into());
    }
    let qd_rows = std::str::from_utf8(&qd_csv_bytes)?
        .lines()
        .skip(1)
        .filter(|l| !l.trim().is_empty())
        .count();

    let sum_bytes = read_bounded_file(&sum_path, HARD_SEGMENT_CEILING_BYTES)?;
    if sha256_bytes(&sum_bytes) != HISTORICAL_SUMMARY_JSON_SHA256 {
        return Err("corpus: historical search_summary.json SHA-256 mismatch".into());
    }
    let sum_val: Value = serde_json::from_slice(&sum_bytes)?;
    if sum_val["evaluations"].as_u64() != Some(1984) {
        return Err("corpus: historical search_summary.json evaluations != 1984".into());
    }

    if total_jevl_events != 2207
        || run_start_events != 1
        || run_end_events != 1
        || births_by_id.len() != 1984
        || csv_rows != 1984
        || hist_rows != 68
        || qd_rows != 12
        || fem_evaluated_births != 1455
        || not_evaluated_births != 529
        || legal_births != 446
        || member_occurrences != 2176
        || duplicate_slot_keys != 9
        || elite_reappearances != 192
        || migration_events != 12
        || migration_copied_ids != 24
        || migration_generations != BTreeSet::from([4, 8, 12])
        || gen_end_events != 17
        || distinct_gene_vectors.len() != 1965
    {
        return Err("corpus: historical w282 byte-level reconciliation counts mismatch".into());
    }

    Ok(json!({
        "schema": "jarambit.historical_w282_byte_reconciliation.v1",
        "jevl_frames": frames.len(),
        "jevl_bytes": jevl_bytes.len(),
        "jevl_first16_hex": HISTORICAL_JEVL_FIRST16_HEX,
        "jevl_events": total_jevl_events,
        "births": births_by_id.len(),
        "csv_rows": csv_rows,
        "history_csv_rows": hist_rows,
        "qd_archive_csv_rows": qd_rows,
        "fem_evaluated_births": fem_evaluated_births,
        "not_evaluated_births": not_evaluated_births,
        "legal_births": legal_births,
        "membership_occurrences": member_occurrences,
        "duplicate_slot_keys": duplicate_slot_keys,
        "elite_reappearances": elite_reappearances,
        "migration_events": migration_events,
        "migration_copied_ids": migration_copied_ids,
        "migration_generations": migration_generations.into_iter().collect::<Vec<_>>(),
        "gen_end_events": gen_end_events,
        "distinct_serialized_gene_vectors": distinct_gene_vectors.len(),
    }))
}

fn collect_round_birth_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    if !dir.exists() {
        return Ok(());
    }
    let mut entries: Vec<_> = fs::read_dir(dir)?.collect::<std::result::Result<Vec<_>, _>>()?;
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let p = entry.path();
        if p.is_dir() {
            collect_round_birth_files(&p, out)?;
        } else if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
            if name.starts_with("round_") && name.ends_with("_births.json") {
                out.push(p);
            }
        }
    }
    Ok(())
}

/// Real byte-reading newer corpus reader (`JR-R1` / `JR-EI-P2-T04`).
/// Walks all `57` `round_*_births.json` and `round_*_screen.json` files across `24` run
/// directories, performing the 1-based `(_cXX -> genomes[XX - 1], family, certificate_identity)`
/// join and computing the 0-based negative-control misjoin count (`2,247`).
pub fn read_and_join_newer_corpus(repo_root: &Path) -> Result<Value> {
    let families = [
        "evidence/evolution_waves",
        "evidence/evolution_plateau",
        "evidence/evolve_certifiable/automatic",
    ];
    let mut birth_files = Vec::new();
    for fam in families {
        let dir = resolve_safe_relpath(repo_root, fam)?;
        collect_round_birth_files(&dir, &mut birth_files)?;
    }
    birth_files.sort();
    if birth_files.len() != 57 {
        return Err(format!(
            "corpus: expected 57 round_*_births.json files, found {}",
            birth_files.len()
        )
        .into());
    }

    let mut run_dirs = BTreeSet::new();
    let mut total_births = 0u64;
    let mut total_screens = 0u64;
    let mut total_exact_1based_joins = 0u64;
    let mut total_0based_misjoins = 0u64;
    let mut by_family: BTreeMap<String, (u64, u64, u64, u64)> = BTreeMap::new();
    let mut screened_points: Vec<Value> = Vec::with_capacity(2259);
    let mut unscreened_points: Vec<Value> = Vec::with_capacity(1390);

    for (file_idx, bpath) in birth_files.iter().enumerate() {
        let rel = bpath
            .strip_prefix(repo_root)
            .map_err(|_| "corpus: birth path outside repo_root")?
            .to_str()
            .ok_or("corpus: non-utf8 path")?
            .to_string();
        let fam = families
            .iter()
            .find(|&&f| rel.starts_with(f))
            .copied()
            .ok_or("corpus: unknown family prefix")?
            .to_string();
        if let Some(parent) = Path::new(&rel).parent().and_then(|p| p.to_str()) {
            run_dirs.insert(parent.to_string());
        }
        let b_val: Value =
            serde_json::from_slice(&read_bounded_file(bpath, HARD_SEGMENT_CEILING_BYTES)?)?;
        let births_vec = b_val["genomes"]
            .as_array()
            .ok_or("corpus: missing genomes array in round_*_births.json")?;
        let s_name = bpath
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace("_births.json", "_screen.json");
        let spath = bpath.with_file_name(s_name);
        let mut screens_count = 0u64;
        let mut screened_cidxs = BTreeSet::new();
        if spath.exists() {
            let s_arr: Vec<Value> =
                serde_json::from_slice(&read_bounded_file(&spath, HARD_SEGMENT_CEILING_BYTES)?)?;
            for sv in &s_arr {
                screens_count += 1;
                let lbl = sv["label"]
                    .as_str()
                    .ok_or("corpus: missing label in screen row")?;
                let (_, c_suffix) = lbl
                    .rsplit_once("_c")
                    .ok_or_else(|| format!("corpus: missing _cXX suffix in label {lbl}"))?;
                let cidx: usize = c_suffix.parse()?;
                let cid = sv["certificate_identity"]
                    .as_str()
                    .ok_or("corpus: missing certificate_identity in screen row")?;
                let sfam = sv["family"]
                    .as_str()
                    .ok_or("corpus: missing family in screen row")?;
                if cidx == 0 || cidx > births_vec.len() {
                    return Err("corpus: 1-based _cXX candidate index out of bounds".into());
                }
                let bv = &births_vec[cidx - 1];
                if bv["certificate_identity"].as_str() != Some(cid)
                    || bv["family"].as_str() != Some(sfam)
                {
                    return Err("corpus: 1-based birth/screen join mismatch".into());
                }
                let deficit = sv["screen_deficit"]
                    .as_f64()
                    .filter(|v| v.is_finite())
                    .ok_or("corpus: missing or non-finite screen_deficit")?;
                let mass_g = sv["mass_g"]
                    .as_f64()
                    .filter(|v| v.is_finite())
                    .ok_or("corpus: missing or non-finite mass_g")?;
                screened_cidxs.insert(cidx);
                screened_points.push(json!([file_idx, cidx, deficit, mass_g]));
                total_exact_1based_joins += 1;
                if cidx < births_vec.len() {
                    if births_vec[cidx]["certificate_identity"].as_str() != Some(cid)
                        || births_vec[cidx]["family"].as_str() != Some(sfam)
                    {
                        total_0based_misjoins += 1;
                    }
                } else {
                    total_0based_misjoins += 1;
                }
            }
        }
        for cidx in 1..=births_vec.len() {
            if !screened_cidxs.contains(&cidx) {
                unscreened_points.push(json!([file_idx, cidx]));
            }
        }
        let b_cnt = births_vec.len() as u64;
        total_births += b_cnt;
        total_screens += screens_count;
        let entry = by_family.entry(fam).or_insert((0, 0, 0, 0));
        entry.0 += 1;
        entry.1 += b_cnt;
        entry.2 += screens_count;
        entry.3 += b_cnt.saturating_sub(screens_count);
    }

    let missing_screens = total_births
        .checked_sub(total_screens)
        .ok_or("corpus: screen overflow")?;
    if run_dirs.len() != 24
        || total_births != 3649
        || total_screens != 2259
        || missing_screens != 1390
        || total_exact_1based_joins != 2259
        || total_0based_misjoins != 2247
        || screened_points.len() != 2259
        || unscreened_points.len() != 1390
    {
        return Err(
            "corpus: newer corpus raw file join totals differ from admitted 57/24/3649/2259/1390/2247"
                .into(),
        );
    }

    Ok(json!({
        "schema": "jarambit.newer_corpus_byte_join.v1",
        "birth_files": birth_files.len(),
        "run_directories": run_dirs.len(),
        "birth_events": total_births,
        "joined_screen_rows": total_screens,
        "unscreened_birth_events": missing_screens,
        "zero_based_misjoins_rejected": total_0based_misjoins,
        "screened_points": screened_points,
        "unscreened_points": unscreened_points,
    }))
}

/// Scan all 200 `.csv.zst` segments (`1,954,560` rows across `Run 2`..`Run 11`) using the
/// full admitted `verify_and_decode_gen800_archive_with_callback` reader (verifying index,
/// manifest, compressed segment, logical decompressed, and whole-run SHA-256 digests plus
/// 146-column schema, boolean `legal` vocabulary, unique IDs, and 801 generations) and write
/// the content-bound high-resolution scatter cache to
/// `docs/paper/generated/population/gen800_scatter_cache.v1.json`.
pub fn populate_gen800_scatter_grid(repo_root: &Path) -> Result<Value> {
    let archive_root = resolve_archive_path(
        repo_root,
        "artifacts/versioned-evolution-archives-gen800-population-v1",
    )?;

    let ymin_asinh = (-215.0_f64).asinh();
    let ymax_asinh = (1.20_f64).asinh();
    let full_y_bins = 440usize;
    let detail_y_bins = 260usize;
    let detail_min = -0.60_f64;
    let detail_max = 1.16_f64;

    let mut full_legal = vec![vec![0u32; full_y_bins]; 801];
    let mut full_penalty = vec![vec![0u32; full_y_bins]; 801];
    let mut detail_legal = vec![vec![0u32; detail_y_bins]; 801];
    let mut detail_penalty = vec![vec![0u32; detail_y_bins]; 801];

    let mut numeric_scored_points = 0u64;
    let mut detail_window_points = 0u64;
    let mut below_detail_window_points = 0u64;
    let mut excluded_or_invalid_points = 0u64;

    let mut on_row = |_run_idx: usize, _id: u64, gen: u64, fit: f64, legal: bool| -> Result<()> {
        if gen > 800 || !(-215.0..=1.20).contains(&fit) {
            excluded_or_invalid_points += 1;
            return Err(
                format!("corpus: out-of-bounds native scatter point gen={gen}, fit={fit}").into(),
            );
        }
        let g = gen as usize;
        let norm_full = ((fit.asinh() - ymin_asinh) / (ymax_asinh - ymin_asinh)).clamp(0.0, 1.0);
        let yb_full =
            ((norm_full * (full_y_bins - 1) as f64).round() as usize).min(full_y_bins - 1);
        if legal {
            full_legal[g][yb_full] += 1;
        } else {
            full_penalty[g][yb_full] += 1;
        }

        if fit >= detail_min {
            let norm_det = ((fit - detail_min) / (detail_max - detail_min)).clamp(0.0, 1.0);
            let yb_det = 1
                + ((norm_det * (detail_y_bins - 2) as f64).round() as usize).min(detail_y_bins - 2);
            if legal {
                detail_legal[g][yb_det] += 1;
            } else {
                detail_penalty[g][yb_det] += 1;
            }
            detail_window_points += 1;
        } else {
            if legal {
                detail_legal[g][0] += 1;
            } else {
                detail_penalty[g][0] += 1;
            }
            below_detail_window_points += 1;
        }

        numeric_scored_points += 1;
        Ok(())
    };

    let adm = verify_and_decode_gen800_archive_with_callback(
        &archive_root,
        Some(GEN800_POPULATION_INDEX_SHA256),
        None,
        Some(&mut on_row),
    )?;

    if numeric_scored_points != 1_954_560
        || excluded_or_invalid_points != 0
        || detail_window_points + below_detail_window_points != 1_954_560
    {
        return Err(format!(
            "corpus: scatter conservation mismatch: numeric={numeric_scored_points}, excluded={excluded_or_invalid_points}"
        )
        .into());
    }

    let mut full_pixels = Vec::new();
    for g in 0..=800usize {
        for yb in 0..full_y_bins {
            let l = full_legal[g][yb];
            let p = full_penalty[g][yb];
            if l > 0 || p > 0 {
                full_pixels.push(json!([g, yb, l, p]));
            }
        }
    }

    let mut detail_pixels = Vec::new();
    for g in 0..=800usize {
        for yb in 0..detail_y_bins {
            let l = detail_legal[g][yb];
            let p = detail_penalty[g][yb];
            if l > 0 || p > 0 {
                detail_pixels.push(json!([g, yb, l, p]));
            }
        }
    }

    let cache_doc = json!({
        "schema": "jarambit.gen800_scatter_cache.v1",
        "admitted_index_sha256": adm["index_sha256"],
        "runs_verified": adm["runs_verified"],
        "segments_decoded": adm["segments_decoded"],
        "rows_decoded": adm["rows_decoded"],
        "full_archive_decoded": adm["full_archive_decoded"],
        "full_asinh_view": {
            "x_generations": 801,
            "y_asinh_bins": full_y_bins,
            "rendered_raster_px": [802, full_y_bins],
            "fit_domain_min": -215.0,
            "fit_domain_max": 1.20,
            "numeric_scored_points": numeric_scored_points,
            "unknown_band_points": 0,
            "excluded_or_invalid_points": excluded_or_invalid_points,
            "nonzero_bins": full_pixels.len(),
            "tuple_schema": "[generation_0_to_800, y_asinh_bin_0_to_439, legal_true_count, legal_false_count]"
        },
        "upper_score_detail_view": {
            "x_generations": 801,
            "y_linear_bins": detail_y_bins,
            "rendered_raster_px": [802, detail_y_bins],
            "linear_fit_domain_min": detail_min,
            "linear_fit_domain_max": detail_max,
            "numeric_scored_points_in_linear_window": detail_window_points,
            "below_linear_window_tail_strip_points": below_detail_window_points,
            "total_conserved_points": numeric_scored_points,
            "unknown_band_points": 0,
            "excluded_or_invalid_points": excluded_or_invalid_points,
            "nonzero_bins": detail_pixels.len(),
            "tuple_schema": "[generation_0_to_800, y_bin_0_tail_or_1_to_259_linear, legal_true_count, legal_false_count]"
        },
        "full_pixels_801x440": full_pixels,
        "detail_pixels_801x260": detail_pixels
    });

    let cache_path = resolve_safe_relpath(
        repo_root,
        "docs/paper/generated/population/gen800_scatter_cache.v1.json",
    )?;
    fs::write(
        &cache_path,
        serde_json::to_string_pretty(&cache_doc)? + "\n",
    )?;
    Ok(json!({
        "schema": cache_doc["schema"],
        "admitted_index_sha256": cache_doc["admitted_index_sha256"],
        "full_asinh_view": cache_doc["full_asinh_view"],
        "upper_score_detail_view": cache_doc["upper_score_detail_view"]
    }))
}

/// Read-only contract, index-binding, transform-domain, raster-dimension, and point-conservation
/// validator for `docs/paper/generated/population/gen800_scatter_cache.v1.json` (`SC-DEC-183AJ`).
pub fn validate_gen800_scatter_cache(
    scatter_cache: &Value,
    expected_index_sha256: &str,
) -> Result<Value> {
    if scatter_cache["schema"].as_str() != Some("jarambit.gen800_scatter_cache.v1") {
        return Err("corpus: unsupported gen800_scatter_cache schema".into());
    }
    let idx_sha = sha256_hex(
        &scatter_cache["admitted_index_sha256"],
        "admitted_index_sha256",
    )?;
    if idx_sha != GEN800_POPULATION_INDEX_SHA256 || idx_sha != expected_index_sha256 {
        return Err(format!(
            "corpus: scatter cache admitted_index_sha256 mismatch: got {idx_sha}, expected {expected_index_sha256}"
        )
        .into());
    }
    if scatter_cache["runs_verified"].as_u64() != Some(10)
        || scatter_cache["segments_decoded"].as_u64() != Some(200)
        || scatter_cache["rows_decoded"].as_u64() != Some(1_954_560)
        || scatter_cache["full_archive_decoded"].as_bool() != Some(true)
    {
        return Err("corpus: scatter cache archive admission counts mismatch".into());
    }

    let fv = &scatter_cache["full_asinh_view"];
    if fv["x_generations"].as_u64() != Some(801)
        || fv["y_asinh_bins"].as_u64() != Some(440)
        || fv["rendered_raster_px"] != json!([802, 440])
        || fv["fit_domain_min"].as_f64() != Some(-215.0)
        || fv["fit_domain_max"].as_f64() != Some(1.20)
        || fv["numeric_scored_points"].as_u64() != Some(1_954_560)
        || fv["unknown_band_points"].as_u64() != Some(0)
        || fv["excluded_or_invalid_points"].as_u64() != Some(0)
        || fv["tuple_schema"].as_str()
            != Some(
                "[generation_0_to_800, y_asinh_bin_0_to_439, legal_true_count, legal_false_count]",
            )
    {
        return Err(
            "corpus: scatter cache full_asinh_view metadata or rendered_raster_px mismatch".into(),
        );
    }

    let dv = &scatter_cache["upper_score_detail_view"];
    if dv["x_generations"].as_u64() != Some(801)
        || dv["y_linear_bins"].as_u64() != Some(260)
        || dv["rendered_raster_px"] != json!([802, 260])
        || dv["linear_fit_domain_min"].as_f64() != Some(-0.60)
        || dv["linear_fit_domain_max"].as_f64() != Some(1.16)
        || dv["numeric_scored_points_in_linear_window"].as_u64() != Some(727_077)
        || dv["below_linear_window_tail_strip_points"].as_u64() != Some(1_227_483)
        || dv["total_conserved_points"].as_u64() != Some(1_954_560)
        || dv["unknown_band_points"].as_u64() != Some(0)
        || dv["excluded_or_invalid_points"].as_u64() != Some(0)
        || dv["tuple_schema"].as_str()
            != Some("[generation_0_to_800, y_bin_0_tail_or_1_to_259_linear, legal_true_count, legal_false_count]")
    {
        return Err("corpus: scatter cache upper_score_detail_view metadata or rendered_raster_px mismatch".into());
    }

    let full_bins = scatter_cache["full_pixels_801x440"]
        .as_array()
        .ok_or("corpus: missing full_pixels_801x440 array")?;
    let detail_bins = scatter_cache["detail_pixels_801x260"]
        .as_array()
        .ok_or("corpus: missing detail_pixels_801x260 array")?;
    if fv["nonzero_bins"].as_u64() != Some(full_bins.len() as u64)
        || dv["nonzero_bins"].as_u64() != Some(detail_bins.len() as u64)
    {
        return Err("corpus: scatter cache nonzero_bins count mismatch".into());
    }

    let (mut f_leg, mut f_pen) = (0u64, 0u64);
    for b in full_bins {
        let arr = b
            .as_array()
            .ok_or("corpus: non-array tuple in full_pixels_801x440")?;
        if arr.len() != 4 {
            return Err("corpus: invalid tuple width in full_pixels_801x440".into());
        }
        let g = arr[0]
            .as_u64()
            .ok_or("corpus: invalid gen in full_pixels_801x440")?;
        let yb = arr[1]
            .as_u64()
            .ok_or("corpus: invalid y_bin in full_pixels_801x440")?;
        let l = arr[2]
            .as_u64()
            .ok_or("corpus: invalid legal_count in full_pixels_801x440")?;
        let p = arr[3]
            .as_u64()
            .ok_or("corpus: invalid penalty_count in full_pixels_801x440")?;
        if g > 800 || yb >= 440 || (l == 0 && p == 0) {
            return Err("corpus: out-of-range or zero bin in full_pixels_801x440".into());
        }
        f_leg += l;
        f_pen += p;
    }

    let (mut d_leg, mut d_pen, mut d_win, mut d_tail) = (0u64, 0u64, 0u64, 0u64);
    for b in detail_bins {
        let arr = b
            .as_array()
            .ok_or("corpus: non-array tuple in detail_pixels_801x260")?;
        if arr.len() != 4 {
            return Err("corpus: invalid tuple width in detail_pixels_801x260".into());
        }
        let g = arr[0]
            .as_u64()
            .ok_or("corpus: invalid gen in detail_pixels_801x260")?;
        let yb = arr[1]
            .as_u64()
            .ok_or("corpus: invalid y_bin in detail_pixels_801x260")?;
        let l = arr[2]
            .as_u64()
            .ok_or("corpus: invalid legal_count in detail_pixels_801x260")?;
        let p = arr[3]
            .as_u64()
            .ok_or("corpus: invalid penalty_count in detail_pixels_801x260")?;
        if g > 800 || yb >= 260 || (l == 0 && p == 0) {
            return Err("corpus: out-of-range or zero bin in detail_pixels_801x260".into());
        }
        d_leg += l;
        d_pen += p;
        if yb == 0 {
            d_tail += l + p;
        } else {
            d_win += l + p;
        }
    }

    if f_leg != 754_236
        || f_pen != 1_200_324
        || f_leg + f_pen != 1_954_560
        || d_leg != 754_236
        || d_pen != 1_200_324
        || d_win != 727_077
        || d_tail != 1_227_483
    {
        return Err("corpus: scatter cache bin-count conservation mismatch".into());
    }

    Ok(json!({
        "schema": "jarambit.gen800_scatter_cache_validation.v1",
        "admitted_index_sha256": idx_sha,
        "full_rendered_raster_px": [802, 440],
        "detail_rendered_raster_px": [802, 260],
        "conserved_numeric_points": 1_954_560,
        "conserved_legal_points": 754_236,
        "conserved_penalty_points": 1_200_324,
        "detail_window_points": 727_077,
        "detail_tail_strip_points": 1_227_483,
    }))
}

fn decompress_segment_bytes(raw: &[u8], expected_logical_bytes: u64) -> Result<Vec<u8>> {
    if expected_logical_bytes == 0 || expected_logical_bytes > HARD_LOGICAL_SEGMENT_CEILING_BYTES {
        return Err(format!(
            "corpus: declared logical_bytes {expected_logical_bytes} violates ceiling"
        )
        .into());
    }
    if raw.len() >= 4 && raw[0..4] == [0x28, 0xB5, 0x2F, 0xFD] {
        let decoder = ruzstd::decoding::StreamingDecoder::new(raw)
            .map_err(|e| format!("corpus: ruzstd init failed: {e}"))?;
        let mut out = Vec::with_capacity((expected_logical_bytes as usize).min(33_554_432));
        decoder
            .take(expected_logical_bytes.saturating_add(1))
            .read_to_end(&mut out)
            .map_err(|e| format!("corpus: ruzstd decode failed: {e}"))?;
        if (out.len() as u64) > expected_logical_bytes {
            return Err("corpus: zstd segment logical length exceeded declared bound".into());
        }
        Ok(out)
    } else {
        Err("corpus: segment missing zstd frame magic (0xFD2FB528)".into())
    }
}

/// Real byte-reading `.csv.zst` archive verifier and CSV schema/ID/generation decoder
/// (`JR-R1` / `JR-EI-P1-T08` / `JR-EI-P2-T05`).
///
/// If `max_segments_per_run` is `None`, decodes all 200 `.csv.zst` segments (`1,954,560` rows)
/// and verifies whole-run `original_csv_sha256` and `decompressed_concat_sha256` plus full row,
/// unique-ID, and 801-generation counts.
pub fn verify_and_decode_gen800_archive(
    archive_root: &Path,
    expected_index_sha256: Option<&str>,
    max_segments_per_run: Option<usize>,
) -> Result<Value> {
    verify_and_decode_gen800_archive_with_callback(
        archive_root,
        expected_index_sha256,
        max_segments_per_run,
        None,
    )
}

/// Callback arguments are archive run ordinal, original CSV id, generation, fit, legal.
/// IDs remain scoped to their run; row and source-byte admission is unchanged.
pub fn verify_and_decode_gen800_archive_with_callback(
    archive_root: &Path,
    expected_index_sha256: Option<&str>,
    max_segments_per_run: Option<usize>,
    mut row_callback: Option<&mut dyn FnMut(usize, u64, u64, f64, bool) -> Result<()>>,
) -> Result<Value> {
    let idx_path = resolve_safe_relpath(archive_root, "index.gen800_population.sha256.json")?;
    let idx_bytes = read_bounded_file(&idx_path, HARD_SEGMENT_CEILING_BYTES)?;
    let actual_idx_sha = sha256_bytes(&idx_bytes);
    if let Some(exp) = expected_index_sha256 {
        if actual_idx_sha != exp {
            return Err(format!(
                "corpus: stale or mismatched gen800 index sha256: expected {exp}, got {actual_idx_sha}"
            )
            .into());
        }
    }
    let pop_index: Value = serde_json::from_slice(&idx_bytes)?;
    let _meta_ok = validate_gen800_population_accession(&pop_index)?;

    // Build indexed file map and verify no unsafe paths or duplicate relative_paths.
    let files_arr = pop_index["files"]
        .as_array()
        .ok_or("corpus: missing files array in gen800 index")?;
    let mut indexed_files: BTreeMap<String, (u64, String)> = BTreeMap::new();
    for entry in files_arr {
        let rel = entry["relative_path"]
            .as_str()
            .ok_or("corpus: missing relative_path in index files")?;
        let _safe = resolve_safe_relpath(archive_root, rel)?;
        let bytes = entry["bytes"]
            .as_u64()
            .ok_or("corpus: missing bytes in index files")?;
        let sha = sha256_hex(&entry["sha256"], "files[].sha256")?;
        if indexed_files
            .insert(rel.to_string(), (bytes, sha))
            .is_some()
        {
            return Err(format!("corpus: duplicate indexed relative_path {rel}").into());
        }
    }

    let runs = pop_index["runs"]
        .as_array()
        .ok_or("corpus: missing runs array")?;
    let mut total_decoded_rows = 0u64;
    let mut total_decoded_segments = 0u64;

    for (run_idx, run) in runs.iter().enumerate() {
        let pkg_id = run["package_id"]
            .as_str()
            .ok_or("corpus: missing package_id")?;
        let man_rel = run["manifest_relpath"]
            .as_str()
            .ok_or("corpus: missing manifest_relpath")?;
        let man_path = resolve_safe_relpath(archive_root, man_rel)?;
        let exp_man_bytes = run["manifest_bytes"]
            .as_u64()
            .ok_or("corpus: missing manifest_bytes")?;
        let man_bytes = read_bounded_file(&man_path, exp_man_bytes)?;
        let man_sha = sha256_bytes(&man_bytes);
        let exp_man_sha = sha256_hex(&run["manifest_sha256"], "manifest_sha256")?;
        if man_sha != exp_man_sha || man_bytes.len() as u64 != exp_man_bytes {
            return Err(format!("corpus: manifest hash/size mismatch for {pkg_id}").into());
        }
        if indexed_files.get(man_rel) != Some(&(exp_man_bytes, exp_man_sha)) {
            return Err(format!("corpus: manifest {man_rel} not matching top-level index").into());
        }

        let man_doc: Value = serde_json::from_slice(&man_bytes)?;
        let segs = man_doc["segments"]
            .as_array()
            .ok_or("corpus: missing segments in run manifest")?;
        if segs.is_empty() {
            return Err(format!("corpus: empty segments in {pkg_id}").into());
        }

        // Check that the segments directory on disk contains no unindexed extra files.
        let run_dir_rel = Path::new(man_rel)
            .parent()
            .and_then(|p| p.to_str())
            .ok_or("corpus: invalid manifest parent")?;
        let seg_dir_rel = format!("{run_dir_rel}/segments");
        let seg_dir = resolve_safe_relpath(archive_root, &seg_dir_rel)?;
        let mut expected_seg_rels = BTreeSet::new();
        for s in segs {
            let fname = s["filename"]
                .as_str()
                .ok_or("corpus: missing segment filename")?;
            let srel = format!("{seg_dir_rel}/{fname}");
            let _ = resolve_safe_relpath(archive_root, &srel)?;
            if !indexed_files.contains_key(&srel) {
                return Err(format!("corpus: segment {srel} missing from top-level index").into());
            }
            expected_seg_rels.insert(srel);
        }
        for dentry in fs::read_dir(&seg_dir)? {
            let dentry = dentry?;
            let fname = dentry
                .file_name()
                .into_string()
                .map_err(|_| "corpus: non-utf8 filename in segments dir")?;
            let srel = format!("{seg_dir_rel}/{fname}");
            if !expected_seg_rels.contains(&srel) {
                return Err(
                    format!("corpus: unindexed extra file in segments directory: {srel}").into(),
                );
            }
        }

        let full_run_decode =
            max_segments_per_run.is_none() || max_segments_per_run.unwrap() >= segs.len();
        let limit = max_segments_per_run.unwrap_or(segs.len()).min(segs.len());

        let mut run_concat_hasher = Sha256::new();
        let mut run_concat_bytes = 0u64;
        let mut run_rows = 0u64;
        let mut run_legal_true = 0u64;
        let mut seen_ids = BTreeSet::new();
        let mut seen_gens = BTreeSet::new();

        for (seg_idx, s) in segs.iter().take(limit).enumerate() {
            let fname = s["filename"].as_str().unwrap();
            let srel = format!("{seg_dir_rel}/{fname}");
            let spath = resolve_safe_relpath(archive_root, &srel)?;
            let exp_stored_bytes = s["stored_bytes"]
                .as_u64()
                .ok_or("corpus: missing stored_bytes")?;
            if exp_stored_bytes == 0 || exp_stored_bytes > HARD_SEGMENT_CEILING_BYTES {
                return Err(format!("corpus: segment {srel} violates 8 MiB ceiling").into());
            }
            let comp_bytes = read_bounded_file(&spath, HARD_SEGMENT_CEILING_BYTES)
                .map_err(|e| format!("corpus: missing or unreadable segment {srel}: {e}"))?;
            let comp_len = comp_bytes.len() as u64;
            let comp_sha = sha256_bytes(&comp_bytes);
            let exp_stored_sha = sha256_hex(&s["stored_sha256"], "stored_sha256")?;
            if comp_len != exp_stored_bytes || comp_sha != exp_stored_sha {
                return Err(
                    format!("corpus: tampered compressed segment bytes/sha256 in {srel}").into(),
                );
            }
            if indexed_files.get(&srel) != Some(&(exp_stored_bytes, exp_stored_sha)) {
                return Err(format!("corpus: segment {srel} mismatches top-level index").into());
            }

            let exp_logical_bytes = s["logical_bytes"]
                .as_u64()
                .ok_or("corpus: missing logical_bytes")?;
            let exp_logical_sha =
                sha256_hex(&s["logical_payload_sha256"], "logical_payload_sha256")?;
            let decomp = decompress_segment_bytes(&comp_bytes, exp_logical_bytes)?;
            let decomp_len = decomp.len() as u64;
            let decomp_sha = sha256_bytes(&decomp);
            if decomp_len != exp_logical_bytes || decomp_sha != exp_logical_sha {
                return Err(
                    format!("corpus: decompressed payload hash/bytes mismatch in {srel}").into(),
                );
            }

            if full_run_decode {
                run_concat_hasher.update(&decomp);
                run_concat_bytes += decomp_len;
            }

            let text = std::str::from_utf8(&decomp)?;
            let mut lines = text.lines();
            let includes_header = s["includes_csv_header"].as_bool().unwrap_or(seg_idx == 0);
            if includes_header {
                let hdr = lines
                    .next()
                    .ok_or("corpus: missing CSV header in first segment")?;
                let hcols: Vec<&str> = hdr.split(',').collect();
                if hcols.len() != GEN800_EXPECTED_COLUMNS
                    || hcols[0] != "id"
                    || hcols[1] != "gen"
                    || hcols[2] != "fit"
                    || hcols[13] != "legal"
                    || hcols[17] != "cp00"
                    || hcols[145] != "material"
                {
                    return Err(format!("corpus: invalid 146-column header in {srel}").into());
                }
            }

            let mut seg_rows = 0u64;
            for line in lines {
                if line.is_empty() {
                    continue;
                }
                let mut col_count = 0usize;
                let mut id_val: Option<u64> = None;
                let mut gen_val: Option<u64> = None;
                let mut fit_val: Option<f64> = None;
                let mut legal_val: Option<bool> = None;
                for (ci, part) in line.split(',').enumerate() {
                    col_count += 1;
                    match ci {
                        0 => id_val = Some(part.parse()?),
                        1 => gen_val = Some(part.parse()?),
                        2 => {
                            let f: f64 = part.parse()?;
                            if !f.is_finite() {
                                return Err("corpus: non-finite fit in gen800 row".into());
                            }
                            fit_val = Some(f);
                        }
                        13 => {
                            legal_val = match part {
                                "true" | "1" => Some(true),
                                "false" | "0" => Some(false),
                                _ => return Err("corpus: invalid boolean in legal column".into()),
                            };
                        }
                        _ => {}
                    }
                }
                if col_count != GEN800_EXPECTED_COLUMNS {
                    return Err(format!(
                        "corpus: wrong-width gene row in {srel}: expected 146 columns, got {col_count}"
                    )
                    .into());
                }
                let id = id_val.ok_or("corpus: missing id")?;
                let gen = gen_val.ok_or("corpus: missing gen")?;
                let fit = fit_val.ok_or("corpus: missing fit")?;
                let legal = legal_val.ok_or("corpus: missing legal")?;
                if gen > 800 {
                    return Err(format!("corpus: out-of-range generation {gen} in {srel}").into());
                }
                if !seen_ids.insert(id) {
                    return Err(format!("corpus: duplicate local id {id} in {pkg_id}").into());
                }
                seen_gens.insert(gen);
                if legal {
                    run_legal_true += 1;
                }
                if let Some(cb) = row_callback.as_deref_mut() {
                    cb(run_idx, id, gen, fit, legal)?;
                }
                seg_rows += 1;
            }
            let exp_seg_rows = s["data_row_count"]
                .as_u64()
                .ok_or("corpus: missing segment data_row_count")?;
            if seg_rows != exp_seg_rows {
                return Err(format!(
                    "corpus: segment row count mismatch in {srel}: expected {exp_seg_rows}, got {seg_rows}"
                )
                .into());
            }
            run_rows += seg_rows;
            total_decoded_segments += 1;
        }

        total_decoded_rows += run_rows;

        if full_run_decode {
            let actual_concat_sha = format!("{:x}", run_concat_hasher.finalize());
            let exp_orig_sha = sha256_hex(&run["original_csv_sha256"], "original_csv_sha256")?;
            let exp_decomp_sha = sha256_hex(
                &run["decompressed_concat_sha256"],
                "decompressed_concat_sha256",
            )?;
            let exp_orig_bytes = run["original_csv_bytes"]
                .as_u64()
                .ok_or("corpus: missing original_csv_bytes")?;
            if actual_concat_sha != exp_orig_sha
                || actual_concat_sha != exp_decomp_sha
                || run_concat_bytes != exp_orig_bytes
            {
                return Err(format!(
                    "corpus: reconstructed CSV hash/bytes mismatch for {pkg_id}: actual={actual_concat_sha} ({run_concat_bytes} B) vs asserted={exp_orig_sha}"
                )
                .into());
            }
            if run_rows != run["row_count"].as_u64().unwrap_or(0)
                || seen_ids.len() as u64 != run["unique_id_count"].as_u64().unwrap_or(0)
                || seen_gens.len() as u64 != run["distinct_generations_count"].as_u64().unwrap_or(0)
                || run_legal_true != run["legal_true_count"].as_u64().unwrap_or(0)
            {
                return Err(format!(
                    "corpus: lying or mismatched row/ID/generation/legal census for {pkg_id}"
                )
                .into());
            }
        }
    }

    Ok(json!({
        "schema": "jarambit.gen800_archive_byte_verification.v1",
        "index_sha256": actual_idx_sha,
        "runs_verified": runs.len(),
        "segments_decoded": total_decoded_segments,
        "rows_decoded": total_decoded_rows,
        "full_archive_decoded": max_segments_per_run.is_none(),
    }))
}

/// Receipt metadata validator for `JR-EI-P1-T04` (does not replace `read_and_reconcile_historical_w282`).
pub fn validate_historical_population_adapter(t04_receipt: &Value) -> Result<Value> {
    if t04_receipt["task_id"].as_str() != Some("JR-EI-P1-T04") {
        return Err("corpus: expected JR-EI-P1-T04 receipt".into());
    }
    let rec = &t04_receipt["reconciliation_summary"];
    let births = rec["births_count"]
        .as_u64()
        .ok_or("corpus: missing births_count")?;
    let members = rec["population_member_occurrences_count"]
        .as_u64()
        .ok_or("corpus: missing population_member_occurrences_count")?;
    let reappearances = rec["elite_reappearances_count"]
        .as_u64()
        .ok_or("corpus: missing elite_reappearances_count")?;
    let migrations = rec["migration_events_count"]
        .as_u64()
        .ok_or("corpus: missing migration_events_count")?;
    let migration_ids = rec["migration_copied_id_occurrences_count"]
        .as_u64()
        .ok_or("corpus: missing migration_copied_id_occurrences_count")?;
    let gen_ends = rec["gen_end_events_count"]
        .as_u64()
        .ok_or("corpus: missing gen_end_events_count")?;
    let distinct_genes = rec["distinct_serialized_gene_vectors_count"]
        .as_u64()
        .ok_or("corpus: missing distinct_serialized_gene_vectors_count")?;

    if births != 1984
        || members != 2176
        || reappearances != 192
        || migrations != 12
        || migration_ids != 24
        || gen_ends != 17
        || distinct_genes != 1965
    {
        return Err("corpus: historical w282 counts do not match reconciled truth set".into());
    }
    Ok(json!({
        "schema": "jarambit.historical_population_adapter.v1",
        "cohort_id": "historical_w282_17gen_4island",
        "births": births,
        "membership_occurrences": members,
        "elite_reappearances": reappearances,
        "migration_events": migrations,
        "migration_copied_ids": migration_ids,
        "gen_end_summaries": gen_ends,
        "distinct_serialized_gene_vectors": distinct_genes,
    }))
}

/// Receipt metadata validator for `JR-EI-P1-T05` (does not replace `read_and_join_newer_corpus`).
pub fn validate_newer_corpus_join(t05_receipt: &Value) -> Result<Value> {
    if t05_receipt["task_id"].as_str() != Some("JR-EI-P1-T05") {
        return Err("corpus: expected JR-EI-P1-T05 receipt".into());
    }
    let totals = &t05_receipt["corpus_totals"];
    let birth_files = totals
        .get("birth_files")
        .or_else(|| totals.get("birth_files_count"))
        .and_then(Value::as_u64)
        .ok_or("corpus: missing birth_files")?;
    let run_dirs = totals
        .get("containing_run_directories")
        .or_else(|| totals.get("run_directories_count"))
        .and_then(Value::as_u64)
        .ok_or("corpus: missing containing_run_directories")?;
    let birth_events = totals
        .get("birth_events_total")
        .or_else(|| totals.get("total_birth_events"))
        .and_then(Value::as_u64)
        .ok_or("corpus: missing birth_events_total")?;
    let screen_rows = totals
        .get("screen_rows_total")
        .or_else(|| totals.get("joined_screen_rows"))
        .and_then(Value::as_u64)
        .ok_or("corpus: missing screen_rows_total")?;
    let missing_screens = totals
        .get("birth_minus_screen_missing_observations")
        .or_else(|| totals.get("births_without_screen_row"))
        .and_then(Value::as_u64)
        .ok_or("corpus: missing birth_minus_screen_missing_observations")?;
    let zero_based_misjoins = totals
        .get("zero_based_misjoins_negative_fixture_count")
        .and_then(Value::as_u64)
        .or_else(|| {
            t05_receipt
                .get("negative_control_zero_based_index")
                .and_then(|n| {
                    n.get("misjoined_rows_count")
                        .or_else(|| n.get("misjoined_screen_rows"))
                })
                .and_then(Value::as_u64)
        })
        .ok_or("corpus: missing zero_based_misjoins_negative_fixture_count")?;

    if birth_files != 57
        || run_dirs != 24
        || birth_events != 3649
        || screen_rows != 2259
        || missing_screens != 1390
        || birth_events != screen_rows + missing_screens
        || zero_based_misjoins != 2247
    {
        return Err(
            "corpus: newer corpus totals do not match reconciled 57/24/3649/2259/1390/2247".into(),
        );
    }
    Ok(json!({
        "schema": "jarambit.newer_corpus_join.v1",
        "birth_files": birth_files,
        "run_directories": run_dirs,
        "birth_events": birth_events,
        "joined_screen_rows": screen_rows,
        "unscreened_birth_events": missing_screens,
        "zero_based_misjoins_rejected": zero_based_misjoins,
        "families": t05_receipt.get("by_source_family").unwrap_or(&t05_receipt["family_breakdown"]),
    }))
}

/// Receipt metadata validator for `JR-EI-P1-T06` and `JR-EI-P1-T07`.
pub fn validate_latest_snapshot_corpus(t06_receipt: &Value, t07_receipt: &Value) -> Result<Value> {
    if t06_receipt["task_id"].as_str() != Some("JR-EI-P1-T06")
        || t07_receipt["task_id"].as_str() != Some("JR-EI-P1-T07")
    {
        return Err("corpus: expected JR-EI-P1-T06 and JR-EI-P1-T07 receipts".into());
    }
    let v2 = &t06_receipt["v2_staged_native_histories_summary"];
    let files_count = v2
        .get("files_count")
        .or_else(|| v2.get("admitted_files_count"))
        .and_then(Value::as_u64);
    let max_gen = v2
        .get("longest_retained_generation_in_v2")
        .or_else(|| v2.get("longest_retained_generation"))
        .and_then(Value::as_u64);
    let gaps = v2
        .get("internal_generation_gaps_count")
        .or_else(|| v2.get("total_internal_generation_gaps"))
        .and_then(Value::as_u64);
    if files_count != Some(11)
        || v2["total_rows"].as_u64() != Some(17672)
        || max_gen != Some(708)
        || gaps != Some(0)
    {
        return Err(
            "corpus: V2 staged native summary totals differ from admitted 11/17672/708".into(),
        );
    }
    let view3 = &t07_receipt["reconciled_three_view_truth_set"]
        ["view_3_latest_campaign_native_and_island_archives"];
    if view3["completed_gen_800_native_runs_across_fleet"].as_u64() != Some(10)
        || view3["completed_gen_800_total_history_rows"].as_u64() != Some(32040)
        || view3["aborted_native_histories_with_rows"].as_u64() != Some(7)
        || view3["aborted_native_histories_missing_null"].as_u64() != Some(2)
        || view3["aborted_native_history_rows"].as_u64() != Some(216)
    {
        return Err("corpus: four-worker or aborted native summary count differs".into());
    }
    Ok(json!({
        "schema": "jarambit.latest_snapshot_corpus_admission.v1",
        "v2_staged_files": 11,
        "v2_staged_rows": 17672,
        "v2_staged_max_generation": 708,
        "completed_gen800_runs": 10,
        "completed_gen800_total_rows": 32040,
        "aborted_attempts_total": 9,
        "aborted_attempts_with_csv": 7,
        "aborted_attempts_total_rows": 216,
        "all_fleet_complete": false,
    }))
}

/// Receipt metadata validator for `index.gen800_population.sha256.json`
/// (does not replace `verify_and_decode_gen800_archive`).
pub fn validate_gen800_population_accession(pop_index: &Value) -> Result<Value> {
    if pop_index["schema"].as_str()
        != Some("jarambit.evolution_archive.gen800_lossless_population_accession.v1")
    {
        return Err("corpus: unsupported gen800 lossless population accession schema".into());
    }
    let custody = &pop_index["preserved_handoff_custody_chain"];
    if custody["v1_first_observed_original"]["sha256"].as_str() != Some(V1_FIRST_OBSERVED_SHA256)
        || custody["v1_first_observed_original"]["bytes"].as_u64() != Some(269_650)
        || custody["v1_second_observed_original"]["sha256"].as_str()
            != Some(V1_SECOND_OBSERVED_SHA256)
        || custody["v1_second_observed_original"]["bytes"].as_u64() != Some(288_343)
        || custody["v1_current_remote_handoff"]["sha256"].as_str() != Some(V1_CURRENT_REMOTE_SHA256)
        || custody["v2_frozen_handoff"]["sha256"].as_str() != Some(V2_FROZEN_HANDOFF_SHA256)
        || custody["v2_additive_erratum"]["sha256"].as_str() != Some(V2_ERRATUM_SHA256)
    {
        return Err("corpus: preserved V1/V2 handoff custody chain digest mismatch".into());
    }

    let base_t07 = &pop_index["base_t07_accession_index_preserved"];
    if base_t07["sha256"].as_str() != Some(T07_BASE_INDEX_SHA256)
        || base_t07["total_indexed_files"].as_u64() != Some(295)
        || base_t07["routine_compact_non_stl_files_count"].as_u64() != Some(271)
        || base_t07["separated_stl_source_evidence_files_count"].as_u64() != Some(24)
        || base_t07["separated_24_stl_files"]
            .as_array()
            .map_or(0, Vec::len)
            != 24
    {
        return Err("corpus: base T07 index or 24-STL separation invariant mismatch".into());
    }

    let totals = &pop_index["completed_gen800_population_totals"];
    if totals["runs_count"].as_u64() != Some(10)
        || totals["independently_censused_not_extrapolated"].as_bool() != Some(true)
        || totals["total_duplicate_ids_across_10_runs"].as_u64() != Some(0)
        || totals["columns_per_row"].as_u64() != Some(146)
        || totals["gene_and_material_columns_per_row"].as_u64() != Some(129)
        || totals["hard_segment_ceiling_bytes"].as_u64() != Some(HARD_SEGMENT_CEILING_BYTES)
    {
        return Err("corpus: gen800 population totals invariant mismatch".into());
    }

    let runs = pop_index["runs"]
        .as_array()
        .ok_or("corpus: missing gen800 runs array")?;
    if runs.len() != 10 {
        return Err("corpus: expected exactly 10 completed gen800 runs".into());
    }
    let mut pkg_ids = BTreeSet::new();
    let mut sum_rows = 0u64;
    let mut sum_raw_bytes = 0u64;
    let mut sum_comp_bytes = 0u64;
    let mut sum_segments = 0u64;

    for run in runs {
        let pkg = run["package_id"]
            .as_str()
            .filter(|s| !s.is_empty())
            .ok_or("corpus: missing package_id")?;
        if !pkg_ids.insert(pkg.to_string()) {
            return Err("corpus: duplicate gen800 package_id".into());
        }
        let orig_sha = sha256_hex(&run["original_csv_sha256"], "original_csv_sha256")?;
        let decomp_sha = sha256_hex(
            &run["decompressed_concat_sha256"],
            "decompressed_concat_sha256",
        )?;
        let orig_bytes = run["original_csv_bytes"]
            .as_u64()
            .ok_or("corpus: missing original_csv_bytes")?;
        let decomp_bytes = run["decompressed_concat_bytes"]
            .as_u64()
            .ok_or("corpus: missing decompressed_concat_bytes")?;
        if orig_sha != decomp_sha
            || orig_bytes != decomp_bytes
            || run["lossless_roundtrip_verified"].as_bool() != Some(true)
        {
            return Err("corpus: lossless round-trip digest/byte verification failed".into());
        }
        let rows = run["row_count"]
            .as_u64()
            .ok_or("corpus: missing row_count")?;
        let uniq = run["unique_id_count"]
            .as_u64()
            .ok_or("corpus: missing unique_id_count")?;
        let dups = run["duplicate_id_count"]
            .as_u64()
            .ok_or("corpus: missing duplicate_id_count")?;
        let distinct_gens = run["distinct_generations_count"]
            .as_u64()
            .ok_or("corpus: missing distinct_generations_count")?;
        if rows < distinct_gens || rows != uniq || dups != 0 {
            return Err(
                "corpus: invalid, duplicate, or impossible row/generation census in gen800 run"
                    .into(),
            );
        }
        if run["generation_range"] != json!([0, 800])
            || distinct_gens != 801
            || run["column_count"].as_u64() != Some(146)
            || run["gene_and_material_column_count"].as_u64() != Some(129)
        {
            return Err("corpus: gen800 run generation range or column schema mismatch".into());
        }
        let max_seg = run["max_segment_compressed_bytes"]
            .as_u64()
            .ok_or("corpus: missing max_segment_compressed_bytes")?;
        if max_seg == 0 || max_seg > HARD_SEGMENT_CEILING_BYTES {
            return Err("corpus: segment exceeds 8 MiB hard ceiling".into());
        }
        sum_rows += rows;
        sum_raw_bytes += orig_bytes;
        sum_comp_bytes += run["total_compressed_bytes"]
            .as_u64()
            .ok_or("corpus: missing total_compressed_bytes")?;
        sum_segments += run["segment_count"]
            .as_u64()
            .ok_or("corpus: missing segment_count")?;
    }

    if totals["total_rows_across_10_runs"].as_u64() != Some(sum_rows)
        || totals["total_uncompressed_csv_bytes"].as_u64() != Some(sum_raw_bytes)
        || totals["total_compressed_zstd_bytes"].as_u64() != Some(sum_comp_bytes)
        || totals["total_zstd_segments"].as_u64() != Some(sum_segments)
    {
        return Err("corpus: aggregate gen800 totals do not match per-run sum".into());
    }

    Ok(json!({
        "schema": "jarambit.gen800_population_accession_validation.v1",
        "runs_count": 10,
        "total_rows": sum_rows,
        "total_uncompressed_bytes": sum_raw_bytes,
        "total_compressed_bytes": sum_comp_bytes,
        "total_segments": sum_segments,
        "separated_stl_files": 24,
    }))
}

/// Automatic Rust selector and file-binding check for `docs/paper/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_20261019.json`
/// (`SC-DEC-183X` §2.1). Verifies every file path + SHA-256 pair and historical/custody hash in the repair receipt
/// directly against on-disk bytes and pinned constants so hand-typed suffix errors are impossible.
pub fn validate_repair_receipt_file_bindings(repo_root: &Path, receipt: &Value) -> Result<Value> {
    if receipt["schema"].as_str() != Some("jarambit.corpus_reader_and_svg_repair_receipt.v1") {
        return Err("corpus: unsupported repair receipt schema".into());
    }
    let fixes = &receipt["sc_dec_183p_and_183s_fixes_verified"];
    let jevl = &fixes["1_jev1_binary_framing"];
    let jevl_rel = jevl["file"]
        .as_str()
        .ok_or("corpus: missing 1_jev1_binary_framing.file")?;
    let jevl_claim_sha = sha256_hex(&jevl["sha256"], "1_jev1_binary_framing.sha256")?;
    let jevl_bytes = read_bounded_file(
        &resolve_safe_relpath(repo_root, jevl_rel)?,
        HARD_SEGMENT_CEILING_BYTES,
    )?;
    let jevl_disk_sha = sha256_bytes(&jevl_bytes);
    if jevl_claim_sha != HISTORICAL_JEVL_SHA256 || jevl_claim_sha != jevl_disk_sha {
        return Err(format!(
            "corpus: repair receipt 1_jev1_binary_framing.sha256 mismatch: claimed {jevl_claim_sha}, disk {jevl_disk_sha}"
        )
        .into());
    }

    let hist_files = &fixes["2_historical_search_files"];
    let hist_expected = [
        (
            "all_evals.json",
            "evidence/rs_v2/w282_ultrarunner/search/all_evals.json",
            HISTORICAL_ALL_EVALS_JSON_SHA256,
        ),
        (
            "all_evals.csv",
            "evidence/rs_v2/w282_ultrarunner/search/all_evals.csv",
            HISTORICAL_ALL_EVALS_CSV_SHA256,
        ),
        (
            "history.csv",
            "evidence/rs_v2/w282_ultrarunner/search/history.csv",
            HISTORICAL_HISTORY_CSV_SHA256,
        ),
        (
            "qd_archive.csv",
            "evidence/rs_v2/w282_ultrarunner/search/qd_archive.csv",
            HISTORICAL_QD_ARCHIVE_CSV_SHA256,
        ),
        (
            "search_summary.json",
            "evidence/rs_v2/w282_ultrarunner/search_summary.json",
            HISTORICAL_SUMMARY_JSON_SHA256,
        ),
    ];
    for (field, rel, expected_const) in hist_expected {
        let claimed = sha256_hex(&hist_files[field], field)?;
        let disk_bytes = read_bounded_file(
            &resolve_safe_relpath(repo_root, rel)?,
            HARD_LOGICAL_SEGMENT_CEILING_BYTES,
        )?;
        let disk_sha = sha256_bytes(&disk_bytes);
        if claimed != expected_const || claimed != disk_sha {
            return Err(format!(
                "corpus: repair receipt 2_historical_search_files.{field} mismatch: claimed {claimed}, disk {disk_sha}"
            )
            .into());
        }
    }

    let custody = &fixes["4_frozen_custody_hashes_restored"];
    for (field, expected_const) in [
        ("v1_first_observed", V1_FIRST_OBSERVED_SHA256),
        ("v1_second_observed", V1_SECOND_OBSERVED_SHA256),
        ("v1_current_remote", V1_CURRENT_REMOTE_SHA256),
        ("v2_frozen_handoff", V2_FROZEN_HANDOFF_SHA256),
        ("v2_erratum", V2_ERRATUM_SHA256),
        ("t07_base_index", T07_BASE_INDEX_SHA256),
        ("t08_gen800_index", GEN800_POPULATION_INDEX_SHA256),
    ] {
        let claimed = sha256_hex(&custody[field], field)?;
        if claimed != expected_const {
            return Err(format!(
                "corpus: repair receipt 4_frozen_custody_hashes_restored.{field} mismatch: claimed {claimed}, expected {expected_const}"
            )
            .into());
        }
    }

    let src_bindings = receipt["source_and_binary_bindings"]
        .as_object()
        .ok_or("corpus: missing source_and_binary_bindings object")?;
    let mut verified_files = 6usize;
    for (k, v) in src_bindings {
        if k.starts_with("crates/") {
            let claimed = sha256_hex(v, k)?;
            let disk_bytes = read_bounded_file(
                &resolve_safe_relpath(repo_root, k)?,
                HARD_SEGMENT_CEILING_BYTES,
            )?;
            let disk_sha = sha256_bytes(&disk_bytes);
            if claimed != disk_sha {
                return Err(format!(
                    "corpus: repair receipt source binding {k} mismatch: claimed {claimed}, disk {disk_sha}"
                )
                .into());
            }
            verified_files += 1;
        }
    }

    let out_bindings = receipt["generated_outputs_sha256"]
        .as_object()
        .ok_or("corpus: missing generated_outputs_sha256 object")?;
    for (k, v) in out_bindings {
        let claimed = sha256_hex(v, k)?;
        let disk_bytes = read_bounded_file(
            &resolve_safe_relpath(repo_root, k)?,
            HARD_LOGICAL_SEGMENT_CEILING_BYTES,
        )?;
        let disk_sha = sha256_bytes(&disk_bytes);
        if claimed != disk_sha {
            return Err(format!(
                "corpus: repair receipt generated output {k} mismatch: claimed {claimed}, disk {disk_sha}"
            )
            .into());
        }
        verified_files += 1;
    }

    Ok(json!({
        "schema": "jarambit.repair_receipt_binding_validation.v1",
        "verified_on_disk_files": verified_files,
        "verified_custody_constants": 7,
    }))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static FIXTURE_SEQ: AtomicU64 = AtomicU64::new(0);

    /// Build an uncompressed zstd frame (`magic = 0xFD2FB528`, single-segment flag,
    /// raw blocks) so unit tests can construct valid `.csv.zst` bytes in memory without
    /// external encoder binaries.
    fn encode_uncompressed_zstd_frame(payload: &[u8]) -> Vec<u8> {
        let mut out = vec![0x28, 0xB5, 0x2F, 0xFD];
        let len = payload.len() as u32;
        // Frame_Header_Descriptor: Single_Segment_Flag=1 (bit 5), FCS_Field_Size=2 (bits 6..7 = 2 -> 4 bytes) -> 0xA0
        out.push(0xA0);
        out.extend_from_slice(&len.to_le_bytes());
        let mut offset = 0usize;
        while offset < payload.len() {
            let rem = payload.len() - offset;
            let blk_len = rem.min(131_072);
            let last_bit = if offset + blk_len == payload.len() {
                1u32
            } else {
                0u32
            };
            // Block_Header: Last_Block (bit 0) | Block_Type=0 (Raw_Block, bits 1..2) | Block_Size << 3
            let hdr = last_bit | ((blk_len as u32) << 3);
            out.push((hdr & 0xFF) as u8);
            out.push(((hdr >> 8) & 0xFF) as u8);
            out.push(((hdr >> 16) & 0xFF) as u8);
            out.extend_from_slice(&payload[offset..offset + blk_len]);
            offset += blk_len;
        }
        out
    }

    fn make_valid_header_146() -> String {
        let mut cols = vec![
            "id".to_string(),
            "gen".to_string(),
            "fit".to_string(),
            "spg".to_string(),
            "mass_g".to_string(),
            "aes_pass".to_string(),
            "aes_score".to_string(),
            "hausdorff".to_string(),
            "emd".to_string(),
            "novelty".to_string(),
            "lat_rob".to_string(),
            "ring_rob".to_string(),
            "sf_hunter".to_string(),
            "legal".to_string(),
            "n_rejects".to_string(),
            "relief_frac".to_string(),
            "n_win".to_string(),
        ];
        for i in 0..120 {
            cols.push(format!("cp{i:02}"));
        }
        for name in [
            "th0", "th1", "th2", "r_b", "r_s", "k2", "k3", "k4", "material",
        ] {
            cols.push(name.to_string());
        }
        assert_eq!(cols.len(), 146);
        cols.join(",")
    }

    fn make_row_146(id: u64, gen: u64, fit: f64, legal: bool, extra_cols: isize) -> String {
        let mut cols = vec![
            id.to_string(),
            gen.to_string(),
            format!("{fit:.4}"),
            "3.3900".to_string(),
            "75.80".to_string(),
            "true".to_string(),
            "0.4079".to_string(),
            "2.285".to_string(),
            "0.0071".to_string(),
            "0.0211".to_string(),
            "257.1".to_string(),
            "13263.2".to_string(),
            "1.010".to_string(),
            if legal { "true" } else { "false" }.to_string(),
            "0".to_string(),
            "0.1100".to_string(),
            "1".to_string(),
        ];
        let gene_cols = (129isize + extra_cols).max(1) as usize;
        for i in 0..gene_cols - 1 {
            cols.push(format!("{:.4}", (i as f64) * 0.01));
        }
        cols.push("Ti6Al4V".to_string());
        cols.join(",")
    }

    pub(crate) struct TempArchive(pub(crate) PathBuf);
    impl Drop for TempArchive {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    pub(crate) fn build_minimal_801gen_archive(row_width_delta: isize) -> (TempArchive, String) {
        let root = std::env::temp_dir().join(format!(
            "jarambit-gen800-test-{}-{}",
            std::process::id(),
            FIXTURE_SEQ.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&root).unwrap();

        let mut csv = make_valid_header_146() + "\n";
        for g in 0..=800u64 {
            csv.push_str(&make_row_146(
                g,
                g,
                1.05 + (g as f64) * 0.0001,
                true,
                row_width_delta,
            ));
            csv.push('\n');
        }
        let csv_bytes = csv.into_bytes();
        let csv_sha = sha256_bytes(&csv_bytes);
        let zst_bytes = encode_uncompressed_zstd_frame(&csv_bytes);
        let zst_sha = sha256_bytes(&zst_bytes);

        let mut files_arr = Vec::new();
        let mut runs_arr = Vec::new();
        for i in 0..10 {
            let pkg = format!("pkg_{i:02}");
            let seg_dir = root.join(&pkg).join("segments");
            fs::create_dir_all(&seg_dir).unwrap();
            let seg_fname = "000000.test.csv.zst";
            let seg_rel = format!("{pkg}/segments/{seg_fname}");
            fs::write(seg_dir.join(seg_fname), &zst_bytes).unwrap();

            let man_doc = json!({
                "package_id": pkg,
                "segments": [{
                    "segment_sequence": 0,
                    "filename": seg_fname,
                    "includes_csv_header": true,
                    "data_row_count": 801,
                    "logical_bytes": csv_bytes.len(),
                    "logical_payload_sha256": csv_sha,
                    "stored_bytes": zst_bytes.len(),
                    "stored_sha256": zst_sha
                }]
            });
            let man_bytes = serde_json::to_vec_pretty(&man_doc).unwrap();
            let man_sha = sha256_bytes(&man_bytes);
            let man_rel = format!("{pkg}/population_manifest.v1.json");
            fs::write(root.join(&man_rel), &man_bytes).unwrap();

            files_arr.push(json!({
                "relative_path": man_rel,
                "bytes": man_bytes.len(),
                "sha256": man_sha,
                "kind": "run_population_manifest"
            }));
            files_arr.push(json!({
                "relative_path": seg_rel,
                "bytes": zst_bytes.len(),
                "sha256": zst_sha,
                "kind": "lossless_zstd_csv_segment"
            }));
            runs_arr.push(json!({
                "package_id": pkg,
                "manifest_relpath": man_rel,
                "manifest_sha256": man_sha,
                "manifest_bytes": man_bytes.len(),
                "original_csv_sha256": csv_sha,
                "decompressed_concat_sha256": csv_sha,
                "original_csv_bytes": csv_bytes.len(),
                "decompressed_concat_bytes": csv_bytes.len(),
                "lossless_roundtrip_verified": true,
                "row_count": 801,
                "unique_id_count": 801,
                "duplicate_id_count": 0,
                "legal_true_count": 801,
                "generation_range": [0, 800],
                "distinct_generations_count": 801,
                "column_count": 146,
                "gene_and_material_column_count": 129,
                "max_segment_compressed_bytes": zst_bytes.len(),
                "total_compressed_bytes": zst_bytes.len(),
                "segment_count": 1
            }));
        }

        let idx_doc = json!({
            "schema": "jarambit.evolution_archive.gen800_lossless_population_accession.v1",
            "preserved_handoff_custody_chain": {
                "v1_first_observed_original": { "bytes": 269650, "sha256": V1_FIRST_OBSERVED_SHA256 },
                "v1_second_observed_original": { "bytes": 288343, "sha256": V1_SECOND_OBSERVED_SHA256 },
                "v1_current_remote_handoff": { "bytes": 288343, "sha256": V1_CURRENT_REMOTE_SHA256 },
                "v2_frozen_handoff": { "bytes": 664314, "sha256": V2_FROZEN_HANDOFF_SHA256 },
                "v2_additive_erratum": { "bytes": 34300, "sha256": V2_ERRATUM_SHA256 }
            },
            "base_t07_accession_index_preserved": {
                "sha256": T07_BASE_INDEX_SHA256,
                "total_indexed_files": 295,
                "routine_compact_non_stl_files_count": 271,
                "separated_stl_source_evidence_files_count": 24,
                "separated_24_stl_files": vec![json!({}); 24]
            },
            "completed_gen800_population_totals": {
                "runs_count": 10,
                "independently_censused_not_extrapolated": true,
                "total_rows_across_10_runs": 8010,
                "total_duplicate_ids_across_10_runs": 0,
                "columns_per_row": 146,
                "gene_and_material_columns_per_row": 129,
                "total_uncompressed_csv_bytes": csv_bytes.len() * 10,
                "total_compressed_zstd_bytes": zst_bytes.len() * 10,
                "total_zstd_segments": 10,
                "hard_segment_ceiling_bytes": HARD_SEGMENT_CEILING_BYTES
            },
            "runs": runs_arr,
            "files": files_arr
        });
        let idx_bytes = serde_json::to_vec_pretty(&idx_doc).unwrap();
        let idx_sha = sha256_bytes(&idx_bytes);
        fs::write(root.join("index.gen800_population.sha256.json"), &idx_bytes).unwrap();
        (TempArchive(root), idx_sha)
    }

    #[test]
    fn row_callback_preserves_original_ids_with_their_run_namespace() {
        let (arch, index_sha) = build_minimal_801gen_archive(0);
        let mut keys = BTreeSet::new();
        let mut rows = 0u64;
        let mut on_row =
            |run: usize, id: u64, generation: u64, fit: f64, legal: bool| -> Result<()> {
                assert_eq!(id, generation);
                assert!((fit - (1.05 + generation as f64 * 0.0001)).abs() < 1e-10);
                assert!(legal);
                assert!(keys.insert((run, id)));
                rows += 1;
                Ok(())
            };
        verify_and_decode_gen800_archive_with_callback(
            &arch.0,
            Some(&index_sha),
            None,
            Some(&mut on_row),
        )
        .unwrap();
        assert_eq!(rows, 8010);
        assert!(keys.contains(&(0, 0)) && keys.contains(&(9, 0)));
        assert!(keys.contains(&(0, 800)) && keys.contains(&(9, 800)));
    }

    #[test]
    fn jr_r1_1_missing_or_tampered_segment_is_refused() {
        let (arch, idx_sha) = build_minimal_801gen_archive(0);
        assert!(verify_and_decode_gen800_archive(&arch.0, Some(&idx_sha), None).is_ok());

        // Tamper with compressed segment bytes on disk
        let seg_path = arch.0.join("pkg_00/segments/000000.test.csv.zst");
        let mut bytes = fs::read(&seg_path).unwrap();
        let last = bytes.len() - 1;
        bytes[last] ^= 0xFF;
        fs::write(&seg_path, &bytes).unwrap();
        assert!(verify_and_decode_gen800_archive(&arch.0, Some(&idx_sha), None).is_err());

        // Remove segment completely
        fs::remove_file(&seg_path).unwrap();
        assert!(verify_and_decode_gen800_archive(&arch.0, Some(&idx_sha), None).is_err());
    }

    #[test]
    fn jr_r1_2_both_asserted_hashes_changed_together_while_bytes_differ_is_refused() {
        let (arch, _) = build_minimal_801gen_archive(0);
        let idx_path = arch.0.join("index.gen800_population.sha256.json");
        let mut idx: Value = serde_json::from_slice(&fs::read(&idx_path).unwrap()).unwrap();
        let fake_hash = "c".repeat(64);
        // Change both original_csv_sha256 and decompressed_concat_sha256 together
        idx["runs"][0]["original_csv_sha256"] = json!(fake_hash);
        idx["runs"][0]["decompressed_concat_sha256"] = json!(fake_hash);
        let new_bytes = serde_json::to_vec_pretty(&idx).unwrap();
        let new_sha = sha256_bytes(&new_bytes);
        fs::write(&idx_path, &new_bytes).unwrap();

        // Metadata-only check would see matching strings, but byte-reading decoder refuses!
        assert!(verify_and_decode_gen800_archive(&arch.0, Some(&new_sha), None).is_err());
    }

    #[test]
    fn jr_r1_3_lying_row_and_generation_census_is_refused() {
        let (arch, _) = build_minimal_801gen_archive(0);
        let idx_path = arch.0.join("index.gen800_population.sha256.json");
        let mut idx: Value = serde_json::from_slice(&fs::read(&idx_path).unwrap()).unwrap();
        // Claim 100 rows while still asserting 801 distinct generations
        idx["runs"][0]["row_count"] = json!(100);
        idx["runs"][0]["unique_id_count"] = json!(100);
        assert!(validate_gen800_population_accession(&idx).is_err());
    }

    #[test]
    fn jr_r1_4_wrong_width_gene_rows_are_refused() {
        let (arch_short, sha_short) = build_minimal_801gen_archive(-1);
        let err = verify_and_decode_gen800_archive(&arch_short.0, Some(&sha_short), None)
            .unwrap_err()
            .to_string();
        assert!(err.contains("wrong-width gene row"), "{err}");
    }

    #[test]
    fn jr_r1_5_unsafe_path_and_extra_unindexed_membership_are_refused() {
        assert!(resolve_safe_relpath(Path::new("."), "../escape.csv.zst").is_err());
        assert!(resolve_safe_relpath(Path::new("."), "/etc/passwd").is_err());

        let (arch, idx_sha) = build_minimal_801gen_archive(0);
        let extra = arch.0.join("pkg_00/segments/extra_unindexed.csv.zst");
        fs::write(&extra, b"unindexed").unwrap();
        let err = verify_and_decode_gen800_archive(&arch.0, Some(&idx_sha), None)
            .unwrap_err()
            .to_string();
        assert!(err.contains("unindexed extra file"), "{err}");
    }

    #[test]
    fn jr_r1_6_stale_input_hash_is_refused() {
        let (arch, _) = build_minimal_801gen_archive(0);
        let stale_sha = "d".repeat(64);
        let err = verify_and_decode_gen800_archive(&arch.0, Some(&stale_sha), None)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("stale or mismatched gen800 index sha256"),
            "{err}"
        );
    }

    #[test]
    fn jr_r1_7_jev1_real_wire_frame_and_tampered_fnv1a_refused() {
        let jsonl = b"{\"ev\":\"elite\",\"gen\":1,\"island\":0,\"id\":5}\n";
        let zstd_frame = encode_uncompressed_zstd_frame(jsonl);
        let mut wire = Vec::new();
        wire.extend_from_slice(b"JEV1");
        wire.extend_from_slice(&(zstd_frame.len() as u32).to_le_bytes());
        wire.extend_from_slice(&fnv1a_64(&zstd_frame).to_le_bytes());
        wire.extend_from_slice(&zstd_frame);

        let parsed = parse_jev1_frames(&wire).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0]["events"][0]["ev"], "elite");

        // Tamper with FNV-1a checksum
        let mut bad_fnv = wire.clone();
        bad_fnv[8] ^= 0xFF;
        assert!(parse_jev1_frames(&bad_fnv).is_err());

        // Truncated frame
        assert!(parse_jev1_frames(&wire[..wire.len() - 1]).is_err());
    }

    #[test]
    fn jr_r1_8_decompression_logical_length_exceeded_and_symlink_escape_refused() {
        let payload = b"id,gen,fit\n0,0,1.0\n";
        let zst = encode_uncompressed_zstd_frame(payload);
        // Exact logical length succeeds
        assert!(decompress_segment_bytes(&zst, payload.len() as u64).is_ok());
        // Logical length exceeded by 1 byte is refused before unbounded allocation
        let err = decompress_segment_bytes(&zst, (payload.len() - 1) as u64)
            .unwrap_err()
            .to_string();
        assert!(err.contains("logical length exceeded"), "{err}");

        #[cfg(unix)]
        {
            let (arch, _) = build_minimal_801gen_archive(0);
            let link_path = arch.0.join("symlink_escape.json");
            std::os::unix::fs::symlink("/etc/hosts", &link_path).unwrap();
            assert!(resolve_safe_relpath(&arch.0, "symlink_escape.json").is_err());
            assert!(read_bounded_file(&link_path, 4096).is_err());
        }
    }

    #[test]
    fn sc183x_repair_receipt_binding_validator_rejects_erroneous_v1_and_accepts_repaired() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let v1_err_path = root.join(
            "docs/paper/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_V1_OBSERVED_ERRONEOUS_20261019.json",
        );
        let v1_err: Value = serde_json::from_slice(&fs::read(v1_err_path).unwrap()).unwrap();
        assert!(validate_repair_receipt_file_bindings(&root, &v1_err).is_err());

        let repaired_path =
            root.join("docs/paper/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_20261019.json");
        let repaired: Value = serde_json::from_slice(&fs::read(repaired_path).unwrap()).unwrap();
        assert!(validate_repair_receipt_file_bindings(&root, &repaired).is_ok());
    }
}
