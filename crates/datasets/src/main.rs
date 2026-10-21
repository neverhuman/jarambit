use jarambit_datasets::{assemble, cv, store, Record, Registry, Result};
use std::path::Path;
fn run(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("ingest") if args.len()==3 => {
            let file=std::fs::read_to_string(&args[1])?;
            let rows:Vec<Record>=file.lines().map(serde_json::from_str).collect::<std::result::Result<_,_>>()?;
            println!("{}",store::write_batch(Path::new(&args[2]),&rows)?.display());
        }
        Some("assemble") if args.len()==4 => {
            let registry:Registry=serde_json::from_slice(&std::fs::read(&args[2])?)?;
            for dir in assemble::assemble(Path::new(&args[1]),&registry,Path::new(&args[3]))?{println!("{}",dir.display());}
        }
        Some("cv") if args.len()==5 => {
            let config:cv::Config=serde_json::from_slice(&std::fs::read(&args[4])?)?;
            let r=cv::evaluate(Path::new(&args[1]),Path::new(&args[2]),Path::new(&args[3]),&config)?;
            println!("{}",serde_json::to_string_pretty(&r)?);
        }
        Some("suite") if args.len() == 6 => {
            let registry: Registry = serde_json::from_slice(&std::fs::read(&args[2])?)?;
            let config: cv::Config = serde_json::from_slice(&std::fs::read(&args[5])?)?;
            let root = Path::new(&args[4]);
            let snapshots = assemble::assemble(Path::new(&args[1]), &registry, &root.join("snapshots"))?;
            if snapshots.len() > 64 { return Err("more than 64 datasets: partition suite by evaluator context".into()); }
            let mut summary = String::from("# Regression dataset CI\n\n");
            if snapshots.is_empty() { summary.push_str("No sealed observations are committed yet. No empirical lift was measured.\n"); }
            for snapshot in snapshots {
                let id = jarambit_datasets::hash(&std::fs::read(snapshot.join("manifest.json"))?);
                let out = root.join("reports").join(id);
                cv::evaluate(&snapshot, Path::new(&args[3]), &out, &config)?;
                summary.push_str(&std::fs::read_to_string(out.join("summary.md"))?);
                summary.push('\n');
            }
            store::put(&root.join("summary.md"), summary.as_bytes())?;
        }
        Some("check") if args.len()==2 => {let (m,_,_)=assemble::read_snapshot(Path::new(&args[1]))?;println!("{} rows, {} groups, target {}",m.rows,m.groups,m.target);}
        Some("publish") if args.len()==5 => {publish(Path::new(&args[1]),Path::new(&args[2]),&args[3],&args[4])?;}
        _=>return Err("usage: jarambit-datasets ingest RECORDS.jsonl SPOOL | assemble SPOOL REGISTRY.json OUTPUT | cv SNAPSHOT CATBOOST OUTPUT CONFIG.json | suite SPOOL REGISTRY CATBOOST OUTPUT CONFIG | check SNAPSHOT | publish SPOOL GIT_CHECKOUT BRANCH BATCH_ID".into()),
    }
    Ok(())
}
/// Explicit publisher. Solver processes never invoke Git. Separate checkout, own branch,
/// no force push and no main writes. Retry identical batches without extra commits.
fn publish(input: &Path, repo: &Path, branch: &str, batch: &str) -> Result<()> {
    use std::process::Command;
    if !branch.starts_with("data/regression-") || !jarambit_datasets::safe_name(batch) {
        return Err("publisher requires data/regression-* branch and safe batch ID".into());
    }
    let git = |args: &[&str]| -> Result<String> {
        let o = Command::new("git")
            .current_dir(repo)
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .env_remove("GIT_INDEX_FILE")
            .args(args)
            .output()?;
        if !o.status.success() {
            return Err(String::from_utf8_lossy(&o.stderr).into_owned().into());
        }
        Ok(String::from_utf8(o.stdout)?.trim().into())
    };
    if git(&["rev-parse", "--abbrev-ref", "HEAD"])? != branch {
        return Err("checkout must already be on the dedicated data branch".into());
    }
    if !git(&["status", "--porcelain"])?.is_empty() {
        return Err("publisher requires a clean dedicated checkout".into());
    }
    git(&["check-ref-format", "--branch", branch])?;
    let target = repo.join("datasets/regression/batches").join(batch);
    let receipts = store::receipts(input)?;
    if receipts.is_empty() {
        return Err("no sealed receipts to publish".into());
    }
    // Validate all inputs and budget before modifying the checkout.
    let mut files = Vec::new();
    let mut total = 0usize;
    for receipt in &receipts {
        store::read_batch(receipt)?;
        let root = receipt
            .parent()
            .and_then(Path::parent)
            .ok_or("bad receipt path")?;
        let r: store::Receipt = serde_json::from_slice(&std::fs::read(receipt)?)?;
        let shard = root.join("segments").join(&r.segment);
        let bytes = std::fs::read(shard)?;
        total += bytes.len();
        if total > 64 * 1024 * 1024 {
            return Err("batch exceeds 64 MiB Git publication budget; split the batch".into());
        }
        files.push((target.join("segments").join(&r.segment), bytes));
        files.push((
            target.join("receipts").join(format!("{}.json", r.sha256)),
            std::fs::read(receipt)?,
        ));
        for name in ["run.json", "context.json", "registry.json"] {
            let path = root.join(name);
            if path.exists() {
                let bytes = std::fs::read(path)?;
                let digest = jarambit_datasets::hash(&bytes);
                files.push((
                    target.join("provenance").join(format!("{digest}-{name}")),
                    bytes,
                ));
            }
        }
    }
    // Content-addressed paths make retries idempotent. No unrelated file is staged.
    for (path, bytes) in files {
        store::put(&path, &bytes)?;
    }
    let scope = format!("datasets/regression/batches/{batch}");
    git(&["add", "--", &scope])?;
    if !git(&["diff", "--cached", "--name-only", "--", &scope])?.is_empty() {
        git(&[
            "commit",
            "-m",
            &format!("data: seal regression batch {batch}"),
            "--",
            &scope,
        ])?;
    }
    git(&["push", "origin", &format!("HEAD:refs/heads/{branch}")])?;
    Ok(())
}
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(e) = run(&args) {
        eprintln!("jarambit-datasets: {e}");
        std::process::exit(1);
    }
}
