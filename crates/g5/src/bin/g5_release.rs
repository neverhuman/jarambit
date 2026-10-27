//! G5 release receipt. Assembles `evidence/g5/v<version>/` from the finished cohort of record and its certification
//! runs, re-checks every link between them, and fails closed: any mismatch exits 1 before anything is written.
//!
//!   g5-release receipt <repo_root> <cohort_root> <baseline_root> <cert_dir> [shortlist=32] [grid_top=8]
//!   g5-release top <tier2_h020.jsonl> <n>     the ranking's top n design ids, comma-separated (for the grid check)
//!
//! Inputs (written by g5-evolve and g5-pilot; no number is typed by hand):
//! - `<cohort_root>`: `islandNN/run/{archive.jsonl, genomes_g5.csv, history.jsonl}` and `supervisor.log`;
//! - `<baseline_root>`: the same for the Sobol random-search control (same budget, not eligible to win);
//! - `<cert_dir>/eval_h040.jsonl`, `baseline_eval_h040.jsonl`: every final elite at Tier 1, h 0.4 (`g5-pilot eval`);
//! - `<cert_dir>/tier2_h020.jsonl`: the shortlist at Tier 2, h 0.2 (`g5-pilot tier2`), the ranking of record;
//! - `<cert_dir>/hconv.jsonl`: the ranking's top designs at Tier 1 on five grids (`g5-pilot hconv`);
//! - `<cert_dir>/tier2_linux_h020.jsonl`: the same top designs at Tier 2 on Linux x86_64 (cross-platform check);
//! - `<cert_dir>/assets/` (optional): release assets too large for git, listed with their sha256.
//!
//! Checks:
//! 1. every record and archive row carries this binary's decoder hash, and every genome row re-hashes to its id;
//! 2. `eval_h040` covers every final elite and reproduces every Tier-1 archive row bit for bit;
//! 3. the shortlist is the top `shortlist` of `eval_h040`; the grid check and the Linux run cover the ranking's top
//!    `grid_top`;
//! 4. the winner is the highest-ranked design whose h 0.2 mass agrees with h 0.15 within RS-karambit-v5 `[mass]
//!    grid_agreement_max_frac` (designs that do not are reported, never ranked), and its meshes are watertight.
//!
//! Outputs: `evidence/g5/v<version>/{release.json, winner_genome.csv, elites_h040.csv, eval_h040.jsonl,
//! baseline_eval_h040.jsonl, ranking_tier2_h020.jsonl, grid_check.jsonl, tier2_linux_h020.jsonl,
//! history/islandNN.jsonl, genomes/islandNN.csv, SHA256SUMS}` (`vv/`
//! keeps its own sums), the evolution figure `docs/g5/v<version>/img/evolution.svg`, and `evidence/g5/RECORD`.
use jarambit_crescent::export::{census, volume};
use jarambit_g5::g5_genome::G5Genome;
use jarambit_g5::g5_namespace::validate_g5_design_id;
use jarambit_g5::g5_qd::{
    g5_decoder_sha256, measure_blade_edge_apex_and_angle, measure_point_planform, serration_tooth_strength, tooth_pry_per_tooth_n,
    tooth_snag_n,
};
use jarambit_g5::g5_sdf::{build_blade_profile_2d, extract_watertight_mesh_fail_closed};
use jarambit_g5::loads::{G5Loads, TIP_FE_MAX_AREA_MM2};
use jarambit_g5::{material, objective, requirements, sha256_bytes};
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const H_SEARCH: f64 = 0.4;
const H_RECORD: f64 = 0.2;
const H_FINE: f64 = 0.15;
const GRID_H: [f64; 5] = [0.4, 0.3, 0.25, 0.2, 0.15];

fn fail(msg: impl std::fmt::Display) -> ! {
    eprintln!("g5-release: FAIL: {msg}");
    std::process::exit(1)
}

fn read(p: &Path) -> String {
    fs::read_to_string(p).unwrap_or_else(|e| fail(format!("{}: {e}", p.display())))
}

fn jsonl(p: &Path) -> Vec<Value> {
    read(p)
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).unwrap_or_else(|e| fail(format!("{}: {e}", p.display()))))
        .collect()
}

fn f(v: &Value, ptr: &str) -> f64 {
    match v.pointer(ptr) {
        Some(Value::Number(n)) => n.as_f64().unwrap(),
        Some(x) => fail(format!("{ptr}: not a finite number ({x})")),
        None => fail(format!("{ptr}: missing")),
    }
}

fn s<'a>(v: &'a Value, ptr: &str) -> &'a str {
    v.pointer(ptr).and_then(Value::as_str).unwrap_or_else(|| fail(format!("{ptr}: missing string")))
}

/// A float for the receipt: non-finite values are written as strings ("inf"), as the archive does.
fn num(x: f64) -> Value {
    if x.is_finite() {
        json!(x)
    } else {
        json!(format!("{x}"))
    }
}

struct Island {
    name: String,
    dir: PathBuf,
    archive: Vec<Value>,
    genomes: Vec<(String, String)>,
    history: Vec<Value>,
}

