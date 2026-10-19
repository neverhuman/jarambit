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

/// Reversible display-loss transform for a declared score profile (`JR-EI-P2-T06`).
/// Never clips out-of-range native scores or mixes distinct profile stages.
pub fn compute_display_loss(native_score: f64, transform: &str) -> Result<f64> {
    if !native_score.is_finite() {
        return Err("score_profiles: nonfinite native score".into());
    }
    match transform {
        "bounded_100_minus_F" => {
            if !(0.0..=100.0).contains(&native_score) {
                return Err("score_profiles: bounded_100_minus_F requires F in [0, 100]".into());
            }
            Ok(100.0 - native_score)
        }
        "negated_native_fit" => Ok(-native_score),
        _ => Err(format!("score_profiles: unsupported display_loss transform '{transform}'").into()),
    }
}

/// Validate `docs/paper/data/score_profiles.v1.json` (`JR-EI-P2-T06`).
pub fn validate_score_profiles(doc: &Value) -> Result<Value> {
    if doc["schema"].as_str() != Some("jarambit.score_profiles.v1") {
        return Err("score_profiles: unsupported schema".into());
    }
    if doc["cross_profile_splicing_allowed"].as_bool() != Some(false) {
        return Err("score_profiles: cross_profile_splicing_allowed must be false".into());
    }
    let profiles = doc["profiles"]
        .as_array()
        .ok_or("score_profiles: missing profiles array")?;
    if profiles.is_empty() {
        return Err("score_profiles: profiles array cannot be empty".into());
    }
    let mut seen_ids = BTreeSet::new();
    for p in profiles {
        let pid = text(&p["profile_id"])?;
        if !seen_ids.insert(pid.to_string()) {
            return Err(format!("score_profiles: duplicate profile_id {pid}").into());
        }
        text(&p["native_direction"])?;
        text(&p["observation_stage"])?;
        let transform = text(&p["display_loss_transform"])?;
        if !["bounded_100_minus_F", "negated_native_fit"].contains(&transform) {
            return Err(format!("score_profiles: invalid transform {transform}").into());
        }
        if pid == "W_ULTRALIGHT_v1" {
            let groups = p["priority_groups"]
                .as_array()
                .ok_or("score_profiles: missing W_ULTRALIGHT_v1 priority_groups")?;
            if groups.len() != REPORT_GROUPS.len() {
                return Err("score_profiles: W_ULTRALIGHT_v1 group count mismatch".into());
            }
            let mut total_w = 0.0;
            for (g, (expected_name, _, expected_w)) in groups.iter().zip(REPORT_GROUPS.iter()) {
                if text(&g["group"])? != *expected_name {
                    return Err("score_profiles: group name mismatch".into());
                }
                let w = number(&g["weight_pct"])?;
                if (w - *expected_w).abs() > 1e-9 {
                    return Err("score_profiles: group weight mismatch".into());
                }
                total_w += w;
            }
            if (total_w - 100.0).abs() > 1e-9 {
                return Err("score_profiles: priority weights must sum to 100".into());
            }
        }
    }
    let series = doc["progress_series"]
        .as_array()
        .ok_or("score_profiles: missing progress_series")?;
    if series.is_empty() {
        return Err("score_profiles: progress_series cannot be empty".into());
    }
    for s in series {
        let pid = text(&s["profile_id"])?;
        if !seen_ids.contains(pid) {
            return Err(format!("score_profiles: unknown series profile_id {pid}").into());
        }
        let p_def = profiles
            .iter()
            .find(|p| p["profile_id"].as_str() == Some(pid))
            .unwrap();
        let transform = text(&p_def["display_loss_transform"])?;
        if s["observation_stage"] != p_def["observation_stage"] {
            return Err("score_profiles: progress_series stage differs from profile stage".into());
        }
        let pts = s["points"]
            .as_array()
            .ok_or("score_profiles: missing series points")?;
        if pts.is_empty() {
            return Err("score_profiles: empty series points".into());
        }
        let mut best_so_far = f64::NEG_INFINITY;
        for pt in pts {
            let gen_max = number(&pt["generation_best_native"])?;
            let cum_best = number(&pt["cumulative_best_native"])?;
            let loss = number(&pt["display_loss"])?;
            best_so_far = best_so_far.max(gen_max);
            if (cum_best - best_so_far).abs() > SCORE_TOLERANCE {
                return Err("score_profiles: cumulative_best_native recurrence mismatch".into());
            }
            let expected_loss = compute_display_loss(cum_best, transform)?;
            if (loss - expected_loss).abs() > SCORE_TOLERANCE {
                return Err("score_profiles: reversible display_loss mismatch".into());
            }
        }
    }
    Ok(serde_json::json!({
        "schema": "jarambit.score_profiles_validation.v1",
        "profile_count": seen_ids.len(),
        "series_count": series.len(),
    }))
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

    #[test]
    fn score_profiles_enforce_reversible_display_loss_and_refuse_stage_splicing() {
        let mut doc = json!({
            "schema": "jarambit.score_profiles.v1",
            "cross_profile_splicing_allowed": false,
            "profiles": [
                {
                    "profile_id": "historical_w282_native_qd_fit",
                    "native_direction": "higher_is_better",
                    "observation_stage": "post_rescore_gen_end",
                    "display_loss_transform": "negated_native_fit"
                }
            ],
            "progress_series": [
                {
                    "series_id": "w282_gen_end",
                    "profile_id": "historical_w282_native_qd_fit",
                    "observation_stage": "post_rescore_gen_end",
                    "points": [
                        { "generation": 0, "generation_best_native": 1.14, "cumulative_best_native": 1.14, "display_loss": -1.14 },
                        { "generation": 1, "generation_best_native": 1.15, "cumulative_best_native": 1.15, "display_loss": -1.15 }
                    ]
                }
            ]
        });
        assert!(validate_score_profiles(&doc).is_ok());
        doc["progress_series"][0]["observation_stage"] = json!("stage1_screen");
        assert!(validate_score_profiles(&doc).is_err());
        assert!(compute_display_loss(-3.8, "bounded_100_minus_F").is_err());
    }
}
