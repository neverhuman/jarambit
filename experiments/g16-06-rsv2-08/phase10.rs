//! Native production three-level plate GCI; no study geometry or solver hooks.
use jarambit_crescent as c;
use serde_json::{json,Value};
use std::{fs,path::Path,time::Instant};
fn hash(bytes:&[u8])->String {jarambit_evidence::identity::ContentRef::from_bytes("phase10","rev1",bytes).sha256}
fn main() {
    let args:Vec<_>=std::env::args().collect();let input=Path::new(&args[1]);let output=Path::new(&args[2]);
    assert!(!output.exists());
    let bytes=fs::read(input).unwrap();let text=std::str::from_utf8(&bytes).unwrap();
    let g=text.lines().find_map(|l|c::Genome::from_csv(&l.split(',').collect::<Vec<_>>())).unwrap();
    let mut canonical=g.clone();canonical.clamp();assert_eq!(g,canonical);assert!(g.rs_v2());
    assert_eq!(std::env::var("CARGO_BUILD_JOBS").as_deref(),Ok("2"));
    assert_eq!(jarambit_physics::fine::solver_backend(),"faer");
    let nice:i32=fs::read_to_string("/proc/self/stat").unwrap().rsplit(')').next().unwrap().split_whitespace().nth(16).unwrap().parse().unwrap();assert!(nice>=10);
    let affinity=fs::read_to_string("/proc/self/status").unwrap().lines().find(|l|l.starts_with("Cpus_allowed_list:")).unwrap().to_string();
    assert_eq!(affinity.split(':').nth(1).unwrap().trim(),"12-13");
    let start=Instant::now();
    let result=c::gci::run_plate_gci_3level(&args[3],&args[1],&hash(&bytes),&g);
    let doc=json!({"task":"G16-06-RSV2-08-p10","name":args[3],"genome":args[1],"genome_sha256":hash(&bytes),"genes":g.v,
        "input_canonical":true,"rs_v2":true,"solver":"faer","nice":nice,"cargo_build_jobs":2,"cpu_affinity":affinity,
        "production_api":"jarambit_crescent::gci::run_plate_gci_3level","native_fem_unmodified":true,
        "source_provenance":serde_json::from_str::<Value>(include_str!("../../target/g16-06-rsv2-08/phase10/source_provenance.json")).unwrap(),
        "plate_gci":result,"seconds":start.elapsed().as_secs_f64(),"all_gate_qualification":false});
    fs::write(output,serde_json::to_string_pretty(&doc).unwrap()).unwrap();
}
