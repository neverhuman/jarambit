//! Display retained element arrays. No mechanics, meshing or CAD code is called.
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const FIELDS: [(&str, &str, &str, &str); 4] = [
    (
        "env_vm",
        "A / Peak multi-load von Mises envelope",
        "MPa",
        "Global envelope factor included",
    ),
    (
        "lat",
        "B / Out-of-plane plate bending response",
        "MPa/N",
        "Bending surface corner maximum",
    ),
    (
        "ring",
        "C / In-plane retention ring traction",
        "MPa/N",
        "Membrane centroid von Mises",
    ),
    (
        "twist",
        "D / Blade & tip torsional response",
        "MPa/(N mm)",
        "Torsional surface corner maximum",
    ),
];
const LUT: [[u8; 3]; 5] = [
    [68, 1, 84],
    [59, 82, 139],
    [33, 145, 140],
    [94, 201, 98],
    [253, 231, 37],
];
fn number(v: &Value) -> Result<f64> {
    v.as_f64()
        .filter(|n| n.is_finite())
        .ok_or_else(|| "non-finite or untyped plate value".into())
}
fn array(v: &Value) -> Result<&Vec<Value>> {
    v.as_array().ok_or_else(|| "missing plate array".into())
}
fn point(v: &Value) -> Result<[f64; 2]> {
    let a = array(v)?;
    if a.len() != 2 {
        return Err("plate coordinates must be pairs".into());
    }
    Ok([number(&a[0])?, number(&a[1])?])
}
fn distance(a: [f64; 2], b: [f64; 2]) -> f64 {
    (a[0] - b[0]).hypot(a[1] - b[1])
}
fn color(t: f64) -> String {
    let u = t.clamp(0., 1.) * 4.;
    let i = (u.floor() as usize).min(3);
    let f = u - i as f64;
    let c: [u8; 3] = std::array::from_fn(|k| {
        ((1. - f) * LUT[i][k] as f64 + f * LUT[i + 1][k] as f64).round() as u8
    });
    format!("#{:02x}{:02x}{:02x}", c[0], c[1], c[2])
}
fn text(s: &mut String, x: f64, y: f64, size: u32, body: &str) {
    let escaped = body
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    writeln!(
        s,
        "<text x=\"{x}\" y=\"{y}\" font-size=\"{size}\">{escaped}</text>"
    )
    .unwrap();
}
struct Mesh {
    xy: Vec<[f64; 2]>,
    tris: Vec<[usize; 3]>,
    values: BTreeMap<String, Vec<f64>>,
    tip: [f64; 2],
    tip_node: usize,
    envelope_mask: Vec<bool>,
    ring_mask: Vec<bool>,
    resolved_ring_boundary: Vec<usize>,
    min_tip_boundary_margin: f64,
}
/// Reuse the admitted producer masks for additive display-only summaries.
pub(crate) fn source_display_masks(
    doc: &Value,
    contract: &Value,
) -> Result<(Vec<bool>, Vec<bool>)> {
    let mesh = admit(doc, contract)?;
    Ok((mesh.envelope_mask, mesh.ring_mask))
}

