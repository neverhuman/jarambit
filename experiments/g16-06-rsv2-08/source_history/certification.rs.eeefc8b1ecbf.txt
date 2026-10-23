//! Measurements on the actual faceted export and content identity for certification evidence.
use crate::{
    export::{self, Solid},
    genome::Genome,
    geom::*,
    pheno::Pheno,
};
use serde::Serialize;

pub const EXPORT_AREA_MM2: f64 = 1.0;

/// Includes the canonical genome, requirements and numerical implementation. Labels are display data.
pub fn identity(g: &Genome) -> String {
    let row = g.csv_row();
    let mut bytes = row
        .rsplit(',')
        .next()
        .expect("material")
        .as_bytes()
        .to_vec();
    for value in &g.v {
        bytes.extend_from_slice(&value.to_bits().to_le_bytes());
    }
    bytes.extend_from_slice(crate::c6_ref::rev().as_bytes());
    if let Some(map)=if g.p1(){crate::pheno::sfmap()}else{None} {
        for v in [map.x0,map.y0,map.g] {bytes.extend_from_slice(&v.to_bits().to_le_bytes());}
        for v in [map.nx,map.ny] {bytes.extend_from_slice(&(v as u64).to_le_bytes());}
        for v in &map.d {bytes.extend_from_slice(&v.to_bits().to_le_bytes());}
    }
    for source in [
        include_str!("aesthetic.rs"),
        include_str!("battery.rs")
            .split("\n#[cfg(test)]\n")
            .next()
            .expect("production source"),
        include_str!("c4_ref.rs"),
        include_str!("c5_ref.rs"),
        include_str!("c6_ref.rs"),
        include_str!("cad_ingest.rs"),
        include_str!("certification.rs"),
        include_str!("export.rs"),
        include_str!("step_export.rs"),
        include_str!("../examples/export_step.rs"),
        include_str!("fem.rs"),
        include_str!("g1.rs"),
        include_str!("gates.rs"),
        include_str!("gci.rs"),
        include_str!("genome.rs"),
        include_str!("geom.rs"),
        include_str!("lib.rs"),
        include_str!("pheno.rs"),
        include_str!("qd.rs"),
        include_str!("record.rs"),
        include_str!("../../physics/src/calib.rs"),
        include_str!("../../physics/src/fem.rs"),
        include_str!("../../physics/src/fine.rs"),
        include_str!("../../physics/src/lib.rs"),
        include_str!("../../physics/src/side_a4.rs"),
        include_str!("../../physics/src/solid_tet10.rs"),
        include_str!("../../physics/src/verify.rs"),
        include_str!("../../genotype/src/lib.rs"),
        include_str!("../../phenotype/src/lib.rs"),
        include_str!("../../phenotype/src/mesh.rs"),
        include_str!("../../redteam/src/lib.rs"),
        include_str!("../../cli/src/bench.rs")
            .split("\n#[cfg(test)]\n")
            .next()
            .expect("production source"),
        include_str!("../../cli/src/crescent.rs")
            .split("\n#[cfg(test)]\n")
            .next()
            .expect("production source"),
        include_str!("../../../Cargo.lock"),
        include_str!("../../../requirements/RS-karambit-v1.toml"),
        include_str!("../../../requirements/RS-karambit-v2.toml"),
        include_str!("../../../ops/ci/evaluate_cad_candidate.py"),
        include_str!("../../../ops/ci/verify_genome_export.py"),
        include_str!("../../../ops/ci/score_cad.py"),
        include_str!("../../../ops/ci/cad_battery.py"),
        include_str!("../../../fem_solid/convergence.py"),
        include_str!("../../../fem_solid/cross_check.py"),
        include_str!("../../../fem_solid/gci.py"),
        include_str!("../../../fem_solid/gci_report.py"),
        include_str!("../../../fem_solid/mesh.py"),
        include_str!("../../../fem_solid/run_tip.py"),
        include_str!("../../../fem_solid/solve_dolfinx.py"),
        include_str!("../../../fem_solid/step_mesh_smoke.py"),
        include_str!("../../../fem_solid/step_solid.py"),
        include_str!("../../../fem_solid/tip_solid.py"),
        include_str!("../../../fem_solid/export_surface.py"),
    ] {
        bytes.push(0);
        bytes.extend_from_slice(source.as_bytes());
    }
    jarambit_evidence::identity::ContentRef::from_bytes("certificate", "actual-export-v1", &bytes)
        .sha256
}

