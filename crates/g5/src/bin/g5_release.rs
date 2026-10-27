//! G5 release receipt. Assembles `evidence/g5/v<version>/` from the finished cohort of record and its certification
//! runs, re-checks every link between them, and fails closed: any mismatch exits 1 before anything is written.
//!
//!   g5-release receipt <repo_root> <cohort_root> <baseline_root> <cert_dir> [shortlist=32] [grid_top=8]
//!   g5-release top <tier2_h020.jsonl> <n>     the ranking's top n design ids, comma-separated (for the grid check)
//!   g5-release plate-meshes <cohort_root> <tier2_h020.jsonl> <n> <out.jsonl> [threads=4]
//!                                             the top n designs' plate cases on the check meshes (platform of record)
//!   g5-release receipt-v6 <repo_root> <run_root> <cert_dir> [shortlist=32] [grid_top=8]
//!                                             the v6.0.0 receipt: the continuation's lineage, then the same
//!                                             certification (module `g5_release/v6.rs`)
//!   g5-release fitness <release.json> <out_dir>
//!                                             the fitness figures of a receipt: the 100-point budget (donut) and the
//!                                             points per term (ranked bars), checked to sum to the score
//!
//! Inputs (written by g5-evolve and g5-pilot; no number is typed by hand):
//! - `<cohort_root>`: `islandNN/run/{archive.jsonl, genomes_g5.csv, history.jsonl}` and `supervisor.log`;
//! - `<baseline_root>`: the same for the Sobol random-search control (same budget, not eligible to win);
//! - `<cert_dir>/eval_h040.jsonl`, `baseline_eval_h040.jsonl`: every final elite at Tier 1, h 0.4 (`g5-pilot eval`);
//! - `<cert_dir>/tier2_h020.jsonl`: the shortlist at Tier 2, h 0.2 (`g5-pilot tier2`), the ranking of record;
//! - `<cert_dir>/hconv.jsonl`: the ranking's top designs at Tier 1 on five grids (`g5-pilot hconv`);
//! - `<cert_dir>/plate_meshes.jsonl`: the ranking's top designs' plate cases on the check meshes (`g5-release plate-meshes`,
//!   on the platform of record);
//! - `<cert_dir>/tier2_linux_h020.jsonl`: the same top designs at Tier 2 on Linux x86_64 (cross-platform check);
//! - `<cert_dir>/launch.json`: the `g5-evolve` arguments of the cohort and the control, and `<cert_dir>/repro/`: one
//!   island re-run from them for its first evaluations on another platform;
//! - `<cert_dir>/assets/` (optional): release assets too large for git (the full Tier-1 re-evaluation records), listed
//!   with their sha256; `elites_h040.csv` and every island's genomes are in git, so any elite can be re-evaluated.
//!
//! Checks:
//! 1. every record and archive row carries this binary's decoder hash, and every genome row re-hashes to its id;
//! 2. `eval_h040` covers every final elite and reproduces every Tier-1 archive row bit for bit;
//! 3. the shortlist is the top `shortlist` of `eval_h040`; the grid check and the Linux run cover the ranking's top
//!    `grid_top`;
//! 4. the launch record is consistent with the runs (Sobol start = the first history entry, budget = the last, seeds
//!    by the seed rule, archive niches in range), and the re-run island reproduces the cohort's history bit for bit;
//! 5. the winner is the highest-ranked design whose h 0.2 mass agrees with h 0.15 within RS-karambit-v5 `[mass]
//!    grid_agreement_max_frac` (designs that do not are reported, never ranked), and its meshes are watertight.
//!
//! Outputs: `evidence/g5/v<version>/{release.json, winner_genome.csv, elites_h040.csv, ranking_tier2_h020.jsonl,
//! grid_check.jsonl, plate_meshes.jsonl, tier2_linux_h020.jsonl, history/islandNN.jsonl, genomes/islandNN.csv, repro/, SHA256SUMS}` (`vv/`
//! keeps its own sums), the evolution figure `docs/g5/v<version>/img/evolution.svg`, and `evidence/g5/RECORD`.
use jarambit_crescent::export::{census, volume};
use jarambit_g5::g5_genome::G5Genome;
use jarambit_g5::g5_namespace::validate_g5_design_id;
use jarambit_g5::g5_qd::{
    g5_decoder_sha256, measure_blade_edge_apex_and_angle, measure_point_planform, serration_tooth_strength, tooth_pry_per_tooth_n,
    tooth_snag_n,
};
use jarambit_g5::g5_sdf::{build_blade_profile_2d, extract_watertight_mesh_fail_closed};
use jarambit_g5::loads::{plate_blade_cases_at, G5Loads, PLATE_MAX_AREA_MM2, TIP_FE_MAX_AREA_MM2};
use jarambit_g5::{material, objective, requirements, sha256_bytes};
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[path = "g5_shared/archive.rs"]
mod archive;
#[path = "g5_release/v6.rs"]
mod v6;
const H_SEARCH: f64 = 0.4;
const H_RECORD: f64 = 0.2;
const H_FINE: f64 = 0.15;
const GRID_H: [f64; 5] = [0.4, 0.3, 0.25, 0.2, 0.15];
/// Plate meshes (max triangle area, mm2) on which a finalist's plate cases are re-solved; a case's margin of record is
/// its minimum over them (the plate's lateral SF is not monotone in the mesh, `vv/plate_mesh.json`).
const PLATE_MESH_CHECK_MM2: [f64; 3] = [0.6, 0.3, 0.15];
/// Input fields carried over from the G3 evaluator that W_ERGOLIGHT_v2 does not read (several are placeholders, such
/// as `solid_tip_sf` = 0 where G5 runs its own solid tip into `loads`). They are kept, apart, as `legacy_unscored`.
const LEGACY_UNSCORED: [&str; 21] = [
    "choil_depth_mm", "crest_r_min_mm", "dolfinx_rel_err", "fat_sf", "gci_admitted", "grip_sf_min", "groove_r_min_mm",
    "hm_spine_sf", "hm_twist_sf", "lat_rob_cert_n", "mid_depth_mm", "ring_back_cap_n", "ring_bearing_cap_n",
    "saddle_depth_mm", "seq_ratio", "solid_tip_sf", "station_err_mm", "strict_lat_n", "twist_body_sf", "twist_tip_sf",
    "twist_tipzone_strict_sf",
];

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

