//! Byte-level and contract verification of Phase 1/2 population, identity, selection,
//! score-profile, scatter-cache, and compressed `.csv.zst` archive contracts (`SC-DEC-183N`
//! `JR-R1`/`JR-R2`, `SC-DEC-183AJ`).
//!
//! Modes:
//! - Default (and `--full-zstd` / `--check`): **Strictly read-only** audit and freshness check.
//!   Never mutates any tracked or generated file, and never invokes CAD, FEM, or search.
//! - `--populate-scatter` (mutually exclusive with read-only audit flags): Explicit offline
//!   cache-generation subcommand that decodes all 200 `.csv.zst` segments (`1,954,560` rows)
//!   and writes `docs/paper/generated/population/gen800_scatter_cache.v1.json`.
use jarambit_paper_artifacts::{corpus, identity, population, publication, validation};
use serde_json::{json, Value};
use std::{env, fs, path::Path};

fn read_json(path: &Path) -> Result<Value, Box<dyn std::error::Error>> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!(
            "Usage: publication_v2_audit [--full-zstd] [--check] | --populate-scatter\n\
             \n\
             Read-only audit mode (default, --full-zstd, --check):\n\
             \x20 Verifies all Phase 1/2 population ledgers, JEV1 frames, 57-file newer join,\n\
             \x20 gen800 .csv.zst segments, gen800_scatter_cache.v1.json schema/index/raster\n\
             \x20 bindings, and SVG/manifest freshness without writing any files.\n\
             \n\
             Explicit generation mode (--populate-scatter, mutually exclusive with audit flags):\n\
             \x20 Decodes all 200 admitted .csv.zst segments (1,954,560 rows) and writes\n\
             \x20 docs/paper/generated/population/gen800_scatter_cache.v1.json."
        );
        return Ok(());
    }
    let full_zstd = args.iter().any(|a| a == "--full-zstd");
    let check_flag = args.iter().any(|a| a == "--check");
    let populate_scatter = args.iter().any(|a| a == "--populate-scatter");
    if populate_scatter && (full_zstd || check_flag || args.len() > 1) {
        return Err(
            "--populate-scatter is an explicit generation mode and cannot be combined with read-only audit flags"
                .into(),
        );
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    if populate_scatter {
        let spec = corpus::populate_gen800_scatter_grid(&root)?;
        println!("{}", serde_json::to_string_pretty(&spec)?);
        return Ok(());
    }

    let pub_v2 = read_json(&corpus::resolve_safe_relpath(
        &root,
        "docs/paper/publication.v2.json",
    )?)?;
    let archive_root_rel = pub_v2["archive_contracts"]["gen800_population_archive_root_relpath"]
        .as_str()
        .ok_or("missing archive_contracts.gen800_population_archive_root_relpath in publication.v2.json")?;
    let expected_idx_sha = pub_v2["archive_contracts"]["gen800_population_index_sha256"]
        .as_str()
        .ok_or("missing archive_contracts.gen800_population_index_sha256 in publication.v2.json")?;
    let archive_root = corpus::resolve_archive_path(&root, archive_root_rel)?;

    let public_ids = read_json(&corpus::resolve_safe_relpath(
        &root,
        "docs/paper/data/public_ids.v1.json",
    )?)?;
    let selection = read_json(&corpus::resolve_safe_relpath(
        &root,
        "docs/paper/data/selection_receipts.v1.json",
    )?)?;
    let score_profiles = read_json(&corpus::resolve_safe_relpath(
        &root,
        "docs/paper/data/score_profiles.v1.json",
    )?)?;
    let t04 = read_json(&corpus::resolve_safe_relpath(
        &root,
        "docs/paper/JR_EI_P1_T04_HISTORICAL_RECONCILIATION_RECEIPT_20261019.json",
    )?)?;
    let t05 = read_json(&corpus::resolve_safe_relpath(
        &root,
        "docs/paper/JR_EI_P1_T05_NEWER_BIRTHS_SCREENS_JOIN_RECEIPT_20261019.json",
    )?)?;
    let t06 = read_json(&corpus::resolve_safe_relpath(
        &root,
        "docs/paper/JR_EI_P1_T06_NATIVE_SUMMARY_ADMISSION_RECEIPT_20261019.json",
    )?)?;
    let t07 = read_json(&corpus::resolve_safe_relpath(
        &root,
        "docs/paper/JR_EI_P1_T07_SEALED_SNAPSHOT_INVENTORY_AND_ACCESSION_RECEIPT_20261019.json",
    )?)?;
    let inheritance = read_json(&corpus::resolve_safe_relpath(
        &root,
        "docs/paper/generated/population/inheritance.json",
    )?)?;
    let repair_receipt = read_json(&corpus::resolve_safe_relpath(
        &root,
        "docs/paper/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_20261019.json",
    )?)?;
    let gen800_idx = read_json(&corpus::resolve_safe_relpath(
        &archive_root,
        "index.gen800_population.sha256.json",
    )?)?;
    let scatter_cache = read_json(&corpus::resolve_safe_relpath(
        &root,
        "docs/paper/generated/population/gen800_scatter_cache.v1.json",
    )?)?;

    // 1. Metadata contract validators
    let id_summary = identity::validate_public_ids(&public_ids)?;
    let sel_summary = identity::validate_selection_receipts(&selection, &public_ids)?;
    let sp_summary = validation::validate_score_profiles(&score_profiles)?;
    let pop_summary = population::validate_historical_population_adapter(&t04)?;
    let newer_summary = corpus::validate_newer_corpus_join(&t05)?;
    let latest_summary = corpus::validate_latest_snapshot_corpus(&t06, &t07)?;
    let gen800_summary = corpus::validate_gen800_population_accession(&gen800_idx)?;
    let scatter_summary = corpus::validate_gen800_scatter_cache(&scatter_cache, expected_idx_sha)?;
    let inh_summary = population::validate_inheritance_ledger(&inheritance)?;
    let repair_summary = corpus::validate_repair_receipt_file_bindings(&root, &repair_receipt)?;

    // 2. Real byte-reading decoders (JEV1 binary stream + 57-file newer join + ruzstd .csv.zst decode)
    let hist_bytes_ok = corpus::read_and_reconcile_historical_w282(&root)?;
    let newer_bytes_ok = corpus::read_and_join_newer_corpus(&root)?;
    let max_segs = if full_zstd { None } else { Some(1) };
    let zstd_bytes_ok =
        corpus::verify_and_decode_gen800_archive(&archive_root, Some(expected_idx_sha), max_segs)?;

    // 3. Verify all generated SVGs, projections, and manifests are fresh in check mode
    publication::generate(&root, true)?;

    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema": "jarambit.publication_v2_audit.v1",
            "mode": "read_only_audit",
            "public_ids": id_summary["explicit_individual_count"],
            "selection_claims": sel_summary["claim_count"],
            "score_profiles": sp_summary["profile_count"],
            "historical_births": pop_summary["births"],
            "historical_jevl_frames_decoded": hist_bytes_ok["jevl_frames"],
            "historical_jevl_events_decoded": hist_bytes_ok["jevl_events"],
            "historical_distinct_gene_tuples": hist_bytes_ok["distinct_serialized_gene_vectors"],
            "newer_births": newer_summary["birth_events"],
            "newer_exact_1based_joins_verified": newer_bytes_ok["joined_screen_rows"],
            "newer_unscreened_births": newer_bytes_ok["unscreened_birth_events"],
            "newer_zero_based_misjoins_rejected": newer_bytes_ok["zero_based_misjoins_rejected"],
            "latest_v2_rows": latest_summary["v2_staged_rows"],
            "gen800_lossless_rows": gen800_summary["total_rows"],
            "gen800_scatter_cache_conserved_points": scatter_summary["conserved_numeric_points"],
            "gen800_zstd_segments_decoded": zstd_bytes_ok["segments_decoded"],
            "gen800_zstd_rows_decoded": zstd_bytes_ok["rows_decoded"],
            "inheritance_edges": inh_summary["directed_edge_count"],
            "repair_receipt_verified_files": repair_summary["verified_on_disk_files"],
        }))?
    );
    Ok(())
}
