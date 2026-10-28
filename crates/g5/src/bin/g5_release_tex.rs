//! `g5-release-tex`: LaTeX macros for the G5 release paper, generated from the committed release receipt, so the
//! paper states no number by hand (owner, 2026-10-27: README and paper "100 % accurate with the best individual").
//! Deterministic and fail-closed: any missing key exits non-zero.
//!
//!   g5-release-tex <release.json> <out_dir> [release_version]
//!
//! `\gRecord` is the receipt's own version (the cohort of record); `\gVersion` is the release the paper belongs to,
//! which defaults to the receipt's version and differs only for a patch release that keeps the cohort of record.
//!
//! Writes `<out_dir>/numbers.tex` (one `\newcommand` per quantity), `<out_dir>/topeight.tex` (ranking rows),
//! `<out_dir>/lineage.tex` (the lineage strip) and `<out_dir>/assets.tex` (CAD files). A missing field is an error.
use jarambit_g5::g5_genome::G5_NUM_GENES;
use jarambit_g5::loads::G5Loads;
use jarambit_g5::requirements::rs_v5;
use serde_json::Value;
use std::fmt::Write as _;
use std::{fs, path::Path, process::exit};

fn fail(msg: impl std::fmt::Display) -> ! {
    eprintln!("g5-release-tex: FAIL: {msg}");
    exit(2)
}

fn get<'a>(r: &'a Value, ptr: &str) -> &'a Value {
    match r.pointer(ptr) {
        Some(v) if !v.is_null() => v,
        _ => fail(format!("missing {ptr}")),
    }
}

fn num(r: &Value, ptr: &str) -> f64 {
    get(r, ptr)
        .as_f64()
        .unwrap_or_else(|| fail(format!("{ptr} is not a finite number")))
}

fn str_val(r: &Value, ptr: &str) -> String {
    get(r, ptr)
        .as_str()
        .unwrap_or_else(|| fail(format!("{ptr} is not a string")))
        .to_string()
}

