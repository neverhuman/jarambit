//! Deterministic publication projections. No scientific evaluator is invoked.
use crate::validation::{validate_weighted_report, REPORT_GROUPS};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const BASE: &str = "009f5ac520fa3afe8ed8025230eef604dbb7f922";
pub(crate) const CURVE: &str =
    "evidence/evolution_waves/2026-10-18_ultralight_v1/xbabe3/w411c3/fitness_curve.json";
const POP: &str = "evidence/rs_v2/w282_ultrarunner/search/all_evals.json";

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn text(v: &Value) -> Result<&str> {
    v.as_str().ok_or_else(|| "expected string".into())
}
fn array(v: &Value) -> Result<&Vec<Value>> {
    v.as_array().ok_or_else(|| "expected array".into())
}
fn number(v: &Value) -> Result<f64> {
    v.as_f64()
        .filter(|n| n.is_finite())
        .ok_or_else(|| "expected finite number".into())
}
fn safe_path(root: &Path, rel: &str) -> Result<std::path::PathBuf> {
    let p = Path::new(rel);
    if p.components().any(|c| !matches!(c, Component::Normal(_))) || p.as_os_str().is_empty() {
        return Err(format!("unsafe relative path: {rel}").into());
    }
    let mut path = root.to_path_buf();
    for c in p.components() {
        path.push(c);
        if path
            .symlink_metadata()
            .is_ok_and(|m| m.file_type().is_symlink())
        {
            return Err(format!("symlink in publication path: {rel}").into());
        }
    }
    Ok(path)
}
fn read(root: &Path, rel: &str, inputs: &mut BTreeMap<String, String>) -> Result<Value> {
    let raw = fs::read(safe_path(root, rel)?)?;
    inputs.insert(rel.to_string(), digest(&raw));
    Ok(serde_json::from_slice(&raw)?)
}
fn unique<'a>(ids: impl Iterator<Item = &'a str>) -> Result<BTreeSet<String>> {
    let mut seen = BTreeSet::new();
    for id in ids {
        if !seen.insert(id.to_string()) {
            return Err(format!("duplicate identity: {id}").into());
        }
    }
    Ok(seen)
}
pub(crate) fn median(values: &[f64]) -> Result<f64> {
    if values.is_empty() || values.iter().any(|v| !v.is_finite()) {
        return Err("invalid median sample".into());
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let n = sorted.len();
    Ok(if n % 2 == 0 {
        sorted[n / 2 - 1] / 2.0 + sorted[n / 2] / 2.0
    } else {
        sorted[n / 2]
    })
}
fn xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
fn svg_start(w: usize, h: usize) -> String {
    format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\"><rect width=\"100%\" height=\"100%\" fill=\"#f5f4ef\"/><g font-family=\"Arial, sans-serif\" fill=\"#132d3b\">\n")
}
// Only display coordinates are quantized. Retained values and reductions stay unchanged.
// libm transforms can differ by an ulp across platforms; 0.001 SVG unit is subpixel.
fn svg_coord(value: f64) -> String {
    format!("{value:.3}")
}
fn label(s: &mut String, x: f64, y: f64, size: usize, content: &str, color: &str) {
    let (x, y) = (svg_coord(x), svg_coord(y));
    s.push_str(&format!(
        "<text x=\"{x}\" y=\"{y}\" font-size=\"{size}\" fill=\"{color}\">{}</text>\n",
        xml(content)
    ));
}

/// Verify the declared metric and the exact retained Hall-of-Fame recurrence.
pub(crate) fn validate_curve(curve: &Value) -> Result<()> {
    if text(&curve["metric"])? != "W_ULTRALIGHT_v1" {
        return Err("wrong metric regime".into());
    }
    let rows = array(&curve["generations"])?;
    if rows.is_empty() {
        return Err("empty curve".into());
    }
    let mut best = f64::NEG_INFINITY;
    let mut generations = BTreeSet::new();
    let mut previous = None;
    for row in rows {
        let g = row["generation"].as_u64().ok_or("invalid generation")?;
        if !generations.insert(g) || previous.is_some_and(|p| g <= p) {
            return Err("duplicate or unordered generation".into());
        }
        previous = Some(g);
        let round = number(&row["round_best_fitness"])?;
        let hof = number(&row["hall_of_fame_best_so_far"])?;
        if !(0.0..=100.0).contains(&round) || !(0.0..=100.0).contains(&hof) {
            return Err("fitness out of range".into());
        }
        best = best.max(round);
        if (best - hof).abs() > 0.000002 {
            return Err("Hall-of-Fame recurrence mismatch".into());
        }
    }
    Ok(())
}

fn validate_immutable_scene(scene: &Value) -> Result<()> {
    if scene["geometry_changed"] != false
        || scene["mesh_modifiers"]
            .as_array()
            .is_none_or(|v| !v.is_empty())
    {
        return Err("cover scene must preserve the imported mesh".into());
    }
    for (before, after) in [
        (
            "imported_vertices_before_sha256",
            "imported_vertices_after_sha256",
        ),
        ("faces_before_sha256", "faces_after_sha256"),
    ] {
        let hash = text(&scene[before])?;
        if hash.len() != 64
            || !hash.bytes().all(|b| b.is_ascii_hexdigit())
            || scene[before] != scene[after]
        {
            return Err("cover scene geometry digests differ".into());
        }
    }
    Ok(())
}

fn admit_cover_render(root: &Path, inputs: &mut BTreeMap<String, String>) -> Result<()> {
    let spec = read(root, "docs/paper/figure-inputs.v1.json", inputs)?;
    let proof = read(root, "evidence/evolve_certifiable/manifest.json", inputs)?;
    let render = read(
        root,
        "docs/paper/figures/v2/hero_dark.manifest.json",
        inputs,
    )?;
    let scene = read(root, "docs/paper/figures/v2/hero_dark.scene.json", inputs)?;
    for path in [
        "docs/paper/figure-inputs.v1.json",
        "ops/render/blender/candidate_scenes.py",
        "ops/render/blender/rerender.py",
        "ops/render/blender/paper_scenes.py",
    ] {
        if render["inputs"][path].as_str().is_none() {
            return Err(format!("missing cover source binding: {path}").into());
        }
    }
    for (path, expected) in render["inputs"]
        .as_object()
        .ok_or("missing render inputs")?
    {
        let actual = digest(&fs::read(safe_path(root, path)?)?);
        if expected.as_str() != Some(actual.as_str()) {
            return Err(format!("cover render input changed: {path}").into());
        }
        inputs.insert(path.clone(), actual);
    }
    for name in ["hero_dark.png", "hero_dark.scene.json"] {
        let path = format!("docs/paper/figures/v2/{name}");
        let actual = digest(&fs::read(safe_path(root, &path)?)?);
        if render["outputs"][name].as_str() != Some(actual.as_str()) {
            return Err(format!("cover render output changed: {name}").into());
        }
        inputs.insert(path, actual);
    }
    for (path, expected) in spec["inputs"].as_object().ok_or("missing figure inputs")? {
        if inputs.get(path).map(String::as_str) != expected.as_str() {
            return Err(format!("cover input binding differs: {path}").into());
        }
    }
    if spec["subject"] != proof["automatic"]["label"]
        || render["subject"] != spec["subject"]
        || scene["subject"] != spec["subject"]
        || spec["certificate_identity"] != proof["automatic"]["certificate_identity"]
        || render["certificate_identity"] != spec["certificate_identity"]
        || scene["stl_sha256"] != spec["geometry"]["sha256"]
        || spec["geometry"]["path"] != proof["automatic"]["artifacts"]["stl"]
    {
        return Err("cover subject or geometry binding differs".into());
    }
    if scene["import_transform"]
        != json!({"uniform_scale":0.001,"translation":[0,0,0],"rotation":[0,0,0]})
    {
        return Err("cover import transform differs".into());
    }
    validate_immutable_scene(&scene)
}

fn cover_evolution(births: &[Value], stats: &[Value]) -> Result<String> {
    let mut s = svg_start(520, 620);
    label(
        &mut s,
        18.,
        30.,
        23,
        "B / Run 1 Historical Population",
        "#007e87",
    );
    label(
        &mut s,
        18.,
        57.,
        21,
        &format!("{} retained births; stored fit", births.len()),
        "#526873",
    );
    let lo = births
        .iter()
        .map(|r| number(&r["fit"]))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .fold(f64::INFINITY, f64::min);
    let hi = births
        .iter()
        .map(|r| number(&r["fit"]))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .fold(f64::NEG_INFINITY, f64::max);
    let maxg = stats.last().ok_or("empty cover population")?["generation"]
        .as_u64()
        .ok_or("invalid cover generation")? as f64;
    let sx = |g: f64| 82. + 410. * g / maxg.max(1.);
    let sy = |v: f64| 258. - 172. * (v.asinh() - lo.asinh()) / (hi.asinh() - lo.asinh()).max(1e-9);
    for value in [-200.0_f64, -10., -1., 0., 1.] {
        let y = sy(value);
        s.push_str(&format!(
            "<path d=\"M82 {}H492\" stroke=\"#d5dfdf\"/>",
            svg_coord(y)
        ));
        label(&mut s, 4., y + 6., 21, &format!("{value:.0}"), "#526873");
    }
    for birth in births {
        let x = sx(number(&birth["gen"])?);
        let y = sy(number(&birth["fit"])?);
        if birth["fem_status"] == "complete" {
            s.push_str(&format!(
                "<circle cx=\"{x:.2}\" cy=\"{y:.2}\" r=\"1.4\" fill=\"#4e8f9c\" opacity=\".3\"/>"
            ));
        } else {
            s.push_str(&format!(
                "<path d=\"M{} {}l3 3m0 -3l-3 3\" stroke=\"#b66a30\" opacity=\".5\"/>",
                svg_coord(x - 1.5),
                svg_coord(y - 1.5)
            ));
        }
    }
    for g in [0, 4, 8, 12, 16] {
        label(
            &mut s,
            sx(g as f64) - 6.,
            286.,
            21,
            &g.to_string(),
            "#526873",
        );
    }
    label(
        &mut s,
        82.,
        312.,
        21,
        "Generation / asinh vertical axis",
        "#526873",
    );
    label(
        &mut s,
        18.,
        341.,
        21,
        "Blue: FEM complete / amber x: no FEM",
        "#132d3b",
    );
    label(
        &mut s,
        18.,
        379.,
        23,
        "C / Run 1 Observed Search Maxima",
        "#007e87",
    );
    let cy = |v: f64| 528. - 111. * (v - 1.135) / (1.16 - 1.135);
    for value in [1.14, 1.15, 1.16] {
        let y = cy(value);
        s.push_str(&format!(
            "<path d=\"M82 {}H492\" stroke=\"#d5dfdf\"/>",
            svg_coord(y)
        ));
        label(&mut s, 4., y + 6., 21, &format!("{value:.2}"), "#526873");
    }
    let mut cumulative = f64::NEG_INFINITY;
    let mut generation_points = Vec::new();
    let mut cumulative_points = Vec::new();
    for row in stats {
        let g = number(&row["generation"])?;
        let value = number(&row["max"])?;
        cumulative = cumulative.max(value);
        generation_points.push(format!("{:.2},{:.2}", sx(g), cy(value)));
        cumulative_points.push(format!("{:.2},{:.2}", sx(g), cy(cumulative)));
        s.push_str(&format!(
            "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"2.5\" fill=\"#007e87\"/>",
            sx(g),
            cy(value)
        ));
    }
    s.push_str(&format!("<polyline points=\"{}\" fill=\"none\" stroke=\"#007e87\" stroke-width=\"2\"/><polyline points=\"{}\" fill=\"none\" stroke=\"#132d3b\" stroke-width=\"2\" stroke-dasharray=\"6 4\"/>",generation_points.join(" "),cumulative_points.join(" ")));
    for g in [0, 4, 8, 12, 16] {
        label(
            &mut s,
            sx(g as f64) - 6.,
            553.,
            21,
            &g.to_string(),
            "#526873",
        );
    }
    label(
        &mut s,
        18.,
        582.,
        21,
        "Teal: generation / dashed: cumulative",
        "#132d3b",
    );
    label(
        &mut s,
        18.,
        609.,
        21,
        "Truncated fit axis; higher is better",
        "#a3442d",
    );
    s.push_str("</g></svg>\n");
    Ok(s)
}

