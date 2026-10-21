use jarambit_datasets::{
    assemble, cv, store, Cell, Feature, Kind, Outcome, Record, Registry, Target,
};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "jarambit-datasets-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn registry() -> Registry {
    Registry {
        schema: 1,
        feature_schema: "fixture_v1".into(),
        features: vec![
            Feature {
                name: "x".into(),
                kind: Kind::Numeric,
                unit: "1".into(),
            },
            Feature {
                name: "z".into(),
                kind: Kind::Numeric,
                unit: "1".into(),
            },
            Feature {
                name: "category".into(),
                kind: Kind::Categorical,
                unit: "category".into(),
            },
        ],
        targets: vec![Target {
            name: "response".into(),
            unit: "1".into(),
            fidelity: "synthetic_software_control_only".into(),
        }],
    }
}
fn row(i: usize) -> Record {
    let x = ((i * 37) % 503) as f64 / 100.0;
    let z = ((i * 71) % 499) as f64 / 100.0;
    Record {
        schema: 1,
        run_id: format!("run{}", i / 50),
        evaluation_id: i.to_string(),
        design_id: format!("d{i}"),
        group_id: format!("family{}", i / 50),
        generation: 0,
        parents: vec![],
        context_id: "fixture-context".into(),
        feature_schema: "fixture_v1".into(),
        features: BTreeMap::from([
            ("x".into(), Cell::Number(x)),
            ("z".into(), Cell::Number(z)),
            (
                "category".into(),
                Cell::Category(if i % 2 == 0 { "a" } else { "b" }.into()),
            ),
        ]),
        outcomes: BTreeMap::from([("response".into(), Outcome::measured(2.0 * x + 0.5 * z))]),
    }
}
#[test]
fn immutable_atomic_idempotent_and_conflict() {
    let t = Temp::new();
    let p = t.0.join("a");
    store::put(&p, b"a").unwrap();
    store::put(&p, b"a").unwrap();
    assert!(store::put(&p, b"b").is_err());
    assert_eq!(std::fs::read(p).unwrap(), b"a");
}
#[test]
fn roundtrip_missing_failures_and_integrity() {
    let t = Temp::new();
    let mut r = row(0);
    r.outcomes.insert("response".into(), Outcome::Failed);
    let p = store::write_batch(&t.0, &[r.clone()]).unwrap();
    assert_eq!(store::read_batch(&p).unwrap(), vec![r]);
    let receipt: store::Receipt = serde_json::from_slice(&std::fs::read(&p).unwrap()).unwrap();
    std::fs::write(t.0.join("segments").join(receipt.segment), b"corrupt").unwrap();
    assert!(store::read_batch(&p).is_err());
}
#[test]
fn target_last_dedup_and_no_target_leak() {
    let t = Temp::new();
    let mut a = row(0);
    a.outcomes
        .insert("unregistered_target".into(), Outcome::measured(999.0));
    let mut b = a.clone();
    b.evaluation_id = "retry".into();
    let mut absent = row(1);
    absent
        .outcomes
        .insert("response".into(), Outcome::NotEvaluated);
    store::write_batch(&t.0.join("in"), &[a, b, absent]).unwrap();
    let out = assemble::assemble(&t.0.join("in"), &registry(), &t.0.join("out")).unwrap();
    let (m, rows, _) = assemble::read_snapshot(&out[0]).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(m.duplicate_designs, 1);
    assert_eq!(m.excluded_missing_target, 1);
    assert!(!m.features.contains(&"response".into()));
    assert_eq!(
        std::fs::read_to_string(out[0].join("dataset.csv"))
            .unwrap()
            .lines()
            .next()
            .unwrap(),
        "x,z,category,target"
    );
}
#[test]
fn conflicting_repeat_is_never_silently_chosen() {
    let t = Temp::new();
    let a = row(0);
    let mut b = a.clone();
    b.evaluation_id = "other".into();
    b.outcomes
        .insert("response".into(), Outcome::measured(99.0));
    store::write_batch(&t.0.join("in"), &[a, b]).unwrap();
    assert!(assemble::assemble(&t.0.join("in"), &registry(), &t.0.join("out")).is_err());
}
#[test]
fn schema_and_nonfinite_are_fail_closed() {
    let mut r = row(0);
    r.features.insert("x".into(), Cell::Number(f64::NAN));
    assert!(r.validate().is_err());
    let mut reg = registry();
    reg.targets[0].name = "x".into();
    assert!(reg.validate().is_err());
    assert_eq!(Outcome::measured(f64::INFINITY), Outcome::NonFinite);
}
#[test]
fn folds_keep_groups_together_and_cover_once() {
    let rows: Vec<_> = (0..500)
        .map(|i| {
            let r = row(i);
            assemble::RowMeta {
                run_id: r.run_id,
                evaluation_id: r.evaluation_id,
                design_id: r.design_id,
                group_id: r.group_id,
                generation: 0,
            }
        })
        .collect();
    let split = cv::folds(&rows, 5, 17).unwrap();
    assert_eq!(split, cv::folds(&rows, 5, 17).unwrap());
    for group in 0..10 {
        assert_eq!(
            split[group * 50..group * 50 + 50]
                .iter()
                .collect::<BTreeSet<_>>()
                .len(),
            1
        );
    }
    assert_eq!(split.iter().collect::<BTreeSet<_>>().len(), 5);
    assert!(cv::folds(&rows[..50], 5, 17).is_err());
}
#[test]
fn r_is_not_r_squared_or_lift() {
    let y = [1.0, 2.0, 3.0];
    let p = [101.0, 102.0, 103.0];
    let m = cv::metrics(&y, &p, &[2.0; 3]).unwrap();
    assert!((m.pearson_r.unwrap() - 1.0).abs() < 1e-12);
    assert!(m.r2.unwrap() < 0.0);
    assert!(m.rmse_lift.unwrap() < 0.0);
    let m = cv::metrics(&[2.0; 3], &[2.0; 3], &[2.0; 3]).unwrap();
    assert_eq!(m.pearson_r, None);
    assert_eq!(m.r2, None);
    assert_eq!(m.rmse_lift, None);
}
#[test]
fn snapshot_tampering_and_wrong_target_header_rejected() {
    let t = Temp::new();
    store::write_batch(&t.0.join("in"), &[row(0)]).unwrap();
    let out = assemble::assemble(&t.0.join("in"), &registry(), &t.0.join("out")).unwrap();
    std::fs::write(out[0].join("dataset.csv"), "target,x,z,category\n1,2,3,a\n").unwrap();
    assert!(assemble::read_snapshot(&out[0]).is_err());
}
/// Explicit integration gate; never silently passes without the real executable.
#[test]
#[ignore = "requires JARAMBIT_CATBOOST_BIN; invoked explicitly in CI"]
fn real_catboost_grouped_oof() {
    let t = Temp::new();
    store::write_batch(&t.0.join("in"), &(0..500).map(row).collect::<Vec<_>>()).unwrap();
    let out = assemble::assemble(&t.0.join("in"), &registry(), &t.0.join("out")).unwrap();
    let binary = std::env::var("JARAMBIT_CATBOOST_BIN").expect("CatBoost binary required");
    let config = cv::Config {
        iterations: 150,
        ..Default::default()
    };
    let report = cv::evaluate(
        &out[0],
        std::path::Path::new(&binary),
        &t.0.join("cv"),
        &config,
    )
    .unwrap();
    assert_eq!(report.status, "complete");
    assert_eq!(report.per_fold.len(), 5);
    let metrics = report.pooled.unwrap();
    assert!(metrics.pearson_r.unwrap() > 0.95, "{metrics:?}");
    assert!(metrics.rmse_lift.unwrap() > 0.80, "{metrics:?}");
    assert!(t.0.join("cv/oof.csv").is_file());
}

