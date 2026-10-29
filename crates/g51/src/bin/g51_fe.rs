//! `g51-fe`: the hand-zone voxel FE of G5.1, run on one design.
//!
//! ```text
//! g51-fe v6check <repo> [threads]
//!     The v6.0.0 hex8 cross-check reproduced with G5.1's FE: the v6 winner on its G5.0 grid at h 0.2, under
//!     `FeDefinition::v6_cross_check`, against evidence/g5/v6.0.0/fem/{grip_squeeze,guard_push,teeth}.json
//!     (relative 1e-6: G5.0 superposed six unit wrenches, G5.1 solves the one wrench). Exit 1 on a mismatch.
//! g51-fe run <genomes.csv> <design_id> <h> <definition.json> [g50]
//!     The three cases of one design on its G5.1 grid (or G5.0's, with `g50`) as JSON, with the seconds per solve.
//! ```
use jarambit_g5::g5_genome::G5Genome;
use jarambit_g5::g5_sdf::{build_blade_profile_2d, build_g5_sdf};
use jarambit_g51::fe::hand::{grip_squeeze_fe, guard_push_fe, teeth_fe, FeCase, FeDefinition, FeSolve};
use jarambit_g51::sdf::build_g51_sdf;
use serde_json::{json, Value};
use std::time::Instant;

fn fail(msg: impl std::fmt::Display) -> ! {
    eprintln!("g51-fe: {msg}");
    std::process::exit(1)
}

fn genome_of(csv: &str, id: &str) -> G5Genome {
    let text = std::fs::read_to_string(csv).unwrap_or_else(|e| fail(format!("{csv}: {e}")));
    let row = text
        .lines()
        .find_map(|l| l.strip_prefix(&format!("{id},")))
        .unwrap_or_else(|| fail(format!("{id} not in {csv}")));
    let g = G5Genome::from_csv_row(row).unwrap_or_else(|e| fail(format!("{id}: {e}")));
    if !id.ends_with(&g.sha256()[..8]) {
        fail(format!("{id}: the genome does not hash to its id"));
    }
    g
}

fn cases(g: &G5Genome, h: f64, def: &FeDefinition, g50: bool) -> (Value, [Result<Option<FeCase>, String>; 3]) {
    let p = g.decode();
    let blade = build_blade_profile_2d(&p);
    let t = Instant::now();
    let grid = if g50 { build_g5_sdf(g, h, true) } else { build_g51_sdf(g, h, true) };
    let grid_s = t.elapsed().as_secs_f64();
    let grip = grip_squeeze_fe(&p, &grid, def);
    let guard = guard_push_fe(&p, &grid, def).map(Some);
    let teeth = teeth_fe(&p, &blade, &grid, def);
    (json!({"grid_seconds": grid_s, "voxels": grid.nx * grid.ny * grid.nz}), [grip, guard, teeth])
}

fn case_json(r: &Result<Option<FeCase>, String>) -> Value {
    match r {
        Ok(Some(c)) => json!({"sf": c.sf, "seconds": c.seconds(), "solves": c.solves}),
        Ok(None) => json!({"sf": "inf", "note": "no such case on this design"}),
        Err(e) => json!({"sf": 0.0, "error": e}),
    }
}

fn v6check(repo: &str, threads: usize) {
    let g = genome_of(&format!("{repo}/evidence/g5/v6.0.0/winner_genome.csv"), "kv4i1404_g00581_29ea3c73");
    let def = FeDefinition::v6_cross_check(threads);
    let (grid, [grip, guard, teeth]) = cases(&g, 0.2, &def, true);
    let mut ok = true;
    let mut out = Vec::new();
    for (name, r) in [("grip_squeeze", &grip), ("guard_push", &guard), ("teeth", &teeth)] {
        let ev: Value = serde_json::from_str(
            &std::fs::read_to_string(format!("{repo}/evidence/g5/v6.0.0/fem/{name}.json")).unwrap_or_else(|e| fail(e)),
        )
        .unwrap_or_else(|e| fail(e));
        let want = ev["sf_fem"].as_f64().unwrap_or_else(|| fail(format!("{name}: no sf_fem")));
        let c = match r {
            Ok(Some(c)) => c,
            other => fail(format!("{name}: {other:?}")),
        };
        let rel = (c.sf / want - 1.0).abs();
        ok &= rel <= 1e-6;
        out.push(json!({"case": name, "sf_g51": c.sf, "sf_v6": want, "rel_diff": rel,
                        "seconds_g51": c.seconds(), "seconds_v6": ev["solver"]["seconds"], "solves": c.solves}));
    }
    println!("{}", serde_json::to_string_pretty(&json!({"grid": grid, "threads": threads, "cases": out, "match": ok})).unwrap());
    if !ok {
        fail("the v6.0.0 cross-check is not reproduced within 1e-6");
    }
}

