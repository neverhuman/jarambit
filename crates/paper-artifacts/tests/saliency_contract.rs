//! Analytic neutral presentation controls. Authored only; no production solve.
use jarambit_paper_artifacts::saliency::{
    coverage_for_bbox, expand_context_viewport, fit, select, Affine2, CropDecision, CropPolicy,
    FieldView, SourceBindings, Viewport,
};

struct Fixture {
    xy: Vec<[f64;2]>, triangles: Vec<[usize;3]>, values: Vec<f64>,
    masked: Vec<bool>, missing: Vec<Option<String>>,
}
impl Fixture {
    fn islands(cells:&[(f64,f64,f64,f64)]) -> Self {
        let mut f=Self{xy:Vec::new(),triangles:Vec::new(),values:Vec::new(),
            masked:vec![false;cells.len()],missing:vec![None;cells.len()]};
        for &(x,y,size,value) in cells {
            let i=f.xy.len();
            f.xy.extend([[x,y],[x+size,y],[x,y+size]]);
            f.triangles.push([i,i+1,i+2]); f.values.push(value);
        }
        f
    }
    fn view(&self)->FieldView<'_> {
        FieldView{xy:&self.xy,triangles:&self.triangles,values:&self.values,
            masked:&self.masked,missing_reasons:&self.missing}
    }
}
fn binding()->SourceBindings {
    SourceBindings{source_path:"neutral-fixture-only".into(),source_sha256:"0".repeat(64),
        certificate_identity:"neutral-no-production-authority".into(),
        field_selector:"/mesh/neutral_response".into(),units:"neutral-unit".into()}
}
fn views()->[Viewport;3] {
    [Viewport{x:0.,y:0.,width:500.,height:185.},
     Viewport{x:0.,y:0.,width:345.,height:185.},
     Viewport{x:390.,y:128.,width:140.,height:100.}]
}
fn choose(f:&Fixture)->jarambit_paper_artifacts::saliency::CropReceipt {
    let [full,zoom,locator]=views();
    select(&f.view(),&binding(),&CropPolicy::proposal_v1(),full,zoom,locator).unwrap()
}
fn near(a:f64,b:f64) { assert!((a-b).abs()<1e-10,"{a} != {b}"); }