#[test]
fn concurrent_writers_publish_one_complete_value() {
    let t = Temp::new();
    let path = t.0.join("shared");
    std::thread::scope(|scope| {
        let a = scope.spawn(|| store::put(&path, b"same"));
        let b = scope.spawn(|| store::put(&path, b"same"));
        a.join().unwrap().unwrap();
        b.join().unwrap().unwrap();
    });
    assert_eq!(std::fs::read(path).unwrap(), b"same");
}

#[test]
fn git_publisher_is_scoped_idempotent_and_refuses_dirty_checkout() {
    use std::process::Command;
    let t = Temp::new();
    let repo = t.0.join("repo");
    let remote = t.0.join("remote.git");
    let spool = t.0.join("spool");
    std::fs::create_dir(&repo).unwrap();
    let git = |dir: &std::path::Path, args: &[&str]| {
        let o = Command::new("git")
            .current_dir(dir)
            .args(args)
            .output()
            .unwrap();
        assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
        String::from_utf8(o.stdout).unwrap()
    };
    git(&t.0, &["init", "--bare", remote.to_str().unwrap()]);
    git(&repo, &["init", "-b", "data/regression-test"]);
    git(&repo, &["config", "user.name", "Dataset test"]);
    git(
        &repo,
        &["config", "user.email", "dataset-test@example.invalid"],
    );
    git(&repo, &["commit", "--allow-empty", "-m", "fixture"]);
    git(
        &repo,
        &["remote", "add", "origin", remote.to_str().unwrap()],
    );
    store::write_batch(&spool, &[row(0)]).unwrap();
    let publish = || {
        Command::new(env!("CARGO_BIN_EXE_jarambit-datasets"))
            .args([
                "publish",
                spool.to_str().unwrap(),
                repo.to_str().unwrap(),
                "data/regression-test",
                "batch1",
            ])
            .output()
            .unwrap()
    };
    let first = publish();
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let head = git(&repo, &["rev-parse", "HEAD"]);
    assert!(publish().status.success());
    assert_eq!(head, git(&repo, &["rev-parse", "HEAD"]));
    let changed = git(
        &repo,
        &["diff-tree", "--no-commit-id", "--name-only", "-r", "HEAD"],
    );
    assert!(changed
        .lines()
        .all(|line| line.starts_with("datasets/regression/batches/batch1/")));
    std::fs::write(repo.join("unrelated"), "user change").unwrap();
    assert!(!publish().status.success());
    assert_eq!(head, git(&repo, &["rev-parse", "HEAD"]));
}

#[test]
fn floating_measurements_roundtrip_bit_exact_without_global_parser_changes() {
    for bits in [
        0x3ff0000000000001,
        0x3fe7edcba9876543,
        0x0000000000000001,
        0x7fefffffffffffff,
        0x8000000000000000,
    ] {
        let v = f64::from_bits(bits);
        let mut r = row(0);
        r.features.insert("x".into(), Cell::Number(v));
        r.outcomes
            .insert("response".into(), Outcome::Observed { value: v });
        let decoded: Record = serde_json::from_slice(&serde_json::to_vec(&r).unwrap()).unwrap();
        let Cell::Number(feature) = decoded.features["x"] else {
            panic!("numeric feature")
        };
        let Outcome::Observed { value } = decoded.outcomes["response"] else {
            panic!("observed outcome")
        };
        assert_eq!(feature.to_bits(), bits);
        assert_eq!(value.to_bits(), bits);
    }
}
