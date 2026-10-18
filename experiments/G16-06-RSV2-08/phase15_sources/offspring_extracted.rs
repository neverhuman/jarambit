const Q7_CPS: [f64; genome::N_CP] = [
    0.0, 2.4720345762624114, 3.0, 3.0, 3.0, 3.0, 2.4727688443313927, 1.4582812727496934,
    0.7542362460948389, 0.9273662522036018, 0.44919959806570164, 0.15399709825071464,
    0.00598327715607002, 0.18153879059595732, 0.021699284063914397, -0.3821724162115406,
    1.006980625093307, 0.8631642138621967, -0.11906173746648413, 0.0, 0.0, 0.0, 0.0,
    3.0, 3.0, 3.0, 1.103243361778542, 0.6220664759723078, 0.21270040153753478, 2.0,
    1.7300803824233073, 0.8845314429627344, 0.5190470296230849, -0.4393508907352227,
    -1.8777655284497983, -1.2615199166944122, -0.13376737149017054, -0.8889664255562653,
    -0.8918546356656645, 0.0,
];
pub fn q7_offspring_arm_label(c: usize) -> Option<&'static str> {
    match c {
        6 => Some("q7_c6_window_pocket"),
        7 => Some("q7_c7_distal_window"),
        8 => Some("q7_c8_delayed_taper"),
        9 => Some("q7_c9_reinforced_ring"),
        10 => Some("q7_c10_spine_widened"),
        11 => Some("q7_c11_spine_widened_p1"),
        _ => None,
    }
}

