//! Design features from the CAD kernel (`jarambit_crescent`) and test outputs from the FEM screen.
//!
//! Families written into records:
//! * `g.*`    exact genes (+ material flag) — round-trip identical to `Genome::csv_row`
//! * `c.*`    CAD / phenotype measurements: blade lengths, ring, mass, planform and thickness-field
//!            statistics, hole ligaments, convex hull, mass moments, section properties at 12
//!            stations along the part, 12 polar sectors around the ring, and a planform distance
//!            transform (narrowest ligament anywhere / near the tip / in the body / at the ring)
//! * `a.*`    C.6 aesthetic metrics + the 32-bin macro-silhouette curvature histogram
//! * `gate.*` hard-gate outcomes, holder fit, ring-bearing geometry (no FEM)
//! * `desc.*` the 16-D QD novelty descriptor
//! * `s1.*`   Stage-1 FEM screen outputs (the regression targets of the search loop)
//!
//! None of the `c/a/gate/desc` features read FEM results, so they are legitimate predictors for
//! every `s1/s2/phys` target. Changing this file changes feature *values* in new records only;
//! `FEATURE_SCHEMA` is stamped on every row so tables can be filtered by schema if ever needed.
use super::gfeat;
use super::records::{Row, HASH_COL};
use jarambit_crescent::{
    aesthetic::{self, Aes},
    fem::{self, Fem},
    gates,
    genome::Genome,
    geom::{dist, min_ring_dist, signed_area, P2},
    pheno::{self, Pheno},
    qd,
};

pub const FEATURE_SCHEMA: &str = "jarambit.regression.features.v1";
pub const N_STATIONS: usize = 12;
pub const N_SECTORS: usize = 12;
const REMOVED: [&str; 7] = ["relief", "lattice", "fuller", "comet_flute", "texture", "iso_pocket", "ring_flute"];

/// Genome hash (`meta.genome_hash`) of a genome.
pub fn genome_hash(g: &Genome) -> Option<String> {
    let mut row = Row::new();
    genome_cells(g, &mut row);
    row.get(HASH_COL).map(str::to_string)
}

/// Exact gene columns (`g.*`) + genome hash for one genome.
pub fn genome_cells(g: &Genome, row: &mut Row) {
    let cols = gfeat::gene_columns();
    for (i, v) in g.v.iter().enumerate().take(cols.len() - 1) {
        row.exact(&cols[i], *v);
    }
    row.exact(gfeat::MATERIAL_COL, gfeat::material_flag(g.mat.name()));
    row.stamp_hash();
}

fn polygon_perimeter(p: &[P2]) -> f64 {
    if p.len() < 2 {
        return 0.0;
    }
    (0..p.len()).map(|i| dist(p[i], p[(i + 1) % p.len()])).sum()
}

fn convex_hull_area(pts: &[P2]) -> f64 {
    let mut v: Vec<P2> = pts.to_vec();
    v.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
    v.dedup();
    if v.len() < 3 {
        return 0.0;
    }
    let cross = |o: P2, a: P2, b: P2| (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0);
    let mut hull: Vec<P2> = Vec::with_capacity(v.len() * 2);
    for pass in 0..2 {
        let start = hull.len();
        let iter: Box<dyn Iterator<Item = &P2>> = if pass == 0 { Box::new(v.iter()) } else { Box::new(v.iter().rev()) };
        for &q in iter {
            while hull.len() >= start + 2 && cross(hull[hull.len() - 2], hull[hull.len() - 1], q) <= 0.0 {
                hull.pop();
            }
            hull.push(q);
        }
        hull.pop();
    }
    signed_area(&hull).abs()
}

fn quantile(sorted: &[f64], q: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
    let pos = q * (sorted.len() - 1) as f64;
    let (i, f) = (pos.floor() as usize, pos.fract());
    if i + 1 < sorted.len() {
        sorted[i] * (1.0 - f) + sorted[i + 1] * f
    } else {
        sorted[i]
    }
}

