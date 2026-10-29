use crate::sdf::{bevel_texture_weight, blade_bevel_detail, build_g51_sdf};
use jarambit_g5::g5_sdf::{build_blade_profile_2d, build_g5_sdf, eval_blade_thickness_and_sdf2, GUARD_X_MM};

/// The copied bevel law gives G5.0's thickness and planform distance bit for bit, on and off the planform.
#[test]
fn bevel_detail_reproduces_the_g5_thickness_law() {
    let mut checked = 0usize;
    for g in super::textured_blade_designs(3, 0.0) {
        let p = g.decode();
        let prof = build_blade_profile_2d(&p);
        let (y_lo, y_hi) = (
            prof.y_in.iter().copied().fold(f64::INFINITY, f64::min) - 2.0,
            prof.y_sp.iter().copied().fold(f64::NEG_INFINITY, f64::max) + 2.0,
        );
        for a in 0..=160 {
            let x = prof.x_heel - 1.0 + (prof.x_tip - prof.x_heel + 2.0) * (a as f64) / 160.0;
            for b in 0..=80 {
                let y = y_lo + (y_hi - y_lo) * (b as f64) / 80.0;
                let (t5, d5) = eval_blade_thickness_and_sdf2(&p, &prof, x, y);
                match blade_bevel_detail(&p, &prof, x, y) {
                    Some(d) => {
                        assert_eq!((d.t_final.to_bits(), d.d2.to_bits()), (t5.to_bits(), d5.to_bits()), "at ({x}, {y})");
                        checked += 1;
                    }
                    None => assert_eq!((t5, d5), (0.0, 20.0)),
                }
            }
        }
    }
    assert!(checked > 10_000, "{checked}");
}

/// Off the bevels the G5.1 field is G5.0's voxel for voxel; on them (weight 0) it is the untextured field; the
/// pre-texture field and the grid are identical everywhere. Some bevel voxels G5.0 textured are untextured in G5.1.
#[test]
fn g51_field_is_g5s_off_the_bevels_and_untextured_on_them() {
    let designs = super::textured_blade_designs(2, 0.03);
    assert_eq!(designs.len(), 2, "no Sobol design with blade texture");
    for g in designs {
        let p = g.decode();
        let prof = build_blade_profile_2d(&p);
        let a = build_g5_sdf(&g, 0.4, false);
        let b = build_g51_sdf(&g, 0.4, false);
        assert_eq!((a.nx, a.ny, a.nz), (b.nx, b.ny, b.nz));
        assert_eq!(a.x, b.x);
        assert_eq!(a.d_pre_tex, b.d_pre_tex);
        assert_eq!(a.tmap_eff_2d, b.tmap_eff_2d);
        let (mut same, mut masked, mut removed) = (0usize, 0usize, 0usize);
        for i in 0..a.nx {
            let x = a.x[i];
            for j in 0..a.ny {
                let w = if x >= GUARD_X_MM { bevel_texture_weight(&p, &prof, x, a.y[j]) } else { 1.0 };
                for k in 0..a.nz {
                    let idx = (i * a.ny + j) * a.nz + k;
                    if w == 1.0 {
                        assert_eq!(a.d[idx].to_bits(), b.d[idx].to_bits(), "off-bevel voxel ({i},{j},{k}) changed");
                        same += 1;
                    } else if w == 0.0 {
                        masked += 1;
                        let interior = i > 0 && i + 1 < a.nx && j > 0 && j + 1 < a.ny && k > 0 && k + 1 < a.nz;
                        if interior {
                            assert_eq!(b.d[idx].to_bits(), b.d_pre_tex[idx].to_bits(), "texture on a bevel ({i},{j},{k})");
                        }
                        if a.d[idx] != a.d_pre_tex[idx] {
                            removed += 1;
                        }
                    }
                }
            }
        }
        assert!(same > 0 && masked > 0, "same {same}, masked {masked}");
        assert!(removed > 0, "G5.0 put no texture on this design's bevels (masked {masked})");
    }
}

/// `blade_outer_field_at` is the G5.1 outer field of the pure blade zone (x >= 92 mm), bit for bit at every interior
/// voxel, textured designs included, so a local tooth model at any voxel size has exactly the decoder's geometry.
#[test]
fn blade_outer_field_matches_the_grid() {
    use crate::sdf::blade_outer_field_at;
    for g in super::textured_blade_designs(2, 0.03) {
        let p = g.decode();
        let prof = build_blade_profile_2d(&p);
        let grid = build_g51_sdf(&g, 0.4, false);
        let mut checked = 0usize;
        for i in 1..grid.nx - 1 {
            for j in 1..grid.ny - 1 {
                for k in 1..grid.nz - 1 {
                    if let Some(v) = blade_outer_field_at(&p, &prof, grid.x[i], grid.y[j], grid.z[k]) {
                        let idx = (i * grid.ny + j) * grid.nz + k;
                        assert_eq!(v.to_bits(), grid.d[idx].to_bits(), "({i},{j},{k}) {v} vs {}", grid.d[idx]);
                        checked += 1;
                    }
                }
            }
        }
        assert!(checked > 100_000, "{checked}");
    }
}

/// No step at the plunge line: walking along each sharpened edge, just inside it, the texture weight rises from 0
/// on the bevel to 1 heel-ward of the grind start smoothly (no jump above 0.1 between samples 0.05 mm apart).
#[test]
fn bevel_mask_has_no_step_at_the_plunge_line() {
    let mut crossings = 0usize;
    for g in super::textured_blade_designs(6, 0.0) {
        let p = g.decode();
        let prof = build_blade_profile_2d(&p);
        for inner in [true, false] {
            let edge = if inner { &prof.y_in } else { &prof.y_sp };
            let inward = if inner { 0.3 } else { -0.3 };
            let at = |x: f64| {
                let u = ((x - prof.x_heel) / (prof.x_tip - prof.x_heel)).clamp(0.0, 1.0) * (prof.xs.len() - 1) as f64;
                let i0 = (u.floor() as usize).min(prof.xs.len() - 2);
                let f = u - i0 as f64;
                bevel_texture_weight(&p, &prof, x, edge[i0] * (1.0 - f) + edge[i0 + 1] * f + inward)
            };
            let (mut prev, mut x) = (at(prof.x_heel + 0.5), prof.x_heel + 0.55);
            let (mut seen_one, mut seen_zero) = (prev == 1.0, prev == 0.0);
            while x < prof.x_tip - 10.0 {
                let w = at(x);
                assert!((w - prev).abs() <= 0.1, "inner {inner}: weight jumps {prev} -> {w} at x {x}");
                seen_one |= w == 1.0;
                seen_zero |= w == 0.0;
                prev = w;
                x += 0.05;
            }
            crossings += usize::from(seen_one && seen_zero);
        }
    }
    assert!(crossings >= 4, "only {crossings} edges cross from unground to ground");
}
