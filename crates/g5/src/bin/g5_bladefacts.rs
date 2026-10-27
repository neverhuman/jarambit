//! Release tool (v3.0.0): measured blade facts of one design as JSON, for the
//! v3.0.0 receipt: serrations per edge as built (depth, teeth, tooth-root SF, flank angle), Gate S1/S3 on true
//! edge-normal sections, the sharpened arcs, reach chord and the decoder hash.
//!
//!   bladefacts <root> <design_id> <out.json>
use jarambit_g5::g5_genome::G5Genome;
use jarambit_g5::g5_qd::{g5_decoder_sha256, measure_blade_edge_apex_and_angle, serration_tooth_strength, TOOTH_PRY_N, TOOTH_SNAG_N};
use jarambit_g5::g5_sdf::build_blade_profile_2d;
use std::{fs, path::Path};

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let mut row = None;
    for isl in fs::read_dir(Path::new(&a[1])).expect("root").flatten() {
        let Ok(t) = fs::read_to_string(isl.path().join("run/genomes_g5.csv")) else { continue };
        for l in t.lines().skip(1) {
            if let Some((d, r)) = l.split_once(',') {
                if d == a[2] {
                    row = Some(r.to_string());
                }
            }
        }
    }
    let row = row.expect("design not found");
    let g = G5Genome::from_csv_row(&row).expect("row");
    let p = g.decode();
    let b = build_blade_profile_2d(&p);
    let (apex, half) = measure_blade_edge_apex_and_angle(&p, &b);
    let edge = |inner: bool| {
        let (sf, teeth, flank) = serration_tooth_strength(&p, &b, inner);
        serde_json::json!({
            "serration_depth_mm": if inner { b.serr_depth_in_mm } else { b.serr_depth_sp_mm },
            "teeth": if inner { b.serr_teeth_in } else { b.serr_teeth_sp },
            "tooth_root_sf_min": sf, "teeth_checked": teeth, "mean_flank_deg": flank,
            "primary_angle_gene_deg": p[if inner { "angle_in" } else { "angle_sp" }],
            "sharpened_arc_mm": if inner { b.sharpened_arc_in_mm } else { b.sharpened_arc_sp_mm },
            "edge_arc_mm": if inner { b.arc_in_mm } else { b.arc_sp_mm },
        })
    };
    let out = serde_json::json!({
        "design_id": a[2], "genome_sha256": g.sha256(), "genome_row": row, "decoder_sha256": g5_decoder_sha256(),
        "gate_s": {"max_apex_mm": apex, "max_half_angle_deg": half},
        "edge_inner": edge(true), "edge_spine": edge(false),
        "tooth_loads_n": {"snag_along_edge": TOOTH_SNAG_N, "pry_across_blade": TOOTH_PRY_N},
        "reach_chord_mm": b.reach_chord_mm, "blade_len_gene_mm": p["blade_len"], "tip_xy": b.tip_xy,
    });
    fs::write(&a[3], serde_json::to_string_pretty(&out).unwrap()).unwrap();
    println!("{}", serde_json::to_string(&out).unwrap().chars().take(600).collect::<String>());
}
