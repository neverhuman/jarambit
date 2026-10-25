//! All-runs evolution data for the paper: every retained Stage-2 assessment and every Stage-1
//! screen of every `auto-evolve` island on every host, on the canonical `W_ULTRALIGHT_v1` scale.
//! Reads the census tables (stage2_all.tsv, stage1_all.tsv.zst, islands.tsv), checks them, and
//! writes the plot data and summary macros that main.tex draws with pgfplots:
//! - stage2.dat: one row per Stage-2 assessment (time, canonical score, campaign);
//! - best.dat: the cumulative best Stage-2 score over all runs, as a step trace;
//! - stage1-round.dat: the best Stage-1 screen score of each island round (time, score, campaign);
//! - allruns-meta.tex: counts, campaigns, hosts and the leader.
//!
//! Time is UTC day of October 2026 (e.g. 22.5 = 2026-10-22 12:00Z), from each record's file time.
//! Scores are the census's canonical recomputation, never stored leaderboard values.
//! Usage: evolution_all_runs --census DIR --out DIR [--until ISO8601Z]
//! `--until` keeps records whose file time is at or before that instant (the winner snapshot).
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::PathBuf};

type Res<T> = Result<T, Box<dyn std::error::Error>>;

/// Rows of a tab-separated table with a header, as column-name maps.
fn table(text: &str) -> Res<Vec<BTreeMap<String, String>>> {
    let mut lines = text.lines();
    let header: Vec<&str> = lines.next().ok_or("empty table")?.split('\t').collect();
    lines
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let cells: Vec<&str> = l.split('\t').collect();
            if cells.len() != header.len() {
                return Err(format!("row has {} cells, header {}: {l:?}", cells.len(), header.len()).into());
            }
            Ok(header.iter().zip(cells).map(|(h, c)| (h.to_string(), c.to_string())).collect())
        })
        .collect()
}

/// ISO-8601 UTC timestamp to day-of-October-2026 with a fraction.
fn october_day(ts: &str) -> Res<f64> {
    let ts = ts.trim_end_matches('Z');
    let (date, time) = ts.split_once('T').ok_or_else(|| format!("bad timestamp {ts:?}"))?;
    let d: Vec<u32> = date.split('-').map(str::parse).collect::<Result<_, _>>()?;
    if d.len() != 3 || d[0] != 2026 || d[1] != 10 {
        return Err(format!("timestamp {ts:?} is outside October 2026").into());
    }
    let t: Vec<f64> = time.split(':').map(|x| x.parse::<f64>()).collect::<Result<_, _>>()?;
    let seconds = t.first().copied().unwrap_or(0.0) * 3600.0 + t.get(1).copied().unwrap_or(0.0) * 60.0 + t.get(2).copied().unwrap_or(0.0);
    Ok(d[2] as f64 + seconds / 86_400.0)
}

/// 0: the 2026-10-16..19 waves (`jarambit/out`, archived wave trees, committed runs); 1: the
/// 2026-10-18 island campaign (`evo_full_20261018`); 2: continuous evolution (`evo_continuous`, since 2026-10-21).
fn campaign(run: &str) -> u8 {
    if run.starts_with("evo_continuous") {
        2
    } else if run.starts_with("evo_full") {
        1
    } else {
        0
    }
}

fn tex_escape(s: &str) -> String {
    s.replace('_', "\\_")
}

