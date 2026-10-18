pub fn run(root: &std::path::Path) -> i32 {
    let manifest = root.join("crates/runner-policy/Cargo.toml");
    if !manifest.is_file() {
        eprintln!("runner policy package is missing: {}", manifest.display());
        return 2;
    }
    match std::process::Command::new("cargo")
        .current_dir(root)
        .args(["run", "--locked", "--manifest-path"])
        .arg(manifest)
        .status()
    {
        Ok(status) => status.code().unwrap_or(1),
        Err(error) => {
            eprintln!("could not launch runner policy package: {error}");
            1
        }
    }
}
