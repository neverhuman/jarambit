//! A bounded view of recorded parent IDs, without inferring missing ancestors.
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
type Result<T> = std::result::Result<T, String>;

fn integer(value: &Value) -> Result<u64> {
    value
        .as_u64()
        .ok_or_else(|| "lineage integer is missing or untyped".into())
}
fn text(value: &Value) -> Result<&str> {
    value
        .as_str()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "missing lineage string".into())
}
fn parents(birth: &Value) -> Result<Vec<u64>> {
    let result = birth["parents"]
        .as_array()
        .ok_or("missing recorded parent list")?
        .iter()
        .map(integer)
        .collect::<Result<Vec<_>>>()?;
    if result.iter().copied().collect::<BTreeSet<_>>().len() != result.len() {
        return Err("duplicate recorded parent".into());
    }
    Ok(result)
}

pub(crate) fn project(births: &[Value], context: &Value, contract: &Value) -> Result<Value> {
    let focal = integer(&contract["focal_id"])?;
    let hops = integer(&contract["max_parent_hops"])?;
    if !(1..=4).contains(&hops) || context["best_observed"]["id"].as_u64() != Some(focal) {
        return Err("lineage focal identity or hop window differs".into());
    }
    let mut index = BTreeMap::new();
    for (position, birth) in births.iter().enumerate() {
        if index
            .insert(integer(&birth["id"])?, (position, birth))
            .is_some()
        {
            return Err("duplicate lineage birth ID".into());
        }
    }
    let mut selected = BTreeMap::new();
    let mut queue = VecDeque::from([(focal, 0)]);
    while let Some((id, depth)) = queue.pop_front() {
        if selected.contains_key(&id) {
            continue;
        }
        let (_, birth) = index
            .get(&id)
            .ok_or("unavailable parent inside lineage window")?;
        let generation = integer(&birth["gen"])?;
        let parent_ids = parents(birth)?;
        for parent in &parent_ids {
            if *parent == id {
                return Err("self-parent in recorded lineage".into());
            }
            if let Some((_, row)) = index.get(parent) {
                if integer(&row["gen"])? >= generation {
                    return Err("recorded parent is not from an earlier generation".into());
                }
            }
            if depth < hops {
                queue.push_back((*parent, depth + 1));
            }
        }
        selected.insert(id, depth);
    }
    let expected = contract["expected_node_ids"]
        .as_array()
        .ok_or("missing lineage node admission")?
        .iter()
        .map(integer)
        .collect::<Result<Vec<_>>>()?;
    if expected.iter().copied().collect::<BTreeSet<_>>().len() != expected.len()
        || expected.into_iter().collect::<BTreeSet<_>>() != selected.keys().copied().collect()
    {
        return Err("lineage node membership requires readmission".into());
    }
    let layout = contract["layout"]
        .as_object()
        .ok_or("missing lineage layout")?;
    if layout.len() != selected.len() {
        return Err("lineage layout membership differs".into());
    }
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut omitted = Vec::new();
    for (id, depth) in &selected {
        let (position, birth) = index[id];
        let fit = birth["fit"]
            .as_f64()
            .filter(|v| v.is_finite())
            .ok_or("invalid stored lineage fitness")?;
        let legal = birth["legal"].as_bool().ok_or("untyped lineage legality")?;
        let fem = text(&birth["fem_status"])?;
        if !["complete", "not_evaluated"].contains(&fem) {
            return Err("unknown lineage FEM status".into());
        }
        let operator = text(&birth["op"])?;
        let version = integer(&birth["op_version"])?;
        let coordinates = layout
            .get(&id.to_string())
            .ok_or("missing lineage node layout")?;
        let x = integer(&coordinates["x"])?;
        let y = integer(&coordinates["y"])?;
        if x + 205 > 1120 || y < 75 || y + 125 > 400 {
            return Err("lineage node outside display viewport".into());
        }
        let parent_ids = parents(birth)?;
        for (ordinal, parent) in parent_ids.iter().enumerate() {
            if selected.contains_key(parent) {
                edges.push(json!({"parent":parent,"child":id,"parent_ordinal":ordinal,"source_pointer":format!("/{position}/parents/{ordinal}")}));
            } else {
                omitted.push(json!({"parent":parent,"child":id,"parent_source_retained":index.contains_key(parent),"reason":"outside displayed parent-hop window","source_pointer":format!("/{position}/parents/{ordinal}")}));
            }
        }
        nodes.push(json!({"id":id,"generation":birth["gen"],"fit":fit,"operator":operator,"operator_version":version,
            "parents":parent_ids,"fem_status":fem,"legal":legal,"minimum_parent_hops":depth,"source_pointer":format!("/{position}"),"layout":{"x":x,"y":y}}));
    }
    let focal_row = index.get(&focal).ok_or("missing focal birth")?.1;
    if focal_row["gen"] != context["best_observed"]["generation"]
        || focal_row["fit"] != context["best_observed"]["fit"]
    {
        return Err("lineage focal observation differs from historical context".into());
    }
    Ok(
        json!({"schema":"jarambit.recorded_lineage_projection.v1","focal_id":focal,"max_parent_hops":hops,
        "source":contract["population"]["path"],"nodes":nodes,"edges":edges,"omitted_parent_edges":omitted,
        "focal_historical_assessment":context["best_observed"]["strict_assessment"],
        "assessment_join":{"source":contract["summary"]["path"],"array":"strict_certification","id_string":focal.to_string(),"generation_string":integer(&focal_row["gen"])?.to_string(),"boolean_conversion":"original true/false strings"},
        "scope":"Recorded parent links, not inferred inheritance, a full ancestry, a fixed physical-objective trajectory or current qualification",
        "layout_semantics":"Illustrative node placement; spacing does not encode generation, elapsed time or fitness"}),
    )
}

