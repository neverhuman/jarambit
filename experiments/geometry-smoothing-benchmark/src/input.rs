//! STL input for input-mesh mode: strict parsing, exact vertex welding and an
//! audit that runs before smoothing. Nothing here repairs or tolerance-merges geometry.
use crate::{cross, dot, norm, sub, Face, Mesh, Point, Result};
use std::collections::{BTreeMap, BTreeSet, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Ascii,
    Binary,
}

impl Format {
    pub fn name(self) -> &'static str {
        match self {
            Format::Ascii => "ascii",
            Format::Binary => "binary",
        }
    }
}

/// Binary STL is recognised by its exact length (84 + 50 * triangle count), which
/// takes priority over a "solid" header. Anything else must be strict ASCII STL.
pub fn parse_stl(bytes: &[u8]) -> Result<(Format, Vec<[Point; 3]>)> {
    if bytes.len() >= 84 {
        let count = u32::from_le_bytes([bytes[80], bytes[81], bytes[82], bytes[83]]) as usize;
        if bytes.len() == 84 + 50 * count {
            return parse_binary(bytes, count).map(|t| (Format::Binary, t));
        }
    }
    parse_ascii(bytes).map(|t| (Format::Ascii, t))
}

fn parse_binary(bytes: &[u8], count: usize) -> Result<Vec<[Point; 3]>> {
    let mut triangles = Vec::with_capacity(count);
    for record in bytes[84..].chunks_exact(50) {
        let mut triangle = [[0.0; 3]; 3];
        for (corner, vertex) in triangle.iter_mut().enumerate() {
            for (axis, value) in vertex.iter_mut().enumerate() {
                // Record layout: 12-byte normal, three 12-byte vertices, 2 attribute bytes.
                let at = 12 + 12 * corner + 4 * axis;
                let x = f32::from_le_bytes([
                    record[at],
                    record[at + 1],
                    record[at + 2],
                    record[at + 3],
                ]);
                if !x.is_finite() {
                    return Err("binary STL contains a nonfinite coordinate".into());
                }
                *value = f64::from(x);
            }
        }
        triangles.push(triangle);
    }
    Ok(triangles)
}

fn parse_ascii(bytes: &[u8]) -> Result<Vec<[Point; 3]>> {
    let text = std::str::from_utf8(bytes)
        .map_err(|_| "STL is neither exact-length binary nor UTF-8 ASCII".to_string())?;
    let mut lines = text.lines().map(str::trim).filter(|line| !line.is_empty());
    let header = lines.next().ok_or("empty STL file")?;
    if !header.starts_with("solid") {
        return Err("ASCII STL must begin with 'solid'".into());
    }
    let mut triangles = Vec::new();
    let mut ended = false;
    while let Some(line) = lines.next() {
        if line.starts_with("endsolid") {
            ended = true;
            break;
        }
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() != 5 || fields[0] != "facet" || fields[1] != "normal" {
            return Err(format!("expected 'facet normal' line, found: {line}"));
        }
        for token in &fields[2..] {
            parse_coordinate(token)?;
        }
        expect(lines.next(), "outer loop")?;
        let mut triangle = [[0.0; 3]; 3];
        for vertex in triangle.iter_mut() {
            let line = lines.next().ok_or("ASCII STL ends inside a facet")?;
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() != 4 || fields[0] != "vertex" {
                return Err(format!("expected 'vertex x y z' line, found: {line}"));
            }
            for (axis, value) in vertex.iter_mut().enumerate() {
                *value = parse_coordinate(fields[axis + 1])?;
            }
        }
        expect(lines.next(), "endloop")?;
        expect(lines.next(), "endfacet")?;
        triangles.push(triangle);
    }
    if !ended {
        return Err("ASCII STL has no 'endsolid' line".into());
    }
    if lines.next().is_some() {
        return Err("ASCII STL has content after 'endsolid'".into());
    }
    if triangles.is_empty() {
        return Err("STL contains no triangles".into());
    }
    Ok(triangles)
}

fn expect(line: Option<&str>, want: &str) -> Result<()> {
    match line {
        Some(line) if line.split_whitespace().eq(want.split_whitespace()) => Ok(()),
        other => Err(format!("expected '{want}' in ASCII STL, found {other:?}")),
    }
}

fn parse_coordinate(text: &str) -> Result<f64> {
    let value: f64 = text
        .parse()
        .map_err(|_| format!("invalid coordinate: {text}"))?;
    if !value.is_finite() {
        return Err(format!("nonfinite coordinate: {text}"));
    }
    Ok(value)
}

/// Welds triangle corners whose coordinates are equal (-0.0 equals 0.0). Corners
/// are never merged by tolerance; vertices keep the first-seen coordinates and
/// faces keep input order and winding.
pub fn weld(triangles: &[[Point; 3]]) -> Mesh {
    let mut index: HashMap<[u64; 3], usize> = HashMap::new();
    let mut vertices: Vec<Point> = Vec::new();
    let mut faces: Vec<Face> = Vec::with_capacity(triangles.len());
    for triangle in triangles {
        let mut face = [0; 3];
        for (slot, &point) in face.iter_mut().zip(triangle) {
            let key = point.map(|x| {
                if x == 0.0 {
                    0.0_f64.to_bits()
                } else {
                    x.to_bits()
                }
            });
            *slot = *index.entry(key).or_insert_with(|| {
                vertices.push(point);
                vertices.len() - 1
            });
        }
        faces.push(face);
    }
    Mesh { vertices, faces }
}