/// Chamfer (3-4) distance transform of the material mask, in mm (distance to the nearest
/// non-material cell: outline exterior or any through-hole).
fn distance_transform(mask: &[bool], nx: usize, ny: usize, cell: f64) -> Vec<f64> {
    let inf = f64::INFINITY;
    let mut d: Vec<f64> = mask.iter().map(|&m| if m { inf } else { 0.0 }).collect();
    let (a, b) = (1.0, std::f64::consts::SQRT_2);
    let at = |d: &Vec<f64>, i: i64, j: i64| -> f64 {
        if i < 0 || j < 0 || i >= nx as i64 || j >= ny as i64 {
            0.0
        } else {
            d[j as usize * nx + i as usize]
        }
    };
    for j in 0..ny as i64 {
        for i in 0..nx as i64 {
            let k = j as usize * nx + i as usize;
            if d[k] == 0.0 {
                continue;
            }
            let v = (at(&d, i - 1, j) + a).min(at(&d, i, j - 1) + a).min(at(&d, i - 1, j - 1) + b).min(at(&d, i + 1, j - 1) + b);
            d[k] = d[k].min(v);
        }
    }
    for j in (0..ny as i64).rev() {
        for i in (0..nx as i64).rev() {
            let k = j as usize * nx + i as usize;
            if d[k] == 0.0 {
                continue;
            }
            let v = (at(&d, i + 1, j) + a).min(at(&d, i, j + 1) + a).min(at(&d, i + 1, j + 1) + b).min(at(&d, i - 1, j + 1) + b);
            d[k] = d[k].min(v);
        }
    }
    d.iter().map(|v| v * cell).collect()
}

