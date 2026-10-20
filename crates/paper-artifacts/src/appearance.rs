//! Appearance only: original triangle corners/order are immutable.
//! Selectively ported RawMesh/STL/angle-weighted corner fans from retained
//! 562e9c1e8f776c37a4a242e7ba545f15d5974639, appearance.rs SHA256
//! ac88ebfe5b179a566be629b14b327701b1bfe300f0d67d101edd7366572fcb37.
//! This adapter replaces the study camera/material/shadows/stride with explicit
//! caller camera, per-face base RGB, complete depth/coverage, and inward outline.
//! No geometry update, semantic-CAD inference, field recovery or fitness call.
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub type Vec3 = [f64; 3];
type Result<T> = std::result::Result<T, String>;

pub fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn sub(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn dot(a: Vec3, b: Vec3) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}
fn cross(a: Vec3, b: Vec3) -> Vec3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn unit(v: Vec3) -> Vec3 {
    let len = dot(v, v).sqrt();
    if len > 0.0 {
        v.map(|x| x / len)
    } else {
        [0.0; 3]
    }
}

/// Triangle soup in original order. Mutable position storage is not exposed.
#[derive(Clone, Debug)]
pub struct RawMesh {
    triangles: Vec<[Vec3; 3]>,
}
impl RawMesh {
    pub fn new(triangles: Vec<[Vec3; 3]>) -> Result<Self> {
        if triangles.is_empty() || triangles.len() > 1_000_000 {
            return Err("expected 1..=1000000 source triangles".into());
        }
        for (i, t) in triangles.iter().enumerate() {
            if t.iter().flatten().any(|x| !x.is_finite() || x.abs() > 1e12) {
                return Err(format!("invalid coordinate in triangle {i}"));
            }
            if dot(
                cross(sub(t[1], t[0]), sub(t[2], t[0])),
                cross(sub(t[1], t[0]), sub(t[2], t[0])),
            ) == 0.0
            {
                return Err(format!(
                    "degenerate source triangle {i}; comparison refused"
                ));
            }
        }
        Ok(Self { triangles })
    }
    pub fn triangles(&self) -> &[[Vec3; 3]] {
        &self.triangles
    }
    pub fn len(&self) -> usize {
        self.triangles.len()
    }
    pub fn is_empty(&self) -> bool {
        self.triangles.is_empty()
    }
    pub fn geometry_digest(&self) -> String {
        let mut h = Sha256::new();
        h.update(b"appearance.triangle_soup.f64le.v1\0");
        h.update((self.len() as u64).to_le_bytes());
        for x in self.triangles.iter().flatten().flatten() {
            h.update(x.to_le_bytes());
        }
        format!("{:x}", h.finalize())
    }
    pub fn flat_normals(&self) -> Vec<Vec3> {
        self.triangles
            .iter()
            .flat_map(|t| [unit(cross(sub(t[1], t[0]), sub(t[2], t[0]))); 3])
            .collect()
    }
}