/// Topology facts about a welded mesh. Counts are reported even when they pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Audit {
    pub vertices: usize,
    pub faces: usize,
    pub edges: usize,
    pub open_edges: usize,
    pub nonmanifold_edges: usize,
    pub inconsistent_edges: usize,
    pub degenerate_faces: usize,
    pub duplicate_faces: usize,
    pub unused_vertices: usize,
    pub components: usize,
    pub euler_characteristic: i64,
}

impl Audit {
    /// Human-readable reasons this mesh must not be smoothed. Empty means accepted.
    pub fn failures(&self) -> Vec<String> {
        let counted = [
            (self.open_edges, "open edges (mesh is not closed)"),
            (self.nonmanifold_edges, "non-manifold edges"),
            (self.inconsistent_edges, "edges with inconsistent winding"),
            (self.degenerate_faces, "degenerate faces"),
            (self.duplicate_faces, "duplicate faces"),
            (self.unused_vertices, "unused vertices"),
        ];
        let mut reasons: Vec<String> = counted
            .iter()
            .filter(|(count, _)| *count > 0)
            .map(|(count, what)| format!("{count} {what}"))
            .collect();
        if self.components != 1 {
            reasons.push(format!(
                "{} connected components (expected 1)",
                self.components
            ));
        }
        reasons
    }
}

pub fn audit(mesh: &Mesh) -> Audit {
    let vertex_count = mesh.vertices.len();
    let mut edges: BTreeMap<(usize, usize), Vec<(usize, usize)>> = BTreeMap::new();
    let mut keys = BTreeSet::new();
    let mut parent: Vec<usize> = (0..vertex_count).collect();
    let mut used = vec![false; vertex_count];
    let (mut degenerate, mut duplicate) = (0, 0);
    for &face in &mesh.faces {
        if face.iter().any(|&i| i >= vertex_count) {
            degenerate += 1;
            continue;
        }
        let [a, b, c] = face;
        let mut key = face;
        key.sort_unstable();
        if !keys.insert(key) {
            duplicate += 1;
        }
        let area2 = norm(cross(
            sub(mesh.vertices[b], mesh.vertices[a]),
            sub(mesh.vertices[c], mesh.vertices[a]),
        ));
        let repeated = key[0] == key[1] || key[1] == key[2];
        if repeated || area2.is_nan() || area2 <= 1e-12 {
            degenerate += 1;
        }
        for (u, v) in [(a, b), (b, c), (c, a)] {
            if u != v {
                edges.entry((u.min(v), u.max(v))).or_default().push((u, v));
            }
        }
        for i in [a, b, c] {
            used[i] = true;
        }
        union(&mut parent, a, b);
        union(&mut parent, a, c);
    }
    let (mut open, mut nonmanifold, mut inconsistent) = (0, 0, 0);
    for incidents in edges.values() {
        match incidents.as_slice() {
            [_] => open += 1,
            [(u, v), (x, y)] if u == y && v == x => {}
            [_, _] => inconsistent += 1,
            _ => nonmanifold += 1,
        }
    }
    let mut components = 0;
    for (i, &is_used) in used.iter().enumerate() {
        if is_used && find(&mut parent, i) == i {
            components += 1;
        }
    }
    Audit {
        vertices: vertex_count,
        faces: mesh.faces.len(),
        edges: edges.len(),
        open_edges: open,
        nonmanifold_edges: nonmanifold,
        inconsistent_edges: inconsistent,
        degenerate_faces: degenerate,
        duplicate_faces: duplicate,
        unused_vertices: used.iter().filter(|&&u| !u).count(),
        components,
        euler_characteristic: vertex_count as i64 - edges.len() as i64 + mesh.faces.len() as i64,
    }
}

fn find(parent: &mut [usize], mut i: usize) -> usize {
    while parent[i] != i {
        parent[i] = parent[parent[i]];
        i = parent[i];
    }
    i
}

fn union(parent: &mut [usize], a: usize, b: usize) {
    let (ra, rb) = (find(parent, a), find(parent, b));
    if ra != rb {
        parent[ra] = rb;
    }
}

/// Signed volume by the divergence theorem (mm³). Positive for outward winding.
pub fn signed_volume(mesh: &Mesh) -> f64 {
    mesh.faces
        .iter()
        .map(|&[a, b, c]| dot(mesh.vertices[a], cross(mesh.vertices[b], mesh.vertices[c])) / 6.0)
        .sum()
}

