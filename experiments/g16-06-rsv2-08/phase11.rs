//! Unmodified production side.a4 on the unrounded native .25 mm2 field mesh.
use jarambit_crescent as c;
use serde_json::{json,Value};
use std::{fs,path::Path,time::Instant};
fn hash(bytes:&[u8])->String {jarambit_evidence::identity::ContentRef::from_bytes("phase11","rev1",bytes).sha256}
fn mesh_hash(fields:&c::fem::Fields)->String {
    let mut bytes=Vec::new();
    for &(x,y) in &fields.xy {bytes.extend(x.to_le_bytes());bytes.extend(y.to_le_bytes());}
    for t in &fields.tris {for &i in t {bytes.extend((i as u64).to_le_bytes());}}
    for &t in &fields.te {bytes.extend(t.to_le_bytes());}hash(&bytes)
}
fn main() {
    let args:Vec<_>=std::env::args().collect();let input=Path::new(&args[1]);let output=Path::new(&args[2]);let mesh_path=output.with_extension("mesh.json");
    assert!(!output.exists()&&!mesh_path.exists());
    let bytes=fs::read(input).unwrap();let text=std::str::from_utf8(&bytes).unwrap();
    let g=text.lines().find_map(|l|c::Genome::from_csv(&l.split(',').collect::<Vec<_>>())).unwrap();
    let mut canonical=g.clone();canonical.clamp();assert_eq!(g,canonical);assert!(g.rs_v2());
    assert_eq!(std::env::var("CARGO_BUILD_JOBS").as_deref(),Ok("2"));assert_eq!(jarambit_physics::fine::solver_backend(),"faer");
    let nice:i32=fs::read_to_string("/proc/self/stat").unwrap().rsplit(')').next().unwrap().split_whitespace().nth(16).unwrap().parse().unwrap();assert!(nice>=10);
    let affinity=fs::read_to_string("/proc/self/status").unwrap().lines().find(|l|l.starts_with("Cpus_allowed_list:")).unwrap().to_string();
    assert_eq!(affinity.split(':').nth(1).unwrap().trim(),"12-13");
    let cert:f64=args[4].parse().unwrap();assert!(cert.is_finite()&&cert>0.);
    let start=Instant::now();let p=c::build(&g);let (f,fields)=c::fem::report_fields(&p,0.25).expect("unbiased native .25 mesh");
    let mesh_sha=mesh_hash(&fields);let mesh_bytes=serde_json::to_vec(&json!({"xy":fields.xy,"tris":fields.tris,"te":fields.te})).unwrap();
    fs::write(&mesh_path,&mesh_bytes).unwrap();
    let result=c::battery::run_side_a4_native_for_genome(&args[3],&args[1],&hash(&bytes),None,Some(&fields),&p,cert);
    let doc=json!({"task":"G16-06-RSV2-08-p11","name":args[3],"genome":args[1],"genome_sha256":hash(&bytes),"genes":g.v,
        "input_canonical":true,"rs_v2":true,"solver":"faer","nice":nice,"cargo_build_jobs":2,"cpu_affinity":affinity,
        "production_api":"jarambit_crescent::battery::run_side_a4_native_for_genome","native_fem_unmodified":true,
        "source_provenance":serde_json::from_str::<Value>(include_str!("../../target/g16-06-rsv2-08/phase11/source_provenance.json")).unwrap(),
        "mesh_area_mm2":0.25,"mesh_sha256":mesh_sha,"mesh_json":mesh_path.to_str().unwrap(),"mesh_json_sha256":hash(&mesh_bytes),
        "same_mesh_ring_clamp_seq_ratio":f.seq_ratio,"seq_cert_input":cert,"a4":result,
        "seconds":start.elapsed().as_secs_f64(),"all_gate_qualification":false});
    fs::write(output,serde_json::to_string_pretty(&doc).unwrap()).unwrap();
}
