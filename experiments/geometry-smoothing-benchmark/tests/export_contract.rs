use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        Self(
            std::env::temp_dir().join(format!("synthetic-smoothing-{}-{time}", std::process::id())),
        )
    }
    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_geometry-smoothing-benchmark"))
            .arg("--out")
            .arg(&self.0)
            .args(args)
            .output()
            .unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn vertices(path: PathBuf, prefix: &str) -> Vec<[f64; 3]> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            if fields.next() != Some(prefix) {
                return None;
            }
            let p = [
                fields.next().unwrap().parse().unwrap(),
                fields.next().unwrap().parse().unwrap(),
                fields.next().unwrap().parse().unwrap(),
            ];
            assert!(fields.next().is_none());
            Some(p)
        })
        .collect()
}

#[test]
fn exported_coordinates_independently_establish_geometry_improvement() {
    let fixture = Fixture::new();
    let run = fixture.run(&[]);
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let raw = vertices(fixture.0.join("rough.obj"), "v");
    let smooth = vertices(fixture.0.join("smoothed.obj"), "v");
    let reference = vertices(fixture.0.join("reference.obj"), "v");
    assert_eq!(raw.len(), 642);
    assert_eq!(raw.len(), smooth.len());
    let rms = |points: &[[f64; 3]]| {
        (points
            .iter()
            .map(|p| {
                let length = p.iter().map(|x| x * x).sum::<f64>().sqrt();
                (length - 30.).powi(2)
            })
            .sum::<f64>()
            / points.len() as f64)
            .sqrt()
    };
    assert!(rms(&smooth) < 0.5 * rms(&raw));
    assert!(rms(&reference) < 1e-12);
    assert_eq!(raw.iter().zip(&smooth).filter(|(a, b)| a != b).count(), 642);
    let face_lines = |name: &str| {
        fs::read_to_string(fixture.0.join(name))
            .unwrap()
            .lines()
            .filter(|line| line.starts_with("f "))
            .map(str::to_owned)
            .collect::<Vec<_>>()
    };
    let faces = face_lines("rough.obj");
    assert_eq!(faces.len(), 1280);
    assert_eq!(faces, face_lines("smoothed.obj"));
    let stl = vertices(fixture.0.join("smoothed.stl"), "vertex");
    let mut expected = Vec::new();
    for face in faces {
        for index in face.split_whitespace().skip(1) {
            expected.push(smooth[index.parse::<usize>().unwrap() - 1]);
        }
    }
    assert_eq!(
        stl, expected,
        "STL must contain exact changed coordinates, not just new normals"
    );
    assert_eq!(
        fixture.run(&[]).status.code(),
        Some(1),
        "existing output must not be overwritten"
    );
}

#[test]
fn insufficient_movement_and_shrinkage_are_reported_as_failures() {
    for args in [
        vec!["--max-displacement-mm", "0.00001"],
        vec!["--mu", "-0.01"],
        vec!["--noise-mm", "0"],
    ] {
        let fixture = Fixture::new();
        let run = fixture.run(&args);
        assert_eq!(
            run.status.code(),
            Some(2),
            "{}",
            String::from_utf8_lossy(&run.stderr)
        );
        let metrics = fs::read_to_string(fixture.0.join("metrics.json")).unwrap();
        assert!(metrics.contains("\"benchmark_passed\":false"));
        assert!(!metrics.contains("NaN"));
        assert!(!metrics.contains("inf"));
    }
}

#[test]
fn invalid_cli_input_fails_before_creating_artifacts() {
    for args in [
        vec!["--lambda", "NaN"],
        vec!["--iterations", "1001"],
        vec!["--unknown", "3"],
        vec!["--iterations", "1", "--iterations", "2"],
    ] {
        let fixture = Fixture::new();
        assert_eq!(fixture.run(&args).status.code(), Some(1));
        assert!(!fixture.0.exists());
    }
}
