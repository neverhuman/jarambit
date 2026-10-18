# RSV2-07 Phase 4: production-mesh 3D tip and DOLFINx verification

**The requested production-export tip protocol completes successfully.** The primary P2 solve converged, its conservative nodal-max SF is **2.376 >= 1**, and the independent DOLFINx v0.9.0 result agrees within **0.01263%**, below the **2%** limit. These establish passing G16-12 tip-SF and G16-16 cross-check observations for this prototype protocol. The complete battery and manufacturing qualification were not run.

| Metric | Native scikit-fem P2 | DOLFINx v0.9.0 P2 | Relative difference vs primary |
| --- | ---: | ---: | ---: |
| Nodal-max SF | **2.376** | **2.3763** | **0.012626%** |
| Nodal-average SF | 2.799 | 2.7987 | 0.010718% |
| Displacement DOFs | 399,249 | 399,249 | Identical mesh |
| Iterations | 201 | 241 | Different preconditioners |

`primary_converged=true`; `dolfinx_cross_check.within_2pct=true`. Both nodal-max observations exceed the unchanged SF>=1 threshold at **1820 N·mm (1.82 N·m)**. Nodal-average SF is diagnostic; it does not replace the conservative nodal-max criterion. Values retain the native reporting precision (primary SF: 3 decimals; independent SF: 4 decimals).

## Candidate and exact production export

Candidate: [critical_fem_3.csv](results/critical_fem_3.csv), SHA-256 `070da10ab3c66dced8592aabf4a06ec2c68708a3f92df43e64685bbe6019da15`. The canonical 128 genes, material and recorded compound-section configuration are unchanged: angle=18 degrees, apex=0.175 mm, d0/d1=0.45/0.55 mm, point shape=3, point slope=0.3, radial=true, point lateral factor=2, smooth heel=true and native knot/core thickness.

