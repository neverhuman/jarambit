# G16-06-RSV2-08 Phase 2 — measured prototype results

Sealed witness: side-A4 **PASS**, plate GCI **FAIL** (`oscillatory`, admitted=false). Full qualification remains false. No native source, requirement, load, support constant or threshold changed.

| Sealed evaluation | Result |
| --- | --- |
| Sequence at 4 / 1 / .25 mm² | 1.314268273436982 / 1.1618522657084458 / 1.309754052926388 |
| Side-A4 buckling eigenvalue | 8.224954151002352 (floor 1) |
| Side-A4 supported sequence | 3.2135334990093467 |
| Certified sequence | 1.309754052926388 |

The native GCI sequence/observable metadata flags are valid, but the numerical fit is oscillatory and `seq_ratio_gci.admitted=false`. This assignment requires admission; metadata alone is not reported as convergence success. The .25 mesh hash and ring-clamp metrics match exactly between native GCI, native side-A4 input and the sealed witness.

Executed 60 deterministic native window-gene trials: 36 broad placement/fill, 8 width/rounded-end-radius, and 16 local placement probes. 60 pass the unchanged geometry checks. No tested adjustment passed the combined convergence, certification/strict and export/support checks. This finite study does not establish infeasibility.

The first screen required all task targets at both coarse levels. The follow-up uses the sealed audit scope: targets at certification min(.25,.125) and strict .0625, with 4/1 as GCI observations; all five sequence values must be monotonic. Every coarse failure remains a failure in the receipt. Native GCI determines admission and positive order; failed fits and fine meshes are retained.

The 7 mm width trial (fillet_06) admits a 3-level fit and passes .25 targets (lateral 360.391068616949 N, fatigue 1.0066894000614173, sequence 1.3680770766716677). At .125 it fails lateral/fatigue (322.012021 N) and reverses sequence to 1.624254496. It is rejected before strict/export/support work; passing a 3-level fit does not establish the required five-level behavior.

