//! Deterministic display-only crop selection on retained piecewise-constant triangles.
//! No geometry, response, mask, score or physical requirement is changed.
use crate::plate::{clipped_polygon, polygon_area};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

type Result<T> = std::result::Result<T, String>;

#[derive(Clone, Debug)]
pub struct CropPolicy {
    pub version: String,
    pub range_cutoffs: [f64; 3],
    pub min_excess_mass_coverage: f64,
    pub max_bbox_area_fraction: f64,
    pub min_scale_gain: f64,
    pub pad_fraction: f64,
}
impl CropPolicy {
    /// Initial fixed presentation proposal, not an engineering acceptance gate.
    pub fn proposal_v1() -> Self {
        Self {
            version: "jarambit.upper_range_crop_policy.v1".into(),
            range_cutoffs: [0.50, 0.75, 0.90],
            min_excess_mass_coverage: 0.95,
            max_bbox_area_fraction: 0.40,
            min_scale_gain: 1.75,
            pad_fraction: 0.08,
        }
    }
    fn validate(&self) -> Result<()> {
        if self.version != "jarambit.upper_range_crop_policy.v1"
            || self.range_cutoffs != [0.50, 0.75, 0.90]
            || self.min_excess_mass_coverage != 0.95
            || self.max_bbox_area_fraction != 0.40
            || self.min_scale_gain != 1.75
            || self.pad_fraction != 0.08
        {
            return Err("unsupported crop policy; review/version a changed display recipe".into());
        }
        Ok(())
    }
    fn json(&self) -> Value {
        json!({"version":self.version,"range_cutoffs":self.range_cutoffs,
            "threshold_rule":"vmin + cutoff * (vmax-vmin); not an area/count quantile",
            "whole_field_excess_baseline_range_fraction":0.5,
            "min_excess_mass_coverage":self.min_excess_mass_coverage,
            "max_bbox_area_fraction":self.max_bbox_area_fraction,
            "min_scale_gain":self.min_scale_gain,"pad_fraction":self.pad_fraction,
            "padding_floor":"sqrt(minimum selected original triangle area), per axis",
            "status":"fixed initial display proposal; not physical gates"})
    }
}

