//! Smoothing figure producer: before/after appearance views and a per-face displacement
//! heatmap of one tessellated mesh, rendered with the paper's appearance pipeline. One camera
//! (framed on the before mesh) and one set of lighting and render settings serve all three.
//! Geometry only: nothing is re-scored, re-assessed or qualified. Writes a NEW output directory.
//! Usage: smoothing_figure --before STL --before-sha256 HEX --after STL --after-sha256 HEX \
//!        --code-commit SHA --out NEW_DIR [--width PX]
//! `--width` (default 1200) scales the image and camera framing, and the outline up to the
//! renderer's 3 px limit; the height keeps the 1200x510 aspect of the hero figure.
use jarambit_paper_artifacts::appearance::{
    self, Camera, Lighting, NormalSettings, RawMesh, RenderSettings, Vec3,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Write as _,
    path::{Path, PathBuf},
};

// Hero camera rotation R_z(-6 deg) * R_x(180 deg): cos and sin, copied from hero_appearance_camera.
const CT: f64 = 0.9945218953682733;
const ST: f64 = -0.10452846326765347;
const WIDTH: usize = 1200;
const HEIGHT: usize = 510;
const OUTLINE_PIXELS: f64 = 2.0;
const SUPERSAMPLE: usize = 2;
// Hero recipe values (docs/paper/generated/hero-appearance.v1.json).
const BASE_RGB: [u8; 3] = [196, 204, 214];
const NORMALS: NormalSettings = NormalSettings {
    crease_degrees: 15.0,
    blend: 1.0,
    max_deviation_degrees: 20.0,
};
// Heatmap colour stops over [0, max displacement], sRGB; read against the unlit colour.
const STOPS: [(f64, [f64; 3]); 3] = [
    (0.0, [18.0, 42.0, 74.0]),
    (0.5, [44.0, 165.0, 141.0]),
    (1.0, [253.0, 231.0, 37.0]),
];

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn sub(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn norm(a: Vec3) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}

fn light() -> Lighting {
    Lighting {
        direction: [-0.35, 0.4, 1.0],
        ambient: 0.35,
        diffuse: 0.65,
        specular: 0.04,
        shininess: 24,
    }
}

/// Frames the before mesh so its projected extent fills the viewport with a margin.
/// The same camera is used for the after mesh and the heatmap.
fn camera_for(mesh: &RawMesh, width: usize, height: usize) -> Camera {
    let rotation = [[CT, ST, 0.0], [ST, -CT, 0.0], [0.0, 0.0, -1.0]];
    let (mut min, mut max) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
    for tri in mesh.triangles() {
        for v in tri {
            let q = [CT * v[0] + ST * v[1], ST * v[0] - CT * v[1]];
            for k in 0..2 {
                min[k] = min[k].min(q[k]);
                max[k] = max[k].max(q[k]);
            }
        }
    }
    let scale =
        (0.86 * width as f64 / (max[0] - min[0])).min(0.80 * height as f64 / (max[1] - min[1]));
    let mid = [(min[0] + max[0]) / 2.0, (min[1] + max[1]) / 2.0];
    Camera {
        rotation,
        scale,
        offset: [
            width as f64 / 2.0 - scale * mid[0],
            height as f64 / 2.0 + scale * mid[1],
        ],
    }
}

fn render_settings(camera: Camera, width: usize, height: usize) -> RenderSettings {
    RenderSettings {
        width,
        height,
        supersample: SUPERSAMPLE,
        // The renderer accepts at most a 3 px outline.
        outline_pixels: (OUTLINE_PIXELS * width as f64 / WIDTH as f64).min(3.0),
        camera,
        lighting: light(),
    }
}

/// Linear colour scale for the heatmap, left (0) to right (1), same stops as `colour_at`.
fn colour_bar(width: usize, height: usize) -> Vec<u8> {
    let row: Vec<u8> = (0..width)
        .flat_map(|x| colour_at(x as f64 / (width - 1) as f64))
        .collect();
    row.repeat(height)
}