The dedicated [production_surface.rs](production_surface.rs) invokes the same call as [export_step.rs](../../crates/crescent/examples/export_step.rs#L11):

```rust
let s = export::solid(&p, 1.0).expect("solid");
```

The isolated wrapper changes only thickness evaluation to the existing [compound.rs](compound.rs) law. From `pub fn solid` onward, its source is **byte-identical to production export.rs**, including meshing, bore construction and `surface_json`. Verified body SHA-256: `4113cf7e9fc9ee75f1479b9bd5c8542f3324092fd8f6e77de6c324e3510c0920`. This preserves the prototype section while applying production meshing; loading the CSV alone in an unmodified production CLI still does not apply the compound law.

A provenance correction matters here: the retained earlier `cad.rs` already called `export::solid(&p, 1.0)`. Its study wrapper used the 0.025 mm grid and 0.001 mm boundary tolerance to create the dense source. The retained code/metadata do not support attributing that particular Phase-3 artifact to an `export::solid(&p, 0.0625)` call. Phase 4 uses production DP tolerance **0.03 mm**, native `fine::mesh_domain`, and **no study grid insertion**.

Also, [production export.rs](../../crates/crescent/src/export.rs#L43) internally caps RS-v2 surface area with `max_area.min(0.25)`. Thus the requested **1.0 call argument** yields an effective **0.25 mm² cap**, before NSUB refinement. The filename `critical_fem_3_surface_1mm.json` records the requested call argument; it is not a claim that the internal RS-v2 cap is 1.0 mm². The unchanged compound configuration still records its study mesh knobs, which production meshing does not consult.

The native `surface_json` result is bound exactly as `d1_doc["export_surface"]`. Its vertices/connectivity survive JSON binding unchanged. The inherited plate context matches the prior native input exactly, and its tip coordinate agrees with the new export. The compatibility `_D1.json` remains prototype-only; no certificate identity is fabricated.

| Export / submodel quantity | Phase-4 measured value |
| --- | ---: |
| Full solid vertices / triangles | 48,694 / 97,396 |
| Upper surface facets / referenced vertices | 46,700 / 24,347 |
| Tip component facets before NSUB | 1,815 |
| R_CUT / NL / NSUB / PATCH | 30 mm / 4 / 1 / 2 mm |
| Tetrahedra | 87,120 |
| Linear vertices / edges (P2 midnodes) | 18,820 / 114,263 |
| Vector P2 DOFs = 3 × (vertices + edges) | 399,249 |
| Full-solid census (boundary, nonmanifold, max directed incidence) | 0 / 0 / 1 |
| Full-solid volume mm³ | 29804.8116793 |

The native count is `1815 × 4 × 4 × 3 = 87120` tets. The observed model is smaller than the assignment's approximate benchmark; these measured counts come from the unchanged current exporter/solver. All tet volumes are positive and nonzero; minimum absolute six-volume is 0.00147927815662 mm³.

## Primary P2 result

Unmodified [tip_solid.py](../../fem_solid/tip_solid.py) ran at R_CUT=30, NL=4, NSUB=1 with the exact saved production surface and `DUMP_MESH=solid_mesh.npz`. It uses scikit-fem TET10/P2 elasticity, AMG-preconditioned CG, and the original 4–15 mm tip ROI with >3 mm cut-boundary exclusion. No loads, thresholds, exclusions, resolution or convergence tolerance were changed.

- E=113800 MPa; nu=0.34; native joint thickness=6 mm; joint-adjusted allowance=730.17456 MPa.
- Load patch: 170 vertices; measured torque=1820 N·mm; net Fz=-4.9738e-14 N; transverse moment=-4.08562e-14 N·mm.
- Cut clamp: 350 vertices and 1009 edges, **4077 displacement DOFs**. Load/clamp vertex sets are disjoint.
- Solver: `skfem_p2_amg_cg`, **201 iterations**, independently recomputed relative residual **8.70741889092e-11 <= 1e-10**.
- Nodal-max peak: 307.3 MPa at [189.39, -48.52, -0.15] mm, tip distance 4.09 mm. Nodal-average peak: 260.9 MPa at [189.24, -48.32, 0.21] mm, distance 4.25 mm.
- Native linear solve time: 600.438 s; primary process wall time: 1033.767 s. The default AMG coarse solver performed a 7890×7890 dense SVD; assembly and that coarse solve explain the runtime. Observed resident memory remained below the 64 GiB execution budget.

## Independent DOLFINx result and 2% adjudication

Unmodified [cross_check.py](../../fem_solid/cross_check.py), using [solve_dolfinx.py](../../fem_solid/solve_dolfinx.py), ran numerically on the **same NPZ geometry, tets, clamp, nodal forces, material and allowance**. Sidecar input hashes match both `solid_mesh.npz` and the converged `skfem_raw.json`. Both files stayed unchanged throughout the comparison.

Container: `ghcr.io/fenics/dolfinx/dolfinx@sha256:628f40dd62c7f37962ccadc5a745f23d52c56b2ac3ad9b9b6a86c567c07aa27b` (actual image ID identical). Runtime: DOLFINx 0.9.0, Basix 0.9.0, PETSc/petsc4py 3.22.0, float64, serial MPI. It uses CG/GAMG with Chebyshev/Jacobi level smoothers; rtol=1e-10, atol=1e-50, max_it=5000, preserving the cross-check's native options.

DOLFINx converged in **241 iterations**, KSP reason **2**, reported KSP residual norm **8.24198939894e-12** (PETSc norm type 1; this is not relabeled as the primary's independently recomputed relative residual). Native cross-check exit code is **0**. Independent process wall time: **126.453 s**.

Native cross-check relative errors use `abs(SF_dolfinx / SF_primary - 1)`: **0.000126262626263** (max) and **0.00010718113612** (average), each <=0.02. The evaluation pipeline's alternative DOLFINx denominator gives **0.012625% / 0.010719%**; both also pass. The receipt records both conventions explicitly. Differences compare the native reported SFs, including their documented rounding.

## Execution, verification and retained evidence

All scientific builds and solves used **CPUs14,15**, **nice=10**, **CARGO_BUILD_JOBS=2**, and `OMP_NUM_THREADS=OPENBLAS_NUM_THREADS=MKL_NUM_THREADS=1`. Primary process affinity/priority/environment were inspected directly. The independent launcher used `--cpuset-cpus=14,15` and `nice -n 10` inside the container; runtime metadata confirms the same affinity/priority. Rust used faer with two Rayon threads and reused this lane's existing `target/`.

Primary environment: Python 3.12.3, scikit-fem 12.0.2, pyamg 5.3.0, NumPy 2.5.4, SciPy 1.18.1. Independent image: NumPy 2.0.2, with SciPy 1.18.1 mounted read-only using the repository launch pattern. Actual versions are recorded; the older NumPy/SciPy pins in `fem_solid/requirements.txt` are not presented as this run's environment.

Preparation checkout: `55fe93dc6388219cfe08f9fad044178ba9c85b3c`. All **77 source/input hashes**, generated sources and evaluated export executable stayed unchanged. Native export executable SHA-256: `ab39bc950c8d5bc649af39a0fd8bf10f2403871d05ba2b7ec5266e634cb58064`. Exact surface binding, production-body identity, unchanged plate context, candidate/configuration identities, tet geometry and load/clamp checks passed. Two relevant software controls passed, zero failed; they verify skin refinement and refusal to certify a nonconverged primary. Python syntax and whitespace checks pass. No full workspace test suite or battery invocation is claimed.

Raw evidence: `target/rsv2-07/solid-tip-3d-p4/20261018T061400Z/`. The prior Phase-3 report and receipt are preserved there as `phase3_SOLID_TIP_3D_VERIFICATION.md` and `phase3_solid_tip_3d_validation.json`; the earlier dense CAD and raw attempt remain untouched. The current production-mesh solve has no resource refusal. The dense Phase-3 surface remains a separate protocol with no solved stress observation.

| Artifact | SHA-256 |
| --- | --- |
| `critical_fem_3_surface_1mm.json` | `8bacb8c0c2eeed5e9b383346a83ff3215576e5e15f242a232117f9437276881b` |
| Bound `critical_fem_3_D1.json` | `e42da2377076aa0636bd477465624c025d567aea16a2fcc2ea043be949cf456d` |
| Identical `solid_mesh.npz` | `0c878c9c4773ace10b5b8d073a4c00f6429b4271b7c947deff4c45d986242cf6` |
| Primary `skfem_raw.json` | `d16f864efa20b8e46c052ed8e5a7ddecfd93463081d62f60660a4fb39b2ee5d6` |
| Independent `dolfinx_raw.json` | `c649329c0ff6c9fc61b2ae14f106d950e97b555967b7856198b2213c4652b374` |
| Independent solver/input sidecar | `3a3d36919cfe6117b86c7dda4d80faba0b708ece739bafca78ad9637c67a7be5` |

Full native records, sidecar descriptors/settings, relative differences, input/source/binary hashes, commands, timings, runtime observations and all raw artifact digests are in [solid_tip_3d_validation.json](results/solid_tip_3d_validation.json).

Reproduce from a normal-priority shell at the repository root:

```sh
nice -n 10 taskset -c 14,15 env CARGO_BUILD_JOBS=2 \
  python3 experiments/rsv2_07/solid_tip_3d_p4.py
```

The driver preserves the previous report/receipt, regenerates the production surface with the recorded section law, and runs both native solvers in a fresh timestamped directory. `production_surface.rs` provides the dedicated export step requested by Phase 4.

This is one specified solid-tip protocol, not a solid mesh-convergence study. The newly sampled production facets have not inherited the earlier dense surface's sharpness measurements. Human-max twist closure, fatigue, production failure sequence, plate GCI admission and manufacturing/stock obligations remain open. The A4 pass remains recorded separately. No gate, physical limit or candidate was relaxed, and no all-gate or manufacturing qualification is claimed.
