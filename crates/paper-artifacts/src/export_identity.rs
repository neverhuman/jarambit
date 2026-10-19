//! Original export-record topology and content associations; no CAD operations.
use serde_json::{json, Value};
use std::fmt::Write;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn text(v: &Value) -> Result<&str> {
    v.as_str().ok_or_else(|| "missing export identity".into())
}
fn count(v: &Value) -> Result<u64> {
    v.as_u64().ok_or_else(|| "untyped export count".into())
}
fn flag(v: &Value) -> Result<bool> {
    v.as_bool().ok_or_else(|| "untyped export flag".into())
}
fn label(s: &mut String, x: u32, y: u32, size: u32, body: &str) {
    let body = body
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    writeln!(
        s,
        "<text x=\"{x}\" y=\"{y}\" font-size=\"{size}\">{body}</text>"
    )
    .unwrap();
}
fn box_label(s: &mut String, x: u32, y: u32, title: &str, hash: &str) {
    writeln!(s,"<rect x=\"{x}\" y=\"{y}\" width=\"230\" height=\"64\" rx=\"4\" fill=\"#e4ebec\" stroke=\"#526873\"/>").unwrap();
    label(s, x + 12, y + 25, 20, title);
    label(
        s,
        x + 12,
        y + 49,
        18,
        &format!("SHA-256: {}...", &hash[..12]),
    );
}
fn hash(v: &Value) -> Result<&str> {
    let v = text(v)?;
    if v.len() != 64
        || !v
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    {
        return Err("invalid export digest".into());
    }
    Ok(v)
}

