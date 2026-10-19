//! Retrospective plots of retained verification records; no solver or estimator.
use serde_json::{json, Value};
use std::fmt::Write;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn num(v: &Value) -> Result<f64> {
    v.as_f64()
        .filter(|n| n.is_finite())
        .ok_or_else(|| "missing/nonfinite verification value".into())
}
fn arr(v: &Value) -> Result<&Vec<Value>> {
    v.as_array()
        .ok_or_else(|| "missing verification array".into())
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
fn line(s: &mut String, x1: f64, y1: f64, x2: f64, y2: f64, color: &str, width: f64) {
    writeln!(s,"<path d=\"M{x1},{y1} L{x2},{y2}\" stroke=\"{color}\" stroke-width=\"{width}\" fill=\"none\"/>").unwrap();
}
fn admit(g: &Value, s: &Value, p: &Value, d: &Value, side: &Value, c: &Value) -> Result<()> {
    if c["schema"] != "jarambit.retained_verification_contract.v1"
        || g["schema"] != "jarambit.plate_gci.v1"
        || s["schema"] != "jarambit.stored_solid_tip.v1"
    {
        return Err("unadmitted verification schema".into());
    }
    for doc in [g, s] {
        for key in ["subject", "certificate_identity"] {
            let identity = c[key].as_str().ok_or("missing verification identity")?;
            if doc[key].as_str() != Some(identity) {
                return Err("verification subject/assessment identity differs".into());
            }
        }
    }
    if g["genome_sha256"] != s["genome_sha256"]
        || s["copied_from_another_genome"] != false
        || s["solve_rerun"] != true
        || s["entered_as_solver_run"] != true
    {
        return Err("retained solver-run binding differs".into());
    }
    for (key, source) in [
        ("skfem_raw_sha256", "skfem_raw"),
        ("dolfinx_raw_sha256", "dolfinx_raw"),
    ] {
        if s[key] != c["sources"][source]["sha256"] {
            return Err("retained raw solver binding differs".into());
        }
    }
    let cc = &s["dolfinx_cross_check"];
    if cc["status"] != "verified"
        || cc["within_2pct"] != true
        || cc["mesh_sha256"] != s["mesh_sha256"]
        || s["mesh_sha256"] != c["sources"]["mesh"]["sha256"]
        || side["schema"] != "jarambit.fem_solid.cross_check_sidecar.v1"
        || side["legacy_stdout"] != *d
    {
        return Err("comparison sidecar or mesh binding differs".into());
    }
    let meshes = side["inputs"]
        .as_object()
        .ok_or("missing comparison mesh receipt")?;
    if !meshes.values().any(|v| *v == s["mesh_sha256"])
        || p["solid"]["n_dof"] != d["n_dof"]
        || p["solid"]["n_tet"] != side["observable"]["mesh"]["cells"]
        || side["solver"]["provider"]["versions"]["dolfinx"] != "0.9.0"
    {
        return Err("comparison solver/mesh identity differs".into());
    }
    if side["roi"]["tip_distance"] != json!({"min_inclusive":4.0,"max_exclusive":15.0})
        || side["roi"]["cut_distance"] != json!({"min_exclusive":3.0})
        || side["roi"]["units"] != "mm"
        || side["recovery"]
            != json!([
                "incident_cell_vertex_max.v1",
                "incident_cell_vertex_unweighted_mean.v1"
            ])
    {
        return Err("comparison observable recovery or region differs".into());
    }
    if p["solver"]["converged"] != true
        || num(&p["solver"]["relative_residual"])? <= 0.
        || side["solver"]["ksp_reason"]
            .as_i64()
            .ok_or("missing convergence reason")?
            <= 0
    {
        return Err("retained solver convergence state differs".into());
    }
    for (name, primary) in [
        ("nodal_avg", "SF_solid_nodal_avg"),
        ("nodal_max", "SF_solid_nodal_max"),
    ] {
        let key = format!("SF_{name}");
        let sk = num(&p["solid"][primary])?;
        let dx = num(&d[&key])?;
        if sk <= 0.
            || dx <= 0.
            || s["stored_solid"][format!("SF_skfem_{name}")] != p["solid"][primary]
            || cc[&key] != d[&key]
        {
            return Err("retained scalar pair disagrees".into());
        }
        let reported = num(&cc[format!("rel_err_{name}")])?;
        if reported > 0.02 || (reported - (dx / sk - 1.).abs()).abs() > 0.0000005001 {
            return Err("retained rounded comparison arithmetic differs".into());
        }
    }
    let levels = arr(&g["levels_fine_to_coarse"])?;
    if levels.len() != 3 || g["refinement_ratio_r"] != 2.0 {
        return Err("retained mesh sequence differs".into());
    }
    for (j, level) in levels.iter().enumerate() {
        let h = num(&level["h_mm"])?;
        if h <= 0.
            || (num(&level["max_area_mm2"])? - h * h).abs() > 1e-12
            || (j > 0 && (h / num(&levels[j - 1]["h_mm"])? - 2.).abs() > 1e-12)
        {
            return Err("retained mesh size metadata differs".into());
        }
        for (report, metric) in [
            ("seq_ratio_gci", "seq_ratio"),
            ("sf_hunter_gci", "sf_hunter"),
        ] {
            num(&level[metric])?;
            let r = &g[report];
            let nested = arr(&r["levels"])?;
            if nested.len() != 3
                || nested[j]["h"] != level["h_mm"]
                || nested[j]["mesh"]["sha256"] != level["mesh_sha256"]
                || nested[j]["observable"]["metric"] != metric
                || nested[j]["observable"]["units"] != "ratio"
                || r["sequence_valid"] != true
                || r["observable_valid"] != true
                || r["admitted"] != false
                || r["relative_error_available"] != false
                || !r["gci_fine"].is_null()
                || !r["f_ext"].is_null()
            {
                return Err("unadmitted mesh/observable or error-estimate claim".into());
            }
        }
    }
    if g["seq_ratio_gci"]["fit_status"] != "nonpositive_order"
        || g["sf_hunter_gci"]["fit_status"] != "oscillatory"
        || num(&g["seq_ratio_gci"]["p"])? >= 0.
        || g["seq_ratio_gci"]["monotone"] != true
        || g["sf_hunter_gci"]["monotone"] != false
    {
        return Err("retained GCI interpretation differs".into());
    }
    let bins = arr(&c["profile_bins_mm"])?;
    if bins.len() != 8 {
        return Err("profile bin membership differs".into());
    }
    for kind in ["plate_raw", "solid_nodal_avg"] {
        let values = p["profile_MPa"][kind]
            .as_object()
            .ok_or("missing retained regional profile")?;
        if values.len() != bins.len() {
            return Err("profile bin membership differs".into());
        }
        for (i, bin) in bins.iter().enumerate() {
            let a = bin[0].as_u64().ok_or("invalid profile bin")?;
            let b = bin[1].as_u64().ok_or("invalid profile bin")?;
            if b <= a || (i > 0 && bins[i - 1][1] != bin[0]) {
                return Err("profile bins overlap or have gaps".into());
            }
            if num(&values[&format!("{a}-{b}")])? < 0. {
                return Err("negative retained profile magnitude".into());
            }
        }
    }
    Ok(())
}

pub(crate) fn project(
    g: &Value,
    solid: &Value,
    primary: &Value,
    comparison: &Value,
    side: &Value,
    contract: &Value,
) -> Result<(String, Value)> {
    admit(g, solid, primary, comparison, side, contract)?;
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1200\" height=\"920\" viewBox=\"0 0 1200 920\"><rect width=\"1200\" height=\"920\" fill=\"white\"/><g font-family=\"Arial, sans-serif\" fill=\"#132d3b\">\n");
    label(
        &mut svg,
        20.,
        30.,
        25,
        "RETAINED NUMERICAL VERIFICATION / DIFFERENT QUESTIONS, DIFFERENT OUTCOMES",
    );
    label(
        &mut svg,
        20.,
        58.,
        19,
        &format!(
            "{} / assessed example, not qualified / saved observations only",
            contract["subject"].as_str().ok_or("missing subject")?
        ),
    );
    let levels = arr(&g["levels_fine_to_coarse"])?;
    for (panel, (metric, title, limit, step, status)) in [
        (
            "seq_ratio",
            "A / Regional yield-order proxy",
            2.,
            0.5,
            "No admitted GCI: nonpositive observed order",
        ),
        (
            "sf_hunter",
            "B / Lateral safety-factor observable",
            1.,
            0.25,
            "No admitted GCI: oscillatory sequence",
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let x = 20. + 600. * panel as f64;
        let y = 80.;
        writeln!(
            svg,
            "<rect x=\"{x}\" y=\"{y}\" width=\"560\" height=\"350\" fill=\"#f7f7f2\"/>"
        )
        .unwrap();
        label(&mut svg, x + 15., y + 28., 22, title);
        label(
            &mut svg,
            x + 15.,
            y + 52.,
            17,
            "Dimensionless; source-reported plate levels",
        );
        for j in 0..=4 {
            let value = j as f64 * step;
            let py = y + 245. - value / limit * 155.;
            line(&mut svg, x + 65., py, x + 530., py, "#dce1e2", 1.);
            label(&mut svg, x + 8., py + 5., 16, &format!("{value:.2}"));
        }
        for (j, row) in levels.iter().enumerate() {
            let h = num(&row["h_mm"])?;
            let value = num(&row[metric])?;
            if value < 0. || value > limit {
                return Err("mesh-plot range would clip retained data".into());
            }
            let px = x + 65. + h / 2. * 440.;
            let py = y + 245. - value / limit * 155.;
            writeln!(svg,"<circle id=\"{metric}-{j}\" cx=\"{px}\" cy=\"{py}\" r=\"5\" fill=\"#007e87\" data-value=\"{value}\" data-h=\"{h}\"/>").unwrap();
            label(&mut svg, px - 33., py - 12., 17, &format!("{value:.4}"));
            label(&mut svg, px - 10., y + 270., 17, &format!("{h}"));
        }
        label(
            &mut svg,
            x + 100.,
            y + 298.,
            17,
            "h = sqrt(maximum triangle area), mm",
        );
        label(&mut svg, x + 15., y + 331., 17, status);
    }
    let x = 20.;
    let y = 450.;
    writeln!(
        svg,
        "<rect x=\"{x}\" y=\"{y}\" width=\"560\" height=\"380\" fill=\"#f7f7f2\"/>"
    )
    .unwrap();
    label(
        &mut svg,
        x + 15.,
        y + 28.,
        22,
        "C / Matched-mesh solver observations",
    );
    label(
        &mut svg,
        x + 15.,
        y + 52.,
        17,
        "Safety factors; separately recovered vertex statistics",
    );
    let mut scalar_pairs = Vec::new();
    for (i, (name, primary_key, title)) in [
        ("nodal_avg", "SF_solid_nodal_avg", "Vertex mean"),
        ("nodal_max", "SF_solid_nodal_max", "Vertex max"),
    ]
    .into_iter()
    .enumerate()
    {
        let py = y + 100. + i as f64 * 90.;
        label(&mut svg, x + 15., py, 18, title);
        let a = num(&primary["solid"][primary_key])?;
        let b = num(&comparison[format!("SF_{name}")])?;
        for (j, (v, color)) in [(a, "#007e87"), (b, "#a3442d")].into_iter().enumerate() {
            if v > 1. {
                return Err("solver-plot range would clip retained data".into());
            }
            let yy = py + 15. + j as f64 * 15.;
            let start = x + 170.;
            let end = start + v * 300.;
            writeln!(svg,"<path id=\"solver-{name}-{j}\" d=\"M{start},{yy} L{end},{yy}\" stroke=\"{color}\" stroke-width=\"5\" fill=\"none\" data-value=\"{v}\"/>").unwrap();
            label(&mut svg, x + 480., yy + 5., 16, &format!("{v:.4}"));
        }
        let err = num(&solid["dolfinx_cross_check"][format!("rel_err_{name}")])?;
        label(
            &mut svg,
            x + 170.,
            py + 62.,
            16,
            &format!("Stored relative SF difference: {:.4}%", 100. * err),
        );
        scalar_pairs.push(json!({"observable":name,"primary":a,"comparison":b,"stored_relative_sf_difference":err}));
    }
    for j in 0..=2 {
        let px = x + 170. + j as f64 * 150.;
        line(&mut svg, px, y + 270., px, y + 275., "#132d3b", 1.);
        label(
            &mut svg,
            px - 8.,
            y + 295.,
            16,
            &format!("{:.1}", j as f64 * 0.5),
        );
    }
    label(
        &mut svg,
        x + 15.,
        y + 323.,
        16,
        "Teal: scikit-fem P2 / orange: DOLFINx 0.9.0 P2",
    );
    label(
        &mut svg,
        x + 15.,
        y + 347.,
        16,
        "Stored comparison verified; rounding is retained.",
    );
    label(
        &mut svg,
        x + 15.,
        y + 370.,
        16,
        "Agreement does not establish physical validation.",
    );
    let x = 620.;
    writeln!(
        svg,
        "<rect x=\"{x}\" y=\"{y}\" width=\"560\" height=\"380\" fill=\"#f7f7f2\"/>"
    )
    .unwrap();
    label(
        &mut svg,
        x + 15.,
        y + 28.,
        22,
        "D / Retained regional maxima (MPa)",
    );
    label(
        &mut svg,
        x + 15.,
        y + 52.,
        17,
        "Bin maxima; different model/recovery definitions",
    );
    for j in 0..=3 {
        let value = j as f64 * 400.;
        let yy = y + 245. - value / 1300. * 160.;
        line(&mut svg, x + 65., yy, x + 535., yy, "#dce1e2", 1.);
        label(&mut svg, x + 10., yy + 5., 16, &format!("{value:.0}"));
    }
    let bins = arr(&contract["profile_bins_mm"])?;
    let mut profiles = Vec::new();
    for (kind, color) in [("plate_raw", "#007e87"), ("solid_nodal_avg", "#a3442d")] {
        let mut observations = Vec::new();
        for (i, bin) in bins.iter().enumerate() {
            let a = num(&bin[0])?;
            let b = num(&bin[1])?;
            let key = format!("{a}-{b}");
            let value = num(&primary["profile_MPa"][kind][&key])?;
            if value > 1300. {
                return Err("profile-plot range would clip retained data".into());
            }
            let x1 = x + 65. + (a - 4.) / 21. * 470.;
            let x2 = x + 65. + (b - 4.) / 21. * 470.;
            let yy = y + 245. - value / 1300. * 160.;
            writeln!(svg,"<path id=\"{kind}-{i}\" d=\"M{x1},{yy} L{x2},{yy}\" stroke=\"{color}\" stroke-width=\"4\" data-value=\"{value}\" data-bin=\"{key}\"/>").unwrap();
            observations.push(json!({"bin_mm":[a,b],"value_MPa":value}));
        }
        profiles.push(json!({"series":kind,"observations":observations}));
    }
    for value in [4., 8., 12., 15., 20., 25.] {
        label(
            &mut svg,
            x + 55. + (value - 4.) / 21. * 470.,
            y + 271.,
            16,
            &format!("{value}"),
        );
    }
    label(
        &mut svg,
        x + 75.,
        y + 298.,
        17,
        "Planar distance from saved tip, mm",
    );
    label(
        &mut svg,
        x + 15.,
        y + 323.,
        16,
        "Teal: plate cell peaks, no global correction",
    );
    label(
        &mut svg,
        x + 15.,
        y + 347.,
        16,
        "Orange: solid recovered-vertex mean peaks",
    );
    label(
        &mut svg,
        x + 15.,
        y + 370.,
        16,
        "Not a pointwise solver-difference field.",
    );
    label(&mut svg,20.,865.,18,"Mesh-study metadata are valid; no GCI error estimate is admitted for either plate observable.");
    label(&mut svg,20.,893.,17,"C uses one matched solid mesh. D contains only saved binned maxima; no 3D stress map or error bars are inferred.");
    svg.push_str("</g></svg>\n");
    let summary = json!({"schema":"jarambit.retained_verification_projection.v1","subject":contract["subject"],"certificate_identity":contract["certificate_identity"],"producer_revision":contract["producer_revision"],"mesh_levels":g["levels_fine_to_coarse"],"sequence_admission":g["seq_ratio_gci"],"lateral_admission":g["sf_hunter_gci"],"scalar_pairs":scalar_pairs,"solid_mesh_sha256":solid["mesh_sha256"],"recovery":side["recovery"],"roi":side["roi"],"profiles":profiles,"profile_semantics":{"plate_raw":"maximum saved plate cell response in each planar-distance bin, scaled in the original producer; no global torsion correction","solid_nodal_avg":"maximum incident-cell unweighted-mean vertex response in each planar-distance bin; original cut distance >3 mm"},"plot_limits":{"sequence":[0.,2.],"lateral_sf":[0.,1.],"solver_sf":[0.,1.],"profile_MPa":[0.,1300.]},"interpolation":"none; six individual mesh-level observations and sixteen bin segments","fresh_solver_execution":false,"full_3d_field_available":false});
    Ok((svg, summary))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn records() -> (Vec<Value>, Value) {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let c: Value = serde_json::from_slice(
            &std::fs::read(root.join("docs/paper/verification-inputs.v1.json")).unwrap(),
        )
        .unwrap();
        let records = [
            "plate_gci",
            "solid_tip",
            "skfem_raw",
            "dolfinx_raw",
            "dolfinx_sidecar",
        ]
        .iter()
        .map(|key| {
            serde_json::from_slice(
                &std::fs::read(root.join(c["sources"][key]["path"].as_str().unwrap())).unwrap(),
            )
            .unwrap()
        })
        .collect();
        (records, c)
    }
    #[test]
    fn actual_observations_preserve_refused_estimates_and_bin_membership() {
        let (r, c) = records();
        let (svg, s) = project(&r[0], &r[1], &r[2], &r[3], &r[4], &c).unwrap();
        assert_eq!(s["sequence_admission"]["admitted"], false);
        assert!(s["sequence_admission"]["gci_fine"].is_null());
        assert_eq!(
            s["profiles"][0]["observations"].as_array().unwrap().len(),
            8
        );
        assert_eq!(svg.matches("<circle id=").count(), 6);
        assert_eq!(svg.matches("data-bin=").count(), 16);
    }
    #[test]
    fn forged_binding_region_and_scalar_agreement_are_refused() {
        let (r, c) = records();
        let mut s = r[1].clone();
        s["subject"] = json!("wrong");
        assert!(project(&r[0], &s, &r[2], &r[3], &r[4], &c).is_err());
        s = r[1].clone();
        s["dolfinx_cross_check"]["rel_err_nodal_max"] = json!(0.);
        assert!(project(&r[0], &s, &r[2], &r[3], &r[4], &c).is_err());
        let mut side = r[4].clone();
        side["roi"]["tip_distance"]["min_inclusive"] = json!(0.);
        assert!(project(&r[0], &r[1], &r[2], &r[3], &side, &c).is_err());
    }
    #[test]
    fn invented_uncertainty_and_missing_or_clipped_profile_are_refused() {
        let (r, c) = records();
        let mut g = r[0].clone();
        g["seq_ratio_gci"]["gci_fine"] = json!(0.01);
        assert!(project(&g, &r[1], &r[2], &r[3], &r[4], &c).is_err());
        let mut p = r[2].clone();
        p["profile_MPa"]["solid_nodal_avg"]
            .as_object_mut()
            .unwrap()
            .remove("4-5");
        assert!(project(&r[0], &r[1], &p, &r[3], &r[4], &c).is_err());
        p = r[2].clone();
        p["profile_MPa"]["plate_raw"]["4-5"] = json!(1400.);
        assert!(project(&r[0], &r[1], &p, &r[3], &r[4], &c).is_err());
    }
}