| Trial | Window genes | Sequence 4 → 1 mm² | GCI fit |
| --- | --- | --- | --- |
| placement_00 | {"fill": 0.6, "s0": 0.05, "s1": 0.5} | 1.36960505 → 1.23073482 | not_run |
| placement_01 | {"fill": 0.65, "s0": 0.05, "s1": 0.5} | 1.31426827 → 1.16185227 | not_run |
| placement_02 | {"fill": 0.7, "s0": 0.05, "s1": 0.5} | 1.37776326 → 1.24892129 | not_run |
| placement_03 | {"fill": 0.6, "s0": 0.05, "s1": 0.525} | 1.36671984 → 1.22606354 | not_run |
| placement_04 | {"fill": 0.65, "s0": 0.05, "s1": 0.525} | 1.27150881 → 1.34829975 | oscillatory |
| placement_05 | {"fill": 0.7, "s0": 0.05, "s1": 0.525} | 1.39380300 → 1.25465308 | admissible_estimate |
| placement_06 | {"fill": 0.6, "s0": 0.05, "s1": 0.55} | 1.36322335 → 1.22646887 | not_run |
| placement_07 | {"fill": 0.65, "s0": 0.05, "s1": 0.55} | 1.34426255 → 1.34010923 | nonpositive_order |
| placement_08 | {"fill": 0.7, "s0": 0.05, "s1": 0.55} | 1.55695425 → 1.23974078 | not_run |
| placement_09 | {"fill": 0.6, "s0": 0.075, "s1": 0.5} | 1.29546278 → 1.20082134 | not_run |
| placement_10 | {"fill": 0.65, "s0": 0.075, "s1": 0.5} | 1.36366975 → 1.25624931 | nonpositive_order |
| placement_11 | {"fill": 0.7, "s0": 0.075, "s1": 0.5} | 1.26975729 → 1.15819357 | not_run |
| placement_12 | {"fill": 0.6, "s0": 0.075, "s1": 0.525} | 1.16027393 → 1.23561540 | oscillatory |
| placement_13 | {"fill": 0.65, "s0": 0.075, "s1": 0.525} | 1.35783004 → 1.17180870 | not_run |
| placement_14 | {"fill": 0.7, "s0": 0.075, "s1": 0.525} | 1.36962605 → 1.20281773 | not_run |
| placement_15 | {"fill": 0.6, "s0": 0.075, "s1": 0.55} | 1.15483915 → 1.28729616 | oscillatory |
| placement_16 | {"fill": 0.65, "s0": 0.075, "s1": 0.55} | 1.29129393 → 1.24723487 | not_run |
| placement_17 | {"fill": 0.7, "s0": 0.075, "s1": 0.55} | 1.37470747 → 1.23518607 | not_run |
| placement_18 | {"fill": 0.6, "s0": 0.1, "s1": 0.5} | 1.16684753 → 1.21021573 | oscillatory |
| placement_19 | {"fill": 0.65, "s0": 0.1, "s1": 0.5} | 1.32361066 → 1.20641165 | not_run |
| placement_20 | {"fill": 0.7, "s0": 0.1, "s1": 0.5} | 1.19184452 → 1.66128693 | oscillatory |
| placement_21 | {"fill": 0.6, "s0": 0.1, "s1": 0.525} | 1.15726873 → 1.22574110 | oscillatory |
| placement_22 | {"fill": 0.65, "s0": 0.1, "s1": 0.525} | 1.28700928 → 1.22738204 | not_run |
| placement_23 | {"fill": 0.7, "s0": 0.1, "s1": 0.525} | 1.16404795 → 1.18265597 | not_run |
| placement_24 | {"fill": 0.6, "s0": 0.1, "s1": 0.55} | 1.20748675 → 1.18760752 | not_run |
| placement_25 | {"fill": 0.65, "s0": 0.1, "s1": 0.55} | 1.38176450 → 1.20979176 | not_run |
| placement_26 | {"fill": 0.7, "s0": 0.1, "s1": 0.55} | 1.26062052 → 1.26725504 | oscillatory |
| placement_27 | {"fill": 0.6, "s0": 0.125, "s1": 0.5} | 1.13661605 → 1.20937917 | oscillatory |
| placement_28 | {"fill": 0.65, "s0": 0.125, "s1": 0.5} | 1.15967778 → 1.15850396 | not_run |
| placement_29 | {"fill": 0.7, "s0": 0.125, "s1": 0.5} | 1.32961085 → 1.20949279 | not_run |
| placement_30 | {"fill": 0.6, "s0": 0.125, "s1": 0.525} | 1.14302370 → 1.17879343 | not_run |
| placement_31 | {"fill": 0.65, "s0": 0.125, "s1": 0.525} | 1.09773744 → 1.20752052 | oscillatory |
| placement_32 | {"fill": 0.7, "s0": 0.125, "s1": 0.525} | 1.17263778 → 1.22535036 | oscillatory |
| placement_33 | {"fill": 0.6, "s0": 0.125, "s1": 0.55} | 1.10296527 → 1.17564575 | not_run |
| placement_34 | {"fill": 0.65, "s0": 0.125, "s1": 0.55} | 1.16981633 → 1.22821031 | oscillatory |
| placement_35 | {"fill": 0.7, "s0": 0.125, "s1": 0.55} | 1.16053962 → 1.18825201 | not_run |
| fillet_00 | {"fill": 0.65, "s0": 0.05, "s1": 0.5, "width": 5.0} | 1.20857227 → 1.15198142 | not_run |
| fillet_01 | {"fill": 0.65, "s0": 0.05, "s1": 0.525, "width": 5.0} | 1.22501563 → 1.19919367 | not_run |
| fillet_02 | {"fill": 0.65, "s0": 0.05, "s1": 0.5, "width": 5.5} | 1.33345829 → 1.28934172 | nonpositive_order |
| fillet_03 | {"fill": 0.65, "s0": 0.05, "s1": 0.525, "width": 5.5} | 1.19355870 → 1.27384636 | oscillatory |
| fillet_04 | {"fill": 0.65, "s0": 0.05, "s1": 0.5, "width": 6.5} | 1.55090155 → 1.30307487 | admissible_estimate |
| fillet_05 | {"fill": 0.65, "s0": 0.05, "s1": 0.525, "width": 6.5} | 1.33166171 → 1.50774643 | oscillatory |
| fillet_06 | {"fill": 0.65, "s0": 0.05, "s1": 0.5, "width": 7.0} | 1.41800549 → 1.36906825 | admissible_estimate |
| fillet_07 | {"fill": 0.65, "s0": 0.05, "s1": 0.525, "width": 7.0} | 1.39925538 → 1.46134266 | oscillatory |
| local_00 | {"fill": 0.65, "s0": 0.05, "s1": 0.5, "width": 6.0} | 1.31426827 → 1.16185227 | not_run |
| local_01 | {"fill": 0.65, "s0": 0.05, "s1": 0.502, "width": 6.0} | 1.39850251 → 1.27422541 | admissible_estimate |
| local_02 | {"fill": 0.65, "s0": 0.05, "s1": 0.505, "width": 6.0} | 1.39630350 → 1.32143776 | nonpositive_order |
| local_03 | {"fill": 0.65, "s0": 0.05, "s1": 0.51, "width": 6.0} | 1.23670962 → 1.22980531 | not_run |
| local_04 | {"fill": 0.65, "s0": 0.052, "s1": 0.5, "width": 6.0} | 1.55419063 → 1.31922480 | admissible_estimate |
| local_05 | {"fill": 0.65, "s0": 0.052, "s1": 0.502, "width": 6.0} | 1.35041180 → 1.31360191 | oscillatory |
| local_06 | {"fill": 0.65, "s0": 0.052, "s1": 0.505, "width": 6.0} | 1.24447265 → 1.22069499 | not_run |
| local_07 | {"fill": 0.65, "s0": 0.052, "s1": 0.51, "width": 6.0} | 1.22820669 → 1.32468491 | oscillatory |
| local_08 | {"fill": 0.65, "s0": 0.055, "s1": 0.5, "width": 6.0} | 1.31148064 → 1.22975174 | not_run |
| local_09 | {"fill": 0.65, "s0": 0.055, "s1": 0.502, "width": 6.0} | 1.21969403 → 1.23760015 | nonpositive_order |
| local_10 | {"fill": 0.65, "s0": 0.055, "s1": 0.505, "width": 6.0} | 1.22547997 → 1.36713721 | oscillatory |
| local_11 | {"fill": 0.65, "s0": 0.055, "s1": 0.51, "width": 6.0} | 1.21800808 → 1.23489825 | nonpositive_order |
| local_12 | {"fill": 0.65, "s0": 0.06, "s1": 0.5, "width": 6.0} | 1.32071234 → 1.27356701 | nonpositive_order |
| local_13 | {"fill": 0.65, "s0": 0.06, "s1": 0.502, "width": 6.0} | 1.31832613 → 1.33442958 | oscillatory |
| local_14 | {"fill": 0.65, "s0": 0.06, "s1": 0.505, "width": 6.0} | 1.26934462 → 1.16526834 | not_run |
| local_15 | {"fill": 0.65, "s0": 0.06, "s1": 0.51, "width": 6.0} | 1.27005558 → 1.20614069 | not_run |

