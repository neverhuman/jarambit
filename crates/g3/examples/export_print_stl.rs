//! Standalone Binary STL (`Units: mm`) exporter for the G3 winner genome.
//!
//! Lives outside the hashed decoder files (`g3_genome.rs`, `g3_sdf.rs`, `g3_qd.rs`, `g3_sandwich.rs`,
//! `ergolight.rs`) so `g3_decoder_sha256()` is 100% unchanged.
//!
//! Usage:
//!   cargo run --release --manifest-path crates/g3/Cargo.toml --example export_print_stl -- \
//!     [winner_genome.csv] [out_dir] [--h 0.25] [--max-mb 50.0]
//!
//! Each of the two meshes is extracted at the finest pitch from --h upward (0.025 mm steps) that fits --max-mb.

use jarambit_crescent::export::{census, volume, Solid as SurfaceMesh};
use jarambit_g3::g3_genome::G3Genome;
use jarambit_g3::g3_qd::g3_decoder_sha256;
use jarambit_g3::g3_sdf::{extract_watertight_mesh_fail_closed, RHO_TI64_G_MM3};
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

/// Serialize a `SurfaceMesh` (coordinates in millimetres) as an IEEE-754 little-endian Binary STL
/// with an explicit `"Units: mm | ..."` 80-byte header for 3D printing bureaus.
fn write_binary_stl_mm(mesh: &SurfaceMesh, label: &str) -> Vec<u8> {
    let n_tris = mesh.tris.len();
    let mut buf = Vec::with_capacity(84 + n_tris * 50);
    let hdr_str = format!(
        "Units: mm | Jarambit G3 Ti-6Al-4V | {} | tris={}",
        label, n_tris
    );
    let mut hdr = [b' '; 80];
    let bytes = hdr_str.as_bytes();
    let copy_len = bytes.len().min(80);
    hdr[..copy_len].copy_from_slice(&bytes[..copy_len]);
    buf.extend_from_slice(&hdr);
    buf.extend_from_slice(&(n_tris as u32).to_le_bytes());

    for &[i0, i1, i2] in &mesh.tris {
        let arr = |v: (f64, f64, f64)| [v.0, v.1, v.2]; // crescent's V3 is a tuple
        let v0 = arr(mesh.verts[i0]);
        let v1 = arr(mesh.verts[i1]);
        let v2 = arr(mesh.verts[i2]);
        let ux = v1[0] - v0[0];
        let uy = v1[1] - v0[1];
        let uz = v1[2] - v0[2];
        let wx = v2[0] - v0[0];
        let wy = v2[1] - v0[1];
        let wz = v2[2] - v0[2];
        let nx = uy * wz - uz * wy;
        let ny = uz * wx - ux * wz;
        let nz = ux * wy - uy * wx;
        let len = (nx * nx + ny * ny + nz * nz).sqrt();
        let (nx, ny, nz) = if len > 1e-12 {
            ((nx / len) as f32, (ny / len) as f32, (nz / len) as f32)
        } else {
            (0.0f32, 0.0f32, 0.0f32)
        };
        for f in [
            nx,
            ny,
            nz,
            v0[0] as f32,
            v0[1] as f32,
            v0[2] as f32,
            v1[0] as f32,
            v1[1] as f32,
            v1[2] as f32,
            v2[0] as f32,
            v2[1] as f32,
            v2[2] as f32,
        ] {
            buf.extend_from_slice(&f.to_le_bytes());
        }
        buf.extend_from_slice(&0u16.to_le_bytes());
    }
    buf
}

fn mesh_bbox_mm(mesh: &SurfaceMesh) -> ([f64; 3], [f64; 3], [f64; 3]) {
    let mut bmin = [f64::INFINITY; 3];
    let mut bmax = [f64::NEG_INFINITY; 3];
    for v in &mesh.verts {
        let v = [v.0, v.1, v.2];
        for k in 0..3 {
            bmin[k] = bmin[k].min(v[k]);
            bmax[k] = bmax[k].max(v[k]);
        }
    }
    let span = [bmax[0] - bmin[0], bmax[1] - bmin[1], bmax[2] - bmin[2]];
    (bmin, bmax, span)
}