/// Strict ASCII grammar or exact-length binary, including binary headers that
/// begin with `solid`. Never groups vertices across malformed ASCII facets.
pub fn decode_stl(bytes: &[u8]) -> Result<RawMesh> {
    if bytes.len() > 256 * 1024 * 1024 {
        return Err("STL exceeds 256 MiB audit limit".into());
    }
    if bytes.len() >= 84 {
        let count = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
        if count > 0 && count <= 1_000_000 && 84 + 50 * count == bytes.len() {
            let mut ts = Vec::with_capacity(count);
            for face in bytes[84..].chunks_exact(50) {
                let mut t = [[0.0; 3]; 3];
                for (corner, p) in t.iter_mut().enumerate() {
                    for (d, x) in p.iter_mut().enumerate() {
                        let o = 12 + corner * 12 + d * 4;
                        *x = f32::from_le_bytes(face[o..o + 4].try_into().unwrap()) as f64;
                    }
                }
                ts.push(t);
            }
            return RawMesh::new(ts);
        }
    }
    let text = std::str::from_utf8(bytes).map_err(|_| "invalid binary length or ASCII encoding")?;
    let mut lines = text.lines().map(str::trim).filter(|s| !s.is_empty());
    let first = lines.next().ok_or("empty STL")?;
    if first.split_whitespace().next() != Some("solid") {
        return Err("invalid STL header".into());
    }
    fn numbers(line: &str, prefix: &str) -> Result<Vec3> {
        let mut words = line.split_whitespace();
        for expected in prefix.split_whitespace() {
            if words.next() != Some(expected) {
                return Err("unexpected ASCII STL record".into());
            }
        }
        let v: Vec<f64> = words
            .map(|x| {
                x.parse::<f64>()
                    .map_err(|_| "invalid STL number".to_string())
            })
            .collect::<Result<_>>()?;
        if v.len() != 3 || v.iter().any(|x| !x.is_finite()) {
            return Err("expected three finite STL numbers".into());
        }
        Ok([v[0], v[1], v[2]])
    }
    let mut ts = Vec::new();
    loop {
        let line = lines.next().ok_or("missing endsolid")?;
        if line.split_whitespace().next() == Some("endsolid") {
            if lines.next().is_some() {
                return Err("unexpected data after endsolid".into());
            }
            break;
        }
        numbers(line, "facet normal ")?;
        if lines.next() != Some("outer loop") {
            return Err("missing outer loop".into());
        }
        let mut t = [[0.0; 3]; 3];
        for p in &mut t {
            *p = numbers(lines.next().ok_or("truncated facet")?, "vertex ")?;
        }
        if lines.next() != Some("endloop") || lines.next() != Some("endfacet") {
            return Err("malformed facet ending".into());
        }
        ts.push(t);
        if ts.len() > 1_000_000 {
            return Err("triangle budget exceeded".into());
        }
    }
    RawMesh::new(ts)
}

#[derive(Clone, Copy, Debug)]
pub struct NormalSettings {
    pub crease_degrees: f64,
    pub blend: f64,
    pub max_deviation_degrees: f64,
}
impl NormalSettings {
    pub fn validate(self) -> Result<()> {
        if !self.crease_degrees.is_finite()
            || !(0.0..=60.0).contains(&self.crease_degrees)
            || !self.blend.is_finite()
            || !(0.0..=1.0).contains(&self.blend)
            || !self.max_deviation_degrees.is_finite()
            || !(0.0..=30.0).contains(&self.max_deviation_degrees)
        {
            return Err(
                "invalid appearance settings: crease 0..60, blend 0..1, deviation 0..30".into(),
            );
        }
        Ok(())
    }
}
#[derive(Default, Debug)]
pub struct NormalStats {
    pub joined_edges: usize,
    pub blocked_edges: usize,
    pub ambiguous_edges: usize,
    pub duplicate_faces: usize,
    pub capped_corners: usize,
    pub changed_corners: usize,
    pub max_deviation_degrees: f64,
}
struct Dsu(Vec<usize>);
impl Dsu {
    fn root(&mut self, x: usize) -> usize {
        let mut r = x;
        while self.0[r] != r {
            r = self.0[r];
        }
        let mut cursor = x;
        while self.0[cursor] != cursor {
            let next = self.0[cursor];
            self.0[cursor] = r;
            cursor = next;
        }
        r
    }
    fn join(&mut self, a: usize, b: usize) {
        let (a, b) = (self.root(a), self.root(b));
        if a != b {
            self.0[a.max(b)] = a.min(b);
        }
    }
}
#[derive(Clone, Copy)]
struct HalfEdge {
    face: usize,
    a: usize,
    b: usize,
    from: usize,
    to: usize,
}

