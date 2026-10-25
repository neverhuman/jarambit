//! Claude-HZ independent dynamic probes for the S4 re-review blockers (review lane only; never committed).
//!
//!   cargo run --release --example hz_probes -- [N_GENOMES=120] [H=0.4]
//!
//! P1 (N1): hollowing the blade (rho_blade 1.0 -> 0.06, win_amp 0.20, i.e. not vented) must not lower Tier-1 mass
//!          while every strength term and am_build stay identical.
//! P2 (N2): on production builds, no solid voxel for x in (97.5, 101.6] may stand more than 0.5 mm outside the evolved
//!          blade's 2D profile band [min(y_in, y_sp), max(y_in, y_sp)] (the seam fin). x in (94, 97.5] is the designed
//!          guard-to-blade blend and is reported for information only.
//! P3 (reach): straight-line tip-to-guard distance, from (GUARD_X_MM, y at the guard midline) to the tip, <= 85.725 mm.
//! P4 (S5-2): in vented hollow blades, no removed material (port or cavity: inside the envelope, outside the AM solid)
//!          lies within 3.5 mm (in y) of either cutting edge line for x beyond the heel.
//! P5 (S5-5): away from both edges (>= 3.5 mm), the envelope thickness must not step between
//!          neighbouring x columns across x in [86, 94] (the old plunge line at x = 90) beyond the design's own
//!          taper: a design fails if its seam step exceeds max(own reference step + 0.8 mm, 2.0 mm); 0.4 mm voxels.
use jarambit_g3::g3_genome::{sobol_unit_genome, G3Genome, G3_GENES};
use jarambit_g3::g3_qd::evaluate_tier1_sdf_and_plate;
use jarambit_g3::g3_sdf::{build_blade_profile_2d, build_g3_sdf, BLADE_REACH_MAX_MM, GUARD_X_MM};

const COMMIT: &str = "22bb869682e4";

fn set_phys(g: &mut G3Genome, name: &str, v: f64) {
    let i = G3_GENES.iter().position(|s| s.name == name).unwrap_or_else(|| panic!("no gene {name}"));
    g.u[i] = G3_GENES[i].unit_from_phys(v).clamp(0.0, 1.0);
}

fn interp(xs: &[f64], ys: &[f64], x: f64) -> Option<f64> {
    if xs.is_empty() || x < xs[0] || x > xs[xs.len() - 1] {
        return None;
    }
    let k = xs.partition_point(|&v| v <= x).clamp(1, xs.len() - 1);
    let t = (x - xs[k - 1]) / (xs[k] - xs[k - 1]).max(1e-12);
    Some(ys[k - 1] + t * (ys[k] - ys[k - 1]))
}

fn p1() {
    println!("== P1 (N1) hollow-blade mass vs strength, Tier 1 at h = 0.5");
    let names = ["lat_rob_cert_n", "strict_lat_n", "twist_body_sf", "twist_tip_sf", "hm_spine_sf", "am_build"];
    let mut rows = Vec::new();
    for (label, rho, win) in [("solid", 1.0, 0.20), ("hollow, not vented", 0.06, 0.20), ("hollow, vented", 0.06, 0.50)] {
        let mut g = G3Genome::benchmark_reference(COMMIT);
        set_phys(&mut g, "rho_blade", rho);
        set_phys(&mut g, "win_amp", win);
        set_phys(&mut g, "lat_cell", 4.0);
        let r = evaluate_tier1_sdf_and_plate(&g, 0.5);
        let i = &r.inputs;
        let v = [i.lat_rob_cert_n, i.strict_lat_n, i.twist_body_sf, i.twist_tip_sf, i.hm_spine_sf, i.am_build];
        println!("  {label:20} tier {:?} mass {:.2} g score {:.3} | {}", r.tier, i.mass_g, r.score.score_100,
                 names.iter().zip(v.iter()).map(|(n, x)| format!("{n} {x:.4}")).collect::<Vec<_>>().join(", "));
        rows.push((i.mass_g, v));
    }
    let (m0, v0) = rows[0];
    let (m1, v1) = rows[1];
    let strength_moved = v0.iter().zip(v1.iter()).any(|(a, b)| (a - b).abs() > 1e-9 * a.abs().max(1.0));
    let verdict = if m0 - m1 > 1.0 && !strength_moved { "FAIL (N1 present: mass credit, no strength/AM debit)" } else { "PASS" };
    println!("  P1 verdict: {verdict} (mass drop {:.2} g, strength or am_build moved: {strength_moved})", m0 - m1);
}

