#![allow(dead_code, unused_imports)]
include!("modules.rs");
#[path = "prototype_export.rs"] pub mod export;
pub mod compound;
pub mod diagnostic;
pub mod pocket;
#[path = "/home/ubuntu/jarambit/.agent/worktrees/lane-G16-06-RSV2-08/crates/crescent/src/gci.rs"] pub mod gci;
use jarambit_evidence::identity::ContentRef;
use serde_json::{json, Value};
use std::{fs, path::Path, time::Instant};

fn sha(bytes: &[u8]) -> String { ContentRef::from_bytes("phase2", "rev1", bytes).sha256 }

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mode = &args[1];
    let out = Path::new(&args[2]);
    let csv = fs::read(&args[3]).unwrap();
    let text = std::str::from_utf8(&csv).unwrap();
    let g = text.lines().find_map(|l| genome::Genome::from_csv(&l.split(',').collect::<Vec<_>>())).unwrap();
    let mut clamped = g.clone(); clamped.clamp(); assert_eq!(g.v, clamped.v);
    assert_eq!(std::env::var("JARAMBIT_SOLVER").unwrap(), "faer");
    let start = Instant::now();
    let p = pheno::build(&g);
    let rejects = gates::check(&g, &p);
    let aes = aesthetic::evaluate(&g, &p);
    let mut doc = json!({"task":"G16-06-RSV2-08-p5", "prototype_only":true,
        "genome":args[3], "genome_sha256":sha(&csv), "config":compound::config(),
        "input_was_canonical":true, "rs_v2":g.rs_v2(), "rejects":rejects,
        "aes_pass":aes.pass, "aes_fails":aes.fails,
        "provenance":serde_json::from_str::<Value>(include_str!("source_provenance.json")).unwrap(),
        "native_gci_source_sha256":sha(include_bytes!("/home/ubuntu/jarambit/.agent/worktrees/lane-G16-06-RSV2-08/crates/crescent/src/gci.rs")),
        "plate_solver":"faer", "all_gate_qualification":false});
    if mode == "gci" {
        let result = gci::run_plate_gci_3level("G16-06-RSV2-08-p5", &args[3], &sha(&csv), &g);
        doc["plate_gci"] = json!(result);
        doc["G16-15"] = json!(result.as_ref().and_then(|v|v["seq_ratio_gci"]["admitted"].as_bool()) == Some(true));
    } else if mode == "a4" {
        let cert: f64 = args[4].parse().unwrap();
        let (f, fields) = fem::report_fields(&p, 0.25).expect("unbiased certification mesh");
        let mesh = json!({"xy":fields.xy, "tris":fields.tris, "te":fields.te});
        let mesh_bytes = serde_json::to_vec(&mesh).unwrap();
        fs::write(out.with_extension("mesh.json"), &mesh_bytes).unwrap();
        let input = jarambit_physics::side_a4::SideA4Input {
            subject:"G16-06-RSV2-08-p5".into(), genome:args[3].clone(), genome_sha256:sha(&csv),
            d1_sha256:sha(&mesh_bytes), xy:fields.xy, tris:fields.tris, te:fields.te,
            ring_id_mm:p.ring_id, ring_od_mm:p.ring_od, heel_x_mm:p.heel.0, tip_xy_mm:p.tip,
            certified_seq_ratio:cert,
        };
        let r = jarambit_physics::side_a4::solve_side_a4(&input).expect("native side.a4 solve");
        doc["a4"] = json!({"schema":"jarambit.side_report.a4.v1", "status":r.status,
            "genome_sha256":r.genome_sha256, "mesh_sha256":r.d1_sha256,
            "mesh_binding":"sha256 of accompanying mesh JSON, not a full D1 report",
            "mesh_max_area_mm2":r.mesh_max_area_mm2,"n_elem":r.n_elem,"n_node":r.n_node,
            "n_bore_nodes":r.n_bore_nodes,"n_grip_nodes":r.n_grip_nodes,
            "buckling_eigenvalue":r.buckling_eigenvalue,"buckling_critical_load_n":r.buckling_critical_load_n,
            "supported_blade_peak_mpa_per_n":r.supported_blade_peak_mpa_per_n,
            "supported_hand_peak_mpa_per_n":r.supported_hand_peak_mpa_per_n,
            "supported_seq_ratio":r.supported_seq_ratio,"certified_seq_ratio":r.certified_seq_ratio,
            "supported_vs_certified_seq_delta":r.supported_vs_certified_seq_delta,
            "same_mesh_ring_clamp_seq_ratio":f.seq_ratio,
            "bore_radial_k":jarambit_physics::side_a4::BORE_RADIAL_K,
            "bore_tangential_k":jarambit_physics::side_a4::BORE_TANGENTIAL_K,
            "bore_flange_k":jarambit_physics::side_a4::BORE_FLANGE_K,
            "grip_foundation_k":jarambit_physics::side_a4::GRIP_FOUNDATION_K,
            "solver":"native side_a4 faer sparse Cholesky / K-orthogonal Lanczos",
            "partial_solver_edit":false,"changes_seq_ratio":false});
        doc["G16-13"] = json!(r.buckling_eigenvalue >= 1.0 && r.supported_seq_ratio >= cert);
    } else { panic!("expected gci or a4"); }
    doc["seconds"] = json!(start.elapsed().as_secs_f64());
    fs::write(out, serde_json::to_string_pretty(&doc).unwrap()).unwrap();
    eprintln!("{}", serde_json::to_string(&doc).unwrap());
}
