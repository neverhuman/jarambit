//! Internal consistency of retained reporting data, not scientific re-evaluation.
use serde_json::Value;
use std::collections::BTreeSet;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
// Stored scores and component values are rounded to six decimal places.
const SCORE_TOLERANCE: f64 = 0.000002;
const HALF_ROUNDING_UNIT: f64 = 0.0000005;

pub(crate) const REPORT_GROUPS: [(&str, &[&str], f64); 4] = [
    (
        "Mass + compactness",
        &["ultralight_mass", "holder_and_compactness"],
        30.0,
    ),
    (
        "Form + comfort proxies",
        &[
            "karambit_ring_and_claw",
            "c6_aesthetic_and_ergo",
            "bore_collar_comfort",
        ],
        25.0,
    ),
    (
        "Exported geometry",
        &[
            "cad_watertight_brep",
            "gate_s_sharpness",
            "gate_s_point_profile",
        ],
        15.0,
    ),
    (
        "Response + verification",
        &[
            "hiker_lateral_180n",
            "hunter_lateral_260n",
            "strict_hunter_260n",
            "ring_pull_1500n",
            "twist_body_1p82nm",
            "twist_tip_plate_1p82nm",
            "solid_tip_3d_p2_and_xcheck",
            "side_a4_buckling",
            "failure_sequence_1p2",
            "plate_gci_3level",
            "human_max_overload",
            "goodman_fatigue_l7",
        ],
        30.0,
    ),
];

fn number(v: &Value) -> Result<f64> {
    v.as_f64()
        .filter(|n| n.is_finite())
        .ok_or_else(|| "weighted report: expected finite number".into())
}

fn text(v: &Value) -> Result<&str> {
    v.as_str()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "weighted report: expected nonempty identity".into())
}

fn score(v: &Value) -> Result<f64> {
    let n = number(v)?;
    if !(0.0..=100.0).contains(&n) {
        return Err("weighted report: fitness out of range".into());
    }
    Ok(n)
}

fn hash(v: &Value) -> Result<&str> {
    let s = text(v)?;
    if s.len() != 64 || !s.bytes().all(|c| c.is_ascii_hexdigit()) {
        return Err("weighted report: invalid content identity".into());
    }
    Ok(s)
}

fn predicate_ids(catalog: &[Value]) -> Result<BTreeSet<String>> {
    let mut ids = BTreeSet::new();
    for entry in catalog {
        if !matches!(entry["kind"].as_str(), Some("G16" | "RSV2")) {
            continue;
        }
        let alias = text(&entry["title"])?
            .split_once(':')
            .ok_or("weighted report: catalog alias missing")?
            .0;
        if !ids.insert(format!("{alias}:{}", text(&entry["id"])?)) {
            return Err("weighted report: duplicate catalog predicate".into());
        }
    }
    if ids.len() != 26 {
        return Err("weighted report: expected 26 catalog predicates".into());
    }
    Ok(ids)
}

