//! Diagnostic-only measurements through the unmodified production crescent library.
use jarambit_crescent as c;
use c::geom::*;
use serde_json::{json,Value};
use std::{fs,path::Path,time::Instant};
fn hash(b:&[u8])->String {jarambit_evidence::identity::ContentRef::from_bytes("phase9","rev1",b).sha256}
fn axis(p:&c::Pheno)->P2 {
    let qi=p.edge_in.iter().copied().find(|&q|dist(q,p.tip)>=2.).unwrap_or(p.heel);
    let qs=p.edge_sp.iter().rev().copied().find(|&q|dist(q,p.tip)>=2.).unwrap_or(p.heel);
    norm(add(norm(sub(qi,p.tip)),norm(sub(qs,p.tip))))
}
fn intervals(p:&c::Pheno,s:f64,u:P2,n:P2)->Vec<(f64,f64)> {
    let origin=add(p.tip,mul(u,s));let vmax=(225.-s*s).sqrt();let mut breaks=vec![-vmax,vmax];
    for ring in std::iter::once(&p.outline).chain(p.holes.iter()) {
        for i in 0..ring.len() {
            let a=sub(ring[i],origin);let b=sub(ring[(i+1)%ring.len()],origin);
            let (a_u,b_u)=(a.0*u.0+a.1*u.1,b.0*u.0+b.1*u.1);
            if (a_u>0.)!=(b_u>0.) {
                let t=-a_u/(b_u-a_u);let v=(a.0+t*(b.0-a.0))*n.0+(a.1+t*(b.1-a.1))*n.1;
                if v>-vmax && v<vmax {breaks.push(v);}
            }
        }
    }
    // Split at native raster-cell boundaries as well as exact polygon intersections.
    for (o,d,start,count) in [(origin.0,n.0,p.field.x0,p.field.nx),(origin.1,n.1,p.field.y0,p.field.ny)] {
        if d.abs()>1e-12 {for k in 0..=count {let v=(start+k as f64*p.field.g-o)/d;if v>-vmax && v<vmax {breaks.push(v);}}}
    }
    breaks.sort_by(f64::total_cmp);breaks.dedup_by(|a,b|(*a-*b).abs()<1e-12);
    breaks.windows(2).filter_map(|w| {
        let q=add(origin,mul(n,(w[0]+w[1])/2.));
        (point_in_poly(q,&p.outline)&&!p.holes.iter().any(|h|point_in_poly(q,h))).then_some((w[0],w[1]))
    }).collect()
}
fn section(p:&c::Pheno,s:f64,u:P2,n:P2,h:f64,spans:&[(f64,f64)])->Value {
    let origin=add(p.tip,mul(u,s));let(mut area,mut first,mut second,mut iz,mut tmax,mut tmin,mut width,mut samples)=(0.,0.,0.,0.,0f64,f64::INFINITY,0.,0usize);
    for &(a,b) in spans {
        let count=((b-a)/h).ceil().max(1.) as usize;let dv=(b-a)/count as f64;width+=b-a;
        for k in 0..count {
            let v=a+(k as f64+0.5)*dv;let q=add(origin,mul(n,v));let r=dist(q,p.tip);
            assert!(r>=4.-1e-12 && r<15.);
            let t=c::export::thickness_at(p,q);assert!(t.is_finite()&&t>0.);
            area+=t*dv;first+=v*t*dv;second+=v*v*t*dv;iz+=t*t*t*dv/12.;tmax=tmax.max(t);tmin=tmin.min(t);samples+=1;
        }
    }
    assert!(area>0.);let centre=first/area;let iy=(second-first*first/area).max(0.);
    let extreme=spans.iter().flat_map(|&(a,b)|[(a-centre).abs(),(b-centre).abs()]).fold(0f64,f64::max);
    json!({"step_max_mm":h,"samples":samples,"width_mm":width,"area_mm2":area,"centroid_v_mm":centre,
        "I_out_of_plane_mm4":iz,"I_in_plane_mm4":iy,"Z_out_of_plane_mm3":2.*iz/tmax,"Z_in_plane_mm3":iy/extreme,
        "min_thickness_mm":tmin,"max_thickness_mm":tmax})
}
fn metrics(f:&c::fem::Fem)->Value {json!({"lat_rob_n":f.lat_rob,"lateral_sf_358":f.lat_rob/358.,"native_hiker_sf":f.sf_hiker,"native_hunter_sf":f.sf_hunter,
    "fatigue_sf":f.fat_sf,"sequence_ratio":f.seq_ratio,"hm_spine_sf":f.hm_spine_sf,"hm_twist_sf":f.hm_twist_sf,"hm_ringback_sf":f.hm_ringback_sf,
    "twist_body_sf":f.twist_body_sf,"twist_tipzone_sf":f.twist_tipzone_sf,"nodes":f.nodes,"tris":f.tris})}
