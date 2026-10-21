//! Input-mesh mode: smooth a supplied STL with fail-closed checks before and after
//! smoothing. Geometry only. The smoothed mesh is not re-scored, re-assessed,
//! certified or qualified, and the sphere criteria of the synthetic benchmark do not apply.
use geometry_smoothing_benchmark::{
    cross, dot,
    input::{self, Audit, Format},
    norm,
    sha256::sha256_hex,
    smooth_traced, sub, Mesh, Point, Result, Settings,
};
use std::{
    fmt::Write as _,
    fs,
    io::Write as _,
    path::{Path, PathBuf},
};

const TOOL: &str = concat!("geometry-smoothing-benchmark ", env!("CARGO_PKG_VERSION"));

pub struct Request {
    pub input: PathBuf,
    pub out: PathBuf,
    pub settings: Settings,
    pub label: String,
    pub command: String,
}

struct InputInfo {
    path: String,
    bytes: usize,
    sha256: String,
}

struct Output {
    name: &'static str,
    bytes: usize,
    sha256: String,
}

struct Measure {
    after: Audit,
    moved: usize,
    max_displacement: f64,
    rms_displacement: f64,
    volume_before: f64,
    volume_after: f64,
    bounds_before: (Point, Point),
    bounds_after: (Point, Point),
}

/// Everything written to the output directory, kept until the end so that failed runs
/// still record what happened.
struct Record {
    input: InputInfo,
    format: Option<Format>,
    audit: Option<Audit>,
    reasons: Vec<String>,
    iterations_completed: usize,
    measure: Option<Measure>,
    outputs: Vec<Output>,
}

/// `Ok(true)` passes, `Ok(false)` means a fail-closed check stopped the run after
/// measurements were recorded (exit 2), and `Err` means rejected input or an operational error (exit 1).
pub fn run(request: &Request) -> Result<bool> {
    let bytes = fs::read(&request.input)
        .map_err(|e| format!("cannot read input {}: {e}", request.input.display()))?;
    create_output_dir(&request.out)?;
    let mut record = Record {
        input: InputInfo {
            path: request.input.display().to_string(),
            bytes: bytes.len(),
            sha256: sha256_hex(&bytes),
        },
        format: None,
        audit: None,
        reasons: Vec::new(),
        iterations_completed: 0,
        measure: None,
        outputs: Vec::new(),
    };
    let (format, triangles) = match input::parse_stl(&bytes) {
        Ok(parsed) => parsed,
        Err(error) => {
            record.reasons.push(error.clone());
            write_new(&request.out, "input-check.json", &input_check_json(&record))?;
            return Err(format!("input rejected: {error}"));
        }
    };
    record.format = Some(format);
    let source = input::weld(&triangles);
    let audit = input::audit(&source);
    record.reasons = audit.failures();
    record.audit = Some(audit.clone());
    write_new(&request.out, "input-check.json", &input_check_json(&record))?;
    if !record.reasons.is_empty() {
        return Err(format!(
            "input rejected before smoothing: {}",
            record.reasons.join("; ")
        ));
    }

    let pins = vec![false; source.vertices.len()];
    let trace = smooth_traced(&source, request.settings, &pins)?;
    record.iterations_completed = trace.iterations_completed;
    if let Some(error) = &trace.error {
        record.reasons.push(format!(
            "smoothing stopped in iteration {} of {}: {error}",
            trace.iterations_completed + 1,
            request.settings.iterations
        ));
    }
    let measure = Measure::new(&source, &trace.mesh);
    let reasons = post_failures(&source, &trace.mesh, &audit, &measure, request.settings);
    record.reasons.extend(reasons);
    record.measure = Some(measure);
    // A stopped run keeps its state under distinct names, so it cannot pass as a smoothed result.
    let (mesh_name, csv_name) = if record.reasons.is_empty() {
        ("smoothed.stl", "displacement.csv")
    } else {
        ("failed-state.stl", "failed-state-displacement.csv")
    };
    if trace.mesh.vertices.iter().flatten().all(|x| x.is_finite()) {
        record.outputs.push(write_new(
            &request.out,
            mesh_name,
            &smoothed_stl(&trace.mesh),
        )?);
        record.outputs.push(write_new(
            &request.out,
            csv_name,
            &displacement_csv(&source, &trace.mesh),
        )?);
    }
    finish(request, &record)?;
    Ok(record.reasons.is_empty())
}

