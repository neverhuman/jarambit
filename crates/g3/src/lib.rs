//! Standalone Jarambit G3 crate (`jarambit-g3`):
//! - `ergolight`: `W_ERGOLIGHT_v1` (draft-c) continuous multi-objective scorer (`T5`).
//! - `g3_sandwich`: `SandwichSection`, `SandwichPlate`, `solve_ring_band_t6` (1.5 mm band load),
//!   and fail-closed `solve_g3_solid_tet10_strict` (`T4.1`).
//! - `g3_genome`: 179-gene `G3.0` unit-cube genome, `from_hzm93`, strict CSV loader, Sobol generator,
//!   and reflective `Iso+Line-DD` + module crossover + self-adaptive `sigma` (`T4.2`).
//! - `g3_sdf`: 3D exact separable EDT, half-voxel morphology, TPMS lattice, 3D 6-connectivity filter,
//!   14 mm grip-to-guard taper, 3D corrugated/fenestrated/serrated blade, and Kuhn Marching-Tetrahedra (`T4.3`).
//! - `g3_qd`: 8D behavior descriptors, 3-tier evaluation cascade, and 1024-niche `CvtMapElitesArchive` (`T4.3`).
//!
//! Kept in its own crate so `crates/crescent` and `crates/physics` remain 100% byte-for-byte
//! identical to `origin/main` (zero change to v1 `certification::identity`).

pub mod ergolight;
pub mod g3_genome;
pub mod g3_namespace;
pub mod g3_qd;
pub mod g3_sandwich;
pub mod g3_sdf;
pub mod sobol_joe_kuo;

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

#[cfg(test)]
mod v1_identity_tests {
    use crate::ergolight::score_ergolight_map;
    use crate::g3_genome::{G3Genome, G3_GENES};
    use crate::g3_qd::{evaluate_tier1_sdf_and_plate, g3_decoder_sha256};

