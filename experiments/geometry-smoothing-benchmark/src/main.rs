use geometry_smoothing_benchmark::{
    benchmark_passes, cross, dot, metrics, norm, smooth, sphere, sub, Mesh, Metrics, Settings,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    fs,
    path::PathBuf,
};

mod input_mode;

fn metric_json(m: &Metrics) -> String {
    format!("{{\"radial_rms_mm\":{:.17e},\"radial_p95_mm\":{:.17e},\"radial_max_mm\":{:.17e},\"mean_radius_mm\":{:.17e},\"volume_mm3\":{:.17e},\"min_double_area_mm2\":{:.17e},\"max_displacement_mm\":{:.17e},\"moved_vertices\":{},\"euler_characteristic\":{},\"boundary_vertices\":{}}}",
        m.radial_rms_mm,m.radial_p95_mm,m.radial_max_mm,m.mean_radius_mm,m.volume_mm3,
        m.min_double_area_mm2,m.max_displacement_mm,m.moved_vertices,m.euler_characteristic,m.boundary_vertices)
}

fn obj(mesh: &Mesh) -> String {
    let mut text =
        String::from("# Synthetic sphere housing benchmark; coordinates in millimeters\n");
    for p in &mesh.vertices {
        writeln!(text, "v {:.17e} {:.17e} {:.17e}", p[0], p[1], p[2]).unwrap();
    }
    for f in &mesh.faces {
        writeln!(text, "f {} {} {}", f[0] + 1, f[1] + 1, f[2] + 1).unwrap();
    }
    text
}

fn stl(mesh: &Mesh) -> String {
    let mut text = String::from("solid synthetic_sphere_mm\n");
    for &[a, b, c] in &mesh.faces {
        let n = cross(
            sub(mesh.vertices[b], mesh.vertices[a]),
            sub(mesh.vertices[c], mesh.vertices[a]),
        );
        let length = norm(n);
        writeln!(
            text,
            " facet normal {:.17e} {:.17e} {:.17e}\n  outer loop",
            n[0] / length,
            n[1] / length,
            n[2] / length
        )
        .unwrap();
        for i in [a, b, c] {
            let p = mesh.vertices[i];
            writeln!(text, "   vertex {:.17e} {:.17e} {:.17e}", p[0], p[1], p[2]).unwrap();
        }
        text.push_str("  endloop\n endfacet\n");
    }
    text.push_str("endsolid synthetic_sphere_mm\n");
    text
}

