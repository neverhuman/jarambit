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
3. **Decoder G5.0.** Done: 139 genes, down from G3's 179.
   - **Removed:**
     - the 37 texture genes (Turing, Voronoi and a 32-weight CPPN); the knurl and ridge that remain are zero-mean;
     - the retired `s1_*` edge-stop genes;
     - the cusped tooth and micro serration genes;
     - `exo_open`.
   - **Added**, in requirement units:
     - `grip_perim` and `grip_hot`: the swell section is scaled to them, and measures within 2 %;
     - `port_d`;
     - `point_angle` and `point_len`: a rounded wedge (tip radius 0.175 mm) bounds the point.
   - **Construction rules:**
     - gene bounds on the RS-v5 bands: edge 20–25°, serration pitch 4–6 mm and depth 0.8–1.5 mm, guard height
       10–18 mm, hand-edge radius ≥ 3 mm;
     - finger pitches clamped to 19–21 mm, grip length ≥ 69 mm;
     - each serration scallop widened until its gullet radius is ≥ 0.5 mm;
     - a sheet-TPMS lattice at least 0.8 mm thick, using the RS-v5 sheet laws;
     - ports clear of the rib, at least two per cavity half, in every hollow zone, cut one cell deep so both TPMS
       labyrinths drain;
     - drain tunnels through the shear webs.
   - **On Sobol designs:**
     - grid agreement is 0.13–0.53 %;
     - most designs drain (150–1,000 mm³ sealed).

   Still to do: the finishing allowance (printed against finished geometry) and the declared build orientation for
   face classes.
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
