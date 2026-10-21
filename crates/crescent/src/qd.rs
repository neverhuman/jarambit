//! Evaluation record, novelty (k-NN in a 3-D-feature descriptor space, aesthetic passers only),
//! MAP-Elites archive, and the GA loop.  Fitness (PR-0.31.2, cap-free) = 1 + ln mass reward
//! + margin/aesthetic/novelty bonuses for FEASIBLE designs; graded negative penalties otherwise (never winners).
use crate::aesthetic::{self, Aes};
use crate::fem::{self, Fem};
use crate::gates;
use crate::genome::*;
use crate::pheno::*;
use jarambit_genotype::Rng;
use jarambit_evidence::{Event, Sink, NullSink, OP_VERSION};

pub const DESC_N: usize = 16;

#[derive(Clone, Debug)]
pub struct Rec {
    pub id: usize, pub gen: usize, pub g: Genome,
    pub mass_g: f64, pub blank_g: f64, pub t: f64, pub blade: Blade, pub grip: f64,
    pub aes: Aes, pub rejects: Vec<String>, pub fem: Option<Fem>,
    pub spg: f64, pub novelty: f64, pub fit: f64, pub desc: [f64; DESC_N], pub holder_lw: (f64, f64, f64),
    pub removed: [f64; NREM], pub min_web: f64, pub n_win: usize, pub win_area: f64, pub n_perf: usize, pub perf_area: f64,
    /// PR-0.29.1 lineage: parent ids, variation operator, operator-semantics version.
    pub parents: Vec<usize>, pub op: &'static str, pub op_version: u32,
}
impl Rec {
    pub fn aesthetic_pass(&self) -> bool { self.aes.pass }
    /// LEGAL = gates + floors; with the P1 layer ON (or JARAMBIT_ALL_GATES=1) also the D1 failure-sequence proxy >= 1.2 (hard)
    /// and the OWNER-LOCKED twist floor (body AND tip submodel >= 1.82 N*m) at search level.
    pub fn legal(&self) -> bool { self.rejects.is_empty() && self.fem.as_ref().map_or(false, |f| f.floors_ok && (!(self.g.p1() || all_gates_env()) || (f.seq_ratio >= fem::SEQ_TARGET && fem::twist_floor_ok(f.twist_body_sf, f.twist_tipzone_sf)))) }
}

pub fn descriptor(g: &Genome, p: &Pheno) -> [f64; DESC_N] {
    let blank = (p.area_mm2 * p.t).max(1.0);
    let zone = |r: std::ops::RangeInclusive<usize>| { let v: Vec<f64> = r.clone().map(|i| g.v[i] / cp_delta(i).max(1e-9)).collect(); 0.5 + 0.5 * v.iter().sum::<f64>() / v.len() as f64 };
    let th: Vec<f64> = p.field.th.iter().filter(|v| !v.is_nan()).map(|&v| v as f64).collect();
    let mean = th.iter().sum::<f64>() / th.len().max(1) as f64;
    let cv = (th.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / th.len().max(1) as f64).sqrt() / mean.max(1e-9);
    let holes_area: f64 = p.holes.iter().skip(1).map(|h| crate::geom::signed_area(h).abs()).sum();
    [(p.removed[0] / blank * 10.0).min(1.0), (p.removed[1] / blank * 10.0).min(1.0), (p.removed[2] / blank * 20.0).min(1.0),
     (p.removed[3] / blank * 20.0).min(1.0), if g.v[TEX_MODE] >= 0.5 { g.v[TEX_D] / 0.4 } else { 0.0 },
     (holes_area / p.area_mm2.max(1.0) * 5.0).min(1.0), zone(1..=7), zone(9..=13), zone(14..=18), zone(30..=35),
     (cv * 2.0).min(1.0), (g.v[DROP] + 4.0) / 8.0,
     // internal machining layer (wave7+): isogrid pockets, pierced-fuller windows, ring flute
     (p.removed[5] / blank * 8.0).min(1.0), (p.win_area / p.area_mm2.max(1.0) * 12.0).min(1.0), (p.removed[6] / blank * 25.0).min(1.0), (p.perf_area / p.area_mm2.max(1.0) * 12.0).min(1.0)]
}

