//! `g5-release receipt-v6`: the v6.0.0 receipt. v6 continues the frozen G5.0 cohort of v5.0.0 on one platform of
//! record (linux/amd64: one static x86_64 binary, native on the Linux hosts and in a Rosetta container on the Mac):
//! the v5 elites re-evaluated, then asynchronous rounds of warm-started islands whose archives the hub merges into a
//! pool lineage, beside fresh-start control islands that are never merged.
//!
//!   g5-release receipt-v6 <repo_root> <run_root> <cert_dir> [shortlist=32] [grid_top=8]
//!
//! `<run_root>` (the hub's copy): `v5_elites/islandNN.csv`, `epoch00/{reeval,pool}`, `pools/pNN`,
//! `epochNN/xbabeH/islandIII/` and `control/epochNN/xbabeH/islandIII/` (archive.jsonl, genomes_g5.csv, history.jsonl,
//! launch.json) with each host's `supervisor.log`. `<cert_dir>`: `binaries.json` (sha256 and source commit of the
//! static binaries), `final/island_v6pool/run/` (the final pool, as `g5-pilot` reads it), `eval_h040.jsonl`,
//! `tier2_h020.jsonl`, `hconv.jsonl`, `plate_meshes.jsonl`, `repro/` (one island re-run on another host) and `v5/` (the
//! v5.0.0 winner through the same Tier 2 and plate meshes on this platform).
//!
//! Checks (any failure exits 1 before anything is written):
//! 1. the v5 elite genomes are the committed v5.0.0 files; their re-evaluation covers exactly them; every row of every
//!    archive carries this decoder hash, re-hashes to its id and re-scores bit for bit (W_ERGOLIGHT_v2);
//! 2. every pool is the merge of its recorded sources, recomputed here byte for byte; the pools form one chain from the
//!    re-evaluation; every warm island started from a recorded pool (sha256) and is merged exactly once, into a later
//!    pool; the control islands start fresh and are never merged; launch records follow the seed rule and one binary;
//! 3. `eval_h040` covers the final pool and reproduces every Tier-1 row bit for bit; the shortlist, grid check and
//!    plate meshes as in v5.0.0 (B1); the winner is the first ranked design passing the grid check that meets every
//!    load case on every plate mesh;
//! 4. the re-run island reproduces its history bit for bit on another host;
//! 5. the v5.0.0 winner's comparison record is its committed genome, at Tier 2 h 0.2 on this platform.
use super::archive;
use super::*;

/// One v6 island: where it ran and its launch record.
pub struct V6Island {
    pub key: String,
    pub host: u32,
    pub round: u32,
    pub island: u32,
    pub launch: Value,
    pub isl: Island,
}

fn num_after(s: &str, prefix: &str) -> u32 {
    s.strip_prefix(prefix).and_then(|x| x.parse().ok()).unwrap_or_else(|| fail(format!("bad directory name {s}")))
}

fn subdirs(p: &Path, prefix: &str) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = fs::read_dir(p)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|q| q.is_dir() && q.file_name().is_some_and(|n| n.to_string_lossy().starts_with(prefix)))
        .collect();
    v.sort();
    v
}

/// Every island under `<root>/epochNN/xbabeH/islandIII` (round NN >= 1), keyed `<prefix>epochNN/xbabeH/islandIII`.
fn load_islands(root: &Path, prefix: &str) -> Vec<V6Island> {
    let mut out = Vec::new();
    for e in subdirs(root, "epoch") {
        let en = e.file_name().unwrap().to_string_lossy().into_owned();
        let round = num_after(&en, "epoch");
        if round == 0 {
            continue;
        }
        for h in subdirs(&e, "xbabe") {
            let hn = h.file_name().unwrap().to_string_lossy().into_owned();
            for d in subdirs(&h, "island") {
                let dn = d.file_name().unwrap().to_string_lossy().into_owned();
                let genomes = read(&d.join("genomes_g5.csv"))
                    .lines()
                    .skip(1)
                    .filter(|l| !l.trim().is_empty())
                    .map(|l| {
                        let (id, row) = l.split_once(',').unwrap_or_else(|| fail(format!("{}: bad row", d.display())));
                        (id.to_string(), row.to_string())
                    })
                    .collect();
                let key = format!("{prefix}{en}/{hn}/{dn}");
                out.push(V6Island {
                    host: num_after(&hn, "xbabe"),
                    round,
                    island: num_after(&dn, "island"),
                    launch: serde_json::from_str(&read(&d.join("launch.json"))).unwrap_or_else(|e| fail(format!("{key}: {e}"))),
                    isl: Island {
                        name: key.clone(),
                        archive: jsonl(&d.join("archive.jsonl")),
                        genomes,
                        history: jsonl(&d.join("history.jsonl")),
                        dir: d,
                    },
                    key,
                });
            }
        }
    }
    out
}

/// The pool directories of the lineage, in order: `epoch00/pool`, then `pools/p01`, `p02`, ...
fn pool_dirs(root: &Path) -> Vec<PathBuf> {
    let mut v = vec![root.join("epoch00/pool")];
    v.extend(subdirs(&root.join("pools"), "p"));
    v
}

fn pool_sha(p: &Path) -> Value {
    json!({"archive.jsonl": sha_file(&p.join("archive.jsonl")), "genomes_g5.csv": sha_file(&p.join("genomes_g5.csv"))})
}

fn rel<'a>(root: &Path, p: &'a str) -> &'a str {
    let r = root.to_string_lossy();
    p.strip_prefix(&*r).map(|x| x.trim_start_matches('/')).unwrap_or_else(|| fail(format!("{p} is not under {r}")))
}

