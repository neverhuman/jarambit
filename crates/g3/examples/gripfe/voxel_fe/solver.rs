//! Vendored for the G3 v3.0.0 voxel FE certification: a verbatim copy of `platform/voxel-fe/src/solver.rs` at commit
//! 53e75771 (implant lane), with `crate::` changed to `super::`. Its controls are in that crate's tests/controls.rs.
//! Matrix-free hex8 operators on nested voxel grids, and the conjugate-gradient solve
//! preconditioned by a geometric multigrid V-cycle.
//!
//! Every vector lives on the free dofs (zero elsewhere). Coarse levels re-discretize: a coarse
//! voxel's stiffness is the mean of its eight children's, so the preconditioner is an
//! approximation that conjugate gradients corrects; it is linear and symmetric (damped block
//! Jacobi smoothing, equal pre and post sweeps), as conjugate gradients requires.
use super::{Convergence, Grid, Options};
use std::thread;

pub(crate) struct Level {
    pub grid: Grid,
    /// E h per element; 0 for void or unsolved.
    pub scale: Vec<f64>,
    /// Stiffness block of each element.
    pub kind: Vec<u8>,
    pub blocks: Vec<Vec<f64>>,
    /// Per dof: displacement prescribed zero.
    pub fixed: Vec<bool>,
    /// Per dof: belongs to a stiff element and is not fixed.
    pub free: Vec<bool>,
    /// Inverse nodal 3 x 3 diagonal blocks, zero rows and columns on non-free dofs.
    pub inverse: Vec<[f64; 9]>,
}

/// Run `f(first_node, values)` over contiguous node ranges of `data` (3 values per node).
fn par_nodes(threads: usize, data: &mut [f64], f: impl Fn(usize, &mut [f64]) + Sync) {
    let nodes = data.len() / 3;
    if threads <= 1 || nodes < SERIAL_NODES {
        return f(0, data);
    }
    let per = nodes.div_ceil(threads.clamp(1, nodes.max(1))).max(1);
    let f = &f;
    thread::scope(|s| {
        for (c, chunk) in data.chunks_mut(3 * per).enumerate() {
            s.spawn(move || f(c * per, chunk));
        }
    });
}

/// Below this many nodes a level is processed on the calling thread.
const SERIAL_NODES: usize = 32_768;

fn strides(g: &Grid) -> (usize, usize, [usize; 8]) {
    let (s0, s1) = (g.n[0] + 1, (g.n[0] + 1) * (g.n[1] + 1));
    let off = std::array::from_fn(|b| (b & 1) + s0 * ((b >> 1) & 1) + s1 * ((b >> 2) & 1));
    (s0, s1, off)
}

