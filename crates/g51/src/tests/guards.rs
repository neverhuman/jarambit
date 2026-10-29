/// G5.1 builds on G5.0 unchanged: the G5.0 decoder hash, recomputed from the sources this crate links, still equals
/// the hash of the v5.0.0 cohort of record (and v6.0.0's, which continued it).
#[test]
fn g5_decoder_of_record_is_unchanged() {
    for receipt in [
        include_str!("../../../../evidence/g5/v5.0.0/release.json"),
        include_str!("../../../../evidence/g5/v6.0.0/release.json"),
    ] {
        let r: serde_json::Value = serde_json::from_str(receipt).expect("G5 receipt");
        let recorded = r["provenance"]["decoder_sha256"].as_str().expect("provenance.decoder_sha256");
        assert_eq!(jarambit_g5::g5_qd::g5_decoder_sha256_uncached(), recorded, "the G5.0 decoder of record changed");
    }
    assert!(jarambit_g5::g5_namespace::G5_DECODER_FROZEN);
}