pub fn bound(doc: &serde_json::Value, key: &str) -> bool {
    doc.get("certificate_identity").and_then(|v| v.as_str()) == Some(key)
}

/// Vertical intersection with the exported triangles, not a query of the construction raster.
pub fn thickness(s: &Solid, q: P2) -> Option<f64> {
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for tr in &s.tris {
        let (a, b, c) = (s.verts[tr[0]], s.verts[tr[1]], s.verts[tr[2]]);
        let den = (b.1 - c.1) * (a.0 - c.0) + (c.0 - b.0) * (a.1 - c.1);
        if den.abs() < 1e-14 {
            continue;
        }
        let u = ((b.1 - c.1) * (q.0 - c.0) + (c.0 - b.0) * (q.1 - c.1)) / den;
        let v = ((c.1 - a.1) * (q.0 - c.0) + (a.0 - c.0) * (q.1 - c.1)) / den;
        // Geometric containment tolerance only; no tolerance is added to any requirement bound.
        if u >= -1e-10 && v >= -1e-10 && u + v <= 1.0 + 1e-10 {
            let z = c.2 + u * (a.2 - c.2) + v * (b.2 - c.2);
            lo = lo.min(z);
            hi = hi.max(z);
        }
    }
    (lo.is_finite() && hi.is_finite() && hi > lo).then_some(hi - lo)
}

/// All piecewise-linear surface breakpoints on a section, plus the required sample locations.
fn stations(s: &Solid, origin: P2, dir: P2, length: f64, required: &[f64]) -> Vec<f64> {
    let mut xs = vec![0.0, length];
    xs.extend_from_slice(required);
    for tr in &s.tris {
        for k in 0..3 {
            let a = s.verts[tr[k]];
            let b = s.verts[tr[(k + 1) % 3]];
            let e = (b.0 - a.0, b.1 - a.1);
            let d = (a.0 - origin.0, a.1 - origin.1);
            let det = dir.0 * e.1 - dir.1 * e.0;
            if det.abs() < 1e-14 {
                continue;
            }
            let x = (d.0 * e.1 - d.1 * e.0) / det;
            let t = (d.0 * dir.1 - d.1 * dir.0) / det;
            if x > 0.0 && x < length && (0.0..=1.0).contains(&t) {
                xs.push(x);
            }
        }
    }
    xs.sort_by(f64::total_cmp);
    xs.dedup_by(|a, b| (*a - *b).abs() < 1e-10);
    xs
}

#[derive(Clone, Debug, Serialize)]
pub struct PointProfile {
    pub pass: bool,
    pub t_0p25_mm: f64,
    pub t_1p00_mm: f64,
    pub t_2p00_mm: f64,
    pub monotonic: bool,
    pub taper_mm: Option<f64>,
    pub taper_lower_bound_mm: f64,
    pub samples: Vec<(f64, f64)>,
}
pub fn point_profile(s: &Solid, tip: P2, bisector: P2, core: f64) -> Option<PointProfile> {
    use crate::fem::gate_s::*;
    let sample = |a| thickness(s, add(tip, mul(bisector, a)));
    let (t025, t1, t2) = (sample(0.25)?, sample(1.0)?, sample(2.0)?);
    let xs = stations(s, tip, bisector, 10.0, &[0.25, 1.0, 2.0, 5.0]);
    let samples: Vec<_> = xs
        .into_iter()
        .map(|a| Some((a, sample(a)?)))
        .collect::<Option<_>>()?;
    let monotonic = samples.windows(2).all(|w| w[1].1 >= w[0].1);
    let level = 0.95 * core;
    let taper = if samples[0].1 >= level {
        Some(0.0)
    } else {
        samples
            .windows(2)
            .find(|w| w[0].1 < level && w[1].1 >= level)
            .map(|w| w[0].0 + (level - w[0].1) * (w[1].0 - w[0].0) / (w[1].1 - w[0].1))
    };
    // No crossing in the measured 10 mm means a measured lower bound, not an invented exact length.
    let lower = taper.unwrap_or(10.0);
    Some(PointProfile {
        pass: t025 <= TIP_T025_MAX_MM
            && t1 <= TIP_T1_MAX_MM
            && t2 <= TIP_T2_MAX_MM
            && monotonic
            && lower >= TIP_TAPER_MIN_MM,
        t_0p25_mm: t025,
        t_1p00_mm: t1,
        t_2p00_mm: t2,
        monotonic,
        taper_mm: taper,
        taper_lower_bound_mm: lower,
        samples,
    })
}

