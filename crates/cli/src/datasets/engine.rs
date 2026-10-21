//! Pure-std command layer shared by `jarambit dataset ...` and the standalone CI binary.
//!
//! Commands: `build`, `cv`, `harvest-waves`, `commit`. Commands that need the CAD kernel
//! (`featurize`, `refeaturize`, `rescreen`, `add-physical`) live in `datasets::mod` (jarambit only).
use super::cv;
use super::gitsync;
use super::harvest;
use super::materialize;
use std::path::{Path, PathBuf};

pub const DEFAULT_RECORDS: &str = "datasets/regression/records";
pub const DEFAULT_RECIPES: &str = "datasets/regression/recipes.conf";
pub const DEFAULT_TABLES: &str = "datasets/regression/tables";
pub const DEFAULT_CV: &str = "datasets/regression/cv";

pub struct Args<'a>(pub &'a [String]);

impl Args<'_> {
    pub fn get(&self, k: &str) -> Option<String> {
        self.0.iter().position(|x| x == k).and_then(|i| self.0.get(i + 1)).cloned()
    }
    pub fn flag(&self, k: &str) -> bool {
        self.0.iter().any(|x| x == k)
    }
    pub fn num<T: std::str::FromStr>(&self, k: &str, default: T) -> Result<T, String> {
        match self.get(k) {
            None => Ok(default),
            Some(v) => v.parse().map_err(|_| format!("{k}: cannot parse {v:?}")),
        }
    }
}

/// Repository root: `--root`, else the nearest ancestor of the cwd holding `docs/VERSION`, else cwd.
pub fn repo_root(a: &Args) -> PathBuf {
    if let Some(r) = a.get("--root") {
        return PathBuf::from(r);
    }
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut p: &Path = &cwd;
    loop {
        if p.join("docs/VERSION").is_file() {
            return p.to_path_buf();
        }
        match p.parent() {
            Some(q) => p = q,
            None => return cwd,
        }
    }
}

fn under(root: &Path, v: Option<String>, default: &str) -> PathBuf {
    let p = PathBuf::from(v.unwrap_or_else(|| default.to_string()));
    if p.is_absolute() {
        p
    } else {
        root.join(p)
    }
}

pub const USAGE: &str = "\
jarambit dataset <command> [options]

  build          materialize regression tables (final column = target)
                 [--recipes datasets/regression/recipes.conf] [--out datasets/regression/tables]
                 [--only SUBSTR] [--max-rows N]
  cv             CatBoost k-fold CV of every table; writes leaderboard.{json,md}
                 [--tables DIR] [--out datasets/regression/cv] [--catboost PATH] [--folds 5]
                 [--seed N] [--iterations 500] [--depth 6] [--lr 0.05] [--threads T] [--jobs J]
                 [--max-rows 50000] [--only SUBSTR] [--permutations P] [--timeout SECONDS]
                 [--keep-work] [--summary FILE] [--fail-on-error]
  harvest-waves  convert retained auto-evolve outputs (births/screen/leaderboard JSON) to records
                 [--evidence evidence/evolution_waves] [--out datasets/regression/records/harvest_waves]
  commit         git-commit record paths only  --paths P1,P2 [--message M] [--push REMOTE:BRANCH]
  featurize      (jarambit only) print the CAD feature row of a genome CSV  --genome FILE
  refeaturize    (jarambit only) add CAD features to records that only carry genes
                 [--records DIR] [--threads T]
  rescreen       (jarambit only) run the Stage-1 FEM screen on genomes and record them
                 [--genomes FILE] [--neighbors N --sigma 0.05] [--random N] [--seed S]
                 [--threads T] [--area 2.0] [--tag NAME] [--commit]
  add-physical   (jarambit only) import physical test results  --csv FILE --campaign NAME
All paths are relative to the repository root (--root DIR overrides discovery).";

pub fn run_pure(cmd: &str, rest: &[String]) -> Option<i32> {
    let a = Args(rest);
    let root = repo_root(&a);
    let result = match cmd {
        "build" => cmd_build(&root, &a),
        "cv" => cmd_cv(&root, &a),
        "harvest-waves" => cmd_harvest(&root, &a),
        "commit" => cmd_commit(&root, &a),
        _ => return None,
    };
    Some(match result {
        Ok(code) => code,
        Err(e) => {
            eprintln!("dataset {cmd}: {e}");
            2
        }
    })
}