#[test]
fn exact_triangle_clip_has_known_area_and_integral_not_centroid_membership() {
    let f=Fixture::islands(&[(0.,0.,2.,4.)]);
    let c=coverage_for_bbox(&f.view(),[0.,0.,0.5,2.],2.).unwrap();
    // Integral from x=0..1/2 of (2-x) dx = 7/8.
    near(c.whole_valid_area,2.); near(c.cropped_valid_area,7./8.);
    near(c.whole_raw_integral,8.); near(c.cropped_raw_integral,3.5);
    near(c.whole_excess_mass,4.); near(c.cropped_excess_mass,1.75);
    near(c.excess_fraction.unwrap(),7./16.);
}
#[test]
fn translated_and_quarter_turned_source_coordinates_preserve_the_clip_oracle() {
    let f=Fixture::islands(&[(17.,-3.,2.,4.)]);
    let c=coverage_for_bbox(&f.view(),[17.,-3.,17.5,-1.],2.).unwrap();
    near(c.cropped_valid_area,7./8.);
    let mut turned=Fixture::islands(&[(0.,0.,2.,4.)]);
    for p in &mut turned.xy { *p=[-p[1],p[0]]; }
    let c=coverage_for_bbox(&turned.view(),[-2.,0.,0.,0.5],2.).unwrap();
    near(c.cropped_valid_area,7./8.); near(c.cropped_raw_integral,3.5);
}
#[test]
fn concentrated_field_keeps_the_original_scale_and_whole_source_area_denominator() {
    let f=Fixture::islands(&[(0.,0.,1.,10.),(9.,9.,1.,0.)]);
    let before:Vec<_>=f.values.iter().map(|x|x.to_bits()).collect();
    let r=choose(&f);
    assert_eq!(r.decision,CropDecision::Zoom);
    near(r.coverage.excess_fraction.unwrap(),1.);
    near(r.coverage.whole_valid_area,1.);
    near(r.coverage.cropped_valid_area,0.5);
    near(r.coverage.area_fraction.unwrap(),0.5);
    assert_eq!(r.record["full_raw_colorbar"],serde_json::json!([0.,10.]));
    assert_eq!(r.record["colors_renormalized_to_crop"],false);
    assert_eq!(before,f.values.iter().map(|x|x.to_bits()).collect::<Vec<_>>());
    // Geometric bbox fraction is not source triangle-area coverage.
    assert_ne!(r.record["attempted_candidates"][0]["bbox_area_fraction"].as_f64().unwrap(),
        r.coverage.area_fraction.unwrap());
}
#[test]
fn a_tiny_disconnected_tied_peak_cannot_be_discarded_for_a_compact_picture() {
    let f=Fixture::islands(&[(0.,0.,1.,10.),(100.,0.,0.0001,10.),(50.,0.,1.,0.)]);
    let r=choose(&f);
    assert_eq!(r.decision,CropDecision::FullContext);
    assert_eq!(r.record["peak_cell_ids"],serde_json::json!([0,1]));
    assert_eq!(r.record["attempted_candidates"][0]["high_value_component_counts"],serde_json::json!([1,1]));
    near(r.coverage.excess_fraction.unwrap(),1.);
}
#[test]
fn upper_half_range_is_not_an_area_quantile_and_threshold_ties_move_together() {
    let f=Fixture::islands(&[(0.,0.,10.,0.),(20.,0.,1.,5.),(25.,0.,1.,5.),(30.,0.,1.,10.)]);
    let r=choose(&f);
    // More than half the valid area has value0, but half of the value range is5.
    near(r.record["whole_field_excess_baseline_t50"].as_f64().unwrap(),5.);
    assert_eq!(r.record["attempted_candidates"][0]["selected_cell_ids"],serde_json::json!([1,2,3]));
    assert_eq!(r.record["attempted_candidates"][0]["value_threshold"],5.);
}
#[test]
fn masks_typed_nan_and_genuine_zero_have_separate_support_states() {
    let mut f=Fixture::islands(&[(0.,0.,1.,0.),(3.,0.,1.,f64::NAN),(6.,0.,1.,10.),(9.,0.,1.,0.)]);
    f.masked[0]=true; f.missing[1]=Some("retained observation unavailable".into());
    let r=choose(&f);
    assert_eq!(r.record["masked_count"],1);
    assert_eq!(r.record["missing_count"],1);
    assert_eq!(r.record["finite_valid_count"],2);
    assert_eq!(r.record["genuine_zero_count"],1);
    near(r.coverage.whole_valid_area,1.);
    let [a,b,c]=views(); f.missing[1]=None;
    assert!(select(&f.view(),&binding(),&CropPolicy::proposal_v1(),a,b,c)
        .unwrap_err().contains("typed missing"));
}
#[test]
fn empty_and_constant_observations_never_manufacture_an_excess_percentage() {
    let mut f=Fixture::islands(&[(0.,0.,1.,f64::NAN)]);
    f.missing[0]=Some("source missing".into());
    let r=choose(&f);
    assert_eq!(r.decision,CropDecision::Missing);
    assert!(r.coverage.excess_fraction.is_none());
    assert!(r.coverage.area_fraction.is_none());
    assert!(r.record["full_raw_colorbar"].is_null());
    f.missing[0]=None; f.values[0]=0.;
    let r=choose(&f);
    assert_eq!(r.decision,CropDecision::Constant);
    assert!(r.coverage.excess_fraction.is_none());
    near(r.coverage.area_fraction.unwrap(),1.);
    f.values[0]=7.;
    assert_eq!(choose(&f).decision,CropDecision::Constant);
}
#[test]
fn signed_infinite_and_invalid_geometry_are_refused_without_a_new_saliency_convention() {
    let [a,b,c]=views();
    let mut f=Fixture::islands(&[(0.,0.,1.,-1.)]);
    assert!(select(&f.view(),&binding(),&CropPolicy::proposal_v1(),a,b,c).is_err());
    f.values[0]=f64::INFINITY;
    assert!(select(&f.view(),&binding(),&CropPolicy::proposal_v1(),a,b,c).is_err());
    f.values[0]=1.; f.xy[2]=f.xy[1];
    assert!(select(&f.view(),&binding(),&CropPolicy::proposal_v1(),a,b,c).is_err());
    f.xy[2]=[0.,1.]; f.triangles[0][2]=99;
    assert!(select(&f.view(),&binding(),&CropPolicy::proposal_v1(),a,b,c).is_err());
}
#[test]
fn edge_components_do_not_join_through_a_vertex_only_contact() {
    let f=Fixture {
        xy:vec![[0.,0.],[1.,0.],[0.,1.],[-1.,0.],[0.,-1.],[8.,8.],[9.,8.],[8.,9.]],
        triangles:vec![[0,1,2],[0,3,4],[5,6,7]],values:vec![10.,10.,0.],
        masked:vec![false;3],missing:vec![None;3],
    };
    let r=choose(&f);
    assert_eq!(r.record["attempted_candidates"][0]["high_value_component_counts"],serde_json::json!([1,1]));
}
#[test]
fn locator_uses_the_same_exact_affine_and_preserves_a_rotated_crop_quadrilateral() {
    let a=fit([0.,0.,10.,5.],Viewport{x:100.,y:200.,width:200.,height:100.}).unwrap();
    assert_eq!(a.rectangle([1.,1.,2.,3.]),[[120.,280.],[140.,280.],[140.,240.],[120.,240.]]);
    let rotated=Affine2{xx:2.,xy:1.,yx:-1.,yy:2.,tx:10.,ty:20.};
    assert_eq!(rotated.rectangle([1.,2.,4.,6.]),[[14.,23.],[20.,20.],[24.,28.],[18.,31.]]);
    let r=choose(&Fixture::islands(&[(0.,0.,1.,10.),(9.,9.,1.,0.)]));
    assert_eq!(r.locator_crop_quad,r.source_to_locator.rectangle(r.bbox_source_mm));
}
#[test]
fn source_rounding_scale_clips_original_triangles_and_does_not_reclassify_a_mask() {
    let mut f=Fixture::islands(&[(0.,0.,0.001,4.),(1.,1.,0.001,0.)]);
    f.masked[1]=true;
    let c=coverage_for_bbox(&f.view(),[0.,0.,0.0005,0.001],2.).unwrap();
    near(c.cropped_valid_area,0.000000375);
    near(c.area_fraction.unwrap(),0.75);
    assert_eq!(f.masked,[false,true]);
}
#[test]
fn changed_policy_or_unbound_source_is_not_silently_accepted() {
    let f=Fixture::islands(&[(0.,0.,1.,10.),(9.,9.,1.,0.)]);
    let [a,b,c]=views(); let mut policy=CropPolicy::proposal_v1();
    policy.min_excess_mass_coverage=0.;
    assert!(select(&f.view(),&binding(),&policy,a,b,c).is_err());
    let mut source=binding(); source.source_sha256.clear();
    assert!(select(&f.view(),&source,&CropPolicy::proposal_v1(),a,b,c).is_err());
}

