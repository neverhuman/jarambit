//! Persistent integer public identity registry and selection-authority validation.
//!
//! Public integer IDs (`Individual N`, `Run M`, `Island K`) are append-only
//! publication labels separated from internal content hashes, repeated slot
//! appearances, and operational hostnames.
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub const PUBLIC_IDS_SCHEMA: &str = "jarambit.public_ids.v1";
pub const SELECTION_RECEIPTS_SCHEMA: &str = "jarambit.selection_receipts.v1";
pub const HISTORICAL_BIRTH_MAX_ID: u64 = 1983;
pub const APPEND_ONLY_INDIVIDUAL_FLOOR: u64 = 2000;

const ALLOWED_ORIGINS: &[&str] = &[
    "Birth",
    "ControlledProbe",
    "RetainedExport",
    "Reference",
    "Unknown",
];

const ALLOWED_AUTHORITIES: &[&str] = &["algorithm", "human", "agent", "mixed", "unknown"];

const ALLOWED_CLAIM_STATUSES: &[&str] = &[
    "demonstrated",
    "derived-reporting",
    "planned",
    "unknown",
    "not-established",
];

fn nonempty_str<'a>(v: &'a Value, ctx: &str) -> Result<&'a str> {
    v.as_str()
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| format!("identity: missing or empty {ctx}").into())
}

fn sha256_hex<'a>(v: &'a Value, ctx: &str) -> Result<&'a str> {
    let s = nonempty_str(v, ctx)?;
    if s.len() != 64 || !s.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(format!("identity: invalid sha256 hex for {ctx}").into());
    }
    Ok(s)
}

fn contains_worker_hostname(label: &str) -> bool {
    let lower = label.to_ascii_lowercase();
    ["xbabe1", "xbabe2", "xbabe3", "xbabe4"]
        .iter()
        .any(|host| lower.contains(host))
}

