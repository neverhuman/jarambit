//! K-fold CatBoost cross-validation of every regression table (final column = target).
//!
//! CatBoost runs as its official standalone CLI binary (no Python, no new crates). Each fold trains
//! on k-1 folds and predicts the held-out fold with `--use-best-model false`, so the held-out rows
//! never steer training. Out-of-fold (OOF) predictions are pooled and scored against the truth:
//! Pearson r (+ Fisher 95 % CI), Spearman rho, R², RMSE/MAE and the RMSE of a fold-mean baseline.
use super::csvio;
use super::json::Json;
use super::stats::{self, SplitMix};
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct CvOptions {
    pub tables: PathBuf,
    pub out: PathBuf,
    pub catboost: PathBuf,
    pub folds: usize,
    pub seed: u64,
    pub iterations: usize,
    pub learning_rate: f64,
    pub depth: usize,
    pub threads: usize,
    pub jobs: usize,
    pub max_rows: usize,
    pub only: Option<String>,
    pub permutations: usize,
    pub timeout_s: u64,
    pub keep_work: bool,
}

impl CvOptions {
    pub fn defaults(tables: PathBuf, out: PathBuf, catboost: PathBuf) -> Self {
        let cpus = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(2);
        CvOptions {
            tables,
            out,
            catboost,
            folds: 5,
            seed: 20_261_020,
            iterations: 500,
            learning_rate: 0.05,
            depth: 6,
            threads: cpus,
            jobs: (cpus / 4).max(1),
            max_rows: 50_000,
            only: None,
            permutations: 0,
            timeout_s: 1800,
            keep_work: false,
        }
    }
}

/// Locate CatBoost: explicit path, `$JARAMBIT_CATBOOST`, or `catboost` on PATH.
pub fn catboost_bin(explicit: Option<&str>) -> Result<PathBuf, String> {
    let candidate = explicit.map(PathBuf::from).or_else(|| std::env::var("JARAMBIT_CATBOOST").ok().map(PathBuf::from));
    if let Some(p) = candidate {
        return if p.is_file() { Ok(p) } else { Err(format!("catboost CLI not found at {}", p.display())) };
    }
    let path = std::env::var("PATH").unwrap_or_default();
    for dir in std::env::split_paths(&path) {
        let c = dir.join("catboost");
        if c.is_file() {
            return Ok(c);
        }
    }
    Err("catboost CLI unavailable: pass --catboost PATH or set JARAMBIT_CATBOOST (see datasets/regression/README.md)".into())
}

pub fn list_tables(dir: &Path, only: Option<&str>) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = std::fs::read_dir(dir)
        .map(|it| {
            it.flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|x| x == "csv"))
                .filter(|p| only.is_none_or(|o| p.file_stem().is_some_and(|s| s.to_string_lossy().contains(o))))
                .collect()
        })
        .unwrap_or_default();
    v.sort();
    v
}

struct Loaded {
    header: Vec<String>,
    prefixes: Vec<String>,
    y: Vec<f64>,
    categorical: Vec<usize>,
    rows_in_file: usize,
    /// Lineage group per used row, from the `<table>.groups` sidecar written by `build`.
    groups: Option<Vec<String>>,
}

fn load(path: &Path, max_rows: usize) -> Result<Loaded, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut lines = text.lines();
    let header = csvio::split(lines.next().ok_or("empty table")?);
    if header.len() < 2 {
        return Err("table needs at least one feature and a target".into());
    }
    let mut prefixes = Vec::new();
    let mut y = Vec::new();
    let mut rows_in_file = 0;
    let mut categorical: Vec<bool> = vec![false; header.len() - 1];
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        rows_in_file += 1;
        if max_rows > 0 && y.len() >= max_rows {
            continue;
        }
        let cells = csvio::split(line);
        if cells.len() != header.len() {
            return Err(format!("row {rows_in_file}: {} cells, header has {}", cells.len(), header.len()));
        }
        let target = csvio::num(&cells[cells.len() - 1]);
        if !target.is_finite() {
            return Err(format!("row {rows_in_file}: non-numeric target"));
        }
        if y.len() < 5000 {
            for (j, c) in cells[..cells.len() - 1].iter().enumerate() {
                categorical[j] |= csvio::is_text(c);
            }
        }
        prefixes.push(csvio::join(&cells[..cells.len() - 1]));
        y.push(target);
    }
    let categorical = categorical.iter().enumerate().filter(|(_, c)| **c).map(|(j, _)| j).collect();
    let groups = std::fs::read_to_string(path.with_extension("groups"))
        .ok()
        .map(|t| t.lines().map(str::to_string).collect::<Vec<_>>())
        .filter(|g| g.len() == rows_in_file)
        .map(|mut g| {
            g.truncate(y.len());
            g
        });
    Ok(Loaded { header, prefixes, y, categorical, rows_in_file, groups })
}

