//! Live recorder used by `jarambit auto-evolve`: every birth of every round becomes one micro-record.
//!
//! Layout (one directory per run, so concurrent hosts never write the same file):
//! ```text
//! datasets/regression/records/auto_evolve/<run_id>/
//!   run.json                 run manifest (host, seed, git HEAD, flags, feature schema)
//!   r00.s1.csv               one row per birth: g.* c.* a.* gate.* desc.* + s1.* (when screened)
//!   r00.s2.<label>.csv       one row per Stage-2 assessment: s2.* (joined on meta.genome_hash)
//! ```
//! After each round the run directory can be committed (`--dataset-commit`) and pushed
//! (`--dataset-push REMOTE:BRANCH`). Recording never changes search, ranking or certification.
use super::gitsync;
use super::harvest;
use super::json::{self, Json};
use super::records::{self, Row, ShardWriter, HASH_COL};
use std::io::Write as _;
use std::path::{Path, PathBuf};

pub const LANE: &str = "auto_evolve";

pub struct RecorderConfig {
    pub enabled: bool,
    pub dir: Option<PathBuf>,
    pub commit: bool,
    pub push: Option<(String, String)>,
}

impl RecorderConfig {
    /// Flags: `--no-dataset`, `--dataset-dir DIR`, `--dataset-commit`, `--dataset-push REMOTE:BRANCH`.
    /// Environment: `JARAMBIT_DATASET=0` disables; `JARAMBIT_DATASET_COMMIT=1`, `JARAMBIT_DATASET_PUSH`.
    pub fn from_args(args: &[String]) -> Result<Self, String> {
        let get = |k: &str| args.iter().position(|x| x == k).and_then(|i| args.get(i + 1)).cloned();
        let env = |k: &str| std::env::var(k).ok().filter(|v| !v.is_empty());
        let enabled = !args.iter().any(|x| x == "--no-dataset") && env("JARAMBIT_DATASET").as_deref() != Some("0");
        let push = match get("--dataset-push").or_else(|| env("JARAMBIT_DATASET_PUSH")) {
            Some(v) => {
                let (r, b) = v.split_once(':').ok_or("--dataset-push expects REMOTE:BRANCH")?;
                Some((r.to_string(), b.to_string()))
            }
            None => None,
        };
        let commit = args.iter().any(|x| x == "--dataset-commit") || env("JARAMBIT_DATASET_COMMIT").as_deref() == Some("1") || push.is_some();
        Ok(RecorderConfig { enabled, dir: get("--dataset-dir").map(PathBuf::from), commit, push })
    }
}

pub struct Recorder {
    root: PathBuf,
    pub run_id: String,
    pub dir: PathBuf,
    cfg: RecorderConfig,
    pub s1_rows: usize,
    pub s2_rows: usize,
    round_s1: usize,
    round_s2: usize,
    pub warnings: Vec<String>,
}

/// UTC `YYYYMMDDTHHMMSSZ` from Unix seconds (civil-from-days, no external crates).
pub fn utc_stamp(secs: u64) -> String {
    let days = (secs / 86_400) as i64;
    let rem = secs % 86_400;
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + if m <= 2 { 1 } else { 0 };
    format!("{y:04}{m:02}{d:02}T{:02}{:02}{:02}Z", rem / 3600, (rem / 60) % 60, rem % 60)
}

pub fn host_name() -> String {
    let raw = std::env::var("HOSTNAME")
        .ok()
        .filter(|h| !h.is_empty())
        .or_else(|| std::fs::read_to_string("/etc/hostname").ok().map(|s| s.trim().to_string()))
        .unwrap_or_else(|| "host".into());
    let clean: String = raw.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '-' }).collect();
    if clean.is_empty() {
        "host".into()
    } else {
        clean
    }
}

fn sanitize(s: &str) -> String {
    s.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '-' }).collect()
}

