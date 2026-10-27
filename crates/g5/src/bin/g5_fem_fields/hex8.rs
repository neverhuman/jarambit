//! Independent hex8 voxel FE checks of the three cases the score takes from formulas: grip squeeze, guard push and
//! teeth. Each is a submodel cut from the design's own AM voxel field at the ranking grid (`build_g5_sdf(.., true)
//! .d_am`: skin, lattice, rib, ports as built), solved with the `voxel_fe` copied from `g5-gripfe`, and loaded with
//! the RS-karambit-v5 loads as uniform tractions on the exposed faces of a patch. The SF is the formula's allowable
//! over the 99.9 % volume quantile of von Mises in the read region (voxel FE peaks at staircase corners do not
//! converge, `voxel_fe`'s own rule). These are cross-checks of the formulas, reported beside them and NOT gated.
use super::svg::{required, Bands, Ctx, Fit};
use super::voxel_fe::{solve_unit_wrenches, von_mises, Face, Grid, Material, Model, Options, Problem, P3};
use super::Params;
use jarambit_g5::g5_genome::G5Genome;
use jarambit_g5::g5_sdf::{eval_blade_thickness_and_sdf2, eval_grip_slice, eval_guard_lobe_y, serration_gullets, BladeProfile2D, VoxelGrid3D, GUARD_X_MM};
use jarambit_g5::loads::G5Loads;
use jarambit_g5::material::{self, AS_BUILT_WALL, GROUND_WALL};
use jarambit_g5::requirements::rs_v5;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::time::Instant;

fn ti() -> Material {
    Material { youngs_mpa: material::e_mpa(), poisson: material::nu() }
}

/// The AM voxels of `grid` in the box `x0..x1`, `y0..y1` (all z), cropped to their solid bounding box.
fn crop(grid: &VoxelGrid3D, x0: f64, x1: f64, y0: f64, y1: f64) -> Option<Model> {
    let d = grid.d_am.as_deref().unwrap_or(&grid.d);
    let (ny, nz) = (grid.ny, grid.nz);
    let i0 = grid.x.iter().position(|&x| x >= x0)?;
    let i1 = grid.x.iter().rposition(|&x| x <= x1)?;
    let ja = grid.y.iter().position(|&y| y >= y0)?;
    let jb = grid.y.iter().rposition(|&y| y <= y1)?;
    let inside = |i: usize, j: usize, k: usize| d[(i * ny + j) * nz + k] < 0.0;
    let (mut j0, mut j1, mut k0, mut k1) = (ny, 0, nz, 0);
    for i in i0..=i1 {
        for j in ja..=jb {
            for k in 0..nz {
                if inside(i, j, k) {
                    (j0, j1, k0, k1) = (j0.min(j), j1.max(j), k0.min(k), k1.max(k));
                }
            }
        }
    }
    if j0 > j1 || k0 > k1 {
        return None;
    }
    let n = [i1 - i0 + 1, j1 - j0 + 1, k1 - k0 + 1];
    let h = grid.h;
    let g = Grid { n, h, origin: [grid.x[i0] - 0.5 * h, grid.y[j0] - 0.5 * h, grid.z[k0] - 0.5 * h] };
    let phase = (0..g.elements())
        .map(|e| {
            let [a, b, c] = g.element_index(e);
            u8::from(inside(i0 + a, j0 + b, k0 + c))
        })
        .collect();
    Some(Model { grid: g, phase, materials: vec![ti()] })
}

/// Faces of the outer surface whose outward normal is `axis`/`side`: on every grid line along `axis`, the face of the
/// last solid element in the outward direction (so the lattice's internal cavity faces are never loaded), kept when
/// its centre passes `keep`.
fn exposed(m: &Model, axis: usize, side: bool, keep: &dyn Fn(P3) -> bool) -> Vec<Face> {
    let g = &m.grid;
    let (a1, a2) = ((axis + 1) % 3, (axis + 2) % 3);
    let mut out = Vec::new();
    for u in 0..g.n[a1] {
        for v in 0..g.n[a2] {
            let at = |w: usize| {
                let mut ijk = [0; 3];
                ijk[axis] = w;
                ijk[a1] = u;
                ijk[a2] = v;
                g.element(ijk)
            };
            let mut line = 0..g.n[axis];
            let last = if side { line.rev().find(|&w| m.phase[at(w)] != 0) } else { line.find(|&w| m.phase[at(w)] != 0) };
            if let Some(w) = last {
                let element = at(w);
                let mut c = g.element_center(element);
                c[axis] += if side { 0.5 } else { -0.5 } * g.h;
                if keep(c) {
                    out.push(Face { element, axis, side });
                }
            }
        }
    }
    out.sort_by_key(|f| f.element);
    out
}

