//! Read-only reporting for the retained native CSV dialect. This does not
//! establish producer identity, run completion, qualification, or birth counts.
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// The caller must bind this declaration to the actual producer source/config.
/// Fourteen columns alone do not establish their meaning or score comparability.
pub const SCHEMA: &str = "gen,island,evals,aesthetic_pass,legal,feasible,best_fit,best_spg,best_mass_g,best_aes,best_nov,archive_cells,min_legal_mass_g,mean_relief_frac_pass";

#[derive(Debug, PartialEq)]
pub struct Row {
    pub generation: u64,
    pub island: u64,
    /// Original field names, without interpreting population sizes as births.
    pub counts: BTreeMap<&'static str, u64>,
    /// Explicit source NaN becomes None, never a zero or favorable observation.
    pub measurements: BTreeMap<&'static str, Option<f64>>,
}

#[derive(Debug, PartialEq)]
pub struct GenerationCoverage {
    pub generation: u64,
    pub observed_island_rows: u64,
    pub expected_island_rows: Option<u64>,
    pub missing_island_rows: Option<u64>,
}

#[derive(Debug)]
pub struct NativeHistory {
    pub source_sha256: String,
    pub rows: Vec<Row>,
}

impl NativeHistory {
    /// Parse an entire captured input; never repair or truncate source bytes.
    /// Unknown schema declarations and incomplete last lines are not publishable.
    /// Raw interrupted inputs may still be archived separately with that status.
    pub fn parse(text: &str, declared_schema: &str) -> Result<Self> {
        if declared_schema != SCHEMA {
            return Err("native history: unsupported column declaration".into());
        }
        if text.is_empty() || !text.ends_with('\n') {
            return Err("native history: empty input or incomplete final line".into());
        }
        let fields: Vec<_> = SCHEMA.split(',').collect();
        let mut rows = Vec::new();
        let mut seen = BTreeSet::new();
        for (index, line) in text.lines().enumerate() {
            let values: Vec<_> = line.split(',').collect();
            if values.len() != fields.len() {
                return Err(
                    format!("native history: row {} must have 14 fields", index + 1).into(),
                );
            }
            let integer = |column: usize| -> Result<u64> {
                Ok(values[column].parse::<u64>().map_err(|_| {
                    format!(
                        "native history: row {} invalid {}",
                        index + 1,
                        fields[column]
                    )
                })?)
            };
            let generation = integer(0)?;
            let island = integer(1)?;
            if !seen.insert((generation, island)) {
                return Err("native history: duplicate generation/island pair".into());
            }
            let mut counts = BTreeMap::new();
            for column in [2, 3, 4, 5, 11] {
                counts.insert(fields[column], integer(column)?);
            }
            let mut measurements = BTreeMap::new();
            for column in [6, 7, 8, 9, 10, 12, 13] {
                let value = if values[column] == "NaN" {
                    None
                } else {
                    let value = values[column].parse::<f64>().map_err(|_| {
                        format!(
                            "native history: row {} invalid {}",
                            index + 1,
                            fields[column]
                        )
                    })?;
                    if !value.is_finite() {
                        return Err("native history: unsupported nonfinite scalar".into());
                    }
                    Some(value)
                };
                measurements.insert(fields[column], value);
            }
            rows.push(Row {
                generation,
                island,
                counts,
                measurements,
            });
        }
        Ok(Self {
            source_sha256: format!("{:x}", Sha256::digest(text.as_bytes())),
            rows,
        })
    }

    /// Expected islands comes only from admitted configuration. None leaves
    /// completeness unknown; observing IDs 0..3 does not prove four were planned.
    pub fn coverage(&self, expected_islands: Option<u64>) -> Result<Vec<GenerationCoverage>> {
        if expected_islands == Some(0) {
            return Err("native history: expected island count must be positive".into());
        }
        let mut counts = BTreeMap::<u64, u64>::new();
        for row in &self.rows {
            if expected_islands.is_some_and(|n| row.island >= n) {
                return Err("native history: island outside declared configuration".into());
            }
            *counts.entry(row.generation).or_default() += 1;
        }
        Ok(counts
            .into_iter()
            .map(|(generation, observed)| GenerationCoverage {
                generation,
                observed_island_rows: observed,
                expected_island_rows: expected_islands,
                missing_island_rows: expected_islands.map(|n| n - observed),
            })
            .collect())
    }

