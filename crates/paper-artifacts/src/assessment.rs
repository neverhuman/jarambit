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
    let _subject_internal = t(&c["subject"])?;
    let wcad = &s["weighted_cad_score"];
    let wcad_total = n(&wcad["W_CAD"])?;
    let wcad_unpen = n(&wcad["unpenalized_quality_score"])?;
    let wcad_pen = n(&wcad["components"]["shortfall_penalty"])?;
    let wcad_base = n(&wcad["components"]["base_feasible"])?;

    let subscore_specs = [
        ("s_mass", "w_mass", "Mass efficiency (w=0.70)"),
        ("s_karambit", "w_karambit", "Ring & claw form (w=0.12)"),
        (
            "s_survival_knee",
            "w_survival_buffer_knee_8pct",
            "Refined survival knee (w=0.10)",
        ),
        (
            "s_seq_knee",
            "w_seq_buffer_knee_1p28",
            "Tip-first yield knee (w=0.05)",
        ),
        (
            "s_carry_knee",
            "w_carry_stability",
            "Carry compactness (w=0.03)",
        ),
    ];
    let mut subscore_rows = Vec::new();
    let mut weighted_sub_sum = 0.0;
    for (s_key, w_key, title) in subscore_specs {
        let sv = n(&wcad["normalized_subscores"][s_key])?;
        let wv = n(&wcad["weights"][w_key])?;
        if !(0.0..=1.0).contains(&sv) || !(0.0..=1.0).contains(&wv) {
            return Err("invalid normalized subscore or weight in weighted_cad_score".into());
        }
        weighted_sub_sum += sv * wv;
        subscore_rows.push(json!({
            "key": s_key,
            "weight_key": w_key,
            "title": title,
            "normalized_subscore": sv,
            "weight": wv,
            "weighted_contribution": sv * wv
        }));
    }
    if ((1.0 + weighted_sub_sum) - wcad_unpen).abs() > 1e-5 {
        return Err("weighted_cad_score unpenalized_quality_score arithmetic mismatch".into());
    }

    let s_mass_val = n(&wcad["normalized_subscores"]["s_mass"])?;
    if ((wcad_base - wcad_pen + 0.1 * s_mass_val) - wcad_total).abs() > 1e-5 {
        return Err("weighted_cad_score failed-branch W_CAD arithmetic mismatch".into());
    }

    let shortfall_specs = [
        ("twist_tip_strict", "Refined tip torsion shortfall"),
        ("hunter_strict", "Refined plate bending shortfall"),
        ("solid_tip_3d", "Quadratic solid tip shortfall"),
    ];
    let mut shortfall_rows = Vec::new();
    for (sf_key, title) in shortfall_specs {
        let sfv = n(&wcad["shortfalls"][sf_key])?;
        if !(0.0..=1.0).contains(&sfv) {
            return Err("invalid shortfall magnitude in weighted_cad_score".into());
        }
        shortfall_rows.push(json!({
            "key": sf_key,
            "title": title,
            "shortfall": sfv
        }));
    }

    let mut flag_rows = Vec::new();
    for (kind, flags) in &groups {
        for (i, (id, passed)) in flags.iter().enumerate() {
            flag_rows.push(json!({"kind":kind,"alias":i+1,"id":id,"met":passed}));
        }
    }

    let mut svg = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1200\" height=\"1180\" viewBox=\"0 0 1200 1180\"><rect width=\"1200\" height=\"1180\" fill=\"white\"/><g font-family=\"Arial, sans-serif\" fill=\"#132d3b\">\n");
    text(
        &mut svg,
        20.,
        36.,
        25,
        "Individual 2000 — Retained Structural Scalars & Historical CAD Subscores",
    );
    text(
        &mut svg,
        20.,
        68.,
        21,
        "Assessed candidate (not qualified) · saved solver observations & historical shortfall branch",
    );
    for (x, y) in [(20., 90.), (620., 90.), (20., 595.), (620., 595.)] {
        writeln!(
            svg,
            "<rect x=\"{x}\" y=\"{y}\" width=\"560\" height=\"485\" fill=\"#f7f7f2\" stroke=\"#d5dfdf\"/>"
        )
        .unwrap();
    }
    text(
        &mut svg,
        35.,
        125.,
        23,
        "A / Alternate-support buckling model",
    );
    text(
        &mut svg,
        35.,
        158.,
        21,
        "Support schematic (no mode shape retained):",
    );
    for (y, title) in [
        (174., "Radial pin bearing + flange springs"),
        (222., "Winkler grip-pad elastic foundation"),
    ] {
        writeln!(svg,"<rect x=\"42\" y=\"{y}\" width=\"510\" height=\"38\" fill=\"#e4ebec\" stroke=\"#526873\"/>").unwrap();
        text(&mut svg, 56., y + 26., 21, title);
    }
    text(
        &mut svg,
        42.,
        296.,
        22,
        &format!("Buckling eigenvalue: {:.4}", n(&a["buckling_eigenvalue"])?),
    );
    text(
        &mut svg,
        42.,
        332.,
        22,
        &format!(
            "Supported yield-order ratio: {:.4}",
            n(&a["supported_seq_ratio"])?
        ),
    );
    text(
        &mut svg,
        42.,
        368.,
        21,
        &format!(
            "Multi-case minimum ratio: {:.4}",
            n(&a["certified_seq_ratio"])?
        ),
    );
    text(
        &mut svg,
        35.,
        416.,
        21,
        "Load cases, regions & mesh scopes differ.",
    );
    text(
        &mut svg,
        35.,
        448.,
        21,
        "Generalized eigen-operator symmetry open.",
    );
    text(
        &mut svg,
        35.,
        480.,
        21,
        "No spatial eigenvector retained in archive.",
    );

    text(
        &mut svg,
        635.,
        125.,
        23,
        "B / Cyclic fatigue & overload factors",
    );
    let factors = [
        ("fat_sf", "Goodman fatigue"),
        ("hm_twist_sf", "Torsion overload"),
        ("hm_tip_inplane_sf", "In-plane (record)"),
        ("hm_spine_sf", "Spine bending"),
        ("hm_ringback_sf", "Ring traction"),
    ];
    let mut factor_rows = Vec::new();
    for (i, (key, title)) in factors.into_iter().enumerate() {
        let y = 182. + i as f64 * 52.;
        let value = n(&r[key])?;
        text(&mut svg, 635., y + 7., 21, title);
        bar(&mut svg, key, [835., y], 215., value, 5., "#007e87")?;
        text(&mut svg, 1062., y + 7., 21, &format!("{value:.4}"));
        factor_rows.push(
            json!({"key":key,"value":value,"units":"ratio","record_only":key=="hm_tip_inplane_sf"}),
        );
    }
    for v in [0., 1., 3., 5.] {
        let x = 835. + 215. * v / 5.;
        rule(&mut svg, x, 435., x, 443., "#526873", 1.5);
        text(&mut svg, x - 8., 470., 21, &format!("{v}"));
    }
    text(
        &mut svg,
        635.,
        516.,
        21,
        "Separate load cases; no spatial fatigue field.",
    );

    text(
        &mut svg,
        35.,
        630.,
        23,
        "C / Coarse vs. refined mesh scalars",
    );
    let pairs = [
        (
            "lat_rob_n",
            "strict_lat_rob_0p0625_n",
            300.,
            "Out-of-plane plate bending capacity (N)",
        ),
        (
            "twist_tipzone_sf",
            "twist_tipzone_sf_strict",
            1.,
            "Local tip torsion safety factor",
        ),
    ];
    let mut pair_rows = Vec::new();
    for (i, (cert, strict, limit, title)) in pairs.into_iter().enumerate() {
        let y = 676. + i as f64 * 165.;
        text(&mut svg, 35., y, 22, title);
        for (j, (key, label_txt, color)) in [
            (cert, "Coarse mesh", "#007e87"),
            (strict, "Refined mesh", "#a3442d"),
        ]
        .into_iter()
        .enumerate()
        {
            let yy = y + 34. + j as f64 * 38.;
            let value = n(&r[key])?;
            text(&mut svg, 40., yy + 7., 21, label_txt);
            bar(&mut svg, key, [195., yy], 240., value, limit, color)?;
            text(&mut svg, 448., yy + 7., 21, &format!("{value:.4}"));
            pair_rows.push(json!({"key":key,"value":value,"limit":limit}));
        }
        for v in [0., limit / 2., limit] {
            text(
                &mut svg,
                185. + 240. * v / limit,
                y + 114.,
                21,
                &format!("{v}"),
            );
        }
    }
    text(
        &mut svg,
        35.,
        1045.,
        21,
        "Saved scalars only; no spatial strict-mesh field.",
    );

    text(
        &mut svg,
        635.,
        630.,
        23,
        "D / Historical CAD subscores & shortfalls",
    );
    text(
        &mut svg,
        635.,
        662.,
        21,
        &format!(
            "Branch W_CAD = {wcad_total:.6} (unpenalized = +{wcad_unpen:.4})"
        ),
    );
    for (i, row) in subscore_rows.iter().enumerate() {
        let y = 702. + i as f64 * 38.;
        let key = t(&row["key"])?;
        let title = t(&row["title"])?;
        let sv = n(&row["normalized_subscore"])?;
        let wv = n(&row["weight"])?;
        text(&mut svg, 635., y + 7., 21, title);
        let end = 925. + 130. * sv;
        writeln!(
            svg,
            "<path id=\"subscore-{key}\" d=\"M925,{y} L{end},{y}\" stroke=\"#007e87\" stroke-width=\"8\" data-subscore=\"{key}\" data-value=\"{sv}\" data-weight=\"{wv}\"/>"
        )
        .unwrap();
        text(&mut svg, 1065., y + 7., 21, &format!("{sv:.4}"));
    }
    text(
        &mut svg,
        635.,
        908.,
        22,
        "Non-zero constraint shortfalls (7 others = 0):",
    );
    for (i, row) in shortfall_rows.iter().enumerate() {
        let y = 944. + i as f64 * 38.;
        let key = t(&row["key"])?;
        let title = t(&row["title"])?;
        let sfv = n(&row["shortfall"])?;
        text(&mut svg, 635., y + 7., 21, title);
        let end = 925. + 130. * sfv;
        writeln!(
            svg,
            "<path id=\"shortfall-{key}\" d=\"M925,{y} L{end},{y}\" stroke=\"#a3442d\" stroke-width=\"8\" data-shortfall=\"{key}\" data-value=\"{sfv}\"/>"
        )
        .unwrap();
        text(&mut svg, 1065., y + 7., 21, &format!("{sfv:.4}"));
    }
    text(
        &mut svg,
        635.,
        1055.,
        21,
        &format!("Failed branch: base {wcad_base:.1} - pen {wcad_pen:.4} + 0.1·s_mass."),
    );

    text(
        &mut svg,
        20.,
        1118.,
        22,
        "Saved structural observations do not establish contact calibration, service life, or qualification.",
    );
    text(
        &mut svg,
        20.,
        1154.,
        21,
        "All panels bind Individual 2000; 20-term weighted profile of Individual 2002 (Run 31) is in fitness.svg.",
    );
    svg.push_str("</g></svg>\n");
    let summary = json!({"schema":"jarambit.retained_assessment_projection.v1","subject":c["subject"],"certificate_identity":c["certificate_identity"],"producer_revision":c["producer_revision"],"support":{"eigenvalue":a["buckling_eigenvalue"],"source_status":a["status"],"supported_lateral_ratio":a["supported_seq_ratio"],"certification_multicase_minimum":a["certified_seq_ratio"],"supported_blade_peak_MPa_per_N":a["supported_blade_peak_mpa_per_n"],"supported_hand_peak_MPa_per_N":a["supported_hand_peak_mpa_per_n"],"case_region_comparability":false,"eigen_operator_audit":"open: eigsh applied to K_support^-1 G; Euclidean symmetry and eigenpair residual not established in retained record","eigenvector_available":false},"factors":factor_rows,"certification_strict_pairs":pair_rows,"weighted_cad_score_breakdown":{"W_CAD":wcad_total,"unpenalized_quality_score":wcad_unpen,"shortfall_penalty":wcad_pen,"base_feasible":wcad_base,"failed_branch_formula":"base_feasible (-1.0) - shortfall_penalty (4.619108) + 0.1 * s_mass (0.048700) = -5.570408 (conditional failed branch, not unpenalized_quality_score - shortfall_penalty)","subscores":subscore_rows,"nonzero_shortfalls":shortfall_rows},"flags":flag_rows,"qualification":false,"fresh_solver_execution":false,"note":"Scalar plots, historical CAD subscores/shortfalls, and a labeled support-model schematic; no synthetic fatigue/stability/strict spatial fields, no aggregation or new thresholds."});
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
        assert_eq!(svg.matches("data-subscore=").count(), 5);
        assert_eq!(svg.matches("data-shortfall=").count(), 3);
        assert!(!svg.contains("cert_pipeline_r01_p14_c02"));
        assert!(!svg.contains("G16"));
        assert!(!svg.contains("RSV2"));
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