fn xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
fn label(svg: &mut String, x: u64, y: u64, size: usize, value: &str, color: &str) {
    svg.push_str(&format!(
        "<text x=\"{x}\" y=\"{y}\" font-size=\"{size}\" fill=\"{color}\">{}</text>\n",
        xml(value)
    ));
}
pub(crate) fn svg(p: &Value) -> Result<String> {
    let nodes = p["nodes"].as_array().ok_or("missing lineage nodes")?;
    let edges = p["edges"].as_array().ok_or("missing lineage edges")?;
    let omitted = p["omitted_parent_edges"]
        .as_array()
        .ok_or("missing lineage cutoff")?;
    if p["focal_id"] != 1095
        || nodes.len() != 5
        || edges.len() != 5
        || omitted.len() != 2
        || p["focal_historical_assessment"]["cert_pass_all"] != false
    {
        return Err("lineage display annotations require admitted historical case".into());
    }
    let omitted_pairs = omitted
        .iter()
        .map(|edge| Ok((integer(&edge["parent"])?, integer(&edge["child"])?)))
        .collect::<Result<BTreeSet<_>>>()?;
    if omitted_pairs != BTreeSet::from([(174, 245), (299, 392)])
        || omitted
            .iter()
            .any(|edge| edge["parent_source_retained"] != true)
    {
        return Err("lineage cutoff annotations differ from saved parent references".into());
    }
    let by_id = nodes
        .iter()
        .map(|n| Ok((integer(&n["id"])?, n)))
        .collect::<Result<BTreeMap<_, _>>>()?;
    let mut out=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1120\" height=\"450\" viewBox=\"0 0 1120 450\"><defs><marker id=\"parent-arrow\" markerWidth=\"7\" markerHeight=\"7\" refX=\"6\" refY=\"3.5\" orient=\"auto\"><path d=\"M0 0 L7 3.5 L0 7 Z\" fill=\"#697983\"/></marker></defs><rect width=\"1120\" height=\"450\" fill=\"#f5f4ef\"/><g font-family=\"Arial, sans-serif\">\n");
    label(
        &mut out,
        20,
        28,
        22,
        "RECORDED PARENT LINKS / HIGHEST HISTORICAL STORED SCORE",
        "#132d3b",
    );
    label(
        &mut out,
        20,
        55,
        18,
        "w282 cohort: five retained births, two parent hops; no inferred or imputed ancestry",
        "#132d3b",
    );
    for edge in edges {
        let parent = integer(&edge["parent"])?;
        let child = integer(&edge["child"])?;
        let a = by_id.get(&parent).ok_or("edge parent outside view")?;
        let b = by_id.get(&child).ok_or("edge child outside view")?;
        let sx = integer(&a["layout"]["x"])? + 205;
        let sy = integer(&a["layout"]["y"])? + 62;
        let tx = integer(&b["layout"]["x"])?;
        if tx <= sx {
            return Err("lineage layout must keep parent arrows rightward".into());
        }
        let parent_count = b["parents"].as_array().ok_or("missing parent ports")?.len();
        let ty = integer(&b["layout"]["y"])?
            + if parent_count == 1 {
                62
            } else {
                35 + integer(&edge["parent_ordinal"])? * 45
            };
        let bend = (tx - sx) / 2;
        out.push_str(&format!("<path id=\"lineage-edge-{parent}-{child}\" d=\"M{sx} {sy} C{} {sy} {} {ty} {tx} {ty}\" fill=\"none\" stroke=\"#697983\" stroke-width=\"2.5\" marker-end=\"url(#parent-arrow)\"/>\n",sx+bend,tx-bend));
    }
    for node in nodes {
        let id = integer(&node["id"])?;
        let x = integer(&node["layout"]["x"])?;
        let y = integer(&node["layout"]["y"])?;
        let focal = node["id"] == p["focal_id"];
        let color = if focal { "#a3442d" } else { "#007e87" };
        out.push_str(&format!("<g id=\"lineage-node-{id}\"><rect x=\"{x}\" y=\"{y}\" width=\"205\" height=\"125\" rx=\"6\" fill=\"white\" stroke=\"{color}\" stroke-width=\"2\"/>\n"));
        label(
            &mut out,
            x + 10,
            y + 24,
            20,
            &format!("ID {id} / gen {}", node["generation"]),
            color,
        );
        label(
            &mut out,
            x + 10,
            y + 48,
            17,
            &format!(
                "{} / v{}",
                text(&node["operator"])?,
                node["operator_version"]
            ),
            "#132d3b",
        );
        label(
            &mut out,
            x + 10,
            y + 72,
            19,
            &format!(
                "stored fit {:.5}",
                node["fit"].as_f64().ok_or("missing node fitness")?
            ),
            "#132d3b",
        );
        label(
            &mut out,
            x + 10,
            y + 94,
            15,
            &format!(
                "FEM {}; legal {}",
                text(&node["fem_status"])?,
                node["legal"]
            ),
            "#132d3b",
        );
        let footer = if focal {
            "later cert_pass_all=false".into()
        } else {
            let outside = omitted
                .iter()
                .filter(|e| e["child"] == node["id"])
                .map(|e| e["parent"].to_string())
                .collect::<Vec<_>>();
            if outside.is_empty() {
                "recorded parent links shown".into()
            } else {
                format!("outside-view parent: {}", outside.join(","))
            }
        };
        label(&mut out, x + 10, y + 115, 14, &footer, color);
        out.push_str("</g>\n");
    }
    label(
        &mut out,
        20,
        420,
        17,
        "Two-hop cutoff omits 174 -> 245 and 299 -> 392; both referenced births are retained.",
        "#132d3b",
    );
    label(&mut out,20,443,17,"Node spacing is layout only. Contextual fitness is not a fixed physical observable or qualification.","#132d3b");
    out.push_str("</g></svg>\n");
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Vec<Value>, Value, Value) {
        let births = vec![
            json!({"id":1,"gen":0,"fit":0.,"parents":[],"op":"seed","op_version":1,"legal":false,"fem_status":"not_evaluated"}),
            json!({"id":2,"gen":1,"fit":1.,"parents":[1],"op":"mutate","op_version":1,"legal":true,"fem_status":"complete"}),
            json!({"id":3,"gen":2,"fit":2.,"parents":[2],"op":"mutate","op_version":1,"legal":true,"fem_status":"complete"}),
        ];
        let context =
            json!({"best_observed":{"id":3,"generation":2,"fit":2.,"strict_assessment":null}});
        let contract = json!({"focal_id":3,"max_parent_hops":1,"expected_node_ids":[2,3],"layout":{"2":{"x":20,"y":90},"3":{"x":310,"y":90}}});
        (births, context, contract)
    }
    #[test]
    fn recorded_edges_and_cutoff_preserve_source_pointers() {
        let (b, c, k) = fixture();
        let p = project(&b, &c, &k).unwrap();
        assert_eq!(
            p["edges"],
            json!([{"parent":2,"child":3,"parent_ordinal":0,"source_pointer":"/2/parents/0"}])
        );
        assert_eq!(p["omitted_parent_edges"][0]["parent"], 1);
        assert_eq!(p["nodes"][0]["source_pointer"], "/1");
        assert!(p["focal_historical_assessment"].is_null());
        assert!(svg(&p).is_err());
        let mut missing_boundary = b.clone();
        missing_boundary[1]["parents"] = json!([999]);
        let p = project(&missing_boundary, &c, &k).unwrap();
        assert_eq!(p["nodes"].as_array().unwrap().len(), 2);
        assert_eq!(p["omitted_parent_edges"][0]["parent"], 999);
        assert_eq!(
            p["omitted_parent_edges"][0]["parent_source_retained"],
            false
        );
    }
    #[test]
    fn unknown_duplicate_future_and_untyped_parents_are_refused() {
        let (b, c, k) = fixture();
        for parents in [json!([999]), json!([2, 2]), json!([3]), json!([true])] {
            let mut changed = b.clone();
            changed[2]["parents"] = parents;
            assert!(project(&changed, &c, &k).is_err());
        }
        let mut changed = b.clone();
        changed[1]["gen"] = json!(2);
        assert!(project(&changed, &c, &k).is_err());
    }
    #[test]
    fn focal_identity_and_node_membership_cannot_be_silently_replaced() {
        let (b, c, mut k) = fixture();
        k["expected_node_ids"] = json!([1, 2, 3]);
        assert!(project(&b, &c, &k).is_err());
        let (_, _, mut k) = fixture();
        k["focal_id"] = json!(2);
        assert!(project(&b, &c, &k).is_err());
    }
}