fn p2_p3(n: usize, h: f64) {
    println!("== P2 (N2) seam fin and P3 reach on {n} scrambled Sobol genomes, h = {h}");
    let mut prot = Vec::new();
    let mut blend = Vec::new();
    let mut reach_bad = 0usize;
    let mut reach = Vec::new();
    for idx in 0..n {
        let g = sobol_unit_genome(idx, 4242, COMMIT);
        let p = g.decode();
        let bp = build_blade_profile_2d(&p);
        // P3: tip to the guard face at the blade's midline height there
        let y_mid = match (interp(&bp.xs, &bp.y_in, GUARD_X_MM.max(bp.xs[0])), interp(&bp.xs, &bp.y_sp, GUARD_X_MM.max(bp.xs[0]))) {
            (Some(a), Some(b)) => 0.5 * (a + b),
            _ => 0.0,
        };
        let r = (bp.tip_xy.0 - GUARD_X_MM).hypot(bp.tip_xy.1 - y_mid);
        if r > BLADE_REACH_MAX_MM + 1e-6 {
            reach_bad += 1;
        }
        reach.push(r);
        // P2: worst out-of-profile solid in (94, 101.6]
        let grid = build_g3_sdf(&g, h, false);
        let mut worst = 0.0_f64;
        let mut worst_blend = 0.0_f64;
        for (i, &x) in grid.x.iter().enumerate() {
            if x <= 94.0 || x > 101.6 {
                continue;
            }
            let (Some(a), Some(b)) = (interp(&bp.xs, &bp.y_in, x), interp(&bp.xs, &bp.y_sp, x)) else { continue };
            let (lo, hi) = (a.min(b), a.max(b));
            for (j, &y) in grid.y.iter().enumerate() {
                let base = (i * grid.ny + j) * grid.nz;
                if (0..grid.nz).any(|k| grid.d[base + k] < 0.0) {
                    let o = (y - hi).max(lo - y);
                    if x > 97.5 { worst = worst.max(o) } else { worst_blend = worst_blend.max(o) }
                }
            }
        }
        prot.push(worst);
        blend.push(worst_blend);
    }
    let mut s = prot.clone();
    s.sort_by(f64::total_cmp);
    let frac = prot.iter().filter(|&&w| w > 0.5).count() as f64 / n as f64;
    println!("  P2 protrusion beyond the blade profile (x 97.5-101.6): >0.5 mm in {:.0} % of designs; median {:.2} mm; p90 {:.2} mm; max {:.2} mm",
             100.0 * frac, s[n / 2], s[(9 * n) / 10], s[n - 1]);
    println!("  P2 verdict: {}", if frac > 0.02 { "FAIL (N2 seam fin present)" } else { "PASS" });
    let mut b = blend.clone();
    b.sort_by(f64::total_cmp);
    println!("  (info) blend zone x 94-97.5 out-of-profile: median {:.2} mm, p90 {:.2} mm", b[n / 2], b[(9 * n) / 10]);
    let mut rs = reach.clone();
    rs.sort_by(f64::total_cmp);
    println!("  P3 tip-to-guard chord: median {:.2} mm, max {:.2} mm, {} of {n} above {BLADE_REACH_MAX_MM}", rs[n / 2], rs[n - 1], reach_bad);
    println!("  P3 verdict: {}", if reach_bad > 0 { "FAIL (reach above the cap)" } else { "PASS" });
}


fn p4(n: usize, h: f64) {
    println!("== P4 (S5-2) vent/cavity vs bevel bands on vented hollow blades, h = {h}");
    let (mut tested, mut bad, mut worst) = (0usize, 0usize, 0usize);
    let mut idx = 0usize;
    while tested < n && idx < 40 * n {
        let mut g = sobol_unit_genome(idx, 777, COMMIT);
        idx += 1;
        set_phys(&mut g, "win_amp", 0.6);
        set_phys(&mut g, "rho_blade", 0.10);
        let p = g.decode();
        let bp = build_blade_profile_2d(&p);
        let grid = build_g3_sdf(&g, h, true);
        let Some(am) = grid.d_am.as_ref() else { continue };
        tested += 1;
        let mut count = 0usize;
        for (i, &x) in grid.x.iter().enumerate() {
            if x < bp.x_heel + 2.0 || x > bp.x_tip - 2.0 { continue; }
            let (Some(a), Some(b)) = (interp(&bp.xs, &bp.y_in, x), interp(&bp.xs, &bp.y_sp, x)) else { continue };
            for (j, &y) in grid.y.iter().enumerate() {
                if (y - a).abs() > 3.5 && (y - b).abs() > 3.5 { continue; }
                let base = (i * grid.ny + j) * grid.nz;
                for k in 0..grid.nz {
                    if grid.d[base + k] < 0.0 && am[base + k] >= 0.0 { count += 1; }
                }
            }
        }
        if count > 0 { bad += 1; worst = worst.max(count); }
    }
    println!("  P4 vented blades tested {tested}: {bad} with removed voxels in a bevel band (worst {worst} voxels)");
    println!("  P4 verdict: {}", if bad > 0 { "FAIL (port or cavity reaches a bevel band)" } else { "PASS" });
}