#[test]
fn context_camera_has_a_minimum_whole_object_span_and_keeps_the_selected_roi() {
    let f=Fixture::islands(&[(0.,0.,1.,10.),(25.,0.,1.,4.),(99.,9.,1.,0.)]);
    let mut r=choose(&f);
    assert_eq!(r.decision,CropDecision::Zoom);
    let selected=r.bbox_source_mm;
    let selected_coverage=r.record["coverage"].clone();
    let original_value_bits:Vec<_>=f.values.iter().map(|v|v.to_bits()).collect();
    expand_context_viewport(&f.view(),&mut r,views()[1]).unwrap();
    assert!(r.bbox_source_mm[2]-r.bbox_source_mm[0] >= 30.);
    assert!(r.bbox_source_mm[0]<=selected[0] && r.bbox_source_mm[2]>=selected[2]);
    assert!(r.bbox_source_mm[1]<=selected[1] && r.bbox_source_mm[3]>=selected[3]);
    assert_eq!(r.record["selection_roi"]["bbox_source_mm"],serde_json::json!(selected));
    assert_eq!(r.record["selection_roi"]["coverage"],selected_coverage);
    assert_eq!(original_value_bits,f.values.iter().map(|v|v.to_bits()).collect::<Vec<_>>());
    assert_eq!(r.record["full_raw_colorbar"],serde_json::json!([0.,10.]));
}

