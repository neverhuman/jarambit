//! `jarambit smoothing-ladder`: smooth one evolved design's exported mesh at a ladder of
//! strengths and re-score every level with the canonical `W_ULTRALIGHT_v1`.
//!
//! Level 0 is the design as exported, scored from its stored scorecard. Every other level runs
//! the geometry-smoothing benchmark (Taubin lambda/mu, normal-only, feature pins) on the exported
//! STL, then re-measures the three objectives that read the exported 3D geometry:
//! - 3D mass: exporter volume plus the smoothed-minus-input STL volume, times 4.43 g/cm^3;
//! - solid tip: `fem_solid/tip_solid.py` on the D1 record with the smoothed upper skin, then the
//!   evaluator's own DOLFINx cross-check on the same saved mesh;
//! - CAD/B-rep: withheld (a smoothed triangle mesh has no validated STEP solid).
//!
//! All other objectives are genome-derived and carry over. `score_with_brep` repeats the score
//! with the B-rep objective left as exported, so the smoothing effect alone is visible.
//! Nothing here is a physical test, a certification or a qualification. Writes a NEW directory.

use crate::auto_evolve::compute_canonical_ultralight_weighted_fitness;
use jarambit_evidence::identity::ContentRef;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;

const USAGE: &str = "usage: jarambit smoothing-ladder --winner DIR --label LABEL --smoother BIN --solver-python PY \
--out NEW_DIR [--levels PIN:ITERS,...] [--jobs N] [--code-commit SHA]\n\
run from the repository root; DIR holds the design's retained Stage-2 files (<label>.stl, _D1.json,\n\
_master_scorecard.json, _rs.json, _export_stdout.txt, _genome.csv, _grind_law.json)";

/// Ten levels: as exported, five iteration counts with 30-degree feature pins (20 is the paper's
/// R1 setting), then four with 60-degree pins, the weakest pin angle the smoother completes on
/// these knife edges without reversing a face.
pub const DEFAULT_LEVELS: &str = "0:0,30:2,30:5,30:10,30:20,30:60,60:20,60:60,60:120,60:200";
const DENSITY_G_PER_MM3: f64 = 0.00443;

type Res<T> = Result<T, String>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Level {
    pub pin_deg: u32,
    pub iterations: u32,
}

pub fn parse_levels(spec: &str) -> Res<Vec<Level>> {
    let levels = spec
        .split(',')
        .map(|item| {
            let (pin, iters) = item.split_once(':').ok_or_else(|| format!("level {item:?} is not PIN:ITERS"))?;
            let level = Level {
                pin_deg: pin.trim().parse().map_err(|_| format!("bad pin angle in {item:?}"))?,
                iterations: iters.trim().parse().map_err(|_| format!("bad iteration count in {item:?}"))?,
            };
            if (level.iterations == 0) != (level.pin_deg == 0) {
                return Err(format!("{item:?}: only the as-exported level may have 0 iterations or 0 pin angle"));
            }
            Ok(level)
        })
        .collect::<Res<Vec<_>>>()?;
    if levels.first() != Some(&Level { pin_deg: 0, iterations: 0 }) || levels[1..].iter().any(|l| l.iterations == 0) {
        return Err("the ladder must start with exactly one as-exported level 0:0".into());
    }
    Ok(levels)
}

fn sha(bytes: &[u8]) -> String {
    ContentRef::from_bytes("smoothing-ladder", "v1", bytes).sha256
}

fn read(path: &Path) -> Res<Vec<u8>> {
    fs::read(path).map_err(|e| format!("{}: {e}", path.display()))
}

fn read_json(path: &Path) -> Res<Value> {
    serde_json::from_slice(&read(path)?).map_err(|e| format!("{}: {e}", path.display()))
}

fn write(path: &Path, bytes: &[u8]) -> Res<()> {
    fs::write(path, bytes).map_err(|e| format!("{}: {e}", path.display()))
}

fn num(doc: &Value, ptr: &str) -> Res<f64> {
    doc.pointer(ptr).and_then(Value::as_f64).ok_or_else(|| format!("missing number {ptr}"))
}

/// Rounds like the exporter's `%.4f` mass and the receipts' fixed decimals.
fn fixed(x: f64, decimals: usize) -> f64 {
    format!("{x:.decimals$}").parse().unwrap_or(x)
}

