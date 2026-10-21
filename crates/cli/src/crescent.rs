//! `jarambit crescent-smoke`: first aesthetic + novelty smoke run of the C.4-anchored crescent
//! genotype (crates/crescent). Writes history / all-evals / QD archive CSVs, plate JSON for
//! aesthetic-passing designs only, watertight STL + faceted STEP of the top design, report.md.
use jarambit_crescent::{aesthetic, export, fem, gates, genome::*, pheno, qd, record};
use std::fmt::Write as _;

/// Shortest of the three blade measures (tip-to-heel, horizontal reach, sharpened arc), inches; gate needs >= 3.375 (+0.5 mm guard).
fn blade_min_in(r: &qd::Rec) -> f64 { r.blade.tip_to_heel.min(r.blade.horizontal_reach).min(r.blade.sharpened_arc) / 25.4 }
fn jarr(v: &[(f64, f64)]) -> String { let s: Vec<String> = v.iter().map(|q| format!("[{:.3},{:.3}]", q.0, q.1)).collect(); format!("[{}]", s.join(",")) }
fn jnum(v: &[f64]) -> String { let s: Vec<String> = v.iter().map(|x| if x.is_finite() { format!("{x:.4}") } else { "null".into() }).collect(); format!("[{}]", s.join(",")) }

/// PR-0.29.1: append-only evidence log `{out}/evidence.jevl` (one fsync'ed zstd frame per generation).
fn open_evidence(out: &str, man: &jarambit_evidence::RunManifest) -> Option<jarambit_evidence::EvidenceLog> {
    match jarambit_evidence::EvidenceLog::create(&format!("{out}/evidence.jevl"), man) { Ok(l) => Some(l), Err(e) => { eprintln!("evidence log disabled: {e}"); None } }
}
fn close_evidence(l: Option<jarambit_evidence::EvidenceLog>, rep: &mut String, c: &qd::Counters) {
    let st = match &l { None => "NOT WRITTEN".to_string(), Some(l) => l.error().map_or(format!("{} frames", l.frames), |e| format!("I/O ERROR {e}")) };
    let _ = writeln!(rep, "evidence log `evidence.jevl`: {st}; fresh evaluations {}, population appearances {}\n", c.fresh_evals, c.appearances);
}

/// Strict, serde-typed record JSON (PR-0.28.3): missing FEM -> null metrics + `fem_status`, unique keys.
pub fn rec_metrics(r: &qd::Rec) -> String { record::rec_json(r) }
fn fo(r: &qd::Rec, get: impl Fn(&fem::Fem) -> f64, d: usize) -> String { record::csv_opt(r.fem.as_ref(), get, d) }

fn plate_json(r: &qd::Rec, label: &str, cert: Option<(fem::Fem, fem::Fields)>) -> Option<String> { plate_json_x(r, label, cert, "{}") }
/// Plate JSON with an extra top-level "d1" object (strict 0.0625 numbers, PASS verdicts) for the D1 plates.
fn plate_json_x(r: &qd::Rec, label: &str, cert: Option<(fem::Fem, fem::Fields)>, d1: &str) -> Option<String> {
    let p = pheno::build(&r.g);
    let (fm, fl) = match cert { Some(c) => c, None => fem::certify(&p)? };
    let fd = &p.field; let step = 2usize; // 0.5 mm thickness raster for the figure
    let mut th = Vec::new();
    for j in (0..fd.ny).step_by(step) { for i in (0..fd.nx).step_by(step) { th.push(fd.th[j * fd.nx + i] as f64); } }
    let tris: Vec<String> = fl.tris.iter().map(|t| format!("[{},{},{}]", t[0], t[1], t[2])).collect();
    let mut rr = r.clone(); rr.spg = fm.lat_rob / rr.mass_g; rr.fem = Some(fm.clone()); rr.fit = qd::fitness(&rr);
    let payload=format!("{{\"label\":{},\"metrics\":{},\"fine_area\":\"certified min(0.25, 0.125)\",\"outline\":{},\"holes\":[{}],\"hole_names\":{},\"pad\":{},\"c4\":{},\"field\":{{\"x0\":{},\"y0\":{},\"g\":{},\"nx\":{},\"ny\":{},\"th\":{}}},\"mesh\":{{\"xy\":{},\"tris\":[{}],\"te\":{},\"env_vm\":{},\"sf\":{},\"lat\":{},\"pry\":{},\"ring\":{},\"twist\":{}}},\"genome_header\":{},\"genome\":{},\"d1\":{}}}",
        serde_json::to_string(label).unwrap(), rec_metrics(&rr), jarr(&p.outline), p.holes.iter().map(|h| jarr(h)).collect::<Vec<_>>().join(","), serde_json::to_string(&p.hole_names).unwrap(), jarr(&p.pad), jarr(&jarambit_crescent::c5_ref::EXTERIOR),
        fd.x0, fd.y0, fd.g * step as f64, (fd.nx + step - 1) / step, (fd.ny + step - 1) / step, jnum(&th),
        jarr(&fl.xy), tris.join(","), jnum(&fl.te), jnum(&fl.env_vm), jnum(&fl.sf), jnum(&fl.lat), jnum(&fl.pry), jnum(&fl.ring), jnum(&fl.twist), serde_json::to_string(&Genome::csv_header()).unwrap(), serde_json::to_string(&r.g.csv_row()).unwrap(), d1);
    let mut doc:serde_json::Value=serde_json::from_str(&payload).ok()?;
    doc["certificate_identity"]=serde_json::json!(jarambit_crescent::certification::identity(&r.g));
    Some(serde_json::to_string(&doc).ok()?)
}

