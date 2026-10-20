//! Neutral analytic controls authored for the complete presentation freeze; UNRUN beforehand.
use jarambit_paper_artifacts::population_story::{PositiveAxis, Preference};
use jarambit_paper_artifacts::positive_points::{
    Abscissa, PositivePointRaster, EPSILON, HEIGHT, SCORE_PALETTE, WIDTH,
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
    let (svg, receipt) = r
        .render(
            &r.score_png_base64().unwrap(),
            "Neutral screening observations",
        )
        .unwrap();
    assert_eq!(
        receipt["x_axis"]["coordinate_formula"],
        "log10(one_based_file_ordinal)"
    );
    assert!(svg.contains("one-based birth-file ordinal"));
    assert!(!svg.contains(">G0<"));
    assert!(!svg.contains("stroke=\"#111827\""));
    assert!(!svg.contains("stroke-width=\"7\""));
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
    let (svg, receipt) = r
        .render(&r.score_png_base64().unwrap(), "Neutral constant scope")
        .unwrap();
    assert!(svg.ends_with("</svg>\n"));
    assert!(!svg.contains("</svg>\\n"));
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
        let (_, receipt) = r
            .render(&r.score_png_base64().unwrap(), "Neutral exact endpoints")
            .unwrap();
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

#[test]
fn score_palette_endpoints_preference_and_axis_reversal_are_truthful() {
    assert_eq!(
        SCORE_PALETTE,
        [
            [8, 28, 74],
            [13, 148, 136],
            [250, 204, 21],
            [234, 88, 12],
            [153, 27, 27]
        ]
    );
    for preference in [Preference::Higher, Preference::Lower] {
        for gap in [false, true] {
            let r = PositivePointRaster::new(
                [-4., 2.],
                preference,
                gap,
                Abscissa::Generation { last: 1 },
                2,
            )
            .unwrap();
            let (worst, best) = if preference == Preference::Higher {
                (-4., 2.)
            } else {
                (2., -4.)
            };
            assert_eq!(r.score_color_fraction(worst).unwrap(), 0.);
            assert_eq!(r.score_color_fraction(best).unwrap(), 1.);
            assert_eq!(r.score_color(worst).unwrap(), [8, 28, 74]);
            assert_eq!(r.score_color(best).unwrap(), [153, 27, 27]);
            let colors: Vec<_> = [-4., -2., 0., 1., 2.]
                .iter()
                .map(|v| r.score_color_fraction(*v).unwrap())
                .collect();
            for pair in colors.windows(2) {
                assert!(if preference == Preference::Higher {
                    pair[0] < pair[1]
                } else {
                    pair[0] > pair[1]
                });
            }
            // Independent known point: normalized full range for f=-1 is1/2.
            let low = EPSILON.log10();
            let high = (1. + EPSILON).log10();
            let log = (0.5 + EPSILON).log10();
            let scaled = (log - low) / (high - low);
            let expected = if gap || preference == Preference::Lower {
                1. - scaled
            } else {
                scaled
            };
            assert!((r.score_color_fraction(-1.).unwrap() - expected).abs() < 1e-15);
        }
    }
}

#[test]
fn color_key_uses_original_units_and_correct_scope_endpoints() {
    for preference in [Preference::Higher, Preference::Lower] {
        for gap in [false, true] {
            let mut r = PositivePointRaster::new(
                [-4., 2.],
                preference,
                gap,
                Abscissa::Generation { last: 1 },
                2,
            )
            .unwrap();
            r.paint(&[1], 0, -4.).unwrap();
            r.paint(&[2], 1, 2.).unwrap();
            let (svg, receipt) = r
                .render(
                    &r.score_png_base64().unwrap(),
                    "Neutral original scorer / stage / run fixture",
                )
                .unwrap();
            let ticks = receipt["score_color"]["legend_original_unit_ticks"]
                .as_array()
                .unwrap();
            assert_eq!(ticks.len(), 5);
            assert_eq!(
                ticks[0]["original_value"],
                if preference == Preference::Higher {
                    -4.
                } else {
                    2.
                }
            );
            assert_eq!(
                ticks[4]["original_value"],
                if preference == Preference::Higher {
                    2.
                } else {
                    -4.
                }
            );
            for tick in ticks {
                let raw = tick["original_value"].as_f64().unwrap();
                let fraction = tick["palette_fraction"].as_f64().unwrap();
                assert!((r.score_color_fraction(raw).unwrap() - fraction).abs() < 1e-9);
                assert!(svg.contains(tick["label"].as_str().unwrap()));
            }
            assert!(svg.contains("Warmer = better"));
            assert!(svg.contains("epsilon = 1e-6"));
            assert!(svg.contains("Source-order overpainting"));
            assert!(svg.contains("Haloed dark curve"));
            assert!(svg.contains("stroke=\"#111827\" stroke-width=\"3\""));
            assert!(svg.contains("stroke=\"#ffffff\" stroke-width=\"7\""));
            assert_eq!(receipt["score_color"]["warmer_is_better"], true);
            assert_eq!(receipt["score_color"]["monotone_luminance_claim"], false);
            assert_eq!(
                receipt["score_color"]["axis_canonical_json_sha256"],
                receipt["axis_canonical_json_sha256"]
            );
        }
    }
}

#[test]
fn score_color_does_not_change_ordered_census_coordinates_ties_or_footprint() {
    let mut r = PositivePointRaster::new(
        [-4., 2.],
        Preference::Higher,
        true,
        Abscissa::Generation { last: 2 },
        4,
    )
    .unwrap();
    let points = [(91u64, 0u64, -4f64), (92, 1, 0.), (93, 2, 2.), (94, 2, 2.)];
    let (mut ids, mut operations) = (Sha256::new(), Sha256::new());
    let mut legacy_ink = vec![0u8; WIDTH * HEIGHT];
    for (id, x, f) in points {
        // Independent predecessor formula; no color method controls the coordinate.
        let px = (((x as f64 + 1.).log10() / 3f64.log10()) * (WIDTH - 1) as f64).round() as usize;
        let log = ((2. - f) / 6. + EPSILON).log10();
        let py = ((1. - (log - EPSILON.log10()) / ((1. + EPSILON).log10() - EPSILON.log10()))
            * (HEIGHT - 1) as f64)
            .round() as usize;
        assert_eq!(r.paint(&[id], x, f).unwrap(), [px, py]);
        ids.update(id.to_le_bytes());
        operations.update(1u64.to_le_bytes());
        operations.update(id.to_le_bytes());
        for part in [
            x.to_le_bytes(),
            f.to_bits().to_le_bytes(),
            (px as u64).to_le_bytes(),
            (py as u64).to_le_bytes(),
        ] {
            operations.update(part);
        }
        for y in py.saturating_sub(1)..=(py + 1).min(HEIGHT - 1) {
            for x in px.saturating_sub(1)..=(px + 1).min(WIDTH - 1) {
                legacy_ink[y * WIDTH + x] = 1;
            }
        }
    }
    let receipt = r.receipt().unwrap();
    assert_eq!(
        receipt["identity_order_sha256"],
        format!("{:x}", ids.finalize())
    );
    assert_eq!(
        receipt["point_operation_sha256"],
        format!("{:x}", operations.finalize())
    );
    assert_eq!(
        receipt["raster"]["visible_mask_sha256"],
        format!("{:x}", Sha256::digest(&legacy_ink))
    );
    assert_eq!(r.pixels(), legacy_ink);
    assert_eq!(receipt["numeric_point_operations"], 4);
    assert_eq!(receipt["original_negative_records"], 1);
    assert_eq!(receipt["original_zero_records"], 1);
    assert_eq!(receipt["leader_ties"], 2);
    assert_eq!(receipt["overlapping_center_operations"], 1);
    assert_eq!(receipt["excluded_numeric_records"], 0);
}

#[test]
fn last_original_mark_overpaints_rgb_without_density_or_winner_selection() {
    // Distinct source values/colors whose exact log positions share a center.
    let value = |fraction: f64| {
        10f64.powf(EPSILON.log10() + fraction * ((1. + EPSILON).log10() - EPSILON.log10()))
            - EPSILON
    };
    let (a, b) = (value(0.2), value(0.2011));
    for sequence in [[a, b], [b, a]] {
        let mut r = PositivePointRaster::new(
            [0., 1.],
            Preference::Higher,
            false,
            Abscissa::Generation { last: 1 },
            4,
        )
        .unwrap();
        r.paint(&[1], 0, 0.).unwrap();
        r.paint(&[2], 1, 1.).unwrap();
        assert_ne!(
            r.score_color(sequence[0]).unwrap(),
            r.score_color(sequence[1]).unwrap()
        );
        let a = r.paint(&[3], 0, sequence[0]).unwrap();
        let b = r.paint(&[4], 0, sequence[1]).unwrap();
        assert_eq!(a, b);
        let at = 3 * (b[1] * WIDTH + b[0]);
        assert_eq!(
            &r.score_rgb_pixels()[at..at + 3],
            &r.score_color(sequence[1]).unwrap()
        );
        assert_eq!(r.receipt().unwrap()["numeric_point_operations"], 4);
    }
}

#[test]
fn constant_color_is_common_midpoint_and_nonfinite_never_mutates_painting() {
    let mut r = PositivePointRaster::new(
        [0., 0.],
        Preference::Lower,
        true,
        Abscissa::Generation { last: 0 },
        2,
    )
    .unwrap();
    let white = r.score_rgb_pixels().to_vec();
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 1.] {
        assert!(r.score_color(bad).is_err());
        assert!(r.paint(&[0], 0, bad).is_err());
        assert_eq!(r.score_rgb_pixels(), white);
    }
    assert!(r.score_png_base64().is_err());
    assert_eq!(r.score_color(0.).unwrap(), [250, 204, 21]);
    r.paint(&[1], 0, 0.).unwrap();
    r.paint(&[2], 0, 0.).unwrap();
    let (svg, receipt) = r
        .render(
            &r.score_png_base64().unwrap(),
            "Neutral constant original score",
        )
        .unwrap();
    assert!(svg.contains("all marks use midpoint yellow"));
    assert!(!svg.contains("fill=\"#081c4a\""));
    assert_eq!(
        receipt["score_color"]["legend_original_unit_ticks"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        receipt["score_color"]["legend_original_unit_ticks"][0]["original_value"],
        0.
    );
    assert!(r.render("AA==", "Neutral wrong or occupancy PNG").is_err());
}

#[test]
fn finite_extreme_domains_keep_correct_score_color_and_no_hidden_saturation() {
    for preference in [Preference::Higher, Preference::Lower] {
        for gap in [false, true] {
            let r = PositivePointRaster::new(
                [-f64::MAX, f64::MAX],
                preference,
                gap,
                Abscissa::Generation { last: 1 },
                2,
            )
            .unwrap();
            assert_eq!(
                r.score_color(if preference == Preference::Higher {
                    -f64::MAX
                } else {
                    f64::MAX
                })
                .unwrap(),
                SCORE_PALETTE[0]
            );
            assert_eq!(
                r.score_color(if preference == Preference::Higher {
                    f64::MAX
                } else {
                    -f64::MAX
                })
                .unwrap(),
                SCORE_PALETTE[4]
            );
            assert!(r.score_color_fraction(0.).unwrap() > 0.);
            assert!(r.score_color_fraction(0.).unwrap() < 1.);
        }
    }
}

#[test]
fn emitted_rgb_png_roundtrips_actual_white_background_and_ordered_score_pixels() {
    let mut r = PositivePointRaster::new(
        [-1., 1.],
        Preference::Higher,
        true,
        Abscissa::Generation { last: 1 },
        2,
    )
    .unwrap();
    r.paint(&[1], 0, -1.).unwrap();
    r.paint(&[2], 1, 1.).unwrap();
    let encoded = r.score_png_base64().unwrap();
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut png = Vec::new();
    for group in encoded.as_bytes().chunks_exact(4) {
        let val = |byte: u8| alphabet.iter().position(|&v| v == byte).unwrap_or(0) as u32;
        let word =
            (val(group[0]) << 18) | (val(group[1]) << 12) | (val(group[2]) << 6) | val(group[3]);
        png.push((word >> 16) as u8);
        if group[2] != b'=' {
            png.push((word >> 8) as u8);
        }
        if group[3] != b'=' {
            png.push(word as u8);
        }
    }
    assert_eq!(&png[..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    assert_eq!(&png[12..16], b"IHDR");
    assert_eq!(
        u32::from_be_bytes(png[16..20].try_into().unwrap()),
        WIDTH as u32
    );
    assert_eq!(
        u32::from_be_bytes(png[20..24].try_into().unwrap()),
        HEIGHT as u32
    );
    assert_eq!(&png[24..29], &[8, 2, 0, 0, 0]); // RGB8, not the legacy 4bit indexed palette.
    let length = u32::from_be_bytes(png[33..37].try_into().unwrap()) as usize;
    assert_eq!(&png[37..41], b"IDAT");
    let zlib = &png[41..41 + length];
    assert_eq!(&zlib[..2], &[0x78, 0x01]);
    let (mut at, mut raw) = (2, Vec::new());
    loop {
        let header = zlib[at];
        assert!(header == 0 || header == 1);
        at += 1;
        let len = u16::from_le_bytes(zlib[at..at + 2].try_into().unwrap());
        let inverse = u16::from_le_bytes(zlib[at + 2..at + 4].try_into().unwrap());
        assert_eq!(len, !inverse);
        at += 4;
        raw.extend_from_slice(&zlib[at..at + len as usize]);
        at += len as usize;
        if header == 1 {
            break;
        }
    }
    assert_eq!(at + 4, zlib.len());
    let (mut a, mut b) = (1u32, 0u32);
    for byte in &raw {
        a = (a + u32::from(*byte)) % 65521;
        b = (b + a) % 65521;
    }
    assert_eq!(
        u32::from_be_bytes(zlib[at..at + 4].try_into().unwrap()),
        (b << 16) | a
    );
    assert_eq!(raw.len(), HEIGHT * (1 + WIDTH * 3));
    for (row, pixels) in raw
        .chunks_exact(1 + WIDTH * 3)
        .zip(r.score_rgb_pixels().chunks_exact(WIDTH * 3))
    {
        assert_eq!(row[0], 0);
        assert_eq!(&row[1..], pixels);
    }
    assert_eq!(
        &r.score_rgb_pixels()[3 * (WIDTH * 200 + 400)..3 * (WIDTH * 200 + 400) + 3],
        &[255, 255, 255]
    );
    assert!(r
        .render("AA==", "Neutral occupancy must not masquerade as score RGB")
        .is_err());
}