struct Solved {
    model: Model,
    vm: Vec<(usize, f64)>,
    secs: f64,
    loaded: usize,
    fixed_nodes: usize,
    excluded: usize,
    residual: Vec<f64>,
}

/// Solve one submodel for the force `f` (N) spread over `loaded` faces, with `fixed` nodes, and keep the element von
/// Mises of that load.
fn solve(model: Model, fixed: Vec<[bool; 3]>, loaded: Vec<Face>, f: [f64; 3]) -> Result<Solved, String> {
    let g = model.grid.clone();
    let nl = loaded.len();
    if nl == 0 {
        return Err("no loaded faces".into());
    }
    let mut c = [0.0; 3];
    for fc in &loaded {
        let e = g.element_center(fc.element);
        for a in 0..3 {
            c[a] += e[a] / nl as f64;
        }
    }
    let fixed_nodes = fixed.iter().filter(|f| f.iter().any(|&c| c)).count();
    let problem = Problem { fixed, loaded, reference: c };
    let opts = Options { relative_tolerance: 1e-8, max_iterations: 20_000, threads: 4, keep_displacements: false };
    let t = Instant::now();
    let sol = solve_unit_wrenches(&model, &problem, &opts)?;
    let w = [f[0], f[1], f[2], 0.0, 0.0, 0.0];
    let vm = sol.elements.iter().enumerate().map(|(i, &e)| (e, von_mises(sol.stress_of(w, i)))).collect();
    Ok(Solved {
        model,
        vm,
        secs: t.elapsed().as_secs_f64(),
        loaded: nl,
        fixed_nodes,
        excluded: sol.exclusion.unsupported_elements,
        residual: sol.convergence.iter().map(|c| c.relative_residual).collect(),
    })
}

fn quantiles(mut v: Vec<f64>) -> Option<(f64, f64, f64, usize)> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(f64::total_cmp);
    let q = |p: f64| v[((p * v.len() as f64).ceil() as usize).clamp(1, v.len()) - 1];
    Some((q(0.99), q(0.999), *v.last().unwrap(), v.len()))
}

fn node_fixed(g: &Grid, nodes: impl Iterator<Item = usize>) -> Vec<[bool; 3]> {
    let mut f = vec![[false; 3]; g.nodes()];
    for v in nodes {
        f[v] = [true; 3];
    }
    f
}

/// Corner nodes of the given faces.
fn face_nodes(g: &Grid, faces: &[Face]) -> Vec<usize> {
    let mut v = Vec::new();
    for f in faces {
        for a in 0..8 {
            if ((a >> f.axis) & 1 == 1) == f.side {
                v.push(g.corner(f.element, a));
            }
        }
    }
    v.sort_unstable();
    v.dedup();
    v
}

/// Planform map (max over z of the read elements' von Mises), with the solid footprint grey where nothing is read.
#[allow(clippy::too_many_arguments)]
fn map_svg(s: &Solved, read: &dyn Fn(usize) -> bool, title: &str, sub: &str, notes: &[String], marks: &[((f64, f64), (f64, f64), &str)], peak_q999: f64) -> String {
    let g = &s.model.grid;
    let (nx, ny) = (g.n[0], g.n[1]);
    let mut col = vec![f64::NAN; nx * ny];
    let mut solid = vec![false; nx * ny];
    for &(e, v) in &s.vm {
        let [i, j, _] = g.element_index(e);
        solid[i * ny + j] = true;
        if read(e) {
            let c = &mut col[i * ny + j];
            *c = if c.is_nan() { v } else { c.max(v) };
        }
    }
    let bands = Bands::new(peak_q999);
    let sat = col.iter().any(|&v| v > bands.top());
    let world = |i: f64, j: f64| (g.origin[0] + g.h * i, g.origin[1] + g.h * j);
    let fit = Fit::new([world(0.0, 0.0), world(nx as f64, ny as f64)].into_iter(), (24.0, 64.0, 624.0, 392.0), 8.0);
    let mut o = super::svg::open(760.0, 440.0, title, sub);
    super::svg::header(&mut o, title, sub);
    // rows of equal class merged into rectangles
    let _ = writeln!(o, "<g stroke-width=\"0.3\">");
    for j in 0..ny {
        let mut i = 0;
        while i < nx {
            let cls = |i: usize| -> Option<String> {
                let v = col[i * ny + j];
                if !v.is_nan() {
                    Some(bands.class(v))
                } else if solid[i * ny + j] {
                    Some("nr".to_string())
                } else {
                    None
                }
            };
            let Some(c) = cls(i) else {
                i += 1;
                continue;
            };
            let mut i2 = i + 1;
            while i2 < nx && cls(i2).as_deref() == Some(c.as_str()) {
                i2 += 1;
            }
            let (a, b) = (fit.at(world(i as f64, j as f64 + 1.0)), fit.at(world(i2 as f64, j as f64)));
            let _ = write!(o, "<rect class=\"{c}\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>", a.0, a.1, b.0 - a.0, b.1 - a.1);
            i = i2;
        }
    }
    let _ = writeln!(o, "</g>");
    let (lo, hi) = (world(0.0, 0.0), world(nx as f64, ny as f64));
    let clamp = |p: (f64, f64)| (p.0.clamp(lo.0, hi.0), p.1.clamp(lo.1, hi.1));
    for (p0, p1, label) in marks {
        let (a, b) = (fit.at(clamp(*p0)), fit.at(clamp(*p1)));
        let _ = writeln!(
            o,
            "<rect class=\"pk\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" stroke-dasharray=\"4 3\" stroke-width=\"1.2\"/><text class=\"v\" x=\"{:.1}\" y=\"{:.1}\">{label}</text>",
            a.0.min(b.0), a.1.min(b.1), (b.0 - a.0).abs().max(2.0), (b.1 - a.1).abs().max(2.0), a.0.min(b.0), a.1.min(b.1) - 5.0
        );
    }
    super::svg::colour_bar(&mut o, &bands, "von Mises, MPa", sat);
    super::svg::footer(&mut o, notes);
    o + "</svg>\n"
}

