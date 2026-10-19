//! Six retained reporting series. No pooled experiment, imputation or rescoring.
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const IDS: [&str; 6] = [
    "xbabe1/w501_ul",
    "xbabe1/w501_xb1",
    "xbabe2/w322",
    "xbabe3/w409_ul",
    "xbabe3/w411c3",
    "xbabe3/w411t3",
];
fn text(v: &Value) -> Result<&str> {
    v.as_str()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "missing weighted-summary identity".into())
}
fn number(v: &Value) -> Result<f64> {
    v.as_f64()
        .filter(|f| f.is_finite())
        .ok_or_else(|| "missing/nonfinite weighted-summary value".into())
}

pub(crate) fn project(docs: &BTreeMap<String, Value>, c: &Value) -> Result<Value> {
    if c["schema"] != "jarambit.weighted_summary_inputs.v1"
        || c["metric"] != "W_ULTRALIGHT_v1"
        || c["comparability"] != "not_established"
        || c["point_stage"] != "not_established"
        || !c["producer_revision"].is_null()
        || !c["scorer_revision"].is_null()
        || c["display"]["generation_range"] != json!([0, 2])
        || c["display"]["fitness_range"] != json!([70, 75])
        || c["display"]["fitness_axis_truncated"] != true
        || c["display"]["point_connections"] != "none"
    {
        return Err("unadmitted weighted-summary interpretation/display".into());
    }
    let specs = c["runs"].as_array().ok_or("missing weighted runs")?;
    if specs.len() != 6 || docs.len() != 6 {
        return Err("weighted summary file membership differs".into());
    }
    let mut seen = BTreeSet::new();
    let mut runs = Vec::new();
    let mut total = 0;
    for spec in specs {
        let id = text(&spec["id"])?;
        if !IDS.contains(&id)
            || !seen.insert(id)
            || spec["declared_round_files"] != "absent_in_retained_tree"
        {
            return Err("unknown/duplicate weighted run or changed population availability".into());
        }
        let expected_path =
            format!("evidence/evolution_waves/2026-10-18_ultralight_v1/{id}/fitness_curve.json");
        if spec["path"] != expected_path {
            return Err("weighted run/path identity differs".into());
        }
        let d = docs.get(id).ok_or("missing admitted weighted run")?;
        if d["schema"] != "jarambit.ultralight_fitness_curve.v1"
            || d["monotone_non_decreasing"] != true
        {
            return Err("wrong weighted-summary schema/recurrence declaration".into());
        }
        crate::publication::validate_curve(d)?;
        let rows = d["generations"]
            .as_array()
            .ok_or("missing weighted generation rows")?;
        if spec["generation_rows"].as_u64() != Some(rows.len() as u64) {
            return Err("weighted generation membership differs".into());
        }
        let last = rows.last().ok_or("empty weighted run")?;
        if text(&d["best_so_far"]["label"])? != text(&last["best_so_far_label"])?
            || (number(&d["best_so_far"]["ultralight_weighted_fitness"])?
                - number(&last["hall_of_fame_best_so_far"])?)
            .abs()
                > 0.0000005
        {
            return Err("weighted summary final identity/score contradicts headline".into());
        }
        let mut points = Vec::new();
        for (i, row) in rows.iter().enumerate() {
            let generation = row["generation"]
                .as_u64()
                .ok_or("untyped summary generation")?;
            let round = number(&row["round_best_fitness"])?;
            let archive = number(&row["hall_of_fame_best_so_far"])?;
            if generation > 2
                || !(70.0..=75.0).contains(&round)
                || !(70.0..=75.0).contains(&archive)
            {
                return Err("weighted display would omit/clip a saved observation".into());
            }
            points.push(json!({"source_pointer":format!("/generations/{i}"),"generation":generation,
                "round_best_fitness":round,"hall_of_fame_best_so_far":archive,
                "best_so_far_label":text(&row["best_so_far_label"])?,"declared_round_file":text(&row["round_file"])?,
                "point_stage":"not_established","producer_revision":null,"scorer_revision":null}));
        }
        total += points.len();
        runs.push(json!({"id":id,"source":spec["path"],"source_sha256":spec["sha256"],"generation_rows":points.len(),
            "stored_best_label":d["best_so_far"]["label"],"stored_best_score":d["best_so_far"]["ultralight_weighted_fitness"],
            "stored_qualification_status":text(&d["best_so_far"]["qualification_status"])?,
            "qualification_status_pointer":"/best_so_far/qualification_status","points":points}));
    }
    if total != 15 {
        return Err("admitted summary-row total differs".into());
    }
    Ok(
        json!({"schema":"jarambit.weighted_summary_projection.v1","metric":"W_ULTRALIGHT_v1",
        "scope":"Retained reporting summaries, not full populations or homogeneous-objective convergence",
        "run_files":runs.len(),"generation_summaries":total,"plotted_observations":total*2,"runs":runs,
        "display":c["display"],"comparability":"not_established","point_stage":"not_established",
        "producer_revision":null,"scorer_revision":null,"point_connections":"none","imputation":"none",
        "declared_round_files_present":0,"independent_replications_established":false,
        "metadata_note":"Null revision fields mean not established by the admitted summaries; no revision or stage is invented from the metric name or filename."}),
    )
}

