//! Voxel (hex8) linear-elastic finite elements on an occupancy grid. The grid, model, face, material and element
//! mathematics are copied verbatim from the voxel FE G5.0 vendored (`crates/g5/src/bin/g5_fem_fields/voxel_fe`,
//! `platform/voxel-fe` at 53e75771). The solve differs: G5.0 always solved the six unit wrenches and superposed
//! them; G5.1 solves the one wrench a load case applies (`solve_wrench`), and fails closed where G5.0 only reported:
//! a loaded face on material without support, or a load that does not reach the solve in full, is an error.
//!
//! Every voxel is one trilinear hexahedron of one declared material, or void; stress is recovered at element
//! centres. Peak values at staircase corners do not converge, so a case reads a volume quantile
//! (`Solution::reading`). Every sum has a fixed order, so results are bit-identical for any thread count.

use super::solver::{hierarchy, invert3, pcg, Level};
use std::thread;

pub type P3 = [f64; 3];

/// A box of `n` voxels of edge `h` (mm) whose lowest corner is `origin`.
#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    pub n: [usize; 3],
    pub h: f64,
    pub origin: P3,
}
impl Grid {
    pub fn elements(&self) -> usize {
        self.n[0] * self.n[1] * self.n[2]
    }
    pub fn nodes(&self) -> usize {
        (self.n[0] + 1) * (self.n[1] + 1) * (self.n[2] + 1)
    }
    pub fn element(&self, [i, j, k]: [usize; 3]) -> usize {
        i + self.n[0] * (j + self.n[1] * k)
    }
    pub fn element_index(&self, e: usize) -> [usize; 3] {
        let (a, b) = (self.n[0], self.n[1]);
        [e % a, (e / a) % b, e / (a * b)]
    }
    pub fn node(&self, [i, j, k]: [usize; 3]) -> usize {
        i + (self.n[0] + 1) * (j + (self.n[1] + 1) * k)
    }
    pub fn node_index(&self, v: usize) -> [usize; 3] {
        let (a, b) = (self.n[0] + 1, self.n[1] + 1);
        [v % a, (v / a) % b, v / (a * b)]
    }
    pub fn node_position(&self, v: usize) -> P3 {
        let ijk = self.node_index(v);
        std::array::from_fn(|c| self.origin[c] + self.h * ijk[c] as f64)
    }
    pub fn element_center(&self, e: usize) -> P3 {
        let ijk = self.element_index(e);
        std::array::from_fn(|c| self.origin[c] + self.h * (ijk[c] as f64 + 0.5))
    }
    /// Node of local corner `a` (bits: x + 2 y + 4 z) of element `e`.
    pub fn corner(&self, e: usize, a: usize) -> usize {
        let [i, j, k] = self.element_index(e);
        self.node([i + (a & 1), j + ((a >> 1) & 1), k + ((a >> 2) & 1)])
    }
    fn validate(&self) -> Result<(), String> {
        if self.n.contains(&0) || !(self.h.is_finite() && self.h > 0.) {
            return Err("invalid voxel grid".into());
        }
        if self.origin.iter().any(|v| !v.is_finite()) {
            return Err("nonfinite grid origin".into());
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub youngs_mpa: f64,
    pub poisson: f64,
}

/// `phase[e]` is 0 for void and m for `materials[m - 1]`.
#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub grid: Grid,
    pub phase: Vec<u8>,
    pub materials: Vec<Material>,
}
impl Model {
    pub fn validate(&self) -> Result<(), String> {
        self.grid.validate()?;
        if self.phase.len() != self.grid.elements() {
            return Err("phase length does not match the grid".into());
        }
        if self.materials.is_empty() || self.materials.len() > 255 {
            return Err("between 1 and 255 materials are required".into());
        }
        for m in &self.materials {
            let stiff = m.youngs_mpa.is_finite() && m.youngs_mpa > 0.;
            let poisson = m.poisson.is_finite() && m.poisson > -1. && m.poisson < 0.5;
            if !(stiff && poisson) {
                return Err(format!("invalid material {m:?}"));
            }
        }
        if self
            .phase
            .iter()
            .any(|&p| p as usize > self.materials.len())
        {
            return Err("phase refers to an undeclared material".into());
        }
        Ok(())
    }
}

/// One face of an element: `axis` 0..3 and `side` false for the low face, true for the high one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Face {
    pub element: usize,
    pub axis: usize,
    pub side: bool,
}