fn admit(doc: &Value, contract: &Value) -> Result<Mesh> {
    if contract["schema"] != "jarambit.retained_plate_contract.v1"
        || contract["coordinate_units"] != "mm"
        || contract["serialized_coordinate_decimals"] != 3
        || contract["serialized_field_decimals"] != 4
        || contract["tip_exclusion_radius_mm"] != 4.0
    {
        return Err("unadmitted plate display semantics".into());
    }
    for key in ["source_label", "certificate_identity"] {
        let actual_key = if key == "source_label" { "label" } else { key };
        let wanted = contract[key].as_str().ok_or("missing plate identity")?;
        if doc[actual_key].as_str() != Some(wanted) {
            return Err("plate subject/assessment identity differs".into());
        }
    }
    let xy = array(&doc["mesh"]["xy"])?
        .iter()
        .map(point)
        .collect::<Result<Vec<_>>>()?;
    let mut tris = Vec::new();
    let mut edges = BTreeMap::new();
    let mut cells = BTreeSet::new();
    for row in array(&doc["mesh"]["tris"])? {
        let a = array(row)?;
        if a.len() != 3 {
            return Err("non-triangular plate cell".into());
        }
        let mut t = [0; 3];
        for k in 0..3 {
            t[k] = a[k]
                .as_u64()
                .and_then(|n| usize::try_from(n).ok())
                .ok_or("invalid plate node index")?;
            if t[k] >= xy.len() {
                return Err("plate cell index out of bounds".into());
            }
        }
        let mut canonical = t;
        canonical.sort();
        if canonical[0] == canonical[1] || canonical[1] == canonical[2] || !cells.insert(canonical)
        {
            return Err("duplicate or collapsed plate cell".into());
        }
        let [a, b, c] = t.map(|i| xy[i]);
        if ((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])).abs() == 0. {
            return Err("zero-area plate cell".into());
        }
        for (i, j) in [(t[0], t[1]), (t[1], t[2]), (t[2], t[0])] {
            let edge = if i < j { (i, j) } else { (j, i) };
            let n = edges.entry(edge).or_insert(0);
            *n += 1;
            if *n > 2 {
                return Err("non-manifold retained plate edge".into());
            }
        }
        tris.push(t);
    }
    if tris.is_empty() {
        return Err("empty plate mesh".into());
    }
    let mut values = BTreeMap::new();
    for (field, _, units, _) in FIELDS {
        if contract["display_fields"][field]["units"] != units {
            return Err("plate field units differ".into());
        }
        let a = array(&doc["mesh"][field])?
            .iter()
            .map(number)
            .collect::<Result<Vec<_>>>()?;
        if a.len() != tris.len() || a.iter().any(|n| *n < 0.) {
            return Err("plate field association or magnitude differs".into());
        }
        values.insert(field.to_string(), a);
    }
    let saved_tip = point(&doc["export_surface"]["tip"])?;
    let boundary: BTreeSet<_> = edges
        .iter()
        .filter(|(_, n)| **n == 1)
        .flat_map(|((i, j), _)| [*i, *j])
        .collect();
    let mut nearest: Vec<_> = boundary
        .iter()
        .map(|i| (*i, distance(xy[*i], saved_tip)))
        .collect();
    nearest.sort_by(|a, b| a.1.total_cmp(&b.1));
    let quantum = 0.0005_f64;
    if nearest.len() < 2 || nearest[1].1 - nearest[0].1 <= 2. * 2_f64.sqrt() * quantum {
        return Err("rounded coordinates do not identify the tip boundary node".into());
    }
    let tip_node = nearest[0].0;
    let tip = xy[tip_node];
    let header = doc["genome_header"]
        .as_str()
        .ok_or("missing retained genome header")?;
    let genome = doc["genome"]
        .as_str()
        .ok_or("missing retained genome row")?;
    let keys: Vec<_> = header.split(',').collect();
    let vals: Vec<_> = genome.split(',').collect();
    if keys.len() != vals.len() {
        return Err("plate genome membership differs".into());
    }
    let positions: Vec<_> = keys
        .iter()
        .enumerate()
        .filter(|(_, v)| **v == "grip_dx")
        .map(|(i, _)| i)
        .collect();
    if positions.len() != 1 {
        return Err("ambiguous retained grip offset".into());
    }
    let dx: f64 = vals[positions[0]].parse()?;
    if !dx.is_finite() {
        return Err("nonfinite retained grip offset".into());
    }
    let threshold = 76. + dx - 3.;
    let mut envelope_mask = Vec::new();
    let mut ring_mask = Vec::new();
    let mut resolved_ring_boundary = Vec::new();
    let mut min_tip_boundary_margin = f64::INFINITY;
    for (i, t) in tris.iter().enumerate() {
        let center = std::array::from_fn(|k| t.iter().map(|v| xy[*v][k]).sum::<f64>() / 3.);
        let radius = distance(center, tip);
        let margin = (radius - 4.).abs();
        min_tip_boundary_margin = min_tip_boundary_margin.min(margin);
        if margin <= 2. * 2_f64.sqrt() * quantum {
            return Err("tip mask is ambiguous after coordinate rounding".into());
        }
        let excluded = radius < 4.;
        if excluded && values["env_vm"][i] != 0. {
            return Err("source tip-zero mask disagrees with envelope".into());
        }
        envelope_mask.push(excluded);
        let seated = if (center[0] - threshold).abs() <= quantum {
            if values["ring"][i] > 0. {
                resolved_ring_boundary.push(i);
                false
            } else {
                return Err("ring mask is ambiguous after coordinate rounding".into());
            }
        } else {
            center[0] >= threshold
        };
        if seated && values["ring"][i] != 0. {
            return Err("source ring half-plane zero mask disagrees with ring field".into());
        }
        ring_mask.push(seated);
    }
    Ok(Mesh {
        xy,
        tris,
        values,
        tip,
        tip_node,
        envelope_mask,
        ring_mask,
        resolved_ring_boundary,
        min_tip_boundary_margin,
    })
}

pub(crate) fn polygon_area(poly: &[[f64; 2]]) -> f64 {
    if poly.len() < 3 {
        return 0.;
    }
    0.5 * poly
        .iter()
        .zip(poly.iter().cycle().skip(1))
        .take(poly.len())
        .map(|(a, b)| a[0] * b[1] - b[0] * a[1])
        .sum::<f64>()
        .abs()
}

pub(crate) fn clipped_polygon(mut poly: Vec<[f64; 2]>, bbox: [f64; 4]) -> Vec<[f64; 2]> {
    for (axis, bound, lower) in [
        (0, bbox[0], true),
        (0, bbox[2], false),
        (1, bbox[1], true),
        (1, bbox[3], false),
    ] {
        if poly.is_empty() {
            break;
        }
        let inside = |p: [f64; 2]| {
            if lower {
                p[axis] >= bound
            } else {
                p[axis] <= bound
            }
        };
        let mut output = Vec::new();
        let mut previous = *poly.last().unwrap();
        let mut was_inside = inside(previous);
        for current in poly {
            let is_inside = inside(current);
            if is_inside != was_inside {
                let fraction = (bound - previous[axis]) / (current[axis] - previous[axis]);
                output.push(std::array::from_fn(|k| {
                    previous[k] + fraction * (current[k] - previous[k])
                }));
            }
            if is_inside {
                output.push(current);
            }
            previous = current;
            was_inside = is_inside;
        }
        poly = output;
    }
    poly
}