/// Numeric leaves of a JSON object as `prefix.path` columns (depth <= 3, short arrays indexed).
pub fn flatten_numeric(v: &Json, prefix: &str, out: &mut Vec<(String, f64)>, depth: usize) {
    if out.len() >= 400 {
        return;
    }
    match v {
        Json::Num(_) | Json::Bool(_) => {
            if let Some(x) = v.as_f64() {
                out.push((prefix.to_string(), x));
            }
        }
        Json::Obj(items) if depth < 3 => {
            for (k, x) in items {
                flatten_numeric(x, &format!("{prefix}.{}", sanitize(k)), out, depth + 1);
            }
        }
        Json::Arr(items) if depth < 3 && items.len() <= 4 => {
            for (i, x) in items.iter().enumerate() {
                flatten_numeric(x, &format!("{prefix}.{i}"), out, depth + 1);
            }
        }
        _ => {}
    }
}

/// Attempts per base run id: the id itself, then `-n2` .. `-n9`.
const MAX_RUN_ATTEMPTS: u32 = 9;

/// Claim a new run directory `lane_dir/<id>` that did not exist before. Tries `base`, then
/// `base-n2` .. `base-n9`; `create_dir` (not `create_dir_all`) makes the claim exclusive.
/// Returns the run id that won and its directory. Fails after the last attempt or on any other error.
fn claim_run_dir(lane_dir: &Path, base: &str) -> std::io::Result<(String, PathBuf)> {
    std::fs::create_dir_all(lane_dir)?;
    for attempt in 1..=MAX_RUN_ATTEMPTS {
        let run_id = if attempt == 1 { base.to_string() } else { format!("{base}-n{attempt}") };
        let dir = lane_dir.join(&run_id);
        match std::fs::create_dir(&dir) {
            Ok(()) => return Ok((run_id, dir)),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e),
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::AlreadyExists,
        format!("run ids {base} and {base}-n2..-n{MAX_RUN_ATTEMPTS} are all taken"),
    ))
}

/// Create `run.json` in a freshly claimed run directory; never overwrites an existing manifest.
fn write_run_manifest(dir: &Path, text: &str) -> std::io::Result<()> {
    let mut file = std::fs::OpenOptions::new().write(true).create_new(true).open(dir.join("run.json"))?;
    file.write_all(text.as_bytes())
}

impl Recorder {
    pub fn start(root: &Path, cfg: RecorderConfig, prefix: &str, seed: u64, manifest: Json) -> Option<Recorder> {
        if !cfg.enabled {
            return None;
        }
        let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        let base_id = format!("{}-{}-{}-s{seed}", sanitize(prefix), utc_stamp(secs), host_name());
        let base = match &cfg.dir {
            Some(d) if d.is_absolute() => d.clone(),
            Some(d) => root.join(d),
            None => root.join(super::engine::DEFAULT_RECORDS),
        };
        let (run_id, dir) = match claim_run_dir(&base.join(LANE), &base_id) {
            Ok(claimed) => claimed,
            Err(e) => {
                eprintln!("dataset recorder: disabled, cannot claim a run directory under {}: {e}", base.join(LANE).display());
                return None;
            }
        };
        let doc = manifest
            .with("schema", "jarambit.regression.run.v1")
            .with("run_id", run_id.as_str())
            .with("lane", LANE)
            .with("host", host_name())
            .with("started_utc", utc_stamp(secs))
            .with("git_head", gitsync::head(root))
            .with("feature_schema", super::featurize::FEATURE_SCHEMA)
            .with("commit", cfg.commit)
            .with("push", cfg.push.as_ref().map(|(r, b)| format!("{r}:{b}")));
        if let Err(e) = write_run_manifest(&dir, &doc.pretty()) {
            eprintln!("dataset recorder: disabled, cannot write run manifest {}: {e}", dir.join("run.json").display());
            return None;
        }
        println!("Dataset recorder: {} (commit={}, push={})", dir.display(), cfg.commit, cfg.push.is_some());
        Some(Recorder { root: root.to_path_buf(), run_id, dir, cfg, s1_rows: 0, s2_rows: 0, round_s1: 0, round_s2: 0, warnings: Vec::new() })
    }

