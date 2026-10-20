//! Configured recipe priorities, not specimen contributions or fitness.
//! The byte-bound recovered Python source is read as data, never evaluated.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fmt::Write;
type Result<T> = std::result::Result<T, String>;
pub const RECIPE_REVISION: &str = "009f5ac520fa3afe8ed8025230eef604dbb7f922";
pub const RECIPE_PATH: &str = "ops/ci/ultralight_weighted_fitness.py";
pub const RETAINED_PATH: &str =
    "docs/paper/history/ultralight_weighted_fitness_009f5ac5.source.txt";
pub const RECIPE_SHA256: &str = "1adbd37191710d439d3ca388e468499fe48bc217546145deb57f87f1da24770b";
pub const PALETTE_ID: &str = "configured-objective-nominal-tab20-v1";
pub const WIDTH: u32 = 1600;
pub const HEIGHT: u32 = 1100;
pub const MIN_FONT: u32 = 28;
const FAMILIES: [&str; 4] = [
    "Mass/compactness",
    "Form/comfort proxies",
    "Exported geometry proxies",
    "Response/verification proxies",
];
const FAMILY_TENTHS: [u16; 4] = [300, 250, 150, 300];
const COLORS: [&str; 20] = [
    "#1f77b4", "#aec7e8", "#ff7f0e", "#ffbb78", "#2ca02c", "#98df8a", "#d62728", "#ff9896",
    "#9467bd", "#c5b0d5", "#8c564b", "#c49c94", "#e377c2", "#f7b6d2", "#7f7f7f", "#c7c7c7",
    "#bcbd22", "#dbdb8d", "#17becf", "#9edae5",
];
struct Expected {
    id: &'static str,
    label: &'static str,
    weight: &'static str,
    weight_line: u32,
    normalization_lines: [u32; 2],
    family: usize,
}
const EXPECTED: [Expected; 20] = [
    Expected {
        id: "ultralight_mass",
        label: "Mass preference",
        weight: "22.0",
        weight_line: 29,
        normalization_lines: [105, 105],
        family: 0,
    },
    Expected {
        id: "holder_and_compactness",
        label: "Holder/compactness proxy",
        weight: "8.0",
        weight_line: 30,
        normalization_lines: [106, 106],
        family: 0,
    },
    Expected {
        id: "karambit_ring_and_claw",
        label: "Ring/claw form proxy",
        weight: "7.0",
        weight_line: 32,
        normalization_lines: [107, 107],
        family: 1,
    },
    Expected {
        id: "c6_aesthetic_and_ergo",
        label: "Reference form proxy",
        weight: "9.0",
        weight_line: 33,
        normalization_lines: [108, 108],
        family: 1,
    },
    Expected {
        id: "bore_collar_comfort",
        label: "Bore/collar comfort proxy",
        weight: "9.0",
        weight_line: 34,
        normalization_lines: [109, 109],
        family: 1,
    },
    Expected {
        id: "cad_watertight_brep",
        label: "Single-solid B-rep status",
        weight: "5.0",
        weight_line: 36,
        normalization_lines: [110, 110],
        family: 2,
    },
    Expected {
        id: "gate_s_sharpness",
        label: "Edge-profile proxy",
        weight: "5.0",
        weight_line: 37,
        normalization_lines: [111, 111],
        family: 2,
    },
    Expected {
        id: "gate_s_point_profile",
        label: "Point-profile proxy",
        weight: "5.0",
        weight_line: 38,
        normalization_lines: [112, 117],
        family: 2,
    },
    Expected {
        id: "hiker_lateral_180n",
        label: "Lateral response, 180 N reference",
        weight: "3.5",
        weight_line: 40,
        normalization_lines: [118, 118],
        family: 3,
    },
    Expected {
        id: "hunter_lateral_260n",
        label: "Lateral response, 260 N reference",
        weight: "2.5",
        weight_line: 41,
        normalization_lines: [119, 119],
        family: 3,
    },
    Expected {
        id: "strict_hunter_260n",
        label: "Strict lateral response",
        weight: "2.0",
        weight_line: 42,
        normalization_lines: [120, 120],
        family: 3,
    },
    Expected {
        id: "ring_pull_1500n",
        label: "Ring-traction response",
        weight: "2.5",
        weight_line: 43,
        normalization_lines: [121, 121],
        family: 3,
    },
    Expected {
        id: "twist_body_1p82nm",
        label: "Body-torsion response",
        weight: "2.5",
        weight_line: 44,
        normalization_lines: [122, 122],
        family: 3,
    },
    Expected {
        id: "twist_tip_plate_1p82nm",
        label: "Local plate-torsion response",
        weight: "2.5",
        weight_line: 45,
        normalization_lines: [123, 123],
        family: 3,
    },
    Expected {
        id: "solid_tip_3d_p2_and_xcheck",
        label: "Reported solid-tip/comparison proxy",
        weight: "3.0",
        weight_line: 46,
        normalization_lines: [124, 124],
        family: 3,
    },
    Expected {
        id: "side_a4_buckling",
        label: "Reported support/stability proxy",
        weight: "2.0",
        weight_line: 47,
        normalization_lines: [125, 125],
        family: 3,
    },
    Expected {
        id: "failure_sequence_1p2",
        label: "Elastic yield-order proxy",
        weight: "3.5",
        weight_line: 48,
        normalization_lines: [126, 126],
        family: 3,
    },
    Expected {
        id: "plate_gci_3level",
        label: "Reported plate-study status",
        weight: "2.0",
        weight_line: 49,
        normalization_lines: [127, 127],
        family: 3,
    },
    Expected {
        id: "human_max_overload",
        label: "Overload proxy",
        weight: "2.0",
        weight_line: 50,
        normalization_lines: [128, 132],
        family: 3,
    },
    Expected {
        id: "goodman_fatigue_l7",
        label: "Goodman fatigue proxy",
        weight: "2.0",
        weight_line: 51,
        normalization_lines: [133, 133],
        family: 3,
    },
];
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn text(v: &Value) -> Result<&str> {
    v.as_str()
        .ok_or_else(|| "expected exact source decimal/string".into())
}
fn keys(v: &Value, expected: &[&str]) -> Result<()> {
    let o = v.as_object().ok_or("expected roster object")?;
    if o.len() != expected.len() || o.keys().any(|k| !expected.contains(&k.as_str())) {
        return Err("unknown, missing or candidate-score field in priorities contract".into());
    }
    Ok(())
}
/// Exactly one decimal place, no sign/exponent/whitespace/nonfinite coercion.
/// Integers represent tenths of one percentage point; never normalized.
pub fn parse_percent_tenths(value: &str) -> Result<u16> {
    let (a, b) = value.split_once('.').ok_or("expected one decimal place")?;
    if a.is_empty()
        || a.len() > 3
        || (a.len() > 1 && a.starts_with('0'))
        || !a.bytes().all(|c| c.is_ascii_digit())
        || b.len() != 1
        || !b.as_bytes()[0].is_ascii_digit()
    {
        return Err("invalid exact nonnegative percentage token".into());
    }
    let whole = a.parse::<u16>().map_err(|_| "percentage overflow")?;
    let value = whole
        .checked_mul(10)
        .and_then(|v| v.checked_add((b.as_bytes()[0] - b'0') as u16))
        .ok_or("percentage overflow")?;
    if value > 1000 {
        return Err("percentage exceeds100.0".into());
    }
    Ok(value)
}
fn percent(t: u16) -> String {
    format!("{}.{:01}", t / 10, t % 10)
}
#[derive(Clone, Debug)]
pub struct Objective {
    pub id: String,
    pub label: String,
    pub weight_tenths: u16,
    pub ordinal: usize,
    pub weight_line: u32,
    pub normalization_lines: [u32; 2],
    pub family: usize,
}
pub struct PriorityRoster {
    rows: Vec<Objective>,
    contract_sha256: String,
}
impl PriorityRoster {
    pub fn rows(&self) -> &[Objective] {
        &self.rows
    }
}

