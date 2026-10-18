#[path = "../../crates/cli/src/version_check.rs"]
mod version_check;

fn main() {
    let root = std::env::current_dir().unwrap_or_else(|e| {
        eprintln!("current directory: {e}");
        std::process::exit(1);
    });
    let reference = std::env::var("GITHUB_REF").unwrap_or_default();
    std::process::exit(version_check::run(&root, &reference));
}
