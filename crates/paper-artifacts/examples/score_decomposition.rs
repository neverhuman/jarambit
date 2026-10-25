//! Deterministic generator and `--check` verifier for Paper Phase 2 (M7, M8, M9):
//! - M7 (P2): Native 800-generation population dynamics (`docs/paper/allruns/native-dynamics.dat`
//!   and `docs/paper/generated/native-dynamics-fig.tex`) from
//!   `docs/paper/data/gen800_population_density_and_traces.v1.json`.
//! - M8 (P3): Score decomposition (`docs/paper/generated/score-decomposition.tex`) from
//!   `experiments/pocket-surface-tension-smoothing/results/ec3i042_r01_p10_c56/scores.json`
//!   and `docs/paper/data/term-evidence-class.v1.json`, verifying fallback-matched terms
//!   directly against the scorer's fallback constants.
//! - M9 (P4): Informational requirements table (`docs/paper/generated/winner-requirements.tex`)
//!   from `re_ec3i042_r01_p10_c56_f075_master_scorecard.json` and `RECEIPT.json`.

use serde_json::Value;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("repo root")
        .to_path_buf()
}

fn write_or_check(path: &Path, content: &str, check: bool) -> Result<()> {
    if check {
        let existing = fs::read_to_string(path)
            .map_err(|e| format!("missing generated file {}: {e}", path.display()))?;
        if existing != content {
            return Err(format!("stale generated file: {}", path.display()).into());
        }
    } else {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
    }
    Ok(())
}

fn median_f64(mut vals: Vec<f64>) -> f64 {
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = vals.len();
    if n == 0 {
        0.0
    } else if n % 2 == 1 {
        vals[n / 2]
    } else {
        0.5 * (vals[n / 2 - 1] + vals[n / 2])
    }
}

