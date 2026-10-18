# RSV2-07: critical_fem_3 GCI and A4 verification

The requested three-level GCI is **not admitted**: every recorded observable is oscillatory. The native compliant-support/buckling diagnostic **passes**, with buckling eigenvalue **9.78293400282** and critical load **2543.56284073 N**. This completes the requested diagnostics; it does not qualify the candidate across all gates. Human-max twist, fatigue and the production sequence requirement remain unresolved.

## Candidate and execution

- Candidate: [critical_fem_3.csv](results/critical_fem_3.csv), SHA-256 `070da10ab3c66dced8592aabf4a06ec2c68708a3f92df43e64685bbe6019da15`.
- Construction: exact compound law from the `critical_fem_3` entry in [validation.json](results/validation.json), replayed through the existing [compound.rs](compound.rs). The CSV alone in the production CLI does not select this law.
- Configuration: angle 18 degrees, apex 0.175 mm, d0/d1 0.45/0.55 mm, point shape 3, point slope 0.3, point end 6 mm, radial=true, point lateral factor 2, smooth heel=true, core=0 (native knot thickness), boundary tolerance 0.001 mm, export area 0.25 mm², export grid 0.025 mm. The FEM keeps native `run_b` meshing and samples this same thickness law.
- Native geometric predicate passes, genes match the canonical CSV exactly and no clamping was applied. No geometry, load, gate, exclusion or production solver was changed.
- Checkout: `6e4f1bd87d198ad16fd723047cbcb7b87b4a0b7e`. Run started 2026-10-18 05:37:39 UTC; all scientific builds, solves and focused tests pinned to CPUs **14,15**, `CARGO_BUILD_JOBS=2`, `JARAMBIT_SOLVER=faer`, `RAYON_NUM_THREADS=2`, `CARGO_INCREMENTAL=0`, `CARGO_PROFILE_DEV_DEBUG=line-tables-only`.
- Audit executable SHA-256: `402e3798de48f18a73c9ab4fbfdb335237cbba80875a470c88e7f354688cd1ae`.

## Three-level GCI

[Audit runner](gci_a4.rs) calls **`jarambit_crescent::gci::compute_gci`** from the compiled native crate. Mesh quantities come from unchanged native `fem::report_fields`, with the isolated compound export wrapper. The call orders values fine to coarse (0.25, 1, 4 mm²), uses nominal h = sqrt(max_area) = (0.5, 1, 2) mm, r=2 and safety factor Fs=1.25. This is the repository's area-based three-level protocol; no nested-mesh or asymptotic-convergence proof is inferred from the nominal h values.

| Max area mm² | Nominal h mm | Nodes | Elements | seq_ratio | sf_hunter |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 4 | 2 | 1975 | 3243 | 0.687814958936609 | 1.27673642886738 |
| 1 | 1 | 4394 | 7937 | 0.65180475268746 | 1.24879611405362 |
| 0.25 | 0.5 | 15560 | 29505 | 0.671969130222755 | 1.26714215255503 |

| Observable | 4 mm² | 1 mm² | 0.25 mm² | Fit / admitted |
| --- | ---: | ---: | ---: | --- |
| seq_ratio | 0.687814958936609 | 0.65180475268746 | 0.671969130222755 | oscillatory / false |
| sf_hunter | 1.27673642886738 | 1.24879611405362 | 1.26714215255503 | oscillatory / false |
| hm_twist_sf | 0.436762687997413 | 0.405196851833175 | 0.464772549148637 | oscillatory / false |
| hm_spine_sf | 1.08830417670716 | 1.00392907340277 | 1.08473554167185 | oscillatory / false |
| hm_ringback_sf | 7.00878864991744 | 7.11260983661331 | 6.91488350230804 | oscillatory / false |
| fat_sf | 0.927248361014683 | 0.906956301877405 | 0.920280378599042 | oscillatory / false |

