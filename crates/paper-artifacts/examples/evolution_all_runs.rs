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

/// 0: the 2026-10-18/19 waves (`jarambit/out`, archived wave trees); 1: the 2026-10-18 island
/// campaign (`evo_full_20261018`); 2: continuous evolution (`evo_continuous`, since 2026-10-21).
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
    let mut older_evaluator = 0usize;
    for r in &stage2 {
        // Keep one evaluator regime: scorecards from evaluators that predate W_ULTRALIGHT_v1 are counted, not plotted.
        if r.get("evaluator_has_w_ultralight_v1").is_some_and(|v| v == "no") {
            older_evaluator += 1;
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
    let meta = format!(
        "% Generated by evolution_all_runs; do not edit.\n\
\\newcommand{{\\AllRunsIslands}}{{{}}}\n\\newcommand{{\\AllRunsHosts}}{{{}}}\n\\newcommand{{\\AllRunsRuns}}{{{}}}\n\
\\newcommand{{\\AllRunsStageTwo}}{{{}}}\n\\newcommand{{\\AllRunsStageTwoWaves}}{{{}}}\n\\newcommand{{\\AllRunsStageTwoCampaign}}{{{}}}\n\\newcommand{{\\AllRunsStageTwoContinuous}}{{{}}}\n\\newcommand{{\\AllRunsOlderEvaluator}}{{{}}}\n\
\\newcommand{{\\AllRunsStageOne}}{{{}}}\n\\newcommand{{\\AllRunsRounds}}{{{}}}\n\
\\newcommand{{\\AllRunsLeader}}{{\\texttt{{{}}}}}\n\\newcommand{{\\AllRunsLeaderScore}}{{{:.6}}}\n\\newcommand{{\\AllRunsLeaderHost}}{{{}}}\n\
\\newcommand{{\\AllRunsFirstDay}}{{{:.2}}}\n\\newcommand{{\\AllRunsLastDay}}{{{:.2}}}\n",
        islands.len(), hosts.len(), runs.len(), rows.len(), count(0), count(1), count(2), older_evaluator, screened, rounds.len(),
        tex_escape(&lead_label), lead_score, lead_host, first_day, last_day,
    );
    fs::write(out.join("stage2.dat"), s2)?;
    fs::write(out.join("best.dat"), best)?;
    fs::write(out.join("stage1-round.dat"), s1)?;
    fs::write(out.join("allruns-meta.tex"), meta)?;
    let receipt = json!({
        "schema": "jarambit.evolution_all_runs.v1",
        "command": std::env::args().collect::<Vec<_>>().join(" "),
        "census": {"stage2_all.tsv": s2_sha, "stage1_all.tsv.zst": s1_sha, "islands.tsv": isl_sha},
        "counts": {"islands": islands.len(), "hosts": hosts, "runs": runs, "stage2": rows.len(), "stage2_older_evaluator_not_plotted": older_evaluator, "stage1_numeric": screened, "island_rounds": rounds.len()},
        "leader": {"label": lead_label, "canonical_w_ultralight_v1": lead_score, "host": lead_host, "day": lead_day},
        "time": "UTC day of October 2026 from each record's file modification time",
        "until": until.as_ref().map(|u| u.0.clone()),
        "score": "canonical W_ULTRALIGHT_v1 recomputed from each retained scorecard and RS-v2 receipt (Stage 2); Stage-1 screen value as recorded by auto-evolve",
    });
    fs::write(out.join("allruns.v1.json"), serde_json::to_string_pretty(&receipt)? + "\n")?;
    println!("wrote {}", out.display());
    Ok(())
}