fn cmd_build(root: &Path, a: &Args) -> Result<i32, String> {
    let max_rows = match a.get("--max-rows") {
        Some(v) => Some(v.parse::<usize>().map_err(|_| format!("--max-rows {v:?}"))?),
        None => None,
    };
    let opts = materialize::BuildOptions {
        recipes: under(root, a.get("--recipes"), DEFAULT_RECIPES),
        out: under(root, a.get("--out"), DEFAULT_TABLES),
        only: a.get("--only"),
        max_rows,
        root: root.to_path_buf(),
    };
    let s = materialize::build(&opts)?;
    let sources = s.index.get("sources").and_then(|v| v.as_arr()).map(|a| a.len()).unwrap_or(0);
    println!("dataset build: {} tables from {sources} source frames written to {} ({} specs skipped)", s.built, opts.out.display(), s.skipped);
    Ok(if s.built > 0 { 0 } else { 3 })
}

fn cmd_cv(root: &Path, a: &Args) -> Result<i32, String> {
    let cb = cv::catboost_bin(a.get("--catboost").as_deref())?;
    let mut o = cv::CvOptions::defaults(under(root, a.get("--tables"), DEFAULT_TABLES), under(root, a.get("--out"), DEFAULT_CV), cb);
    o.folds = a.num("--folds", o.folds)?.max(2);
    o.seed = a.num("--seed", o.seed)?;
    o.iterations = a.num("--iterations", o.iterations)?.max(1);
    o.depth = a.num("--depth", o.depth)?.clamp(1, 16);
    o.learning_rate = a.num("--lr", o.learning_rate)?;
    o.threads = a.num("--threads", o.threads)?.max(1);
    o.jobs = a.num("--jobs", o.jobs)?.max(1);
    o.max_rows = a.num("--max-rows", o.max_rows)?;
    o.permutations = a.num("--permutations", o.permutations)?;
    o.timeout_s = a.num("--timeout", o.timeout_s)?.max(1);
    o.only = a.get("--only");
    o.keep_work = a.flag("--keep-work");
    let s = cv::run(&o)?;
    let md = cv::leaderboard_markdown(&s.results, &o, 60);
    println!("{md}");
    if let Some(p) = a.get("--summary") {
        use std::io::Write as _;
        let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&p).map_err(|e| format!("{p}: {e}"))?;
        f.write_all(md.as_bytes()).map_err(|e| e.to_string())?;
    }
    Ok(if s.errors > 0 && a.flag("--fail-on-error") { 4 } else { 0 })
}

fn cmd_harvest(root: &Path, a: &Args) -> Result<i32, String> {
    let evidence = under(root, a.get("--evidence"), "evidence/evolution_waves");
    let out = under(root, a.get("--out"), &format!("{DEFAULT_RECORDS}/harvest_waves"));
    let rep = harvest::harvest_waves(&evidence, root, &out)?;
    println!(
        "harvest-waves: {} runs, {} births ({} screened), {} assessed ({} unmatched), {} shards -> {}",
        rep.runs,
        rep.births,
        rep.screened,
        rep.assessed,
        rep.unmatched_assessments,
        rep.shards.len(),
        out.display()
    );
    Ok(0)
}

fn cmd_commit(root: &Path, a: &Args) -> Result<i32, String> {
    let paths: Vec<String> =
        a.get("--paths").ok_or("--paths P1,P2 required")?.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    let msg = a.get("--message").unwrap_or_else(|| "data(regression): add micro-records".into());
    match gitsync::commit_paths(root, &paths, &msg)? {
        Some(sha) => println!("dataset commit: {sha}"),
        None => println!("dataset commit: nothing to commit"),
    }
    if let Some(target) = a.get("--push") {
        let (remote, branch) = target.split_once(':').ok_or("--push REMOTE:BRANCH")?;
        gitsync::push(root, remote, branch, 3)?;
        println!("dataset push: {remote}/{branch}");
    }
    Ok(0)
}
