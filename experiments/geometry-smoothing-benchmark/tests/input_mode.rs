//! CLI tests for input-mesh mode. Scratch files live under `CARGO_TARGET_TMPDIR`
//! (inside this crate's target directory), not in the system temp directory.
use geometry_smoothing_benchmark::{input, smooth, sphere, Mesh, Settings};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

fn scratch(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("input-mode-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn write_ascii_stl(path: &Path, mesh: &Mesh) {
    let mut text = String::from("solid test\n");
    for &[a, b, c] in &mesh.faces {
        text.push_str("  facet normal 0 0 1\n    outer loop\n");
        for i in [a, b, c] {
            let p = mesh.vertices[i];
            text.push_str(&format!("      vertex {} {} {}\n", p[0], p[1], p[2]));
        }
        text.push_str("    endloop\n  endfacet\n");
    }
    text.push_str("endsolid test\n");
    fs::write(path, text).unwrap();
}

fn run_input(input: &Path, out: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_geometry-smoothing-benchmark"))
        .arg("--input")
        .arg(input)
        .arg("--out")
        .arg(out)
        .arg("--label")
        .arg("test fixture")
        .output()
        .unwrap()
}

#[test]
fn open_mesh_is_rejected_before_smoothing() {
    let dir = scratch("open");
    let mut mesh = sphere(2, 30., 0.8).unwrap();
    mesh.faces.pop();
    let input = dir.join("open.stl");
    write_ascii_stl(&input, &mesh);
    let out = dir.join("out");
    let run = run_input(&input, &out);
    assert_eq!(
        run.status.code(),
        Some(1),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let check = fs::read_to_string(out.join("input-check.json")).unwrap();
    assert!(check.contains("\"status\":\"rejected\""), "{check}");
    assert!(check.contains("open edges"), "{check}");
    assert!(!out.join("metrics.json").exists());
    assert!(!out.join("smoothed.stl").exists());
    assert!(!out.join("failed-state.stl").exists());
}

#[test]
fn noisy_closed_input_passes_and_output_reloads_with_identical_topology() {
    let dir = scratch("pass");
    let source = sphere(3, 30., 0.8).unwrap();
    let input = dir.join("noisy.stl");
    write_ascii_stl(&input, &source);
    let out = dir.join("out");
    let run = run_input(&input, &out);
    assert_eq!(
        run.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );

    // Recompute what the CLI should produce from the same parsed input.
    let (_, triangles) = input::parse_stl(&fs::read(&input).unwrap()).unwrap();
    let welded = input::weld(&triangles);
    let pins = vec![false; welded.vertices.len()];
    let expected = smooth(&welded, Settings::default(), &pins).unwrap();

    let stl = fs::read(out.join("smoothed.stl")).unwrap();
    let (_, reloaded_triangles) = input::parse_stl(&stl).unwrap();
    assert_eq!(reloaded_triangles.len(), welded.faces.len());
    for (facet, face) in reloaded_triangles.iter().zip(&welded.faces) {
        let corners = face.map(|i| expected.vertices[i]);
        assert_eq!(
            facet, &corners,
            "STL corners must be the smoothed coordinates"
        );
    }
    let reloaded = input::weld(&reloaded_triangles);
    assert_eq!(reloaded.faces.len(), welded.faces.len());
    assert_eq!(reloaded.vertices.len(), welded.vertices.len());
    let audit = input::audit(&reloaded);
    assert!(audit.failures().is_empty(), "{audit:?}");
    assert_eq!(audit.euler_characteristic, 2);

    let metrics = fs::read_to_string(out.join("metrics.json")).unwrap();
    assert!(metrics.contains("\"status\":\"pass\""), "{metrics}");
    assert!(metrics.contains("\"not_qualified\":true"), "{metrics}");
    let stl_sha = geometry_smoothing_benchmark::sha256::sha256_hex(&stl);
    assert!(
        metrics.contains(&stl_sha),
        "metrics must record the STL hash"
    );
    assert!(out.join("displacement.csv").exists());
    assert!(out.join("REPORT.md").exists());
}

#[test]
fn input_mode_refuses_synthetic_options_and_existing_output() {
    let dir = scratch("options");
    let source = sphere(1, 30., 0.8).unwrap();
    let input = dir.join("in.stl");
    write_ascii_stl(&input, &source);
    let out = dir.join("out");
    let synthetic = Command::new(env!("CARGO_BIN_EXE_geometry-smoothing-benchmark"))
        .args(["--input"])
        .arg(&input)
        .args(["--out"])
        .arg(&out)
        .args(["--noise-mm", "0.8"])
        .output()
        .unwrap();
    assert_eq!(synthetic.status.code(), Some(1));
    assert!(!out.exists(), "no artifacts before argument rejection");
    fs::create_dir_all(&out).unwrap();
    assert_eq!(run_input(&input, &out).status.code(), Some(1));
}