#[derive(Clone, Debug, Serialize)]
pub struct Sharpness {
    pub pass: bool,
    pub complete: bool,
    pub max_apex_mm: f64,
    pub max_half_angle_deg: f64,
    pub samples: usize,
}
pub fn sharpness(s: &Solid, p: &Pheno) -> Option<Sharpness> {
    use crate::fem::gate_s::*;
    let (mut apex, mut angle, mut count) = (0.0f64, 0.0f64, 0usize);
    let mut complete = true;
    for edge in [&p.edge_in, &p.edge_sp] {
        let index=Segments::new(edge);
        let distance=|q|index.nearest(q,0.5);
        let mut seen = std::collections::HashSet::new();
        let mut edge_count = 0;
        for &(x,y,_) in &s.verts {
            if !seen.insert((x.to_bits(),y.to_bits())) || distance((x,y)) > 0.031 { continue; }
            if let Some(t) = thickness(s,(x,y)) { apex=apex.max(t);edge_count+=1; }
            else { complete=false; }
        }
        if edge_count == 0 { return None; }
        count+=edge_count;
        // Exact slopes of the exported planar faces in the first 0.4 mm. This also measures
        // the acute endpoint, where a full 0.4 mm normal ray leaves the part. The gradient
        // magnitude bounds EVERY directional half-angle, including the edge-normal direction.
        for tr in &s.tris {
            let [a,b,c]=tr.map(|i|s.verts[i]);
            let u=(b.0-a.0,b.1-a.1,b.2-a.2);let v=(c.0-a.0,c.1-a.1,c.2-a.2);
            let nx=u.1*v.2-u.2*v.1;let ny=u.2*v.0-u.0*v.2;let nz=u.0*v.1-u.1*v.0;
            if nz.abs()<1e-14 { continue; } // vertical boundary walls are the measured apex, not a bevel face
            if !index.within_triangle((a.0,a.1),(b.0,b.1),(c.0,c.1),0.4){continue;}
            angle=angle.max((nx.hypot(ny)/nz.abs()).atan().to_degrees());
        }
    }
    Some(Sharpness { pass:complete && apex<=APEX_MAX_MM && angle<=HALF_ANGLE_MAX_DEG,
        complete,max_apex_mm:apex,max_half_angle_deg:angle,samples:count })
}

