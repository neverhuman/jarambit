//! Categorical projection of retained battery records, never a scientific evaluator.
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

type Result<T> = std::result::Result<T, String>;
const STATUSES: [&str; 4] = ["pass", "fail", "blocked", "incomplete"];

fn string(v: &Value) -> Result<&str> {
    v.as_str()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "missing battery string".into())
}
fn strings(v: &Value) -> Result<Vec<&str>> {
    let result = v
        .as_array()
        .ok_or("missing battery array")?
        .iter()
        .map(string)
        .collect::<Result<Vec<_>>>()?;
    if result.is_empty() || result.iter().copied().collect::<BTreeSet<_>>().len() != result.len() {
        return Err("empty or duplicate battery membership".into());
    }
    Ok(result)
}
fn color(status: &str) -> Result<&'static str> {
    match status {
        "pass" => Ok("#007e87"),
        "fail" => Ok("#a3442d"),
        "blocked" => Ok("#697983"),
        "incomplete" => Ok("#8c651c"),
        _ => Err("unknown battery status".into()),
    }
}
fn symbol(status: &str) -> Result<&'static str> {
    match status {
        "pass" => Ok("P"),
        "fail" => Ok("F"),
        "blocked" => Ok("B"),
        "incomplete" => Ok("I"),
        _ => Err("unknown battery status".into()),
    }
}
fn zero_counts() -> BTreeMap<String, usize> {
    STATUSES.into_iter().map(|s| (s.into(), 0)).collect()
}

