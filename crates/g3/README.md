# jarambit-g3: the G3 genome, decoder and W_ERGOLIGHT_v1 evolution

A standalone Cargo workspace (it is **not** a member of the root workspace). It is the fresh G3 cohort for the
ergonomic, hollow-AM redesign of the Jarambit knife, in which the finger-ring hand zone and the double-edged blade
evolve together. It is kept apart from the published v1 pipeline by construction:

- schema `G3.0`, metric `W_ERGOLIGHT_v1` (draft c), rules `requirements/RS-karambit-v3.toml`;
- design IDs `hz<host>i<island>_g<niche>_<sha8>`, run root under `evo_g3` (never `evo_continuous`);
- the v1 certification identity (root `Cargo.lock` and pinned v1 certificates) is asserted unchanged by
  `v1_identity_tests::v1_certification_identity_is_unchanged_from_origin_main`.

## What is in the crate

| Module | Role |
|---|---|
| `g3_genome.rs` | 179-gene unit-cube genome (`G3.0`), strict CSV/serde loader, Sobol start, iso-line DD variation |
| `sobol_joe_kuo.rs` | Joe–Kuo `new-joe-kuo-6.21201` direction numbers for 179 dimensions, LMS scrambling + digital shift |
| `g3_sdf.rs` | Voxel signed-distance decoder: ring, grip with finger grooves, guard, blade with both edges, hollow TPMS interior and ports |
| `g3_sandwich.rs` | Sandwich-plate FE, ring band T6 FE, fail-closed linear solves (relative residual or Oettli–Prager backward error at the f64 floor) |
| `ergolight.rs` | `W_ERGOLIGHT_v1`: continuous multi-objective score with ergonomics as a multiplier; no hard gates |
| `g3_qd.rs` | Three-tier evaluation (analytic, voxel + plate, solid tip) and CVT-MAP-Elites |
| `bin/g3_evolve.rs` | `g3-evolve --out <path containing evo_g3> --init N --evals N --seed S --host H --island I --commit SHA` |

## Build and test

```sh
cd crates/g3
CARGO_INCREMENTAL=0 cargo test --release          # 27 tests
cargo build --release --bin g3-evolve
```

Some parity tests read Python reference fixtures from `~/jarambit_runs/handzone-ergo-20261024/` (the hand-zone
prototype run directory on xbabe2). Where those files are absent the tests skip, except
`run6_finalist_fa_g11285_rust_vs_python_parity`, which needs them; run the full suite on xbabe2.
`examples/hz_probes.rs` holds the independent review probes (P1 hollow-blade coupling, P2 seam fin, P3 reach,
P4 vent and cavity versus bevel bands, P5 medial thickness steps): `cargo run --release --example hz_probes`.

## Verification record (snapshot S7, 2026-10-25)

Tests 27/27; probes P1–P5 pass; 256-evaluation smoke run passes; static reviews of S2 through S7 with no open
blocking findings. Reports: `review_g3/REVIEW.md`, `REREVIEW_S3.md` (S4), `REREVIEW_S5.md`, `REREVIEW_S6.md` and
`REREVIEW_S7.md` in the run directory above.

## Known limitations (read before quoting a W_ERGOLIGHT_v1 number)

- Tier 0 uses analytic surrogates: ring capacities are uncalibrated, and grip length and guard height are read
  from genes. Tier 1 measures the voxel geometry and solves the plate; only Tier ≥ 1 numbers are measurements.
- Gate S sharpness is measured on the analytic blade function at fully sharpened stations, not on the voxels.
- Rust-versus-Python parity: the scorer matches the Python of record exactly on 25 cases. On the production path,
  only the hand-zone AM mass is asserted (5 cases, 8 %); centre of mass, spin and guard height are not.
- Fenestration windows start at x ≥ 92 + ½·max(pitch, 6) mm, which removes windows from some short blades.
- The GPU solver shell-out is unused; Tier 2 uses faer. It is slated for removal.
- All results are computational; nothing here is qualified or has been built.