/// Check 4: the launch record against the runs, and the re-run island against the cohort's history.
fn verify_launch(launch: &Value, cohort: &[Island], baseline: &[Island], repro: &Path) -> Value {
    for (key, isl, init_is_budget) in [("cohort", cohort, false), ("baseline", baseline, true)] {
        let a = &launch[key];
        let (init, evals) = (f(a, "/init"), f(a, "/evals"));
        if init_is_budget != (init == evals) {
            fail(format!("launch.{key}: init {init} vs evals {evals}"));
        }
        for i in isl {
            let (first, last) = (i.history.first().unwrap(), i.history.last().unwrap());
            if f(first, "/evals") != init.min(evals) || f(last, "/evals") != evals {
                fail(format!("{}: history spans {}..{}, launch says init {init}, evals {evals}", i.name, first["evals"], last["evals"]));
            }
            for r in &i.archive {
                if f(r, "/niche_id") >= f(launch, "/archive_niches") {
                    fail(format!("{}: niche {} out of range", i.name, r["niche_id"]));
                }
            }
        }
    }
    let island = s(launch, "/repro/island");
    let mine = &cohort.iter().find(|i| i.name == island).unwrap_or_else(|| fail(format!("repro island {island}"))).history;
    let re = jsonl(&repro.join(format!("{island}_history.jsonl")));
    let keys = ["/evals", "/niches", "/n_tier2", "/qd_score", "/best_raw_score_100", "/best_mass_g"];
    let mut matched = 0;
    for e in &re {
        let Some(m) = mine.iter().find(|m| m["evals"] == e["evals"]) else { continue };
        for k in keys {
            if f(e, k).to_bits() != f(m, k).to_bits() {
                fail(format!("repro {island} at {} evals: {k} {} != {}", e["evals"], f(e, k), f(m, k)));
            }
        }
        if s(e, "/best_genome_sha256") != s(m, "/best_genome_sha256") {
            fail(format!("repro {island} at {} evals: best genome differs", e["evals"]));
        }
        matched += 1;
    }
    if matched < 2 {
        fail(format!("repro {island}: only {matched} history entries to compare"));
    }
    json!({"island": island, "platform": launch["repro"]["platform"], "evaluations": re.last().unwrap()["evals"],
           "history_entries_identical": matched})
}

/// The same island re-run on Linux x86_64 (`<island>_linux_history.jsonl`), compared with the record: maths-library
/// differences between platforms (`sin`, `exp`, `powf`) make the runs agree closely rather than bit for bit.
fn cross_platform_history(launch: &Value, cohort: &[Island], repro: &Path) -> Value {
    let island = s(launch, "/repro/island");
    let mine = &cohort.iter().find(|i| i.name == island).unwrap().history;
    let re = jsonl(&repro.join(format!("{island}_linux_history.jsonl")));
    let (mut n, mut same_genome_through, mut max_rel, mut identical) = (0, Value::Null, 0.0_f64, 0);
    for e in &re {
        let Some(m) = mine.iter().find(|m| m["evals"] == e["evals"]) else { continue };
        n += 1;
        let (x, y) = (f(e, "/best_raw_score_100"), f(m, "/best_raw_score_100"));
        identical += usize::from(x.to_bits() == y.to_bits());
        if s(e, "/best_genome_sha256") == s(m, "/best_genome_sha256") {
            same_genome_through = e["evals"].clone();
            max_rel = max_rel.max(((x - y) / y).abs());
        } else {
            break;
        }
    }
    json!({"island": island, "platform": "Linux x86_64 (xbabe2)", "history_entries_compared": n,
           "best_score_bit_identical_entries": identical, "same_best_genome_through_evals": same_genome_through,
           "max_rel_diff_best_score_while_same_genome": max_rel})
}

