//! Rust-vs-Python parity for the canonical `W_ULTRALIGHT_v1` score.
//!
//! Stage-2 ranks by `compute_canonical_ultralight_weighted_fitness` (Rust), which mirrors
//! `ops/ci/ultralight_weighted_fitness.py::compute_ultralight_weighted_fitness` (Python).
//! This test scores every committed genome-backed scorecard / RS-v2 receipt pair, the
//! xbabe2 i81-i86 refill scorecards, and fallback-branch variants with both implementations
//! and requires identical results, so neither copy of the formula can drift silently.

use super::{
    canonical_ultralight_scores_match, compute_canonical_ultralight_weighted_fitness,
    stage2_score_from_scorecard,
};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_json(p: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(p).unwrap_or_else(|e| panic!("{}: {e}", p.display())))
        .unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}

#[test]
fn adjacent_six_decimal_scores_allow_binary_rounding_noise() {
    let lower = 70.000002_f64;
    let adjacent = 70.000003_f64;
    assert_eq!(format!("{lower:.6}"), "70.000002");
    assert_eq!(format!("{adjacent:.6}"), "70.000003");
    assert!((adjacent - lower) > 1e-6);
    assert!(canonical_ultralight_scores_match(lower, adjacent));
    assert!(!canonical_ultralight_scores_match(lower, 70.000004_f64));
}

/// Every committed `<label>_master_scorecard.json` / `<label>_cad_eval.json` under `evidence/`
/// that has a sibling `<label>_rs.json`, sorted for a stable order.
fn committed_pairs(root: &Path) -> Vec<(PathBuf, PathBuf)> {
    let mut out = Vec::new();
    let mut stack = vec![root.join("evidence")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap().flatten() {
            let p = entry.path();
            if p.is_dir() {
                stack.push(p);
                continue;
            }
            let name = p.file_name().unwrap().to_string_lossy().into_owned();
            for suffix in ["_master_scorecard.json", "_cad_eval.json"] {
                if let Some(label) = name.strip_suffix(suffix) {
                    let rs = p.with_file_name(format!("{label}_rs.json"));
                    if rs.is_file() {
                        out.push((p.clone(), rs));
                    }
                }
            }
        }
    }
    out.sort();
    out
}

/// Fallback-branch variants: the receipt without export measurements (placeholder
/// geometry chosen by RS-v2 gate status), with each relevant gate forced pass/fail,
/// without human-max data, and with optional engineering metrics removed.
fn variants(sc: &Value, rs: &Value) -> Vec<(String, Value, Value)> {
    let mut v = Vec::new();
    for status in ["pass", "fail"] {
        let mut sc2 = sc.clone();
        let mut rs2 = rs.clone();
        if let Some(o) = sc2.as_object_mut() {
            o.remove("export_measurements");
            o.remove("rs_v2_gates");
        }
        if let Some(o) = rs2.as_object_mut() {
            o.remove("export_measurements");
            o.remove("human_max");
            let gates = o.entry("gates").or_insert_with(|| json!({}));
            for g in ["human_max", "gate_s_sharp_law", "gate_s_tip_point_brep"] {
                gates[g] = json!(status);
            }
        }
        v.push((format!("placeholders_gates_{status}"), sc2, rs2));
    }
    let mut sc3 = sc.clone();
    if let Some(em) = sc3.get_mut("engineering_metrics").and_then(Value::as_object_mut) {
        for k in [
            "mass_3d_g",
            "twist_body_sf",
            "solid_tip_dolfinx_rel_err",
            "ring_bearing_w_mm",
            "buckling_eigenvalue_a4",
        ] {
            em.remove(k);
        }
    }
    v.push(("metric_defaults".into(), sc3, rs.clone()));
    v
}

