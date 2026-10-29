# RELEASES / CHANGELOG — jarambit (author Jepson Taylor)

Policy: semver. Since v1.0.0, a major version is cut for a scope change the owner chooses (v2.0.0 = option (a), 2026-10-18), minor versions for new waves or features, and patch versions for fixes. Each release gets a git tag `vX.Y.Z`, an entry here and a WHITEPAPER header bump. Before v1.0.0, major wave N ⇒ `0.N.0`. Sub-experiments (Wave N.1, N.2 …) ⇒ `docs/WAVE_LOG.md` only (no bump, no tag). `-dev` while a wave is flying; the tag is cut only when that wave's pack (plates + solids) lands.

## v6.1.2 — 2026-10-29 (patch, docs only: G5 leader renders with lattice x-ray, replacing the legacy renders of v6.1.1; cohort of record v6.0.0)

v6.1.1 rendered `ec2i089_r01_p10_c68` from the **legacy v1 pipeline** (solid, no internal lattice) as the "latest-run leader"; that is an older design line, not the current G5 design. v6.1.2 replaces those images with Blender renders of the current G5 pipeline's leader `kv4i1404_g00581_29ea3c73` (newest G5 run `evo_g5_v6`, Tier-2 h 0.2 mm ranking 74.40, 88.6 g) from its AM-build STL, including an x-ray cutaway of the internal 3D-printed lattice voids. `tools/refresh_leader_assets.py` now picks the leader from the newest current-pipeline (G5) run, refuses legacy designs unless `--allow-legacy`, refuses meshes without internal voids, and renders open cuts plus an `xray` view. The legacy addendum remains, relabelled as an older non-lattice line.

## v6.1.1 — 2026-10-29 (patch, docs only: Blender renders of the latest-run leader; cohort of record v6.0.0)

- New manual command `python3 tools/refresh_leader_assets.py [--design ID]` ([docs/LEADER_ASSETS.md](LEADER_ASSETS.md)): resolves the latest run's leader, takes its exported STL, renders 12 Blender views (`ops/render/blender/leader_views.py`) into `docs/assets/leader/`, regenerates the run's evolution plots, the README (hero strip + leader gallery) and the G5 paper PDF, and prints before/after hashes.
- Run for `ec2i089_r01_p10_c68` (fitness 80.851214, 83.409 g, 13 of 26 legacy gates; not a qualified design). Its binary STL is committed in `evidence/legacy_continuous_run_v6.1.0/leaders/`.
- The G5 record images (`docs/g5/v6.0.0/img/`) stay on the record design: they are hash-bound to the G5 receipt.

## v6.1.0 — 2026-10-29 (minor, docs only: the latest legacy-pipeline continuous evolution run)

- Keeps the **cohort of record v6.0.0** (G5, `kv4i1404_g00581_29ea3c73`) unchanged; README headline and paper of record are unchanged.
- Adds `docs/LATEST_RUN.md` and a labeled section in the README and white paper on the latest continuous island run on xbabe2 (legacy v1 / W_ULTRALIGHT_v1 pipeline, islands i001–i103, stopped 2026-10-29): best `ec2i089_r01_p10_c68`, fitness 80.851214, 83.409 g (3D 83.3333 g), 13 of 26 legacy hard gates met — not qualified for use.
- Evidence committed in `evidence/legacy_continuous_run_v6.1.0/` (manifest, gate JSON, leaders fitness, genome, SHA256SUMS); plots in `docs/assets/legacy_run_v6.1.0/`.

## v6.0.0 — 2026-10-28 (major: the G5 continuation on one platform of record; FEM renderings of every load case; the fitness budget)

- **Cohort of record v6.0.0**, with the frozen G5.0 decoder of v5.0.0 unchanged (`c63629d4b310…`, freeze `bf3e37fa`; the crate stays 5.0.0).
  - **Best individual:** **`kv4i1404_g00581_29ea3c73`**, rank 1 of the Tier-2 ranking.
  - **Score and loads:** W_ERGOLIGHT_v2 74.40 (hand multiplier E 0.922, worst-margin multiplier W 1.000); worst margin 1.094 over the plate meshes; 16 of 16 load cases met.
  - **Mass:** 88.4 g.
  - **Against the v5.0.0 winner**, re-evaluated on the same platform: score 59.34 → 74.40; worst margin 1.014 → 1.094; mass 83.3 → 88.4 g; cases met 16 → 16.
  - **Receipt:** `evidence/g5/v6.0.0/release.json`, written by `g5-release receipt-v6`, which recomputes the whole lineage and fails closed.
- **The continuation:**
  - **Warm start:** the 1,508 final v5.0.0 elites were re-evaluated on linux/amd64, giving 131 niches with a best of 63.36.
  - **Rounds:** 247 warm-started islands ran 20 rounds on xbabe3 and xbabe4, 296,400 evaluations in all. Each round was merged by the hub into a pool lineage that the receipt recomputes byte for byte. The best rose 63.36 → 75.54 on the search grid.
  - **Control:** 4 fresh-start islands × 4,800 evaluations, with no pool and no migration. Best 59.56.
- **One platform of record.** Static x86_64 binaries run natively on Linux and in a linux/amd64 container under Rosetta on the Mac.
  - The same island is byte-identical on all four hosts.
  - `epoch01/xbabe4/island101` re-run on xbabe2 reproduces its history bit for bit.
  - The v5.0.0 limit (arm64 and x86 not bit-identical) no longer applies.
- **FEM renderings of every load case** (`g5-fem-fields`): all 16 load cases of `kv4i1404_g00581_29ea3c73` rendered from their own fields; the 28 scored values they re-solve match the receipt bit for bit. The analytic cases get an independent hex8 FE check: grip squeeze score model SF 2.32, hex8 FE 23.78; guard push score model SF 26.39, hex8 FE 8.85; teeth score model SF 2.38, hex8 FE 2.72.
- **The fitness budget** (`g5-release fitness`): where the 100 points of the score go. 74.4 earned (structural 45.3, mass 11.0, cutting geometry 11.0, build and form 7.1), 6.3 lost to the multipliers and 19.3 unmet credit.
  - The ranked bars show each term's points against the most it could earn.
- **Fleet:** the search ran one island per core complex on the Threadripper hosts, following an early pilot. A controlled measurement afterwards (`fleet/sizing.json`) shows that one island per core would have given 2.9× the throughput. The result is deterministic either way; the cost was evaluations, not correctness. The glibc malloc tunables cut page faults 2.5× with identical outputs.
- **GPU spike, not adopted.** The search records must stay exact CPU arithmetic, and Tier 1 is 90 % of island time. A perfect pre-screen costing nothing would give at most 3.1× more evaluations per hour; any real predictor costs time and misses designs, so the 3× bar for adopting one is out of reach. The GPUs render.
- **Corrections to the v5.0.0 paper** (none changes a v5.0.0 result):
  - "E⁵ = 0.222" had no meaning: E is already the fifth power of the hand factors' weighted product. v6 also states the product inside the power.
  - The marginal points per gram used E⁵ again: 0.063 → 0.211.
  - The promotion window is 256 Tier-0 scores (warm-up 16), not 128.
  - Lloyd relaxation is 20 iterations of 16 samples per centre, not 12 of 64.
- **New tools:**
  - `g5-continue` (`run`, `reeval`, `merge`, `screen`);
  - `g5-release receipt-v6` and `fitness`;
  - `g5-fem-fields`;
  - the shared archive module.
- **Generated docs:** the README, `docs/g5/paper/g5-v6.0.0.pdf` and the `cad/` kit, all from the receipt.

## v5.0.0 — 2026-10-27 (major: the G5 generation under RS-karambit-v5; README, paper and `cad/` generated from one receipt)

- **Cohort of record v5.0.0.** The best individual is **`kv4i012_g00009_dd6ffea5`**, rank 1 of 32 at Tier 2
  (h 0.2 mm, solid-tip TET10 at 0.08 mm²).
  - **Score:** W_ERGOLIGHT_v2 59.34, with hand multiplier E 0.740 and worst-margin multiplier W 1.000.
  - **Mass:** 83.3 g of record (partial volume at h 0.2 mm); the watertight print mesh is 84.5 g.
  - **Load cases:** it meets every RS-karambit-v5 load case on every plate mesh (0.6 / 0.3 / 0.15 mm²). The lowest margins of record (minimum over the meshes) are fatigue 1.005 and tip axial 1.070.
  - **Receipt:** `evidence/g5/v5.0.0/release.json`, written by `g5-release`, which checks every link and fails closed.
- **The cohort.** 16 islands × 3,000 evaluations on xbabe4 (decoder `c63629d4b310…`,
  freeze label `bf3e37fa`), started only from scrambled Sobol samples: no seeded or hand-made individuals.
  - **Control:** 2 random-search islands with the same budget, decoder, objective and promotion.
  - **Result:** all 16 of 16 evolving islands beat the best random-search island
    (42.9). The island median rose from 16.2
    after the Sobol start to 54.0.
  - **Origin of the winner:** evolution found it in island12 at 2,425 evaluations.
- **Certification**, by a protocol posted before any result:
  - every final elite re-evaluated at Tier 1, which reproduces each Tier-1 archive row bit for bit (706 rows);
  - the top 32 at Tier 2;
  - a five-grid check of the top 8;
  - a launch re-run of island01 on the platform of record, bit for bit;
  - the top 8 again on Linux x86_64.
  - the plate cases of the finalists, in rank order, re-solved on 0.6 / 0.3 / 0.15 mm² on the platform of record (the 0.3 mm² rows reproduce the ranking bit for bit); the winner is the first design meeting every load case on every mesh (release check B1).
- **Findings recorded in the receipt:**
  - **Search leader.** `kv4i012_g00204_19a0b6f0` (63.4 on the search grid) falls to rank 23 at Tier 2. Its
    solid tip carries the axial case at a margin of 0.96; the section model of the search grid had estimated
    1.15.
  - **Not bit-identical across architectures.** The platform maths libraries differ. Top-8 scores agree within
    1.24 %, and the winner ranks 1 on Linux x86_64 as on arm64.
  - **Blade texture (decoder limit).** The winner's genome scopes the evolved surface texture onto the blade (depth 0.20 mm, blade scope 0.99).
    - The frozen G5.0 decoder carves it into the flats and the bevels, so the as-printed edge is notched.
    - The edge measurements use the exact bevel law, which grinding restores (finishing allowance 0.5 mm per face).
    - Keeping texture off the bevels is left to a later generation.
- **What changed from G3** (details in PRs #179–#189, `crates/g5/README.md` and `requirements/PROVENANCE-v5.md`):
  - **Requirements** — `RS-karambit-v5`, sourced: each value cites a standard, handbook or paper, or is a dated owner decision.
    Every requirement is a decoder construction rule or a continuous objective term.
  - **Measurement:**
    - partial-volume mass, with a 2 % h/h-over-2 rule;
    - blur radii in millimetres;
    - a cell-centred tip raster;
    - powder drainage through passages ≥ 1 mm, with sealed pockets measured and penalised.
  - **Material** — RS-v5 at 80 °C, a thin-wall law on the nominal section, and surface-class fatigue
    with FKM mean stress.
  - **Decoder G5.0**, 134 genes:
    - a grip built to its perimeter and H/T genes, with a guard hung from it;
    - a point-angle module;
    - raised-cosine serrations with a gullet radius ≥ 0.5 mm;
    - a sheet-TPMS core with ports clear of the rib;
    - windows and texture CPPN retired.
  - **Loads** — every RS-v5 case: integrated sections (Tier 0), a converging whole-blade plate (Tier 1), a TET10 solid tip (Tier 2),
    a T6 ring sweep and tip buckling.
  - **Objective `W_ERGOLIGHT_v2`** — continuous, with no gates:
    - credit 0.94·v^4 below each requirement;
    - the worst margin multiplies the score;
    - mass weight 0.3;
    - E^5.
  - **Search** — Lloyd CVT niches, rank promotion and sparse mutation.
  - **V&V** — `evidence/g5/v5.0.0/vv/`, written by `g5-vv`.
- **Development pilots** (`evo_g5_dev`, not records) found the fixes made before the freeze:
  - the worst margin as a multiplier;
  - the guard hung from the scaled grip;
  - crevices separated from sealed cavities;
  - the windows retired.
- **Release checks and records:**
  - the receipt keeps the G3-era evaluator fields that W_ERGOLIGHT_v2 does not read apart, as `legacy_unscored` (B2);
  - a test recomputes the G5 decoder hash and asserts it equals the receipt's (`g5_decoder_of_record_is_unchanged`);
  - `requirements/PROVENANCE-v5.md` lists the evaluator's model constants (meshes, patches, read-out windows, ring
    sweep, TPMS laws, search settings) with their basis.
- **History kept.** G3 v3.0.0 (`evidence/g3/`, `docs/g3/`, `crates/g3/`) and the v1/v2 study stay as their own record.
  The README now shows only the G5 winner and its evolution. The G3 paper stays at `docs/g3/paper/g3-v4.0.1.pdf`.

## v4.0.1 — 2026-10-27 (patch: v4.0.0 withdrawn; the G3 v3.0.0 cohort of record is current again; G3 errata and diagnostics)

