# RSV2-07 Phase 3: exported-surface 3D tip audit

**Status: numerical verification incomplete due to resources.** The exact exported surface at the requested `NL=4, NSUB=1` produces **8,375,520 tetrahedra and 37,822,275 P2 displacement DOFs**. The native primary was invoked; its last observed stage was basis construction, and it was stopped after confirming that its stiffness COO arrays alone require **112.324 GiB**. No 3D stress result or identical-mesh DOLFINx comparison exists. Neither G16-12 nor G16-16 is established by this audit.

| Requested field | Recorded value | Interpretation |
| --- | --- | --- |
| `solid_tip_sf_nodal_max` | **null** | No solved 3D stress field |
| `solid_tip_sf_nodal_avg` | **null** | No solved 3D stress field |
| `primary_converged` | **false** | Attempt stopped before the linear solve; no residual measured |
| `dolfinx_cross_check.within_2pct` | **null** | Numerical comparison not run; primary SF and shared mesh dump absent |

A missing comparison is distinct from a measured disagreement. The primary stop is an execution/resource outcome, not evidence that the candidate fails a physical SF threshold. The complete machine-readable record is [solid_tip_3d_validation.json](results/solid_tip_3d_validation.json).

## Candidate and geometry binding

- Existing candidate: [critical_fem_3.csv](results/critical_fem_3.csv), SHA-256 `070da10ab3c66dced8592aabf4a06ec2c68708a3f92df43e64685bbe6019da15`. No candidate, stock, section law or genome bound was changed.
- Source facets: `target/rsv2-07/cad/critical_fem_3/prototype.mesh.json`, SHA-256 `7d4b2a513efb69f3fcad7de85f66f86697b248b432e9d3581412be23fcc96ffd`. This matches the prior [CAD manifest](results/cad_manifest.json).
- Native adapter [solid_tip_input.rs](solid_tip_input.rs) supplies the unchanged 0.25 mm² plate context, native tip coordinate and 1 mm compound-law raster. It selects the exact recorded compound configuration through the existing isolated export wrapper. The solid geometry uses actual saved skin heights, not this legacy raster.
- The [driver](solid_tip_3d.py) selects the 597,999 original triangles whose three saved z coordinates are positive, using 304,523 original vertices. Only vertex indices are remapped. Coordinates and triangle connectivity were verified exactly against the source JSON; no facet simplification, thickness resampling or coarsening was applied. Python reads the original CAD to avoid default Rust JSON-reader rounding differences.
- All selected triangles have positive projected area; minimum projected double area is 2.033128821153574e-07 mm². The lower skin is exactly the reflected upper skin. The native plate `xy/tris/te` arrays match the prior GCI mesh exactly. These checks are retained in `surface_validation.json`.
- Native tip xy = (188.313689026131, -52.4655532385621) mm; joint thickness = 6 mm. The compatibility input named `critical_fem_3_D1.json` is explicitly prototype-only, not a D1 certificate. Its companion `critical_fem_3_grind_law.json` supplies only the legacy reader's native tip-coordinate entry; geometry and compound settings are bound separately.

## Requested native protocol and count

The run uses unmodified [tip_solid.py](../../fem_solid/tip_solid.py): R_CUT=30 mm (the repository evaluation default), **NL=4, NSUB=1**, PATCH=2 mm and chord torque=1820 N·mm. Material and joint allowance follow the native script: E=113800 MPa, nu=0.34, yield=828 MPa, joint thin/hot factor from the native 6 mm joint. Stress recovery, if reached, would use its existing 4–15 mm tip annulus and >3 mm cut-boundary exclusion. Its configured primary relative-residual tolerance remains 1e-10.

The source has one connected tip component within R_CUT, containing **174,490 original upper facets**. The native midpoint refinement preserves their piecewise planar heights. Four subtriangles per original facet, four through-thickness layers and three tetrahedra per prism give 174490 × 4 × 4 × 3 = 8375520 tetrahedra.

| Native COUNT_ONLY output | Value |
| --- | ---: |
| Tetrahedra | 8,375,520 |
| Linear vertices | 1,757,165 |
| Edges / P2 midnodes | 10,850,260 |
| Vector P2 DOFs = 3 × (vertices + edges) | 37,822,275 |

The count command completed successfully (exit 0). Its log retains the legacy plate reference SF 4.13698642648787, which matches the prior 0.25 mm² plate result; **that number is not a 3D solid SF**.

## Primary attempt and resource adjudication

All builds, native counting, primary execution, software controls and container checks were pinned to **CPUs 14,15**, with **CARGO_BUILD_JOBS=2**. The actual primary process affinity and inherited 24 GiB address-space limit were inspected and saved in `primary_process_observation.json`. BLAS/OpenMP threads were limited to one; Rust used faer and two Rayon threads.