/// Exporter-reported enclosed volume (`volume <mm3> mm3` in the export log).
pub fn exporter_volume(stdout: &str) -> Res<f64> {
    stdout
        .split("volume ")
        .skip(1)
        .filter_map(|rest| rest.split_whitespace().next()?.parse().ok())
        .last()
        .ok_or_else(|| "export log has no 'volume <mm3> mm3' line".into())
}

/// Smoothing-input position (rounded to 1e-4 mm) to smoothed position, from displacement.csv.
fn displacement_map(csv: &str) -> Res<HashMap<[i64; 3], [f64; 3]>> {
    let mut lines = csv.lines();
    let header: Vec<&str> = lines.next().ok_or("empty displacement.csv")?.split(',').collect();
    let col = |name: &str| header.iter().position(|h| *h == name).ok_or_else(|| format!("displacement.csv lacks {name}"));
    let before = [col("x_before_mm")?, col("y_before_mm")?, col("z_before_mm")?];
    let after = [col("x_after_mm")?, col("y_after_mm")?, col("z_after_mm")?];
    let mut map = HashMap::new();
    for line in lines.filter(|l| !l.is_empty()) {
        let cells: Vec<&str> = line.split(',').collect();
        let get = |i: usize| -> Res<f64> { cells.get(i).and_then(|c| c.parse().ok()).ok_or_else(|| format!("bad row {line:?}")) };
        let b = [get(before[0])?, get(before[1])?, get(before[2])?];
        if map.insert(key(b), [get(after[0])?, get(after[1])?, get(after[2])?]).is_some() {
            return Err(format!("duplicate smoothing-input vertex {b:?}"));
        }
    }
    Ok(map)
}

fn key(p: [f64; 3]) -> [i64; 3] {
    p.map(|c| (c * 1e4).round() as i64)
}

/// The D1 record with `export_surface.verts` replaced by the smoothed positions. Every surface
/// vertex must match exactly one smoothing-input vertex; anything else fails closed.
pub fn smoothed_d1(d1: &Value, csv: &str) -> Res<(Value, usize)> {
    let map = displacement_map(csv)?;
    let verts = d1.pointer("/export_surface/verts").and_then(Value::as_array).ok_or("D1 lacks export_surface.verts")?;
    let mut seen = std::collections::HashSet::new();
    let moved = verts
        .iter()
        .map(|v| {
            let p: Vec<f64> = v.as_array().map(|a| a.iter().filter_map(Value::as_f64).collect()).unwrap_or_else(Vec::new);
            let p: [f64; 3] = p.try_into().map_err(|_| "surface vertex is not three numbers".to_string())?;
            let k = key(p);
            if !seen.insert(k) {
                return Err(format!("duplicate surface vertex key {p:?}"));
            }
            map.get(&k).map(|q| json!(q)).ok_or_else(|| format!("surface vertex {p:?} has no smoothing-input match"))
        })
        .collect::<Res<Vec<_>>>()?;
    let mut doc = d1.clone();
    doc["export_surface"]["verts"] = Value::Array(moved);
    Ok((doc, verts.len()))
}

/// Smoothed scorecard: the as-exported one with only the geometry-reading measurements replaced.
pub fn smoothed_scorecard(card: &Value, mass_g: f64, skfem: &Value, dolfinx: &Value, level: usize, withhold_brep: bool) -> Res<Value> {
    let solid = skfem.get("solid").unwrap_or(skfem);
    let (sk_max, sk_avg) = (num(solid, "/SF_solid_nodal_max")?, num(solid, "/SF_solid_nodal_avg")?);
    let (dx_max, dx_avg) = (num(dolfinx, "/SF_nodal_max")?, num(dolfinx, "/SF_nodal_avg")?);
    let err = [((sk_max - dx_max) / dx_max).abs(), ((sk_avg - dx_avg) / dx_avg).abs()];
    let mut out = card.clone();
    let em = out.get_mut("engineering_metrics").and_then(Value::as_object_mut).ok_or("scorecard lacks engineering_metrics")?;
    em.insert("mass_3d_g".into(), json!(mass_g));
    em.insert("solid_tip_sf_nodal_max".into(), json!(dx_max));
    em.insert("solid_tip_sf_nodal_avg".into(), json!(dx_avg));
    em.insert("solid_tip_dolfinx_rel_err".into(), json!((err[0] * 1e6).round() / 1e6));
    let gates = out.pointer_mut("/weighted_cad_score/gates").and_then(Value::as_object_mut).ok_or("scorecard lacks weighted_cad_score.gates")?;
    gates.insert("solid_tip_3d_p2_1p82nm".into(), json!(dx_max >= 1.0));
    gates.insert("dolfinx_cross_check_2pct".into(), json!(err.iter().all(|e| *e <= 0.02)));
    if withhold_brep {
        gates.insert("cad_watertight_and_single_solid".into(), json!(false));
        out["cad"]["cad_valid"] = json!(false);
    }
    let subject = out.get("subject").and_then(Value::as_str).unwrap_or("design").to_string();
    out["subject"] = json!(format!("{subject}_smoothed_L{level}"));
    Ok(out)
}

