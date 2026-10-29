//! Hex8 voxel FE submodels of the three hand-zone cases: grip squeeze, guard push and the serration teeth.
//!
//! Each case is cut from the design's AM voxel field (`VoxelGrid3D::d_am`: skin, lattice, rib and ports as built) and
//! loaded with its RS-karambit-v5 load as a uniform traction on exposed faces; the SF is the allowable over a von Mises
//! reading of a read region (`voxel::Solution::reading`). The geometry of the three submodels is that of the v6.0.0
//! cross-check (`crates/g5/src/bin/g5_fem_fields/hex8.rs`; its crop, face and support helpers are copied). What the
//! v6.0.0 check fixed, an `FeDefinition` makes a choice: which grip stations and teeth, the guard clamp depth, the
//! stress reading and the tooth model's voxel size. `FeDefinition::v6_cross_check` reproduces the v6.0.0 values.
//! Every failure is an error, which a scored case turns into SF 0 (fail closed).

use super::voxel::{solve_wrench, Face, Grid, Material, Model, Options, Problem, Reading, P3};
use crate::hand::{teeth_census, Tooth};
use crate::sdf::blade_outer_field_at;
use jarambit_g5::g5_sdf::{eval_grip_slice, eval_guard_lobe_y, BladeProfile2D, VoxelGrid3D, GUARD_X_MM};
use jarambit_g5::material::{self, AS_BUILT_WALL, GROUND_WALL};
use jarambit_g5::requirements::rs_v5;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::Instant;

type Params = BTreeMap<&'static str, f64>;

/// The choices that define the three FE cases.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeDefinition {
    /// Read stress: von Mises exceeded by this volume (mm³) of the read region; `None`, by 0.1 % of it (v6.0.0)
    pub stress_volume_mm3: Option<f64>,
    /// Solve all three grip stations (25, 50, 75 % of the grip); false: only the squeeze formula's governing one
    pub grip_all_stations: bool,
    /// Guard clamp: every node more than this far above the grip underside is fixed (mm; 4 in v6.0.0)
    pub guard_clamp_mm: f64,
    /// Teeth solved: the k weakest by the notched-root formula on each edge; `None`, only its governing tooth
    pub teeth_per_edge: Option<usize>,
    /// Voxel size of a local tooth model built from `sdf::blade_outer_field_at` (mm); `None`, crop the design grid
    pub teeth_h_mm: Option<f64>,
    pub threads: usize,
}

impl FeDefinition {
    /// The v6.0.0 hex8 cross-check: governing station and tooth, 4 mm guard clamp, 99.9 % quantile, design grid.
    pub fn v6_cross_check(threads: usize) -> Self {
        Self {
            stress_volume_mm3: None,
            grip_all_stations: false,
            guard_clamp_mm: 4.0,
            teeth_per_edge: None,
            teeth_h_mm: None,
            threads,
        }
    }
}

/// One solved submodel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeSolve {
    pub label: String,
    pub sf: f64,
    pub allowable_mpa: f64,
    pub reading: ReadingJson,
    pub h_mm: f64,
    pub voxels: usize,
    pub loaded_faces: usize,
    pub fixed_nodes: usize,
    pub unsupported_voxels: usize,
    pub iterations: usize,
    pub relative_residual: f64,
    pub seconds: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadingJson {
    pub stress_mpa: f64,
    pub volume_mm3: f64,
    pub elements: usize,
    pub max_mpa: f64,
    pub q99_mpa: f64,
    pub q999_mpa: f64,
}

impl From<&Reading> for ReadingJson {
    fn from(r: &Reading) -> Self {
        Self {
            stress_mpa: r.stress_mpa,
            volume_mm3: r.volume_mm3,
            elements: r.elements,
            max_mpa: r.max_mpa,
            q99_mpa: r.q99_mpa,
            q999_mpa: r.q999_mpa,
        }
    }
}