pub(crate) fn project(
    s: &Value,
    brep: &Value,
    scene: &Value,
    c: &Value,
) -> Result<(String, Value)> {
    if c["schema"] != "jarambit.export_identity_inputs.v1"
        || s["schema"] != "jarambit.master_cad_scorecard.v1"
        || scene["schema"] != "jarambit.publication_scene.v1"
        || s["subject"] != c["subject"]
        || scene["subject"] != c["subject"]
        || s["certificate_identity"] != c["certificate_identity"]
        || c["producer_source"]["revision"] != c["producer_revision"]
        || c["producer_source"]["path"] != "ops/ci/evaluate_cad_candidate.py"
        || c["producer_source"]["sha256"]
            != "e5478dba651bfaf327c44746b2388e786a8609916fecafdd51c836ecf4c81945"
        || c["producer_revision"] != "e8cb049df7d03c7d3113556812cc140ca99a29c3"
    {
        return Err("export subject/schema/producer binding differs".into());
    }
    let cad = &s["cad"];
    if s["genome_csv"] != c["sources"]["genome"]["path"]
        || s["genome_sha256"] != c["sources"]["genome"]["sha256"]
        || cad["brep_check"] != *brep
        || cad["units"] != "mm"
        || cad["units_source"] != "step_declaration"
        || s["engineering_metrics"]["promote_eligible"] != false
    {
        return Err("export record associations or qualification differ".into());
    }
    for kind in ["step", "stl"] {
        if cad[format!("{kind}_path")] != c["sources"][kind]["path"]
            || cad[format!("{kind}_sha256")] != c["sources"][kind]["sha256"]
        {
            return Err("export path/digest association differs".into());
        }
    }
    if scene["stl_sha256"] != cad["stl_sha256"]
        || scene["geometry_changed"] != false
        || scene["mesh_modifiers"] != json!([])
        || scene["imported_vertices_before_sha256"] != scene["imported_vertices_after_sha256"]
        || scene["faces_before_sha256"] != scene["faces_after_sha256"]
        || flag(&cad["cad_valid"])?
            != flag(&s["weighted_cad_score"]["gates"]["cad_watertight_and_single_solid"])?
    {
        return Err("render mesh or original CAD flag differs".into());
    }
    for key in [
        "imported_vertices_before_sha256",
        "imported_vertices_after_sha256",
        "faces_before_sha256",
        "faces_after_sha256",
    ] {
        hash(&scene[key])?;
    }
    let mut observations = Vec::new();
    for (source, pointer, kind, meaning) in [
        (
            "scorecard",
            "/cad/manifold_solids",
            "count",
            "Original raw STEP MANIFOLD_SOLID_BREP token count",
        ),
        (
            "scorecard",
            "/cad/closed_shells",
            "count",
            "Original raw STEP CLOSED_SHELL token count",
        ),
        (
            "scorecard",
            "/cad/facets",
            "count",
            "Original raw STEP ADVANCED_FACE token count; not STL triangles",
        ),
        (
            "scorecard",
            "/cad/boundary_edges",
            "count",
            "Original exporter stdout edge census",
        ),
        (
            "scorecard",
            "/cad/non_manifold_edges",
            "count",
            "Original exporter stdout edge census",
        ),
        (
            "scorecard",
            "/cad/max_directed_edge_repeat",
            "count",
            "Original exporter stdout directed-edge repeat",
        ),
        (
            "scorecard",
            "/cad/watertight_exact",
            "flag",
            "Original exporter-census flag, not a fresh mesh inspection",
        ),
        (
            "scorecard",
            "/cad/cad_valid",
            "flag",
            "Original composite CAD flag, not current qualification",
        ),
        (
            "step_check",
            "/solids",
            "count",
            "Original OpenCascade reader result",
        ),
        (
            "step_check",
            "/valid",
            "flag",
            "Original BRepCheck_Analyzer result",
        ),
        (
            "scene",
            "/vertices",
            "count",
            "Blender imported-mesh vertices; not a mechanics mesh",
        ),
        (
            "scene",
            "/triangles",
            "count",
            "Blender imported-mesh triangles; not STEP faces",
        ),
    ] {
        let doc = match source {
            "scorecard" => s,
            "step_check" => brep,
            _ => scene,
        };
        let value = doc.pointer(pointer).ok_or("missing export observation")?;
        if kind == "count" {
            count(value)?;
        } else {
            flag(value)?;
        }
        let path = if source == "scene" {
            &c["scene"]["path"]
        } else {
            &c["sources"][source]["path"]
        };
        observations.push(
            json!({"source":path,"pointer":pointer,"value":value,"kind":kind,"meaning":meaning}),
        );
    }
    // Reconcile the original reporting semantics without running a CAD check.
    let census_flag = count(&cad["boundary_edges"])? == 0
        && count(&cad["non_manifold_edges"])? == 0
        && count(&cad["max_directed_edge_repeat"])? == 1;
    if flag(&cad["watertight_exact"])? != census_flag
        || (flag(&cad["cad_valid"])?
            && (!census_flag
                || !flag(&brep["valid"])?
                || count(&brep["solids"])? != 1
                || count(&cad["manifold_solids"])? != 1
                || count(&cad["closed_shells"])? == 0))
    {
        return Err("original topology flags contradict their stored observations".into());
    }
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1200\" height=\"440\" viewBox=\"0 0 1200 440\"><defs><marker id=\"arrow\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" markerWidth=\"6\" markerHeight=\"6\" orient=\"auto\"><path d=\"M0,0 L10,5 L0,10 Z\" fill=\"#526873\"/></marker></defs><rect width=\"1200\" height=\"440\" fill=\"white\"/><g font-family=\"Arial, sans-serif\" fill=\"#132d3b\">\n");
    label(
        &mut svg,
        20,
        30,
        25,
        "RETAINED EXPORT IDENTITY AND TOPOLOGY RECORDS",
    );
    label(
        &mut svg,
        20,
        58,
        19,
        &format!(
            "{} / assessed example / not qualified",
            text(&c["subject"])?
        ),
    );
    for x in [20, 620] {
        writeln!(
            svg,
            "<rect x=\"{x}\" y=\"78\" width=\"560\" height=\"307\" fill=\"#f7f7f2\"/>"
        )
        .unwrap();
    }
    label(&mut svg, 35, 105, 22, "A / Saved content associations");
    box_label(
        &mut svg,
        42,
        120,
        "Genome CSV",
        hash(&c["sources"]["genome"]["sha256"])?,
    );
    box_label(
        &mut svg,
        310,
        120,
        "Assessment JSON",
        hash(&c["sources"]["scorecard"]["sha256"])?,
    );
    box_label(&mut svg, 42, 255, "STEP export", hash(&cad["step_sha256"])?);
    box_label(&mut svg, 310, 255, "STL export", hash(&cad["stl_sha256"])?);
    for d in [
        "M272,151 L310,151",
        "M425,184 L425,215 L157,215 L157,255",
        "M425,215 L425,255",
    ] {
        writeln!(svg,"<path d=\"{d}\" stroke=\"#526873\" stroke-width=\"2\" stroke-dasharray=\"5 4\" fill=\"none\" marker-end=\"url(#arrow)\"/>").unwrap();
    }
    label(
        &mut svg,
        42,
        344,
        18,
        &format!(
            "Blender import: {} vertices / {} triangles",
            count(&scene["vertices"])?,
            count(&scene["triangles"])?
        ),
    );
    label(
        &mut svg,
        42,
        372,
        17,
        "Dashed links: recorded associations, not fresh regeneration",
    );
    label(&mut svg, 635, 105, 22, "B / Original producer observations");
    let rows = [
        (
            "STEP text: solid / shell tokens",
            format!(
                "{} / {}",
                count(&cad["manifold_solids"])?,
                count(&cad["closed_shells"])?
            ),
        ),
        ("STEP text: face tokens", count(&cad["facets"])?.to_string()),
        (
            "Exporter: boundary / nonmanifold",
            format!(
                "{} / {}",
                count(&cad["boundary_edges"])?,
                count(&cad["non_manifold_edges"])?
            ),
        ),
        (
            "Exporter: directed-edge max repeat",
            count(&cad["max_directed_edge_repeat"])?.to_string(),
        ),
        (
            "BRep reader: solids / valid",
            format!("{} / {}", count(&brep["solids"])?, flag(&brep["valid"])?),
        ),
        (
            "Original census / composite flags",
            format!(
                "{} / {}",
                flag(&cad["watertight_exact"])?,
                flag(&cad["cad_valid"])?
            ),
        ),
    ];
    for (i, (name, value)) in rows.iter().enumerate() {
        let y = 143 + i as u32 * 35;
        label(&mut svg, 638, y, 19, name);
        label(&mut svg, 1035, y, 20, value);
    }
    label(
        &mut svg,
        638,
        363,
        17,
        "Reported units: mm; no new CAD validation performed",
    );
    label(
        &mut svg,
        20,
        410,
        18,
        "STEP token counts, exporter census and Blender mesh counts have different meanings.",
    );
    label(&mut svg,20,434,18,"Historical checks are preserved; no FEM field, manufacturing evidence or current qualification is inferred.");
    svg.push_str("</g></svg>\n");
    let receipt = json!({"schema":"jarambit.export_identity_projection.v1","subject":c["subject"],"certificate_identity":c["certificate_identity"],"producer_revision":c["producer_revision"],"producer_source":c["producer_source"],"sources":c["sources"],"scene":c["scene"],"observations":observations,"units":{"value":cad["units"],"provenance":cad["units_source"],"meaning":"original producer report; no new unit validation"},"semantics":c["semantics"],"qualification":"not qualified","fresh_cad_validation":false,"fresh_genome_regeneration":false,"fem_field":false,"digest_display":"first 12 hexadecimal characters; full identities in sources","associations":"recorded content links, not a fresh execution trace"});
    Ok((svg, receipt))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Value, Value, Value, Value) {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let read = |p: &str| {
            serde_json::from_slice::<Value>(&std::fs::read(root.join(p)).unwrap()).unwrap()
        };
        let c = read("docs/paper/export-inputs.v1.json");
        (
            read(c["sources"]["scorecard"]["path"].as_str().unwrap()),
            read(c["sources"]["step_check"]["path"].as_str().unwrap()),
            read(c["scene"]["path"].as_str().unwrap()),
            c,
        )
    }
    #[test]
    fn retains_different_representation_counts_and_original_flags() {
        let (s, b, scene, c) = fixture();
        let (svg, p) = project(&s, &b, &scene, &c).unwrap();
        let observations = p["observations"].as_array().unwrap();
        assert_eq!(observations.len(), 12);
        assert_eq!(
            observations
                .iter()
                .find(|r| r["pointer"] == "/cad/facets")
                .unwrap()["value"],
            55698
        );
        assert_eq!(
            observations
                .iter()
                .find(|r| r["pointer"] == "/triangles")
                .unwrap()["value"],
            90000
        );
        assert!(svg.contains("recorded associations, not fresh regeneration"));
        assert_eq!(p["fresh_cad_validation"], false);
        assert_eq!(p["fem_field"], false);
    }
    #[test]
    fn rejects_crossed_identity_or_fresh_render_modification() {
        let (s, b, scene, c) = fixture();
        for pointer in [
            "/subject",
            "/certificate_identity",
            "/cad/stl_sha256",
            "/genome_sha256",
        ] {
            let mut bad = s.clone();
            *bad.pointer_mut(pointer).unwrap() = json!("wrong");
            assert!(project(&bad, &b, &scene, &c).is_err());
        }
        let mut bad = scene.clone();
        bad["geometry_changed"] = json!(true);
        assert!(project(&s, &b, &bad, &c).is_err());
        let mut bad = b.clone();
        bad["valid"] = json!(false);
        assert!(project(&s, &bad, &scene, &c).is_err());
    }
    #[test]
    fn refuses_untyped_counts_and_preserves_unfavorable_flags() {
        let (s, b, scene, c) = fixture();
        for bad in [json!(null), json!("0"), json!(false), json!(-1), json!(0.5)] {
            let mut v = s.clone();
            v["cad"]["boundary_edges"] = bad;
            assert!(project(&v, &b, &scene, &c).is_err());
        }
        let mut unfavorable = s.clone();
        unfavorable["cad"]["cad_valid"] = json!(false);
        unfavorable["weighted_cad_score"]["gates"]["cad_watertight_and_single_solid"] =
            json!(false);
        let (_, p) = project(&unfavorable, &b, &scene, &c).unwrap();
        assert_eq!(
            p["observations"]
                .as_array()
                .unwrap()
                .iter()
                .find(|r| r["pointer"] == "/cad/cad_valid")
                .unwrap()["value"],
            false
        );
        assert_eq!(p["qualification"], "not qualified");
    }
    #[test]
    fn rejects_contradictory_census_without_promoting_an_unfavorable_record() {
        let (s, b, scene, c) = fixture();
        let mut inconsistent = s.clone();
        inconsistent["cad"]["boundary_edges"] = json!(1);
        assert!(project(&inconsistent, &b, &scene, &c).is_err());
        inconsistent["cad"]["watertight_exact"] = json!(false);
        assert!(project(&inconsistent, &b, &scene, &c).is_err());
        inconsistent["cad"]["cad_valid"] = json!(false);
        inconsistent["weighted_cad_score"]["gates"]["cad_watertight_and_single_solid"] =
            json!(false);
        let (_, p) = project(&inconsistent, &b, &scene, &c).unwrap();
        assert_eq!(p["qualification"], "not qualified");
        assert_eq!(p["fresh_cad_validation"], false);
    }
}