fn main(){
    let args:Vec<_>=std::env::args().collect();let input=Path::new(&args[1]);let output=Path::new(&args[2]);
    let bytes=fs::read(input).unwrap();let text=std::str::from_utf8(&bytes).unwrap();
    let g=text.lines().find_map(|l|c::Genome::from_csv(&l.split(',').collect::<Vec<_>>())).unwrap();
    let mut canonical=g.clone();canonical.clamp();assert_eq!(g,canonical,"never accept clamped candidate");assert!(g.rs_v2());
    assert_eq!(std::env::var("CARGO_BUILD_JOBS").as_deref(),Ok("2"));assert_eq!(jarambit_physics::fine::solver_backend(),"faer");
    let nice:i32=fs::read_to_string("/proc/self/stat").unwrap().rsplit(')').next().unwrap().split_whitespace().nth(16).unwrap().parse().unwrap();assert!(nice>=10);
    let start=Instant::now();let p=c::build(&g);let u=axis(&p);let n=(-u.1,u.0);let aes=c::aesthetic::evaluate(&g,&p);
    let mut doc=json!({"task":"G16-06-RSV2-08-p9","genome_sha256":hash(&bytes),"genes":g.v,"input_canonical":true,"rs_v2":p.rs_v2,
        "geometry_rejects":c::gates::check(&g,&p),"aesthetic_pass":aes.pass,"aesthetic_failures":aes.fails,"tip":p.tip,"axis":u,"section_normal":n,
        "production_thickness_law":true,"native_fem_unmodified":true,"solver":"faer","nice":nice,"cargo_build_jobs":2,
        "cpu_affinity":fs::read_to_string("/proc/self/status").unwrap().lines().find(|l|l.starts_with("Cpus_allowed_list:")).unwrap(),
        "certificate_identity":c::certification::identity(&g),"source_provenance":serde_json::from_str::<Value>(include_str!("../../target/g16-06-rsv2-08/phase9/source_provenance.json")).unwrap(),
        "all_gate_qualification":false});
    let mut sections=Vec::new();
    for k in 0..44 {let s=4.+k as f64*0.25;let spans=intervals(&p,s,u,n);assert!(!spans.is_empty());
        let estimates:Vec<_>=[0.02,0.01,0.005].into_iter().map(|h|section(&p,s,u,n,h,&spans)).collect();
        sections.push(json!({"s_mm":s,"radial_annulus_mm":[4.,15.],"integration":"midpoint, split at polygon and native raster boundaries","estimates":estimates}));}
    doc["sections"]=json!(sections);
    let point:Vec<_>=[(0.25,c::fem::gate_s::TIP_T025_MAX_MM),(1.,c::fem::gate_s::TIP_T1_MAX_MM),(2.,c::fem::gate_s::TIP_T2_MAX_MM)].into_iter().map(|(s,limit)| {
        let t=c::export::thickness_at(&p,add(p.tip,mul(u,s)));json!({"s_mm":s,"thickness_mm":t,"limit_mm":limit,"margin_mm":limit-t})}).collect();
    doc["exact_law_point_samples"]=json!(point);
    let m=c::certification::measure(&p);
    doc["gate_s"]=json!({"measurement":m,"apex_limit_mm":c::fem::gate_s::APEX_MAX_MM,"half_angle_limit_deg":c::fem::gate_s::HALF_ANGLE_MAX_DEG,
        "point_limits_mm":[c::fem::gate_s::TIP_T025_MAX_MM,c::fem::gate_s::TIP_T1_MAX_MM,c::fem::gate_s::TIP_T2_MAX_MM],"point_taper_min_mm":c::fem::gate_s::TIP_TAPER_MIN_MM});
    fs::write(output,serde_json::to_string_pretty(&doc).unwrap()).unwrap();
    let mut levels=Vec::new();
    for area in [0.25,0.125,0.0625] {
        let time=Instant::now();let result=c::fem::report_fields(&p,area);
        let entry=result.map(|(f,fields)|{
            let mut b=Vec::new();for (x,y) in fields.xy {b.extend(x.to_le_bytes());b.extend(y.to_le_bytes());}for t in fields.tris {for i in t {b.extend((i as u64).to_le_bytes());}}for t in fields.te {b.extend(t.to_le_bytes());}
            json!({"area_mm2":area,"metrics":metrics(&f),"mesh_thickness_sha256":hash(&b),"seconds":time.elapsed().as_secs_f64(),"unbiased":true})
        });
        levels.push(json!({"area_mm2":area,"result":entry}));doc["levels"]=json!(levels);fs::write(output,serde_json::to_string_pretty(&doc).unwrap()).unwrap();
    }
    doc["seconds"]=json!(start.elapsed().as_secs_f64());fs::write(output,serde_json::to_string_pretty(&doc).unwrap()).unwrap();
}
