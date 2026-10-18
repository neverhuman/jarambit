# Certification measurement code audit

Task: `G16-12-RSV2-04-p19`. Static baseline: `9d42813`.

This report audits source formulas and existing synthetic non-weapon rectangular-strip/document fixtures. No production geometry, parameter search, structural solve, or design evaluation is performed. No production source is changed. Statements below describe the implementation, not a physical certification or an optimization recommendation.

## Source provenance

Paths are relative to the repository root. SHA-256 hashes exact source bytes; the Python verifier checks these bindings as well as the audit document's anchored file and table-schema digests. The historical V6..V14 index remains unchanged.

|Source path|SHA-256|
|---|---|
|crates/crescent/src/certification.rs|eeefc8b1ecbf21df85aefb967e351ada4f0070f8b481959afef419379215c4ac|
|crates/crescent/src/export.rs|575eef5f12aebc81a345ef2fe7f52a51ab73e6f7ed837eb1c5e556f2e81bec06|
|crates/crescent/src/geom.rs|d924c16ae1ea0dfe0f1d36a6c1b38b47209b8d641508d4e96332f86318d4d682|
|crates/crescent/src/fem.rs|9c62d764e8df8d02513b22fa8f2ce76da5bee4c612e81e9f2f58d43ad9f7fdca|
|crates/crescent/tests/cert_integrity.rs|6fb98daf1e0eb4ebc44498bd5791aceceb58ccee66f7f1a3f5b804beaecd1917|

## Vertical triangle intersections (`thickness`)

Let projected query `q=(qx,qy)` and triangle vertices `a,b,c` have coordinates `(x,y,z)`. The code uses barycentric coordinates in the XY projection:

```text
D = (by-cy)(ax-cx) + (cx-bx)(ay-cy)
u = ((by-cy)(qx-cx) + (cx-bx)(qy-cy)) / D
v = ((cy-ay)(qx-cx) + (ax-cx)(qy-cy)) / D
z = cz + u(az-cz) + v(bz-cz)
```

Triangles with `abs(D)<1e-14` are skipped. A projected hit requires `u>=-1e-10`, `v>=-1e-10`, `u+v<=1+1e-10`. Across all hits, `lo=min(z)`, `hi=max(z)`; return `Some(hi-lo)` only when both extrema are finite and `hi>lo`, otherwise `None`. The containment tolerance is geometric; it is not added to requirement bounds. A single sheet has no positive thickness. Multiple disjoint vertical intervals are represented by their outer span, not their summed occupied length; this function does not establish watertightness.

## Station generation (`stations`)

Write the 2D cross product as `cross(a,b)=ax*by-ay*bx`. For every directed edge of every triangle, with projected endpoints `A,B`, set `e=B-A`, `d=A-origin`, and `det=cross(dir,e)`. Solving `origin+x*dir=A+t*e` gives:

```text
x = cross(d,e) / det
t = cross(d,dir) / det
```

Skip edges with `abs(det)<1e-14`. Add `x` only if `0<x<length` and `0<=t<=1`. Start with `0`, `length`, and the caller's required stations; sort using `f64::total_cmp`, then use Rust `Vec::dedup_by` with `abs(a-b)<1e-10`. Thus stations include projected facet-edge crossings, not just a fixed-distance raster. Collinear/near-parallel edges take the skip branch. The function does not normalize `dir` or filter caller-supplied required stations to the segment. Distance units require a unit direction from the caller.

These are floating-point operations, not exact-arithmetic intersection predicates. The source enumerates projected triangle-edge crossings; this alone is not a universal proof that arbitrary overlapping surfaces cannot have additional upper/lower-envelope switches inside projected facets.

## Profile measurement (`point_profile`)

Query `q(d)=tip+d*bisector`. First measure `t(.25)`, `t(1)`, `t(2)` with `thickness`; any unavailable required thickness returns `None`. Generate all native stations over `length=10` with required `[.25,1,2,5]`, and measure every station; any missing sample again returns `None`.

