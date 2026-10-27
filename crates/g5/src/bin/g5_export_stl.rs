//! Standalone 3D-Printing & Manufacturing CAD Bundle Exporter (`STL`, `3MF`, `DXF`, `JSON`, `Units: mm`)
//! for the G5 winner genome.
//!
//! Lives outside the hashed decoder files (`g5_genome.rs`, `g5_sdf.rs`, `g5_qd.rs`, `g5_sandwich.rs`,
//! `ergolight.rs`) so `g5_decoder_sha256()` is 100% unchanged.
//!
//! Usage:
//!   1. Extract from genome CSV and emit STL + 3MF + DXF + print_spec.json:
//!      cargo run --release --manifest-path crates/g5/Cargo.toml --bin g5-export-stl -- \
//!        [winner_genome.csv] [out_dir] [--h 0.25] [--max-mb 50.0]
//!   2. Generate 3MF + DXF companion files from the genome CSV and an existing STL directory:
//!      cargo run --release --manifest-path crates/g5/Cargo.toml --bin g5-export-stl -- \
//!        --companion-from-dir <winner_genome.csv> <stl_dir> <out_dir>

use jarambit_crescent::export::{census, volume, Solid as SurfaceMesh};
use jarambit_g5::g5_genome::G5Genome;
use jarambit_g5::g5_qd::g5_decoder_sha256;
use jarambit_g5::g5_sdf::{extract_watertight_mesh_fail_closed, RHO_TI64_G_MM3};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Serialize a `SurfaceMesh` (coordinates in millimetres) as an IEEE-754 little-endian Binary STL
/// with an explicit `"Units: mm | ..."` 80-byte header for 3D printing bureaus.
fn write_binary_stl_mm(mesh: &SurfaceMesh, label: &str) -> Vec<u8> {
    let n_tris = mesh.tris.len();
    let mut buf = Vec::with_capacity(84 + n_tris * 50);
    let hdr_str = format!(
        "Units: mm | Jarambit G5 Ti-6Al-4V | {} | tris={}",
        label, n_tris
    );
    let mut hdr = [b' '; 80];
    let bytes = hdr_str.as_bytes();
    let copy_len = bytes.len().min(80);
    hdr[..copy_len].copy_from_slice(&bytes[..copy_len]);
    buf.extend_from_slice(&hdr);
    buf.extend_from_slice(&(n_tris as u32).to_le_bytes());

    for &[i0, i1, i2] in &mesh.tris {
        let arr = |v: (f64, f64, f64)| [v.0, v.1, v.2];
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

/// Read an IEEE-754 little-endian Binary STL into a deduplicated `SurfaceMesh`.
fn read_binary_stl_mm(bytes: &[u8]) -> SurfaceMesh {
    assert!(bytes.len() >= 84, "binary STL too short");
    let n_tris = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
    assert_eq!(bytes.len(), 84 + n_tris * 50, "binary STL length mismatch");
    let mut verts: Vec<(f64, f64, f64)> = Vec::new();
    let mut tris: Vec<[usize; 3]> = Vec::with_capacity(n_tris);
    let mut vmap: HashMap<[u32; 3], usize> = HashMap::with_capacity(n_tris / 2 + 16);

    for t in 0..n_tris {
        let base = 84 + t * 50;
        let mut idx = [0usize; 3];
        for k in 0..3 {
            let off = base + 12 + k * 12;
            let bx = u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
            let by = u32::from_le_bytes(bytes[off + 4..off + 8].try_into().unwrap());
            let bz = u32::from_le_bytes(bytes[off + 8..off + 12].try_into().unwrap());
            let key = [bx, by, bz];
            let vi = *vmap.entry(key).or_insert_with(|| {
                let id = verts.len();
                verts.push((
                    f32::from_bits(bx) as f64,
                    f32::from_bits(by) as f64,
                    f32::from_bits(bz) as f64,
                ));
                id
            });
            idx[k] = vi;
        }
        tris.push(idx);
    }
    SurfaceMesh {
        verts,
        tris,
        bore: None,
    }
}

/// Write a 3D Manufacturing Format (`.3mf`, v1.2.3 OPC ZIP archive) file with explicit
/// `unit="millimeter"` and `Ti-6Al-4V (design assumption: yield 828 MPa)` material metadata.
fn write_3mf_mm(mesh: &SurfaceMesh, design_id: &str, span: [f64; 3], out_3mf: &Path) {
    let tmp_dir = out_3mf.with_extension("3mf_tmp");
    let _ = fs::remove_dir_all(&tmp_dir);
    fs::create_dir_all(tmp_dir.join("_rels")).expect("create _rels");
    fs::create_dir_all(tmp_dir.join("3D")).expect("create 3D");

    let content_types = r#"<?xml version="1.0" encoding="UTF-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="model" ContentType="application/vnd.ms-package.3dmanufacturing-3dmodel+xml"/>
</Types>
"#;
    fs::write(tmp_dir.join("[Content_Types].xml"), content_types).expect("write [Content_Types].xml");

    let rels = r#"<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Target="/3D/3dmodel.model" Id="rel0" Type="http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel"/>
</Relationships>
"#;
    fs::write(tmp_dir.join("_rels/.rels"), rels).expect("write _rels/.rels");

    let model_path = tmp_dir.join("3D/3dmodel.model");
    let mut f = std::io::BufWriter::with_capacity(8 * 1024 * 1024, fs::File::create(&model_path).expect("create 3dmodel.model"));
    writeln!(f, r#"<?xml version="1.0" encoding="UTF-8"?>"#).unwrap();
    writeln!(
        f,
        r#"<model unit="millimeter" xml:lang="en-US" xmlns="http://schemas.microsoft.com/3dmanufacturing/core/2015/02">"#
    )
    .unwrap();
    writeln!(f, "  <metadata name=\"Title\">Jarambit G5 {design_id} (AMbuild, mm)</metadata>").unwrap();
    writeln!(f, "  <metadata name=\"Designer\">Jarambit G5 Evolutionary Search</metadata>").unwrap();
    writeln!(
        f,
        "  <metadata name=\"Description\">Units: mm | Overall dimensions: {:.2} x {:.2} x {:.2} mm | Material: Ti-6Al-4V, L-PBF, stress-relieved (ASTM F2924 min yield 825 MPa, 750.8 MPa at 80 C) | Remove powder through the ports in every hollow zone before stress relief</metadata>",
        span[0], span[1], span[2]
    )
    .unwrap();
    writeln!(f, "  <resources>").unwrap();
    writeln!(f, "    <basematerials id=\"1\">").unwrap();
    writeln!(
        f,
        "      <base name=\"Ti-6Al-4V, L-PBF, stress-relieved (ASTM F2924 min yield 825 MPa, 750.8 MPa at 80 C)\" displaycolor=\"#8A929A\"/>"
    )
    .unwrap();
    writeln!(f, "    </basematerials>").unwrap();
    writeln!(
        f,
        "    <object id=\"2\" name=\"{design_id}_AMbuild_mm\" type=\"model\" pid=\"1\" pindex=\"0\">"
    )
    .unwrap();
    writeln!(f, "      <mesh>").unwrap();
    writeln!(f, "        <vertices>").unwrap();
    for &(x, y, z) in &mesh.verts {
        writeln!(
            f,
            "          <vertex x=\"{:.4}\" y=\"{:.4}\" z=\"{:.4}\"/>",
            x, y, z
        )
        .unwrap();
    }
    writeln!(f, "        </vertices>").unwrap();
    writeln!(f, "        <triangles>").unwrap();
    for &[i0, i1, i2] in &mesh.tris {
        writeln!(
            f,
            "          <triangle v1=\"{}\" v2=\"{}\" v3=\"{}\"/>",
            i0, i1, i2
        )
        .unwrap();
    }
    writeln!(f, "        </triangles>").unwrap();
    writeln!(f, "      </mesh>").unwrap();
    writeln!(f, "    </object>").unwrap();
    writeln!(f, "  </resources>").unwrap();
    writeln!(f, "  <build>").unwrap();
    writeln!(f, "    <item objectid=\"2\"/>").unwrap();
    writeln!(f, "  </build>").unwrap();
    writeln!(f, "</model>").unwrap();
    f.flush().unwrap();
    drop(f);

    let abs_out = if out_3mf.is_absolute() {
        out_3mf.to_path_buf()
    } else {
        std::env::current_dir().unwrap().join(out_3mf)
    };
    let _ = fs::remove_file(&abs_out);
    let status = Command::new("zip")
        .current_dir(&tmp_dir)
        .args([
            "-X",
            "-q",
            "-9",
            abs_out.to_str().unwrap(),
            "[Content_Types].xml",
            "_rels/.rels",
            "3D/3dmodel.model",
        ])
        .status()
        .expect("run zip to package .3mf");
    assert!(status.success(), "zip failed while creating .3mf");
    let _ = fs::remove_dir_all(&tmp_dir);
}

/// Generate a 1:1 true-scale 2D AutoCAD R12 (`AC1009`) DXF engineering drawing (1 drawing unit = 1 mm)
/// containing the exact `z = 0` mid-plane outline of the outer envelope, the decoded ring bore circle,
/// the `z`-extent thickness envelope, the `x = bmax[0] - 30 mm` blade cross-section, and overall
/// dimension callouts for manufacturing quote forms.
fn write_dxf_drawing_mm(
    am_mesh: &SurfaceMesh,
    env_mesh: &SurfaceMesh,
    design_id: &str,
    ring_cx: f64,
    ring_r: f64,
    bmin: [f64; 3],
    bmax: [f64; 3],
    span: [f64; 3],
) -> Vec<u8> {
    let n_bins = 180usize;
    let dx = (span[0] / n_bins as f64).max(0.5);
    let mut z_max_bin = vec![0.0f64; n_bins + 1];
    let mut has_bin = vec![false; n_bins + 1];

    for &(x, _y, z) in &env_mesh.verts {
        let bi = (((x - bmin[0]) / dx).round() as isize).clamp(0, n_bins as isize) as usize;
        z_max_bin[bi] = z_max_bin[bi].max(z.abs());
        has_bin[bi] = true;
    }

    // Extract exact mid-plane outline segments at z = 0.0 mm from the outer envelope mesh
    let z_mid = 0.0_f64;
    let mut mid_segs: Vec<([f64; 2], [f64; 2])> = Vec::new();
    for &[i0, i1, i2] in &env_mesh.tris {
        let p = [env_mesh.verts[i0], env_mesh.verts[i1], env_mesh.verts[i2]];
        let mut pts: Vec<[f64; 2]> = Vec::with_capacity(3);
        for &(a, b) in &[(0, 1), (1, 2), (2, 0)] {
            let (xa, ya, za) = p[a];
            let (xb, yb, zb) = p[b];
            if (za <= z_mid && zb >= z_mid) || (zb <= z_mid && za >= z_mid) {
                let denom = zb - za;
                if denom.abs() > 1e-9 {
                    let t = ((z_mid - za) / denom).clamp(0.0, 1.0);
                    let pt = [xa + t * (xb - xa), ya + t * (yb - ya)];
                    if !pts
                        .iter()
                        .any(|q| (q[0] - pt[0]).hypot(q[1] - pt[1]) < 1e-6)
                    {
                        pts.push(pt);
                    }
                }
            }
        }
        if pts.len() >= 2 {
            mid_segs.push((pts[0], pts[1]));
        }
    }

    // Extract exact cross-section segments at x_sec = bmax[0] - 30.0 mm (30 mm from the point)
    let x_sec = bmax[0] - 30.0;
    let mut sec_segs: Vec<([f64; 2], [f64; 2])> = Vec::new();
    for &[i0, i1, i2] in &am_mesh.tris {
        let p = [am_mesh.verts[i0], am_mesh.verts[i1], am_mesh.verts[i2]];
        let mut pts: Vec<[f64; 2]> = Vec::with_capacity(3);
        for &(a, b) in &[(0, 1), (1, 2), (2, 0)] {
            let (xa, ya, za) = p[a];
            let (xb, yb, zb) = p[b];
            if (xa <= x_sec && xb >= x_sec) || (xb <= x_sec && xa >= x_sec) {
                let denom = xb - xa;
                if denom.abs() > 1e-9 {
                    let t = ((x_sec - xa) / denom).clamp(0.0, 1.0);
                    let pt = [ya + t * (yb - ya), za + t * (zb - za)];
                    if !pts
                        .iter()
                        .any(|q| (q[0] - pt[0]).hypot(q[1] - pt[1]) < 1e-6)
                    {
                        pts.push(pt);
                    }
                }
            }
        }
        if pts.len() >= 2 {
            sec_segs.push((pts[0], pts[1]));
        }
    }

    let mut dxf = String::with_capacity(512 * 1024);
    // R12 (AC1009) ASCII DXF header; units are stated explicitly in the DIMENSIONS note block (1 unit = 1 mm)
    dxf.push_str("0\nSECTION\n2\nHEADER\n");
    dxf.push_str("9\n$ACADVER\n1\nAC1009\n");
    dxf.push_str("9\n$MEASUREMENT\n70\n1\n");
    dxf.push_str(&format!(
        "9\n$EXTMIN\n10\n{:.3}\n20\n{:.3}\n30\n{:.3}\n",
        bmin[0] - 10.0,
        bmin[1] - 45.0,
        bmin[2]
    ));
    dxf.push_str(&format!(
        "9\n$EXTMAX\n10\n{:.3}\n20\n{:.3}\n30\n{:.3}\n",
        bmax[0] + 60.0,
        bmax[1] + 25.0,
        bmax[2]
    ));
    dxf.push_str("0\nENDSEC\n");

    // Entities section
    dxf.push_str("0\nSECTION\n2\nENTITIES\n");

    fn add_line(dxf: &mut String, layer: &str, x1: f64, y1: f64, x2: f64, y2: f64) {
        dxf.push_str(&format!(
            "0\nLINE\n8\n{layer}\n10\n{:.4}\n20\n{:.4}\n30\n0.0\n11\n{:.4}\n21\n{:.4}\n31\n0.0\n",
            x1, y1, x2, y2
        ));
    }
    fn add_text(dxf: &mut String, layer: &str, x: f64, y: f64, h: f64, txt: &str) {
        dxf.push_str(&format!(
            "0\nTEXT\n8\n{layer}\n10\n{:.3}\n20\n{:.3}\n30\n0.0\n40\n{:.2}\n1\n{txt}\n",
            x, y, h
        ));
    }

    // 1. Planform bounding box (1:1 scale, mm)
    add_line(&mut dxf, "BOUNDING_BOX", bmin[0], bmin[1], bmax[0], bmin[1]);
    add_line(&mut dxf, "BOUNDING_BOX", bmax[0], bmin[1], bmax[0], bmax[1]);
    add_line(&mut dxf, "BOUNDING_BOX", bmax[0], bmax[1], bmin[0], bmax[1]);
    add_line(&mut dxf, "BOUNDING_BOX", bmin[0], bmax[1], bmin[0], bmin[1]);

    // 2. Exact mid-plane z = 0.0 mm outline of the outer envelope (1:1 scale, mm)
    for &(p0, p1) in &mid_segs {
        add_line(&mut dxf, "MIDPLANE_Z0_OUTLINE", p0[0], p0[1], p1[0], p1[1]);
    }

    // 3. Z-extent thickness envelope (max |z| vs x, offset to y_off = bmin[1] - 22.0 mm; not a section)
    let valid_z_bins: Vec<(f64, f64)> = (0..=n_bins)
        .filter_map(|bi| {
            if has_bin[bi] {
                Some((bmin[0] + bi as f64 * dx, z_max_bin[bi]))
            } else {
                None
            }
        })
        .collect();
    let y_off = bmin[1] - 22.0;
    for w in valid_z_bins.windows(2) {
        let (x0, z0) = w[0];
        let (x1, z1) = w[1];
        add_line(&mut dxf, "Z_EXTENT_ENVELOPE", x0, y_off + z0, x1, y_off + z1);
        add_line(&mut dxf, "Z_EXTENT_ENVELOPE", x0, y_off - z0, x1, y_off - z1);
    }

    // 4. Retention ring nominal 24.9 mm internal bore circle at decoded (cx, 0)
    dxf.push_str(&format!(
        "0\nCIRCLE\n8\nRING_BORE\n10\n{:.4}\n20\n0.0000\n30\n0.0000\n40\n{:.4}\n",
        ring_cx, ring_r
    ));

    // 5. Blade cross-section at 30 mm from the point (drawn to the right of the blade at x_draw = bmax[0] + 25.0 mm)
    let x_draw = bmax[0] + 25.0;
    for &(p0, p1) in &sec_segs {
        add_line(&mut dxf, "BLADE_SECTION_30MM", x_draw + p0[1], p0[0], x_draw + p1[1], p1[0]);
    }

    // 6. Title block & dimension callouts
    add_text(
        &mut dxf,
        "DIMENSIONS",
        bmin[0],
        bmax[1] + 14.0,
        3.2,
        &format!("JARAMBIT G5 WINNER: {design_id} (1:1 TRUE SCALE, UNITS: MILLIMETERS, 1 DRAWING UNIT = 1 MM)"),
    );
    add_text(
        &mut dxf,
        "DIMENSIONS",
        bmin[0],
        bmax[1] + 9.0,
        2.5,
        &format!(
            "OVERALL BOUNDING BOX: {:.2} mm (L) x {:.2} mm (H) x {:.2} mm (T) | RING BORE: {:.2} mm AT X = {:.2} mm",
            span[0], span[1], span[2], ring_r * 2.0, ring_cx
        ),
    );
    add_text(
        &mut dxf,
        "DIMENSIONS",
        bmin[0],
        bmax[1] + 4.5,
        2.5,
        "MATERIAL: Ti-6Al-4V, L-PBF, stress-relieved (ASTM F2924 min yield 825 MPa, 750.8 MPa at 80 C) | MID-PLANE Z=0 OUTLINE + 30 MM BLADE SECTION",
    );
    add_text(
        &mut dxf,
        "DIMENSIONS",
        bmin[0],
        bmin[1] - 12.0,
        2.2,
        "Z-EXTENT THICKNESS ENVELOPE (MAX |Z| VS X, NOT A SECTION):",
    );
    add_text(
        &mut dxf,
        "DIMENSIONS",
        x_draw - 12.0,
        bmax[1] + 4.5,
        2.2,
        "BLADE SECTION (30 MM FROM POINT):",
    );

    dxf.push_str("0\nENDSEC\n0\nEOF\n");
    dxf.into_bytes()
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
    jarambit_g5::sha256_bytes(bytes)
}

fn load_first_genome(csv_path: &Path) -> Result<(String, G5Genome), String> {
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
        if trimmed.starts_with("G5.") {
            let g = G5Genome::from_csv_row(trimmed)?;
            return Ok(("winner".to_string(), g));
        }
        if let Some((id, rest)) = trimmed.split_once(',') {
            let g = G5Genome::from_csv_row(rest)?;
            return Ok((id.to_string(), g));
        }
    }
    Err(format!("no valid G5 genome row found in {}", csv_path.display()))
}

fn write_print_spec_json(
    am_mesh: &SurfaceMesh,
    am_bytes: &[u8],
    h_am: f64,
    env_mesh: &SurfaceMesh,
    env_bytes: &[u8],
    h_env: f64,
    design_id: &str,
    genome: &G5Genome,
    out_dir: &Path,
) {
    let (am_bnd, am_nm, am_dmax) = census(&am_mesh.tris);
    let (env_bnd, env_nm, env_dmax) = census(&env_mesh.tris);
    assert_eq!((am_bnd, am_nm), (0, 0), "AMbuild mesh must be watertight");
    assert_eq!((env_bnd, env_nm), (0, 0), "envelope mesh must be watertight");

    let am_vol = volume(am_mesh);
    let env_vol = volume(env_mesh);
    let am_mass_g = am_vol * RHO_TI64_G_MM3;
    let env_mass_g = env_vol * RHO_TI64_G_MM3;
    let (bmin, bmax, span) = mesh_bbox_mm(am_mesh);

    let am_name = format!("{design_id}_AMbuild_mm.stl");
    let env_name = format!("{design_id}_envelope_mm.stl");
    let am_sha = sha256_hex(am_bytes);
    let env_sha = sha256_hex(env_bytes);

    let spec = json!({
        "design_id": design_id,
        "schema": genome.schema,
        "genome_sha256": genome.sha256(),
        "decoder_sha256": g5_decoder_sha256(),
        "units": "mm",
        "material": "Ti-6Al-4V, L-PBF, stress-relieved (ASTM F2924 min yield 825 MPa, 750.8 MPa at 80 C)",
        "density_g_mm3": RHO_TI64_G_MM3,

        "overall_dimensions_mm": {
            "length_x_mm": (span[0] * 100.0).round() / 100.0,
            "height_y_mm": (span[1] * 100.0).round() / 100.0,
            "thickness_z_mm": (span[2] * 100.0).round() / 100.0,
            "bbox_min_mm": bmin.map(|v| (v * 100.0).round() / 100.0),
            "bbox_max_mm": bmax.map(|v| (v * 100.0).round() / 100.0),
        },
        "order_notes": format!(
            "Units: mm | Overall dimensions: {:.2} x {:.2} x {:.2} mm | Material: Ti-6Al-4V LPBF | Remove powder through the ports in every hollow zone before stress relief",
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

fn emit_companions(
    am_mesh: &SurfaceMesh,
    env_mesh: &SurfaceMesh,
    design_id: &str,
    genome: &G5Genome,
    out_dir: &Path,
) {
    let params = genome.decode();
    let ring_cx = params["cx"];
    let ring_r = params["R_b"];
    let (bmin, bmax, span) = mesh_bbox_mm(am_mesh);
    let threemf_path = out_dir.join(format!("{design_id}_AMbuild_mm.3mf"));
    write_3mf_mm(am_mesh, design_id, span, &threemf_path);

    let dxf_bytes = write_dxf_drawing_mm(
        am_mesh, env_mesh, design_id, ring_cx, ring_r, bmin, bmax, span,
    );
    let dxf_path = out_dir.join(format!("{design_id}_drawing_mm.dxf"));
    fs::write(&dxf_path, &dxf_bytes).expect("write DXF drawing");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(|s| s.as_str()) == Some("--companion-from-dir") {
        let csv_path = PathBuf::from(args.get(2).expect("winner_genome.csv"));
        let stl_dir = PathBuf::from(args.get(3).expect("stl_dir"));
        let out_dir = PathBuf::from(args.get(4).expect("out_dir"));
        let h_spec: f64 = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(0.20);
        fs::create_dir_all(&out_dir).expect("create out_dir");
        let (design_id, genome) = load_first_genome(&csv_path).expect("load genome");
        let am_bytes = fs::read(stl_dir.join(format!("{design_id}_AMbuild_mm.stl")))
            .expect("read AMbuild_mm.stl");
        let env_bytes = fs::read(stl_dir.join(format!("{design_id}_envelope_mm.stl")))
            .expect("read envelope_mm.stl");
        let am_mesh = read_binary_stl_mm(&am_bytes);
        let env_mesh = read_binary_stl_mm(&env_bytes);
        let src_spec = stl_dir.join(format!("{design_id}_print_spec.json"));
        if !src_spec.exists() {
            write_print_spec_json(
                &am_mesh, &am_bytes, h_spec, &env_mesh, &env_bytes, h_spec, &design_id, &genome, &stl_dir,
            );
        }
        emit_companions(&am_mesh, &env_mesh, &design_id, &genome, &out_dir);
        return;
    }

    let mut csv_path = PathBuf::from("evidence/g5/v5.0.0/winner_genome.csv");
    let mut out_dir = PathBuf::from("docs/g5/v5.0.0/cad");
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

    let am_name = format!("{design_id}_AMbuild_mm.stl");
    let env_name = format!("{design_id}_envelope_mm.stl");
    let am_bytes = write_binary_stl_mm(&am_mesh, &format!("{design_id}_AMbuild"));
    let env_bytes = write_binary_stl_mm(&env_mesh, &format!("{design_id}_envelope"));

    let am_path = out_dir.join(&am_name);
    let env_path = out_dir.join(&env_name);
    fs::write(&am_path, &am_bytes).expect("write AMbuild STL");
    fs::write(&env_path, &env_bytes).expect("write envelope STL");

    write_print_spec_json(
        &am_mesh, &am_bytes, h_am, &env_mesh, &env_bytes, h_env, &design_id, &genome, &out_dir,
    );
}