pub fn smoke(out: &str, pop: usize, gens: usize, threads: usize, seed: u64, t_seed: f64, fem_area: f64, plate_area: f64) -> bool {
    std::fs::create_dir_all(out).unwrap();
    let mut rep = String::new();
    let _ = writeln!(rep, "# Crescent smoke (C.4-anchored aesthetic genotype) — author Jepson Taylor\n\npop {pop} gens {gens} threads {threads} seed {seed} seed-stock {t_seed} mm, FEM search area {fem_area} mm^2, plates/winners certified at min(0.25, 0.125) mm^2 (plate-area arg {plate_area} unused), T_MAX {:.1}\n", t_max());
    // ---- probe the C.4 seed ----
    let sg = Genome::c4_seed(t_seed); let sp = pheno::build(&sg);
    let sa = aesthetic::evaluate(&sg, &sp); let sr = gates::check(&sg, &sp);
    let sf = fem::run(&sp, fem_area, false).map(|x| x.0);
    let _ = writeln!(rep, "## C.4 seed at {t_seed} mm\nmass {:.2} g (blank {:.2} g), blade {:?}, grip {:.2}\naesthetic {:?}\ngates {:?}\nFEM {:?}\nholder box {:?}\n", sp.mass_g, sp.blank_g, sp.blade, sp.grip_len, sa, sr, sf, gates::holder_fit(&sp.outline));
    eprintln!("seed: mass {:.1} g, aes pass {}, rejects {:?}, fem {:?}", sp.mass_g, sa.pass, sr, sf);
    let cfg = qd::RunCfg { pop, gens, threads, seed, fem_area, t_seed };
    let t0 = std::time::Instant::now();
    let man = jarambit_evidence::RunManifest::new("crescent-smoke", seed, fem::REQUIREMENT_SET_ID).param("pop", pop).param("gens", gens).param("threads", threads).param("t_seed", t_seed).param("fem_area", fem_area);
    let collector = match crate::regression::Collector::open(std::path::Path::new(out), &man) { Ok(c) => c, Err(e) => { eprintln!("regression collector: {e}"); return false; } };
    let mut evl = open_evidence(out, &man);
    let ro = qd::run_observed(&cfg, &qd::evaluate, |l| eprintln!("gen {l}  ({:.0}s)", t0.elapsed().as_secs_f64()), evl.as_mut().map_or(&mut jarambit_evidence::NullSink as &mut dyn jarambit_evidence::Sink, |l| l as &mut dyn jarambit_evidence::Sink), &mut |rows| collector.batch(rows));
    let ro = match ro { Ok(r) => r, Err(e) => { eprintln!("{e}"); return false; } };
    close_evidence(evl, &mut rep, &ro.counters);
    std::fs::write(format!("{out}/history.csv"), ro.history.join("\n") + "\n").unwrap();
    let mut all = String::from("[\n"); let mut csv = format!("id,gen,fit,spg,mass_g,aes_pass,aes_score,hausdorff,emd,novelty,lat_rob,ring_rob,sf_hunter,legal,n_rejects,{}\n", Genome::csv_header());
    for (i, r) in ro.all.iter().enumerate() {
        let _ = writeln!(csv, "{},{},{:.4},{:.4},{:.2},{},{:.4},{:.3},{:.4},{:.4},{},{},{},{},{},{}", r.id, r.gen, r.fit, r.spg, r.mass_g, r.aes.pass, r.aes.score, r.aes.hausdorff, r.aes.emd, r.novelty, fo(r, |f| f.lat_rob, 1), fo(r, |f| f.ring_rob, 1), fo(r, |f| f.sf_hunter, 3), r.legal(), r.rejects.len(), r.g.csv_row());
        all.push_str(&rec_metrics(r)); all.push_str(if i + 1 < ro.all.len() { ",\n" } else { "\n" });
    }
    all.push(']'); std::fs::write(format!("{out}/all_evals.json"), all).unwrap(); std::fs::write(format!("{out}/all_evals.csv"), csv).unwrap();
    let mut ac = String::from("cell_removal,cell_mass_bin,id,gen,fit,spg,mass_g,aes_score,novelty,sf_hunter\n");
    for ((a, b), r) in &ro.archive { let _ = writeln!(ac, "{a},{b},{},{},{:.4},{:.4},{:.2},{:.4},{:.4},{}", r.id, r.gen, r.fit, r.spg, r.mass_g, r.aes.score, r.novelty, fo(r, |f| f.sf_hunter, 3)); }
    std::fs::write(format!("{out}/qd_archive.csv"), ac).unwrap();
    // ---- reject census ----
    let mut cnt: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for r in &ro.all { for x in r.aes.fails.iter().chain(r.rejects.iter()) { let k: String = x.split(|c: char| c.is_ascii_digit() || c == '(').next().unwrap_or("").trim().to_string(); *cnt.entry(k).or_default() += 1; } }
    let n_all = ro.all.len(); let n_aes = ro.all.iter().filter(|r| r.aes.pass).count(); let n_gate = ro.all.iter().filter(|r| r.aes.pass && r.rejects.is_empty()).count(); let n_leg = ro.all.iter().filter(|r| r.legal()).count();
    let _ = writeln!(rep, "## Census\nevaluations {n_all}; aesthetic passers {n_aes}; aesthetic + all hard gates {n_gate}; LEGAL (also floors 180/260/1500 robust) {n_leg}\n\nreject reasons (count):");
    for (k, v) in &cnt { let _ = writeln!(rep, "- {k}: {v}"); }
    // ---- pick plate designs: aesthetic passers only ----
    let mut cand: Vec<&qd::Rec> = ro.all.iter().filter(|r| r.aes.pass && r.rejects.is_empty() && r.fem.is_some()).collect();
    cand.sort_by(|a, b| { let ka = (a.legal(), a.fit); let kb = (b.legal(), b.fit); kb.partial_cmp(&ka).unwrap() });
    // certify in search-fitness order; a design is plated only if it is still LEGAL at the
    // converged reference levels (min over 0.25 / 0.125 mm^2, no bias). SPG / fitness re-derived.
    let mut picked: Vec<qd::Rec> = Vec::new(); let mut cells = std::collections::HashSet::new();
    let mut certs = Vec::new(); let (mut n_try, mut n_fail) = (0usize, 0usize);
    for r in &cand { if picked.len() >= 3 || n_try >= 40 { break; }
        if picked.iter().any(|p| p.g == r.g) { continue; }
        let cell = qd::qd_cell(r); if cells.contains(&cell) && cand.len() >= 6 { continue; }
        n_try += 1;
        let p = pheno::build(&r.g);
        let Some((cf, fl)) = fem::certify(&p) else { n_fail += 1; continue; };
        if r.legal() && !cf.floors_ok { n_fail += 1; let _ = writeln!(rep, "- certification FAIL id {} (search lat_rob {:.0} -> certified {:.0} N, ring {:.0})", r.id, r.fem.as_ref().unwrap().lat_rob, cf.lat_rob, cf.ring_rob); continue; }
        let mut rr = (*r).clone(); rr.spg = cf.lat_rob / rr.mass_g; rr.fem = Some(cf.clone()); rr.fit = qd::fitness(&rr);
        cells.insert(cell); certs.push((cf, fl)); picked.push(rr);
    }
    let _ = writeln!(rep, "\n## Certification\ncandidates certified {n_try}, rejected at the converged mesh {n_fail}\n\n## Plate designs (aesthetic passers only; certified metrics)");
    let mut plates = Vec::new();
    for (k, (r, c)) in picked.iter().zip(certs.into_iter()).enumerate() {
        let label = format!("crescent_smoke_{}", ["top", "qd2", "qd3"][k]);
        if let Some(j) = plate_json(r, &label, Some(c)) { std::fs::write(format!("{out}/{label}.json"), &j).unwrap(); plates.push(label.clone()); }
        let _ = writeln!(rep, "- {label}: {}", rec_metrics(r));
    }
    // seed reference plate (aesthetic passer by construction) for comparison
    let seed_rec = qd::evaluate(&sg, usize::MAX, 0, fem_area);
    if seed_rec.aes.pass { if let Some(j) = plate_json(&seed_rec, "crescent_c4_seed", None) { std::fs::write(format!("{out}/crescent_c4_seed.json"), j).unwrap(); plates.push("crescent_c4_seed".into()); } }
    // ---- watertight STL + faceted STEP of the top design ----
    if let Some(r) = picked.first() {
        let p = pheno::build(&r.g);
        if let Some(s) = export::solid(&p, 1.0) {
            let (once, more, dmax) = export::census(&s.tris); let v = export::volume(&s);
            std::fs::write(format!("{out}/best.stl"), export::stl(&s, "jarambit_crescent_best")).unwrap();
            std::fs::write(format!("{out}/best.step"), export::step(&s, "jarambit_crescent_best")).unwrap();
            let _ = writeln!(rep, "\n## Export (top design)\nwatertight: boundary edges {once}, non-manifold {more}, max directed-edge repeat {dmax} => {}; solid volume {v:.0} mm^3 = {:.2} g (raster {:.2} g); faces {}", if once == 0 && more == 0 && dmax == 1 { "WATERTIGHT" } else { "NOT WATERTIGHT" }, v * jarambit_physics::material(r.g.mat).density / 1000.0, r.mass_g, s.tris.len());
        }
        std::fs::write(format!("{out}/best_genome.csv"), format!("{}\n{}\n", Genome::csv_header(), r.g.csv_row())).unwrap();
    }
    let _ = writeln!(rep, "\nplates: {:?}\nwall time {:.0} s", plates, t0.elapsed().as_secs_f64());
    std::fs::write(format!("{out}/report.md"), rep).unwrap();
    std::fs::write(format!("{out}/plates.txt"), plates.join("\n") + "\n").unwrap();
    true
}

/// Quick probe: seed gates / aesthetic / FEM at several stocks and tip-reinforcement variants (no GA).
pub fn probe(fem_area: f64) {
    let mut vars: Vec<(String, Genome)> = [5.3, 5.7, 6.0].iter().map(|&t| (format!("seed t{t}"), Genome::c4_seed(t))).collect();
    let mut v = Genome::c4_seed(5.7); v.v[EA_IN] = 22.0; v.v[EA_SP] = 22.0; v.v[EDGE_TH] = 0.9; v.v[T_TIP] = 4.5; vars.push(("ea22 edge0.9 tip4.5".into(), v.clone()));
    for i in [1usize, 2, 3] { v.v[i] = 3.0; } for i in [36usize, 37, 38] { v.v[i] = 2.5; } v.clamp(); vars.push(("+fat tip offsets".into(), v.clone()));
    for (name, g) in vars {
        let t = name.clone(); let p = pheno::build(&g);
        let a = aesthetic::evaluate(&g, &p); let r = gates::check(&g, &p);
        let f = fem::run(&p, fem_area, false).map(|x| x.0);
        println!("t {t}: mass {:.2} blank {:.2} aes {:.3} pass {} H {:.2} rejects {:?}\n  fem {:?}\n  holder {:?} land {:.0} minweb {:.2}", p.mass_g, p.blank_g, a.score, a.pass, a.hausdorff, r, f, gates::holder_fit(&p.outline), p.land_full_mm2, p.min_web);
    }
}

