//! `g3-release-tex`: LaTeX macros for the G3 release paper, generated from the committed release receipt, so the
//! paper states no number by hand (owner, 2026-10-26: README and paper "100 % up to date"). Deterministic.
//!
//!   g3-release-tex <release.json> <out_dir>
//!
//! Writes `<out_dir>/numbers.tex` (one `\newcommand` per quantity), `<out_dir>/topeight.tex` (ranking rows),
//! `<out_dir>/lineage.tex` (the lineage strip) and `<out_dir>/assets.tex` (CAD files). A missing field is an error.
use serde_json::Value;
use std::fmt::Write as _;
use std::{fs, path::Path, process::exit};

fn get<'a>(r: &'a Value, ptr: &str) -> &'a Value {
    r.pointer(ptr).unwrap_or_else(|| {
        eprintln!("g3-release-tex: release.json lacks {ptr}");
        exit(2)
    })
}

fn num(r: &Value, ptr: &str) -> f64 {
    get(r, ptr).as_f64().unwrap_or_else(|| {
        eprintln!("g3-release-tex: {ptr} is not a number");
        exit(2)
    })
}

/// LaTeX-safe text (identifiers with underscores, hex digests)
fn tex(s: &str) -> String {
    s.replace('\\', "\\textbackslash{}").replace('_', "\\_").replace('%', "\\%").replace('&', "\\&").replace('#', "\\#")
}

