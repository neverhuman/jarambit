//! Versioned historical population adapter, membership occurrence tables,
//! recorded parent/genome-difference projections, and deterministic Rust
//! emitters for the six multi-cohort SVGs and `publication.v2.json`
//! (`JR-EI-P2-T03`..`T08` and `SC-DEC-183N` `JR-R2` / `JR-R3`).

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn sha256_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn svg_coord(v: f64) -> String {
    let rounded = (v * 1000.0).round() / 1000.0;
    let clean = if rounded.abs() < 0.0005 { 0.0 } else { rounded };
    format!("{clean:.3}")
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Explicit versioned adapter for historical JEV1 event status strings.
pub fn adapt_historical_jevl_fem_status(raw_status: &str) -> Result<&'static str> {
    match raw_status {
        "complete" => Ok("complete"),
        "notevaluated" => Ok("not_evaluated"),
        _ => Err(format!("population: unrecognized historical JEV1 status '{raw_status}'").into()),
    }
}

/// Validate the reconciled historical `w282_ultrarunner` population tables
/// (`JR-EI-P1-T04` / `JR-EI-P2-T03`) without inferring missing elite scores or
/// post-migration slot replacements.
pub fn validate_historical_population_adapter(t04_receipt: &Value) -> Result<Value> {
    if t04_receipt["task_id"].as_str() != Some("JR-EI-P1-T04") {
        return Err("population: expected JR-EI-P1-T04 receipt".into());
    }
    let rc = &t04_receipt["reconciled_counts"];
    if rc["intact_zstd_frames"].as_u64() != Some(19)
        || rc["torn_tail"].as_bool() != Some(false)
        || rc["exact_eof_bytes"].as_u64() != Some(64162)
        || rc["total_events"].as_u64() != Some(2207)
    {
        return Err("population: historical JEV1 frame/event count or EOF check failed".into());
    }
    let kinds = &rc["event_breakdown"];
    if kinds["run_start"].as_u64() != Some(1)
        || kinds["eval_births"].as_u64() != Some(1984)
        || kinds["elite_reappearances"].as_u64() != Some(192)
        || kinds["migration_events"].as_u64() != Some(12)
        || kinds["gen_end_summaries"].as_u64() != Some(17)
        || kinds["run_end"].as_u64() != Some(1)
    {
        return Err("population: historical JEV1 event kind breakdown mismatch".into());
    }

    let rules = &t04_receipt["semantic_preservation_rules"];
    let dup_mult = &rules["duplicate_elite_slot_multiplicity"];
    if rc["membership_occurrences_total"].as_u64() != Some(2176)
        || dup_mult["duplicate_generation_island_id_keys_count"].as_u64() != Some(9)
    {
        return Err(
            "population: expected 2176 membership occurrences and 9 duplicate slot keys".into(),
        );
    }
    let dup_keys = dup_mult["duplicate_keys"]
        .as_array()
        .ok_or("population: missing duplicate_keys")?;
    if dup_keys.len() != 9 {
        return Err("population: duplicate_keys array length must be 9".into());
    }
    for dup in dup_keys {
        if dup["occurrences"].as_u64().unwrap_or(0) < 2 {
            return Err("population: duplicate key must have at least 2 slot occurrences".into());
        }
    }

    let mig_events = rules["migration_copy_semantics"]["events"]
        .as_array()
        .ok_or("population: missing migration events")?;
    if rc["migration_events_count"].as_u64() != Some(12)
        || rc["migration_copied_id_occurrences"].as_u64() != Some(24)
        || mig_events.len() != 12
    {
        return Err("population: migration audit must retain 12 events / 24 copies".into());
    }

    let anc = &rc["parent_arity_counts"];
    if anc["roots_0_parents"].as_u64() != Some(128)
        || anc["one_parent_mutations"].as_u64() != Some(938)
        || anc["two_parent_crossovers"].as_u64() != Some(918)
        || anc["all_parents_from_earlier_generation"].as_bool() != Some(true)
    {
        return Err("population: historical ancestry counts 128/938/918 mismatch".into());
    }

    let csv_rec = &rc["genome_csv_schema"];
    if rc["births_reconciled_across_jevl_json_csv"].as_u64() != Some(1984)
        || csv_rec["total_columns"].as_u64() != Some(146)
        || csv_rec["gene_columns_count_17_to_145"].as_u64() != Some(129)
        || csv_rec["distinct_serialized_gene_vectors"].as_u64() != Some(1965)
    {
        return Err(
            "population: historical CSV 1984x146 / 1965 distinct serializations mismatch".into(),
        );
    }

    Ok(json!({
        "schema": "jarambit.historical_population_adapter.v1",
        "births": 1984,
        "generations": 17,
        "membership_occurrences": 2176,
        "elite_reappearances": 192,
        "duplicate_slot_keys": 9,
        "migration_events": 12,
        "migration_copied_occurrences": 24,
        "migration_generations": [4, 8, 12],
        "elite_current_score_policy": "null_with_explicit_reason",
        "migration_replacement_policy": "null_with_explicit_reason",
        "distinct_csv_gene_serializations": 1965,
    }))
}

pub const HISTORICAL_ALL_EVALS_JSON_RELPATH: &str =
    "evidence/rs_v2/w282_ultrarunner/search/all_evals.json";
pub const HISTORICAL_ALL_EVALS_JSON_SHA256: &str =
    "307370108fcede8a4399569582f6ab19d436603246531266afa1754f9ba65363";
pub const HISTORICAL_ALL_EVALS_CSV_RELPATH: &str =
    "evidence/rs_v2/w282_ultrarunner/search/all_evals.csv";
pub const HISTORICAL_ALL_EVALS_CSV_SHA256: &str =
    "a28f6d2c8a676661280981733a1e33c80136aba157c7a9d2f5a5c3db444a223c";

fn round6(v: f64) -> f64 {
    (v * 1_000_000.0).round() / 1_000_000.0
}

/// Recompute the 3-hop ancestry and 8-edge induced subgraph genome-difference ledger
/// directly from the original `evidence/rs_v2/w282_ultrarunner/search/{all_evals.json,all_evals.csv}`
/// (`SC-DEC-183AA` / `SC-DEC-183AB`).
pub fn recompute_w282_inheritance_ledger(root: &Path) -> Result<Value> {
    let json_bytes = fs::read(crate::corpus::resolve_safe_relpath(
        root,
        HISTORICAL_ALL_EVALS_JSON_RELPATH,
    )?)?;
    let json_sha = sha256_bytes(&json_bytes);
    if json_sha != HISTORICAL_ALL_EVALS_JSON_SHA256 {
        return Err(format!(
            "population: {} sha256 mismatch: expected {}, got {}",
            HISTORICAL_ALL_EVALS_JSON_RELPATH, HISTORICAL_ALL_EVALS_JSON_SHA256, json_sha
        )
        .into());
    }
    let births: Vec<Value> = serde_json::from_slice(&json_bytes)?;
    if births.len() != 1984 {
        return Err("population: expected 1984 births in w282 all_evals.json".into());
    }
    let mut births_by_id: BTreeMap<u64, &Value> = BTreeMap::new();
    for b in &births {
        let id = b["id"].as_u64().ok_or("population: missing birth id")?;
        if births_by_id.insert(id, b).is_some() {
            return Err(format!("population: duplicate birth id {id}").into());
        }
    }

    let csv_bytes = fs::read(crate::corpus::resolve_safe_relpath(
        root,
        HISTORICAL_ALL_EVALS_CSV_RELPATH,
    )?)?;
    let csv_sha = sha256_bytes(&csv_bytes);
    if csv_sha != HISTORICAL_ALL_EVALS_CSV_SHA256 {
        return Err(format!(
            "population: {} sha256 mismatch: expected {}, got {}",
            HISTORICAL_ALL_EVALS_CSV_RELPATH, HISTORICAL_ALL_EVALS_CSV_SHA256, csv_sha
        )
        .into());
    }
    let csv_str = std::str::from_utf8(&csv_bytes)?;
    let mut lines = csv_str.lines();
    let header_line = lines.next().ok_or("population: empty all_evals.csv")?;
    let header_cols: Vec<&str> = header_line.split(',').collect();
    if header_cols.len() != 146 || header_cols[17] != "cp00" || header_cols[145] != "material" {
        return Err("population: unexpected 146-column header in all_evals.csv".into());
    }
    let gene_names: Vec<String> = header_cols[17..=144]
        .iter()
        .map(|s| (*s).to_string())
        .collect();

    struct CsvRow {
        genes: Vec<f64>,
        material: String,
        gene_ser_sha256: String,
    }
    let mut csv_by_id: BTreeMap<u64, CsvRow> = BTreeMap::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() != 146 {
            return Err("population: malformed CSV row width".into());
        }
        let id: u64 = cols[0].parse()?;
        let mut genes = Vec::with_capacity(128);
        for c in &cols[17..=144] {
            let v: f64 = c.parse()?;
            if !v.is_finite() {
                return Err("population: non-finite gene value in all_evals.csv".into());
            }
            genes.push(v);
        }
        let material = cols[145].to_string();
        let gene_ser_sha256 = sha256_bytes(cols[17..=145].join(",").as_bytes());
        csv_by_id.insert(
            id,
            CsvRow {
                genes,
                material,
                gene_ser_sha256,
            },
        );
    }

    // BFS up to 3 parent hops from focal individual 1095
    let focal_id = 1095u64;
    let max_hops = 3u64;
    let mut min_hops: BTreeMap<u64, u64> = BTreeMap::new();
    let mut queue = std::collections::VecDeque::from([(focal_id, 0u64)]);
    while let Some((uid, depth)) = queue.pop_front() {
        if min_hops.contains_key(&uid) {
            continue;
        }
        min_hops.insert(uid, depth);
        if depth < max_hops {
            let b = births_by_id
                .get(&uid)
                .ok_or_else(|| format!("population: missing birth {uid}"))?;
            let parents = b["parents"]
                .as_array()
                .ok_or("population: missing parents array")?;
            for p_val in parents {
                let pid = p_val.as_u64().ok_or("population: non-u64 parent")?;
                if !min_hops.contains_key(&pid) {
                    queue.push_back((pid, depth + 1));
                }
            }
        }
    }

    let mut nodes_out = Vec::with_capacity(min_hops.len());
    for (&uid, &hops) in &min_hops {
        let b = births_by_id[&uid];
        let c = csv_by_id
            .get(&uid)
            .ok_or_else(|| format!("population: missing CSV row for {uid}"))?;
        let parents: Vec<u64> = b["parents"]
            .as_array()
            .ok_or("population: missing parents")?
            .iter()
            .map(|v| v.as_u64().ok_or("population: invalid parent id"))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        nodes_out.push(json!({
            "public_id": uid,
            "public_label": format!("Individual {uid}"),
            "generation": b["gen"].as_u64().ok_or("population: missing gen")?,
            "stored_fit": b["fit"].as_f64().ok_or("population: missing fit")?,
            "operator": b["op"].as_str().ok_or("population: missing op")?,
            "operator_version": b["op_version"].as_u64().ok_or("population: missing op_version")?,
            "recorded_parents": parents,
            "minimum_parent_hops_from_focal": hops,
            "fem_status": b["fem_status"].as_str().ok_or("population: missing fem_status")?,
            "legal": b["legal"].as_bool().ok_or("population: missing legal")?,
            "material_category": c.material,
            "csv_gene_serialization_sha256": c.gene_ser_sha256,
        }));
    }

    let build_edge_diff = |pid: u64, cid: u64, ord: usize, edge_hops: u64| -> Result<Value> {
        let cb = births_by_id[&cid];
        let p_csv = &csv_by_id[&pid];
        let c_csv = &csv_by_id[&cid];
        let mut changed = 0u64;
        let mut sq_sum = 0.0_f64;
        let mut max_abs = 0.0_f64;
        let mut samples = Vec::new();
        for i in 0..128usize {
            let pv = p_csv.genes[i];
            let cv = c_csv.genes[i];
            let delta = cv - pv;
            if pv != cv {
                changed += 1;
                sq_sum += delta * delta;
                let ad = delta.abs();
                if ad > max_abs {
                    max_abs = ad;
                }
                if samples.len() < 12 {
                    samples.push(json!({
                        "gene_index": i,
                        "csv_column_index": 17 + i,
                        "gene_name": gene_names[i],
                        "parent_val": pv,
                        "child_val": cv,
                        "delta": round6(delta),
                        "delta_exact": delta,
                    }));
                }
            }
        }
        let l2_exact = sq_sum.sqrt();
        Ok(json!({
            "parent_id": pid,
            "child_id": cid,
            "parent_ordinal": ord,
            "child_operator": cb["op"].as_str().unwrap_or(""),
            "min_hops_to_focal": edge_hops,
            "within_3_hop_edge_horizon": edge_hops <= max_hops,
            "changed_f64_gene_count": changed,
            "identical_f64_gene_count": 128 - changed,
            "material_changed": p_csv.material != c_csv.material,
            "l2_gene_distance": round6(l2_exact),
            "l2_gene_distance_exact": l2_exact,
            "max_abs_coordinate_delta": round6(max_abs),
            "sample_changed_coordinates": samples,
        }))
    };

    // Order children by (generation, id) and iterate their recorded parents in original order
    let mut ordered_children: Vec<u64> = min_hops.keys().copied().collect();
    ordered_children.sort_by_key(|&id| (births_by_id[&id]["gen"].as_u64().unwrap_or(0), id));

    let mut hop_limited_edges = Vec::new();
    let mut fourth_hop_induced_edges = Vec::new();
    let mut induced_edges = Vec::new();

    for cid in ordered_children {
        let child_hops = min_hops[&cid];
        let parents = births_by_id[&cid]["parents"]
            .as_array()
            .ok_or("population: missing parents")?;
        for (ord, p_val) in parents.iter().enumerate() {
            let pid = p_val.as_u64().ok_or("population: invalid parent id")?;
            if min_hops.contains_key(&pid) {
                let edge_hops = child_hops + 1;
                let edge_val = build_edge_diff(pid, cid, ord, edge_hops)?;
                if edge_hops <= max_hops {
                    hop_limited_edges.push(edge_val.clone());
                } else {
                    fourth_hop_induced_edges.push(edge_val.clone());
                }
                induced_edges.push(edge_val);
            }
        }
    }

    Ok(json!({
        "schema": "jarambit.inheritance_genome_diff.v1",
        "source_json_relpath": HISTORICAL_ALL_EVALS_JSON_RELPATH,
        "source_json_sha256": HISTORICAL_ALL_EVALS_JSON_SHA256,
        "source_csv_relpath": HISTORICAL_ALL_EVALS_CSV_RELPATH,
        "source_csv_sha256": HISTORICAL_ALL_EVALS_CSV_SHA256,
        "public_run_id": 1,
        "focal_public_id": focal_id,
        "max_parent_hops_included": max_hops,
        "gene_coordinate_count": 128,
        "csv_genome_column_count": 129,
        "changed_coordinate_definition": "exact_numeric_delta_ne_0_across_csv_columns_17_to_144",
        "l2_distance_note": "Euclidean L2 norm across raw CSV numeric coordinates (columns 17..144); mixes coordinate units (mm, degrees, dimensionless, discrete counts) and is not a physical distance.",
        "reused_ancestor_finding": "Individual 413 (gen 3, fit 1.15296, parents [245], archive_mutate) is reused across two generations in Individual 1095's ancestry: first as parent 0 of Individual 526 (gen 4, fit 1.15563, parents [413, 392], crossover_mutate) and again as parent 0 of Individual 1095 (gen 9, fit 1.15727, parents [413, 526], crossover_mutate). In the induced 7-node subgraph, Individual 174 (gen 1, fit 1.14979, parents [57, 61]) also branches to both Individual 245 (3 hops from 1095 via 413) and Individual 299 (4 hops from 1095 via 526 <- 392 <- 299).",
        "focal_subgraph_nodes": nodes_out,
        "three_hop_edge_count": hop_limited_edges.len(),
        "induced_subgraph_edge_count": induced_edges.len(),
        "parent_child_gene_diffs": hop_limited_edges,
        "fourth_hop_induced_closure_edges": fourth_hop_induced_edges,
        "induced_subgraph_parent_child_gene_diffs": induced_edges,
        "scope": "Recorded parent links from all_evals.json and exact stored CSV gene coordinate differences (delta != 0) across columns 17..144 plus material column 145; coordinate difference does not establish causal attribution of individual genes."
    }))
}

/// Validate the inheritance and genome-difference ledger (`JR-EI-P2-T07` & `SC-DEC-183AA` / `SC-DEC-183AB`).
pub fn validate_inheritance_ledger(ledger: &Value) -> Result<Value> {
    if ledger["schema"].as_str() != Some("jarambit.inheritance_genome_diff.v1") {
        return Err("population: unsupported inheritance ledger schema".into());
    }
    if ledger["source_json_sha256"].as_str() != Some(HISTORICAL_ALL_EVALS_JSON_SHA256)
        || ledger["source_csv_sha256"].as_str() != Some(HISTORICAL_ALL_EVALS_CSV_SHA256)
    {
        return Err("population: inheritance ledger source SHA-256 mismatch".into());
    }
    if ledger["focal_public_id"].as_u64() != Some(1095)
        || ledger["gene_coordinate_count"].as_u64() != Some(128)
        || ledger["csv_genome_column_count"].as_u64() != Some(129)
    {
        return Err("population: inheritance focal ID or 128+1 gene schema mismatch".into());
    }
    let nodes = ledger["focal_subgraph_nodes"]
        .as_array()
        .ok_or("population: missing focal_subgraph_nodes")?;
    let mut node_parents: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
    let mut node_ids = BTreeSet::new();
    for n in nodes {
        let id = n["public_id"]
            .as_u64()
            .ok_or("population: missing node public_id")?;
        let parents: Vec<u64> = n["recorded_parents"]
            .as_array()
            .ok_or("population: missing recorded_parents")?
            .iter()
            .map(|v| v.as_u64().ok_or("population: invalid recorded_parent"))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        node_ids.insert(id);
        node_parents.insert(id, parents);
    }
    let expected: BTreeSet<u64> = [174, 245, 299, 392, 413, 526, 1095].into_iter().collect();
    if node_ids != expected {
        return Err(
            "population: focal 3-hop ancestry must be {174,245,299,392,413,526,1095}".into(),
        );
    }
    if node_parents.get(&413) != Some(&vec![245])
        || node_parents.get(&526) != Some(&vec![413, 392])
        || node_parents.get(&1095) != Some(&vec![413, 526])
    {
        return Err(
            "population: recorded_parents for 413/526/1095 disagree with all_evals.json".into(),
        );
    }

    let edges = ledger["parent_child_gene_diffs"]
        .as_array()
        .ok_or("population: missing parent_child_gene_diffs")?;
    if edges.len() != 7 {
        return Err("population: expected 7 directed parent-child edges in 3-hop traversal".into());
    }
    let mut pairs = BTreeMap::new();
    for edge in edges {
        let p = edge["parent_id"]
            .as_u64()
            .ok_or("population: missing parent_id")?;
        let c = edge["child_id"]
            .as_u64()
            .ok_or("population: missing child_id")?;
        let ord = edge["parent_ordinal"]
            .as_u64()
            .ok_or("population: missing parent_ordinal")? as usize;
        let rec_p = node_parents
            .get(&c)
            .ok_or("population: edge child missing from focal_subgraph_nodes")?;
        if rec_p.get(ord) != Some(&p) {
            return Err(format!(
                "population: edge ({p} -> {c}) at ordinal {ord} is not a recorded parent in all_evals.json"
            )
            .into());
        }
        let changed = edge["changed_f64_gene_count"]
            .as_u64()
            .ok_or("population: missing changed_f64_gene_count")?;
        let identical = edge["identical_f64_gene_count"]
            .as_u64()
            .ok_or("population: missing identical_f64_gene_count")?;
        if changed + identical != 128 {
            return Err("population: changed + identical f64 genes must equal 128".into());
        }
        pairs.insert((p, c), changed);
    }
    if pairs.contains_key(&(392, 413)) {
        return Err("population: false non-parent edge (392, 413) must not be present".into());
    }
    for (req, exp_chg) in [
        ((174, 245), 24u64),
        ((299, 392), 8),
        ((245, 413), 5),
        ((413, 526), 7),
        ((392, 526), 35),
        ((413, 1095), 13),
        ((526, 1095), 14),
    ] {
        if pairs.get(&req) != Some(&exp_chg) {
            return Err(format!(
                "population: missing or mismatched 3-hop directed edge {:?} (expected {} changed genes)",
                req, exp_chg
            )
            .into());
        }
    }

    let induced = ledger["induced_subgraph_parent_child_gene_diffs"]
        .as_array()
        .ok_or("population: missing induced_subgraph_parent_child_gene_diffs")?;
    if induced.len() != 8 {
        return Err(
            "population: expected 8 directed parent-child edges in 7-node induced subgraph".into(),
        );
    }
    let fourth_hop = ledger["fourth_hop_induced_closure_edges"]
        .as_array()
        .ok_or("population: missing fourth_hop_induced_closure_edges")?;
    if fourth_hop.len() != 1
        || fourth_hop[0]["parent_id"].as_u64() != Some(174)
        || fourth_hop[0]["child_id"].as_u64() != Some(299)
        || fourth_hop[0]["changed_f64_gene_count"].as_u64() != Some(2)
    {
        return Err(
            "population: expected fourth-hop induced closure edge 174->299 (2/128 changed)".into(),
        );
    }

    Ok(json!({
        "schema": "jarambit.inheritance_validation.v1",
        "focal_public_id": 1095,
        "node_count": nodes.len(),
        "directed_edge_count": edges.len(),
        "induced_subgraph_edge_count": induced.len(),
        "reused_ancestor_id": 413,
    }))
}