/// Crescent mesh-convergence calibration: detailed plate at the search area vs finer levels on
/// aesthetic passers sampled from a smoke run (all_evals.csv). Ground edges and the hawkbill tip
/// make the crescent more mesh-sensitive than the flat jarambit plate, so it carries its own bias.
pub fn conv(evals_csv: &str, n: usize, threads: usize, out: &str) -> bool {
    let txt = match std::fs::read_to_string(evals_csv) { Ok(t) => t, Err(e) => { eprintln!("{evals_csv}: {e}"); return false; } };
    let mut gs: Vec<(String, Genome)> = vec![("c4_seed".into(), Genome::c4_seed(5.7))];
    let hdr: Vec<&str> = txt.lines().next().unwrap_or("").split(',').collect();
    let off = hdr.iter().position(|h| *h == "cp00").unwrap_or(15);
    let col = |n: &str, d: usize| hdr.iter().position(|h| *h == n).unwrap_or(d);
    let (c_aes, c_fit, c_leg) = (col("aes_pass", 5), col("fit", 2), col("legal", 13));
    let rows: Vec<Vec<&str>> = txt.lines().skip(1).map(|l| l.split(',').collect()).filter(|f: &Vec<&str>| f.len() > off && f[c_aes] == "true").collect();
    // legal first (highest fitness), then a stride through the remaining aesthetic passers
    let mut legal: Vec<&Vec<&str>> = rows.iter().filter(|f| f[c_leg] == "true").collect();
    legal.sort_by(|a, b| b[c_fit].parse::<f64>().unwrap_or(-9.0).partial_cmp(&a[c_fit].parse::<f64>().unwrap_or(-9.0)).unwrap());
    let mut seen = std::collections::HashSet::new();
    let mut add = |gs: &mut Vec<(String, Genome)>, f: &Vec<&str>| { if seen.insert(f[off..].join(",")) { if let Some(g) = Genome::from_csv(&f[off..]) { gs.push((format!("id{}", f[0]), g)); } } };
    for f in legal.iter() { if gs.len() > n / 2 { break; } add(&mut gs, f); }
    let stride = (rows.len() / (n - n / 2).max(1)).max(1);
    for f in rows.iter().step_by(stride) { if gs.len() > n { break; } add(&mut gs, f); }
    const LV: [f64; 5] = [2.0, 1.0, 0.5, 0.25, 0.125];
    let res: Vec<(String, Vec<Option<fem::Fem>>)> = std::thread::scope(|s| {
        let chunks: Vec<_> = gs.chunks(gs.len().div_ceil(threads.max(1))).map(|c| s.spawn(move || c.iter().map(|(k, g)| {
            let p = pheno::build(g); (k.clone(), LV.iter().map(|&a| fem::run(&p, a, false).map(|x| x.0)).collect::<Vec<_>>()) }).collect::<Vec<_>>())).collect();
        chunks.into_iter().flat_map(|h| h.join().unwrap()).collect()
    });
    let mut md = format!("# Crescent mesh convergence (detailed plate, 20 C, per-element thickness)\n\nlevels (max triangle area, mm^2): {LV:?}; reference = 0.125 mm^2. ratio = capacity(level) / capacity(0.125).\n\n| design | lat20 @2 | @1 | @0.5 | @0.25 | @0.125 | lat ratio 2/ref | 0.25/ref | ring ratio 2/ref | 0.25/ref | lat_at @ref |\n|---|---|---|---|---|---|---|---|---|---|---|\n");
    let (mut lmax, mut rmax, mut lv, mut rv) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    for (k, v) in &res {
        if v.iter().any(|x| x.is_none()) { let _ = writeln!(md, "| {k} | mesh failed | | | | | | | | | |"); continue; }
        let f: Vec<&fem::Fem> = v.iter().map(|x| x.as_ref().unwrap()).collect(); let r = f[4];
        let (l2, l25, r2, r25) = (f[0].lat20 / r.lat20, f[3].lat20 / r.lat20, f[0].ring20 / r.ring20, f[3].ring20 / r.ring20);
        lmax = lmax.max(l2); rmax = rmax.max(r2); lv = lv.max(l25); rv = rv.max(r25);
        let _ = writeln!(md, "| {k} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {l2:.3} | {l25:.3} | {r2:.3} | {r25:.3} | ({:.1}, {:.1}) |", f[0].lat20, f[1].lat20, f[2].lat20, f[3].lat20, r.lat20, r.lat_at.0, r.lat_at.1);
    }
    let _ = writeln!(md, "\nMAX search(2.0)/ref: lateral {lmax:.3}, ring {rmax:.3}. MAX 0.25/ref: lateral {lv:.3}, ring {rv:.3}.");
    println!("{md}");
    std::fs::create_dir_all(out).ok(); std::fs::write(format!("{out}/convergence.md"), md).is_ok()
}

/// Load seed genomes: every CSV row that parses as a genome (current or legacy 90-gene layout) at
/// the column of the `cp00` header; rows of an all_evals.csv are filtered to LEGAL and ranked by fit.
pub fn load_seeds(paths: &str, top: usize) -> Vec<Genome> {
    let mut out: Vec<Genome> = Vec::new();
    for path in paths.split(',').filter(|s| !s.is_empty()) {
        let Ok(txt) = std::fs::read_to_string(path) else { eprintln!("seeds: cannot read {path}"); continue; };
        let mut lines = txt.lines(); let Some(h) = lines.next() else { continue; };
        let hdr: Vec<&str> = h.split(',').collect();
        let Some(off) = hdr.iter().position(|c| *c == "cp00") else { continue; };
        let (fi, li) = (hdr.iter().position(|c| *c == "fit"), hdr.iter().position(|c| *c == "legal"));
        let mut rows: Vec<(f64, Genome)> = lines.filter_map(|l| { let f: Vec<&str> = l.split(',').collect();
            if let Some(li) = li { if f.get(li) != Some(&"true") { return None; } }
            let fit = fi.and_then(|i| f.get(i)).and_then(|v| v.parse().ok()).unwrap_or(0.0);
            Genome::from_csv(&f[off.min(f.len())..]).map(|g| (fit, g)) }).collect();
        rows.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        for (_, g) in rows.into_iter().take(top) { if !out.contains(&g) { out.push(g); } }
    }
    out
}

fn write_progress(out: &str, gen: usize, all: &[qd::Rec], archive: &std::collections::BTreeMap<(usize, usize), qd::Rec>) {
    let mut ac = String::from("cell_removal,cell_mass_bin,id,gen,fit,spg,mass_g,aes_score,novelty,relief_frac,sf_hunter,n_win\n");
    for ((a, b), r) in archive { let _ = writeln!(ac, "{a},{b},{},{},{:.4},{:.4},{:.2},{:.4},{:.4},{:.4},{},{}", r.id, r.gen, r.fit, r.spg, r.mass_g, r.aes.score, r.novelty, qd::relief_frac(r), fo(r, |f| f.sf_hunter, 3), r.n_win); }
    std::fs::write(format!("{out}/qd_archive.csv"), ac).ok();
    let mut leg: Vec<&qd::Rec> = all.iter().filter(|r| r.legal()).collect();
    leg.sort_by(|a, b| a.mass_g.partial_cmp(&b.mass_g).unwrap());
    let mut s = format!("# progress gen {gen}: evals {}, LEGAL (search level) {}, FEASIBLE (all strength floors) {}\n\nlightest LEGAL (search-level, NOT yet certified):\n", all.len(), leg.len(), leg.iter().filter(|r| qd::feasible(r)).count());
    for r in leg.iter().take(12) { let f = r.fem.as_ref().unwrap(); let _ = writeln!(s, "- id {} gen {} mass {:.2} g t {:.1} SPG {:.3} lat_rob {:.0} ring_rob {:.0} A {:.3} nov {:.3} relief {:.3} win {} fit {:.3}", r.id, r.gen, r.mass_g, r.t, r.spg, f.lat_rob, f.ring_rob, r.aes.score, r.novelty, qd::relief_frac(r), r.n_win, r.fit); }
    std::fs::write(format!("{out}/progress.md"), s).ok();
}

