# `JR-EI-P5-T01`: Reusable Scientific Identity, Compact Archive Schemas, and README/Publication Narrative Parity Specification

- **Task ID:** `JR-EI-P5-T01` (`jarambit:JR-EI-P5-T01@r2:sha256:4434cfb66301840b449eb74a154d42428eeb98131980a98a2bc6dac7fab10364`)
- **Directive Authority:** `SC-DEC-183K` §4.2 (`2026-10-19T18:13:00Z`) & Controlling Review `CODEX_EI_AND_JOPE_REVIEW_20261019T1807Z.md`
- **Assigned Disjoint Worker:** `1409298.agy_jarambit_paper:0`
- **Plan Sections Covered:** §§12.3, 12.4, 12.7 (Canonical `JR-EI-P5-T01` Compact Schema & Scientific Identity Contract) and §6 (README & Publication Narrative Parity Specification)

---

## 1. Reuse of `crates/evidence/src/identity.rs` (`SubjectRef`, `ScientificIdentity`, `ContentRef`)

All compact archive manifests, segment envelopes, and publication projections MUST reuse the existing canonical wire and identity primitives in `crates/evidence/src/identity.rs` (`WIRE = "jarambit.science.canonical.v1"`) rather than introducing a competing scientific-identity hash:

1. **`ContentRef { id, revision, sha256 }`:**
   - Validates non-empty NFC-normalized `id` ($\le 512$ chars, no control chars), non-empty `revision`, and 64-hex lowercase `sha256`.
2. **`SubjectRef`:**
   - `SubjectRef::Genome { representation: ContentRef, bits: Vec<String> }`: requires non-empty 16-hex-character IEEE-754 binary64 bit strings (`{:016x}`) and rejects nonfinite floats (`NaN`, `±Inf`) while preserving `-0.0` (`8000000000000000`).
   - `SubjectRef::NativeCad { artifact: ContentRef, geometry_state: ContentRef, format: CadFormat, unit: LengthUnit }`: binds exported STEP/STL geometry and explicit length unit (`Millimetre | Metre`).
   - `SubjectRef::Specimen { id: String, passport: ContentRef }`: binds physical or controlled-probe specimens.
3. **`ScientificIdentity` (`schema: 2`):**
   - Binds `subject`, `requirements`, `source`, `build`, `cargo_lock`, `config`, `evaluator`, `scenario`, `objective`, `descriptor`, `material` (`MaterialState::Identified | Unknown`), and `dependencies`.
   - `RuntimeMetadata` (`host`, `threads`, operational tags) is stored alongside operational records and NEVER participates in `ScientificIdentity::sha256()`.

---

## 2. Three-Hash Separation Contract (`scientific` vs. `transport` vs. `source`)

When importing historical or native evolutionary records (such as `w282_ultrarunner` or the 10 completed generation-800 `all_evals.csv` runs under `JR-EI-P1-T08`) and when writing prospective compact journals (`JR-EI-P5-T02`..`T04`), every genome-bearing record separates three distinct cryptographic digests:

| Digest Field | Domain / Wire | Exact Meaning & Invariant |
|---|---|---|
| `scientific_subject_sha256` | `jarambit.science.canonical.v1` | SHA-256 of `canonical_wire(SubjectRef::Genome { representation, bits })`. Invariant across zstd compression, chunking, or file paths. Never fabricated from decimal-rounded CSV strings as if 53-bit IEEE-754 mantissas had been recovered; rounded historical imports bind `representation` to an explicit decimal-import schema revision. |
| `transport_envelope_sha256` | `jarambit.genome.transport.v1` | SHA-256 of the deterministic uncompressed genome transport envelope: `transport_domain_separator \| genome_schema_version \| decoder_identity \| material_schema_and_value \| gene_count \| ordered_f64_little_endian_bits`. Used for content deduplication of identical genomes under the same decoder/material schema without dropping distinct `Birth` events. |
| `source_serialization_sha256` | Raw producer bytes | SHA-256 of the exact original byte slice in the producer file (e.g., raw line bytes in `all_evals.csv` or JSON object bytes in `all_evals.json`), binding compact chunks back to the immutable host files on `xbabe1`..`xbabe4`. |

---

## 3. Compact Segment, Snapshot, and Custody Contracts (§12.3, §12.5, §12.7 & `SC-DEC-183K`)

The machine-readable schema bundle in `JR_EI_P5_T01_scientific_identity_and_compact_schemas.v1.json` defines `run.v1.schema.json`, `event_and_genome.v1.schema.json`, `segment.v1.schema.json`, and `snapshot.v1.schema.json` with the following mandatory rules:

1. **Birth-Before-Evaluation & Distinct Event Kinds:**
   - `BirthRecord`, `ObservationRecord`, `MembershipRecord`, `SelectionRecord`, `MigrationRecord`, `LifecycleRecord`, and `GenerationSummaryRecord` are distinct typed records.
   - `MembershipRecord` keys occurrences by `(run_uuid, segment_index, frame_index, frame_event_index)` with `(generation, island, slot_index, producer_birth_id)` as attributes, preserving the 9 duplicate `(generation, island, individual)` elite slot occurrences in `w282_ultrarunner` and keeping unrecorded refreshed elite scores explicitly `unrecorded_in_source_elite_event`.
   - `GenerationSummaryRecord` retains headerless row 0 (`headerless_row_zero_retained: true`) and explicitly models `NaN` tokens in `min_legal_mass_g` without coercing `NaN` to `0.0`.
2. **Bounded Immutable Segments (`segment.v1.schema.json`):**
   - Target ~`1 MiB` compressed zstd segment size; hard ceiling `8 MiB` (`8,388,608` bytes) compressed and `64 MiB` (`67,108,864` bytes) uncompressed to prevent decompression bombs or silent truncation.
3. **Snapshot Scope, 24 STL Separation, and Custody Lineage (`snapshot.v1.schema.json`):**
   - **Four Distinct Publication Acknowledgment States:** `local_sealed`, `transferred_and_verified`, `committed_on_archive_branch`, and `reviewed_merged_accession`.
   - **`JR-N1` / `SC-DEC-183K` Scope Rule:** `publication_acknowledgment_state = local_sealed` on `T07` (`295` files / `190,807,895` bytes, index `dd69650f66a0c6cf29cbfcfc80e1302529717ba7e31a9af6b0b825d13970c49b`) seals only the finite summary/JEV1/QD file selection (`completion_scope = finite_summary_jev1_qd_selection_only_no_all_evals`). It MUST NOT be cited as complete native genome/score Git reproducibility until `JR-EI-P1-T08` streams, chunks, and losslessly compresses the authoritative `all_evals` tables across all 10 completed generation-800 runs. Furthermore, `xbabe1/c01`'s independently counted `195,456` retained IDs (generations 0–800) is never extrapolated to the other 9 runs.
   - **Separation of the 24 Retained STL Files:** `retained_source_evidence_assets_separation` explicitly separates the 24 STL files (including ~`2.6 MB` `*_fit.stl` certification/render assets) retained as source evidence from the routine compact score/gene Git payload, without deleting or rewriting their manifests.
   - **`JR-N2` Custody Lineage Preservation:** All distinct V1 and V2 handoff variants remain bound in `custody_lineage`:
     - First-observed `269,650`-byte V1 original (`target/archive-red-team/first-handoff/JARAMBIT_EVOLUTION_ARCHIVE_HANDOFF_20261019.json`): `081f34e5659ce518ce45ef6ed731840609b221aa3c2f6b1bfbc24322fceacf98`
     - Second-observed `288,343`-byte V1 original (`target/archive-red-team/second-observed-v1/JARAMBIT_EVOLUTION_ARCHIVE_HANDOFF_20261019.json`): `62b67373d45f5a1174561591e6ba49508f3ec1d3bce0bed6e2adad2f04807eae`
     - Current remote V1 handoff file: `010a91d7e4865cd661d14b56c7c1bf50feccfcc8080c5e65c3040628881bf311`
     - Frozen V2 handoff (`664,314` bytes): `b1f884622aaffc9929069424685ed0aaf8b950470b254141336c193f1ec2529e`
     - Additive V2 erratum (`34,300` bytes): `a3cefc5ce7f841aa6e7b1edfd8503c988dfa1e4e04754b22ea7babc4bcc1ff8a`

---

## 4. Shared README & Publication Narrative Parity Specification (§6 & `publication.v2.json`)

To guarantee that `README.md`, `docs/paper/main.tex`, the Supplement, and the `web/` population viewer never drift on counts, subject roles, score profiles, or archive scope, all surfaces consume a single Rust-projected contract (`docs/paper/publication.v2.json`):

### 4.1 Required 8-Part `README.md` Structural Hierarchy (Target for `JR-EI-P7-T02`)

1. **Title & Operational Definition (`Jarambit — Engineered Intelligence`):**
   - Matches the white-paper title (*Engineered Intelligence: The Evolution of Physical Design — Executable Genomes, Recorded Inheritance, and Full Population Evidence*) and gives a one-sentence operational statement of executable representation, recorded search, and traceable numerical evidence.
2. **Shared Full-Width Hero Master & Caption:**
   - Displays the web derivative of the exact same full-width horizontal white-studio hero used on Page 1 of the paper (`cert_pipeline_r01_p14_c02`, ring left, handle horizontal, blade curving upward right), with a concise caption stating its assessed-export role (`not qualified`, `champion=null`), integer public ID, and available-field percentile composite recipe (`4` retained plate maps at `0.25 mm^2`, common support `10,334 / 25,209` triangles).
