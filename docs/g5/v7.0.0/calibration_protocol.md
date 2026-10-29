# G5.1 hand-zone calibration protocol (pre-registered)

This protocol is committed before any FE run of the calibration dataset. It sets how decoder G5.1 scores the three
hand-zone load cases of RS-karambit-v5 — grip squeeze, guard push and the serration teeth — and the rules that decide
every value still open. The values are fixed by these rules in the order below. They are recorded in the v7.0.0
receipt with the data they came from. A rule that fails stops the release; it is not relaxed.

Owner decision (2026-10-29): during the search, analytic models recalibrated against hex8 FE, refusing
extrapolation. At the ranking, the shortlist is scored on hex8 FE, and the winner must pass FE.

## 1. The FE of record (the calibration target and the Tier-2 score)

Hex8 voxel FE (`crates/g51/src/fe`) on the design's G5.1 AM voxel field.
- **Geometry:** the v6.0.0 cross-check's submodels (`fe::hand`).
- **Solve:** one wrench per load case.
- **Fails closed:** a solve that does not converge (true relative residual above 1e-7), a loaded face on unsupported
  material, nodal load on fixed nodes, or unsupported material in the read region scores the case 0.

| Case | Model | Solved | Read |
|---|---|---|---|
| Grip squeeze | 28 mm grip slice at each station: 460 N finger pad (2.3 × 800 N / 4) on the -z face, palm z-support on the +z face | all three stations (25, 50, 75 % of the grip); SF = minimum | within 8 mm of the station, off the supports |
| Guard push | guard lobe and the grip above it, clamped at depth `c_g` inside the grip; 300 N +x on the lobe's rear faces at 60 % of its height | one solve | between the load band (+2.5 mm) and the clamp (−2 mm) |
| Teeth | local model of each tooth and 4 mm of blade at voxel size `h_t`, from the G5.1 blade field; 180 N snag along the edge, and separately 20 N pry across the blade | the 3 weakest teeth by the notched-root formula on each serrated edge; SF = minimum | the tooth and its root, clear of the crest and 1.5 mm clear of the clamp |

- **Allowables:**
  - as-built skin for the grip;
  - skin, or `t_guard` for a solid guard, as-built, for the guard;
  - ground for the teeth (RS-karambit-v5 and `material`).
- **Read stress:** the von Mises stress exceeded by a fixed volume `V` of the read region. It is not a fraction of the
  region, because a fraction makes a bulkier region read lower.
- **Voxel size:** grip and guard at h = 0.2 mm, the ranking grid.

### 1.1 Rules that fix `c_g`, `V`, `h_t` (M3 study; 5 designs)

- **Study designs:** the v6.0.0 winner, plus four designs drawn with seed 20261029 from the v6.0.0 final pool, one per
  quartile of its score.
- **Guard clamp `c_g`:** the smallest of {4, 8, 12} mm whose SF differs from the next deeper clamp's by at most 5 % on
  all five designs. If none qualifies, 12 mm, and the spread is reported.
- **Read volume `V`:** the smallest of {0.05, 0.1, 0.25, 0.5, 1.0} mm³ whose grip and guard readings differ by at most
  5 % between h 0.2 and h 0.15 on all five designs. If none qualifies, 1.0 mm³.
- **Teeth `h_t`:** the coarsest of {0.1, 0.075, 0.05} mm whose tooth SF differs from the next finer one's by at most
  5 % on all five designs. If none qualifies, 0.05 mm.
- **Reported, not decided here:**
  - the h 0.3 / 0.2 / 0.15 values;
  - the teeth on the h 0.2 design grid;
  - the FE noise `s_FE`: the standard deviation of log SF over sub-voxel origin shifts {0, h/4, h/2}
    (`build_g5_sdf_from_params_with_origin_shift`).
- **Sub-hash:** once `c_g`, `V` and `h_t` are fixed, the FE definition and the sha256 of `sdf.rs`, `hand.rs` and
  `fe/*.rs` form the **FE sub-hash**. Every calibration row carries it, and the receipt checks it.

## 2. The physics models (Tiers 0 and 1)

These are improved analytic models `SF_phys`, computed from the genome alone.
- **Grip squeeze:** the skin as a plate strip on an elastic foundation (Hetényi). The foundation stiffness comes from
  the TPMS core (`material::core_relative_stiffness(rho_grip, lat_cell, lat_mix)`) and the mid-plane rib, under the
  finger pad, at each station. G5.0's model reads neither `rho_grip` nor `lat_mix`.
- **Guard push:** G5.0's root-section cantilever with transverse shear, a Peterson root-fillet `Kt`, and the root
  taken at the grip underside.
