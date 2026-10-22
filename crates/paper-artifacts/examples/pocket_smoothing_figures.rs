//! Figures for surface-tension pocket smoothing (crates/crescent/src/pocket_smooth.rs). Geometry and
//! retained plate fields only: nothing is re-solved or re-scored here.
//!
//! Modes (every output directory must be new):
//!   hero  --stl OURS.stl --d1 D1.json --out DIR --score F [--percentile 99] [--layer-width 1400]
//!         [--exaggeration 1.25] [--crease 45] [--tilt 58] [--dx -34] [--dy 118] [--label-margin 400]
//!         Every retained plate response of the design (D1 `mesh` channels), each normalized by its
//!         own area-weighted percentile, painted on the smoothed CAD and stacked as exploded layers
//!         under one oblique camera and raking light, with the normalized envelope (pointwise
//!         maximum of the unit-load channels) on top. Fine 3D crease lines are drawn with a depth
//!         test against the same render.
//!   bw    --raw RAW.stl --ours OURS.stl --ladder LADDER.render.v1.json --out DIR [--width 1400]
//!         Raw export versus smoothed, white material, high-contrast raking light, black silhouette
//!         and crease lines, under the smoothing ladder's guard and blade cameras plus a handle
//!         camera framed the same way.
//!   morph --stl OURS.stl --ladder LADDER.render.v1.json --out DIR
//!         The ladder's level-0 oblique and guard views (camera, light, height tint, 2.5x relief)
//!         of the smoothed CAD.
use jarambit_paper_artifacts::appearance::{self, Camera, Lighting, NormalSettings, RawMesh, RenderSettings, Rendered, Vec3};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::{fs, path::Path, path::PathBuf};

type Res<T> = Result<T, Box<dyn std::error::Error>>;

const CT: f64 = 0.9945218953682733;
const ST: f64 = -0.10452846326765347;
const SUPERSAMPLE: usize = 2;
const NORMALS: NormalSettings = NormalSettings { crease_degrees: 15.0, blend: 1.0, max_deviation_degrees: 20.0 };
/// Low (indigo) to saturated (red): luminance stays high enough for the raking light to model the
/// relief at every value.
const BRILLIANT: [(f64, [f64; 3]); 7] = [
    (0.0, [46.0, 52.0, 146.0]),
    (0.18, [38.0, 110.0, 214.0]),
    (0.38, [32.0, 178.0, 196.0]),
    (0.55, [92.0, 204.0, 120.0]),
    (0.72, [246.0, 214.0, 64.0]),
    (0.86, [246.0, 142.0, 36.0]),
    (1.0, [222.0, 46.0, 38.0]),
];
const MASKED: [u8; 3] = [178, 182, 188];
const WHITE: [u8; 3] = [255, 255, 255];

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn ramp(stops: &[(f64, [f64; 3])], t: f64) -> [u8; 3] {
    let t = t.clamp(0.0, 1.0);
    for pair in stops.windows(2) {
        let (a, ca) = pair[0];
        let (b, cb) = pair[1];
        if t <= b {
            let f = (t - a) / (b - a);
            return std::array::from_fn(|k| (ca[k] + f * (cb[k] - ca[k])).round() as u8);
        }
    }
    stops[stops.len() - 1].1.map(|c| c.round() as u8)
}

/// Stored (uncompressed) zlib PNG, RGB 8-bit, as in smoothing_figure; the build may re-encode it
/// losslessly.
fn png(rgb: &[u8], width: usize, height: usize) -> Vec<u8> {
    fn chunk(tag: &[u8; 4], data: &[u8], out: &mut Vec<u8>) {
        out.extend_from_slice(&(data.len() as u32).to_be_bytes());
        out.extend_from_slice(tag);
        out.extend_from_slice(data);
        let mut crc = 0xffff_ffffu32;
        for &b in tag.iter().chain(data) {
            crc ^= b as u32;
            for _ in 0..8 {
                crc = if crc & 1 != 0 { (crc >> 1) ^ 0xedb8_8320 } else { crc >> 1 };
            }
        }
        out.extend_from_slice(&(!crc).to_be_bytes());
    }
    let mut raw = Vec::with_capacity((width * 3 + 1) * height);
    for row in rgb.chunks_exact(width * 3) {
        raw.push(0);
        raw.extend_from_slice(row);
    }
    let mut zlib = vec![0x78, 0x01];
    let chunks: Vec<&[u8]> = raw.chunks(65_535).collect();
    for (i, ch) in chunks.iter().enumerate() {
        zlib.push(u8::from(i + 1 == chunks.len()));
        let l = ch.len() as u16;
        zlib.extend_from_slice(&l.to_le_bytes());
        zlib.extend_from_slice(&(!l).to_le_bytes());
        zlib.extend_from_slice(ch);
    }
    let (mut s1, mut s2) = (1u32, 0u32);
    for &b in &raw {
        s1 = (s1 + b as u32) % 65_521;
        s2 = (s2 + s1) % 65_521;
    }
    zlib.extend_from_slice(&((s2 << 16) | s1).to_be_bytes());
    let mut out = vec![137, 80, 78, 71, 13, 10, 26, 10];
    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&(width as u32).to_be_bytes());
    ihdr.extend_from_slice(&(height as u32).to_be_bytes());
    ihdr.extend_from_slice(&[8, 2, 0, 0, 0]);
    chunk(b"IHDR", &ihdr, &mut out);
    chunk(b"IDAT", &zlib, &mut out);
    chunk(b"IEND", &[], &mut out);
    out
}

