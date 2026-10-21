//! Group-held-out CatBoost CLI runner. Outer test labels never reach fit.
use crate::{
    assemble::{read_snapshot, RowMeta},
    hash, store, Result,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub folds: usize,
    pub seed: u64,
    pub iterations: usize,
    pub depth: usize,
    pub threads: usize,
    pub min_rows: usize,
    pub min_test_rows: usize,
    pub timeout_seconds: u64,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            folds: 5,
            seed: 20261020,
            iterations: 500,
            depth: 6,
            threads: 2,
            min_rows: 200,
            min_test_rows: 20,
            timeout_seconds: 600,
        }
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct Metrics {
    pub n: usize,
    pub pearson_r: Option<f64>,
    pub r2: Option<f64>,
    pub rmse: f64,
    pub mae: f64,
    pub baseline_rmse: f64,
    pub rmse_lift: Option<f64>,
    pub undefined_reasons: Vec<String>,
}
pub fn metrics(y: &[f64], p: &[f64], baseline: &[f64]) -> Result<Metrics> {
    if y.is_empty()
        || y.len() != p.len()
        || y.len() != baseline.len()
        || y.iter().chain(p).chain(baseline).any(|x| !x.is_finite())
    {
        return Err("invalid prediction vectors".into());
    }
    let n = y.len() as f64;
    let ym = y.iter().sum::<f64>() / n;
    let pm = p.iter().sum::<f64>() / n;
    let yy = y.iter().map(|v| (v - ym).powi(2)).sum::<f64>();
    let pp = p.iter().map(|v| (v - pm).powi(2)).sum::<f64>();
    let cov = y
        .iter()
        .zip(p)
        .map(|(a, b)| (a - ym) * (b - pm))
        .sum::<f64>();
    let se = y.iter().zip(p).map(|(a, b)| (a - b).powi(2)).sum::<f64>();
    let rmse = (se / n).sqrt();
    let baseline_rmse = (y
        .iter()
        .zip(baseline)
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f64>()
        / n)
        .sqrt();
    let values = [ym, pm, yy, pp, cov, se, rmse, baseline_rmse];
    if values.iter().any(|v| !v.is_finite()) {
        return Err("metric overflow".into());
    }
    let result = Metrics {
        n: y.len(),
        pearson_r: if yy > 0.0 && pp > 0.0 {
            Some((cov / yy.sqrt() / pp.sqrt()).clamp(-1.0, 1.0))
        } else {
            None
        },
        r2: if yy > 0.0 { Some(1.0 - se / yy) } else { None },
        rmse,
        mae: y.iter().zip(p).map(|(a, b)| (a - b).abs()).sum::<f64>() / n,
        baseline_rmse,
        rmse_lift: if baseline_rmse > 0.0 {
            Some(1.0 - rmse / baseline_rmse)
        } else {
            None
        },
        undefined_reasons: [
            (yy == 0.0, "constant_target"),
            (pp == 0.0, "constant_predictions"),
            (baseline_rmse == 0.0, "zero_baseline_error"),
        ]
        .into_iter()
        .filter(|(yes, _)| *yes)
        .map(|(_, why)| why.into())
        .collect(),
    };
    if [
        result.pearson_r,
        result.r2,
        result.rmse_lift,
        Some(result.mae),
    ]
    .into_iter()
    .flatten()
    .any(|v| !v.is_finite())
    {
        return Err("metric ratio overflow".into());
    }
    Ok(result)
}
/// Largest groups first; deterministic hashed tie breaks, balanced without looking at targets.
pub fn folds(rows: &[RowMeta], k: usize, seed: u64) -> Result<Vec<usize>> {
    if !(2..=20).contains(&k) {
        return Err("fold count must be 2..20".into());
    }
    let mut groups = BTreeMap::<&str, Vec<usize>>::new();
    for (i, row) in rows.iter().enumerate() {
        groups.entry(&row.group_id).or_default().push(i);
    }
    if groups.len() < k {
        return Err("insufficient independent groups".into());
    }
    let mut groups: Vec<_> = groups.into_iter().collect();
    groups.sort_by_key(|(id, v)| {
        (
            std::cmp::Reverse(v.len()),
            hash(format!("{seed}:{id}").as_bytes()),
        )
    });
    let mut sizes = vec![0; k];
    let mut assignment = vec![usize::MAX; rows.len()];
    for (_, indices) in groups {
        let fold = (0..k).min_by_key(|&i| (sizes[i], i)).unwrap();
        sizes[fold] += indices.len();
        for i in indices {
            assignment[i] = fold;
        }
    }
    Ok(assignment)
}
fn run(mut command: Command, seconds: u64, log: &Path) -> Result<()> {
    command
        .stdin(Stdio::null())
        .stdout(std::fs::File::create(log)?)
        .stderr(std::fs::File::create(log.with_extension("stderr"))?);
    let mut child = command.spawn()?;
    let started = Instant::now();
    loop {
        if let Some(status) = child.try_wait()? {
            return if status.success() {
                Ok(())
            } else {
                Err(format!("CatBoost failed ({status}); see {}", log.display()).into())
            };
        }
        if started.elapsed() > Duration::from_secs(seconds) {
            let _ = child.kill();
            let _ = child.wait();
            return Err("CatBoost timed out".into());
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}
#[derive(Debug, Serialize)]
pub struct Report {
    pub schema: u32,
    pub status: String,
    pub reason: Option<String>,
    pub target: String,
    pub context_id: String,
    pub feature_schema: String,
    pub fidelity: String,
    pub unit: String,
    pub excluded_missing_target: usize,
    pub duplicate_designs: usize,
    pub snapshot_sha256: String,
    pub catboost_sha256: String,
    pub config: Config,
    pub rows: usize,
    pub groups: usize,
    pub pooled: Option<Metrics>,
    pub per_fold: Vec<Metrics>,
    pub positive_lift_folds: usize,
    pub oof_sha256: Option<String>,
    pub split_sha256: Option<String>,
    pub interpretation: String,
}
pub fn evaluate(snapshot: &Path, binary: &Path, output: &Path, config: &Config) -> Result<Report> {
    if !(2..=20).contains(&config.folds)
        || config.iterations == 0
        || config.iterations > 5000
        || config.depth == 0
        || config.depth > 10
        || config.threads == 0
        || config.threads > 8
        || config.timeout_seconds == 0
        || config.min_rows < 2
        || config.min_test_rows < 2
    {
        return Err("invalid/beyond-budget CV configuration".into());
    }
    let (m, rows, meta) = read_snapshot(snapshot)?;
    let binary = std::fs::canonicalize(binary)?;
    let mut report=Report {schema:1,status:"insufficient_data".into(),reason:None,target:m.target.clone(),context_id:m.context_id.clone(),feature_schema:m.feature_schema.clone(),fidelity:m.fidelity.clone(),unit:m.unit.clone(),excluded_missing_target:m.excluded_missing_target,duplicate_designs:m.duplicate_designs,snapshot_sha256:hash(&std::fs::read(snapshot.join("manifest.json"))?),catboost_sha256:hash(&std::fs::read(&binary)?),config:config.clone(),rows:m.rows,groups:m.groups,pooled:None,per_fold:vec![],positive_lift_folds:0,oof_sha256:None,split_sha256:None,interpretation:"Observed group-held-out predictability; not physical validation, certification, or proof of search improvement.".into()};
    let y: Vec<f64> = rows
        .iter()
        .map(|r| r.last().unwrap().parse::<f64>())
        .collect::<std::result::Result<_, _>>()?;
    let reason = if m.rows < config.min_rows {
        Some("below minimum rows")
    } else if m.groups < config.folds {
        Some("below minimum independent groups")
    } else if y.iter().all(|v| *v == y[0]) {
        Some("constant target")
    } else {
        None
    };
    if let Some(reason) = reason {
        report.reason = Some(reason.into());
        save(output, &report)?;
        return Ok(report);
    }
    let split = folds(&meta, config.folds, config.seed)?;
    if (0..config.folds).any(|f| split.iter().filter(|&&v| v == f).count() < config.min_test_rows) {
        report.reason = Some("a test fold is too small".into());
        save(output, &report)?;
        return Ok(report);
    }
    // Fresh output directory: models/predictions from a previous attempt can never be reused.
    std::fs::create_dir_all(output)?;
    let work = output.join("work");
    std::fs::create_dir(&work)?;
    let mut cd = String::new();
    for (i, feature) in m.features.iter().enumerate() {
        cd += &format!(
            "{i}\t{}\t{feature}\n",
            if m.categorical.contains(&i) {
                "Categ"
            } else {
                "Num"
            }
        );
    }
    cd += &format!("{}\tLabel\ttarget\n", m.features.len());
    let split_bytes = serde_json::to_vec_pretty(&split)?;
    store::put(&output.join("folds.json"), &split_bytes)?;
    report.split_sha256 = Some(hash(&split_bytes));
    let mut prediction = vec![f64::NAN; y.len()];
    let mut baseline = vec![f64::NAN; y.len()];
    for fold in 0..config.folds {
        let dir = work.join(fold.to_string());
        std::fs::create_dir(&dir)?;
        let dir = std::fs::canonicalize(dir)?;
        std::fs::write(dir.join("columns.cd"), &cd)?;
        let train: Vec<_> = (0..y.len()).filter(|&i| split[i] != fold).collect();
        let test: Vec<_> = (0..y.len()).filter(|&i| split[i] == fold).collect();
        if train.len() < 2 || train.iter().all(|&i| y[i] == y[train[0]]) {
            return Err("constant/insufficient outer training fold".into());
        }
        let mean = train.iter().map(|&i| y[i]).sum::<f64>() / train.len() as f64;
        let data = |indices: &[usize]| {
            indices
                .iter()
                .map(|&i| rows[i].join("\t") + "\n")
                .collect::<String>()
        };
        std::fs::write(dir.join("train.tsv"), data(&train))?;
        // Strip hidden outer labels even from calc inputs: retain schema with dummy label 0.
        let test_data = test
            .iter()
            .map(|&i| rows[i][..m.features.len()].join("\t") + "\t0\n")
            .collect::<String>();
        std::fs::write(dir.join("test.tsv"), test_data)?;
        let mut fit = Command::new(&binary);
        fit.current_dir(&dir).args([
            "fit",
            "--learn-set",
            "train.tsv",
            "--column-description",
            "columns.cd",
            "--loss-function",
            "RMSE",
            "--iterations",
            &config.iterations.to_string(),
            "--depth",
            &config.depth.to_string(),
            "--learning-rate",
            "0.05",
            "--random-seed",
            &config.seed.to_string(),
            "--thread-count",
            &config.threads.to_string(),
            "--task-type",
            "CPU",
            "--allow-writing-files",
            "false",
            "--model-file",
            "model.cbm",
        ]);
        run(fit, config.timeout_seconds, &dir.join("fit.log"))?;
        let mut calc = Command::new(&binary);
        calc.current_dir(&dir).args([
            "calc",
            "--model-file",
            "model.cbm",
            "--input-path",
            "test.tsv",
            "--column-description",
            "columns.cd",
            "--output-columns",
            "RawFormulaVal",
            "--output-path",
            "predictions.tsv",
            "--thread-count",
            &config.threads.to_string(),
        ]);
        run(calc, config.timeout_seconds, &dir.join("calc.log"))?;
        let raw = std::fs::read_to_string(dir.join("predictions.tsv"))?;
        let mut lines = raw.lines();
        if lines.next() != Some("RawFormulaVal") {
            return Err("unexpected CatBoost prediction header".into());
        }
        let p: Vec<f64> = lines
            .map(str::parse)
            .collect::<std::result::Result<_, _>>()?;
        if p.len() != test.len() || p.iter().any(|v| !v.is_finite()) {
            return Err("prediction count/nonfinite output".into());
        }
        for (&i, &p) in test.iter().zip(&p) {
            prediction[i] = p;
            baseline[i] = mean;
        }
        let fold_metrics = metrics(
            &test.iter().map(|&i| y[i]).collect::<Vec<_>>(),
            &p,
            &vec![mean; p.len()],
        )?;
        if fold_metrics.rmse_lift.is_some_and(|x| x > 0.0) {
            report.positive_lift_folds += 1;
        }
        report.per_fold.push(fold_metrics);
    }
    report.pooled = Some(metrics(&y, &prediction, &baseline)?);
    report.status = "complete".into();
    let mut oof = "row,fold,target,prediction,train_mean\n".to_string();
    for i in 0..y.len() {
        oof += &format!(
            "{i},{},{},{},{}\n",
            split[i], y[i], prediction[i], baseline[i]
        );
    }
    store::put(&output.join("oof.csv"), oof.as_bytes())?;
    report.oof_sha256 = Some(hash(oof.as_bytes()));
    save(output, &report)?;
    Ok(report)
}
fn save(output: &Path, report: &Report) -> Result<()> {
    let optional = |v: Option<f64>| {
        v.map(|v| format!("{v:.4}"))
            .unwrap_or_else(|| "undefined".into())
    };
    let mut md = format!("### {}\n\nFidelity: `{}`; unit: `{}`; context: `{}`. Excluded missing targets: {}; repeated designs removed: {}.\n\n", report.target, report.fidelity, report.unit, report.context_id, report.excluded_missing_target, report.duplicate_designs);
    if let Some(m) = &report.pooled {
        md += &format!("| Rows | Groups | OOF Pearson r | OOF R² | RMSE | Mean-baseline RMSE | RMSE lift (%) | Positive folds |\n|---:|---:|---:|---:|---:|---:|---:|---:|\n| {} | {} | {} | {} | {:.6} | {:.6} | {} | {}/{} |\n",report.rows,report.groups,optional(m.pearson_r),optional(m.r2),m.rmse,m.baseline_rmse,optional(m.rmse_lift.map(|x|100.0*x)),report.positive_lift_folds,report.config.folds);
    } else {
        md += &format!(
            "{} ({}) — {} rows, {} groups.\n",
            report.status,
            report.reason.as_deref().unwrap_or("unknown"),
            report.rows,
            report.groups
        );
    }
    store::put(
        &output.join("summary.md"),
        (md + "\n" + &report.interpretation + "\n").as_bytes(),
    )?;
    store::put(
        &output.join("report.json"),
        &serde_json::to_vec_pretty(report)?,
    )
}