/// A case: the minimum SF over its solves.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeCase {
    pub sf: f64,
    pub solves: Vec<FeSolve>,
}

impl FeCase {
    fn of(solves: Vec<FeSolve>) -> Self {
        Self { sf: solves.iter().map(|s| s.sf).fold(f64::INFINITY, f64::min), solves }
    }
    pub fn seconds(&self) -> f64 {
        self.solves.iter().map(|s| s.seconds).sum()
    }
}

fn ti() -> Material {
    Material { youngs_mpa: material::e_mpa(), poisson: material::nu() }
}

/// The AM voxels of `grid` in the box `x0..x1`, `y0..y1` (all z), cropped to their solid bounding box (v6.0.0).
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
/// last solid element in the outward direction, kept when its centre passes `keep` (v6.0.0).
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

/// Solve one submodel for the force `f` (N) over `loaded` (moment about the faces' mean centre zero, as v6.0.0) and
/// read it.
#[allow(clippy::too_many_arguments)]
fn solve_and_read(
    label: String,
    model: &Model,
    fixed: Vec<[bool; 3]>,
    loaded: Vec<Face>,
    f: [f64; 3],
    allowable_mpa: f64,
    read: &dyn Fn(P3) -> bool,
    def: &FeDefinition,
) -> Result<FeSolve, String> {
    let g = model.grid.clone();
    let nl = loaded.len();
    if nl == 0 {
        return Err(format!("{label}: no loaded faces"));
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
    let opts = Options { relative_tolerance: 1e-8, max_iterations: 20_000, threads: def.threads, keep_displacements: false };
    let t = Instant::now();
    let sol = solve_wrench(model, &problem, [f[0], f[1], f[2], 0.0, 0.0, 0.0], &opts).map_err(|e| format!("{label}: {e}"))?;
    let seconds = t.elapsed().as_secs_f64();
    let r = sol.reading(read, def.stress_volume_mm3).map_err(|e| format!("{label}: {e}"))?;
    Ok(FeSolve {
        label,
        sf: allowable_mpa / r.stress_mpa,
        allowable_mpa,
        reading: (&r).into(),
        h_mm: g.h,
        voxels: sol.elements.len(),
        loaded_faces: nl,
        fixed_nodes,
        unsupported_voxels: sol.unsupported.len(),
        iterations: sol.convergence.iterations,
        relative_residual: sol.convergence.relative_residual,
        seconds,
    })
}

// ---------------- grip squeeze ----------------

/// A grip squeeze station of the analytic model (`loads::analytic_loads`, `grip_section`): x, pad force, section
/// mid-height and height, and the panel/ovalisation SF of G5.0.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GripStation {
    pub s_frac: f64,
    pub x_mm: f64,
    pub pad_force_n: f64,
    pub ym_mm: f64,
    pub h_mm: f64,
    pub t_mm: f64,
    pub sf_g50: f64,
}

/// The three squeeze stations of a hollow grip (`None` for a solid grip, which has no squeeze case).
pub fn grip_stations(p: &Params) -> Option<[GripStation; 3]> {
    let ld = &rs_v5().loads;
    let skin = p["skin_t"];
    if p["rho_grip"] >= 0.95 {
        return None;
    }
    let a_grip = material::yield_allowable(skin, AS_BUILT_WALL);
    Some([0.25, 0.50, 0.75].map(|s_frac| {
        let x = p["x_t"] + s_frac * p["L_g"];
        let gc = eval_grip_slice(p, x);
        let h = (gc.yt - gc.yb).max(8.0);
        let t = gc.t_sec.max(4.0);
        let f_pad = ld.grip.normal_sum_factor * ld.grip.limit_squeeze_n / 4.0;
        let q = f_pad / (20.0 * (0.5 * h.min(30.0)).max(5.0));
        let panel = 0.75 * q * p["lat_cell"].powi(2) / (skin * skin);
        let r_m = 0.125 * (h + t);
        let oval = 6.0 * 0.318 * (f_pad / 20.0) * r_m / (skin * skin);
        GripStation { s_frac, x_mm: x, pad_force_n: f_pad, ym_mm: gc.ym, h_mm: h, t_mm: t, sf_g50: a_grip / panel.max(oval).max(1e-9) }
    }))
}