/// Writes a stored PNG and an SVG wrapper of its own size, so `rsvg-convert` can re-encode it
/// losslessly (as smoothing_ladder_figure does).
fn put_png(out: &Path, name: &str, bytes: &[u8], w: usize, h: usize) -> Res<()> {
    fs::write(out.join(name), bytes)?;
    let svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" width=\"{w}\" height=\"{h}\"><image xlink:href=\"{name}\" x=\"0\" y=\"0\" width=\"{w}\" height=\"{h}\"/></svg>"
    );
    fs::write(out.join(name.replace(".png", ".svg")), svg)?;
    Ok(())
}

fn oblique_rotation(tilt_deg: f64) -> [[f64; 3]; 3] {
    let (s, c) = tilt_deg.to_radians().sin_cos();
    // R_x(tilt) * hero rotation R_z(-6 deg) R_x(180 deg), as in smoothing_ladder_figure.
    [[CT, ST, 0.0], [c * ST, -c * CT, s], [s * ST, -s * CT, -c]]
}

fn frame(rotation: [[f64; 3]; 3], points: &[Vec3], width: usize, height: usize, fill: f64) -> Camera {
    let (mut min, mut max) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
    for v in points {
        for k in 0..2 {
            let q = rotation[k][0] * v[0] + rotation[k][1] * v[1] + rotation[k][2] * v[2];
            min[k] = min[k].min(q);
            max[k] = max[k].max(q);
        }
    }
    let scale = (fill * width as f64 / (max[0] - min[0])).min(fill * height as f64 / (max[1] - min[1]));
    let mid = [(min[0] + max[0]) / 2.0, (min[1] + max[1]) / 2.0];
    Camera { rotation, scale, offset: [width as f64 / 2.0 - scale * mid[0], height as f64 / 2.0 + scale * mid[1]] }
}

fn camera_from(v: &Value, factor: f64) -> Res<Camera> {
    let r = v["rotation"].as_array().ok_or("camera rotation")?;
    let mut rotation = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            rotation[i][j] = r[i][j].as_f64().ok_or("rotation entry")?;
        }
    }
    let scale = v["scale"].as_f64().ok_or("camera scale")? * factor;
    let o = v["offset"].as_array().ok_or("camera offset")?;
    Ok(Camera { rotation, scale, offset: [o[0].as_f64().ok_or("offset")? * factor, o[1].as_f64().ok_or("offset")? * factor] })
}

fn camera_json(c: Camera) -> Value {
    json!({"rotation": c.rotation, "scale": c.scale, "offset": c.offset, "sha256": c.digest()})
}

fn exaggerate(mesh: &RawMesh, e: f64) -> Res<RawMesh> {
    Ok(RawMesh::new(mesh.triangles().iter().map(|t| t.map(|v| [v[0], v[1], e * v[2]])).collect())?)
}

fn face_normal(t: &[Vec3; 3]) -> Vec3 {
    let u = [t[1][0] - t[0][0], t[1][1] - t[0][1], t[1][2] - t[0][2]];
    let w = [t[2][0] - t[0][0], t[2][1] - t[0][1], t[2][2] - t[0][2]];
    let n = [u[1] * w[2] - u[2] * w[1], u[2] * w[0] - u[0] * w[2], u[0] * w[1] - u[1] * w[0]];
    let l = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if l > 0.0 { n.map(|x| x / l) } else { [0.0, 0.0, 1.0] }
}

/// Edges shared by exactly two faces (exact coordinate welding) whose face normals differ by more
/// than `deg`.
fn crease_edges(mesh: &RawMesh, deg: f64) -> Vec<[Vec3; 2]> {
    let key = |v: Vec3| (v[0].to_bits(), v[1].to_bits(), v[2].to_bits());
    type Corner = (u64, u64, u64);
    let mut map: HashMap<(Corner, Corner), Vec<usize>> = HashMap::new();
    let tris = mesh.triangles();
    for (f, t) in tris.iter().enumerate() {
        for k in 0..3 {
            let (a, b) = (key(t[k]), key(t[(k + 1) % 3]));
            map.entry(if a < b { (a, b) } else { (b, a) }).or_default().push(f);
        }
    }
    let cos_lim = deg.to_radians().cos();
    let mut keys: Vec<_> = map.into_iter().filter(|(_, fs)| fs.len() == 2).collect();
    keys.sort_by_key(|k| k.0);
    let mut out = Vec::new();
    for (e, fs) in keys {
        let (na, nb) = (face_normal(&tris[fs[0]]), face_normal(&tris[fs[1]]));
        if na[0] * nb[0] + na[1] * nb[1] + na[2] * nb[2] < cos_lim {
            let p = |k: Corner| [f64::from_bits(k.0), f64::from_bits(k.1), f64::from_bits(k.2)];
            out.push([p(e.0), p(e.1)]);
        }
    }
    out
}