#[test]
fn canonical_ultralight_rust_matches_python_on_committed_and_refill_scorecards() {
    let root = root();
    let pairs = committed_pairs(&root);
    let n_master = pairs
        .iter()
        .filter(|(p, _)| p.to_string_lossy().ends_with("_master_scorecard.json"))
        .count();
    // Guard against a vacuous pass if evidence moves: Stage-2 sees master scorecards.
    assert!(n_master >= 10, "only {n_master} committed master_scorecard/_rs pairs found");
    assert!(pairs.len() > n_master, "no committed cad_eval/_rs pairs found");

    let tmp = std::env::temp_dir().join(format!("jarambit_ul_parity_{}", std::process::id()));
    std::fs::create_dir_all(&tmp).unwrap();
    let mut cases: Vec<(String, f64, PathBuf, PathBuf)> = Vec::new();
    for (i, (sc_path, rs_path)) in pairs.iter().enumerate() {
        let (sc, rs) = (read_json(sc_path), read_json(rs_path));
        let rel = sc_path.strip_prefix(&root).unwrap().display().to_string();
        cases.push((
            rel.clone(),
            compute_canonical_ultralight_weighted_fitness(&sc, &rs),
            sc_path.clone(),
            rs_path.clone(),
        ));
        for (tag, sc_v, rs_v) in variants(&sc, &rs) {
            let sp = tmp.join(format!("{i}_{tag}_sc.json"));
            let rp = tmp.join(format!("{i}_{tag}_rs.json"));
            std::fs::write(&sp, serde_json::to_vec(&sc_v).unwrap()).unwrap();
            std::fs::write(&rp, serde_json::to_vec(&rs_v).unwrap()).unwrap();
            let score = compute_canonical_ultralight_weighted_fitness(&sc_v, &rs_v);
            cases.push((format!("{rel} [{tag}]"), score, sp, rp));
        }
    }

    let refill_fixture = read_json(&root.join(
        "crates/cli/tests/fixtures/wultralight_i81_i86_scorecards.json",
    ));
    assert_eq!(
        refill_fixture["schema"].as_str(),
        Some("jarambit.wultralight_i81_i86_parity_fixture.v1")
    );
    assert_eq!(
        refill_fixture["source_commit"].as_str(),
        Some("009f5ac520fa3afe8ed8025230eef604dbb7f922")
    );
    let refill_cases = refill_fixture["cases"].as_array().unwrap();
    assert_eq!(refill_cases.len(), 6, "expected i81-i86 scorecards");
    let refill_start = cases.len();
    let mut refill_labels = Vec::new();
    let mut refill_expected = Vec::new();
    for (i, row) in refill_cases.iter().enumerate() {
        let label = row["label"].as_str().expect("fixture label").to_owned();
        for key in ["source_scorecard_sha256", "source_rs_sha256"] {
            let digest = row[key].as_str().expect("source digest");
            assert_eq!(digest.len(), 64, "{label}: invalid {key}");
            assert!(digest.bytes().all(|b| b.is_ascii_hexdigit()), "{label}: invalid {key}");
        }
        let sc = row["scorecard"].clone();
        let rs = row["rs"].clone();
        let rust_score = compute_canonical_ultralight_weighted_fitness(&sc, &rs);
        let expected = row["expected_score"].as_f64().expect("fixture score");
        assert!(
            (rust_score - expected).abs() <= 1e-9,
            "{label}: Rust scorecard fixture={rust_score}, expected Python snapshot={expected}"
        );
        let mut stored_match = sc.clone();
        stored_match["ultralight_weighted_fitness"] =
            json!({"global_weighted_fitness": expected});
        assert_eq!(stage2_score_from_scorecard(&stored_match, &rs), Ok(expected));
        stored_match["ultralight_weighted_fitness"]["global_weighted_fitness"] =
            json!(expected + 4.0);
        assert!(
            stage2_score_from_scorecard(&stored_match, &rs).is_err(),
            "{label}: mismatched stored W_ULTRALIGHT_v1 must be rejected"
        );
        let sp = tmp.join(format!("refill_{i}_sc.json"));
        let rp = tmp.join(format!("refill_{i}_rs.json"));
        std::fs::write(&sp, serde_json::to_vec(&sc).unwrap()).unwrap();
        std::fs::write(&rp, serde_json::to_vec(&rs).unwrap()).unwrap();
        cases.push((label.clone(), rust_score, sp, rp));
        refill_labels.push(label);
        refill_expected.push(expected);
    }
    assert_eq!(
        refill_labels.iter().map(String::as_str).collect::<Vec<_>>(),
        [
            "ef2i81_r01_p10_c44",
            "ef2i82_r01_p14_c90",
            "ef2i83_r01_p10_c70",
            "ef2i84_r01_p14_c49",
            "ef2i85_r01_p10_c92",
            "ef2i86_r01_p14_c50",
        ]
    );

    let list = tmp.join("cases.json");
    let paths: Vec<[String; 2]> = cases
        .iter()
        .map(|(_, _, s, r)| [s.display().to_string(), r.display().to_string()])
        .collect();
    std::fs::write(&list, serde_json::to_vec(&paths).unwrap()).unwrap();
    let out = Command::new("python3")
        .current_dir(&root)
        .args([
            "-B",
            "-c",
            "import json,sys\n\
             sys.path.insert(0,'.')\n\
             from ops.ci.ultralight_weighted_fitness import compute_ultralight_weighted_fitness as f\n\
             cases=json.load(open(sys.argv[1]))\n\
             print(json.dumps([f(json.load(open(s)),json.load(open(r)))['global_weighted_fitness'] for s,r in cases]))",
        ])
        .arg(&list)
        .output()
        .expect("python3 is required for the W_ULTRALIGHT_v1 parity test");
    let _ = std::fs::remove_dir_all(&tmp);
    assert!(
        out.status.success(),
        "python scorer failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let py: Vec<f64> = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(py.len(), cases.len());
    for (offset, ((label, _, _, _), expected)) in cases[refill_start..]
        .iter()
        .zip(&refill_expected)
        .enumerate()
    {
        let index = refill_start + offset;
        assert!(
            (py[index] - expected).abs() <= 1e-9,
            "{label}: Python scorecard fixture={}, expected Python snapshot={expected}",
            py[index]
        );
    }

    let mismatches: Vec<String> = cases
        .iter()
        .zip(&py)
        .filter(|((_, rust, _, _), &p)| !rust.is_finite() || (rust - p).abs() > 1e-9)
        .map(|((name, rust, _, _), p)| format!("{name}: rust={rust} python={p}"))
        .collect();
    assert!(
        mismatches.is_empty(),
        "W_ULTRALIGHT_v1 Rust/Python drift on {} of {} cases:\n{}",
        mismatches.len(),
        cases.len(),
        mismatches.join("\n")
    );
    // The scores must actually vary, or the comparison proves little.
    let distinct: std::collections::BTreeSet<u64> = py.iter().map(|x| x.to_bits()).collect();
    assert!(distinct.len() >= 10, "only {} distinct scores", distinct.len());
}
