use jarambit_physics::fine;
use crate::geom::*;
use serde_json::{Value,json};
use std::cell::RefCell;
thread_local! {static LAST:RefCell<Value>=const{RefCell::new(Value::Null)};}
pub fn record(t6:&fine::T6,sp:Option<&[[f64;3]]>,rb:Option<&[[f64;3]]>,tp:P2,sx0:f64){
    let peak=|nod:Option<&[[f64;3]]>,seated:bool| {
        let mut value=0.;let mut at=(0.,0.);
        if let Some(nod)=nod {for (i,&q) in t6.xy.iter().enumerate(){
            if dist(q,tp)<fine::TIP_EXCL_MM || (seated&&q.0>=sx0-3.) {continue;}
            let v=fine::von_mises(nod[i]);if v>value {value=v;at=q;}
        }}
        json!({"unit_stress":value,"at":at})
    };
    LAST.with(|last|*last.borrow_mut()=json!({"spine":peak(sp,false),"ringback":peak(rb,true)}));
}
pub fn take()->Value {LAST.with(|last|last.borrow().clone())}

pub fn record_sequence(cen:&[P2],excl:&[bool],lat:&[f64],tw:&[f64],heel:f64,te:&[f64],t6:&fine::T6,nod_pr:&[[f64;3]],ring:&[bool],tip:P2,p:&crate::pheno::Pheno) {
    let mut cases=serde_json::Map::new();
    for (name,values) in [("lateral",lat),("chord-torsion",tw)] {
        let(mut h,mut b)=(0usize,0usize);let(mut hs,mut bs)=(0f64,0f64);
        for e in 0..cen.len(){if excl[e]||!values[e].is_finite(){continue;}
            if cen[e].0>=heel && values[e]>bs {bs=values[e];b=e;}
            else if cen[e].0<heel-3. && values[e]>hs {hs=values[e];h=e;}
        }
        let mut flat_peak=0f64;let mut flat_at=(0.,0.);let mut flat_n=0usize;
        let floor=crate::compound::config().pocket_t;
        if name=="lateral" && floor>0. {
            for e in 0..cen.len(){if excl[e]||!values[e].is_finite(){continue;}
                if crate::pocket::weight(p,cen[e])>=1.-1e-12 && (te[e]-floor).abs()<1e-10{
                    flat_n+=1;if values[e]>flat_peak{flat_peak=values[e];flat_at=cen[e];}
                }
            }
        }
        cases.insert(name.into(),json!({"hand_stress":hs,"blade_stress":bs,"ratio":bs/hs,"hand_at":cen[h],"blade_at":cen[b],"hand_t":te[h],"blade_t":te[b],"blade_centroid_pocket_weight":crate::pocket::weight(p,cen[b]),"flat_floor_peak":if flat_n>0{Some(json!({"stress":flat_peak,"at":flat_at,"element_t":floor,"elements":flat_n}))}else{None}}));
    }
    let(mut hs,mut bs)=(0f64,0f64);let(mut h,mut b)=(tip,tip);
    for(i,&q)in t6.xy.iter().enumerate(){if dist(q,tip)<fine::TIP_EXCL_MM{continue;}let v=fine::von_mises(nod_pr[i]);
        if q.0>=heel && v>bs {bs=v;b=q;}else if q.0<heel-3. && !ring[i] && v>hs {hs=v;h=q;}
    }
    cases.insert("pry".into(),json!({"hand_stress":hs,"blade_stress":bs,"ratio":bs/hs,"hand_at":h,"blade_at":b}));
    LAST.with(|last|last.borrow_mut()["sequence_cases"]=Value::Object(cases));
}
