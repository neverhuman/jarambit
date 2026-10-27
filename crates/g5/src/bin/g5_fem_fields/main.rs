//! FEM renderings of every RS-karambit-v5 load case of one G5 design (v6.0.0; frozen decoder G5.0, hash-neutral).
//!
//!   g5-fem-fields <genomes.csv> <design_id> <record.json|.jsonl> <out_dir> [h=0.2]
//!
//! Re-solves the design as the Tier-2 evaluation of record does (ranking grid h 0.2), keeping the fields that the
//! scored solvers discard, and writes `<out_dir>/<case>.json` and `<case>.svg` for the 16 cases, plus `index.svg` and
//! `figures.json`. The models are copies of the scored ones (each copy says where it came from):
//! - the CST+DKT sandwich plate of the whole blade (`plate`): lateral limit and ultimate, twist, pull, fatigue;
//! - the TET10 solid of the last 30 mm (`tip`): tip axial, side and twist;
//! - the T6 ring and neck membrane (`ring`): ring bearing, back and any direction (12 directions);
//! - the Rayleigh-Ritz column (`ritz`): buckling of the point, with its mode;
//! - the blade-before-hand sequence: the governing pair of fields side by side.
//!
//! **Fail closed.** Every number of the record's `loads` that these models produce (the 11 FE safety factors, the
//! Ritz λ, the sequence ratios and the analytic hand-side values) must reproduce the record bit for bit on the
//! platform that wrote it. Mismatches are rendered and written with `match: false`, and the run exits 1.
//!
//! **Independent checks.** Grip squeeze, guard push and teeth are analytic in the score. Each also gets a hex8 voxel
//! FE solve on the design's own AM voxel field at `h` (`hex8`, on the `voxel_fe` copied from `g5-gripfe`). Its SF is
//! reported beside the formula's, as a cross-check that is not gated.
mod hex8;
#[allow(dead_code)] // kept fields not drawn
mod plate;
mod ring;
mod ritz;
mod svg;
#[allow(dead_code)] // verbatim copy: every result field is kept
mod tip;
#[allow(dead_code)] // verbatim copy of g5-gripfe's voxel_fe
mod voxel_fe;

use jarambit_g5::g5_genome::G5Genome;
use jarambit_g5::g5_qd::serration_tooth_strength;
use jarambit_g5::g5_sdf::{build_blade_profile_2d, build_g5_sdf, eval_blade_thickness_and_sdf2, BladeProfile2D};
use jarambit_g5::loads::{analytic_loads, blade_section, G5Loads, Midline, TipModel, PLATE_MAX_AREA_MM2, TIP_FE_MAX_AREA_MM2};
use jarambit_g5::material::{self, GROUND_WALL};
use jarambit_g5::requirements::rs_v5;
use jarambit_g5::{objective, sha256_bytes};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::time::Instant;

pub type Params = BTreeMap<&'static str, f64>;

fn fail(msg: impl std::fmt::Display) -> ! {
    eprintln!("g5-fem-fields: FAIL: {msg}");
    std::process::exit(1)
}

/// The genome of `id` from a `design_id,<genome row>` CSV; it must re-hash to its id.
fn genome_of(csv: &Path, id: &str) -> G5Genome {
    let text = fs::read_to_string(csv).unwrap_or_else(|e| fail(format!("{}: {e}", csv.display())));
    for line in text.lines().skip(1) {
        if let Some((d, row)) = line.split_once(',') {
            if d == id {
                let g = G5Genome::from_csv_row(row).unwrap_or_else(|e| fail(format!("{id}: {e}")));
                if !id.ends_with(&g.sha256()[..8]) {
                    fail(format!("{id}: genome does not re-hash to its id"));
                }
                return g;
            }
        }
    }
    fail(format!("{id} not in {}", csv.display()))
}

/// The record of `id`: a JSON object, or the row of a JSONL file with that `design_id`; a row may wrap the record as
/// `{"design_id", "h", "record"}`. The caller checks the genome hash.
fn record_of(path: &Path, id: &str) -> Value {
    let text = fs::read_to_string(path).unwrap_or_else(|e| fail(format!("{}: {e}", path.display())));
    let row = match serde_json::from_str::<Value>(&text) {
        Ok(v) if v.get("design_id").is_none_or(|d| d == &json!(id)) => v,
        Ok(_) => fail(format!("{} is not the record of {id}", path.display())),
        Err(_) => text
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| serde_json::from_str::<Value>(l).unwrap_or_else(|e| fail(format!("{}: {e}", path.display()))))
            .find(|r| r["design_id"] == json!(id))
            .unwrap_or_else(|| fail(format!("{id} not in {}", path.display()))),
    };
    if row.get("record").is_some() {
        row["record"].clone()
    } else {
        row
    }
}