/// Supersample mask of crease segments visible in `r` (depth-tested against the same render).
fn crease_mask(edges: &[[Vec3; 2]], cam: Camera, width: usize, height: usize, r: &Rendered) -> Vec<u8> {
    let (w, h) = (width * SUPERSAMPLE, height * SUPERSAMPLE);
    let mut mask = vec![0u8; w * h];
    let proj = |v: Vec3| {
        let q = cam.rotate(v);
        [(cam.offset[0] + q[0] * cam.scale) * SUPERSAMPLE as f64, (cam.offset[1] - q[1] * cam.scale) * SUPERSAMPLE as f64, q[2]]
    };
    let tol = 0.04 + 1.5 / cam.scale; // mm: about one and a half output pixels of depth slack
    for e in edges {
        let (a, b) = (proj(e[0]), proj(e[1]));
        let n = (((b[0] - a[0]).hypot(b[1] - a[1])) * 2.0).ceil().max(1.0) as usize;
        for s in 0..=n {
            let t = s as f64 / n as f64;
            let (x, y, z) = (a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1]), a[2] + t * (b[2] - a[2]));
            if x < 0.0 || y < 0.0 {
                continue;
            }
            let (xi, yi) = (x as usize, y as usize);
            if xi + 1 >= w || yi + 1 >= h {
                continue;
            }
            let idx = yi * w + xi;
            if r.coverage[idx] != 0 && z >= r.depth[idx] - tol {
                for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                    mask[(yi + dy) * w + xi + dx] = 1;
                }
            }
        }
    }
    mask
}

/// Delivered-pixel coverage alpha and crease alpha from supersample buffers.
fn downsample(buf: &[u8], width: usize, height: usize, pred: impl Fn(u8) -> bool) -> Vec<f64> {
    let w = width * SUPERSAMPLE;
    let mut out = vec![0.0; width * height];
    for y in 0..height {
        for x in 0..width {
            let mut c = 0;
            for dy in 0..SUPERSAMPLE {
                for dx in 0..SUPERSAMPLE {
                    if pred(buf[(y * SUPERSAMPLE + dy) * w + x * SUPERSAMPLE + dx]) {
                        c += 1;
                    }
                }
            }
            out[y * width + x] = c as f64 / (SUPERSAMPLE * SUPERSAMPLE) as f64;
        }
    }
    out
}

fn ink(rgb: &mut [u8], alpha: &[f64], strength: f64) {
    for (p, &a) in rgb.chunks_exact_mut(3).zip(alpha) {
        if a > 0.0 {
            for c in p.iter_mut() {
                *c = (*c as f64 * (1.0 - strength * a)).round() as u8;
            }
        }
    }
}

struct Args(Vec<String>);
impl Args {
    fn get(&self, key: &str) -> Option<String> {
        self.0.iter().position(|a| a == key).and_then(|i| self.0.get(i + 1)).cloned()
    }
    fn need(&self, key: &str) -> Res<String> {
        self.get(key).ok_or_else(|| format!("missing {key}").into())
    }
}

fn new_dir(out: &PathBuf) -> Res<()> {
    if out.exists() {
        return Err(format!("{} exists; select a new directory", out.display()).into());
    }
    fs::create_dir_all(out)?;
    Ok(())
}

fn load_stl(path: &str) -> Res<(RawMesh, String)> {
    let bytes = fs::read(path)?;
    let h = sha(&bytes);
    Ok((appearance::decode_stl(&bytes)?, h))
}

// ---------------------------------------------------------------- hero

struct Locator {
    xy: Vec<[f64; 2]>,
    tris: Vec<[usize; 3]>,
    cell: f64,
    x0: f64,
    y0: f64,
    bins: HashMap<(i64, i64), Vec<usize>>,
}
impl Locator {
    fn new(xy: Vec<[f64; 2]>, tris: Vec<[usize; 3]>) -> Self {
        let cell = 1.0;
        let x0 = xy.iter().map(|p| p[0]).fold(f64::INFINITY, f64::min);
        let y0 = xy.iter().map(|p| p[1]).fold(f64::INFINITY, f64::min);
        let mut bins: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
        for (k, t) in tris.iter().enumerate() {
            let ps = t.map(|i| xy[i]);
            let (lx, hx) = (ps.iter().map(|p| p[0]).fold(f64::INFINITY, f64::min), ps.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max));
            let (ly, hy) = (ps.iter().map(|p| p[1]).fold(f64::INFINITY, f64::min), ps.iter().map(|p| p[1]).fold(f64::NEG_INFINITY, f64::max));
            for i in ((lx - x0) / cell).floor() as i64..=((hx - x0) / cell).floor() as i64 {
                for j in ((ly - y0) / cell).floor() as i64..=((hy - y0) / cell).floor() as i64 {
                    bins.entry((i, j)).or_default().push(k);
                }
            }
        }
        Locator { xy, tris, cell, x0, y0, bins }
    }
    fn area(&self, k: usize) -> f64 {
        let [a, b, c] = self.tris[k].map(|i| self.xy[i]);
        0.5 * ((b[0] - a[0]) * (c[1] - a[1]) - (c[0] - a[0]) * (b[1] - a[1])).abs()
    }
    /// Containing triangle, else the triangle with the nearest centroid within 3 cells.
    fn locate(&self, q: [f64; 2]) -> Option<usize> {
        let (ci, cj) = (((q[0] - self.x0) / self.cell).floor() as i64, ((q[1] - self.y0) / self.cell).floor() as i64);
        if let Some(list) = self.bins.get(&(ci, cj)) {
            for &k in list {
                let [a, b, c] = self.tris[k].map(|i| self.xy[i]);
                let d = (b[1] - c[1]) * (a[0] - c[0]) + (c[0] - b[0]) * (a[1] - c[1]);
                if d == 0.0 {
                    continue;
                }
                let l1 = ((b[1] - c[1]) * (q[0] - c[0]) + (c[0] - b[0]) * (q[1] - c[1])) / d;
                let l2 = ((c[1] - a[1]) * (q[0] - c[0]) + (a[0] - c[0]) * (q[1] - c[1])) / d;
                if l1 >= -1e-9 && l2 >= -1e-9 && 1.0 - l1 - l2 >= -1e-9 {
                    return Some(k);
                }
            }
        }
        let mut best: Option<(f64, usize)> = None;
        for ring in 0..=3i64 {
            for i in ci - ring..=ci + ring {
                for j in cj - ring..=cj + ring {
                    if (i - ci).abs() != ring && (j - cj).abs() != ring {
                        continue;
                    }
                    for &k in self.bins.get(&(i, j)).map(|v| v.as_slice()).unwrap_or(&[]) {
                        let [a, b, c] = self.tris[k].map(|i| self.xy[i]);
                        let m = [(a[0] + b[0] + c[0]) / 3.0, (a[1] + b[1] + c[1]) / 3.0];
                        let d = (m[0] - q[0]).hypot(m[1] - q[1]);
                        if best.is_none_or(|(bd, bk)| d < bd || (d == bd && k < bk)) {
                            best = Some((d, k));
                        }
                    }
                }
            }
            if best.is_some() && ring >= 1 {
                break;
            }
        }
        best.map(|b| b.1)
    }
}