/// A source-bound template for Captain's versioned JSON, not an executed recipe.
pub fn current_priority_contract() -> Value {
    let objectives:Vec<_>=EXPECTED.iter().enumerate().map(|(i,e)|json!({
        "ordinal":i+1,"id":e.id,"scientific_label":e.label,"weight_pct":e.weight,
        "weight_source_selector":format!("ULTRALIGHT_WEIGHTS_V1['{}']",e.id),"weight_source_line":e.weight_line,
        "normalization_source_selector":format!("compute_ultralight_weighted_fitness.s_scores['{}']",e.id),
        "normalization_source_lines":e.normalization_lines,"family":FAMILIES[e.family]})).collect();
    json!({"schema":"jarambit.configured_weighted_priorities_contract.v1",
        "recipe":{"revision":RECIPE_REVISION,"original_path":RECIPE_PATH,"retained_path":RETAINED_PATH,"sha256":RECIPE_SHA256,
            "weight_selector":"ULTRALIGHT_WEIGHTS_V1","normalizer_selector":"compute_ultralight_weighted_fitness.s_scores","final_clamp_selector":"compute_ultralight_weighted_fitness:138-145"},
        "declared_sum_pct":"100.0","objectives":objectives,
        "scope":{"specimen":null,"normalized_scores_included":false,"contributions_included":false,"fitness_total_included":false,"qualification_claim":false}})
}
pub fn validate_priority_roster(contract: &Value, recipe_bytes: &[u8]) -> Result<PriorityRoster> {
    keys(
        contract,
        &[
            "schema",
            "recipe",
            "declared_sum_pct",
            "objectives",
            "scope",
        ],
    )?;
    if hash(recipe_bytes) != RECIPE_SHA256
        || contract["schema"] != "jarambit.configured_weighted_priorities_contract.v1"
        || contract["recipe"] != current_priority_contract()["recipe"]
        || contract["scope"] != current_priority_contract()["scope"]
        || text(&contract["declared_sum_pct"])? != "100.0"
    {
        return Err("recipe hash/source/scope or original total differs".into());
    }
    let rows = contract["objectives"]
        .as_array()
        .ok_or("missing objective array")?;
    if rows.len() != 20 {
        return Err("expected exactly20 configured objectives".into());
    }
    let (mut ids, mut out, mut total, mut family) =
        (BTreeSet::new(), Vec::new(), 0_u16, [0_u16; 4]);
    for row in rows {
        keys(
            row,
            &[
                "ordinal",
                "id",
                "scientific_label",
                "weight_pct",
                "weight_source_selector",
                "weight_source_line",
                "normalization_source_selector",
                "normalization_source_lines",
                "family",
            ],
        )?;
        let id = text(&row["id"])?;
        if !ids.insert(id) {
            return Err("duplicate configured objective".into());
        }
        let (i, e) = EXPECTED
            .iter()
            .enumerate()
            .find(|(_, e)| e.id == id)
            .ok_or("unknown configured objective")?;
        let weight = text(&row["weight_pct"])?;
        let tenths = parse_percent_tenths(weight)?;
        if weight != e.weight
            || row["ordinal"] != json!(i + 1)
            || text(&row["scientific_label"])? != e.label
            || row["weight_source_line"] != e.weight_line
            || row["normalization_source_lines"] != json!(e.normalization_lines)
            || text(&row["weight_source_selector"])? != format!("ULTRALIGHT_WEIGHTS_V1['{}']", id)
            || text(&row["normalization_source_selector"])?
                != format!("compute_ultralight_weighted_fitness.s_scores['{}']", id)
            || text(&row["family"])? != FAMILIES[e.family]
        {
            return Err("original configured ID/weight/name/selector binding differs".into());
        }
        total = total.checked_add(tenths).ok_or("total overflow")?;
        family[e.family] = family[e.family]
            .checked_add(tenths)
            .ok_or("family sum overflow")?;
        out.push(Objective {
            id: id.into(),
            label: e.label.into(),
            weight_tenths: tenths,
            ordinal: i + 1,
            weight_line: e.weight_line,
            normalization_lines: e.normalization_lines,
            family: e.family,
        });
    }
    if total != 1000 || family != FAMILY_TENTHS {
        return Err("original sum/family partition differs; no normalization".into());
    }
    out.sort_by_key(|r| r.ordinal);
    Ok(PriorityRoster {
        rows: out,
        contract_sha256: hash(&serde_json::to_vec(contract).map_err(|e| e.to_string())?),
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Arc {
    pub start: u16,
    pub end: u16,
}
/// Fixed1000-tenth circle after EXACT sum validation; never divides by observed sum.
/// Zero retains a row/count but start==end, hence no filled path.
pub fn arcs_tenths(weights: &[u16]) -> Result<Vec<Arc>> {
    if weights.is_empty()
        || weights.iter().any(|v| *v > 1000)
        || weights
            .iter()
            .try_fold(0_u32, |sum, v| sum.checked_add(u32::from(*v)))
            != Some(1000)
    {
        return Err("arc weights must total exact100.0; normalization refused".into());
    }
    let mut at = 0;
    Ok(weights
        .iter()
        .map(|w| {
            let a = Arc {
                start: at,
                end: at + *w,
            };
            at += *w;
            a
        })
        .collect())
}
pub fn unit_point(tick: u16) -> Result<[f64; 2]> {
    if tick > 1000 {
        return Err("angle outside configured circle".into());
    }
    // Identical endpoint bits close the final ring without rescaling any weight.
    let tick = if tick == 1000 { 0 } else { tick };
    let a = tick as f64 * std::f64::consts::TAU / 1000. - std::f64::consts::FRAC_PI_2;
    Ok([a.cos(), a.sin()])
}
pub fn arc_path(arc: Arc, center: [f64; 2], outer: f64, inner: f64) -> Result<Option<String>> {
    if arc.start > arc.end
        || arc.end > 1000
        || center.iter().any(|v| !v.is_finite())
        || !outer.is_finite()
        || !inner.is_finite()
        || inner <= 0.
        || outer <= inner
    {
        return Err("invalid annular geometry".into());
    }
    if arc.start == arc.end {
        return Ok(None);
    }
    let point = |t, r| -> Result<[f64; 2]> {
        let p = unit_point(t)?;
        let q = [center[0] + r * p[0], center[1] + r * p[1]];
        if q.iter().any(|v| !v.is_finite()) {
            return Err("nonfinite arc endpoint".into());
        }
        Ok(q)
    };
    let start = point(arc.start, outer)?;
    let mut d = format!("M {:.6} {:.6}", start[0], start[1]);
    let mut at = arc.start;
    while at < arc.end {
        at = (at + 500).min(arc.end);
        let p = point(at, outer)?;
        write!(d, " A {outer:.6} {outer:.6} 0 0 1 {:.6} {:.6}", p[0], p[1]).unwrap();
    }
    let end = point(arc.end, inner)?;
    write!(d, " L {:.6} {:.6}", end[0], end[1]).unwrap();
    at = arc.end;
    while at > arc.start {
        at = at.saturating_sub(500).max(arc.start);
        let p = point(at, inner)?;
        write!(d, " A {inner:.6} {inner:.6} 0 0 0 {:.6} {:.6}", p[0], p[1]).unwrap();
    }
    d.push_str(" Z");
    Ok(Some(d))
}
fn xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
fn label(svg: &mut String, x: f64, y: f64, size: u32, anchor: &str, body: &str) {
    writeln!(
        svg,
        r##"<text x="{x}" y="{y}" font-size="{size}" text-anchor="{anchor}">{}</text>"##,
        xml(body)
    )
    .unwrap();
}
pub struct PriorityOutput {
    pub svg: String,
    pub manifest: Value,
}
pub fn render_priority_donut(
    roster: &PriorityRoster,
    declared_print_width_pt: f64,
) -> Result<PriorityOutput> {
    let effective_pt = MIN_FONT as f64 * declared_print_width_pt / WIDTH as f64;
    if !declared_print_width_pt.is_finite() || !effective_pt.is_finite() || effective_pt < 8.5 {
        return Err("priority key must be full-width and at least8.5pt".into());
    }
    let weights: Vec<_> = roster.rows.iter().map(|r| r.weight_tenths).collect();
    let arcs = arcs_tenths(&weights)?;
    let (center, outer, inner) = ([310., 500.], 205., 125.);
    let mut svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {WIDTH} {HEIGHT}" width="{WIDTH}" height="{HEIGHT}"><rect width="{WIDTH}" height="{HEIGHT}" fill="#fff"/><g font-family="Arial, sans-serif" fill="#132d3b">"##
    );
    label(
        &mut svg,
        40.,
        46.,
        32,
        "start",
        "Configured priorities of the recovered weighted reporting recipe",
    );
    label(
        &mut svg,
        40.,
        88.,
        28,
        "start",
        "20 objectives; 100.0% configured weight; colors identify objectives",
    );
    let mut manifest_rows = Vec::new();
    let mut leaders = [Vec::new(), Vec::new()];
    for (i, (row, arc)) in roster.rows.iter().zip(&arcs).enumerate() {
        let color = COLORS[i];
        if let Some(d) = arc_path(*arc, center, outer, inner)? {
            writeln!(svg,r##"<path data-role="slice" data-objective-id="{}" data-ordinal="{}" d="{d}" fill="{color}" stroke="#111" stroke-width="1"/>"##,xml(&row.id),row.ordinal).unwrap();
            let mid = (arc.start as f64 + arc.end as f64) * 0.5 * std::f64::consts::TAU / 1000.
                - std::f64::consts::FRAC_PI_2;
            let (x, y) = (center[0] + 225. * mid.cos(), center[1] + 225. * mid.sin());
            let side = usize::from(mid.cos() >= 0.);
            leaders[side].push((y, x, row.ordinal, mid));
        }
        let y = 148. + 38. * i as f64;
        writeln!(svg,r##"<g data-role="key" data-objective-id="{}" data-ordinal="{}"><rect x="620" y="{}" width="30" height="30" fill="{color}" stroke="#111" stroke-width="1"/>"##,xml(&row.id),row.ordinal,y-24.).unwrap();
        label(
            &mut svg,
            666.,
            y,
            28,
            "start",
            &format!("{:02}", row.ordinal),
        );
        label(&mut svg, 716., y, 28, "start", &row.label);
        label(
            &mut svg,
            1560.,
            y,
            28,
            "end",
            &format!(
                "{}%{}",
                percent(row.weight_tenths),
                if row.weight_tenths == 0 {
                    " (zero weight)"
                } else {
                    ""
                }
            ),
        );
        svg.push_str("</g>");
        manifest_rows.push(json!({"id":row.id,"ordinal":row.ordinal,"scientific_label":row.label,"weight_pct_exact_decimal":percent(row.weight_tenths),"weight_tenths":row.weight_tenths,
            "source_weight_selector":format!("ULTRALIGHT_WEIGHTS_V1['{}']",row.id),"source_weight_line":row.weight_line,
            "normalization_selector":format!("compute_ultralight_weighted_fitness.s_scores['{}']",row.id),"normalization_lines":row.normalization_lines,
            "final_clamp_selector":"compute_ultralight_weighted_fitness:138-145","family":FAMILIES[row.family],"nominal_color":color,"start_tick":arc.start,"end_tick":arc.end,
            "angle_degrees_exact_rational":format!("{}/1000 * 360",row.weight_tenths),"zero_weight":row.weight_tenths==0}));
    }
    // Visible numbers plus geometric connectors make the argument independent of
    // distinguishing all20 hues. Two side columns prevent tiny-slice text overlap.
    for (side, list) in leaders.iter_mut().enumerate() {
        list.sort_by(|a, b| a.0.total_cmp(&b.0));
        let count = list.len();
        for (j, &(_, x, ordinal, mid)) in list.iter().enumerate() {
            let y = if count == 1 {
                500.
            } else {
                168. + 684. * j as f64 / (count - 1) as f64
            };
            let lx = if side == 0 { 62. } else { 568. };
            let edge_x = if side == 0 { 88. } else { 542. };
            let edge = [center[0] + outer * mid.cos(), center[1] + outer * mid.sin()];
            writeln!(svg,r##"<path data-role="ordinal-connector" data-ordinal="{ordinal}" d="M {:.6} {:.6} L {x:.6} {:.6} L {edge_x} {}" fill="none" stroke="#333" stroke-width="1"/>"##,edge[0],edge[1],center[1]+225.*mid.sin(),y-10.).unwrap();
            label(&mut svg, lx, y, 28, "middle", &format!("{ordinal:02}"));
        }
    }
    label(&mut svg, 310., 448., 30, "middle", "20 configured");
    label(&mut svg, 310., 487., 30, "middle", "objectives");
    label(&mut svg, 310., 536., 34, "middle", "100.0%");
    label(&mut svg, 310., 575., 28, "middle", "total weight");
    label(
        &mut svg,
        40.,
        958.,
        28,
        "start",
        "Recipe preferences, not specimen performance or safety probabilities.",
    );
    label(
        &mut svg,
        40.,
        1000.,
        28,
        "start",
        "Exact historical execution and summary-stage bindings remain unresolved.",
    );
    label(
        &mut svg,
        40.,
        1042.,
        28,
        "start",
        "Numbers link slices and key. Weights confer no physical qualification.",
    );
    svg.push_str("</g></svg>\n");
    let manifest = priority_manifest(roster, declared_print_width_pt, &svg, manifest_rows);
    Ok(PriorityOutput { svg, manifest })
}
fn priority_manifest(
    roster: &PriorityRoster,
    declared_print_width_pt: f64,
    svg: &str,
    rows: Vec<Value>,
) -> Value {
    let family_rows: Vec<_> = FAMILIES
        .iter()
        .zip(FAMILY_TENTHS)
        .map(|(name, w)| json!({"family":name,"sum_pct":percent(w)}))
        .collect();
    json!({"schema":"jarambit.configured_weighted_priorities_projection.v1","recipe":{"revision":RECIPE_REVISION,"path":RECIPE_PATH,"retained_path":RETAINED_PATH,"sha256":RECIPE_SHA256},
        "contract_json_value_sha256":roster.contract_sha256,"objective_count":roster.rows.len(),"source_weight_sum_pct_exact_decimal":"100.0","display_weight_sum_pct_exact_decimal":"100.0",
        "zero_weight_ids":roster.rows.iter().filter(|r|r.weight_tenths==0).map(|r|r.id.as_str()).collect::<Vec<_>>(),"family_sums":family_rows,"objectives":rows,
        "angles":{"denominator_tenths":1000,"start":"12 o'clock; clockwise","normalization":"none; input sum must already be100.0","path_coordinates":"six decimal SVG units; final1000 tick reuses exact0 endpoint","closure":"exact integer tick1000; no adjusted last weight; full-circle arcs split into <=180-degree pieces"},
        "palette":{"id":PALETTE_ID,"semantics":"nominal objective identity only; no ordinal magnitude/good-bad meaning","additional_cues":"visible objective numbers on both diagram and key, connected to actual slices; black separators","universal_color_vision_accessibility_claim":false},
        "typography":{"svg_width":WIDTH,"svg_height":HEIGHT,"minimum_font_svg_units":MIN_FONT,"declared_fullwidth_pt":declared_print_width_pt,"minimum_effective_pt":MIN_FONT as f64*declared_print_width_pt/WIDTH as f64,"actual_PDF_readback":"pending independent artifact review"},
        "specimen":null,"normalized_scores_included":false,"contributions_included":false,"fitness_total_included":false,"qualification_claim":false,
        "historical_execution":"Compatible recipe recovered; exact executed source/call inputs/point-stage binding unresolved; this chart does not evaluate any term", "gate_free_claim":false,
        "outputs":{"svg_sha256":hash(svg.as_bytes()),"png_sha256":null},"png_binding":"Captain must add actual encoded PNG hash after generation, never invent one"})
}
