//! Standalone Jarambit G5 crate (`jarambit-g5`):
//! - `ergolight`: `W_ERGOLIGHT_v1` (draft-d) continuous multi-objective scorer (`T5`).
//! - `g5_sandwich`: `SandwichSection`, `SandwichPlate`, `solve_ring_band_t6` (1.5 mm band load),
//!   and fail-closed `solve_g5_solid_tet10_strict` (`T4.1`).
//! - `g5_genome`: 179-gene `G5.0` unit-cube genome, `from_hzm93`, strict CSV loader, Sobol generator,
//!   and reflective `Iso+Line-DD` + module crossover + self-adaptive `sigma` (`T4.2`).
//! - `g5_sdf`: 3D exact separable EDT, half-voxel morphology, TPMS lattice, 3D 6-connectivity filter,
//!   14 mm grip-to-guard taper, 3D corrugated/fenestrated/serrated blade, and Kuhn Marching-Tetrahedra (`T4.3`).
//! - `g5_qd`: 8D behavior descriptors, 3-tier evaluation cascade, and 1024-niche `CvtMapElitesArchive` (`T4.3`).
//!
//! - `requirements`: strict typed loader of `requirements/RS-karambit-v5.toml`.
//!
//! G5 starts as a copy of the validated G3 crate (`crates/g3`, v3.0.0) and is changed to RS-karambit-v5 in
//! reviewed steps; `crates/g4` is the withdrawn v4.0.0 prototype and is not used. Kept in its own crate with its own
//! lock file so `crates/crescent`, `crates/physics` and `crates/g3` stay byte-for-byte unchanged (v1
//! `certification::identity` and the G3 decoder of record; see `record_guards`).

pub mod ergolight;
pub mod g5_genome;
pub mod g5_namespace;
pub mod g5_qd;
pub mod g5_sandwich;
pub mod g5_sdf;
pub mod loads;
pub mod material;
pub mod objective;
pub mod requirements;
pub mod sobol_joe_kuo;
pub mod tip_tet10;

/// Pure-Rust FIPS 180-4 SHA-256 hex digest helper (matches `battery::sha256_bytes` bit-for-bit).
pub fn sha256_bytes(data: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let bit_len = (data.len() as u64) * 8;
    let mut msg = data.to_vec();
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0x00);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());
    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[4 * i],
                chunk[4 * i + 1],
                chunk[4 * i + 2],
                chunk[4 * i + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        for ( dst, src) in h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
            *dst = dst.wrapping_add(src);
        }
    }
    format!(
        "{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}",
        h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]
    )
}

