# Engineering Gates, Multi-Fidelity Physics, and Evolutionary Dynamics in `jarambit`

**Authors:** Jepson Taylor & the `jarambit` Engineering/Assurance Pipeline  
**Document Version:** `1.0.0` — `2026-10-17` (synchronized with [WHITEPAPER](WHITEPAPER.md) and [VERSION](VERSION))

**Scope:** utility-load numerical screening, reproducible search history, and certificate integrity.

## Abstract

`jarambit` couples a constrained titanium CAD genotype to plate elasticity, a local quadratic-tetrahedron solid model, geometric checks, and evolutionary search. This companion extends the existing white paper with an implementation-derived account of every gate and its verification limits. Three distinct denominators matter: **16 W_CAD_v2 engineering gates**, **10 RS-karambit-v2 hard-gate groups**, and **18 CAD battery evaluators applied to 23 subjects, giving 414 recorded cells**. They are not interchangeable pass counts. Selected historical candidates became lighter, but the v1.0.0 audit exposed geometry, measurement, solver-independence, and evidence-identity defects. **There is no currently certified all-16 winner: `champion=null`.** The w283 all-hard claims are withdrawn; any w284–w29x result scored by the rejected pipeline is **PROVISIONAL** until re-certified on fixed main. The merged #84 pipeline retains real independent assessments, including failures. The new Blender atlas depicts its committed failed offspring, with actual per-gate verdicts; it occupies no certified-winner slot.

## I. Architecture and interpretation

The controlling predicate is the `gates_map` in [evaluate_cad_candidate.py::evaluate_candidate](../ops/ci/evaluate_cad_candidate.py); native scorecards are assembled by [battery.rs](../crates/crescent/src/battery.rs) and [score_cad.rs](../crates/cli/src/score_cad.rs). A genome is decoded, exported, measured, screened, re-solved on certification meshes, evaluated by the independent solid pipeline, and summarized. A missing observation cannot establish qualification. A negative evolutionary fitness ranks failures for continued search; it does not admit them through a hard gate.

