//! RESEARCH, default OFF: surface-tension smoothing of the weight-saving pocket layer.
//!
//! The exported solid is a height field over a FIXED planform: top/bottom faces z = +-th(x,y)/2
//! with side walls on the outline and every through-hole. Outside the grinds, every weight-saving
//! feature (isogrid pockets, lattice dimples, fuller, flutes, relief, ring flute/groove, texture,
//! P1 pockets) is a removal depth D(x,y) = base(x,y) - th(x,y). Today that depth reaches the CAD
//! through the 0.25 mm raster by nearest-cell lookup, and most pocket walls are vertical steps.
//! A ~0.7 mm CDT sampling a 0.25 mm staircase of a step is what renders as chipped, jagged walls.
//!
//! This module never moves the planform (outline, through-holes), the grind faces, the bore
//! collar or the brand pad. Each pocket is a basin a liquid would fill (watershed: 4-connected
//! components of D > D_EPS). Its face z = D/2 then evolves as a liquid surface under surface
//! tension for time T = r^2/2: graph mean-curvature flow
//!     z_t = ((1+z_y^2) z_xx - 2 z_x z_y z_xy + (1+z_x^2) z_yy) / (1 + z_x^2 + z_y^2)
//! plus, per basin, a pressure (Lagrange) term lambda_b |grad D| that keeps the basin's removed
//! volume (the oil volume) constant. On steep walls the flow moves the pocket outline by its
//! curvature (chips and spikes, which have the highest curvature, go first; a circle stays a
//! circle; straight rib edges stay straight; narrow necks pinch or bridge), and it rounds the
//! top and bottom of each wall into a meniscus. On gentle bowls (dimples, fuller) it acts like
//! diffusion, so raster terraces flatten while the bowl keeps its volume.
//!
//! Enabled only by `JARAMBIT_POCKET_SMOOTH`, e.g. `r=1.0,h=0.125,grow=0.5,mesh=0.3,rim=1.0,vp=1`.
//! Unset or empty: no code path changes and exports stay byte-identical.

use crate::genome::*;
use crate::geom::*;
use crate::pheno::Pheno;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::sync::{Arc, Mutex, OnceLock};

pub const ENV: &str = "JARAMBIT_POCKET_SMOOTH";
pub const REPORT_ENV: &str = "JARAMBIT_POCKET_SMOOTH_REPORT";
/// Total removal depth (mm, both faces) below which a cell counts as unpocketed.
pub const D_EPS: f64 = 0.02;

#[derive(Clone, Debug, PartialEq)]
pub struct Params {
    /// Surface-tension length scale (mm): outline features with radius below r are removed.
    pub r: f64,
    /// Flow grid spacing (mm), 0.25 / k for integer k in 1..=8.
    pub h: f64,
    /// Furthest a pocket may spread beyond its original footprint (mm).
    pub grow: f64,
    /// Spacing (mm) of extra CDT vertices in pocket-wall zones of the export mesh (0 = none).
    pub mesh: f64,
    /// Keep-out distance (mm) to the outline and through-holes for cells that were unpocketed.
    pub rim: f64,
    /// Keep each basin's removed volume constant (pressure term).
    pub vp: bool,
}

impl Default for Params {
    fn default() -> Self {
        Params { r: 1.0, h: 0.125, grow: 0.5, mesh: 0.3, rim: 1.0, vp: true }
    }
}

impl Params {
    pub fn parse(s: &str) -> Result<Params, String> {
        let mut p = Params::default();
        for part in s.split(',').map(str::trim).filter(|x| !x.is_empty()) {
            let (k, v) = part
                .split_once('=')
                .ok_or_else(|| format!("{ENV}: expected key=value, got {part:?}"))?;
            let x: f64 = v
                .trim()
                .parse()
                .map_err(|_| format!("{ENV}: {k} needs a number, got {v:?}"))?;
            match k.trim() {
                "r" => p.r = x,
                "h" => p.h = x,
                "grow" => p.grow = x,
                "mesh" => p.mesh = x,
                "rim" => p.rim = x,
                "vp" => p.vp = x != 0.0,
                other => return Err(format!("{ENV}: unknown key {other:?} (r, h, grow, mesh, rim, vp)")),
            }
        }
        p.validate()?;
        Ok(p)
    }

    pub fn validate(&self) -> Result<(), String> {
        let k = 0.25 / self.h;
        if !(self.r.is_finite() && (0.0..=5.0).contains(&self.r)) {
            return Err(format!("{ENV}: r must be in [0, 5] mm"));
        }
        if !(self.h.is_finite() && self.h > 0.0 && (k - k.round()).abs() < 1e-9 && (1.0..=8.0).contains(&k.round())) {
            return Err(format!("{ENV}: h must be 0.25/k mm for k in 1..=8"));
        }
        if !(self.grow.is_finite() && (0.0..=5.0).contains(&self.grow)) {
            return Err(format!("{ENV}: grow must be in [0, 5] mm"));
        }
        if !(self.mesh == 0.0 || (self.mesh.is_finite() && (0.15..=2.0).contains(&self.mesh))) {
            return Err(format!("{ENV}: mesh must be 0 or in [0.15, 2] mm"));
        }
        if !(self.rim.is_finite() && (0.0..=5.0).contains(&self.rim)) {
            return Err(format!("{ENV}: rim must be in [0, 5] mm"));
        }
        Ok(())
    }