/// The design `id` from the v6.0.0 winner or final pool (committed evidence).
fn evidence_genome(repo: &str, id: &str) -> G5Genome {
    for csv in ["winner_genome.csv", "final_pool/genomes_g5.csv"] {
        let path = format!("{repo}/evidence/g5/v6.0.0/{csv}");
        if std::fs::read_to_string(&path).is_ok_and(|t| t.lines().any(|l| l.starts_with(&format!("{id},")))) {
            return genome_of(&path, id);
        }
    }
    fail(format!("{id} is neither the v6.0.0 winner nor in its final pool"))
}

fn run_json(r: Result<FeSolve, String>, tags: Value) -> Value {
    let mut v = tags;
    match r {
        Ok(s) => v["solve"] = json!(s),
        Err(e) => v["error"] = json!(e),
    }
    v
}

/// The M3 FE study of one design (calibration protocol §1.1): every solve the rules read, as one JSON document.
fn study(repo: &str, id: &str, out: &str, threads: usize) {
    use jarambit_g51::fe::hand::{grip_station_fe, grip_stations, CANDIDATE_READ_VOLUMES_MM3};
    use jarambit_g51::sdf::build_g51_sdf_from_params_with_origin_shift;
    let t0 = Instant::now();
    let g = evidence_genome(repo, id);
    let p = g.decode();
    let blade = build_blade_profile_2d(&p);
    let base = FeDefinition {
        stress_volume_mm3: None,
        grip_all_stations: true,
        guard_clamp_mm: 4.0,
        teeth_per_edge: Some(3),
        teeth_h_mm: None,
        teeth_origin_shift_mm: 0.0,
        threads,
    };
    let clamps = [4.0, 8.0, 12.0];
    let stations = grip_stations(&p);
    let mut runs: Vec<Value> = Vec::new();
    let log = |m: &str| eprintln!("[{:>7.1} s] {id}: {m}", t0.elapsed().as_secs_f64());

    // h 0.2, no shift: every grip station, the three clamps, teeth on the grid and as local models
    let grid = build_g51_sdf_from_params_with_origin_shift(&p, 0.2, true, 0.0);
    log("grid h 0.2");
    let mut governing: Option<usize> = None;
    if let Some(st) = &stations {
        let mut best = f64::INFINITY;
        for (k, s) in st.iter().enumerate() {
            let r = grip_station_fe(&grid, &p, s, &base);
            if let Ok(f) = &r {
                if f.sf < best {
                    best = f.sf;
                    governing = Some(k);
                }
            }
            runs.push(run_json(r, json!({"case": "grip_squeeze", "h": 0.2, "shift_mm": 0.0, "station": s.s_frac})));
            log(&format!("grip station {}", s.s_frac));
        }
    }
    for c in clamps {
        let def = FeDefinition { guard_clamp_mm: c, ..base.clone() };
        let r = guard_push_fe(&p, &grid, &def).map(|c| c.solves[0].clone());
        runs.push(run_json(r, json!({"case": "guard_push", "h": 0.2, "shift_mm": 0.0, "clamp_mm": c})));
    }
    log("guard clamps");
    let teeth_run = |def: &FeDefinition, tags: Value| -> Value {
        let mut v = tags;
        match teeth_fe(&p, &blade, &grid, def) {
            Ok(Some(c)) => v["solves"] = json!(c.solves),
            Ok(None) => v["note"] = json!("no serrated edge"),
            Err(e) => v["error"] = json!(e),
        }
        v
    };
    runs.push(teeth_run(&base, json!({"case": "teeth", "h": 0.2, "teeth_h": "grid", "shift_mm": 0.0})));
    for ht in [0.1, 0.075, 0.05] {
        for frac in [0.0, 0.25, 0.5] {
            let def = FeDefinition { teeth_h_mm: Some(ht), teeth_origin_shift_mm: frac * ht, ..base.clone() };
            runs.push(teeth_run(&def, json!({"case": "teeth", "h": 0.2, "teeth_h": ht, "shift_mm": frac * ht})));
        }
        log(&format!("teeth h_t {ht}"));
    }
    drop(grid);

    // other voxel sizes and sub-voxel origin shifts: the governing grip station and the three clamps
    for (h, shift) in [(0.15, 0.0), (0.3, 0.0), (0.2, 0.05), (0.2, 0.1)] {
        let grid = build_g51_sdf_from_params_with_origin_shift(&p, h, true, shift);
        if let (Some(st), Some(k)) = (&stations, governing) {
            let r = grip_station_fe(&grid, &p, &st[k], &base);
            runs.push(run_json(r, json!({"case": "grip_squeeze", "h": h, "shift_mm": shift, "station": st[k].s_frac})));
        }
        for c in clamps {
            let def = FeDefinition { guard_clamp_mm: c, ..base.clone() };
            let r = guard_push_fe(&p, &grid, &def).map(|c| c.solves[0].clone());
            runs.push(run_json(r, json!({"case": "guard_push", "h": h, "shift_mm": shift, "clamp_mm": c})));
        }
        log(&format!("grid h {h} shift {shift}"));
    }
    let doc = json!({
        "design_id": id, "decoder": "G5.1", "decoder_sha256": jarambit_g51::hash::g51_decoder_sha256(),
        "threads": threads, "candidate_read_volumes_mm3": CANDIDATE_READ_VOLUMES_MM3,
        "grip_governing_station": governing.and_then(|k| stations.map(|s| s[k].s_frac)),
        "seconds": t0.elapsed().as_secs_f64(), "runs": runs,
    });
    std::fs::write(out, serde_json::to_string_pretty(&doc).unwrap() + "\n").unwrap_or_else(|e| fail(format!("{out}: {e}")));
    log("done");
}