struct Channel {
    key: &'static str,
    name: &'static str,
    units: &'static str,
    zero_is_mask: bool,
}

const CHANNELS: [Channel; 5] = [
    Channel { key: "lat", name: "Lateral bending", units: "MPa/N", zero_is_mask: false },
    Channel { key: "twist", name: "Chord torsion", units: "MPa/(N mm)", zero_is_mask: false },
    Channel { key: "ring", name: "Ring traction", units: "MPa/N", zero_is_mask: true },
    Channel { key: "pry", name: "In-plane tip pry", units: "MPa/N", zero_is_mask: false },
    Channel { key: "env_vm", name: "Multi-case envelope", units: "MPa", zero_is_mask: false },
];
/// Unit-load channels whose normalized maximum forms the top layer.
const ENVELOPE_OF: [&str; 4] = ["lat", "twist", "ring", "pry"];

fn weighted_percentile(vals: &[(f64, f64)], p: f64) -> f64 {
    let mut v: Vec<(f64, f64)> = vals.iter().copied().filter(|(x, a)| *x > 0.0 && *a > 0.0).collect();
    v.sort_by(|a, b| a.0.total_cmp(&b.0));
    let total: f64 = v.iter().map(|x| x.1).sum();
    let mut acc = 0.0;
    for (x, a) in &v {
        acc += a;
        if acc >= p / 100.0 * total {
            return *x;
        }
    }
    v.last().map_or(1.0, |x| x.0)
}

