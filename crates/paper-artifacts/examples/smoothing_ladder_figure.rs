//! Smoothing-ladder figure producer: for every level of a `jarambit smoothing-ladder` receipt,
//! an oblique relief view of the whole design, oblique close-ups of the guard (isogrid pockets) and
//! of the blade, and a top-view
//! map of the displacement from level 0. Views are chosen to show 3D variation:
//! - one fixed oblique camera (tilted 55 degrees from the hero top view) and a low raking light;
//! - z (thickness direction) exaggerated by `--exaggeration` (default 2.5) in the oblique views;
//! - base colour from the face's height above the mid-plane (0 to the stock half-thickness, then
//!   saturated), so faces, grinds and channels separate by colour as well as by shading;
//! - one displacement colour range shared by every level.
//!
//! Every camera, light, colour range and exaggeration is shared by all levels, so differences
//! between images are differences between meshes. Geometry only: nothing is re-scored here.
//! Usage: smoothing_ladder_figure --ladder LADDER_JSON --out DIR --code-commit SHA
//!        [--width PX] [--exaggeration E]
//! Writes stored-deflate PNGs, an SVG wrapper per PNG for lossless re-encoding,
//! smoothing-ladder.render.v1.json and ladder-meta.tex into DIR (created if absent).
use jarambit_paper_artifacts::appearance::{
    self, Camera, Lighting, NormalSettings, RawMesh, RenderSettings, Vec3,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};

// Hero camera rotation R_z(-6 deg) * R_x(180 deg), as in smoothing_figure.
const CT: f64 = 0.9945218953682733;
const ST: f64 = -0.10452846326765347;
const TILT_DEG: f64 = 55.0;
const SUPERSAMPLE: usize = 2;
/// Close-up regions in the design frame (x, mm): the guard and pivot zone, where isogrid
/// pockets and fuller windows sit, and everything beyond it (blade, grinds, serrations, tip).
const GUARD_X_MM: [f64; 2] = [70.0, 112.0];
const DETAIL_X_MM: f64 = 112.0;
const NORMALS: NormalSettings = NormalSettings {
    crease_degrees: 15.0,
    blend: 1.0,
    max_deviation_degrees: 20.0,
};
// Height tint over [0, stock half-thickness], sRGB.
const HEIGHT_STOPS: [(f64, [f64; 3]); 4] = [
    (0.0, [52.0, 62.0, 120.0]),
    (0.4, [64.0, 140.0, 170.0]),
    (0.75, [150.0, 200.0, 160.0]),
    (1.0, [240.0, 228.0, 196.0]),
];
// Displacement heatmap stops (as smoothing_figure), read against the unlit colour.
const DISP_STOPS: [(f64, [f64; 3]); 3] = [
    (0.0, [18.0, 42.0, 74.0]),
    (0.5, [44.0, 165.0, 141.0]),
    (1.0, [253.0, 231.0, 37.0]),
];

type Res<T> = Result<T, Box<dyn std::error::Error>>;

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

fn bar(stops: &[(f64, [f64; 3])], width: usize, height: usize) -> Vec<u8> {
    let row: Vec<u8> = (0..width)
        .flat_map(|x| ramp(stops, x as f64 / (width - 1) as f64))
        .collect();
    row.repeat(height)
}

/// Stored (uncompressed) zlib PNG, RGB 8-bit, as in smoothing_figure; the build re-encodes it
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

fn oblique_rotation() -> [[f64; 3]; 3] {
    let (s, c) = TILT_DEG.to_radians().sin_cos();
    // R_x(tilt) * hero rotation.
    [[CT, ST, 0.0], [c * ST, -c * CT, s], [s * ST, -s * CT, -c]]
}

fn top_rotation() -> [[f64; 3]; 3] {
    [[CT, ST, 0.0], [ST, -CT, 0.0], [0.0, 0.0, -1.0]]
}