fn generate_m7_native_dynamics(root: &Path, check: bool) -> Result<()> {
    let json_path = root.join("docs/paper/data/gen800_population_density_and_traces.v1.json");
    let doc: Value = serde_json::from_str(&fs::read_to_string(&json_path)?)?;
    let runs = doc["runs"].as_array().ok_or("missing runs array")?;
    if runs.len() != 10 {
        return Err(format!("expected 10 native runs, found {}", runs.len()).into());
    }

    // Logarithmic generation bins: (0,0), (1,1), (2,3), (4,7), (8,15), (16,31),
    // (32,63), (64,127), (128,255), (256,511), (512,800)
    let bins: &[(usize, usize)] = &[
        (0, 0),
        (1, 1),
        (2, 3),
        (4, 7),
        (8, 15),
        (16, 31),
        (32, 63),
        (64, 127),
        (128, 255),
        (256, 511),
        (512, 800),
    ];

    let mut dat = String::from(
        "bin_idx gen_end x_log1p \
r0_leg r1_leg r2_leg r3_leg r4_leg r5_leg r6_leg r7_leg r8_leg r9_leg med_leg \
r0_med r1_med r2_med r3_med r4_med r5_med r6_med r7_med r8_med r9_med med_med med_q10 med_q90 \
r0_best r1_best r2_best r3_best r4_best r5_best r6_best r7_best r8_best r9_best med_best\n",
    );

    for (b_idx, &(g_lo, g_hi)) in bins.iter().enumerate() {
        let x_log1p = ((g_hi as f64) + 1.0).log10();
        let mut leg_runs = Vec::with_capacity(10);
        let mut med_runs = Vec::with_capacity(10);
        let mut q10_runs = Vec::with_capacity(10);
        let mut q90_runs = Vec::with_capacity(10);
        let mut best_runs = Vec::with_capacity(10);

        for r in runs {
            let leg_arr = r["gen_legal_count"].as_array().ok_or("missing gen_legal_count")?;
            let tot_arr = r["gen_total_count"].as_array().ok_or("missing gen_total_count")?;
            let med_arr = r["gen_median_fit"].as_array().ok_or("missing gen_median_fit")?;
            let q10_arr = r["gen_q10_fit"].as_array().ok_or("missing gen_q10_fit")?;
            let q90_arr = r["gen_q90_fit"].as_array().ok_or("missing gen_q90_fit")?;
            let best_arr = r["gen_cum_best_fit"].as_array().ok_or("missing gen_cum_best_fit")?;

            let count = (g_hi - g_lo + 1) as f64;
            let mut sum_leg_pct = 0.0;
            let mut sum_med = 0.0;
            let mut sum_q10 = 0.0;
            let mut sum_q90 = 0.0;
            for g in g_lo..=g_hi {
                let l = leg_arr[g].as_f64().unwrap_or(0.0);
                let t = tot_arr[g].as_f64().unwrap_or(1.0).max(1.0);
                sum_leg_pct += 100.0 * l / t;
                sum_med += med_arr[g].as_f64().unwrap_or(0.0);
                sum_q10 += q10_arr[g].as_f64().unwrap_or(0.0);
                sum_q90 += q90_arr[g].as_f64().unwrap_or(0.0);
            }
            leg_runs.push(sum_leg_pct / count);
            med_runs.push(sum_med / count);
            q10_runs.push(sum_q10 / count);
            q90_runs.push(sum_q90 / count);
            best_runs.push(best_arr[g_hi].as_f64().unwrap_or(0.0));
        }

        let med_leg = median_f64(leg_runs.clone());
        let med_med = median_f64(med_runs.clone());
        let med_q10 = median_f64(q10_runs);
        let med_q90 = median_f64(q90_runs);
        let med_best = median_f64(best_runs.clone());

        let mut row = format!("{b_idx} {g_hi} {x_log1p:.6}");
        for v in &leg_runs {
            row.push_str(&format!(" {v:.4}"));
        }
        row.push_str(&format!(" {med_leg:.4}"));
        for v in &med_runs {
            row.push_str(&format!(" {v:.4}"));
        }
        row.push_str(&format!(" {med_med:.4} {med_q10:.4} {med_q90:.4}"));
        for v in &best_runs {
            row.push_str(&format!(" {v:.4}"));
        }
        row.push_str(&format!(" {med_best:.4}\n"));
        dat.push_str(&row);
    }

    write_or_check(&root.join("docs/paper/allruns/native-dynamics.dat"), &dat, check)?;

    let fig_tex = r#"% Generated by crates/paper-artifacts/examples/score_decomposition.rs -- do not hand-edit.
\begin{figure}[t]
\centering
\begin{tikzpicture}
\begin{axis}[
  name=axleg,
  width=\columnwidth, height=3.8cm,
  font=\sffamily\fontsize{6.6}{8}\selectfont,
  every axis label/.append style={font=\sffamily\fontsize{6.8}{8.2}\selectfont, text=ink},
  xmin=0, xmax=2.92, ymin=0, ymax=54,
  xtick={0,0.30103,0.95424,1.70757,2.30320,2.90363},
  xticklabels={0,1,8,50,200,800},
  ytick={0,10,20,30,40,50},
  ylabel={search-legal share (\%)},
  ylabel style={yshift=-2pt},
  axis x line*=bottom, axis y line*=left,
  axis line style={draw=axisgray, line width=0.4pt},
  tick style={draw=axisgray, line width=0.4pt}, major tick length=2pt,
  tick label style={text=slate},
  ymajorgrids, grid style={draw=gridgray, line width=0.3pt},
  clip=false
]
\foreach \r in {0,...,9} {
  \addplot[draw=axisgray, line width=0.4pt, forget plot] table[x=x_log1p, y=r\r_leg] {allruns/native-dynamics.dat};
}
\addplot[seriesA, line width=1.0pt] table[x=x_log1p, y=med_leg] {allruns/native-dynamics.dat};
\node[font=\sffamily\fontsize{6.2}{7.4}\selectfont, text=ink, anchor=south west, inner sep=1pt] at (axis cs:0.04,45.5) {\textbf{a}\enspace Legal share per generation (10 runs; gen 50: \NativeLegalShareGenFifty\,\%)};
\end{axis}

\begin{axis}[
  at={(axleg.below south west)}, anchor=above north west, yshift=-3mm,
  width=\columnwidth, height=3.8cm,
  font=\sffamily\fontsize{6.6}{8}\selectfont,
  every axis label/.append style={font=\sffamily\fontsize{6.8}{8.2}\selectfont, text=ink},
  xmin=0, xmax=2.92, ymin=-5.2, ymax=1.55,
  xtick={0,0.30103,0.95424,1.70757,2.30320,2.90363},
  xticklabels={0,1,8,50,200,800},
  ytick={-4,-2,0,1},
  xlabel={generation (logarithmic scale, $\log_{10}(\text{gen}+1)$)},
  ylabel={native search $f$},
  ylabel style={yshift=-2pt}, xlabel style={yshift=2pt},
  axis x line*=bottom, axis y line*=left,
  axis line style={draw=axisgray, line width=0.4pt},
  tick style={draw=axisgray, line width=0.4pt}, major tick length=2pt,
  tick label style={text=slate},
  ymajorgrids, grid style={draw=gridgray, line width=0.3pt},
  clip=false
]
\foreach \r in {0,...,9} {
  \addplot[draw=axisgray, line width=0.4pt, forget plot] table[x=x_log1p, y=r\r_med] {allruns/native-dynamics.dat};
  \addplot[draw=axisgray, line width=0.4pt, densely dashed, forget plot] table[x=x_log1p, y=r\r_best] {allruns/native-dynamics.dat};
}
\addplot[seriesA, line width=1.0pt] table[x=x_log1p, y=med_med] {allruns/native-dynamics.dat};
\addplot[ink, line width=0.9pt] table[x=x_log1p, y=med_best] {allruns/native-dynamics.dat};
\node[font=\sffamily\fontsize{6.2}{7.4}\selectfont, text=ink, anchor=south west, inner sep=1pt] at (axis cs:0.04,-4.95) {\textbf{b}\enspace Population median $f$ ({\color{seriesA}blue}) and cumulative best ({\color{ink}dark})};
\end{axis}
\end{tikzpicture}
\caption{Population dynamics across the ten completed 800-generation native runs (\num{\CohortThreeTotalEvaluations} evaluated births; Runs~2--11). \textbf{a},~Share of births passing the search's legality checks (generation~0: \NativeLegalShareGenZero\,\%; generation~50: \NativeLegalShareGenFifty\,\%; overall: \num{\CohortThreeLegalEvaluations} births, $38.59\%$). \textbf{b},~Native contextual fitness $f$: per-run population medians and cumulative maxima (thin grey lines, $0.4\,\mathrm{pt}$) and across-run medians (thick lines, $1.0\,\mathrm{pt}$), averaged within logarithmic generation bins ($0, 1, 2\text{--}3, \ldots, 512\text{--}800$). Source: \code{docs/paper/data/gen800_population_density_and_traces.v1.json}.}
\label{fig:native-dynamics}
\end{figure}
"#;
    write_or_check(
        &root.join("docs/paper/generated/native-dynamics-fig.tex"),
        fig_tex,
        check,
    )?;
    Ok(())
}

fn verify_fallback_score(term: &Value, recorded_norm: f64) -> Result<()> {
    let fc = &term["fallback_constants"];
    let expected = if let Some(r_fb) = fc["ring_edge_r_fallback_mm"].as_f64() {
        let tgt = fc["target_mm"].as_f64().ok_or("missing target_mm")?;
        (r_fb / tgt).clamp(0.0, 1.0)
    } else if let Some(apex) = fc["apex_mm"].as_f64() {
        let tgt_apex = fc["target_apex_mm"].as_f64().ok_or("missing target_apex_mm")?;
        let ang = fc["half_angle_deg"].as_f64().ok_or("missing half_angle_deg")?;
        let tgt_ang = fc["target_half_angle_deg"]
            .as_f64()
            .ok_or("missing target_half_angle_deg")?;
        0.5 * (tgt_apex / apex).clamp(0.0, 1.0) + 0.5 * (tgt_ang / ang).clamp(0.0, 1.0)
    } else if let Some(t025) = fc["t_025_mm"].as_f64() {
        let tgt025 = fc["target_025_mm"].as_f64().ok_or("missing target_025_mm")?;
        let t100 = fc["t_100_mm"].as_f64().ok_or("missing t_100_mm")?;
        let tgt100 = fc["target_100_mm"].as_f64().ok_or("missing target_100_mm")?;
        let t200 = fc["t_200_mm"].as_f64().ok_or("missing t_200_mm")?;
        let tgt200 = fc["target_200_mm"].as_f64().ok_or("missing target_200_mm")?;
        let mono = fc["monotone_flag"].as_f64().unwrap_or(0.0);
        0.25 * ((tgt025 / t025).clamp(0.0, 1.0)
            + (tgt100 / t100).clamp(0.0, 1.0)
            + (tgt200 / t200).clamp(0.0, 1.0)
            + mono)
    } else if let Some(hm_t) = fc["hm_twist_sf"].as_f64() {
        let hm_s = fc["hm_spine_sf"].as_f64().ok_or("missing hm_spine_sf")?;
        let hm_r = fc["hm_ringback_sf_clamped"]
            .as_f64()
            .ok_or("missing hm_ringback_sf_clamped")?;
        (hm_t.clamp(0.0, 1.0) + hm_s.clamp(0.0, 1.0) + hm_r.clamp(0.0, 1.0)) / 3.0
    } else {
        return Err("unrecognized fallback_constants structure".into());
    };

    if (expected - recorded_norm).abs() > 2e-6 {
        return Err(format!(
            "fallback verification failed: expected {expected:.6}, recorded {recorded_norm:.6}"
        )
        .into());
    }
    Ok(())
}

fn generate_m8_score_decomposition(root: &Path, check: bool) -> Result<()> {
    let contract_path = root.join("docs/paper/data/term-evidence-class.v1.json");
    let scores_path = root.join(
        "experiments/pocket-surface-tension-smoothing/results/ec3i042_r01_p10_c56/scores.json",
    );
    let contract: Value = serde_json::from_str(&fs::read_to_string(&contract_path)?)?;
    let scores_doc: Value = serde_json::from_str(&fs::read_to_string(&scores_path)?)?;

    let runs = scores_doc["runs"].as_array().ok_or("missing runs in scores.json")?;
    let ctrl_objs = &runs
        .iter()
        .find(|r| r["name"].as_str() == Some("ctrl"))
        .ok_or("missing ctrl run")?["objectives"];
    let f075_objs = &runs
        .iter()
        .find(|r| r["name"].as_str() == Some("f075"))
        .ok_or("missing f075 run")?["objectives"];

    let mut class_weights: BTreeMap<String, f64> = BTreeMap::new();
    let mut class_pts_ctrl: BTreeMap<String, f64> = BTreeMap::new();
    let mut class_pts_f075: BTreeMap<String, f64> = BTreeMap::new();

    #[derive(Clone)]
    struct BarRow {
        cls: String,
        label: String,
        weight: f64,
        pts_ctrl: f64,
    }
    let mut bar_rows: Vec<BarRow> = Vec::new();
    let mut failed_req_weight = 0.0;
    let mut failed_req_earned_f075 = 0.0;

    let terms = contract["terms"].as_array().ok_or("missing terms in contract")?;
    for t in terms {
        let key = t["key"].as_str().ok_or("missing key")?;
        let w = t["weight_pct"].as_f64().ok_or("missing weight_pct")?;
        let cls = t["evidence_class"].as_str().ok_or("missing evidence_class")?;
        let failed_f075 = t["failed_requirement_in_f075"].as_bool().unwrap_or(false);

        let c_obj = &ctrl_objs[key];
        let f_obj = &f075_objs[key];
        let c_w = c_obj["weight_pct"].as_f64().ok_or("missing ctrl weight_pct")?;
        if (c_w - w).abs() > 1e-6 {
            return Err(format!("weight mismatch for {key}: {c_w} vs {w}").into());
        }
        let c_pts = c_obj["weighted_points"].as_f64().ok_or("missing ctrl weighted_points")?;
        let f_pts = f_obj["weighted_points"].as_f64().ok_or("missing f075 weighted_points")?;
        let c_norm = c_obj["normalized_score"].as_f64().ok_or("missing ctrl normalized_score")?;

        if failed_f075 {
            failed_req_weight += w;
            failed_req_earned_f075 += f_pts;
        }

        if cls == "split" {
            let parts = t["split_parts"].as_array().ok_or("missing split_parts")?;
            let p_meas = &parts[0];
            let p_fb = &parts[1];
            let w_m = p_meas["weight_pct"].as_f64().unwrap_or(4.5);
            let w_fb = p_fb["weight_pct"].as_f64().unwrap_or(4.5);
            let s_m = p_meas["expected_normalized_score"].as_f64().unwrap_or(1.0);
            // Derive fallback half normalized score from recorded combined score
            let s_fb_rec = (c_norm * (w_m + w_fb) - w_m * s_m) / w_fb;
            verify_fallback_score(p_fb, s_fb_rec)?;
            let pts_m = w_m * s_m;
            let pts_fb = c_pts - pts_m;

            *class_weights.entry("measured".into()).or_default() += w_m;
            *class_pts_ctrl.entry("measured".into()).or_default() += pts_m;
            *class_pts_f075.entry("measured".into()).or_default() += pts_m;

            *class_weights.entry("constant fallback".into()).or_default() += w_fb;
            *class_pts_ctrl.entry("constant fallback".into()).or_default() += pts_fb;
            *class_pts_f075.entry("constant fallback".into()).or_default() += f_pts - pts_m;

            bar_rows.push(BarRow {
                cls: "measured".into(),
                label: p_meas["label"].as_str().unwrap_or(key).into(),
                weight: w_m,
                pts_ctrl: pts_m,
            });
            bar_rows.push(BarRow {
                cls: "constant fallback".into(),
                label: p_fb["label"].as_str().unwrap_or(key).into(),
                weight: w_fb,
                pts_ctrl: pts_fb,
            });
        } else {
            if cls == "constant fallback" {
                verify_fallback_score(t, c_norm)?;
            }
            *class_weights.entry(cls.into()).or_default() += w;
            *class_pts_ctrl.entry(cls.into()).or_default() += c_pts;
            *class_pts_f075.entry(cls.into()).or_default() += f_pts;
            bar_rows.push(BarRow {
                cls: cls.into(),
                label: t["label"].as_str().unwrap_or(key).into(),
                weight: w,
                pts_ctrl: c_pts,
            });
        }
    }

    let meas_ctrl = class_pts_ctrl["measured"];
    let rule_ctrl = class_pts_ctrl["rule-based"];
    let fb_ctrl = class_pts_ctrl["constant fallback"];
    let na_ctrl = class_pts_ctrl["not admitted"];
    let meas_f075 = class_pts_f075["measured"];

    if (meas_ctrl - 41.441443).abs() > 1e-4
        || (rule_ctrl - 24.0).abs() > 1e-6
        || (fb_ctrl - 11.182353).abs() > 1e-4
        || (na_ctrl - 4.0).abs() > 1e-6
    {
        return Err(format!(
            "unexpected score decomposition totals: measured={meas_ctrl:.6}, rule={rule_ctrl:.6}, fallback={fb_ctrl:.6}, not_admitted={na_ctrl:.6}"
        )
        .into());
    }

    let mut tex = String::new();
    tex.push_str("% Generated by crates/paper-artifacts/examples/score_decomposition.rs -- do not hand-edit.\n");
    tex.push_str(&format!(
        "\\newcommand{{\\ScoreDecompMeasured}}{{{meas_ctrl:.2}}}\n\
         \\newcommand{{\\ScoreDecompMeasuredSmoothed}}{{{meas_f075:.2}}}\n\
         \\newcommand{{\\ScoreDecompRuleBased}}{{{rule_ctrl:.2}}}\n\
         \\newcommand{{\\ScoreDecompFallback}}{{{fb_ctrl:.2}}}\n\
         \\newcommand{{\\ScoreDecompNotAdmitted}}{{{na_ctrl:.2}}}\n\
         \\newcommand{{\\ScoreDecompFailedReqWeight}}{{{failed_req_weight:.0}}}\n\
         \\newcommand{{\\ScoreDecompFailedReqEarned}}{{{failed_req_earned_f075:.2}}}\n\n"
    ));

    // Emit horizontal meter-bar figure grouped by neutral class
    tex.push_str("\\newcommand{\\ScoreDecompositionFigure}{%\n");
    tex.push_str("\\begin{figure}[t]\n\\centering\n");
    tex.push_str("\\definecolor{trackblue}{HTML}{CDE2FB}\n");
    tex.push_str("\\begin{tikzpicture}[x=1mm,y=1mm]\n");

    let class_order = [
        ("measured", "Measured on exported CAD / FE", meas_ctrl, class_weights["measured"]),
        ("rule-based", "Rule-based geometry checks", rule_ctrl, class_weights["rule-based"]),
        ("constant fallback", "Constant fallback substitutes", fb_ctrl, class_weights["constant fallback"]),
        ("not admitted", "Not admitted as evidence", na_ctrl, class_weights["not admitted"]),
    ];

    let mut y = 0.0_f64;
    let bar_x0 = 44.5_f64;
    let scale = 1.32_f64; // 22 pts -> 29.04 mm

    for (cls_key, cls_title, cls_pts, cls_w) in class_order {
        tex.push_str(&format!(
            "\\node[font=\\sffamily\\bfseries\\fontsize{{6.4}}{{7.6}}\\selectfont, text=ink, anchor=west] at (0,{y:.1}) {{{cls_title} ({cls_pts:.2} / {cls_w:.1} pts)}};\n"
        ));
        y -= 3.3;
        for row in bar_rows.iter().filter(|r| r.cls == cls_key) {
            let w_mm = row.weight * scale;
            let p_mm = row.pts_ctrl * scale;
            let lbl = row.label.replace("&", "\\&").replace(">=", "$\\ge$");
            tex.push_str(&format!(
                "\\node[font=\\sffamily\\fontsize{{6.0}}{{7.0}}\\selectfont, text=slate, anchor=east] at ({:.1},{y:.1}) {{{lbl}}};\n",
                bar_x0 - 1.2
            ));
            tex.push_str(&format!(
                "\\fill[trackblue] ({bar_x0:.2},{:.2}) rectangle ({:.2},{:.2});\n",
                y - 1.1,
                bar_x0 + w_mm,
                y + 1.1
            ));
            tex.push_str(&format!(
                "\\fill[seriesA] ({bar_x0:.2},{:.2}) rectangle ({:.2},{:.2});\n",
                y - 1.1,
                bar_x0 + p_mm,
                y + 1.1
            ));
            tex.push_str(&format!(
                "\\node[font=\\sffamily\\fontsize{{5.8}}{{6.8}}\\selectfont, text=ink, anchor=west] at ({:.2},{y:.1}) {{{:.2}/{:.1}}};\n",
                bar_x0 + w_mm + 0.8,
                row.pts_ctrl,
                row.weight
            ));
            y -= 2.95;
        }
        y -= 1.2;
    }

    tex.push_str("\\end{tikzpicture}\n");
    tex.push_str("\\caption{Decomposition of the cover design's canonical \\code{W_ULTRALIGHT_v1} score ($F=\\AllRunsLeaderScore{}$ as exported; $\\PocketSmoothedScore{}$ after pocket smoothing) by evidence class under \\code{docs/paper/data/term-evidence-class.v1.json}: \\ScoreDecompMeasured{} points from exported CAD and finite-element measurements (\\ScoreDecompMeasuredSmoothed{} after smoothing), \\ScoreDecompRuleBased{} from rule-based geometry checks, \\ScoreDecompFallback{} from terms matching the scorer's constant fallbacks for missing export measurements, and \\ScoreDecompNotAdmitted{} from terms not admitted as engineering evidence. Light tracks show each term's weight; dark bars show earned points.}\n");
    tex.push_str("\\label{fig:score-decomposition}\n");
    tex.push_str("\\end{figure}%\n}\n");

    write_or_check(
        &root.join("docs/paper/generated/score-decomposition.tex"),
        &tex,
        check,
    )?;
    Ok(())
}

fn generate_m9_winner_requirements(root: &Path, check: bool) -> Result<()> {
    let sc_path = root.join(
        "experiments/pocket-surface-tension-smoothing/results/ec3i042_r01_p10_c56/re_ec3i042_r01_p10_c56_f075_master_scorecard.json",
    );
    let receipt_path = root.join(
        "experiments/pocket-surface-tension-smoothing/results/ec3i042_r01_p10_c56/RECEIPT.json",
    );
    let sc: Value = serde_json::from_str(&fs::read_to_string(&sc_path)?)?;
    let receipt: Value = serde_json::from_str(&fs::read_to_string(&receipt_path)?)?;

    let em = &sc["engineering_metrics"];
    let runs = receipt["runs"].as_array().ok_or("missing runs in RECEIPT.json")?;
    let ctrl = runs
        .iter()
        .find(|r| r["name"].as_str() == Some("ctrl"))
        .ok_or("missing ctrl in RECEIPT.json")?;
    let f075 = runs
        .iter()
        .find(|r| r["name"].as_str() == Some("f075"))
        .ok_or("missing f075 in RECEIPT.json")?;

    let cases = sc["cad_battery"]["cases"]
        .as_array()
        .ok_or("missing cad_battery.cases")?;
    let plate_loads = &cases
        .iter()
        .find(|c| c["id"].as_str() == Some("plate.loads"))
        .ok_or("missing plate.loads")?["detail"]["loads"];

    let lat_ctrl = ctrl["lat_rob_cert_n"].as_f64().unwrap_or(0.0);
    let lat_f075 = f075["lat_rob_cert_n"].as_f64().unwrap_or(0.0);
    let strict_ctrl = ctrl["strict_lat_rob_0p0625_n"].as_f64().unwrap_or(0.0);
    let strict_f075 = f075["strict_lat_rob_0p0625_n"].as_f64().unwrap_or(0.0);
    let ring_ctrl = ctrl["ring_rob_cert_n"].as_f64().unwrap_or(0.0);
    let ring_f075 = f075["ring_rob_cert_n"].as_f64().unwrap_or(0.0);
    let body_tw_ctrl = ctrl["twist_body_sf"].as_f64().unwrap_or(0.0);
    let body_tw_f075 = f075["twist_body_sf"].as_f64().unwrap_or(0.0);
    let tip_tw_cert = em["twist_tipzone_sf_cert"].as_f64().unwrap_or(0.0);
    let tip_tw_strict = em["twist_tipzone_sf_strict"].as_f64().unwrap_or(0.0);
    let solid_max_ctrl = ctrl["solid_tip_sf_nodal_max"].as_f64().unwrap_or(0.0);
    let solid_avg_ctrl = ctrl["solid_tip_sf_nodal_avg"].as_f64().unwrap_or(0.0);
    let solid_max_f075 = f075["solid_tip_sf_nodal_max"].as_f64().unwrap_or(0.0);
    let solid_avg_f075 = f075["solid_tip_sf_nodal_avg"].as_f64().unwrap_or(0.0);
    let seq_ctrl = ctrl["seq_ratio_cert"].as_f64().unwrap_or(0.0);
    let seq_f075 = f075["seq_ratio_cert"].as_f64().unwrap_or(0.0);
    let fat_f075 = f075["fat_sf_l7"].as_f64().unwrap_or(0.0);
    let hm_twist = plate_loads["chord_twist_human_max_sf"]["value_n_or_sf"]
        .as_f64()
        .unwrap_or(0.0);
    let hm_spine = plate_loads["spine_sf"]["value_n_or_sf"]
        .as_f64()
        .unwrap_or(0.0);
    let hm_ringback = plate_loads["ring_back_sf"]["value_n_or_sf"]
        .as_f64()
        .unwrap_or(0.0);
    let mass_ctrl = ctrl["mass_3d_g"].as_f64().unwrap_or(0.0);
    let mass_f075 = f075["mass_3d_g"].as_f64().unwrap_or(0.0);

    let tex = format!(
        r#"% Generated by crates/paper-artifacts/examples/score_decomposition.rs -- do not hand-edit.
\begin{{table}}[t]
\centering
\caption{{Cover design (\code{{ec3i042_r01_p10_c56}}) against reference checks (\textbf{{informational; does not veto selection}}). Values come from the retained Stage-2 scorecard and \code{{RECEIPT.json}}.}}
\label{{tab:winner-requirements}}
\setlength{{\tabcolsep}}{{2.8pt}}
\begin{{tabular}}{{@{{}}> {{\raggedright\arraybackslash}}p{{0.40\columnwidth}}rrr@{{}}}}
\toprule
\textbf{{Requirement}} & \textbf{{Reference}} & \textbf{{Measured}} & \textbf{{Status}} \\
\midrule
Lateral, hiker & 180\,N & \ifdefined\JRSmoothedVariant {lat_f075:.1}\else {lat_ctrl:.1}\fi\,N & met $\checkmark$ \\
Lateral, hunter & 260\,N & \ifdefined\JRSmoothedVariant {lat_f075:.1}\else {lat_ctrl:.1}\fi\,N & not met $\times$ \\
Lateral, hunter (strict mesh) & 260\,N & \ifdefined\JRSmoothedVariant {strict_f075:.1}\else {strict_ctrl:.1}\fi\,N & not met $\times$ \\
Ring pull & 1500\,N & \ifdefined\JRSmoothedVariant \num{{{ring_f075:.0}}}\else \num{{{ring_ctrl:.0}}}\fi\,N & met $\checkmark$ \\
Body torsion SF (1.82\,N\,m) & 1.0 & \ifdefined\JRSmoothedVariant {body_tw_f075:.2}\else {body_tw_ctrl:.2}\fi & met $\checkmark$ \\
Tip torsion SF (plate cert./strict) & 1.0 & {tip_tw_cert:.2} / {tip_tw_strict:.2} & not met $\times$ \\
3D solid-tip SF (nodal max/mean) & 1.0 & \ifdefined\JRSmoothedVariant {solid_max_f075:.3} / {solid_avg_f075:.3}\else {solid_max_ctrl:.3} / {solid_avg_ctrl:.3}\fi & not met $\times$ \\
Failure-sequence ratio & 1.20 & \ifdefined\JRSmoothedVariant {seq_f075:.2}\else {seq_ctrl:.2}\fi & \ifdefined\JRSmoothedVariant met $\checkmark$\else not met $\times$\fi \\
Goodman fatigue SF (0--180\,N) & 1.0 & {fat_f075:.2} & not met $\times$ \\
Human-max twist / spine SF & 1.0 & {hm_twist:.2} / {hm_spine:.2} & not met $\times$ \\
Human-max ring-back SF & 1.0 & {hm_ringback:.2} & met $\checkmark$ \\
Edge-sharpness law (export) & law & not measured & not met $\times$ \\
Tip point profile (B-rep) & law & not measured & not met $\times$ \\
Single watertight STEP/STL solid & yes & yes & met $\checkmark$ \\
3D mass (continuous preference) & 45--130\,g & \ifdefined\JRSmoothedVariant {mass_f075:.2}\else {mass_ctrl:.2}\fi\,g & informational \\
\bottomrule
\end{{tabular}}
\end{{table}}
"#
    );

    write_or_check(
        &root.join("docs/paper/generated/winner-requirements.tex"),
        &tex,
        check,
    )?;
    Ok(())
}

fn main() -> Result<()> {
    let check = env::args().any(|a| a == "--check");
    let root = repo_root();
    generate_m7_native_dynamics(&root, check)?;
    generate_m8_score_decomposition(&root, check)?;
    generate_m9_winner_requirements(&root, check)?;
    println!(
        "score_decomposition: {} OK (M7 native-dynamics, M8 score-decomposition, M9 winner-requirements)",
        if check { "check" } else { "generated" }
    );
    Ok(())
}