impl Level {
    pub fn new(
        grid: Grid,
        scale: Vec<f64>,
        kind: Vec<u8>,
        blocks: Vec<Vec<f64>>,
        fixed: Vec<bool>,
    ) -> Self {
        let mut free = vec![false; 3 * grid.nodes()];
        for (e, &stiff) in scale.iter().enumerate() {
            if stiff > 0. {
                for a in 0..8 {
                    let v = grid.corner(e, a);
                    for c in 0..3 {
                        free[3 * v + c] = !fixed[3 * v + c];
                    }
                }
            }
        }
        let mut level = Self {
            grid,
            scale,
            kind,
            blocks,
            fixed,
            free,
            inverse: vec![],
        };
        level.inverse = level.block_jacobi();
        level
    }
    pub fn dofs(&self) -> usize {
        3 * self.grid.nodes()
    }
    /// y = K x for x zero off the free dofs; y is zero off them too.
    pub fn apply(&self, x: &[f64], y: &mut [f64], threads: usize) {
        let g = &self.grid;
        let [n0, n1, n2] = g.n;
        let (s0, s1, off) = strides(g);
        let planes = n2 + 1;
        let t = if g.nodes() < SERIAL_NODES {
            1
        } else {
            threads.clamp(1, planes)
        };
        let per = planes.div_ceil(t);
        thread::scope(|s| {
            for (c, chunk) in y.chunks_mut(3 * s1 * per).enumerate() {
                s.spawn(move || {
                    let first = c * per * s1;
                    for (o, out) in chunk.chunks_mut(3).enumerate() {
                        let v = first + o;
                        if !(self.free[3 * v] || self.free[3 * v + 1] || self.free[3 * v + 2]) {
                            out.fill(0.);
                            continue;
                        }
                        let (i, j, k) = (v % s0, (v / s0) % (n1 + 1), v / s1);
                        let mut acc = [0.; 3];
                        for a in 0..8 {
                            let (di, dj, dk) = (a & 1, (a >> 1) & 1, (a >> 2) & 1);
                            if i < di
                                || j < dj
                                || k < dk
                                || i - di >= n0
                                || j - dj >= n1
                                || k - dk >= n2
                            {
                                continue;
                            }
                            let e = (i - di) + n0 * ((j - dj) + n1 * (k - dk));
                            let scale = self.scale[e];
                            if scale == 0. {
                                continue;
                            }
                            let block = &self.blocks[self.kind[e] as usize];
                            let base = v - off[a];
                            for (b, &ob) in off.iter().enumerate() {
                                let w = 3 * (base + ob);
                                let xs = [scale * x[w], scale * x[w + 1], scale * x[w + 2]];
                                for (ca, acc) in acc.iter_mut().enumerate() {
                                    let row = &block[24 * (3 * a + ca) + 3 * b..][..3];
                                    *acc += row[0] * xs[0] + row[1] * xs[1] + row[2] * xs[2];
                                }
                            }
                        }
                        for ca in 0..3 {
                            out[ca] = if self.free[3 * v + ca] { acc[ca] } else { 0. };
                        }
                    }
                });
            }
        });
    }
    fn block_jacobi(&self) -> Vec<[f64; 9]> {
        let g = &self.grid;
        let mut diag = vec![[0.; 9]; g.nodes()];
        for e in 0..g.elements() {
            if self.scale[e] == 0. {
                continue;
            }
            let block = &self.blocks[self.kind[e] as usize];
            for a in 0..8 {
                let m = &mut diag[g.corner(e, a)];
                for r in 0..3 {
                    for c in 0..3 {
                        m[3 * r + c] += self.scale[e] * block[24 * (3 * a + r) + 3 * a + c];
                    }
                }
            }
        }
        diag.iter()
            .enumerate()
            .map(|(v, m)| {
                let mut m = *m;
                let free: [bool; 3] = std::array::from_fn(|c| self.free[3 * v + c]);
                for c in 0..3 {
                    if !free[c] {
                        for t in 0..3 {
                            m[3 * c + t] = 0.;
                            m[3 * t + c] = 0.;
                        }
                        m[4 * c] = 1.;
                    }
                }
                let mut inv = invert3(m);
                for r in 0..3 {
                    for c in 0..3 {
                        if !free[r] || !free[c] {
                            inv[3 * r + c] = 0.;
                        }
                    }
                }
                inv
            })
            .collect()
    }
    /// x += omega D^-1 (b - ax); `ax` None means zero.
    fn jacobi(&self, b: &[f64], ax: Option<&[f64]>, x: &mut [f64], omega: f64, threads: usize) {
        par_nodes(threads, x, |first, x| {
            for (o, x) in x.chunks_mut(3).enumerate() {
                let v = first + o;
                let m = &self.inverse[v];
                let r: [f64; 3] =
                    std::array::from_fn(|c| b[3 * v + c] - ax.map_or(0., |a| a[3 * v + c]));
                for (row, x) in x.iter_mut().enumerate() {
                    *x +=
                        omega * (m[3 * row] * r[0] + m[3 * row + 1] * r[1] + m[3 * row + 2] * r[2]);
                }
            }
        });
    }
    /// The next coarser level: half the voxels per axis (rounded up), children's mean stiffness.
    pub fn coarsen(&self) -> Self {
        let f = &self.grid;
        let n = f.n.map(|n| n.div_ceil(2));
        let grid = Grid {
            n,
            h: 2. * f.h,
            origin: f.origin,
        };
        let mut scale = vec![0.; grid.elements()];
        for e in 0..f.elements() {
            let [i, j, k] = f.element_index(e);
            // Mean of eight children; E h doubles with h.
            scale[grid.element([i / 2, j / 2, k / 2])] += self.scale[e] / 4.;
        }
        let mut fixed = vec![false; 3 * grid.nodes()];
        for v in 0..grid.nodes() {
            let [i, j, k] = grid.node_index(v);
            let w = f.node([
                (2 * i).min(f.n[0]),
                (2 * j).min(f.n[1]),
                (2 * k).min(f.n[2]),
            ]);
            for c in 0..3 {
                fixed[3 * v + c] = self.fixed[3 * w + c];
            }
        }
        let kind = vec![0; grid.elements()];
        Self::new(grid, scale, kind, vec![self.blocks[0].clone()], fixed)
    }
    /// Fine-level values interpolated (trilinearly) from this coarser level's.
    fn prolong(&self, fine: &Level, xc: &[f64], x: &mut [f64], threads: usize) {
        let g = &fine.grid;
        let c = &self.grid;
        let taps = |i: usize| -> [(usize, f64); 2] {
            if i % 2 == 0 {
                [(i / 2, 1.), (i / 2, 0.)]
            } else {
                [((i - 1) / 2, 0.5), ((i + 1) / 2, 0.5)]
            }
        };
        par_nodes(threads, x, |first, x| {
            for (o, x) in x.chunks_mut(3).enumerate() {
                let v = first + o;
                let [i, j, k] = g.node_index(v);
                let mut acc = [0.; 3];
                for (a, wa) in taps(i) {
                    for (b, wb) in taps(j) {
                        for (d, wd) in taps(k) {
                            let w = wa * wb * wd;
                            if w == 0. {
                                continue;
                            }
                            let u = 3 * c.node([a, b, d]);
                            for t in 0..3 {
                                acc[t] += w * xc[u + t];
                            }
                        }
                    }
                }
                for t in 0..3 {
                    if fine.free[3 * v + t] {
                        x[t] += acc[t];
                    }
                }
            }
        });
    }
    /// The transpose of `prolong`: fine residual gathered onto this coarser level.
    fn restrict(&self, fine: &Level, r: &[f64], threads: usize) -> Vec<f64> {
        let g = &fine.grid;
        let c = &self.grid;
        let mut out = vec![0.; self.dofs()];
        par_nodes(threads, &mut out, |first, out| {
            for (o, out) in out.chunks_mut(3).enumerate() {
                let v = first + o;
                if !(self.free[3 * v] || self.free[3 * v + 1] || self.free[3 * v + 2]) {
                    continue;
                }
                let [i, j, k] = c.node_index(v);
                let mut acc = [0.; 3];
                for di in -1i64..=1 {
                    for dj in -1i64..=1 {
                        for dk in -1i64..=1 {
                            let p = [2 * i as i64 + di, 2 * j as i64 + dj, 2 * k as i64 + dk];
                            if (0..3).any(|t| p[t] < 0 || p[t] > g.n[t] as i64) {
                                continue;
                            }
                            let w = [di, dj, dk]
                                .iter()
                                .map(|&d| if d == 0 { 1. } else { 0.5 })
                                .product::<f64>();
                            let u = 3 * g.node([p[0] as usize, p[1] as usize, p[2] as usize]);
                            for t in 0..3 {
                                acc[t] += w * r[u + t];
                            }
                        }
                    }
                }
                for t in 0..3 {
                    if self.free[3 * v + t] {
                        out[t] = acc[t];
                    }
                }
            }
        });
        out
    }
}

