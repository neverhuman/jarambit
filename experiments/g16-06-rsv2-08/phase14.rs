//! Native seed, assigned coordinate grid and unmodified production FEM/GCI.
use jarambit_crescent as c;
use serde_json::{json,Value};
use std::{fs,path::Path,time::Instant};
fn hash(b:&[u8])->String {jarambit_evidence::identity::ContentRef::from_bytes("phase14","rev1",b).sha256}
fn genome(path:&Path)->c::Genome {
    let text=fs::read_to_string(path).unwrap();let g=text.lines().find_map(|l|c::Genome::from_csv(&l.split(',').collect::<Vec<_>>())).unwrap();
    let mut canonical=g.clone();canonical.clamp();assert_eq!(g,canonical,"offspring clamping forbidden");assert!(g.rs_v2());g
}
fn metrics(f:&c::fem::Fem)->Value {json!({"F_lat":f.lat_rob,"F_ax":f.hm_tip_inplane_n,"SF_fat":f.fat_sf,"twist_joint_nmm":f.twist_joint_nmm,
    "twist_sf":f.twist_sf,"twist_body_sf":f.twist_body_sf,"twist_tipzone_sf":f.twist_tipzone_sf,"hm_twist_sf":f.hm_twist_sf,
    "seq_ratio":f.seq_ratio,"native_hunter_sf":f.sf_hunter,"hm_tip_inplane_sf":f.hm_tip_inplane_sf,"nodes":f.nodes,"tris":f.tris})}
fn main() {
    let a:Vec<_>=std::env::args().collect();assert_eq!(std::env::var("CARGO_BUILD_JOBS").as_deref(),Ok("2"));
    assert_eq!(jarambit_physics::fine::solver_backend(),"faer");
    let nice:i32=fs::read_to_string("/proc/self/stat").unwrap().rsplit(')').next().unwrap().split_whitespace().nth(16).unwrap().parse().unwrap();assert!(nice>=10);
    let affinity=fs::read_to_string("/proc/self/status").unwrap().lines().find(|l|l.starts_with("Cpus_allowed_list:")).unwrap().to_string();assert_eq!(affinity.split(':').nth(1).unwrap().trim(),"12-13");
    if a[1]=="--seed" {
        let dir=Path::new(&a[2]);let original=c::Genome::c4_seed(6.0);let mut g=original.clone();
        for (i,v) in [(89,0.192),(47,148.0),(46,4.4),(42,6.20),(117,1.0),(120,0.25)] {g.v[i]=v;}
        let before=g.v.clone();g.clamp();
        for (name,seed) in [("c4_seed",&original),("fixed_seed",&g)] {let p=dir.join(format!("{name}.csv"));assert!(!p.exists());fs::write(p,format!("{}\n{}\n",c::Genome::csv_header(),seed.csv_row())).unwrap();}
        fs::write(dir.join("seed_provenance.json"),serde_json::to_string_pretty(&json!({"api":"Genome::c4_seed(6.0)","original_genes":original.v,"before_p1_clamp_genes":before,"fixed_genes":g.v,"material":g.mat.name(),"fixed_control_points":c::pheno::control_points(&g)})).unwrap()).unwrap();return;
    }
    let input=Path::new(&a[1]);let output=Path::new(&a[2]);assert!(!output.exists());let g=genome(input);let parent=genome(Path::new(&a[3]));
    let start=Instant::now();let p=c::build(&g);let aes=c::aesthetic::evaluate(&g,&p);let name=input.file_stem().unwrap().to_str().unwrap();
    let mut doc=json!({"task":"G16-06-RSV2-08-p14","name":name,"genome":a[1],"genome_sha256":hash(&fs::read(input).unwrap()),"genes":g.v,
        "input_canonical":true,"rs_v2":p.rs_v2,"control_points":c::pheno::control_points(&g),"parent_control_points":c::pheno::control_points(&parent),
        "geometry_rejects":c::gates::check(&g,&p),"aesthetic_pass":aes.pass,"aesthetic_failures":aes.fails,
        "native_gate_s_law_pass":c::fem::gate_s::law_pass(g.v[89],g.v[48],g.v[49]),
        "solver":"faer","nice":nice,"cargo_build_jobs":2,"cpu_affinity":affinity,"native_fem_unmodified":true,"production_thickness_law":true,
        "certificate_identity":c::certification::identity(&g),"all_gate_qualification":false,
        "source_provenance":serde_json::from_str::<Value>(include_str!("../../target/g16-06-rsv2-08/phase14/source_provenance.json")).unwrap()});
    let mut levels=Vec::new();
    for area in [4.0,1.0,0.25] {
        let time=Instant::now();let result=c::fem::report_fields(&p,area).map(|(f,fields)| {
            let mut bytes=Vec::new();for (x,y) in fields.xy {bytes.extend(x.to_le_bytes());bytes.extend(y.to_le_bytes());}
            for t in fields.tris {for i in t {bytes.extend((i as u64).to_le_bytes());}}for t in fields.te {bytes.extend(t.to_le_bytes());}
            json!({"area_mm2":area,"metrics":metrics(&f),"mesh_thickness_sha256":hash(&bytes),"unbiased":true,"seconds":time.elapsed().as_secs_f64()})
        });levels.push(json!({"area_mm2":area,"result":result}));doc["levels"]=json!(levels);fs::write(output,serde_json::to_string_pretty(&doc).unwrap()).unwrap();
    }
    if levels.iter().all(|l| !l["result"].is_null()) {
        for (key,metric,units,projection) in [("tip_gci","F_lat","N","plate_dkt"),("torsional_gci","twist_joint_nmm","Nmm","plate_dkt_with_native_section_correction")] {
            let obs=c::gci::Observable{id:format!("phase14_{metric}"),units:units.into(),region:"whole_plate_outside_native_4mm_tip_exclusion".into(),projection:projection.into(),metric:metric.into()};
            let bound:Vec<_>=levels.iter().rev().map(|l|c::gci::Level::new(c::gci::MeshRef{id:format!("{name}_area_{}",l["area_mm2"].as_f64().unwrap()),revision:"rev1".into(),sha256:l["result"]["mesh_thickness_sha256"].as_str().unwrap().into()},l["area_mm2"].as_f64().unwrap().sqrt(),obs.clone())).collect();
            let values:Vec<_>=levels.iter().rev().map(|l|l["result"]["metrics"][metric].as_f64().unwrap()).collect();
            doc[key]=c::gci::compute_gci(values[0],values[1],values[2],2.0,1.25,Some(&bound)).to_json();
        }
    }
    doc["seconds"]=json!(start.elapsed().as_secs_f64());fs::write(output,serde_json::to_string_pretty(&doc).unwrap()).unwrap();
}