/// Fits the projected extent of `points` into the viewport with a margin.
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
    Camera {
        rotation,
        scale,
        offset: [width as f64 / 2.0 - scale * mid[0], height as f64 / 2.0 + scale * mid[1]],
    }
}

fn raking() -> Lighting {
    Lighting {
        direction: [-0.75, 0.55, 0.37],
        ambient: 0.22,
        diffuse: 0.78,
        specular: 0.06,
        shininess: 40,
    }
}

fn exaggerate(mesh: &RawMesh, e: f64) -> Res<RawMesh> {
    Ok(RawMesh::new(
        mesh.triangles().iter().map(|t| t.map(|v| [v[0], v[1], e * v[2]])).collect(),
    )?)
}

/// Area-weighted median |z| of near-flat faces (|n_z| > 0.9): the stock half-thickness.
fn stock_half_thickness(mesh: &RawMesh) -> f64 {
    let mut faces: Vec<(f64, f64)> = mesh
        .triangles()
        .iter()
        .filter_map(|t| {
            let u = [t[1][0] - t[0][0], t[1][1] - t[0][1], t[1][2] - t[0][2]];
            let w = [t[2][0] - t[0][0], t[2][1] - t[0][1], t[2][2] - t[0][2]];
            let n = [u[1] * w[2] - u[2] * w[1], u[2] * w[0] - u[0] * w[2], u[0] * w[1] - u[1] * w[0]];
            let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            // Near-flat face: unit normal within ~25 degrees of z; weight by area.
            (len > 0.0 && n[2].abs() / len > 0.9).then(|| ((t[0][2] + t[1][2] + t[2][2]).abs() / 3.0, 0.5 * len))
        })
        .collect();
    faces.sort_by(|a, b| a.0.total_cmp(&b.0));
    let half = faces.iter().map(|f| f.1).sum::<f64>() / 2.0;
    let mut acc = 0.0;
    for (z, a) in &faces {
        acc += a;
        if acc >= half {
            return *z;
        }
    }
    faces.last().map_or(1.0, |f| f.0)
}

