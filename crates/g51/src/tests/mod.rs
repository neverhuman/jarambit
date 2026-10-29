//! Tests of the G5.1 crate (outside the decoder hash, so tests can be added after the freeze).

mod fe;
mod guards;
mod hand;
mod hash;
mod namespace;
mod sdf;
mod tiers;

/// Sobol designs of the G5.0 genome with blade texture switched on (`tex_amp × scope_blade` at least `min_depth`),
/// so the bevel mask has something to remove.
pub(crate) fn textured_blade_designs(n: usize, min_depth: f64) -> Vec<jarambit_g5::g5_genome::G5Genome> {
    (1..400)
        .map(|i| jarambit_g5::g5_genome::sobol_unit_genome(i, 7, crate::namespace::G51_GENOME_COHORT_LABEL))
        .filter(|g| {
            let p = g.decode();
            p["tex_amp"] * p["scope_blade"] >= min_depth
        })
        .take(n)
        .collect()
}