/// CAD feature cells (`c.*`, `a.*`, `gate.*`, `desc.*`) for an already-built phenotype.
pub fn design_cells(g: &Genome, p: &Pheno, aes: &Aes, rejects: &[String], desc: &[f64], row: &mut Row) {
    let mut c = |k: &str, v: f64| {
        row.num(&format!("c.{k}"), v);
    };
    let b = &p.blade;
    for (k, v) in [
        ("mass_g", p.mass_g),
        ("blank_g", p.blank_g),
        ("area_mm2", p.area_mm2),
        ("removed_frac", if p.blank_g > 0.0 { 1.0 - p.mass_g / p.blank_g } else { f64::NAN }),
        ("t", p.t),
        ("t_joint", p.t_joint),
        ("surface_x_max", p.surface_x_max),
        ("grip_dx", p.dx),
        ("grip_len", p.grip_len),
        ("ring_od", p.ring_od),
        ("ring_id", p.ring_id),
        ("ring_wall", (p.ring_od - p.ring_id) / 2.0),
        ("teeth_in", p.teeth_in as f64),
        ("teeth_sp", p.teeth_sp as f64),
        ("ripple_in", p.ripple_in),
        ("ripple_sp", p.ripple_sp),
        ("web_clamped_cells", p.web_clamped as f64),
        ("min_web", p.min_web),
        ("n_win", p.n_win as f64),
        ("win_area", p.win_area),
        ("n_perf", p.n_perf as f64),
        ("perf_area", p.perf_area),
        ("land_full_mm2", p.land_full_mm2),
        ("pad_flat", p.pad_flat as u8 as f64),
        ("groove_w", p.groove_w),
        ("groove_area", p.groove_area),
        ("pocket_area", p.pocket_area),
        ("p1", p.p1 as u8 as f64),
        ("rs_v2", p.rs_v2 as u8 as f64),
        ("pheno_fail_count", p.fails.len() as f64),
        ("blade_tip_to_heel", b.tip_to_heel),
        ("blade_horizontal_reach", b.horizontal_reach),
        ("blade_arc_in", b.arc_in),
        ("blade_arc_sp", b.arc_sp),
        ("blade_sharpened_arc", b.sharpened_arc),
        ("blade_spine_offset_used", b.spine_offset_used),
        ("blade_hook_drop", b.hook_drop),
        ("tip_x", p.tip.0),
        ("tip_y", p.tip.1),
        ("heel_x", p.heel.0),
        ("heel_y", p.heel.1),
        ("tip_radius", p.tip.0.hypot(p.tip.1)),
        ("tip_angle_deg", p.tip.1.atan2(p.tip.0).to_degrees()),
        ("edge_in_len", polygon_perimeter(&p.edge_in) - dist(*p.edge_in.first().unwrap_or(&(0.0, 0.0)), *p.edge_in.last().unwrap_or(&(0.0, 0.0)))),
        ("outline_perimeter", polygon_perimeter(&p.outline)),
        ("macro_perimeter", polygon_perimeter(&p.macro_outline)),
        ("outline_vertices", p.outline.len() as f64),
    ] {
        c(k, v);
    }
    for (k, v) in REMOVED.iter().zip(p.removed.iter()) {
        c(&format!("removed_mm3_{k}"), *v);
    }
    let removed_total: f64 = p.removed.iter().sum();
    c("removed_mm3_total", removed_total);

    // planform envelope
    let (mut lo, mut hi) = ((f64::MAX, f64::MAX), (f64::MIN, f64::MIN));
    for q in &p.outline {
        lo = (lo.0.min(q.0), lo.1.min(q.1));
        hi = (hi.0.max(q.0), hi.1.max(q.1));
    }
    c("bbox_w", hi.0 - lo.0);
    c("bbox_h", hi.1 - lo.1);
    let hull = convex_hull_area(&p.outline);
    c("hull_area", hull);
    c("solidity", if hull > 0.0 { signed_area(&p.outline).abs() / hull } else { f64::NAN });
    c("macro_area", signed_area(&p.macro_outline).abs());

    // holes and ligaments (fixed column order: first hole of each known kind)
    let mut holes_area = 0.0;
    let mut lig_min = f64::INFINITY;
    for (h, name) in p.holes.iter().zip(&p.hole_names) {
        if *name != "ring" {
            holes_area += signed_area(h).abs();
            lig_min = lig_min.min(min_ring_dist(h, &p.outline));
        }
    }
    for name in ["ring", "comet", "eye", "cairn", "window", "perf"] {
        let n_kind = p.hole_names.iter().filter(|n| **n == name).count();
        match p.holes.iter().zip(&p.hole_names).find(|(_, n)| **n == name) {
            Some((h, _)) => {
                c(&format!("hole_{name}_area"), signed_area(h).abs());
                c(&format!("hole_{name}_lig_mm"), min_ring_dist(h, &p.outline));
            }
            None => {
                c(&format!("hole_{name}_area"), 0.0);
                c(&format!("hole_{name}_lig_mm"), f64::NAN);
            }
        }
        c(&format!("hole_{name}_count"), n_kind as f64);
    }
    c("hole_count", p.holes.len() as f64);
    c("holes_area_nonring", holes_area);
    c("lig_min_hole_outline_mm", if lig_min.is_finite() { lig_min } else { f64::NAN });
    let mut hh = f64::INFINITY;
    for i in 0..p.holes.len() {
        for j in i + 1..p.holes.len() {
            hh = hh.min(min_ring_dist(&p.holes[i], &p.holes[j]));
        }
    }
    c("lig_min_hole_hole_mm", if hh.is_finite() { hh } else { f64::NAN });

    // thickness raster: statistics, mass moments, sections, sectors, distance transform
    let f = &p.field;
    let (nx, ny, cell) = (f.nx, f.ny, f.g);
    let rho = p.mass_g / f.th.iter().filter(|v| !v.is_nan()).map(|&v| v as f64 * cell * cell).sum::<f64>().max(1e-12); // g / mm^3
    let mut th: Vec<f64> = Vec::new();
    let (mut m, mut mx, mut my, mut ax, mut ay, mut an) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let mut mask = vec![false; nx * ny];
    for j in 0..ny {
        for i in 0..nx {
            let v = f.th[j * nx + i];
            if v.is_nan() {
                continue;
            }
            let v = v as f64;
            mask[j * nx + i] = true;
            let (x, y) = (f.x0 + (i as f64 + 0.5) * cell, f.y0 + (j as f64 + 0.5) * cell);
            th.push(v);
            let dm = v * cell * cell * rho;
            m += dm;
            mx += dm * x;
            my += dm * y;
            ax += x;
            ay += y;
            an += 1.0;
        }
    }
    let (cx, cy) = if m > 0.0 { (mx / m, my / m) } else { (f64::NAN, f64::NAN) };
    let (pax, pay) = if an > 0.0 { (ax / an, ay / an) } else { (f64::NAN, f64::NAN) };
    let (mut jxx, mut jyy, mut jxy, mut j0) = (0.0, 0.0, 0.0, 0.0);
    for j in 0..ny {
        for i in 0..nx {
            let v = f.th[j * nx + i];
            if v.is_nan() {
                continue;
            }
            let (x, y) = (f.x0 + (i as f64 + 0.5) * cell, f.y0 + (j as f64 + 0.5) * cell);
            let dm = v as f64 * cell * cell * rho;
            jxx += dm * (y - cy).powi(2);
            jyy += dm * (x - cx).powi(2);
            jxy += dm * (x - cx) * (y - cy);
            j0 += dm * (x * x + y * y);
        }
    }
    let tr = 0.5 * (jxx + jyy);
    let det = ((0.5 * (jxx - jyy)).powi(2) + jxy * jxy).sqrt();
    c("com_x", cx);
    c("com_y", cy);
    c("com_radius", cx.hypot(cy));
    c("planform_centroid_x", pax);
    c("planform_centroid_y", pay);
    c("inertia_com_gmm2", jxx + jyy);
    c("inertia_ring_gmm2", j0);
    c("inertia_principal_max", tr + det);
    c("inertia_principal_min", tr - det);
    c("inertia_principal_angle_deg", 0.5 * (2.0 * jxy).atan2(jyy - jxx).to_degrees());
    c("radius_gyration_com", if m > 0.0 { ((jxx + jyy) / m).sqrt() } else { f64::NAN });
    let mut sorted = th.clone();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let n = sorted.len().max(1) as f64;
    let mean = sorted.iter().sum::<f64>() / n;
    let sd = (sorted.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n).sqrt();
    c("th_mean", mean);
    c("th_sd", sd);
    c("th_cv", if mean > 0.0 { sd / mean } else { f64::NAN });
    for (k, q) in [("min", 0.0), ("p05", 0.05), ("p25", 0.25), ("p50", 0.5), ("p75", 0.75), ("p95", 0.95), ("max", 1.0)] {
        c(&format!("th_{k}"), quantile(&sorted, q));
    }
    c("th_frac_full", sorted.iter().filter(|v| **v >= p.t - 0.05).count() as f64 / n);
    c("th_frac_below_half_t", sorted.iter().filter(|v| **v < 0.5 * p.t).count() as f64 / n);
    c("th_frac_below_1mm", sorted.iter().filter(|v| **v < 1.0).count() as f64 / n);

    // stations along x (ring centre at the origin, blade toward +x)
    let cols_with_material: Vec<usize> = (0..nx).filter(|&i| (0..ny).any(|j| mask[j * nx + i])).collect();
    let (i_lo, i_hi) = (*cols_with_material.first().unwrap_or(&0), *cols_with_material.last().unwrap_or(&0));
    let mut min_ioop_blade = f64::INFINITY;
    for s in 0..N_STATIONS {
        let i = i_lo + ((s as f64 + 0.5) / N_STATIONS as f64 * (i_hi - i_lo) as f64).round() as usize;
        let (mut area, mut ioop, mut cnt, mut sy, mut runs, mut prev) = (0.0, 0.0, 0usize, 0.0, 0usize, false);
        let mut cells: Vec<(f64, f64)> = Vec::new();
        for j in 0..ny {
            let k = j * nx + i.min(nx.saturating_sub(1));
            let on = mask.get(k).copied().unwrap_or(false);
            if on {
                let t = f.th[k] as f64;
                let y = f.y0 + (j as f64 + 0.5) * cell;
                area += t * cell;
                ioop += cell * t.powi(3) / 12.0;
                cnt += 1;
                sy += t * y;
                cells.push((y, t));
                if !prev {
                    runs += 1;
                }
            }
            prev = on;
        }
        let ybar = if area > 0.0 { sy * cell / area } else { f64::NAN };
        let iip: f64 = cells.iter().map(|(y, t)| t * cell * (y - ybar).powi(2) + t * cell.powi(3) / 12.0).sum();
        let x = f.x0 + (i as f64 + 0.5) * cell;
        c(&format!("sec{s:02}_x"), x);
        c(&format!("sec{s:02}_area"), area);
        c(&format!("sec{s:02}_ioop"), ioop);
        c(&format!("sec{s:02}_iip"), iip);
        c(&format!("sec{s:02}_width"), cnt as f64 * cell);
        c(&format!("sec{s:02}_runs"), runs as f64);
        if x > p.heel.0 && ioop > 0.0 {
            min_ioop_blade = min_ioop_blade.min(ioop);
        }
    }
    c("sec_min_ioop_blade", if min_ioop_blade.is_finite() { min_ioop_blade } else { f64::NAN });

    // polar sectors around the ring centre
    let mut sec_area = [0.0f64; N_SECTORS];
    let mut sec_mass = [0.0f64; N_SECTORS];
    let mut sec_rmax = [0.0f64; N_SECTORS];
    for j in 0..ny {
        for i in 0..nx {
            if !mask[j * nx + i] {
                continue;
            }
            let (x, y) = (f.x0 + (i as f64 + 0.5) * cell, f.y0 + (j as f64 + 0.5) * cell);
            let a = y.atan2(x).rem_euclid(std::f64::consts::TAU);
            let s = ((a / std::f64::consts::TAU) * N_SECTORS as f64) as usize % N_SECTORS;
            sec_area[s] += cell * cell;
            sec_mass[s] += f.th[j * nx + i] as f64 * cell * cell * rho;
            sec_rmax[s] = sec_rmax[s].max(x.hypot(y));
        }
    }
    for s in 0..N_SECTORS {
        c(&format!("sector{s:02}_area"), sec_area[s]);
        c(&format!("sector{s:02}_mass_frac"), if m > 0.0 { sec_mass[s] / m } else { f64::NAN });
        c(&format!("sector{s:02}_mean_t"), if sec_area[s] > 0.0 { sec_mass[s] / rho / sec_area[s] } else { 0.0 });
        c(&format!("sector{s:02}_rmax"), sec_rmax[s]);
    }

    // distance transform: inscribed radius and the narrowest ligament (ridge) per zone
    let dt = distance_transform(&mask, nx, ny, cell);
    let (mut dmax, mut dsum, mut dn) = (0.0f64, 0.0, 0.0);
    let mut ridge = [f64::INFINITY; 4]; // all, tip zone (8..40 mm from tip), body, ring zone
    let mut ridge_wt3 = f64::INFINITY;
    let r_ring = p.ring_od / 2.0 + 3.0;
    for j in 1..ny.saturating_sub(1) {
        for i in 1..nx.saturating_sub(1) {
            let k = j * nx + i;
            if !mask[k] {
                continue;
            }
            let v = dt[k];
            dmax = dmax.max(v);
            dsum += v;
            dn += 1.0;
            let is_ridge = [(-1i64, 0i64), (1, 0), (0, -1), (0, 1), (-1, -1), (1, 1), (-1, 1), (1, -1)]
                .iter()
                .all(|(di, dj)| dt[(j as i64 + dj) as usize * nx + (i as i64 + di) as usize] <= v + 1e-12);
            if !is_ridge || v < 2.0 * cell {
                continue;
            }
            let q = (f.x0 + (i as f64 + 0.5) * cell, f.y0 + (j as f64 + 0.5) * cell);
            let dtip = dist(q, p.tip);
            if dtip < 8.0 {
                continue;
            }
            ridge[0] = ridge[0].min(v);
            if dtip < 40.0 {
                ridge[1] = ridge[1].min(v);
            } else if q.0.hypot(q.1) <= r_ring {
                ridge[3] = ridge[3].min(v);
            } else {
                ridge[2] = ridge[2].min(v);
            }
            ridge_wt3 = ridge_wt3.min(2.0 * v * (f.th[k] as f64).powi(3));
        }
    }
    let fin = |x: f64| if x.is_finite() { x } else { f64::NAN };
    c("dt_max_inscribed_mm", dmax);
    c("dt_mean_mm", if dn > 0.0 { dsum / dn } else { f64::NAN });
    c("ridge_min_halfwidth_mm", fin(ridge[0]));
    c("ridge_min_tipzone_mm", fin(ridge[1]));
    c("ridge_min_body_mm", fin(ridge[2]));
    c("ridge_min_ringzone_mm", fin(ridge[3]));
    c("ridge_min_w_t3", fin(ridge_wt3));

    // aesthetics
    let mut a = |k: &str, v: f64| {
        row.num(&format!("a.{k}"), v);
    };
    for (k, v) in [
        ("hausdorff", aes.hausdorff),
        ("emd", aes.emd),
        ("straight_mm", aes.straight_mm),
        ("min_concave_r", aes.min_concave_r),
        ("spine_convex", aes.spine_convex),
        ("inside_concave", aes.inside_concave),
        ("seat_depth", aes.seat_depth),
        ("lobe_prom", aes.lobe_prom),
        ("hook_drop", aes.hook_drop),
        ("score", aes.score),
        ("pass", aes.pass as u8 as f64),
        ("fail_count", aes.fails.len() as f64),
    ] {
        a(k, v);
    }
    let (hist, _, _) = aesthetic::shape_stats(&p.macro_outline, p.tip);
    for (i, h) in hist.iter().enumerate() {
        a(&format!("khist{i:02}"), *h);
    }

    // gates (geometry only)
    let (fit, hl, hw, hang) = gates::holder_fit(&p.outline);
    let (bw, band) = fem::ring_bearing(p);
    let (bore_r, bore_bearing) = pheno::bore_dimensions(p);
    for (k, v) in [
        ("reject_count", rejects.len() as f64),
        ("pass", rejects.is_empty() as u8 as f64),
        ("holder_fit", fit as u8 as f64),
        ("holder_len", hl),
        ("holder_w", hw),
        ("holder_angle_deg", hang),
        ("ring_bearing_w_mm", bw),
        ("ring_band_mm", band),
        ("bore_roll_radius_mm", bore_r),
        ("bore_bearing_len_mm", bore_bearing),
        ("edge_th_rs_v2", g.rs_v2() as u8 as f64),
    ] {
        row.num(&format!("gate.{k}"), v);
    }
    for kind in ["Blade", "Grip", "Ring", "Ligament", "Disconnected", "Holder", "Stock", "NotDouble", "Unsafe"] {
        let n = rejects.iter().filter(|r| r.starts_with(kind)).count();
        row.num(&format!("gate.reject_{}", kind.to_lowercase()), n as f64);
    }
    for (i, d) in desc.iter().enumerate() {
        row.num(&format!("desc.{i:02}"), *d);
    }
}