fn hero(a: &Args) -> Res<()> {
    let stl = a.need("--stl")?;
    let d1_path = a.need("--d1")?;
    let out = PathBuf::from(a.need("--out")?);
    let score: f64 = a.need("--score")?.parse()?;
    let pct: f64 = a.get("--percentile").map_or(Ok(99.0), |s| s.parse())?;
    let exaggeration: f64 = a.get("--exaggeration").map_or(Ok(1.25), |s| s.parse())?;
    let crease_deg: f64 = a.get("--crease").map_or(Ok(45.0), |s| s.parse())?;
    let tilt: f64 = a.get("--tilt").map_or(Ok(58.0), |s| s.parse())?;
    let layer_w: usize = a.get("--layer-width").map_or(Ok(1400), |s| s.parse())?;
    let step: [f64; 2] = [a.get("--dx").map_or(Ok(-34.0), |s| s.parse())?, a.get("--dy").map_or(Ok(118.0), |s| s.parse())?];
    let label_margin: f64 = a.get("--label-margin").map_or(Ok(400.0), |s| s.parse())?;
    if !(90.0..=99.9).contains(&pct) || !(800..=2400).contains(&layer_w) || !(1.0..=4.0).contains(&exaggeration) {
        return Err("--percentile 90..=99.9, --layer-width 800..=2400, --exaggeration 1..=4".into());
    }
    new_dir(&out)?;
    let (mesh, stl_sha) = load_stl(&stl)?;
    let d1_bytes = fs::read(&d1_path)?;
    let d1: Value = serde_json::from_slice(&d1_bytes)?;
    let m = &d1["mesh"];
    let xy: Vec<[f64; 2]> = m["xy"].as_array().ok_or("mesh.xy")?.iter().map(|p| [p[0].as_f64().unwrap_or(f64::NAN), p[1].as_f64().unwrap_or(f64::NAN)]).collect();
    let tris: Vec<[usize; 3]> = m["tris"].as_array().ok_or("mesh.tris")?.iter().map(|t| [0, 1, 2].map(|k| t[k].as_u64().unwrap_or(0) as usize)).collect();
    let loc = Locator::new(xy, tris);
    let areas: Vec<f64> = (0..loc.tris.len()).map(|k| loc.area(k)).collect();
    // Per D1 triangle: normalized value per channel (None = producer-masked).
    let mut norm: HashMap<&str, Vec<Option<f64>>> = HashMap::new();
    let mut stats = Vec::new();
    for ch in &CHANNELS {
        let raw: Vec<f64> = m[ch.key].as_array().ok_or(format!("mesh.{}", ch.key))?.iter().map(|v| v.as_f64().unwrap_or(f64::NAN)).collect();
        if raw.len() != loc.tris.len() {
            return Err(format!("mesh.{} length differs from mesh.tris", ch.key).into());
        }
        let masked = |v: f64| !v.is_finite() || v < 0.0 || (ch.zero_is_mask && v == 0.0);
        let pairs: Vec<(f64, f64)> = raw.iter().zip(&areas).filter(|(v, _)| !masked(**v)).map(|(v, a)| (*v, *a)).collect();
        let p = weighted_percentile(&pairs, pct);
        let n: Vec<Option<f64>> = raw.iter().map(|&v| if masked(v) { None } else { Some((v / p).min(1.0)) }).collect();
        let total: f64 = pairs.iter().map(|x| x.1).sum();
        let sat: f64 = pairs.iter().filter(|(v, _)| *v >= p).map(|x| x.1).sum();
        let all_area: f64 = areas.iter().sum();
        let peak = pairs.iter().map(|x| x.0).fold(0.0, f64::max);
        stats.push(json!({"channel": ch.key, "name": ch.name, "units": ch.units, "percentile": pct, "normalizer": p, "peak": peak,
            "saturated_area_fraction": sat / total.max(1e-300), "masked_area_fraction": 1.0 - total / all_area,
            "mask_rule": if ch.zero_is_mask { "non-finite, negative or exactly zero (producer seated-region zeroing)" } else { "non-finite or negative" }}));
        norm.insert(ch.key, n);
    }
    let env: Vec<Option<f64>> = (0..loc.tris.len())
        .map(|k| ENVELOPE_OF.iter().filter_map(|c| norm[c][k]).fold(None, |acc: Option<f64>, v| Some(acc.map_or(v, |x| x.max(v)))))
        .collect();
    // Map every CAD face to a D1 triangle by its centroid (x, y).
    let owner: Vec<Option<usize>> = mesh.triangles().iter().map(|t| loc.locate([(t[0][0] + t[1][0] + t[2][0]) / 3.0, (t[0][1] + t[1][1] + t[2][1]) / 3.0])).collect();
    let unmapped = owner.iter().filter(|o| o.is_none()).count();
    let lifted = exaggerate(&mesh, exaggeration)?;
    let rot = oblique_rotation(tilt);
    let pts: Vec<Vec3> = lifted.triangles().iter().flatten().copied().collect();
    let layer_h = {
        let (mut lo, mut hi) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
        for v in &pts {
            for k in 0..2 {
                let q = rot[k][0] * v[0] + rot[k][1] * v[1] + rot[k][2] * v[2];
                lo[k] = lo[k].min(q);
                hi[k] = hi[k].max(q);
            }
        }
        ((hi[1] - lo[1]) / (hi[0] - lo[0]) * layer_w as f64 / 0.96).ceil() as usize
    };
    let cam = frame(rot, &pts, layer_w, layer_h, 0.96);
    let light = Lighting { direction: [-0.72, 0.58, 0.38], ambient: 0.30, diffuse: 0.70, specular: 0.10, shininess: 36 };
    let settings = RenderSettings { width: layer_w, height: layer_h, supersample: SUPERSAMPLE, outline_pixels: 1.25, camera: cam, lighting: light };
    let (normals, _) = appearance::derive_normals(&lifted, NORMALS, None)?;
    let edges = crease_edges(&lifted, crease_deg);
    let mut layers: Vec<(String, Vec<Option<f64>>)> = CHANNELS.iter().map(|c| (c.key.to_string(), norm[c.key].clone())).collect();
    layers.push(("envelope".into(), env));
    let mut images = Vec::new();
    let mut crease_alpha: Option<Vec<f64>> = None;
    let mut cover_alpha: Option<Vec<f64>> = None;
    let mut outputs = Vec::new();
    for (key, vals) in &layers {
        let colors: Vec<[u8; 3]> = owner.iter().map(|o| match o.and_then(|k| vals[k]) { Some(v) => ramp(&BRILLIANT, v), None => MASKED }).collect();
        let r = appearance::render(&lifted, &normals, &colors, settings)?;
        if crease_alpha.is_none() {
            let mask = crease_mask(&edges, cam, layer_w, layer_h, &r);
            crease_alpha = Some(downsample(&mask, layer_w, layer_h, |b| b != 0));
            cover_alpha = Some(downsample(&r.coverage, layer_w, layer_h, |b| b != 0));
        }
        let mut rgb = r.rgb.clone();
        ink(&mut rgb, crease_alpha.as_ref().unwrap(), 0.62);
        let bytes = png(&rgb, layer_w, layer_h);
        let name = format!("layer-{key}.png");
        put_png(&out, &name, &bytes, layer_w, layer_h)?;
        outputs.push(json!({"name": name, "sha256": sha(&bytes), "width": layer_w, "height": layer_h}));
        images.push(rgb);
    }
    // Exploded stack: bottom layer first; each higher layer moves by -step and is drawn over.
    let n = images.len();
    let span = [step[0].abs() * (n - 1) as f64, step[1] * (n - 1) as f64];
    let margin = [label_margin, 34.0];
    let width = (margin[0] + span[0] + layer_w as f64 + 30.0).ceil() as usize;
    let height = (layer_h as f64 + span[1] + 2.0 * margin[1]).ceil() as usize;
    let mut canvas = vec![255u8; width * height * 3];
    let cover = cover_alpha.unwrap();
    let origin = |k: usize| -> [f64; 2] {
        let up = (n - 1 - k) as f64; // 0 for the top layer
        let x = if step[0] < 0.0 { margin[0] + span[0] - up * step[0].abs() } else { margin[0] + up * step[0] };
        [x, margin[1] + up * step[1]]
    };
    let blend = |canvas: &mut [u8], img: &[u8], alpha: &[f64], o: [f64; 2], shadow: bool| {
        let (ox, oy) = (o[0].round() as isize, o[1].round() as isize);
        for y in 0..layer_h {
            for x in 0..layer_w {
                let a = alpha[y * layer_w + x];
                if a <= 0.0 {
                    continue;
                }
                let (cx, cy) = if shadow { (ox + x as isize + 7, oy + y as isize + 12) } else { (ox + x as isize, oy + y as isize) };
                if cx < 0 || cy < 0 || cx as usize >= width || cy as usize >= height {
                    continue;
                }
                let i = (cy as usize * width + cx as usize) * 3;
                for c in 0..3 {
                    let dst = canvas[i + c] as f64;
                    let v = if shadow { dst * (1.0 - 0.22 * a) } else { dst * (1.0 - a) + img[(y * layer_w + x) * 3 + c] as f64 * a };
                    canvas[i + c] = v.round() as u8;
                }
            }
        }
    };
    for (k, img) in images.iter().enumerate() {
        if k > 0 {
            blend(&mut canvas, img, &cover, origin(k), true);
        }
        blend(&mut canvas, img, &cover, origin(k), false);
    }
    let stack = png(&canvas, width, height);
    put_png(&out, "hero-stack.png", &stack, width, height)?;
    // Label anchors: the projected finger-ring centre of every layer.
    let ring = cam.rotate([0.0, 0.0, 0.0]);
    let ring_px = [cam.offset[0] + ring[0] * cam.scale, cam.offset[1] - ring[1] * cam.scale];
    let mut svg = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\">\n<rect width=\"{width}\" height=\"{height}\" fill=\"#ffffff\"/>\n<image xlink:href=\"hero-stack.png\" x=\"0\" y=\"0\" width=\"{width}\" height=\"{height}\"/>\n");
    let titles = ["Lateral bending", "Chord torsion", "Ring traction", "In-plane tip pry", "Multi-case envelope", "Normalized envelope"];
    for (k, (key, _)) in layers.iter().enumerate() {
        let o = origin(k);
        let (lx, ly) = (o[0] + ring_px[0] - 150.0, o[1] + ring_px[1] + 8.0);
        let sub = match stats.iter().find(|s| s["channel"] == key.as_str()) {
            Some(s) => format!("P{} = {:.4} {}", pct, s["normalizer"].as_f64().unwrap_or(0.0), s["units"].as_str().unwrap_or("")),
            None => "max of the four unit-load layers".to_string(),
        };
        let weight = if key == "envelope" { "bold" } else { "normal" };
        svg += &format!("<text x=\"{lx:.0}\" y=\"{ly:.0}\" text-anchor=\"end\" font-family=\"TeX Gyre Heros\" font-weight=\"{weight}\" font-size=\"27\" fill=\"#132D3B\">{}</text>\n<text x=\"{lx:.0}\" y=\"{:.0}\" text-anchor=\"end\" font-family=\"TeX Gyre Heros\" font-size=\"20\" fill=\"#55606a\">{}</text>\n", titles[k], ly + 25.0, sub.replace('&', "&amp;"));
    }
    // Colorbar and badge.
    let (bx, by, bw, bh) = (40.0, 44.0, 440.0, 22.0);
    svg += "<defs><linearGradient id=\"brilliant\" x1=\"0\" x2=\"1\" y1=\"0\" y2=\"0\">";
    for (t, c) in BRILLIANT {
        svg += &format!("<stop offset=\"{t}\" stop-color=\"rgb({},{},{})\"/>", c[0], c[1], c[2]);
    }
    svg += "</linearGradient></defs>\n";
    svg += &format!("<rect x=\"{bx}\" y=\"{by}\" width=\"{bw}\" height=\"{bh}\" fill=\"url(#brilliant)\" stroke=\"#132D3B\" stroke-width=\"1.2\"/>\n<text x=\"{bx}\" y=\"{:.0}\" font-family=\"TeX Gyre Heros\" font-size=\"20\" fill=\"#132D3B\">0</text>\n<text x=\"{:.0}\" y=\"{:.0}\" text-anchor=\"end\" font-family=\"TeX Gyre Heros\" font-size=\"20\" fill=\"#132D3B\">&#8805; P{pct} of each test</text>\n<rect x=\"{:.0}\" y=\"{:.0}\" width=\"22\" height=\"22\" fill=\"rgb({},{},{})\" stroke=\"#132D3B\" stroke-width=\"1.2\"/>\n<text x=\"{:.0}\" y=\"{:.0}\" font-family=\"TeX Gyre Heros\" font-size=\"20\" fill=\"#132D3B\">masked by the producer</text>\n",
        by + bh + 24.0, bx + bw, by + bh + 24.0, bx, by + bh + 38.0, MASKED[0], MASKED[1], MASKED[2], bx + 32.0, by + bh + 56.0);
    svg += &format!("<g transform=\"translate({:.0},{:.0})\"><rect width=\"470\" height=\"166\" rx=\"10\"", width as f64 - 500.0, height as f64 - 196.0);
    svg += &format!(" fill=\"#ffffff\" stroke=\"#132D3B\" stroke-width=\"2.5\"/>\n<text x=\"20\" y=\"38\" font-family=\"TeX Gyre Heros\" font-size=\"25\" fill=\"#132D3B\">W_ULTRALIGHT_v1 fitness</text>\n<text x=\"20\" y=\"108\" font-family=\"TeX Gyre Heros\" font-weight=\"bold\" font-size=\"68\" fill=\"#132D3B\">{score:.2}</text>\n<text x=\"222\" y=\"108\" font-family=\"TeX Gyre Heros\" font-size=\"30\" fill=\"#132D3B\">/ 100</text>\n<text x=\"20\" y=\"146\" font-family=\"TeX Gyre Heros\" font-size=\"22\" fill=\"#007E87\">Surface-tension pocket smoothing</text></g>\n");
    svg += "</svg>\n";
    fs::write(out.join("hero.svg"), &svg)?;
    let manifest = json!({
        "schema": "jarambit.pocket_smoothing_hero.v1",
        "research_only": true,
        "command": std::env::args().collect::<Vec<_>>().join(" "),
        "inputs": {"stl": {"path": stl, "sha256": stl_sha}, "d1": {"path": d1_path, "sha256": sha(&d1_bytes)}},
        "score_shown": score,
        "fields": stats,
        "envelope_layer": {"definition": "pointwise maximum of the percentile-normalized unit-load layers", "of": ENVELOPE_OF},
        "association": "each CAD face takes the stored value of the D1 plate triangle containing its centroid (x, y); no interpolation or new solve",
        "unmapped_faces": unmapped,
        "colormap": {"stops_srgb": BRILLIANT.iter().map(|(t, c)| json!({"t": t, "rgb": c})).collect::<Vec<_>>(), "masked_rgb": MASKED},
        "camera": camera_json(cam), "tilt_deg": tilt, "z_exaggeration": exaggeration,
        "lighting": {"direction_camera": light.direction, "ambient": light.ambient, "diffuse": light.diffuse, "specular": light.specular, "shininess": light.shininess},
        "crease_lines": {"dihedral_deg_over": crease_deg, "edges": edges.len(), "depth_tested": true, "strength": 0.62},
        "stack": {"order_bottom_to_top": layers.iter().map(|l| l.0.clone()).collect::<Vec<_>>(), "step_px": step, "canvas": [width, height], "layer": [layer_w, layer_h]},
        "outputs": outputs,
        "hero_stack_sha256": sha(&stack),
        "not_qualified": true,
    });
    fs::write(out.join("hero.render.v1.json"), serde_json::to_string_pretty(&manifest)? + "\n")?;
    println!("wrote {}", out.display());
    Ok(())
}