pub(crate) fn project(manifest: &Value, records: &BTreeMap<String, Value>) -> Result<Value> {
    if manifest["schema"] != "jarambit.cad_battery.manifest.v1" {
        return Err("wrong battery manifest schema".into());
    }
    let subjects = strings(&manifest["subjects"])?;
    let columns = strings(&manifest["case_ids"])?;
    let files = strings(&manifest["files"])?;
    let revision = string(&manifest["source_commit"])?;
    if files.len() != subjects.len() || files.len() != records.len() {
        return Err("battery file/subject count differs".into());
    }
    let mut by_subject = BTreeMap::new();
    for file in &files {
        let record = records.get(*file).ok_or("missing battery record")?;
        let subject = string(&record["subject"])?;
        if Path::new(file).file_stem().and_then(|x| x.to_str()) != Some(subject)
            || record["schema"] != "jarambit.cad_battery.subject.v1"
            || by_subject.insert(subject, (*file, record)).is_some()
        {
            return Err("battery record identity differs".into());
        }
    }
    if by_subject.keys().copied().collect::<BTreeSet<_>>() != subjects.iter().copied().collect() {
        return Err("battery subject membership differs".into());
    }
    let mut counts = zero_counts();
    let mut column_counts = columns
        .iter()
        .map(|id| ((*id).to_string(), zero_counts()))
        .collect::<BTreeMap<_, _>>();
    let mut roles = BTreeMap::<String, usize>::new();
    let mut source_revisions = BTreeMap::<String, usize>::new();
    let mut rows = Vec::new();
    let mut cells = Vec::new();
    let mut shared_build_pass_references = 0;
    for (row, subject) in subjects.iter().enumerate() {
        let (file, record) = by_subject[subject];
        let record_revision = string(&record["source_commit"])?;
        *source_revisions.entry(record_revision.into()).or_default() += 1;
        let role = string(&record["role"])?;
        if !["native_cad", "archived_not_on_main", "genome"].contains(&role) {
            return Err("unknown battery role".into());
        }
        *roles.entry(role.into()).or_default() += 1;
        let cases = record["cases"].as_array().ok_or("missing battery cases")?;
        let mut by_id = BTreeMap::new();
        for (index, case) in cases.iter().enumerate() {
            if by_id.insert(string(&case["id"])?, (index, case)).is_some() {
                return Err("duplicate battery case".into());
            }
        }
        if by_id.keys().copied().collect::<BTreeSet<_>>() != columns.iter().copied().collect() {
            return Err("battery case membership differs".into());
        }
        let mut row_counts = zero_counts();
        for (column, id) in columns.iter().enumerate() {
            let (index, case) = by_id[id];
            let status = string(&case["status"])?;
            let cell_color = color(status)?;
            let cell_symbol = symbol(status)?;
            if case["fresh_this_lane"].as_bool() != Some(false)
                || case["source_commit"] != record_revision
            {
                return Err("battery freshness or source revision differs".into());
            }
            string(&case["kind"])?;
            for key in ["solver", "geometry_sha256", "evidence_path"] {
                let value = case.get(key).ok_or("missing battery nullable field")?;
                if !value.is_null() {
                    string(value)?;
                }
            }
            *counts.get_mut(status).ok_or("unknown battery status")? += 1;
            *row_counts.get_mut(status).ok_or("unknown battery status")? += 1;
            *column_counts
                .get_mut(*id)
                .ok_or("unknown battery column")?
                .get_mut(status)
                .ok_or("unknown battery status")? += 1;
            if id.starts_with("build.") && status == "pass" {
                shared_build_pass_references += 1;
            }
            cells.push(json!({"row":row,"column":column,"subject":subject,"role":role,"evaluator":id,
                "status":status,"symbol":cell_symbol,"color":cell_color,"record_source":file,"source_pointer":format!("/cases/{index}"),
                "kind":case["kind"],"solver":case["solver"],"evidence_path":case["evidence_path"],"source_commit":case["source_commit"],
                "geometry_sha256":case["geometry_sha256"],"fresh_this_lane":case["fresh_this_lane"]}));
        }
        if record["pass_count"].as_u64() != Some(row_counts["pass"] as u64) {
            return Err("battery stored row pass count differs".into());
        }
        rows.push(json!({"subject":subject,"role":role,"record_source":file,"source_commit":record_revision,"stored_status_counts":row_counts}));
    }
    Ok(
        json!({"schema":"jarambit.battery_matrix_projection.v1","manifest_source_revision":revision,"record_source_revision_counts":source_revisions,
        "scope":"Historical stored labels, not independent physical experiments; no new scientific execution",
        "row_order":"manifest subjects","column_order":"manifest case_ids","rows":rows,"columns":columns,
        "cells":cells,"stored_status_counts":counts,"per_evaluator":column_counts,"role_counts":roles,
        "shared_build_pass_references":shared_build_pass_references,"all_fresh_this_lane":false,
        "palette":{"pass":"#007e87","fail":"#a3442d","blocked":"#697983","incomplete":"#8c651c"}}),
    )
}

