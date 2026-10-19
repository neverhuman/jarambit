//! Catalog visual coverage, distinct from spatial-field availability or qualification.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const SUBJECT: &str = "cert_pipeline_r01_p14_c02";
const CERTIFICATE: &str = "db5d8e32e99d0b2d8ef6d7c7f9622a0601487c25b80879bbc8be1a502693c55e";

pub(crate) struct Visual {
    pub figure: &'static str,
    pub panel: &'static str,
}

pub(crate) fn assessed(kind: &str, id: &str) -> Option<Visual> {
    let (figure, panel) = match (kind, id) {
        ("G16", "cert_hiker_180n" | "cert_hunter_260n") => ("plate-fields", "B: lateral unit response; one shared field"),
        ("G16", "cert_ring_pull_1500n") => ("plate-fields", "C: ring unit response with seated-region source zeros masked"),
        ("G16", "twist_body_1p82nm" | "twist_tip_only_1p82nm") => ("plate-fields", "D: raw torsion response; regional capacity selection is a separate operation"),
        ("G16", "solid_tip_3d_p2_1p82nm") | ("RSV2", "solid_tip_sf") | ("battery", "solid.tip") => ("verification", "C--D: matched scalar comparison and binned profiles; no full 3D field"),
        ("G16", "failure_sequence_1p2") | ("RSV2", "seq_hard") | ("battery", "plate.seq") => ("verification", "A: retained yield-order proxy across mesh levels; no admitted GCI"),
        ("G16", "plate_gci_valid_3level") | ("battery", "plate.gci") => ("verification", "A--B: valid metadata, but nonpositive-order/oscillatory observations and no admitted GCI"),
        ("G16", "dolfinx_cross_check_2pct") => ("verification", "C: original matched-mesh safety-factor observations and stored rounded differences"),
        ("G16", "strict_hunter_260n" | "twist_tip_submodel_strict_1p82nm") | ("RSV2", "strict_hunter_lateral" | "twist_floor_body_and_tipsub_strict") => ("assessment", "C: original certification/strict scalar pairs; no strict full field or error estimate"),
        ("G16", "side_a4_buckling_and_support") | ("battery", "side.a4") => ("assessment", "A: support-model schematic and archived scalars; eigen-operator verification remains open"),
        ("RSV2", "fatigue") | ("battery", "plate.l7") => ("assessment", "B: original fatigue-model scalar; no fatigue field or measured lifetime"),
        ("RSV2", "human_max") => ("assessment", "B: separate recorded response factors; in-plane value remains record-only"),
        ("RSV2", "floors_828_joint") => ("plate-fields", "A--C: shared retained envelope and unit-response families; the original grouped flag is separate from these fields"),
        _ => return None,
    };
    Some(Visual { figure, panel })
}

fn gap(kind: &str, id: &str) -> Option<&'static str> {
    match (kind, id) {
        ("G16", "cad_watertight_and_single_solid") => Some("Subsection topology/identity visual not generated; the cover rendering does not establish topology validity."),
        ("G16", "geometric_and_holder_gates" | "karambit_ring_and_curved_claw" | "c6_aesthetic_floor") | ("RSV2", "aesthetic_and_geometric") => Some("Subsection geometry/reference visual not generated; original flags are retained separately, without an admitted visual comparison."),
        ("RSV2", "gate_s_sharp_law" | "gate_s_tip_point_brep") => Some("Subsection exported-measurement visual not generated; nominal geometry and measured export observations must remain distinct."),
        _ => None,
    }
}

fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn string(v: &Value) -> Result<&str> {
    v.as_str()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "missing visual identity".into())
}