/// One station: a 28 mm grip slice, the finger pad (20 mm along x, the middle half of the height) on the -z face,
/// the palm on the +z face in the same window as a frictionless z support, two palm nodes against rigid motion; read
/// within 8 mm of the station, off the supports (v6.0.0).
fn grip_station_fe(grid: &VoxelGrid3D, p: &Params, st: &GripStation, def: &FeDefinition) -> Result<FeSolve, String> {
    let label = format!("grip squeeze at {:.0} % of the grip (x {:.1} mm)", 100.0 * st.s_frac, st.x_mm);
    let (xs, ym, h) = (st.x_mm, st.ym_mm, st.h_mm);
    let model = crop(grid, xs - 14.0, xs + 14.0, f64::NEG_INFINITY, f64::INFINITY).ok_or(format!("{label}: empty crop"))?;
    let win = |c: P3| (c[0] - xs).abs() <= 10.0 && (c[1] - ym).abs() <= 0.25 * h;
    let pad = exposed(&model, 2, false, &win);
    let palm = exposed(&model, 2, true, &win);
    let g = model.grid.clone();
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
    match (nearest([xs, ym, 0.0]), nearest([xs + 8.0, ym, 0.0])) {
        (Some(a), Some(b)) => {
            fixed[a] = [true; 3];
            fixed[b][1] = true;
        }
        _ => return Err(format!("{label}: no palm support")),
    }
    let mut touches = vec![false; g.elements()];
    for (e, t) in touches.iter_mut().enumerate() {
        *t = (0..8).any(|a| fixed[g.corner(e, a)].iter().any(|&f| f));
    }
    let h_vox = g.h;
    let origin = g.origin;
    let n = g.n;
    let read = move |c: P3| {
        let ijk: [usize; 3] = std::array::from_fn(|a| ((c[a] - origin[a]) / h_vox - 0.5).round() as usize);
        let e = ijk[0] + n[0] * (ijk[1] + n[1] * ijk[2]);
        (c[0] - xs).abs() <= 8.0 && !touches[e]
    };
    let allow = material::yield_allowable(p["skin_t"], AS_BUILT_WALL);
    solve_and_read(label, &model, fixed, pad, [0.0, 0.0, st.pad_force_n], allow, &read, def)
}

/// Grip squeeze: `None` for a solid grip; otherwise the stations of `def` (all, or the formula's governing one).
pub fn grip_squeeze_fe(p: &Params, grid: &VoxelGrid3D, def: &FeDefinition) -> Result<Option<FeCase>, String> {
    let Some(st) = grip_stations(p) else { return Ok(None) };
    let chosen: Vec<&GripStation> = if def.grip_all_stations {
        st.iter().collect()
    } else {
        // the first station with the lowest formula SF, as the v6.0.0 cross-check chose it
        let mut gov = &st[0];
        for s in &st[1..] {
            if s.sf_g50 < gov.sf_g50 {
                gov = s;
            }
        }
        vec![gov]
    };
    let solves = chosen.into_iter().map(|s| grip_station_fe(grid, p, s, def)).collect::<Result<Vec<_>, _>>()?;
    Ok(Some(FeCase::of(solves)))
}

// ---------------- guard push ----------------