fn comparison(before: &Mesh, after: &Mesh, radius: f64) -> String {
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1000\" height=\"650\" viewBox=\"0 0 1000 650\"><rect width=\"1000\" height=\"650\" fill=\"#101820\"/><g font-family=\"system-ui,sans-serif\" fill=\"#f4f8fc\"><text x=\"32\" y=\"38\" font-size=\"24\">Actual geometry: synthetic rough sphere → smoothed mesh</text><text x=\"32\" y=\"65\" font-size=\"15\">Same camera and scale. Wireframe only. Orange circle = analytic reference.</text>");
    for (index, (mesh, label)) in [
        (before, "BEFORE: rough coordinates"),
        (after, "AFTER: changed coordinates"),
    ]
    .into_iter()
    .enumerate()
    {
        let cx = 250.0 + 500.0 * index as f64;
        let zoom = 185.0 / radius;
        let project = |p: [f64; 3]| {
            (
                cx + zoom * (0.8 * p[0] - 0.6 * p[1]),
                300.0 + zoom * (0.3 * p[0] + 0.4 * p[1] - 0.8660254037844386 * p[2]),
            )
        };
        writeln!(
            svg,
            "<text x=\"{}\" y=\"100\" text-anchor=\"middle\" font-size=\"18\">{label}</text>",
            cx
        )
        .unwrap();
        let mut visible_edges = BTreeSet::new();
        let mut silhouettes: BTreeMap<(usize, usize), (usize, usize)> = BTreeMap::new();
        for &[a, b, c] in &mesh.faces {
            let n = cross(
                sub(mesh.vertices[b], mesh.vertices[a]),
                sub(mesh.vertices[c], mesh.vertices[a]),
            );
            let front = dot(n, [0.5196152422706632, 0.6928203230275509, 0.5]) > 0.;
            for (i, j) in [(a, b), (b, c), (c, a)] {
                let edge = (i.min(j), i.max(j));
                let count = silhouettes.entry(edge).or_default();
                if front {
                    count.0 += 1;
                    visible_edges.insert(edge);
                } else {
                    count.1 += 1;
                }
            }
        }
        for ((a, b), color, width) in visible_edges
            .into_iter()
            .map(|e| (e, "#638399", 0.35))
            .chain(
                silhouettes
                    .into_iter()
                    .filter(|(_, n)| n.0 > 0 && n.1 > 0)
                    .map(|(e, _)| (e, "#6fe3cc", 1.5)),
            )
        {
            let (x, y) = project(mesh.vertices[a]);
            let (u, v) = project(mesh.vertices[b]);
            writeln!(svg,"<path d=\"M{x:.3},{y:.3}L{u:.3},{v:.3}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"{width}\"/>").unwrap();
        }
        writeln!(svg,"<circle cx=\"{cx}\" cy=\"300\" r=\"185\" fill=\"none\" stroke=\"#ffb35c\" stroke-dasharray=\"5 5\"/>").unwrap();
    }
    svg.push_str("<text x=\"32\" y=\"510\" font-size=\"17\">Vertex radial error, before (pink) and after (green), in input vertex order</text>");
    let max_error = before
        .vertices
        .iter()
        .map(|&p| (norm(p) - radius).abs())
        .fold(0., f64::max)
        .max(1e-9);
    svg.push_str("<path d=\"M32,575L968,575\" stroke=\"#ffb35c\" stroke-dasharray=\"5 5\"/>");
    for (mesh, color) in [(before, "#f29db3"), (after, "#6fe3cc")] {
        let mut points = String::new();
        // A declared bounded sample for the graphic only; metrics use every vertex.
        for i in 0..mesh.vertices.len().min(300) {
            let x = 32.0 + 936.0 * i as f64 / (mesh.vertices.len().min(300) - 1) as f64;
            let y = 575.0 - 45.0 * (norm(mesh.vertices[i]) - radius) / max_error;
            write!(points, "{x:.2},{y:.2} ").unwrap();
        }
        writeln!(
            svg,
            "<polyline points=\"{points}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"1\"/>"
        )
        .unwrap();
    }
    svg.push_str("<text x=\"32\" y=\"640\" font-size=\"13\">Graph displays first 300 vertices; numerical checks use all vertices and faces. This is a mesh benchmark, not B-rep CAD.</text></g></svg>");
    svg
}

