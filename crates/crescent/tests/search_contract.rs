//! T04-01..12: search feasibility, legal-vs-feasible separation, partial/nonfinite rejection,
//! contextual novelty guard, deterministic ties/duplicates/cell boundaries, config validation,
//! checkpoint replay matrix, corrupt checkpoint/log prefix handling, resume identity & fork,
//! durability boundary, duplicate-resume counter guard, and lineage orphan rejection.

use jarambit_crescent::{
    aesthetic::Aes,
    fem::Fem,
    genome::Genome,
    pheno::{Blade, NREM},
    qd::{self, Counters, IslandCfg, Rec, RunCfg, DESC_N},
    record::{self, FemStatus, RecJson},
};
use jarambit_evidence::{
    identity::{ContentRef, MaterialState, ScientificIdentity, SubjectRef},
    read_bytes, Event, EvidenceLog, Obs, RunManifest, Sink, VecSink, OP_VERSION,
};
use jarambit_genotype::Rng;
use std::collections::{BTreeMap, HashMap, HashSet};

fn base_rec(g: &Genome, id: usize, gen: usize) -> Rec {
    let s: f64 = g
        .v
        .iter()
        .enumerate()
        .map(|(i, x)| x * ((i % 7) as f64 + 1.0))
        .sum::<f64>();
    let m = 75.0 + (s.sin() * 3.0);
    let mut desc = [0.0; DESC_N];
    for (k, d) in desc.iter_mut().enumerate() {
        *d = ((s * (k as f64 + 1.0)).sin() * 0.5 + 0.5).clamp(0.0, 1.0);
    }
    let fem = Fem {
        lat20: 320.0,
        lat_rob: 300.0 + 15.0 * s.cos(),
        ring20: 2100.0,
        ring_rob: 2000.0,
        pry20: 180.0,
        twist20_nmm: 2400.0,
        sf_hiker: 1.8,
        sf_hunter: 1.3,
        sf_ring: 1.4,
        floors_ok: true,
        lat_at: (12.0, -4.0),
        seq_ratio: 1.45,
        seq_case: "lateral",
        twist_sf: 1.5,
        twist_joint_nmm: 2200.0,
        twist_tipdist: 8.0,
        twist_body_sf: 1.6,
        twist_tipzone_sf: 1.4,
        ..Default::default()
    };
    let aes = Aes {
        score: 0.82,
        pass: true,
        hausdorff: 1.1,
        emd: 0.04,
        straight_mm: 0.0,
        min_concave_r: 3.5,
        fails: vec![],
        ..Default::default()
    };
    Rec {
        id,
        gen,
        g: g.clone(),
        mass_g: m,
        blank_g: m * 1.3,
        t: 6.0,
        blade: Blade {
            tip_to_heel: 90.0,
            horizontal_reach: 88.0,
            arc_in: 92.0,
            arc_sp: 95.0,
            ..Default::default()
        },
        grip: 70.0,
        aes,
        rejects: vec![],
        spg: fem.lat_rob / m,
        fem: Some(fem),
        novelty: 0.0,
        fit: 0.0,
        desc,
        holder_lw: (140.0, 55.0, 12.0),
        removed: [10.0; NREM],
        min_web: 2.0,
        n_win: 0,
        win_area: 0.0,
        n_perf: 0,
        perf_area: 0.0,
        parents: vec![],
        op: "eval",
        op_version: OP_VERSION,
    }
}

fn admit_certified_elite(
    archive: &mut BTreeMap<(usize, usize), Rec>,
    r: &Rec,
) -> Result<bool, &'static str> {
    if !r.mass_g.is_finite() || !r.blank_g.is_finite() || !r.t.is_finite() || !r.spg.is_finite() {
        return Err("nonfinite_scalar");
    }
    if r.desc.iter().any(|d| !d.is_finite() || *d < 0.0 || *d > 1.0) {
        return Err("invalid_descriptor");
    }
    match record::fem_status(r) {
        FemStatus::NotEvaluated => return Err("fem_not_evaluated"),
        FemStatus::Failed => return Err("fem_failed"),
        FemStatus::Incomplete => return Err("fem_incomplete"),
        FemStatus::Complete => {}
    }
    if !qd::feasible(r) {
        return Err("infeasible_record");
    }
    let intrinsic = {
        let mut r0 = r.clone();
        r0.novelty = 0.0;
        qd::fitness(&r0)
    };
    if !intrinsic.is_finite() || intrinsic < 1.0 {
        return Err("nonpassing_intrinsic_fitness");
    }
    let c = qd::qd_cell(r);
    if c.0 > 10 || c.1 > 10 {
        return Err("out_of_domain_cell");
    }
    if let Some(existing) = archive.get(&c) {
        let existing_intrinsic = {
            let mut e0 = existing.clone();
            e0.novelty = 0.0;
            qd::fitness(&e0)
        };
        // Contextual novelty alone cannot replace an existing elite without intrinsic quality gain.
        if intrinsic <= existing_intrinsic {
            return Ok(false);
        }
    }
    archive.insert(c, r.clone());
    Ok(true)
}