fn bind(v: &Visual, out: &BTreeMap<String, String>) -> Result<Value> {
    let figure = format!("docs/paper/generated/{}.svg", v.figure);
    let receipt = format!("docs/paper/generated/{}.json", v.figure);
    let manifest = format!("{figure}.manifest.json");
    let raw = out.get(&figure).ok_or("missing mapped visual")?;
    let projected = out
        .get(&receipt)
        .ok_or("missing visual projection receipt")?;
    let manifest_raw = out.get(&manifest).ok_or("missing visual hash manifest")?;
    let m: Value = serde_json::from_str(manifest_raw)?;
    let p: Value = serde_json::from_str(projected)?;
    if m["figure"] != figure
        || m["figure_sha256"] != hash(raw.as_bytes())
        || m["projection_receipt"] != receipt
    {
        return Err("visual manifest/path/digest differs".into());
    }
    let (schema, category, scope) = match v.figure {
        "plate-fields" => ("jarambit.retained_plate_projection.v1", "retained_plate_cell_field", "Original 0.25 mm2 plate arrays only; not strict or solid fields; shared fields are not independent tests."),
        "verification" => ("jarambit.retained_verification_projection.v1", "retained_scalars_and_binned_profiles", "Saved mesh observations, matched scalar pairs and recovery-dependent profiles; no full solved solid field or admitted GCI estimate."),
        "assessment" => ("jarambit.retained_assessment_projection.v1", "retained_scalars_flags_and_support_schematic", "Saved scalars/flags and illustrative support boxes; no strict full field, eigenmode or fatigue field; A4 numerical audit open."),
        "battery-matrix" => ("jarambit.battery_matrix_projection.v1", "historical_categorical_status_map", "23 historical subjects; completeness and shared references are not independent physical tests or FEM fields."),
        _ => return Err("unadmitted visual type".into()),
    };
    if p["schema"] != schema {
        return Err("visual projection schema differs".into());
    }
    if (v.figure == "verification"
        && (p["full_3d_field_available"] != false || p["fresh_solver_execution"] != false))
        || (v.figure == "assessment" && p["fresh_solver_execution"] != false)
    {
        return Err("scalar/profile visual asserted unavailable field or fresh solve".into());
    }
    let binding = if v.figure == "battery-matrix" {
        if p["all_fresh_this_lane"] != false {
            return Err("historical status map asserted fresh execution".into());
        }
        json!({"cohort":"historical 23-subject battery", "relationship":"original corpus; separate from assessed-example panels"})
    } else {
        if p["subject"] != SUBJECT || p["certificate_identity"] != CERTIFICATE {
            return Err("visual subject/certificate binding differs".into());
        }
        json!({"subject":SUBJECT,"certificate_identity":CERTIFICATE,"qualification":"not qualified","relationship":"separate assessed example, not a rerun of the atlas historical example or battery"})
    };
    Ok(
        json!({"figure":figure,"figure_sha256":hash(raw.as_bytes()),"panel":v.panel,
        "projection_receipt":receipt,"projection_receipt_sha256":hash(projected.as_bytes()),
        "figure_manifest":manifest,"figure_manifest_sha256":hash(manifest_raw.as_bytes()),
        "visual_kind":category,"scope":scope,"binding":binding}),
    )
}