- **Teeth:** G5.0's notched-root model, snag and pry separately.
- **Acceptance (M4, about 40 v6.0.0 elites):** for each case, Spearman correlation of `SF_phys` against FE ≥ 0.8, and
  a smaller spread of log(FE/SF_phys) than of log(FE/SF_G5.0). One revision is allowed, and recorded.

## 3. Calibration dataset (M5)

- **Size and sources:** 600 designs.
  - 200 are drawn from the v6.0.0 final pool (seed 20261029).
  - 400 are Sobol points over the hand-zone genes, on 20 backgrounds drawn from the v6.0.0 elites, one per score
    vigintile. The genes are `skin_t`, `rho_grip`, `lat_mix`, `lat_cell`, `lat_aniso_x`, `lat_grade`, `rho_grad_wall`,
    `shear_web_amp`, `rib_w`, `t_guard`, `h_g`, `rho_guard` and the serration genes, each over its full gene range.
- **What each design gets:** every design gets the guard-push and teeth FE (seconds per design). The grip-squeeze FE
  (about 14 core-minutes per station; M2 measured 208 s at 4 threads on the v6.0.0 winner) runs on a subset of hollow
  grips: the first 250 in dataset order whose `SF_phys` puts at least one station in the band. Of the 600 designs, 250
  have grip FE.
- **Pre-screen:** `SF_phys` oversamples the band that matters. Each case keeps at least 40 % of its units with
  `SF_phys`/1.2 in [0.9, 1.5].
- **Rows:** ids `kd…`. These rows never enter a search pool. Each row carries its FE solves, its definition and the FE
  sub-hash.
- **Repeat check:** 5 rows are re-run on a second host and must be bit-identical.

## 4. Fit, offset, domain and gate (M6)

- **Strata:**
  - grip, per station, hollow grips only (a solid grip has no squeeze case);
  - guard, hollow and solid;
  - teeth, snag and pry, inner and spine edge.
- **Splits:** by design, with seed 20261029. Fit 30 %, offset 30 %, test 40 %.
- **Fit:** per stratum, log(SF_FE / SF_phys) = a. Up to 2 sign-constrained slopes on these dimensionless features
  enter only if the fit split shows a trend with |t| > 3:
  - skin_t / lat_cell;
  - core relative stiffness;
  - h / t of the section;
  - depth / t_guard;
  - tooth height / gullet radius.
- **Conformal offset:** q is the ⌊α(n+1)⌋-th smallest residual of the offset split, with α = 0.05.
  SF_cal = SF_phys · exp(fit + q).
- **Domain:** the ranges of the derived features over the fit and offset splits, plus a standardised k-nearest-neighbour
  distance (k = 5) at most the 99th percentile of the offset split's.
  - Outside the domain, SF = SF_phys · min(1, r_lo), where r_lo is the lowest FE / SF_phys of the fit and offset splits
    in that stratum. No trend is extrapolated.
  - The out-of-domain fraction of every island is logged.
- **Gate, on the untouched test split.** Coverage is P(SF_cal ≤ SF_FE). It must be at least 95 % overall, and at least
  95 % within SF_cal/1.2 in [0.9, 1.5], with at least 100 test units there, for each of the three cases. Otherwise the
  calibration is not frozen.
- **Pilot loop, at most 2 iterations:**
  1. Run a 2-hour search with provisional coefficients (under `evo_g51_dev`).
  2. Run FE on the pilot pool's top 40 elites.
  3. If their coverage is below 95 %, add them to the fit and offset splits, draw a fresh test split from unused
     dataset rows, refit, and gate again.

  After two failed iterations the release stops and the owner is told.
- **Freeze:** the coefficients, offsets, domains and the training feature matrix go into
  `crates/g51/calibration/hand_zone.json`, which is in the G5.1 decoder hash. The hash is recorded before epoch 0.

## 5. After the search

- **Ranking:** Tier 2 scores the top-64 shortlist on the FE of record.
- **Grid check:** it extends to hand-zone FE v ≥ 1 at h 0.15.
- **Reported:** the calibrated model's coverage on the shortlist.
- **The receipt:**
  - refits the calibration from the committed dataset bit for bit;
  - checks the FE sub-hash of every row;
  - recomputes a seeded sample of the FE rows.

## 6. Compute

Every claim goes in `~/master_log.md`, at host CPU ≤ 85 %, with the memory, PSI and disk floors of the fleet rules.
- xbabe4: the colima VM `jarambit-evo-x64`.
- xbabe2: cores 0–3.
- xbabe3: once its disk is back above 20 GB.
- One island or FE job per physical core, with `RAYON_NUM_THREADS=1` and FE threads 1, recorded in each launch
  record.