fn hero(curve: &Value, births: &[Value]) -> Result<String> {
    validate_curve(curve)?;
    let best = &curve["best_so_far"];
    let f = number(&best["ultralight_weighted_fitness"])?;
    let terms = best["ultralight_fitness_breakdown"]["objectives"]
        .as_object()
        .ok_or("missing objective profile")?;
    let mut sum = 0.0;
    let mut s = svg_start(1200, 1140);
    label(
        &mut s,
        24.,
        34.,
        24,
        "INDIVIDUAL 2002 (RUN 31) — HISTORICAL SAVED WEIGHTED ARITHMETIC",
        "#007e87",
    );
    label(
        &mut s,
        24.,
        64.,
        22,
        &format!("Saved F = {f:.6} / 100.00 (101.139 g; no current qualification inferred)"),
        "#132d3b",
    );

    let term_title = |key: &str| -> &'static str {
        match key {
            "ultralight_mass" => "Ultralight mass (101.14 g)",
            "holder_and_compactness" => "Envelope & compactness",
            "karambit_ring_and_claw" => "Ring & claw curvature",
            "c6_aesthetic_and_ergo" => "Silhouette concordance",
            "bore_collar_comfort" => "Bore collar comfort",
            "cad_watertight_brep" => "Watertight single-solid B-Rep",
            "gate_s_sharpness" => "Bevel sharpness angle",
            "gate_s_point_profile" => "Tip point angle & radius",
            "hiker_lateral_180n" => "180 N lateral bend",
            "hunter_lateral_260n" => "260 N lateral bend",
            "strict_hunter_260n" => "Strict 260 N bend (FAIL)",
            "ring_pull_1500n" => "1500 N ring pull",
            "twist_body_1p82nm" => "1.82 N·m body torsion",
            "twist_tip_plate_1p82nm" => "1.82 N·m tip plate torsion",
            "solid_tip_3d_p2_and_xcheck" => "3D P2 tip & cross-check",
            "side_a4_buckling" => "A4 term (unadmitted)",
            "failure_sequence_1p2" => "Yield sequence ratio",
            "plate_gci_3level" => "Saved mesh-sequence flag",
            "human_max_overload" => "Overload reserve",
            "goodman_fatigue_l7" => "Historical fatigue term",
            _ => "Objective term",
        }
    };

    let mut cur_y = 104.0;
    for (name, keys, cap) in REPORT_GROUPS.iter() {
        let points = keys
            .iter()
            .map(|k| number(&terms[*k]["weighted_points"]))
            .collect::<Result<Vec<_>>>()?
            .iter()
            .sum::<f64>();
        sum += points;
        label(
            &mut s,
            24.,
            cur_y,
            22,
            &format!("{name}: {points:.2} / {cap:.0} pts"),
            "#007e87",
        );
        cur_y += 28.0;
        for &k in *keys {
            let w_pct = number(&terms[k]["weight_pct"])?;
            let pts = number(&terms[k]["weighted_points"])?;
            let norm = number(&terms[k]["normalized_score"])?;
            label(&mut s, 32., cur_y, 21, term_title(k), "#132d3b");
            let bar_w = 130.0 * norm.clamp(0.0, 1.0);
            let bar_col = if norm < 0.999 { "#a3442d" } else { "#007e87" };
            s.push_str(&format!(
                "<rect x=\"330\" y=\"{}\" width=\"130\" height=\"16\" rx=\"4\" fill=\"#dce4e3\"/><rect x=\"330\" y=\"{}\" width=\"{}\" height=\"16\" rx=\"4\" fill=\"{bar_col}\"/>\n",
                svg_coord(cur_y - 15.0),
                svg_coord(cur_y - 15.0),
                svg_coord(bar_w)
            ));
            label(
                &mut s,
                470.,
                cur_y,
                21,
                &format!("{pts:.2}/{w_pct}"),
                "#526873",
            );
            cur_y += 38.0;
        }
        cur_y += 10.0;
    }
    if (sum - f).abs() > 0.00001 {
        return Err("profile sum differs from displayed fitness".into());
    }
    let tally = best["ultralight_fitness_breakdown"]["legacy_hard_gates_passed_count"]
        .as_u64()
        .ok_or("missing legacy tally")?;
    label(
        &mut s,
        24.,
        1060.,
        21,
        &format!(
            "Historical producer tally: {tally}/26; current dependent qualification unestablished."
        ),
        "#a3442d",
    );
    label(
        &mut s,
        24.,
        1094.,
        21,
        "A4 is unadmitted; saved F and weights preserved; partly default-compatible/proxy terms.",
        "#526873",
    );
    label(&mut s, 24., 1126., 21,
        "Recipe execution/call inputs unestablished; Individual 2000 has a separate saved CAD score.",
        "#526873");

    // Right column Panel A: Run 31 3-generation trajectory
    label(
        &mut s,
        620.,
        104.,
        23,
        "A. Run 31 Weighted Trajectory (Gen 0–2)",
        "#007e87",
    );
    let rows = array(&curve["generations"])?;
    let values = rows
        .iter()
        .flat_map(|r| [&r["round_best_fitness"], &r["hall_of_fame_best_so_far"]])
        .map(number)
        .collect::<Result<Vec<_>>>()?;
    let lo = (values.iter().copied().fold(f64::INFINITY, f64::min) * 2.).floor() / 2. - 0.2;
    let hi = (values.iter().copied().fold(f64::NEG_INFINITY, f64::max) * 2.).ceil() / 2. + 0.1;
    let maxg = number(&rows.last().ok_or("empty curve")?["generation"])?.max(1.);
    let sx = |g: f64| 710. + 440. * g / maxg;
    let sy = |f: f64| 380. - 220. * (f - lo) / (hi - lo);
    for i in 0..4 {
        let v = lo + (hi - lo) * i as f64 / 3.;
        let y = sy(v);
        s.push_str(&format!(
            "<path d=\"M710 {}H1150\" stroke=\"#d5dfdf\"/>",
            svg_coord(y)
        ));
        label(&mut s, 620., y + 7., 21, &format!("{v:.2}"), "#526873");
    }
    for (field, color) in [
        ("round_best_fitness", "#d07b42"),
        ("hall_of_fame_best_so_far", "#007e87"),
    ] {
        let points = rows
            .iter()
            .map(|r| {
                Ok(format!(
                    "{:.2},{:.2}",
                    sx(number(&r["generation"])?),
                    sy(number(&r[field])?)
                ))
            })
            .collect::<Result<Vec<_>>>()?
            .join(" ");
        s.push_str(&format!(
            "<polyline points=\"{points}\" stroke=\"{color}\" stroke-width=\"4\" fill=\"none\"/>"
        ));
        for r in rows {
            s.push_str(&format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"6\" fill=\"{color}\"/>",
                sx(number(&r["generation"])?),
                sy(number(&r[field])?)
            ));
        }
    }
    for r in rows {
        let g = number(&r["generation"])?;
        label(
            &mut s,
            sx(g) - 18.,
            416.,
            21,
            &format!("G{}", g as u64),
            "#526873",
        );
    }
    label(
        &mut s,
        620.,
        454.,
        21,
        "Teal: archive best (Ind 2009 G0 -> Ind 2002 G1–G2)",
        "#132d3b",
    );
    label(
        &mut s,
        620.,
        486.,
        21,
        "Orange: round best (74.17, 74.21, 73.61)",
        "#526873",
    );

    // Right column Panel B: Run 1 historical fit
    label(
        &mut s,
        620.,
        556.,
        23,
        &format!("B. Run 1 Historical Search ({} Births)", births.len()),
        "#007e87",
    );
    let max_generation = births
        .iter()
        .map(|b| number(&b["gen"]))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .fold(1.0, f64::max);
    let fit_values = births
        .iter()
        .map(|b| number(&b["fit"]))
        .collect::<Result<Vec<_>>>()?;
    let fit_lo = fit_values
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min)
        .asinh();
    let fit_hi = fit_values
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max)
        .asinh();
    let px = |g: f64| 710. + 440. * g / max_generation;
    let py = |f: f64| 860. - 250. * (f.asinh() - fit_lo) / (fit_hi - fit_lo).max(1e-9);
    for tick in [-200.0_f64, -10., -1., 1.] {
        label(
            &mut s,
            620.,
            py(tick) + 7.,
            21,
            &format!("{tick:.0}"),
            "#526873",
        );
        s.push_str(&format!(
            "<path d=\"M710 {}H1150\" stroke=\"#d5dfdf\"/>",
            svg_coord(py(tick))
        ));
    }
    let mut gen_values: BTreeMap<u64, Vec<f64>> = BTreeMap::new();
    for (b, f) in births.iter().zip(fit_values) {
        let g = b["gen"].as_u64().ok_or("invalid birth generation")?;
        gen_values.entry(g).or_default().push(f);
        s.push_str(&format!(
            "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"1.5\" fill=\"#78999e\" opacity=\".22\"/>",
            px(g as f64),
            py(f)
        ));
    }
    for (mode, color) in [("median", "#d07b42"), ("max", "#007e87")] {
        let points = gen_values
            .iter()
            .map(|(g, v)| {
                Ok(format!(
                    "{:.2},{:.2}",
                    px(*g as f64),
                    py(if mode == "median" {
                        median(v)?
                    } else {
                        v.iter().copied().fold(f64::NEG_INFINITY, f64::max)
                    })
                ))
            })
            .collect::<Result<Vec<_>>>()?
            .join(" ");
        s.push_str(&format!(
            "<polyline points=\"{points}\" stroke=\"{color}\" stroke-width=\"2.5\" fill=\"none\"/>"
        ));
    }
    for g in [0., 8., 16.] {
        label(
            &mut s,
            px(g) - 14.,
            896.,
            21,
            &format!("G{g:.0}"),
            "#526873",
        );
    }
    label(
        &mut s,
        620.,
        936.,
        21,
        "Asinh(fit/1); L = 1; original fit ticks",
        "#526873",
    );
    label(
        &mut s,
        620.,
        968.,
        21,
        "Teal: gen max (Ind 1095 at G9) / Orange: median",
        "#132d3b",
    );
    label(
        &mut s,
        620.,
        1000.,
        21,
        "Separate search regime; no qualification inferred",
        "#a3442d",
    );
    s.push_str("</g></svg>\n");
    Ok(s)
}