fn label(s: &mut String, x: f64, y: f64, size: u32, body: &str) {
    let escaped = body
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    writeln!(
        s,
        "<text x=\"{x}\" y=\"{y}\" font-size=\"{size}\">{escaped}</text>"
    )
    .unwrap();
}
pub(crate) fn svg(p: &Value) -> Result<String> {
    if p["schema"] != "jarambit.weighted_summary_projection.v1"
        || p["comparability"] != "not_established"
    {
        return Err("unadmitted weighted plot projection".into());
    }
    let mut s=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1200\" height=\"620\" viewBox=\"0 0 1200 620\"><rect width=\"1200\" height=\"620\" fill=\"white\"/><g font-family=\"Arial, sans-serif\" fill=\"#132d3b\">\n");
    label(
        &mut s,
        20.,
        29.,
        24,
        "RETAINED WEIGHTED SUMMARIES / SIX SEPARATE RUN FILES",
    );
    label(&mut s,20.,57.,19,"15 saved generation summaries / no full round populations / point-level stage lineage unresolved");
    writeln!(s,"<circle cx=\"29\" cy=\"82\" r=\"7\" stroke=\"#007e87\" stroke-width=\"3\" fill=\"none\"/><path d=\"M370,76 L382,88 M370,88 L382,76\" stroke=\"#d07b42\" stroke-width=\"3\"/>").unwrap();
    label(&mut s, 45., 88., 18, "Archive (stored Hall of Fame)");
    label(&mut s, 392., 88., 18, "Round best");
    label(
        &mut s,
        665.,
        88.,
        18,
        "Symbols overlap when the saved values coincide",
    );
    let runs = p["runs"].as_array().ok_or("missing weighted plot runs")?;
    if runs.len() != 6 {
        return Err("weighted plot facet membership differs".into());
    }
    for (i, run) in runs.iter().enumerate() {
        let x = 20. + 400. * (i % 3) as f64;
        let y = 105. + 225. * (i / 3) as f64;
        writeln!(
            s,
            "<rect x=\"{x}\" y=\"{y}\" width=\"380\" height=\"215\" fill=\"#f7f7f2\"/>"
        )
        .unwrap();
        let id = text(&run["id"])?;
        label(
            &mut s,
            x + 12.,
            y + 27.,
            20,
            &format!("{id} / n={}", run["generation_rows"]),
        );
        let sx = |g: f64| x + 55. + 305. * g / 2.;
        let sy = |v: f64| y + 155. - 100. * (v - 70.) / 5.;
        for tick in [70., 72.5, 75.] {
            let py = sy(tick);
            label(&mut s, x + 9., py + 6., 17, &format!("{tick}"));
            writeln!(
                s,
                "<path d=\"M{},{} H{}\" stroke=\"#d5dfdf\"/>",
                sx(0.),
                py,
                sx(2.)
            )
            .unwrap();
        }
        for g in 0..=2 {
            label(&mut s, sx(g as f64) - 5., y + 177., 18, &g.to_string());
        }
        let points = run["points"]
            .as_array()
            .ok_or("missing weighted plot points")?;
        for point in points {
            let generation = number(&point["generation"])?;
            let px = sx(generation);
            for (field, series) in [
                ("hall_of_fame_best_so_far", "archive"),
                ("round_best_fitness", "round"),
            ] {
                let value = number(&point[field])?;
                if !(0.0..=2.0).contains(&generation) || !(70.0..=75.0).contains(&value) {
                    return Err("weighted marker would be clipped".into());
                }
                let py = sy(value);
                let selector = format!("{}/{}", text(&point["source_pointer"])?, field);
                let attrs=format!("data-run=\"{id}\" data-series=\"{series}\" data-source-pointer=\"{selector}\" data-generation=\"{generation}\" data-value=\"{value}\"");
                if series == "archive" {
                    writeln!(s,"<circle {attrs} cx=\"{px}\" cy=\"{py}\" r=\"7\" stroke=\"#007e87\" stroke-width=\"3\" fill=\"none\"/>").unwrap();
                } else {
                    writeln!(s,"<path {attrs} d=\"M{},{} L{},{} M{},{} L{},{}\" stroke=\"#d07b42\" stroke-width=\"3\"/>",px-5.,py-5.,px+5.,py+5.,px-5.,py+5.,px+5.,py-5.).unwrap();
                }
            }
        }
        let last = points.last().ok_or("empty weighted plot facet")?;
        label(
            &mut s,
            x + 12.,
            y + 204.,
            16,
            &format!(
                "Last round {:.6} / archive {:.6}",
                number(&last["round_best_fitness"])?,
                number(&last["hall_of_fame_best_so_far"])?
            ),
        );
    }
    label(&mut s,20.,573.,19,"Vertical: reported fitness, 70-75 (truncated). Horizontal: stored generation, 0-2. Higher is better.");
    label(&mut s,20.,603.,18,"No interpolation or pooled trajectory. A shared metric name does not establish comparable scorer/stage inputs.");
    s.push_str("</g></svg>\n");
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (BTreeMap<String, Value>, Value) {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let c: Value = serde_json::from_slice(
            &std::fs::read(root.join("docs/paper/weighted-inputs.v1.json")).unwrap(),
        )
        .unwrap();
        let docs = c["runs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                (
                    r["id"].as_str().unwrap().to_string(),
                    serde_json::from_slice(
                        &std::fs::read(root.join(r["path"].as_str().unwrap())).unwrap(),
                    )
                    .unwrap(),
                )
            })
            .collect();
        (docs, c)
    }
    #[test]
    fn preserves_all_summaries_and_does_not_extend_the_single_point_run() {
        let (docs, c) = fixture();
        let p = project(&docs, &c).unwrap();
        let s = svg(&p).unwrap();
        assert_eq!(p["generation_summaries"], 15);
        assert_eq!(p["plotted_observations"], 30);
        let one = p["runs"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["id"] == "xbabe1/w501_xb1")
            .unwrap();
        assert_eq!(one["points"].as_array().unwrap().len(), 1);
        assert_eq!(s.matches("data-run=\"xbabe1/w501_xb1\"").count(), 2);
        assert_eq!(s.matches("data-run=").count(), 30);
        assert!(!s.contains("<polyline"));
        assert!(p["producer_revision"].is_null());
        assert_eq!(p["comparability"], "not_established");
    }
    #[test]
    fn rejects_cherry_picked_membership_and_invented_comparability() {
        let (docs, c) = fixture();
        let mut missing = docs.clone();
        missing.remove(IDS[0]);
        assert!(project(&missing, &c).is_err());
        let mut duplicate = c.clone();
        duplicate["runs"][1] = duplicate["runs"][0].clone();
        assert!(project(&docs, &duplicate).is_err());
        let mut homogeneous = c.clone();
        homogeneous["comparability"] = json!("homogeneous");
        assert!(project(&docs, &homogeneous).is_err());
        let mut invented = c.clone();
        invented["scorer_revision"] = json!("made_up_revision");
        assert!(project(&docs, &invented).is_err());
    }
    #[test]
    fn rejects_contradictory_headline_recurrence_and_display_clipping() {
        let (docs, c) = fixture();
        let mut bad = docs.clone();
        bad.get_mut(IDS[4]).unwrap()["best_so_far"]["label"] = json!("wrong_identity");
        assert!(project(&bad, &c).is_err());
        let mut bad = docs.clone();
        bad.get_mut(IDS[4]).unwrap()["generations"][2]["hall_of_fame_best_so_far"] = json!(74.0);
        assert!(project(&bad, &c).is_err());
        let mut bad = docs.clone();
        let d = bad.get_mut(IDS[1]).unwrap();
        d["generations"][0]["round_best_fitness"] = json!(69.);
        d["generations"][0]["hall_of_fame_best_so_far"] = json!(69.);
        d["best_so_far"]["ultralight_weighted_fitness"] = json!(69.);
        assert!(project(&bad, &c).is_err());
        let mut bad = docs.clone();
        bad.get_mut(IDS[1]).unwrap()["generations"][0]["generation"] = json!(3);
        assert!(project(&bad, &c).is_err());
    }
}