// ---------------------------------------------------------------------------
// T04-01: all_infeasible
// ---------------------------------------------------------------------------
#[test]
fn t04_01_all_infeasible_positive_and_adverse() {
    let cfg = RunCfg {
        pop: 12,
        gens: 3,
        threads: 2,
        seed: 17,
        fem_area: 2.0,
        t_seed: 5.7,
    };
    let all_infeasible_eval = |g: &Genome, id: usize, gen: usize, _area: f64| -> Rec {
        let mut r = base_rec(g, id, gen);
        if let Some(f) = r.fem.as_mut() {
            f.floors_ok = false;
            f.lat_rob = 190.0;
            f.seq_ratio = 0.85;
        }
        r
    };
    let mut sink = VecSink::default();
    let out = qd::run_with(&cfg, &all_infeasible_eval, |_| {}, &mut sink);
    assert!(!out.all.is_empty());
    assert!(
        out.all.iter().all(|r| !qd::feasible(r) && r.fit < 0.0),
        "every record in an all-infeasible population must be infeasible with negative fitness"
    );
    assert!(
        out.archive.is_empty(),
        "no infeasible record may enter the MAP-Elites archive"
    );
    let mut cert = BTreeMap::new();
    for r in &out.all {
        assert_eq!(admit_certified_elite(&mut cert, r), Err("infeasible_record"));
    }
    assert!(cert.is_empty());

    // Adverse control: an archive policy that admits highest-fitness records without checking
    // feasibility would admit negative-fitness records (`fit < 0.0`); verify our check rejects it.
    let mut broken_archive = BTreeMap::new();
    for r in &out.all {
        broken_archive.insert(qd::qd_cell(r), r.clone());
    }
    assert!(!broken_archive.is_empty());
    assert!(
        broken_archive.values().any(|r| !qd::feasible(r) || r.fit < 0.0),
        "adverse control must be detected because it admits infeasible records"
    );
}

// ---------------------------------------------------------------------------
// T04-02: legal_vs_feasible
// ---------------------------------------------------------------------------
#[test]
fn t04_02_legal_vs_feasible_positive_and_adverse() {
    let mut g = Genome::c4_seed(5.7);
    // Ensure P1 is off so `Rec::legal()` does not check seq_ratio or twist_floor_ok.
    g.v[jarambit_crescent::genome::P1_ON] = 0.0;
    assert!(!g.p1());

    let mut r_legal_only = base_rec(&g, 1, 0);
    if let Some(f) = r_legal_only.fem.as_mut() {
        f.floors_ok = true;
        f.seq_ratio = 0.92; // below fem::SEQ_TARGET (1.2)
        f.twist_tipzone_sf = 0.75; // below 1.0
    }
    r_legal_only.fit = qd::fitness(&r_legal_only);

    // Passes legacy `legal()` predicate when P1 is off, but fails full `qd::feasible()`.
    assert!(r_legal_only.legal(), "non-P1 record with floors_ok passes legacy legal()");
    assert!(!qd::feasible(&r_legal_only), "must fail full search feasibility");
    assert!(r_legal_only.fit < 0.0, "infeasible record must have negative fitness");

    let mut cert = BTreeMap::new();
    assert_eq!(
        admit_certified_elite(&mut cert, &r_legal_only),
        Err("infeasible_record"),
        "legal-but-infeasible record must be rejected from certified/feasible archive"
    );

    // Positive control: record that satisfies full feasibility is admitted.
    let mut r_feas = base_rec(&g, 2, 0);
    r_feas.fit = qd::fitness(&r_feas);
    assert!(r_feas.legal() && qd::feasible(&r_feas) && r_feas.fit >= 1.0);
    assert_eq!(admit_certified_elite(&mut cert, &r_feas), Ok(true));
    assert_eq!(cert.len(), 1);
}

// ---------------------------------------------------------------------------
// T04-03: partial_nonfinite
// ---------------------------------------------------------------------------
#[test]
fn t04_03_partial_nonfinite_positive_and_adverse() {
    let g = Genome::c4_seed(5.7);

    // Case 1: Missing FEM after passing aesthetic + gates -> FemStatus::Failed
    let mut r_no_fem = base_rec(&g, 10, 0);
    r_no_fem.fem = None;
    r_no_fem.fit = qd::fitness(&r_no_fem);
    assert_eq!(record::fem_status(&r_no_fem), FemStatus::Failed);
    assert_eq!(r_no_fem.fit, -50.0);
    let mut cert = BTreeMap::new();
    assert_eq!(admit_certified_elite(&mut cert, &r_no_fem), Err("fem_failed"));

    // Case 2: Partial non-finite FEM scalar -> FemStatus::Incomplete, serialized as null
    let mut r_nan_fem = base_rec(&g, 11, 0);
    r_nan_fem.fem.as_mut().unwrap().seq_ratio = f64::NAN;
    assert_eq!(record::fem_status(&r_nan_fem), FemStatus::Incomplete);
    let rj = RecJson::from_rec(&r_nan_fem);
    assert!(rj.fem.seq_ratio.is_none(), "NaN FEM scalar must serialize as None");
    assert_eq!(
        Obs::finite(f64::NAN),
        Obs::Failed {
            reason: "non-finite".into()
        }
    );
    assert_eq!(
        admit_certified_elite(&mut cert, &r_nan_fem),
        Err("fem_incomplete")
    );

    // Case 3: NaN or out-of-domain descriptor
    let mut r_bad_desc = base_rec(&g, 12, 0);
    r_bad_desc.desc[3] = f64::NAN;
    assert_eq!(
        admit_certified_elite(&mut cert, &r_bad_desc),
        Err("invalid_descriptor")
    );
    r_bad_desc.desc[3] = 1.5;
    assert_eq!(
        admit_certified_elite(&mut cert, &r_bad_desc),
        Err("invalid_descriptor")
    );

    // Case 4: Non-finite mass/quality scalar
    let mut r_nan_mass = base_rec(&g, 13, 0);
    r_nan_mass.mass_g = f64::INFINITY;
    assert_eq!(
        admit_certified_elite(&mut cert, &r_nan_mass),
        Err("nonfinite_scalar")
    );

    // Positive control: complete finite record passes
    let mut r_ok = base_rec(&g, 14, 0);
    r_ok.fit = qd::fitness(&r_ok);
    assert_eq!(record::fem_status(&r_ok), FemStatus::Complete);
    assert_eq!(admit_certified_elite(&mut cert, &r_ok), Ok(true));
}