pub fn generate(root: &Path, check: bool) -> Result<()> {
    let root = root.canonicalize()?;
    let mut inputs = BTreeMap::new();
    let catalog = read(&root, "docs/tests/catalog.json", &mut inputs)?;
    let entries = array(&catalog["entries"])?;
    let battery = read(&root, "evidence/cad_battery/manifest.json", &mut inputs)?;
    let battery_contract = read(&root, "docs/paper/battery-inputs.v1.json", &mut inputs)?;
    if battery_contract["schema"] != "jarambit.battery_figure_inputs.v1"
        || battery_contract["manifest"]["path"] != "evidence/cad_battery/manifest.json"
        || battery_contract["manifest"]["sha256"].as_str()
            != Some(inputs["evidence/cad_battery/manifest.json"].as_str())
        || battery_contract["manifest_source_revision"] != battery["source_commit"]
    {
        return Err("historical battery figure binding differs".into());
    }
    let battery_digests = battery_contract["record_sha256"]
        .as_object()
        .ok_or("missing battery record digests")?;
    let evaluators = array(&battery["case_ids"])?;
    let evaluator_ids = unique(evaluators.iter().map(|v| v.as_str().unwrap_or("")))?;
    let files = array(&battery["files"])?;
    unique(files.iter().map(|v| v.as_str().unwrap_or("")))?;
    let declared_subjects = unique(
        array(&battery["subjects"])?
            .iter()
            .map(|v| v.as_str().unwrap_or("")),
    )?;
    if declared_subjects.len() != files.len() {
        return Err("subject/file count mismatch".into());
    }
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut per: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    let mut actual_subjects = BTreeSet::new();
    let mut battery_records = BTreeMap::new();
    for file in files {
        let d = read(&root, text(file)?, &mut inputs)?;
        if battery_digests.get(text(file)?).and_then(Value::as_str)
            != Some(inputs[text(file)?].as_str())
        {
            return Err("historical battery record digest differs".into());
        }
        // Subject identity is encoded by the manifest filename, not display order.
        let sid = Path::new(text(file)?)
            .file_stem()
            .ok_or("missing subject filename")?
            .to_str()
            .ok_or("non-UTF8 subject")?;
        if text(&d["subject"])? != sid {
            return Err("subject identity differs from manifest filename".into());
        }
        if !actual_subjects.insert(sid.to_string()) {
            return Err("duplicate subject".into());
        }
        let cases = array(&d["cases"])?;
        let ids = unique(cases.iter().map(|v| v["id"].as_str().unwrap_or("")))?;
        if ids != evaluator_ids {
            return Err("missing or extra evaluator cells".into());
        }
        for c in cases {
            let status = text(&c["status"])?;
            if !["pass", "fail", "blocked", "incomplete"].contains(&status) {
                return Err("unknown stored status".into());
            }
            if c["fresh_this_lane"] != false {
                return Err("historical corpus freshness changed".into());
            }
            *counts.entry(status.into()).or_default() += 1;
            *per.entry(text(&c["id"])?.into())
                .or_default()
                .entry(status.into())
                .or_default() += 1;
        }
        battery_records.insert(text(file)?.to_string(), d);
    }
    if actual_subjects != declared_subjects || battery_digests.len() != battery_records.len() {
        return Err("manifest subject membership mismatch".into());
    }
    let battery_matrix = crate::battery::project(&battery, &battery_records)?;
    if battery_matrix["stored_status_counts"] != battery_contract["expected_status_counts"]
        || battery_matrix["role_counts"] != battery_contract["expected_role_counts"]
        || battery_matrix["record_source_revision_counts"]
            != battery_contract["expected_record_source_revision_counts"]
    {
        return Err("historical battery figure count binding differs".into());
    }
    let mut out: BTreeMap<String, String> = BTreeMap::new();
    out.insert(
        "docs/paper/generated/battery-matrix.svg".into(),
        crate::battery::svg(&battery_matrix)?,
    );
    out.insert(
        "docs/paper/generated/battery-matrix.json".into(),
        serde_json::to_string_pretty(&battery_matrix)? + "\n",
    );
    let mut atlas=String::from("# Engineering assessment atlas\n\nGenerated by `cargo run --locked --manifest-path crates/paper-artifacts/Cargo.toml --example paper_artifacts`. Explanations describe existing assessments; no new CAD, FEM or physical tests run during generation.\n\n**16 legacy predicates, ten overlapping RS-v2 groups, and 18 battery evaluators count different things.** The historical battery has 23 subjects and 414 retained cells; all cells say `fresh_this_lane: false`. A complete result can contain an unfavorable finding.\n\n[Science: all 178 definitions](science.md) · [Visual coverage and missing fields](visuals.md) · [Paper](../paper/README.md) · [Engineering specification](../PAPER_AND_README_ENGINEERING_SPEC.md)\n\n");
    let canonical = read(
        &root,
        "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_cad_eval.json",
        &mut inputs,
    )?;
    let g16 = canonical["weighted_cad_score"]["gates"]
        .as_object()
        .ok_or("missing predicates")?;
    for kind in ["G16", "RSV2", "battery"] {
        let matching = entries
            .iter()
            .filter(|e| e["kind"] == kind)
            .collect::<Vec<_>>();
        unique(matching.iter().map(|e| e["id"].as_str().unwrap_or("")))?;
        let expected = if kind == "G16" {
            16
        } else if kind == "RSV2" {
            10
        } else {
            18
        };
        if matching.len() != expected {
            return Err("explanation inventory mismatch".into());
        }
        atlas.push_str(&format!(
            "## {kind}\n\n| Identity | Explanation |\n|---|---|\n"
        ));
        for e in &matching {
            atlas.push_str(&format!(
                "| `{}` | [{}](#{}-{}) |\n",
                text(&e["id"])?,
                text(&e["title"])?,
                kind.to_lowercase(),
                text(&e["id"])?
            ));
        }
        atlas.push('\n');
        for e in matching {
            let id = text(&e["id"])?;
            if kind == "G16" && !g16.contains_key(id) {
                return Err(format!("unknown G16 predicate: {id}").into());
            }
            if kind == "battery" && !evaluator_ids.contains(id) {
                return Err("unknown evaluator".into());
            }
            for field in ["source", "control"] {
                let p = text(&e[field])?;
                let raw = fs::read(safe_path(&root, p)?)?;
                inputs.insert(p.into(), digest(&raw));
            }
            if kind == "RSV2"
                && !fs::read_to_string(root.join("crates/cli/src/bench.rs"))?
                    .contains(&format!("\"{id}\""))
            {
                return Err("unknown RS-v2 group".into());
            }
            atlas.push_str(&format!("<a id=\"{}-{id}\"></a>\n\n### {}\n\n**Exact ID:** `{id}`. **Observation family:** {}.\n\n{}\n\n**Interpretation and limits:** {}\n\n[Current implementation](../../{}) · [Pinned implementation](https://github.com/neverhuman/jarambit/blob/{BASE}/{}) · [Related software controls](../../{})\n\n",kind.to_lowercase(),text(&e["title"])?,text(&e["fields"])?,text(&e["purpose"])?,text(&e["limit"])?,text(&e["source"])?,text(&e["source"])?,text(&e["control"])?));
            if kind == "battery" {
                let c = &per[id];
                atlas.push_str(&format!("**Historical stored outcomes (23 subjects):** {} pass / {} fail / {} blocked / {} incomplete. These preserve the original column semantics. [Corpus manifest](../../evidence/cad_battery/manifest.json); select `cases[]` by `id == \"{id}\"`, then inspect `evidence_path`, `kind`, `solver`, `source_commit`, `geometry_sha256` and `fresh_this_lane`.\n\n",c.get("pass").unwrap_or(&0),c.get("fail").unwrap_or(&0),c.get("blocked").unwrap_or(&0),c.get("incomplete").unwrap_or(&0)));
                atlas.push_str(&format!("**Actual historical-corpus visual:** [all 23 stored labels in the `{id}` column](../paper/generated/battery-matrix.svg). [Cell-level source/selector receipt](../paper/generated/battery-matrix.json). These categorical labels are not FEM fields or independent physical-test counts.\n\n"));
            } else {
                atlas.push_str(if kind=="G16" {"**Retained example:** [corrected candidate assessment](../../evidence/rs_v2/w283_ultrarunner/w283_p14_c01_cad_eval.json), select `weighted_cad_score.gates` by exact ID. [Predicate authority](../../ops/ci/evaluate_cad_candidate.py). No current qualification follows from a historical favorable field.\n\n"} else {"**Retained example:** [historical grouped result](../../evidence/rs_v2/w283_ultrarunner/w283_p14_c01_rs.json). Use exact group IDs from `rs_v2_eval` in the implementation; do not infer group identity from table position. [Requirement namespace](../../requirements/RS-karambit-v2.toml).\n\n"});
            }
            let visual = crate::visuals::assessed(kind, id);
            if let Some(v) = visual {
                let (figure, panel) = (v.figure, v.panel);
                atlas.push_str(&format!("**Separate assessed-example visual:** [{panel}](../paper/generated/{figure}.svg), for `cert_pipeline_r01_p14_c02`, not qualified. [Projection receipt](../paper/generated/{figure}.json). This is a separately bound example, not a rerun of the historical example or the 23-subject battery.\n\n"));
            } else if kind != "battery" {
                let why =
                    crate::visuals::missing_reason(kind, id).ok_or("unmapped subsection visual")?;
                atlas.push_str(&format!(
                    "**Visual gap:** {why} [Complete coverage ledger](visuals.md).\n\n"
                ));
            }
        }
    }
    out.insert(
        "docs/tests/README.md".into(),
        atlas.trim_end().to_string() + "\n",
    );
    let science = read(
        &root,
        "docs/v1/science/tests/TEST_CASE_CATALOG.json",
        &mut inputs,
    )?;
    let cases = array(&science["cases"])?;
    unique(cases.iter().map(|v| v["id"].as_str().unwrap_or("")))?;
    if cases.len()
        != science["case_group_count"]
            .as_u64()
            .ok_or("invalid science count")? as usize
    {
        return Err("science count mismatch".into());
    }
    let mut sc=String::from("# Scientific assurance: complete frozen definition index\n\nGenerated from [TEST_CASE_CATALOG.json](../v1/science/tests/TEST_CASE_CATALOG.json). These are **178 definitions across 20 T00–T19 families**, with 356 required paired-control slots. Frozen status is historical; source presence is not an execution receipt. Expected receipt paths containing `<attempt>` are patterns, not evidence that a receipt exists. No execution-receipt join is asserted by this index.\n\n[CAD atlas](README.md) · [Paper](../paper/README.md)\n\n");
    let mut families: BTreeMap<String, usize> = BTreeMap::new();
    for c in cases {
        *families.entry(text(&c["work_unit"])?.into()).or_default() += 1;
    }
    sc.push_str("| Family | Frozen definitions |\n|---|---:|\n");
    for (f, n) in &families {
        sc.push_str(&format!("| [{f}](#{}) | {n} |\n", f.to_lowercase()));
    }
    for f in families.keys() {
        sc.push_str(&format!("\n## {f}\n\n"));
        for c in cases.iter().filter(|c| c["work_unit"] == *f) {
            sc.push_str(&format!("<a id=\"{}\"></a>\n\n### {} — {}\n\n**Challenge:** {}.\n\n**Oracle:** {}. **Frozen status:** {}.\n\n**Allowed implementation location:** `{}`. **Expected receipt pattern:** `{}`.\n\n**Claim limit:** {}.\n\n",text(&c["id"])?,text(&c["id"])?,text(&c["title"])?,text(&c["challenge"])?,text(&c["oracle"])?,text(&c["status"])?,text(&c["allowed_test_path"])?,text(&c["receipt_artifact"])?,text(&c["scientific_claim_limit"])?));
        }
    }
    out.insert(
        "docs/tests/science.md".into(),
        sc.trim_end().to_string() + "\n",
    );
    let curve = read(&root, CURVE, &mut inputs)?;
    validate_curve(&curve)?;
    validate_weighted_report(&curve, entries)?;
    let births = read(&root, POP, &mut inputs)?;
    let births = array(&births)?;
    let history_root = "evidence/rs_v2/w282_ultrarunner";
    let history_manifest = read(&root, &format!("{history_root}/manifest.json"), &mut inputs)?;
    if inputs[&format!("{history_root}/manifest.json")]
        != "543f7e8410214a7e34e4ef55772e9d671221001d0e11a36b4d68d1a0ef0a96cf"
    {
        return Err("historical population snapshot requires editorial readmission".into());
    }
    let provenance = read(
        &root,
        &format!("{history_root}/provenance.json"),
        &mut inputs,
    )?;
    let search_summary = read(
        &root,
        &format!("{history_root}/search_summary.json"),
        &mut inputs,
    )?;
    for member in [
        "search/all_evals.json",
        "provenance.json",
        "search_summary.json",
    ] {
        let path = format!("{history_root}/{member}");
        if history_manifest["files"][member].as_str() != inputs.get(&path).map(String::as_str) {
            return Err(format!("historical manifest digest differs: {member}").into());
        }
    }
    if history_manifest["source_commit"] != provenance["source_commit"] {
        return Err("historical producer revisions differ".into());
    }
    let history = crate::historical::context(births, &provenance, &search_summary)?;
    if history["best_observed"]["strict_assessment"]["cert_pass_all"] != false {
        return Err(
            "retained assessment does not support the manuscript's historical finding".into(),
        );
    }
    let lineage_contract = read(&root, "docs/paper/lineage-inputs.v1.json", &mut inputs)?;
    if lineage_contract["schema"] != "jarambit.lineage_inputs.v1" {
        return Err("wrong lineage figure contract".into());
    }
    for (key, expected_path) in [
        ("population", POP),
        (
            "summary",
            "evidence/rs_v2/w282_ultrarunner/search_summary.json",
        ),
    ] {
        if lineage_contract[key]["path"] != expected_path
            || lineage_contract[key]["sha256"].as_str() != Some(inputs[expected_path].as_str())
        {
            return Err("lineage source binding differs".into());
        }
    }
    let lineage = crate::lineage::project(births, &history, &lineage_contract)?;
    out.insert(
        "docs/paper/generated/lineage.svg".into(),
        crate::lineage::svg(&lineage)?,
    );
    out.insert(
        "docs/paper/generated/lineage.json".into(),
        serde_json::to_string_pretty(&lineage)? + "\n",
    );
    admit_cover_render(&root, &mut inputs)?;
    let weighted_contract = read(&root, "docs/paper/weighted-inputs.v1.json", &mut inputs)?;
    let weighted_specs = array(&weighted_contract["runs"])?;
    let mut weighted_docs = BTreeMap::new();
    for spec in weighted_specs {
        let id = text(&spec["id"])?;
        let path = text(&spec["path"])?;
        let doc = read(&root, path, &mut inputs)?;
        if inputs[path] != text(&spec["sha256"])? {
            return Err("weighted summary source digest differs".into());
        }
        let parent = Path::new(path)
            .parent()
            .ok_or("missing weighted source directory")?;
        for row in array(&doc["generations"])? {
            let file = text(&row["round_file"])?;
            if Path::new(file).components().count() != 1
                || !matches!(
                    Path::new(file).components().next(),
                    Some(Component::Normal(_))
                )
            {
                return Err("unsafe declared weighted round filename".into());
            }
            let relative = parent.join(file);
            if safe_path(
                &root,
                relative.to_str().ok_or("invalid weighted round path")?,
            )?
            .exists()
            {
                return Err(
                    "weighted population availability changed; review the admitted scope".into(),
                );
            }
        }
        if weighted_docs.insert(id.to_string(), doc).is_some() {
            return Err("duplicate weighted summary source".into());
        }
    }
    let weighted_projection = crate::weighted_runs::project(&weighted_docs, &weighted_contract)?;
    out.insert(
        "docs/paper/generated/weighted-runs.svg".into(),
        crate::weighted_runs::svg(&weighted_projection)?,
    );
    out.insert(
        "docs/paper/generated/weighted-runs.json".into(),
        serde_json::to_string_pretty(&weighted_projection)? + "\n",
    );
    let field_contract = read(&root, "docs/paper/field-inputs.v1.json", &mut inputs)?;
    let assessment_manifest = read(
        &root,
        "evidence/evolve_certifiable/manifest.json",
        &mut inputs,
    )?;
    if field_contract["source"] != assessment_manifest["automatic"]["artifacts"]["d1"]
        || field_contract["subject"] != assessment_manifest["automatic"]["label"]
        || field_contract["producer_revision"] != assessment_manifest["numerical_kernel_commit"]
    {
        return Err("retained plate source or producer differs from assessment manifest".into());
    }
    let field_source = text(&field_contract["source"])?;
    let field_doc = read(&root, field_source, &mut inputs)?;
    if inputs[field_source] != text(&field_contract["source_sha256"])? {
        return Err("retained plate source digest differs".into());
    }
    let producer_sources = field_contract["producer_sources"]
        .as_object()
        .ok_or("missing plate producer sources")?;
    let expected_producers = [
        "crates/crescent/src/fem.rs",
        "crates/cli/src/crescent.rs",
        "crates/physics/src/fine.rs",
        "crates/crescent/src/pheno.rs",
    ];
    if producer_sources.len() != expected_producers.len()
        || expected_producers
            .iter()
            .any(|path| !producer_sources.contains_key(*path))
    {
        return Err("plate interpretation source membership differs".into());
    }
    for (path, expected) in producer_sources {
        let actual = digest(&fs::read(safe_path(&root, path)?)?);
        if actual != text(expected)? {
            return Err(format!("plate interpretation source changed: {path}").into());
        }
        inputs.insert(path.clone(), actual);
    }
    let (plate_svg, plate_summary) = crate::plate::project(&field_doc, &field_contract)?;
    out.insert("docs/paper/generated/plate-fields.svg".into(), plate_svg);
    out.insert(
        "docs/paper/generated/plate-fields.json".into(),
        serde_json::to_string_pretty(&plate_summary)? + "\n",
    );
    // Additional camera view only; retain the complete original plate projection above.
    let smart_recipe_path = "docs/paper/data/smart-crop-contract.v1.json";
    let smart_recipe = read(&root, smart_recipe_path, &mut inputs)?;
    if smart_recipe["schema"] != "jarambit.smart_crop.display_contract.v1"
        || smart_recipe["source_contract"] != "docs/paper/field-inputs.v1.json"
    {
        return Err("smart crop: unadmitted recipe/schema/source contract".into());
    }
    let cutoffs = array(&smart_recipe["range_cutoffs"])?;
    if cutoffs.len() != 3 {
        return Err("smart crop: expected exact three fixed cutoffs".into());
    }
    let smart_policy = crate::saliency::CropPolicy {
        version: text(&smart_recipe["policy_version"])?.into(),
        range_cutoffs: [
            number(&cutoffs[0])?,
            number(&cutoffs[1])?,
            number(&cutoffs[2])?,
        ],
        min_excess_mass_coverage: number(&smart_recipe["min_excess_mass_coverage"])?,
        max_bbox_area_fraction: number(&smart_recipe["max_bbox_area_fraction"])?,
        min_scale_gain: number(&smart_recipe["min_scale_gain"])?,
        pad_fraction: number(&smart_recipe["pad_fraction"])?,
    };
    let (smart_svg, mut smart_receipt) =
        crate::plate::project_smart(&field_doc, &field_contract, &smart_policy)?;
    smart_receipt["display_recipe"] =
        json!({"path":smart_recipe_path,"sha256":inputs[smart_recipe_path]});
    out.insert(
        "docs/paper/generated/plate-smart-crops.svg".into(),
        smart_svg,
    );
    out.insert(
        "docs/paper/generated/plate-smart-crops.v1.json".into(),
        serde_json::to_string_pretty(&smart_receipt)? + "\n",
    );
    let verification_contract = read(&root, "docs/paper/verification-inputs.v1.json", &mut inputs)?;
    if verification_contract["producer_revision"] != assessment_manifest["numerical_kernel_commit"]
        || verification_contract["subject"] != field_contract["subject"]
        || verification_contract["certificate_identity"] != field_contract["certificate_identity"]
    {
        return Err("verification and plate subjects/producers differ".into());
    }
    let verification_members = [
        "plate_gci",
        "solid_tip",
        "skfem_raw",
        "dolfinx_raw",
        "dolfinx_sidecar",
        "mesh",
    ];
    let sources = verification_contract["sources"]
        .as_object()
        .ok_or("missing verification sources")?;
    if sources.len() != verification_members.len() {
        return Err("verification source membership differs".into());
    }
    let mut verification_docs = BTreeMap::new();
    for key in verification_members {
        let record = &sources[key];
        let path = text(&record["path"])?;
        if assessment_manifest["automatic"]["artifacts"][key] != path {
            return Err("verification artifact path differs".into());
        }
        let value = if key == "mesh" {
            inputs.insert(path.into(), digest(&fs::read(safe_path(&root, path)?)?));
            Value::Null
        } else {
            read(&root, path, &mut inputs)?
        };
        if inputs[path] != text(&record["sha256"])? {
            return Err("verification artifact digest differs".into());
        }
        verification_docs.insert(key, value);
    }
    let interpretation = verification_contract["interpretation_sources"]
        .as_object()
        .ok_or("missing verification interpretation sources")?;
    let expected_interpretation = [
        "fem_solid/tip_solid.py",
        "fem_solid/cross_check.py",
        "fem_solid/gci.py",
    ];
    if interpretation.len() != expected_interpretation.len() {
        return Err("verification interpretation source membership differs".into());
    }
    for path in expected_interpretation {
        let hash = digest(&fs::read(safe_path(&root, path)?)?);
        if interpretation[path].as_str() != Some(hash.as_str()) {
            return Err("verification interpretation source changed".into());
        }
        inputs.insert(path.into(), hash);
    }
    let (verification_svg, verification_summary) = crate::verification::project(
        &verification_docs["plate_gci"],
        &verification_docs["solid_tip"],
        &verification_docs["skfem_raw"],
        &verification_docs["dolfinx_raw"],
        &verification_docs["dolfinx_sidecar"],
        &verification_contract,
    )?;
    out.insert(
        "docs/paper/generated/verification.svg".into(),
        verification_svg,
    );
    out.insert(
        "docs/paper/generated/verification.json".into(),
        serde_json::to_string_pretty(&verification_summary)? + "\n",
    );
    if verification_docs["solid_tip"]["d1_sha256"] != field_contract["source_sha256"] {
        return Err("verification and displayed plate input digests differ".into());
    }
    let assessment_contract = read(&root, "docs/paper/assessment-inputs.v1.json", &mut inputs)?;
    if assessment_contract["producer_revision"] != assessment_manifest["numerical_kernel_commit"]
        || assessment_contract["subject"] != field_contract["subject"]
        || assessment_contract["certificate_identity"] != field_contract["certificate_identity"]
    {
        return Err("assessment scalar identity/producer differs".into());
    }
    let scalar_sources = assessment_contract["sources"]
        .as_object()
        .ok_or("missing scalar sources")?;
    if scalar_sources.len() != 3 {
        return Err("scalar source membership differs".into());
    }
    let mut scalar_docs = BTreeMap::new();
    for key in ["a4", "rs", "scorecard"] {
        let source = &scalar_sources[key];
        let path = text(&source["path"])?;
        if assessment_manifest["automatic"]["artifacts"][key] != path {
            return Err("scalar artifact path differs".into());
        }
        let doc = read(&root, path, &mut inputs)?;
        if inputs[path] != text(&source["sha256"])? {
            return Err("scalar artifact digest differs".into());
        }
        scalar_docs.insert(key, doc);
    }
    let scalar_interpretation = assessment_contract["interpretation_sources"]
        .as_object()
        .ok_or("missing scalar interpretation sources")?;
    if scalar_interpretation.len() != 3 {
        return Err("scalar interpretation membership differs".into());
    }
    for path in [
        "ops/ci/side_a4.py",
        "crates/crescent/src/fem.rs",
        "crates/cli/src/bench.rs",
    ] {
        let hash = digest(&fs::read(safe_path(&root, path)?)?);
        if scalar_interpretation[path].as_str() != Some(hash.as_str()) {
            return Err("scalar interpretation source changed".into());
        }
        inputs.insert(path.into(), hash);
    }
    if scalar_docs["a4"]["d1_sha256"] != field_contract["source_sha256"] {
        return Err("support and displayed plate bindings differ".into());
    }
    let figure_semantics = read(&root, "docs/paper/figure-semantics.v1.json", &mut inputs)?;
    let claim_evidence = read(&root, "docs/paper/claim-evidence.v1.json", &mut inputs)?;
    for (path, expected) in figure_semantics["original_contracts"]
        .as_object()
        .ok_or("missing original semantic input bindings")?
    {
        if inputs.get(path).map(String::as_str) != expected.as_str() {
            return Err(format!("presentation original contract changed: {path}").into());
        }
    }
    let crop_admission = &claim_evidence["crop_svg_admission"];
    let crop_path = text(&crop_admission["path"])?;
    if crop_path != "docs/paper/drafts_p3_p5/JR_EI_P4_T04_fem_saliency_crops_and_locators.svg"
        || crop_admission["source_d1_sha256"] != field_contract["source_sha256"]
    {
        return Err("crop artwork admission source differs".into());
    }
    let crop_bytes = fs::read(safe_path(&root, crop_path)?)?;
    let crop_sha = digest(&crop_bytes);
    if crop_admission["sha256"].as_str() != Some(crop_sha.as_str()) {
        return Err("crop artwork differs from source-triangle correspondence accession".into());
    }
    inputs.insert(crop_path.into(), crop_sha.clone());
    let mut crop_semantics = crate::plate::project_semantics(
        &field_doc,
        &field_contract,
        &assessment_contract,
        &lineage_contract,
        &figure_semantics,
        std::str::from_utf8(&crop_bytes)?,
    )?;
    crop_semantics["source_svg_sha256"] = json!(crop_sha);
    out.insert(
        "docs/paper/generated/crop-source-hash.tex".into(),
        format!("\\nolinkurl{{{crop_sha}}}%\n"),
    );
    out.insert(
        "docs/paper/generated/crop-semantics.v1.json".into(),
        serde_json::to_string_pretty(&crop_semantics)? + "\n",
    );
    let (assessment_svg, assessment_summary) = crate::assessment::project(
        &scalar_docs["a4"],
        &scalar_docs["rs"],
        &scalar_docs["scorecard"],
        &assessment_contract,
        &catalog,
    )?;
    out.insert("docs/paper/generated/assessment.svg".into(), assessment_svg);
    out.insert(
        "docs/paper/generated/assessment.json".into(),
        serde_json::to_string_pretty(&assessment_summary)? + "\n",
    );
    let export_contract = read(&root, "docs/paper/export-inputs.v1.json", &mut inputs)?;
    if export_contract["subject"] != field_contract["subject"]
        || export_contract["certificate_identity"] != field_contract["certificate_identity"]
        || export_contract["producer_revision"] != assessment_manifest["numerical_kernel_commit"]
    {
        return Err("export and assessed-example binding differs".into());
    }
    let export_sources = export_contract["sources"]
        .as_object()
        .ok_or("missing export sources")?;
    if export_sources.len() != 5 {
        return Err("export source membership differs".into());
    }
    let mut export_docs = BTreeMap::new();
    for key in ["scorecard", "step_check", "step", "stl", "genome"] {
        let source = &export_sources[key];
        let path = text(&source["path"])?;
        if assessment_manifest["automatic"]["artifacts"][key] != path {
            return Err("export artifact path differs".into());
        }
        let raw = fs::read(safe_path(&root, path)?)?;
        let actual = digest(&raw);
        if source["sha256"].as_str() != Some(actual.as_str()) {
            return Err("export artifact digest differs".into());
        }
        inputs.insert(path.into(), actual);
        if ["scorecard", "step_check"].contains(&key) {
            export_docs.insert(key, serde_json::from_slice::<Value>(&raw)?);
        }
    }
    let scene_path = "docs/paper/figures/v2/hero_dark.scene.json";
    let export_scene = read(&root, scene_path, &mut inputs)?;
    if export_contract["scene"]["path"] != scene_path
        || export_contract["scene"]["sha256"].as_str() != Some(inputs[scene_path].as_str())
    {
        return Err("export scene binding differs".into());
    }
    let (export_svg, export_summary) = crate::export_identity::project(
        &export_docs["scorecard"],
        &export_docs["step_check"],
        &export_scene,
        &export_contract,
    )?;
    out.insert(
        "docs/paper/generated/export-identity.svg".into(),
        export_svg,
    );
    out.insert(
        "docs/paper/generated/export-identity.json".into(),
        serde_json::to_string_pretty(&export_summary)? + "\n",
    );
    let reference_contract = read(&root, "docs/paper/reference-inputs.v1.json", &mut inputs)?;
    if reference_contract["subject"] != field_contract["subject"]
        || reference_contract["certificate_identity"] != field_contract["certificate_identity"]
        || reference_contract["producer_revision"] != field_contract["producer_revision"]
    {
        return Err("reference and assessed-example identity differs".into());
    }
    let reference_sources = reference_contract["sources"]
        .as_object()
        .ok_or("missing reference sources")?;
    if reference_sources.len() != 3 {
        return Err("reference source membership differs".into());
    }
    for key in ["d1", "scorecard", "rs"] {
        let source = &reference_sources[key];
        let path = text(&source["path"])?;
        if assessment_manifest["automatic"]["artifacts"][key] != path
            || inputs.get(path).map(String::as_str) != source["sha256"].as_str()
        {
            return Err("reference source binding differs".into());
        }
    }
    let reference_producers = reference_contract["producer_sources"]
        .as_object()
        .ok_or("missing reference producer sources")?;
    let reference_members = [
        "crates/crescent/src/aesthetic.rs",
        "crates/crescent/src/c5_ref.rs",
        "crates/crescent/src/c6_ref.rs",
        "crates/cli/src/crescent.rs",
        "crates/crescent/src/record.rs",
    ];
    if reference_producers.len() != reference_members.len() {
        return Err("reference producer membership differs".into());
    }
    for path in reference_members {
        let actual = digest(&fs::read(safe_path(&root, path)?)?);
        if reference_producers[path].as_str() != Some(actual.as_str()) {
            return Err("reference producer digest differs".into());
        }
        inputs.insert(path.into(), actual);
    }
    let (reference_svg, reference_summary) = crate::reference::project(
        &field_doc,
        &scalar_docs["scorecard"],
        &scalar_docs["rs"],
        &reference_contract,
    )?;
    out.insert("docs/paper/generated/reference.svg".into(), reference_svg);
    out.insert(
        "docs/paper/generated/reference.json".into(),
        serde_json::to_string_pretty(&reference_summary)? + "\n",
    );
    let birth_ids = births
        .iter()
        .map(|r| {
            r["id"]
                .as_u64()
                .map(|n| n.to_string())
                .ok_or("invalid birth ID")
        })
        .collect::<std::result::Result<Vec<_>, _>>()?;
    unique(birth_ids.iter().map(String::as_str))?;
    out.insert(
        "docs/paper/generated/fitness.svg".into(),
        hero(&curve, births)?,
    );
    let mut groups: BTreeMap<u64, Vec<f64>> = BTreeMap::new();
    for row in births {
        groups
            .entry(row["gen"].as_u64().ok_or("invalid birth generation")?)
            .or_default()
            .push(number(&row["fit"])?);
    }
    let mut stats = Vec::new();
    for (g, values) in &groups {
        let census = array(&history["by_generation"])?
            .iter()
            .find(|row| row["generation"] == *g)
            .ok_or("missing historical census generation")?;
        stats.push(json!({"generation":g,"n":values.len(),"min":values.iter().copied().fold(f64::INFINITY,f64::min),"median":median(values)?,"max":values.iter().copied().fold(f64::NEG_INFINITY,f64::max),"fem_complete":census["fem_complete"],"fem_not_evaluated":census["fem_not_evaluated"],"legal":census["legal"],"fem_complete_median":census["fem_complete_median"]}));
    }
    out.insert(
        "docs/paper/generated/cover-evolution.svg".into(),
        cover_evolution(births, &stats)?,
    );
    let mut popsvg = svg_start(1200, 680);
    label(
        &mut popsvg,
        32.,
        40.,
        24,
        &format!(
            "RUN 1 HISTORICAL POPULATION — ALL {} RETAINED BIRTH EVALUATIONS",
            births.len()
        ),
        "#007e87",
    );
    label(&mut popsvg,32.,72.,21,"Stored historical fit; asinh scale with original-value ticks. Penalty-valued records remain visible.","#526873");
    label(
        &mut popsvg,
        32.,
        102.,
        21,
        &format!(
            "{} FEM-complete / {} not evaluated / {} legal under the recorded search",
            history["fem_status_counts"]["complete"],
            history["fem_status_counts"]["not_evaluated"],
            history["legal_count"]
        ),
        "#526873",
    );
    let lo = stats
        .iter()
        .map(|s| s["min"].as_f64().unwrap())
        .fold(f64::INFINITY, f64::min);
    let hi = stats
        .iter()
        .map(|s| s["max"].as_f64().unwrap())
        .fold(f64::NEG_INFINITY, f64::max);
    let maxg = *groups.keys().last().ok_or("empty births")? as f64;
    let sx = |g: f64| 120. + 1020. * g / maxg.max(1.);
    let sy = |v: f64| 380. - 245. * (v.asinh() - lo.asinh()) / (hi.asinh() - lo.asinh()).max(1e-9);
    for v in [-200.0_f64, -100., -10., -1., 0., 1.] {
        let y = sy(v);
        popsvg.push_str(&format!(
            "<path d=\"M120 {}H1140\" stroke=\"#d5dfdf\"/>",
            svg_coord(y)
        ));
        label(&mut popsvg, 10., y + 7., 21, &format!("{v:.1}"), "#526873");
    }
    for row in births {
        let x = sx(number(&row["gen"])?);
        let y = sy(number(&row["fit"])?);
        if row["fem_status"] == "complete" {
            popsvg.push_str(&format!(
                "<circle cx=\"{x:.2}\" cy=\"{y:.2}\" r=\"2.0\" fill=\"#4e8f9c\" opacity=\".28\"/>"
            ));
        } else {
            popsvg.push_str(&format!("<path d=\"M{} {}l4 4m0 -4l-4 4\" stroke=\"#b66a30\" stroke-width=\"1.2\" opacity=\".55\"/>",svg_coord(x-2.),svg_coord(y-2.)));
        }
    }
    for (field, color) in [("median", "#132d3b"), ("max", "#007e87")] {
        let pts = stats
            .iter()
            .map(|s| {
                format!(
                    "{:.2},{:.2}",
                    sx(s["generation"].as_f64().unwrap()),
                    sy(s[field].as_f64().unwrap())
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        popsvg.push_str(&format!(
            "<polyline points=\"{pts}\" stroke=\"{color}\" stroke-width=\"3.2\" fill=\"none\"/>"
        ));
    }
    for g in groups.keys().step_by((groups.len() / 8).max(1)) {
        label(
            &mut popsvg,
            sx(*g as f64) - 16.,
            414.,
            21,
            &format!("G{g}"),
            "#526873",
        );
    }
    label(&mut popsvg,32.,450.,21,"Blue dots: FEM complete · Amber crosses: not evaluated · Teal: generation max · Dark: median", "#132d3b");
    label(
        &mut popsvg,
        32.,
        482.,
        21,
        "FEM evaluation status by generation (G0 = 128 births; G1–G16 = 116 births/gen)",
        "#526873",
    );
    let max_n = stats
        .iter()
        .map(|r| r["n"].as_u64().unwrap())
        .max()
        .unwrap() as f64;
    for row in &stats {
        let g = row["generation"].as_u64().unwrap();
        let x = sx(g as f64);
        let solved = number(&row["fem_complete"])? * 80. / max_n;
        let unsolved = number(&row["fem_not_evaluated"])? * 80. / max_n;
        popsvg.push_str(&format!("<rect x=\"{}\" y=\"{}\" width=\"32\" height=\"{solved}\" fill=\"#4e8f9c\"/><rect x=\"{}\" y=\"{}\" width=\"32\" height=\"{unsolved}\" fill=\"#b66a30\"/>",x-16.,590.-solved,x-16.,590.-solved-unsolved));
        if g % 2 == 0 {
            label(&mut popsvg, x - 16., 620., 21, &format!("G{g}"), "#526873");
        }
    }
    label(
        &mut popsvg,
        32.,
        658.,
        21,
        "Search fit includes legality penalties and novelty; not a physical qualification score.",
        "#a3442d",
    );
    popsvg.push_str("</g></svg>\n");
    out.insert("docs/paper/generated/population.svg".into(), popsvg);
    let summary = json!({"schema":"jarambit.publication_projection.v1","source_baseline":BASE,"scope":"Retained evidence projection; no new scientific execution","battery_subjects":files.len(),"battery_evaluators":evaluator_ids.len(),"stored_statuses":counts,"per_evaluator":per,"science_definitions":cases.len(),"science_family_counts":families,"population_births":births.len(),"population_statistics":stats,"historical_context":history,"weighted_curve":curve["generations"],"weighted_best":curve["best_so_far"]["ultralight_weighted_fitness"],"weighted_stage_lineage":"incomplete; retained summaries are not a homogeneous objective-convergence experiment","score_direction":"higher_is_better","missing":"Three proposal birth/screen pairs and a later unqualified Individual2002 summary are retained across accessions; full underlying CAD/scorecards remain unretained. See reporting-interpretation.v2.json.","weighted_interpretation":"Historical saved arithmetic; A4 term unadmitted and partly default-compatible/proxy inputs; no current qualification or proven original recipe execution"});
    out.insert(
        "docs/paper/generated/summary.json".into(),
        serde_json::to_string_pretty(&summary)? + "\n",
    );
    out.insert("docs/paper/generated/metrics.tex".into(),format!("% Generated publication projection; not a solver result.\n\\newcommand{{\\BestFitness}}{{{:.6}}}\n\\newcommand{{\\BatteryPass}}{{{}}}\n\\newcommand{{\\BatteryFail}}{{{}}}\n\\newcommand{{\\BatteryBlocked}}{{{}}}\n\\newcommand{{\\BirthCount}}{{{}}}\n",number(&curve["best_so_far"]["ultralight_weighted_fitness"])?,counts.get("pass").unwrap_or(&0),counts.get("fail").unwrap_or(&0),counts.get("blocked").unwrap_or(&0),births.len()));
    let metrics = out.get_mut("docs/paper/generated/metrics.tex").unwrap();
    for (name, value) in [
        ("FemComplete", &history["fem_status_counts"]["complete"]),
        (
            "FemNotEvaluated",
            &history["fem_status_counts"]["not_evaluated"],
        ),
        ("LegalBirths", &history["legal_count"]),
        ("HistoricalBestId", &history["best_observed"]["id"]),
        (
            "HistoricalBestGeneration",
            &history["best_observed"]["generation"],
        ),
    ] {
        metrics.push_str(&format!("\\newcommand{{\\{name}}}{{{value}}}\n"));
    }
    metrics.push_str(&format!(
        "\\newcommand{{\\HistoricalBestFit}}{{{:.5}}}\n",
        number(&history["best_observed"]["fit"])?
    ));
    let assessed = &history["best_observed"]["strict_assessment"];
    let all_pass = assessed["cert_pass_all"]
        .as_bool()
        .map(|v| v.to_string())
        .unwrap_or_else(|| "unavailable".into());
    metrics.push_str(&format!(
        "\\newcommand{{\\HistoricalBestAllPass}}{{{all_pass}}}\n"
    ));
    for rel in [
        "crates/paper-artifacts/src/publication.rs",
        "crates/paper-artifacts/src/lib.rs",
        "crates/paper-artifacts/src/appearance.rs",
        "crates/paper-artifacts/src/hero_appearance.rs",
        "crates/paper-artifacts/src/population_story.rs",
        "crates/paper-artifacts/src/saliency.rs",
        "crates/paper-artifacts/src/interpretation.rs",
        "crates/paper-artifacts/src/validation.rs",
        "crates/paper-artifacts/src/historical.rs",
        "crates/paper-artifacts/src/native_history.rs",
        "crates/paper-artifacts/src/lineage.rs",
        "crates/paper-artifacts/src/population.rs",
        "crates/paper-artifacts/src/display.rs",
        "crates/paper-artifacts/src/score_axis.rs",
        "crates/paper-artifacts/src/plate.rs",
        "crates/paper-artifacts/src/verification.rs",
        "crates/paper-artifacts/src/assessment.rs",
        "crates/paper-artifacts/src/battery.rs",
        "crates/paper-artifacts/src/export_identity.rs",
        "crates/paper-artifacts/src/weighted_runs.rs",
        "crates/paper-artifacts/src/reference.rs",
        "crates/paper-artifacts/src/visuals.rs",
        "crates/paper-artifacts/examples/paper_artifacts.rs",
        "crates/paper-artifacts/Cargo.toml",
        "crates/paper-artifacts/Cargo.lock",
        "Cargo.lock",
    ] {
        inputs.insert(rel.into(), digest(&fs::read(root.join(rel))?));
    }
    let mut table=String::from("% Generated stored-label counts, not physical-test outcomes.\n\\begin{table}[t]\n\\caption{Historical battery outcomes; P/F/B are stored labels.}\n\\centering\\small\n\\begin{tabular}{lrrr}\\toprule\nEvaluator & P & F & B\\\\\\midrule\n");
    for id in evaluators {
        let id = text(id)?;
        let c = &per[id];
        table.push_str(&format!(
            "{} & {} & {} & {}\\\\\n",
            id.replace('_', "\\_"),
            c.get("pass").unwrap_or(&0),
            c.get("fail").unwrap_or(&0),
            c.get("blocked").unwrap_or(&0)
        ));
    }
    table.push_str(&format!("\\midrule\nTotal & {} & {} & {}\\\\\\bottomrule\n\\end{{tabular}}\n\\label{{tab:battery}}\n\\end{{table}}\n",counts.get("pass").unwrap_or(&0),counts.get("fail").unwrap_or(&0),counts.get("blocked").unwrap_or(&0)));
    out.insert("docs/paper/generated/battery.tex".into(), table);
    let mut lineage_inputs = BTreeMap::new();
    for path in [
        "docs/paper/lineage-inputs.v1.json",
        POP,
        "evidence/rs_v2/w282_ultrarunner/manifest.json",
        "evidence/rs_v2/w282_ultrarunner/provenance.json",
        "evidence/rs_v2/w282_ultrarunner/search_summary.json",
        "crates/paper-artifacts/src/lineage.rs",
        "crates/paper-artifacts/src/historical.rs",
    ] {
        lineage_inputs.insert(path.to_string(), inputs[path].clone());
    }
    let lineage_manifest = json!({"schema":1,"figure":"docs/paper/generated/lineage.svg","figure_sha256":digest(out["docs/paper/generated/lineage.svg"].as_bytes()),"inputs":lineage_inputs,"script":"crates/paper-artifacts/src/lineage.rs","projection_receipt":"docs/paper/generated/lineage.json","scope":"five saved birth records and five original parent-ID links; two-hop cutoff, no inferred ancestry or geometry join"});
    out.insert(
        "docs/paper/generated/lineage.svg.manifest.json".into(),
        serde_json::to_string_pretty(&lineage_manifest)? + "\n",
    );
    let mut battery_figure_inputs = BTreeMap::new();
    for path in [
        "docs/paper/battery-inputs.v1.json",
        "evidence/cad_battery/manifest.json",
        "crates/paper-artifacts/src/battery.rs",
    ] {
        battery_figure_inputs.insert(path.to_string(), inputs[path].clone());
    }
    for path in battery_digests.keys() {
        battery_figure_inputs.insert(path.clone(), inputs[path].clone());
    }
    let battery_figure_manifest = json!({"schema":1,"figure":"docs/paper/generated/battery-matrix.svg","figure_sha256":digest(out["docs/paper/generated/battery-matrix.svg"].as_bytes()),"inputs":battery_figure_inputs,"script":"crates/paper-artifacts/src/battery.rs","projection_receipt":"docs/paper/generated/battery-matrix.json","scope":"414 original categorical status labels; shared references and record completeness are not independent physical tests"});
    out.insert(
        "docs/paper/generated/battery-matrix.svg.manifest.json".into(),
        serde_json::to_string_pretty(&battery_figure_manifest)? + "\n",
    );
    let mut plate_inputs = BTreeMap::new();
    for path in [
        "docs/paper/field-inputs.v1.json",
        field_source,
        "crates/paper-artifacts/src/plate.rs",
    ] {
        plate_inputs.insert(path.to_string(), inputs[path].clone());
    }
    for path in producer_sources.keys() {
        plate_inputs.insert(path.clone(), inputs[path].clone());
    }
    let plate_manifest = json!({"schema":1,"figure":"docs/paper/generated/plate-fields.svg","figure_sha256":digest(out["docs/paper/generated/plate-fields.svg"].as_bytes()),"inputs":plate_inputs,"script":"crates/paper-artifacts/src/plate.rs","projection_receipt":"docs/paper/generated/plate-fields.json","scope":"stored cell values only; no new mechanics, interpolation, clipping or solid field"});
    out.insert(
        "docs/paper/generated/plate-fields.svg.manifest.json".into(),
        serde_json::to_string_pretty(&plate_manifest)? + "\n",
    );
    let mut smart_inputs = BTreeMap::new();
    for path in [
        smart_recipe_path,
        "docs/paper/field-inputs.v1.json",
        field_source,
        "crates/paper-artifacts/src/plate.rs",
        "crates/paper-artifacts/src/saliency.rs",
    ] {
        smart_inputs.insert(path.to_string(), inputs[path].clone());
    }
    for path in producer_sources.keys() {
        smart_inputs.insert(path.clone(), inputs[path].clone());
    }
    let smart_manifest = json!({"schema":1,"figure":"docs/paper/generated/plate-smart-crops.svg",
        "figure_sha256":digest(out["docs/paper/generated/plate-smart-crops.svg"].as_bytes()),
        "inputs":smart_inputs,"script":"crates/paper-artifacts/src/plate.rs",
        "projection_receipt":"docs/paper/generated/plate-smart-crops.v1.json",
        "scope":"Display camera crop only; original piecewise-constant raw fields/masks/full-range keys; whole-field exact triangle-area coverage and original plate-boundary locator. No new mechanics or scalar-value clipping."});
    out.insert(
        "docs/paper/generated/plate-smart-crops.svg.manifest.json".into(),
        serde_json::to_string_pretty(&smart_manifest)? + "\n",
    );
    let mut verification_inputs = BTreeMap::new();
    for path in [
        "docs/paper/verification-inputs.v1.json",
        "crates/paper-artifacts/src/verification.rs",
    ] {
        verification_inputs.insert(path.to_string(), inputs[path].clone());
    }
    for record in sources.values() {
        let path = text(&record["path"])?;
        verification_inputs.insert(path.to_string(), inputs[path].clone());
    }
    for path in interpretation.keys() {
        verification_inputs.insert(path.clone(), inputs[path].clone());
    }
    let verification_manifest = json!({"schema":1,"figure":"docs/paper/generated/verification.svg","figure_sha256":digest(out["docs/paper/generated/verification.svg"].as_bytes()),"inputs":verification_inputs,"script":"crates/paper-artifacts/src/verification.rs","projection_receipt":"docs/paper/generated/verification.json","scope":"saved mesh levels, scalar pairs and binned profiles; no full solid field or admitted GCI estimate"});
    out.insert(
        "docs/paper/generated/verification.svg.manifest.json".into(),
        serde_json::to_string_pretty(&verification_manifest)? + "\n",
    );
    let mut scalar_inputs = BTreeMap::new();
    for path in [
        "docs/paper/assessment-inputs.v1.json",
        "crates/paper-artifacts/src/assessment.rs",
        "docs/tests/catalog.json",
    ] {
        scalar_inputs.insert(path.to_string(), inputs[path].clone());
    }
    for record in scalar_sources.values() {
        let path = text(&record["path"])?;
        scalar_inputs.insert(path.to_string(), inputs[path].clone());
    }
    for path in scalar_interpretation.keys() {
        scalar_inputs.insert(path.clone(), inputs[path].clone());
    }
    let scalar_manifest = json!({"schema":1,"figure":"docs/paper/generated/assessment.svg","figure_sha256":digest(out["docs/paper/generated/assessment.svg"].as_bytes()),"inputs":scalar_inputs,"script":"crates/paper-artifacts/src/assessment.rs","projection_receipt":"docs/paper/generated/assessment.json","scope":"original scalar observations and flags only; support model is schematic; no eigenmode, fatigue field or new qualification"});
    out.insert(
        "docs/paper/generated/assessment.svg.manifest.json".into(),
        serde_json::to_string_pretty(&scalar_manifest)? + "\n",
    );
    let mut export_inputs = BTreeMap::new();
    for path in [
        "docs/paper/export-inputs.v1.json",
        "evidence/evolve_certifiable/manifest.json",
        "crates/paper-artifacts/src/export_identity.rs",
        scene_path,
    ] {
        export_inputs.insert(path.to_string(), inputs[path].clone());
    }
    for source in export_sources.values() {
        let path = text(&source["path"])?;
        export_inputs.insert(path.to_string(), inputs[path].clone());
    }
    let export_manifest = json!({"schema":1,"figure":"docs/paper/generated/export-identity.svg","figure_sha256":digest(out["docs/paper/generated/export-identity.svg"].as_bytes()),"inputs":export_inputs,"script":"crates/paper-artifacts/src/export_identity.rs","projection_receipt":"docs/paper/generated/export-identity.json","scope":"original export record observations and content associations; no fresh CAD validation or genome regeneration"});
    out.insert(
        "docs/paper/generated/export-identity.svg.manifest.json".into(),
        serde_json::to_string_pretty(&export_manifest)? + "\n",
    );
    let mut weighted_inputs = BTreeMap::new();
    for path in [
        "docs/paper/weighted-inputs.v1.json",
        "crates/paper-artifacts/src/weighted_runs.rs",
        "crates/paper-artifacts/src/publication.rs",
    ] {
        weighted_inputs.insert(path.to_string(), inputs[path].clone());
    }
    for spec in weighted_specs {
        let path = text(&spec["path"])?;
        weighted_inputs.insert(path.to_string(), inputs[path].clone());
    }
    let weighted_manifest = json!({"schema":1,"figure":"docs/paper/generated/weighted-runs.svg","figure_sha256":digest(out["docs/paper/generated/weighted-runs.svg"].as_bytes()),"inputs":weighted_inputs,"script":"crates/paper-artifacts/src/weighted_runs.rs","projection_receipt":"docs/paper/generated/weighted-runs.json","scope":"six distinct retained reporting files, fifteen saved generation summaries; no interpolation, pooled trajectory or established stage/scorer comparability"});
    out.insert(
        "docs/paper/generated/weighted-runs.svg.manifest.json".into(),
        serde_json::to_string_pretty(&weighted_manifest)? + "\n",
    );
    let mut reference_inputs = BTreeMap::new();
    for path in [
        "docs/paper/reference-inputs.v1.json",
        "evidence/evolve_certifiable/manifest.json",
        "crates/paper-artifacts/src/reference.rs",
        "crates/paper-artifacts/src/publication.rs",
    ] {
        reference_inputs.insert(path.to_string(), inputs[path].clone());
    }
    for source in reference_sources.values() {
        let path = text(&source["path"])?;
        reference_inputs.insert(path.to_string(), inputs[path].clone());
    }
    for path in reference_producers.keys() {
        reference_inputs.insert(path.clone(), inputs[path].clone());
    }
    let reference_manifest = json!({"schema":1,"figure":"docs/paper/generated/reference.svg","figure_sha256":digest(out["docs/paper/generated/reference.svg"].as_bytes()),"inputs":reference_inputs,"script":"crates/paper-artifacts/src/reference.rs","projection_receipt":"docs/paper/generated/reference.json","scope":"Original serialized silhouettes and saved rounded reference descriptors/flags; no new measurement, registration, histogram, FEM or human approval"});
    out.insert(
        "docs/paper/generated/reference.svg.manifest.json".into(),
        serde_json::to_string_pretty(&reference_manifest)? + "\n",
    );
    let visual_coverage = crate::visuals::project(entries, &out)?;
    out.insert(
        "docs/tests/visuals.md".into(),
        crate::visuals::markdown(&visual_coverage)?,
    );
    out.insert(
        "docs/tests/visuals.v1.json".into(),
        serde_json::to_string_pretty(&visual_coverage)? + "\n",
    );
    let architecture_path = "docs/paper/figures/evidence-architecture.svg";
    let mut architecture_inputs = BTreeMap::new();
    for path in [
        architecture_path,
        "crates/paper-artifacts/src/publication.rs",
        "docs/paper/sections/study-design.tex",
        "docs/paper/supplement/cohort-inventory.tex",
    ] {
        let hash = digest(&fs::read(safe_path(&root, path)?)?);
        inputs.insert(path.to_string(), hash.clone());
        architecture_inputs.insert(path.to_string(), hash);
    }
    let architecture_manifest = json!({
        "schema":"jarambit.authored_figure_manifest.v1",
        "figure":architecture_path,
        "figure_sha256":architecture_inputs[architecture_path],
        "inputs":architecture_inputs,
        "script":"crates/paper-artifacts/src/publication.rs",
        "script_role":"Manifest writer and input validator; the SVG is a static authored explanatory input, not generated CAD or assessment data",
        "role":"Authored explanatory diagram bound to retained study coverage; not a generated assessment join",
        "scope":"Separate search denominators and individually assessed export bundles; no per-birth native CAD/assessment or physical validation claim"
    });
    out.insert(
        "docs/paper/generated/evidence-architecture.svg.manifest.json".into(),
        serde_json::to_string_pretty(&architecture_manifest)? + "\n",
    );
    crate::population::generate_v2_outputs(&root, &mut out, &mut inputs)?;
    let outputs = out
        .iter()
        .map(|(k, v)| (k.clone(), digest(v.as_bytes())))
        .collect::<BTreeMap<_, _>>();
    let manifest = json!({"schema":"jarambit.publication_manifest.v1","baseline":BASE,"inputs":inputs,"outputs":outputs,"generation":"cargo run --locked --manifest-path crates/paper-artifacts/Cargo.toml --example paper_artifacts","scope":"Documentation only; source hashes establish identity, not physical truth",
    "visual_coverage":"docs/tests/visuals.v1.json",
    "figures": {
        "evidence-architecture.svg":{"source":"docs/paper/figures/evidence-architecture.svg","manifest":"docs/paper/generated/evidence-architecture.svg.manifest.json","scope":"Authored reporting diagram, not a generated join or population-wide assessment rate"},
        "weighted-runs.svg":{"contract":"docs/paper/weighted-inputs.v1.json","projection_receipt":"docs/paper/generated/weighted-runs.json","membership":"six admitted source files / fifteen generation summaries / thirty saved score observations","selectors":["/generations/*/generation","/generations/*/round_best_fitness","/generations/*/hall_of_fame_best_so_far","/generations/*/best_so_far_label","/generations/*/round_file","/best_so_far/label","/best_so_far/ultralight_weighted_fitness"],"scope":"retained reporting summaries only; stage/scorer lineage not established, no homogeneous-convergence or independent-replication claim"},
        "export-identity.svg":{"contract":"docs/paper/export-inputs.v1.json","projection_receipt":"docs/paper/generated/export-identity.json","selectors":["/cad","/genome_sha256","/genome_csv","/solids","/valid","/vertices","/triangles"],"scope":"stored STEP token counts, original exporter census and BRep observations plus Blender import metadata; no fresh validation or qualification"},
        "lineage.svg":{"contract":"docs/paper/lineage-inputs.v1.json","projection_receipt":"docs/paper/generated/lineage.json","selectors":["/*/id","/*/gen","/*/fit","/*/parents","/*/op","/*/op_version","/*/fem_status","/*/legal"],"membership":"two recorded parent hops from highest historical stored fit, with original parent-list order","scope":"layout does not encode elapsed time or fitness; no inferred inheritance or best-geometry identity"},
        "battery-matrix.svg":{"contract":"docs/paper/battery-inputs.v1.json","projection_receipt":"docs/paper/generated/battery-matrix.json","membership":"manifest subjects and case_ids; exact-ID cell joins","selectors":["/subject","/role","/cases/*/id","/cases/*/status","/cases/*/fresh_this_lane","/cases/*/kind","/cases/*/evidence_path"],"scope":"stored historical labels only; not a FEM heatmap, new assessment or physical reliability estimate"},
        "assessment.svg":{"contract":"docs/paper/assessment-inputs.v1.json","projection_receipt":"docs/paper/generated/assessment.json","selectors":["/buckling_eigenvalue","/supported_seq_ratio","/fat_sf","/hm_*_sf","/strict_lat_rob_0p0625_n","/twist_tipzone_sf_strict","/weighted_cad_score/gates","/gates"],"scope":"saved scalars and overlapping status flags; no solver or new physical evidence","support_audit":"eigsh operator symmetry/eigenpair residual unresolved; no eigenvector retained"},
        "verification.svg":{"contract":"docs/paper/verification-inputs.v1.json","projection_receipt":"docs/paper/generated/verification.json","mesh_selectors":["/levels_fine_to_coarse","/seq_ratio_gci","/sf_hunter_gci"],"solver_selectors":["/solid/SF_solid_nodal_avg","/solid/SF_solid_nodal_max","/SF_nodal_avg","/SF_nodal_max"],"profile_selector":"/profile_MPa","scope":"retained observations; separate recovery definitions and original ROI; no new solve, estimate, interpolation or pointwise difference"},
        "plate-fields.svg":{"source":field_source,"contract":"docs/paper/field-inputs.v1.json","projection_receipt":"docs/paper/generated/plate-fields.json","selectors":["/mesh/xy","/mesh/tris","/mesh/env_vm","/mesh/lat","/mesh/ring","/mesh/twist"],"association":"one stored rounded value per triangle; no interpolation","limits":"independent zero-to-source-maximum scales; no clipping","masks":"source-forced zeros only; raw unit-response tip region retained","scope":"retained 0.25 mm2 plate mesh, not strict or solid fields"},
        "cover-evolution.svg":{"source":POP,"membership":"all retained births in panel B; per-generation and cumulative maxima of stored fit in panel C","selectors":["/*/id","/*/gen","/*/fit","/*/fem_status"],"population_vertical_transform":"asinh(fit); original-value ticks","maximum_axis_limits":[1.135,1.16],"maximum_axis_truncated":true,"lines":"straight segments between observed generations; no fitted convergence model","producer_revision":history["producer_revision"],"semantics":history["fitness_semantics"]},
        "fitness.svg": {"weighted_source":CURVE,"weighted_selectors":["/best_so_far/ultralight_weighted_fitness","/best_so_far/ultralight_fitness_breakdown/objectives/*/weighted_points","/generations/*/generation","/generations/*/round_best_fitness","/generations/*/hall_of_fame_best_so_far"],"weighted_direction":"higher_is_better","weighted_interpretation":"historical saved arithmetic; unadmitted A4 and partly default-compatible/proxy terms; no current qualification","interpretation_contract":"docs/paper/data/reporting-interpretation-contract.v2.json","weighted_stage_lineage":"incomplete; reporting producer can substitute assessment values for screening values","population_source":POP,"population_selectors":["/*/id","/*/gen","/*/fit"],"population_reduction":"all retained births; generation maximum and sorted-sample median","population_vertical_transform":"asinh(fit); ticks in original units","regimes":"separate panels; no cross-regime score comparison"},
        "population.svg":{"source":POP,"membership":"all retained births, including not-evaluated penalty records","selectors":["/*/id","/*/gen","/*/fit","/*/fem_status","/*/legal"],"reduction":"generation median and maximum; status census by generation; no imputation","vertical_transform":"asinh(fit); ticks in original units","producer_revision":history["producer_revision"],"producer_qd_sha256":history["producer_qd_sha256"],"semantics":history["fitness_semantics"]},
        "population-births.svg":{"source":POP,"projection_receipt":"docs/paper/generated/population/historical_births.json","membership":"all 1,984 historical w282 births in Run 1 (1,455 FEM-complete + 529 not-evaluated; 446 legal)","vertical_transform":"asinh(fit); ticks in original units"},
        "population-membership.svg":{"source":POP,"projection_receipt":"docs/paper/generated/population/historical_membership.json","membership":"all 2,176 island-membership occurrences in Run 1 (1,984 births + 192 elite reappearances) and 12 ring migrations at G4, G8, G12 (24 copied IDs)"},
        "population-newer.svg":{"contract":"docs/paper/JR_EI_P1_T05_NEWER_BIRTHS_SCREENS_JOIN_RECEIPT_20261019.json","projection_receipt":"docs/paper/generated/population/newer_events.json","membership":"57 birth files across 24 run directories (3,649 births = 2,259 1-based joined screens + 1,390 unscreened births); proportional 0..3,200 common scale"},
        "population-latest.svg":{"contract":"artifacts/versioned-evolution-archives-gen800-population-v1/index.gen800_population.sha256.json","projection_receipt":"docs/paper/generated/population/point-rendering.v1.json","membership":"all 1,954,560 admitted native rows as direct neutral point operations, plus separate 1,984 historical birth and 2,259 numeric screen panels; 1,390 unknown proposals have a nonnumeric coverage row","axis_contract":"docs/paper/data/score-axis-contract.v1.json","axis_receipt":"docs/paper/generated/population/score-axis.v1.json","vertical_transform":"source-derived per-regime linear or asinh(y/L), explicit L and original-unit ticks; full actual observed domain without clipping or jitter","class_handling":"neutral occupancy, no class-majority substitution; counts/IDs/extrema/overlap in point receipt"},
        "fitness-progress.svg":{"contract":"docs/paper/data/score_profiles.v1.json","projection_receipt":"docs/paper/generated/shared_projection.v2.json","membership":"three separate numerical-axis panels for Cohort 1 (Run 1, 17-gen), Cohort 2 (weighted summaries), and Cohort 3 (Run 2..Run 11, 10 retained 800-gen runs qd_fit)"},
        "hero-full-range.svg":{"source":"docs/paper/drafts_p3_p5/JR_EI_P4_T01_blender_exact_cad_hero_white.svg","contract":"docs/paper/data/display-composite-contract.v1.json","roster":"docs/paper/data/test-field-roster.v1.json","independent_contract_review":"docs/paper/data/display-composite-review.v1.json","projection_receipt":"docs/paper/generated/display-composite.v1.json","surface":"retained STL SVG projection, flat shared palette, no fresh Cycles","supersedes":"historical figure-semantics.v1.json#/hero_composite five-map recipe; original bytes preserved","scope":"4 available retained response channels;44catalog entries+8metadata addenda are not52independent tests; not scientific fitness"},
        "hero-support.svg":{"contract":"docs/paper/data/display-composite-contract.v1.json","projection_receipt":"docs/paper/generated/display-composite.v1.json","denominator":"4 eligible registered response channels; exact original source-triangle area, before overlays/projection"},
        "hero-contrast-preview.svg":{"contract":"docs/paper/data/display-composite-contract.v1.json","projection_receipt":"docs/paper/generated/display-composite.v1.json","scope":"additional Q05–Q95 contrast preview with original index thresholds/tails; full[0,1] companion required; not selected without independent readability review"},
        "hero-mesh-on.svg":{"contract":"docs/paper/data/display-composite-contract.v1.json","scope":"all exact retained visible STL triangle edges,0.12unit line width/0.03opacity per triangle; mesh-off fullrange remains default"},
        "population-detail.svg":{"contract":"docs/paper/data/score-axis-contract.v1.json","projection_receipt":"docs/paper/generated/population/point-rendering.v1.json","scope":"additional empirical Q01–Q99 detail with explicit strict-tail counts and inclusive tied boundaries; fullrange allrecord population-latest.svg retained"},
        "inheritance-details.svg":{"contract":"docs/paper/generated/population/inheritance.json","projection_receipt":"docs/paper/generated/population/inheritance.json","scope":"Supplementary raw coordinate differences and mixed-unit norms; not normalized or physical distance"},
        "inheritance.svg":{"contract":"docs/paper/generated/population/inheritance.json","projection_receipt":"docs/paper/generated/population/inheritance.json","membership":"7-node 3-hop ancestry DAG plus recorded induced fourth-hop closure around Individual 1095; compact IDs, generations and operations; exact coordinate differences retained in unchanged ledger and supplementary view"}
    }});
    out.insert(
        "docs/paper/generated/manifest.json".into(),
        serde_json::to_string_pretty(&manifest)? + "\n",
    );
    // Check the complete projection before writing any output.
    if check {
        let stale = out
            .iter()
            .filter(|(p, v)| {
                safe_path(&root, p)
                    .ok()
                    .and_then(|p| fs::read(p).ok())
                    .as_deref()
                    != Some(v.as_bytes())
            })
            .map(|(p, _)| p.clone())
            .collect::<Vec<_>>();
        if !stale.is_empty() {
            return Err(format!("stale publication outputs: {}", stale.join(", ")).into());
        }
    } else {
        for (rel, value) in &out {
            let p = safe_path(&root, rel)?;
            fs::create_dir_all(p.parent().ok_or("missing parent")?)?;
            fs::write(p, value)?;
        }
    }
    println!(
        "publication {}: {} outputs, {} historical cells, {} science definitions",
        if check { "check" } else { "write" },
        out.len(),
        counts.values().sum::<usize>(),
        cases.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn svg_coordinates_discard_platform_ulp_noise_but_not_source_values() {
        // Actual Linux/macOS mismatches from the reviewed asinh projections.
        for (a, b, expected) in [
            (357.5170852133724, 357.51708521337247, "357.517"),
            (256.1465474753194, 256.14654747531944, "256.147"),
        ] {
            assert_ne!(a, b);
            assert_eq!(svg_coord(a), expected);
            assert_eq!(svg_coord(b), expected);
            let (mut one, mut two) = (String::new(), String::new());
            label(&mut one, 4., a, 17, "original-value tick", "#526873");
            label(&mut two, 4., b, 17, "original-value tick", "#526873");
            assert_eq!(one, two);
            assert_ne!(json!(a), json!(b)); // Numerical receipts retain their inputs.
        }
        for value in [-200.123456789, -1.234499, 0.0, 1.15727, 599.999999999] {
            let shown: f64 = svg_coord(value).parse().unwrap();
            assert!((shown - value).abs() <= 0.000500000001);
        }
    }
    #[test]
    fn changed_or_unbound_render_geometry_is_refused() {
        let hash = "a".repeat(64);
        let scene = json!({"geometry_changed":false,"mesh_modifiers":[],
            "imported_vertices_before_sha256":hash,"imported_vertices_after_sha256":hash,
            "faces_before_sha256":hash,"faces_after_sha256":hash});
        assert!(validate_immutable_scene(&scene).is_ok());
        for (field, value) in [
            ("geometry_changed", json!(true)),
            ("mesh_modifiers", json!(["bevel"])),
            ("faces_after_sha256", json!("b".repeat(64))),
            ("imported_vertices_before_sha256", Value::Null),
        ] {
            let mut changed = scene.clone();
            changed[field] = value;
            assert!(validate_immutable_scene(&changed).is_err());
        }
    }
    #[test]
    fn median_handles_zero_even_and_invalid_samples() {
        assert_eq!(median(&[4., 0., 2., 8.]).unwrap(), 3.);
        assert!(median(&[]).is_err());
        assert!(median(&[f64::NAN]).is_err());
        assert!(number(&json!(true)).is_err());
        assert_eq!(number(&json!(0)).unwrap(), 0.);
    }
    #[test]
    fn duplicate_identity_and_path_escape_are_refused() {
        assert!(unique(["one", "one"].into_iter()).is_err());
        assert!(safe_path(Path::new("."), "../evidence.json").is_err());
        assert!(safe_path(Path::new("."), "/tmp/evidence.json").is_err());
    }
    #[test]
    fn hall_of_fame_is_not_round_maximum_or_loss() {
        let mut v = json!({"metric":"W_ULTRALIGHT_v1","generations":[{"generation":0,"round_best_fitness":70.,"hall_of_fame_best_so_far":70.},{"generation":1,"round_best_fitness":65.,"hall_of_fame_best_so_far":70.}]});
        assert!(validate_curve(&v).is_ok());
        v["generations"][1]["hall_of_fame_best_so_far"] = json!(65.);
        assert!(validate_curve(&v).is_err());
        v["metric"] = json!("new_loss");
        assert!(validate_curve(&v).is_err());
    }
    #[test]
    fn saved_weighted_view_preserves_values_without_admitting_a4_or_default_inputs() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let mut inputs = BTreeMap::new();
        let curve = read(&root, CURVE, &mut inputs).unwrap();
        let births = read(&root, POP, &mut inputs).unwrap();
        let original = curve.clone();
        let svg = hero(&curve, array(&births).unwrap()).unwrap();
        let best = &curve["best_so_far"];
        let terms = best["ultralight_fitness_breakdown"]["objectives"]
            .as_object()
            .unwrap();
        assert_eq!(terms.len(), 20);
        assert_eq!(svg.matches("<rect x=\"330\"").count(), 40);
        assert!(svg.contains(&format!(
            "Saved F = {:.6}",
            number(&best["ultralight_weighted_fitness"]).unwrap()
        )));
        for term in terms.values() {
            assert!(svg.contains(&format!(
                "{:.2}/{}",
                number(&term["weighted_points"]).unwrap(),
                number(&term["weight_pct"]).unwrap()
            )));
        }
        assert_eq!(curve, original);
        assert!(svg.contains("3.50/3.5"));
        assert!(svg.contains("2.50/2.5"));
        assert!(svg.contains(&format!(
            "Historical producer tally: {}/26",
            best["ultralight_fitness_breakdown"]["legacy_hard_gates_passed_count"]
                .as_u64()
                .unwrap()
        )));
        assert!(svg.contains("HISTORICAL SAVED WEIGHTED ARITHMETIC"));
        assert!(svg.contains("A4 term (unadmitted)"));
        assert!(svg.contains("partly default-compatible/proxy terms"));
        assert!(svg.contains("Recipe execution/call inputs unestablished"));
        assert!(svg.contains("current dependent qualification unestablished"));
        assert!(!svg.contains("checks passed"));
        assert!(!svg.contains("Lateral buckling"));
    }
    #[test]
    fn inconsistent_reports_are_refused_before_write_and_in_check_mode() {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(0);
        struct Fixture(std::path::PathBuf);
        impl Drop for Fixture {
            fn drop(&mut self) {
                let _ = fs::remove_dir_all(&self.0);
            }
        }
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .unwrap();
        let manifest: Value = serde_json::from_slice(
            &fs::read(root.join("docs/paper/generated/manifest.json")).unwrap(),
        )
        .unwrap();
        let parent = root.join("target/paper-report-fixtures");
        fs::create_dir_all(&parent).unwrap();
        for (mutation, expected_error) in [
            ("score", "selected best differs"),
            ("label", "selected label differs"),
            ("tally", "tally differs"),
        ] {
            let path = parent.join(format!(
                "{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            let fixture = Fixture(path);
            for rel in manifest["inputs"].as_object().unwrap().keys() {
                let source = safe_path(&root, rel).unwrap();
                let dest = safe_path(&fixture.0, rel).unwrap();
                fs::create_dir_all(dest.parent().unwrap()).unwrap();
                fs::copy(source, dest).unwrap();
            }
            let curve_path = fixture.0.join(CURVE);
            let mut curve: Value = serde_json::from_slice(&fs::read(&curve_path).unwrap()).unwrap();
            match mutation {
                "score" => {
                    for row in curve["generations"].as_array_mut().unwrap() {
                        row["round_best_fitness"] = json!(0.0);
                        row["hall_of_fame_best_so_far"] = json!(0.0);
                    }
                }
                "label" => {
                    curve["best_so_far"]["label"] = json!("REVIEW_FIXTURE_WRONG_SUBJECT");
                }
                "tally" => {
                    curve["best_so_far"]["ultralight_fitness_breakdown"]
                        ["legacy_hard_gates_passed_count"] = json!(26);
                }
                _ => unreachable!(),
            }
            fs::write(curve_path, serde_json::to_vec(&curve).unwrap()).unwrap();
            let sentinel = fixture.0.join("docs/paper/generated/summary.json");
            fs::create_dir_all(sentinel.parent().unwrap()).unwrap();
            fs::write(&sentinel, b"unchanged before rejected publication").unwrap();
            for check in [false, true] {
                let error = generate(&fixture.0, check).unwrap_err().to_string();
                assert!(error.contains(expected_error), "{mutation}: {error}");
                assert_eq!(
                    fs::read(&sentinel).unwrap(),
                    b"unchanged before rejected publication"
                );
                assert!(!fixture.0.join("docs/paper/generated/fitness.svg").exists());
                assert!(!fixture.0.join("docs/tests/README.md").exists());
            }
        }
    }
    #[test]
    fn committed_publication_inputs_reconcile() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        generate(&root, true).unwrap();
    }
}
