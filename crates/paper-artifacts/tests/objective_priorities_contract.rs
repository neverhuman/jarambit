// Authored neutral/source-text controls. No Python module/scorer is imported or
// executed; original source is an immutable byte/text accession only.
#[path = "../src/objective_priorities.rs"]
pub mod objective_priorities;
use objective_priorities::*;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
const RECIPE: &[u8] =
    include_bytes!("../../../docs/paper/history/ultralight_weighted_fitness_009f5ac5.source.txt");
fn valid() -> PriorityRoster {
    validate_priority_roster(&current_priority_contract(), RECIPE).unwrap()
}
fn entries<'a>(source: &'a str, header: &str, end: &str) -> Vec<(&'a str, &'a str)> {
    let body = source
        .split_once(header)
        .unwrap()
        .1
        .split_once(end)
        .unwrap()
        .0;
    body.lines()
        .filter_map(|l| l.trim().strip_prefix('\'').and_then(|l| l.split_once("':")))
        .map(|(id, value)| (id, value.trim().trim_end_matches(',')))
        .collect()
}
#[test]
fn original_twenty_weights_normalizers_and_selectors_match_independent_source_text() {
    let source = std::str::from_utf8(RECIPE).unwrap();
    assert_eq!(format!("{:x}", Sha256::digest(RECIPE)), RECIPE_SHA256);
    let weights = entries(source, "ULTRALIGHT_WEIGHTS_V1: dict[str, float] = {", "\n}");
    let normalizers = entries(source, "s_scores: dict[str, float] = {", "\n    }");
    assert_eq!(weights.len(), 20);
    assert_eq!(normalizers.len(), 20);
    let w: BTreeMap<_, _> = weights.iter().copied().collect();
    let n: BTreeSet<_> = normalizers.iter().map(|r| r.0).collect();
    assert_eq!(w.len(), 20);
    assert_eq!(n.len(), 20);
    assert_eq!(w.keys().copied().collect::<BTreeSet<_>>(), n);
    // Independent decimal arithmetic over ORIGINAL source tokens, not the helper.
    let tenths: Vec<u32> = weights
        .iter()
        .map(|(_, v)| {
            let (a, b) = v.split_once('.').unwrap();
            a.parse::<u32>().unwrap() * 10 + b.parse::<u32>().unwrap()
        })
        .collect();
    assert_eq!(tenths.iter().sum::<u32>(), 1000);
    assert_eq!(
        [
            tenths[..2].iter().sum::<u32>(),
            tenths[2..5].iter().sum(),
            tenths[5..8].iter().sum(),
            tenths[8..].iter().sum()
        ],
        [300, 250, 150, 300]
    );
    let roster = valid();
    let contract = current_priority_contract();
    for row in roster.rows() {
        assert_eq!(
            format!("{}.{:01}", row.weight_tenths / 10, row.weight_tenths % 10),
            w[row.id.as_str()]
        );
        let line = source
            .lines()
            .nth(row.weight_line as usize - 1)
            .unwrap()
            .trim();
        assert_eq!(line, format!("'{}': {},", row.id, w[row.id.as_str()]));
        let start = source
            .lines()
            .nth(row.normalization_lines[0] as usize - 1)
            .unwrap()
            .trim();
        assert!(start.starts_with(&format!("'{}':", row.id)));
        let i = row.ordinal - 1;
        assert_eq!(
            contract["objectives"][i]["normalization_source_selector"],
            format!("compute_ultralight_weighted_fitness.s_scores['{}']", row.id)
        );
        if i + 1 < 20 {
            assert_eq!(
                row.normalization_lines[1] + 1,
                roster.rows()[i + 1].normalization_lines[0]
            );
        }
    }
}
#[test]
fn missing_extra_duplicate_unknown_and_selector_or_source_changes_are_refused() {
    let c = current_priority_contract();
    let mut bad = c.clone();
    bad["objectives"].as_array_mut().unwrap().pop();
    assert!(validate_priority_roster(&bad, RECIPE).is_err());
    let mut bad = c.clone();
    bad["objectives"]
        .as_array_mut()
        .unwrap()
        .push(c["objectives"][0].clone());
    assert!(validate_priority_roster(&bad, RECIPE).is_err());
    let mut bad = c.clone();
    bad["objectives"][1] = c["objectives"][0].clone();
    assert!(validate_priority_roster(&bad, RECIPE).is_err());
    let mut bad = c.clone();
    bad["objectives"][0]["id"] = json!("unknown_objective");
    assert!(validate_priority_roster(&bad, RECIPE).is_err());
    let mut bad = c.clone();
    bad["objectives"][0]["normalization_source_selector"] = json!("another_stage.score");
    assert!(validate_priority_roster(&bad, RECIPE).is_err());
    let mut bad = c.clone();
    bad["objectives"][15]["scientific_label"] = json!("Validated buckling capacity");
    assert!(validate_priority_roster(&bad, RECIPE).is_err());
    assert!(validate_priority_roster(&c, b"changed recipe bytes").is_err());
}
#[test]
fn exact_decimal_and_per_id_weight_rules_never_silently_normalize() {
    for v in [
        "", "NaN", "Infinity", "-1.0", "+1.0", "1.5e0", " 2.0", "2.00", "02.0", "100.1",
    ] {
        assert!(parse_percent_tenths(v).is_err(), "{v}");
    }
    assert_eq!(parse_percent_tenths("22.0").unwrap(), 220);
    assert_eq!(parse_percent_tenths("0.0").unwrap(), 0);
    let c = current_priority_contract();
    for value in [
        json!(22.0),
        Value::Null,
        json!("NaN"),
        json!("-1.0"),
        json!("22.1"),
    ] {
        let mut bad = c.clone();
        bad["objectives"][0]["weight_pct"] = value;
        assert!(validate_priority_roster(&bad, RECIPE).is_err());
    }
    let mut compensated = c.clone();
    compensated["objectives"][0]["weight_pct"] = json!("21.0");
    compensated["objectives"][1]["weight_pct"] = json!("9.0");
    assert!(validate_priority_roster(&compensated, RECIPE).is_err()); // still100%, wrong priorities
    let mut bad = c;
    bad["declared_sum_pct"] = json!("99.9");
    assert!(validate_priority_roster(&bad, RECIPE).is_err());
    assert!(arcs_tenths(&[499, 250, 250]).is_err());
    assert!(arcs_tenths(&[]).is_err());
}
#[test]
fn three_weight_arc_oracle_has_half_and_quarter_circle_geometry() {
    let a = arcs_tenths(&[500, 250, 250]).unwrap();
    assert_eq!(
        a,
        vec![
            Arc { start: 0, end: 500 },
            Arc {
                start: 500,
                end: 750
            },
            Arc {
                start: 750,
                end: 1000
            }
        ]
    );
    for (arc, expected_degrees) in a.iter().zip([180., 90., 90.]) {
        assert_eq!(
            (arc.end - arc.start) as f64 * 360. / 1000.,
            expected_degrees
        );
    }
    for (tick, expected) in [(0, [0., -1.]), (500, [0., 1.]), (750, [-1., 0.])] {
        let point = unit_point(tick).unwrap();
        assert!((point[0] - expected[0]).abs() < 1e-12);
        assert!((point[1] - expected[1]).abs() < 1e-12);
    }
    assert_eq!(unit_point(0).unwrap(), unit_point(1000).unwrap());
    let path = arc_path(a[0], [0., 0.], 10., 5.).unwrap().unwrap();
    assert!(path.contains("A 10.000000 10.000000 0 0 1 0.000000 10.000000"));
    assert!(path.contains("L 0.000000 5.000000"));
}
#[test]
fn zero_weight_keeps_ordinal_and_count_but_has_no_filled_area() {
    let a = arcs_tenths(&[500, 0, 500]).unwrap();
    assert_eq!(a.len(), 3);
    assert_eq!(
        a[1],
        Arc {
            start: 500,
            end: 500
        }
    );
    assert!(arc_path(a[1], [0., 0.], 10., 5.).unwrap().is_none());
    let full = arc_path(arcs_tenths(&[1000]).unwrap()[0], [0., 0.], 10., 5.)
        .unwrap()
        .unwrap();
    assert_eq!(full.matches("A 10.000000").count(), 2);
    assert_eq!(full.matches("A 5.000000").count(), 2);
    let mut c = current_priority_contract();
    c["objectives"][19]["weight_pct"] = json!("0.0");
    assert!(validate_priority_roster(&c, RECIPE).is_err()); // future source needs explicit new admission
}
#[test]
fn every_source_id_has_one_slice_key_color_and_visible_number_with_readable_type() {
    let roster = valid();
    let output = render_priority_donut(&roster, 514.044).unwrap();
    assert_eq!(output.svg.matches("data-role=\"slice\"").count(), 20);
    assert_eq!(output.svg.matches("data-role=\"key\"").count(), 20);
    assert_eq!(
        output
            .svg
            .matches("data-role=\"ordinal-connector\"")
            .count(),
        20
    );
    let mut colors = BTreeSet::new();
    for (i, row) in roster.rows().iter().enumerate() {
        assert_eq!(
            output
                .svg
                .matches(&format!("data-objective-id=\"{}\"", row.id))
                .count(),
            2
        );
        assert!(output.svg.contains(&row.label));
        assert_eq!(
            output
                .svg
                .matches(&format!(">{:02}</text>", row.ordinal))
                .count(),
            2
        );
        let projected = &output.manifest["objectives"][i];
        assert_eq!(projected["id"], row.id);
        assert_eq!(projected["weight_tenths"], row.weight_tenths);
        let color = projected["nominal_color"].as_str().unwrap();
        assert!(colors.insert(color));
        assert_eq!(output.svg.matches(&format!("fill=\"{color}\"")).count(), 2);
        assert!(output.svg.contains(&format!(
            ">{}.{:01}%</text>",
            row.weight_tenths / 10,
            row.weight_tenths % 10
        )));
    }
    for part in output.svg.split("font-size=\"").skip(1) {
        let size = part.split('"').next().unwrap().parse::<u32>().unwrap();
        assert!(size >= 28);
    }
    assert!(
        output.manifest["typography"]["minimum_effective_pt"]
            .as_f64()
            .unwrap()
            >= 8.5
    );
    assert_eq!(
        output.manifest["outputs"]["svg_sha256"],
        format!("{:x}", Sha256::digest(output.svg.as_bytes()))
    );
    assert_eq!(
        output.manifest["source_weight_sum_pct_exact_decimal"],
        "100.0"
    );
    assert_eq!(
        output.manifest["display_weight_sum_pct_exact_decimal"],
        "100.0"
    );
    assert_eq!(output.manifest["objectives"][19]["end_tick"], 1000);
    assert_eq!(output.manifest["zero_weight_ids"], json!([]));
    assert!(render_priority_donut(&roster, 480.).is_err());
    assert!(render_priority_donut(&roster, f64::INFINITY).is_err());
}
#[test]
fn candidate_scores_contributions_and_qualification_cannot_be_ingested() {
    let c = current_priority_contract();
    for key in [
        "normalized_score",
        "weighted_points",
        "fitness_total",
        "individual_2000",
    ] {
        let mut bad = c.clone();
        bad["objectives"][0][key] = json!(74.207299);
        assert!(validate_priority_roster(&bad, RECIPE).is_err());
    }
    let mut bad = c.clone();
    bad["scope"]["specimen"] = json!(2000);
    assert!(validate_priority_roster(&bad, RECIPE).is_err());
    let mut bad = c;
    bad["scope"]["qualification_claim"] = json!(true);
    assert!(validate_priority_roster(&bad, RECIPE).is_err());
    let out = render_priority_donut(&valid(), 514.044).unwrap();
    assert!(out.manifest["specimen"].is_null());
    for key in [
        "normalized_scores_included",
        "contributions_included",
        "fitness_total_included",
        "qualification_claim",
        "gate_free_claim",
    ] {
        assert_eq!(out.manifest[key], false);
    }
    assert!(!out.svg.contains("74.207299"));
    assert!(!out.svg.contains("Individual2000"));
    assert!(out.svg.contains("Reported support/stability proxy"));
    assert!(out.svg.contains("Reported plate-study status"));
}
#[test]
fn one_canonical_shared_svg_is_stable_under_input_row_permutation() {
    let a = render_priority_donut(&valid(), 514.044).unwrap();
    let mut c = current_priority_contract();
    c["objectives"].as_array_mut().unwrap().reverse();
    let b = render_priority_donut(&validate_priority_roster(&c, RECIPE).unwrap(), 514.044).unwrap();
    assert_eq!(a.svg, b.svg);
    assert_eq!(
        a.manifest["outputs"]["svg_sha256"],
        b.manifest["outputs"]["svg_sha256"]
    );
    assert_ne!(
        a.manifest["contract_json_value_sha256"],
        b.manifest["contract_json_value_sha256"]
    );
}