fn validate_profile(best: &Value, expected_predicates: &BTreeSet<String>) -> Result<()> {
    text(&best["label"])?;
    hash(&best["certificate_identity"])?;
    hash(&best["genome_sha256"])?;
    let total = score(&best["ultralight_weighted_fitness"])?;
    let profile = &best["ultralight_fitness_breakdown"];
    if profile["metric"] != "W_ULTRALIGHT_v1" {
        return Err("weighted report: wrong profile metric".into());
    }
    for key in ["score_100", "global_weighted_fitness"] {
        if (score(&profile[key])? - total).abs() > SCORE_TOLERANCE {
            return Err("weighted report: profile score differs from selected score".into());
        }
    }
    let terms = profile["objectives"]
        .as_object()
        .ok_or("weighted report: missing objectives")?;
    let expected_terms: BTreeSet<_> = REPORT_GROUPS
        .iter()
        .flat_map(|(_, keys, _)| keys.iter().copied())
        .collect();
    if terms.keys().map(String::as_str).collect::<BTreeSet<_>>() != expected_terms {
        return Err("weighted report: objective membership mismatch".into());
    }
    let mut sum = 0.0;
    for (_, keys, cap) in REPORT_GROUPS {
        let mut group_weight = 0.0;
        for key in keys {
            let term = &terms[*key];
            let weight = number(&term["weight_pct"])?;
            let normalized = number(&term["normalized_score"])?;
            let points = number(&term["weighted_points"])?;
            if !(0.0..=100.0).contains(&weight) || !(0.0..=1.0).contains(&normalized) {
                return Err("weighted report: invalid objective weight or normalized value".into());
            }
            // Both stored factors and points have independent rounding error.
            let tolerance = HALF_ROUNDING_UNIT * (weight + 1.0) + 1e-9;
            if (points - weight * normalized).abs() > tolerance {
                return Err("weighted report: objective arithmetic mismatch".into());
            }
            group_weight += weight;
            sum += points;
        }
        if (group_weight - cap).abs() > 1e-9 {
            return Err("weighted report: displayed group weight mismatch".into());
        }
    }
    let sum_tolerance = HALF_ROUNDING_UNIT * (terms.len() as f64 + 1.0) + 1e-9;
    if (sum - total).abs() > sum_tolerance
        || (number(&profile["weight_sum_pct"])? - 100.0).abs() > 1e-9
    {
        return Err("weighted report: profile sum mismatch".into());
    }
    let predicates = profile["legacy_26_gate_breakdown"]
        .as_object()
        .ok_or("weighted report: missing predicate flags")?;
    if predicates.keys().cloned().collect::<BTreeSet<_>>() != *expected_predicates {
        return Err("weighted report: predicate membership mismatch".into());
    }
    let mut failed = BTreeSet::new();
    for (id, predicate) in predicates {
        if !predicate["legacy_pass"]
            .as_bool()
            .ok_or("weighted report: predicate flag must be Boolean")?
        {
            failed.insert(id.as_str());
        }
    }
    let passed = predicates.len() - failed.len();
    if profile["legacy_hard_gates_passed_count"].as_u64() != Some(passed as u64)
        || profile["all_26_legacy_hard_gates_pass"].as_bool() != Some(failed.is_empty())
    {
        return Err("weighted report: predicate tally differs from flags".into());
    }
    let declared_failed = profile["legacy_hard_gates_failed"]
        .as_array()
        .ok_or("weighted report: missing failed-predicate list")?;
    let declared: BTreeSet<_> = declared_failed.iter().map(text).collect::<Result<_>>()?;
    if declared.len() != declared_failed.len() || declared != failed {
        return Err("weighted report: failed-predicate list differs from flags".into());
    }
    Ok(())
}