/// Guards for the records this crate must never change: the G3 decoder of the v3.0.0 cohort of record, and the
/// cached G5 decoder hash.
#[cfg(test)]
mod record_guards {
    /// The G3 decoder hash, computed exactly as `crates/g3/src/g3_qd.rs::g3_decoder_sha256`, must still equal the
    /// hash recorded in the v3.0.0 receipt. It fails if G5 work edits any G3 source, the shared physics or crescent
    /// files G3 hashes, or either lock file.
    #[test]
    fn g3_decoder_of_record_is_unchanged() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"G3.0\0");
        for src in [
            include_str!("../../g3/src/g3_genome.rs"),
            include_str!("../../g3/src/sobol_joe_kuo.rs"),
            include_str!("../../g3/src/g3_sdf.rs"),
            include_str!("../../g3/src/g3_sandwich.rs"),
            include_str!("../../g3/src/ergolight.rs"),
            include_str!("../../g3/src/g3_qd.rs"),
            include_str!("../../physics/src/fine.rs"),
            include_str!("../../physics/src/solid_tet10.rs"),
            include_str!("../../crescent/src/lib.rs"),
            include_str!("../../crescent/src/fem.rs"),
            include_str!("../../crescent/src/certification.rs"),
            include_str!("../../crescent/src/export.rs"),
            include_str!("../../g3/Cargo.lock"),
            include_str!("../../../Cargo.lock"),
        ] {
            bytes.extend_from_slice(src.as_bytes());
            bytes.push(0);
        }
        let receipt: serde_json::Value =
            serde_json::from_str(include_str!("../../../evidence/g3/v3.0.0/release.json")).expect("G3 receipt");
        let recorded = receipt["provenance"]["decoder_sha256"].as_str().expect("provenance.decoder_sha256");
        assert_eq!(crate::sha256_bytes(&bytes), recorded, "the G3 v3.0.0 decoder of record changed");
    }

    /// RS-karambit-v5 `[mass] grid_agreement_max_frac`: the partial-volume mass of a Sobol design agrees between
    /// h 0.4 and h 0.2 (whole-voxel counting and one-voxel drainage gaps made G3 masses move by up to 47 %).
    #[test]
    fn partial_volume_mass_agrees_between_h_and_half_h() {
        let g = crate::g5_genome::sobol_unit_genome(5, 7, "gridcheck");
        let a = crate::g5_qd::mass_grid_agreement(&g, 0.4);
        println!("{a:?}");
        assert!(a.within_requirement(), "{a:?}");
    }

    /// G5.0 construction targets on Sobol designs: the grip swell section measured on the voxel SDF has the
    /// perimeter and H/T of its genes, the point is no blunter than its point-angle gene, every serration gullet has
    /// at least the required radius, and most designs drain their powder.
    #[test]
    fn g5_decoder_meets_its_construction_targets() {
        use crate::g5_qd::measure_point_planform;
        use crate::g5_sdf::{build_blade_profile_2d, build_g5_sdf, serration_duty_for_gullet, serration_gullet_radius};
        let rs = crate::requirements::rs_v5();
        let mut drained = 0usize;
        let n = 6usize;
        for i in 0..n {
            let g = crate::g5_genome::sobol_unit_genome(i + 1, 3, "decoder-targets");
            let p = g.decode();
            let grid = build_g5_sdf(&g, 0.4, true);
            let hz = grid.measure_handzone_masses_and_swell(&p);
            assert!((hz.swell_perimeter_mm / p["grip_perim"] - 1.0).abs() < 0.02, "{i}: perimeter {hz:?}");
            assert!((hz.swell_h_over_t / p["grip_hot"] - 1.0).abs() < 0.03, "{i}: H/T {hz:?}");
            assert!(hz.l_g_mm >= rs.ergonomics.grip.length_min_mm - 1.0, "{i}: grip length {}", hz.l_g_mm);
            let pt = measure_point_planform(&build_blade_profile_2d(&p));
            assert!(pt.angle_1mm_deg <= p["point_angle"] + 5.0, "{i}: point {pt:?} vs {}", p["point_angle"]);
            if grid.am_connectivity.sealed_void_mm3 < 1500.0 {
                drained += 1;
            }
        }
        assert!(drained * 3 >= n * 2, "only {drained} of {n} Sobol designs drain (sealed < 1500 mm3)");
        // Gullet radius by construction over the whole gene box (pitch 4-6 mm, depth 0.8-1.5 mm, depth <= pitch/4)
        for pitch in [4.0, 5.0, 6.0] {
            for depth in [0.8, 1.0, 1.5_f64] {
                let d = depth.min(rs.serration.depth_max_fraction_of_pitch * pitch);
                for duty in [0.45, 0.65, 0.85] {
                    let w = serration_duty_for_gullet(duty, pitch, d) * pitch;
                    assert!(serration_gullet_radius(w, d) >= rs.serration.gullet_radius_min_mm - 1e-9);
                }
            }
        }
        assert_eq!(crate::g5_sdf::SERR_DEPTH_PER_PITCH, rs.serration.depth_max_fraction_of_pitch);
    }

    /// Tier 1 records the RS-karambit-v5 load cases with the plate blade model and the ring sweep, and the sweep is
    /// never weaker than the owner's bearing direction it includes.
    #[test]
    fn tier1_records_rs_v5_loads_with_plate_and_ring_sweep() {
        let g = crate::g5_genome::sobol_unit_genome(2, 3, "tier1-loads");
        let rec = crate::g5_qd::evaluate_tier1_sdf_and_plate(&g, 0.5);
        let l = rec.loads;
        assert_eq!(l.blade_model, crate::loads::BladeModel::Plate, "{l:?}");
        assert!(l.ring_any_direction_sf <= l.ring_bearing_sf * 300.0 / 300.0 + 1e-12, "{l:?}");
        assert!(l.teeth_sf.is_finite() && l.teeth_sf > 0.0, "{l:?}");
        assert!(l.seq_min().is_finite() && l.seq_min() > 0.0, "{l:?}");
    }

    #[test]
    fn cached_g5_decoder_hash_equals_a_fresh_recompute() {
        let cached = crate::g5_qd::g5_decoder_sha256();
        assert_eq!(cached, crate::g5_qd::g5_decoder_sha256_uncached());
        assert_eq!(cached, crate::g5_qd::g5_decoder_sha256());
        assert_eq!(cached.len(), 64);
    }
}