/// Derived presentation checks run only after publication has verified the
/// original source bytes, producer files and their immutable input contracts.
/// The original field contract is not extended or rewritten by this ledger.
pub(crate) fn project_semantics(
    doc: &Value,
    contract: &Value,
    assessment: &Value,
    lineage: &Value,
    semantics: &Value,
    crop_svg: &str,
) -> Result<Value> {
    let mesh = admit(doc, contract)?;
    if semantics["schema"] != "jarambit.figure_semantics.v1"
        || semantics["source"] != contract["source"]
        || semantics["source_sha256"] != contract["source_sha256"]
        || semantics["certificate_identity"] != contract["certificate_identity"]
        || semantics["producer_revision"] != contract["producer_revision"]
        || semantics["producer_sources"] != contract["producer_sources"]
        || semantics["scalar_interpretation_sources"] != assessment["interpretation_sources"]
        || assessment["producer_revision"] != contract["producer_revision"]
    {
        return Err("plate semantics: original source/subject/producer binding differs".into());
    }
    let stages = &semantics["scalar_stages"];
    if stages["certification"]["keys"] != json!(["lat_rob_n", "twist_tipzone_sf"])
        || stages["certification"]["mesh_max_area_mm2"] != json!([0.25, 0.125])
        || stages["certification"]["aggregation"] != "minimum"
        || stages["strict"]["keys"] != json!(["strict_lat_rob_0p0625_n", "twist_tipzone_sf_strict"])
        || stages["strict"]["mesh_max_area_mm2"] != json!([0.0625])
        || stages["strict"]["aggregation"] != "single_mesh"
        || stages["separate_plate_gci_max_area_mm2"] != json!([4, 1, 0.25])
    {
        return Err("plate semantics: certification/strict/GCI source-key stages differ".into());
    }
    let pry = array(&doc["mesh"]["pry"])?
        .iter()
        .map(number)
        .collect::<Result<Vec<_>>>()?;
    if pry.len() != mesh.tris.len()
        || pry.iter().any(|v| *v < 0.)
        || semantics["pry_additive_admission"]["selector"] != "/mesh/pry"
        || semantics["pry_additive_admission"]["units"] != "MPa/N"
        || semantics["pry_additive_admission"]["count"].as_u64() != Some(pry.len() as u64)
    {
        return Err("plate semantics: additive pry association/units differ".into());
    }
    let ancestry = &semantics["ancestry_views"];
    if ancestry["generated_lineage"]["max_parent_hops"] != lineage["max_parent_hops"]
        || lineage["max_parent_hops"] != 2
        || ancestry["inheritance"]["max_parent_hops"] != 3
        || ancestry["inheritance"]["source_json_sha256"] != lineage["population"]["sha256"]
        || ancestry["inheritance"]["source_csv_sha256"].as_str()
            != Some(crate::population::HISTORICAL_ALL_EVALS_CSV_SHA256)
        || ancestry["inheritance"]["expected_node_ids"]
            != json!([174, 245, 299, 392, 413, 526, 1095])
        || ancestry["inheritance"]["separate_induced_fourth_hop_edge"] != json!([174, 299])
    {
        return Err("plate semantics: separate lineage/inheritance authority differs".into());
    }
    let mut contributor_counts: BTreeMap<String, u64> = BTreeMap::new();
    for i in 0..mesh.tris.len() {
        // This matches the existing hero display's active-map rule, rather than
        // assigning zero stress to producer-zeroed envelope/ring regions.
        let q = 3
            + usize::from(mesh.values["env_vm"][i] > 0.)
            + usize::from(mesh.values["ring"][i] > 0.);
        *contributor_counts.entry(q.to_string()).or_default() += 1;
    }
    if semantics["hero_composite"]["fields"] != json!(["env_vm", "lat", "pry", "ring", "twist"])
        || semantics["hero_composite"]["contributor_count_q"]
            != serde_json::to_value(&contributor_counts)?
        || semantics["hero_composite"]["visibility"]
            != "The cover does not display a separate per-triangle contributor-count map."
    {
        return Err("plate semantics: composite support/visibility differs".into());
    }
    let panels = array(&semantics["crop_panels"])?;
    if panels.len() != 3 {
        return Err("plate semantics: crop panel membership differs".into());
    }
    let theta = -6.0_f64.to_radians();
    let (ct, st) = (theta.cos(), theta.sin());
    let posed: Vec<[f64; 2]> = mesh
        .xy
        .iter()
        .map(|p| [p[0] * ct + p[1] * st, p[0] * st - p[1] * ct])
        .collect();
    let mut receipts = Vec::new();
    for (index, (id, field, units)) in [
        ("A", "ring", "MPa/N"),
        ("B", "twist", "MPa/(N mm)"),
        ("C", "pry", "MPa/N"),
    ]
    .into_iter()
    .enumerate()
    {
        let panel = &panels[index];
        if panel["panel"] != id
            || panel["selector"] != format!("/mesh/{field}")
            || panel["units"] != units
            || panel["source_path"] != contract["source"]
            || panel["source_sha256"] != contract["source_sha256"]
            || panel["denominator_scope"]
                != "whole retained source-active field domain, not a preselected terminal ROI"
        {
            return Err("plate semantics: wrong crop field/units/source/denominator".into());
        }
        let bounds = array(&panel["crop_domain_posed_mm"])?;
        let rect = array(&panel["svg_clip_rect"])?;
        if bounds.len() != 4 || rect.len() != 4 {
            return Err("plate semantics: malformed crop rectangle".into());
        }
        let bbox = [
            number(&bounds[0])?,
            number(&bounds[1])?,
            number(&bounds[2])?,
            number(&bounds[3])?,
        ];
        let aperture = [
            number(&rect[0])?,
            number(&rect[1])?,
            number(&rect[2])?,
            number(&rect[3])?,
        ];
        let transform = &panel["svg_transform"];
        let scale = number(&transform["scale"])?;
        let ox = number(&transform["origin_x"])?;
        let oy = number(&transform["origin_y"])?;
        if bbox[0] >= bbox[2]
            || bbox[1] >= bbox[3]
            || scale <= 0.
            || aperture[2] <= 0.
            || aperture[3] <= 0.
        {
            return Err("plate semantics: invalid crop domain".into());
        }
        let inverse = [
            (aperture[0] - ox) / scale,
            (oy - aperture[1] - aperture[3]) / scale,
            (aperture[0] + aperture[2] - ox) / scale,
            (oy - aperture[1]) / scale,
        ];
        let clip_element = panel["svg_clip_rect_element"]
            .as_str()
            .ok_or("plate semantics: missing SVG clip binding")?;
        if inverse.iter().zip(bbox).any(|(a, b)| (*a - b).abs() > 1e-9)
            || crop_svg.matches(clip_element).count() != 1
        {
            return Err(
                "plate semantics: actual SVG aperture differs from physical crop window".into(),
            );
        }
        let values = if field == "pry" {
            &pry
        } else {
            &mesh.values[field]
        };
        let mut denominator = 0.;
        let mut numerator = 0.;
        let mut active = 0u64;
        for (i, triangle) in mesh.tris.iter().enumerate() {
            if field == "ring" && mesh.ring_mask[i] {
                continue;
            }
            active += 1;
            let poly: Vec<[f64; 2]> = triangle.iter().map(|j| posed[*j]).collect();
            denominator += polygon_area(&poly) * values[i].abs();
            numerator += polygon_area(&clipped_polygon(poly, bbox)) * values[i].abs();
        }
        // A zero integral has no defined coverage fraction; never manufacture100%.
        let fraction = (denominator > 0.).then(|| numerator / denominator);
        if active
            != panel["source_active_denominator_cells"]
                .as_u64()
                .ok_or("plate semantics: missing active denominator")?
            || (number(&panel["whole_domain_denominator"])? - denominator).abs() > 1e-7
            || (number(&panel["crop_numerator"])? - numerator).abs() > 1e-7
            || match fraction {
                Some(value) => {
                    (number(&panel["achieved_whole_domain_fraction"])? - value).abs() > 1e-10
                }
                None => !panel["achieved_whole_domain_fraction"].is_null(),
            }
        {
            return Err("plate semantics: source-domain crop denominator/fraction differs".into());
        }
        receipts.push(json!({"panel":id, "selector":panel["selector"], "units":units,
            "crop_domain_posed_mm":bbox, "active_denominator_cells":active,
            "whole_domain_denominator":denominator, "crop_numerator":numerator,
            "achieved_whole_domain_fraction":fraction,
            "coverage_interpretation":"rectangular physical source-domain integral before decorative PIP; not visible ink"}));
    }
    if !crop_svg.contains(">C. In-plane tip pry (MPa/N)</text>")
        || !crop_svg.contains(">≥0.6611</text>")
        || !crop_svg.contains(">(over-range)</text>")
        || !crop_svg.contains(">raw max 34.8589</text>")
        || crop_svg.contains("23-subject automatic CAD battery")
        || crop_svg.contains("&gt;= 90%")
    {
        return Err("plate semantics: crop observable/scope/over-range label differs".into());
    }
    Ok(
        json!({"schema":"jarambit.crop_semantics_projection.v1", "source":contract["source"],
        "source_sha256":contract["source_sha256"], "producer_revision":contract["producer_revision"],
        "panels":receipts, "contributor_count_q":contributor_counts,
        "contributor_count_q_context":"Historical five-map figure-semantics /hero_composite only; active four-response display and support are bound separately in generated/display-composite.v1.json",
        "lineage_contract_preserved":true, "new_solver_execution":false,
        "scope":"Reporting/source-domain projection; original values/masks unchanged; PNG/PDF visual acceptance separate."}),
    )
}