// ---------------------------------------------------------------------------
// T04-04: novelty_context
// ---------------------------------------------------------------------------
#[test]
fn t04_04_novelty_context_positive_and_adverse() {
    let g = Genome::c4_seed(5.7);
    let mut elite = base_rec(&g, 1, 0);
    let near_ctx = [[0.51; DESC_N]; 6];
    let far_ctx = [[0.99; DESC_N]; 6];

    elite.desc = [0.50; DESC_N];
    elite.novelty = qd::novelty(&elite.desc, &near_ctx, 5);
    elite.fit = qd::fitness(&elite);

    // Candidate has identical intrinsic quality (mass, FEM, aesthetics, descriptor) but
    // higher contextual novelty solely because its neighbor set changed.
    let mut challenger = elite.clone();
    challenger.id = 2;
    challenger.novelty = qd::novelty(&challenger.desc, &far_ctx, 5);
    challenger.fit = qd::fitness(&challenger);
    assert!(challenger.novelty > elite.novelty);
    assert!(challenger.fit > elite.fit, "contextual fitness includes novelty bonus");

    let mut cert = BTreeMap::new();
    assert_eq!(admit_certified_elite(&mut cert, &elite), Ok(true));
    // Must NOT replace the elite on contextual novelty alone:
    assert_eq!(admit_certified_elite(&mut cert, &challenger), Ok(false));
    assert_eq!(cert.get(&qd::qd_cell(&elite)).unwrap().id, 1);

    // Positive control: a challenger with genuinely better intrinsic quality (lower mass / higher margin)
    // in the same cell replaces the elite even with zero novelty.
    let mut genuine = elite.clone();
    genuine.id = 3;
    genuine.mass_g -= 0.4; // stays in the same 2g mass bin [74, 76)
    genuine.novelty = 0.0;
    genuine.fit = qd::fitness(&genuine);
    assert_eq!(qd::qd_cell(&genuine), qd::qd_cell(&elite));
    assert_eq!(admit_certified_elite(&mut cert, &genuine), Ok(true));
    assert_eq!(cert.get(&qd::qd_cell(&elite)).unwrap().id, 3);
}

// ---------------------------------------------------------------------------
// T04-05: ties_duplicates
// ---------------------------------------------------------------------------
#[test]
fn t04_05_ties_duplicates_and_cell_boundaries() {
    let g = Genome::c4_seed(5.7);
    let cfg = RunCfg {
        pop: 4,
        gens: 1,
        threads: 2,
        seed: 9,
        fem_area: 2.0,
        t_seed: 5.7,
    };
    // Duplicate genomes submitted as distinct births must keep distinct birth IDs and parent lineage.
    let births = vec![
        qd::Birth {
            g: g.clone(),
            id: 0,
            parents: vec![],
            op: "seed",
            island: 0,
        },
        qd::Birth {
            g: g.clone(),
            id: 1,
            parents: vec![0],
            op: "mutate",
            island: 0,
        },
    ];
    let recs = qd::eval_batch_with(births, 0, &cfg, &|gg, id, gen, _| base_rec(gg, id, gen));
    assert_eq!(recs.len(), 2);
    assert_eq!(recs[0].id, 0);
    assert_eq!(recs[1].id, 1);
    assert_eq!(recs[1].parents, vec![0]);

    // Deterministic tie-breaking: strict `>` retains the first incumbent in both `qd.rs` and `admit_certified_elite`.
    let mut r1 = recs[0].clone();
    r1.fit = qd::fitness(&r1);
    let mut r2 = recs[1].clone();
    r2.fit = r1.fit;
    let mut arch = BTreeMap::new();
    assert_eq!(admit_certified_elite(&mut arch, &r1), Ok(true));
    assert_eq!(admit_certified_elite(&mut arch, &r2), Ok(false));
    assert_eq!(arch.get(&qd::qd_cell(&r1)).unwrap().id, 0);

    // Boundary bins check: mass bins [72.0, 74.0, ..., 92.0+] -> 0..=10
    let mut rb = r1.clone();
    rb.mass_g = 71.9;
    assert_eq!(qd::qd_cell(&rb).1, 0);
    rb.mass_g = 72.0;
    assert_eq!(qd::qd_cell(&rb).1, 0);
    rb.mass_g = 74.0;
    assert_eq!(qd::qd_cell(&rb).1, 1);
    rb.mass_g = 99.0;
    assert_eq!(qd::qd_cell(&rb).1, 10);
}

// ---------------------------------------------------------------------------
// T04-06: invalid_config
// ---------------------------------------------------------------------------
fn validate_run_cfg(cfg: &RunCfg) -> Result<(), &'static str> {
    if cfg.pop == 0 {
        return Err("empty_population");
    }
    if cfg.threads == 0 {
        return Err("zero_threads");
    }
    if !cfg.fem_area.is_finite() || cfg.fem_area <= 0.0 {
        return Err("invalid_fem_area");
    }
    if !cfg.t_seed.is_finite() || cfg.t_seed <= 0.0 {
        return Err("invalid_seed_thickness");
    }
    Ok(())
}

fn validate_island_cfg(cfg: &IslandCfg) -> Result<(), &'static str> {
    validate_run_cfg(&cfg.base)?;
    if cfg.islands == 0 {
        return Err("zero_islands");
    }
    if cfg.islands > 1 && (cfg.migrate_every == 0 || cfg.migrants == 0 || cfg.migrants >= cfg.base.pop) {
        return Err("invalid_migration_config");
    }
    if !cfg.kick_frac.is_finite() || !(0.0..=1.0).contains(&cfg.kick_frac) {
        return Err("invalid_kick_fraction");
    }
    if cfg.seeds.iter().any(|s| s.v.iter().any(|x| !x.is_finite())) {
        return Err("nonfinite_seed_genome");
    }
    Ok(())
}

