//! Authored neutral fixtures only: UNRUN until actual serial integration/freeze.
use jarambit_paper_artifacts::population_story::{
    BoundLeader,BoundRunTrace,GenerationView,LeaderTie,PositiveAxis,Preference,
    SourceBinding,TraceSample,View,event_receipt,render_run_panel,validate_run,
};
fn fixture()->BoundRunTrace {
    let values=[(-4.,-4.),(1.,-2.),(2.,-1.),(2.,0.),(2.,1.)];
    BoundRunTrace {
        label:"Neutral Run A".into(),metric:"stored neutral score".into(),preference:Preference::Higher,
        original_domain:[-4.,2.],
        source:SourceBinding{path:"neutral/source.json".into(),sha256:"a".repeat(64),
            scope_pointer:"/runs/0".into(),compatible_run:"neutral-A".into(),stage:"retained observations".into(),
            membership_identity_order_sha256:"b".repeat(64)},
        leader:BoundLeader{original_value:2.,original_token:"2.0".into(),ledger_sha256:"c".repeat(64),
            representative_tie_index:0,
            ties:vec![LeaderTie{public_individual:11,original_id:"source-11".into(),generation:2},
                      LeaderTie{public_individual:13,original_id:"source-13".into(),generation:4}]},
        samples:values.into_iter().enumerate().map(|(i,(best,median))|TraceSample {
            generation:i as u64,cumulative_best:Some(best),population_median:Some(median),
            best_source_pointer:Some(format!("/runs/0/best/{i}")),
            median_source_pointer:Some(format!("/runs/0/median/{i}")),
        }).collect(),
    }
}
#[test]
fn original_signed_zero_order_and_inverse_are_preserved() {
    let axis=PositiveAxis::new([-4.,2.],Preference::Higher,false,1e-6).unwrap();
    let source=[-4.,-1.,0.,1.,2.];
    let coords:Vec<_>=source.iter().map(|v|axis.coordinate(*v).unwrap()).collect();
    assert!(coords.windows(2).all(|w|w[0]<w[1]));
    for (f,c) in source.into_iter().zip(coords) {
        assert!((axis.inverse_coordinate(c).unwrap()-f).abs()<1e-12);
    }
    // Independent affine oracle: original zero is4/6 of the complete range.
    assert!((axis.coordinate(0.).unwrap()-(2./3.+1e-6)).abs()<1e-15);
}
#[test]
fn fixed_endpoint_gap_does_not_flatten_the_running_best() {
    let r=fixture();let axis=PositiveAxis::new(r.original_domain,r.preference,true,1e-6).unwrap();
    let got:Vec<_>=r.samples.iter().map(|s|axis.coordinate(s.cumulative_best.unwrap()).unwrap()).collect();
    // Fixed b=2, s=6: gaps are1,1/6,0,0,0, then display floor.
    for (actual,gap) in got.iter().zip([1.,1./6.,0.,0.,0.]) {
        assert!((*actual-(gap+1e-6)).abs()<1e-14);
    }
    assert!(got[0]>got[1] && got[1]>got[2]);
    assert_eq!(got[2],got[3]);assert_eq!(got[3],got[4]);
}
#[test]
fn lower_preference_gap_uses_the_retained_minimum() {
    let a=PositiveAxis::new([-4.,2.],Preference::Lower,true,1e-6).unwrap();
    assert_eq!(a.coordinate(-4.).unwrap(),1e-6);
    assert!((a.coordinate(2.).unwrap()-(1.+1e-6)).abs()<1e-15);
    assert!((a.inverse_coordinate(a.coordinate(-1.).unwrap()).unwrap()+1.).abs()<1e-12);
}
#[test]
fn constant_singleton_has_one_matching_axis_tick() {
    let mut r=fixture();r.original_domain=[2.,2.];
    r.samples.truncate(1);r.samples[0].cumulative_best=Some(2.);r.samples[0].population_median=Some(2.);
    r.leader.ties.truncate(1);r.leader.ties[0].generation=0;
    let out=render_run_panel(&r,View::PositiveGapLog10{epsilon:1e-6},GenerationView::Linear).unwrap();
    assert_eq!(out.receipt["tick_rows"].as_array().unwrap().len(),1);
    assert_eq!(out.receipt["tick_rows"][0]["label"],"0 (best)");
    assert_eq!(out.receipt["progress_events"]["terminal_contiguous_no_strict_improvement_transitions"],0);
    assert_eq!(out.receipt["view_axis"]["scale"]["units"],1.);
    assert!(out.svg.contains("multi-generation plateau")); // "no" may wrap onto preceding SVG text row; numerical state checked above.
}
#[test]
fn finite_extreme_range_never_requires_an_infinite_scale() {
    let m=f64::MAX;
    for gap in [false,true] {
        let axis=PositiveAxis::new([-m,m],Preference::Higher,gap,1e-6).unwrap();
        for (f,position) in [(-m,0.),(0.,0.5),(m,1.)] {
            let expected=if gap {1.-position} else {position};
            assert!((axis.coordinate(f).unwrap()-(expected+1e-6)).abs()<1e-15);
            let restored=axis.inverse_log(axis.transform(f).unwrap()).unwrap();
            if f==0. {assert!(restored.abs()/m<1e-14);} else {assert!((restored/f-1.).abs()<1e-14);}
        }
        assert_ne!(axis.coordinate(-m).unwrap(),axis.coordinate(m).unwrap());
        assert_eq!(axis.metadata()["scale"]["factorized_overflow_safe"],true);
    }
}
#[test]
fn invalid_floors_domains_and_scope_values_fail_closed() {
    for e in [0.,-1.,f64::NAN,f64::INFINITY,1.] {
        assert!(PositiveAxis::new([-4.,2.],Preference::Higher,false,e).is_err());
    }
    assert!(PositiveAxis::new([2.,-4.],Preference::Higher,false,1e-6).is_err());
    assert!(PositiveAxis::new([0.,f64::INFINITY],Preference::Higher,false,1e-6).is_err());
    let axis=PositiveAxis::new([-4.,2.],Preference::Higher,false,1e-6).unwrap();
    for v in [-5.,3.,f64::NAN,f64::NEG_INFINITY] {assert!(axis.coordinate(v).is_err());}
}
#[test]
fn original_leader_token_ties_and_representative_are_not_reselected() {
    let mut r=fixture();r.leader.representative_tie_index=1;
    let out=render_run_panel(&r,View::Asinh{linear_threshold:1.},GenerationView::Linear).unwrap();
    assert!(out.svg.contains("Individual 13 at G4"));
    assert!(out.svg.contains("2.0"));
    assert_eq!(out.receipt["original_leader"]["exact_ties"].as_array().unwrap().len(),2);
    assert_eq!(out.receipt["original_leader"]["representative_tie_index_from_ledger"],1);
    r.leader.original_token="2.0001".into();assert!(validate_run(&r).is_err());
    r.leader.original_token="2.0".into();r.leader.representative_tie_index=2;assert!(validate_run(&r).is_err());
    r.leader.representative_tie_index=0;r.leader.ties[1].public_individual=11;assert!(validate_run(&r).is_err());
}
#[test]
fn source_pointers_use_original_array_positions_not_generation() {
    let mut r=fixture();r.samples.remove(1);
    // Source is a sparse array: G2 resides at source index1, NOT index2.
    for (i,s) in r.samples.iter_mut().enumerate() {
        s.best_source_pointer=Some(format!("/runs/0/sparse_best/{i}"));
        s.median_source_pointer=Some(format!("/runs/0/sparse_median/{i}"));
    }
    let event=event_receipt(&r).unwrap();
    assert_eq!(event["events"][1]["generation"],2);
    assert_eq!(event["events"][1]["best_source_pointer"],"/runs/0/sparse_best/1");
    assert_eq!(event["events"][1]["adjacent_observed_generation"],false);
    assert_eq!(event["events"][1]["improvement_generation_resolved"],false);
}
#[test]
fn invalid_missing_duplicate_or_out_of_scope_source_pointers_are_rejected() {
    let mut r=fixture();r.samples[1].best_source_pointer=None;assert!(validate_run(&r).is_err());
    let mut r=fixture();r.samples[1].best_source_pointer=Some("/runs/1/best/1".into());assert!(validate_run(&r).is_err());
    let mut r=fixture();r.samples[1].best_source_pointer=r.samples[0].best_source_pointer.clone();assert!(validate_run(&r).is_err());
    let mut r=fixture();r.samples[1].best_source_pointer=Some("/runs/0/invalid~2/1".into());assert!(validate_run(&r).is_err());
}
#[test]
fn missing_is_typed_and_breaks_plateau_and_does_not_become_zero() {
    let mut r=fixture();r.samples[3].cumulative_best=None;r.samples[3].population_median=None;
    let event=event_receipt(&r).unwrap();
    assert!(event["events"][3]["original_cumulative_best"].is_null());
    assert!(event["events"][3]["strict_improvement_since_preceding_observed"].is_null());
    assert!(event["events"][4]["strict_improvement_since_preceding_observed"].is_null());
    assert_eq!(event["terminal_interval_start"],4);
    assert_eq!(event["terminal_contiguous_no_strict_improvement_transitions"],0);
    let out=render_run_panel(&r,View::PositiveGapLog10{epsilon:1e-6},GenerationView::Log10GenerationPlusOne).unwrap();
    assert_eq!(out.receipt["missing_best_slots"],1);
    assert!(out.svg.contains("G0"));assert!(out.svg.contains("log10(g+1)"));
}
#[test]
fn actual_plateau_is_visible_and_not_a_physical_convergence_claim() {
    let r=fixture();let event=event_receipt(&r).unwrap();
    assert_eq!(event["first_observed_retained_best_generation"],2);
    assert_eq!(event["terminal_interval_start"],2);
    assert_eq!(event["terminal_interval_end"],4);
    assert_eq!(event["terminal_contiguous_no_strict_improvement_transitions"],2);
    let out=render_run_panel(&r,View::PositiveGapLog10{epsilon:1e-6},GenerationView::Linear).unwrap();
    assert!(out.svg.contains("G2–G4:"));assert!(out.svg.contains("transitions at saved precision"));
    assert_eq!(out.receipt["physical_convergence_or_qualification_claim"],false);
    assert!(out.receipt["global_comparable_champion"].is_null());
}
#[test]
fn dishonest_regression_or_nonfinite_numeric_input_is_not_silently_clipped() {
    let mut r=fixture();r.samples[3].cumulative_best=Some(1.5);assert!(validate_run(&r).is_err());
    let mut r=fixture();r.samples[1].population_median=Some(1.5);assert!(validate_run(&r).is_err());
    let mut r=fixture();r.samples[1].population_median=Some(f64::NAN);assert!(validate_run(&r).is_err());
    let mut r=fixture();r.samples[1].generation=0;assert!(validate_run(&r).is_err());
    let mut r=fixture();r.samples[4].generation=1u64<<53;assert!(validate_run(&r).is_err());
}
#[test]
fn generated_summaries_do_not_claim_all_record_point_coverage() {
    let mut r=fixture();r.label="Neutral <A> & B".into();
    let out=render_run_panel(&r,View::PositiveAffineLog10{epsilon:1e-6},GenerationView::Linear).unwrap();
    assert!(out.svg.contains("Neutral &lt;A&gt; &amp; B"));
    assert_eq!(out.receipt["all_record_points_drawn_here"],0);
    assert_eq!(out.receipt["all_record_companion_required"],true);
    assert_eq!(out.receipt["all_record_membership_changed"],false);
    assert_eq!(out.receipt["selection_on_display_transform"],false);
    assert_eq!(out.receipt["minimum_font_svg_units"],22);
}