/// Check 5 (B1): a finalist's plate cases on every mesh of PLATE_MESH_CHECK_MM2, from `<cert>/plate_meshes.jsonl`
/// (`g5-release plate-meshes`, run on the platform of record). The production mesh must reproduce the Tier-2 record bit
/// for bit. Returns (per-mesh rows, minimum margin per case, worst).
fn plate_mesh_check(id: &str, rec: &Value, rows_in: &[Value]) -> (Value, Map<String, Value>, f64) {
    let mut rows = Vec::new();
    let mut min_m: BTreeMap<&str, f64> = BTreeMap::new();
    for &a in &PLATE_MESH_CHECK_MM2 {
        let r = rows_in
            .iter()
            .find(|r| s(r, "/design_id") == id && f(r, "/max_area_mm2") == a)
            .unwrap_or_else(|| fail(format!("plate_meshes: {id} at {a} mm2 missing")));
        if r["solved"] != json!(true) {
            fail(format!("{id}: plate solve failed at {a} mm2"));
        }
        if s(r, "/decoder_sha256") != g5_decoder_sha256() {
            fail(format!("{id}: plate_meshes row from another decoder"));
        }
        if a == PLATE_MAX_AREA_MM2 && r["loads"] != rec["loads"] {
            fail(format!("{id}: the {a} mm2 plate does not reproduce the Tier-2 record (run plate-meshes on the platform of record)"));
        }
        let l: G5Loads = serde_json::from_value(r["loads"].clone()).unwrap_or_else(|e| fail(format!("{id} loads: {e}")));
        let mut mm = Map::new();
        for (k, v) in objective::margins(&l) {
            let v = if v.is_nan() { 0.0 } else { v };
            let e = min_m.entry(k).or_insert(f64::INFINITY);
            *e = e.min(v);
            mm.insert(k.to_string(), num(v));
        }
        rows.push(json!({"max_area_mm2": a, "margins": mm, "lateral_limit_sf": num(l.lateral_limit_sf),
                         "lateral_ultimate_sf": num(l.lateral_ultimate_sf), "twist_limit_sf": num(l.twist_limit_sf),
                         "pull_sf": num(l.pull_sf), "fatigue_sf": num(l.fatigue_sf)}));
    }
    let worst = min_m.values().copied().fold(f64::INFINITY, f64::min);
    (Value::Array(rows), min_m.into_iter().map(|(k, v)| (k.to_string(), num(v))).collect(), worst)
}

