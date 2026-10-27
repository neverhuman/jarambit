//! Release tooling for g5-evolve runs (v3.0.0): re-evaluation, export and grid-convergence of archived designs.
//!
//!   cargo run --release --bin g5-pilot -- eval   <run_root> <out.jsonl> [h=0.4] [threads=4]
//!   cargo run --release --bin g5-pilot -- export <run_root> <design_id> <out_dir> [h=0.2]
//!   cargo run --release --bin g5-pilot -- hconv  <run_root> <id,id,...> <h,h,...> <out.jsonl> [threads=4]
//!
//! eval: every archived elite of every island (`<run_root>/island*/run/genomes_g5.csv`, rows are
//! `design_id,` + the strict G5.0 row, whose genome sha256 is re-verified on load) is re-evaluated at
//! Tier 1 (voxel SDF + plate) and written as one JSON line: island, design_id and the full record
//! (tier, inputs, score, descriptors). The archive itself stores no per-niche scores, hence this.
//! export: the envelope and the AM build of one design as watertight STL (fail-closed census:
//! no boundary or non-manifold edges), with volumes, at grid h.
//! hconv: grid-convergence check: the named designs re-evaluated at Tier 1 at each grid h.
//! tier2: the top N designs of an `eval` file re-evaluated at Tier 2 (adds the measured watertight census and
//!        the 3D solid-tip FE), so designs are compared on measured, not zero-credit, terms.
//!   cargo run --release --bin g5-pilot -- tier2 <run_root> <eval.jsonl> <top_n> <out.jsonl> [h=0.4] [threads=4]
use jarambit_crescent::export::{census, stl, volume};
use jarambit_g5::g5_genome::G5Genome;
use jarambit_g5::g5_qd::{evaluate_tier1_sdf_and_plate, evaluate_tier2_elites};
use jarambit_g5::g5_sdf::extract_watertight_mesh_fail_closed;
use rayon::prelude::*;
use std::{fs, io::Write, path::Path};

