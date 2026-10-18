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
