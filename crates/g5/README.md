# jarambit-g5: the G5 generation (in development, not frozen)

`jarambit-g5` is the G5 genome, decoder, evaluator and search. G5 targets the requirement set
[`requirements/RS-karambit-v5.toml`](../../requirements/RS-karambit-v5.toml) (sources:
[`PROVENANCE-v5.md`](../../requirements/PROVENANCE-v5.md)) and the objective `W_ERGOLIGHT_v2`. It is planned for
release v5.0.0.

## Status

**This crate has no results.** It starts as a copy of the validated G3 crate ([`crates/g3`](../g3/README.md), the
v3.0.0 cohort of record), renamed to the G5 namespace. Reviewed pull requests then change it to RS-karambit-v5.
Until that work is complete:

- the decoder, the physics and the objective are still the G3 ones;
- `G5_DECODER_FROZEN` is `false`, so `g5-evolve` accepts only development run roots (`evo_g5_dev`);
- no run of this crate can become a cohort of record, and no number from it may appear in the README or a paper.

`crates/g4` is the withdrawn v4.0.0 prototype (see its `WITHDRAWN.md`). G5 does not use it.

## What G5 will change

The planned changes, in order (`docs/RELEASES.md`, Unreleased):

1. **Measurement fixes from the G3 audit.** Done (first four items):
   - partial-volume mass (`g5_sdf::solid_fraction`), with the 2 % grid agreement check
     (`g5_qd::mass_grid_agreement`);
   - blur radii in millimetres;
   - sealed voids measured. Powder drains only through passages at least `[am] drain_channel_min_mm` (1 mm) wide.
     Undrainable cavities stay solid in the mass, and their volume is recorded (`AmConnectivityStats`);
   - the tip thickness raster sampled at cell centres.

   Still to do:
   - the yield sequence per load case;
   - the fatigue knock-down applied once;
   - Lloyd CVT niche centres;
   - promotion by rank;
   - sparse mutation.
2. **Material.** Done, in `material.rs`:
   - stress-relieved L-PBF Ti-6Al-4V at 825/895 MPa, with the 80 °C derate (×0.91 on yield, ×0.92 on ultimate,
     ×0.97 on E and G);
   - ν 0.31;
   - the thin-wall law per pair of wall faces, on the nominal section;
   - fatigue by surface class with FKM mean stress.

   Every evaluator takes E, ν, G and its allowables from this module. Still to do:
   - the sheet-TPMS laws (with the decoder);
   - the solid-tip TET10 still uses the physics crate's hard-coded G3 material until the load-case PR copies it.
3. **Decoder G5.0:**
   - the 20–25° edge on the finished blade, with a finishing allowance;
   - 4–6 mm serrations with gullet radius ≥ 0.5 mm;
   - the sharp-point planform;
   - the 84–100 mm grip, the 15 mm guard and the hand-edge radii;
   - ports ≥ 3 mm, two per cavity;
   - a declared build orientation.
4. **Load cases:**
   - lateral 12/18 N·m and twist 16.2 N·m, reacted by a hand model;
   - the tip set (150 N, 10 N, 0.3 N·m);
   - grip 800 N, guard 300 N, pull 600 N;
   - the ring at 300 N in every direction plus the owner's 300 N / 30 N at SF 2;
   - the teeth;
   - buckling under the tip axial load.
5. **`W_ERGOLIGHT_v2`**, continuous and with no hard gates:
   - safety-factor credit `0.94·v⁴` below the requirement, plus a worst-margin term;
   - the blunt-point penalty;
   - mass weight 0.30;
   - E⁵, so that 1 % of E costs at least 10 g.
6. **V&V, then the decoder freeze.** Followed by cohorts started only from Sobol samples (no hand-set genomes), with
   at least 5 seeds plus baselines.

## Guards that already apply

| Test | What it fails on |
|---|---|
| `requirements::tests` | An unknown, missing or mistyped key in RS-karambit-v5, `hard_gates = true`, weights or betas that do not sum to 1, or the owner rule on E broken |
| `g5_namespace::requirement_identity` | The namespace metric or requirement ID drifting from the file |
| `g5_namespace::tests` | v1 (`ec`) or G3 (`hz`) design IDs, v1 metric or roots, `/tmp` roots, or a record root before the freeze |
| `record_guards::g3_decoder_of_record_is_unchanged` | Any change to the G3 decoder of record, `a203278d…` in `evidence/g3/v3.0.0/release.json` |
| `record_guards::cached_g5_decoder_hash_equals_a_fresh_recompute` | The cached decoder hash differing from a fresh recompute |
| `record_guards::partial_volume_mass_agrees_between_h_and_half_h` | Partial-volume mass of a Sobol design moving by more than 2 % between h 0.4 and h 0.2 |
| `g5_sdf::tests::drainage_needs_a_passage_at_least_drain_channel_min_wide` | A 0.4 mm passage draining a cavity, a 1.5 mm passage failing to, or an exterior slot being filled |
| `v1_identity_tests` | Any change to the v1 certification identity |

The G5 decoder hash (`g5_qd::g5_decoder_sha256`) covers:
- the G5 genome, decoder, evaluator and objective sources;
- `requirements.rs` and RS-karambit-v5;
- the physics and crescent sources it calls;
- this crate's `Cargo.lock`.

It is computed once per process.

## Build and test

```bash
cargo test --release --locked --manifest-path crates/g5/Cargo.toml -- --test-threads=2
```

G5 design IDs are `kv<host>i<island>_g<gen>_<sha8>`. G3 IDs start `hz` and v1 IDs start `ec`.