/// Thousands separator for counts (12,345)
fn count(v: f64) -> String {
    let n = v.round() as i64;
    let s = n.abs().to_string();
    let mut out = String::new();
    for (k, ch) in s.chars().enumerate() {
        if k > 0 && (s.len() - k) % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    if n < 0 { format!("-{out}") } else { out }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() != 3 {
        eprintln!("usage: g3-release-tex <release.json> <out_dir>");
        exit(2);
    }
    let r: Value = serde_json::from_str(&fs::read_to_string(&a[1]).expect("release.json")).expect("release.json is JSON");
    let out = Path::new(&a[2]);
    fs::create_dir_all(out).expect("out_dir");

    let s = |p: &str| get(&r, p).as_str().unwrap_or_else(|| { eprintln!("g3-release-tex: {p} is not text"); exit(2) }).to_string();
    let f = |p: &str, d: usize| format!("{:.*}", d, num(&r, p));
    let mut m: Vec<(&str, String)> = vec![
        ("gVersion", s("/version")),
        ("gDesign", format!("\\texttt{{{}}}", tex(&s("/design_id")))),
        ("gStatus", tex(&s("/status"))),
        ("gCommit", format!("\\texttt{{{}}}", &s("/provenance/git_commit")[..12])),
        ("gDecoder", format!("\\texttt{{{}}}", &s("/provenance/decoder_sha256")[..12])),
        ("gGenomeSha", format!("\\texttt{{{}}}", &s("/provenance/genome_sha256")[..12])),
        ("gMetric", tex(&s("/provenance/metric"))),
        ("gPlatform", tex(&s("/provenance/platform_of_record"))),
        ("gScore", f("/score/score_100", 2)),
        ("gBase", f("/score/base", 3)),
        ("gE", f("/score/E", 3)),
        ("gScoreArm", f("/arm64_score_100", 2)),
        ("gMassG", f("/mass/mesh_g", 1)),
        ("gMassVoxelG", f("/mass/search_voxel_g", 1)),
        ("gVolAM", count(num(&r, "/mass/mesh_volume_mm3"))),
        ("gVolEnv", count(num(&r, "/mass/envelope_volume_mm3"))),
        ("gDensity", f("/mass/density_g_mm3", 5)),
        ("gIslands", count(num(&r, "/cohort/islands"))),
        ("gEvalsIsland", count(num(&r, "/cohort/evaluations_per_island"))),
        ("gEvalsTotal", count(num(&r, "/cohort/evaluations_total"))),
        ("gElites", count(num(&r, "/cohort/elites"))),
        ("gTierTwo", count(num(&r, "/cohort/tier2_elites"))),
        ("gGateOne", f("/cohort/tier_gates/0", 0)),
        ("gGateTwo", f("/cohort/tier_gates/1", 0)),
        ("gH", f("/cohort/h_mm", 1)),
        ("gHRank", f("/cohort/ranking_h_mm", 1)),
        ("gMassRankG", f("/mass/ranking_voxel_g", 1)),
        ("gScoreSearch", f("/search_score_100", 2)),
        ("gLeader", format!("\\texttt{{{}}}", tex(&s("/grid_check/search_leader/design_id")))),
        ("gLeaderTierTwo", f("/grid_check/search_leader/tier2_score_100_h04", 2)),
        ("gLeaderMassA", f("/grid_check/search_leader/mass_g_h04", 1)),
        ("gLeaderMassB", f("/grid_check/search_leader/mass_g_h02", 1)),
        ("gLeaderMassC", f("/grid_check/search_leader/mass_g_h015", 1)),
        ("gGridAgree", f("/grid_check/h02_vs_h015_max_rel_pct", 1)),
        ("gLeaderScoreA", f("/grid_check/search_leader/tier1_score_100_h04", 2)),
        ("gLeaderScoreB", f("/grid_check/search_leader/tier1_score_100_h02", 2)),
        ("gHost", tex(&s("/cohort/host"))),
        ("gStarted", tex(&s("/cohort/started_utc"))),
        ("gFinished", tex(&s("/cohort/finished_utc"))),
        ("gWinIsland", tex(&s("/evolution/winner_island"))),
        ("gScoreSobol", f("/evolution/winner_island_after_sobol/best_score_100", 2)),
        ("gTipSF", f("/inputs/solid_tip_sf", 2)),
        ("gGripSF", f("/inputs/grip_sf_min", 2)),
        ("gLatN", f("/inputs/lat_rob_cert_n", 0)),
        ("gHmTwist", f("/inputs/hm_twist_sf", 2)),
        ("gHmSpine", f("/inputs/hm_spine_sf", 2)),
        ("gFat", f("/inputs/fat_sf", 2)),
        ("gSeq", f("/inputs/seq_ratio", 2)),
        ("gBuck", f("/inputs/buckling_lambda", 1)),
        ("gRingBear", count(num(&r, "/inputs/ring_bearing_cap_n"))),
        ("gRingBack", count(num(&r, "/inputs/ring_back_cap_n"))),
        ("gLg", f("/inputs/L_g_mm", 1)),
        ("gSwellPerim", f("/inputs/swell_perimeter_mm", 1)),
        ("gHoT", f("/inputs/swell_h_over_t", 2)),
        ("gComIdx", f("/inputs/com_to_index_mm", 1)),
        ("gGuardH", f("/inputs/guard_h_mm", 1)),
        ("gApexMax", f("/blade/gate_s/max_apex_mm", 3)),
        ("gHalfAngleMax", f("/blade/gate_s/max_half_angle_deg", 1)),
        ("gReach", f("/blade/reach_chord_mm", 1)),
        ("gToothSnag", f("/blade/tooth_loads_n/snag_along_edge", 0)),
        ("gToothPry", f("/blade/tooth_loads_n/pry_across_blade", 0)),
        ("gFeAllow", f("/fe_certification/allow_mpa", 0)),
        ("gFeH", f("/fe_certification/hex8_voxel_fe_h_mm", 1)),
        ("gFeGripLat", f("/fe_certification/grip_sf/lat", 2)),
        ("gFeGripTwist", f("/fe_certification/grip_sf/twist", 2)),
        ("gFeGripSpine", f("/fe_certification/grip_sf/spine", 2)),
        ("gFeRootLat", f("/fe_certification/root_sf/lat", 2)),
        ("gFeRootTwist", f("/fe_certification/root_sf/twist", 2)),
        ("gFeRootSpine", f("/fe_certification/root_sf/spine", 2)),
        ("gFormLat", f("/fe_certification/formula_sf/sf_lat", 2)),
        ("gFormTwist", f("/fe_certification/formula_sf/sf_twist", 2)),
        ("gFormSpine", f("/fe_certification/formula_sf/sf_spine", 2)),
        ("gTipConvA", f("/fe_certification/tip_tet10_sf_by_mesh_area_mm2/area_0p4", 3)),
        ("gTipConvB", f("/fe_certification/tip_tet10_sf_by_mesh_area_mm2/area_0p15_production", 3)),
        ("gTipConvC", f("/fe_certification/tip_tet10_sf_by_mesh_area_mm2/area_0p08", 3)),
    ];
    for (edge, tag) in [("edge_inner", "A"), ("edge_spine", "B")] {
        let p = |k: &str| format!("/blade/{edge}/{k}");
        let names = |k: &str| -> &'static str { Box::leak(format!("gEdge{tag}{k}").into_boxed_str()) };
        m.push((names("ApexLo"), f(&p("per_side_angle_apex_deg/0"), 1)));
        m.push((names("ApexHi"), f(&p("per_side_angle_apex_deg/1"), 1)));
        m.push((names("BevelLo"), f(&p("per_side_angle_bevel_deg/0"), 1)));
        m.push((names("BevelHi"), f(&p("per_side_angle_bevel_deg/1"), 1)));
        m.push((names("Steep"), f(&p("steepest_within_2mm_deg"), 1)));
        m.push((names("From"), f(&p("ground_from_mm_from_point"), 1)));
        m.push((names("To"), f(&p("ground_to_mm_from_point"), 1)));
        m.push((names("Plunge"), f(&p("unground_plunge_mm"), 1)));
        m.push((names("Depth"), f(&p("serration_depth_mm"), 2)));
        m.push((names("Teeth"), f(&p("teeth"), 0)));
        m.push((names("ToothSF"), f(&p("tooth_root_sf_min"), 2)));
        m.push((names("Flank"), f(&p("mean_flank_deg"), 0)));
        m.push((names("Arc"), f(&p("edge_arc_mm"), 1)));
        m.push((names("Sharpened"), f(&p("sharpened_arc_mm"), 1)));
    }
    // readings against the targets stated in the text, so prose never asserts an outcome by hand
    let reading = |ok: bool| if ok { "meets the target".to_string() } else { "is below the target".to_string() };
    let tooth = num(&r, "/blade/edge_inner/tooth_root_sf_min").min(num(&r, "/blade/edge_spine/tooth_root_sf_min"));
    m.push(("gMassReading", if num(&r, "/mass/mesh_g") <= 50.0 { "meets the 50 g goal".into() } else { "is above the 50 g goal".into() }));
    m.push(("gTipReading", reading(num(&r, "/inputs/solid_tip_sf") >= 1.0)));
    m.push(("gGripReading", reading(num(&r, "/inputs/grip_sf_min").min(num(&r, "/fe_certification/grip_sf/spine")) >= 1.0)));
    m.push(("gTwistReading", reading(num(&r, "/inputs/hm_twist_sf").min(num(&r, "/fe_certification/root_sf/twist")) >= 1.0)));
    m.push(("gSpineReading", reading(num(&r, "/inputs/hm_spine_sf") >= 1.0)));
    m.push(("gToothReading", reading(tooth >= 1.0)));
    m.push(("gToothMin", format!("{tooth:.2}")));
    m.push(("gFatReading", reading(num(&r, "/inputs/fat_sf") >= 1.0)));
    m.push(("gSeqReading", reading(num(&r, "/inputs/seq_ratio") >= 1.2)));
    m.push(("gLatReading", reading(num(&r, "/inputs/lat_rob_cert_n") >= 260.0)));
    m.push(("gRingReading", reading(num(&r, "/inputs/ring_bearing_cap_n") >= 600.0 && num(&r, "/inputs/ring_back_cap_n") >= 60.0)));
    m.push(("gBuckReading", reading(num(&r, "/inputs/buckling_lambda") >= 1.0)));
    let mut numbers = String::from("% generated by g3-release-tex from evidence/g3/v<VERSION>/release.json; do not edit\n");
    for (k, v) in &m {
        let _ = writeln!(numbers, "\\newcommand{{\\{k}}}{{{v}}}");
    }
    let _ = writeln!(numbers, "\\graphicspath{{{{../v{}/img/}}}}", s("/version"));
    fs::write(out.join("numbers.tex"), numbers).unwrap();

    let mut top = String::from("% generated by g3-release-tex; rows: rank & design & Linux Tier-2 score & mass, both at the ranking grid\n");
    for (k, t) in get(&r, "/cohort/top8_linux").as_array().expect("top8").iter().enumerate() {
        let _ = writeln!(top, "{} & \\texttt{{{}}} & {:.2} & {:.1} \\\\", k + 1, tex(t["design_id"].as_str().unwrap()),
                         t["score_100"].as_f64().unwrap(), t["mass_g"].as_f64().unwrap());
    }
    fs::write(out.join("topeight.tex"), &top).unwrap();

    let mut grid = String::from("% generated by g3-release-tex: design & mass (g) at each grid h of /grid_check/h_mm\n");
    for g in get(&r, "/grid_check/rows").as_array().expect("grid rows") {
        let masses: Vec<String> = g["mass_g"].as_array().unwrap().iter().map(|v| format!("{:.1}", v.as_f64().unwrap())).collect();
        let _ = writeln!(grid, "\\texttt{{{}}} & {} \\\\", tex(g["design_id"].as_str().unwrap()), masses.join(" & "));
    }

    let lin = get(&r, "/evolution/lineage").as_array().expect("lineage");
    let w = 0.98 / lin.len().max(1) as f64;
    let mut l = String::from("% generated by g3-release-tex: best design of the winning island at each logged checkpoint\n");
    for c in lin {
        let _ = writeln!(l, "\\begin{{minipage}}[t]{{{w:.3}\\textwidth}}\\centering\\includegraphics[width=\\linewidth]{{../../../{}}}\\\\[-1pt]\
            {{\\EItablefont eval {}\\\\ {:.2}, {:.1} g{}}}\\end{{minipage}}\\hfill", c["image"].as_str().unwrap(),
            count(c["evals"].as_f64().unwrap()), c["best_score_100"].as_f64().unwrap(), c["best_mass_g"].as_f64().unwrap(),
            c["role"].as_str().map(|r| format!("\\\\ \\textbf{{{r}}}")).unwrap_or_default());
    }
    fs::write(out.join("lineage.tex"), l).unwrap();

    let in_git: Vec<&str> = get(&r, "/cad_in_git").as_array().unwrap().iter().filter_map(|v| v.as_str()).collect();
    let mut asset = String::from("% generated by g3-release-tex: file & triangles & MiB & where\n");
    for x in get(&r, "/release_assets").as_array().expect("assets") {
        let file = x["file"].as_str().unwrap();
        let git = in_git.iter().any(|p| p.ends_with(file));
        let short = file.strip_prefix(&format!("{}_", s("/design_id"))).unwrap_or(file); // the caption names the design
        let _ = writeln!(asset, "\\texttt{{{}}} & {} & {:.0} & {} \\\\", tex(short), count(x["triangles"].as_f64().unwrap()),
                         x["bytes"].as_f64().unwrap() / 1_048_576.0, if git { "git and release" } else { "release" });
    }
    fs::write(out.join("assets.tex"), &asset).unwrap();
    // table rows also as macros: \input inside a tabular breaks \bottomrule ("Misplaced \noalign")
    let rows_macro = |name: &str, body: &str| {
        let rows: String = body.lines().filter(|l| !l.starts_with('%')).collect::<Vec<_>>().join("\n");
        format!("\\newcommand{{\\{name}}}{{%\n{rows}}}\n")
    };
    let mut numbers = fs::read_to_string(out.join("numbers.tex")).unwrap();
    numbers.push_str(&rows_macro("gTopEightRows", &top));
    numbers.push_str(&rows_macro("gAssetRows", &asset));
    numbers.push_str(&rows_macro("gGridRows", &grid));
    fs::write(out.join("numbers.tex"), numbers).unwrap();
    println!("g3-release-tex: {} macros, {} ranking rows, {} lineage frames -> {}", m.len(), get(&r, "/cohort/top8_linux").as_array().unwrap().len(), lin.len(), out.display());
}