/// `jarambit crescent-evolve`: island GA + shared novelty/MAP-Elites over the crescent genotype WITH the
/// internal-machining layer (isogrid pockets, pierced-fuller windows, ring flute). Seeds from certified
/// wave6 crescents + C.4. Hard floors / gates / aesthetic floor identical to crescent-smoke. Every plated
/// design is CERTIFIED at min(0.25, 0.125 mm^2); STEP exported for every certified plate.
#[allow(clippy::too_many_arguments)]
pub fn evolve(out: &str, wave: &str, pop: usize, islands: usize, gens: usize, threads: usize, seed: u64, fem_area: f64, seeds_csv: &str, seed_top: usize, migrate: usize, kick: f64, n_cert: usize) -> bool {
    std::fs::create_dir_all(out).unwrap();
    // D1 P1 (JARAMBIT_P1=1): seeds are clamped so the continuous-rail layer is ON and every through-feature OFF
    let seeds = { let mut s = load_seeds(seeds_csv, seed_top); s.push(Genome::c6_seed(5.7)); if p1_env() { for g in s.iter_mut() { g.clamp(); } } s };
    let mut rep = String::new();
    let _ = writeln!(rep, "# {wave} — crescent internal-machining evolve (island GA + QD) — author Jepson Taylor\n\nislands {islands} x pop {pop}, gens {gens}, migrate every {migrate}, threads {threads}, seed {seed}, FEM search area {fem_area} mm^2 (search bias lat/ring {:?}, JARAMBIT_SEARCH_BIAS_* override), certification min(0.25, 0.125 mm^2) no bias, T_MAX {:.1}, fitness cap-free v2 (no mass cap; ln(84.8/m) reward, knee 70 g; strength floors hard){:?}, seeds {} (from {seeds_csv} + {}), aesthetic anchor SWITCHBACK Rev {} (C.5 jimping R0.8/0.45 + C.6 two-sided ring-face groove 0.6/face, 1.6 mm rims), machining kick {kick}, min through-features {} (JARAMBIT_MIN_THROUGH)\n", fem::search_bias(), t_max(), "", seeds.len(), jarambit_crescent::c6_ref::rev(), jarambit_crescent::c6_ref::rev(), qd::min_through());
    let cfg = qd::IslandCfg { base: qd::RunCfg { pop, gens, threads, seed, fem_area, t_seed: 5.7 }, islands, migrate_every: migrate, migrants: 2, seeds, kick_frac: kick };
    let t0 = std::time::Instant::now();
    let mut hist_f = String::new();
    let man = jarambit_evidence::RunManifest::new("crescent-evolve", seed, fem::REQUIREMENT_SET_ID).param("wave", wave).param("pop", pop).param("islands", islands).param("gens", gens).param("threads", threads)
        .param("fem_area", fem_area).param("seeds_csv", seeds_csv).param("seed_top", seed_top).param("migrate", migrate).param("kick", kick).param("n_cert", n_cert).param("p1_env", p1_env())
        .param("fitness", "capfree-v2").param("min_through", qd::min_through()).param("search_bias", format!("{:?}", fem::search_bias()));
    let collector = match crate::regression::Collector::open(std::path::Path::new(out), &man) { Ok(c) => c, Err(e) => { eprintln!("regression collector: {e}"); return false; } };
    let mut evl = open_evidence(out, &man);
    let ro = qd::run_islands_observed(&cfg, &qd::evaluate, |l| { eprintln!("{l}  ({:.0}s)", t0.elapsed().as_secs_f64()); hist_f.push_str(l); hist_f.push('\n'); std::fs::write(format!("{out}/history_live.csv"), &hist_f).ok(); },
        |gen, all, arch| { if gen % 5 == 0 { write_progress(out, gen, all, arch); } }, evl.as_mut().map_or(&mut jarambit_evidence::NullSink as &mut dyn jarambit_evidence::Sink, |l| l as &mut dyn jarambit_evidence::Sink), &mut |rows| collector.batch(rows));
    let ro = match ro { Ok(r) => r, Err(e) => { eprintln!("{e}"); return false; } };
    close_evidence(evl, &mut rep, &ro.counters);
    std::fs::write(format!("{out}/history.csv"), ro.history.join("\n") + "\n").unwrap();
    write_progress(out, gens, &ro.all, &ro.archive);
    let mut all = String::from("[\n"); let mut csv = format!("id,gen,fit,spg,mass_g,aes_pass,aes_score,hausdorff,emd,novelty,lat_rob,ring_rob,sf_hunter,legal,n_rejects,relief_frac,n_win,{}\n", Genome::csv_header());
    for (i, r) in ro.all.iter().enumerate() {
        let _ = writeln!(csv, "{},{},{:.4},{:.4},{:.2},{},{:.4},{:.3},{:.4},{:.4},{},{},{},{},{},{:.4},{},{}", r.id, r.gen, r.fit, r.spg, r.mass_g, r.aes.pass, r.aes.score, r.aes.hausdorff, r.aes.emd, r.novelty, fo(r, |f| f.lat_rob, 1), fo(r, |f| f.ring_rob, 1), fo(r, |f| f.sf_hunter, 3), r.legal(), r.rejects.len(), qd::relief_frac(r), r.n_win, r.g.csv_row());
        all.push_str(&rec_metrics(r)); all.push_str(if i + 1 < ro.all.len() { ",\n" } else { "\n" });
    }
    all.push(']'); std::fs::write(format!("{out}/all_evals.json"), all).unwrap(); std::fs::write(format!("{out}/all_evals.csv"), csv).unwrap();
    let mut cnt: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for r in &ro.all { for x in r.aes.fails.iter().chain(r.rejects.iter()) { let k: String = x.split(|c: char| c.is_ascii_digit() || c == '(').next().unwrap_or("").trim().to_string(); *cnt.entry(k).or_default() += 1; } }
    let n_all = ro.all.len(); let n_aes = ro.all.iter().filter(|r| r.aes.pass).count(); let n_gate = ro.all.iter().filter(|r| r.aes.pass && r.rejects.is_empty()).count();
    let n_leg = ro.all.iter().filter(|r| r.legal()).count(); let n_cap = ro.all.iter().filter(|r| qd::feasible(r)).count();
    let _ = writeln!(rep, "## Census\nevaluations {n_all}; aesthetic passers {n_aes}; aesthetic + all hard gates {n_gate}; LEGAL (search level) {n_leg}; FEASIBLE (all strength floors incl. seq + twist) {n_cap}\n\nreject reasons (count):");
    for (k, v) in &cnt { let _ = writeln!(rep, "- {k}: {v}"); }
    // ---- certification: LEGAL+under-cap first (by fit), then lightest LEGAL, then most novel; distinct genomes ----
    let mut cand: Vec<&qd::Rec> = ro.all.iter().filter(|r| r.aes.pass && r.rejects.is_empty() && r.fem.is_some()).collect();
    cand.sort_by(|a, b| { let ka = (a.legal() && qd::feasible(a), a.legal(), a.fit); let kb = (b.legal() && qd::feasible(b), b.legal(), b.fit); kb.partial_cmp(&ka).unwrap() });
    let mut light: Vec<&qd::Rec> = ro.all.iter().filter(|r| r.legal()).collect();
    light.sort_by(|a, b| (!qd::feasible(a)).cmp(&(!qd::feasible(b))).then_with(|| a.mass_g.partial_cmp(&b.mass_g).unwrap()));
    let mut novel: Vec<&qd::Rec> = ro.all.iter().filter(|r| r.legal()).collect(); novel.sort_by(|a, b| b.novelty.partial_cmp(&a.novelty).unwrap());
    let mut order: Vec<(&str, &qd::Rec)> = Vec::new();
    if qd::all_gates_env() {
        for r in cand.iter().take(24) { order.push(("fit", r)); }
        for r in light.iter().take(16) { order.push(("light", r)); }
        for r in novel.iter().take(8) { order.push(("novel", r)); }
    } else {
        for (i, r) in cand.iter().enumerate() { if i < 30 { order.push(("fit", r)); } }
        for r in light.iter().take(20) { order.push(("light", r)); }
        for r in novel.iter().take(10) { order.push(("novel", r)); }
    }
    let mut picked: Vec<(String, qd::Rec, fem::Fem, fem::Fields)> = Vec::new(); let (mut n_try, mut n_fail) = (0usize, 0usize);
    let mut seen: Vec<Genome> = Vec::new(); let mut cert_rows = String::from("kind,id,gen,mass_g,t,search_lat_rob,cert_lat_rob,cert_ring_rob,sf_hiker,sf_hunter,sf_ring,twist_sf_1p82_joint3d,floors_ok_cert,spg_cert,aes,novelty,relief_frac,n_win,n_perf,strict_lat_rob_0p0625,strict_ok,seq_ratio_cert,seq_case,seq_ok,twist_tip_sf_default_hard,twist_body_sf,twist_tipzone_sf_strict,t_joint,cert_pass_all,twist_floor_ok_body_and_tipsub,mass_band_74_77,blade_min_in,promote_eligible_pending_aesthetic_signoff\n");
    let mut queue: Vec<(&str, &qd::Rec)> = Vec::new();
    let q_cap = if qd::all_gates_env() { (6 * n_cert).max(36) } else { (2 * n_cert).max(12) };
    for (kind, r) in order { if queue.len() >= q_cap { break; } // P1: floors-OK designs that miss only the seq>=1.2 proxy are still certified + plated (labelled FAIL), so the wave is documented
        let floors_only = r.g.p1() && r.rejects.is_empty() && r.aes.pass && r.fem.as_ref().map_or(false, |f| f.floors_ok);
        if seen.contains(&r.g) || !(r.legal() || floors_only) { continue; } seen.push(r.g.clone()); queue.push((kind, r)); }
    // certify the queue in parallel (each certification is single-threaded: 0.25 + 0.125 mm^2 solves)
    let th = threads.max(1).min(queue.len().max(1));
    // JARAMBIT_CERT_STRICT=1: also solve 0.0625 mm^2 (refinement beyond certification; machined designs
    // were found to keep losing 3-7 % below 0.125 mm^2). strict_ok = robust floors also hold there.
    let strict = std::env::var("JARAMBIT_CERT_STRICT").map_or(false, |v| v == "1");
    let certs: Vec<(Option<(fem::Fem, fem::Fields)>, Option<fem::Fem>)> = std::thread::scope(|s| {
        let hs: Vec<_> = (0..th).map(|t| { let q = &queue; s.spawn(move || q.iter().enumerate().filter(|(i, _)| i % th == t).map(|(i, (_, r))| { let p = pheno::build(&r.g); (i, fem::certify(&p), if strict { fem::run_unbiased(&p, 0.0625) } else { None }) }).collect::<Vec<_>>()) }).collect();
        let mut v: Vec<(usize, Option<(fem::Fem, fem::Fields)>, Option<fem::Fem>)> = hs.into_iter().flat_map(|h| h.join().unwrap()).collect(); v.sort_by_key(|x| x.0); v.into_iter().map(|x| (x.1, x.2)).collect()
    });
    let mut strict_ok_ids: Vec<usize> = Vec::new();
    let mut strict_info: std::collections::HashMap<usize, String> = std::collections::HashMap::new();
    for ((kind, r), (c, f16)) in queue.into_iter().zip(certs) {
        n_try += 1;
        let Some((cf, fl)) = c else { n_fail += 1; continue; };
        let (s_lat, s_ok) = match &f16 { Some(f) => { let m = jarambit_physics::material(r.g.mat); let rob = (f.lat20 * fem::joint_k(m, r.g.t_joint(), 2)).min(cf.lat_rob);
            (rob, rob >= jarambit_physics::floors::HUNTER_LATERAL_N) } None => (f64::NAN, !strict) };
        // D1 P1 (or JARAMBIT_ALL_GATES=1): certified PASS also needs the failure-sequence proxy >= 1.2 at certification (hard)
        let seq_ok = !(r.g.p1() || qd::all_gates_env()) || cf.seq_ratio >= fem::SEQ_TARGET;
        let tz16 = f16.as_ref().map_or(f64::NAN, |f| f.twist_tipzone_sf);
        let prom = fem::promote_eligible(r.aes.pass, r.rejects.is_empty(), cf.floors_ok, s_lat, cf.seq_ratio, cf.twist_body_sf, tz16);
        if s_ok && cf.floors_ok && seq_ok && (!qd::all_gates_env() || (prom && cf.twist_sf >= 1.0)) { strict_ok_ids.push(r.id); }
        strict_info.insert(r.id, serde_json::json!({"strict_lat_rob_0p0625": record::fin(s_lat, 1), "strict_ok": s_ok, "seq_ok": seq_ok, "twist_tipzone_sf_strict": record::fin(tz16, 3),
            "pass_all_828_joint_strict_seq": s_ok && cf.floors_ok && seq_ok, "twist_floor_ok": fem::twist_floor_ok(cf.twist_body_sf, tz16), "promote_eligible": prom}).to_string());
        let tw = cf.twist_sf;
        let _ = writeln!(cert_rows, "{kind},{},{},{:.2},{:.1},{:.1},{:.1},{:.1},{:.3},{:.3},{:.3},{:.3},{},{:.4},{:.4},{:.4},{:.4},{},{},{:.1},{},{:.3},{},{},{:.3},{:.3},{:.3},{:.3},{},{},{},{:.3},{}", r.id, r.gen, r.mass_g, r.t, r.fem.as_ref().unwrap().lat_rob, cf.lat_rob, cf.ring_rob, cf.sf_hiker, cf.sf_hunter, cf.sf_ring, tw, cf.floors_ok, cf.lat_rob / r.mass_g, r.aes.score, r.novelty, qd::relief_frac(r), r.n_win, r.n_perf, s_lat, s_ok && cf.floors_ok,
            cf.seq_ratio, cf.seq_case, seq_ok, cf.twist_sf, cf.twist_body_sf, tz16, r.g.t_joint(), s_ok && cf.floors_ok && seq_ok,
            fem::twist_floor_ok(cf.twist_body_sf, tz16), r.mass_g >= fem::MASS_BAND.0 && r.mass_g <= fem::MASS_BAND.1, blade_min_in(r), prom);
        if !cf.floors_ok || (qd::all_gates_env() && !(prom && cf.twist_sf >= 1.0)) { n_fail += 1; let _ = writeln!(rep, "- certification FAIL id {} ({kind}, {:.1} g): search lat_rob {:.0} -> certified {:.0} N, strict {:.0} N, seq {:.2}, tip_sf {:.2}, tipz_strict {:.2}", r.id, r.mass_g, r.fem.as_ref().unwrap().lat_rob, cf.lat_rob, s_lat, cf.seq_ratio, cf.twist_sf, tz16); continue; }
        if picked.len() >= n_cert { continue; }
        let mut rr = (*r).clone(); rr.spg = cf.lat_rob / rr.mass_g;
        let mut cf_fit = cf.clone();
        if qd::all_gates_env() { if s_lat.is_finite() { cf_fit.lat_rob = cf_fit.lat_rob.min(s_lat); } if tz16.is_finite() { cf_fit.twist_tipzone_sf = cf_fit.twist_tipzone_sf.min(tz16); } }
        rr.fem = Some(cf_fit); rr.fit = qd::fitness(&rr); rr.fem = Some(cf.clone());
        picked.push((kind.to_string(), rr, cf, fl));
    }
    std::fs::write(format!("{out}/certification.csv"), cert_rows).unwrap();
    let _ = writeln!(rep, "\n## Certification (min over 0.25 / 0.125 mm^2, no bias)\ncandidates certified {n_try}, rejected at the converged mesh {n_fail}\n\n## Plate designs (aesthetic passers only; certified metrics; twist SF = advisory)");
    // rank certified: under-cap by fitness first
    picked.sort_by(|a, b| { let ka = (strict_ok_ids.contains(&a.1.id), qd::feasible(&a.1), a.1.fit); let kb = (strict_ok_ids.contains(&b.1.id), qd::feasible(&b.1), b.1.fit); kb.partial_cmp(&ka).unwrap() });
    let mut plates = Vec::new();
    for (k, (kind, r, cf, fl)) in picked.into_iter().enumerate() {
        let label = format!("{wave}_c{:02}_{kind}", k + 1);
        let tw = cf.twist_sf;
        let d1x = strict_info.get(&r.id).cloned().unwrap_or_else(|| "{}".into());
        if let Some(j) = plate_json_x(&r, &label, Some((cf, fl)), &d1x) { std::fs::write(format!("{out}/{label}.json"), &j).unwrap(); plates.push(label.clone()); }
        let _ = writeln!(rep, "- {label} (strict 0.0625 mm^2 check: {}; chord-torsion SF vs 1.82 N*m joint+3D {tw:.2}, advisory; n_win {}, n_perf {}, relief_frac {:.4}): {}", if strict_ok_ids.contains(&r.id) { "PASS" } else if strict { "FAIL" } else { "not run" }, r.n_win, r.n_perf, qd::relief_frac(&r), rec_metrics(&r));
        let p = pheno::build(&r.g);
        if let Some(s) = export::solid(&p, 1.0) {
            let (once, more, dmax) = export::census(&s.tris); let v = export::volume(&s);
            std::fs::write(format!("{out}/{label}.stl"), export::stl(&s, &label)).unwrap();
            std::fs::write(format!("{out}/{label}.step"), export::step(&s, &label)).unwrap();
            let _ = writeln!(rep, "  export: boundary edges {once}, non-manifold {more}, max directed repeat {dmax} => {}; volume {v:.0} mm^3 = {:.2} g (raster {:.2} g)", if once == 0 && more == 0 && dmax == 1 { "WATERTIGHT" } else { "NOT WATERTIGHT" }, v * jarambit_physics::material(r.g.mat).density / 1000.0, r.mass_g);
        }
        std::fs::write(format!("{out}/{label}_genome.csv"), format!("{}\n{}\n", Genome::csv_header(), r.g.csv_row())).unwrap();
        if k == 0 || std::env::var("JARAMBIT_CAD_BATTERY_ALL").as_deref() == Ok("1") {
            evolve_tail_hook(out, &label);
        }
    }
    let _ = writeln!(rep, "\nplates: {:?}\nwall time {:.0} s", plates, t0.elapsed().as_secs_f64());
    std::fs::write(format!("{out}/report.md"), rep).unwrap();
    std::fs::write(format!("{out}/plates.txt"), plates.join("\n") + "\n").unwrap();
    true
}

