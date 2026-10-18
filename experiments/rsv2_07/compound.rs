use crate::{geom::*,pheno::*,genome::*};
use std::sync::OnceLock;
#[derive(Clone,Debug,serde::Serialize)]
pub struct Config {pub baseline:bool,pub angle:f64,pub apex:f64,pub d0:f64,pub d1:f64,pub point_slope:f64,pub point_end:f64,pub core:f64,pub boundary_tol:f64,pub export_area:f64,pub grid:f64,pub point_shape:usize,pub radial:bool,pub smooth_heel:bool,pub point_lateral:f64}
pub fn config()->&'static Config {
    static C:OnceLock<Config>=OnceLock::new();C.get_or_init(||{
        let e=|k:&str,d:f64|std::env::var(k).ok().map(|s|s.parse().unwrap()).unwrap_or(d);
        Config {baseline:std::env::var("STUDY_BASELINE").as_deref()==Ok("1"),angle:e("STUDY_ANGLE",14.),apex:e("STUDY_APEX",0.08),d0:e("STUDY_D0",1.1),d1:e("STUDY_D1",1.6),point_slope:e("STUDY_POINT_SLOPE",0.3),point_end:e("STUDY_POINT_END",6.),core:e("STUDY_CORE",0.),boundary_tol:e("STUDY_BOUNDARY_TOL",0.03),export_area:e("STUDY_EXPORT_AREA",0.25),grid:e("STUDY_GRID",0.),point_shape:e("STUDY_POINT_SHAPE",0.) as usize,radial:e("STUDY_RADIAL",0.)>0.,smooth_heel:e("STUDY_SMOOTH_HEEL",0.)>0.,point_lateral:e("STUDY_POINT_LATERAL",1.)}
    })
}
pub fn axis(p:&Pheno)->P2 {
    static U:OnceLock<P2>=OnceLock::new();*U.get_or_init(||{
        let qi=p.edge_in.iter().copied().find(|&q|dist(q,p.tip)>=2.).unwrap_or(p.heel);
        let qs=p.edge_sp.iter().rev().copied().find(|&q|dist(q,p.tip)>=2.).unwrap_or(p.heel);
        norm(add(norm(sub(qi,p.tip)),norm(sub(qs,p.tip))))
    })
}
pub fn smooth(u:f64)->f64 {let u=u.clamp(0.,1.);u*u*(3.-2.*u)}
pub fn point_cap(a:f64,core:f64)->f64 {
    let c=config();let a=a.max(0.);let p2=c.apex+2.*c.point_slope;
    if c.point_shape==3 {
        if a<=2. {c.apex+c.point_slope*a}
        else if a<=3.2 {p2+(0.94*core-p2)*(a-2.)/1.2}
        else if a<=5.2 {(0.94+0.005*(a-3.2))*core}
        else if a<=12. {(0.95+0.05*(a-5.2)/6.8)*core}
        else {core}
    } else if c.point_shape==2 {
        if a<=2. {c.apex+c.point_slope*a}
        else if a<=3.5 {p2+(0.90*core-p2)*(a-2.)/1.5}
        else if a<=10. {(0.90+0.004*(a-3.5))*core}
        else if a<=12. {(0.926+0.074*(a-10.)/2.)*core}
        else {core}
    } else if c.point_shape==1 {
        if a<=2. {c.apex+c.point_slope*a}
        else if a<=5. {p2+(0.88*core-p2)*(a-2.)/3.}
        else if a<=10. {(0.88+0.005*(a-5.))*core}
        else if a<=12. {(0.905+0.095*(a-10.)/2.)*core}
        else {core}
    } else if a<=2. {c.apex+c.point_slope*a}
    else if a<c.point_end {p2+(core-p2)*smooth((a-2.)/(c.point_end-2.))}
    else {core}
}
pub fn thickness_at(p:&Pheno,q:P2)->f64 {
    let c=config();
    if c.baseline {return crate::export::baseline_thickness_at(p,q);}
    let raw=p.field.at(q);let g=&p.genome;
    let core=if c.core>0. {c.core}else{g.knot_t(q.0.min(g.v[TAPER_X0]+p.dx),g.v[TAPER_X0]+p.dx)};
    // Retain the machined handle/ring field; reinforce the blade's core instead
    // of retaining the original full-width grinds and distal stock taper.
    let base=if c.smooth_heel {raw+(core-raw)*smooth((q.0-(p.heel.0-15.))/10.)}
        else if q.0>=p.heel.0-5. {core}else{raw};
    let d=p.cut_in.nearest(q,c.d1+0.01).min(p.cut_sp.nearest(q,c.d1+0.01));
    let edge=if d>=c.d1 {base}else{
        let near=c.apex+2.*d*c.angle.to_radians().tan();
        if d<=c.d0 {near}else{near+(base-near)*smooth((d-c.d0)/(c.d1-c.d0))}
    };
    let u=axis(p);let delta=sub(q,p.tip);let axial=delta.0*u.0+delta.1*u.1;let lateral=-delta.0*u.1+delta.1*u.0;
    let a=if c.radial {if c.point_lateral==1. {dist(q,p.tip)}else{axial.hypot(c.point_lateral*lateral)}}else{axial};
    let end=if c.point_shape>0{12.}else{c.point_end};
    let cap=if a<=end {point_cap(a,core)}else{f64::INFINITY};
    let mut value=base.min(edge).min(cap);
    let (radius,_)=bore_dimensions(p);
    if q.0.hypot(q.1)<=p.ring_id/2.+1.5*radius+1e-9 {value=bore_collar_thickness(p,q,value);}
    value.max(0.05)
}
pub fn export_mesh(p:&Pheno,outer:&[P2],holes:&[Vec<P2>],max_area:f64)->Option<jarambit_physics::fine::Mesh> {
    use jarambit_physics::fine;
    let c=config();if c.grid<=0. {return fine::mesh_domain(outer,holes,max_area);}
    let mut pts=Vec::new();let h=c.grid;
    let edges:Vec<_>=p.edge_in.iter().chain(&p.edge_sp).copied().collect();
    let margin=c.d1+0.3;
    let x0=edges.iter().map(|q|q.0).fold(f64::INFINITY,f64::min)-margin;
    let x1=edges.iter().map(|q|q.0).fold(f64::NEG_INFINITY,f64::max)+margin;
    let y0=edges.iter().map(|q|q.1).fold(f64::INFINITY,f64::min)-margin;
    let y1=edges.iter().map(|q|q.1).fold(f64::NEG_INFINITY,f64::max)+margin;
    for j in 0..=((y1-y0)/h).ceil() as usize {for i in 0..=((x1-x0)/h).ceil() as usize {
        let q=(x0+i as f64*h,y0+j as f64*h);
        let d=p.cut_in.nearest(q,margin).min(p.cut_sp.nearest(q,margin));
        if d>margin || d<0.008 {continue;}
        if point_in_poly(q,outer)&&!holes.iter().any(|hole|point_in_poly(q,hole)){pts.push(q);}
    }}
    let u=axis(p);
    for i in 1..=480 {let q=add(p.tip,mul(u,i as f64*0.025));
        if point_in_poly(q,outer)&&!holes.iter().any(|hole|point_in_poly(q,hole)){pts.push(q);}
    }
    fine::mesh_domain_pts(outer,holes,&pts,max_area)
}