fn xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
fn label(svg: &mut String, x: usize, y: usize, size: usize, value: &str) {
    svg.push_str(&format!(
        "<text x=\"{x}\" y=\"{y}\" font-size=\"{size}\">{}</text>\n",
        xml(value)
    ));
}
pub(crate) fn svg(p: &Value) -> Result<String> {
    let rows = p["rows"].as_array().ok_or("missing matrix rows")?;
    let columns = strings(&p["columns"])?;
    let cells = p["cells"].as_array().ok_or("missing matrix cells")?;
    if rows.len() != 23 || columns.len() != 18 || cells.len() != 414 {
        return Err("matrix layout requires admitted 23 by 18 corpus".into());
    }
    if p["role_counts"]["native_cad"] != 3
        || p["role_counts"]["archived_not_on_main"] != 1
        || p["role_counts"]["genome"] != 19
        || p["shared_build_pass_references"] != 92
        || p["record_source_revision_counts"]["6d18ef8cbe7b995890451a8ac7b93b75c928d150"] != 21
        || p["record_source_revision_counts"]["4e54019dd016ddde71b416460b96e0a9b08a95de"] != 2
        || rows[..4].iter().any(|r| r["role"] == "genome")
        || rows[4..].iter().any(|r| r["role"] != "genome")
    {
        return Err(
            "matrix annotations require admitted historical roles and source fields".into(),
        );
    }
    let mut out = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1120\" height=\"860\" viewBox=\"0 0 1120 860\"><rect width=\"1120\" height=\"860\" fill=\"#f5f4ef\"/><g font-family=\"Arial, sans-serif\" fill=\"#132d3b\">\n");
    label(
        &mut out,
        20,
        30,
        22,
        "HISTORICAL BATTERY / 414 RETAINED STATUS LABELS",
    );
    label(
        &mut out,
        20,
        58,
        18,
        "23 subjects x 18 evaluator columns; no new FEM execution or qualification",
    );
    for (i, status) in STATUSES.iter().enumerate() {
        let x = 20;
        let y = 76 + i * 30;
        out.push_str(&format!(
            "<rect x=\"{x}\" y=\"{y}\" width=\"22\" height=\"22\" fill=\"{}\"/>\n",
            color(status)?
        ));
        label(
            &mut out,
            x + 30,
            y + 18,
            18,
            &format!(
                "{}: {} ({})",
                symbol(status)?,
                status,
                p["stored_status_counts"][status]
            ),
        );
    }
    for (column, id) in columns.iter().enumerate() {
        let x = 230 + column * 40 + 25;
        out.push_str(&format!(
            "<text transform=\"translate({x},235) rotate(-90)\" font-size=\"18\">{}</text>\n",
            xml(id)
        ));
    }
    label(&mut out, 20, 236, 18, "Role / subject");
    label(&mut out, 975, 235, 16, "P/F/B/I; rev");
    for (row, record) in rows.iter().enumerate() {
        let y = 250 + row * 22;
        let prefix = match string(&record["role"])? {
            "native_cad" => "N",
            "archived_not_on_main" => "A",
            "genome" => "G",
            _ => return Err("unknown matrix role".into()),
        };
        label(
            &mut out,
            20,
            y + 16,
            18,
            &format!("{prefix} / {}", string(&record["subject"])?),
        );
        let c = &record["stored_status_counts"];
        label(
            &mut out,
            975,
            y + 16,
            17,
            &format!(
                "{}/{}/{}/{} {}",
                c["pass"],
                c["fail"],
                c["blocked"],
                c["incomplete"],
                if record["source_commit"] == p["manifest_source_revision"] {
                    "rA"
                } else {
                    "rB"
                }
            ),
        );
    }
    for cell in cells {
        let row = cell["row"].as_u64().ok_or("missing matrix row")? as usize;
        let column = cell["column"].as_u64().ok_or("missing matrix column")? as usize;
        let x = 230 + column * 40;
        let y = 250 + row * 22;
        let status = string(&cell["status"])?;
        out.push_str(&format!("<g id=\"battery-r{row}-c{column}\"><title>{}</title><rect x=\"{x}\" y=\"{y}\" width=\"39\" height=\"21\" fill=\"{}\"/><text x=\"{}\" y=\"{}\" font-size=\"16\" text-anchor=\"middle\" fill=\"white\">{}</text></g>\n",
            xml(&format!("{} / {}: {}; {}", string(&cell["subject"])?, string(&cell["evaluator"])?, status, string(&cell["kind"])?)), color(status)?, x+20, y+16, symbol(status)?));
    }
    for column in [0, 4, 8, 12, 16, 17, 18] {
        let x = 230 + column * 40;
        out.push_str(&format!(
            "<path d=\"M{x} 246 V758\" stroke=\"#f5f4ef\" stroke-width=\"3\"/>\n"
        ));
    }
    out.push_str("<path d=\"M18 337 H1100\" stroke=\"#132d3b\" stroke-width=\"1\"/>\n");
    label(
        &mut out,
        20,
        780,
        17,
        "N: native CAD (3); A: archived_not_on_main (1); G: genome (19). Manifest order retained.",
    );
    label(&mut out, 20, 802, 17, "Source tags: rA=6d18ef8c (21 subjects); rB=4e54019d (2). These are retained provenance fields.");
    label(&mut out, 20, 824, 17, "Four build columns repeat 92 shared pass references; solid.tip pass means record completeness.");
    label(&mut out, 20, 846, 17, "All 414 cells: fresh_this_lane=false. Colors are stored labels, not stress or physical reliability.");
    out.push_str("</g></svg>\n");
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Value, BTreeMap<String, Value>) {
        let manifest = json!({"schema":"jarambit.cad_battery.manifest.v1","source_commit":"fixture_revision","subjects":["alpha","beta"],"case_ids":["build.analytic","plate.loads"],"files":["alpha.json","beta.json"]});
        let records = ["alpha", "beta"].into_iter().map(|s| {
            let cases = ["plate.loads", "build.analytic"].into_iter().map(|id| json!({"id":id,"status":if id=="build.analytic" {"pass"} else {"blocked"},"kind":"software_contract","solver":null,"geometry_sha256":null,"fresh_this_lane":false,"source_commit":"fixture_revision","evidence_path":"neutral/control.rs"})).collect::<Vec<_>>();
            (format!("{s}.json"),json!({"schema":"jarambit.cad_battery.subject.v1","subject":s,"role":"genome","source_commit":"fixture_revision","pass_count":1,"cases":cases}))
        }).collect();
        (manifest, records)
    }
    #[test]
    fn case_identity_controls_columns_not_source_position() {
        let (m, mut records) = fixture();
        records.get_mut("alpha.json").unwrap()["cases"][0]["evidence_path"] = Value::Null;
        let p = project(&m, &records).unwrap();
        assert_eq!(p["cells"][0]["evaluator"], "build.analytic");
        assert_eq!(p["cells"][0]["source_pointer"], "/cases/1");
        assert_eq!(p["stored_status_counts"]["blocked"], 2);
        assert!(p["cells"][0]["geometry_sha256"].is_null());
        assert!(p["cells"][1]["evidence_path"].is_null());
        assert!(svg(&p).is_err()); // A neutral unit fixture is not a publishable corpus.
    }
    #[test]
    fn mixed_record_revisions_are_preserved_not_replaced_by_manifest() {
        let (m, mut records) = fixture();
        let beta = records.get_mut("beta.json").unwrap();
        beta["source_commit"] = json!("fixture_other_revision");
        for case in beta["cases"].as_array_mut().unwrap() {
            case["source_commit"] = json!("fixture_other_revision");
        }
        let p = project(&m, &records).unwrap();
        assert_eq!(p["manifest_source_revision"], "fixture_revision");
        assert_eq!(p["record_source_revision_counts"]["fixture_revision"], 1);
        assert_eq!(
            p["record_source_revision_counts"]["fixture_other_revision"],
            1
        );
        assert_eq!(p["rows"][1]["source_commit"], "fixture_other_revision");
        assert_eq!(p["cells"][2]["source_commit"], "fixture_other_revision");
    }
    #[test]
    fn inconsistent_membership_status_freshness_and_tally_are_refused() {
        let (m, original) = fixture();
        for (key, value) in [
            ("status", json!("success")),
            ("fresh_this_lane", json!("false")),
            ("fresh_this_lane", json!(true)),
            ("source_commit", json!("other")),
            ("kind", Value::Null),
        ] {
            let mut records = original.clone();
            records.get_mut("alpha.json").unwrap()["cases"][0][key] = value;
            assert!(project(&m, &records).is_err(), "{key}");
        }
        let mut records = original.clone();
        records.get_mut("alpha.json").unwrap()["pass_count"] = json!(2);
        assert!(project(&m, &records).is_err());
        let mut records = original.clone();
        records.get_mut("alpha.json").unwrap()["cases"][0]["id"] = json!("build.analytic");
        assert!(project(&m, &records).is_err());
        let mut changed = m.clone();
        changed["subjects"][1] = json!("alpha");
        assert!(project(&changed, &original).is_err());
    }
}