/// Validate `docs/paper/data/public_ids.v1.json` and return a normalized lookup summary.
pub fn validate_public_ids(registry: &Value) -> Result<Value> {
    if registry["schema"].as_str() != Some(PUBLIC_IDS_SCHEMA) {
        return Err("identity: unsupported public_ids schema".into());
    }
    let res = &registry["historical_reservation"];
    if res["run_public_id"].as_u64() != Some(1)
        || res["id_min"].as_u64() != Some(0)
        || res["id_max"].as_u64() != Some(HISTORICAL_BIRTH_MAX_ID)
        || res["count"].as_u64() != Some(HISTORICAL_BIRTH_MAX_ID + 1)
        || res["origin"].as_str() != Some("Birth")
    {
        return Err("identity: historical w282 reservation 0..1983 must stay intact".into());
    }
    sha256_hex(
        &res["source_all_evals_json_sha256"],
        "historical all_evals.json sha256",
    )?;

    let runs = registry["runs"]
        .as_array()
        .ok_or("identity: missing runs array")?;
    if runs.is_empty() {
        return Err("identity: runs array cannot be empty".into());
    }
    let mut run_ids = BTreeSet::new();
    let mut run_keys = BTreeSet::new();
    for run in runs {
        let public_run_id = run["public_run_id"]
            .as_u64()
            .ok_or("identity: missing public_run_id")?;
        if public_run_id == 0 || !run_ids.insert(public_run_id) {
            return Err("identity: duplicate or zero public_run_id".into());
        }
        let label = nonempty_str(&run["public_label"], "run public_label")?;
        if label != format!("Run {public_run_id}") || contains_worker_hostname(label) {
            return Err(
                "identity: public run label must be 'Run <N>' without worker hostnames".into(),
            );
        }
        let key = nonempty_str(&run["internal_run_key"], "internal_run_key")?;
        if !run_keys.insert(key.to_string()) {
            return Err("identity: duplicate internal_run_key".into());
        }
        nonempty_str(&run["cohort_family"], "run cohort_family")?;
    }

    let islands = registry["islands"]
        .as_array()
        .ok_or("identity: missing islands array")?;
    if islands.is_empty() {
        return Err("identity: islands array cannot be empty".into());
    }
    let mut island_ids = BTreeSet::new();
    for island in islands {
        let public_island_id = island["public_island_id"]
            .as_u64()
            .ok_or("identity: missing public_island_id")?;
        if public_island_id == 0 || !island_ids.insert(public_island_id) {
            return Err("identity: duplicate or zero public_island_id".into());
        }
        let label = nonempty_str(&island["public_label"], "island public_label")?;
        if label != format!("Island {public_island_id}") || contains_worker_hostname(label) {
            return Err("identity: public island label must be 'Island <N>'".into());
        }
    }

    let allocated = registry["allocated_individuals"]
        .as_array()
        .ok_or("identity: missing allocated_individuals")?;
    let mut ind_ids = BTreeSet::new();
    let mut aliases = BTreeMap::new();
    for item in allocated {
        let public_id = item["public_id"]
            .as_u64()
            .ok_or("identity: missing individual public_id")?;
        if public_id > HISTORICAL_BIRTH_MAX_ID && public_id < APPEND_ONLY_INDIVIDUAL_FLOOR {
            return Err("identity: unallocated gap 1984..1999 is reserved".into());
        }
        if !ind_ids.insert(public_id) {
            return Err("identity: duplicate individual public_id".into());
        }
        let label = nonempty_str(&item["public_label"], "individual public_label")?;
        if label != format!("Individual {public_id}") || contains_worker_hostname(label) {
            return Err("identity: individual public_label must be 'Individual <N>'".into());
        }
        let origin = nonempty_str(&item["origin"], "individual origin")?;
        if !ALLOWED_ORIGINS.contains(&origin) {
            return Err(format!("identity: unsupported origin {origin}").into());
        }
        if origin != "Birth" && !item["source_birth_event_key"].is_null() {
            return Err("identity: non-birth individual cannot claim a birth event key".into());
        }
        if let Some(g) = item.get("genome_sha256").filter(|v| !v.is_null()) {
            sha256_hex(g, "individual genome_sha256")?;
        }
        if let Some(c) = item.get("certificate_identity").filter(|v| !v.is_null()) {
            sha256_hex(c, "individual certificate_identity")?;
        }
        let item_aliases = item["aliases"]
            .as_array()
            .ok_or("identity: missing individual aliases")?;
        if item_aliases.is_empty() {
            return Err("identity: allocated individual must bind at least one source alias".into());
        }
        for alias in item_aliases {
            let alias_str = nonempty_str(alias, "individual alias")?;
            if aliases.insert(alias_str.to_string(), public_id).is_some() {
                return Err(format!("identity: duplicate source alias {alias_str}").into());
            }
        }
    }

    let newer_range = &registry["newer_birth_range_allocation"];
    let newer_start = newer_range["public_id_start"]
        .as_u64()
        .ok_or("identity: missing newer public_id_start")?;
    let newer_end = newer_range["public_id_end"]
        .as_u64()
        .ok_or("identity: missing newer public_id_end")?;
    let newer_count = newer_range["birth_event_count"]
        .as_u64()
        .ok_or("identity: missing newer birth_event_count")?;
    if newer_start < APPEND_ONLY_INDIVIDUAL_FLOOR
        || newer_end != newer_start + newer_count - 1
        || newer_count != 3649
    {
        return Err("identity: newer birth range allocation must cover 3649 events".into());
    }
    for &id in &ind_ids {
        if id >= newer_start && id <= newer_end {
            return Err("identity: explicit individual collides with newer birth range".into());
        }
    }
    let next_watermark = registry["next_individual_id_watermark"]
        .as_u64()
        .ok_or("identity: missing next_individual_id_watermark")?;
    if next_watermark <= newer_end
        || ind_ids.iter().any(|&id| id >= next_watermark)
    {
        return Err("identity: next_individual_id_watermark must exceed all allocated IDs".into());
    }

    Ok(json!({
        "schema": "jarambit.public_ids_validation.v1",
        "historical_reserved_count": HISTORICAL_BIRTH_MAX_ID + 1,
        "explicit_individual_count": ind_ids.len(),
        "newer_birth_range": [newer_start, newer_end],
        "run_count": run_ids.len(),
        "island_count": island_ids.len(),
        "next_individual_id_watermark": next_watermark,
        "alias_lookup": aliases,
    }))
}

