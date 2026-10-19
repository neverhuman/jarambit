//! Exercise the public command path: unavailable independent solvers cannot produce a certificate.
use std::{path::PathBuf, process::Command};
#[test]
fn top_k_cli_keeps_an_unavailable_runtime_unverified() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let out = root
        .join("target/evolve-process-controls")
        .join(unique.to_string());
    let result = Command::new(env!("CARGO_BIN_EXE_jarambit"))
        .args([
            "auto-evolve",
            "--rounds",
            "1",
            "--candidates-per-round",
            "1",
            "--top-k-certify",
            "1",
            "--threads",
            "1",
            "--seed",
            "17",
            "--seeds",
            "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_genome.csv",
            "--prefix",
            "runtime_control",
            "--out-dir",
        ])
        .arg(&out)
        .current_dir(&root)
        .env(
            "JARAMBIT_SOLVER_PYTHON",
            "/missing/independent-solver-python",
        )
        .output()
        .unwrap();
    assert!(!result.status.success());
    std::fs::write(out.join("command_stdout.txt"), &result.stdout).unwrap();
    std::fs::write(out.join("command_stderr.txt"), &result.stderr).unwrap();
    let report: serde_json::Value = serde_json::from_slice(
        &std::fs::read(out.join("leaderboard.json")).expect("attempt report even when unverified"),
    )
    .unwrap();
    assert_eq!(report["qualified_count"], 0);
    assert!(report["champion"].is_null());
    assert_eq!(report["designs"].as_array().unwrap().len(), 0);
    let errors = report["unverified"].as_array().unwrap();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["status"], "unverified");
    assert!(errors[0]["reason"]
        .as_str()
        .unwrap()
        .contains("independent certification"));
    assert!(out.join("round_00_births.json").is_file());
}

#[test]
fn scorecard_identity_nonfinite_and_independent_disagreement_fail_closed_through_cli() {
    use std::os::unix::fs::PermissionsExt;
    let root=PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let stamp=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let folder=root.join("target/evolve-process-controls").join(stamp.to_string());
    std::fs::create_dir_all(&folder).unwrap();
    let fake=folder.join("synthetic_evaluator.py");
    std::fs::write(&fake,r#"#!/usr/bin/python3
# Software boundary controls only. This process performs no physical solve.
import json,os,sys
from pathlib import Path
a=sys.argv
out=Path(a[a.index('--out-dir')+1]);label=a[a.index('--label')+1]
m=json.loads(Path('evidence/evolve_certifiable/manifest.json').read_text())
sc=json.loads(Path(m['automatic']['artifacts']['scorecard']).read_text())
sc['certificate_identity']=out.parent.name
sc['ultralight_weighted_fitness']={'global_weighted_fitness':74.319}
mode=os.environ['EVO_TEST_MODE']
if mode=='identity':sc['certificate_identity']='wrong'
if mode=='nonfinite':sc['engineering_metrics']['mass_g']=None
if mode=='disagreement':
 sc['engineering_metrics']['rs_v2_all_hard']=True
 sc['engineering_metrics']['solid_tip_dolfinx_rel_err']=.2
 sc['weighted_cad_score']['gates']={k:True for k in sc['weighted_cad_score']['gates']}
 sc['weighted_cad_score']['gates']['dolfinx_cross_check_2pct']=False
 sc['weighted_cad_score']['all_engineering_gates_pass']=False
(out/(label+'_master_scorecard.json')).write_text(json.dumps(sc))
"#).unwrap();
    std::fs::set_permissions(&fake,std::fs::Permissions::from_mode(0o755)).unwrap();
    for mode in ["identity","nonfinite","disagreement"] {
        let out=folder.join(mode);
        let result=Command::new(env!("CARGO_BIN_EXE_jarambit"))
            .args(["auto-evolve","--rounds","1","--candidates-per-round","1","--top-k-certify","1","--threads","1","--seeds","evidence/rs_v2/w283_ultrarunner/w283_p14_c01_genome.csv","--out-dir"])
            .arg(&out).current_dir(&root).env("JARAMBIT_SOLVER_PYTHON",&fake).env("EVO_TEST_MODE",mode).output().unwrap();
        std::fs::write(out.join("command_stdout.txt"),&result.stdout).unwrap();
        std::fs::write(out.join("command_stderr.txt"),&result.stderr).unwrap();
        let report:serde_json::Value=serde_json::from_slice(&std::fs::read(out.join("leaderboard.json")).unwrap()).unwrap();
        assert_eq!(report["qualified_count"],0);assert!(report["champion"].is_null());
        if mode=="disagreement" {assert!(result.status.success());assert_eq!(report["designs"][0]["all_16_gates"],false);}
        else {assert!(!result.status.success());assert_eq!(report["unverified"].as_array().unwrap().len(),1);
            let reason=report["unverified"][0]["reason"].as_str().unwrap();
            assert!(reason.contains(if mode=="identity" {"different genome/model"}else{"missing/nonfinite"}),"{reason}");}
    }
}