fn write(out: &Path, case: &str, svg: String, mut j: Value, ctx: &Ctx) -> Value {
    fs::write(out.join(format!("{case}.svg")), svg).unwrap_or_else(|e| super::fail(e));
    j["design_id"] = json!(ctx.id);
    j["h_mm"] = json!(ctx.h);
    j["record_sha256"] = json!(ctx.record_sha);
    fs::write(out.join(format!("{case}.json")), serde_json::to_string_pretty(&j).unwrap() + "\n").unwrap_or_else(|e| super::fail(e));
    json!({"case": case, "svg": format!("{case}.svg"), "json": format!("{case}.json"), "title": j["title"],
           "sf": j["sf_fem"], "record_sf": j["sf_record"], "match": Value::Null, "gated": false})
}

fn failed(ctx: &Ctx, out: &Path, case: &str, title: &str, why: String, rec_sf: f64) -> Value {
    eprintln!("g5-fem-fields: hex8 {case}: {why}");
    let mut o = super::svg::open(760.0, 440.0, title, &why);
    super::svg::header(&mut o, title, &format!("hex8 cross-check not available: {why}"));
    let j = json!({"case": case, "title": title, "sf_fem": Value::Null, "sf_record": rec_sf, "sf_formula": rec_sf,
                   "error": why, "gated": false});
    write(out, case, o + "</svg>\n", j, ctx)
}

#[allow(clippy::too_many_arguments)]
pub fn checks(ctx: &Ctx, out: &Path, _g: &G5Genome, p: &Params, b: &BladeProfile2D, grid: &VoxelGrid3D, rec: &G5Loads, m: &BTreeMap<&str, f64>) -> Vec<Value> {
    vec![grip_squeeze(ctx, out, p, grid, rec, m), guard_push(ctx, out, p, grid, rec, m), teeth(ctx, out, p, b, grid, rec, m)]
}

// ---------------- grip squeeze ----------------

/// The squeeze formula of `loads::analytic_loads` at its three stations (the `grip_section` lines copied from
/// `crates/g5/src/loads.rs` at 48d3a8c7): the governing station, its pad force and section, and the SF.
fn squeeze_formula(p: &Params) -> Option<(f64, f64, f64, f64, f64, f64)> {
    let ld = &rs_v5().loads;
    let skin = p["skin_t"];
    let a_grip = material::yield_allowable(skin, AS_BUILT_WALL);
    let hollow = p["rho_grip"] < 0.95;
    if !hollow {
        return None;
    }
    let mut best: Option<(f64, f64, f64, f64, f64, f64)> = None;
    for s_frac in [0.25, 0.50, 0.75] {
        let x = p["x_t"] + s_frac * p["L_g"];
        let gc = eval_grip_slice(p, x);
        let h = (gc.yt - gc.yb).max(8.0);
        let t = gc.t_sec.max(4.0);
        let f_pad = ld.grip.normal_sum_factor * ld.grip.limit_squeeze_n / 4.0;
        let q = f_pad / (20.0 * (0.5 * h.min(30.0)).max(5.0));
        let panel = 0.75 * q * p["lat_cell"].powi(2) / (skin * skin);
        let r_m = 0.125 * (h + t);
        let oval = 6.0 * 0.318 * (f_pad / 20.0) * r_m / (skin * skin);
        let sf = a_grip / panel.max(oval).max(1e-9);
        if best.is_none_or(|b| sf < b.0) {
            best = Some((sf, x, f_pad, gc.ym, h, t));
        }
    }
    best
}

