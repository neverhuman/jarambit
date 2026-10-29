use crate::hand::teeth_census;
use jarambit_g5::g5_qd::serration_tooth_strength;
use jarambit_g5::g5_sdf::build_blade_profile_2d;

/// The census's minimum tooth SF on each edge is G5.0's `serration_tooth_strength`, bit for bit, and it counts the
/// same teeth.
#[test]
fn teeth_census_reproduces_g5_tooth_strength() {
    let mut serrated = 0usize;
    for i in 1..60 {
        let g = jarambit_g5::g5_genome::sobol_unit_genome(i, 11, crate::namespace::G51_GENOME_COHORT_LABEL);
        let p = g.decode();
        let blade = build_blade_profile_2d(&p);
        for inner in [true, false] {
            let census = teeth_census(&p, &blade, inner);
            let (sf, teeth, _) = serration_tooth_strength(&p, &blade, inner);
            let min = census.iter().map(|t| t.sf).fold(f64::INFINITY, f64::min);
            assert_eq!(min.to_bits(), sf.to_bits(), "design {i} inner {inner}");
            assert_eq!(census.iter().filter(|t| t.kt.is_finite()).count(), teeth, "design {i} inner {inner}");
            for t in &census {
                assert_eq!(t.sf.to_bits(), t.snag_sf.min(t.pry_sf).to_bits());
            }
            serrated += usize::from(!census.is_empty());
        }
    }
    assert!(serrated > 20, "{serrated}");
}