    pub fn key(&self) -> String {
        format!("r={},h={},grow={},mesh={},rim={},vp={}", self.r, self.h, self.grow, self.mesh, self.rim, u8::from(self.vp))
    }
}

/// Parameters from the environment. Invalid settings abort (fail closed): a requested smoothing
/// must never silently fall back to the unsmoothed CAD.
pub fn params_from_env() -> Option<Params> {
    let s = std::env::var(ENV).ok()?;
    if s.trim().is_empty() {
        return None;
    }
    match Params::parse(&s) {
        Ok(p) => Some(p),
        Err(e) => panic!("{e}"),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Grid {
    pub x0: f64,
    pub y0: f64,
    pub h: f64,
    pub nx: usize,
    pub ny: usize,
}

impl Grid {
    pub fn center(&self, i: usize, j: usize) -> P2 {
        (self.x0 + (i as f64 + 0.5) * self.h, self.y0 + (j as f64 + 0.5) * self.h)
    }
    pub fn len(&self) -> usize {
        self.nx * self.ny
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Cell is outside the planform (mirror boundary).
pub const OUTSIDE: u8 = 0;
/// Cell keeps its original depth (grind, bore collar, pad, rim).
pub const FIXED: u8 = 1;
/// Cell may evolve if it lies within `grow` of a basin.
pub const FREE: u8 = 2;

/// A pure flow problem on a grid, independent of the phenotype (unit-testable).
pub struct Problem {
    pub grid: Grid,
    pub state: Vec<u8>,
    /// Original total removal depth (mm, both faces).
    pub d0: Vec<f64>,
    /// Largest permitted removal depth (minimum web).
    pub dmax: Vec<f64>,
}

pub struct Flow {
    pub d: Vec<f64>,
    /// Basin that owns each evolving cell (0 = not evolving).
    pub owner: Vec<u32>,
    pub basins: usize,
    pub steps: usize,
    pub volume_before_mm3: f64,
    pub volume_after_mm3: f64,
    pub worst_basin_volume_error: f64,
}

/// Watershed basins: pocket cells (d0 > D_EPS) whose 8-connected neighbourhoods, dilated by
/// `merge` mm, touch belong to one basin (nearby drops coalesce; isolated raster specks join the
/// pocket they chip). Only pocket cells receive a label.
fn basins(pb: &Problem, merge: f64) -> (Vec<u32>, usize) {
    let g = pb.grid;
    let n = g.len();
    let pocket = |s: usize| pb.state[s] == FREE && pb.d0[s] > D_EPS;
    let reach = (merge / g.h).ceil().max(0.0) as usize;
    // Dilated mask by BFS steps (Chebyshev) over FREE cells.
    let mut steps = vec![usize::MAX; n];
    let mut queue = VecDeque::new();
    for s in 0..n {
        if pocket(s) {
            steps[s] = 0;
            queue.push_back(s);
        }
    }
    let neighbours = |c: usize| {
        let (i, j) = ((c % g.nx) as isize, (c / g.nx) as isize);
        let mut v = [usize::MAX; 8];
        let mut k = 0;
        for (di, dj) in [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)] {
            let (a, b) = (i + di, j + dj);
            if a >= 0 && b >= 0 && a < g.nx as isize && b < g.ny as isize {
                v[k] = b as usize * g.nx + a as usize;
            }
            k += 1;
        }
        v
    };
    while let Some(c) = queue.pop_front() {
        if steps[c] >= reach {
            continue;
        }
        for t in neighbours(c) {
            if t != usize::MAX && pb.state[t] == FREE && steps[t] == usize::MAX {
                steps[t] = steps[c] + 1;
                queue.push_back(t);
            }
        }
    }
    let mut comp = vec![0u32; n];
    let mut count = 0u32;
    for s in 0..n {
        if steps[s] == usize::MAX || comp[s] != 0 {
            continue;
        }
        count += 1;
        comp[s] = count;
        queue.push_back(s);
        while let Some(c) = queue.pop_front() {
            for t in neighbours(c) {
                if t != usize::MAX && steps[t] != usize::MAX && comp[t] == 0 {
                    comp[t] = count;
                    queue.push_back(t);
                }
            }
        }
    }
    // Renumber so that only components holding pocket cells count, in first-seen order.
    let mut map = vec![0u32; count as usize + 1];
    let mut used = 0u32;
    let mut label = vec![0u32; n];
    for s in 0..n {
        if pocket(s) {
            let c = comp[s] as usize;
            if map[c] == 0 {
                used += 1;
                map[c] = used;
            }
            label[s] = map[c];
        }
    }
    (label, used as usize)
}

/// Nearest basin (8-neighbour Dijkstra over FREE cells) within `limit` mm of a basin.
fn owners(pb: &Problem, label: &[u32], limit: f64) -> Vec<u32> {
    let g = pb.grid;
    let n = g.len();
    let mut dist = vec![u64::MAX; n];
    let mut owner = vec![0u32; n];
    let scale = 1e6 / g.h;
    let lim = (limit * scale).round() as u64;
    let mut heap = BinaryHeap::new();
    for s in 0..n {
        if label[s] != 0 {
            dist[s] = 0;
            owner[s] = label[s];
            heap.push(Reverse((0u64, s)));
        }
    }
    let diag = (std::f64::consts::SQRT_2 * 1e6).round() as u64;
    let step = 1_000_000u64;
    while let Some(Reverse((d, s))) = heap.pop() {
        if d > dist[s] {
            continue;
        }
        let (i, j) = ((s % g.nx) as isize, (s / g.nx) as isize);
        for (di, dj, w) in [(-1, 0, step), (1, 0, step), (0, -1, step), (0, 1, step), (-1, -1, diag), (1, -1, diag), (-1, 1, diag), (1, 1, diag)] {
            let (a, b) = (i + di, j + dj);
            if a < 0 || b < 0 || a >= g.nx as isize || b >= g.ny as isize {
                continue;
            }
            let t = b as usize * g.nx + a as usize;
            if pb.state[t] != FREE {
                continue;
            }
            let nd = d + w;
            if nd <= lim && (nd < dist[t] || (nd == dist[t] && owner[s] < owner[t])) {
                dist[t] = nd;
                owner[t] = owner[s];
                heap.push(Reverse((nd, t)));
            }
        }
    }
    owner
}

/// Neighbour value with a mirror (zero-flux) boundary at the grid edge and outside the planform.
fn at(pb: &Problem, d: &[f64], s: usize, i: isize, j: isize) -> f64 {
    let g = pb.grid;
    if i < 0 || j < 0 || i >= g.nx as isize || j >= g.ny as isize {
        return d[s];
    }
    let t = j as usize * g.nx + i as usize;
    if pb.state[t] == OUTSIDE { d[s] } else { d[t] }
}

/// Volume-preserving graph mean-curvature flow of the removal depth for time r^2/2.
pub fn evolve(pb: &Problem, r: f64, grow: f64, vp: bool) -> Flow {
    let g = pb.grid;
    let (nx, h) = (g.nx, g.h);
    let n = g.len();
    assert!(pb.state.len() == n && pb.d0.len() == n && pb.dmax.len() == n, "problem arrays must match the grid");
    let (label, nb) = basins(pb, (0.5 * r).min(0.5).max(2.0 * pb.grid.h));
    let owner = owners(pb, &label, grow + 1e-9);
    let active: Vec<usize> = (0..n).filter(|&s| owner[s] != 0).collect();
    let mut d = pb.d0.clone();
    let mut target = vec![0.0f64; nb + 1];
    for &s in &active {
        target[owner[s] as usize] += pb.d0[s];
    }
    let volume_before: f64 = target.iter().sum::<f64>() * h * h;
    let t_end = r * r / 2.0;
    let steps = if r > 0.0 { (t_end / (0.2 * h * h)).ceil() as usize } else { 0 };
    let dt = if steps > 0 { t_end / steps as f64 } else { 0.0 };
    let mut vel = vec![0.0f64; n];
    let mut gp = vec![0.0f64; n];
    let mut gm = vec![0.0f64; n];
    let correction_passes = if vp { 40 } else { 0 };
    for it in 0..steps + correction_passes {
        let flowing = it < steps;
        let mut cur = vec![0.0f64; nb + 1];
        let mut add = vec![0.0f64; nb + 1];
        let mut sum_gp = vec![0.0f64; nb + 1];
        let mut sum_gm = vec![0.0f64; nb + 1];
        for &s in &active {
            let (i, j) = ((s % nx) as isize, (s / nx) as isize);
            let c = d[s];
            let e = at(pb, &d, s, i + 1, j);
            let w = at(pb, &d, s, i - 1, j);
            let no = at(pb, &d, s, i, j + 1);
            let so = at(pb, &d, s, i, j - 1);
            if flowing {
                let ne = at(pb, &d, s, i + 1, j + 1);
                let nw = at(pb, &d, s, i - 1, j + 1);
                let se = at(pb, &d, s, i + 1, j - 1);
                let sw = at(pb, &d, s, i - 1, j - 1);
                // z = d/2 is one face's depth; d_t = 2 z_t has the same diffusion coefficients.
                let zx = (e - w) / (4.0 * h);
                let zy = (no - so) / (4.0 * h);
                let dxx = (e - 2.0 * c + w) / (h * h);
                let dyy = (no - 2.0 * c + so) / (h * h);
                let dxy = (ne - nw - se + sw) / (4.0 * h * h);
                vel[s] = ((1.0 + zy * zy) * dxx - 2.0 * zx * zy * dxy + (1.0 + zx * zx) * dyy) / (1.0 + zx * zx + zy * zy);
            } else {
                vel[s] = 0.0;
            }
            let (dxm, dxp, dym, dyp) = ((c - w) / h, (e - c) / h, (c - so) / h, (no - c) / h);
            gp[s] = (dxm.max(0.0).powi(2) + dxp.min(0.0).powi(2) + dym.max(0.0).powi(2) + dyp.min(0.0).powi(2)).sqrt();
            gm[s] = (dxm.min(0.0).powi(2) + dxp.max(0.0).powi(2) + dym.min(0.0).powi(2) + dyp.max(0.0).powi(2)).sqrt();
            let b = owner[s] as usize;
            cur[b] += c;
            add[b] += vel[s] * dt;
            sum_gp[b] += gp[s];
            sum_gm[b] += gm[s];
        }
        let mut lambda = vec![0.0f64; nb + 1];
        if vp {
            for b in 1..=nb {
                let resid = target[b] - (cur[b] + add[b]);
                let gsum = if resid > 0.0 { sum_gm[b] } else { sum_gp[b] };
                if gsum > 1e-12 {
                    lambda[b] = (resid / gsum).clamp(-0.5 * h, 0.5 * h);
                }
            }
        }
        let next: Vec<f64> = active
            .iter()
            .map(|&s| {
                let b = owner[s] as usize;
                let lam = lambda[b];
                let grad = if lam > 0.0 { gm[s] } else { gp[s] };
                (d[s] + vel[s] * dt + lam * grad).clamp(0.0, pb.dmax[s].max(0.0))
            })
            .collect();
        for (k, &s) in active.iter().enumerate() {
            d[s] = next[k];
        }
    }
    let mut after = vec![0.0f64; nb + 1];
    for &s in &active {
        after[owner[s] as usize] += d[s];
    }
    let worst = (1..=nb)
        .filter(|&b| target[b] > 1e-9)
        .map(|b| ((after[b] - target[b]) / target[b]).abs())
        .fold(0.0f64, f64::max);
    Flow {
        volume_after_mm3: after.iter().sum::<f64>() * h * h,
        d,
        owner,
        basins: nb,
        steps,
        volume_before_mm3: volume_before,
        worst_basin_volume_error: worst,
    }
}

/// Smoothed removal-depth field consumed by `export::thickness_at` and `export::solid`.
pub struct SmoothField {
    pub params: Params,
    pub grid: Grid,
    /// Cells whose depth the flow owns; elsewhere the original raster value is used unchanged.
    pub active: Vec<bool>,
    /// Smoothed depth on evolving cells, original depth on fixed cells, NaN outside.
    pub d: Vec<f32>,
    /// Original depth (NaN outside), retained for metrics.
    pub d0: Vec<f32>,
    /// Extra CDT vertices in pocket-wall zones.
    pub steiner: Vec<P2>,
    pub report: serde_json::Value,
}

impl std::fmt::Debug for SmoothField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SmoothField")
            .field("params", &self.params)
            .field("grid", &self.grid)
            .field("steiner", &self.steiner.len())
            .finish()
    }
}

impl SmoothField {
    fn cell(&self, q: P2) -> Option<usize> {
        let g = &self.grid;
        let i = ((q.0 - g.x0) / g.h).floor();
        let j = ((q.1 - g.y0) / g.h).floor();
        if i < 0.0 || j < 0.0 || i >= g.nx as f64 || j >= g.ny as f64 {
            return None;
        }
        Some(j as usize * g.nx + i as usize)
    }

    /// Bilinear depth on the cell-centre lattice; an outside neighbour takes the containing value.
    fn bilinear(&self, q: P2, home: usize) -> f64 {
        let g = &self.grid;
        let fx = ((q.0 - g.x0) / g.h - 0.5).clamp(0.0, (g.nx - 1) as f64);
        let fy = ((q.1 - g.y0) / g.h - 0.5).clamp(0.0, (g.ny - 1) as f64);
        let i0 = (fx.floor() as usize).min(g.nx.saturating_sub(2));
        let j0 = (fy.floor() as usize).min(g.ny.saturating_sub(2));
        let (tx, ty) = ((fx - i0 as f64).clamp(0.0, 1.0), (fy - j0 as f64).clamp(0.0, 1.0));
        let v = |i: usize, j: usize| {
            let x = self.d[j * g.nx + i];
            if x.is_nan() { self.d[home] as f64 } else { x as f64 }
        };
        let a = v(i0, j0) * (1.0 - tx) + v(i0 + 1, j0) * tx;
        let b = v(i0, j0 + 1) * (1.0 - tx) + v(i0 + 1, j0 + 1) * tx;
        a * (1.0 - ty) + b * ty
    }

    /// Smoothed total removal depth at q when q lies in an evolving cell; otherwise None.
    pub fn depth_at(&self, q: P2) -> Option<f64> {
        let s = self.cell(q)?;
        if !self.active[s] {
            return None;
        }
        Some(self.bilinear(q, s).max(0.0))
    }
}

fn base_and_grind(p: &Pheno, q: P2) -> (f64, f64) {
    // Same expressions as export::thickness_at.
    let g = &p.genome;
    let xt = g.v[TAPER_X0] + p.dx;
    let xmax = p.surface_x_max;
    let stock = g.knot_t(q.0.min(xt), xt);
    let base = stock - (stock - g.v[T_TIP]) * ((q.0 - xt) / (xmax - xt)).clamp(0.0, 1.0);
    let ti = g.v[EA_IN].to_radians().tan();
    let ts = g.v[EA_SP].to_radians().tan();
    let di = p.cut_in.nearest(q, ((base - g.v[EDGE_TH]) / (2.0 * ti)).max(0.0));
    let ds = p.cut_sp.nearest(q, ((base - g.v[EDGE_TH]) / (2.0 * ts)).max(0.0));
    let grind = (g.v[EDGE_TH] + 2.0 * di * ti).min(g.v[EDGE_TH] + 2.0 * ds * ts);
    (base, grind)
}

/// Build the smoothing problem from the phenotype's raster (before any rs_v2 replacement).
pub fn problem(p: &Pheno, prm: &Params) -> Problem {
    let f = &p.field;
    let k = (f.g / prm.h).round() as usize;
    let grid = Grid { x0: f.x0, y0: f.y0, h: f.g / k as f64, nx: f.nx * k, ny: f.ny * k };
    let (rb, _) = crate::pheno::bore_dimensions(p);
    let bore_r = p.ring_id / 2.0 + 1.5 * rb + 0.5;
    let web_min = (0.3 * p.t).max(1.5);
    let mut bpts = densify(&p.outline, 0.25, true);
    for hole in &p.holes {
        bpts.extend(densify(hole, 0.25, true));
    }
    let nn_b = Nn::new(&bpts, 2.0);
    let pad = p.pad;
    let in_pad = |q: P2, m: f64| q.0 >= pad[0].0 - m && q.0 <= pad[1].0 + m && q.1 >= pad[0].1 - m && q.1 <= pad[2].1 + m;
    let n = grid.len();
    let (mut state, mut d0, mut dmax) = (vec![OUTSIDE; n], vec![0.0f64; n], vec![0.0f64; n]);
    // The coarse cell's own depth: base and grind at the coarse centre, raw raster value.
    let mut coarse: HashMap<usize, f64> = HashMap::new();
    for j in 0..grid.ny {
        for i in 0..grid.nx {
            let s = j * grid.nx + i;
            let (ci, cj) = (i / k, j / k);
            let ck = cj * f.nx + ci;
            let raw = f.th[ck];
            if raw.is_nan() {
                continue;
            }
            let depth0 = *coarse.entry(ck).or_insert_with(|| {
                let qc = (f.x0 + (ci as f64 + 0.5) * f.g, f.y0 + (cj as f64 + 0.5) * f.g);
                let (bc, gc) = base_and_grind(p, qc);
                if gc < bc - 1e-9 { 0.0 } else { (bc - raw as f64).max(0.0) }
            });
            let depth0 = if depth0 > D_EPS { depth0 } else { 0.0 };
            let q = grid.center(i, j);
            let (base, grind) = base_and_grind(p, q);
            // Raised features (ring collar, bosses) stand above the base face: never evolve them.
            let raised = raw as f64 > base + D_EPS;
            let fixed = grind < base - 1e-9
                || raised
                || q.0.hypot(q.1) <= bore_r
                || in_pad(q, 1.0)
                || (depth0 == 0.0 && nn_b.nearest(q, prm.rim) <= prm.rim);
            state[s] = if fixed { FIXED } else { FREE };
            d0[s] = if grind < base - 1e-9 { 0.0 } else { depth0 };
            dmax[s] = (base - web_min).max(d0[s]).max(0.0);
        }
    }
    Problem { grid, state, d0, dmax }
}

fn steiner_points(sf_grid: Grid, active: &[bool], d: &[f32], state: &[u8], spacing: f64) -> Vec<P2> {
    let g = sf_grid;
    let mut pts = Vec::new();
    if spacing <= 0.0 {
        return pts;
    }
    let near_edge = |i: usize, j: usize| -> bool {
        // within two cells of a non-evolving cell or the grid edge
        for b in j.saturating_sub(2)..=(j + 2).min(g.ny - 1) {
            for a in i.saturating_sub(2)..=(i + 2).min(g.nx - 1) {
                let t = b * g.nx + a;
                if !active[t] || state[t] == OUTSIDE {
                    return true;
                }
            }
        }
        i < 2 || j < 2 || i + 2 >= g.nx || j + 2 >= g.ny
    };
    let (w, hgt) = (g.nx as f64 * g.h, g.ny as f64 * g.h);
    let (ni, nj) = ((w / spacing).floor() as usize, (hgt / (spacing * 0.866)).floor() as usize);
    for b in 0..nj {
        let y = g.y0 + (b as f64 + 0.5) * spacing * 0.866;
        let shift = if b % 2 == 1 { 0.5 * spacing } else { 0.0 };
        for a in 0..ni {
            let x = g.x0 + (a as f64 + 0.25) * spacing + shift;
            let i = ((x - g.x0) / g.h).floor();
            let j = ((y - g.y0) / g.h).floor();
            if i < 1.0 || j < 1.0 || i >= (g.nx - 1) as f64 || j >= (g.ny - 1) as f64 {
                continue;
            }
            let (i, j) = (i as usize, j as usize);
            let s = j * g.nx + i;
            if !active[s] || near_edge(i, j) {
                continue;
            }
            let gx = (d[s + 1] - d[s - 1]) as f64 / (2.0 * g.h);
            let gy = (d[s + g.nx] - d[s - g.nx]) as f64 / (2.0 * g.h);
            // pocket wall: one face slopes faster than 1 in 10
            if gx.hypot(gy) > 0.2 {
                pts.push((x, y));
            }
        }
    }
    pts
}

fn isoline_length(g: Grid, v: &[f32], level: f32) -> f64 {
    // Marching squares on cell centres; NaN corners skip the square.
    let mut len = 0.0;
    for j in 0..g.ny.saturating_sub(1) {
        for i in 0..g.nx.saturating_sub(1) {
            let c = [v[j * g.nx + i], v[j * g.nx + i + 1], v[(j + 1) * g.nx + i + 1], v[(j + 1) * g.nx + i]];
            if c.iter().any(|x| x.is_nan()) {
                continue;
            }
            let pts: Vec<(f64, f64)> = (0..4)
                .filter_map(|e| {
                    let (a, b) = (c[e], c[(e + 1) % 4]);
                    if (a > level) != (b > level) {
                        let t = ((level - a) / (b - a)) as f64;
                        let corner = |k: usize| match k { 0 => (0.0, 0.0), 1 => (1.0, 0.0), 2 => (1.0, 1.0), _ => (0.0, 1.0) };
                        let (p, q) = (corner(e), corner((e + 1) % 4));
                        Some((p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1)))
                    } else {
                        None
                    }
                })
                .collect();
            if pts.len() == 2 {
                len += (pts[0].0 - pts[1].0).hypot(pts[0].1 - pts[1].1) * g.h;
            } else if pts.len() == 4 {
                len += ((pts[0].0 - pts[1].0).hypot(pts[0].1 - pts[1].1) + (pts[2].0 - pts[3].0).hypot(pts[2].1 - pts[3].1)) * g.h;
            }
        }
    }
    len
}

