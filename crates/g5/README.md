# jarambit-g5: the G5 generation (decoder G5.0, frozen for v5.0.0)

`jarambit-g5` is the G5 genome, decoder, evaluator and search. G5 targets the requirement set
[`requirements/RS-karambit-v5.toml`](../../requirements/RS-karambit-v5.toml) (sources:
[`PROVENANCE-v5.md`](../../requirements/PROVENANCE-v5.md)) and the objective `W_ERGOLIGHT_v2`. It is the generation of
release v5.0.0.

## Status

**The decoder is frozen (crate version 5.0.0).**
- `G5_DECODER_FROZEN` is `true`, so record cohorts run under an `evo_g5` root. The development pilots ran under
  `evo_g5_dev` and none of their numbers are records.
- The freeze V&V receipts are in [`evidence/g5/v5.0.0/vv/`](../../evidence/g5/v5.0.0/vv/), written by `g5-vv`.
- The cohort of record and its winner arrive with the v5.0.0 release receipt.

G5 started as a copy of the validated G3 crate ([`crates/g3`](../g3/README.md), the v3.0.0 cohort of record).
Reviewed pull requests then changed it to RS-karambit-v5. `crates/g4` is the withdrawn v4.0.0 prototype (see its
`WITHDRAWN.md`); G5 does not use it.

## What G5 changed from G3

In order (`docs/RELEASES.md`):

1. **Measurement fixes from the G3 audit.** Done (first four items):
   - partial-volume mass (`g5_sdf::solid_fraction`), with the 2 % grid agreement check
     (`g5_qd::mass_grid_agreement`);
   - blur radii in millimetres;
   - sealed voids measured. Powder drains only through passages at least `[am] drain_channel_min_mm` (1 mm) wide.
     Undrainable cavities stay solid in the mass, and their volume is recorded (`AmConnectivityStats`);
   - the tip thickness raster sampled at cell centres.

   Later PRs fixed the remaining audit items:
   - the yield sequence per load case (`loads.rs`);
   - the fatigue knock-down applied once (`material.rs`);
   - Lloyd CVT niche centres, promotion by rank and sparse mutation (search, `g5_qd.rs` and `g5_genome.rs`).
2. **Material.** Done, in `material.rs`:
   - stress-relieved L-PBF Ti-6Al-4V at 825/895 MPa, with the 80 °C derate (×0.91 on yield, ×0.92 on ultimate,
     ×0.97 on E and G);
   - ν 0.31;
   - the thin-wall law per pair of wall faces, on the nominal section;
   - fatigue by surface class with FKM mean stress.

   Every evaluator takes E, ν, G and its allowables from this module. Still to do:
   - the sheet-TPMS laws (with the decoder);
   - the solid-tip TET10 still uses the physics crate's hard-coded G3 material until the load-case PR copies it.