For every observable, `fit_status=oscillatory`, `reason=oscillatory_sequence`, `monotone=false`, `admitted=false`, `relative_error_available=false`, and **p, extrapolated value and fine-grid relative GCI are null**. The primary seq_ratio differences in fine-to-coarse order are -0.020164377535295 and +0.0360102062491497: their signs reverse. The native [oscillation check](../../crates/crescent/src/gci.rs#L263) rejects the sequence before fitting an order. No absolute-difference substitution or arithmetic-only fit was used.

Mesh identities and observable metadata validate (`sequence_valid=true`, `observable_valid=true`), while `asymptotic_validation=not_established`. These metadata flags do not establish numerical convergence. In particular, the battery's aggregate `plate_gci_valid_3level` expression currently checks those two flags alone ([battery.rs](../../crates/crescent/src/battery.rs#L3171)); the separate `plate.gci` case checks `admitted` ([battery.rs](../../crates/crescent/src/battery.rs#L1408)). This audit follows `compute_gci` admission and records **failure to establish the <3% discretization criterion**, since no error estimate is available. The full battery was not invoked and its metadata-only aggregate flag is not presented as a GCI pass.

## Compliant support and buckling

**`jarambit_physics::side_a4::solve_side_a4`** receives the exact retained 0.25 mm² mesh, including its compound element thicknesses: 15560 nodes, 29505 elements. It uses native CST + Batoz DKT with faer sparse Cholesky and deterministic K-orthogonal Lanczos (maximum 32 steps).

The reference is the native certification definition `min(seq_ratio at 0.25, 0.125 mm²)`: min(0.6719691302227545, 0.570264799833376) = **0.570264799833376**. The 0.125 mm² solve was replayed specifically to verify this value; it is not a fourth GCI level. The input field named `d1_sha256` is bound to the retained mesh-file hash here; this standalone audit does not fabricate a D1 certificate.

| Quantity | Value |
| --- | ---: |
| Native A4 status | **pass** |
| Buckling eigenvalue (260 N reference) | 9.78293400282365 |
| Critical compressive load N = 260 × eigenvalue | 2543.56284073415 |
| Supported blade peak MPa/N | 0.633114627964315 |
| Supported hand peak MPa/N | 0.210946918210389 |
| Supported sequence ratio (blade / hand) | 3.00129830449987 |
| Certification reference sequence ratio | 0.570264799833376 |
| Supported minus certification ratio | 2.4310335046665 |
| Supported minus same-mesh clamp ratio | 2.32932917427712 |
| Bore / grip support nodes | 131 / 7064 |

Native supports: radial/tangential/flange bore stiffnesses 50,000 / 5,000 / 100,000 N/mm; Winkler grip foundation 200 N/mm³; E=113,800 MPa and nu=0.34. The solver computes bending peaks under unit lateral load outside the 15 mm tip exclusion and geometric stiffness under 260 N in-plane compression. These are its existing diagnostic definitions, not replacements for the production worst-case sequence observable.

The [native A4 status](../../crates/physics/src/side_a4.rs#L578) requires eigenvalue >0 and supported sequence ratio >= the reference; both pass. The stronger aggregate-battery condition `status == pass && eigenvalue >= 1` also evaluates true for these results ([battery.rs](../../crates/crescent/src/battery.rs#L3178)). A positive eigenvalue alone would not demonstrate capacity above 260 N. The implementation floors the transformed eigenvalue at 1e-15 and does not return an eigenpair residual or a buckling mesh-convergence certificate; this report records its native diagnostic, without claiming independent buckling validation.

The improved support ratio does **not** change the production ratio 0.570264799833376 or resolve its requirement. A4 is a plate diagnostic at one mesh level, not independent 3D P2/DOLFINx or manufacturing qualification. The prior three-fine-area HM twist minimum remains 0.431361466405 <1, spine minimum remains 1.02457926938 >=1, and fatigue remains below requirement. The inherited collar/flat-stock issue remains open.

## Verification and provenance

The isolated [Python driver](gci_a4.py) checks the candidate hash/configuration/genes, replays prior 0.25 and 0.125 mm² seq_ratio/HM twist/spine/ring-back/fatigue results within relative 1e-10 and absolute 1e-12, verifies every mesh-file hash, checks support-ratio and critical-load identities, and confirms all 65 recorded source/input hashes plus the executable stayed unchanged. Those checks passed. The local FEM source is unmodified; only the existing compound thickness wrapper is selected in the audit executable.

| Max area mm² | Retained mesh SHA-256 |
| ---: | --- |
| 4 | `39a22f898ad9d93bf8817cbcc1a2671063bd57ff196c11a27a03ee443ca8158d` |
| 1 | `efc175db527ccfcf999365ab9ba6ac3c84804711bbbf0824d6e5888496eadb2f` |
| 0.25 | `c83d739d5751aa646353b8706f5e532be9c05ee334244d3640d96c9f16cb7689` |

Hashes cover the exact UTF-8 serialized `xy`, `tris`, `te` mesh files, rather than the binary concatenation used by `run_plate_gci_3level`; each `Level` binds to the retained file. Complete native estimate objects, full-precision quantities, configuration, source/input/generated-source hashes, command, environment, binary hash and raw-artifact hashes are in [gci_a4_validation.json](results/gci_a4_validation.json). Raw meshes, logs, source snapshots and executable are in `target/rsv2-07/gci-a4/20261018T053739Z/`.

Focused native GCI tests: **2 passed, 0 failed** (`gci_admits_monotone_positive_order_sequence`, `gci_detects_oscillatory_and_precision_floor`); log: `target/rsv2-07/gci-a4-unit-tests.log`. The prior 14-test certification-integrity pass is historical evidence in `validation.json`; it was not rerun for this diagnostic-only addition. No push or full-workspace gate is claimed.

Reproduce from the repository root:

```sh
taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/gci_a4.py
taskset -c 14,15 env CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 \
  CARGO_PROFILE_DEV_DEBUG=line-tables-only JARAMBIT_SOLVER=faer RAYON_NUM_THREADS=2 \
  TMPDIR="$PWD/target/rsv2-07/compiler-tmp" \
  cargo test --release --locked -p jarambit-crescent --lib gci::tests -- --test-threads=2
```

Each replay uses a fresh timestamped evidence subdirectory inside the existing lane's `target/` and updates the compact validation receipt. It does not overwrite prior raw evidence. The earlier `20261018T053653Z` numerical run is retained separately; its driver stopped on a receipt lookup before producing the final validation, so the recorded verification uses only `20261018T053739Z`.