    fn warn(&mut self, msg: String) {
        eprintln!("dataset recorder: {msg}");
        self.warnings.push(msg);
    }

    /// Stamp run provenance on a birth row.
    pub fn tag(&self, row: &mut Row, round: usize, idx: usize, family: &str, identity: &str) {
        row.text("meta.lane", LANE).text("meta.source", "live").text("meta.run_id", &self.run_id);
        row.text("meta.round", &format!("{round:02}")).text("meta.idx", &idx.to_string());
        row.text("meta.family", family).text("meta.certificate_identity", identity);
    }

    pub fn write_round_s1(&mut self, round: usize, rows: &[Row]) {
        if rows.is_empty() {
            return;
        }
        match records::write_shard_create_new(&self.dir.join(format!("r{round:02}.s1.csv")), rows) {
            Ok(n) => {
                self.s1_rows += n;
                self.round_s1 += n;
            }
            Err(e) => self.warn(format!("round {round} Stage-1 shard not written: {e}")),
        }
    }

    /// One Stage-2 assessment. `design` is the leaderboard design object; `folder` holds the
    /// evaluator's `<label>_master_scorecard.json` and `<label>_rs.json`, whose numeric
    /// leaves are added as `s2.em.*`, `s2.wc.*`, `s2.hm.*`, `s2.x.*`.
    pub fn append_s2(&mut self, round: usize, design: &Json, genome_hash: &str, folder: Option<&Path>) {
        let mut row = harvest::s2_row(design, genome_hash);
        row.text("meta.lane", LANE).text("meta.run_id", &self.run_id).text("meta.round", &format!("{round:02}"));
        let label = design.get("label").and_then(Json::as_str).unwrap_or("design").to_string();
        if let Some(folder) = folder {
            let read = |name: String| std::fs::read_to_string(folder.join(name)).ok().and_then(|t| json::parse(&t).ok());
            let mut cells: Vec<(String, f64)> = Vec::new();
            if let Some(sc) = read(format!("{label}_master_scorecard.json")) {
                if let Some(em) = sc.get("engineering_metrics") {
                    flatten_numeric(em, "s2.em", &mut cells, 0);
                }
                if let Some(wc) = sc.get("weighted_cad_score") {
                    flatten_numeric(wc, "s2.wc", &mut cells, 0);
                }
                if let Some(x) = sc.get("export_measurements") {
                    flatten_numeric(x, "s2.x", &mut cells, 0);
                }
            }
            if let Some(rs) = read(format!("{label}_rs.json")) {
                if let Some(hm) = rs.get("human_max") {
                    flatten_numeric(hm, "s2.hm", &mut cells, 0);
                }
            }
            for (k, v) in cells {
                if row.get(&k).is_none() {
                    row.num(&k, v);
                }
            }
        }
        let path = self.dir.join(format!("r{round:02}.s2.{}.csv", sanitize(&label)));
        let result = ShardWriter::open(&path, &row.columns()).and_then(|mut w| w.append(&row));
        match result {
            Ok(()) => {
                self.s2_rows += 1;
                self.round_s2 += 1;
            }
            Err(e) => self.warn(format!("Stage-2 record for {label} not written: {e}")),
        }
        debug_assert!(row.get(HASH_COL).is_some());
    }

