//! Hand-written SVG renderings of the fields (the style of `g5-release`'s figures): banded contours in five bands of
//! the dataviz sequential blue ramp (light: steps 250-650, dark: 600-200, every other step, so neighbouring bands
//! differ by ΔL ≈ 0.09 and the lowest band clears 2:1 on its surface; both pass `validate_palette.js --ordinal`),
//! a colour bar, the deformed shape with its scale factor, the governing point, and the case, load and SF in the title.
//! Elements a case does not read (clamp zones, load patches) are drawn grey.
use super::plate::{PlateCase, PlateFields};
use super::ring::{RingCaseField, RingFields};
use super::tip::SolidTipResult;
use jarambit_g5::g5_sandwich::RingBandSummary;
use jarambit_g5::g5_sdf::BladeProfile2D;
use jarambit_g5::loads::G5Loads;
use jarambit_g5::requirements::rs_v5;
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

pub type P2 = (f64, f64);

pub struct Ctx<'a> {
    pub id: &'a str,
    pub h: f64,
    pub record_sha: String,
}

/// The bands' colours, low to high: light mode steps 250, 350, 450, 550, 650; dark mode 600, 500, 400, 300, 200.
pub const RAMP_LIGHT: [&str; 5] = ["#86b6ef", "#5598e7", "#2a78d6", "#1c5cab", "#104281"];
pub const RAMP_DARK: [&str; 5] = ["#184f95", "#256abf", "#3987e5", "#6da7ec", "#9ec5f4"];

const W: f64 = 760.0;
const H: f64 = 440.0;
/// The plot panel and the colour bar
const PANEL: (f64, f64, f64, f64) = (24.0, 64.0, 624.0, 392.0);
const BAR_X: f64 = 652.0;

/// The required SF (or ratio) of a case, as `objective::margins` divides by it.
pub fn required(case: &str) -> f64 {
    let rs = rs_v5();
    let m = &rs.margins;
    match case {
        "lateral_ultimate" => m.sf_ultimate,
        "ring_bearing" | "ring_back" => m.sf_ring,
        "fatigue" => rs.fatigue.sf_fatigue,
        "sequence" => rs.sequence.ratio_min,
        "buckling" => rs.buckling.lambda_min,
        _ => m.sf_yield,
    }
}

fn style() -> String {
    let fills = |dark: bool| -> String {
        (0..5).map(|k| format!(".b{k}{{fill:{c};stroke:{c}}}", c = if dark { RAMP_DARK[k] } else { RAMP_LIGHT[k] })).collect()
    };
    format!(
        "<style>\n.bg{{fill:#fcfcfb}}.t1,.v{{fill:#0b0b0b}}.t2,.lb{{fill:#52514e}}.tk{{fill:#898781}}.ol{{fill:none;stroke:#52514e}}\
         .nr{{fill:#e1e0d9;stroke:#e1e0d9}}.pk{{fill:none;stroke:#0b0b0b}}.ax{{stroke:#c3c2b7}}.ar{{stroke:#0b0b0b;fill:#0b0b0b}}\
         .ln{{fill:none;stroke:#2a78d6}}.ln2{{fill:none;stroke:#eb6834}}.grid{{stroke:#e1e0d9}}{}\n\
         @media (prefers-color-scheme: dark){{.bg{{fill:#1a1a19}}.t1,.v{{fill:#f0efec}}.t2,.lb{{fill:#c3c2b7}}.tk{{fill:#898781}}\
         .ol{{stroke:#c3c2b7}}.nr{{fill:#2c2c2a;stroke:#2c2c2a}}.pk{{stroke:#f0efec}}.ax{{stroke:#383835}}.ar{{stroke:#f0efec;fill:#f0efec}}\
         .ln{{stroke:#3987e5}}.ln2{{stroke:#d95926}}.grid{{stroke:#2c2c2a}}{}}}\n\
         .t1{{font-size:15px;font-weight:600}}.t2,.lb{{font-size:12px}}.tk{{font-size:11px}}.v{{font-size:12px;font-weight:600}}\
         .ol{{stroke-width:0.8;stroke-linejoin:round}}.pk{{stroke-width:1.8}}.ax,.grid{{stroke-width:1}}.ar{{stroke-width:1.6}}\
         .ln,.ln2{{stroke-width:2;stroke-linejoin:round}}\n</style>\n",
        fills(false),
        fills(true)
    )
}

pub fn open(w: f64, h: f64, title: &str, desc: &str) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {w} {h}\" width=\"{w}\" height=\"{h}\" role=\"img\" \
         aria-labelledby=\"t d\" font-family=\"system-ui, -apple-system, 'Segoe UI', sans-serif\">\n{}<title id=\"t\">{}</title>\n\
         <desc id=\"d\">{}</desc>\n<rect class=\"bg\" width=\"{w}\" height=\"{h}\" rx=\"8\"/>\n",
        style(),
        esc(title),
        esc(desc)
    )
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

pub fn header(o: &mut String, title: &str, sub: &str) {
    let _ = writeln!(o, "<text class=\"t1\" x=\"24\" y=\"30\">{}</text>", esc(title));
    let _ = writeln!(o, "<text class=\"t2\" x=\"24\" y=\"50\">{}</text>", esc(sub));
}

pub fn footer(o: &mut String, lines: &[String]) {
    for (k, l) in lines.iter().rev().enumerate() {
        let _ = writeln!(o, "<text class=\"tk\" x=\"24\" y=\"{:.0}\">{}</text>", H - 12.0 - 15.0 * k as f64, esc(l));
    }
}

/// Five equal bands from 0 to a nice top at or above the largest read value (the top band takes everything above).
pub struct Bands {
    pub step: f64,
    pub n: usize,
}

impl Bands {
    pub fn new(vmax: f64) -> Self {
        let vmax = if vmax.is_finite() && vmax > 0.0 { vmax } else { 1.0 };
        let mag = 10f64.powf(vmax.log10().floor());
        let top = [1.0, 1.2, 1.5, 2.0, 2.5, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0].iter().map(|m| m * mag).find(|&t| t >= vmax * (1.0 - 1e-12)).unwrap_or(10.0 * mag);
        Self { step: top / 5.0, n: 5 }
    }
    fn colour(&self, i: usize) -> usize {
        i.min(4)
    }
    pub fn class(&self, v: f64) -> String {
        let i = if v.is_finite() { ((v / self.step).floor().max(0.0) as usize).min(self.n - 1) } else { self.n - 1 };
        format!("b{}", self.colour(i))
    }
    pub fn top(&self) -> f64 {
        self.step * self.n as f64
    }
}