    /// Compact inclusive gaps between observed generations, without allocating
    /// a synthetic record for each missing generation or padding a curve.
    pub fn generation_gaps(&self) -> Vec<(u64, u64)> {
        let generations: Vec<_> = self
            .rows
            .iter()
            .map(|r| r.generation)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        generations
            .windows(2)
            .filter(|w| w[1] - w[0] > 1)
            .map(|w| (w[0] + 1, w[1] - 1))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Synthetic reporting-only rows, never publication observations.
    fn row(generation: u64, island: u64) -> String {
        format!("{generation},{island},8,7,1,0,-0.25,2.0,9.0,0.4,0.03,1,NaN,0.06\n")
    }

    #[test]
    fn headerless_first_generation_and_island_zero_are_retained() {
        let text = row(0, 0) + &row(0, 1);
        let history = NativeHistory::parse(&text, SCHEMA).unwrap();
        assert_eq!(history.rows.len(), 2);
        assert_eq!((history.rows[0].generation, history.rows[0].island), (0, 0));
        assert_eq!(history.rows[0].counts["evals"], 8);
        assert_eq!(
            history.coverage(Some(2)).unwrap()[0].missing_island_rows,
            Some(0)
        );
    }

    #[test]
    fn source_nan_stays_missing_and_negative_contextual_scores_are_preserved() {
        let history = NativeHistory::parse(&row(0, 0), SCHEMA).unwrap();
        assert_eq!(history.rows[0].measurements["min_legal_mass_g"], None);
        assert_eq!(history.rows[0].measurements["best_fit"], Some(-0.25));
    }

    #[test]
    fn gaps_and_partial_generations_are_reported_without_padding() {
        let text = row(0, 0) + &row(0, 1) + &row(2, 0);
        let history = NativeHistory::parse(&text, SCHEMA).unwrap();
        assert_eq!(history.rows.len(), 3);
        assert_eq!(history.generation_gaps(), vec![(1, 1)]);
        assert_eq!(
            history.coverage(Some(2)).unwrap()[1].missing_island_rows,
            Some(1)
        );
        assert_eq!(history.coverage(None).unwrap()[1].missing_island_rows, None);
        assert_eq!(
            history.coverage(None).unwrap()[1].expected_island_rows,
            None
        );
    }

    #[test]
    fn missing_or_incorrect_configuration_cannot_imply_complete_coverage() {
        let history = NativeHistory::parse(&row(0, 3), SCHEMA).unwrap();
        assert!(history.coverage(Some(0)).is_err());
        assert!(history.coverage(Some(3)).is_err());
        assert_eq!(history.coverage(None).unwrap()[0].missing_island_rows, None);
    }

    #[test]
    fn repeated_generation_island_pairs_are_rejected() {
        assert!(NativeHistory::parse(&(row(0, 0) + &row(0, 0)), SCHEMA).is_err());
    }

    #[test]
    fn headers_partial_lines_and_unknown_dialects_are_rejected() {
        for text in [
            String::new(),
            row(0, 0).trim_end().to_string(),
            format!("{SCHEMA}\n{}", row(0, 0)),
            row(0, 0) + "\n",
        ] {
            assert!(NativeHistory::parse(&text, SCHEMA).is_err());
        }
        assert!(NativeHistory::parse(&row(0, 0), &SCHEMA.replace("best_fit", "score")).is_err());
        assert!(NativeHistory::parse(&row(0, 0).replacen(",8,", ",8.0,", 1), SCHEMA).is_err());
    }

    #[test]
    fn infinities_unknown_tokens_and_missing_counts_are_not_zero_filled() {
        for changed in [
            row(0, 0).replace("-0.25", "inf"),
            row(0, 0).replace("-0.25", "-inf"),
            row(0, 0).replace("-0.25", "unknown"),
            row(0, 0).replacen(",8,", ",NaN,", 1),
            row(0, 0).replacen(",8,", ",18446744073709551616,", 1),
        ] {
            assert!(NativeHistory::parse(&changed, SCHEMA).is_err());
        }
    }

    #[test]
    fn large_generation_gaps_remain_compact_and_original_bytes_are_bound() {
        let text = row(0, 0) + &row(u64::MAX, 0);
        let history = NativeHistory::parse(&text, SCHEMA).unwrap();
        assert_eq!(history.generation_gaps(), vec![(1, u64::MAX - 1)]);
        assert_eq!(history.rows.len(), 2);
        assert_eq!(
            history.source_sha256,
            format!("{:x}", Sha256::digest(text.as_bytes()))
        );
        let changed = NativeHistory::parse(&(row(u64::MAX, 0) + &row(0, 0)), SCHEMA).unwrap();
        assert_ne!(history.source_sha256, changed.source_sha256);
        assert_eq!(history.generation_gaps(), changed.generation_gaps());
    }
}
