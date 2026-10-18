# Static gates and FEM invariant audit

status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)
qualified_count: 0

Task: `G16-12-RSV2-04-p23`. Static baseline: `56d9016`.

This is a strictly static source-text and SHA-256 receipt for four production files. No production candidate optimization, design comparison, mesh generation, finite-element solve or physical evaluation is run. No production source is edited. Qualification remains zero.

## Immutable source fingerprints

SHA-256 hashes exact file bytes. Bytes is the exact byte length; Lines is `len(text.splitlines())` after UTF-8 decoding. Each file was compared byte-for-byte with its baseline committed version before inspection. Original source snapshots are retained with the task receipt; the verifier checks current source bytes, sizes and line counts against this table without executing Rust code.

|Source path|SHA-256|Bytes|Lines|
|---|---|---|---|
|crates/crescent/src/gates.rs|0e364aab4d5d16d43d4e2c95c2e393707edb35b614f35d4b9eaf11c4303f28d7|5204|68|
|crates/crescent/src/fem.rs|9c62d764e8df8d02513b22fa8f2ce76da5bee4c612e81e9f2f58d43ad9f7fdca|36052|500|
|crates/crescent/src/certification.rs|eeefc8b1ecbf21df85aefb967e351ada4f0070f8b481959afef419379215c4ac|14755|364|
|crates/crescent/src/export.rs|575eef5f12aebc81a345ef2fe7f52a51ab73e6f7ed837eb1c5e556f2e81bec06|10067|172|

## Requested predicates versus observed implementation

The assignment's predicate list is treated as a claim to audit, not as evidence of the implementation. Several requested values or names differ from these files. None is changed to manufacture agreement. The following source paths and symbols are the static evidence; an absent identifier is absent only from this four-file scope, not necessarily from the entire repository.

|Assignment predicate|Observed source evidence|Audit finding|
|---|---|---|
|S1 <=0.20 mm|`fem::gate_s::APEX_MAX_MM=0.20`; certification `sharpness` requires complete measurements and `apex<=APEX_MAX_MM`|Matches this bound; missing measurement remains unavailable|
|S2 monotonic ==true|Certification `point_profile` includes strict adjacent `next>=previous` monotonicity in its combined pass condition|Matches; no monotonicity tolerance is added|
|S3 <=22.0 degrees|`fem::gate_s::HALF_ANGLE_MAX_DEG=25.0`; certification `sharpness` compares against that imported constant|Mismatch: audited source uses25, not22; it already does so at baseline|
|`lat_n>=358.0 N`|No `lat_n` identifier or358 literal in the four files; FEM stores `lat_rob` and compares it to imported `floors::HIKER_LATERAL_N` and `floors::HUNTER_LATERAL_N`|Not established as a local hard gate; do not relabel an imported floor as358|
|`twist_sf>=1.0`|`twist_floor_ok` requires both `body_sf>=1.0` and `tipzone_sf_strict>=1.0`; `twist_sf` itself is a computed reported ratio|Related unit-floor gating exists, but the compound check is not a direct single-field `twist_sf` comparison|
|`drop_sf>=1.0`|`drop_sf` does not occur in the four files|Not established within scope; no replacement metric invented|
|`seq_ratio>=1.0`|`fem::SEQ_TARGET=1.2`; `promote_eligible` requires its `seq` argument to satisfy `seq>=SEQ_TARGET`|Mismatch: explicit local target is1.2, not1.0|
|`seq_gci<=0.15`|`seq_gci` does not occur in the four files|Not established within scope; geometric tolerances are not a GCI criterion|
|`p2_sf>=1.50`|`p2_sf` does not occur in the four files|Not established within scope; unrelated1.5 constants are not this predicate|

The requested22-degree assertion cannot be confirmed from these files. The existing25-degree bound is not a relaxation introduced by this audit; both baseline and post-audit source bytes are identical. Likewise, this audit neither replaces the1.2 sequence target with1.0 nor adds missing predicates. Whether an external policy requires different bounds is outside this immutable-source audit.

## Exact local constants and predicate ownership

