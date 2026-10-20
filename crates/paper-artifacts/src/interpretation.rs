//! Additive retained-record interpretation. This reader never executes producers.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const CONTRACT: &str = "docs/paper/data/reporting-interpretation-contract.v2.json";
fn text(v: &Value) -> Result<&str> {
    v.as_str()
        .ok_or_else(|| "missing interpretation string".into())
}
fn array(v: &Value) -> Result<&Vec<Value>> {
    v.as_array()
        .ok_or_else(|| "missing interpretation array".into())
}
fn digest(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn read(root: &Path, path: &str) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(
        crate::corpus::resolve_safe_relpath(root, path)?,
    )?)?)
}

fn occurrence(birth: &Value, expected: &Value, registry: &Value) -> Result<Value> {
    let index = expected["one_based_candidate_index"]
        .as_u64()
        .ok_or("missing proposal index")?;
    if index == 0 {
        return Err("proposal indices are one based".into());
    }
    let row = array(&birth["genomes"])?
        .get(index as usize - 1)
        .ok_or("proposal row absent")?;
    let csv = format!(
        "{}\n{}\n",
        text(&birth["genome_header"])?,
        text(&row["row"])?
    );
    let hash = digest(csv.as_bytes());
    let cert = text(&row["certificate_identity"])?;
    if hash != text(&expected["reconstructed_genome_sha256"])?
        || cert != text(&expected["certificate_identity"])?
        || !row["parents"].is_null()
    {
        return Err("proposal content/certificate or parentage contract differs".into());
    }
    let alias = expected["public_alias"]
        .as_u64()
        .ok_or("missing public alias")?;
    let original = array(&registry["allocated_individuals"])?
        .iter()
        .find(|r| r["public_id"].as_u64() == Some(alias))
        .ok_or("unknown illustrated alias")?;
    for (key, value) in [
        ("genome_sha256", hash.as_str()),
        ("certificate_identity", cert),
    ] {
        if !original[key].is_null() && original[key].as_str() != Some(value) {
            return Err("proposal differs from existing content identity".into());
        }
    }
    let content_alias = if alias == 2009 {
        let other = array(&registry["allocated_individuals"])?
            .iter()
            .find(|r| r["public_id"] == 2003)
            .ok_or("missing existing content alias")?;
        if other["genome_sha256"] != hash || other["certificate_identity"] != cert {
            return Err("distinct observation content alias differs".into());
        }
        json!(2003)
    } else {
        Value::Null
    };
    Ok(
        json!({"illustrated_alias":alias,"proposal_observation_alias":expected["derived_proposal_alias_under_existing_range_rule"],
        "births_path":expected["births_path"],"selector":expected["selector"],
        "genome_sha256":hash,"certificate_identity":cert,"parent_ids":null,
        "relation":"exact content and certificate; separate recorded observations, not parentage",
        "equal_content_other_alias":content_alias,"merge_birth_events":false}),
    )
}

fn historical_cases(births: &Value, csv: &str, expected: &Value) -> Result<Value> {
    // The admitted source is an unquoted numeric/Boolean table. Refuse other dialects.
    if csv.contains('"') {
        return Err("unadmitted certification CSV quoting".into());
    }
    let mut lines = csv.lines();
    let header = lines
        .next()
        .ok_or("missing certification header")?
        .split(',')
        .collect::<Vec<_>>();
    let id_col = header
        .iter()
        .position(|k| *k == "id")
        .ok_or("missing original id column")?;
    let pass_col = header
        .iter()
        .position(|k| *k == "cert_pass_all")
        .ok_or("missing earlier predicate")?;
    let mut predicates = BTreeMap::new();
    for line in lines.filter(|line| !line.is_empty()) {
        let cols = line.split(',').collect::<Vec<_>>();
        if cols.len() != header.len() {
            return Err("certification column count differs".into());
        }
        let id: u64 = cols[id_col].parse()?;
        let pass = match cols[pass_col] {
            "true" => true,
            "false" => false,
            _ => return Err("predicate is not Boolean".into()),
        };
        if predicates.insert(id, pass).is_some() {
            return Err("duplicate certification original id".into());
        }
    }
    let rows = array(births)?;
    if rows.len() != 1984 {
        return Err("historical birth denominator differs".into());
    }
    let mut cases = Vec::new();
    for e in array(expected)? {
        let id = e["original_id"]
            .as_u64()
            .ok_or("missing original birth id")?;
        let matches = rows
            .iter()
            .filter(|r| r["id"].as_u64() == Some(id))
            .collect::<Vec<_>>();
        if matches.len() != 1 {
            return Err("nonunique historical birth id".into());
        }
        let fit = matches[0]["fit"]
            .as_f64()
            .filter(|v| v.is_finite())
            .ok_or("missing stored birth fit")?;
        let rank = 1 + rows
            .iter()
            .filter(|r| r["fit"].as_f64().is_some_and(|f| f > fit))
            .count();
        let pass = *predicates
            .get(&id)
            .ok_or("birth lacks earlier certification row")?;
        if e["birth_fit"].as_f64() != Some(fit)
            || e["rank_competition"].as_u64() != Some(rank as u64)
            || e["cert_pass_all"].as_str() != Some(if pass { "true" } else { "false" })
        {
            return Err("historical predicate comparison differs".into());
        }
        cases.push(json!({"original_id":id,"stored_birth_fit":fit,"birth_rank":rank,
            "earlier_cert_pass_all":pass,"predicate_scope":"this run's earlier Boolean predicate; not later 26-requirement qualification"}));
    }
    Ok(json!(cases))
}