pub(crate) fn invert3(m: [f64; 9]) -> [f64; 9] {
    let c = [
        m[4] * m[8] - m[5] * m[7],
        m[2] * m[7] - m[1] * m[8],
        m[1] * m[5] - m[2] * m[4],
        m[5] * m[6] - m[3] * m[8],
        m[0] * m[8] - m[2] * m[6],
        m[2] * m[3] - m[0] * m[5],
        m[3] * m[7] - m[4] * m[6],
        m[1] * m[6] - m[0] * m[7],
        m[0] * m[4] - m[1] * m[3],
    ];
    let det = m[0] * c[0] + m[1] * c[3] + m[2] * c[6];
    c.map(|v| v / det)
}
pub(crate) fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

const OMEGA: f64 = 0.6;
const SWEEPS: usize = 2;
const COARSEST_SWEEPS: usize = 40;
const REPLACE: usize = 50;

/// The levels below `fine`, coarsened while every axis keeps at least two voxels.
pub(crate) fn hierarchy(fine: &Level) -> Vec<Level> {
    let mut out: Vec<Level> = vec![];
    loop {
        let last = out.last().unwrap_or(fine);
        if last.grid.n.iter().any(|&n| n < 4) || last.grid.nodes() < 512 {
            return out;
        }
        out.push(last.coarsen());
    }
}

