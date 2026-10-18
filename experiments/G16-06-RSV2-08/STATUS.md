# G16-06-RSV2-08 — WIP, awaiting captain

The skyline prototype and sealed receipt are committed in `7a92be61550c52106004c85f512bb4769f20608d`; the construction harness is in `7de5cf764ca0f5f6be619f9be0a44698569e0b5c`.

Reference skyline certification minima (area 0.25/0.125 mm²): lateral 376.0620540923634 N, fatigue SF 1.0504635563610114, hunter SF 1.4463925157398592, sequence ratio 1.309754052926388. The separate 0.0625 mm² strict evaluation also passes the task targets. Actual export checks pass; native crescent library tests: 38 passed, 0 failed, 0 ignored. Runs used CPUs 12,13 and CARGO_BUILD_JOBS=2. No gate or threshold was relaxed.

**Full qualification remains blocked:** the 2 mm² search observation fails fatigue/lateral/hunter targets; faer parity fails the existing 1e-9 owner bound; human-max twist/spine fail; the inherited 16 mm collar exceeds 6.35 mm flat stock. Independent solid/cross-solver, plate GCI and support/buckling checks remain unassessed. Mass 135.20387381766406 g exceeds the unchanged soft 80 g objective. `all_gate_qualification=false`.

Review `candidate_receipt.json` and the lane handoff for exact scope, hashes and reproduction commands. The explicit construction law, config and genome are required together; the CSV alone through the production exporter does not reproduce the witness. Raw and rejected-run evidence remains in this lane’s `target/G16-06-RSV2-08/`.

This is a WIP checkpoint. Per the 2026-10-18 05:20Z instruction, await the captain’s next milestone. No PR while main CI on d046093 is in progress; no stacking, deletes or additional hand-built designs.

Checkpoint delivery is blocked by this sandbox: the shared Git metadata is read-only, so git add cannot create its index.lock. The status and lane COMMS handoff remain uncommitted for captain review; the numerical witness is already saved in the receipt commit above. Shared main COMMS is outside the writable workspace; its handoff requires captain relay.

Phase 2 (`G16-06-RSV2-08-p2`) is evaluated: sealed side-A4 passes (buckling 8.224954151002352, supported sequence 3.2135334990093467 >= certified 1.309754052926388). Sealed plate GCI is oscillatory, admitted=false. All 60 deterministic window probes are retained; no combined adjustment passed. See PHASE2.md and phase2_receipt.json. Full qualification remains false; gates/thresholds unchanged.

Phase 3 (`G16-06-RSV2-08-p3`) complete: 9/9 candidates, all three requested FEM meshes plus export/side-A4, no numerical prefilter. placement_05/local_01/local_04 pass scoped criteria at unchanged repository sequence floor1.20 and retained strength targets. All9 export/side-A4 pass; all6 interpolations fail GCI, .5012 also fails fatigue/358N and .5018 also fails sequence1.20. All9 five-level sequence histories are nonmonotonic. Full RS-v2/manufacturing qualification remains false. See PHASE3.md, phase3_receipt.json and phase3_candidates/.

Phase-3 delivery: external checkpoint2ddaae9 contains only an interim2/9 receipt; complete9/9 final results are local. Final commit attempted with required p3 trailers but blocked by read-only shared Git index.lock. Final source/raw archives and manifest preserve the complete results; captain must commit from a writable session.
