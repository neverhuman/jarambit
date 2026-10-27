//! The T6 ring submodel of `g5_sandwich::solve_ring_band_t6`, copied (that file is hashed into the G5 decoder, so it
//! cannot change) so that the displacement and nodal stress fields of every case can be kept for the renderings.
//! Copied verbatim from `crates/g5/src/g5_sandwich.rs` at 48d3a8c7: `a2_of`, `btcb`, `rcm`, `Skyline`, `T6_GP`,
//! `t6_shape`, `t6_bmat` and `solve_ring_band_t6`. The only change is in `solve_ring_band_t6`, which also returns
//! each case's fields (`RingCaseField`); every floating-point operation that feeds a capacity is unchanged, and the
//! capacities are checked against the record bit for bit.
#![allow(dead_code)]

use jarambit_g5::g5_sandwich::{joint_k, RingBandSummary, RingCaseResult};
use jarambit_g5::material::{self, AS_BUILT_WALL};
use jarambit_g5::g5_genome::FIXED_BORE_D_MM;
use jarambit_g5::g5_sdf::{eval_grip_slice, VoxelGrid3D};
use jarambit_physics::fine::{mesh_domain, von_mises, Mesh, T6};
use std::collections::BTreeMap;

#[inline]
fn a2_of(xs: &[f64; 3], ys: &[f64; 3]) -> f64 {
    (xs[1] - xs[0]) * (ys[2] - ys[0]) - (xs[2] - xs[0]) * (ys[1] - ys[0])
}

fn btcb<const M: usize>(
    b: &[[f64; M]; 3],
    c: &[[f64; 3]; 3],
    s: f64,
    k: &mut [[f64; M]; M],
) {
    let mut cb = [[0.0; M]; 3];
    for i in 0..3 {
        for j in 0..M {
            cb[i][j] = c[i][0] * b[0][j] + c[i][1] * b[1][j] + c[i][2] * b[2][j];
        }
    }
    for i in 0..M {
        for j in 0..M {
            k[i][j] += s * (b[0][i] * cb[0][j] + b[1][i] * cb[1][j] + b[2][i] * cb[2][j]);
        }
    }
}

fn rcm(n: usize, tris: &[[usize; 3]]) -> Vec<usize> {
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for t in tris {
        for a in 0..3 {
            for b in 0..3 {
                if a != b {
                    adj[t[a]].push(t[b]);
                }
            }
        }
    }
    for a in adj.iter_mut() {
        a.sort_unstable();
        a.dedup();
    }
    let mut order = Vec::with_capacity(n);
    let mut seen = vec![false; n];
    loop {
        let Some(mut start) = (0..n).filter(|&i| !seen[i]).min_by_key(|&i| adj[i].len()) else {
            break;
        };
        for _ in 0..2 {
            let mut lvl = vec![usize::MAX; n];
            let mut q = std::collections::VecDeque::from([start]);
            lvl[start] = 0;
            let mut last = start;
            while let Some(u) = q.pop_front() {
                last = u;
                for &v in &adj[u] {
                    if !seen[v] && lvl[v] == usize::MAX {
                        lvl[v] = lvl[u] + 1;
                        q.push_back(v);
                    }
                }
            }
            start = last;
        }
        let mut q = std::collections::VecDeque::from([start]);
        seen[start] = true;
        while let Some(u) = q.pop_front() {
            order.push(u);
            let mut nb: Vec<usize> = adj[u].iter().copied().filter(|&v| !seen[v]).collect();
            nb.sort_by_key(|&v| adj[v].len());
            for v in nb {
                seen[v] = true;
                q.push_back(v);
            }
        }
    }
    order.reverse();
    order
}

struct Skyline {
    n: usize,
    first: Vec<usize>,
    start: Vec<usize>,
    a: Vec<f64>,
}