fn finish(request: &Request, record: &Record) -> Result<()> {
    write_new(&request.out, "metrics.json", &metrics_json(request, record))?;
    write_new(&request.out, "REPORT.md", &report_md(request, record))?;
    let status = if record.reasons.is_empty() {
        "PASS"
    } else {
        "FAIL"
    };
    println!(
        "{status}: input mesh {} (sha256 {}); report {}",
        request.input.display(),
        record.input.sha256,
        request.out.join("REPORT.md").display()
    );
    for reason in &record.reasons {
        println!("reason: {reason}");
    }
    Ok(())
}

fn face_normal(mesh: &Mesh, face: [usize; 3]) -> Point {
    let [a, b, c] = face;
    cross(
        sub(mesh.vertices[b], mesh.vertices[a]),
        sub(mesh.vertices[c], mesh.vertices[a]),
    )
}

impl Measure {
    fn new(source: &Mesh, smoothed: &Mesh) -> Self {
        let displacements: Vec<f64> = source
            .vertices
            .iter()
            .zip(&smoothed.vertices)
            .map(|(a, b)| norm(sub(*b, *a)))
            .collect();
        let count = displacements.len().max(1) as f64;
        Self {
            after: input::audit(smoothed),
            moved: displacements.iter().filter(|&&d| d > 1e-12).count(),
            max_displacement: displacements.iter().copied().fold(0.0, f64::max),
            rms_displacement: (displacements.iter().map(|d| d * d).sum::<f64>() / count).sqrt(),
            volume_before: input::signed_volume(source),
            volume_after: input::signed_volume(smoothed),
            bounds_before: input::bounds(source),
            bounds_after: input::bounds(smoothed),
        }
    }

    fn all_finite(&self) -> bool {
        [
            self.max_displacement,
            self.rms_displacement,
            self.volume_before,
            self.volume_after,
        ]
        .iter()
        .chain(self.bounds_before.0.iter())
        .chain(self.bounds_before.1.iter())
        .chain(self.bounds_after.0.iter())
        .chain(self.bounds_after.1.iter())
        .all(|x| x.is_finite())
    }
}

/// Checks that need the smoothed mesh. Smoothing itself already refuses to continue after
/// a reversed face, so these recheck the final result against the input.
fn post_failures(
    source: &Mesh,
    smoothed: &Mesh,
    before: &Audit,
    measure: &Measure,
    settings: Settings,
) -> Vec<String> {
    let after = &measure.after;
    let mut reasons = Vec::new();
    if after.vertices != before.vertices
        || after.faces != before.faces
        || after.edges != before.edges
        || after.euler_characteristic != before.euler_characteristic
    {
        reasons.push(format!(
            "topology changed: V {}->{}, E {}->{}, F {}->{}, Euler {}->{}",
            before.vertices,
            after.vertices,
            before.edges,
            after.edges,
            before.faces,
            after.faces,
            before.euler_characteristic,
            after.euler_characteristic
        ));
    }
    reasons.extend(
        after
            .failures()
            .into_iter()
            .map(|reason| format!("output: {reason}")),
    );
    let reversed = source
        .faces
        .iter()
        .filter(|&&f| dot(face_normal(source, f), face_normal(smoothed, f)) <= 0.0)
        .count();
    if reversed > 0 {
        reasons.push(format!("{reversed} faces reversed relative to input"));
    }
    if smoothed.vertices.iter().flatten().any(|x| !x.is_finite()) {
        reasons.push("nonfinite output positions".into());
    }
    if measure.max_displacement > settings.max_displacement_mm + 1e-12 {
        reasons.push(format!(
            "max displacement {} mm exceeds budget {} mm",
            measure.max_displacement, settings.max_displacement_mm
        ));
    }
    if !measure.volume_before.is_finite() || measure.volume_before.abs() < 1e-9 {
        reasons.push("input volume is zero or nonfinite".into());
    }
    if !measure.all_finite() {
        reasons.push("nonfinite measurement".into());
    }
    reasons
}

fn create_output_dir(out: &Path) -> Result<()> {
    if out.exists() {
        return Err("output directory already exists; select a new directory".into());
    }
    fs::create_dir_all(out.parent().ok_or("output has no parent")?).map_err(|e| e.to_string())?;
    fs::create_dir(out).map_err(|e| e.to_string())
}