3. **Three Core Contribution Bullets:**
   - (a) *Executable Representation:* 128 continuous genes + material state decoded deterministically to B-rep and FEM meshes (`jarambit.science.canonical.v1`).
   - (b) *Complete Recorded Population Histories:* Reconstructed 17-generation genealogy (`1,984` births, `2,176` island-slot appearances), `3,649` newer birth events across `57` files (`2,259` one-based screen joins, `1,390` unscreened), and `17,672` native summary rows across `11` staged histories (plus `10` completed generation-800 host runs undergoing lossless full-population accession under `JR-EI-P1-T08`).
   - (c) *Traceable Numerical Observations:* Eleven observation families separating search ranking from discretization verification and physical qualification.
4. **Full-Width Population Preview & Data Accession Links:**
   - Static full-population preview (`population-births.svg` / `population-latest.svg`) linked directly to the read-only population explorer and immutable archive manifests.
5. **Compact Source-Derived Objective & Single-Profile Progress Preview:**
   - Explains the distinction between historical piecewise native fitness $f$ and the 20-term weighted reporting profile $F \in [0,100]$ (30% mass/compactness, 25% form/comfort proxies, 15% exported geometry, 30% response/verification), showing one compatible stage/profile per trace without splicing Stage-1 and Stage-2 curves.
6. **Primary Navigation Bar:**
   - Direct links to: White Paper (`docs/paper/main.tex` / built PDF), Supplement & 44-Entry Crosswalk (`docs/paper/supplement/`), Test Atlas (`docs/tests/README.md`), Reproduce Figures (`docs/paper/README.md`), and Evolution Archives (`data/evolution-archives/`).
7. **Install, Build, and Read-Only Exploration Instructions:**
   - Exact reproducible commands for read-only inspection without launching unrequested evolution or heavy FEM runs; standalone reporting crate validation uses `rtk cargo test --locked --manifest-path crates/paper-artifacts/Cargo.toml -- --test-threads=2` (executed in `P8`).
8. **Scoped Historical Records, Market Census Cohort Scope, and Corrections:**
   - Moves exhaustive test-catalog count tables below the principal narrative.
   - Explicitly scopes `<!-- MARKET:CENSUS:BEGIN -->...<!-- MARKET:CENSUS:END -->` and `best_so_far_26_gates.json` leaders as leaders *within their committed genome-pipeline cohort* (`w319_r02_p10_c03` at `74.21/100` in the 26-gate Stage-2 ledger; `0/23` `all_hard` passes in the historical battery), never as an unqualified physical champion or an imported hand-built CAD baseline (`SWITCHBACK C.10a` is excluded from evolved headline figures).

### 4.2 Single-Source Parity Projection Table (`publication.v2.json` -> All Consumers)

| Shared Fact / Entity | Canonical Source Key | `README.md` (`overview.md`) | `docs/paper/main.tex` (`metrics.tex`) | `web/` Studio & Population Viewer |
|---|---|---|---|---|
| Title & Subtitle | `publication.v2.json::title` | `Jarambit — Engineered Intelligence` | `Engineered Intelligence: The Evolution of Physical Design` | Header & document metadata |
| Opening Subject | `cert_pipeline_r01_p14_c02` (`db5d8e32...`) | Illustrated assessed export (`not qualified`, `champion=null`) | Fig. 1 (`\ref{fig:cover}`), `not qualified` | Admitted studio subject with nested schema guard |
| Historical Genealogy (`w282`) | `1,984` births (`1,455` FEM complete, `529` not evaluated), `2,176` appearances | Population bullet & preview caption | `\BirthCount`, `\FemComplete`, `\FemNotEvaluated` | Cohort `w282_ultrarunner` timeline & inspector |
| Newer Births & Screens | `57` files, `24` dirs, `3,649` births, `2,259` screens (1-based join), `1,390` unscreened | Population bullet & cohort table | `\NewerBirthEventsCount`, `\NewerScreenRowsCount` | Cohort `newer_births_screens` with no-screen band |
| Native QD Histories (`T06`/`T07`/`T08`) | `11` staged files (`17,672` summary rows, max gen `708`); `10` gen-800 host runs (`T08` full `all_evals` accession) | Scoped archive table (no `c01` `195,456` extrapolation) | `\NativeSummaryRowsCount`, explicit `T07` vs `T08` scope | Cohort `native_qd_summaries` & `T08` chunk loader |
| Historical Battery (414 cells) | `23` subjects $\times$ `18` cols (`221` pass, `81` fail, `112` blocked, `fresh_this_lane: false`) | Linked via Supplement / Test Atlas | Supplement S1 (`\BatteryPass`, `\BatteryFail`, `\BatteryBlocked`) | Battery tab (`battery_records` nested-validated) |
| Catalog & Visual Coverage | `44` entries (`16` G16, `10` RSV2, `18` battery); `40/44` linked, `4` missing visuals, `4` `archive_required` fields | Linked to `docs/tests/visuals.md` | Section 6 (11 families) + Supplement S2/S3 | Visual coverage & explicit missing-field badges |
