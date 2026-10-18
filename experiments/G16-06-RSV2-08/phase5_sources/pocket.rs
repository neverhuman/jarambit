use crate::{compound,geom::*,pheno::Pheno};
use serde_json::{json,Value};
use std::sync::OnceLock;
pub const X0:f64=102.;
pub const X1:f64=165.;
pub const BLEND:f64=1.5;
pub fn distances(p:&Pheno,q:P2)->(f64,f64) {
    static SPINE:OnceLock<Segments>=OnceLock::new();
    let sp=SPINE.get_or_init(||Segments::new(&p.curve[p.cidx[30]..]));
    let c=compound::config();
    let ls=c.pocket_spine+BLEND+0.001;let le=c.pocket_edge+BLEND+0.001;
    (sp.nearest(q,ls).min(p.cut_sp.nearest(q,ls)),p.cut_in.nearest(q,le))
}
pub fn weight(p:&Pheno,q:P2)->f64 {
    let c=compound::config();
    if c.pocket_t<=0. || q.0<=c.pocket_x0 || q.0>=c.pocket_x1{return 0.;}
    let(ds,de)=distances(p,q);
    compound::smooth((q.0-c.pocket_x0)/BLEND)*compound::smooth((c.pocket_x1-q.0)/BLEND)
        *compound::smooth((ds-c.pocket_spine)/BLEND)*compound::smooth((de-c.pocket_edge)/BLEND)
}
pub fn apply(p:&Pheno,q:P2,t:f64)->f64 {t-(t-compound::config().pocket_t).max(0.)*weight(p,q)}
pub fn audit(p:&Pheno)->Value {
    let c=compound::config();let(mut material,mut modified,mut fullstock)=(0usize,0usize,0usize);let mut flat=0usize;
    let(mut protected_min,mut min_t,mut max_depth)=(f64::INFINITY,f64::INFINITY,0f64);
    let(mut spine_delta,mut edge_delta,mut outside_delta,mut removed)=(0f64,0f64,0f64,0f64);
    for(i,&v)in p.field.th.iter().enumerate(){
        if !v.is_finite(){continue;}
        let q=(p.field.x0+(i%p.field.nx)as f64*p.field.g+p.field.g/2.,p.field.y0+(i/p.field.nx)as f64*p.field.g+p.field.g/2.);
        let before=compound::base_thickness_at(p,q);let after=compound::thickness_at(p,q);let delta=(before-after).abs();
        if q.0<c.pocket_x0||q.0>c.pocket_x1{outside_delta=outside_delta.max(delta);continue;}
        material+=1;let(ds,de)=distances(p,q);if c.pocket_t>0. && weight(p,q)>=1.-1e-12 && (after-c.pocket_t).abs()<1e-10{flat+=1;}
        if ds<=c.pocket_spine{spine_delta=spine_delta.max(delta);}
        if de<=c.pocket_edge{edge_delta=edge_delta.max(delta);}
        if (ds<=c.pocket_spine||de<=c.pocket_edge)&&(before-6.).abs()<1e-12{fullstock+=1;protected_min=protected_min.min(after);}
        if delta>1e-12{modified+=1;min_t=min_t.min(after);max_depth=max_depth.max(delta);removed+=(before-after)*p.field.g*p.field.g;}
        assert!(after<=before+1e-12);
    }
    let ymin=p.outline.iter().map(|q|q.1).fold(f64::INFINITY,f64::min);let ymax=p.outline.iter().map(|q|q.1).fold(f64::NEG_INFINITY,f64::max);
    let sections:Vec<Value>=[104.,110.,120.,130.,140.,150.,160.,164.].into_iter().map(|x|{
        let mut pts=Vec::new();let h=0.05;
        for j in 0..=((ymax-ymin)/h).ceil()as usize{
            let q=(x,ymin+(j as f64+0.5)*h);
            if !point_in_poly(q,&p.outline)||p.holes.iter().any(|hole|point_in_poly(q,hole)){continue;}
            pts.push((q.1,compound::base_thickness_at(p,q),compound::thickness_at(p,q)));
        }
        let proxy=|new:bool|{
            let t=|v:&(f64,f64,f64)|if new{v.2}else{v.1};
            let a=pts.iter().map(|v|t(v)*h).sum::<f64>();let y0=pts.iter().map(|v|v.0*t(v)*h).sum::<f64>()/a;
            (a,y0,pts.iter().map(|v|t(v)*(v.0-y0).powi(2)*h).sum::<f64>(),pts.iter().map(|v|t(v).powi(3)*h).sum::<f64>())
        };
        let old=proxy(false);let new=proxy(true);
        json!({"x_mm":x,"samples":pts.len(),"dy_mm":h,"baseline_area_mm2":old.0,"pocket_area_mm2":new.0,"baseline_neutral_y_mm":old.1,"pocket_neutral_y_mm":new.1,"inplane_I_ratio":new.2/old.2,"outofplane_integral_t3_ratio":new.3/old.3})
    }).collect();
    json!({"x0_mm":c.pocket_x0,"x1_mm":c.pocket_x1,"blend_mm":BLEND,"material_samples":material,"modified_samples":modified,"flat_floor_samples":flat,"modified_min_t_mm":if modified>0{Some(min_t)}else{None},"max_removed_thickness_mm":max_depth,"raster_removed_volume_mm3":removed,"protected_full_stock_samples":fullstock,"protected_full_stock_min_mm":protected_min,"spine_guard_max_delta_mm":spine_delta,"edge_guard_max_delta_mm":edge_delta,"outside_extent_max_delta_mm":outside_delta,"sections":sections,"section_scope":"dy=0.05 mm quadrature diagnostic, not an FEM substitute"})
}