/// `g5-release plate-meshes`: the ranking's top `n` designs with their plate cases re-solved on every mesh of
/// PLATE_MESH_CHECK_MM2, on top of their Tier-2 loads, one JSON line per design and mesh.
fn plate_meshes(cohort_root: &Path, t2_path: &Path, n: usize, out: &Path) {
    use rayon::prelude::*;
    let hash = g5_decoder_sha256();
    let isl = islands(cohort_root);
    let genomes = verify_root(&isl, &hash);
    let t2 = jsonl(t2_path);
    let jobs: Vec<(&Value, f64)> = rank(&t2).into_iter().take(n).flat_map(|l| PLATE_MESH_CHECK_MM2.iter().map(move |&a| (l, a))).collect();
    let lines: Vec<String> = jobs
        .par_iter()
        .map(|&(l, a)| {
            let id = s(l, "/design_id");
            let g = &genomes.get(id).unwrap_or_else(|| fail(format!("{id}: no genome"))).1;
            let p = g.decode();
            let b = build_blade_profile_2d(&p);
            let mut loads: G5Loads =
                serde_json::from_value(l["record"]["loads"].clone()).unwrap_or_else(|e| fail(format!("{id} loads: {e}")));
            let solved = plate_blade_cases_at(&p, &b, &mut loads, a);
            json!({"design_id": id, "max_area_mm2": a, "solved": solved, "decoder_sha256": hash, "loads": loads}).to_string()
        })
        .collect();
    fs::write(out, lines.join("\n") + "\n").unwrap_or_else(|e| fail(format!("{}: {e}", out.display())));
    eprintln!("wrote {} rows to {}", lines.len(), out.display());
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
                     \x20      g5-release top <tier2_h020.jsonl> <n>\n\
                     \x20      g5-release plate-meshes <cohort_root> <tier2_h020.jsonl> <n> <out.jsonl> [threads=4]\n\
                     \x20      g5-release fitness <release.json> <out_dir>\n\
                     \x20      g5-release receipt-v6 <repo_root> <run_root> <cert_dir> [shortlist=32] [grid_top=8]";

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
        Some("receipt-v6") if a.len() >= 5 => v6::receipt_v6(&a[2..]),
        Some("plate-meshes") if a.len() >= 6 => {
            let threads: usize = a.get(6).map_or(4, |v| v.parse().unwrap_or_else(|_| fail(USAGE)));
            rayon::ThreadPoolBuilder::new().num_threads(threads).build_global().ok();
            plate_meshes(Path::new(&a[2]), Path::new(&a[3]), a[4].parse().unwrap_or_else(|_| fail(USAGE)), Path::new(&a[5]));
        }
        Some("fitness") if a.len() == 4 => {
            let b = write_fitness_figures(&serde_json::from_str(&read(Path::new(&a[2]))).unwrap_or_else(|e| fail(e)), Path::new(&a[3]));
            println!("{}", serde_json::to_string_pretty(&fitness_budget_json(&b)).unwrap());
        }
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
    let launch = serde_json::from_str::<Value>(&read(&cert.join("launch.json"))).unwrap_or_else(|e| fail(format!("launch.json: {e}")));
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
    let rank_of = |id: &str| ranking.iter().position(|l| s(l, "/design_id") == id).unwrap() + 1;

    // the search leader (best Tier-1 score at h 0.4) and what Tier 2 found
    let leader_id = ev[0].1;
    let leader_t1 = &eval.iter().find(|l| s(l, "/design_id") == leader_id).unwrap()["record"];
    let leader_t2 = &ranking[rank_of(leader_id) - 1]["record"];
    let (lm, lmu) = margins_json(leader_t2);
    let binding = lm.as_object().unwrap().iter().filter(|(_, v)| v.as_f64().is_some_and(|x| x < 1.0))
        .map(|(k, v)| json!({"case": k, "margin": v})).collect::<Vec<_>>();
    let search_leader = json!({"design_id": leader_id, "tier1_h04_score_100": leader_t1["score"]["score_100"],
        "tier2_h02_score_100": leader_t2["score"]["score_100"], "tier2_rank": rank_of(leader_id),
        "tier2_worst_margin": num(lmu), "tier2_cases_below_requirement": binding,
        "tip_sf_sections_h04": {"axial": leader_t1["loads"]["tip_axial_sf"], "lateral": leader_t1["loads"]["tip_lateral_sf"],
                                "twist": leader_t1["loads"]["tip_twist_sf"]},
        "tip_sf_solid_h02": {"axial": leader_t2["loads"]["tip_axial_sf"], "lateral": leader_t2["loads"]["tip_lateral_sf"],
                             "twist": leader_t2["loads"]["tip_twist_sf"]}});

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
    // B1: in rank order, the first design that passes the grid check and meets every load case on every plate mesh
    let plate_rows = jsonl(&cert.join("plate_meshes.jsonl"));
    let mut mesh_rows = Vec::new();
    let mut chosen = None;
    for r in grid_rows.iter().filter(|r| r["pass"] == json!(true)) {
        let id = s(r, "/design_id");
        let (per, min_m, worst) = plate_mesh_check(id, &ranking[rank_of(id) - 1]["record"], &plate_rows);
        mesh_rows.push(json!({"design_id": id, "rank": rank_of(id), "worst_margin_over_meshes": num(worst),
                              "meets_every_load_case_on_every_mesh": worst >= 1.0, "meshes": per}));
        if worst >= 1.0 {
            chosen = Some((id.to_string(), min_m, worst));
            break;
        }
    }
    let (w, margins_min, mu_min) = chosen.unwrap_or_else(|| {
        fail(format!("no design in the top {grid_top} passes the grid check and meets every load case on every plate mesh"))
    });
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
    let mut lin_rows = Vec::new();
    for l in &lin {
        let id = s(l, "/design_id");
        let arm = &ranking[rank_of(id) - 1]["record"];
        if s(l, "/record/decoder_sha256") != hash || f(l, "/h") != H_RECORD {
            fail(format!("{id}: Linux record has another decoder or grid"));
        }
        let mut rel = [0.0; 2];
        for (k, p) in ["/score/raw_score_100", "/inputs/mass_g"].iter().enumerate() {
            let (x, y) = (f(&l["record"], p), f(arm, p));
            lin_identical &= x.to_bits() == y.to_bits();
            rel[k] = (x - y) / y;
            lin_max_rel = lin_max_rel.max(rel[k].abs());
        }
        lin_rows.push(json!({"design_id": id, "score_100_linux": l["record"]["score"]["score_100"], "score_rel_diff": rel[0],
                             "mass_rel_diff": rel[1]}));
    }
    lin_rows.sort_by(|x, y| f(y, "/score_100_linux").total_cmp(&f(x, "/score_100_linux")));

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
    // B2: the inputs W_ERGOLIGHT_v2 reads, and apart from them the G3-era fields it does not read
    let all_inputs = inp.as_object().unwrap_or_else(|| fail("inputs"));
    let legacy: Map<String, Value> = LEGACY_UNSCORED
        .iter()
        .map(|&k| (k.to_string(), all_inputs.get(k).cloned().unwrap_or_else(|| fail(format!("inputs.{k} missing")))))
        .collect();
    let scored_inputs: Map<String, Value> =
        all_inputs.iter().filter(|(k, _)| !LEGACY_UNSCORED.contains(&k.as_str())).map(|(k, v)| (k.clone(), v.clone())).collect();
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
    let host = f(&launch, "/cohort/host");
    for (name, seed) in seeds.iter().chain(base_seeds.iter()) {
        let n: f64 = name.trim_start_matches("island").parse().unwrap_or_else(|_| fail(name));
        if *seed as f64 != host * 1000.0 + n {
            fail(format!("{name}: seed {seed} is not host*1000+island"));
        }
    }
    let repro = verify_launch(&launch, &cohort, &baseline, &cert.join("repro"));
    let cross_evolution = cross_platform_history(&launch, &cohort, &cert.join("repro"));
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
    for d in [ev_dir.join("history"), ev_dir.join("genomes"), ev_dir.join("repro"), img_dir.clone()] {
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
    fs::copy(cert.join("plate_meshes.jsonl"), ev_dir.join("plate_meshes.jsonl")).unwrap();
    fs::copy(cert.join("tier2_linux_h020.jsonl"), ev_dir.join("tier2_linux_h020.jsonl")).unwrap();
    let ri = s(&launch, "/repro/island");
    fs::copy(cert.join("launch.json"), ev_dir.join("repro/launch.json")).unwrap();
    for suffix in ["history", "linux_history"] {
        let name = format!("repro/{ri}_{suffix}.jsonl");
        fs::copy(cert.join(&name), ev_dir.join(&name)).unwrap();
    }
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
    // written only when it changes, so its mtime does not trigger a rebuild of derived files (the paper's evolution.pdf)
    let svg = evolution_svg(&cohort, &baseline, island, &win_sha, n_evals);
    if fs::read_to_string(repo.join(&figure)).ok().as_deref() != Some(svg.as_str()) {
        fs::write(repo.join(&figure), svg).unwrap();
    }

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
            "h_mm": H_SEARCH, "launch": launch, "reproduced": repro,
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
            "linux_check": {"designs": grid_top, "bit_identical": lin_identical, "max_rel_diff": lin_max_rel,
                            "winner_rank_linux": lin_rows.iter().position(|r| r["design_id"] == json!(w)).map(|k| k + 1),
                            "rows": lin_rows, "evolution": cross_evolution},
            "search_leader": search_leader,
        },
        "grid_check": {
            "h_mm": GRID_H, "requirement_max_frac": max_frac, "evaluation": format!("Tier 1 of the ranking's top {grid_top}"),
            "rule": "the winner is the highest-ranked design whose h 0.2 mass agrees with h 0.15; others are reported, never ranked",
            "rows": grid_rows,
        },
        "rank_of_record": rank_of(&w),
        "score": score_json(&rec["score"]),
        "margins": margins, "margins_min_over_plate_meshes": margins_min, "worst_margin": num(mu_min),
        "worst_margin_production_mesh": num(mu), "meets_every_load_case": mu_min >= 1.0,
        "plate_mesh_check": {"max_area_mm2": PLATE_MESH_CHECK_MM2, "production_max_area_mm2": PLATE_MAX_AREA_MM2,
            "rule": "in rank order, the first design passing the grid check whose every load case meets its requirement on every plate mesh; a case's margin of record is its minimum over the meshes",
            "rows": mesh_rows},
        "loads": rec["loads"], "inputs": scored_inputs, "legacy_unscored": legacy, "descriptors": rec["descriptors"],
        "am": rec["am_connectivity"],
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
                          "mass_g": rec["inputs"]["mass_g"], "worst_margin_over_plate_meshes": num(mu_min),
                          "worst_margin_production_mesh": num(mu), "files": rel.len()}));
}