/// The four study designs besides the v6.0.0 winner (protocol §1.1): one per quartile of the final pool's score,
/// drawn with seed 20261029 (`splitmix64(seed + quartile)` modulo the quartile's size, ids sorted by score then id).
fn study_designs(repo: &str) {
    let text = std::fs::read_to_string(format!("{repo}/evidence/g5/v6.0.0/final_pool/archive.jsonl")).unwrap_or_else(|e| fail(e));
    let mut rows: Vec<(f64, String)> = text
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let v: Value = serde_json::from_str(l).unwrap_or_else(|e| fail(e));
            (v["raw_score_100"].as_f64().unwrap_or_else(|| fail("raw_score_100")), v["design_id"].as_str().unwrap().to_string())
        })
        .collect();
    rows.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    let n = rows.len();
    let mut out = vec!["kv4i1404_g00581_29ea3c73".to_string()];
    for q in 0..4u64 {
        let (lo, hi) = (q as usize * n / 4, (q as usize + 1) * n / 4);
        let k = (jarambit_g5::g5_qd::splitmix64(20261029 + q) as usize) % (hi - lo);
        out.push(rows[lo + k].1.clone());
    }
    println!("{}", out.join("\n"));
}

/// One solve's SF under reading `r` (0: the 99.9 % quantile; 1..: `candidate_read_volumes_mm3[r - 1]`).
fn sf_under(solve: &Value, r: usize) -> Option<f64> {
    let allow = solve["allowable_mpa"].as_f64()?;
    let stress = if r == 0 { solve["reading"]["q999_mpa"].as_f64()? } else { solve["stress_by_volume_mpa"][r - 1].as_f64()? };
    Some(allow / stress)
}