/// Check reporting identities/algebra without substituting or recomputing physics.
/// Recurrence/order checks remain in `publication::validate_curve`.
pub(crate) fn validate_weighted_report(curve: &Value, catalog: &[Value]) -> Result<()> {
    let expected = predicate_ids(catalog)?;
    let best = &curve["best_so_far"];
    validate_profile(best, &expected)?;
    let last = curve["generations"]
        .as_array()
        .and_then(|rows| rows.last())
        .ok_or("weighted report: empty curve")?;
    if (score(&last["hall_of_fame_best_so_far"])? - score(&best["ultralight_weighted_fitness"])?)
        .abs()
        > SCORE_TOLERANCE
    {
        return Err("weighted report: selected best differs from final Hall of Fame".into());
    }
    if text(&last["best_so_far_label"])? != text(&best["label"])? {
        return Err("weighted report: selected label differs from final Hall of Fame".into());
    }
    if let Some(stage2) = curve
        .get("best_stage2_26_gate_design")
        .filter(|v| !v.is_null())
    {
        validate_profile(stage2, &expected)?;
        // A screen best and a Stage-2 best may be different candidates. Only
        // require duplicate records of the same selected candidate to agree.
        if text(&stage2["label"])? == text(&best["label"])? {
            for key in ["certificate_identity", "genome_sha256"] {
                if !hash(&stage2[key])?.eq_ignore_ascii_case(hash(&best[key])?) {
                    return Err("weighted report: matching Stage-2 identity differs".into());
                }
            }
            if (score(&stage2["ultralight_weighted_fitness"])?
                - score(&best["ultralight_weighted_fitness"])?)
            .abs()
                > SCORE_TOLERANCE
                || stage2["ultralight_fitness_breakdown"] != best["ultralight_fitness_breakdown"]
            {
                return Err("weighted report: matching Stage-2 profile differs".into());
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Retained input is read-only. Each mutation is a reporting fixture only.
    fn fixture() -> (Value, Vec<Value>) {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let curve = serde_json::from_slice(
            &std::fs::read(root.join(super::super::publication::CURVE)).unwrap(),
        )
        .unwrap();
        let catalog: Value =
            serde_json::from_slice(&std::fs::read(root.join("docs/tests/catalog.json")).unwrap())
                .unwrap();
        (curve, catalog["entries"].as_array().unwrap().clone())
    }

    fn rejected(curve: &Value, catalog: &[Value], message: &str) {
        let error = validate_weighted_report(curve, catalog)
            .unwrap_err()
            .to_string();
        assert!(error.contains(message), "unexpected rejection: {error}");
    }

    #[test]
    fn admitted_report_preserves_recorded_rounding_and_stage_differences() {
        let (curve, catalog) = fixture();
        validate_weighted_report(&curve, &catalog).unwrap();
        // Outer screen metrics differ legitimately from assessed profile fields.
        assert_ne!(
            curve["best_so_far"]["seq_ratio"],
            curve["best_so_far"]["ultralight_fitness_breakdown"]["legacy_26_gate_breakdown"]
                ["G16-14:failure_sequence_1p2"]["raw"]
        );
    }

    #[test]
    fn selected_score_must_match_final_history() {
        let (mut curve, catalog) = fixture();
        for row in curve["generations"].as_array_mut().unwrap() {
            row["round_best_fitness"] = json!(0.0);
            row["hall_of_fame_best_so_far"] = json!(0.0);
        }
        rejected(&curve, &catalog, "selected best differs");
    }

    #[test]
    fn selected_label_must_match_final_history() {
        let (mut curve, catalog) = fixture();
        curve["best_so_far"]["label"] = json!("REVIEW_FIXTURE_WRONG_SUBJECT");
        rejected(&curve, &catalog, "selected label differs");
    }

    #[test]
    fn tally_and_aggregate_must_follow_typed_predicate_flags() {
        let (curve, catalog) = fixture();
        for (key, value) in [
            ("legacy_hard_gates_passed_count", json!(26)),
            ("all_26_legacy_hard_gates_pass", json!(true)),
        ] {
            let mut changed = curve.clone();
            changed["best_so_far"]["ultralight_fitness_breakdown"][key] = value;
            rejected(&changed, &catalog, "tally differs");
        }
        let mut changed = curve.clone();
        changed["best_so_far"]["ultralight_fitness_breakdown"]["legacy_26_gate_breakdown"]
            ["G16-01:cad_watertight_and_single_solid"]["legacy_pass"] = json!(1);
        rejected(&changed, &catalog, "must be Boolean");
    }

    #[test]
    fn incomplete_or_renamed_predicate_membership_is_refused() {
        let (mut curve, catalog) = fixture();
        curve["best_so_far"]["ultralight_fitness_breakdown"]["legacy_26_gate_breakdown"]
            .as_object_mut()
            .unwrap()
            .remove("G16-01:cad_watertight_and_single_solid");
        rejected(&curve, &catalog, "predicate membership");
    }

    #[test]
    fn failed_list_cannot_disagree_with_flags_or_repeat_ids() {
        let (mut curve, catalog) = fixture();
        let failed = curve["best_so_far"]["ultralight_fitness_breakdown"]
            ["legacy_hard_gates_failed"]
            .as_array_mut()
            .unwrap();
        failed.push(failed[0].clone());
        rejected(&curve, &catalog, "failed-predicate list");
    }

    #[test]
    fn same_label_cannot_hide_different_content_identity() {
        let (mut curve, catalog) = fixture();
        curve["best_stage2_26_gate_design"]["genome_sha256"] = json!("0".repeat(64));
        rejected(&curve, &catalog, "Stage-2 identity differs");
    }

    #[test]
    fn unknown_objectives_and_inconsistent_component_arithmetic_are_refused() {
        let (curve, catalog) = fixture();
        let mut changed = curve.clone();
        changed["best_so_far"]["ultralight_fitness_breakdown"]["objectives"]
            ["review_fixture_extra"] = json!({});
        rejected(&changed, &catalog, "objective membership");
        let mut changed = curve;
        changed["best_so_far"]["ultralight_fitness_breakdown"]["objectives"]["ultralight_mass"]
            ["normalized_score"] = json!(0.0);
        rejected(&changed, &catalog, "objective arithmetic");
    }

    #[test]
    fn distinct_stage2_best_is_not_silently_promoted_to_screen_best() {
        let (mut curve, catalog) = fixture();
        curve["best_stage2_26_gate_design"]["label"] = json!("REVIEW_FIXTURE_OTHER_STAGE2_SUBJECT");
        curve["best_stage2_26_gate_design"]["genome_sha256"] = json!("0".repeat(64));
        validate_weighted_report(&curve, &catalog).unwrap();
        curve["best_stage2_26_gate_design"] = Value::Null;
        validate_weighted_report(&curve, &catalog).unwrap();
    }
}