fn run_with_timeout(mut cmd: Command, log: &Path, timeout: Duration) -> Result<(), String> {
    let logf = std::fs::File::create(log).map_err(|e| e.to_string())?;
    let errf = logf.try_clone().map_err(|e| e.to_string())?;
    let mut child = cmd.stdout(Stdio::from(logf)).stderr(Stdio::from(errf)).spawn().map_err(|e| format!("catboost spawn: {e}"))?;
    let start = Instant::now();
    loop {
        match child.try_wait().map_err(|e| e.to_string())? {
            Some(s) if s.success() => return Ok(()),
            Some(s) => {
                let tail = std::fs::read_to_string(log).unwrap_or_default();
                let tail: String = tail.lines().rev().take(8).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join(" | ");
                return Err(format!("catboost exited {s}: {tail}"));
            }
            None if start.elapsed() > timeout => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(format!("catboost exceeded {}s", timeout.as_secs()));
            }
            None => std::thread::sleep(Duration::from_millis(50)),
        }
    }
}

/// Random k-fold: a SplitMix64 permutation seeded by `seed ^ FNV(table name)`, dealt round-robin.
pub fn random_folds(seed: u64, name: &str, n: usize, k: usize) -> Vec<usize> {
    let k = k.clamp(2, n.max(2));
    let mut fold = vec![0usize; n];
    let perm = SplitMix(seed ^ stats::fnv1a(name.as_bytes())).permutation(n);
    for (pos, &row) in perm.iter().enumerate() {
        fold[row] = pos % k;
    }
    fold
}

/// Grouped k-fold: every lineage group is held out as a whole. Groups are dealt largest first
/// (ties in a seeded order) to the fold with the fewest rows. Returns (folds, k) with
/// k = min(requested, distinct groups), or None with fewer than 3 groups.
pub fn group_folds(groups: &[String], k: usize, seed: u64) -> Option<(Vec<usize>, usize)> {
    let mut sizes: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
    for g in groups {
        *sizes.entry(g.as_str()).or_insert(0) += 1;
    }
    if sizes.len() < 3 {
        return None;
    }
    let k = k.clamp(2, sizes.len());
    let mut order: Vec<(&str, usize, u64)> = sizes.iter().map(|(g, n)| (*g, *n, stats::fnv1a(g.as_bytes()) ^ seed)).collect();
    order.sort_by(|a, b| b.1.cmp(&a.1).then(a.2.cmp(&b.2)));
    let mut load = vec![0usize; k];
    let mut of: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
    for (g, n, _) in order {
        let f = (0..k).min_by_key(|&f| (load[f], f)).unwrap_or(0);
        load[f] += n;
        of.insert(g, f);
    }
    Some((groups.iter().map(|g| of[g.as_str()]).collect(), k))
}

