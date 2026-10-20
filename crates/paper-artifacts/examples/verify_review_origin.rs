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
const FILES: [&str; 17] = [
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
    "policy",
    "required-contexts",
    "policy-record",
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

// Source-derived unconditional pull_request jobs from the reviewed repository
// workflow bytes. Assignment/owner policy may add fresh requirements, never drop
// any of these jobs or replace their workflow identity.
const PRODUCTION_CI: [(&str, &str, &str, &str); 7] = [
    ("VERSION == tag == WHITEPAPER header", ".github/workflows/version-check.yml", "version", "version-check"),
    ("cargo test --release + D1 gate smoke", ".github/workflows/d1-gates.yml", "test", "d1-gates"),
    ("jankurai repo score & repair queue", ".github/workflows/jankurai.yml", "audit", "jankurai"),
    ("local runner policy (xbabe1)", ".github/workflows/runner-policy.yml", "local-runners", "runner-policy"),
    ("local runner policy (xbabe2)", ".github/workflows/runner-policy.yml", "local-runners", "runner-policy"),
    ("local runner policy (xbabe3)", ".github/workflows/runner-policy.yml", "local-runners", "runner-policy"),
    ("local runner policy (xbabe4)", ".github/workflows/runner-policy.yml", "local-runners", "runner-policy"),
];
const REQUIRED_ARTIFACTS: [&str; 11] = [
    "docs/paper/main.pdf", "docs/paper/supplement.pdf",
    "target/ieee/paper/main.pdf", "target/ieee/paper/supplement-main.pdf",
    "target/ieee/paper/supplement.pdf", "target/ieee/paper/INPUT_FILES",
    "target/ieee/paper/SOURCE_SHA256SUMS", "target/ieee/paper/OUTPUT_SHA256SUMS",
    "target/ieee/paper/toolchain.txt", "docs/paper/generated/manifest.json",
    "docs/paper/publication.v2.json",
];
struct ProductionPolicy {
    document: Value,
    contexts: BTreeMap<String, Value>,
    required_apps: BTreeMap<String, u64>,
}
fn map<'a>(v: &'a Value, key: &str) -> Check<&'a serde_json::Map<String, Value>> {
    v.get(key).and_then(Value::as_object).ok_or_else(|| format!("missing object {key}"))
}
fn canonical_path(name: &str, base: &str, checkout: &str) -> Check<String> {
    need(!name.is_empty() && !name.chars().any(|c| matches!(c, '\0' | '\n' | '\r' | '\\')), "invalid closure path")?;
    let owned;
    let relative = if let Some(rest) = name.strip_prefix(&format!("{checkout}/")) { rest }
        else if name.starts_with('/') { return Ok(format!("external:{name}")); }
        else { owned = format!("{base}{name}"); &owned };
    let mut parts = Vec::new();
    for part in relative.split('/') {
        match part { "" | "." => (), ".." => { need(parts.pop().is_some(), "closure path escapes checkout")?; }, _ => parts.push(part) }
    }
    need(!parts.is_empty(), "empty canonical closure path")?;
    Ok(parts.join("/"))
}
fn checksum_rows(contents: &str, checkout: &str) -> Check<BTreeMap<String, String>> {
    let mut rows = BTreeMap::new();
    for line in contents.lines() {
        need(line.len() > 66 && line.as_bytes()[..66].is_ascii() && hex(&line[..64], 64) && matches!(&line[64..66], "  " | " *"), "malformed closure checksum")?;
        let name = canonical_path(&line[66..], "docs/paper/", checkout)?;
        if let Some(old) = rows.insert(name, line[..64].to_owned()) { need(old == line[..64], "conflicting closure checksum")?; }
    }
    need(!rows.is_empty(), "empty closure checksums")?;
    Ok(rows)
}
fn covered(files: &serde_json::Map<String, Value>, name: &str, hash: &str) -> Check<()> {
    need(hex(hash, 64) && files.get(name).and_then(Value::as_str) == Some(hash), "artifact closure hash/path missing or changed")
}
fn fresh_contexts(i: &Inputs, a: &Value, o: &Value) -> Check<(BTreeSet<String>, BTreeMap<String, u64>)> {
    let f = document(i, "required-contexts")?;
    need(text(&f, "/schema")? == "jarambit.required_context_capture.v1", "wrong fresh policy schema")?;
    context(&f, text(a, "/head")?, text(a, "/tree")?)?;
    equal(a, &f, "/repository", "fresh policy repository")?;
    equal(a, &f, "/merge_owner_runtime_id", "fresh policy owner")?;
    need(text(&f, "/base_branch")? == "main", "wrong fresh policy base")?;
    need(number(&o, "/fetch_started_at_unix")? <= number(&f, "/fetch_started_at_unix")?
        && number(&f, "/fetch_started_at_unix")? <= number(&f, "/fetched_at_unix")?
        && number(&f, "/fetched_at_unix")? <= number(&o, "/fetched_at_unix")?, "fresh policy fetch interval mismatch")?;
    let captures = array(&f, "/captures")?;
    need(captures.len() == 3, "incomplete required-context API captures")?;
    let repo = text(a, "/repository")?;
    let mut bodies = BTreeMap::new();
    for capture in captures {
        let kind = text(capture, "/kind")?;
        let suffix = match kind { "branch" => "", "legacy" => "/protection", "rules" => "/rules", _ => return Err("unknown required-context capture".into()) };
        let endpoint = if kind == "rules" { format!("repos/{repo}/rules/branches/main") }
            else { format!("repos/{repo}/branches/main{suffix}") };
        need(text(capture, "/endpoint")? == endpoint, "required-context endpoint mismatch")?;
        let body = text(capture, "/body_utf8")?;
        need(digest(body.as_bytes()) == text(capture, "/body_sha256")?, "required-context response hash mismatch")?;
        let value: Value = serde_json::from_str(body).map_err(|_| "invalid required-context API JSON")?;
        need(bodies.insert(kind.to_owned(), (number(capture, "/http_status")?, value)).is_none(), "duplicate required-context capture")?;
    }
    let (branch_status, branch) = bodies.get("branch").ok_or("missing branch policy capture")?;
    need(*branch_status == 200, "branch policy unavailable")?;
    let protected = branch.get("protected").and_then(Value::as_bool).ok_or("branch protected flag unavailable")?;
    let mut names = BTreeSet::new();
    let mut apps = BTreeMap::new();
    let mut add = |name: &str, app: Option<u64>| -> Check<()> {
        need(!name.is_empty(), "empty fresh required context")?; names.insert(name.to_owned());
        if let Some(id) = app { if let Some(old) = apps.insert(name.to_owned(), id) { need(old == id, "required context app conflict")?; } }
        Ok(())
    };
    let (legacy_status, legacy) = bodies.get("legacy").ok_or("missing legacy policy capture")?;
    if *legacy_status == 200 {
        let required = legacy.get("required_status_checks").ok_or("missing legacy required checks field")?;
        if !required.is_null() {
            for name in array(required, "/contexts")? { add(name.as_str().ok_or("invalid legacy context")?, None)?; }
            if let Some(checks) = required.get("checks") {
                for check in checks.as_array().ok_or("invalid legacy check app bindings")? {
                    add(text(check, "/context")?, check.get("app_id").and_then(Value::as_u64))?;
                }
            }
        }
    } else { need(!protected && *legacy_status == 404, "legacy policy unavailable; no empty-policy inference")?; }
    let (rules_status, rules) = bodies.get("rules").ok_or("missing rules policy capture")?;
    need(*rules_status == 200, "effective branch rules unavailable")?;
    let effective_rules = rules.as_array().ok_or("invalid effective branch rules")?;
    // This exact endpoint uses GitHub's default 30-item page. Without a
    // complete-page/header capture protocol, a full page cannot prove that a
    // later page has no additional required rule. Refuse the bounded case.
    need(
        effective_rules.len() < 30,
        "effective-rules array reaches default page bound; complete capture support required",
    )?;
    for rule in effective_rules {
        match text(rule, "/type")? {
            "required_status_checks" => for check in array(rule, "/parameters/required_status_checks")? {
                add(text(check, "/context")?, check.get("integration_id").and_then(Value::as_u64))?;
            },
            "workflows" => return Err("required workflow rule needs explicit reviewed support; cannot omit".into()),
            "creation" | "update" | "deletion" | "required_linear_history" | "required_signatures" | "pull_request" | "non_fast_forward" | "commit_message_pattern" | "commit_author_email_pattern" | "committer_email_pattern" | "branch_name_pattern" | "tag_name_pattern" => (),
            _ => return Err("unsupported effective branch rule; cannot infer complete CI policy".into()),
        }
    }
    Ok((names, apps))
}
fn production_policy(i: &Inputs, a: &Value, o: &Value) -> Check<ProductionPolicy> {
    hash_is(i, "policy", text(a, "/policy_sha256")?)?;
    equal(a, o, "/policy_sha256", "pre-review policy")?;
    need(o.get("policy_checked_against_actual_required_contexts") == Some(&Value::Bool(true)), "owner has not verified actual required-context policy")?;
    let p = document(i, "policy")?;
    need(text(&p, "/schema")? == "jarambit.production_review_policy.v1", "wrong production policy schema")?;
    context(&p, text(a, "/head")?, text(a, "/tree")?)?;
    equal(a, &p, "/merge_owner_runtime_id", "pre-review policy owner")?;
    let files = map(&p, "artifact_sha256")?;
    for (name, hash) in files { need(!name.is_empty() && hash.as_str().is_some_and(|s| hex(s,64)), "invalid pinned artifact scope")?; }
    for name in REQUIRED_ARTIFACTS { need(files.contains_key(name), "missing mandatory publication artifact")?; }
    for (left, right) in [("docs/paper/main.pdf", "target/ieee/paper/main.pdf"), ("docs/paper/supplement.pdf", "target/ieee/paper/supplement.pdf"), ("docs/paper/supplement.pdf", "target/ieee/paper/supplement-main.pdf")] {
        need(files.get(left) == files.get(right), "main/companion PDF hash binding mismatch")?;
    }
    let checkout = text(&p, "/checkout_root")?;
    need(checkout.starts_with('/') && !checkout.ends_with('/'), "invalid frozen checkout root")?;
    let closure = map(&p, "closure_utf8")?;
    for name in ["target/ieee/paper/INPUT_FILES", "target/ieee/paper/SOURCE_SHA256SUMS", "target/ieee/paper/OUTPUT_SHA256SUMS", "target/ieee/paper/toolchain.txt", "docs/paper/generated/manifest.json"] {
        let contents = closure.get(name).and_then(Value::as_str).filter(|s| !s.is_empty()).ok_or("missing actual publication closure bytes")?;
        covered(files, name, &digest(contents.as_bytes()))?;
    }
    let source = checksum_rows(text(&Value::Object(closure.clone()), "/target~1ieee~1paper~1SOURCE_SHA256SUMS")?, checkout)?;
    let output = checksum_rows(text(&Value::Object(closure.clone()), "/target~1ieee~1paper~1OUTPUT_SHA256SUMS")?, checkout)?;
    let inputs: BTreeSet<_> = text(&Value::Object(closure.clone()), "/target~1ieee~1paper~1INPUT_FILES")?.lines()
        .map(|line| canonical_path(line, "docs/paper/", checkout)).collect::<Check<_>>()?;
    need(!inputs.is_empty() && inputs == source.keys().cloned().collect(), "INPUT_FILES/source closure membership mismatch")?;
    for (name, hash) in source.iter().chain(output.iter()) { covered(files, name, hash)?; }
    let generated: Value = serde_json::from_str(text(&Value::Object(closure.clone()), "/docs~1paper~1generated~1manifest.json")?).map_err(|_| "invalid generated manifest closure")?;
    need(text(&generated, "/schema")? == "jarambit.publication_manifest.v1", "wrong generated manifest schema")?;
    for kind in ["inputs", "outputs"] {
        let rows = map(&generated, kind)?; need(!rows.is_empty(), "empty generated input/output closure")?;
        for (name, hash) in rows { covered(files, &canonical_path(name, "", checkout)?, hash.as_str().ok_or("invalid generated closure hash")?)?; }
    }
    let workflows = map(&p, "workflows")?;
    for (path, wf) in workflows {
        need(path.starts_with(".github/workflows/") && path.ends_with(".yml"), "invalid source workflow path")?;
        need(number(wf, "/github_workflow_id")? > 0, "missing actual workflow identity")?;
        let bytes = text(wf, "/source_utf8")?;
        let hash = text(wf, "/source_sha256")?;
        need(digest(bytes.as_bytes()) == hash, "source workflow hash mismatch")?;
        covered(files, path, hash)?;
    }
    let mut contexts = BTreeMap::new();
    for row in array(&p, "/contexts")? {
        let name = text(row, "/name")?;
        let wf = workflows.get(text(row, "/workflow_path")?).ok_or("context source workflow missing")?;
        need(text(wf, "/source_utf8")?.contains(&format!("  {}:", text(row, "/job_id")?)), "context job absent from source workflow")?;
        need(contexts.insert(name.to_owned(), row.clone()).is_none(), "duplicate production context")?;
    }
    for (name, path, job, workflow_name) in PRODUCTION_CI {
        let row = contexts.get(name).ok_or("production minimum CI context omitted")?;
        let wf = workflows.get(path).ok_or("production workflow source omitted")?;
        need(text(row, "/workflow_path")? == path && text(row, "/job_id")? == job
            && text(wf, "/name")? == workflow_name, "production workflow/job identity changed")?;
    }
    let (fresh, apps) = fresh_contexts(i, a, o)?;
    let complete: BTreeSet<_> = PRODUCTION_CI.iter().map(|row| row.0.to_owned()).chain(fresh).collect();
    need(contexts.keys().cloned().collect::<BTreeSet<_>>() == complete, "production policy differs from complete fresh CI scope")?;
    let assigned = array(a, "/required_check_names")?;
    let mut assigned_names = BTreeSet::new();
    for name in assigned { need(name.as_str().is_some_and(|s| !s.is_empty() && assigned_names.insert(s.to_owned())), "invalid/duplicate required check name")?; }
    need(assigned_names == complete, "assignment narrowed or changed complete CI policy")?;
    Ok(ProductionPolicy {document:p, contexts, required_apps:apps})
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
        "required-contexts",
        "policy-record",
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
    let policy = production_policy(i, &a, &o)?;
    for name in ["source-report", "artifact-report"] {
        let report = document(i, name)?;
        context(&report, head, tree)?;
        equal(&a, &report, "/policy_sha256", "reviewed production scope")?;
        need(text(&report, "/schema")? == "jarambit.independent_publication_report.v1"
            && text(&report, "/scope")? == if name == "source-report" { "source" } else { "artifact" }, "wrong independent report scope/schema")?;
        need(text(&report, "/verdict")? == "PASS", "report is not PASS")?;
        equal(&a, &report, "/reviewer_runtime", "report runtime")?;
        if name == "artifact-report" {
            let hashes = report
                .get("artifact_sha256")
                .and_then(Value::as_object)
                .ok_or("missing artifact hash map")?;
            need(!hashes.is_empty(), "empty artifact hash map")?;
            need(hashes == map(&policy.document, "artifact_sha256")?, "artifact report differs from full Root-pinned scope")?;
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
        "policy_sha256",
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
    // Preserve the complete observed argv in the receipt. Only validation may
    // look through the host's exact optional `rtk proxy` shell prefix.
    let recorded_args: Vec<_> = argv.iter().filter_map(Value::as_str).collect();
    let args = if recorded_args[0].rsplit('/').next() == Some("rtk") {
        need(
            recorded_args.len() >= 3 && recorded_args[1] == "proxy",
            "unbound review submission command",
        )?;
        &recorded_args[2..]
    } else {
        recorded_args.as_slice()
    };
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
    let ci = array(&o, "/required_ci")?;
    let names: BTreeSet<_> = policy.contexts.keys().cloned().collect();
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
        let expected_context = policy.contexts.get(name).ok_or("unassigned CI context")?;
        let workflow_path = text(expected_context, "/workflow_path")?;
        let expected_workflow = map(&policy.document, "workflows")?.get(workflow_path).ok_or("missing CI source workflow")?;
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
        if let Some(app) = policy.required_apps.get(name) {
            need(number(&check, "/app/id")? == *app, "fresh required context app mismatch")?;
        }
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
        need(text(&run, "/path")? == workflow_path
            && number(&run, "/workflow_id")? == number(expected_workflow, "/github_workflow_id")?
            && text(&run, "/name")? == text(expected_workflow, "/name")?, "required source workflow identity mismatch")?;
    }
    Ok(
        json!({"schema":"jarambit.verified_review_origin.v1","review_origin":"VERIFIED",
        "head":head,"tree":tree,"review_id":id,"review_url":review["html_url"],
        "assignment_sha256":assignment_hash,"origin_sha256":origin_hash,"policy_sha256":a["policy_sha256"],"reviewer_runtime":reviewer,
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
        a["policy_sha256"] = digest(raw(i, "policy").unwrap()).into();
        i.bytes.insert("assignment".into(), encode(&a));
        let ah = digest(raw(i, "assignment").unwrap());
        i.pins.insert("assignment-sha256".into(), ah.clone());
        let mut s = document(i, "submission").unwrap();
        s["assignment_sha256"] = ah.clone().into();
        s["policy_sha256"] = a["policy_sha256"].clone();
        for name in ["source-report", "artifact-report", "request", "response"] {
            s[format!("{}_sha256", name.replace('-', "_"))] = digest(raw(i, name).unwrap()).into();
        }
        i.bytes.insert("submission".into(), encode(&s));
        let mut o = document(i, "origin").unwrap();
        o["assignment_sha256"] = ah.into();
        o["policy_sha256"] = a["policy_sha256"].clone();
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
            "required-contexts",
            "policy-record",
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
