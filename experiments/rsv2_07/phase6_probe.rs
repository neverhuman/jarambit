//! Full plate/native GCI/A4 and production-export observations for Phase 6.
#![allow(dead_code, unused_imports)]
include!("modules.rs");
#[path = "prototype_export.rs"] pub mod export;
pub mod compound;
use jarambit_crescent::gci::{compute_gci, Level, MeshRef, Observable};
use jarambit_evidence::identity::ContentRef;
use jarambit_physics::side_a4::{self, SideA4Input};
use serde_json::{json, Value};
use std::{env, fs, path::Path, time::Instant};

fn sha(bytes: &[u8]) -> String { ContentRef::from_bytes("phase6", "rev1", bytes).sha256 }
fn metrics(f: &fem::Fem) -> Value {
    json!({"lat20":f.lat20,"lat_rob":f.lat_rob,"ring_rob":f.ring_rob,
        "sf_hiker":f.sf_hiker,"sf_hunter":f.sf_hunter,"sf_ring":f.sf_ring,"floors_ok":f.floors_ok,
        "twist_sf":f.twist_sf,"twist_body_sf":f.twist_body_sf,"twist_tipzone_sf":f.twist_tipzone_sf,
        "seq_ratio":f.seq_ratio,"seq_case":f.seq_case,"fat_sf":f.fat_sf,
        "hm_twist_sf":f.hm_twist_sf,"hm_spine_sf":f.hm_spine_sf,"hm_ringback_sf":f.hm_ringback_sf,
        "hm_tip_inplane_sf":f.hm_tip_inplane_sf,"nodes":f.nodes,"tris":f.tris})
}
fn main() {
    let args: Vec<_> = env::args().collect();
    let bytes = fs::read(&args[1]).unwrap();
    let text = std::str::from_utf8(&bytes).unwrap();
    let g = text.lines().find_map(|l|genome::Genome::from_csv(&l.split(',').collect::<Vec<_>>())).unwrap();
    let mut canonical = g.clone(); canonical.clamp(); assert_eq!(g.v,canonical.v);
    let out = Path::new(&args[2]); let label = &args[3];
    let p = pheno::build(&g); let aes = aesthetic::evaluate(&g,&p);
    let mut doc = json!({"candidate":label,"candidate_sha256":sha(&bytes),"genes":g.v,
        "prototype_only":true,"compound_config":compound::config(),"canonical":true,
        "rejects":gates::check(&g,&p),"rs_v2":p.rs_v2,"t_joint":p.t_joint,"mass_raster_g":p.mass_g,
        "aes":{"pass":aes.pass,"fails":aes.fails,"score":aes.score,"hook_drop_mm":aes.hook_drop,
            "hausdorff_mm":aes.hausdorff,"emd":aes.emd,"inside_concave":aes.inside_concave,"spine_convex":aes.spine_convex},
        "geometry":{"ring_id_mm":p.ring_id,"ring_wall_mm":g.v[genome::RING_WALL],"groove_w_mm":p.groove_w,
            "teeth_in":p.teeth_in,"teeth_sp":p.teeth_sp,"tip_to_heel_mm":p.blade.tip_to_heel,
            "horizontal_reach_mm":p.blade.horizontal_reach,"sharpened_arc_mm":p.blade.sharpened_arc,
            "window_count":p.n_win,"window_area_mm2":p.win_area,"hole_names":p.hole_names,"tip":p.tip}});
    if args.get(4).map(|x|x.as_str()) == Some("export") {
        let s = export::solid(&p,1.0).expect("production solid");
        let mut surface=export::surface_json(&s);
        surface["tip"]=json!(p.tip); surface["rs_v2"]=json!(p.rs_v2);
        surface["prototype_only"]=json!(true); surface["compound_config"]=json!(compound::config());
        surface["genes"]=json!(g.v); surface["export_call_max_area_mm2"]=json!(1.0);
        fs::write(out.join("surface.json"),serde_json::to_vec(&surface).unwrap()).unwrap();
        // Both serializations call the unmodified production exporter with a fixed label.
        fs::write(out.join("candidate.step"),export::step(&s,label)).unwrap();
        fs::write(out.join("candidate.stl"),export::stl(&s,label)).unwrap();
        let (a,b,c)=export::census(&s.tris);
        doc["solid"]=json!({"census":[a,b,c],"volume_mm3":export::volume(&s),"vertices":s.verts.len(),"triangles":s.tris.len()});
        doc["export_stdout"]=json!(format!("boundary {a} nonmanifold {b} dmax {c} volume {:.9} mm3",export::volume(&s)));
        doc["measurements"]=json!(certification::measure(&p));
        doc["native_bearing_width_and_band_mm"]=json!(fem::ring_bearing(&p));
        fs::write(out.join("export.json"),serde_json::to_vec_pretty(&doc).unwrap()).unwrap();
        eprintln!("export complete {label}"); return;
    }
    let mut rows=Vec::new();
    for area in [4.0_f64,1.0,0.25,0.125,0.0625] {
        let start=Instant::now();
        let (f,fl)=fem::report_fields(&p,area).expect("native plate solve");
        let mesh=serde_json::to_vec(&json!({"xy":fl.xy,"tris":fl.tris,"te":fl.te})).unwrap();
        fs::write(out.join(format!("mesh_{area}.json")),&mesh).unwrap();
        let row=json!({"area_mm2":area,"h_mm":area.sqrt(),"mesh_sha256":sha(&mesh),
            "metrics":metrics(&f),"seconds":start.elapsed().as_secs_f64()});
        eprintln!("{label} plate {row}");rows.push(row);
        doc["levels_coarse_to_fine"]=json!(rows);
        fs::write(out.join("plate.json"),serde_json::to_vec_pretty(&doc).unwrap()).unwrap();
    }
    let mut fits=serde_json::Map::new();
    for (key,region) in [("seq_ratio","blade_vs_ring_neck"),("sf_hunter","blade_outside_4mm"),
        ("hm_twist_sf","native_run_b_eligible_region"),("hm_spine_sf","native_run_b_eligible_region"),
        ("hm_ringback_sf","native_run_b_eligible_region"),("fat_sf","blade_outside_4mm")] {
        let levels:Vec<_>=[2usize,1,0].iter().map(|&i|Level::new(MeshRef {
            id:format!("{label}_area_{}",rows[i]["area_mm2"]),revision:"phase6_rev1".into(),
            sha256:rows[i]["mesh_sha256"].as_str().unwrap().into()},rows[i]["h_mm"].as_f64().unwrap(),
            Observable {id:key.into(),units:"ratio".into(),region:region.into(),projection:"plate_cst_dkt".into(),metric:key.into()})).collect();
        let vals:Vec<_>=[2usize,1,0].iter().map(|&i|rows[i]["metrics"][key].as_f64().unwrap()).collect();
        fits.insert(key.into(),compute_gci(vals[0],vals[1],vals[2],2.0,1.25,Some(&levels)).to_json());
    }
    doc["gci"]=json!(fits);
    let start=Instant::now();
    let (cert,fl)=fem::certify(&p).expect("native certification pair");
    doc["certification"]=metrics(&cert);
    doc["certification_seconds"]=json!(start.elapsed().as_secs_f64());
    // Bind A4 to the actual fields returned by native certify, independently of display names.
    let mesh=serde_json::to_vec(&json!({"xy":fl.xy,"tris":fl.tris,"te":fl.te})).unwrap();
    assert_eq!(sha(&mesh),rows[2]["mesh_sha256"].as_str().unwrap());
    let start=Instant::now();
    let a4=side_a4::solve_side_a4(&SideA4Input {subject:label.clone(),genome:args[1].clone(),
        genome_sha256:sha(&bytes),d1_sha256:sha(&mesh),xy:fl.xy,tris:fl.tris,te:fl.te,
        ring_id_mm:p.ring_id,ring_od_mm:p.ring_od,heel_x_mm:p.heel.0,tip_xy_mm:p.tip,
        certified_seq_ratio:cert.seq_ratio}).expect("native A4 solve");
    doc["side_a4"]=json!({"status":a4.status,"buckling_eigenvalue":a4.buckling_eigenvalue,
        "buckling_critical_load_n":a4.buckling_critical_load_n,"supported_seq_ratio":a4.supported_seq_ratio,
        "certified_seq_ratio":a4.certified_seq_ratio,"supported_vs_certified_seq_delta":a4.supported_vs_certified_seq_delta,
        "supported_blade_peak_mpa_per_n":a4.supported_blade_peak_mpa_per_n,
        "supported_hand_peak_mpa_per_n":a4.supported_hand_peak_mpa_per_n,"n_bore_nodes":a4.n_bore_nodes,
        "n_grip_nodes":a4.n_grip_nodes,"mesh_sha256":a4.d1_sha256,"mesh_max_area_mm2":a4.mesh_max_area_mm2,
        "n_node":a4.n_node,"n_elem":a4.n_elem,"seconds":start.elapsed().as_secs_f64(),
        "support_constants":{"radial":side_a4::BORE_RADIAL_K,"tangential":side_a4::BORE_TANGENTIAL_K,
            "flange":side_a4::BORE_FLANGE_K,"foundation":side_a4::GRIP_FOUNDATION_K},
        "changes_production_sequence":false,"partial_solver_edit":false});
    fs::write(out.join("plate.json"),serde_json::to_vec_pretty(&doc).unwrap()).unwrap();
    eprintln!("{label} GCI, certification and A4 complete");
}