// ---------------------------------------------------------------- bw and morph

fn ladder(a: &Args) -> Res<(Value, String)> {
    let p = a.need("--ladder")?;
    let bytes = fs::read(&p)?;
    Ok((serde_json::from_slice(&bytes)?, sha(&bytes)))
}

fn bw(a: &Args) -> Res<()> {
    let out = PathBuf::from(a.need("--out")?);
    let width: usize = a.get("--width").map_or(Ok(1400), |s| s.parse())?;
    let (lad, lad_sha) = ladder(a)?;
    let lw = lad["render"]["width"].as_f64().ok_or("ladder render width")?;
    let lh = lad["render"]["height"].as_f64().ok_or("ladder render height")?;
    let e = lad["z_exaggeration"].as_f64().ok_or("ladder z_exaggeration")?;
    let f = width as f64 / lw;
    let height = (lh * f).round() as usize;
    new_dir(&out)?;
    let (raw, raw_sha) = load_stl(&a.need("--raw")?)?;
    let (ours, ours_sha) = load_stl(&a.need("--ours")?)?;
    let raw_l = exaggerate(&raw, e)?;
    let ours_l = exaggerate(&ours, e)?;
    let guard = camera_from(&lad["cameras"]["guard"], f)?;
    let blade = camera_from(&lad["cameras"]["blade"], f)?;
    // Handle: slot, flutes and dimples, framed like the ladder's close-ups.
    let handle_pts: Vec<Vec3> = raw_l.triangles().iter().flatten().filter(|v| (14.0..76.0).contains(&v[0])).copied().collect();
    let handle = frame(guard.rotation, &handle_pts, width, height, 0.94);
    let light = Lighting { direction: [-0.80, 0.52, 0.30], ambient: 0.10, diffuse: 0.90, specular: 0.0, shininess: 8 };
    let mut outputs = Vec::new();
    for (view, cam) in [("guard", guard), ("handle", handle), ("blade", blade)] {
        for (label, m) in [("raw", &raw_l), ("ours", &ours_l)] {
            let (normals, _) = appearance::derive_normals(m, NORMALS, None)?;
            let colors = vec![WHITE; m.len()];
            let settings = RenderSettings { width, height, supersample: SUPERSAMPLE, outline_pixels: 1.5, camera: cam, lighting: light };
            let r = appearance::render(m, &normals, &colors, settings)?;
            let edges = crease_edges(m, 30.0);
            let mask = crease_mask(&edges, cam, width, height, &r);
            let mut rgb = r.rgb.clone();
            ink(&mut rgb, &downsample(&mask, width, height, |b| b != 0), 0.95);
            let bytes = png(&rgb, width, height);
            let name = format!("bw-{view}-{label}.png");
            put_png(&out, &name, &bytes, width, height)?;
            outputs.push(json!({"name": name, "sha256": sha(&bytes), "width": width, "height": height, "crease_edges": edges.len(), "camera": camera_json(cam)}));
        }
    }
    let manifest = json!({
        "schema": "jarambit.pocket_smoothing_bw.v1",
        "research_only": true,
        "command": std::env::args().collect::<Vec<_>>().join(" "),
        "inputs": {"raw_stl_sha256": raw_sha, "ours_stl_sha256": ours_sha, "ladder_render_sha256": lad_sha},
        "views": {"guard": "smoothing-ladder guard camera", "blade": "smoothing-ladder blade camera", "handle": "same rotation, framed on 14 mm <= x < 76 mm of the raw export"},
        "z_exaggeration": e,
        "material": "white, no specular", "lighting": {"direction_camera": light.direction, "ambient": light.ambient, "diffuse": light.diffuse},
        "lines": "black silhouette (1.5 px) and depth-tested crease edges (dihedral over 30 deg)",
        "outputs": outputs,
    });
    fs::write(out.join("bw.render.v1.json"), serde_json::to_string_pretty(&manifest)? + "\n")?;
    println!("wrote {}", out.display());
    Ok(())
}