3. **Decoder G5.0.** Done: 134 genes, down from G3's 179.
   - **Removed:**
     - the 37 texture genes (Turing, Voronoi and a 32-weight CPPN); the knurl and ridge that remain are zero-mean;
     - the retired `s1_*` edge-stop genes;
     - the cusped tooth and micro serration genes;
     - `exo_open`;
     - the five fenestration-window genes (see load cases).
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
4. **Load cases.** Done for Tiers 0 and 1, in `loads.rs`; recorded on every evaluation as `G5EvalRecord::loads`.
   - **Blade sections:** integrated from the decoded thickness law along the smoothed midline. Each force is resolved
     into axial force, bending through the thickness, in-plane bending and torsion, so a claw's offset point loads
     its body in torsion.
   - **Cases:**
     - lateral 12 N·m (SF 1.2) and 18 N·m ultimate (SF 1.3), applied 5 mm behind the point;
     - body twist 16.2 N·m over the working span;
     - tip axial 150 N, tip side 10 N and tip twist 0.3 N·m;
     - pull 600 N with the last 20 mm held;
     - the teeth (180 N snag; 60 N pry shared by three teeth; end teeth included);
     - grip squeeze 800 N, guard push 300 N;
     - the ring 300 N / 30 N at SF 2, plus 300 N in 12 directions;
     - fatigue by surface class (a hollow blade's cavity faces are as-built);
     - the blade-before-hand sequence per load case;
     - buckling of the point under 150 N, as a Rayleigh-Ritz column (checked against Euler).
   - **Tier-1 plate:** the whole blade at 0.3 mm², with smooth loads: a 2 mm patch, a couple on the blade body, a
     held 20 mm region. Lateral and twist agree within 15 % between 0.3 and 0.15 mm²; the single-element peak in the
     thin bevels scatters about ±6 %. G3's plate did not converge.
   - **No fenestration windows:** G5.0 retired them (5 genes, 134 genes in all). Their lens-shaped slot ends raised
     the plate stress beyond what the search mesh resolves: a windowed design's lateral SF fell from 0.64 to 0.38
     between 0.3 and 0.08 mm².

   - **Tier-2 solid tip** (`tip_tet10.rs`, a parameterised copy of `physics::solid_tet10`):
     - the RS-v5 axial, side and twist tip cases about the tip bisector, with E and ν from RS-v5;
     - stresses read at the RS-v5 stations;
     - a residual check that keeps 1e-10, accepting at the float64 floor only with a componentwise backward error
       ≤ 1e-12, plus two steps of iterative refinement.
     The section model is conservative against it (side 0.57 against 0.79, twist 0.39 against 0.80 on one design).
     Twist is converged at 0.15 mm²; axial and side change 13–19 % to 0.08 mm², the certification mesh.
5. **`W_ERGOLIGHT_v2`.** Done, in `objective.rs`; scores every tier.
   - **Structural terms:** one per load case, v = SF/SF_req, scored 0.94·v⁴ below 1 with a knee above.
   - **Worst margin:** a multiplier W = min(1, s(μ)/0.94) on the whole score, so comfort and mass cannot buy back a
     failed load case (pilot finding, 2026-10-27).
   - **Mass:** linear, weight 0.30.
   - **Geometry terms:**
     - edge angle and apex;
     - the point: the owner's blunt-point penalty plus the tip thickness caps;
     - serrations;
     - AM sealed volume (unmeasured at Tier 0, so no credit);
     - form.
   - **Ergonomics:** E⁵ over the RS-v5 betas, with a first-order contact-pressure model.
   - **Tests:**
     - a single failing SF raised to its requirement buys more than 10 g of mass credit;
     - 1 % of E costs ≥ 10 g;
     - a record re-scores bit-for-bit from its measurements.
   - **Decoder change:** the point wedge now sets the point directly (sharper or stouter), and `tip_reserve` (4–12 mm)
     and `t_tip` (1.5–5 mm) allow the stout, quickly thickening point the tip loads need.
6. **Search.** Done:
   - **CVT centres:** the 1024 niche centres are a Lloyd CVT (20 iterations over 16 Sobol samples per centre,
     computed once per process), replacing G3's Halton points.
   - **Promotion** (`RankPromotion`): the top `--tier1-top` (default 0.4) of an island's last 256 Tier-0 scores go
     to Tier 1, replacing G3's fixed 62/71 gates, which left 8 of 24 G3 islands without a Tier-2 evaluation. Tier 2
     stays off (`--tier2-top 0`) until the solid tip carries the RS-v5 tip loads.
   - **Mutation:** each gene moves with probability 3/n_module, about 15 genes per child where G3 moved all 179.
7. **V&V, then the decoder freeze.** Done. Three development pilots on xbabe4 (12 islands × 600 / 1,500 / 600
   evaluations) found the objective and decoder issues fixed above. `g5-vv` writes the freeze receipts:
   - grid agreement;
   - plate and solid-tip mesh studies;
   - decoder construction targets;
   - sheet-TPMS density;
   - Ritz/Euler and Lloyd/Halton checks;
   - the objective properties.

   Followed by cohorts started only from Sobol samples (no hand-set genomes), with
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