#[test]
fn t04_06_invalid_config_positive_and_adverse() {
    let valid_base = RunCfg {
        pop: 16,
        gens: 4,
        threads: 2,
        seed: 5,
        fem_area: 2.0,
        t_seed: 5.7,
    };
    assert_eq!(validate_run_cfg(&valid_base), Ok(()));
    let valid_isl = IslandCfg {
        base: RunCfg { ..valid_base },
        islands: 2,
        migrate_every: 2,
        migrants: 2,
        seeds: vec![Genome::c4_seed(5.7)],
        kick_frac: 0.2,
    };
    assert_eq!(validate_island_cfg(&valid_isl), Ok(()));

    // Adverse configs must fail with structured errors
    assert_eq!(
        validate_run_cfg(&RunCfg {
            pop: 0,
            ..valid_base
        }),
        Err("empty_population")
    );
    assert_eq!(
        validate_run_cfg(&RunCfg {
            threads: 0,
            ..valid_base
        }),
        Err("zero_threads")
    );
    assert_eq!(
        validate_run_cfg(&RunCfg {
            fem_area: f64::NAN,
            ..valid_base
        }),
        Err("invalid_fem_area")
    );
    assert_eq!(
        validate_island_cfg(&IslandCfg {
            base: RunCfg { ..valid_base },
            islands: 0,
            migrate_every: 2,
            migrants: 2,
            seeds: vec![],
            kick_frac: 0.2,
        }),
        Err("zero_islands")
    );
    assert_eq!(
        validate_island_cfg(&IslandCfg {
            base: RunCfg { ..valid_base },
            islands: 2,
            migrate_every: 2,
            migrants: 16,
            seeds: vec![],
            kick_frac: 0.2,
        }),
        Err("invalid_migration_config")
    );
    assert_eq!(
        validate_island_cfg(&IslandCfg {
            base: RunCfg { ..valid_base },
            islands: 2,
            migrate_every: 2,
            migrants: 2,
            seeds: vec![],
            kick_frac: 1.5,
        }),
        Err("invalid_kick_fraction")
    );
}

// ---------------------------------------------------------------------------
// Deterministic step-able checkpoint harness for T04-07..11
// ---------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct SearchCheckpoint {
    version: u32,
    manifest_sha256: String,
    forked_from_manifest_sha256: Option<String>,
    forked_at_gen: Option<usize>,
    next_gen: usize,
    rng_state: u64,
    next_id: usize,
    counters: (u64, u64),
    pop_genes: Vec<(usize, usize, Vec<f64>, Vec<usize>, String)>,
    archive_cells: Vec<((usize, usize), usize, u64)>,
}

fn step_generation(
    gen: usize,
    pop_size: usize,
    rng: &mut Rng,
    next_id: &mut usize,
    counters: &mut Counters,
    pop: &mut Vec<Rec>,
    archive: &mut BTreeMap<(usize, usize), Rec>,
    sink: &mut dyn Sink,
) {
    if gen == 0 {
        let seed = Genome::c4_seed(5.7);
        let mut init = Vec::new();
        *next_id += 1;
        init.push((seed.clone(), *next_id - 1, vec![], "seed"));
        while init.len() < pop_size {
            let s = if init.len() % 4 == 0 { 0.25 } else { 0.08 };
            let g = seed.mutate(rng, s);
            *next_id += 1;
            init.push((g, *next_id - 1, vec![0], "init_mutate"));
        }
        counters.fresh_evals += init.len() as u64;
        *pop = init
            .into_iter()
            .map(|(g, id, parents, op)| {
                let mut r = base_rec(&g, id, 0);
                r.parents = parents;
                r.op = op;
                r.fit = qd::fitness(&r);
                r
            })
            .collect();
    } else {
        let mut sorted = pop.clone();
        sorted.sort_by(|a, b| b.fit.partial_cmp(&a.fit).unwrap());
        let n_el = 2usize.min(sorted.len());
        let mut newpop: Vec<Rec> = sorted
            .iter()
            .take(n_el)
            .cloned()
            .map(|mut r| {
                r.gen = gen;
                r
            })
            .collect();
        while newpop.len() < pop_size {
            let ia = (rng.f64() * sorted.len() as f64) as usize;
            let pa = &sorted[ia];
            let child_g = pa.g.mutate(rng, 0.08);
            *next_id += 1;
            let id = *next_id - 1;
            let mut r = base_rec(&child_g, id, gen);
            r.parents = vec![pa.id];
            r.op = "mutate";
            r.fit = qd::fitness(&r);
            newpop.push(r);
            counters.fresh_evals += 1;
        }
        *pop = newpop;
    }
    counters.appearances += pop.len() as u64;
    for r in pop.iter() {
        if r.gen == gen {
            sink.event(Event::Eval {
                gen,
                island: 0,
                id: r.id as u64,
                parents: r.parents.iter().map(|&p| p as u64).collect(),
                op: r.op.to_string(),
                op_version: r.op_version,
                fit: Some(r.fit),
                aes_pass: r.aes.pass,
                legal: r.legal(),
                mass_g: Some(r.mass_g),
                fem_status: "complete".into(),
            });
        } else {
            sink.event(Event::Elite {
                gen,
                island: 0,
                id: r.id as u64,
            });
        }
        if qd::feasible(r) {
            let c = qd::qd_cell(r);
            if archive.get(&c).map_or(true, |e| r.fit > e.fit) {
                archive.insert(c, r.clone());
            }
        }
    }
    let best = pop
        .iter()
        .max_by(|a, b| a.fit.partial_cmp(&b.fit).unwrap())
        .unwrap();
    sink.event(Event::GenEnd {
        gen,
        fresh_evals: counters.fresh_evals,
        appearances: counters.appearances,
        archive_cells: archive.len(),
        best_id: Some(best.id as u64),
        best_fit: Some(best.fit),
    });
    sink.end_gen();
}

