//! The RS-karambit-v5 blade cases on the CST+DKT sandwich plate, copied from `loads::plate_blade_cases_at`
//! (`crates/g5/src/loads.rs` at 48d3a8c7; that file is hashed into the G5 decoder) through the public
//! `SandwichPlate` API, so that the fields can be kept. The setup, the three solves and every read-out are the
//! original lines; the additions only collect the element stress, displacement, read-out masks and governing
//! elements (`PlateFields`). The safety factors are checked against the record bit for bit.

use jarambit_g5::g5_sandwich::{SandwichPlate, SandwichSection};
use jarambit_g5::g5_sdf::{effective_blade_rho, eval_blade_thickness_and_sdf2, BladeProfile2D};
use jarambit_g5::loads::{blade_section, BladeModel, G5Loads, Midline};
use jarambit_g5::material::{self, GROUND_WALL, HOLLOW_BLADE_SKIN};
use jarambit_g5::requirements::rs_v5;
use std::collections::BTreeMap;

type Params = BTreeMap<&'static str, f64>;

/// One plate case: the element stress at the case load (MPa; for fatigue the amplitude max(k_b s1, k_t s2)), the
/// nodal displacement (5 per node: u, v, w, w_x, w_y), the elements read for its SF, and the governing element.
pub struct PlateCase {
    pub name: &'static str,
    pub sigma: Vec<f64>,
    pub u: Vec<f64>,
    pub read: Vec<bool>,
    pub governing: usize,
    /// Case load (N; N mm for the twist)
    pub load: f64,
}

pub struct PlateFields {
    pub xy: Vec<(f64, f64)>,
    pub tris: Vec<[usize; 3]>,
    /// Element yield and ultimate allowables (MPa; infinite for the near-empty outline slivers)
    pub allow_y: Vec<f64>,
    pub allow_u: Vec<f64>,
    pub hollow: Vec<bool>,
    pub cases: Vec<PlateCase>,
    /// The lateral patch centre (5 mm behind the point)
    pub patch_centre: (f64, f64),
    /// Nodes with x <= clamp_x are clamped; elements are read from x >= read_x
    pub clamp_x: f64,
    pub read_x: f64,
    pub held_s: f64,
}

