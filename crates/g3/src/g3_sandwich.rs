//! G3 structural mechanics (`T4.1` + HPC fail-closed solid tip):
//! 1. `SandwichSection { t_env, t_mem, t3_bend }` and `SandwichPlate` (`CST` membrane + Batoz `DKT`
//!    bending) for hollow/TPMS-cored 3D shells and corrugated blades.
//! 2. `solve_ring_band_t6`: 6-node quadratic membrane (`T6`) ring pull and ring-back solver with
//!    the 1.5 mm thickness-weighted distributed load band (`fe2d.py` parity, avoiding the rounded
//!    outer/inner edge $t \to 0$ boundary singularity).
//! 3. `solve_g3_solid_tet10_strict`: fail-closed 3D `TET10` tip submodel solver built directly from
//!    the exported surface height law and enforcing true float64 residual
//!    $\|\mathbf{K}\mathbf{u} - \mathbf{f}\|_2 / \|\mathbf{f}\|_2 \le 10^{-10}$ (`HPC_REPORT.md` §4).

use jarambit_physics::fine::{c_plane, corners, von_mises, Mesh, P2, T6, NU};
use serde::{Deserialize, Serialize};

pub const E_TI64_MPA: f64 = 113_800.0;
pub const YIELD_TI64_MPA: f64 = 828.0;
pub const K100_HOT_DERATE: f64 = 0.912;

/// Gibson-Ashby relative stiffness exponent for stretching/bending-dominated TPMS network lattices.
pub const GIBSON_ASHBY_EXP: f64 = 1.8;

/// Joint thin + hot (100 C) knockdown factor matching `fe2d.py::joint_k`.
#[inline]
pub fn joint_k(t_joint_mm: f64) -> f64 {
    let t = t_joint_mm.max(0.2);
    ((t - 0.1) / t) * K100_HOT_DERATE
}

/// Per-element or per-(x, y) sandwich section properties for a z-symmetric shell + TPMS lattice core.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SandwichSection {
    /// Outer envelope thickness $t_{\text{env}}$ (mm), used for outer-fiber distance $z = \pm t_{\text{env}}/2$.
    pub t_env: f64,
    /// Membrane-equivalent thickness $t_{\text{mem}} = \int w(z)\,dz$ (mm), used for `CST`/`T6` membrane stiffness $A_{\text{eff}}$.
    pub t_mem: f64,
    /// Bending-equivalent cubed thickness $t_{\text{bend}}^3 = 12 \int w(z) z^2\,dz$ ($\text{mm}^3$), used for `DKT` bending stiffness $D_{\text{eff}}$.
    pub t3_bend: f64,
}

impl SandwichSection {
    /// Solid monolithic section of thickness `t` ($t_{\text{env}} = t_{\text{mem}} = t$, $t_{\text{bend}}^3 = t^3$).
    #[inline]
    pub fn solid(t: f64) -> Self {
        let t = t.max(0.05);
        Self {
            t_env: t,
            t_mem: t,
            t3_bend: t * t * t,
        }
    }

    /// Analytic symmetric sandwich section with outer skins of thickness `skin_t` (each side),
    /// interior cavity height $h_c = \max(0, t_{\text{env}} - 2 t_{\text{skin}})$, lattice density
    /// $\rho \in [0.06, 1.0]$ ($\rho_e = \rho^{1.8}$), and optional mid-plane solid rib of half-thickness `rib_half_t`.
    #[inline]
    pub fn from_skin_and_lattice(t_env: f64, skin_t: f64, rho: f64, rib_half_t: f64) -> Self {
        let t_env = t_env.max(0.05);
        if rho >= 0.9999 || t_env <= 2.0 * skin_t {
            return Self::solid(t_env);
        }
        let rho_e = rho.clamp(0.0, 1.0).powf(GIBSON_ASHBY_EXP);
        let hc = (t_env - 2.0 * skin_t.max(0.0)).max(0.0);
        let h_rib = (2.0 * rib_half_t.max(0.0)).min(hc);
        let t_mem = (t_env - hc) + rho_e * (hc - h_rib) + h_rib;
        let t3_bend = (t_env.powi(3) - hc.powi(3))
            + rho_e * (hc.powi(3) - h_rib.powi(3))
            + h_rib.powi(3);
        Self {
            t_env,
            t_mem: t_mem.max(0.05),
            t3_bend: t3_bend.max(0.05_f64.powi(3)),
        }
    }

    /// Integrate `SandwichSection` directly from a 1D voxel column along $z$.
    pub fn from_voxel_column(
        solid_z: &[bool],
        cav_z: &[bool],
        z_coords: &[f64],
        h: f64,
        rho: f64,
        rib_half_t: f64,
    ) -> Self {
        debug_assert_eq!(solid_z.len(), cav_z.len());
        debug_assert_eq!(solid_z.len(), z_coords.len());
        let rho_e = rho.clamp(0.0, 1.0).powf(GIBSON_ASHBY_EXP);
        let mut n_solid = 0usize;
        let mut t_mem = 0.0_f64;
        let mut i_yy = 0.0_f64;
        for ((&sol, &cav), &z) in solid_z.iter().zip(cav_z.iter()).zip(z_coords.iter()) {
            if !sol {
                continue;
            }
            n_solid += 1;
            let is_rib = rib_half_t > 0.0 && z.abs() <= rib_half_t;
            let w = if cav && !is_rib { rho_e } else { 1.0 };
            t_mem += w * h;
            i_yy += w * (z * z + (h * h) / 12.0) * h;
        }
        if n_solid == 0 {
            return Self::solid(0.3);
        }
        let t_env = (n_solid as f64 * h).max(0.3);
        let t_mem = t_mem.max(0.2);
        let t3_bend = (12.0 * i_yy).max(t_mem.powi(3));
        Self {
            t_env,
            t_mem,
            t3_bend,
        }
    }

    /// Effective section modulus $Z_{\text{eff}} = \frac{I_{\text{eff}}}{t_{\text{env}}/2} = \frac{t_{\text{bend}}^3}{6 t_{\text{env}}}$.
    #[inline]
    pub fn section_modulus(&self) -> f64 {
        self.t3_bend / (6.0 * self.t_env.max(1e-6))
    }
}

// ---------------- CST + DKT Sandwich Plate ----------------

const GP: [[f64; 3]; 3] = [
    [2.0 / 3.0, 1.0 / 6.0, 1.0 / 6.0],
    [1.0 / 6.0, 2.0 / 3.0, 1.0 / 6.0],
    [1.0 / 6.0, 1.0 / 6.0, 2.0 / 3.0],
];

#[inline]
fn a2_of(xs: &[f64; 3], ys: &[f64; 3]) -> f64 {
    (xs[1] - xs[0]) * (ys[2] - ys[0]) - (xs[2] - xs[0]) * (ys[1] - ys[0])
}

fn cst_b(xs: &[f64; 3], ys: &[f64; 3], a2: f64) -> [[f64; 6]; 3] {
    let mut b = [[0.0; 6]; 3];
    for j in 0..3 {
        let nx = (ys[(j + 1) % 3] - ys[(j + 2) % 3]) / a2;
        let ny = (xs[(j + 2) % 3] - xs[(j + 1) % 3]) / a2;
        b[0][2 * j] = nx;
        b[1][2 * j + 1] = ny;
        b[2][2 * j] = ny;
        b[2][2 * j + 1] = nx;
    }
    b
}

