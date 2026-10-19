//! Publication views only: never runs CAD, FEM, search, or qualification.
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if !args.is_empty() && args != ["--check"] {
        eprintln!("usage: paper_artifacts [--check]");
        std::process::exit(2);
    }
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    if let Err(error) = jarambit_paper_artifacts::publication::generate(&root, args == ["--check"]) {
        eprintln!("publication: {error}");
        std::process::exit(1);
    }
}