#[cfg(test)]
mod v1_identity_tests {

    #[test]
    fn v1_certification_identity_is_unchanged_from_origin_main() {
        let policy_doc: serde_json::Value = serde_json::from_str(include_str!(
            "../../../assurance/testing/policy.json"
        ))
        .expect("parse assurance/testing/policy.json");
        let expected_lock_sha = policy_doc["source_bundle"]["Cargo.lock"]
            .as_str()
            .expect("source_bundle.Cargo.lock in policy.json");
        let lock_sha = crate::sha256_bytes(include_bytes!("../../../Cargo.lock"));
        assert_eq!(
            lock_sha, expected_lock_sha,
            "root Cargo.lock must match assurance/testing/policy.json"
        );
        let manifest_doc: serde_json::Value = serde_json::from_str(include_str!(
            "../../../evidence/evolve_certifiable/manifest.json"
        ))
        .expect("parse evidence/evolve_certifiable/manifest.json");
        let exports = manifest_doc["regeneration"]["exports"]
            .as_array()
            .expect("regeneration.exports array");
        let expected_ctrl_id = exports
            .iter()
            .find(|e| e["id"].as_str() == Some("legal_genome"))
            .and_then(|e| e["certificate_identity"].as_str())
            .expect("legal_genome certificate_identity");
        let expected_auto_id = exports
            .iter()
            .find(|e| e["id"].as_str() == Some("automatic"))
            .and_then(|e| e["certificate_identity"].as_str())
            .expect("automatic certificate_identity");

        let g = jarambit_crescent::genome::Genome::c4_seed(5.7);
        let id = jarambit_crescent::certification::identity(&g);
        println!("v1 certification::identity(c4_seed(5.7)) = {id}");
        assert_eq!(
            id,
            "4b72bbbb667af755b36a181449b5e80be044802b14aac9531db98385742919a4"
        );
        let ctrl_csv = include_str!(
            "../../../evidence/evolve_certifiable/controls/legal_genome/control_genome.csv"
        );
        let ctrl_row = ctrl_csv.lines().nth(1).expect("control_genome row");
        let ctrl_fields: Vec<&str> = ctrl_row.split(',').collect();
        let g_ctrl =
            jarambit_crescent::genome::Genome::from_csv(&ctrl_fields).expect("parse control_genome");
        let ctrl_id = jarambit_crescent::certification::identity(&g_ctrl);
        assert_eq!(
            ctrl_id, expected_ctrl_id,
            "control_genome certificate_identity must match pinned evidence/evolve_certifiable/manifest.json"
        );
        let auto_csv = include_str!(
            "../../../evidence/evolve_certifiable/automatic/by-content/db5d8e32e99d0b2d8ef6d7c7f9622a0601487c25b80879bbc8be1a502693c55e/attempt-1792206525341070917/cert_pipeline_r01_p14_c02_genome.csv"
        );
        let auto_row = auto_csv.lines().nth(1).expect("cert_pipeline_r01_p14_c02 row");
        let auto_fields: Vec<&str> = auto_row.split(',').collect();
        let g_auto =
            jarambit_crescent::genome::Genome::from_csv(&auto_fields).expect("parse cert_pipeline_r01_p14_c02");
        let auto_id = jarambit_crescent::certification::identity(&g_auto);
        assert_eq!(
            auto_id, expected_auto_id,
            "cert_pipeline_r01_p14_c02 certificate_identity must match pinned evidence/evolve_certifiable/manifest.json"
        );
    }

