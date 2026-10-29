# jarambit-g51: decoder G5.1 (in development for v7.0.0; not frozen, no results)

`jarambit-g51` is the decoder generation of the coming v7.0.0, approved by the owner on 2026-10-29. It makes the changes
the v6.0.0 paper left "to a later decoder generation"
([`docs/g5/paper/sections/09-limits.tex`](../../docs/g5/paper/sections/09-limits.tex)):

1. **Surface texture kept off the ground bevels by construction.**
2. **Grip squeeze, guard push and the serration teeth scored on the solid.** The search uses analytic models
   recalibrated against hex8 FE, refusing extrapolation. The Tier-2 ranking scores the shortlist on the FE, and the
   winner must pass it ([`docs/g5/v7.0.0/calibration_protocol.md`](../../docs/g5/v7.0.0/calibration_protocol.md),
   pre-registered).

RS-karambit-v5, W_ERGOLIGHT_v2 and the genome (schema `G5.0`, 134 genes) are G5.0's. So v6.0.0 genomes load
unchanged and keep their `kv…` ids; designs born under G5.1 get `kw…` ids.

## How it relates to G5.0

[`crates/g5`](../g5/README.md) is a path dependency and is not changed, so the G5.0 decoder of record stays
verifiable on main: its guard test passes, `g5-release receipt-v6` runs, and `tests::guards` here re-checks the
G5.0 hash against the v5.0.0 and v6.0.0 receipts. This crate holds only what changes and calls G5.0 for everything
else.

The **G5.1 decoder hash** is `G5.1` + the G5.0 decoder hash + every source below + this crate's `Cargo.lock`, which is
G5.0's lock plus this crate's entry, so no dependency version moves. `lib.rs`, `src/tests/` and `src/bin/` are
outside the hash, as in G5.0. A test fails if any other source is left out.

| Module | What it is |
|---|---|
| `sdf` | G5.0's production SDF builder, copied verbatim except step 6. Texture fades to zero across each ground bevel plus the 0.5 mm finishing allowance, over a 1 mm taper, along the edge before the grind start as well. `blade_outer_field_at` gives the blade field at any point. |
| `tiers` | G5.0's Tier 0, plus its Tier 1/2 and rank promotion copied onto the G5.1 grid, with the hand-zone block. A G5.1 archive is G5.0's CVT archive keyed to the G5.1 hash. |
| `hand` | The hand-zone block, and a census of every serration tooth with its snag and pry SFs. Until the calibration lands it scores G5.0's analytic models, for development only. |
| `fe` | Hex8 voxel FE. It is the voxel FE G5.0 vendored, solving the one wrench of a load case (G5.0 superposed six). It fails closed on loaded faces without support, load on fixed nodes, and unsupported material in the read region. It holds the grip-squeeze, guard-push and teeth submodels. |
| `namespace`, `hash`, `f64x` | Ids and run roots (`evo_g51_dev` until frozen), the decoder hash, and exact JSON for non-finite SFs. |

## Tools

- `g51-fe v6check <repo>` reproduces the v6.0.0 hex8 cross-check on the v6.0.0 winner: grip squeeze 23.78, guard push
  8.854, teeth 2.724, within 1e-11.
- `g51-fe study`, `study-designs`, `rules` run the FE study of the protocol (§1.1).
- `g51-fe run` evaluates one design's three cases.

## Tests

```sh
cargo test --release --locked --manifest-path crates/g51/Cargo.toml
```

The tests cover:
- the G5.0 decoder unchanged;
- every decoder source hashed;
- the bevel law and the G5.1 field equal to G5.0's off the bevels, and untextured on them, with no step at the plunge
  line;
- Tiers 0, 1 and 2 equal to G5.0's with no blade texture;
- the voxel FE against tension, beam bending and Saint-Venant torsion;
- every fail-closed gate;
- the tooth census against G5.0's tooth strength.

CI runs them in `d1-gates`.