/// Run the flow for one phenotype and package the field for the exporter.
pub fn build(p: &Pheno, prm: &Params) -> SmoothField {
    let started = std::time::Instant::now();
    let pb = problem(p, prm);
    let flow = evolve(&pb, prm.r, prm.grow, prm.vp);
    let n = pb.grid.len();
    let active: Vec<bool> = flow.owner.iter().map(|&o| o != 0).collect();
    let d: Vec<f32> = (0..n)
        .map(|s| if pb.state[s] == OUTSIDE { f32::NAN } else { flow.d[s] as f32 })
        .collect();
    let d0: Vec<f32> = (0..n)
        .map(|s| if pb.state[s] == OUTSIDE { f32::NAN } else { pb.d0[s] as f32 })
        .collect();
    let steiner = steiner_points(pb.grid, &active, &d, &pb.state, prm.mesh);
    let (mut changed, mut max_change, mut sq) = (0usize, 0.0f64, 0.0f64);
    for s in 0..n {
        if active[s] {
            let c = (flow.d[s] - pb.d0[s]).abs();
            if c > 1e-6 {
                changed += 1;
                sq += c * c;
            }
            max_change = max_change.max(c);
        }
    }
    let mut lines = serde_json::Map::new();
    for level in [0.1f32, 0.5, 1.0, 2.0] {
        lines.insert(
            format!("{level}"),
            serde_json::json!({
                "before_mm": isoline_length(pb.grid, &d0, level),
                "after_mm": isoline_length(pb.grid, &d, level),
            }),
        );
    }
    let report = serde_json::json!({
        "schema": "jarambit.pocket_smooth.v1",
        "research_only": true,
        "params": {"r_mm": prm.r, "h_mm": prm.h, "grow_mm": prm.grow, "mesh_mm": prm.mesh, "rim_mm": prm.rim, "volume_preserving": prm.vp},
        "model": "graph mean-curvature flow of the face z = D/2 for T = r^2/2, per-basin volume (pressure) term, planform/grinds/bore/pad fixed",
        "grid": {"x0": pb.grid.x0, "y0": pb.grid.y0, "h": pb.grid.h, "nx": pb.grid.nx, "ny": pb.grid.ny},
        "cells": {
            "inside": pb.state.iter().filter(|&&s| s != OUTSIDE).count(),
            "fixed": pb.state.iter().filter(|&&s| s == FIXED).count(),
            "evolving": active.iter().filter(|&&a| a).count(),
            "changed": changed,
        },
        "basins": flow.basins,
        "steps": flow.steps,
        "removed_volume_mm3": {"before": flow.volume_before_mm3, "after": flow.volume_after_mm3},
        "worst_basin_volume_error": flow.worst_basin_volume_error,
        "depth_change_mm": {"max": max_change, "rms_changed": if changed > 0 { (sq / changed as f64).sqrt() } else { 0.0 }},
        "isoline_length_by_depth_level_mm": lines,
        "steiner_points": steiner.len(),
        "seconds": started.elapsed().as_secs_f64(),
    });
    SmoothField { params: prm.clone(), grid: pb.grid, active, d, d0, steiner, report }
}