pub fn evaluate(g: &Genome, id: usize, gen: usize, fem_area: f64) -> Rec {
    let p = build(g);
    let aes = aesthetic::evaluate(g, &p);
    let rejects = gates::check(g, &p);
    let (_, l, w, ang) = gates::holder_fit(&p.outline);
    let desc = descriptor(g, &p);
    let fem = if aes.pass && rejects.is_empty() { fem::run(&p, fem_area, false).map(|x| x.0) } else { None };
    let spg = fem.as_ref().map_or(0.0, |f| f.lat_rob / p.mass_g);
    Rec { id, gen, g: g.clone(), mass_g: p.mass_g, blank_g: p.blank_g, t: p.t, blade: p.blade.clone(), grip: p.grip_len, aes, rejects, fem,
        spg, novelty: 0.0, fit: 0.0, desc, holder_lw: (l, w, ang), removed: p.removed, min_web: p.min_web, n_win: p.n_win, win_area: p.win_area, n_perf: p.n_perf, perf_area: p.perf_area,
        parents: Vec::new(), op: "eval", op_version: OP_VERSION }
}

fn d2(a: &[f64; DESC_N], b: &[f64; DESC_N]) -> f64 { a.iter().zip(b).map(|(x, y)| (x - y).powi(2)).sum::<f64>().sqrt() }

/// Novelty among aesthetic passers only: mean distance to the k nearest passers (archive + pop).
pub fn novelty(d: &[f64; DESC_N], others: &[[f64; DESC_N]], k: usize) -> f64 {
    let mut v: Vec<f64> = others.iter().map(|o| d2(d, o)).filter(|x| *x > 1e-12).collect();
    if v.is_empty() { return 0.0; }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let k = k.min(v.len()); v[..k].iter().sum::<f64>() / k as f64
}

/// JARAMBIT_MIN_THROUGH: minimum count of through-features for the through-lattice lane (default 0 = off).
pub fn min_through() -> usize { std::env::var("JARAMBIT_MIN_THROUGH").ok().and_then(|v| v.parse().ok()).unwrap_or(0) }
/// JARAMBIT_TIPTWIST_GATE=1 (PR-0.30.1c): make the twist floor (body AND 4-15 mm tip submodel SF >= 1 at 1.82 N*m)
/// constraint-dominated for every design, not just P1 genomes. A floors-OK design that misses it ranks in (-1, -0.1),
/// graded by min(body, tip) SF, so the search cannot buy mass by thinning the tip rail/ribs.
pub fn tiptwist_gate() -> bool { std::env::var("JARAMBIT_TIPTWIST_GATE").map_or(false, |v| v == "1") }
/// JARAMBIT_ALL_GATES=1: enforce all D1 gates (seq_ratio >= 1.2, twist_body_sf >= 1.0, twist_tipzone_sf >= 1.0)
/// as hard legality gates for all genomes, with knee-capped survival margin (+8% knee) so mass reduction dominates.
pub fn all_gates_env() -> bool { std::env::var("JARAMBIT_ALL_GATES").map_or(false, |v| v == "1") }
/// Provisional reference mass (g) for the cap-free mass-reward term (PR-0.31.2).
pub const MASS_REF_G: f64 = 84.8;
/// Below this mass (g) the log mass reward keeps rising but at `MASS_KNEE_SLOPE` of the rate (diminishing returns).
pub const MASS_KNEE_G: f64 = 70.0;
pub const MASS_KNEE_SLOPE: f64 = 0.25;
/// Weight of the strength-margin bonus (saturating; keeps the search off the floor edge without buying mass).
pub const MARGIN_W: f64 = 0.03;
/// Soft mass TARGET (Jepson decision 2026-10-12, PR-0.31.4): mass is never a gate, but G carries a strong smooth
/// penalty above 80 g on top of the ln reward: MASS_OVER_W * (1 - exp(-MASS_OVER_K * ((m - 80) / 80)^2)).
/// C1 at 80 g (no cliff); 84.8 g -> -0.04, 90 g -> -0.13, 95 g -> -0.23 (bonuses total <= 0.07).
pub const MASS_TARGET_G: f64 = 80.0;
pub const MASS_OVER_W: f64 = 0.4;
pub const MASS_OVER_K: f64 = 30.0;
pub fn mass_over_penalty(m: f64) -> f64 { let x = ((m - MASS_TARGET_G) / MASS_TARGET_G).max(0.0); MASS_OVER_W * (1.0 - (-MASS_OVER_K * x * x).exp()) }
pub const MARGIN_SCALE: f64 = 0.10;