Native GCI tests: 2 passed, 0 failed. Python and shell syntax and source/receipt integrity checks pass. All numerical work used CPUs 12,13 and CARGO_BUILD_JOBS=2. The previous 38-test native library result is retained; it was not rerun for this adapter-only phase.

Review `phase2_receipt.json`. Full raw GCI, side-A4 mesh, per-trial CSV/JSON/logs, refinement and export evidence remain under `target/G16-06-RSV2-08/phase2/`. The sealed genome/config/receipt are unchanged; adjusted variants are not substituted into the production exporter or declared manufacturing-qualified.

Reproduce in the lane (preserve prior logs before another sweep):
```sh
taskset -c 12,13 bash experiments/G16-06-RSV2-08/build_phase2.sh
taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/run_phase2.py
taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/phase2_window_search.py
taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/phase2_convergence_search.py
taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/phase2_convergence_search.py --local
taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/summarize_phase2.py
```

Delivery: Phase-2 code, report and numerical receipt are saved in external checkpoint `2100b75384ec3c0f19ece97bfa9a492784ee3489` with all required Agent-* trailers, verified after it appeared during finalization. Local delivery metadata and lane COMMS remain uncommitted because shared Git metadata is read-only. Both lane milestone logs are updated; shared main COMMS requires captain relay. No PR, push, stacking or deletes. Final source archive: target/G16-06-RSV2-08/phase2-sources-final.tar.gz.