/// Guard push: the guard lobe and the grip above it, clamped `def.guard_clamp_mm` above the grip underside; 300 N
/// forward (+x) on the lobe's rear faces in a 4 mm band at 60 % of its height; read between the band and the clamp
/// (2.5 mm clear of the band, 2 mm clear of the clamp) (v6.0.0 at a 4 mm clamp).
pub fn guard_push_fe(p: &Params, grid: &VoxelGrid3D, def: &FeDefinition) -> Result<FeCase, String> {
    let label = format!("guard push (clamp {} mm into the grip)", def.guard_clamp_mm);
    let h_g = p["h_g"];
    let yb_ref = eval_grip_slice(p, GUARD_X_MM - 6.0).yb;
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
        return Err(format!("{label}: no guard lobe"));
    }
    let y_clamp = yb_ref + def.guard_clamp_mm;
    let model = crop(grid, x_lo - 6.0, x_hi + 6.0, yb_ref - h_g - 3.0, y_clamp + 2.0).ok_or(format!("{label}: empty crop"))?;
    let g = model.grid.clone();
    let y_load = yb_ref - 0.6 * h_g;
    let pad = exposed(&model, 0, false, &|c| (c[1] - y_load).abs() <= 2.0 && c[1] < yb_ref - 0.5);
    let fixed = node_fixed(&g, (0..g.nodes()).filter(|&v| g.node_position(v)[1] >= y_clamp));
    let f = rs_v5().loads.guard.limit_push_n;
    let read = |c: P3| c[1] > y_load + 2.5 && c[1] < y_clamp - 2.0;
    let allow = material::yield_allowable(if p["rho_guard"] < 0.95 { p["skin_t"] } else { p["t_guard"] }, AS_BUILT_WALL);
    Ok(FeCase::of(vec![solve_and_read(label, &model, fixed, pad, [f, 0.0, 0.0], allow, &read, def)?]))
}

// ---------------- teeth ----------------

/// A local model of the blade between `x0..x1`, `y0..y1` at voxel size `h`, from the G5.1 outer field (exact in the
/// pure blade zone, x >= 92 mm; solid within 3.55 mm of an edge, where the AM field equals it).
fn local_blade_model(p: &Params, blade: &BladeProfile2D, x0: f64, x1: f64, y0: f64, y1: f64, h: f64) -> Option<Model> {
    if x0 < 92.0 || x1 > blade.x_tip {
        return None;
    }
    let z_max = 0.5 * p["t_core"].max(1.0) + p["ridge_amp"] + 0.6;
    let n = [((x1 - x0) / h).ceil() as usize, ((y1 - y0) / h).ceil() as usize, ((2.0 * z_max) / h).ceil() as usize];
    let g = Grid { n, h, origin: [x0, y0, -z_max] };
    let phase: Vec<u8> = (0..g.elements())
        .map(|e| {
            let c = g.element_center(e);
            u8::from(blade_outer_field_at(p, blade, c[0], c[1], c[2]).is_some_and(|d| d < 0.0))
        })
        .collect();
    if phase.iter().all(|&v| v == 0) {
        return None;
    }
    Some(Model { grid: g, phase, materials: vec![ti()] })
}