pub fn plate_blade_cases_at(p: &Params, b: &BladeProfile2D, loads: &mut G5Loads, max_area_mm2: f64) -> Option<PlateFields> {
    let rs = rs_v5();
    let ld = &rs.loads;
    let n_st = b.xs.len();
    let mut outer: Vec<(f64, f64)> = (0..n_st).map(|i| (b.xs[i], b.y_in[i])).collect();
    outer.extend((0..n_st).rev().map(|i| (b.xs[i], b.y_sp[i])));
    let Some(mesh) = jarambit_physics::fine::mesh_domain(&outer, &[], max_area_mm2) else {
        return None;
    };
    let ml = Midline::of(b);
    let len = ml.length();
    let rho_b = effective_blade_rho(p);
    let core_e = material::core_relative_stiffness(rho_b, p["lat_cell"], p["lat_mix"]);
    let skin = p["skin_t"];
    let core_x_max = b.x_tip - p["tip_reserve"].max(6.0) - 4.0;
    // Midline samples every 0.25 mm and section half-widths every 1 mm, computed once
    let samples: Vec<(f64, (f64, f64), (f64, f64))> = (0..=((len / 0.25) as usize))
        .map(|k| {
            let s = k as f64 * 0.25;
            let (m, t) = ml.at(s);
            (s, m, t)
        })
        .collect();
    let half_w: Vec<f64> = (0..=(len as usize))
        .map(|k| blade_section(p, b, &ml, k as f64).map(|sec| 0.5 * sec.width_mm).unwrap_or(0.0))
        .collect();
    let half_width_at = |s: f64| half_w[(s.round() as usize).min(half_w.len() - 1)];
    // midline arc position and signed offset across the blade of a point
    let locate = |x: f64, y: f64| -> (f64, f64) {
        let mut best = (f64::INFINITY, 0.0, 0.0);
        for &(s, m, t) in &samples {
            let d2 = (x - m.0).powi(2) + (y - m.1).powi(2);
            if d2 < best.0 {
                best = (d2, s, (x - m.0) * -t.1 + (y - m.1) * t.0);
            }
        }
        (best.1, best.2)
    };
    let tris = mesh.tris.clone();
    let xy = mesh.xy.clone();
    let mut elem = Vec::with_capacity(tris.len());
    let mut secs = Vec::with_capacity(tris.len());
    for t in &tris {
        let xm = (xy[t[0]].0 + xy[t[1]].0 + xy[t[2]].0) / 3.0;
        let ym = (xy[t[0]].1 + xy[t[1]].1 + xy[t[2]].1) / 3.0;
        let (th, d2) = eval_blade_thickness_and_sdf2(p, b, xm, ym);
        let (s_e, eta) = locate(xm, ym);
        // an element with no material at its centroid (a sliver at the outline): near-zero stiffness, not read
        let window = d2 > 0.0 || th <= 0.0;
        let hollow =
            rho_b < 0.95 && half_width_at(s_e) - eta.abs() > 4.2 && xm <= core_x_max && th > 2.0 * skin + 1.0;
        secs.push(if window {
            SandwichSection::solid(0.05)
        } else {
            SandwichSection::from_skin_and_lattice(th.max(0.3), skin, if hollow { core_e } else { 1.0 }, 0.0)
        });
        let (tw, wall) = if hollow { (skin, HOLLOW_BLADE_SKIN) } else { (th.max(0.3), GROUND_WALL) };
        let (a_y, a_u) = if window {
            (f64::INFINITY, f64::INFINITY)
        } else {
            (material::yield_allowable(tw, wall), material::ultimate_allowable(tw, wall))
        };
        elem.push((xm, ym, s_e, hollow, th, a_y, a_u));
    }
    let plate = SandwichPlate::new(mesh, p["t_core"], material::e_mpa(), material::nu()).with_sections(secs);
    let n = plate.n();
    let mut fixed = vec![false; n * 5];
    let mut n_fix = 0usize;
    for (i, &(x, _)) in plate.mesh.xy.iter().enumerate() {
        if x <= b.x_heel + 3.5 {
            n_fix += 1;
            for c in 0..5 {
                fixed[i * 5 + c] = true;
            }
        }
    }
    if n_fix < 3 {
        return None;
    }
    let Some(sys) = plate.system(&fixed) else {
        return None;
    };
    let node_pos: Vec<(f64, f64)> = plate.mesh.xy.iter().map(|&(x, y)| locate(x, y)).collect();

    // 1. Lateral: the limit force normal to the blade on a 2 mm patch around the point 5 mm from the tip
    let (q5, _) = ml.at(ld.lateral_bend.apply_mm_from_point);
    let heel = ml.at(len).0;
    let f_lat = ld.lateral_bend.limit_root_moment_nm * 1000.0 / (q5.0 - heel.0).hypot(q5.1 - heel.1).max(1.0);
    let patch: Vec<usize> =
        (0..n).filter(|&i| (plate.mesh.xy[i].0 - q5.0).hypot(plate.mesh.xy[i].1 - q5.1) <= 2.0).collect();
    // 2. Body twist: a couple linear across the blade over the 25-75 % span, unit torque about the midline, on the
    //    blade body (3.5 mm clear of each edge, so no concentrated force lands on the thin bevels)
    let span: Vec<usize> = (0..n)
        .filter(|&i| {
            let (s_i, eta_i) = node_pos[i];
            s_i >= 0.25 * len && s_i <= 0.75 * len && !fixed[i * 5] && eta_i.abs() <= half_width_at(s_i) - 3.5
        })
        .collect();
    let eta2: f64 = span.iter().map(|&i| node_pos[i].1.powi(2)).sum();
    // 3. Pull: the held 20 mm pushed +x (the hand pulls the knife -x)
    let held: Vec<usize> = (0..n).filter(|&i| node_pos[i].0 <= ld.pull.held_length_from_point_mm).collect();
    if patch.is_empty() || span.is_empty() || eta2 <= 1e-9 || held.is_empty() {
        return None;
    }
    let mut f1 = vec![0.0; n * 5];
    for &i in &patch {
        f1[i * 5 + 2] = f_lat / patch.len() as f64;
    }
    let t_lim = ld.body_twist.limit_nm * 1000.0;
    let mut f2 = vec![0.0; n * 5];
    for &i in &span {
        f2[i * 5 + 2] = t_lim * node_pos[i].1 / eta2;
    }
    let mut f3 = vec![0.0; n * 5];
    for &i in &held {
        f3[i * 5] = ld.pull.limit_n / held.len() as f64;
    }
    let solve = |f: &[f64]| -> Option<(Vec<f64>, Vec<f64>)> {
        let (u, _) = plate.solve_checked(&sys, f)?;
        let (sm, sb) = plate.stresses(&u);
        Some((sm.iter().zip(sb.iter()).map(|(a, c)| a + c).collect(), u))
    };
    let (Some((s1, u1)), Some((s2, u2)), Some((s3, u3))) = (solve(&f1), solve(&f2), solve(&f3)) else {
        return None;
    };
    // read 6 mm clear of the heel clamp, whose corners are singular (the real root is checked on the hand side)
    let clamp_ok = |e: usize| elem[e].0 >= b.x_heel + 6.0;
    let worst_at = |sig: &[f64], keep: &dyn Fn(usize) -> bool, ult: bool, k: f64| -> (f64, usize) {
        (0..sig.len())
            .filter(|&e| clamp_ok(e) && keep(e))
            .map(|e| ((if ult { elem[e].6 } else { elem[e].5 }) / (k * sig[e]).max(1e-9), e))
            .fold((f64::INFINITY, 0), |a, c| if c.0 < a.0 { c } else { a })
    };
    let worst = |sig: &[f64], keep: &dyn Fn(usize) -> bool, ult: bool, k: f64| worst_at(sig, keep, ult, k).0;
    let away_from_patch = |e: usize| (elem[e].0 - q5.0).hypot(elem[e].1 - q5.1) >= 4.0;
    let k_u = ld.lateral_bend.ultimate_root_moment_nm / ld.lateral_bend.limit_root_moment_nm;
    let (lat_sf, e_lat) = worst_at(&s1, &away_from_patch, false, 1.0);
    loads.lateral_limit_sf = lat_sf;
    loads.lateral_peak_at = locate(elem[e_lat].0, elem[e_lat].1);
    loads.lateral_ultimate_sf = worst(&s1, &away_from_patch, true, k_u);
    let (tw_sf, e_tw) = worst_at(&s2, &|_| true, false, 1.0);
    loads.twist_limit_sf = tw_sf;
    loads.twist_peak_at = locate(elem[e_tw].0, elem[e_tw].1);
    let (pull_sf, e_pull) = worst_at(&s3, &|e| elem[e].2 >= ld.pull.held_length_from_point_mm + 2.0, false, 1.0);
    loads.pull_sf = pull_sf;
    let (_, e_ult) = worst_at(&s1, &away_from_patch, true, k_u);
    // Fatigue duties on the plate: ground outer faces, as-built cavity faces of a hollow blade
    let fat = &rs.fatigue;
    let k_b = fat.blade_root_moment_amplitude_nm / ld.lateral_bend.limit_root_moment_nm;
    let k_t = fat.twist_amplitude_nm / ld.body_twist.limit_nm;
    let mut fat_sf = f64::INFINITY;
    let mut e_fat = 0usize;
    let mut s_fat = vec![0.0; s1.len()];
    for e in 0..s1.len() {
        s_fat[e] = (k_b * s1[e]).max(k_t * s2[e]);
        if !clamp_ok(e) || !away_from_patch(e) || !elem[e].5.is_finite() {
            continue;
        }
        let s_a = (k_b * s1[e]).max(k_t * s2[e]);
        let mut sf = material::fatigue_sf(s_a, 0.0, material::FatigueClass::Ground);
        if elem[e].3 {
            let k_in = (1.0 - 2.0 * skin / elem[e].4.max(1e-6)).max(0.0);
            let class = material::fatigue_class(skin, HOLLOW_BLADE_SKIN);
            sf = sf.min(material::fatigue_sf(s_a * k_in, 0.0, class));
        }
        if sf / fat.sf_fatigue < fat_sf {
            e_fat = e;
        }
        fat_sf = fat_sf.min(sf / fat.sf_fatigue);
    }
    loads.fatigue_sf = fat_sf;
    loads.blade_model = BladeModel::Plate;
    loads.seq_lateral = loads.grip_lateral_sf / loads.lateral_limit_sf.max(1e-9);
    loads.seq_twist = loads.grip_twist_sf / loads.twist_limit_sf.max(1e-9);
    loads.seq_pull = loads.grip_pull_sf.min(loads.ring_any_direction_sf) / loads.pull_sf.max(1e-9);
    let read_lat: Vec<bool> = (0..s1.len()).map(|e| clamp_ok(e) && away_from_patch(e)).collect();
    let read_tw: Vec<bool> = (0..s1.len()).map(clamp_ok).collect();
    let read_pull: Vec<bool> =
        (0..s1.len()).map(|e| clamp_ok(e) && elem[e].2 >= ld.pull.held_length_from_point_mm + 2.0).collect();
    let read_fat: Vec<bool> = (0..s1.len()).map(|e| read_lat[e] && elem[e].5.is_finite()).collect();
    let allow_y: Vec<f64> = elem.iter().map(|e| e.5).collect();
    let allow_u: Vec<f64> = elem.iter().map(|e| e.6).collect();
    let hollow: Vec<bool> = elem.iter().map(|e| e.3).collect();
    Some(PlateFields {
        xy: plate.mesh.xy.clone(),
        tris: plate.mesh.tris.clone(),
        allow_y,
        allow_u,
        hollow,
        cases: vec![
            PlateCase { name: "lateral_limit", sigma: s1.clone(), u: u1.clone(), read: read_lat.clone(), governing: e_lat, load: f_lat },
            PlateCase { name: "lateral_ultimate", sigma: s1.iter().map(|v| k_u * v).collect(), u: u1.iter().map(|v| k_u * v).collect(), read: read_lat, governing: e_ult, load: f_lat * k_u },
            PlateCase { name: "twist_limit", sigma: s2, u: u2.clone(), read: read_tw, governing: e_tw, load: t_lim },
            PlateCase { name: "pull", sigma: s3, u: u3, read: read_pull, governing: e_pull, load: ld.pull.limit_n },
            PlateCase { name: "fatigue", sigma: s_fat, u: u1.iter().map(|v| k_b * v).collect(), read: read_fat, governing: e_fat, load: k_b * f_lat },
        ],
        patch_centre: q5,
        clamp_x: b.x_heel + 3.5,
        read_x: b.x_heel + 6.0,
        held_s: ld.pull.held_length_from_point_mm,
    })
}
