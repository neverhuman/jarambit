//! T6 Namespace Isolation & Census Filter (`plan.md` §6 T6 & §7).
//!
//! Enforces strict separation between the frozen v1 (`W_ULTRALIGHT_v1`, 128-gene, `ec<host>i<NNN>_`)
//! and G3 (`W_ERGOLIGHT_v1`, `G3.0` 179-gene, `hz<host>i<NNN>_`) evolutionary cohorts so v1 papers,
//! leaderboards, S3 prefixes, and keepalive units can never be polluted or confused.

use crate::g3_genome::{G3Genome, G3_SCHEMA};
use serde::{Deserialize, Serialize};

pub const G3_SCORE_METRIC: &str = "W_ERGOLIGHT_v1";
pub const G3_REQUIREMENTS_ID: &str = "RS-karambit-v3";
pub const G3_DESIGN_ID_PREFIX: &str = "hz";
pub const V1_DESIGN_ID_PREFIX: &str = "ec";
pub const G3_RUN_ROOT_DIR: &str = "evo_g3";
pub const V1_RUN_ROOT_DIR: &str = "evo_continuous";
pub const G3_KEEPALIVE_UNIT: &str = "jarambit-evo-g3-keepalive";
pub const V1_KEEPALIVE_UNIT: &str = "jarambit-evo-keepalive";
pub const G3_S3_PREFIX_SEGMENT: &str = "/evo-g3/";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct G3CohortConfig {
    pub schema: String,
    pub metric: String,
    pub requirements_id: String,
    pub design_id_prefix: String,
    pub run_root: String,
    pub systemd_unit: String,
    pub s3_prefix: String,
    pub allow_v1_seeds: bool,
}

impl Default for G3CohortConfig {
    fn default() -> Self {
        Self {
            schema: G3_SCHEMA.to_string(),
            metric: G3_SCORE_METRIC.to_string(),
            requirements_id: G3_REQUIREMENTS_ID.to_string(),
            design_id_prefix: G3_DESIGN_ID_PREFIX.to_string(),
            run_root: "/home/ubuntu/jarambit_runs/evo_g3".to_string(),
            systemd_unit: G3_KEEPALIVE_UNIT.to_string(),
            s3_prefix: "s3://jarambit-artifacts/evo-g3/".to_string(),
            allow_v1_seeds: false,
        }
    }
}

impl G3CohortConfig {
    /// Fail-closed validation of a G3 cohort launch configuration against `plan.md` §7.
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != G3_SCHEMA {
            return Err(format!(
                "invalid G3 schema '{}': must be '{}'",
                self.schema, G3_SCHEMA
            ));
        }
        if self.metric != G3_SCORE_METRIC {
            return Err(format!(
                "invalid G3 score metric '{}': must be '{}' (W_ULTRALIGHT_v1 is forbidden in G3)",
                self.metric, G3_SCORE_METRIC
            ));
        }
        if self.requirements_id != G3_REQUIREMENTS_ID {
            return Err(format!(
                "invalid G3 requirements '{}': must be '{}'",
                self.requirements_id, G3_REQUIREMENTS_ID
            ));
        }
        if !self.design_id_prefix.starts_with(G3_DESIGN_ID_PREFIX)
            || self.design_id_prefix.starts_with(V1_DESIGN_ID_PREFIX)
        {
            return Err(format!(
                "invalid G3 design_id_prefix '{}': must start with '{}' and never '{}'",
                self.design_id_prefix, G3_DESIGN_ID_PREFIX, V1_DESIGN_ID_PREFIX
            ));
        }
        let p = std::path::Path::new(&self.run_root);
        let is_abs_home = self.run_root.starts_with("/home/") || self.run_root.starts_with("/Users/");
        let has_tmp_or_v1 = p.components().any(|c| {
            let s = c.as_os_str().to_string_lossy();
            s == "tmp" || s == ".." || s.contains(V1_RUN_ROOT_DIR)
        });
        let has_evo_g3_dir = p.components().any(|c| {
            let s = c.as_os_str().to_string_lossy();
            s == G3_RUN_ROOT_DIR || s.starts_with("evo_g3_")
        });
        if !is_abs_home || has_tmp_or_v1 || !has_evo_g3_dir {
            return Err(format!(
                "invalid G3 run_root '{}': must be an absolute home path with an '{}' directory component and never touch '/tmp' or '{}'",
                self.run_root, G3_RUN_ROOT_DIR, V1_RUN_ROOT_DIR
            ));
        }
        if self.systemd_unit == V1_KEEPALIVE_UNIT || self.systemd_unit != G3_KEEPALIVE_UNIT {
            return Err(format!(
                "invalid G3 systemd_unit '{}': must be '{}'",
                self.systemd_unit, G3_KEEPALIVE_UNIT
            ));
        }
        if !self.s3_prefix.contains(G3_S3_PREFIX_SEGMENT) {
            return Err(format!(
                "invalid G3 s3_prefix '{}': must contain '{}'",
                self.s3_prefix, G3_S3_PREFIX_SEGMENT
            ));
        }
        if self.allow_v1_seeds {
            return Err("G3 cohort forbids v1 seeds (allow_v1_seeds must be false)".to_string());
        }
        Ok(())
    }
}