fn sha256_hex(bytes: &[u8]) -> String {
    jarambit_g3::sha256_bytes(bytes)
}

fn load_first_genome(csv_path: &Path) -> Result<(String, G3Genome), String> {
    let text = fs::read_to_string(csv_path)
        .map_err(|e| format!("failed to read {}: {e}", csv_path.display()))?;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed.starts_with("design_id")
            || trimmed.starts_with("schema")
        {
            continue;
        }
        if trimmed.starts_with("G3.") {
            let g = G3Genome::from_csv_row(trimmed)?;
            return Ok(("winner".to_string(), g));
        }
        if let Some((id, rest)) = trimmed.split_once(',') {
            let g = G3Genome::from_csv_row(rest)?;
            return Ok((id.to_string(), g));
        }
    }
    Err(format!("no valid G3 genome row found in {}", csv_path.display()))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut csv_path = PathBuf::from("evidence/g3/v3.0.0/winner_genome.csv");
    let mut out_dir = PathBuf::from("docs/g3/v3.0.0/cad");
    let mut h_start = 0.25f64;
    let mut max_mb = 50.0f64;

    let mut pos = Vec::new();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--h" => {
                h_start = args[i + 1].parse().expect("--h float");
                i += 2;
            }
            "--max-mb" => {
                max_mb = args[i + 1].parse().expect("--max-mb float");
                i += 2;
            }
            s if !s.starts_with("--") => {
                pos.push(s.to_string());
                i += 1;
            }
            other => {
                eprintln!("unknown flag: {other}");
                std::process::exit(2);
            }
        }
    }
    if let Some(p0) = pos.first() {
        csv_path = PathBuf::from(p0);
    }
    if let Some(p1) = pos.get(1) {
        out_dir = PathBuf::from(p1);
    }

    fs::create_dir_all(&out_dir).expect("create out_dir");
    let (design_id, genome) = load_first_genome(&csv_path).expect("load genome");
    let max_bytes = (max_mb * 1024.0 * 1024.0) as usize;

    // Each mesh gets the finest voxel pitch (from h_start up in 0.025 mm steps) whose binary STL fits max_mb:
    // the envelope has far fewer triangles than the lattice-filled AM build, so it can stay much finer.
    let fit = |use_am: bool, what: &str| -> (SurfaceMesh, f64) {
        let mut h = h_start;
        loop {
            if h > 0.60 + 1e-9 {
                panic!("no watertight {what} mesh within {max_mb:.1} MiB up to h = 0.60 mm");
            }
            if let Some(m) = extract_watertight_mesh_fail_closed(&genome, h, use_am) {
                let bytes = 84 + m.tris.len() * 50;
                if bytes <= max_bytes {
                    return (m, h);
                }
                eprintln!(
                    "[export_print_stl] {what} h={h:.3} mm produced {:.2} MiB ({} tris) > {max_mb:.1} MiB limit; stepping h + 0.025 mm",
                    bytes as f64 / (1024.0 * 1024.0),
                    m.tris.len()
                );
            }
            h += 0.025;
        }
    };
    let (am_mesh, h_am) = fit(true, "AMbuild");
    let (env_mesh, h_env) = fit(false, "envelope");

    let (am_bnd, am_nm, am_dmax) = census(&am_mesh.tris);
    let (env_bnd, env_nm, env_dmax) = census(&env_mesh.tris);
    assert_eq!((am_bnd, am_nm), (0, 0), "AMbuild mesh must be watertight");
    assert_eq!((env_bnd, env_nm), (0, 0), "envelope mesh must be watertight");

    let am_vol = volume(&am_mesh);
    let env_vol = volume(&env_mesh);
    let am_mass_g = am_vol * RHO_TI64_G_MM3;
    let env_mass_g = env_vol * RHO_TI64_G_MM3;
    let (bmin, bmax, span) = mesh_bbox_mm(&am_mesh);

    let am_name = format!("{design_id}_AMbuild_mm.stl");
    let env_name = format!("{design_id}_envelope_mm.stl");
    let am_bytes = write_binary_stl_mm(&am_mesh, &format!("{design_id}_AMbuild"));
    let env_bytes = write_binary_stl_mm(&env_mesh, &format!("{design_id}_envelope"));
    let am_sha = sha256_hex(&am_bytes);
    let env_sha = sha256_hex(&env_bytes);

    let am_path = out_dir.join(&am_name);
    let env_path = out_dir.join(&env_name);
    fs::write(&am_path, &am_bytes).expect("write AMbuild STL");
    fs::write(&env_path, &env_bytes).expect("write envelope STL");

    let spec = json!({
        "design_id": design_id,
        "schema": genome.schema,
        "genome_sha256": genome.sha256(),
        "decoder_sha256": g3_decoder_sha256(),
        "units": "mm",
        "material": "Ti-6Al-4V (Grade 5 / ELI LPBF)",
        "density_g_mm3": RHO_TI64_G_MM3,

        "overall_dimensions_mm": {
            "length_x_mm": (span[0] * 100.0).round() / 100.0,
            "height_y_mm": (span[1] * 100.0).round() / 100.0,
            "thickness_z_mm": (span[2] * 100.0).round() / 100.0,
            "bbox_min_mm": bmin.map(|v| (v * 100.0).round() / 100.0),
            "bbox_max_mm": bmax.map(|v| (v * 100.0).round() / 100.0),
        },
        "order_notes": format!(
            "Units: mm | Overall dimensions: {:.2} x {:.2} x {:.2} mm | Material: Ti-6Al-4V LPBF | Depowder via neutral-axis drainage ports",
            span[0], span[1], span[2]
        ),
        "am_build_stl": {
            "file": am_name,
            "voxel_pitch_h_mm": (h_am * 1000.0).round() / 1000.0,
            "bytes": am_bytes.len(),
            "mib": ((am_bytes.len() as f64 / (1024.0 * 1024.0)) * 100.0).round() / 100.0,
            "sha256": am_sha,
            "triangles": am_mesh.tris.len(),
            "vertices": am_mesh.verts.len(),
            "volume_mm3": (am_vol * 10.0).round() / 10.0,
            "mass_ti64_g": (am_mass_g * 100.0).round() / 100.0,
            "watertight": am_bnd == 0 && am_nm == 0,
            "boundary_edges": am_bnd,
            "non_manifold_edges": am_nm,
            "max_edge_degree": am_dmax,
        },
        "outer_envelope_stl": {
            "file": env_name,
            "voxel_pitch_h_mm": (h_env * 1000.0).round() / 1000.0,
            "bytes": env_bytes.len(),
            "mib": ((env_bytes.len() as f64 / (1024.0 * 1024.0)) * 100.0).round() / 100.0,
            "sha256": env_sha,
            "triangles": env_mesh.tris.len(),
            "vertices": env_mesh.verts.len(),
            "volume_mm3": (env_vol * 10.0).round() / 10.0,
            "solid_mass_ti64_g": (env_mass_g * 100.0).round() / 100.0,
            "watertight": env_bnd == 0 && env_nm == 0,
            "boundary_edges": env_bnd,
            "non_manifold_edges": env_nm,
            "max_edge_degree": env_dmax,
        }
    });
    let spec_path = out_dir.join(format!("{design_id}_print_spec.json"));
    fs::write(&spec_path, serde_json::to_string_pretty(&spec).unwrap() + "\n")
        .expect("write print_spec.json");
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());
}