/// The runs of a study matching `case`, `h` and `shift_mm` (and `key` = `val` when given).
fn runs_of<'a>(doc: &'a Value, case: &str, h: f64, shift: f64, key: Option<(&str, f64)>) -> Vec<&'a Value> {
    doc["runs"]
        .as_array()
        .map(|v| {
            v.iter()
                .filter(|r| {
                    r["case"] == case
                        && r["h"].as_f64() == Some(h)
                        && r["shift_mm"].as_f64().is_some_and(|s| (s - shift).abs() < 1e-12)
                        && key.is_none_or(|(k, x)| r[k].as_f64().is_some_and(|y| (y - x).abs() < 1e-12))
                })
                .collect()
        })
        .unwrap_or_default()
}

/// A single-solve run's SF (`None` when it failed or is absent).
fn one_sf(doc: &Value, case: &str, h: f64, shift: f64, key: Option<(&str, f64)>, r: usize) -> Option<f64> {
    runs_of(doc, case, h, shift, key).first().and_then(|run| sf_under(&run["solve"], r))
}

/// A teeth run's SF: the minimum over its solved teeth (`None` when it failed or has no teeth).
fn teeth_sf(doc: &Value, teeth_h: f64, shift: f64, r: usize) -> Option<f64> {
    let run = doc["runs"].as_array()?.iter().find(|x| {
        x["case"] == "teeth"
            && x["teeth_h"].as_f64().is_some_and(|t| (t - teeth_h).abs() < 1e-12)
            && x["shift_mm"].as_f64().is_some_and(|s| (s - shift).abs() < 1e-12)
    })?;
    run["solves"].as_array()?.iter().map(|s| sf_under(s, r)).collect::<Option<Vec<f64>>>()?.into_iter().reduce(f64::min)
}

fn rel(a: f64, b: f64) -> f64 {
    (a / b - 1.0).abs()
}

