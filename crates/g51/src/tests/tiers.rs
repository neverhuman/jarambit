use crate::hash::g51_decoder_sha256;
use crate::tiers::{evaluate_tier0, evaluate_tier1_sdf_and_plate, new_archive, verify_archive};
use jarambit_g5::g5_genome::{G5Genome, G5_GENES};
use jarambit_g5::g5_qd::{evaluate_tier0_analytic, evaluate_tier1_sdf_and_plate as g5_tier1, G5EvalRecord};

/// The same record bit for bit, apart from the decoder hash.
fn assert_same_record(a: &G5EvalRecord, b: &G5EvalRecord) {
    let strip = |r: &G5EvalRecord| {
        let mut v = serde_json::to_value(r).expect("record json");
        v["decoder_sha256"] = serde_json::Value::Null;
        v.to_string()
    };
    assert_eq!(strip(a), strip(b));
    assert_eq!(a.score.raw_score_100.to_bits(), b.score.raw_score_100.to_bits());
}

fn without_blade_texture(mut g: G5Genome) -> G5Genome {
    let k = G5_GENES.iter().position(|s| s.name == "scope_blade").expect("scope_blade gene");
    g.u[k] = 0.0;
    g
}

/// Tier 0 is G5.0's (the hand block still carries G5.0's analytic SFs), under the G5.1 decoder hash.
#[test]
fn tier0_is_g5s_under_the_g51_hash() {
    for g in super::textured_blade_designs(4, 0.0) {
        let (r, hand) = evaluate_tier0(&g);
        let r5 = evaluate_tier0_analytic(&g);
        assert_same_record(&r, &r5);
        assert_eq!(r.decoder_sha256, g51_decoder_sha256());
        assert_eq!(hand.teeth.scored.to_bits(), r5.loads.teeth_sf.to_bits());
    }
}

/// With no blade texture the bevel mask has nothing to do, and Tier 1 is G5.0's bit for bit. With blade texture
/// the load cases are unchanged and the AM mass can only grow (the mask keeps material G5.0's texture removed).
#[test]
fn tier1_is_g5s_off_the_bevels() {
    let designs = super::textured_blade_designs(1, 0.03);
    let g = designs[0].clone();
    let plain = without_blade_texture(g.clone());
    let (r, _) = evaluate_tier1_sdf_and_plate(&plain, 0.5);
    assert_same_record(&r, &g5_tier1(&plain, 0.5));

    let (r, _) = evaluate_tier1_sdf_and_plate(&g, 0.5);
    let r5 = g5_tier1(&g, 0.5);
    assert_eq!(serde_json::to_string(&r.loads).unwrap(), serde_json::to_string(&r5.loads).unwrap());
    assert!(r.inputs.mass_g >= r5.inputs.mass_g, "{} < {}", r.inputs.mass_g, r5.inputs.mass_g);
    assert!(r.inputs.mass_g - r5.inputs.mass_g < 0.01 * r5.inputs.mass_g);
}

/// A G5.1 archive holds G5.1 records and verifies; a G5.0 record is refused.
#[test]
fn g51_archive_refuses_g5_records() {
    let g = super::textured_blade_designs(1, 0.0)[0].clone();
    let mut a = new_archive();
    let (r, _) = evaluate_tier0(&g);
    assert!(a.insert(g.clone(), r));
    verify_archive(&a).expect("G5.1 archive verifies");
    let r5 = evaluate_tier0_analytic(&g);
    let refused = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| new_archive().insert(g, r5)));
    assert!(refused.is_err(), "a G5.0 record entered a G5.1 archive");
}