fn p5(n: usize, h: f64) {
    println!("== P5 (S5-5) medial thickness steps along x, h = {h}");
    let (mut seam, mut refm) = (Vec::new(), Vec::new());
    for idx in 0..n {
        let g = sobol_unit_genome(idx, 5150, COMMIT);
        let p = g.decode();
        let bp = build_blade_profile_2d(&p);
        let grid = build_g3_sdf(&g, h, false);
        let thick = |i: usize, j: usize| -> f64 {
            let base = (i * grid.ny + j) * grid.nz;
            (0..grid.nz).filter(|&k| grid.d[base + k] < 0.0).count() as f64 * grid.h
        };
        let (mut ms, mut mr) = (0.0_f64, 0.0_f64);
        for i in 0..grid.nx.saturating_sub(1) {
            let (x0, x1) = (grid.x[i], grid.x[i + 1]);
            let in_seam = x0 >= 86.0 && x1 <= 94.0;
            let in_ref = x0 >= 94.0 && x1 <= 140.0 && x1 < bp.x_tip - 4.0;
            if !in_seam && !in_ref { continue; }
            for (j, &y) in grid.y.iter().enumerate() {
                let ok = |x: f64| match (interp(&bp.xs, &bp.y_in, x), interp(&bp.xs, &bp.y_sp, x)) {
                    (Some(a), Some(b)) => y > a.min(b) + 3.5 && y < a.max(b) - 3.5,
                    _ => false,
                };
                if !(ok(x0) && ok(x1)) { continue; }
                let (t0, t1) = (thick(i, j), thick(i + 1, j));
                if t0 <= 0.0 || t1 <= 0.0 { continue; }
                let d = (t1 - t0).abs();
                if in_seam { ms = ms.max(d) } else { mr = mr.max(d) }
            }
        }
        if ms > (mr + 0.8).max(2.0) {
            // locate the worst seam step for the report
            let (mut wx, mut wy, mut wd) = (0.0, 0.0, 0.0_f64);
            for i in 0..grid.nx.saturating_sub(1) {
                let (x0, x1) = (grid.x[i], grid.x[i + 1]);
                if !(x0 >= 86.0 && x1 <= 94.0) { continue; }
                for (j, &y) in grid.y.iter().enumerate() {
                    let ok = |x: f64| match (interp(&bp.xs, &bp.y_in, x), interp(&bp.xs, &bp.y_sp, x)) {
                        (Some(a), Some(b)) => y > a.min(b) + 3.5 && y < a.max(b) - 3.5, _ => false };
                    if !(ok(x0) && ok(x1)) { continue; }
                    let (t0, t1) = (thick(i, j), thick(i + 1, j));
                    if t0 > 0.0 && t1 > 0.0 && (t1 - t0).abs() > wd { wd = (t1 - t0).abs(); wx = x0; wy = y; }
                }
            }
            println!("  P5 outlier: genome sobol({idx}, 5150) step {wd:.2} mm at x {wx:.1}-{:.1}, y {wy:.1} (own reference max {mr:.2}); x_heel {:.1}, t_core {:.2}, t_guard {:.2}",
                     wx + grid.h, bp.x_heel, p["t_core"], p["t_guard"]);
        }
        seam.push(ms); refm.push(mr);
    }
    let mut s = seam.clone(); s.sort_by(f64::total_cmp);
    let mut r = refm.clone(); r.sort_by(f64::total_cmp);
    let k = s.len();
    println!("  P5 max column step x 86-94: median {:.2} mm, p90 {:.2}, max {:.2} | reference x 94-140: median {:.2}, max {:.2}",
             s[k / 2], s[(9 * k) / 10], s[k - 1], r[k / 2], r[k - 1]);
    let outliers = seam.iter().zip(refm.iter()).filter(|(m, r)| **m > (**r + 0.8).max(2.0)).count();
    println!("  P5 designs whose seam step exceeds max(own reference + 0.8 mm, 2.0 mm): {outliers} of {k}");
    println!("  P5 verdict: {}", if outliers > 0 { "FAIL (a real step at the old plunge line)" } else { "PASS (steps within voxel quantisation of the blade's own taper)" });
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let n: usize = a.get(1).and_then(|v| v.parse().ok()).unwrap_or(120);
    let h: f64 = a.get(2).and_then(|v| v.parse().ok()).unwrap_or(0.4);
    if std::env::var("P5_ONLY").is_err() { p1(); p2_p3(n, h); }
    if std::env::var("P5_ONLY").is_err() { p4(30, 0.4); }
    p5(120, 0.4);
}
