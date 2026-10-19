# Scientific assurance: complete frozen definition index

Generated from [TEST_CASE_CATALOG.json](../v1/science/tests/TEST_CASE_CATALOG.json). These are **178 definitions across 20 T00–T19 families**, with 356 required paired-control slots. Frozen status is historical; source presence is not an execution receipt. Expected receipt paths containing `<attempt>` are patterns, not evidence that a receipt exists. No execution-receipt join is asserted by this index.

[CAD atlas](README.md) · [Paper](../paper/README.md)

| Family | Frozen definitions |
|---|---:|
| [T00](#t00) | 11 |
| [T01](#t01) | 18 |
| [T02](#t02) | 12 |
| [T03](#t03) | 10 |
| [T04](#t04) | 12 |
| [T05](#t05) | 12 |
| [T06](#t06) | 10 |
| [T07](#t07) | 13 |
| [T08](#t08) | 6 |
| [T09](#t09) | 6 |
| [T10](#t10) | 5 |
| [T11](#t11) | 7 |
| [T12](#t12) | 5 |
| [T13](#t13) | 7 |
| [T14](#t14) | 9 |
| [T15](#t15) | 6 |
| [T16](#t16) | 6 |
| [T17](#t17) | 6 |
| [T18](#t18) | 7 |
| [T19](#t19) | 10 |

## T00

<a id="T00-01-inventory_delete"></a>

### T00-01-inventory_delete — inventory delete

**Challenge:** Remove one required receipt from otherwise complete inventory.

**Oracle:** Incomplete; deletion cannot strengthen coverage. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_contracts.py`. **Expected receipt pattern:** `.agent/science-s00/<attempt>/cases/T00-01-inventory_delete.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T00-02-inventory_duplicate"></a>

### T00-02-inventory_duplicate — inventory duplicate

**Challenge:** Duplicate a case ID or submit two receipts for one attempt.

**Oracle:** Error with duplicate identity. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_contracts.py`. **Expected receipt pattern:** `.agent/science-s00/<attempt>/cases/T00-02-inventory_duplicate.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T00-03-optional_skip"></a>

### T00-03-optional_skip — optional skip

**Challenge:** Unavailable optional branch with claim disabled vs required case unavailable.

**Oracle:** Explicit optional reason accepted; required incomplete. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_contracts.py`. **Expected receipt pattern:** `.agent/science-s00/<attempt>/cases/T00-03-optional_skip.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T00-04-strict_json"></a>

### T00-04-strict_json — strict json

**Challenge:** Duplicate key, bool numeric, NaN, infinity, or malformed status.

**Oracle:** Reject with structural reason; finite genuine zero accepted. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_contracts.py`. **Expected receipt pattern:** `.agent/science-s00/<attempt>/cases/T00-04-strict_json.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T00-05-receipt_tamper"></a>

### T00-05-receipt_tamper — receipt tamper

**Challenge:** Modify raw artifact bytes after receipt hashing.

**Oracle:** Integrity error; no claim support. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_contracts.py`. **Expected receipt pattern:** `.agent/science-s00/<attempt>/cases/T00-05-receipt_tamper.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T00-06-receipt_crash"></a>

### T00-06-receipt_crash — receipt crash

**Challenge:** Interrupt before atomic completion receipt publication.

**Oracle:** Incomplete attempt, not empty pass. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_contracts.py`. **Expected receipt pattern:** `.agent/science-s00/<attempt>/cases/T00-06-receipt_crash.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T00-07-schema_bounds"></a>

### T00-07-schema_bounds — schema bounds

**Challenge:** Unknown schema, oversized/deep payload, normalization-colliding IDs.

**Oracle:** Reject ambiguity/resource limit; bounded valid record accepted. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_contracts.py`. **Expected receipt pattern:** `.agent/science-s00/<attempt>/cases/T00-07-schema_bounds.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T00-08-controls_missing"></a>

### T00-08-controls_missing — controls missing

**Challenge:** Remove positive control or claimed killed-mutant proof.

**Oracle:** Incomplete test family. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_contracts.py`. **Expected receipt pattern:** `.agent/science-s00/<attempt>/cases/T00-08-controls_missing.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T00-09-finding_vs_execution"></a>

### T00-09-finding_vs_execution — finding vs execution

**Challenge:** Correctly executed negative/null scientific experiment.

**Oracle:** Execution passed; finding rejected/inconclusive remains distinct. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_contracts.py`. **Expected receipt pattern:** `.agent/science-s00/<attempt>/cases/T00-09-finding_vs_execution.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T00-10-claim_mutant"></a>

### T00-10-claim_mutant — claim mutant

**Challenge:** Drop inventory/hash or unknown-to-healthy guard in disposable mutation.

**Oracle:** Targeted control kills mutant. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_contracts.py`. **Expected receipt pattern:** `.agent/science-s00/<attempt>/cases/T00-10-claim_mutant.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T00-11-standard_scope"></a>

### T00-11-standard_scope — standard scope

**Challenge:** Out-of-scope coating test asserts named standard compliance.

**Oracle:** Reject scope/edition/method mismatch; applicable verified control accepted. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_contracts.py`. **Expected receipt pattern:** `.agent/science-s00/<attempt>/cases/T00-11-standard_scope.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.


## T01

<a id="T01-01-analytic_order"></a>

### T01-01-analytic_order — analytic order

**Challenge:** f_exact+C*h^p at known positive p and ordered levels.

**Oracle:** Independent textbook order/error estimate within stated tolerance. **Frozen status:** executed; fem_solid/tests/test_gci.py; version-check 38662560217.

**Allowed implementation location:** `fem_solid/tests/test_gci.py`. **Expected receipt pattern:** `.agent/science-s02/<attempt>/cases/T01-01-analytic_order.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T01-02-divergent_order"></a>

### T01-02-divergent_order — divergent order

**Challenge:** Archived (10,12,13,r=2) and an analytic divergent family.

**Oracle:** Nonpassing validity; negative GCI cannot qualify. **Frozen status:** executed; fem_solid/tests/test_gci.py; version-check 38662560217.

**Allowed implementation location:** `fem_solid/tests/test_gci.py`. **Expected receipt pattern:** `.agent/science-s02/<attempt>/cases/T01-02-divergent_order.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T01-03-zero_order"></a>

### T01-03-zero_order — zero order

**Challenge:** Sequence (10,11,12,r=2) produces observed p=0.

**Oracle:** Invalid/undetermined, no divide-by-zero or PASS. **Frozen status:** executed; fem_solid/tests/test_gci.py; version-check 38662560217.

**Allowed implementation location:** `fem_solid/tests/test_gci.py`. **Expected receipt pattern:** `.agent/science-s02/<attempt>/cases/T01-03-zero_order.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T01-04-plateau"></a>

### T01-04-plateau — plateau

**Challenge:** Equal coarse/mid with changed fine, including (10,10,100).

**Oracle:** Undetermined/invalid, never zero-GCI PASS. **Frozen status:** executed; fem_solid/tests/test_gci.py; version-check 38662560217.

**Allowed implementation location:** `fem_solid/tests/test_gci.py`. **Expected receipt pattern:** `.agent/science-s02/<attempt>/cases/T01-04-plateau.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T01-05-oscillatory"></a>

### T01-05-oscillatory — oscillatory

**Challenge:** Alternating levels and archived w271b_11772 record.

**Oracle:** Oscillatory/undetermined, not asymptotic PASS. **Frozen status:** executed; fem_solid/tests/test_gci.py; version-check 38662560217.

**Allowed implementation location:** `fem_solid/tests/test_gci.py`. **Expected receipt pattern:** `.agent/science-s02/<attempt>/cases/T01-05-oscillatory.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T01-06-exact_equal"></a>

### T01-06-exact_equal — exact equal

**Challenge:** Three equal values without independent convergence support.

**Oracle:** Undetermined, not automatic qualification. **Frozen status:** executed; fem_solid/tests/test_gci.py; version-check 38662560217.

**Allowed implementation location:** `fem_solid/tests/test_gci.py`. **Expected receipt pattern:** `.agent/science-s02/<attempt>/cases/T01-06-exact_equal.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T01-07-invalid_ratio"></a>

### T01-07-invalid_ratio — invalid ratio

**Challenge:** r<=1, zero/negative/nonfinite ratio, or nonfinite/boolean coarse/mid/fine quantity or observed-order input.

**Oracle:** Structured invalid input; no divide exception publication. **Frozen status:** executed; fem_solid/tests/test_gci.py; version-check 38662560217.

**Allowed implementation location:** `fem_solid/tests/test_gci.py`. **Expected receipt pattern:** `.agent/science-s02/<attempt>/cases/T01-07-invalid_ratio.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T01-08-zero_reference"></a>

### T01-08-zero_reference — zero reference

**Challenge:** Zero reference/fine normalization and legitimate zero observable.

**Oracle:** Explicit absolute-error/undetermined rule; no arbitrary ratio. **Frozen status:** executed; fem_solid/tests/test_gci.py; version-check 38662560217.

**Allowed implementation location:** `fem_solid/tests/test_gci.py`. **Expected receipt pattern:** `.agent/science-s02/<attempt>/cases/T01-08-zero_reference.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T01-09-precision_floor"></a>

### T01-09-precision_floor — precision floor

**Challenge:** Near-equal values and log/roundoff instability.

**Oracle:** Validity/uncertainty reason, no fabricated order. **Frozen status:** executed; fem_solid/tests/test_gci.py; version-check 38662560217.

**Allowed implementation location:** `fem_solid/tests/test_gci.py`. **Expected receipt pattern:** `.agent/science-s02/<attempt>/cases/T01-09-precision_floor.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T01-10-mesh_sequence"></a>

### T01-10-mesh_sequence — mesh sequence

**Challenge:** Wrong ordering, repeated mesh, anisotropic unqualified r.

**Oracle:** Invalid/unsupported refinement sequence. **Frozen status:** executed; fem_solid/tests/test_gci.py; version-check 38662560217.

**Allowed implementation location:** `fem_solid/tests/test_gci.py`. **Expected receipt pattern:** `.agent/science-s02/<attempt>/cases/T01-10-mesh_sequence.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T01-11-observable_identity"></a>

### T01-11-observable_identity — observable identity

**Challenge:** Different units, region, projection or metric among levels.

**Oracle:** Reject incompatible observable sequence. **Frozen status:** executed; fem_solid/tests/test_gci.py; version-check 38662560217.

**Allowed implementation location:** `fem_solid/tests/test_gci.py`. **Expected receipt pattern:** `.agent/science-s02/<attempt>/cases/T01-11-observable_identity.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T01-12-threshold_edge"></a>

### T01-12-threshold_edge — threshold edge

**Challenge:** Valid error immediately below/at/above frozen tolerance.

**Oracle:** Unrounded boundary policy matches independent comparison. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `fem_solid/tests/test_gci.py`. **Expected receipt pattern:** `.agent/science-s02/<attempt>/cases/T01-12-threshold_edge.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T01-13-negative_error_band"></a>

### T01-13-negative_error_band — negative error band

**Challenge:** Negative uncertainty/error band in otherwise plausible record.

**Oracle:** Invalid observation, never upper-threshold PASS. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `fem_solid/tests/test_gci.py`. **Expected receipt pattern:** `.agent/science-s02/<attempt>/cases/T01-13-negative_error_band.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T01-14-inventory_empty"></a>

### T01-14-inventory_empty — inventory empty

**Challenge:** Empty or mixed missing/invalid/valid design inventory.

**Oracle:** Incomplete unless all expected valid cases present. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `fem_solid/tests/test_gci.py`. **Expected receipt pattern:** `.agent/science-s02/<attempt>/cases/T01-14-inventory_empty.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T01-15-stale_noop"></a>

### T01-15-stale_noop — stale noop

**Challenge:** Preexisting output plus no-op, failed or nonzero solver invocation.

**Oracle:** No fresh execution accepted. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `fem_solid/tests/test_gci.py`. **Expected receipt pattern:** `.agent/science-s02/<attempt>/cases/T01-15-stale_noop.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T01-16-cache_identity"></a>

### T01-16-cache_identity — cache identity

**Challenge:** Change material, source, geometry or mesh key vs compatible reuse.

**Oracle:** Wrong key rejected; compatible hit labeled reuse. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `fem_solid/tests/test_gci.py`. **Expected receipt pattern:** `.agent/science-s02/<attempt>/cases/T01-16-cache_identity.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T01-17-partial_atomic"></a>

### T01-17-partial_atomic — partial atomic

**Challenge:** Missing completion/manifest, interrupted rename, disk-full, concurrent attempts.

**Oracle:** Partial attempts incomplete; attempts never overwrite each other. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `fem_solid/tests/test_gci.py`. **Expected receipt pattern:** `.agent/science-s02/<attempt>/cases/T01-17-partial_atomic.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T01-18-peak_singularity"></a>

### T01-18-peak_singularity — peak singularity

**Challenge:** Global norm converges but local peak changes with refinement.

**Oracle:** No local certification from global-only convergence. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `fem_solid/tests/test_gci.py`. **Expected receipt pattern:** `.agent/science-s02/<attempt>/cases/T01-18-peak_singularity.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.


## T02

<a id="T02-01-contradictory_block"></a>

### T02-01-contradictory_block — contradictory block

**Challenge:** Canonical FAIL followed by appended scoped PASS.

**Oracle:** Exact canonical/scoped integrity rejects contradiction. **Frozen status:** not_run; independent B/C/D runtime admission required.

**Allowed implementation location:** `ops/ci/tests/test_claim_integrity.py`. **Expected receipt pattern:** `.agent/science-s03/<attempt>/cases/T02-01-contradictory_block.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T02-02-missing_reopened"></a>

### T02-02-missing_reopened — missing reopened

**Challenge:** Delete V4 or reopen V3 approval.

**Oracle:** Current promotion incomplete; historical result readable. **Frozen status:** not_run; independent B/C/D runtime admission required.

**Allowed implementation location:** `ops/ci/tests/test_claim_integrity.py`. **Expected receipt pattern:** `.agent/science-s03/<attempt>/cases/T02-02-missing_reopened.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T02-03-wrong_candidate"></a>

### T02-03-wrong_candidate — wrong candidate

**Challenge:** C9.3 heading backed by C10a or another genome.

**Oracle:** Reject identity borrowing. **Frozen status:** not_run; independent B/C/D runtime admission required.

**Allowed implementation location:** `ops/ci/tests/test_claim_integrity.py`. **Expected receipt pattern:** `.agent/science-s03/<attempt>/cases/T02-03-wrong_candidate.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T02-04-requirement_content"></a>

### T02-04-requirement_content — requirement content

**Challenge:** Same RS-id but modified TOML bytes or hard/soft crosswalk.

**Oracle:** Hash mismatch or explicit new reviewed requirement version. **Frozen status:** not_run; independent B/C/D runtime admission required.

**Allowed implementation location:** `ops/ci/tests/test_claim_integrity.py`. **Expected receipt pattern:** `.agent/science-s03/<attempt>/cases/T02-04-requirement_content.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T02-05-wrong_physics"></a>

### T02-05-wrong_physics — wrong physics

**Challenge:** Wrong material/ROI/observable/mesh/geometry role.

**Oracle:** Bound claim incomplete. **Frozen status:** not_run; independent B/C/D runtime admission required.

**Allowed implementation location:** `ops/ci/tests/test_claim_integrity.py`. **Expected receipt pattern:** `.agent/science-s03/<attempt>/cases/T02-05-wrong_physics.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T02-06-revoked_review"></a>

### T02-06-revoked_review — revoked review

**Challenge:** Approval predates changed dependency or reopened record.

**Oracle:** Review invalidated for current claim. **Frozen status:** not_run; independent B/C/D runtime admission required.

**Allowed implementation location:** `ops/ci/tests/test_claim_integrity.py`. **Expected receipt pattern:** `.agent/science-s03/<attempt>/cases/T02-06-revoked_review.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T02-07-presentation_noise"></a>

### T02-07-presentation_noise — presentation noise

**Challenge:** Duplicate blocks, HTML/markdown/comment/token noise.

**Oracle:** No substring-based false PASS; canonical valid control survives. **Frozen status:** not_run; independent B/C/D runtime admission required.

**Allowed implementation location:** `ops/ci/tests/test_claim_integrity.py`. **Expected receipt pattern:** `.agent/science-s03/<attempt>/cases/T02-07-presentation_noise.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T02-08-nonfinite_scalar"></a>

### T02-08-nonfinite_scalar — nonfinite scalar

**Challenge:** Nonfinite, boolean, negative SF or invalid fraction.

**Oracle:** Typed invalid observation, no qualifying value. **Frozen status:** not_run; independent B/C/D runtime admission required.

**Allowed implementation location:** `ops/ci/tests/test_claim_integrity.py`. **Expected receipt pattern:** `.agent/science-s03/<attempt>/cases/T02-08-nonfinite_scalar.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T02-09-mass_scope"></a>

### T02-09-mass_scope — mass scope

**Challenge:** Soft/record mass target reported as hard certification.

**Oracle:** Reject scope upgrade. **Frozen status:** not_run; independent B/C/D runtime admission required.

**Allowed implementation location:** `ops/ci/tests/test_claim_integrity.py`. **Expected receipt pattern:** `.agent/science-s03/<attempt>/cases/T02-09-mass_scope.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T02-10-physical_missing"></a>

### T02-10-physical_missing — physical missing

**Challenge:** Simulation presented as physical validation.

**Oracle:** Reject evidence-class promotion. **Frozen status:** not_run; independent B/C/D runtime admission required.

**Allowed implementation location:** `ops/ci/tests/test_claim_integrity.py`. **Expected receipt pattern:** `.agent/science-s03/<attempt>/cases/T02-10-physical_missing.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T02-11-g1_delete"></a>

### T02-11-g1_delete — g1 delete

**Challenge:** Remove any G1 input from valid fixture, preserve genuine zero.

**Oracle:** Completeness cannot improve; zero remains measured. **Frozen status:** not_run; independent B/C/D runtime admission required.

**Allowed implementation location:** `ops/ci/tests/test_claim_integrity.py`. **Expected receipt pattern:** `.agent/science-s03/<attempt>/cases/T02-11-g1_delete.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T02-12-truthful_negative"></a>

### T02-12-truthful_negative — truthful negative

**Challenge:** Known-negative D1 reported correctly and historical adapter loaded.

**Oracle:** Integrity execution passes while qualification remains negative. **Frozen status:** not_run; independent B/C/D runtime admission required.

**Allowed implementation location:** `ops/ci/tests/test_claim_integrity.py`. **Expected receipt pattern:** `.agent/science-s03/<attempt>/cases/T02-12-truthful_negative.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.


## T03

<a id="T03-01-empty_count"></a>

### T03-01-empty_count — empty count

**Challenge:** n=0 then n=1 valid control.

**Oracle:** Zero returns typed error, never coerced to one. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/monte_carlo/tests/checked_reduction.rs`. **Expected receipt pattern:** `.agent/science-s04/<attempt>/cases/T03-01-empty_count.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T03-02-quantile_boundary"></a>

### T03-02-quantile_boundary — quantile boundary

**Challenge:** Small odd/even enumerated populations and convention boundaries.

**Oracle:** Exact independent rank convention. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/monte_carlo/tests/checked_reduction.rs`. **Expected receipt pattern:** `.agent/science-s04/<attempt>/cases/T03-02-quantile_boundary.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T03-03-finite_reduction"></a>

### T03-03-finite_reduction — finite reduction

**Challenge:** Cancellation, dynamic range and huge finite sum.

**Oracle:** Reference-compensated result or explicit overflow error. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/monte_carlo/tests/checked_reduction.rs`. **Expected receipt pattern:** `.agent/science-s04/<attempt>/cases/T03-03-finite_reduction.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T03-04-invalid_sample"></a>

### T03-04-invalid_sample — invalid sample

**Challenge:** NaN/inf in one column, missing result, invalid/duplicate sample ID.

**Oracle:** Counts reconcile; no silent survivor-conditioned estimate. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/monte_carlo/tests/checked_reduction.rs`. **Expected receipt pattern:** `.agent/science-s04/<attempt>/cases/T03-04-invalid_sample.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T03-05-worker_panic"></a>

### T03-05-worker_panic — worker panic

**Challenge:** Owned worker panics partway through batch.

**Oracle:** Typed incomplete/error with partial counts. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/monte_carlo/tests/checked_reduction.rs`. **Expected receipt pattern:** `.agent/science-s04/<attempt>/cases/T03-05-worker_panic.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T03-06-joint_correlation"></a>

### T03-06-joint_correlation — joint correlation

**Challenge:** Equal marginals but different enumerated correlations.

**Oracle:** Samplewise joint truth distinguishes populations. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/monte_carlo/tests/checked_reduction.rs`. **Expected receipt pattern:** `.agent/science-s04/<attempt>/cases/T03-06-joint_correlation.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T03-07-independence_control"></a>

### T03-07-independence_control — independence control

**Challenge:** Known independent finite joint population.

**Oracle:** Exact joint and marginal positive control. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/monte_carlo/tests/checked_reduction.rs`. **Expected receipt pattern:** `.agent/science-s04/<attempt>/cases/T03-07-independence_control.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T03-08-thread_replay"></a>

### T03-08-thread_replay — thread replay

**Challenge:** Same indexed seeds at 1/2/4 workers and batch permutations.

**Oracle:** Same-build scientific results stable. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/monte_carlo/tests/checked_reduction.rs`. **Expected receipt pattern:** `.agent/science-s04/<attempt>/cases/T03-08-thread_replay.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T03-09-population_shift"></a>

### T03-09-population_shift — population shift

**Challenge:** Two plausible distributions, large n, different joint risk.

**Oracle:** Assumption/discrepancy persists; no universal narrowing. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/monte_carlo/tests/checked_reduction.rs`. **Expected receipt pattern:** `.agent/science-s04/<attempt>/cases/T03-09-population_shift.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T03-10-media_retirement"></a>

### T03-10-media_retirement — media retirement

**Challenge:** Historical legacy read vs active retired lookup invocation.

**Oracle:** Historical adapter works; active lookup forbidden. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/monte_carlo/tests/checked_reduction.rs`. **Expected receipt pattern:** `.agent/science-s04/<attempt>/cases/T03-10-media_retirement.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.


## T04

<a id="T04-01-all_infeasible"></a>

### T04-01-all_infeasible — all infeasible

**Challenge:** Population entirely fails search feasibility.

**Oracle:** No feasible/certified elites. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/crescent/tests/search_contract.rs`. **Expected receipt pattern:** `.agent/science-s05/<attempt>/cases/T04-01-all_infeasible.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T04-02-legal_vs_feasible"></a>

### T04-02-legal_vs_feasible — legal vs feasible

**Challenge:** Record passes old legal predicate but fails full feasibility.

**Oracle:** Rejected from feasible/certified archive. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/crescent/tests/search_contract.rs`. **Expected receipt pattern:** `.agent/science-s05/<attempt>/cases/T04-02-legal_vs_feasible.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T04-03-partial_nonfinite"></a>

### T04-03-partial_nonfinite — partial nonfinite

**Challenge:** Missing FEM, NaN descriptor/quality, out-of-domain cell.

**Oracle:** Nonpassing/rejected with reason. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/crescent/tests/search_contract.rs`. **Expected receipt pattern:** `.agent/science-s05/<attempt>/cases/T04-03-partial_nonfinite.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T04-04-novelty_context"></a>

### T04-04-novelty_context — novelty context

**Challenge:** Change only novelty neighbors with same intrinsic quality.

**Oracle:** Cannot replace elite on contextual novelty alone. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/crescent/tests/search_contract.rs`. **Expected receipt pattern:** `.agent/science-s05/<attempt>/cases/T04-04-novelty_context.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T04-05-ties_duplicates"></a>

### T04-05-ties_duplicates — ties duplicates

**Challenge:** Tied quality, duplicate genome, distinct birth IDs, boundary bins.

**Oracle:** Frozen tie/cell/identity policy and complete lineage. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/crescent/tests/search_contract.rs`. **Expected receipt pattern:** `.agent/science-s05/<attempt>/cases/T04-05-ties_duplicates.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T04-06-invalid_config"></a>

### T04-06-invalid_config — invalid config

**Challenge:** Empty population or invalid island/migration/seed setup.

**Oracle:** Structured validation error. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/crescent/tests/search_contract.rs`. **Expected receipt pattern:** `.agent/science-s05/<attempt>/cases/T04-06-invalid_config.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T04-07-checkpoint_matrix"></a>

### T04-07-checkpoint_matrix — checkpoint matrix

**Challenge:** 3 seeds x generation 3/7/12, uninterrupted vs resumed.

**Oracle:** Exact scientific state/RNG/archive/counters same build. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/crescent/tests/search_contract.rs`. **Expected receipt pattern:** `.agent/science-s05/<attempt>/cases/T04-07-checkpoint_matrix.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T04-08-checkpoint_corrupt"></a>

### T04-08-checkpoint_corrupt — checkpoint corrupt

**Challenge:** Truncated/corrupt/missing checkpoint and incomplete event frame.

**Oracle:** Reject incompatible state; intact prefix retained only within scope. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/crescent/tests/search_contract.rs`. **Expected receipt pattern:** `.agent/science-s05/<attempt>/cases/T04-08-checkpoint_corrupt.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T04-09-resume_identity"></a>

### T04-09-resume_identity — resume identity

**Challenge:** Changed evaluator/RS/material/operator/descriptor hash.

**Oracle:** Refuse resume; explicit fork creates new identity. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/crescent/tests/search_contract.rs`. **Expected receipt pattern:** `.agent/science-s05/<attempt>/cases/T04-09-resume_identity.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T04-10-durability_boundary"></a>

### T04-10-durability_boundary — durability boundary

**Challenge:** Owned-child interruption around fsync/rename and log disk-full.

**Oracle:** No qualified publication after durability failure. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/crescent/tests/search_contract.rs`. **Expected receipt pattern:** `.agent/science-s05/<attempt>/cases/T04-10-durability_boundary.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T04-11-duplicate_resume"></a>

### T04-11-duplicate_resume — duplicate resume

**Challenge:** Repeat resume or replay accepted evaluation.

**Oracle:** No duplicate fresh evaluations/counter inflation. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/crescent/tests/search_contract.rs`. **Expected receipt pattern:** `.agent/science-s05/<attempt>/cases/T04-11-duplicate_resume.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T04-12-lineage_orphan"></a>

### T04-12-lineage_orphan — lineage orphan

**Challenge:** Missing parent/elite reference or migration order mismatch.

**Oracle:** Lineage validation rejects orphan state. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/crescent/tests/search_contract.rs`. **Expected receipt pattern:** `.agent/science-s05/<attempt>/cases/T04-12-lineage_orphan.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.


## T05

<a id="T05-01-linear_scaling"></a>

### T05-01-linear_scaling — linear scaling

**Challenge:** Multiply load under linear elastic valid model.

**Oracle:** Stress/displacement scale and SF inverse relation within tolerance. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/physics/tests/invariance.rs`. **Expected receipt pattern:** `.agent/science-s07/<attempt>/cases/T05-01-linear_scaling.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T05-02-rigid_transform"></a>

### T05-02-rigid_transform — rigid transform

**Challenge:** Translate/physically rotate geometry, loads, constraints together.

**Oracle:** Scalar invariants stable, tensors transformed correctly. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/physics/tests/invariance.rs`. **Expected receipt pattern:** `.agent/science-s07/<attempt>/cases/T05-02-rigid_transform.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T05-03-unit_consistency"></a>

### T05-03-unit_consistency — unit consistency

**Challenge:** Equivalent units plus deliberately inconsistent conversion.

**Oracle:** Equivalent physical answer; incompatible dimensions rejected. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/physics/tests/invariance.rs`. **Expected receipt pattern:** `.agent/science-s07/<attempt>/cases/T05-03-unit_consistency.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T05-04-analytic_balance"></a>

### T05-04-analytic_balance — analytic balance

**Challenge:** Beam/torsion/hole theory and force/moment/work balance.

**Oracle:** Independent theory/BC-compatible balance within registered tolerance. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/physics/tests/invariance.rs`. **Expected receipt pattern:** `.agent/science-s07/<attempt>/cases/T05-04-analytic_balance.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T05-05-bc_failure"></a>

### T05-05-bc_failure — bc failure

**Challenge:** Bad tags, incompatible loads, singular restraints.

**Oracle:** Explicit solver/model failure, no finite fallback certification. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/physics/tests/invariance.rs`. **Expected receipt pattern:** `.agent/science-s07/<attempt>/cases/T05-05-bc_failure.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T05-06-mesh_failure"></a>

### T05-06-mesh_failure — mesh failure

**Challenge:** Inverted/degenerate/disconnected elements and invalid material.

**Oracle:** Reject invalid model or solver nonconvergence. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/physics/tests/invariance.rs`. **Expected receipt pattern:** `.agent/science-s07/<attempt>/cases/T05-06-mesh_failure.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T05-07-field_semantics"></a>

### T05-07-field_semantics — field semantics

**Challenge:** DG interpolation mislabeled as L2/nodal and controlled known field.

**Oracle:** Exact observable version or rejection; no label laundering. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/physics/tests/invariance.rs`. **Expected receipt pattern:** `.agent/science-s07/<attempt>/cases/T05-07-field_semantics.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T05-08-localized_peak"></a>

### T05-08-localized_peak — localized peak

**Challenge:** Planted local peak amid good global mean/L2 agreement.

**Oracle:** Local extreme discrepancy remains visible. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/physics/tests/invariance.rs`. **Expected receipt pattern:** `.agent/science-s07/<attempt>/cases/T05-08-localized_peak.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T05-09-certificate_field"></a>

### T05-09-certificate_field — certificate field

**Challenge:** Mesh B scalar shown on A field, mixed-mesh minima.

**Oracle:** Separate bound observations; incompatible display rejected. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/physics/tests/invariance.rs`. **Expected receipt pattern:** `.agent/science-s07/<attempt>/cases/T05-09-certificate_field.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T05-10-native_binding"></a>

### T05-10-native_binding — native binding

**Challenge:** Same genome but different native geometry/process/mesh.

**Oracle:** Genome-only binding insufficient for native certificate. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/physics/tests/invariance.rs`. **Expected receipt pattern:** `.agent/science-s07/<attempt>/cases/T05-10-native_binding.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T05-11-calibration_failures"></a>

### T05-11-calibration_failures — calibration failures

**Challenge:** Tier calibration with failed/missing cases retained.

**Oracle:** All requested denominator/scope and errors reported. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/physics/tests/invariance.rs`. **Expected receipt pattern:** `.agent/science-s07/<attempt>/cases/T05-11-calibration_failures.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T05-12-tier_collision"></a>

### T05-12-tier_collision — tier collision

**Challenge:** Two incompatible evaluators both named T2.

**Oracle:** Explicit model/solver identity prevents exchange. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `crates/physics/tests/invariance.rs`. **Expected receipt pattern:** `.agent/science-s07/<attempt>/cases/T05-12-tier_collision.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.


## T06

<a id="T06-01-cpu_accounting"></a>

### T06-01-cpu_accounting — cpu accounting

**Challenge:** Owned workload with distinct wall/parent/child CPU.

**Oracle:** Report measured quantities, not wall-sum labeled core-seconds. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_statistics.py`. **Expected receipt pattern:** `.agent/science-s09/<attempt>/cases/T06-01-cpu_accounting.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T06-02-workload_collision"></a>

### T06-02-workload_collision — workload collision

**Challenge:** Different genomes with identical rounded-sum checksum.

**Oracle:** Exact content/order hash rejects false workload equality. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_statistics.py`. **Expected receipt pattern:** `.agent/science-s09/<attempt>/cases/T06-02-workload_collision.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T06-03-equal_budget"></a>

### T06-03-equal_budget — equal budget

**Challenge:** Methods with failed/cached/confirmed/attempted evaluations.

**Oracle:** Common total cost/evaluator/representation and complete accounting. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_statistics.py`. **Expected receipt pattern:** `.agent/science-s09/<attempt>/cases/T06-03-equal_budget.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T06-04-related_split"></a>

### T06-04-related_split — related split

**Challenge:** Repeated frames, siblings, parent stock/lot and trajectories across splits.

**Oracle:** Leakage reject; lineage groups kept together. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_statistics.py`. **Expected receipt pattern:** `.agent/science-s09/<attempt>/cases/T06-04-related_split.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T06-05-session_shortcut"></a>

### T06-05-session_shortcut — session shortcut

**Challenge:** Operator/session identity predicts label; nuisance holdout.

**Oracle:** No unqualified generalization from shortcut. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_statistics.py`. **Expected receipt pattern:** `.agent/science-s09/<attempt>/cases/T06-05-session_shortcut.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T06-06-preprocessing_leak"></a>

### T06-06-preprocessing_leak — preprocessing leak

**Challenge:** Fit normalization/segmentation/threshold on holdout.

**Oracle:** Reject leaked protocol, valid train-only control accepted. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_statistics.py`. **Expected receipt pattern:** `.agent/science-s09/<attempt>/cases/T06-06-preprocessing_leak.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T06-07-single_specimen"></a>

### T06-07-single_specimen — single specimen

**Challenge:** Many grains/scans from one Ti7Al specimen.

**Oracle:** Method result only, no multi-lot production gate. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_statistics.py`. **Expected receipt pattern:** `.agent/science-s09/<attempt>/cases/T06-07-single_specimen.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T06-08-uncertainty_censor"></a>

### T06-08-uncertainty_censor — uncertainty censor

**Challenge:** Clustered/censored/empty/zero-variance strata.

**Oracle:** Correct intervals/status; no independent-count inflation. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_statistics.py`. **Expected receipt pattern:** `.agent/science-s09/<attempt>/cases/T06-08-uncertainty_censor.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T06-09-null_benchmark"></a>

### T06-09-null_benchmark — null benchmark

**Challenge:** No optimization advantage or valid negative result.

**Oracle:** Honest effect/interval report; not forced superiority. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_statistics.py`. **Expected receipt pattern:** `.agent/science-s09/<attempt>/cases/T06-09-null_benchmark.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T06-10-surrogate_gate"></a>

### T06-10-surrogate_gate — surrogate gate

**Challenge:** Near-threshold false-feasible/OOD/shared-bias ensemble.

**Oracle:** Abstention/independent confirmation; confidence not qualification. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/testing/tests/test_statistics.py`. **Expected receipt pattern:** `.agent/science-s09/<attempt>/cases/T06-10-surrogate_gate.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.


## T07

<a id="T07-01-specimen_identity"></a>

### T07-01-specimen_identity — specimen identity

**Challenge:** Same specimen ID in different lot or absent route.

**Oracle:** Reject collision/unknown applicability. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/tests/test_history_observation.py`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T07-01-specimen_identity.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T07-02-unapproved_transfer"></a>

### T07-02-unapproved_transfer — unapproved transfer

**Challenge:** Change material process or surface state after qualification.

**Oracle:** Dependency invalidated; no unapproved property transfer. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/tests/test_history_observation.py`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T07-02-unapproved_transfer.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T07-03-history_prefix"></a>

### T07-03-history_prefix — history prefix

**Challenge:** Future/post-exposure observation attached to pre-exposure history.

**Oracle:** Binding rejects wrong prefix. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/tests/test_history_observation.py`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T07-03-history_prefix.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T07-04-clock_segments"></a>

### T07-04-clock_segments — clock segments

**Challenge:** Time-zone mismatch, monotonic reset, interruption/overlap.

**Oracle:** Explicit segments/uncertainty; no invented continuous history. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/tests/test_history_observation.py`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T07-04-clock_segments.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T07-05-raw_calibration"></a>

### T07-05-raw_calibration — raw calibration

**Challenge:** Changed raw file, invalid/future calibration or missing trace.

**Oracle:** Observation unavailable for claim. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/tests/test_history_observation.py`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T07-05-raw_calibration.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T07-06-lab_reduction"></a>

### T07-06-lab_reduction — lab reduction

**Challenge:** Known raw load/strain/mass traces with drift, units, censoring and missing segments.

**Oracle:** Independent reduction matches; derived allowables cannot omit missing/failure records. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/tests/test_history_observation.py`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T07-06-lab_reduction.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T07-07-finished_mass"></a>

### T07-07-finished_mass — finished mass

**Challenge:** Bare part versus finish/carrier/assembly measured mass with uncertainty.

**Oracle:** Report correct configuration; no unmeasured finish/assembly mass substitution. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/tests/test_history_observation.py`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T07-07-finished_mass.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T07-08-frame_unit"></a>

### T07-08-frame_unit — frame unit

**Challenge:** Wrong handedness/unit/orientation/ROI transform.

**Oracle:** Reject incompatible observation. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/tests/test_history_observation.py`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T07-08-frame_unit.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T07-09-censored_zero"></a>

### T07-09-censored_zero — censored zero

**Challenge:** Nondetection/limit vs legitimate measured zero.

**Oracle:** Nondetection censored/unknown, zero remains a value. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/tests/test_history_observation.py`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T07-09-censored_zero.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T07-10-chemistry_scope"></a>

### T07-10-chemistry_scope — chemistry scope

**Challenge:** Bulk composition used as local organization or prep-H as intrinsic.

**Oracle:** Scope/inference rejected without supporting observation. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/tests/test_history_observation.py`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T07-10-chemistry_scope.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T07-11-measurement_lineage"></a>

### T07-11-measurement_lineage — measurement lineage

**Challenge:** Duplicated scan or missing session/operator grouping.

**Oracle:** No false independence, unknown grouping explicit. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/tests/test_history_observation.py`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T07-11-measurement_lineage.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T07-12-interpretation_unknown"></a>

### T07-12-interpretation_unknown — interpretation unknown

**Challenge:** Absent/invalid measurement interpreted healthy.

**Oracle:** Unable_to_determine; positive valid measurement accepted. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/tests/test_history_observation.py`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T07-12-interpretation_unknown.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T07-13-historical_adapter"></a>

### T07-13-historical_adapter — historical adapter

**Challenge:** Old incomplete lab record loaded under new schema.

**Oracle:** Readable historical scope; cannot autoqualify new claim. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `assurance/tests/test_history_observation.py`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T07-13-historical_adapter.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.


## T08

<a id="T08-01-bounded_download"></a>

### T08-01-bounded_download — bounded download

**Challenge:** Archive expanded size/file access exceeds 5GB approved cap.

**Oracle:** No download/extract; unavailable full reproduction. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/multimodal/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T08-01-bounded_download.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T08-02-units_metadata"></a>

### T08-02-units_metadata — units metadata

**Challenge:** Ambiguous units or DOI/version/license mismatch.

**Oracle:** Stop physical comparison until resolved metadata. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/multimodal/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T08-02-units_metadata.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T08-03-registration_truth"></a>

### T08-03-registration_truth — registration truth

**Challenge:** Tiny synthetic known transform plus mirrored/wrong-grain mapping.

**Oracle:** Correct transform; false match rejected. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/multimodal/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T08-03-registration_truth.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T08-04-orientation_symmetry"></a>

### T08-04-orientation_symmetry — orientation symmetry

**Challenge:** Equivalent symmetry/orientation vs degrees/radians mismatch.

**Oracle:** Equivalent accepted; convention mismatch rejected. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/multimodal/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T08-04-orientation_symmetry.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T08-05-matched_slip_missing"></a>

### T08-05-matched_slip_missing — matched slip missing

**Challenge:** Small EBSD/DCT subset without matched DIC/slip acquisitions.

**Oracle:** Importer-only receipt, not full slip reproduction. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/multimodal/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T08-05-matched_slip_missing.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T08-06-group_acquisition"></a>

### T08-06-group_acquisition — group acquisition

**Challenge:** Repeated acquisitions/unobserved grains from one specimen.

**Oracle:** Coverage visible and specimen grouping retained. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/multimodal/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T08-06-group_acquisition.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.


## T09

<a id="T09-01-latent_collision"></a>

### T09-01-latent_collision — latent collision

**Challenge:** Identical summaries but hidden synthetic histories differ in futures.

**Oracle:** Detect omitted state on heldout data within power. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/state/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T09-01-latent_collision.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T09-02-sufficient_null"></a>

### T09-02-sufficient_null — sufficient null

**Challenge:** Synthetic summary actually sufficient, no future history effect.

**Oracle:** No forced discovery; scoped null/inconclusive. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/state/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T09-02-sufficient_null.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T09-03-matching_bias"></a>

### T09-03-matching_bias — matching bias

**Challenge:** Post-history matching/survivor filtering chosen after outcomes.

**Oracle:** Protocol invalid/confounded, not causal state evidence. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/state/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T09-03-matching_bias.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T09-04-early_irreversible"></a>

### T09-04-early_irreversible — early irreversible

**Challenge:** Measured early slip vs reversible strain and uncertain reference.

**Oracle:** Measured irreversibility separate from crack/lifetime claim. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/state/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T09-04-early_irreversible.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T09-05-lot_confirmation"></a>

### T09-05-lot_confirmation — lot confirmation

**Challenge:** Pilot calibration reused as confirmatory holdout or grains as lots.

**Oracle:** Reject leakage/count inflation. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/state/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T09-05-lot_confirmation.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T09-06-low_power_null"></a>

### T09-06-low_power_null — low power null

**Challenge:** Few independent specimens with wide effect interval.

**Oracle:** Inconclusive, not universal state sufficiency. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/state/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T09-06-low_power_null.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.


## T10

<a id="T10-01-same_marginals_graph"></a>

### T10-01-same_marginals_graph — same marginals graph

**Challenge:** Graphs with same marginals and different connectivity.

**Oracle:** Topology contrast test against histogram baseline. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/neighborhood/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T10-01-same_marginals_graph.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T10-02-physical_rotation"></a>

### T10-02-physical_rotation — physical rotation

**Challenge:** Graph/orientation physically rotated or symmetry-equivalent.

**Oracle:** Equivalent feature/prediction within tolerance. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/neighborhood/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T10-02-physical_rotation.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T10-03-shuffle_control"></a>

### T10-03-shuffle_control — shuffle control

**Challenge:** Statistical edge shuffle preserves named marginals.

**Oracle:** Valid analysis control, no manufactured-state claim. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/neighborhood/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T10-03-shuffle_control.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T10-04-resolution_boundary"></a>

### T10-04-resolution_boundary — resolution boundary

**Challenge:** ROI-truncated neighborhoods/registration uncertainty.

**Oracle:** Boundary/resolution uncertainty visible. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/neighborhood/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T10-04-resolution_boundary.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T10-05-chemistry_organization"></a>

### T10-05-chemistry_organization — chemistry organization

**Challenge:** Same bulk chemistry with unknown vs measured atomic state.

**Oracle:** No inferred local order from certificate alone. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/neighborhood/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T10-05-chemistry_organization.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.


## T11

<a id="T11-01-sequence_controls"></a>

### T11-01-sequence_controls — sequence controls

**Challenge:** A→B/B→A/A/B/unexposed/sham with matched total clocks.

**Oracle:** Protocol isolates order and includes intermediate observations. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/history/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T11-01-sequence_controls.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T11-02-order_null"></a>

### T11-02-order_null — order null

**Challenge:** Additive order-independent synthetic generator.

**Oracle:** Null accepted; no forced history term. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/history/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T11-02-order_null.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T11-03-order_positive"></a>

### T11-03-order_positive — order positive

**Challenge:** Synthetic state transition where order changes outcome.

**Oracle:** Detect order effect with preregistered analysis. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/history/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T11-03-order_positive.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T11-04-cycle_time_clock"></a>

### T11-04-cycle_time_clock — cycle time clock

**Challenge:** Equal cycles but unequal elapsed time/heating, and equal-time control.

**Oracle:** Clock confounding explicit, no cycle-only causal inference. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/history/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T11-04-cycle_time_clock.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T11-05-hold_redistribution"></a>

### T11-05-hold_redistribution — hold redistribution

**Challenge:** External hold with local response observed/unknown.

**Oracle:** Local evolution represented; unknown not assumed invariant. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/history/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T11-05-hold_redistribution.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T11-06-acceleration_mode"></a>

### T11-06-acceleration_mode — acceleration mode

**Challenge:** Accelerated condition changes initiation/failure mode.

**Oracle:** Nontransferable; no service-year claim. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/history/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T11-06-acceleration_mode.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T11-07-competing_censor"></a>

### T11-07-competing_censor — competing censor

**Challenge:** Multiple failure modes and right/interval-censored units.

**Oracle:** Mode-specific analysis and denominator retained. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/history/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T11-07-competing_censor.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.


## T12

<a id="T12-01-observation_arms"></a>

### T12-01-observation_arms — observation arms

**Challenge:** Frequent/sparse/endpoint, prep/sham, matched handling.

**Oracle:** Schedule/preparation identifiable or causal claim incomplete. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/observer/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T12-01-observation_arms.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T12-02-created_erased"></a>

### T12-02-created_erased — created erased

**Challenge:** Synthetic observer creates or removes signal/material effect.

**Oracle:** Distinguish scoped effect, no universal artifact label. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/observer/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T12-02-created_erased.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T12-03-drift_only"></a>

### T12-03-drift_only — drift only

**Challenge:** Nuisance instrument drift with material unchanged.

**Oracle:** Not diagnosed as damage inside validated bounds. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/observer/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T12-03-drift_only.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T12-04-true_nuisance"></a>

### T12-04-true_nuisance — true nuisance

**Challenge:** True change with nuisance range exceeded vs validated range.

**Oracle:** Detect within scope; outside range unknown. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/observer/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T12-04-true_nuisance.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T12-05-prep_transfer"></a>

### T12-05-prep_transfer — prep transfer

**Challenge:** Material-specific preparation study generalized to different alloy.

**Oracle:** Unsupported applicability rejected. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/observer/`. **Expected receipt pattern:** `.agent/science-s13/<attempt>/cases/T12-05-prep_transfer.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.


## T13

<a id="T13-01-process_vs_film"></a>

### T13-01-process_vs_film — process vs film

**Challenge:** Film effects with/without sham deposition process control.

**Oracle:** No film attribution without process control. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/surfaces/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T13-01-process_vs_film.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T13-02-coupling_null"></a>

### T13-02-coupling_null — coupling null

**Challenge:** Synergy/antagonism/additive/null synthetic exposures.

**Oracle:** All outcomes allowed; no hard-coded synergy. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/surfaces/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T13-02-coupling_null.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T13-03-topology_average"></a>

### T13-03-topology_average — topology average

**Challenge:** Same average thickness/fraction, different connected defects.

**Oracle:** Connectivity evaluated separately from averages. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/surfaces/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T13-03-topology_average.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T13-04-segmentation_bias"></a>

### T13-04-segmentation_bias — segmentation bias

**Challenge:** Threshold/resolution chosen using damage outcomes.

**Oracle:** Reject leakage; sensitivity reported. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/surfaces/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T13-04-segmentation_bias.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T13-05-balance_reservoir"></a>

### T13-05-balance_reservoir — balance reservoir

**Challenge:** Synthetic conserved mass and missing debris/solution reservoir.

**Oracle:** Correct balance; incomplete collection prevents closure claim. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/surfaces/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T13-05-balance_reservoir.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T13-06-charge_not_loss"></a>

### T13-06-charge_not_loss — charge not loss

**Challenge:** Mechanical loss omitted from Faradaic-current estimate.

**Oracle:** No total-loss claim from charge alone. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/surfaces/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T13-06-charge_not_loss.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T13-07-barrier_vs_structure"></a>

### T13-07-barrier_vs_structure — barrier vs structure

**Challenge:** Barrier signal recovers with substrate damage persistent.

**Oracle:** Barrier-only recovery, no mechanical restoration. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/surfaces/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T13-07-barrier_vs_structure.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.


## T14

<a id="T14-01-truth_independent"></a>

### T14-01-truth_independent — truth independent

**Challenge:** Candidate signal used as reference damage label.

**Oracle:** Reject circular qualification. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/inspection/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T14-01-truth_independent.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T14-02-silent_damage"></a>

### T14-02-silent_damage — silent damage

**Challenge:** Relevant reference damage without alarm.

**Oracle:** Count miss, not healthy/drop. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/inspection/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T14-02-silent_damage.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T14-03-healthy_nuisance"></a>

### T14-03-healthy_nuisance — healthy nuisance

**Challenge:** Healthy sample changes fixture/temp/excitation/session.

**Oracle:** False calls assessed; invalid scope unknown. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/inspection/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T14-03-healthy_nuisance.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T14-04-instrument_nonlinearity"></a>

### T14-04-instrument_nonlinearity — instrument nonlinearity

**Challenge:** Fixture/transducer/gain saturation generates nonlinear signal.

**Oracle:** Calibration/nuisance reject or unknown. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/inspection/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T14-04-instrument_nonlinearity.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T14-05-lot_session_holdout"></a>

### T14-05-lot_session_holdout — lot session holdout

**Challenge:** Train/test share specimen or session identity shortcut.

**Oracle:** Reject leakage; valid grouped holdout evaluated. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/inspection/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T14-05-lot_session_holdout.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T14-06-pod_prevalence"></a>

### T14-06-pod_prevalence — pod prevalence

**Challenge:** Balanced challenge set used for unconditional population risk.

**Oracle:** Only conditional rates; prevalence assumption explicit. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/inspection/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T14-06-pod_prevalence.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T14-07-blind_zone"></a>

### T14-07-blind_zone — blind zone

**Challenge:** Inaccessible location or below reference-method resolution.

**Oracle:** Unknown region, no universal no-damage conclusion. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/inspection/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T14-07-blind_zone.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T14-08-warning_censor"></a>

### T14-08-warning_censor — warning censor

**Challenge:** No/late alarm, interval times, unreadable sample.

**Oracle:** All denominator retained; margin intervals and misses visible. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/inspection/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T14-08-warning_censor.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T14-09-shared_sensor_bias"></a>

### T14-09-shared_sensor_bias — shared sensor bias

**Challenge:** Two sensors agree via common fixture/prep error.

**Oracle:** Agreement cannot establish independent truth. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/inspection/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T14-09-shared_sensor_bias.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.


## T15

<a id="T15-01-strain_vs_damage"></a>

### T15-01-strain_vs_damage — strain vs damage

**Challenge:** Bright reversible strain signal vs independently damaged carrier.

**Oracle:** No component-damage diagnosis from strain brightness alone. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/witness/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T15-01-strain_vs_damage.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T15-02-witness_invalid"></a>

### T15-02-witness_invalid — witness invalid

**Challenge:** Faded/detached/blocked/reference-failed witness.

**Oracle:** Unknown, never healthy. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/witness/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T15-02-witness_invalid.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T15-03-shared_selftest"></a>

### T15-03-shared_selftest — shared selftest

**Challenge:** Active/reference share fading or delamination failure.

**Oracle:** No false valid self-test. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/witness/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T15-03-shared_selftest.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T15-04-warning_persistence"></a>

### T15-04-warning_persistence — warning persistence

**Challenge:** Alarm then unloading/rest/apparent recovery or signal fade.

**Oracle:** Prior alarm/history retained. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/witness/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T15-04-warning_persistence.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T15-05-reader_environment"></a>

### T15-05-reader_environment — reader environment

**Challenge:** Lighting/reader/camera/wetness/dirt/lot shift.

**Oracle:** Frozen scope/holdout and unknown outside it. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/witness/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T15-05-reader_environment.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T15-06-mass_coupling"></a>

### T15-06-mass_coupling — mass coupling

**Challenge:** Unmeasured added mass or witness own exposure mistaken component damage.

**Oracle:** No mass/strength/diagnostic claim without direct evidence. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/witness/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T15-06-mass_coupling.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.


## T16

<a id="T16-01-closure_false_heal"></a>

### T16-01-closure_false_heal — closure false heal

**Challenge:** Appearance/contact closes while cohesion/function not restored.

**Oracle:** Only supported observable, stronger healing rejected. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/recovery/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T16-01-closure_false_heal.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T16-02-recovery_positive"></a>

### T16-02-recovery_positive — recovery positive

**Challenge:** Scoped independent structural/function/rechallenge positive control.

**Oracle:** Only corresponding evidence level supported. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/recovery/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T16-02-recovery_positive.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T16-03-failure_migration"></a>

### T16-03-failure_migration — failure migration

**Challenge:** Original site improves but new/internal mode initiates.

**Oracle:** Competing failure retained; no disappearance from totals. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/recovery/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T16-03-failure_migration.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T16-04-isotope_exchange"></a>

### T16-04-isotope_exchange — isotope exchange

**Challenge:** Isotope exchange without net material growth.

**Oracle:** No growth/healing claim. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/recovery/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T16-04-isotope_exchange.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T16-05-source_ambiguity"></a>

### T16-05-source_ambiguity — source ambiguity

**Challenge:** Environment/prep/original sources indistinguishable or contamination.

**Oracle:** Unresolved source, not preferred causal explanation. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/recovery/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T16-05-source_ambiguity.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T16-06-nanoscale_transfer"></a>

### T16-06-nanoscale_transfer — nanoscale transfer

**Challenge:** Vacuum nanoscale Pt result offered as bulk Ti strength credit.

**Oracle:** Applicability rejects transfer. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/recovery/`. **Expected receipt pattern:** `.agent/science-s14/<attempt>/cases/T16-06-nanoscale_transfer.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.


## T17

<a id="T17-01-model_domain"></a>

### T17-01-model_domain — model domain

**Challenge:** Correct elements but unsupported charge/reactivity/phase/surface.

**Oracle:** Potential not qualified for mechanism. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/atomistic/`. **Expected receipt pattern:** `.agent/science-s11/<attempt>/cases/T17-01-model_domain.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T17-02-transition_error"></a>

### T17-02-transition_error — transition error

**Challenge:** Low average RMSE but wrong barrier/force direction.

**Oracle:** Mechanism challenge rejects model. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/atomistic/`. **Expected receipt pattern:** `.agent/science-s11/<attempt>/cases/T17-02-transition_error.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T17-03-ensemble_shared_bias"></a>

### T17-03-ensemble_shared_bias — ensemble shared bias

**Challenge:** Models agree because of shared training/approximation bias.

**Oracle:** Agreement not correctness, uncertainty/discrepancy exposed. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/atomistic/`. **Expected receipt pattern:** `.agent/science-s11/<attempt>/cases/T17-03-ensemble_shared_bias.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T17-04-rate_sensitivity"></a>

### T17-04-rate_sensitivity — rate sensitivity

**Challenge:** 0.1eV difference, 300K, identical prefactor mathematical control.

**Oracle:** Rate ratio approximately 48; no Ti measured-rate inference. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/atomistic/`. **Expected receipt pattern:** `.agent/science-s11/<attempt>/cases/T17-04-rate_sensitivity.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T17-05-time_bridge"></a>

### T17-05-time_bridge — time bridge

**Challenge:** Short MD no-event trajectory converted into service life.

**Oracle:** Unsupported kinetic bridge rejected. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/atomistic/`. **Expected receipt pattern:** `.agent/science-s11/<attempt>/cases/T17-05-time_bridge.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T17-06-holdout_reference"></a>

### T17-06-holdout_reference — holdout reference

**Challenge:** Fine-tune against failed holdout then reuse it or DFT declared exact.

**Oracle:** New holdout/reference uncertainty required. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/atomistic/`. **Expected receipt pattern:** `.agent/science-s11/<attempt>/cases/T17-06-holdout_reference.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.


## T18

<a id="T18-01-mixed_none_models"></a>

### T18-01-mixed_none_models — mixed none models

**Challenge:** Coexisting mechanisms or missing true explanation.

**Oracle:** Mixture/none-fits/inconclusive permitted. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/falsification/`. **Expected receipt pattern:** `.agent/science-s10/<attempt>/cases/T18-01-mixed_none_models.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T18-02-nonidentifiable"></a>

### T18-02-nonidentifiable — nonidentifiable

**Challenge:** Distinct mechanisms fit all available observations.

**Oracle:** No forced confident selection. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/falsification/`. **Expected receipt pattern:** `.agent/science-s10/<attempt>/cases/T18-02-nonidentifiable.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T18-03-audit_independent"></a>

### T18-03-audit_independent — audit independent

**Challenge:** Agent chooses all data or reads sealed truth.

**Oracle:** Protocol rejects leaked/self-selected validation. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/falsification/`. **Expected receipt pattern:** `.agent/science-s10/<attempt>/cases/T18-03-audit_independent.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T18-04-budget_menu"></a>

### T18-04-budget_menu — budget menu

**Challenge:** Unavailable experiment, cost exhaustion, no informative action.

**Oracle:** Stop/abstain with cost ledger; no invented acquisition. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/falsification/`. **Expected receipt pattern:** `.agent/science-s10/<attempt>/cases/T18-04-budget_menu.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T18-05-strategy_comparison"></a>

### T18-05-strategy_comparison — strategy comparison

**Challenge:** Fixed DOE/random/agent unequal total acquisition cost.

**Oracle:** Reject comparison; equal-budget control usable. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/falsification/`. **Expected receipt pattern:** `.agent/science-s10/<attempt>/cases/T18-05-strategy_comparison.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T18-06-failed_acquisition"></a>

### T18-06-failed_acquisition — failed acquisition

**Challenge:** No-result experiment, contradiction, invalid priors.

**Oracle:** Failures retained; hypotheses revised/revoked visibly. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/falsification/`. **Expected receipt pattern:** `.agent/science-s10/<attempt>/cases/T18-06-failed_acquisition.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T18-07-authority_boundary"></a>

### T18-07-authority_boundary — authority boundary

**Challenge:** Agent attempts threshold/requirement/physical-control mutation.

**Oracle:** Offline menu refuses unsupported action. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `research/science_v1/falsification/`. **Expected receipt pattern:** `.agent/science-s10/<attempt>/cases/T18-07-authority_boundary.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.


## T19

<a id="T19-01-required_skip"></a>

### T19-01-required_skip — required skip

**Challenge:** No discovered tests, missing dependency or required ignored case.

**Oracle:** Incomplete/error, not green. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `ops/ci/tests/test_science_suite.py`. **Expected receipt pattern:** `.agent/science-s16/<attempt>/cases/T19-01-required_skip.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T19-02-path_filter"></a>

### T19-02-path_filter — path filter

**Challenge:** Requirements/schema/solver/claims change skips dependent suite.

**Oracle:** Reject incomplete workflow selection. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `ops/ci/tests/test_science_suite.py`. **Expected receipt pattern:** `.agent/science-s16/<attempt>/cases/T19-02-path_filter.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T19-03-capability_omission"></a>

### T19-03-capability_omission — capability omission

**Challenge:** Feature enabled with unexecuted or omitted qualification cases.

**Oracle:** Capability gate incomplete. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `ops/ci/tests/test_science_suite.py`. **Expected receipt pattern:** `.agent/science-s16/<attempt>/cases/T19-03-capability_omission.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T19-04-runner_escape"></a>

### T19-04-runner_escape — runner escape

**Challenge:** Receipt argv/path/symlink escapes owned roots.

**Oracle:** Reject untrusted execution/path. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `ops/ci/tests/test_science_suite.py`. **Expected receipt pattern:** `.agent/science-s16/<attempt>/cases/T19-04-runner_escape.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T19-05-upload_missing"></a>

### T19-05-upload_missing — upload missing

**Challenge:** Missing/corrupt artifact, timeout or oversized output.

**Oracle:** Attempt incomplete and raw failure retained. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `ops/ci/tests/test_science_suite.py`. **Expected receipt pattern:** `.agent/science-s16/<attempt>/cases/T19-05-upload_missing.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T19-06-fresh_vs_reuse"></a>

### T19-06-fresh_vs_reuse — fresh vs reuse

**Challenge:** Reused historical evidence labeled new run.

**Oracle:** Reject freshness claim; scoped compatible reuse explicit. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `ops/ci/tests/test_science_suite.py`. **Expected receipt pattern:** `.agent/science-s16/<attempt>/cases/T19-06-fresh_vs_reuse.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T19-07-dossier_mismatch"></a>

### T19-07-dossier_mismatch — dossier mismatch

**Challenge:** CLI/table/render claim uses different candidate/material/case hashes.

**Oracle:** Reconstruction integrity fails. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `ops/ci/tests/test_science_suite.py`. **Expected receipt pattern:** `.agent/science-s16/<attempt>/cases/T19-07-dossier_mismatch.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T19-08-render_finish_metadata"></a>

### T19-08-render_finish_metadata — render finish metadata

**Challenge:** Color/thickness appearance metadata populates structural material values.

**Oracle:** Reject nonmeasured structural-property transfer. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `ops/ci/tests/test_science_suite.py`. **Expected receipt pattern:** `.agent/science-s16/<attempt>/cases/T19-08-render_finish_metadata.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T19-09-protective_interface"></a>

### T19-09-protective_interface — protective interface

**Challenge:** Unseated/interrupted/directional retention record missing while scalar sheath result favorable.

**Oracle:** Preserve existing sheath tests; incomplete directional/handling claims cannot promote. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `ops/ci/tests/test_science_suite.py`. **Expected receipt pattern:** `.agent/science-s16/<attempt>/cases/T19-09-protective_interface.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.

<a id="T19-10-final_policy"></a>

### T19-10-final_policy — final policy

**Challenge:** Required §19 lab gate missing or owner amendment absent.

**Oracle:** Scoped RC only; final release blocked by policy. **Frozen status:** proposed; not implemented or executed.

**Allowed implementation location:** `ops/ci/tests/test_science_suite.py`. **Expected receipt pattern:** `.agent/science-s16/<attempt>/cases/T19-10-final_policy.json`.

**Claim limit:** No new physical validation from synthetic or metadata-only execution.