fn morph(a: &Args) -> Res<()> {
    let out = PathBuf::from(a.need("--out")?);
    let (lad, lad_sha) = ladder(a)?;
    let width = lad["render"]["width"].as_u64().ok_or("ladder width")? as usize;
    let height = lad["render"]["height"].as_u64().ok_or("ladder height")? as usize;
    let e = lad["z_exaggeration"].as_f64().ok_or("ladder z_exaggeration")?;
    let zs = lad["height_tint"]["range_mm"][1].as_f64().ok_or("ladder height range")?;
    let stops: Vec<(f64, [f64; 3])> = lad["height_tint"]["stops_srgb"]
        .as_array()
        .ok_or("ladder height stops")?
        .iter()
        .map(|s| (s["t"].as_f64().unwrap_or(0.0), [0, 1, 2].map(|k| s["rgb"][k].as_f64().unwrap_or(0.0))))
        .collect();
    let l = &lad["lighting"];
    let light = Lighting {
        direction: [0, 1, 2].map(|k| l["direction_camera"][k].as_f64().unwrap_or(0.0)),
        ambient: l["ambient"].as_f64().ok_or("ambient")?,
        diffuse: l["diffuse"].as_f64().ok_or("diffuse")?,
        specular: l["specular"].as_f64().ok_or("specular")?,
        shininess: l["shininess"].as_i64().ok_or("shininess")? as i32,
    };
    let outline = lad["render"]["outline_pixels"].as_f64().ok_or("outline")?;
    new_dir(&out)?;
    let stl = a.need("--stl")?;
    let (mesh, stl_sha) = load_stl(&stl)?;
    let lifted = exaggerate(&mesh, e)?;
    let tint: Vec<[u8; 3]> = mesh.triangles().iter().map(|t| ramp(&stops, ((t[0][2] + t[1][2] + t[2][2]).abs() / 3.0) / zs)).collect();
    let (normals, _) = appearance::derive_normals(&lifted, NORMALS, None)?;
    let mut outputs = Vec::new();
    for view in ["oblique", "guard", "blade"] {
        let cam = camera_from(&lad["cameras"][view], 1.0)?;
        let r = appearance::render(&lifted, &normals, &tint, RenderSettings { width, height, supersample: SUPERSAMPLE, outline_pixels: outline, camera: cam, lighting: light })?;
        let bytes = png(&r.rgb, width, height);
        let name = format!("morph-{view}.png");
        put_png(&out, &name, &bytes, width, height)?;
        outputs.push(json!({"name": name, "sha256": sha(&bytes), "width": width, "height": height, "camera_sha256": cam.digest()}));
    }
    let manifest = json!({
        "schema": "jarambit.pocket_smoothing_morph.v1",
        "research_only": true,
        "command": std::env::args().collect::<Vec<_>>().join(" "),
        "inputs": {"stl": {"path": stl, "sha256": stl_sha}, "ladder_render_sha256": lad_sha},
        "settings": "the smoothing ladder's level-0 cameras, light, outline, height tint and z exaggeration, unchanged",
        "outputs": outputs,
    });
    fs::write(out.join("morph.render.v1.json"), serde_json::to_string_pretty(&manifest)? + "\n")?;
    println!("wrote {}", out.display());
    Ok(())
}

fn main() -> Res<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = args.first().cloned().unwrap_or_default();
    let a = Args(args);
    match mode.as_str() {
        "hero" => hero(&a),
        "bw" => bw(&a),
        "morph" => morph(&a),
        _ => Err("usage: pocket_smoothing_figures hero|bw|morph ... (see the module comment)".into()),
    }
}