pub struct FieldView<'a> {
    pub xy: &'a [[f64; 2]],
    pub triangles: &'a [[usize; 3]],
    pub values: &'a [f64],
    pub masked: &'a [bool],
    /// A missing observation needs its own original/source-bound reason.
    /// A NaN without a reason is refused; it never becomes a measured zero.
    pub missing_reasons: &'a [Option<String>],
}
#[derive(Clone, Debug)]
pub struct SourceBindings {
    pub source_path: String,
    pub source_sha256: String,
    pub certificate_identity: String,
    pub field_selector: String,
    pub units: String,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    pub x: f64, pub y: f64, pub width: f64, pub height: f64,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Affine2 {
    pub xx: f64, pub xy: f64, pub yx: f64, pub yy: f64,
    pub tx: f64, pub ty: f64,
}
impl Affine2 {
    pub fn point(self, p: [f64; 2]) -> [f64; 2] {
        [self.xx*p[0] + self.xy*p[1] + self.tx,
         self.yx*p[0] + self.yy*p[1] + self.ty]
    }
    pub fn rectangle(self, b: [f64; 4]) -> [[f64; 2]; 4] {
        [[b[0],b[1]],[b[2],b[1]],[b[2],b[3]],[b[0],b[3]]].map(|p|self.point(p))
    }
    pub fn json(self) -> Value {
        json!([[self.xx,self.xy,self.tx],[self.yx,self.yy,self.ty]])
    }
}
pub fn fit(b: [f64; 4], v: Viewport) -> Result<Affine2> {
    check_bbox(b)?;
    if [v.x,v.y,v.width,v.height].iter().any(|x| !x.is_finite())
        || v.width <= 0. || v.height <= 0.
    {
        return Err("invalid crop viewport".into());
    }
    let scale = (v.width/(b[2]-b[0])).min(v.height/(b[3]-b[1]));
    let a = Affine2 {xx:scale,xy:0.,yx:0.,yy:-scale,
        tx:v.x+(v.width-(b[2]-b[0])*scale)/2.-b[0]*scale,
        ty:v.y+(v.height-(b[3]-b[1])*scale)/2.+b[3]*scale};
    if [a.xx,a.yy,a.tx,a.ty].iter().any(|x| !x.is_finite()) {
        return Err("nonfinite crop transform".into());
    }
    Ok(a)
}
fn check_bbox(b: [f64; 4]) -> Result<()> {
    if b.iter().any(|x| !x.is_finite()) || b[0] >= b[2] || b[1] >= b[3] {
        return Err("invalid source crop bounds".into());
    }
    Ok(())
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CropDecision { Zoom, FullContext, Missing, Constant }
#[derive(Clone, Debug)]
pub struct Coverage {
    pub whole_valid_area: f64,
    pub cropped_valid_area: f64,
    pub whole_raw_integral: f64,
    pub cropped_raw_integral: f64,
    pub whole_excess_mass: f64,
    pub cropped_excess_mass: f64,
    pub area_fraction: Option<f64>,
    pub raw_integral_fraction: Option<f64>,
    pub excess_fraction: Option<f64>,
}
impl Coverage {
    fn json(&self) -> Value {
        json!({"whole_valid_area":self.whole_valid_area,"cropped_valid_area":self.cropped_valid_area,
            "whole_raw_response_integral":self.whole_raw_integral,
            "cropped_raw_response_integral":self.cropped_raw_integral,
            "whole_positive_excess_mass":self.whole_excess_mass,
            "cropped_positive_excess_mass":self.cropped_excess_mass,
            "source_area_fraction":self.area_fraction,
            "raw_response_integral_fraction":self.raw_integral_fraction,
            "positive_excess_mass_fraction":self.excess_fraction,
            "denominator_scope":"whole original eligible field; never a selected ROI",
            "excess_meaning":"display saliency integral above original t50; not physical energy"})
    }
}

/// Fixed display camera for the distal lateral/torsion panels, independent of
/// the unchanged saliency selector. No field value or selection is recomputed.
pub const CONTEXT_MIN_WIDTH_FRACTION: f64 = 0.30;
pub const CONTEXT_VIEWPORT_VERSION: &str = "jarambit.curved_tip_context_viewport.v1";

pub fn expand_context_viewport(
    f: &FieldView<'_>, crop: &mut CropReceipt, zoom_view: Viewport,
) -> Result<()> {
    if crop.decision != CropDecision::Zoom { return Ok(()); }
    let p = prepare(f)?;
    let selected = crop.bbox_source_mm;
    check_bbox(selected)?;
    fit(selected, zoom_view)?;
    if selected[0] < p.bbox[0] || selected[1] < p.bbox[1]
        || selected[2] > p.bbox[2] || selected[3] > p.bbox[3]
    { return Err("context viewport selection outside original source bounds".into()); }
    let aspect = zoom_view.width / zoom_view.height;
    let width = (selected[2]-selected[0])
        .max(CONTEXT_MIN_WIDTH_FRACTION*(p.bbox[2]-p.bbox[0]));
    let height = (selected[3]-selected[1]).max(width/aspect);
    let width = width.max(height*aspect);
    let (x0,x1) = bounded_span(selected[0],selected[2],width,p.bbox[0],p.bbox[2]);
    let (y0,y1) = bounded_span(selected[1],selected[3],height,p.bbox[1],p.bbox[3]);
    let bbox = [x0,y0,x1,y1];
    let baseline = crop.record["whole_field_excess_baseline_t50"].as_f64()
        .ok_or("context viewport missing whole-field excess baseline")?;
    let selected_coverage = crop.coverage.json();
    let coverage = measure(f,&p,bbox,baseline)?;
    let panel = fit(bbox,zoom_view)?;
    let quad = crop.source_to_locator.rectangle(bbox);
    crop.record["selection_roi"] = json!({"bbox_source_mm":selected,
        "chosen_value_threshold":crop.chosen_value_threshold,
        "coverage":selected_coverage,"decision":"Zoom",
        "scope":"unchanged saliency-selection result, distinct from the wider display camera"});
    crop.record["display_context"] = json!({"version":CONTEXT_VIEWPORT_VERSION,
        "minimum_whole_object_x_extent_fraction":CONTEXT_MIN_WIDTH_FRACTION,
        "actual_whole_object_x_extent_fraction":(x1-x0)/(p.bbox[2]-p.bbox[0]),
        "requested_physical_aspect":aspect,
        "clamped_to_original_source_bounds":true,
        "contains_complete_selected_roi":true,
        "aspect_fit_may_letterbox_at_domain_limits":true,
        "locator_red_rectangle":"exact displayed source-coordinate viewport, not the smaller selection ROI"});
    crop.record["bbox_source_mm"] = json!(bbox);
    crop.record["bbox_posed_mm"] = json!(bbox);
    crop.record["coverage"] = coverage.json();
    crop.record["source_to_panel"] = panel.json();
    crop.record["locator_crop_quad"] = json!(quad);
    crop.bbox_source_mm = bbox;
    crop.source_to_panel = panel;
    crop.locator_crop_quad = quad;
    crop.coverage = coverage;
    Ok(())
}
#[derive(Debug)]
pub struct CropReceipt {
    pub decision: CropDecision,
    pub bbox_source_mm: [f64; 4],
    pub chosen_value_threshold: Option<f64>,
    pub source_to_panel: Affine2,
    pub source_to_locator: Affine2,
    pub locator_crop_quad: [[f64; 2]; 4],
    pub coverage: Coverage,
    pub record: Value,
}
struct Prepared {
    area: Vec<f64>, valid: Vec<usize>, bbox: [f64; 4],
    masked: usize, missing: usize, zero: usize,
}
fn prepare(f: &FieldView<'_>) -> Result<Prepared> {
    let n=f.triangles.len();
    if n == 0 || [f.values.len(),f.masked.len(),f.missing_reasons.len()].iter().any(|m|*m!=n) {
        return Err("empty or inconsistent field association".into());
    }
    let mut b=[f64::INFINITY,f64::INFINITY,f64::NEG_INFINITY,f64::NEG_INFINITY];
    for p in f.xy {
        if p.iter().any(|x| !x.is_finite()) { return Err("nonfinite source coordinate".into()); }
        b[0]=b[0].min(p[0]); b[1]=b[1].min(p[1]);
        b[2]=b[2].max(p[0]); b[3]=b[3].max(p[1]);
    }
    check_bbox(b)?;
    let mut p=Prepared {area:Vec::with_capacity(n),valid:Vec::new(),bbox:b,masked:0,missing:0,zero:0};
    let mut cells=BTreeSet::new(); let mut edges=BTreeMap::new();
    for (i,t) in f.triangles.iter().enumerate() {
        if t.iter().any(|j|*j>=f.xy.len()) { return Err("source triangle index out of bounds".into()); }
        let mut key=*t; key.sort_unstable();
        if key[0]==key[1] || key[1]==key[2] || !cells.insert(key) {
            return Err("duplicate or collapsed source triangle".into());
        }
        let poly=t.map(|j|f.xy[j]); let area=polygon_area(&poly);
        if !area.is_finite() || area<=0. { return Err("zero/nonfinite source triangle area".into()); }
        p.area.push(area);
        for (a,b) in [(t[0],t[1]),(t[1],t[2]),(t[2],t[0])] {
            let count=edges.entry((a.min(b),a.max(b))).or_insert(0);
            *count+=1; if *count>2 { return Err("nonmanifold source edge".into()); }
        }
        if f.masked[i] {
            if f.missing_reasons[i].is_some() { return Err("masked and missing states overlap".into()); }
            p.masked+=1;
        } else if let Some(reason)=&f.missing_reasons[i] {
            if reason.trim().is_empty() || f.values[i].is_infinite() {
                return Err("invalid typed missing value/reason".into());
            }
            p.missing+=1;
        } else {
            let v=f.values[i];
            if !v.is_finite() { return Err("nonfinite value requires a typed missing reason".into()); }
            if v<0. { return Err("unsupported signed field; no absolute-value saliency inference".into()); }
            p.zero+=usize::from(v==0.); p.valid.push(i);
        }
    }
    Ok(p)
}
fn measure(f:&FieldView<'_>, p:&Prepared, bbox:[f64;4], baseline:f64) -> Result<Coverage> {
    check_bbox(bbox)?;
    if !baseline.is_finite() || baseline<0. { return Err("invalid excess baseline".into()); }
    let (mut wa,mut ca,mut wr,mut cr,mut we,mut ce)=(0.,0.,0.,0.,0.,0.);
    for &i in &p.valid {
        let poly=f.triangles[i].iter().map(|j|f.xy[*j]).collect();
        let clipped=polygon_area(&clipped_polygon(poly,bbox));
        let a=p.area[i]; let v=f.values[i]; let e=(v-baseline).max(0.);
        wa+=a; ca+=clipped; wr+=a*v; cr+=clipped*v; we+=a*e; ce+=clipped*e;
    }
    if [wa,ca,wr,cr,we,ce].iter().any(|v| !v.is_finite()) {
        return Err("nonfinite source area/integral".into());
    }
    Ok(Coverage {whole_valid_area:wa,cropped_valid_area:ca,whole_raw_integral:wr,
        cropped_raw_integral:cr,whole_excess_mass:we,cropped_excess_mass:ce,
        area_fraction:(wa>0.).then(||ca/wa),raw_integral_fraction:(wr>0.).then(||cr/wr),
        excess_fraction:(we>0.).then(||ce/we)})
}
/// Public analytic-oracle interface; original triangles are clipped, never their centroids.
pub fn coverage_for_bbox(f:&FieldView<'_>, bbox:[f64;4], baseline:f64) -> Result<Coverage> {
    measure(f,&prepare(f)?,bbox,baseline)
}
fn components(f:&FieldView<'_>, ids:&[usize]) -> Vec<Vec<usize>> {
    let mut owners:BTreeMap<(usize,usize),Vec<usize>>=BTreeMap::new();
    let mut adj:BTreeMap<usize,Vec<usize>>=ids.iter().map(|i|(*i,Vec::new())).collect();
    for &i in ids {
        let t=f.triangles[i];
        for (a,b) in [(t[0],t[1]),(t[1],t[2]),(t[2],t[0])] {
            owners.entry((a.min(b),a.max(b))).or_default().push(i);
        }
    }
    for cells in owners.values().filter(|v|v.len()==2) {
        adj.get_mut(&cells[0]).unwrap().push(cells[1]);
        adj.get_mut(&cells[1]).unwrap().push(cells[0]);
    }
    let mut seen=BTreeSet::new(); let mut result=Vec::new();
    for &id in ids {
        if !seen.insert(id) { continue; }
        let mut stack=vec![id]; let mut group=Vec::new();
        while let Some(i)=stack.pop() {
            group.push(i);
            for &j in &adj[&i] { if seen.insert(j) { stack.push(j); } }
        }
        group.sort_unstable(); result.push(group);
    }
    result
}
fn bounded_span(lo:f64,hi:f64,wanted:f64,full_lo:f64,full_hi:f64)->(f64,f64) {
    let width=wanted.max(hi-lo).min(full_hi-full_lo);
    if width >= full_hi-full_lo { return (full_lo,full_hi); }
    let left=((lo+hi-width)/2.).clamp(full_lo,(full_hi-width).max(full_lo));
    (left,left+width)
}
fn candidate_bbox(f:&FieldView<'_>,p:&Prepared,ids:&[usize],policy:&CropPolicy,aspect:f64)
    -> ([f64;4],[f64;4],f64)
{
    let mut b=[f64::INFINITY,f64::INFINITY,f64::NEG_INFINITY,f64::NEG_INFINITY];
    let mut cell_area=f64::INFINITY;
    for &i in ids {
        cell_area=cell_area.min(p.area[i]);
        for &j in &f.triangles[i] {
            b[0]=b[0].min(f.xy[j][0]); b[1]=b[1].min(f.xy[j][1]);
            b[2]=b[2].max(f.xy[j][0]); b[3]=b[3].max(f.xy[j][1]);
        }
    }
    let selected=b;
    let floor=cell_area.sqrt();
    let px=((b[2]-b[0])*policy.pad_fraction).max(floor);
    let py=((b[3]-b[1])*policy.pad_fraction).max(floor);
    b=[(b[0]-px).max(p.bbox[0]),(b[1]-py).max(p.bbox[1]),
       (b[2]+px).min(p.bbox[2]),(b[3]+py).min(p.bbox[3])];
    if (b[2]-b[0])/(b[3]-b[1]) < aspect {
        (b[0],b[2])=bounded_span(b[0],b[2],(b[3]-b[1])*aspect,p.bbox[0],p.bbox[2]);
    } else {
        (b[1],b[3])=bounded_span(b[1],b[3],(b[2]-b[0])/aspect,p.bbox[1],p.bbox[3]);
    }
    (b,selected,floor)
}
fn hashes(f:&FieldView<'_>) -> (String,String,String) {
    let mut mesh=Sha256::new(); let mut values=Sha256::new(); let mut masks=Sha256::new();
    mesh.update(b"ordered-xy-f64le-and-tris-u64le-v1");
    for p in f.xy { for v in p {mesh.update(v.to_bits().to_le_bytes());} }
    for t in f.triangles {for i in t {mesh.update((*i as u64).to_le_bytes());}}
    for v in f.values {values.update(v.to_bits().to_le_bytes());}
    for (masked,reason) in f.masked.iter().zip(f.missing_reasons) {
        masks.update([u8::from(*masked)]);
        match reason {
            None => masks.update([0]),
            Some(s) => {masks.update([1]);masks.update((s.len() as u64).to_le_bytes());masks.update(s.as_bytes());}
        }
    }
    (format!("{:x}",mesh.finalize()),format!("{:x}",values.finalize()),format!("{:x}",masks.finalize()))
}
pub fn select(f:&FieldView<'_>,source:&SourceBindings,policy:&CropPolicy,
    full_view:Viewport,zoom_view:Viewport,locator_view:Viewport)->Result<CropReceipt>
{
    policy.validate()?;
    if source.source_path.trim().is_empty() || source.units.trim().is_empty()
        || source.certificate_identity.trim().is_empty()
        || !source.field_selector.starts_with("/mesh/")
        || source.source_sha256.len()!=64
        || !source.source_sha256.bytes().all(|c|c.is_ascii_digit()||(b'a'..=b'f').contains(&c))
    { return Err("missing/unbound crop field identity".into()); }
    let p=prepare(f)?; let full_transform=fit(p.bbox,full_view)?;
    // Validate both reserved viewports even when the selected decision is full context.
    fit(p.bbox,zoom_view)?; let locator=fit(p.bbox,locator_view)?;
    let min=p.valid.iter().map(|i|f.values[*i]).reduce(f64::min);
    let max=p.valid.iter().map(|i|f.values[*i]).reduce(f64::max);
    let domain=min.zip(max).map(|(a,b)|[a,b]);
    let baseline=domain.map(|d|d[0]+0.5*(d[1]-d[0]));
    let peaks:Vec<_>=p.valid.iter().copied().filter(|i|Some(f.values[*i])==max).collect();
    let mut decision=if p.valid.is_empty(){CropDecision::Missing}
        else if min==max {CropDecision::Constant} else {CropDecision::FullContext};
    let mut reason=match decision {
        CropDecision::Missing=>"no eligible finite values",CropDecision::Constant=>"constant field; no positive excess",
        _=>"no fixed candidate satisfies coverage, compactness and scale gain"
    };
    let mut bbox=p.bbox; let mut threshold=None; let mut attempted=Vec::new();
    if decision==CropDecision::FullContext {
        let d=domain.unwrap(); let t50=baseline.unwrap();
        for cutoff in policy.range_cutoffs {
            let t=d[0]+cutoff*(d[1]-d[0]);
            let ids:Vec<_>=p.valid.iter().copied().filter(|i|f.values[*i]>=t).collect();
            let groups=components(f,&ids);
            let (candidate,selected,pad_floor)=candidate_bbox(f,&p,&ids,policy,zoom_view.width/zoom_view.height);
            let coverage=measure(f,&p,candidate,t50)?;
            let fraction=(candidate[2]-candidate[0])*(candidate[3]-candidate[1])
                /((p.bbox[2]-p.bbox[0])*(p.bbox[3]-p.bbox[1]));
            let transform=fit(candidate,zoom_view)?;
            let gain=transform.xx/full_transform.xx;
            let all_peaks=peaks.iter().all(|i|f.triangles[*i].iter().all(|j|{
                let q=f.xy[*j]; q[0]>=candidate[0]&&q[0]<=candidate[2]&&q[1]>=candidate[1]&&q[1]<=candidate[3]
            }));
            let high_area: f64 = ids.iter().map(|i|p.area[*i]).sum();
            let clipped_high_area: f64 = ids.iter().map(|i| {
                let poly=f.triangles[*i].iter().map(|j|f.xy[*j]).collect();
                polygon_area(&clipped_polygon(poly,candidate))
            }).sum();
            let accepted=coverage.excess_fraction.is_some_and(|c|c>=policy.min_excess_mass_coverage)
                && fraction<=policy.max_bbox_area_fraction && gain>=policy.min_scale_gain && all_peaks;
            attempted.push(json!({"range_fraction":cutoff,"value_threshold":t,
                "selected_cell_ids":ids,"edge_connected_components":groups,
                "high_value_component_counts":groups.iter().map(Vec::len).collect::<Vec<_>>(),
                "selected_vertex_bbox_source_mm":selected,"bbox_source_mm":candidate,
                "padding_floor_mm":pad_floor,"bbox_area_fraction":fraction,"scale_gain":gain,
                "padding_and_aspect_extent_expansion_mm":[selected[0]-candidate[0],selected[1]-candidate[1],candidate[2]-selected[2],candidate[3]-selected[3]],
                "bound_to_full_source_bbox":true,"aspect_fit_may_letterbox_at_domain_limits":true,
                "high_value_cell_count":ids.len(),"whole_high_value_area":high_area,
                "cropped_high_value_area":clipped_high_area,
                "high_value_source_area_fraction":(high_area>0.).then(||clipped_high_area/high_area),
                "all_peak_triangles_retained":all_peaks,"coverage":coverage.json(),"accepted":accepted}));
            if accepted { bbox=candidate;threshold=Some(t);decision=CropDecision::Zoom;
                reason="least aggressive fixed accepted range cutoff";break; }
        }
    }
    let coverage=measure(f,&p,bbox,baseline.unwrap_or(0.))?;
    let panel=fit(bbox,if decision==CropDecision::Zoom {zoom_view}else{full_view})?;
    let quad=locator.rectangle(bbox); let (mesh_hash,value_hash,mask_hash)=hashes(f);
    let record=json!({"schema":"jarambit.source_triangle_smart_crop.v1","decision":format!("{decision:?}"),
        "reason":reason,"policy":policy.json(),"source":source.source_path,"source_sha256":source.source_sha256,
        "certificate_identity":source.certificate_identity,"field_selector":source.field_selector,
        "units":source.units,"coordinate_units":"mm","source_to_pose":"identity; no geometric rotation",
        "ordered_mesh_sha256":mesh_hash,"ordered_raw_f64_bits_sha256":value_hash,
        "source_mask_and_missing_reason_sha256":mask_hash,
        "finite_valid_count":p.valid.len(),"missing_count":p.missing,"masked_count":p.masked,
        "genuine_zero_count":p.zero,"full_eligible_raw_domain":domain,
        "full_raw_colorbar":max.map(|v|[0.,v]),"whole_field_excess_baseline_t50":baseline,
        "peak_cell_ids":peaks,"chosen_value_threshold":threshold,"full_bbox_source_mm":p.bbox,
        "bbox_source_mm":bbox,"bbox_posed_mm":bbox,"coverage":coverage.json(),
        "source_to_panel":panel.json(),"source_to_locator":locator.json(),
        "locator_crop_quad":quad,"attempted_candidates":attempted,
        "colors_renormalized_to_crop":false,"new_solver_execution":false,
        "visible_ink_coverage_claim":false,"full_context_companion_required":true});
    Ok(CropReceipt {decision,bbox_source_mm:bbox,chosen_value_threshold:threshold,
        source_to_panel:panel,source_to_locator:locator,locator_crop_quad:quad,coverage,record})
}