fn islands(root: &Path) -> Vec<Island> {
    let mut dirs: Vec<PathBuf> = fs::read_dir(root)
        .unwrap_or_else(|e| fail(format!("{}: {e}", root.display())))
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir() && p.file_name().is_some_and(|n| n.to_string_lossy().starts_with("island")))
        .collect();
    dirs.sort();
    if dirs.is_empty() {
        fail(format!("{}: no islands", root.display()));
    }
    dirs.into_iter()
        .map(|d| {
            let run = d.join("run");
            let genomes = read(&run.join("genomes_g5.csv"))
                .lines()
                .skip(1)
                .filter(|l| !l.trim().is_empty())
                .map(|l| {
                    let (id, row) = l.split_once(',').unwrap_or_else(|| fail(format!("{}: bad row", d.display())));
                    (id.to_string(), row.to_string())
                })
                .collect();
            Island {
                name: d.file_name().unwrap().to_string_lossy().into_owned(),
                archive: jsonl(&run.join("archive.jsonl")),
                genomes,
                history: jsonl(&run.join("history.jsonl")),
                dir: d,
            }
        })
        .collect()
}

/// Check 1 for one root: decoder hash on every archive row, every genome row re-hashing to its design id.
fn verify_root(isl: &[Island], hash: &str) -> HashMap<String, (String, G5Genome)> {
    let mut genomes = HashMap::new();
    for i in isl {
        for r in &i.archive {
            if s(r, "/decoder_sha256") != hash {
                fail(format!("{}: {} has decoder {}", i.name, s(r, "/design_id"), s(r, "/decoder_sha256")));
            }
        }
        for (id, row) in &i.genomes {
            validate_g5_design_id(id).unwrap_or_else(|e| fail(format!("{id}: {e}")));
            let g = G5Genome::from_csv_row(row).unwrap_or_else(|e| fail(format!("{id}: {e}")));
            if !id.ends_with(&g.sha256()[..8]) {
                fail(format!("{id}: genome sha256 {} does not match the id", g.sha256()));
            }
            if genomes.insert(id.clone(), (i.name.clone(), g)).is_some() {
                fail(format!("{id}: duplicate design id"));
            }
        }
        if i.genomes.len() != i.archive.len() {
            fail(format!("{}: {} genomes vs {} archive rows", i.name, i.genomes.len(), i.archive.len()));
        }
    }
    genomes
}

/// Check 2: the Tier-1 re-evaluation covers every elite and reproduces every Tier-1 archive row bit for bit.
fn verify_eval(isl: &[Island], eval: &[Value], hash: &str) -> usize {
    let rows: HashMap<&str, &Value> =
        isl.iter().flat_map(|i| i.archive.iter()).map(|r| (s(r, "/design_id"), r)).collect();
    let ids: BTreeSet<&str> = eval.iter().map(|l| s(l, "/design_id")).collect();
    if ids.len() != eval.len() || ids != rows.keys().copied().collect::<BTreeSet<_>>() {
        fail("eval_h040 does not cover exactly the final elites");
    }
    let mut reproduced = 0;
    for l in eval {
        let (id, rec) = (s(l, "/design_id"), &l["record"]);
        if s(rec, "/decoder_sha256") != hash || f(l, "/h") != H_SEARCH {
            fail(format!("{id}: eval record has another decoder or grid"));
        }
        let row = rows[id];
        if s(row, "/tier") == "Tier1SdfAndPlate" {
            for (a, b) in [("/raw_score_100", "/score/raw_score_100"), ("/inputs/mass_g", "/inputs/mass_g")] {
                if f(row, a).to_bits() != f(rec, b).to_bits() {
                    fail(format!("{id}: Tier-1 re-evaluation {} != archive {} ({a})", f(rec, b), f(row, a)));
                }
            }
            reproduced += 1;
        }
    }
    reproduced
}

/// The score breakdown with named fields (terms and hand factors as objects, not tuples).
fn score_json(sc: &Value) -> Value {
    let mut terms = Map::new();
    for t in sc["terms"].as_array().unwrap_or_else(|| fail("score.terms")) {
        terms.insert(s(t, "/0").to_string(), json!({"weight": t[1], "s": t[2], "points": t[3]}));
    }
    let mut ergo = Map::new();
    for e in sc["ergo"].as_array().unwrap_or_else(|| fail("score.ergo")) {
        ergo.insert(s(e, "/0").to_string(), json!({"beta": e[1], "e": e[2]}));
    }
    json!({"metric": sc["metric"], "draft": sc["draft"], "score_100": sc["score_100"], "raw_score_100": sc["raw_score_100"],
           "E": sc["E"], "base": sc["base"], "worst_margin_W": terms["worst_margin"]["s"], "terms": terms, "hand_factors": ergo})
}

fn margins_json(rec: &Value) -> (Value, f64) {
    let loads: G5Loads = serde_json::from_value(rec["loads"].clone()).unwrap_or_else(|e| fail(format!("loads: {e}")));
    let mut m = Map::new();
    let mut mu = f64::INFINITY;
    for (k, v) in objective::margins(&loads) {
        mu = mu.min(if v.is_nan() { 0.0 } else { v });
        m.insert(k.to_string(), num(v));
    }
    (Value::Object(m), mu)
}

