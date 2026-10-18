# RSV2-07 Phase 5: witness production-surface 3D verification

**Both requested witness runs completed with converged primary P2 solutions and independent DOLFINx v0.9.0 comparisons.** Results below use the unchanged Phase-4 production-surface protocol. `within_2pct` compares both reported SF observables against the native primary; nodal-max is the conservative strength observable and nodal-average is diagnostic.

| Witness | Primary SF max / avg | DOLFINx SF max / avg | Relative errors vs primary max / avg | within_2pct |
| --- | ---: | ---: | ---: | :---: |
| placement_05 | 2.346 / 3.127 | 2.3465 / 3.1273 | 0.02131287% / 0.00959386% | true |
| Q7 | 2.824 / 3.787 | 2.8232 / 3.7876 | 0.02832861% / 0.01584368% | true |

The comparison is `abs(SF_DOLFINx / SF_primary - 1) <= 0.02` for both observables, exactly as native `cross_check.py`. Primary SFs retain 3 decimal places; independent SFs retain 4. The receipt also records the evaluation pipeline's alternative DOLFINx denominator separately. This is the requested tip protocol, without a 3D mesh-convergence study or an all-gate/manufacturing qualification.

## Exact witness construction and production surface

The assigned CSVs and configuration files were copied byte for byte into each raw evidence directory, along with the corresponding existing `compound.rs` and witness `prototype_pheno.rs`. Source SHA-256s, complete genes and requested/effective configurations are in the JSON receipt. Canonical genes are asserted without clamping changes.

`placement_05` uses the RSV2-08 compound law: d0/d1=0.65/1.1 mm, radial point shape=2, full 6 mm neck and blade core, heel runout=5 mm, solid neck, compound window clearance, unchanged slot-smoothing setting. Q7 uses its unified-study law: d0/d1=0.48/0.65 mm, point shape=3 and lateral factor=2, with the same full neck/core and compound window construction; window belly shift=0. Both use angle=18 degrees and apex=0.175 mm. The two laws were kept distinct.

Witness phenotype sources match local production `pheno.rs` except the explicitly verified existing construction variants: optional neck-slot omission and compound window clearance; Q7 also retains its optional belly-shift code (zero in this configuration). Their exact source snapshots are retained. Production geometry, FEM, materials, load definitions, exclusions and threshold sources were not edited.

The Phase-4 [production_surface.rs](production_surface.rs) is copied unchanged into each isolated harness and calls `export::solid(&p, 1.0)`. The wrapper redirects thickness evaluation to the witness's exact compound law. From `pub fn solid` onward the generated export source is byte-identical to production. Thus the native RS-v2 internal area cap remains **0.25 mm²**, outer DP tolerance **0.03 mm**, and no study-grid points are inserted. Study grid/boundary/export-area settings remain recorded but do not override this production mesher.

Each witness receives its own fresh native plate reference/raster through unchanged [solid_tip_input.rs](solid_tip_input.rs). The exported surface is bound exactly in `d1_doc['export_surface']`, preserving coordinates/connectivity; candidate genes, effective configuration and native tip coordinates agree. The legacy filenames `critical_fem_3_D1.json` and `critical_fem_3_surface_1mm.json` are retained by the Phase-4 adapters inside separate witness directories; their contents belong to the named witness, not the earlier critical_fem_3 genome.

## Solved observations

### placement_05

- Canonical CSV SHA-256: `f49f26c1af148ff321307788dfac384e6ebb1a59773ff0d0e35abccc50f7cdc3`. Exact assigned source: `/home/ubuntu/jarambit-audits/2026-10-18-g16-06-rsv2-08/phase3_candidates/placement_05.csv`.
- Full solid: 49,828 vertices / 99,664 triangles; 47,786 upper facets; census `0/0/1`.
- Tip mesh: **88,848 tets / 406,809 vector P2 DOFs**; 19,160 linear vertices and 116,443 edge midnodes.
- Native primary: `primary_converged=true`, 199 iterations, independently recomputed relative residual **9.59727117488e-11 <= 1e-10**.
- Independent DOLFINx: 244 iterations; KSP reason 2; reported KSP residual norm 1.58647343592e-11. This norm is not relabeled as the primary relative residual.
- Load/clamp checks: 145 loaded vertices, 315 fixed vertices, 882 fixed edges / 3591 fixed displacement DOFs. Net Fz=-2.27374e-13 N, chord torque=1820 N·mm, transverse moment=6.66134e-15 N·mm. Loaded and fixed vertices are disjoint.
- All 88,848 tets have positive finite volume; minimum six-volume=0.00146092578864 mm³. Joint thickness=6 mm, allowance=730.17456 MPa.

### Q7

- Canonical CSV SHA-256: `0871168936d25b66a8bcd849193f63fd09f00185229f1cce8df71b0f43cd7e7a`. Exact assigned source: `/home/ubuntu/jarambit_wt_diag/target/gate-feasibility-bound/unified_study_v3/split_hand_blade_literal_neckdrop_w3_s0.5_0.85_b0_qual.csv`.
- Full solid: 50,514 vertices / 101,036 triangles; 48,549 upper facets; census `0/0/1`.
- Tip mesh: **87,312 tets / 402,813 vector P2 DOFs**; 19,110 linear vertices and 115,161 edge midnodes.
- Native primary: `primary_converged=true`, 186 iterations, independently recomputed relative residual **9.10159195865e-11 <= 1e-10**.
- Independent DOLFINx: 228 iterations; KSP reason 2; reported KSP residual norm 1.3291165866e-11. This norm is not relabeled as the primary relative residual.
- Load/clamp checks: 155 loaded vertices, 235 fixed vertices, 629 fixed edges / 2592 fixed displacement DOFs. Net Fz=2.4869e-13 N, chord torque=1820 N·mm, transverse moment=1.04805e-13 N·mm. Loaded and fixed vertices are disjoint.
- All 87,312 tets have positive finite volume; minimum six-volume=0.00146060602773 mm³. Joint thickness=6 mm, allowance=730.17456 MPa.