/// Per-process cache: the D1, export and probe tools rebuild the same phenotype repeatedly.
pub fn cached(p: &Pheno, prm: &Params) -> Arc<SmoothField> {
    static CACHE: OnceLock<Mutex<HashMap<String, Arc<SmoothField>>>> = OnceLock::new();
    let key = format!("{}|{:?}|{}|{}x{}", prm.key(), p.mat, p.genome.csv_row(), p.field.nx, p.field.ny);
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = cache.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(sf) = map.get(&key) {
        return sf.clone();
    }
    let sf = Arc::new(build(p, prm));
    if let Ok(path) = std::env::var(REPORT_ENV) {
        if !path.trim().is_empty() && !std::path::Path::new(&path).exists() {
            let _ = std::fs::write(&path, serde_json::to_string_pretty(&sf.report).unwrap_or_default() + "\n");
        }
    }
    map.insert(key, sf.clone());
    sf
}

#[cfg(test)]
mod tests {
    use super::*;

    fn disk_problem(radius: f64, depth: f64, rough: f64, h: f64) -> (Problem, f64) {
        let nx = (24.0 / h) as usize;
        let grid = Grid { x0: -12.0, y0: -12.0, h, nx, ny: nx };
        let n = grid.len();
        let (mut state, mut d0) = (vec![FREE; n], vec![0.0; n]);
        for j in 0..nx {
            for i in 0..nx {
                let (x, y) = grid.center(i, j);
                let th = y.atan2(x);
                // chipped rim: 9 sharp lobes plus a high-frequency saw
                let rr = radius + rough * ((9.0 * th).sin().signum() * 0.5 + (37.0 * th).sin());
                if x.hypot(y) <= rr {
                    d0[j * nx + i] = depth;
                }
                if x.abs() > 11.0 || y.abs() > 11.0 {
                    state[j * nx + i] = FIXED;
                }
            }
        }
        let dmax = vec![depth * 1.5; n];
        (Problem { grid, state, d0, dmax }, radius)
    }