/// The evolution figure: each evolving island's best score against evaluations, as steps (the best never falls).
/// The winner's island is series 1 (blue) and the other cohort islands thin neutral context. A random-search island
/// logs once, at the end of its samples, so the control is drawn as a reference level: its best score after the full
/// budget (series 2, orange, dashed). A ring marks where the winner of record first became its island's best: the
/// line's end is the island's best on the search grid, which Tier 2 can re-rank. Colours are per-class CSS with a dark-mode set, both validated for colour-vision deficiency
/// (adjacent CVD dE 24.7 light / 26.8 dark); identity is never colour alone (legend, direct labels, dash).
fn evolution_svg(cohort: &[Island], baseline: &[Island], winner_island: &str, winner_sha: &str, n_evals: f64) -> String {
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
             .axis{{stroke:{axis}}}.ctx{{stroke:{muted}}}.win{{stroke:{s1}}}.ring{{fill:{bg};stroke:{s1}}}.base{{stroke:{s2}}}"
        )
    };
    o += &format!(
        "<style>\n{}\n@media (prefers-color-scheme: dark){{{}}}\n\
         .t1{{font-size:15px;font-weight:600}}.t2,.lb{{font-size:12px}}.tk{{font-size:11px}}\
         .grid,.axis{{stroke-width:1}}.ctx{{fill:none;stroke-width:1;stroke-opacity:.55}}\
         .win{{fill:none;stroke-width:2;stroke-linejoin:round}}.base{{fill:none;stroke-width:2;stroke-dasharray:6 4}}\
         .ring{{stroke-width:2.5}}\n</style>\n",
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
    let found = wi.history.iter().find(|e| s(e, "/best_genome_sha256") == winner_sha).map(|e| (f(e, "/evals"), f(e, "/best_score_100")));
    if let Some((fe, fs)) = found {
        o += &format!("<circle class=\"ring\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"5.5\"><title>winner of record first \
                       found at {fe:.0} evaluations ({fs:.1} on the search grid)</title></circle>\n", x(fe), y(fs));
        o += &format!("<text class=\"lb\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">winner of record found ({fe:.0})</text>\n",
                      x(fe) - 10.0, y(fs) - 10.0);
    }
    // direct labels at the line ends, pushed apart so they never overlap
    let mut ctx_end: Vec<f64> = cohort.iter().filter(|i| i.name != winner_island).map(|i| pts(i).last().unwrap().1).collect();
    let ctx_med = median(&mut ctx_end);
    let mut labels = vec![
        (y(wbest), format!("{winner_island} (winner's) · {wbest:.1}")),
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

/// The groups of the fitness figures, in their fixed colour order (light, dark: the validated categorical slots 1–4).
const FITNESS_GROUPS: [(&str, &str, &str, &str); 4] = [
    ("structural", "Structural safety", "#2a78d6", "#3987e5"),
    ("mass", "Mass", "#eb6834", "#d95926"),
    ("cutting", "Cutting geometry", "#1baf7a", "#199e70"),
    ("build", "Build and form", "#eda100", "#c98500"),
];

/// A W_ERGOLIGHT_v2 score as a 100-point budget: 100 = earned + lost to the multipliers + unmet credit, with
/// earned = Σ points = 100·E·W·base, lost to the multipliers = 100·base·(1 − E·W) and unmet credit = 100·(1 − base).
struct FitnessBudget {
    /// (term, readable name, group, points earned, most it could earn = 100·weight)
    terms: Vec<(String, &'static str, &'static str, f64, f64)>,
    score: f64,
    e: f64,
    w: f64,
    base: f64,
    lost_multipliers: f64,
    unmet: f64,
    mass_s: f64,
    /// hand factors below full credit, as (name, e, e^β), lowest e^β first
    hand_losses: Vec<(String, f64, f64)>,
}

impl FitnessBudget {
    fn group_points(&self, g: &str) -> f64 {
        self.terms.iter().filter(|t| t.2 == g).map(|t| t.3).sum()
    }
}

/// "Edge angle" -> "edge angle", but "AM drainage" stays.
fn lower_first(name: &str) -> String {
    let mut c = name.chars();
    match (c.next(), c.next()) {
        (Some(a), Some(b)) if !b.is_uppercase() => a.to_lowercase().chain(name.chars().skip(1)).collect(),
        _ => name.to_string(),
    }
}

fn term_name(t: &str) -> &'static str {
    match t {
        "am_build" => "AM drainage",
        "buckling" => "Buckling",
        "edge" => "Edge angle",
        "fatigue" => "Fatigue",
        "form" => "Form",
        "grip_squeeze" => "Grip squeeze",
        "guard_push" => "Guard push",
        "lateral_limit" => "Lateral, limit",
        "lateral_ultimate" => "Lateral, ultimate",
        "mass" => "Mass",
        "point" => "Point",
        "pull" => "Pull",
        "ring_any_direction" => "Ring, any direction",
        "ring_back" => "Ring, back",
        "ring_bearing" => "Ring, bearing",
        "sequence" => "Failure sequence",
        "serration" => "Serration",
        "teeth" => "Teeth",
        "tip_axial" => "Tip, axial",
        "tip_lateral" => "Tip, lateral",
        "tip_twist" => "Tip, twist",
        "twist_limit" => "Twist, limit",
        _ => fail(format!("fitness figure: no name for score term '{t}'")),
    }
}

/// The budget of a receipt's score, checked: every weighted term in exactly one group (the structural group is the
/// objective's own margin list), the points summing to the score, the score equal to 100·E·W·base, and the budget to 100.
fn fitness_budget(receipt: &Value) -> FitnessBudget {
    let sc = &receipt["score"];
    let (e, w, base, score) = (f(sc, "/E"), f(sc, "/worst_margin_W"), f(sc, "/base"), f(sc, "/raw_score_100"));
    let loads: G5Loads = serde_json::from_value(receipt["loads"].clone()).unwrap_or_else(|x| fail(format!("receipt loads: {x}")));
    let structural: BTreeSet<&str> = objective::margins(&loads).iter().map(|m| m.0).collect();
    let group = |t: &str| -> Option<&'static str> {
        if structural.contains(t) {
            Some("structural")
        } else {
            match t {
                "mass" => Some("mass"),
                "edge" | "point" | "serration" => Some("cutting"),
                "am_build" | "form" => Some("build"),
                "worst_margin" => None, // weight 0: it acts as the multiplier W
                _ => fail(format!("fitness figure: score term '{t}' is in no group")),
            }
        }
    };
    let Some(Value::Object(tm)) = sc.get("terms") else { fail("receipt score: no terms") };
    let mut terms = Vec::new();
    let mut base_sum = 0.0;
    for (t, v) in tm {
        let (pts, wt, s_) = (f(v, "/points"), f(v, "/weight"), f(v, "/s"));
        base_sum += wt * s_;
        match group(t) {
            Some(g) => terms.push((t.clone(), term_name(t), g, pts, 100.0 * wt)),
            None if wt == 0.0 && pts == 0.0 => {}
            None => fail(format!("fitness figure: '{t}' has weight {wt} and {pts} points")),
        }
    }
    if terms.iter().filter(|t| t.2 == "structural").count() != structural.len() {
        fail("fitness figure: the score does not carry every structural margin term");
    }
    let earned: f64 = terms.iter().map(|t| t.3).sum();
    let close = |a: f64, b: f64| (a - b).abs() <= 1e-9 * a.abs().max(1.0);
    if !close(earned, score) || !close(100.0 * e * w * base, score) || !close(base_sum, base) {
        fail(format!("fitness figure: points {earned} / 100·E·W·base {} / Σ w·s {base_sum} do not match score {score}", 100.0 * e * w * base));
    }
    let (lost_multipliers, unmet) = (100.0 * base * (1.0 - e * w), 100.0 * (1.0 - base));
    if !close(earned + lost_multipliers + unmet, 100.0) {
        fail("fitness figure: the budget does not sum to 100");
    }
    let Some(Value::Object(hf)) = sc.get("hand_factors") else { fail("receipt score: no hand_factors") };
    let mut hand_losses: Vec<(String, f64, f64)> = hf
        .iter()
        .map(|(k, v)| (k.replace('_', " "), f(v, "/e"), f(v, "/e").powf(f(v, "/beta"))))
        .filter(|h| h.1 < 1.0)
        .collect();
    hand_losses.sort_by(|a, b| a.2.total_cmp(&b.2));
    terms.sort_by(|a, b| b.3.total_cmp(&a.3).then_with(|| a.0.cmp(&b.0)));
    let mass_s = f(sc, "/terms/mass/s");
    FitnessBudget { terms, score, e, w, base, lost_multipliers, unmet, mass_s, hand_losses }
}

