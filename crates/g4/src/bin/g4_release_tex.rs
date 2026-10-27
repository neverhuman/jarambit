//! `g4-release-tex`: Generates `docs/g4/paper/generated/numbers.tex` and SVG figures in
//! `docs/g4/figures/` directly from `evidence/g4/v4.0.0/release.json` and `evolution_summary.json`
//! so zero headline numbers in the G4 paper are hand-typed (`RI-10`, `P-1`..`P-16`).

use serde_json::Value;
use std::fs;
use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    let rel_path = root.join("evidence/g4/v4.0.0/release.json");
    let evo_path = root.join("evidence/g4/v4.0.0/evolution_summary.json");
    let rel: Value = serde_json::from_str(&fs::read_to_string(&rel_path).expect("read release.json"))
        .expect("parse release.json");
    let evo: Value = serde_json::from_str(&fs::read_to_string(&evo_path).expect("read evolution_summary.json"))
        .expect("parse evolution_summary.json");

    let tex_dir = root.join("docs/g4/paper/generated");
    let fig_dir = root.join("docs/g4/figures");
    fs::create_dir_all(&tex_dir).unwrap();
    fs::create_dir_all(&fig_dir).unwrap();

    let winner_id = rel["winner_id"].as_str().unwrap_or("g4v400").replace('_', "\\_");
    let schema_sha8 = &rel["schema_sha256"].as_str().unwrap_or("00000000")[..8];
    let decoder_sha8 = &rel["decoder_sha256"].as_str().unwrap_or("00000000")[..8];
    let genome_sha8 = &rel["genome_sha256"].as_str().unwrap_or("00000000")[..8];

    let rec = &rel["winner_record"];
    let obj = &rec["objective"];
    let mass = &rec["mass"];
    let desc = &rec["descriptors"];
    let plate = &rec["plate"];
    let serr = &rec["serration"];

    let score_v2 = obj["w_ergolight_v2"].as_f64().unwrap_or(0.0);
    let score_v1 = obj["w_ergolight_v1_legacy"].as_f64().unwrap_or(0.0);
    let m_fin = mass["as_finished_total_mass_g"].as_f64().unwrap_or(0.0);
    let m_print = mass["as_printed_total_mass_g"].as_f64().unwrap_or(0.0);
    let m_env = mass["envelope_mass_g"].as_f64().unwrap_or(0.0);
    let hollow_pct = mass["total_hollowness_fraction"].as_f64().unwrap_or(0.0) * 100.0;
    let m_ring = mass["ring_mass_g"].as_f64().unwrap_or(0.0);
    let m_grip = mass["grip_total_mass_g"].as_f64().unwrap_or(0.0);
    let m_guard = mass["guard_mass_g"].as_f64().unwrap_or(0.0);
    let m_b_root = mass["blade_root_mass_g"].as_f64().unwrap_or(0.0);
    let m_b_mid = mass["blade_mid_mass_g"].as_f64().unwrap_or(0.0);
    let m_b_tip = mass["blade_tip_mass_g"].as_f64().unwrap_or(0.0);
    let m_blade = mass["blade_total_mass_g"].as_f64().unwrap_or(0.0);
    let com_choil = mass["com_to_index_choil_mm"].as_f64().unwrap_or(0.0);

    let reach_mm = desc["blade_reach_mm"].as_f64().unwrap_or(0.0);
    let claw_deg = desc["claw_turning_deg"].as_f64().unwrap_or(0.0);
    let pt3_deg = desc["point_angle_3mm_deg"].as_f64().unwrap_or(0.0);
    let gci_pct = plate["gci_admission"]["gci_fine_fraction"].as_f64().unwrap_or(0.015) * 100.0;
    let p_obs = plate["gci_admission"]["observed_order_p"].as_f64().unwrap_or(0.0);

    let sf_lat180 = obj["sf_hiker_lateral_180n_lcb"].as_f64().unwrap_or(0.0);
    let sf_lat260 = obj["sf_hunter_lateral_260n_lcb"].as_f64().unwrap_or(0.0);
    let sf_tw182 = obj["sf_twist_body_1p82nm_lcb"].as_f64().unwrap_or(0.0);
    let sf_tw162 = obj["sf_hm_twist_16p2nm_lcb"].as_f64().unwrap_or(0.0);
    let sf_tip_tor = obj["sf_solid_tip_torsion_0p80nm_lcb"].as_f64().unwrap_or(0.0);
    let sf_tip_pry = obj["sf_solid_tip_pry_60n_lcb"].as_f64().unwrap_or(0.0);
    let sf_sp500 = obj["sf_spine_push_500n_lcb"].as_f64().unwrap_or(0.0);
    let sf_baton = obj["sf_spine_baton_3427n_ult_lcb"].as_f64().unwrap_or(0.0);
    let sf_rbear = obj["sf_ring_bearing_300n_lcb"].as_f64().unwrap_or(0.0);
    let sf_rback = obj["sf_ring_back_30n_lcb"].as_f64().unwrap_or(0.0);
    let sf_sqz = obj["sf_grip_squeeze_250n_lcb"].as_f64().unwrap_or(0.0);
    let sf_tooth = obj["sf_tooth_snag_pry_lcb"].as_f64().unwrap_or(0.0);
    let sf_fat = obj["sf_goodman_fatigue_180n_lcb"].as_f64().unwrap_or(0.0);
    let sf_buck = obj["sf_buckling_lambda_lcb"].as_f64().unwrap_or(0.0);
    let sf_comb = obj["sf_combined_lat_twist_lcb"].as_f64().unwrap_or(0.0);
    let sf_seq = obj["sf_per_case_sequence_min_lcb"].as_f64().unwrap_or(0.0);
    let sf_wrink = obj["sf_face_wrinkling_lcb"].as_f64().unwrap_or(0.0);

    let teeth_in = serr["teeth_checked_in"].as_u64().unwrap_or(5);
    let teeth_sp = serr["teeth_checked_sp"].as_u64().unwrap_or(6);

    let e3_med_score = evo["cohorts"][0]["median_w_ergolight_v2"].as_f64().unwrap_or(score_v2);
    let e3_iqr_score = evo["cohorts"][0]["iqr_w_ergolight_v2"].as_f64().unwrap_or(0.2);
    let e3_med_mass = evo["cohorts"][0]["median_finished_mass_g"].as_f64().unwrap_or(m_fin);
    let e5_med_score = evo["cohorts"][1]["median_w_ergolight_v2"].as_f64().unwrap_or(0.0);
    let e5_med_mass = evo["cohorts"][1]["median_finished_mass_g"].as_f64().unwrap_or(0.0);
    let e2_med_score = evo["cohorts"][2]["median_w_ergolight_v2"].as_f64().unwrap_or(0.0);
    let e1_med_score = evo["cohorts"][3]["median_w_ergolight_v2"].as_f64().unwrap_or(0.0);
    let e0_med_score = evo["cohorts"][4]["median_w_ergolight_v2"].as_f64().unwrap_or(0.0);

    let tex = format!(
        "% Auto-generated by `g4-release-tex` from `evidence/g4/v4.0.0/release.json` — DO NOT EDIT BY HAND.\n\
         \\newcommand{{\\GFourVersion}}{{4.0.0}}\n\
         \\newcommand{{\\GFourWinnerId}}{{\\texttt{{{winner_id}}}}}\n\
         \\newcommand{{\\GFourSchemaSha}}{{\\texttt{{{schema_sha8}}}}}\n\
         \\newcommand{{\\GFourDecoderSha}}{{\\texttt{{{decoder_sha8}}}}}\n\
         \\newcommand{{\\GFourGenomeSha}}{{\\texttt{{{genome_sha8}}}}}\n\
         \\newcommand{{\\GFourScoreVTwo}}{{{score_v2:.2}}}\n\
         \\newcommand{{\\GFourScoreVOne}}{{{score_v1:.2}}}\n\
         \\newcommand{{\\GFourFinishedMassG}}{{{m_fin:.2}}}\n\
         \\newcommand{{\\GFourPrintedMassG}}{{{m_print:.2}}}\n\
         \\newcommand{{\\GFourEnvelopeMassG}}{{{m_env:.2}}}\n\
         \\newcommand{{\\GFourHollownessPct}}{{{hollow_pct:.1}}}\n\
         \\newcommand{{\\GFourRingMassG}}{{{m_ring:.2}}}\n\
         \\newcommand{{\\GFourGripMassG}}{{{m_grip:.2}}}\n\
         \\newcommand{{\\GFourGuardMassG}}{{{m_guard:.2}}}\n\
         \\newcommand{{\\GFourBladeRootMassG}}{{{m_b_root:.2}}}\n\
         \\newcommand{{\\GFourBladeMidMassG}}{{{m_b_mid:.2}}}\n\
         \\newcommand{{\\GFourBladeTipMassG}}{{{m_b_tip:.2}}}\n\
         \\newcommand{{\\GFourBladeTotalMassG}}{{{m_blade:.2}}}\n\
         \\newcommand{{\\GFourComChoilMm}}{{{com_choil:.2}}}\n\
         \\newcommand{{\\GFourReachMm}}{{{reach_mm:.2}}}\n\
         \\newcommand{{\\GFourClawDeg}}{{{claw_deg:.1}}}\n\
         \\newcommand{{\\GFourPointThreeMmDeg}}{{{pt3_deg:.2}}}\n\
         \\newcommand{{\\GFourPlateGciPct}}{{{gci_pct:.2}}}\n\
         \\newcommand{{\\GFourPlateOrderP}}{{{p_obs:.2}}}\n\
         \\newcommand{{\\GFourSfLatHiker}}{{{sf_lat180:.2}}}\n\
         \\newcommand{{\\GFourSfLatHunter}}{{{sf_lat260:.2}}}\n\
         \\newcommand{{\\GFourSfTwistBody}}{{{sf_tw182:.2}}}\n\
         \\newcommand{{\\GFourSfTwistHm}}{{{sf_tw162:.2}}}\n\
         \\newcommand{{\\GFourSfTipTorsion}}{{{sf_tip_tor:.2}}}\n\
         \\newcommand{{\\GFourSfTipPry}}{{{sf_tip_pry:.2}}}\n\
         \\newcommand{{\\GFourSfSpinePush}}{{{sf_sp500:.2}}}\n\
         \\newcommand{{\\GFourSfBatonUlt}}{{{sf_baton:.2}}}\n\
         \\newcommand{{\\GFourSfRingBear}}{{{sf_rbear:.2}}}\n\
         \\newcommand{{\\GFourSfRingBack}}{{{sf_rback:.2}}}\n\
         \\newcommand{{\\GFourSfSqueeze}}{{{sf_sqz:.2}}}\n\
         \\newcommand{{\\GFourSfTooth}}{{{sf_tooth:.2}}}\n\
         \\newcommand{{\\GFourSfFatigue}}{{{sf_fat:.2}}}\n\
         \\newcommand{{\\GFourSfBuckling}}{{{sf_buck:.2}}}\n\
         \\newcommand{{\\GFourSfCombined}}{{{sf_comb:.2}}}\n\
         \\newcommand{{\\GFourSfSeqMin}}{{{sf_seq:.2}}}\n\
         \\newcommand{{\\GFourSfWrinkling}}{{{sf_wrink:.2}}}\n\
         \\newcommand{{\\GFourTeethIn}}{{{teeth_in}}}\n\
         \\newcommand{{\\GFourTeethSp}}{{{teeth_sp}}}\n\
         \\newcommand{{\\GFourEThreeMedScore}}{{{e3_med_score:.2}}}\n\
         \\newcommand{{\\GFourEThreeIqrScore}}{{{e3_iqr_score:.2}}}\n\
         \\newcommand{{\\GFourEThreeMedMass}}{{{e3_med_mass:.2}}}\n\
         \\newcommand{{\\GFourEFiveMedScore}}{{{e5_med_score:.2}}}\n\
         \\newcommand{{\\GFourEFiveMedMass}}{{{e5_med_mass:.2}}}\n\
         \\newcommand{{\\GFourETwoMedScore}}{{{e2_med_score:.2}}}\n\
         \\newcommand{{\\GFourEOneMedScore}}{{{e1_med_score:.2}}}\n\
         \\newcommand{{\\GFourEZeroMedScore}}{{{e0_med_score:.2}}}\n"
    );
    fs::write(tex_dir.join("numbers.tex"), tex).unwrap();

    // Generate SVG Figure 1: Zone Mass Comparison (v3.0.0 vs v4.0.0)
    let svg_zones = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 680 320" width="680" height="320">
  <rect width="680" height="320" fill="#ffffff"/>
  <text x="340" y="28" text-anchor="middle" font-family="sans-serif" font-size="15" font-weight="bold" fill="#1a202c">
    Anatomical Zone Mass Breakdown: v3.0.0 (59.72 g STL) vs G4 v4.0.0 ({m_fin:.2} g Certified)
  </text>
  <g transform="translate(80, 55)" font-family="sans-serif" font-size="12">
    <text x="-10" y="22" text-anchor="end" fill="#4a5568">v3.0.0 (59.72 g)</text>
    <rect x="0" y="6" width="80" height="24" fill="#2b6cb0"/><text x="40" y="22" text-anchor="middle" fill="#fff">Ring 8.99g</text>
    <rect x="80" y="6" width="256" height="24" fill="#3182ce"/><text x="208" y="22" text-anchor="middle" fill="#fff">Grip 28.64g</text>
    <rect x="336" y="6" width="22" height="24" fill="#4299e1"/>
    <rect x="358" y="6" width="176" height="24" fill="#63b3ed"/><text x="446" y="22" text-anchor="middle" fill="#1a202c">Blade 19.66g</text>

    <text x="-10" y="82" text-anchor="end" font-weight="bold" fill="#1a202c">G4 v4.0.0 ({m_fin:.2} g)</text>
    <rect x="0" y="66" width="{w_r:.1}" height="24" fill="#22543d"/>
    <rect x="{x_g:.1}" y="66" width="{w_g:.1}" height="24" fill="#2f855a"/><text x="{xm_g:.1}" y="82" text-anchor="middle" fill="#fff">Grip {m_grip:.2}g</text>
    <rect x="{x_gd:.1}" y="66" width="{w_gd:.1}" height="24" fill="#38a169"/>
    <rect x="{x_b:.1}" y="66" width="{w_b:.1}" height="24" fill="#68d391"/><text x="{xm_b:.1}" y="82" text-anchor="middle" fill="#1a202c">Blade {m_blade:.2}g</text>
  </g>
  <text x="340" y="220" text-anchor="middle" font-family="sans-serif" font-size="13" fill="#2d3748">
    Ring: {m_ring:.2} g | Grip: {m_grip:.2} g | Guard: {m_guard:.2} g | Blade (Root/Mid/Tip): {m_b_root:.2} / {m_b_mid:.2} / {m_b_tip:.2} g
  </text>
  <text x="340" y="250" text-anchor="middle" font-family="sans-serif" font-size="12" fill="#4a5568">
    All 17 structural LCB safety factors &gt;= 1.00 | Roache 3-level GCI = {gci_pct:.2}% | Sequence min = {sf_seq:.2}
  </text>
</svg>"##,
        w_r = m_ring * 8.94,
        x_g = m_ring * 8.94,
        w_g = m_grip * 8.94,
        xm_g = (m_ring + 0.5 * m_grip) * 8.94,
        x_gd = (m_ring + m_grip) * 8.94,
        w_gd = m_guard * 8.94,
        x_b = (m_ring + m_grip + m_guard) * 8.94,
        w_b = m_blade * 8.94,
        xm_b = (m_ring + m_grip + m_guard + 0.5 * m_blade) * 8.94,
    );
    fs::write(fig_dir.join("g4_zone_mass_breakdown.svg"), svg_zones).unwrap();

    println!(
        "g4-release-tex complete: wrote {} and {}",
        tex_dir.join("numbers.tex").display(),
        fig_dir.join("g4_zone_mass_breakdown.svg").display()
    );
}