/// The tip cases as `loads::tip_solid_cases_at` runs them (lines copied from `crates/g5/src/loads.rs` at 48d3a8c7),
/// on the copied solver that keeps the fields.
fn tip_cases(p: &Params, b: &BladeProfile2D, max_area_mm2: f64) -> Option<Vec<(&'static str, f64, tip::SolidTipResult, (f64, f64))>> {
    use tip::{solve_solid_tet10, SolidTipInput, ThicknessField, TipLoad};
    let rs = rs_v5();
    let tip = b.tip_xy;
    let ml = Midline::of(b);
    let (_, axis) = ml.at(0.0);
    let n = b.xs.len();
    let near: Vec<usize> = (0..n)
        .filter(|&i| (b.xs[i] - tip.0).hypot(0.5 * (b.y_in[i] + b.y_sp[i]) - tip.1) <= 34.0)
        .collect();
    if near.len() < 8 {
        return None;
    }
    let mut outer: Vec<(f64, f64)> = near.iter().map(|&i| (b.xs[i], b.y_in[i])).collect();
    outer.extend(near.iter().rev().map(|&i| (b.xs[i], b.y_sp[i])));
    let mesh = jarambit_physics::fine::mesh_domain(&outer, &[], max_area_mm2)?;
    let (g, nr) = (0.25_f64, 272usize);
    let (x0, y0) = (tip.0 - 34.0, tip.1 - 34.0);
    let mut th = Vec::with_capacity(nr * nr);
    for j in 0..nr {
        let y = y0 + (j as f64 + 0.5) * g;
        for i in 0..nr {
            let x = x0 + (i as f64 + 0.5) * g;
            let (t, d2) = eval_blade_thickness_and_sdf2(p, b, x, y);
            th.push(if d2 <= 0.0 && t > 0.05 { Some(t) } else { None });
        }
    }
    let field = ThicknessField { x0, y0, g, nx: nr, ny: nr, th };
    let allow = material::yield_allowable(1.0, GROUND_WALL);
    let ld = &rs.loads.tip;
    let solve = |load: TipLoad, patch: f64, readout: (f64, f64)| -> Option<(f64, tip::SolidTipResult)> {
        let input = SolidTipInput {
            xy: mesh.xy.clone(),
            tris: mesh.tris.clone(),
            tip_xy: tip,
            axis,
            field: field.clone(),
            r_cut: 30.0,
            nl: 4,
            nsub: 0,
            patch_mm: patch,
            e_mpa: material::e_mpa(),
            nu: material::nu(),
            load,
            readout_mm: readout,
            allow_mpa: allow,
        };
        let r = solve_solid_tet10(&input);
        let r = r?;
        let converged = r.rel_res <= 1e-10 || (r.rel_res <= r.residual_floor && r.backward_error <= 1e-12);
        let ok = r.ksp_its == 1 && r.rel_res.is_finite() && converged && r.nodal_max_peak.mpa > 0.0;
        ok.then(|| (allow / r.nodal_max_peak.mpa, r))
    };
    let s_tw = ld.twist_apply_mm_from_point;
    let ax_win = (ld.axial_no_yield_from_mm, 10.0);
    let (axial_sf, ax) = solve(TipLoad::Axial { f_n: ld.axial_n }, 0.75, ax_win)?;
    let st_hi = ld.lateral_check_stations_mm.iter().copied().fold(0.0, f64::max) + 0.5;
    let lat_win = (ld.axial_no_yield_from_mm, st_hi);
    let (lateral_sf, lat) = solve(TipLoad::Lateral { f_n: ld.lateral_n }, 0.75, lat_win)?;
    let tw_win = (s_tw + 1.5, 15.0);
    let (twist_sf, tw) = solve(TipLoad::Twist { t_nmm: ld.twist_nm * 1000.0, at_mm: s_tw }, 1.0, tw_win)?;
    Some(vec![("tip_axial", axial_sf, ax, ax_win), ("tip_lateral", lateral_sf, lat, lat_win), ("tip_twist", twist_sf, tw, tw_win)])
}

