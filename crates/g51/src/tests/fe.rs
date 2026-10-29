//! Controls of the voxel FE against closed-form answers on prismatic bars, and its fail-closed gates.
use crate::fe::voxel::{solve_wrench, Face, Grid, Material, Model, Options, Problem, P3};

const E: f64 = 113_800.0;

fn bar(l: f64, a: f64, b: f64, h: f64) -> Model {
    let n = [(l / h).round() as usize, (a / h).round() as usize, (b / h).round() as usize];
    let grid = Grid { n, h, origin: [0.0, -a / 2.0, -b / 2.0] };
    let phase = vec![1u8; grid.elements()];
    Model { grid, phase, materials: vec![Material { youngs_mpa: E, poisson: 0.31 }] }
}

/// The high-x end faces, and a problem clamping the low-x node plane with the reference at the end's centre.
fn end_loaded(m: &Model) -> Problem {
    let g = &m.grid;
    let i = g.n[0] - 1;
    let loaded: Vec<Face> = (0..g.elements())
        .filter(|&e| g.element_index(e)[0] == i)
        .map(|element| Face { element, axis: 0, side: true })
        .collect();
    let x_lo = g.origin[0];
    let h = g.h;
    Problem::new(g, |q| q[0] <= x_lo + 0.25 * h, loaded, [g.origin[0] + h * g.n[0] as f64, 0.0, 0.0])
}

fn opts() -> Options {
    Options { relative_tolerance: 1e-9, max_iterations: 20_000, threads: 4, keep_displacements: false }
}

/// Uniform tension: away from the clamp, sigma_xx = F / A, so von Mises = F / A.
#[test]
fn tension_bar_carries_f_over_a() {
    let (l, a, b, h) = (40.0, 8.0, 6.0, 0.5);
    let m = bar(l, a, b, h);
    let f = 1000.0;
    let s = solve_wrench(&m, &end_loaded(&m), [f, 0.0, 0.0, 0.0, 0.0, 0.0], &opts()).expect("solve");
    let mid = |q: P3| (15.0..25.0).contains(&q[0]);
    let r = s.reading(mid, None).expect("reading");
    let exact = f / (a * b);
    assert!((r.max_mpa / exact - 1.0).abs() < 0.01, "max {} vs {exact}", r.max_mpa);
    assert!((r.q99_mpa / exact - 1.0).abs() < 0.01, "q99 {} vs {exact}", r.q99_mpa);
}

/// Cantilever bending under an end load P in y: at the outer element centres of a section x (y = a/2 - h/2),
/// sigma_xx = P (L - x) y / I with I = b a^3 / 12. Read in a thin slab at mid-span, within 3 %.
#[test]
fn cantilever_bending_matches_beam_theory() {
    let (l, a, b, h) = (60.0, 6.0, 4.0, 0.25);
    let m = bar(l, a, b, h);
    let p = 50.0;
    let s = solve_wrench(&m, &end_loaded(&m), [0.0, p, 0.0, 0.0, 0.0, 0.0], &opts()).expect("solve");
    let x0 = 30.0;
    let slab = |q: P3| (q[0] - (x0 + 0.5 * h)).abs() < 0.5 * h;
    let r = s.reading(slab, None).expect("reading");
    let i = b * a.powi(3) / 12.0;
    let exact = p * (l - (x0 + 0.5 * h)) * (0.5 * a - 0.5 * h) / i;
    assert!((r.max_mpa / exact - 1.0).abs() < 0.03, "max {} vs beam {exact}", r.max_mpa);
}

/// Saint-Venant torsion of a 1.5:1 rectangle: tau_max = T / (0.231 a b^2) at the middle of the long sides; the
/// element centres sit h/2 inside the surface, so the read is a little lower (within 6 %, and never above).
#[test]
fn rectangular_bar_torsion_matches_saint_venant() {
    let (l, a, b, h) = (60.0, 27.0, 18.0, 1.0);
    let m = bar(l, a, b, h);
    let t = 16_200.0;
    let s = solve_wrench(&m, &end_loaded(&m), [0.0, 0.0, 0.0, t, 0.0, 0.0], &opts()).expect("solve");
    let mid = |q: P3| (15.0..45.0).contains(&q[0]);
    let r = s.reading(mid, None).expect("reading");
    let tau = t / (0.231 * a * b * b);
    let vm = tau * 3f64.sqrt();
    assert!(r.max_mpa < 1.02 * vm && r.max_mpa > 0.94 * vm, "max vm {} vs Saint-Venant {vm}", r.max_mpa);
}

/// Bit-identical results for any thread count.
#[test]
fn solve_is_deterministic_across_thread_counts() {
    let m = bar(30.0, 6.0, 4.0, 0.5);
    let pr = end_loaded(&m);
    let w = [10.0, 20.0, -5.0, 100.0, 0.0, 30.0];
    let a = solve_wrench(&m, &pr, w, &Options { threads: 1, ..opts() }).expect("1 thread");
    let b = solve_wrench(&m, &pr, w, &Options { threads: 5, ..opts() }).expect("5 threads");
    let bits = |s: &crate::fe::voxel::Solution| s.stress.iter().flat_map(|v| v.map(f64::to_bits)).collect::<Vec<_>>();
    assert_eq!(bits(&a), bits(&b));
}

/// The gates: a loaded face on a floating piece, load on a fixed node, and unsupported material in the read region
/// are errors, never a lower or higher stress.
#[test]
fn fe_fails_closed() {
    let mut m = bar(20.0, 4.0, 4.0, 0.5);
    let g = m.grid.clone();
    // a floating 1-voxel-thick slab cut off at x 10..10.5 splits the bar: the far half floats
    for e in 0..g.elements() {
        if g.element_index(e)[0] == 20 {
            m.phase[e] = 0;
        }
    }
    let err = solve_wrench(&m, &end_loaded(&m), [100.0, 0.0, 0.0, 0.0, 0.0, 0.0], &opts()).unwrap_err();
    assert!(err.contains("no path to a support"), "{err}");

    // loaded faces on the clamped plane: part of the load lands on fixed nodes
    let m = bar(20.0, 4.0, 4.0, 0.5);
    let g = m.grid.clone();
    let loaded: Vec<Face> = (0..g.elements()).filter(|&e| g.element_index(e)[0] == 0).map(|element| Face { element, axis: 0, side: false }).collect();
    let pr = Problem::new(&g, |q| q[0] <= 0.1, loaded, [0.0, 0.0, 0.0]);
    let err = solve_wrench(&m, &pr, [100.0, 0.0, 0.0, 0.0, 0.0, 0.0], &opts()).unwrap_err();
    assert!(err.contains("falls on fixed nodes"), "{err}");

    // a floating voxel inside the read region: the reading is refused
    let mut m = bar(20.0, 4.0, 4.0, 0.5);
    let g = m.grid.clone();
    let lone = g.element([20, 0, 0]);
    for e in [g.element([19, 0, 0]), g.element([21, 0, 0]), g.element([20, 1, 0]), g.element([20, 0, 1])] {
        m.phase[e] = 0;
    }
    let s = solve_wrench(&m, &end_loaded(&m), [100.0, 0.0, 0.0, 0.0, 0.0, 0.0], &opts()).expect("solve");
    assert_eq!(s.unsupported, vec![lone]);
    assert!(s.reading(|q| (8.0..12.0).contains(&q[0]), None).is_err());
    assert!(s.reading(|q| (2.0..6.0).contains(&q[0]), None).is_ok());
}