    fn radial_spread(pb: &Problem, d: &[f64], level: f64) -> (f64, f64) {
        // RMS deviation of the rim radius from its mean, and the area inside the level
        let g = pb.grid;
        let mut radii = Vec::new();
        let mut area = 0.0;
        for j in 0..g.ny {
            for i in 0..g.nx {
                let s = j * g.nx + i;
                if d[s] > level {
                    area += g.h * g.h;
                    let inside_neighbours = [(1isize, 0isize), (-1, 0), (0, 1), (0, -1)].iter().all(|(a, b)| {
                        let (ii, jj) = (i as isize + a, j as isize + b);
                        d[jj as usize * g.nx + ii as usize] > level
                    });
                    if !inside_neighbours {
                        let (x, y) = g.center(i, j);
                        radii.push(x.hypot(y));
                    }
                }
            }
        }
        let m = radii.iter().sum::<f64>() / radii.len() as f64;
        ((radii.iter().map(|r| (r - m).powi(2)).sum::<f64>() / radii.len() as f64).sqrt(), area)
    }

    #[test]
    fn chipped_circle_becomes_round_and_keeps_volume() {
        let (pb, _) = disk_problem(5.0, 2.0, 0.35, 0.125);
        let (spread0, _) = radial_spread(&pb, &pb.d0, 1.0);
        let flow = evolve(&pb, 1.0, 0.5, true);
        let (spread1, _) = radial_spread(&pb, &flow.d, 1.0);
        assert_eq!(flow.basins, 1);
        assert!(spread1 < 0.35 * spread0, "rim roughness {spread0} -> {spread1}");
        let rel = (flow.volume_after_mm3 - flow.volume_before_mm3).abs() / flow.volume_before_mm3;
        assert!(rel < 2e-3, "volume drift {rel}");
    }