fn current_claims(c: &Value) -> Result<()> {
    if c["schema"] != "jarambit.reporting_interpretation_contract.v2"
        || c["original_contracts_remain_unchanged"] != true
        || c.pointer("/buckling/admitted_engineering_value") != Some(&Value::Null)
        || c.pointer("/buckling/current_qualification") != Some(&Value::Null)
        || c["buckling"]["qualification_state"] != "unestablished_due_to_unqualified_operator"
        || c["runtime"]["state"] != "retained_capture_not_exact_executed_environment"
        || c.pointer("/runtime/original_solve_runtime_hash_binding") != Some(&Value::Null)
        || c["weighted_recipe"]["status"]
            != "recovered_compatible_recipe_not_proven_original_execution"
        || c.pointer("/weighted_recipe/executed_source_binding") != Some(&Value::Null)
        || c.pointer("/weighted_recipe/imputed_points_claim") != Some(&Value::Null)
        || c["weighted_recipe"]["sha256"] != c["sources"]["recovered_recipe"]["sha256"]
        || c["ring_mask"]["predicate"] != "centroid.x >= sx0 - 3.0"
        || c["ring_mask"]["masked_cell_count"] != 14875
        || c["ring_mask"]["unmasked_cell_count"] != 10334
    {
        return Err("unadmitted current interpretation or invented execution binding".into());
    }
    Ok(())
}

