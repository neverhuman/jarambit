# Phase-8 synthetic GCI arithmetic audit

PASS: 24 synthetic sequences, 39 triplets, all seven native fit statuses. Twelve five-number sequences use f(h)=2+c*h^p with p in {0.5,1,1.5,2,2.5,3}, r=2 or sqrt(2), and alternating signs of c. Twelve controls cover oscillation, constants, pair plateaus, negative/zero order, invalid ratios, precision floor, zero reference and duplicate synthetic identities.

| Native status | Triplets |
|---|---:|
| admissible_estimate | 24 |
| invalid_input | 2 |
| nonpositive_order | 3 |
| oscillatory | 3 |
| precision_limited | 1 |
| unresolved | 5 |
| unsupported_sequence | 1 |

Inputs are ordered finest to coarsest; five-number cases use slices [0:3] and [2:5]. Each triplet checks expected status, monotonicity, error availability and admission. Inputs cross the Rust boundary as IEEE-754 bit strings. Native output p, f_ext and gci_fine match the existing Python reference bit for bit for every triplet. Two independent executions of the same native binary produce identical output bytes.

For every polynomial triplet, recovered order and extrapolate agree with the manufactured p and f0 within 1e-11 relative/absolute tolerance; relative GCI agrees with Fs*abs(c*h^p)/abs(f1) within 1e-11 relative or 1e-13 absolute tolerance. Fs=1.25 bounds the known relative error for these test functions. This check is not a universal error-bound proof.

The assignment's unresolved_difference maps to native unresolved (with a specific reason); invalid_grid_ratio maps to invalid_input / invalid_ratio_or_safety_factor. Coverage here means all seven FitStatus variants, not exhaustive coverage of every error reason or source-code branch. Synthetic level identities are test fixtures. Every result retains asymptotic_validation=not_established.

These checks use no saved design arrays, geometry or FEM solves and establish no physical convergence or design qualification. Bit equality is established for the recorded toolchain/platform, not guaranteed across other libm or compiler versions.

Execution affinity: [14, 15]; nice: 10; build jobs: 2. Existing lane target/ reused.

Reproduce acquisition: `taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/phase8_synthetic_gci_check.py`. Verify saved hashes and repeat native checks: `taskset -c 14,15 python3 experiments/rsv2_07/audit_phase8.py`. The verifier imposes no CPU affinity restriction; pair 8,9 can be substituted.

Raw evidence: `target/rsv2-07/phase8/20261018T090024871641Z`. Per-sequence values, sign patterns, native results, IEEE-754 bits and SHA-256 provenance are in `results/phase8_synthetic_gci_audit.json`.
