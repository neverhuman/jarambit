# JR-EI-P5-T02: `README.md` / `publication.v2.json` 8-Part Narrative Parity Matrix & Lossless Genomes / All-Event Record Specification

- **Task ID:** `JR-EI-P5-T02`
- **Canonical Ledger Title:** `Implement lossless genomes and all-event records`
- **Admiralty Task ID:** `jarambit:JR-EI-P5-T02@r2:sha256:e08b8ccea5c6d49cc829c6ad8685fbdbad25c8e4a238e397843e10827135ba22`
- **Governing Directives:** `SC-DEC-183K`, `SC-DEC-183L`, `SC-DEC-183M`, and `SC-DEC-183N`
- **Validation Status:** `implemented_unvalidated` (`completed_draft_pending_primary_merge_and_p8_ci`; `CI_HOLD_UNTIL_P8 = true`; **NO git push to `origin`**)

---

## 1. Bound Primary Contracts and SHA-256 Digests

| Surface / Contract | Repository Path | Byte Count | SHA-256 Digest |
|---|---|---:|---|
| Root Narrative & Quick-Start | `README.md` | `22,172` | `758783957385170340f02f09e9ec7283e9a2500e1c95337d4e8d0a8c678d5b17` |
| Publication v2 Master Contract | `docs/paper/publication.v2.json` | `4,862` | `0259684e53e6bd08b225951349007668a9133915a3f1a6f33fd1f322f30f2213` |
| Persistent Integer Public IDs | `docs/paper/data/public_ids.v1.json` | `8,839` | `4a7d8d25ab06b85b7f6586663407fb5abde3b1aa50c0277a104046866ba57c82` |
| Selection Authority & Claims Audit | `docs/paper/data/selection_receipts.v1.json` | `6,340` | `9f8f5f05bef98304e37f3e90d02abc646d10b99d92d745c2f8f501e893f1d2e8` |
| Score Profiles & Display Loss | `docs/paper/data/score_profiles.v1.json` | `7,679` | `5d4403900023e9d602ae08c2ae7408e08699d4cd960e2fb660d5e5cc52d2e81d` |
| Historical Births Ledger (`Run 1`) | `docs/paper/generated/population/historical_births.json` | `6,496` | `d118223cbff9db451d54f8a9aac684f8365b6850a0374f8c72bfe8917094d68f` |
| Historical Membership Ledger (`Run 1`) | `docs/paper/generated/population/historical_membership.json` | `12,642` | `8274b8532e4df26295be9ff4f3f7ac40e111b796307939a438f0075db69a1180` |
| Newer Multi-Stage Events Ledger | `docs/paper/generated/population/newer_events.json` | `1,164` | `a25f15e39cbdd524a586bae942b2668dc7043d22f71c1b26b7465c15e8a26f59` |
| Latest Gen-800 Snapshots Ledger | `docs/paper/generated/population/latest_snapshots.json` | `5,138` | `daeea520ba8a37b01dab3ec443fc03605a0885eacb1d4d87c73dd75d11e3d36e` |
| Multi-Hop Inheritance & Genome Diffs | `docs/paper/generated/population/inheritance.json` | `20,249` | `337abc826a4032256db7700ba68abdee8aee51ab09817853ee6741a27d99f3df` |
| Gen-800 Population Accession Receipt | `docs/paper/JR_EI_P1_T08_LOSSLESS_GEN800_POPULATION_ACCESSION_RECEIPT_20261019.json` | `60,069` | `79ba4b6da9df8fda27dfa035f20187499cf032420a5b15e90ccfbaa06a1479a1` |
| Gen-800 Population Master Index (`210` files) | `/home/ubuntu/jarambit/artifacts/versioned-evolution-archives-gen800-population-v1/index.gen800_population.sha256.json` | `176,429` | `66c84ed4cdc943df885da4cdd72d25c0054195d9d8b9a01e1c35b6f3119c38cf` |

---

## 2. Eight-Part Narrative Parity Matrix (`README.md` ↔ `publication.v2.json` ↔ Manuscript Drafts)

1. **Part 1 — Category & Title Framing (`Engineered Intelligence`):**
   - `publication.v2.json`: `title = "Engineered Intelligence: The Evolution of Physical Design"`, `subtitle = "Executable Genomes, Recorded Inheritance, and Full Population Evidence"`, `category = "Engineered Intelligence"`.
   - `JR_EI_P3_T01_main.tex` & `JR_EI_P3_T01_01-introduction.tex`: Matches title/subtitle/category verbatim; enforces `JR-R4` reader-facing integer labels (`Run 1`–`Run 11`, `Individual 2000`, `Individual 1095`, `Individual 2001`, `Individual 2002`) with zero hostnames, slugs, operational paths, or checksums in main text.
2. **Part 2 — 129-Field Executable Genome & Decoder Contract:**
   - `128` numeric `f64` coordinates (`cp00..cp39` + `blade_drop_deg..k4`) + `1` categorical alloy column (`material`) = `129` genome-and-material fields (`146` total columns in `all_evals.csv` when preceded by `17` scalar evaluation columns; `header_sha256: 2f1469c4...`).