pub(crate) fn project(entries: &[Value], out: &BTreeMap<String, String>) -> Result<Value> {
    let atlas = out
        .get("docs/tests/README.md")
        .ok_or("missing visual atlas")?;
    let mut seen = BTreeSet::new();
    let mut counts = BTreeMap::<String, usize>::new();
    let mut rows = Vec::new();
    let mut linked = 0;
    let mut field_entries = 0;
    let mut missing = Vec::new();
    for e in entries {
        let kind = string(&e["kind"])?;
        let id = string(&e["id"])?;
        let anchor = format!("{}-{id}", kind.to_lowercase());
        if !seen.insert(anchor.clone()) || !atlas.contains(&format!("<a id=\"{anchor}\"></a>")) {
            return Err("duplicate catalog identity or missing atlas anchor".into());
        }
        let marker = format!("<a id=\"{anchor}\"></a>");
        let section = atlas.split_once(&marker).ok_or("missing visual section")?.1;
        let section = section
            .split("<a id=\"")
            .next()
            .ok_or("empty visual section")?;
        *counts.entry(kind.to_string()).or_default() += 1;
        let mut visuals = Vec::new();
        if kind == "battery" {
            let r = out
                .get("docs/paper/generated/battery-matrix.json")
                .ok_or("missing battery projection")?;
            let p: Value = serde_json::from_str(r)?;
            if !p["columns"]
                .as_array()
                .ok_or("missing matrix columns")?
                .contains(&json!(id))
            {
                return Err("visual column identity differs from evaluator".into());
            }
            visuals.push(bind(
                &Visual {
                    figure: "battery-matrix",
                    panel: "Exact evaluator-ID column, all 23 original stored labels",
                },
                out,
            )?);
        }
        if let Some(v) = assessed(kind, id) {
            field_entries += usize::from(v.figure == "plate-fields");
            visuals.push(bind(&v, out)?);
        }
        let missing_reason = if visuals.is_empty() {
            let why =
                gap(kind, id).ok_or("unmapped catalog identity without an explicit visual gap")?;
            if !section.contains(why) {
                return Err("visual gap not disclosed in atlas".into());
            }
            missing.push(anchor.clone());
            Some(why)
        } else {
            linked += 1;
            for v in &visuals {
                let figure = string(&v["figure"])?;
                let receipt = string(&v["projection_receipt"])?;
                // Match Markdown targets, rather than incidental prose or hashes.
                for path in [figure, receipt] {
                    let relative = path
                        .strip_prefix("docs/")
                        .ok_or("non-document visual target")?;
                    if !section.contains(&format!("](../{relative})")) {
                        return Err("mapped visual link missing from atlas".into());
                    }
                }
            }
            None
        };
        rows.push(
            json!({"kind":kind,"id":id,"atlas_anchor":anchor,"title":string(&e["title"])?,
            "coverage_status":if visuals.is_empty(){"subsection_visual_missing"}else{"linked"},
            "missing_reason":missing_reason,"visuals":visuals}),
        );
    }
    if counts
        != BTreeMap::from([
            ("G16".into(), 16),
            ("RSV2".into(), 10),
            ("battery".into(), 18),
        ])
    {
        return Err("visual coverage catalog membership differs".into());
    }
    Ok(json!({"schema":"jarambit.test_visual_coverage.v1",
        "scope":"Catalog-to-visual links, not full FEM-field coverage, qualification or main-text proximity",
        "catalog":"docs/tests/catalog.json","catalog_entries":entries.len(),"entries_with_visual":linked,
        "entries_with_retained_plate_field_link":field_entries,"entries_missing_subsection_visual":missing.len(),
        "missing_subsection_visuals":missing,"entries":rows,
        "spatial_requests":[
            {"id":"strict_plate_field","status":"archive_required","available_alternative":"original certification/strict scalars; no error estimate"},
            {"id":"solved_solid_field","status":"archive_required","available_alternative":"matched scalar pairs and binned profiles with distinct recovery semantics"},
            {"id":"buckling_mode","status":"archive_required","available_alternative":"archived scalar; eigen-operator verification remains open"},
            {"id":"fatigue_field","status":"archive_required","available_alternative":"saved fatigue-model scalar; no measured lifetime"}
        ],"completion":"incomplete; existing categorical or scalar images do not satisfy unavailable spatial requests"}))
}

pub(crate) fn missing_reason(kind: &str, id: &str) -> Option<&'static str> {
    gap(kind, id)
}