For the returned ordered vector `(d_i,t_i)`, monotonicity is exactly `t_(i+1)>=t_i` at every adjacent pair, with no thickness tolerance. Set `level=.95*core`. If `t_0>=level`, crossing distance is `Some(0)`. Otherwise use the first pair with `t_i<level` and `t_(i+1)>=level`:

```text
taper = d_i + (level-t_i)*(d_(i+1)-d_i)/(t_(i+1)-t_i)
```

If no crossing occurs, `taper_mm=None` and `taper_lower_bound_mm=10`. This is a lower bound from the measured interval, not an exact length or proof of a constant plateau. In particular, the code compares against `.95*core`, not a construction-law base value. `None` for a crossing does not itself force failure if all other conditions pass.

|Profile predicate|Literal implementation condition|
|---|---|
|First thickness|`t(.25)<=.30`|
|Second thickness|`t(1)<=.60`|
|Third thickness|`t(2)<=1.00`|
|Monotonicity|Every adjacent thickness is nondecreasing, no waiver|
|Crossing-distance lower bound|`taper_lower_bound_mm>=5`|
|Combined profile result|Logical AND of the five predicates|

The calling `measure` function builds its direction from the first inner-edge point at least2 units from the origin point and the reverse-ordered outer-edge counterpart, each falling back to the stored fallback point if absent. In source notation: `u=norm(norm(qi-tip)+norm(qs-tip))`; `norm(v)=v/max(hypot(vx,vy),1e-12)`. Degenerate vectors are not turned into a guaranteed unit direction. The direct synthetic fixtures explicitly supply `(1,0)`.

## Surface slope and boundary sampling (`sharpness`)

For each of the two supplied boundary polylines, construct `geom::Segments`. Deduplicate projected vertex `(x,y)` values by their exact IEEE-754 bit pairs within that boundary. `Segments::nearest(q,.5)` computes point-to-segment distances in spatial bins; returned distances outside that search limit are infinity. Visit unique projected vertices only when distance to the boundary is `<=.031`. A successful `thickness` hit updates the maximum thickness and count; a failed hit marks `complete=false`. If either boundary has zero successful samples, return `None`.

For each triangle, set `U=b-a`, `V=c-a`, `n=U cross V`. Skip `abs(nz)<1e-14`. Keep the triangle only when `Segments::within_triangle` says its filled XY projection is within `.4` of that boundary; the helper tests contained segment endpoints, segment/triangle-edge intersections, and both directions of endpoint-to-segment distance. It is not a triangle-vertex-only proximity test. For a nonvertical triangle:

```text
z_x = -nx/nz
z_y = -ny/nz
half_angle_degrees = atan(hypot(nx,ny)/abs(nz)) * 180/pi
```

The maximum is a planar gradient magnitude angle, bounding directional slopes on the selected facets. Final `pass=complete && max_apex_mm<=.20 && max_half_angle_deg<=25`. `samples` sums successful thickness queries across the two boundaries; the same projected vertex can count once per boundary.

`complete` tracks the implemented boundary-vertex thickness availability, not universal surface coverage. The angle accumulator starts at0; there is no separate minimum number of retained slope facets. No arbitrary-mesh validity check is performed inside this function. This report does not exercise the source's baseline-initializing slope fixtures or add a physical model.

## Circular-profile evidence (`bore_radius`)

Set `r=ring_id/2`. For each vertex use `rho=hypot(x,y)` and `theta=atan2(y,x) rem_euclid TAU`; retain `r-1e-7<=rho<=r+(ring_od-ring_id)/4`. Group by integer cast of `theta*1e8`; positive-z vertices give `(rho,z)`, negative-z vertices `(rho,-z)`, and zero-z vertices contribute to neither list. Sort each face's points by radius then height and deduplicate adjacent pairs with Euclidean distance `<1e-8`.