3. **Part 3 — Cohort 1 (`Run 1`, Historical 17-Gen Reconstructable Genealogy):**
   - `1,984` retained births across generations `0..16` (`1,455` FEM-complete, `529` FEM-not-evaluated, `446` search-legal, `1,965` distinct CSV gene serializations); `2,176` island-slot memberships (`4` islands × `32` slots × `17` gens, `192` elite reappearances, `12` ring migrations at generations `4, 8, 12`). Highest birth fitness: `Individual 1095` at generation `9` (`fit = 1.15727`, `strict_certification.cert_pass_all = false`).
4. **Part 4 — Cohort 2 (Intermediate Multi-Stage Screening):**
   - `3,649` birth events across `57` birth files in `24` run directories, joined via 1-based candidate index (`2,247` zero-based misjoins rejected) to `2,259` Stage-1 screen rows and `1,390` unscreened birth events retained in a dedicated unscreened observation band.
5. **Part 5 — Cohort 3 (`Run 2`–`Run 11`, 10-Run 800-Gen Complete Population Archive):**
   - `10` completed 4-island runs × `195,456` births per run = `1,954,560` total evaluated births (`754,236` search-legal = `38.59%`), `32,040` generation-by-island summary rows (`3,204` per run across `801` generations `0..800`), compressed losslessly from `3,811,896,749` raw CSV bytes into `494,843,141` zstd bytes across `200` segments (`210` indexed files including `10` run manifests).
   - *Cross-reference reconciliation item for `P7`/`P8` integration:* `public_ids.v1.json` assigns `Run 2`..`Run 11` to these 10 runs, whereas `latest_snapshots.json` currently labels them `Run 37`..`Run 46`.
6. **Part 6 — Score Profiles, Reversible Display Loss, and Controlled-Probe Erratum (`A01` / `SC-DEC-183M`):**
   - `cross_profile_splicing_allowed = false` across `historical_w282_native_qd_fit` (`L = -f`), `W_ULTRALIGHT_v1` (`L = 100 - F`, `Individual 2002` `F = 74.207299`, `L = 25.792701`), and `W_CAD_v2` (`L = -W_CAD`).
   - Controlled probe `Individual 2001` (`w283_p14_c01`): `W_CAD = -3.878704`, `G1 = 0.6239029999999999`, `all_engineering_gates_pass = false`. No `0.45 mm` thickness gate or `0.45` dimensionless fitness cutoff exists for `G1`.
7. **Part 7 — Illustrated Assessed Geometry (`Individual 2000`) & Multi-Model Verification:**
   - Editorially selected assessed export (`selection_authority = "human"`, `qualification_status = "assessed_not_qualified"`, `champion = null`, `qualified_count = 0`).
   - Retained $0.25\,\mathrm{mm}^2$ D1 plate mesh (`13,503` nodes, `25,209` triangles; `75` tip-excluded cells in `env_vm`, `14,875` seated-ring masked cells in `ring`), 3-level GCI study (`25,209`, `6,803`, `3,166` triangles; both GCI error estimates refused due to $p=-2.9003$ and oscillatory differences), and matched-mesh quadratic solid submodel (`91,008` tetrahedra, `416,583` DOFs; `skfem` vs `DOLFINx 0.9.0` safety factors `0.669` vs `0.6693` and `0.726` vs `0.7263`).
8. **Part 8 — 44-Entry Assessment Atlas, 414-Cell Historical Battery, & Read-Only Projection Boundary:**
   - `16` legacy `G16` predicates + `10` overlapping `RS-v2` groups + `18` historical CAD battery evaluators (`23` subjects × `18` evaluators = `414` cells: `221` pass, `81` fail, `112` blocked, including `92` shared software passes).
   - Read-only projection (`paper_artifacts`) never invokes live `export-web-data` or `score_cad_native`.

---

## 3. Canonical `JR-EI-P5-T02` Lossless Genomes & All-Event Record Specification

1. **Birth-Before-Evaluation Recording:**
   Every proposed candidate receives its monotonic run-local birth identifier and parent/operator lineage record prior to phenotype decoding or finite-element meshing, ensuring that rejected or unscreened births remain in the permanent population denominator.
2. **Exact 129-Column Genome + Material Preservation:**
   All 128 `f64` gene coordinates (`cp00..k4`) and the categorical `material` column (`Ti-6Al-4V` or `Ti-3Al-2.5V`) are preserved without precision loss in bounded zstd segments (`<= 8 MiB` compressed ceiling; max observed segment across all 200 gen-800 chunks is `2,627,104` B).
3. **Real Retained-Evidence Rust Reader Requirement (`JR-R1` / `SC-DEC-183N` §2.1):**
   Receipt metadata validators (`validate_gen800_population_accession`, `validate_newer_corpus_join`, `validate_historical_population_adapter`) validate receipt JSON invariants, while the retained-evidence Rust reader path verifies actual segment membership, compressed chunk SHA-256, decompressed payload SHA-256, 146-column / 129-gene+material widths, 195,456 unique local IDs per run, and `0..800` generation coverage.