pub(crate) fn project(doc: &Value, contract: &Value) -> Result<(String, Value)> {
    let m = admit(doc, contract)?;
    let xmin = m.xy.iter().map(|p| p[0]).fold(f64::INFINITY, f64::min);
    let xmax = m.xy.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max);
    let ymin = m.xy.iter().map(|p| p[1]).fold(f64::INFINITY, f64::min);
    let ymax = m.xy.iter().map(|p| p[1]).fold(f64::NEG_INFINITY, f64::max);
    let scale = (500. / (xmax - xmin)).min(185. / (ymax - ymin));
    let _subject_internal = contract["subject"].as_str().unwrap();
    let mut s = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1200\" height=\"960\" viewBox=\"0 0 1200 960\"><rect width=\"1200\" height=\"960\" fill=\"white\"/><g font-family=\"Arial, sans-serif\" fill=\"#132d3b\">\n");
    text(
        &mut s,
        20.,
        34.,
        25,
        "Individual 2000 — Retained Plate-Element Response Fields (Not Qualified)",
    );
    text(
        &mut s,
        20.,
        66.,
        21,
        &format!(
            "Assessed candidate · {} nodes · {} triangles · independent full-range scales",
            m.xy.len(),
            m.tris.len()
        ),
    );
    let mut field_summary = serde_json::Map::new();
    for (panel, (field, title, units, description)) in FIELDS.into_iter().enumerate() {
        let x = 20. + 600. * (panel % 2) as f64;
        let y = 86. + 405. * (panel / 2) as f64;
        writeln!(
            s,
            "<rect x=\"{x}\" y=\"{y}\" width=\"560\" height=\"385\" fill=\"#f7f7f2\" stroke=\"#d5dfdf\"/>"
        )
        .unwrap();
        text(&mut s, x + 15., y + 32., 23, title);
        text(
            &mut s,
            x + 15.,
            y + 62.,
            21,
            &format!("{units} · {description}"),
        );
        let origin_x = x + 30. + (500. - (xmax - xmin) * scale) / 2. - xmin * scale;
        let origin_y = y + 78. + (185. - (ymax - ymin) * scale) / 2. + ymax * scale;
        let mask: Vec<_> = (0..m.tris.len())
            .map(|i| match field {
                "env_vm" => m.envelope_mask[i],
                "ring" => m.ring_mask[i],
                _ => false,
            })
            .collect();
        let max = m.values[field].iter().copied().fold(0., f64::max);
        writeln!(s, "<g id=\"field-{field}\" shape-rendering=\"crispEdges\">").unwrap();
        for (i, t) in m.tris.iter().enumerate() {
            let value = m.values[field][i];
            let fill = if mask[i] {
                "#b8bfc3".to_string()
            } else {
                color(if max > 0. { value / max } else { 0. })
            };
            let points = t
                .iter()
                .map(|v| {
                    format!(
                        "{:.3},{:.3}",
                        origin_x + m.xy[*v][0] * scale,
                        origin_y - m.xy[*v][1] * scale
                    )
                })
                .collect::<Vec<_>>()
                .join(" ");
            writeln!(
                s,
                "<polygon id=\"{field}-{i}\" points=\"{points}\" fill=\"{fill}\"/>"
            )
            .unwrap();
        }
        s.push_str("</g>\n");
        if field == "lat" || field == "twist" {
            writeln!(s,"<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"none\" stroke=\"#a3442d\" stroke-width=\"1.6\" stroke-dasharray=\"4 3\"/>",origin_x+m.tip[0]*scale,origin_y-m.tip[1]*scale,4.*scale).unwrap();
        }
        writeln!(
            s,
            "<defs><linearGradient id=\"lut-{field}\" color-interpolation=\"sRGB\">"
        )
        .unwrap();
        for (j, c) in LUT.iter().enumerate() {
            writeln!(
                s,
                "<stop offset=\"{}%\" stop-color=\"#{:02x}{:02x}{:02x}\"/>",
                j * 25,
                c[0],
                c[1],
                c[2]
            )
            .unwrap();
        }
        s.push_str("</linearGradient></defs>\n");
        let fill = if max > 0. {
            format!("url(#lut-{field})")
        } else {
            color(0.)
        };
        writeln!(s,"<rect id=\"legend-{field}\" x=\"{}\" y=\"{}\" width=\"500\" height=\"16\" fill=\"{fill}\"/>",x+30.,y+282.).unwrap();
        for (j, t) in [0., 0.5, 1.].into_iter().enumerate() {
            let anchor = ["start", "middle", "end"][j];
            writeln!(
                s,
                "<text x=\"{}\" y=\"{}\" font-size=\"21\" text-anchor=\"{anchor}\">{:.4}</text>",
                x + 30. + t * 500.,
                y + 325.,
                t * max
            )
            .unwrap();
        }
        let masked: Vec<_> = mask
            .iter()
            .enumerate()
            .filter_map(|(i, b)| b.then_some(i))
            .collect();
        let note = if masked.is_empty() {
            "Dashed circle: capacity excludes 4 mm tip zone".to_string()
        } else {
            format!("Gray: {} producer-masked elements", masked.len())
        };
        text(&mut s, x + 15., y + 364., 21, &note);
        field_summary.insert(field.into(),json!({"selector":format!("/mesh/{field}"),"units":units,"count":m.tris.len(),"association":"triangle","limits":[0.,max],"clipped_count":0,"masked_cell_ids":masked,"raw_zero_count":m.values[field].iter().filter(|v|**v==0.).count(),"plot_transform":{"scale":scale,"origin_x":origin_x,"origin_y":origin_y,"formula":"svg_x=origin_x+source_x*scale; svg_y=origin_y-source_y*scale"}}));
    }
    text(
        &mut s,
        20.,
        912.,
        21,
        "Element colors use saved values without smoothing or clipping; gray denotes producer-masked regions.",
    );
    text(
        &mut s,
        20.,
        944.,
        21,
        "Fields come from the 0.25 mm2 plate mesh of Individual 2000 (not the refined mesh or 3D solid).",
    );
    s.push_str("</g></svg>\n");
    let summary = json!({"schema":"jarambit.retained_plate_projection.v1","subject":contract["subject"],"certificate_identity":contract["certificate_identity"],"source":contract["source"],"producer_revision":contract["producer_revision"],"nodes":m.xy.len(),"triangles":m.tris.len(),"palette":{"name":"piecewise-linear viridis anchors","space":"sRGB","knots":LUT,"positions":[0.,0.25,0.5,0.75,1.]},"tip_node":m.tip_node,"tip_coordinate":m.tip,"minimum_tip_mask_threshold_margin_mm":m.min_tip_boundary_margin,"tip_mask_coordinate_error_bound_mm":2.*2_f64.sqrt()*0.0005,"ring_boundary_membership_resolved_by_positive_source_value":m.resolved_ring_boundary,"fields":field_summary,"note":"Lateral and torsion show all raw cells, including the original capacity-excluded tip region. Envelope and ring producer-forced zeros are masked, not colored as zero stress."});
    Ok((s, summary))
}