/// Stage-1 FEM screen outputs as `s1.*` targets (plus mesh size as provenance).
pub fn screen_cells(fm: &Fem, mass_g: f64, row: &mut Row) {
    for (k, v) in [
        ("lat20", fm.lat20),
        ("pry20", fm.pry20),
        ("twist20_nmm", fm.twist20_nmm),
        ("ring20", fm.ring20),
        ("lat_rob", fm.lat_rob),
        ("ring_rob", fm.ring_rob),
        ("sf_hiker", fm.sf_hiker),
        ("sf_hunter", fm.sf_hunter),
        ("sf_ring", fm.sf_ring),
        ("couple20_nmm", fm.couple20_nmm),
        ("twist_kc", fm.twist_kc),
        ("twist_b_mm", fm.twist_b_mm),
        ("twist_t_mm", fm.twist_t_mm),
        ("twist_joint_nmm", fm.twist_joint_nmm),
        ("twist_sf", fm.twist_sf),
        ("seq_ratio", fm.seq_ratio),
        ("twist_tipdist", fm.twist_tipdist),
        ("twist_body_joint_nmm", fm.twist_body_joint_nmm),
        ("twist_hand_joint_nmm", fm.twist_hand_joint_nmm),
        ("twist_body_kc", fm.twist_body_kc),
        ("twist_body_sf", fm.twist_body_sf),
        ("twist_tipzone_kc", fm.twist_tipzone_kc),
        ("twist_tipzone_joint_nmm", fm.twist_tipzone_joint_nmm),
        ("twist_tipzone_sf", fm.twist_tipzone_sf),
        ("hm_twist_nmm", fm.hm_twist_nmm),
        ("hm_tip_inplane_n", fm.hm_tip_inplane_n),
        ("hm_spine_n", fm.hm_spine_n),
        ("hm_ringback_n", fm.hm_ringback_n),
        ("hm_twist_sf", fm.hm_twist_sf),
        ("hm_tip_inplane_sf", fm.hm_tip_inplane_sf),
        ("hm_spine_sf", fm.hm_spine_sf),
        ("hm_ringback_sf", fm.hm_ringback_sf),
        ("fat_sf", fm.fat_sf),
        ("lat_at_x", fm.lat_at.0),
        ("lat_at_y", fm.lat_at.1),
        ("ring_at_x", fm.ring_at.0),
        ("ring_at_y", fm.ring_at.1),
        ("twist_at_x", fm.twist_at.0),
        ("twist_at_y", fm.twist_at.1),
        ("floors_ok", fm.floors_ok as u8 as f64),
        ("spg", if mass_g > 0.0 { fm.lat_rob / mass_g } else { f64::NAN }),
    ] {
        row.num(&format!("s1.{k}"), v);
    }
    row.text("meta.fem_nodes", &fm.nodes.to_string());
    row.text("meta.fem_tris", &fm.tris.to_string());
    row.text("meta.seq_case", fm.seq_case);
}