The unchanged primary ran for **623.638 seconds** and produced no `skfem_raw.json` or `solid_mesh.npz`. A saved stack observation locates it in `Basis(mesh, ElementVector(ElementTetP2()), intorder=2)` at [tip_solid.py:113](../../fem_solid/tip_solid.py#L113), inside affine-gradient mapping. At that observation it had not entered stiffness assembly or the linear solve. The worker then sent SIGTERM to its own process under the recorded resource guard; actual subprocess exit code is **-15**. **No NumPy MemoryError or iterative nonconvergence is claimed.**

The installed scikit-fem assembler allocates 30 × 30 entries per vector P2 tetrahedron: 7,537,968,000 local COO slots. Float64 data plus two int32 index arrays require:

`8375520 × 30 × 30 × (8 + 4 + 4) = 120607488000 bytes = 112.324 GiB`.

This excludes basis arrays, geometry, sparse conversion copies, the assembled matrix and AMG/CG work. The assembler source path/hash is recorded in the receipt. At recording, this host had 125.578 GiB total RAM, 84.667 GiB available RAM and 6.468 GiB free swap. The COO arrays alone exceed the available RAM plus free swap and the process's 24 GiB address-space budget. Continuing the unchanged assembly within that budget cannot succeed. The stop avoided extending a futile allocation attempt; it did not substitute another mesh or a surrogate solver.

The updated driver applies this count-based guard before starting future oversized primary attempts. `RSV2_07_PRIMARY_MAX_AS_GIB` controls the execution budget; a larger budget is meaningful only on a suitably provisioned host. No SF, residual or convergence flag is synthesized to fill missing output.

## Pinned DOLFINx runtime and absent cross-check

The independent image was verified by immutable identity:

`ghcr.io/fenics/dolfinx/dolfinx@sha256:628f40dd62c7f37962ccadc5a745f23d52c56b2ac3ad9b9b6a86c567c07aa27b`.

A fresh container, with `--cpuset-cpus=14,15 --network=none`, imported **DOLFINx 0.9.0**, the repository's `solve_dolfinx.py` and PETSc **3.22.0**. Its NumPy is 2.0.2; SciPy 1.18.1 is mounted read-only from the existing solver environment using the repository evaluation launch pattern. The returned reference model is `dolfinx3d_isotropic_linear_elasticity.cg_gamg.v1`. `dolfinx_runtime.log` records this successful import check; it is **runtime evidence, not a numerical solve or agreement result**.

The primary script writes `DUMP_MESH` only after basis construction, stiffness assembly and load/clamp preparation ([tip_solid.py:141](../../fem_solid/tip_solid.py#L141)). It never reached that point. Consequently [cross_check.py](../../fem_solid/cross_check.py) was **not invoked numerically**: there is no validated identical NPZ mesh and no primary SF against which to compare. `within_2pct=null` and `numerical_solver_invoked=false` preserve that distinction. The required max/average SF comparisons at <=2% remain open.

## Verification, artifacts and reproduction

- Primary environment: Python 3.12.3; scikit-fem 12.0.2; pyamg 5.3.0; NumPy 2.5.4; SciPy 1.18.1. This is the existing `jarambit-sheath` evaluation environment. Its NumPy/SciPy versions differ from the older baseline pins in `fem_solid/requirements.txt`; actual versions are recorded, with no completed baseline-equivalence claim.
- Preparation checkout: `c580b03994edebf9b9ec49b35e124ed247d19482`. The 76 source/input digests and all generated adapter sources/binary were checked. Only the orchestration driver was revised after preparation to record the resource outcome and add the preflight guard; its before/after digests are explicit in the receipt. Scientific solver sources, candidate, CAD and adapter stayed unchanged.
- Adapter binary SHA-256: `8232c9f90f791ea2791571bb27968f8f9a787a74a00addea8f2561111433ac5b`.
- Existing software controls: **2 passed, 0 failed** (`test_refinement_preserves_the_actual_thin_cad_skin`, `test_nonconverged_primary_never_certifies`). These verify software behavior, not physical validation. Python syntax checks also pass.
- All attempt inputs, count/primary/build/runtime logs, resource/affinity/stack observations and source snapshots are retained under `target/rsv2-07/solid-tip-3d/20261018T054942Z/`. The compact receipt includes their SHA-256 digests. Earlier adapter-only attempts remain separate and supply no numerical conclusion.

| Artifact | SHA-256 |
| --- | --- |
| Native compatibility input / actual upper skin | `a1e1feda32046abb53ce581c6f64a3b270cd194001657d44dfc852b4fe55b2cc` |
| Native count log | `fc8d26ded65b304df49ae27e9a47c74ce9a354f419a3ff1061aece1d3f8f7878` |
| Primary attempt log | `7bae99175f33b07debfdb37da2914e3cf76731ec4efe65b5c1aaa5c944f9314a` |
| Exact surface/plate identity checks | `79866b2e03409e868ba539ddc9cc82fbb5d459cf7068e3377d0cf915f9699f0d` |
| DOLFINx runtime import check | `1583205ee24e88769782d9cc78eab4dcb393a617412253471fc72245dca62e16` |
| Software controls | `029593fc3c62ae5ab2d4e36acf46daf3f3d0861c21870a9cc07d4f6e27485537` |

Reproduce preparation and the guarded audit from the repository root:

```sh
taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/solid_tip_3d.py
```

To re-record this retained attempt without repeating the native solve:

```sh
taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/solid_tip_3d.py \
  --record-existing target/rsv2-07/solid-tip-3d/20261018T054942Z
```

The guarded replay writes an explicit incomplete receipt at the requested resolution. A completed numerical audit still requires this exact 37.8-million-DOF protocol on a machine with enough memory for its native basis, assembly and solve, or a separately reviewed solver implementation. No gate/threshold, exclusion, geometry or resolution was relaxed, and no manufacturing or all-gate qualification is claimed. The earlier human-max twist/fatigue/production-sequence deficits and GCI rejection remain; the A4 diagnostic pass is unchanged.