#[derive(Clone, Debug, Serialize)]
pub struct Measurements {
    pub source: &'static str,
    pub sharpness: Option<Sharpness>,
    pub point: Option<PointProfile>,
    pub bore_edge_radius_mm: Option<f64>,
}
/// Fit the circular rolls from actual exported vertices on both faces, around the entire bore.
/// Width, a gene or the presence of metadata can never prove this measurement. A sharp or
/// incomplete profile remains unverified; a known flat native bore still measures radius zero.
pub fn bore_radius(s: &Solid, p: &Pheno) -> Option<f64> {
    let r=p.ring_id/2.0;
    let mut profiles: std::collections::BTreeMap<i64,(Vec<P2>,Vec<P2>)>=std::collections::BTreeMap::new();
    for &(x,y,z) in &s.verts {
        let rho=x.hypot(y);
        if rho < r-1e-7 || rho > r+(p.ring_od-p.ring_id)/4.0 {continue;}
        let theta=y.atan2(x).rem_euclid(std::f64::consts::TAU);
        let row=profiles.entry((theta*1e8).round() as i64).or_default();
        if z>0.0 {row.0.push((rho,z));} else if z<0.0 {row.1.push((rho,-z));}
    }
    let mut sectors=[false;72];let mut minimum=f64::INFINITY;
    for (theta,(mut top,mut bottom)) in profiles {
        for points in [&mut top,&mut bottom] {
            points.sort_by(|a,b|a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
            points.dedup_by(|a,b|dist(*a,*b)<1e-8);
        }
        if top.first().map_or(true,|v|(v.0-r).abs()>1e-7)
            && bottom.first().map_or(true,|v|(v.0-r).abs()>1e-7) {continue;}
        // The native export supplies 16 meridional facets per quarter-circle on each face.
        // Check every one; do not infer a radius from one convenient three-point subset.
        for points in [&top,&bottom] {
            if points.len()<17 || (points[0].0-r).abs()>1e-7 {return None;}
            let (a,b,c)=(points[0],points[8],points[16]);
            let den=2.0*(a.0*(b.1-c.1)+b.0*(c.1-a.1)+c.0*(a.1-b.1));
            if den.abs()<1e-12 {return Some(0.0);}
            let aa=a.0*a.0+a.1*a.1;let bb=b.0*b.0+b.1*b.1;let cc=c.0*c.0+c.1*c.1;
            let center=((aa*(b.1-c.1)+bb*(c.1-a.1)+cc*(a.1-b.1))/den,
                        (aa*(c.0-b.0)+bb*(a.0-c.0)+cc*(b.0-a.0))/den);
            let radius=dist(a,center);
            if !radius.is_finite() || (center.0-a.0-radius).abs()>1e-7
                || (center.1-a.1).abs()>1e-7 || (c.0-center.0).abs()>0.03
                || points[..17].iter().any(|&v|(dist(v,center)-radius).abs()>1e-7) {return None;}
            minimum=minimum.min(radius);
        }
        sectors[(((theta as f64/1e8)/std::f64::consts::TAU*72.0) as usize).min(71)]=true;
    }
    if sectors.iter().all(|v|*v) && minimum.is_finite() {return Some(minimum);}
    // Legacy planar/unfilleted geometry can still establish a sharp corner.
    if !p.rs_v2 {
        for k in 0..72 {
            let a=k as f64*std::f64::consts::TAU/72.0;let n=(a.cos(),a.sin());
            if thickness(s,mul(n,r+0.05))?<=thickness(s,mul(n,r))? {return Some(0.0);}
        }
    }
    None
}

pub fn measure(p: &Pheno) -> Measurements {
    let mut m = Measurements {
        source: "export::solid(max_area=1.0); triangle intersections",
        sharpness: None,
        point: None,
        bore_edge_radius_mm: None,
    };
    let Some(s) = export::solid(p, EXPORT_AREA_MM2) else {
        return m;
    };
    if export::census(&s.tris) != (0, 0, 1) || export::volume(&s) <= 0.0 {
        return m;
    }
    let qi = p
        .edge_in
        .iter()
        .copied()
        .find(|&q| dist(q, p.tip) >= 2.0)
        .unwrap_or(p.heel);
    let qs = p
        .edge_sp
        .iter()
        .rev()
        .copied()
        .find(|&q| dist(q, p.tip) >= 2.0)
        .unwrap_or(p.heel);
    let u = norm(add(norm(sub(qi, p.tip)), norm(sub(qs, p.tip))));
    m.point = point_profile(&s, p.tip, u, p.t_joint);
    m.sharpness = sharpness(&s, p);
    m.bore_edge_radius_mm = bore_radius(&s, p);
    m
}

/// A primary native solve is not an independent comparison. Compute errors from distinct solver observations.
pub fn independent_pass(doc: &serde_json::Value) -> Option<bool> {
    let dx = doc.get("dolfinx_cross_check")?;
    if dx["status"].as_str() != Some("verified")
        || dx["primary_solver"].as_str() != Some("skfem_p2")
        || dx["independent_solver"].as_str() != Some("dolfinx_v0.9.0")
    {
        return None;
    }
    let mesh = dx["mesh_sha256"].as_str()?;
    if mesh.len() != 64 || !mesh.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    if doc["mesh_sha256"].as_str() != Some(mesh) {
        return None;
    }
    let stored = doc.get("stored_solid")?;
    if stored["NL"].as_u64() != Some(4) || stored["NSUB"].as_u64() != Some(1) {
        return None;
    }
    let mut ok = true;
    for (primary, other) in [
        ("SF_skfem_nodal_max", "SF_nodal_max"),
        ("SF_skfem_nodal_avg", "SF_nodal_avg"),
    ] {
        let a = stored[primary].as_f64()?;
        let b = dx[other].as_f64()?;
        if !a.is_finite() || !b.is_finite() || a <= 0.0 || b <= 0.0 {
            return None;
        }
        if stored[other].as_f64() != Some(b) {
            return None;
        }
        ok &= (a - b).abs() / b <= 0.02;
    }
    Some(ok)
}