/// Validate `docs/paper/data/selection_receipts.v1.json` against the public ID registry.
pub fn validate_selection_receipts(selection: &Value, registry: &Value) -> Result<Value> {
    let reg_summary = validate_public_ids(registry)?;
    if selection["schema"].as_str() != Some(SELECTION_RECEIPTS_SCHEMA) {
        return Err("selection: unsupported selection_receipts schema".into());
    }
    let cover = &selection["cover_illustration"];
    if cover["subject_alias"].as_str() != Some("cert_pipeline_r01_p14_c02")
        || cover["public_individual_id"].as_u64() != Some(2000)
        || cover["qualification_status"].as_str() != Some("assessed_not_qualified")
        || !cover["champion"].is_null()
        || cover["qualified_count"].as_u64() != Some(0)
    {
        return Err(
            "selection: cover must remain an illustrated assessed individual with champion=null"
                .into(),
        );
    }
    let cover_auth = nonempty_str(&cover["editorial_authority"], "cover editorial_authority")?;
    if !ALLOWED_AUTHORITIES.contains(&cover_auth) {
        return Err("selection: invalid cover editorial_authority".into());
    }

    let decisions = selection["decisions"]
        .as_array()
        .ok_or("selection: missing decisions array")?;
    if decisions.len() < 4 {
        return Err("selection: must record all four decision classes".into());
    }
    let mut classes = BTreeSet::new();
    for d in decisions {
        nonempty_str(&d["decision_id"], "decision_id")?;
        let class = nonempty_str(&d["decision_class"], "decision_class")?;
        if ![
            "creation_proposal",
            "parent_survivor_selection",
            "assessment_admission",
            "editorial_illustration",
        ]
        .contains(&class)
        {
            return Err(format!("selection: unknown decision_class {class}").into());
        }
        classes.insert(class.to_string());
        let auth = nonempty_str(&d["authority"], "decision authority")?;
        if !ALLOWED_AUTHORITIES.contains(&auth) {
            return Err(format!("selection: invalid authority {auth}").into());
        }
        nonempty_str(&d["candidate_pool"], "candidate_pool")?;
        nonempty_str(&d["evidence_selector"], "evidence_selector")?;
    }
    if classes.len() != 4 {
        return Err("selection: all four decision classes must be represented".into());
    }

    let claims = selection["claims_audit"]
        .as_array()
        .ok_or("selection: missing claims_audit array")?;
    if claims.is_empty() {
        return Err("selection: claims_audit cannot be empty".into());
    }
    let mut claim_ids = BTreeSet::new();
    for claim in claims {
        let cid = nonempty_str(&claim["claim_id"], "claim_id")?;
        if !claim_ids.insert(cid.to_string()) {
            return Err(format!("selection: duplicate claim_id {cid}").into());
        }
        let status = nonempty_str(&claim["status"], "claim status")?;
        if !ALLOWED_CLAIM_STATUSES.contains(&status) {
            return Err(format!("selection: invalid claim status {status}").into());
        }
        let auth = nonempty_str(&claim["selection_authority"], "claim selection_authority")?;
        if !ALLOWED_AUTHORITIES.contains(&auth) {
            return Err(format!("selection: invalid claim authority {auth}").into());
        }
        nonempty_str(&claim["named_pool"], "claim named_pool")?;
        nonempty_str(&claim["source_selector"], "claim source_selector")?;
    }

    Ok(json!({
        "schema": "jarambit.selection_receipts_validation.v1",
        "decision_count": decisions.len(),
        "claim_count": claims.len(),
        "cover_public_individual_id": cover["public_individual_id"],
        "cover_champion": Value::Null,
        "public_ids_summary": reg_summary,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_registry() -> Value {
        json!({
            "schema": PUBLIC_IDS_SCHEMA,
            "historical_reservation": {
                "run_public_id": 1,
                "id_min": 0,
                "id_max": 1983,
                "count": 1984,
                "origin": "Birth",
                "source_all_evals_json_sha256": "307370108fcede8a4399569582f6ab19d436603246531266afa1754f9ba65363"
            },
            "runs": [
                {
                    "public_run_id": 1,
                    "public_label": "Run 1",
                    "internal_run_key": "evidence/rs_v2/w282_ultrarunner",
                    "cohort_family": "historical_reconstructable_w282"
                }
            ],
            "islands": [
                {
                    "public_island_id": 1,
                    "public_label": "Island 1",
                    "internal_island_index": 0
                }
            ],
            "allocated_individuals": [
                {
                    "public_id": 1095,
                    "public_label": "Individual 1095",
                    "origin": "Birth",
                    "source_birth_event_key": "w282_ultrarunner:birth:1095",
                    "genome_sha256": null,
                    "certificate_identity": null,
                    "aliases": ["w282:id:1095"]
                },
                {
                    "public_id": 2000,
                    "public_label": "Individual 2000",
                    "origin": "RetainedExport",
                    "source_birth_event_key": null,
                    "genome_sha256": "a526b2ef73b25e51a56bcfab1f67640bcbe734955b95a6b432a99dab5cd9f654",
                    "certificate_identity": "db5d8e32e99d0b2d8ef6d7c7f9622a0601487c25b80879bbc8be1a502693c55e",
                    "aliases": ["cert_pipeline_r01_p14_c02"]
                }
            ],
            "newer_birth_range_allocation": {
                "public_id_start": 3000,
                "public_id_end": 6648,
                "birth_event_count": 3649
            },
            "next_individual_id_watermark": 7000
        })
    }

    fn sample_selection() -> Value {
        json!({
            "schema": SELECTION_RECEIPTS_SCHEMA,
            "cover_illustration": {
                "subject_alias": "cert_pipeline_r01_p14_c02",
                "public_individual_id": 2000,
                "qualification_status": "assessed_not_qualified",
                "champion": null,
                "qualified_count": 0,
                "editorial_authority": "human"
            },
            "decisions": [
                {
                    "decision_id": "D1",
                    "decision_class": "creation_proposal",
                    "authority": "algorithm",
                    "candidate_pool": "w282_ultrarunner",
                    "evidence_selector": "evidence/rs_v2/w282_ultrarunner/search/all_evals.json"
                },
                {
                    "decision_id": "D2",
                    "decision_class": "parent_survivor_selection",
                    "authority": "algorithm",
                    "candidate_pool": "w282_ultrarunner",
                    "evidence_selector": "evidence/rs_v2/w282_ultrarunner/search/evidence.jevl"
                },
                {
                    "decision_id": "D3",
                    "decision_class": "assessment_admission",
                    "authority": "algorithm",
                    "candidate_pool": "w282_ultrarunner top-5",
                    "evidence_selector": "evidence/rs_v2/w282_ultrarunner/search_summary.json"
                },
                {
                    "decision_id": "D4",
                    "decision_class": "editorial_illustration",
                    "authority": "human",
                    "candidate_pool": "evidence/evolve_certifiable/manifest.json",
                    "evidence_selector": "docs/paper/figure-inputs.v1.json"
                }
            ],
            "claims_audit": [
                {
                    "claim_id": "C1",
                    "status": "demonstrated",
                    "selection_authority": "algorithm",
                    "named_pool": "w282 1984 births",
                    "source_selector": "evidence/rs_v2/w282_ultrarunner/search/all_evals.json"
                }
            ]
        })
    }

    #[test]
    fn valid_registry_and_selection_receipts_pass() {
        let reg = sample_registry();
        let sel = sample_selection();
        let summary = validate_selection_receipts(&sel, &reg).unwrap();
        assert_eq!(summary["cover_public_individual_id"], 2000);
        assert!(summary["cover_champion"].is_null());
    }

    #[test]
    fn worker_hostnames_duplicate_ids_and_fabricated_birth_links_are_refused() {
        let mut reg = sample_registry();
        reg["runs"][0]["public_label"] = json!("Run 1 (xbabe2)");
        assert!(validate_public_ids(&reg).is_err());

        let mut reg = sample_registry();
        reg["allocated_individuals"][1]["source_birth_event_key"] = json!("invented:birth:2000");
        assert!(validate_public_ids(&reg).is_err());

        let mut sel = sample_selection();
        sel["cover_illustration"]["champion"] = json!("cert_pipeline_r01_p14_c02");
        assert!(validate_selection_receipts(&sel, &sample_registry()).is_err());
    }
}
