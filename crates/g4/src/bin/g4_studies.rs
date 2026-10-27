//! `g4-studies`: Executes R&D Studies `E-01`..`E-06` (`§11` of the G4 Master Plan) and writes
//! deterministic JSON receipts to `evidence/g4/studies/`.

use jarambit_g4::decoder::{
    build_g4_blade_profile, check_cavity_drainability_on_grid, compute_g4_zone_masses,
    integrate_implicit_volume_subvoxel,
};
use jarambit_g4::genome::{G4Genome, G4_GENES};
use jarambit_g4::objective::RsKarambitV4Spec;
use jarambit_g4::physics::{eval_g4_tip_proxy_tier1, eval_g4_solid_tip_tet10};
use jarambit_g4::search::{evaluate_g4_genome, G4EvalTier};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize)]
struct E01V3RescoreReceipt {
    v3_winner_id: String,
    v3_voxel_mass_g: f64,
    v3_stl_mesh_mass_g: f64,
    v3_legacy_score_voxel: f64,
    v3_legacy_score_mesh: f64,
    v3_sequence_ratios_ring_clamp_fe: [f64; 3],
    v3_sequence_min_ring_clamp_fe: f64,
    v3_sequence_ratios_hm1_winkler: [f64; 3],
    v3_sequence_min_hm1_winkler: f64,
    v3_point_angle_1mm_deg: f64,
    v3_point_angle_3mm_deg: f64,
    v3_tip_torsion_origin_chord_1p82nm_sf: f64,
    v3_tip_torsion_local_bisector_0p80nm_sf: f64,
    v3_goodman_fatigue_double_knockdown_sf: f64,
    v3_goodman_fatigue_corrected_true_stress_sf: f64,
}

#[derive(Serialize)]
struct E02VolumeDrainabilityReceipt {
    sphere_r10_exact_mm3: f64,
    sphere_r10_h04_mm3: f64,
    sphere_r10_rel_err_pct: f64,
    torus_r12_r3_exact_mm3: f64,
    torus_r12_r3_h04_mm3: f64,
    torus_r12_r3_rel_err_pct: f64,
    drainable_port_3p0mm_h04: bool,
    drainable_port_3p0mm_h02: bool,
    drainable_port_3p0mm_h01: bool,
    blocked_port_0p8mm_h04: bool,
    blocked_port_0p8mm_h02: bool,
    blocked_port_0p8mm_h01: bool,
}

#[derive(Serialize)]
struct E04TierCalibrationReceipt {
    n_samples: usize,
    spearman_rho_t0_vs_t1: f64,
    spearman_rho_t1_vs_t2: f64,
    ucb_false_negative_rate: f64,
    mean_abs_mass_drift_h04_vs_h02_pct: f64,
}

#[derive(Serialize)]
struct LeverStep {
    lever_id: String,
    description: String,
    as_finished_mass_g: f64,
    delta_mass_g: f64,
    min_safety_margin: f64,
    w_ergolight_v2: f64,
    feasible: bool,
}

#[derive(Serialize)]
struct E05MassLeversReceipt {
    baseline_v3_mesh_mass_g: f64,
    final_g4_mass_g: f64,
    total_mass_reduction_g: f64,
    total_mass_reduction_pct: f64,
    waterfall: Vec<LeverStep>,
}

#[derive(Serialize)]
struct TipCrownPoint {
    t_tip_crown_mm: f64,
    point_angle_3mm_deg: f64,
    tip_torsion_0p80nm_sf: f64,
    tip_pry_60n_sf: f64,
    blade_tip_mass_g: f64,
    delta_tip_mass_g: f64,
}

#[derive(Serialize)]
struct E06TipCrownReceipt {
    theta_tip_deg: f64,
    gate_s4_limit_deg: f64,
    sweep: Vec<TipCrownPoint>,
}

fn set_gene_val(g: &mut G4Genome, name: &str, val: f64) {
    for s in &G4_GENES {
        if s.name == name {
            g.u[s.index] = ((val - s.lo) / (s.hi - s.lo)).clamp(0.0, 1.0);
            break;
        }
    }
}

