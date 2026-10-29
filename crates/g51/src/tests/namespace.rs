use crate::namespace::*;

#[test]
fn ids_and_run_roots_keep_generations_apart() {
    let sha = "29ea3c73aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let id = format_g51_design_id(4, 1404, 581, sha);
    assert_eq!(id, "kw4i1404_g00581_29ea3c73");
    assert!(validate_g51_pool_design_id(&id).is_ok());
    assert!(validate_g51_pool_design_id("kv4i1404_g00581_29ea3c73").is_ok(), "inherited G5.0 ids stay valid");
    for bad in ["ec3i042_r01_p10_c56", "hz4i013_g00995_d206e3c9", &format_g51_doe_id("grip", 7, sha), "xx1"] {
        assert!(validate_g51_pool_design_id(bad).is_err(), "{bad}");
    }
    let dev = "/home/ubuntu/jarambit_runs/evo_g51_dev/doe";
    let rec = "/home/ubuntu/jarambit_runs/evo_g51/epoch01";
    assert_eq!(validate_g51_run_root(dev).is_ok(), !G51_DECODER_FROZEN);
    assert_eq!(validate_g51_run_root(rec).is_ok(), G51_DECODER_FROZEN);
    for bad in ["/tmp/evo_g51_dev", "/home/ubuntu/jarambit_runs/evo_continuous/evo_g51_dev", "evo_g51_dev"] {
        assert!(validate_g51_run_root(bad).is_err(), "{bad}");
    }
}
