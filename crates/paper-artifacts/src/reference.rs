//! Retained silhouette and similarity observations; no decoding or measurements.
use serde_json::{json, Value};
use std::fmt::Write;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn n(v: &Value) -> Result<f64> {
    v.as_f64()
        .filter(|x| x.is_finite())
        .ok_or_else(|| "missing/nonfinite reference observation".into())
}
fn points(v: &Value, count: usize) -> Result<Vec<[f64; 2]>> {
    let a = v.as_array().ok_or("missing retained curve")?;
    if a.len() != count {
        return Err("retained curve membership differs".into());
    }
    a.iter()
        .map(|p| {
            if p.as_array().is_none_or(|a| a.len() != 2) {
                return Err("invalid retained point".into());
            }
            Ok([n(&p[0])?, n(&p[1])?])
        })
        .collect()
}
fn label(s: &mut String, x: u32, y: u32, size: u32, body: &str) {
    let body = body
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    writeln!(
        s,
        "<text x=\"{x}\" y=\"{y}\" font-size=\"{size}\">{body}</text>"
    )
    .unwrap();
}
pub(crate) fn project(d: &Value, s: &Value, r: &Value, c: &Value) -> Result<(String, Value)> {
    if c["schema"] != "jarambit.reference_inputs.v1"
        || c["subject"] != "cert_pipeline_r01_p14_c02"
        || c["certificate_identity"]
            != "db5d8e32e99d0b2d8ef6d7c7f9622a0601487c25b80879bbc8be1a502693c55e"
        || c["producer_revision"] != "e8cb049df7d03c7d3113556812cc140ca99a29c3"
        || d["label"] != "cert_pipeline_r01_p14_c02_D1"
        || s["schema"] != "jarambit.master_cad_scorecard.v1"
        || r["requirement_set"] != "RS-karambit-v2"
        || s["subject"] != c["subject"]
        || r["label"] != c["subject"]
        || [d, s, r]
            .iter()
            .any(|v| v["certificate_identity"] != c["certificate_identity"])
        || s["engineering_metrics"]["promote_eligible"] != false
        || r["all_hard"] != false
        || c["comparison_recomputed"] != false
        || c["reference_pointer"] != "/c4"
        || c["reference_interpretation"]
            != "producer c5_ref::EXTERIOR, re-exported by c6_ref; historical JSON key c4"
    {
        return Err("reference identity/semantics binding differs".into());
    }
    let outline = points(&d["outline"], 4230)?;
    let reference = points(&d["c4"], 1117)?;
    let m = &d["metrics"];
    for key in [
        "hausdorff",
        "emd",
        "straight_mm",
        "min_blend_r",
        "aes_score",
    ] {
        if n(&m[key])? < 0. {
            return Err("negative saved reference descriptor".into());
        }
    }
    if n(&m["aes_score"])? > 1.
        || n(&m["emd"])? > 1.
        || m["aes_score"] != s["engineering_metrics"]["aes_score"]
        || m["aes_pass"].as_bool().is_none()
        || m["aes_pass"] != s["engineering_metrics"]["aes_pass"]
        || m["aes_pass"] != s["weighted_cad_score"]["gates"]["c6_aesthetic_floor"]
        || m["aes_pass"] != r["aes_pass"]
        || m["rejects"] != r["rejects"]
    {
        return Err("saved reference summary differs".into());
    }
    for key in ["aes_fails", "rejects"] {
        if m[key]
            .as_array()
            .is_none_or(|a| a.iter().any(|v| v.as_str().is_none()))
        {
            return Err("untyped saved rejection list".into());
        }
    }
    let aes = m["aes_pass"].as_bool().unwrap();
    if aes != m["aes_fails"].as_array().unwrap().is_empty()
        || r["gates"]["aesthetic_and_geometric"]
            != if aes && m["rejects"].as_array().unwrap().is_empty() {
                "pass"
            } else {
                "fail"
            }
    {
        return Err("original reference flags contradict their saved lists".into());
    }
    // One common display transform only: no registration, separate normalization or smoothing.
    let (mut xmin, mut xmax, mut ymin, mut ymax) = (
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
    );
    for [x, y] in outline.iter().chain(reference.iter()) {
        xmin = xmin.min(*x);
        xmax = xmax.max(*x);
        ymin = ymin.min(*y);
        ymax = ymax.max(*y);
    }
    if xmax <= xmin || ymax <= ymin {
        return Err("degenerate reference display".into());
    }
    let scale = (540. / (xmax - xmin)).min(225. / (ymax - ymin));
    let tx = 38. + (540. - scale * (xmax - xmin)) / 2. - scale * xmin;
    let ty = 122. + (225. - scale * (ymax - ymin)) / 2. + scale * ymax;
    let mut svg = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1200\" height=\"490\" viewBox=\"0 0 1200 490\"><rect width=\"1200\" height=\"490\" fill=\"white\"/><g font-family=\"Arial, sans-serif\" fill=\"#132d3b\">\n");
    label(
        &mut svg,
        20,
        34,
        25,
        "Individual 2000 — Silhouette Reference Overlay & Aesthetic Descriptors",
    );
    label(
        &mut svg,
        20,
        66,
        21,
        "Assessed candidate (not qualified) · common isotropic scale without registration",
    );
    label(
        &mut svg,
        20,
        102,
        23,
        "A / Planar silhouette overlay (mm)",
    );
    for (id, curve, color, dash) in [
        ("reference", &reference, "#b65e2c", "9 5"),
        ("outline", &outline, "#126b79", "none"),
    ] {
        write!(
            svg,
            "<path id=\"{id}\" data-source-pointer=\"/{}\" d=\"",
            if id == "reference" { "c4" } else { "outline" }
        )
        .unwrap();
        for (i, [x, y]) in curve.iter().enumerate() {
            write!(
                svg,
                "{} {:.6},{:.6} ",
                if i == 0 { "M" } else { "L" },
                tx + scale * x,
                ty - scale * y
            )
            .unwrap();
        }
        writeln!(svg, "Z\" fill=\"none\" stroke=\"{color}\" stroke-width=\"2.4\" stroke-dasharray=\"{dash}\"/>").unwrap();
    }
    for (x, color, dash, body) in [
        (25, "#126b79", "none", "Individual 2000 outline"),
        (320, "#b65e2c", "9 5", "Target silhouette"),
    ] {
        writeln!(svg, "<path d=\"M{x},382 h40\" stroke=\"{color}\" stroke-width=\"3\" stroke-dasharray=\"{dash}\"/>").unwrap();
        label(&mut svg, x + 50, 389, 21, body);
    }
    label(&mut svg, 625, 102, 23, "B / Saved geometric descriptors");
    let mut observations = Vec::new();
    for (i, (key, name, unit)) in [
        ("hausdorff", "Hausdorff distance", "mm"),
        ("emd", "Curvature-histogram EMD", "unitless"),
        ("straight_mm", "Longest straight segment", "mm"),
        ("min_blend_r", "Minimum fillet blend radius", "mm"),
        ("aes_score", "Composite aesthetic score", "unitless"),
    ]
    .iter()
    .enumerate()
    {
        label(
            &mut svg,
            625,
            142 + i as u32 * 33,
            21,
            &format!("{name}: {} {unit}", m[key]),
        );
        observations.push(json!({"source":c["sources"]["d1"]["path"],"pointer":format!("/metrics/{key}"),"value":m[key],"units":unit,"recomputed":false}));
    }
    label(
        &mut svg,
        625,
        318,
        21,
        &format!("Aesthetic threshold met: {aes} (failures: {})", m["aes_fails"]),
    );
    label(
        &mut svg,
        625,
        351,
        21,
        &format!("Geometric rejection list: {}", m["rejects"]),
    );
    label(
        &mut svg,
        625,
        384,
        21,
        &format!(
            "Aesthetic & geometric group: {}",
            r["gates"]["aesthetic_and_geometric"].as_str().unwrap()
        ),
    );
    label(
        &mut svg,
        20,
        436,
        21,
        "Closed polylines retain all 4,230 candidate points and 1,117 target silhouette points.",
    );
    label(
        &mut svg,
        20,
        470,
        21,
        "Planar silhouette similarity does not establish structural capacity or ergonomic qualification.",
    );
    svg.push_str("</g></svg>\n");
    let receipt = json!({"schema":"jarambit.reference_projection.v1", "subject":c["subject"],"certificate_identity":c["certificate_identity"],"producer_revision":c["producer_revision"],"sources":c["sources"],"producer_sources":c["producer_sources"],"reference_interpretation":c["reference_interpretation"],"curves":[{"source":c["sources"]["d1"]["path"],"pointer":"/outline","serialized_point_count":outline.len()},{"source":c["sources"]["d1"]["path"],"pointer":"/c4","serialized_point_count":reference.len()}],"transform":{"scale":scale,"translate_x":tx,"translate_y":ty,"svg_y_reflection":true,"common_isotropic_scale":true,"registration":false,"smoothing":false,"closure":"Z connects final serialized point to first; points are not added to source"},"observations":observations,"original_flags":{"aes_pass":{"source":c["sources"]["d1"]["path"],"pointer":"/metrics/aes_pass","value":m["aes_pass"]},"aes_fails":{"source":c["sources"]["d1"]["path"],"pointer":"/metrics/aes_fails","value":m["aes_fails"]},"rejects":{"source":c["sources"]["d1"]["path"],"pointer":"/metrics/rejects","value":m["rejects"]},"rs_shape_geometry":{"source":c["sources"]["rs"]["path"],"pointer":"/gates/aesthetic_and_geometric","value":r["gates"]["aesthetic_and_geometric"]}},"fresh_genome_regeneration":false,"comparison_recomputed":false,"fem_field":false,"human_approval":false,"qualification":"not qualified","scope":"Retained 2D silhouettes, original rounded descriptors and flags only; not a reconstructed histogram, full C.6 feature audit or new geometry finding."});
    Ok((svg, receipt))
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Value, Value, Value, Value) {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let c: Value = serde_json::from_slice(
            &std::fs::read(root.join("docs/paper/reference-inputs.v1.json")).unwrap(),
        )
        .unwrap();
        let read = |key: &str| {
            serde_json::from_slice(
                &std::fs::read(root.join(c["sources"][key]["path"].as_str().unwrap())).unwrap(),
            )
            .unwrap()
        };
        (read("d1"), read("scorecard"), read("rs"), c)
    }
    #[test]
    fn preserves_saved_curves_descriptors_and_flags() {
        let (d, s, r, c) = fixture();
        let (svg, p) = project(&d, &s, &r, &c).unwrap();
        assert_eq!(p["curves"][0]["serialized_point_count"], 4230);
        assert_eq!(p["curves"][1]["serialized_point_count"], 1117);
        assert_eq!(p["observations"][0]["value"], 2.285);
        assert_eq!(p["observations"][4]["value"], 0.4079);
        assert_eq!(p["original_flags"]["aes_pass"]["value"], true);
        assert_eq!(svg.matches(" L ").count(), 5345);
        assert!(!svg.contains("histogram bins"));
        assert_eq!(p["comparison_recomputed"], false);
    }
    #[test]
    fn rejects_malformed_curves_and_crossed_identities() {
        let (d, s, r, c) = fixture();
        for key in ["outline", "c4"] {
            let mut bad = d.clone();
            bad[key].as_array_mut().unwrap().pop();
            assert!(project(&bad, &s, &r, &c).is_err());
        }
        let mut bad = d.clone();
        bad["outline"][0] = json!([1, 2, 3]);
        assert!(project(&bad, &s, &r, &c).is_err());
        let mut bad = d.clone();
        bad["outline"][0][0] = Value::Null;
        assert!(project(&bad, &s, &r, &c).is_err());
        let mut bad = r.clone();
        bad["certificate_identity"] = json!("another");
        assert!(project(&d, &s, &bad, &c).is_err());
        let mut bad = c.clone();
        bad["comparison_recomputed"] = json!(true);
        assert!(project(&d, &s, &r, &bad).is_err());
        let mut bad = c.clone();
        bad["reference_interpretation"] = json!("fresh C.6 feature audit");
        assert!(project(&d, &s, &r, &bad).is_err());
    }
    #[test]
    fn rejects_summary_or_flag_contradictions_but_preserves_unfavorable_flags() {
        let (mut d, mut s, mut r, c) = fixture();
        let mut bad = s.clone();
        bad["engineering_metrics"]["aes_score"] = json!(0.9);
        assert!(project(&d, &bad, &r, &c).is_err());
        let mut bad = d.clone();
        bad["metrics"]["aes_fails"] = json!(["saved reason"]);
        assert!(project(&bad, &s, &r, &c).is_err());
        let mut bad = d.clone();
        bad["metrics"]["rejects"] = json!([1]);
        assert!(project(&bad, &s, &r, &c).is_err());
        d["metrics"]["aes_pass"] = json!(false);
        d["metrics"]["aes_fails"] = json!(["saved reason"]);
        s["engineering_metrics"]["aes_pass"] = json!(false);
        s["weighted_cad_score"]["gates"]["c6_aesthetic_floor"] = json!(false);
        r["aes_pass"] = json!(false);
        r["gates"]["aesthetic_and_geometric"] = json!("fail");
        let (_, p) = project(&d, &s, &r, &c).unwrap();
        assert_eq!(p["original_flags"]["aes_pass"]["value"], false);
    }
}