/// Boundary conditions shared by the six unit-wrench solves.
#[derive(Clone, Debug)]
pub struct Problem {
    /// Per node and component: displacement fixed at zero.
    pub fixed: Vec<[bool; 3]>,
    /// Faces carrying the wrench traction.
    pub loaded: Vec<Face>,
    /// Point the moments are taken about.
    pub reference: P3,
}
impl Problem {
    /// Fix every component of the nodes for which `fixed(position)` holds.
    pub fn new(grid: &Grid, fixed: impl Fn(P3) -> bool, loaded: Vec<Face>, reference: P3) -> Self {
        let fixed = (0..grid.nodes())
            .map(|v| [fixed(grid.node_position(v)); 3])
            .collect();
        Self {
            fixed,
            loaded,
            reference,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Options {
    pub relative_tolerance: f64,
    pub max_iterations: usize,
    pub threads: usize,
    /// Keep the six nodal displacement fields (3 per node) in the solution.
    pub keep_displacements: bool,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            relative_tolerance: 1e-8,
            max_iterations: 20_000,
            threads: thread::available_parallelism().map_or(1, |n| n.get().min(16)),
            keep_displacements: false,
        }
    }
}

/// What the solve left out, and why.
#[derive(Clone, Debug, PartialEq)]
pub struct Exclusion {
    /// Non-void elements not face-connected to any fixed node.
    pub unsupported_elements: usize,
    pub unsupported_volume_mm3: f64,
    /// Face-connected groups of non-void elements, all of them, and how many reach a support.
    pub components: usize,
    pub supported_components: usize,
    /// Loaded faces left without a supported element (their traction is not applied).
    pub unloaded_faces: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Convergence {
    pub iterations: usize,
    /// Final ||b - K x|| / ||b||, recomputed from the solution.
    pub relative_residual: f64,
}

pub fn von_mises(s: [f64; 6]) -> f64 {
    let [xx, yy, zz, yz, xz, xy] = s;
    (0.5 * ((xx - yy).powi(2) + (yy - zz).powi(2) + (zz - xx).powi(2))
        + 3. * (yz * yz + xz * xz + xy * xy))
        .sqrt()
}

/// Isotropic elasticity matrix for E = 1 (Voigt order xx, yy, zz, yz, xz, xy; engineering shear).
fn elasticity(nu: f64) -> [[f64; 6]; 6] {
    let l = nu / ((1. + nu) * (1. - 2. * nu));
    let m = 0.5 / (1. + nu);
    let mut d = [[0.; 6]; 6];
    for i in 0..3 {
        for j in 0..3 {
            d[i][j] = l;
        }
        d[i][i] = l + 2. * m;
        d[i + 3][i + 3] = m;
    }
    d
}
/// Strain-displacement matrix of the unit cube at local point `x`.
fn strain(x: P3) -> [[f64; 24]; 6] {
    let mut b = [[0.; 24]; 6];
    for a in 0..8 {
        let bit = |c: usize| (a >> c) & 1 == 1;
        let f = |c: usize| if bit(c) { x[c] } else { 1. - x[c] };
        let df = |c: usize| if bit(c) { 1. } else { -1. };
        let g = [
            df(0) * f(1) * f(2),
            f(0) * df(1) * f(2),
            f(0) * f(1) * df(2),
        ];
        let col = 3 * a;
        b[0][col] = g[0];
        b[1][col + 1] = g[1];
        b[2][col + 2] = g[2];
        b[3][col + 1] = g[2];
        b[3][col + 2] = g[1];
        b[4][col] = g[2];
        b[4][col + 2] = g[0];
        b[5][col] = g[1];
        b[5][col + 1] = g[0];
    }
    b
}
/// Unit-cube element stiffness for E = 1, exact 2 x 2 x 2 Gauss integration.
fn unit_stiffness(nu: f64) -> Vec<f64> {
    let d = elasticity(nu);
    let g = [0.5 - 0.5 / 3f64.sqrt(), 0.5 + 0.5 / 3f64.sqrt()];
    let mut k = vec![0.; 24 * 24];
    for p in 0..8 {
        let b = strain([g[p & 1], g[(p >> 1) & 1], g[(p >> 2) & 1]]);
        let mut db = [[0.; 24]; 6];
        for i in 0..6 {
            for j in 0..24 {
                db[i][j] = (0..6).map(|m| d[i][m] * b[m][j]).sum();
            }
        }
        for i in 0..24 {
            for j in 0..24 {
                k[24 * i + j] += 0.125 * (0..6).map(|m| b[m][i] * db[m][j]).sum::<f64>();
            }
        }
    }
    k
}

/// Union-find root with path halving.
fn root(parent: &mut [usize], mut a: usize) -> usize {
    while parent[a] != a {
        parent[a] = parent[parent[a]];
        a = parent[a];
    }
    a
}

/// The solution of one wrench.
#[derive(Clone, Debug)]
pub struct Solution {
    pub grid: Grid,
    /// Elements in the solve, in increasing order, with their centre stress (MPa; xx, yy, zz, yz, xz, xy).
    pub elements: Vec<usize>,
    pub stress: Vec<[f64; 6]>,
    /// Non-void elements left out of the solve because nothing connects them to a support.
    pub unsupported: Vec<usize>,
    pub exclusion: Exclusion,
    pub loaded_area_mm2: f64,
    pub convergence: Convergence,
    /// Nodal displacements (mm), when requested.
    pub displacement: Option<Vec<f64>>,
}

/// A von Mises reading of a load case over a set of elements.
#[derive(Clone, Debug, PartialEq)]
pub struct Reading {
    /// The von Mises stress exceeded by the given volume of the selected elements (the read stress).
    pub stress_mpa: f64,
    pub volume_mm3: f64,
    pub elements: usize,
    pub max_mpa: f64,
    pub q99_mpa: f64,
    pub q999_mpa: f64,
}

impl Solution {
    /// Von Mises over the solved elements whose centre passes `select`: the stress exceeded by `volume_mm3` of them
    /// (`None`: by 0.1 % of them, the G5.0 cross-check's 99.9 % quantile). Fails closed when an unsupported element
    /// lies in the selection (its stress is unknown, not zero) or nothing is selected.
    pub fn reading(&self, select: impl Fn(P3) -> bool, volume_mm3: Option<f64>) -> Result<Reading, String> {
        let g = &self.grid;
        if let Some(&e) = self.unsupported.iter().find(|&&e| select(g.element_center(e))) {
            return Err(format!("unsupported material in the read region (element {e} of {})", self.unsupported.len()));
        }
        let mut vm: Vec<f64> = self
            .elements
            .iter()
            .zip(&self.stress)
            .filter(|(&e, _)| select(g.element_center(e)))
            .map(|(_, &s)| von_mises(s))
            .collect();
        if vm.is_empty() {
            return Err("nothing to read".into());
        }
        vm.sort_by(f64::total_cmp);
        let n = vm.len();
        let quantile = |q: f64| vm[((q * n as f64).ceil() as usize).clamp(1, n) - 1];
        let v_el = g.h.powi(3);
        let stress = match volume_mm3 {
            Some(v) => {
                let k = ((v / v_el).ceil() as usize).clamp(1, n);
                vm[n - k]
            }
            None => quantile(0.999),
        };
        Ok(Reading {
            stress_mpa: stress,
            volume_mm3: n as f64 * v_el,
            elements: n,
            max_mpa: vm[n - 1],
            q99_mpa: quantile(0.99),
            q999_mpa: quantile(0.999),
        })
    }
}

/// Solve the wrench `w` (Fx, Fy, Fz in N; Mx, My, Mz in N mm about `problem.reference`) applied as the
/// traction-equivalent nodal forces of a linear traction over `problem.loaded` (G5.0's construction for each unit
/// wrench, applied once to `w`). Fails closed when a loaded face is on unsupported material, when part of the load
/// falls on fixed nodes (the applied resultant must equal the force), or when the solve does not converge.
pub fn solve_wrench(model: &Model, problem: &Problem, w: [f64; 6], options: &Options) -> Result<Solution, String> {
    model.validate()?;
    let g = &model.grid;
    if problem.fixed.len() != g.nodes() {
        return Err("fixed flags do not match the node count".into());
    }
    if problem.reference.iter().chain(w.iter()).any(|v| !v.is_finite()) {
        return Err("nonfinite wrench or moment reference".into());
    }
    if !(options.relative_tolerance > 0. && options.relative_tolerance < 1.) {
        return Err("invalid solver tolerance".into());
    }
    // Face-connected components of non-void elements; keep those touching a fixed node.
    let ne = g.elements();
    let mut parent: Vec<usize> = (0..ne).collect();
    for e in 0..ne {
        if model.phase[e] == 0 {
            continue;
        }
        let ijk = g.element_index(e);
        for axis in 0..3 {
            if ijk[axis] + 1 < g.n[axis] {
                let mut n = ijk;
                n[axis] += 1;
                let f = g.element(n);
                if model.phase[f] != 0 {
                    let (a, b) = (root(&mut parent, e), root(&mut parent, f));
                    if a != b {
                        parent[a.max(b)] = a.min(b);
                    }
                }
            }
        }
    }
    let mut supported = vec![false; ne];
    let mut component = vec![false; ne];
    for e in 0..ne {
        if model.phase[e] == 0 {
            continue;
        }
        let r = root(&mut parent, e);
        component[r] = true;
        if (0..8).any(|a| problem.fixed[g.corner(e, a)].iter().any(|&f| f)) {
            supported[r] = true;
        }
    }
    let mut kind = vec![0u8; ne];
    let mut unsupported = Vec::new();
    for e in 0..ne {
        if model.phase[e] == 0 {
            continue;
        }
        if supported[root(&mut parent, e)] {
            kind[e] = model.phase[e];
        } else {
            unsupported.push(e);
        }
    }
    let mut exclusion = Exclusion {
        unsupported_elements: unsupported.len(),
        unsupported_volume_mm3: unsupported.len() as f64 * g.h.powi(3),
        components: component.iter().filter(|&&c| c).count(),
        supported_components: (0..ne).filter(|&e| component[e] && supported[e]).count(),
        unloaded_faces: 0,
    };
    let blocks: Vec<Vec<f64>> = model.materials.iter().map(|m| unit_stiffness(m.poisson)).collect();
    let scale: Vec<f64> = (0..ne)
        .map(|e| if kind[e] == 0 { 0. } else { model.materials[kind[e] as usize - 1].youngs_mpa * g.h })
        .collect();
    let fixed: Vec<bool> = problem.fixed.iter().flatten().copied().collect();
    let fine = Level::new(g.clone(), scale, kind.iter().map(|&k| k.saturating_sub(1)).collect(), blocks, fixed);
    // Traction-equivalent nodal weights of the loaded faces (h^2 / 4 per corner).
    let mut weight = std::collections::BTreeMap::<usize, f64>::new();
    for f in &problem.loaded {
        if f.element >= ne || f.axis > 2 {
            return Err("loaded face outside the grid".into());
        }
        if kind[f.element] == 0 {
            exclusion.unloaded_faces += 1;
            continue;
        }
        for a in 0..8 {
            if ((a >> f.axis) & 1 == 1) == f.side {
                *weight.entry(g.corner(f.element, a)).or_default() += 0.25 * g.h * g.h;
            }
        }
    }
    if exclusion.unloaded_faces > 0 {
        return Err(format!(
            "{} of {} loaded faces are on material with no path to a support",
            exclusion.unloaded_faces,
            problem.loaded.len()
        ));
    }
    let area: f64 = weight.values().sum();
    if weight.is_empty() || area <= 0. {
        return Err("no loaded face".into());
    }
    let mut c = [0.; 3];
    for (&v, &a) in &weight {
        let x = g.node_position(v);
        for d in 0..3 {
            c[d] += a * x[d] / area;
        }
    }
    let mut j = [0.; 9];
    for (&v, &a) in &weight {
        let x = g.node_position(v);
        let d: P3 = std::array::from_fn(|t| x[t] - c[t]);
        let dd = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
        for r in 0..3 {
            for s in 0..3 {
                j[3 * r + s] += a * (if r == s { dd } else { 0. } - d[r] * d[s]);
            }
        }
    }
    let jinv = invert3(j);
    if jinv.iter().any(|v| !v.is_finite()) {
        return Err("loaded faces cannot carry a moment (collinear)".into());
    }
    let force = [w[0], w[1], w[2]];
    let arm: P3 = std::array::from_fn(|t| c[t] - problem.reference[t]);
    let mc = sub([w[3], w[4], w[5]], cross(arm, force));
    let wv: P3 = std::array::from_fn(|r| (0..3).map(|s| jinv[3 * r + s] * mc[s]).sum());
    let nd = fine.dofs();
    let mut b = vec![0.; nd];
    let (mut total, mut lost) = (0.0_f64, 0.0_f64);
    for (&v, &a) in &weight {
        let x = g.node_position(v);
        let d: P3 = std::array::from_fn(|t| x[t] - c[t]);
        let t = cross(wv, d);
        for comp in 0..3 {
            let f = a * (force[comp] / area + t[comp]);
            total += f.abs();
            if fine.free[3 * v + comp] {
                b[3 * v + comp] += f;
            } else {
                lost += f.abs();
            }
        }
    }
    if lost > 1e-12 * total {
        return Err(format!("{lost:.3e} of {total:.3e} N of nodal load falls on fixed nodes, so the load does not reach the solve in full"));
    }
    let coarse = hierarchy(&fine);
    let (x, convergence) = pcg(&fine, &coarse, &b, options)?;
    let elements: Vec<usize> = (0..ne).filter(|&e| kind[e] != 0).collect();
    let centre = strain([0.5; 3]);
    let stress = elements
        .iter()
        .map(|&e| {
            let m = model.materials[kind[e] as usize - 1];
            let d = elasticity(m.poisson);
            let mut eps = [0.; 6];
            for a in 0..8 {
                let v = g.corner(e, a);
                for comp in 0..3 {
                    let u = x[3 * v + comp];
                    for (r, row) in centre.iter().enumerate() {
                        eps[r] += row[3 * a + comp] * u / g.h;
                    }
                }
            }
            std::array::from_fn(|r| m.youngs_mpa * (0..6).map(|s| d[r][s] * eps[s]).sum::<f64>())
        })
        .collect();
    Ok(Solution {
        grid: g.clone(),
        elements,
        stress,
        unsupported,
        exclusion,
        loaded_area_mm2: area,
        convergence,
        displacement: options.keep_displacements.then_some(x),
    })
}

fn cross(a: P3, b: P3) -> P3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn sub(a: P3, b: P3) -> P3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
