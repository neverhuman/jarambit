//! Offline review-origin enforcement. No HTTP, credentials, writes, or merge action.
//!
//! Trust roots are TWO hashes supplied out of band by the merge owner: a
//! pre-review assignment and a post-submission origin attestation. The owner
//! must inspect authentic dispatch/submission/fetch tool records before pinning
//! them. This program checks their exact byte bindings; self-authored JSON or a
//! shared GitHub login cannot authenticate an operator. Never obtain the pins
//! automatically from reviewer-controlled files. Neutral fixtures are controls,
//! not real merge clearance. A fresh expected-head/sole-PR check remains required.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    time::{SystemTime, UNIX_EPOCH},
};

type Check<T> = Result<T, String>;
const FILES: [&str; 14] = [
    "assignment",
    "origin",
    "authors",
    "source-report",
    "artifact-report",
    "submission",
    "request",
    "response",
    "review",
    "checks",
    "runs",
    "dispatch-record",
    "submission-record",
    "fetch-record",
];
const PINS: [&str; 4] = [
    "assignment-sha256",
    "origin-sha256",
    "expected-head",
    "expected-tree",
];

#[derive(Clone)]
struct Inputs {
    bytes: BTreeMap<String, Vec<u8>>,
    pins: BTreeMap<String, String>,
}
fn need(ok: bool, message: &str) -> Check<()> {
    if ok {
        Ok(())
    } else {
        Err(message.into())
    }
}
fn text<'a>(v: &'a Value, pointer: &str) -> Check<&'a str> {
    v.pointer(pointer)
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| format!("missing/non-string {pointer}"))
}
fn number(v: &Value, pointer: &str) -> Check<u64> {
    v.pointer(pointer)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("missing/non-u64 {pointer}"))
}
fn array<'a>(v: &'a Value, pointer: &str) -> Check<&'a Vec<Value>> {
    v.pointer(pointer)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("missing/non-array {pointer}"))
}
fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn hex(s: &str, length: usize) -> bool {
    s.len() == length
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn raw<'a>(i: &'a Inputs, name: &str) -> Check<&'a [u8]> {
    i.bytes
        .get(name)
        .map(Vec::as_slice)
        .filter(|b| !b.is_empty())
        .ok_or_else(|| format!("missing input {name}"))
}
fn document(i: &Inputs, name: &str) -> Check<Value> {
    serde_json::from_slice(raw(i, name)?).map_err(|e| format!("invalid JSON {name}: {e}"))
}
fn pin<'a>(i: &'a Inputs, name: &str, length: usize) -> Check<&'a str> {
    let value = i
        .pins
        .get(name)
        .ok_or_else(|| format!("missing trusted pin {name}"))?;
    need(hex(value, length), &format!("invalid trusted pin {name}"))?;
    Ok(value)
}
fn hash_is(i: &Inputs, name: &str, expected: &str) -> Check<()> {
    need(hex(expected, 64), &format!("invalid expected hash {name}"))?;
    need(
        digest(raw(i, name)?) == expected,
        &format!("{name} SHA256 mismatch"),
    )
}
fn equal(a: &Value, b: &Value, pointer: &str, label: &str) -> Check<()> {
    let av = a
        .pointer(pointer)
        .ok_or_else(|| format!("missing {label} {pointer}"))?;
    let bv = b
        .pointer(pointer)
        .ok_or_else(|| format!("missing {label} {pointer}"))?;
    need(av == bv, &format!("{label} mismatch at {pointer}"))
}
fn runtime(v: &Value) -> Check<()> {
    for key in ["/worker_id", "/thread_id", "/dispatch_id"] {
        text(v, key)?;
    }
    need(
        matches!(text(v, "/mode")?, "manual" | "native"),
        "invalid runtime mode",
    )?;
    if text(v, "/mode")? == "manual" {
        need(
            v.get("native_task_id") == Some(&Value::Null),
            "manual runtime must retain null native_task_id",
        )
    } else {
        text(v, "/native_task_id").map(|_| ())
    }
}
fn context(v: &Value, head: &str, tree: &str) -> Check<()> {
    need(text(v, "/head")? == head, "head mismatch")?;
    need(text(v, "/tree")? == tree, "tree mismatch")
}
// GitHub submitted_at/completed_at are UTC RFC3339 seconds. Reject other forms,
// invalid dates, leap seconds, pre-epoch and future/fabricated chronology.
fn utc_seconds(s: &str) -> Check<u64> {
    let b = s.as_bytes();
    need(
        b.len() == 20
            && b[4] == b'-'
            && b[7] == b'-'
            && b[10] == b'T'
            && b[13] == b':'
            && b[16] == b':'
            && b[19] == b'Z',
        "invalid UTC timestamp",
    )?;
    let part = |a: usize, z: usize| -> Check<i64> {
        need(
            b[a..z].iter().all(u8::is_ascii_digit),
            "invalid timestamp digits",
        )?;
        s[a..z]
            .parse()
            .map_err(|_| "invalid timestamp number".into())
    };
    let (year, month, day) = (part(0, 4)?, part(5, 7)?, part(8, 10)?);
    let (hour, minute, second) = (part(11, 13)?, part(14, 16)?, part(17, 19)?);
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let lengths = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    need(
        year >= 1970
            && (1..=12).contains(&month)
            && day >= 1
            && day <= lengths[(month - 1).clamp(0, 11) as usize]
            && (0..24).contains(&hour)
            && (0..60).contains(&minute)
            && (0..60).contains(&second),
        "invalid UTC date",
    )?;
    let y = year - i64::from(month <= 2);
    let era = y / 400;
    let yoe = y - era * 400;
    let mp = month + if month > 2 { -3 } else { 9 };
    let days =
        era * 146097 + yoe * 365 + yoe / 4 - yoe / 100 + (153 * mp + 2) / 5 + day - 1 - 719468;
    u64::try_from(days * 86400 + hour * 3600 + minute * 60 + second)
        .map_err(|_| "timestamp before epoch".into())
}
fn complete_collection<'a>(v: &'a Value, key: &str) -> Check<&'a Vec<Value>> {
    let rows = array(v, &format!("/{key}"))?;
    need(
        number(v, "/total_count")? == rows.len() as u64,
        "incomplete/paginated API collection",
    )?;
    Ok(rows)
}
fn by_id(rows: &[Value], id: u64, label: &str) -> Check<Value> {
    let found: Vec<_> = rows
        .iter()
        .filter(|v| v.get("id").and_then(Value::as_u64) == Some(id))
        .collect();
    need(found.len() == 1, &format!("missing/duplicate {label} ID"))?;
    Ok(found[0].clone())
}
fn binding_block(a: &Value, origin: &Value, assignment_hash: &str) -> Check<String> {
    let r = &a["reviewer_runtime"];
    let mut ci = Vec::new();
    for item in array(origin, "/required_ci")? {
        ci.push(format!(
            "{}:{}:{}:{}",
            text(item, "/name")?,
            number(item, "/check_run_id")?,
            number(item, "/workflow_run_id")?,
            number(item, "/workflow_run_attempt")?
        ));
    }
    ci.sort();
    Ok(format!("BEGIN JARAMBIT REVIEW ORIGIN V1\nassignment_sha256={}\nworker={}\nthread={}\ndispatch={}\nhead={}\ntree={}\nsource_report_sha256={}\nartifact_report_sha256={}\nrequired_ci={}\nEND JARAMBIT REVIEW ORIGIN V1",
        assignment_hash, text(r,"/worker_id")?, text(r,"/thread_id")?, text(r,"/dispatch_id")?,
        text(a,"/head")?, text(a,"/tree")?, text(origin,"/source_report_sha256")?,
        text(origin,"/artifact_report_sha256")?, ci.join(",")))
}
fn verify(i: &Inputs, now: u64) -> Check<Value> {
    let head = pin(i, "expected-head", 40)?;
    let tree = pin(i, "expected-tree", 40)?;
    let assignment_hash = pin(i, "assignment-sha256", 64)?;
    let origin_hash = pin(i, "origin-sha256", 64)?;
    hash_is(i, "assignment", assignment_hash)?;
    hash_is(i, "origin", origin_hash)?;
    let a = document(i, "assignment")?;
    let o = document(i, "origin")?;
    need(
        text(&a, "/schema")? == "jarambit.review_assignment.v1",
        "wrong assignment schema",
    )?;
    need(
        text(&o, "/schema")? == "jarambit.merge_owner_review_origin.v1",
        "wrong origin schema",
    )?;
    context(&a, head, tree)?;
    context(&o, head, tree)?;
    need(
        text(&o, "/assignment_sha256")? == assignment_hash,
        "origin assignment mismatch",
    )?;
    equal(&a, &o, "/merge_owner_runtime_id", "merge owner")?;
    need(
        o.get("verified_against_actual_tool_records") == Some(&Value::Bool(true)),
        "origin not verified from actual tool records",
    )?;
    runtime(&a["reviewer_runtime"])?;
    equal(&a, &o, "/reviewer_runtime", "runtime")?;
    let reviewer = &a["reviewer_runtime"];
    need(
        text(&a, "/merge_owner_runtime_id")? != text(reviewer, "/worker_id")?
            && text(&a, "/merge_owner_runtime_id")? != text(reviewer, "/thread_id")?,
        "reviewer cannot authorize own assignment",
    )?;
    let issued = number(&a, "/issued_at_unix")?;
    let age = number(&a, "/maximum_fetch_age_seconds")?;
    let start = number(&o, "/fetch_started_at_unix")?;
    let fetched = number(&o, "/fetched_at_unix")?;
    need(
        (1..=300).contains(&age)
            && issued <= start
            && start <= fetched
            && fetched <= now
            && now - start <= age,
        "stale/future/unordered owner fetch",
    )?;
    hash_is(i, "authors", text(&a, "/authors_sha256")?)?;
    hash_is(i, "dispatch-record", text(&a, "/dispatch_record_sha256")?)?;
    for name in [
        "source-report",
        "artifact-report",
        "submission",
        "request",
        "response",
        "review",
        "checks",
        "runs",
        "submission-record",
        "fetch-record",
    ] {
        hash_is(
            i,
            name,
            text(&o, &format!("/{}_sha256", name.replace('-', "_")))?,
        )?;
    }
    let authors = document(i, "authors")?;
    context(&authors, head, tree)?;
    let author_rows = array(&authors, "/authors")?;
    need(!author_rows.is_empty(), "empty source-author map")?;
    for author in author_rows {
        need(
            text(author, "/worker_id")? != text(reviewer, "/worker_id")?
                && text(author, "/thread_id")? != text(reviewer, "/thread_id")?,
            "reviewer is source author",
        )?;
    }
    for name in ["source-report", "artifact-report"] {
        let report = document(i, name)?;
        context(&report, head, tree)?;
        need(text(&report, "/verdict")? == "PASS", "report is not PASS")?;
        equal(&a, &report, "/reviewer_runtime", "report runtime")?;
        if name == "artifact-report" {
            let hashes = report
                .get("artifact_sha256")
                .and_then(Value::as_object)
                .ok_or("missing artifact hash map")?;
            need(!hashes.is_empty(), "empty artifact hash map")?;
            for hash in hashes.values() {
                need(
                    hash.as_str().is_some_and(|s| hex(s, 64)),
                    "invalid artifact hash",
                )?;
            }
        }
    }
    let s = document(i, "submission")?;
    context(&s, head, tree)?;
    equal(&a, &s, "/reviewer_runtime", "submission runtime")?;
    for field in [
        "assignment_sha256",
        "source_report_sha256",
        "artifact_report_sha256",
        "request_sha256",
        "response_sha256",
        "body_sha256",
        "review_id",
        "required_ci",
    ] {
        equal(&s, &o, &format!("/{field}"), "submission origin")?;
    }
    let command = &s["command"];
    need(
        number(command, "/exit_code")? == 0,
        "submission command failed",
    )?;
    need(
        text(command, "/tool_record_sha256")? == text(&o, "/submission_record_sha256")?,
        "submission tool record mismatch",
    )?;
    equal(&s, &o, "/command", "observed submission command")?;
    let argv = array(command, "/argv")?;
    need(
        !argv.is_empty()
            && argv
                .iter()
                .all(|v| v.as_str().is_some_and(|s| !s.is_empty())),
        "missing submission argv",
    )?;
    let args: Vec<_> = argv.iter().filter_map(Value::as_str).collect();
    let repo = text(&a, "/repository")?;
    let parts: Vec<_> = repo.split('/').collect();
    need(
        parts.len() == 2
            && parts.iter().all(|p| {
                !p.is_empty()
                    && p.bytes()
                        .all(|c| c.is_ascii_alphanumeric() || b"-_.".contains(&c))
            }),
        "invalid repository",
    )?;
    let pr = number(&a, "/pull_request")?;
    let endpoint = format!("repos/{repo}/pulls/{pr}/reviews");
    need(
        pr > 0
            && args[0].rsplit('/').next() == Some("gh")
            && args.contains(&"api")
            && args.contains(&endpoint.as_str())
            && args
                .windows(2)
                .any(|w| matches!(w[0], "--method" | "-X") && w[1] == "POST")
            && args
                .windows(2)
                .any(|w| w[0] == "--input" && !w[1].starts_with('-')),
        "unbound review submission command",
    )?;
    let request = document(i, "request")?;
    need(
        text(&request, "/event")? == "APPROVE" && text(&request, "/commit_id")? == head,
        "request not explicit exact-head APPROVE",
    )?;
    let review = document(i, "review")?;
    let response = document(i, "response")?;
    let id = number(&o, "/review_id")?;
    need(
        id > 0 && number(&review, "/id")? == id,
        "wrong designated review ID",
    )?;
    need(
        text(&review, "/state")? == "APPROVED",
        "review state is not APPROVED",
    )?;
    need(
        text(&review, "/commit_id")? == head,
        "review commit mismatch",
    )?;
    need(
        text(&review, "/user/login")? == text(&a, "/github_login")?
            && number(&review, "/user/id")? == number(&a, "/github_user_id")?,
        "review account mismatch",
    )?;
    for field in [
        "/id",
        "/user/id",
        "/user/login",
        "/state",
        "/commit_id",
        "/submitted_at",
        "/html_url",
        "/pull_request_url",
        "/body",
    ] {
        equal(&review, &response, field, "submission API response")?;
    }
    need(
        text(&review, "/html_url")?
            == format!("https://github.com/{repo}/pull/{pr}#pullrequestreview-{id}")
            && text(&review, "/pull_request_url")?
                == format!("https://api.github.com/repos/{repo}/pulls/{pr}"),
        "review URL mismatch",
    )?;
    equal(&review, &o, "/submitted_at", "submitted time")?;
    let submitted = utc_seconds(text(&review, "/submitted_at")?)?;
    need(
        issued <= submitted && submitted <= start,
        "review predates assignment or follows fetch",
    )?;
    let body = text(&review, "/body")?;
    need(
        body == text(&request, "/body")? && digest(body.as_bytes()) == text(&o, "/body_sha256")?,
        "review body mismatch",
    )?;
    let block = binding_block(&a, &o, assignment_hash)?;
    need(
        body.matches("BEGIN JARAMBIT REVIEW ORIGIN V1").count() == 1 && body.contains(&block),
        "review body missing exact provenance binding",
    )?;
    let required = array(&a, "/required_check_names")?;
    let ci = array(&o, "/required_ci")?;
    let mut names = BTreeSet::new();
    for name in required {
        need(
            name.as_str()
                .is_some_and(|s| !s.is_empty() && names.insert(s.to_owned())),
            "invalid/duplicate required check name",
        )?;
    }
    need(
        !names.is_empty() && names.len() == ci.len(),
        "missing required CI bindings",
    )?;
    let checks_doc = document(i, "checks")?;
    let runs_doc = document(i, "runs")?;
    let checks = complete_collection(&checks_doc, "check_runs")?;
    let runs = complete_collection(&runs_doc, "workflow_runs")?;
    let mut selected = BTreeSet::new();
    for item in ci {
        let name = text(item, "/name")?;
        need(
            names.contains(name) && selected.insert(name.to_owned()),
            "unassigned/duplicate required CI",
        )?;
        let check_id = number(item, "/check_run_id")?;
        let run_id = number(item, "/workflow_run_id")?;
        let attempt = number(item, "/workflow_run_attempt")?;
        need(
            check_id > 0 && run_id > 0 && attempt > 0,
            "invalid CI ID/attempt",
        )?;
        let check = by_id(checks, check_id, "required check")?;
        let run = by_id(runs, run_id, "required workflow run")?;
        need(
            text(&check, "/name")? == name
                && text(&check, "/head_sha")? == head
                && text(&check, "/status")? == "completed"
                && text(&check, "/conclusion")? == "success",
            "required check not exact-head success",
        )?;
        need(
            !checks.iter().any(|c| {
                c.get("name").and_then(Value::as_str) == Some(name)
                    && c.get("id")
                        .and_then(Value::as_u64)
                        .is_some_and(|n| n > check_id)
            }),
            "required check superseded",
        )?;
        need(
            utc_seconds(text(&check, "/completed_at")?)? <= submitted,
            "CI completed after formal approval",
        )?;
        let run_url = format!("https://github.com/{repo}/actions/runs/{run_id}");
        let detail = text(&check, "/details_url")?;
        let suffix = detail
            .strip_prefix(&run_url)
            .ok_or("check/workflow URL mismatch")?;
        need(
            suffix.is_empty()
                || suffix
                    .strip_prefix("/job/")
                    .is_some_and(|s| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())),
            "check/workflow ID mismatch",
        )?;
        need(
            text(&run, "/head_sha")? == head
                && text(&run, "/status")? == "completed"
                && text(&run, "/conclusion")? == "success"
                && number(&run, "/run_attempt")? == attempt
                && text(&run, "/repository/full_name")? == repo
                && text(&run, "/html_url")? == run_url,
            "required workflow not exact-head success",
        )?;
    }
    Ok(
        json!({"schema":"jarambit.verified_review_origin.v1","review_origin":"VERIFIED",
        "head":head,"tree":tree,"review_id":id,"review_url":review["html_url"],
        "assignment_sha256":assignment_hash,"origin_sha256":origin_hash,"reviewer_runtime":reviewer,
        "required_ci":ci,"checked_at_unix":now,"merge_action_performed":false,
        "remaining_guard":"merge owner must still check current open sole PR and expected head immediately before merge"}),
    )
}
fn run() -> Check<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() == 1 && args[0] == "--help" {
        println!("Read-only review-origin guard. Required value flags:\n{}\n{}\nPins must come from the trusted merge owner, never reviewer files. No ignore/override/auth/merge mode.",
            FILES.iter().map(|s| format!("--{s} PATH")).collect::<Vec<_>>().join(" "),
            PINS.iter().map(|s| format!("--{s} HEX")).collect::<Vec<_>>().join(" "));
        return Err("help only; review origin not verified".into());
    }
    need(
        !args.is_empty() && args.len() % 2 == 0,
        "all flags require one value; use --help",
    )?;
    let mut values = BTreeMap::new();
    for pair in args.chunks_exact(2) {
        let key = pair[0].strip_prefix("--").ok_or("invalid flag")?;
        need(
            FILES.contains(&key) || PINS.contains(&key),
            "unknown flag; no bypass supported",
        )?;
        need(
            !pair[1].is_empty() && values.insert(key.to_owned(), pair[1].clone()).is_none(),
            "duplicate/empty flag",
        )?;
    }
    let mut i = Inputs {
        bytes: BTreeMap::new(),
        pins: BTreeMap::new(),
    };
    for name in FILES {
        let path = values
            .get(name)
            .ok_or_else(|| format!("missing --{name}"))?;
        need(
            fs::metadata(path)
                .map_err(|e| format!("cannot inspect {name}: {e}"))?
                .len()
                <= 16 * 1024 * 1024,
            "input exceeds16MiB; use bounded actual-record export",
        )?;
        let bytes = fs::read(path).map_err(|e| format!("cannot read {name}: {e}"))?;
        need(bytes.len() <= 16 * 1024 * 1024, "input grew beyond16MiB")?;
        i.bytes.insert(name.into(), bytes);
    }
    for name in PINS {
        i.pins.insert(
            name.into(),
            values
                .get(name)
                .ok_or_else(|| format!("missing --{name}"))?
                .clone(),
        );
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "system time before epoch")?
        .as_secs();
    println!(
        "{}",
        serde_json::to_string_pretty(&verify(&i, now)?).map_err(|e| e.to_string())?
    );
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("REVIEW_ORIGIN_BLOCKED: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // All records below are explicitly neutral synthetic data. The fixture-only
    // repinning helper is inaccessible to the executable and is NOT authority.
    fn encode(v: &Value) -> Vec<u8> {
        serde_json::to_vec(v).expect("neutral JSON")
    }
    fn repin(i: &mut Inputs) {
        let mut a = document(i, "assignment").unwrap();
        a["authors_sha256"] = digest(raw(i, "authors").unwrap()).into();
        a["dispatch_record_sha256"] = digest(raw(i, "dispatch-record").unwrap()).into();
        i.bytes.insert("assignment".into(), encode(&a));
        let ah = digest(raw(i, "assignment").unwrap());
        i.pins.insert("assignment-sha256".into(), ah.clone());
        let mut s = document(i, "submission").unwrap();
        s["assignment_sha256"] = ah.clone().into();
        for name in ["source-report", "artifact-report", "request", "response"] {
            s[format!("{}_sha256", name.replace('-', "_"))] = digest(raw(i, name).unwrap()).into();
        }
        i.bytes.insert("submission".into(), encode(&s));
        let mut o = document(i, "origin").unwrap();
        o["assignment_sha256"] = ah.into();
        for name in [
            "source-report",
            "artifact-report",
            "submission",
            "request",
            "response",
            "review",
            "checks",
            "runs",
            "submission-record",
            "fetch-record",
        ] {
            o[format!("{}_sha256", name.replace('-', "_"))] = digest(raw(i, name).unwrap()).into();
        }
        i.bytes.insert("origin".into(), encode(&o));
        i.pins
            .insert("origin-sha256".into(), digest(raw(i, "origin").unwrap()));
    }
    #[test]
    fn neutral_review_origin_positive_and_negative_cases() {
        let fixture: Value = serde_json::from_str(include_str!(
            "fixtures/verify-review-origin-neutral.v1.json"
        ))
        .unwrap();
        assert_eq!(fixture["fixture_only"], true);
        for case in fixture["cases"].as_array().unwrap() {
            let mut input = Inputs {
                bytes: BTreeMap::new(),
                pins: BTreeMap::new(),
            };
            for (k, v) in fixture["documents"].as_object().unwrap() {
                input
                    .bytes
                    .insert(k.clone(), v.as_str().unwrap().as_bytes().to_vec());
            }
            for (k, v) in fixture["pins"].as_object().unwrap() {
                input.pins.insert(k.clone(), v.as_str().unwrap().into());
            }
            for mutation in case["mutations"].as_array().unwrap() {
                let name = mutation["document"].as_str().unwrap();
                if mutation["remove_document"] == true {
                    input.bytes.remove(name);
                    continue;
                }
                let mut doc = document(&input, name).unwrap();
                *doc.pointer_mut(mutation["pointer"].as_str().unwrap())
                    .unwrap() = mutation["value"].clone();
                input.bytes.insert(name.into(), encode(&doc));
            }
            if case["repin_owner_fixture"] == true {
                repin(&mut input);
            }
            let now = case
                .get("now_override")
                .and_then(Value::as_u64)
                .unwrap_or_else(|| fixture["now_unix"].as_u64().unwrap());
            let result = verify(&input, now);
            if case["expect_pass"] == true {
                assert!(result.is_ok(), "{}: {:?}", case["name"], result);
            } else {
                let error = result.expect_err("neutral negative must block");
                assert!(
                    error.contains(case["expected_error"].as_str().unwrap()),
                    "{}: {}",
                    case["name"],
                    error
                );
            }
            eprintln!(
                "NEUTRAL_FIXTURE {} expected disposition observed",
                case["name"]
            );
        }
    }
}
