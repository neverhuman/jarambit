//! Geometry-changing smoothing on synthetic, nonweapon meshes. Coordinates: mm.
use std::collections::{BTreeMap, BTreeSet};

pub type Point = [f64; 3];
pub type Face = [usize; 3];
pub type Result<T> = std::result::Result<T, String>;

#[derive(Clone, Debug, PartialEq)]
pub struct Mesh {
    pub vertices: Vec<Point>,
    pub faces: Vec<Face>,
}

#[derive(Clone, Copy, Debug)]
pub struct Settings {
    pub iterations: usize,
    pub lambda: f64,
    pub mu: f64,
    pub max_displacement_mm: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            iterations: 20,
            lambda: 0.5,
            mu: -0.5,
            max_displacement_mm: 2.0,
        }
    }
}

impl Settings {
    pub fn validate(self) -> Result<()> {
        if self.iterations == 0 || self.iterations > 1000 {
            return Err("iterations must be in 1..=1000".into());
        }
        if !self.lambda.is_finite()
            || self.lambda <= 0.0
            || self.lambda > 1.0
            || !self.mu.is_finite()
            || self.mu >= 0.0
            || self.mu < -1.0
            || !self.max_displacement_mm.is_finite()
            || self.max_displacement_mm <= 0.0
        {
            return Err("invalid filter weights or displacement limit".into());
        }
        Ok(())
    }
}