/// Every f64 of `G5Loads` that the re-solve produces, by name, with the record's value.
fn loads_pairs(mine: &G5Loads, rec: &G5Loads) -> Vec<(&'static str, f64, f64)> {
    macro_rules! pairs { ($($f:ident),*) => { vec![$((stringify!($f), mine.$f, rec.$f)),*] } }
    let mut v = pairs!(
        lateral_force_n, lateral_arm_mm, lateral_limit_sf, lateral_ultimate_sf, twist_limit_sf, tip_axial_sf,
        tip_lateral_sf, tip_twist_sf, pull_sf, buckling_lambda, teeth_sf, fatigue_sf, grip_lateral_sf, grip_twist_sf,
        grip_pull_sf, grip_squeeze_sf, guard_push_sf, ring_bearing_sf, ring_back_sf, ring_any_direction_sf,
        hand_fatigue_sf, seq_lateral, seq_twist, seq_pull
    );
    v.push(("lateral_peak_at.s", mine.lateral_peak_at.0, rec.lateral_peak_at.0));
    v.push(("lateral_peak_at.eta", mine.lateral_peak_at.1, rec.lateral_peak_at.1));
    v.push(("twist_peak_at.s", mine.twist_peak_at.0, rec.twist_peak_at.0));
    v.push(("twist_peak_at.eta", mine.twist_peak_at.1, rec.twist_peak_at.1));
    v
}