impl Skyline {
    fn new(n: usize, first: Vec<usize>) -> Self {
        let mut start = vec![0; n + 1];
        for i in 0..n {
            start[i + 1] = start[i] + (i - first[i] + 1);
        }
        let a = vec![0.0; start[n]];
        Self { n, first, start, a }
    }
    #[inline]
    fn add(&mut self, i: usize, j: usize, v: f64) {
        let (i, j) = if i >= j { (i, j) } else { (j, i) };
        let k = self.start[i] + (j - self.first[i]);
        self.a[k] += v;
    }
    fn factor(&mut self) -> bool {
        let mut scale = 0.0f64;
        for i in 0..self.n {
            scale = scale.max(self.a[self.start[i] + i - self.first[i]].abs());
        }
        let tol = (scale * 1e-13).max(1e-300);
        for i in 0..self.n {
            let fi = self.first[i];
            let si = self.start[i];
            for j in fi..=i {
                let fj = self.first[j];
                let sj = self.start[j];
                let k0 = fi.max(fj);
                let mut s = self.a[si + j - fi];
                let ri = &self.a[si + k0 - fi..si + j - fi];
                let rj = &self.a[sj + k0 - fj..sj + j - fj];
                let mut acc = 0.0;
                for (x, y) in ri.iter().zip(rj) {
                    acc += x * y;
                }
                s -= acc;
                if j == i {
                    if !(s > tol) {
                        return false;
                    }
                    self.a[si + i - fi] = s.sqrt();
                } else {
                    self.a[si + j - fi] = s / self.a[sj + j - fj];
                }
            }
        }
        true
    }
    fn solve(&self, b: &mut [f64]) {
        for i in 0..self.n {
            let fi = self.first[i];
            let si = self.start[i];
            let mut s = b[i];
            for k in fi..i {
                s -= self.a[si + k - fi] * b[k];
            }
            b[i] = s / self.a[si + i - fi];
        }
        for i in (0..self.n).rev() {
            let fi = self.first[i];
            let si = self.start[i];
            b[i] /= self.a[si + i - fi];
            let bi = b[i];
            for k in fi..i {
                b[k] -= self.a[si + k - fi] * bi;
            }
        }
    }

    /// Compute `(rel_res, floor_eps_absK_absu, componentwise_backward_error)` against the unfactored
    /// skyline lower-triangular matrix `k_orig` (`I6`).
    fn residual_and_backward_error(
        &self,
        k_orig: &[f64],
        f_free: &[f64],
        u_free: &[f64],
    ) -> (f64, f64, f64) {
        let mut r = f_free.to_vec();
        let mut abs_ku = vec![0.0_f64; self.n];
        for i in 0..self.n {
            let fi = self.first[i];
            let si = self.start[i];
            let ui = u_free[i];
            let aui = ui.abs();
            for j in fi..=i {
                let kij = k_orig[si + (j - fi)];
                if kij == 0.0 {
                    continue;
                }
                let ak = kij.abs();
                let uj = u_free[j];
                r[i] -= kij * uj;
                abs_ku[i] += ak * uj.abs();
                if i != j {
                    r[j] -= kij * ui;
                    abs_ku[j] += ak * aui;
                }
            }
        }
        let r_norm = r.iter().map(|x| x * x).sum::<f64>().sqrt();
        let f_norm = f_free.iter().map(|x| x * x).sum::<f64>().sqrt().max(1e-300);
        let abs_ku_norm = abs_ku.iter().map(|x| x * x).sum::<f64>().sqrt();
        let rel_res = r_norm / f_norm;
        let floor_eps = f64::EPSILON * abs_ku_norm / f_norm;
        let comp_be = r
            .iter()
            .zip(abs_ku.iter())
            .zip(f_free.iter())
            .map(|((&ri, &aku), &fi)| ri.abs() / (aku + fi.abs() + 1e-300))
            .fold(0.0_f64, f64::max);
        (rel_res, floor_eps, comp_be)
    }
}

const T6_GP: [([f64; 3], f64); 3] = [
    ([2.0 / 3.0, 1.0 / 6.0, 1.0 / 6.0], 1.0 / 3.0),
    ([1.0 / 6.0, 2.0 / 3.0, 1.0 / 6.0], 1.0 / 3.0),
    ([1.0 / 6.0, 1.0 / 6.0, 2.0 / 3.0], 1.0 / 3.0),
];

#[inline]
fn t6_shape(l: &[f64; 3]) -> [f64; 6] {
    [
        l[0] * (2.0 * l[0] - 1.0),
        l[1] * (2.0 * l[1] - 1.0),
        l[2] * (2.0 * l[2] - 1.0),
        4.0 * l[0] * l[1],
        4.0 * l[1] * l[2],
        4.0 * l[2] * l[0],
    ]
}