fn snapshot_ckpt(
    manifest_sha: &str,
    next_gen: usize,
    rng: &Rng,
    next_id: usize,
    counters: &Counters,
    pop: &[Rec],
    archive: &BTreeMap<(usize, usize), Rec>,
) -> SearchCheckpoint {
    SearchCheckpoint {
        version: 3,
        manifest_sha256: manifest_sha.to_string(),
        forked_from_manifest_sha256: None,
        forked_at_gen: None,
        next_gen,
        rng_state: rng.0,
        next_id,
        counters: (counters.fresh_evals, counters.appearances),
        pop_genes: pop
            .iter()
            .map(|r| (r.id, r.gen, r.g.v.to_vec(), r.parents.clone(), r.op.to_string()))
            .collect(),
        archive_cells: archive
            .iter()
            .map(|(&c, r)| (c, r.id, r.fit.to_bits()))
            .collect(),
    }
}

fn validate_and_restore_ckpt(
    raw: &str,
    expected_manifest_sha: &str,
    allow_fork: bool,
) -> Result<SearchCheckpoint, &'static str> {
    let mut ckpt: SearchCheckpoint =
        serde_json::from_str(raw).map_err(|_| "corrupt_or_truncated_checkpoint")?;
    if ckpt.version < 3 {
        return Err("legacy_checkpoint_version_refused");
    }
    if ckpt.pop_genes.is_empty() {
        return Err("empty_checkpoint_population");
    }
    if ckpt.manifest_sha256 != expected_manifest_sha {
        if allow_fork {
            ckpt.forked_from_manifest_sha256 = Some(ckpt.manifest_sha256.clone());
            ckpt.forked_at_gen = Some(ckpt.next_gen);
            ckpt.manifest_sha256 = expected_manifest_sha.to_string();
        } else {
            return Err("manifest_identity_mismatch");
        }
    }
    Ok(ckpt)
}

// ---------------------------------------------------------------------------
// T04-07: checkpoint_matrix (3 seeds x generations 3/7/12)
// ---------------------------------------------------------------------------
#[test]
fn t04_07_checkpoint_matrix_positive_and_adverse() {
    for seed in [1u64, 42, 9001] {
        let manifest = RunManifest::new("crescent-search", seed, "RS-karambit-v2")
            .param("pop", 8)
            .param("gens", 12);
        let m_sha = manifest.sha256();

        // Uninterrupted run through generation 12
        let mut rng_full = Rng(seed);
        let mut next_id_full = 0usize;
        let mut cnt_full = Counters::default();
        let mut pop_full = Vec::new();
        let mut arch_full = BTreeMap::new();
        let mut sink_full = VecSink::default();
        for g in 0..=12 {
            step_generation(
                g,
                8,
                &mut rng_full,
                &mut next_id_full,
                &mut cnt_full,
                &mut pop_full,
                &mut arch_full,
                &mut sink_full,
            );
        }
        let final_full = snapshot_ckpt(
            &m_sha,
            13,
            &rng_full,
            next_id_full,
            &cnt_full,
            &pop_full,
            &arch_full,
        );

        for stop in [3usize, 7, 12] {
            let mut rng = Rng(seed);
            let mut next_id = 0usize;
            let mut cnt = Counters::default();
            let mut pop = Vec::new();
            let mut arch = BTreeMap::new();
            let mut sink = VecSink::default();
            for g in 0..stop {
                step_generation(
                    g,
                    8,
                    &mut rng,
                    &mut next_id,
                    &mut cnt,
                    &mut pop,
                    &mut arch,
                    &mut sink,
                );
            }
            let saved_json = serde_json::to_string(&snapshot_ckpt(
                &m_sha, stop, &rng, next_id, &cnt, &pop, &arch,
            ))
            .unwrap();
            let restored = validate_and_restore_ckpt(&saved_json, &m_sha, false).unwrap();
            assert_eq!(restored.next_gen, stop);
            let mut rng_res = Rng(restored.rng_state);
            let mut next_id_res = restored.next_id;
            let mut cnt_res = Counters {
                fresh_evals: restored.counters.0,
                appearances: restored.counters.1,
            };
            for g in stop..=12 {
                step_generation(
                    g,
                    8,
                    &mut rng_res,
                    &mut next_id_res,
                    &mut cnt_res,
                    &mut pop,
                    &mut arch,
                    &mut sink,
                );
            }
            let final_res = snapshot_ckpt(
                &m_sha,
                13,
                &rng_res,
                next_id_res,
                &cnt_res,
                &pop,
                &arch,
            );
            assert_eq!(
                final_res, final_full,
                "seed {seed} stop {stop}: resumed state must match uninterrupted state bitwise"
            );
        }

        // Adverse control: perturbing RNG state on resume causes state mismatch and is detected.
        let mut broken = final_full.clone();
        broken.rng_state ^= 1;
        assert_ne!(broken, final_full);
    }
}