/// Train/predict every fold of `fold` (values 0..k). Returns (OOF predictions, fold-mean baseline,
/// mean importances).
#[allow(clippy::type_complexity)]
fn kfold(
    o: &CvOptions,
    d: &Loaded,
    y: &[f64],
    fold: &[usize],
    k: usize,
    work: &Path,
    threads: usize,
) -> Result<(Vec<f64>, Vec<f64>, Vec<(String, f64)>), String> {
    let n = y.len();
    let header = csvio::join(&d.header);
    let target_idx = d.header.len() - 1;
    let mut cd = format!("{target_idx}\tLabel\n");
    for j in &d.categorical {
        cd.push_str(&format!("{j}\tCateg\n"));
    }
    let mut pred = vec![f64::NAN; n];
    let mut base = vec![f64::NAN; n];
    let mut importance: Vec<(String, f64)> = d.header[..target_idx].iter().map(|h| (h.clone(), 0.0)).collect();
    for f in 0..k {
        let dir = work.join(format!("fold{f}"));
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let mut learn = String::with_capacity(header.len() + d.prefixes.len() * 64);
        let mut test = learn.clone();
        learn.push_str(&header);
        learn.push('\n');
        test.push_str(&header);
        test.push('\n');
        let mut test_rows = Vec::new();
        let (mut sum, mut cnt) = (0.0, 0usize);
        for i in 0..n {
            let line = format!("{},{}\n", d.prefixes[i], csvio::sig7(y[i]));
            if fold[i] == f {
                test.push_str(&line);
                test_rows.push(i);
            } else {
                learn.push_str(&line);
                sum += y[i];
                cnt += 1;
            }
        }
        if test_rows.is_empty() || cnt < 2 {
            return Err(format!("fold {f} has too few rows"));
        }
        std::fs::write(dir.join("learn.csv"), learn).map_err(|e| e.to_string())?;
        std::fs::write(dir.join("test.csv"), test).map_err(|e| e.to_string())?;
        std::fs::write(dir.join("train.cd"), &cd).map_err(|e| e.to_string())?;
        let mut cmd = Command::new(&o.catboost);
        cmd.current_dir(&dir).args([
            "fit",
            "--learn-set",
            "learn.csv",
            "--test-set",
            "test.csv",
            "--column-description",
            "train.cd",
            "--delimiter",
            ",",
            "--has-header",
            "--loss-function",
            "RMSE",
            "--use-best-model",
            "false",
            "--iterations",
            &o.iterations.to_string(),
            "--learning-rate",
            &o.learning_rate.to_string(),
            "--depth",
            &o.depth.to_string(),
            "--random-seed",
            &(o.seed % 1_000_000_007).to_string(),
            "--thread-count",
            &threads.max(1).to_string(),
            "--train-dir",
            "train",
            "--model-file",
            "model.bin",
            "--eval-file",
            "eval.tsv",
            "--fstr-file",
            "fstr.tsv",
            "--fstr-type",
            "PredictionValuesChange",
            "--logging-level",
            "Silent",
        ]);
        run_with_timeout(cmd, &dir.join("catboost.log"), Duration::from_secs(o.timeout_s))?;
        let eval = std::fs::read_to_string(dir.join("train/eval.tsv")).map_err(|e| format!("fold {f}: eval output missing: {e}"))?;
        let mut el = eval.lines();
        let cols: Vec<&str> = el.next().unwrap_or("").split('\t').collect();
        let pc = cols.iter().position(|c| *c == "RawFormulaVal").ok_or("eval output lacks RawFormulaVal")?;
        let preds: Vec<f64> = el.filter(|l| !l.is_empty()).map(|l| l.split('\t').nth(pc).map(csvio::num).unwrap_or(f64::NAN)).collect();
        if preds.len() != test_rows.len() || preds.iter().any(|p| !p.is_finite()) {
            return Err(format!("fold {f}: {} predictions for {} held-out rows", preds.len(), test_rows.len()));
        }
        let mean = sum / cnt as f64;
        for (&i, p) in test_rows.iter().zip(preds) {
            pred[i] = p;
            base[i] = mean;
        }
        if let Ok(fs) = std::fs::read_to_string(dir.join("train/fstr.tsv")) {
            for l in fs.lines() {
                let mut it = l.split('\t');
                if let (Some(v), Some(name)) = (it.next(), it.next()) {
                    if let Some(e) = importance.iter_mut().find(|(n, _)| n == name) {
                        e.1 += csvio::num(v) / k as f64;
                    }
                }
            }
        }
    }
    importance.sort_by(|a, b| b.1.total_cmp(&a.1));
    Ok((pred, base, importance))
}