fn rows(run_root: &Path) -> Vec<(String, String, G5Genome)> {
    let mut out = Vec::new();
    let mut islands: Vec<_> = fs::read_dir(run_root).expect("run_root").flatten()
        .filter(|e| e.file_name().to_string_lossy().starts_with("island")).collect();
    islands.sort_by_key(|e| e.file_name());
    for isl in islands {
        let csv = isl.path().join("run/genomes_g5.csv");
        let Ok(text) = fs::read_to_string(&csv) else { continue };
        for line in text.lines().skip(1).filter(|l| !l.trim().is_empty()) {
            let (id, rest) = line.split_once(',').expect("design_id column");
            match G5Genome::from_csv_row(rest) {
                Ok(g) => out.push((isl.file_name().to_string_lossy().into_owned(), id.to_string(), g)),
                Err(e) => eprintln!("skip {id}: {e}"),
            }
        }
    }
    out
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("eval") => {
            let root = Path::new(&a[2]);
            let h: f64 = a.get(4).and_then(|v| v.parse().ok()).unwrap_or(0.4);
            let threads: usize = a.get(5).and_then(|v| v.parse().ok()).unwrap_or(4);
            rayon::ThreadPoolBuilder::new().num_threads(threads).build_global().ok();
            let rs = rows(root);
            eprintln!("evaluating {} elites at h = {h} on {threads} threads", rs.len());
            let lines: Vec<String> = rs.par_iter().map(|(isl, id, g)| {
                let rec = evaluate_tier1_sdf_and_plate(g, h);
                serde_json::json!({"island": isl, "design_id": id, "h": h, "record": rec}).to_string()
            }).collect();
            let mut f = fs::File::create(&a[3]).expect("out");
            for l in lines { writeln!(f, "{l}").unwrap(); }
            eprintln!("wrote {}", a[3]);
        }
        Some("export") => {
            let root = Path::new(&a[2]);
            let id = &a[3];
            let out = Path::new(&a[4]);
            let h: f64 = a.get(5).and_then(|v| v.parse().ok()).unwrap_or(0.2);
            fs::create_dir_all(out).unwrap();
            let (_, _, g) = rows(root).into_iter().find(|(_, d, _)| d == id).expect("design_id not found");
            // decoded parameters (mm), for the render scene: ring centre, grip start and length, swell station
            fs::write(out.join(format!("{id}_params.json")), serde_json::to_string_pretty(&g.decode()).unwrap()).unwrap();
            for (am, tag) in [(false, "envelope"), (true, "AMbuild")] {
                match extract_watertight_mesh_fail_closed(&g, h, am) {
                    Some(s) => {
                        let (boundary, non_manifold, dmax) = census(&s.tris);
                        let p = out.join(format!("{id}_{tag}.stl"));
                        fs::write(&p, stl(&s, &format!("{id}_{tag}"))).unwrap();
                        println!("{tag}: {} triangles, volume {:.1} mm3, boundary {boundary}, non-manifold {non_manifold}, max degree {dmax} -> {}",
                                 s.tris.len(), volume(&s), p.display());
                    }
                    None => println!("{tag}: not watertight after 4 grid shifts; nothing written (fail closed)"),
                }
            }
        }
        Some("hconv") => {
            let root = Path::new(&a[2]);
            let ids: Vec<&str> = a[3].split(',').collect();
            let hs: Vec<f64> = a[4].split(',').map(|v| v.parse().expect("h")).collect();
            let threads: usize = a.get(6).and_then(|v| v.parse().ok()).unwrap_or(4);
            rayon::ThreadPoolBuilder::new().num_threads(threads).build_global().ok();
            let all = rows(root);
            let jobs: Vec<(String, f64, &G5Genome)> = ids.iter().flat_map(|id| {
                let g = &all.iter().find(|(_, d, _)| d == id).expect("design_id not found").2;
                hs.iter().map(move |&h| (id.to_string(), h, g))
            }).collect();
            let lines: Vec<String> = jobs.par_iter().map(|(id, h, g)| {
                let rec = evaluate_tier1_sdf_and_plate(g, *h);
                serde_json::json!({"design_id": id, "h": h, "record": rec}).to_string()
            }).collect();
            let mut f = fs::File::create(&a[5]).expect("out");
            for l in lines { writeln!(f, "{l}").unwrap(); }
            eprintln!("wrote {}", a[5]);
        }
        Some("tier2") => {
            let root = Path::new(&a[2]);
            let n: usize = a[4].parse().expect("top_n");
            let h: f64 = a.get(6).and_then(|v| v.parse().ok()).unwrap_or(0.4);
            let threads: usize = a.get(7).and_then(|v| v.parse().ok()).unwrap_or(4);
            rayon::ThreadPoolBuilder::new().num_threads(threads).build_global().ok();
            let mut ev: Vec<(f64, String)> = fs::read_to_string(&a[3]).expect("eval.jsonl").lines().filter(|l| !l.trim().is_empty())
                .map(|l| { let v: serde_json::Value = serde_json::from_str(l).unwrap();
                           (v["record"]["score"]["score_100"].as_f64().unwrap(), v["design_id"].as_str().unwrap().to_string()) })
                .collect();
            ev.sort_by(|x, y| y.0.total_cmp(&x.0));
            let all = rows(root);
            let picks: Vec<(String, &G5Genome)> = ev.iter().take(n)
                .map(|(_, id)| (id.clone(), &all.iter().find(|(_, d, _)| d == id).expect("design_id").2)).collect();
            let lines: Vec<String> = picks.par_iter().map(|(id, g)| {
                let rec = evaluate_tier2_elites(g, h);
                serde_json::json!({"design_id": id, "h": h, "record": rec}).to_string()
            }).collect();
            let mut f = fs::File::create(&a[5]).expect("out");
            for l in lines { writeln!(f, "{l}").unwrap(); }
            eprintln!("wrote {}", a[5]);
        }
        _ => eprintln!("usage: pilot_tools eval <run_root> <out.jsonl> [h] [threads] | export <run_root> <design_id> <out_dir> [h]"),
    }
}