fn main() -> Res<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let arg = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned().ok_or_else(|| format!("missing {k}"));
    let census = PathBuf::from(arg("--census")?);
    let until = match arg("--until") {
        Ok(ts) => Some((ts.clone(), october_day(&ts)?)),
        Err(_) => None,
    };
    let keep = |day: f64| until.as_ref().is_none_or(|u| day <= u.1);
    let out = PathBuf::from(arg("--out")?);
    let read = |name: &str| -> Res<(String, String)> {
        let bytes = fs::read(census.join(name))?;
        Ok((String::from_utf8(bytes.clone())?, format!("{:x}", Sha256::digest(&bytes))))
    };
    let (s2_text, s2_sha) = read("stage2_all.tsv")?;
    // The Stage-1 table is committed zstd-compressed (stage1_all.tsv.zst); the receipt hashes the compressed bytes.
    let (s1_text, s1_sha) = {
        let bytes = fs::read(census.join("stage1_all.tsv.zst"))?;
        let mut text = String::new();
        std::io::Read::read_to_string(&mut ruzstd::decoding::StreamingDecoder::new(bytes.as_slice())?, &mut text)?;
        (text, format!("{:x}", Sha256::digest(&bytes)))
    };
    let (isl_text, isl_sha) = read("islands.tsv")?;
    let stage2 = table(&s2_text)?;
    let stage1 = table(&s1_text)?;
    let islands = table(&isl_text)?;
    fs::create_dir_all(&out)?;

    // Stage 2: every assessment, and the cumulative best over all runs in time order.
    let mut rows: Vec<(f64, f64, u8, String, String)> = Vec::new();
    let mut inflated = 0usize;
    for r in &stage2 {
        // Scorecards from the 2026-10-16 evaluators (class `inflated_1e93e6f3`) overstate the canonical score
        // of the same genome under the current evaluator (92.29 stored, 79.17 re-assessed): counted, not plotted.
        // Later evaluators reproduce the current one exactly on re-assessment and are plotted.
        if r.get("evaluator_class").is_some_and(|v| v != "ok") {
            inflated += 1;
            continue;
        }
        let score: f64 = r["canonical_w_ultralight_v1"].parse()?;
        if !(0.0..=100.0).contains(&score) {
            return Err(format!("canonical score {score} outside [0, 100]").into());
        }
        let day = october_day(&r["scorecard_mtime_utc"])?;
        if keep(day) {
            rows.push((day, score, campaign(&r["run"]), r["label"].clone(), r["host"].clone()));
        }
    }
    rows.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut s2 = String::from("day score campaign\n");
    let mut best = String::from("day best\n");
    let mut leader: Option<(f64, f64, String, String)> = None;
    for (day, score, camp, label, host) in &rows {
        s2 += &format!("{day:.5} {score:.6} {camp}\n");
        if leader.as_ref().is_none_or(|l| *score > l.1) {
            if let Some(l) = &leader {
                best += &format!("{day:.5} {:.6}\n", l.1);
            }
            best += &format!("{day:.5} {score:.6}\n");
            leader = Some((*day, *score, label.clone(), host.clone()));
        }
    }
    let (lead_day, lead_score, lead_label, lead_host) = leader.ok_or("no Stage-2 assessments")?;
    let last_day = rows.last().map_or(lead_day, |r| r.0);
    best += &format!("{last_day:.5} {lead_score:.6}\n");

    // Stage 1: best numeric screen score per island round.
    let mut rounds: BTreeMap<(String, String, String, String), (f64, f64, u8)> = BTreeMap::new();
    let mut screened = 0usize;
    for r in &stage1 {
        let Ok(score) = r["stage1_w_ultralight_v1"].parse::<f64>() else { continue };
        let day = october_day(&r["screen_mtime_utc"])?;
        if !keep(day) {
            continue;
        }
        screened += 1;
        let key = (r["host"].clone(), r["run"].clone(), r["island"].clone(), r["round"].clone());
        let e = rounds.entry(key).or_insert((day, f64::NEG_INFINITY, campaign(&r["run"])));
        e.1 = e.1.max(score);
    }
    let mut s1 = String::from("day score campaign\n");
    let mut by_day: Vec<_> = rounds.values().collect();
    by_day.sort_by(|a, b| a.0.total_cmp(&b.0));
    for (day, score, camp) in by_day {
        s1 += &format!("{day:.5} {score:.6} {camp}\n");
    }

    let count = |camp: u8| rows.iter().filter(|r| r.2 == camp).count();
    let hosts: std::collections::BTreeSet<&str> = islands.iter().map(|r| r["host"].as_str()).collect();
    let runs: std::collections::BTreeSet<&str> = islands.iter().map(|r| r["run"].as_str()).collect();
    let first_day = rows.first().map_or(lead_day, |r| r.0);

    // Read winner-selection.v1.json for distinct-genome count, re-assessments, and previous leader.
    let sel_path = match arg("--selection") {
        Ok(p) => PathBuf::from(p),
        Err(_) => out.join("../figures/cover/winner-selection.v1.json"),
    };
    let sel_bytes = fs::read(&sel_path)?;
    let sel: serde_json::Value = serde_json::from_slice(&sel_bytes)?;
    let distinct_genomes = sel["distinct_genomes"].as_u64().ok_or("missing distinct_genomes")? as usize;
    let leader_reassessed = sel["winner_re_assessment"]["re_assessed"]
        .as_f64()
        .ok_or("missing winner_re_assessment.re_assessed")?;
    let reassess_list = sel["re_assessment"]["results"]
        .as_array()
        .ok_or("missing re_assessment.results")?;
    // w290_r03_p14_c04 is the named stored-score inflation example from the 2026-10-16 evaluator (1e93e6f3).
    let w290 = reassess_list
        .iter()
        .find(|r| r["label"].as_str() == Some("w290_r03_p14_c04"))
        .ok_or("missing w290_r03_p14_c04 in re_assessment.results")?;
    if !w290["stored_evaluator"]
        .as_str()
        .is_some_and(|s| s.starts_with("1e93e6f3"))
    {
        return Err("w290_r03_p14_c04 stored_evaluator is not 1e93e6f3*".into());
    }
    let inflated_stored = w290["stored"].as_f64().ok_or("missing w290 stored")?;
    let inflated_reassessed = w290["re_assessed"].as_f64().ok_or("missing w290 re_assessed")?;
    let top20 = sel["top20_distinct_genomes"]
        .as_array()
        .ok_or("missing top20_distinct_genomes")?;
    let seed_parent_score = top20
        .iter()
        .find(|r| r["label"].as_str() == Some("ec1i012_r04_p10_c101"))
        .ok_or("missing ec1i012_r04_p10_c101 in top20_distinct_genomes")?["canonical_w_ultralight_v1"]
        .as_f64()
        .ok_or("missing ec1i012_r04_p10_c101 canonical_w_ultralight_v1")?;
    let mut inf_diffs = Vec::new();
    for r in reassess_list {
        if r["stored_evaluator"].as_str().is_some_and(|s| s.starts_with("1e93e6f3")) {
            let st = r["stored"].as_f64().ok_or("missing stored")?;
            let re = r["re_assessed"].as_f64().ok_or("missing re_assessed")?;
            inf_diffs.push(st - re);
        }
    }
    if inf_diffs.is_empty() {
        return Err("no 1e93e6f3 re-assessment entries found".into());
    }
    let inf_min = inf_diffs.iter().copied().fold(f64::INFINITY, f64::min);
    let inf_max = inf_diffs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let prev0 = &sel["previous_selections"][0];
    let prev_leader = prev0["winner"].as_str().ok_or("missing previous_selections[0].winner")?;
    let prev_leader_score = prev0["canonical_w_ultralight_v1"]
        .as_f64()
        .ok_or("missing previous_selections[0].canonical_w_ultralight_v1")?;

    // Read gen800_population_density_and_traces.v1.json and verification.json for Cohort 3 and solver macros.
    let gen800_path = match arg("--gen800") {
        Ok(p) => PathBuf::from(p),
        Err(_) => out.join("../data/gen800_population_density_and_traces.v1.json"),
    };
    let gen800: serde_json::Value = serde_json::from_slice(&fs::read(&gen800_path)?)?;
    let c3_total = gen800["total_rows"].as_u64().ok_or("missing total_rows")? as usize;
    let c3_legal = gen800["total_legal_true_rows"].as_u64().ok_or("missing total_legal_true_rows")? as usize;
    let c3_runs = gen800["total_runs"].as_u64().ok_or("missing total_runs")? as usize;
    let g800_runs = gen800["runs"].as_array().ok_or("missing gen800 runs")?;
    let share_range = |gen_idx: usize| -> Res<(i64, i64)> {
        let mut pcts = Vec::new();
        for r in g800_runs {
            let leg = r["gen_legal_count"][gen_idx].as_f64().ok_or("missing gen_legal_count")?;
            let tot = r["gen_total_count"][gen_idx].as_f64().ok_or("missing gen_total_count")?;
            pcts.push((leg / tot * 100.0).round() as i64);
        }
        let lo = *pcts.iter().min().ok_or("empty gen800 runs")?;
        let hi = *pcts.iter().max().ok_or("empty gen800 runs")?;
        Ok((lo, hi))
    };
    let (g0_lo, g0_hi) = share_range(0)?;
    let (g50_lo, g50_hi) = share_range(50)?;

    let ver_path = match arg("--verification") {
        Ok(p) => PathBuf::from(p),
        Err(_) => out.join("../generated/verification.json"),
    };
    let ver: serde_json::Value = serde_json::from_slice(&fs::read(&ver_path)?)?;
    let solver_max_pct = ver["scalar_pairs"]
        .as_array()
        .ok_or("missing scalar_pairs")?
        .iter()
        .filter_map(|p| p["stored_relative_sf_difference"].as_f64())
        .fold(0.0_f64, f64::max)
        * 100.0;

    // Read pocket-smoothing receipts for \Pocket* macros.
    let win_rec_path = match arg("--winner-receipt") {
        Ok(p) => PathBuf::from(p),
        Err(_) => out.join("../../../experiments/pocket-surface-tension-smoothing/results/ec3i042_r01_p10_c56/RECEIPT.json"),
    };
    let set_rec_path = match arg("--setting-receipt") {
        Ok(p) => PathBuf::from(p),
        Err(_) => out.join("../../../experiments/pocket-surface-tension-smoothing/results/w406t3_r01_p10_c16/RECEIPT.json"),
    };
    let win_rec: serde_json::Value = serde_json::from_slice(&fs::read(&win_rec_path)?)?;
    let set_rec: serde_json::Value = serde_json::from_slice(&fs::read(&set_rec_path)?)?;
    let find_run = |doc: &serde_json::Value, name: &str| -> Res<serde_json::Value> {
        doc["runs"]
            .as_array()
            .ok_or("missing runs")?
            .iter()
            .find(|r| r["name"].as_str() == Some(name))
            .cloned()
            .ok_or_else(|| format!("missing run {name}").into())
    };
    let w_ctrl = find_run(&win_rec, "ctrl")?;
    let w_f075 = find_run(&win_rec, "f075")?;
    let s_ctrl = find_run(&set_rec, "ctrl")?;
    let s_f075 = find_run(&set_rec, "f075")?;
    let w_ctrl_score = w_ctrl["score_100"].as_f64().ok_or("w_ctrl score_100")?;
    let w_f075_score = w_f075["score_100"].as_f64().ok_or("w_f075 score_100")?;
    let s_ctrl_score = s_ctrl["score_100"].as_f64().ok_or("s_ctrl score_100")?;
    let s_f075_score = s_f075["score_100"].as_f64().ok_or("s_f075 score_100")?;
    let delta_f = w_f075_score - w_ctrl_score;
    let delta_f_set = s_f075_score - s_ctrl_score;
    let fmt_signed = |d: f64| -> String {
        if d >= 0.0 {
            format!("$+{d:.6}$")
        } else {
            format!("${d:.6}$")
        }
    };
    let bat_before = w_ctrl["battery_pass"].as_u64().ok_or("w_ctrl battery_pass")?;
    let bat_after = w_f075["battery_pass"].as_u64().ok_or("w_f075 battery_pass")?;
    let mass_before = w_ctrl["mass_3d_g"].as_f64().ok_or("w_ctrl mass_3d_g")?;
    let mass_after = w_f075["mass_3d_g"].as_f64().ok_or("w_f075 mass_3d_g")?;
    let rep = &w_f075["smoothing_report"];
    let vol_before = rep["removed_volume_mm3"]["before"].as_f64().ok_or("vol before")?;
    let vol_after = rep["removed_volume_mm3"]["after"].as_f64().ok_or("vol after")?;
    let vol_pct = ((vol_after / vol_before - 1.0).abs()) * 100.0;
    let iso = rep["isoline_length_by_depth_level_mm"]
        .as_object()
        .ok_or("isoline_length_by_depth_level_mm")?;
    let mut iso_pcts = Vec::new();
    for v in iso.values() {
        let b = v["before_mm"].as_f64().ok_or("before_mm")?;
        let af = v["after_mm"].as_f64().ok_or("after_mm")?;
        iso_pcts.push((1.0 - af / b) * 100.0);
    }
    let iso_min = iso_pcts.iter().copied().fold(f64::INFINITY, f64::min).round() as i64;
    let iso_max = iso_pcts.iter().copied().fold(f64::NEG_INFINITY, f64::max).round() as i64;
    let w_sec = rep["seconds"].as_f64().ok_or("w_f075 seconds")?;
    let s_sec = s_f075["smoothing_report"]["seconds"].as_f64().ok_or("s_f075 seconds")?;
    let sec_min = w_sec.min(s_sec);
    let sec_max = w_sec.max(s_sec);

    let meta = format!(
        "% Generated by evolution_all_runs; do not edit.\n\
\\newcommand{{\\AllRunsIslands}}{{{}}}\n\\newcommand{{\\AllRunsHosts}}{{{}}}\n\\newcommand{{\\AllRunsRuns}}{{{}}}\n\
\\newcommand{{\\AllRunsStageTwo}}{{{}}}\n\\newcommand{{\\AllRunsDistinctGenomes}}{{{}}}\n\\newcommand{{\\AllRunsStageTwoWaves}}{{{}}}\n\\newcommand{{\\AllRunsStageTwoCampaign}}{{{}}}\n\\newcommand{{\\AllRunsStageTwoContinuous}}{{{}}}\n\\newcommand{{\\AllRunsInflated}}{{{}}}\n\
\\newcommand{{\\AllRunsStageOne}}{{{}}}\n\\newcommand{{\\AllRunsRounds}}{{{}}}\n\
\\newcommand{{\\AllRunsLeader}}{{\\texttt{{{}}}}}\n\\newcommand{{\\AllRunsLeaderScore}}{{{:.6}}}\n\\newcommand{{\\AllRunsLeaderReassessed}}{{{:.6}}}\n\\newcommand{{\\AllRunsLeaderHost}}{{{}}}\n\
\\newcommand{{\\AllRunsInflatedStored}}{{{:.2}}}\n\\newcommand{{\\AllRunsInflatedReassessed}}{{{:.6}}}\n\\newcommand{{\\AllRunsInflationRange}}{{{:.2}--{:.2}}}\n\
\\newcommand{{\\AllRunsPreviousLeader}}{{\\texttt{{{}}}}}\n\\newcommand{{\\AllRunsPreviousLeaderScore}}{{{:.6}}}\n\
\\newcommand{{\\SeedParentScore}}{{{:.6}}}\n\
\\newcommand{{\\AllRunsFirstDay}}{{{:.2}}}\n\\newcommand{{\\AllRunsLastDay}}{{{:.2}}}\n\
\\newcommand{{\\CohortThreeTotalEvaluations}}{{{}}}\n\\newcommand{{\\CohortThreeLegalEvaluations}}{{{}}}\n\\newcommand{{\\CohortThreeRuns}}{{{}}}\n\
\\newcommand{{\\NativeLegalShareGenZero}}{{{}--{}}}\n\\newcommand{{\\NativeLegalShareGenFifty}}{{{}--{}}}\n\
\\newcommand{{\\SolverAgreementMaxPct}}{{{:.4}}}\n\
\\newcommand{{\\PocketSmoothedScore}}{{{:.6}}}\n\\newcommand{{\\PocketDeltaF}}{{{}}}\n\\newcommand{{\\PocketDeltaFSettingDesign}}{{{}}}\n\
\\newcommand{{\\PocketBatteryBefore}}{{{}}}\n\\newcommand{{\\PocketBatteryAfter}}{{{}}}\n\
\\newcommand{{\\PocketMassBefore}}{{{:.2}}}\n\\newcommand{{\\PocketMassAfter}}{{{:.2}}}\n\
\\newcommand{{\\PocketRemovedVolumePct}}{{{:.3}}}\n\\newcommand{{\\PocketContourRange}}{{{}--{}}}\n\\newcommand{{\\PocketSecondsRange}}{{{:.1}--{:.1}}}\n",
        islands.len(), hosts.len(), runs.len(),
        rows.len(), distinct_genomes, count(0), count(1), count(2), inflated,
        screened, rounds.len(),
        tex_escape(&lead_label), lead_score, leader_reassessed, lead_host,
        inflated_stored, inflated_reassessed, inf_min, inf_max,
        tex_escape(prev_leader), prev_leader_score,
        seed_parent_score,
        first_day, last_day,
        c3_total, c3_legal, c3_runs,
        g0_lo, g0_hi, g50_lo, g50_hi,
        solver_max_pct,
        w_f075_score, fmt_signed(delta_f), fmt_signed(delta_f_set),
        bat_before, bat_after,
        mass_before, mass_after,
        vol_pct, iso_min, iso_max, sec_min, sec_max,
    );
    fs::write(out.join("stage2.dat"), s2)?;
    fs::write(out.join("best.dat"), best)?;
    fs::write(out.join("stage1-round.dat"), s1)?;
    fs::write(out.join("allruns-meta.tex"), meta)?;
    let receipt = json!({
        "schema": "jarambit.evolution_all_runs.v1",
        "command": std::env::args().collect::<Vec<_>>().join(" "),
        "census": {"stage2_all.tsv": s2_sha, "stage1_all.tsv.zst": s1_sha, "islands.tsv": isl_sha},
        "counts": {"islands": islands.len(), "hosts": hosts, "runs": runs, "stage2": rows.len(), "stage2_inflated_evaluator_not_plotted": inflated, "stage1_numeric": screened, "island_rounds": rounds.len()},
        "leader": {"label": lead_label, "canonical_w_ultralight_v1": lead_score, "host": lead_host, "day": lead_day},
        "time": "UTC day of October 2026 from each record's file modification time",
        "until": until.as_ref().map(|u| u.0.clone()),
        "score": "canonical W_ULTRALIGHT_v1 recomputed from each retained scorecard and RS-v2 receipt (Stage 2); Stage-1 screen value as recorded by auto-evolve",
    });
    fs::write(out.join("allruns.v1.json"), serde_json::to_string_pretty(&receipt)? + "\n")?;
    println!("wrote {}", out.display());
    Ok(())
}
