//! Assigned production Q7 offspring; native plate fields and actual exported skin.
use jarambit_crescent as c;
use c::genome::{self,*};
use jarambit_genotype::Rng;
use serde_json::json;
use std::{fs,path::Path};
include!("phase15_sources/offspring_extracted.rs");
fn hash(b:&[u8])->String {jarambit_evidence::identity::ContentRef::from_bytes("phase15","rev1",b).sha256}
fn main() {
 let a:Vec<_>=std::env::args().collect();let out=Path::new("target/G16-06-RSV2-08/phase15");let inputs=Path::new("experiments/G16-06-RSV2-08/phase15_candidates");
 assert_eq!(std::env::var("CARGO_BUILD_JOBS").as_deref(),Ok("2"));assert_eq!(jarambit_physics::fine::solver_backend(),"faer");
 let nice:i32=fs::read_to_string("/proc/self/stat").unwrap().rsplit(')').next().unwrap().split_whitespace().nth(16).unwrap().parse().unwrap();assert!(nice>=10);
 let affinity=fs::read_to_string("/proc/self/status").unwrap().lines().find(|l|l.starts_with("Cpus_allowed_list:")).unwrap().to_string();assert_eq!(affinity.split(':').nth(1).unwrap().trim(),"12-13");
 if a[1]=="--generate" {
  fs::create_dir(inputs).unwrap();let parent=c::Genome::c4_seed(6.0);let batch=spawn_offspring_batch(&[("c4".into(),parent.clone())],12,1,&mut Rng(1));assert_eq!(batch.len(),12);
  let mut rows=vec![];
  for slot in [8,10,11] {
   let g=&batch[slot].1;let name=format!("q7_slot{slot}_seed1");let path=inputs.join(format!("{name}.csv"));assert!(!path.exists());
   fs::write(&path,format!("{}\n{}\n",Genome::csv_header(),g.csv_row())).unwrap();
   rows.push(json!({"name":name,"slot":slot,"arm":q7_offspring_arm_label(slot),"rng_seed":1,"round":1,"batch_size":12,"genome":path,"genome_sha256":hash(&fs::read(&path).unwrap()),"genes":g.v,"parent_genes":parent.v,"q7_baseline_cp":Q7_CPS.to_vec(),"dn36_38_including_jitter":[g.v[36]-Q7_CPS[36],g.v[37]-Q7_CPS[37],g.v[38]-Q7_CPS[38]],"material":g.mat.name()}));
  }
  fs::write(out.join("candidates.json"),serde_json::to_string_pretty(&rows).unwrap()).unwrap();return;
 }
 let slot:usize=a[1].parse().unwrap();let name=format!("q7_slot{slot}_seed1");let path=inputs.join(format!("{name}.csv"));let text=fs::read_to_string(&path).unwrap();
 let g=text.lines().find_map(|l|Genome::from_csv(&l.split(',').collect::<Vec<_>>())).unwrap();let mut canonical=g.clone();canonical.clamp();assert_eq!(canonical,g);assert!(g.rs_v2());
 let p=c::build(&g);let aes=c::aesthetic::evaluate(&g,&p);let mut levels=vec![];
 for area in [0.25,0.125,0.0625] {
  let(f,fields)=c::fem::report_fields(&p,area).expect("native plate solve");let mut bytes=vec![];
  for &(x,y) in &fields.xy {bytes.extend(x.to_le_bytes());bytes.extend(y.to_le_bytes());}for t in &fields.tris {for &i in t {bytes.extend((i as u64).to_le_bytes());}}for &t in &fields.te {bytes.extend(t.to_le_bytes());}
  levels.push(json!({"area_mm2":area,"lat_rob":f.lat_rob,"fat_sf":f.fat_sf,"twist_tip_sf":f.twist_sf,"twist_tipzone_sf":f.twist_tipzone_sf,"seq_ratio":f.seq_ratio,"twist_body_sf":f.twist_body_sf,"nodes":f.nodes,"tris":f.tris,"mesh_thickness_sha256":hash(&bytes)}));
  if area==0.25 {
   let solid=c::export::solid(&p,1.0).unwrap();let mut surface=c::export::surface_json(&solid);surface["tip"]=json!(p.tip);surface["rs_v2"]=json!(p.rs_v2);
   let census=c::export::census(&solid.tris);
   fs::write(out.join(format!("{name}_surface.json")),serde_json::to_vec(&surface).unwrap()).unwrap();
   fs::write(out.join(format!("{name}.step")),c::export::step(&solid,&name)).unwrap();
   let d1=json!({"metrics":{"t_joint":p.t_joint},"mesh":{"xy":fields.xy,"tris":fields.tris,"te":fields.te,"twist":fields.twist},"outline":p.outline,"holes":p.holes,
    "field":{"x0":p.field.x0,"y0":p.field.y0,"g":p.field.g,"nx":p.field.nx,"ny":p.field.ny,"th":p.field.th},"export_surface":surface,"export_census":census});
   fs::write(out.join(format!("{name}_D1.json")),serde_json::to_vec(&d1).unwrap()).unwrap();
   fs::write(out.join(format!("{name}_grind_law.json")),c::export::grind_law_json(&p,&g,&name)).unwrap();
  }
 }
 let doc=json!({"name":name,"slot":slot,"genes":g.v,"genome_sha256":hash(&fs::read(&path).unwrap()),"input_canonical":true,"rs_v2":true,"geometry_rejects":c::gates::check(&g,&p),"aesthetic_pass":aes.pass,"aesthetic_failures":aes.fails,"levels":levels,"nice":nice,"cpus":[12,13],"cargo_build_jobs":2,"solver":"faer","unbiased":true,"native_sources_unmodified":true});
 fs::write(out.join(format!("{name}_plate.json")),serde_json::to_string_pretty(&doc).unwrap()).unwrap();
}