/// Probe the internal-machining layer on a genome (no GA): ablation of each feature family.
pub fn mprobe(genome_csv: &str, fem_area: f64) {
    let gs = load_seeds(genome_csv, 1); let Some(g0) = gs.first() else { eprintln!("no genome"); return; };
    let mut vars: Vec<(String, Genome)> = vec![("as-is".into(), g0.clone())];
    for t in [5.3, 5.5] { let mut g = g0.clone(); g.v[T] = t; g.clamp(); vars.push((format!("t{t}"), g)); }
    let mut g = g0.clone(); g.v[T] = 5.5; g.v[ISO_D] = 1.2; g.v[ISO_ZB] = 1.0; g.v[ISO_XB] = 0.55; g.v[ISO_P] = 11.0; g.clamp(); vars.push(("t5.5 iso blade".into(), g.clone()));
    g.v[ISO_ZH] = 1.0; vars.push(("t5.5 iso blade+handle".into(), g.clone()));
    g.v[WIN_N] = 4.0; g.v[WIN_S0] = 0.1; g.v[WIN_S1] = 0.6; g.v[WIN_W] = 7.0; g.clamp(); vars.push(("+4 windows".into(), g.clone()));
    g.v[RFL_D] = 1.0; g.v[RFL_ARC] = 220.0; vars.push(("+ring flute".into(), g.clone()));
    let mut g2 = g0.clone(); g2.v[T] = 6.0; g2.v[ISO_D] = 1.4; g2.v[ISO_ZL] = 1.0; g2.v[ISO_ZB] = 1.0; g2.v[ISO_XB] = 0.5; g2.v[ISO_RIM] = 2.0; g2.v[RING_WALL] = 4.8; g2.v[RFL_D] = 1.0; g2.v[RFL_ARC] = 220.0; g2.clamp(); vars.push(("t6 land+blade iso, ring wall 4.8, flute".into(), g2.clone()));
    g2.v[ISO_ZH] = 1.0; g2.v[ISO_XH] = 0.6; g2.v[ISO_GR] = -0.8; vars.push(("+handle iso (far 40%, graded)".into(), g2.clone()));
    let mut g3 = g0.clone(); g3.v[T] = 6.0; g3.v[PERF_D] = 4.5; g3.v[PERF_GAP] = 3.5; g3.v[PERF_ZL] = 1.0; g3.clamp(); vars.push(("t6 perf land d4.5".into(), g3.clone()));
    g3.v[PERF_ZH] = 1.0; g3.v[PERF_XH] = 0.4; g3.v[PERF_E] = 1.8; vars.push(("t6 perf land+handle(far 60%) e1.8".into(), g3.clone()));
    for (name, g) in vars {
        let p = pheno::build(&g);
        { let fd = &p.field; let rho = jarambit_physics::material(g.mat).density / 1000.0; let mut z = [0.0f64; 4];
          for j in 0..fd.ny { for i in 0..fd.nx { let v = fd.th[j * fd.nx + i]; if v.is_nan() { continue; } let x = fd.x0 + (i as f64 + 0.5) * fd.g;
            let k = if x < p.ring_od / 2.0 { 0 } else if x < 78.0 + p.dx { 1 } else if x < 100.0 + p.dx { 2 } else { 3 }; z[k] += v as f64 * fd.g * fd.g * rho; } }
          println!("  mass by zone (g): ring {:.1} handle {:.1} land {:.1} blade {:.1}; land_full {:.0} mm2", z[0], z[1], z[2], z[3], p.land_full_mm2); } let a = aesthetic::evaluate(&g, &p); let r = gates::check(&g, &p);
        let f = fem::run(&p, fem_area, false).map(|x| x.0);
        println!("{name}: perf {} mass {:.2} t {:.1} removed {:?} win {} aes {} rejects {:?}\n  lat_rob {:.0} ring_rob {:.0} lat_at {:?} floors {}", p.n_perf, p.mass_g, p.t, p.removed.map(|x| x.round()), p.n_win, a.pass, r,
            f.as_ref().map_or(0.0, |f| f.lat_rob), f.as_ref().map_or(0.0, |f| f.ring_rob), f.as_ref().map(|f| f.lat_at), f.as_ref().map_or(false, |f| f.floors_ok));
    }
}