fn dkt_t(xs: &[f64; 3], ys: &[f64; 3]) -> [[f64; 9]; 12] {
    let mut t = [[0.0; 9]; 12];
    for i in 0..3 {
        t[i][3 * i + 1] = 1.0;
        t[6 + i][3 * i + 2] = 1.0;
    }
    for (m, (i, j)) in [(3usize, (1usize, 2usize)), (4, (2, 0)), (5, (0, 1))] {
        let (dx, dy) = (xs[j] - xs[i], ys[j] - ys[i]);
        let l = dx.hypot(dy);
        let (c, s) = (dx / l, dy / l);
        let mut ws = [0.0; 9];
        let mut wn = [0.0; 9];
        ws[3 * i] = -1.5 / l;
        ws[3 * j] = 1.5 / l;
        for k in [i, j] {
            ws[3 * k + 1] += -0.25 * c;
            ws[3 * k + 2] += -0.25 * s;
            wn[3 * k + 1] += -0.5 * s;
            wn[3 * k + 2] += 0.5 * c;
        }
        for q in 0..9 {
            t[m][q] = c * ws[q] - s * wn[q];
            t[6 + m][q] = s * ws[q] + c * wn[q];
        }
    }
    t
}

fn dkt_b(
    xs: &[f64; 3],
    ys: &[f64; 3],
    a2: f64,
    t: &[[f64; 9]; 12],
    l: [f64; 3],
) -> [[f64; 9]; 3] {
    let bq: [f64; 3] = std::array::from_fn(|i| (ys[(i + 1) % 3] - ys[(i + 2) % 3]) / a2);
    let cq: [f64; 3] = std::array::from_fn(|i| (xs[(i + 2) % 3] - xs[(i + 1) % 3]) / a2);
    let dn = |d: &[f64; 3]| -> [f64; 6] {
        [
            (4.0 * l[0] - 1.0) * d[0],
            (4.0 * l[1] - 1.0) * d[1],
            (4.0 * l[2] - 1.0) * d[2],
            4.0 * (l[1] * d[2] + l[2] * d[1]),
            4.0 * (l[2] * d[0] + l[0] * d[2]),
            4.0 * (l[0] * d[1] + l[1] * d[0]),
        ]
    };
    let (nx, ny) = (dn(&bq), dn(&cq));
    let mut b = [[0.0; 9]; 3];
    for q in 0..9 {
        let (mut bx_x, mut by_y, mut bx_y, mut by_x) = (0.0, 0.0, 0.0, 0.0);
        for k in 0..6 {
            bx_x += nx[k] * t[k][q];
            by_y += ny[k] * t[6 + k][q];
            bx_y += ny[k] * t[k][q];
            by_x += nx[k] * t[6 + k][q];
        }
        b[0][q] = bx_x;
        b[1][q] = by_y;
        b[2][q] = bx_y + by_x;
    }
    b
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

/// Assembled sandwich plate (5 DOFs/node: $u, v, w, w_x, w_y$) with independent per-element
/// membrane thickness $t_{\text{mem}}$ and bending rigidity $t_{\text{bend}}^3 / 12$.
pub struct SandwichPlate {
    pub mesh: Mesh,
    pub default_sec: SandwichSection,
    pub cmat: [[f64; 3]; 3],
    perm: Vec<usize>,
    pub sections: Vec<SandwichSection>,
}

pub struct SandwichSystem {
    sky: Skyline,
    k_orig: Vec<f64>,
    map: Vec<isize>,
}

impl SandwichPlate {
    pub fn new(mesh: Mesh, default_t: f64, e_mpa: f64, nu: f64) -> Self {
        let perm = rcm(mesh.xy.len(), &mesh.tris);
        Self {
            mesh,
            default_sec: SandwichSection::solid(default_t),
            cmat: c_plane(e_mpa, nu),
            perm,
            sections: Vec::new(),
        }
    }

    pub fn with_sections(mut self, sections: Vec<SandwichSection>) -> Self {
        assert_eq!(sections.len(), self.mesh.tris.len());
        self.sections = sections;
        self
    }

    #[inline]
    pub fn section_of(&self, e: usize) -> SandwichSection {
        self.sections.get(e).copied().unwrap_or(self.default_sec)
    }

    pub fn n(&self) -> usize {
        self.mesh.xy.len()
    }

    pub fn system(&self, fixed: &[bool]) -> Option<SandwichSystem> {
        let n = self.n();
        let mut map = vec![-1isize; n * 5];
        let mut nf = 0usize;
        for &node in &self.perm {
            for c in 0..5 {
                if !fixed[node * 5 + c] {
                    map[node * 5 + c] = nf as isize;
                    nf += 1;
                }
            }
        }
        if nf == 0 {
            return None;
        }
        let mut first: Vec<usize> = (0..nf).collect();
        for tri in &self.mesh.tris {
            let ids: Vec<usize> = tri
                .iter()
                .flat_map(|&nd| (0..5).map(move |c| nd * 5 + c))
                .filter_map(|d| (map[d] >= 0).then(|| map[d] as usize))
                .collect();
            if let Some(&mn) = ids.iter().min() {
                for &i in &ids {
                    first[i] = first[i].min(mn);
                }
            }
        }
        let mut sky = Skyline::new(nf, first);
        let c = self.cmat;
        for (e, tri) in self.mesh.tris.iter().enumerate() {
            let sec = self.section_of(e);
            let xs = [
                self.mesh.xy[tri[0]].0,
                self.mesh.xy[tri[1]].0,
                self.mesh.xy[tri[2]].0,
            ];
            let ys = [
                self.mesh.xy[tri[0]].1,
                self.mesh.xy[tri[1]].1,
                self.mesh.xy[tri[2]].1,
            ];
            let a2 = a2_of(&xs, &ys);
            let area = 0.5 * a2.abs();
            let bm = cst_b(&xs, &ys, a2);
            let mut km = [[0.0; 6]; 6];
            btcb(&bm, &c, sec.t_mem * area, &mut km);
            let tt = dkt_t(&xs, &ys);
            let mut kb = [[0.0; 9]; 9];
            for l in GP {
                let bb = dkt_b(&xs, &ys, a2, &tt, l);
                btcb(&bb, &c, area / 3.0 * sec.t3_bend / 12.0, &mut kb);
            }
            let idm: [isize; 6] = std::array::from_fn(|q| map[tri[q / 2] * 5 + q % 2]);
            let idb: [isize; 9] = std::array::from_fn(|q| map[tri[q / 3] * 5 + 2 + q % 3]);
            for a in 0..6 {
                if idm[a] < 0 {
                    continue;
                }
                for b in 0..=a {
                    if idm[b] >= 0 {
                        sky.add(idm[a] as usize, idm[b] as usize, km[a][b]);
                    }
                }
            }
            for a in 0..9 {
                if idb[a] < 0 {
                    continue;
                }
                for b in 0..=a {
                    if idb[b] >= 0 {
                        sky.add(idb[a] as usize, idb[b] as usize, kb[a][b]);
                    }
                }
            }
        }
        let k_orig = sky.a.clone();
        sky.factor().then_some(SandwichSystem { sky, k_orig, map })
    }

    /// Solve `K u = f` and verify the fail-closed residual / Oettli-Prager backward-error gate (`I6`):
    /// `rel_res <= 1e-10 || (rel_res <= floor_eps_absK_absu && comp_be <= 1e-12)`.
    pub fn solve_checked(&self, sys: &SandwichSystem, f: &[f64]) -> Option<(Vec<f64>, f64)> {
        let nf = sys.sky.n;
        let mut f_free = vec![0.0; nf];
        for (d, &g) in sys.map.iter().enumerate() {
            if g >= 0 {
                f_free[g as usize] += f[d];
            }
        }
        let mut b = f_free.clone();
        sys.sky.solve(&mut b);
        if b.iter().any(|x| !x.is_finite()) {
            return None;
        }
        let (rel_res, floor_eps, comp_be) =
            sys.sky.residual_and_backward_error(&sys.k_orig, &f_free, &b);
        let ok = rel_res.is_finite()
            && (rel_res <= 1e-10 || (rel_res <= floor_eps && comp_be <= 1e-12));
        if !ok {
            return None;
        }
        let mut u = vec![0.0; f.len()];
        for (d, &g) in sys.map.iter().enumerate() {
            if g >= 0 {
                u[d] = b[g as usize];
            }
        }
        Some((u, rel_res))
    }

    /// Multi-RHS checked solve reusing one Cholesky factor (`S5-M5`: returns `None` if any RHS fails `solve_checked`).
    pub fn solve_many_checked(&self, sys: &SandwichSystem, fs: &[&[f64]]) -> Option<Vec<Vec<f64>>> {
        let mut out = Vec::with_capacity(fs.len());
        for f in fs {
            let (u, _) = self.solve_checked(sys, f)?;
            out.push(u);
        }
        Some(out)
    }

    /// Per-element `(membrane_vm, outer_skin_bending_vm)` in MPa per unit load.
    /// Outer-skin bending stress is evaluated at $z = \pm t_{\text{env}}/2$:
    /// $\boldsymbol{\sigma}_{\text{skin}}^{\text{bend}} = \frac{t_{\text{env}}}{2} \mathbf{C} \boldsymbol{\kappa} = \frac{\mathbf{M} (t_{\text{env}}/2)}{I_{\text{eff}}}$.
    pub fn stresses(&self, u: &[f64]) -> (Vec<f64>, Vec<f64>) {
        let mut sm = Vec::with_capacity(self.mesh.tris.len());
        let mut sb = Vec::with_capacity(self.mesh.tris.len());
        for (e, tri) in self.mesh.tris.iter().enumerate() {
            let sec = self.section_of(e);
            let (s_m, s_b) = self.elem_stress_section(tri, u, sec);
            sm.push(von_mises(s_m));
            sb.push(
                corners(&s_b)
                    .iter()
                    .map(|s| von_mises(*s))
                    .fold(0.0, f64::max),
            );
        }
        (sm, sb)
    }

    pub fn elem_stress_section(
        &self,
        tri: &[usize; 3],
        u: &[f64],
        sec: SandwichSection,
    ) -> ([f64; 3], [[f64; 3]; 3]) {
        let xs = [
            self.mesh.xy[tri[0]].0,
            self.mesh.xy[tri[1]].0,
            self.mesh.xy[tri[2]].0,
        ];
        let ys = [
            self.mesh.xy[tri[0]].1,
            self.mesh.xy[tri[1]].1,
            self.mesh.xy[tri[2]].1,
        ];
        let a2 = a2_of(&xs, &ys);
        let c = &self.cmat;
        let um: [f64; 6] = std::array::from_fn(|q| u[tri[q / 2] * 5 + q % 2]);
        let ub: [f64; 9] = std::array::from_fn(|q| u[tri[q / 3] * 5 + 2 + q % 3]);
        let bm = cst_b(&xs, &ys, a2);
        let mut e = [0.0; 3];
        for r in 0..3 {
            for q in 0..6 {
                e[r] += bm[r][q] * um[q];
            }
        }
        let s_m = [
            c[0][0] * e[0] + c[0][1] * e[1],
            c[1][0] * e[0] + c[1][1] * e[1],
            c[2][2] * e[2],
        ];
        let tt = dkt_t(&xs, &ys);
        let mut s_b = [[0.0; 3]; 3];
        let h_skin = sec.t_env * 0.5;
        for (g, l) in GP.iter().enumerate() {
            let bb = dkt_b(&xs, &ys, a2, &tt, *l);
            let mut k = [0.0; 3];
            for r in 0..3 {
                for q in 0..9 {
                    k[r] += bb[r][q] * ub[q];
                }
            }
            s_b[g] = [
                h_skin * (c[0][0] * k[0] + c[0][1] * k[1]),
                h_skin * (c[1][0] * k[0] + c[1][1] * k[1]),
                h_skin * c[2][2] * k[2],
            ];
        }
        (s_m, s_b)
    }

    pub fn centroid(&self, e: usize) -> P2 {
        let t = self.mesh.tris[e];
        let p = |i: usize| self.mesh.xy[t[i]];
        (
            (p(0).0 + p(1).0 + p(2).0) / 3.0,
            (p(0).1 + p(1).1 + p(2).1) / 3.0,
        )
    }
}

// ---------------- Ring T6 Submodel with 1.5 mm Distributed Load Band ----------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RingCaseResult {
    pub sigma_pk_per_n: f64,
    pub capacity_n: f64,
    pub at: [f64; 2],
    pub load_norm: f64,
    pub n_load_pts: usize,
    pub true_rel_res: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RingBandSummary {
    pub bearing: RingCaseResult,
    pub back: RingCaseResult,
    pub joint_k: f64,
    pub t_joint: f64,
    pub n_el: usize,
    pub n_dof: usize,
}

const T6_GP: [([f64; 3], f64); 3] = [
    ([2.0 / 3.0, 1.0 / 6.0, 1.0 / 6.0], 1.0 / 3.0),
    ([1.0 / 6.0, 2.0 / 3.0, 1.0 / 6.0], 1.0 / 3.0),
    ([1.0 / 6.0, 1.0 / 6.0, 2.0 / 3.0], 1.0 / 3.0),
];

/// Evaluate T6 quadratic shape functions $N_0 \dots N_5$ at area coordinates `l = [l0, l1, l2]`.
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

/// Solve the canonical ring bearing (`+x` on $r_q < R_b + 1.5\text{ mm}$, $|\theta| < 60^\circ$) and ring-back (`+x` on rear
/// $x_q < c_x - 0.7 R_{\text{od}}$ and $r_q > r_{\text{out}} - 1.5\text{ mm}$) cases on a `Mesh`
/// using 6-node quadratic `T6` membrane elements, per-quadrature-point thickness `t_at(xq, yq)` (`I2(d)`),
/// a 1.5 mm thickness-weighted load band (`fe2d.py` parity), and a fail-closed residual / backward-error gate (`I6`).
pub fn solve_ring_band_t6(
    mesh: &Mesh,
    t_elem: &[f64],
    t_at: &dyn Fn(f64, f64) -> f64,
    cx: f64,
    r_b: f64,
    r_od: f64,
    t_joint: f64,
    seat_x: (f64, f64),
) -> Option<RingBandSummary> {
    if mesh.tris.is_empty() || t_elem.len() != mesh.tris.len() {
        return None;
    }
    let t6 = T6::new(mesh, 2.0, E_TI64_MPA, NU).with_thickness(t_elem.to_vec());
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

    let assemble_band_load = |is_bearing: bool| -> Option<(Vec<f64>, f64, usize)> {
        // Owner 2026-10-25 (~02:15Z, fe2d.py v3): both handle-side bore bearing (+/-60 deg about theta=0)
        // and outer ring-back push in +x (`dirx = +1.0`).
        let dirx = 1.0_f64;
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
                    (rq < r_b + 1.5) && (thq.abs() < 60.0_f64.to_radians())
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
    let eval_case = |is_bearing: bool| -> Option<RingCaseResult> {
        let (f, lb, n_load_pts) = assemble_band_load(is_bearing)?;
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
        let cap_n = (YIELD_TI64_MPA / best_vm) * jk;
        Some(RingCaseResult {
            sigma_pk_per_n: best_vm,
            capacity_n: cap_n,
            at: best_at,
            load_norm: lb,
            n_load_pts,
            true_rel_res: rel_res,
        })
    };

    let bearing = eval_case(true)?;
    let back = eval_case(false)?;
    Some(RingBandSummary {
        bearing,
        back,
        joint_k: jk,
        t_joint,
        n_el: mesh.tris.len(),
        n_dof: n6 * 2,
    })
}

// ---------------- Fail-Closed G3 3D TET10 Solid Tip & GPU Stage-2 Solver ----------------

/// Recompute `(rel_res, floor_eps_absK_absu, componentwise_backward_error)` in Rust for a full CSR matrix (`I6` + `03:33:07Z` floor check).
/// - `rel_res = ||f - K u||_2 / ||f||_2`
/// - `floor_eps_absK_absu = eps_f64 * || |K| |u| ||_2 / ||f||_2`
/// - `componentwise_backward_error = max_i |r_i| / ((|K| |u|)_i + |f_i| + 1e-300)` (Oettli-Prager)
pub fn csr_residual_and_backward_error_f64(
    indptr: &[usize],
    indices: &[usize],
    data: &[f64],
    f: &[f64],
    u: &[f64],
) -> Option<(f64, f64, f64)> {
    let n = f.len();
    if u.len() != n || indptr.len() != n + 1 {
        return None;
    }
    if u.iter().any(|x| !x.is_finite()) || f.iter().any(|x| !x.is_finite()) {
        return None;
    }
    let mut r_norm2 = 0.0_f64;
    let mut f_norm2 = 0.0_f64;
    let mut abs_ku_norm2 = 0.0_f64;
    let mut max_comp_be = 0.0_f64;
    for i in 0..n {
        let mut ku_i = 0.0_f64;
        let mut abs_ku_i = 0.0_f64;
        for idx in indptr[i]..indptr[i + 1] {
            let v = data[idx];
            let uj = u[indices[idx]];
            ku_i += v * uj;
            abs_ku_i += v.abs() * uj.abs();
        }
        let ri = f[i] - ku_i;
        r_norm2 += ri * ri;
        f_norm2 += f[i] * f[i];
        abs_ku_norm2 += abs_ku_i * abs_ku_i;
        let comp_be = ri.abs() / (abs_ku_i + f[i].abs() + 1e-300);
        if comp_be > max_comp_be {
            max_comp_be = comp_be;
        }
    }
    if f_norm2 <= 1e-300 {
        return None;
    }
    let f_norm = f_norm2.sqrt();
    let rel = r_norm2.sqrt() / f_norm;
    let floor_eps = f64::EPSILON * abs_ku_norm2.sqrt() / f_norm;
    (rel.is_finite() && floor_eps.is_finite() && max_comp_be.is_finite())
        .then_some((rel, floor_eps, max_comp_be))
}

/// Recompute the true float64 relative residual `||K * u - f||_2 / ||f||_2` in Rust for a full CSR matrix (`I6`).
pub fn csr_true_rel_residual_f64(
    indptr: &[usize],
    indices: &[usize],
    data: &[f64],
    f: &[f64],
    u: &[f64],
) -> Option<f64> {
    csr_residual_and_backward_error_f64(indptr, indices, data, f, u).map(|(rel, _, _)| rel)
}

/// Strict fail-closed 3D `TET10` tip submodel result (`HPC_REPORT.md` §4 & `T11` compliance).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrictSolidTipResult {
    pub solver_algorithm: String,
    pub sf_nodal_max: f64,
    pub sf_nodal_avg: f64,
    pub sf_elem_mean: f64,
    pub peak_nodal_max_mpa: f64,
    pub peak_nodal_avg_mpa: f64,
    pub peak_elem_mean_mpa: f64,
    pub true_rel_res: f64,
    pub n_tet: usize,
    pub n_dof: usize,
}

/// Run the 3D `TET10` tip submodel on a thickness map sampled directly from an explicit surface
/// thickness closure `thickness_fn(x, y)` and enforce a strict fail-closed residual check (`<= 1e-10`).
pub fn solve_g3_solid_tet10_strict(
    xy: Vec<(f64, f64)>,
    tris: Vec<[usize; 3]>,
    tip_xy: (f64, f64),
    t_joint: f64,
    x0: f64,
    y0: f64,
    g: f64,
    nx: usize,
    ny: usize,
    thickness_fn: &dyn Fn(f64, f64) -> Option<f64>,
    nsub: usize,
) -> Option<StrictSolidTipResult> {
    let mut th = Vec::with_capacity(nx * ny);
    for j in 0..ny {
        let y = y0 + (j as f64) * g;
        for i in 0..nx {
            let x = x0 + (i as f64) * g;
            th.push(thickness_fn(x, y));
        }
    }
    let field = jarambit_physics::solid_tet10::ThicknessField {
        x0,
        y0,
        g,
        nx,
        ny,
        th,
    };
    let input = jarambit_physics::solid_tet10::SolidTipInput {
        xy,
        tris,
        tip_xy,
        t_joint,
        field,
        r_cut: 30.0,
        nl: 4,
        nsub,
        patch_mm: 2.0,
    };
    let res = jarambit_physics::solid_tet10::solve_solid_tet10(&input)?;
    // Fail-closed (`S5-3`): reject if PCG fallback was triggered (ksp_its > 1), if non-finite, or if true
    // residual exceeds 1e-10.
    if res.ksp_its != 1
        || !res.rel_res.is_finite()
        || res.rel_res > 1e-10
        || !res.sf_solid_nodal_max.is_finite()
        || res.sf_solid_nodal_max <= 0.0
    {
        return None;
    }
    Some(StrictSolidTipResult {
        solver_algorithm: "faer_sparse_cholesky".to_string(),
        sf_nodal_max: res.sf_solid_nodal_max,
        sf_nodal_avg: res.sf_solid_nodal_avg,
        sf_elem_mean: res.sf_solid_elem_mean,
        peak_nodal_max_mpa: res.nodal_max_peak.mpa,
        peak_nodal_avg_mpa: res.nodal_avg_peak.mpa,
        peak_elem_mean_mpa: res.elem_mean_peak_mpa,
        true_rel_res: res.rel_res,
        n_tet: res.n_tet,
        n_dof: res.n_dof,
    })
}

// ---------------- V-S1: Fail-Closed 3D TET10 Ring Pull & Ring-Back Submodel ----------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ring3dTet10Case {
    pub peak_mpa: f64,
    pub capacity_n: f64,
    pub at: [f64; 3],
    pub true_rel_res: f64,
    pub n_load_pts: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ring3dTet10Summary {
    pub bearing: Ring3dTet10Case,
    pub pull: Ring3dTet10Case,
    pub back: Ring3dTet10Case,
    pub joint_k: f64,
    pub t_joint: f64,
    pub n_tet: usize,
    pub n_dof: usize,
    pub n_free_dof: usize,
}

fn vs1_tets_of_prism(v: [usize; 6]) -> [[usize; 4]; 3] {
    let mut i0 = 0usize;
    for i in 1..6 {
        if v[i] < v[i0] {
            i0 = i;
        }
    }
    let rot: [usize; 6] = match i0 {
        0 => [0, 1, 2, 3, 4, 5],
        1 => [1, 2, 0, 4, 5, 3],
        2 => [2, 0, 1, 5, 3, 4],
        3 => [3, 5, 4, 0, 2, 1],
        4 => [4, 3, 5, 1, 0, 2],
        _ => [5, 4, 3, 2, 1, 0],
    };
    let w = [v[rot[0]], v[rot[1]], v[rot[2]], v[rot[3]], v[rot[4]], v[rot[5]]];
    if w[1].min(w[5]) < w[2].min(w[4]) {
        [
            [w[0], w[1], w[2], w[5]],
            [w[0], w[1], w[5], w[4]],
            [w[0], w[4], w[5], w[3]],
        ]
    } else {
        [
            [w[0], w[1], w[2], w[4]],
            [w[0], w[4], w[2], w[5]],
            [w[0], w[4], w[5], w[3]],
        ]
    }
}

#[inline]
fn vs1_tet10_dn_dl(l: [f64; 4]) -> [[f64; 4]; 10] {
    let mut dn = [[0.0f64; 4]; 10];
    for a in 0..4 {
        dn[a][a] = 4.0 * l[a] - 1.0;
    }
    let edges = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
    for (k, &(i, j)) in edges.iter().enumerate() {
        dn[4 + k][i] = 4.0 * l[j];
        dn[4 + k][j] = 4.0 * l[i];
    }
    dn
}

#[inline]
fn vs1_tet_grad_l(p: [[f64; 3]; 4]) -> Option<([[f64; 3]; 4], f64)> {
    let j11 = p[1][0] - p[0][0];
    let j12 = p[2][0] - p[0][0];
    let j13 = p[3][0] - p[0][0];
    let j21 = p[1][1] - p[0][1];
    let j22 = p[2][1] - p[0][1];
    let j23 = p[3][1] - p[0][1];
    let j31 = p[1][2] - p[0][2];
    let j32 = p[2][2] - p[0][2];
    let j33 = p[3][2] - p[0][2];

    let c11 = j22 * j33 - j23 * j32;
    let c12 = -(j21 * j33 - j23 * j31);
    let c13 = j21 * j32 - j22 * j31;
    let det = j11 * c11 + j12 * c12 + j13 * c13;
    if det <= 1e-14 {
        return None;
    }
    let inv_det = 1.0 / det;
    let c21 = -(j12 * j33 - j13 * j32);
    let c22 = j11 * j33 - j13 * j31;
    let c23 = -(j11 * j32 - j12 * j31);
    let c31 = j12 * j23 - j13 * j22;
    let c32 = -(j11 * j23 - j13 * j21);
    let c33 = j11 * j22 - j12 * j21;

    let g1 = [c11 * inv_det, c21 * inv_det, c31 * inv_det];
    let g2 = [c12 * inv_det, c22 * inv_det, c32 * inv_det];
    let g3 = [c13 * inv_det, c23 * inv_det, c33 * inv_det];
    let g0 = [
        -(g1[0] + g2[0] + g3[0]),
        -(g1[1] + g2[1] + g3[1]),
        -(g1[2] + g2[2] + g3[2]),
    ];
    Some(([g0, g1, g2, g3], det / 6.0))
}

/// 2.5D extruded `TET10` quadratic-tetrahedron linear-elasticity solver (`comms.md` 02:25:54Z)
/// for the finger-ring + neck submodel under 1.5 mm radial-band load (bearing, pull, and ring-back),
/// with grip clamped in `x in [seat.0, seat.1]` (~20 mm from ring tangent) and a fail-closed Cholesky
/// relative residual gate `true_rel_res <= 1e-10`.
pub fn solve_ring_2p5d_extruded_tet10(
    mesh: &Mesh,
    t_at: &dyn Fn(f64, f64) -> f64,
    cx: f64,
    r_b: f64,
    r_od: f64,
    t_joint: f64,
    seat: (f64, f64),
    nl: usize,
) -> Option<Ring3dTet10Summary> {
    use faer::dyn_stack::{MemBuffer, MemStack};
    use faer::sparse::linalg::cholesky as ch;
    use faer::sparse::{SparseColMat, Triplet};

    let n2 = mesh.xy.len();
    if n2 < 3 || mesh.tris.is_empty() || nl == 0 {
        return None;
    }
    let hh: Vec<f64> = mesh
        .xy
        .iter()
        .map(|&(x, y)| t_at(x, y).max(0.20))
        .collect();
    let n_vert = (nl + 1) * n2;
    let mut pts = Vec::with_capacity(n_vert);
    for k in 0..=nl {
        let z_frac = (k as f64) / (nl as f64) - 0.5;
        for i in 0..n2 {
            pts.push([mesh.xy[i].0, mesh.xy[i].1, z_frac * hh[i]]);
        }
    }

    let mut tets = Vec::with_capacity(nl * mesh.tris.len() * 3);
    for k in 0..nl {
        let off0 = k * n2;
        let off1 = (k + 1) * n2;
        for &[a, b, c] in &mesh.tris {
            let prism = [off0 + a, off0 + b, off0 + c, off1 + a, off1 + b, off1 + c];
            for mut tet in vs1_tets_of_prism(prism) {
                let p0 = pts[tet[0]];
                let p1 = pts[tet[1]];
                let p2 = pts[tet[2]];
                let p3 = pts[tet[3]];
                let ux = p1[0] - p0[0];
                let uy = p1[1] - p0[1];
                let uz = p1[2] - p0[2];
                let vx = p2[0] - p0[0];
                let vy = p2[1] - p0[1];
                let vz = p2[2] - p0[2];
                let wx = p3[0] - p0[0];
                let wy = p3[1] - p0[1];
                let wz = p3[2] - p0[2];
                let vol6 = (uy * vz - uz * vy) * wx
                    + (uz * vx - ux * vz) * wy
                    + (ux * vy - uy * vx) * wz;
                if vol6 < 0.0 {
                    tet.swap(1, 2);
                }
                tets.push(tet);
            }
        }
    }

    let edge_pairs = [(0usize, 1usize), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
    let mut edge_map: std::collections::HashMap<(usize, usize), usize> =
        std::collections::HashMap::with_capacity(tets.len() * 2);
    let mut edges: Vec<(usize, usize)> = Vec::with_capacity(tets.len() * 2);
    let mut tet10_nodes: Vec<[usize; 10]> = Vec::with_capacity(tets.len());
    for tet in &tets {
        let mut nodes = [0usize; 10];
        nodes[0] = tet[0];
        nodes[1] = tet[1];
        nodes[2] = tet[2];
        nodes[3] = tet[3];
        for (k, &(i, j)) in edge_pairs.iter().enumerate() {
            let u = tet[i].min(tet[j]);
            let v = tet[i].max(tet[j]);
            let eid = *edge_map.entry((u, v)).or_insert_with(|| {
                let id = edges.len();
                edges.push((u, v));
                id
            });
            nodes[4 + k] = n_vert + eid;
        }
        tet10_nodes.push(nodes);
    }

    let n_nodes_p2 = n_vert + edges.len();
    let n_dof = 3 * n_nodes_p2;

    // Clamp nodes in grip seat x in [seat.0, seat.1]
    let mut fixed_node = vec![false; n_nodes_p2];
    for v in 0..n_vert {
        let x = pts[v][0];
        if x >= seat.0 && x <= seat.1 {
            fixed_node[v] = true;
        }
    }
    for (eid, &(u, v)) in edges.iter().enumerate() {
        let xm = 0.5 * (pts[u][0] + pts[v][0]);
        if (fixed_node[u] && fixed_node[v]) || (xm >= seat.0 && xm <= seat.1) {
            fixed_node[n_vert + eid] = true;
        }
    }

    let mut dof_map = vec![-1isize; n_dof];
    let mut n_free = 0usize;
    for nd in 0..n_nodes_p2 {
        if !fixed_node[nd] {
            for c in 0..3 {
                dof_map[3 * nd + c] = n_free as isize;
                n_free += 1;
            }
        }
    }
    if n_free == 0 {
        return None;
    }

    // 4-point degree-2 reference quadrature for TET10
    let alpha = (5.0 + 3.0 * 5.0f64.sqrt()) / 20.0;
    let beta = (5.0 - 5.0f64.sqrt()) / 20.0;
    let q_pts = [
        [alpha, beta, beta, beta],
        [beta, alpha, beta, beta],
        [beta, beta, alpha, beta],
        [beta, beta, beta, alpha],
    ];
    let q_dn: [[[f64; 4]; 10]; 4] = std::array::from_fn(|q| vs1_tet10_dn_dl(q_pts[q]));
    let mut w_ref = [[[[0.0f64; 4]; 4]; 10]; 10];
    for p in 0..10 {
        for r in 0..=p {
            for q in 0..4 {
                for a in 0..4 {
                    for b in 0..4 {
                        w_ref[p][r][a][b] += 0.25 * q_dn[q][p][a] * q_dn[q][r][b];
                    }
                }
            }
        }
    }

    let lam = E_TI64_MPA * NU / ((1.0 + NU) * (1.0 - 2.0 * NU));
    let mu = E_TI64_MPA / (2.0 * (1.0 + NU));

    let mut triplets: Vec<Triplet<usize, usize, f64>> = Vec::with_capacity(tets.len() * 180);
    let mut dmax = vec![0.0f64; n_free];

    for nodes in &tet10_nodes {
        let p_tet = [pts[nodes[0]], pts[nodes[1]], pts[nodes[2]], pts[nodes[3]]];
        let Some((grad_l, vol)) = vs1_tet_grad_l(p_tet) else {
            continue;
        };
        for p in 0..10 {
            let gp = [
                dof_map[3 * nodes[p]],
                dof_map[3 * nodes[p] + 1],
                dof_map[3 * nodes[p] + 2],
            ];
            if gp[0] < 0 {
                continue;
            }
            for r in 0..=p {
                let gr = [
                    dof_map[3 * nodes[r]],
                    dof_map[3 * nodes[r] + 1],
                    dof_map[3 * nodes[r] + 2],
                ];
                if gr[0] < 0 {
                    continue;
                }
                let mut tmp = [[0.0f64; 3]; 4];
                for a in 0..4 {
                    for j in 0..3 {
                        tmp[a][j] = w_ref[p][r][a][0] * grad_l[0][j]
                            + w_ref[p][r][a][1] * grad_l[1][j]
                            + w_ref[p][r][a][2] * grad_l[2][j]
                            + w_ref[p][r][a][3] * grad_l[3][j];
                    }
                }
                let mut m_blk = [[0.0f64; 3]; 3];
                for i in 0..3 {
                    for j in 0..3 {
                        m_blk[i][j] = vol
                            * (grad_l[0][i] * tmp[0][j]
                                + grad_l[1][i] * tmp[1][j]
                                + grad_l[2][i] * tmp[2][j]
                                + grad_l[3][i] * tmp[3][j]);
                    }
                }
                let tr_m = m_blk[0][0] + m_blk[1][1] + m_blk[2][2];
                for i in 0..3 {
                    let j_end = if p == r { i + 1 } else { 3 };
                    for j in 0..j_end {
                        let mut kij = lam * m_blk[i][j] + mu * m_blk[j][i];
                        if i == j {
                            kij += mu * tr_m;
                        }
                        let row = (gp[i] as usize).max(gr[j] as usize);
                        let col = (gp[i] as usize).min(gr[j] as usize);
                        if row == col {
                            dmax[row] += kij;
                        }
                        triplets.push(Triplet::new(row, col, kij));
                    }
                }
            }
        }
    }

    let k_mat = SparseColMat::<usize, f64>::try_new_from_triplets(n_free, n_free, &triplets).ok()?;
    drop(triplets);

    // Direct sparse supernodal Cholesky factorization with zero dynamic regularization (fail-closed on any non-SPD pivot, I6)
    let sym = ch::factorize_symbolic_cholesky(
        k_mat.symbolic(),
        faer::Side::Lower,
        Default::default(),
        Default::default(),
    )
    .ok()?;
    let _ = dmax;
    let par = faer::Par::Seq;
    let mut buf =
        MemBuffer::try_new(sym.factorize_numeric_llt_scratch::<f64>(par, Default::default()))
            .ok()?;
    let mut l_vals = vec![0.0f64; sym.len_val()];
    sym.factorize_numeric_llt::<f64>(
        &mut l_vals,
        k_mat.as_ref(),
        faer::Side::Lower,
        Default::default(),
        par,
        MemStack::new(&mut buf),
        Default::default(),
    )
    .ok()?;
    let llt = ch::LltRef::new(&sym, &l_vals);

    let corner_l = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let corner_dn: [[[f64; 4]; 10]; 4] = std::array::from_fn(|c| vs1_tet10_dn_dl(corner_l[c]));
    let jk = joint_k(t_joint);
    let sallow = YIELD_TI64_MPA * jk;

    // All P2 node coordinates for 1.5 mm band load evaluation
    let mut p2_xy = Vec::with_capacity(n_nodes_p2);
    for v in 0..n_vert {
        p2_xy.push((pts[v][0], pts[v][1]));
    }
    for &(u, v) in &edges {
        p2_xy.push((0.5 * (pts[u][0] + pts[v][0]), 0.5 * (pts[u][1] + pts[v][1])));
    }

    // mode: 0 = bearing (handle-side bore +/-60 deg about theta=0, pushing +x),
    //       1 = pull (rear bore +/-60 deg about theta=pi, pulling -x),
    //       2 = back (rear OD +/-45 deg about theta=pi, pushing +x).
    let solve_case = |mode: u8| -> Option<Ring3dTet10Case> {
        const W_BAND: f64 = 1.5;
        let mut f_free = vec![0.0f64; n_free];
        let mut load_sum = 0.0f64;
        let mut n_load_pts = 0usize;
        for (nd, &(px, py)) in p2_xy.iter().enumerate() {
            let gdof_x = dof_map[3 * nd];
            if gdof_x < 0 {
                continue;
            }
            let dx = px - cx;
            let dy = py;
            let r = dx.hypot(dy);
            let th_loc = t_at(px, py).max(0.20);
            if mode == 0 {
                // Handle-side bore bearing: within +/-60 deg of theta=0, pushing +x
                if dx <= 0.0 {
                    continue;
                }
                let th_ang = dy.atan2(dx).abs();
                let dr = r - r_b;
                if (0.0..=W_BAND).contains(&dr) && th_ang <= std::f64::consts::PI / 3.0 {
                    let w = th_ang.cos().max(0.0) * (1.0 - dr / W_BAND).max(0.0) * th_loc;
                    if w > 0.0 {
                        f_free[gdof_x as usize] += w;
                        load_sum += w;
                        n_load_pts += 1;
                    }
                }
            } else {
                if dx >= 0.0 {
                    continue;
                }
                let phi = dy.atan2(dx).rem_euclid(2.0 * std::f64::consts::PI);
                let d_ang = (phi - std::f64::consts::PI).abs();
                if mode == 1 {
                    let dr = r - r_b;
                    if (0.0..=W_BAND).contains(&dr) && d_ang <= std::f64::consts::PI / 3.0 {
                        let w = d_ang.cos().max(0.0) * (1.0 - dr / W_BAND).max(0.0) * th_loc;
                        if w > 0.0 {
                            f_free[gdof_x as usize] -= w;
                            load_sum += w;
                            n_load_pts += 1;
                        }
                    }
                } else {
                    let dr = r_od - r;
                    if (0.0..=W_BAND).contains(&dr) && d_ang <= std::f64::consts::PI / 4.0 {
                        let w =
                            (2.0 * d_ang).cos().max(0.0) * (1.0 - dr / W_BAND).max(0.0) * th_loc;
                        if w > 0.0 {
                            f_free[gdof_x as usize] += w;
                            load_sum += w;
                            n_load_pts += 1;
                        }
                    }
                }
            }
        }
        if load_sum <= 1e-9 || n_load_pts < 4 {
            return None;
        }
        for v in &mut f_free {
            *v /= load_sum;
        }

        let mut u_free = f_free.clone();
        let mut solve_buf = MemBuffer::new(sym.solve_in_place_scratch::<f64>(1, par));
        llt.solve_in_place_with_conj(
            faer::Conj::No,
            faer::MatMut::from_column_major_slice_mut(&mut u_free, n_free, 1),
            par,
            MemStack::new(&mut solve_buf),
        );

        // Compute true float64 relative residual ||K * u - f||_2 / ||f||_2 and Oettli-Prager componentwise backward error from lower-triangular k_mat
        let mut r_vec = f_free.clone();
        let mut abs_ku = vec![0.0f64; n_free];
        let sym_ref = k_mat.symbolic();
        let col_ptrs = sym_ref.col_ptr();
        let row_idxs = sym_ref.row_idx();
        let vals = k_mat.val();
        for col in 0..n_free {
            let uc = u_free[col];
            let auc = uc.abs();
            for idx in col_ptrs[col]..col_ptrs[col + 1] {
                let row = row_idxs[idx];
                let v = vals[idx];
                let av = v.abs();
                r_vec[row] -= v * uc;
                abs_ku[row] += av * auc;
                if row != col {
                    r_vec[col] -= v * u_free[row];
                    abs_ku[col] += av * u_free[row].abs();
                }
            }
        }
        let r_norm = r_vec.iter().map(|x| x * x).sum::<f64>().sqrt();
        let f_norm = f_free.iter().map(|x| x * x).sum::<f64>().sqrt().max(1e-30);
        let abs_ku_norm = abs_ku.iter().map(|x| x * x).sum::<f64>().sqrt();
        let true_rel_res = r_norm / f_norm;
        let floor_eps = f64::EPSILON * abs_ku_norm / f_norm;
        let comp_be = r_vec
            .iter()
            .zip(abs_ku.iter())
            .zip(f_free.iter())
            .map(|((&ri, &aku), &fi)| ri.abs() / (aku + fi.abs() + 1e-300))
            .fold(0.0_f64, f64::max);
        // Fail-closed residual gate: accept if true_rel_res <= 1e-10 OR if the relative residual is
        // below the float64 evaluation floor eps*|| |K||u| ||/||f|| AND componentwise backward error <= 1e-12
        let accepted = true_rel_res.is_finite()
            && (true_rel_res <= 1e-10 || (true_rel_res <= floor_eps && comp_be <= 1e-12));
        if !accepted {
            return None;
        }

        let mut u_full = vec![0.0f64; n_dof];
        for (d, &g) in dof_map.iter().enumerate() {
            if g >= 0 {
                u_full[d] = u_free[g as usize];
            }
        }

        let mut peak_mpa = 0.0f64;
        let mut peak_at = [0.0f64; 3];
        for nodes in &tet10_nodes {
            let p_tet = [pts[nodes[0]], pts[nodes[1]], pts[nodes[2]], pts[nodes[3]]];
            let Some((grad_l, _)) = vs1_tet_grad_l(p_tet) else {
                continue;
            };
            let ex = 0.25 * (p_tet[0][0] + p_tet[1][0] + p_tet[2][0] + p_tet[3][0]);
            let ey = 0.25 * (p_tet[0][1] + p_tet[1][1] + p_tet[2][1] + p_tet[3][1]);
            let ez = 0.25 * (p_tet[0][2] + p_tet[1][2] + p_tet[2][2] + p_tet[3][2]);
            let r_el = (ex - cx).hypot(ey);
            if ex >= seat.0 - 2.0 || (r_el - r_b).abs() < 0.65 || (r_od - r_el).abs() < 0.65 {
                continue;
            }
            let mut vm_sum = 0.0f64;
            for c_idx in 0..4 {
                let mut dudx = [[0.0f64; 3]; 3];
                for p in 0..10 {
                    let dn_l = &corner_dn[c_idx][p];
                    let gx = dn_l[0] * grad_l[0][0]
                        + dn_l[1] * grad_l[1][0]
                        + dn_l[2] * grad_l[2][0]
                        + dn_l[3] * grad_l[3][0];
                    let gy = dn_l[0] * grad_l[0][1]
                        + dn_l[1] * grad_l[1][1]
                        + dn_l[2] * grad_l[2][1]
                        + dn_l[3] * grad_l[3][1];
                    let gz = dn_l[0] * grad_l[0][2]
                        + dn_l[1] * grad_l[1][2]
                        + dn_l[2] * grad_l[2][2]
                        + dn_l[3] * grad_l[3][2];
                    let ux = u_full[3 * nodes[p]];
                    let uy = u_full[3 * nodes[p] + 1];
                    let uz = u_full[3 * nodes[p] + 2];
                    dudx[0][0] += ux * gx;
                    dudx[0][1] += ux * gy;
                    dudx[0][2] += ux * gz;
                    dudx[1][0] += uy * gx;
                    dudx[1][1] += uy * gy;
                    dudx[1][2] += uy * gz;
                    dudx[2][0] += uz * gx;
                    dudx[2][1] += uz * gy;
                    dudx[2][2] += uz * gz;
                }
                let exx = dudx[0][0];
                let eyy = dudx[1][1];
                let ezz = dudx[2][2];
                let exy = 0.5 * (dudx[0][1] + dudx[1][0]);
                let eyz = 0.5 * (dudx[1][2] + dudx[2][1]);
                let exz = 0.5 * (dudx[0][2] + dudx[2][0]);
                let tr = exx + eyy + ezz;
                let sxx = 2.0 * mu * exx + lam * tr;
                let syy = 2.0 * mu * eyy + lam * tr;
                let szz = 2.0 * mu * ezz + lam * tr;
                let sxy = 2.0 * mu * exy;
                let syz = 2.0 * mu * eyz;
                let sxz = 2.0 * mu * exz;
                let vm = (0.5 * ((sxx - syy).powi(2) + (syy - szz).powi(2) + (szz - sxx).powi(2))
                    + 3.0 * (sxy * sxy + syz * syz + sxz * sxz))
                    .max(0.0)
                    .sqrt();
                vm_sum += vm;
            }
            let vm_mean = vm_sum * 0.25;
            if vm_mean > peak_mpa {
                peak_mpa = vm_mean;
                peak_at = [ex, ey, ez];
            }
        }
        if peak_mpa <= 1e-12 || !peak_mpa.is_finite() {
            return None;
        }
        Some(Ring3dTet10Case {
            peak_mpa,
            capacity_n: sallow / peak_mpa,
            at: peak_at,
            true_rel_res,
            n_load_pts,
        })
    };

    let bearing = solve_case(0)?;
    let pull = solve_case(1)?;
    let back = solve_case(2)?;
    Some(Ring3dTet10Summary {
        bearing,
        pull,
        back,
        joint_k: jk,
        t_joint,
        n_tet: tets.len(),
        n_dof,
        n_free_dof: n_free,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use jarambit_physics::fine::{mesh_domain, Plate};

    #[test]
    fn solid_sandwich_section_matches_monolithic_plate_bit_for_bit() {
        let outer = vec![(0.0, -5.0), (40.0, -5.0), (40.0, 5.0), (0.0, 5.0)];
        let mesh = mesh_domain(&outer, &[], 2.0).expect("mesh");
        let ne = mesh.tris.len();
        let t_vec: Vec<f64> = (0..ne).map(|e| 4.0 + 0.01 * (e as f64)).collect();
        let sec_vec: Vec<SandwichSection> =
            t_vec.iter().map(|&t| SandwichSection::solid(t)).collect();

        let p_mono = Plate::new(mesh.clone(), 4.0, E_TI64_MPA, NU).with_thickness(t_vec);
        let p_sand = SandwichPlate::new(mesh.clone(), 4.0, E_TI64_MPA, NU).with_sections(sec_vec);

        let n = mesh.xy.len();
        let mut fixed = vec![false; n * 5];
        for (i, &(x, _)) in mesh.xy.iter().enumerate() {
            if x < 1.0 {
                for c in 0..5 {
                    fixed[i * 5 + c] = true;
                }
            }
        }
        let sys_mono = p_mono.system_with(&fixed, true).expect("mono factor");
        let sys_sand = p_sand.system(&fixed).expect("sand factor");

        let mut f = vec![0.0; n * 5];
        for (i, &(x, _)) in mesh.xy.iter().enumerate() {
            if x > 39.0 {
                f[i * 5] = 10.0;
                f[i * 5 + 2] = 5.0;
            }
        }
        let u_mono = p_mono.solve(&sys_mono, &f);
        let (u_sand, _) = p_sand.solve_checked(&sys_sand, &f).expect("solve_checked");
        for (a, b) in u_mono.iter().zip(u_sand.iter()) {
            assert!((a - b).abs() < 1e-12, "displacement mismatch: {a} vs {b}");
        }
        let (sm_m, sb_m) = p_mono.stresses(&u_mono);
        let (sm_s, sb_s) = p_sand.stresses(&u_sand);
        for ((am, bm), (as_, bs)) in sm_m.iter().zip(sb_m.iter()).zip(sm_s.iter().zip(sb_s.iter()))
        {
            assert!((am - as_).abs() < 1e-9);
            assert!(
                (bm - bs).abs() < 1e-9,
                "bending stress mismatch: {bm} vs {bs}"
            );
        }
    }

    #[test]
    fn hollow_sandwich_has_strictly_higher_bending_stiffness_and_lower_skin_stress_than_equal_mass_solid() {
        // 8 mm total envelope, 1.0 mm skins, 10 % TPMS infill -> true equal-mass solid thickness = 2*1.0 + 0.10*(8.0 - 2.0) = 2.60 mm (I10)
        let sand = SandwichSection::from_skin_and_lattice(8.0, 1.0, 0.10, 0.0);
        let t_equal_mass: f64 = 2.0 * 1.0 + 0.10 * (8.0 - 2.0 * 1.0);
        assert!((t_equal_mass - 2.60).abs() < 1e-12);
        let eq_mass_solid = SandwichSection::solid(t_equal_mass);
        assert!(
            sand.t3_bend > 5.0 * eq_mass_solid.t3_bend,
            "sandwich t3_bend ({}) must be >5x equal-mass solid t3_bend ({})",
            sand.t3_bend,
            eq_mass_solid.t3_bend
        );
        assert!(
            sand.section_modulus() > 2.5 * eq_mass_solid.section_modulus(),
            "sandwich Z_eff ({}) must exceed equal-mass solid Z_eff ({}) by >2.5x",
            sand.section_modulus(),
            eq_mass_solid.section_modulus()
        );

        // Verify in actual cantilever bending FEM
        let outer = vec![(0.0, -5.0), (50.0, -5.0), (50.0, 5.0), (0.0, 5.0)];
        let mesh = mesh_domain(&outer, &[], 2.0).expect("mesh");
        let ne = mesh.tris.len();
        let p_sand = SandwichPlate::new(mesh.clone(), 4.0, E_TI64_MPA, NU)
            .with_sections(vec![sand; ne]);
        let p_eq = SandwichPlate::new(mesh.clone(), 4.0, E_TI64_MPA, NU)
            .with_sections(vec![eq_mass_solid; ne]);

        let n = mesh.xy.len();
        let mut fixed = vec![false; n * 5];
        for (i, &(x, _)) in mesh.xy.iter().enumerate() {
            if x < 0.5 {
                for c in 0..5 {
                    fixed[i * 5 + c] = true;
                }
            }
        }
        let mut f = vec![0.0; n * 5];
        for (i, &(x, _)) in mesh.xy.iter().enumerate() {
            if x > 49.5 {
                f[i * 5 + 2] = 1.0;
            }
        }
        let (u_sand, _) = p_sand
            .solve_checked(&p_sand.system(&fixed).unwrap(), &f)
            .expect("solve_checked sand");
        let (u_eq, _) = p_eq
            .solve_checked(&p_eq.system(&fixed).unwrap(), &f)
            .expect("solve_checked eq");
        let (_, sb_sand) = p_sand.stresses(&u_sand);
        let (_, sb_eq) = p_eq.stresses(&u_eq);
        let pk_sand = sb_sand.iter().copied().fold(0.0, f64::max);
        let pk_eq = sb_eq.iter().copied().fold(0.0, f64::max);
        assert!(
            pk_sand < 0.45 * pk_eq,
            "hollow sandwich skin bending stress ({pk_sand}) must be <45% of equal-mass solid plate ({pk_eq})"
        );
    }

    #[test]
    fn ring_band_t6_solves_rounded_edge_ring_without_boundary_singularity() {
        // Annular ring centered at (0, 0) with bore R_b = 12.45, R_od = 16.0, + grip stub extending to x = 46
        let mut outer = Vec::new();
        let r_od = 16.0;
        let r_b = 12.45;
        for k in 0..48 {
            let th = (k as f64) * 2.0 * std::f64::consts::PI / 48.0;
            let (mut x, y) = (r_od * th.cos(), r_od * th.sin());
            if x > 10.0 && y.abs() < 11.0 {
                x = 46.0;
            }
            outer.push((x, y));
        }
        let hole: Vec<(f64, f64)> = (0..36)
            .map(|k| {
                let th = (k as f64) * 2.0 * std::f64::consts::PI / 36.0;
                (r_b * th.cos(), r_b * th.sin())
            })
            .collect();
        let mesh = mesh_domain(&outer, &[hole], 1.5).expect("ring mesh");
        // Rounded edge thickness profile: drops to 0.3 mm right at r = r_b and r = r_od, 8.0 mm in lip
        let t_at = |x: f64, y: f64| -> f64 {
            let r = x.hypot(y);
            if x > 14.0 {
                return 10.0;
            }
            let d_in = (r - r_b).max(0.0);
            let d_out = (r_od - r).max(0.0);
            let edge_d = d_in.min(d_out);
            (0.3 + 7.7 * (edge_d / 1.5).clamp(0.0, 1.0)).min(8.0)
        };
        let t_elem: Vec<f64> = mesh
            .tris
            .iter()
            .map(|t| {
                let cx = (mesh.xy[t[0]].0 + mesh.xy[t[1]].0 + mesh.xy[t[2]].0) / 3.0;
                let cy = (mesh.xy[t[0]].1 + mesh.xy[t[1]].1 + mesh.xy[t[2]].1) / 3.0;
                t_at(cx, cy)
            })
            .collect();

        let res = solve_ring_band_t6(&mesh, &t_elem, &t_at, 0.0, r_b, r_od, 3.0, (40.0, 46.0))
            .expect("ring band solve");
        assert!(
            res.bearing.capacity_n > 400.0 && res.bearing.capacity_n < 20_000.0,
            "bearing cap = {}",
            res.bearing.capacity_n
        );
        assert!(
            res.back.capacity_n > 250.0 && res.back.capacity_n < 5000.0,
            "back cap = {}",
            res.back.capacity_n
        );
    }

    #[test]
    fn csr_true_rel_residual_f64_verifies_exact_solution_and_rejects_perturbed_vector() {
        // 3x3 SPD tridiagonal matrix: [2 -1 0; -1 2 -1; 0 -1 2]
        let indptr = vec![0usize, 2, 5, 7];
        let indices = vec![0usize, 1, 0, 1, 2, 1, 2];
        let data = vec![2.0, -1.0, -1.0, 2.0, -1.0, -1.0, 2.0];
        let f = vec![1.0, 0.0, 1.0];
        let u_exact = vec![1.0, 1.0, 1.0];
        let res_ok =
            csr_true_rel_residual_f64(&indptr, &indices, &data, &f, &u_exact).expect("valid");
        assert!(res_ok <= 1e-15, "res_ok = {res_ok}");
        let u_bad = vec![1.0, 1.0 + 1e-4, 1.0];
        let res_bad =
            csr_true_rel_residual_f64(&indptr, &indices, &data, &f, &u_bad).expect("valid");
        assert!(res_bad > 1e-5, "res_bad = {res_bad}");
        assert!(csr_true_rel_residual_f64(&indptr, &indices, &data, &f, &[1.0, f64::NAN, 1.0]).is_none());
    }
}