fn grip_squeeze(ctx: &Ctx, out: &Path, p: &Params, grid: &VoxelGrid3D, rec: &G5Loads, m: &BTreeMap<&str, f64>) -> Value {
    let title = "Grip squeeze (hex8 voxel FE cross-check)";
    let Some((sf_f, xs, f_pad, ym, h, _t)) = squeeze_formula(p) else {
        return failed(ctx, out, "grip_squeeze", title, "solid grip: the formula has no squeeze case".into(), rec.grip_squeeze_sf);
    };
    if sf_f.to_bits() != rec.grip_squeeze_sf.to_bits() {
        return failed(ctx, out, "grip_squeeze", title, format!("copied formula {sf_f} does not reproduce the record {}", rec.grip_squeeze_sf), rec.grip_squeeze_sf);
    }
    let Some(model) = crop(grid, xs - 14.0, xs + 14.0, f64::NEG_INFINITY, f64::INFINITY) else {
        return failed(ctx, out, "grip_squeeze", title, "empty crop".into(), rec.grip_squeeze_sf);
    };
    // finger pad on the -z face, palm on the +z face: 20 mm along the grip, over the middle half of its height
    let win = |c: P3| (c[0] - xs).abs() <= 10.0 && (c[1] - ym).abs() <= 0.25 * h;
    let pad = exposed(&model, 2, false, &win);
    let palm = exposed(&model, 2, true, &win);
    let g = model.grid.clone();
    // the palm: a frictionless normal (z) support, so the section is free to ovalise; the in-plane rigid-body motion
    // is stopped at two palm nodes (x and y at the one nearest the pad centre, y at the one nearest 8 mm along x)
    let palm_nodes = face_nodes(&g, &palm);
    let mut fixed = vec![[false; 3]; g.nodes()];
    for &v in &palm_nodes {
        fixed[v][2] = true;
    }
    let nearest = |q: P3| {
        palm_nodes.iter().copied().min_by(|&a, &b| {
            let d = |v: usize| {
                let p = g.node_position(v);
                (p[0] - q[0]).powi(2) + (p[1] - q[1]).powi(2)
            };
            d(a).total_cmp(&d(b))
        })
    };
    if let (Some(a), Some(b)) = (nearest([xs, ym, 0.0]), nearest([xs + 8.0, ym, 0.0])) {
        fixed[a] = [true; 3];
        fixed[b][1] = true;
    }
    let touches_support: Vec<bool> = {
        let mut t = vec![false; g.elements()];
        for e in 0..g.elements() {
            t[e] = (0..8).any(|a| fixed[g.corner(e, a)].iter().any(|&f| f));
        }
        t
    };
    let s = match solve(model, fixed, pad, [0.0, 0.0, f_pad]) {
        Ok(s) => s,
        Err(e) => return failed(ctx, out, "grip_squeeze", title, e, rec.grip_squeeze_sf),
    };
    let read = |e: usize| {
        let c = g.element_center(e);
        (c[0] - xs).abs() <= 8.0 && !touches_support[e]
    };
    let allow = material::yield_allowable(p["skin_t"], AS_BUILT_WALL);
    let Some((q99, q999, mx, n)) = quantiles(s.vm.iter().filter(|(e, _)| read(*e)).map(|v| v.1).collect()) else {
        return failed(ctx, out, "grip_squeeze", title, "nothing to read".into(), rec.grip_squeeze_sf);
    };
    let sf = allow / q999;
    let sub = format!("{f_pad:.0} N finger pad (2.3 × 800 N / 4), 20 mm × h/2 on one face, palm held on the other; station x {xs:.1} mm");
    let svg = map_svg(
        &s,
        &read,
        &format!("Grip squeeze · hex8 SF {sf:.2} vs formula {sf_f:.2} (required {})", required("grip_squeeze")),
        &sub,
        &[
            format!("Hex8 voxel FE on the AM field (h {} mm), a 28 mm grip slice, {} solid voxels · map: max through the thickness", ctx.h, s.vm.len()),
            format!("SF = skin allowable {allow:.0} MPa / 99.9 % quantile {q999:.0} MPa (max {mx:.0}) · grey: not read (beyond ±8 mm, on the palm support)"),
            "Cross-check of the closed-section panel/ovalisation formula, not gated; the score uses the formula.".to_string(),
        ],
        &[((xs - 10.0, ym - 0.25 * h), (xs + 10.0, ym + 0.25 * h), "pad")],
        q999,
    );
    let j = json!({
        "case": "grip_squeeze", "title": "Grip squeeze", "gated": false,
        "sf_fem": sf, "sf_formula": sf_f, "sf_record": rec.grip_squeeze_sf, "fem_over_formula": sf / sf_f,
        "model": "hex8 voxel FE (voxel_fe, copied from g5-gripfe) on the design's AM voxel field; 28 mm grip slice at the formula's governing station",
        "load": {"pad_force_n": f_pad, "pad": "20 mm along x, |y - ym| <= h/4, the outer -z surface, uniform traction +z", "palm": "the outer +z surface in the same window, normal (z) support only; two nodes also hold x, y against rigid-body motion"},
        "station_x_mm": xs, "solid_voxels": s.vm.len(), "loaded_faces": s.loaded, "fixed_nodes": s.fixed_nodes, "unsupported_voxels": s.excluded,
        "vm_mpa": {"q99": q99, "q999": q999, "max": mx, "read_elements": n}, "allowable_mpa": allow,
        "solver": {"seconds": s.secs, "relative_residuals": s.residual},
        "required_sf": required("grip_squeeze"), "scored_margin": m.get("grip_squeeze"),
        "note": "Cross-check, not gated: the score uses the formula SF (sf_formula = sf_record). The voxel SF uses the 99.9 % von Mises quantile; voxel peaks at staircase corners do not converge.",
    });
    write(out, "grip_squeeze", svg, j, ctx)
}