|Owner and symbol|Exact source value|Static use|
|---|---|---|
|`gates::LIGAMENT_MIN_MM`|`3.0`|Local geometry/manufacturability predicate|
|`gates::RING_WALL_MIN_MM`|`4.5`|Local geometry/manufacturability predicate|
|`gates::LAND_MIN_MM2`|`40.0`|Local contact-area predicate|
|`fem::gate_s::APEX_MAX_MM`|`0.20`|Imported by certification for measured S1|
|`fem::gate_s::HALF_ANGLE_MAX_DEG`|`25.0`|Imported by certification for measured S3|
|`fem::gate_s::TIP_T025_MAX_MM`|`0.30`|Required profile-thickness predicate|
|`fem::gate_s::TIP_T1_MAX_MM`|`0.60`|Required profile-thickness predicate|
|`fem::gate_s::TIP_T2_MAX_MM`|`1.00`|Required profile-thickness predicate|
|`fem::gate_s::TIP_TAPER_MIN_MM`|`5.0`|Profile crossing-distance lower-bound predicate|
|`fem::SEQ_TARGET`|`1.2`|Used by `promote_eligible`|
|`fem::twist_floor_ok`|Both arguments `>=1.0`|Compound local predicate, not a new field alias|

`gates.rs::check` returns geometry/manufacturability rejection strings and is not the owner of all structural and measured-surface decisions. `certification.rs` imports the S constants from `fem.rs`; attributing every requested threshold to `gates.rs` or defining a second numeric copy in this receipt would misstate ownership. Imported numerical floor definitions are outside the requested four-file scope; only their symbolic use is asserted here.

`point_profile` uses mesh-derived stations over its measured interval and checks every adjacent thickness without a waiver. Its combined pass also includes the three thickness limits and the taper lower-bound predicate. `sharpness` returns unavailable if either boundary has zero successful thickness samples; an individual missing hit marks completeness false. Its available result combines completeness, the measured maximum-thickness bound, and the measured maximum-angle bound. Geometric containment and station-deduplication tolerances do not amend these requirement comparisons.

`promote_eligible` combines its aesthetic, geometry and floor flags, an imported lateral-floor comparison, the local sequence target, and `twist_floor_ok`. This source-level Boolean expression is documented without invoking it on a candidate. `rs_v2_human_max_pass` has separate existing unit-factor predicates and an existing record-only flag; none of those branches is changed or reclassified by this audit.

## Export and evidence invariants

|Static invariant|Observed control flow|Limit of this audit|
|---|---|---|
|Shared thickness source|FEM element thickness sampling calls `export::thickness_at`; export construction also calls that function|Source call relationship only; no geometry evaluated|
|Measured export prerequisite|Certification `measure` retains unavailable fields if export fails, edge census differs from `(0,0,1)`, or signed volume is `<=0`|Literal guards are documented, not a universal mesh-validity proof|
|Export surface JSON|`surface_json` keeps selected original top-face triangles and full original vertices with schema `jarambit.export_surface.v1`|No resampling, renumbering or new surface is performed by this audit|
|Independent-observation comparison|`independent_pass` checks recognized metadata, mesh-string binding, finite positive paired values and computed discrepancies|No external solver execution or physical independence is inferred|
|No source relaxation|All four whole-file hashes and byte counts match the baseline committed bytes|Proves no threshold or branch change by this task, not all historical policy claims|

The four-file audit does not claim that the assignment's missing metric names are implemented elsewhere, that every gate is valid for every input, or that no historical revision ever changed a threshold. The supported invariant is narrower and exact: zero production edits and zero gate-threshold relaxation by this task, relative to the recorded baseline. No acceptance decision or qualified result is produced.

## Portable verification

The provenance index now contains13 ordered reports: V5 throughV14, the two earlier audit receipts, and this invariant receipt. Every row has exact SHA-256, UTF-8 line count, byte size, table-schema digest, IN_PROGRESS status and qualified_count0. Indexed Markdown, the index and the verifier contain no literal host-home prefix; the detector is constructed dynamically.

The existing unittest methods are extended with this report's digest/tampering cases, four-source fingerprint verification and source-corruption subtests. Relocated document fixtures include the four source files as inert byte copies. Existing source bindings, all37 archived JSON, and the exact legacy27-file/340-occurrence catalog remain unchanged. No Rust production tests or candidate evaluation are executed by the verification command.

```sh
nice -n 10 taskset -c 10,11 env CARGO_BUILD_JOBS=2 python3 -m unittest discover -s experiments/G16-12-RSV2-04 -p 'test_*.py' -v
git diff --check
```

Execution is pinned to CPUs10,11 with jobs2 and the requested niceness increment; inherited effective niceness is19. Logs and immutable before/after receipts are retained in the task receipt directory and handoff. Matching digests establish consistency relative to trusted repository anchors, not authorship or physical qualification.