fn run(cmd: &mut Command, log: &Path) -> Res<()> {
    let file = fs::File::create(log).map_err(|e| format!("{}: {e}", log.display()))?;
    let err = file.try_clone().map_err(|e| e.to_string())?;
    let status = cmd.stdout(Stdio::from(file)).stderr(Stdio::from(err)).status().map_err(|e| format!("{cmd:?}: {e}"))?;
    if status.success() { Ok(()) } else { Err(format!("{cmd:?} exited {status}; see {}", log.display())) }
}

struct Ctx {
    root: PathBuf,
    winner: PathBuf,
    label: String,
    smoother: PathBuf,
    python: PathBuf,
    out: PathBuf,
    card: Value,
    rs: Value,
    d1: Value,
    volume0: f64,
}

fn file(ctx: &Ctx, suffix: &str) -> PathBuf {
    ctx.winner.join(format!("{}{suffix}", ctx.label))
}

/// Smooth, re-solve and re-score one level above 0.
fn smoothed_level(ctx: &Ctx, index: usize, level: Level) -> Res<Value> {
    let dir = ctx.out.join(format!("L{index}"));
    let smooth = dir.join("smoothing");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    run(
        Command::new(&ctx.smoother)
            .args(["--input".as_ref(), file(ctx, ".stl").as_os_str(), "--out".as_ref(), smooth.as_os_str()])
            .args(["--label", &format!("{} smoothing ladder L{index}, not qualified", ctx.label)])
            .args(["--lambda", "0.5", "--mu", "-0.5", "--max-displacement-mm", "2", "--normal-only"])
            .args(["--pin-feature-angle", &level.pin_deg.to_string(), "--iterations", &level.iterations.to_string()]),
        &dir.join("smoothing.log"),
    )?;
    let metrics = read_json(&smooth.join("metrics.json"))?;
    if metrics["status"] != "pass" || num(&metrics, "/settings/iterations_completed")? as u32 != level.iterations {
        return Err(format!("L{index}: smoothing did not complete: {}", metrics["reasons"]));
    }
    let csv = String::from_utf8(read(&smooth.join("displacement.csv"))?).map_err(|e| e.to_string())?;
    let (d1, matched) = smoothed_d1(&ctx.d1, &csv)?;
    let case = dir.join("case");
    fs::create_dir_all(&case).map_err(|e| e.to_string())?;
    let l = &ctx.label;
    for suffix in ["_genome.csv", "_grind_law.json"] {
        fs::copy(file(ctx, suffix), case.join(format!("{l}{suffix}"))).map_err(|e| e.to_string())?;
    }
    let d1_path = case.join(format!("{l}_D1.json"));
    write(&d1_path, serde_json::to_string(&d1).map_err(|e| e.to_string())?.as_bytes())?;
    let skfem_path = case.join(format!("{l}_skfem_raw.json"));
    run(
        Command::new(&ctx.python)
            .current_dir(&ctx.root)
            .env("PYTHONPATH", &ctx.root)
            .env("DUMP_MESH", case.join(format!("{l}_solid_mesh.npz")))
            .envs([("OMP_NUM_THREADS", "1"), ("OPENBLAS_NUM_THREADS", "1"), ("MKL_NUM_THREADS", "1")])
            .arg("fem_solid/tip_solid.py")
            .arg(&d1_path)
            .args(["30", "4", "1"])
            .arg(&skfem_path),
        &case.join(format!("{l}_skfem_stdout.txt")),
    )?;
    // The evaluator's own runtime probe and DOLFINx cross-check, unchanged, on the saved mesh.
    let probe = "import json,sys;from pathlib import Path;sys.path.insert(0,sys.argv[1])\n\
from ops.ci.evaluate_cad_candidate import independent_runtime,run_independent_cross_check\n\
d=Path(sys.argv[2]);rt=independent_runtime();(d/(sys.argv[3]+'_runtime.json')).write_text(json.dumps(rt,indent=2,sort_keys=True))\n\
print(json.dumps(run_independent_cross_check(d/(sys.argv[3]+'_solid_mesh.npz'),d,sys.argv[3],rt)))";
    let dx_path = case.join(format!("{l}_dolfinx_result.json"));
    let output = Command::new(&ctx.python)
        .current_dir(&ctx.root)
        .args(["-I", "-c", probe])
        .arg(&ctx.root)
        .arg(&case)
        .arg(l)
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(format!("L{index}: DOLFINx cross-check failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    write(&dx_path, &output.stdout)?;
    let skfem = read_json(&skfem_path)?;
    let dolfinx: Value = serde_json::from_slice(&output.stdout).map_err(|e| format!("L{index}: cross-check output: {e}"))?;
    if skfem.pointer("/solver/converged") != Some(&json!(true)) {
        return Err(format!("L{index}: skfem tip solve did not converge"));
    }
    let (vb, va) = (num(&metrics, "/result/volume_mm3/before")?, num(&metrics, "/result/volume_mm3/after")?);
    let mass = fixed((ctx.volume0 + (va - vb)) * DENSITY_G_PER_MM3, 4);
    let card = smoothed_scorecard(&ctx.card, mass, &skfem, &dolfinx, index, true)?;
    let card_brep = smoothed_scorecard(&ctx.card, mass, &skfem, &dolfinx, index, false)?;
    write(&dir.join(format!("{l}_L{index}_scorecard.json")), (serde_json::to_string_pretty(&card).map_err(|e| e.to_string())? + "\n").as_bytes())?;
    let solid = skfem.get("solid").unwrap_or(&skfem);
    let stl = read(&smooth.join("smoothed.stl"))?;
    Ok(json!({
        "smoothing": {
            "dir": smooth, "smoothed_stl": smooth.join("smoothed.stl"), "smoothed_stl_sha256": sha(&stl),
            "displacement_csv_sha256": sha(csv.as_bytes()),
            "max_displacement_mm": metrics["result"]["max_displacement_mm"], "rms_displacement_mm": metrics["result"]["rms_displacement_mm"],
            "moved_over_0_01_mm": metrics["result"]["moved_over_0_01_mm"], "volume_mm3": metrics["result"]["volume_mm3"],
            "pinned_vertices": metrics["pins"]["feature_pinned_vertices"], "iterations_completed": metrics["settings"]["iterations_completed"],
        },
        "surface_vertices_matched": matched,
        "case_dir": case,
        "mass_3d_g": mass,
        "solid_tip": {
            "skfem": {"SF_nodal_max": solid["SF_solid_nodal_max"], "SF_nodal_avg": solid["SF_solid_nodal_avg"], "n_tet": solid["n_tet"], "n_dof": solid["n_dof"],
                      "iterations": skfem["solver"]["iterations"], "relative_residual": skfem["solver"]["relative_residual"]},
            "dolfinx": {"SF_nodal_max": dolfinx["SF_nodal_max"], "SF_nodal_avg": dolfinx["SF_nodal_avg"], "ksp_its": dolfinx["ksp_its"]},
            "rel_err_nodal_max": card["engineering_metrics"]["solid_tip_dolfinx_rel_err"],
        },
        "score": fixed(compute_canonical_ultralight_weighted_fitness(&card, &ctx.rs), 6),
        "score_with_brep": fixed(compute_canonical_ultralight_weighted_fitness(&card_brep, &ctx.rs), 6),
    }))
}

/// LaTeX rows for the ladder table: level, pins, iterations, displacement, volume, mass, tip SF, F.
pub fn table_tex(levels: &[Value]) -> String {
    let mut s = String::from("% Generated by `jarambit smoothing-ladder`; do not edit.\n");
    for lv in levels {
        let f = |p: &str| lv.pointer(p).and_then(Value::as_f64);
        let opt = |v: Option<f64>, d: usize| v.map_or("--".to_string(), |x| format!("{x:.d$}"));
        let pins = if lv["iterations"] == 0 { "--".to_string() } else { format!("{}$^\\circ$", lv["pin_deg"]) };
        s += &format!(
            "L{} & {} & {} & {} & {} & {} & {:.4} & {:.4} & {:.6} & {:.6} \\\\\n",
            lv["level"], pins, lv["iterations"],
            opt(f("/smoothing/max_displacement_mm"), 3), opt(f("/smoothing/rms_displacement_mm"), 4),
            opt(f("/smoothing/volume_mm3/change_percent"), 3),
            f("/mass_3d_g").unwrap_or(f64::NAN), f("/solid_tip/dolfinx/SF_nodal_max").unwrap_or(f64::NAN),
            f("/score").unwrap_or(f64::NAN), f("/score_with_brep").unwrap_or(f64::NAN),
        );
    }
    s
}

/// One `\LadderLevel{level}{pins}{iterations}{F}{F with B-rep}{max mm}{rms mm}{tip SF}{mass g}` per level.
pub fn levels_tex(levels: &[Value]) -> String {
    let mut s = String::from("% Generated by `jarambit smoothing-ladder`; do not edit.\n");
    for lv in levels {
        let f = |p: &str| lv.pointer(p).and_then(Value::as_f64).unwrap_or(0.0);
        s += &format!(
            "\\LadderLevel{{{}}}{{{}}}{{{}}}{{{:.6}}}{{{:.6}}}{{{:.3}}}{{{:.4}}}{{{:.4}}}{{{:.4}}}\n",
            lv["level"], lv["pin_deg"], lv["iterations"], f("/score"), f("/score_with_brep"),
            f("/smoothing/max_displacement_mm"), f("/smoothing/rms_displacement_mm"), f("/solid_tip/dolfinx/SF_nodal_max"), f("/mass_3d_g"),
        );
    }
    s
}

/// Whitespace-separated table for the PDF's score chart.
pub fn scores_dat(levels: &[Value]) -> String {
    let mut s = String::from("level rms_mm score score_with_brep tip_sf mass_g\n");
    for lv in levels {
        let f = |p: &str| lv.pointer(p).and_then(Value::as_f64).unwrap_or(0.0);
        s += &format!("{} {:.5} {:.6} {:.6} {:.4} {:.4}\n", lv["level"], f("/smoothing/rms_displacement_mm"), f("/score"), f("/score_with_brep"), f("/solid_tip/dolfinx/SF_nodal_max"), f("/mass_3d_g"));
    }
    s
}

pub fn run_smoothing_ladder_cli(args: &[String]) -> i32 {
    match run_ladder(args) {
        Ok(out) => {
            println!("smoothing ladder written to {}", out.display());
            0
        }
        Err(e) => {
            eprintln!("smoothing-ladder: {e}\n{USAGE}");
            2
        }
    }
}

fn run_ladder(args: &[String]) -> Res<PathBuf> {
    let arg = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
    let need = |k: &str| arg(k).ok_or_else(|| format!("missing {k}"));
    let root = std::env::current_dir().map_err(|e| e.to_string())?;
    if !root.join("fem_solid/tip_solid.py").is_file() || !root.join("ops/ci/evaluate_cad_candidate.py").is_file() {
        return Err("run from the repository root".into());
    }
    let levels = parse_levels(&arg("--levels").unwrap_or_else(|| DEFAULT_LEVELS.into()))?;
    let jobs: usize = arg("--jobs").map_or(Ok(5), |j| j.parse().map_err(|_| "bad --jobs".to_string()))?.max(1);
    let out = PathBuf::from(need("--out")?);
    if out.exists() {
        return Err(format!("{} exists; choose a new output directory", out.display()));
    }
    let label = need("--label")?;
    let winner = PathBuf::from(need("--winner")?);
    let smoother = PathBuf::from(need("--smoother")?);
    let python = PathBuf::from(need("--solver-python")?);
    let at = |s: &str| winner.join(format!("{label}{s}"));
    let ctx = Ctx {
        card: read_json(&at("_master_scorecard.json"))?,
        rs: read_json(&at("_rs.json"))?,
        d1: read_json(&at("_D1.json"))?,
        volume0: exporter_volume(&String::from_utf8_lossy(&read(&at("_export_stdout.txt"))?))?,
        root,
        winner: winner.clone(),
        label: label.clone(),
        smoother: smoother.clone(),
        python,
        out: out.clone(),
    };
    fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let exported = fixed(compute_canonical_ultralight_weighted_fitness(&ctx.card, &ctx.rs), 6);
    let results: Mutex<Vec<Option<Res<Value>>>> = Mutex::new(vec![None; levels.len()]);
    let next = Mutex::new(1usize);
    std::thread::scope(|scope| {
        for _ in 0..jobs.min(levels.len() - 1) {
            scope.spawn(|| loop {
                let i = {
                    let mut n = next.lock().expect("queue");
                    let i = *n;
                    *n += 1;
                    i
                };
                if i >= levels.len() {
                    break;
                }
                let r = smoothed_level(&ctx, i, levels[i]);
                eprintln!("L{i}: {}", r.as_ref().map_or_else(|e| format!("FAILED {e}"), |v| format!("F = {}", v["score"])));
                results.lock().expect("results")[i] = Some(r);
            });
        }
    });
    let stored = ctx.card.pointer("/engineering_metrics/solid_tip_sf_nodal_max").cloned().unwrap_or(Value::Null);
    let mut rows = vec![json!({
        "level": 0, "pin_deg": 0, "iterations": 0, "description": "as exported, stored scorecard, nothing re-solved",
        "mass_3d_g": ctx.card["engineering_metrics"]["mass_3d_g"],
        "solid_tip": {"dolfinx": {"SF_nodal_max": stored, "SF_nodal_avg": ctx.card["engineering_metrics"]["solid_tip_sf_nodal_avg"]}},
        "score": exported, "score_with_brep": exported, "smoothed_stl": file(&ctx, ".stl"),
    })];
    let mut failures = Vec::new();
    for (i, r) in results.into_inner().expect("results").into_iter().enumerate().skip(1) {
        match r {
            Some(Ok(mut v)) => {
                v["level"] = json!(i);
                v["pin_deg"] = json!(levels[i].pin_deg);
                v["iterations"] = json!(levels[i].iterations);
                v["smoothed_stl"] = v["smoothing"]["smoothed_stl"].clone();
                rows.push(v);
            }
            Some(Err(e)) => failures.push(format!("L{i}: {e}")),
            None => failures.push(format!("L{i}: not run")),
        }
    }
    if !failures.is_empty() {
        return Err(failures.join("; "));
    }
    let stl = read(&file(&ctx, ".stl"))?;
    let receipt = json!({
        "schema": "jarambit.smoothing_ladder.v1",
        "label": label,
        "code_commit": arg("--code-commit"),
        "command": std::env::args().collect::<Vec<_>>().join(" "),
        "winner_dir": winner,
        "inputs": {
            "exported_stl_sha256": sha(&stl),
            "master_scorecard_sha256": sha(&read(&at("_master_scorecard.json"))?),
            "rs_receipt_sha256": sha(&read(&at("_rs.json"))?),
            "d1_sha256": sha(&read(&at("_D1.json"))?),
            "exporter_volume_mm3": ctx.volume0,
        },
        "smoother": {"path": smoother, "sha256": sha(&read(&smoother)?), "lambda": 0.5, "mu": -0.5, "normal_only": true, "max_displacement_mm": 2.0},
        "scorer": "crates/cli/src/auto_evolve.rs::compute_canonical_ultralight_weighted_fitness (same value as ops/ci/ultralight_weighted_fitness.py)",
        "method": {
            "re_measured": "3D mass (exporter volume + smoothed - input STL volume, x 0.00443 g/mm3, 4 decimals) and the solid tip (fem_solid/tip_solid.py R_CUT 30 mm, NL 4, NSUB 1 on the smoothed upper skin, then the evaluator's DOLFINx 0.9.0 cross-check on the same mesh)",
            "withheld": "CAD/B-rep objective (cad_watertight_and_single_solid, cad.cad_valid) is false on every smoothed level: a smoothed triangle mesh has no validated STEP solid (owner decision 2026-10-22); score_with_brep keeps it as exported",
            "carried_over": "every other objective is genome-derived (plate responses, sharpness, point profile, ring, holder, aesthetics) and is not re-solved on the smoothed mesh",
        },
        "levels": rows,
        "not_qualified": true,
    });
    write(&out.join("ladder.json"), (serde_json::to_string_pretty(&receipt).map_err(|e| e.to_string())? + "\n").as_bytes())?;
    let rows = receipt["levels"].as_array().ok_or("levels")?;
    write(&out.join("ladder-table.tex"), table_tex(rows).as_bytes())?;
    write(&out.join("ladder-levels.tex"), levels_tex(rows).as_bytes())?;
    write(&out.join("ladder-scores.dat"), scores_dat(rows).as_bytes())?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ladder_starts_as_exported_and_rejects_zero_iteration_smoothing() {
        let levels = parse_levels(DEFAULT_LEVELS).unwrap();
        assert_eq!(levels.len(), 10);
        assert_eq!(levels[4], Level { pin_deg: 30, iterations: 20 });
        assert!(parse_levels("30:2,0:0").is_err());
        assert!(parse_levels("0:0,30:0").is_err());
        assert!(parse_levels("0:0,0:0").is_err());
    }

    #[test]
    fn surface_vertices_must_each_match_one_smoothing_input_vertex() {
        let csv = "vertex,x_before_mm,y_before_mm,z_before_mm,x_after_mm,y_after_mm,z_after_mm,displacement_mm\n\
0,1.0000,2.0000,0.5000,1.0,2.0,0.6,0.1\n1,3.0000,4.0000,0.5000,3.0,4.0,0.5,0\n";
        let d1 = json!({"export_surface": {"verts": [[1.00001, 2.0, 0.5], [3.0, 4.0, 0.49999]]}});
        let (doc, n) = smoothed_d1(&d1, csv).unwrap();
        assert_eq!(n, 2);
        assert_eq!(doc["export_surface"]["verts"][0], json!([1.0, 2.0, 0.6]));
        let unmatched = json!({"export_surface": {"verts": [[9.0, 9.0, 9.0]]}});
        assert!(smoothed_d1(&unmatched, csv).is_err());
        let duplicate = json!({"export_surface": {"verts": [[1.0, 2.0, 0.5], [1.0, 2.0, 0.5]]}});
        assert!(smoothed_d1(&duplicate, csv).is_err());
    }

    #[test]
    fn exporter_volume_reads_the_last_volume_line() {
        assert_eq!(exporter_volume("boundary 0 volume 12.5 mm3\nvolume 20928.245750651 mm3\n").unwrap(), 20928.245750651);
        assert!(exporter_volume("no volume here").is_err());
    }

    #[test]
    fn smoothed_scorecard_replaces_only_geometry_reading_measurements() {
        let card = json!({"subject": "d", "cad": {"cad_valid": true},
            "engineering_metrics": {"mass_3d_g": 92.7, "solid_tip_sf_nodal_max": 0.35, "spg": 4.0},
            "weighted_cad_score": {"gates": {"cad_watertight_and_single_solid": true, "ring_pull_1500n": true}}});
        let skfem = json!({"solid": {"SF_solid_nodal_max": 0.377, "SF_solid_nodal_avg": 0.412}});
        let dx = json!({"SF_nodal_max": 0.3766, "SF_nodal_avg": 0.4123});
        let out = smoothed_scorecard(&card, 92.6455, &skfem, &dx, 4, true).unwrap();
        assert_eq!(out["engineering_metrics"]["mass_3d_g"], json!(92.6455));
        assert_eq!(out["engineering_metrics"]["solid_tip_sf_nodal_max"], json!(0.3766));
        assert_eq!(out["engineering_metrics"]["spg"], json!(4.0));
        assert_eq!(out["weighted_cad_score"]["gates"]["cad_watertight_and_single_solid"], json!(false));
        assert_eq!(out["weighted_cad_score"]["gates"]["ring_pull_1500n"], json!(true));
        assert_eq!(out["weighted_cad_score"]["gates"]["dolfinx_cross_check_2pct"], json!(true));
        assert_eq!(out["cad"]["cad_valid"], json!(false));
        assert_eq!(out["subject"], json!("d_smoothed_L4"));
        let kept = smoothed_scorecard(&card, 92.6455, &skfem, &dx, 4, false).unwrap();
        assert_eq!(kept["weighted_cad_score"]["gates"]["cad_watertight_and_single_solid"], json!(true));
        assert_eq!(kept["cad"]["cad_valid"], json!(true));
    }
}