/// The SVG `<style>` of the fitness figures: literal colours per class, re-stepped for dark mode, as in `evolution_svg`.
fn fitness_style() -> String {
    let theme = |dark: bool| {
        let (bg, ink, ink2, muted, track, lossm, lossu) = if dark {
            ("#1a1a19", "#f0efec", "#c3c2b7", "#898781", "#2c2c2a", "#898781", "#383835")
        } else {
            ("#fcfcfb", "#0b0b0b", "#52514e", "#898781", "#e1e0d9", "#898781", "#c3c2b7")
        };
        let mut c = format!(
            ".bg{{fill:{bg}}}.t1,.v{{fill:{ink}}}.t2,.lb{{fill:{ink2}}}.tk{{fill:{muted}}}.trk{{fill:{track}}}\
             .sl{{stroke:{bg}}}.lm{{fill:{lossm}}}.lu{{fill:{lossu}}}"
        );
        for (k, _, light, dk) in FITNESS_GROUPS {
            c += &format!(".g-{k}{{fill:{}}}", if dark { dk } else { light });
        }
        c
    };
    format!(
        "<style>\n{}\n@media (prefers-color-scheme: dark){{{}}}\n\
         .t1{{font-size:15px;font-weight:600}}.t2,.lb{{font-size:12px}}.tk{{font-size:11px}}.v{{font-size:12px;font-weight:600}}\
         .big{{font-size:34px;font-weight:600}}.sl{{stroke-width:2;stroke-linejoin:round}}\n</style>\n",
        theme(false),
        theme(true)
    )
}

