//! Exercise the real paper Makefile's dependency graph and failure propagation.
use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn repository() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned()
}

#[test]
fn smoothing_precedes_figure_and_document_consumers() {
    let output = Command::new("make")
        .args(["--dry-run", "--always-make", "-C"])
        .arg(repository().join("docs/paper"))
        .arg("pdfs")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let commands = String::from_utf8(output.stdout).unwrap();
    let smoothing = commands
        .find("--manifest-path experiments/geometry-smoothing-benchmark/Cargo.toml")
        .unwrap();
    assert!(smoothing < commands.find("rsvg-convert").unwrap());
    assert!(smoothing < commands.find("lualatex -recorder").unwrap());
    let hero = Command::new("make")
        .args(["--dry-run", "--always-make", "-C"])
        .arg(repository().join("docs/paper"))
        .arg("drafts_p3_p5/JR_EI_P4_T01_blender_exact_cad_hero_white.png")
        .output()
        .unwrap();
    assert!(hero.status.success());
    let commands = String::from_utf8(hero.stdout).unwrap();
    assert!(
        commands
            .find("--manifest-path experiments/geometry-smoothing-benchmark/Cargo.toml")
            .unwrap()
            < commands.find("rsvg-convert").unwrap()
    );
}

#[test]
fn failed_preflight_stops_before_any_render_command() {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("paper-preflight-{}-{stamp}", std::process::id()));
    struct Cleanup(PathBuf);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    let _cleanup = Cleanup(root.clone());
    let paper = root.join("docs/paper");
    fs::create_dir_all(&paper).unwrap();
    fs::copy(
        repository().join("docs/paper/Makefile"),
        paper.join("Makefile"),
    )
    .unwrap();
    let failure = root.join("fail-cargo.sh");
    fs::write(
        &failure,
        "printf '%s\\n' 'EXPECTED_SYNTHETIC_PREFLIGHT_FAILURE' >&2\nexit 23\n",
    )
    .unwrap();
    let output = Command::new("make")
        .arg("-C")
        .arg(&paper)
        .arg("pdfs")
        .arg(format!("CARGO=sh {}", failure.display()))
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("EXPECTED_SYNTHETIC_PREFLIGHT_FAILURE")
    );
    let commands = String::from_utf8(output.stdout).unwrap();
    assert!(!commands.contains("rsvg-convert"));
    assert!(!commands.contains("lualatex -recorder"));
    assert!(!root
        .join("target/ieee/paper/synthetic-geometry/LATEST")
        .exists());
}