fn fmt_tick(v: f64, step: f64) -> String {
    let d = (0..6).find(|&d| ((step * 10f64.powi(d)).round() - step * 10f64.powi(d)).abs() < 1e-9 * step.max(1e-300) * 10f64.powi(d)).unwrap_or(3) as usize;
    format!("{v:.d$}")
}

pub fn colour_bar(o: &mut String, b: &Bands, quantity: &str, saturated: bool) {
    let (y0, y1) = (PANEL.1 + 18.0, PANEL.3 - 8.0);
    let bh = (y1 - y0) / b.n as f64;
    let _ = writeln!(o, "<text class=\"lb\" x=\"{BAR_X}\" y=\"{:.0}\">{}</text>", PANEL.1 + 4.0, esc(quantity));
    for i in 0..b.n {
        let y = y1 - bh * (i + 1) as f64;
        let _ = write!(o, "<rect class=\"b{}\" x=\"{BAR_X}\" y=\"{y:.1}\" width=\"16\" height=\"{:.1}\"/>", b.colour(i), bh);
    }
    for i in 0..=b.n {
        let y = y1 - bh * i as f64;
        let label = if i == b.n && saturated { format!("{}+", fmt_tick(b.step * i as f64, b.step)) } else { fmt_tick(b.step * i as f64, b.step) };
        let _ = write!(o, "<text class=\"tk\" x=\"{:.0}\" y=\"{:.1}\">{label}</text>", BAR_X + 22.0, y + 4.0);
    }
    o.push('\n');
}

/// Screen mapping of a world box into a panel, y up, equal scales.
#[derive(Clone, Copy)]
pub struct Fit {
    s: f64,
    ox: f64,
    oy: f64,
    xmin: f64,
    ymax: f64,
}

impl Fit {
    pub fn new(pts: impl Iterator<Item = P2>, panel: (f64, f64, f64, f64), pad: f64) -> Self {
        let (mut xmin, mut xmax, mut ymin, mut ymax) = (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY);
        for (x, y) in pts {
            xmin = xmin.min(x);
            xmax = xmax.max(x);
            ymin = ymin.min(y);
            ymax = ymax.max(y);
        }
        let (pw, ph) = (panel.2 - panel.0 - 2.0 * pad, panel.3 - panel.1 - 2.0 * pad);
        let s = (pw / (xmax - xmin).max(1e-9)).min(ph / (ymax - ymin).max(1e-9));
        let ox = panel.0 + pad + 0.5 * (pw - s * (xmax - xmin));
        let oy = panel.1 + pad + 0.5 * (ph - s * (ymax - ymin));
        Self { s, ox, oy, xmin, ymax }
    }
    pub fn at(&self, p: P2) -> P2 {
        (self.ox + self.s * (p.0 - self.xmin), self.oy + self.s * (self.ymax - p.1))
    }
}

/// Filled triangles, optionally drawn far to near (`depth`, larger = farther), consecutive same-class runs merged.
fn tris(o: &mut String, mut t: Vec<(f64, [P2; 3], String)>, sort: bool) {
    if sort {
        t.sort_by(|a, b| b.0.total_cmp(&a.0));
    }
    let _ = writeln!(o, "<g stroke-width=\"0.35\" stroke-linejoin=\"round\">");
    let mut k = 0;
    while k < t.len() {
        let cls = t[k].2.clone();
        let mut d = String::new();
        while k < t.len() && t[k].2 == cls {
            let [a, b, c] = t[k].1;
            let _ = write!(d, "M{:.1} {:.1}L{:.1} {:.1}L{:.1} {:.1}Z", a.0, a.1, b.0, b.1, c.0, c.1);
            k += 1;
        }
        let _ = writeln!(o, "<path class=\"{cls}\" d=\"{d}\"/>");
    }
    let _ = writeln!(o, "</g>");
}

/// Outline edges (edges of one triangle only) as one path.
fn outline(o: &mut String, tri_idx: &[[usize; 3]], at: &dyn Fn(usize) -> P2) {
    let mut count: HashMap<(usize, usize), usize> = HashMap::new();
    for t in tri_idx {
        for (a, b) in [(t[0], t[1]), (t[1], t[2]), (t[2], t[0])] {
            *count.entry((a.min(b), a.max(b))).or_default() += 1;
        }
    }
    let mut edges: Vec<(usize, usize)> = count.into_iter().filter(|e| e.1 == 1).map(|e| e.0).collect();
    edges.sort_unstable();
    let mut d = String::new();
    for (a, b) in edges {
        let (p, q) = (at(a), at(b));
        let _ = write!(d, "M{:.1} {:.1}L{:.1} {:.1}", p.0, p.1, q.0, q.1);
    }
    let _ = writeln!(o, "<path class=\"ol\" d=\"{d}\"/>");
}

fn peak(o: &mut String, at: P2, label: &str) {
    let _ = write!(o, "<circle class=\"pk\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"6\"/>", at.0, at.1);
    let right = at.0 < PANEL.2 - 240.0;
    let (lx, ly) = (if right { at.0 + 12.0 } else { at.0 - 12.0 }, (at.1 - 14.0).max(PANEL.1 + 12.0));
    let _ = write!(o, "<path class=\"pk\" d=\"M{:.1} {:.1}L{lx:.1} {ly:.1}\" stroke-width=\"1\"/>", at.0 + if right { 4.5 } else { -4.5 }, at.1 - 4.0);
    let _ = writeln!(
        o,
        "<text class=\"v\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"{}\">{}</text>",
        if right { lx + 3.0 } else { lx - 3.0 },
        ly - 3.0,
        if right { "start" } else { "end" },
        esc(label)
    );
}

/// A scale factor that makes the largest displacement `frac` of `span`, rounded down to 1, 2 or 5 x 10^n.
fn deform_scale(umax: f64, span: f64, frac: f64) -> f64 {
    if umax <= 0.0 || !umax.is_finite() {
        return 0.0;
    }
    let raw = frac * span / umax;
    let mag = 10f64.powf(raw.log10().floor());
    [5.0, 2.0, 1.0].iter().map(|m| m * mag).find(|&k| k <= raw).unwrap_or(mag)
}

fn fmt_k(k: f64) -> String {
    if k >= 1.0 {
        format!("{k:.0}")
    } else {
        format!("{k}")
    }
}