/// OOF metrics for one fold scheme.
fn scores(y: &[f64], pred: &[f64], basep: &[f64]) -> (f64, f64, f64, Json) {
    let n = y.len();
    let r = stats::pearson(y, pred);
    let (lo, hi) = stats::fisher_ci95(r, n);
    let rmse = stats::rmse(y, pred);
    let rmse_base = stats::rmse(y, basep);
    let r2 = stats::r2(y, pred);
    let verdict = stats::verdict(r, lo, rmse, rmse_base);
    let doc = Json::obj()
        .with("pearson_r", r)
        .with("pearson_r_ci95", Json::Arr(vec![lo.into(), hi.into()]))
        .with("spearman_rho", stats::spearman(y, pred))
        .with("r2_oof", r2)
        .with("rmse", rmse)
        .with("mae", stats::mae(y, pred))
        .with("rmse_fold_mean_baseline", rmse_base)
        .with("lift_pct", if rmse_base > 0.0 { 100.0 * (1.0 - rmse / rmse_base) } else { f64::NAN })
        .with("verdict", verdict)
        .with("strength", stats::strength(verdict, r2));
    (r, lo, rmse, doc)
}

pub fn evaluate_table(o: &CvOptions, path: &Path, threads: usize) -> Json {
    let name = path.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(String::new);
    let started = Instant::now();
    let base = Json::obj().with("dataset", name.as_str()).with("file", path.display().to_string());
    let d = match load(path, o.max_rows) {
        Ok(d) => d,
        Err(e) => return base.with("status", "error").with("error", e),
    };
    let target = d.header.last().cloned().unwrap_or_else(String::new);
    let (tname, fset) = match name.rsplit_once("__") {
        Some((t, f)) => (t.to_string(), f.to_string()),
        None => (target.clone(), String::new()),
    };
    let n = d.y.len();
    let mut doc = base
        .with("target", tname.as_str())
        .with("featureset", fset.as_str())
        .with("n_rows", n)
        .with("rows_in_file", d.rows_in_file)
        .with("n_features", d.header.len() - 1)
        .with("folds", o.folds.min(n));
    if n < o.folds.max(10) {
        return doc.with("status", "error").with("error", format!("too few rows ({n}) for {}-fold CV", o.folds));
    }
    let work = o.out.join("work").join(&name);
    let _ = std::fs::remove_dir_all(&work);
    let fail = |doc: Json, e: String| {
        if !o.keep_work {
            let _ = std::fs::remove_dir_all(&work);
        }
        doc.with("status", "error").with("error", e)
    };
    let k = o.folds.clamp(2, n);
    let fold = random_folds(o.seed, &name, n, k);
    let (pred, basep, imp) = match kfold(o, &d, &d.y, &fold, k, &work.join("random"), threads) {
        Ok(r) => r,
        Err(e) => return fail(doc, e),
    };
    let (r, _, _, random) = scores(&d.y, &pred, &basep);
    doc.set("status", "ok");
    for (key, value) in random.as_obj().unwrap_or(&[]) {
        doc.set(key, value.clone());
    }
    doc.set("verdict_random", random.get("verdict").cloned().unwrap_or(Json::Null));
    doc.set(
        "top_features",
        Json::Arr(imp.iter().take(10).map(|(f, v)| Json::obj().with("feature", f.as_str()).with("importance", *v)).collect()),
    );
    // Lineage-grouped k-fold: siblings / DOE neighbours / one GA run are held out together.
    let mut grouped_pred: Option<(Vec<f64>, Vec<usize>)> = None;
    match d.groups.as_deref().and_then(|g| group_folds(g, o.folds, o.seed).map(|f| (f, g))) {
        Some(((gfold, gk), groups)) => match kfold(o, &d, &d.y, &gfold, gk, &work.join("grouped"), threads) {
            Ok((gp, gb, _)) => {
                let (_, _, _, gdoc) = scores(&d.y, &gp, &gb);
                let n_groups = groups.iter().collect::<std::collections::BTreeSet<_>>().len();
                doc.set("verdict", gdoc.get("verdict").cloned().unwrap_or(Json::Null));
                doc.set("strength", gdoc.get("strength").cloned().unwrap_or(Json::Null));
                doc.set("verdict_basis", "grouped");
                doc.set("grouped", gdoc.with("folds", gk).with("n_groups", n_groups));
                grouped_pred = Some((gp, gfold));
            }
            Err(e) => return fail(doc, format!("grouped CV: {e}")),
        },
        None => {
            doc.set("verdict_basis", "random");
            doc.set("grouped", Json::Null);
        }
    }
    if o.permutations > 0 {
        let mut null_r = Vec::new();
        let mut rng = SplitMix(o.seed ^ 0xA5A5_5A5A ^ stats::fnv1a(name.as_bytes()));
        for p in 0..o.permutations {
            let perm = rng.permutation(n);
            let ys: Vec<f64> = perm.iter().map(|&i| d.y[i]).collect();
            match kfold(o, &d, &ys, &fold, k, &work.join(format!("perm{p}")), threads) {
                Ok((pp, _, _)) => null_r.push(stats::pearson(&ys, &pp)),
                Err(e) => return fail(doc, format!("permutation {p}: {e}")),
            }
        }
        let exceed = null_r.iter().filter(|x| x.is_finite() && **x >= r).count();
        doc.set("permutation_null_r", Json::Arr(null_r.iter().map(|x| Json::from(*x)).collect()));
        doc.set("permutation_p", (1 + exceed) as f64 / (1 + null_r.len()) as f64);
    }
    let mut oof = String::from("row,y,yhat,fold,yhat_grouped,fold_grouped\n");
    for i in 0..n {
        let (gp, gf) = match &grouped_pred {
            Some((gp, gf)) => (csvio::sig7(gp[i]), gf[i].to_string()),
            None => (String::new(), String::new()),
        };
        oof.push_str(&format!("{i},{},{},{},{gp},{gf}\n", csvio::sig7(d.y[i]), csvio::sig7(pred[i]), fold[i]));
    }
    let _ = std::fs::write(o.out.join(format!("{name}.oof.csv")), oof);
    if !o.keep_work {
        let _ = std::fs::remove_dir_all(&work);
    }
    doc.set("seconds", (started.elapsed().as_secs_f64() * 10.0).round() / 10.0);
    doc
}