// ---------------- guard push ----------------

fn guard_push(ctx: &Ctx, out: &Path, p: &Params, grid: &VoxelGrid3D, rec: &G5Loads, m: &BTreeMap<&str, f64>) -> Value {
    let title = "Guard push (hex8 voxel FE cross-check)";
    let sf_f = jarambit_g5::loads::guard_push_sf(p);
    if sf_f.to_bits() != rec.guard_push_sf.to_bits() {
        return failed(ctx, out, "guard_push", title, format!("guard_push_sf {sf_f} does not reproduce the record {}", rec.guard_push_sf), rec.guard_push_sf);
    }
    let h_g = p["h_g"];
    let yb_ref = eval_grip_slice(p, GUARD_X_MM - 6.0).yb;
    // the lobe's x extent (as `guard_push_sf` scans it) and where it is at least 5 % deep, for the crop
    let (mut x_lo, mut x_hi) = (f64::INFINITY, f64::NEG_INFINITY);
    let mut x = 67.6;
    while x <= 99.0 {
        if yb_ref - eval_guard_lobe_y(p, x).0 >= 0.05 * h_g {
            x_lo = x_lo.min(x);
            x_hi = x_hi.max(x);
        }
        x += 0.25;
    }
    if !(x_hi > x_lo) {
        return failed(ctx, out, "guard_push", title, "no guard lobe".into(), rec.guard_push_sf);
    }
    let Some(model) = crop(grid, x_lo - 6.0, x_hi + 6.0, yb_ref - h_g - 3.0, yb_ref + 6.0) else {
        return failed(ctx, out, "guard_push", title, "empty crop".into(), rec.guard_push_sf);
    };
    let g = model.grid.clone();
    let y_load = yb_ref - 0.6 * h_g;
    // the hand slides forward (+x) onto the lobe's rear face (outward normal -x), on a 4 mm band at 60 % of h_g
    let pad = exposed(&model, 0, false, &|c| (c[1] - y_load).abs() <= 2.0 && c[1] < yb_ref - 0.5);
    let y_clamp = yb_ref + 4.0;
    let fixed = node_fixed(&g, (0..g.nodes()).filter(|&v| g.node_position(v)[1] >= y_clamp));
    let f = rs_v5().loads.guard.limit_push_n;
    let band_x = pad.iter().map(|fc| g.element_center(fc.element)[0]).fold((f64::INFINITY, f64::NEG_INFINITY), |a, x| (a.0.min(x), a.1.max(x)));
    let s = match solve(model, fixed, pad, [f, 0.0, 0.0]) {
        Ok(s) => s,
        Err(e) => return failed(ctx, out, "guard_push", title, e, rec.guard_push_sf),
    };
    // read the lobe and its root, clear of the load band and the clamp
    let read = |e: usize| {
        let c = g.element_center(e);
        c[1] > y_load + 2.5 && c[1] < y_clamp - 2.0
    };
    let rho = p["rho_guard"];
    let allow = material::yield_allowable(if rho < 0.95 { p["skin_t"] } else { p["t_guard"] }, AS_BUILT_WALL);
    let Some((q99, q999, mx, n)) = quantiles(s.vm.iter().filter(|(e, _)| read(*e)).map(|v| v.1).collect()) else {
        return failed(ctx, out, "guard_push", title, "nothing to read".into(), rec.guard_push_sf);
    };
    let sf = allow / q999;
    let svg = map_svg(
        &s,
        &read,
        &format!("Guard push · hex8 SF {sf:.2} vs formula {sf_f:.2} (required {})", required("guard_push")),
        &format!("{f:.0} N forward (+x) on the lobe's rear face, a 4 mm band at 60 % of h_g = {h_g:.1} mm below the grip"),
        &[
            format!("Hex8 voxel FE on the AM field (h {} mm), {} solid voxels, clamped {:.0} mm inside the grip · map: max through the thickness", ctx.h, s.vm.len(), y_clamp - yb_ref),
            format!("SF = allowable {allow:.0} MPa / 99.9 % quantile {q999:.0} MPa (max {mx:.0}) · grey: not read (load band, clamp)"),
            "Cross-check of the root-section cantilever formula, not gated; the score uses the formula.".to_string(),
        ],
        &[((band_x.0 - 0.5, y_load - 2.0), (band_x.1 + 0.5, y_load + 2.0), "push")],
        q999,
    );
    let j = json!({
        "case": "guard_push", "title": "Guard push", "gated": false,
        "sf_fem": sf, "sf_formula": sf_f, "sf_record": rec.guard_push_sf, "fem_over_formula": sf / sf_f,
        "model": "hex8 voxel FE (voxel_fe) on the design's AM voxel field; the guard lobe and the grip above it, clamped 4 mm above the grip underside",
        "load": {"force_n": f, "direction": "+x", "band": "rear (-x facing) faces within 2 mm of 60 % of h_g below the grip underside"},
        "lobe_x_mm": [x_lo, x_hi], "solid_voxels": s.vm.len(), "loaded_faces": s.loaded, "fixed_nodes": s.fixed_nodes, "unsupported_voxels": s.excluded,
        "vm_mpa": {"q99": q99, "q999": q999, "max": mx, "read_elements": n}, "allowable_mpa": allow,
        "solver": {"seconds": s.secs, "relative_residuals": s.residual},
        "required_sf": required("guard_push"), "scored_margin": m.get("guard_push"),
        "note": "Cross-check, not gated: the score uses the formula SF (sf_formula = sf_record).",
    });
    write(out, "guard_push", svg, j, ctx)
}

