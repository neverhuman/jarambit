#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc; // per-thread heaps: glibc malloc spent ~75% in kernel at >8 MC threads

use jarambit_evolve::{format_cut_for_current_log, format_stats_mean_cell, format_stats_sharp_cell, history_header_status, nominal, Config, Indiv, SharpFormat, CURRENT_HISTORY_COLUMNS};
use jarambit_genotype::{limits, Genotype};
use jarambit_physics::{material, plate_capacities};
use std::fmt::Write;
mod fem_smoke;
mod calib;
mod crescent;
mod bench;
mod score_cad;
mod auto_evolve;
mod datasets;
mod version_check;
mod runner_policy;
mod smoothing_ladder;

/// Validate an existing history header before any append, then open the log.
/// A fresh path is created with `fresh_header` and only then opened for append.
fn prepare_history_log(path: &str, resume_or_fork: bool, current_columns: &[&str], fresh_header: &str) -> Result<std::fs::File, String> {
    let exists = std::path::Path::new(path).exists();
    if resume_or_fork && exists {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
        let header = text.lines().next().unwrap_or("");
        let existing: Vec<&str> = header.split(',').collect();
        history_header_status(&existing, current_columns)?;
    } else {
        std::fs::write(path, fresh_header).map_err(|e| format!("{path}: {e}"))?;
    }
    std::fs::OpenOptions::new().append(true).open(path).map_err(|e| format!("{path}: {e}"))
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.iter().any(|s| s == "version-check") {
        let root = std::env::current_dir().unwrap_or_else(|e| {
            eprintln!("current directory: {e}");
            std::process::exit(1);
        });
        let reference = std::env::var("GITHUB_REF").unwrap_or_default();
        std::process::exit(version_check::run(&root, &reference));
    }
    if a.iter().any(|s| s == "check-runner-policy") {
        let root = std::env::current_dir().unwrap_or_else(|e| {
            eprintln!("current directory: {e}");
            std::process::exit(1);
        });
        std::process::exit(runner_policy::run(&root));
    }
    if a.iter().any(|s| s == "fem-smoke") {
        let out = a.iter().position(|x| x == "--out").and_then(|i| a.get(i + 1)).cloned().unwrap_or_else(|| "out/fem_smoke".into());
        let mh1 = a.iter().position(|x| x == "--mh1").and_then(|i| a.get(i + 1)).cloned().unwrap_or_else(|| "out/mh1".into());
        let ok = fem_smoke::run(&out, &mh1);
        std::process::exit(if ok { 0 } else { 1 });
    }
    let sarg = |k: &str, d: &str| a.iter().position(|x| x == k).and_then(|i| a.get(i + 1)).cloned().unwrap_or_else(|| d.to_string());
    let narg = |k: &str, d: usize| a.iter().position(|x| x == k).and_then(|i| a.get(i + 1)).and_then(|v| v.parse().ok()).unwrap_or(d);
    let farg = |k: &str, d: f64| a.iter().position(|x| x == k).and_then(|i| a.get(i + 1)).and_then(|v| v.parse::<f64>().ok()).unwrap_or(d);
    if let Some(pos) = a.iter().position(|s| s == "score-cad") {
        std::process::exit(score_cad::run_score_cad_cli(&a[(pos + 1)..]));
    }
    if let Some(pos) = a.iter().position(|s| s == "cad-battery") {
        std::process::exit(score_cad::run_cad_battery_cli(&a[(pos + 1)..]));
    }
    if let Some(pos) = a.iter().position(|s| s == "export-web-data") {
        std::process::exit(score_cad::run_export_web_data_cli(&a[(pos + 1)..]));
    }
    if a.get(1).map(String::as_str) == Some("dataset") {
        // regression dataset factory: build / cv / harvest-waves / commit / featurize / rescreen / ...
        std::process::exit(datasets::run_dataset_cli(&a[2..]));
    }
    if let Some(pos) = a.iter().position(|s| s == "smoothing-ladder") {
        std::process::exit(smoothing_ladder::run_smoothing_ladder_cli(&a[(pos + 1)..]));
    }
    if let Some(pos) = a.iter().position(|s| s == "auto-evolve") {
        std::process::exit(auto_evolve::run_auto_evolve_cli(&a[(pos + 1)..]));
    }
    if a.iter().any(|s| s == "bench-make") { std::process::exit(if bench::make(&sarg("--seeds", ""), narg("--n", 1000), narg("--seed", 31) as u64, &sarg("--out", "bench/fixed_1k/genomes.csv")) { 0 } else { 1 }); }
    if a.iter().any(|s| s == "rs-v2-eval") { std::process::exit(if bench::rs_v2_eval(&sarg("--genomes", ""), &sarg("--out", "evidence/rs_v2"), &sarg("--solid-dir", "evidence/fem_solid/gci")) { 0 } else { 1 }); }
    if a.iter().any(|s| s == "calib-t1t2") { std::process::exit(if bench::calib_t1t2(&sarg("--genomes", "bench/fixed_1k/genomes.csv"), narg("--n", 40), narg("--stride", 25), narg("--threads", 1).min(jarambit_monte_carlo::max_threads()), farg("--area", 2.0), &sarg("--out", "docs/calibration/t1_t2.csv")) { 0 } else { 1 }); }
    if a.iter().any(|s| s == "bench-fixed") { std::process::exit(if bench::run(&sarg("--genomes", "bench/fixed_1k/genomes.csv"), &sarg("--tier", "search"), narg("--n", 1000), narg("--threads", 1).min(jarambit_monte_carlo::max_threads()), farg("--area", 2.0)) { 0 } else { 1 }); }
    if a.iter().any(|s| s == "crescent-probe") { crescent::probe(farg("--area", 4.0)); return; }
    if a.iter().any(|s| s == "crescent-conv") {
        let ok = crescent::conv(&sarg("--evals", "out/crescent_smoke/all_evals.csv"), narg("--n", 24), narg("--threads", 24).min(jarambit_monte_carlo::max_threads()), &sarg("--out", "out/crescent_conv"));
        std::process::exit(if ok { 0 } else { 1 });
    }
    if a.iter().any(|s| s == "crescent-rescore") { let ok = crescent::rescore(&sarg("--genomes", ""), narg("--top", 12), narg("--threads", 24).min(jarambit_monte_carlo::max_threads()), &sarg("--out", "out/rescore_c5")); std::process::exit(if ok { 0 } else { 1 }); }
    if a.iter().any(|s| s == "crescent-d1") { let ok = crescent::d1(&sarg("--genomes", ""), narg("--threads", 8).min(jarambit_monte_carlo::max_threads()), &sarg("--out", "out/d1")); std::process::exit(if ok { 0 } else { 1 }); }
    if a.iter().any(|s| s == "crescent-refine") { let ok = crescent::refine(&sarg("--genomes", ""), narg("--threads", 24).min(jarambit_monte_carlo::max_threads()), &sarg("--out", "out/refine")); std::process::exit(if ok { 0 } else { 1 }); }
    if a.iter().any(|s| s == "crescent-mprobe") { crescent::mprobe(&sarg("--genome", "docs/crescent/best_genome.csv"), farg("--area", 2.0)); return; }
    if a.iter().any(|s| s == "crescent-evolve") {
        // internal-machining lane: island GA + QD, seeds from certified crescents + C.4 (crates/crescent)
        let ok = crescent::evolve(&sarg("--out", "out/wave7_lattice"), &sarg("--wave", "wave7"), narg("--pop", 48), narg("--islands", 4), narg("--gens", 40),
            narg("--threads", 48).min(jarambit_monte_carlo::max_threads()), narg("--seed", 7) as u64, farg("--area", 2.0),
            &sarg("--seeds", "docs/crescent/best_genome.csv"), narg("--seed-top", 12), narg("--migrate", 5), farg("--kick", 0.5), narg("--cert", 6));
        std::process::exit(if ok { 0 } else { 1 });
    }
    if a.iter().any(|s| s == "crescent-smoke") {
        // SWITCHBACK C.4-anchored crescent genotype: aesthetic floor + novelty QD (crates/crescent)
        let ok = crescent::smoke(&sarg("--out", "out/crescent_smoke"), narg("--pop", 48), narg("--gens", 16), narg("--threads", 32).min(jarambit_monte_carlo::max_threads()),
            narg("--seed", 7) as u64, farg("--t", 5.7), farg("--area", 4.0), farg("--plate-area", 1.0));
        std::process::exit(if ok { 0 } else { 1 });
    }
    if a.iter().any(|s| s == "evidence-cat") {
        // PR-0.29.1: dump an evidence log (*.jevl) as JSON Lines; a torn tail (crash) is reported on stderr
        let path = sarg("--log", "out/evidence.jevl");
        match jarambit_evidence::read_log(&path) {
            Ok(r) => { for e in &r.events { println!("{}", serde_json::to_string(e).unwrap()); } eprintln!("{path}: {} frames, {} events{}", r.frames, r.events.len(), if r.torn_tail { ", TORN TAIL dropped" } else { "" }); std::process::exit(0); }
            Err(e) => { eprintln!("{e}"); std::process::exit(1); }
        }
    }
    if a.iter().any(|s| s == "fem-verify") {
        let ok = calib::fem_verify(&sarg("--out", "out/fem_verify"), &sarg("--mh1", "out/mh1"), narg("--threads", 16).min(jarambit_monte_carlo::max_threads()));
        std::process::exit(if ok { 0 } else { 1 });
    }
    if a.iter().any(|s| s == "diag") {
        // diag --ckpt PATH [--ckpt PATH..]: detailed-plate (verify mesh) capacities + peak location of each checkpoint best
        for (i, x) in a.iter().enumerate() {
            if x != "--ckpt" { continue; }
            let Some(pth) = a.get(i + 1) else { continue };
            let g = match jarambit_evolve::checkpoint_best(pth) { Ok(g) => g, Err(e) => { eprintln!("{pth}: {e}"); continue } };
            let (p, rt, e) = nominal(&g);
            let m = material(g.material);
            match jarambit_physics::fine::capacities(&p, m, 20.0, (g.serr_inner.depth, g.serr_outer.depth), jarambit_physics::fine::VERIFY_AREA) {
                Some(c) => {
                    let h = jarambit_physics::fine::at_temp(&c, m, 20.0, 100.0);
                    println!("{pth}: mass {:.1} g t {:.1} {} lat20 {:.0} N lat100 {:.0} N ring100 {:.0} N peak {} ({:.1},{:.1}) ring_at ({:.1},{:.1}) rt_ok {} | grip {:.1} ric {:.1} sh {:.2} angle {:.1} R {:.1} arc {:.1} wall {:.1} flare {:.2}/{:.1}/{:.1} blade {:.1}/{:.1}/{:.1}",
                        e.mass_g, g.thickness, g.material.name(), c.lateral_n, h.lateral_n, h.ring_pull_n, c.lateral_region, c.lateral_at.0, c.lateral_at.1, c.ring_at.0, c.ring_at.1, rt.is_ok(),
                        g.grip_len, g.holder.ricasso_len, g.holder.shoulder_w, g.blade_angle_deg, g.curve_radius, g.blade_arc_len, g.ring_wall, g.neck_flare, g.neck_flare_len, g.neck_fillet_r, p.blade.tip_to_heel, p.blade.horizontal_reach, p.blade.sharpened_arc);
                }
                None => println!("{pth}: mesh fail"),
            }
        }
        return;
    }
    if a.iter().any(|s| s == "calib") {
        calib::calib(&sarg("--out", "out/calib"), &sarg("--mh1", "out/mh1"), narg("--threads", 16).min(jarambit_monte_carlo::max_threads()), narg("--n", 120));
        return;
    }
    let arg = |k: &str, d: usize| a.iter().position(|x| x == k).and_then(|i| a.get(i + 1)).and_then(|v| v.parse().ok()).unwrap_or(d);
    let threads = arg("--threads", 16).min(jarambit_monte_carlo::max_threads()); // per-host cap: JARAMBIT_MAX_THREADS (default 48)
    let out = a.iter().position(|x| x == "--out").and_then(|i| a.get(i + 1)).cloned().unwrap_or("out".into());
    std::fs::create_dir_all(&out).unwrap();
    let mut cfg = Config::new(arg("--pop", 32), arg("--gens", 5), arg("--mc", 256), threads, arg("--seed", 42) as u64);
    cfg.islands = arg("--islands", 1).max(1);
    cfg.migrate_every = arg("--migrate", 5);
    cfg.checkpoint = Some(format!("{out}/checkpoint.txt"));
    cfg.resume = a.iter().any(|x| x == "--resume");
    cfg.fork = a.iter().any(|x| x == "--fork"); // PR-0.29.2: resume across a manifest change, deliberately (recorded in the checkpoint manifest)
    cfg.fem = std::env::var("JARAMBIT_FEM").ok().as_deref() == Some("1");
    cfg.fine = a.iter().any(|x| x == "--fine") || std::env::var("JARAMBIT_FINE").ok().as_deref() == Some("1");
    // --seed-ckpt PATH (repeatable): best genome of each checkpoint seeds the initial islands
    for (i, x) in a.iter().enumerate() {
        if x == "--seed-ckpt" { if let Some(pth) = a.get(i + 1) { match jarambit_evolve::checkpoint_best(pth) { Ok(g) => cfg.seeds.push(g), Err(e) => eprintln!("seed {pth}: {e}") } } }
    }
    if a.iter().any(|x| x == "--seed-ref") {
        // SWITCHBACK-class reference genome from the red-team suite (all-three blade floor, every gate)
        let mut g = Genotype::random(&mut jarambit_genotype::Rng(9));
        g.blade_arc_len = 87.55; g.curve_radius = 90.0; g.blade_angle_deg = 13.0; g.ring_wall = 4.5;
        g.thickness = 4.5; g.neck_flare = 4.0; g.neck_flare_len = 20.0; g.neck_fillet_r = 8.0; g.clamp();
        cfg.seeds.push(g);
    }
    // --seed-thick T (repeatable): also seed a copy of every seed at stock T mm (clamped to the
    // process ceiling, see JARAMBIT_T_MAX), so a thicker-stock search starts next to known designs.
    let thick: Vec<f64> = a.iter().enumerate().filter(|(_, x)| *x == "--seed-thick").filter_map(|(i, _)| a.get(i + 1)?.parse().ok()).collect();
    let base = cfg.seeds.clone();
    for t in thick { for g in &base { let mut h = g.clone(); h.thickness = t; h.clamp(); cfg.seeds.push(h); } }
    let hist_path = format!("{out}/history.csv");
    let q_path = format!("{out}/quarantine.csv");
    let mut rows = vec![];
    // PR-0.29.3: history / quarantine are STREAMED (appended + flushed every generation), so an interrupted run
    // keeps its log; the mean is over feasible() designs only; quarantine rows carry the genotype hash.
    let resuming = (cfg.resume || cfg.fork) && std::path::Path::new(&hist_path).exists();
    let mut current: Vec<&str> = CURRENT_HISTORY_COLUMNS.to_vec();
    current.extend(Genotype::CSV_HEADER.split(','));
    let fresh_header = format!("{},{}\n", CURRENT_HISTORY_COLUMNS.join(","), Genotype::CSV_HEADER);
    let mut hf_w = match prepare_history_log(&hist_path, cfg.resume || cfg.fork, &current, &fresh_header) {
        Ok(file) => file,
        Err(e) => { eprintln!("evolve: {e}"); std::process::exit(2); }
    };
    if !resuming {
        std::fs::write(&q_path, "gen,genotype_hash,reasons\n").unwrap();
    }
    let open_app = |p: &str| std::fs::OpenOptions::new().append(true).open(p).unwrap_or_else(|e| panic!("{p}: {e}"));
    let mut qf_w = open_app(&q_path);
    let mut raw: Vec<(f64, f64)> = vec![];
    let show = |r: Result<String, String>| match r { Ok(s) => s, Err(e) => { eprintln!("evolve: {e}"); std::process::exit(2); } };
    let res = jarambit_evolve::try_run(&cfg, |s| {
        let b = &s.pop[0];
        let mean = jarambit_evolve::feasible_mean(s.pop);
        let n_feas = s.pop.iter().filter(|i| i.feasible()).count();
        if !s.terminal { for i in s.pop { raw.push((s.gen as f64, i.fit.max(-20.0))); } }
        let cut_log = show(format_cut_for_current_log(&b.cut));
        let sharp_log = show(format_stats_sharp_cell(&b.stats, SharpFormat::Log2));
        println!("gen {:3}{}  best p05 spg {:.3} N/g  cut {}  sharp {}  S={:.0}N m={:.1}g  div={:.3}  rejected={}  islands={}  t={:.1}mm grip={:.1}", s.gen, if s.terminal { " (final)" } else { "" }, b.fit, cut_log, sharp_log, b.strength_n, b.mass_g, s.diversity, s.rejected, cfg.islands, b.g.thickness, b.g.grip_len);
        if !s.terminal { rows.push((s.gen, b.fit, mean.unwrap_or(f64::NAN), s.diversity)); }
        let mean_cell = show(format_stats_mean_cell(&b.stats));
        let sharp_csv = show(format_stats_sharp_cell(&b.stats, SharpFormat::Csv4));
        let line = format!("{},{},{:.5},{},{},{:.4},{},{:.1},{:.2},{},{},{},{},{}\n", s.gen, s.terminal, b.fit, mean.map_or(String::new(), |m| format!("{m:.5}")), n_feas, s.diversity, s.rejected, b.strength_n, b.mass_g, cut_log, mean_cell, sharp_csv, jarambit_evolve::genotype_hash(&b.g), b.g.to_csv_row());
        let mut q = String::new();
        for (g, why) in &s.quarantine { let _ = writeln!(q, "{},{},\"{}\"", s.gen, jarambit_evolve::genotype_hash(g), why.replace('"', "'")); }
        { use std::io::Write as _; hf_w.write_all(line.as_bytes()).and_then(|_| hf_w.flush()).expect("history.csv append"); qf_w.write_all(q.as_bytes()).and_then(|_| qf_w.flush()).expect("quarantine.csv append"); }
    });
    let (best, front) = match res { Ok(x) => x, Err(e) => { eprintln!("evolve: {e}"); std::process::exit(2); } };
    let (p, rt, e) = nominal(&best.g);
    let hf = jarambit_redteam::holder_forces(best.g.holder.holder_wall, best.g.holder.detent_depth);
    let plate = plate_capacities(&p, material(best.g.material), 20.0, (best.g.serr_inner.depth, best.g.serr_outer.depth));
    let plate_s = plate.as_ref().map(|c| c.blade_lat_n.min(c.grip_lat_n)).unwrap_or(f64::NAN);
    let logged_s = if cfg.fem { best.strength_n } else { e.strength_n };
    let best_cut = show(format_cut_for_current_log(&best.cut));
    let best_sharp = show(format_stats_sharp_cell(&best.stats, SharpFormat::Report3));
    println!("final p05 spg {:.3} N/g  cut {}  redteam={:?}  floors_ok={}  mass={:.1}g  S={:.0}N  plate={:.0}N  insert/draw {:.1}/{:.1} N  blade tip-heel/reach/arc = {:.1}/{:.1}/{:.1} mm (limit {:.3})",
        best.fit, best_cut, rt, best.floors_ok, e.mass_g, logged_s, plate_s, hf.insertion_n, hf.draw_n, p.blade.tip_to_heel, p.blade.horizontal_reach, p.blade.sharpened_arc, limits::BLADE_LIMIT_MM);
    let w = |f: &str, s: String| std::fs::write(format!("{out}/{f}"), s).unwrap();
    let mut pc = format!("strength_n,mass_g,spg,p05_spg,{}\n", Genotype::CSV_HEADER);
    let mut fr: Vec<&Indiv> = front.iter().collect(); fr.sort_by(|a, b| a.mass_g.partial_cmp(&b.mass_g).unwrap());
    for i in &fr { let _ = writeln!(pc, "{:.1},{:.2},{:.3},{:.3},{}", i.strength_n, i.mass_g, i.strength_n / i.mass_g, i.fit, i.g.to_csv_row()); }
    w("pareto.csv", pc);
    w("pareto.svg", jarambit_docs::svg_plot("Pareto: strength vs mass (feasible, red-team clean)", "mass (g)", "governing lateral strength (N)",
        &[("Pareto front", fr.iter().map(|i| (i.mass_g, i.strength_n)).collect(), true)]));
    w("diversity.svg", jarambit_docs::svg_plot("Population diversity", "generation", "mean pairwise normalised gene distance", &[("diversity", rows.iter().map(|r| (r.0 as f64, r.3)).collect(), false)]));
    w("fitness.svg", jarambit_docs::svg_plot("Fitness (p05 strength-per-gram, N/g)", "generation", "N/g", &[
        ("best", rows.iter().map(|r| (r.0 as f64, r.1)).collect(), false), ("pop mean (feasible)", rows.iter().map(|r| (r.0 as f64, r.2)).collect(), false), ("raw individuals", raw, true)]));
    w("best.stl", p.to_ascii_stl("switchback_best"));
    w("best.step", p.to_step_stub("switchback_best"));
    w("best.svg", p.to_svg());
    w("results.md", jarambit_docs::results_table(&rows));
    let mut rep = String::new();
    let score = if cfg.fem { format!("nominal plate {:.1} N; robust plate {:.3} N/g", best.strength_n, best.fit) }
        else { format!("surrogate {:.1} N; MC p05 {:.3} N/g", e.strength_n, best.fit) };
    let _ = writeln!(rep, "# Best design constraint report\n\nredteam: {rt:?}\nmaterial: {}\nthickness: {:.1} mm\nmass: {:.2} g\nbeam surrogate: {:.1} N (blade {:.1} / grip {:.1}); ring pull {:.0} N; {:.3} N/g\nsearch score: {score}\nfloors (surrogate nominal): hiker={} hunter={} ring={}\n",
        best.g.material.name(), p.thickness, e.mass_g, e.strength_n, e.blade_lat_n, e.grip_lat_n, e.ring_pull_n, e.strength_per_gram, e.hiker_ok, e.hunter_ok, e.ring_ok);
    let _ = writeln!(rep, "blade (limit {:.3} mm, guard {:.1}): tip_to_heel {:.2} | horizontal_reach {:.2} | sharpened_arc {:.2} (outer {:.2}, inner {:.2}) | sweep {:.1} deg",
        limits::BLADE_LIMIT_MM, limits::BLADE_GUARD_MM, p.blade.tip_to_heel, p.blade.horizontal_reach, p.blade.sharpened_arc, p.blade.sharpened_arc_outer, p.blade.sharpened_arc_inner, p.blade.sweep_deg);
    let model = if cfg.fem {
        "fitness model: nominal Kirchhoff plate at 20 C, times the beam-surrogate Monte Carlo knockdown (tolerance and temperature). The plate is not solved inside each sample. Floors 180/260/1500 N apply to that robust plate estimate. Logged strength is the nominal plate."
    } else {
        "fitness model: beam surrogate (Monte Carlo). Plate check is one Kirchhoff solve at 20 C, not inside the sample loop."
    };
    let _ = writeln!(rep, "grip (ring-neck to guard): {:.2} mm\nring ID: {:.2} mm\nholder: ricasso land {:.1} mm, shoulder {:.2} mm/side, detent {:.2} mm @ {:.0}% grip, friction {:.1} mm, drain {:.1} mm, shell {:.2} mm\nholder forces (proxy): insertion {:.1} N, draw {:.1} N\nMC p05 cut margin {}  p05 sharpness {}\nsearch: NSGA-II, {} island(s), {} samples/candidate\n{model}",
        p.grip_len, p.ring_id, best.g.holder.ricasso_len, best.g.holder.shoulder_w, best.g.holder.detent_depth, best.g.holder.detent_pos * 100.0, best.g.holder.friction_len, best.g.holder.drain_d, best.g.holder.holder_wall,
        hf.insertion_n, hf.draw_n, best_cut, best_sharp, cfg.islands, cfg.mc_samples);
    match &plate {
        Some(c) => { let _ = writeln!(rep, "plate FEM: lateral {:.1} N (blade {:.1} / grip {:.1}), in-plane {:.0} N, ring pull {:.0} N, {} nodes / {} free dofs",
            c.blade_lat_n.min(c.grip_lat_n), c.blade_lat_n, c.grip_lat_n, c.inplane_n, c.ring_pull_n, c.nodes, c.free_dofs); }
        None => { let _ = writeln!(rep, "plate FEM: mesh did not solve"); }
    }
    if cfg.fine {
        let _ = writeln!(rep, "calibrated search (detailed plate in loop): robust lateral {:.1} N, robust SPG {:.3} N/g; see calibrated_pareto.md", best.strength_n, best.fit);
    }
    w("best_report.md", rep);
    if cfg.fine {
        let n = calib::verify_front(&out, &front, threads);
        println!("calibrated front: {n} LEGAL on the verification mesh -> {out}/calibrated_pareto.md");
    }
}