/// Check 2 for one pool: its recorded sources (paths under the run root, sha256 as recorded) merged again here must
/// give its files byte for byte. Returns its sources relative to the run root, in order, and its summary.
fn verify_pool(root: &Path, dir: &Path) -> (Vec<String>, Value) {
    let pj: Value = serde_json::from_str(&read(&dir.join("pool.json"))).unwrap_or_else(|e| fail(format!("{}: {e}", dir.display())));
    if s(&pj, "/decoder_sha256") != g5_decoder_sha256() {
        fail(format!("{}: pool from another decoder", dir.display()));
    }
    let srcs = pj["sources"].as_array().unwrap_or_else(|| fail(format!("{}: no sources", dir.display())));
    let mut rels = Vec::new();
    for src in srcs {
        let r = rel(root, s(src, "/dir")).to_string();
        let p = root.join(&r);
        if sha_file(&p.join("archive.jsonl")) != s(src, "/archive.jsonl") {
            fail(format!("{}: source {r} changed since the merge", dir.display()));
        }
        rels.push(r);
    }
    let abs: Vec<String> = rels.iter().map(|r| root.join(r).to_string_lossy().into_owned()).collect();
    let refs: Vec<&str> = abs.iter().map(String::as_str).collect();
    let m = archive::merge_dirs(&refs).unwrap_or_else(|e| fail(format!("{}: {e}", dir.display())));
    let (csv, jl) = archive::rows_text(&m.named());
    if csv != read(&dir.join("genomes_g5.csv")) || jl != read(&dir.join("archive.jsonl")) {
        fail(format!("{}: is not the merge of its sources", dir.display()));
    }
    let best = m.archive.best_elite().unwrap();
    if f(&pj, "/best_raw_score_100").to_bits() != best.record.score.raw_score_100.to_bits() || f(&pj, "/niches") as usize != m.archive.elites.len() {
        fail(format!("{}: pool.json summary does not match the merge", dir.display()));
    }
    let name = dir.strip_prefix(root).unwrap().to_string_lossy().into_owned();
    let summary = json!({"pool": name, "sources": rels, "rows_read": m.rows_read(), "niches": m.archive.elites.len(),
        "qd_score": m.archive.qd_score(), "best_design_id": pj["best_design_id"], "best_raw_score_100": best.record.score.raw_score_100,
        "best_mass_g": best.record.inputs.mass_g, "archive_niches": m.archive.centroids.len(),
        "sha256": pool_sha(dir)});
    (rels, summary)
}

/// Seed rule of the supervisors: raw seed = host * 1e6 + round * 1000 + k, island = round * 100 + k (k from 1).
fn check_launch(i: &V6Island, binary: &str) {
    let l = &i.launch;
    let k = i.island % 100;
    if i.island / 100 != i.round || k == 0 {
        fail(format!("{}: island number is not round*100 + k", i.key));
    }
    let want_seed = i.host as f64 * 1e6 + i.round as f64 * 1000.0 + k as f64;
    let checks = [
        (s(l, "/decoder_sha256") == g5_decoder_sha256(), "decoder"),
        (s(l, "/binary_sha256") == binary, "binary"),
        (f(l, "/host") == i.host as f64 && f(l, "/island") == i.island as f64, "host/island"),
        (f(l, "/raw_seed") == want_seed, "seed rule"),
        (f(l, "/h") == H_SEARCH && f(l, "/tier1_top") == 0.4 && f(l, "/tier2_top") == 0.0, "grid and promotion"),
        (s(l, "/commit") == "bf3e37fa", "decoder commit label"),
    ];
    for (ok, what) in checks {
        if !ok {
            fail(format!("{}: launch.json {what} does not match", i.key));
        }
    }
    let h = &i.isl.history;
    if h.is_empty() || f(h.last().unwrap(), "/evals") != f(l, "/evals") {
        fail(format!("{}: history does not end at the launch budget", i.key));
    }
}

/// Rows of an archive directory, each re-scored bit for bit (check 1).
fn rescored(dir: &Path) -> usize {
    let d = dir.to_string_lossy();
    let rows = archive::read_dir_rows(&d).unwrap_or_else(|e| fail(e));
    for (id, g, row) in &rows {
        archive::record_from_row(id, g, row).unwrap_or_else(|e| fail(e));
    }
    rows.len()
}

/// `[TS] ... started` first and `supervisor done` last in a supervisor.log.
fn supervisor_span(p: &Path) -> (String, String) {
    let log = read(p);
    let ts = |l: &str| l.trim_start_matches('[').split(']').next().unwrap_or("").to_string();
    let started = log.lines().find(|l| l.contains(" started ") || l.contains(" container ")).map(ts);
    let done = log.lines().rev().find(|l| l.ends_with("supervisor done")).map(ts);
    match (started, done) {
        (Some(a), Some(b)) => (a, b),
        _ => fail(format!("{}: no start or no 'supervisor done'", p.display())),
    }
}