/// One figure's files and summary row.
fn write_fig(out: &Path, case: &str, svg: String, mut j: Value, ctx: &Ctx) -> Value {
    fs::write(out.join(format!("{case}.svg")), svg).unwrap_or_else(|e| super::fail(e));
    j["design_id"] = json!(ctx.id);
    j["h_mm"] = json!(ctx.h);
    j["record_sha256"] = json!(ctx.record_sha);
    fs::write(out.join(format!("{case}.json")), serde_json::to_string_pretty(&j).unwrap() + "\n").unwrap_or_else(|e| super::fail(e));
    json!({"case": case, "svg": format!("{case}.svg"), "json": format!("{case}.json"), "title": j["title"],
           "sf": j["sf_fem"], "record_sf": j["sf_record"], "match": j["match"], "gated": j["gated"]})
}

fn sf_json(sf: f64, rec: f64) -> Value {
    json!({"sf_fem": sf, "sf_record": rec, "match": sf.to_bits() == rec.to_bits(), "gated": true})
}

fn merge(mut a: Value, b: Value) -> Value {
    if let (Value::Object(x), Value::Object(y)) = (&mut a, b) {
        for (k, v) in y {
            x.insert(k, v);
        }
    }
    a
}

fn match_note(sf: f64, rec: f64) -> String {
    if sf.to_bits() == rec.to_bits() {
        "Reproduces the record's SF bit for bit (Tier 2 at h 0.2, same platform).".to_string()
    } else {
        format!("MISMATCH: this solve gives {sf} but the record has {rec}.")
    }
}

// ---------------- plate ----------------