#[test]
fn tiny_finite_domain_has_an_independent_midpoint_and_inverse_oracle() {
    for gap in [false,true] {
        let a=PositiveAxis::new([-1e-300,1e-300],Preference::Higher,gap,1e-6).unwrap();
        for (f,p) in [(-1e-300,0.),(0.,0.5),(1e-300,1.)] {
            let expected=if gap {1.-p} else {p};
            assert!((a.coordinate(f).unwrap()-(expected+1e-6)).abs()<1e-15);
            assert!((a.inverse_log(a.transform(f).unwrap()).unwrap()-f).abs()/1e-300<1e-12);
        }
    }
}
#[test]
fn rendered_affine_ticks_handle_constant_and_finite_extreme_domains() {
    let mut r=fixture();r.original_domain=[2.,2.];
    r.samples.truncate(1);r.samples[0].cumulative_best=Some(2.);r.samples[0].population_median=Some(2.);
    r.leader.ties.truncate(1);r.leader.ties[0].generation=0;
    let out=render_run_panel(&r,View::PositiveAffineLog10{epsilon:1e-6},GenerationView::Linear).unwrap();
    assert_eq!(out.receipt["tick_rows"].as_array().unwrap().len(),1);
    assert_eq!(out.receipt["tick_rows"][0]["label"],"2.0000");
    let mut r=fixture();let m=f64::MAX;r.original_domain=[-m,m];
    r.leader.original_value=m;r.leader.original_token=format!("{m:e}");
    for (s,(best,median)) in r.samples.iter_mut().zip([(-m,-m),(0.,-m),(m,0.),(m,m),(m,m)]) {
        s.cumulative_best=Some(best);s.population_median=Some(median);
    }
    let out=render_run_panel(&r,View::PositiveAffineLog10{epsilon:1e-6},GenerationView::Linear).unwrap();
    assert_eq!(out.receipt["tick_rows"].as_array().unwrap().len(),5);
    assert!(out.receipt["tick_rows"].as_array().unwrap().iter().all(|v|v["display_log_or_asinh_coordinate"].as_f64().unwrap().is_finite()));
    assert!(out.receipt["tick_rows"][0]["label"].as_str().unwrap().contains('e'));
}

#[test]
fn distinct_editorial_aliases_cannot_duplicate_one_original_observation() {
    let mut r=fixture();
    r.leader.ties[1].original_id=r.leader.ties[0].original_id.clone();
    assert_ne!(r.leader.ties[1].public_individual,r.leader.ties[0].public_individual);
    assert!(validate_run(&r).is_err());
}