pub fn receipt_v6(a: &[String]) {
    let (repo, root, cert) = (Path::new(&a[0]), Path::new(&a[1]), Path::new(&a[2]));
    let shortlist: usize = a.get(3).map_or(32, |v| v.parse().unwrap_or_else(|_| fail("shortlist")));
    let grid_top: usize = a.get(4).map_or(8, |v| v.parse().unwrap_or_else(|_| fail("grid_top")));
    let version = read(&repo.join("docs/VERSION")).trim().to_string();
    let hash = g5_decoder_sha256();
    let rs = requirements::rs_v5();

    // --- check 1: the v5 elites, their re-evaluation on this platform
    let mut v5_files = Vec::new();
    let mut v5_rows: HashMap<String, String> = HashMap::new();
    for n in 1..=16 {
        let name = format!("island{n:02}.csv");
        let (mine, git) = (root.join("v5_elites").join(&name), repo.join("evidence/g5/v5.0.0/genomes").join(&name));
        if fs::read(&mine).ok() != fs::read(&git).ok() {
            fail(format!("v5_elites/{name} is not the committed evidence/g5/v5.0.0/genomes/{name}"));
        }
        for l in read(&git).lines().skip(1).filter(|l| !l.trim().is_empty()) {
            let (id, row) = l.split_once(',').unwrap();
            v5_rows.insert(id.to_string(), row.to_string());
        }
        v5_files.push(json!({"file": format!("evidence/g5/v5.0.0/genomes/{name}"), "sha256": sha_file(&git)}));
    }
    let reeval_dir = root.join("epoch00/reeval");
    let reeval = archive::read_dir_rows(&reeval_dir.to_string_lossy()).unwrap_or_else(|e| fail(e));
    if reeval.len() != v5_rows.len() {
        fail(format!("epoch00/reeval has {} rows for {} v5 elites", reeval.len(), v5_rows.len()));
    }
    for (id, g, row) in &reeval {
        if v5_rows.get(id).map(String::as_str) != Some(g.to_csv_row().as_str()) {
            fail(format!("epoch00/reeval: {id} is not the v5 elite of that id"));
        }
        if s(row, "/tier") != "Tier1SdfAndPlate" {
            fail(format!("epoch00/reeval: {id} is not a Tier-1 record"));
        }
        archive::record_from_row(id, g, row).unwrap_or_else(|e| fail(e));
    }

    // --- check 2: the pool lineage and the islands
    let pools = pool_dirs(root);
    let mut lineage = Vec::new();
    let mut merged_into: HashMap<String, String> = HashMap::new();
    for (k, p) in pools.iter().enumerate() {
        let (srcs, summary) = verify_pool(root, p);
        let name = p.strip_prefix(root).unwrap().to_string_lossy().into_owned();
        let prev = if k == 0 { "epoch00/reeval".to_string() } else { lineage_name(&pools[k - 1], root) };
        if srcs.first() != Some(&prev) || (k == 0 && srcs.len() != 1) {
            fail(format!("{name}: does not continue the lineage from {prev}"));
        }
        for src in &srcs[1..] {
            if merged_into.insert(src.clone(), name.clone()).is_some() {
                fail(format!("{src}: merged into more than one pool"));
            }
        }
        lineage.push(summary);
    }
    let final_pool = pools.last().unwrap().clone();
    let final_name = lineage_name(&final_pool, root);
    let pool_index: HashMap<String, usize> = pools.iter().enumerate().map(|(k, p)| (lineage_name(p, root), k)).collect();

    let warm = load_islands(root, "");
    let control = load_islands(&root.join("control"), "control/");
    let binaries: Value = serde_json::from_str(&read(&cert.join("binaries.json"))).unwrap_or_else(|e| fail(format!("binaries.json: {e}")));
    let binary = s(&binaries, "/g5-continue/sha256").to_string();
    let mut warm_rows = 0;
    for i in &warm {
        check_launch(i, &binary);
        let pool = rel(root, s(&i.launch, "/pool")).to_string();
        let k_in = *pool_index.get(&pool).unwrap_or_else(|| fail(format!("{}: started from {pool}, not a recorded pool", i.key)));
        if i.launch["pool_sha256"] != pool_sha(&pools[k_in]) || f(&i.launch, "/init") != 0.0 {
            fail(format!("{}: its pool sha256 or init does not match {pool}", i.key));
        }
        let into = merged_into.get(&i.key).unwrap_or_else(|| fail(format!("{}: never merged into a pool", i.key)));
        if pool_index[into] <= k_in {
            fail(format!("{}: merged into {into}, not after its start pool {pool}", i.key));
        }
        if f(&i.isl.history[0], "/evals") != 0.0 {
            fail(format!("{}: history does not start at its pool", i.key));
        }
        warm_rows += i.isl.archive.len();
    }
    if merged_into.len() != warm.len() {
        fail("a pool source is neither an island nor the previous pool");
    }
    let control_evals = control.first().map(|i| f(&i.launch, "/evals")).unwrap_or_else(|| fail("no control islands"));
    for i in &control {
        check_launch(i, &binary);
        if !i.launch["pool"].is_null() || f(&i.launch, "/init") != 64.0 || f(&i.launch, "/evals") != control_evals {
            fail(format!("{}: a control island must start fresh (Sobol 64, no pool) with the control budget", i.key));
        }
        if merged_into.keys().any(|k| k.starts_with("control/")) {
            fail(format!("{}: a control island was merged into the lineage", i.key));
        }
        rescored(&i.isl.dir);
    }

    // --- check 3: the final pool through the v5.0.0 certification
    let fp = cert.join("final/island_v6pool/run");
    for f_ in ["archive.jsonl", "genomes_g5.csv"] {
        if fs::read(fp.join(f_)).ok() != fs::read(final_pool.join(f_)).ok() {
            fail(format!("cert/final/island_v6pool/run/{f_} is not the final pool {final_name}"));
        }
    }
    let fin = islands(&cert.join("final"));
    let genomes = verify_root(&fin, &hash);
    let eval = jsonl(&cert.join("eval_h040.jsonl"));
    let reproduced = verify_eval(&fin, &eval, &hash);
    let tier1_rows = fin[0].archive.iter().filter(|r| s(r, "/tier") == "Tier1SdfAndPlate").count();
    if reproduced != tier1_rows {
        fail(format!("eval_h040 reproduced {reproduced} of {tier1_rows} Tier-1 rows"));
    }
    let mut ev: Vec<(f64, &str)> = eval.iter().map(|l| (f(l, "/record/score/score_100"), s(l, "/design_id"))).collect();
    ev.sort_by(|x, y| y.0.total_cmp(&x.0));
    let expect: BTreeSet<&str> = ev.iter().take(shortlist).map(|x| x.1).collect();
    let t2 = jsonl(&cert.join("tier2_h020.jsonl"));
    let got: BTreeSet<&str> = t2.iter().map(|l| s(l, "/design_id")).collect();
    if got != expect || t2.len() != shortlist.min(ev.len()) {
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
    let leader_id = ev[0].1;
    let leader_t1 = &eval.iter().find(|l| s(l, "/design_id") == leader_id).unwrap()["record"];
    let leader_rank = ranking.iter().position(|l| s(l, "/design_id") == leader_id).map(|k| k + 1);
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
    let rec = &ranking[rank_of(&w) - 1]["record"];
    let g = &genomes[&w].1;
    let win_sha = g.sha256();
    // where the winner was created: its id names the host and island (islands of 100 and up are v6 rounds; below, v5.0.0's)
    let (o_host, o_island) = id_origin(&w);
    let origin = if o_island >= 100 {
        warm.iter().find(|i| i.host == o_host && i.island == o_island).map(|i| i.key.clone())
            .unwrap_or_else(|| fail(format!("{w}: created on host {o_host} island {o_island}, which is no warm island of this run")))
    } else {
        format!("v5.0.0 island{o_island:02}")
    };

    // --- check 4: one island re-run on another host
    let repro_meta: Value = serde_json::from_str(&read(&cert.join("repro/repro.json"))).unwrap_or_else(|e| fail(format!("repro.json: {e}")));
    let ri = s(&repro_meta, "/island");
    let mine = &warm.iter().find(|i| i.key == ri).unwrap_or_else(|| fail(format!("repro island {ri}"))).isl.history;
    let re = jsonl(&cert.join("repro/history.jsonl"));
    let keys = ["/evals", "/niches", "/n_tier2", "/qd_score", "/best_raw_score_100", "/best_mass_g"];
    let mut matched = 0;
    for e in &re {
        let Some(m) = mine.iter().find(|m| m["evals"] == e["evals"]) else { continue };
        for k in keys {
            if f(e, k).to_bits() != f(m, k).to_bits() {
                fail(format!("repro {ri} at {} evals: {k} {} != {}", e["evals"], f(e, k), f(m, k)));
            }
        }
        if s(e, "/best_genome_sha256") != s(m, "/best_genome_sha256") {
            fail(format!("repro {ri} at {} evals: best genome differs", e["evals"]));
        }
        matched += 1;
    }
    if matched < 4 {
        fail(format!("repro {ri}: only {matched} history entries to compare"));
    }

    // --- check 5: the v5.0.0 winner on this platform
    let v5w = read(&repo.join("evidence/g5/v5.0.0/winner_genome.csv"));
    let v5_line = v5w.lines().nth(1).unwrap_or_else(|| fail("v5 winner genome"));
    let (v5_id, v5_row) = v5_line.split_once(',').unwrap();
    let v5_t1 = jsonl(&cert.join("v5/tier2_h020.jsonl"));
    let v5r = v5_t1.iter().find(|l| s(l, "/design_id") == v5_id).unwrap_or_else(|| fail("cert/v5: no v5 winner row"));
    let v5_cert_genome = read(&cert.join("v5/island_v5winner/run/genomes_g5.csv"));
    if !v5_cert_genome.lines().any(|l| l == v5_line) || s(&v5r["record"], "/decoder_sha256") != hash || f(v5r, "/h") != H_RECORD {
        fail("cert/v5: not the committed v5.0.0 winner at Tier 2, h 0.2, with this decoder");
    }
    let v5_t1 = reeval.iter().find(|r| r.0 == v5_id).map(|r| f(&r.2, "/raw_score_100")).unwrap_or_else(|| fail("v5 winner not re-evaluated"));
    if v5_rows.get(v5_id).map(String::as_str) != Some(v5_row) {
        fail("the v5.0.0 winner genome is not among the v5 elites");
    }
    let (v5_per, v5_min, v5_worst) = plate_mesh_check(v5_id, &v5r["record"], &jsonl(&cert.join("v5/plate_meshes.jsonl")));
    let v5_in = &v5r["record"]["inputs"];
    let v5_met = v5_min.values().filter(|m| m.as_f64().is_some_and(|x| x >= 1.0)).count();
    let v5_same_platform = json!({"design_id": v5_id, "tier1_h04_score_100": v5_t1, "score": score_json(&v5r["record"]["score"]),
        "mass_g": v5_in["mass_g"], "grip_length_mm": v5_in["L_g_mm"], "grip_perimeter_mm": v5_in["swell_perimeter_mm"],
        "genome_sha256": genome_of(&v5_cert_genome, v5_id), "cases_met": v5_met, "cases": v5_min.len(),
        "margins_min_over_plate_meshes": v5_min, "worst_margin": num(v5_worst),
        "plate_meshes": v5_per, "loads": v5r["record"]["loads"],
        "what": "the v5.0.0 winner's committed genome at Tier 2, h 0.2 and on the plate check meshes, on the v6 platform of record"});

    // --- the winner, as v5.0.0
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
    let search_row = fin[0].archive.iter().find(|r| s(r, "/design_id") == w).unwrap();

    // --- evolution: rounds per host, the lineage, the control
    let mut rounds: BTreeMap<(u32, u32), Vec<&V6Island>> = BTreeMap::new();
    for i in &warm {
        rounds.entry((i.host, i.round)).or_default().push(i);
    }
    let best_of = |name: &str| lineage.iter().find(|l| l["pool"] == json!(name)).map(|l| l["best_raw_score_100"].clone());
    let round_rows: Vec<Value> = rounds
        .iter()
        .map(|((h, r), v)| {
            let pool_in = rel(root, s(&v[0].launch, "/pool")).to_string();
            let into = merged_into[&v[0].key].clone();
            let (st, dn) = supervisor_span(&root.join(format!("epoch{r:02}/xbabe{h}/supervisor.log")));
            json!({"host": format!("xbabe{h}"), "round": r, "islands": v.len(), "evaluations_per_island": v[0].launch["evals"],
                   "pool_in": pool_in, "pool_in_best": best_of(&pool_in), "merged_into": into, "merged_best": best_of(&into),
                   "started_utc": st, "finished_utc": dn,
                   "island_final_best": v.iter().map(|i| i.isl.history.last().unwrap()["best_raw_score_100"].clone()).collect::<Vec<_>>()})
        })
        .collect();
    let seeds: BTreeMap<String, u64> = warm.iter().chain(control.iter()).map(|i| (i.key.clone(), f(&i.launch, "/raw_seed") as u64)).collect();
    let control_evo: Vec<Value> = control.iter().map(|i| island_evolution(&i.isl, &seeds)).collect();
    let control_best: Vec<f64> = control_evo.iter().map(|e| f(e, "/final/best_score_100")).collect();
    let warm_evals: f64 = warm.iter().map(|i| f(&i.launch, "/evals")).sum();
    let depth = rounds.keys().map(|k| k.1).max().unwrap_or(0);

    // --- write (nothing above wrote a file)
    let ev_dir = repo.join(format!("evidence/g5/v{version}"));
    let img_dir = repo.join(format!("docs/g5/v{version}/img"));
    for d in [ev_dir.join("history"), ev_dir.join("launch"), ev_dir.join("pools"), ev_dir.join("final_pool"), ev_dir.join("repro"),
              ev_dir.join("v5_same_platform"), ev_dir.join("spike"), ev_dir.join("fleet"), img_dir.clone()] {
        fs::create_dir_all(&d).unwrap();
    }
    let wrow = fin[0].genomes.iter().find(|(id, _)| *id == w).unwrap();
    fs::write(ev_dir.join("winner_genome.csv"), format!("design_id,genome_row\n{},{}\n", wrow.0, wrow.1)).unwrap();
    for f_ in ["archive.jsonl", "genomes_g5.csv", "pool.json"] {
        fs::copy(final_pool.join(f_), ev_dir.join("final_pool").join(f_)).unwrap();
    }
    for p_ in &pools {
        let n = lineage_name(p_, root).replace('/', "_");
        fs::copy(p_.join("pool.json"), ev_dir.join(format!("pools/{n}.json"))).unwrap();
    }
    for i in warm.iter().chain(control.iter()) {
        let n = i.key.replace('/', "_");
        fs::copy(i.isl.dir.join("history.jsonl"), ev_dir.join(format!("history/{n}.jsonl"))).unwrap();
        fs::copy(i.isl.dir.join("launch.json"), ev_dir.join(format!("launch/{n}.json"))).unwrap();
    }
    let mut csv = String::from("design_id,archive_tier,tier1_score_100,mass_g,worst_margin_W,E\n");
    let tiers: HashMap<&str, &str> = fin[0].archive.iter().map(|r| (s(r, "/design_id"), s(r, "/tier"))).collect();
    let mut lines: Vec<&Value> = eval.iter().collect();
    lines.sort_by(|x, y| s(x, "/design_id").cmp(s(y, "/design_id")));
    for l in lines {
        let (id, r) = (s(l, "/design_id"), &l["record"]);
        let sc = score_json(&r["score"]);
        csv += &format!("{id},{},{},{},{},{}\n", tiers[id], r["score"]["score_100"], r["inputs"]["mass_g"], sc["worst_margin_W"], r["score"]["E"]);
    }
    fs::write(ev_dir.join("elites_h040.csv"), csv).unwrap();
    fs::write(ev_dir.join("ranking_tier2_h020.jsonl"), ranking.iter().map(|l| l.to_string() + "\n").collect::<String>()).unwrap();
    fs::copy(cert.join("hconv.jsonl"), ev_dir.join("grid_check.jsonl")).unwrap();
    fs::copy(cert.join("plate_meshes.jsonl"), ev_dir.join("plate_meshes.jsonl")).unwrap();
    fs::copy(cert.join("repro/repro.json"), ev_dir.join("repro/repro.json")).unwrap();
    fs::copy(cert.join("repro/history.jsonl"), ev_dir.join("repro/history.jsonl")).unwrap();
    fs::copy(cert.join("binaries.json"), ev_dir.join("binaries.json")).unwrap();
    fs::copy(cert.join("v5/tier2_h020.jsonl"), ev_dir.join("v5_same_platform/tier2_h020.jsonl")).unwrap();
    fs::copy(cert.join("v5/plate_meshes.jsonl"), ev_dir.join("v5_same_platform/plate_meshes.jsonl")).unwrap();
    // the GPU spike and the fleet sizing measurements (their own JSON; the paper and README read them)
    for (src, dst) in [("spike/spike.json", "spike/spike.json"), ("spike/profile_tier1_h040.tsv", "spike/profile_tier1_h040.tsv"),
                       ("spike/screen.jsonl", "spike/screen.jsonl"), ("fleet/sizing.json", "fleet/sizing.json")] {
        fs::copy(cert.join(src), ev_dir.join(dst)).unwrap_or_else(|e| fail(format!("cert/{src}: {e}")));
    }
    let assets: Vec<Value> = {
        let mut v = Vec::new();
        files_under(&cert.join("assets"), &mut v);
        v.sort();
        v.iter()
            .map(|p| json!({"file": p.file_name().unwrap().to_string_lossy(), "bytes": fs::metadata(p).unwrap().len(), "sha256": sha_file(p)}))
            .collect()
    };
    let figure = format!("docs/g5/v{version}/img/evolution.svg");
    let svg = evolution_v6_svg(&lineage, &round_rows, &control, v5_t1, reeval.len() as f64);
    if fs::read_to_string(repo.join(&figure)).ok().as_deref() != Some(svg.as_str()) {
        fs::write(repo.join(&figure), svg).unwrap();
    }
    let ranking_rows: Vec<Value> = ranking
        .iter()
        .enumerate()
        .map(|(k, l)| {
            let r = &l["record"];
            let id = s(l, "/design_id");
            json!({"rank": k + 1, "design_id": id, "score_100": r["score"]["score_100"], "mass_g": r["inputs"]["mass_g"],
                   "E": r["score"]["E"], "worst_margin_W": score_json(&r["score"])["worst_margin_W"],
                   "grid_pass": grid_rows.iter().find(|g| g["design_id"] == json!(id)).map(|g| g["pass"].clone())})
        })
        .collect();
    let mut receipt = json!({
        "version": version,
        "generation": "G5",
        "schema": "G5.0",
        "crate_version": VERSION,
        "design_id": w,
        "status": "computational design; not built, printed or physically tested",
        "provenance": {
            "decoder_sha256": hash, "decoder_commit_label": g.decoder_commit, "genome_sha256": win_sha,
            "genome_csv": format!("evidence/g5/v{version}/winner_genome.csv"), "requirements": "requirements/RS-karambit-v5.toml",
            "metric": rec["score"]["metric"], "metric_draft": rec["score"]["draft"],
            "platform_of_record": "linux/amd64: static x86_64 binaries, native on Linux and in a Rosetta container on macOS; Tier 2 at h 0.2 mm",
            "binaries": binaries, "vv": "evidence/g5/v5.0.0/vv",
        },
        "warm_start": {
            "v5_elite_genomes": v5_files, "v5_elites": v5_rows.len(), "reevaluated": "epoch00/reeval",
            "pool": lineage[0].clone(),
            "what": "the 1,508 final elites of the v5.0.0 cohort (islands 01-16; the random-search control excluded), re-evaluated at Tier 1, h 0.4 on this platform and merged to the best record per niche",
        },
        "lineage": lineage,
        "cohort": {
            "what": "warm-started islands in asynchronous rounds per host; at the end of a host's round the hub merges its islands into the latest pool, from which the host's next round starts",
            "islands": warm.len(), "rounds": round_rows, "round_depth_max": depth, "evaluations_total": warm_evals,
            "evaluations_per_island": warm.first().map(|i| i.launch["evals"].clone()), "h_mm": H_SEARCH,
            "elites": genomes.len(), "final_pool": final_name, "tier1_rows_reproduced": reproduced,
            "island_archive_rows": warm_rows,
            "promotion": {"tier1_top": warm[0].launch["tier1_top"], "tier2_top": warm[0].launch["tier2_top"],
                          "log_every": warm[0].launch["log_every"]},
            "reproduced": {"island": ri, "rerun": repro_meta, "history_entries_identical": matched},
        },
        "baseline": {
            "what": "fresh-start control: scrambled Sobol start (64), no pool and no migration, the same decoder, objective and promotion; never merged and not eligible to win",
            "islands": control.len(), "evaluations_per_island": control_evals, "final_best_score_100": control_best,
            "islands_detail": control_evo,
        },
        "ranking": {
            "h_mm": H_RECORD, "tip_mesh_max_area_mm2": TIP_FE_MAX_AREA_MM2, "shortlist": shortlist,
            "how": format!("every elite of the final pool re-evaluated at Tier 1, h {H_SEARCH}; the top {shortlist} at Tier 2, h {H_RECORD}"),
            "rows": ranking_rows,
            "search_leader": {"design_id": leader_id, "tier1_h04_score_100": leader_t1["score"]["score_100"], "tier2_rank": leader_rank},
            "v5_winner_same_platform": v5_same_platform,
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
        "cases_met": margins_min.values().filter(|m| m.as_f64().is_some_and(|x| x >= 1.0)).count(), "cases": margins_min.len(),
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
            "history_dir": format!("evidence/g5/v{version}/history"), "figure": figure,
            "winner_island": origin,
            "rounds": rounds.len(), "control_final_best_score_100": control_best,
        },
        "release_assets": assets,
    });
    let budget = write_fitness_figures(&receipt, &img_dir);
    receipt["fitness_budget"] = fitness_budget_json(&budget);
    fs::write(ev_dir.join("release.json"), serde_json::to_string_pretty(&receipt).unwrap() + "\n").unwrap();
    fs::write(repo.join("evidence/g5/RECORD"), format!("{version}\n")).unwrap();

    let mut files = Vec::new();
    files_under(&ev_dir, &mut files);
    files_under(&repo.join(format!("docs/g5/v{version}")), &mut files);
    files.retain(|p| p.file_name().is_some_and(|n| n != "SHA256SUMS"));
    let mut relp: Vec<(String, &PathBuf)> = files.iter().map(|p| (p.strip_prefix(repo).unwrap().to_string_lossy().into_owned(), p)).collect();
    relp.sort();
    let sums: String = relp.iter().map(|(r, p)| format!("{}  {r}\n", sha_file(p))).collect();
    fs::write(ev_dir.join("SHA256SUMS"), sums).unwrap();
    println!("{}", json!({"design_id": w, "rank": rank_of(&w), "score_100": rec["score"]["score_100"],
                          "mass_g": rec["inputs"]["mass_g"], "worst_margin_over_plate_meshes": num(mu_min),
                          "v5_winner_same_platform_score_100": v5_same_platform["score"]["score_100"], "files": relp.len()}));
}

/// The host and island a design id names (`kv<host>i<island>_g<niche>_<sha8>`).
fn id_origin(id: &str) -> (u32, u32) {
    let body = id.strip_prefix("kv").unwrap_or_else(|| fail(format!("{id}: not a G5 id")));
    let (h, rest) = body.split_once('i').unwrap_or_else(|| fail(format!("{id}: no island")));
    let isl = rest.split('_').next().unwrap_or("");
    match (h.parse(), isl.parse()) {
        (Ok(h), Ok(i)) => (h, i),
        _ => fail(format!("{id}: cannot read host and island")),
    }
}

/// The genome sha256 of a design id's row in a `genomes_g5.csv` text.
fn genome_of(csv: &str, id: &str) -> String {
    let row = csv.lines().find_map(|l| l.strip_prefix(&format!("{id},"))).unwrap_or_else(|| fail(format!("{id}: no genome row")));
    G5Genome::from_csv_row(row).unwrap_or_else(|e| fail(format!("{id}: {e}"))).sha256()
}

fn lineage_name(p: &Path, root: &Path) -> String {
    p.strip_prefix(root).unwrap().to_string_lossy().into_owned()
}

/// The v6 evolution figure, best score (Tier 1, h 0.4) against the evaluations spent, log scale: the pool lineage (series
/// 1, blue) from the re-evaluation of the v5.0.0 elites on, a step at each merge; the fresh-start control (series 2, orange,
/// dashed) as the best over all its islands against their combined evaluations; the v5.0.0 winner's Tier-1 score on this
/// platform as a dotted reference. The validated pair of `evolution_svg`.
fn evolution_v6_svg(lineage: &[Value], rounds: &[Value], control: &[V6Island], v5_t1: f64, reeval_evals: f64) -> String {
    let mut pts: Vec<(f64, f64, String)> = Vec::new();
    let mut cum = reeval_evals;
    for l in lineage {
        let name = s(l, "/pool").to_string();
        cum += rounds.iter().filter(|r| r["merged_into"] == json!(name)).map(|r| f(r, "/islands") * f(r, "/evaluations_per_island")).sum::<f64>();
        pts.push((cum, f(l, "/best_raw_score_100"), name));
    }
    // the control's best over all its islands after e evaluations each, at n * e evaluations spent
    let mut evals: Vec<f64> = control.iter().flat_map(|i| i.isl.history.iter().map(|e| f(e, "/evals"))).collect();
    evals.sort_by(f64::total_cmp);
    evals.dedup();
    let nc = control.len() as f64;
    let ctl: Vec<(f64, f64)> = evals
        .iter()
        .filter(|&&e| e > 0.0)
        .map(|&e| {
            let b = control
                .iter()
                .filter_map(|i| i.isl.history.iter().filter(|h| f(h, "/evals") <= e).last().map(|h| f(h, "/best_raw_score_100")))
                .fold(f64::NEG_INFINITY, f64::max);
            (nc * e, b)
        })
        .filter(|p| p.1.is_finite())
        .collect();
    let xs: Vec<f64> = pts.iter().map(|p| p.0).chain(ctl.iter().map(|p| p.0)).collect();
    let (xlo, xhi) = (xs.iter().copied().fold(f64::INFINITY, f64::min).max(1.0), xs.iter().copied().fold(1.0, f64::max));
    let (lx0, lx1) = (xlo.log10().floor(), xhi.log10().ceil().max(xlo.log10().floor() + 1.0));
    let vals: Vec<f64> = pts.iter().map(|p| p.1).chain(ctl.iter().map(|p| p.1)).chain([v5_t1]).collect();
    let lo = (vals.iter().copied().fold(f64::INFINITY, f64::min) / 10.0).floor() * 10.0;
    let hi = (vals.iter().copied().fold(f64::NEG_INFINITY, f64::max) / 5.0).ceil() * 5.0;
    let (w, h, l, r, t, b) = (760.0, 400.0, 48.0, 210.0, 78.0, 46.0);
    let x = |e: f64| l + (w - l - r) * (e.max(1.0).log10() - lx0) / (lx1 - lx0);
    let y = |v: f64| t + (h - t - b) * (hi - v) / (hi - lo);
    let mut o = String::new();
    o += &format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {w} {h}\" width=\"{w}\" height=\"{h}\" role=\"img\" \
         aria-labelledby=\"t d\" font-family=\"system-ui, -apple-system, 'Segoe UI', sans-serif\">\n"
    );
    let theme = |bg: &str, ink: &str, ink2: &str, muted: &str, grid: &str, axis: &str, s1: &str, s2: &str| {
        format!(
            ".bg{{fill:{bg}}}.t1{{fill:{ink}}}.t2,.lb{{fill:{ink2}}}.tk{{fill:{muted}}}.grid{{stroke:{grid}}}\
             .axis{{stroke:{axis}}}.ref{{stroke:{muted}}}.win{{stroke:{s1}}}.dot{{fill:{bg};stroke:{s1}}}.base{{stroke:{s2}}}"
        )
    };
    o += &format!(
        "<style>\n{}\n@media (prefers-color-scheme: dark){{{}}}\n\
         .t1{{font-size:15px;font-weight:600}}.t2,.lb{{font-size:12px}}.tk{{font-size:11px}}\
         .grid,.axis{{stroke-width:1}}.ref{{fill:none;stroke-width:1;stroke-dasharray:2 3}}\
         .win{{fill:none;stroke-width:2;stroke-linejoin:round}}.base{{fill:none;stroke-width:2;stroke-dasharray:6 4}}\
         .dot{{stroke-width:2}}\n</style>\n",
        theme("#fcfcfb", "#0b0b0b", "#52514e", "#898781", "#e1e0d9", "#c3c2b7", "#2a78d6", "#eb6834"),
        theme("#1a1a19", "#f0efec", "#c3c2b7", "#898781", "#2c2c2a", "#383835", "#3987e5", "#d95926")
    );
    let last = pts.last().cloned().unwrap_or((0.0, 0.0, String::new()));
    let cbest = ctl.last().map_or(f64::NAN, |p| p.1);
    let cspent = ctl.last().map_or(0.0, |p| p.0);
    o += &format!(
        "<title id=\"t\">Best score found against evaluations spent, v6 continuation and fresh-start control</title>\n<desc id=\"d\">\
         The pool lineage starts from the re-evaluated v5.0.0 elites at {:.1} and reaches {:.1} after {:.0} evaluations; the {} \
         fresh-start control islands reach {cbest:.1} after {cspent:.0} evaluations; the v5.0.0 winner scores {v5_t1:.1} on the same \
         grid.</desc>\n",
        pts.first().map_or(0.0, |p| p.1),
        last.1,
        last.0,
        control.len()
    );
    o += &format!("<rect class=\"bg\" width=\"{w}\" height=\"{h}\" rx=\"8\"/>\n");
    o += &format!("<text class=\"t1\" x=\"{l}\" y=\"26\">Best score found, by evaluations spent</text>\n");
    o += &format!(
        "<text class=\"t2\" x=\"{l}\" y=\"45\">W_ERGOLIGHT_v2, Tier 1 at h 0.4 mm · evaluations on a log scale, summed over \
         each search's islands</text>\n"
    );
    let lg = [("win", "Pool lineage (warm start)"), ("base", "Fresh-start control"), ("ref", "v5.0.0 winner (same grid)")];
    let mut lgx = l;
    for (cls, name) in lg {
        o += &format!("<line class=\"{cls}\" x1=\"{lgx}\" y1=\"62\" x2=\"{}\" y2=\"62\"/>", lgx + 22.0);
        o += &format!("<text class=\"lb\" x=\"{}\" y=\"66\">{name}</text>\n", lgx + 28.0);
        lgx += 28.0 + 6.6 * name.len() as f64 + 22.0;
    }
    let step = if hi - lo > 30.0 { 10.0 } else { 5.0 };
    let mut v = lo;
    while v <= hi + 1e-9 {
        o += &format!("<line class=\"grid\" x1=\"{l}\" x2=\"{}\" y1=\"{:.1}\" y2=\"{:.1}\"/>", w - r, y(v), y(v));
        o += &format!("<text class=\"tk\" x=\"{}\" y=\"{:.1}\" text-anchor=\"end\">{v:.0}</text>\n", l - 8.0, y(v) + 4.0);
        v += step;
    }
    let mut d = lx0;
    while d <= lx1 + 1e-9 {
        let e = 10f64.powf(d);
        let lab = if e >= 1e6 { format!("{:.0}M", e / 1e6) } else if e >= 1e3 { format!("{:.0}k", e / 1e3) } else { format!("{e:.0}") };
        o += &format!("<text class=\"tk\" x=\"{:.1}\" y=\"{}\" text-anchor=\"middle\">{lab}</text>", x(e), h - b + 18.0);
        d += 1.0;
    }
    o += &format!("<line class=\"axis\" x1=\"{l}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n", w - r, h - b, h - b);
    o += &format!("<text class=\"tk\" x=\"{:.1}\" y=\"{}\" text-anchor=\"middle\">evaluations spent (log scale)</text>\n",
                  0.5 * (l + w - r), h - 8.0);
    o += &format!("<line class=\"ref\" x1=\"{l}\" x2=\"{}\" y1=\"{:.1}\" y2=\"{:.1}\"/>\n", w - r, y(v5_t1), y(v5_t1));
    let path = |p: &[(f64, f64)]| -> String {
        let mut d = format!("M{:.1} {:.1}", x(p[0].0), y(p[0].1));
        for q in &p[1..] {
            d += &format!(" H{:.1} V{:.1}", x(q.0), y(q.1));
        }
        d
    };
    if !ctl.is_empty() {
        o += &format!("<path class=\"base\" d=\"{}\"><title>fresh-start control: {cbest:.2} after {cspent:.0} evaluations</title></path>\n", path(&ctl));
    }
    if !pts.is_empty() {
        let lp: Vec<(f64, f64)> = pts.iter().map(|p| (p.0, p.1)).collect();
        o += &format!("<path class=\"win\" d=\"{}\"/>\n", path(&lp));
        for p in &pts {
            o += &format!("<circle class=\"dot\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"3.5\"><title>{}: {:.2} after {:.0} evaluations</title></circle>\n",
                          x(p.0), y(p.1), p.2, p.1, p.0);
        }
    }
    let mut labels = vec![
        (y(last.1), format!("pool lineage · {:.1}", last.1)),
        (y(cbest), format!("control · best {cbest:.1}")),
        (y(v5_t1), format!("v5.0.0 winner · {v5_t1:.1}")),
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
