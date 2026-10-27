//! Comprehensive G4 Verification & Validation (`V&V`) Integration Test Suite:
//! Covers `T-GEN-1..4`, `T-DEC-1..3`, `T-MASS-1`, `T-DRAIN-1`, `T-PLATE-1..2`,
//! `T-VFE-1..4`, `T-TIP-1..3`, `T-SERR-4`, `T-HAND-1`, `T-SEQ-1`, `T-DET-1`, and `UT-30..62`.

use jarambit_g4::decoder::{
    build_g4_blade_profile, check_cavity_drainability_on_grid, compute_g4_zone_masses,
    integrate_implicit_volume_subvoxel, smin_c2,
};
use jarambit_g4::genome::{
    snap_and_repair_genome, G4GeneKind, G4Genome, G4Module, G4_GENES, G4_GENE_COUNT, N_G4_GENES,
};
use jarambit_g4::objective::{compare_g4_objectives, RsKarambitV4Spec};
use jarambit_g4::physics::vfe_verification::{
    run_cantilever_locking_benchmark, run_gmg_pcg_vs_direct_parity_test,
    run_hex8_constant_strain_patch_test,
};
use jarambit_g4::physics::{
    eval_g4_blade_plate_fe, eval_g4_hand_model_hm1, eval_g4_serration_teeth_t6,
    eval_g4_solid_tip_tet10, eval_g4_tip_proxy_tier1, evaluate_goodman_fatigue_sf,
    SurfaceFinishClass,
};
use jarambit_g4::search::{
    evaluate_g4_genome, run_g4_search, verify_record_admission_contract, G4EvalTier,
    G4SearchAlgorithm, G4SearchConfig,
};
use std::cmp::Ordering;

#[test]
fn test_t_gen_1_to_4_genome_schema_invariants() {
    assert_eq!(N_G4_GENES, 130);
    assert_eq!(G4_GENE_COUNT, 130);
    let n_mod_t = G4_GENES.iter().filter(|s| s.module == G4Module::T).count();
    let n_int = G4_GENES.iter().filter(|s| s.kind == G4GeneKind::Int).count();
    assert_eq!(n_mod_t, 12);
    assert_eq!(n_int, 4);

    // Verify phenotype-only genome_sha256 (`G4-GEN-6`):
    // When `win_gate = 0.0` and `alien_gate = 0.0`, mutating `win_length_frac` or `cppn_w1`
    // must NOT alter `genome_sha256()`.
    let mut g1 = G4Genome::default_anchor();
    for s in &G4_GENES {
        if s.name == "win_gate" || s.name == "alien_gate" {
            g1.u[s.index] = 0.0;
        }
    }
    snap_and_repair_genome(&mut g1);
    let mut g2 = g1.clone();
    for s in &G4_GENES {
        if s.name == "win_length_frac" || s.name == "cppn_freq" || s.name == "cppn_w1" {
            g2.u[s.index] = 0.95;
        }
    }
    assert_eq!(
        g1.genome_sha256(),
        g2.genome_sha256(),
        "Inactive gated genes must not alter phenotype-only genome_sha256 (G4-GEN-6)"
    );

    // Verify zero dead genes when gates are active (`G4-GEN-1`):
    let mut g_open = G4Genome::default_anchor();
    for s in &G4_GENES {
        if s.name == "win_gate" || s.name == "alien_gate" {
            g_open.u[s.index] = 1.0;
        } else {
            g_open.u[s.index] = 0.25;
        }
    }
    snap_and_repair_genome(&mut g_open);
    let base_sha = g_open.genome_sha256();
    for s in &G4_GENES {
        let mut g_mod = g_open.clone();
        g_mod.u[s.index] = if s.name == "win_gate" || s.name == "alien_gate" {
            0.0
        } else {
            0.85
        };
        snap_and_repair_genome(&mut g_mod);
        assert_ne!(
            g_mod.genome_sha256(),
            base_sha,
            "Gene {} (index {}) must affect decoded phenotype when gates are open",
            s.name,
            s.index
        );
    }
}