/// Refinement study beyond certification: unbiased lateral / ring capacities at 0.25, 0.125, 0.0625 mm^2
/// for every genome row of the given CSVs (certified winners), in parallel.
pub fn refine(csvs: &str, threads: usize, out: &str) -> bool {
    let mut gs: Vec<(String, Genome)> = Vec::new();
    for path in csvs.split(',').filter(|s| !s.is_empty()) { for g in load_seeds(path, 1) { gs.push((path.rsplit('/').next().unwrap_or(path).replace("_genome.csv", ""), g)); } }
    const LV: [f64; 3] = [0.25, 0.125, 0.0625];
    let th = threads.max(1).min(gs.len().max(1) * LV.len());
    let jobs: Vec<(usize, usize)> = (0..gs.len()).flat_map(|i| (0..LV.len()).map(move |l| (i, l))).collect();
    let res: Vec<((usize, usize), Option<fem::Fem>)> = std::thread::scope(|s| {
        let hs: Vec<_> = (0..th).map(|t| { let (jobs, gs) = (&jobs, &gs); s.spawn(move || jobs.iter().enumerate().filter(|(k, _)| k % th == t).map(|(_, &(i, l))| ((i, l), fem::run_unbiased(&pheno::build(&gs[i].1), LV[l]))).collect::<Vec<_>>()) }).collect();
        hs.into_iter().flat_map(|h| h.join().unwrap()).collect()
    });
    let mut md = String::from("# Refinement beyond certification (unbiased, 20 C capacities)\n\n| design | mass g | lat20 @0.25 | @0.125 | @0.0625 | 0.0625/min(0.25,0.125) | ring20 @0.25 | @0.125 | @0.0625 | lat_at @0.0625 | robust lat @0.0625 | hunter SF @0.0625 |\n|---|---|---|---|---|---|---|---|---|---|---|---|\n");
    for (i, (k, g)) in gs.iter().enumerate() {
        let f: Vec<Option<&fem::Fem>> = (0..LV.len()).map(|l| res.iter().find(|x| x.0 == (i, l)).and_then(|x| x.1.as_ref())).collect();
        if f.iter().any(|x| x.is_none()) { let _ = writeln!(md, "| {k} | mesh failed |||||||||||"); continue; }
        let f: Vec<&fem::Fem> = f.into_iter().map(|x| x.unwrap()).collect(); let p = pheno::build(g);
        let m = jarambit_physics::material(g.mat);
        let rob = f[2].lat20 * fem::joint_k(m, p.t, 2);
        let _ = writeln!(md, "| {k} | {:.2} | {:.1} | {:.1} | {:.1} | {:.3} | {:.0} | {:.0} | {:.0} | ({:.1}, {:.1}) | {:.1} | {:.3} |", p.mass_g, f[0].lat20, f[1].lat20, f[2].lat20, f[2].lat20 / f[0].lat20.min(f[1].lat20), f[0].ring20, f[1].ring20, f[2].ring20, f[2].lat_at.0, f[2].lat_at.1, rob, rob / 260.0);
    }
    println!("{md}"); std::fs::create_dir_all(out).ok(); std::fs::write(format!("{out}/refine.md"), md).is_ok()
}

/// Re-score prior certified crescents against the adopted SWITCHBACK Rev C.5 anchor: every genome
/// (one per `*_genome.csv`, or the top `top` legal rows of an all_evals CSV) is rebuilt with the C.5
/// phenotype (C.5 jimping; jimp_scale clamped into the C.5 band), then aesthetic floor vs C.5, all hard
/// gates, certification min(0.25, 0.125 mm^2) and the strict 0.0625 mm^2 level. Writes rescore.csv /
/// rescore.md and `c5_pass_genomes.csv` (only designs passing EVERYTHING) for re-seeding.
pub fn rescore(csvs: &str, top: usize, threads: usize, out: &str) -> bool {
    let mut gs: Vec<(String, Genome)> = Vec::new();
    for path in csvs.split(',').filter(|s| !s.is_empty()) {
        let tag = path.rsplit('/').next().unwrap_or(path).replace("_genome.csv", "").replace(".csv", "");
        for (k, mut g) in load_seeds(path, top).into_iter().enumerate() { g.clamp();
            // C.6 repair: a ring wall too thin for the 1.6 mm rims + 2 mm groove is re-tried at the minimum C.6 wall
            let wmin = 2.0 * jarambit_crescent::c6_ref::RING_GROOVE_RIM + jarambit_crescent::c6_ref::RING_GROOVE_MIN_W + 0.01;
            if jarambit_crescent::c6_ref::active() && g.v[RING_WALL] < wmin { let mut h = g.clone(); h.v[RING_WALL] = wmin; h.clamp(); if !gs.iter().any(|x| x.1 == h) { gs.push((format!("{tag}#{k}+c6wall"), h)); } }
            if !gs.iter().any(|x| x.1 == g) { gs.push((format!("{tag}#{k}"), g)); } }
    }
    gs.push((format!("{}_seed_t5.7", jarambit_crescent::c6_ref::rev()), Genome::c6_seed(5.7)));
    let th = threads.max(1).min(gs.len().max(1));
    let res: Vec<(usize, qd::Rec, Option<fem::Fem>, Option<fem::Fem>)> = std::thread::scope(|s| {
        let hs: Vec<_> = (0..th).map(|t| { let gs = &gs; s.spawn(move || gs.iter().enumerate().filter(|(i, _)| i % th == t).map(|(i, (_, g))| {
            let r = qd::evaluate(g, i, 0, 2.0); let p = pheno::build(g);
            let ok = r.aes.pass && r.rejects.is_empty();
            let c = if ok { fem::certify(&p).map(|x| x.0) } else { None };
            let s16 = if ok { fem::run_unbiased(&p, 0.0625) } else { None };
            (i, r, c, s16) }).collect::<Vec<_>>()) }).collect();
        let mut v: Vec<_> = hs.into_iter().flat_map(|h| h.join().unwrap()).collect(); v.sort_by_key(|x| x.0); v
    });
    let rev_tag = jarambit_crescent::c6_ref::rev();
    let mut csv = String::from("design,mass_g,t,aes_pass_c5,aes_score_c5,hausdorff_c5,emd,jimp_depth_mm,gates_ok,cert_lat_rob,cert_ring_rob,sf_hiker,sf_hunter,sf_ring,cert_floors_ok,strict_lat_rob_0p0625,strict_sf_hunter,strict_ok,spg_cert,all_pass,fails\n");
    let mut md = format!("# Re-score vs SWITCHBACK Rev {rev_tag} (C.5 jimping R0.8 / 0.45 mm; C.6 adds the two-sided ring-face groove)\n\nEvery design rebuilt with the {rev_tag} phenotype; certification = min(0.25, 0.125 mm^2) unbiased; strict = 0.0625 mm^2 unbiased (robust factors applied).\n\n| design | mass g | t | A({rev_tag}) | H mm | gates | cert lat_rob N | SF hiker | SF hunter | SF ring | strict lat_rob @0.0625 | SPG cert | ALL PASS | fails |\n|---|---|---|---|---|---|---|---|---|---|---|---|---|---|\n");
    let mut pass_rows = format!("{}\n", Genome::csv_header()); let mut n_pass = 0;
    for (i, r, c, s16) in &res {
        let name = &gs[*i].0; let gates_ok = r.rejects.is_empty();
        let m = jarambit_physics::material(r.g.mat);
        let s_rob = s16.as_ref().map_or(f64::NAN, |f| f.lat20 * fem::joint_k(m, r.t, 2));
        let cfl = c.as_ref().map_or(false, |f| f.floors_ok); let s_ok = s_rob >= jarambit_physics::floors::HUNTER_LATERAL_N;
        let all = r.aes.pass && gates_ok && cfl && r.mass_g < 80.0;
        let mut fails: Vec<String> = r.aes.fails.clone(); fails.extend(r.rejects.iter().cloned());
        if c.is_some() && !cfl { fails.push("cert floors".into()); } if r.mass_g >= 80.0 { fails.push(format!("mass {:.1} >= 80 g", r.mass_g)); }
        let (cl, cr, sh, su, sr) = c.as_ref().map_or((f64::NAN, f64::NAN, f64::NAN, f64::NAN, f64::NAN), |f| (f.lat_rob, f.ring_rob, f.sf_hiker, f.sf_hunter, f.sf_ring));
        let _ = writeln!(csv, "{name},{:.2},{:.1},{},{:.4},{:.3},{:.4},{:.3},{},{:.1},{:.1},{:.3},{:.3},{:.3},{},{:.1},{:.3},{},{:.4},{},\"{}\"", r.mass_g, r.t, r.aes.pass, r.aes.score, r.aes.hausdorff, r.aes.emd, 0.45 * r.g.v[JIMP], gates_ok, cl, cr, sh, su, sr, cfl, s_rob, s_rob / 260.0, s_ok, cl / r.mass_g, all, fails.join("; "));
        let _ = writeln!(md, "| {name} | {:.2} | {:.1} | {:.3}{} | {:.2} | {} | {:.1} | {:.3} | {:.3} | {:.2} | {:.1}{} | {:.3} | {} | {} |", r.mass_g, r.t, r.aes.score, if r.aes.pass { "" } else { " FAIL" }, r.aes.hausdorff, if gates_ok { "ok" } else { "FAIL" }, cl, sh, su, sr, s_rob, if s_ok { "" } else { " (<260)" }, cl / r.mass_g, if all { "**YES**" } else { "no" }, fails.join("; "));
        if all { n_pass += 1; let _ = writeln!(pass_rows, "{}", r.g.csv_row()); }
    }
    let _ = writeln!(md, "\n{n_pass} of {} designs pass {rev_tag} aesthetic + all gates + certified floors + < 80 g.", res.len());
    std::fs::create_dir_all(out).ok();
    println!("{md}");
    std::fs::write(format!("{out}/rescore.csv"), csv).is_ok() && std::fs::write(format!("{out}/rescore.md"), md).is_ok() && std::fs::write(format!("{out}/{}_pass_genomes.csv", rev_tag.replace('.', "").to_lowercase()), pass_rows).is_ok()
}


