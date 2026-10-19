//! Reporting-only audit. Schema/provenance admission remains the caller's job.
use jarambit_paper_artifacts::native_history::{NativeHistory, SCHEMA};
use serde_json::json;
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: native_history_audit <captured.csv> <expected-islands|unknown>".into());
    }
    let expected = if args[1] == "unknown" {
        None
    } else {
        Some(args[1].parse::<u64>()?)
    };
    let history = NativeHistory::parse(&fs::read_to_string(&args[0])?, SCHEMA)?;
    let coverage = history.coverage(expected)?;
    let generations: Vec<_> = coverage.iter().map(|c| c.generation).collect();
    let partial: Vec<_> = coverage
        .iter()
        .filter(|c| c.missing_island_rows.is_some_and(|n| n > 0))
        .map(|c| json!({"generation":c.generation,"observed_island_rows":c.observed_island_rows,
            "expected_island_rows":c.expected_island_rows,"missing_island_rows":c.missing_island_rows}))
        .collect();
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema":"jarambit.native_history_structural_audit.v1",
            "path":args[0], "source_sha256":history.source_sha256,
            "declared_columns":SCHEMA,"record_kind":"generation_island_summary",
            "rows":history.rows.len(),"generation_count":generations.len(),
            "generation_bounds":[generations.first(),generations.last()],
            "expected_islands":expected,"partial_generations":partial,
            "generation_gaps":history.generation_gaps(),
            "unknown_scalar_cells":history.rows.iter().flat_map(|r|r.measurements.values()).filter(|v|v.is_none()).count(),
            "execution_complete":null,"unique_births":null,"qualification":null,
            "scope":"Structural checks only; no producer admission, candidate-ID inference, rescoring or run-completion verdict"
        }))?
    );
    Ok(())
}