fn svg_open(w: f64, h: f64, title: &str, desc: &str) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {w} {h}\" width=\"{w}\" height=\"{h}\" role=\"img\" \
         aria-labelledby=\"t d\" font-family=\"system-ui, -apple-system, 'Segoe UI', sans-serif\">\n{}<title id=\"t\">{title}</title>\n\
         <desc id=\"d\">{desc}</desc>\n<rect class=\"bg\" width=\"{w}\" height=\"{h}\" rx=\"8\"/>\n",
        fitness_style()
    )
}

/// Donut of the 100-point budget: the four earned groups in their colours, then the two losses in neutral inks.
fn fitness_budget_svg(b: &FitnessBudget, design: &str) -> String {
    let (w, h, cx, cy, ro, ri) = (760.0, 400.0, 196.0, 236.0, 136.0, 86.0);
    let hand = match b.hand_losses.first() {
        Some((n, e, _)) => format!("E = {:.3} (hand: {n} e = {e:.3})", b.e),
        None => format!("E = {:.3} (hand)", b.e),
    };
    let mut slices: Vec<(String, String, f64, String)> = FITNESS_GROUPS
        .iter()
        .map(|(k, name, _, _)| {
            let detail = match *k {
                "structural" => format!("{} load-case margins", b.terms.iter().filter(|t| t.2 == "structural").count()),
                "mass" => format!("mass credit s = {:.3}", b.mass_s),
                _ => b.terms.iter().filter(|t| t.2 == *k).map(|t| lower_first(t.1)).collect::<Vec<_>>().join(", "),
            };
            (format!("g-{k}"), name.to_string(), b.group_points(k), detail)
        })
        .collect();
    slices.push(("lm".into(), "Lost to the multipliers".into(), b.lost_multipliers, format!("{hand} · W = {:.3}", b.w)));
    slices.push(("lu".into(), "Unmet credit".into(), b.unmet, format!("base = {:.3}: terms short of full credit", b.base)));
    let mut o = svg_open(
        w,
        h,
        "Where the 100 points of the score go",
        &format!(
            "W_ERGOLIGHT_v2 score of {design}: {:.1} of 100 points earned. {}",
            b.score,
            slices.iter().map(|s| format!("{} {:.1}", s.1, s.2)).collect::<Vec<_>>().join("; ")
        ),
    );
    o += "<text class=\"t1\" x=\"24\" y=\"30\">Where the 100 points of the score go</text>\n";
    o += &format!(
        "<text class=\"t2\" x=\"24\" y=\"50\">{design} · W_ERGOLIGHT_v2 = 100 · E · W · base = {:.1} earned</text>\n",
        b.score
    );
    // slices clockwise from 12 o'clock, with a 2 px surface gap
    let pt = |a: f64, r: f64| (cx + r * a.sin(), cy - r * a.cos());
    let mut a0 = 0.0;
    for (cls, name, v, detail) in &slices {
        if *v <= 0.0 {
            continue;
        }
        let a1 = a0 + std::f64::consts::TAU * v / 100.0;
        let large = if a1 - a0 > std::f64::consts::PI { 1 } else { 0 };
        let ((x0, y0), (x1, y1), (x2, y2), (x3, y3)) = (pt(a0, ro), pt(a1, ro), pt(a1, ri), pt(a0, ri));
        o += &format!(
            "<path class=\"{cls} sl\" d=\"M{x0:.2} {y0:.2} A{ro} {ro} 0 {large} 1 {x1:.2} {y1:.2} L{x2:.2} {y2:.2} \
             A{ri} {ri} 0 {large} 0 {x3:.2} {y3:.2} Z\"><title>{name}: {v:.2} points ({detail})</title></path>\n"
        );
        a0 = a1;
    }
    o += &format!("<text class=\"t1 big\" x=\"{cx}\" y=\"{}\" text-anchor=\"middle\">{:.1}</text>\n", cy + 6.0, b.score);
    o += &format!("<text class=\"tk\" x=\"{cx}\" y=\"{}\" text-anchor=\"middle\">of 100 points earned</text>\n", cy + 26.0);
    // legend with values (the direct labels): swatch, name, value, detail
    let (lx, vx) = (380.0, 736.0);
    for (k, (cls, name, v, detail)) in slices.iter().enumerate() {
        let y = 102.0 + 46.0 * k as f64;
        o += &format!("<rect class=\"{cls}\" x=\"{lx}\" y=\"{}\" width=\"12\" height=\"12\" rx=\"3\"/>", y - 10.0);
        o += &format!("<text class=\"t2\" x=\"{}\" y=\"{y}\">{name}</text>", lx + 20.0);
        o += &format!("<text class=\"v\" x=\"{vx}\" y=\"{y}\" text-anchor=\"end\">{v:.1}</text>\n");
        o += &format!("<text class=\"tk\" x=\"{}\" y=\"{}\">{detail}</text>\n", lx + 20.0, y + 16.0);
    }
    o += &format!(
        "<text class=\"tk\" x=\"24\" y=\"{}\">Earned = Σ term points; lost to the multipliers = 100 · base · (1 − E · W); \
         unmet credit = 100 · (1 − base). The three sum to 100.</text>\n",
        h - 14.0
    );
    o + "</svg>\n"
}