/// LaTeX-safe text (identifiers with underscores, hex digests)
fn tex(s: &str) -> String {
    s.replace('\\', "\\textbackslash{}")
        .replace('_', "\\_")
        .replace('%', "\\%")
        .replace('&', "\\&")
        .replace('#', "\\#")
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

/// The v6.0.0 macros: the continuation (warm start, pool lineage, rounds, control), the cross-host re-run, the v5.0.0
/// winner on the same platform, the fitness budget and the GPU spike (`spike/spike.json` beside the receipt).
fn v6_macros(r: &Value, receipt_path: &Path) -> Vec<(&'static str, String)> {
    let f = |ptr: &str, d: usize| format!("{:.*}", d, num(r, ptr));
    let arr = |ptr: &str| get(r, ptr).as_array().unwrap_or_else(|| fail(format!("{ptr} not array"))).clone();
    let lineage = arr("/lineage");
    let (p0, pn) = (&lineage[0], lineage.last().unwrap());
    let rounds = arr("/cohort/rounds");
    let hosts: std::collections::BTreeSet<String> = rounds.iter().map(|x| str_val(x, "/host")).collect();
    let control_best = arr("/baseline/final_best_score_100").iter().map(|v| v.as_f64().unwrap_or_else(|| fail("control score"))).fold(f64::NEG_INFINITY, f64::max);
    let lead_rank = get(r, "/ranking/search_leader/tier2_rank");
    let spike_path = receipt_path.parent().expect("receipt dir").join("spike/spike.json");
    let sp: Value = serde_json::from_str(&fs::read_to_string(&spike_path).unwrap_or_else(|e| fail(format!("{}: {e}", spike_path.display()))))
        .unwrap_or_else(|e| fail(format!("{}: {e}", spike_path.display())));
    let spf = |ptr: &str, d: usize| format!("{:.*}", d, num(&sp, ptr));
    let sizing_path = receipt_path.parent().expect("receipt dir").join("fleet/sizing.json");
    let sz: Value = serde_json::from_str(&fs::read_to_string(&sizing_path).unwrap_or_else(|e| fail(format!("{}: {e}", sizing_path.display()))))
        .unwrap_or_else(|e| fail(format!("{}: {e}", sizing_path.display())));
    if sz["outputs_identical_tunables"] != serde_json::json!(true) {
        fail("fleet/sizing.json: the malloc tunables changed an output");
    }
    vec![
        ("gVFiveElites", count(num(r, "/warm_start/v5_elites"))),
        ("gLaunchTierOneTopPct", format!("{:.0}", 100.0 * num(r, "/cohort/promotion/tier1_top"))),
        ("gArchiveNiches", count(num(p0, "/archive_niches"))),
        ("gPoolZeroNiches", count(num(p0, "/niches"))),
        ("gPoolZeroBest", format!("{:.2}", num(p0, "/best_raw_score_100"))),
        ("gPoolFinalNiches", count(num(pn, "/niches"))),
        ("gPoolFinalBest", format!("{:.2}", num(pn, "/best_raw_score_100"))),
        ("gPools", count(lineage.len() as f64)),
        ("gFinalPool", format!("\\texttt{{{}}}", tex(&str_val(r, "/cohort/final_pool")))),
        ("gRounds", count(rounds.len() as f64)),
        ("gRoundDepth", count(num(r, "/cohort/round_depth_max"))),
        ("gHostCount", count(hosts.len() as f64)),
        ("gHostList", tex(&hosts.into_iter().collect::<Vec<_>>().join(", "))),
        ("gControlIslands", count(num(r, "/baseline/islands"))),
        ("gControlEvals", count(num(r, "/baseline/evaluations_per_island"))),
        ("gControlBest", format!("{control_best:.2}")),
        ("gReproIsland", format!("\\texttt{{{}}}", tex(&str_val(r, "/cohort/reproduced/island")))),
        ("gReproHost", tex(&str_val(r, "/cohort/reproduced/rerun/rerun_host"))),
        ("gReproEvals", count(num(r, "/cohort/reproduced/rerun/evaluations"))),
        ("gReproEntries", count(num(r, "/cohort/reproduced/history_entries_identical"))),
        ("gWinnerSearchScore", f("/search_score_100", 2)),
        ("gWinOrigin", tex(&str_val(r, "/evolution/winner_island"))),
        ("gSearchLeaderId", format!("\\texttt{{{}}}", tex(&str_val(r, "/ranking/search_leader/design_id")))),
        ("gSearchLeaderTierOne", f("/ranking/search_leader/tier1_h04_score_100", 2)),
        ("gSearchLeaderRank", lead_rank.as_f64().map_or("outside the shortlist".to_string(), |k| format!("rank {}", count(k)))),
        ("gVFiveId", format!("\\texttt{{{}}}", tex(&str_val(r, "/ranking/v5_winner_same_platform/design_id")))),
        ("gVFiveScore", f("/ranking/v5_winner_same_platform/score/score_100", 2)),
        ("gVFiveTierOne", f("/ranking/v5_winner_same_platform/tier1_h04_score_100", 2)),
        ("gVFiveWorst", f("/ranking/v5_winner_same_platform/worst_margin", 3)),
        ("gVFiveMass", f("/ranking/v5_winner_same_platform/mass_g", 1)),
        ("gVFiveMet", count(num(r, "/ranking/v5_winner_same_platform/cases_met"))),
        ("gVFiveE", f("/ranking/v5_winner_same_platform/score/E", 3)),
        ("gVFiveLg", f("/ranking/v5_winner_same_platform/grip_length_mm", 1)),
        ("gVFivePerim", f("/ranking/v5_winner_same_platform/grip_perimeter_mm", 1)),
        ("gCasesMet", count(num(r, "/cases_met"))),
        ("gCases", count(num(r, "/cases"))),
        ("gBudgetEarned", f("/fitness_budget/earned", 1)),
        ("gBudgetStruct", f("/fitness_budget/groups/structural", 1)),
        ("gBudgetMass", f("/fitness_budget/groups/mass", 1)),
        ("gBudgetCut", f("/fitness_budget/groups/cutting", 1)),
        ("gBudgetBuild", f("/fitness_budget/groups/build", 1)),
        ("gBudgetLost", f("/fitness_budget/lost_to_multipliers", 1)),
        ("gBudgetUnmet", f("/fitness_budget/unmet_credit", 1)),
        ("gSpikeEdtPct", spf("/profile/euclidean_distance_transform_pct", 0)),
        ("gSpikeConnPct", spf("/profile/am_connectivity_pct", 0)),
        ("gSpikePlatePct", spf("/profile/plate_skyline_solve_pct", 0)),
        ("gSpikeKernelBound", spf("/bounds/voxel_kernel_offload_max_speedup_tier1", 2)),
        ("gSpikeScreenBound", spf("/bounds/perfect_free_screen_top10_speedup", 2)),
        ("gSpikeSpearman", spf("/screen/spearman_tier0_tier1", 2)),
        ("gSpikeKept", count(num(&sp, "/screen/tier1_top10_kept_by_tier0_top40"))),
        ("gSpikeTopTen", count(num(&sp, "/screen/tier1_top10"))),
        ("gSpikeTierOneShare", format!("{:.0}", 100.0 * num(&sp, "/screen/share_of_island_time_in_tier1"))),
        ("gSpikeTierZeroS", spf("/screen/mean_tier0_s", 2)),
        ("gSpikeTierOneS", spf("/screen/mean_tier1_s", 1)),
        ("gSpikeChildren", count(num(&sp, "/screen/n"))),
        ("gSizingGenomes", count(num(&sz, "/genomes"))),
        ("gSizingOneS", format!("{:.1}", num(&sz, "/one_per_ccx_s_per_eval"))),
        ("gSizingFourS", format!("{:.1}", num(&sz, "/four_per_ccx_s_per_eval"))),
        ("gSizingGain", format!("{:.2}", num(&sz, "/throughput_gain_four_vs_one_per_ccx"))),
        ("gSizingFaultRatio", format!("{:.1}", num(&sz, "/fault_ratio"))),
        // comparisons in words, so the text stays true whatever the numbers are
        ("gVsVFiveScore", cmp_word(num(r, "/score/score_100"), num(r, "/ranking/v5_winner_same_platform/score/score_100"), "higher than", "lower than", "equal to")),
        ("gVsVFiveMass", cmp_word(num(r, "/mass/of_record_g"), num(r, "/ranking/v5_winner_same_platform/mass_g"), "heavier than", "lighter than", "as heavy as")),
        ("gLineageVsControl", cmp_word(num(pn, "/best_raw_score_100"), control_best, "above", "below", "level with")),
        ("gLineageGain", format!("{:.2}", num(pn, "/best_raw_score_100") - num(p0, "/best_raw_score_100"))),
    ]
}

/// "higher than" when a > b, "lower than" when a < b, else the third word.
fn cmp_word(a: f64, b: f64, more: &str, less: &str, same: &str) -> String {
    (if a > b { more } else if a < b { less } else { same }).to_string()
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() != 3 && a.len() != 4 {
        fail("usage: g5-release-tex <release.json> <out_dir> [release_version]");
    }
    let receipt_path = Path::new(&a[1]);
    let r: Value = serde_json::from_str(
        &fs::read_to_string(receipt_path).unwrap_or_else(|e| fail(format!("{}: {e}", receipt_path.display()))),
    )
    .unwrap_or_else(|e| fail(format!("{}: {e}", receipt_path.display())));
    let out = Path::new(&a[2]);
    fs::create_dir_all(out).unwrap_or_else(|e| fail(format!("{}: {e}", out.display())));

    let rs = rs_v5();
    let f = |ptr: &str, d: usize| format!("{:.*}", d, num(&r, ptr));

    let loads: G5Loads = serde_json::from_value(get(&r, "/loads").clone())
        .unwrap_or_else(|e| fail(format!("invalid /loads: {e}")));
    // B1: the margins of record are the minimum over the plate check meshes
    let marg_obj = get(&r, "/margins_min_over_plate_meshes")
        .as_object()
        .unwrap_or_else(|| fail("/margins_min_over_plate_meshes is not an object"));
    let (worst_name, _) = marg_obj
        .iter()
        .map(|(k, v)| {
            (
                k.as_str(),
                v.as_f64().unwrap_or_else(|| fail(format!("/margins_min_over_plate_meshes/{k} not a number"))),
            )
        })
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .unwrap_or_else(|| fail("empty /margins"));
    let worst_v = num(&r, "/worst_margin");
    let w_mult = num(&r, "/score/worst_margin_W");

    // the receipt's E is the hand multiplier itself, (prod_j e_j^beta_j)^e_exponent; gEGeo is the product inside
    let e_mult = num(&r, "/score/E");
    let e_inner = e_mult.powf(1.0 / rs.objective.e_exponent);
    let v6 = r.get("lineage").is_some();

    let commit_label = str_val(&r, "/provenance/decoder_commit_label");
    let dec_full = str_val(&r, "/provenance/decoder_sha256");
    let gen_full = str_val(&r, "/provenance/genome_sha256");

    let mass_rec = num(&r, "/mass/of_record_g");
    let mass_mesh = num(&r, "/mass/print_mesh_g");
    let env_mm3 = num(&r, "/mass/envelope_mesh_volume_mm3");
    let density = num(&r, "/mass/density_g_mm3");

    let win_island = if v6 { String::new() } else { str_val(&r, "/evolution/winner_island") };
    let base_isl = num(&r, "/baseline/islands");
    let base_evals_per = num(&r, "/baseline/evaluations_per_island");
    let base_scores = get(&r, if v6 { "/baseline/final_best_score_100" } else { "/evolution/baseline_final_best_score_100" })
        .as_array()
        .unwrap_or_else(|| fail("baseline final best scores not an array"));
    let base_best = base_scores
        .iter()
        .map(|v| v.as_f64().unwrap_or_else(|| fail("baseline score not number")))
        .fold(f64::NEG_INFINITY, f64::max);

    let port_n = num(&r, "/decoded/port_n").round();

    let mut m: Vec<(&str, String)> = vec![
        ("gVersion", a.get(3).cloned().unwrap_or_else(|| str_val(&r, "/version"))),
        ("gRecord", str_val(&r, "/version")),
        ("gDesign", format!("\\texttt{{{}}}", tex(&str_val(&r, "/design_id")))),
        ("gStatus", tex(&str_val(&r, "/status"))),
        ("gCommit", format!("\\texttt{{{}}}", tex(&commit_label))),
        ("gDecoder", format!("\\texttt{{{}}}", &dec_full[..dec_full.len().min(12)])),
        ("gGenomeSha", format!("\\texttt{{{}}}", &gen_full[..gen_full.len().min(12)])),
        ("gGeneCount", G5_NUM_GENES.to_string()),
        ("gScore", f("/score/score_100", 2)),
        ("gBase", f("/score/base", 3)),
        ("gE", format!("{e_mult:.3}")),
        ("gEGeo", format!("{e_inner:.3}")),
        ("gW", format!("{w_mult:.3}")),
        ("gWorstMarginRatio", format!("{worst_v:.3}")),
        ("gWorstMarginProdMesh", f("/worst_margin_production_mesh", 3)),
        ("gPlateCheckMeshes", get(&r, "/plate_mesh_check/max_area_mm2").as_array().unwrap_or_else(|| fail("plate_mesh_check"))
            .iter().map(|a| format!("{}", a.as_f64().unwrap_or_else(|| fail("plate mesh area")))).collect::<Vec<_>>().join("/")),
        ("gWorstMarginCase", tex(worst_name)),
        ("gMassFinishedG", format!("{mass_rec:.2}")),
        ("gMassG", format!("{mass_mesh:.2}")),
        ("gVolEnv", count(env_mm3)),
        ("gMassEnvG", format!("{:.1}", env_mm3 * density)),
        (
            "gMassReading",
            if mass_rec <= rs.mass.goal_g {
                format!("meets the {:.0} g goal (frontier)", rs.mass.goal_g)
            } else if mass_rec <= rs.mass.ceiling_g {
                format!(
                    "is above the {:.0} g goal (frontier) and under the {:.0} g ceiling",
                    rs.mass.goal_g, rs.mass.ceiling_g
                )
            } else {
                format!("exceeds the {:.0} g ceiling", rs.mass.ceiling_g)
            },
        ),
        ("gHost", tex(&str_val(&r, "/provenance/platform_of_record"))),
        ("gIslands", count(num(&r, "/cohort/islands"))),
        ("gEvalsIsland", count(num(&r, "/cohort/evaluations_per_island"))),
        ("gEvalsIslandRaw", f("/cohort/evaluations_per_island", 0)),
        ("gEvalsTotal", count(num(&r, "/cohort/evaluations_total"))),
        ("gCohortElites", count(num(&r, "/cohort/elites"))),
        ("gTierOneReproduced", count(num(&r, "/cohort/tier1_rows_reproduced"))),
        ("gH", f("/cohort/h_mm", 1)),
        ("gHRank", f("/ranking/h_mm", 1)),
        ("gShortlist", count(num(&r, "/ranking/shortlist"))),
        ("gSealedVoid", f("/am/sealed_void_mm3", 1)),
        ("gCreviceVoid", f("/am/crevice_mm3", 1)),
        ("gPortCount", format!("{port_n:.0}")),
        ("gPortD", f("/decoded/port_d", 2)),
        ("gLg", f("/ergonomics/grip_length_mm", 1)),
        ("gSwellPerim", f("/ergonomics/grip_perimeter_mm", 1)),
        ("gHoT", f("/ergonomics/grip_h_over_t", 2)),
        ("gGuardH", f("/ergonomics/guard_height_mm", 1)),
        ("gComIdx", f("/ergonomics/com_to_index_mm", 1)),
        ("gReach", f("/blade/reach_chord_mm", 1)),
        ("gPointAngleOne", f("/point/angle_1mm_deg", 1)),
        ("gPointAngleThree", f("/point/angle_3mm_deg", 1)),
        ("gPointWidthTwo", f("/point/width_2mm", 2)),
        ("gEdgeAAngle", f("/blade/edge_inner/per_side_angle_gene_deg", 1)),
        ("gEdgeBAngle", f("/blade/edge_spine/per_side_angle_gene_deg", 1)),
        ("gEdgeMaxAngle", f("/blade/max_half_angle_deg", 1)),
        ("gEdgeMaxApex", f("/blade/max_apex_mm", 3)),
        ("gEdgeADepth", f("/blade/edge_inner/serration_depth_mm", 2)),
        ("gEdgeATeeth", f("/blade/edge_inner/teeth", 0)),
        ("gEdgeBDepth", f("/blade/edge_spine/serration_depth_mm", 2)),
        ("gEdgeBTeeth", f("/blade/edge_spine/teeth", 0)),
        ("gTexAmp", f("/decoded/tex_amp", 2)),
        ("gTexPitch", f("/decoded/tex_pitch", 2)),
        ("gScopeBlade", f("/decoded/scope_blade", 2)),
        ("gScopeBladeThree", f("/decoded/scope_blade", 3)),
    ];

    if v6 {
        m.extend(v6_macros(&r, receipt_path));
    } else {
        let evo_islands = get(&r, "/evolution/islands").as_array().unwrap_or_else(|| fail("/evolution/islands not array"));
        let win_isl_obj = evo_islands
            .iter()
            .find(|i| str_val(i, "/island") == win_island)
            .unwrap_or_else(|| fail(format!("winner_island {win_island} missing in /evolution/islands")));
        let score_sobol = num(win_isl_obj, "/after_sobol_init/best_score_100");
        let score_search = num(win_isl_obj, "/final/best_score_100");
        let v5: Vec<(&str, String)> = vec![
        ("gStarted", tex(&str_val(&r, "/cohort/started_utc"))),
            ("gFinished", tex(&str_val(&r, "/cohort/finished_utc"))),
            ("gArchiveNiches", count(num(&r, "/cohort/launch/archive_niches"))),
            ("gLaunchInit", count(num(&r, "/cohort/launch/cohort/init"))),
            ("gLaunchHost", f("/cohort/launch/cohort/host", 0)),
            (
                "gLaunchTierOneTopPct",
                format!("{:.0}", 100.0 * num(&r, "/cohort/launch/cohort/tier1_top")),
            ),
            (
                "gLaunchTierOneTop",
                format!("{}", num(&r, "/cohort/launch/cohort/tier1_top")),
            ),
            (
                "gLaunchTierTwoTop",
                format!("{}", num(&r, "/cohort/launch/cohort/tier2_top")),
            ),
            ("gLaunchLogEvery", f("/cohort/launch/cohort/log_every", 0)),
            ("gReproEvals", count(num(&r, "/cohort/reproduced/evaluations"))),
            ("gReproEntries", count(num(&r, "/cohort/reproduced/history_entries_identical"))),
            ("gBaselineIslands", count(base_isl)),
            ("gBaselineEvals", count(base_isl * base_evals_per)),
            ("gBaselineBest", format!("{base_best:.2}")),
            ("gBeatingBaseline", count(num(&r, "/evolution/cohort_islands_beating_best_baseline"))),
            ("gCohortSobolMed", f("/evolution/cohort_after_sobol_init_median_score_100", 2)),
            ("gCohortMed", f("/evolution/cohort_final_best_score_100/median", 2)),
            ("gCohortMin", f("/evolution/cohort_final_best_score_100/min", 2)),
            ("gCohortMax", f("/evolution/cohort_final_best_score_100/max", 2)),
            ("gWinIsland", tex(&win_island)),
            ("gWinFoundEvals", count(num(&r, "/evolution/winner_first_island_best_at_evals"))),
            ("gScoreSobol", format!("{score_sobol:.2}")),
            ("gScoreSearch", format!("{score_search:.2}")),
            ("gWinnerSearchScore", f("/search_score_100", 2)),
            (
                "gSearchLeaderId",
                format!("\\texttt{{{}}}", tex(&str_val(&r, "/ranking/search_leader/design_id"))),
            ),
            ("gSearchLeaderTierOne", f("/ranking/search_leader/tier1_h04_score_100", 2)),
            ("gSearchLeaderTierTwo", f("/ranking/search_leader/tier2_h02_score_100", 2)),
            ("gSearchLeaderRank", count(num(&r, "/ranking/search_leader/tier2_rank"))),
            ("gSearchLeaderMargin", f("/ranking/search_leader/tier2_worst_margin", 2)),
            ("gSearchLeaderTipSecAx", f("/ranking/search_leader/tip_sf_sections_h04/axial", 2)),
            ("gSearchLeaderTipSolAx", f("/ranking/search_leader/tip_sf_solid_h02/axial", 2)),
            ("gLinuxDesigns", count(num(&r, "/ranking/linux_check/designs"))),
            ("gLinuxMaxRelDiff", format!("{:.2e}", num(&r, "/ranking/linux_check/max_rel_diff"))),
            ("gLinuxMaxRelDiffPct", format!("{:.2}", 100.0 * num(&r, "/ranking/linux_check/max_rel_diff"))),
            ("gLinuxWinRank", count(num(&r, "/ranking/linux_check/winner_rank_linux"))),
            ("gLinuxEvoEvals", count(num(&r, "/ranking/linux_check/evolution/same_best_genome_through_evals"))),
            (
                "gLinuxEvoRelDiff",
                format!(
                "{:.2e}",
                    num(&r, "/ranking/linux_check/evolution/max_rel_diff_best_score_while_same_genome")
            ),
            ),
        ];
        m.extend(v5);
    }

    // 16 RS-karambit-v5 load cases: SF on the production plate mesh, margin of record v = min over the plate check meshes
    let load_macros: [(&str, &str, f64, &str); 16] = [
        ("gLoadLatLim", "gMarginLatLim", loads.lateral_limit_sf.min(loads.grip_lateral_sf), "/margins_min_over_plate_meshes/lateral_limit"),
        ("gLoadLatUlt", "gMarginLatUlt", loads.lateral_ultimate_sf, "/margins_min_over_plate_meshes/lateral_ultimate"),
        ("gLoadTwistLim", "gMarginTwistLim", loads.twist_limit_sf.min(loads.grip_twist_sf), "/margins_min_over_plate_meshes/twist_limit"),
        ("gLoadTipAxial", "gMarginTipAxial", loads.tip_axial_sf, "/margins_min_over_plate_meshes/tip_axial"),
        ("gLoadTipLat", "gMarginTipLat", loads.tip_lateral_sf, "/margins_min_over_plate_meshes/tip_lateral"),
        ("gLoadTipTwist", "gMarginTipTwist", loads.tip_twist_sf, "/margins_min_over_plate_meshes/tip_twist"),
        ("gLoadGripSqueeze", "gMarginGripSqueeze", loads.grip_squeeze_sf, "/margins_min_over_plate_meshes/grip_squeeze"),
        ("gLoadGuardPush", "gMarginGuardPush", loads.guard_push_sf, "/margins_min_over_plate_meshes/guard_push"),
        ("gLoadRingBear", "gMarginRingBear", loads.ring_bearing_sf, "/margins_min_over_plate_meshes/ring_bearing"),
        ("gLoadRingBack", "gMarginRingBack", loads.ring_back_sf, "/margins_min_over_plate_meshes/ring_back"),
        ("gLoadRingAny", "gMarginRingAny", loads.ring_any_direction_sf, "/margins_min_over_plate_meshes/ring_any_direction"),
        ("gLoadPull", "gMarginPull", loads.pull_sf.min(loads.grip_pull_sf), "/margins_min_over_plate_meshes/pull"),
        ("gLoadTeeth", "gMarginTeeth", loads.teeth_sf, "/margins_min_over_plate_meshes/teeth"),
        ("gLoadFatigue", "gMarginFatigue", loads.fatigue_sf.min(loads.hand_fatigue_sf), "/margins_min_over_plate_meshes/fatigue"),
        ("gLoadSeqMin", "gMarginSeq", loads.seq_min(), "/margins_min_over_plate_meshes/sequence"),
        ("gLoadBuckling", "gMarginBuckling", loads.buckling_lambda, "/margins_min_over_plate_meshes/buckling"),
    ];
    for (sf_k, m_k, sf_val, m_ptr) in load_macros {
        m.push((sf_k, format!("{sf_val:.2}")));
        m.push((m_k, f(m_ptr, 3)));
    }

    // Mass-vs-ergonomics trade at the winning design under W_ERGOLIGHT_v2
    let pts_per_g = 100.0 * e_mult * w_mult * rs.objective.mass_weight / (rs.mass.ceiling_g - rs.mass.floor_g);
    let grams_per_pct_e = rs.objective.grams_per_percent_e(num(&r, "/score/base"), &rs.mass);
    m.push(("gPtsPerGram", format!("{pts_per_g:.3}")));
    m.push(("gGramsPerPctE", format!("{grams_per_pct_e:.1}")));

    // Ranking & grid check tables
    let top_arr = get(&r, "/ranking/rows")
        .as_array()
        .unwrap_or_else(|| fail("/ranking/rows not array"));
    m.push(("gTopRanked", top_arr.len().to_string()));
    let n_show = top_arr.len().min(8);
    if n_show == 0 {
        fail("empty /ranking/rows");
    }
    m.push((
        "gTopLast",
        format!("{:.2}", num(&top_arr[n_show - 1], "/score_100")),
    ));

    let mut top = String::from("% generated by g5-release-tex; rows: rank & design & Tier-2 score & W & mass\n");
    for t in top_arr.iter().take(8) {
        let _ = writeln!(
            top,
            "{:.0} & \\texttt{{{}}} & {:.2} & {:.3} & {:.1} \\\\",
            num(t, "/rank"),
            tex(&str_val(t, "/design_id")),
            num(t, "/score_100"),
            num(t, "/worst_margin_W"),
            num(t, "/mass_g")
        );
    }
    fs::write(out.join("topeight.tex"), &top).unwrap();

    let gc_rows = get(&r, "/grid_check/rows")
        .as_array()
        .unwrap_or_else(|| fail("/grid_check/rows not array"));
    let mut max_h02_h015_pct = 0.0_f64;
    let mut max_h04_h02_pct = 0.0_f64;
    let mut grid = String::from("% generated by g5-release-tex: design & mass (g) at each grid h\n");
    for g in gc_rows {
        let masses: Vec<f64> = get(g, "/mass_g")
            .as_array()
            .unwrap_or_else(|| fail("grid_check row /mass_g not array"))
            .iter()
            .map(|v| v.as_f64().unwrap_or_else(|| fail("grid mass not number")))
            .collect();
        if masses.len() != 5 {
            fail("grid_check row must have 5 masses");
        }
        let d_fine = num(g, "/mass_rel_h020_vs_h015") * 100.0;
        let d_span = ((masses[0] - masses[3]).abs() / masses[3].max(1e-9)) * 100.0;
        max_h02_h015_pct = max_h02_h015_pct.max(d_fine);
        max_h04_h02_pct = max_h04_h02_pct.max(d_span);
        let m_str: Vec<String> = masses.iter().map(|v| format!("{v:.1}")).collect();
        let _ = writeln!(
            grid,
            "\\texttt{{{}}} & {} \\\\",
            tex(&str_val(g, "/design_id")),
            m_str.join(" & ")
        );
    }
    m.push(("gGridAgree", format!("{max_h02_h015_pct:.2}")));
    m.push(("gGridSpanPct", format!("{max_h04_h02_pct:.2}")));

    // Verification & validation receipts (the receipt's provenance.vv, relative to the repository root)
    let ev_dir = receipt_path.parent().expect("receipt dir");
    let repo_root = ev_dir
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .unwrap_or_else(|| fail("cannot resolve repo root from receipt path"));
    let vv_dir = repo_root.join(str_val(&r, "/provenance/vv"));
    let read_vv = |name: &str| -> Value {
        let p = vv_dir.join(format!("{name}.json"));
        serde_json::from_str(&fs::read_to_string(&p).unwrap_or_else(|e| fail(format!("{}: {e}", p.display()))))
            .unwrap_or_else(|e| fail(format!("{}: {e}", p.display())))
    };
    let plate_vv = read_vv("plate_mesh");
    let tip_vv = read_vv("tip_mesh");
    let checks_vv = read_vv("checks");
    let targets_vv = read_vv("decoder_targets");
    let p_rows = get(&plate_vv, "/rows").as_array().unwrap_or_else(|| fail("plate_mesh rows"));
    let p_row = get(&p_rows[p_rows.len().min(3) - 1], "/plate")
        .as_array()
        .unwrap_or_else(|| fail("plate array"));
    // plate_mesh.json has areas [1.2, 0.6, 0.3, 0.15] -> indices 0, 2, 3 are 1.2, 0.3, 0.15
    m.push(("gVvPlateLatA", format!("{:.2}", num(&p_row[0], "/lateral_sf"))));
    m.push(("gVvPlateLatB", format!("{:.2}", num(&p_row[2], "/lateral_sf"))));
    m.push(("gVvPlateLatC", format!("{:.2}", num(&p_row[3], "/lateral_sf"))));
    m.push(("gVvPlateTwistA", format!("{:.2}", num(&p_row[0], "/twist_sf"))));
    m.push(("gVvPlateTwistB", format!("{:.2}", num(&p_row[2], "/twist_sf"))));
    m.push(("gVvPlateTwistC", format!("{:.2}", num(&p_row[3], "/twist_sf"))));
    m.push(("gVvPlateProdArea", format!("{:.1}", num(&plate_vv, "/production_max_area_mm2"))));
    let mut max_lat_spread_pct = 0.0_f64;
    let mut max_tw_spread_pct = 0.0_f64;
    for r_pl in p_rows {
        let pl = get(r_pl, "/plate").as_array().unwrap_or_else(|| fail("plate array"));
        let lats: Vec<f64> = pl.iter().map(|x| num(x, "/lateral_sf")).collect();
        let tws: Vec<f64> = pl.iter().map(|x| num(x, "/twist_sf")).collect();
        let l_min = lats.iter().copied().fold(f64::INFINITY, f64::min);
        let l_max = lats.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let t_min = tws.iter().copied().fold(f64::INFINITY, f64::min);
        let t_max = tws.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        max_lat_spread_pct = max_lat_spread_pct.max(100.0 * (l_max - l_min) / l_min);
        max_tw_spread_pct = max_tw_spread_pct.max(100.0 * (t_max - t_min) / t_min);
    }
    m.push(("gVvPlateLatSpreadPct", format!("{max_lat_spread_pct:.1}")));
    m.push(("gVvPlateTwistSpreadPct", format!("{max_tw_spread_pct:.1}")));

    let t_rows = get(&tip_vv, "/rows").as_array().unwrap_or_else(|| fail("tip_mesh rows"));
    let t_row = &t_rows[t_rows.len().min(3) - 1];
    let t_sec = get(t_row, "/sections");
    let t_sol = get(t_row, "/solid").as_array().unwrap_or_else(|| fail("tip solid array"));
    m.push(("gVvTipAxA", format!("{:.2}", num(t_sec, "/axial_sf"))));
    m.push(("gVvTipAxB", format!("{:.2}", num(&t_sol[0], "/axial_sf"))));
    m.push(("gVvTipAxC", format!("{:.2}", num(&t_sol[1], "/axial_sf"))));
    m.push(("gVvTipLatA", format!("{:.2}", num(t_sec, "/lateral_sf"))));
    m.push(("gVvTipLatB", format!("{:.2}", num(&t_sol[0], "/lateral_sf"))));
    m.push(("gVvTipLatC", format!("{:.2}", num(&t_sol[1], "/lateral_sf"))));
    m.push(("gVvTipTwA", format!("{:.2}", num(t_sec, "/twist_sf"))));
    m.push(("gVvTipTwB", format!("{:.2}", num(&t_sol[0], "/twist_sf"))));
    m.push(("gVvTipTwC", format!("{:.2}", num(&t_sol[1], "/twist_sf"))));

    let perim_err_pct = num(&targets_vv, "/max_perimeter_rel_err") * 100.0;
    let lloyd_q = num(&checks_vv, "/cvt_1024/lloyd_quantisation_error");
    let halton_q = num(&checks_vv, "/cvt_1024/halton_quantisation_error");
    let grams_per_pct_e = num(&checks_vv, "/objective/grams_per_percent_e_at_base_0_6");
    let ritz_err = num(&checks_vv, "/ritz_column/rel_err").abs();
    m.push(("gVvPerimErrPct", format!("{perim_err_pct:.2}")));
    m.push(("gVvLloydErr", format!("{lloyd_q:.3}")));
    m.push(("gVvHaltonErr", format!("{halton_q:.3}")));
    m.push(("gVvGramsPerPctE", format!("{grams_per_pct_e:.1}")));
    m.push(("gVvRitzErr", format!("{ritz_err:.1e}")));

    // RS-karambit-v5.toml requirement macros (from compiled-in strict loader)
    let rs = jarambit_g5::requirements::rs_v5();
    m.push(("gReqMassCeil", format!("{:.0}", rs.mass.ceiling_g)));
    m.push(("gReqMassFloor", format!("{:.0}", rs.mass.floor_g)));
    m.push(("gReqMassGoal", format!("{:.0}", rs.mass.goal_g)));
    m.push(("gReqPortD", format!("{:.1}", rs.am.port_d_min_mm)));
    m.push(("gReqLg", format!("{:.0}", rs.ergonomics.grip.length_min_mm)));
    m.push(("gReqPerimMin", format!("{:.0}", rs.ergonomics.grip.perimeter_min_mm)));
    m.push(("gReqPerimMax", format!("{:.0}", rs.ergonomics.grip.perimeter_max_mm)));
    m.push(("gReqHoTMin", format!("{:.1}", rs.ergonomics.grip.height_over_thickness_min)));
    m.push(("gReqHoTMax", format!("{:.1}", rs.ergonomics.grip.height_over_thickness_max)));
    m.push(("gReqGuardH", format!("{:.0}", rs.ergonomics.guard.height_target_mm)));
    m.push(("gReqComIdx", format!("{:.0}", rs.ergonomics.balance.com_to_index_max_mm)));
    m.push(("gReqPtOne", format!("{:.0}", rs.point.angle_max_at_1mm_deg)));
    m.push(("gReqPtThree", format!("{:.0}", rs.point.angle_max_at_3mm_deg)));
    m.push(("gReqEdgeMin", format!("{:.0}", rs.edge.per_side_angle_min_deg)));
    m.push(("gReqEdgeMax", format!("{:.0}", rs.edge.per_side_angle_max_deg)));
    m.push(("gReqEdgeFinishMm", format!("{:.1}", rs.edge.finishing_allowance_mm)));
    m.push(("gReqSerrMin", format!("{:.1}", rs.serration.depth_min_mm)));
    m.push(("gReqSerrMax", format!("{:.1}", rs.serration.depth_max_mm)));
    m.push(("gReqScallopsMin", format!("{}", rs.serration.full_depth_scallops_min)));
    m.push(("gReqGulletR", format!("{:.1}", rs.serration.gullet_radius_min_mm)));
    m.push(("gReqSfYield", format!("{:.2}", rs.margins.sf_yield)));
    m.push(("gReqSfUlt", format!("{:.2}", rs.margins.sf_ultimate)));
    m.push(("gReqSfRing", format!("{:.2}", rs.margins.sf_ring)));
    m.push(("gReqSfFat", format!("{:.2}", rs.fatigue.sf_fatigue)));
    m.push(("gReqSeqRatio", format!("{:.2}", rs.sequence.ratio_min)));
    m.push(("gReqBuckling", format!("{:.2}", rs.buckling.lambda_min)));
    m.push(("gReqLatLimNm", format!("{:.0}", rs.loads.lateral_bend.limit_root_moment_nm)));
    m.push(("gReqLatUltNm", format!("{:.0}", rs.loads.lateral_bend.ultimate_root_moment_nm)));
    m.push(("gReqTwistLimNm", format!("{:.1}", rs.loads.body_twist.limit_nm)));
    m.push(("gReqTipAxN", format!("{:.0}", rs.loads.tip.axial_n)));
    m.push(("gReqTipLatN", format!("{:.0}", rs.loads.tip.lateral_n)));
    m.push(("gReqTipTwNm", format!("{:.2}", rs.loads.tip.twist_nm)));
    m.push(("gTipAxFromMm", format!("{:.1}", rs.loads.tip.axial_no_yield_from_mm)));
    let tip_lat_hi = rs
        .loads
        .tip
        .lateral_check_stations_mm
        .iter()
        .copied()
        .fold(0.0_f64, f64::max)
        + 0.5;
    m.push(("gTipLatHiMm", format!("{tip_lat_hi:.1}")));
    m.push(("gTipTwApplyMm", format!("{:.1}", rs.loads.tip.twist_apply_mm_from_point)));
    m.push(("gTipTwReadLoMm", format!("{:.1}", rs.loads.tip.twist_apply_mm_from_point + 1.5)));
    m.push(("gReqGripSqN", format!("{:.0}", rs.loads.grip.limit_squeeze_n)));
    m.push(("gReqGuardPushN", format!("{:.0}", rs.loads.guard.limit_push_n)));
    m.push(("gReqRingBearN", format!("{:.0}", rs.loads.ring.bearing_n)));
    m.push(("gReqRingArcDeg", format!("{:.0}", rs.loads.ring.bearing_arc_deg / 2.0)));
    m.push(("gReqRingBackN", format!("{:.0}", rs.loads.ring.back_n)));
    m.push(("gReqRingAnyN", format!("{:.0}", rs.loads.ring.any_direction_n)));
    m.push(("gReqPullN", format!("{:.0}", rs.loads.pull.limit_n)));
    m.push(("gReqSnagN", format!("{:.0}", rs.loads.teeth.snag_n)));
    m.push(("gReqPryN", format!("{:.0}", rs.loads.teeth.pry_n)));
    m.push(("gWMass", format!("{:.4}", rs.objective.mass_weight)));
    m.push(("gWPoint", format!("{:.4}", rs.objective.weights.point)));
    m.push(("gWAmBuild", format!("{:.4}", rs.objective.weights.am_build)));
    m.push(("gWFatigue", format!("{:.4}", rs.objective.weights.fatigue)));
    m.push(("gWSequence", format!("{:.4}", rs.objective.weights.sequence)));
    m.push(("gWEdge", format!("{:.4}", rs.objective.weights.edge)));
    m.push(("gWLatLim", format!("{:.4}", rs.objective.weights.lateral_limit)));
    m.push(("gWLatUlt", format!("{:.4}", rs.objective.weights.lateral_ultimate)));
    m.push(("gWTwistLim", format!("{:.4}", rs.objective.weights.twist_limit)));
    m.push(("gWTipAx", format!("{:.4}", rs.objective.weights.tip_axial)));
    m.push(("gWTipLat", format!("{:.4}", rs.objective.weights.tip_lateral)));
    m.push(("gWTipTw", format!("{:.4}", rs.objective.weights.tip_twist)));
    m.push(("gWGripSq", format!("{:.4}", rs.objective.weights.grip_squeeze)));
    m.push(("gWGuardPush", format!("{:.4}", rs.objective.weights.guard_push)));
    m.push(("gWRingBear", format!("{:.4}", rs.objective.weights.ring_bearing)));
    m.push(("gWTeeth", format!("{:.4}", rs.objective.weights.teeth)));
    m.push(("gWForm", format!("{:.4}", rs.objective.weights.form)));
    m.push(("gWRingBack", format!("{:.4}", rs.objective.weights.ring_back)));
    m.push(("gWRingAny", format!("{:.4}", rs.objective.weights.ring_any_direction)));
    m.push(("gWPull", format!("{:.4}", rs.objective.weights.pull)));
    m.push(("gWBuckling", format!("{:.4}", rs.objective.weights.buckling)));
    m.push(("gWSerration", format!("{:.4}", rs.objective.weights.serration)));
    m.push(("gBetaGripSec", format!("{:.2}", rs.objective.betas.grip_section)));
    m.push(("gBetaRing", format!("{:.2}", rs.objective.betas.ring)));
    m.push(("gBetaEdges", format!("{:.2}", rs.objective.betas.hand_edges)));
    m.push(("gBetaPressure", format!("{:.2}", rs.objective.betas.contact_pressure)));
    m.push(("gBetaGuard", format!("{:.2}", rs.objective.betas.guard)));
    m.push(("gBetaGripLen", format!("{:.2}", rs.objective.betas.grip_length)));
    m.push(("gBetaBalance", format!("{:.2}", rs.objective.betas.balance)));
    m.push(("gBetaSurface", format!("{:.2}", rs.objective.betas.grip_surface)));
    m.push(("gEExp", format!("{:.1}", rs.objective.e_exponent)));
    // promotion constants of the search, from the code (`RankPromotion`)
    m.push(("gPromoWindow", jarambit_g5::g5_qd::RankPromotion::WINDOW.to_string()));
    m.push(("gPromoWarmup", jarambit_g5::g5_qd::RankPromotion::WARMUP.to_string()));

    // Lineage strip (from docs/g5/v<rv>/img/manifest.json + evidence/g5/v<rv>/history/<win_island>.jsonl)
    let rv = str_val(&r, "/version");
    let manifest_path = repo_root.join(format!("docs/g5/v{rv}/img/manifest.json"));
    let manifest: Value = serde_json::from_str(
        &fs::read_to_string(&manifest_path)
            .unwrap_or_else(|e| fail(format!("{}: {e}", manifest_path.display()))),
    )
    .unwrap_or_else(|e| fail(format!("{}: {e}", manifest_path.display())));
    let hist_path = ev_dir.join("history").join(format!("{win_island}.jsonl"));
    let hist_txt = if v6 { String::new() } else { fs::read_to_string(&hist_path).unwrap_or_else(|e| fail(format!("{}: {e}", hist_path.display()))) };
    let mut hist_map = std::collections::BTreeMap::new();
    for line in hist_txt.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let h_val: Value = serde_json::from_str(line)
            .unwrap_or_else(|e| fail(format!("{}: {e}", hist_path.display())));
        let ev_n = num(&h_val, "/evals").round() as i64;
        hist_map.insert(ev_n, (num(&h_val, "/best_score_100"), num(&h_val, "/best_mass_g")));
    }
    let lin = get(&manifest, "/lineage")
        .as_array()
        .unwrap_or_else(|| fail("manifest /lineage not array"));
    let w = 0.98 / lin.len().max(1) as f64;
    let mut l = String::from("% generated by g5-release-tex: best design of the winning island (v6: of the pool lineage) at each checkpoint\n");
    let lineage = if v6 { get(&r, "/lineage").as_array().unwrap_or_else(|| fail("/lineage not array")).clone() } else { Vec::new() };
    for c in lin {
        let (label, (sc_100, mg_g)) = if v6 {
            let pool = str_val(c, "/pool");
            let p = lineage.iter().find(|x| str_val(x, "/pool") == pool).unwrap_or_else(|| fail(format!("manifest pool {pool} not in /lineage")));
            (format!("pool {}", tex(&pool)), (num(p, "/best_raw_score_100"), num(p, "/best_mass_g")))
        } else {
            let ev_n = num(c, "/evals").round() as i64;
            let &v = hist_map.get(&ev_n).unwrap_or_else(|| fail(format!("eval {ev_n} missing in {}", hist_path.display())));
            (format!("eval {}", count(ev_n as f64)), v)
        };
        let role_str = match c.get("role").and_then(Value::as_str) {
            Some(role) if !role.is_empty() => format!("\\\\ \\textbf{{{}}}", tex(role)),
            _ => String::new(),
        };
        let _ = writeln!(
            l,
            "\\begin{{minipage}}[t]{{{w:.3}\\textwidth}}\\centering\\includegraphics[width=\\linewidth]{{../../../{}}}\\\\[-1pt]\
            {{\\EItablefont {}\\\\ {:.2}, {:.1} g{role_str}}}\\end{{minipage}}\\hfill",
            str_val(c, "/image"),
            label,
            sc_100,
            mg_g,
        );
    }
    fs::write(out.join("lineage.tex"), l).unwrap();

    // v6: the FEM renderings of every load case (docs/g5/v<rv>/img/fem/figures.json, written by g5-fem-fields), as two
    // figure* floats of eight panels; fail closed unless they are of this design and decoder and every scored value matches
    let mut fem_files = Vec::new();
    if v6 {
        let fp = repo_root.join(format!("docs/g5/v{rv}/img/fem/figures.json"));
        let fx: Value = serde_json::from_str(&fs::read_to_string(&fp).unwrap_or_else(|e| fail(format!("{}: {e}", fp.display()))))
            .unwrap_or_else(|e| fail(format!("{}: {e}", fp.display())));
        if str_val(&fx, "/design_id") != str_val(&r, "/design_id") || str_val(&fx, "/decoder_sha256") != str_val(&r, "/provenance/decoder_sha256") {
            fail("fem/figures.json is not of the winner of record");
        }
        let figs = get(&fx, "/figures").as_array().unwrap_or_else(|| fail("fem figures")).clone();
        let reproduced = get(&fx, "/reproduced").as_array().unwrap_or_else(|| fail("fem reproduced")).clone();
        let mismatches = get(&fx, "/mismatches").as_array().map_or(1, |v| v.len());
        if mismatches != 0
            || figs.iter().any(|x| x["gated"] == serde_json::json!(true) && x["match"] != serde_json::json!(true))
            || reproduced.iter().any(|x| x["match"] != serde_json::json!(true))
        {
            fail("fem/figures.json: a rendering does not reproduce its scored value");
        }
        m.push(("gFemCases", count(figs.len() as f64)));
        m.push(("gFemReproduced", count(reproduced.len() as f64)));
        m.push(("gFemH", format!("{}", num(&fx, "/h"))));
        let mut hex_min = f64::INFINITY;
        for (case, hex, model, word) in [("grip_squeeze", "gFemHexGrip", "gFemModelGrip", "gFemGripCmp"),
                                         ("guard_push", "gFemHexGuard", "gFemModelGuard", "gFemGuardCmp"),
                                         ("teeth", "gFemHexTeeth", "gFemModelTeeth", "gFemTeethCmp")] {
            let x = figs.iter().find(|x| x["case"] == serde_json::json!(case)).unwrap_or_else(|| fail(format!("fem: no {case}")));
            let (fe, sm) = (num(x, "/sf"), num(x, "/record_sf"));
            hex_min = hex_min.min(fe);
            m.push((hex, format!("{fe:.2}")));
            m.push((model, format!("{sm:.2}")));
            m.push((word, cmp_word(sm, fe, "above", "below", "level with")));
        }
        m.push(("gFemHexMin", format!("{hex_min:.2}")));
        m.push(("gFemHexClears", (if hex_min >= rs.margins.sf_yield { "clears" } else { "does not clear" }).to_string()));
        for (half, slice) in figs.chunks(8).enumerate() {
            let mut t = String::from("% generated by g5-release-tex from docs/g5/v<rv>/img/fem/figures.json\n");
            for (k, x) in slice.iter().enumerate() {
                let stem = str_val(x, "/svg").trim_end_matches(".svg").to_string();
                let cap = if x["gated"] == serde_json::json!(true) {
                    format!("{}: SF {:.2}", tex(&str_val(x, "/title")), num(x, "/sf"))
                } else {
                    format!("{}: hex8 FE SF {:.2} (score model {:.2})", tex(&str_val(x, "/title")), num(x, "/sf"), num(x, "/record_sf"))
                };
                let _ = writeln!(
                    t,
                    "\\begin{{minipage}}[t]{{0.49\\textwidth}}\\centering\\includegraphics[width=\\linewidth]{{fem/{stem}.pdf}}\\\\[-2pt]\
                     {{\\EItablefont {cap}}}\\end{{minipage}}{}",
                    if k % 2 == 0 { "\\hfill" } else { "\\\\[4pt]" }
                );
            }
            let name = format!("fem{}.tex", half + 1);
            fs::write(out.join(&name), t).unwrap();
            fem_files.push(name);
        }
    }

    // CAD assets (from docs/g5/v<rv>/cad/<design_id>_print_spec.json + /release_assets)
    let design_id = str_val(&r, "/design_id");
    let spec_path = repo_root.join(format!("docs/g5/v{rv}/cad/{design_id}_print_spec.json"));
    let spec: Value = serde_json::from_str(
        &fs::read_to_string(&spec_path)
            .unwrap_or_else(|e| fail(format!("{}: {e}", spec_path.display()))),
    )
    .unwrap_or_else(|e| fail(format!("{}: {e}", spec_path.display())));
    let mut asset = String::from("% generated by g5-release-tex: file & triangles & MiB & where\n");
    for ptr in ["/am_build_stl", "/outer_envelope_stl"] {
        let st = get(&spec, ptr);
        let file = str_val(st, "/file");
        let short = file.strip_prefix(&format!("{design_id}_")).unwrap_or(&file);
        let _ = writeln!(
            asset,
            "\\texttt{{{}}} & {} & {:.2} & git (\\code{{cad/}}) \\\\",
            tex(short),
            count(num(st, "/triangles")),
            num(st, "/mib"),
        );
    }
    let rel_assets = get(&r, "/release_assets")
        .as_array()
        .unwrap_or_else(|| fail("/release_assets not array"));
    let rel_prefix = format!("g5-v{}-", str_val(&r, "/version"));
    for x in rel_assets {
        let file = str_val(x, "/file");
        // release assets are named g5-v<version>-[<design>_]<name>; the table shows <name> (the caption names the rule)
        let short = file.strip_prefix(&rel_prefix).unwrap_or(&file);
        let short = short.strip_prefix(&format!("{design_id}_")).unwrap_or(short);
        let _ = writeln!(
            asset,
            "\\texttt{{{}}} & --- & {:.2} & release \\\\",
            tex(short),
            num(x, "/bytes") / 1_048_576.0,
        );
    }
    fs::write(out.join("assets.tex"), &asset).unwrap();

    let rows_macro = |name: &str, body: &str| {
        let rows: String = body.lines().filter(|l| !l.starts_with('%')).collect::<Vec<_>>().join("\n");
        format!("\\newcommand{{\\{name}}}{{%\n{rows}}}\n")
    };
    let mut numbers = String::from("% generated by g5-release-tex from evidence/g5/v<VERSION>/release.json; do not edit\n");
    for (k, v) in &m {
        let _ = writeln!(numbers, "\\newcommand{{\\{k}}}{{{v}}}");
    }
    let _ = writeln!(numbers, "\\graphicspath{{{{../v{}/img/}}}}", str_val(&r, "/version"));
    numbers.push_str(&rows_macro("gTopEightRows", &top));
    numbers.push_str(&rows_macro("gAssetRows", &asset));
    numbers.push_str(&rows_macro("gGridRows", &grid));
    fs::write(out.join("numbers.tex"), numbers).unwrap();
    println!(
        "g5-release-tex: {} macros, {} ranking rows, {} lineage frames -> {}",
        m.len(),
        n_show,
        lin.len(),
        out.display()
    );
}