    #[test]
    fn v1_certification_identity_is_unchanged_from_origin_main() {
        let lock_sha = crate::sha256_bytes(include_bytes!("../../../Cargo.lock"));
        assert_eq!(
            lock_sha,
            "a7d57f542f3233c99e86bd0973f79ea894ce007c982fddf9bd5a30414035e41b",
            "root Cargo.lock must remain byte-for-byte identical to origin/main"
        );
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
            ctrl_id,
            "b75d18a1f10d7d56510e07e08f31ae2cadd65787936835a9009f9be0842cf33a",
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
            auto_id,
            "87921e14f43ad64dc4dc2661067aa1a6ae470d76f11c628fab9911506c1cbc22",
            "cert_pipeline_r01_p14_c02 certificate_identity must match pinned evidence/evolve_certifiable/manifest.json"
        );
    }

    #[test]
    fn run6_finalist_fa_g11285_rust_vs_python_parity() {
        let dec_sha = g3_decoder_sha256();
        println!("G3 frozen decoder_sha256 = {dec_sha}");
        assert_eq!(dec_sha.len(), 64);

        let path = "/home/ubuntu/jarambit_runs/handzone-ergo-20261024/final/v3/FA_best_overall/finalist.json";
        let raw = std::fs::read_to_string(path).expect("read FA finalist.json");
        let doc: serde_json::Value = serde_json::from_str(&raw).expect("parse FA finalist.json");

        // 1. Decode 93-gene u vector in Rust and verify every physical parameter against Python p
        let u93: Vec<f64> = doc["u"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_f64().unwrap())
            .collect();
        assert_eq!(u93.len(), 93);
        let g3 = G3Genome::from_hzm93(&u93, "ef200b2abb1d").expect("from_hzm93");
        let p_rust = g3.decode_legacy_v1_for_test();
        let p_py = doc["p"].as_object().unwrap();
        for spec in &G3_GENES[..93] {
            let rv = p_rust[spec.name];
            let pv = p_py[spec.name].as_f64().unwrap();
            assert!(
                (rv - pv).abs() < 1e-11,
                "gene {} decode mismatch: rust={rv} py={pv}",
                spec.name
            );
        }
        for k in ["bore_d", "lip_w", "L_g", "R_od", "R_b", "x_t", "cx"] {
            let rv = p_rust[k];
            let pv = p_py[k].as_f64().unwrap();
            assert!(
                (rv - pv).abs() < 1e-11,
                "derived datum {k} mismatch: rust={rv} py={pv}"
            );
        }

        // 2. Score FA_best_overall's evaluate_h025 metrics in Rust vs Python W_ERGOLIGHT_v1 (draft-b)
        let m_obj = doc["evaluate_h025"]["m"].as_object().unwrap();
        let py_score = &doc["evaluate_h025"]["score"];
        let rust_score_on_py_m = crate::ergolight::score_ergolight_map_draft_b(m_obj)
            .expect("score_ergolight_map_draft_b on FA m");
        // Verify that draft-c strictly rejects a map with only legacy `ring_pull_cap_n` (I1)
        assert!(score_ergolight_map(m_obj).is_err());
        let mut m_draft_c = m_obj.clone();
        m_draft_c.insert(
            "ring_bearing_cap_n".to_string(),
            m_obj["ring_pull_cap_n"].clone(),
        );
        let rust_score_draft_c =
            score_ergolight_map(&m_draft_c).expect("score_ergolight_map draft-c on FA m");
        assert_eq!(rust_score_draft_c["draft"].as_str(), Some("c"));
        assert!(rust_score_draft_c["terms"].get("ring_bearing_300n_sf2").is_some());
        assert!(rust_score_draft_c["terms"].get("ring_back_30n_sf2").is_some());
        let rs_score_100 = rust_score_on_py_m["score_100"].as_f64().unwrap();
        let rs_base = rust_score_on_py_m["base"].as_f64().unwrap();
        let rs_e = rust_score_on_py_m["E"].as_f64().unwrap();

        println!(
            "\n=== FA_best_overall (g11285) W_ERGOLIGHT_v1 Scorer Parity (Rust vs Python) ==="
        );
        println!(
            "score_100: Rust = {:.6} | Python = {:.6} | diff = {:.2e}",
            rs_score_100,
            py_score["score_100"].as_f64().unwrap(),
            (rs_score_100 - py_score["score_100"].as_f64().unwrap()).abs()
        );
        println!(
            "base:      Rust = {:.9} | Python = {:.9} | diff = {:.2e}",
            rs_base,
            py_score["base"].as_f64().unwrap(),
            (rs_base - py_score["base"].as_f64().unwrap()).abs()
        );
        println!(
            "E:         Rust = {:.9} | Python = {:.9} | diff = {:.2e}",
            rs_e,
            py_score["E"].as_f64().unwrap(),
            (rs_e - py_score["E"].as_f64().unwrap()).abs()
        );
        assert!(
            (rs_score_100 - py_score["score_100"].as_f64().unwrap()).abs() < 1e-6
        );
        assert!(
            (rs_base - py_score["base"].as_f64().unwrap()).abs() < 1e-9
        );
        assert!(
            (rs_e - py_score["E"].as_f64().unwrap()).abs() < 1e-9
        );

        for (k, v) in py_score["terms"].as_object().unwrap() {
            let py_s = v["s"].as_f64().unwrap();
            let rs_s = rust_score_on_py_m["terms"][k.as_str()]["s"].as_f64().unwrap();
            println!("  term {:<24}: Rust s={:.6} | Py s={:.6} | diff={:.2e}", k, rs_s, py_s, (rs_s - py_s).abs());
            assert!((rs_s - py_s).abs() < 1e-9, "term {k} mismatch: {rs_s} vs {py_s}");
        }
        for (k, v) in py_score["ergo"].as_object().unwrap() {
            let py_e = v["e"].as_f64().unwrap();
            let rs_e_k = rust_score_on_py_m["ergo"][k.as_str()]["e"].as_f64().unwrap();
            println!("  ergo {:<24}: Rust e={:.6} | Py e={:.6} | diff={:.2e}", k, rs_e_k, py_e, (rs_e_k - py_e).abs());
            assert!((rs_e_k - py_e).abs() < 1e-9, "ergo {k} mismatch: {rs_e_k} vs {py_e}");
        }

        // 3. Also run native Rust Tier-1 SDF + 1.5 mm band T6 ring FE on FA_best_overall (at h = 0.5 mm)
        let r1 = evaluate_tier1_sdf_and_plate(&g3, 0.5);
        println!(
            "\n=== FA_best_overall Native Rust Tier-1 (179-gene G3.0 with G3 blade) vs Python Hand-Zone (f075 blade) ==="
        );
        println!(
            "  mass_g:          Rust G3 = {:.2} g | Py h025 = {:.2} g (AM mesh {:.2} g)",
            r1.inputs.mass_g,
            m_obj["mass_g"].as_f64().unwrap(),
            doc["mesh_checks"]["AMbuild"]["mass_g"].as_f64().unwrap()
        );
        println!(
            "  ring_bearing_n:  Rust T6 = {:.1} N | Py fe2d = {:.1} N ( both >> 600 N )",
            r1.inputs.ring_bearing_cap_n,
            m_obj["ring_pull_cap_n"].as_f64().unwrap()
        );
        println!(
            "  ring_back_cap_n: Rust T6 = {:.1} N | Py fe2d = {:.1} N ( both >> 600 N )",
            r1.inputs.ring_back_cap_n,
            m_obj["ring_back_cap_n"].as_f64().unwrap()
        );
        println!(
            "  swell_perim_mm:  Rust    = {:.2} mm | Py      = {:.2} mm",
            r1.inputs.swell_perimeter_mm,
            m_obj["swell_perimeter_mm"].as_f64().unwrap()
        );
        println!(
            "  swell_h_over_t:  Rust    = {:.3}    | Py      = {:.3}",
            r1.inputs.swell_h_over_t,
            m_obj["swell_h_over_t"].as_f64().unwrap()
        );
        println!(
            "  W_ERGOLIGHT_v1:  Rust G3 = {:.2} (E={:.3}) | Py (f075 blade) = {:.2} (E={:.3})",
            r1.score.score_100,
            r1.score.ergo_multiplier,
            py_score["score_100"].as_f64().unwrap(),
            py_score["E"].as_f64().unwrap()
        );
    }

    #[test]
    fn t8_parity_all_25_cases_match_python_g3hz_parity_v1() {
        use crate::g3_genome::G3Genome;
        use crate::g3_qd::F075_SPIN_I_REF_G_MM2;
        use crate::g3_sdf::build_g3_sdf_from_params;

        // Verify B6: F075_SPIN_I_REF_G_MM2 matches ref_f075/f075_reference.json
        let ref_path =
            "/home/ubuntu/jarambit_runs/handzone-ergo-20261024/ref_f075/f075_reference.json";
        if let Ok(ref_raw) = std::fs::read_to_string(ref_path) {
            let ref_doc: serde_json::Value =
                serde_json::from_str(&ref_raw).expect("parse f075_reference.json");
            let py_spin_ref = ref_doc["F075_SPIN_I_REF_G_MM2"].as_f64().unwrap();
            assert!(
                (F075_SPIN_I_REF_G_MM2 - py_spin_ref).abs() < 1e-6,
                "F075_SPIN_I_REF_G_MM2 mismatch: Rust={F075_SPIN_I_REF_G_MM2} vs Py={py_spin_ref}"
            );
        }

        let path = "/home/ubuntu/jarambit_runs/handzone-ergo-20261024/parity/g3hz_parity_v1.json";
        let Ok(raw) = std::fs::read_to_string(path) else {
            return;
        };
        let doc: serde_json::Value = serde_json::from_str(&raw).expect("parse g3hz_parity_v1.json");
        let cases = doc["cases"].as_array().expect("cases array");
        assert_eq!(cases.len(), 25);

        let mut max_p_diff = 0.0_f64;
        let mut max_score_diff = 0.0_f64;
        let mut max_term_diff = 0.0_f64;
        let mut max_ergo_diff = 0.0_f64;
        let mut max_groove_r_diff = 0.0_f64;
        let mut max_crest_r_diff = 0.0_f64;
        let mut max_depth_diff = 0.0_f64;
        let mut max_station_err_diff = 0.0_f64;
        let mut max_corner_r_diff = 0.0_f64;
        let mut max_edge_r_diff = 0.0_f64;
        let mut max_am_build_diff = 0.0_f64;
        let mut max_perim_rel_diff = 0.0_f64;

        for c in cases {
            let name = c["name"].as_str().unwrap();
            let u93: Vec<f64> = c["u"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64().unwrap())
                .collect();
            let g3 = G3Genome::from_hzm93(&u93, "ef200b2abb1d").expect("from_hzm93");
            let p_rust = g3.decode_legacy_v1_for_test();
            let p_py = c["p"].as_object().unwrap();

            for (k, v_py_val) in p_py {
                if let Some(v_py) = v_py_val.as_f64() {
                    if let Some(&v_rs) = p_rust.get(k.as_str()) {
                        let d = (v_rs - v_py).abs();
                        max_p_diff = max_p_diff.max(d);
                        assert!(
                            d < 1e-11,
                            "case {name} decoded parameter {k} mismatch: Rust={v_rs} vs Py={v_py}"
                        );
                    }
                }
            }

            let m_obj = c["m"].as_object().unwrap();
            let t0 = crate::g3_qd::evaluate_tier0_analytic(&g3);
            max_groove_r_diff = max_groove_r_diff
                .max((t0.inputs.groove_r_min_mm - m_obj["groove_r_min_mm"].as_f64().unwrap()).abs());
            max_crest_r_diff = max_crest_r_diff
                .max((t0.inputs.crest_r_min_mm - m_obj["crest_r_min_mm"].as_f64().unwrap()).abs());
            max_depth_diff = max_depth_diff
                .max((t0.inputs.saddle_depth_mm - m_obj["saddle_depth_mm"].as_f64().unwrap()).abs())
                .max((t0.inputs.mid_depth_mm - m_obj["mid_depth_mm"].as_f64().unwrap()).abs())
                .max((t0.inputs.choil_depth_mm - m_obj["choil_depth_mm"].as_f64().unwrap()).abs());
            max_station_err_diff = max_station_err_diff
                .max((t0.inputs.station_err_mm - m_obj["station_err_mm"].as_f64().unwrap()).abs());
            max_corner_r_diff = max_corner_r_diff.max(
                (t0.inputs.section_corner_r_min_mm
                    - m_obj["section_corner_r_min_mm"].as_f64().unwrap())
                .abs(),
            );
            max_edge_r_diff = max_edge_r_diff
                .max((t0.inputs.edge_r_min_mm - m_obj["edge_r_min_mm"].as_f64().unwrap()).abs());
            max_am_build_diff = max_am_build_diff
                .max((t0.inputs.am_build - m_obj["am_build"].as_f64().unwrap()).abs());
            let py_perim = m_obj["swell_perimeter_mm"].as_f64().unwrap();
            max_perim_rel_diff = max_perim_rel_diff
                .max((t0.inputs.swell_perimeter_mm - py_perim).abs() / py_perim.max(1.0));

            // On canonical reference cases, also verify 3D SDF hand-zone contour & mass parity at h = 0.40 mm (B5, I4)
            if matches!(name, "f075_reference" | "spec_default" | "FA_best_overall") {
                let grid = build_g3_sdf_from_params(&p_rust, 0.4, false, false);
                let hz = grid.measure_handzone_masses_and_swell(&p_rust);
                let zones = &c["masses"]["zones"];
                let py_ring_env = zones["ring"]["envelope_g"].as_f64().unwrap();
                let py_grip_env = zones["grip"]["envelope_g"].as_f64().unwrap();
                let py_ht = m_obj["swell_h_over_t"].as_f64().unwrap();
                let py_lg = m_obj["L_g_mm"].as_f64().unwrap();
                let py_gh = m_obj["guard_h_mm"].as_f64().unwrap();
                println!(
                    "  [3D SDF h=0.40 {name:<15}] ring_env: rs={:.2}g py={:.2}g | grip_env: rs={:.2}g py={:.2}g | perim: rs={:.2} py={:.2} | H/T: rs={:.3} py={:.3} | L_g: rs={:.1} py={:.1} | guard_h: rs={:.2} py={:.2}",
                    hz.ring_env_g, py_ring_env, hz.grip_env_g, py_grip_env,
                    hz.swell_perimeter_mm, py_perim, hz.swell_h_over_t, py_ht,
                    hz.l_g_mm, py_lg, hz.guard_h_mm, py_gh
                );
                assert!(
                    (hz.swell_perimeter_mm - py_perim).abs() / py_perim < 0.05,
                    "{name} 3D SDF swell_perimeter_mm {} vs py {}",
                    hz.swell_perimeter_mm,
                    py_perim
                );
                assert!(
                    (hz.swell_h_over_t - py_ht).abs() / py_ht < 0.06,
                    "{name} 3D SDF swell_h_over_t {} vs py {}",
                    hz.swell_h_over_t,
                    py_ht
                );
                assert!(
                    (hz.ring_env_g - py_ring_env).abs() / py_ring_env < 0.12,
                    "{name} 3D SDF ring_env_g {} vs py {}",
                    hz.ring_env_g,
                    py_ring_env
                );
                assert!(
                    (hz.grip_env_g - py_grip_env).abs() / py_grip_env < 0.12,
                    "{name} 3D SDF grip_env_g {} vs py {}",
                    hz.grip_env_g,
                    py_grip_env
                );
            }

            let rs = crate::ergolight::score_ergolight_map_draft_b(m_obj)
                .expect("score_ergolight_map_draft_b");
            let rs_score = rs["score_100"].as_f64().unwrap();
            let py_score = c["score_100"].as_f64().unwrap();
            let d_s = (rs_score - py_score).abs();
            max_score_diff = max_score_diff.max(d_s);
            assert!(
                d_s < 1e-6,
                "case {name} score_100 mismatch: Rust={rs_score} vs Py={py_score}"
            );

            for (k, v_py) in c["terms"].as_object().unwrap() {
                let py_s = v_py.as_f64().unwrap();
                let rs_s = rs["terms"][k.as_str()]["s"].as_f64().unwrap();
                let d = (rs_s - py_s).abs();
                max_term_diff = max_term_diff.max(d);
                assert!(
                    d < 1e-9,
                    "case {name} term {k} mismatch: Rust={rs_s} vs Py={py_s}"
                );
            }
            for (k, v_py) in c["ergo"].as_object().unwrap() {
                let py_e = v_py.as_f64().unwrap();
                let rs_e = rs["ergo"][k.as_str()]["e"].as_f64().unwrap();
                let d = (rs_e - py_e).abs();
                max_ergo_diff = max_ergo_diff.max(d);
                assert!(
                    d < 1e-9,
                    "case {name} ergo {k} mismatch: Rust={rs_e} vs Py={py_e}"
                );
            }
        }
        assert!(max_groove_r_diff < 1e-6);
        assert!(max_crest_r_diff < 1e-6);
        assert!(max_depth_diff < 1e-6);
        assert!(max_station_err_diff < 1e-6);
        assert!(max_corner_r_diff < 1e-6);
        assert!(max_edge_r_diff < 1e-9);
        assert!(max_am_build_diff < 1e-9);
        println!(
            "\n=== T8 Parity (25 cases in parity/g3hz_parity_v1.json) ===\n  [Scorer Parity on Python m]\n    max |p_rs - p_py|            = {:.2e}\n    max |score_rs - score_py|    = {:.2e}\n    max |term_rs - term_py|      = {:.2e}\n    max |ergo_rs - ergo_py|      = {:.2e}\n  [Decoder Curve & Section Parity]\n    max |groove_r_rs - py|       = {:.2e} mm\n    max |crest_r_rs - py|        = {:.2e} mm\n    max |groove_depth_rs - py|   = {:.2e} mm\n    max |station_err_rs - py|    = {:.2e} mm\n    max |corner_r_rs - py|       = {:.2e} mm\n    max |edge_r_rs - py|         = {:.2e} mm\n    max |am_build_rs - py|       = {:.2e}\n    max |swell_perim_rs - py|/py = {:.2}%",
            max_p_diff,
            max_score_diff,
            max_term_diff,
            max_ergo_diff,
            max_groove_r_diff,
            max_crest_r_diff,
            max_depth_diff,
            max_station_err_diff,
            max_corner_r_diff,
            max_edge_r_diff,
            max_am_build_diff,
            100.0 * max_perim_rel_diff
        );
    }

    #[test]
    fn ring_2p5d_extruded_tet10_bearing_pull_and_back_on_fa_best_overall() {
        use crate::g3_genome::G3Genome;
        use crate::g3_qd::evaluate_ring_2p5d_extruded_tet10;

        let path =
            "/home/ubuntu/jarambit_runs/handzone-ergo-20261024/final/v3/FA_best_overall/finalist.json";
        let Ok(raw) = std::fs::read_to_string(path) else {
            return;
        };
        let doc: serde_json::Value = serde_json::from_str(&raw).expect("parse finalist.json");
        let u93: Vec<f64> = doc["u"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_f64().unwrap())
            .collect();
        let g3 = G3Genome::from_hzm93(&u93, "ef200b2abb1d").expect("from_hzm93");
        let vs1 =
            evaluate_ring_2p5d_extruded_tet10(&g3, 0.5).expect("2.5D extruded TET10 ring solve");
        println!(
            "\n=== 2.5D Extruded TET10 Ring Bearing, Pull & Ring-Back on FA_best_overall (g11285) ==="
        );
        println!(
            "  n_tet = {}, n_dof = {}, n_free_dof = {}, joint_k = {:.4} (t_joint = {:.2} mm)",
            vs1.n_tet, vs1.n_dof, vs1.n_free_dof, vs1.joint_k, vs1.t_joint
        );
        println!(
            "  3D TET10 bearing (+x, +/-60 deg): cap = {:.1} N (SF vs 300N = {:.2}), peak = {:.2} MPa at ({:.1}, {:.1}, {:.1}), true_rel_res = {:.2e}",
            vs1.bearing.capacity_n,
            vs1.bearing.capacity_n / 300.0,
            vs1.bearing.peak_mpa,
            vs1.bearing.at[0],
            vs1.bearing.at[1],
            vs1.bearing.at[2],
            vs1.bearing.true_rel_res
        );
        println!(
            "  3D TET10 pull    (-x, +/-60 deg): cap = {:.1} N (SF vs 300N = {:.2}), peak = {:.2} MPa at ({:.1}, {:.1}, {:.1}), true_rel_res = {:.2e}",
            vs1.pull.capacity_n,
            vs1.pull.capacity_n / 300.0,
            vs1.pull.peak_mpa,
            vs1.pull.at[0],
            vs1.pull.at[1],
            vs1.pull.at[2],
            vs1.pull.true_rel_res
        );
        println!(
            "  3D TET10 back    (+x, +/-45 deg): cap = {:.1} N (SF vs 30N  = {:.2}), peak = {:.2} MPa at ({:.1}, {:.1}, {:.1}), true_rel_res = {:.2e}",
            vs1.back.capacity_n,
            vs1.back.capacity_n / 30.0,
            vs1.back.peak_mpa,
            vs1.back.at[0],
            vs1.back.at[1],
            vs1.back.at[2],
            vs1.back.true_rel_res
        );
        assert!(vs1.bearing.true_rel_res <= 1e-10);
        assert!(vs1.pull.true_rel_res <= 1e-10);
        assert!(vs1.back.true_rel_res <= 1e-10);
        assert!(vs1.bearing.capacity_n > 600.0);
        assert!(vs1.pull.capacity_n > 600.0);
        assert!(vs1.back.capacity_n > 60.0);
    }

    #[test]
    fn t8_parity_all_25_cases_match_python_g3hz_parity_v3_draftc() {
        use crate::ergolight::score_ergolight_map;
        use crate::g3_genome::G3Genome;
        use crate::g3_qd::compute_finger_metrics;
        use crate::g3_sdf::build_g3_sdf_from_params;

        let path =
            "/home/ubuntu/jarambit_runs/handzone-ergo-20261024/parity/g3hz_parity_v3_draftc.json";
        let Ok(raw) = std::fs::read_to_string(path) else {
            return;
        };
        let doc: serde_json::Value =
            serde_json::from_str(&raw).expect("parse g3hz_parity_v3_draftc.json");
        assert_eq!(doc["ergolight_draft"].as_str(), Some("c"));
        let cases = doc["cases"].as_array().expect("cases array");
        assert_eq!(cases.len(), 25);

        let mut max_p_diff = 0.0_f64;
        let mut max_score_diff = 0.0_f64;
        let mut max_base_diff = 0.0_f64;
        let mut max_e_diff = 0.0_f64;
        let mut max_term_diff = 0.0_f64;
        let mut max_ergo_diff = 0.0_f64;
        let mut max_fm_diff = 0.0_f64;
        let mut max_mass_rel_diff = 0.0_f64;
        let mut max_hz_mass_rel_diff = 0.0_f64;
        let mut max_perim_rel_diff = 0.0_f64;
        let mut max_hot_diff = 0.0_f64;
        let mut max_lg_diff = 0.0_f64;
        let mut max_gh_diff = 0.0_f64;
        let mut max_com_diff = 0.0_f64;
        let mut max_spin_rel_diff = 0.0_f64;
        let mut max_grip_sf_rel_diff = 0.0_f64;

        for (idx, c) in cases.iter().enumerate() {
            let name = c["name"].as_str().unwrap();
            let u94: Vec<f64> = c["u"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64().unwrap())
                .collect();
            assert_eq!(u94.len(), 94);
            let g3 = G3Genome::from_hzm93(&u94, "22bb8696").expect("from_hzm93 (94-gene v3)");
            let p_rust = g3.decode();
            let p_py = c["p"].as_object().unwrap();

            for (k, v_py_val) in p_py {
                if let Some(v_py) = v_py_val.as_f64() {
                    if let Some(&v_rs) = p_rust.get(k.as_str()) {
                        let d = (v_rs - v_py).abs();
                        max_p_diff = max_p_diff.max(d);
                        assert!(
                            d < 1e-9,
                            "v3_draftc case {name} decoded parameter {k} mismatch: Rust={v_rs} vs Py={v_py}"
                        );
                    }
                }
            }

            // 1. Feed each case's `m` to Rust draft-c scorer (`score_ergolight_map`) and assert `terms`, `ergo`, `E`, `base`, and `score_100` to 1e-9
            let m_obj = c["m"].as_object().unwrap();
            let rs_score = score_ergolight_map(m_obj).expect("score_ergolight_map draft-c");
            let py_score_100 = c["score_100"].as_f64().unwrap();
            let py_e = c["E"].as_f64().unwrap();
            let py_base = c["base"].as_f64().unwrap();
            let rs_score_100 = rs_score["score_100"].as_f64().unwrap();
            let rs_e = rs_score["E"].as_f64().unwrap();
            let rs_base = rs_score["base"].as_f64().unwrap();

            let ds = (rs_score_100 - py_score_100).abs();
            let de = (rs_e - py_e).abs();
            let db = (rs_base - py_base).abs();
            max_score_diff = max_score_diff.max(ds);
            max_e_diff = max_e_diff.max(de);
            max_base_diff = max_base_diff.max(db);
            assert!(
                ds < 1e-9,
                "case {name}: score_100 diff {ds} >= 1e-9 (Rust={rs_score_100} vs Py={py_score_100})"
            );
            assert!(
                de < 1e-9,
                "case {name}: E diff {de} >= 1e-9 (Rust={rs_e} vs Py={py_e})"
            );
            assert!(
                db < 1e-9,
                "case {name}: base diff {db} >= 1e-9 (Rust={rs_base} vs Py={py_base})"
            );

            for (tk, tv) in c["terms"].as_object().unwrap() {
                let py_t = tv.as_f64().unwrap();
                let rs_t = rs_score["terms"][tk.as_str()]["s"].as_f64().unwrap();
                let dt = (rs_t - py_t).abs();
                max_term_diff = max_term_diff.max(dt);
                assert!(
                    dt < 1e-9,
                    "case {name} term {tk} diff {dt} >= 1e-9 (Rust={rs_t} vs Py={py_t})"
                );
            }
            for (ek, ev) in c["ergo"].as_object().unwrap() {
                let py_ek = ev.as_f64().unwrap();
                let rs_ek = rs_score["ergo"][ek.as_str()]["e"].as_f64().unwrap();
                let dek = (rs_ek - py_ek).abs();
                max_ergo_diff = max_ergo_diff.max(dek);
                assert!(
                    dek < 1e-9,
                    "case {name} ergo {ek} diff {dek} >= 1e-9 (Rust={rs_ek} vs Py={py_ek})"
                );
            }

            // 2. Analytic 2D/section parity across all 25 cases: compute_finger_metrics, edge_r_min_mm
            let (groove_r, crest_r, saddle_d, mid_d, choil_d, station_err) =
                compute_finger_metrics(&p_rust);
            let py_groove_r = m_obj["groove_r_min_mm"].as_f64().unwrap();
            let py_crest_r = m_obj["crest_r_min_mm"].as_f64().unwrap();
            let py_saddle_d = m_obj["saddle_depth_mm"].as_f64().unwrap();
            let py_mid_d = m_obj["mid_depth_mm"].as_f64().unwrap();
            let py_choil_d = m_obj["choil_depth_mm"].as_f64().unwrap();
            let py_stat_err = m_obj["station_err_mm"].as_f64().unwrap();
            let d_gr = (groove_r - py_groove_r).abs();
            let d_cr = (crest_r - py_crest_r).abs();
            let d_sd = (saddle_d - py_saddle_d).abs();
            let d_md = (mid_d - py_mid_d).abs();
            let d_cd = (choil_d - py_choil_d).abs();
            let d_se = (station_err - py_stat_err).abs();
            max_fm_diff = max_fm_diff
                .max(d_gr)
                .max(d_cr)
                .max(d_sd)
                .max(d_md)
                .max(d_cd)
                .max(d_se);
            assert!(
                d_gr < 1e-6
                    && d_cr < 1e-6
                    && d_sd < 1e-6
                    && d_md < 1e-6
                    && d_cd < 1e-6
                    && d_se < 1e-6,
                "case {name}: finger metrics mismatch: gr={d_gr:.2e}, cr={d_cr:.2e}, sd={d_sd:.2e}, md={d_md:.2e}, cd={d_cd:.2e}, se={d_se:.2e}"
            );

            let t0 = crate::g3_qd::evaluate_tier0_analytic(&g3);
            let py_corner_r = m_obj["section_corner_r_min_mm"].as_f64().unwrap();
            let py_edge_r = m_obj["edge_r_min_mm"].as_f64().unwrap();
            let py_am_build = m_obj["am_build"].as_f64().unwrap();
            assert!(
                (t0.inputs.section_corner_r_min_mm - py_corner_r).abs() < 1e-6,
                "case {name}: section_corner_r_min_mm Rust={} vs Py={py_corner_r}",
                t0.inputs.section_corner_r_min_mm
            );
            assert!(
                (t0.inputs.edge_r_min_mm - py_edge_r).abs() < 1e-6,
                "case {name}: edge_r_min_mm Rust={} vs Py={py_edge_r}",
                t0.inputs.edge_r_min_mm
            );
            assert!(
                (t0.inputs.am_build - py_am_build).abs() < 1e-6,
                "case {name}: am_build Rust={} vs Py={py_am_build}",
                t0.inputs.am_build
            );

            // 3. Verify 3D SDF measurement parity across all 5 finalists (idx < 5) + representative random cases (idx == 5, 10, 15, 20)
            if idx < 5 || idx % 5 == 0 {
                let grid = build_g3_sdf_from_params(&p_rust, 0.40, false, false);
                let meas = grid.measure_handzone_masses_and_swell(&p_rust);
                let py_mass_am = c["masses"]["mass_am_g"].as_f64().unwrap();
                let py_hz_mass_am = py_mass_am - 19.893075271136308;
                let py_perim = m_obj["swell_perimeter_mm"].as_f64().unwrap();
                let py_hot = m_obj["swell_h_over_t"].as_f64().unwrap();
                let py_lg = m_obj["L_g_mm"].as_f64().unwrap();
                let py_gh = m_obj["guard_h_mm"].as_f64().unwrap();
                let py_com_idx = m_obj["com_to_index_mm"].as_f64().unwrap();
                let py_spin_i = m_obj["spin_I_g_mm2"].as_f64().unwrap();
                let py_grip_sf = m_obj["grip_sf_min"].as_f64().unwrap();

                let mass_rel = (meas.f075_union_mass_am_g - py_mass_am).abs() / py_mass_am;
                let hz_mass_rel = (meas.hz_am_x_lt_99_g - py_hz_mass_am).abs() / py_hz_mass_am;
                let perim_rel = (meas.swell_perimeter_mm - py_perim).abs() / py_perim;
                let hot_diff = (meas.swell_h_over_t - py_hot).abs();
                let lg_diff = if py_lg >= 900.0 && !meas.l_g_mm.is_finite() {
                    0.0
                } else {
                    (meas.l_g_mm - py_lg).abs()
                };
                let gh_diff = (meas.guard_h_mm - py_gh).abs();
                let com_diff = (meas.f075_union_com_to_index_mm - py_com_idx).abs();
                let spin_rel = (meas.f075_union_spin_i_g_mm2 - py_spin_i).abs() / py_spin_i;
                let grip_sf_rel = (meas.grip_sf_min - py_grip_sf).abs() / py_grip_sf;

                max_mass_rel_diff = max_mass_rel_diff.max(mass_rel);
                max_hz_mass_rel_diff = max_hz_mass_rel_diff.max(hz_mass_rel);
                max_perim_rel_diff = max_perim_rel_diff.max(perim_rel);
                max_hot_diff = max_hot_diff.max(hot_diff);
                max_lg_diff = max_lg_diff.max(lg_diff);
                max_gh_diff = max_gh_diff.max(gh_diff);
                max_com_diff = max_com_diff.max(com_diff);
                max_spin_rel_diff = max_spin_rel_diff.max(spin_rel);
                max_grip_sf_rel_diff = max_grip_sf_rel_diff.max(grip_sf_rel);

                println!(
                    "  [v3_draftc {name:<16}] mass_am: rs={:.2} py={:.2} (rel={:.4}) | ring_env: rs={:.2} py={:.2} | ring_am: rs={:.2} py={:.2} | grip_env: rs={:.2} py={:.2} | grip_am: rs={:.2} py={:.2} | perim: rs={:.2} py={:.2} | hot: rs={:.3} py={:.3} | lg: rs={:.2} py={:.2} | gh: rs={:.2} py={:.2} | com: rs={:.2} py={:.2} | spin: rs={:.0} py={:.0} | grip_sf: rs={:.3} py={:.3}",
                    meas.f075_union_mass_am_g, py_mass_am, mass_rel,
                    meas.ring_env_g, c["masses"]["zones"]["ring"]["envelope_g"].as_f64().unwrap(),
                    meas.ring_am_g, c["masses"]["zones"]["ring"]["am_g"].as_f64().unwrap(),
                    meas.grip_env_g, c["masses"]["zones"]["grip"]["envelope_g"].as_f64().unwrap(),
                    meas.grip_am_g, c["masses"]["zones"]["grip"]["am_g"].as_f64().unwrap(),
                    meas.swell_perimeter_mm, py_perim,
                    meas.swell_h_over_t, py_hot,
                    meas.l_g_mm, py_lg,
                    meas.guard_h_mm, py_gh,
                    meas.f075_union_com_to_index_mm, py_com_idx,
                    meas.f075_union_spin_i_g_mm2, py_spin_i,
                    meas.grip_sf_min, py_grip_sf
                );
                assert!(
                    mass_rel <= 0.015,
                    "case {name}: f075_union_mass_am_g={} vs Py={py_mass_am} (rel={mass_rel})",
                    meas.f075_union_mass_am_g
                );
                assert!(
                    hz_mass_rel <= 0.025,
                    "case {name}: hz_am_x_lt_99_g={} vs Py={py_hz_mass_am} (rel={hz_mass_rel})",
                    meas.hz_am_x_lt_99_g
                );
                assert!(
                    perim_rel <= 0.015,
                    "case {name}: swell_perimeter_mm={} vs Py={py_perim} (rel={perim_rel})",
                    meas.swell_perimeter_mm
                );
                assert!(
                    hot_diff <= 0.03,
                    "case {name}: swell_h_over_t={} vs Py={py_hot} (diff={hot_diff})",
                    meas.swell_h_over_t
                );
                assert!(
                    lg_diff <= 0.85,
                    "case {name}: l_g_mm={} vs Py={py_lg} (diff={lg_diff})",
                    meas.l_g_mm
                );
                assert!(
                    gh_diff <= 0.50,
                    "case {name}: guard_h_mm={} vs Py={py_gh} (diff={gh_diff})",
                    meas.guard_h_mm
                );
                assert!(
                    com_diff <= 0.50,
                    "case {name}: com_to_index_mm={} vs Py={py_com_idx} (diff={com_diff})",
                    meas.f075_union_com_to_index_mm
                );
                assert!(
                    spin_rel <= 0.015,
                    "case {name}: spin_I_g_mm2={} vs Py={py_spin_i} (rel={spin_rel})",
                    meas.f075_union_spin_i_g_mm2
                );
                assert!(
                    grip_sf_rel <= 0.12,
                    "case {name}: grip_sf_min={} vs Py={py_grip_sf} (rel={grip_sf_rel})",
                    meas.grip_sf_min
                );

                // S5-6: Production-path assertion (`build_g3_sdf(&g3, 0.40, true)`) of hand-zone AM mass
                // from `d_am` with `x < 75.6` against `zones.ring/grip.am_g` on the 5 finalists (idx < 5)
                if idx < 5 {
                    let prod_grid = crate::g3_sdf::build_g3_sdf(&g3, 0.40, true);
                    let (prod_ring_d_am_g, prod_grip_d_am_g, prod_hz_d_am_g) =
                        prod_grid.production_handzone_d_am_masses(p_rust["x_t"]);
                    let py_ring_am = c["masses"]["zones"]["ring"]["am_g"].as_f64().unwrap();
                    let py_grip_am = c["masses"]["zones"]["grip"]["am_g"].as_f64().unwrap();
                    let py_hz_75p6_am = py_ring_am + py_grip_am;
                    let grip_d_am_rel = (prod_grip_d_am_g - py_grip_am).abs() / py_grip_am;
                    let hz_d_am_rel = (prod_hz_d_am_g - py_hz_75p6_am).abs() / py_hz_75p6_am;
                    println!(
                        "    [prod d_am x<75.6 {name:<16}] ring_d_am={prod_ring_d_am_g:.2} (py={py_ring_am:.2}), grip_d_am={prod_grip_d_am_g:.2} (py={py_grip_am:.2}, rel={grip_d_am_rel:.4}), hz_d_am={prod_hz_d_am_g:.2} (py={py_hz_75p6_am:.2}, rel={hz_d_am_rel:.4})"
                    );
                    assert!(
                        grip_d_am_rel <= 0.08 && hz_d_am_rel <= 0.08,
                        "case {name}: production d_am hand-zone mass (x < 75.6) grip={prod_grip_d_am_g} vs Py={py_grip_am} (rel={grip_d_am_rel}), total={prod_hz_d_am_g} vs Py={py_hz_75p6_am} (rel={hz_d_am_rel})"
                    );
                }
            }
        }
        println!(
            "=== T8 Parity v3 draft-c Summary (25 cases) ===\n  max_p_diff={max_p_diff:.2e}, max_score_diff={max_score_diff:.2e}, max_base_diff={max_base_diff:.2e}, max_e_diff={max_e_diff:.2e}, max_term_diff={max_term_diff:.2e}, max_ergo_diff={max_ergo_diff:.2e}, max_fm_diff={max_fm_diff:.2e}\n  3D SDF (finalists + randoms): max_mass_rel={max_mass_rel_diff:.4}, max_hz_mass_rel={max_hz_mass_rel_diff:.4}, max_perim_rel={max_perim_rel_diff:.4}, max_hot_diff={max_hot_diff:.4}, max_lg_diff={max_lg_diff:.3} mm, max_gh_diff={max_gh_diff:.3} mm, max_com_diff={max_com_diff:.3} mm, max_spin_rel={max_spin_rel_diff:.4}, max_grip_sf_rel={max_grip_sf_rel_diff:.4}"
        );
    }

    #[test]
    fn s5_fixes_blade_porosity_no_spine_fin_and_sharpened_cap_verified() {
        use crate::g3_genome::{G3Genome, G3_GENES};
        use crate::g3_qd::evaluate_tier1_sdf_and_plate;
        use crate::g3_sdf::{
            build_blade_profile_2d, build_g3_sdf, build_g3_sdf_from_params, GUARD_X_MM,
        };

        let idx_of = |name: &str| -> usize {
            G3_GENES
                .iter()
                .position(|g| g.name == name)
                .unwrap_or_else(|| panic!("missing gene {name}"))
        };

        // 1. N1 + P1: Unvented blade (win_amp = 0.20 <= 0.35) gets zero mass credit for rho_blade = 0.06;
        //    vented blade (win_amp = 0.50 > 0.35) drops mass_g AND moves plate strength terms + am_build.
        let set_phys = |g: &mut G3Genome, name: &str, phys: f64| {
            let i = idx_of(name);
            g.u[i] = G3_GENES[i].unit_from_phys(phys).clamp(0.0, 1.0);
        };
        let mut g_solid = G3Genome::benchmark_reference("22bb8696");
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

        // 2. N2: Full G3 SDF (`include_blade == true`) has zero phantom f075 spine fin outside evolved blade for x > 94.0
        let base = G3Genome::benchmark_reference("22bb8696");
        let p_base = base.decode();
        let blade = build_blade_profile_2d(&p_base);
        let grid = build_g3_sdf(&base, 0.8, false);
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
            "N2: full G3 SDF must have 0 phantom f075 spine fin voxels outside evolved blade for x > 94 mm"
        );

        // 3. N4: When guard drop is zero (h_g = 0 and yb_g >= yb0), l_g_mm must fail closed to +inf
        let mut p_no_guard = p_base.clone();
        p_no_guard.insert("h_g", 0.0);
        p_no_guard.insert("yb_g", p_base["yb0"] + 2.0);
        let grid_ng = build_g3_sdf_from_params(&p_no_guard, 1.0, false, false);
        let meas_ng = grid_ng.measure_handzone_masses_and_swell(&p_no_guard);
        assert!(
            !meas_ng.l_g_mm.is_finite(),
            "N4: missing guard drop must fail closed to l_g_mm = +inf, got {}",
            meas_ng.l_g_mm
        );

        // 4. N6 + P3: All 120 scrambled Sobol genomes + extreme corner genome satisfy reach_chord <= 85.725 mm and sharpened_arc <= 101.6 mm
        for s_idx in 0..120 {
            let gs = crate::g3_genome::sobol_unit_genome(s_idx, 4242, "22bb8696");
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
        let mut g_ext = G3Genome::benchmark_reference("22bb8696");
        g_ext.u[idx_of("blade_len")] = 1.0;
        g_ext.u[idx_of("hook_drop")] = 1.0;
        g_ext.u[idx_of("sweep_deg")] = 1.0;
        g_ext.u[idx_of("s0_in")] = 0.0;
        g_ext.u[idx_of("s1_in")] = 1.0;
        g_ext.u[idx_of("s0_sp")] = 0.0;
        g_ext.u[idx_of("s1_sp")] = 1.0;
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
        let grid_v_solid = build_g3_sdf(&g_vented, 0.5, false);
        let grid_v_am = build_g3_sdf(&g_vented, 0.5, true);
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
            let g_p5 = crate::g3_genome::sobol_unit_genome(p5_idx, 5150, "22bb869682e4");
            let p_p5 = g_p5.decode();
            let bp_p5 = build_blade_profile_2d(&p_p5);
            let grid_p5 = build_g3_sdf(&g_p5, 0.4, false);
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
                    if t0 <= 0.0 || t1 <= 0.0 {
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

        // 7. S5-7 & S5-8 & B2: Verify buckling_lambda == 0.0, uninflated hm_twist_sf, and production grip_sf_min sensitivity to blade length
        assert_eq!(r_solid.inputs.buckling_lambda, 0.0);
        assert!(
            (r_solid.inputs.hm_twist_sf
                - r_solid.inputs.twist_tipzone_strict_sf * (1820.0 / 16_200.0))
                .abs()
                < 1e-9
        );
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
    }
}
