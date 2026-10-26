# jarambit-g3: the G3 genome, decoder and W_ERGOLIGHT_v1 evolution

A standalone Cargo workspace (it is **not** a member of the root workspace). It is the fresh G3 cohort for the
ergonomic, hollow-AM redesign of the Jarambit knife, in which the finger-ring hand zone and the double-edged blade
evolve together. It is kept apart from the published v1 pipeline by construction:

- schema `G3.0`, metric `W_ERGOLIGHT_v1` (draft d: owner 2026-10-25 raised `solid_tip_3d` from 0.030 to 0.070), rules `requirements/RS-karambit-v3.toml`;
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

All 27 tests are hermetic and read their reference fixtures from `tests/fixtures/` (`fa_best_overall_finalist.json`,
`f075_reference.json`, `g3hz_parity_v1.json`, `g3hz_parity_v3_draftd.json`, and `sobol_test_vector.json`), and run on
every pull request in `.github/workflows/d1-gates.yml`. Note: if a PR updates dependencies in `crates/crescent` or
`crates/physics`, refresh `crates/g3/Cargo.lock` (`cargo update --workspace --manifest-path crates/g3/Cargo.toml`) so
`--locked` remains satisfied.
`examples/hz_probes.rs` holds the independent review probes (P1 hollow-blade coupling, P2 seam fin, P3 reach,
P4 vent and cavity versus bevel bands, P5 medial thickness steps): `cargo run --release --example hz_probes`.
`g3-evolve` writes both `genomes_g3.csv` and `archive.jsonl` (per-niche record with `design_id`, `niche_id`, `tier`,
`tier_provenance`, `score_100`, `raw_score_100`, `base`, `ergo_multiplier`, `descriptors`, scorer-keyed `inputs`,
`terms`, and `ergo`) after running `CvtMapElitesArchive::verify_integrity` and `load_and_verify_persisted_dir`.

## Verification record (snapshots S7–S9, 2026-10-25)

Tests 27/27 (29/29 at decoder r2); probes P1–P5 pass; 256-evaluation smoke run passes; static reviews of S2 through S8 with no open
blocking findings. Reports: `review_g3/REVIEW.md`, `REREVIEW_S3.md` (S4), `REREVIEW_S5.md`, `REREVIEW_S6.md`,
`REREVIEW_S7.md` and `REREVIEW_S8.md` in `~/jarambit_runs/handzone-ergo-20261024/`.

## Known limitations (read before quoting a W_ERGOLIGHT_v1 number)

- Tier 0 uses analytic surrogates: ring capacities are uncalibrated, and grip length and guard height are read
  from genes (`archive.jsonl` records `tier_provenance` per row). Tier 1 measures the voxel geometry and solves the
  plate; only Tier ≥ 1 numbers are measurements.
- Gate S sharpness is measured on the analytic blade function at fully sharpened stations, not on the voxels.
- Edge geometry (`EDGE-1..5`, owner request 2026-10-26: one ideal sharpening angle for the whole two-sided
  blade). Both edges carry a single primary grind of 17.5–22.5° per side (35–45° inclusive), measured normal to
  the actual serrated edge and carried at full height until it meets the medial thickness, so there is no
  micro-bevel and no shoulder. Both grinds run unbroken to the tip (`s1_in`, `s1_sp` are retired); only a
  5–12 % plunge at the heel stays unsharpened. Apex flat 0.05–0.12 mm. Gate S3 samples the true edge-normal
  section over 0–2 mm, so a steep shoulder behind the apex band would now be measured. Archives made before this
  change (decoder `120ba792`) do not reproduce on it and are not comparable.
- Decoder r2 (v3.0.0):
  - Both edges are serrated (`SERR-1..3`, owner: "double-edged, double-serrated"): 0.8–2.2 mm (inner) and
    0.8–2.0 mm (spine) deep, at least 3 full-depth teeth per edge (the pitch is capped on the span at full
    amplitude). The serrated span starts past the plunge and ends at least 18 mm from the point, so the piercing
    point stays plain. `BladeProfile2D` records the depth and tooth count as built.
  - `hm_twist_sf` (16.2 N·m human-max twist) is the blade-body SF of the 1.82 N·m plate solve scaled linearly; it
    used to read the tip zone through a 4.0 clamp, which capped it at 0.337 for every design. On the edge-pilot
    winner it reads 1.01 against 1.64 from a hex8 voxel FE of the root, so it errs on the safe side.
  - `buckling_lambda` is the lateral-torsional buckling factor of the blade as a cantilever under the 3427 N spine
    load (Timoshenko, station-averaged sqrt(E·I_y·G·J)). It is an analytic surrogate, labelled as such, and no
    longer a zero-credit term.
  - `edge_r_min_mm` and `section_corner_r_min_mm` are labelled `design_law` in the tier provenance: they come
    from genes and the exact section law, not from the voxel mesh.
  - The Tier-2 solid-tip TET10 uses a 0.15 mm² planform mesh (`TIP_FE_MAX_AREA_MM2`). On sharp points 0.4 mm² was
    3–19 % away from 0.15 mm², while 0.15 and 0.08 mm² agree within 0.7–8.2 %.
  - `g3-evolve --log-every N` writes `history.jsonl`: best-so-far, archive size, QD score and Tier-2 count every
    N evaluations, with the best genome row at power-of-two counts and at the end. Logging draws no random
    numbers, so logged and unlogged runs give identical archives (tested).
- Rust-versus-Python parity: the scorer matches the Python of record exactly on 25 cases. On the production path,
  the hand-zone AM mass is asserted on the 5 finalists and 4 representative Sobol designs (9 cases, ≤ 8 %);
  centre of mass, spin and guard height are asserted on the parity path (`build_g3_sdf_from_params`).
- Fenestration windows start at `max(x_s0, 92 + ½·max(pitch, 6))` mm with phase shifted by the window half-extent
  (`S8-4`) so the first window is whole and never clipped at the bound.
- All results are computational; nothing here is qualified or has been built.