fn t6_bmat(b: &[f64; 3], c: &[f64; 3], l: &[f64; 3]) -> [[f64; 12]; 3] {
    let mut dx = [0.0; 6];
    let mut dy = [0.0; 6];
    for i in 0..3 {
        dx[i] = (4.0 * l[i] - 1.0) * b[i];
        dy[i] = (4.0 * l[i] - 1.0) * c[i];
    }
    for (k, (i, j)) in [(3usize, (0usize, 1usize)), (4, (1, 2)), (5, (2, 0))] {
        dx[k] = 4.0 * (l[i] * b[j] + l[j] * b[i]);
        dy[k] = 4.0 * (l[i] * c[j] + l[j] * c[i]);
    }
    let mut bm = [[0.0; 12]; 3];
    for k in 0..6 {
        bm[0][2 * k] = dx[k];
        bm[1][2 * k + 1] = dy[k];
        bm[2][2 * k] = dy[k];
        bm[2][2 * k + 1] = dx[k];
    }
    bm
}

/// One ring case's fields: the T6 displacement (2 per node) and the corner-node von Mises stress per newton of load
/// (the capacity is the allowable over the peak of this field outside the seat).
pub struct RingCaseField {
    pub name: String,
    /// Band direction (rad) of a bearing case; None for the ring back
    pub theta: Option<f64>,
    pub result: RingCaseResult,
    pub u: Vec<f64>,
    pub vm_corner: Vec<f64>,
    /// Corner nodes read for the peak (outside the seat window)
    pub read: Vec<bool>,
}

/// The T6 mesh the fields live on.
pub struct RingFields {
    /// Bore centre (x; y = 0)
    pub cx: f64,
    pub xy: Vec<(f64, f64)>,
    pub tris: Vec<[usize; 6]>,
    pub n_corner: usize,
    pub t_elem: Vec<f64>,
    pub cases: Vec<RingCaseField>,
}