/// One tooth: the tooth and 4 mm of blade behind it (clamped there), the load on the outward faces within 0.6 mm (x)
/// and 0.8 mm (y) of the crest: 180 N snag along the edge, and separately the per-tooth pry across the blade; read
/// the tooth and its root between the gullets, clear of the crest and 1.5 mm clear of the clamp (v6.0.0).
fn tooth_fe(p: &Params, blade: &BladeProfile2D, grid: &VoxelGrid3D, t: &Tooth, def: &FeDefinition) -> Result<FeSolve, String> {
    let edge = if t.inner { &blade.y_in } else { &blade.y_sp };
    let xs = &blade.xs;
    let (xa, xb) = (xs[t.gullet_a], xs[t.gullet_b]);
    let label = format!("{} tooth x {xa:.1}-{xb:.1} mm", if t.inner { "inner" } else { "spine" });
    let seg = &edge[t.gullet_a..=t.gullet_b];
    let (ylo, yhi) = (seg.iter().copied().fold(f64::INFINITY, f64::min), seg.iter().copied().fold(f64::NEG_INFINITY, f64::max));
    let (y0, y1, y_clamp) = if t.inner { (ylo - 1.0, yhi + 5.0, yhi + 4.0) } else { (ylo - 5.0, yhi + 1.0, ylo - 4.0) };
    let model = match def.teeth_h_mm {
        Some(h) => local_blade_model(p, blade, xa - 2.0, xb + 2.0, y0, y1, h),
        None => crop(grid, xa - 2.0, xb + 2.0, y0, y1),
    }
    .ok_or(format!("{label}: no model (empty, or a local model outside the pure blade zone)"))?;
    let g = model.grid.clone();
    let (cx, cy) = (xs[t.crest], edge[t.crest]);
    let bl = (xb - xa).hypot(edge[t.gullet_b] - edge[t.gullet_a]).max(1e-9);
    let (ux, uy) = ((xb - xa) / bl, (edge[t.gullet_b] - edge[t.gullet_a]) / bl);
    let pads = exposed(&model, 1, !t.inner, &|c| (c[0] - cx).abs() <= 0.6 && (c[1] - cy).abs() <= 0.8);
    let inner = t.inner;
    let fixed = || node_fixed(&g, (0..g.nodes()).filter(|&v| if inner { g.node_position(v)[1] >= y_clamp } else { g.node_position(v)[1] <= y_clamp }));
    let read = |c: P3| {
        c[0] >= xa - 0.5 && c[0] <= xb + 0.5 && (c[0] - cx).hypot(c[1] - cy) > 0.9 && if inner { c[1] < y_clamp - 1.5 } else { c[1] > y_clamp + 1.5 }
    };
    let allow = material::yield_allowable(1.0, GROUND_WALL);
    let snag_n = rs_v5().loads.teeth.snag_n;
    let pry_n = rs_v5().loads.teeth.pry_n / rs_v5().loads.teeth.pry_teeth_sharing as f64;
    let snag = solve_and_read(format!("{label}, snag"), &model, fixed(), pads.clone(), [snag_n * ux, snag_n * uy, 0.0], allow, &read, def)?;
    let pry = solve_and_read(format!("{label}, pry"), &model, fixed(), pads, [0.0, 0.0, pry_n], allow, &read, def)?;
    // the tooth's SF: allowable over the larger reading, as the v6.0.0 cross-check combined the two loads
    let gov = if snag.reading.stress_mpa >= pry.reading.stress_mpa { snag } else { pry };
    Ok(FeSolve { label: format!("{label} ({})", if gov.label.ends_with("snag") { "snag" } else { "pry" }), ..gov })
}

/// Teeth: `None` when neither edge is serrated; otherwise the teeth of `def` (the formula's governing tooth, or the k
/// weakest per edge).
pub fn teeth_fe(p: &Params, blade: &BladeProfile2D, grid: &VoxelGrid3D, def: &FeDefinition) -> Result<Option<FeCase>, String> {
    let mut chosen: Vec<Tooth> = Vec::new();
    let census: Vec<Vec<Tooth>> = [true, false].iter().map(|&inner| teeth_census(p, blade, inner)).collect();
    if census.iter().all(Vec::is_empty) {
        return Ok(None);
    }
    match def.teeth_per_edge {
        None => {
            // the formula's governing tooth: first minimum over the inner edge, then the spine edge (v6.0.0)
            let mut best: Option<Tooth> = None;
            for t in census.iter().flatten() {
                if best.is_none_or(|b| t.sf < b.sf) {
                    best = Some(*t);
                }
            }
            chosen.extend(best);
        }
        Some(k) => {
            for edge in &census {
                let mut e = edge.clone();
                e.sort_by(|a, b| a.sf.total_cmp(&b.sf).then(a.gullet_a.cmp(&b.gullet_a)));
                chosen.extend(e.into_iter().take(k));
            }
        }
    }
    let solves = chosen.iter().map(|t| tooth_fe(p, blade, grid, t, def)).collect::<Result<Vec<_>, _>>()?;
    Ok(Some(FeCase::of(solves)))
}