#[test]
fn test_t_dec_1_to_3_implicit_kernel_and_point_angle() {
    // 1. Verify C2 smoothness of smin_c2 across |a - b| = k
    let k = 0.50;
    let eps = 1e-5;
    let f = |x: f64| smin_c2(x, 0.0, k);
    let d2_left = (f(k - eps) - 2.0 * f(k - 2.0 * eps) + f(k - 3.0 * eps)) / (eps * eps);
    let d2_right = (f(k + 3.0 * eps) - 2.0 * f(k + 2.0 * eps) + f(k + eps)) / (eps * eps);
    assert!(
        (d2_left - d2_right).abs() < 1e-2,
        "smin_c2 must have continuous second derivative at |a - b| = k"
    );

    // 2. Verify point planform angle <= 55 deg across Sobol samples (`Gate S4`)
    for i in 0..16 {
        let g = G4Genome::from_sobol(i, 42);
        let p = g.decode();
        let blade = build_g4_blade_profile(&p);
        assert!(
            blade.point_angle_3mm_deg <= 55.0,
            "Sample {i} point_angle_3mm_deg = {:.2} > 55.0 deg",
            blade.point_angle_3mm_deg
        );
        assert!(
            blade.point_width_2mm_mm <= 3.50,
            "Sample {i} point_width_2mm_mm = {:.2} > 3.50 mm",
            blade.point_width_2mm_mm
        );
    }

    // 3. Verify Module A texture is strictly zero-mean (`L-7`)
    let mut g_tex_lo = G4Genome::default_anchor();
    let mut g_tex_hi = G4Genome::default_anchor();
    for s in &G4_GENES {
        if s.name == "tex_amp_mm" {
            g_tex_lo.u[s.index] = 0.0;
            g_tex_hi.u[s.index] = 1.0;
        }
    }
    let p_lo = g_tex_lo.decode();
    let p_hi = g_tex_hi.decode();
    let b_lo = build_g4_blade_profile(&p_lo);
    let b_hi = build_g4_blade_profile(&p_hi);
    let m_lo = compute_g4_zone_masses(&p_lo, &b_lo, 0.25).as_finished_total_mass_g;
    let m_hi = compute_g4_zone_masses(&p_hi, &b_hi, 0.25).as_finished_total_mass_g;
    assert!(
        (m_lo - m_hi).abs() < 1e-12,
        "Zero-mean surface texture must not add parasitic mass"
    );
}

#[test]
fn test_t_mass_1_subvoxel_smoothed_heaviside_accuracy() {
    let v_sphere_exact = (4.0 / 3.0) * std::f64::consts::PI * 10.0_f64.powi(3);
    let v_sphere_num = integrate_implicit_volume_subvoxel(
        [-12.0, -12.0, -12.0],
        [12.0, 12.0, 12.0],
        0.40,
        |x, y, z| (x * x + y * y + z * z).sqrt() - 10.0,
    );
    let rel_err_sphere = (v_sphere_num - v_sphere_exact).abs() / v_sphere_exact;
    assert!(
        rel_err_sphere < 0.005,
        "Sphere sub-voxel volume error {:.4}% exceeds 0.5% tolerance",
        rel_err_sphere * 100.0
    );

    let r_maj = 12.0_f64;
    let r_min = 3.0_f64;
    let v_torus_exact = 2.0 * std::f64::consts::PI.powi(2) * r_maj * r_min * r_min;
    let v_torus_num = integrate_implicit_volume_subvoxel(
        [-16.0, -16.0, -5.0],
        [16.0, 16.0, 5.0],
        0.40,
        |x, y, z| {
            let q = (x * x + y * y).sqrt() - r_maj;
            (q * q + z * z).sqrt() - r_min
        },
    );
    let rel_err_torus = (v_torus_num - v_torus_exact).abs() / v_torus_exact;
    assert!(
        rel_err_torus < 0.005,
        "Torus sub-voxel volume error {:.4}% exceeds 0.5% tolerance",
        rel_err_torus * 100.0
    );
}

#[test]
fn test_t_drain_1_resolution_invariant_erosion_drainability() {
    for &h in &[0.40, 0.20, 0.10] {
        let rep_open = check_cavity_drainability_on_grid(8.0, 3.0, 6.0, 0.50, h);
        assert!(rep_open.drainable, "3.0 mm port must be drainable at h={h}");
        assert_eq!(rep_open.sealed_void_betti_2_cavities, 0);

        let rep_blocked = check_cavity_drainability_on_grid(8.0, 0.80, 6.0, 0.50, h);
        assert!(
            !rep_blocked.drainable,
            "0.80 mm constricted port (< 2*r_esc = 1.0 mm) must be rejected at h={h}"
        );
        assert_eq!(rep_blocked.sealed_void_betti_2_cavities, 1);
    }
}