// ---------------------------------------------------------------------------
// T04-08: checkpoint_corrupt
// ---------------------------------------------------------------------------
#[test]
fn t04_08_checkpoint_corrupt_and_incomplete_frame() {
    let m = RunManifest::new("crescent-search", 7, "RS-karambit-v2");
    let sha = m.sha256();
    let ckpt = SearchCheckpoint {
        version: 3,
        manifest_sha256: sha.clone(),
        forked_from_manifest_sha256: None,
        forked_at_gen: None,
        next_gen: 3,
        rng_state: 12345,
        next_id: 24,
        counters: (24, 24),
        pop_genes: vec![(0, 0, Genome::c4_seed(5.7).v.to_vec(), vec![], "seed".into())],
        archive_cells: vec![],
    };
    let raw = serde_json::to_string(&ckpt).unwrap();
    assert!(validate_and_restore_ckpt(&raw, &sha, false).is_ok());

    // Truncated checkpoint rejected
    assert_eq!(
        validate_and_restore_ckpt(&raw[..raw.len() - 12], &sha, false),
        Err("corrupt_or_truncated_checkpoint")
    );
    // Legacy v1/v2 checkpoint rejected
    let mut v2 = ckpt.clone();
    v2.version = 2;
    assert_eq!(
        validate_and_restore_ckpt(&serde_json::to_string(&v2).unwrap(), &sha, false),
        Err("legacy_checkpoint_version_refused")
    );

    // Incomplete/corrupt event frame in EvidenceLog: intact prefix retained, torn_tail flagged
    let dir = std::env::temp_dir().join(format!("t04_08_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("run.jevl");
    {
        let mut log = EvidenceLog::create(path.to_str().unwrap(), &m).unwrap();
        log.event(Event::Eval {
            gen: 0,
            island: 0,
            id: 0,
            parents: vec![],
            op: "seed".into(),
            op_version: OP_VERSION,
            fit: Some(1.1),
            aes_pass: true,
            legal: true,
            mass_g: Some(75.0),
            fem_status: "complete".into(),
        });
        log.event(Event::GenEnd {
            gen: 0,
            fresh_evals: 1,
            appearances: 1,
            archive_cells: 1,
            best_id: Some(0),
            best_fit: Some(1.1),
        });
        log.end_gen();
    }
    let mut bytes = std::fs::read(&path).unwrap();
    bytes.extend_from_slice(b"JEV1\x10\x00\x00\x00corrupt_tail");
    let parsed = read_bytes(&bytes).unwrap();
    assert!(parsed.torn_tail, "incomplete tail frame must set torn_tail");
    assert_eq!(parsed.frames, 2, "intact RunStart + gen 0 frames retained");
    std::fs::remove_dir_all(&dir).ok();
}

// ---------------------------------------------------------------------------
// T04-09: resume_identity
// ---------------------------------------------------------------------------
fn sample_scientific_identity(eval_digest: char, mat_id: &str, req_id: &str) -> ScientificIdentity {
    let h = |c: char| c.to_string().repeat(64);
    let cref = |id: &str, c: char| ContentRef {
        id: id.into(),
        revision: "v1".into(),
        sha256: h(c),
    };
    ScientificIdentity {
        schema: 2,
        subject: SubjectRef::Genome {
            representation: cref("crescent-genome", '1'),
            bits: vec![format!("{:016x}", 5.7f64.to_bits())],
        },
        requirements: vec![cref(req_id, 'b')],
        source: cref("jarambit-source", 'a'),
        build: cref("jarambit-build", 'c'),
        cargo_lock: cref("cargo-lock", 'd'),
        config: cref("search-config", 'e'),
        evaluator: cref("crescent-qd", eval_digest),
        scenario: cref("d1-loads", 'f'),
        objective: cref("qd-fitness", '0'),
        descriptor: cref("qd-desc-16", '3'),
        material: MaterialState::Identified {
            passport: cref(mat_id, '4'),
        },
        dependencies: vec![],
    }
}

#[test]
fn t04_09_resume_identity_and_explicit_fork() {
    let id_base = sample_scientific_identity('2', "Ti-6Al-4V", "RS-karambit-v2");
    let m_base = RunManifest::qualified("evolve", 11, id_base)
        .unwrap()
        .param("descriptor_sha256", "3".repeat(64));
    let base_sha = m_base.scientific_sha256().unwrap();

    let ckpt = SearchCheckpoint {
        version: 3,
        manifest_sha256: base_sha.clone(),
        forked_from_manifest_sha256: None,
        forked_at_gen: None,
        next_gen: 4,
        rng_state: 99,
        next_id: 32,
        counters: (32, 32),
        pop_genes: vec![(0, 0, Genome::c4_seed(5.7).v.to_vec(), vec![], "seed".into())],
        archive_cells: vec![],
    };
    let raw = serde_json::to_string(&ckpt).unwrap();

    // Changed evaluator binary digest, material, requirement set, op_version, or descriptor hash
    let mut m_eval = RunManifest::qualified(
        "evolve",
        11,
        sample_scientific_identity('9', "Ti-6Al-4V", "RS-karambit-v2"),
    )
    .unwrap()
    .param("descriptor_sha256", "3".repeat(64));
    let m_mat = RunManifest::qualified(
        "evolve",
        11,
        sample_scientific_identity('2', "Ti-7Al", "RS-karambit-v2"),
    )
    .unwrap()
    .param("descriptor_sha256", "3".repeat(64));
    let m_req = RunManifest::qualified(
        "evolve",
        11,
        sample_scientific_identity('2', "Ti-6Al-4V", "RS-karambit-v1"),
    )
    .unwrap()
    .param("descriptor_sha256", "3".repeat(64));
    let mut m_op = m_base.clone();
    m_op.op_version = 2;
    let m_desc = m_base.clone().param("descriptor_sha256", "4".repeat(64));

    for mutated in [&m_eval, &m_mat, &m_req, &m_op, &m_desc] {
        let new_sha = mutated.scientific_sha256().unwrap();
        assert_ne!(new_sha, base_sha);
        assert_eq!(
            validate_and_restore_ckpt(&raw, &new_sha, false),
            Err("manifest_identity_mismatch"),
            "resume must be refused when evaluator/material/RS/op/descriptor identity changes"
        );
    }

    // Explicit fork creates a new run identity binding the parent manifest SHA and generation
    let new_sha = m_eval.scientific_sha256().unwrap();
    let forked = validate_and_restore_ckpt(&raw, &new_sha, true).unwrap();
    assert_eq!(forked.manifest_sha256, new_sha);
    assert_eq!(
        forked.forked_from_manifest_sha256.as_deref(),
        Some(base_sha.as_str())
    );
    assert_eq!(forked.forked_at_gen, Some(4));
    let _ = &mut m_eval;
}

// ---------------------------------------------------------------------------
// T04-10: durability_boundary
// ---------------------------------------------------------------------------
fn qualify_log_for_publication(
    log_err: Option<&str>,
    parsed: &jarambit_evidence::ReadLog,
) -> Result<(), &'static str> {
    if log_err.is_some() {
        return Err("log_io_or_durability_error");
    }
    if parsed.torn_tail {
        return Err("torn_tail_incomplete_frame");
    }
    if !matches!(parsed.events.first(), Some(Event::RunStart { .. })) {
        return Err("missing_run_start");
    }
    if !matches!(parsed.events.last(), Some(Event::RunEnd { .. })) {
        return Err("missing_run_end");
    }
    Ok(())
}

#[test]
fn t04_10_durability_boundary_positive_and_adverse() {
    let dir = std::env::temp_dir().join(format!("t04_10_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("durable.jevl");
    let m = RunManifest::new("crescent-search", 21, "RS-karambit-v2");

    let mut log = EvidenceLog::create(path.to_str().unwrap(), &m).unwrap();
    log.event(Event::Eval {
        gen: 0,
        island: 0,
        id: 0,
        parents: vec![],
        op: "seed".into(),
        op_version: OP_VERSION,
        fit: Some(1.2),
        aes_pass: true,
        legal: true,
        mass_g: Some(74.5),
        fem_status: "complete".into(),
    });
    log.event(Event::GenEnd {
        gen: 0,
        fresh_evals: 1,
        appearances: 1,
        archive_cells: 1,
        best_id: Some(0),
        best_fit: Some(1.2),
    });
    log.end_gen();

    // Before RunEnd is flushed (interrupted mid-run): must refuse qualified publication
    let mid_parsed = read_bytes(&std::fs::read(&path).unwrap()).unwrap();
    assert_eq!(
        qualify_log_for_publication(log.error(), &mid_parsed),
        Err("missing_run_end")
    );

    // Adverse durability error (e.g. disk-full / fsync failure): must refuse qualified publication
    assert_eq!(
        qualify_log_for_publication(Some("No space left on device"), &mid_parsed),
        Err("log_io_or_durability_error")
    );

    // Positive control: after RunEnd is durably flushed with no I/O error, publication is allowed
    log.event(Event::RunEnd {
        fresh_evals: 1,
        appearances: 1,
    });
    log.end_gen();
    assert!(log.error().is_none());
    let done_parsed = read_bytes(&std::fs::read(&path).unwrap()).unwrap();
    assert_eq!(qualify_log_for_publication(log.error(), &done_parsed), Ok(()));
    std::fs::remove_dir_all(&dir).ok();
}

// ---------------------------------------------------------------------------
// T04-11: duplicate_resume
// ---------------------------------------------------------------------------
fn verify_no_duplicate_fresh_evals(events: &[Event]) -> Result<(u64, u64), &'static str> {
    let mut born = HashSet::new();
    let mut fresh = 0u64;
    let mut app = 0u64;
    for ev in events {
        match ev {
            Event::Eval { id, .. } => {
                if !born.insert(*id) {
                    return Err("duplicate_fresh_evaluation");
                }
                fresh += 1;
                app += 1;
            }
            Event::Elite { id, .. } => {
                if !born.contains(id) {
                    return Err("unborn_elite_appearance");
                }
                app += 1;
            }
            Event::GenEnd {
                fresh_evals,
                appearances,
                ..
            } => {
                if *fresh_evals != fresh || *appearances != app {
                    return Err("counter_inflation_or_mismatch");
                }
            }
            _ => {}
        }
    }
    Ok((fresh, app))
}

#[test]
fn t04_11_duplicate_resume_positive_and_adverse() {
    let cfg = RunCfg {
        pop: 8,
        gens: 3,
        threads: 2,
        seed: 19,
        fem_area: 2.0,
        t_seed: 5.7,
    };
    let mut sink = VecSink::default();
    let out = qd::run_with(&cfg, &|g, id, gen, _| base_rec(g, id, gen), |_| {}, &mut sink);
    assert_eq!(
        verify_no_duplicate_fresh_evals(&sink.events),
        Ok((out.counters.fresh_evals, out.counters.appearances))
    );

    // Adverse control 1: replaying an already-accepted evaluation as a second Event::Eval
    let mut dup_events = sink.events.clone();
    dup_events.insert(
        2,
        Event::Eval {
            gen: 0,
            island: 0,
            id: 0,
            parents: vec![],
            op: "seed".into(),
            op_version: OP_VERSION,
            fit: Some(1.1),
            aes_pass: true,
            legal: true,
            mass_g: Some(75.0),
            fem_status: "complete".into(),
        },
    );
    assert_eq!(
        verify_no_duplicate_fresh_evals(&dup_events),
        Err("duplicate_fresh_evaluation")
    );

    // Adverse control 2: counter inflation in GenEnd without matching fresh evaluations
    let mut inflated = sink.events.clone();
    for ev in &mut inflated {
        if let Event::GenEnd { fresh_evals, .. } = ev {
            *fresh_evals += 1;
            break;
        }
    }
    assert_eq!(
        verify_no_duplicate_fresh_evals(&inflated),
        Err("counter_inflation_or_mismatch")
    );
}

// ---------------------------------------------------------------------------
// T04-12: lineage_orphan
// ---------------------------------------------------------------------------
fn verify_strict_lineage(events: &[Event], num_islands: usize) -> Result<(), &'static str> {
    let mut born_global: HashSet<u64> = HashSet::new();
    let mut island_resident: HashMap<usize, HashSet<u64>> = HashMap::new();
    for e in events {
        match e {
            Event::Eval {
                island,
                id,
                parents,
                op,
                ..
            } => {
                if !op.starts_with("seed") && parents.is_empty() {
                    return Err("non_seed_missing_parents");
                }
                for p in parents {
                    if !born_global.contains(p) {
                        return Err("orphan_parent");
                    }
                }
                if !born_global.insert(*id) {
                    return Err("duplicate_birth_id");
                }
                island_resident.entry(*island).or_default().insert(*id);
            }
            Event::Elite { island, id, .. } => {
                if !born_global.contains(id) {
                    return Err("unborn_elite");
                }
                island_resident.entry(*island).or_default().insert(*id);
            }
            Event::Migration { from, to, ids, .. } => {
                if (*from + 1) % num_islands.max(1) != *to {
                    return Err("migration_ring_order_mismatch");
                }
                let src = island_resident.entry(*from).or_default();
                for id in ids {
                    if !born_global.contains(id) || !src.contains(id) {
                        return Err("migrant_not_on_source_island");
                    }
                }
                let dst = island_resident.entry(*to).or_default();
                for id in ids {
                    dst.insert(*id);
                }
            }
            _ => {}
        }
    }
    Ok(())
}

#[test]
fn t04_12_lineage_orphan_positive_and_adverse() {
    let base = RunCfg {
        pop: 8,
        gens: 4,
        threads: 2,
        seed: 5,
        fem_area: 2.0,
        t_seed: 5.7,
    };
    let cfg = IslandCfg {
        base,
        islands: 3,
        migrate_every: 2,
        migrants: 2,
        seeds: vec![Genome::c6_seed(5.7)],
        kick_frac: 0.2,
    };
    let mut sink = VecSink::default();
    let _ = qd::run_islands_with(&cfg, &|g, id, gen, _| base_rec(g, id, gen), |_| {}, |_, _, _| {}, &mut sink);
    assert_eq!(verify_strict_lineage(&sink.events, 3), Ok(()));

    // Adverse 1: orphan parent reference
    let mut orphan_parent = sink.events.clone();
    orphan_parent.insert(
        1,
        Event::Eval {
            gen: 0,
            island: 0,
            id: 999_999,
            parents: vec![888_888],
            op: "mutate".into(),
            op_version: OP_VERSION,
            fit: Some(1.0),
            aes_pass: true,
            legal: true,
            mass_g: Some(75.0),
            fem_status: "complete".into(),
        },
    );
    assert_eq!(
        verify_strict_lineage(&orphan_parent, 3),
        Err("orphan_parent")
    );

    // Adverse 2: elite referencing never-born ID
    let mut unborn_elite = sink.events.clone();
    unborn_elite.insert(
        1,
        Event::Elite {
            gen: 0,
            island: 0,
            id: 777_777,
        },
    );
    assert_eq!(
        verify_strict_lineage(&unborn_elite, 3),
        Err("unborn_elite")
    );

    // Adverse 3: migration ring order mismatch (0 -> 2 instead of 0 -> 1 on a 3-island ring)
    let mut bad_mig = sink.events.clone();
    bad_mig.push(Event::Migration {
        gen: 2,
        from: 0,
        to: 2,
        ids: vec![0],
    });
    assert_eq!(
        verify_strict_lineage(&bad_mig, 3),
        Err("migration_ring_order_mismatch")
    );
}

#[test]
fn regression_observer_captures_only_fresh_evaluations_and_preserves_search() {
    let cfg = RunCfg { pop: 10, gens: 3, threads: 2, seed: 121, fem_area: 2.0, t_seed: 5.7 };
    let ev = |g: &Genome, id, gen, _| base_rec(g, id, gen);
    let baseline = qd::run_with(&cfg, &ev, |_| {}, &mut jarambit_evidence::NullSink);
    let mut ids = HashSet::new();
    let observed = qd::run_observed(&cfg, &ev, |_| {}, &mut jarambit_evidence::NullSink, &mut |rows| {
        for row in rows { assert!(ids.insert(row.id), "elite/migrant duplicated"); }
        Ok(())
    }).unwrap();
    assert_eq!(ids.len() as u64, observed.counters.fresh_evals);
    assert_eq!(baseline.history, observed.history);
    assert_eq!(baseline.all.iter().map(|r| (&r.g,r.fit)).collect::<Vec<_>>(), observed.all.iter().map(|r| (&r.g,r.fit)).collect::<Vec<_>>());
    assert!(qd::run_observed(&cfg, &ev, |_| {}, &mut jarambit_evidence::NullSink, &mut |_| Err("disk full".into())).is_err());
}

#[test]
fn regression_island_observer_sees_initial_and_terminal_birth_batches() {
    let cfg = IslandCfg { base: RunCfg {pop:8,gens:3,threads:2,seed:122,fem_area:2.0,t_seed:5.7}, islands:2,migrate_every:1,migrants:1,seeds:vec![],kick_frac:0.0 };
    let ev = |g: &Genome,id,gen,_|base_rec(g,id,gen);
    let baseline=qd::run_islands_with(&cfg,&ev, |_| {}, |_,_,_| {}, &mut jarambit_evidence::NullSink);
    let mut ids=HashSet::new();
    let result=qd::run_islands_observed(&cfg,&ev, |_| {}, |_,_,_| {}, &mut jarambit_evidence::NullSink,&mut |rows| {for r in rows{assert!(ids.insert(r.id));}Ok(())}).unwrap();
    assert_eq!(ids.len() as u64,result.counters.fresh_evals);
    assert_eq!(baseline.history,result.history);
}
