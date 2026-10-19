//! Read-only interpretation of the admitted historical population snapshot.
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub(crate) const PRODUCER: &str = "82f450f7fb418e28c4b229031920dec5454435f3";
const QD_SOURCE: &str = "fe3d057eeec74c3278b46027265406ec0ee97dd65938ae49cb003e08ee5b3a5f";

pub(crate) fn context(births: &[Value], provenance: &Value, summary: &Value) -> Result<Value> {
    if provenance["source_commit"] != PRODUCER
        || provenance["source_files_sha256"]["crates/crescent/src/qd.rs"] != QD_SOURCE
    {
        return Err("historical score producer has not been admitted".into());
    }
    if summary["evaluations"].as_u64() != Some(births.len() as u64) {
        return Err("historical summary membership differs from population".into());
    }
    let selection = summary["selection"]
        .as_str()
        .filter(|s| !s.is_empty())
        .ok_or("missing historical selection statement")?;
    let mut ids = BTreeSet::new();
    let mut generations: BTreeMap<u64, (usize, usize, usize, Vec<f64>)> = BTreeMap::new();
    let mut best: Option<(u64, u64, f64)> = None;
    for birth in births {
        let id = birth["id"].as_u64().ok_or("invalid historical ID")?;
        if !ids.insert(id) {
            return Err("duplicate historical ID".into());
        }
        let generation = birth["gen"]
            .as_u64()
            .ok_or("invalid historical generation")?;
        let fitness = birth["fit"]
            .as_f64()
            .filter(|n| n.is_finite())
            .ok_or("invalid historical fitness")?;
        let legal = birth["legal"]
            .as_bool()
            .ok_or("historical legality must be Boolean")?;
        let group = generations.entry(generation).or_default();
        match birth["fem_status"].as_str() {
            Some("complete") => {
                group.0 += 1;
                group.3.push(fitness);
            }
            Some("not_evaluated") => {
                group.1 += 1;
            }
            _ => return Err("unrecognized historical FEM status".into()),
        }
        group.2 += usize::from(legal);
        if best.is_none_or(|(_, _, previous)| fitness > previous) {
            best = Some((id, generation, fitness));
        }
    }
    let (best_id, best_generation, best_fit) = best.ok_or("empty historical population")?;
    let mut complete = 0;
    let mut not_evaluated = 0;
    let mut legal = 0;
    let mut by_generation = Vec::new();
    for (generation, (solved, unsolved, valid, values)) in generations {
        complete += solved;
        not_evaluated += unsolved;
        legal += valid;
        let solved_median = if values.is_empty() {
            None
        } else {
            Some(crate::publication::median(&values)?)
        };
        by_generation.push(json!({"generation":generation,"fem_complete":solved,"fem_not_evaluated":unsolved,"legal":valid,"fem_complete_median":solved_median}));
    }
    let rows = summary["strict_certification"]
        .as_array()
        .ok_or("missing historical assessment rows")?;
    let selected: Vec<_> = rows
        .iter()
        .filter(|r| r["id"].as_str() == Some(best_id.to_string().as_str()))
        .collect();
    if selected.len() > 1 {
        return Err("ambiguous historical best assessment".into());
    }
    let assessment = if let Some(row) = selected.first() {
        if row["gen"].as_str().and_then(|s| s.parse::<u64>().ok()) != Some(best_generation) {
            return Err("historical best assessment generation differs".into());
        }
        let mut result = serde_json::Map::new();
        for key in ["cert_pass_all", "seq_ok", "strict_ok"] {
            let flag = match row[key].as_str() {
                Some("true") => true,
                Some("false") => false,
                _ => return Err("invalid historical assessment flag".into()),
            };
            result.insert(key.into(), json!(flag));
        }
        Value::Object(result)
    } else {
        Value::Null
    };
    Ok(json!({
        "producer_revision":PRODUCER,"producer_qd_sha256":QD_SOURCE,
        "fitness_semantics":"Legality-dependent stored score with contextual novelty; not a fixed physical observable",
        "fem_status_counts":{"complete":complete,"not_evaluated":not_evaluated},
        "legal_count":legal,"by_generation":by_generation,
        "best_observed":{"id":best_id,"generation":best_generation,"fit":best_fit,"strict_assessment":assessment},
        "selection_statement":selection
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Vec<Value>, Value, Value) {
        let births = vec![
            json!({"id":1,"gen":0,"fit":-2.,"legal":false,"fem_status":"not_evaluated"}),
            json!({"id":2,"gen":0,"fit":1.,"legal":true,"fem_status":"complete"}),
        ];
        let provenance = json!({"source_commit":PRODUCER,"source_files_sha256":{"crates/crescent/src/qd.rs":QD_SOURCE}});
        let summary = json!({"evaluations":2,"selection":"Neutral reporting fixture, not an engineering result","strict_certification":[{"id":"2","gen":"0","cert_pass_all":"false","seq_ok":"false","strict_ok":"true"}]});
        (births, provenance, summary)
    }
    #[test]
    fn separates_unevaluated_records_and_binds_the_best_assessment() {
        let (births, provenance, summary) = fixture();
        let report = context(&births, &provenance, &summary).unwrap();
        assert_eq!(
            report["fem_status_counts"],
            json!({"complete":1,"not_evaluated":1})
        );
        assert_eq!(report["by_generation"][0]["fem_complete_median"], 1.);
        assert_eq!(report["best_observed"]["id"], 2);
        assert_eq!(
            report["best_observed"]["strict_assessment"]["cert_pass_all"],
            false
        );
    }
    #[test]
    fn unknown_status_and_untyped_legality_are_not_solved_results() {
        let (births, provenance, summary) = fixture();
        for (field, value) in [("fem_status", json!("unknown")), ("legal", json!(1))] {
            let mut changed = births.clone();
            changed[0][field] = value;
            assert!(context(&changed, &provenance, &summary).is_err());
        }
    }
    #[test]
    fn missing_assessment_stays_unavailable_and_conflicting_join_is_refused() {
        let (births, provenance, mut summary) = fixture();
        summary["strict_certification"][0]["gen"] = json!("9");
        assert!(context(&births, &provenance, &summary).is_err());
        summary["strict_certification"] = json!([]);
        assert!(
            context(&births, &provenance, &summary).unwrap()["best_observed"]["strict_assessment"]
                .is_null()
        );
    }
    #[test]
    fn producer_and_population_membership_cannot_silently_change() {
        let (births, mut provenance, summary) = fixture();
        provenance["source_commit"] = json!("another producer");
        assert!(context(&births, &provenance, &summary).is_err());
        let (_, provenance, mut summary) = fixture();
        summary["evaluations"] = json!(3);
        assert!(context(&births, &provenance, &summary).is_err());
    }
}