/// One island's evolution: Sobol start, final best, and the checkpoints where its best design changed.
fn island_evolution(i: &Island, seeds: &BTreeMap<String, u64>) -> Value {
    let h = &i.history;
    let first = h.first().unwrap_or_else(|| fail(format!("{}: empty history", i.name)));
    let last = h.last().unwrap();
    let mut changes = Vec::new();
    let mut prev = "";
    for e in h {
        let g = s(e, "/best_genome_sha256");
        if g != prev {
            changes.push(json!({"evals": e["evals"], "best_score_100": e["best_score_100"], "best_mass_g": e["best_mass_g"],
                                "best_genome_sha256": g}));
            prev = g;
        }
    }
    json!({"island": i.name, "seed": seeds.get(&i.name), "evaluations": last["evals"], "niches": last["niches"],
           "qd_score": last["qd_score"], "after_sobol_init": {"evals": first["evals"], "best_score_100": first["best_score_100"],
           "best_mass_g": first["best_mass_g"]}, "final": {"best_score_100": last["best_score_100"],
           "best_mass_g": last["best_mass_g"], "best_genome_sha256": last["best_genome_sha256"]}, "best_changes": changes})
}

/// `[TS] island NN started pid P seed S` lines, the first start and the `supervisor done` time.
fn supervisor(root: &Path) -> (BTreeMap<String, u64>, String, String) {
    let log = read(&root.join("supervisor.log"));
    let ts = |l: &str| l.trim_start_matches('[').split(']').next().unwrap_or("").to_string();
    let mut seeds = BTreeMap::new();
    let mut started = String::new();
    let mut done = String::new();
    for l in log.lines() {
        let w: Vec<&str> = l.split_whitespace().collect();
        if w.len() >= 8 && w[1] == "island" && w[3] == "started" && w[6] == "seed" {
            seeds.insert(format!("island{}", w[2]), w[7].parse().unwrap_or_else(|_| fail(format!("seed in '{l}'"))));
            if started.is_empty() {
                started = ts(l);
            }
        }
        if l.ends_with("supervisor done") {
            done = ts(l);
        }
    }
    if started.is_empty() || done.is_empty() {
        fail(format!("{}: supervisor.log has no start or no 'supervisor done'", root.display()));
    }
    (seeds, started, done)
}

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(f64::total_cmp);
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        0.5 * (v[n / 2 - 1] + v[n / 2])
    }
}

fn sha_file(p: &Path) -> String {
    sha256_bytes(&fs::read(p).unwrap_or_else(|e| fail(format!("{}: {e}", p.display()))))
}

fn files_under(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(dir).into_iter().flatten().flatten() {
        let p = e.path();
        if p.is_dir() {
            files_under(&p, out);
        } else {
            out.push(p);
        }
    }
}

/// The ranking of record: Tier-2 score, highest first, ties by design id.
fn rank(t2: &[Value]) -> Vec<&Value> {
    let mut r: Vec<&Value> = t2.iter().collect();
    r.sort_by(|x, y| {
        f(y, "/record/score/score_100").total_cmp(&f(x, "/record/score/score_100")).then(s(x, "/design_id").cmp(s(y, "/design_id")))
    });
    r
}

const USAGE: &str = "usage: g5-release receipt <repo_root> <cohort_root> <baseline_root> <cert_dir> [shortlist=32] [grid_top=8]\n\
                     \x20      g5-release top <tier2_h020.jsonl> <n>";

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("top") if a.len() == 4 => {
            let t2 = jsonl(Path::new(&a[2]));
            let n: usize = a[3].parse().unwrap_or_else(|_| fail(USAGE));
            let ids: Vec<&str> = rank(&t2).iter().take(n).map(|l| s(l, "/design_id")).collect();
            println!("{}", ids.join(","));
        }
        Some("receipt") if a.len() >= 6 => receipt(&a[2..]),
        _ => fail(USAGE),
    }
}