The native path and standard independent pipeline have different capabilities. Native `solid_tet10` is a numerical screen. Actual independent solver certification uses [evaluate_cad_candidate.py](../ops/ci/evaluate_cad_candidate.py), [tip_solid.py](../fem_solid/tip_solid.py), and [cross_check.py](../fem_solid/cross_check.py). A matching label, a saved scorecard, a successful renderer, or the word “certified” in a console message cannot supply missing evidence. Historical faceted STEP exports were not validated sewn OCC B-reps. Following [PR #84](https://github.com/neverhuman/jarambit/pull/84), merged on main at `ae86f188`, the standard evaluator additionally inspects the actual STEP with OpenCascade, requires one valid solid, and records the analytic bore rolls (§IV-E). These geometry changes do not re-certify earlier observations.

### I-A. Shared mechanics, loads, and fidelity

The crescent implementation is [fem.rs::run_b, certify, run_unbiased](../crates/crescent/src/fem.rs), backed by [physics/fine.rs](../crates/physics/src/fine.rs). Linear isotropic elasticity solves $K u=f$; the plate combines membrane response with Batoz DKT bending. In-plane diagnostics use quadratic T6 triangles. The flexural rigidity is $D=Et^3/[12(1-\nu^2)]$. The default Ti-6Al-4V screening model has $E=113.8$ GPa, $\nu=0.34$, density $4.43$ g/cm³ and provisional 20 °C yield $828$ MPa; these values are code assumptions, not coupon measurements for a manufactured part. The temperature law in [physics::Material::yield_at](../crates/physics/src/lib.rs) is linear, $\sigma_y(T)=\sigma_y(20)[1-0.0011(T-20)]$ for this alloy.

D1 applies simultaneous stock and temperature knockdowns:

$$k_p(t)=\left(\frac{t-0.1\ {\rm mm}}{t}\right)^p\frac{\sigma_y(100\ ^\circ{\rm C})}{\sigma_y(20\ ^\circ{\rm C})},\qquad C_{\rm robust}=C_{20}k_p.$$

The code uses $p=2$ for bending/torsion capacity and $p=1$ for membrane ring pull. This conservative multiplicative proxy is not a re-mesh of every manufacturing realization. Certification takes conservative metrics over maximum triangle areas **0.25 and 0.125 mm²**, with no search bias; the strict solve uses **0.0625 mm²**. Search meshes are coarser and configurable (2 mm² in the normal evaluator; 4 mm² in the retained w282 experiment). Area is not edge length. These tiers do not provide the same accuracy.

For lateral loading the bore nodes are clamped in all five plate DOFs, with a unit transverse force distributed over a 2 mm tip patch. A 4 mm tip stress exclusion isolates the load singularity. For ring pull, the guard/ricasso band $x\in[76,100]+\Delta x$ mm is seated and a unit $-x$ bore traction is integrated. First-yield capacity is $C_{20}=\sigma_y(20)/\hat\sigma_{\rm vm}$ for a unit-load stress peak. Chord torsion loads slope DOFs along $n=(-d_y,d_x)$, perpendicular to the ring-to-tip chord $d$. The thick rectangular section correction is $k_c=(1/3)/\alpha(b/t)$, implemented by `alpha_rect` and `torsion_3d_factor`; it is applied to plate torsion, not to a separately solved solid model.

PR #82 removes the pre-removal-field substitution and global ring-width strength credits. The current post-#84 [export::thickness_at](../crates/crescent/src/export.rs) evaluates the explicit cutting-face law $t_g(q)=\min(e+2d_{in}\tan\theta_{in},e+2d_{sp}\tan\theta_{sp})$ from actual serrated boundaries, using apex gene $e$, edge distances $d$ and half-angle genes $\theta$. Within the grinds it uses $t_g$; elsewhere it uses the post-removal field, then applies the local bore-collar construction (§IV-E). The positive floor remains **0.05 mm** in RS-v2 mode; legacy mode uses $\max(t_{field},0.30\ {\rm mm})$. `fem::run_b` averages four evaluations per triangle (centroid and the three centroid-to-vertex midpoints) for element thickness. The independent solid instead preserves the saved exported facet heights through refinement. Consistent geometry does not eliminate plate-model error or turn a local submodel into a whole-part qualification.

### I-B. Reading the verification citations

Each gate below names its exact Boolean predicate, computation, retained evidence, and verification limits. File links denote implementations or reusable test procedures; they do not assert that those tests ran for every candidate. The frozen T00–T19 catalog is mapped in §IV-C using its actual contents. In particular T01 concerns GCI, T02 claim honesty, T03 checked stochastic reductions, T04 search contracts, and T05 field/representation semantics. Proposed entries and software controls supply no physical validation. No per-gate physical test coverage is inferred from a T-family label.

## II. The 16 W_CAD_v2 engineering gates, grouped by what they test

### II-A. Geometry and manufacturability

#### 1. `cad_watertight_and_single_solid` — exported topology and mass consistency

**Predicate and units.** `cad_info.cad_valid` requires the STEP header, an explicit millimetre length-unit declaration (`has_mm_decl`) in the comment/string-stripped `DATA;` section, exactly **one** `MANIFOLD_SOLID_BREP`, at least **one** closed shell, STL **zero boundary edges**, **zero non-manifold edges**, maximum directed-edge repeat **one**, relative solid/raster mass difference **≤0.01** (1%), and byte-for-byte deterministic export regeneration (`regeneration_deterministic` across `.step`, `.stl`, and `_surface.json`). In the post-#84 independent evaluator it also requires OpenCascade `BRepCheck_Analyzer` validity and exactly **one** transferred solid. Lengths are mm. At Ti-6Al-4V density, $m_{3D}=0.00443 V$ grams for volume in mm³; $V=|\sum a\cdot(b\times c)|/6$ on oriented surface triangles.

**Computation.** [export.rs::solid, census, volume](../crates/crescent/src/export.rs), [evaluate_cad_candidate.py::verify_step_and_stl, evaluate_candidate](../ops/ci/evaluate_cad_candidate.py), [verify_genome_export.py::inspect_step](../ops/ci/verify_genome_export.py), and native [cad_ingest.rs::mesh_census_and_volume](../crates/crescent/src/cad_ingest.rs). Mesh census and volume consistency alone do not prove absence of every triangle self-intersection or valid sewing; the independent OCC observation is an additional check on the STEP, not a deduction from its entity names.

**Verification.** CAD ingest/export unit controls and [test_cad_battery.py](../ops/ci/tests/test_cad_battery.py) (`test_evaluated_genome_spec_binds_step_stl_surface_and_fails_closed`); retained `*_cad_eval.json:/cad` binds actual STEP/STL/surface hashes, mass observations, and battery `geom.unit`, `geom.one_solid`, `geom.mass`, and `geom.pin` verdicts. The #84 proof retains `*_step_check.json` with one valid OCC solid. T02 governs whether those observations may support a claim; it is not a topology solver. The #75 regression `plate_and_export_use_actual_thickness_not_the_thicker_machining_base` additionally checks geometry consistency. `previous_model_certificates_never_bind_to_the_new_surface_model` rejects reuse from the old geometry regime.

[Blender topology plate](figures/whitepaper/blender/cad_watertight_and_single_solid.webp).

#### 2. `geometric_and_holder_gates` — legal geometry and assumed holder compatibility

**Predicate and units.** `len(rs_doc.rejects)==0`, ultimately [gates.rs::check](../crates/crescent/src/gates.rs). Each of tip-to-heel, horizontal reach and sharpened arc is **≥85.725 mm**; each cutting-edge arc is **≤101.6 mm**. Grip length is **60–80 mm**; ring ID **25.4±0.5 mm**; ring-wall ligament **≥4.5 mm**; other ligaments **≥3 mm** (P1 may require the wider rail). Stock is **5.3–6.0 mm** in **0.1 mm** increments. Both edges require ≥3 teeth and ripple **≥0.05 mm**; primary edge angles are **≥14°/side**. Titanium, a non-self-intersecting connected outline, clear flat brand pad and valid feature containment are required.

**Holder model.** `holder_fit` scans rotations −30°…30° in 0.5° increments against **200×100 mm**. Stack $t+2w+2c\le9$ mm with $c=0.2$ mm, insertion **3–80 N**, draw **1.5–45 N**, cairn retention feature **≥4 mm**, friction land **≥40 mm²**. [redteam::holder_forces](../crates/redteam/src/lib.rs) uses $F_{in}=6.6w^3\delta$ N, derived from a 2.2 GPa cantilever proxy; $F_{draw}=0.55F_{in}$. Additional decoded failures and the sanity mass interval **20<m<200 g** also reject. This broad sanity interval is separate from the soft 80 g ranking target.

Here $t$ is nominal body stock and $w$ is the holder wall. The implemented stack check does not substitute the maximum height of #84's locally raised bore collar for $t$. A true gate flag therefore does not establish that the entire changed export fits a real sheath or can be machined from the nominal body-stock blank. The holder plate shows only the declared plan envelope and insertion direction.

**Verification.** `gates.rs`, red-team tests, phenotype tests in [crescent/src/tests.rs](../crates/crescent/src/tests.rs), and battery `geom.*` records. T02 covers scoped claims and T19 prevents a missing required execution being counted as success. No measured insertion/draw curve or directional retention validation is supplied by the scalar holder proxy.

[Blender holder/envelope plate](figures/whitepaper/blender/geometric_and_holder_gates.webp).

### II-B. Ergonomics and appearance

#### 3. `karambit_ring_and_curved_claw` — ring and hooked-outline archetype

**Predicate.** The conjunction in `evaluate_candidate`: ring ID **25.4±0.5 mm**, ring wall **≥4.5 mm**, C.6 ring-face groove width **≥2.0 mm**, hook drop **≥25 mm**, minimum of the three blade-length measures **≥85.725 mm**, ≥3 teeth on each edge, and aesthetic pass. The implemented comparisons allow $10^{-6}$ mm around the stated ring-ID/archetype length limits (`+1e-6` on ID deviation; `-1e-6` on minimum lengths); `gates::check` separately uses $10^{-9}$ mm on ring ID. These numerical comparisons are stated rather than silently changed. The reported inside-concave/spine-convex fractions contribute to the score; the code applies reference continuity limits through the aesthetic evaluator, not through an invented fixed 95% gate.

**Computation and science.** [pheno.rs::build](../crates/crescent/src/pheno.rs) decodes geometry; [aesthetic.rs::evaluate, continuity](../crates/crescent/src/aesthetic.rs) samples curvature; `evaluate_candidate` reads export metadata and D1. This is a deterministic shape/retention-interface proxy, without FEM or anthropometric sampling. Ring ID does not establish comfortable pressure distribution, and groove width does not establish the bore-edge radius required by RS-v2.

**Verification.** Reference-seed/feature tests in `crescent/src/tests.rs`, explicit `/karambit_archetype` observations in `*_cad_eval.json`, and exported-mesh measurements from [certification.rs::measure](../crates/crescent/src/certification.rs). T02's truthful-negative controls address reporting; the catalog supplies no executed human-fit study for this gate.

[Blender ring plate, with an illustrative 22 mm gauge](figures/whitepaper/blender/karambit_ring_and_curved_claw.webp).

#### 4. `c6_aesthetic_floor` — reference geometry and required features

**Predicate.** `mt.aes_pass`, from [aesthetic.rs::evaluate](../crates/crescent/src/aesthetic.rs). Symmetric Hausdorff distance **≤6 mm**; normalized curvature-histogram EMD **≤0.06** (32 bins); longest nearly straight run **≤reference+8 mm**, with straight defined by $|\kappa|<1/250$ mm$^{-1}$. Minimum concave blend radius is **≥min(2 mm, 0.8×reference minimum)**. Convex-spine and concave-inside sample fractions are **≥reference−0.03**. Thumb-seat depth **≥0.4 mm**, guard-lobe prominence **≥5 mm**, hook drop **≥25 mm**, neck-fillet gene **≥8 mm**. Wrap length **≥45 mm** and texture condition, flat pad, jimp scale **0.8–1.2** (0.36–0.54 mm nominal depth), and active C.6 groove width **≥2 mm** with groove area **>50 mm²** are also checked.

**Computation.** Outline resampling and curvature statistics are geometric screens. The soft score $A=e^{-H/6}e^{-EMD/0.05}[1-0.5\min(\Delta L/8,1)]$ is separate: **A≥0.50 is not the hard predicate**. A score near 0.408 can pass every actual sub-check.

**Verification.** Reference-feature tests in `crescent/src/tests.rs` and D1 `metrics.aes_fails`, Hausdorff and EMD fields provide inspectable diagnostics. There is no frozen T-family that establishes human aesthetic approval; automated acceptance and owner sign-off remain separate.

[Blender C.6 plate](figures/whitepaper/blender/c6_aesthetic_floor.webp).

### II-C. Structural strength

The following eight gates share §I-A's material law and geometry. Their load capacities and safety factors are predictions of specified linear-elastic models, not physical proof loads.

#### 5. `cert_hiker_180n` — transverse utility-load capacity

**Predicate:** `lat_rob_cert_n >= 180.0` N, equivalently $SF_h=C_{lat}/180\ge1$. `fem::certify` takes conservative lateral capacity over **0.25/0.125 mm²** ring-clamped DKT meshes, using the unit out-of-plane tip patch, excluded 4 mm load zone, and $k_2$ joint thin/hot factor. It tests bending strength under the lower lateral utility load, not impact response.

**Verification:** [physics/fine.rs](../crates/physics/src/fine.rs) elasticity tests, [physics/tests/solver_parity.rs](../crates/physics/tests/solver_parity.rs), and candidate `*_D1.json`/`*_cad_eval.json` fields `lat_rob` and `sf_hiker`. T05 concerns field/representation contracts, with execution scope checked separately; T01 addresses mesh evidence. Calibration residuals and lack of physical coupons are discussed in §IV.

[Blender hiker plate](figures/whitepaper/blender/cert_hiker_180n.webp). Its color field is the saved multi-load D1 envelope, labeled accordingly; the arrow identifies this gate's fixture/load.

#### 6. `cert_hunter_260n` — higher transverse utility-load capacity

**Predicate:** `lat_rob_cert_n >= 260.0` N; $SF_H=C_{lat}/260\ge1$. The solver, support, mesh pair and knockdown are the same as gate 5; the required capacity is higher. There is no independent second material model behind these two Booleans. `fem::certify` in [fem.rs](../crates/crescent/src/fem.rs) supplies both values to `evaluate_candidate`.

**Verification:** unit-load linearity, solver parity, actual-export thickness regression (#75), and the `lat_rob_cert_n`/`sf_hunter_cert` fields in each CAD evaluation. The 40-design T1→T2 calibration in [t1_t2.json](calibration/t1_t2.json) has a historical lateral residual sigma of 6.5%; that is not a per-design uncertainty bound. T01/T05 concern numerical contracts; neither supplies physical strength validation.

[Blender hunter plate](figures/whitepaper/blender/cert_hunter_260n.webp).

#### 7. `strict_hunter_260n` — transverse strength on the strict mesh

**Predicate:** `strict_lat_rob_0p0625_n >= 260.0` N. [bench.rs::rs_v2_eval](../crates/cli/src/bench.rs) and the D1 CLI call `fem::run_unbiased` at **0.0625 mm²**, then apply $k_2$. Boundary conditions and 260 N floor remain unchanged. Refining the same model probes mesh sensitivity; it does not add a new load case or remove geometric/model uncertainty.

**Verification:** strict D1 rescreen CSV plus bound `*_D1.json` and `*_rs.json`; T01 mesh-sequence/observable-identity controls; #75's export-field consistency regression. W283's fixed value **176.720 N** for p14 is materially below the unchanged floor despite the earlier 262.6 N claim. A valid independent solid comparison cannot repair this plate failure.

[Blender strict-lateral plate](figures/whitepaper/blender/strict_hunter_260n.webp). The displayed stress mesh is the retained certification mesh, not a claimed new strict solve.

#### 8. `cert_ring_pull_1500n` — seated ring-traction capacity

**Predicate:** `ring_rob_cert_n >= 1500.0` N. A T6 membrane solve seats the guard/ricasso band and integrates unit $-x$ traction on the bore. Capacity derives from the hand-side nodal von Mises peak outside the excluded seated region, using $k_1$ rather than the flexural $k_2$. [fem.rs::run_b, certify](../crates/crescent/src/fem.rs) conservatively combines the certification meshes.

**Verification:** consistent edge-traction implementation in `physics/fine.rs`, solver/representation tests, `*_D1.json:/metrics/ring_rob`, and `*_cad_eval.json:/engineering_metrics/ring_rob_cert_n`. T05's balance/invariance controls address numerical semantics; no physical ring proof test is inferred. This 1500 N static requirement differs from RS-v2's provisional 3000 N ring-back static equivalent and from finger-bearing comfort.

[Blender ring-pull plate](figures/whitepaper/blender/cert_ring_pull_1500n.webp).

#### 9. `twist_body_1p82nm` — chord torsion away from the tip

**Predicate:** `twist_body_sf >= 1.0` at **1.82 N·m = 1820 N·mm**. A ring-held tip torque is applied about the chord. The body stress peak excludes the inner **15 mm** around the tip; its own local rectangular-section $k_c$ and $k_2$ give $SF=T_{y,body}/1820$. This is a body-region metric of the same tip-loaded solve, not a second body-load fixture.

**Computation:** `fem::run_b` constructs `excl15`, obtains `kc_body`, and reports `body_j3`; `certify` combines meshes. **Verification:** chord-axis load tests and section correction in `physics/fine.rs`/`crescent/src/fem.rs`, saved D1 body/hand/tip diagnostics, and #76's `rs_mode_does_not_create_ring_width_strength_or_fatigue_credits`. T05 addresses load/field semantics; certification is required because historical torsion calibration misses its residual bar (§IV-B).

[Blender body-torsion plate](figures/whitepaper/blender/twist_body_1p82nm.webp).

#### 10. `twist_tip_only_1p82nm` — tip-loaded torsional peak

**Predicate:** `twist_tip_sf >= 1.0` at **1.82 N·m**. The name is retained from the scorecard: it tests the corrected peak from the tip-loaded chord-torsion solve outside the **4 mm** load singularity exclusion. It is not an independently resolved 3D point test. [fem.rs::run_b](../crates/crescent/src/fem.rs) computes the unit-torque stress, walks the local section in 0.05 mm increments to obtain $b/t$, applies $k_c$ and $k_2$, and the D1 writer exposes `twist_tip_sf`.

**Verification:** unit-load/rotation tests, explicit slope-DOF torque direction, historical T1/T2 torsion calibration, D1 `twist_tip_*` values and actual-export regression. T01/T05 offer numerical contracts only. A point can pass this plate metric yet fail solid-tip SF, measured edge thickness or point monotonicity.

[Blender tip-loaded torsion plate](figures/whitepaper/blender/twist_tip_only_1p82nm.webp).

#### 11. `twist_tip_submodel_strict_1p82nm` — strict-mesh tip annulus torsion

**Predicate:** `twist_tipzone_sf_strict >= 1.0` at **1.82 N·m**, evaluated at **0.0625 mm²**. The peak region is the **4≤distance<15 mm** tip annulus. The strict unbiased solve obtains that annulus's local $k_c$ and thin/hot factor. Despite the historical “submodel” name, this gate is a restricted plate observable; gate 12 supplies the independent 3D local model.

**Computation and verification:** `fem::run_unbiased`/`run_b` (`tipzone_ex`, `kc_tz`), D1 strict reports and `rs_v2_eval`; T01 requires consistent regions/observables across mesh levels, T05 addresses representation semantics. Missing strict data is unverified. A certification-mesh picture does not demonstrate strict convergence, and a strict-mesh pass does not establish point sharpness.

[Blender strict-tip-annulus plate](figures/whitepaper/blender/twist_tip_submodel_strict_1p82nm.webp).

#### 12. `solid_tip_3d_p2_1p82nm` — local three-dimensional tip strength

**Predicate:** `solid_tip_sf_nodal_max >= 1.0` under **1.82 N·m**, using the recorded fine-mesh result. [tip_solid.py](../fem_solid/tip_solid.py) and [solve_dolfinx.py](../fem_solid/solve_dolfinx.py) solve isotropic 3D elasticity, $\sigma=2\mu\epsilon+\lambda\operatorname{tr}(\epsilon)I$, with all cut-face displacement DOFs clamped. The 1.82 N·m chord torque is applied through **minimum-norm vertex z-forces within a 2 mm tip patch**, across the thickness layers: $\sum F_z=0$, $\sum sF_z=1820$ N·mm and $\sum qF_z=0$, where $s$ and $q$ are the in-plane chord-normal and chord coordinates relative to the tip. This balances net force and the unwanted orthogonal moment. The local radius is **30 mm**; the standard saved refinement is **NL=4, NSUB=1**, with quadratic P2 tetrahedra. The reported nodal-maximum SF is the joint thin/hot allowable divided by the recovered von Mises maximum in the **4≤tip distance<15 mm annulus**, with **cut-boundary distance>3 mm**. `solve_dolfinx.py::tip_roi_mask` fixes these region semantics. No plate rectangular-section correction is applied to the 3D stress. Nodal maximum, nodal average and element mean are distinct observables.

**Computation of the geometry.** The post-#84 route loads the saved full-precision exported facets through [export_surface.py::load_surface, refine](../fem_solid/export_surface.py). Refinement interpolates the actual vertex heights; the local solid is not reconstructed from the 0.5 mm display raster. [tip_solid.py](../fem_solid/tip_solid.py) saves the fine mesh used by both solvers. The equations, loads, ROI and strength floor above remain unchanged.

**Verification:** [verify_analytic.py](../fem_solid/tests/verify_analytic.py) benchmarks, `test_tip_regression.py`, `test_field_semantics.py`, and actual saved `*_solid_mesh.npz`, `*_skfem_raw.json`, `*_dolfinx_raw.json`, `*_solid_tip.json`. [test_evolve_certifiable.py](../ops/ci/tests/test_evolve_certifiable.py) includes thin-skin refinement, nonconverged-primary rejection, changed raw-observation rejection and unavailable-runtime controls. [test_evolve_committed_evidence.py](../ops/ci/tests/test_evolve_committed_evidence.py) checks retained raw observations and hashes without claiming another solve. T05 covers observable/projection contracts; T01 covers admissible convergence. PR #82's independently observed w283 p14/p10 SF **0.9577/0.9578** fails. Native TET10 alone is unverified for independent certification; no native result is relabeled as two solvers.

[Blender solid-tip load illustration](figures/whitepaper/blender/solid_tip_3d_p2_1p82nm.webp); no unsaved solid stress field is synthesized.

### II-D. Stiffness and stability

#### 13. `side_a4_buckling_and_support` — compliant support and linearized buckling

**Predicate:** A4 `status == 'pass'` **and** `buckling_eigenvalue >=1.0`. `status` itself requires positive eigenvalue and supported sequence **≥the certified sequence**. [side_a4.py::evaluate_subject](../ops/ci/side_a4.py) and [physics/side_a4.rs::solve_side_a4](../crates/physics/src/side_a4.rs) add radial/tangential bore springs **50,000/5,000 N/mm per boundary node**, flange stiffness **100,000 N/mm**, and grip Winkler stiffness **200 N/mm³** times tributary area. The former paper's 50 N/mm³ value was stale.

The linearized eigenproblem is $K_{support}\phi=\lambda_{cr}G\phi$, with compression-positive geometric stiffness assembled for the 260 N reference load; $P_{cr}=260\lambda_{cr}$ N. Python uses sparse factorization/eigensolution; Rust uses sparse Cholesky and K-orthogonal Lanczos. This is an elastic buckling/support diagnostic, not nonlinear collapse, contact validation or permission to replace the clamped failure-sequence gate.

**Verification:** retained `*_a4.json` (support coefficients, sequence, eigenvalue and critical load), native/Python parity fixtures in `battery.rs`, T05 numerical semantics, and T01 where mesh studies are available. The compliant-support model lacks physical hand/pin stiffness calibration; the node-based bore stiffness is itself mesh dependent.

[Blender compliant-support plate](figures/whitepaper/blender/side_a4_buckling_and_support.webp).

### II-E. Failure sequence

#### 14. `failure_sequence_1p2` — hand-side yield must follow blade-side yield

**Predicate:** `seq_ratio_cert >=1.20`, dimensionless. [fem.rs::run_b](../crates/crescent/src/fem.rs) partitions blade $x\ge x_{heel}$ and hand $x<x_{heel}-3$ mm, excluding the 4 mm tip patch. For common material and unit load, $C_{hand}/C_{blade}=\hat\sigma_{blade}/\hat\sigma_{hand}$. It takes the worst ratio across lateral, chord torsion and in-plane pry; `certify` retains the conservative value over the mesh pair. Pry is a sequence diagnostic without a separate W_CAD load-floor gate.

**Verification:** recorded `seq_case`, hand/blade field diagnostics, `*_plate_gci.json` and D1/RS records. T01 controls region/observable identity and extrapolation; T04 distinguishes legal, feasible and ranked designs. This elastic yield-order proxy does not simulate crack initiation, fracture propagation, or safe real-world failure. A4 supported sequence is reported separately and cannot override this clamped-fixture predicate. W283 p14/p10 re-certification gives **0.7436/0.7179**, below 1.20.

[Blender failure-sequence plate](figures/whitepaper/blender/failure_sequence_1p2.webp).

### II-F. Numerical verification

#### 15. `plate_gci_valid_3level` — mesh/observable metadata predicate

**Exact implemented predicate:** the sequence-ratio GCI record must have truthy **`sequence_valid` and `observable_valid`**. This particular W_CAD Boolean does **not** test a maximum GCI percentage or require `fit_status == admissible_estimate`. Consequently, its name must not be read as proof of an asymptotic error bound. The battery's `plate.gci` reports the fuller convergence status separately.

**Computation:** [plate_gci_probe.rs](../crates/crescent/examples/plate_gci_probe.rs) and [gci.rs::run_plate_gci_3level](../crates/crescent/src/gci.rs) supply three **4, 1, 0.25 mm²** meshes, corresponding to edge-size ratio $r=2$, to [evaluate_cad_candidate.py](../ops/ci/evaluate_cad_candidate.py) / [fem_solid/gci.py::compute_gci](../fem_solid/gci.py), hashing the actual solved `fem::report_fields` triangulation (`mesh_hash(&fl.xy, &fl.tris, &fl.te)`) at each refinement level rather than a separate re-mesh. For a monotone valid sequence, $p=\ln|\epsilon_{32}/\epsilon_{21}|/\ln r$ and $GCI_{fine}=1.25| (f_1-f_2)/f_1 |/(r^p-1)$. The frozen implementation refuses unsupported sequences and distinguishes oscillation, unresolved/precision-limited behavior and nonpositive order. Three levels alone do not establish asymptotic validity.

**Verification:** `fem_solid/tests/test_gci.py`, `test_gci_evidence.py`, `test_plate_gci_hashes_the_solved_mesh_not_a_separate_remesh` in [test_cad_battery.py](../ops/ci/tests/test_cad_battery.py), actual T01's 18 catalog entries, and saved candidate mesh/observable identities. Several historical scorecards show this W_CAD flag as true while the battery records oscillatory GCI. We retain that discrepancy explicitly; no threshold or gate code is changed in this paper update.

[Blender mesh plate](figures/whitepaper/blender/plate_gci_valid_3level.webp).

#### 16. `dolfinx_cross_check_2pct` — independently observed solver agreement

**Predicate:** the independent record's recomputed `within_2pct` is true; the unchanged relative comparison limit is **0.02 (2%)**. The gate recomputes $e=|SF_{skfem}-SF_{DOLFINx}|/SF_{DOLFINx}\le0.02$ for **both** declared nodal-maximum and nodal-average observations; inputs must be finite and positive, with bound mesh identity and NL=4/NSUB=1. The comparison uses the **same saved mesh**, with independent scikit-fem P2 and pinned DOLFINx v0.9.0 execution. [certification.rs::independent_pass](../crates/crescent/src/certification.rs) rejects native/missing/mislabeled records; [cross_check.py](../fem_solid/cross_check.py) retains the distinct solver observations. Stresses, projections, material law, load and constrained region must agree before comparison is meaningful.

**Verification:** `fem_solid/tests/test_dolfinx_verification_record.py`, field-semantics/representation controls, #79's `native_or_missing_independent_solver_never_becomes_a_pass`, and hashed mesh/raw artifacts in [cert_integrity/recertified](../evidence/cert_integrity/recertified/). T05 concerns field semantics; T02 concerns honest claims; T19 distinguishes reuse from fresh execution. Agreement confirms implementation consistency for that problem. Both solvers can agree on a failed strength gate or share an incorrect physical assumption.

[Blender independent-solver illustration](figures/whitepaper/blender/dolfinx_cross_check_2pct.webp).

## III. RS-karambit-v2 and the 414-cell CAD battery

### III-A. Ten RS-v2 hard-gate groups

The authoritative writer is [bench.rs::rs_v2_eval](../crates/cli/src/bench.rs), with requirements in [RS-karambit-v2.toml](../requirements/RS-karambit-v2.toml). All ten groups must be observed and true for `all_hard=true`; missing observations enter `unverified`. These groups overlap W_CAD and add requirements; they are not ten extra disjoint load cases.

| Group | Actual requirement / computation |
|---|---|
| `aesthetic_and_geometric` | Aesthetic pass and empty geometric rejects (§II-A/B). |
| `floors_828_joint` | Joint thin/hot lateral ≥180/260 N and ring pull ≥1500 N (`Fem.floors_ok`). |
| `twist_floor_body_and_tipsub_strict` | Body and strict 4–15 mm tip-annulus SF ≥1 at 1.82 N·m. |
| `solid_tip_sf` | Content-bound independently measured solid-tip SF ≥1. |
| `strict_hunter_lateral` | Strict 0.0625 mm² lateral ≥260 N. |
| `seq_hard` | Certified yield-sequence proxy ≥1.20. |
| `human_max` | Twist **16.2 N·m**, distributed spine **3427 N**, ring-back **3000 N**, each SF ≥1; measured bearing width **≥10 mm** and bore-edge radius **≥1.5 mm** on both faces. |
| `fatigue` | Modified Goodman SF **≥1** for lateral cycling **0…180 N**, $R=0$, $10^7$ cycles (L7). |
| `gate_s_sharp_law` | Measured cutting-edge apex **≤0.20 mm** and half-angle **≤25°** over the first **0.4 mm**, on the exported solid. |
| `gate_s_tip_point_brep` | Measured thickness at 0.25/1/2 mm behind point **≤0.30/0.60/1.00 mm**, taper length **≥5 mm**, monotone non-decreasing thickness over the full **0–10 mm** profile. |

The **1000 N in-plane tip** case is solved and reported, but is **record-only**. Ring-back's factor-two static equivalent is provisional pending measured impact data. The prose header of `evolve.sh` contains different human-max/fatigue/sharpness values; those comments are stale and are not the specification. This paper uses the unchanged TOML/Rust constants above and does not modify the owner's pipeline files.

For fatigue [fem.rs::fatigue::goodman_sf](../crates/crescent/src/fem.rs) uses $S_{ut}=895$ MPa, $S'_e=0.5S_{ut}$, $k_a=4.51S_{ut}^{-0.265}$, $k_b=\min(1,1.24\max(t,2.79)^{-0.107})$ and $k_e=0.814$, giving $S_e=k_ak_bk_eS'_e$. For the $R=0$ load, $\sigma_a=\sigma_m=\sigma_{max}/2$ and $n=[\sigma_a/S_e+\sigma_m/S_{ut}]^{-1}$. No unsupported bore-width multiplier reduces the stress. Marin/Goodman is an engineering screening model, not measured titanium endurance or a life prediction validated on the finished geometry.

[certification.rs::thickness, sharpness, point_profile, bore_radius](../crates/crescent/src/certification.rs) intersects exported triangles. Point monotonicity includes surface breakpoints over the entire interval; a missing exact 95%-of-core-thickness crossing is a measured **lower bound**, with exact `taper_mm=null`, not a made-up 10 mm length. Point direction is the bisector of the inside/spine edge directions sampled at least 2 mm from the point. `point_profile` defines taper as the first crossing of **0.95×joint thickness**, interpolated between measured surface breakpoints; monotonicity uses the exact non-decrease comparison with no downward allowance. `sharpness` computes each local half-angle as $\arctan(|\Delta t|/(2\Delta s))$ over the first 0.4 mm, using exported edge stations within 0.031 mm of the declared cutting curves. A wide sharp-edged bore measures zero radius; an unestablished positive fillet radius cannot pass. #77/#78/#81 regressions include positive thin-edge controls, reversals after 5 mm, sub-former-tolerance reversals, and wide but unrounded bores.

### III-B. Eighteen evaluators, 23 subjects

[manifest.json](../evidence/cad_battery/manifest.json) enumerates **23 subjects** and these **18 unique case IDs**. Their Cartesian product has **414 cells**. The matrix preserves `pass`, `fail`, `blocked`, and `incomplete`; a missing capability is never a passed structural test. `solid.tip` can report that a solver record is complete while its SF fails the physical strength requirement; read case `kind`, metrics and `gate.rs_v2` together.

The subjects comprise **19 genomes and four hand-built SWITCHBACK reference CADs: `c10a`, `c93`, `c92`, and `c10c`**. Those four are **not genome or Jarambit pipeline results**; their labeled rows retain existing reference records. In the `solid.tip` column, **P means record complete, not SF ≥1 or independent validation**. The two w283 rows retain P with SF 0.9577/0.9578. The [c10a solid-tip record](../evidence/cad_battery/c10a.json) reports SF 0.2101 and copy-identical primary/comparison SF values; its stored `dolfinx_cross_check` is **not an independent cross-check**. That reference record supplies neither a strength pass nor independent certification.

| Evaluators | What their records establish |
|---|---|
| `build.analytic`, `build.gci_contracts`, `build.claims`, `build.dolfinx` | Software or runtime capability contracts, with their own execution/admission status. |
| `geom.unit`, `geom.one_solid`, `geom.mass`, `geom.pin` | Units, topology, mass consistency and bore geometry observations. |
| `plate.loads`, `plate.seq`, `plate.l7`, `plate.gci` | Plate capacities, failure-sequence proxy, Goodman fatigue and mesh-study status. |
| `side.a1`, `side.a2`, `side.a3`, `side.a4` | Local stress/zone diagnostics, alternative support and elastic buckling; diagnostic values do not override certified gates. |
| `solid.tip` | Local quadratic-solid solve record and its metrics, including independence metadata. |
| `gate.rs_v2` | Full observed RS-v2 conjunction; missing inputs prevent qualification. |

![The 23-subject matrix separates 19 genomes from four hand-built SWITCHBACK reference CADs](figures/engineering_gates/fig_battery_23.png)

*Figure 1. All 414 recorded cells, regenerated by [battery.py](../ops/render/paper/battery.py). The four labeled rows above the separator (`c10a`, `c93`, `c92`, `c10c`) are hand-built SWITCHBACK reference CADs, not genome or Jarambit pipeline results. In `solid.tip`, P means record complete, not SF ≥1; c10a's copy-identical comparison is not an independent cross-check. Every cell is traced by JSON pointer in [battery_points.json](figures/engineering_gates/battery_points.json); the PNG manifest hashes the 23 source records. Historical record reuse and corrected w283 observations remain distinguishable from a fresh certification. The earlier 17-subject illustration is superseded here.*

A full catalog command uses all 23 explicit IDs, or the CLI's default catalog; [score_cad.rs](../crates/cli/src/score_cad.rs) documents the actual flags:

```sh
cargo run --release --locked --bin jarambit -- cad-battery --all --out-dir target/paper/battery
```

This paper's numerical tables reuse committed records; the command is a reproduction entry point and is not a claim that this documentation run re-solved all 23 subjects.

### III-C. Retained RS-v2 measurement table: `RS-karambit-v2` Detailed Gate Breakdown Across Evolved Genomes (`evidence/rs_v2/`)

The existing paper’s detailed RS-v2 table is retained below. Pre-w283 rows are historical observations under their recorded model; w283 rows are the corrected rejected records from PR #82. These scalar fields do not fill missing bore/edge/solver evidence, and none establishes an RS-v2 all-hard pass. The six top-level named-genome records in the main paper are a subset of this broader retained evidence.


| Genome | `w20_hard_pass` (`RS-karambit-v1`) | `solid_tip_sf` ($\ge 1.0$) | `seq_ratio` ($\ge 1.20$) | `hm_twist_sf` ($16.2\text{ N}\cdot\text{m}$, $\ge 1.0$) | `hm_spine_sf` ($3427\text{ N}$, $\ge 1.0$) | `hm_ringback_sf` ($3000\text{ N}$, $\ge 1.0$) | `l7_goodman_sf` ($10^7$ cyc, $\ge 1.0$) | `ring_bearing_w` ($\ge 10.0\text{ mm}$) | `RS-karambit-v2` Overall Verdict |
|:---|:---:|---:|---:|---:|---:|---:|---:|---:|:---:|
| `g3_tip+00_t60_tt3` | `FAIL` | **1.6003** | 0.9591 | 0.1321 | 0.4189 | **2.6602** | 0.9823 | 5.88 | `FAIL` |
| `w271b_11772` | `FAIL` | 0.8337 | 0.8823 | 0.0757 | 0.4178 | **2.4188** | 0.8557 | 5.11 | `FAIL` |
| `w271b_11860` | `FAIL` | 0.8140 | 0.8918 | 0.0750 | 0.3841 | **1.9114** | 0.7560 | 4.75 | `FAIL` |
| `w271c_11522` | `FAIL` | **1.0330** | 0.7784 | 0.0965 | 0.4194 | **1.9923** | 0.7673 | 4.79 | `FAIL` |
| `w272_s2902_c01` | `FAIL` | 0.8750 | 1.0000 | 0.0782 | 0.4299 | **2.1738** | 0.8651 | 5.19 | `FAIL` |
| `w273_s2903_c04` | `FAIL` | 0.8750 | 1.0088 | 0.0782 | 0.4299 | **2.1738** | 0.8575 | 5.19 | `FAIL` |
| `w273_s2904_c01` | `FAIL` | 0.8750 | 1.0012 | 0.0782 | 0.4299 | **2.1134** | 0.8650 | 5.11 | `FAIL` |
| `w274_s2905_c01` | `pass` | **1.3040** | **1.2730** | 0.1257 | 0.3854 | **2.8210** | 0.7478 | 6.00 | `FAIL` |
| `w275_s2909_c01` | `pass` | **1.2967** | **1.2056** | 0.1250 | 0.3844 | **2.3951** | 0.7666 | 5.90 | `FAIL` |
| `w276_s2913_c01` | `pass` | **1.3393** | **1.2129** | 0.1289 | 0.3934 | **2.3115** | 0.7675 | 5.60 | `FAIL` |
| `w277_s2919_c01` | `pass` | **1.3393** | **1.2183** | 0.1289 | 0.3934 | **2.3056** | 0.7631 | 5.60 | `FAIL` |
| `w278_s2922_c01` | `pass` | **1.3819** | **1.2229** | 0.1233 | 0.4245 | **2.2292** | 0.7638 | 5.37 | `FAIL` |
| `w279_s2928_c01` | `pass` | **1.3866** | **1.2022** | 0.1262 | 0.4301 | **2.1658** | 0.7662 | 5.29 | `FAIL` |
| `w280_s2931_c01` | `pass` | **1.3866** | **1.2016** | 0.1262 | 0.4301 | **2.1731** | 0.7635 | 5.30 | `FAIL` |
| `w281_s2932_c01` | `pass` | **1.4094** | **1.2115** | 0.1282 | 0.4318 | **2.1559** | 0.7414 | 5.30 | `FAIL` |
| `w282_p10_c02` | `pass` | **1.4094** | **1.2150** | 0.1282 | 0.4318 | **2.1458** | 0.7414 | 5.26 | `FAIL` |
| `w282_p14_c01` | `pass` | **1.4093** | **1.2010** | 0.1281 | 0.4315 | **2.1458** | 0.7319 | 5.26 | `FAIL` |
| `w283_p10_c02` | `fail` | **0.9578** | **0.7180** | **0.1110** | **0.4317** | **2.8596** | **0.5431** | **10.20** | `fail` (`RS-karambit-v2`, measured export) |
| `w283_p14_c01` | `fail` | **0.9577** | **0.7440** | **0.1110** | **0.4313** | **2.8595** | **0.5442** | **10.20** | `fail` (`RS-karambit-v2`, measured export) |

## IV. Science verification and certificate integrity

### IV-A. Analytic benchmarks and solver semantics

Verification asks whether a code solves its specified equations. Validation asks whether those equations predict the intended physical part. The archived [verify_0.30.2.json](../evidence/fem_solid/verify_0.30.2.json) records two analytic checks implemented by [verify_analytic.py](../fem_solid/tests/verify_analytic.py): Saint-Venant torsion of a square bar, $\theta'=T/(0.140577Ga^4)$, and a thin plate with a hole against the finite-width Heywood/Kirsch stress concentration $K_t=[2+(1-d/W)^3]/(1-d/W)$. The archived results satisfy the recorded **1%** error bar. [test_dolfinx_verification_record.py](../fem_solid/tests/test_dolfinx_verification_record.py) checks those saved observations; running that reader is not a fresh analytic FEM execution.

A separate archived DOLFINx/scikit-fem tip comparison uses the same 422,955-DOF problem and a **2%** agreement bar. Plate backend parity compares skyline and faer with their actual rejection/drift limits; [issue32_drift.json](../evidence/bench/issue32_drift.json) reports maximum drift above the historical $10^{-9}$ bar. Equal pivot thresholds do not ensure equal rejection on differently ordered factorizations. These are implementation checks with specific fixtures, not universal error bounds.

Stress recovery is part of the observable: von Mises of averaged stress is not necessarily average von Mises, a nodal maximum is not an element mean, and a changed excluded region creates a different comparison. [fem_solid/tests/test_field_semantics.py](../fem_solid/tests/test_field_semantics.py), [test_representation_invariants.py](../fem_solid/tests/test_representation_invariants.py), and [assurance/numerical/observable_catalog.json](../assurance/numerical/observable_catalog.json) make those distinctions explicit. Tests requiring the independent DOLFINx/Basix/PETSc runtime must fail closed when the capability is absent; catalog/metadata controls do not substitute for operator execution.

### IV-B. Mesh convergence, calibration and robustness

Roache's GCI is an uncertainty estimate conditional on a meaningful refinement sequence and a consistent observable [R2]. A three-point curve fit can produce a number without demonstrating that the solution is in the asymptotic range. [gci.py](../fem_solid/gci.py) retains distinct states for oscillatory, nonpositive-order, precision-limited, unresolved and unsupported sequences; missing mesh/observable identity prevents admission. The finest-grid estimate must not be substituted for an unobserved physical error bound. In particular, the `plate_gci_valid_3level` gate's implemented metadata check (§II-F) is weaker than a positive convergence finding. Historical “all 16” flags and the battery's oscillatory plate GCI can coexist; the paper reports both.

The archived [T1/T2 calibration](calibration/t1_t2.md) evaluates 40 designs from `bench/fixed_1k`. Lateral, ring and pry residual sigmas are **6.5%, 5.5%, 3.3%**; torsion is **10.8%**, above the declared **7%** bar. Even its fitted alternative remains above the bar. The documented policy therefore gates torsion on certification rather than calibrated search values. This calibration belongs to its recorded source/model regime; it does not validate the altered v1.0.0 geometry substitution, nor automatically calibrate post-#82 main.

Monte Carlo in [monte_carlo/src/lib.rs::perturb, env_sample, mix_seed, try_run_cols](../crates/monte_carlo/src/lib.rs) samples manufacturing perturbations, temperature **−10…100 °C**, and a specified utility-media mixture. Thickness uses a Gaussian perturbation with standard deviation **0.03 mm**; the environmental sampler assigns 25% probability to band extremes and otherwise samples the interval. Sample identity derives from `(seed,index)`, allowing the sample multiset to remain invariant to thread scheduling. Checked reducers in [stats.rs](../crates/monte_carlo/src/stats.rs) reject empty/nonfinite/incomplete inputs; joint scenarios preserve paired outcomes rather than multiplying marginal pass rates. T03's exact-n, quantile, correlation and replay controls test these software semantics.

These probability statements are conditional on the assumed input distributions and correlations. A p05 estimate is not a certified reliability level without population representativeness and sampling uncertainty. The deterministic D1 joint thin/hot capacity is not itself a full Monte Carlo reliability analysis. Neither procedure establishes fatigue life, impact response, human comfort or manufacturing-process capability without the corresponding physical data.

### IV-C. What the T00–T19 assurance catalog actually contains

The frozen source is [TEST_CASE_CATALOG.json](v1/science/tests/TEST_CASE_CATALOG.json), pinned by [release-v1.json](../assurance/policies/release-v1.json). It has **178 entries**, each requiring positive and adverse controls. **356 required control slots is not 356 executed, passing controls.** The catalog's recorded status strings describe 11 historical executed T01 entries, 12 T02 entries requiring independent runtime admission, and 155 proposed entries. Later test modules and receipts must be assessed on their own identities and execution evidence; those static strings are not this run's execution totals. [assurance_inventory.json](figures/engineering_gates/assurance_inventory.json) is regenerated directly from the catalog.

| Family | Entries | Actual catalog subject and its relevance / limit |
|---|---:|---|
| T00 | 11 | Inventory, strict records, receipt/control completeness and scoped findings. |
| T01 | 18 | GCI arithmetic, mesh sequence, observable identity, cache and singularity contracts. |
| T02 | 12 | Claim honesty, candidate/requirement/physics identity, revoked review and truthful negatives. |
| T03 | 10 | Checked stochastic reductions, joint correlation, replay, population shift and provenance. |
| T04 | 12 | Feasibility/ranking, novelty context, archive behavior, checkpoint and lineage contracts. |
| T05 | 12 | Numerical field semantics, representation invariance, units, balance and solver/operator controls. |
| T06 | 10 | Benchmark accounting, equal budgets, splits, telemetry and audit independence. |
| T07 | 13 | Specimen/material/history identity and unapproved evidence transfer. |
| T08 | 6 | Download/units/registration and observation provenance controls. |
| T09 | 6 | Latent-state/observation identifiability and matched controls. |
| T10 | 5 | Graph/representation identifiability, physical rotation, shuffle and resolution controls. |
| T11 | 7 | Sequence/order, cycle/time and duration controls. |
| T12 | 5 | Observation arms, creation/erasure, drift and nuisance controls. |
| T13 | 7 | Process versus film, coupling, topology and segmentation controls. |
| T14 | 9 | Independent truth, silent damage, nuisance and instrument response. |
| T15 | 6 | Strain versus damage, witness/self-test validity and warning persistence. |
| T16 | 6 | Recovery/closure claims, failure migration and mechanism discrimination. |
| T17 | 6 | Model domain, transition error, common ensemble bias and rate sensitivity. |
| T18 | 7 | Competing models, nonidentifiability, independent audit and budget controls. |
| T19 | 10 | Required executions, CI path/capability omissions, custody, historical reuse and final policy. |
| Total | **178** | Frozen requirement inventory; execution and physical findings require their own receipts. |

The earlier paper incorrectly mapped many of these families to knife-specific topics and claimed 100% execution. For example, T02 is claim honesty, not a manufactured-solution plate test; T16 does not by itself establish knife-edge retention. [science_suite.py](../ops/ci/science_suite.py) implements T19 orchestration/custody controls, not a monolithic fresh execution of every catalog entry. T19-08 specifically prevents render/finish metadata from becoming structural properties. An illustrated anodized surface therefore contributes no material-strength evidence.

### IV-D. The #75–#81 audit and PR #82 correction

The independent audit was performed against released main `1e93e6f`; [PR #82](https://github.com/neverhuman/jarambit/pull/82), merged at `4cd0f994e803`, fixes certificate integrity and withdraws the affected claims. The audit observations and the final re-certification use different physical kernels; intermediate issue numbers must not be mixed with the final corrected values.

| Finding | Defect in the rejected pipeline | Correction and adverse control |
|---|---|---|
| [#75](https://github.com/neverhuman/jarambit/issues/75) | Plate/native solid used thicker pre-removal `field.base`, while CAD/mass used sharpened, pocketed geometry. | Shared post-removal export thickness law and physical floor; actual-thickness regression. |
| [#76](https://github.com/neverhuman/jarambit/issues/76) | Bore width globally multiplied remote strength/fatigue and substituted a body peak for required tip torque. | Remove unsupported algebraic credits and restore specified peaks; local bore changes cannot magically strengthen a remote section. |
| [#77](https://github.com/neverhuman/jarambit/issues/77) | 0.18 mm gene passed although exported cutting edges exceeded the accepted apex cap. | Measure apex/angle on exported triangle intersections; positive thin-edge and adverse blunt-edge controls. |
| [#78](https://github.com/neverhuman/jarambit/issues/78) | “B-rep” point test queried a raster and skipped monotonicity over 5–10 mm. | Measure the actual export at all relevant breakpoints through 10 mm; reject later and small reversals. |
| [#79](https://github.com/neverhuman/jarambit/issues/79) | One native solve was copied into two solver slots with literal zero error and a true independence flag. | Distinct actual scikit-fem/DOLFINx observations on the identical saved fine mesh; native/missing independence stays unverified. |
| [#80](https://github.com/neverhuman/jarambit/issues/80) | Subject labels and adjacent stale records authorized reuse after editing a genome. | Canonical genome/model content identities plus D1/mesh/raw/payload hashes; edited-genome and label-collision tests. |
| [#81](https://github.com/neverhuman/jarambit/issues/81) | Available ring width stood in for an unmeasured bore-edge radius. | Measure exported bore geometry; zero/unverified radius cannot satisfy the unchanged ≥1.5 mm criterion. |

[cert_integrity.rs](../crates/crescent/tests/cert_integrity.rs) contains the named regression controls. [certification.rs::identity](../crates/crescent/src/certification.rs) binds material and every exact IEEE-754 genome bit to implementation/requirement content, avoiding display-CSV precision collisions. A label may change without changing content identity; a numerical source or genome change invalidates reuse. Identity hashes protect custody, not scientific truth: they cannot make a wrong equation correct.

The real post-fix comparison agrees to less than 0.04% in the reported SF observations while both tip strengths fail. Re-certification retains the **91,008 quadratic tetrahedra / 416,583 DOFs** fine mesh and distinct raw solver outputs for each w283 candidate. [model-provenance.json](../evidence/cert_integrity/model-provenance.json), [solver-verification.json](../evidence/cert_integrity/solver-verification.json) and [unchanged-requirements.json](../evidence/cert_integrity/unchanged-requirements.json) record the measured regime and unchanged requirements. The final [identity-upgrade.json](../evidence/cert_integrity/identity-upgrade.json) is explicitly metadata-only: it retains original observations and `solve_rerun=false`, refusing changed numerical sources, measurements, inputs or raw artifacts. No additional independent solve is claimed for re-issued bindings.

Both original genomes are rejected. Old false-pass certificates remain in [v1.0.0-retracted](../evidence/cert_integrity/v1.0.0-retracted/) solely for audit. No currently certified all-gates winner exists; the CAD battery has **0/23 RS-v2 all-hard passes**. G1 completeness means its required scalar inputs exist, not that the candidate passes requirements. The correction table below retains PR #82's full-precision RS-based G1; CAD-eval summaries preserve their rounded D1 inputs.
<!-- GEN:CERT_INTEGRITY:BEGIN -->
**v1.0.0 certification claims withdrawn (#75–#81).** Neither Wave 283 candidate passes the fixed pipeline. The exported geometry and all requirement thresholds are unchanged. Native-only solid solves are unverified; the results below use actual scikit-fem P2 and DOLFINx solves on the same mesh.

| candidate | raster / solid g | strict lateral N | cert seq | solid-tip SF | G1 | W_CAD | RS-v2 |
|---|---:|---:|---:|---:|---:|---:|---|
| `w283_p14_c01` | 75.382 / 75.199 | 176.720 | 0.7436 | 0.9577 | 0.623888 | -3.878704 | FAIL |
| `w283_p10_c02` | 75.642 / 75.505 | 176.763 | 0.7179 | 0.9578 | 0.621632 | -3.964924 | FAIL |

Source: `evidence/rs_v2/w283_ultrarunner/*_{cad_eval,rs,solid_tip}.json`; raw solver outputs and exact meshes are retained in `evidence/cert_integrity/recertified/`. The retracted certificates are preserved under `evidence/cert_integrity/v1.0.0-retracted/`. Lateral strength, failure sequence, human-maximum loads, fatigue, exported edge sharpness and full 0–10 mm point monotonicity remain blockers; the bore has no verified required fillet. Historical 16-gate screen results below do not establish an RS-v2 all-hard pass or manufacturing promotion. G1 above uses the full-precision RS inputs in each battery record; CAD-eval JSON retains its rounded D1 inputs.
<!-- GEN:CERT_INTEGRITY:END -->
![Numerical verification and the corrected rejected w283 designs](figures/engineering_gates/fig_verification_and_retraction.png)

*Verification plate. Archived analytic benchmark errors and g3 refinement observations are separated from the corrected w283 measurements. The independent solvers agree on tip SF below 1, while strict lateral capacities are below 260 N. These are saved observations, not fresh solves or a new GCI admission. [verification.py](../ops/render/paper/verification.py) regenerates the plot; [verification_points.json](figures/engineering_gates/verification_points.json) traces every plotted observation.*

### IV-E. Post-#84 exported geometry and independently retained failures

[PR #84](https://github.com/neverhuman/jarambit/pull/84) makes the ordinary genome pipeline capable of constructing and independently assessing its changed export. It does not declare a winner. [export.rs](../crates/crescent/src/export.rs) constructs cutting faces from the existing apex and edge-angle genes; [pheno.rs::bore_dimensions, bore_collar_thickness](../crates/crescent/src/pheno.rs) uses the existing radial wall $w$ for a roll radius $r=w/4$ and a straight axial bearing length $b=2w$. [step_export.rs::write](../crates/crescent/src/step_export.rs) retains analytic toroidal rolls on both faces and a cylindrical bearing; the STL tessellates the same circular surfaces. A larger collar changes local geometry, not remote strength through an algebraic multiplier. Gene bounds and all requirement thresholds remain unchanged.

The standard evaluator independently reads the STEP with [OpenCascade](../ops/ci/verify_genome_export.py), then refines the saved exported top-surface facets for the local solid. It resolves and records the DOLFINx image identity, retains both raw observations on the identical fine mesh, and checks that the summaries agree with the raw data, that the primary solver converged, and that exact genome/model/D1/mesh bindings match. The [regression controls](../ops/ci/tests/test_evolve_certifiable.py) reject changed raw values even when a caller supplies matching hashes, reject a nonconverged primary, preserve thin facets during refinement, and fail closed when independent execution is unavailable. Content hashes provide custody; their presence alone is not numerical validation.

The [committed proof manifest](../evidence/evolve_certifiable/manifest.json) names `cert_pipeline_r01_p14_c02` and its STEP/STL, surface, fine mesh, raw outputs and runtime. Its assessment passes **10/16 W_CAD predicates**, with **RS-v2 `all_hard=false`** and **11/18 battery records passing**. These are three different counts. The actual measured solid SF is **0.669** in the rounded scikit-fem raw record and **0.6693** in DOLFINx; strict lateral capacity is **168.2 N**, strict tip-twist SF **0.529**, and L7 fatigue SF **0.4718**. Its valid STEP has bore rolls on both faces, but measured radius **1.3002 mm <1.5 mm**. Independent agreement and a valid CAD solid do not repair these physical shortfalls. The saved [leaderboard](../evidence/evolve_certifiable/automatic/leaderboard.json) therefore reports **assessed=1, qualified=0, `champion=null`**.

The collar construction reaches $b+2r=2.5w\approx13.002$ mm for this example's radial wall, despite nominal body stock of **5.9 mm**. This distinction matters for stock selection and holder clearance; the nominal-stock stack predicate in §II-A does not validate those manufacturing details. No machining or holder-fit claim is supplied by the images.

The proof also illustrates §II-F's distinction: `plate_gci_valid_3level=true` records valid sequence/observable metadata, while its saved sequence fit has **nonpositive observed order**, `admitted=false` and no GCI error estimate; the hunter sequence is oscillatory. This true gate flag is not an admitted asymptotic bound. The actual `*_plate_gci.json` is identified in the proof manifest.

This proof was executed before #84 merged and was committed with that change. [execution.json](../evidence/evolve_certifiable/execution.json) records the numerical kernel `e8cb049`, exact command/environment and executable/source hashes; the later broadened search operator was not used in that run. No post-merge wave, reissued historical certificate, additional solve or certified record is inferred. The **23×18 catalog remains 414 cells**: this separate automatic assessment does not silently create a 24th catalog subject. The four hand-built reference rows retain their explicit labels and limited scope in §III-B.

## V. Automated evolution and historical population behavior

### V-A. Two search mechanisms with different roles

The legacy general search includes NSGA-II in [crates/evolve](../crates/evolve/src/lib.rs) [R3]. The retained crescent runs use [qd.rs::run_islands_with](../crates/crescent/src/qd.rs): tournament selection on scalar fitness, Gaussian mutation, crossover, archive parents, machining kicks and ring migration. They should not all be described as a single NSGA-II run. The descriptor has **16 dimensions** (`DESC_N`), covering removal fractions, shape zones, thickness variation, hook drop and internal machining features. Novelty is mean distance to the $k$ nearest aesthetic-pass descriptors; rejected appearance does not earn novelty. The MAP-Elites cell uses relief thresholds `[0.005, 0.01, 0.02, 0.03, 0.045, 0.065, 0.09, 0.12, 0.16, 0.21]` and mass index `floor((m−72)/2)` clamped to 0…10. This code is not the earlier paper's supposed 10×12 uniform grid.

Each generation retains up to three fitness elites per island. Two-way tournaments choose ordinary parents; a nonempty archive supplies the first parent with probability 0.3. Crossover-plus-mutation is selected with probability 0.5; otherwise the child is mutated. When enabled, an additional machining kick occurs with probability 0.04. Migration sends the best configured migrants from island $i$ to replace the worst in island $(i+1)\bmod k$ at the configured interval. IDs, parent IDs and operator tags preserve ancestry; retained elites/migrants are appearances, not additional fresh evaluations.

`qd::fitness` gives infeasible candidates negative values with distinct aesthetic, geometry, absent-FEM, load and sequence/torsion shortfall branches. Feasible search fitness under `JARAMBIT_ALL_GATES=1` uses `w_cad_v2_feasible`: 70% mass, 12% shape, 10% survival, 5% sequence and 3% carry terms. **This search function uses plate/novelty proxies**: its survival mix differs from the full scorecard and it lacks an independently observed solid-tip or A4 buckling term. The final [compute_weighted_cad_score](../ops/ci/evaluate_cad_candidate.py) uses actual full-evaluation inputs. Enabling an environment variable cannot turn every birth into a full 16-gate certification. G1 remains the separately locked cross-wave metric in [GLOBAL_METRIC.md](GLOBAL_METRIC.md).

### V-B. `./evolve.sh`: family proposals, Stage 1 and Stage 2

The wrapper builds the release CLI and export/probe examples, checks their Cargo fingerprints, sets the FEM/SF-map environment and forwards options to [auto_evolve.rs::run_auto_evolve_cli](../crates/cli/src/auto_evolve.rs). On main after merged #87 (with defaults unchanged from `ae86f188`), its defaults are **2 rounds**, **12 candidates per elite per round**, **2 Stage-2 candidates**, seed **2830**, and at most **16 worker threads**. Seed paths default to the two w283 and two w282 retained genomes; a fallback C.6 seed is available. A default seed is not an endorsed certified parent. [EVOLUTION_CERTIFICATION.md](EVOLUTION_CERTIFICATION.md) specifies solver Python, cadquery-ocp and the pinned independent Docker runtime.

`spawn_offspring_batch` initially retains the RS-v2 parents (a legacy non-RS-v2 seed first receives a 0.18 mm apex gene) and proposes six alternating operator families: pocket/rail/neck or machining adjustments; taper/fuller adjustments; edge/bevel adjustments; thickness knots; outline control points; and combined Gaussian variation or blended crossover. Perturbation scale decreases as $0.35/\sqrt{round+1}$. The merged operator also samples existing edge-angle/ring-wall bounds in the first round and occasionally applies a broader mutation with $\sigma=0.08$, samples stock thickness within existing bounds and toggles the existing P1 architecture. Other combined mutations use the smaller round-scaled magnitude. All offspring are clamped to the existing genotype domain; no external or hand-built CAD supplies a search parent. These are family-centered proposals, not the island migration algorithm above. The retained proof predates the later broad-mutation addition (§IV-E).

**Stage 1: valid-geometry provisional ranking.** `screen_candidate_stage1` requires RS-v2 mode and a returned [qd::evaluate](../crates/crescent/src/qd.rs) FEM result, which is available only after aesthetic acceptance and an empty geometry-reject list. It uses a **2 mm²** search mesh. Unlike the earlier baseline, it retains measured physical failures. It ranks by $D=\sum_j\max(0,1-v_j/f_j)$ over lateral, ring pull, sequence, body/tip twist, human-max twist/spine/ring-back, fatigue, bearing width and bore radius; $v_j$ and $f_j$ use matching units. Lower shortfall comes first, then lower mass. This scalar is neither G1 nor the full W_CAD score. Coarse screening supplies no independent solid certificate, strict-mesh verdict or full-population 16-gate pass rate.

**Stage 2: standard independent assessment.** `certify_candidate_stage2` writes the exact genome under `by-content/<identity>/attempt-<id>/` and calls [score_cad.py](../ops/ci/score_cad.py), which invokes the standard [evaluate_candidate](../ops/ci/evaluate_cad_candidate.py) path. This runs strict plate, GCI, A4, measured exported geometry, RS-v2 and actual scikit-fem/DOLFINx fine-mesh solves. Selection gives distinct families first choice, then fills remaining slots with distinct candidates. `JARAMBIT_SOLVER_PYTHON` selects the required interpreter. Missing runtime, failed execution, nonfinite fields or incompatible content identity retains an **unverified** report, never an inferred pass.

Completed assessments, including physical failures, stay in the leaderboard. Merged [PR #86](https://github.com/neverhuman/jarambit/pull/86) preserves qualified genomes. Following [PR #87](https://github.com/neverhuman/jarambit/pull/87), `replacement_elites` then takes one best-ranked distinct genome per available seed family before filling provisional slots to four where possible. It deduplicates before counting; more than four available families keep their representatives, so the per-parent proposal budget can increase. If neither source supplies a parent, it retains the previous parents. Family labels derive from seed filenames; preserving them does not guarantee phenotypic diversity or feasibility. Qualification requires **both** `all_16_gates` and `rs_v2_all_hard`; only that conjunction can populate `champion`. A zero process exit reports completed requested assessments, not a winner. `--top-k-certify [N]` enables the independent top-k path (`--certify-top N` remains an alias). A coarse native `score-cad` result alone remains unverified for independence.

A current reproduction entry point is:

```sh
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=line-tables-only \
  JARAMBIT_SOLVER_PYTHON=/path/to/solver-environment/bin/python \
  nice -n 10 ~/bin/memcap 32G -- ./evolve.sh \
  --rounds 2 --candidates-per-round 12 --top-k-certify 2 \
  --threads 4 --seed 20261017 --prefix paper_replay \
  --seeds evidence/rs_v2/w282_ultrarunner/w282_p14_c01_genome.csv,evidence/rs_v2/w282_ultrarunner/w282_p10_c02_genome.csv \
  --out-dir target/paper/replay
```

This documents actual flags; it does not claim this paper update ran a new search, replayed the pre-merge proof under its original operator, or produced a winner. The proof's exact retained argv uses **one round, six candidates, top-k 1, two threads, seed 20261017**, one w283 p14 seed and prefix `cert_pipeline`. The wrapper's stale prose load constants are superseded by §III-A. W284–w29x output produced under the rejected v1.0.0 regime stays **PROVISIONAL** until the actual exported candidate is re-certified on fixed main.

### V-C. Reproducibility and the retained w282 experiment

[w282 provenance](../evidence/rs_v2/w282_ultrarunner/provenance.json) records source commit, numerical-source and binary hashes, seed genomes and hashes, solver/container versions, environment and exact parameters: seed **2934**, **4 islands ×32 population**, **16 generations**, maximum search triangle area **4 mm²**, migration interval **4**, kick fraction **0.2**, **8 threads**, and **6** certification selections. Environment includes `JARAMBIT_ALL_GATES=1`, `JARAMBIT_CAD_BATTERY=1`, `JARAMBIT_CERT_STRICT=1`, and lateral/ring search bias overrides **1**. The committed `search/` files, rather than a replay under changed main, are the observations plotted here. An earlier failed biased probe is separately identified in provenance and must not be silently pooled with the retained run.

The legacy engine's [evolve/src/lib.rs::save_ckpt,load_ckpt](../crates/evolve/src/lib.rs) preserves exact floating-point gene bits and island RNG state, tested by [replay.rs](../crates/evolve/tests/replay.rs). This checkpoint capability is not evidence of an unimplemented crescent checkpoint/resume path. [crates/evidence/src/lib.rs](../crates/evidence/src/lib.rs) frames zstd-compressed JSONL events with magic, length and checksum. Genome precision, source/model identity, sample/RNG seed, initial parents, configuration and stop condition all matter. A seed number alone cannot reproduce a changed implementation. The #82 evidence identity upgrade likewise cannot stand in for a solve under changed physics.

### V-D. Population mass and fitness trajectories

![Retained birth-level w282 mass and fitness, and earlier island champion histories](figures/engineering_gates/fig_population_trajectories.png)

*Figure 2. Panels (a,b) reduce all **1,984 unique fresh evaluations** in [all_evals.json](../evidence/rs_v2/w282_ultrarunner/search/all_evals.json) by generation: minimum/median mass and maximum/median recorded search fitness. These are evaluated births including rejected candidates, not a reconstructed survivor population. Panels (c,d) use committed w6/w13/w15/w16/w272/w273 `history.csv` files: at each generation the highest-fitness island champion supplies the paired mass/fitness point. Those files do not contain population medians, so none are drawn. Earlier fitness scales belong to their original objectives and are not pooled into one comparable score.*

The w282 birth sample contains **128 initial evaluations plus 16×116 offspring**, matching four islands with three retained elites each. Its best mass among all births need not be feasible. The [search summary](../evidence/rs_v2/w282_ultrarunner/search_summary.json) records **27 strict-certified retained selections**; the strict passers did not beat the baseline. The eventual lighter w282 named candidates arose from controlled probes of committed w281 genomes, not from an invented smooth improvement of the GA champion.

### V-E. Per-gate pass rates and the missing-data boundary

![Sixteen gate rates over retained selected candidate scorecards](figures/engineering_gates/fig_gate_pass_rates.png)

*Figure 3. All 16 W_CAD predicates over the retained w274–w283 candidate evaluations. Each column reports its actual **n selected candidates**; these are **selected-sample rates**, not population success probabilities. Historical w274–w282 flags retain their original scoring regime. W283 uses corrected re-certification and cannot appear as a passing record. The point ledger records the exact source and `/weighted_cad_score/gates/<gate>` pointer for every observation.*

![Search-condition rates for all retained w282 births](figures/engineering_gates/fig_population_screen_rates.png)

*Figure 4. Birth-level search conditions in w282, including appearance, geometry, lateral/ring floors, sequence and tip-zone twist. All retained births form each denominator; an absent solve is counted as **not demonstrated**, with that convention recorded. These are coarse-screen conditions; strict mesh, full GCI and independent solid certification were not performed for every birth. Their full-population 16-gate rates are unavailable, rather than imputed from the elite scorecards.*

There is no committed population telemetry for every project wave. W20–w25 retain selected-winner summaries, several w27x runs retain histories or archives, and many w274–w281 entries retain only finalists. Missing generations/waves are left missing. No uncommitted `out/`, shared-checkout scratch, fabricated intermediate mass, or rejected-pipeline w284–w29x result is admitted into these plots.

### V-F. Genotype and descriptor diversity

![w282 pairwise genotype spread, relief IQR and novelty](figures/engineering_gates/fig_population_diversity.png)

*Figure 5. The genotype statistic is the root mean squared pairwise distance over varying numeric genes, each divided by its observed range over this retained run; fixed genes and categorical material are excluded. For generation matrix $X$ with $n$ rows and $d$ varying normalized genes, $D=\sqrt{2n\sum_j\operatorname{Var}(X_j)/[(n-1)d]}$. No sampling or fabricated bound table is used. Panels (b,c) report the actual relief-fraction interquartile range and median saved novelty. These are descriptor projections and the recorded novelty statistic; the retained files do not contain every birth's full 16-component descriptor.*

Observed-range normalization allows within-run spread analysis; it does not provide cross-wave distances under different observed ranges. Novelty depends on the available archive/population context, so an increasing or decreasing value alone cannot establish morphological diversity or causality. The population ledger exposes the normalization and contributing CSV lines for each generation.

![Retained archive descriptor spread across historical runs](figures/engineering_gates/fig_archive_descriptor_spread.png)

*Figure 5b. Across the committed w7–w16, w272/w273 and w282 final MAP-Elites archives, the bars show observed median, interquartile range and full retained range of mass and relief fraction, with actual archive counts. These are selected niches under different run objectives, not complete population samples or identical descriptor distributions. W6 lacks the relief column and is omitted from this comparison. Every contributing CSV row is recorded in the point ledger.*

### V-G. Lineage and the margin–mass frontier

![Actual ancestry and separately sourced controlled-probe winners](figures/engineering_gates/fig_winner_lineage.png)

*Figure 6. The left graph follows actual parent IDs and operator tags for the lightest retained strict-certified GA passer; layout ordinate has no descriptor meaning. The right graph shows the actual named w282 record and reserve peer from [controlled_probes.json](../evidence/rs_v2/w282_ultrarunner/controlled_probes.json): p14 descends from `w281_s2932_c02_light`, p10 from `w281_s2932_c01`, with the exact recorded relief/flute patches. The named winners are not falsely attached to a GA ancestry they never had. Missing earlier ancestry is not reconstructed by geometric similarity.*

![Historical selected-candidate structural margin and G1 versus mass](figures/engineering_gates/fig_pareto_history.png)

*Figure 7. Each point is a committed candidate evaluation. The displayed structural margin is $\min(C_{lat,strict}/260,seq/1.2,SF_{twist,strict},SF_{solid})-1$, explicitly a **four-metric subset**, not proof that all 16 gates pass. Mass and G1 retain their own observations and model regime. Red w283 points show the corrected failures. The figure displays trade-offs among retained points; it does not assert an exhaustive global Pareto optimum.*

Historical selected mass falls from w274's **84.979 g** raster to w282 p14's **75.568 g**; the w282 reserve peer is **75.836 g**. These recorded selections illustrate a mass/reserve trade-off under their then-used models. They do not establish a record certified under the corrected main. The w283 **75.382/75.642 g** candidates remain measured geometries with rejected certificates. Cross-solver agreement and a complete G1 do not restore their failed strength or handling requirements.

All seven population figures regenerate with [populations.py](../ops/render/paper/populations.py). [population_points.json](figures/engineering_gates/population_points.json) records source file, JSON pointers/CSV line numbers, denominator, reduction and plotted values; each PNG manifest binds that ledger, the script and the actual committed input files. Every observation input is checked with `git ls-files`. Sparse histories remain sparse.

### V-H. The retained certifiable-pipeline round: selection without a winner

![Real retained births, provisional rank and independently assessed gate failures](figures/engineering_gates/fig_certifiable_pipeline.png)

*Figure 7b. The #84 proof retains **seven birth/screen instances**, including the original parent, with **six distinct genome content identities**. Panel (a) shows every recorded screen mass and structural-shortfall sum; equal genomes are counted as retained instances, not relabeled unique births. The star identifies the selected assessment. Panel (b) separates seven screens, one completed assessment, zero unverified attempts and zero qualified designs. Panel (c) gives that assessment's actual 10/16 predicates in §II's grouping; panel (d) reports selected physical ratios, explicitly including the separate RS-v2 fatigue test. All observations and reductions trace by file/JSON pointer in [certifiable_points.json](figures/engineering_gates/certifiable_points.json); [certifiable.py](../ops/render/paper/certifiable.py) regenerates the figure.*

This is **one pre-merge proof committed with #84**, not a post-merge wave or a new historical record. It retains no multi-generation G1/fitness series or full-population independent-gate evaluations, so none are interpolated from this round. The historical w282 population, earlier archive distributions and selected-candidate Pareto projection above retain their own source/model regime. Comparing masses across different exported geometry laws does not demonstrate causal improvement of the new pipeline.

### V-I. Retained plateau diagnostics and the selection repair

Merged [PR #87](https://github.com/neverhuman/jarambit/pull/87), on main at `7cb952e`, retains [hashed diagnostic snapshots](../evidence/evolution_plateau/2026-10-17/README.md) captured at 08:02 UTC on 17 October 2026. These files document historical failures; they omit exported CAD, meshes and raw solver sidecars and cannot independently re-certify any design. W298 scorecards identify source `ae86f188`; xbabe3 scorecards identify `6438f077`. The samples retain their own source/model identities and are not a matched-code experiment.

| Wave / retained scope | Births / rankable, by proposal round | Completed assessments in capture |
|---|---|---:|
| w298, final leaderboard | 68/53; 64/63; 64/63 | 6 |
| w401c3, final leaderboard | 52/45; 48/47 | 4 |
| w300, partial first-screen capture | 100/64 | 0 |
| w402c3, partial first-round capture | 52/44 | 1 |

The w300 row describes only the committed capture, **not the later completed wave**. Subsequent runtime results remain outside these plots until committed. Neither retained final leaderboard has a qualified design; all eleven captured assessments fail the same six W_CAD predicates: hiker, hunter, strict hunter, tip-only twist, strict tip-annulus twist and solid tip. Recorded solid SF spans 0.6361–0.6716, human-twist SF 0.05748–0.05906, spine SF 0.22312–0.23345 and fatigue SF 0.45737–0.48302. These are selected failures, not a population certification rate or a new mass record. The battery catalog remains 23×18=414 cells, with 0/23 RS-v2 all-hard passes.

![Historical plateau: coarse ranking, assessed failures, seed-family collapse and physical deficits](figures/engineering_gates/fig_plateau_history.png)

*Figure 8. (a) Best coarse shortfall D over retained rounds; partial captures supply single points only. (b) Actual selected-assessment solid SF versus raster mass, below the unchanged SF=1 floor. (c) Birth instances by filename-derived family, with distinct CSV rows / total births above each bar; an asterisk marks partial captures. (d) Individual SF observations for six binding metrics across the eleven captured assessments; vertical offsets are layout. [plateau.py](../ops/render/paper/plateau.py) checks committed bytes and original host hashes, then regenerates the figure and [plateau_points.json](figures/engineering_gates/plateau_points.json), which records every observation, JSON pointer and reduction. No fresh solve or current certificate is inferred.*

W298's best coarse shortfall falls from 3.104828 to 3.068820 over three rounds without producing a qualified candidate. Its initial 68 birth instances contain 56 distinct CSV rows. All four leading first-round parents are p14, and later births lose p10; w401c3 exhibits the opposite family loss. The retained [geometry-only replay](../evidence/evolution_plateau/2026-10-17/rejection_probe.json), with [input/source/output hashes](../evidence/evolution_plateau/2026-10-17/probe_manifest.json), finds all 25 w300 control-family births rejected by the assumed holder envelope, despite aesthetic acceptance. Family preservation addresses one measured selection failure. It supplies neither a physical-feasibility proof nor measured improvement under the new selection rule; no matched post-fix run is retained here.

### V-J. Completed automatic-wave observations

The following observations come from committed automatic-wave leaderboards and [`wave_summary.json`](../evidence/evolution_waves/2026-10-17/wave_summary.json) (verified by [`summarize.py`](../evidence/evolution_waves/2026-10-17/summarize.py)), reduced by [`wave_history.py`](../ops/render/paper/wave_history.py) into [`wave_history_points.json`](figures/engineering_gates/wave_history_points.json). Across all 40 committed wave records (`w298` and `w300`–`w322` on `xbabe2`; `w400c3`–`w408t3` and `w410c3`–`w411t3` on `xbabe3`), Stage-2 completed 177 independent assessments (`0` qualified, `11` unverified across `w320`, `w400c3`, `w403c3`, `w407c3`, `w410c3`, and `w410t3`), all verified directly from committed `leaderboard.json` files. For Stage-1 (`6,914` births / `5,189` survivors across all 40 waves; `6,051` births / `4,636` survivors across the 34 plotted waves), 56 of the 94 rounds (112 `round_*_births.json` / `round_*_screen.json` files under `evidence/evolution_plateau/2026-10-17/` and `evidence/evolution_waves/2026-10-17/`: `w298` rounds 0–2, `w300` round 0, `w315`–`w322` rounds 0–1, `w401c3` rounds 0–1, `w402c3` round 0, `w405c3`–`w408t3` rounds 0–2, and `w410c3`–`w411t3` rounds 0–2; `3,642` births / `2,252` survivors across all 40 waves, or `3,015` births / `1,907` survivors across the 34 plotted waves) are verified against committed raw JSON in Git, while the remaining 38 rounds (76 birth/screen digests; `3,272` births / `2,937` survivors across all 40 waves, or `3,036` births / `2,729` survivors across the 34 plotted waves, marked `*` below) are unverified producer-reported host metadata whose raw `round_*.json` files remain outside Git. The 34 zero-unverified completed waves (`6,051` births, `4,636` survivors, `156` assessments) are tabulated and plotted below:

| Wave (host / commit) | Stage-1 screens | Assessed | Qualified | Mass min / med (g) | W_CAD best / med | Battery best / med (/18) | Solid-tip SF best / med |
|---|---|---:|---:|---:|---:|---:|---:|
| `w298` (`xbabe2` / `ae86f18`) | 53/68, 63/64, 63/64 | 6 | 0 | 83.047 / 83.059 | -5.565306 / -5.765577 | 12 / 11 | 0.6694 / 0.6527 |
| `w300` (`xbabe2` / `9d64160`) | 64/100, 92/96* | 4 | 0 | 83.324 / 86.927 | -5.389172 / -5.557408 | 11 / 11 | 0.6932 / 0.6747 |
| `w301` (`xbabe2` / `7cb952e/9d64160`) | 83/100*, 95/96* | 4 | 0 | 83.364 / 85.708 | -5.287856 / -5.754633 | 11 / 11 | 0.6932 / 0.6613 |
| `w302` (`xbabe2` / `edc0f40`) | 81/100*, 92/96* | 4 | 0 | 88.015 / 89.720 | -5.321724 / -5.529497 | 11 / 11 | 0.6932 / 0.6747 |
| `w303` (`xbabe2` / `edc0f40`) | 82/100*, 94/96* | 4 | 0 | 86.854 / 89.144 | -5.473826 / -5.558305 | 11 / 11 | 0.6746 / 0.6740 |
| `w304` (`xbabe2` / `805faaa`) | 80/100*, 91/96* | 4 | 0 | 86.841 / 89.162 | -5.280962 / -5.453326 | 11 / 11 | 0.6934 / 0.6885 |
| `w305` (`xbabe2` / `805faaa`) | 82/100*, 93/96* | 4 | 0 | 86.888 / 91.307 | -5.276352 / -5.326866 | 11 / 11 | 0.6934 / 0.6885 |
| `w306` (`xbabe2` / `805faaa`) | 83/100*, 94/96* | 4 | 0 | 91.408 / 92.118 | -5.286892 / -5.402706 | 12 / 11.5 | 0.6892 / 0.6741 |
| `w307` (`xbabe2` / `2539448`) | 84/100*, 92/96* | 4 | 0 | 91.491 / 92.165 | -5.226954 / -5.470097 | 15 / 14 | 0.7068 / 0.6607 |
| `w308` (`xbabe2` / `a8a7de4`) | 83/100*, 94/96* | 4 | 0 | 91.437 / 92.141 | -5.306115 / -5.323521 | 14 / 14 | 0.6938 / 0.6841 |
| `w309` (`xbabe2` / `0c8d169`) | 86/100*, 92/96* | 4 | 0 | 91.420 / 92.143 | -5.183108 / -5.211783 | 15 / 14 | 0.7163 / 0.7090 |
| `w310` (`xbabe2` / `dae0222`) | 82/100*, 94/96* | 4 | 0 | 87.531 / 91.412 | -5.183108 / -5.297277 | 15 / 14.5 | 0.7113 / 0.7067 |
| `w311` (`xbabe2` / `1927687`) | 84/100*, 93/96* | 4 | 0 | 87.530 / 89.477 | -5.262728 / -5.324286 | 15 / 14.5 | 0.7090 / 0.7067 |
| `w312` (`xbabe2` / `1927687`) | 84/100*, 96/96* | 4 | 0 | 87.567 / 89.493 | -5.262717 / -5.372534 | 14 / 14 | 0.7090 / 0.7067 |
| `w313` (`xbabe2` / `1eee4d1`) | 82/100*, 96/96* | 4 | 0 | 87.588 / 90.507 | -5.224336 / -5.315769 | 14 / 14 | 0.7100 / 0.7067 |
| `w314` (`xbabe2` / `1eee4d1`) | 82/100*, 93/96* | 4 | 0 | 87.589 / 89.519 | -5.320623 / -5.367266 | 14 / 14 | 0.7090 / 0.7001 |
| `w315` (`xbabe2` / `21425fe/f1b2671`) | 83/100, 93/96 | 4 | 0 | 87.595 / 89.528 | -5.258458 / -5.367665 | 14 / 14 | 0.7091 / 0.7060 |
| `w316` (`xbabe2` / `f1b2671`) | 20/104, 14/96 | 4 | 0 | 87.600 / 96.508 | -5.258458 / -7.333562 | 14 / 14 | 0.7091 / 0.4605 |
| `w317` (`xbabe2` / `0a6aa94`) | 20/104, 13/96 | 4 | 0 | 87.600 / 96.508 | -5.258458 / -7.333562 | 14 / 14 | 0.7091 / 0.4605 |
| `w318` (`xbabe2` / `b38642b/90f77f3`) | 49/104, 41/96 | 4 | 0 | 87.600 / 99.850 | -5.258458 / -7.216916 | 14 / 14 | 0.7091 / 0.4816 |
| `w319` (`xbabe2` / `90f77f3`) | 56/104, 48/96 | 4 | 0 | 87.600 / 100.903 | -5.258458 / -7.254717 | 14 / 14 | 0.7091 / 0.4724 |
| `w321` (`xbabe2` / `9443b11`) | 57/104, 72/96 | 4 | 0 | 112.965 / 113.419 | -7.431754 / -7.473815 | 14 / 14 | 0.3816 / 0.3762 |
| `w322` (`xbabe2` / `b7ef2de`) | 77/102, 94/96 | 4 | 0 | 112.943 / 113.398 | -7.495754 / -7.502600 | 14 / 14 | 0.3511 / 0.3508 |
| `w401c3` (`xbabe3` / `6438f07`) | 45/52, 47/48 | 4 | 0 | 82.861 / 82.869 | -5.556243 / -5.569637 | 11 / 11 | 0.6716 / 0.6701 |
| `w402c3` (`xbabe3` / `6438f07`) | 44/52, 46/48* | 4 | 0 | 83.072 / 83.292 | -5.570458 / -5.640241 | 11 / 11 | 0.6693 / 0.6535 |
| `w404c3` (`xbabe3` / `7cb952e`) | 32/52*, 45/48*, 47/48* | 6 | 0 | 82.865 / 82.971 | -5.496513 / -5.664944 | 12 / 11 | 0.6777 / 0.6534 |
| `w405c3` (`xbabe3` / `7cb952e`) | 34/52, 48/48, 47/48 | 6 | 0 | 82.897 / 82.980 | -5.433651 / -5.718887 | 12 / 11 | 0.6822 / 0.6571 |
| `w406c3` (`xbabe3` / `1eee4d1`) | 33/52, 46/48, 48/48 | 6 | 0 | 82.904 / 82.996 | -5.384448 / -5.563714 | 14 / 14 | 0.6849 / 0.6730 |
| `w406t3` (`xbabe3` / `1eee4d1`) | 33/52, 48/48, 47/48 | 6 | 0 | 83.274 / 84.517 | -5.252210 / -5.772613 | 14 / 14 | 0.6823 / 0.6372 |
| `w407t3` (`xbabe3` / `d046093`) | 12/69, 4/48, 36/36 | 6 | 0 | 83.047 / 98.561 | -5.252210 / -9.253661 | 14 / 14 | 0.6823 / 0.2187 |
| `w408c3` (`xbabe3` / `f1b2671`) | 9/55, 16/48, 47/48 | 6 | 0 | 83.047 / 96.733 | -5.252210 / -8.179430 | 14 / 14 | 0.6823 / 0.2960 |
| `w408t3` (`xbabe3` / `f1b2671`) | 9/55, 15/48, 48/48 | 6 | 0 | 83.047 / 98.563 | -5.252210 / -9.071262 | 15 / 14 | 0.6823 / 0.2493 |
| `w411c3` (`xbabe3` / `9443b11`) | 18/41, 41/48, 47/48 | 6 | 0 | 101.108 / 102.070 | -7.414831 / -7.553985 | 15 / 15 | 0.3636 / 0.3584 |
| `w411t3` (`xbabe3` / `9443b11`) | 18/41, 40/48, 47/48 | 6 | 0 | 101.108 / 105.592 | -7.426523 / -7.512293 | 15 / 15 | 0.3626 / 0.3472 |

![Selected Stage-2 wave history](figures/engineering_gates/fig_wave_history.png)

*Figure 8b. Script-generated from the named leaderboards: dots show selected-assessment medians, lines their ranges, and diamonds the best per-wave W_CAD, battery pass count (/18), or 3D P2 solid-tip safety factor (`solid_tip_SF`). These are not full-population statistics. Different wave source/model identities are not a controlled comparison of the search rule.*

On `xbabe2`, adding ~8.5 g of stock/tip mass across `w300`–`w320` raised the best Stage-2 `W_CAD` from `-5.565306` (`w298`) to `-5.183108` (`w309`, `w310`; `-5.224336` in `w313`, `-5.320623` in `w314`, `-5.258458` in `w315` on `w315_r01_p14_c50` and in `w316`–`w320` on `w316_r01_p14_c27` / `w317_r01_p14_c27` / `w318_r01_p14_c27` / `w319_r01_p14_c27` / `w320_r01_p14_c27`) and lifted the 3D P2 solid-tip safety factor `solid_tip_SF` from `0.6694` to `0.7163` (`w309_r02_p14_c35`; `0.7100` in `w313`, `0.7090` in `w314`, `0.7091` in `w315` on `w315_r01_p10_c03` and in `w316`–`w320` on `w316_r01_p10_c01` / `w317_r01_p10_c01` / `w318_r01_p10_c01` / `w319_r01_p10_c01` / `w320_r01_p10_c01`), while PR #91 (`2539448`, `w307` onward) resolved the STEP unit/regeneration evaluator blockers so battery pass counts rose from `11–12/18` to `14–15/18`. In `w321` (`9443b11`) and `w322` (`b7ef2de`), novel-offspring Stage-2 prioritization evaluated eight newly spawned Q7 offspring (`112.943–113.864 g`, `ring_bearing_w_mm = 12.19–12.41 mm >= 10.0 mm`), where `w321_r01_p10_c100` (`lat_rob_cert_n = 201.1 N >= 180.0 N`, `strict_hunter_SF = 0.7500` / `195.0 N`, `fat_sf = 0.5618`, `11/16` `W_CAD_v2` gates), `w321_r02_p10_c81` (`lat_rob_cert_n = 195.2 N >= 180.0 N`, `strict_hunter_SF = 0.7292`, `fat_sf = 0.5452`, `11/16` `W_CAD_v2` gates), `w322_r01_p10_c91` (`lat_rob_cert_n = 191.56 N >= 180.0 N`, `strict_hunter_SF = 0.7315`, `seq_ratio = 1.8018`, `fat_sf = 0.5351`, `11/16` `W_CAD_v2` gates), and `w322_r02_p10_c02` (`lat_rob_cert_n = 191.6 N >= 180.0 N`, `strict_hunter_SF = 0.7315`, `seq_ratio = 1.802`, `fat_sf = 0.5351`, `11/16` `W_CAD_v2` gates) pass `cert_hiker_180n` while `w322_r01_p14_c66` and `w322_r02_p14_c25` satisfy `bore_edge_radius_mm = 1.5237 mm >= 1.50 mm` (`W_CAD = -7.495754`, `global_weighted_fitness = 71.0109` on `W_ULTRALIGHT_v1`). On `xbabe3` (`w401c3`–`w405c3` on pre-#91 engine commits and the paired `w406c3`/`w406t3` (`1eee4d1`), `w407c3`/`w407t3` (`d046093`), `w408c3`/`w408t3` (`f1b2671`), `w410c3`/`w410t3` (`8e7950d`), and `w411c3`/`w411t3` (`9443b11`) tip-band A/B waves), battery pass counts rose from `11–12/18` to `14–15/18` (`10/12` candidates in `w411c3`/`w411t3` reaching `15/18` battery passes and `11/16` `W_CAD_v2` gates, led by `w411c3_r02_p10_c12` at `strict_hunter_SF = 0.7577` / `197.0 N`, `W_CAD = -7.414831`, and `w411t3_r03_p10_c12` at `strict_hunter_SF = 0.7558` / `196.5 N`, `W_CAD = -7.426523`) and best `W_CAD` reached `-5.252210` (`w406t3_r01_p10_c16`, `w407t3_r01_p10_c29`, `w408c3_r01_p10_c15`, `w408t3_r01_p10_c15`, `w410c3_r01_p10_c15`, and `w410t3_r01_p10_c15`, `G1 = 0.479637`, `solid_tip_SF = 0.6823`; best `solid_tip_SF = 0.6849` on `w406c3_r03_p10_c15`). Both lanes remain limited by the coupled 4–15 mm tip bending/torsion and 260 N strict lateral regime (`solid_tip_SF < 1.0`, `strict_hunter_SF < 1.0`). Completed independent solves do not imply physical qualification. All listed leaderboards retain `qualified_count=0` and `champion=null`. No certified winner is established.

One `xbabe2` wave (`w320`: `assessed_count=3`, `unverified_count=1`, best `W_CAD = -5.258458`, best `solid_tip_SF = 0.7091`, battery `14/18`) and five `xbabe3` runs (`w400c3`: `assessed_count=0`, `unverified_count=4`; `w403c3`: `assessed_count=5`, `unverified_count=1`, best `W_CAD = -5.556243`, best `solid_tip_SF = 0.6708`, battery `11/18`; `w407c3`: `assessed_count=5`, `unverified_count=1`, best `W_CAD = -5.252210`, best `solid_tip_SF = 0.6823`, battery `14/18`; `w410c3`: `assessed_count=5`, `unverified_count=1`, best `W_CAD = -5.252210`, best `solid_tip_SF = 0.6823`, battery `15/18`; `w410t3`: `assessed_count=3`, `unverified_count=3`, best `W_CAD = -5.252210`, best `solid_tip_SF = 0.6823`, battery `14/18`) recorded in [`wave_summary.json`](../evidence/evolution_waves/2026-10-17/wave_summary.json) contain non-empty `unverified` lists and are excluded from the plotted completed-wave series by `read_board` while retained in committed evidence. Later waves enter only after an actual leaderboard exists and the cited evidence is committed on main. No missing wave or unfinished assessment is imputed. The catalog remains 23 subjects × 18 evaluators (414 cells); wave assessments are a separate denominator.

Sources: `evidence/evolution_plateau/2026-10-17/xbabe2/out/w298/leaderboard.json` (SHA256 `4ae79eda014772848816063e3f157d8899fa3968385700d7d600b2527d9aa748`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w300/leaderboard.json` (SHA256 `b4407ecb0cdbefca10b0b57e2cdefa714b76c3202a7d0ee9b187fd7c549ff53b`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w301/leaderboard.json` (SHA256 `a2c1c871b7e8a502f0ce62d417ff9973f8f1409facf1280fb2e2d817d7f409b8`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w302/leaderboard.json` (SHA256 `bd297cc3eaefc1dae187d96f3c9f81f74272f8c14be1f1a226df0222eff1f940`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w303/leaderboard.json` (SHA256 `4c23836204a44097fe90c341c8726b0c5090678b568ff94e7cc130c265cea239`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w304/leaderboard.json` (SHA256 `64b64de83a7b6a56ea8429633e986824a859ca113b2586ba5dd494367e26a502`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w305/leaderboard.json` (SHA256 `be2a7cbaa38bb8107185f794b46e0a2c2c0297769792fe11185391e2a7e3306c`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w306/leaderboard.json` (SHA256 `a9a73105f1fc3e97d5698c8a708e637c4e930a0d34dd7de3b44154da1142d321`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w307/leaderboard.json` (SHA256 `f886d2296288de1fb81a4324a0d5c81dac91913485222a47a56b1eaa9de20bb9`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w308/leaderboard.json` (SHA256 `37a154315d53e6b652a2153df388ca45bc612804d1556a89438f5deaa1700113`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w309/leaderboard.json` (SHA256 `1462fe6feff41a59da9e53d2362c3e425da691b49e08ea3037e97b85f0b1c818`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w310/leaderboard.json` (SHA256 `517e3bb0292b3edf1e41a5d3d943dfe70b047f684d7201a5694d6aae2de915f9`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w311/leaderboard.json` (SHA256 `77e9f339e73fcb9f11e90248d71ccca0fc98518a990276e26ba3e40ef0eb7ea4`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w312/leaderboard.json` (SHA256 `5c418e8eb65d2948b65c5a2711cd4b3cd53146ea61587aa975e7ee0e9e9efe2a`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w313/leaderboard.json` (SHA256 `b01a9f23e8d35d9078048fd29a2056e4dceaf632abba25113aefc9984f1fc885`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w314/leaderboard.json` (SHA256 `cee1a486e921cb3631e7b7de6f14fb37af806bfb3c887b2238e6aa30740e140b`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w315/leaderboard.json` (SHA256 `49f9dcc245b9a2d81518ccbd992fe9b9c445d521a25080baf9b0eb98a611924d`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w316/leaderboard.json` (SHA256 `63df2d7b2c3613a4411b619b48200ec227ca9ef03515bc53e5e5dacb7405f8f5`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w317/leaderboard.json` (SHA256 `429efbe4be1db44befc662ed5fe7a4a1de8ebe3cb53beaa0902a88179b72bb0e`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w318/leaderboard.json` (SHA256 `7eeeb41a5dd36718ee6d300b626c76b01e0be5215bd36c97d9c37ca354c654a9`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w319/leaderboard.json` (SHA256 `df129159ec3d3e3d95044827f190c082b0ccd4165fc935dca002011b3b05fadf`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w321/leaderboard.json` (SHA256 `0553f5adb001b7132123b2d84c2a232308709fdf6ce6953ca3328256ae45ccb9`); `evidence/evolution_waves/2026-10-17/xbabe2/out/w322/leaderboard.json` (SHA256 `0c17ba289f3b4c5087fae63f3c0836da38a2e9bfa9c490e090758e12121eb0ce`); `evidence/evolution_plateau/2026-10-17/xbabe3/out/w401c3/leaderboard.json` (SHA256 `1c2916610a51bb5a1114210e29e587791ad5d2849ccf1f9f206e1c5c276a80fe`); `evidence/evolution_waves/2026-10-17/xbabe3/out/w402c3/leaderboard.json` (SHA256 `0f39500ad3c785589a2e124309e9f32862a4c011c99c31b26a51e448a7124ca6`); `evidence/evolution_waves/2026-10-17/xbabe3/out/w404c3/leaderboard.json` (SHA256 `01340fd0a2948b9d3807d967f1de5c7048efcbabf53ed9129a14c10aba111ed2`); `evidence/evolution_waves/2026-10-17/xbabe3/out/w405c3/leaderboard.json` (SHA256 `faf231f9b2c96dfc3665750303f6d18c00bf41ba43c40df94054c5db7b75cb50`); `evidence/evolution_waves/2026-10-17/xbabe3/out/w406c3/leaderboard.json` (SHA256 `56915cfc513a27e7e17a3a87ad3788e84cb65c23f5009472d3ca3def7c271af9`); `evidence/evolution_waves/2026-10-17/xbabe3/out/w406t3/leaderboard.json` (SHA256 `376f268e74c7de5abb5af1e74c96098b5cb43ab6afddad9e2a02a8660bdd62eb`); `evidence/evolution_waves/2026-10-17/xbabe3/out/w407t3/leaderboard.json` (SHA256 `a57a19ee420f859d96c870f977ded1e3def391926fc03d3e17a54719d4e063e6`); `evidence/evolution_waves/2026-10-17/xbabe3/out/w408c3/leaderboard.json` (SHA256 `5928d01586e72e8d315359a3c63adc570693ea732098add8808fea2811370a41`); `evidence/evolution_waves/2026-10-17/xbabe3/out/w408t3/leaderboard.json` (SHA256 `e9b69682cf090106083d7db4f3bcdb5610d8849fb1e5b98ce873466e6f43f036`); `evidence/evolution_waves/2026-10-17/xbabe3/out/w411c3/leaderboard.json` (SHA256 `70da1cb69f460711300dc34e8f37b996b0e97f479f9c6ea13e4e47116bd2da16`); `evidence/evolution_waves/2026-10-17/xbabe3/out/w411t3/leaderboard.json` (SHA256 `46d1a7f143dabf7c36b03fb0646b221bfaee19d60a1ac9221d41ea7f0bcd74ba`).

### V-K. History source availability

The earlier [main snapshot `0c8d169`](https://github.com/neverhuman/jarambit/tree/0c8d1692a2f113726f8a74aa796f2dea959b7094) contained no `out/w301/leaderboard.json` through `out/w308/leaderboard.json` (and retained only the partial first-screen capture for `w300`). The completed Stage-2 `leaderboard.json`, `RESULT.md`/`leaderboard.md`, and representative top `master_scorecard.md` outputs for `xbabe2` `w300`–`w322` and `xbabe3` `w400c3`–`w408t3` and `w410c3`–`w411t3` are now committed under [`evidence/evolution_waves/2026-10-17/`](../evidence/evolution_waves/2026-10-17/wave_summary.json) while keeping the `08:02 UTC` plateau snapshot under `evidence/evolution_plateau/2026-10-17/` immutable. Raw Stage-1 `round_*_births.json` and `round_*_screen.json` files are committed in Git for 56 of the 94 rounds (112 files under `evidence/evolution_plateau/2026-10-17/` and `evidence/evolution_waves/2026-10-17/`); the remaining 38 Stage-1 rounds (76 birth/screen digests) are recorded in `wave_summary.json` as `unverified_producer_reported` host metadata because their raw `round_*.json` files and per-candidate large binary exports (`.step`, `.stl`, `_D1.json`, `_surface.json`) remain in host `out/` directories rather than Git. Any subsequent waves (`w323+`, `w412+`) remain excluded until their completed leaderboards are committed on `main`; no missing-wave statistics are inferred.

## VI. Blender atlas and reproducible publication

### VI-A. Geometry and image semantics

**CERTIFIED-WINNER RE-RENDER SLOT — EMPTY.** No certified all-16 winner exists yet; the committed automatic leaderboard has `qualified_count=0` and `champion=null`. This slot has no final-design mesh or winner rendering. Fill it only after an evolved export and its compatible all-16 and RS-v2 evidence are committed on fixed main. Record that source commit, genome/model identity and STEP/STL/raw-solver hashes, then re-render all gate plates and hero angles. The assessed example below is not the final design.

![Committed assessed offspring, explicitly not a qualified winner](figures/whitepaper/blender/hero_oblique.webp)

*Figure 8. Cycles CPU studio rendering of the exact committed `cert_pipeline_r01_p14_c02.stl` named by the #84 proof manifest. The faceted geometry is preserved, including its changed bore/edge export. This assessed failure passes 10/16 engineering predicates, fails RS-v2 and is not a winner. Titanium reflectance, lighting and depth of field are illustrative; no surface-finish strength, film thickness or manufactured specimen is claimed.*

![Reverse face with illustrative anodized material](figures/whitepaper/blender/hero_reverse.webp)

*Figure 9. Reverse-angle rendering of the same unchanged mesh with an illustrative anodized-titanium shader. Appearance adds no coating/material evidence.*

![Actual mesh section and illustrative ring gauge](figures/whitepaper/blender/hero_section.webp)

*Figure 10. A plane cut through the actual mesh and a declared **22 mm cylindrical sizing gauge**. The gauge illustrates the ring context; it is not a human hand, comfort test, or validated finger-bearing/contact model. The section is a view operation, not a newly designed part.*

The **16 individual gate renderings** linked in §II show the same committed assessed example with identified load, boundary, envelope, sizing or mesh annotations and its actual per-gate verdict. Ring and local-tip plates use closer views of the exact mesh. Ring pull is drawn in the solver's $-x$ direction, and torque glyphs rotate about the actual saved ring-to-tip chord. Where a field is shown, it is that subject's saved D1 **design-load plate envelope**, mapped on its retained element mesh at fixed **0–828 MPa**, including lateral/pry 260 N, chord torsion 1.82 N·m and ring 1500 N. The manifest declares the transfer function and clipping. The color scale is an archived display convention, not a replacement for the load-specific joint thin/hot acceptance calculation. A plate envelope is not relabeled as a single-case field, strict-mesh field or 3D solid solution. No missing stress/displacement data are invented. The support springs/contact and holder envelope are illustrations where no exported counterpart exists.

### VI-B. Rebuild commands and artifact custody

Blender 4.5.3's official Linux archive is checksum-verified; the Ubuntu 4.0.2 package lacked OpenImageDenoiser. [render_paper.py](../ops/render/blender/render_paper.py) runs headless Cycles with **4 CPU threads**, **48 samples**, seed **42**, denoising, `nice -n 10` and `~/bin/memcap 32G --`. A separate Pillow publication layer typesets the requirement/status labels after Cycles using system DejaVu Sans Bold (`fonts-dejavu-core`), so studio depth of field and Monte Carlo glyph sampling cannot obscure them. Each manifest retains the exact caption text, positions, font SHA-256 and Pillow version; the unlabelled PNG and caption sidecar remain build artifacts. Images are approximately **1600×1000**, with compressed publication outputs and per-image input/script/output manifests. Every output states **assessed example, not qualified, champion=null**; each gate image reads its verdict from the actual scorecard. The wrapper checks that all inputs are committed and that the selected proof remains a nonqualifying example. When a winner is certified on fixed main, its mesh and evidence must be admitted explicitly and the figures re-rendered; an attractive image cannot serve as that certification.

```sh
python3 ops/render/paper/populations.py
python3 ops/render/paper/battery.py
python3 ops/render/paper/verification.py
python3 ops/render/paper/certifiable.py
python3 ops/render/paper/plateau.py
python3 evidence/evolution_waves/2026-10-17/summarize.py
python3 ops/render/paper/wave_history.py --output-dir docs/figures/engineering_gates \
  evidence/evolution_plateau/2026-10-17/xbabe2/out/w298/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w300/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w301/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w302/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w303/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w304/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w305/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w306/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w307/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w308/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w309/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w310/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w311/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w312/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w313/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w314/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w315/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w316/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w317/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w318/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w319/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe2/out/w321/leaderboard.json \
  evidence/evolution_plateau/2026-10-17/xbabe3/out/w401c3/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe3/out/w402c3/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe3/out/w404c3/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe3/out/w405c3/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe3/out/w406c3/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe3/out/w406t3/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe3/out/w407t3/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe3/out/w408c3/leaderboard.json \
  evidence/evolution_waves/2026-10-17/xbabe3/out/w408t3/leaderboard.json
BLENDER=$HOME/opt/blender-4.5.3-linux-x64/blender \
  python3 ops/render/blender/render_paper.py
python3 ops/render/paper/build_pdf.py
python3 ops/ci/version_check.py
python3 ops/ci/verify_figures.py
python3 ops/ci/gen_whitepaper.py --check
```

The PDF builder reads the existing two Markdown documents and their real figures. Build output and logs stay in this lane's `target/paper/`, following the repository's generated-output convention; no redundant full-paper PDF binary is committed. Figure PDFs elsewhere in the repository remain their existing artifacts. The Markdown, render/build sources, observation ledgers and compact images are the reviewable source publication. The source white paper retains its prior draft appendix for traceability; the publication PDF explicitly points to that archive rather than reproducing a second obsolete paper body.

### VI-C. Individual gate renderings

The following plates retain the actual committed automatic-assessment mesh, **not a hand-built reference CAD or a certified winner**. Load arrows, support springs, sizing gauges and envelopes identify the test setup; they are illustrative annotations. Any shown color field is the saved, explicitly labeled multi-load plate envelope from this same subject. Each plate gives its recorded W_CAD gate verdict and the assessment's nonqualified status. A passing numerical-verification gate does not establish a passing physical-strength gate.

![Gate 1: cad watertight and single solid](figures/whitepaper/blender/cad_watertight_and_single_solid.webp)

![Gate 2: geometric and holder gates](figures/whitepaper/blender/geometric_and_holder_gates.webp)

![Gate 3: karambit ring and curved claw](figures/whitepaper/blender/karambit_ring_and_curved_claw.webp)

![Gate 4: c6 aesthetic floor](figures/whitepaper/blender/c6_aesthetic_floor.webp)

![Gate 5: cert hiker 180n](figures/whitepaper/blender/cert_hiker_180n.webp)

![Gate 6: cert hunter 260n](figures/whitepaper/blender/cert_hunter_260n.webp)

![Gate 7: strict hunter 260n](figures/whitepaper/blender/strict_hunter_260n.webp)

![Gate 8: cert ring pull 1500n](figures/whitepaper/blender/cert_ring_pull_1500n.webp)

![Gate 9: twist body 1p82nm](figures/whitepaper/blender/twist_body_1p82nm.webp)

![Gate 10: twist tip only 1p82nm](figures/whitepaper/blender/twist_tip_only_1p82nm.webp)

![Gate 11: twist tip submodel strict 1p82nm](figures/whitepaper/blender/twist_tip_submodel_strict_1p82nm.webp)

![Gate 12: solid tip 3d p2 1p82nm](figures/whitepaper/blender/solid_tip_3d_p2_1p82nm.webp)

![Gate 13: side a4 buckling and support](figures/whitepaper/blender/side_a4_buckling_and_support.webp)

![Gate 14: failure sequence 1p2](figures/whitepaper/blender/failure_sequence_1p2.webp)

![Gate 15: plate gci valid 3level](figures/whitepaper/blender/plate_gci_valid_3level.webp)

![Gate 16: dolfinx cross check 2pct](figures/whitepaper/blender/dolfinx_cross_check_2pct.webp)

## References

[R1] J.-L. Batoz, K.-J. Bathe and L.-W. Ho, “A study of three-node triangular plate bending elements,” *International Journal for Numerical Methods in Engineering*, 15, 1771–1812, 1980. [Publisher / DOI](https://doi.org/10.1002/nme.1620151205).

[R2] P. J. Roache, “Perspective: A Method for Uniform Reporting of Grid Refinement Studies,” *Journal of Fluids Engineering*, 116(3), 405–413, 1994. [DOI](https://doi.org/10.1115/1.2910291); [original paper](https://www3.nd.edu/~coast/jjwteach/www/www/60130/CourseLectureNotes/Roache_1994.pdf).

[R3] K. Deb, A. Pratap, S. Agarwal and T. Meyarivan, “A fast and elitist multiobjective genetic algorithm: NSGA-II,” *IEEE Transactions on Evolutionary Computation*, 6(2), 182–197, 2002. [DOI](https://doi.org/10.1109/4235.996017).

[R4] Repository implementation, requirements and saved observations are linked next to each predicate. [PR #82](https://github.com/neverhuman/jarambit/pull/82) and [issues #75–#81](https://github.com/neverhuman/jarambit/issues/75) establish the audit/correction history; final re-certification observations reside in [evidence/cert_integrity](../evidence/cert_integrity/). A citation to a test source is not a fresh test-execution receipt.