/// FEASIBLE = every strength / quality requirement is a hard floor: aesthetic pass (incl. C.6/C.7 floors),
/// all hard gates (jimping, holder, ...), FEM floors (180/260/1500 N robust), D1 failure-sequence seq >= 1.2,
/// and chord torsion 1.82 N*m with body AND 4-15 mm tip submodel SF >= 1 — for EVERY genome (P1 or not).
pub fn feasible(r: &Rec) -> bool {
    r.aes.pass && r.rejects.is_empty() && r.fem.as_ref().map_or(false, |f| f.floors_ok && f.seq_ratio >= fem::SEQ_TARGET && fem::twist_floor_ok(f.twist_body_sf, f.twist_tipzone_sf))
}
/// Smooth mass reward: ln(84.8 / m) (0 at the provisional, +0.19 at 70 g); below 70 g the slope drops to 25 %.
pub fn mass_reward(m: f64) -> f64 {
    let m = m.max(1.0);
    if m >= MASS_KNEE_G { (MASS_REF_G / m).ln() } else { (MASS_REF_G / MASS_KNEE_G).ln() + MASS_KNEE_SLOPE * (MASS_KNEE_G / m).ln() }
}
/// Min relative margin over every strength floor (0 at the floor edge).
pub fn strength_margin(r: &Rec) -> f64 {
    r.fem.as_ref().map_or(0.0, |f| [f.lat_rob / 260.0, f.ring_rob / 1500.0, f.seq_ratio / fem::SEQ_TARGET, f.twist_body_sf, f.twist_tipzone_sf]
        .iter().cloned().fold(f64::INFINITY, f64::min) - 1.0).max(0.0)
}
fn knee(x: f64, tau: f64) -> f64 {
    if x <= 0.0 { 0.0 } else { 1.0 - (-x / tau.max(1e-9)).exp() }
}

/// W_CAD_v2 combined fitness weighting for feasible all-gates candidates (70% mass on [92g -> 74g],
/// 12% karambit authenticity, 10% knee-capped survival buffer, 5% knee-capped sequence buffer, 3% carry/fatigue stability).
pub fn w_cad_v2_feasible(r: &Rec) -> f64 {
    let Some(f) = &r.fem else { return -50.0; };
    let lin_m = (92.0 - r.mass_g) / (92.0 - 74.0);
    let log_m = (92.0 / r.mass_g.max(20.0)).ln() / (92.0_f64 / 74.0).ln();
    let s_mass = (0.65 * lin_m + 0.35 * log_m).clamp(0.0, 1.0);
    let s_aes = ((r.aes.score - 0.50) / 0.20).clamp(0.0, 1.0);
    let s_hook = ((r.blade.hook_drop - 25.0) / 10.0).clamp(0.0, 1.0);
    let s_curve = ((r.aes.inside_concave.min(r.aes.spine_convex) - 0.85) / 0.12).clamp(0.0, 1.0);
    let s_karambit = 0.50 * s_aes + 0.25 * s_hook + 0.25 * s_curve;
    let k_hunter = knee(f.lat_rob / 260.0 - 1.0, 0.03);
    let k_twist = knee(f.twist_tipzone_sf - 1.0, 0.03);
    let k_body = knee(f.twist_body_sf - 1.0, 0.03);
    let s_survival = 0.45 * k_hunter + 0.40 * k_twist + 0.15 * k_body;
    let s_seq = knee((f.seq_ratio - fem::SEQ_TARGET) / fem::SEQ_TARGET, 0.025);
    let s_carry = 0.60 * r.novelty.clamp(0.0, 1.0) + 0.40 * knee(f.fat_sf - 1.0, 0.20);
    1.0 + 0.70 * s_mass + 0.12 * s_karambit + 0.10 * s_survival + 0.05 * s_seq + 0.03 * s_carry
}