    #[test]
    fn s5_fixes_blade_porosity_no_spine_fin_and_sharpened_cap_verified() {
        use crate::g5_genome::{G5Genome, G5_GENES};
        use crate::g5_qd::evaluate_tier1_sdf_and_plate;
        use crate::g5_sdf::{
            build_blade_profile_2d, build_g5_sdf, build_g5_sdf_from_params, GUARD_X_MM,
        };

        let idx_of = |name: &str| -> usize {
            G5_GENES
                .iter()
                .position(|g| g.name == name)
                .unwrap_or_else(|| panic!("missing gene {name}"))
        };

        // 1. N1 + P1: Unvented blade (win_amp = 0.20 <= 0.35) gets zero mass credit for rho_blade = 0.06;
        //    vented blade (win_amp = 0.50 > 0.35) drops mass_g AND moves plate strength terms + am_build.
        let set_phys = |g: &mut G5Genome, name: &str, phys: f64| {
            let i = idx_of(name);
            g.u[i] = G5_GENES[i].unit_from_phys(phys).clamp(0.0, 1.0);
        };
        let mut g_solid = G5Genome::benchmark_reference("22bb8696");
        set_phys(&mut g_solid, "rho_blade", 1.0);
        set_phys(&mut g_solid, "win_amp", 0.20);
        set_phys(&mut g_solid, "lat_cell", 4.0);

        let mut g_unvented = g_solid.clone();
        set_phys(&mut g_unvented, "rho_blade", 0.06);

        let mut g_vented = g_unvented.clone();
        set_phys(&mut g_vented, "win_amp", 0.50);

        let r_solid = evaluate_tier1_sdf_and_plate(&g_solid, 0.8);
        let r_unvented = evaluate_tier1_sdf_and_plate(&g_unvented, 0.8);
        let r_vented = evaluate_tier1_sdf_and_plate(&g_vented, 0.8);

        assert!(
            (r_solid.inputs.mass_g - r_unvented.inputs.mass_g).abs() < 1e-6,
            "P1/N1: unvented blade (win_amp=0.20 <= 0.35) must not get mass credit for rho_blade=0.06 (solid={} vs unvented={})",
            r_solid.inputs.mass_g,
            r_unvented.inputs.mass_g
        );
        assert!(
            r_vented.inputs.mass_g < r_solid.inputs.mass_g - 1.0,
            "P1/N1: vented hollow blade (win_amp=0.50 > 0.35, rho_blade=0.06) must reduce mass_g (vented={} vs solid={})",
            r_vented.inputs.mass_g,
            r_solid.inputs.mass_g
        );
        assert!(
            r_vented.inputs.twist_body_sf < r_solid.inputs.twist_body_sf
                && r_vented.inputs.hm_spine_sf < r_solid.inputs.hm_spine_sf
                && r_vented.inputs.am_build < r_solid.inputs.am_build,
            "P1/N1: vented hollow blade must reduce plate capacities and am_build (vented twist_body={} vs solid={})",
            r_vented.inputs.twist_body_sf,
            r_solid.inputs.twist_body_sf
        );

        // 2. N2: Full G5 SDF (`include_blade == true`) has zero phantom f075 spine fin outside evolved blade for x > 94.0
        let base = G5Genome::benchmark_reference("22bb8696");
        let p_base = base.decode();
        let blade = build_blade_profile_2d(&p_base);
        let grid = build_g5_sdf(&base, 0.8, false);
        let mut fin_voxels = 0usize;
        for ix in 0..grid.nx {
            let x = grid.x[ix];
            if x <= 94.0 {
                continue;
            }
            if x > blade.x_tip + 1.5 {
                for iy in 0..grid.ny {
                    for iz in 0..grid.nz {
                        if grid.d[grid.idx(ix, iy, iz)] < 0.0 {
                            fin_voxels += 1;
                        }
                    }
                }
                continue;
            }
            let u = ((x - blade.x_heel) / (blade.x_tip - blade.x_heel).max(1e-9)).clamp(0.0, 1.0);
            let f_idx = u * ((blade.xs.len() - 1) as f64);
            let i0 = (f_idx.floor() as usize).min(blade.xs.len() - 2);
            let frac = f_idx - (i0 as f64);
            let y_in = blade.y_in[i0] * (1.0 - frac) + blade.y_in[i0 + 1] * frac;
            let y_sp = blade.y_sp[i0] * (1.0 - frac) + blade.y_sp[i0 + 1] * frac;
            for iy in 0..grid.ny {
                let y = grid.y[iy];
                // If outside evolved blade span by > 1.8 mm, no f075 spine fin voxel should be inside
                if y > y_sp + 1.8 || y < y_in - 1.8 {
                    for iz in 0..grid.nz {
                        if grid.d[grid.idx(ix, iy, iz)] < 0.0 {
                            fin_voxels += 1;
                        }
                    }
                }
            }
        }
        assert_eq!(
            fin_voxels, 0,
            "N2: full G5 SDF must have 0 phantom f075 spine fin voxels outside evolved blade for x > 94 mm"
        );

        // 3. N4: When guard drop is zero (h_g = 0 and yb_g >= yb0), l_g_mm must fail closed to +inf
        let mut p_no_guard = p_base.clone();
        p_no_guard.insert("h_g", 0.0);
        p_no_guard.insert("yb_g", p_base["yb0"] + 2.0);
        let grid_ng = build_g5_sdf_from_params(&p_no_guard, 1.0, false, false);
        let meas_ng = grid_ng.measure_handzone_masses_and_swell(&p_no_guard);
        assert!(
            !meas_ng.l_g_mm.is_finite(),
            "N4: missing guard drop must fail closed to l_g_mm = +inf, got {}",
            meas_ng.l_g_mm
        );

        // 4. N6 + P3: All 120 scrambled Sobol genomes + extreme corner genome satisfy reach_chord <= 85.725 mm and sharpened_arc <= 101.6 mm
        for s_idx in 0..120 {
            let gs = crate::g5_genome::sobol_unit_genome(s_idx, 4242, "22bb8696");
            let bp = build_blade_profile_2d(&gs.decode());
            let y_mid = 0.5 * (bp.y_in[0] + bp.y_sp[0]);
            let r_mid = (bp.tip_xy.0 - GUARD_X_MM).hypot(bp.tip_xy.1 - y_mid);
            let r_zero = (bp.tip_xy.0 - GUARD_X_MM).hypot(bp.tip_xy.1);
            assert!(
                r_mid <= 85.725 + 1e-6 && r_zero <= 85.725 + 1e-6,
                "P3/N6: Sobol genome {s_idx} exceeded 85.725 mm reach (r_mid={r_mid}, r_zero={r_zero})"
            );
            assert!(
                bp.sharpened_arc_in_mm <= 101.6 + 1e-6 && bp.sharpened_arc_sp_mm <= 101.6 + 1e-6,
                "N6: Sobol genome {s_idx} exceeded 101.6 mm sharpened edge cap (in={}, sp={})",
                bp.sharpened_arc_in_mm,
                bp.sharpened_arc_sp_mm
            );
        }
        let mut g_ext = G5Genome::benchmark_reference("22bb8696");
        g_ext.u[idx_of("blade_len")] = 1.0;
        g_ext.u[idx_of("hook_drop")] = 1.0;
        g_ext.u[idx_of("sweep_deg")] = 1.0;
        g_ext.u[idx_of("s0_in")] = 0.0;
        g_ext.u[idx_of("s0_sp")] = 0.0;
        let b_ext = build_blade_profile_2d(&g_ext.decode());
        let y_mid_ext = 0.5 * (b_ext.y_in[0] + b_ext.y_sp[0]);
        let r_mid_ext = (b_ext.tip_xy.0 - GUARD_X_MM).hypot(b_ext.tip_xy.1 - y_mid_ext);
        assert!(
            r_mid_ext <= 85.725 + 1e-6 && b_ext.reach_chord_mm <= 85.725 + 1e-6,
            "N6: straight-line reach chord from tip to guard face must be <= 85.725 mm, got {}",
            b_ext.reach_chord_mm
        );
        assert!(
            b_ext.sharpened_arc_in_mm <= 101.6 + 1e-6
                && b_ext.sharpened_arc_sp_mm <= 101.6 + 1e-6,
            "N6: measured sharpened edge arc must be <= 101.6 mm (4.0 in), got in={} mm, sp={} mm",
            b_ext.sharpened_arc_in_mm,
            b_ext.sharpened_arc_sp_mm
        );

        // 5. S5-2 / P4: Verify that on a vented hollow blade (`g_vented`), the z-axis blade drain port
        //    has ZERO voxels inside either 3.5 mm edge bevel band (`d_in < 3.5 || d_sp < 3.5`).
        let grid_v_solid = build_g5_sdf(&g_vented, 0.5, false);
        let grid_v_am = build_g5_sdf(&g_vented, 0.5, true);
        let d_am_v = grid_v_am.d_am.as_ref().expect("d_am present");
        let bp_v = build_blade_profile_2d(&g_vented.decode());
        let mut bevel_port_voxels = 0usize;
        for (ix, &x) in grid_v_am.x.iter().enumerate() {
            if x < bp_v.x_heel || x > bp_v.x_tip {
                continue;
            }
            let u = ((x - bp_v.x_heel) / (bp_v.x_tip - bp_v.x_heel).max(1e-9)).clamp(0.0, 1.0);
            let f_idx = u * ((bp_v.xs.len() - 1) as f64);
            let i0 = (f_idx.floor() as usize).min(bp_v.xs.len() - 2);
            let frac = f_idx - (i0 as f64);
            let yi = bp_v.y_in[i0] * (1.0 - frac) + bp_v.y_in[i0 + 1] * frac;
            let ys = bp_v.y_sp[i0] * (1.0 - frac) + bp_v.y_sp[i0 + 1] * frac;
            for (iy, &y) in grid_v_am.y.iter().enumerate() {
                if y < yi || y > ys {
                    continue;
                }
                let in_bevel_band = (y - yi) < 3.5 || (ys - y) < 3.5;
                if !in_bevel_band {
                    continue;
                }
                for iz in 0..grid_v_am.nz {
                    let idx = grid_v_am.idx(ix, iy, iz);
                    // Surface or near-surface voxel that was solid in `d` (`d < 0`) but opened by a port (`d_am >= 0`)
                    if grid_v_solid.d[idx] < 0.0 && d_am_v[idx] >= 0.0 {
                        bevel_port_voxels += 1;
                    }
                }
            }
        }
        assert_eq!(
            bevel_port_voxels, 0,
            "S5-2 / P4: blade drain port must have 0 voxels inside either 3.5 mm edge bevel band"
        );

        // 6. S5-5 / P5 / S6-7: Verify that across the plunge/seam zone x in [86.0, 94.0] at h = 0.4,
        //    all 7 regression genomes from REREVIEW_S6 (idx = 3, 6, 9, 50, 55, 91, 106 of
        //    `sobol_unit_genome(idx, 5150, "22bb869682e4")`) pass the P5 seam-vs-reference step check.
        let interp_line = |xs: &[f64], ys: &[f64], xq: f64| -> Option<f64> {
            if xs.is_empty() || xq < xs[0] || xq > xs[xs.len() - 1] {
                return None;
            }
            let k = xs.partition_point(|&v| v <= xq).clamp(1, xs.len() - 1);
            let t = (xq - xs[k - 1]) / (xs[k] - xs[k - 1]).max(1e-12);
            Some(ys[k - 1] + t * (ys[k] - ys[k - 1]))
        };
        for &p5_idx in &[3usize, 6, 9, 50, 55, 91, 106] {
            let g_p5 = crate::g5_genome::sobol_unit_genome(p5_idx, 5150, "22bb869682e4");
            let p_p5 = g_p5.decode();
            let bp_p5 = build_blade_profile_2d(&p_p5);
            let grid_p5 = build_g5_sdf(&g_p5, 0.4, false);
            let thick = |i: usize, j: usize| -> f64 {
                let base = (i * grid_p5.ny + j) * grid_p5.nz;
                (0..grid_p5.nz)
                    .filter(|&k| grid_p5.d[base + k] < 0.0)
                    .count() as f64
                    * grid_p5.h
            };
            let mut ms = 0.0_f64;
            let mut mr = 0.0_f64;
            for i in 0..grid_p5.nx.saturating_sub(1) {
                let (x0, x1) = (grid_p5.x[i], grid_p5.x[i + 1]);
                let in_seam = x0 >= 86.0 && x1 <= 94.0;
                let in_ref = x0 >= 94.0 && x1 <= 140.0 && x1 < bp_p5.x_tip - 4.0;
                if !in_seam && !in_ref {
                    continue;
                }
                for (j, &y) in grid_p5.y.iter().enumerate() {
                    let ok = |xq: f64| match (
                        interp_line(&bp_p5.xs, &bp_p5.y_in, xq),
                        interp_line(&bp_p5.xs, &bp_p5.y_sp, xq),
                    ) {
                        (Some(a), Some(b)) => y > a.min(b) + 3.5 && y < a.max(b) - 3.5,
                        _ => false,
                    };
                    if !(ok(x0) && ok(x1)) {
                        continue;
                    }
                    let (t0, t1) = (thick(i, j), thick(i + 1, j));
                    if in_seam {
                        assert!(
                            t0 > 0.0 && t1 > 0.0,
                            "S7-5: medial seam band (86..94 mm) must have no zero-thickness holes on sobol({p5_idx}, 5150) at x=({x0:.1},{x1:.1}), y={y:.1}"
                        );
                    } else if t0 <= 0.0 || t1 <= 0.0 {
                        continue;
                    }
                    let d_step = (t1 - t0).abs();
                    if in_seam {
                        ms = ms.max(d_step);
                    } else {
                        mr = mr.max(d_step);
                    }
                }
            }
            assert!(
                ms <= (mr + 0.8).max(2.0),
                "S5-5 / P5: sobol({p5_idx}, 5150) seam step {ms:.2} mm exceeded max(ref {mr:.2} + 0.8, 2.0)"
            );
        }

        // 7. S5-7 & S5-8 & B2, r2 S2/S3: buckling_lambda is measured (lateral-torsional, analytic on the exact blade
        //    law), hm_twist_sf is the plate's blade-body SF scaled linearly from 1.82 to 16.2 N*m (no inflation), and
        //    production grip_sf_min depends on blade length
        assert!(r_solid.inputs.buckling_lambda > 0.0 && r_solid.inputs.buckling_lambda.is_finite());
        let hm_from_body = r_solid.inputs.twist_body_sf * (1820.0 / 16_200.0);
        if r_solid.inputs.twist_body_sf < 4.0 {
            assert!((r_solid.inputs.hm_twist_sf - hm_from_body).abs() < 1e-9);
        } else {
            assert!(r_solid.inputs.hm_twist_sf >= hm_from_body - 1e-9 && r_solid.inputs.hm_twist_sf <= 3.0);
        }
        let mut g_short = g_solid.clone();
        set_phys(&mut g_short, "blade_len", 60.0);
        set_phys(&mut g_short, "hook_drop", 25.0);
        let mut g_long = g_solid.clone();
        set_phys(&mut g_long, "blade_len", 85.0);
        set_phys(&mut g_long, "hook_drop", 52.0);
        let r_short = evaluate_tier1_sdf_and_plate(&g_short, 0.8);
        let r_long = evaluate_tier1_sdf_and_plate(&g_long, 0.8);
        assert!(
            (r_short.inputs.grip_sf_min - r_long.inputs.grip_sf_min).abs() > 1e-3,
            "S5-7: production grip_sf_min must depend on evolved blade tip_xy and centroid_x (short={} vs long={})",
            r_short.inputs.grip_sf_min,
            r_long.inputs.grip_sf_min
        );

        // 8. I5, I7, S8-2, S8-3: Verify strict CSV decoder_commit check, archive.jsonl re-scoring + "inf" sentinel,
        //    and CvtMapElitesArchive self-check (positive and negative cases).
        let csv_row = g_solid.to_csv_row();
        assert!(G5Genome::from_csv_row_checked(&csv_row, "22bb8696").is_ok());
        assert!(G5Genome::from_csv_row_checked(&csv_row, "deadbeef").is_err());

        // S8-2: Serializing ErgolightInputs emits scorer keys (`L_g_mm`, `spin_I_g_mm2`, `spin_I_ref_g_mm2`)
        // and `"inf"` for +infinity sentinels, and round-trips through `score_ergolight_map`.
        let mut inp_inf = r_solid.inputs.clone();
        inp_inf.max_apex_mm = f64::INFINITY;
        inp_inf.max_half_angle_deg = f64::INFINITY;
        let inp_val = serde_json::to_value(&inp_inf).expect("serialize ErgolightInputs");
        assert_eq!(inp_val["max_apex_mm"].as_str(), Some("inf"));
        assert_eq!(inp_val["max_half_angle_deg"].as_str(), Some("inf"));
        assert!(inp_val.get("L_g_mm").is_some());
        assert!(inp_val.get("spin_I_g_mm2").is_some());
        assert!(inp_val.get("spin_I_ref_g_mm2").is_some());
        // G5 scores with W_ERGOLIGHT_v2 (`objective`); a record is re-scored from its own measurements bit-for-bit
        let bp = crate::g5_sdf::build_blade_profile_2d(&g_solid.decode());
        let pt = crate::g5_qd::measure_point_planform(&bp);
        let rescored = crate::objective::score_v2(&crate::objective::ObjectiveInputs {
            inputs: &r_solid.inputs,
            loads: &r_solid.loads,
            point: &pt,
            am: &r_solid.am_connectivity,
            blade: &bp,
        });
        assert_eq!(rescored.metric, "W_ERGOLIGHT_v2");
        assert!((rescored.score_100 - r_solid.score.score_100).abs() < 1e-12);
        assert!(r_solid.tier.input_provenance()["measured"].is_array());

        let mut arch_chk = crate::g5_qd::CvtMapElitesArchive::new(1024);
        arch_chk.insert(g_solid.clone(), r_solid.clone());
        assert!(arch_chk.verify_integrity().is_ok());
        // S8-3 negative tests: tampered genome, tampered descriptor, or tampered decoder_sha256 must fail closed
        let nid = *arch_chk.elites.keys().next().unwrap();
        let mut arch_bad_g = arch_chk.clone();
        arch_bad_g.elites.get_mut(&nid).unwrap().genome.u[0] =
            (arch_bad_g.elites[&nid].genome.u[0] + 0.25).fract();
        assert!(arch_bad_g.verify_integrity().is_err());
        // A descriptor tampered so that the design maps to another niche: move the mass to the far end of its
        // descriptor range (25-105 g).
        let mut arch_bad_d = arch_chk.clone();
        let m0 = arch_bad_d.elites[&nid].record.descriptors.mass_g;
        arch_bad_d.elites.get_mut(&nid).unwrap().record.descriptors.mass_g =
            if m0 > 65.0 { 25.0 } else { 105.0 };
        assert_ne!(arch_bad_d.niche_of(&arch_bad_d.elites[&nid].record.descriptors), nid);
        assert!(arch_bad_d.verify_integrity().is_err());
        let mut arch_bad_sha = arch_chk.clone();
        arch_bad_sha.decoder_sha256 = "0".repeat(64);
        assert!(arch_bad_sha.verify_integrity().is_err());

        // 9. P6 / S9-4: Verify Tier-2 solid-tip TET10 mesh convergence (full xs outline including exact tip_xy,
        //    the production max area `TIP_FE_MAX_AREA_MM2` vs a finer 0.08 mm^2 agreeing within 15%) across 2 fixture
        //    genomes.
        let mut g_tip2 = g_solid.clone();
        set_phys(&mut g_tip2, "blade_len", 82.0);
        set_phys(&mut g_tip2, "hook_drop", 42.0);
        set_phys(&mut g_tip2, "tip_reserve", 9.0);
        for (case_idx, g_case) in [&g_solid, &g_tip2].into_iter().enumerate() {
            let p_c = g_case.decode();
            let blade_c = build_blade_profile_2d(&p_c);
            let mut b_outer = Vec::with_capacity(blade_c.xs.len() * 2);
            for i in 0..blade_c.xs.len() {
                b_outer.push((blade_c.xs[i], blade_c.y_in[i]));
            }
            for i in (0..blade_c.xs.len()).rev() {
                b_outer.push((blade_c.xs[i], blade_c.y_sp[i]));
            }
            let max_x = b_outer
                .iter()
                .map(|q| q.0)
                .fold(f64::NEG_INFINITY, f64::max);
            assert!(
                (max_x - blade_c.tip_xy.0).abs() < 1e-9,
                "P6 case {case_idx}: outline max_x {max_x} must equal blade.tip_xy.0 {}",
                blade_c.tip_xy.0
            );
            let x0 = blade_c.x_tip - 32.0;
            let y0 = blade_c.tip_xy.1 - 32.0;
            let th_fn = |x: f64, y: f64| -> Option<f64> {
                let (t, d2) =
                    crate::g5_sdf::eval_blade_thickness_and_sdf2(&p_c, &blade_c, x, y);
                (d2 <= 0.0 && t > 0.05).then_some(t)
            };
            let a_prod = crate::g5_qd::TIP_FE_MAX_AREA_MM2;
            let m_04 = jarambit_physics::fine::mesh_domain(&b_outer, &[], a_prod).expect("production mesh");
            let near_04 = m_04
                .xy
                .iter()
                .filter(|q| (q.0 - blade_c.tip_xy.0).hypot(q.1 - blade_c.tip_xy.1) <= 2.0)
                .count();
            assert!(
                near_04 >= 15,
                "P6 case {case_idx}: expected >= 15 2D vertices within 2 mm tip patch at area {a_prod}, got {near_04}"
            );
            let res_04 = crate::g5_sandwich::solve_g5_solid_tet10_strict(
                m_04.xy,
                m_04.tris,
                blade_c.tip_xy,
                p_c["t_core"],
                x0,
                y0,
                0.5,
                130,
                130,
                &th_fn,
                0,
            )
            .expect("solve_g5_solid_tet10_strict production area");
            let m_015 =
                jarambit_physics::fine::mesh_domain(&b_outer, &[], 0.08).expect("mesh 0.08");
            let res_015 = crate::g5_sandwich::solve_g5_solid_tet10_strict(
                m_015.xy,
                m_015.tris,
                blade_c.tip_xy,
                p_c["t_core"],
                x0,
                y0,
                0.5,
                130,
                130,
                &th_fn,
                0,
            )
            .expect("solve_g5_solid_tet10_strict area 0.08");
            let rel_diff = (res_04.sf_nodal_max - res_015.sf_nodal_max).abs()
                / res_015.sf_nodal_max.max(1e-6);
            assert!(
                rel_diff <= 0.15,
                "P6 case {case_idx}: solid_tip_sf production area ({:.4}) vs 0.08 ({:.4}) rel_diff {:.3} > 0.15",
                res_04.sf_nodal_max,
                res_015.sf_nodal_max,
                rel_diff
            );
        }
    }
}