pub(crate) fn project(root: &Path, inputs: &mut BTreeMap<String, String>) -> Result<Value> {
    let c = read(root, CONTRACT)?;
    current_claims(&c)?;
    inputs.insert(CONTRACT.into(), digest(&fs::read(root.join(CONTRACT))?));
    let source_path = "crates/paper-artifacts/src/interpretation.rs";
    inputs.insert(
        source_path.into(),
        digest(&fs::read(root.join(source_path))?),
    );
    let bindings = c["sources"]
        .as_object()
        .ok_or("missing interpretation sources")?;
    for b in bindings.values() {
        let path = text(&b["path"])?;
        let actual = digest(&fs::read(crate::corpus::resolve_safe_relpath(root, path)?)?);
        if actual != text(&b["sha256"])? {
            return Err(format!("interpretation source hash differs: {path}").into());
        }
        inputs.insert(path.into(), actual);
    }
    let source = |key: &str| -> Result<Value> { read(root, text(&c["sources"][key]["path"])?) };
    let ids = source("public_ids")?;
    let join = source("proposal_join")?;
    let mut files = array(&join["file_pairs"])?.iter().collect::<Vec<_>>();
    files.sort_by_key(|r| r["births_path"].as_str().unwrap_or(""));
    let mut occurrences = Vec::new();
    for e in array(&c["proposal_occurrences"])? {
        let mut alias = 3000u64;
        let mut found = false;
        for f in &files {
            if f["births_path"] == e["births_path"] {
                alias += e["one_based_candidate_index"]
                    .as_u64()
                    .ok_or("missing index")?
                    .checked_sub(1)
                    .ok_or("proposal indices are one based")?;
                found = true;
                break;
            }
            alias += f["births_count"].as_u64().ok_or("missing birth count")?;
        }
        if !found || e["derived_proposal_alias_under_existing_range_rule"].as_u64() != Some(alias) {
            return Err("proposal observation alias is not the original range allocation".into());
        }
        let key = format!(
            "proposal_{}",
            e["public_alias"].as_u64().ok_or("missing alias")?
        );
        if c["sources"][&key]["path"] != e["births_path"] {
            return Err("proposal binding path differs".into());
        }
        occurrences.push(occurrence(&source(&key)?, e, &ids)?);
    }
    let cert_csv = fs::read_to_string(crate::corpus::resolve_safe_relpath(
        root,
        text(&c["sources"]["historical_predicates"]["path"])?,
    )?)?;
    let historical = historical_cases(
        &source("historical_births")?,
        &cert_csv,
        &c["historical_predicate_cases"],
    )?;
    let cert = text(&c["proposal_occurrences"][1]["certificate_identity"])?;
    let summary = source("candidate_summary")?;
    let later = array(&summary["designs"])?
        .iter()
        .filter(|r| r["certificate_identity"] == cert)
        .collect::<Vec<_>>();
    if later.len() != 1 || later[0]["w_cad"] != -7.414831 || later[0]["qualified"] != false {
        return Err("candidate 2002 later summary differs".into());
    }
    let curve = source("weighted_summary")?;
    let weighted = &curve["best_stage2_26_gate_design"];
    let objectives = weighted["ultralight_fitness_breakdown"]["objectives"]
        .as_object()
        .ok_or("missing saved objectives")?;
    if weighted["certificate_identity"] != cert
        || objectives.len() != 20
        || weighted["ultralight_fitness_breakdown"]["global_weighted_fitness"] != 74.207299
    {
        return Err("retained weighted vector differs".into());
    }
    let fallback = c["weighted_recipe"]["four_entire_partly_fallback_compatible_terms"]
        .as_object()
        .ok_or("missing default-compatible terms")?;
    let mut whole_sum = 0.;
    let mut term_rows = Vec::new();
    for (key, saved) in objectives {
        let class = if let Some(expected) = fallback.get(key) {
            if saved != expected {
                return Err("stored partly fallback-compatible term differs".into());
            }
            whole_sum += saved["weighted_points"]
                .as_f64()
                .ok_or("missing retained contribution")?;
            "partly_fallback_compatible_not_proven_imputed"
        } else if [
            "cad_watertight_brep",
            "c6_aesthetic_and_ergo",
            "karambit_ring_and_claw",
            "plate_gci_3level",
        ]
        .contains(&key.as_str())
        {
            "historical_binary_or_proxy_term_not_physical_measurement"
        } else {
            "retained_summary_term_original_call_input_provenance_not_established"
        };
        term_rows.push(json!({"objective":key,"saved":saved,"source_class":class,
            "source_selector":format!("$.best_stage2_26_gate_design.ultralight_fitness_breakdown.objectives.{key}"),
            "new_scoring_or_measurement":false}));
    }
    if (whole_sum - 15.682353f64).abs() > 0.000001 {
        return Err("whole-term sum differs".into());
    }
    let runtime = source("runtime")?;
    for (key, version) in [
        ("numpy", "2.5.4"),
        ("pyamg", "5.3.0"),
        ("scipy", "1.18.1"),
        ("skfem", "12.0.2"),
    ] {
        if runtime["versions"][key] != version {
            return Err("runtime capture version differs".into());
        }
    }
    if !text(&runtime["versions"]["python"])?.starts_with("3.12.3 ") {
        return Err("retained Python capture version differs".into());
    }
    let a4 = source("a4")?;
    if a4["buckling_eigenvalue"] != 7.0350618718604325 || a4["status"] != "pass" {
        return Err("historical A4 observation differs".into());
    }
    let index = source("native_index")?;
    let native_runs = array(&index["runs"])?;
    let missing_configs = native_runs
        .iter()
        .filter(|r| r["decoder_and_schema_binding"]["proc_config_sha256"].is_null())
        .count();
    if native_runs.len() != 10
        || missing_configs != 5
        || c["native_provenance"]["missing_process_config_bindings"].as_u64()
            != Some(missing_configs as u64)
    {
        return Err("native run denominator differs".into());
    }
    Ok(
        json!({"schema":"jarambit.reporting_interpretation_projection.v2","contract":CONTRACT,
        "sources":c["sources"],"proposal_occurrences":occurrences,"historical_predicate_cases":historical,
        "candidate_2002_later_summary":{"observed":later[0],"state":"retained_later_summary_not_complete_underlying_artifacts",
            "full_exported_CAD_available":false,"full_scorecard_available":false,"missing_is_not_measured_zero":true},
        "weighted_recipe":{"binding":c["weighted_recipe"],"saved_F":74.207299,"terms":term_rows,
            "four_entire_partly_fallback_compatible_terms_sum":whole_sum,"sum_is_not_imputed_points":true,
            "source_presence_is_not_proven_original_execution":true},
        "runtime":{"captured":runtime,"interpretation":c["runtime"],"exact_executed_environment_proven":false},
        "buckling":{"historical_producer_reported_value":a4["buckling_eigenvalue"],"historical_producer_reported_status":a4["status"],
            "admitted_engineering_value":null,"current_qualification":null,"interpretation":c["buckling"]},
        "ring_mask":c["ring_mask"],"native_provenance":{"index_run_bindings":native_runs,"T06_producer_claim":source("native_admission")?["producer_and_config_scope"],
            "interpretation":c["native_provenance"],"exact_executed_binary_proven":false},
        "original_evidence_score_weights_thresholds_masks_and_geometry_changed":false,
        "producer_scorer_solver_or_search_executed":false}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn proposal_content_and_certificate_do_not_create_parents_or_merge_events() {
        let b = json!({"genome_header":"x","genomes":[{"row":"1","certificate_identity":"cert"}]});
        let hash = digest(b"x\n1\n");
        let e = json!({"public_alias":2000,"one_based_candidate_index":1,"reconstructed_genome_sha256":hash,"certificate_identity":"cert"});
        let registry = json!({"allocated_individuals":[{"public_id":2000,"genome_sha256":hash,"certificate_identity":"cert"}]});
        let p = occurrence(&b, &e, &registry).unwrap();
        assert!(p["parent_ids"].is_null());
        assert_eq!(p["merge_birth_events"], false);
        let mut bad = b.clone();
        bad["genomes"][0]["certificate_identity"] = json!("other");
        assert!(occurrence(&bad, &e, &registry).is_err());
        bad = b.clone();
        bad["genomes"][0]["row"] = json!("2");
        assert!(occurrence(&bad, &e, &registry).is_err());
        bad = b;
        bad["genomes"][0]["parents"] = json!([1, 2]);
        assert!(occurrence(&bad, &e, &registry).is_err());
    }
    #[test]
    fn actual_retained_interpretation_preserves_missingness_and_history() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let p = project(&root, &mut BTreeMap::new()).unwrap();
        assert_eq!(
            p["proposal_occurrences"][0]["proposal_observation_alias"],
            6643
        );
        assert_eq!(
            p["proposal_occurrences"][1]["proposal_observation_alias"],
            6420
        );
        assert_eq!(
            p["candidate_2002_later_summary"]["observed"]["qualified"],
            false
        );
        assert_eq!(p["weighted_recipe"]["terms"].as_array().unwrap().len(), 20);
        assert!(p["buckling"]["admitted_engineering_value"].is_null());
        assert!(p["buckling"]["current_qualification"].is_null());
        assert_eq!(p["buckling"]["historical_producer_reported_status"], "pass");
        assert_eq!(p["runtime"]["exact_executed_environment_proven"], false);
        assert_eq!(p["producer_scorer_solver_or_search_executed"], false);
    }
    #[test]
    fn capture_recipe_and_historical_flags_cannot_assert_missing_execution_or_capacity() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contract = read(&root, CONTRACT).unwrap();
        current_claims(&contract).unwrap();
        for (pointer, invented) in [
            (
                "/buckling/admitted_engineering_value",
                json!(7.0350618718604325),
            ),
            ("/buckling/current_qualification", json!(true)),
            (
                "/runtime/original_solve_runtime_hash_binding",
                json!("captured_is_not_executed"),
            ),
            (
                "/weighted_recipe/executed_source_binding",
                json!("source_presence_is_not_execution"),
            ),
            ("/weighted_recipe/imputed_points_claim", json!(15.682353)),
            ("/ring_mask/masked_cell_count", json!(0)),
        ] {
            let mut bad = contract.clone();
            *bad.pointer_mut(pointer).unwrap() = invented;
            assert!(current_claims(&bad).is_err(), "{pointer}");
        }
        let mut omitted = contract;
        omitted["runtime"]
            .as_object_mut()
            .unwrap()
            .remove("original_solve_runtime_hash_binding");
        assert!(current_claims(&omitted).is_err());
    }
}