fn receipt(a: &[String]) {
    let (repo, cohort_root, base_root, cert) = (Path::new(&a[0]), Path::new(&a[1]), Path::new(&a[2]), Path::new(&a[3]));
    let shortlist: usize = a.get(4).map_or(32, |v| v.parse().unwrap_or_else(|_| fail("shortlist")));
    let grid_top: usize = a.get(5).map_or(8, |v| v.parse().unwrap_or_else(|_| fail("grid_top")));
    let hash = g5_decoder_sha256();
    let rs = requirements::rs_v5();

    // --- check 1 and 2: the cohort, the control and their Tier-1 re-evaluations
    let cohort = islands(cohort_root);
    let baseline = islands(base_root);
    let genomes = verify_root(&cohort, &hash);
    let base_genomes = verify_root(&baseline, &hash);
    if genomes.keys().any(|k| base_genomes.contains_key(k)) {
        fail("a design id appears in both the cohort and the control");
    }
    let eval = jsonl(&cert.join("eval_h040.jsonl"));
    let base_eval = jsonl(&cert.join("baseline_eval_h040.jsonl"));
    let reproduced = verify_eval(&cohort, &eval, &hash);
    let base_reproduced = verify_eval(&baseline, &base_eval, &hash);

    // --- check 3: the shortlist (the same stable sort as `g5-pilot tier2`) and the ranking of record
    let mut ev: Vec<(f64, &str)> = eval.iter().map(|l| (f(l, "/record/score/score_100"), s(l, "/design_id"))).collect();
    ev.sort_by(|x, y| y.0.total_cmp(&x.0));
    let expect: BTreeSet<&str> = ev.iter().take(shortlist).map(|x| x.1).collect();
    let t2 = jsonl(&cert.join("tier2_h020.jsonl"));
    let got: BTreeSet<&str> = t2.iter().map(|l| s(l, "/design_id")).collect();
    if got != expect || t2.len() != shortlist {
        fail(format!("tier2_h020 is not the top {shortlist} of eval_h040"));
    }
    for l in &t2 {
        if s(l, "/record/tier") != "Tier2Full3D" || f(l, "/h") != H_RECORD || s(l, "/record/decoder_sha256") != hash {
            fail(format!("{}: ranking record is not Tier 2 at h {H_RECORD} with this decoder", s(l, "/design_id")));
        }
    }
    let ranking = rank(&t2);
    let top: Vec<&str> = ranking.iter().take(grid_top).map(|l| s(l, "/design_id")).collect();

    // --- check 4: the grid check, then the winner
    let hc = jsonl(&cert.join("hconv.jsonl"));
    let mut grid: BTreeMap<&str, BTreeMap<u64, &Value>> = BTreeMap::new();
    for l in &hc {
        if s(l, "/record/decoder_sha256") != hash {
            fail("hconv record with another decoder");
        }
        grid.entry(s(l, "/design_id")).or_default().insert((f(l, "/h") * 1000.0).round() as u64, &l["record"]);
    }
    if grid.keys().copied().collect::<BTreeSet<_>>() != top.iter().copied().collect::<BTreeSet<_>>() {
        fail(format!("hconv does not cover exactly the ranking's top {grid_top}"));
    }
    let at = |id: &str, h: f64| -> &Value {
        grid[id].get(&((h * 1000.0).round() as u64)).copied().unwrap_or_else(|| fail(format!("hconv: {id} at h {h}")))
    };
    let max_frac = rs.mass.grid_agreement_max_frac;
    let grid_rows: Vec<Value> = top
        .iter()
        .map(|&id| {
            let rel = (f(at(id, H_RECORD), "/inputs/mass_g") / f(at(id, H_FINE), "/inputs/mass_g") - 1.0).abs();
            json!({"design_id": id, "mass_g": GRID_H.iter().map(|&h| f(at(id, h), "/inputs/mass_g")).collect::<Vec<_>>(),
                   "tier1_score_100": GRID_H.iter().map(|&h| f(at(id, h), "/score/score_100")).collect::<Vec<_>>(),
                   "sealed_void_mm3": GRID_H.iter().map(|&h| f(at(id, h), "/am_connectivity/sealed_void_mm3")).collect::<Vec<_>>(),
                   "mass_rel_h020_vs_h015": rel, "pass": rel <= max_frac})
        })
        .collect();
    let rank_of = |id: &str| ranking.iter().position(|l| s(l, "/design_id") == id).unwrap() + 1;
    let win_row = grid_rows
        .iter()
        .find(|r| r["pass"] == json!(true))
        .unwrap_or_else(|| fail(format!("no design in the top {grid_top} passes the grid check")));
    let w = s(win_row, "/design_id").to_string();
    let wl = ranking[rank_of(&w) - 1];
    let rec = &wl["record"];
    let (island, g) = &genomes[&w];

    // cross-platform: the top designs at Tier 2, h 0.2 on Linux x86_64
    let lin = jsonl(&cert.join("tier2_linux_h020.jsonl"));
    let lin_ids: BTreeSet<&str> = lin.iter().map(|l| s(l, "/design_id")).collect();
    if lin_ids != top.iter().copied().collect::<BTreeSet<_>>() {
        fail(format!("tier2_linux_h020 does not cover exactly the ranking's top {grid_top}"));
    }
    let mut lin_max_rel = 0.0_f64;
    let mut lin_identical = true;
    for l in &lin {
        let id = s(l, "/design_id");
        let arm = &ranking[rank_of(id) - 1]["record"];
        if s(l, "/record/decoder_sha256") != hash || f(l, "/h") != H_RECORD {
            fail(format!("{id}: Linux record has another decoder or grid"));
        }
        for p in ["/score/raw_score_100", "/inputs/mass_g"] {
            let (x, y) = (f(&l["record"], p), f(arm, p));
            lin_identical &= x.to_bits() == y.to_bits();
            lin_max_rel = lin_max_rel.max(((x - y) / y).abs());
        }
    }

    // --- the winner: meshes, point, margins, its search record
    let am = extract_watertight_mesh_fail_closed(g, H_RECORD, true).unwrap_or_else(|| fail("winner AM build not watertight"));
    let env = extract_watertight_mesh_fail_closed(g, H_RECORD, false).unwrap_or_else(|| fail("winner envelope not watertight"));
    let (am_census, env_census) = (census(&am.tris), census(&env.tris));
    let rho = material::density_g_mm3();
    let p = g.decode();
    let bp = build_blade_profile_2d(&p);
    let point = measure_point_planform(&bp);
    let (apex, half) = measure_blade_edge_apex_and_angle(&p, &bp);
    let edge = |inner: bool| {
        let (sf, checked, flank) = serration_tooth_strength(&p, &bp, inner);
        json!({"serration_depth_mm": if inner { bp.serr_depth_in_mm } else { bp.serr_depth_sp_mm },
               "teeth": if inner { bp.serr_teeth_in } else { bp.serr_teeth_sp }, "tooth_root_sf_min": num(sf),
               "teeth_checked": checked, "mean_flank_deg": num(flank),
               "per_side_angle_gene_deg": p[if inner { "angle_in" } else { "angle_sp" }],
               "sharpened_arc_mm": if inner { bp.sharpened_arc_in_mm } else { bp.sharpened_arc_sp_mm },
               "edge_arc_mm": if inner { bp.arc_in_mm } else { bp.arc_sp_mm }})
    };
    let blade = json!({"max_apex_mm": apex, "max_half_angle_deg": half, "edge_inner": edge(true), "edge_spine": edge(false),
                       "tooth_loads_n": {"snag_along_edge": tooth_snag_n(), "pry_across_blade_per_tooth": tooth_pry_per_tooth_n()},
                       "reach_chord_mm": bp.reach_chord_mm, "tip_xy_mm": [bp.tip_xy.0, bp.tip_xy.1]});
    let inp = &rec["inputs"];
    let ergonomics = json!({"grip_perimeter_mm": inp["swell_perimeter_mm"], "grip_h_over_t": inp["swell_h_over_t"],
                            "grip_length_mm": inp["L_g_mm"], "guard_height_mm": inp["guard_h_mm"],
                            "com_to_index_mm": inp["com_to_index_mm"],
                            "spin_inertia_ratio": f(inp, "/spin_I_g_mm2") / f(inp, "/spin_I_ref_g_mm2"),
                            "hand_factors": score_json(&rec["score"])["hand_factors"]});
    let (margins, mu) = margins_json(rec);
    let search_row = cohort.iter().flat_map(|i| i.archive.iter()).find(|r| s(r, "/design_id") == w).unwrap();

    // --- evolution
    let (seeds, started, finished) = supervisor(cohort_root);
    let (base_seeds, base_started, base_finished) = supervisor(base_root);
    let evo: Vec<Value> = cohort.iter().map(|i| island_evolution(i, &seeds)).collect();
    let base_evo: Vec<Value> = baseline.iter().map(|i| island_evolution(i, &base_seeds)).collect();
    let finals = |v: &[Value]| v.iter().map(|e| f(e, "/final/best_score_100")).collect::<Vec<f64>>();
    let (mut cf, bf) = (finals(&evo), finals(&base_evo));
    let best_base = bf.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let cohort_beats_baseline = cf.iter().filter(|&&x| x > best_base).count();
    let (cmin, cmax) = (cf.iter().copied().fold(f64::INFINITY, f64::min), cf.iter().copied().fold(f64::NEG_INFINITY, f64::max));
    let cmed = median(&mut cf);
    let mut init_scores: Vec<f64> = evo.iter().map(|e| f(e, "/after_sobol_init/best_score_100")).collect();
    let init_med = median(&mut init_scores);
    let win_sha = g.sha256();
    let first_seen = cohort
        .iter()
        .find(|i| i.name == *island)
        .unwrap()
        .history
        .iter()
        .find(|e| s(e, "/best_genome_sha256") == win_sha)
        .map(|e| e["evals"].clone());
    let n_evals = f(&evo[0], "/evaluations");
    let init = f(&evo[0], "/after_sobol_init/evals");

    // --- write (nothing above wrote a file)
    let ev_dir = repo.join(format!("evidence/g5/v{VERSION}"));
    let img_dir = repo.join(format!("docs/g5/v{VERSION}/img"));
    for d in [ev_dir.join("history"), ev_dir.join("genomes"), img_dir.clone()] {
        fs::create_dir_all(&d).unwrap();
    }
    let row = cohort.iter().flat_map(|i| i.genomes.iter()).find(|(id, _)| *id == w).unwrap();
    fs::write(ev_dir.join("winner_genome.csv"), format!("design_id,genome_row\n{},{}\n", row.0, row.1)).unwrap();
    let mut csv = String::from("group,island,design_id,archive_tier,tier1_score_100,mass_g,worst_margin_W,E\n");
    for (group, isl, ev) in [("cohort", &cohort, &eval), ("baseline", &baseline, &base_eval)] {
        let tiers: HashMap<&str, &str> =
            isl.iter().flat_map(|i| i.archive.iter()).map(|r| (s(r, "/design_id"), s(r, "/tier"))).collect();
        let mut lines: Vec<&Value> = ev.iter().collect();
        lines.sort_by(|x, y| s(x, "/design_id").cmp(s(y, "/design_id")));
        for l in lines {
            let (id, r) = (s(l, "/design_id"), &l["record"]);
            let sc = score_json(&r["score"]);
            csv += &format!("{group},{},{id},{},{},{},{},{}\n", s(l, "/island"), tiers[id], r["score"]["score_100"],
                            r["inputs"]["mass_g"], sc["worst_margin_W"], r["score"]["E"]);
        }
    }
    fs::write(ev_dir.join("elites_h040.csv"), csv).unwrap();
    let ranked: String = ranking.iter().map(|l| l.to_string() + "\n").collect();
    fs::write(ev_dir.join("ranking_tier2_h020.jsonl"), ranked).unwrap();
    fs::copy(cert.join("hconv.jsonl"), ev_dir.join("grid_check.jsonl")).unwrap();
    fs::copy(cert.join("eval_h040.jsonl"), ev_dir.join("eval_h040.jsonl")).unwrap();
    fs::copy(cert.join("baseline_eval_h040.jsonl"), ev_dir.join("baseline_eval_h040.jsonl")).unwrap();
    fs::copy(cert.join("tier2_linux_h020.jsonl"), ev_dir.join("tier2_linux_h020.jsonl")).unwrap();
    for i in cohort.iter().chain(baseline.iter()) {
        fs::copy(i.dir.join("run/history.jsonl"), ev_dir.join(format!("history/{}.jsonl", i.name))).unwrap();
        fs::copy(i.dir.join("run/genomes_g5.csv"), ev_dir.join(format!("genomes/{}.csv", i.name))).unwrap();
    }
    let assets: Vec<Value> = {
        let mut v = Vec::new();
        files_under(&cert.join("assets"), &mut v);
        v.sort();
        v.iter()
            .map(|p| json!({"file": p.file_name().unwrap().to_string_lossy(), "bytes": fs::metadata(p).unwrap().len(),
                            "sha256": sha_file(p)}))
            .collect()
    };
    let figure = format!("docs/g5/v{VERSION}/img/evolution.svg");
    fs::write(repo.join(&figure), evolution_svg(&cohort, &baseline, island, n_evals)).unwrap();

    let ranking_rows: Vec<Value> = ranking
        .iter()
        .enumerate()
        .map(|(k, l)| {
            let r = &l["record"];
            let id = s(l, "/design_id");
            json!({"rank": k + 1, "design_id": id, "island": genomes[id].0, "score_100": r["score"]["score_100"],
                   "mass_g": r["inputs"]["mass_g"], "E": r["score"]["E"], "worst_margin_W": score_json(&r["score"])["worst_margin_W"],
                   "grid_pass": grid_rows.iter().find(|g| g["design_id"] == json!(id)).map(|g| g["pass"].clone())})
        })
        .collect();
    let receipt = json!({
        "version": VERSION,
        "generation": "G5",
        "schema": "G5.0",
        "design_id": w,
        "status": "computational design; not built, printed or physically tested",
        "provenance": {
            "decoder_sha256": hash, "decoder_commit_label": g.decoder_commit, "genome_sha256": win_sha,
            "genome_csv": format!("evidence/g5/v{VERSION}/winner_genome.csv"), "requirements": "requirements/RS-karambit-v5.toml",
            "metric": rec["score"]["metric"], "metric_draft": rec["score"]["draft"],
            "platform_of_record": "arm64 macOS (xbabe4), Tier 2 at h 0.2 mm; checked on Linux x86_64 (xbabe2)",
            "vv": format!("evidence/g5/v{VERSION}/vv"),
        },
        "cohort": {
            "root": cohort_root.file_name().unwrap().to_string_lossy(), "islands": cohort.len(),
            "evaluations_per_island": n_evals, "evaluations_total": n_evals * cohort.len() as f64,
            "init": format!("scrambled Sobol only ({init} per island); no seeded or hand-made individuals"),
            "h_mm": H_SEARCH, "promotion": "rank promotion: the top 40 % of each island's recent scores go on to Tier 1",
            "elites": genomes.len(), "tier1_rows_reproduced": reproduced, "started_utc": started, "finished_utc": finished,
        },
        "baseline": {
            "root": base_root.file_name().unwrap().to_string_lossy(), "islands": baseline.len(),
            "evaluations_per_island": f(&base_evo[0], "/evaluations"),
            "what": "Sobol random search with the cohort's budget, decoder, objective and promotion; not eligible to win",
            "elites": base_genomes.len(), "tier1_rows_reproduced": base_reproduced,
            "started_utc": base_started, "finished_utc": base_finished, "islands_detail": base_evo,
        },
        "ranking": {
            "h_mm": H_RECORD, "tip_mesh_max_area_mm2": TIP_FE_MAX_AREA_MM2, "shortlist": shortlist,
            "how": format!("every final elite re-evaluated at Tier 1, h {H_SEARCH}; the top {shortlist} at Tier 2, h {H_RECORD}"),
            "rows": ranking_rows,
            "linux_check": {"designs": grid_top, "bit_identical": lin_identical, "max_rel_diff": lin_max_rel},
        },
        "grid_check": {
            "h_mm": GRID_H, "requirement_max_frac": max_frac, "evaluation": format!("Tier 1 of the ranking's top {grid_top}"),
            "rule": "the winner is the highest-ranked design whose h 0.2 mass agrees with h 0.15; others are reported, never ranked",
            "rows": grid_rows,
        },
        "rank_of_record": rank_of(&w),
        "score": score_json(&rec["score"]),
        "margins": margins, "worst_margin": num(mu), "meets_every_load_case": mu >= 1.0,
        "loads": rec["loads"], "inputs": rec["inputs"], "descriptors": rec["descriptors"], "am": rec["am_connectivity"],
        "mass": {
            "of_record_g": rec["inputs"]["mass_g"], "of_record": "partial-volume integral of the decoded AM field at h 0.2 mm",
            "search_h04_g": search_row["inputs"]["mass_g"], "print_mesh_g": volume(&am) * rho,
            "print_mesh_volume_mm3": volume(&am), "envelope_mesh_volume_mm3": volume(&env), "density_g_mm3": rho,
            "print_mesh_census": {"triangles": am.tris.len(), "boundary_edges": am_census.0, "non_manifold_edges": am_census.1},
            "envelope_census": {"triangles": env.tris.len(), "boundary_edges": env_census.0, "non_manifold_edges": env_census.1},
        },
        "search_score_100": search_row["score_100"],
        "blade": blade, "ergonomics": ergonomics,
        "point": {"angle_1mm_deg": point.angle_1mm_deg, "angle_3mm_deg": point.angle_3mm_deg, "width_2mm": point.width_2mm},
        "decoded": p.iter().map(|(k, v)| (k.to_string(), num(*v))).collect::<Map<_, _>>(),
        "evolution": {
            "history_dir": format!("evidence/g5/v{VERSION}/history"), "figure": figure, "winner_island": island,
            "winner_first_island_best_at_evals": first_seen,
            "cohort_final_best_score_100": {"median": cmed, "min": cmin, "max": cmax},
            "cohort_after_sobol_init_median_score_100": init_med,
            "baseline_final_best_score_100": bf, "cohort_islands_beating_best_baseline": cohort_beats_baseline,
            "islands": evo,
        },
        "release_assets": assets,
    });
    fs::write(ev_dir.join("release.json"), serde_json::to_string_pretty(&receipt).unwrap() + "\n").unwrap();
    fs::write(repo.join("evidence/g5/RECORD"), format!("{VERSION}\n")).unwrap();

    let mut files = Vec::new();
    files_under(&ev_dir, &mut files);
    files_under(&repo.join(format!("docs/g5/v{VERSION}")), &mut files);
    files.retain(|p| p.file_name().is_some_and(|n| n != "SHA256SUMS") && !p.starts_with(ev_dir.join("vv")));
    let mut rel: Vec<(String, &PathBuf)> =
        files.iter().map(|p| (p.strip_prefix(repo).unwrap().to_string_lossy().into_owned(), p)).collect();
    rel.sort();
    let sums: String = rel.iter().map(|(r, p)| format!("{}  {r}\n", sha_file(p))).collect();
    fs::write(ev_dir.join("SHA256SUMS"), sums).unwrap();
    println!("{}", json!({"design_id": w, "rank": rank_of(&w), "score_100": rec["score"]["score_100"],
                          "mass_g": rec["inputs"]["mass_g"], "worst_margin": num(mu), "files": rel.len()}));
}