/// D1 review re-screen: every genome (one per CSV row 1) is rebuilt with the CURRENT phenotype (C.6 anchor),
/// aesthetic + gates, certification min(0.25, 0.125 mm^2) and strict 0.0625 mm^2, all at the 828 MPa screening
/// yield with the JOINT thin+hot factor; chord-torsion fixture vs the 1.82 N*m floor (plate + St-Venant 3D);
/// the pre-D1 numbers (880 MPa, min(thin, hot), bending-couple "twist") are reported alongside for the record.
/// `name=path` entries label rows; `ref:` prefix marks thin-stock REFERENCE rows (never ranked with evolved parts).
/// `name=path` or a bare path. A bare path is named by its file stem without `.csv` / `_genome` (PR-0.30.1b):
/// before, the whole path (with '/') became the name, so the D1 plate JSON and grind-law file were written to
/// non-existent directories and silently dropped, which broke the D1 -> fem_solid pipeline.
pub fn design_entry(ent: &str) -> (String, &str) {
    if let Some((n, p)) = ent.split_once('=') { return (n.to_string(), p); }
    let stem = std::path::Path::new(ent).file_name().and_then(|s| s.to_str()).unwrap_or(ent);
    let stem = stem.strip_suffix(".csv").unwrap_or(stem); let stem = stem.strip_suffix("_genome").unwrap_or(stem);
    (stem.to_string(), ent)
}

