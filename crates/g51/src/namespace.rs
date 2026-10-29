//! G5.1 namespace: decoder id, design-id prefixes and run roots.
//!
//! G5.1 keeps the G5.0 genome (schema `G5.0`, 134 genes, cohort label `bf3e37fa`), so a v6.0.0 genome row loads
//! unchanged and keeps its `kv…` id; designs born under G5.1 get `kw…` ids. Rows of the calibration design of
//! experiments get `kd…` ids and never enter a search pool.

use jarambit_g5::g5_namespace::{G3_DESIGN_ID_PREFIX, G5_DESIGN_ID_PREFIX, V1_DESIGN_ID_PREFIX};

/// The decoder generation recorded in every G5.1 record and archive.
pub const G51_DECODER: &str = "G5.1";
/// Record schema of a G5.1 evaluation (the genome inside it stays schema `G5.0`).
pub const G51_RECORD_SCHEMA: &str = "G5.1";
/// The genome's cohort label (`decoder_commit` of every G5.0 row since the G5.0 freeze), kept for G5.1 genomes so
/// v6.0.0 rows and G5.1 rows share one genome lineage and the ids of inherited designs do not change.
pub const G51_GENOME_COHORT_LABEL: &str = "bf3e37fa";
pub const G51_DESIGN_ID_PREFIX: &str = "kw";
/// Design-of-experiments rows of the hand-zone calibration study.
pub const G51_DOE_ID_PREFIX: &str = "kd";

/// True once the decoder and its calibration are frozen for the cohort of record. Until then G5.1 runs only under
/// `evo_g51_dev`, and nothing written there can become a cohort of record.
pub const G51_DECODER_FROZEN: bool = false;
pub const G51_DEV_RUN_ROOT_DIR: &str = "evo_g51_dev";
pub const G51_RUN_ROOT_DIR: &str = "evo_g51";

/// `kw<host>i<island>_g<gen>_<short_sha>`: cannot collide with v1 (`ec`), G3 (`hz`) or G5.0-born (`kv`) ids.
pub fn format_g51_design_id(host_num: u32, island: u32, gen_num: u64, genome_sha256: &str) -> String {
    let short = &genome_sha256[..genome_sha256.len().min(8)];
    format!("{G51_DESIGN_ID_PREFIX}{host_num}i{island:03}_g{gen_num:05}_{short}")
}

/// `kd<stratum>_<index>_<short_sha>` for a calibration design.
pub fn format_g51_doe_id(stratum: &str, index: usize, genome_sha256: &str) -> String {
    let short = &genome_sha256[..genome_sha256.len().min(8)];
    format!("{G51_DOE_ID_PREFIX}{stratum}_{index:04}_{short}")
}

/// A design id a G5.1 search pool may hold: born under G5.1 (`kw`) or inherited from G5.0 (`kv`). v1, G3 and
/// calibration (`kd`) ids are refused.
pub fn validate_g51_pool_design_id(design_id: &str) -> Result<(), String> {
    for (prefix, what) in [
        (V1_DESIGN_ID_PREFIX, "v1"),
        (G3_DESIGN_ID_PREFIX, "G3"),
        (G51_DOE_ID_PREFIX, "calibration"),
    ] {
        if design_id.starts_with(prefix) {
            return Err(format!("refusing {what} design id '{design_id}' in a G5.1 pool (starts with '{prefix}')"));
        }
    }
    if design_id.starts_with(G51_DESIGN_ID_PREFIX) || design_id.starts_with(G5_DESIGN_ID_PREFIX) {
        Ok(())
    } else {
        Err(format!(
            "invalid G5.1 design id '{design_id}': must start with '{G51_DESIGN_ID_PREFIX}' or '{G5_DESIGN_ID_PREFIX}'"
        ))
    }
}

/// A G5.1 run root: an absolute home path with an `evo_g51` component once frozen (`evo_g51_dev` before), never
/// under `/tmp`, `..` or the v1 `evo_continuous` tree.
pub fn validate_g51_run_root(run_root: &str) -> Result<(), String> {
    let p = std::path::Path::new(run_root);
    let abs_home = run_root.starts_with("/home/") || run_root.starts_with("/Users/");
    let forbidden = p.components().any(|c| {
        let s = c.as_os_str().to_string_lossy();
        s == "tmp" || s == ".." || s.contains("evo_continuous")
    });
    let root_ok = p.components().any(|c| {
        let s = c.as_os_str().to_string_lossy();
        if G51_DECODER_FROZEN {
            s == G51_RUN_ROOT_DIR || (s.starts_with("evo_g51_") && !s.starts_with(G51_DEV_RUN_ROOT_DIR))
        } else {
            s == G51_DEV_RUN_ROOT_DIR
        }
    });
    if abs_home && !forbidden && root_ok {
        Ok(())
    } else {
        Err(format!(
            "invalid G5.1 run root '{run_root}': must be an absolute home path with an '{}' component and never touch \
             '/tmp' or 'evo_continuous'",
            if G51_DECODER_FROZEN { G51_RUN_ROOT_DIR } else { G51_DEV_RUN_ROOT_DIR }
        ))
    }
}