/// Creates a file that must not exist yet, so outputs are never overwritten.
fn write_new(dir: &Path, name: &'static str, contents: &str) -> Result<Output> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(dir.join(name))
        .map_err(|e| format!("cannot create {name}: {e}"))?;
    file.write_all(contents.as_bytes())
        .map_err(|e| format!("cannot write {name}: {e}"))?;
    Ok(Output {
        name,
        bytes: contents.len(),
        sha256: sha256_hex(contents.as_bytes()),
    })
}

fn smoothed_stl(mesh: &Mesh) -> String {
    let mut text = String::from("solid smoothed_input_mesh\n");
    for &face in &mesh.faces {
        let n = face_normal(mesh, face);
        let length = norm(n);
        let unit = if length > 0.0 && length.is_finite() {
            [n[0] / length, n[1] / length, n[2] / length]
        } else {
            [0.0; 3]
        };
        writeln!(
            text,
            "  facet normal {} {} {}\n    outer loop",
            unit[0], unit[1], unit[2]
        )
        .unwrap();
        for i in face {
            let p = mesh.vertices[i];
            writeln!(text, "      vertex {} {} {}", p[0], p[1], p[2]).unwrap();
        }
        text.push_str("    endloop\n  endfacet\n");
    }
    text.push_str("endsolid smoothed_input_mesh\n");
    text
}

fn displacement_csv(source: &Mesh, smoothed: &Mesh) -> String {
    let mut text = String::from(
        "vertex,x_before_mm,y_before_mm,z_before_mm,x_after_mm,y_after_mm,z_after_mm,displacement_mm\n",
    );
    for (i, (a, b)) in source.vertices.iter().zip(&smoothed.vertices).enumerate() {
        writeln!(
            text,
            "{i},{},{},{},{},{},{},{}",
            a[0],
            a[1],
            a[2],
            b[0],
            b[1],
            b[2],
            norm(sub(*b, *a))
        )
        .unwrap();
    }
    text
}

/// JSON number; non-finite values become null rather than invalid JSON.
fn num(x: f64) -> String {
    if x.is_finite() {
        format!("{x}")
    } else {
        "null".into()
    }
}

fn point_json(p: Point) -> String {
    format!("[{},{},{}]", num(p[0]), num(p[1]), num(p[2]))
}