pub fn plate_figures(
    ctx: &Ctx,
    out: &Path,
    f: &PlateFields,
    loads: &G5Loads,
    rec: &G5Loads,
    margins: &BTreeMap<&str, f64>,
    blade: &BladeProfile2D,
) -> Vec<Value> {
    let ld = &rs_v5().loads;
    let fat = &rs_v5().fatigue;
    let span = blade.x_tip - blade.x_heel;
    f.cases
        .iter()
        .map(|c| {
            let (title, load, sf, sf_rec, grip): (&str, String, f64, f64, Option<(&str, f64)>) = match c.name {
                "lateral_limit" => ("Lateral bending, limit load", format!("{:.1} N normal to the blade, {} mm behind the point (root moment {} N m)", c.load, ld.lateral_bend.apply_mm_from_point, ld.lateral_bend.limit_root_moment_nm), loads.lateral_limit_sf, rec.lateral_limit_sf, Some(("grip, lateral (formula)", rec.grip_lateral_sf))),
                "lateral_ultimate" => ("Lateral bending, ultimate load", format!("{:.1} N normal to the blade (root moment {} N m), against the ultimate strength", c.load, ld.lateral_bend.ultimate_root_moment_nm), loads.lateral_ultimate_sf, rec.lateral_ultimate_sf, None),
                "twist_limit" => ("Body twist, limit load", format!("{} N m about the blade axis, over 25-75 % of the span", ld.body_twist.limit_nm), loads.twist_limit_sf, rec.twist_limit_sf, Some(("grip, twist (formula)", rec.grip_twist_sf))),
                "pull" => ("Pull", format!("{} N along the grip axis, the blade held over its last {} mm", ld.pull.limit_n, ld.pull.held_length_from_point_mm), loads.pull_sf, rec.pull_sf, Some(("grip, pull (formula)", rec.grip_pull_sf))),
                _ => ("Fatigue, fully reversed duty", format!("amplitudes {} N m root moment and {} N m twist, {:.0e} cycles", fat.blade_root_moment_amplitude_nm, fat.twist_amplitude_nm, fat.cycles), loads.fatigue_sf, rec.fatigue_sf, Some(("hand side (formula)", rec.hand_fatigue_sf))),
            };
            let in_plane = c.name == "pull";
            let svg = plate_svg(f, c, title, &load, sf, in_plane, span, margins, grip, sf_rec);
            let e = c.governing;
            let t = f.tris[e];
            let cen = ((f.xy[t[0]].0 + f.xy[t[1]].0 + f.xy[t[2]].0) / 3.0, (f.xy[t[0]].1 + f.xy[t[1]].1 + f.xy[t[2]].1) / 3.0);
            let max_read = (0..c.sigma.len()).filter(|&e| c.read[e]).map(|e| c.sigma[e]).fold(0.0, f64::max);
            let allow = if c.name == "fatigue" { Value::Null } else if c.name == "lateral_ultimate" { json!(f.allow_u[e]) } else { json!(f.allow_y[e]) };
            let j = json!({
                "case": c.name, "title": title, "load": load, "model": "CST+DKT sandwich plate of the whole blade, clamped at the heel",
                "mesh": {"nodes": f.xy.len(), "elements": f.tris.len(), "max_area_mm2": jarambit_g5::loads::PLATE_MAX_AREA_MM2},
                "field": {"quantity": if c.name == "fatigue" { "stress amplitude max(k_b s_lat, k_t s_twist), MPa" } else { "von Mises (membrane + outer-skin bending), MPa" },
                          "max_read_mpa": max_read, "max_all_mpa": c.sigma.iter().copied().fold(0.0, f64::max),
                          "governing_element": e, "governing_at_mm": [cen.0, cen.1], "governing_sigma_mpa": c.sigma[e],
                          "governing_allowable_mpa": allow, "governing_hollow": f.hollow[e],
                          "allowable_note": if c.name == "fatigue" { "fatigue SF = fatigue strength of the face class (ground outer faces; as-built cavity faces of a hollow blade at (t - 2 skin)/t of the amplitude) over the amplitude" } else { "first-yield (ultimate for the ultimate case) allowable of the element's wall" }},
                "required_sf": required(c.name), "scored_margin": margins.get(c.name),
                "scored_sf_note": grip.map(|g| format!("the scored SF is min(plate, {}) = min({sf}, {})", g.0, g.1)),
            });
            write_fig(out, c.name, svg, merge(j, sf_json(sf, sf_rec)), ctx)
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn plate_svg(
    f: &PlateFields,
    c: &PlateCase,
    title: &str,
    load: &str,
    sf: f64,
    in_plane: bool,
    span: f64,
    margins: &BTreeMap<&str, f64>,
    grip: Option<(&str, f64)>,
    sf_rec: f64,
) -> String {
    let n = f.xy.len();
    let (phi, deg) = (35f64.to_radians(), 35.0);
    let umax = (0..n)
        .map(|i| if in_plane { c.u[i * 5].hypot(c.u[i * 5 + 1]) } else { c.u[i * 5 + 2].abs() })
        .fold(0.0, f64::max);
    let k = deform_scale(umax, span, if in_plane { 0.04 } else { 0.10 });
    // world -> projected plane (and depth): in-plane cases on the deformed planform; out-of-plane ones in an oblique
    // view tilted 35 degrees about x, the deflection w drawn upward
    let proj = |i: usize, deformed: bool| -> (P2, f64) {
        let (x, y) = f.xy[i];
        let kk = if deformed { k } else { 0.0 };
        if in_plane {
            ((x + kk * c.u[i * 5], y + kk * c.u[i * 5 + 1]), 0.0)
        } else {
            let w = kk * c.u[i * 5 + 2];
            ((x, y * phi.cos() + w * phi.sin()), y * phi.sin() - w * phi.cos())
        }
    };
    let fit = Fit::new((0..n).flat_map(|i| [proj(i, true).0, proj(i, false).0]), PANEL, 6.0);
    let max_read = (0..c.sigma.len()).filter(|&e| c.read[e]).map(|e| c.sigma[e]).fold(0.0, f64::max);
    let bands = Bands::new(max_read);
    let saturated = c.sigma.iter().enumerate().any(|(e, &s)| c.read[e] && s > bands.top()) || c.sigma.iter().any(|&s| s > bands.top());
    let mut t = Vec::with_capacity(f.tris.len());
    for (e, tri) in f.tris.iter().enumerate() {
        let pts = [fit.at(proj(tri[0], true).0), fit.at(proj(tri[1], true).0), fit.at(proj(tri[2], true).0)];
        let depth = (proj(tri[0], true).1 + proj(tri[1], true).1 + proj(tri[2], true).1) / 3.0;
        let cls = if c.read[e] { bands.class(c.sigma[e]) } else { "nr".to_string() };
        t.push((depth, pts, cls));
    }
    let quantity = if c.name == "fatigue" { "amplitude, MPa" } else { "von Mises, MPa" };
    let scored = margins.get(c.name).map(|m| format!(" · margin {m:.3}")).unwrap_or_default();
    let mut o = open(W, H, &format!("{title}: SF {sf:.3}"), &format!("{title}. {load}. Plate SF {sf:.3}, required {}.", required(c.name)));
    header(&mut o, &format!("{title} · SF {sf:.3} (required {}){scored}", required(c.name)), load);
    tris(&mut o, t, !in_plane);
    outline(&mut o, &f.tris, &|i| fit.at(proj(i, false).0));
    let e = c.governing;
    let tri = f.tris[e];
    let g = fit.at(((proj(tri[0], true).0 .0 + proj(tri[1], true).0 .0 + proj(tri[2], true).0 .0) / 3.0,
                    (proj(tri[0], true).0 .1 + proj(tri[1], true).0 .1 + proj(tri[2], true).0 .1) / 3.0));
    let allow = if c.name == "lateral_ultimate" { f.allow_u[e] } else { f.allow_y[e] };
    let label = if c.name == "fatigue" {
        format!("governing: amplitude {:.0} MPa, fatigue SF {sf:.2}", c.sigma[e])
    } else {
        format!("governing: {:.0} MPa of {:.0} allowable", c.sigma[e], allow)
    };
    peak(&mut o, g, &label);
    colour_bar(&mut o, &bands, quantity, saturated);
    let view = if in_plane { format!("deformation ×{}", fmt_k(k)) } else { format!("oblique {deg:.0}°, deflection ×{}", fmt_k(k)) };
    let mut notes = vec![
        format!("CST+DKT plate, {} elements of 0.3 mm² · clamped x ≤ heel + 3.5 mm · {view} · line: undeformed", f.tris.len()),
        format!("Grey: not read (x < heel + 6 mm{}).", if c.name == "pull" { "; the held 20 mm + 2 mm" } else if c.name == "twist_limit" { "" } else { "; 4 mm around the load patch" }),
    ];
    if let Some((what, v)) = grip {
        notes[1] += &format!(" The scored SF is min(plate {sf:.2}, {what} {v:.2}).");
    }
    notes.push(match_note(sf, sf_rec));
    footer(&mut o, &notes);
    o + "</svg>\n"
}

// ---------------- tip ----------------

pub fn tip_figures(
    ctx: &Ctx,
    out: &Path,
    tips: &[(&'static str, f64, SolidTipResult, (f64, f64))],
    rec: &G5Loads,
    margins: &BTreeMap<&str, f64>,
    blade: &BladeProfile2D,
) -> Vec<Value> {
    let ld = &rs_v5().loads.tip;
    tips.iter()
        .map(|(case, sf, r, win)| {
            let (title, load, sf_rec) = match *case {
                "tip_axial" => ("Point, axial", format!("{} N along the tip bisector on the last 0.75 mm, read from {} to {} mm", ld.axial_n, win.0, win.1), rec.tip_axial_sf),
                "tip_lateral" => ("Point, side load", format!("{} N normal to the blade on the last 0.75 mm, read from {} to {} mm", ld.lateral_n, win.0, win.1), rec.tip_lateral_sf),
                _ => ("Point, twist", format!("{} N m about the tip bisector, {} mm behind the point; read from {} to {} mm", ld.twist_nm, ld.twist_apply_mm_from_point, win.0, win.1), rec.tip_twist_sf),
            };
            let svg = tip_svg(r, title, &load, *sf, sf_rec, case, *win, blade, margins);
            let fld = &r.fields;
            let pv = fld.peak_vertex;
            let j = json!({
                "case": case, "title": title, "load": load, "model": "TET10 solid of the last 30 mm of the blade, 4 layers through the thickness, clamped at the cut",
                "mesh": {"vertices": fld.pts.len(), "tets": r.n_tet, "dof": r.n_dof, "planform_max_area_mm2": jarambit_g5::loads::TIP_FE_MAX_AREA_MM2},
                "field": {"quantity": "von Mises at the vertices (largest corner value), MPa", "readout_mm": [win.0, win.1],
                          "peak_mpa": r.nodal_max_peak.mpa, "peak_at_mm": r.nodal_max_peak.at, "peak_distance_mm": r.nodal_max_peak.dist,
                          "peak_vertex_raw_mpa": fld.vm_max[pv], "allowable_mpa": r.sallow_joint_mpa},
                "solver": {"rel_res": r.rel_res, "backward_error": r.backward_error, "direct": r.ksp_its == 1},
                "required_sf": required(case), "scored_margin": margins.get(case),
            });
            write_fig(out, case, svg, merge(j, sf_json(*sf, sf_rec)), ctx)
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn tip_svg(
    r: &SolidTipResult,
    title: &str,
    load: &str,
    sf: f64,
    sf_rec: f64,
    case: &str,
    win: (f64, f64),
    blade: &BladeProfile2D,
    margins: &BTreeMap<&str, f64>,
) -> String {
    let f = &r.fields;
    let tip = blade.tip_xy;
    let ml = jarambit_g5::loads::Midline::of(blade);
    let (_, axis) = ml.at(0.0);
    let an = axis.0.hypot(axis.1).max(1e-12);
    let (dv, nv) = ((axis.0 / an, axis.1 / an), (-axis.1 / an, axis.0 / an));
    let local = |p: [f64; 3]| -> [f64; 3] {
        let (rx, ry) = (p[0] - tip.0, p[1] - tip.1);
        [rx * dv.0 + ry * dv.1, rx * nv.0 + ry * nv.1, p[2]]
    };
    let zoom = (win.1 + 4.0).min(30.0);
    let umax = f.u.iter().zip(&f.pts).filter(|(_, p)| local(**p)[0] <= zoom).map(|(u, _)| (u[0] * u[0] + u[1] * u[1] + u[2] * u[2]).sqrt()).fold(0.0, f64::max);
    let k = deform_scale(umax, zoom, 0.06);
    let th = 50f64.to_radians();
    let proj = |v: usize, deformed: bool| -> (P2, f64) {
        let kk = if deformed { k } else { 0.0 };
        let p = f.pts[v];
        let q = [p[0] + kk * f.u[v][0], p[1] + kk * f.u[v][1], p[2] + kk * f.u[v][2]];
        let l = local(q);
        // the point on the left, looking down on the blade face, tilted 50 degrees about the bisector
        ((l[0], l[1] * th.cos() + l[2] * th.sin()), l[1] * th.sin() - l[2] * th.cos())
    };
    // boundary faces of the tets
    let mut faces: HashMap<[usize; 3], (usize, [usize; 3])> = HashMap::new();
    for t in &f.tets {
        for fc in [[t[0], t[1], t[2]], [t[0], t[1], t[3]], [t[0], t[2], t[3]], [t[1], t[2], t[3]]] {
            let mut key = fc;
            key.sort_unstable();
            faces.entry(key).and_modify(|e| e.0 += 1).or_insert((1, fc));
        }
    }
    let mut surf: Vec<[usize; 3]> = faces.into_values().filter(|e| e.0 == 1).map(|e| e.1).collect();
    surf.sort_unstable();
    let shown: Vec<[usize; 3]> = surf.into_iter().filter(|fc| fc.iter().all(|&v| local(f.pts[v])[0] <= zoom)).collect();
    let fit = Fit::new(shown.iter().flat_map(|fc| fc.iter().map(|&v| proj(v, true).0)), PANEL, 8.0);
    let max_read = (0..f.pts.len()).filter(|&v| f.read[v]).map(|v| f.vm_max[v]).fold(0.0, f64::max);
    let bands = Bands::new(max_read);
    let saturated = shown.iter().any(|fc| fc.iter().map(|&v| f.vm_max[v]).sum::<f64>() / 3.0 > bands.top());
    let t: Vec<(f64, [P2; 3], String)> = shown
        .iter()
        .map(|fc| {
            let pr: Vec<(P2, f64)> = fc.iter().map(|&v| proj(v, true)).collect();
            let val = fc.iter().map(|&v| f.vm_max[v]).sum::<f64>() / 3.0;
            ((pr[0].1 + pr[1].1 + pr[2].1) / 3.0, [fit.at(pr[0].0), fit.at(pr[1].0), fit.at(pr[2].0)], bands.class(val))
        })
        .collect();
    let scored = margins.get(case).map(|m| format!(" · margin {m:.3}")).unwrap_or_default();
    let mut o = open(W, H, &format!("{title}: SF {sf:.3}"), &format!("{title}. {load}. Solid SF {sf:.3}, required {}.", required(case)));
    header(&mut o, &format!("{title} · SF {sf:.3} (required {}){scored}", required(case)), load);
    tris(&mut o, t, true);
    let pv = f.peak_vertex;
    peak(&mut o, fit.at(proj(pv, true).0), &format!("peak {:.0} MPa at {:.2} mm, allowable {:.0}", r.nodal_max_peak.mpa, r.nodal_max_peak.dist, r.sallow_joint_mpa));
    // the scale: 1 mm along the bisector
    let (a0, a1) = (fit.at((0.0, -0.0)), fit.at((1.0, 0.0)));
    let ys = PANEL.1 + 6.0;
    let _ = writeln!(o, "<path class=\"ax\" d=\"M{:.1} {ys:.1}H{:.1}\" stroke-width=\"2\"/><text class=\"tk\" x=\"{:.1}\" y=\"{:.1}\">1 mm</text>", PANEL.0 + 8.0, PANEL.0 + 8.0 + (a1.0 - a0.0), PANEL.0 + 14.0 + (a1.0 - a0.0), ys + 4.0);
    colour_bar(&mut o, &bands, "von Mises, MPa", saturated);
    footer(
        &mut o,
        &[
            format!("TET10 solid, {} tets, {} DOF · first {zoom:.1} mm of the point, tilted 50° about the bisector · deformation ×{}", r.n_tet, r.n_dof, fmt_k(k)),
            format!("Colour scale tops at the read-out peak ({:.1}-{:.1} mm from the point); stresses at the loaded patch run above it.", win.0, win.1),
            match_note(sf, sf_rec),
        ],
    );
    o + "</svg>\n"
}

// ---------------- ring ----------------

/// The governing direction of the any-direction sweep: the lowest capacity.
pub fn ring_governing(f: &RingFields) -> &RingCaseField {
    f.cases
        .iter()
        .filter(|c| c.name.starts_with("any_"))
        .min_by(|a, b| a.result.capacity_n.total_cmp(&b.result.capacity_n))
        .unwrap_or(&f.cases[0])
}

pub fn ring_figures(ctx: &Ctx, out: &Path, f: &RingFields, sum: &RingBandSummary, rec: &G5Loads, margins: &BTreeMap<&str, f64>) -> Vec<Value> {
    let ring = &rs_v5().loads.ring;
    let gov = ring_governing(f);
    let any_deg = gov.theta.unwrap_or(0.0).to_degrees();
    [
        ("ring_bearing", &f.cases[0], ring.bearing_n, sum.bearing.capacity_n / ring.bearing_n, rec.ring_bearing_sf,
         "Ring, bearing toward the handle", format!("{} N on the bore, ±{}° band facing the handle, pushing +x", ring.bearing_n, ring.bearing_arc_deg)),
        ("ring_back", &f.cases[1], ring.back_n, sum.back.capacity_n / ring.back_n, rec.ring_back_sf,
         "Ring, back", format!("{} N on the outer rear of the ring, pushing +x", ring.back_n)),
        ("ring_any_direction", gov, ring.any_direction_n, sum.any_direction_min_capacity_n / ring.any_direction_n, rec.ring_any_direction_sf,
         "Ring, any in-plane direction", format!("{} N on the bore, the band swept every 30°; governing direction {any_deg:.0}° shown", ring.any_direction_n)),
    ]
    .into_iter()
    .map(|(case, c, load_n, sf, sf_rec, title, load)| {
        let svg = ring_svg(f, c, load_n, title, &load, sf, sf_rec, case, margins, None);
        let j = json!({
            "case": case, "title": title, "load": load, "model": "T6 membrane of the ring and neck, per-Gauss-point thickness, seat clamped at x 40-46 mm",
            "mesh": {"nodes": f.xy.len(), "elements": f.tris.len()},
            "field": {"quantity": "von Mises at the corner nodes at the case load, MPa", "peak_mpa": c.result.sigma_pk_per_n * load_n,
                      "peak_at_mm": c.result.at, "capacity_n": c.result.capacity_n},
            "sweep": if case == "ring_any_direction" { json!(f.cases.iter().filter(|c| c.name.starts_with("any_")).map(|c| json!({"direction_deg": c.theta.map(f64::to_degrees), "capacity_n": c.result.capacity_n})).collect::<Vec<_>>()) } else { Value::Null },
            "required_sf": required(case), "scored_margin": margins.get(case),
        });
        write_fig(out, case, svg, merge(j, sf_json(sf, sf_rec)), ctx)
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn ring_svg(
    f: &RingFields,
    c: &RingCaseField,
    load_n: f64,
    title: &str,
    load: &str,
    sf: f64,
    sf_rec: f64,
    case: &str,
    margins: &BTreeMap<&str, f64>,
    shared: Option<(&Bands, (f64, f64, f64, f64))>,
) -> String {
    let body = ring_body(f, c, load_n, shared.map(|s| s.1).unwrap_or(PANEL), shared.map(|s| s.0));
    let scored = margins.get(case).map(|m| format!(" · margin {m:.3}")).unwrap_or_default();
    let mut o = open(W, H, &format!("{title}: SF {sf:.3}"), &format!("{title}. {load}. SF {sf:.3}, required {}.", required(case)));
    header(&mut o, &format!("{title} · SF {sf:.3} (required {}){scored}", required(case)), load);
    o += &body.0;
    colour_bar(&mut o, &body.1, "von Mises, MPa", body.2);
    footer(
        &mut o,
        &[
            format!("T6 ring and neck membrane, {} elements · seat x 40-46 mm clamped · deformation ×{} · line: undeformed", f.tris.len(), fmt_k(body.3)),
            "Grey: not read (within 3 mm of the seat). The capacity is the as-built wall allowable over the peak per newton.".to_string(),
            match_note(sf, sf_rec),
        ],
    );
    o + "</svg>\n"
}

/// The ring field in a panel: (svg, bands, saturated, deformation scale).
fn ring_body(f: &RingFields, c: &RingCaseField, load_n: f64, panel: (f64, f64, f64, f64), bands_in: Option<&Bands>) -> (String, Bands, bool, f64) {
    let nc = f.n_corner;
    let xs: Vec<f64> = f.xy[..nc].iter().map(|p| p.0).collect();
    let span = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max) - xs.iter().copied().fold(f64::INFINITY, f64::min);
    let umax = (0..nc).map(|i| (c.u[2 * i] * load_n).hypot(c.u[2 * i + 1] * load_n)).fold(0.0, f64::max);
    let k = deform_scale(umax, span, 0.03);
    let at = |i: usize, kk: f64| -> P2 { (f.xy[i].0 + kk * load_n * c.u[2 * i], f.xy[i].1 + kk * load_n * c.u[2 * i + 1]) };
    let fit = Fit::new((0..nc).flat_map(|i| [at(i, k), at(i, 0.0)]), panel, 6.0);
    let vm = |i: usize| c.vm_corner[i] * load_n;
    let max_read = (0..nc).filter(|&i| c.read[i]).map(vm).fold(0.0, f64::max);
    let bands = bands_in.map(|b| Bands { step: b.step, n: b.n }).unwrap_or_else(|| Bands::new(max_read));
    let mut sat = false;
    let t: Vec<(f64, [P2; 3], String)> = f
        .tris
        .iter()
        .map(|t6| {
            let ids = [t6[0], t6[1], t6[2]];
            let v = ids.iter().map(|&i| vm(i)).sum::<f64>() / 3.0;
            sat |= v > bands.top();
            let cls = if ids.iter().all(|&i| c.read[i]) { bands.class(v) } else { "nr".to_string() };
            (0.0, [fit.at(at(ids[0], k)), fit.at(at(ids[1], k)), fit.at(at(ids[2], k))], cls)
        })
        .collect();
    let mut o = String::new();
    tris(&mut o, t, false);
    let corner_tris: Vec<[usize; 3]> = f.tris.iter().map(|t| [t[0], t[1], t[2]]).collect();
    outline(&mut o, &corner_tris, &|i| fit.at(at(i, 0.0)));
    // load arrow at the bore (bearing cases) or at the rear (ring back)
    let pk = fit.at(c.result.at.into());
    let pk_label = format!("peak {:.0} MPa", c.result.sigma_pk_per_n * load_n);
    let mut s = String::new();
    peak(&mut s, pk, &pk_label);
    o += &s;
    if let Some(th) = c.theta {
        let ctr = (f.cx, 0.0);
        let r_b = jarambit_g5::g5_genome::FIXED_BORE_D_MM * 0.5;
        let p0 = fit.at((ctr.0 + 0.2 * r_b * th.cos(), ctr.1 + 0.2 * r_b * th.sin()));
        let p1 = fit.at((ctr.0 + 0.85 * r_b * th.cos(), ctr.1 + 0.85 * r_b * th.sin()));
        arrow(&mut o, p0, p1, &format!("{load_n:.0} N"));
    }
    (o, bands, sat, k)
}

fn arrow(o: &mut String, p0: P2, p1: P2, label: &str) {
    let (dx, dy) = (p1.0 - p0.0, p1.1 - p0.1);
    let l = dx.hypot(dy).max(1e-9);
    let (ux, uy) = (dx / l, dy / l);
    let (hx, hy) = (p1.0 - 9.0 * ux, p1.1 - 9.0 * uy);
    // label at the middle of the shaft: above a horizontal arrow, to the right of a vertical one
    let (mx, my) = (0.5 * (p0.0 + p1.0), 0.5 * (p0.1 + p1.1));
    let (lx, ly) = if ux.abs() > 0.5 { (mx, my - 8.0) } else { (mx + 26.0, my + 4.0) };
    let _ = writeln!(
        o,
        "<path class=\"ar\" d=\"M{:.1} {:.1}L{hx:.1} {hy:.1}\" fill=\"none\"/><path class=\"ar\" d=\"M{:.1} {:.1}L{:.1} {:.1}L{:.1} {:.1}Z\"/>\
         <text class=\"v\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
        p0.0, p0.1, p1.0, p1.1, hx - 4.0 * uy, hy + 4.0 * ux, hx + 4.0 * uy, hy - 4.0 * ux, lx, ly, esc(label)
    );
}

// ---------------- sequence ----------------

pub fn sequence_figure(
    ctx: &Ctx,
    out: &Path,
    ring_f: &RingFields,
    plate_f: &PlateFields,
    loads: &G5Loads,
    rec: &G5Loads,
    margins: &BTreeMap<&str, f64>,
) -> Value {
    let ring = &rs_v5().loads.ring;
    let ld = &rs_v5().loads;
    let seq = [("lateral", loads.seq_lateral, rec.seq_lateral), ("twist", loads.seq_twist, rec.seq_twist), ("pull", loads.seq_pull, rec.seq_pull)];
    let (gname, gval, grec) = seq.iter().copied().min_by(|a, b| a.1.total_cmp(&b.1)).unwrap();
    let gov = ring_governing(ring_f);
    let pull = plate_f.cases.iter().find(|c| c.name == "pull").unwrap();
    let ring_hand = gname == "pull" && loads.ring_any_direction_sf <= loads.grip_pull_sf;
    // shared colour scale over both fields' read regions
    let ring_max = (0..ring_f.n_corner).filter(|&i| gov.read[i]).map(|i| gov.vm_corner[i] * ring.any_direction_n).fold(0.0, f64::max);
    let pull_max = (0..pull.sigma.len()).filter(|&e| pull.read[e]).map(|e| pull.sigma[e]).fold(0.0, f64::max);
    let bands = Bands::new(ring_max.max(pull_max));
    let left = (PANEL.0, PANEL.1 + 20.0, 0.5 * (PANEL.0 + PANEL.2) - 8.0, PANEL.3);
    let right = (0.5 * (PANEL.0 + PANEL.2) + 8.0, PANEL.1 + 20.0, PANEL.2, PANEL.3);
    let title = "Blade before hand (failure sequence)";
    let mut o = open(W, H, title, &format!("Sequence ratios: lateral {:.3}, twist {:.3}, pull {:.3}; the lowest, {gname}, governs.", loads.seq_lateral, loads.seq_twist, loads.seq_pull));
    let hand_sf = if gname == "pull" { loads.grip_pull_sf.min(loads.ring_any_direction_sf) } else if gname == "lateral" { loads.grip_lateral_sf } else { loads.grip_twist_sf };
    let blade_sf = match gname { "pull" => loads.pull_sf, "lateral" => loads.lateral_limit_sf, _ => loads.twist_limit_sf };
    let scored = margins.get("sequence").map(|m| format!(" · margin {m:.3}")).unwrap_or_default();
    header(
        &mut o,
        &format!("{title} · ratio {gval:.3} (required {}){scored}", required("sequence")),
        &format!("{gname} governs: hand side SF {hand_sf:.2} / blade SF {blade_sf:.2} (lateral {:.2}, twist {:.2}, pull {:.2})", loads.seq_lateral, loads.seq_twist, loads.seq_pull),
    );
    let mut sat = false;
    let mut k_ring = 0.0;
    if ring_hand {
        let (b, _, s, k) = ring_body(ring_f, gov, ring.any_direction_n, left, Some(&bands));
        o += &b;
        sat |= s;
        let _ = writeln!(o, "<text class=\"lb\" x=\"{:.0}\" y=\"{:.0}\">Hand side: ring, {} N at {:.0}° · SF {:.2}</text>", left.0, PANEL.1 + 8.0, ring.any_direction_n, gov.theta.unwrap_or(0.0).to_degrees(), loads.ring_any_direction_sf);
        k_ring = k;
    } else {
        let _ = writeln!(o, "<text class=\"lb\" x=\"{:.0}\" y=\"{:.0}\">Hand side: closed-section grip formula, SF {hand_sf:.2} (no FE field)</text>", left.0, PANEL.1 + 8.0);
    }
    // plate pull on the right, in-plane deformed
    let k_pull = {
        let n = plate_f.xy.len();
        let span = plate_f.xy.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max) - plate_f.xy.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
        let umax = (0..n).map(|i| pull.u[i * 5].hypot(pull.u[i * 5 + 1])).fold(0.0, f64::max);
        let k = deform_scale(umax, span, 0.04);
        let at = |i: usize, kk: f64| (plate_f.xy[i].0 + kk * pull.u[i * 5], plate_f.xy[i].1 + kk * pull.u[i * 5 + 1]);
        let fit = Fit::new((0..n).flat_map(|i| [at(i, k), at(i, 0.0)]), right, 6.0);
        let t: Vec<(f64, [P2; 3], String)> = plate_f
            .tris
            .iter()
            .enumerate()
            .map(|(e, tr)| {
                sat |= pull.read[e] && pull.sigma[e] > bands.top();
                (0.0, [fit.at(at(tr[0], k)), fit.at(at(tr[1], k)), fit.at(at(tr[2], k))], if pull.read[e] { bands.class(pull.sigma[e]) } else { "nr".into() })
            })
            .collect();
        tris(&mut o, t, false);
        outline(&mut o, &plate_f.tris, &|i| fit.at(at(i, 0.0)));
        let _ = writeln!(o, "<text class=\"lb\" x=\"{:.0}\" y=\"{:.0}\">Blade: pull, {} N · SF {:.2}</text>", right.0, PANEL.1 + 8.0, ld.pull.limit_n, loads.pull_sf);
        k
    };
    colour_bar(&mut o, &bands, "von Mises, MPa", sat);
    let ok = gval.to_bits() == grec.to_bits();
    footer(
        &mut o,
        &[
            "The hand side must outlast the blade: ratio = hand-side SF / blade SF under the same load, for lateral, twist and pull.".to_string(),
            format!("One colour scale; grey: not read; deformation ×{} (ring), ×{} (blade). Hand-side pull SF = min(grip formula, ring).", fmt_k(k_ring), fmt_k(k_pull)),
            match_note(gval, grec),
        ],
    );
    let svg = o + "</svg>\n";
    let j = json!({
        "case": "sequence", "title": title, "load": "the lateral, twist and pull limit loads on blade and hand side",
        "ratios": {"lateral": loads.seq_lateral, "twist": loads.seq_twist, "pull": loads.seq_pull, "governing": gname},
        "record_ratios": {"lateral": rec.seq_lateral, "twist": rec.seq_twist, "pull": rec.seq_pull},
        "match_all": seq.iter().all(|s| s.1.to_bits() == s.2.to_bits()),
        "hand_side": if ring_hand { format!("ring any direction {:.0} deg (T6)", gov.theta.unwrap_or(0.0).to_degrees()) } else { "grip formula".into() },
        "required_sf": required("sequence"), "scored_margin": margins.get("sequence"),
        "sf_fem": gval, "sf_record": grec, "match": ok, "gated": true,
    });
    write_fig(out, "sequence", svg, j, ctx)
}

// ---------------- buckling ----------------

#[allow(clippy::too_many_arguments)]
pub fn buckling_figure(ctx: &Ctx, out: &Path, a: &[f64; 7], s0: f64, len: f64, lambda: f64, rec: &G5Loads, margins: &BTreeMap<&str, f64>) -> Value {
    let mode = super::ritz::mode_shape(a, s0, len, 200);
    let wmax = mode.iter().map(|m| m.1.abs()).fold(0.0, f64::max).max(1e-300);
    let sign = if mode[0].1 < 0.0 { -1.0 } else { 1.0 };
    let pts: Vec<P2> = mode.iter().map(|m| (m.0, sign * m.1 / wmax)).collect();
    let title = "Buckling of the point";
    let req = required("buckling");
    let mut o = open(W, H, title, &format!("Rayleigh-Ritz column of the last {len:.0} mm: lambda = P_cr / {} N = {lambda:.3}.", rs_v5().loads.tip.axial_n));
    let scored = margins.get("buckling").map(|m| format!(" · margin {m:.3}")).unwrap_or_default();
    header(
        &mut o,
        &format!("{title} · λ = {lambda:.2} (required {req}){scored}"),
        &format!("P_cr = {:.0} N against the {} N tip axial load; tapered column clamped {len:.0} mm behind the point", lambda * rs_v5().loads.tip.axial_n, rs_v5().loads.tip.axial_n),
    );
    let (x0, y0, x1, y1) = (PANEL.0 + 46.0, PANEL.1 + 16.0, PANEL.2 + 70.0, PANEL.3 - 50.0);
    let x = |s: f64| x0 + (x1 - x0) * s / len;
    let y = |w: f64| y1 - (y1 - y0) * w;
    for k in 0..=4 {
        let v = k as f64 * 0.25;
        let _ = write!(o, "<path class=\"grid\" d=\"M{x0:.1} {:.1}H{x1:.1}\"/><text class=\"tk\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">{v:.2}</text>", y(v), x0 - 8.0, y(v) + 4.0);
    }
    for s in (0..=(len as usize)).step_by(5) {
        let _ = write!(o, "<text class=\"tk\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{s}</text>", x(s as f64), y1 + 18.0);
    }
    let _ = writeln!(o, "<path class=\"ax\" d=\"M{x0:.1} {y1:.1}H{x1:.1}\"/>");
    let _ = writeln!(o, "<text class=\"tk\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">distance from the point, mm (clamped at {len:.0} mm)</text>", 0.5 * (x0 + x1), y1 + 34.0);
    let mut d = String::new();
    for (k, p) in pts.iter().enumerate() {
        let _ = write!(d, "{}{:.1} {:.1}", if k == 0 { "M" } else { "L" }, x(p.0), y(p.1.clamp(-0.05, 1.05)));
    }
    let _ = writeln!(o, "<path class=\"ln\" d=\"{d}\"/>");
    let _ = writeln!(o, "<text class=\"lb\" x=\"{:.1}\" y=\"{:.1}\">first buckling mode: deflection w / w_max</text>", x(0.5) + 10.0, y(1.0) - 8.0);
    footer(
        &mut o,
        &[
            "Rayleigh-Ritz, 7 functions (1 - ξ)^(k+2) that satisfy the clamp; E·I(s) of the decoded sections (minor axis), 400 points.".to_string(),
            "The mode comes from the same eigenproblem as the scored λ (eigenvectors accumulated alongside the Jacobi rotations).".to_string(),
            match_note(lambda, rec.buckling_lambda),
        ],
    );
    let svg = o + "</svg>\n";
    let j = json!({
        "case": "buckling", "title": title, "load": format!("{} N tip axial", rs_v5().loads.tip.axial_n),
        "model": "Rayleigh-Ritz tapered column (7 functions)", "column_mm": [s0, len], "ritz_coefficients": a,
        "mode": pts.iter().step_by(10).map(|p| json!([p.0, p.1])).collect::<Vec<_>>(),
        "required_sf": req, "scored_margin": margins.get("buckling"),
    });
    write_fig(out, "buckling", svg, merge(j, sf_json(lambda, rec.buckling_lambda)), ctx)
}

// ---------------- contact sheet ----------------

pub fn index(ctx: &Ctx, out: &Path, figs: &[Value]) {
    let (cols, tw, th) = (4usize, 360.0, 208.0);
    let rows = figs.len().div_ceil(cols);
    let (w, h) = (24.0 + cols as f64 * (tw + 12.0), 64.0 + rows as f64 * (th + 30.0));
    let mut o = open(w, h, "FEM renderings of every load case", &format!("{} figures of {}", figs.len(), ctx.id));
    header(&mut o, "FEM renderings of every RS-karambit-v5 load case", &format!("{} · Tier 2 at h {} mm · {} figures", ctx.id, ctx.h, figs.len()));
    for (k, f) in figs.iter().enumerate() {
        let (c, r) = (k % cols, k / cols);
        let (x, y) = (24.0 + c as f64 * (tw + 12.0), 64.0 + r as f64 * (th + 30.0));
        let _ = writeln!(o, "<image href=\"{}\" x=\"{x}\" y=\"{y}\" width=\"{tw}\" height=\"{th}\"/>", f["svg"].as_str().unwrap_or(""));
        let _ = writeln!(o, "<text class=\"tk\" x=\"{x}\" y=\"{:.0}\">{}</text>", y + th + 14.0, esc(f["title"].as_str().unwrap_or("")));
    }
    let _ = fs::write(out.join("index.svg"), o + "</svg>\n");
}