/// Full record row for a genome whose phenotype and QD evaluation are already available.
pub fn record_from_eval(g: &Genome, p: &Pheno, r: &qd::Rec) -> Row {
    let mut row = Row::new();
    genome_cells(g, &mut row);
    row.text("meta.feature_schema", FEATURE_SCHEMA);
    design_cells(g, p, &r.aes, &r.rejects, &r.desc, &mut row);
    let status = if !g.rs_v2() {
        "not_rs_v2"
    } else if !r.aes.pass {
        "rejected_aesthetic"
    } else if !r.rejects.is_empty() {
        "rejected_gate"
    } else if r.fem.is_none() {
        "fem_failed"
    } else {
        "screened"
    };
    row.text("meta.status", status);
    if let Some(fm) = &r.fem {
        screen_cells(fm, p.mass_g, &mut row);
        row.num("s1.mass_g", p.mass_g);
        let (bw, _) = fem::ring_bearing(p);
        row.num("s1.ring_bearing_w_mm", bw);
    }
    debug_assert!(row.get(HASH_COL).is_some());
    row
}

/// Featurize a genome from scratch (no FEM): phenotype, aesthetics, gates, descriptor.
pub fn record_from_genome(g: &Genome) -> (Row, Pheno) {
    let p = pheno::build(g);
    let aes = aesthetic::evaluate(g, &p);
    let rejects = gates::check(g, &p);
    let desc = qd::descriptor(g, &p);
    let mut row = Row::new();
    genome_cells(g, &mut row);
    row.text("meta.feature_schema", FEATURE_SCHEMA);
    design_cells(g, &p, &aes, &rejects, &desc, &mut row);
    (row, p)
}