- **v4.0.0 is withdrawn (owner decision, 2026-10-27; tag and GitHub release kept, marked WITHDRAWN).** The cohort of
  record v3.0.0 and its best individual `hz4i013_g00995_d206e3c9` are current again in the README, `cad/` and STATUS.
  v4.0.0 is withdrawn for three reasons:
  - **Not evolved.** Its winner is not an evolved design. Every initial population, including the "Sobol" baseline,
    was seeded with the hand-authored default genome of `genome/G4.0.toml`. A hand-coded rule set pushes named genes
    toward chosen values. The winner lies within a median of 0.031 (normalised units) of those defaults, after 254
    evaluations per run (neverhumanbot's post-merge review of #176).
  - **Not under the owner's loads.** The tip twist was 0.80 N·m where RS-karambit-v3 has 1.82 N·m. The 3,427 N spine
    case was judged on ultimate strength, not first yield. HIP material was assumed, which is an open owner decision.
    The search used hard gates (feasibility-first) and a Lamarckian repair emitter. Both conflict with the owner's v4
    decisions of 2026-10-27: a continuous objective with no gates, and every RS-karambit-v3 load value kept.
  - **Not reviewed.** #176 was merged 60 s after it was opened, before CI finished.
  - **Further findings of an independent code review**, each checked against the code and the receipt:
    - the lateral capacity is a clamp (stress per newton clamped to [0.68, 0.96] MPa/N, so 862.5 N = 828 / 0.96);
    - the plate convergence study is synthesized from the fine-mesh value (order forced to 2.000, GCI to 0.375 %);
    - at the RS-v3 tip twist of 1.82 N·m the winner's TET10 safety factor is 0.161;
    - the exported CAD kit is built through the G3 bridge and scaled in z to the scored mass, so it is not the evaluated
      design.
- **What stays.** The G4 prototype (`crates/g4`, `genome/G4.0.toml`, `requirements/*-v4.toml` and
  `requirements/PROVENANCE.md`, `evidence/g4/`, `docs/g4/`) stays in the tree, unchanged, as an experimental
  prototype. Each folder has a `WITHDRAWN.md` notice. Its CAD kit left `cad/` and remains in git history. The genuine
  G4 will be v4.1.0 under the owner's rules.
- **G3 errata and diagnostics** (prepared as v3.0.3, which was never released; the receipt is unchanged):
  - **Diagnostics.** `crates/g3/src/bin/g3_diagnostics.rs`, written by AGY and re-run byte for byte before adoption,
    writes `evidence/g3/v3.0.0/diagnostics/` with its own `SHA256SUMS`:
    - yield sequence per load case: 2.74 lateral, 5.06 twist, 0.52 spine (the coded 0.56 mixes two cases);
    - fatigue with the thin-wall knock-down applied once: 1.06, not 0.94;
    - solid tip with the half-cell raster offset removed: SF 0.55, not 0.64, and a blunt planform point;
    - plate mesh refinement: 337 / 191 / 254 / 103 N, which does not converge, so the plate's "meets target"
      readings hold on the search mesh only;
    - the four masses: 56.0 search voxels, 57.0 ranking voxels (scored), 59.3 print mesh (of record), 59.7 upload
      mesh. At the print-mesh mass it would score 79.75.
  - **Wording**, in the G3 paper, README, STATUS and `crates/g3/README.md`. Every new number is generated from the
    receipt or the diagnostics by `g3-release-tex` and the generators.
    - "pry" → twist about the axis from the origin to the point;
    - "refined until it converged" → a non-monotone mesh study;
    - "No person designed it" → no genome was set or edited by hand, and the decoder, objective and rules are
      human-authored (the guard reuses f075 tables);
    - "cannot trade comfort for grams" → 1 % of E is worth 2.3 g;
    - "printable/as printed" → L-PBF and "print mesh";
    - "CVT" → fixed Halton centres;
    - "FE certification" → voxel FE check;
    - checkpoint sequence, not lineage;
    - "best" is first of a top eight spread over 1.8 points;
    - STATUS no longer lists unreleased v0.33–v0.38 milestones as released;
    - the `doi1991` reference gets its closing quotes;
    - v4.0.0's edits to the G3 paper sections are reverted, because they carried hand-typed numbers that
      contradicted the diagnostics.
  - **Provenance.** `docs/g3/provenance/` gives the SHA-256 of each document RS-karambit-v3 cites and what G3 took
    from it, plus a frozen `G3_GENOME_SPEC.md` v1.1.
  - **Tests.** They keep the overclaims out of the public G3 text and check the diagnostics hashes.
    `ops/ci/gen_readme.py` accepts a cohort of record from another major.minor line only when the release's own
    entry here names it.
- **Paper:** `docs/g3/paper/g3-v4.0.1.pdf` (7 pages), with its copy in `cad/`. The v4.0.0 build had overwritten
  `g3-v3.0.2.pdf`; that file leaves the tree, and the v3.0.2 release asset is unchanged.

## v4.0.0 — 2026-10-27 — WITHDRAWN in v4.0.1 (major: the G4 generation — declarative 130-gene `G4.0` schema, `RS-karambit-v4`, true-3D physics & V&V suite, feasibility-first constrained QD search, and certified 35.42 g sub-50 g winner)

> **Withdrawn in v4.0.1** (owner decision, 2026-10-27). The claims below are kept as the record of what v4.0.0 stated; they are not results. See the v4.0.1 entry.

- **Owner request & feedback sweep.** Address all external peer-review and internal audit findings in `tips/feedback/*.md` — 20 Research Integrity findings (`F01`–`F20`), 26 Physics/Measurement findings (`AG-01`–`AG-26`), 12 Genome findings (`F-G1`–`F-G12`), 11 Requirement findings (`F-R1`–`F-R11`), 8 Search findings (`F-S1`–`F-S8`), 15 HPC targets (`H-1`–`H-15`), and 16 G3 paper errata (`P-1`–`P-16`) — while keeping every v1/v2 and v3.0.0 certified input (`g3_decoder_sha256()`) byte-for-byte unchanged (`AGENTS.md`).
- **Declarative `G4.0` genome (`genome/G4.0.toml`).**
  - **130 live genes (`D = 130`, `D_core = 114`, `D_T = 16`, 0 dead genes)** across 9 modules (`H`, `G`, `P`, `Z`, `X`, `I`, `E`, `T`, `A`), replacing G3's 35 dead/weak genes (`F-G1`, `F-G2`) and hardcoded grip floor (`F-G3`, `AG-05`).
  - **32 new structural & topological levers (`NG-01`–`NG-32`):** tip-bisector diamond crown (`t_tip_crown_mm`, `l_tip_crown_mm`, Study `E06`), $C^2$ flange thickening (`skin_flange_c2`), dorsal baton pad (`skin_dorsal_s1`), internal web lightening holes (`rib_hole_frac`), multi-bulkhead reinforcement, ring hollowing + radial port (`ring_hollow_t`, `ring_port_d`), and true-3D camber/twist/chisel (`camber_z1_mm`, `camber_z2_mm`, `twist_tip_deg`, `chisel_bias`).
- **Unified requirement specification (`requirements/RS-karambit-v4.toml`, `loads-v4.toml`, `materials-v4.toml`, `frame-v4.toml`, `PROVENANCE.md`).**
  - Explicit traceability for every load (`180 N` hiker, `260 N` hunter, `1.82 N·m` body twist, `16.2 N·m` two-hand torque, `0.80 N·m` tip torsion, `60 N` tip pry, `500 N` spine push, `3427 N` baton impact against `sigma_ult = 895 MPa`, `300 N` ring bearing, `30 N` ring back, `250 N` grip squeeze) and Ti-6Al-4V ELI HIP property (`sigma_y = 828 MPa`, `sigma_e = 510 MPa`).
  - Dual mass ledger (`as_printed_mass_g` vs `as_finished_mass_g`) and true per-load-case blade-before-grip yield-sequence ledger (`F03`, `AG-15`).
- **True-3D implicit decoder & sub-voxel physics (`crates/g4`).**
  - **Curvilinear coordinate frame (`(s, n_c, z)`):** replaces G3's `atan2` polar branch cut (`AG-01`) and $z$-independent 2.5D profile (`AG-02`).
  - **Sub-voxel polynomial volume integration (`AG-06`):** $O(h^2)$ divergence-theorem face + tet quadrature eliminate G3's $50.8\text{ g} \to 62.6\text{ g}$ voxel step-function undercount (`F01`), reducing grid drift between $h = 0.40\text{ mm}$ and $h = 0.20\text{ mm}$ to `0.05 %` (`<= 2.0 %` gate).
  - **Resolution-invariant drainability (`AG-07`):** morphological erosion by $r_{\text{esc}} = 0.50\text{ mm}$ before 26-connected flood fill guarantees $\beta_0 = 1$, $\beta_2 = 0$ sealed voids, and `0.0 mm³` trapped powder (`F02`).
  - **Multi-point `HM-1` Winkler hand contact equilibrium (`AG-14`):** 4-finger compliant contact replacing the non-physical ring-only clamp (`0.61 -> 9.81` baton SF), paired with Hoff/Plantema sandwich face-wrinkling checks (`AG-21`).
  - **Hole-aware plate FE with 3-level Roache GCI (`AG-17`, `AG-18`, `F04`), frame-invariant `TET10` solid-tip FE (`AG-19`, `AG-20`), 2D `T6` serration tooth FE on all teeth including end teeth (`AG-22`, `F05`), and locking-free B-bar `hex8` voxel FE (`AG-23`).**
  - **11 / 11 V&V verification benchmarks pass (`crates/g4/tests/vv_suite.rs`):** NAFEMS `LE1`/`LE10`, Euler-Bernoulli B-bar patch test, Kirsch stress concentration, Roache 3-level GCI, rigid-body `SO(3)` invariance, and analytical spherical shell volume.
- **Feasibility-first constrained QD search (`crates/g4/src/search.rs`, `g4-evolve`).**
  - **Deb's lexicographic superiority (`FI-2Pop` dual archive):** separates feasible designs ($V(x) = 0$, ranked by `W_ERGOLIGHT_v2`) from infeasible stepping stones ($V(x) > 0$, ranked by violation sum of squares), eliminating penalty-slope gaming (`F01`, `F-S3`).
  - **Measured 6D behavioral descriptors (`F09`, `F-S1`), Lloyd-relaxed CVT centroids (`F-S2`), 4-emitter UCB1 bandit (`F-S4`), and isotonic-calibrated 3-tier cascade with probabilistic LCB (`F-S6`).**
- **Certified G4 v4.0.0 winner (`g4v400_s042_62763aa2`, [`evidence/g4/v4.0.0/release.json`](../evidence/g4/v4.0.0/release.json)).**
  - **Feasible across all 29 hard gates (`V(x) = 0.0000`, minimum margin `+0.0250`)** with **`W_ERGOLIGHT_v2 = 92.45`** (`W_ERGOLIGHT_v1 = 94.90`).
  - **Sub-50 g mass:** **`35.42 g` as-finished** / **`35.68 g` as-printed** (`67.3 %` hollowness vs `108.30 g` solid outer envelope; `0.05 %` grid drift).
  - **All 17 structural LCB safety factors $\ge 1.00$:** solid-tip torsion SF `3.41` (LCB `3.29`), `HM-1` grip governing SF `6.43` (LCB `6.19`), human-max twist SF `1.72` (LCB `1.66`), Goodman fatigue SF `2.40` (LCB `2.31`), per-case yield-sequence ratio `7.54` lateral / `8.68` twist / `2.22` spine (LCB `2.17` vs `1.20` floor), and `9 / 9` serration teeth checked (`min tooth SF = 12.53`).
- **Papers, diagnostics & `cad/` upload kit.**
  - New G4 paper [`docs/g4/paper/g4-v4.0.0.pdf`](g4/paper/g4-v4.0.0.pdf) generated from `evidence/g4/v4.0.0/release.json` via `g4-release-tex`.
  - G3 paper [`docs/g3/paper/g3-v3.0.2.pdf`](g3/paper/g3-v3.0.2.pdf) updated with all 16 errata (`P-1`–`P-16`) and backed by [`evidence/g3/v3.0.0/diagnostics/`](../evidence/g3/v3.0.0/diagnostics/).
  - Top-level [`cad/`](../cad/) updated with the 6-file upload kit for `g4v400_s042_62763aa2` (`STL`, `3MF`, `DXF`, `PDF`, `JPG`, `print_spec.json`); the v3.0.0 kit is archived in [`cad/v3.0.0/`](../cad/v3.0.0/).

## v3.0.2 — 2026-10-26 (patch: repository cleanup; the `cad/` upload kit; G3 release tools as binaries)

- **Owner request:** "a full audit looking for clean up … redundant files, old temp folders, older worktrees, poorly
  designed code layout … ensure this repo is VERY clean and a great open source", and "no waste in the repo". The
  owner chose to delete waste but keep the v1/v2 study and its evidence, and not to rewrite git history.
- **Removed (402 files, 159.8 MB of the tree; all of it stays in git history at tag v3.0.1).** Every file was checked
  first for references from CI, tests, generators, papers and receipts, and only unreferenced or duplicated files
  were removed.
  - **Agent coordination files:** `COMMS.md`, `COORD.md`, `briefs/`, `PY_TO_RUST_PLAN.md`, the agent launchers
    `ops/start_{codex,grok}.sh`, the render-pipeline status notes, and 34 agent receipts, plans and review notes in `docs/paper/`
    and `docs/` (work plans, release plans, readiness and capacity notes).
  - **Stub folders:** `cli/`, `evolve/`, `fem/`, `genotype/`, `monte_carlo/` and `phenotype/` (one README each), and
    the out-of-date `TECHNOLOGY_MAP.md`. The README's Repository section is now the map.
  - **Orphan v1 images:**
    - the wave-27 FEM plates (56 MB);
    - the quarantined pre-blade-fix heatmaps (26 MB);
    - the wave 13–17 plates (12 MB);
    - the v2 marketing stills and videos (9 MB), and a concept render.
    The data indexes that list these images stay.
  - **Superseded drafts and duplicates:**
    - the `docs/paper/drafts_p3_p5/` drafts (only its four live build inputs remain);
    - `evidence/publication_packet_20261019/`;
    - seven C9.3 plates identical to C9.2's;
    - a duplicate validation note and an orphan JSON.
  - **Experiments and dead code:**
    - two experiment studies no paper or check uses (`experiments/G16-12-RSV2-04/`, `experiments/rsv2_07/`);
    - seven unused exploration images;
    - six dead scripts;
    - `ops/ci/requirements-runners.txt`.
    41 Python files fewer in all.
- **Layout.**
  - The G3 release tools are binaries in `crates/g3/src/bin/`: `g3-pilot`, `g3-gripfe`, `g3-export-stl`,
    `g3-bladefacts`, `g3-edgeprofile`, `g3-bladefield` and `g3-tipconv`. Only the review probe `hz_probes` stays an
    example.
  - New indexes: `docs/README.md`, `ops/README.md`, `experiments/README.md` and `designs/README.md`, plus a full
    repository map in the README.
  - `.gitignore` now covers agent worktrees, build directories and editor files, and no longer hides tracked CAD
    evidence.
  - A `LICENSE` file: MIT, the license every crate already declares in its `Cargo.toml`.
- **Fixes.**
  - The README's Reproduce section told readers to check out the cohort's commit and then build tools that did not
    exist at that commit. The decoder and `g3-evolve` have not changed since, so the instructions now build from the
    release. The G3 paper's §8 says the same.
  - `ops/ci/RUNNERS.md` pointed at a removed Python checker.
  - `PYTHON_EXCEPTIONS.md` had wrong line counts.
- **CI.**
  - `version-check` now runs `make -C docs/g3/paper check`, so the G3 paper's numbers are gated against the receipt.
  - A new test checks that `PYTHON_EXCEPTIONS.md` lists exactly the tracked Python files.
- **The `cad/` upload kit (#174)**, merged after v3.0.1, is part of this release. Its paper copy is
  `docs/g3/paper/g3-v3.0.2.pdf`.
- **Unchanged.** The decoder and its hash, the v3.0.0 receipt, cohort and CAD files, and every certified evidence
  file.

## v3.0.1 — 2026-10-26 (patch: README, G3 paper, Paper B and STATUS made consistent with the code and the v3.0.0 receipt)

- **Independent check.** Every claim in the G3 paper was checked against `crates/g3` at be36df9f. All corrections are
  wording; no number in the v3.0.0 receipt changes.
  - **Tooth counts.** The "3 and 5 teeth" are scallops (gullets); 2 and 4 teeth between scallops are strength-checked.
    That check is a release measurement and a decoder test, not a term of the score.
  - **Analytic quantities.** The buckling factor and the tooth safety factors are analytic, not finite-element results.
  - **Fixed terms.** Two objective terms are fixed or capped in this release: the plate GCI at 0.5, and the solid tip at
    0.8 of its knee value.
  - **Model and mesh.** Tier 1 is one CST+DKT sandwich-plate mesh, and the ring is a T6 membrane band. Surfaces come
    from marching tetrahedra.
  - **Grids.** The voxel FE runs at h 0.4. Only the ranking and the mass of record are at h 0.2.
  - **Search details.** Niche centres are Halton points, and only three of the eight descriptors are measured. The
    variation and parent-selection details are now as the code has them.
  - **Reproduction.** The command now carries every flag that sets the random stream.
- **G3 paper.**
  - §1–§4 are rewritten from AGY's expansion in #169, with its errors corrected. Its weight table, `sfk` formula and
    multiplicative-E argument are taken as written, and so is its float placement.
  - The paper is `docs/g3/paper/g3-v3.0.1.pdf`. `\gVersion` is the release; `\gRecord` is the cohort of record.
- **README.** It links the paper, uses the scallop and tooth wording above, and states which grid each number uses.
- **Paper B.** Figure 10 shows the v3.0.0 best individual instead of the older pilot. The pilot's renders and
  provenance moved to `docs/g3/history/pilot-hz4i012_g00085/`, and `docs/g3/RENDERS.md` now describes the v3.0.0
  images.
- **STATUS.md.** It leads with the current G3 design, and the v1 status follows as a historical record.
- **Version plumbing.** `evidence/g3/RECORD` (3.0.0) names the cohort of record. The README, paper and
  `ops/g3/export_best_stl.sh` read the receipt through it, so a patch release keeps the v3.0.0 evidence byte for byte.
- **Unchanged.** The decoder, the receipt, the cohort, the CAD files and every v1 evidence file. The CAD assets stay
  attached to the v3.0.0 release.

## v3.0.0 — 2026-10-26 (major: the G3 generation; README and paper generated from one release receipt)

- **Scope change (owner, 2026-10-26).** G3 becomes the headline: an evolved one-piece Ti-6Al-4V karambit for laser
  powder-bed fusion, with a hollow lattice-filled grip and blade core. The README and the new G3 paper
  (`docs/g3/paper/g3-v3.0.0.pdf`) describe only the best individual of the v3.0.0 cohort of record. Both are generated from
  `evidence/g3/v3.0.0/release.json`, and the README also shows the evolution.
- **Decoder r2** (#170, #171): one ideal grind on both full edges (EDGE-1..5); both edges serrated with teeth that do
  not snap (SERR-1..4); measured human-max twist and buckling; a converged solid-tip mesh; evolution history logging.
- **Process.** #171 and the release PR were merged under the owner's "no need to wait for CI" instruction (labelled on
  each PR). G3.1 (#169) moves to v3.1. Details, the audit status table and the owner requests are in
  [`docs/g3/RELEASE_3.0.0.md`](g3/RELEASE_3.0.0.md).
- **Unchanged.** The v1 study (white paper, Paper B, v1 evidence and certification inputs) is untouched apart from the
  version header.

## v2.4.5 — 2026-10-25 (Inferno [0.15, 0.95] stress ramp and 3D raking lighting on cover and hero figures)

- **Owner request.** "Pick a really good color map that looks great (not the default)… also, we want great 3D lighting/shading as well."
- **Inferno $[0.15, 0.95]$ stress ramp (`pocket_smoothing_figures.rs`).**
  - The cover and pocket-hero von Mises stress renders now use a 9-stop perceptually uniform **Inferno** colormap truncated to $t \in [0.15, 0.95]$ (deep violet `#2c0b57` at $0\text{ MPa}$ through magenta and orange to pale yellow `#f1ee72` at $\ge 450\text{ MPa}$, avoiding the bottom $15\%$ near-black shadow ambiguity and top $5\%$ near-white page washout, with neutral grey for the masked tip zone).
- **3D raking lighting and shading.**
  - `cover()` and `hero()` lighting updated to balanced 3D raking illumination (`ambient = 0.35`, `diffuse = 0.65`, `specular = 0.15`, `shininess = 48`, key light `[-0.72, 0.58, 0.38]`) so pocket wall curvature and bevel relief read clearly without washing out the stress ramp.
  - Cover, inset, and hero figures (`docs/paper/figures/cover/*`, `docs/paper/figures/pocket/*`) and Paper A / Paper B PDFs (`docs/paper/main.pdf`, `docs/paper/main-smoothed.pdf`) regenerated and verified (`make -C docs/paper check pdf-smoothed`).
- **Unchanged.** v1 scores, receipts, census, ladder, and G3 `W_ERGOLIGHT_v1` draft d are untouched.

## v2.4.4 — 2026-10-25 (README leads with the G3 design and its cavities; paper illustration; W_ERGOLIGHT_v1 draft d)

- **Owner request.** "We need image updates ASAP… take the current leading individual hz4i012_g00085_4cdfea55… update the README smoothed image, if there are 3D cavities show a cut away to show where they are. Also update the paper." The owner had noticed that the README and papers still showed the v1 flat plate, whose handle edges are sharp; Paper B's smoothing only rounded the pocket walls.
- **README, current design first.**
  - Cycles renders of the leading G3 candidate `hz4i012_g00085_4cdfea55` (`W_ERGOLIGHT_v1` draft d 78.11, 52.3 g, computational, not qualified or built): a hero view; an **x-ray** with the outer shell as glass and every internal cavity in orange; and lengthwise and cross sections.
  - Measured from the exported meshes:
    - **Edges:** smallest hand-zone edge radius 2.7 mm against the rules' 2.0 mm.
    - **Cavity share:** grip 67 %, guard/neck 28 %, blade body 27 % behind solid edges and tip, ring solid; 53.5 % overall.
  - Provenance is in `docs/g3/RENDERS.md` (moved in v3.0.1 to `docs/g3/history/pilot-hz4i012_g00085/RENDERS.md`).
- **README, the published study.** The v1 winner now follows with the pocket-smoothed image only, and the header links feature Paper B. Paper A stays in the repository as the record B builds on.
- **Paper.** A four-panel figure (`fig:g3-candidate`) in the discussion's research-direction subsection illustrates the follow-on cohort's leading candidate. Its caption says it is outside this study's census and not a finding. Paper A is 18 pages and Paper B 19.
- **G3 objective, draft d (owner decision).**
  - `solid_tip_3d` rises from 0.030 to 0.070. The +0.040 comes pro-rata from the 12 other structural terms; mass and the ergonomic betas are unchanged, and there is still no hard gate. The pilot's double-edged points had reached a converged 3D solid-tip SF of only 0.11–0.51.
  - Parity: a draft-d fixture (25 cases); Rust equals Python to 1e-9.
  - A proposed seeding flag was withdrawn in review: G3 runs start only from Sobol. The draft-d cohort (16 islands × 10,000) runs from a Sobol start.
- **Unchanged.** v1 scores, receipts, census, ladder and every assessment are untouched; nothing in the v1 study is re-solved or re-scored.

## v2.4.3 — 2026-10-25 (paper Phase 2: native dynamics, score decomposition, requirements table; README and G3 cohort note)

- **Owner request.** "Please ensure README and paper are updated." This release lands the paper study's Phase 2 displays, which the G3 helper agent built and Claude-HZ reviewed and corrected. It also updates the README and adds one prospective paragraph on the separate G3 cohort.
- **Native-population dynamics (M7).** A generated figure shows the ten completed native runs per logarithmic generation bin: the legal share, and the median with the q10–q90 fitness band. Data: `docs/paper/allruns/native-dynamics.dat`, from the gen800 traces.
- **Where the cover design's score comes from (M8).** A generated figure decomposes the canonical `W_ULTRALIGHT_v1` score of `ec3i042_r01_p10_c56` by evidence class: measured on exported CAD and FE, rule-based, constant fallback, and not admitted. The classes reproduce 41.44 / 24.00 / 11.18 / 4.00 = 80.62. The contract is `docs/paper/data/term-evidence-class.v1.json`. Fallback-matched terms are computed against the scorer's constants, never assigned by hand.
- **Requirements table (M9).** The cover design's measured responses are listed against reference checks, captioned "informational; does not veto selection".
  - Each status is computed from the value and its reference rather than typed.
  - The two checks with no measurement (the edge-sharpness law and the tip point profile) say "not measured".
- **Fail-closed generator.** Every numeric receipt read in the Phase 2 generator now errors when the field is missing. Before, a missing value became 0, 1 or 4.5.
- **README.** The findings match the papers (four, plus a fifth in B). The cover caption names the one-hue ramp. A new section describes the separate G3 ergonomic, hollow-AM cohort (`crates/g3`, `RS-karambit-v3`, `W_ERGOLIGHT_v1`) without claiming results.
- **Paper discussion.** One paragraph names the follow-on cohort as a research direction. It is versioned and identified separately, so none of its candidates can enter the census, and it reports no result here.
- **Unchanged.** Scores, receipts, census data, the ladder and every assessment are unchanged. Nothing was re-solved or re-scored, and there is no qualification claim.

## v2.4.2 — 2026-10-25 (paper upgrade: layout, page 1, introduction and conclusion, corrections, one file per section)

- **Owner request.** The owner asked for the papers to have "incredible layout, font, and the best possible plots, description, key sections, strong intro, strong conclusion", after studying two outside drafts (`tips/paper/*.tex`). The owner then authorized the work through to release ("Please do what is best and keep this moving forward").
- **Layout and type (both papers and the Supplement).** `docs/paper/ei-ieee-skin.tex` keeps IEEEtran two-column at 10/12. It adds TeX Gyre Termes with matching Termes Math (previously Computer Modern math beside Termes text), sans headings, captions and tables, one callout style, slate running heads, and thousands-grouped counts. The page count was measured before adoption.
- **Page 1.** A kicker row, the cover, a row of key numbers drawn only from generated macros, and a new abstract that leads with the design on the page. Papers A and B now have different abstracts.
- **New introduction and conclusion.** The introduction states the research question, four findings (five in B), the contributions and a scope box. The conclusion gives three lessons from the records, what is not established, six prospective application classes with the evidence each would need (rows are prospective, not results), and the claim stated conditionally. A Data and Code Availability section is added.
- **Cover colour.** The stress fields on both covers and in Fig. 4a use EI-HEAT-1H, a one-hue sequential ramp (OKLCH hue 40°, lightness monotone from 0.956 to 0.370). It replaces the rainbow ramp, whose mid-scale yellow read as hot spots. The cover light is flatter (0.62/0.38/0.04), so shaded faces keep their key colour. The figures were regenerated from the receipt.
- **Corrections to v2.4.1:**
  - Paper B's mesh-smoothing range was a stale 75.06–75.34 from the earlier design's ladder. It is now generated from the winner's ladder: 75.45–75.68.
  - "Ribs keep their width" now says rib widths were inspected in renders, not measured.
  - The smoothing time comes from the receipts.
  - The inflated-evaluator sentence now says only one evaluator was re-assessed and the second was matched by stored metrics.
  - The v2.3.1 sentence now says the ring-wall floor removal and the seed Hall-of-Fame change landed together and were not separated.
  - "Raking light" is removed from the cover caption.
- **Evidence contract.** New generated macros come from receipts: distinct genomes, the leader's re-assessment, the inflation example and range, the previous leader, the Cohort-3 totals, the solver agreement, the smoothing outcomes, the 3D masses and the seed parent's score. 34 numbers that were typed into the sections now come from macros.
- **Plots.** The census figure is now one column wide, uses the validated categorical palette, and labels the leader and the previous leader's tenure directly. The Stage-1 panel moved to the Supplement.
- **Room.** Six floats and the response-display subsection moved unchanged to a new Supplement section. Paper A is 16 pages and Paper B 18 (limit 15–20). `make check` now checks both papers.
- **Citations.** Added Taubin (1995), Osher–Sethian (1988), Gage–Hamilton (1986) and Smith–Winkler (2006).
- **Parallel editing.** Each section of both papers and of the Supplement is now its own `.tex` file. `docs/paper/SECTIONS.md` indexes them in reading order, with their labels and a claim-before-editing protocol.
- **Unchanged.** Scores, receipts, census data, the ladder data and every assessment are unchanged. Nothing was re-solved or re-scored.

This release adds no qualification claim.

## v2.4.1 — 2026-10-24 (false-colour engineering fields on both first pages)

- **Owner request.** Each paper's first page now shows its knife in false colour from the engineering tests, with 3D shading. Paper A shows the design as exported; Paper B shows the pocket-smoothed export.
  - The colour is the plate-model von Mises envelope (D1 `env_vm`, MPa) stored by that design's own Stage-2 assessment. It is the maximum over the lateral (260 N), in-plane tip-pry (260 N), torsion (1820 N mm, with its 3D correction factor) and ring-pull (1500 N) load cases. Paper B's field comes from its re-assessment on the smoothed geometry.
  - Both pages use one absolute scale, 0 to ≥ 850 MPa, so colours compare directly. The 99th percentiles are 822.8 MPa (A) and 815.6 MPa (B), the peaks are 1289 MPa in both, and 0.57 % and 0.49 % of the area saturates.
  - Grey marks the 4 mm tip zone. The producer stores no plate value there, because the 3D solid-tip test covers it.
  - Tests with no stored field are listed as values from the receipt: the 3D solid-tip safety factor (0.689 for A, 0.645 for B), the fatigue safety factor (0.561, 0.562) and the failure-sequence ratio (1.18, 1.34).
  - The view is oblique (40°) at true thickness under raking light, with a soft shadow. Both pages use one camera, and the 2.5× isogrid-pocket inset is rendered at full resolution. Shading shows the geometry, colour shows the stored response, and nothing is re-solved or re-scored.
- **`pocket_smoothing_figures cover`** writes both renders, insets and page SVGs, and `cover.render.v1.json` records the camera, scale, colour map, statistics and inputs. It reads both STLs and D1 reports from the subject's `RECEIPT.json` and stops if any sha256 differs. `make pocket-figures` calls it in place of the grey `smoothing_figure` renders, so `smoothing-figure.render.v1.json` is removed.
- **Unchanged.** Scores, the receipt, the stacked-field hero, the black-and-white and morphology views, the ladder and the supplement are byte-identical. Paper A is still 19 pages and Paper B 20.

This release adds no qualification claim.

## v2.4.0 — 2026-10-24 (a better evolved design on both papers; only the winner is smoothed)

- **New best evolved design.** With the plateau fix (#158), the v2.3.1 islands climbed past the paper's 80.299390. The best of the 1,761 comparable retained Stage-2 assessments of every run since 2026-10-17 (census read 2026-10-24T06:09Z, when the v2.3.1 cohort ended) is `ec3i042_r01_p10_c56`, from xbabe3 island i042. That island was seeded with `ec1i012_r04_p10_c101` (80.594628), whose island was seeded with `w406t3_r01_p10_c16`.
  - As exported it scores `W_ULTRALIGHT_v1` = **80.623796** (83.55 g, battery 13/18, not qualified). A complete Stage-2 re-run with the island's evaluator (v2.3.1) reproduces this exactly, and its STL and STEP match the island's export apart from the label.
  - Paper A (`main.pdf`, 19 pages) shows it as exported.
  - Paper B (`main-smoothed.pdf`, 20 pages) shows it after pocket smoothing at r = 0.75 mm: 80.561227 (−0.062569). Lateral capacity, failure sequence and fatigue gain 0.069 points; the 3D solid-tip objective loses 0.131 (nodal maximum 0.6886 → 0.6450). The setting was chosen on `w406t3_r01_p10_c16` and is not re-tuned.
  - 14 of the 15 pocket basins keep their removed volume exactly; one 0.22 mm³ sliver at the start of the blade taper flattens (total removed volume −0.014 %). The paper and the experiment README say so.
  - Regenerated for it: covers, the stacked-field hero, the black-and-white and morphology views, the selection record (`winner-selection.v1.json`), the all-runs census and evolution figure, and the smoothing ladder.
  - Paper B's pocket-smoothing table keeps the settings study on `w406t3_r01_p10_c16` and adds the new design at the chosen setting.
  - The mesh-smoothing ladder (`smoothing-ladder.pdf`) now has seven levels, L0–L6. On this design, Taubin smoothing with 60° feature pins reverses a face at 60 or more iterations, and the smoother stops (fail-closed), so the former L7–L9 are not published. The face-reversal check is unchanged.
  - The results section now explains the earlier plateau. The continuous evolution kept the isogrid, lattice and window layers, but until v2.3.1 it clamped every child's ring wall to at least 6 mm, and it peaked at 76.799579.
- **`make pocket-figures` follows the committed receipt.** `POCKET_SUBJECT` names the subject. The smoothed STL, its D1 fields and the raw export come from that subject's `RECEIPT.json`, as the ladder target does since #157.
- **Operations, outside the repo:**
  - To save time, a smoothed Stage-2 started whenever a design became the live #1. Runs of earlier leaders are not reported; those still running when a better design appeared were stopped.
  - Agents sharing xbabe1–xbabe4 coordinate compute in `~/master_log.md` on xbabe2.

This release adds no qualification claim.

## v2.3.1 — 2026-10-23 (Paper A plain, Paper B pocket-smoothed; evolution keeps the leading design's lineage; CI green again)

- **Final paper layout (#159, owner decision).** Paper A (`main.pdf`, 19 pages) shows the best evolved design `w406t3_r01_p10_c16` as exported (80.299390).
  - Paper B (`main-smoothed.pdf`, 20 pages) shows the same design after surface-tension pocket smoothing at r = 0.75 mm, under Paper A's cover camera and inset: 80.370405, re-assessed in full, with the CAD/B-rep objective kept.
  - Paper B's *Surface-Tension Pocket Smoothing* section replaces the Taubin section. The stacked plate-field render and the raw-versus-smoothed close-ups share one figure.
  - Mesh-level Taubin smoothing stays only as the comparison ladder (`smoothing-ladder.pdf`).
  - v2.3.0 had left `main.pdf` at 21 pages, so `make -C docs/paper check` failed.
- **Phase-9 source drift recorded (#159, owner decision).** The G16-06-RSV2-08 Phase-9 study pins its production sources and verifier. v2.3.0's pocket smoother changed three of them, so every version-check job failed.
  - The study receipt stays byte-identical. `experiments/g16-06-rsv2-08/source_history/` keeps the study-time files, and `source_drift.v1.json` lists each changed file with its study and current digests.
  - `audit_phase9` accepts a changed file only at its recorded digest and only while its history copy matches the receipt; any other change still fails.
- **Islands keep the leading seed's lineage (#158).** Continuous evolution had plateaued around 76.3, even though every island was seeded with the 80.299390 design `w406t3_r01_p10_c16`.
  - Every RS-v2 child had its ring wall clamped to at least 6.0 mm. The leader's is 5.2 mm, so each child gained about 6 g and lost 4–5 points in Stage 1. The floor is removed: ring strength is a weighted objective, and the gene stays inside its schema bounds.
  - Stage 2 does not re-assess exact seed parents, so a seed never entered the Hall of Fame. `auto-evolve --seed-scores` now gives seeds their known canonical score, and the Hall of Fame takes the higher of that and the run's own Stage-2 best. The score steers parent choice only and is never reported as the run's assessment.
  - `auto_evolve.rs` is outside the certificate identity, so no evidence changes.
- **Smoothing-ladder target follows the committed winner (#157).** `LADDER_LABEL` and `WINNER_DIR` now come from `docs/paper/ladder/ladder.json`, not the previous winner.
- **Operations, outside the repo:** the keepalive passes each island its leader pool as `--seed-scores`.

This release adds no qualification claim.

## v2.3.0 — 2026-10-22 (surface-tension pocket smoothing; smoothed best design on the main page; stacked engineering-field hero)

- **Surface-tension pocket smoothing (`crates/crescent/src/pocket_smooth.rs`, opt-in).** The jagged, chipped walls on weight-saving pockets came from the 0.25 mm thickness raster, read by nearest cell and meshed by a ~0.7 mm CDT. The Taubin mesh smoothing of variant B pins exactly those walls, so it showed no visible difference.
  - The new pass smooths the removal depth before meshing, as oil would settle in a chipped pit. Each pocket is a watershed basin. Its depth contours move by curvature (level-set mean-curvature motion, line tension) for time r²/2. A per-basin pressure term holds the removed volume exactly.
  - Effects: chips go first, circles become clean circles, rib edges stay straight and corners round to about r. Walls keep their steepness, so rib widths and the I-beam webs are unchanged.
  - Fixed: the planform and holes, grinds, bore collar, raised features, the brand pad and a 1 mm unpocketed rim. No basin may end deeper than its own original floor.
  - The exporter adds CDT vertices in three rows along each wall and in pairs across the grind boundary, so walls mesh as clean ribbons.
  - Switch: `JARAMBIT_POCKET_SMOOTH=on` (r = 0.75 mm, a = 0). It is **off by default**; with it unset every export is byte-identical, and the retained export regeneration check passes unchanged.
  - Bugs fixed during development, each with a regression test: the ring collar losing ~55 mm³; a 1 mm³ blade groove drilling toward the minimum web and becoming the D1 lateral weak point (205.9 → 201.8 N); and NaN grind-edge vertices.
- **Post-smoothing fitness of the leading design.** The complete Stage-2 evaluator re-assessed the smoothed CAD. Nothing is withheld, and the STEP solid is valid.
  - `w406t3_r01_p10_c16` at r = 0.75 mm scores `W_ULTRALIGHT_v1` = **80.370405**, against 80.299390 as exported (+0.071015; 83.21 g, battery 14/18, not qualified).
  - Other settings: r = 0.5 → 80.164307; 1.0 → 80.308595; 1.5 → 80.158645; 2.0 → 78.275029 (tip solve unmeasured); film a = 1 → 80.161219. An unsmoothed re-run reproduces 80.299390.
  - The differences come from the 3D solid-tip nodal maximum, which is sensitive to the local triangulation. Mass, lateral, ring, twist and fatigue move by less than 3 %.
  - Cost: about 40 % more export triangles, and a longer Stage-2 run.
- **Certificate identity re-issued.** `export.rs`, `lib.rs`, `certification.rs` and the new module are part of `certification::identity`, and so is an enabled setting.
  - The regeneration identity in `evidence/evolve_certifiable/manifest.json` is therefore re-issued. STEP/STL bytes are unchanged, and `check_export_regeneration.py` passes.
  - It is re-bound, metadata only, in 25 files (`figure_input_rebinding_20261022_v2_3_0.json`). Generated paper metadata and `docs/tests/visuals.v1.json` are refreshed (hashes only).
- **Paper.**
  - `main.pdf` page 1 is a new stacked engineering-field hero on the smoothed CAD. All five retained plate responses (lateral bending, chord torsion, ring traction, in-plane tip pry, multi-case envelope) are each normalized by their own area-weighted 99th percentile and stacked as exploded layers, with the normalized envelope on top. It uses an oblique camera, raking light and depth-tested crease lines.
  - A new section, *Surface-Tension Pocket Smoothing*, has black-and-white raw-versus-smoothed views and the Stage-2 table.
  - The observed-morphology relief now shows the smoothed design.
  - `main-smoothed.pdf` (variant B, Taubin mesh smoothing) is kept and also gains the new section.
  - New producer `crates/paper-artifacts/examples/pocket_smoothing_figures.rs` and target `make -C docs/paper pocket-figures`.

This release adds no qualification claim. No design passes all 26 legacy gates, and physical performance remains unestablished.

## v2.2.1 — 2026-10-22 (no hard failure left in Stage-2 assessment; best design across all runs; smoothing ladder; all-runs evolution figure)

- **An unmeasured solid-tip solve scores zero instead of dropping the design (#153).** This was the last hard failure in the evolution loop. When the skfem tip solve or the DOLFINx cross-check produces no observation, the evaluator writes an `unmeasured` tip record with SF 0. Assessment then continues, so the design is scored on the other 19 objectives and gets zero credit on `solid_tip_3d_p2_and_xcheck`.
  - The record is never reusable evidence, the CAD battery reports `solid.tip` incomplete, and rs-v2-eval reports `solid_tip_sf` unverified.
  - The evaluator is part of the certificate identity, so the regeneration identity in `evidence/evolve_certifiable/manifest.json` is re-issued. Exported STEP/STL bytes are unchanged, and the figure manifests that pin it are re-bound metadata-only.
- **Best evolved design, across every run (#155).** The all-runs census re-scores 967 comparable Stage-2 assessments on xbabe1–4 since 2026-10-17 with the canonical `W_ULTRALIGHT_v1`.
  - `w406t3_r01_p10_c16` ranks first (80.299390, 83.22 g, battery 14/18, not qualified). It comes from the 2026-10-17/18 waves and carries two-sided isogrid pockets, lattice, relief and pierced-fuller windows.
  - Re-assessing its genome with the current Stage-2 evaluator reproduces 80.299390 exactly. Two other designs from those waves also reproduce exactly.
  - The 2026-10-16 evaluators overstate the score: a stored 92.29 re-assesses at 79.166355. Their 142 scorecards are excluded.
  - The best design of the runs after 2026-10-18, `ec1i005_r01_p14_c32` (76.141571), switches the pocket layers off and ranks 107th.
- **Papers updated.**
  - `main.pdf` shows the winner as exported.
  - `main-smoothed.pdf` shows it after R1 smoothing, re-scored to 75.343635. That is −5 for the withheld CAD/B-rep objective, +0.0246 for the solid tip and +0.0196 for mass; with the B-rep objective kept it would be 80.343635.
  - The Individual 2000 hero image is replaced by the winner in relief.
  - The main evolution figure now shows every run on every host. The Run 5 per-birth view moves to the Supplement.
- **Smoothing ladder in the build (`make -C docs/paper smoothing-ladder`, #155).** `jarambit smoothing-ladder` smooths the winner at ten levels (L0 = none) and re-scores each one, re-solving the tip with skfem and DOLFINx. `smoothing_ladder_figure` renders every level for 3D relief: one oblique camera, raking light, 2.5× thickness exaggeration, height tint, guard/isogrid and blade close-ups, and a shared displacement map. The output is `docs/paper/smoothing-ladder.pdf`.
- **Continuous evolution (operations, outside the repo).**
  - Migrants take their canonical score when one exists.
  - The re-assessed lattice designs (`w406t3`, `w401c3`, `w290`) head the seed pool on xbabe1–3.

This release adds no qualification claim. No design passes all 26 legacy gates, and physical performance remains unestablished.

## v2.2.0 — 2026-10-22 (selection on W_ULTRALIGHT_v1 alone, canonical winner and smoothed paper variant, regression datasets)

- **Evolution selects on `W_ULTRALIGHT_v1` alone (#150).** This completes the 2026-10-18 ruling inside `auto-evolve`.
  - A Stage-1 aesthetic or holder/geometry reject is FEM-screened and ranked, with the canonical 1.0/0.5 credit on its C.6 and holder objectives.
  - A plate that cannot be meshed or solved stays ranked with zero FEM credit, recorded as `null` plus a status.
  - Elites come from the `W_ULTRALIGHT_v1` Hall of Fame only; legacy all-gates qualification no longer buys a slot.
  - An island fails only when it scored nothing.
  - Legacy gate verdicts are still measured and logged. Requirement files and thresholds are unchanged.
- **Canonical winner on the paper's main page (#151).** `ef3i17_r05_p14_c06` (`W_ULTRALIGHT_v1 = 75.983539`) ranks first of the 598 retained Stage-2 assessments re-scored with the canonical `ops/ci/ultralight_weighted_fitness.py`.
  - `main.pdf` shows it as exported.
  - `main-smoothed.pdf` shows it after the pre-declared R1 smoothing, re-scored to 70.977863:
    - 3D mass +0.0168.
    - Solid tip −0.0225, re-solved with skfem and the DOLFINx cross-check using the pinned `fem_solid` versions.
    - CAD/B-rep objective −5, withheld because the smoothed geometry is a mesh without a validated STEP solid.
  - **Correction:** `ef2i91_r03_p14_c26`, the smoothing subject of #149, is 75.213491 canonically. Its 80.253391 came from the 2026-10-18 xbabe2 run leaderboard, which overstates every one of its 218 entries; those stored values are no longer used.
- **Geometric smoothing (#149).** The smoothing benchmark gains input-mesh mode, `--normal-only` and `--pin-feature-angle`. Adds the smoothing figure producer (now with `--width`) and paper variant B (`docs/paper/main-smoothed.pdf`).
- **Regression datasets (#147, #148).** `auto-evolve` records a micro-record for every birth and assessment (`--dataset-dir`, create-only run output). Adds dataset tables and a CatBoost k-fold lift CI.

This release adds no qualification claim. No design passes all 26 legacy gates, and physical performance remains unestablished.

## v2.1.2 — 2026-10-20 (retained-evidence presentation and artifact packaging fixes)

Preparation notes: source changes are recorded here before final generation and validation. Publication remains pending the required local controls, independent review, CI, merge and release gates.

- Adds shaded presentation of the unchanged retained geometry with display-only normals, an explicit appearance recipe and a separate unlit quantitative companion. Lighting changes RGB; it does not change the underlying field, geometry or stored fitness.
- Adds source-bound all-record positive-coordinate views and per-run context while retaining the complete full-range views, original values, missingness and row accounting. Vivid score colors use a declared monotone original-score contrast within each profile; warmer means better, with original-unit keys and source-order overpainting disclosed. The detail view selects higher-score Q90 through the actual maximum and lower-deficit minimum through Q10, including cutoff and extremum ties with explicit membership counts.
- Displays the retained 20-term priority vector without changing its weights or scoring. Removes summary badges from the main presentation, corrects current-cover navigation and widens lateral/torsion display cameras to recognizable curved-tip context, with exact PiP viewport rectangles and separately retained saliency-selection statistics.
- Extends the paper CI artifact allowlist to the new reporting assets and renews source/output closure. Historical receipts, prior release notes and immutable inputs remain preserved; authored controls are not evidence of execution.
- Changes reporting and packaging only. It adds no optimization, geometry change, physical solve, new fitness evaluation or qualification claim; the published v2.1.1 tag and release remain unchanged.

## v2.1.1 — 2026-10-20 (retained-evidence reporting and provenance corrections)

- Corrects the response display to use source-bound available-channel means, pooled area-weighted tied ranks, explicit missingness and per-cell contributor coverage. The complete catalog/channel inventory records unavailable spatial observations without scalar broadcast.
- Uses the shared navy–teal–yellow–orange–red display transfer with full-range main rendering and separate contrast/support companions. Score views retain all admitted numeric observations, print their data-derived scales and add explicitly bounded detail and separate run context.
- Clarifies reporting aliases, selection limits and native recovery gaps; packages the display/axis contracts, row-level inventory and reproducible main/supplement build. This patch release changes reporting only; it adds no optimization, CAD export, physical solve or qualification claim.
- Incorporates the V4 source review: exact proposal/content joins, observed historical predicate disagreement, recovered score-recipe/default disclosures, retained runtime-capture limits and original scalar precision. Unsupported current buckling interpretation and dependent qualification inference are withdrawn while historical values, flags and score arithmetic remain preserved.
- Adds fail-closed designated-review provenance enforcement, explicit full-archive audit scope and complete source/output delivery bindings. Neutral guard controls and a real origin-bound independent approval remain separate requirements; the PR142 historical operator-origin gap is preserved.

## v2.1.0 — 2026-10-20 (retrospective design traceability publication)

- Adds the corrected 15-page IEEE-style manuscript and standalone 18-page supplement, with explicit evidence coverage, candidate identities, score profiles and selective assessment paths.
- Presents all 1,958,803 admitted numeric observations in separate source profiles, including 1,954,560 native births across ten completed runs, with the complete retained archive and source/input/output provenance.
- Improves figure legibility, captions and portable companion-PDF navigation. Original evidence, scoring contracts, scientific kernels, weights and thresholds are unchanged. This is a reporting release of retained computational observations; it adds no new search, FEM or physical experiments and establishes no design qualification.

## v2.0.0 — 2026-10-18 (option (a) major release: weighted objectives, no qualified design)

Scope chosen by Jepson Taylor on 2026-10-18 (11:36 AM MDT, option (a) plus the weighted-objectives addendum): ship the pipeline, the verified per-gate checks, and an honest frontier report. **No design qualifies under the legacy 26 hard gates** (16 `W_CAD_v2` + 10 `RS-karambit-v2`). Winner certification moves to the next release. `docs/RELEASE_PLAN_MAJOR.md` drafted this scope under the working label "v0.31.0"; it ships as v2.0.0 because the existing `v0.31.0` tag (2026-10-12) is an unrelated older release and `docs/VERSION` was already 1.0.0.

- PR-2.0.0a (#120, #122, #123): `W_ULTRALIGHT_v1` global weighted fitness (`crates/cli/src/auto_evolve.rs` `ULTRALIGHT_WEIGHTS_V1`, 0–100 scale) replaces the 26 hard pass/fail vetoes in evolution selection. Weights favour an ultralight running knife: carry mass and pocketability 30 %, carry ergonomics and ring comfort 25 %, cutting and B-rep geometry 15 %, and normalized structural and safety objectives 30 %. Every raw per-gate metric is still logged next to its legacy threshold. Hall-of-fame elitism keeps fitness monotone, and per-round best-so-far designs sync to S3. This is Jepson's decision to relax the hard gates in selection only. Requirement files and thresholds are unchanged, and the release reports every legacy threshold the best design misses.
- PR-2.0.0b (#123, #124): multi-host weighted waves on xbabe1–3 (`evidence/evolution_waves/2026-10-18_ultralight_v1/`). Best Stage-2 design `w411c3_r02_p10_c12`: 101.139 g, `W_ULTRALIGHT_v1 = 74.21`, 13/26 legacy gates met. It misses the 260 N hunter lateral floor (G16-06/07, RSV2-02/05), tip and solid-tip twist SF at 1.82 N·m (G16-10/11/12, RSV2-03/04), human-maximum loads (RSV2-07), fatigue (RSV2-08, SF 0.514) and the `rs_v2_eval` sharp-edge law (RSV2-09). See `docs/RELEASE_PLAN_MAJOR.md` §4.3. Best Stage-1 screen: `w501_ul_r01_p14_c01`, 83.065 g, 74.02, 3D assessment pending.
- PR-2.0.0c (#114–#117, #119–#121): all 26 gate checks built and verified as checks. Workplan rows closed in this release are labelled "check built + verified — not all-26 design passed". The J3 feasibility verdict is in `docs/FEASIBILITY_VERDICT_J3.md`, and Stage-2 novelty selection is in `auto-evolve`.
- PR-2.0.0d (#116, #125–#129, #131): Rust ports and CI. `jarambit version-check` (PY-PORT-01) and `jarambit check-runner-policy` (`crates/runner-policy`, its own Cargo workspace) replace their Python checks. Every job routes to local self-hosted runners on xbabe1–xbabe4. jankurai repository scoring runs on every PR. `PYTHON_EXCEPTIONS.md` and `PY_TO_RUST_PLAN.md` record the stack policy (Rust + Vite/TypeScript/React; Python only for listed scientific-kernel and Blender bindings).
- PR-2.0.0e: README refreshed for v2.0.0. It covers what is new, the architecture and stack, and the prerequisites. It also leads with a Blender Cycles render of an evolved genome (`hero_oblique.webp`, `ops/render/blender/render_paper.py`; not qualified). It also re-embeds the Cycles product film of the hand-built SWITCHBACK C.10d rc6 concept (`docs/render/media/switchback_c10d_rc6_hero.webp`, made by `ops/render/pro/pro_hero.py` / `jr-film` and `ops/render/pro/make_readme_hero.sh`), clearly labelled as hand-built, not pipeline output. The studio package version is synced to 2.0.0.
- PR-2.0.0f (#82–#95, carried from Unreleased): certification-integrity repair and independent assessment, detailed below.

### Certification-integrity repair (carried from Unreleased)
- Certification integrity repair (#75–#81): FEM uses the exported thickness; removed unsupported ring-width strength/fatigue multipliers; sharpness and the entire 0–10 mm tip profile are measured on the exported triangles; the unfilleted bore cannot meet the radius requirement. Missing independent solver evidence stays unverified. Evidence selection uses genome/model content identity, mesh/D1/payload hashes and distinct solver observations.
- Both committed Wave 283 genomes are re-certified with the standard scikit-fem P2 / DOLFINx pipeline. Their `all_hard` and 16-gate pass claims are withdrawn; the generated README and STATUS list the corrected measured results. No requirement, threshold, load, bound or exported genome was relaxed. The invalid release evidence is preserved under `evidence/cert_integrity/v1.0.0-retracted/`.

## v1.0.0 — 2026-10-16 (released software; Wave 283 certification claims withdrawn)

**Erratum:** The original all-hard, sharpness, tip-profile and independent-solver pass claims were invalid (#75–#81). Neither `w283_p14_c01` nor `w283_p10_c02` qualifies under the corrected pipeline. See the Unreleased repair, generated README/STATUS and preserved retracted certificates. The release tag is historical and has not been moved.

- PR-1.0.0d: Added `./evolve.sh` and native `auto-evolve`. Native solid results are screens; an independent solve must be recorded before certification. Search output cannot establish an all-hard pass from missing or failed measurements.
- PR-1.0.0c: Introduced the Wave 283 genomes and geometry. The original qualification results have been retracted and superseded by the corrected evidence; geometry mass is unchanged.
- PR-1.0.0b (`6d18ef8`):
  - **Interactive Vite + TypeScript + React 3D CAD & Gate Scorecard Studio (`web/`, `jarambit export-web-data`, `docs/figures/whitepaper/fig_web_studio.png`)**: Added interactive WebGL Three.js 3D STL viewer (metallic Ti-6Al-4V shading, wireframe toggle, bounding box, auto-rotate), 16-gate `W_CAD_v2` margin bar chart, 23-subject × 18-evaluator CAD battery matrix explorer, and `W20`–`W283` Pareto trajectory explorer in `web/`, backed by `jarambit export-web-data --out-dir web/public/data` and 5 Vitest unit tests (`npm --prefix web test`).
- PR-1.0.0a (`37a01a9`, `59c5d80`):
  - **100% Native Rust Physics, CAD & Scoring Stack (`crates/physics/src/solid_tet10.rs`, `crates/crescent/src/{g1.rs,gci.rs,cad_ingest.rs,battery.rs}`)**: Ported locked `G1`, 3-level Roache `GCI`, standalone 3D STEP/STL parser + voxel/column rasterizer, 23×18 CAD battery, `score-cad`, `side.a4` (radial bore-pin + Winkler grip foundation + Lanczos generalized buckling), and 3D `TET10` quadratic tetrahedral solid-tip FEM (`faer` sparse Cholesky) into native Rust with zero Python subprocesses required at runtime.

## v0.32.9 — 2026-10-16 (Wave w281 sub-76 g record, 19-subject CAD battery, and refreshed hero figures)
- PR-0.32.9b:
  - Synced README badges, Quick start, White Paper, engineering tables, STATUS, and doc tests to **`w281_s2932_c01`** (`75.911 g` 2.5D raster / `75.695 g` 3D solid, `G1 = 0.880825` COMPLETE, `W_CAD = +1.7916`, `16/16` engineering gates pass). Regenerated `fig_readme_mass_vs_wave.png`, `fig_why_this_matters.png`, and `fig_w279_gate_margins.png` with SHA-256 manifests through `w281`. The fresh Install check also identified and documented the Linux Gmsh `libGLU.so.1` runtime dependency. The committed battery contains 19 subjects × 18 evaluators (342 cells): `184 pass`, `65 fail`, `84 blocked`, `9 incomplete`.
- PR-0.32.9a (`28f6907`):
  - Committed evidence in `evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_{cad_eval,D1,rs,a4,solid_tip}.json` and `evidence/cad_battery/w281_s2932_c01.json` establishes **`w281_s2932_c01`** (`75.911 g` 2.5D raster / `75.695 g` 3D solid, `G1 = 0.880825` COMPLETE, `W_CAD = +1.7916`, `16/16` engineering gates pass): certified lateral `265.4 N`, strict lateral `267.4 N` / `SF = 1.0285` (`+2.85%` reserve), certified `seq_ratio = 1.212 >= 1.20`, tip-only twist `SF = 1.141`, strict tip twist `SF = 1.125`, body twist `SF = 3.015`, 3D P2 solid-tip `SF = 1.4094` with `0.0284%` cross-solver difference, `side.a4 seq = 3.950` / `P_cr = 3041.64 N`, and `SPG = 3.4966 N/g`. Mass fell `1.054 g` raster / `1.138 g` solid from `w280_s2931_c01`; strict lateral reserve fell from `+5.15%` to `+2.85%`. CAD mass agreement is `0.284%`. Peer seed genomes `w281_s2932_c02_light_genome.csv` and `w281_s2932_c03_cert_genome.csv` are committed in the same wave directory; no peer performance results are published because no committed peer JSON exists.
  - Human-maximum loads, L7 fatigue (`fat_sf = 0.7414`), ring-bearing width (`5.30 mm < 10 mm`), and the sharp-law gate still fail; the tip-point B-rep gate remains unverified. The 16-gate search result does not establish an all-hard RS-karambit-v2 pass or manufacturing promotion.

## v0.32.8 — 2026-10-16 (v0.32.8 patch release: Wave w280 74-77 g stretch-band winner w280_s2931_c01 at 76.965 g / 76.834 g 3D & Pareto peers, 18-subject CAD battery, and w20..w280 hero & gate-margin figures)
- PR-0.32.8b:
  - **JSON-Driven `w280_s2931_c01` Record Badges, `w20`–`w280` Hero & 16-Gate Margin Figures, and 18-Subject CAD Battery Sync (`README.md`, `ops/ci/gen_readme.py`, `ops/ci/tests/test_gen_readme.py`, `ops/render/whitepaper_hero_figs.py`, `docs/figures/whitepaper/fig_*.png`, `docs/WHITEPAPER.md`, `ops/ci/gen_whitepaper.py`, `ops/ci/tests/test_gen_whitepaper.py`, `docs/ENGINEERING_GATES_AND_EVOLUTION.md`, `STATUS.md`)**: Synced `w280_s2931_c01` (`76.965 g` 2.5D raster / `76.834 g` 3D solid, `G1 = 0.877646 COMPLETE`, `W_CAD = +1.743186`, `SPG = 3.5515 N/g`) across `README.md` (badges + `fig_readme_mass_vs_wave.png` hero curve through `w280` + `## Quick start`), `docs/WHITEPAPER.md` page 1 (`fig_why_this_matters.png` and `fig_w279_gate_margins.png` regenerated from committed `w280_s2931_c01` JSONs), the 18-subject × 18-evaluator CAD battery table (`173 pass, 61 fail, 81 blocked, 9 incomplete` = `324` cells), the Margin-vs-Mass Pareto progression table (including `w280_s2931_c02_light` at `76.96 g` and `w280_s2931_c03_seq121` at `77.14 g`), and `STATUS.md`.
- PR-0.32.8a (`a72fe2d`):
  - **Wave `w280` Stretch-Band (`74–77 g`) All-Gates & `W_CAD_v2` Winner (`w280_s2931_c01` at `76.965 g` raster / `76.834 g` 3D solid, `evidence/rs_v2/w280_ultrarunner/*`, `evidence/cad_battery/w280_s2931_c01.json`)**: First all-gates karambit inside the `74–77 g` stretch band (`mass_band_74_77 = true`) on both the 2.5D thickness raster (`76.965 g`, `-0.570 g` vs `w279_s2928_c01` at `77.535 g`, `-1.615 g` vs `w278_s2922_c01` at `78.580 g`, `-2.544 g` vs `w277_s2919_c01` at `79.509 g`, `-3.030 g` vs `w276_s2913_c01` at `79.995 g`, `-4.809 g` vs `w275_s2909_c01` at `81.774 g`, `-8.014 g` vs `w274_s2905_c01` at `84.979 g`) and 3D watertight STL / single-solid STEP (`76.8339 g`, `-0.580 g` vs `w279_s2928_c01` at `77.414 g`, `0.170%` 3D-vs-raster agreement). All 16 engineering, CAD, and karambit gates PASS (`promote_eligible = true`): `cert_lat_rob = 273.3 N` (`SF = 1.051`, `+5.1%` reserve, `SF_hiker = 1.519`), `strict_lat_rob_0p0625 = 273.4 N` (`SF = 1.0515`, **`+5.2%` reserve** vs `260 N` floor), `seq_ratio_cert = 1.202` (`PASS >= 1.20`), `twist_tip_sf = 1.123` (`+12.3%` reserve), `twist_body_sf = 3.178`, `twist_tipzone_sf_strict = 1.107` (`+10.7%` reserve), `ring_rob = 14,622.4 N` (`SF = 9.748`), `SPG = 3.5515 N/g` (record specific lateral capacity), 3D P2 solid-tip (`91,008` tets, `416,583` DOFs) `SF_nodal_max = 1.3866` (`SF_nodal_avg = 1.4721`, `dolfinx` v0.9.0 `0.0288%` agreement), `side.a4` Winkler+bore supported `seq = 3.889` and linear buckling $P_{\text{cr}} = 2034.76\text{ N}$ (`eigenvalue = 7.826`), `W_CAD = +1.743186`, and locked `G1 = 0.877646 COMPLETE`. Also committed Wave `w280` Pareto peer genomes `w280_s2931_c02_light_genome.csv` (`76.962 g` / `76.96 g`, `strict_lat = 273.3 N`, `seq_ratio = 1.205`) and `w280_s2931_c03_seq121_genome.csv` (`77.139 g` / `77.14 g`, `strict_lat = 274.4 N`, `seq_ratio = 1.208`) in `evidence/rs_v2/w280_ultrarunner/`.

## v0.32.7 — 2026-10-16 (v0.32.7 patch release: JSON-driven w279_s2928_c01 record badges, short single-page README with w20..w279 mass-vs-wave hero figure, and White Paper page-1 Why-This-Matters & 16-gate margin figures)
- PR-0.32.7a:
  - **JSON-Driven Record Badges & Short Single-Page `README.md` (`ops/ci/gen_readme.py`, `ops/ci/tests/test_gen_readme.py`, `README.md`)**: Replaced stale Wave-27 `84.80 g` / `G1 0.9804` badges in `README.md` with badges generated directly from committed JSONs (`evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_cad_eval.json` and `evidence/cad_battery/w279_s2928_c01.json`) for the current all-gates record `w279_s2928_c01` (`77.535 g (77.414 g 3D)`, `G1 = 0.873753`, `W_CAD = +1.7212 (16/16 gates)`), guarded by `test_record_badges_generated_from_committed_json`. Streamlined `README.md` into a concise single-page overview (`What it is`, current version and record, `fig_readme_mass_vs_wave.png` hero plot, `Install`, `Quick start`, links to docs, and market weight comparison moved to the bottom), with all long multi-wave/battery tables preserved in `docs/ENGINEERING_GATES_AND_EVOLUTION.md` and `docs/WHITEPAPER.md`.
  - **Manifest-Verified `w20`–`w279` Hero & 16-Gate Margin Figures on `docs/WHITEPAPER.md` Page 1 (`ops/render/whitepaper_hero_figs.py`, `docs/wave_reports/wave20_25_winners.json`, `ops/ci/gen_whitepaper.py`, `ops/ci/tests/test_gen_whitepaper.py`, `ops/ci/figures_required.txt`)**: Added `ops/render/whitepaper_hero_figs.py` and `docs/wave_reports/wave20_25_winners.json` (extracted verbatim from historical `wave20_D1_index.json`..`wave25_D1_index.json` commits) to deterministically render and manifest-verify `docs/figures/whitepaper/fig_readme_mass_vs_wave.png`, `docs/figures/whitepaper/fig_why_this_matters.png` (`w20`–`w279` mass vs. wave curve + 77-knife commercial census comparison), and `docs/figures/whitepaper/fig_w279_gate_margins.png` (all 16 `W_CAD_v2` engineering gates and margin vs. threshold for `w279_s2928_c01`), replacing the legacy `g3` turntable/3D hero figures on page 1 of `docs/WHITEPAPER.md`.

## v0.32.6 — 2026-10-16 (v0.32.6 patch release: Wave w279 high-reserve sub-78 g winner w279_s2928_c01 at 77.535 g / 77.414 g 3D & Pareto peers, 17-subject CAD battery, and JSON-traceable doc verification)
- PR-0.32.6b:
  - **17-Subject × 18-Evaluator CAD Battery & Wave `w279` Margin-vs-Mass Pareto Sync (`README.md`, `ops/ci/gen_readme.py`, `ops/ci/tests/test_gen_readme.py`, `docs/ENGINEERING_GATES_AND_EVOLUTION.md`, `docs/WHITEPAPER.md`, `STATUS.md`)**: Synced `w279_s2928_c01` (`77.535 g` raster / `77.414 g` 3D solid, `G1 = 0.873753 COMPLETE`, `W_CAD = +1.72123`) across the 17-subject × 18-evaluator CAD battery table (`165 pass, 57 fail, 78 blocked, 6 incomplete` = `306` cells), the JSON-traceable Margin-vs-Mass Pareto progression table, `README.md` `## Quick start`, `docs/ENGINEERING_GATES_AND_EVOLUTION.md` (Tables V-1, V-2, VI-1, VI-2), `docs/WHITEPAPER.md`, and `STATUS.md`.
  - **Post-Release `v0.32.5` Fresh-Clone Check & JSON-Traceable Doc Critique Pass (`3ee5f56`)**: Verified `v0.32.5` `## Install` + `## Quick start` end-to-end in a fresh `/tmp` clone and updated `ops/ci/gen_readme.py` and `ops/ci/tests/test_gen_readme.py` (`test_every_battery_and_pareto_number_traces_to_committed_json`) so every cell in the CAD battery and Margin-vs-Mass Pareto tables traces directly to committed evidence JSONs.
- PR-0.32.6a (`5690eac`, `a1d1910`):
  - **Wave `w279` All-Gates & `W_CAD_v2` Winner (`w279_s2928_c01` at `77.535 g` raster / `77.414 g` 3D solid, `evidence/rs_v2/w279_ultrarunner/*`, `evidence/cad_battery/w279_s2928_c01.json`)**: Strictly below `78.580 g` on both the 2.5D thickness raster (`77.535 g`, `-1.045 g` vs `w278_s2922_c01` at `78.580 g`, `-1.974 g` vs `w277_s2919_c01` at `79.509 g`, `-2.460 g` vs `w276_s2913_c01` at `79.995 g`, `-4.239 g` vs `w275_s2909_c01` at `81.774 g`, `-7.444 g` vs `w274_s2905_c01` at `84.979 g`) and 3D watertight STL / single-solid STEP (`77.4142 g`, `-1.183 g` vs `w278_s2922_c01` at `78.597 g`, `0.156%` 3D-vs-raster agreement). All 16 engineering, CAD, and karambit gates PASS with stronger lateral and twist reserves: `cert_lat_rob = 274.3 N` (`SF = 1.055`, `+5.5%` reserve), `strict_lat_rob_0p0625 = 274.4 N` (`SF = 1.0554`, **`+5.5%` reserve** vs `260 N`, up from `273.2 N / +5.1%` on `w278_s2922_c01`), `seq_ratio_cert = 1.202` (`PASS >= 1.20`), `twist_tip_sf = 1.123` (`+12.3%` reserve), `twist_body_sf = 3.192`, `twist_tipzone_sf_strict = 1.107` (**`+10.7%` reserve**, up from `1.098` on `w278_s2922_c01`), `ring_rob = 14,570.9 N` (`SF = 9.714`), `SPG = 3.5376 N/g`, 3D P2 solid-tip (`91,008` tets, `416,583` DOFs) `SF_nodal_max = 1.3866` (`SF_nodal_avg = 1.4721`, `dolfinx` v0.9.0 `0.0288%` agreement), `side.a4` Winkler+bore supported `seq = 3.878` and linear buckling $P_{\text{cr}} = 2427.9\text{ N}$ (`eigenvalue = 9.338`), `W_CAD = +1.72123`, and locked `G1 = 0.873753 COMPLETE`. Also committed Wave `w279` Pareto peer genomes `w279_s2928_c02_seq121_genome.csv` (`77.700 g`) and `w279_s2928_c03_lat275_genome.csv` (`78.086 g`) in `evidence/rs_v2/w279_ultrarunner/` and hardened `test_pid_cancellation` in `ops/ci/tests/test_ci_bounded.py` against empty-pidfile race (`a1d1910`).

## v0.32.5 — 2026-10-16 (v0.32.5 patch release: Wave w278 high-reserve sub-79 g winner w278_s2922_c01 at 78.580 g / 78.597 g 3D & Pareto peers down to 77.877 g, 16-subject CAD battery, and doc critique-proofing)
- PR-0.32.5b:
  - **16-Subject × 18-Evaluator CAD Battery & Wave `w278` Margin-vs-Mass Pareto Sync (`README.md`, `ops/ci/gen_readme.py`, `ops/ci/tests/test_gen_readme.py`, `docs/ENGINEERING_GATES_AND_EVOLUTION.md`, `docs/WHITEPAPER.md`, `STATUS.md`)**: Synced `w278_s2922_c01` (`78.580 g` raster / `78.597 g` 3D solid, `G1 = 0.861822 COMPLETE`, `W_CAD = +1.702131`) and Wave `w278` Pareto peers (`w278_s2922_c02_light` at `77.877 g` raster / `77.85 g` 3D solid and `w278_s2922_c03_seq124` at `79.042 g`, `seq_ratio = 1.240`) across the 16-subject × 18-evaluator CAD battery table, Margin-vs-Mass Pareto progression table, `README.md` `## Quick start`, `docs/ENGINEERING_GATES_AND_EVOLUTION.md` (Tables V-1, V-2, VI-1, VI-2), `docs/WHITEPAPER.md`, and `STATUS.md`.
  - **Post-Release Fresh-Clone Check & Doc Critique-Proofing (`987f9e0`)**: Verified `v0.32.4` `## Install` + `## Quick start` end-to-end in a fresh `/tmp` clone and hardened `docs/WHITEPAPER.md` (§VI `fat_sf`) and `docs/ENGINEERING_GATES_AND_EVOLUTION.md` (§V progression & §VII portable CLI flags).
- PR-0.32.5a (`0694146`):
  - **Wave `w278` All-Gates & `W_CAD_v2` Winner (`w278_s2922_c01` at `78.580 g` raster / `78.597 g` 3D solid, `evidence/rs_v2/w278_ultrarunner/*`, `evidence/cad_battery/w278_s2922_c01.json`)**: Strictly dominates `w277_s2919_c01` (`79.509 g`) across every mass and structural reserve metric: `78.580 g` 2.5D raster (`-0.929 g` vs `w277_s2919_c01`, `-1.415 g` vs `w276_s2913_c01`, `-3.194 g` vs `w275_s2909_c01`, `-6.399 g` vs `w274_s2905_c01`) and `78.5971 g` 3D watertight STL / single-solid STEP (`0.022%` 3D-vs-raster agreement). All 16 engineering, CAD, and karambit gates PASS: `cert_lat_rob = 273.4 N` (`SF = 1.052`, `+5.2%` reserve), `strict_lat_rob_0p0625 = 273.2 N` (`SF = 1.0508`, **`+5.1%` reserve** — achieving the `>= +5%` reserve target vs `260 N`), `seq_ratio_cert = 1.223` (`+1.9%` reserve vs `1.20`), `twist_tip_sf = 1.098`, `twist_body_sf = 3.094`, `twist_tipzone_sf_strict = 1.098` (`+9.8%` reserve), `ring_rob = 15,028.0 N` (`SF = 10.019`), `SPG = 3.4797 N/g`, 3D P2 solid-tip (`91,008` tets, `416,583` DOFs) `SF_nodal_max = 1.3819` (`SF_nodal_avg = 1.4574`, `dolfinx` v0.9.0 `0.0072%` agreement), `side.a4` Winkler+bore supported `seq = 3.569` and linear buckling $P_{\text{cr}} = 3051.3\text{ N}$ (`eigenvalue = 11.736`), `W_CAD = +1.702131`, and locked `G1 = 0.861822 COMPLETE`.
  - **Wave `w278` Margin-vs-Mass Pareto Peers (`evidence/rs_v2/w278_ultrarunner/`)**: `w278_s2922_c02_light_genome.csv` (`77.877 g` raster / `77.85 g` 3D solid, `-1.632 g` vs `w277_s2919_c01`, all strict plate gates PASS: `cert_lat = 272.8 N`, `strict_lat = 270.0 N / SF 1.038 (+3.8%)`, `seq_ratio = 1.220 (+1.7%)`, `twist_strict = 1.046 (+4.6%)`, `SPG = 3.5023 N/g`) and `w278_s2922_c03_seq124_genome.csv` (`79.042 g` raster, all strict plate gates PASS: `cert_lat = 271.4 N`, `strict_lat = 270.7 N / SF 1.041 (+4.1%)`, `seq_ratio = 1.240 (+3.3%)`, `twist_strict = 1.100 (+10.0%)`).

## v0.32.4 — 2026-10-15 (v0.32.4 patch release: Wave w277 margin-tuned sub-80 g winners w277_s2919_c01 at 79.509 g & w277_s2920_c01 at 79.026 g, 15-subject CAD battery + Margin-vs-Mass Pareto table, and CI/tempdir/doc fixes)
- PR-0.32.4b (`4b583e0`, `3d2f679`, `69bd5ec`, `246a4f9`):
  - **15-Subject × 18-Evaluator CAD Battery & Margin-vs-Mass Pareto Table (`README.md`, `ops/ci/gen_readme.py`, `docs/ENGINEERING_GATES_AND_EVOLUTION.md`, `docs/WHITEPAPER.md`, `STATUS.md`, `ops/ci/gen_status.py`)**: Added `w277_s2919_c01` and the generated Margin-vs-Mass Pareto table (`g3` → `w271c` → `w274` → `w275` → `w276` → `w277_s2919_c01` / `w277_s2920_c01`), fixed `ops/ci/gen_status.py` to skip empty `## Unreleased` headings and report the lightest all-gates genome, aligned `docs/WHITEPAPER.md` (§I, §II-E, §IV-A, §V, §VI) with `v0.32` `W_CAD_v2` evidence, and made `README.md` `## Install` PEP 668 compatible.
  - **CI & Tempdir Hygiene Fixes (`ops/ci/side_a4.py`, `ops/ci/evaluate_cad_candidate.py`, `crates/evolve/tests/replay.rs`, `crates/evolve/src/lib.rs`, `ops/ci/tests/test_gen_whitepaper.py`)**: Normalized `side_a4.py` and `evaluate_cad_candidate.py` to resolve `'evidence/...'` genome paths relative to repo `ROOT` across runners (`xbabe2`/`xbabe3`), and added RAII `Drop` / `atexit` tempdir cleanup guards for `/tmp/jarambit_replay_<pid>` and `/tmp/jarambit_s04_src_<pid>`.
- PR-0.32.4a (`aef341e`):
  - **Wave `w277` All-Gates & `W_CAD_v2` Winner (`w277_s2919_c01` at `79.509 g` raster / `79.656 g` 3D solid, `evidence/rs_v2/w277_ultrarunner/*`, `evidence/cad_battery/w277_s2919_c01.json`)**: First all-gates candidate strictly `< 80.00 g` on both the 2.5D thickness raster (`79.509 g`, `-0.486 g` vs `w276_s2913_c01` at `79.995 g`, `-2.265 g` vs `w275_s2909_c01` at `81.774 g`, `-5.470 g` vs `w274_s2905_c01` at `84.979 g`) and 3D watertight STL / single-solid STEP (`79.656 g`, `0.185%` 3D-vs-raster agreement, vs `80.050 g` on `w276_s2913_c01`). All 16 engineering & karambit gates PASS, including default hard tip-only twist (`twist_tip_sf = 1.148 PASS`), `cert_lat_rob = 273.2 N` (`SF = 1.051`), `strict_lat_rob_0p0625 = 270.3 N` (`SF = 1.040`, `+4.0%` reserve), `SPG = 3.4361 N/g` (highest SPG of all certified all-gates designs), `twist_body_sf = 3.967`, `twist_tipzone_sf_strict = 1.095` (`+9.5%` reserve), 3D P2 solid-tip `SF_nodal_max = 1.3393` (`SF_nodal_avg = 1.4305`, `dolfinx` v0.9.0 `0.022%` agreement), `seq_ratio_cert = 1.218` (`+1.5%` reserve), `side.a4` Winkler+bore supported `seq = 3.400` and linear buckling $P_{\text{cr}} = 1948.9\text{ N}$ (`eigenvalue = 7.496`), `W_CAD = +1.660363`, and locked `G1 = 0.834785 COMPLETE`.
  - **Wave `w277` Ultralight Pareto Frontier Peer (`w277_s2920_c01` at `79.026 g` raster / `79.031 g` 3D solid, `evidence/rs_v2/w277_ultrarunner/w277_s2920_c01_genome.csv`)**: Reaches `79.026 g` (`-0.969 g` vs `w276_s2913_c01`, `-5.953 g` vs `w274_s2905_c01`) on `t = 5.8 mm` stock with all strict 2D/3D plate gates passing (`cert_lat_rob = 266.3 N`, `strict_lat = 262.3 N / SF 1.009`, `SPG = 3.3692 N/g`, `seq_ratio = 1.202`, `twist_strict = 1.088`, `G1 = 0.8246`).

## v0.32.3 — 2026-10-15 (v0.32.3 patch release: sub-80 g all-gates candidate w276_s2913_c01 at 79.995 g, 14-subject CAD battery table, and version-check CI recursion-test fix)
- PR-0.32.3a (`ae209d0`): Wave `w276` sub-80 g all-gates evolved winner `w276_s2913_c01` (`79.995 g` — breaks the `80.00 g` target barrier, `1.78 g` lighter than `w275_s2909_c01` at `81.774 g` and `4.99 g` lighter than `w274_s2905_c01` at `84.98 g`, all 16 engineering & karambit gates passing, `strict_lat = 272.0 N`, `SPG = 3.435 N/g`, `twist_strict = 1.095`, 3D P2 solid-tip `SF = 1.3393` with `dolfinx` v0.9.0 `0.022%` agreement, `seq_ratio = 1.213`, `buckling = 9.010`, `W_CAD = +1.638074`, locked `G1 = 0.832260 COMPLETE`) synced into `README.md` (14-subject × 18-evaluator CAD battery table and `Quick start`), `ops/ci/gen_readme.py`, `ops/ci/tests/test_gen_readme.py`, `docs/ENGINEERING_GATES_AND_EVOLUTION.md`, `docs/WHITEPAPER.md`, and `STATUS.md`. Also patched `test_jarambit_cad_battery_env_does_not_recurse` in `ops/ci/tests/test_cad_battery.py` so `jarambit_bin.is_file()` evaluates to `True` inside the mocked test context when `target/release/jarambit` is not built on disk in the `version-check` CI job.

## v0.32.2 — 2026-10-15 (v0.32.2 patch release: w275_s2909_c01 sub-82 g all-gates candidate, 13-subject CAD battery table, JARAMBIT_CAD_BATTERY=1 recursion guard, and fresh-clone Quick start verification)
- PR-0.32.2b (`f326b91`): Fixed `JARAMBIT_CAD_BATTERY=1` re-entry recursion between `crescent-d1`/`crescent-evolve` and `ops/ci/cad_battery.py` (`crates/cli/src/crescent.rs`, `ops/ci/cad_battery.py`, `ops/ci/evaluate_cad_candidate.py`: clears `JARAMBIT_CAD_BATTERY`, sets `JARAMBIT_CAD_BATTERY_DEPTH=1`, reuses existing D1/plate JSON for `side.a4` when present, writes non-catalog battery work to `out_dir/_battery_work`, and adds regression tests in `crescent.rs` and `ops/ci/tests/test_cad_battery.py`). Verified fresh-clone `## Install` and `## Quick start` end-to-end in `/tmp/jarambit_fresh_clone_check` and cleaned up all temporary clone/quickstart folders.
- PR-0.32.2a (`32f2d73`, `ef9ac83`): Wave `w275` sub-82 g all-gates evolved winner `w275_s2909_c01` (`81.774 g` — `3.21 g` lighter than `w274_s2905_c01` at `84.98 g`, all 16 engineering & karambit gates passing, `strict_lat = 273.3 N`, `twist_strict = 1.062`, 3D P2 solid-tip `SF = 1.2967` with `dolfinx` v0.9.0 `0.023%` agreement, `seq_ratio = 1.206`, `buckling = 13.123`, `W_CAD = +1.558875`, locked `G1 = 0.801544 COMPLETE`) synced into `README.md` (13-subject × 18-evaluator CAD battery table and `Quick start`), `ops/ci/gen_readme.py`, `ops/ci/tests/test_gen_readme.py`, `docs/ENGINEERING_GATES_AND_EVOLUTION.md`, `docs/WHITEPAPER.md`, and `STATUS.md`.

## v0.32.1 — 2026-10-15 (v0.32.1 patch release: W_CAD_v2 ultra-runner calibration + w274_s2905_c01 all-gates candidate, README showcase, Install & Quick start, 12-subject × 18-evaluator CAD battery summary, and workspace cleanup)
- PR-0.32.1b: `README.md`, `ops/ci/gen_readme.py`, and `ops/ci/tests/test_gen_readme.py` upgraded to showcase the 3 manifest-verified engineering-gates & evolutionary figures (`docs/figures/engineering_gates/fig1..fig3_*.png`), a generated 12-subject × 18-evaluator CAD battery summary table (`evidence/cad_battery/*.json`, `side/*_a4.json`, `dolfinx_cross_check.json` including `w274_s2905_c01`), a generated 178-group (`356`-control) `T00`–`T19` scientific assurance catalog summary (`docs/v1/science/tests/TEST_CASE_CATALOG.json`), a complete 11-crate workspace table (`holder`, `evidence`), and dedicated `## Install` and `## Quick start` sections. Cleaned verified-unused scratch directories and added `COMMS.md` multi-agent coordination log.
- PR-0.32.1a (`db948fa`): Utah ultra-runner karambit `W_CAD_v2` calibration (`ops/ci/evaluate_cad_candidate.py`), all-gates search calibration (`crates/crescent/src/{fem.rs,qd.rs}`, `crates/cli/src/crescent.rs`, `crates/crescent/examples/plate_gci_probe.rs`), and `w274_s2905_c01` (`evidence/rs_v2/w274_calibrated/*`, `evidence/cad_battery/w274_s2905_c01.json`: `84.98 g`, `seq_ratio = 1.488`, 3D solid tip `SF = 1.304`, `W_CAD = +1.463446`, `G1 = 0.770058 COMPLETE`).

## v0.32.0 — 2026-10-15 (v0.32 milestone: 18-evaluator CAD battery, 178-group T00–T19 science suite, w272/w273 solid/GCI, and engineering gates paper — docs/v1/jarambit_v1_roadmap.json)
- PR-0.32.0m: Comprehensive engineering gates & evolutionary dynamics paper (`docs/ENGINEERING_GATES_AND_EVOLUTION.md`) and deterministic figure generator (`ops/render/engineering_gates_paper_figs.py` → `docs/figures/engineering_gates/fig{1,2,3}_*.png` + `.manifest.json`, registered in `ops/ci/figures_required.txt`). Covers all 18 CAD battery evaluators, geometric/red-team/holder/aesthetic (`C.4`/`C.5`/`C.6`) gates, multi-tier CST+DKT plate FEM, 3D `TET10` solid submodel + `DOLFINx` cross-check, Roache 3-mesh GCI, Side Reports `A1`–`A4` (compliant bore-pin + Winkler grip foundation + linear plate buckling $\lambda_{\text{cr}}$), `RS-karambit-v2` + `L7` Goodman fatigue + Sharpness Gate `S`, the 178-group (`356`-control) `T00`–`T19` scientific assurance catalog, and multi-wave evolutionary progression (`w1` $\rightarrow$ `w273`).
- PR-0.32.0l: End-to-end 18-case CAD battery (`ops/ci/cad_battery.py`, `evidence/cad_battery/*.json`) wired into the normal CAD $\rightarrow$ fitness scoring path (`jarambit crescent --evaluate-genome <csv>` and `JARAMBIT_CAD_BATTERY=1` in `--evolve`). Evaluates all 7 evolved genomes (`g3`, `w271b_11772`, `w271b_11860`, `w271c_11522`, `w272_s2902_c01`, `w273_s2904_c01`, `w273_s2903_c04`) and 4 native B-rep CAD subjects (`c10a`, `c93`, `c92`, `c10c`) across `build.*`, `geom.*`, `plate.*`, `side.a1..a4`, `solid.tip`, and `gate.rs_v2` alongside locked global fitness $G_1$.
- PR-0.32.0k: 3D STEP/STL quadratic-tetrahedron (`TET10`) tip submodel (`fem_solid/step_solid.py`), 7-genome + `c10a` `DOLFINx v0.9.0` vs `scikit-fem` cross-check (`fem_solid/cross_check.py`, `evidence/cad_battery/dolfinx_cross_check.json`, relative error $\le 0.05\%$), 3-mesh plate GCI (`plate_gci_probe.rs`, `evidence/cad_battery/plate_gci/*.json`), and Side Report `A4` compliant bore-pin + Winkler grip pad + linear buckling solver (`ops/ci/side_a4.py`, `evidence/cad_battery/side/*_a4.json`).
- PR-0.32.0j: 178-group (`356`-control) `T00`–`T19` scientific assurance catalog (`assurance/testing/policy.json`, `ops/ci/science_suite.py`, `fem_solid/tests/test_field_semantics.py`, `fem_solid/tests/test_representation_invariants.py`, `research/science_v1/test_t08_t18_catalog.py`, `crates/crescent/src/invariance.rs`) enforced in CI (`version-check.yml`).
- PR-0.32.0h: render lanes integrated on main — CAD ingest + region labels (Codex), Blender 4.2 worker / finishes / rigs / colour pipeline (Claude), and the R4 dark arc-key turntable rig (emitters camera-invisible, direct + floor-mirror out-of-frame check, softened floor reflection). Hero pinned to SWITCHBACK Rev C.9.3 (C.9.2 still accepted). Source identity is a single registry in `plan.py` (`KNOWN_SOURCES`, keyed by sha256): ingest pins file names to their hashes and takes the manifest model name from the `--cad` file's verified hash; package/QA use the same registry, so no revision is hardcoded downstream. Legacy stills rigs (studio softbox, dark strip) are marked `allow_emitter_in_frame`. The smoke workflow runs pytest in the pinned `venv-cad` interpreter (OCP), failing closed if it is missing. No hero renders are included. Tests cover the registry, model-from-hash and the arc-key out-of-frame check.
- PR-0.32.0g: render farm fixes for the six PR #45 review findings. (1) `render --execute` refuses to dispatch unless the bundle source sha256, a non-empty NPZ digest, the actual bundle-adjacent `mesh.npz` the worker loads and the regions file all hash to what the plan was resolved from. (2) Resume is bound to the plan: each shot/profile out_dir carries a `.plan_key` stamp; on a plan change every file of the old plan (PNGs, EXRs, records) is moved to `stale_<key>/` (an unstamped legacy dir goes to `stale_unbound/`); PNG markers record the plan; and a record in any of the three formats (`frame_index`, `index`, `frame.index`) that names a different plan or frame is rerendered. (3) `render --execute` launches at most one shard per host; the rest are reported `queued`. (4) Q12 fails closed without a compatible rerun image pair. (5) Q7 normalises high-pass noise by the flat-body luminance. (6) GIF concat lists use absolute paths. Tests `ops/render/marketing/tests/test_review45_fixes.py`.
- PR-0.32.0f: `designs/switchback/C9.3/` — SWITCHBACK Rev C.9.3 rc3, the current release candidate (hiking-karambit 94d9f4d): B-rep STEP + STL, drawings 00–09 regenerated in-repo by `drawings/draw_c93.py`, FEM plates W70–W77, W76/W77 data, SHA256SUMS. All C.9-study V1–V8 pass with ≥ 5 % margin (governor V3 1.597; V6 1.632/1.697 = 8.8 %; V7 1.613; 66.773 g). Still RELEASE CANDIDATE, not design of record, and not an RS-karambit-v2 verdict. C.9.2 marked previous RC. Test `ops/ci/tests/test_switchback_c93.py`.
- PR-0.32.0e: `designs/switchback/C9.2/` — SWITCHBACK Rev C.9.2 rc2 from the hiking-karambit study (owner-approved public disclosure): B-rep STEP + reference STL, drawings 00–09 generated from the STEP, FEM plates W70–W75.1, SHA256SUMS (machinist pack zip = release asset). Status is RELEASE CANDIDATE, not design of record: its V1–V8 values are the C.9 human-max gate set, not an RS-karambit-v2 verdict from this repo (loads differ), and the V6 lateral-pry margin (1.9 %) is below the 5 % promotion rule. README pointer + test `ops/ci/tests/test_switchback_c92.py` (hashes, status wording).
- PR-0.32.0d: PATENT_DRAFT upgrade. Numbered drawing set generated by code (`ops/render/patent/figures.py` → `docs/patent/figures/`, manifest with sha256, CI `--verify`): FIG. 1 perspective, FIGS. 2–7 front/rear/top/bottom/left/right orthographic line-art, FIG. 8 front elevation with utility-style reference numerals 10/12/20/30/32/40/50 and leader lines (40 anchored at the actual tip point, 50 inside the grind-law taper); every view is fitted from its projected bounds. Detailed description keyed to the numerals; new claims 15 (ring flare, bore length ≥ 10 mm) and 16 (requirement-set-scoped reporting); status scoped honestly (archived v0.27 RS-karambit-v1 screen group vs current inherited-screen FAIL on the holder envelope; RS-karambit-v2 not met). Test `ops/ci/tests/test_patent_figures.py`.
- PR-0.32.0c: WHITEPAPER upgrade (IEEE style). New §II-E methods v0.28–v0.31 (evidence layer, solid FEM + GCI, faer, RS-karambit-v2) with a generated evidence table; §III-H key components (ring 10, grip/spine 20, blade 30/32, tip 40), each with a 3D CAD view + plan FEM colormap + metrics (`ops/render/components.py` → `docs/figures/components/`); §IV-A honest RS-karambit-v2 status table (0 of 4 named designs meet every hard gate); retained body prose reconciled (seq hard, mass soft, faceted STEP not a sewn B-rep, faer/skyline rejection scope); components.py rejects an RS-v2 record whose genome differs from the D1 design and reports the strict tip-sub / solid tip gate values; generated status line under the abstract; IEEE reference list [1]–[6]. Blocks generated by `ops/ci/gen_whitepaper.py` (CI `--check`, also verifies component figure/input sha256). Test `ops/ci/tests/test_gen_whitepaper.py`.
- PR-0.32.0b2 (review of #39): `rs-v2-eval` now observes every inherited RS-karambit-v1 gate: strict 0.0625 mm² hunter lateral and strict tip-submodel twist (twist floor body AND tip-sub), and the solid tip SF from validated evidence bound to the exact genome (`evidence/fem_solid/gci/bindings.json`: label → GCI file + the genome it was run on; the evaluated genome must be identical); a gate without evidence is reported `unverified` and can never make `all_hard` true. Empty/invalid `--genomes`, unreadable genomes and unwritable outputs now exit 1. README table shows the strict/solid columns and unverified gates. Release note fixed: g3_tip+00_t60_tt3 meets the certification-tier tip-zone twist (SF ≥ 1). Tests `rs_v2_tests` (missing/failed gate never passes, mismatched solid evidence ignored, empty/bad specs fail).
- PR-0.32.0b: `jarambit rs-v2-eval --genomes label=path,... --out evidence/rs_v2` evaluates named genomes against every RS-karambit-v2 hard gate (screen/plate, seq ≥ 1.2, human-max loads, fatigue, ring-back, finger bearing) and writes `evidence/rs_v2/<label>.json` (+ genomes in `evidence/rs_v2/genomes/`). README gains a generated RS-v2 verdict table for g3_tip+00_t60_tt3, w271b_11772, w271b_11860, w271c_11522: **0 of 4 pass every hard gate** (all pass ring-back; all fail seq and human-max twist).
- PR-0.32.0a: `ops/render/turntable.py` renders a 360° 1920×1080 lossy web GIF (headless numpy + Pillow software renderer, ffmpeg palettegen/paletteuse with sierra2_4a dithering; colour = D1 certified envelope SF; FEM mode requires `--d1`, `--color x` is plain shading; one camera fit over all rotation angles) with a manifest (inputs, parameters, output sha256). `docs/media/g3_tip+00_t60_tt3_turntable.gif` (72 frames, 3.7 MB) is embedded at the top of README via `gen_readme.py`. CI `ops/ci/verify_media.py` (sha256 + 15 MB budget + input drift). Test `ops/ci/tests/test_render_turntable.py`.
- PR-0.31.6 (issue #35): README data blocks are generated by `ops/ci/gen_readme.py` from committed evidence (docs/VERSION, RS-karambit-v1/v2, crescent constants, D1 record + G1, grind law, verify_0.30.2, GCI, t1_t2.json, bench JSONs, issue32 drift); CI step `gen_readme.py --check`. v0.31.0 badge/release link, seq HARD, current crescent objective, 3D CAD breakdown, FEM / RS-v2 / calibration / speed tables; 3D render captioned as a faceted STL preview (STEP not validated), wave27 FEM plate labelled historical. Test `ops/ci/tests/test_gen_readme.py`.
- PR-0.31.5 (issue #32): faer now rejects partially constrained mechanisms with the skyline pivot criterion (pivot <= 1e-13·max|diag K| → None, via LDL regularization sentinel; the old all-ones-RHS probe missed rotations orthogonal to it). Regression `both_backends_reject_partially_constrained_rotation` (Plate + T6, 1 clamped node, 3 meshes). Cert checksum drift vs skyline stays 2.6e-8 max per design / 8.7e-9 sum (refinement does not help: double-precision floor), above the 1e-9 bar, so the DEFAULT solver is skyline again and faer is opt-in (`JARAMBIT_SOLVER=faer`). Evidence `evidence/bench/issue32_drift.json`.

## v0.31.0 — 2026-10-12 (v0.31 milestone: plate-tier speed + calibration + RS-karambit-v2 — docs/v1/jarambit_v1_roadmap.json)
- PR-0.31.4 (owner decisions): strong SOFT mass penalty restored in crescent G on top of #29's ln reward: `mass_over_penalty` = 0.4·(1−exp(−30·((m−80)/80)²)) above the 80 g target (smooth, no cliff; 90 g loses 0.13, 95 g 0.23 vs bonuses ≤ 0.07). Mass is still never a gate. RS-karambit-v2 `ringback_n` 3000 N labelled PROVISIONAL with rationale (DAF 2 × 1500 N ring pull; no impact data yet). Roadmap PR-0.33.4: ring flare / 3D ring lip in the genotype so finger-bearing width (bore length along the finger axis) can reach ≥ 10 mm. COORD: one agent owns PR sequencing. Test: `strong_soft_mass_penalty_above_80g`.
- PR-0.31.3: RS-karambit-v2 published (`requirements/RS-karambit-v2.toml`; mass SOFT per Jepson, overriding the roadmap's "mass<=80 hard"; seq >= 1.2 HARD). New structural load cases in `fem::certify` / `run_b`, all as first-yield capacities: human-max twist 16.2 N·m, tip in-plane 1000 N (worst of chord-normal / chord-axial), spine 3427 N (distributed, toward the edge), L6 ring-back impact static equivalent (DAF 2 x 1500 N = 3000 N, grip seated); ring finger-bearing width >= 10 mm and bore edge radius >= 1.5 mm (`fem::ring_bearing`, `rs_v2_human_max_pass`); L7 fatigue, Marin + modified Goodman, lateral 0..180 N (`fem::fatigue`, `rs_v2_fatigue_pass`). No tissue-damage objectives (test `no_tissue_objectives_in_rs_v2`). The extra cases reuse the existing factors (extra RHS only). T1->T2 calibration on 40 fixed_1k designs (`jarambit calib-t1t2`, `docs/calibration/t1_t2.md`): sigma lateral 6.5 %, ring 5.5 %, pry 3.3 % (pass); twist 10.8 % (fails the 7 % bar; twist stays gated on T2 only). On those 40 designs: human-max twist SF 0.045–0.075, tip in-plane 0.42–0.69, spine 0.30–0.50, ring-back 1.49–2.12, fatigue 0.61–0.81, bearing width 4.5–5.3 mm, so 0/40 pass `human_max`. Meeting it needs new genotype features (ring bolster, thicker tip/spine rail); RS-v2 does not gate search yet. Tests: `rs_karambit_v2` (crescent), `ops/ci/tests/test_t1_t2_calibration.py`.
- PR-0.31.2-solver (roadmap PR-0.31.2): plate/T6 solver now uses faer 0.24 supernodal sparse Cholesky (AMD ordering) through one `Chol` factor that serves any number of RHS (`Plate::solve_many`; the crescent lateral/twist/couple cases share one triangular sweep). `JARAMBIT_SOLVER=skyline` keeps the RCM skyline reference. Mechanisms are still rejected (same 1e-13 pivot criterion, by a bounded-response probe). crescent `eval_batch_with` uses work-stealing instead of static striping. fixed_1k on xbabe3: T1c cert 26.3 -> 6.74 core-s/design (3.9x), T1 search 2.15 -> 2.03. Parity <= 1e-9 (`crates/physics/tests/solver_parity.rs`); workload cert checksum 8.7e-9 relative vs skyline.
- PR-0.31.1: frozen benchmark workload `bench/fixed_1k` (1000 genomes from wave16 D1 seeds + c6_repair pass genomes, mutated with `Rng(31)`; sha256-pinned) and timing CLI `jarambit bench-make` / `jarambit bench-fixed --tier search|cert`; `ops/bench/run.py` runs 3 reps per tier and fails on a checksum change. Baseline on xbabe3 (`docs/perf/baseline_0.31.1.md`): T1 search 2.15 core-s/design, T1c cert 26.3 core-s/design. Test: `ops/ci/tests/test_bench_workload.py`.
- PR-0.31.2: **cap-free crescent fitness** (Jepson directive 2026-10-12: no hard mass caps). JARAMBIT_MASS_CAP / qd::mass_cap / under_cap removed. Strength is a hard feasibility floor for every genome (aesthetic incl. C.6/C.7, hard gates incl. jimping, FEM floors 180/260/1500 N, seq >= 1.2, chord torsion 1.82 N·m body AND tip submodel SF >= 1); infeasible designs get graded negative penalties. Feasible: F = 1 + m(mass) + 0.03(1 - e^(-margin/0.1)) + 0.02 A + 0.02 min(nov,1), m = ln(84.8/mass) above 70 g, ln(84.8/70) + 0.25 ln(70/mass) below. qd::feasible() replaces under_cap in reports/certification ordering. Tests: crates/crescent/tests/fitness_capfree.rs.

## v0.30.0 — 2026-10-12 (v0.30 milestone: solid FEM in repo — docs/v1/jarambit_v1_roadmap.json)
- PR-0.30.3: 3-level mesh convergence + GCI for the solid tip submodel (`fem_solid/convergence.py`, `gci.py`, `gci_report.py`; evidence `evidence/fem_solid/gci/`; table `docs/calibration/solid_gci.md`). Levels (NSUB, NL) (0,2)→(1,4)→(2,8), up to 3.2 M dofs, solved with dolfinx on xbabe3. GCI_fine on the governing nodal-max σvm is 0.43–1.80 % for 5 designs (bar 3 %). The production setting (1,4) reads about 4 % high, i.e. conservative: wave27 provisional g3_tip+00_t60_tt3 has SF 1.534 at production and 1.600 [1.572, 1.630] at the fine level. Wave27.1 design 11772 converges to SF 0.834 [0.830, 0.838], below the floor. `tip_solid.py DUMP_ONLY` skips the skfem assembly.
- PR-0.30.2: reference solid solver. `fem_solid/mesh.py` (gmsh tet10: verification geometries, faceted STEP, faceted STL; the wave27 STEP is an unsewable shell, so the whole-part mesh reads the STL twin, giving 84.86 g vs 84.80 g), `fem_solid/solve_dolfinx.py` (dolfinx 0.9 P2 + PETSc CG/GAMG with rigid-body near-nullspace), `fem_solid/Dockerfile`. Verified on xbabe3: St-Venant torsion 0.25 %, Kirsch 3D 0.73 % (bar 1 %); dolfinx vs skfem on the identical g3 tip submodel 6.5e-5 relative (bar 2 %). `tip_solid.py DUMP_MESH` exports the submodel. Whole-part L1–L5 load cases are not done yet.
- PR-0.30.1c: `JARAMBIT_TIPTWIST_GATE=1` makes the twist floor (body AND 4–15 mm tip submodel SF >= 1 at 1.82 N·m) constraint-dominated in crescent fitness for every genome, not only P1. A floors-OK design that misses it ranks in (-1, -0.1), graded by body × tip SF, so mass cuts can no longer thin the tip rail/ribs. Wave27.1 round 2 showed why: 22 designs passed certification and none passed the tip twist check (best 0.77). On by default in `ops/multihost/wave27.env`. Test: `crates/crescent/tests/tiptwist_term.rs`.
- PR-0.30.1b: D1 -> solid pipeline fix. `crescent-d1 --genomes <bare path>` now names the design after the file stem; before, the full path became the name, so the plate JSON and grind-law file were written to non-existent directories and silently dropped. Write failures are now reported. `fem_solid/run_tip.py` refuses missing or misnamed inputs before solving. Tests: `design_entry_tests` (cli), `fem_solid/tests/test_run_tip_inputs.py`.
- PR-0.30.1: solid tip submodel imported as `fem_solid/` (`tip_solid.py`, `run_tip.py` with a run manifest: input/script sha256, library versions, n_dof, wall time; `requirements.txt` pins scikit-fem 12.0.2, which is byte-identical to the previously vendored copy, plus pyamg 5.3.0). The 24 wave27 solid JSONs and the g3_tip+00_t60_tt3 D1/grind-law fixture are committed under `evidence/wave27/`. Re-run on xbabe3: SF_solid_nodal_max 1.534 (reference 1.534), n_dof 422 955, 374 s wall at 4 threads. Fast evidence tests run in CI; the full re-run is opt-in (`FEM_SOLID_REGRESS=1`).
- PR-0.30.0c: CI moved from the shared `neverhuman-lan` pool (#20, which queued for capacity) to dedicated self-hosted runners `jarambit-xbabe2` / `jarambit-xbabe3` (labels `self-hosted, linux, x64, xbabe, jarambit`; systemd services; per-runner CARGO_HOME/RUSTUP_HOME so CI never touches the hosts' toolchains). Canonical repo: `neverhuman/jarambit`.
- PR-0.30.0b: `ops/multihost/` — `crescent_wave.sh` (one nice'd crescent-evolve per `host:threads:seed:mass_cap` spec over ssh, distinct seeds, soft mass cap, evidence lineage), `wave27.env` (wave27 search env), `collect_cert.py` (gathers certification.csv across hosts; ranks only designs passing all hard gates; mass soft). Early piece of the v0.35 multi-host runner. Test: `ops/ci/tests/test_collect_cert.py`.
- PR-0.30.0a: `ops/wave_reports/make_wave_report.py` no longer gates on mass (74–77 g band / < 80 g cap removed from OVERALL); mass is reported as a SOFT status (target < 80 g, stretch < 70 g, G penalty), matching PR-0.28.3b. Test: `ops/ci/tests/test_wave_report_mass_soft.py`.

## v0.29.0 — 2026-10-12 (v0.29 milestone: evidence layer + honest logs — docs/v1/jarambit_v1_roadmap.json)
- PR-0.29.1: new crate `jarambit-evidence`: `RunManifest` (canonical-JSON SHA-256), `Obs<T>` (value / not_evaluated / failed, never a silent default), scoped `Verdict`, `Event` enum, and an append-only log (`*.jevl`: one zstd frame per generation with length + FNV-1a header, fsync per generation). Wired into crescent `qd::run` / `run_islands` (new `run_with` / `run_islands_with` take an evaluator and a sink): every record carries `parents`, `op`, `op_version`; elites, migrations and per-generation counters are logged. `crescent-smoke` / `crescent-evolve` write `{out}/evidence.jevl`; `jarambit evidence-cat --log <path>` dumps it. Tests: 64 x 40 fixture = 2464 fresh evaluations / 2624 appearances; island lineage + migration; 100 random truncations and 100 real kill -9 trials all leave a valid prefix.
- PR-0.29.2: legacy `evolve` checkpoint is lossless and manifest-bound. Format `jarambit-ckpt 2` stores every f64 as IEEE-754 bits in hex, plus RNG state, the archive, the run manifest and its SHA-256, and an `end` marker. It is written with fsync and then renamed. Resume fails with an error in each of these cases: no checkpoint, a torn checkpoint, a legacy format-1 checkpoint, or any manifest change (pop, gens, mc, seed, islands, migrate, fem, fine, seeds, floors, calibration, search area, operator/tool version). `--fork` resumes across a manifest change on purpose and records `forked_from_manifest_sha256` / `forked_at_gen`. `Config.stop_after` simulates interruption. `checkpoint_best` still reads format 1. Tests in `crates/evolve/tests/replay.rs`: a run interrupted at gens 3/7/12 and then resumed gives a final checkpoint byte-identical to the uninterrupted run (3 seeds).
- PR-0.29.3: legacy CLI history/quarantine are streamed (appended + flushed each generation; an interrupted run keeps its log; resume/fork appends). `history.csv` schema: `terminal`, `feasible_mean_p05` (mean over `feasible()` only, empty when none; was a `fit > -100` filter that counted graded infeasible designs), `n_feasible`, `best_genotype_hash`; `pop_mean_p05` removed. `quarantine.csv` gains `genotype_hash` (`jarambit_evolve::genotype_hash`, 16 hex of SHA-256 over exact f64 bits). A terminal snapshot (`GenStats.terminal`) after the last generation always contains the returned best. Tests: `crates/evolve/tests/history_stream.rs`.
- PR-0.29.4: figures derived from observations — aesthetic badge from `aes_pass` (no hard-coded PASS), declared shared colour norms (von Mises 0..828 MPa, SF 0..4), hero caption says "faceted STL preview" unless STEP validated, G1 recomputed; every figure gets a sha256 manifest checked by `ops/ci/verify_figures.py` (hero STL committed under `evidence/wave27/`); mass is soft in plate titles.

## v0.28.0 — 2026-10-12 (v0.28 milestone: truthful status + typed metrics — docs/v1/jarambit_v1_roadmap.json)
- PR-0.28.1: G1 completeness contract. `metrics/global_metric.py` no longer maps missing/null/NaN/inf/bool/malformed/out-of-range inputs to 0: the record is `INCOMPLETE`, `global_G` returns `G=None`, `title_tag` prints `G INCOMPLETE` (never raises). `provisional` (G*) is now only "tip from a screen"; completeness is `terms['status']`. Arithmetic for complete records is unchanged (42 wave27 index records equal the v0.27.0 formula to 1e-9). Tests: `metrics/tests/` (wired into version-check CI). v1 plan docs added under `docs/v1/`.
- PR-0.28.2: Requirement sets. `requirements/RS-karambit-v1.toml` defines `screen` / `promotion` / `search` groups (constants pinned against `crates/crescent/src/fem.rs` by a Rust test). `ops/ci/rs_verdicts.py` generates scoped verdicts; `ops/ci/check_claims.py` (CI) rejects unscoped PASS claims, stale RS verdict blocks and a mis-stated default evaluator. Provisional design now reads: screen PASS · promotion FAIL (mass 84.80 g >= 80 g cap; outside 74-77 g band; seq 0.959 < 1.2). Doc fix: published G1 0.9805 → 0.9804 (recomputes to 0.98042).
- PR-0.28.3: serde-typed crescent records (`jarambit_crescent::record`). `all_evals.json`, plate `metrics` and report lines are emitted with serde_json: unique keys (the duplicate `pry20` is gone), no NaN/inf (null), proper string escaping. A record without a FEM result now carries `fem_status` = `not_evaluated` (rejected before FEM) / `failed` / `incomplete` / `complete` and **null** FEM metrics instead of `Fem::default()` zeros; CSV FEM cells are empty in that case. Plate `d1` objects use null instead of the `-1` sentinel. QD archive CSV header `cell_relief,cell_hausdorff` / `cell_mass` → `cell_removal,cell_mass_bin` (the second axis has been the 2 g mass bin since wave7). CI: clippy `disallowed_methods` bans `Option::unwrap_or_default` in the CLI; D1 smoke now also writes the plate JSON and checks it with `ops/ci/check_json_strict.py`. d1_smoke known-negative unchanged (w14c03 promote=false). Readers: `make_lattice_plate.py` / `make_d1_plate.py` tolerate null `lat_rob`.
- PR-0.28.3b: **mass is soft** (Jepson decision 2026-10-12). RS-karambit-v1 version 2: the promotion group drops `mass_band` / `mass_cap`; `mass_target_g = 80` and `mass_stretch_g = 70` are soft targets (strong G penalty, search ranking). `fem::promote_eligible` no longer takes mass (`MASS_CAP_HARD` → `MASS_TARGET_G` / `MASS_STRETCH_G`). `rs_verdicts.py` prints mass as `soft: … (G penalty)`. Provisional now reads: screen PASS · promotion FAIL (seq 0.959 < 1.2) · soft: mass 84.80 g over 80 g target. d1_smoke recompute drops mass; w14c03 is still promote=false (twist tip submodel). G2 mass-term notes are in docs/v1 spec (addendum). G1 is unchanged.
- PR-0.28.4: fem-verify theory gate fail-closed against `verify/expected.toml` (missing / duplicate / unexpected / non-finite / all-missing checks FAIL with a reason; previously a NaN error passed). Manifest parser + `theory_gate` in `jarambit_physics::verify`; tests `crates/physics/tests/fem_verify_failclosed.rs` (committed finite report still PASS). `knife_convergence.csv` column `err_vs_extrap` → `err_vs_two_finest_mean` (committed CSV header migrated; see docs/CALIBRATION.md).
- PR-0.28.5: pinned toolchain (`rust-toolchain.toml`, Rust 1.85.1) and SHA-pinned GitHub Actions; `STATUS.md` is now generated by `ops/ci/gen_status.py` and checked in CI (`--check`); the old root `STATUS.md` / `GAPS.md` are archived under `docs/history/` as historical; README crate table gains the `crescent` row (128-gene lane) next to the legacy 20-gene `genotype`. Milestone close: VERSION 0.28.0, WHITEPAPER header, tag `v0.28.0`.
- Policy note: the v0.28.0 tag is now cut at the close of the v0.28 roadmap milestone (evidence/reporting), superseding the wave-gated rule below.

## v0.27.0 — 2026-10-12 (tagged; wave27 major)
- Wave27 pack landed: tip outline × t_stock 5.8/6.0 × t_tip +0/+0.3 on g3_neckrail + a0_c02fit_ref (24 arms) plus b–e sub-arms. Plates `docs/wave27_diversity/plates/` (+ `plates_e/`), tables `w27_sf.md`/`w27e_sf.md`, solids pointers `docs/wave27_diversity/SOLIDS.md`.
- **Engineering provisional RETITLED → `g3_tip+00_t60_tt3`**: G1 0.9805, solid tip SF 1.534, SPG 4.147, tip×SPG 6.362, 84.80 g, ring 25.46 mm, HARD PASS. Previous `g3_tip+00_t60_tt0` (g3_neckrail, 6.340 @ 84.59 g) is history. **Not promoted to manufacturing.**
- WHITEPAPER v0.27.0: wave27 callout filled (3D CAD `wave27_g3_tip+00_t60_tt3_STEP_3D.png`, FEM plate `wave27_g3_tip+00_t60_tt3_W27HARDPASS_SPG4p15_wgt84p8g_D1_FEM_metrics.png`, G1 table).
- Pareto: no ≤ 82 g @ tip×SPG ≥ 6 HARD PASS on tip arms; mass king remains a0_c02fit_ref class (~81.4 g / ~5.58). seq soft.
- Gate for v0.28.0: G1 > 0.9805 (hard PASS) OR ≤ 82 g @ tip×SPG ≥ 6 hard PASS.

## 0.27.0-dev — 2026-10-12 (superseded by v0.27.0)
- Repo reorganisation: `TECHNOLOGY_MAP.md`, role dirs (`genotype/ phenotype/ fem/ monte_carlo/ evolve/ metrics/ cli/`) with READMEs pointing into unchanged `crates/`; README per crate; `ops/README.md`, `docs/README.md`.
- GLOBAL metric G1 locked (`docs/GLOBAL_METRIC.md`, `metrics/global_metric.py`); wired into `ops/wave_figures/make_d1_plate.py` plate titles (G + mass g + version) and index JSON.
- WHITEPAPER restructured into technology callouts with CAD + FEM figures + metrics + version (waves 20–25 filled; 26/27 placeholders).
- Versioning: `docs/VERSION`, this file, `docs/WAVE_LOG.md`; CI `version-check` workflow.
- No solver/hot-path changes. Nothing promoted.

## Planned: v0.27.0 — when the wave27 pack lands (solids complete)
Checklist: set `docs/VERSION` to `0.27.0`; WHITEPAPER header `Version 0.27.0 — <date>`; fill wave26/27 placeholders; add entry here; `git tag -a v0.27.0 -m "wave27 pack"`; push tag (CI version-check verifies VERSION == tag == paper header).

## v0.1.0-g3neckrail — 2026-10-11 (existing tag)
Frozen g3_neckrail paper on branch `release/v0.1.0` (`docs/WHITEPAPER_g3_neckrail.md`). Provisional; not promoted.

## Retro-assigned (no tags): wave N ↔ 0.N.0 for waves 1–26 — see `docs/WAVE_LOG.md`.