/// A reserved, non-overlaid whole-field locator; all boundary loops remain visible.
fn smart_locator(s:&mut String,m:&Mesh,crop:&crate::saliency::CropReceipt,x:f64,y:f64) -> Option<Value> {
    writeln!(s,"<rect x=\"{}\" y=\"{}\" width=\"170\" height=\"155\" fill=\"white\"/>",x+380.,y+100.).unwrap();
    text(s,x+388.,y+124.,21,"Locator");
    let mut edges:BTreeMap<(usize,usize),usize>=BTreeMap::new();
    for t in &m.tris {
        for (a,b) in [(t[0],t[1]),(t[1],t[2]),(t[2],t[0])] {
            *edges.entry((a.min(b),a.max(b))).or_default()+=1;
        }
    }
    let mut path=String::new();
    for ((a,b),count) in edges {
        if count!=1 { continue; }
        let p=crop.source_to_locator.point(m.xy[a]);
        let q=crop.source_to_locator.point(m.xy[b]);
        write!(path,"M{:.6},{:.6}L{:.6},{:.6}",p[0],p[1],q[0],q[1]).unwrap();
    }
    writeln!(s,"<path d=\"{path}\" fill=\"none\" stroke=\"#000000\" stroke-width=\"1.4\"/>").unwrap();
    let points=crop.locator_crop_quad.iter()
        .map(|p|format!("{:.6},{:.6}",p[0],p[1])).collect::<Vec<_>>().join(" ");
    writeln!(s,"<polygon points=\"{points}\" fill=\"none\" stroke=\"#c5292b\" stroke-width=\"1.5\"/>").unwrap();
    if crop.coverage.area_fraction.is_some_and(|area| area < 0.01) {
        let center = [
            crop.locator_crop_quad.iter().map(|p| p[0]).sum::<f64>() / 4.,
            crop.locator_crop_quad.iter().map(|p| p[1]).sum::<f64>() / 4.,
        ];
        let leader = [[x+496.,y+237.], [x+514.,y+224.], center];
        writeln!(s,"<polyline points=\"{:.6},{:.6} {:.6},{:.6} {:.6},{:.6}\" fill=\"none\" stroke=\"#c5292b\" stroke-width=\"1.5\"/>",
            leader[0][0],leader[0][1],leader[1][0],leader[1][1],center[0],center[1]).unwrap();
        writeln!(s,"<text x=\"{}\" y=\"{}\" font-size=\"21\" fill=\"#c5292b\">Exact ROI</text>",x+388.,y+245.).unwrap();
        Some(json!({"label":"Exact ROI","leader_svg_points":leader,
            "label_svg_baseline":[x+388.,y+245.],"roi_enlarged":false,
            "target":"center of the unchanged exact source-coordinate crop quadrilateral"}))
    } else {
        None
    }
}