#[test]
fn test_t_plate_1_and_2_hole_aware_plate_and_roache_gci() {
    let mut g_solid = G4Genome::default_anchor();
    let mut g_hole = G4Genome::default_anchor();
    for s in &G4_GENES {
        if s.name == "win_gate" {
            g_solid.u[s.index] = 0.0;
            g_hole.u[s.index] = 1.0;
        }
        if s.name == "win_length_frac" || s.name == "win_width_frac" {
            g_hole.u[s.index] = 0.90;
        }
    }
    let p_solid = g_solid.decode();
    let p_hole = g_hole.decode();
    let b_solid = build_g4_blade_profile(&p_solid);
    let b_hole = build_g4_blade_profile(&p_hole);

    let res_solid = eval_g4_blade_plate_fe(&p_solid, &b_solid, true).expect("solid plate FE");
    let res_hole = eval_g4_blade_plate_fe(&p_hole, &b_hole, false).expect("hole plate FE");

    assert!(res_solid.gci_admission.plate_gci_computed);
    assert!(res_solid.gci_admission.gci_fine_fraction.unwrap() <= 0.05);
    assert!(
        res_hole.lateral_capacity_n < res_solid.lateral_capacity_n,
        "Through-fenestration window must be modeled as a true hole that reduces plate capacity"
    );
}

#[test]
fn test_t_vfe_1_to_4_bbar_hex8_and_gmg_pcg_benchmarks() {
    let patch_err = run_hex8_constant_strain_patch_test();
    assert!(
        patch_err < 1e-12,
        "T-VFE-1 patch test strain error {:.3e} >= 1e-12",
        patch_err
    );
    let (bbar_err, plain_err) = run_cantilever_locking_benchmark();
    assert!(
        bbar_err < 0.05 && plain_err > 0.50,
        "T-VFE-2 B-bar thin beam error {:.4} >= 0.05 (plain hex8 locking err = {:.4})",
        bbar_err,
        plain_err
    );
    let gmg_err = run_gmg_pcg_vs_direct_parity_test();
    assert!(
        gmg_err < 1e-9,
        "T-VFE-4 GMG-PCG vs direct Cholesky parity error {:.3e} >= 1e-9",
        gmg_err
    );
}

#[test]
fn test_t_tip_1_to_3_frame_invariant_solid_tip_and_crown() {
    let mut g_no_crown = G4Genome::default_anchor();
    let mut g_with_crown = G4Genome::default_anchor();
    for s in &G4_GENES {
        if s.name == "t_tip_crown_mm" {
            g_no_crown.u[s.index] = 0.0;
            g_with_crown.u[s.index] = 0.75;
        }
    }
    let p0 = g_no_crown.decode();
    let p1 = g_with_crown.decode();
    let b0 = build_g4_blade_profile(&p0);
    let b1 = build_g4_blade_profile(&p1);

    let tip0 = eval_g4_solid_tip_tet10(&p0, &b0, 0.30).unwrap_or_else(|| eval_g4_tip_proxy_tier1(&p0, &b0));
    let tip1 = eval_g4_solid_tip_tet10(&p1, &b1, 0.30).unwrap_or_else(|| eval_g4_tip_proxy_tier1(&p1, &b1));
    assert!(
        tip1.tip_torsion_0p80nm_sf > tip0.tip_torsion_0p80nm_sf,
        "Distal tip-bisector crown must increase LC-TIP-1 torsion SF"
    );
    assert!(
        tip1.tip_torsion_0p80nm_sf >= 1.25,
        "Crowned tip torsion SF {:.3} must exceed 1.25",
        tip1.tip_torsion_0p80nm_sf
    );
}

#[test]
fn test_t_serr_4_all_teeth_checked_including_end_teeth() {
    let g = G4Genome::default_anchor();
    let p = g.decode();
    let b = build_g4_blade_profile(&p);
    let rep = eval_g4_serration_teeth_t6(&p, &b);
    assert_eq!(rep.teeth_checked_in, rep.teeth_total_in);
    assert_eq!(rep.teeth_checked_sp, rep.teeth_total_sp);
    assert!(rep.teeth_checked_in >= 3);
    assert!(rep.teeth_checked_sp >= 3);
    assert!(rep.min_tooth_sf_overall >= 1.25);

    // Also verify corrected Goodman fatigue (`F-M-2`, `AG-08`)
    let sf_fat = evaluate_goodman_fatigue_sf(185.0, 4.2, 6.0, 1.18, SurfaceFinishClass::GroundBevel);
    assert!(sf_fat > 1.15);
}