#[test]
fn context_locator_and_accounting_bind_the_actual_larger_camera() {
    let f=Fixture::islands(&[(0.,0.,1.,10.),(25.,0.,1.,4.),(99.,9.,1.,0.)]);
    let mut r=choose(&f);
    expand_context_viewport(&f.view(),&mut r,views()[1]).unwrap();
    assert_eq!(r.locator_crop_quad,r.source_to_locator.rectangle(r.bbox_source_mm));
    let independently_measured=coverage_for_bbox(&f.view(),r.bbox_source_mm,5.).unwrap();
    near(r.coverage.area_fraction.unwrap(),independently_measured.area_fraction.unwrap());
    near(r.coverage.area_fraction.unwrap(),2./3.);
    near(r.record["selection_roi"]["coverage"]["source_area_fraction"].as_f64().unwrap(),1./3.);
    near(r.coverage.excess_fraction.unwrap(),1.);
    assert_eq!(r.record["bbox_source_mm"],serde_json::json!(r.bbox_source_mm));
    assert_eq!(r.record["source_to_panel"],r.source_to_panel.json());
}

#[test]
fn context_camera_clamps_at_distal_source_limits_without_losing_peak_ties() {
    let f=Fixture::islands(&[(99.,9.,1.,10.),(99.5,8.,0.1,10.),(0.,0.,1.,0.)]);
    let mut r=choose(&f);
    let peaks=r.record["peak_cell_ids"].clone();
    expand_context_viewport(&f.view(),&mut r,views()[1]).unwrap();
    assert!(r.bbox_source_mm[0]>=0. && r.bbox_source_mm[2]<=100.);
    assert!(r.bbox_source_mm[1]>=0. && r.bbox_source_mm[3]<=10.);
    assert!(r.bbox_source_mm[2]-r.bbox_source_mm[0]>=30.);
    assert_eq!(r.record["peak_cell_ids"],peaks);
    near(r.coverage.excess_fraction.unwrap(),1.);
}

#[test]
fn context_camera_does_not_invent_saliency_for_constant_or_missing_fields() {
    let f=Fixture::islands(&[(0.,0.,1.,7.)]);
    let mut r=choose(&f);let before=r.record.clone();
    expand_context_viewport(&f.view(),&mut r,views()[1]).unwrap();
    assert_eq!(r.decision,CropDecision::Constant);assert_eq!(r.record,before);
    let mut missing=Fixture::islands(&[(0.,0.,1.,f64::NAN)]);
    missing.missing[0]=Some("retained neutral observation missing".into());
    let mut r=choose(&missing);let before=r.record.clone();
    expand_context_viewport(&missing.view(),&mut r,views()[1]).unwrap();
    assert_eq!(r.decision,CropDecision::Missing);assert_eq!(r.record,before);
}