fn main() -> Res<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let arg = |key: &str| args.iter().position(|a| a == key).and_then(|i| args.get(i + 1)).cloned();
    let need = |key: &str| arg(key).ok_or_else(|| format!("missing {key}"));
    let ladder_path = PathBuf::from(need("--ladder")?);
    let out = PathBuf::from(need("--out")?);
    let code_commit = need("--code-commit")?;
    let width: usize = arg("--width").map_or(Ok(2000), |w| w.parse())?;
    let exaggeration: f64 = arg("--exaggeration").map_or(Ok(2.5), |e| e.parse())?;
    if !(800..=2000).contains(&width) || !(1.0..=10.0).contains(&exaggeration) {
        return Err("--width must be 800..=2000 and --exaggeration 1..=10".into());
    }
    let height = width * 17 / 40;
    let ladder_bytes = fs::read(&ladder_path)?;
    let ladder: Value = serde_json::from_slice(&ladder_bytes)?;
    let levels = ladder["levels"].as_array().ok_or("ladder has no levels")?;
    let mut meshes = Vec::new();
    for (i, lv) in levels.iter().enumerate() {
        let path = PathBuf::from(lv["smoothed_stl"].as_str().ok_or("level lacks smoothed_stl")?);
        let expected = if i == 0 { &ladder["inputs"]["exported_stl_sha256"] } else { &lv["smoothing"]["smoothed_stl_sha256"] };
        let bytes = fs::read(&path)?;
        if Some(sha(&bytes).as_str()) != expected.as_str() {
            return Err(format!("{} does not match the ladder receipt sha256", path.display()).into());
        }
        meshes.push(appearance::decode_stl(&bytes)?);
    }
    if meshes.iter().any(|m| m.len() != meshes[0].len()) {
        return Err("face counts differ between levels; the smoother keeps face order".into());
    }
    let base = &meshes[0];
    let zs = stock_half_thickness(base);
    let lifted: Vec<RawMesh> = meshes.iter().map(|m| exaggerate(m, exaggeration)).collect::<Res<_>>()?;
    let all: Vec<Vec3> = lifted[0].triangles().iter().flatten().copied().collect();
    let blade: Vec<Vec3> = lifted[0].triangles().iter().flatten().filter(|v| v[0] >= DETAIL_X_MM).copied().collect();
    let guard: Vec<Vec3> = lifted[0].triangles().iter().flatten().filter(|v| (GUARD_X_MM[0]..GUARD_X_MM[1]).contains(&v[0])).copied().collect();
    let flat: Vec<Vec3> = base.triangles().iter().flatten().copied().collect();
    let cam_full = frame(oblique_rotation(), &all, width, height, 0.92);
    let cam_detail = frame(oblique_rotation(), &blade, width, height, 0.94);
    let cam_guard = frame(oblique_rotation(), &guard, width, height, 0.94);
    let cam_top = frame(top_rotation(), &flat, width, height, 0.88);
    let settings = |camera: Camera| RenderSettings {
        width,
        height,
        supersample: SUPERSAMPLE,
        outline_pixels: 1.5,
        camera,
        lighting: raking(),
    };
    // Shared displacement range: the largest mean per-face displacement over every level.
    let disp: Vec<Vec<f64>> = meshes
        .iter()
        .map(|m| {
            m.triangles()
                .iter()
                .zip(base.triangles())
                .map(|(ta, tb)| {
                    (0..3)
                        .map(|k| ((ta[k][0] - tb[k][0]).powi(2) + (ta[k][1] - tb[k][1]).powi(2) + (ta[k][2] - tb[k][2]).powi(2)).sqrt() / 3.0)
                        .sum()
                })
                .collect()
        })
        .collect();
    let disp_max = disp.iter().flatten().copied().fold(0.0_f64, f64::max).max(1e-12);
    fs::create_dir_all(&out)?;
    let mut outputs = Vec::new();
    // Each stored PNG gets an SVG wrapper of its own size, so `rsvg-convert` can re-encode it
    // losslessly (the build does this; the stored bytes are recorded here).
    let mut put = |name: String, bytes: Vec<u8>, w: usize, h: usize| -> Res<()> {
        fs::write(out.join(&name), &bytes)?;
        let svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" width=\"{w}\" height=\"{h}\"><image xlink:href=\"{name}\" x=\"0\" y=\"0\" width=\"{w}\" height=\"{h}\"/></svg>"
        );
        fs::write(out.join(name.replace(".png", ".svg")), svg)?;
        outputs.push(json!({"name": name, "bytes": bytes.len(), "sha256": sha(&bytes), "width": w, "height": h}));
        Ok(())
    };
    for (i, (mesh, lifted)) in meshes.iter().zip(&lifted).enumerate() {
        // Tint by the true (unexaggerated) height above the mid-plane.
        let tint: Vec<[u8; 3]> = mesh
            .triangles()
            .iter()
            .map(|t| ramp(&HEIGHT_STOPS, ((t[0][2] + t[1][2] + t[2][2]).abs() / 3.0) / zs))
            .collect();
        let (normals, _) = appearance::derive_normals(lifted, NORMALS, None)?;
        let full = appearance::render(lifted, &normals, &tint, settings(cam_full))?;
        let detail = appearance::render(lifted, &normals, &tint, settings(cam_detail))?;
        let guard_view = appearance::render(lifted, &normals, &tint, settings(cam_guard))?;
        let (flat_normals, _) = appearance::derive_normals(base, NORMALS, None)?;
        let heat: Vec<[u8; 3]> = disp[i].iter().map(|d| ramp(&DISP_STOPS, d / disp_max)).collect();
        let map = appearance::render(base, &flat_normals, &heat, settings(cam_top))?;
        put(format!("L{i}-oblique.png"), png(&full.rgb, width, height), width, height)?;
        put(format!("L{i}-blade.png"), png(&detail.rgb, width, height), width, height)?;
        put(format!("L{i}-guard.png"), png(&guard_view.rgb, width, height), width, height)?;
        put(format!("L{i}-displacement.png"), png(&map.unlit_rgb, width, height), width, height)?;
        eprintln!("L{i} rendered");
    }
    put("height-colorbar.png".into(), png(&bar(&HEIGHT_STOPS, width, 28), width, 28), width, 28)?;
    put("displacement-colorbar.png".into(), png(&bar(&DISP_STOPS, width, 28), width, 28), width, 28)?;
    let camera = |c: Camera| json!({"rotation": c.rotation, "scale": c.scale, "offset": c.offset, "sha256": c.digest()});
    let manifest = json!({
        "schema": "smoothing_ladder_figure.v1",
        "code_commit": code_commit,
        "command": std::env::args().collect::<Vec<_>>().join(" "),
        "ladder": {"path": ladder_path, "sha256": sha(&ladder_bytes), "label": ladder["label"]},
        "outputs": outputs,
        "views": {
            "oblique": "whole design; camera tilted 55 deg from the hero top view about the screen x axis, framed on level 0 and shared by all levels",
            "blade": format!("close-up of everything at x >= {DETAIL_X_MM} mm (blade, grinds, serrations, tip), same rotation, framed on level 0"),
            "guard": format!("close-up of {} mm <= x < {} mm (guard and pivot zone: isogrid pockets, fuller windows), same rotation, framed on level 0", GUARD_X_MM[0], GUARD_X_MM[1]),
            "displacement": "hero top view, unlit; colour is the mean of the face's three corner displacements from level 0",
        },
        "z_exaggeration": exaggeration,
        "height_tint": {"value": "|z| of the face centroid (true, unexaggerated mm)", "range_mm": [0.0, zs],
                        "range_rule": "0 to the area-weighted median |z| of near-flat faces of level 0 (stock half-thickness); higher faces (ring) saturate",
                        "stops_srgb": HEIGHT_STOPS.iter().map(|(t, c)| json!({"t": t, "rgb": c})).collect::<Vec<_>>()},
        "displacement": {"range_mm": [0.0, disp_max], "shared_by_all_levels": true,
                         "stops_srgb": DISP_STOPS.iter().map(|(t, c)| json!({"t": t, "rgb": c})).collect::<Vec<_>>()},
        "lighting": {"direction_camera": raking().direction, "ambient": 0.22, "diffuse": 0.78, "specular": 0.06, "shininess": 40, "kind": "low raking light from the upper left"},
        "cameras": {"oblique": camera(cam_full), "blade": camera(cam_detail), "guard": camera(cam_guard), "displacement": camera(cam_top)},
        "render": {"width": width, "height": height, "supersample": SUPERSAMPLE, "outline_pixels": 1.5, "renderer": "deterministic Rust CPU depth raster (appearance::render)"},
        "normals": {"crease_degrees": NORMALS.crease_degrees, "blend": NORMALS.blend, "max_deviation_degrees": NORMALS.max_deviation_degrees},
        "not_qualified": true,
    });
    fs::write(out.join("smoothing-ladder.render.v1.json"), serde_json::to_string_pretty(&manifest)? + "\n")?;
    // Values the ladder PDF prints next to the shared scales.
    let label = ladder["label"].as_str().unwrap_or("design").replace('_', "\\_");
    let exported = levels[0]["score"].as_f64().ok_or("level 0 has no score")?;
    let meta = format!(
        "% Generated by smoothing_ladder_figure; do not edit.\n\\newcommand{{\\LadderLabel}}{{\\texttt{{{label}}}}}\n\\newcommand{{\\LadderExported}}{{{exported:.6}}}\n\\newcommand{{\\LadderExaggeration}}{{{exaggeration}}}\n\\newcommand{{\\LadderStockHalf}}{{{zs:.2}}}\n\\newcommand{{\\LadderDispMax}}{{{disp_max:.3}}}\n"
    );
    fs::write(out.join("ladder-meta.tex"), meta)?;
    println!("wrote {}", out.display());
    Ok(())
}