/// Cap-free fitness (PR-0.31.2, Jepson directive: no hard mass caps).
/// Infeasible: graded constraint-violation penalty, always < 0 (aes -200.., gates -100.., no FEM -50,
/// FEM floors -1-10*deficit, seq/twist (-1, -0.1) graded by the violated ratios).
/// Feasible: under JARAMBIT_ALL_GATES=1, directly evaluates W_CAD_v2 combined fitness in [1.0, 2.0];
/// otherwise F = 1 + ln-mass reward + MARGIN_W*(1 - exp(-margin/0.1)) + 0.02*A + 0.02*min(novelty, 1) (always >= 1).
pub fn fitness(r: &Rec) -> f64 {
    if !r.aes.pass { return -200.0 - r.aes.fails.len() as f64 - r.aes.hausdorff; }
    if !r.rejects.is_empty() { return -100.0 - r.rejects.len() as f64; }
    let ag = all_gates_env();
    match &r.fem { None => -50.0,
        Some(f) if !f.floors_ok => -1.0 - 10.0 * ((1.0 - f.lat_rob / 260.0).max(0.0) + (1.0 - f.ring_rob / 1500.0).max(0.0)),
        Some(f) if f.seq_ratio < fem::SEQ_TARGET || !fem::twist_floor_ok(f.twist_body_sf, f.twist_tipzone_sf) => {
            let q = (f.seq_ratio / fem::SEQ_TARGET).clamp(0.0, 1.0) * f.twist_body_sf.clamp(0.0, 1.0) * f.twist_tipzone_sf.clamp(0.0, 1.0);
            let m_tie = if ag { 0.05 * ((90.0 - r.mass_g) / 20.0).clamp(0.0, 1.0) } else { 0.0 };
            -1.0 + 0.85 * q.min(0.999) + m_tie }
        // through-lattice experiment lane: feasible but too few through-features ranks below every feasible design that has them
        Some(_) if r.n_win + r.n_perf < min_through() => -0.09 + 0.08 * ((r.n_win + r.n_perf) as f64 / min_through() as f64),
        Some(_) => {
            if ag {
                w_cad_v2_feasible(r)
            } else {
                1.0 + mass_reward(r.mass_g) - mass_over_penalty(r.mass_g)
                    + MARGIN_W * (1.0 - (-strength_margin(r) / MARGIN_SCALE).exp())
                    + 0.02 * r.aes.score.clamp(0.0, 1.0) + 0.02 * r.novelty.clamp(0.0, 1.0)
            }
        }
    }
}
/// Back-compat shim: the tip-twist floor is now always hard; the flag is ignored.
pub fn fitness_opt(r: &Rec, _tiptwist: bool) -> f64 { fitness(r) }

/// Total machined volume removed / blank volume: 3-D relief + lattice + fuller + flutes + texture +
/// isogrid + ring flute + pierced-fuller windows (window area x stock).
pub fn relief_frac(r: &Rec) -> f64 { let blank = (r.blank_g / jarambit_physics::material(r.g.mat).density * 1000.0).max(1.0);
    (r.removed.iter().sum::<f64>() + (r.win_area + r.perf_area) * r.t) / blank }
/// MAP-Elites cell = (machined-removal fraction bin, finished-mass bin). Removal bins (`relief_frac`
/// thresholds RB) span 0.5-21 % of the blank; mass bin = floor((mass_g - 72) / 2) clamped to 0..=10
/// (2 g bins from 72 g). CSV columns: `cell_removal,cell_mass_bin`. (Pre-wave7 docs called the second
/// axis a Hausdorff-to-C.4 bin; it has been the mass bin since wave7.)
pub fn qd_cell(r: &Rec) -> (usize, usize) {
    const RB: [f64; 10] = [0.005, 0.01, 0.02, 0.03, 0.045, 0.065, 0.09, 0.12, 0.16, 0.21];
    let f = relief_frac(r);
    (RB.iter().filter(|&&b| f >= b).count(), (((r.mass_g - 72.0) / 2.0).floor().max(0.0) as usize).min(10))
}

pub struct RunCfg { pub pop: usize, pub gens: usize, pub threads: usize, pub seed: u64, pub fem_area: f64, pub t_seed: f64 }

/// Island GA + MAP-Elites/novelty for the internal-machining lane.
pub struct IslandCfg { pub base: RunCfg, pub islands: usize, pub migrate_every: usize, pub migrants: usize, pub seeds: Vec<Genome>, pub kick_frac: f64 }

