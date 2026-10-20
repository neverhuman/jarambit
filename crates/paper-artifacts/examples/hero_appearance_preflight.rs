//! Reporting preflight only: stdout JSON; no output file, renderer, solve or score.
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args_os().skip(1);
    let root = args
        .next()
        .map(PathBuf::from)
        .ok_or("usage: hero_appearance_preflight REPO_ROOT")?;
    if args.next().is_some() {
        return Err("usage: hero_appearance_preflight REPO_ROOT".into());
    }
    let receipt = jarambit_paper_artifacts::population::hero_appearance_preflight(&root)?;
    println!("{}", serde_json::to_string_pretty(&receipt)?);
    Ok(())
}