A group with neither face starting within `1e-7` of `r` is skipped. Every participating group's two faces must each have at least17 points and start at `r` within that tolerance, or return `None`. With `a=points[0]`, `b=points[8]`, `c=points[16]` in the meridional 2D plane:

```text
D = 2*(ax*(by-cy)+bx*(cy-ay)+cx*(ay-by))
A = ax*ax+ay*ay; B = bx*bx+by*by; C = cx*cx+cy*cy
center_x = (A*(by-cy)+B*(cy-ay)+C*(ay-by))/D
center_y = (A*(cx-bx)+B*(ax-cx)+C*(bx-ax))/D
radius = distance(a,center)
```

If `abs(D)<1e-12`, return `Some(0)`. Otherwise require finite radius, `abs(center_x-ax-radius)<=1e-7`, `abs(center_y-ay)<=1e-7`, `abs(cx-center_x)<=.03`, and every one of the first17 points to have radial residual `<=1e-7`; failure returns `None`. Additional points after the first17 are not included in this residual test.

Accumulate the minimum fitted radius across both faces/all participating groups. Mark sector `min(floor((group_key/1e8)/TAU*72),71)` for each completed group. Return the minimum only when all72 sectors are marked and it is finite. If coverage is incomplete and `rs_v2` is false, the legacy fallback samples72 radial directions; the first `thickness(r+.05)<=thickness(r)` gives `Some(0)`, while a missing thickness returns `None`. Otherwise return `None`. Neither width metadata nor a nominal radius field substitutes for these vertex checks.

This path was reviewed statically; the two selected runtime tests do not test circular-profile fits or sector coverage.

## Independent-observation metadata (`independent_pass`)

|Condition|Literal check or behavior|
|---|---|
|Recognition|`status=verified`, `primary_solver=skfem_p2`, `independent_solver=dolfinx_v0.9.0`|
|Mesh binding|Cross-check mesh string is64 ASCII hex characters and exactly equals top-level mesh string|
|Stored setup|Unsigned integer `NL=4` and `NSUB=1`|
|Two observation pairs|Stored primary `SF_skfem_nodal_max/avg` versus cross-check `SF_nodal_max/avg`|
|Availability|Both numbers finite and positive; stored secondary observation exactly equals corresponding cross-check number|
|Computed comparison|For both pairs, `abs(primary-secondary)/secondary<=.02`|
|Return state|`Some(true/false)` for a recognized available comparison; `None` for missing/invalid evidence|

ASCII hex validation permits uppercase, but string binding is case-sensitive. The relative-error denominator is the secondary observation. The function computes discrepancies and does not trust a supplied `within_2pct` flag. It checks supplied metadata/numbers, not actual solver execution, mesh-file bytes, or solver independence by itself; upstream provenance checks remain necessary. Missing evidence and a measured out-of-tolerance comparison are distinct states.

## Topology, signed volume, and serialized facets

|Function|Exact behavior|Limit of the check|
|---|---|---|
|`census`|Count every triangle's three directed edges and their sorted undirected pairs; return number of undirected edges seen once, number seen more than twice, and maximum directed-edge multiplicity (0 for empty input)|Index-incidence check; no coordinate, self-intersection, nonzero-area, connectedness or vertex-link validation|
|`volume`|Sum `dot(a,cross(b,c))/6` over all triangles; preserve sign, no absolute value|A signed-volume sum is not by itself a solid-validity proof|
|`surface_json`|Keep triangles with all three vertex z values strictly positive and projected oriented double area `(bx-ax)*(cy-ay)-(by-ay)*(cx-ax)>1e-14`|Selected top-facet subset, not a full closed mesh|

For `census==(0,0,1)`, every represented undirected edge occurs twice, and no directed edge repeats. This is useful incidence/orientation evidence, not proof of an embedded connected manifold. Triangle indices are assumed valid by the coordinate-indexing functions.