/// A newborn genome awaiting evaluation: id, parent ids, variation operator.
#[derive(Clone)]
pub struct Birth { pub g: Genome, pub id: usize, pub parents: Vec<usize>, pub op: &'static str, pub island: usize }
fn birth(g: Genome, next_id: &mut usize, parents: Vec<usize>, op: &'static str, island: usize) -> Birth { *next_id += 1; Birth { g, id: *next_id - 1, parents, op, island } }

/// Evaluator: (genome, id, gen, fem_area) -> record. `evaluate` in production; a synthetic one in tests.
pub type EvalFn = dyn Fn(&Genome, usize, usize, f64) -> Rec + Sync;

/// Run counters: fresh evaluations (births) vs appearances (population slots per generation, incl. elites/migrants).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Counters { pub fresh_evals: u64, pub appearances: u64 }

pub fn eval_batch_with(bs: Vec<Birth>, gen: usize, cfg: &RunCfg, ev: &EvalFn) -> Vec<Rec> {
    let n = bs.len(); let th = cfg.threads.max(1).min(n.max(1));
    // Work-stealing (PR-0.31.2): design costs vary ~3x, so static striping left workers idle.
    // Output order is fixed by the sort on id, so results are independent of scheduling.
    let next = std::sync::atomic::AtomicUsize::new(0);
    let mut out: Vec<Rec> = std::thread::scope(|s| {
        let hs: Vec<_> = (0..th).map(|_| s.spawn(|| { let mut v = Vec::new(); loop {
            let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed); if i >= n { break v; }
            let b = &bs[i]; let mut r = ev(&b.g, b.id, gen, cfg.fem_area); r.parents = b.parents.clone(); r.op = b.op; r.op_version = OP_VERSION; v.push(r); } })).collect();
        hs.into_iter().flat_map(|h| h.join().unwrap()).collect()
    });
    out.sort_by_key(|r| r.id); out
}
pub fn eval_batch(gs: Vec<(Genome, usize)>, gen: usize, cfg: &RunCfg) -> Vec<Rec> {
    eval_batch_with(gs.into_iter().map(|(g, id)| Birth { g, id, parents: Vec::new(), op: "eval", island: 0 }).collect(), gen, cfg, &evaluate)
}

fn eval_event(r: &Rec, island: usize) -> Event {
    let f = |x: f64| if x.is_finite() { Some(x) } else { None };
    Event::Eval { gen: r.gen, island, id: r.id as u64, parents: r.parents.iter().map(|&p| p as u64).collect(), op: r.op.to_string(), op_version: r.op_version,
        fit: f(r.fit), aes_pass: r.aes.pass, legal: r.legal(), mass_g: f(r.mass_g), fem_status: format!("{:?}", crate::record::fem_status(r)).to_lowercase() }
}

pub struct RunOut { pub all: Vec<Rec>, pub archive: std::collections::BTreeMap<(usize, usize), Rec>, pub history: Vec<String>, pub counters: Counters }

pub fn run(cfg: &RunCfg, log: impl FnMut(&str)) -> RunOut { run_with(cfg, &evaluate, log, &mut NullSink) }

/// Single-population GA + MAP-Elites with lineage + evidence events (PR-0.29.1).
pub fn run_with(cfg: &RunCfg, ev: &EvalFn, log: impl FnMut(&str), sink: &mut dyn Sink) -> RunOut {
    run_observed(cfg, ev, log, sink, &mut |_| Ok(())).expect("infallible observation")
}

/// Observe each fresh evaluated batch once, before any selection or population reuse.
/// An observer error stops collection explicitly; it cannot change a solver result.
pub fn run_observed(cfg: &RunCfg, ev: &EvalFn, mut log: impl FnMut(&str), sink: &mut dyn Sink, observe: &mut dyn FnMut(&[Rec]) -> Result<(), String>) -> Result<RunOut, String> {
    let mut rng = Rng(cfg.seed);
    let seed = Genome::c4_seed(cfg.t_seed);
    let mut next_id = 0usize; let mut cnt = Counters::default();
    let mut init = vec![birth(seed.clone(), &mut next_id, vec![], "seed", 0)];
    while init.len() < cfg.pop { let s = if init.len() % 4 == 0 { 0.25 } else { 0.08 }; let g = seed.mutate(&mut rng, s); init.push(birth(g, &mut next_id, vec![0], "init_mutate", 0)); }
    cnt.fresh_evals += init.len() as u64;
    let mut pop = eval_batch_with(init, 0, cfg, ev);
    observe(&pop)?;
    let mut all: Vec<Rec> = Vec::new();
    let mut nov_mem: Vec<[f64; DESC_N]> = Vec::new();
    let mut archive: std::collections::BTreeMap<(usize, usize), Rec> = std::collections::BTreeMap::new();
    let mut history = vec!["gen,evals,aesthetic_pass,legal,best_fit,best_spg,best_mass_g,best_aes,best_nov,archive_cells,mean_hausdorff_pass".to_string()];
    let mut fresh_ids: std::collections::HashSet<usize> = pop.iter().map(|r| r.id).collect();
    for gen in 0..=cfg.gens {
        // novelty among aesthetic passers only (archive memory + current passers)
        let pass_desc: Vec<[f64; DESC_N]> = pop.iter().filter(|r| r.aes.pass).map(|r| r.desc).collect();
        let mut ctx = nov_mem.clone(); ctx.extend(pass_desc.iter().copied());
        for r in pop.iter_mut() { r.novelty = if r.aes.pass { novelty(&r.desc, &ctx, 5) } else { 0.0 }; r.fit = fitness(r); }
        for r in pop.iter() { if r.aes.pass { nov_mem.push(r.desc); }
            if r.legal() { let c = qd_cell(r); if archive.get(&c).map_or(true, |e| r.fit > e.fit) { archive.insert(c, r.clone()); } } }
        if nov_mem.len() > 4000 { let k = nov_mem.len() - 4000; nov_mem.drain(..k); }
        all.extend(pop.iter().cloned());
        cnt.appearances += pop.len() as u64;
        for r in pop.iter() { if fresh_ids.contains(&r.id) { sink.event(eval_event(r, 0)); } else { sink.event(Event::Elite { gen, island: 0, id: r.id as u64 }); } }
        let np = pop.iter().filter(|r| r.aes.pass).count(); let nl = pop.iter().filter(|r| r.legal()).count();
        let best = pop.iter().max_by(|a, b| a.fit.partial_cmp(&b.fit).unwrap()).unwrap();
        let mh = { let v: Vec<f64> = pop.iter().filter(|r| r.aes.pass).map(|r| r.aes.hausdorff).collect(); v.iter().sum::<f64>() / v.len().max(1) as f64 };
        let line = format!("{gen},{},{np},{nl},{:.4},{:.4},{:.2},{:.3},{:.3},{},{:.2}", pop.len(), best.fit, best.spg, best.mass_g, best.aes.score, best.novelty, archive.len(), mh);
        sink.event(Event::GenEnd { gen, fresh_evals: cnt.fresh_evals, appearances: cnt.appearances, archive_cells: archive.len(), best_id: Some(best.id as u64), best_fit: best.fit.is_finite().then_some(best.fit) });
        sink.end_gen();
        log(&line); history.push(line);
        if gen == cfg.gens { break; }
        // ---- next generation: elitism + tournament + crossover + envelope mutation + archive draws ----
        let mut sorted = pop.clone(); sorted.sort_by(|a, b| b.fit.partial_cmp(&a.fit).unwrap());
        let mut kids: Vec<Birth> = Vec::new();
        let n_el = 4usize.min(sorted.len());
        let arch: Vec<(Genome, usize)> = archive.values().map(|r| (r.g.clone(), r.id)).collect();
        let tour = |rng: &mut Rng| { let a = &sorted[(rng.f64() * sorted.len() as f64) as usize]; let b = &sorted[(rng.f64() * sorted.len() as f64) as usize]; if a.fit >= b.fit { (a.g.clone(), a.id) } else { (b.g.clone(), b.id) } };
        while kids.len() < cfg.pop - n_el {
            let (pa, ia, from_arch) = if !arch.is_empty() && rng.f64() < 0.25 { let x = &arch[(rng.f64() * arch.len() as f64) as usize]; (x.0.clone(), x.1, true) } else { let (g, i) = tour(&mut rng); (g, i, false) };
            let b = if rng.f64() < 0.5 { let (pb, ib) = tour(&mut rng); birth(pa.crossover(&pb, &mut rng).mutate(&mut rng, 0.06), &mut next_id, vec![ia, ib], if from_arch { "archive_crossover_mutate" } else { "crossover_mutate" }, 0) }
                    else { birth(pa.mutate(&mut rng, 0.1), &mut next_id, vec![ia], if from_arch { "archive_mutate" } else { "mutate" }, 0) };
            kids.push(b);
        }
        cnt.fresh_evals += kids.len() as u64;
        let mut newpop: Vec<Rec> = sorted.into_iter().take(n_el).map(|mut r| { r.gen = gen + 1; r }).collect();
        let born = eval_batch_with(kids, gen + 1, cfg, ev);
        observe(&born)?;
        fresh_ids = born.iter().map(|r| r.id).collect();
        newpop.extend(born);
        pop = newpop;
    }
    sink.event(Event::RunEnd { fresh_evals: cnt.fresh_evals, appearances: cnt.appearances }); sink.end_gen();
    Ok(RunOut { all, archive, history, counters: cnt })
}


/// Island GA (ring migration) with a SHARED novelty memory and MAP-Elites archive. Islands differ in
/// mutation pressure (exploit .. explore). Seeds: certified crescents + C.4; a `kick_frac` share of
/// each initial island gets the internal-machining layer switched on at random.
pub fn run_islands(cfg: &IslandCfg, log: impl FnMut(&str), snap: impl FnMut(usize, &[Rec], &std::collections::BTreeMap<(usize, usize), Rec>)) -> RunOut {
    run_islands_with(cfg, &evaluate, log, snap, &mut NullSink)
}

pub fn run_islands_with(cfg: &IslandCfg, ev: &EvalFn, log: impl FnMut(&str), snap: impl FnMut(usize, &[Rec], &std::collections::BTreeMap<(usize, usize), Rec>), sink: &mut dyn Sink) -> RunOut {
    run_islands_observed(cfg, ev, log, snap, sink, &mut |_| Ok(())).expect("infallible observation")
}

/// Same search with a durable observation hook, independent of scoring and RNG.
pub fn run_islands_observed(cfg: &IslandCfg, ev: &EvalFn, mut log: impl FnMut(&str), mut snap: impl FnMut(usize, &[Rec], &std::collections::BTreeMap<(usize, usize), Rec>), sink: &mut dyn Sink, observe: &mut dyn FnMut(&[Rec]) -> Result<(), String>) -> Result<RunOut, String> {
    let b = &cfg.base; let k = cfg.islands.max(1);
    let mut rng = Rng(b.seed);
    let mut seeds = cfg.seeds.clone(); if seeds.is_empty() { seeds.push(Genome::c4_seed(b.t_seed)); }
    let mut next_id = 0usize; let mut cnt = Counters::default();
    let sig_scale: f64 = std::env::var("JARAMBIT_SIGMA_SCALE").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0);
    let mut init: Vec<Birth> = Vec::new();
    for isl in 0..k { for j in 0..b.pop {
        let s = &seeds[(isl + j) % seeds.len()];
        let (g, op) = if j < seeds.len().min(4) { (s.clone(), "seed") } else if rng.f64() < cfg.kick_frac { (s.machining_kick(&mut rng).mutate(&mut rng, 0.05 * sig_scale), "seed_kick_mutate") } else { (s.mutate(&mut rng, (if j % 4 == 0 { 0.2 } else { 0.08 }) * sig_scale), "seed_mutate") };
        init.push(birth(g, &mut next_id, vec![], op, isl)); } }
    cnt.fresh_evals += init.len() as u64;
    let ev0 = eval_batch_with(init, 0, b, ev);
    observe(&ev0)?;
    let mut pops: Vec<Vec<Rec>> = ev0.chunks(b.pop).map(|c| c.to_vec()).collect();
    let sig: Vec<f64> = (0..k).map(|i| (0.05 + 0.12 * i as f64 / (k.max(2) - 1) as f64) * sig_scale).collect();
    let mut all: Vec<Rec> = Vec::new();
    let mut nov_mem: Vec<[f64; DESC_N]> = Vec::new();
    let mut archive: std::collections::BTreeMap<(usize, usize), Rec> = std::collections::BTreeMap::new();
    let mut history = vec!["gen,island,evals,aesthetic_pass,legal,feasible,best_fit,best_spg,best_mass_g,best_aes,best_nov,archive_cells,min_legal_mass_g,mean_relief_frac_pass".to_string()];
    let mut fresh_ids: std::collections::HashSet<usize> = pops.iter().flatten().map(|r| r.id).collect();
    for gen in 0..=b.gens {
        let mut ctx = nov_mem.clone();
        for p in &pops { ctx.extend(p.iter().filter(|r| r.aes.pass).map(|r| r.desc)); }
        for p in pops.iter_mut() { for r in p.iter_mut() { r.novelty = if r.aes.pass { novelty(&r.desc, &ctx, 5) } else { 0.0 }; r.fit = fitness(r); } }
        let mut gbest: Option<(usize, f64)> = None;
        for (isl, p) in pops.iter().enumerate() {
            cnt.appearances += p.len() as u64;
            for r in p.iter() { if r.gen == gen || gen == 0 { if r.aes.pass { nov_mem.push(r.desc); } all.push(r.clone()); }
                if fresh_ids.contains(&r.id) { sink.event(eval_event(r, isl)); } else { sink.event(Event::Elite { gen, island: isl, id: r.id as u64 }); }
                if r.legal() { let c = qd_cell(r); if archive.get(&c).map_or(true, |e| r.fit > e.fit) { archive.insert(c, r.clone()); } } }
            let np = p.iter().filter(|r| r.aes.pass).count(); let nl = p.iter().filter(|r| r.legal()).count(); let nu = p.iter().filter(|r| feasible(r)).count();
            let best = p.iter().max_by(|a, b| a.fit.partial_cmp(&b.fit).unwrap()).unwrap();
            if gbest.map_or(true, |(_, f)| best.fit > f) { gbest = Some((best.id, best.fit)); }
            let mlm = p.iter().filter(|r| r.legal()).map(|r| r.mass_g).fold(f64::NAN, f64::min);
            let mrf = { let v: Vec<f64> = p.iter().filter(|r| r.aes.pass).map(relief_frac).collect(); v.iter().sum::<f64>() / v.len().max(1) as f64 };
            let line = format!("{gen},{isl},{},{np},{nl},{nu},{:.4},{:.4},{:.2},{:.3},{:.3},{},{:.2},{:.4}", p.len(), best.fit, best.spg, best.mass_g, best.aes.score, best.novelty, archive.len(), mlm, mrf);
            log(&line); history.push(line);
        }
        if nov_mem.len() > 6000 { let d = nov_mem.len() - 6000; nov_mem.drain(..d); }
        snap(gen, &all, &archive);
        if gen == b.gens {
            sink.event(Event::GenEnd { gen, fresh_evals: cnt.fresh_evals, appearances: cnt.appearances, archive_cells: archive.len(), best_id: gbest.map(|x| x.0 as u64), best_fit: gbest.and_then(|x| x.1.is_finite().then_some(x.1)) });
            sink.end_gen(); break;
        }
        // ---- migration (ring): best `migrants` of island i replace the worst of island i+1 ----
        if k > 1 && cfg.migrate_every > 0 && gen > 0 && gen % cfg.migrate_every == 0 {
            let best: Vec<Vec<Rec>> = pops.iter().map(|p| { let mut s = p.clone(); s.sort_by(|a, b| b.fit.partial_cmp(&a.fit).unwrap()); s.into_iter().take(cfg.migrants).collect() }).collect();
            for i in 0..k { let to = (i + 1) % k; let p = &mut pops[to]; p.sort_by(|a, b| b.fit.partial_cmp(&a.fit).unwrap());
                let n = p.len(); let mut moved = Vec::new(); for (j, m) in best[i].iter().enumerate() { if j < n { p[n - 1 - j] = m.clone(); moved.push(m.id as u64); } }
                sink.event(Event::Migration { gen, from: i, to, ids: moved }); }
        }
        sink.event(Event::GenEnd { gen, fresh_evals: cnt.fresh_evals, appearances: cnt.appearances, archive_cells: archive.len(), best_id: gbest.map(|x| x.0 as u64), best_fit: gbest.and_then(|x| x.1.is_finite().then_some(x.1)) });
        sink.end_gen();
        let arch: Vec<(Genome, usize)> = archive.values().map(|r| (r.g.clone(), r.id)).collect();
        let mut kids: Vec<Birth> = Vec::new(); let mut keep: Vec<Vec<Rec>> = Vec::new();
        for (isl, p) in pops.iter().enumerate() {
            let mut sorted = p.clone(); sorted.sort_by(|a, b| b.fit.partial_cmp(&a.fit).unwrap());
            let ne = 3usize.min(sorted.len());
            keep.push(sorted.iter().take(ne).cloned().collect());
            let tour = |rng: &mut Rng| { let a = &sorted[(rng.f64() * sorted.len() as f64) as usize]; let c = &sorted[(rng.f64() * sorted.len() as f64) as usize]; if a.fit >= c.fit { (a.g.clone(), a.id) } else { (c.g.clone(), c.id) } };
            for _ in 0..b.pop - ne {
                let (pa, ia, from_arch) = if !arch.is_empty() && rng.f64() < 0.3 { let x = &arch[(rng.f64() * arch.len() as f64) as usize]; (x.0.clone(), x.1, true) } else { let (g, i) = tour(&mut rng); (g, i, false) };
                let (mut child, mut parents, mut op) = if rng.f64() < 0.5 { let (pb, ib) = tour(&mut rng); (pa.crossover(&pb, &mut rng).mutate(&mut rng, sig[isl] * 0.6), vec![ia, ib], if from_arch { "archive_crossover_mutate" } else { "crossover_mutate" }) }
                    else { (pa.mutate(&mut rng, sig[isl]), vec![ia], if from_arch { "archive_mutate" } else { "mutate" }) };
                if cfg.kick_frac > 0.0 && rng.f64() < 0.04 { child = child.machining_kick(&mut rng); op = match op { "crossover_mutate" => "crossover_mutate_kick", "archive_crossover_mutate" => "archive_crossover_mutate_kick", "archive_mutate" => "archive_mutate_kick", _ => "mutate_kick" }; }
                parents.dedup();
                kids.push(birth(child, &mut next_id, std::mem::take(&mut parents), op, isl));
            }
        }
        cnt.fresh_evals += kids.len() as u64;
        let born = eval_batch_with(kids, gen + 1, b, ev);
        observe(&born)?;
        fresh_ids = born.iter().map(|r| r.id).collect();
        let mut it = born.into_iter();
        for (isl, p) in pops.iter_mut().enumerate() { let mut np = keep[isl].clone(); while np.len() < b.pop { if let Some(r) = it.next() { np.push(r); } else { break; } } *p = np; }
    }
    sink.event(Event::RunEnd { fresh_evals: cnt.fresh_evals, appearances: cnt.appearances }); sink.end_gen();
    Ok(RunOut { all, archive, history, counters: cnt })
}