/// Additive smart view. The original project() remains the full-context companion.
pub(crate) fn project_smart(doc: &Value, contract: &Value, policy: &crate::saliency::CropPolicy) -> Result<(String, Value)> {
    let m = admit(doc, contract)?;
    let missing = vec![None; m.tris.len()];
    let _subject_internal = contract["subject"].as_str().unwrap();
    let mut s = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1200\" height=\"960\" viewBox=\"0 0 1200 960\"><rect width=\"1200\" height=\"960\" fill=\"white\"/><g font-family=\"Arial, sans-serif\" fill=\"#132d3b\">\n");
    text(
        &mut s,
        20.,
        34.,
        25,
        "Individual 2000 — Retained Plate-Element Response Fields",
    );
    text(
        &mut s,
        20.,
        66.,
        21,
        &format!(
            "Assessed candidate · {} nodes · {} triangles · independent full-range scales",
            m.xy.len(),
            m.tris.len()
        ),
    );
    let mut field_summary = serde_json::Map::new();
    for (panel, (field, title, units, description)) in FIELDS.into_iter().enumerate() {
        let x = 20. + 600. * (panel % 2) as f64;
        let y = 86. + 405. * (panel / 2) as f64;
        writeln!(
            s,
            "<rect x=\"{x}\" y=\"{y}\" width=\"560\" height=\"385\" fill=\"#f7f7f2\" stroke=\"#d5dfdf\"/>"
        )
        .unwrap();
        text(&mut s, x + 15., y + 32., 23, title);
        text(
            &mut s,
            x + 15.,
            y + 62.,
            21,
            &format!("{units} · {description}"),
        );
        let mask: Vec<_> = (0..m.tris.len())
            .map(|i| match field {
                "env_vm" => m.envelope_mask[i],
                "ring" => m.ring_mask[i],
                _ => false,
            })
            .collect();
        let max = m.values[field].iter().copied().fold(0., f64::max);
        let full_view = crate::saliency::Viewport { x:x+30., y:y+78., width:500., height:185. };
        let zoom_view = crate::saliency::Viewport { x:x+30., y:y+78., width:345., height:185. };
        let locator_view = crate::saliency::Viewport { x:x+390., y:y+128., width:140., height:100. };
        let bindings = crate::saliency::SourceBindings {
            source_path: contract["source"].as_str().ok_or("missing crop source")?.into(),
            source_sha256: contract["source_sha256"].as_str().ok_or("missing crop source hash")?.into(),
            certificate_identity: contract["certificate_identity"].as_str().ok_or("missing crop identity")?.into(),
            field_selector: format!("/mesh/{field}"), units:units.into(),
        };
        let field_view = crate::saliency::FieldView { xy:&m.xy, triangles:&m.tris,
            values:&m.values[field], masked:&mask, missing_reasons:&missing };
        let mut crop = crate::saliency::select(&field_view,&bindings,policy,full_view,zoom_view,locator_view)
            .map_err(|e| format!("plate crop: {e}"))?;
        let scale = crop.source_to_panel.xx;
        let origin_x = crop.source_to_panel.tx;
        let origin_y = crop.source_to_panel.ty;
        // Clip the transformed physical bbox, not its possibly letterboxed viewport.
        // Otherwise extra source cells in the letterbox would falsify the locator.
        let b = crop.bbox_source_mm;
        let aperture = [origin_x+b[0]*scale,origin_y-b[3]*scale,
            (b[2]-b[0])*scale,(b[3]-b[1])*scale];
        crop.record["svg_clip_rect"] = json!(aperture);
        writeln!(s,"<defs><clipPath id=\"smart-clip-{field}\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/></clipPath></defs>",aperture[0],aperture[1],aperture[2],aperture[3]).unwrap();
        writeln!(s, "<g id=\"field-{field}\" shape-rendering=\"crispEdges\" clip-path=\"url(#smart-clip-{field})\">").unwrap();
        for (i, t) in m.tris.iter().enumerate() {
            let value = m.values[field][i];
            let fill = if mask[i] {
                "#b8bfc3".to_string()
            } else {
                color(if max > 0. { value / max } else { 0. })
            };
            let points = t
                .iter()
                .map(|v| {
                    format!(
                        "{:.3},{:.3}",
                        origin_x + m.xy[*v][0] * scale,
                        origin_y - m.xy[*v][1] * scale
                    )
                })
                .collect::<Vec<_>>()
                .join(" ");
            writeln!(
                s,
                "<polygon id=\"{field}-{i}\" points=\"{points}\" fill=\"{fill}\"/>"
            )
            .unwrap();
        }
        s.push_str("</g>\n");
        if field == "lat" || field == "twist" {
            writeln!(s,"<g clip-path=\"url(#smart-clip-{field})\">").unwrap();
            writeln!(s,"<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"none\" stroke=\"#a3442d\" stroke-width=\"1.6\" stroke-dasharray=\"4 3\"/>",origin_x+m.tip[0]*scale,origin_y-m.tip[1]*scale,4.*scale).unwrap();
            s.push_str("</g>\n");
        }
        if crop.decision == crate::saliency::CropDecision::Zoom {
            if let Some(callout) = smart_locator(&mut s, &m, &crop, x, y) {
                crop.record["locator_callout"] = callout;
            }
        }
        crop.record["locator_outline_source"] = json!("original plate boundary edges; not a detailed CAD silhouette");
        crop.record["locator_svg_coordinate_decimals"] = json!(6);
        writeln!(
            s,
            "<defs><linearGradient id=\"lut-{field}\" color-interpolation=\"sRGB\">"
        )
        .unwrap();
        for (j, c) in LUT.iter().enumerate() {
            writeln!(
                s,
                "<stop offset=\"{}%\" stop-color=\"#{:02x}{:02x}{:02x}\"/>",
                j * 25,
                c[0],
                c[1],
                c[2]
            )
            .unwrap();
        }
        s.push_str("</linearGradient></defs>\n");
        let fill = if max > 0. {
            format!("url(#lut-{field})")
        } else {
            color(0.)
        };
        writeln!(s,"<rect id=\"legend-{field}\" x=\"{}\" y=\"{}\" width=\"500\" height=\"16\" fill=\"{fill}\"/>",x+30.,y+282.).unwrap();
        for (j, t) in [0., 0.5, 1.].into_iter().enumerate() {
            let anchor = ["start", "middle", "end"][j];
            writeln!(
                s,
                "<text x=\"{}\" y=\"{}\" font-size=\"21\" text-anchor=\"{anchor}\">{:.4}</text>",
                x + 30. + t * 500.,
                y + 325.,
                t * max
            )
            .unwrap();
        }
        let masked: Vec<_> = mask
            .iter()
            .enumerate()
            .filter_map(|(i, b)| b.then_some(i))
            .collect();
        let note = if crop.decision == crate::saliency::CropDecision::Zoom {
            format!("Zoom t >= {:.4}; excess {:.1}%; area {:.1}%",
                crop.chosen_value_threshold.unwrap(),
                100.*crop.coverage.excess_fraction.ok_or("missing crop excess denominator")?,
                100.*crop.coverage.area_fraction.ok_or("missing crop area denominator")?)
        } else if !masked.is_empty() {
            format!("Full context; {} source-masked cells", masked.len())
        } else {
            "Full context; original field and raw scale".into()
        };
        text(&mut s, x + 15., y + 364., 21, &note);
        field_summary.insert(field.into(),json!({"selector":format!("/mesh/{field}"),"units":units,"count":m.tris.len(),"association":"triangle","limits":[0.,max],"clipped_count":0,"masked_cell_ids":masked,"raw_zero_count":m.values[field].iter().filter(|v|**v==0.).count(),"plot_transform":{"scale":scale,"origin_x":origin_x,"origin_y":origin_y,"formula":"svg_x=origin_x+source_x*scale; svg_y=origin_y-source_y*scale"}}));
        field_summary.get_mut(field).unwrap()["clipped_count_scope"] = json!("color values only; camera coverage is separate");
        field_summary.get_mut(field).unwrap()["smart_crop"] = crop.record;
    }
    text(
        &mut s,
        20.,
        912.,
        21,
        "Camera zoom clips geometry only; colors keep full raw scales and original source masks.",
    );
    text(
        &mut s,
        20.,
        944.,
        21,
        "Fields come from the 0.25 mm2 plate mesh of Individual 2000 (not the refined mesh or 3D solid).",
    );
    s.push_str("</g></svg>\n");
    let summary = json!({"schema":"jarambit.retained_plate_smart_projection.v1","subject":contract["subject"],"certificate_identity":contract["certificate_identity"],"source":contract["source"],"producer_revision":contract["producer_revision"],"nodes":m.xy.len(),"triangles":m.tris.len(),"palette":{"name":"piecewise-linear viridis anchors","space":"sRGB","knots":LUT,"positions":[0.,0.25,0.5,0.75,1.]},"tip_node":m.tip_node,"tip_coordinate":m.tip,"minimum_tip_mask_threshold_margin_mm":m.min_tip_boundary_margin,"tip_mask_coordinate_error_bound_mm":2.*2_f64.sqrt()*0.0005,"ring_boundary_membership_resolved_by_positive_source_value":m.resolved_ring_boundary,"fields":field_summary,"note":"Lateral and torsion show all raw cells, including the original capacity-excluded tip region. Envelope and ring producer-forced zeros are masked, not colored as zero stress."});
    Ok((s, summary))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Value, Value) {
        let doc = json!({"label":"neutral-reporting-fixture","certificate_identity":"fixture-only","genome_header":"grip_dx","genome":"0","export_surface":{"tip":[0.,0.]},"mesh":{"xy":[[0.,0.],[1.,0.],[0.,1.],[10.,0.],[11.,0.],[10.,1.]],"tris":[[0,1,2],[3,4,5]],"env_vm":[0.,2.],"lat":[4.,5.],"ring":[0.2,0.3],"twist":[1.,2.]}});
        let mut contract = json!({"schema":"jarambit.retained_plate_contract.v1","subject":"neutral-reporting-fixture","source_label":"neutral-reporting-fixture","certificate_identity":"fixture-only","coordinate_units":"mm","serialized_coordinate_decimals":3,"serialized_field_decimals":4,"tip_exclusion_radius_mm":4.,"display_fields":{}});
        for (field, _, units, _) in FIELDS {
            contract["display_fields"][field] = json!({"units":units});
        }
        (doc, contract)
    }
    #[test]
    fn rectangle_clipping_has_an_independent_area_oracle_and_handles_empty_domains() {
        let square = vec![[0., 0.], [2., 0.], [2., 2.], [0., 2.]];
        assert!(
            (polygon_area(&clipped_polygon(square.clone(), [0.5, 0.5, 1.5, 1.5])) - 1.).abs()
                < 1e-12
        );
        assert_eq!(polygon_area(&clipped_polygon(square, [3., 3., 4., 4.])), 0.);
        assert_eq!(polygon_area(&[]), 0.);
    }

    #[test]
    fn semantic_ledger_rejects_wrong_units_subject_stage_and_crop_denominator() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let read = |path: &str| -> Value {
            serde_json::from_slice(&std::fs::read(root.join(path)).unwrap()).unwrap()
        };
        let contract = read("docs/paper/field-inputs.v1.json");
        let doc = read(contract["source"].as_str().unwrap());
        let assessment = read("docs/paper/assessment-inputs.v1.json");
        let lineage = read("docs/paper/lineage-inputs.v1.json");
        let ledger = read("docs/paper/figure-semantics.v1.json");
        let svg = std::fs::read_to_string(
            root.join("docs/paper/drafts_p3_p5/JR_EI_P4_T04_fem_saliency_crops_and_locators.svg"),
        )
        .unwrap();
        assert!(project_semantics(&doc, &contract, &assessment, &lineage, &ledger, &svg).is_ok());
        for (pointer, value) in [
            ("/crop_panels/2/units", json!("MPa")),
            ("/certificate_identity", json!("another candidate")),
            ("/scalar_stages/strict/mesh_max_area_mm2", json!([0.125])),
            ("/crop_panels/1/whole_domain_denominator", json!(73.5323)),
            ("/crop_panels/1/achieved_whole_domain_fraction", json!(0.9)),
            (
                "/ancestry_views/generated_lineage/max_parent_hops",
                json!(3),
            ),
        ] {
            let mut changed = ledger.clone();
            *changed.pointer_mut(pointer).unwrap() = value;
            assert!(
                project_semantics(&doc, &contract, &assessment, &lineage, &changed, &svg).is_err(),
                "{pointer}"
            );
        }
    }

    #[test]
    fn source_mask_is_not_colored_as_zero_and_raw_fields_remain_distinct() {
        let (doc, contract) = fixture();
        let (svg, summary) = project(&doc, &contract).unwrap();
        assert_eq!(summary["fields"]["env_vm"]["masked_cell_ids"], json!([0]));
        assert_eq!(summary["fields"]["lat"]["masked_cell_ids"], json!([]));
        assert_eq!(summary["fields"]["lat"]["limits"], json!([0., 5.]));
        assert!(svg.contains("id=\"env_vm-0\" points="));
        assert_eq!(color(0.), "#440154");
        assert_eq!(color(0.5), "#21918c");
        assert_eq!(color(1.), "#fde725");
    }
    #[test]
    fn association_units_identity_and_forced_zero_conflicts_are_refused() {
        let (doc, contract) = fixture();
        for (key, value) in [
            ("lat", json!([1.])),
            ("twist", json!([-1., 2.])),
            ("env_vm", json!([1., 2.])),
        ] {
            let mut changed = doc.clone();
            changed["mesh"][key] = value;
            assert!(project(&changed, &contract).is_err());
        }
        let mut changed = contract.clone();
        changed["display_fields"]["twist"]["units"] = json!("MPa");
        assert!(project(&doc, &changed).is_err());
        let mut changed = doc.clone();
        changed["certificate_identity"] = json!("another");
        assert!(project(&changed, &contract).is_err());
        changed = doc;
        changed["mesh"]["tris"][0] = json!([0, 1, 99]);
        assert!(project(&changed, &contract).is_err());
    }
    #[test]
    fn rounded_boundary_zeros_require_resolution_but_positive_values_are_retained() {
        let (mut doc, contract) = fixture();
        doc["genome"] = json!("-0.4");
        doc["mesh"]["xy"][3] = json!([72.5997, 0.]);
        doc["mesh"]["xy"][4] = json!([73.5997, 0.]);
        doc["mesh"]["xy"][5] = json!([71.5997, 1.]);
        doc["mesh"]["ring"][1] = json!(0.);
        assert!(project(&doc, &contract).is_err());
        doc["mesh"]["ring"][1] = json!(0.2);
        let (_, summary) = project(&doc, &contract).unwrap();
        assert_eq!(
            summary["ring_boundary_membership_resolved_by_positive_source_value"],
            json!([1])
        );
    }
    #[test]
    fn smart_camera_preserves_original_color_scales_masks_and_every_cell_submission() {
        let (doc, mut contract)=fixture();
        contract["source"]=json!("neutral-fixture-only");
        contract["source_sha256"]=json!("0".repeat(64));
        let (full, original)=project(&doc,&contract).unwrap();
        let (smart, zoomed)=project_smart(&doc,&contract,&crate::saliency::CropPolicy::proposal_v1()).unwrap();
        for (field,_,_,_) in FIELDS {
            for key in ["limits","masked_cell_ids","raw_zero_count","count","units"] {
                assert_eq!(original["fields"][field][key],zoomed["fields"][field][key],"{field}/{key}");
            }
            for i in 0..2 {
                assert!(full.contains(&format!("id=\"{field}-{i}\"")));
                assert!(smart.contains(&format!("id=\"{field}-{i}\"")));
            }
            assert_eq!(zoomed["fields"][field]["smart_crop"]["colors_renormalized_to_crop"],false);
        }
        assert!(smart.contains("Locator"));
        assert!(smart.contains("stroke=\"#c5292b\""));
        assert!(!full.contains("Locator"));
    }

}