    #[test]
    fn clean_circle_stays_a_circle() {
        let (pb, _) = disk_problem(5.0, 2.0, 0.0, 0.125);
        let (spread0, a0) = radial_spread(&pb, &pb.d0, 1.0);
        let flow = evolve(&pb, 1.0, 0.5, true);
        let (spread1, a1) = radial_spread(&pb, &flow.d, 1.0);
        assert!(spread1 <= spread0 + 0.05, "{spread0} -> {spread1}");
        // The wall becomes an S-shaped meniscus of width ~r, so the mid-depth contour moves a little.
        assert!((a1 - a0).abs() / a0 < 0.10, "mid-depth area {a0} -> {a1}");
    }

    #[test]
    fn without_pressure_the_pocket_shrinks() {
        let (pb, _) = disk_problem(3.0, 2.0, 0.0, 0.125);
        let flow = evolve(&pb, 1.0, 0.5, false);
        assert!(flow.volume_after_mm3 < flow.volume_before_mm3);
    }

    #[test]
    fn fixed_and_outside_cells_never_change_and_no_pocket_means_no_change() {
        let (mut pb, _) = disk_problem(5.0, 2.0, 0.35, 0.125);
        let flow = evolve(&pb, 1.0, 0.5, true);
        for s in 0..pb.grid.len() {
            if pb.state[s] != FREE {
                assert_eq!(flow.d[s], pb.d0[s]);
            }
        }
        pb.d0.iter_mut().for_each(|x| *x = 0.0);
        let flow = evolve(&pb, 1.0, 0.5, true);
        assert_eq!(flow.basins, 0);
        assert!(flow.d.iter().all(|&x| x == 0.0));
    }