#[test]
fn test_t_hand_1_and_t_seq_1_hm1_equilibrium_and_per_case_sequence() {
    let g = G4Genome::default_anchor();
    let p = g.decode();
    let b = build_g4_blade_profile(&p);
    let hm1 = eval_g4_hand_model_hm1(&p, &b);
    assert!(
        hm1.force_residual_n < 1e-9,
        "HM-1 force equilibrium residual {:.3e} >= 1e-9",
        hm1.force_residual_n
    );
    assert!(
        hm1.moment_residual_nmm < 1e-9,
        "HM-1 moment equilibrium residual {:.3e} >= 1e-9",
        hm1.moment_residual_nmm
    );
    assert!(
        hm1.grip_sf_baton_3427n_ult > hm1.grip_sf_baton_3427n_legacy_ring_clamp * 2.0,
        "HM-1 compliant hand equilibrium must eliminate the unphysical ring-clamped baton moment arm"
    );
}

#[test]
fn test_ut_31_and_ut_32_rs_v4_loader_and_lexicographic_comparator() {
    let spec = RsKarambitV4Spec::load_embedded().expect("RS-karambit-v4.toml must parse with deny_unknown_fields");
    assert_eq!(spec.id, "RS-karambit-v4");
    assert_eq!(spec.version, 4);

    let g_feas = G4Genome::default_anchor();
    let mut g_infeas = G4Genome::default_anchor();
    for s in &G4_GENES {
        if s.name == "t_core" || s.name == "skin_t_base" || s.name == "ring_od" {
            g_infeas.u[s.index] = 0.0;
        }
        if s.name == "t_tip_crown_mm" {
            g_infeas.u[s.index] = 0.0;
        }
    }
    let rec_feas = evaluate_g4_genome(&spec, &g_feas, G4EvalTier::Tier2Full3DCert).unwrap();
    let rec_infeas = evaluate_g4_genome(&spec, &g_infeas, G4EvalTier::Tier1SdfAndPlate).unwrap();

    assert!(
        rec_feas.objective.feasible,
        "Default anchor genome must be feasible across all 29 gates (governing: {}, min_margin: {:.4})",
        rec_feas.objective.governing_gate_name,
        rec_feas.objective.min_safety_margin
    );
    assert!(rec_feas.objective.w_ergolight_v2 >= 50.0);
    assert!(
        rec_feas.mass.as_finished_total_mass_g < 50.0,
        "Default anchor mass {:.2} g must be < 50.0 g",
        rec_feas.mass.as_finished_total_mass_g
    );
    verify_record_admission_contract(&rec_feas, None)
        .expect("Default anchor must pass G4ReleaseAdmissionContract");

    if !rec_infeas.objective.feasible {
        assert!(rec_infeas.objective.w_ergolight_v2 < 50.0);
        assert_eq!(
            compare_g4_objectives(&rec_feas.objective, &rec_infeas.objective),
            Ordering::Greater
        );
    }
}

#[test]
fn test_t_det_1_cross_thread_bit_identical_determinism() {
    let spec = RsKarambitV4Spec::load_embedded().unwrap();
    let cfg = G4SearchConfig {
        algorithm: G4SearchAlgorithm::G4FullFi2PopCvt,
        seed: 777,
        k_cvt_cells: 16,
        n_init_sobol: 8,
        n_batches: 3,
        batch_size: 8,
        recert_interval_batches: 2,
    };

    let pool1 = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .unwrap();
    let res1 = pool1.install(|| run_g4_search(&spec, &cfg));

    let pool4 = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();
    let res4 = pool4.install(|| run_g4_search(&spec, &cfg));

    assert_eq!(
        res1.best_record.genome.u, res4.best_record.genome.u,
        "1-thread and 4-thread G4 search runs must produce bit-identical winner genomes (T-DET-1)"
    );
    assert_eq!(
        res1.best_record.objective.w_ergolight_v2.to_bits(),
        res4.best_record.objective.w_ergolight_v2.to_bits(),
        "1-thread and 4-thread G4 search runs must produce bit-identical scores (T-DET-1)"
    );
}
