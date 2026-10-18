//! Diagnostic replay of the recorded compound prototype; production solvers stay unchanged.
#![allow(dead_code, unused_imports)]
include!("modules.rs");
#[path = "prototype_export.rs"]
pub mod export;
pub mod compound;
use jarambit_crescent::gci::{compute_gci, Level, MeshRef, Observable};
use jarambit_evidence::identity::ContentRef;
use jarambit_physics::side_a4::{self, SideA4Input};
use serde_json::{json, Value};
use std::{env, fs, path::Path, time::Instant};

fn sha(bytes: &[u8]) -> String {
    ContentRef::from_bytes("audit", "rev1", bytes).sha256
}

fn metrics(f: &fem::Fem) -> Value {
    json!({"seq_ratio": f.seq_ratio, "sf_hunter": f.sf_hunter,
        "lat20_n": f.lat20, "lat_rob_n": f.lat_rob, "fat_sf": f.fat_sf,
        "hm_twist_sf": f.hm_twist_sf, "hm_spine_sf": f.hm_spine_sf,
        "hm_ringback_sf": f.hm_ringback_sf})
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let seed = &args[1];
    let out = Path::new(&args[2]);
    let bytes = fs::read(seed).unwrap();
    let text = std::str::from_utf8(&bytes).unwrap();
    let g = text.lines().find_map(|l| genome::Genome::from_csv(&l.split(',').collect::<Vec<_>>())).unwrap();
    let mut clamped = g.clone();
    clamped.clamp();
    assert_eq!(g.v, clamped.v, "audit must not silently clamp the candidate");
    let p = pheno::build(&g);
    let mut rows = Vec::new();
    let mut fine_fields = None;
    for area in [0.25_f64, 1.0, 4.0] {
        let start = Instant::now();
        let (f, fl) = fem::report_fields(&p, area).expect("native plate solve");
        let mesh = json!({"xy": fl.xy, "tris": fl.tris, "te": fl.te});
        let mesh_bytes = serde_json::to_vec(&mesh).unwrap();
        let mesh_path = out.join(format!("mesh_{area}.json"));
        fs::write(&mesh_path, &mesh_bytes).unwrap();
        let row = json!({"max_area_mm2": area, "h_mm": area.sqrt(),
            "mesh_file": mesh_path, "mesh_sha256": sha(&mesh_bytes),
            "n_node": fl.xy.len(), "n_elem": fl.tris.len(),
            "metrics": metrics(&f), "seconds": start.elapsed().as_secs_f64()});
        eprintln!("plate {row}");
        rows.push(row);
        if area == 0.25 { fine_fields = Some(fl); }
    }
    let start = Instant::now();
    let middle_cert = fem::run_unbiased(&p, 0.125).expect("second certification mesh");
    let reference = rows[0]["metrics"]["seq_ratio"].as_f64().unwrap().min(middle_cert.seq_ratio);
    let certification_reference = json!({"definition": "min(seq_ratio at 0.25, 0.125 mm2)",
        "seq_ratio": reference, "area_0p125_metrics": metrics(&middle_cert),
        "seconds": start.elapsed().as_secs_f64()});
    let mut fits = serde_json::Map::new();
    for (key, region) in [("seq_ratio", "blade_vs_ring_neck"),
        ("sf_hunter", "blade_outside_4mm"), ("hm_twist_sf", "native_run_b_eligible_region"),
        ("hm_spine_sf", "native_run_b_eligible_region"),
        ("hm_ringback_sf", "native_run_b_eligible_region"), ("fat_sf", "blade_outside_4mm")] {
        let levels: Vec<_> = rows.iter().map(|r| Level::new(MeshRef {
            id: format!("critical_fem_3_area_{}", r["max_area_mm2"]),
            revision: "compound_audit_rev1".into(), sha256: r["mesh_sha256"].as_str().unwrap().into(),
        }, r["h_mm"].as_f64().unwrap(), Observable {
            id: key.into(), units: "ratio".into(), region: region.into(),
            projection: "plate_cst_dkt".into(), metric: key.into(),
        })).collect();
        let values: Vec<_> = rows.iter().map(|r| r["metrics"][key].as_f64().unwrap()).collect();
        fits.insert(key.into(), compute_gci(values[0], values[1], values[2], 2.0, 1.25, Some(&levels)).to_json());
    }
    let fl = fine_fields.unwrap();
    let start = Instant::now();
    let a4 = side_a4::solve_side_a4(&SideA4Input {
        subject: "critical_fem_3_compound_prototype".into(), genome: seed.into(),
        genome_sha256: sha(&bytes), d1_sha256: rows[0]["mesh_sha256"].as_str().unwrap().into(),
        xy: fl.xy, tris: fl.tris, te: fl.te, ring_id_mm: p.ring_id,
        ring_od_mm: p.ring_od, heel_x_mm: p.heel.0, tip_xy_mm: p.tip,
        certified_seq_ratio: reference,
    }).expect("native side_a4 solve");
    let side = json!({"schema": "jarambit.side_report.a4.v1", "status": a4.status,
        "mesh_max_area_mm2": a4.mesh_max_area_mm2, "n_node": a4.n_node, "n_elem": a4.n_elem,
        "mesh_sha256": a4.d1_sha256,
        "bore_support": {"model": "radial_pin_bearing_and_flange", "n_bore_nodes": a4.n_bore_nodes,
            "k_radial_n_per_mm": side_a4::BORE_RADIAL_K, "k_tangential_n_per_mm": side_a4::BORE_TANGENTIAL_K,
            "k_flange_n_per_mm": side_a4::BORE_FLANGE_K},
        "grip_foundation": {"model": "winkler_grip_pad_foundation", "n_grip_nodes": a4.n_grip_nodes,
            "k_foundation_n_per_mm3": side_a4::GRIP_FOUNDATION_K},
        "e_mpa": side_a4::E_MPA, "nu": side_a4::NU,
        "eigensolver": "lanczos_k_orth; maximum 32 steps",
        "buckling_eigenvalue": a4.buckling_eigenvalue,
        "buckling_critical_load_n": a4.buckling_critical_load_n,
        "supported_blade_peak_mpa_per_n": a4.supported_blade_peak_mpa_per_n,
        "supported_hand_peak_mpa_per_n": a4.supported_hand_peak_mpa_per_n,
        "supported_seq_ratio": a4.supported_seq_ratio,
        "certified_seq_ratio": a4.certified_seq_ratio,
        "supported_vs_certified_seq_delta": a4.supported_vs_certified_seq_delta,
        "native_status_rule": "buckling_eigenvalue > 0 && supported_seq_ratio >= certified_seq_ratio",
        "buckling_above_260n": a4.buckling_eigenvalue >= 1.0,
        "changes_seq_ratio": false, "partial_solver_edit": false,
        "seconds": start.elapsed().as_secs_f64()});
    let result = json!({"candidate": seed, "candidate_sha256": sha(&bytes), "genes": g.v,
        "compound_config": compound::config(), "prototype_only": true,
        "native_geometry_pass": g.rs_v2() && aesthetic::evaluate(&g, &p).pass && gates::check(&g, &p).is_empty(),
        "levels_fine_to_coarse": rows, "refinement_ratio_r": 2.0, "safety_factor_fs": 1.25,
        "gci": fits, "certification_reference": certification_reference, "side_a4": side});
    fs::write(out.join("raw.json"), serde_json::to_string_pretty(&result).unwrap() + "\n").unwrap();
    eprintln!("side_a4 {}", result["side_a4"]);
}