pub(crate) fn markdown(p: &Value) -> Result<String> {
    let mut s = format!("# Assessment visual coverage\n\nGenerated with the publication artifacts. [Machine-readable coverage and hashes](visuals.v1.json) · [All test explanations](README.md) · [Paper and build](../paper/README.md).\n\n**{} of {} catalog entries have subsection visual links; {} remain unpictured.** This denominator counts links, not unique figures, independent tests or completed FEM heatmaps. Six entries link to the same four-panel retained plate-field asset. Other links show saved scalars/profiles, a support schematic or the historical categorical battery. Main-text figure proximity is not checked by this ledger.\n\nThe separate assessed example is `cert_pipeline_r01_p14_c02`, not qualified. The battery map has its own historical 23-subject binding. The cover beauty render does not replace a missing geometry/measurement subsection visual.\n\n| Catalog explanation | Visual kinds and exact panels | Status |\n|---|---|---|\n", p["entries_with_visual"],p["catalog_entries"],p["entries_missing_subsection_visual"]);
    for e in p["entries"]
        .as_array()
        .ok_or("missing visual coverage entries")?
    {
        let mut links = Vec::new();
        for v in e["visuals"].as_array().ok_or("missing visual links")? {
            let figure = string(&v["figure"])?
                .strip_prefix("docs/")
                .ok_or("unsafe figure link")?;
            let receipt = string(&v["projection_receipt"])?
                .strip_prefix("docs/")
                .ok_or("unsafe receipt link")?;
            let label = match string(&v["visual_kind"])? {
                "retained_plate_cell_field" => "Actual retained plate field",
                "retained_scalars_and_binned_profiles" => "Saved scalars and binned profiles",
                "retained_scalars_flags_and_support_schematic" => {
                    "Saved scalars, flags and support schematic"
                }
                "historical_categorical_status_map" => "Historical categorical status map",
                _ => return Err("unknown visual-kind label".into()),
            };
            links.push(format!(
                "[{}: {}](../{figure}) ([receipt](../{receipt}))",
                label,
                string(&v["panel"])?
            ));
        }
        let details = if links.is_empty() {
            string(&e["missing_reason"])?.to_string()
        } else {
            links.join("; ")
        };
        s.push_str(&format!(
            "| [{}](README.md#{}) | {} | {} |\n",
            string(&e["title"])?,
            string(&e["atlas_anchor"])?,
            details,
            if links.is_empty() {
                "Missing"
            } else {
                "Linked"
            }
        ));
    }
    s.push_str("\n## Unavailable spatial evidence\n\nThese requests remain incomplete in the admitted publication inputs. Their scalar or schematic alternatives do not supply a spatial field.\n\n| Requested evidence | Availability | Existing narrower visual |\n|---|---|---|\n");
    for r in p["spatial_requests"]
        .as_array()
        .ok_or("missing spatial requests")?
    {
        s.push_str(&format!(
            "| `{}` | `{}` | {} |\n",
            string(&r["id"])?,
            string(&r["status"])?,
            string(&r["available_alternative"])?
        ));
    }
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn outputs() -> BTreeMap<String, String> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let mut out = BTreeMap::new();
        for figure in [
            "plate-fields",
            "verification",
            "assessment",
            "battery-matrix",
        ] {
            for suffix in ["svg", "json", "svg.manifest.json"] {
                let p = format!("docs/paper/generated/{figure}.{suffix}");
                out.insert(p.clone(), std::fs::read_to_string(root.join(p)).unwrap());
            }
        }
        out.insert(
            "docs/tests/README.md".into(),
            std::fs::read_to_string(root.join("docs/tests/README.md")).unwrap(),
        );
        out
    }
    fn entries() -> Vec<Value> {
        serde_json::from_str::<Value>(include_str!("../../../docs/tests/catalog.json")).unwrap()
            ["entries"]
            .as_array()
            .unwrap()
            .clone()
    }
    #[test]
    fn coverage_preserves_gaps_and_visual_kinds() {
        let p = project(&entries(), &outputs()).unwrap();
        assert_eq!(p["entries_with_visual"], 37);
        assert_eq!(p["entries_missing_subsection_visual"], 7);
        assert_eq!(p["entries_with_retained_plate_field_link"], 6);
        assert!(p["spatial_requests"]
            .as_array()
            .unwrap()
            .iter()
            .all(|r| r["status"] == "archive_required"));
        let rows = p["entries"].as_array().unwrap();
        let b = rows.iter().find(|e| e["id"] == "solid.tip").unwrap();
        assert_eq!(
            b["visuals"][0]["visual_kind"],
            "historical_categorical_status_map"
        );
        assert_eq!(
            b["visuals"][1]["visual_kind"],
            "retained_scalars_and_binned_profiles"
        );
        assert!(b["visuals"]
            .as_array()
            .unwrap()
            .iter()
            .all(|v| v["visual_kind"] != "retained_plate_cell_field"));
    }
    #[test]
    fn rejects_catalog_or_link_drift() {
        let e = entries();
        let out = outputs();
        let mut duplicate = e.clone();
        duplicate[1] = duplicate[0].clone();
        assert!(project(&duplicate, &out).is_err());
        let mut unknown = e.clone();
        unknown[0]["id"] = json!("unmapped_new_id");
        assert!(project(&unknown, &out).is_err());
        let mut missing = out.clone();
        missing.remove("docs/paper/generated/plate-fields.svg");
        assert!(project(&e, &missing).is_err());
        let mut unlinked = out.clone();
        let atlas = unlinked.get_mut("docs/tests/README.md").unwrap();
        *atlas = atlas.replacen(
            "](../paper/generated/plate-fields.svg)",
            "](missing.svg)",
            1,
        );
        assert!(project(&e, &unlinked).is_err());
    }
    #[test]
    fn rejects_wrong_identity_and_false_freshness() {
        for (figure, key, value) in [
            ("assessment", "subject", json!("different_subject")),
            (
                "verification",
                "certificate_identity",
                json!("wrong_certificate"),
            ),
            ("battery-matrix", "all_fresh_this_lane", json!(true)),
            ("verification", "full_3d_field_available", json!(true)),
            ("assessment", "fresh_solver_execution", json!(true)),
        ] {
            let mut out = outputs();
            let path = format!("docs/paper/generated/{figure}.json");
            let mut p: Value = serde_json::from_str(&out[&path]).unwrap();
            p[key] = value;
            out.insert(path, p.to_string());
            assert!(project(&entries(), &out).is_err());
        }
        let mut out = outputs();
        out.get_mut("docs/paper/generated/plate-fields.svg")
            .unwrap()
            .push(' ');
        assert!(project(&entries(), &out).is_err());
    }
}