/// Axis-aligned bounding box as (min, max).
pub fn bounds(mesh: &Mesh) -> (Point, Point) {
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for p in &mesh.vertices {
        for (axis, &x) in p.iter().enumerate() {
            min[axis] = min[axis].min(x);
            max[axis] = max[axis].max(x);
        }
    }
    (min, max)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere;

    fn ascii_stl(mesh: &Mesh) -> Vec<u8> {
        let mut text = String::from("solid test\n");
        for &[a, b, c] in &mesh.faces {
            text.push_str("  facet normal 0 0 1\n    outer loop\n");
            for i in [a, b, c] {
                let p = mesh.vertices[i];
                text.push_str(&format!("      vertex {} {} {}\n", p[0], p[1], p[2]));
            }
            text.push_str("    endloop\n  endfacet\n");
        }
        text.push_str("endsolid test\n");
        text.into_bytes()
    }

    fn binary_stl(mesh: &Mesh) -> Vec<u8> {
        let mut bytes = vec![0u8; 80];
        bytes.extend_from_slice(&(mesh.faces.len() as u32).to_le_bytes());
        for &[a, b, c] in &mesh.faces {
            bytes.extend_from_slice(&[0u8; 12]);
            for i in [a, b, c] {
                for value in mesh.vertices[i] {
                    bytes.extend_from_slice(&(value as f32).to_le_bytes());
                }
            }
            bytes.extend_from_slice(&[0u8; 2]);
        }
        bytes
    }

    fn corners(mesh: &Mesh) -> Vec<[Point; 3]> {
        mesh.faces
            .iter()
            .map(|f| f.map(|i| mesh.vertices[i]))
            .collect()
    }

    #[test]
    fn ascii_and_binary_weld_to_the_same_closed_topology() {
        // f32-representable coordinates, so binary and ASCII carry identical values.
        let mut source = sphere(2, 30., 0.).unwrap();
        for p in &mut source.vertices {
            for x in p.iter_mut() {
                *x = f64::from(*x as f32);
            }
        }
        let (ascii_format, ascii) = parse_stl(&ascii_stl(&source)).unwrap();
        let (binary_format, binary) = parse_stl(&binary_stl(&source)).unwrap();
        assert_eq!(ascii_format, Format::Ascii);
        assert_eq!(binary_format, Format::Binary);
        let from_ascii = weld(&ascii);
        let from_binary = weld(&binary);
        assert_eq!(from_ascii, from_binary);
        assert_eq!(from_ascii.faces.len(), source.faces.len());
        assert_eq!(from_ascii.vertices.len(), source.vertices.len());
        assert_eq!(corners(&from_ascii), corners(&source));
        let clean = audit(&from_ascii);
        assert!(clean.failures().is_empty(), "{clean:?}");
        assert_eq!(clean.euler_characteristic, 2);
    }

    #[test]
    fn open_mesh_is_reported_with_counts() {
        let mut mesh = sphere(1, 30., 0.).unwrap();
        mesh.faces.pop();
        let report = audit(&mesh);
        assert_eq!(report.open_edges, 3);
        assert_eq!(report.components, 1);
        assert!(report.failures().iter().any(|r| r.contains("open edges")));
    }

    #[test]
    fn non_manifold_and_disconnected_meshes_are_rejected() {
        let mut mesh = sphere(1, 30., 0.).unwrap();
        mesh.faces.push([mesh.faces[0][0], mesh.faces[0][1], 6]);
        let report = audit(&mesh);
        assert!(report.nonmanifold_edges > 0 || report.inconsistent_edges > 0);
        assert!(!report.failures().is_empty());

        let mut two = sphere(1, 30., 0.).unwrap();
        let other = sphere(1, 30., 0.).unwrap();
        let offset = two.vertices.len();
        two.vertices
            .extend(other.vertices.iter().map(|p| add3(*p, 100.)));
        two.faces
            .extend(other.faces.iter().map(|f| f.map(|i| i + offset)));
        assert_eq!(audit(&two).components, 2);
    }

    #[test]
    fn degenerate_and_nonfinite_input_is_rejected_at_parse_time() {
        let mut mesh = sphere(1, 30., 0.).unwrap();
        mesh.faces[0] = [mesh.faces[0][0], mesh.faces[0][0], mesh.faces[0][1]];
        assert!(audit(&mesh).degenerate_faces > 0);
        let nan = String::from_utf8(ascii_stl(&sphere(1, 30., 0.).unwrap()))
            .unwrap()
            .replacen("vertex", "vertex NaN", 1);
        assert!(parse_stl(nan.as_bytes()).is_err());
        assert!(parse_stl(b"solid x\nfacet normal 0 0 1\n").is_err());
        assert!(parse_stl(b"not an stl").is_err());
    }

    #[test]
    fn exact_welding_keeps_distinct_coordinates_apart() {
        let a = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
        let b = [[0., 0., 0.], [1., 0., 0.], [0., 1., 1e-12]];
        let mesh = weld(&[a, b]);
        assert_eq!(
            mesh.vertices.len(),
            4,
            "1e-12 offset must not be tolerance-merged"
        );
        let signed = weld(&[[[-0.0, 0., 0.], [1., 0., 0.], [0., 1., 0.]]]);
        assert_eq!(signed.vertices.len(), 3);
    }

    fn add3(p: Point, t: f64) -> Point {
        [p[0] + t, p[1] + t, p[2] + t]
    }
}