/// Format a canonical G3 design ID (`hz<host>i<island>_g<gen>_<short_sha>`) and verify it cannot
/// collide with v1 `ec<host>i<NNN>_` IDs.
pub fn format_g3_design_id(host_num: u32, island: u32, gen_num: u64, genome_sha256: &str) -> String {
    let short = &genome_sha256[..genome_sha256.len().min(8)];
    format!("hz{host_num}i{island:03}_g{gen_num:05}_{short}")
}

/// Validate that a design ID belongs to the G3 namespace (`hz<host>i<NNN>_...`) and is not a v1 ID.
pub fn validate_g3_design_id(design_id: &str) -> Result<(), String> {
    if design_id.starts_with(V1_DESIGN_ID_PREFIX) {
        return Err(format!(
            "refusing v1 design ID '{design_id}' in G3 pipeline (starts with 'ec')"
        ));
    }
    if !design_id.starts_with(G3_DESIGN_ID_PREFIX) {
        return Err(format!(
            "invalid G3 design ID '{design_id}': must start with 'hz'"
        ));
    }
    Ok(())
}

/// Census filter (`plan.md` §6 T6): scans a multi-line CSV stream, skips header lines, and parses
/// only verified `schema == "G3.0"` rows. Returns `Err` if any v1 row or tampered row is encountered
/// when `strict_reject_v1` is true, or filters to valid `G3.0` rows while counting rejected lines.
pub fn filter_g3_census_csv(
    csv_text: &str,
    strict_reject_non_g3: bool,
) -> Result<(Vec<G3Genome>, usize), String> {
    let mut genomes = Vec::new();
    let mut rejected = 0usize;
    for (line_no, raw_line) in csv_text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with("schema,") {
            continue;
        }
        if !line.starts_with("G3.0,") {
            rejected += 1;
            if strict_reject_non_g3 {
                return Err(format!(
                    "census filter rejected non-G3.0 row at line {}: '{}'",
                    line_no + 1,
                    &line[..line.len().min(48)]
                ));
            }
            continue;
        }
        let g = G3Genome::from_csv_row(line).map_err(|e| {
            format!(
                "census filter rejected invalid G3.0 row at line {}: {e}",
                line_no + 1
            )
        })?;
        genomes.push(g);
    }
    Ok((genomes, rejected))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::g3_genome::sobol_unit_genome;

    #[test]
    fn t6_namespace_isolation_rejects_v1_paths_units_metrics_and_rows() {
        let cfg = G3CohortConfig::default();
        assert!(cfg.validate().is_ok());

        let mut bad_root = cfg.clone();
        bad_root.run_root = "/home/ubuntu/jarambit_runs/evo_continuous/island01".to_string();
        assert!(bad_root.validate().is_err());

        let mut tmp_root = cfg.clone();
        tmp_root.run_root = "/tmp/x_evo_g3".to_string();
        assert!(tmp_root.validate().is_err());

        let mut bad_unit = cfg.clone();
        bad_unit.systemd_unit = "jarambit-evo-keepalive".to_string();
        assert!(bad_unit.validate().is_err());

        let mut bad_metric = cfg.clone();
        bad_metric.metric = "W_ULTRALIGHT_v1".to_string();
        assert!(bad_metric.validate().is_err());

        let mut bad_seeds = cfg.clone();
        bad_seeds.allow_v1_seeds = true;
        assert!(bad_seeds.validate().is_err());

        let g = sobol_unit_genome(0, 0, "22bb8696");
        let did = format_g3_design_id(4, 12, 105, &g.sha256());
        assert!(did.starts_with("hz4i012_g00105_"));
        assert!(validate_g3_design_id(&did).is_ok());
        assert!(validate_g3_design_id("ec3i042_r01_p10_c56").is_err());

        // Census filter accepts G3.0 and rejects/filters v1 128-gene rows
        let v1_row = vec!["0.5"; 128].join(",");
        let mixed_csv = format!(
            "{}\n{}\n{}\n",
            G3Genome::csv_header(),
            g.to_csv_row(),
            v1_row
        );
        assert!(filter_g3_census_csv(&mixed_csv, true).is_err());
        let (kept, rej) = filter_g3_census_csv(&mixed_csv, false).expect("non-strict filter");
        assert_eq!(kept.len(), 1);
        assert_eq!(rej, 1);
        assert_eq!(kept[0], g);
    }
}