/// Returns normals only. Exact edges are an inferred adjacency, not proof of
/// CAD face identity. Production semantic labels remain a separate requirement.
pub fn derive_normals(
    mesh: &RawMesh,
    settings: NormalSettings,
    regions: Option<&[u32]>,
) -> Result<(Vec<Vec3>, NormalStats)> {
    settings.validate()?;
    if regions.is_some_and(|r| r.len() != mesh.len()) {
        return Err("region count must equal face count".into());
    }
    let flat = mesh.flat_normals();
    let mut vertices = BTreeMap::new();
    let mut ids = Vec::with_capacity(mesh.len());
    let mut faces: BTreeMap<[usize; 3], Vec<usize>> = BTreeMap::new();
    for (i, t) in mesh.triangles.iter().enumerate() {
        let mut v = [0; 3];
        for c in 0..3 {
            let key = t[c].map(|x| if x == 0.0 { 0 } else { x.to_bits() });
            let next = vertices.len();
            v[c] = *vertices.entry(key).or_insert(next);
        }
        ids.push(v);
        let mut key = v;
        key.sort();
        faces.entry(key).or_default().push(i);
    }
    let mut duplicate = vec![false; mesh.len()];
    for f in faces.values().filter(|f| f.len() > 1) {
        for i in f {
            duplicate[*i] = true;
        }
    }
    let mut edges: BTreeMap<[usize; 2], Vec<HalfEdge>> = BTreeMap::new();
    for (i, v) in ids.iter().enumerate() {
        for a in 0..3 {
            let b = (a + 1) % 3;
            edges
                .entry([v[a].min(v[b]), v[a].max(v[b])])
                .or_default()
                .push(HalfEdge {
                    face: i,
                    a,
                    b,
                    from: v[a],
                    to: v[b],
                });
        }
    }
    let mut stats = NormalStats {
        duplicate_faces: duplicate.iter().filter(|x| **x).count(),
        ..NormalStats::default()
    };
    let mut dsu = Dsu((0..3 * mesh.len()).collect());
    let cutoff = settings.crease_degrees.to_radians().cos();
    for hs in edges.values() {
        let valid = hs.len() == 2
            && hs[0].from == hs[1].to
            && hs[0].to == hs[1].from
            && !duplicate[hs[0].face]
            && !duplicate[hs[1].face];
        if !valid {
            stats.blocked_edges += 1;
            if hs.len() > 2
                || (hs.len() == 2
                    && (duplicate[hs[0].face] || duplicate[hs[1].face] || hs[0].from != hs[1].to))
            {
                stats.ambiguous_edges += 1;
            }
            continue;
        }
        let (a, b) = (hs[0], hs[1]);
        if regions.is_some_and(|r| r[a.face] != r[b.face])
            || dot(flat[3 * a.face], flat[3 * b.face]) < cutoff
        {
            stats.blocked_edges += 1;
            continue;
        }
        stats.joined_edges += 1;
        dsu.join(3 * a.face + a.a, 3 * b.face + b.b);
        dsu.join(3 * a.face + a.b, 3 * b.face + b.a);
    }
    let mut sums = vec![[0.0; 3]; flat.len()];
    let mut group_sizes = vec![0; flat.len()];
    for (i, t) in mesh.triangles.iter().enumerate() {
        for c in 0..3 {
            let u = sub(t[(c + 1) % 3], t[c]);
            let v = sub(t[(c + 2) % 3], t[c]);
            let cv = cross(u, v);
            let angle = dot(cv, cv).sqrt().atan2(dot(u, v));
            let r = dsu.root(3 * i + c);
            group_sizes[r] += 1;
            for d in 0..3 {
                sums[r][d] += angle * flat[3 * i + c][d];
            }
        }
    }
    let mut out = flat.clone();
    let cap = settings.max_deviation_degrees.to_radians().cos();
    for i in 0..out.len() {
        let r = dsu.root(i);
        if group_sizes[r] == 1 {
            continue;
        }
        let candidate = unit(sums[r]);
        // Reject a corner rather than trusting transitive fan chaining to keep
        // its direction local. This is deliberately conservative.
        if dot(candidate, flat[i]) < cap {
            stats.capped_corners += 1;
            continue;
        }
        if settings.blend > 0.0 {
            out[i] = unit(std::array::from_fn(|d| {
                (1.0 - settings.blend) * flat[i][d] + settings.blend * candidate[d]
            }));
        }
        let cos = dot(out[i], flat[i]).clamp(-1.0, 1.0);
        let angle = cos.acos().to_degrees();
        stats.max_deviation_degrees = stats.max_deviation_degrees.max(angle);
        if cos < 1.0 - 1e-12 {
            stats.changed_corners += 1;
        }
    }
    Ok((out, stats))
}