fn spawn_offspring_batch(
    elites: &[(String, Genome)],
    candidates_per_elite: usize,
    round: usize,
    rng: &mut Rng,
) -> Vec<(String, Genome)> {
    let mut batch = Vec::new();
    let b = genome::bounds();

    for (fam, parent) in elites {
        // Always include the exact parent in round 0 if not yet certified
        if round == 0 {
            let mut p0 = parent.clone();
            if !p0.rs_v2() {
                p0.v[EDGE_TH] = 0.18;
                p0.clamp();
            }
            batch.push((fam.clone(), p0.clone()));
            if p0.v[genome::RING_WALL] < 6.0 {
                p0.v[genome::RING_WALL] = p0.v[genome::RING_WALL].max(6.05);
                p0.v[..N_CP].fill(0.0);
                p0.clamp();
                batch.push((fam.clone(), p0));
            }
        }

        for k in 0..candidates_per_elite {
            let mut child = parent.clone();
            let scale = 0.35 / ((round + 1) as f64).sqrt();

            // Keep the legacy witness and reinforcement slots available in the
            // production 12-child cycle; 11-child test batches retain their Q7 slots.
            let p1_tip_slot = candidates_per_elite == 12 && k % 12 == 11;
            let q7_slot = (matches!(k % 11, 6..=10)
                && !(candidates_per_elite == 12 && k % 12 == 7))
                || p1_tip_slot;
            if q7_slot {
                let slot = if p1_tip_slot { 11 } else { k % 11 };
                debug_assert!(q7_offspring_arm_label(slot).is_some());
                child.v[..N_CP].copy_from_slice(&Q7_CPS);
                if slot != 9 {
                    for (idx, cp) in child.v[..N_CP].iter_mut().enumerate() {
                        *cp = (*cp + rng.range(-0.005, 0.005)).clamp(b[idx].0, b[idx].1);
                    }
                    child.v[TAPER_X0] = rng.range(116.0, 118.0).clamp(b[TAPER_X0].0, b[TAPER_X0].1);
                } else {
                    child.v[TAPER_X0] = 116.0;
                }
                child.v[genome::DROP] = 0.0;
                child.v[genome::GRIP_DX] = -0.4;
                child.v[genome::RING_ID] = 24.9;
                child.v[genome::NECK_FILLET] = 8.0;
                child.v[genome::SPINE_OFF] = 42.0;
                child.v[genome::T] = 6.0;
                child.v[T_TIP] = 2.62;
                child.v[EA_IN] = 18.0;
                child.v[EA_SP] = 18.0;

                match slot {
                    6 => {
                        let variant = (rng.next_u64() % 3) as f64;
                        child.v[genome::WIN_N] = 1.0;
                        child.v[genome::WIN_S0] = (0.45 + 0.02 * variant + rng.range(-0.003, 0.003))
                            .clamp(b[genome::WIN_S0].0, b[genome::WIN_S0].1);
                        child.v[genome::WIN_S1] = 0.84_f64
                            .clamp(b[genome::WIN_S1].0, b[genome::WIN_S1].1);
                        child.v[genome::WIN_W] = (2.7 + 0.2 * variant + rng.range(-0.02, 0.02))
                            .clamp(b[genome::WIN_W].0, b[genome::WIN_W].1);
                        child.v[genome::WIN_FILL] = (0.66 + 0.02 * variant + rng.range(-0.005, 0.005))
                            .clamp(b[genome::WIN_FILL].0, b[genome::WIN_FILL].1);
                        // The schema names shallow pocket relief POCKET_D; POCKET_W is not a gene.
                        child.v[POCKET_D] = (0.15 + 0.05 * variant + rng.range(-0.01, 0.01))
                            .clamp(b[POCKET_D].0, b[POCKET_D].1);
                        child.v[genome::RING_WALL] = 6.20_f64
                            .clamp(b[genome::RING_WALL].0, b[genome::RING_WALL].1);
                        child.v[genome::RING_ID] = 25.4_f64
                            .clamp(b[genome::RING_ID].0, b[genome::RING_ID].1);
                        child.v[genome::GRIP_DX] = -0.4 + child.v[genome::RING_WALL] - 6.0;
                        child.v[TAPER_X0] = 126.0_f64.clamp(b[TAPER_X0].0, b[TAPER_X0].1);
                        child.v[T_TIP] = (2.65 + 0.08 * variant + rng.range(-0.01, 0.01))
                            .clamp(b[T_TIP].0, b[T_TIP].1);
                        // EA_IN's schema floor is 14.0; FUL_D is bounded to 1.2 (0.6 in P1).
                        child.v[EA_IN] = 14.0_f64.clamp(b[EA_IN].0, b[EA_IN].1);
                        child.v[FUL_D] = 0.5_f64.clamp(b[FUL_D].0, b[FUL_D].1);
                    }
                    7 => {
                        // Distal-window armB neighborhood from the V7 prototype target.
                        child.v[genome::WIN_N] = 1.0;
                        child.v[genome::WIN_S0] = rng.range(0.495, 0.499)
                            .clamp(b[genome::WIN_S0].0, b[genome::WIN_S0].1);
                        child.v[genome::WIN_S1] = rng.range(0.82, 0.85)
                            .clamp(b[genome::WIN_S1].0, b[genome::WIN_S1].1);
                        child.v[genome::WIN_W] = rng.range(3.01, 3.10)
                            .clamp(b[genome::WIN_W].0, b[genome::WIN_W].1);
                        child.v[genome::WIN_FILL] = 0.65_f64
                            .clamp(b[genome::WIN_FILL].0, b[genome::WIN_FILL].1);
                        child.v[TAPER_X0] = rng.range(143.5, 144.5)
                            .clamp(b[TAPER_X0].0, b[TAPER_X0].1);
                        child.v[T_TIP] = rng.range(2.78, 2.82)
                            .clamp(b[T_TIP].0, b[T_TIP].1);
                        child.v[genome::RING_WALL] = rng.range(6.38, 6.42)
                            .clamp(b[genome::RING_WALL].0, b[genome::RING_WALL].1);
                        child.v[genome::RING_ID] = rng.range(24.91, 24.99)
                            .clamp(b[genome::RING_ID].0, b[genome::RING_ID].1);
                        child.v[genome::GRIP_DX] = -0.4;
                    }
                    8 => {
                        child.v[T_TIP] = rng.range(4.0, 4.45).clamp(b[T_TIP].0, b[T_TIP].1);
                        child.v[TAPER_X0] = rng.range(142.0, 149.0).clamp(b[TAPER_X0].0, b[TAPER_X0].1);
                        child.v[EA_IN] = rng.range(18.0, 18.6).clamp(b[EA_IN].0, b[EA_IN].1);
                        child.v[EA_SP] = rng.range(18.0, 18.6).clamp(b[EA_SP].0, b[EA_SP].1);
                        child.v[EDGE_TH] = rng.range(0.185, 0.195).clamp(0.12, 0.20);
                        child.v[genome::RING_WALL] = rng.range(6.02, 6.22).clamp(b[genome::RING_WALL].0, b[genome::RING_WALL].1);
                        child.v[genome::GRIP_DX] = -0.4;
                        child.v[genome::WIN_N] = 1.0;
                        child.v[genome::WIN_S0] = 0.50_f64.clamp(b[genome::WIN_S0].0, b[genome::WIN_S0].1);
                        child.v[genome::WIN_S1] = 0.85_f64.clamp(b[genome::WIN_S1].0, b[genome::WIN_S1].1);
                        child.v[genome::WIN_W] = 3.8_f64.clamp(b[genome::WIN_W].0, b[genome::WIN_W].1);
                        child.v[genome::WIN_FILL] = 0.72_f64.clamp(b[genome::WIN_FILL].0, b[genome::WIN_FILL].1);
                    }
                    9 => {
                        child.v[T_TIP] = rng.range(2.62, 2.85);
                        child.v[genome::WIN_N] = 1.0;
                        child.v[genome::WIN_W] = 3.0;
                        child.v[genome::WIN_S0] = rng.range(0.48, 0.52);
                        child.v[genome::WIN_S1] = rng.range(0.82, 0.86);
                        child.v[genome::WIN_FILL] = 0.65;
                        child.v[genome::RING_WALL] = rng.range(6.15, 6.35);
                        child.v[genome::GRIP_DX] =
                            -0.4 + child.v[genome::RING_WALL] - 6.0;
                    }
                    10 | 11 => {
                        child.v[genome::WIN_N] = 1.0;
                        child.v[genome::WIN_W] = 3.0;
                        child.v[genome::WIN_S0] = rng.range(0.48, 0.52);
                        child.v[genome::WIN_S1] = rng.range(0.82, 0.86);
                        child.v[genome::WIN_FILL] = 0.65;
                        child.v[genome::RING_WALL] = rng.range(6.02, 6.22);
                        child.v[genome::GRIP_DX] = -0.4;
                        for (idx, low, high) in [(36, 0.25, 0.40), (37, 0.45, 0.60), (38, 0.30, 0.50)] {
                            child.v[idx] = (child.v[idx] + rng.range(low, high))
                                .clamp(b[idx].0, b[idx].1);
                        }
                        child.v[T_TIP] = rng.range(4.0, 4.45).clamp(b[T_TIP].0, b[T_TIP].1);
                        child.v[TAPER_X0] = rng.range(142.0, 149.0)
                            .clamp(b[TAPER_X0].0, b[TAPER_X0].1);
                        child.v[EA_IN] = rng.range(18.0, 18.6).clamp(b[EA_IN].0, b[EA_IN].1);
                        child.v[EA_SP] = rng.range(18.0, 18.6).clamp(b[EA_SP].0, b[EA_SP].1);
                        child.v[EDGE_TH] = rng.range(0.185, 0.195).clamp(0.12, 0.20);
                        if slot == 11 {
                            child.v[genome::P1_ON] = 1.0;
                            child.v[POCKET_D] = rng.range(0.15, 0.35)
                                .clamp(b[POCKET_D].0, b[POCKET_D].1);
                            child.v[RAIL_W] = rng.range(4.2, 5.2).clamp(b[RAIL_W].0, b[RAIL_W].1);
                            child.v[NECK_L] = rng.range(9.0, 11.5).clamp(b[NECK_L].0, b[NECK_L].1);
                            child.v[POCKET_R] = rng.range(3.2, 4.5)
                                .clamp(b[POCKET_R].0, b[POCKET_R].1);
                        }
                    }
                    _ => unreachable!(),
                }
                // All Q7 split-hand arms stay inside the Gate S bevel and apex limits,
                // including arms whose individual recipes inherit these genes.
                child.v[EA_IN] = child.v[EA_IN].clamp(b[EA_IN].0, 18.6);
                child.v[EA_SP] = child.v[EA_SP].clamp(b[EA_SP].0, 18.6);
                child.v[EDGE_TH] = child.v[EDGE_TH].clamp(0.12, 0.195);
                if !jarambit_crescent::aesthetic::evaluate(&child, &jarambit_crescent::pheno::build(&child)).pass {
                    child.v[..36].copy_from_slice(&Q7_CPS[..36]);
                    if slot < 10 {
                        child.v[37] = (Q7_CPS[37] + 0.003).clamp(b[37].0, b[37].1);
                    }
                }
            } else {
            match (
                k % 8,
                k % 9 == 8 || (candidates_per_elite == 12 && k % 12 == 4),
                k % 11 == 9,
                k % 11 == 10,
            ) {
                (_, true, _, _) => {
                    // Witness-guided arm: increase the blade-root section controls and use the
                    // retained multi-mesh window witness when this parent's window is enabled.
                    for (idx, delta_lo, delta_hi) in
                        [(TAPER_X0, 0.15, 0.30), (EA_IN, 0.10, 0.25)]
                    {
                        child.v[idx] = (child.v[idx] + rng.range(delta_lo, delta_hi))
                            .clamp(b[idx].0, b[idx].1);
                    }
                    if child.v[genome::WIN_N] >= 1.0 {
                        child.v[genome::WIN_S0] = rng
                            .range(0.05, 0.052)
                            .clamp(b[genome::WIN_S0].0, b[genome::WIN_S0].1);
                        child.v[genome::WIN_S1] = rng
                            .range(0.502, 0.525)
                            .clamp(b[genome::WIN_S1].0, b[genome::WIN_S1].1);
                        child.v[genome::WIN_W] =
                            6.0_f64.clamp(b[genome::WIN_W].0, b[genome::WIN_W].1);
                        child.v[genome::WIN_FILL] = rng
                            .range(0.65, 0.70)
                            .clamp(b[genome::WIN_FILL].0, b[genome::WIN_FILL].1);
                    }
                    child.v[genome::RING_WALL] = child.v[genome::RING_WALL]
                        .max(6.0)
                        .min(b[genome::RING_WALL].1);
                    child.v[EDGE_TH] = child.v[EDGE_TH].clamp(0.12, 0.20);
                }
                (0, false, false, false) => {
                    if child.p1() {
                        child.v[POCKET_D] = (child.v[POCKET_D] + rng.normal() * 0.015 * scale)
                            .clamp(b[POCKET_D].0, b[POCKET_D].1);
                        child.v[POCKET_R] = (child.v[POCKET_R] + rng.normal() * 0.08 * scale)
                            .clamp(b[POCKET_R].0, b[POCKET_R].1);
                        child.v[RAIL_W] = (child.v[RAIL_W] + rng.normal() * 0.08 * scale)
                            .clamp(b[RAIL_W].0, b[RAIL_W].1);
                        child.v[NECK_L] = (child.v[NECK_L] + rng.normal() * 0.12 * scale)
                            .clamp(b[NECK_L].0, b[NECK_L].1);
                    } else {
                        for idx in [
                            genome::ISO_D,
                            genome::ISO_RIB,
                            genome::WIN_W,
                            genome::WIN_FILL,
                            genome::PERF_D,
                            genome::PERF_GAP,
                        ] {
                            let span = b[idx].1 - b[idx].0;
                            child.v[idx] = (child.v[idx] + rng.normal() * 0.008 * span * scale)
                                .clamp(b[idx].0, b[idx].1);
                        }
                    }
                }
                (1, false, false, false) => {
                    // Distal taper & fuller tuning (tip torsion + sequence ratio balance)
                    child.v[TAPER_X0] = (child.v[TAPER_X0] + rng.normal() * 0.8 * scale)
                        .clamp(b[TAPER_X0].0, b[TAPER_X0].1);
                    child.v[T_TIP] = (child.v[T_TIP] + rng.normal() * 0.03 * scale)
                        .clamp(b[T_TIP].0, b[T_TIP].1);
                    child.v[FUL_D] = (child.v[FUL_D] + rng.normal() * 0.015 * scale)
                        .clamp(b[FUL_D].0, b[FUL_D].1);
                }
                (2, false, false, false) => {
                    // Bevel & micro-bevel apex refinement within Gate S law (edge_t <= 0.20 mm)
                    child.v[EDGE_TH] =
                        (child.v[EDGE_TH] + rng.normal() * 0.015 * scale).clamp(0.12, 0.20);
                    child.v[EA_IN] = (child.v[EA_IN] + rng.normal() * 0.15 * scale)
                        .clamp(b[EA_IN].0, b[EA_IN].1);
                    child.v[EA_SP] = (child.v[EA_SP] + rng.normal() * 0.15 * scale)
                        .clamp(b[EA_SP].0, b[EA_SP].1);
                    if round == 0 {
                        child.v[EA_IN] = rng.range(b[EA_IN].0, b[EA_IN].1);
                        child.v[EA_SP] = rng.range(b[EA_SP].0, b[EA_SP].1);
                        child.v[genome::RING_WALL] =
                            rng.range(6.0, b[genome::RING_WALL].1);
                    }
                }
                (3, false, false, false) => {
                    // Thickness knot fine-tuning (K0..K0+4)
                    for idx in 0..5 {
                        let g_idx = K0 + idx;
                        child.v[g_idx] = (child.v[g_idx] + rng.normal() * 0.025 * scale)
                            .clamp(b[g_idx].0, b[g_idx].1);
                    }
                }
                (4, false, false, false) => {
                    // Outline control point micro-sculpting (within aesthetic Hausdorff corridor)
                    for _ in 0..3 {
                        let cp = ((rng.f64() * (N_CP as f64)) as usize).min(N_CP - 1);
                        child.v[cp] =
                            (child.v[cp] + rng.normal() * 0.06 * scale).clamp(b[cp].0, b[cp].1);
                    }
                }
                (5, false, false, false) => {
                    // Multi-gene Gaussian + optional crossover with another elite
                    if k % 12 != 5 && elites.len() > 1 && rng.f64() < 0.5 {
                        let pick =
                            ((rng.f64() * (elites.len() as f64)) as usize).min(elites.len() - 1);
                        let other = &elites[pick].1;
                        let alpha = 0.15 + 0.35 * rng.f64();
                        for idx in 0..NG {
                            if idx != K_N && rng.f64() < 0.35 {
                                child.v[idx] = child.v[idx] * (1.0 - alpha) + other.v[idx] * alpha;
                            }
                        }
                    } else {
                        if k % 12 == 5 {
                            // Preserve exploration after formerly "certified" seeds become failures.
                            // All changes use the pre-existing genotype domain and clamp rules.
                            child = child.mutate(rng, 0.08);
                            child.v[genome::T] = rng.range(b[genome::T].0, b[genome::T].1);
                            child.v[genome::P1_ON] = if parent.p1() { 0.0 } else { 1.0 };
                        } else {
                            child = child.mutate(rng, 0.008 * scale);
                        }
                    }
                }
                (_, false, false, false) => {
                    // Increase section modulus through thickness, taper and grind angles.
                    let reinforcement = if k % 8 == 6 {
                        rng.range(0.5, 0.9)
                    } else {
                        1.0
                    };
                    for idx in [
                        genome::T,
                        T_TIP,
                        TAPER_X0,
                        K0,
                        K0 + 1,
                        K0 + 2,
                        K0 + 3,
                        K0 + 4,
                        EA_IN,
                        EA_SP,
                    ] {
                        child.v[idx] += reinforcement * (b[idx].1 - child.v[idx]);
                    }
                    for idx in [
                        FUL_D,
                        POCKET_D,
                        genome::ISO_D,
                        genome::LAT_D,
                        genome::FLUTE,
                        genome::WIN_N,
                    ] {
                        child.v[idx] *= 1.0 - reinforcement;
                    }
                    // Reduce outline offsets to leave holder-envelope space for the bore collar.
                    for offset in &mut child.v[..N_CP] {
                        *offset *= 1.0 - reinforcement;
                    }
                }
                (_, false, true, _) | (_, false, _, true) => {
                    unreachable!("Q7 slot selection must bypass the legacy arms")
                }
            }

            }

            if fam == "p14" && k % 2 == 0 {
                child.v[COMET_W0] = parent.v[COMET_W0];
                child.v[COMET_BOW] = parent.v[COMET_BOW];
                child.v[FUL_S0] = parent.v[FUL_S0];
            }
            child.v[EDGE_TH] = child.v[EDGE_TH].clamp(0.12, 0.20);
            if child.rs_v2() {
                child.v[genome::RING_WALL] =
                    child.v[genome::RING_WALL].clamp(6.0, b[genome::RING_WALL].1);
            }
            child.clamp();
            batch.push((fam.clone(), child));
        }
    }
    batch
}