fn same(a: f64, b: f64) -> bool {
    a.to_bits() == b.to_bits()
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 5 {
        fail("usage: g5-fem-fields <genomes.csv> <design_id> <record.json|.jsonl> <out_dir> [h=0.2]");
    }
    let t_start = Instant::now();
    let (id, out) = (a[2].as_str(), Path::new(&a[4]));
    let h: f64 = a.get(5).map_or(0.2, |v| v.parse().unwrap_or_else(|_| fail("h")));
    let g = genome_of(Path::new(&a[1]), id);
    let rec = record_of(Path::new(&a[3]), id);
    if rec["genome_sha256"] != json!(g.sha256()) {
        fail(format!("{id}: the record is of genome {}, not {}", rec["genome_sha256"], g.sha256()));
    }
    if rec["decoder_sha256"] != json!(jarambit_g5::g5_qd::g5_decoder_sha256()) {
        fail(format!("{id}: the record is from decoder {}", rec["decoder_sha256"]));
    }
    let rec_loads: G5Loads = serde_json::from_value(rec["loads"].clone()).unwrap_or_else(|e| fail(format!("record loads: {e}")));
    if rec_loads.tip_model != TipModel::Solid {
        fail("the record is not a Tier-2 (solid tip) record");
    }
    fs::create_dir_all(out).unwrap_or_else(|e| fail(e));
    let p = g.decode();
    let blade = build_blade_profile_2d(&p);

    // 1. Tier 0: analytic hand side, guard, buckling, teeth (`evaluate_tier0_analytic`)
    let mut loads = analytic_loads(&p, &blade, 0.0, 0.0);
    loads.teeth_sf = [true, false].iter().map(|&inner| serration_tooth_strength(&p, &blade, inner).0).fold(f64::INFINITY, f64::min);
    // 2. Tier 1: the ring on the voxel field at h, then the plate (`evaluate_tier1_with_grid`)
    let t = Instant::now();
    let grid = build_g5_sdf(&g, h, true);
    let t_grid = t.elapsed().as_secs_f64();
    let t = Instant::now();
    let ring_ld = &rs_v5().loads.ring;
    let Some((ring_sum, ring_f)) = ring::ring_solve(&p, &grid) else { fail("ring mesh or solve failed") };
    loads.ring_bearing_sf = ring_sum.bearing.capacity_n / ring_ld.bearing_n;
    loads.ring_back_sf = ring_sum.back.capacity_n / ring_ld.back_n;
    loads.ring_any_direction_sf = ring_sum.any_direction_min_capacity_n / ring_ld.any_direction_n;
    let t_ring = t.elapsed().as_secs_f64();
    let t = Instant::now();
    let Some(plate_f) = plate::plate_blade_cases_at(&p, &blade, &mut loads, PLATE_MAX_AREA_MM2) else { fail("plate mesh or solve failed") };
    let t_plate = t.elapsed().as_secs_f64();
    // 3. Tier 2: the solid tip (`evaluate_tier2_from_tier1`)
    let t = Instant::now();
    let Some(tips) = tip_cases(&p, &blade, TIP_FE_MAX_AREA_MM2) else { fail("tip mesh or solve failed") };
    loads.tip_axial_sf = tips[0].1;
    loads.tip_lateral_sf = tips[1].1;
    loads.tip_twist_sf = tips[2].1;
    loads.tip_model = TipModel::Solid;
    let t_tip = t.elapsed().as_secs_f64();
    // 4. Buckling with its mode (the column of `analytic_loads`)
    let ml = Midline::of(&blade);
    let e_mod = material::e_mpa();
    let ei = |s: f64| blade_section(&p, &blade, &ml, s.max(0.25)).map(|sec| e_mod * sec.i_minor_mm4.min(sec.i_major_mm4)).unwrap_or(1e-6);
    let col_len = 30.0_f64.min(ml.length() - 1.0);
    let (p_cr, mode_a) = ritz::tip_buckling_load_n(&ei, 0.5, col_len);
    let lambda = p_cr / rs_v5().loads.tip.axial_n;

    // The check of record: every reproduced number, bit for bit
    let pairs = loads_pairs(&loads, &rec_loads);
    let mut mismatches: Vec<String> = pairs.iter().filter(|(_, m, r)| !same(*m, *r)).map(|(n, m, r)| format!("{n}: {m} vs record {r}")).collect();
    if !same(lambda, rec_loads.buckling_lambda) {
        mismatches.push(format!("buckling_lambda (Ritz copy): {lambda} vs record {}", rec_loads.buckling_lambda));
    }
    let check: Vec<Value> = pairs.iter().map(|(n, m, r)| json!({"field": n, "fem_fields": m, "record": r, "match": same(*m, *r)})).collect();
    let timing = json!({"grid_s": t_grid, "ring_s": t_ring, "plate_s": t_plate, "tip_s": t_tip});
    eprintln!("g5-fem-fields: {} reproduced fields, {} mismatches; {timing}", pairs.len(), mismatches.len());
    for m in &mismatches {
        eprintln!("  MISMATCH {m}");
    }

    let ctx = svg::Ctx { id, h, record_sha: sha256_bytes(serde_json::to_string(&rec).unwrap().as_bytes()) };
    let margins: BTreeMap<&str, f64> = objective::margins(&rec_loads).into_iter().collect();
    let mut figures = Vec::new();
    figures.extend(svg::plate_figures(&ctx, out, &plate_f, &loads, &rec_loads, &margins, &blade));
    figures.extend(svg::tip_figures(&ctx, out, &tips, &rec_loads, &margins, &blade));
    figures.extend(svg::ring_figures(&ctx, out, &ring_f, &ring_sum, &rec_loads, &margins));
    figures.push(svg::sequence_figure(&ctx, out, &ring_f, &plate_f, &loads, &rec_loads, &margins));
    figures.push(svg::buckling_figure(&ctx, out, &mode_a, 0.5, col_len, lambda, &rec_loads, &margins));
    let t = Instant::now();
    figures.extend(hex8::checks(&ctx, out, &g, &p, &blade, &grid, &rec_loads, &margins));
    let t_hex8 = t.elapsed().as_secs_f64();
    svg::index(&ctx, out, &figures);
    let summary = json!({
        "tool": "g5-fem-fields", "design_id": id, "h": h, "decoder_sha256": jarambit_g5::g5_qd::g5_decoder_sha256(),
        "record_sha256": ctx.record_sha, "reproduced": check, "mismatches": mismatches,
        "timing_s": {"grid": t_grid, "ring": t_ring, "plate": t_plate, "tip": t_tip, "hex8": t_hex8, "total": t_start.elapsed().as_secs_f64()},
        "figures": figures,
    });
    fs::write(out.join("figures.json"), serde_json::to_string_pretty(&summary).unwrap() + "\n").unwrap_or_else(|e| fail(e));
    if !mismatches.is_empty() {
        fail(format!("{} reproduced values differ from the record (see figures.json)", mismatches.len()));
    }
    println!("g5-fem-fields: {id}: every reproduced value matches the record bit for bit; {} figures in {}", figures.len(), out.display());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ritz_copy_reproduces_the_scored_eigenvalue_and_its_mode_solves_the_eigenproblem() {
        let profiles: [&dyn Fn(f64) -> f64; 3] = [&|_| 2.0e6, &|s| 1.0e5 * (1.0 + s).powi(3), &|s| 3.0e4 + 4.0e3 * s * s + 1.0e5 * (0.3 * s).sin().abs()];
        for (k, ei) in profiles.iter().enumerate() {
            let (s0, len) = (0.5, 30.0);
            let scored = jarambit_g5::loads::tip_buckling_load_n(*ei, s0, len);
            let (copy, a) = ritz::tip_buckling_load_n(*ei, s0, len);
            assert_eq!(copy.to_bits(), scored.to_bits(), "profile {k}: {copy} vs {scored}");
            // the mode solves the discrete eigenproblem K a = lambda G a of the same 7-function, 400-point column
            const NB: usize = 7;
            let (nq, l) = (400usize, len - s0);
            let (mut k_m, mut g_m) = ([[0.0_f64; NB]; NB], [[0.0_f64; NB]; NB]);
            for q in 0..nq {
                let xi = (q as f64 + 0.5) / nq as f64;
                let (e, w) = (ei(s0 + xi * l), l / nq as f64);
                let d1: [f64; NB] = std::array::from_fn(|i| -((i + 2) as f64) * (1.0 - xi).powf((i + 1) as f64) / l);
                let d2: [f64; NB] = std::array::from_fn(|i| ((i + 2) * (i + 1)) as f64 * (1.0 - xi).powf(i as f64) / (l * l));
                for r in 0..NB {
                    for c in 0..NB {
                        k_m[r][c] += e * d2[r] * d2[c] * w;
                        g_m[r][c] += d1[r] * d1[c] * w;
                    }
                }
            }
            let ka: Vec<f64> = (0..NB).map(|r| (0..NB).map(|c| k_m[r][c] * a[c]).sum()).collect();
            let ga: Vec<f64> = (0..NB).map(|r| (0..NB).map(|c| g_m[r][c] * a[c]).sum()).collect();
            let res = (0..NB).map(|r| (ka[r] - scored * ga[r]).powi(2)).sum::<f64>().sqrt();
            let norm = ka.iter().map(|v| v * v).sum::<f64>().sqrt();
            assert!(res / norm < 1e-9, "profile {k}: |K a - lambda G a| / |K a| = {}", res / norm);
            // and the mode is the plotted one: w(s0) is its largest deflection (the free end at the point)
            let mode = ritz::mode_shape(&a, s0, len, 200);
            let w0 = mode[0].1.abs();
            assert!(mode.iter().all(|m| m.1.abs() <= w0 * (1.0 + 1e-12)), "profile {k}: the mode's largest deflection is not at the point");
        }
    }

    /// The plate copy against the committed Linux x86_64 Tier-2 record of the v5.0.0 winner (bit for bit, so only on
    /// that platform; other platforms differ in the last bits by their libm).
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    #[test]
    fn plate_copy_reproduces_the_v5_winner_linux_record() {
        let ev = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../evidence/g5/v5.0.0");
        let id = "kv4i012_g00009_dd6ffea5";
        let g = genome_of(&ev.join("winner_genome.csv"), id);
        let rec = record_of(&ev.join("tier2_linux_h020.jsonl"), id);
        let rl: G5Loads = serde_json::from_value(rec["loads"].clone()).unwrap();
        let p = g.decode();
        let blade = build_blade_profile_2d(&p);
        let mut l = analytic_loads(&p, &blade, 0.0, 0.0);
        l.ring_any_direction_sf = rl.ring_any_direction_sf;
        let f = plate::plate_blade_cases_at(&p, &blade, &mut l, PLATE_MAX_AREA_MM2).expect("plate");
        for (n, a, b) in [
            ("lateral_limit_sf", l.lateral_limit_sf, rl.lateral_limit_sf),
            ("lateral_ultimate_sf", l.lateral_ultimate_sf, rl.lateral_ultimate_sf),
            ("twist_limit_sf", l.twist_limit_sf, rl.twist_limit_sf),
            ("pull_sf", l.pull_sf, rl.pull_sf),
            ("fatigue_sf", l.fatigue_sf, rl.fatigue_sf),
            ("seq_lateral", l.seq_lateral, rl.seq_lateral),
            ("seq_twist", l.seq_twist, rl.seq_twist),
            ("seq_pull", l.seq_pull, rl.seq_pull),
        ] {
            assert_eq!(a.to_bits(), b.to_bits(), "{n}: {a} vs record {b}");
        }
        // the kept fields are those the SFs were read from
        let lat = &f.cases[0];
        let e = lat.governing;
        assert_eq!((f.allow_y[e] / lat.sigma[e]).to_bits(), rl.lateral_limit_sf.to_bits());
    }
}
