//! Neutral analytic controls authored for the complete presentation freeze; UNRUN beforehand.
use jarambit_paper_artifacts::population_story::{PositiveAxis, Preference};
use jarambit_paper_artifacts::positive_points::{
    Abscissa, PositivePointRaster, EPSILON, HEIGHT, WIDTH,
};
use sha2::{Digest, Sha256};

#[test]
fn every_duplicate_signed_and_zero_observation_has_an_operation() {
    let mut r = PositivePointRaster::new(
        [-4., 2.],
        Preference::Higher,
        true,
        Abscissa::Generation { last: 2 },
        5,
    )
    .unwrap();
    for (id, x, f) in [
        (7, 0, -4.),
        (8, 1, 0.),
        (9, 2, 2.),
        (10, 2, 2.),
        (11, 2, 2.),
    ] {
        r.paint(&[id], x, f).unwrap();
    }
    let receipt = r.receipt().unwrap();
    assert_eq!(receipt["numeric_point_operations"], 5);
    assert_eq!(receipt["original_negative_records"], 1);
    assert_eq!(receipt["original_zero_records"], 1);
    assert_eq!(receipt["leader_ties"], 3);
    assert_eq!(receipt["overlapping_center_operations"], 2);
    assert_eq!(receipt["excluded_numeric_records"], 0);
    let mut h = Sha256::new();
    for id in [7_u64, 8, 9, 10, 11] {
        h.update(id.to_le_bytes());
    }
    assert_eq!(r.identity_order_sha256(), format!("{:x}", h.finalize()));
}
#[test]
fn final_fixed_gap_keeps_early_improvements_distinct_and_exact_endpoints_visible() {
    let r = PositivePointRaster::new(
        [-4., 2.],
        Preference::Higher,
        true,
        Abscissa::Generation { last: 99 },
        3,
    )
    .unwrap();
    assert_eq!(r.position(0, -4.).unwrap(), [0, 0]);
    assert_eq!(r.position(99, 2.).unwrap(), [WIDTH - 1, HEIGHT - 1]);
    let early = r.position(9, -4.).unwrap();
    let later = r.position(9, 1.).unwrap();
    assert_eq!(early[0], later[0]);
    assert!(early[1] < later[1]);
    // Independent fixed final b=2,s=6: f=1 has gap1/6, not running-best zero.
    let expected = ((1.
        - ((1. / 6. + EPSILON).log10() - EPSILON.log10())
            / ((1. + EPSILON).log10() - EPSILON.log10()))
        * (HEIGHT - 1) as f64)
        .round() as usize;
    assert_eq!(later[1], expected);
}
#[test]
fn file_ordinals_are_not_shifted_or_labeled_as_generations() {
    let mut r = PositivePointRaster::new(
        [2., 8.],
        Preference::Lower,
        true,
        Abscissa::FileOrdinal { last: 100 },
        3,
    )
    .unwrap();
    assert!(r.paint(&[0, 1], 0, 8.).is_err());
    assert_eq!(r.paint(&[0, 1], 1, 8.).unwrap()[0], 0);
    // Consecutive source ordinals would expose the forbidden pooled step path.
    r.paint(&[1, 1], 2, 6.).unwrap();
    assert_eq!(r.paint(&[99, 1], 100, 2.).unwrap()[0], WIDTH - 1);
    let (svg, receipt) = r.render("AA==", "Neutral screening observations").unwrap();
    assert_eq!(
        receipt["x_axis"]["coordinate_formula"],
        "log10(one_based_file_ordinal)"
    );
    assert!(svg.contains("one-based birth-file ordinal"));
    assert!(!svg.contains(">G0<"));
    assert!(!svg.contains("stroke=\"#0072b2\""));
    assert!(svg.contains("No trajectory: accession order"));
    assert!(receipt["first_retained_leader_generation"].is_null());
}
#[test]
fn missing_extra_outside_and_nonfinite_values_fail_without_imputation() {
    let mut r = PositivePointRaster::new(
        [-4., 2.],
        Preference::Higher,
        false,
        Abscissa::Generation { last: 2 },
        2,
    )
    .unwrap();
    assert!(r.paint(&[1], 0, f64::NAN).is_err());
    assert!(r.paint(&[1], 0, f64::INFINITY).is_err());
    assert!(r.paint(&[1], 3, -4.).is_err());
    assert!(r.paint(&[1], 0, -4.1).is_err());
    r.paint(&[1], 0, -4.).unwrap();
    assert!(r.receipt().is_err());
    r.paint(&[2], 2, 2.).unwrap();
    assert!(r.receipt().is_ok());
    assert!(r.paint(&[3], 1, 0.).is_err());
}
#[test]
fn complete_extrema_are_required_even_when_the_row_count_matches() {
    let mut r = PositivePointRaster::new(
        [-4., 2.],
        Preference::Higher,
        true,
        Abscissa::Generation { last: 2 },
        2,
    )
    .unwrap();
    r.paint(&[1], 0, -3.).unwrap();
    r.paint(&[2], 2, 2.).unwrap();
    assert!(r.receipt().is_err());
}
#[test]
fn raw_points_and_summary_axis_share_original_values_and_inverse() {
    let domain = [-4., 2.];
    let axis = PositiveAxis::new(domain, Preference::Higher, true, EPSILON).unwrap();
    let r = PositivePointRaster::new(
        domain,
        Preference::Higher,
        true,
        Abscissa::Generation { last: 2 },
        2,
    )
    .unwrap();
    for raw in [-4., -2., 0., 1., 2.] {
        let expected = ((1.
            - (axis.transform(raw).unwrap() - EPSILON.log10())
                / ((1. + EPSILON).log10() - EPSILON.log10()))
            * (HEIGHT - 1) as f64)
            .round() as usize;
        assert_eq!(r.position(1, raw).unwrap()[1], expected);
        assert!(
            (axis
                .inverse_coordinate(axis.coordinate(raw).unwrap())
                .unwrap()
                - raw)
                .abs()
                < 1e-9
        );
    }
}
#[test]
fn constant_scope_has_one_raw_tick_and_no_fake_variation() {
    let mut r = PositivePointRaster::new(
        [0., 0.],
        Preference::Higher,
        true,
        Abscissa::Generation { last: 0 },
        2,
    )
    .unwrap();
    r.paint(&[1], 0, 0.).unwrap();
    r.paint(&[2], 0, 0.).unwrap();
    let (_, receipt) = r.render("AA==", "Neutral constant scope").unwrap();
    assert_eq!(receipt["tick_rows"].as_array().unwrap().len(), 1);
    assert_eq!(receipt["tick_rows"][0]["original_value"], 0.);
    assert_eq!(receipt["axis"]["scale"]["constant_fallback"], true);
}
#[test]
fn extreme_finite_original_endpoints_remain_drawn() {
    let mut r = PositivePointRaster::new(
        [-f64::MAX, f64::MAX],
        Preference::Higher,
        false,
        Abscissa::Generation { last: 1 },
        2,
    )
    .unwrap();
    assert_eq!(r.paint(&[1], 0, -f64::MAX).unwrap(), [0, HEIGHT - 1]);
    assert_eq!(r.paint(&[2], 1, f64::MAX).unwrap(), [WIDTH - 1, 0]);
    assert!(r.receipt().is_ok());
}

#[test]
fn rendered_ticks_restore_exact_raw_endpoints_before_log_inverse_roundoff() {
    for gap in [false, true] {
        let mut r = PositivePointRaster::new(
            [-4., 2.],
            Preference::Higher,
            gap,
            Abscissa::Generation { last: 1 },
            2,
        )
        .unwrap();
        r.paint(&[1], 0, -4.).unwrap();
        r.paint(&[2], 1, 2.).unwrap();
        let (_, receipt) = r.render("AA==", "Neutral exact endpoints").unwrap();
        let ticks = receipt["tick_rows"].as_array().unwrap();
        assert_eq!(
            ticks.first().unwrap()["original_value"],
            if gap { 2. } else { -4. }
        );
        assert_eq!(
            ticks.last().unwrap()["original_value"],
            if gap { -4. } else { 2. }
        );
    }
}
