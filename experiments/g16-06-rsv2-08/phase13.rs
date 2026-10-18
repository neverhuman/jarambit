//! Exact native genotype coordinates and unmodified production FEM/export thickness.
use jarambit_crescent as c;
use serde_json::{json,Value};
use std::{fs,path::Path,time::Instant};
fn hash(bytes:&[u8])->String {jarambit_evidence::identity::ContentRef::from_bytes("phase13","rev1",bytes).sha256}
fn genome(path:&Path)->c::Genome {
    let text=fs::read_to_string(path).unwrap();
    let g=text.lines().find_map(|l|c::Genome::from_csv(&l.split(',').collect::<Vec<_>>())).unwrap();
    let mut canonical=g.clone();canonical.clamp();assert_eq!(g,canonical,"clamped inputs forbidden");assert!(g.rs_v2());g
}
fn metrics(f:&c::fem::Fem)->Value {json!({"lat_rob_n":f.lat_rob,"lateral_sf_358":f.lat_rob/358.,"native_hunter_sf":f.sf_hunter,"fat_sf":f.fat_sf,
    "seq_ratio":f.seq_ratio,"twist_tip_sf":f.twist_sf,"twist_tipzone_sf":f.twist_tipzone_sf,"twist_body_sf":f.twist_body_sf,
    "twist_joint_nmm":f.twist_joint_nmm,"hm_twist_sf":f.hm_twist_sf,"hm_spine_sf":f.hm_spine_sf,"hm_ringback_sf":f.hm_ringback_sf,
    "nodes":f.nodes,"tris":f.tris})}
fn main() {
    let args:Vec<_>=std::env::args().collect();let input=Path::new(&args[1]);let output=Path::new(&args[2]);let parent_path=Path::new(&args[3]);assert!(!output.exists());
    let g=genome(input);let parent=genome(parent_path);
    assert_eq!(std::env::var("CARGO_BUILD_JOBS").as_deref(),Ok("2"));assert_eq!(jarambit_physics::fine::solver_backend(),"faer");
    let nice:i32=fs::read_to_string("/proc/self/stat").unwrap().rsplit(')').next().unwrap().split_whitespace().nth(16).unwrap().parse().unwrap();assert!(nice>=10);
    let affinity=fs::read_to_string("/proc/self/status").unwrap().lines().find(|l|l.starts_with("Cpus_allowed_list:")).unwrap().to_string();assert_eq!(affinity.split(':').nth(1).unwrap().trim(),"12-13");
    let start=Instant::now();let p=c::build(&g);let aes=c::aesthetic::evaluate(&g,&p);
    let mut doc=json!({"task":"G16-06-RSV2-08-p13","name":input.file_stem().unwrap().to_str().unwrap(),"genome":args[1],"genome_sha256":hash(&fs::read(input).unwrap()),"genes":g.v,
        "parent_genome":args[3],"parent_genome_sha256":hash(&fs::read(parent_path).unwrap()),"input_canonical":true,"rs_v2":p.rs_v2,
        "control_points":c::pheno::control_points(&g),"parent_control_points":c::pheno::control_points(&parent),
        "geometry_rejects":c::gates::check(&g,&p),"aesthetic_pass":aes.pass,"aesthetic_failures":aes.fails,
        "solver":"faer","nice":nice,"cargo_build_jobs":2,"cpu_affinity":affinity,"native_fem_unmodified":true,"production_thickness_law":true,
        "certificate_identity":c::certification::identity(&g),
        "source_provenance":serde_json::from_str::<Value>(include_str!("../../target/g16-06-rsv2-08/phase13/source_provenance.json")).unwrap(),
        "all_gate_qualification":false});
    let mut levels=Vec::new();
    for area in [0.25,0.125,0.0625] {
        let time=Instant::now();let result=c::fem::report_fields(&p,area).map(|(f,fields)| {
            let mut bytes=Vec::new();for (x,y) in fields.xy {bytes.extend(x.to_le_bytes());bytes.extend(y.to_le_bytes());}
            for t in fields.tris {for i in t {bytes.extend((i as u64).to_le_bytes());}}for t in fields.te {bytes.extend(t.to_le_bytes());}
            json!({"area_mm2":area,"metrics":metrics(&f),"mesh_thickness_sha256":hash(&bytes),"unbiased":true,"seconds":time.elapsed().as_secs_f64()})
        });
        levels.push(json!({"area_mm2":area,"result":result}));doc["levels"]=json!(levels);fs::write(output,serde_json::to_string_pretty(&doc).unwrap()).unwrap();
    }
    doc["seconds"]=json!(start.elapsed().as_secs_f64());fs::write(output,serde_json::to_string_pretty(&doc).unwrap()).unwrap();
}