#[cfg(test)]
mod science_s04_tests {
    use super::*;
    use jarambit_evolve::{AbsenceReason, CutAvailability, SampleStatistics};
    use std::os::unix::fs::PermissionsExt;

    fn columns() -> Vec<&'static str> {
        let mut current: Vec<&'static str> = CURRENT_HISTORY_COLUMNS.to_vec();
        current.extend(Genotype::CSV_HEADER.split(','));
        current
    }

    fn fresh_header() -> String {
        format!("{},{}\n", CURRENT_HISTORY_COLUMNS.join(","), Genotype::CSV_HEADER)
    }

    fn fixture(name: &str) -> std::path::PathBuf {
        let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.agent/science-s04/s04-cli-unit");
        std::fs::create_dir_all(&dir).unwrap();
        dir.join(format!("{name}-{}", std::process::id()))
    }

    fn readonly(path: &std::path::Path) {
        let mut perm = std::fs::metadata(path).unwrap().permissions();
        perm.set_mode(0o444);
        std::fs::set_permissions(path, perm).unwrap();
    }

    fn restore(path: &std::path::Path) {
        let mut perm = std::fs::metadata(path).unwrap().permissions();
        perm.set_mode(0o644);
        std::fs::set_permissions(path, perm).unwrap();
    }

    #[test]
    fn science_s04_old_cut_header_errors_before_append() {
        let path = fixture("old-cut");
        std::fs::write(&path, "best_cut_p05,rest\n").unwrap();
        let before = std::fs::read(&path).unwrap();
        readonly(&path);
        let err = prepare_history_log(path.to_str().unwrap(), true, &columns(), &fresh_header()).unwrap_err();
        assert_eq!(err, "historical_cut_column_not_current");
        assert_eq!(std::fs::read(&path).unwrap(), before);
        restore(&path);
        println!("S04_CASE id=T03-10-cli header=best_cut_p05 error={err} bytes_unchanged=1");
    }

    #[test]
    fn science_s04_old_sharp_header_errors_before_append() {
        let path = fixture("old-sharp");
        std::fs::write(&path, "best_sharp_p05\n").unwrap();
        let before = std::fs::read(&path).unwrap();
        readonly(&path);
        let err = prepare_history_log(path.to_str().unwrap(), true, &columns(), &fresh_header()).unwrap_err();
        assert_eq!(err, "historical_statistics_column_not_current");
        assert_eq!(std::fs::read(&path).unwrap(), before);
        restore(&path);
        println!("S04_CASE id=T03-10-cli header=best_sharp_p05 error={err} bytes_unchanged=1");
    }

    #[test]
    fn science_s04_current_header_appends_without_duplication() {
        let path = fixture("current");
        let _ = std::fs::remove_file(&path);
        assert!(!path.exists());
        let header = fresh_header();
        let mut created = prepare_history_log(path.to_str().unwrap(), false, &columns(), &header).unwrap();
        std::io::Write::write_all(&mut created, b"row-a\n").unwrap();
        std::io::Write::flush(&mut created).unwrap();
        drop(created);
        let mut resumed = prepare_history_log(path.to_str().unwrap(), true, &columns(), &header).unwrap();
        std::io::Write::write_all(&mut resumed, b"row-b\n").unwrap();
        std::io::Write::flush(&mut resumed).unwrap();
        drop(resumed);
        let after = std::fs::read(&path).unwrap();
        let expected = format!("{header}row-a\nrow-b\n").into_bytes();
        assert_eq!(after, expected);
        let header_line = header.trim_end_matches('\n');
        let text = std::str::from_utf8(&after).unwrap();
        assert_eq!(text.lines().filter(|line| *line == header_line).count(), 1);
        println!("S04_CASE id=T03-10-cli header=current bytes={} header_count=1 rows=2", after.len());
    }

    #[test]
    fn science_s04_typed_cells_keep_historical_unavailable_and_finite_sentinel() {
        let historical = CutAvailability::Historical { value: 0.1 };
        assert_eq!(format_cut_for_current_log(&historical).unwrap_err(), "historical_cut_column_not_current");
        let unavailable = SampleStatistics::unavailable(AbsenceReason::ThreadBudget);
        assert_eq!(format_stats_mean_cell(&unavailable).unwrap(), "unavailable:thread_budget");
        assert_eq!(format_stats_sharp_cell(&unavailable, SharpFormat::Report3).unwrap(), "unavailable:thread_budget");
        let credited = SampleStatistics::credited(-1000.0, 0.0).unwrap();
        assert_eq!(format_stats_mean_cell(&credited).unwrap(), "-1000.00000");
        assert_eq!(format_stats_sharp_cell(&credited, SharpFormat::Csv4).unwrap(), "0.0000");
        // The formatted cell is "-1000.00000". Case values reject a hyphen, so the minus is spelled neg.
        println!("S04_CASE id=T03-10-cli-cells historical_cut=refused unavailable=thread_budget mean=neg1000.00000 sharp=0.0000");
    }
}