`surface_json` emits schema `jarambit.export_surface.v1`, source string `exact exported CAD top facets`, the complete input vertex array, and the filtered original index triples in original order. It neither compacts/reindexes vertices nor resamples heights; unreferenced vertices can remain. It does not itself emit a content digest or the optional circular-patch metadata. A byte SHA depends on the caller's serialization, separately from this value-level schema.

For context, `measure` returns unavailable measurement fields if exporting fails, the census is not `(0,0,1)`, or signed volume is `<=0`; it then calls the three measurement functions. The literal volume guard is `<=0`, without an explicit finite-number check. This audit neither invokes this production export path nor treats its guard as a universal arbitrary-input validator.

## Existing synthetic fixture audit and executed checks

`strip` in `cert_integrity.rs` creates a rectangular-prism strip spanning lateral coordinates-1..1. Each supplied `(x,t)` gives four vertices `(x,-1,+t/2)`, `(x,+1,+t/2)`, `(x,-1,-t/2)`, `(x,+1,-t/2)`. Each adjacent station pair supplies eight triangles; each end supplies two. These are small numerical test solids, not a production design. The profile test uses origin `(0,0)`, direction `(1,0)`, and reference value4.

|Fixture|Existing source control|Assertions made by the executed test|
|---|---|---|
|`good`|Seven stations, nondecreasing piecewise-linear strip profile|Profile passes; monotonicity true; thickness at1 equals.5 within1e-12|
|`bad`|Same prefix through7; final thickness at10 drops from3 to2.5|Monotonicity false and profile fails; reversal after5 is not ignored|
|`small_reversal`|Interior station1.5 has thickness.49 after thickness.5 at1|Monotonicity false; explicitly rejects a.05-thickness waiver|
|`early`|Thickness reaches4 at distance4 instead of later|Profile fails; the test does not independently assert every returned field|

The following two exact existing Rust tests ran against unchanged production functions and passed. No other test body ran in these filtered invocations. In particular, no baseline construction or physical optimization test ran.

|Exact test filter|Executed|Passed|Failed|Filtered out|
|---|---|---|---|---|
|`brep_point_measures_intersections_and_rejects_reversals_after_five_mm`|1|1|0|13|
|`native_or_missing_independent_solver_never_becomes_a_pass`|1|1|0|13|

The independent-document test rejects an unrecognized native-only solver record, accepts its consistent synthetic two-solver record, then sets the secondary observation to2 and verifies `Some(false)` despite a true display flag; changing the independent solver label to `native_rust` produces `None`.

Both commands used the following prefix and exact filter names from the table:

```sh
nice -n 10 taskset -c 10,11 env CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=line-tables-only cargo test --offline --release --locked -p jarambit-crescent --test cert_integrity FILTER -- --exact --test-threads=2
```

Actual invocations also scoped `TMPDIR` to `target/gate-feasibility-bound/measurement_code_audit_p19` in the existing checkout. Effective inherited niceness was19; the requested `nice -n 10` cannot reduce inherited niceness. All work was pinned to CPUs10,11. Logs remain `point_fixture_test.log` and `independent_fixture_test.log` in that ignored receipt directory, but the portable document/source verifier does not need those logs or a compiled binary.

## Verification boundary

The extended standard-library unittest verifier anchors this document's exact bytes, byte length, table count and canonical table-schema SHA-256, verifies all five source-file digests, and retains all V6..V14 checks. It includes relocated-copy and corruption controls. Run from a repository copy:

```sh
nice -n 10 taskset -c 10,11 env CARGO_BUILD_JOBS=2 python3 -m unittest experiments/G16-12-RSV2-04/test_report_provenance.py
```

Source review covers all named functions; the two executed synthetic tests cover the specified strip fixtures and independent-observation metadata. They do not establish exhaustive coverage of slope sampling, circular fits, mesh topology or serialization, and no such runtime claim is made. A matching digest binds exact bytes relative to the trusted verifier revision; it does not establish authorship or protect against coordinated replacement of both code and anchors.