The unmodified `fem_solid/tip_solid.py <D1> 30 4 1 <raw>` uses R_CUT=30 mm, NL=4, NSUB=1, PATCH=2 mm and 1820 N·mm pure chord torque. E=113800 MPa and nu=0.34. The original 4–15 mm tip ROI and >3 mm cut-boundary exclusion remain unchanged. Native primary assembly/AMG/CG and the independently recomputed residual criterion are unchanged.

The unmodified `fem_solid/cross_check.py` / `solve_dolfinx.py` uses the exact dumped NPZ: coordinates, linear tetrahedral geometry and P2 displacement field, fixed vertices and edge midnodes, nodal forces, material and allowance. Sidecar hashes match both that mesh and the converged primary result; both files stayed unchanged across the cross-check. The independent solver retains native CG/GAMG, Chebyshev/Jacobi level smoothers and rtol=1e-10. Both runtimes and solver metadata are recorded in full.

## Execution and evidence

Preparation checkout: `a4c008ca48d050907a4b8303d08b95cdc33f0174`. All builds and solves run on **CPUs14,15, nice10, CARGO_BUILD_JOBS=2**, with OMP/OpenBLAS/MKL threads=1. Native primary processes were inspected directly; container runtime metadata confirms CPUs14,15 and nice10. Each primary inherited a 64 GiB address-space bound. Builds reuse this lane's `target/`; no new checkout or production source change was made.

Primary runtime: Python3.12.3 / NumPy2.5.4 / SciPy1.18.1 / scikit-fem12.0.2 / pyamg5.3.0. Independent runtime: DOLFINx0.9.0 / PETSc3.22.0 / NumPy2.0.2, with the same read-only SciPy mount as Phase4. Pinned image: `ghcr.io/fenics/dolfinx/dolfinx@sha256:628f40dd62c7f37962ccadc5a745f23d52c56b2ac3ad9b9b6a86c567c07aa27b`. Exact versions, actual image ID, commands, timings, exit codes, source/binary hashes, raw solver records and consistency checks appear in [phase5_witness_3d_validation.json](results/phase5_witness_3d_validation.json).

Raw evidence: `target/rsv2-07/solid-tip-3d-p5/20261018T064234Z/`, with separate `placement_05/` and `Q7/` directories. Every evaluated external input, generated source and executable was checked unchanged after its run; common scientific sources were checked unchanged after both runs. Surface and mesh/load/clamp consistency checks pass. Full workspace tests and the full certification battery were not run for this diagnostic assignment.

| Artifact | SHA-256 |
| --- | --- |
| placement_05: surface | `08bbad9bcd1d9781d9c8b058ab07f73ead53e274c821463f57e34fac5f562ba8` |
| placement_05: bound context | `fc81aab08a318f830f4dcc5ae4e7bdd3aa7c1fdc84d1aa489615a22e78a5a94b` |
| placement_05: shared mesh | `ac496f3d8643f22695032b88da3608bf28ced845976d2ff998b7eaacf04b72b4` |
| placement_05: primary | `3fd49df6bae745e731eda763fc507e07cffe56ef346d5e799dea1b457a26a998` |
| placement_05: independent | `5fed478b437647f288c4205031cabea0865612c83497e229773ef5f60ac10fcf` |
| placement_05: independent sidecar | `e92fee8489cdc11cc554072ec3b75282973e4f4162533b967fd54cfef7bb8d0a` |
| Q7: surface | `015dd6d791b2cdaa5057dc2c81b3feefdb4fd3bbc56f8b6fa8aebc5837d9c9c2` |
| Q7: bound context | `87e8dfd05d54682e0e19a0711f77ebae6b6789b5393a10818d29e0a87dc6a940` |
| Q7: shared mesh | `37c776a5f03791c8d90b0052a9a1c94e68dfecd9677a7bde1169cdc888a5dba2` |
| Q7: primary | `f07c7a06d47f407dc85c24d12bad581f09ec6624c282bfc94d810c66c19129d6` |
| Q7: independent | `82aa907fb1c34a8b9ab0cf9e266d523e323aca6d503233d0f6675f3b8199e5ea` |
| Q7: independent sidecar | `b11582284bbb889ebd906ca2d02371fb8ab47fb10154cd1efaf4d68eff1fd260` |

Reproduce from the repository root (normal-priority shell):

```sh
nice -n 10 taskset -c 14,15 env CARGO_BUILD_JOBS=2 \
  python3 experiments/rsv2_07/solid_tip_3d_p5.py
```

The [driver](solid_tip_3d_p5.py) uses the exact assigned external sources, snapshots them and refuses timestamp-directory reuse. Geometry-only and software consistency checks supplement the numerical comparisons; they do not establish complete candidate qualification. Earlier plate witness observations and dense-surface sharpness checks are not automatically transferred to these newly sampled production surfaces.