pub struct CvSummary {
    pub results: Vec<Json>,
    pub errors: usize,
}

pub fn run(o: &CvOptions) -> Result<CvSummary, String> {
    let tables = list_tables(&o.tables, o.only.as_deref());
    if tables.is_empty() {
        return Err(format!("no tables in {}", o.tables.display()));
    }
    std::fs::create_dir_all(&o.out).map_err(|e| format!("{}: {e}", o.out.display()))?;
    let jobs = o.jobs.clamp(1, tables.len());
    let per_job_threads = (o.threads / jobs).max(1);
    let queue = Arc::new(Mutex::new(tables.into_iter().collect::<VecDeque<_>>()));
    let (tx, rx) = mpsc::channel::<Json>();
    let mut handles = Vec::new();
    for _ in 0..jobs {
        let q = Arc::clone(&queue);
        let tx = tx.clone();
        let o = o.clone();
        handles.push(std::thread::spawn(move || loop {
            let next = q.lock().ok().and_then(|mut g| g.pop_front());
            let Some(path) = next else { break };
            let doc = evaluate_table(&o, &path, per_job_threads);
            let name = doc.get("dataset").and_then(Json::as_str).unwrap_or("").to_string();
            let _ = std::fs::write(o.out.join(format!("{name}.cv.json")), doc.pretty());
            eprintln!(
                "dataset cv: {name} -> {} r={} lift={}",
                doc.get("status").and_then(Json::as_str).unwrap_or("?"),
                doc.get("pearson_r").and_then(Json::as_f64).map(|x| format!("{x:.4}")).unwrap_or_else(|| "-".into()),
                doc.get("verdict").and_then(Json::as_str).unwrap_or("-")
            );
            if tx.send(doc).is_err() {
                break;
            }
        }));
    }
    drop(tx);
    let mut results: Vec<Json> = rx.iter().collect();
    for h in handles {
        let _ = h.join();
    }
    let _ = std::fs::remove_dir_all(o.out.join("work"));
    results.sort_by(|a, b| rank_key(b).total_cmp(&rank_key(a)).then_with(|| name_of(a).cmp(name_of(b))));
    let errors = results.iter().filter(|r| r.get("status").and_then(Json::as_str) != Some("ok")).count();
    write_leaderboard(o, &results)?;
    Ok(CvSummary { results, errors })
}