fn run() -> Result<bool, String> {
    let mut settings = Settings::default();
    let mut out = PathBuf::from("results/default");
    let mut radius = 30.0;
    let mut noise = 0.8;
    let mut subdivisions = 3;
    let mut input = None;
    let mut label = String::from("input mesh");
    let mut pin_feature_deg = None;
    let command = std::env::args().collect::<Vec<_>>().join(" ");
    let mut args = std::env::args().skip(1);
    let mut seen = BTreeSet::new();
    while let Some(key) = args.next() {
        if key == "--help" {
            println!("Synthetic geometry benchmark (millimeters)\n--out NEW_DIRECTORY --iterations 20 --lambda 0.5 --mu -0.5 --max-displacement-mm 2 --radius-mm 30 --noise-mm 0.8 --subdivisions 3\nOutputs actual rough, smooth and reference OBJ/STL meshes, CSV, JSON, Markdown and SVG. Existing output directories are refused. Exit 2 means benchmark criteria failed.");
            return Ok(true);
        }
        if !seen.insert(key.clone()) {
            return Err(format!("duplicate option: {key}"));
        }
        if key == "--normal-only" {
            settings.normal_only = true;
            continue;
        }
        let value = args.next().ok_or_else(|| format!("missing value: {key}"))?;
        match key.as_str() {
            "--input" => input = Some(PathBuf::from(value)),
            "--label" => label = value,
            "--pin-feature-angle" => {
                pin_feature_deg = Some(value.parse().map_err(|_| "invalid feature angle")?)
            }
            "--out" => out = PathBuf::from(value),
            "--iterations" => {
                settings.iterations = value.parse().map_err(|_| "invalid iterations")?
            }
            "--lambda" => settings.lambda = value.parse().map_err(|_| "invalid lambda")?,
            "--mu" => settings.mu = value.parse().map_err(|_| "invalid mu")?,
            "--max-displacement-mm" => {
                settings.max_displacement_mm = value.parse().map_err(|_| "invalid displacement")?
            }
            "--radius-mm" => radius = value.parse().map_err(|_| "invalid radius")?,
            "--noise-mm" => noise = value.parse().map_err(|_| "invalid noise")?,
            "--subdivisions" => subdivisions = value.parse().map_err(|_| "invalid subdivisions")?,
            _ => return Err(format!("unknown option: {key}")),
        }
    }
    settings.validate()?;
    if let Some(input) = input {
        // Input-mesh mode: the synthetic sphere options do not apply.
        if seen.contains("--radius-mm")
            || seen.contains("--noise-mm")
            || seen.contains("--subdivisions")
        {
            return Err("--radius-mm, --noise-mm and --subdivisions are synthetic-only; not valid with --input".into());
        }
        if !seen.contains("--out") {
            return Err("--input requires --out NEW_DIRECTORY".into());
        }
        return input_mode::run(&input_mode::Request {
            input,
            out,
            settings,
            label,
            pin_feature_deg,
            command,
        });
    }
    if seen.contains("--label")
        || seen.contains("--normal-only")
        || seen.contains("--pin-feature-angle")
    {
        return Err(
            "--label, --normal-only and --pin-feature-angle are only valid with --input".into(),
        );
    }
    if out.exists() {
        return Err("output directory already exists; select a new directory".into());
    }
    let reference = sphere(subdivisions, radius, 0.)?;
    let source = sphere(subdivisions, radius, noise)?;
    let result = smooth(&source, settings, &vec![false; source.vertices.len()])?;
    let before = metrics(&source, &source, radius)?;
    let after = metrics(&result, &source, radius)?;
    let target = metrics(&reference, &reference, radius)?;
    let passed = benchmark_passes(&before, &after, &target, settings);
    fs::create_dir_all(out.parent().ok_or("output has no parent")?).map_err(|e| e.to_string())?;
    fs::create_dir(&out).map_err(|e| e.to_string())?;
    let write = |name: &str, contents: String| {
        fs::write(out.join(name), contents).map_err(|e| e.to_string())
    };
    for (label, mesh) in [
        ("rough", &source),
        ("smoothed", &result),
        ("reference", &reference),
    ] {
        write(&format!("{label}.obj"), obj(mesh))?;
        write(&format!("{label}.stl"), stl(mesh))?;
    }
    let mut csv=String::from("vertex,source_x_mm,source_y_mm,source_z_mm,smoothed_x_mm,smoothed_y_mm,smoothed_z_mm,source_radial_error_mm,smoothed_radial_error_mm,displacement_mm\n");
    for (i, (&a, &b)) in source.vertices.iter().zip(&result.vertices).enumerate() {
        writeln!(
            csv,
            "{i},{:.17e},{:.17e},{:.17e},{:.17e},{:.17e},{:.17e},{:.17e},{:.17e},{:.17e}",
            a[0],
            a[1],
            a[2],
            b[0],
            b[1],
            b[2],
            norm(a) - radius,
            norm(b) - radius,
            norm(sub(a, b))
        )
        .unwrap();
    }
    write("vertices.csv", csv)?;
    write("comparison.svg", comparison(&source, &result, radius))?;
    write("metrics.json",format!("{{\n\"schema\":\"synthetic_geometry_smoothing.v1\",\n\"fixture\":\"deterministic_rough_icosphere\",\n\"representation\":\"triangle_mesh_not_brep\",\n\"units\":\"mm\",\n\"vertices\":{},\n\"triangles\":{},\n\"radius_mm\":{radius},\n\"noise_amplitude_mm\":{noise},\n\"subdivisions\":{subdivisions},\n\"settings\":{{\"iterations\":{},\"lambda\":{},\"mu\":{},\"max_displacement_mm\":{}}},\n\"before\":{},\n\"after\":{},\n\"reference\":{},\n\"topology_unchanged\":{},\n\"benchmark_passed\":{passed}\n}}\n",source.vertices.len(),source.faces.len(),settings.iterations,settings.lambda,settings.mu,settings.max_displacement_mm,metric_json(&before),metric_json(&after),metric_json(&target),source.faces==result.faces))?;
    let reduction = 100.0 * (1.0 - after.radial_rms_mm / before.radial_rms_mm);
    let reduction_text = if reduction.is_finite() {
        format!("{reduction:.3}%")
    } else {
        "undefined (zero input error)".into()
    };
    write("REPORT.md",format!("# Synthetic geometry smoothing benchmark\n\nResult: **{}** on a made-up rough sphere; radius {radius} mm. Geometry changed at {} / {} vertices. Topology remains {} triangles.\n\n| Measurement | Before | After |\n|---|---:|---:|\n| Vertex radial RMS error, mm | {:.6} | {:.6} |\n| Vertex radial p95 error, mm | {:.6} | {:.6} |\n| Vertex radial maximum error, mm | {:.6} | {:.6} |\n| Mean radius, mm | {:.6} | {:.6} |\n| Enclosed volume, mm³ | {:.6} | {:.6} |\n\nRMS error reduction: {reduction_text}. Maximum movement: {:.6} mm (budget {} mm). Volume difference from the clean reference mesh: {:.6}%.\n\nDeclared pass criteria: vertices move; RMS error falls at least 50%; p95 improves; maximum error does not increase; volume differs from the clean reference mesh by at most 1%; closed connected oriented mesh retains Euler characteristic 2; no degenerate or reversed faces; total displacement budget holds. All metrics use all vertices/faces.\n\nMethod: uniform Taubin-style lambda/mu filtering, {} iterations, lambda {}, mu {}. Pins and open boundaries are fixed. The method uses only noisy input positions and adjacency; the known sphere is used only for assessment.\n\n[Wireframe comparison](comparison.svg) · [Vertex coordinates](vertices.csv) · [Machine-readable measurements](metrics.json) · [Rough STL](rough.stl) · [Smoothed STL](smoothed.stl) · [Reference STL](reference.stl)\n\nLimits: a synthetic mesh benchmark, not reconstructed analytic B-rep/STEP CAD. Vertex-to-sphere error is exact for the sampled vertices; no continuous Hausdorff bound is claimed. Volume uses the triangulated reference, not the analytic sphere. No exhaustive triangle-intersection check, general feature-preservation claim, functional evaluation, or real-object qualification is supplied. Passing this fixture does not establish success on arbitrary CAD.\n",if passed{"PASS"}else{"FAIL"},after.moved_vertices,source.vertices.len(),source.faces.len(),before.radial_rms_mm,after.radial_rms_mm,before.radial_p95_mm,after.radial_p95_mm,before.radial_max_mm,after.radial_max_mm,before.mean_radius_mm,after.mean_radius_mm,before.volume_mm3,after.volume_mm3,after.max_displacement_mm,settings.max_displacement_mm,100.0*(after.volume_mm3/target.volume_mm3-1.0),settings.iterations,settings.lambda,settings.mu))?;
    println!(
        "{}: {} vertices moved; RMS {:.6} → {:.6} mm; reduction {}; report {}",
        if passed { "PASS" } else { "FAIL" },
        after.moved_vertices,
        before.radial_rms_mm,
        after.radial_rms_mm,
        reduction_text,
        out.join("REPORT.md").display()
    );
    Ok(passed)
}

fn main() {
    match run() {
        Ok(true) => {}
        Ok(false) => std::process::exit(2),
        Err(error) => {
            eprintln!("ERROR: {error}");
            std::process::exit(1);
        }
    }
}