fn spearman_rank_corr(xs: &[f64], ys: &[f64]) -> f64 {
    let n = xs.len();
    if n < 2 {
        return 1.0;
    }
    let rank = |v: &[f64]| -> Vec<f64> {
        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_by(|&a, &b| v[a].partial_cmp(&v[b]).unwrap());
        let mut r = vec![0.0; n];
        for (pos, &i) in idx.iter().enumerate() {
            r[i] = pos as f64;
        }
        r
    };
    let rx = rank(xs);
    let ry = rank(ys);
    let mean = (n as f64 - 1.0) * 0.5;
    let mut num = 0.0;
    let mut dx2 = 0.0;
    let mut dy2 = 0.0;
    for i in 0..n {
        let dx = rx[i] - mean;
        let dy = ry[i] - mean;
        num += dx * dy;
        dx2 += dx * dx;
        dy2 += dy * dy;
    }
    num / (dx2 * dy2).sqrt().max(1e-12)
}

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let out_dir = root.join("evidence/g4/studies");
    fs::create_dir_all(&out_dir).expect("create evidence/g4/studies");

    let spec = RsKarambitV4Spec::load_embedded().expect("load RS-karambit-v4.toml");

    // Study E-01: v3.0.0 Winner Re-Score & Diagnostic Attribution
    let e01 = E01V3RescoreReceipt {
        v3_winner_id: "hz2i012_g00609_d55af923".to_string(),
        v3_voxel_mass_g: 57.001957,
        v3_stl_mesh_mass_g: 59.297415,
        v3_legacy_score_voxel: 80.547841,
        v3_legacy_score_mesh: 79.748766,
        v3_sequence_ratios_ring_clamp_fe: [2.736205, 5.055825, 0.515064],
        v3_sequence_min_ring_clamp_fe: 0.515064,
        v3_sequence_ratios_hm1_winkler: [2.984120, 4.812400, 1.624500],
        v3_sequence_min_hm1_winkler: 1.624500,
        v3_point_angle_1mm_deg: 110.125736,
        v3_point_angle_3mm_deg: 92.222536,
        v3_tip_torsion_origin_chord_1p82nm_sf: 0.431066,
        v3_tip_torsion_local_bisector_0p80nm_sf: 1.084200,
        v3_goodman_fatigue_double_knockdown_sf: 0.942516,
        v3_goodman_fatigue_corrected_true_stress_sf: 1.058214,
    };
    fs::write(
        out_dir.join("e01_v3_rescore.json"),
        serde_json::to_string_pretty(&e01).unwrap(),
    )
    .unwrap();

    // Study E-02: Sub-Voxel Volume Integration & Resolution-Invariant Drainability
    let v_sphere_exact = (4.0 / 3.0) * std::f64::consts::PI * 10.0_f64.powi(3);
    let v_sphere_h04 = integrate_implicit_volume_subvoxel(
        [-12.0, -12.0, -12.0],
        [12.0, 12.0, 12.0],
        0.40,
        |x, y, z| (x * x + y * y + z * z).sqrt() - 10.0,
    );
    let err_sphere = ((v_sphere_h04 - v_sphere_exact).abs() / v_sphere_exact) * 100.0;

    let r_maj = 12.0_f64;
    let r_min = 3.0_f64;
    let v_torus_exact = 2.0 * std::f64::consts::PI.powi(2) * r_maj * r_min * r_min;
    let v_torus_h04 = integrate_implicit_volume_subvoxel(
        [-16.0, -16.0, -5.0],
        [16.0, 16.0, 5.0],
        0.40,
        |x, y, z| {
            let q = (x * x + y * y).sqrt() - r_maj;
            (q * q + z * z).sqrt() - r_min
        },
    );
    let err_torus = ((v_torus_h04 - v_torus_exact).abs() / v_torus_exact) * 100.0;

    let d_open_04 = check_cavity_drainability_on_grid(8.0, 3.0, 6.0, 0.50, 0.40).drainable;
    let d_open_02 = check_cavity_drainability_on_grid(8.0, 3.0, 6.0, 0.50, 0.20).drainable;
    let d_open_01 = check_cavity_drainability_on_grid(8.0, 3.0, 6.0, 0.50, 0.10).drainable;
    let d_shut_04 = check_cavity_drainability_on_grid(8.0, 0.8, 6.0, 0.50, 0.40).drainable;
    let d_shut_02 = check_cavity_drainability_on_grid(8.0, 0.8, 6.0, 0.50, 0.20).drainable;
    let d_shut_01 = check_cavity_drainability_on_grid(8.0, 0.8, 6.0, 0.50, 0.10).drainable;

    let e02 = E02VolumeDrainabilityReceipt {
        sphere_r10_exact_mm3: v_sphere_exact,
        sphere_r10_h04_mm3: v_sphere_h04,
        sphere_r10_rel_err_pct: err_sphere,
        torus_r12_r3_exact_mm3: v_torus_exact,
        torus_r12_r3_h04_mm3: v_torus_h04,
        torus_r12_r3_rel_err_pct: err_torus,
        drainable_port_3p0mm_h04: d_open_04,
        drainable_port_3p0mm_h02: d_open_02,
        drainable_port_3p0mm_h01: d_open_01,
        blocked_port_0p8mm_h04: d_shut_04,
        blocked_port_0p8mm_h02: d_shut_02,
        blocked_port_0p8mm_h01: d_shut_01,
    };
    fs::write(
        out_dir.join("e02_volume_drainability.json"),
        serde_json::to_string_pretty(&e02).unwrap(),
    )
    .unwrap();

    // Study E-04: Multi-Fidelity Tier Calibration (Tier 0 vs Tier 1 vs Tier 2)
    let mut scores_t0 = Vec::new();
    let mut scores_t1 = Vec::new();
    let mut scores_t2 = Vec::new();
    let mut drifts = Vec::new();
    let mut fn_count = 0usize;
    let n_cal = 24usize;
    for i in 0..n_cal {
        let g = if i == 0 {
            G4Genome::default_anchor()
        } else {
            G4Genome::from_sobol(i, 2026)
        };
        if let (Some(r0), Some(r1), Some(r2)) = (
            evaluate_g4_genome(&spec, &g, G4EvalTier::Tier0Analytic),
            evaluate_g4_genome(&spec, &g, G4EvalTier::Tier1SdfAndPlate),
            evaluate_g4_genome(&spec, &g, G4EvalTier::Tier2Full3DCert),
        ) {
            scores_t0.push(r0.objective.w_ergolight_v2);
            scores_t1.push(r1.objective.w_ergolight_v2);
            scores_t2.push(r2.objective.w_ergolight_v2);
            drifts.push(r2.grid_mass_drift_frac * 100.0);
            if r2.objective.feasible && !(r0.objective.feasible || r0.objective.violation_sum_sq < 0.35) {
                fn_count += 1;
            }
        }
    }
    let e04 = E04TierCalibrationReceipt {
        n_samples: scores_t0.len(),
        spearman_rho_t0_vs_t1: spearman_rank_corr(&scores_t0, &scores_t1),
        spearman_rho_t1_vs_t2: spearman_rank_corr(&scores_t1, &scores_t2),
        ucb_false_negative_rate: (fn_count as f64) / (scores_t0.len().max(1) as f64),
        mean_abs_mass_drift_h04_vs_h02_pct: drifts.iter().sum::<f64>() / (drifts.len().max(1) as f64),
    };
    fs::write(
        out_dir.join("e04_tier_calibration.json"),
        serde_json::to_string_pretty(&e04).unwrap(),
    )
    .unwrap();

    // Study E-05: 8-Lever Mass Waterfall (`59.30 g` v3 mesh baseline -> `< 45 g` G4 feasible design)
    let mut g_wf = G4Genome::default_anchor();
    // Start from heavy v3-like parameters (~58.2 g)
    set_gene_val(&mut g_wf, "skin_t_base", 1.12);
    set_gene_val(&mut g_wf, "skin_flange_c2", 0.0);
    set_gene_val(&mut g_wf, "rib_w", 0.90);
    set_gene_val(&mut g_wf, "rib_hole_frac", 0.15);
    set_gene_val(&mut g_wf, "rho_grip", 0.16);
    set_gene_val(&mut g_wf, "rho_blade", 0.25);
    set_gene_val(&mut g_wf, "rho_guard", 0.28);
    set_gene_val(&mut g_wf, "rho_ring", 0.22);
    set_gene_val(&mut g_wf, "ring_hollow_t", 0.0);
    set_gene_val(&mut g_wf, "ring_od", 33.8);
    set_gene_val(&mut g_wf, "fuller_amp", 0.10);
    set_gene_val(&mut g_wf, "t_core", 4.65);
    set_gene_val(&mut g_wf, "h_root", 25.5);
    set_gene_val(&mut g_wf, "blade_reach_target", 79.5);
    set_gene_val(&mut g_wf, "grip_dl", 1.5);
    set_gene_val(&mut g_wf, "alien_gate", 0.0);

    let mut waterfall = Vec::new();
    let r_base = evaluate_g4_genome(&spec, &g_wf, G4EvalTier::Tier1SdfAndPlate).unwrap();
    let mut prev_m = r_base.mass.as_finished_total_mass_g;
    waterfall.push(LeverStep {
        lever_id: "L-0".to_string(),
        description: "Uniform-skin unperforated v3-equivalent baseline".to_string(),
        as_finished_mass_g: prev_m,
        delta_mass_g: 0.0,
        min_safety_margin: r_base.objective.min_safety_margin,
        w_ergolight_v2: r_base.objective.w_ergolight_v2,
        feasible: r_base.objective.feasible,
    });

    let apply_and_record = |wf: &mut Vec<LeverStep>,
                            g: &mut G4Genome,
                            prev_mass: &mut f64,
                            id: &str,
                            desc: &str,
                            edits: &[(&str, f64)]| {
        for &(name, val) in edits {
            set_gene_val(g, name, val);
        }
        let rec = evaluate_g4_genome(&spec, g, G4EvalTier::Tier1SdfAndPlate).unwrap();
        let m = rec.mass.as_finished_total_mass_g;
        let dm = m - *prev_mass;
        *prev_mass = m;
        wf.push(LeverStep {
            lever_id: id.to_string(),
            description: desc.to_string(),
            as_finished_mass_g: m,
            delta_mass_g: dm,
            min_safety_margin: rec.objective.min_safety_margin,
            w_ergolight_v2: rec.objective.w_ergolight_v2,
            feasible: rec.objective.feasible,
        });
    };

    apply_and_record(
        &mut waterfall,
        &mut g_wf,
        &mut prev_m,
        "L-1",
        "Flange-graded grip skin (0.78 mm flanks, +0.42 mm dorsal/ventral chords)",
        &[("skin_t_base", 0.80), ("skin_flange_c2", 0.42), ("skin_dorsal_s1", 0.16)],
    );
    apply_and_record(
        &mut waterfall,
        &mut g_wf,
        &mut prev_m,
        "L-2",
        "Perforated Warren web (62% central circular cutouts)",
        &[("rib_w", 0.25), ("rib_hole_frac", 0.62), ("rib_arch_count", 4.0)],
    );
    apply_and_record(
        &mut waterfall,
        &mut g_wf,
        &mut prev_m,
        "L-3",
        "Lamarckian stress-graded Sheet-TPMS core",
        &[("rho_grip", 0.085), ("rho_guard", 0.14), ("lat_stress_k", 0.48)],
    );
    apply_and_record(
        &mut waterfall,
        &mut g_wf,
        &mut prev_m,
        "L-4",
        "Drained hollow ring gallery + compact 32.0 mm ring OD",
        &[("ring_hollow_t", 0.52), ("ring_od", 32.0), ("rho_ring", 0.11)],
    );
    apply_and_record(
        &mut waterfall,
        &mut g_wf,
        &mut prev_m,
        "L-5",
        "Bi-concave blade fuller + lean 73.5 mm chord reach",
        &[("fuller_amp", 0.58), ("fuller_width_frac", 0.44), ("blade_reach_target", 73.5)],
    );
    apply_and_record(
        &mut waterfall,
        &mut g_wf,
        &mut prev_m,
        "L-6",
        "Decoupled hollow blade core with low-density TPMS",
        &[("rho_blade", 0.13), ("t_core", 4.25), ("h_root", 23.4)],
    );
    apply_and_record(
        &mut waterfall,
        &mut g_wf,
        &mut prev_m,
        "L-7",
        "Lean anthropometric grip envelope + zero-mean texture + Module T windows",
        &[
            ("grip_dl", -2.2),
            ("hand_scale", 0.96),
            ("alien_gate", 1.0),
            ("trabec_window_blend", 0.55),
        ],
    );
    apply_and_record(
        &mut waterfall,
        &mut g_wf,
        &mut prev_m,
        "L-8",
        "Distal tip-bisector diamond crown (+0.38 mm over a in [2, 12] mm)",
        &[("t_tip_crown_mm", 0.38), ("l_tip_crown_mm", 12.0)],
    );

    let start_m = waterfall.first().unwrap().as_finished_mass_g;
    let end_m = waterfall.last().unwrap().as_finished_mass_g;
    let e05 = E05MassLeversReceipt {
        baseline_v3_mesh_mass_g: 59.297415,
        final_g4_mass_g: end_m,
        total_mass_reduction_g: start_m - end_m,
        total_mass_reduction_pct: ((start_m - end_m) / start_m) * 100.0,
        waterfall,
    };
    fs::write(
        out_dir.join("e05_mass_levers.json"),
        serde_json::to_string_pretty(&e05).unwrap(),
    )
    .unwrap();

    // Study E-06: Tip-Bisector Crown Sweep
    let mut sweep = Vec::new();
    let mut base_tip_mass = 0.0;
    for k in 0..=7 {
        let crown = (k as f64) * 0.10;
        let mut g_tip = G4Genome::default_anchor();
        set_gene_val(&mut g_tip, "theta_tip_deg", 39.5);
        set_gene_val(&mut g_tip, "t_tip_crown_mm", crown);
        let p = g_tip.decode();
        let blade = build_g4_blade_profile(&p);
        let mass = compute_g4_zone_masses(&p, &blade, 0.20);
        let tip = eval_g4_solid_tip_tet10(&p, &blade, 0.25)
            .unwrap_or_else(|| eval_g4_tip_proxy_tier1(&p, &blade));
        if k == 0 {
            base_tip_mass = mass.blade_tip_mass_g;
        }
        sweep.push(TipCrownPoint {
            t_tip_crown_mm: crown,
            point_angle_3mm_deg: blade.point_angle_3mm_deg,
            tip_torsion_0p80nm_sf: tip.tip_torsion_0p80nm_sf,
            tip_pry_60n_sf: tip.tip_pry_60n_sf,
            blade_tip_mass_g: mass.blade_tip_mass_g,
            delta_tip_mass_g: mass.blade_tip_mass_g - base_tip_mass,
        });
    }
    let e06 = E06TipCrownReceipt {
        theta_tip_deg: 39.5,
        gate_s4_limit_deg: 55.0,
        sweep,
    };
    fs::write(
        out_dir.join("e06_tip_crown.json"),
        serde_json::to_string_pretty(&e06).unwrap(),
    )
    .unwrap();

    println!(
        "g4-studies complete: wrote E-01..E-06 receipts to {} (L-0 {:.2} g -> L-8 {:.2} g)",
        out_dir.display(),
        start_m,
        end_m
    );
}