pub fn d1(csvs: &str, threads: usize, out: &str) -> bool {
    let mut gs: Vec<(String, Genome)> = Vec::new();
    for ent in csvs.split(',').filter(|s| !s.is_empty()) {
        let (name, path) = design_entry(ent);
        if path == "C6_SEED_T5.7" { gs.push((name.to_string(), Genome::c6_seed(5.7))); continue; }
        if path == "C6_STOCK_3P175" { // thin-stock REFERENCE as built (bypasses clamp, like tests::c4_stock_genome)
            let mut g = Genome::c6_seed(5.7); g.v[T] = 3.175; g.v[T_TIP] = 2.6; g.v[TAPER_X0] = 130.0; g.v[SPINE_OFF] = 0.0;
            g.v[EA_IN] = ((3.175 - 0.5) / 2.0 / 6.0f64).atan().to_degrees(); g.v[EA_SP] = ((3.175 - 0.5) / 2.0 / 5.0f64).atan().to_degrees();
            for k in 0..5 { g.v[K0 + k] = 3.175; }
            gs.push((name.to_string(), g)); continue; }
        if let Some(g) = load_seeds(path, 1).into_iter().next() { gs.push((name.to_string(), g)); } else { eprintln!("d1: no genome in {path}"); }
    }
    // D1_PLATES=1: also write a D1-rule plate JSON per design (certified fields; plates.txt) for make_d1_plate.py
    let plates_on = std::env::var("D1_PLATES").map_or(false, |v| v == "1");
    let th = threads.max(1).min(gs.len().max(1));
    type Row = (usize, qd::Rec, Option<(fem::Fem, fem::Fields)>, Option<fem::Fem>);
    let res: Vec<Row> = std::thread::scope(|s| {
        let hs: Vec<_> = (0..th).map(|t| { let gs = &gs; s.spawn(move || gs.iter().enumerate().filter(|(i, _)| i % th == t).map(|(i, (_, g))| {
            let r = qd::evaluate(g, i, 0, 2.0); let p = pheno::build(g);
            if std::env::var("D1_DUMP").is_ok() { eprintln!("D1_DUMP {}: holes {:?} outline_n {} mass {:.4}", gs[i].0, p.holes.iter().zip(&p.hole_names).map(|(h, n)| (*n, (h.iter().map(|q| q.0).fold(f64::MAX, f64::min) * 100.0).round() / 100.0, (h.iter().map(|q| q.1).fold(f64::MAX, f64::min) * 100.0).round() / 100.0)).collect::<Vec<_>>(), p.outline.len(), p.mass_g); }
            std::fs::create_dir_all(out).ok(); let gp = format!("{out}/{}_grind_law.json", gs[i].0.replace(['(', ')', '.'], "_")); if let Err(e) = std::fs::write(&gp, export::grind_law_json(&p, g, &gs[i].0)) { eprintln!("d1: cannot write {gp}: {e}"); }
            let c = fem::certify(&p); if let Some(c) = &c { eprintln!("D1 {}: lat_at {:?} lat20 {:.1} twist_at {:?}", gs[i].0, c.0.lat_at, c.0.lat20, c.0.twist_at); }
            (i, r, c, fem::run_unbiased(&p, 0.0625)) }).collect::<Vec<_>>()) }).collect();
        let mut v: Vec<_> = hs.into_iter().flat_map(|h| h.join().unwrap()).collect(); v.sort_by_key(|x| x.0); v
    });
    let fl = jarambit_physics::floors::HIKER_LATERAL_N; let fh = jarambit_physics::floors::HUNTER_LATERAL_N; let fr = jarambit_physics::floors::RING_PULL_N;
    let k880 = jarambit_physics::TI64_YIELD_TYPICAL_MPA / jarambit_physics::TI_6AL_4V.yield_mpa;
    let mut csv = String::from("design,mass_g,t,t_joint,aes_pass,gates_ok,lat20_828,joint_lat_cert,sf_hiker,sf_hunter,joint_ring_cert,sf_ring,strict_joint_lat_0p0625,strict_sf_hunter,pass_828_joint,old_lat_rob_880_min,old_sf_hunter_880_min,twist20_plate_nmm,twist_kc_3d,twist_b_mm,twist_t_mm,twist_joint3d_nmm,twist_tip_sf_1p82_default_hard,twist_tip_pass,twist_peak_tipdist_mm,twist_body15_joint_nmm,twist_body_kc,twist_body_sf_1p82,twist_tipzone_sf_strict,twist_hand_joint_nmm,old_couple20_nmm,old_twist_sf_proxy,seq_ratio,seq_case,seq_pass,hunter_aim_286,pass_all_p1,twist_floor_ok_body_and_tipsub,mass_band_74_77,blade_min_in,promote_eligible_pending_aesthetic_signoff\n");
    let mut md = String::from("| design | mass g | t (t_joint) mm | A pass | gates | joint lat N (cert) | SF hiker | SF hunter | joint ring N | SF ring | strict 0.0625 joint lat N (SF hunter) | PASS @828 joint | aim 286 | pre-D1 lat_rob 880/min N | **tip-only twist SF (default hard)** | body twist SF (+3D) | tip-submodel SF (4-15 mm, strict) | peak dist from tip mm | seq ratio (case) | P1 PASS (floors+strict+seq) | twist floor (body AND tip-sub) | mass 74-77 | blade min in | **PROMOTE-ELIGIBLE** (pending aesthetic sign-off) |\n|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|\n");
    let mut plates: Vec<String> = Vec::new();
    for (i, r, c, s16) in &res {
        let name = &gs[*i].0; let gates_ok = r.rejects.is_empty(); let m = jarambit_physics::material(r.g.mat);
        let Some((c, fields)) = c else { let _ = writeln!(md, "| {name} | {:.2} | {:.1} | {} | {} | certify failed |", r.mass_g, r.t, r.aes.pass, gates_ok); continue; };
        let tj = r.g.t_joint();
        let s_j = s16.as_ref().map_or(f64::NAN, |f| f.lat20 * fem::joint_k(m, tj, 2));
        let tz = s16.as_ref().map_or(f64::NAN, |f| f.twist_tipzone_sf);
        let pass = c.lat_rob >= fl && c.lat_rob >= fh && c.ring_rob >= fr && s_j >= fh;
        let hot = m.yield_at(100.0) / m.yield_at(20.0); let tk = ((r.t - 0.1) / r.t).powi(2);
        let old = c.lat20 * k880 * tk.min(hot); let old_tw = c.couple20_nmm * k880 * hot / fem::TWIST_DESIGN_NMM;
        let tw_pass = c.twist_sf >= 1.0; let seq_pass = c.seq_ratio >= fem::SEQ_TARGET; let aim = c.lat_rob >= 286.0 && s_j >= 286.0;
        let pass_all = pass && seq_pass && gates_ok && r.aes.pass;
        let tw_ok = fem::twist_floor_ok(c.twist_body_sf, tz); let band = r.mass_g >= fem::MASS_BAND.0 && r.mass_g <= fem::MASS_BAND.1;
        let prom = fem::promote_eligible(r.aes.pass, gates_ok, c.floors_ok, s_j, c.seq_ratio, c.twist_body_sf, tz);
        let _ = writeln!(csv, "{name},{:.2},{:.1},{:.3},{},{},{:.1},{:.1},{:.3},{:.3},{:.0},{:.3},{:.1},{:.3},{},{:.1},{:.3},{:.0},{:.3},{:.2},{:.2},{:.0},{:.3},{},{:.1},{:.0},{:.3},{:.3},{:.3},{:.0},{:.0},{:.3},{:.3},{},{},{},{},{},{},{:.3},{}",
            r.mass_g, r.t, tj, r.aes.pass, gates_ok, c.lat20, c.lat_rob, c.sf_hiker, c.sf_hunter, c.ring_rob, c.sf_ring, s_j, s_j / fh, pass, old, old / fh,
            c.twist20_nmm, c.twist_kc, c.twist_b_mm, c.twist_t_mm, c.twist_joint_nmm, c.twist_sf, tw_pass, c.twist_tipdist, c.twist_body_joint_nmm, c.twist_body_kc, c.twist_body_sf, tz, c.twist_hand_joint_nmm, c.couple20_nmm, old_tw, c.seq_ratio, c.seq_case, seq_pass, aim, pass_all, tw_ok, band, blade_min_in(r), prom);
        let _ = writeln!(md, "| {name} | {:.2} | {:.1} ({:.2}) | {} | {} | {:.1} | {:.3} | {:.3} | {:.0} | {:.2} | {:.1} ({:.3}) | {} | {} | {:.1} | **{:.2} {}** | {:.2} {} | {:.2} {} | {:.1} | {:.2} ({}) {} | {} | {} | {} | {:.3} | {} |",
            r.mass_g, r.t, tj, if r.aes.pass { "yes" } else { "NO" }, if gates_ok { "ok" } else { "FAIL" }, c.lat_rob, c.sf_hiker, c.sf_hunter, c.ring_rob, c.sf_ring, s_j, s_j / fh,
            if pass { "**PASS**" } else { "**FAIL**" }, if aim { "yes" } else { "no" }, old, c.twist_sf, if tw_pass { "PASS" } else { "FAIL" }, c.twist_body_sf, if c.twist_body_sf >= 1.0 { "PASS" } else { "FAIL" },
            tz, if tz >= 1.0 { "PASS" } else { "FAIL" }, c.twist_tipdist, c.seq_ratio, c.seq_case, if seq_pass { "PASS" } else { "FAIL" }, if pass_all { "**PASS**" } else { "**FAIL**" },
            if tw_ok { "PASS" } else { "FAIL" }, if band { "yes" } else { "no" }, blade_min_in(r), if prom { "**YES**" } else { "no" });
        if plates_on {
            let label = format!("{}_D1", name.replace(['(', ')', '.', ' '], "_"));
            let d1x = serde_json::json!({"strict_lat_rob_0p0625": record::fin(s_j, 1), "strict_ok": s_j >= fh, "seq_ok": seq_pass, "twist_tipzone_sf_strict": record::fin(tz, 3),
                "pass_828_joint_strict": pass, "pass_all_828_joint_strict_seq": pass_all, "old_lat_rob_880_min": record::fin(old, 1), "twist_floor_ok": tw_ok, "promote_eligible": prom}).to_string();
            let mut rr = r.clone(); rr.fem = Some(c.clone()); rr.spg = c.lat_rob / rr.mass_g;
            if let Some(j) = plate_json_x(&rr, &label, Some((c.clone(), fem::Fields { xy: fields.xy.clone(), tris: fields.tris.clone(), te: fields.te.clone(), env_vm: fields.env_vm.clone(), sf: fields.sf.clone(), lat: fields.lat.clone(), pry: fields.pry.clone(), ring: fields.ring.clone(), twist: fields.twist.clone() })), &d1x) {
                let jp = format!("{out}/{label}.json");
                match std::fs::write(&jp, j) { Ok(()) => plates.push(label), Err(e) => eprintln!("d1: cannot write plate JSON {jp}: {e}") }
                let gcsv = format!("{out}/{}_genome.csv", name.replace(['(', ')', '.', ' '], "_"));
                std::fs::write(&gcsv, format!("{}\n{}\n", Genome::csv_header(), r.g.csv_row())).ok();
                maybe_run_cad_battery(out, &name.replace(['(', ')', '.', ' '], "_"), &gcsv);
            }
        }
    }
    let hdr = format!("# D1 re-screen — screening yield {:.0} MPa (TIMET min, provisional), JOINT thin+hot, chord-torsion floor {:.2} N*m\n\nCertification min(0.25, 0.125 mm^2) unbiased; strict 0.0625 mm^2 unbiased; joint factor = ((t_j-0.1)/t_j)^2 x yield(100C)/yield(20C) (ring: ^1), t_j = thinnest knot (P1) else stock. PASS @828 joint = hiker 180 / hunter 260 / ring 1500 N at certification AND hunter at strict. P1 PASS also needs seq ratio >= {:.1}. Twist floor (OWNER LOCK 2026-10-11 14:52 MT): **body AND tip submodel** both >= 1.82 N*m (hard, certification); **tip-only** (1.82 N*m at the 2 mm tip patch, peak outside 4 mm, St-Venant 3D at the peak) ALWAYS reported, not the gate; **body** = same torque, peak > 15 mm from the tip with its own 3D factor; **tip submodel** = peak restricted to the 4-15 mm tip annulus at the strict 0.0625 mesh with its own 3D factor. Seq ratio = hand-zone / blade-zone yield-onset load (min over lateral, chord-torsion, pry). PROMOTE-ELIGIBLE (OWNER LOCK) = C.6 aesthetic + all hard gates (blade >= 3.375 in on all three) + 828 joint floors + strict hunter + seq >= 1.2 + twist body AND tip-sub; mass is SOFT (Jepson 2026-10-12: G penalty / target < 80 g then < 70 g, 74-77 band reported, not a gate); hunter aim 286 reported. NOTHING is promoted without human aesthetic sign-off.\n\n",
        jarambit_physics::TI_6AL_4V.yield_mpa, fem::TWIST_FLOOR_NMM / 1000.0, fem::SEQ_TARGET);
    let md = hdr + &md;
    std::fs::create_dir_all(out).ok(); println!("{md}");
    if plates_on { std::fs::write(format!("{out}/plates.txt"), plates.join("\n") + "\n").ok(); }
    std::fs::write(format!("{out}/d1_rescreen.csv"), csv).is_ok() && std::fs::write(format!("{out}/d1_rescreen.md"), md).is_ok()
}
pub fn should_run_cad_battery_from(flag: Option<&str>, depth: Option<&str>) -> bool {
    flag == Some("1") && depth != Some("1")
}
fn default_battery_probes() -> jarambit_crescent::battery::Probes {
    jarambit_crescent::battery::Probes::default()
}
fn maybe_run_cad_battery(out: &str, label: &str, genome_csv: &str) {
    let flag = std::env::var("JARAMBIT_CAD_BATTERY").ok();
    let depth = std::env::var("JARAMBIT_CAD_BATTERY_DEPTH").ok();
    if !should_run_cad_battery_from(flag.as_deref(), depth.as_deref()) { return; }
    let root = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let entry = format!("{label}={genome_csv}");
    let probes = default_battery_probes();
    if let Err(e) = jarambit_crescent::battery::run_cad_battery_native(
        &root,
        None,
        Some(&entry),
        std::path::Path::new(out),
        None,
        &probes,
    ) {
        eprintln!("cad_battery({label}): {e}");
    }
}

pub fn evolve_tail_hook(out: &str, label: &str) {
    maybe_run_cad_battery(&format!("{out}/cad_battery"), label, &format!("{out}/{label}_genome.csv"));
}
#[cfg(test)]
mod design_entry_tests {
    use super::{default_battery_probes, design_entry, should_run_cad_battery_from};
    #[test]
    fn bare_path_is_named_by_stem() {
        assert_eq!(design_entry("out/x/w271b_11772.csv"), ("w271b_11772".to_string(), "out/x/w271b_11772.csv"));
        assert_eq!(design_entry("out/wave27/g3_tip+00_t60_tt3_genome.csv").0, "g3_tip+00_t60_tt3");
        assert_eq!(design_entry("g3=out/a.csv"), ("g3".to_string(), "out/a.csv"));
        assert_eq!(design_entry("C6_SEED_T5.7").0, "C6_SEED_T5.7");
    }
    #[test]
    fn cad_battery_recursion_guard_clears_env_and_stops_at_depth_one() {
        assert!(!should_run_cad_battery_from(None, None));
        assert!(!should_run_cad_battery_from(Some("0"), None));
        assert!(should_run_cad_battery_from(Some("1"), None));
        assert!(should_run_cad_battery_from(Some("1"), Some("0")));
        assert!(!should_run_cad_battery_from(Some("1"), Some("1")));
        let probes = default_battery_probes();
        assert_eq!(probes.analytic.as_str(), "pass");
        assert_eq!(probes.dolfinx.as_deref(), None);
    }
}