// ---------------- teeth ----------------

/// The governing tooth of `g5_qd::serration_tooth_strength` (its loop copied from `crates/g5/src/g5_qd.rs` at
/// 48d3a8c7): (sf, inner edge?, gullet a, gullet b, tooth height, crest index).
fn governing_tooth(p: &Params, blade: &BladeProfile2D) -> Option<(f64, bool, usize, usize, f64, usize)> {
    let mut best: Option<(f64, bool, usize, usize, f64, usize)> = None;
    for inner in [true, false] {
        let (edge, smooth, sign) = if inner { (&blade.y_in, &blade.y_in_smooth, 1.0) } else { (&blade.y_sp, &blade.y_sp_smooth, -1.0) };
        let xs = &blade.xs;
        let n = xs.len();
        let (depth, gullets) = serration_gullets(edge, smooth);
        if depth < 0.2 {
            continue;
        }
        let allow = material::yield_allowable(1.0, GROUND_WALL);
        let at = |x: f64, y: f64| eval_blade_thickness_and_sdf2(p, blade, x, y);
        let interp = |v: &[f64], x: f64| -> f64 {
            let i = xs.partition_point(|&q| q < x).clamp(1, n - 1);
            let f = ((x - xs[i - 1]) / (xs[i] - xs[i - 1]).max(1e-12)).clamp(0.0, 1.0);
            v[i - 1] * (1.0 - f) + v[i] * f
        };
        let mut stations = gullets.clone();
        if rs_v5().serration.check_end_teeth && gullets.len() >= 2 {
            let span = (gullets[gullets.len() - 1] - gullets[0]) / (gullets.len() - 1);
            stations.insert(0, gullets[0].saturating_sub(span));
            stations.push((gullets[gullets.len() - 1] + span).min(n - 1));
        }
        let (snag_n, pry_n) = (jarambit_g5::g5_qd::tooth_snag_n(), jarambit_g5::g5_qd::tooth_pry_per_tooth_n());
        for w in stations.windows(2) {
            let (ga, gb) = (w[0], w[1]);
            let (pa, pb) = ((xs[ga], edge[ga]), (xs[gb], edge[gb]));
            let bl = (pb.0 - pa.0).hypot(pb.1 - pa.1);
            let (ux, uy) = ((pb.0 - pa.0) / bl.max(1e-9), (pb.1 - pa.1) / bl.max(1e-9));
            let (mut h_t, mut crest) = (0.0_f64, ga);
            for k in ga..=gb {
                let d = ((xs[k] - pa.0) * -uy + (edge[k] - pa.1) * ux).abs();
                if d > h_t {
                    h_t = d;
                    crest = k;
                }
            }
            if bl < 0.3 || h_t < 0.2 {
                continue;
            }
            let (nx, ny) = (-uy * sign, ux * sign);
            let mm = 40usize;
            let (mut a0, mut a1, mut a2, mut i_op, mut t_max) = (0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64);
            let ds = bl / mm as f64;
            for j in 0..mm {
                let sj = (j as f64 + 0.5) * ds;
                let (x, y) = (pa.0 + ux * sj + 0.05 * nx, pa.1 + uy * sj + 0.05 * ny);
                let (t, d2) = at(x, y);
                let t = if d2 <= 0.0 { t.max(0.0) } else { 0.0 };
                a0 += t * ds;
                a1 += t * sj * ds;
                a2 += t * sj * sj * ds;
                i_op += t.powi(3) / 12.0 * ds;
                t_max = t_max.max(t);
            }
            let sf = if a0 <= 1e-9 || i_op <= 1e-12 {
                0.0
            } else {
                let s_bar = a1 / a0;
                let i_ip = (a2 - a0 * s_bar * s_bar).max(1e-12);
                let c = s_bar.max(bl - s_bar);
                let rho_at = |g: usize| -> f64 {
                    let (x1, x2, x3) = (xs[g] - 0.6, xs[g], xs[g] + 0.6);
                    let (y1, y2, y3) = (interp(edge, x1), edge[g], interp(edge, x3));
                    let (a, bb, cc) = ((x2 - x1).hypot(y2 - y1), (x3 - x2).hypot(y3 - y2), (x3 - x1).hypot(y3 - y1));
                    let area2 = ((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)).abs();
                    if area2 < 1e-12 { 1e3 } else { a * bb * cc / (2.0 * area2) }
                };
                let is_gullet = |g: usize| gullets.contains(&g);
                let rho_g = |g: usize| if is_gullet(g) { rho_at(g) } else { 1e3 };
                let rho = rho_g(ga).min(rho_g(gb)).max(0.02);
                let kt = 1.0 + 2.0 * (h_t / rho).sqrt();
                let sig_snag = kt * snag_n * h_t * c / i_ip;
                let sig_pry = pry_n * h_t * (0.5 * t_max) / i_op;
                allow / sig_snag.max(sig_pry).max(1e-9)
            };
            if best.is_none_or(|b| sf < b.0) {
                best = Some((sf, inner, ga, gb, h_t, crest));
            }
        }
    }
    best
}

