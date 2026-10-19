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
            return Err("source seated-zero mask disagrees with ring field".into());
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
}