/// Protocol 1.1 applied to the five study documents: the guard clamp, the read volume, the tooth voxel size, and the
/// FE noise. A failed or missing solve counts against a candidate, never for it.
fn rules(paths: &[String]) {
    let docs: Vec<Value> = paths
        .iter()
        .map(|p| serde_json::from_str(&std::fs::read_to_string(p).unwrap_or_else(|e| fail(format!("{p}: {e}")))).unwrap_or_else(|e| fail(e)))
        .collect();
    let vols: Vec<f64> = serde_json::from_value(docs[0]["candidate_read_volumes_mm3"].clone()).unwrap_or_else(|e| fail(e));
    let readings = 1 + vols.len();
    let tol = 0.05;
    let mut report = serde_json::Map::new();

    // 1. guard clamp: c within 5 % of the next deeper clamp on every design, under every reading
    let clamps = [4.0, 8.0, 12.0];
    let mut c_g = None;
    let mut clamp_rows = Vec::new();
    for w in clamps.windows(2) {
        let mut worst = 0.0_f64;
        let mut ok = true;
        for d in &docs {
            for r in 0..readings {
                match (one_sf(d, "guard_push", 0.2, 0.0, Some(("clamp_mm", w[0])), r), one_sf(d, "guard_push", 0.2, 0.0, Some(("clamp_mm", w[1])), r)) {
                    (Some(a), Some(b)) => worst = worst.max(rel(a, b)),
                    _ => ok = false,
                }
            }
        }
        ok &= worst <= tol;
        clamp_rows.push(json!({"clamp_mm": w[0], "next_mm": w[1], "worst_rel_diff": worst, "ok": ok}));
        if ok && c_g.is_none() {
            c_g = Some(w[0]);
        }
    }
    let c_g = c_g.unwrap_or(12.0);
    report.insert("guard_clamp_mm".into(), json!({"value": c_g, "candidates": clamp_rows}));

    // 2. read volume: grip (governing station) and guard (at c_g) within 5 % between h 0.2 and h 0.15 on every design
    let mut v_rows = Vec::new();
    let mut v_pick = None;
    for (i, &v) in vols.iter().enumerate() {
        let r = i + 1;
        let mut worst = 0.0_f64;
        let mut ok = true;
        for d in &docs {
            if let Some(st) = d["grip_governing_station"].as_f64() {
                match (one_sf(d, "grip_squeeze", 0.2, 0.0, Some(("station", st)), r), one_sf(d, "grip_squeeze", 0.15, 0.0, Some(("station", st)), r)) {
                    (Some(a), Some(b)) => worst = worst.max(rel(a, b)),
                    _ => ok = false,
                }
            }
            match (one_sf(d, "guard_push", 0.2, 0.0, Some(("clamp_mm", c_g)), r), one_sf(d, "guard_push", 0.15, 0.0, Some(("clamp_mm", c_g)), r)) {
                (Some(a), Some(b)) => worst = worst.max(rel(a, b)),
                _ => ok = false,
            }
        }
        ok &= worst <= tol;
        v_rows.push(json!({"volume_mm3": v, "worst_rel_diff_h020_h015": worst, "ok": ok}));
        if ok && v_pick.is_none() {
            v_pick = Some((r, v));
        }
    }
    let (r_v, v) = v_pick.unwrap_or((vols.len(), *vols.last().unwrap()));
    report.insert("read_volume_mm3".into(), json!({"value": v, "candidates": v_rows}));

    // 3. tooth voxel size: the coarsest h_t within 5 % of the next finer on every serrated design, under V
    let hts = [0.1, 0.075, 0.05];
    let mut t_rows = Vec::new();
    let mut h_t = None;
    for w in hts.windows(2) {
        let mut worst = 0.0_f64;
        let mut ok = true;
        for d in &docs {
            let serrated = d["runs"].as_array().is_some_and(|v| v.iter().any(|x| x["case"] == "teeth" && x.get("note").is_none()));
            if !serrated {
                continue;
            }
            match (teeth_sf(d, w[0], 0.0, r_v), teeth_sf(d, w[1], 0.0, r_v)) {
                (Some(a), Some(b)) => worst = worst.max(rel(a, b)),
                _ => ok = false,
            }
        }
        ok &= worst <= tol;
        t_rows.push(json!({"teeth_h_mm": w[0], "next_mm": w[1], "worst_rel_diff": worst, "ok": ok}));
        if ok && h_t.is_none() {
            h_t = Some(w[0]);
        }
    }
    let h_t = h_t.unwrap_or(0.05);
    report.insert("teeth_h_mm".into(), json!({"value": h_t, "candidates": t_rows}));

    // 4. FE noise: standard deviation of log SF over sub-voxel origin shifts, per case, under the chosen definition
    let sd = |xs: &[f64]| -> f64 {
        let l: Vec<f64> = xs.iter().map(|x| x.ln()).collect();
        let m = l.iter().sum::<f64>() / l.len() as f64;
        (l.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (l.len() as f64 - 1.0)).sqrt()
    };
    let mut noise = Vec::new();
    for d in &docs {
        let id = d["design_id"].as_str().unwrap_or("?");
        let grip: Option<Vec<f64>> = d["grip_governing_station"].as_f64().and_then(|st| {
            [0.0, 0.05, 0.1].iter().map(|&s| one_sf(d, "grip_squeeze", 0.2, s, Some(("station", st)), r_v)).collect()
        });
        let guard: Option<Vec<f64>> = [0.0, 0.05, 0.1].iter().map(|&s| one_sf(d, "guard_push", 0.2, s, Some(("clamp_mm", c_g)), r_v)).collect();
        let teeth: Option<Vec<f64>> = [0.0, 0.25, 0.5].iter().map(|&f| teeth_sf(d, h_t, f * h_t, r_v)).collect();
        noise.push(json!({"design_id": id, "grip_sd_log": grip.map(|v| sd(&v)), "guard_sd_log": guard.map(|v| sd(&v)),
                          "teeth_sd_log": teeth.map(|v| sd(&v))}));
    }
    report.insert("fe_noise".into(), json!(noise));

    // reported beside the rules: each design's SFs under the chosen definition, h 0.3 / 0.2 / 0.15, teeth on the grid
    let mut designs = Vec::new();
    for d in &docs {
        let st = d["grip_governing_station"].as_f64();
        let grip_stations: Vec<Value> = runs_of(d, "grip_squeeze", 0.2, 0.0, None)
            .iter()
            .map(|r| json!({"station": r["station"], "sf": sf_under(&r["solve"], r_v), "error": r.get("error")}))
            .collect();
        let at = |case: &str, h: f64, key: Option<(&str, f64)>| one_sf(d, case, h, 0.0, key, r_v);
        designs.push(json!({
            "design_id": d["design_id"], "seconds": d["seconds"],
            "grip_stations_h020": grip_stations,
            "grip_governing_h": st.map(|s| json!({"0.3": at("grip_squeeze", 0.3, Some(("station", s))), "0.2": at("grip_squeeze", 0.2, Some(("station", s))), "0.15": at("grip_squeeze", 0.15, Some(("station", s)))})),
            "guard_h": json!({"0.3": at("guard_push", 0.3, Some(("clamp_mm", c_g))), "0.2": at("guard_push", 0.2, Some(("clamp_mm", c_g))), "0.15": at("guard_push", 0.15, Some(("clamp_mm", c_g)))}),
            "guard_clamps_h020": clamps.iter().map(|&c| at("guard_push", 0.2, Some(("clamp_mm", c)))).collect::<Vec<_>>(),
            "teeth": json!({"grid_h020": d["runs"].as_array().and_then(|v| v.iter().find(|x| x["case"] == "teeth" && x["teeth_h"] == "grid")).and_then(|x| x["solves"].as_array()).map(|s| s.iter().filter_map(|t| sf_under(t, r_v)).fold(f64::INFINITY, f64::min)),
                            "0.1": teeth_sf(d, 0.1, 0.0, r_v), "0.075": teeth_sf(d, 0.075, 0.0, r_v), "0.05": teeth_sf(d, 0.05, 0.0, r_v)}),
            "errors": d["runs"].as_array().map(|v| v.iter().filter_map(|x| x.get("error")).cloned().collect::<Vec<_>>()),
        }));
    }
    report.insert("designs".into(), json!(designs));
    report.insert("tolerance".into(), json!(tol));
    println!("{}", serde_json::to_string_pretty(&Value::Object(report)).unwrap());
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("rules") if a.len() >= 3 => rules(&a[2..]),
        Some("study") if a.len() >= 5 => study(&a[2], &a[3], &a[4], a.get(5).and_then(|v| v.parse().ok()).unwrap_or(2)),
        Some("study-designs") if a.len() >= 3 => study_designs(&a[2]),
        Some("v6check") if a.len() >= 3 => v6check(&a[2], a.get(3).and_then(|v| v.parse().ok()).unwrap_or(4)),
        Some("run") if a.len() >= 6 => {
            let g = genome_of(&a[2], &a[3]);
            let h: f64 = a[4].parse().unwrap_or_else(|e| fail(e));
            let def: FeDefinition = serde_json::from_str(&std::fs::read_to_string(&a[5]).unwrap_or_else(|e| fail(e)))
                .unwrap_or_else(|e| fail(e));
            let g50 = a.get(6).map(String::as_str) == Some("g50");
            let t = Instant::now();
            let (grid, [grip, guard, teeth]) = cases(&g, h, &def, g50);
            println!(
                "{}",
                json!({"design_id": a[3], "h": h, "decoder": if g50 { "G5.0" } else { "G5.1" }, "definition": def,
                       "grid": grid, "grip_squeeze": case_json(&grip), "guard_push": case_json(&guard),
                       "teeth": case_json(&teeth), "seconds": t.elapsed().as_secs_f64()})
            );
        }
        _ => fail("usage: g51-fe v6check <repo> [threads] | run <genomes.csv> <design_id> <h> <definition.json> [g50]"),
    }
}