pub fn add(a: Point, b: Point) -> Point {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
pub fn sub(a: Point, b: Point) -> Point {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
pub fn scale(a: Point, t: f64) -> Point {
    [a[0] * t, a[1] * t, a[2] * t]
}
pub fn dot(a: Point, b: Point) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
pub fn norm(a: Point) -> f64 {
    dot(a, a).sqrt()
}
pub fn cross(a: Point, b: Point) -> Point {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn normal(mesh: &Mesh, face: Face) -> Point {
    cross(
        sub(mesh.vertices[face[1]], mesh.vertices[face[0]]),
        sub(mesh.vertices[face[2]], mesh.vertices[face[0]]),
    )
}

struct Topology {
    neighbors: Vec<Vec<usize>>,
    boundary: Vec<bool>,
    edges: usize,
}

fn topology(mesh: &Mesh) -> Result<Topology> {
    if mesh.vertices.len() < 3
        || mesh.faces.is_empty()
        || mesh.vertices.iter().flatten().any(|x| !x.is_finite())
    {
        return Err("mesh is empty, too small, or nonfinite".into());
    }
    let mut neighbors = vec![BTreeSet::new(); mesh.vertices.len()];
    let mut edges: BTreeMap<(usize, usize), Vec<(usize, usize)>> = BTreeMap::new();
    let mut unique = BTreeSet::new();
    for &face in &mesh.faces {
        if face.iter().any(|&i| i >= mesh.vertices.len()) {
            return Err("face index out of range".into());
        }
        let mut key = face;
        key.sort();
        if key[0] == key[1]
            || key[1] == key[2]
            || !unique.insert(key)
            || !norm(normal(mesh, face)).is_finite()
            || norm(normal(mesh, face)) <= 1e-12
        {
            return Err("duplicate or degenerate face".into());
        }
        for (a, b) in [(face[0], face[1]), (face[1], face[2]), (face[2], face[0])] {
            neighbors[a].insert(b);
            neighbors[b].insert(a);
            edges.entry((a.min(b), a.max(b))).or_default().push((a, b));
        }
    }
    let mut boundary = vec![false; mesh.vertices.len()];
    for (&(a, b), incidents) in &edges {
        match incidents.as_slice() {
            [_] => {
                boundary[a] = true;
                boundary[b] = true;
            }
            [(u, v), (x, y)] if u == y && v == x => {}
            _ => return Err("nonmanifold edge or inconsistent winding".into()),
        }
    }
    let mut visited = vec![false; mesh.vertices.len()];
    let mut stack = vec![0];
    while let Some(i) = stack.pop() {
        if visited[i] {
            continue;
        }
        visited[i] = true;
        stack.extend(neighbors[i].iter().copied());
    }
    if visited.iter().any(|x| !x) {
        return Err("disconnected or unused vertices".into());
    }
    Ok(Topology {
        neighbors: neighbors
            .into_iter()
            .map(|x| x.into_iter().collect())
            .collect(),
        boundary,
        edges: edges.len(),
    })
}

/// Two simultaneous Laplacian passes per iteration; second pass uses first-pass
/// positions. Boundary vertices and explicit pins remain bitwise unchanged.
/// Total displacement is capped against the original input, not each pass.
/// No universal feature-preservation or nonintersection guarantee is made.
pub fn smooth(mesh: &Mesh, settings: Settings, pins: &[bool]) -> Result<Mesh> {
    settings.validate()?;
    let topo = topology(mesh)?;
    if pins.len() != mesh.vertices.len() {
        return Err("pin count differs from vertex count".into());
    }
    let fixed: Vec<bool> = pins
        .iter()
        .zip(&topo.boundary)
        .map(|(a, b)| *a || *b)
        .collect();
    let mut out = mesh.clone();
    for _ in 0..settings.iterations {
        for weight in [settings.lambda, settings.mu] {
            let old = &out.vertices;
            let next: Vec<Point> = old
                .iter()
                .enumerate()
                .map(|(i, &point)| {
                    if fixed[i] {
                        return point;
                    }
                    let mean = scale(
                        topo.neighbors[i]
                            .iter()
                            .fold([0.; 3], |sum, &j| add(sum, old[j])),
                        1.0 / topo.neighbors[i].len() as f64,
                    );
                    add(point, scale(sub(mean, point), weight))
                })
                .collect();
            out.vertices = next;
        }
        for (i, point) in out.vertices.iter_mut().enumerate() {
            let delta = sub(*point, mesh.vertices[i]);
            let distance = norm(delta);
            if distance > settings.max_displacement_mm {
                *point = add(
                    mesh.vertices[i],
                    scale(delta, settings.max_displacement_mm / distance),
                );
            }
        }
        topology(&out)?;
        if mesh
            .faces
            .iter()
            .any(|&face| dot(normal(mesh, face), normal(&out, face)) <= 0.0)
        {
            return Err("smoothing reversed a face relative to the input".into());
        }
    }
    Ok(out)
}

/// Icosphere housing benchmark. Analytic reference is a sphere, radius in mm.
pub fn sphere(subdivisions: usize, radius: f64, noise_mm: f64) -> Result<Mesh> {
    sphere_with_phase(subdivisions, radius, noise_mm, 0.0)
}

pub fn sphere_with_phase(
    subdivisions: usize,
    radius: f64,
    noise_mm: f64,
    phase: f64,
) -> Result<Mesh> {
    if !(1..=5).contains(&subdivisions)
        || !radius.is_finite()
        || radius <= 0.0
        || !phase.is_finite()
        || phase.abs() > 100.0
        || !noise_mm.is_finite()
        || noise_mm < 0.0
        || noise_mm > radius * 0.1
    {
        return Err(
            "sphere requires subdivisions 1..=5, positive radius, noise 0..=10% radius".into(),
        );
    }
    let t = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let mut mesh = Mesh {
        vertices: vec![
            [-1., t, 0.],
            [1., t, 0.],
            [-1., -t, 0.],
            [1., -t, 0.],
            [0., -1., t],
            [0., 1., t],
            [0., -1., -t],
            [0., 1., -t],
            [t, 0., -1.],
            [t, 0., 1.],
            [-t, 0., -1.],
            [-t, 0., 1.],
        ],
        faces: vec![
            [0, 11, 5],
            [0, 5, 1],
            [0, 1, 7],
            [0, 7, 10],
            [0, 10, 11],
            [1, 5, 9],
            [5, 11, 4],
            [11, 10, 2],
            [10, 7, 6],
            [7, 1, 8],
            [3, 9, 4],
            [3, 4, 2],
            [3, 2, 6],
            [3, 6, 8],
            [3, 8, 9],
            [4, 9, 5],
            [2, 4, 11],
            [6, 2, 10],
            [8, 6, 7],
            [9, 8, 1],
        ],
    };
    for p in &mut mesh.vertices {
        *p = scale(*p, 1.0 / norm(*p));
    }
    for _ in 0..subdivisions {
        let mut mids = BTreeMap::new();
        let mut faces = Vec::new();
        for [a, b, c] in mesh.faces.clone() {
            let mut midpoint = |u: usize, v: usize| -> usize {
                *mids.entry((u.min(v), u.max(v))).or_insert_with(|| {
                    let p = add(mesh.vertices[u], mesh.vertices[v]);
                    mesh.vertices.push(scale(p, 1.0 / norm(p)));
                    mesh.vertices.len() - 1
                })
            };
            let ab = midpoint(a, b);
            let bc = midpoint(b, c);
            let ca = midpoint(c, a);
            faces.extend([[a, ab, ca], [b, bc, ab], [c, ca, bc], [ab, bc, ca]]);
        }
        mesh.faces = faces;
    }
    for p in &mut mesh.vertices {
        let noise = ((37. * p[0] + 13. * p[1] - 19. * p[2] + phase).sin()
            + (29. * p[1] + 23. * p[2] + 11. * p[0] - 0.7 * phase).sin())
            * 0.5;
        *p = scale(*p, radius + noise_mm * noise);
    }
    topology(&mesh)?;
    Ok(mesh)
}

#[derive(Debug)]
pub struct Metrics {
    pub radial_rms_mm: f64,
    pub radial_p95_mm: f64,
    pub radial_max_mm: f64,
    pub mean_radius_mm: f64,
    pub volume_mm3: f64,
    pub min_double_area_mm2: f64,
    pub max_displacement_mm: f64,
    pub moved_vertices: usize,
    pub euler_characteristic: i64,
    pub boundary_vertices: usize,
}

pub fn metrics(mesh: &Mesh, source: &Mesh, radius: f64) -> Result<Metrics> {
    let topo = topology(mesh)?;
    if !radius.is_finite() || radius <= 0.0 {
        return Err("reference radius must be finite and positive".into());
    }
    if mesh.faces != source.faces || mesh.vertices.len() != source.vertices.len() {
        return Err("metrics require matching topology".into());
    }
    let mut errors: Vec<f64> = mesh
        .vertices
        .iter()
        .map(|&p| (norm(p) - radius).abs())
        .collect();
    errors.sort_by(f64::total_cmp);
    let displacements: Vec<f64> = mesh
        .vertices
        .iter()
        .zip(&source.vertices)
        .map(|(&a, &b)| norm(sub(a, b)))
        .collect();
    let volume = mesh
        .faces
        .iter()
        .map(|&[a, b, c]| dot(mesh.vertices[a], cross(mesh.vertices[b], mesh.vertices[c])) / 6.0)
        .sum::<f64>();
    let result = Metrics {
        radial_rms_mm: (errors.iter().map(|e| e * e).sum::<f64>() / errors.len() as f64).sqrt(),
        radial_p95_mm: errors[((errors.len() - 1) as f64 * 0.95).ceil() as usize],
        radial_max_mm: *errors.last().unwrap(),
        mean_radius_mm: mesh.vertices.iter().map(|&p| norm(p)).sum::<f64>()
            / mesh.vertices.len() as f64,
        volume_mm3: volume,
        min_double_area_mm2: mesh
            .faces
            .iter()
            .map(|&f| norm(normal(mesh, f)))
            .fold(f64::INFINITY, f64::min),
        max_displacement_mm: displacements.iter().copied().fold(0., f64::max),
        moved_vertices: displacements.iter().filter(|&&d| d > 1e-12).count(),
        euler_characteristic: mesh.vertices.len() as i64 - topo.edges as i64
            + mesh.faces.len() as i64,
        boundary_vertices: topo.boundary.iter().filter(|&&x| x).count(),
    };
    if [
        result.radial_rms_mm,
        result.radial_p95_mm,
        result.radial_max_mm,
        result.mean_radius_mm,
        result.volume_mm3,
        result.min_double_area_mm2,
        result.max_displacement_mm,
    ]
    .iter()
    .any(|x| !x.is_finite())
    {
        return Err("measurement overflow or nonfinite source coordinates".into());
    }
    Ok(result)
}

/// Deliberately declared before benchmark execution. These criteria apply to
/// this known sphere fixture only; no real CAD qualification follows from them.
pub fn benchmark_passes(
    before: &Metrics,
    after: &Metrics,
    reference: &Metrics,
    settings: Settings,
) -> bool {
    after.moved_vertices > 0
        && after.radial_rms_mm <= before.radial_rms_mm * 0.5
        && after.radial_p95_mm < before.radial_p95_mm
        && after.radial_max_mm <= before.radial_max_mm
        && (after.volume_mm3 / reference.volume_mm3 - 1.0).abs() <= 0.01
        && after.euler_characteristic == 2
        && after.boundary_vertices == 0
        && after.min_double_area_mm2 > 1e-12
        && after.max_displacement_mm <= settings.max_displacement_mm + 1e-12
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometric_error_reduces_on_three_noise_amplitudes() {
        let settings = Settings::default();
        for amplitude in [0.4, 0.8, 1.2] {
            let source = sphere(3, 30., amplitude).unwrap();
            let reference = sphere(3, 30., 0.).unwrap();
            let output = smooth(&source, settings, &vec![false; source.vertices.len()]).unwrap();
            let before = metrics(&source, &source, 30.).unwrap();
            let after = metrics(&output, &source, 30.).unwrap();
            let target = metrics(&reference, &reference, 30.).unwrap();
            assert!(
                benchmark_passes(&before, &after, &target, settings),
                "amplitude={amplitude}; {before:?}; {after:?}"
            );
            assert_eq!(source.faces, output.faces);
        }
    }

    #[test]
    fn pins_and_open_boundary_stay_exact_and_interior_moves() {
        let mut mesh = Mesh {
            vertices: Vec::new(),
            faces: Vec::new(),
        };
        for y in 0..5 {
            for x in 0..5 {
                mesh.vertices
                    .push([x as f64, y as f64, if x == 2 && y == 2 { 0.3 } else { 0. }]);
            }
        }
        for y in 0..4 {
            for x in 0..4 {
                let i = y * 5 + x;
                mesh.faces
                    .extend([[i, i + 1, i + 5], [i + 1, i + 6, i + 5]]);
            }
        }
        let mut pins = vec![false; 25];
        pins[6] = true;
        let original = mesh.clone();
        let out = smooth(&mesh, Settings::default(), &pins).unwrap();
        for y in 0..5 {
            for x in 0..5 {
                let i = y * 5 + x;
                if x == 0 || x == 4 || y == 0 || y == 4 || i == 6 {
                    assert_eq!(out.vertices[i], mesh.vertices[i]);
                }
            }
        }
        assert!(out.vertices[12][2] < mesh.vertices[12][2]);
        assert_eq!(mesh, original, "input mutated");
    }

    #[test]
    fn displacement_budget_is_total_and_deterministic() {
        let mesh = sphere(2, 30., 1.).unwrap();
        let settings = Settings {
            max_displacement_mm: 0.01,
            ..Settings::default()
        };
        let pins = vec![false; mesh.vertices.len()];
        let a = smooth(&mesh, settings, &pins).unwrap();
        assert_eq!(a, smooth(&mesh, settings, &pins).unwrap());
        let m = metrics(&a, &mesh, 30.).unwrap();
        assert!(m.max_displacement_mm <= 0.01 + 1e-12);
        assert!(m.moved_vertices > 0);
    }

    #[test]
    fn invalid_settings_and_pin_lengths_are_rejected() {
        let mesh = sphere(1, 30., 0.8).unwrap();
        let pins = vec![false; mesh.vertices.len()];
        for settings in [
            Settings {
                lambda: f64::NAN,
                ..Settings::default()
            },
            Settings {
                mu: 0.1,
                ..Settings::default()
            },
            Settings {
                iterations: 0,
                ..Settings::default()
            },
            Settings {
                iterations: 1001,
                ..Settings::default()
            },
            Settings {
                max_displacement_mm: f64::INFINITY,
                ..Settings::default()
            },
        ] {
            assert!(smooth(&mesh, settings, &pins).is_err());
        }
        assert!(smooth(&mesh, Settings::default(), &[]).is_err());
        assert!(sphere(6, 30., 1.).is_err());
        assert!(sphere(2, 30., f64::NAN).is_err());
        assert!(sphere(2, 1e100, 0.8).is_err());
        assert!(metrics(&mesh, &mesh, f64::NAN).is_err());
    }

    #[test]
    fn invalid_geometry_fails_without_output() {
        let mesh = sphere(1, 30., 0.8).unwrap();
        let mut bad = mesh.clone();
        bad.vertices[0][0] = f64::NAN;
        assert!(topology(&bad).is_err());
        bad = mesh.clone();
        bad.faces.push(bad.faces[0]);
        assert!(topology(&bad).is_err());
        bad = mesh.clone();
        bad.faces[0][0] = usize::MAX;
        assert!(topology(&bad).is_err());
        bad = mesh.clone();
        bad.faces[0].swap(0, 1);
        assert!(topology(&bad).is_err());
        bad = mesh.clone();
        bad.vertices.push([100., 100., 100.]);
        assert!(topology(&bad).is_err());
    }

    #[test]
    fn unchanged_geometry_cannot_pass_as_smoothing() {
        let mesh = sphere(3, 30., 0.8).unwrap();
        let reference = sphere(3, 30., 0.).unwrap();
        let before = metrics(&mesh, &mesh, 30.).unwrap();
        let target = metrics(&reference, &reference, 30.).unwrap();
        assert!(!benchmark_passes(
            &before,
            &before,
            &target,
            Settings::default()
        ));
    }

    #[test]
    fn held_out_roughness_phases_pass_frozen_defaults() {
        for phase in [0.73, 2.17] {
            let source = sphere_with_phase(3, 30., 0.8, phase).unwrap();
            let clean = sphere(3, 30., 0.).unwrap();
            let output = smooth(
                &source,
                Settings::default(),
                &vec![false; source.vertices.len()],
            )
            .unwrap();
            let before = metrics(&source, &source, 30.).unwrap();
            let after = metrics(&output, &source, 30.).unwrap();
            let reference = metrics(&clean, &clean, 30.).unwrap();
            assert!(
                benchmark_passes(&before, &after, &reference, Settings::default()),
                "phase={phase}; {after:?}"
            );
        }
    }

    #[test]
    fn translation_and_scale_commute_with_smoothing() {
        let source = sphere(2, 30., 0.8).unwrap();
        let pins = vec![false; source.vertices.len()];
        let output = smooth(&source, Settings::default(), &pins).unwrap();
        let shift = [12., -8., 4.];
        let mut transformed = source.clone();
        for point in &mut transformed.vertices {
            *point = add(scale(*point, 2.), shift);
        }
        let adjusted = Settings {
            max_displacement_mm: 4.,
            ..Settings::default()
        };
        let other = smooth(&transformed, adjusted, &pins).unwrap();
        for (&a, &b) in output.vertices.iter().zip(&other.vertices) {
            assert!(norm(sub(add(scale(a, 2.), shift), b)) < 1e-10);
        }
    }
}
