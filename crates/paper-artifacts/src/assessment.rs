//! Saved assessment scalars and flags. No mechanics, estimator or objective runs.
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fmt::Write;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type FlagGroups = Vec<(String, Vec<(String, bool)>)>;
fn n(v: &Value) -> Result<f64> {
    v.as_f64()
        .filter(|v| v.is_finite())
        .ok_or_else(|| "missing/nonfinite assessment scalar".into())
}
fn t(v: &Value) -> Result<&str> {
    v.as_str()
        .ok_or_else(|| "missing assessment identity".into())
}
fn text(s: &mut String, x: f64, y: f64, size: u32, body: &str) {
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
fn rule(s: &mut String, x1: f64, y1: f64, x2: f64, y2: f64, color: &str, width: f64) {
    writeln!(s,"<path d=\"M{x1},{y1} L{x2},{y2}\" stroke=\"{color}\" stroke-width=\"{width}\" fill=\"none\"/>").unwrap();
}
fn bar(
    s: &mut String,
    id: &str,
    origin: [f64; 2],
    width: f64,
    value: f64,
    limit: f64,
    color: &str,
) -> Result<()> {
    let [x, y] = origin;
    if value < 0. || value > limit {
        return Err("assessment axis would clip source value".into());
    }
    let end = x + width * value / limit;
    writeln!(s,"<path id=\"{id}\" d=\"M{x},{y} L{end},{y}\" stroke=\"{color}\" stroke-width=\"6\" data-value=\"{value}\" data-limit=\"{limit}\"/>").unwrap();
    Ok(())
}
fn admit(a: &Value, r: &Value, s: &Value, c: &Value, catalog: &Value) -> Result<FlagGroups> {
    if c["schema"] != "jarambit.retained_assessment_contract.v1"
        || a["schema"] != "jarambit.side_report.a4.v1"
        || r["requirement_set"] != "RS-karambit-v2"
    {
        return Err("unadmitted assessment schema".into());
    }
    let subject = t(&c["subject"])?;
    let certificate = t(&c["certificate_identity"])?;
    if a["subject"] != subject
        || r["label"] != subject
        || s["subject"] != subject
        || r["certificate_identity"] != certificate
        || s["certificate_identity"] != certificate
        || a["genome_sha256"] != s["genome_sha256"]
        || a["genome"] != r["genome"]
    {
        return Err("assessment subject/content binding differs".into());
    }
    if a["status"] != "pass"
        || a["partial_solver_edit"] != false
        || a["changes_seq_ratio"] != false
        || r["all_hard"] != false
        || s["engineering_metrics"]["promote_eligible"] != false
    {
        return Err("assessment status/qualification interpretation differs".into());
    }
    if a["bore_support"]["model"] != "radial_pin_bearing_and_flange"
        || a["grip_foundation_stiffness"]["model"] != "winkler_grip_pad_foundation"
        || a["eigensolver"] != "scipy.sparse.linalg.eigsh"
        || a["mesh_max_area_mm2"] != 0.25
    {
        return Err("support-model interpretation differs".into());
    }
    for (source, summary) in [
        ("buckling_eigenvalue", "buckling_eigenvalue_a4"),
        ("supported_seq_ratio", "seq_ratio_supported_a4"),
    ] {
        if a[source] != s["engineering_metrics"][summary] {
            return Err("support scalar summary differs".into());
        }
    }
    let ratio = n(&a["supported_blade_peak_mpa_per_n"])? / n(&a["supported_hand_peak_mpa_per_n"])?;
    if (ratio - n(&a["supported_seq_ratio"])?).abs() > 1e-10
        || a["certified_seq_ratio"] != r["seq_ratio"]
    {
        return Err("support ratio association differs".into());
    }
    if r["fat_sf"] != s["engineering_metrics"]["fat_sf_l7"]
        || r["hm_tip_inplane_record_only"] != true
    {
        return Err("fatigue or record-only interpretation differs".into());
    }
    for key in [
        "fat_sf",
        "hm_twist_sf",
        "hm_tip_inplane_sf",
        "hm_spine_sf",
        "hm_ringback_sf",
        "lat_rob_n",
        "strict_lat_rob_0p0625_n",
        "twist_tipzone_sf",
        "twist_tipzone_sf_strict",
    ] {
        if n(&r[key])? < 0. {
            return Err("negative assessment magnitude".into());
        }
    }
    let entries = catalog["entries"]
        .as_array()
        .ok_or("missing assessment catalog")?;
    let mut groups = Vec::new();
    for (kind, actual, expected) in [
        ("G16", &s["weighted_cad_score"]["gates"], 16),
        ("RSV2", &r["gates"], 10),
    ] {
        let members = entries
            .iter()
            .filter(|e| e["kind"] == kind)
            .map(|e| t(&e["id"]).map(str::to_string))
            .collect::<Result<Vec<_>>>()?;
        let wanted: BTreeSet<_> = members.iter().cloned().collect();
        let map = actual.as_object().ok_or("missing exact assessment flags")?;
        if wanted.len() != expected
            || members.len() != expected
            || wanted != map.keys().cloned().collect()
        {
            return Err("assessment flag membership differs".into());
        }
        let mut flags = Vec::new();
        for id in members {
            let passed = if kind == "G16" {
                actual[&id]
                    .as_bool()
                    .ok_or("untyped legacy comparison flag")?
            } else {
                match t(&actual[&id])? {
                    "pass" => true,
                    "fail" => false,
                    _ => return Err("unknown grouped comparison state".into()),
                }
            };
            flags.push((id, passed));
        }
        if kind == "G16"
            && s["weighted_cad_score"]["all_engineering_gates_pass"].as_bool()
                != Some(flags.iter().all(|(_, passed)| *passed))
        {
            return Err("legacy aggregate disagrees with exact flags".into());
        }
        groups.push((kind.to_string(), flags));
    }
    Ok(groups)
}

pub(crate) fn project(
    a: &Value,
    r: &Value,
    s: &Value,
    c: &Value,
    catalog: &Value,
) -> Result<(String, Value)> {
    let groups = admit(a, r, s, c, catalog)?;
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1200\" height=\"900\" viewBox=\"0 0 1200 900\"><rect width=\"1200\" height=\"900\" fill=\"white\"/><g font-family=\"Arial, sans-serif\" fill=\"#132d3b\">\n");
    text(
        &mut svg,
        20.,
        30.,
        25,
        "RETAINED ASSESSMENT SCALARS / ORIGINAL FLAGS ARE NOT QUALIFICATION",
    );
    text(
        &mut svg,
        20.,
        57.,
        19,
        &format!(
            "{} / one assessed example / no new solves or thresholds",
            t(&c["subject"])?
        ),
    );
    for (x, y) in [(20., 75.), (620., 75.), (20., 450.), (620., 450.)] {
        writeln!(
            svg,
            "<rect x=\"{x}\" y=\"{y}\" width=\"560\" height=\"355\" fill=\"#f7f7f2\"/>"
        )
        .unwrap();
    }
    text(
        &mut svg,
        35.,
        105.,
        22,
        "A / Alternate-support model record",
    );
    text(
        &mut svg,
        35.,
        135.,
        17,
        "Model schematic, not a computed field or mode:",
    );
    for (y, title) in [
        (151., "Bore bearing + flange springs"),
        (190., "Winkler grip-pad foundation"),
    ] {
        writeln!(svg,"<rect x=\"45\" y=\"{y}\" width=\"470\" height=\"30\" fill=\"#e4ebec\" stroke=\"#526873\"/>").unwrap();
        text(&mut svg, 58., y + 21., 17, title);
    }
    text(
        &mut svg,
        45.,
        248.,
        18,
        &format!("Recorded eigenvalue: {:.4}", n(&a["buckling_eigenvalue"])?),
    );
    text(
        &mut svg,
        45.,
        277.,
        18,
        &format!(
            "Supported lateral ratio: {:.4}",
            n(&a["supported_seq_ratio"])?
        ),
    );
    text(
        &mut svg,
        45.,
        305.,
        17,
        &format!(
            "Certification multi-case minimum: {:.4}",
            n(&a["certified_seq_ratio"])?
        ),
    );
    text(
        &mut svg,
        35.,
        342.,
        17,
        "Cases, regions and mesh scopes differ; no causal delta.",
    );
    text(
        &mut svg,
        35.,
        371.,
        17,
        "Eigen-operator symmetry/residual audit is open.",
    );
    text(
        &mut svg,
        35.,
        400.,
        17,
        "No retained eigenvector; no mode shape inferred.",
    );
    text(
        &mut svg,
        635.,
        105.,
        22,
        "B / Fatigue and additional response factors",
    );
    let factors = [
        ("fat_sf", "Fatigue model"),
        ("hm_twist_sf", "Twist"),
        ("hm_tip_inplane_sf", "In-plane (record only)"),
        ("hm_spine_sf", "Spine"),
        ("hm_ringback_sf", "Ring-back"),
    ];
    let mut factor_rows = Vec::new();
    for (i, (key, title)) in factors.into_iter().enumerate() {
        let y = 152. + i as f64 * 39.;
        let value = n(&r[key])?;
        text(&mut svg, 635., y + 5., 16, title);
        bar(&mut svg, key, [845., y], 205., value, 5., "#007e87")?;
        text(&mut svg, 1063., y + 5., 17, &format!("{value:.4}"));
        factor_rows.push(
            json!({"key":key,"value":value,"units":"ratio","record_only":key=="hm_tip_inplane_sf"}),
        );
    }
    for v in [0., 1., 3., 5.] {
        let x = 845. + 205. * v / 5.;
        rule(&mut svg, x, 345., x, 350., "#526873", 1.);
        text(&mut svg, x - 5., 371., 16, &format!("{v}"));
    }
    text(
        &mut svg,
        635.,
        400.,
        17,
        "Separate saved cases; no aggregate or fatigue field.",
    );
    text(
        &mut svg,
        35.,
        480.,
        22,
        "C / Certification and strict observations",
    );
    let pairs = [
        (
            "lat_rob_n",
            "strict_lat_rob_0p0625_n",
            200.,
            "Lateral observation (N)",
        ),
        (
            "twist_tipzone_sf",
            "twist_tipzone_sf_strict",
            1.,
            "Local torsion safety factor",
        ),
    ];
    let mut pair_rows = Vec::new();
    for (i, (cert, strict, limit, title)) in pairs.into_iter().enumerate() {
        let y = 516. + i as f64 * 120.;
        text(&mut svg, 35., y, 18, title);
        for (j, (key, title, color)) in [
            (cert, "Certification", "#007e87"),
            (strict, "Strict", "#a3442d"),
        ]
        .into_iter()
        .enumerate()
        {
            let yy = y + 26. + j as f64 * 27.;
            let value = n(&r[key])?;
            text(&mut svg, 40., yy + 5., 16, title);
            bar(&mut svg, key, [190., yy], 250., value, limit, color)?;
            text(&mut svg, 452., yy + 5., 17, &format!("{value:.4}"));
            pair_rows.push(json!({"key":key,"value":value,"limit":limit}));
        }
        for v in [0., limit / 2., limit] {
            text(
                &mut svg,
                185. + 250. * v / limit,
                y + 83.,
                16,
                &format!("{v}"),
            );
        }
    }
    text(
        &mut svg,
        35.,
        778.,
        17,
        "Retained scalars, not an error estimate or strict field.",
    );
    text(
        &mut svg,
        635.,
        480.,
        22,
        "D / Stored comparison flags (overlapping)",
    );
    let mut flag_rows = Vec::new();
    for (g, (kind, flags)) in groups.iter().enumerate() {
        let count = flags.iter().filter(|(_, b)| *b).count();
        text(
            &mut svg,
            635.,
            510. + g as f64 * 168.,
            17,
            &format!(
                "{kind}: {count}/{} met under original comparisons",
                flags.len()
            ),
        );
        for (i, (id, passed)) in flags.iter().enumerate() {
            let cols = if g == 0 { 8 } else { 5 };
            let width = if g == 0 { 61. } else { 98. };
            let x = 638. + (i % cols) as f64 * width;
            let y = 526. + g as f64 * 168. + (i / cols) as f64 * 47.;
            let color = if *passed { "#007e87" } else { "#a3442d" };
            writeln!(svg,"<rect id=\"flag-{kind}-{id}\" x=\"{x}\" y=\"{y}\" width=\"{}\" height=\"30\" fill=\"{color}\" data-passed=\"{passed}\"/>",width-8.).unwrap();
            writeln!(
                svg,
                "<text x=\"{}\" y=\"{}\" font-size=\"16\" fill=\"white\">{:02}</text>",
                x + 10.,
                y + 21.,
                i + 1
            )
            .unwrap();
            flag_rows.push(json!({"kind":kind,"alias":i+1,"id":id,"met":passed}));
        }
    }
    text(
        &mut svg,
        635.,
        632.,
        16,
        "Teal: met / orange: unmet; numbers follow atlas IDs.",
    );
    text(
        &mut svg,
        635.,
        655.,
        16,
        "G16-15: metadata only / G16-13: audit open.",
    );
    text(&mut svg,20.,845.,18,"These source observations do not establish contact calibration, service life, human-safety bounds or qualification.");
    text(&mut svg,20.,875.,17,"Original assessment and scientific code are unchanged. Missing fields and the A4 verification gap remain explicit.");
    svg.push_str("</g></svg>\n");
    let summary = json!({"schema":"jarambit.retained_assessment_projection.v1","subject":c["subject"],"certificate_identity":c["certificate_identity"],"producer_revision":c["producer_revision"],"support":{"eigenvalue":a["buckling_eigenvalue"],"source_status":a["status"],"supported_lateral_ratio":a["supported_seq_ratio"],"certification_multicase_minimum":a["certified_seq_ratio"],"supported_blade_peak_MPa_per_N":a["supported_blade_peak_mpa_per_n"],"supported_hand_peak_MPa_per_N":a["supported_hand_peak_mpa_per_n"],"case_region_comparability":false,"eigen_operator_audit":"open: eigsh applied to K_support^-1 G; Euclidean symmetry and eigenpair residual not established in retained record","eigenvector_available":false},"factors":factor_rows,"certification_strict_pairs":pair_rows,"flags":flag_rows,"qualification":false,"fresh_solver_execution":false,"note":"Scalar plots and a labeled support-model schematic; no synthetic fatigue/stability/strict spatial fields, no aggregation or new thresholds."});
    Ok((svg, summary))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn records() -> (Vec<Value>, Value, Value) {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let c: Value = serde_json::from_slice(
            &std::fs::read(root.join("docs/paper/assessment-inputs.v1.json")).unwrap(),
        )
        .unwrap();
        let docs = ["a4", "rs", "scorecard"]
            .into_iter()
            .map(|k| {
                serde_json::from_slice(
                    &std::fs::read(root.join(c["sources"][k]["path"].as_str().unwrap())).unwrap(),
                )
                .unwrap()
            })
            .collect();
        let catalog =
            serde_json::from_slice(&std::fs::read(root.join("docs/tests/catalog.json")).unwrap())
                .unwrap();
        (docs, c, catalog)
    }
    #[test]
    fn original_flags_and_record_only_semantics_survive_projection() {
        let (r, c, cat) = records();
        let (svg, s) = project(&r[0], &r[1], &r[2], &c, &cat).unwrap();
        assert_eq!(s["flags"].as_array().unwrap().len(), 26);
        assert_eq!(svg.matches("data-passed=").count(), 26);
        assert_eq!(s["factors"][2]["record_only"], true);
        assert_eq!(s["support"]["case_region_comparability"], false);
        assert_eq!(s["qualification"], false);
    }
    #[test]
    fn mismatched_subject_and_forged_summary_are_refused() {
        let (r, c, cat) = records();
        let mut a = r[0].clone();
        a["subject"] = json!("wrong");
        assert!(project(&a, &r[1], &r[2], &c, &cat).is_err());
        a = r[0].clone();
        a["supported_seq_ratio"] = json!(6.);
        assert!(project(&a, &r[1], &r[2], &c, &cat).is_err());
        let mut s = r[2].clone();
        s["engineering_metrics"]["fat_sf_l7"] = json!(1.);
        assert!(project(&r[0], &r[1], &s, &c, &cat).is_err());
        s = r[2].clone();
        s["weighted_cad_score"]["all_engineering_gates_pass"] = json!(true);
        assert!(project(&r[0], &r[1], &s, &c, &cat).is_err());
    }
    #[test]
    fn flags_cannot_be_retyped_or_dropped_and_axes_cannot_hide_high_values() {
        let (r, c, cat) = records();
        let mut s = r[2].clone();
        s["weighted_cad_score"]["gates"]["cad_watertight_and_single_solid"] = json!("true");
        assert!(project(&r[0], &r[1], &s, &c, &cat).is_err());
        s = r[2].clone();
        s["weighted_cad_score"]["gates"]
            .as_object_mut()
            .unwrap()
            .remove("cad_watertight_and_single_solid");
        assert!(project(&r[0], &r[1], &s, &c, &cat).is_err());
        let mut rs = r[1].clone();
        rs["hm_ringback_sf"] = json!(6.);
        assert!(project(&r[0], &rs, &r[2], &c, &cat).is_err());
    }
}
