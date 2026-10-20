// Neutral analytic fixtures only. Authored for later admitted execution; no
// retained artifact, CAD decoder, scientific evaluator or image file is used.
#[path = "../src/appearance.rs"]
pub mod appearance;
#[path = "../src/hero_appearance.rs"]
pub mod hero_appearance;
use appearance::*;
use hero_appearance::{
    AppearanceRecipe, CellColor, RegisteredCellColors, RegistrationInput, RetainedSurface,
};

fn settings(crease: f64) -> NormalSettings {
    NormalSettings {
        crease_degrees: crease,
        blend: 1.,
        max_deviation_degrees: 20.,
    }
}
fn camera() -> Camera {
    Camera {
        rotation: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
        scale: 6.,
        offset: [8., 56.],
    }
}
fn cfg() -> RenderSettings {
    RenderSettings {
        width: 64,
        height: 64,
        supersample: 2,
        outline_pixels: 1.,
        camera: camera(),
        lighting: Lighting {
            direction: [-0.35, 0.4, 1.],
            ambient: 0.35,
            diffuse: 0.65,
            specular: 0.04,
            shininess: 24,
        },
    }
}
fn bent_panel() -> RawMesh {
    RawMesh::new(vec![
        [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        [[1., 0., 0.], [1., 1., 0.25], [0., 1., 0.]],
    ])
    .unwrap()
}
fn quad(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<[Vec3; 3]> {
    vec![
        [[x0, y0, 0.], [x1, y0, 0.], [x1, y1, 0.]],
        [[x0, y0, 0.], [x1, y1, 0.], [x0, y1, 0.]],
    ]
}
fn pixel(rgb: &[u8], x: usize, y: usize) -> &[u8] {
    &rgb[3 * (y * 64 + x)..3 * (y * 64 + x) + 3]
}
fn binary(mesh: &RawMesh) -> Vec<u8> {
    let mut b = vec![0_u8; 84 + 50 * mesh.len()];
    b[..5].copy_from_slice(b"solid");
    b[80..84].copy_from_slice(&(mesh.len() as u32).to_le_bytes());
    for (i, t) in mesh.triangles().iter().enumerate() {
        for (corner, p) in t.iter().enumerate() {
            for (d, &x) in p.iter().enumerate() {
                let o = 84 + 50 * i + 12 + corner * 12 + d * 4;
                b[o..o + 4].copy_from_slice(&(x as f32).to_le_bytes());
            }
        }
    }
    b
}
fn surface(mesh: &RawMesh) -> RetainedSurface {
    let bytes = binary(mesh);
    assert_eq!(decode_stl(&bytes).unwrap().triangles(), mesh.triangles());
    RetainedSurface::from_stl(
        "neutral fixture",
        &bytes,
        &digest(&bytes),
        &mesh.geometry_digest(),
    )
    .unwrap()
}
fn cell(value: Option<f64>) -> CellColor {
    CellColor {
        index: value,
        missing_reason: value.is_none().then(|| "explicit fixture absence".into()),
        source_masks: vec![value.is_none(); 4],
        contributors: if value.is_some() {
            vec!["a".into(), "b".into(), "c".into(), "d".into()]
        } else {
            vec![]
        },
        base_rgb: if value.is_some() {
            [13, 148, 136]
        } else {
            [203, 213, 225]
        },
    }
}
fn input(mesh: &RawMesh) -> RegistrationInput {
    RegistrationInput {
        subject: "neutral fixture".into(),
        d1_sha256: digest(b"neutral original D1 bytes"),
        channel_ids: ["a", "b", "c", "d"].map(str::to_string).to_vec(),
        cells: vec![cell(Some(0.))],
        face_to_cell: vec![0; mesh.len()],
    }
}

#[test]
fn normals_and_lighting_do_not_change_ordered_geometry_visibility_or_unlit_colors() {
    let m = bent_panel();
    let before = m.geometry_digest();
    let flat = m.flat_normals();
    let (smooth, stats) = derive_normals(&m, settings(30.), None).unwrap();
    assert!(stats.changed_corners > 0);
    let colors = vec![[13, 148, 136]; m.len()];
    let camera = Camera {
        scale: 40.,
        offset: [12., 52.],
        ..camera()
    };
    let a = render(&m, &flat, &colors, RenderSettings { camera, ..cfg() }).unwrap();
    let b = render(&m, &smooth, &colors, RenderSettings { camera, ..cfg() }).unwrap();
    assert_eq!(a.coverage, b.coverage);
    assert_eq!(a.depth, b.depth);
    assert_eq!(a.winning_face, b.winning_face);
    assert_eq!(a.outline, b.outline);
    assert_eq!(a.unlit_rgb, b.unlit_rgb);
    assert_ne!(a.rgb, b.rgb);
    assert_eq!(m.geometry_digest(), before);
    assert_eq!(b.submitted_triangles, m.len());
    assert_eq!(derive_normals(&m, settings(30.), None).unwrap().0, smooth);
    // Internal source triangle edges are not black contour lines.
    assert_ne!(pixel(&b.unlit_rgb, 32, 32), &[0, 0, 0]);
}
#[test]
fn crease_regions_deviation_zero_blend_and_invalid_coefficients_are_controls() {
    let m = bent_panel();
    let flat = m.flat_normals();
    assert_eq!(derive_normals(&m, settings(5.), None).unwrap().0, flat);
    assert_eq!(
        derive_normals(&m, settings(30.), Some(&[1, 2])).unwrap().0,
        flat
    );
    let (capped, stats) = derive_normals(
        &m,
        NormalSettings {
            max_deviation_degrees: 1.,
            ..settings(30.)
        },
        None,
    )
    .unwrap();
    assert!(stats.capped_corners > 0);
    assert_eq!(capped, flat);
    assert_eq!(
        derive_normals(
            &m,
            NormalSettings {
                blend: 0.,
                ..settings(30.)
            },
            None
        )
        .unwrap()
        .0,
        flat
    );
    assert!(derive_normals(&m, settings(f64::NAN), None).is_err());
    assert!(derive_normals(&m, settings(30.), Some(&[1])).is_err());
}
#[test]
fn duplicate_nonmanifold_reversed_and_vertex_only_contacts_are_barriers() {
    let m = bent_panel();
    let mut ts = m.triangles().to_vec();
    ts.push(ts[0]);
    let duplicate = RawMesh::new(ts).unwrap();
    let (n, s) = derive_normals(&duplicate, settings(60.), None).unwrap();
    assert_eq!(n, duplicate.flat_normals());
    assert_eq!(s.duplicate_faces, 2);
    assert!(s.ambiguous_edges > 0);
    let contact = RawMesh::new(vec![
        m.triangles()[0],
        [[0., 0., 0.], [-1., 0., 0.25], [0., -1., 0.25]],
    ])
    .unwrap();
    assert_eq!(
        derive_normals(&contact, settings(60.), None).unwrap().0,
        contact.flat_normals()
    );
    let mut reversed = m.triangles().to_vec();
    reversed[1].swap(0, 2);
    assert_eq!(
        derive_normals(&RawMesh::new(reversed).unwrap(), settings(60.), None)
            .unwrap()
            .1
            .joined_edges,
        0
    );
}
#[test]
fn cylinder_has_known_radial_normal_without_any_vertex_update() {
    let mut ts = Vec::new();
    let n = 24;
    let points: Vec<Vec3> = (0..n)
        .map(|i| {
            let a = i as f64 * std::f64::consts::TAU / n as f64;
            [a.cos(), a.sin(), 0.]
        })
        .collect();
    for i in 0..n {
        let a = points[i];
        let b = points[(i + 1) % n];
        let aa = [a[0], a[1], 1.];
        let bb = [b[0], b[1], 1.];
        ts.push([a, b, bb]);
        ts.push([a, bb, aa]);
    }
    let m = RawMesh::new(ts).unwrap();
    let before = m.geometry_digest();
    let (smooth, _) = derive_normals(&m, settings(30.), None).unwrap();
    let error = |ns: &[Vec3]| -> f64 {
        m.triangles()
            .iter()
            .flatten()
            .zip(ns)
            .map(|(p, n)| {
                (p[0] * n[0] + p[1] * n[1])
                    .clamp(-1., 1.)
                    .acos()
                    .to_degrees()
            })
            .sum::<f64>()
            / ns.len() as f64
    };
    assert!((error(&m.flat_normals()) - 7.5).abs() < 1e-6);
    assert!(error(&smooth) < 1e-5);
    assert_eq!(before, m.geometry_digest());
}
#[test]
fn analytic_hole_and_exterior_are_hard_white_with_only_inward_black_contours() {
    // Exact annular rectangular frame: outer[0,8]^2, hole(3,5)^2.
    let mut t = quad(0., 0., 3., 8.);
    t.extend(quad(5., 0., 8., 8.));
    t.extend(quad(3., 0., 5., 3.));
    t.extend(quad(3., 5., 5., 8.));
    let m = RawMesh::new(t).unwrap();
    let colors = vec![[200, 50, 20]; m.len()];
    let a = render(&m, &m.flat_normals(), &colors, cfg()).unwrap();
    assert_eq!(a.submitted_triangles, 8);
    assert_eq!(pixel(&a.rgb, 32, 32), &[255, 255, 255]);
    assert_eq!(pixel(&a.unlit_rgb, 32, 32), &[255, 255, 255]);
    assert_eq!(pixel(&a.rgb, 0, 0), &[255, 255, 255]);
    assert_eq!(pixel(&a.rgb, 8, 32), &[0, 0, 0]);
    assert_eq!(pixel(&a.rgb, 25, 32), &[0, 0, 0]);
    let w = 128;
    for y in 0..64 {
        for x in 0..64 {
            let covered =
                (0..2).any(|dy| (0..2).any(|dx| a.coverage[(2 * y + dy) * w + 2 * x + dx] != 0));
            if !covered {
                assert_eq!(pixel(&a.rgb, x, y), &[255, 255, 255]);
                assert_eq!(pixel(&a.unlit_rgb, x, y), &[255, 255, 255]);
            }
        }
    }
    assert!(a.outline.iter().any(|v| *v != 0));
    for (&outline, &coverage) in a.outline.iter().zip(&a.coverage) {
        assert!(outline == 0 || coverage == 255);
    }
    let b = render(
        &m,
        &m.flat_normals(),
        &colors,
        RenderSettings {
            lighting: Lighting {
                direction: [0.8, 0.1, 1.],
                ..cfg().lighting
            },
            ..cfg()
        },
    )
    .unwrap();
    assert_eq!(a.coverage, b.coverage);
    assert_eq!(a.unlit_rgb, b.unlit_rgb);
    assert_ne!(a.rgb, b.rgb);
}
#[test]
fn intersecting_planes_use_true_depth_instead_of_centroid_painter_order() {
    let a = [
        [[0., 0., 0.], [8., 0., 8.], [8., 8., 8.]],
        [[0., 0., 0.], [8., 8., 8.], [0., 8., 0.]],
    ];
    let b = [
        [[0., 0., 8.], [8., 0., 0.], [8., 8., 0.]],
        [[0., 0., 8.], [8., 8., 0.], [0., 8., 8.]],
    ];
    let m = RawMesh::new(a.into_iter().chain(b).collect()).unwrap();
    let colors = [[200, 30, 20], [200, 30, 20], [20, 40, 200], [20, 40, 200]];
    let r = render(&m, &m.flat_normals(), &colors, cfg()).unwrap();
    assert_eq!(pixel(&r.unlit_rgb, 20, 32), &[20, 40, 200]);
    assert_eq!(pixel(&r.unlit_rgb, 44, 32), &[200, 30, 20]);
    assert_eq!(r.submitted_triangles, 4);
    let reverse = RawMesh::new(m.triangles().iter().rev().copied().collect()).unwrap();
    let r2 = render(
        &reverse,
        &reverse.flat_normals(),
        &colors.into_iter().rev().collect::<Vec<_>>(),
        cfg(),
    )
    .unwrap();
    assert_eq!(r.unlit_rgb, r2.unlit_rgb);
}
#[test]
fn adapter_preserves_zero_missing_mask_contributors_and_assignment_bits() {
    let m = bent_panel();
    let s = surface(&m);
    let mut data = input(&m);
    data.cells.push(cell(None));
    data.face_to_cell[1] = 1;
    let before = data.digest().unwrap();
    let reg =
        RegisteredCellColors::bind(data.clone(), b"neutral original D1 bytes", &before).unwrap();
    let recipe = AppearanceRecipe {
        normals: settings(15.),
        regions: None,
        render: cfg(),
        expected_camera_sha256: cfg().camera.digest(),
    };
    let a = hero_appearance::render(&s, &reg, &recipe).unwrap();
    let b = hero_appearance::render(
        &s,
        &reg,
        &AppearanceRecipe {
            normals: NormalSettings {
                blend: 0.,
                ..settings(15.)
            },
            ..recipe.clone()
        },
    )
    .unwrap();
    assert_eq!(a.receipt["registered_assignment"]["sha256"], before);
    assert_eq!(
        a.receipt["registered_assignment"]["component_hashes_before"],
        a.receipt["registered_assignment"]["component_hashes_after"]
    );
    assert_eq!(a.image.unlit_rgb, b.image.unlit_rgb);
    assert_eq!(a.image.coverage, b.image.coverage);
    let changed_camera = Camera {
        offset: [9., 56.],
        ..camera()
    };
    let drift = AppearanceRecipe {
        render: RenderSettings {
            camera: changed_camera,
            ..cfg()
        },
        ..recipe
    };
    assert!(hero_appearance::render(&s, &reg, &drift).is_err());
    assert_eq!(data.digest().unwrap(), before);
    assert!(a.receipt["fitness_delta"].is_null());
    assert_eq!(a.receipt["fresh_fitness_or_solver_execution"], false);
    assert!(a
        .required_visible_disclosure
        .contains("RGB differs from base palette"));
    let mut changed = data.clone();
    changed.cells[0].index = Some(0.25);
    assert_ne!(changed.digest().unwrap(), before);
    assert!(RegisteredCellColors::bind(changed, b"neutral original D1 bytes", &before).is_err());
    let mut changed = data.clone();
    changed.face_to_cell.swap(0, 1);
    assert!(RegisteredCellColors::bind(changed, b"neutral original D1 bytes", &before).is_err());
    let mut invalid = data.clone();
    invalid.cells[0].source_masks[0] = true;
    assert!(invalid.digest().is_err());
    let mut invalid = data.clone();
    invalid.cells[0].index = Some(f64::NAN);
    assert!(invalid.digest().is_err());
    let mut invalid = data.clone();
    invalid.cells[1].missing_reason = None;
    assert!(invalid.digest().is_err());
    assert!(RegisteredCellColors::bind(data, b"other original bytes", &before).is_err());
}
#[test]
fn strict_stl_grammar_binary_solid_header_and_source_binding_refuse_corruption() {
    let s="solid test\nfacet normal 0 0 1\nouter loop\nvertex 0 0 0\nvertex 1 0 0\nvertex 0 1 0\nendloop\nendfacet\nendsolid test\n";
    assert_eq!(decode_stl(s.as_bytes()).unwrap().len(), 1);
    for bad in [
        s.replace("vertex 0 1 0\n", ""),
        s.replace("vertex 1 0 0", "vertex NaN 0 0"),
        s.replace("endsolid test", ""),
        format!("{s}vertex 9 9 9"),
        s.replace("vertex 0 1 0", "vertex 1 0 0"),
    ] {
        assert!(decode_stl(bad.as_bytes()).is_err());
    }
    let bytes = binary(&bent_panel());
    assert_eq!(decode_stl(&bytes).unwrap().len(), 2);
    assert!(decode_stl(&bytes[..bytes.len() - 1]).is_err());
    assert!(RetainedSurface::from_stl(
        "neutral fixture",
        &bytes,
        "wrong",
        &bent_panel().geometry_digest()
    )
    .is_err());
    assert!(RetainedSurface::from_stl(
        "neutral fixture",
        &bytes,
        &digest(&bytes),
        "wrong geometry"
    )
    .is_err());
}
#[test]
fn camera_sample_budget_color_registration_and_normals_fail_closed() {
    let m = bent_panel();
    let flat = m.flat_normals();
    let colors = vec![[13, 148, 136]; 2];
    assert!(render(
        &m,
        &flat,
        &colors,
        RenderSettings {
            width: 4096,
            height: 4096,
            supersample: 4,
            ..cfg()
        }
    )
    .is_err());
    let mirror = Camera {
        rotation: [[-1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
        ..camera()
    };
    assert!(render(
        &m,
        &flat,
        &colors,
        RenderSettings {
            camera: mirror,
            ..cfg()
        }
    )
    .is_err());
    assert!(render(&m, &flat, &[], cfg()).is_err());
    let mut invalid = flat.clone();
    invalid[0] = [0.; 3];
    assert!(render(&m, &invalid, &colors, cfg()).is_err());
    assert!(render(
        &m,
        &flat,
        &colors,
        RenderSettings {
            lighting: Lighting {
                specular: f64::NAN,
                ..cfg().lighting
            },
            ..cfg()
        }
    )
    .is_err());
}
