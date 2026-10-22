//! RESEARCH: render the same model-space windows of several STL meshes with the paper's appearance
//! pipeline (hero rotation, lighting and crease normals), so pocket walls can be compared crop by
//! crop. Geometry only: nothing is re-scored, re-assessed or qualified. Writes a NEW directory.
//! Usage:
//!   pocket_crops --out NEW_DIR [--px-per-mm N] --mesh LABEL=STL [--mesh ...] --crop NAME=x0,y0,x1,y1 [--crop ...]
use jarambit_paper_artifacts::appearance::{self, Camera, Lighting, NormalSettings, RenderSettings};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};

// Hero camera rotation R_z(-6 deg) * R_x(180 deg), copied from smoothing_figure.
const CT: f64 = 0.9945218953682733;
const ST: f64 = -0.10452846326765347;
const BASE_RGB: [u8; 3] = [196, 204, 214];
const NORMALS: NormalSettings = NormalSettings { crease_degrees: 15.0, blend: 1.0, max_deviation_degrees: 20.0 };

fn light() -> Lighting {
    Lighting { direction: [-0.35, 0.4, 1.0], ambient: 0.35, diffuse: 0.65, specular: 0.04, shininess: 24 }
}

/// Stored (uncompressed) zlib PNG, RGB 8-bit, as in smoothing_figure.
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut out: Option<PathBuf> = None;
    let mut ppm = 24.0f64;
    let mut meshes: Vec<(String, PathBuf)> = Vec::new();
    let mut crops: Vec<(String, [f64; 4])> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let v = args.get(i + 1).ok_or("missing value")?;
        match args[i].as_str() {
            "--out" => out = Some(PathBuf::from(v)),
            "--px-per-mm" => ppm = v.parse()?,
            "--mesh" => {
                let (l, p) = v.split_once('=').ok_or("--mesh LABEL=STL")?;
                meshes.push((l.to_string(), PathBuf::from(p)));
            }
            "--crop" => {
                let (n, r) = v.split_once('=').ok_or("--crop NAME=x0,y0,x1,y1")?;
                let w: Vec<f64> = r.split(',').map(str::parse).collect::<Result<_, _>>()?;
                if w.len() != 4 {
                    return Err("crop needs 4 numbers".into());
                }
                crops.push((n.to_string(), [w[0], w[1], w[2], w[3]]));
            }
            other => return Err(format!("unknown argument {other}").into()),
        }
        i += 2;
    }
    let out = out.ok_or("missing --out")?;
    if out.exists() {
        return Err("output directory already exists; select a new directory".into());
    }
    if !(4.0..=200.0).contains(&ppm) {
        return Err("--px-per-mm outside 4..=200".into());
    }
    fs::create_dir_all(&out)?;
    let rotation = [[CT, ST, 0.0], [ST, -CT, 0.0], [0.0, 0.0, -1.0]];
    let mut records = Vec::new();
    for (label, path) in &meshes {
        let bytes = fs::read(path)?;
        let sha = format!("{:x}", Sha256::digest(&bytes));
        let mesh = appearance::decode_stl(&bytes)?;
        let (normals, _) = appearance::derive_normals(&mesh, NORMALS, None)?;
        let base = vec![BASE_RGB; mesh.len()];
        for (name, w) in &crops {
            let corners = [(w[0], w[1]), (w[2], w[1]), (w[2], w[3]), (w[0], w[3])];
            let (mut lo, mut hi) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
            for (x, y) in corners {
                let q = [CT * x + ST * y, ST * x - CT * y];
                for k in 0..2 {
                    lo[k] = lo[k].min(q[k]);
                    hi[k] = hi[k].max(q[k]);
                }
            }
            let width = ((hi[0] - lo[0]) * ppm).round() as usize;
            let height = ((hi[1] - lo[1]) * ppm).round() as usize;
            let camera = Camera { rotation, scale: ppm, offset: [-ppm * lo[0], ppm * hi[1]] };
            let settings = RenderSettings { width, height, supersample: 2, outline_pixels: 2.0, camera, lighting: light() };
            let image = appearance::render(&mesh, &normals, &base, settings)?;
            let file = format!("{name}-{label}.png");
            fs::write(out.join(&file), png(&image.rgb, width, height))?;
            records.push(json!({"mesh": label, "stl": path.display().to_string(), "stl_sha256": sha, "crop": name, "window_mm": w, "file": file, "width": width, "height": height}));
        }
    }
    let manifest = json!({
        "schema": "jarambit.pocket_crops.v1",
        "research_only": true,
        "px_per_mm": ppm,
        "camera_rotation": rotation,
        "renders": records,
    });
    fs::write(out.join("pocket-crops.json"), serde_json::to_string_pretty(&manifest)? + "\n")?;
    println!("wrote {}", out.display());
    Ok(())
}