fn colour_at(t: f64) -> [u8; 3] {
    let t = t.clamp(0.0, 1.0);
    for pair in STOPS.windows(2) {
        let (a, ca) = pair[0];
        let (b, cb) = pair[1];
        if t <= b {
            let f = (t - a) / (b - a);
            return std::array::from_fn(|k| (ca[k] + f * (cb[k] - ca[k])).round() as u8);
        }
    }
    let last = STOPS[STOPS.len() - 1].1;
    last.map(|c| c.round() as u8)
}

/// Stored (uncompressed) zlib PNG, RGB 8-bit; avoids a new codec dependency.
fn png(rgb: &[u8], width: usize, height: usize) -> Vec<u8> {
    fn chunk(tag: &[u8; 4], data: &[u8], out: &mut Vec<u8>) {
        out.extend_from_slice(&(data.len() as u32).to_be_bytes());
        out.extend_from_slice(tag);
        out.extend_from_slice(data);
        let mut crc = 0xffff_ffffu32;
        for &b in tag.iter().chain(data) {
            crc ^= b as u32;
            for _ in 0..8 {
                crc = if crc & 1 != 0 {
                    (crc >> 1) ^ 0xedb8_8320
                } else {
                    crc >> 1
                };
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

struct Input {
    path: PathBuf,
    bytes: Vec<u8>,
    mesh: RawMesh,
}

fn load(path: &Path, expected: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let bytes = fs::read(path)?;
    let digest = sha(&bytes);
    if digest != expected {
        return Err(format!(
            "{} sha256 {digest} differs from expected {expected}",
            path.display()
        )
        .into());
    }
    let mesh = appearance::decode_stl(&bytes)?;
    Ok(Input {
        path: path.to_path_buf(),
        bytes,
        mesh,
    })
}

fn write_new(dir: &Path, name: &str, bytes: &[u8]) -> Result<Value, Box<dyn std::error::Error>> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(dir.join(name))?;
    file.write_all(bytes)?;
    Ok(json!({"name": name, "bytes": bytes.len(), "sha256": sha(bytes)}))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let arg = |key: &str| -> Result<String, Box<dyn std::error::Error>> {
        args.iter()
            .position(|a| a == key)
            .and_then(|i| args.get(i + 1))
            .cloned()
            .ok_or_else(|| format!("missing {key}").into())
    };
    let before_path = PathBuf::from(arg("--before")?);
    let after_path = PathBuf::from(arg("--after")?);
    let out = PathBuf::from(arg("--out")?);
    let code_commit = arg("--code-commit")?;
    let width: usize = match args.iter().position(|a| a == "--width") {
        Some(i) => args.get(i + 1).ok_or("missing --width value")?.parse()?,
        None => WIDTH,
    };
    if !(WIDTH..=7200).contains(&width) {
        return Err(format!("--width {width} outside {WIDTH}..=7200").into());
    }
    let height = (width * HEIGHT + WIDTH / 2) / WIDTH;
    let before = load(&before_path, &arg("--before-sha256")?)?;
    let after = load(&after_path, &arg("--after-sha256")?)?;
    if out.exists() {
        return Err("output directory already exists; select a new directory".into());
    }

    // Same faces in the same order: the smoothing writer emits one facet per input face.
    if before.mesh.len() != after.mesh.len() {
        return Err("before and after face counts differ".into());
    }
    let camera = camera_for(&before.mesh, width, height);
    let settings = render_settings(camera, width, height);
    let base = vec![BASE_RGB; before.mesh.len()];

    let (normals_b, stats_b) = appearance::derive_normals(&before.mesh, NORMALS, None)?;
    let (normals_a, stats_a) = appearance::derive_normals(&after.mesh, NORMALS, None)?;
    let image_b = appearance::render(&before.mesh, &normals_b, &base, settings)?;
    let image_a = appearance::render(&after.mesh, &normals_a, &base, settings)?;

    // Per-corner displacement; corners correspond one-to-one by face order and corner index.
    let mut face_disp = Vec::with_capacity(before.mesh.len());
    let mut corner_max: f64 = 0.0;
    let mut corner_sum = 0.0;
    for (tb, ta) in before.mesh.triangles().iter().zip(after.mesh.triangles()) {
        let mut mean = 0.0;
        for k in 0..3 {
            let d = norm(sub(ta[k], tb[k]));
            corner_max = corner_max.max(d);
            corner_sum += d;
            mean += d / 3.0;
        }
        face_disp.push(mean);
    }
    let heat_base: Vec<[u8; 3]> = face_disp
        .iter()
        .map(|&d| colour_at(d / corner_max.max(1e-12)))
        .collect();
    let image_h = appearance::render(&before.mesh, &normals_b, &heat_base, settings)?;

    fs::create_dir(&out)?;
    let outputs = vec![
        write_new(&out, "winner-before.png", &png(&image_b.rgb, width, height))?,
        write_new(&out, "winner-after.png", &png(&image_a.rgb, width, height))?,
        write_new(
            &out,
            "winner-displacement.png",
            &png(&image_h.unlit_rgb, width, height),
        )?,
        write_new(
            &out,
            "winner-displacement-colorbar.png",
            &png(&colour_bar(width, 28), width, 28),
        )?,
    ];
    let manifest = json!({
        "schema": "smoothing_figure.v1",
        "outputs": outputs,
        "code_commit": code_commit,
        "command": std::env::args().collect::<Vec<_>>().join(" "),
        "not_qualified": true,
        "inputs": {
            "before": {"path": before.path.display().to_string(), "bytes": before.bytes.len(), "sha256": sha(&before.bytes)},
            "after": {"path": after.path.display().to_string(), "bytes": after.bytes.len(), "sha256": sha(&after.bytes)},
        },
        "geometry": {
            "faces": before.mesh.len(),
            "face_order_identical": true,
            "max_corner_displacement_mm": corner_max,
            "mean_corner_displacement_mm": corner_sum / (3.0 * before.mesh.len() as f64),
            "normals_before": {"joined_edges": stats_b.joined_edges, "blocked_edges": stats_b.blocked_edges, "capped_corners": stats_b.capped_corners},
            "normals_after": {"joined_edges": stats_a.joined_edges, "blocked_edges": stats_a.blocked_edges, "capped_corners": stats_a.capped_corners},
        },
        "camera": {
            "rotation": camera.rotation,
            "scale": camera.scale,
            "offset": camera.offset,
            "sha256": camera.digest(),
            "framed_on": "before mesh, shared by all three images; no refit",
        },
        "lighting": {"direction_camera": light().direction, "ambient": 0.35, "diffuse": 0.65, "specular": 0.04, "shininess": 24},
        "normals": {"crease_degrees": NORMALS.crease_degrees, "blend": NORMALS.blend, "max_deviation_degrees": NORMALS.max_deviation_degrees},
        "render": {"width": width, "height": height, "supersample": SUPERSAMPLE, "outline_pixels": settings.outline_pixels, "renderer": "deterministic Rust CPU depth raster (appearance::render)"},
        "base_rgb": BASE_RGB,
        "images": {
            "winner-before.png": "lit appearance of the before mesh (base colour uniform)",
            "winner-after.png": "lit appearance of the after mesh (base colour uniform)",
            "winner-displacement.png": "unlit heatmap of mean per-face corner displacement; colour is the value",
            "winner-displacement-colorbar.png": "unlit colour scale, linear from 0 mm (left) to heatmap range maximum (right)",
        },
        "heatmap": {"range_mm": [0.0, corner_max], "value": "mean of the three corner displacements per face", "stops_srgb": STOPS.iter().map(|(t, c)| json!({"t": t, "rgb": c})).collect::<Vec<_>>()},
    });
    let manifest_text = serde_json::to_string_pretty(&manifest)? + "\n";
    write_new(
        &out,
        "smoothing-figure.render.v1.json",
        manifest_text.as_bytes(),
    )?;
    println!("wrote {}", out.display());
    Ok(())
}