/// Generate `docs/paper/figures/population-births.svg` from raw Cohort 1 (`w282`) births.
fn render_population_births_svg(births: &[Value]) -> Result<String> {
    let mut s = String::with_capacity(180_000);
    s.push_str(r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1200 700" width="1200" height="700" font-family="Inter, -apple-system, BlinkMacSystemFont, sans-serif">"##);
    s.push('\n');
    s.push_str(r##"<rect width="1200" height="700" fill="#f8fafc"/>"##);
    s.push_str(r##"<rect x="20" y="16" width="1160" height="668" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-width="1.5"/>"##);
    s.push_str(r##"<text x="40" y="48" font-size="24" font-weight="700" fill="#0f172a">Cohort 1 (Run 1): All 1,984 Birth Evaluations Across Generations 0–16</text>"##);
    s.push_str(r##"<text x="40" y="78" font-size="21" fill="#475569">1,455 FEM-complete births (blue) vs. 529 unevaluated penalty records (amber); 446 search-legal</text>"##);

    let x0 = 175.0_f64;
    let x1 = 1140.0_f64;
    let y0 = 104.0_f64;
    let y1 = 460.0_f64;
    let ymin_asinh = (-220.0_f64).asinh();
    let ymax_asinh = (1.5_f64).asinh();
    let map_x = |g: u64, id: u64| -> f64 {
        let jitter = ((id % 29) as f64 - 14.0) * 0.45;
        x0 + (g as f64 / 16.0) * (x1 - x0) + jitter
    };
    let map_y = |fit: f64| -> f64 {
        let a = fit.clamp(-220.0, 1.5).asinh();
        y1 - ((a - ymin_asinh) / (ymax_asinh - ymin_asinh)) * (y1 - y0)
    };

    s.push_str(&format!(
        r##"<rect x="{}" y="{}" width="{}" height="{}" fill="#f8fafc" stroke="#cbd5e1" stroke-width="1"/>"##,
        svg_coord(x0),
        svg_coord(y0),
        svg_coord(x1 - x0),
        svg_coord(y1 - y0)
    ));

    for (tick_val, tick_lbl) in [
        (-200.0, "-200.0"),
        (-50.0, "-50.0"),
        (-10.0, "-10.0"),
        (-2.0, "-2.0"),
        (0.0, "0.0"),
        (1.157, "+1.157"),
    ] {
        let ty = map_y(tick_val);
        s.push_str(&format!(
            r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#e2e8f0" stroke-width="1" stroke-dasharray="3,3"/>"##,
            svg_coord(x0),
            svg_coord(ty),
            svg_coord(x1),
            svg_coord(ty)
        ));
        s.push_str(&format!(
            r##"<text x="{}" y="{}" text-anchor="end" font-size="21" fill="#475569">{}</text>"##,
            svg_coord(x0 - 10.0),
            svg_coord(ty + 7.0),
            tick_lbl
        ));
    }

    let mut per_gen_fits: Vec<Vec<f64>> = vec![Vec::new(); 17];
    let mut per_gen_fem = vec![0u64; 17];
    let mut per_gen_not = vec![0u64; 17];

    for b in births {
        let id = b["id"].as_u64().ok_or("missing birth id")?;
        let g = b["gen"].as_u64().ok_or("missing birth gen")? as usize;
        let fit = b["fit"].as_f64().ok_or("missing birth fit")?;
        let fem = b["fem_status"].as_str().unwrap_or("");
        per_gen_fits[g].push(fit);
        let cx = map_x(g as u64, id);
        let cy = map_y(fit);
        if fem == "fem_evaluated" || fem == "complete" {
            per_gen_fem[g] += 1;
            s.push_str(&format!(
                r##"<circle id="birth-{id}" cx="{}" cy="{}" r="2.4" fill="#0284c7" fill-opacity="0.65"/>"##,
                svg_coord(cx),
                svg_coord(cy)
            ));
        } else {
            per_gen_not[g] += 1;
            s.push_str(&format!(
                r##"<path id="birth-{id}" d="M{} {} L{} {} M{} {} L{} {}" stroke="#d97706" stroke-width="1.3" stroke-opacity="0.75"/>"##,
                svg_coord(cx - 2.2),
                svg_coord(cy - 2.2),
                svg_coord(cx + 2.2),
                svg_coord(cy + 2.2),
                svg_coord(cx - 2.2),
                svg_coord(cy + 2.2),
                svg_coord(cx + 2.2),
                svg_coord(cy - 2.2)
            ));
        }
    }

    let mut max_pts = Vec::new();
    let mut med_pts = Vec::new();
    for g in 0..17 {
        let mut arr = per_gen_fits[g].clone();
        arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mx = *arr.last().unwrap_or(&0.0);
        let med = if arr.len() % 2 == 1 {
            arr[arr.len() / 2]
        } else {
            0.5 * (arr[arr.len() / 2 - 1] + arr[arr.len() / 2])
        };
        let gx = x0 + (g as f64 / 16.0) * (x1 - x0);
        max_pts.push(format!("{},{}", svg_coord(gx), svg_coord(map_y(mx))));
        med_pts.push(format!("{},{}", svg_coord(gx), svg_coord(map_y(med))));
    }
    s.push_str(&format!(
        r##"<polyline fill="none" stroke="#0f172a" stroke-width="2.5" points="{}"/>"##,
        max_pts.join(" ")
    ));
    s.push_str(&format!(
        r##"<polyline fill="none" stroke="#0d9488" stroke-width="2.2" stroke-dasharray="5,3" points="{}"/>"##,
        med_pts.join(" ")
    ));

    // Bottom stacked bars per generation (y = 485..565)
    let by0 = 485.0_f64;
    let by1 = 565.0_f64;
    for g in 0..17 {
        let gx = x0 + (g as f64 / 16.0) * (x1 - x0);
        let bw = 26.0_f64;
        let fem_h = (per_gen_fem[g] as f64 / 128.0) * (by1 - by0);
        let not_h = (per_gen_not[g] as f64 / 128.0) * (by1 - by0);
        s.push_str(&format!(
            r##"<rect x="{}" y="{}" width="{}" height="{}" fill="#0284c7" fill-opacity="0.85"/>"##,
            svg_coord(gx - bw / 2.0),
            svg_coord(by1 - fem_h),
            svg_coord(bw),
            svg_coord(fem_h)
        ));
        s.push_str(&format!(
            r##"<rect x="{}" y="{}" width="{}" height="{}" fill="#f59e0b" fill-opacity="0.85"/>"##,
            svg_coord(gx - bw / 2.0),
            svg_coord(by1 - fem_h - not_h),
            svg_coord(bw),
            svg_coord(not_h)
        ));
        if g % 2 == 0 {
            s.push_str(&format!(
                r##"<text x="{}" y="{}" text-anchor="middle" font-size="21" fill="#334155">G{}</text>"##,
                svg_coord(gx),
                svg_coord(by1 + 26.0),
                g
            ));
        }
    }
    s.push_str(r##"<text x="40" y="630" font-size="21" fill="#475569">Conservation: G0 = 128 births; G1–G16 = 116 births/gen (1,856); total = 1,984 unique births.</text>"##);
    s.push_str(r##"<text x="40" y="662" font-size="21" fill="#475569">Peak search score at G9: Individual 1095 (fit = +1.15727; strict verification not met).</text>"##);
    s.push_str("</svg>\n");
    Ok(s)
}

/// Generate `docs/paper/figures/population-membership.svg` (`JR-R3(1)` fix: migration at `G4 / G8 / G12`).
fn render_population_membership_svg(hist_rec: &Value) -> Result<String> {
    let mut s = String::with_capacity(32_000);
    s.push_str(r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1200 740" width="1200" height="740" font-family="Inter, -apple-system, BlinkMacSystemFont, sans-serif">"##);
    s.push('\n');
    s.push_str(r##"<rect width="1200" height="740" fill="#f8fafc"/>"##);
    s.push_str(r##"<rect x="20" y="16" width="1160" height="708" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-width="1.5"/>"##);
    s.push_str(r##"<text x="40" y="48" font-size="24" font-weight="700" fill="#0f172a">Cohort 1 (Run 1): All 2,176 Island-Slot Memberships &amp; G4 / G8 / G12 Migrations</text>"##);
    s.push_str(r##"<text x="40" y="78" font-size="21" fill="#475569">1,984 new-birth placements (teal) + 192 elite reappearances (indigo) = 2,176 slot occurrences</text>"##);

    let x0 = 250.0_f64;
    let x1 = 1140.0_f64;
    let y0 = 128.0_f64;
    let y1 = 468.0_f64;
    let col_w = (x1 - x0) / 17.0;

    // Highlight G4, G8, G12 migration columns directly from raw JEV1 event generations
    let mig_gens = hist_rec["migration_generations"]
        .as_array()
        .ok_or("missing migration_generations in hist_rec")?;
    for mg_val in mig_gens {
        let mg = mg_val.as_u64().ok_or("invalid migration gen")? as usize;
        let gx = x0 + (mg as f64 + 0.5) * col_w;
        s.push_str(&format!(
            r##"<rect id="migration-highlight-G{mg}" x="{}" y="{}" width="{}" height="{}" rx="4" fill="#fef3c7" stroke="#f59e0b" stroke-width="1.5"/>"##,
            svg_coord(gx - col_w * 0.44),
            svg_coord(y0 - 24.0),
            svg_coord(col_w * 0.88),
            svg_coord(y1 - y0 + 58.0)
        ));
        s.push_str(&format!(
            r##"<text x="{}" y="{}" text-anchor="middle" font-size="21" font-weight="700" fill="#b45309">G{mg} MIG</text>"##,
            svg_coord(gx),
            svg_coord(y0 - 6.0)
        ));
    }

    // 4 island tracks across 17 generations
    for isl in 0..4usize {
        let iy0 = y0 + (isl as f64) * 85.0;
        let iy1 = iy0 + 66.0;
        s.push_str(&format!(
            r##"<rect x="{}" y="{}" width="{}" height="{}" fill="#f1f5f9" stroke="#cbd5e1" stroke-width="1"/>"##,
            svg_coord(x0),
            svg_coord(iy0),
            svg_coord(x1 - x0),
            svg_coord(iy1 - iy0)
        ));
        s.push_str(&format!(
            r##"<text x="{}" y="{}" text-anchor="end" font-size="21" font-weight="600" fill="#1e293b">Island {} (32 slots)</text>"##,
            svg_coord(x0 - 12.0),
            svg_coord(iy0 + 40.0),
            isl + 1
        ));

        for g in 0..17usize {
            let gx = x0 + (g as f64 + 0.5) * col_w;
            let bw = col_w * 0.62;
            let (new_cnt, elite_cnt) = if g == 0 { (32.0, 0.0) } else { (29.0, 3.0) };
            let new_h = (new_cnt / 32.0) * 66.0;
            let elite_h = (elite_cnt / 32.0) * 66.0;
            s.push_str(&format!(
                r##"<rect x="{}" y="{}" width="{}" height="{}" fill="#0d9488"/>"##,
                svg_coord(gx - bw / 2.0),
                svg_coord(iy1 - new_h),
                svg_coord(bw),
                svg_coord(new_h)
            ));
            if elite_h > 0.0 {
                s.push_str(&format!(
                    r##"<rect x="{}" y="{}" width="{}" height="{}" fill="#4f46e5"/>"##,
                    svg_coord(gx - bw / 2.0),
                    svg_coord(iy1 - new_h - elite_h),
                    svg_coord(bw),
                    svg_coord(elite_h)
                ));
            }
            if isl == 3 && g % 2 == 0 {
                s.push_str(&format!(
                    r##"<text x="{}" y="{}" text-anchor="middle" font-size="21" font-weight="600" fill="#1e293b">G{g}</text>"##,
                    svg_coord(gx),
                    svg_coord(y1 + 26.0)
                ));
            }
        }
    }

    // Bottom provenance & semantics box (y = 515..695)
    s.push_str(r##"<rect x="40" y="515" width="1120" height="185" rx="6" fill="#f8fafc" stroke="#cbd5e1" stroke-width="1"/>"##);
    s.push_str(r##"<text x="58" y="548" font-size="22" font-weight="700" fill="#0f172a">Pre-Migration Membership &amp; Duplicate Elite Slot Semantics (Run 1 Event Stream)</text>"##);
    s.push_str(r##"<text x="58" y="582" font-size="21" fill="#334155">• Total slot rows: 2,176 (G0: 128 new + 0 elites; G1–G16: 116 new + 12 elites/gen = 1,856 + 192).</text>"##);
    s.push_str(r##"<text x="58" y="616" font-size="21" fill="#334155">• Ring migrations: 12 events at G4, G8, G12 (Island 1→2, 2→3, 3→4, 4→1 × 2 copied IDs = 24 copies).</text>"##);
    s.push_str(r##"<text x="58" y="650" font-size="21" fill="#334155">• Pre-migration logging: island slot records are emitted before migration copy events.</text>"##);
    s.push_str(r##"<text x="58" y="684" font-size="21" fill="#334155">• Duplicate slot multiplicity: 9 (gen, island, id) keys appear twice in elite carry-forward slots.</text>"##);
    s.push_str("</svg>\n");
    Ok(s)
}

/// Generate `docs/paper/figures/population-newer.svg` (`JR-R2`, `JR-R3(2)` & `SC-DEC-183W` fix).
fn render_population_newer_svg(t05_receipt: &Value) -> Result<String> {
    let mut s = String::with_capacity(48_000);
    s.push_str(r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1200 760" width="1200" height="760" font-family="Inter, -apple-system, BlinkMacSystemFont, sans-serif">"##);
    s.push('\n');
    s.push_str(r##"<rect width="1200" height="760" fill="#f8fafc"/>"##);
    s.push_str(r##"<rect x="20" y="16" width="1160" height="728" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-width="1.5"/>"##);
    s.push_str(r##"<text x="40" y="48" font-size="24" font-weight="700" fill="#0f172a">Cohort 2 (Screening Corpus): 3,649 Birth Proposals &amp; 2,259 Joined Screen Rows</text>"##);
    s.push_str(r##"<text x="40" y="78" font-size="21" fill="#475569">Panel A: common linear scale (0–3,200 births). Panel B: all 57 round files across 24 run directories.</text>"##);

    // Panel A: Proportional horizontal bars on common scale 0..3200 (x = 340..788, 448 px = 3200 births -> 0.14 px/birth)
    let bx0 = 340.0_f64;
    let scale = 0.14_f64;
    s.push_str(r##"<text x="40" y="114" font-size="22" font-weight="700" fill="#0f172a">Panel A — Common-Scale Family Census (0 to 3,200 Birth Proposals; 1-Based Join)</text>"##);

    for tick in [0u64, 1000, 2000, 3000] {
        let tx = bx0 + (tick as f64) * scale;
        s.push_str(&format!(
            r##"<line x1="{}" y1="128" x2="{}" y2="308" stroke="#e2e8f0" stroke-width="1"/>"##,
            svg_coord(tx),
            svg_coord(tx)
        ));
        s.push_str(&format!(
            r##"<text x="{}" y="332" text-anchor="middle" font-size="21" fill="#475569">{}</text>"##,
            svg_coord(tx),
            tick
        ));
    }

    let fam_rows = [
        (
            "Wave Search (19 runs)",
            3194u64,
            1873u64,
            1321u64,
            136.0_f64,
        ),
        ("Plateau Search (4 runs)", 448u64, 379u64, 69u64, 196.0_f64),
        ("Automatic Probe (1 run)", 7u64, 7u64, 0u64, 256.0_f64),
    ];

    for (label_txt, total, screened, unscreened, ry) in fam_rows {
        let w_scr = (screened as f64) * scale;
        let w_unscr = (unscreened as f64) * scale;
        let pct_scr = (screened as f64 / total as f64) * 100.0;
        let pct_unscr = (unscreened as f64 / total as f64) * 100.0;

        s.push_str(&format!(
            r##"<text x="{}" y="{}" text-anchor="end" font-size="21" font-weight="600" fill="#1e293b">{}</text>"##,
            svg_coord(bx0 - 12.0),
            svg_coord(ry + 28.0),
            xml_escape(label_txt)
        ));
        s.push_str(&format!(
            r##"<rect x="{}" y="{}" width="{}" height="44" fill="#0d9488" stroke="#0f766e" stroke-width="0.8"/>"##,
            svg_coord(bx0),
            svg_coord(ry),
            svg_coord(w_scr)
        ));
        if w_unscr > 0.0 {
            s.push_str(&format!(
                r##"<rect x="{}" y="{}" width="{}" height="44" fill="#f59e0b" fill-opacity="0.85" stroke="#d97706" stroke-width="0.8"/>"##,
                svg_coord(bx0 + w_scr),
                svg_coord(ry),
                svg_coord(w_unscr)
            ));
        }
        let callout_x = bx0 + w_scr + w_unscr + 10.0;
        s.push_str(&format!(
            r##"<text x="{}" y="{}" font-size="21" font-weight="600" fill="#0f172a">{total}: {screened} scr ({pct_scr:.1}%)</text>"##,
            svg_coord(callout_x),
            svg_coord(ry + 18.0)
        ));
        s.push_str(&format!(
            r##"<text x="{}" y="{}" font-size="21" fill="#475569">+ {unscreened} unscr ({pct_unscr:.1}%)</text>"##,
            svg_coord(callout_x),
            svg_coord(ry + 42.0)
        ));
    }

    // Panel B: All 57 round files across 24 run directories (y = 390..615)
    s.push_str(r##"<text x="40" y="374" font-size="22" font-weight="700" fill="#0f172a">Panel B — All 57 Retained Round Files Across 24 Runs (Screened vs. Unscreened)</text>"##);
    let px0 = 96.0_f64;
    let px1 = 1140.0_f64;
    let py0 = 392.0_f64;
    let py1 = 612.0_f64;
    s.push_str(&format!(
        r##"<rect x="{}" y="{}" width="{}" height="{}" fill="#f8fafc" stroke="#cbd5e1" stroke-width="1"/>"##,
        svg_coord(px0),
        svg_coord(py0),
        svg_coord(px1 - px0),
        svg_coord(py1 - py0)
    ));

    for ytick in [0u64, 50, 100] {
        let ty = py1 - (ytick as f64 / 100.0) * (py1 - py0);
        s.push_str(&format!(
            r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#e2e8f0" stroke-width="1" stroke-dasharray="3,3"/>"##,
            svg_coord(px0),
            svg_coord(ty),
            svg_coord(px1),
            svg_coord(ty)
        ));
        s.push_str(&format!(
            r##"<text x="{}" y="{}" text-anchor="end" font-size="21" fill="#475569">{}</text>"##,
            svg_coord(px0 - 10.0),
            svg_coord(ty + 7.0),
            ytick
        ));
    }

    if let Some(files) = t05_receipt
        .get("file_pairs")
        .or_else(|| t05_receipt.get("per_file_reconciliation"))
        .and_then(Value::as_array)
    {
        let n = files.len().max(1);
        let slot_w = (px1 - px0) / (n as f64);
        let bw = (slot_w * 0.72).max(4.0);
        for (i, f) in files.iter().enumerate() {
            let b_cnt = f["births_count"].as_u64().unwrap_or(0) as f64;
            let s_cnt = f
                .get("screen_count")
                .or_else(|| f.get("screen_rows_count"))
                .and_then(Value::as_u64)
                .unwrap_or(0) as f64;
            let u_cnt = (b_cnt - s_cnt).max(0.0);
            let cx = px0 + (i as f64 + 0.5) * slot_w;
            let sh = (s_cnt / 100.0).min(1.0) * (py1 - py0);
            let uh = (u_cnt / 100.0).min(1.0 - sh / (py1 - py0)) * (py1 - py0);
            s.push_str(&format!(
                r##"<rect id="newer-file-{}" x="{}" y="{}" width="{}" height="{}" fill="#0d9488"/>"##,
                i + 1,
                svg_coord(cx - bw / 2.0),
                svg_coord(py1 - sh),
                svg_coord(bw),
                svg_coord(sh)
            ));
            if uh > 0.0 {
                s.push_str(&format!(
                    r##"<rect x="{}" y="{}" width="{}" height="{}" fill="#f59e0b" fill-opacity="0.85"/>"##,
                    svg_coord(cx - bw / 2.0),
                    svg_coord(py1 - sh - uh),
                    svg_coord(bw),
                    svg_coord(uh)
                ));
            }
            if i % 10 == 0 || i + 1 == n {
                s.push_str(&format!(
                    r##"<text x="{}" y="{}" text-anchor="middle" font-size="21" fill="#475569">F{}</text>"##,
                    svg_coord(cx),
                    svg_coord(py1 + 26.0),
                    i + 1
                ));
            }
        }
    }

    s.push_str(r##"<text x="40" y="676" font-size="21" fill="#475569">Conservation: 57 files (24 runs) = 3,649 proposals (2,259 screened [61.9%], 1,390 unscreened [38.1%]).</text>"##);
    s.push_str(r##"<text x="40" y="710" font-size="21" fill="#475569">1-based candidate index join: 2,259/2,259 exact matches; 0-based negative control rejects 2,247 misjoins.</text>"##);
    s.push_str("</svg>\n");
    Ok(s)
}

fn png_chunk(tag: &[u8; 4], data: &[u8], out: &mut Vec<u8>) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(tag);
    out.extend_from_slice(data);
    let mut crc = 0xffff_ffffu32;
    for &b in tag.iter().chain(data.iter()) {
        crc ^= b as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xedb8_8320
            } else {
                crc >> 1
            };
        }
    }
    out.extend_from_slice(&(!crc).to_be_bytes());
}

fn encode_point_png_b64(w: usize, h: usize, grid: &[u8]) -> String {
    let row_bytes = w.div_ceil(2);
    let mut raw = Vec::with_capacity(h * (1 + row_bytes));
    for r in 0..h {
        raw.push(0u8);
        for bx in 0..row_bytes {
            let p0 = grid[r * w + bx * 2] & 0x0f;
            let p1 = if bx * 2 + 1 < w {
                grid[r * w + bx * 2 + 1] & 0x0f
            } else {
                0
            };
            raw.push((p0 << 4) | p1);
        }
    }
    let mut zlib = Vec::with_capacity(raw.len() + 64);
    zlib.extend_from_slice(&[0x78, 0x01]);
    let chunks: Vec<&[u8]> = raw.chunks(65_535).collect();
    for (i, ch) in chunks.iter().enumerate() {
        zlib.push(if i + 1 == chunks.len() { 0x01 } else { 0x00 });
        let l = ch.len() as u16;
        zlib.extend_from_slice(&l.to_le_bytes());
        zlib.extend_from_slice(&(!l).to_le_bytes());
        zlib.extend_from_slice(ch);
    }
    let (mut s1, mut s2) = (1u32, 0u32);
    for &b in &raw {
        s1 = (s1 + b as u32) % 65_521;
        s2 = (s2 + s1) % 65_521;
    }
    zlib.extend_from_slice(&((s2 << 16) | s1).to_be_bytes());
    let mut png = vec![137, 80, 78, 71, 13, 10, 26, 10];
    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&(w as u32).to_be_bytes());
    ihdr.extend_from_slice(&(h as u32).to_be_bytes());
    ihdr.extend_from_slice(&[4, 3, 0, 0, 0]);
    png_chunk(b"IHDR", &ihdr, &mut png);
    let plte: [u8; 6] = [248, 250, 252, 51, 65, 85];
    png_chunk(b"PLTE", &plte, &mut png);
    png_chunk(b"IDAT", &zlib, &mut png);
    png_chunk(b"IEND", &[], &mut png);
    const B64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(png.len().div_ceil(3) * 4);
    for c in png.chunks(3) {
        let b0 = c[0] as usize;
        let b1 = if c.len() > 1 { c[1] as usize } else { 0 };
        let b2 = if c.len() > 2 { c[2] as usize } else { 0 };
        out.push(B64[b0 >> 2] as char);
        out.push(B64[((b0 & 3) << 4) | (b1 >> 4)] as char);
        out.push(if c.len() > 1 {
            B64[((b1 & 15) << 2) | (b2 >> 6)] as char
        } else {
            '='
        });
        out.push(if c.len() > 2 {
            B64[b2 & 63] as char
        } else {
            '='
        });
    }
    out
}

// A point operation occurs for every admitted row, including overlaps. The image
// stores neutral occupancy, never density or a majority legal/penalty class.
struct PointRaster {
    width: usize,
    height: usize,
    x_domain: [f64; 2],
    y_domain: [f64; 2],
    score_axis: crate::score_axis::ScoreAxis,
    pixels: Vec<u8>,
    point_operations: u64,
    overlap_operations: u64,
    observed_x: [f64; 2],
    observed_y: [f64; 2],
}

impl PointRaster {
    fn new(width: usize, height: usize, x_domain: [f64; 2], y_domain: [f64; 2]) -> Result<Self> {
        if width < 2
            || height < 2
            || x_domain
                .iter()
                .chain(y_domain.iter())
                .any(|v| !v.is_finite())
            || x_domain[0] > x_domain[1]
            || y_domain[0] > y_domain[1]
        {
            return Err("population: invalid point-raster domain".into());
        }
        Ok(Self {
            width,
            height,
            x_domain,
            y_domain,
            score_axis: crate::score_axis::ScoreAxis::Asinh {
                linear_threshold: 1.0,
            },
            pixels: vec![0; width * height],
            point_operations: 0,
            overlap_operations: 0,
            observed_x: [f64::INFINITY, f64::NEG_INFINITY],
            observed_y: [f64::INFINITY, f64::NEG_INFINITY],
        })
    }

    fn fraction(v: f64, domain: [f64; 2]) -> f64 {
        if domain[0] == domain[1] {
            0.5
        } else {
            (v - domain[0]) / (domain[1] - domain[0])
        }
    }

    fn with_axis(mut self, axis: crate::score_axis::ScoreAxis) -> Self {
        self.score_axis = axis;
        self
    }

    fn y_fraction(&self, y: f64) -> f64 {
        Self::fraction(
            self.score_axis.transform(y),
            [
                self.score_axis.transform(self.y_domain[0]),
                self.score_axis.transform(self.y_domain[1]),
            ],
        )
    }

    fn paint(&mut self, x: f64, y: f64) -> Result<[usize; 2]> {
        if !x.is_finite()
            || !y.is_finite()
            || x < self.x_domain[0]
            || x > self.x_domain[1]
            || y < self.y_domain[0]
            || y > self.y_domain[1]
        {
            return Err(
                format!("population: point outside full display domain: x={x}, y={y}").into(),
            );
        }
        let px = (Self::fraction(x, self.x_domain) * (self.width - 1) as f64).round() as usize;
        let py = self.height - 1 - (self.y_fraction(y) * (self.height - 1) as f64).round() as usize;
        let pixel = &mut self.pixels[py * self.width + px];
        self.overlap_operations += u64::from(*pixel != 0);
        *pixel = 1;
        self.point_operations += 1;
        self.observed_x[0] = self.observed_x[0].min(x);
        self.observed_x[1] = self.observed_x[1].max(x);
        self.observed_y[0] = self.observed_y[0].min(y);
        self.observed_y[1] = self.observed_y[1].max(y);
        Ok([px, py])
    }

    fn receipt(&self) -> Result<Value> {
        if self.point_operations == 0 {
            return Err("population: empty numeric point panel".into());
        }
        Ok(json!({
            "point_operations": self.point_operations,
            "overlap_operations": self.overlap_operations,
            "occupied_pixels": self.pixels.iter().filter(|&&p| p != 0).count(),
            "raster_px": [self.width, self.height],
            "x_display_domain": self.x_domain, "score_display_domain": self.y_domain,
            "observed_x_extrema": self.observed_x, "observed_score_extrema": self.observed_y,
            "vertical_transform": self.score_axis.metadata(),
            "pixel_sha256": sha256_bytes(&self.pixels),
            "clipping": false, "jitter": false, "majority_class_replacement": false,
            "point_color": "neutral occupancy; overlaps do not encode count"
        }))
    }
}

fn finite_point_number(v: &Value) -> Result<f64> {
    v.as_f64()
        .filter(|v| v.is_finite())
        .ok_or_else(|| "population: missing/nonfinite point value".into())
}

fn point_bounds(values: impl Iterator<Item = f64>) -> Result<[f64; 2]> {
    let mut bounds = [f64::INFINITY, f64::NEG_INFINITY];
    for value in values {
        if !value.is_finite() {
            return Err("population: nonfinite point domain".into());
        }
        bounds[0] = bounds[0].min(value);
        bounds[1] = bounds[1].max(value);
    }
    if !bounds[0].is_finite() {
        return Err("population: missing numeric point domain".into());
    }
    Ok(bounds)
}

#[derive(Clone, Copy, Debug)]
enum DetailDirection {
    Higher,
    Lower,
}

// Membership is computed in original score units, independently of the display
// transform. Constant domains stay exact; PointRaster centers tied observations.
fn directed_detail_bounds(values: &[f64], direction: DetailDirection) -> Result<[f64; 2]> {
    if values.is_empty() || values.iter().any(|value| !value.is_finite()) {
        return Err("population: empty or nonfinite directed-detail source".into());
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    Ok(match direction {
        DetailDirection::Higher => [
            crate::score_axis::quantile_sorted(&sorted, 0.90).unwrap(),
            sorted[sorted.len() - 1],
        ],
        DetailDirection::Lower => [
            sorted[0],
            crate::score_axis::quantile_sorted(&sorted, 0.10).unwrap(),
        ],
    })
}

fn detail_row_hash(hash: &mut Sha256, identity: &[u64], value: f64) {
    hash.update((identity.len() as u64).to_le_bytes());
    for part in identity {
        hash.update(part.to_le_bytes());
    }
    hash.update(value.to_bits().to_le_bytes());
}

struct DirectedDetail {
    bounds: [f64; 2],
    direction: DetailDirection,
    expected_counts: [u64; 3],
    counts: [u64; 3],
    extremum: f64,
    cutoff: f64,
    source_extremum_ties: u64,
    included_extremum_ties: u64,
    source_cutoff_ties: u64,
    included_cutoff_ties: u64,
    all_rows_hash: Sha256,
    selected_rows_hash: Sha256,
    extremum_rows_hash: Sha256,
}

impl DirectedDetail {
    fn new(values: &[f64], direction: DetailDirection) -> Result<Self> {
        let bounds = directed_detail_bounds(values, direction)?;
        let (extremum, cutoff) = match direction {
            DetailDirection::Higher => (bounds[1], bounds[0]),
            DetailDirection::Lower => (bounds[0], bounds[1]),
        };
        let mut expected_counts = [0; 3];
        for &value in values {
            expected_counts[if value < bounds[0] { 0 } else if value > bounds[1] { 2 } else { 1 }] += 1;
        }
        Ok(Self {
            bounds, direction, expected_counts, counts: [0; 3], extremum, cutoff,
            source_extremum_ties: values.iter().filter(|&&v| v == extremum).count() as u64,
            included_extremum_ties: 0,
            source_cutoff_ties: values.iter().filter(|&&v| v == cutoff).count() as u64,
            included_cutoff_ties: 0,
            all_rows_hash: Sha256::new(), selected_rows_hash: Sha256::new(),
            extremum_rows_hash: Sha256::new(),
        })
    }

    fn observe(&mut self, identity: &[u64], value: f64) -> Result<bool> {
        if identity.is_empty() || !value.is_finite() {
            return Err("population: malformed directed-detail identity or score".into());
        }
        let bucket = if value < self.bounds[0] { 0 } else if value > self.bounds[1] { 2 } else { 1 };
        self.counts[bucket] += 1;
        detail_row_hash(&mut self.all_rows_hash, identity, value);
        self.all_rows_hash.update([bucket as u8]);
        if bucket == 1 {
            detail_row_hash(&mut self.selected_rows_hash, identity, value);
            if value == self.extremum {
                self.included_extremum_ties += 1;
                detail_row_hash(&mut self.extremum_rows_hash, identity, value);
            }
            self.included_cutoff_ties += u64::from(value == self.cutoff);
        }
        Ok(bucket == 1)
    }

    fn receipt(&self, raster: &PointRaster, identity_fields: &[&str]) -> Result<Value> {
        let forbidden_tail = match self.direction { DetailDirection::Higher => 2, DetailDirection::Lower => 0 };
        if self.counts != self.expected_counts || self.counts[forbidden_tail] != 0
            || raster.point_operations != self.counts[1]
            || raster.y_domain != self.bounds || raster.observed_y != self.bounds
            || self.included_extremum_ties != self.source_extremum_ties
            || self.included_cutoff_ties != self.source_cutoff_ties
        {
            return Err("population: directed-detail count, boundary or extremum retention differs".into());
        }
        Ok(json!({
            "direction": match self.direction { DetailDirection::Higher => "higher_is_better", DetailDirection::Lower => "lower_is_better" },
            "selection": match self.direction { DetailDirection::Higher => "Q90..MAX", DetailDirection::Lower => "MIN..Q10" },
            "bounds": self.bounds, "membership_bounds_original_units": self.bounds,
            "display_axis_widened": false, "constant_domain_policy": "exact membership and display domain retained; tied points centered, no widened selection",
            "numeric_universe": self.expected_counts.iter().sum::<u64>(),
            "below_inside_above": self.counts, "cutoff": self.cutoff,
            "cutoff_ties": {"source_count": self.source_cutoff_ties, "included_count": self.included_cutoff_ties},
            "extremum": {"value": self.extremum, "source_tie_count": self.source_extremum_ties,
                "included_tie_count": self.included_extremum_ties,
                "ordered_identity_score_sha256": format!("{:x}", self.extremum_rows_hash.clone().finalize())},
            "identity_fields": identity_fields,
            "hash_encoding": "ordered LE u64 identity arity, each identity field, original f64 score bits; all-row hash additionally appends u8 below=0/inside=1/above=2",
            "all_rows_membership_sha256": format!("{:x}", self.all_rows_hash.clone().finalize()),
            "selected_identity_score_sha256": format!("{:x}", self.selected_rows_hash.clone().finalize()),
            "membership_transform": "none; inclusive original scores and all boundary ties",
            "raster": raster.receipt()?
        }))
    }
}

fn point_panel(
    svg: &mut String,
    raster: &PointRaster,
    id: &str,
    rect: [f64; 4],
    x_label: &str,
    y_label: &str,
) {
    let [x, y, width, height] = rect;
    let encoded = encode_point_png_b64(raster.width, raster.height, &raster.pixels);
    svg.push_str(&format!(
        r##"<rect x="{x}" y="{y}" width="{width}" height="{height}" fill="none" stroke="#94a3b8"/><image id="{id}" x="{x}" y="{y}" width="{width}" height="{height}" preserveAspectRatio="none" data-numeric-scored-points="{}" data-point-operations="{}" data-excluded-or-invalid-points="0" href="data:image/png;base64,{encoded}"/>"##,
        raster.point_operations, raster.point_operations
    ));
    for value in raster.score_axis.ticks(raster.y_domain) {
        let ty = y + height * (1.0 - raster.y_fraction(value));
        svg.push_str(&format!(r##"<text x="{}" y="{}" text-anchor="end" font-size="21" fill="#334155">{value:.4}</text>"##, x - 10.0, svg_coord(ty + 7.0)));
    }
    let mut x_ticks = Vec::new();
    for i in 0..5 {
        let value = (raster.x_domain[0]
            + i as f64 / 4.0 * (raster.x_domain[1] - raster.x_domain[0]))
            .round();
        if x_ticks.last() != Some(&value) {
            x_ticks.push(value);
        }
    }
    for value in x_ticks {
        let tx = x + width * PointRaster::fraction(value, raster.x_domain);
        svg.push_str(&format!(r##"<text x="{}" y="{}" text-anchor="middle" font-size="21" fill="#334155">{value:.0}</text>"##, svg_coord(tx), y + height + 28.0));
    }
    let axis_key = match raster.score_axis {
        crate::score_axis::ScoreAxis::Linear => "linear".to_string(),
        crate::score_axis::ScoreAxis::Asinh { linear_threshold } => {
            format!("asinh(y/L), L={linear_threshold:.5}")
        }
    };
    let y_label = format!("{y_label}; {axis_key}");
    svg.push_str(&format!(r##"<text x="{}" y="{}" text-anchor="middle" font-size="21" fill="#334155">{}</text><text x="{x}" y="{}" font-size="21" fill="#334155">{}</text>"##,
        x + width / 2.0, y + height + 56.0, xml_escape(x_label), y - 12.0, xml_escape(&y_label)));
}

fn render_population_latest_svg(
    root: &Path,
    births: &[Value],
    newer_rec: &Value,
    t05: &Value,
) -> Result<(String, Value, BTreeMap<String, String>)> {
    // Existing archive admission checks compressed/logical/source CSV hashes,
    // per-run original IDs and complete row/generation/legal censuses before return.
    let archive_root = crate::corpus::resolve_archive_path(
        root,
        "artifacts/versioned-evolution-archives-gen800-population-v1",
    )?;
    let mut native_points = Vec::with_capacity(1_954_560);
    let mut native_visits = 0u64;
    let mut native_legal = 0u64;
    let mut negative_legal = 0u64;
    let mut visit_hash = Sha256::new();
    let mut paint_hash = Sha256::new();
    let mut run_visits: BTreeMap<usize, (u64, Sha256)> = BTreeMap::new();
    let mut on_row =
        |run_idx: usize, id: u64, generation: u64, fit: f64, legal: bool| -> Result<()> {
            native_visits += 1;
            for part in [
                (run_idx as u64).to_le_bytes(),
                id.to_le_bytes(),
                generation.to_le_bytes(),
                fit.to_bits().to_le_bytes(),
            ] {
                visit_hash.update(part);
            }
            visit_hash.update([u8::from(legal)]);
            native_points.push((run_idx, id, generation, fit));
            let run = run_visits
                .entry(run_idx)
                .or_insert_with(|| (0, Sha256::new()));
            run.0 += 1;
            run.1.update(id.to_le_bytes());
            native_legal += u64::from(legal);
            negative_legal += u64::from(legal && fit < 0.0);
            Ok(())
        };
    let admission = crate::corpus::verify_and_decode_gen800_archive_with_callback(
        &archive_root,
        Some(crate::corpus::GEN800_POPULATION_INDEX_SHA256),
        None,
        Some(&mut on_row),
    )?;
    if admission["full_archive_decoded"] != true
        || admission["rows_decoded"] != native_visits
        || native_visits != 1_954_560
        || native_legal != 754_236
        || negative_legal != 610_051
        || run_visits.len() != 10
        || run_visits.values().any(|(count, _)| *count != 195_456)
    {
        return Err("population: native source visits and point operations disagree".into());
    }
    let native_values: Vec<_> = native_points.iter().map(|p| p.3).collect();
    let native_axis = crate::score_axis::ScoreAxis::derive(&native_values)?;
    let native_bounds = point_bounds(native_values.iter().copied())?;
    let mut native_selection = DirectedDetail::new(&native_values, DetailDirection::Higher)?;
    let native_detail_bounds = native_selection.bounds;
    let mut native =
        PointRaster::new(1201, 321, [0.0, 800.0], native_bounds)?.with_axis(native_axis);
    let mut native_detail =
        PointRaster::new(1201, 321, [0.0, 800.0], native_detail_bounds)?.with_axis(native_axis);
    let mut native_leaders: BTreeMap<usize, (f64, Vec<(u64, u64)>)> = BTreeMap::new();
    for &(run, id, generation, fit) in &native_points {
        let pixel = native.paint(generation as f64, fit)?;
        for part in [
            (run as u64).to_le_bytes(),
            id.to_le_bytes(),
            (pixel[0] as u64).to_le_bytes(),
            (pixel[1] as u64).to_le_bytes(),
        ] {
            paint_hash.update(part);
        }
        if native_selection.observe(&[run as u64, id, generation], fit)? {
            native_detail.paint(generation as f64, fit)?;
        }
        let leader = native_leaders.entry(run).or_insert((fit, Vec::new()));
        if fit > leader.0 {
            leader.0 = fit;
            leader.1.clear();
        }
        if fit == leader.0 {
            leader.1.push((id, generation));
        }
    }
    let native_tails = native_selection.counts;
    if native.point_operations != native_visits || native_tails.iter().sum::<u64>() != native_visits
    {
        return Err("population: full/detail point accounting differs".into());
    }
    let index: Value = serde_json::from_slice(&crate::corpus::read_bounded_file(
        &crate::corpus::resolve_safe_relpath(&archive_root, "index.gen800_population.sha256.json")?,
        crate::corpus::HARD_SEGMENT_CEILING_BYTES,
    )?)?;
    let run_receipts: Vec<Value> = run_visits
        .into_iter()
        .map(|(ordinal, (count, ids))| {
            json!({
                "archive_run_ordinal": ordinal, "package_id": index["runs"][ordinal]["package_id"],
                "original_id_order_sha256": format!("{:x}", ids.finalize()),
                "records_visited": count, "point_operations": count
            })
        })
        .collect();

    let mut historical_points = Vec::with_capacity(births.len());
    let mut historical_ids = BTreeSet::new();
    let mut historical_hash = Sha256::new();
    for row in births {
        let id = row["id"]
            .as_u64()
            .ok_or("population: missing historical id")?;
        let generation = row["gen"]
            .as_u64()
            .ok_or("population: missing historical generation")?;
        let fit = finite_point_number(&row["fit"])?;
        if !historical_ids.insert(id) {
            return Err("population: duplicate historical id".into());
        }
        historical_points.push((id, generation, fit));
    }
    if historical_points.len() != 1_984 {
        return Err("population: historical point census differs".into());
    }
    let historical_values: Vec<_> = historical_points.iter().map(|p| p.2).collect();
    let historical_axis = crate::score_axis::ScoreAxis::derive(&historical_values)?;
    let mut historical_selection = DirectedDetail::new(&historical_values, DetailDirection::Higher)?;
    let historical_detail_bounds = historical_selection.bounds;
    let mut historical_detail = PointRaster::new(951, 221, [0.0, 16.0], historical_detail_bounds)?
        .with_axis(historical_axis);
    let mut historical = PointRaster::new(
        951,
        221,
        point_bounds(historical_points.iter().map(|p| p.1 as f64))?,
        point_bounds(historical_points.iter().map(|p| p.2))?,
    )?
    .with_axis(historical_axis);
    for &(id, generation, fit) in &historical_points {
        let pixel = historical.paint(generation as f64, fit)?;
        if historical_selection.observe(&[id, generation], fit)? {
            historical_detail.paint(generation as f64, fit)?;
        }
        for part in [
            id.to_le_bytes(),
            generation.to_le_bytes(),
            fit.to_bits().to_le_bytes(),
            (pixel[0] as u64).to_le_bytes(),
            (pixel[1] as u64).to_le_bytes(),
        ] {
            historical_hash.update(part);
        }
    }

    let historical_tails = historical_selection.counts;

    // Bind the existing raw joined screen reader to every admitted original pair.
    // Its sorted birth-file index is the x identity, never a synthetic generation.
    let pairs = t05["file_pairs"]
        .as_array()
        .ok_or("population: missing screen source pairs")?;
    let mut pair_map = BTreeMap::new();
    for pair in pairs {
        let birth_path = pair["births_path"]
            .as_str()
            .ok_or("population: missing birth source path")?;
        if pair_map.insert(birth_path, pair).is_some() {
            return Err("population: duplicate admitted source pair".into());
        }
        for (path_key, hash_key) in [
            ("births_path", "births_sha256"),
            ("screen_path", "screen_sha256"),
        ] {
            let rel = pair[path_key]
                .as_str()
                .ok_or("population: missing source-pair path")?;
            let bytes = crate::corpus::read_bounded_file(
                &crate::corpus::resolve_safe_relpath(root, rel)?,
                crate::corpus::HARD_SEGMENT_CEILING_BYTES,
            )?;
            if pair[hash_key].as_str() != Some(sha256_bytes(&bytes).as_str()) {
                return Err(format!("population: unadmitted screen source bytes: {rel}").into());
            }
        }
    }
    if pair_map.len() != 57 {
        return Err("population: screen source-pair census differs".into());
    }
    let source_pairs: Vec<Value> = pair_map.values().enumerate().map(|(i, pair)| json!({
        "birth_file_ordinal": i + 1, "births_path": pair["births_path"], "births_sha256": pair["births_sha256"],
        "screen_path": pair["screen_path"], "screen_sha256": pair["screen_sha256"]
    })).collect();
    let mut screen_points = Vec::new();
    let mut proposal_keys = BTreeSet::new();
    for row in newer_rec["screened_points"]
        .as_array()
        .ok_or("population: missing numeric screens")?
    {
        let file = row[0]
            .as_u64()
            .ok_or("population: missing screen file index")?;
        let candidate = row[1]
            .as_u64()
            .ok_or("population: missing screen candidate index")?;
        let deficit = finite_point_number(&row[2])?;
        if file >= 57 || candidate == 0 || !proposal_keys.insert((file, candidate)) {
            return Err("population: duplicate or invalid scored proposal identity".into());
        }
        screen_points.push((file, candidate, deficit));
    }
    if screen_points.len() != 2_259 {
        return Err("population: scored screen census differs".into());
    }
    let screening_values: Vec<_> = screen_points.iter().map(|p| p.2).collect();
    let screening_axis = crate::score_axis::ScoreAxis::derive(&screening_values)?;
    let mut screening_selection = DirectedDetail::new(&screening_values, DetailDirection::Lower)?;
    let screening_detail_bounds = screening_selection.bounds;
    let mut screening_detail =
        PointRaster::new(951, 221, [1.0, 57.0], screening_detail_bounds)?.with_axis(screening_axis);
    let mut screening = PointRaster::new(
        951,
        221,
        [1.0, 57.0],
        point_bounds(screen_points.iter().map(|p| p.2))?,
    )?
    .with_axis(screening_axis);
    let mut screening_hash = Sha256::new();
    for &(file, candidate, deficit) in &screen_points {
        let pixel = screening.paint((file + 1) as f64, deficit)?;
        if screening_selection.observe(&[file + 1, candidate], deficit)? {
            screening_detail.paint((file + 1) as f64, deficit)?;
        }
        for part in [
            file.to_le_bytes(),
            candidate.to_le_bytes(),
            deficit.to_bits().to_le_bytes(),
            (pixel[0] as u64).to_le_bytes(),
            (pixel[1] as u64).to_le_bytes(),
        ] {
            screening_hash.update(part);
        }
    }
    let screening_tails = screening_selection.counts;
    let unknown = newer_rec["unscreened_points"]
        .as_array()
        .ok_or("population: missing unknown proposals")?;
    let mut unknown_hash = Sha256::new();
    for row in unknown {
        let file = row[0]
            .as_u64()
            .ok_or("population: missing unknown file index")?;
        let candidate = row[1]
            .as_u64()
            .ok_or("population: missing unknown candidate index")?;
        if file >= 57 || candidate == 0 || !proposal_keys.insert((file, candidate)) {
            return Err("population: duplicate/overlapping unknown proposal identity".into());
        }
        unknown_hash.update(file.to_le_bytes());
        unknown_hash.update(candidate.to_le_bytes());
    }
    if unknown.len() != 1_390 || proposal_keys.len() != 3_649 {
        return Err("population: unknown proposal census differs".into());
    }

    let mut receipt = json!({
        "schema": "jarambit.population_point_rendering.v1",
        "numeric_records_visited": native_visits + historical_points.len() as u64 + screen_points.len() as u64,
        "numeric_point_operations": native.point_operations + historical.point_operations + screening.point_operations,
        "selected_figure_universe": "ten retained native runs, historical births, joined numeric screens; unknown proposals disclosed in a nonnumeric coverage note",
        "excluded_numeric_records": 0,
        "native": {"admission": admission, "source_visit_sha256": format!("{:x}", visit_hash.finalize()),
            "paint_operation_sha256": format!("{:x}", paint_hash.finalize()),
            "identity_namespace": "archive package_id + original CSV id; no cross-run deduplication",
            "source_visit_hash_encoding": "ordered LE u64: run ordinal, original id, generation, f64 fit bits; u8 legal",
            "paint_hash_encoding": "ordered LE u64: run ordinal, original id, raster x, raster y",
            "runs": run_receipts, "legal_true_rows": native_legal, "legal_false_rows": native_visits - native_legal,
            "legal_true_with_negative_fit": negative_legal, "raster": native.receipt()?},
        "historical": {"source_path": HISTORICAL_ALL_EVALS_JSON_RELPATH,
            "source_sha256": HISTORICAL_ALL_EVALS_JSON_SHA256,
            "source_and_paint_sha256": format!("{:x}", historical_hash.finalize()), "raster": historical.receipt()?},
        "screening": {"source_pairs": source_pairs, "identity_namespace": "original birth-file path + one-based candidate index",
            "source_and_paint_sha256": format!("{:x}", screening_hash.finalize()), "raster": screening.receipt()?},
        "unknown": {"records": unknown.len(), "numeric_point_operations": 0, "display": "nonnumeric coverage note; no plotted band or invented score",
            "source_identity_sha256": format!("{:x}", unknown_hash.finalize())}
    });
    let registry: Value =
        serde_json::from_slice(&fs::read(root.join("docs/paper/data/public_ids.v1.json"))?)?;
    let start = registry["gen800_population_range_allocation"]["public_id_start"]
        .as_u64()
        .ok_or("population: missing admitted native alias range")?;
    let mut packages: Vec<_> = index["runs"]
        .as_array()
        .ok_or("population: missing native runs")?
        .iter()
        .map(|r| {
            r["package_id"]
                .as_str()
                .ok_or("population: missing package ID")
        })
        .collect::<std::result::Result<_, _>>()?;
    packages.sort();
    let newer_start = registry["newer_birth_range_allocation"]["public_id_start"]
        .as_u64()
        .ok_or("population: missing proposal alias range")?;
    let mut proposal_offsets = Vec::new();
    let mut proposal_total = 0u64;
    for pair in &source_pairs {
        proposal_offsets.push(proposal_total);
        let rel = pair["births_path"]
            .as_str()
            .ok_or("population: missing proposal source")?;
        let source: Value =
            serde_json::from_slice(&fs::read(crate::corpus::resolve_safe_relpath(root, rel)?)?)?;
        proposal_total += source["genomes"]
            .as_array()
            .ok_or("population: missing original genomes list")?
            .len() as u64;
    }
    if proposal_total != 3649 {
        return Err("population: registered proposal alias census differs".into());
    }
    let native_leader_rows: Vec<_> = native_leaders.iter().map(|(&run,(fit,ties))| {
        let package = index["runs"][run]["package_id"].as_str().unwrap();
        let ordinal = packages.iter().position(|p| *p == package).unwrap() as u64;
        json!({"archive_run_ordinal":run,"package_id":package,"native_direction":"higher_is_better","selection":"argmax original retained fitness; exact stored-value ties","value":fit,"tie_count":ties.len(),"ties":ties.iter().map(|&(id,generation)|json!({"original_id":id,"generation":generation,"public_individual":start+ordinal*195456+id})).collect::<Vec<_>>(),"later_assessment":null,"assessment_reason":"No retained complete per-birth CAD/assessment join","source":index["runs"][run],"comparability":"within this original run and retained stage only; producer/scorer equivalence across regimes not asserted"})
    }).collect();
    let historical_max = historical_values
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    let screening_min = screening_values
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min);
    receipt["retained_leaders"] = json!({"native_by_run":native_leader_rows,"historical":{"native_direction":"higher_is_better","value":historical_max,"ties":historical_points.iter().filter(|p| p.2==historical_max).map(|p|json!({"original_id":p.0,"public_individual":p.0,"generation":p.1})).collect::<Vec<_>>(),"source":HISTORICAL_ALL_EVALS_JSON_RELPATH,"source_sha256":HISTORICAL_ALL_EVALS_JSON_SHA256,"later_assessment":"Individual1095: earlier run-specific Boolean predicate cert_pass_all=false, seq_ok=false; not a later 26-requirement qualification result"},"screening":{"native_direction":"lower_is_better","value":screening_min,"ties":screen_points.iter().filter(|p|p.2==screening_min).map(|p|json!({"birth_file_ordinal":p.0+1,"candidate_one_based":p.1,"public_individual":newer_start+proposal_offsets[p.0 as usize]+p.1-1,"source_pair":source_pairs[p.0 as usize]})).collect::<Vec<_>>(),"later_assessment":null},"global_comparable_champion":null,"selection_on_display_transform":false,"tie_rule":"retain all exact original numeric ties in source traversal order"});
    receipt["detail"] = json!({
        "policy":"higher-is-better Q90..MAX; lower-is-better MIN..Q10; floor(p*(n-1)); inclusive cutoff and extremum ties",
        "full_range_companion":"docs/paper/figures/population-latest.svg",
        "pooled_native_scope":"display context only; not a comparable global engineering winner or connected cross-run trajectory",
        "native":native_selection.receipt(&native_detail, &["archive_run_ordinal","original_id","generation"])?,
        "historical":historical_selection.receipt(&historical_detail, &["original_id","generation"])?,
        "screening":screening_selection.receipt(&screening_detail, &["birth_file_ordinal_one_based","candidate_one_based"])?
    });
    let mut outputs = BTreeMap::new();
    let mut run_context_receipts = Vec::new();
    let run_keys: Vec<_> = native_leaders.keys().copied().collect();
    for pair in run_keys.chunks(2) {
        let mut svg = String::from(
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1200 930" width="1200" height="930" font-family="Inter, -apple-system, BlinkMacSystemFont, sans-serif"><rect width="1200" height="930" fill="#ffffff"/><text x="40" y="42" font-size="25" font-weight="700" fill="#0f172a">Native Run Context — Separate Original Runs</text><text x="40" y="76" font-size="21" fill="#475569">Every original row; retained-best birth trace is context, not physical improvement.</text>"##,
        );
        let mut public_runs = Vec::new();
        for (slot, &run) in pair.iter().enumerate() {
            let package = index["runs"][run]["package_id"]
                .as_str()
                .ok_or("population: missing context package")?;
            let record = registry["runs"]
                .as_array()
                .ok_or("population: missing registry runs")?
                .iter()
                .find(|r| r["internal_run_key"] == package)
                .ok_or("population: unregistered context run")?;
            let public_run = record["public_run_id"]
                .as_u64()
                .ok_or("population: missing public run")?;
            public_runs.push(public_run);
            let values: Vec<_> = native_points
                .iter()
                .filter(|p| p.0 == run)
                .map(|p| p.3)
                .collect();
            let axis = crate::score_axis::ScoreAxis::derive(&values)?;
            let mut raster =
                PointRaster::new(951, 221, [0., 800.], point_bounds(values.iter().copied())?)?
                    .with_axis(axis);
            let mut generation_max = vec![f64::NEG_INFINITY; 801];
            for &(ordinal, _, generation, fit) in &native_points {
                if ordinal == run {
                    raster.paint(generation as f64, fit)?;
                    generation_max[generation as usize] =
                        generation_max[generation as usize].max(fit);
                }
            }
            let top = 120. + slot as f64 * 420.;
            svg.push_str(&format!(r##"<text x="40" y="{top}" font-size="23" font-weight="700" fill="#0f172a">Run {public_run} — 195,456 retained birth observations</text>"##));
            point_panel(
                &mut svg,
                &raster,
                &format!("native-run-{public_run}"),
                [190., top + 48., 950., 220.],
                "Recorded generation",
                "Native contextual fitness (higher is better)",
            );
            let mut best = f64::NEG_INFINITY;
            let mut trace = String::new();
            let mut trace_values = Vec::new();
            for (generation, fit) in generation_max.into_iter().enumerate() {
                best = best.max(fit);
                if !best.is_finite() {
                    return Err("population: missing initial retained-best context".into());
                }
                trace_values.push(best);
                trace.push_str(&format!(
                    "{:.3},{:.3} ",
                    190. + 950. * generation as f64 / 800.,
                    top + 48. + 220. * (1. - raster.y_fraction(best))
                ));
            }
            svg.push_str(&format!(
                r##"<polyline points="{trace}" fill="none" stroke="#0f766e" stroke-width="2.5"/>"##
            ));
            let (fit, ties) = &native_leaders[&run];
            svg.push_str(&format!(r##"<text x="40" y="{}" font-size="21" fill="#475569">Retained maximum {fit:.4}; {} tied rows. No per-birth downstream assessment retained.</text>"##,top+350.,ties.len()));
            run_context_receipts.push(json!({"archive_run_ordinal":run,"public_run_id":public_run,"package_id":package,"source":index["runs"][run],"raster":raster.receipt()?,"axis":axis.metadata(),"retained_best_birth_trace":trace_values,"trace_scope":"cumulative maximum of original retained birth fitness, monotone by construction; not post-rescore membership or physical quality"}));
        }
        svg.push_str("</svg>\n");
        outputs.insert(
            format!(
                "docs/paper/generated/native-runs-{}-{}.svg",
                public_runs[0], public_runs[1]
            ),
            svg,
        );
    }
    receipt["run_context"] = json!(run_context_receipts);
    receipt["point_operations_by_view"] = json!({"full_overview":native.point_operations+historical.point_operations+screening.point_operations,"additional_directed_tail_detail":native_detail.point_operations+historical_detail.point_operations+screening_detail.point_operations,"additional_separate_native_run_context":native_points.len(),"interpretation":"The same retained observations are shown in multiple explicitly labeled views; additional views are not new data."});
    let mut full = String::new();
    for detail in [false, true] {
        let height = if detail { 1360 } else { 1280 };
        let mut svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1200 {height}" width="1200" height="{height}" font-family="Inter, -apple-system, BlinkMacSystemFont, sans-serif"><rect width="1200" height="{height}" fill="#ffffff"/>"##
        );
        let title = if detail {
            "High-Fitness / Low-Deficit Tail Detail — Full Range Retained"
        } else {
            "Every Retained Scored Record — Separate Source Profiles"
        };
        let subtitle = if detail {
            "Original-score tails; all boundary/extremum ties; companion: population-latest.svg."
        } else {
            "Original score units; source-derived axes; overlapping marks are neutral occupancy."
        };
        svg.push_str(&format!(r##"<text x="40" y="42" font-size="25" font-weight="700" fill="#0f172a">{title}</text><text x="40" y="76" font-size="21" fill="#475569">{subtitle}</text>"##));
        for (i, (raster, label, x_label, y_label, tails)) in [
            (
                if detail { &native_detail } else { &native },
                if detail { "A / Runs 2–11 — Higher-fitness tail (pooled context only)" } else { "A / Runs 2–11 — 1,954,560 native observations" },
                "Recorded generation",
                "Native contextual fitness (higher is better)",
                native_tails,
            ),
            (
                if detail {
                    &historical_detail
                } else {
                    &historical
                },
                if detail { "B / Run 1 — Higher-fitness tail" } else { "B / Run 1 — 1,984 historical births" },
                "Recorded generation",
                "Historical search fitness (higher is better)",
                historical_tails,
            ),
            (
                if detail {
                    &screening_detail
                } else {
                    &screening
                },
                if detail { "C / Joined numeric screens — Lower-deficit tail" } else { "C / 2,259 joined numeric screens" },
                "Retained birth-file ordinal",
                "Screen deficit (lower is better)",
                screening_tails,
            ),
        ]
        .into_iter()
        .enumerate()
        {
            let top = 120.0 + i as f64 * if detail { 400.0 } else { 365.0 };
            svg.push_str(&format!(r##"<text x="40" y="{top}" font-size="23" font-weight="700" fill="#0f172a">{label}</text>"##));
            point_panel(
                &mut svg,
                raster,
                &format!(
                    "{}-per-record-point-raster",
                    ["gen800", "historical", "screening"][i]
                ),
                [190.0, top + 48.0, 950.0, 220.0],
                x_label,
                y_label,
            );
            if detail {
                let selection = [&native_selection, &historical_selection, &screening_selection][i];
                let range = if i == 2 { "MIN..Q10" } else { "Q90..MAX" };
                svg.push_str(&format!(r##"<text x="190" y="{}" font-size="21" fill="#475569">{range}: [{:.4}, {:.4}]; inside {} (all cutoff ties).</text><text x="190" y="{}" font-size="21" fill="#475569">Below {}; above {}; retained extreme {:.4}; {} tied rows.</text>"##,
                    top+350.0,selection.bounds[0],selection.bounds[1],tails[1],
                    top+378.0,tails[0],tails[2],selection.extremum,selection.source_extremum_ties));
            }
        }
        svg.push_str(&format!(r##"<text x="40" y="{}" font-size="21" fill="#92400e" data-unknown-proposal-count="1390">1,390 proposals have no retained numeric screen value; no score is imputed.</text>"##,height-20));
        svg.push_str("</svg>\n");
        if detail {
            outputs.insert("docs/paper/generated/population-detail.svg".into(), svg);
        } else {
            full = svg;
        }
    }
    outputs.insert("docs/paper/generated/population/score-axis.v1.json".into(),serde_json::to_string_pretty(&json!({"schema":"jarambit.score_axis_projection.v1","contract":"docs/paper/data/score-axis-contract.v1.json","native":native_axis.metadata(),"historical":historical_axis.metadata(),"screening":screening_axis.metadata(),"detail":receipt["detail"],"full_domain_clipping":false}))?+"\n");
    outputs.insert(
        "docs/paper/generated/population/retained-leaders.v1.json".into(),
        serde_json::to_string_pretty(&receipt["retained_leaders"])? + "\n",
    );
    append_positive_point_views(&native_points, &historical_points, &screen_points, &mut receipt, &mut outputs)?;
    let s = full;
    Ok((s, receipt, outputs))
}

pub const W411C3_FITNESS_CURVE_RELPATH: &str =
    "evidence/evolution_waves/2026-10-18_ultralight_v1/xbabe3/w411c3/fitness_curve.json";
pub const W411C3_FITNESS_CURVE_SHA256: &str =
    "afe065170218f0f5f20942a9e737e7e91a300244f8e6a99fd3c1d617d16f1412";

fn render_fitness_progress_svg(
    births: &[Value],
    w411c3_curve: &Value,
    w411c3_sha256: &str,
    gen800_density: &Value,
) -> Result<String> {
    let mut s = String::with_capacity(64_000);
    s.push_str(r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1200 1080" width="1200" height="1080" font-family="Inter, -apple-system, BlinkMacSystemFont, sans-serif">"##);
    s.push('\n');
    s.push_str(r##"<rect width="1200" height="1080" fill="#f8fafc"/>"##);
    s.push_str(r##"<rect x="20" y="16" width="1160" height="1048" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-width="1.5"/>"##);
    s.push_str(r##"<text x="40" y="48" font-size="24" font-weight="700" fill="#0f172a">Profile-Separated Fitness Progression Across Cohorts (No Cross-Regime Pooling)</text>"##);
    s.push_str(r##"<text x="40" y="78" font-size="21" fill="#475569">Each cohort uses its own native objective definition and scale (qualified count = 0).</text>"##);

    let ax0 = 115.0_f64;
    let ax1 = 485.0_f64;
    let ay0 = 130.0_f64;
    let ay1 = 330.0_f64;
    s.push_str(r##"<text x="40" y="114" font-size="22" font-weight="700" fill="#0f172a">Panel A — Cohort 1 (Run 1): 17-Gen Search Fitness (1,984 Births, Gen 0–16)</text>"##);
    s.push_str(&format!(r##"<rect x="{}" y="{}" width="{}" height="{}" fill="#f8fafc" stroke="#cbd5e1" stroke-width="1"/>"##, svg_coord(ax0), svg_coord(ay0), svg_coord(ax1 - ax0), svg_coord(ay1 - ay0)));
    let map_ay = |v: f64| -> f64 { ay1 - ((v - 0.80) / 0.40).clamp(0.0, 1.0) * (ay1 - ay0) };
    for yv in [0.80_f64, 1.00, 1.20] {
        let ty = map_ay(yv);
        s.push_str(&format!(r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#e2e8f0" stroke-width="1" stroke-dasharray="3,3"/><text x="{}" y="{}" text-anchor="end" font-size="21" fill="#475569">{yv:+.2}</text>"##, svg_coord(ax0), svg_coord(ty), svg_coord(ax1), svg_coord(ty), svg_coord(ax0 - 8.0), svg_coord(ty + 7.0)));
    }
    for gv in (0..=16usize).step_by(4) {
        let tx = ax0 + (gv as f64 / 16.0) * (ax1 - ax0);
        s.push_str(&format!(r##"<text x="{}" y="{}" text-anchor="middle" font-size="21" fill="#475569">G{gv}</text>"##, svg_coord(tx), svg_coord(ay1 + 26.0)));
    }
    let mut g_max = vec![-1e9_f64; 17];
    for b in births {
        let g = b["gen"].as_u64().unwrap_or(0) as usize;
        let f = b["fit"].as_f64().unwrap_or(-1e9);
        if f > g_max[g] {
            g_max[g] = f;
        }
    }
    let (mut cum_pts, mut gen_pts, mut cur_best) = (Vec::new(), Vec::new(), -1e9_f64);
    for g in 0..17usize {
        if g_max[g] > cur_best {
            cur_best = g_max[g];
        }
        let tx = ax0 + (g as f64 / 16.0) * (ax1 - ax0);
        let (cy_cum, cy_gen) = (map_ay(cur_best), map_ay(g_max[g]));
        cum_pts.push(format!("{},{}", svg_coord(tx), svg_coord(cy_cum)));
        gen_pts.push(format!("{},{}", svg_coord(tx), svg_coord(cy_gen)));
        s.push_str(&format!(r##"<circle cx="{}" cy="{}" r="3.0" fill="#0f172a"/><circle cx="{}" cy="{}" r="2.5" fill="#0d9488"/>"##, svg_coord(tx), svg_coord(cy_cum), svg_coord(tx), svg_coord(cy_gen)));
    }
    s.push_str(&format!(r##"<polyline fill="none" stroke="#0f172a" stroke-width="2.2" points="{}"/><polyline fill="none" stroke="#0d9488" stroke-width="1.8" stroke-dasharray="4,3" points="{}"/>"##, cum_pts.join(" "), gen_pts.join(" ")));
    s.push_str(r##"<rect x="505" y="130" width="655" height="200" rx="6" fill="#f8fafc" stroke="#cbd5e1" stroke-width="1"/>"##);
    s.push_str(r##"<text x="518" y="160" font-size="22" font-weight="700" fill="#0f172a">Cohort 1 (Run 1) Summary</text>"##);
    s.push_str(r##"<text x="518" y="192" font-size="21" fill="#334155">• Cum. best: +1.1573 at G9 (Ind 1095)</text>"##);
    s.push_str(r##"<text x="518" y="224" font-size="21" fill="#334155">• Per-gen max: +0.961..+1.157</text>"##);
    s.push_str(r##"<text x="518" y="256" font-size="21" fill="#334155">• Strict cert_pass_all = false</text>"##);
    s.push_str(r##"<text x="518" y="288" font-size="21" fill="#334155">• Probes: Ind 2001 (-3.8787), Ind 2008 (+1.7790)</text>"##);

    let (by0, by1) = (420.0_f64, 640.0_f64);
    s.push_str(r##"<text x="40" y="402" font-size="22" font-weight="700" fill="#0f172a">Panel B — Cohort 2 (Run 31): 3-Gen Weighted Reporting Trajectory (Runs 31–36)</text>"##);
    s.push_str(&format!(r##"<rect x="{}" y="{}" width="{}" height="{}" fill="#f8fafc" stroke="#cbd5e1" stroke-width="1"/>"##, svg_coord(ax0), svg_coord(by0), svg_coord(ax1 - ax0), svg_coord(by1 - by0)));
    let map_by = |v: f64| -> f64 { by1 - ((v - 70.0) / 5.0).clamp(0.0, 1.0) * (by1 - by0) };
    for yv in [70.0_f64, 72.5, 75.0] {
        let ty = map_by(yv);
        s.push_str(&format!(r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#e2e8f0" stroke-width="1" stroke-dasharray="3,3"/><text x="{}" y="{}" text-anchor="end" font-size="21" fill="#475569">{yv:.1}</text>"##, svg_coord(ax0), svg_coord(ty), svg_coord(ax1), svg_coord(ty), svg_coord(ax0 - 8.0), svg_coord(ty + 7.0)));
    }
    let gen_rows = w411c3_curve["generations"]
        .as_array()
        .ok_or("missing generations")?;
    let (mut hof_pts, mut rnd_pts, mut parsed_rows) =
        (Vec::new(), Vec::new(), Vec::with_capacity(3));
    for (idx, row) in gen_rows.iter().enumerate() {
        let gen = row["generation"].as_u64().ok_or("missing generation")?;
        let hof = row["hall_of_fame_best_so_far"]
            .as_f64()
            .ok_or("missing hof")?;
        let rnd = row["round_best_fitness"].as_f64().ok_or("missing rnd")?;
        parsed_rows.push((gen, hof, rnd));
        let tx = ax0 + (gen as f64 / 2.0) * (ax1 - ax0);
        s.push_str(&format!(r##"<text x="{}" y="{}" text-anchor="middle" font-size="21" fill="#475569">Gen {gen} (R{})</text>"##, svg_coord(tx), svg_coord(by1 + 26.0), gen + 1));
        let (ty_hof, ty_rnd) = (map_by(hof), map_by(rnd));
        hof_pts.push(format!("{},{}", svg_coord(tx), svg_coord(ty_hof)));
        rnd_pts.push(format!("{},{}", svg_coord(tx), svg_coord(ty_rnd)));
        s.push_str(&format!(r##"<circle id="run31-gen{gen}-archive" data-run="Run 31" data-series="hall_of_fame_best_so_far" data-source-sha256="{w411c3_sha256}" data-source-pointer="/generations/{idx}/hall_of_fame_best_so_far" data-generation="{gen}" data-value="{hof:.6}" cx="{}" cy="{}" r="4.0" fill="#0d9488"/>"##, svg_coord(tx), svg_coord(ty_hof)));
        s.push_str(&format!(r##"<circle id="run31-gen{gen}-round" data-run="Run 31" data-series="round_best_fitness" data-source-sha256="{w411c3_sha256}" data-source-pointer="/generations/{idx}/round_best_fitness" data-generation="{gen}" data-value="{rnd:.6}" cx="{}" cy="{}" r="3.2" fill="#d97706"/>"##, svg_coord(tx), svg_coord(ty_rnd)));
    }
    s.push_str(&format!(r##"<polyline fill="none" stroke="#0d9488" stroke-width="2.4" points="{}"/><polyline fill="none" stroke="#d97706" stroke-width="2.0" stroke-dasharray="4,3" points="{}"/>"##, hof_pts.join(" "), rnd_pts.join(" ")));
    let ((_, g0_hof, g0_rnd), (_, g1_hof, g1_rnd), (_, g2_hof, g2_rnd)) =
        (parsed_rows[0], parsed_rows[1], parsed_rows[2]);
    s.push_str(r##"<rect x="505" y="420" width="655" height="220" rx="6" fill="#f8fafc" stroke="#cbd5e1" stroke-width="1"/>"##);
    s.push_str(r##"<text x="518" y="448" font-size="21" font-weight="700" fill="#0f172a">Cohort 2 Selected Run 31 Summary (Retained Profile)</text>"##);
    s.push_str(&format!(r##"<text x="518" y="478" font-size="21" fill="#334155">• Gen 0 (Ind 2009): archive {g0_hof:.6} / round {g0_rnd:.6}</text>"##));
    s.push_str(&format!(r##"<text x="518" y="508" font-size="21" fill="#334155">• Gen 1 (Ind 2002): archive {g1_hof:.6} / round {g1_rnd:.6}</text>"##));
    s.push_str(&format!(r##"<text x="518" y="538" font-size="21" fill="#334155">• Gen 2 (Ind 2002): archive {g2_hof:.6} / round {g2_rnd:.6}</text>"##));
    s.push_str(r##"<text x="518" y="568" font-size="21" fill="#334155">• Selected Individual 2002: F = 74.207299 / 100</text>"##);
    s.push_str(r##"<text x="518" y="598" font-size="21" fill="#334155">• Runs 31–36 census in weighted-runs.svg</text>"##);

    let (cy0, cy1) = (735.0_f64, 965.0_f64);
    s.push_str(r##"<text x="40" y="716" font-size="22" font-weight="700" fill="#0f172a">Panel C — Cohort 3: Ten Retained 800-Gen Runs (Run 2–Run 11, 1,954,560 Rows)</text>"##);
    s.push_str(&format!(r##"<rect x="{}" y="{}" width="{}" height="{}" fill="#f8fafc" stroke="#cbd5e1" stroke-width="1"/>"##, svg_coord(ax0), svg_coord(cy0), svg_coord(ax1 - ax0), svg_coord(cy1 - cy0)));
    let map_cy = |v: f64| -> f64 { cy1 - ((v + 1.00) / 2.20).clamp(0.0, 1.0) * (cy1 - cy0) };
    for yv in [-1.00_f64, 0.00, 1.20] {
        let ty = map_cy(yv);
        s.push_str(&format!(r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#e2e8f0" stroke-width="1" stroke-dasharray="3,3"/><text x="{}" y="{}" text-anchor="end" font-size="21" fill="#475569">{yv:+.2}</text>"##, svg_coord(ax0), svg_coord(ty), svg_coord(ax1), svg_coord(ty), svg_coord(ax0 - 8.0), svg_coord(ty + 7.0)));
    }
    for gv in (0..=800usize).step_by(200) {
        let tx = ax0 + (gv as f64 / 800.0) * (ax1 - ax0);
        s.push_str(&format!(r##"<text x="{}" y="{}" text-anchor="middle" font-size="21" fill="#475569">G{gv}</text>"##, svg_coord(tx), svg_coord(cy1 + 26.0)));
    }
    let (
        mut g0_lmed_min,
        mut g0_lmed_max,
        mut g0_non_null,
        mut all_lmed_min,
        mut g800_lmed_min,
        mut g800_lmed_max,
    ) = (
        f64::INFINITY,
        f64::NEG_INFINITY,
        0usize,
        f64::INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
    );
    if let Some(runs) = gen800_density["runs"].as_array() {
        for run in runs {
            let cum_arr = run["gen_cum_best_fit"]
                .as_array()
                .ok_or("missing gen_cum_best_fit")?;
            let lmed_arr = run["gen_legal_median_fit"]
                .as_array()
                .ok_or("missing gen_legal_median_fit")?;
            if let Some(v0) = lmed_arr.first().and_then(Value::as_f64) {
                g0_non_null += 1;
                g0_lmed_min = g0_lmed_min.min(v0);
                g0_lmed_max = g0_lmed_max.max(v0);
            }
            for lv in lmed_arr.iter().filter_map(Value::as_f64) {
                all_lmed_min = all_lmed_min.min(lv);
            }
            if let Some(v800) = lmed_arr.get(800).and_then(Value::as_f64) {
                g800_lmed_min = g800_lmed_min.min(v800);
                g800_lmed_max = g800_lmed_max.max(v800);
            }
            let (mut cpts, mut mpts) = (Vec::new(), Vec::new());
            for g in (0..=800usize).step_by(10) {
                let tx = ax0 + (g as f64 / 800.0) * (ax1 - ax0);
                if let Some(cb) = cum_arr.get(g).and_then(Value::as_f64) {
                    cpts.push(format!("{},{}", svg_coord(tx), svg_coord(map_cy(cb))));
                }
                if let Some(lm) = lmed_arr.get(g).and_then(Value::as_f64) {
                    mpts.push(format!("{},{}", svg_coord(tx), svg_coord(map_cy(lm))));
                } else if mpts.len() >= 2 {
                    s.push_str(&format!(r##"<polyline fill="none" stroke="#0284c7" stroke-width="1.2" stroke-opacity="0.65" points="{}"/>"##, mpts.join(" ")));
                    mpts.clear();
                } else {
                    mpts.clear();
                }
            }
            if mpts.len() >= 2 {
                s.push_str(&format!(r##"<polyline fill="none" stroke="#0284c7" stroke-width="1.2" stroke-opacity="0.65" points="{}"/>"##, mpts.join(" ")));
            }
            s.push_str(&format!(r##"<polyline fill="none" stroke="#0f172a" stroke-width="1.4" stroke-opacity="0.75" points="{}"/>"##, cpts.join(" ")));
        }
    }
    s.push_str(r##"<rect x="505" y="735" width="655" height="230" rx="6" fill="#f8fafc" stroke="#cbd5e1" stroke-width="1"/>"##);
    s.push_str(r##"<text x="518" y="765" font-size="21" font-weight="700" fill="#0f172a">Cohort 3 Native Search Summary (Runs 2–11)</text>"##);
    s.push_str(r##"<text x="518" y="798" font-size="21" fill="#334155">• Cum. bests: +1.1089..+1.1440 (Run 5 best)</text>"##);
    s.push_str(r##"<text x="518" y="831" font-size="21" fill="#334155">• Null-median generations left un-bridged</text>"##);
    s.push_str(r##"<text x="518" y="864" font-size="21" fill="#334155">• 1,954,560 rows (754,236 legal [38.59%])</text>"##);
    s.push_str(r##"<text x="518" y="897" font-size="21" fill="#334155">• Stage-2 3D solid/plate: unavailable retained assessment</text>"##);
    s.push_str(&format!(r##"<text x="40" y="1036" font-size="21" fill="#334155">• Cohort 3 legal medians (G0: {g0_non_null}/10 {g0_lmed_min:.4}..{g0_lmed_max:.4}; min {all_lmed_min:.4}; G800: {g800_lmed_min:.4}..{g800_lmed_max:.4})</text>"##));
    s.push_str("</svg>\n");
    Ok(s)
}

/// Generate the main ancestry DAG or its supplementary coordinate-detail view from `inheritance.json`.
fn render_inheritance_svg(inheritance_ledger: &Value, include_details: bool) -> Result<String> {
    let mut s = String::with_capacity(32_000);
    let (height, card_height) = if include_details {
        (1160, 1128)
    } else {
        (744, 712)
    };
    s.push_str(&format!(r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1200 {height}" width="1200" height="{height}" font-family="Inter, -apple-system, BlinkMacSystemFont, sans-serif">"##));
    s.push('\n');
    s.push_str(&format!(
        r##"<rect width="1200" height="{height}" fill="#f8fafc"/>"##
    ));
    s.push_str(&format!(r##"<rect x="20" y="16" width="1160" height="{card_height}" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-width="1.5"/>"##));
    if include_details {
        s.push_str(r##"<text x="40" y="48" font-size="24" font-weight="700" fill="#0f172a">Recorded 3-Hop Ancestry DAG &amp; 128-Gene Differences (Individual 1095, Run 1)</text>"##);
        s.push_str(r##"<text x="40" y="78" font-size="21" fill="#475569">7 birth nodes &amp; 7 directed 3-hop edges (+ 1 dashed 4th-hop closure edge 174→299); Individual 413 reused</text>"##);
    } else {
        s.push_str(r##"<text x="40" y="48" font-size="24" font-weight="700" fill="#0f172a">Recorded Ancestry — Individual 1095, Run 1</text>"##);
        s.push_str(r##"<text x="40" y="78" font-size="21" fill="#475569">7 births; arrows run parent → child. Individual 413 is reused.</text>"##);
    }

    s.push_str(r##"<defs><marker id="parent-child-arrow" viewBox="0 0 10 10" refX="10" refY="5" markerWidth="8" markerHeight="8" orient="auto"><path d="M0,0 L10,5 L0,10 Z" fill="#334155"/></marker></defs>"##);

    // Top panel: DAG (x = 40..1160, y = 98..636)
    s.push_str(r##"<rect x="40" y="98" width="1120" height="538" rx="6" fill="#f8fafc" stroke="#cbd5e1" stroke-width="1"/>"##);
    let pos_map: BTreeMap<u64, (f64, f64)> = BTreeMap::from([
        (174, (600.0, 140.0)),
        (245, (315.0, 254.0)),
        (299, (885.0, 254.0)),
        (413, (315.0, 368.0)),
        (392, (885.0, 368.0)),
        (526, (820.0, 482.0)),
        (1095, (420.0, 594.0)),
    ]);

    let mut node_lines: BTreeMap<u64, (String, String)> = BTreeMap::new();
    if let Some(nodes) = inheritance_ledger["focal_subgraph_nodes"].as_array() {
        for n in nodes {
            let id = n["public_id"].as_u64().ok_or("missing node public_id")?;
            let gen = n["generation"].as_u64().ok_or("missing node generation")?;
            let fit = n["stored_fit"].as_f64().ok_or("missing node stored_fit")?;
            let op = n["operator"].as_str().ok_or("missing node operator")?;
            let full_op = match op {
                "archive_crossover_mutate" => "Archive crossover + mutation",
                "archive_mutate" => "Archive mutation",
                "crossover_mutate" => "Crossover + mutation",
                "mutate" => "Mutation",
                other => other,
            };
            let role_tag = if id == 1095 {
                " [focal]"
            } else if id == 413 {
                " [reused]"
            } else if id == 174 {
                " [branch]"
            } else {
                ""
            };
            node_lines.insert(
                id,
                (
                    if include_details {
                        format!("Individual {id} (Gen {gen}, {fit:+.5}){role_tag}")
                    } else {
                        format!("Individual {id} (Gen {gen}){role_tag}")
                    },
                    full_op.to_string(),
                ),
            );
        }
    }

    if let Some(edges) = inheritance_ledger["parent_child_gene_diffs"].as_array() {
        for e in edges {
            let p = e["parent_id"].as_u64().unwrap_or(0);
            let c = e["child_id"].as_u64().unwrap_or(0);
            let chg = e["changed_f64_gene_count"].as_u64().unwrap_or(0);
            if let (Some(&(x1, y1)), Some(&(x2, y2))) = (pos_map.get(&p), pos_map.get(&c)) {
                s.push_str(&format!(
                    r##"<line id="edge-{p}-{c}" data-hops="3" marker-end="url(#parent-child-arrow)" x1="{}" y1="{}" x2="{}" y2="{}" stroke="#334155" stroke-width="2.4"/>"##,
                    svg_coord(x1), svg_coord(y1 + 28.0), svg_coord(x2), svg_coord(y2 - 35.0)
                ));
                if include_details {
                    let (lx, ly) = match (p, c) {
                        (174, 245) => (0.5 * (x1 + x2) - 24.0, 0.5 * (y1 + y2) + 5.0),
                        (413, 526) => (0.5 * (x1 + x2) - 36.0, 0.5 * (y1 + y2) + 6.0),
                        (392, 526) => (0.5 * (x1 + x2) + 32.0, 0.5 * (y1 + y2) + 6.0),
                        (413, 1095) => (0.5 * (x1 + x2) - 42.0, 0.5 * (y1 + y2) + 8.0),
                        (526, 1095) => (0.5 * (x1 + x2) + 50.0, 0.5 * (y1 + y2) + 8.0),
                        _ => (0.5 * (x1 + x2), 0.5 * (y1 + y2) + 6.0),
                    };
                    s.push_str(&format!(
                        r##"<rect x="{}" y="{}" width="128" height="24" rx="4" fill="#f8fafc" fill-opacity="0.94"/><text x="{}" y="{}" text-anchor="middle" font-size="21" font-weight="600" fill="#0f766e">Δ={chg}/128</text>"##,
                        svg_coord(lx - 64.0), svg_coord(ly - 19.0), svg_coord(lx), svg_coord(ly)
                    ));
                }
            }
        }
    }

    if let Some(edges4) = inheritance_ledger["fourth_hop_induced_closure_edges"].as_array() {
        for e in edges4 {
            let p = e["parent_id"].as_u64().unwrap_or(0);
            let c = e["child_id"].as_u64().unwrap_or(0);
            let chg = e["changed_f64_gene_count"].as_u64().unwrap_or(0);
            if let (Some(&(x1, y1)), Some(&(x2, y2))) = (pos_map.get(&p), pos_map.get(&c)) {
                s.push_str(&format!(
                    r##"<line id="edge-{p}-{c}-4th-hop" data-hops="4" marker-end="url(#parent-child-arrow)" x1="{}" y1="{}" x2="{}" y2="{}" stroke="#64748b" stroke-width="2.0" stroke-dasharray="5,4"/>"##,
                    svg_coord(x1), svg_coord(y1 + 28.0), svg_coord(x2), svg_coord(y2 - 35.0)
                ));
                if include_details {
                    let lx4 = 0.5 * (x1 + x2) + 50.0;
                    let ly4 = 0.5 * (y1 + y2) + 5.0;
                    s.push_str(&format!(
                        r##"<rect x="{}" y="{}" width="230" height="24" rx="4" fill="#f8fafc" fill-opacity="0.94"/><text x="{}" y="{}" text-anchor="middle" font-size="21" font-weight="600" fill="#475569">Δ={chg}/128 (4th hop)</text>"##,
                        svg_coord(lx4 - 115.0), svg_coord(ly4 - 19.0), svg_coord(lx4), svg_coord(ly4)
                    ));
                }
            }
        }
    }

    for (&id, &(cx, cy)) in &pos_map {
        let (l1, l2) = node_lines
            .get(&id)
            .map(|(a, b)| (a.as_str(), b.as_str()))
            .unwrap_or(("Individual ?", ""));
        let fill = if id == 1095 {
            "#0f172a"
        } else if id == 413 {
            "#0d9488"
        } else {
            "#0284c7"
        };
        s.push_str(&format!(
            r##"<rect id="node-{id}" x="{}" y="{}" width="516" height="58" rx="6" fill="{fill}"/>"##,
            svg_coord(cx - 258.0), svg_coord(cy - 29.0)
        ));
        s.push_str(&format!(
            r##"<text x="{}" y="{}" text-anchor="middle" font-size="21" font-weight="700" fill="#ffffff">{}</text>"##,
            svg_coord(cx), svg_coord(cy - 4.0), xml_escape(l1)
        ));
        s.push_str(&format!(
            r##"<text x="{}" y="{}" text-anchor="middle" font-size="21" fill="#e2e8f0">{}</text>"##,
            svg_coord(cx),
            svg_coord(cy + 21.0),
            xml_escape(l2)
        ));
    }

    if include_details {
        // Bottom panel: 128-gene diff bars + lineage disclosure (x = 40..1160, y = 650..1126)
        s.push_str(r##"<rect x="40" y="650" width="1120" height="476" rx="6" fill="#f8fafc" stroke="#cbd5e1" stroke-width="1"/>"##);
        s.push_str(r##"<text x="60" y="684" font-size="22" font-weight="700" fill="#0f172a">Directed Edge Changes Across 128 Numeric Coordinates (Δ != 0)</text>"##);

        if let Some(edges) =
            inheritance_ledger["induced_subgraph_parent_child_gene_diffs"].as_array()
        {
            for (i, e) in edges.iter().enumerate() {
                let p = e["parent_id"].as_u64().unwrap_or(0);
                let c = e["child_id"].as_u64().unwrap_or(0);
                let chg = e["changed_f64_gene_count"].as_u64().unwrap_or(0);
                let iden = e["identical_f64_gene_count"].as_u64().unwrap_or(0);
                let l2 = e["l2_gene_distance"].as_f64().unwrap_or(0.0);
                let within3 = e["within_3_hop_edge_horizon"].as_bool().unwrap_or(true);
                let ry = 702.0 + (i as f64) * 36.0;
                let w_chg = (chg as f64 / 128.0) * 320.0;
                let w_id = (iden as f64 / 128.0) * 320.0;
                let edge_lbl = if within3 {
                    format!("{p} → {c}")
                } else {
                    format!("{p} → {c}*")
                };
                s.push_str(&format!(
                    r##"<text x="60" y="{}" font-size="21" font-weight="600" fill="#1e293b">{}</text><rect x="210" y="{}" width="{}" height="22" fill="#0d9488"/><rect x="{}" y="{}" width="{}" height="22" fill="#cbd5e1"/><text x="550" y="{}" font-size="21" fill="#334155">{chg}/128 changed (L2 = {l2:.3})</text>"##,
                    svg_coord(ry + 18.0), xml_escape(&edge_lbl), svg_coord(ry), svg_coord(w_chg), svg_coord(210.0 + w_chg), svg_coord(ry), svg_coord(w_id), svg_coord(ry + 18.0)
                ));
            }
        }

        s.push_str(r##"<text x="60" y="1015" font-size="21" fill="#475569">*174 → 299 is the 4th-hop induced closure edge; Ti-6Al-4V material (col 145) unchanged on all 8 edges.</text>"##);
        s.push_str(r##"<text x="60" y="1048" font-size="21" fill="#334155">• Run 1 (1,984 births): 128 roots + 938 1-parent mutations + 918 2-parent crossovers.</text>"##);
        s.push_str(r##"<text x="60" y="1080" font-size="21" fill="#334155">• Screening (3,649 proposals) &amp; Runs 2–11 (1,954,560 rows): parent_ids = unavailable_in_source.</text>"##);
    } else {
        s.push_str(r##"<text x="40" y="672" font-size="21" fill="#475569">Dashed arrow 174 → 299: induced fourth-hop closure outside the three-hop traversal.</text>"##);
        s.push_str(r##"<text x="40" y="706" font-size="21" fill="#334155">Stored scores and numeric-coordinate differences are retained in the supplement.</text>"##);
    }
    s.push_str("</svg>\n");
    Ok(s)
}

#[derive(Clone, Copy, PartialEq)]
enum HeroView {
    FullRange,
    ContrastPreview,
    MeshOn,
    Support,
}

/// Render high-contrast first-page hero SVG (`SC-DEC-183AL` Item 3) directly from retained
/// `cert_pipeline_r01_p14_c02.stl` (`90,000` triangles) and `cert_pipeline_r01_p14_c02_D1.json`
/// (`25,209` triangles) under proper `SO(3)` pose (`det = +1.0`) with crisp dark silhouette/hole
/// contours and readable `28–34px` typography.
pub fn render_hero_exact_cad_svg(repo_root: &Path) -> Result<String> {
    Ok(render_hero_view(repo_root, HeroView::FullRange)?.0)
}

const HERO_D1_REL: &str = "evidence/evolve_certifiable/automatic/by-content/db5d8e32e99d0b2d8ef6d7c7f9622a0601487c25b80879bbc8be1a502693c55e/attempt-1792206525341070917/cert_pipeline_r01_p14_c02_D1.json";
const HERO_STL_REL: &str = "evidence/evolve_certifiable/automatic/by-content/db5d8e32e99d0b2d8ef6d7c7f9622a0601487c25b80879bbc8be1a502693c55e/attempt-1792206525341070917/cert_pipeline_r01_p14_c02.stl";
const HERO_SUBJECT: &str = "Individual 2000";
const HERO_ORDERED_GEOMETRY_SHA: &str =
    "270fe8fee8aaa0d74c5a59d8b310c4ffc454fe985e479f86203c77fb48bb3b73";

// Shared predecessor source loading, exact nearest-cell registration and camera.
// Flat quantitative output keeps its original culling/painter/SVG code below.
struct HeroSource {
    d1_bytes: Vec<u8>,
    stl_bytes: Vec<u8>,
    d1: Value,
    composite: crate::display::Composite,
    raw_tris: Vec<([f64; 3], [[f64; 3]; 3])>,
    face_to_cell: Vec<usize>,
    edge_counts: BTreeMap<(usize, usize), usize>,
    posed_nodes: Vec<[f64; 2]>,
    ct: f64,
    st: f64,
    scale: f64,
    ox: f64,
    oy: f64,
}

fn hero_nearest_cell(
    cx: f64,
    cy: f64,
    centroids: &[[f64; 2]],
    grid_buckets: &BTreeMap<(i32, i32), Vec<usize>>,
) -> Result<usize> {
    let (gx, gy) = ((cx * 0.5).floor() as i32, (cy * 0.5).floor() as i32);
    let mut best_i = None;
    let mut best_d2 = f64::INFINITY;
    for dx in -2..=2 {
        for dy in -2..=2 {
            if let Some(bucket) = grid_buckets.get(&(gx + dx, gy + dy)) {
                for &di in bucket {
                    let d2 = (centroids[di][0] - cx).powi(2) + (centroids[di][1] - cy).powi(2);
                    if d2 < best_d2 || (d2 == best_d2 && best_i.map_or(true, |old| di < old)) {
                        best_d2 = d2;
                        best_i = Some(di);
                    }
                }
            }
        }
    }
    // The grid accelerates an exact nearest-centroid query. If an outside
    // bucket could contain a nearer cell, finish against the complete source.
    let outside_margin = (cx - (gx - 2) as f64 * 2.0)
        .min((gx + 3) as f64 * 2.0 - cx)
        .min(cy - (gy - 2) as f64 * 2.0)
        .min((gy + 3) as f64 * 2.0 - cy);
    if best_d2 >= outside_margin.powi(2) {
        for (di, center) in centroids.iter().enumerate() {
            let d2 = (center[0] - cx).powi(2) + (center[1] - cy).powi(2);
            if d2 < best_d2 || (d2 == best_d2 && best_i.map_or(true, |old| di < old)) {
                best_d2 = d2;
                best_i = Some(di);
            }
        }
    }
    best_i.ok_or_else(|| "population: no source cell for surface display transfer".into())
}

fn load_hero_source(repo_root: &Path) -> Result<HeroSource> {
    let d1_bytes = crate::corpus::read_bounded_file(
        &crate::corpus::resolve_safe_relpath(repo_root, HERO_D1_REL)?,
        16 * 1024 * 1024,
    )?;
    let stl_bytes = crate::corpus::read_bounded_file(
        &crate::corpus::resolve_safe_relpath(repo_root, HERO_STL_REL)?,
        16 * 1024 * 1024,
    )?;
    let d1: Value = serde_json::from_slice(&d1_bytes)?;
    let xy_arr = d1["mesh"]["xy"].as_array().ok_or("missing D1 xy")?;
    let tris_arr = d1["mesh"]["tris"].as_array().ok_or("missing D1 tris")?;
    let xy: Vec<[f64; 2]> = xy_arr
        .iter()
        .map(|p| {
            let a = p.as_array().unwrap();
            [a[0].as_f64().unwrap(), a[1].as_f64().unwrap()]
        })
        .collect();
    let tris: Vec<[usize; 3]> = tris_arr
        .iter()
        .map(|t| {
            let a = t.as_array().unwrap();
            [
                a[0].as_u64().unwrap() as usize,
                a[1].as_u64().unwrap() as usize,
                a[2].as_u64().unwrap() as usize,
            ]
        })
        .collect();
    let n_tris = tris.len();
    let mut areas = Vec::with_capacity(n_tris);
    let mut centroids = Vec::with_capacity(n_tris);
    let mut edge_counts: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    for t in &tris {
        let [a, b, c] = [xy[t[0]], xy[t[1]], xy[t[2]]];
        let area = 0.5 * ((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])).abs();
        areas.push(area);
        centroids.push([(a[0] + b[0] + c[0]) / 3.0, (a[1] + b[1] + c[1]) / 3.0]);
        for (u, v) in [(t[0], t[1]), (t[1], t[2]), (t[2], t[0])] {
            let e = if u < v { (u, v) } else { (v, u) };
            *edge_counts.entry(e).or_insert(0) += 1;
        }
    }
    let mut composite = crate::display::composite(repo_root, &d1, &areas)?;
    if composite.receipt["display_transfer"]["surface_source"]["sha256"].as_str()
        != Some(sha256_bytes(&stl_bytes).as_str())
    {
        return Err("population: retained hero surface binding differs".into());
    }
    composite
        .input_hashes
        .insert(HERO_STL_REL.into(), sha256_bytes(&stl_bytes));

    // Proper SO(3) pose R = R_z(-6 deg) * R_x(180 deg) (det = +1.0)
    let theta = -6.0_f64.to_radians();
    let (ct, st) = (theta.cos(), theta.sin());
    let pose_xy = |x: f64, y: f64| -> [f64; 2] {
        let yr = -y;
        [x * ct - yr * st, x * st + yr * ct]
    };
    let posed_nodes: Vec<[f64; 2]> = xy.iter().map(|p| pose_xy(p[0], p[1])).collect();
    let mut xmin = f64::INFINITY;
    let mut xmax = f64::NEG_INFINITY;
    let mut ymin = f64::INFINITY;
    let mut ymax = f64::NEG_INFINITY;
    for p in &posed_nodes {
        xmin = xmin.min(p[0]);
        xmax = xmax.max(p[0]);
        ymin = ymin.min(p[1]);
        ymax = ymax.max(p[1]);
    }
    let scale = (1380.0 / (xmax - xmin)).min(470.0 / (ymax - ymin));
    let ox = 110.0 + (1380.0 - (xmax - xmin) * scale) * 0.5 - xmin * scale;
    let oy = 104.0 + (470.0 - (ymax - ymin) * scale) * 0.5 + ymax * scale;

    // Build 2D spatial grid on unposed D1 centroids so 90,000 STL triangles map in O(1)
    let mut grid_buckets: BTreeMap<(i32, i32), Vec<usize>> = BTreeMap::new();
    for (i, c) in centroids.iter().enumerate() {
        let gx = (c[0] * 0.5).floor() as i32;
        let gy = (c[1] * 0.5).floor() as i32;
        grid_buckets.entry((gx, gy)).or_default().push(i);
    }

    // Parse 90,000-triangle STL (supports both ASCII and binary STL) and shade visible upper-surface triangles
    if stl_bytes.len() < 84 {
        return Err("STL too short".into());
    }
    let mut raw_tris: Vec<([f64; 3], [[f64; 3]; 3])> = Vec::with_capacity(90_000);
    if stl_bytes.starts_with(b"solid") {
        let text = std::str::from_utf8(&stl_bytes)?;
        let mut cur_n = [0.0f64; 3];
        let mut cur_v = [[0.0f64; 3]; 3];
        let mut vi = 0usize;
        for line in text.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("facet normal ") {
                let mut it = rest.split_whitespace();
                if let (Some(nx), Some(ny), Some(nz)) = (it.next(), it.next(), it.next()) {
                    cur_n = [
                        nx.parse().unwrap_or(0.0),
                        ny.parse().unwrap_or(0.0),
                        nz.parse().unwrap_or(0.0),
                    ];
                }
                vi = 0;
            } else if let Some(rest) = t.strip_prefix("vertex ") {
                if vi < 3 {
                    let mut it = rest.split_whitespace();
                    if let (Some(vx), Some(vy), Some(vz)) = (it.next(), it.next(), it.next()) {
                        cur_v[vi] = [
                            vx.parse().unwrap_or(0.0),
                            vy.parse().unwrap_or(0.0),
                            vz.parse().unwrap_or(0.0),
                        ];
                        vi += 1;
                        if vi == 3 {
                            raw_tris.push((cur_n, cur_v));
                        }
                    }
                }
            }
        }
    } else {
        let tri_count = u32::from_le_bytes(stl_bytes[80..84].try_into().unwrap()) as usize;
        for idx in 0..tri_count {
            let base = 84 + idx * 50;
            let read_f32 = |off: usize| -> f64 {
                f32::from_le_bytes(stl_bytes[base + off..base + off + 4].try_into().unwrap()) as f64
            };
            raw_tris.push((
                [read_f32(0), read_f32(4), read_f32(8)],
                [
                    [read_f32(12), read_f32(16), read_f32(20)],
                    [read_f32(24), read_f32(28), read_f32(32)],
                    [read_f32(36), read_f32(40), read_f32(44)],
                ],
            ));
        }
    }
    let face_to_cell = raw_tris
        .iter()
        .map(|(_, [v0, v1, v2])| {
            let cx = (v0[0] + v1[0] + v2[0]) / 3.0;
            let cy = (v0[1] + v1[1] + v2[1]) / 3.0;
            hero_nearest_cell(cx, cy, &centroids, &grid_buckets)
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(HeroSource {
        d1_bytes,
        stl_bytes,
        d1,
        composite,
        raw_tris,
        face_to_cell,
        edge_counts,
        posed_nodes,
        ct,
        st,
        scale,
        ox,
        oy,
    })
}

// Components use the exact byte encodings in hero_appearance::RegistrationInput.
// Kept here because that module intentionally exposes no mutable registration.
fn hero_registration_components(input: &crate::hero_appearance::RegistrationInput) -> Value {
    fn string(h: &mut Sha256, value: &str) {
        h.update((value.len() as u64).to_le_bytes());
        h.update(value.as_bytes());
    }
    let (mut scalar, mut mask, mut contributors, mut colors, mut mapping) = (
        Sha256::new(),
        Sha256::new(),
        Sha256::new(),
        Sha256::new(),
        Sha256::new(),
    );
    for cell in &input.cells {
        match cell.index {
            Some(v) => {
                scalar.update([1_u8]);
                scalar.update(v.to_le_bytes());
            }
            None => scalar.update([0_u8]),
        }
        for &v in &cell.source_masks {
            mask.update([u8::from(v)]);
        }
        contributors.update((cell.contributors.len() as u64).to_le_bytes());
        for id in &cell.contributors {
            string(&mut contributors, id);
        }
        colors.update(cell.base_rgb);
    }
    for &i in &input.face_to_cell {
        mapping.update((i as u64).to_le_bytes());
    }
    json!({
        "registered_index_bits":format!("{:x}",scalar.finalize()),
        "original_source_mask_bits":format!("{:x}",mask.finalize()),
        "contributor_ids_and_counts":format!("{:x}",contributors.finalize()),
        "unlit_cell_palette_rgb":format!("{:x}",colors.finalize()),
        "ordered_face_to_original_cell_ids":format!("{:x}",mapping.finalize())
    })
}

fn hero_appearance_camera(source: &HeroSource) -> crate::appearance::Camera {
    // Same R_z(-6deg) * R_x(180deg), D1 bounds, scale and offsets as the flat SVG.
    crate::appearance::Camera {
        rotation: [
            [source.ct, source.st, 0.0],
            [source.st, -source.ct, 0.0],
            [0.0, 0.0, -1.0],
        ],
        scale: source.scale,
        offset: [source.ox, source.oy],
    }
}

fn hero_appearance_binding(
    repo_root: &Path,
    source: &HeroSource,
) -> Result<(crate::hero_appearance::RegistrationInput, Value)> {
    use crate::hero_appearance::{CellColor, RegistrationInput};
    let decoded = crate::appearance::decode_stl(&source.stl_bytes)?;
    let geometry = decoded.geometry_digest();
    // Recompute from actual retained bytes, then check the historical accession.
    // Neither this known geometry pin nor any observed camera/assignment digest
    // substitutes for the separately admitted additive recipe's expected pins.
    if geometry != HERO_ORDERED_GEOMETRY_SHA
        || decoded.len() != 90_000
        || source.composite.values.len() != 25_209
        || decoded.len() != source.raw_tris.len()
        || decoded
            .triangles()
            .iter()
            .zip(&source.raw_tris)
            .any(|(a, (_, b))| {
                a.iter()
                    .flatten()
                    .zip(b.iter().flatten())
                    .any(|(x, y)| x.to_bits() != y.to_bits())
            })
    {
        return Err(
            "population: appearance ordered geometry/count differs from retained predecessor"
                .into(),
        );
    }
    let rows = source.composite.receipt["rows"]
        .as_array()
        .ok_or("missing composite rows")?;
    let members: Vec<_> = rows
        .iter()
        .filter(|r| r["composite_member"] == true)
        .collect();
    let mut channel_ids = Vec::new();
    let mut masks = Vec::new();
    for row in members {
        channel_ids.push(
            row["row_id"]
                .as_str()
                .ok_or("missing channel ID")?
                .to_string(),
        );
        let mut masked = BTreeSet::new();
        for id in row["source_masked_cell_ids"]
            .as_array()
            .ok_or("missing original channel mask")?
        {
            let id = usize::try_from(id.as_u64().ok_or("untyped masked cell")?)?;
            if id >= source.composite.values.len() || !masked.insert(id) {
                return Err("population: invalid original channel mask IDs".into());
            }
        }
        masks.push(masked);
    }
    let projected = source.composite.receipt["cells"]
        .as_array()
        .ok_or("missing composite cells")?;
    if projected.len() != source.composite.values.len() || channel_ids.len() != 4 {
        return Err("population: appearance source channel/cell count differs".into());
    }
    let mut cells = Vec::with_capacity(projected.len());
    let full_range = crate::display::Contrast {
        low: 0.0,
        high: 1.0,
    };
    for (i, row) in projected.iter().enumerate() {
        let index = source.composite.values[i];
        let contributors = row["contributor_test_ids"]
            .as_array()
            .ok_or("missing contributor IDs")?
            .iter()
            .map(|id| {
                id.as_str()
                    .map(str::to_string)
                    .ok_or("untyped contributor ID")
            })
            .collect::<std::result::Result<Vec<_>, _>>()?;
        if row["cell_id"].as_u64() != Some(i as u64)
            || row["value"] != json!(index)
            || contributors.len() != source.composite.counts[i]
            || row["contributor_count"].as_u64() != Some(contributors.len() as u64)
        {
            return Err("population: original cell/contributor projection differs".into());
        }
        cells.push(CellColor {
            index,
            missing_reason: index
                .is_none()
                .then(|| "no_available_positive_area_response".into()),
            source_masks: masks.iter().map(|m| m.contains(&i)).collect(),
            contributors,
            base_rgb: index
                .map(|v| source.composite.palette.rgb(full_range.map(v)))
                .unwrap_or([203, 213, 225]),
        });
    }
    let input = RegistrationInput {
        subject: HERO_SUBJECT.into(),
        d1_sha256: sha256_bytes(&source.d1_bytes),
        channel_ids,
        cells,
        face_to_cell: source.face_to_cell.clone(),
    };
    let mut input_hashes = source.composite.input_hashes.clone();
    // Pin the reporting code as well as its retained input closure. No Git/API,
    // compiler, scorer or renderer is invoked by this observation-only path.
    for path in [
        "crates/paper-artifacts/src/population.rs",
        "crates/paper-artifacts/src/display.rs",
        "crates/paper-artifacts/src/plate.rs",
        "crates/paper-artifacts/src/appearance.rs",
        "crates/paper-artifacts/src/hero_appearance.rs",
        "crates/paper-artifacts/examples/hero_appearance_preflight.rs",
    ] {
        let bytes = crate::corpus::read_bounded_file(
            &crate::corpus::resolve_safe_relpath(repo_root, path)?,
            8 * 1024 * 1024,
        )?;
        input_hashes.insert(path.into(), sha256_bytes(&bytes));
    }
    let observed = json!({
        "schema":"jarambit.hero_appearance_expected_pins.v1",
        "subject":HERO_SUBJECT,
        "source_sha256":input_hashes,
        "ordered_decoded_geometry_sha256":geometry,
        "camera_sha256":hero_appearance_camera(source).digest(),
        "registered_assignment_sha256":input.digest()?,
        "component_sha256":hero_registration_components(&input),
        "source_faces":decoded.len(), "source_cells":input.cells.len(),
        "channel_ids":input.channel_ids,
    });
    Ok((input, observed))
}

/// Observation-only reporting preflight. It reuses the admitted source loading,
/// area-weighted display index, exact original-centroid query/ties and camera.
/// It does NOT derive normals, allocate an image, render, solve or score. The
/// returned digests are observations awaiting independent source comparison;
/// calling this function does not admit its own results as expected pins.
pub fn hero_appearance_preflight(repo_root: &Path) -> Result<Value> {
    let source = load_hero_source(repo_root)?;
    let (_, observed) = hero_appearance_binding(repo_root, &source)?;
    Ok(json!({
        "schema":"jarambit.hero_appearance_preflight.v1",
        "status":"OBSERVED_ONLY_NOT_INDEPENDENT_PIN_ADMISSION",
        "observed_pins":observed,
        "historical_ordered_geometry_checked":HERO_ORDERED_GEOMETRY_SHA,
        "camera":{"rotation":hero_appearance_camera(&source).rotation,
            "scale":source.scale,"offset":[source.ox,source.oy],"refit":false},
        "registration":"same predecessor unposed STL XY centroid -> exact nearest original D1 triangle centroid; exact ties lowest source cell ID; all original STL faces",
        "image_rendered":false,"normals_derived":false,"fresh_science_or_fitness":false,
        "required_next_step":"Captain captures this actual preflight; independent Q compares original source, complete registration and camera before expected pins enter the additive recipe."
    }))
}

fn hero_sha_pin(value: &Value) -> bool {
    value.as_str().is_some_and(|s| {
        s.len() == 64
            && s.bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    })
}
fn hero_require_expected_pins(expected: &Value, observed: &Value) -> Result<()> {
    // Missing/null pins are not defaults. Compare the complete source closure,
    // ordered geometry, camera, assignment, scalar/mask/contributor/RGB/map bits.
    let sources = expected["source_sha256"].as_object();
    let components = expected["component_sha256"].as_object();
    if !hero_sha_pin(&expected["ordered_decoded_geometry_sha256"])
        || !hero_sha_pin(&expected["camera_sha256"])
        || !hero_sha_pin(&expected["registered_assignment_sha256"])
        || !sources.is_some_and(|s| !s.is_empty() && s.values().all(hero_sha_pin))
        || !components.is_some_and(|s| s.len() == 5 && s.values().all(hero_sha_pin))
        || expected != observed
    {
        return Err(
            "population: missing/null/unadmitted or mismatching appearance expected pins".into(),
        );
    }
    Ok(())
}

// Independent RGB8 PNG path; the existing indexed point encoder stays untouched.
// Stored DEFLATE blocks and Adler32/PNG CRC follow the same existing pattern.
fn encode_hero_rgb_png(w: usize, h: usize, rgb: &[u8]) -> Result<Vec<u8>> {
    let pixels = w.checked_mul(h).ok_or("RGB PNG size overflow")?;
    if w == 0
        || h == 0
        || w > 4096
        || h > 4096
        || rgb.len() != pixels.checked_mul(3).ok_or("RGB PNG size overflow")?
    {
        return Err("population: invalid RGB PNG shape".into());
    }
    let row_bytes = w * 3;
    let mut raw = Vec::with_capacity(h * (row_bytes + 1));
    for row in rgb.chunks_exact(row_bytes) {
        raw.push(0);
        raw.extend_from_slice(row);
    }
    let mut zlib = vec![0x78, 0x01];
    let chunks: Vec<_> = raw.chunks(65_535).collect();
    for (i, chunk) in chunks.iter().enumerate() {
        zlib.push(if i + 1 == chunks.len() { 1 } else { 0 });
        let len = chunk.len() as u16;
        zlib.extend_from_slice(&len.to_le_bytes());
        zlib.extend_from_slice(&(!len).to_le_bytes());
        zlib.extend_from_slice(chunk);
    }
    let (mut s1, mut s2) = (1u32, 0u32);
    for &b in &raw {
        s1 = (s1 + b as u32) % 65_521;
        s2 = (s2 + s1) % 65_521;
    }
    zlib.extend_from_slice(&((s2 << 16) | s1).to_be_bytes());
    let mut png = vec![137, 80, 78, 71, 13, 10, 26, 10];
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&(w as u32).to_be_bytes());
    ihdr.extend_from_slice(&(h as u32).to_be_bytes());
    ihdr.extend_from_slice(&[8, 2, 0, 0, 0]); // RGB8, no indexed/binary String conversion.
    png_chunk(b"IHDR", &ihdr, &mut png);
    png_chunk(b"IDAT", &zlib, &mut png);
    png_chunk(b"IEND", &[], &mut png);
    Ok(png)
}
fn hero_base64(bytes: &[u8]) -> String {
    const B64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for c in bytes.chunks(3) {
        let n = ((c[0] as u32) << 16)
            | ((c.get(1).copied().unwrap_or(0) as u32) << 8)
            | c.get(2).copied().unwrap_or(0) as u32;
        out.push(B64[((n >> 18) & 63) as usize] as char);
        out.push(B64[((n >> 12) & 63) as usize] as char);
        out.push(if c.len() > 1 {
            B64[((n >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if c.len() > 2 {
            B64[(n & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}
fn hero_clean_svg(rgb: &[u8]) -> Result<(String, String)> {
    let png = encode_hero_rgb_png(1600, 680, rgb)?;
    let svg = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 1600 680\" width=\"1600\" height=\"680\"><rect width=\"1600\" height=\"680\" fill=\"#ffffff\"/><image width=\"1600\" height=\"680\" href=\"data:image/png;base64,{}\"/></svg>\n",hero_base64(&png));
    Ok((svg, sha256_bytes(&png)))
}

/// Preserve the lighting API's bounded integer exponent; never truncate a recipe value.
fn hero_shininess(value: &Value) -> Result<i32> {
    let n = finite_point_number(value)?;
    if !(1. ..=128.).contains(&n) || n.fract() != 0. {
        return Err("population: lighting shininess must be an integer in1..128".into());
    }
    Ok(n as i32)
}

/// Render only from a separately SHA-bound additive source recipe. Its expected
/// pins MUST come from Captain's actual preflight + independent source review.
/// No observed digest is installed as an expected pin inside this render path.
/// Returns text artifacts; caller owns publication wiring and writing outputs.
pub fn generate_hero_appearance_outputs(
    repo_root: &Path,
    recipe_rel: &str,
    expected_recipe_sha256: &str,
) -> Result<BTreeMap<String, String>> {
    if !hero_sha_pin(&json!(expected_recipe_sha256)) {
        return Err("population: absent appearance recipe accession".into());
    }
    let bytes = crate::corpus::read_bounded_file(
        &crate::corpus::resolve_safe_relpath(repo_root, recipe_rel)?,
        128 * 1024,
    )?;
    if sha256_bytes(&bytes) != expected_recipe_sha256 {
        return Err("population: appearance recipe bytes differ".into());
    }
    let recipe: Value = serde_json::from_slice(&bytes)?;
    if recipe["schema"] != "jarambit.hero_appearance_source_recipe.v1"
        || recipe["pin_admission"]["status"] != "independently_source_reviewed"
        || !hero_sha_pin(&recipe["pin_admission"]["actual_preflight_receipt_sha256"])
        || !hero_sha_pin(&recipe["pin_admission"]["independent_source_review_receipt_sha256"])
        || recipe["render"]
            != json!({"width":1600,"height":680,"supersample":2,"outline_pixels":2.0})
        || recipe["regions"] != Value::Null
        || recipe["quantitative_companion"] != "docs/paper/generated/hero-full-range.svg"
    {
        return Err("population: appearance pins lack explicit external admission".into());
    }
    let source = load_hero_source(repo_root)?;
    let (input, observed) = hero_appearance_binding(repo_root, &source)?;
    hero_require_expected_pins(&recipe["expected_pins"], &observed)?;
    let scalar = |v: &Value| -> Result<f64> {
        v.as_f64()
            .filter(|v| v.is_finite())
            .ok_or_else(|| "missing/nonfinite appearance setting".into())
    };
    let normals = crate::appearance::NormalSettings {
        crease_degrees: scalar(&recipe["normals"]["crease_degrees"])?,
        blend: scalar(&recipe["normals"]["blend"])?,
        max_deviation_degrees: scalar(&recipe["normals"]["max_deviation_degrees"])?,
    };
    let light = &recipe["lighting"];
    let direction = light["direction_camera"]
        .as_array()
        .ok_or("missing light direction")?;
    if direction.len() != 3 {
        return Err("invalid light direction shape".into());
    }
    let camera = hero_appearance_camera(&source);
    let surface = crate::hero_appearance::RetainedSurface::from_stl(
        HERO_SUBJECT,
        &source.stl_bytes,
        recipe["expected_pins"]["source_sha256"][HERO_STL_REL]
            .as_str()
            .ok_or("missing STL pin")?,
        recipe["expected_pins"]["ordered_decoded_geometry_sha256"]
            .as_str()
            .ok_or("missing geometry pin")?,
    )?;
    let registered = crate::hero_appearance::RegisteredCellColors::bind(
        input,
        &source.d1_bytes,
        recipe["expected_pins"]["registered_assignment_sha256"]
            .as_str()
            .ok_or("missing assignment pin")?,
    )?;
    let output = crate::hero_appearance::render(
        &surface,
        &registered,
        &crate::hero_appearance::AppearanceRecipe {
            normals,
            regions: None,
            render: crate::appearance::RenderSettings {
                width: 1600,
                height: 680,
                supersample: 2,
                outline_pixels: 2.0,
                camera,
                lighting: crate::appearance::Lighting {
                    direction: [
                        scalar(&direction[0])?,
                        scalar(&direction[1])?,
                        scalar(&direction[2])?,
                    ],
                    ambient: scalar(&light["ambient"])?,
                    diffuse: scalar(&light["diffuse"])?,
                    specular: scalar(&light["specular"])?,
                    shininess: hero_shininess(&light["shininess"])?,
                },
            },
            expected_camera_sha256: recipe["expected_pins"]["camera_sha256"]
                .as_str()
                .ok_or("missing camera pin")?
                .into(),
        },
    )?;
    if output.receipt["registered_assignment"]["component_hashes_before"]
        != observed["component_sha256"]
    {
        return Err("population: appearance adapter component encoding differs".into());
    }
    let (lit, lit_png_sha) = hero_clean_svg(&output.image.rgb)?;
    let (unlit, unlit_png_sha) = hero_clean_svg(&output.image.unlit_rgb)?;
    let mut receipt = output.receipt;
    receipt["caller"] = json!({
        "source_recipe":{"path":recipe_rel,"sha256":expected_recipe_sha256},
        "pin_admission":recipe["pin_admission"],"checked_expected_pins":recipe["expected_pins"],
        "required_visible_caption_disclosure":output.required_visible_disclosure,
        "png":{"illuminated_sha256":lit_png_sha,"matched_unlit_sha256":unlit_png_sha,"color_type":"RGB8; filter0; stored DEFLATE"},
        "svg":{"illuminated_sha256":sha256_bytes(lit.as_bytes()),"matched_unlit_sha256":sha256_bytes(unlit.as_bytes()),"visible_exterior_labels_or_effects":false},
        "unchanged_quantitative_companion":"docs/paper/generated/hero-full-range.svg",
        "matched_unlit_is_additional_visibility_control":true,
    });
    Ok(BTreeMap::from([
        ("docs/paper/generated/hero-appearance.svg".into(), lit),
        (
            "docs/paper/generated/hero-appearance-unlit.svg".into(),
            unlit,
        ),
        (
            "docs/paper/generated/hero-appearance.v1.json".into(),
            serde_json::to_string_pretty(&receipt)? + "\n",
        ),
    ]))
}

fn render_hero_view(
    repo_root: &Path,
    view: HeroView,
) -> Result<(String, crate::display::Composite)> {
    use std::fmt::Write as _;
    let HeroSource {
        d1, composite, raw_tris, face_to_cell, edge_counts, posed_nodes,
        ct, st, scale, ox, oy, ..
    } = load_hero_source(repo_root)?;
    let contrast = if view == HeroView::ContrastPreview {
        composite.preview
    } else {
        crate::display::Contrast {
            low: 0.0,
            high: 1.0,
        }
    };

    let pose_xy = |x: f64, y: f64| -> [f64; 2] {
        let yr = -y;
        [x * ct - yr * st, x * st + yr * ct]
    };
    let to_svg = |px: f64, py: f64| -> (f64, f64) { (ox + px * scale, oy - py * scale) };
    let mut stl_vis: Vec<(f64, [(f64, f64); 3], [u8; 3])> =
        Vec::with_capacity(raw_tris.len() / 2 + 1024);
    for (face_id, (n_vec, [v0, v1, v2])) in raw_tris.into_iter().enumerate() {
        let nz = n_vec[2];
        // Under R = R_z(-6 deg) * R_x(180 deg), posed z is -z
        let nzp = -nz;
        if nzp < -0.05 {
            continue;
        }
        let czp = -(v0[2] + v1[2] + v2[2]) / 3.0;
        let best_i = face_to_cell[face_id];
        let value = if view == HeroView::Support {
            (composite.counts[best_i] > 0).then_some(composite.counts[best_i] as f64 / 4.0)
        } else {
            composite.values[best_i].map(|v| contrast.map(v))
        };
        let rgb = value
            .map(|v| composite.palette.rgb(v))
            .unwrap_or([203, 213, 225]);
        let p0 = pose_xy(v0[0], v0[1]);
        let p1 = pose_xy(v1[0], v1[1]);
        let p2 = pose_xy(v2[0], v2[1]);
        stl_vis.push((
            czp,
            [
                to_svg(p0[0], p0[1]),
                to_svg(p1[0], p1[1]),
                to_svg(p2[0], p2[1]),
            ],
            rgb,
        ));
    }
    stl_vis.sort_by(|a, b| a.0.total_cmp(&b.0));

    let mut s = String::with_capacity(6_400_000);
    s.push_str(r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1600 680" width="1600" height="680" font-family="Inter, -apple-system, BlinkMacSystemFont, sans-serif">"##);
    s.push('\n');
    let color_metadata = json!({
        "palette": composite.palette.name,
        "contract": crate::display::CONTRACT,
        "projection_receipt": "docs/paper/generated/display-composite.v1.json",
        "surface_and_legend_transfer": "display::Palette::rgb",
        "surface_lighting": "none",
        "scalar_domain": [0.0, 1.0],
        "scalar_transform": "mean_available_area_weighted_pooled_tie_midranks_no_postmean_minmax",
        "contrast_window": [contrast.low, contrast.high],
        "view": match view { HeroView::FullRange => "full_range", HeroView::ContrastPreview => "additional_Q05_Q95_preview", HeroView::MeshOn => "full_range_exact_mesh_on", HeroView::Support => "available_response_count" },
        "meaning": "relative_dimensionless_display_index_not_stress_fitness_independent_tests_or_probability"
    });
    let _ = writeln!(
        s,
        "<metadata id=\"hero-color-transfer\">{}</metadata>",
        xml_escape(&color_metadata.to_string())
    );
    s.push_str(r##"<rect width="1600" height="680" fill="#ffffff"/>"##);
    let title = match view {
        HeroView::Support => "Individual 2000 — Available Response Support",
        HeroView::ContrastPreview => "Individual 2000 — Display Contrast Preview (Q05–Q95)",
        HeroView::MeshOn => "Individual 2000 — CAD Tessellation Overlay",
        HeroView::FullRange => "Individual 2000 — Available-Response Display Index",
    };
    s.push_str(&format!(
        r##"<text x="36" y="44" font-size="34" font-weight="700" fill="#0f172a">{title}</text>"##
    ));
    let subtitle = if view == HeroView::MeshOn {
        "CAD STL: 90,000 triangles. Colors: retained plate-response display index."
    } else {
        "Four retained response channels; scalar predicates and unavailable fields add no spatial weight."
    };
    s.push_str(&format!(
        r##"<text x="36" y="80" font-size="28" fill="#334155">{subtitle}</text>"##
    ));

    // Draw projected STL triangles with direct quantitative palette colors.
    for (_, [s0, s1, s2], [r, g, b]) in &stl_vis {
        let _ = write!(
            s,
            "<polygon points=\"{:.2},{:.2} {:.2},{:.2} {:.2},{:.2}\" fill=\"#{r:02x}{g:02x}{b:02x}\" stroke=\"#{r:02x}{g:02x}{b:02x}\" stroke-width=\"0.22\"/>\n",
            s0.0, s0.1, s1.0, s1.1, s2.0, s2.1
        );
    }

    if view == HeroView::MeshOn {
        s.push_str(
            r##"<g fill="none" stroke="#0f172a" stroke-width="0.12" stroke-opacity="0.03">"##,
        );
        for (_, [a, b, c], _) in &stl_vis {
            let _ = write!(
                s,
                "<path d=\"M{:.2},{:.2} L{:.2},{:.2} L{:.2},{:.2} Z\"/>",
                a.0, a.1, b.0, b.1, c.0, c.1
            );
        }
        s.push_str("</g>");
    }

    // Crisp dark silhouette and finger-ring / handle-slot hole contours from D1 boundary edges
    s.push_str(r##"<g stroke="#0f172a" stroke-width="1.8" stroke-linecap="round">"##);
    for (&(u, v), &cnt) in &edge_counts {
        if cnt == 1 {
            let (x1, y1) = to_svg(posed_nodes[u][0], posed_nodes[u][1]);
            let (x2, y2) = to_svg(posed_nodes[v][0], posed_nodes[v][1]);
            let _ = write!(
                s,
                "<line x1=\"{x1:.2}\" y1=\"{y1:.2}\" x2=\"{x2:.2}\" y2=\"{y2:.2}\"/>"
            );
        }
    }
    s.push_str("</g>\n");

    // 4.0 mm tip-exclusion dashed circle
    if let Some(tip_arr) = d1["export_surface"]["tip"].as_array() {
        let tp = pose_xy(
            tip_arr[0].as_f64().unwrap_or(0.0),
            tip_arr[1].as_f64().unwrap_or(0.0),
        );
        let (tx, ty) = to_svg(tp[0], tp[1]);
        let r_px = 4.0 * scale;
        let _ = writeln!(
            s,
            r##"<circle cx="{tx:.2}" cy="{ty:.2}" r="{r_px:.2}" fill="none" stroke="#b91c1c" stroke-width="2.0" stroke-dasharray="5,4"/><text x="{:.2}" y="{:.2}" text-anchor="end" font-size="28" font-weight="600" fill="#991b1b">4 mm capacity exclusion</text>"##,
            tx - r_px - 10.0,
            ty + 6.0
        );
    }

    // High-contrast sequential colorbar + readable dark legend & scale bar
    let legend_title = if view == HeroView::Support {
        "Available response channels [0, 4]"
    } else {
        "Available-response display index"
    };
    s.push_str(&format!(r##"<text x="36" y="596" font-size="28" font-weight="700" fill="#0f172a">{legend_title}</text>"##));
    let cb_x0 = 36.0_f64;
    let cb_w = 640.0_f64;
    let n_steps = if view == HeroView::Support {
        5usize
    } else {
        80usize
    };
    for i in 0..n_steps {
        let t = if view == HeroView::Support {
            i as f64 / 4.0
        } else {
            (i as f64 + 0.5) / n_steps as f64
        };
        let [r, g, b] = if view == HeroView::Support && i == 0 {
            [203, 213, 225]
        } else {
            composite.palette.rgb(t)
        };
        let x = cb_x0 + (i as f64) * (cb_w / n_steps as f64);
        let w = (cb_w / n_steps as f64) + 0.8;
        let _ = write!(
            s,
            "<rect x=\"{x:.2}\" y=\"606\" width=\"{w:.2}\" height=\"18\" fill=\"#{r:02x}{g:02x}{b:02x}\"/>"
        );
    }
    s.push_str(r##"<rect x="36" y="606" width="640" height="18" fill="none" stroke="#0f172a" stroke-width="1.4"/>"##);
    for i in 0..5 {
        let frac = i as f64 / 4.0;
        let position = if view == HeroView::Support {
            (i as f64 + 0.5) / 5.0
        } else {
            frac
        };
        let label = if view == HeroView::Support {
            i.to_string()
        } else {
            let value = contrast.low + frac * (contrast.high - contrast.low);
            if view == HeroView::ContrastPreview && (i == 0 || i == 4) {
                format!("{value:.4}")
            } else {
                format!("{value:.3}")
            }
        };
        let anchor = if view == HeroView::Support {
            "middle"
        } else if i == 0 {
            "start"
        } else if i == 4 {
            "end"
        } else {
            "middle"
        };
        let tick_x = cb_x0 + position * cb_w;
        let _ = write!(s, "<line x1=\"{tick_x:.2}\" y1=\"624\" x2=\"{tick_x:.2}\" y2=\"630\" stroke=\"#0f172a\" stroke-width=\"1.4\"/><text x=\"{tick_x:.2}\" y=\"650\" text-anchor=\"{anchor}\" font-size=\"28\" font-weight=\"600\" fill=\"#0f172a\">{label}</text>");
    }
    let bar_25mm = 25.0 * scale;
    let bx1 = 1554.0_f64;
    let bx0 = bx1 - bar_25mm;
    let _ = writeln!(
        s,
        r##"<text x="{:.2}" y="596" text-anchor="middle" font-size="28" font-weight="700" fill="#0f172a">25 mm</text><line x1="{bx0:.2}" y1="615" x2="{bx1:.2}" y2="615" stroke="#0f172a" stroke-width="3.0"/><line x1="{bx0:.2}" y1="607" x2="{bx0:.2}" y2="623" stroke="#0f172a" stroke-width="2.4"/><line x1="{bx1:.2}" y1="607" x2="{bx1:.2}" y2="623" stroke="#0f172a" stroke-width="2.4"/>"##,
        0.5 * (bx0 + bx1)
    );
    let context = if view == HeroView::Support {
        let q = &composite.receipt["support_area_fractions"];
        format!(
            "Source area: 3 maps {:.1}%; 4 maps {:.1}%",
            q[3].as_f64().unwrap() * 100.0,
            q[4].as_f64().unwrap() * 100.0
        )
    } else if view == HeroView::ContrastPreview {
        let tails = &composite.receipt["Q05_Q95_preview"];
        let counts = &tails["counts_below_inside_above"];
        let areas = &tails["area_mm2_below_inside_above"];
        let whole = tails["whole_source_area_mm2"].as_f64().unwrap();
        format!(
            "Below: {} ({:.1}%); above: {} ({:.1}%)",
            counts[0].as_u64().unwrap(),
            areas[0].as_f64().unwrap() / whole * 100.0,
            counts[2].as_u64().unwrap(),
            areas[2].as_f64().unwrap() / whole * 100.0
        )
    } else {
        "44 catalog rows + 8 channel metadata rows".to_string()
    };
    s.push_str(&format!(
        r##"<text x="720" y="596" font-size="28" fill="#334155">{}</text>"##,
        xml_escape(&context)
    ));
    if view == HeroView::ContrastPreview {
        s.push_str(r##"<text x="720" y="650" font-size="28" fill="#334155">Full-range companion retained</text>"##);
    }
    s.push_str("</svg>\n");
    Ok((s, composite))
}

fn append_positive_point_views(
    native: &[(usize, u64, u64, f64)],
    historical: &[(u64, u64, f64)],
    screening: &[(u64, u64, f64)],
    point: &mut Value,
    outputs: &mut BTreeMap<String, String>,
) -> Result<()> {
    use crate::population_story::Preference;
    use crate::positive_points::{Abscissa, PositivePointRaster};
    if native.len() != 1_954_560
        || historical.len() != 1984
        || screening.len() != 2259
        || point["unknown"]["records"] != 1390u64
        || point["numeric_point_operations"] != 1_958_803u64
    {
        return Err("positive points: unadmitted full-source census".into());
    }
    let contexts = point["run_context"]
        .as_array()
        .ok_or("positive points: missing original contexts")?;
    let runs = point["native"]["runs"]
        .as_array()
        .ok_or("positive points: missing original identity order")?;
    if contexts.len() != 10 || runs.len() != 10 {
        return Err("positive points: exact ten-run scope differs".into());
    }
    let mut index_rows = Vec::new();
    let mut totals = [0_u64; 2];
    for (variant, gap) in [("affine", false), ("gap", true)] {
        let mut scope_outputs = Vec::new();
        for context in contexts {
            let ordinal = context["archive_run_ordinal"]
                .as_u64()
                .ok_or("positive points: missing run ordinal")? as usize;
            let public = context["public_run_id"]
                .as_u64()
                .ok_or("positive points: missing public run")?;
            let bounds = context["raster"]["observed_score_extrema"]
                .as_array()
                .ok_or("positive points: missing complete extrema")?;
            if bounds.len() != 2 || !(2..=11).contains(&public) || ordinal > 9 {
                return Err("positive points: invalid compatible run scope".into());
            }
            let domain = [
                finite_point_number(&bounds[0])?,
                finite_point_number(&bounds[1])?,
            ];
            let mut raster = PositivePointRaster::new(
                domain,
                Preference::Higher,
                gap,
                Abscissa::Generation { last: 800 },
                195456,
            )?;
            for &(run, id, generation, f) in native {
                if run == ordinal {
                    raster.paint(&[id], generation, f)?;
                }
            }
            let matches: Vec<_> = runs
                .iter()
                .filter(|row| {
                    row["archive_run_ordinal"] == ordinal as u64
                        && row["package_id"] == context["package_id"]
                })
                .collect();
            if matches.len() != 1
                || matches[0]["original_id_order_sha256"].as_str()
                    != Some(raster.identity_order_sha256().as_str())
            {
                return Err("positive points: original callback identity order differs".into());
            }
            let prefix=format!("docs/paper/generated/population/positive/run-{public:02}-positive-points-{variant}");
            let png = raster.score_png_base64()?;
            let title =
                format!("Run {public}: every retained birth, fixed positive {variant} view");
            let (svg, mut receipt) = raster.render(&png, &title)?;
            let leader = point["retained_leaders"]["native_by_run"]
                .as_array()
                .ok_or("positive points: missing leaders")?
                .iter()
                .find(|r| r["archive_run_ordinal"] == ordinal as u64)
                .ok_or("positive points: missing compatible leader")?;
            if receipt["leader_ties"] != leader["tie_count"]
                || domain[1] != finite_point_number(&leader["value"])?
            {
                return Err("positive points: original leader/tie census differs".into());
            }
            receipt["source"] = json!({"original_archive":context["source"],"package_id":context["package_id"],"archive_run_ordinal":ordinal,"public_run_id":public,
                "admitted_original_identity_order_sha256":matches[0]["original_id_order_sha256"],"leader_ledger":leader,
                "stage":"original retained native birth observations; not post-rescore membership or physical improvement"});
            receipt["summary_companion"] = json!({"path":format!("docs/paper/generated/population/story/run-{public:02}-positive-{variant}.v1.json"),
                "same_complete_original_domain_and_positive_axis":true,"summary_y_viewport_may_differ":true});
            scope_outputs.push((prefix, svg, receipt));
        }
        for historical_scope in [true, false] {
            let values: Vec<f64> = if historical_scope {
                historical.iter().map(|p| p.2).collect()
            } else {
                screening.iter().map(|p| p.2).collect()
            };
            let domain = point_bounds(values.into_iter())?;
            let (name, preference, x_axis, count) = if historical_scope {
                let last = historical
                    .iter()
                    .map(|p| p.1)
                    .max()
                    .ok_or("positive points: empty historical scope")?;
                (
                    "historical",
                    Preference::Higher,
                    Abscissa::Generation { last },
                    1984,
                )
            } else {
                (
                    "screening",
                    Preference::Lower,
                    Abscissa::FileOrdinal { last: 57 },
                    2259,
                )
            };
            let mut raster = PositivePointRaster::new(domain, preference, gap, x_axis, count)?;
            if historical_scope {
                for &(id, generation, f) in historical {
                    raster.paint(&[id], generation, f)?;
                }
            } else {
                for &(file, candidate, f) in screening {
                    raster.paint(&[file, candidate], file + 1, f)?;
                }
            }
            let prefix = format!(
                "docs/paper/generated/population/positive/{name}-positive-points-{variant}"
            );
            let png = raster.score_png_base64()?;
            let title =
                format!("{name}: every retained numeric row, fixed positive {variant} view");
            let (svg, mut receipt) = raster.render(&png, &title)?;
            receipt["source"] = if historical_scope {
                json!({"path":HISTORICAL_ALL_EVALS_JSON_RELPATH,"sha256":HISTORICAL_ALL_EVALS_JSON_SHA256,
                "identity":"original historical id; original generation retained","leader_ledger":point["retained_leaders"]["historical"]})
            } else {
                json!({"source_pairs":point["screening"]["source_pairs"],"identity":"original zero-based file index plus one-based candidate; x is unchanged one-based file ordinal",
                    "unknown":point["unknown"],"leader_ledger":point["retained_leaders"]["screening"]})
            };
            let leader = &point["retained_leaders"][if historical_scope {
                "historical"
            } else {
                "screening"
            }];
            if receipt["leader_ties"].as_u64()
                != Some(
                    leader["ties"]
                        .as_array()
                        .ok_or("positive points: missing original ties")?
                        .len() as u64,
                )
            {
                return Err("positive points: historical/screening original ties differ".into());
            }
            scope_outputs.push((prefix, svg, receipt));
        }
        if scope_outputs.len() != 12 {
            return Err("positive points: incomplete compatible scope list".into());
        }
        let mut scope_names = BTreeSet::new();
        let mut total = 0_u64;
        for (prefix, svg, receipt) in scope_outputs {
            if !scope_names.insert(prefix.clone()) {
                return Err("positive points: duplicate compatible scope".into());
            }
            total += receipt["numeric_point_operations"]
                .as_u64()
                .ok_or("positive points: missing operation count")?;
            let svg_path = format!("{prefix}.svg");
            let receipt_path = format!("{prefix}.v1.json");
            let receipt_json = serde_json::to_string_pretty(&receipt)? + "\n";
            index_rows.push(json!({"variant":variant,"svg":svg_path,"svg_sha256":sha256_bytes(svg.as_bytes()),"receipt":receipt_path,
                "receipt_sha256":sha256_bytes(receipt_json.as_bytes()),"numeric_point_operations":receipt["numeric_point_operations"]}));
            if outputs.insert(svg_path, svg).is_some()
                || outputs.insert(receipt_path, receipt_json).is_some()
            {
                return Err("positive points: duplicate output path".into());
            }
        }
        if total != 1_958_803 {
            return Err("positive points: full all-record operation census differs".into());
        }
        totals[usize::from(gap)] = total;
    }
    let index = json!({"schema":"jarambit.positive_all_record_index.v1","views":index_rows,"operations_by_variant":{"positive_affine":totals[0],"positive_gap":totals[1]},
        "unknown":point["unknown"],"distinct_admitted_numeric_observations":1_958_803,"additional_views_are_not_new_data":true,
        "full_range_asinh_companion":"docs/paper/figures/population-latest.svg","quantile_clipping":false,"density_or_subsampling":false,
        "source_profiles":"ten separate native runs, historical births, lower-preferred screening deficits; no cross-run score calibration or global winner",
        "main_candidate":"run-05-positive-points-gap.png; pending actual final-width/readability/finding review"});
    outputs.insert(
        "docs/paper/generated/population/positive/index.v1.json".into(),
        serde_json::to_string_pretty(&index)? + "\n",
    );
    point["positive_all_record_views"] = json!({"index":"docs/paper/generated/population/positive/index.v1.json","operations_by_variant":index["operations_by_variant"],
        "unknown_nonnumeric":1390,"additional_views_are_not_new_data":true});
    Ok(())
}

/// Additive, source-joined summaries; this never replaces the all-record point layer.
fn append_native_story_views(
    density: &Value,
    point: &Value,
    contract: &Value,
    inputs: &mut BTreeMap<String, String>,
    outputs: &mut BTreeMap<String, String>,
) -> Result<()> {
    use crate::population_story::{
        self, BoundLeader, BoundRunTrace, GenerationView, LeaderTie, Preference, SourceBinding,
        TraceSample, View,
    };
    const SOURCE: &str = "docs/paper/data/gen800_population_density_and_traces.v1.json";
    const CONTRACT: &str = "docs/paper/data/population-story-contract.v1.json";
    if contract["schema"] != "jarambit.population_story.display_contract.v1"
        || contract["required_all_record_numeric_operations"] != 1_958_803u64
        || contract["required_unknown_nonnumeric"] != 1390u64
        || contract["native_source"] != SOURCE
        || contract["native_run_public_ids"] != json!([2, 3, 4, 5, 6, 7, 8, 9, 10, 11])
        || contract["views"] != json!(["asinh", "positive-affine", "positive-gap"])
        || contract["positive_display_floor"].as_f64() != Some(1e-6)
    {
        return Err("population story: unadmitted display contract revision".into());
    }
    let epsilon = finite_point_number(&contract["positive_display_floor"])?;
    // These are the actual archive admission/point-accounting boundary above, not summary guesses.
    if point["numeric_point_operations"] != 1_958_803u64
        || point["excluded_numeric_records"] != 0u64
        || point["unknown"]["records"] != 1390u64
        || point["native"]["admission"]["rows_decoded"] != 1_954_560u64
    {
        return Err("population story: original all-record coverage/admission changed".into());
    }
    let runs = density["runs"]
        .as_array()
        .ok_or("population story: missing source runs")?;
    let contexts = point["run_context"]
        .as_array()
        .ok_or("population story: missing admitted run context")?;
    let ids = point["native"]["runs"]
        .as_array()
        .ok_or("population story: missing original identity order")?;
    let leaders = point["retained_leaders"]["native_by_run"]
        .as_array()
        .ok_or("population story: missing bound leaders")?;
    if [runs.len(), contexts.len(), ids.len(), leaders.len()]
        .iter()
        .any(|n| *n != 10)
    {
        return Err("population story: exact ten-run scope differs".into());
    }
    let leader_json = serde_json::to_string_pretty(&point["retained_leaders"])? + "\n";
    let leader_sha = sha256_bytes(leader_json.as_bytes());
    let source_sha = inputs
        .get(SOURCE)
        .ok_or("population story: source not in input closure")?
        .clone();
    let mut index_rows = Vec::new();
    for (i, row) in runs.iter().enumerate() {
        let public = row["public_run_id"]
            .as_u64()
            .ok_or("population story: missing public run")?;
        let package = row["package_id"]
            .as_str()
            .ok_or("population story: missing source package")?;
        if public != i as u64 + 2
            || row["generation_range"] != json!([0, 800])
            || row["distinct_generations_count"] != 801u64
        {
            return Err("population story: public run/dense source addressing differs".into());
        }
        let matches = |v: &&Value| v["package_id"].as_str() == Some(package);
        let context_matches: Vec<_> = contexts.iter().filter(matches).collect();
        let id_matches: Vec<_> = ids.iter().filter(matches).collect();
        let leader_matches: Vec<_> = leaders.iter().filter(matches).collect();
        if context_matches.len() != 1 || id_matches.len() != 1 || leader_matches.len() != 1 {
            return Err("population story: missing/ambiguous compatible source join".into());
        }
        let (context, id, leader) = (context_matches[0], id_matches[0], leader_matches[0]);
        if context["public_run_id"] != public
            || context["archive_run_ordinal"] != i as u64
            || id["archive_run_ordinal"] != i as u64
            || leader["archive_run_ordinal"] != i as u64
            || row["original_csv_sha256"] != context["source"]["original_csv_sha256"]
            || row["manifest_sha256"] != context["source"]["manifest_sha256"]
            || row["row_count"] != id["records_visited"]
        {
            return Err("population story: archive/identity/summary joins differ".into());
        }
        let domain = [
            finite_point_number(&row["overall_min_fit"])?,
            finite_point_number(&row["overall_max_fit"])?,
        ];
        if context["raster"]["observed_score_extrema"] != json!(domain) {
            return Err("population story: summary extrema differ from decoded records".into());
        }
        let best = row["gen_cum_best_fit"]
            .as_array()
            .ok_or("population story: missing best source array")?;
        let median = row["gen_median_fit"]
            .as_array()
            .ok_or("population story: missing full-population median")?;
        let verified = context["retained_best_birth_trace"]
            .as_array()
            .ok_or("population story: missing decoded best trace")?;
        if best.len() != 801 || median.len() != 801 || verified.len() != 801 || best != verified {
            return Err("population story: preserved trace differs from decoded archive".into());
        }
        let mut samples = Vec::with_capacity(best.len());
        for (source_index, (b, m)) in best.iter().zip(median.iter()).enumerate() {
            let optional = |value: &Value| -> Result<Option<f64>> {
                if value.is_null() {
                    Ok(None)
                } else {
                    Ok(Some(finite_point_number(value)?))
                }
            };
            samples.push(TraceSample {
                generation: source_index as u64,
                cumulative_best: optional(b)?,
                population_median: optional(m)?,
                best_source_pointer: Some(format!("/runs/{i}/gen_cum_best_fit/{source_index}")),
                median_source_pointer: Some(format!("/runs/{i}/gen_median_fit/{source_index}")),
            });
        }
        let ties = leader["ties"]
            .as_array()
            .ok_or("population story: missing admitted ties")?
            .iter()
            .map(|tie| {
                Ok(LeaderTie {
                    public_individual: tie["public_individual"]
                        .as_u64()
                        .ok_or("population story: missing public individual")?,
                    original_id: tie["original_id"]
                        .as_u64()
                        .ok_or("population story: missing original ID")?
                        .to_string(),
                    generation: tie["generation"]
                        .as_u64()
                        .ok_or("population story: missing tie generation")?,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        let trace = BoundRunTrace {
            label: format!("Run {public}"),
            metric: "contextual search fitness f".into(),
            preference: Preference::Higher,
            original_domain: domain,
            source: SourceBinding {
                path: SOURCE.into(),
                sha256: source_sha.clone(),
                scope_pointer: format!("/runs/{i}"),
                compatible_run: format!("Run {public}: {package}"),
                stage: "original retained native birth observations".into(),
                membership_identity_order_sha256: id["original_id_order_sha256"]
                    .as_str()
                    .ok_or("population story: missing identity-order digest")?
                    .into(),
            },
            leader: BoundLeader {
                original_value: finite_point_number(&leader["value"])?,
                original_token: leader["value"].to_string(),
                ledger_sha256: leader_sha.clone(),
                ties,
                representative_tie_index: 0,
            },
            samples,
        };
        let l = finite_point_number(&context["axis"]["linear_threshold_L"])?;
        let mut views = Vec::new();
        for (name, view, generation) in [
            (
                "asinh",
                View::Asinh {
                    linear_threshold: l,
                },
                GenerationView::Linear,
            ),
            (
                "positive-affine",
                View::PositiveAffineLog10 { epsilon },
                GenerationView::Log10GenerationPlusOne,
            ),
            (
                "positive-gap",
                View::PositiveGapLog10 { epsilon },
                GenerationView::Log10GenerationPlusOne,
            ),
        ] {
            let mut rendered = population_story::render_run_panel(&trace, view, generation)?;
            if name != "asinh" {
                let variant = if name == "positive-gap" {"gap"} else {"affine"};
                let path = format!("docs/paper/generated/population/positive/run-{public:02}-positive-points-{variant}.v1.json");
                let all_points: Value = serde_json::from_str(outputs.get(&path).ok_or("population story: missing real positive point companion")?)?;
                // Compare the canonical parameter bytes bound BEFORE the point receipt's
                // JSON parse. Numeric Value equality after a second f64 parse can lose1ulp.
                let expected_axis_sha = sha256_bytes(&serde_json::to_vec(&rendered.receipt["view_axis"])?);
                let parsed_axis_sha = sha256_bytes(&serde_json::to_vec(&all_points["axis"])?);
                if all_points["axis_canonical_json_sha256"] != json!(expected_axis_sha)
                    || all_points["source"]["admitted_original_identity_order_sha256"] != json!(trace.source.membership_identity_order_sha256)
                    || all_points["numeric_point_operations"] != 195456u64 {
                    return Err(format!("population story: Run{public}/{name} point and summary binding differs; point_axis={} expected_axis={expected_axis_sha} point_ids={} expected_ids={} point_operations={}",all_points["axis_canonical_json_sha256"],all_points["source"]["admitted_original_identity_order_sha256"],trace.source.membership_identity_order_sha256,all_points["numeric_point_operations"]).into());
                }
                rendered.receipt["positive_point_companion_binding"] = json!({"path":path,"canonical_axis_sha256_before_parse":expected_axis_sha,
                    "point_receipt_parsed_axis_sha256":parsed_axis_sha,"json_numeric_roundtrip_exact":all_points["axis"]==rendered.receipt["view_axis"],
                    "identity_order_and195456operations_exact":true,"reason":"Exact original parameter serialization is bound before parsing; no approximate numeric comparison or changed coordinate."});
            }
            let prefix = format!("docs/paper/generated/population/story/run-{public:02}-{name}");
            let svg_path = format!("{prefix}.svg");
            let receipt_path = format!("{prefix}.v1.json");
            let svg_sha = sha256_bytes(rendered.svg.as_bytes());
            let mut receipt = rendered.receipt;
            receipt["recipe_source"] = json!({"path":CONTRACT,"sha256":inputs.get(CONTRACT).ok_or("population story: unbound recipe")?});
            receipt["source_summary_archive_join"] = json!({"package_id":package,"original_csv_sha256":row["original_csv_sha256"],"manifest_sha256":row["manifest_sha256"],"best_trace_matches_decoded_births":true,"median":"preserved bound full-population source summary; not recomputed by story renderer","leader_token_authority":"bound retained-leaders JSON numeric serialization, not raw CSV spelling"});
            receipt["svg_sha256"] = json!(svg_sha);
            let receipt_json = serde_json::to_string_pretty(&receipt)? + "\n";
            views.push(json!({"view":name,"svg":svg_path,"svg_sha256":svg_sha,"receipt":receipt_path,"receipt_sha256":sha256_bytes(receipt_json.as_bytes())}));
            outputs.insert(svg_path, rendered.svg);
            outputs.insert(receipt_path, receipt_json);
        }
        index_rows.push(json!({"public_run_id":public,"package_id":package,"views":views}));
    }
    outputs.insert("docs/paper/generated/population/story/index.v1.json".into(),serde_json::to_string_pretty(&json!({"schema":"jarambit.native_story_comparison_index.v1","runs":index_rows,
        "selection":"bounded comparison only; main view selection requires actual final-width readback",
        "all_record_companion":"docs/paper/figures/population-latest.svg","all_record_numeric_operations":1_958_803,"unknown_nonnumeric":1390,"global_comparable_champion":null}))?+"\n");
    Ok(())
}

/// Deterministic Rust emitter for all 6 multi-cohort SVGs, `shared_projection.v2.json`,
/// `generated/overview.md`, and `publication.v2.json` (`JR-R2`, `JR-R3`, `SC-DEC-183Z`, & `SC-DEC-183AB`).
/// Called directly by `crate::publication::generate` and covered by `--check`.
/// Extend outputs after the admitted six-source weighted-summary projection.
pub fn generate_v2_outputs(
    root: &Path,
    out: &mut BTreeMap<String, String>,
    inputs: &mut BTreeMap<String, String>,
) -> Result<()> {
    let weighted: Value = serde_json::from_str(
        out.get("docs/paper/generated/weighted-runs.json")
            .ok_or("population: missing admitted weighted-summary projection")?,
    )?;
    let interpretation = crate::interpretation::project(root, inputs)?;
    out.insert(
        "docs/paper/generated/reporting-interpretation.v2.json".into(),
        serde_json::to_string_pretty(&interpretation)? + "\n",
    );
    let required_rels = [
        "evidence/rs_v2/w282_ultrarunner/search/evidence.jevl",
        "evidence/rs_v2/w282_ultrarunner/search/all_evals.json",
        "evidence/rs_v2/w282_ultrarunner/search/all_evals.csv",
        "evidence/rs_v2/w282_ultrarunner/search_summary.json",
        W411C3_FITNESS_CURVE_RELPATH,
        "docs/paper/JR_EI_P1_T04_HISTORICAL_RECONCILIATION_RECEIPT_20261019.json",
        "docs/paper/JR_EI_P1_T05_NEWER_BIRTHS_SCREENS_JOIN_RECEIPT_20261019.json",
        "docs/paper/JR_EI_P1_T06_NATIVE_SUMMARY_ADMISSION_RECEIPT_20261019.json",
        "docs/paper/JR_EI_P1_T07_SEALED_SNAPSHOT_INVENTORY_AND_ACCESSION_RECEIPT_20261019.json",
        "docs/paper/JR_EI_P1_T08_LOSSLESS_GEN800_POPULATION_ACCESSION_RECEIPT_20261019.json",
        "docs/paper/data/public_ids.v1.json",
        "docs/paper/data/selection_receipts.v1.json",
        "docs/paper/data/score_profiles.v1.json",
        "docs/paper/data/gen800_population_density_and_traces.v1.json",
        "docs/paper/generated/population/historical_births.json",
        "docs/paper/generated/population/historical_membership.json",
        "docs/paper/generated/population/newer_events.json",
        "docs/paper/generated/population/latest_snapshots.json",
        "docs/paper/data/score-axis-contract.v1.json",
        "crates/paper-artifacts/src/identity.rs",
        "crates/paper-artifacts/src/corpus.rs",
        "crates/paper-artifacts/src/population.rs",
        "crates/paper-artifacts/src/population_story.rs",
        "docs/paper/data/population-story-contract.v1.json",
        "crates/paper-artifacts/examples/publication_v2_audit.rs",
    ];
    for rel in required_rels {
        let p = crate::corpus::resolve_safe_relpath(root, rel)?;
        let bytes = fs::read(&p)?;
        inputs.insert(rel.to_string(), sha256_bytes(&bytes));
    }

    if inputs[W411C3_FITNESS_CURVE_RELPATH] != W411C3_FITNESS_CURVE_SHA256 {
        return Err(format!(
            "population: {} sha256 mismatch: expected {}, got {}",
            W411C3_FITNESS_CURVE_RELPATH,
            W411C3_FITNESS_CURVE_SHA256,
            inputs[W411C3_FITNESS_CURVE_RELPATH]
        )
        .into());
    }

    // Recompute and validate the Cohort 1 3-hop inheritance ledger directly from raw w282 JSON + CSV
    let inheritance_ledger = recompute_w282_inheritance_ledger(root)?;
    validate_inheritance_ledger(&inheritance_ledger)?;
    let inheritance_json_str = serde_json::to_string_pretty(&inheritance_ledger)? + "\n";
    let inheritance_json_sha = sha256_bytes(inheritance_json_str.as_bytes());
    inputs.insert(
        "docs/paper/generated/population/inheritance.json".into(),
        inheritance_json_sha.clone(),
    );
    out.insert(
        "docs/paper/generated/population/inheritance.json".into(),
        inheritance_json_str,
    );

    // Reconcile Cohort 1 (JEV1 binary frames + CSV + JSON) and Cohort 2 (57 JSONL pairs) from raw bytes
    let hist_rec = crate::corpus::read_and_reconcile_historical_w282(root)?;
    let newer_rec = crate::corpus::read_and_join_newer_corpus(root)?;

    // Read raw w282 births from all_evals.json
    let pop_bytes = fs::read(root.join("evidence/rs_v2/w282_ultrarunner/search/all_evals.json"))?;
    let births: Vec<Value> = serde_json::from_slice(&pop_bytes)?;
    if births.len() != 1984 {
        return Err("population: expected 1984 births in w282 all_evals.json".into());
    }

    // Read retained original w411c3 fitness_curve.json and cross-verify against score_profiles.v1.json
    let w411c3_curve: Value =
        serde_json::from_slice(&fs::read(root.join(W411C3_FITNESS_CURVE_RELPATH))?)?;
    let score_profiles: Value = serde_json::from_slice(&fs::read(
        root.join("docs/paper/data/score_profiles.v1.json"),
    )?)?;
    let sp_w411_pts = score_profiles["progress_series"]
        .as_array()
        .and_then(|arr| {
            arr.iter()
                .find(|s| s["series_id"].as_str() == Some("w411c3_ultralight_reporting"))
        })
        .and_then(|s| s["points"].as_array())
        .ok_or("population: missing w411c3_ultralight_reporting in score_profiles.v1.json")?;
    let w411_gen_rows = w411c3_curve["generations"]
        .as_array()
        .ok_or("population: missing generations in w411c3 fitness_curve.json")?;
    if sp_w411_pts.len() != 3 || w411_gen_rows.len() != 3 {
        return Err("population: expected 3 generation points for w411c3".into());
    }
    let mut w411_points_proj = Vec::with_capacity(3);
    for (idx, (raw_row, sp_pt)) in w411_gen_rows.iter().zip(sp_w411_pts.iter()).enumerate() {
        let gen = raw_row["generation"]
            .as_u64()
            .ok_or("population: missing generation")?;
        let hof = raw_row["hall_of_fame_best_so_far"]
            .as_f64()
            .ok_or("population: missing hall_of_fame_best_so_far")?;
        let rnd = raw_row["round_best_fitness"]
            .as_f64()
            .ok_or("population: missing round_best_fitness")?;
        let sp_cum = sp_pt["cumulative_best_native"]
            .as_f64()
            .ok_or("population: missing cumulative_best_native")?;
        let sp_gen = sp_pt["generation_best_native"]
            .as_f64()
            .ok_or("population: missing generation_best_native")?;
        if (hof - sp_cum).abs() > 1e-9 || (rnd - sp_gen).abs() > 1e-9 {
            return Err(
                "population: w411c3/fitness_curve.json and score_profiles.v1.json disagree".into(),
            );
        }
        let best_lbl = raw_row["best_so_far_label"]
            .as_str()
            .ok_or("population: missing best_so_far_label")?;
        let pub_ind_id = match best_lbl {
            "w411c3_r01_p10_c11" => 2009u64,
            "w411c3_r02_p10_c12" => 2002u64,
            _ => return Err(format!("population: unmapped w411c3 label {best_lbl}").into()),
        };
        w411_points_proj.push(json!({
            "source_pointer": format!("/generations/{idx}"),
            "generation": gen,
            "round_one_based": gen + 1,
            "declared_round_file": raw_row["round_file"],
            "best_so_far_public_individual_id": pub_ind_id,
            "best_so_far_public_label": format!("Individual {pub_ind_id}"),
            "best_so_far_internal_alias": best_lbl,
            "hall_of_fame_best_so_far": hof,
            "round_best_fitness": rnd,
            "point_stage_provenance": "unknown"
        }));
    }

    let t05: Value = serde_json::from_slice(&fs::read(
        root.join("docs/paper/JR_EI_P1_T05_NEWER_BIRTHS_SCREENS_JOIN_RECEIPT_20261019.json"),
    )?)?;
    let gen800_density: Value = serde_json::from_slice(&fs::read(
        root.join("docs/paper/data/gen800_population_density_and_traces.v1.json"),
    )?)?;
    let scatter_cache_rel = "docs/paper/generated/population/gen800_scatter_cache.v1.json";
    let scatter_cache_bytes = fs::read(root.join(scatter_cache_rel))?;
    inputs.insert(
        scatter_cache_rel.to_string(),
        sha256_bytes(&scatter_cache_bytes),
    );

    let svg_births = render_population_births_svg(&births)?;
    let svg_membership = render_population_membership_svg(&hist_rec)?;
    let svg_newer = render_population_newer_svg(&t05)?;
    let (svg_latest, mut point_receipt, mut point_outputs) =
        render_population_latest_svg(root, &births, &newer_rec, &t05)?;
    let registry: Value =
        serde_json::from_slice(&fs::read(root.join("docs/paper/data/public_ids.v1.json"))?)?;
    let mut weighted_leaders = Vec::new();
    for run in weighted["runs"]
        .as_array()
        .ok_or("population: missing weighted summary projection")?
    {
        let rows = run["points"]
            .as_array()
            .ok_or("population: missing weighted summary rows")?;
        for metric in ["round_best_fitness", "hall_of_fame_best_so_far"] {
            let maximum = point_bounds(rows.iter().map(|row| row[metric].as_f64().unwrap()))?[1];
            let ties: Vec<_> = rows.iter().filter(|row|row[metric].as_f64()==Some(maximum)).map(|row| {
                let alias=if metric == "round_best_fitness" { Value::Null } else { registry["allocated_individuals"].as_array().unwrap().iter().find(|entry|entry["aliases"].as_array().unwrap().contains(&row["best_so_far_label"])).map(|entry|entry["public_id"].clone()).unwrap_or(Value::Null) };
                json!({"generation":row["generation"],"source_pointer":format!("{}/{}",row["source_pointer"].as_str().unwrap(),metric),"best_so_far_label":row["best_so_far_label"],"public_individual_label_binding":alias,"alias_limit":"Label binding does not identify a round population/CAD artifact or prove scorer/stage comparability"})
            }).collect();
            weighted_leaders.push(json!({"summary_source":run["source"],"source_sha256":run["source_sha256"],"metric":metric,"native_direction":"higher_is_better","value":maximum,"ties":ties,"producer_revision":null,"scorer_revision":null,"point_stage":"not_established","comparability":"not_established","scope":"Numerically largest retained summary within this source/metric only; no birth distribution, global champion or downstream CAD assessment inferred"}));
        }
    }
    point_receipt["retained_leaders"]["weighted_summary_by_source_and_metric"] =
        json!(weighted_leaders);
    point_outputs.insert(
        "docs/paper/generated/population/retained-leaders.v1.json".into(),
        serde_json::to_string_pretty(&point_receipt["retained_leaders"])? + "\n",
    );
    let story_contract: Value = serde_json::from_slice(&fs::read(
        root.join("docs/paper/data/population-story-contract.v1.json"),
    )?)?;
    append_native_story_views(
        &gen800_density,
        &point_receipt,
        &story_contract,
        inputs,
        &mut point_outputs,
    )?;
    point_receipt["additional_outputs_sha256"] = json!(point_outputs
        .iter()
        .map(|(path, contents)| (path.clone(), sha256_bytes(contents.as_bytes())))
        .collect::<BTreeMap<_, _>>());
    out.extend(point_outputs);
    point_receipt["figure_sha256"] = json!(sha256_bytes(svg_latest.as_bytes()));
    let point_receipt_json = serde_json::to_string_pretty(&point_receipt)? + "\n";
    let point_receipt_sha = sha256_bytes(point_receipt_json.as_bytes());
    out.insert(
        "docs/paper/generated/population/point-rendering.v1.json".into(),
        point_receipt_json,
    );
    let svg_fitness = render_fitness_progress_svg(
        &births,
        &w411c3_curve,
        W411C3_FITNESS_CURVE_SHA256,
        &gen800_density,
    )?;
    let svg_inheritance = render_inheritance_svg(&inheritance_ledger, false)?;
    let svg_inheritance_details = render_inheritance_svg(&inheritance_ledger, true)?;
    let (svg_hero, display_composite) = render_hero_view(root, HeroView::FullRange)?;
    inputs.extend(display_composite.input_hashes.clone());
    out.insert(
        "docs/paper/generated/test-field-roster.v1.tsv".into(),
        display_composite.roster_table.clone(),
    );
    let mut display_receipt = display_composite.receipt;
    let mut display_outputs = BTreeMap::new();
    for (path, view) in [
        (
            "docs/paper/generated/hero-contrast-preview.svg",
            HeroView::ContrastPreview,
        ),
        ("docs/paper/generated/hero-mesh-on.svg", HeroView::MeshOn),
        ("docs/paper/generated/hero-support.svg", HeroView::Support),
    ] {
        let (svg, _) = render_hero_view(root, view)?;
        display_outputs.insert(path.to_string(), sha256_bytes(svg.as_bytes()));
        out.insert(path.into(), svg);
    }
    let hero_path = "docs/paper/drafts_p3_p5/JR_EI_P4_T01_blender_exact_cad_hero_white.svg";
    display_outputs.insert(hero_path.into(), sha256_bytes(svg_hero.as_bytes()));
    out.insert(
        "docs/paper/generated/hero-full-range.svg".into(),
        svg_hero.clone(),
    );
    display_outputs.insert(
        "docs/paper/generated/hero-full-range.svg".into(),
        sha256_bytes(svg_hero.as_bytes()),
    );
    display_outputs.insert(
        "docs/paper/generated/test-field-roster.v1.tsv".into(),
        sha256_bytes(out["docs/paper/generated/test-field-roster.v1.tsv"].as_bytes()),
    );
    display_receipt["outputs_sha256"] = json!(display_outputs);
    out.insert(
        "docs/paper/generated/display-composite.v1.json".into(),
        serde_json::to_string_pretty(&display_receipt)? + "\n",
    );

    out.insert(
        "docs/paper/figures/population-births.svg".into(),
        svg_births,
    );
    out.insert(
        "docs/paper/figures/population-membership.svg".into(),
        svg_membership,
    );
    out.insert("docs/paper/figures/population-newer.svg".into(), svg_newer);
    out.insert(
        "docs/paper/figures/population-latest.svg".into(),
        svg_latest,
    );
    out.insert(
        "docs/paper/figures/fitness-progress.svg".into(),
        svg_fitness,
    );
    out.insert("docs/paper/figures/inheritance.svg".into(), svg_inheritance);
    out.insert(
        "docs/paper/figures/inheritance-details.svg".into(),
        svg_inheritance_details,
    );
    out.insert(
        "docs/paper/drafts_p3_p5/JR_EI_P4_T01_blender_exact_cad_hero_white.svg".into(),
        svg_hero,
    );

    let fig_hashes: BTreeMap<String, String> = [
        "docs/paper/figures/population-births.svg",
        "docs/paper/figures/population-membership.svg",
        "docs/paper/figures/population-newer.svg",
        "docs/paper/figures/population-latest.svg",
        "docs/paper/figures/fitness-progress.svg",
        "docs/paper/figures/inheritance.svg",
        "docs/paper/figures/inheritance-details.svg",
        "docs/paper/drafts_p3_p5/JR_EI_P4_T01_blender_exact_cad_hero_white.svg",
    ]
    .iter()
    .map(|&k| (k.to_string(), sha256_bytes(out[k].as_bytes())))
    .collect();

    let shared_proj = json!({
        "schema": "jarambit.shared_projection.v2",
        "generator": "cargo run --locked --manifest-path crates/paper-artifacts/Cargo.toml --example paper_artifacts",
        "live_export_web_data_invoked": false,
        "qualified_candidate_count": 0,
        "qualification_scope": {
            "source_relpath": "evidence/evolve_certifiable/automatic/leaderboard.json",
            "assessed_count": 1,
            "qualified_count": 0,
            "champion": Value::Null,
            "cohort_stage2_note": "Historical Individual1095 (fit=+1.157270,rank1) fails that run's earlier Boolean cert_pass_all predicate; Individual1543 (fit=1.15415,rank8) passes it. This is not later continuous-score rank reversal or 26-requirement qualification. Proposal-linked Individual2002 has a retained later unqualified assessment summary, not full underlying CAD/scorecards. Completed native per-birth later-assessment joins remain absent; missing retention is not proof no computation occurred."
        },
        "cohorts": {
            "cohort_1_historical_17gen": {
                "reader_label": "Cohort 1 (Historical 17-Gen Reconstruction)",
                "public_run_ids": [1],
                "source_dir": "evidence/rs_v2/w282_ultrarunner",
                "producer_commit": "82f450f7fb418e28c4b229031920dec5454435f3",
                "births": hist_rec["births"],
                "fem_evaluated_births": hist_rec["fem_evaluated_births"],
                "not_evaluated_births": hist_rec["not_evaluated_births"],
                "legal_births": hist_rec["legal_births"],
                "membership_occurrences": hist_rec["membership_occurrences"],
                "elite_reappearances": hist_rec["elite_reappearances"],
                "duplicate_slot_keys": hist_rec["duplicate_slot_keys"],
                "migration_events": hist_rec["migration_events"],
                "migration_copied_ids": hist_rec["migration_copied_ids"],
                "migration_generations": hist_rec["migration_generations"],
                "distinct_serialized_gene_vectors": hist_rec["distinct_serialized_gene_vectors"]
            },
            "cohort_2_newer_screening": {
                "reader_label": "Cohort 2 (Intermediate Screening Corpus)",
                "birth_files": newer_rec["birth_files"],
                "run_directories": newer_rec["run_directories"],
                "birth_events": newer_rec["birth_events"],
                "joined_screen_rows": newer_rec["joined_screen_rows"],
                "unscreened_birth_events": newer_rec["unscreened_birth_events"],
                "join_index_base": 1,
                "zero_based_misjoins_rejected": newer_rec["zero_based_misjoins_rejected"],
                "selected_run_31_reporting_curve": {
                    "public_run_id": 31,
                    "public_label": "Run 31",
                    "internal_run_key": "evidence/evolution_waves/2026-10-18_ultralight_v1/xbabe3/w411c3",
                    "source_relpath": W411C3_FITNESS_CURVE_RELPATH,
                    "source_sha256": W411C3_FITNESS_CURVE_SHA256,
                    "selected_public_individual_id": 2002,
                    "selected_public_label": "Individual 2002",
                    "selected_internal_alias": "w411c3_r02_p10_c12",
                    "point_stage_provenance": "unknown",
                    "points": w411_points_proj
                }
            },
            "cohort_3_gen800_population": {
                "reader_label": "Cohort 3 (Ten Retained 800-Gen Runs: Run 2–Run 11)",
                "public_run_ids": [2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
                "legacy_superseded_run_ids": [37, 38, 39, 40, 41, 42, 43, 44, 45, 46],
                "archive_index_relpath": "artifacts/versioned-evolution-archives-gen800-population-v1/index.gen800_population.sha256.json",
                "archive_index_sha256": crate::corpus::GEN800_POPULATION_INDEX_SHA256,
                "density_contract_relpath": "docs/paper/data/gen800_population_density_and_traces.v1.json",
                "density_contract_sha256": inputs["docs/paper/data/gen800_population_density_and_traces.v1.json"],
                "completed_runs": 10,
                "rows_per_run": 195456,
                "total_rows": 1954560,
                "total_legal_rows": 754236,
                "total_penalty_rows": 1200324,
                "legal_rows_with_negative_fit": 610051,
                "legal_rows_with_nonnegative_fit": 144185,
                "generations_per_run": 801,
                "zstd_segments": 200,
                "compressed_bytes": 494843141,
                "uncompressed_csv_bytes": 3811896749u64,
                "separated_stl_files": 24,
                "separated_stl_bytes": 62331126,
                "v2_staged_summary_files": 11,
                "v2_staged_summary_rows": 17672,
                "aborted_attempts": 9,
                "aborted_attempts_with_csv": 7,
                "aborted_summary_rows": 216,
                "fem_status_inferred": false
            }
        },
        "figures_sha256": fig_hashes
    });
    let shared_proj_str = serde_json::to_string_pretty(&shared_proj)? + "\n";
    let shared_proj_sha = sha256_bytes(shared_proj_str.as_bytes());
    out.insert(
        "docs/paper/generated/shared_projection.v2.json".into(),
        shared_proj_str,
    );

    let overview_md = format!(
        "# Multi-Cohort Population & Evidence Overview (Generated by `crates/paper-artifacts`)\n\n\
         - **Generator:** `cargo run --locked --manifest-path crates/paper-artifacts/Cargo.toml --example paper_artifacts` (`--check` is a separate validation command)\n\
         - **Qualification posture (`evidence/evolve_certifiable/automatic/leaderboard.json`):** `assessed_count = 1`, `qualified_count = 0`, `champion = null`. In historical Run1, Individual1095 (rank1,fit1.15727) fails that run's earlier Boolean predicate while Individual1543 (rank8,fit1.15415) passes it; neither label is a later26-requirement qualification. Proposal-linked Individual2002 has a retained later unqualified summary, with full underlying CAD/scorecards missing. Completed native per-birth later-assessment joins remain unretained.\n\
         - **Cohort 1 (Historical 17-Gen Reconstruction, `Run 1`):** `1,984` unique births (`1,455` FEM-complete + `529` not-evaluated; `446` legal), `2,176` membership occurrences (`192` elite reappearances; `9` duplicate slot keys), `12` ring migrations at **`G4 / G8 / G12`** (`Island 1→2, 2→3, 3→4, 4→1`, source `island: 0..3`, `24` copied IDs), `1,965` distinct 129-column gene+material serializations.\n\
         - **Cohort 2 (Intermediate Screening Corpus):** `57` birth files across `24` run directories = `3,649` birth events (`2,259` 1-based joined screen rows + `1,390` unscreened birth events; `2,247` 0-based misjoins rejected). Selected reporting series `Run 31` (`{W411C3_FITNESS_CURVE_RELPATH}`, `sha256: {W411C3_FITNESS_CURVE_SHA256}`) records 20-term weighted profile `F = 74.207299 / 100.0` (`30%` mass/compactness `15.470`, `25%` form/comfort `23.875`, `15%` exported geometry `11.841`, `30%` response/verification `23.022`) with `hall_of_fame_best_so_far = [74.167488, 74.207299, 74.207299]` and `round_best_fitness = [74.167488, 74.207299, 73.608446]` (`Individual 2009` at Gen 0, `Individual 2002` at Gen 1–2).\n\
         - **Cohort 3 (Ten Retained 800-Gen Runs, `Run 2`–`Run 11`):** `10` retained 800-generation runs (`Run 2`–`Run 11`, superseding legacy `Run 37`–`Run 46`) = **`1,954,560` evaluated individuals** (`195,456` per run across `801` generations `0..800`; `754,236` `legal=true` rows [`610,051` with `fit < 0` + `144,185` with `fit >= 0`] + `1,200,324` `legal=false` penalty rows) stored across `200` lossless `.csv.zst` segments (`494,843,141` B compressed / `3,811,896,749` B raw CSV), with `24` STL files (`62,331,126` B) separated from routine compact Git payload, plus `11` V2 staged summary snapshots (`17,672` rows) and `9` aborted attempts (`216` rows across `7` CSVs, `2` null).\n\
         - **Shared projection SHA-256:** `{shared_proj_sha}`\n"
    );
    let overview_sha = sha256_bytes(overview_md.as_bytes());
    out.insert("docs/paper/generated/overview.md".into(), overview_md);

    let mut pub_v2 = json!({
        "schema": "jarambit.publication.v2",
        "generator": "cargo run --locked --manifest-path crates/paper-artifacts/Cargo.toml --example paper_artifacts",
        "audit_command": "cargo run --release --locked --manifest-path crates/paper-artifacts/Cargo.toml --example publication_v2_audit -- --full-zstd",
        "quick_audit_command": "cargo run --release --locked --manifest-path crates/paper-artifacts/Cargo.toml --example publication_v2_audit",
        "current_interpretation": {"contract":"docs/paper/data/reporting-interpretation-contract.v2.json","projection":"docs/paper/generated/reporting-interpretation.v2.json","original_contracts_preserved":true},
        "live_export_web_data_invoked": false,
        "archive_contracts": {
            "gen800_population_archive_root_relpath": "artifacts/versioned-evolution-archives-gen800-population-v1",
            "gen800_population_index_relpath": "artifacts/versioned-evolution-archives-gen800-population-v1/index.gen800_population.sha256.json",
            "gen800_population_index_sha256": crate::corpus::GEN800_POPULATION_INDEX_SHA256,
            "gen800_density_and_traces_relpath": "docs/paper/data/gen800_population_density_and_traces.v1.json",
            "gen800_density_and_traces_sha256": inputs["docs/paper/data/gen800_population_density_and_traces.v1.json"]
        },
        "inputs_sha256": {
            "docs/paper/data/reporting-interpretation-contract.v2.json": inputs["docs/paper/data/reporting-interpretation-contract.v2.json"],
            "crates/paper-artifacts/src/interpretation.rs": inputs["crates/paper-artifacts/src/interpretation.rs"],
            "docs/paper/history/ultralight_weighted_fitness_009f5ac5.source.txt": inputs["docs/paper/history/ultralight_weighted_fitness_009f5ac5.source.txt"],
            "docs/paper/data/public_ids.v1.json": inputs["docs/paper/data/public_ids.v1.json"],
            "docs/paper/data/selection_receipts.v1.json": inputs["docs/paper/data/selection_receipts.v1.json"],
            "docs/paper/data/score_profiles.v1.json": inputs["docs/paper/data/score_profiles.v1.json"],
            "docs/paper/data/gen800_population_density_and_traces.v1.json": inputs["docs/paper/data/gen800_population_density_and_traces.v1.json"],
            "evidence/evolution_waves/2026-10-18_ultralight_v1/xbabe3/w411c3/fitness_curve.json": inputs[W411C3_FITNESS_CURVE_RELPATH],
            "docs/paper/JR_EI_P1_T04_HISTORICAL_RECONCILIATION_RECEIPT_20261019.json": inputs["docs/paper/JR_EI_P1_T04_HISTORICAL_RECONCILIATION_RECEIPT_20261019.json"],
            "docs/paper/JR_EI_P1_T05_NEWER_BIRTHS_SCREENS_JOIN_RECEIPT_20261019.json": inputs["docs/paper/JR_EI_P1_T05_NEWER_BIRTHS_SCREENS_JOIN_RECEIPT_20261019.json"],
            "docs/paper/JR_EI_P1_T06_NATIVE_SUMMARY_ADMISSION_RECEIPT_20261019.json": inputs["docs/paper/JR_EI_P1_T06_NATIVE_SUMMARY_ADMISSION_RECEIPT_20261019.json"],
            "docs/paper/JR_EI_P1_T07_SEALED_SNAPSHOT_INVENTORY_AND_ACCESSION_RECEIPT_20261019.json": inputs["docs/paper/JR_EI_P1_T07_SEALED_SNAPSHOT_INVENTORY_AND_ACCESSION_RECEIPT_20261019.json"],
            "docs/paper/JR_EI_P1_T08_LOSSLESS_GEN800_POPULATION_ACCESSION_RECEIPT_20261019.json": inputs["docs/paper/JR_EI_P1_T08_LOSSLESS_GEN800_POPULATION_ACCESSION_RECEIPT_20261019.json"]
        },
        "population_ledgers_sha256": {
            "docs/paper/generated/population/historical_births.json": inputs["docs/paper/generated/population/historical_births.json"],
            "docs/paper/generated/population/historical_membership.json": inputs["docs/paper/generated/population/historical_membership.json"],
            "docs/paper/generated/population/newer_events.json": inputs["docs/paper/generated/population/newer_events.json"],
            "docs/paper/generated/population/latest_snapshots.json": inputs["docs/paper/generated/population/latest_snapshots.json"],
            "docs/paper/generated/population/inheritance.json": inheritance_json_sha,
            "docs/paper/generated/population/gen800_scatter_cache.v1.json": inputs["docs/paper/generated/population/gen800_scatter_cache.v1.json"]
        },
        "outputs_sha256": {
            "docs/paper/figures/population-births.svg": fig_hashes["docs/paper/figures/population-births.svg"],
            "docs/paper/figures/population-membership.svg": fig_hashes["docs/paper/figures/population-membership.svg"],
            "docs/paper/figures/population-newer.svg": fig_hashes["docs/paper/figures/population-newer.svg"],
            "docs/paper/figures/population-latest.svg": fig_hashes["docs/paper/figures/population-latest.svg"],
            "docs/paper/generated/population/point-rendering.v1.json": point_receipt_sha,
            "docs/paper/figures/fitness-progress.svg": fig_hashes["docs/paper/figures/fitness-progress.svg"],
            "docs/paper/figures/inheritance.svg": fig_hashes["docs/paper/figures/inheritance.svg"],
            "docs/paper/figures/inheritance-details.svg": fig_hashes["docs/paper/figures/inheritance-details.svg"],
            "docs/paper/drafts_p3_p5/JR_EI_P4_T01_blender_exact_cad_hero_white.svg": fig_hashes["docs/paper/drafts_p3_p5/JR_EI_P4_T01_blender_exact_cad_hero_white.svg"],
            "docs/paper/generated/shared_projection.v2.json": shared_proj_sha,
            "docs/paper/generated/overview.md": overview_sha
        }
    });
    for (path, contents) in out.iter().filter(|(path, _)| {
        (path.starts_with("docs/paper/generated/hero-")
            || path.starts_with("docs/paper/generated/native-runs-"))
            || path.as_str() == "docs/paper/generated/display-composite.v1.json"
            || path.as_str() == "docs/paper/generated/reporting-interpretation.v2.json"
            || path.as_str() == "docs/paper/generated/test-field-roster.v1.tsv"
            || path.as_str() == "docs/paper/generated/population-detail.svg"
            || path.as_str() == "docs/paper/generated/population/score-axis.v1.json"
            || path.as_str() == "docs/paper/generated/population/retained-leaders.v1.json"
    }) {
        pub_v2["outputs_sha256"][path] = json!(sha256_bytes(contents.as_bytes()));
    }
    out.insert(
        "docs/paper/publication.v2.json".into(),
        serde_json::to_string_pretty(&pub_v2)? + "\n",
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn neutral_tail_receipt(rows: &[(u64, f64)], direction: DetailDirection) -> Value {
        let values: Vec<_> = rows.iter().map(|row| row.1).collect();
        let mut selection = DirectedDetail::new(&values, direction).unwrap();
        let mut raster = PointRaster::new(11, 11, [0., rows.len() as f64], selection.bounds)
            .unwrap().with_axis(crate::score_axis::ScoreAxis::Linear);
        for (position, &(id, score)) in rows.iter().enumerate() {
            if selection.observe(&[id], score).unwrap() {
                raster.paint(position as f64, score).unwrap();
            }
        }
        selection.receipt(&raster, &["neutral_id"]).unwrap()
    }

    #[test]
    fn directed_detail_uses_original_direction_and_retains_extrema() {
        let values: Vec<_> = (0..=10).map(f64::from).collect();
        assert_eq!(directed_detail_bounds(&values, DetailDirection::Higher).unwrap(), [9., 10.]);
        assert_eq!(directed_detail_bounds(&values, DetailDirection::Lower).unwrap(), [0., 1.]);
        let rows: Vec<_> = values.iter().enumerate().map(|(i, &v)| (i as u64, v)).collect();
        let higher = neutral_tail_receipt(&rows, DetailDirection::Higher);
        let lower = neutral_tail_receipt(&rows, DetailDirection::Lower);
        assert_eq!(higher["below_inside_above"], json!([9, 2, 0]));
        assert_eq!(lower["below_inside_above"], json!([0, 2, 9]));
        assert_eq!(higher["extremum"]["value"], 10.);
        assert_eq!(lower["extremum"]["value"], 0.);
    }

    #[test]
    fn directed_detail_includes_every_cutoff_and_extremum_tie() {
        // Q90 index18 cuts at18, including both18s plus the20 maximum.
        let mut rows: Vec<_> = (0..=20).map(|id| (id, id as f64)).collect();
        rows[19].1 = 18.;
        let higher = neutral_tail_receipt(&rows, DetailDirection::Higher);
        assert_eq!(higher["bounds"], json!([18., 20.]));
        assert_eq!(higher["below_inside_above"], json!([18, 3, 0]));
        assert_eq!(higher["cutoff_ties"], json!({"source_count":2,"included_count":2}));
        let lower_rows: Vec<_> = rows.iter().map(|&(id, value)| (id, -value)).collect();
        let lower = neutral_tail_receipt(&lower_rows, DetailDirection::Lower);
        assert_eq!(lower["bounds"], json!([-20., -18.]));
        assert_eq!(lower["below_inside_above"], json!([0, 3, 18]));
        for result in [&higher, &lower] {
            assert_eq!(result["extremum"]["source_tie_count"], 1);
            assert_eq!(result["extremum"]["included_tie_count"], 1);
            assert_eq!(result["raster"]["point_operations"], 3);
        }
    }

    #[test]
    fn directed_detail_all_tied_membership_and_display_stay_exact() {
        let rows = [(11, 2.), (12, 2.), (13, 2.), (14, 2.)];
        for direction in [DetailDirection::Higher, DetailDirection::Lower] {
            let result = neutral_tail_receipt(&rows, direction);
            assert_eq!(result["membership_bounds_original_units"], json!([2., 2.]));
            assert_eq!(result["raster"]["score_display_domain"], json!([2., 2.]));
            assert_eq!(result["display_axis_widened"], false);
            assert_eq!(result["below_inside_above"], json!([0, 4, 0]));
            assert_eq!(result["extremum"]["included_tie_count"], 4);
        }
    }

    #[test]
    fn directed_detail_rejects_empty_nonfinite_and_malformed_numeric_input() {
        assert!(DirectedDetail::new(&[], DetailDirection::Higher).is_err());
        for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            for direction in [DetailDirection::Higher, DetailDirection::Lower] {
                assert!(directed_detail_bounds(&[0., bad], direction).is_err());
            }
        }
        for bad in [Value::Null, json!("1"), json!("NaN"), json!({})] {
            assert!(finite_point_number(&bad).is_err());
        }
        let mut selection = DirectedDetail::new(&[1.], DetailDirection::Higher).unwrap();
        assert!(selection.observe(&[], 1.).is_err());
        assert!(selection.observe(&[1], f64::NAN).is_err());
        assert_eq!(selection.counts, [0, 0, 0]);
    }

    #[test]
    fn directed_detail_count_conservation_rejects_missing_and_extra_visits() {
        let mut selection = DirectedDetail::new(&[0., 1., 2.], DetailDirection::Higher).unwrap();
        let mut raster = PointRaster::new(5, 5, [0., 2.], selection.bounds).unwrap();
        assert!(selection.observe(&[1], 1.).unwrap());
        raster.paint(1., 1.).unwrap();
        assert!(selection.receipt(&raster, &["id"]).is_err());
        assert!(!selection.observe(&[0], 0.).unwrap());
        assert!(selection.observe(&[2], 2.).unwrap());
        raster.paint(2., 2.).unwrap();
        assert!(selection.receipt(&raster, &["id"]).is_ok());
        assert!(selection.observe(&[2], 2.).unwrap());
        raster.paint(2., 2.).unwrap();
        assert!(selection.receipt(&raster, &["id"]).is_err());
    }

    #[test]
    fn directed_detail_binding_distinguishes_identity_score_and_order() {
        let rows = [(1, 0.), (2, 9.), (3, 9.), (4, 10.)];
        let original = neutral_tail_receipt(&rows, DetailDirection::Higher);
        let reversed: Vec<_> = rows.iter().rev().copied().collect();
        let reversed = neutral_tail_receipt(&reversed, DetailDirection::Higher);
        assert_eq!(original["below_inside_above"], reversed["below_inside_above"]);
        assert_ne!(original["selected_identity_score_sha256"], reversed["selected_identity_score_sha256"]);
        let mut identity_changed = rows;
        identity_changed[3].0 = 999;
        let changed = neutral_tail_receipt(&identity_changed, DetailDirection::Higher);
        assert_ne!(original["selected_identity_score_sha256"], changed["selected_identity_score_sha256"]);
        let mut score_changed = rows;
        score_changed[3].1 = 11.;
        let changed = neutral_tail_receipt(&score_changed, DetailDirection::Higher);
        assert_ne!(original["extremum"]["ordered_identity_score_sha256"], changed["extremum"]["ordered_identity_score_sha256"]);
    }

    #[test]
    fn directed_detail_single_observation_and_signed_zero_remain_numeric() {
        for direction in [DetailDirection::Higher, DetailDirection::Lower] {
            let one = neutral_tail_receipt(&[(7, -3.)], direction);
            assert_eq!(one["bounds"], json!([-3., -3.]));
            assert_eq!(one["below_inside_above"], json!([0, 1, 0]));
            let zeros = neutral_tail_receipt(&[(1, -0.), (2, 0.)], direction);
            assert_eq!(zeros["below_inside_above"], json!([0, 2, 0]));
            let positive_zeros = neutral_tail_receipt(&[(1, 0.), (2, 0.)], direction);
            assert_ne!(zeros["selected_identity_score_sha256"], positive_zeros["selected_identity_score_sha256"]);
        }
    }

    #[test]
    fn directed_detail_receipt_rejects_undisclosed_axis_widening() {
        let mut selection = DirectedDetail::new(&[0., 9., 10.], DetailDirection::Higher).unwrap();
        let mut raster = PointRaster::new(5, 5, [0., 2.], [0., 10.]).unwrap();
        for (id, value) in [0., 9., 10.].into_iter().enumerate() {
            if selection.observe(&[id as u64], value).unwrap() {
                raster.paint(id as f64, value).unwrap();
            }
        }
        assert!(selection.receipt(&raster, &["id"]).is_err());
    }

    #[test]
    fn hero_lighting_recipe_requires_a_bounded_integral_exponent() {
        assert_eq!(hero_shininess(&json!(24.0)).unwrap(),24);
        assert_eq!(hero_shininess(&json!(128)).unwrap(),128);
        for invalid in [json!(24.5),json!(0),json!(129),json!(-1),Value::Null,json!("24")] {
            assert!(hero_shininess(&invalid).is_err());
        }
    }


    #[test]
    fn hero_rgb_png_preserves_rgb_bytes_white_and_stored_deflate_checksums() {
        let png = encode_hero_rgb_png(1, 1, &[255, 0, 128]).unwrap();
        assert_eq!(&png[..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
        // Literal external PNG layout: width/height1, RGB8, no palette/transparency.
        assert_eq!(&png[12..16], b"IHDR");
        assert_eq!(&png[16..29], &[0, 0, 0, 1, 0, 0, 0, 1, 8, 2, 0, 0, 0]);
        assert_eq!(&png[37..41], b"IDAT");
        assert_eq!(
            &png[41..56],
            &[0x78, 0x01, 0x01, 4, 0, 0xfb, 0xff, 0, 255, 0, 128, 0x03, 0x81, 0x01, 0x80]
        );
        assert_eq!(&png[64..68], b"IEND");
        // Independent bit-polynomial CRC control, rather than invoking png_chunk.
        let mut at = 8;
        while at < png.len() {
            let n = u32::from_be_bytes(png[at..at + 4].try_into().unwrap()) as usize;
            let mut crc = !0u32;
            for &b in &png[at + 4..at + 8 + n] {
                crc ^= u32::from(b);
                for _ in 0..8 {
                    crc = (crc >> 1) ^ (0xedb8_8320 & 0u32.wrapping_sub(crc & 1));
                }
            }
            assert_eq!(
                !crc,
                u32::from_be_bytes(png[at + 8 + n..at + 12 + n].try_into().unwrap())
            );
            at += n + 12;
        }
        assert_eq!(at, png.len());
        let white = encode_hero_rgb_png(2, 1, &[255; 6]).unwrap();
        assert_eq!(&white[48..55], &[0, 255, 255, 255, 255, 255, 255]);
        assert!(encode_hero_rgb_png(0, 1, &[]).is_err());
        assert!(encode_hero_rgb_png(2, 1, &[255; 5]).is_err());
        assert!(encode_hero_rgb_png(4097, 1, &[]).is_err());
        assert_eq!(hero_base64(&[0, 255, 128]), "AP+A");
        assert_eq!(hero_base64(&[255]), "/w==");
        assert_eq!(hero_base64(&[255, 0]), "/wA=");
    }

    #[test]
    fn hero_registration_preflight_preserves_ties_fallback_and_all_component_bindings() {
        use crate::hero_appearance::{CellColor, RegistrationInput};
        let centroids: [[f64; 2]; 3] = [[-1.0, 0.0], [1.0, 0.0], [100.0, 100.0]];
        let mut grid = BTreeMap::<(i32, i32), Vec<usize>>::new();
        for (i, c) in centroids.iter().enumerate() {
            grid.entry(((c[0] * 0.5).floor() as i32, (c[1] * 0.5).floor() as i32))
                .or_default()
                .push(i);
        }
        assert_eq!(hero_nearest_cell(0.0, 0.0, &centroids, &grid).unwrap(), 0);
        assert_eq!(
            hero_nearest_cell(90.0, 100.0, &centroids, &grid).unwrap(),
            2
        );
        assert!(hero_nearest_cell(0.0, 0.0, &[], &BTreeMap::new()).is_err());
        let channels: Vec<String> = ["a", "b", "c", "d"]
            .into_iter()
            .map(str::to_string)
            .collect();
        let input = RegistrationInput {
            subject: "neutral fixture".into(),
            d1_sha256: sha256_bytes(b"fixture D1"),
            channel_ids: channels.clone(),
            cells: vec![
                CellColor {
                    index: Some(0.0),
                    missing_reason: None,
                    source_masks: vec![false; 4],
                    contributors: channels,
                    base_rgb: [8, 28, 74],
                },
                CellColor {
                    index: None,
                    missing_reason: Some("fixture unavailable".into()),
                    source_masks: vec![true; 4],
                    contributors: vec![],
                    base_rgb: [203, 213, 225],
                },
            ],
            face_to_cell: vec![0, 1, 0],
        };
        let parts = hero_registration_components(&input);
        assert_eq!(
            parts["ordered_face_to_original_cell_ids"],
            sha256_bytes(
                &[0u64, 1, 0]
                    .into_iter()
                    .flat_map(u64::to_le_bytes)
                    .collect::<Vec<_>>()
            )
        );
        assert_eq!(
            parts["original_source_mask_bits"],
            sha256_bytes(&[0, 0, 0, 0, 1, 1, 1, 1])
        );
        let mut changed = input.clone();
        changed.cells[0].index = Some(0.1);
        assert_ne!(
            parts["registered_index_bits"],
            hero_registration_components(&changed)["registered_index_bits"]
        );
        assert_ne!(input.digest().unwrap(), changed.digest().unwrap());
        let mut changed = input.clone();
        changed.cells[0].contributors.swap(0, 1);
        assert_ne!(
            parts["contributor_ids_and_counts"],
            hero_registration_components(&changed)["contributor_ids_and_counts"]
        );
        let mut changed = input.clone();
        changed.cells[0].base_rgb[0] += 1;
        assert_ne!(
            parts["unlit_cell_palette_rgb"],
            hero_registration_components(&changed)["unlit_cell_palette_rgb"]
        );
        let mut changed = input.clone();
        changed.face_to_cell.swap(0, 1);
        assert_ne!(
            parts["ordered_face_to_original_cell_ids"],
            hero_registration_components(&changed)["ordered_face_to_original_cell_ids"]
        );
        let mut changed = input.clone();
        changed.cells[0].source_masks[1] = true;
        assert_ne!(
            parts["original_source_mask_bits"],
            hero_registration_components(&changed)["original_source_mask_bits"]
        );
        assert!(changed.digest().is_err()); // a masked source cannot remain a contributor.
        assert!(crate::hero_appearance::RegisteredCellColors::bind(
            input.clone(),
            b"wrong D1",
            &input.digest().unwrap()
        )
        .is_err());
        assert!(crate::hero_appearance::RegisteredCellColors::bind(
            input.clone(),
            b"fixture D1",
            &"f".repeat(64)
        )
        .is_err());
    }

    #[test]
    fn hero_production_pins_refuse_null_partial_camera_and_assignment_changes() {
        let h = sha256_bytes(b"neutral observed bytes");
        let observed = json!({"schema":"jarambit.hero_appearance_expected_pins.v1",
            "source_sha256":{"neutral-source":h},"ordered_decoded_geometry_sha256":h,
            "camera_sha256":h,"registered_assignment_sha256":h,
            "component_sha256":{"registered_index_bits":h,"original_source_mask_bits":h,
                "contributor_ids_and_counts":h,"unlit_cell_palette_rgb":h,"ordered_face_to_original_cell_ids":h},
            "source_faces":3,"source_cells":2,"channel_ids":["a","b","c","d"]});
        assert!(hero_require_expected_pins(&observed, &observed).is_ok()); // neutral control only, not production pin admission.
        assert!(hero_require_expected_pins(&Value::Null, &observed).is_err());
        for pointer in [
            "/source_sha256/neutral-source",
            "/ordered_decoded_geometry_sha256",
            "/camera_sha256",
            "/registered_assignment_sha256",
            "/component_sha256/registered_index_bits",
            "/component_sha256/original_source_mask_bits",
            "/component_sha256/contributor_ids_and_counts",
            "/component_sha256/unlit_cell_palette_rgb",
            "/component_sha256/ordered_face_to_original_cell_ids",
        ] {
            let mut missing = observed.clone();
            *missing.pointer_mut(pointer).unwrap() = Value::Null;
            assert!(
                hero_require_expected_pins(&missing, &observed).is_err(),
                "{pointer}"
            );
            let mut wrong = observed.clone();
            *wrong.pointer_mut(pointer).unwrap() = json!("f".repeat(64));
            assert!(
                hero_require_expected_pins(&wrong, &observed).is_err(),
                "{pointer}"
            );
        }
        let mut partial = observed.clone();
        partial.as_object_mut().unwrap().remove("source_faces");
        assert!(hero_require_expected_pins(&partial, &observed).is_err());
        let camera = crate::appearance::Camera {
            rotation: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            scale: 2.,
            offset: [3., 4.],
        };
        let changed = crate::appearance::Camera {
            offset: [3., 4.001],
            ..camera
        };
        assert_ne!(camera.digest(), changed.digest());
    }

    fn reviewed_palette() -> crate::display::Palette {
        let contract: Value = serde_json::from_str(include_str!(
            "../../../docs/paper/data/display-composite-contract.v1.json"
        ))
        .unwrap();
        crate::display::Palette::admit(&contract["palette"]).unwrap()
    }

    #[test]
    fn hero_multihue_palette_has_reviewed_endpoints_and_interpolation() {
        let palette = reviewed_palette();
        assert_eq!(palette.rgb(0.0), [8, 28, 74]);
        assert_eq!(palette.rgb(0.25), [13, 148, 136]);
        assert_eq!(palette.rgb(0.5), [250, 204, 21]);
        assert_eq!(palette.rgb(0.75), [234, 88, 12]);
        assert_eq!(palette.rgb(1.0), [153, 27, 27]);
        assert_eq!(palette.rgb(-1.0), [8, 28, 74]);
        assert_eq!(palette.rgb(2.0), [153, 27, 27]);
        assert_eq!(palette.rgb(0.125), [11, 88, 105]);
    }

    #[test]
    fn hero_multihue_palette_does_not_claim_monotone_luminance() {
        let luminance = |rgb: [u8; 3]| {
            let linear = rgb.map(|channel| {
                let value = f64::from(channel) / 255.0;
                if value <= 0.04045 {
                    value / 12.92
                } else {
                    ((value + 0.055) / 1.055).powf(2.4)
                }
            });
            0.2126 * linear[0] + 0.7152 * linear[1] + 0.0722 * linear[2]
        };
        let palette = reviewed_palette();
        assert!(luminance(palette.rgb(0.5)) > luminance(palette.rgb(1.0)));
        assert!(luminance(palette.rgb(0.25)) > luminance(palette.rgb(0.0)));
    }

    #[test]
    fn point_raster_oracle_retains_extrema_and_counts_overlapping_operations() {
        // Explicit independent 5x5 oracle: corners and origin, with one overlap.
        let mut raster =
            PointRaster::new(5, 5, [0.0, 4.0], [-2.0_f64.sinh(), 2.0_f64.sinh()]).unwrap();
        assert_eq!(raster.paint(0.0, -2.0_f64.sinh()).unwrap(), [0, 4]);
        assert_eq!(raster.paint(4.0, 2.0_f64.sinh()).unwrap(), [4, 0]);
        assert_eq!(raster.paint(2.0, 0.0).unwrap(), [2, 2]);
        assert_eq!(raster.paint(2.0, 0.0).unwrap(), [2, 2]);
        let mut expected = vec![0u8; 25];
        expected[20] = 1;
        expected[4] = 1;
        expected[12] = 1;
        assert_eq!(raster.pixels, expected);
        let receipt = raster.receipt().unwrap();
        assert_eq!(receipt["point_operations"], 4);
        assert_eq!(receipt["occupied_pixels"], 3);
        assert_eq!(receipt["overlap_operations"], 1);
        assert!(raster.paint(-0.01, 0.0).is_err());
        assert!(raster.paint(2.0, 2.0_f64.sinh() + 0.01).is_err());
        assert!(raster.paint(2.0, f64::NAN).is_err());
        assert_eq!(raster.point_operations, 4);
    }

    #[test]
    fn point_raster_uniform_domain_has_a_real_center_and_refuses_other_values() {
        let mut raster = PointRaster::new(5, 5, [7.0, 7.0], [-3.0, -3.0]).unwrap();
        assert_eq!(raster.paint(7.0, -3.0).unwrap(), [2, 2]);
        assert!(raster.paint(7.0, -2.99).is_err());
        assert!(PointRaster::new(5, 5, [1.0, 0.0], [0.0, 1.0]).is_err());
    }

    #[test]
    fn off_center_asinh_mapping_is_distinct_from_linear_mapping_and_png_roundtrips() {
        // sinh(2) and sinh(0.8), supplied numerically so the expected mapping is independent.
        let mut raster =
            PointRaster::new(9, 9, [0., 8.], [-3.626860407847019, 3.626860407847019]).unwrap();
        assert_eq!(raster.paint(1., 0.888105982187623).unwrap(), [1, 2]);
        // Linear y mapping would place this point on row3, not row2.
        let encoded = encode_point_png_b64(raster.width, raster.height, &raster.pixels);
        let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut png = Vec::new();
        for block in encoded.as_bytes().chunks(4) {
            let mut packed = 0u32;
            for &byte in block {
                packed = (packed << 6)
                    | if byte == b'=' {
                        0
                    } else {
                        alphabet.iter().position(|&v| v == byte).unwrap() as u32
                    };
            }
            png.push((packed >> 16) as u8);
            if block[2] != b'=' {
                png.push((packed >> 8) as u8);
            }
            if block[3] != b'=' {
                png.push(packed as u8);
            }
        }
        assert_eq!(&png[..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
        let mut cursor = 8usize;
        let mut zlib = Vec::new();
        while cursor < png.len() {
            let len = u32::from_be_bytes(png[cursor..cursor + 4].try_into().unwrap()) as usize;
            let tag = &png[cursor + 4..cursor + 8];
            let data = &png[cursor + 8..cursor + 8 + len];
            if tag == b"IHDR" {
                assert_eq!(&data[8..10], &[4, 3]);
            }
            if tag == b"IDAT" {
                zlib.extend_from_slice(data);
            }
            cursor += 12 + len;
        }
        let mut raw = Vec::new();
        cursor = 2;
        loop {
            let last = zlib[cursor] & 1 != 0;
            assert_eq!(zlib[cursor] & 6, 0); // Stored DEFLATE block.
            let len = u16::from_le_bytes(zlib[cursor + 1..cursor + 3].try_into().unwrap());
            let complement = u16::from_le_bytes(zlib[cursor + 3..cursor + 5].try_into().unwrap());
            assert_eq!(len, !complement);
            cursor += 5;
            raw.extend_from_slice(&zlib[cursor..cursor + len as usize]);
            cursor += len as usize;
            if last {
                break;
            }
        }
        let mut pixels = Vec::new();
        for row in raw.chunks_exact(1 + raster.width.div_ceil(2)) {
            assert_eq!(row[0], 0);
            for x in 0..raster.width {
                pixels.push(if x % 2 == 0 {
                    row[1 + x / 2] >> 4
                } else {
                    row[1 + x / 2] & 15
                });
            }
        }
        assert_eq!(pixels, raster.pixels);
        assert_eq!(pixels.iter().filter(|&&v| v != 0).count(), 1);
        assert_eq!(pixels[2 * 9 + 1], 1);
    }

    #[test]
    fn admitted_source_callback_paints_every_run_scoped_original_record() {
        let (archive, index_sha) = crate::corpus::tests::build_minimal_801gen_archive(0);
        let mut raster = PointRaster::new(801, 161, [0., 800.], [1.05, 1.13]).unwrap();
        let mut identities = BTreeSet::new();
        let mut on_row =
            |run: usize, id: u64, generation: u64, fit: f64, _legal: bool| -> Result<()> {
                assert!(identities.insert((run, id)));
                raster.paint(generation as f64, fit)?;
                Ok(())
            };
        let admission = crate::corpus::verify_and_decode_gen800_archive_with_callback(
            &archive.0,
            Some(&index_sha),
            None,
            Some(&mut on_row),
        )
        .unwrap();
        assert_eq!(admission["rows_decoded"], 8010);
        assert_eq!(identities.len(), 8010);
        assert_eq!(raster.point_operations, 8010);
        assert_eq!(raster.overlap_operations, 7209);
        assert_eq!(raster.pixels.iter().filter(|&&v| v != 0).count(), 801);
    }

    #[test]
    fn historical_jevl_status_adapter_maps_notevaluated_only() {
        assert_eq!(
            adapt_historical_jevl_fem_status("complete").unwrap(),
            "complete"
        );
        assert_eq!(
            adapt_historical_jevl_fem_status("notevaluated").unwrap(),
            "not_evaluated"
        );
        assert!(adapt_historical_jevl_fem_status("not_evaluated").is_err());
        assert!(adapt_historical_jevl_fem_status("unknown").is_err());
    }

    #[test]
    fn svg_renderers_satisfy_sc183w_sc183z_and_sc183ab_invariants() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let mut out = BTreeMap::new();
        let mut inputs = BTreeMap::new();
        assert!(generate_v2_outputs(&root, &mut out, &mut inputs)
            .unwrap_err()
            .to_string()
            .contains("missing admitted weighted-summary projection"));
        let weighted_contract: Value = serde_json::from_slice(
            &fs::read(root.join("docs/paper/weighted-inputs.v1.json")).unwrap(),
        )
        .unwrap();
        let weighted_docs = weighted_contract["runs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|run| {
                let bytes = fs::read(root.join(run["path"].as_str().unwrap())).unwrap();
                assert_eq!(json!(sha256_bytes(&bytes)), run["sha256"]);
                (
                    run["id"].as_str().unwrap().to_string(),
                    serde_json::from_slice(&bytes).unwrap(),
                )
            })
            .collect();
        let weighted = crate::weighted_runs::project(&weighted_docs, &weighted_contract).unwrap();
        out.insert(
            "docs/paper/generated/weighted-runs.json".into(),
            serde_json::to_string_pretty(&weighted).unwrap() + "\n",
        );
        generate_v2_outputs(&root, &mut out, &mut inputs).unwrap();

        let mem_svg = &out["docs/paper/figures/population-membership.svg"];
        assert!(mem_svg.contains("Island 1 (32 slots)"));
        assert!(mem_svg.contains("Island 4 (32 slots)"));
        assert!(!mem_svg.contains("Island 0 (32 slots)"));

        let lat_svg = &out["docs/paper/figures/population-latest.svg"];
        // Simplified visible keys retain the original source/count invariants
        // in the numeric receipt, without restoring administrative plot text.
        let points: Value =
            serde_json::from_str(&out["docs/paper/generated/population/point-rendering.v1.json"])
                .unwrap();
        assert_eq!(points["native"]["legal_true_with_negative_fit"], 610_051);
        assert_eq!(points["native"]["legal_true_rows"], 754_236);
        assert_eq!(points["native"]["raster"]["point_operations"], 1_954_560);
        assert_eq!(points["numeric_point_operations"], 1_958_803);
        assert_eq!(points["excluded_numeric_records"], 0);
        assert_eq!(points["unknown"]["records"], 1390);
        assert_eq!(points["unknown"]["numeric_point_operations"], 0);
        assert_eq!(
            points["native"]["source_visit_sha256"],
            "f78dc1ba20301f50d47c3034aeb1e3d65c9f0e863d942ed047fb6d4ebc710dbf"
        );
        for field in ["clipping", "jitter", "majority_class_replacement"] {
            assert_eq!(points["native"]["raster"][field], false);
        }
        assert!(lat_svg.contains(r#"data-numeric-scored-points="1954560""#));
        assert!(lat_svg.contains(r#"data-unknown-proposal-count="1390""#));
        assert!(!lat_svg.contains("severe penalty"));
        assert!(!lat_svg.contains("multi-gate pen."));

        // The actual visible labels must disclose each source-derived transform,
        // not merely retain it in machine metadata or resurrect removed prose.
        let detail_svg = &out["docs/paper/generated/population-detail.svg"];
        let axes: Value =
            serde_json::from_str(&out["docs/paper/generated/population/score-axis.v1.json"])
                .unwrap();
        for (profile, threshold) in [("native", 0.4054_f64), ("historical", 0.29066_f64)] {
            assert_eq!(axes[profile]["transform"], "asinh(y/L)");
            assert_eq!(axes[profile]["linear_threshold_L"], json!(threshold));
            let visible = format!("asinh(y/L), L={threshold:.5}</text>");
            for svg in [lat_svg, detail_svg] {
                assert!(svg.contains(&visible));
            }
        }
        assert_eq!(axes["screening"]["transform"], "linear");
        for svg in [lat_svg, detail_svg] {
            assert!(svg.contains("Screen deficit (lower is better); linear</text>"));
            assert!(svg.contains("1,390 proposals have no retained numeric screen value; no score is imputed.</text>"));
        }
        for profile in ["native", "historical", "screening"] {
            for value in points[profile]["raster"]["score_display_domain"]
                .as_array()
                .unwrap()
            {
                assert!(lat_svg.contains(&format!(">{:.4}</text>", value.as_f64().unwrap())));
            }
        }
        assert!(detail_svg.contains("High-Fitness / Low-Deficit Tail Detail — Full Range Retained</text>"));
        for (profile, numeric_universe, selection, extremum, forbidden_tail) in [
            ("native", 1_954_560u64, "Q90..MAX", 1.144, 2usize),
            ("historical", 1_984, "Q90..MAX", 1.15727, 2),
            ("screening", 2_259, "MIN..Q10", 2.769032384488388, 0),
        ] {
            let detail = &points["detail"][profile];
            let tails: Vec<_> = detail["below_inside_above"].as_array().unwrap()
                .iter().map(|count| count.as_u64().unwrap()).collect();
            assert_eq!(tails.len(), 3);
            assert_eq!(tails.iter().sum::<u64>(), numeric_universe);
            assert_eq!(detail["numeric_universe"], numeric_universe);
            assert_eq!(points[profile]["raster"]["point_operations"], numeric_universe);
            assert_eq!(tails[forbidden_tail], 0);
            assert!(tails[1] > 0);
            assert_eq!(detail["raster"]["point_operations"], tails[1]);
            assert_eq!(detail["selection"], selection);
            assert_eq!(detail["extremum"]["value"], json!(extremum));
            assert_eq!(detail["extremum"]["source_tie_count"], detail["extremum"]["included_tie_count"]);
            assert!(detail["extremum"]["included_tie_count"].as_u64().unwrap() > 0);
            assert_eq!(detail["cutoff_ties"]["source_count"], detail["cutoff_ties"]["included_count"]);
            assert_eq!(detail["display_axis_widened"], false);
            assert_eq!(detail["bounds"], detail["raster"]["observed_score_extrema"]);
            for binding in ["all_rows_membership_sha256", "selected_identity_score_sha256"] {
                assert_eq!(detail[binding].as_str().unwrap().len(), 64);
            }
            assert!(detail_svg.contains(&format!(
                "{selection}: [{:.4}, {:.4}]; inside {} (all cutoff ties).</text>",
                detail["bounds"][0].as_f64().unwrap(), detail["bounds"][1].as_f64().unwrap(), tails[1]
            )));
            assert!(detail_svg.contains(&format!(
                "Below {}; above {}; retained extreme {:.4}; {} tied rows.</text>",
                tails[0], tails[2], extremum, detail["extremum"]["source_tie_count"].as_u64().unwrap()
            )));
            for value in detail["bounds"].as_array().unwrap() {
                assert!(detail_svg.contains(&format!(">{:.4}</text>", value.as_f64().unwrap())));
            }
        }
        // Independent original source namespaces and known full-range extrema
        // must survive the additional detail selection.
        assert_eq!(points["native"]["runs"].as_array().unwrap().len(), 10);
        assert_eq!(points["historical"]["source_path"], HISTORICAL_ALL_EVALS_JSON_RELPATH);
        assert_eq!(points["historical"]["source_sha256"], HISTORICAL_ALL_EVALS_JSON_SHA256);
        assert_eq!(points["screening"]["source_pairs"].as_array().unwrap().len(), 57);
        assert_eq!(points["detail"]["native"]["extremum"]["included_tie_count"], 7);
        let run_context = points["run_context"].as_array().unwrap();
        assert_eq!(run_context.len(), 10);
        for pair in run_context.chunks_exact(2) {
            let key = format!(
                "docs/paper/generated/native-runs-{}-{}.svg",
                pair[0]["public_run_id"].as_u64().unwrap(),
                pair[1]["public_run_id"].as_u64().unwrap()
            );
            for run in pair {
                let visible = match run["axis"]["transform"].as_str().unwrap() {
                    "asinh(y/L)" => {
                        let threshold = run["axis"]["linear_threshold_L"].as_f64().unwrap();
                        assert!(threshold.is_finite() && threshold > 0.);
                        format!("asinh(y/L), L={threshold:.5}</text>")
                    }
                    "linear" => "linear</text>".to_string(),
                    other => panic!("unexpected run axis transform: {other}"),
                };
                assert!(out[&key].contains(&visible));
                assert_eq!(run["raster"]["point_operations"], 195456);
            }
        }

        let fit_svg = &out["docs/paper/figures/fitness-progress.svg"];
        // SC-DEC-183Z Panel B exact source-derived points and SHA-256 pointer checks
        assert!(fit_svg.contains(W411C3_FITNESS_CURVE_SHA256));
        assert!(fit_svg.contains(
            r#"data-source-pointer="/generations/0/hall_of_fame_best_so_far" data-generation="0" data-value="74.167488""#
        ));
        assert!(fit_svg.contains(
            r#"data-source-pointer="/generations/1/hall_of_fame_best_so_far" data-generation="1" data-value="74.207299""#
        ));
        assert!(fit_svg.contains(
            r#"data-source-pointer="/generations/2/hall_of_fame_best_so_far" data-generation="2" data-value="74.207299""#
        ));
        assert!(fit_svg.contains(
            r#"data-source-pointer="/generations/0/round_best_fitness" data-generation="0" data-value="74.167488""#
        ));
        assert!(fit_svg.contains(
            r#"data-source-pointer="/generations/1/round_best_fitness" data-generation="1" data-value="74.207299""#
        ));
        assert!(fit_svg.contains(
            r#"data-source-pointer="/generations/2/round_best_fitness" data-generation="2" data-value="73.608446""#
        ));
        assert!(fit_svg.contains("• Gen 0 (Ind 2009): archive 74.167488 / round 74.167488"));
        assert!(fit_svg.contains("• Gen 1 (Ind 2002): archive 74.207299 / round 74.207299"));
        assert!(fit_svg.contains("• Gen 2 (Ind 2002): archive 74.207299 / round 73.608446"));
        assert!(fit_svg.contains("• Stage-2 3D solid/plate: unavailable retained assessment"));
        assert!(fit_svg.contains("Cohort 2 Selected Run 31 Summary (Retained Profile)"));
        assert!(fit_svg.contains(
            "legal medians (G0: 5/10 -0.2620..-0.2070; min -0.5782; G800: -0.2610..-0.1933)"
        ));

        let inh_svg = &out["docs/paper/figures/inheritance.svg"];
        assert!(inh_svg.contains("Individual 413 (Gen 3) [reused]"));
        assert!(inh_svg.contains("Individual 526 (Gen 4)"));
        let inh_details = &out["docs/paper/figures/inheritance-details.svg"];
        assert!(inh_details.contains("Individual 413 (Gen 3, +1.15296) [reused]"));
        assert!(inh_details.contains("Individual 526 (Gen 4, +1.15563)"));
        assert!(inh_svg.contains("Archive mutation"));
        assert!(inh_svg.contains("Crossover + mutation"));
        assert!(inh_svg.contains(r#"id="edge-413-526""#));
        assert!(inh_svg.contains(r#"id="edge-392-526""#));
        assert!(inh_svg.contains(r#"id="edge-174-299-4th-hop""#));
        assert!(!inh_svg.contains(r#"id="edge-392-413""#));

        // Ensure erroneous hardcoded numbers, 'unrun', and cryptic internal aliases are absent
        assert!(!fit_svg.contains("73.9218"));
        assert!(!fit_svg.contains("73.8541"));
        assert!(!fit_svg.contains("unrun"));
        assert!(!fit_svg.contains("w411c3"));
        assert!(!fit_svg.contains("w282_p14_c01"));
        assert!(!fit_svg.contains("crescent"));
        assert!(!fit_svg.contains("Selected Individual 2001"));
        assert!(!fit_svg.contains("Controlled probe Individual 1985"));
        assert!(!fit_svg.contains("(~0.94)"));
    }
}