pub fn solve_ring_band_t6(
    mesh: &Mesh,
    t_elem: &[f64],
    t_at: &dyn Fn(f64, f64) -> f64,
    cx: f64,
    r_b: f64,
    r_od: f64,
    t_joint: f64,
    seat_x: (f64, f64),
) -> Option<(RingBandSummary, RingFields)> {
    if mesh.tris.is_empty() || t_elem.len() != mesh.tris.len() {
        return None;
    }
    let t6 = T6::new(mesh, 2.0, material::e_mpa(), material::nu()).with_thickness(t_elem.to_vec());
    let n6 = t6.n();
    let mut fixed = vec![false; n6 * 2];
    let mut n_fixed = 0usize;
    for (i, &(x, _)) in t6.xy.iter().enumerate() {
        if x >= seat_x.0 && x <= seat_x.1 {
            fixed[i * 2] = true;
            fixed[i * 2 + 1] = true;
            n_fixed += 1;
        }
    }
    if n_fixed < 3 {
        return None;
    }

    // Assemble T6 skyline system with per-Gauss-point thickness `t_at(xq, yq).max(0.3)` (`I2(d)`)
    // and retain `k_orig` for exact residual / backward-error verification (`I6`).
    let adj6: Vec<[usize; 3]> = t6
        .tris
        .iter()
        .flat_map(|t| {
            [
                [t[0], t[3], t[5]],
                [t[3], t[1], t[4]],
                [t[5], t[4], t[2]],
                [t[3], t[4], t[5]],
            ]
        })
        .collect();
    let perm = rcm(n6, &adj6);
    let mut map = vec![-1isize; n6 * 2];
    let mut nf = 0usize;
    for &node in &perm {
        for c in 0..2 {
            if !fixed[node * 2 + c] {
                map[node * 2 + c] = nf as isize;
                nf += 1;
            }
        }
    }
    if nf == 0 {
        return None;
    }
    let mut first: Vec<usize> = (0..nf).collect();
    for t in &t6.tris {
        let ids: Vec<usize> = t
            .iter()
            .flat_map(|&nd| (0..2).map(move |c| nd * 2 + c))
            .filter_map(|d| (map[d] >= 0).then(|| map[d] as usize))
            .collect();
        if let Some(&mn) = ids.iter().min() {
            for &i in &ids {
                first[i] = first[i].min(mn);
            }
        }
    }
    let mut sky = Skyline::new(nf, first);
    for t in &t6.tris {
        let xs = [t6.xy[t[0]].0, t6.xy[t[1]].0, t6.xy[t[2]].0];
        let ys = [t6.xy[t[0]].1, t6.xy[t[1]].1, t6.xy[t[2]].1];
        let a2 = a2_of(&xs, &ys);
        if a2.abs() <= 1e-14 {
            continue;
        }
        let b_grad: [f64; 3] = std::array::from_fn(|i| (ys[(i + 1) % 3] - ys[(i + 2) % 3]) / a2);
        let c_grad: [f64; 3] = std::array::from_fn(|i| (xs[(i + 2) % 3] - xs[(i + 1) % 3]) / a2);
        let area = 0.5 * a2.abs();
        let mut ke = [[0.0; 12]; 12];
        for (l, w_gp) in T6_GP {
            let xq = l[0] * xs[0] + l[1] * xs[1] + l[2] * xs[2];
            let yq = l[0] * ys[0] + l[1] * ys[1] + l[2] * ys[2];
            let tq = t_at(xq, yq).max(0.3);
            let bm = t6_bmat(&b_grad, &c_grad, &l);
            btcb(&bm, &t6.cmat, w_gp * area * tq, &mut ke);
        }
        let id: [isize; 12] = std::array::from_fn(|q| map[t[q / 2] * 2 + q % 2]);
        for a in 0..12 {
            if id[a] < 0 {
                continue;
            }
            for bb in 0..=a {
                if id[bb] >= 0 {
                    sky.add(id[a] as usize, id[bb] as usize, ke[a][bb]);
                }
            }
        }
    }
    let k_orig = sky.a.clone();
    if !sky.factor() {
        return None;
    }

    // Precompute Gauss point coordinates and r_out for ring-back
    let mut r_out = r_od;
    let mut found_rear = false;
    let mut max_r_rear = 0.0_f64;
    for tri in &mesh.tris {
        let p0 = mesh.xy[tri[0]];
        let p1 = mesh.xy[tri[1]];
        let p2 = mesh.xy[tri[2]];
        for (l, _) in T6_GP {
            let xq = l[0] * p0.0 + l[1] * p1.0 + l[2] * p2.0;
            let yq = l[0] * p0.1 + l[1] * p1.1 + l[2] * p2.1;
            let tq = t_at(xq, yq);
            if xq < cx - 0.7 * r_od && tq > 0.31 {
                let rq = (xq - cx).hypot(yq);
                if !found_rear || rq > max_r_rear {
                    max_r_rear = rq;
                    found_rear = true;
                }
            }
        }
    }
    if found_rear {
        r_out = max_r_rear;
    }

    // Bearing band of +/-60 deg on the bore centred at `theta_c`, pushing the bore outward along theta_c (the owner's
    // bearing case is theta_c = 0, toward the handle); `None` selects the owner's ring-back case (outer rear, +x).
    let assemble_band_load = |bearing_at: Option<f64>| -> Option<(Vec<f64>, f64, usize)> {
        let is_bearing = bearing_at.is_some();
        let theta_c = bearing_at.unwrap_or(0.0);
        let (dirx, diry) = if is_bearing { (theta_c.cos(), theta_c.sin()) } else { (1.0, 0.0) };
        let mut f = vec![0.0; n6 * 2];
        let mut lb = 0.0_f64;
        let mut n_load = 0usize;

        for t6_nodes in &t6.tris {
            let p0 = t6.xy[t6_nodes[0]];
            let p1 = t6.xy[t6_nodes[1]];
            let p2 = t6.xy[t6_nodes[2]];
            let area = 0.5
                * ((p1.0 - p0.0) * (p2.1 - p0.1) - (p2.0 - p0.0) * (p1.1 - p0.1)).abs();
            for (l, w_gp) in T6_GP {
                let xq = l[0] * p0.0 + l[1] * p1.0 + l[2] * p2.0;
                let yq = l[0] * p0.1 + l[1] * p1.1 + l[2] * p2.1;
                let rq = (xq - cx).hypot(yq);
                let in_band = if is_bearing {
                    let thq = yq.atan2(xq - cx);
                    let dth = (thq - theta_c + std::f64::consts::PI).rem_euclid(2.0 * std::f64::consts::PI)
                        - std::f64::consts::PI;
                    (rq < r_b + 1.5) && (dth.abs() < 60.0_f64.to_radians())
                } else {
                    (xq < cx - 0.7 * r_od) && (rq > r_out - 1.5)
                };
                if !in_band {
                    continue;
                }
                n_load += 1;
                let tq = t_at(xq, yq).max(0.3);
                let wq = w_gp * area * tq;
                lb += wq;
                let n_shape = t6_shape(&l);
                for (k, &nd) in t6_nodes.iter().enumerate() {
                    f[nd * 2] += dirx * wq * n_shape[k];
                    f[nd * 2 + 1] += diry * wq * n_shape[k];
                }
            }
        }
        if lb <= 1e-12 || n_load == 0 {
            return None;
        }
        for v in &mut f {
            *v /= lb;
        }
        Some((f, lb, n_load))
    };

    let jk = joint_k(t_joint);
    let eval_case = |bearing_at: Option<f64>| -> Option<(RingCaseResult, Vec<f64>, Vec<f64>, Vec<bool>)> {
        let (f, lb, n_load_pts) = assemble_band_load(bearing_at)?;
        let mut f_free = vec![0.0; nf];
        for (d, &g) in map.iter().enumerate() {
            if g >= 0 {
                f_free[g as usize] += f[d];
            }
        }
        let mut u_free = f_free.clone();
        sky.solve(&mut u_free);
        if u_free.iter().any(|x| !x.is_finite()) {
            return None;
        }
        let (rel_res, floor_eps, comp_be) =
            sky.residual_and_backward_error(&k_orig, &f_free, &u_free);
        let ok = rel_res.is_finite()
            && (rel_res <= 1e-10 || (rel_res <= floor_eps && comp_be <= 1e-12));
        if !ok {
            return None;
        }
        let mut u = vec![0.0; f.len()];
        for (d, &g) in map.iter().enumerate() {
            if g >= 0 {
                u[d] = u_free[g as usize];
            }
        }
        let nod = t6.nodal_stress(&u);
        let vm_corner: Vec<f64> = (0..t6.n_corner).map(|i| von_mises(nod[i])).collect();
        let read: Vec<bool> = (0..t6.n_corner).map(|i| t6.xy[i].0 < seat_x.0 - 3.0).collect();
        let mut best_vm = 0.0_f64;
        let mut best_at = [cx, 0.0];
        // Match fe2d.py line 179: exclude nodes with x >= seat[0] - 3.0 across corner nodes
        for i in 0..t6.n_corner {
            let (x, y) = t6.xy[i];
            if x >= seat_x.0 - 3.0 {
                continue;
            }
            let vm = von_mises(nod[i]);
            if vm.is_finite() && vm > best_vm {
                best_vm = vm;
                best_at = [x, y];
            }
        }
        if best_vm <= 1e-12 {
            return None;
        }
        let cap_n = material::yield_allowable(t_joint, AS_BUILT_WALL) / best_vm;
        Some((
            RingCaseResult {
                sigma_pk_per_n: best_vm,
                capacity_n: cap_n,
                at: best_at,
                load_norm: lb,
                n_load_pts,
                true_rel_res: rel_res,
            },
            u,
            vm_corner,
            read,
        ))
    };

    let field = |name: String, theta: Option<f64>, c: (RingCaseResult, Vec<f64>, Vec<f64>, Vec<bool>)| RingCaseField {
        name,
        theta,
        result: c.0,
        u: c.1,
        vm_corner: c.2,
        read: c.3,
    };
    let bearing = eval_case(Some(0.0))?;
    let back = eval_case(None)?;
    let mut cases = vec![field("bearing".into(), Some(0.0), bearing.clone()), field("back".into(), None, back.clone())];
    // RS-karambit-v5 [loads.ring] any_direction_n: the bore bearing band swept round the ring every 30 deg
    let sweep: Vec<(usize, Option<(RingCaseResult, Vec<f64>, Vec<f64>, Vec<bool>)>)> =
        (0..12).map(|k| (k, eval_case(Some((k as f64 * 30.0).to_radians())))).collect();
    let any_direction_min_capacity_n =
        sweep.iter().filter_map(|(_, c)| c.as_ref().map(|c| c.0.capacity_n)).fold(f64::INFINITY, f64::min);
    for (k, c) in sweep {
        if let Some(c) = c {
            cases.push(field(format!("any_{:03}", k * 30), Some((k as f64 * 30.0).to_radians()), c));
        }
    }
    Some((
        RingBandSummary {
            bearing: bearing.0,
            back: back.0,
            any_direction_min_capacity_n,
            joint_k: jk,
            t_joint,
            n_el: mesh.tris.len(),
            n_dof: n6 * 2,
        },
        RingFields { cx, xy: t6.xy.clone(), tris: t6.tris.clone(), n_corner: t6.n_corner, t_elem: t_elem.to_vec(), cases },
    ))
}