/// One V-cycle approximating K^-1 b on `levels[0]`.
fn vcycle(levels: &[&Level], b: &[f64], threads: usize) -> Vec<f64> {
    let l = levels[0];
    let mut x = vec![0.; b.len()];
    let mut ax = vec![0.; b.len()];
    let smooth = |x: &mut Vec<f64>, ax: &mut Vec<f64>, sweeps: usize| {
        for _ in 0..sweeps {
            l.apply(x, ax, threads);
            l.jacobi(b, Some(ax), x, OMEGA, threads);
        }
    };
    l.jacobi(b, None, &mut x, OMEGA, threads);
    if levels.len() == 1 {
        smooth(&mut x, &mut ax, COARSEST_SWEEPS - 1);
        return x;
    }
    smooth(&mut x, &mut ax, SWEEPS - 1);
    l.apply(&x, &mut ax, threads);
    let r: Vec<f64> = b.iter().zip(&ax).map(|(b, a)| b - a).collect();
    let coarse = levels[1];
    let bc = coarse.restrict(l, &r, threads);
    let xc = vcycle(&levels[1..], &bc, threads);
    coarse.prolong(l, &xc, &mut x, threads);
    smooth(&mut x, &mut ax, SWEEPS);
    x
}

pub(crate) fn pcg(
    fine: &Level,
    coarse: &[Level],
    b: &[f64],
    options: &Options,
) -> Result<(Vec<f64>, Convergence), String> {
    let n = b.len();
    let t = options.threads.max(1);
    let levels: Vec<&Level> = std::iter::once(fine).chain(coarse).collect();
    let bb = dot(b, b).sqrt();
    if !(bb.is_finite() && bb > 0.) {
        return Err("zero or nonfinite load".into());
    }
    let mut x = vec![0.; n];
    let mut r = b.to_vec();
    let mut z = vcycle(&levels, &r, t);
    let mut p = z.clone();
    let mut q = vec![0.; n];
    let mut rz = dot(&r, &z);
    let mut iterations = 0;
    while iterations < options.max_iterations {
        if dot(&r, &r).sqrt() <= options.relative_tolerance * bb {
            break;
        }
        fine.apply(&p, &mut q, t);
        let pq = dot(&p, &q);
        if !(pq.is_finite() && pq > 0.) {
            return Err(format!("stiffness not positive definite (p.Kp = {pq})"));
        }
        let alpha = rz / pq;
        for i in 0..n {
            x[i] += alpha * p[i];
            r[i] -= alpha * q[i];
        }
        if (iterations + 1) % REPLACE == 0 {
            // Residual replacement: the recurrence drifts from b - K x on ill-conditioned bodies.
            fine.apply(&x, &mut q, t);
            for i in 0..n {
                r[i] = b[i] - q[i];
            }
        }
        z = vcycle(&levels, &r, t);
        let rz_new = dot(&r, &z);
        let beta = rz_new / rz;
        rz = rz_new;
        for i in 0..n {
            p[i] = z[i] + beta * p[i];
        }
        iterations += 1;
    }
    fine.apply(&x, &mut q, t);
    let res: Vec<f64> = b.iter().zip(&q).map(|(b, q)| b - q).collect();
    let relative_residual = dot(&res, &res).sqrt() / bb;
    if !(relative_residual.is_finite() && relative_residual <= 10. * options.relative_tolerance) {
        return Err(format!(
            "not converged: relative residual {relative_residual:e} after {iterations} iterations"
        ));
    }
    Ok((
        x,
        Convergence {
            iterations,
            relative_residual,
        },
    ))
}