/// The evolution figure: each evolving island's best score against evaluations, as steps (the best never falls).
/// The winner's island is series 1 (blue) and the other cohort islands thin neutral context. A random-search island
/// logs once, at the end of its samples, so the control is drawn as a reference level: its best score after the full
/// budget (series 2, orange, dashed). Colours are per-class CSS with a dark-mode set, both validated for colour-vision deficiency
/// (adjacent CVD dE 24.7 light / 26.8 dark); identity is never colour alone (legend, direct labels, dash).
fn evolution_svg(cohort: &[Island], baseline: &[Island], winner_island: &str, n_evals: f64) -> String {
    let pts = |i: &Island| -> Vec<(f64, f64)> { i.history.iter().map(|e| (f(e, "/evals"), f(e, "/best_score_100"))).collect() };
    let vals: Vec<f64> = cohort.iter().chain(baseline).flat_map(|i| pts(i)).map(|p| p.1).collect();
    let step = if vals.iter().copied().fold(f64::NEG_INFINITY, f64::max) - vals.iter().copied().fold(f64::INFINITY, f64::min) > 30.0 {
        10.0
    } else {
        5.0
    };
    let lo = (vals.iter().copied().fold(f64::INFINITY, f64::min) / step).floor() * step;
    let hi = (vals.iter().copied().fold(f64::NEG_INFINITY, f64::max) / step).ceil() * step;
    let (w, h, l, r, t, b) = (760.0, 400.0, 48.0, 210.0, 78.0, 46.0);
    let x = |e: f64| l + (w - l - r) * e / n_evals;
    let y = |v: f64| t + (h - t - b) * (hi - v) / (hi - lo);
    let path = |p: &[(f64, f64)]| -> String {
        let mut d = format!("M{:.1} {:.1}", x(p[0].0), y(p[0].1));
        for k in 1..p.len() {
            d += &format!(" H{:.1} V{:.1}", x(p[k].0), y(p[k].1));
        }
        d + &format!(" H{:.1}", x(n_evals))
    };
    let mut o = String::new();
    o += &format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {w} {h}\" width=\"{w}\" height=\"{h}\" role=\"img\" \
         aria-labelledby=\"t d\" font-family=\"system-ui, -apple-system, 'Segoe UI', sans-serif\">\n"
    );
    // Literal colours per class (no CSS variables, which some SVG renderers ignore), re-stepped for dark mode
    let theme = |bg: &str, ink: &str, ink2: &str, muted: &str, grid: &str, axis: &str, s1: &str, s2: &str| {
        format!(
            ".bg{{fill:{bg}}}.t1{{fill:{ink}}}.t2,.lb{{fill:{ink2}}}.tk{{fill:{muted}}}.grid{{stroke:{grid}}}\
             .axis{{stroke:{axis}}}.ctx{{stroke:{muted}}}.win{{stroke:{s1}}}.dot{{fill:{s1};stroke:{bg}}}.base{{stroke:{s2}}}"
        )
    };
    o += &format!(
        "<style>\n{}\n@media (prefers-color-scheme: dark){{{}}}\n\
         .t1{{font-size:15px;font-weight:600}}.t2,.lb{{font-size:12px}}.tk{{font-size:11px}}\
         .grid,.axis{{stroke-width:1}}.ctx{{fill:none;stroke-width:1;stroke-opacity:.55}}\
         .win{{fill:none;stroke-width:2;stroke-linejoin:round}}.base{{fill:none;stroke-width:2;stroke-dasharray:6 4}}\
         .dot{{stroke-width:2}}\n</style>\n",
        theme("#fcfcfb", "#0b0b0b", "#52514e", "#898781", "#e1e0d9", "#c3c2b7", "#2a78d6", "#eb6834"),
        theme("#1a1a19", "#f0efec", "#c3c2b7", "#898781", "#2c2c2a", "#383835", "#3987e5", "#d95926")
    );
    let wi = cohort.iter().find(|i| i.name == winner_island).unwrap();
    let wbest = pts(wi).last().unwrap().1;
    let bbest = baseline.iter().map(|i| pts(i).last().unwrap().1).fold(f64::NEG_INFINITY, f64::max);
    o += &format!(
        "<title id=\"t\">Best score found against evaluations, G5 cohort of record</title>\n<desc id=\"d\">{} evolving \
         islands and {} Sobol random-search islands, {} evaluations each. The winner's island ({winner_island}) reaches \
         {wbest:.1}; the best random-search island reaches {bbest:.1}.</desc>\n",
        cohort.len(),
        baseline.len(),
        n_evals
    );
    o += &format!("<rect class=\"bg\" width=\"{w}\" height=\"{h}\" rx=\"8\"/>\n");
    o += &format!("<text class=\"t1\" x=\"{l}\" y=\"26\">Best score found, by evaluations</text>\n");
    o += &format!(
        "<text class=\"t2\" x=\"{l}\" y=\"45\">W_ERGOLIGHT_v2, Tier 1 at h 0.4 mm · {} evolving islands · random search: \
         best of {} × {n_evals} samples</text>\n",
        cohort.len(),
        baseline.len()
    );
    // legend (one row above the plot)
    let lg = [("win", "Winner's island"), ("ctx", "Other evolving islands"), ("base", "Random search, best (control)")];
    let mut lx = l;
    for (cls, name) in lg {
        o += &format!("<line class=\"{cls}\" x1=\"{lx}\" y1=\"62\" x2=\"{}\" y2=\"62\"/>", lx + 22.0);
        o += &format!("<text class=\"lb\" x=\"{}\" y=\"66\">{name}</text>\n", lx + 28.0);
        lx += 28.0 + 7.2 * name.len() as f64 + 22.0;
    }
    // grid and axes
    let mut v = lo;
    while v <= hi + 1e-9 {
        o += &format!("<line class=\"grid\" x1=\"{l}\" x2=\"{}\" y1=\"{:.1}\" y2=\"{:.1}\"/>", w - r, y(v), y(v));
        o += &format!("<text class=\"tk\" x=\"{}\" y=\"{:.1}\" text-anchor=\"end\">{v:.0}</text>\n", l - 8.0, y(v) + 4.0);
        v += step;
    }
    let de = [25.0, 50.0, 100.0, 250.0, 500.0, 1000.0].into_iter().find(|&d| n_evals / d <= 6.0).unwrap_or(1000.0);
    let mut e = 0.0;
    while e <= n_evals + 1e-9 {
        o += &format!("<text class=\"tk\" x=\"{:.1}\" y=\"{}\" text-anchor=\"middle\">{e:.0}</text>", x(e), h - b + 18.0);
        e += de;
    }
    o += &format!("<line class=\"axis\" x1=\"{l}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n", w - r, h - b, h - b);
    o += &format!(
        "<text class=\"tk\" x=\"{:.1}\" y=\"{}\" text-anchor=\"middle\">evaluations per island</text>\n",
        x(0.5 * n_evals),
        h - 8.0
    );
    // series: context first, then the control, then the winner's island on top
    for i in cohort.iter().filter(|i| i.name != winner_island) {
        o += &format!("<path class=\"ctx\" d=\"{}\"><title>{}: {:.1}</title></path>\n", path(&pts(i)), i.name, pts(i).last().unwrap().1);
    }
    for i in baseline {
        let v = pts(i).last().unwrap().1;
        o += &format!(
            "<line class=\"base\" x1=\"{l}\" x2=\"{}\" y1=\"{:.1}\" y2=\"{:.1}\"><title>{} (random search, best of \
             {n_evals} samples): {v:.1}</title></line>\n",
            w - r,
            y(v),
            y(v),
            i.name
        );
    }
    o += &format!("<path class=\"win\" d=\"{}\"><title>{winner_island}: {wbest:.1}</title></path>\n", path(&pts(wi)));
    o += &format!(
        "<circle class=\"dot\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"4.5\"/>\n",
        x(n_evals),
        y(wbest)
    );
    // direct labels at the line ends, pushed apart so they never overlap
    let mut ctx_end: Vec<f64> = cohort.iter().filter(|i| i.name != winner_island).map(|i| pts(i).last().unwrap().1).collect();
    let ctx_med = median(&mut ctx_end);
    let mut labels = vec![
        (y(wbest), format!("{winner_island} · {wbest:.1}")),
        (y(ctx_med), format!("other islands · median {ctx_med:.1}")),
        (y(bbest), format!("random search · best {bbest:.1}")),
    ];
    labels.sort_by(|a, b| a.0.total_cmp(&b.0));
    for k in 1..labels.len() {
        if labels[k].0 < labels[k - 1].0 + 16.0 {
            labels[k].0 = labels[k - 1].0 + 16.0;
        }
    }
    for (ly, text) in labels {
        o += &format!("<text class=\"lb\" x=\"{}\" y=\"{:.1}\">{text}</text>\n", w - r + 10.0, ly + 4.0);
    }
    o + "</svg>\n"
}