    /// Commit (and push) this run's directory after a round. Failures are reported, never fatal.
    pub fn end_round(&mut self, round: usize) {
        let (n1, n2) = (self.round_s1, self.round_s2);
        self.round_s1 = 0;
        self.round_s2 = 0;
        println!("Dataset recorder: round {} -> {n1} Stage-1 and {n2} Stage-2 records ({})", round + 1, self.dir.display());
        if !self.cfg.commit {
            return;
        }
        if !gitsync::is_repo(&self.root) {
            self.warn("--dataset-commit: not a git checkout; records left on disk".into());
            return;
        }
        let rel = self.dir.strip_prefix(&self.root).unwrap_or(&self.dir).to_string_lossy().to_string();
        let msg = format!("data(regression): {} round {:02} (+{n1} s1, +{n2} s2 records)", self.run_id, round);
        match gitsync::commit_paths(&self.root, &[rel], &msg) {
            Ok(Some(sha)) => println!("Dataset recorder: committed {sha}"),
            Ok(None) => {}
            Err(e) => return self.warn(format!("commit failed (records kept on disk): {e}")),
        }
        if let Some((remote, branch)) = self.cfg.push.clone() {
            match gitsync::push(&self.root, &remote, &branch, 3) {
                Ok(()) => println!("Dataset recorder: pushed to {remote}/{branch}"),
                Err(e) => self.warn(format!("push failed (commit kept locally): {e}")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utc_stamp_matches_known_instants() {
        assert_eq!(utc_stamp(0), "19700101T000000Z");
        assert_eq!(utc_stamp(1_792_560_000), "20261021T052000Z");
        assert_eq!(utc_stamp(951_782_400), "20000229T000000Z");
    }

    #[test]
    fn flags_and_flattening() {
        let a: Vec<String> = ["--dataset-push", "origin:data/regression", "--dataset-dir", "x"].iter().map(|s| s.to_string()).collect();
        let c = RecorderConfig::from_args(&a).unwrap();
        assert!(c.enabled && c.commit);
        assert_eq!(c.push, Some(("origin".into(), "data/regression".into())));
        assert!(RecorderConfig::from_args(&["--dataset-push".into(), "nocolon".into()]).is_err());
        let j = json::parse(r#"{"a":1,"b":{"c":true,"d":[1,2]},"e":"text","f":[1,2,3,4,5]}"#).unwrap();
        let mut out = Vec::new();
        flatten_numeric(&j, "s2.em", &mut out, 0);
        assert_eq!(
            out,
            vec![("s2.em.a".into(), 1.0), ("s2.em.b.c".into(), 1.0), ("s2.em.b.d.0".into(), 1.0), ("s2.em.b.d.1".into(), 2.0)]
        );
    }

    fn unique_dir(name: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let d = std::env::temp_dir().join(format!("jarambit-recorder-{name}-{}-{nanos}", std::process::id()));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn claim_run_dir_never_reuses_an_existing_run() {
        let lane = unique_dir("claim");
        let (id1, d1) = claim_run_dir(&lane, "run-a").unwrap();
        assert_eq!((id1.as_str(), d1.clone()), ("run-a", lane.join("run-a")));
        let (id2, d2) = claim_run_dir(&lane, "run-a").unwrap();
        assert_eq!(id2, "run-a-n2");
        assert_ne!(d1, d2);
        // Exhaust the remaining suffixes, then the claim must fail instead of reusing a directory.
        for n in 3..=MAX_RUN_ATTEMPTS {
            assert_eq!(claim_run_dir(&lane, "run-a").unwrap().0, format!("run-a-n{n}"));
        }
        assert!(claim_run_dir(&lane, "run-a").is_err());
        let _ = std::fs::remove_dir_all(&lane);
    }

    #[test]
    fn start_twice_keeps_each_manifest_intact() {
        let root = unique_dir("start");
        let cfg = || RecorderConfig { enabled: true, dir: Some(root.join("records")), commit: false, push: None };
        let manifest = || json::parse(r#"{"prefix":"t"}"#).unwrap();
        let first = Recorder::start(&root, cfg(), "t", 7, manifest()).expect("first recorder");
        let first_bytes = std::fs::read(first.dir.join("run.json")).unwrap();
        let second = Recorder::start(&root, cfg(), "t", 7, manifest()).expect("second recorder");
        assert_ne!(first.dir, second.dir);
        assert_ne!(first.run_id, second.run_id);
        assert_eq!(std::fs::read(first.dir.join("run.json")).unwrap(), first_bytes);
        assert!(second.dir.join("run.json").is_file());
        let _ = std::fs::remove_dir_all(&root);
    }
}