fn teeth(ctx: &Ctx, out: &Path, p: &Params, blade: &BladeProfile2D, grid: &VoxelGrid3D, rec: &G5Loads, m: &BTreeMap<&str, f64>) -> Value {
    let title = "Teeth (hex8 voxel FE cross-check)";
    let Some((sf_f, inner, ga, gb, h_t, crest)) = governing_tooth(p, blade) else {
        return failed(ctx, out, "teeth", title, "no serration teeth".into(), rec.teeth_sf);
    };
    if sf_f.to_bits() != rec.teeth_sf.to_bits() {
        return failed(ctx, out, "teeth", title, format!("copied tooth loop {sf_f} does not reproduce the record {}", rec.teeth_sf), rec.teeth_sf);
    }
    let (xs, edge) = (&blade.xs, if inner { &blade.y_in } else { &blade.y_sp });
    let (xa, xb) = (xs[ga], xs[gb]);
    let (ylo, yhi) = (edge[ga..=gb].iter().copied().fold(f64::INFINITY, f64::min), edge[ga..=gb].iter().copied().fold(f64::NEG_INFINITY, f64::max));
    // the blade lies on +y of the inner edge and on -y of the spine edge
    let (y0, y1, y_clamp_in) = if inner { (ylo - 1.0, yhi + 5.0, yhi + 4.0) } else { (ylo - 5.0, yhi + 1.0, ylo - 4.0) };
    let Some(model) = crop(grid, xa - 2.0, xb + 2.0, y0, y1) else {
        return failed(ctx, out, "teeth", title, "empty crop".into(), rec.teeth_sf);
    };
    let g = model.grid.clone();
    let (cx, cy) = (xs[crest], edge[crest]);
    let bl = (xb - xa).hypot(edge[gb] - edge[ga]).max(1e-9);
    let (ux, uy) = ((xb - xa) / bl, (edge[gb] - edge[ga]) / bl);
    // the crest: the outward-facing (y) faces within 0.6 mm of the crest point
    let crest_faces = |m: &Model| exposed(m, 1, !inner, &|c| (c[0] - cx).abs() <= 0.6 && (c[1] - cy).abs() <= 0.8);
    let fixed_of = || node_fixed(&g, (0..g.nodes()).filter(|&v| if inner { g.node_position(v)[1] >= y_clamp_in } else { g.node_position(v)[1] <= y_clamp_in }));
    let snag_n = rs_v5().loads.teeth.snag_n;
    let pry_n = rs_v5().loads.teeth.pry_n / rs_v5().loads.teeth.pry_teeth_sharing as f64;
    let pads = crest_faces(&model);
    let s_snag = match solve(model.clone(), fixed_of(), pads.clone(), [snag_n * ux, snag_n * uy, 0.0]) {
        Ok(s) => s,
        Err(e) => return failed(ctx, out, "teeth", title, e, rec.teeth_sf),
    };
    let s_pry = match solve(model, fixed_of(), pads, [0.0, 0.0, pry_n]) {
        Ok(s) => s,
        Err(e) => return failed(ctx, out, "teeth", title, e, rec.teeth_sf),
    };
    // read the tooth and its root between the gullets, clear of the loaded crest
    let read = |e: usize| {
        let c = g.element_center(e);
        c[0] >= xa - 0.5 && c[0] <= xb + 0.5 && (c[0] - cx).hypot(c[1] - cy) > 0.9 && if inner { c[1] < y_clamp_in - 1.5 } else { c[1] > y_clamp_in + 1.5 }
    };
    let allow = material::yield_allowable(1.0, GROUND_WALL);
    let qs = |s: &Solved| quantiles(s.vm.iter().filter(|(e, _)| read(*e)).map(|v| v.1).collect());
    let (Some(qn), Some(qp)) = (qs(&s_snag), qs(&s_pry)) else {
        return failed(ctx, out, "teeth", title, "nothing to read".into(), rec.teeth_sf);
    };
    let sf = allow / qn.1.max(qp.1);
    let gov = if qn.1 >= qp.1 { &s_snag } else { &s_pry };
    let svg = map_svg(
        gov,
        &read,
        &format!("Teeth · hex8 SF {sf:.2} vs formula {sf_f:.2} (required {})", required("teeth")),
        &format!("governing tooth on the {} edge, x {xa:.1}-{xb:.1} mm, height {h_t:.2} mm: {snag_n:.0} N snag along the edge, {pry_n:.0} N pry", if inner { "inner" } else { "spine" }),
        &[
            format!("Hex8 voxel FE on the AM field (h {} mm), the tooth and 4 mm of blade (clamped), {} voxels · shown: {}", ctx.h, gov.vm.len(), if qn.1 >= qp.1 { "snag" } else { "pry" }),
            format!("SF = allowable {allow:.0} MPa / 99.9 % quantile (snag {:.0}, pry {:.0} MPa) · grey: not read (crest, clamp)", qn.1, qp.1),
            "Cross-check of the notched root-section formula, not gated; h 0.2 mm voxels resolve the thin bevel coarsely.".to_string(),
        ],
        &[((cx - 0.6, cy - 0.8), (cx + 0.6, cy + 0.8), "load")],
        qn.1.max(qp.1),
    );
    let j = json!({
        "case": "teeth", "title": "Teeth", "gated": false,
        "sf_fem": sf, "sf_formula": sf_f, "sf_record": rec.teeth_sf, "fem_over_formula": sf / sf_f,
        "model": "hex8 voxel FE (voxel_fe) on the design's AM voxel field; the formula's governing tooth with 4 mm of blade behind it, clamped there",
        "tooth": {"edge": if inner { "inner" } else { "spine" }, "x_mm": [xa, xb], "height_mm": h_t, "crest_mm": [cx, cy]},
        "load": {"snag_n": snag_n, "snag_direction": [ux, uy, 0.0], "pry_n": pry_n, "patch": "outward y faces within 0.6 mm (x) and 0.8 mm (y) of the crest"},
        "vm_mpa": {"snag": {"q99": qn.0, "q999": qn.1, "max": qn.2, "read_elements": qn.3}, "pry": {"q99": qp.0, "q999": qp.1, "max": qp.2, "read_elements": qp.3}},
        "allowable_mpa": allow, "solid_voxels": s_snag.vm.len(), "loaded_faces": s_snag.loaded,
        "solver": {"seconds": s_snag.secs + s_pry.secs, "relative_residuals": [s_snag.residual, s_pry.residual]},
        "required_sf": required("teeth"), "scored_margin": m.get("teeth"),
        "note": "Cross-check, not gated: the score uses the formula SF (sf_formula = sf_record).",
    });
    write(out, "teeth", svg, j, ctx)
}