#[cfg(test)]
mod tests {
    use super::*;
    use jarambit_crescent::genome::{NAMES, N_CP};

    /// Gene-name parity between the pure dataset engine and the crescent genome.
    fn gene_names_match() -> bool {
        NAMES.len() == gfeat::NAMES.len() && NAMES.iter().zip(gfeat::NAMES.iter()).all(|(a, b)| a == b) && N_CP == gfeat::N_CP
    }

    #[test]
    fn gene_lists_never_drift_from_the_crescent_genome() {
        assert!(gene_names_match());
        assert_eq!(gfeat::gene_columns().len(), jarambit_crescent::genome::NG + 1);
        for i in 0..N_CP {
            assert_eq!(gfeat::cp_delta(i), jarambit_crescent::genome::cp_delta(i), "cp_delta({i})");
        }
    }

    #[test]
    fn c6_seed_featurizes_with_exact_genes_and_finite_cad_features() {
        let g = Genome::c6_seed(5.7);
        let (row, p) = record_from_genome(&g);
        // exact genes round-trip to the same genome
        let genes = row.genes().expect("complete genome");
        assert_eq!(&genes[..genes.len() - 1], g.v.as_slice());
        assert_eq!(row.get("meta.feature_schema"), Some(FEATURE_SCHEMA));
        let num = |k: &str| super::super::csvio::num(row.get(k).unwrap_or(""));
        assert!((num("c.mass_g") - p.mass_g).abs() / p.mass_g < 1e-6);
        assert!(num("c.blade_tip_to_heel") > 80.0);
        assert!(num("c.ridge_min_halfwidth_mm") > 0.0);
        assert!(num("c.th_max") <= p.t + 1e-6);
        assert!((0.0..=1.0).contains(&num("c.solidity")));
        let sectors: f64 = (0..N_SECTORS).map(|s| num(&format!("c.sector{s:02}_mass_frac"))).sum();
        assert!((sectors - 1.0).abs() < 1e-3, "{sectors}");
        assert!(row.columns().iter().any(|c| c.starts_with("a.khist")));
        assert!(row.columns().iter().filter(|c| c.starts_with("desc.")).count() == 16);
        // no FEM-derived target leaks into a design feature family
        assert!(!row.columns().iter().any(|c| c.starts_with("s1.")));
    }

    #[test]
    fn feature_columns_are_stable_across_genomes() {
        let a = record_from_genome(&Genome::c6_seed(5.7)).0.columns();
        let mut r = jarambit_genotype::Rng(11);
        let b = record_from_genome(&Genome::c6_seed(5.7).mutate(&mut r, 0.2)).0.columns();
        assert_eq!(a, b, "every record from the featurizer carries the same columns in the same order");
    }
}
