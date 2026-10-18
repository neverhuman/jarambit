#![allow(dead_code,unused_imports)]
include!("modules.rs");
#[path="prototype_export.rs"]pub mod export;
pub mod compound;
pub mod diagnostic;
use genome::*;
use serde_json::{json,Value};
use std::{fs,path::Path,time::Instant};
fn metrics(f:&fem::Fem)->Value {json!({"sf_hunter":f.sf_hunter,"lat_rob":f.lat_rob,"ring_rob":f.ring_rob,"twist_sf":f.twist_sf,"twist_body_sf":f.twist_body_sf,"twist_tipzone_sf":f.twist_tipzone_sf,"seq_ratio":f.seq_ratio,"seq_case":f.seq_case,"fat_sf":f.fat_sf,"hm_twist_sf":f.hm_twist_sf,"hm_spine_sf":f.hm_spine_sf,"hm_ringback_sf":f.hm_ringback_sf,"tw_joint_nmm":f.twist_joint_nmm,"hm_spine_n":f.hm_spine_n,"hm_ringback_n":f.hm_ringback_n,"twist_b_mm":f.twist_b_mm,"twist_t_mm":f.twist_t_mm,"twist_kc":f.twist_kc,"twist_at":f.twist_at,"twist_tipdist":f.twist_tipdist,"lat_at":f.lat_at,"nodes":f.nodes,"tris":f.tris})}
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
    let txt=fs::read_to_string(&args[3]).unwrap();let mut g=txt.lines().find_map(|l|Genome::from_csv(&l.split(',').collect::<Vec<_>>())).unwrap();let input=g.clone();g.clamp();
    if let Ok(s)=std::env::var("STUDY_STOCK") {let t:f64=s.parse().unwrap();g.v[T]=t;for k in 0..5 {g.v[K0+k]=t;}}
    let mut reclamped=g.clone();reclamped.clamp();let domain_ok=g.v==reclamped.v;
    if compound::config().neck_full {
        assert!((5.3..=g.v[T]).contains(&compound::config().blade_core));
        assert!(compound::config().heel_runout>0.);
    }
    let start=Instant::now();let p=pheno::build(&g);let aes=aesthetic::evaluate(&g,&p);let rejects=gates::check(&g,&p);
    assert_eq!(std::env::var("CARGO_BUILD_JOBS").as_deref(),Ok("2"));
    let mut doc=json!({"input_was_canonical":input.v==g.v,"solver":std::env::var("JARAMBIT_SOLVER").unwrap_or_else(|_|"skyline".into()),"prototype_only":true,"cpu_affinity":fs::read_to_string("/proc/self/status").unwrap().lines().find(|l|l.starts_with("Cpus_allowed_list:")).unwrap(),"cargo_build_jobs":std::env::var("CARGO_BUILD_JOBS").ok(),"window_count":p.n_win,"window_area_mm2":p.win_area,"source":args[3],"genes":g.v,"within_genome_domain":domain_ok,"config":compound::config(),"rs_v2":g.rs_v2(),"aes_pass":aes.pass,"aes_fails":aes.fails,"rejects":rejects,"hole_x_bounds":p.holes.iter().map(|h|[h.iter().map(|q|q.0).fold(f64::INFINITY,f64::min),h.iter().map(|q|q.0).fold(f64::NEG_INFINITY,f64::max)]).collect::<Vec<_>>(),"holes":p.hole_names,"heel":p.heel,"tip":p.tip,"mass_g":p.mass_g,"t_joint":p.t_joint});
    doc["provenance"]=serde_json::from_str(include_str!("source_provenance.json")).unwrap();
    fs::write(out.with_extension("canonical.csv"),format!("{}\n{}\n",Genome::csv_header(),g.csv_row())).unwrap();
    let mut section=Vec::new();
    for (i,&v) in p.field.th.iter().enumerate() {
        if !v.is_finite(){continue;}
        let q=(p.field.x0+(i%p.field.nx) as f64*p.field.g+p.field.g/2.,p.field.y0+(i/p.field.nx) as f64*p.field.g+p.field.g/2.);
        if q.0>=p.ring_od/2.+3. && q.0<p.heel.0 && p.cut_in.nearest(q,2.).min(p.cut_sp.nearest(q,2.))>=compound::config().d1 {
            section.push(export::thickness_at(&p,q));
        }
    }
    doc["neck_section"]=json!({"samples":section.len(),"min_mm":section.iter().copied().fold(f64::INFINITY,f64::min),"max_mm":section.iter().copied().fold(f64::NEG_INFINITY,f64::max),"scope":"material beyond ring transition, before heel, outside edge band"});
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
        let areas=std::env::var("STUDY_AREAS").ok().map(|s|s.split(',').map(|x|x.parse::<f64>().unwrap()).collect::<Vec<_>>()).unwrap_or_else(||if mode=="coarse"{vec![2.]}else{vec![2.,0.25,0.125,0.0625]});
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
