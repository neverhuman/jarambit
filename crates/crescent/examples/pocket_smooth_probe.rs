//! RESEARCH probe for the default-off pocket smoothing (`pocket_smooth`). Geometry only: nothing is
//! scored. Builds the UNSMOOTHED phenotype once, runs the flow for each parameter set and writes one
//! JSON report: flow metrics, removed volume and its mass equivalent, isoline lengths, through-hole
//! positions (for picking render windows) and, for each `--window`, 8-bit PGM depth maps.
//! Usage:
//!   pocket_smooth_probe <genome.csv> <NEW_out_dir> [--window NAME=x0,y0,x1,y1]... PARAMS...
//! PARAMS use the JARAMBIT_POCKET_SMOOTH syntax, e.g. r=1.0,h=0.125,grow=0.5,mesh=0.3,rim=1.0,vp=1
use jarambit_crescent::{genome::Genome, pheno, pocket_smooth};
use std::path::Path;

fn pgm(path: &Path, w: usize, h: usize, px: &[u8]) {
    let mut out = format!("P5\n{w} {h}\n255\n").into_bytes();
    out.extend_from_slice(px);
    std::fs::write(path, out).expect("write pgm");
}

fn main() {
    // The probe passes parameters explicitly; the environment switch must not smooth the base.
    std::env::remove_var(pocket_smooth::ENV);
    let a: Vec<String> = std::env::args().collect();
    assert!(a.len() >= 4, "usage: pocket_smooth_probe <genome.csv> <NEW_out_dir> [--window NAME=x0,y0,x1,y1]... PARAMS...");
    let txt = std::fs::read_to_string(&a[1]).expect("read genome");
    let mut l = txt.lines();
    let h: Vec<&str> = l.next().unwrap().split(',').collect();
    let off = h.iter().position(|c| *c == "cp00").expect("cp00");
    let f: Vec<&str> = l.next().unwrap().split(',').collect();
    let g = Genome::from_csv(&f[off..]).expect("genome parse");
    let out = Path::new(&a[2]);
    assert!(!out.exists(), "output directory must be new");
    std::fs::create_dir_all(out).expect("mkdir");
    let mut windows: Vec<(String, [f64; 4])> = Vec::new();
    let mut sets: Vec<String> = Vec::new();
    let mut i = 3;
    while i < a.len() {
        if a[i] == "--window" {
            let (name, rest) = a[i + 1].split_once('=').expect("--window NAME=x0,y0,x1,y1");
            let v: Vec<f64> = rest.split(',').map(|x| x.parse().expect("window number")).collect();
            assert_eq!(v.len(), 4, "window needs 4 numbers");
            windows.push((name.to_string(), [v[0], v[1], v[2], v[3]]));
            i += 2;
        } else {
            sets.push(a[i].clone());
            i += 1;
        }
    }
    let p = pheno::build(&g);
    assert!(p.pocket_smooth.is_none());
    assert!(p.rs_v2, "pocket smoothing is defined for rs_v2 genomes only");
    let holes: Vec<_> = p
        .holes
        .iter()
        .zip(&p.hole_names)
        .map(|(hole, name)| {
            let (mut lo, mut hi) = ((f64::MAX, f64::MAX), (f64::MIN, f64::MIN));
            let mut c = (0.0, 0.0);
            for q in hole {
                lo = (lo.0.min(q.0), lo.1.min(q.1));
                hi = (hi.0.max(q.0), hi.1.max(q.1));
                c = (c.0 + q.0 / hole.len() as f64, c.1 + q.1 / hole.len() as f64);
            }
            serde_json::json!({"name": name, "centroid": [c.0, c.1], "bbox": [lo.0, lo.1, hi.0, hi.1]})
        })
        .collect();
    let rho_g_per_mm3 = jarambit_physics::material(p.mat).density / 1000.0;
    let mut runs = Vec::new();
    for s in &sets {
        let prm = pocket_smooth::Params::parse(s).expect("params");
        let sf = pocket_smooth::build(&p, &prm);
        let rep = sf.report.clone();
        // Volume check through the exporter's own thickness law: coarse raster cells and the fine grid.
        let mut ps = p.clone();
        let sfa = std::sync::Arc::new(sf);
        ps.pocket_smooth = Some(sfa.clone());
        let fld = &p.field;
        let (mut v0, mut v1) = (0.0f64, 0.0f64);
        for j in 0..fld.ny {
            for i in 0..fld.nx {
                if fld.th[j * fld.nx + i].is_nan() { continue; }
                let q = (fld.x0 + (i as f64 + 0.5) * fld.g, fld.y0 + (j as f64 + 0.5) * fld.g);
                v0 += jarambit_crescent::export::thickness_at(&p, q) * fld.g * fld.g;
                v1 += jarambit_crescent::export::thickness_at(&ps, q) * fld.g * fld.g;
            }
        }
        let (mut f0, mut f1) = (0.0f64, 0.0f64);
        let mut bd = [0.0f64; 7];
        let mut dump = std::env::var("POCKET_PROBE_DUMP").ok().map(|p| std::fs::File::create(p).expect("dump"));
        let g = sfa.grid;
        for j in 0..g.ny {
            for i in 0..g.nx {
                let s = j * g.nx + i;
                if sfa.d0[s].is_nan() { continue; }
                let q = g.center(i, j);
                let (t0, t1) = (jarambit_crescent::export::thickness_at(&p, q), jarambit_crescent::export::thickness_at(&ps, q));
                f0 += t0 * g.h * g.h;
                f1 += t1 * g.h * g.h;
                let a = g.h * g.h;
                match sfa.depth_at(q) {
                    Some(dd) => {
                        let mis = (t1 + dd) - sfa.d0[s] as f64 - t0;
                        bd[0] += (dd - sfa.d0[s] as f64) * a;
                        bd[1] += mis * a;
                        if mis < -0.01 {
                            bd[2] += 1.0;
                            bd[3] += mis * a;
                            if let Some(f) = dump.as_mut() {
                                use std::io::Write;
                                let _ = writeln!(f, "{:.4},{:.4},{:.4},{:.4},{:.4},{:.4}", q.0, q.1, t0, t1 + dd, sfa.d0[s], dd);
                            }
                        }
                        if mis > 0.01 { bd[4] += 1.0; bd[5] += mis * a; }
                    }
                    None => bd[6] += (t1 - t0) * a,
                }
            }
        }
        eprintln!("thickness-law volume coarse {v0:.3} -> {v1:.3} ({:+.3}); fine {f0:.3} -> {f1:.3} ({:+.3})", v1 - v0, f1 - f0);
        eprintln!("  active: flow dD {:+.3} mm3; law mismatch {:+.3} (neg {} cells {:+.3}, pos {} cells {:+.3}); inactive diff {:+.3}", bd[0], bd[1], bd[2], bd[3], bd[4], bd[5], bd[6]);
        drop(ps);
        let sf = std::sync::Arc::try_unwrap(sfa).ok().expect("sole owner");
        let dv = rep["removed_volume_mm3"]["after"].as_f64().unwrap() - rep["removed_volume_mm3"]["before"].as_f64().unwrap();
        let tag = prm.key().replace([',', '='], "_");
        let pbm = pocket_smooth::problem(&p, &prm);
        for (name, w) in &windows {
            let g = sf.grid;
            let i0 = (((w[0] - g.x0) / g.h).floor().max(0.0) as usize).min(g.nx - 1);
            let i1 = (((w[2] - g.x0) / g.h).ceil().max(0.0) as usize).min(g.nx);
            let j0 = (((w[1] - g.y0) / g.h).floor().max(0.0) as usize).min(g.ny - 1);
            let j1 = (((w[3] - g.y0) / g.h).ceil().max(0.0) as usize).min(g.ny);
            let (ww, hh) = (i1 - i0, j1 - j0);
            let mut dmax = 0.0f32;
            for j in j0..j1 {
                for i in i0..i1 {
                    let v = sf.d0[j * g.nx + i];
                    if v.is_finite() {
                        dmax = dmax.max(v);
                    }
                }
            }
            {
                // state map: 255 outside, 170 fixed, 110 free but not evolving, 30 evolving, 0 fixed with depth
                let mut px = Vec::with_capacity(ww * hh);
                for jj in 0..hh {
                    let j = j1 - 1 - jj;
                    for i in i0..i1 {
                        let t = j * g.nx + i;
                        px.push(match pbm.state[t] {
                            pocket_smooth::OUTSIDE => 255,
                            pocket_smooth::FIXED => if pbm.d0[t] > 0.0 { 0 } else { 170 },
                            _ => if sf.active[t] { 30 } else { 110 },
                        });
                    }
                }
                pgm(&out.join(format!("{name}-state-{tag}.pgm")), ww, hh, &px);
            }
            for (field, label) in [(&sf.d0, "before"), (&sf.d, "after")] {
                let mut px = Vec::with_capacity(ww * hh);
                for jj in 0..hh {
                    let j = j1 - 1 - jj; // +y up in the image
                    for i in i0..i1 {
                        let v = field[j * g.nx + i];
                        px.push(if v.is_nan() { 255 } else { (230.0 * (1.0 - v / dmax.max(1e-6))).round().clamp(0.0, 230.0) as u8 });
                    }
                }
                pgm(&out.join(format!("{name}-{label}-{tag}.pgm")), ww, hh, &px);
            }
        }
        runs.push(serde_json::json!({
            "params": s,
            "report": rep,
            "removed_volume_change_mm3": dv,
            "mass_change_g": -dv * rho_g_per_mm3,
        }));
    }
    let doc = serde_json::json!({
        "schema": "jarambit.pocket_smooth_probe.v1",
        "research_only": true,
        "genome": a[1],
        "label_mass_raster_g": p.mass_g,
        "density_g_per_mm3": rho_g_per_mm3,
        "holes": holes,
        "windows": windows.iter().map(|(n, w)| serde_json::json!({"name": n, "window": w})).collect::<Vec<_>>(),
        "runs": runs,
    });
    std::fs::write(out.join("probe.json"), serde_json::to_string_pretty(&doc).unwrap() + "\n").expect("write probe.json");
    println!("wrote {}", out.join("probe.json").display());
}
