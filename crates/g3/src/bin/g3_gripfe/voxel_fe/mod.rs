//! Vendored for the G3 v3.0.0 voxel FE certification: a verbatim copy of `platform/voxel-fe/src/lib.rs` at commit
//! 53e75771 (implant lane), with `crate::` changed to `super::`. Its controls are in that crate's tests/controls.rs.
//! Voxel (hex8) linear-elastic finite elements on an occupancy grid: a fast research screen for
//! any item whose geometry can be sampled, independent of surface meshing.
//!
//! - Every voxel is one trilinear hexahedron of one declared material, or void. Stress is
//!   recovered at element centres. Results are numerically unverified screens: a staircase
//!   boundary is not the item's surface, and peak values at re-entrant corners do not converge.
//!   Use volume quantiles, energies and ratios to a control in the same model, never a single
//!   element's stress as a strength claim.
//! - Loads are the six unit wrench components (Fx, Fy, Fz in N; Mx, My, Mz in N mm about a
//!   reference point) applied as the traction-equivalent nodal forces of a linear traction
//!   t(x) = F / A + w x (x - c) over declared element faces, which reproduces the wrench
//!   exactly. Any combined case is then a superposition of the six solutions.
//! - Material that is not face-connected to a support is excluded from the solve and reported:
//!   it carries no load in this model and would make the stiffness singular.
//! - Deterministic: every sum has a fixed order, so results are bit-identical for any thread
//!   count. The preconditioned conjugate-gradient solve fails closed when its true residual is
//!   above the requested tolerance.
mod solver;
use solver::{dot, hierarchy, invert3, pcg, Level};
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
/// The faces of elements of the given phases (every non-void one when `phases` is empty) that
/// lie on the plane `axis` = `index` of the node lattice, on the low side of layer `index`
/// (`side` false) or the high side of layer `index - 1` (`side` true).
pub fn faces_on_plane(
    model: &Model,
    axis: usize,
    index: usize,
    side: bool,
    phases: &[u8],
) -> Vec<Face> {
    let g = &model.grid;
    let layer = if side { index.wrapping_sub(1) } else { index };
    (0..g.elements())
        .filter(|&e| {
            let p = model.phase[e];
            p != 0
                && (phases.is_empty() || phases.contains(&p))
                && g.element_index(e)[axis] == layer
        })
        .map(|element| Face {
            element,
            axis,
            side,
        })
        .collect()
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

/// The six unit-wrench solutions: Fx, Fy, Fz (1 N), Mx, My, Mz (1 N mm about the reference).
#[derive(Clone, Debug)]
pub struct UnitSolution {
    pub grid: Grid,
    /// Elements in the solve, in increasing order.
    pub elements: Vec<usize>,
    /// Material of each solved element.
    pub phases: Vec<u8>,
    /// Centre stress (MPa; xx, yy, zz, yz, xz, xy) of each solved element for each unit load.
    pub stress: [Vec<[f64; 6]>; 6],
    /// u_i . f_j: the 6 x 6 flexibility at the loaded faces (mm / N, rad / (N mm)).
    pub flexibility: [[f64; 6]; 6],
    pub exclusion: Exclusion,
    pub loaded_area_mm2: f64,
    pub convergence: [Convergence; 6],
    /// Nodal displacements (mm) of the six unit loads, when requested.
    pub displacement: Option<[Vec<f64>; 6]>,
}

/// Statistics of one combined load case over a set of elements.
#[derive(Clone, Debug, PartialEq)]
pub struct CaseStats {
    /// Elastic energy of the whole model, 1/2 w . C w (N mm).
    pub energy_n_mm: f64,
    pub elements: usize,
    pub max_von_mises_mpa: f64,
    /// Von Mises stress exceeded by 1% and 0.1% of the selected volume.
    pub q99_von_mises_mpa: f64,
    pub q999_von_mises_mpa: f64,
    pub mean_von_mises_mpa: f64,
}

impl UnitSolution {
    /// Von Mises statistics of the wrench `w` (Fx, Fy, Fz, Mx, My, Mz) over the solved elements
    /// whose centre satisfies `select` and whose material is in `phases` (all when empty).
    /// None when no element is selected.
    pub fn case(
        &self,
        w: [f64; 6],
        select: impl Fn(P3) -> bool,
        phases: &[u8],
    ) -> Option<CaseStats> {
        let mut vm: Vec<f64> = self
            .elements
            .iter()
            .enumerate()
            .filter(|&(i, &e)| {
                (phases.is_empty() || phases.contains(&self.phases[i]))
                    && select(self.grid.element_center(e))
            })
            .map(|(i, _)| {
                let s: [f64; 6] =
                    std::array::from_fn(|c| (0..6).map(|k| w[k] * self.stress[k][i][c]).sum());
                von_mises(s)
            })
            .collect();
        if vm.is_empty() {
            return None;
        }
        let mean = vm.iter().sum::<f64>() / vm.len() as f64;
        vm.sort_by(f64::total_cmp);
        let quantile = |q: f64| vm[((q * vm.len() as f64).ceil() as usize).clamp(1, vm.len()) - 1];
        let mut energy = 0.;
        for i in 0..6 {
            for j in 0..6 {
                energy += 0.5 * w[i] * self.flexibility[i][j] * w[j];
            }
        }
        Some(CaseStats {
            energy_n_mm: energy,
            elements: vm.len(),
            max_von_mises_mpa: *vm.last().unwrap(),
            q99_von_mises_mpa: quantile(0.99),
            q999_von_mises_mpa: quantile(0.999),
            mean_von_mises_mpa: mean,
        })
    }
}

impl UnitSolution {
    /// Centre stress of solved element `i` (an index into `elements`) under wrench `w`.
    pub fn stress_of(&self, w: [f64; 6], i: usize) -> [f64; 6] {
        std::array::from_fn(|c| (0..6).map(|k| w[k] * self.stress[k][i][c]).sum())
    }
}
/// Strain (engineering shear, Voigt order) of an isotropic material carrying stress `s`.
pub fn strain_of(m: Material, s: [f64; 6]) -> [f64; 6] {
    let [xx, yy, zz, yz, xz, xy] = s;
    let (e, nu) = (m.youngs_mpa, m.poisson);
    let g = e / (2. * (1. + nu));
    [
        (xx - nu * (yy + zz)) / e,
        (yy - nu * (zz + xx)) / e,
        (zz - nu * (xx + yy)) / e,
        yz / g,
        xz / g,
        xy / g,
    ]
}
/// Equivalent (von Mises) strain of an engineering-shear strain: sqrt(2/3 e':e').
pub fn equivalent_strain(e: [f64; 6]) -> f64 {
    let [xx, yy, zz, yz, xz, xy] = e;
    let m = (xx + yy + zz) / 3.;
    let d = [xx - m, yy - m, zz - m];
    (2. / 3. * (d[0] * d[0] + d[1] * d[1] + d[2] * d[2] + 0.5 * (yz * yz + xz * xz + xy * xy)))
        .sqrt()
}
/// Octahedral shear strain (engineering): 2/3 sqrt((exx-eyy)^2 + (eyy-ezz)^2 + (ezz-exx)^2
/// + 3/2 (gyz^2 + gxz^2 + gxy^2)).
pub fn octahedral_shear_strain(e: [f64; 6]) -> f64 {
    let [xx, yy, zz, yz, xz, xy] = e;
    2. / 3.
        * ((xx - yy).powi(2)
            + (yy - zz).powi(2)
            + (zz - xx).powi(2)
            + 1.5 * (yz * yz + xz * xz + xy * xy))
            .sqrt()
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

/// Solve the six unit wrenches.
pub fn solve_unit_wrenches(
    model: &Model,
    problem: &Problem,
    options: &Options,
) -> Result<UnitSolution, String> {
    model.validate()?;
    let g = &model.grid;
    if problem.fixed.len() != g.nodes() {
        return Err("fixed flags do not match the node count".into());
    }
    if problem.reference.iter().any(|v| !v.is_finite()) {
        return Err("nonfinite moment reference".into());
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
    let mut exclusion = Exclusion {
        unsupported_elements: 0,
        unsupported_volume_mm3: 0.,
        components: component.iter().filter(|&&c| c).count(),
        supported_components: (0..ne).filter(|&e| component[e] && supported[e]).count(),
        unloaded_faces: 0,
    };
    for e in 0..ne {
        if model.phase[e] == 0 {
            continue;
        }
        if supported[root(&mut parent, e)] {
            kind[e] = model.phase[e];
        } else {
            exclusion.unsupported_elements += 1;
        }
    }
    exclusion.unsupported_volume_mm3 = exclusion.unsupported_elements as f64 * g.h.powi(3);
    let blocks: Vec<Vec<f64>> = model
        .materials
        .iter()
        .map(|m| unit_stiffness(m.poisson))
        .collect();
    let scale: Vec<f64> = (0..ne)
        .map(|e| {
            if kind[e] == 0 {
                0.
            } else {
                model.materials[kind[e] as usize - 1].youngs_mpa * g.h
            }
        })
        .collect();
    let fixed: Vec<bool> = problem.fixed.iter().flatten().copied().collect();
    let fine = Level::new(
        g.clone(),
        scale,
        kind.iter().map(|&k| k.saturating_sub(1)).collect(),
        blocks,
        fixed,
    );
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
    let area: f64 = weight.values().sum();
    if weight.is_empty() || area <= 0. {
        return Err(format!(
            "load path broken: no loaded face on material face-connected to a support ({} of {} components supported, {:.4} mm3 unsupported)",
            exclusion.supported_components, exclusion.components, exclusion.unsupported_volume_mm3
        ));
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
    let coarse = hierarchy(&fine);
    let nd = fine.dofs();
    let loads: Vec<Vec<f64>> = (0..6)
        .map(|k| {
            let mut force = [0.; 3];
            let mut moment = [0.; 3];
            if k < 3 {
                force[k] = 1.;
            } else {
                moment[k - 3] = 1.;
            }
            // Moment about the load centroid.
            let arm: P3 = std::array::from_fn(|t| c[t] - problem.reference[t]);
            let mc = sub(moment, cross(arm, force));
            let w: P3 = std::array::from_fn(|r| (0..3).map(|s| jinv[3 * r + s] * mc[s]).sum());
            let mut b = vec![0.; nd];
            for (&v, &a) in &weight {
                let x = g.node_position(v);
                let d: P3 = std::array::from_fn(|t| x[t] - c[t]);
                let t = cross(w, d);
                for comp in 0..3 {
                    if fine.free[3 * v + comp] {
                        b[3 * v + comp] += a * (force[comp] / area + t[comp]);
                    }
                }
            }
            b
        })
        .collect();
    // The six solves run side by side; each is deterministic for any thread count.
    let each = Options {
        threads: options.threads.div_ceil(6).max(1),
        ..options.clone()
    };
    let solved: Vec<Result<(Vec<f64>, Convergence), String>> = thread::scope(|s| {
        let handles: Vec<_> = loads
            .iter()
            .map(|b| s.spawn(|| pcg(&fine, &coarse, b, &each)))
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("solve panicked"))
            .collect()
    });
    let mut fields = Vec::with_capacity(6);
    let mut convergence = Vec::with_capacity(6);
    for (k, r) in solved.into_iter().enumerate() {
        let (x, conv) = r.map_err(|e| format!("unit load {k}: {e}"))?;
        fields.push(x);
        convergence.push(conv);
    }
    let flexibility: [[f64; 6]; 6] =
        std::array::from_fn(|i| std::array::from_fn(|j| dot(&fields[i], &loads[j])));
    let elements: Vec<usize> = (0..ne).filter(|&e| kind[e] != 0).collect();
    let centre = strain([0.5; 3]);
    let stress: [Vec<[f64; 6]>; 6] = std::array::from_fn(|k| {
        elements
            .iter()
            .map(|&e| {
                let m = model.materials[kind[e] as usize - 1];
                let d = elasticity(m.poisson);
                let mut eps = [0.; 6];
                for a in 0..8 {
                    let v = g.corner(e, a);
                    for comp in 0..3 {
                        let u = fields[k][3 * v + comp];
                        for (r, row) in centre.iter().enumerate() {
                            eps[r] += row[3 * a + comp] * u / g.h;
                        }
                    }
                }
                std::array::from_fn(|r| {
                    m.youngs_mpa * (0..6).map(|s| d[r][s] * eps[s]).sum::<f64>()
                })
            })
            .collect()
    });
    let phases = elements.iter().map(|&e| kind[e]).collect();
    let displacement = options
        .keep_displacements
        .then(|| fields.try_into().expect("six solves"));
    Ok(UnitSolution {
        grid: g.clone(),
        elements,
        phases,
        stress,
        flexibility,
        exclusion,
        loaded_area_mm2: area,
        convergence: convergence.try_into().expect("six solves"),
        displacement,
    })
}

fn cross(a: P3, b: P3) -> P3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn sub(a: P3, b: P3) -> P3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