/// The ring + neck submodel of a design, meshed and solved as `g5_qd::evaluate_tier1_with_grid` does (the outline and
/// thickness lines are copied from `crates/g5/src/g5_qd.rs` at 48d3a8c7), with the fields kept.
pub fn ring_solve(p: &BTreeMap<&'static str, f64>, grid: &VoxelGrid3D) -> Option<(RingBandSummary, RingFields)> {
    let cx = p["cx"];
    let r_b = FIXED_BORE_D_MM * 0.5;
    let r_od = p["R_od"];
    let x_clip = 46.5_f64;
    let th_join = p["junction_deg"].clamp(24.0, 45.0).to_radians();
    let mut outer = Vec::with_capacity(80);
    // 1. Rear ring arc from +th_join around pi to 2*pi - th_join (counter-clockwise)
    let n_arc = 44usize;
    for k in 0..=n_arc {
        let th = th_join
            + (k as f64 / n_arc as f64) * (2.0 * std::f64::consts::PI - 2.0 * th_join);
        let r_th = r_od + p["od_tear"] * 0.5 * (1.0 + th.cos()) + p["od_h2"] * (2.0 * th).cos();
        outer.push((cx + r_th * th.cos(), r_th * th.sin()));
    }
    // 2. Bottom neck & grip outline from x_join to x_clip (including neck_r smooth fillet, I2)
    let x_join = (cx + r_od * th_join.cos()).max(p["x_t"]);
    let y_join_bot = -r_od * th_join.sin();
    let y_join_top = r_od * th_join.sin();
    let neck_span = (0.8 * p["neck_r"]).clamp(2.5, 6.0);
    let n_neck = 18usize;
    for k in 1..=n_neck {
        let x = x_join + (k as f64 / n_neck as f64) * (x_clip - x_join);
        let gc = eval_grip_slice(&p, x);
        let w_fillet = (-(x - x_join) / neck_span).exp();
        let yb = gc.yb.min(y_join_bot * w_fillet + gc.yb * (1.0 - w_fillet));
        outer.push((x, yb));
    }
    // 3. Top grip & neck outline from x_clip back to x_join
    for k in (1..=n_neck).rev() {
        let x = x_join + (k as f64 / n_neck as f64) * (x_clip - x_join);
        let gc = eval_grip_slice(&p, x);
        let w_fillet = (-(x - x_join) / neck_span).exp();
        let yt = gc.yt.max(y_join_top * w_fillet + gc.yt * (1.0 - w_fillet));
        outer.push((x, yt));
    }
    let hole: Vec<(f64, f64)> = (0..48)
        .map(|k| {
            let th = (k as f64) * 2.0 * std::f64::consts::PI / 48.0;
            (cx + r_b * th.cos(), r_b * th.sin())
        })
        .collect();
    let holes = [hole];
    if let Some(mesh) = mesh_domain(&outer, &holes, 0.35).or_else(|| mesh_domain(&outer, &holes, 0.65)) {
        // N7: Use the homogenized 2D `skin + rho^1.8` thickness map interpolated bilinearly at quadrature points (`evaluate.py::tmap_eff` parity)
        let t_at = |x: f64, y: f64| -> f64 { grid.ring_tmap_eff_bilinear_at(x, y).max(0.3) };
        let t_elem: Vec<f64> = mesh
            .tris
            .iter()
            .map(|t| {
                let xm = (mesh.xy[t[0]].0 + mesh.xy[t[1]].0 + mesh.xy[t[2]].0) / 3.0;
                let ym = (mesh.xy[t[0]].1 + mesh.xy[t[1]].1 + mesh.xy[t[2]].1) / 3.0;
                t_at(xm, ym)
            })
            .collect();
        return solve_ring_band_t6(&mesh, &t_elem, &t_at, cx, r_b, r_od, p["web_t"], (40.0, 46.0));
    }
    None
}