    #[test]
    fn growth_is_bounded_and_depth_respects_dmax() {
        let (mut pb, radius) = disk_problem(5.0, 2.0, 0.35, 0.125);
        pb.dmax.iter_mut().for_each(|x| *x = 2.0);
        let flow = evolve(&pb, 1.5, 0.5, true);
        for j in 0..pb.grid.ny {
            for i in 0..pb.grid.nx {
                let s = j * pb.grid.nx + i;
                let (x, y) = pb.grid.center(i, j);
                assert!(flow.d[s] <= 2.0 + 1e-12);
                if x.hypot(y) > radius + 0.35 * 1.5 + 0.5 + 2.0 * pb.grid.h {
                    assert_eq!(flow.d[s], 0.0, "spread beyond grow at {x},{y}");
                }
            }
        }
    }

    #[test]
    fn deterministic() {
        let (pb, _) = disk_problem(5.0, 2.0, 0.35, 0.125);
        let a = evolve(&pb, 1.0, 0.5, true);
        let b = evolve(&pb, 1.0, 0.5, true);
        assert!(a.d.iter().zip(&b.d).all(|(x, y)| x.to_bits() == y.to_bits()));
    }

    #[test]
    fn two_pockets_keep_their_own_volume() {
        let h = 0.125;
        let nx = (24.0 / h) as usize;
        let grid = Grid { x0: -12.0, y0: -12.0, h, nx, ny: nx };
        let n = grid.len();
        let mut d0 = vec![0.0; n];
        for j in 0..nx {
            for i in 0..nx {
                let (x, y) = grid.center(i, j);
                if (x + 5.0).hypot(y) < 3.0 { d0[j * nx + i] = 2.0; }
                if (x - 5.0).abs() < 2.0 && y.abs() < 4.0 { d0[j * nx + i] = 1.0; }
            }
        }
        let pb = Problem { grid, state: vec![FREE; n], d0, dmax: vec![3.0; n] };
        let flow = evolve(&pb, 1.0, 0.5, true);
        assert_eq!(flow.basins, 2);
        assert!(flow.worst_basin_volume_error < 5e-3, "{}", flow.worst_basin_volume_error);
    }

    #[test]
    fn params_parse_and_fail_closed() {
        let p = Params::parse("r=1.5,h=0.125,grow=0.4,mesh=0,rim=1.2,vp=0").unwrap();
        assert_eq!(p, Params { r: 1.5, h: 0.125, grow: 0.4, mesh: 0.0, rim: 1.2, vp: false });
        assert!(Params::parse("r=9").is_err());
        assert!(Params::parse("h=0.1").is_err());
        assert!(Params::parse("bogus=1").is_err());
        assert!(Params::parse("r=abc").is_err());
    }
}