fn name_of(j: &Json) -> &str {
    j.get("dataset").and_then(Json::as_str).unwrap_or("")
}

/// The scores the verdict rests on: the grouped ones when grouped CV ran, else the random ones.
fn basis(j: &Json) -> &Json {
    match j.get("grouped") {
        Some(g @ Json::Obj(_)) => g,
        _ => j,
    }
}

fn rank_key(j: &Json) -> f64 {
    let v = match j.get("verdict").and_then(Json::as_str) {
        Some("lift") => 2.0,
        Some("weak") => 1.0,
        _ => 0.0,
    };
    let r = basis(j).get("pearson_r").and_then(Json::as_f64).filter(|x| x.is_finite()).unwrap_or(-2.0);
    v * 10.0 + r
}

fn f3(j: &Json, k: &str) -> String {
    j.get(k).and_then(Json::as_f64).filter(|x| x.is_finite()).map(|x| format!("{x:.3}")).unwrap_or_else(|| "–".into())
}

pub fn leaderboard_markdown(results: &[Json], o: &CvOptions, limit: usize) -> String {
    let count = |v: &str| results.iter().filter(|r| r.get("verdict").and_then(Json::as_str) == Some(v)).count();
    let errors = results.iter().filter(|r| r.get("status").and_then(Json::as_str) != Some("ok")).count();
    let random_lift = results.iter().filter(|r| r.get("verdict_random").and_then(Json::as_str) == Some("lift")).count();
    let strong = results.iter().filter(|r| r.get("strength").and_then(Json::as_str) == Some("strong")).count();
    let grouped = results.iter().filter(|r| matches!(r.get("grouped"), Some(Json::Obj(_)))).count();
    let mut md = format!(
        "## Regression dataset lift (CatBoost {}-fold out-of-fold)\n\n**{}** tables · **{}** lift (**{}** strong, R² ≥ 0.5) · {} weak · {} none · {} errors \
(random k-fold alone: {} lift; {} tables also ran lineage-grouped k-fold)\n\n\
`lift` = OOF Pearson r > 0 with its 95 % CI above 0 **and** RMSE below the fold-mean baseline; strength = OOF R² (strong ≥ 0.5, moderate ≥ 0.2, else marginal). \
The verdict uses **lineage-grouped** folds when a table has ≥ 3 lineage groups (DOE parent, GA run + family, or gen-800 run held out together), \
otherwise random folds. CatBoost: {} iterations, depth {}, lr {}, seed {}, ≤ {} rows/table.\n\n\
| # | table | n | p | groups | r random | r grouped (95 % CI) | R² | lift % | verdict | top features |\n|---:|---|---:|---:|---:|---:|---|---:|---:|---|---|\n",
        o.folds,
        results.len(),
        count("lift"),
        strong,
        count("weak"),
        count("none"),
        errors,
        random_lift,
        grouped,
        o.iterations,
        o.depth,
        o.learning_rate,
        o.seed,
        if o.max_rows == 0 { "all".to_string() } else { o.max_rows.to_string() },
    );
    for (i, r) in results.iter().filter(|r| r.get("status").and_then(Json::as_str) == Some("ok")).take(limit).enumerate() {
        let b = basis(r);
        let ci = b.get("pearson_r_ci95").and_then(Json::as_arr).map(|a| {
            let g = |k: usize| a.get(k).and_then(Json::as_f64).filter(|x| x.is_finite()).map(|x| format!("{x:.3}")).unwrap_or_else(|| "–".into());
            format!("[{}, {}]", g(0), g(1))
        });
        let g = r.get("grouped").filter(|g| matches!(g, Json::Obj(_)));
        let top: Vec<String> = r
            .get("top_features")
            .and_then(Json::as_arr)
            .map(|a| a.iter().take(3).filter_map(|t| t.get("feature").and_then(Json::as_str)).map(|s| format!("`{s}`")).collect())
            .unwrap_or_else(Vec::new);
        md.push_str(&format!(
            "| {} | `{}` | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            i + 1,
            name_of(r),
            r.get("n_rows").and_then(Json::as_f64).unwrap_or(0.0),
            r.get("n_features").and_then(Json::as_f64).unwrap_or(0.0),
            g.and_then(|g| g.get("n_groups")).and_then(Json::as_f64).map(|x| x.to_string()).unwrap_or_else(|| "–".into()),
            f3(r, "pearson_r"),
            match g {
                Some(g) => format!("{} {}", f3(g, "pearson_r"), ci.unwrap_or_else(String::new)),
                None => "–".into(),
            },
            f3(b, "r2_oof"),
            b.get("lift_pct").and_then(Json::as_f64).filter(|x| x.is_finite()).map(|x| format!("{x:.1}")).unwrap_or_else(|| "–".into()),
            match (r.get("verdict").and_then(Json::as_str), r.get("strength").and_then(Json::as_str)) {
                (Some(v), Some(s)) if s != "-" => format!("{v} ({s})"),
                (Some(v), _) => v.to_string(),
                _ => "–".to_string(),
            },
            top.join(" ")
        ));
    }
    let errs: Vec<&Json> = results.iter().filter(|r| r.get("status").and_then(Json::as_str) != Some("ok")).collect();
    if !errs.is_empty() {
        md.push_str("\n### Errors\n\n");
        for e in errs {
            md.push_str(&format!("- `{}`: {}\n", name_of(e), e.get("error").and_then(Json::as_str).unwrap_or("?")));
        }
    }
    md
}

