//! `g51-fe`: the hand-zone voxel FE of G5.1, run on one design.
//!
//! ```text
//! g51-fe v6check <repo> [threads]
//!     The v6.0.0 hex8 cross-check reproduced with G5.1's FE: the v6 winner on its G5.0 grid at h 0.2, under
//!     `FeDefinition::v6_cross_check`, against evidence/g5/v6.0.0/fem/{grip_squeeze,guard_push,teeth}.json
//!     (relative 1e-6: G5.0 superposed six unit wrenches, G5.1 solves the one wrench). Exit 1 on a mismatch.
//! g51-fe run <genomes.csv> <design_id> <h> <definition.json> [g50]
//!     The three cases of one design on its G5.1 grid (or G5.0's, with `g50`) as JSON, with the seconds per solve.
//! ```
use jarambit_g5::g5_genome::G5Genome;
use jarambit_g5::g5_sdf::{build_blade_profile_2d, build_g5_sdf};
use jarambit_g51::fe::hand::{grip_squeeze_fe, guard_push_fe, teeth_fe, FeCase, FeDefinition};
use jarambit_g51::sdf::build_g51_sdf;
use serde_json::{json, Value};
use std::time::Instant;

fn fail(msg: impl std::fmt::Display) -> ! {
    eprintln!("g51-fe: {msg}");
    std::process::exit(1)
}

fn genome_of(csv: &str, id: &str) -> G5Genome {
    let text = std::fs::read_to_string(csv).unwrap_or_else(|e| fail(format!("{csv}: {e}")));
    let row = text
        .lines()
        .find_map(|l| l.strip_prefix(&format!("{id},")))
        .unwrap_or_else(|| fail(format!("{id} not in {csv}")));
    let g = G5Genome::from_csv_row(row).unwrap_or_else(|e| fail(format!("{id}: {e}")));
    if !id.ends_with(&g.sha256()[..8]) {
        fail(format!("{id}: the genome does not hash to its id"));
    }
    g
}

fn cases(g: &G5Genome, h: f64, def: &FeDefinition, g50: bool) -> (Value, [Result<Option<FeCase>, String>; 3]) {
    let p = g.decode();
    let blade = build_blade_profile_2d(&p);
    let t = Instant::now();
    let grid = if g50 { build_g5_sdf(g, h, true) } else { build_g51_sdf(g, h, true) };
    let grid_s = t.elapsed().as_secs_f64();
    let grip = grip_squeeze_fe(&p, &grid, def);
    let guard = guard_push_fe(&p, &grid, def).map(Some);
    let teeth = teeth_fe(&p, &blade, &grid, def);
    (json!({"grid_seconds": grid_s, "voxels": grid.nx * grid.ny * grid.nz}), [grip, guard, teeth])
}

fn case_json(r: &Result<Option<FeCase>, String>) -> Value {
    match r {
        Ok(Some(c)) => json!({"sf": c.sf, "seconds": c.seconds(), "solves": c.solves}),
        Ok(None) => json!({"sf": "inf", "note": "no such case on this design"}),
        Err(e) => json!({"sf": 0.0, "error": e}),
    }
}

fn v6check(repo: &str, threads: usize) {
    let g = genome_of(&format!("{repo}/evidence/g5/v6.0.0/winner_genome.csv"), "kv4i1404_g00581_29ea3c73");
    let def = FeDefinition::v6_cross_check(threads);
    let (grid, [grip, guard, teeth]) = cases(&g, 0.2, &def, true);
    let mut ok = true;
    let mut out = Vec::new();
    for (name, r) in [("grip_squeeze", &grip), ("guard_push", &guard), ("teeth", &teeth)] {
        let ev: Value = serde_json::from_str(
            &std::fs::read_to_string(format!("{repo}/evidence/g5/v6.0.0/fem/{name}.json")).unwrap_or_else(|e| fail(e)),
        )
        .unwrap_or_else(|e| fail(e));
        let want = ev["sf_fem"].as_f64().unwrap_or_else(|| fail(format!("{name}: no sf_fem")));
        let c = match r {
            Ok(Some(c)) => c,
            other => fail(format!("{name}: {other:?}")),
        };
        let rel = (c.sf / want - 1.0).abs();
        ok &= rel <= 1e-6;
        out.push(json!({"case": name, "sf_g51": c.sf, "sf_v6": want, "rel_diff": rel,
                        "seconds_g51": c.seconds(), "seconds_v6": ev["solver"]["seconds"], "solves": c.solves}));
    }
    println!("{}", serde_json::to_string_pretty(&json!({"grid": grid, "threads": threads, "cases": out, "match": ok})).unwrap());
    if !ok {
        fail("the v6.0.0 cross-check is not reproduced within 1e-6");
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("v6check") if a.len() >= 3 => v6check(&a[2], a.get(3).and_then(|v| v.parse().ok()).unwrap_or(4)),
        Some("run") if a.len() >= 6 => {
            let g = genome_of(&a[2], &a[3]);
            let h: f64 = a[4].parse().unwrap_or_else(|e| fail(e));
            let def: FeDefinition = serde_json::from_str(&std::fs::read_to_string(&a[5]).unwrap_or_else(|e| fail(e)))
                .unwrap_or_else(|e| fail(e));
            let g50 = a.get(6).map(String::as_str) == Some("g50");
            let t = Instant::now();
            let (grid, [grip, guard, teeth]) = cases(&g, h, &def, g50);
            println!(
                "{}",
                json!({"design_id": a[3], "h": h, "decoder": if g50 { "G5.0" } else { "G5.1" }, "definition": def,
                       "grid": grid, "grip_squeeze": case_json(&grip), "guard_push": case_json(&guard),
                       "teeth": case_json(&teeth), "seconds": t.elapsed().as_secs_f64()})
            );
        }
        _ => fail("usage: g51-fe v6check <repo> [threads] | run <genomes.csv> <design_id> <h> <definition.json> [g50]"),
    }
}
