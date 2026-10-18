#![allow(dead_code,unused_imports)]
include!("modules.rs");
#[path="prototype_export.rs"]pub mod export;
pub mod compound;
pub mod diagnostic;
use genome::*;
use serde_json::{json,Value};
use std::{fs,path::Path,time::Instant};
fn metrics(f:&fem::Fem)->Value {json!({"lat_rob":f.lat_rob,"ring_rob":f.ring_rob,"twist_sf":f.twist_sf,"twist_body_sf":f.twist_body_sf,"twist_tipzone_sf":f.twist_tipzone_sf,"seq_ratio":f.seq_ratio,"seq_case":f.seq_case,"fat_sf":f.fat_sf,"hm_twist_sf":f.hm_twist_sf,"hm_spine_sf":f.hm_spine_sf,"hm_ringback_sf":f.hm_ringback_sf,"tw_joint_nmm":f.twist_joint_nmm,"hm_spine_n":f.hm_spine_n,"hm_ringback_n":f.hm_ringback_n,"twist_b_mm":f.twist_b_mm,"twist_t_mm":f.twist_t_mm,"twist_kc":f.twist_kc,"twist_at":f.twist_at,"twist_tipdist":f.twist_tipdist,"lat_at":f.lat_at,"nodes":f.nodes,"tris":f.tris})}
fn worst_face(s:&export::Solid,p:&pheno::Pheno)->Value {
    let mut worst=0f64;let mut result=Value::Null;
    for tr in &s.tris {
        let [a,b,c]=tr.map(|i|s.verts[i]);let u=(b.0-a.0,b.1-a.1,b.2-a.2);let v=(c.0-a.0,c.1-a.1,c.2-a.2);
        let nx=u.1*v.2-u.2*v.1;let ny=u.2*v.0-u.0*v.2;let nz=u.0*v.1-u.1*v.0;if nz.abs()<1e-14{continue;}
        let angle=(nx.hypot(ny)/nz.abs()).atan().to_degrees();if angle<=worst{continue;}
        let aa=(a.0,a.1);let bb=(b.0,b.1);let cc=(c.0,c.1);
        if !p.cut_in.within_triangle(aa,bb,cc,0.4)&&!p.cut_sp.within_triangle(aa,bb,cc,0.4){continue;}
        worst=angle;let distances=[aa,bb,cc].map(|q|p.cut_in.nearest(q,10.).min(p.cut_sp.nearest(q,10.)));
        let tip_distances=[aa,bb,cc].map(|q|geom::dist(q,p.tip));
        result=json!({"angle":angle,"triangle":[a,b,c],"edge_distances":distances,"tip_distances":tip_distances});
    }result
}
fn main(){
    let args:Vec<_>=std::env::args().collect();let mode=&args[1];let out=Path::new(&args[2]);
    let txt=fs::read_to_string(&args[3]).unwrap();let mut g=txt.lines().find_map(|l|Genome::from_csv(&l.split(',').collect::<Vec<_>>())).unwrap();g.clamp();
    if let Ok(s)=std::env::var("STUDY_STOCK") {let t:f64=s.parse().unwrap();g.v[T]=t;for k in 0..5 {g.v[K0+k]=t;}}
    let mut reclamped=g.clone();reclamped.clamp();let domain_ok=g.v==reclamped.v;
    let start=Instant::now();let p=pheno::build(&g);let aes=aesthetic::evaluate(&g,&p);let rejects=gates::check(&g,&p);
    let mut doc=json!({"prototype_only":true,"source":args[3],"genes":g.v,"within_genome_domain":domain_ok,"config":compound::config(),"rs_v2":g.rs_v2(),"aes_pass":aes.pass,"aes_fails":aes.fails,"rejects":rejects,"native_field_mass_g":p.mass_g,"t_joint":p.t_joint});
    doc["point_cap_probe"]=json!([0.,0.25,1.,2.,3.2,5.2,12.].map(|a|(a,compound::point_cap(a,p.t_joint))));
    if mode=="all"||mode=="export" {
        let measure=certification::measure(&p);
        let solid=export::solid(&p,1.);
        doc["measurement"]=json!(measure);
        doc["solid"]=solid.as_ref().map(|s|json!({"census":export::census(&s.tris),"volume_mm3":export::volume(s),"vertices":s.verts.len(),"triangles":s.tris.len()})).unwrap_or(Value::Null);
        doc["worst_edge_face"]=solid.as_ref().map(|s|worst_face(s,&p)).unwrap_or(Value::Null);
        let u=compound::axis(&p);
        doc["axis_samples"]=json!((0..=200).map(|i|{let a=i as f64*0.05;json!([a,export::thickness_at(&p,geom::add(p.tip,geom::mul(u,a)))])}).collect::<Vec<_>>());
        eprintln!("export {}",doc["measurement"]);
    }
    if mode=="all"||mode=="fem"||mode=="coarse" {
        let areas=if let Ok(s)=std::env::var("STUDY_AREA") {vec![s.parse().unwrap()]}else if mode=="coarse"{vec![2.]}else{vec![2.,0.25,0.125,0.0625]};
        let mut levels=vec![];
        for area in areas {
            let time=Instant::now();let f=fem::run_unbiased(&p,area);
            let v=json!({"area":area,"unbiased":true,"metrics":f.as_ref().map(metrics),"diagnostic_peaks":diagnostic::take(),"seconds":time.elapsed().as_secs_f64()});
            eprintln!("mesh {area} {}",v["metrics"]);levels.push(v);
            doc["levels"]=json!(levels);fs::write(out,serde_json::to_string_pretty(&doc).unwrap()).unwrap();
        }
    }
    doc["seconds"]=json!(start.elapsed().as_secs_f64());fs::write(out,serde_json::to_string_pretty(&doc).unwrap()).unwrap();
}