fn json_str(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if c.is_control() => write!(out, "\\u{:04x}", c as u32).unwrap(),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn audit_json(a: &Audit) -> String {
    format!(
        "{{\"vertices\":{},\"faces\":{},\"edges\":{},\"open_edges\":{},\"nonmanifold_edges\":{},\"inconsistent_edges\":{},\"degenerate_faces\":{},\"duplicate_faces\":{},\"unused_vertices\":{},\"components\":{},\"euler_characteristic\":{}}}",
        a.vertices,
        a.faces,
        a.edges,
        a.open_edges,
        a.nonmanifold_edges,
        a.inconsistent_edges,
        a.degenerate_faces,
        a.duplicate_faces,
        a.unused_vertices,
        a.components,
        a.euler_characteristic
    )
}

fn reasons_json(reasons: &[String]) -> String {
    let items: Vec<String> = reasons.iter().map(|r| json_str(r)).collect();
    format!("[{}]", items.join(","))
}

fn input_check_json(record: &Record) -> String {
    let status = if record.reasons.is_empty() {
        "accepted"
    } else {
        "rejected"
    };
    let format = record.format.map_or("null".into(), |f| json_str(f.name()));
    let audit = record.audit.as_ref().map_or("null".into(), audit_json);
    format!(
        "{{\n\"schema\":\"smoothing_input_check.v1\",\n\"status\":\"{status}\",\n\"reasons\":{},\n\"input\":{{\"path\":{},\"bytes\":{},\"sha256\":\"{}\",\"format\":{format}}},\n\"audit\":{audit}\n}}\n",
        reasons_json(&record.reasons),
        json_str(&record.input.path),
        record.input.bytes,
        record.input.sha256
    )
}

fn metrics_json(request: &Request, record: &Record) -> String {
    let status = if record.reasons.is_empty() {
        "pass"
    } else {
        "fail"
    };
    let format = record.format.map_or("null".into(), |f| json_str(f.name()));
    let audit = record.audit.as_ref().map_or("null".into(), audit_json);
    let result = match &record.measure {
        None => "null".into(),
        Some(m) => format!(
            "{{\"output_audit\":{},\"moved_vertices\":{},\"max_displacement_mm\":{},\"rms_displacement_mm\":{},\"displacement_at_budget\":{},\"volume_mm3\":{{\"before\":{},\"after\":{},\"change_percent\":{}}},\"bounding_box_mm\":{{\"before\":{{\"min\":{},\"max\":{}}},\"after\":{{\"min\":{},\"max\":{}}},\"change\":{{\"min\":{},\"max\":{}}}}}}}",
            audit_json(&m.after),
            m.moved,
            num(m.max_displacement),
            num(m.rms_displacement),
            m.max_displacement >= request.settings.max_displacement_mm * (1.0 - 1e-9),
            num(m.volume_before),
            num(m.volume_after),
            num(100.0 * (m.volume_after / m.volume_before - 1.0)),
            point_json(m.bounds_before.0),
            point_json(m.bounds_before.1),
            point_json(m.bounds_after.0),
            point_json(m.bounds_after.1),
            point_json(sub(m.bounds_after.0, m.bounds_before.0)),
            point_json(sub(m.bounds_after.1, m.bounds_before.1)),
        ),
    };
    let outputs: Vec<String> = record
        .outputs
        .iter()
        .map(|o| {
            format!(
                "{{\"name\":{},\"bytes\":{},\"sha256\":\"{}\"}}",
                json_str(o.name),
                o.bytes,
                o.sha256
            )
        })
        .collect();
    let s = request.settings;
    format!(
        "{{\n\"schema\":\"smoothing_input_mesh.v1\",\n\"status\":\"{status}\",\n\"not_qualified\":true,\n\"label\":{},\n\"tool\":{},\n\"command\":{},\n\"input\":{{\"path\":{},\"bytes\":{},\"sha256\":\"{}\",\"format\":{format}}},\n\"input_audit\":{audit},\n\"settings\":{{\"iterations\":{},\"lambda\":{},\"mu\":{},\"max_displacement_mm\":{},\"iterations_completed\":{}}},\n\"pins\":{{\"user_pinned_vertices\":0,\"boundary_pinned_vertices\":0}},\n\"result\":{result},\n\"outputs\":[{}],\n\"reasons\":{}\n}}\n",
        json_str(&request.label),
        json_str(TOOL),
        json_str(&request.command),
        json_str(&record.input.path),
        record.input.bytes,
        record.input.sha256,
        s.iterations,
        num(s.lambda),
        num(s.mu),
        num(s.max_displacement_mm),
        record.iterations_completed,
        outputs.join(","),
        reasons_json(&record.reasons),
    )
}

fn report_md(request: &Request, record: &Record) -> String {
    let status = if record.reasons.is_empty() {
        "PASS"
    } else {
        "FAIL"
    };
    let s = request.settings;
    let mut text = String::new();
    writeln!(text, "# Input mesh smoothing report\n").unwrap();
    writeln!(
        text,
        "Status: **{status}** (geometry checks only). Label: {}\n",
        request.label
    )
    .unwrap();
    text.push_str("This is a geometry study. The smoothed mesh is NOT re-scored, re-assessed, certified or qualified. It is a triangle mesh, not B-rep CAD, and nothing in this report measures function. The synthetic sphere criteria do not apply to this input.\n\n");
    writeln!(text, "Tool: {TOOL}\n\nCommand: `{}`\n", request.command).unwrap();
    text.push_str("## Input\n\n");
    writeln!(text, "- File: `{}`", record.input.path).unwrap();
    writeln!(text, "- Bytes: {}", record.input.bytes).unwrap();
    writeln!(text, "- SHA-256: `{}`", record.input.sha256).unwrap();
    writeln!(
        text,
        "- Format: {}\n",
        record.format.map_or("not parsed", Format::name)
    )
    .unwrap();
    if let Some(a) = &record.audit {
        text.push_str("## Input audit (before smoothing)\n\n| Measure | Value |\n|---|---:|\n");
        for (name, value) in audit_rows(a) {
            writeln!(text, "| {name} | {value} |").unwrap();
        }
        text.push('\n');
    }
    text.push_str("## Smoothing\n\n");
    writeln!(
        text,
        "Method: `smooth_traced()`, the synthetic benchmark arithmetic, unchanged: {} iterations of paired lambda {} and mu {} Laplacian passes; {} completed. Total displacement is capped at {} mm against the input. Boundary and pinned vertices stay fixed. Pins: none. Sharp-feature pinning is not implemented, and an open boundary is rejected before smoothing.\n",
        s.iterations, s.lambda, s.mu, record.iterations_completed, s.max_displacement_mm
    )
    .unwrap();
    if let Some(m) = &record.measure {
        let (min_b, max_b) = m.bounds_before;
        let (min_a, max_a) = m.bounds_after;
        text.push_str("## Measurements\n\n| Measure | Before | After |\n|---|---:|---:|\n");
        let before = record.audit.as_ref();
        if let Some(b) = before {
            writeln!(text, "| Vertices | {} | {} |", b.vertices, m.after.vertices).unwrap();
            writeln!(text, "| Faces | {} | {} |", b.faces, m.after.faces).unwrap();
            writeln!(text, "| Edges | {} | {} |", b.edges, m.after.edges).unwrap();
            writeln!(
                text,
                "| Euler characteristic | {} | {} |",
                b.euler_characteristic, m.after.euler_characteristic
            )
            .unwrap();
        }
        writeln!(
            text,
            "| Signed volume, mm³ | {:.6} | {:.6} |",
            m.volume_before, m.volume_after
        )
        .unwrap();
        writeln!(
            text,
            "| Bounding box min, mm | {:.6}, {:.6}, {:.6} | {:.6}, {:.6}, {:.6} |",
            min_b[0], min_b[1], min_b[2], min_a[0], min_a[1], min_a[2]
        )
        .unwrap();
        writeln!(
            text,
            "| Bounding box max, mm | {:.6}, {:.6}, {:.6} | {:.6}, {:.6}, {:.6} |",
            max_b[0], max_b[1], max_b[2], max_a[0], max_a[1], max_a[2]
        )
        .unwrap();
        text.push('\n');
        writeln!(
            text,
            "Volume change: {:.6}%. Bounding box change, min/max per axis (mm): ({:.6}, {:.6}, {:.6}) / ({:.6}, {:.6}, {:.6}).",
            100.0 * (m.volume_after / m.volume_before - 1.0),
            min_a[0] - min_b[0], min_a[1] - min_b[1], min_a[2] - min_b[2],
            max_a[0] - max_b[0], max_a[1] - max_b[1], max_a[2] - max_b[2]
        )
        .unwrap();
        writeln!(
            text,
            "\nVertices moved: {} of {}. Max displacement {:.6} mm (budget {} mm). RMS displacement {:.6} mm.\n",
            m.moved,
            m.after.vertices,
            m.max_displacement,
            s.max_displacement_mm,
            m.rms_displacement
        )
        .unwrap();
    } else {
        text.push_str("Smoothing did not return a mesh; no measurements after smoothing.\n\n");
    }
    if !record.outputs.is_empty() {
        text.push_str("## Outputs\n\n| File | Bytes | SHA-256 |\n|---|---:|---|\n");
        for o in &record.outputs {
            writeln!(text, "| {} | {} | `{}` |", o.name, o.bytes, o.sha256).unwrap();
        }
        text.push('\n');
    }
    if !record.reasons.is_empty() {
        text.push_str("## Reasons\n\n");
        for reason in &record.reasons {
            writeln!(text, "- {reason}").unwrap();
        }
        text.push('\n');
    }
    text.push_str("## Limits\n\nSmoothing moves vertices of the supplied triangle mesh. It does not reconstruct analytic surfaces, check continuous Hausdorff distance, or check feature preservation. No self-intersection test was run. The winner is not qualified by this report.\n");
    text
}

fn audit_rows(a: &Audit) -> Vec<(&'static str, String)> {
    vec![
        ("Vertices", a.vertices.to_string()),
        ("Faces", a.faces.to_string()),
        ("Edges", a.edges.to_string()),
        ("Euler characteristic", a.euler_characteristic.to_string()),
        ("Open edges", a.open_edges.to_string()),
        ("Non-manifold edges", a.nonmanifold_edges.to_string()),
        (
            "Inconsistent-winding edges",
            a.inconsistent_edges.to_string(),
        ),
        ("Degenerate faces", a.degenerate_faces.to_string()),
        ("Duplicate faces", a.duplicate_faces.to_string()),
        ("Unused vertices", a.unused_vertices.to_string()),
        ("Connected components", a.components.to_string()),
    ]
}