/// Exact caller pose and viewport: no automatic refit and no camera sweep.
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub rotation: [[f64; 3]; 3],
    pub scale: f64,
    pub offset: [f64; 2],
}
impl Camera {
    pub fn rotate(self, v: Vec3) -> Vec3 {
        self.rotation.map(|row| dot(row, v))
    }
    pub fn digest(self) -> String {
        let mut h = Sha256::new();
        h.update(b"appearance.fixed_camera.f64le.v1\0");
        for v in self
            .rotation
            .iter()
            .flatten()
            .copied()
            .chain([self.scale])
            .chain(self.offset)
        {
            h.update(v.to_le_bytes());
        }
        format!("{:x}", h.finalize())
    }
    fn validate(self) -> Result<()> {
        if self
            .rotation
            .iter()
            .flatten()
            .chain(self.offset.iter())
            .any(|v| !v.is_finite())
            || !self.scale.is_finite()
            || self.scale <= 0.
        {
            return Err("nonfinite or invalid fixed camera".into());
        }
        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1. } else { 0. };
                if (dot(self.rotation[i], self.rotation[j]) - expected).abs() > 1e-10 {
                    return Err("camera rotation is not orthogonal".into());
                }
            }
        }
        if (dot(self.rotation[0], cross(self.rotation[1], self.rotation[2])) - 1.).abs() > 1e-10 {
            return Err("camera must preserve proper source pose".into());
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Lighting {
    /// Camera-space direction; never used to change visibility or coverage.
    pub direction: Vec3,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: i32,
}
impl Lighting {
    fn validate(self) -> Result<()> {
        if self
            .direction
            .iter()
            .any(|x| !x.is_finite() || x.abs() > 1e6)
            || dot(self.direction, self.direction) <= 0.
            || !self.ambient.is_finite()
            || !(0. ..=1.).contains(&self.ambient)
            || !self.diffuse.is_finite()
            || !(0. ..=1.).contains(&self.diffuse)
            || self.ambient + self.diffuse > 1.
            || !self.specular.is_finite()
            || !(0. ..=0.25).contains(&self.specular)
            || !(1..=128).contains(&self.shininess)
        {
            return Err("invalid interior-only lighting recipe".into());
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RenderSettings {
    pub width: usize,
    pub height: usize,
    pub supersample: usize,
    /// Inward coverage-boundary width in delivered pixels; no outward dilation.
    pub outline_pixels: f64,
    pub camera: Camera,
    pub lighting: Lighting,
}

pub struct Rendered {
    pub rgb: Vec<u8>,
    /// Same visibility and base colors, without lighting. Outline/AA are shared.
    pub unlit_rgb: Vec<u8>,
    /// One byte per high-resolution geometric sample, independent of normals.
    pub coverage: Vec<u8>,
    pub outline: Vec<u8>,
    /// Original face index; usize::MAX denotes an uncovered sample.
    pub winning_face: Vec<usize>,
    pub depth: Vec<f64>,
    pub submitted_triangles: usize,
    pub edge_on_triangles: usize,
}
fn linear(x: u8) -> f64 {
    let x = x as f64 / 255.;
    if x <= 0.04045 {
        x / 12.92
    } else {
        ((x + 0.055) / 1.055).powf(2.4)
    }
}
fn srgb(x: f64) -> u8 {
    let x = x.clamp(0., 1.);
    let y = if x <= 0.0031308 {
        12.92 * x
    } else {
        1.055 * x.powf(1. / 2.4) - 0.055
    };
    (255. * y).round() as u8
}

/// All original triangles are submitted in source order. A correct per-sample
/// depth test chooses visibility; face normals never cull triangles. Exact depth
/// ties choose the later source face, a deterministic original-order tie rule.
/// No external receiver, shadow, blur, reflection environment or stride exists.
pub fn render(
    mesh: &RawMesh,
    normals: &[Vec3],
    base_rgb: &[[u8; 3]],
    cfg: RenderSettings,
) -> Result<Rendered> {
    if !(16..=4096).contains(&cfg.width)
        || !(16..=4096).contains(&cfg.height)
        || !(1..=4).contains(&cfg.supersample)
        || !cfg.outline_pixels.is_finite()
        || !(0.25..=3.).contains(&cfg.outline_pixels)
    {
        return Err("invalid dimensions, sampling or thin outline".into());
    }
    cfg.camera.validate()?;
    cfg.lighting.validate()?;
    let w = cfg
        .width
        .checked_mul(cfg.supersample)
        .ok_or("width overflow")?;
    let h = cfg
        .height
        .checked_mul(cfg.supersample)
        .ok_or("height overflow")?;
    let len = w
        .checked_mul(h)
        .filter(|n| *n <= 8_000_000)
        .ok_or("sample budget exceeded")?;
    if base_rgb.len() != mesh.len()
        || normals.len() != 3 * mesh.len()
        || normals
            .iter()
            .any(|n| n.iter().any(|x| !x.is_finite()) || (dot(*n, *n) - 1.).abs() > 1e-6)
    {
        return Err("source-face colors or finite unit corner-normal registration differs".into());
    }
    let mut depth = vec![f64::NEG_INFINITY; len];
    let mut coverage = vec![0; len];
    let mut winning_face = vec![usize::MAX; len];
    let mut lit = vec![[1_f32; 3]; len];
    let light = unit(cfg.lighting.direction);
    let half = unit([light[0], light[1], light[2] + 1.]);
    let mut edge_on = 0;
    for (i, triangle) in mesh.triangles.iter().enumerate() {
        let p = triangle.map(|v| {
            let q = cfg.camera.rotate(v);
            [
                (cfg.camera.offset[0] + q[0] * cfg.camera.scale) * cfg.supersample as f64,
                (cfg.camera.offset[1] - q[1] * cfg.camera.scale) * cfg.supersample as f64,
                q[2],
            ]
        });
        if p.iter().flatten().any(|x| !x.is_finite() || x.abs() > 1e15) {
            return Err("source projection is nonfinite or outside numeric budget".into());
        }
        let area =
            (p[1][1] - p[2][1]) * (p[0][0] - p[2][0]) + (p[2][0] - p[1][0]) * (p[0][1] - p[2][1]);
        if area == 0. {
            edge_on += 1;
            continue;
        }
        let bound = |axis: usize, max: bool, limit: usize| -> usize {
            let v = if max {
                p.iter()
                    .map(|q| q[axis])
                    .fold(f64::NEG_INFINITY, f64::max)
                    .ceil()
            } else {
                p.iter()
                    .map(|q| q[axis])
                    .fold(f64::INFINITY, f64::min)
                    .floor()
            };
            v.clamp(0., limit as f64) as usize
        };
        let ns = [
            cfg.camera.rotate(normals[3 * i]),
            cfg.camera.rotate(normals[3 * i + 1]),
            cfg.camera.rotate(normals[3 * i + 2]),
        ];
        for y in bound(1, false, h)..bound(1, true, h) {
            for x in bound(0, false, w)..bound(0, true, w) {
                let (px, py) = (x as f64 + 0.5, y as f64 + 0.5);
                let a = ((p[1][1] - p[2][1]) * (px - p[2][0])
                    + (p[2][0] - p[1][0]) * (py - p[2][1]))
                    / area;
                let b = ((p[2][1] - p[0][1]) * (px - p[2][0])
                    + (p[0][0] - p[2][0]) * (py - p[2][1]))
                    / area;
                let c = 1. - a - b;
                // Exact sample inclusion; no epsilon expands the silhouette.
                if a < 0. || b < 0. || c < 0. {
                    continue;
                }
                let index = y * w + x;
                let z = a * p[0][2] + b * p[1][2] + c * p[2][2];
                if z < depth[index] {
                    continue;
                }
                let n = unit(std::array::from_fn(|d| {
                    a * ns[0][d] + b * ns[1][d] + c * ns[2][d]
                }));
                if dot(n, n) == 0. {
                    return Err("zero interpolated appearance normal".into());
                }
                let diffuse = cfg.lighting.ambient + cfg.lighting.diffuse * dot(n, light).max(0.);
                let specular =
                    cfg.lighting.specular * dot(n, half).max(0.).powi(cfg.lighting.shininess);
                lit[index] =
                    base_rgb[i].map(|v| (linear(v) * diffuse + specular).clamp(0., 1.) as f32);
                depth[index] = z;
                winning_face[index] = i;
                coverage[index] = 255;
            }
        }
    }
    // L1 distance to uncovered samples/canvas edge gives an inward thin contour.
    // It neither fills holes nor adds any exterior black or colored pixels.
    let mut distance = vec![0_u8; len];
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            if coverage[i] != 0 {
                let left = if x == 0 { 0 } else { distance[i - 1] };
                let above = if y == 0 { 0 } else { distance[i - w] };
                distance[i] = left.min(above).saturating_add(1);
            }
        }
    }
    for y in (0..h).rev() {
        for x in (0..w).rev() {
            let i = y * w + x;
            if coverage[i] != 0 {
                let right = if x + 1 == w { 0 } else { distance[i + 1] };
                let below = if y + 1 == h { 0 } else { distance[i + w] };
                distance[i] = distance[i].min(right.min(below).saturating_add(1));
            }
        }
    }
    let radius = (cfg.outline_pixels * cfg.supersample as f64).ceil() as u8;
    let outline: Vec<u8> = coverage
        .iter()
        .zip(&distance)
        .map(|(&c, &d)| u8::from(c != 0 && d <= radius) * 255)
        .collect();
    let mut rgb = Vec::with_capacity(3 * cfg.width * cfg.height);
    let mut unlit_rgb = Vec::with_capacity(rgb.capacity());
    for y in 0..cfg.height {
        for x in 0..cfg.width {
            let (mut a, mut b, mut covered) = ([0.; 3], [0.; 3], false);
            for dy in 0..cfg.supersample {
                for dx in 0..cfg.supersample {
                    let i = (y * cfg.supersample + dy) * w + x * cfg.supersample + dx;
                    if coverage[i] == 0 {
                        for d in 0..3 {
                            a[d] += 1.;
                            b[d] += 1.;
                        }
                    } else {
                        covered = true;
                        if outline[i] == 0 {
                            for d in 0..3 {
                                a[d] += lit[i][d] as f64;
                                b[d] += linear(base_rgb[winning_face[i]][d]);
                            }
                        }
                    }
                }
            }
            if !covered {
                rgb.extend([255; 3]);
                unlit_rgb.extend([255; 3]);
            } else {
                let count = (cfg.supersample * cfg.supersample) as f64;
                rgb.extend(a.map(|v| srgb(v / count)));
                unlit_rgb.extend(b.map(|v| srgb(v / count)));
            }
        }
    }
    Ok(Rendered {
        rgb,
        unlit_rgb,
        coverage,
        outline,
        winning_face,
        depth,
        submitted_triangles: mesh.len(),
        edge_on_triangles: edge_on,
    })
}

/// Minimal uncompressed 24-bit BMP avoids a new image/codec dependency.
pub fn bmp(rgb: &[u8], width: usize, height: usize) -> Result<Vec<u8>> {
    if !(1..=4096).contains(&width)
        || !(1..=4096).contains(&height)
        || rgb.len() != width * height * 3
    {
        return Err("invalid BMP dimensions".into());
    }
    let row = (width * 3 + 3) & !3;
    let size = 54 + row * height;
    let mut out = vec![0; size];
    out[0..2].copy_from_slice(b"BM");
    for (offset, value) in [
        (2, size as u32),
        (10, 54),
        (14, 40),
        (18, width as u32),
        (22, height as u32),
        (34, (row * height) as u32),
    ] {
        out[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    out[26..28].copy_from_slice(&1_u16.to_le_bytes());
    out[28..30].copy_from_slice(&24_u16.to_le_bytes());
    for y in 0..height {
        for x in 0..width {
            let src = 3 * (y * width + x);
            let dst = 54 + (height - 1 - y) * row + x * 3;
            out[dst..dst + 3].copy_from_slice(&[rgb[src + 2], rgb[src + 1], rgb[src]]);
        }
    }
    Ok(out)
}