/// Bars of every weighted term: points earned (bar, its group's colour) against the most it could earn (track), grouped in
/// a fixed order (mass, structural, cutting, build) and ranked within each group. The group order keeps every change of
/// colour between neighbouring bars on a validated adjacent pair of the palette; every bar carries its name and value.
fn fitness_terms_svg(b: &FitnessBudget, design: &str) -> String {
    const ORDER: [&str; 4] = ["mass", "structural", "cutting", "build"];
    let mut terms = b.terms.clone();
    terms.sort_by(|x, y| {
        let gi = |g: &str| ORDER.iter().position(|o| *o == g).unwrap_or(ORDER.len());
        gi(x.2).cmp(&gi(y.2)).then_with(|| y.3.total_cmp(&x.3)).then_with(|| x.0.cmp(&y.0))
    });
    let (lx, x0, x1, top, pitch) = (24.0, 168.0, 640.0, 96.0, 19.0);
    let h = top + pitch * terms.len() as f64 + 40.0;
    let w = 760.0;
    let vmax = terms.iter().map(|t| t.4).fold(0.0, f64::max);
    let x = |v: f64| x0 + (x1 - x0) * v / vmax;
    let mut o = svg_open(
        w,
        h,
        "Points per score term",
        &format!(
            "{design}: points earned by each of the {} weighted terms of W_ERGOLIGHT_v2, against the most each could earn. {}",
            terms.len(),
            terms.iter().map(|t| format!("{} {:.2} of {:.1}", t.1, t.3, t.4)).collect::<Vec<_>>().join("; ")
        ),
    );
    o += &format!("<text class=\"t1\" x=\"{lx}\" y=\"30\">Points per score term: earned (bar) and the most it could earn (track)</text>\n");
    o += &format!(
        "<text class=\"t2\" x=\"{lx}\" y=\"50\">{design} · earned = 100 · E · W · weight · s; the track is 100 · weight (full \
         credit, E = W = 1)</text>\n"
    );
    let mut gx = lx;
    for k in ORDER {
        let name = FITNESS_GROUPS.iter().find(|g| g.0 == k).map(|g| g.1).unwrap();
        o += &format!("<rect class=\"g-{k}\" x=\"{gx}\" y=\"62\" width=\"12\" height=\"12\" rx=\"3\"/>");
        o += &format!("<text class=\"lb\" x=\"{}\" y=\"72\">{name}</text>\n", gx + 18.0);
        gx += 18.0 + 7.0 * name.len() as f64 + 24.0;
    }
    for (k, (t, name, g, pts, most)) in terms.iter().enumerate() {
        let y = top + pitch * k as f64;
        o += &format!("<text class=\"lb\" x=\"{}\" y=\"{:.1}\" text-anchor=\"end\">{name}</text>", x0 - 10.0, y + 9.0);
        o += &format!("<rect class=\"trk\" x=\"{x0}\" y=\"{y:.1}\" width=\"{:.2}\" height=\"12\" rx=\"3\"/>", x(*most) - x0);
        o += &format!(
            "<rect class=\"g-{g}\" x=\"{x0}\" y=\"{y:.1}\" width=\"{:.2}\" height=\"12\" rx=\"3\"><title>{name} ({t}): {pts:.3} of \
             {most:.1} points</title></rect>",
            (x(*pts) - x0).max(0.0)
        );
        o += &format!("<text class=\"tk\" x=\"{:.1}\" y=\"{:.1}\">{pts:.2} / {most:.1}</text>\n", x(*most) + 8.0, y + 10.0);
    }
    o += &format!(
        "<text class=\"tk\" x=\"{lx}\" y=\"{}\">Each bar falls short of its track by the multipliers (E = {:.3}, W = {:.3}) \
         and by its own credit s. The bars sum to the score, {:.1}.</text>\n",
        h - 14.0,
        b.e,
        b.w,
        b.score
    );
    o + "</svg>\n"
}

/// Write the two fitness figures for a receipt (only when they change, as for the evolution figure).
fn write_fitness_figures(receipt: &Value, out_dir: &Path) -> FitnessBudget {
    let b = fitness_budget(receipt);
    let design = s(receipt, "/design_id");
    fs::create_dir_all(out_dir).unwrap_or_else(|e| fail(format!("{}: {e}", out_dir.display())));
    for (name, svg) in [("fitness_budget.svg", fitness_budget_svg(&b, design)), ("fitness_terms.svg", fitness_terms_svg(&b, design))] {
        let p = out_dir.join(name);
        if fs::read_to_string(&p).ok().as_deref() != Some(svg.as_str()) {
            fs::write(&p, svg).unwrap_or_else(|e| fail(format!("{}: {e}", p.display())));
        }
    }
    b
}

fn fitness_budget_json(b: &FitnessBudget) -> Value {
    let mut groups = Map::new();
    for (k, _, _, _) in FITNESS_GROUPS {
        groups.insert(k.to_string(), json!(b.group_points(k)));
    }
    json!({"earned": b.score, "groups": groups, "lost_to_multipliers": b.lost_multipliers, "unmet_credit": b.unmet,
           "E": b.e, "worst_margin_W": b.w, "base": b.base,
           "rule": "100 = earned (sum of term points = 100*E*W*base) + 100*base*(1-E*W) + 100*(1-base)"})
}
