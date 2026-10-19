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
fn label(s: &mut String, x: f64, y: f64, size: usize, content: &str, color: &str) {
    s.push_str(&format!(
        "<text x=\"{x}\" y=\"{y}\" font-size=\"{size}\" fill=\"{color}\">{}</text>\n",
        xml(content)
    ));
}

/// Verify the declared metric and the exact retained Hall-of-Fame recurrence.
fn validate_curve(curve: &Value) -> Result<()> {
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

fn hero(curve: &Value, births: &[Value]) -> Result<String> {
    validate_curve(curve)?;
    let best = &curve["best_so_far"];
    let f = number(&best["ultralight_weighted_fitness"])?;
    let terms = best["ultralight_fitness_breakdown"]["objectives"]
        .as_object()
        .ok_or("missing objective profile")?;
    let mut sum = 0.0;
    let mut s = svg_start(1200, 620);
    label(
        &mut s,
        32.,
        37.,
        16,
        "RETAINED FITNESS / NOT PHYSICAL QUALIFICATION",
        "#007e87",
    );
    label(&mut s, 32., 113., 67, &format!("{f:.2}"), "#132d3b");
    label(
        &mut s,
        230.,
        108.,
        20,
        "/ 100 · higher is better",
        "#526873",
    );
    label(&mut s, 32., 149., 19, text(&best["label"])?, "#132d3b");
    label(
        &mut s,
        32.,
        181.,
        17,
        "Best reported profile in selected w411c3 cohort",
        "#526873",
    );
    for (i, (name, keys, cap)) in REPORT_GROUPS.iter().enumerate() {
        let points = keys
            .iter()
            .map(|k| number(&terms[*k]["weighted_points"]))
            .collect::<Result<Vec<_>>>()?
            .iter()
            .sum::<f64>();
        sum += points;
        let y = 237. + i as f64 * 70.;
        label(&mut s, 32., y, 17, name, "#132d3b");
        label(
            &mut s,
            382.,
            y,
            17,
            &format!("{points:.2} / {cap:.0}"),
            "#526873",
        );
        s.push_str(&format!("<rect x=\"32\" y=\"{}\" width=\"445\" height=\"14\" rx=\"7\" fill=\"#dce4e3\"/><rect x=\"32\" y=\"{}\" width=\"{}\" height=\"14\" rx=\"7\" fill=\"#007e87\"/>\n", y+12., y+12., 445. * points / cap));
    }
    if (sum - f).abs() > 0.00001 {
        return Err("profile sum differs from displayed fitness".into());
    }
    let tally = best["ultralight_fitness_breakdown"]["legacy_hard_gates_passed_count"]
        .as_u64()
        .ok_or("missing legacy tally")?;
    label(
        &mut s,
        32.,
        523.,
        16,
        &format!("{tally}/26 legacy predicates met; shared observations overlap"),
        "#a3442d",
    );
    label(
        &mut s,
        32.,
        552.,
        16,
        "Reporting profile includes placeholder inputs",
        "#a3442d",
    );
    label(
        &mut s,
        32.,
        580.,
        16,
        "Best-record CAD geometry unavailable in retained cohort",
        "#526873",
    );
    label(
        &mut s,
        590.,
        37.,
        16,
        "RETAINED SUMMARIES / SEPARATE REGIMES",
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
    let sx = |g: f64| 640. + 480. * g / maxg;
    let sy = |f: f64| 263. - 150. * (f - lo) / (hi - lo);
    for i in 0..4 {
        let v = lo + (hi - lo) * i as f64 / 3.;
        let y = sy(v);
        s.push_str(&format!("<path d=\"M640 {y}H1120\" stroke=\"#d5dfdf\"/>"));
        label(&mut s, 584., y + 5., 15, &format!("{v:.2}"), "#526873");
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
        label(
            &mut s,
            sx(number(&r["generation"])?) - 5.,
            287.,
            16,
            &r["generation"].to_string(),
            "#526873",
        );
    }
    label(&mut s, 786., 309., 15, "Generation (zero-based)", "#526873");
    label(
        &mut s,
        640.,
        82.,
        17,
        "A. w411c3 reporting summary; truncated scale",
        "#526873",
    );
    label(
        &mut s,
        640.,
        337.,
        14,
        "Stage lineage incomplete; teal: archive / orange: round",
        "#132d3b",
    );
    label(
        &mut s,
        640.,
        370.,
        16,
        &format!("B. w282 historical fit; {} retained births", births.len()),
        "#526873",
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
    let px = |g: f64| 640. + 480. * g / max_generation;
    let py = |f: f64| 505. - 110. * (f.asinh() - fit_lo) / (fit_hi - fit_lo).max(1e-9);
    for tick in [-200.0_f64, -10., -1., 1.] {
        label(
            &mut s,
            590.,
            py(tick) + 4.,
            13,
            &format!("{tick:.0}"),
            "#526873",
        );
        s.push_str(&format!(
            "<path d=\"M640 {}H1120\" stroke=\"#d5dfdf\"/>",
            py(tick)
        ));
    }
    let mut gen_values: BTreeMap<u64, Vec<f64>> = BTreeMap::new();
    for (b, f) in births.iter().zip(fit_values) {
        let g = b["gen"].as_u64().ok_or("invalid birth generation")?;
        gen_values.entry(g).or_default().push(f);
        s.push_str(&format!(
            "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"1.2\" fill=\"#78999e\" opacity=\".18\"/>",
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
            "<polyline points=\"{points}\" stroke=\"{color}\" stroke-width=\"2\" fill=\"none\"/>"
        ));
    }
    for g in [0., 8., 16.] {
        label(&mut s, px(g) - 4., 526., 13, &format!("{g:.0}"), "#526873");
    }
    label(
        &mut s,
        640.,
        550.,
        14,
        "Generation; asinh vertical scale, ticks in original fit",
        "#526873",
    );
    label(
        &mut s,
        640.,
        575.,
        14,
        "Teal: generation max    Orange: median    Dots: births",
        "#132d3b",
    );
    label(
        &mut s,
        640.,
        600.,
        14,
        "Historical data; no current qualification inferred",
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
    for file in files {
        let d = read(&root, text(file)?, &mut inputs)?;
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
    }
    if actual_subjects != declared_subjects {
        return Err("manifest subject membership mismatch".into());
    }
    let mut out: BTreeMap<String, String> = BTreeMap::new();
    let mut atlas=String::from("# Engineering assessment atlas\n\nGenerated by `cargo run --locked --manifest-path crates/paper-artifacts/Cargo.toml --example paper_artifacts`. Explanations describe existing assessments; no new CAD, FEM or physical tests run during generation.\n\n**16 legacy predicates, ten overlapping RS-v2 groups, and 18 battery evaluators count different things.** The historical battery has 23 subjects and 414 retained cells; all cells say `fresh_this_lane: false`. A complete result can contain an unfavorable finding.\n\n[Science: all 178 definitions](science.md) · [Paper](../paper/README.md) · [Engineering specification](../PAPER_AND_README_ENGINEERING_SPEC.md)\n\n");
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
            } else {
                atlas.push_str(if kind=="G16" {"**Retained example:** [corrected candidate assessment](../../evidence/rs_v2/w283_ultrarunner/w283_p14_c01_cad_eval.json), select `weighted_cad_score.gates` by exact ID. [Predicate authority](../../ops/ci/evaluate_cad_candidate.py). No current qualification follows from a historical favorable field.\n\n"} else {"**Retained example:** [historical grouped result](../../evidence/rs_v2/w283_ultrarunner/w283_p14_c01_rs.json). Use exact group IDs from `rs_v2_eval` in the implementation; do not infer group identity from table position. [Requirement namespace](../../requirements/RS-karambit-v2.toml).\n\n"});
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
    let mut popsvg = svg_start(1200, 610);
    label(
        &mut popsvg,
        32.,
        37.,
        20,
        &format!(
            "POPULATION BEHAVIOR / {} RETAINED BIRTHS / HISTORICAL w282",
            births.len()
        ),
        "#007e87",
    );
    label(&mut popsvg,32.,69.,18,"Stored historical fit; asinh scale with original-value ticks. Penalty-valued records remain visible.","#526873");
    label(
        &mut popsvg,
        32.,
        96.,
        18,
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
    let sx = |g: f64| 95. + 1040. * g / maxg.max(1.);
    let sy = |v: f64| 360. - 235. * (v.asinh() - lo.asinh()) / (hi.asinh() - lo.asinh()).max(1e-9);
    for v in [-200.0_f64, -100., -10., -1., 0., 1.] {
        let y = sy(v);
        popsvg.push_str(&format!("<path d=\"M95 {y}H1135\" stroke=\"#d5dfdf\"/>"));
        label(&mut popsvg, 10., y + 5., 15, &format!("{v:.2}"), "#526873");
    }
    for row in births {
        let x = sx(number(&row["gen"])?);
        let y = sy(number(&row["fit"])?);
        if row["fem_status"] == "complete" {
            popsvg.push_str(&format!(
                "<circle cx=\"{x:.2}\" cy=\"{y:.2}\" r=\"1.8\" fill=\"#4e8f9c\" opacity=\".25\"/>"
            ));
        } else {
            popsvg.push_str(&format!("<path d=\"M{} {}l4 4m0 -4l-4 4\" stroke=\"#b66a30\" stroke-width=\"1\" opacity=\".5\"/>",x-2.,y-2.));
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
            "<polyline points=\"{pts}\" stroke=\"{color}\" stroke-width=\"3\" fill=\"none\"/>"
        ));
    }
    for g in groups.keys().step_by((groups.len() / 8).max(1)) {
        label(
            &mut popsvg,
            sx(*g as f64) - 5.,
            390.,
            15,
            &g.to_string(),
            "#526873",
        );
    }
    label(&mut popsvg,95.,422.,18,"Blue dots: FEM complete   Amber crosses: not evaluated   Teal: maximum   Dark: pooled median", "#132d3b");
    label(
        &mut popsvg,
        95.,
        454.,
        18,
        "FEM status by generation (record counts; numbers above bars are total births)",
        "#526873",
    );
    let max_n = stats
        .iter()
        .map(|r| r["n"].as_u64().unwrap())
        .max()
        .unwrap() as f64;
    for row in &stats {
        let x = sx(number(&row["generation"])?);
        let solved = number(&row["fem_complete"])? * 65. / max_n;
        let unsolved = number(&row["fem_not_evaluated"])? * 65. / max_n;
        popsvg.push_str(&format!("<rect x=\"{}\" y=\"{}\" width=\"30\" height=\"{solved}\" fill=\"#4e8f9c\"/><rect x=\"{}\" y=\"{}\" width=\"30\" height=\"{unsolved}\" fill=\"#b66a30\"/>",x-15.,545.-solved,x-15.,545.-solved-unsolved));
        label(
            &mut popsvg,
            x - 13.,
            545. - solved - unsolved - 7.,
            15,
            &row["n"].to_string(),
            "#526873",
        );
        label(
            &mut popsvg,
            x - 8.,
            568.,
            15,
            &row["generation"].to_string(),
            "#526873",
        );
    }
    label(&mut popsvg,95.,599.,17,"Fit includes legality penalties and contextual novelty; these are not fixed physical-performance values.", "#a3442d");
    popsvg.push_str("</g></svg>\n");
    out.insert("docs/paper/generated/population.svg".into(), popsvg);
    let summary = json!({"schema":"jarambit.publication_projection.v1","source_baseline":BASE,"scope":"Retained evidence projection; no new scientific execution","battery_subjects":files.len(),"battery_evaluators":evaluator_ids.len(),"stored_statuses":counts,"per_evaluator":per,"science_definitions":cases.len(),"science_family_counts":families,"population_births":births.len(),"population_statistics":stats,"historical_context":history,"weighted_curve":curve["generations"],"weighted_best":curve["best_so_far"]["ultralight_weighted_fitness"],"weighted_stage_lineage":"incomplete; retained summaries are not a homogeneous objective-convergence experiment","score_direction":"higher_is_better","missing":"Weighted cohort CAD geometry and round population files are unavailable in retained cohort"});
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
        "crates/paper-artifacts/src/validation.rs",
        "crates/paper-artifacts/src/historical.rs",
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
    let outputs = out
        .iter()
        .map(|(k, v)| (k.clone(), digest(v.as_bytes())))
        .collect::<BTreeMap<_, _>>();
    let manifest = json!({"schema":"jarambit.publication_manifest.v1","baseline":BASE,"inputs":inputs,"outputs":outputs,"generation":"cargo run --locked --manifest-path crates/paper-artifacts/Cargo.toml --example paper_artifacts","scope":"Documentation only; source hashes establish identity, not physical truth",
    "figures": {
        "fitness.svg": {"weighted_source":CURVE,"weighted_selectors":["/best_so_far/ultralight_weighted_fitness","/best_so_far/ultralight_fitness_breakdown/objectives/*/weighted_points","/generations/*/generation","/generations/*/round_best_fitness","/generations/*/hall_of_fame_best_so_far"],"weighted_direction":"higher_is_better","weighted_stage_lineage":"incomplete; reporting producer can substitute assessment values for screening values","population_source":POP,"population_selectors":["/*/id","/*/gen","/*/fit"],"population_reduction":"all retained births; generation maximum and sorted-sample median","population_vertical_transform":"asinh(fit); ticks in original units","regimes":"separate panels; no cross-regime score comparison"},
        "population.svg":{"source":POP,"membership":"all retained births, including not-evaluated penalty records","selectors":["/*/id","/*/gen","/*/fit","/*/fem_status","/*/legal"],"reduction":"generation median and maximum; status census by generation; no imputation","vertical_transform":"asinh(fit); ticks in original units","producer_revision":history["producer_revision"],"producer_qd_sha256":history["producer_qd_sha256"],"semantics":history["fitness_semantics"]}
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
