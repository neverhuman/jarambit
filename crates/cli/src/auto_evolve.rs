//! Automated multi-wave evolutionary search with ALL hard engineering gates
//! (`16/16 W_CAD_v2` + `10/10 RS-karambit-v2 all_hard` + 3D `TET10` solid-tip FEM +
//! `side.a4` buckling + 3-level Roache `GCI` + strict `0.0625 mm^2` plate + 3D watertight STEP/STL).

use jarambit_crescent::{
    aesthetic, certification, fem, gates,
    genome::{
        self, Genome, COMET_BOW, COMET_W0, EA_IN, EA_SP, EDGE_TH, FUL_D, FUL_S0, K0, K_N, NECK_L,
        NG, N_CP, POCKET_D, POCKET_R, RAIL_W, TAPER_X0, T_TIP,
    },
    pheno, qd,
};
use jarambit_genotype::Rng;
use serde_json::{json, Value};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

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

fn repo_root() -> PathBuf {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if cwd.join("docs/VERSION").is_file() {
        return cwd;
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(Path::to_path_buf)
        .unwrap_or(cwd)
}

#[derive(Clone, Debug)]
struct ScreenSurvivor {
    label: String,
    family: String,
    g: Genome,
    mass_g: f64,
    seq_ratio: f64,
    fat_sf: f64,
    ring_bearing_w_mm: f64,
    screen_deficit: f64,
}

#[derive(Clone, Debug)]
struct AssessedDesign {
    label: String,
    family: String,
    g: Genome,
    mass_g: f64,
    mass_3d_g: f64,
    w_cad: f64,
    g1: f64,
    strict_hunter_sf: f64,
    solid_tip_sf: f64,
    seq_ratio: f64,
    fat_sf: f64,
    ring_bearing_w_mm: f64,
    buckling_eig: f64,
    battery_pass: i64,
    all_16_gates: bool,
    rs_v2_all_hard: bool,
    genome_csv: PathBuf,
    scorecard_md: PathBuf,
}

fn ensure_sfmap_env(root: &Path) {
    if std::env::var("JARAMBIT_P1_SFMAP").is_err() {
        let sfmap = root.join("evidence/d1_p1/w249_sfmap.txt");
        if sfmap.is_file() {
            std::env::set_var("JARAMBIT_P1_SFMAP", sfmap);
        }
    }
}

fn load_seed_genomes(root: &Path, seeds_arg: Option<&str>) -> Vec<(String, Genome)> {
    let default_paths = [
        "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_genome.csv",
        "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_genome.csv",
        "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_genome.csv",
        "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_genome.csv",
    ];
    let paths: Vec<String> = match seeds_arg {
        Some(s) if !s.trim().is_empty() => s
            .split(',')
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect(),
        _ => default_paths
            .iter()
            .filter(|rel| root.join(rel).is_file())
            .map(|rel| (*rel).to_string())
            .collect(),
    };

    let mut out = Vec::new();
    for raw in paths {
        let p = if Path::new(&raw).is_absolute() {
            PathBuf::from(&raw)
        } else {
            root.join(&raw)
        };
        let stem = p
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("seed")
            .trim_end_matches("_genome")
            .to_string();
        let family = if stem.contains("p10") {
            "p10".to_string()
        } else if stem.contains("p14") {
            "p14".to_string()
        } else {
            stem.clone()
        };
        if let Some(mut g) = crate::crescent::load_seeds(&p.to_string_lossy(), 1)
            .into_iter()
            .next()
        {
            if !g.rs_v2() {
                g.v[EDGE_TH] = 0.18;
            }
            g.clamp();
            if !out
                .iter()
                .any(|(_, existing): &(String, Genome)| *existing == g)
            {
                out.push((family, g));
            }
        }
    }
    out
}

/// Generate a batch of targeted + stochastic `RS-karambit-v2` offspring around the current elite pool.
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
            let q7_slot = matches!(k % 11, 6..=10)
                && !(candidates_per_elite == 12 && matches!(k % 12, 7 | 8));
            if q7_slot {
                let slot = k % 11;
                child.v[..N_CP].copy_from_slice(&Q7_CPS);
                if slot != 9 {
                    for (idx, cp) in child.v[..N_CP].iter_mut().enumerate() {
                        *cp = (*cp + rng.range(-0.005, 0.005)).clamp(b[idx].0, b[idx].1);
                    }
                    child.v[TAPER_X0] = rng.range(116.0, 118.0)
                        .clamp(b[TAPER_X0].0, b[TAPER_X0].1);
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
                        child.v[T_TIP] = rng.range(2.62, 2.75);
                        child.v[genome::WIN_W] = 6.0;
                        child.v[genome::WIN_FILL] = rng.range(0.65, 0.70);
                        child.v[genome::RING_WALL] = rng.range(6.0, 6.05);
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
                    10 => {
                        child.v[genome::WIN_N] = 1.0;
                        child.v[genome::WIN_W] = 3.0;
                        child.v[genome::WIN_S0] = rng.range(0.48, 0.52);
                        child.v[genome::WIN_S1] = rng.range(0.82, 0.86);
                        child.v[genome::WIN_FILL] = 0.65;
                        // 6.4 mm is above every c9 wall while retaining holder clearance.
                        child.v[genome::RING_WALL] = rng.range(6.40, 6.41);
                        child.v[genome::GRIP_DX] = -0.4;
                    }
                    _ => unreachable!(),
                }
            } else {
            match (
                k % 8,
                k % 9 == 8 || (candidates_per_elite == 12 && k % 12 == 11),
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

/// Stage 1: provisional parallel FEM ranking. No qualification verdict is issued here.
fn screen_candidate_stage1(
    round: usize,
    idx: usize,
    family: &str,
    g: &Genome,
    prefix: &str,
) -> Option<ScreenSurvivor> {
    if !g.rs_v2() {
        return None;
    }
    let r = qd::evaluate(g, idx, round, 2.0);
    let p = pheno::build(g);
    let f = r.fem.as_ref()?;
    // Search may rank a measured shortfall. It never turns that shortfall into a qualification.
    let deficit = |value: f64, floor: f64| ((floor - value) / floor).max(0.0);
    let (bw, _) = fem::ring_bearing(&p);
    let (radius, _) = pheno::bore_dimensions(&p);
    let screen_deficit = deficit(f.lat_rob, 260.0)
        + deficit(f.lat_rob, 358.0)
        + deficit(f.sf_hiker, 1.0)
        + deficit(f.ring_rob, 1500.0)
        + deficit(f.seq_ratio, fem::SEQ_TARGET)
        + deficit(f.twist_sf, 1.0)
        + deficit(f.twist_body_sf, 1.0)
        + deficit(f.twist_tipzone_sf, 1.0)
        + deficit(f.hm_twist_sf, 1.0)
        + deficit(f.hm_spine_sf, 1.0)
        + deficit(f.hm_ringback_sf, 1.0)
        + deficit(f.fat_sf, 1.0)
        + deficit(bw, fem::hm::RING_BEARING_W_MIN_MM)
        + deficit(radius, fem::hm::RING_EDGE_R_MIN_MM);
    Some(ScreenSurvivor {
        label: format!("{prefix}_r{:02}_{family}_c{:02}", round + 1, idx + 1),
        family: family.to_string(),
        g: g.clone(),
        mass_g: p.mass_g,
        seq_ratio: f.seq_ratio,
        fat_sf: f.fat_sf,
        ring_bearing_w_mm: bw,
        screen_deficit,
    })
}

/// Stage 2: attempt the standard independent fine-mesh assessment and retain its measured verdict.
fn certify_candidate_stage2(
    root: &Path,
    out_dir: &Path,
    cand: &ScreenSurvivor,
) -> Result<AssessedDesign, String> {
    let attempt = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
    let folder = out_dir
        .join("by-content")
        .join(certification::identity(&cand.g))
        .join(format!("attempt-{attempt}"));
    let out_dir = folder.as_path();
    std::fs::create_dir_all(out_dir).map_err(|e| e.to_string())?;
    let genome_csv = out_dir.join(format!("{}_genome.csv", cand.label));
    let csv_text = format!("{}\n{}\n", Genome::csv_header(), cand.g.csv_row());
    std::fs::write(&genome_csv, csv_text).map_err(|e| e.to_string())?;

    // The native coarse screen deliberately remains unverified. Qualification dispatches
    // the standard evaluator, which executes skfem P2 and DOLFINx on one saved fine mesh.
    let python = std::env::var("JARAMBIT_SOLVER_PYTHON").unwrap_or_else(|_| "python3".into());
    let status = std::process::Command::new(&python)
        .arg(root.join("ops/ci/score_cad.py"))
        .args([
            "--genome",
            &genome_csv.to_string_lossy(),
            "--label",
            &cand.label,
            "--out-dir",
            &out_dir.to_string_lossy(),
        ])
        .current_dir(root)
        .env("OMP_NUM_THREADS", "1")
        .env("OPENBLAS_NUM_THREADS", "1")
        .env("MKL_NUM_THREADS", "1")
        .status()
        .map_err(|e| format!("independent certification unavailable: {e}"))?;
    if !status.success() {
        return Err(format!("independent certification unverified: evaluator exited {status}; raw artifacts retained in {}",out_dir.display()));
    }
    let file = out_dir.join(format!("{}_master_scorecard.json", cand.label));
    let sc: Value = serde_json::from_slice(&std::fs::read(&file).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if !certification::bound(&sc, &certification::identity(&cand.g)) {
        return Err(
            "standard evaluator returned a certificate for a different genome/model".into(),
        );
    }
    let em = sc.get("engineering_metrics").unwrap_or(&Value::Null);
    let wc = sc.get("weighted_cad_score").unwrap_or(&Value::Null);
    let g1 = sc.get("global_fitness_G1").unwrap_or(&Value::Null);
    let bat = sc.get("cad_battery").unwrap_or(&Value::Null);

    let finite = |value: &Value, name: &str| {
        value
            .as_f64()
            .filter(|x| x.is_finite())
            .ok_or_else(|| format!("standard assessment unverified: missing/nonfinite {name}"))
    };
    let all_16_gates = wc
        .get("all_engineering_gates_pass")
        .and_then(Value::as_bool)
        == Some(true);
    let w_cad = finite(&wc["W_CAD"], "W_CAD")?;
    let g1_val = finite(&g1["G"], "G1")?;
    let mass_g = finite(&em["mass_g"], "raster mass")?;
    let mass_3d_g = finite(&em["mass_3d_g"], "3D mass")?;
    let strict_hunter_sf = finite(&em["sf_hunter_strict"], "strict hunter SF")?;
    let solid_tip_sf = finite(&em["solid_tip_sf_nodal_max"], "independent solid tip SF")?;
    let seq_ratio = finite(&em["seq_ratio_cert"], "sequence ratio")?;
    let fat_sf = finite(&em["fat_sf_l7"], "fatigue SF")?;
    let ring_bearing_w_mm = finite(&em["ring_bearing_w_mm"], "bore bearing width")?;
    let buckling_eig = finite(&em["buckling_eigenvalue_a4"], "buckling eigenvalue")?;
    let battery_pass = bat
        .get("pass_count")
        .and_then(Value::as_i64)
        .ok_or("battery assessment unavailable")?;
    let rs_case_pass = em.get("rs_v2_all_hard").and_then(Value::as_bool) == Some(true);

    Ok(AssessedDesign {
        label: cand.label.clone(),
        family: cand.family.clone(),
        g: cand.g.clone(),
        mass_g,
        mass_3d_g,
        w_cad,
        g1: g1_val,
        strict_hunter_sf,
        solid_tip_sf,
        seq_ratio,
        fat_sf,
        ring_bearing_w_mm,
        buckling_eig,
        battery_pass,
        all_16_gates,
        rs_v2_all_hard: rs_case_pass,
        genome_csv,
        scorecard_md: out_dir.join(format!("{}_master_scorecard.md", cand.label)),
    })
}

/// Keep qualified genomes, then preserve each rankable seed family before filling search slots.
fn replacement_elites(
    qualified: impl Iterator<Item = (String, Genome)>,
    survivors: &[ScreenSurvivor],
    previous: &[(String, Genome)],
) -> Vec<(String, Genome)> {
    let mut next = Vec::new();
    for pair in qualified {
        if !next.iter().any(|(_, g)| *g == pair.1) {
            next.push(pair);
        }
    }
    let mut provisional: Vec<(String, Genome)> = Vec::new();
    // Match Stage-2's family-first policy. Four global leaders can all be near-identical
    // members of one family, permanently starving the others in subsequent rounds.
    for s in survivors {
        if !provisional.iter().any(|(family, _)| *family == s.family)
            && !next.iter().chain(&provisional).any(|(_, g)| *g == s.g)
        {
            provisional.push((s.family.clone(), s.g.clone()));
        }
    }
    // Count distinct genomes, rather than taking four rows and deduplicating afterwards.
    // With more than four available families, keep their representatives too.
    for s in survivors {
        if provisional.len() >= 4 {
            break;
        }
        if !next.iter().chain(&provisional).any(|(_, g)| *g == s.g) {
            provisional.push((s.family.clone(), s.g.clone()));
        }
    }
    next.extend(provisional);
    if next.is_empty() {
        previous.to_vec()
    } else {
        next
    }
}

/// CLI entrypoint for `jarambit auto-evolve`.
pub fn run_auto_evolve_cli(args: &[String]) -> i32 {
    let root = repo_root();
    ensure_sfmap_env(&root);

    let mut rounds = 2usize;
    let mut candidates_per_round = 12usize;
    let mut certify_top = 2usize;
    let mut threads = jarambit_monte_carlo::max_threads().min(16);
    let mut seed = 2830u64;
    let mut seeds_arg: Option<String> = None;
    let mut out_dir = root.join("out/auto_evolve");
    let mut prefix = String::from("auto");

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--rounds" | "--waves" => {
                if let Some(v) = args.get(i + 1).and_then(|x| x.parse::<usize>().ok()) {
                    rounds = v.max(1);
                    i += 1;
                }
            }
            "--candidates-per-round" | "--pop" => {
                if let Some(v) = args.get(i + 1).and_then(|x| x.parse::<usize>().ok()) {
                    candidates_per_round = v.max(1);
                    i += 1;
                }
            }
            "--certify-top" | "--top-k-certify" => {
                if let Some(v) = args.get(i + 1).and_then(|x| x.parse::<usize>().ok()) {
                    certify_top = v.max(1);
                    i += 1;
                }
            }
            "--threads" => {
                if let Some(v) = args.get(i + 1).and_then(|x| x.parse::<usize>().ok()) {
                    threads = v.max(1).min(jarambit_monte_carlo::max_threads());
                    i += 1;
                }
            }
            "--seed" => {
                if let Some(v) = args.get(i + 1).and_then(|x| x.parse::<u64>().ok()) {
                    seed = v;
                    i += 1;
                }
            }
            "--seeds" => {
                if let Some(v) = args.get(i + 1) {
                    seeds_arg = Some(v.clone());
                    i += 1;
                }
            }
            "--out-dir" | "--out" => {
                if let Some(v) = args.get(i + 1) {
                    out_dir = PathBuf::from(v);
                    i += 1;
                }
            }
            "--prefix" => {
                if let Some(v) = args.get(i + 1) {
                    prefix = v.clone();
                    i += 1;
                }
            }
            "-h" | "--help" => {
                println!(
                    "Usage: jarambit auto-evolve [--rounds N] [--candidates-per-round K] [--top-k-certify [M]] [--threads T] [--seeds <csv1,csv2>] [--out-dir <dir>] [--prefix <name>] [--seed <u64>]"
                );
                return 0;
            }
            _ => {}
        }
        i += 1;
    }

    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        eprintln!("auto-evolve: cannot create {}: {e}", out_dir.display());
        return 1;
    }

    let mut elites = load_seed_genomes(&root, seeds_arg.as_deref());
    if elites.is_empty() {
        eprintln!("auto-evolve: supply at least one valid committed/evolved genome CSV; no external CAD fallback");
        return 1;
    }
    // A direct CLI invocation prepares the same tools that evolve.sh builds up front.
    let examples = root.join("target/release/examples");
    {
        let _ = &examples; // Cargo checks fingerprints even when executables already exist.
        let built = std::process::Command::new("cargo")
            .args([
                "build",
                "--release",
                "--locked",
                "-p",
                "jarambit-crescent",
                "--example",
                "export_step",
                "--example",
                "plate_gci_probe",
                "--example",
                "g3_side_report",
            ])
            .current_dir(&root)
            .env("CARGO_INCREMENTAL", "0")
            .env("CARGO_PROFILE_DEV_DEBUG", "line-tables-only")
            .status();
        if !matches!(built,Ok(s) if s.success()) {
            eprintln!("auto-evolve: certification tool build failed; no certificate issued");
            return 1;
        }
    }
    let mut rng = Rng(seed);
    let mut assessed_all: Vec<AssessedDesign> = Vec::new();
    let mut errors = Vec::new();

    println!(
        "=== JARAMBIT AUTO-EVOLVE: ranked search, independent top-k assessment, qualification requires every hard gate ==="
    );
    println!(
        "Config: rounds={rounds}, candidates/elite/round={candidates_per_round}, certify_top={certify_top}, threads={threads}, seed={seed}, out={}",
        out_dir.display()
    );

    for round in 0..rounds {
        let batch = spawn_offspring_batch(&elites, candidates_per_round, round, &mut rng);
        let births = json!({"round":round,"seed":seed,"source":"spawn_offspring_batch from evolved genome CSV seeds",
            "genome_header":Genome::csv_header(),"genomes":batch.iter().map(|(family,g)|json!({"family":family,"row":g.csv_row(),"certificate_identity":certification::identity(g)})).collect::<Vec<_>>()});
        if let Err(e) = std::fs::write(
            out_dir.join(format!("round_{round:02}_births.json")),
            serde_json::to_string_pretty(&births).unwrap() + "\n",
        ) {
            eprintln!("auto-evolve: cannot retain birth history: {e}");
            return 1;
        }
        let th = threads.max(1).min(batch.len().max(1));
        let prefix_ref = prefix.as_str();

        let mut survivors: Vec<ScreenSurvivor> = std::thread::scope(|s| {
            let handles: Vec<_> = (0..th)
                .map(|t| {
                    let batch_ref = &batch;
                    s.spawn(move || {
                        batch_ref
                            .iter()
                            .enumerate()
                            .filter(|(idx, _)| idx % th == t)
                            .filter_map(|(idx, (fam, g))| {
                                screen_candidate_stage1(round, idx, fam, g, prefix_ref)
                            })
                            .collect::<Vec<_>>()
                    })
                })
                .collect();
            handles
                .into_iter()
                .flat_map(|h| h.join().expect("stage1 worker thread"))
                .collect()
        });

        // Sort Stage-1 survivors by lightest mass / highest estimated W_CAD_v2
        survivors.sort_by(|a, b| {
            a.screen_deficit
                .partial_cmp(&b.screen_deficit)
                .filter(|o| !o.is_eq())
                .or_else(|| a.mass_g.partial_cmp(&b.mass_g))
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        println!(
            "\n[Round {}/{}] Stage-1 ranking: {}/{} candidates have valid geometry and a FEM screen; qualification pending",
            round + 1,
            rounds,
            survivors.len(),
            batch.len()
        );

        let ranked:Vec<_>=survivors.iter().map(|s|json!({"label":s.label,"family":s.family,"certificate_identity":certification::identity(&s.g),
            "mass_g":s.mass_g,"seq_ratio":s.seq_ratio,"fat_sf":s.fat_sf,"ring_bearing_w_mm":s.ring_bearing_w_mm,
            "screen_deficit":s.screen_deficit,"qualification_status":"pending independent top-k assessment"})).collect();
        if let Err(e) = std::fs::write(
            out_dir.join(format!("round_{round:02}_screen.json")),
            serde_json::to_string_pretty(&ranked).unwrap() + "\n",
        ) {
            eprintln!("auto-evolve: cannot retain screen observations: {e}");
            return 1;
        }

        // Select up to `certify_top` distinct candidates (one per family first, then lightest overall)
        let mut to_certify: Vec<ScreenSurvivor> = Vec::new();
        for surv in &survivors {
            if to_certify.len() >= certify_top {
                break;
            }
            let already_certified = assessed_all.iter().any(|c| c.g == surv.g);
            let dup_fam = to_certify.iter().any(|c| c.family == surv.family);
            if !already_certified && !dup_fam {
                to_certify.push(surv.clone());
            }
        }
        for surv in &survivors {
            if to_certify.len() >= certify_top {
                break;
            }
            let already_certified = assessed_all.iter().any(|c| c.g == surv.g);
            let dup_in_batch = to_certify.iter().any(|c| c.g == surv.g);
            if !already_certified && !dup_in_batch {
                to_certify.push(surv.clone());
            }
        }

        for cand in &to_certify {
            println!(
                "  -> Stage-2 full 3D TET10 + side.a4 + 0.0625 plate + GCI certification for {} (family={}, screen_mass={:.3}g, seq={:.3}, fat_sf={:.3}, ring_w={:.2}mm)...",
                cand.label,
                cand.family,
                cand.mass_g,
                cand.seq_ratio,
                cand.fat_sf,
                cand.ring_bearing_w_mm
            );
            match certify_candidate_stage2(&root, &out_dir, cand) {
                Ok(cert) => {
                    println!(
                        "     [ASSESSED] {} | 16_gates={} | RS-v2_all_hard={} | battery={}/18 | mass={:.3}g (3D {:.3}g) | W_CAD={:+.6} | G1={:.6} | solid_tip_SF={:.4} | strict_hunter_SF={:.4}",
                        cert.label,
                        if cert.all_16_gates { "PASS" } else { "FAIL" },
                        if cert.rs_v2_all_hard { "PASS" } else { "FAIL" },
                        cert.battery_pass,
                        cert.mass_g,
                        cert.mass_3d_g,
                        cert.w_cad,
                        cert.g1,
                        cert.solid_tip_sf,
                        cert.strict_hunter_sf
                    );
                    assessed_all.push(cert);
                }
                Err(e) => {
                    eprintln!("     [STAGE-2 UNVERIFIED] {}: {e}", cand.label);
                    errors.push(json!({"label":cand.label,"status":"unverified","reason":e,"certificate_identity":certification::identity(&cand.g)}));
                }
            }
        }

        elites = replacement_elites(
            assessed_all.iter().filter(|c| c.all_16_gates && c.rs_v2_all_hard)
                .map(|c| (c.family.clone(), c.g.clone())),
            &survivors, &elites,
        );
    }

    assessed_all.sort_by(|a, b| {
        a.mass_g
            .partial_cmp(&b.mass_g)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let qualified: Vec<_> = assessed_all
        .iter()
        .filter(|c| c.all_16_gates && c.rs_v2_all_hard)
        .collect();
    let mut md=String::from("# Evolution certification attempts\n\nSearch rank is provisional. Qualification requires all 16 engineering gates and the complete measured RS-v2 verdict, including both independent fine-mesh solvers.\n\n| Design | Qualified | Mass g | W_CAD | G1 | Solid tip SF | RS-v2 all_hard | Battery |\n|---|---|---:|---:|---:|---:|---|---|\n");
    let mut rows = Vec::new();
    for c in &assessed_all {
        let qualifies = c.all_16_gates && c.rs_v2_all_hard;
        let _ = writeln!(
            md,
            "| `{}` | {} | {:.3} | {:+.6} | {:.6} | {:.4} | {} | {}/18 |",
            c.label,
            qualifies,
            c.mass_g,
            c.w_cad,
            c.g1,
            c.solid_tip_sf,
            c.rs_v2_all_hard,
            c.battery_pass
        );
        rows.push(json!({"label":c.label,"family":c.family,"qualified":qualifies,
            "mass_g":c.mass_g,"mass_3d_g":c.mass_3d_g,"w_cad":c.w_cad,"g1":c.g1,
            "solid_tip_sf":c.solid_tip_sf,"strict_hunter_sf":c.strict_hunter_sf,"seq_ratio":c.seq_ratio,
            "fat_sf":c.fat_sf,"ring_bearing_w_mm":c.ring_bearing_w_mm,"buckling_eigenvalue_a4":c.buckling_eig,
            "battery_pass_count":c.battery_pass,"all_16_gates":c.all_16_gates,"rs_v2_all_hard":c.rs_v2_all_hard,
            "certificate_identity":certification::identity(&c.g),"genome_csv":c.genome_csv,"scorecard_md":c.scorecard_md}));
    }
    let champion = rows.iter().find(|r| r["qualified"] == true).cloned();
    let result = json!({"schema":"jarambit.auto_evolve.leaderboard.v2","champion":champion,
        "qualified_count":qualified.len(),"assessed_count":rows.len(),"designs":rows,"unverified":errors});
    if let Err(e) = std::fs::write(out_dir.join("leaderboard.md"), &md).and_then(|_| {
        std::fs::write(
            out_dir.join("leaderboard.json"),
            serde_json::to_string_pretty(&result).unwrap() + "\n",
        )
    }) {
        eprintln!("auto-evolve: cannot save evaluation results: {e}");
        return 1;
    }
    println!(
        "{md}\nCompleted assessments: {}; qualified: {}; unverified: {}",
        assessed_all.len(),
        qualified.len(),
        errors.len()
    );
    if assessed_all.is_empty() || !errors.is_empty() {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn failed_strength_screen_remains_available_for_top_k_assessment() {
        let root = repo_root();
        let seeds = load_seed_genomes(
            &root,
            Some("evidence/rs_v2/w283_ultrarunner/w283_p14_c01_genome.csv"),
        );
        let candidate = screen_candidate_stage1(0, 0, &seeds[0].0, &seeds[0].1, "test")
            .expect("rankable failed candidate");
        assert!(candidate.screen_deficit > 0.0);
    }

    #[test]
    fn stage1_screen_deficit_penalizes_sub_358n_lateral_shortfall() {
        let seeds = load_seed_genomes(
            &repo_root(),
            Some("evidence/rs_v2/w283_ultrarunner/w283_p14_c01_genome.csv"),
        );
        let (family, genome) = &seeds[0];
        let candidate = screen_candidate_stage1(0, 0, family, genome, "test")
            .expect("seed should be rankable in Stage 1");
        let fem = qd::evaluate(genome, 0, 0, 2.0)
            .fem
            .expect("seed should have a FEM result");
        assert!(fem.lat_rob < 358.0);

        let deficit = |value: f64, floor: f64| ((floor - value) / floor).max(0.0);
        let p = pheno::build(genome);
        let (bearing_width, _) = fem::ring_bearing(&p);
        let (bore_radius, _) = pheno::bore_dimensions(&p);
        let prior_deficit = deficit(fem.lat_rob, 260.0)
            + deficit(fem.sf_hiker, 1.0)
            + deficit(fem.ring_rob, 1500.0)
            + deficit(fem.seq_ratio, fem::SEQ_TARGET)
            + deficit(fem.twist_sf, 1.0)
            + deficit(fem.twist_body_sf, 1.0)
            + deficit(fem.twist_tipzone_sf, 1.0)
            + deficit(fem.hm_twist_sf, 1.0)
            + deficit(fem.hm_spine_sf, 1.0)
            + deficit(fem.hm_ringback_sf, 1.0)
            + deficit(fem.fat_sf, 1.0)
            + deficit(bearing_width, fem::hm::RING_BEARING_W_MIN_MM)
            + deficit(bore_radius, fem::hm::RING_EDGE_R_MIN_MM);

        let expected = prior_deficit + deficit(fem.lat_rob, 358.0);
        assert!((candidate.screen_deficit - expected).abs() < 1e-12);
        assert!(candidate.screen_deficit > prior_deficit);
    }

    #[test]
    fn witness_and_q7_offspring_arms_produce_rankable_stage1_survivors() {
        let seeds = load_seed_genomes(
            &repo_root(),
            Some(
                "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_genome.csv,evidence/rs_v2/w283_ultrarunner/w283_p10_c02_genome.csv",
            ),
        );
        assert_eq!(seeds.len(), 2);
        let mut unrankable = Vec::new();

        for rng_seed in 0..2 {
            let batch = spawn_offspring_batch(
                &seeds,
                11,
                1,
                &mut Rng(20261018 + rng_seed),
            );
            for (seed_idx, (family, _)) in seeds.iter().enumerate() {
                let c6_idx = seed_idx * 11 + 6;
                let (_, c6_genome) = &batch[c6_idx];
                let c6_pheno = pheno::build(c6_genome);
                let c6_aes = aesthetic::evaluate(c6_genome, &c6_pheno);
                let c6_rejects = gates::check(c6_genome, &c6_pheno);
                assert!((0.447..=0.493).contains(&c6_genome.v[genome::WIN_S0]));
                assert!(c6_genome.v[POCKET_D] >= 0.12);
                assert!(
                    c6_aes.pass && c6_rejects.is_empty(),
                    "rng={rng_seed} {family} c=6: aesthetics {:?}; rejects {:?}",
                    c6_aes.fails,
                    c6_rejects,
                );
                for c in 7..=10 {
                    let idx = seed_idx * 11 + c;
                    let (_, genome) = &batch[idx];
                    let phenotype = pheno::build(genome);
                    let aes = aesthetic::evaluate(genome, &phenotype);
                    let rejects = gates::check(genome, &phenotype);
                    assert!(genome.v[T_TIP] > 2.60, "{family} c={c} T_TIP at lower bound");
                    assert!(genome.v[TAPER_X0] > 115.0, "{family} c={c} TAPER_X0 at lower bound");
                    if !aes.pass || !rejects.is_empty() {
                        unrankable.push(format!(
                            "rng={rng_seed} {family} c={c}: aesthetics {:?}; rejects {:?}; dx={} wall={}",
                            aes.fails,
                            rejects,
                            genome.v[genome::GRIP_DX],
                            genome.v[genome::RING_WALL],
                        ));
                    }
                    if rng_seed == 0 {
                        let Some(survivor) =
                            screen_candidate_stage1(1, idx, family, genome, "test")
                        else {
                            unrankable.push(format!(
                                "{family} c={c}: stage 1 returned no survivor"
                            ));
                            continue;
                        };
                        assert!(survivor.screen_deficit.is_finite());
                        assert!(survivor.ring_bearing_w_mm >= 12.0);
                        assert!(survivor.g.v[genome::RING_WALL] >= 6.0);
                    }
                }
            }
        }
        assert!(unrankable.is_empty(), "unrankable offspring: {unrankable:?}");
    }

    #[test]
    fn default_per_elite_12_keeps_witness_and_q7_slots_reachable() {
        let mut parent = Genome::c4_seed(genome::T_MIN);
        parent.v[TAPER_X0] = 130.0;
        parent.v[EA_IN] = 18.0;
        parent.v[EDGE_TH] = 0.18;
        parent.clamp();
        let children = spawn_offspring_batch(
            &[("p14".into(), parent.clone())],
            12,
            1,
            &mut Rng(20261018),
        );

        // Slot 6 remains Q7, slot 7 reaches the legacy reinforcement fallback,
        // slot 8 reaches the window witness, and slot 11 reaches the remapped witness.
        assert_ne!(&children[6].1.v[..N_CP], &Q7_CPS);
        assert_ne!(&children[7].1.v[..N_CP], &Q7_CPS);
        assert_eq!(children[7].1.v[TAPER_X0], genome::bounds()[TAPER_X0].1);
        assert!(children[8].1.v[TAPER_X0] > parent.v[TAPER_X0]);
        assert!(children[11].1.v[TAPER_X0] > parent.v[TAPER_X0]);
        assert_eq!(children[9].1.v[genome::WIN_N], 1.0);
        assert_eq!(children[10].1.v[genome::WIN_N], 1.0);
        assert_eq!(&children[9].1.v[..N_CP], &Q7_CPS);
    }

    #[test]
    fn q7_offspring_micro_jitter_is_distinct_and_keeps_slot_9_anchor() {
        let mut first = Genome::c4_seed(genome::T_MIN);
        first.v[EDGE_TH] = 0.18;
        first.clamp();
        let mut second = first.clone();
        second.v[TAPER_X0] = 132.0;
        second.clamp();
        let elites = vec![("p14".into(), first), ("p10".into(), second)];
        let bounds = genome::bounds();

        for rng_seed in [20261018, 20261019] {
            let batch = spawn_offspring_batch(&elites, 11, 1, &mut Rng(rng_seed));
            let mut non_anchor_vectors = Vec::new();
            for elite_idx in 0..elites.len() {
                for slot in 6..=10 {
                    let child = &batch[elite_idx * 11 + slot].1;
                    if slot == 9 {
                        assert_eq!(&child.v[..N_CP], &Q7_CPS);
                    } else {
                        non_anchor_vectors.push(child.v[..N_CP].to_vec());
                    }
                    for (idx, &value) in child.v[..N_CP].iter().enumerate() {
                        assert!((bounds[idx].0..=bounds[idx].1).contains(&value));
                    }
                }
            }
            for i in 0..non_anchor_vectors.len() {
                for j in i + 1..non_anchor_vectors.len() {
                    assert_ne!(non_anchor_vectors[i], non_anchor_vectors[j]);
                }
            }
        }
    }

    #[test]
    fn offspring_can_explore_stock_and_rail_architecture_inside_existing_bounds() {
        let seeds = load_seed_genomes(
            &repo_root(),
            Some("evidence/rs_v2/w283_ultrarunner/w283_p14_c01_genome.csv,evidence/rs_v2/w283_ultrarunner/w283_p10_c02_genome.csv"),
        );
        let bounds = genome::bounds();
        for round in [0, 1, 8] {
            let batch = spawn_offspring_batch(&seeds, 64, round, &mut Rng(20261017 + round as u64));
            assert!(batch.iter().any(|(_, g)| g.p1() != seeds[0].1.p1()));
            assert!(batch
                .iter()
                .any(|(_, g)| g.v[genome::T] != seeds[0].1.v[genome::T]));
            for (idx, (_, g)) in batch.iter().enumerate() {
                assert!(g.rs_v2());
                if round != 0 || !seeds.iter().any(|(_, parent)| parent == g) {
                    assert!((6.0..=bounds[genome::RING_WALL].1).contains(&g.v[genome::RING_WALL]));
                }
                for (i, &value) in g.v.iter().enumerate() {
                    if i == EDGE_TH {
                        // Genome::clamp preserves the RS-v2 apex below the legacy edge bounds.
                        assert!((0.12..=0.20).contains(&value));
                    } else {
                        assert!(
                            (bounds[i].0..=bounds[i].1).contains(&value),
                            "round {round}, candidate {idx}, gene {i}: {value}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn even_p14_offspring_preserve_internal_boundary_genes() {
        let seeds = load_seed_genomes(
            &repo_root(),
            Some("evidence/rs_v2/w283_ultrarunner/w283_p14_c01_genome.csv"),
        );
        let (family, parent) = &seeds[0];
        assert_eq!(family, "p14");

        let children = spawn_offspring_batch(
            std::slice::from_ref(&seeds[0]),
            32,
            1,
            &mut Rng(51815),
        );
        assert_eq!(children.len(), 32);
        for (index, (_, child)) in children.iter().enumerate() {
            if index % 2 == 0 {
                for gene in [COMET_W0, COMET_BOW, FUL_S0] {
                    assert_eq!(
                        child.v[gene], parent.v[gene],
                        "even p14 child {index} changed internal-boundary gene {gene}"
                    );
                }
            }
        }
    }

    #[test]
    fn witness_guided_offspring_remain_clamped_and_bore_compliant() {
        let mut parent = Genome::c4_seed(genome::T_MIN);
        parent.v[TAPER_X0] = 120.0;
        parent.v[EA_IN] = 16.0;
        parent.v[genome::WIN_N] = 2.0;
        parent.v[genome::WIN_W] = 4.0;
        parent.v[genome::WIN_FILL] = 0.5;
        parent.v[genome::RING_WALL] = 6.0;
        parent.v[EDGE_TH] = 0.18;
        parent.clamp();

        let children = spawn_offspring_batch(
            &[("p14".into(), parent.clone())],
            9,
            1,
            &mut Rng(20261018),
        );
        let child = &children[8].1;
        let bounds = genome::bounds();

        assert_ne!(&child.v[..N_CP], &Q7_CPS);
        assert!((2.62..=2.75).contains(&child.v[T_TIP]));
        assert!((116.0..=118.0).contains(&child.v[TAPER_X0]));
        assert_eq!(child.v[genome::EA_IN], 18.0);
        assert_eq!(child.v[genome::EA_SP], 18.0);
        assert_eq!(child.v[genome::WIN_W], 6.0);
        assert!((0.65..=0.70).contains(&child.v[genome::WIN_FILL]));
        assert!(child.v[genome::RING_WALL] >= 6.0);
        assert!((0.12..=0.20).contains(&child.v[EDGE_TH]));
        for (idx, &value) in child.v.iter().enumerate() {
            if idx != EDGE_TH {
                assert!(
                    (bounds[idx].0..=bounds[idx].1).contains(&value),
                    "gene {idx} outside legal bounds: {value}"
                );
            }
        }
    }

    #[test]
    fn admissible_gci_window_targets_are_steered_and_legal() {
        let mut parent = Genome::c4_seed(genome::T_MIN);
        parent.v[genome::WIN_N] = 2.0;
        parent.v[genome::WIN_S0] = 0.25;
        parent.v[genome::WIN_S1] = 0.75;
        parent.v[genome::WIN_W] = 4.0;
        parent.v[genome::WIN_FILL] = 0.5;
        parent.v[genome::RING_WALL] = 6.0;
        parent.v[EDGE_TH] = 0.18;
        parent.clamp();

        let children = spawn_offspring_batch(
            &[("p14".into(), parent)],
            27,
            1,
            &mut Rng(20261018),
        );
        let child = &children[26].1;
        let bounds = genome::bounds();

        assert!((0.05..=0.052).contains(&child.v[genome::WIN_S0]));
        assert!((0.502..=0.525).contains(&child.v[genome::WIN_S1]));
        assert_eq!(child.v[genome::WIN_W], 6.0);
        assert!((0.65..=0.70).contains(&child.v[genome::WIN_FILL]));
        assert!(child.v[genome::WIN_S1] >= child.v[genome::WIN_S0] + 0.2);
        assert!(child.v[genome::RING_WALL] >= 6.0);

        for (idx, &value) in child.v.iter().enumerate() {
            if idx == EDGE_TH {
                assert!((0.12..=0.20).contains(&value));
            } else {
                assert!(
                    (bounds[idx].0..=bounds[idx].1).contains(&value),
                    "gene {idx} outside legal bounds: {value}"
                );
            }
        }
    }

    #[test]
    fn q7_split_hand_blade_offspring_arm_is_steered_and_legal() {
        let mut parent = Genome::c4_seed(genome::T_MIN);
        parent.v[genome::WIN_N] = 2.0;
        parent.v[genome::WIN_S0] = 0.25;
        parent.v[genome::WIN_S1] = 0.75;
        parent.v[genome::WIN_W] = 4.0;
        parent.v[genome::WIN_FILL] = 0.5;
        parent.v[genome::RING_WALL] = 6.0;
        parent.v[EDGE_TH] = 0.18;
        parent.clamp();

        let children = spawn_offspring_batch(
            &[("p14".into(), parent)],
            10,
            1,
            &mut Rng(20261018),
        );
        let child = &children[9].1;
        let bounds = genome::bounds();

        assert_eq!(child.v[genome::WIN_N], 1.0);
        assert_eq!(child.v[genome::WIN_W], 3.0);
        assert_eq!(&child.v[..N_CP], &Q7_CPS);
        assert!((0.48..=0.52).contains(&child.v[genome::WIN_S0]));
        assert!((0.82..=0.86).contains(&child.v[genome::WIN_S1]));
        assert!((2.62..=2.85).contains(&child.v[T_TIP]));
        assert!((6.15..=6.35).contains(&child.v[genome::RING_WALL]));
        for (idx, &value) in child.v.iter().enumerate() {
            if idx == EDGE_TH {
                assert!((0.12..=0.20).contains(&value));
            } else {
                assert!(
                    (bounds[idx].0..=bounds[idx].1).contains(&value),
                    "gene {idx} outside legal bounds: {value}"
                );
            }
        }
    }

    #[test]
    fn q7_armb_distal_window_offspring_stays_in_bounds_and_passes_gates() {
        let mut parent = Genome::c4_seed(genome::T_MIN);
        parent.v[EDGE_TH] = 0.18;
        parent.clamp();
        let children = spawn_offspring_batch(
            &[("armB".into(), parent)],
            8,
            1,
            &mut Rng(20261018),
        );
        let child = &children[7].1;
        let bounds = genome::bounds();

        assert_eq!(child.v[genome::WIN_N], 1.0);
        assert!((0.495..0.5).contains(&child.v[genome::WIN_S0]));
        assert!((3.01..3.10).contains(&child.v[genome::WIN_W]));
        assert!((143.5..144.5).contains(&child.v[TAPER_X0]));
        assert!((2.78..2.82).contains(&child.v[T_TIP]));
        assert!((6.38..6.42).contains(&child.v[genome::RING_WALL]));
        assert!((24.91..24.99).contains(&child.v[genome::RING_ID]));
        for (idx, &value) in child.v.iter().enumerate() {
            if idx == EDGE_TH {
                assert!((0.12..=0.20).contains(&value));
            } else {
                assert!(
                    (bounds[idx].0..=bounds[idx].1).contains(&value),
                    "gene {idx} outside legal bounds: {value}"
                );
            }
        }
        let phenotype = pheno::build(child);
        let rejects = gates::check(child, &phenotype);
        assert!(rejects.is_empty(), "armB gate rejects: {rejects:?}");
    }

    #[test]
    fn q7_reinforced_ring_sequence_arm_is_steered_and_legal() {
        let mut parent = Genome::c4_seed(genome::T_MIN);
        parent.v[genome::WIN_N] = 2.0;
        parent.v[genome::WIN_S0] = 0.25;
        parent.v[genome::WIN_S1] = 0.75;
        parent.v[genome::WIN_W] = 4.0;
        parent.v[genome::WIN_FILL] = 0.5;
        parent.v[genome::RING_WALL] = 6.0;
        parent.v[EDGE_TH] = 0.18;
        parent.clamp();

        let children = spawn_offspring_batch(
            &[("p14".into(), parent)],
            11,
            1,
            &mut Rng(20261018),
        );
        let child = &children[10].1;
        let bounds = genome::bounds();

        assert!((6.40..=6.65).contains(&child.v[genome::RING_WALL]));
        assert_eq!(child.v[genome::WIN_N], 1.0);
        assert_eq!(child.v[genome::WIN_W], 3.0);
        assert!((0.48..=0.52).contains(&child.v[genome::WIN_S0]));
        assert!((0.82..=0.86).contains(&child.v[genome::WIN_S1]));
        assert_eq!(child.v[genome::WIN_FILL], 0.65);
        for (idx, &value) in child.v.iter().enumerate() {
            if idx == EDGE_TH {
                assert!((0.12..=0.20).contains(&value));
            } else {
                assert!(
                    (bounds[idx].0..=bounds[idx].1).contains(&value),
                    "gene {idx} outside legal bounds: {value}"
                );
            }
        }
    }

    #[test]
    fn w311_vs_w312_p14_seq_ratio_gci_classification() {
        use jarambit_crescent::gci::{compute_gci, Level, MeshRef, Observable};

        let observable = Observable {
            id: "seq".into(),
            units: "ratio".into(),
            region: "blade".into(),
            projection: "plate".into(),
            metric: "seq_ratio".into(),
        };
        let levels = vec![
            Level::new(
                MeshRef {
                    id: "m1".into(),
                    revision: "r1".into(),
                    sha256: "a".repeat(64),
                },
                0.5,
                observable.clone(),
            ),
            Level::new(
                MeshRef {
                    id: "m2".into(),
                    revision: "r1".into(),
                    sha256: "b".repeat(64),
                },
                1.0,
                observable.clone(),
            ),
            Level::new(
                MeshRef {
                    id: "m3".into(),
                    revision: "r1".into(),
                    sha256: "c".repeat(64),
                },
                2.0,
                observable,
            ),
        ];

        let w311 = compute_gci(
            1.719034772937246,
            1.789168040193865,
            1.9125229406964612,
            2.0,
            1.25,
            Some(&levels),
        );
        assert!(w311.admitted());
        assert_eq!(w311.fit_status.as_str(), "admissible_estimate");
        assert!((w311.p.unwrap() - 0.8146441858562493).abs() < 1e-12);

        let w312 = compute_gci(
            1.8133957229798907,
            1.7837767388307684,
            1.9109168441593696,
            2.0,
            1.25,
            Some(&levels),
        );
        assert!(!w312.admitted());
        assert_eq!(w312.fit_status.as_str(), "oscillatory");
    }

    #[test]
    fn round_zero_preserves_exact_parents_and_adds_bore_compliant_siblings() {
        let mut parent = Genome::c4_seed(genome::T_MIN);
        parent.v[EDGE_TH] = 0.18;
        parent.v[genome::RING_WALL] = 4.5;
        parent.v[1] = 1.0;
        parent.clamp();
        let mut compliant = parent.clone();
        compliant.v[genome::RING_WALL] = 7.0;
        let elites = vec![
            ("thin_bore".into(), parent.clone()),
            ("compliant".into(), compliant),
        ];
        let batch = spawn_offspring_batch(&elites, 0, 0, &mut Rng(20261018));
        assert_eq!(batch.len(), 3);
        assert_eq!(batch[0], elites[0]);
        let mut sibling = parent;
        sibling.v[genome::RING_WALL] = 6.05;
        sibling.v[..N_CP].fill(0.0);
        assert_eq!(batch[1], ("thin_bore".into(), sibling.clone()));
        assert_eq!(batch[2], elites[1]);
        let (radius, bearing) = pheno::bore_dimensions(&pheno::build(&sibling));
        assert!(radius >= 1.5 && bearing >= 12.0);
        assert!(spawn_offspring_batch(&elites, 0, 1, &mut Rng(20261018)).is_empty());
    }
    #[test]
    fn reinforcement_increases_sections_and_attenuates_removal_in_both_architectures() {
        let bounds = genome::bounds();
        let sections = [
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
        ];
        let removal = [
            FUL_D,
            POCKET_D,
            genome::ISO_D,
            genome::LAT_D,
            genome::FLUTE,
            genome::WIN_N,
        ];
        for p1 in [false, true] {
            let mut parent = Genome::c4_seed(genome::T_MIN);
            parent.v[EDGE_TH] = 0.18;
            parent.v[genome::P1_ON] = if p1 { 1.0 } else { 0.0 };
            parent.v[1] = 1.5;
            parent.v[10] = -1.0;
            for idx in sections {
                parent.v[idx] = bounds[idx].0;
            }
            for idx in removal {
                parent.v[idx] = bounds[idx].1 * 0.5;
            }
            parent.clamp();
            let elites = vec![("section".into(), parent.clone())];
            let batch = spawn_offspring_batch(&elites, 15, 1, &mut Rng(20261018));
            assert_eq!(
                batch,
                spawn_offspring_batch(&elites, 15, 1, &mut Rng(20261018))
            );
            for (_, child) in &batch[14..15] {
                assert_eq!(child.p1(), p1);
                assert!(child.rs_v2());
                for idx in 0..N_CP {
                    assert!(child.v[idx].abs() <= parent.v[idx].abs());
                    if parent.v[idx] != 0.0 {
                        assert!(child.v[idx].abs() < parent.v[idx].abs());
                    }
                }
                for idx in sections {
                    assert!(child.v[idx] >= parent.v[idx], "section gene {idx}");
                    assert!(child.v[idx] <= bounds[idx].1, "section bound {idx}");
                    let ceiling = if idx == genome::T || idx >= K0 {
                        bounds[genome::T].1
                    } else {
                        bounds[idx].1
                    };
                    if parent.v[idx] < ceiling {
                        assert!(child.v[idx] > parent.v[idx], "section increase {idx}");
                    }
                }
                for idx in removal {
                    assert!(child.v[idx] <= parent.v[idx], "removal gene {idx}");
                    assert!(child.v[idx] >= bounds[idx].0, "removal bound {idx}");
                }
            }
            assert_ne!(&batch[7].1.v[..N_CP], &Q7_CPS);
        }
    }
    #[test]
    fn stage1_ranks_bore_compliance_and_section_reinforcement_with_lower_deficits() {
        let seeds = load_seed_genomes(
            &repo_root(),
            Some("evidence/rs_v2/w283_ultrarunner/w283_p14_c01_genome.csv,evidence/rs_v2/w283_ultrarunner/w283_p10_c02_genome.csv"),
        );
        assert_eq!(seeds.len(), 2);
        for seed in &seeds {
            assert!(seed.1.v[..N_CP].iter().any(|&offset| offset != 0.0));
            let batch = spawn_offspring_batch(std::slice::from_ref(seed), 8, 0, &mut Rng(20261018));
            assert_eq!(&batch[0], seed);
            let screen = |idx: usize| {
                let candidate = &batch[idx];
                screen_candidate_stage1(0, idx, &candidate.0, &candidate.1, "test").unwrap_or_else(
                    || {
                        let r = qd::evaluate(&candidate.1, idx, 0, 2.0);
                        panic!(
                            "{} candidate {idx}: aesthetic failures {:?}; CAD rejects {:?}",
                            candidate.0, r.aes.fails, r.rejects
                        );
                    },
                )
            };
            let sibling = screen(1);
            let reinforced = screen(9);
            assert!(seed.1.v[genome::RING_WALL] < 6.0);
            for candidate in [&sibling, &reinforced] {
                assert!(candidate.g.v[genome::RING_WALL] >= 6.0);
                if std::ptr::eq(candidate, &reinforced) {
                    for (value, baseline) in candidate.g.v[..N_CP].iter().zip(Q7_CPS) {
                        assert!((value - baseline).abs() <= 0.04);
                    }
                } else {
                    assert!(candidate.g.v[..N_CP].iter().all(|&offset| offset == 0.0));
                }
                let p = pheno::build(&candidate.g);
                assert!(jarambit_crescent::gates::holder_fit(&p.outline).0);
                let (radius, bearing) = pheno::bore_dimensions(&p);
                assert!(radius >= 1.5 && bearing >= 12.0);
            }
            // Hold the generated outline fixed when measuring the bore deficit alone.
            let mut bore_deficient = sibling.g.clone();
            bore_deficient.v[genome::RING_WALL] = seed.1.v[genome::RING_WALL];
            let baseline = screen_candidate_stage1(0, 10, &seed.0, &bore_deficient, "test")
                .expect("rankable bore-deficient comparison at the same outline");
            eprintln!(
                "{} Stage-1 deficits: bore-deficient at fixed outline={}, bore-compliant={}, reinforced={}",
                seed.0, baseline.screen_deficit, sibling.screen_deficit, reinforced.screen_deficit
            );
            assert!(
                sibling.screen_deficit < baseline.screen_deficit,
                "bore deficit: {} >= {}",
                sibling.screen_deficit,
                baseline.screen_deficit
            );
            assert!(
                reinforced.screen_deficit < sibling.screen_deficit,
                "section deficit: {} >= {}",
                reinforced.screen_deficit,
                sibling.screen_deficit
            );
        }
    }
    #[test]
    fn qualified_elite_survives_replacement_by_four_different_provisional_genomes() {
        let seeds = load_seed_genomes(&repo_root(), Some("evidence/rs_v2/w283_ultrarunner/w283_p14_c01_genome.csv"));
        let champion = seeds[0].clone();
        let survivors: Vec<_> = (0..4).map(|i| {
            let mut g=champion.1.clone();g.v[genome::EA_IN]=14.0+i as f64;
            ScreenSurvivor {label:i.to_string(),family:"provisional".into(),g,mass_g:1.,seq_ratio:0.,fat_sf:0.,ring_bearing_w_mm:0.,screen_deficit:0.}
        }).collect();
        let next=replacement_elites(std::iter::once(champion.clone()),&survivors,&[]);
        assert_eq!(next.len(),5);assert_eq!(next[0],champion);
        let next_round=replacement_elites(std::iter::once(next[0].clone()),&[],&next);
        assert_eq!(next_round,vec![champion]);
        assert_eq!(replacement_elites(std::iter::empty(),&[],&seeds),seeds);
        let dedup=replacement_elites(seeds.clone().into_iter().chain(seeds.clone()),&[],&[]);
        assert_eq!(dedup,seeds);
    }
    fn replay_screen(host: &str, wave: &str) -> Vec<ScreenSurvivor> {
        let root = repo_root().join("evidence/evolution_plateau/2026-10-17").join(host).join("out").join(wave);
        let births: Value = serde_json::from_slice(&std::fs::read(root.join("round_00_births.json")).unwrap()).unwrap();
        let screen: Value = serde_json::from_slice(&std::fs::read(root.join("round_00_screen.json")).unwrap()).unwrap();
        screen.as_array().unwrap().iter().map(|row| {
            let label = row["label"].as_str().unwrap();
            let idx: usize = label.rsplit_once("_c").unwrap().1.parse().unwrap();
            let csv = births["genomes"][idx - 1]["row"].as_str().unwrap();
            let g = Genome::from_csv(&csv.split(',').collect::<Vec<_>>()).unwrap();
            ScreenSurvivor { label: label.into(), family: row["family"].as_str().unwrap().into(), g,
                mass_g: row["mass_g"].as_f64().unwrap(), seq_ratio: row["seq_ratio"].as_f64().unwrap(),
                fat_sf: row["fat_sf"].as_f64().unwrap(), ring_bearing_w_mm: row["ring_bearing_w_mm"].as_f64().unwrap(),
                screen_deficit: row["screen_deficit"].as_f64().unwrap() }
        }).collect()
    }
    #[test]
    fn completed_wave_screens_retain_both_seed_families_for_the_next_round() {
        for (host, wave) in [("xbabe2", "w298"), ("xbabe3", "w401c3")] {
            let screen = replay_screen(host, wave);
            // The old first-four replacement collapses to one lineage in both real waves.
            assert!(screen[..4].iter().all(|s| s.family == screen[0].family));
            let next = replacement_elites(std::iter::empty(), &screen, &[]);
            assert_eq!(next.len(), 4);
            for family in ["p10", "p14"] {
                let best = screen.iter().find(|s| s.family == family).unwrap();
                assert!(next.iter().any(|(f, g)| f == family && *g == best.g), "{wave}: lost {family}");
            }
            let offspring = spawn_offspring_batch(&next, 12, 1, &mut Rng(2980));
            for family in ["p10", "p14"] {
                assert!(offspring.iter().any(|(f, _)| f == family));
            }
        }
    }
    #[test]
    fn elite_replacement_counts_unique_genomes_and_preserves_all_available_families() {
        let mut screen = replay_screen("xbabe2", "w298");
        let mut other = screen[0].clone();
        other.family = "third".into();
        other.g.v[EA_IN] = 14.5;
        screen.push(other.clone());
        let mut fourth = other.clone();
        fourth.family = "fourth".into();
        fourth.g.v[EA_IN] = 14.75;
        screen.push(fourth.clone());
        // More available families than four require enough slots to keep one per family.
        let mut fifth = other.clone();
        fifth.family = "fifth".into();
        fifth.g.v[EA_IN] = 15.0;
        screen.push(fifth.clone());
        let first = screen[0].clone();
        screen.splice(0..0, vec![first; 4]);
        let champion = ("qualified".into(), screen[0].g.clone());
        let next = replacement_elites(std::iter::once(champion.clone()), &screen, &[]);
        assert_eq!(next[0], champion);
        assert!(next.iter().any(|(_, g)| *g == other.g));
        assert!(next.iter().any(|(_, g)| *g == fourth.g));
        assert!(next.iter().any(|(_, g)| *g == fifth.g));
        assert!(next.iter().any(|(f, _)| f == "p14"));
        assert!(next.iter().any(|(f, _)| f == "p10"));
        for (i, (_, g)) in next.iter().enumerate() {
            assert!(!next[..i].iter().any(|(_, prev)| prev == g));
        }
    }
    #[test]
    fn invalid_seed_input_has_no_external_cad_fallback() {
        assert!(load_seed_genomes(&repo_root(), Some("missing_genome.csv")).is_empty());
    }
}