fn write_leaderboard(o: &CvOptions, results: &[Json]) -> Result<(), String> {
    let doc = Json::obj()
        .with("schema", "jarambit.regression.leaderboard.v1")
        .with(
            "config",
            Json::obj()
                .with("folds", o.folds)
                .with("seed", o.seed)
                .with("iterations", o.iterations)
                .with("learning_rate", o.learning_rate)
                .with("depth", o.depth)
                .with("max_rows", o.max_rows)
                .with("permutations", o.permutations)
                .with("lift_rule", "OOF pearson r > 0, Fisher 95% CI lower bound > 0, and RMSE < fold-mean baseline RMSE")
                .with("verdict_basis", "lineage-grouped k-fold when a table has >= 3 groups (<table>.groups), else random k-fold"),
        )
        .with("results", Json::Arr(results.to_vec()));
    std::fs::write(o.out.join("leaderboard.json"), doc.pretty()).map_err(|e| e.to_string())?;
    std::fs::write(o.out.join("leaderboard.md"), leaderboard_markdown(results, o, usize::MAX)).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// End-to-end against the real CatBoost CLI when `JARAMBIT_CATBOOST` is set; otherwise exercises
    /// the fail-closed path only.
    #[test]
    fn kfold_cv_finds_lift_on_a_learnable_table_and_none_on_noise() {
        let root = std::env::temp_dir().join(format!("jarambit-cv-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let tables = root.join("tables");
        std::fs::create_dir_all(&tables).unwrap();
        let mut rng = SplitMix(7);
        let mut learn = String::from("f.a,f.b,f.c,s1.y\n");
        let mut noise = String::from("f.a,f.b,s1.z\n");
        for _ in 0..300 {
            let a = (rng.next_u64() % 10_000) as f64 / 10_000.0;
            let b = (rng.next_u64() % 10_000) as f64 / 10_000.0;
            let c = (rng.next_u64() % 10_000) as f64 / 10_000.0;
            let e = (rng.next_u64() % 10_000) as f64 / 100_000.0;
            learn.push_str(&format!("{a},{b},{c},{}\n", 3.0 * a + b * b + e));
            noise.push_str(&format!("{a},{b},{}\n", (rng.next_u64() % 10_000) as f64));
        }
        std::fs::write(tables.join("s1.y__t.csv"), learn).unwrap();
        std::fs::write(tables.join("s1.z__t.csv"), noise).unwrap();
        let Ok(cb) = catboost_bin(None) else {
            assert!(catboost_bin(Some("/nonexistent/catboost")).is_err());
            return;
        };
        let mut o = CvOptions::defaults(tables, root.join("cv"), cb);
        o.iterations = 150;
        o.jobs = 2;
        o.threads = 2;
        let s = run(&o).unwrap();
        assert_eq!(s.errors, 0, "{:?}", s.results);
        let get = |n: &str| s.results.iter().find(|r| name_of(r) == n).unwrap().clone();
        let y = get("s1.y__t");
        assert_eq!(y.get("verdict").and_then(Json::as_str), Some("lift"));
        assert_eq!(y.get("strength").and_then(Json::as_str), Some("strong"));
        assert!(y.get("pearson_r").and_then(Json::as_f64).unwrap() > 0.9);
        assert_eq!(y.get("top_features").unwrap().as_arr().unwrap()[0].get("feature").and_then(Json::as_str), Some("f.a"));
        let z = get("s1.z__t");
        assert_ne!(z.get("verdict").and_then(Json::as_str), Some("lift"));
        assert!(o.out.join("leaderboard.md").is_file() && o.out.join("s1.y__t.oof.csv").is_file());
        assert_eq!(name_of(&s.results[0]), "s1.y__t", "lift ranks first");
        assert_eq!(y.get("verdict_basis").and_then(Json::as_str), Some("random"), "no .groups sidecar");
    }

    #[test]
    fn group_folds_hold_lineages_out_together_and_balance_rows() {
        let groups: Vec<String> = (0..60).map(|i| format!("g{}", i % 7)).collect();
        let (fold, k) = group_folds(&groups, 5, 3).unwrap();
        assert_eq!(k, 5);
        for g in 0..7 {
            let fs: std::collections::BTreeSet<usize> = (0..60).filter(|i| i % 7 == g).map(|i| fold[i]).collect();
            assert_eq!(fs.len(), 1, "group g{g} split across folds");
        }
        let sizes: Vec<usize> = (0..k).map(|f| fold.iter().filter(|x| **x == f).count()).collect();
        assert!(sizes.iter().all(|s| *s > 0) && sizes.iter().max().unwrap() - sizes.iter().min().unwrap() <= 9, "{sizes:?}");
        assert_eq!(group_folds(&["a".to_string(), "b".to_string(), "a".to_string()], 5, 1), None);
        let (_, k3) = group_folds(&["a".to_string(), "b".to_string(), "c".to_string()], 5, 1).unwrap();
        assert_eq!(k3, 3);
        assert_eq!(random_folds(1, "t", 10, 5), random_folds(1, "t", 10, 5));
    }

    /// A target that is a per-lineage constant, with a feature that only fingerprints the lineage:
    /// random folds see every lineage in training (looks predictable), grouped folds do not.
    #[test]
    fn grouped_cv_removes_lineage_memorisation_lift() {
        let Ok(cb) = catboost_bin(None) else { return };
        let root = std::env::temp_dir().join(format!("jarambit-cv-grouped-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let tables = root.join("tables");
        std::fs::create_dir_all(&tables).unwrap();
        let mut rng = SplitMix(11);
        let offsets: Vec<f64> = (0..12).map(|_| (rng.next_u64() % 1000) as f64).collect();
        let mut csv = String::from("f.lineage_code,f.noise,s1.t\n");
        let mut groups = String::new();
        for i in 0..240 {
            let g = i % 12;
            let code = (g * 7919 % 101) as f64; // arbitrary, unordered fingerprint
            let noise = (rng.next_u64() % 1000) as f64 / 1000.0;
            csv.push_str(&format!("{code},{noise},{}\n", offsets.get(g).copied().unwrap_or(0.0) + noise));
            groups.push_str(&format!("run:{g}\n"));
        }
        std::fs::write(tables.join("s1.t__g.csv"), csv).unwrap();
        std::fs::write(tables.join("s1.t__g.groups"), groups).unwrap();
        let mut o = CvOptions::defaults(tables, root.join("cv"), cb);
        o.iterations = 150;
        o.jobs = 1;
        o.threads = 2;
        let s = run(&o).unwrap();
        let t = s.results.first().unwrap();
        assert_eq!(t.get("verdict_random").and_then(Json::as_str), Some("lift"), "random folds memorise lineages");
        assert_eq!(t.get("verdict_basis").and_then(Json::as_str), Some("grouped"));
        assert_ne!(t.get("verdict").and_then(Json::as_str), Some("lift"), "{}", t.pretty());
        assert_eq!(t.get("grouped").and_then(|g| g.get("n_groups")).and_then(Json::as_f64), Some(12.0));
    }
}
