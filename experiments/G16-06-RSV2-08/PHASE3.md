# G16-06-RSV2-08 Phase 3

Status: complete; 9/9 candidates fully evaluated. All numerical work uses CPUs 12,13; CARGO_BUILD_JOBS=2. No native source, gate, load, support constant or threshold changed.

The assignment uses the frozen repository sequence floor **1.20**. The earlier internal 1.25 prefilter is absent; 1.25 is reported only as a comparison. All three FEM levels, actual exported-triangle sharpness/point/census and side-A4 are evaluated for every candidate, including failures. New interpolation candidates also receive native 4/1/.25 GCI; the three prior fits are reused with exact input/source/mesh identity checks.

| Candidate | Cert lat N | Cert fatigue | Cert seq | Strict lat N | Strict fatigue | Strict seq | GCI admitted | Export | A4 | Scoped pass |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| placement_05 | 399.115859 | 1.114860 | 1.231106 | 386.841161 | 1.080573 | 1.325638 | True | True | True | True |
| local_01 | 366.281155 | 1.023142 | 1.218227 | 385.507448 | 1.076848 | 1.330887 | True | True | True | True |
| local_04 | 406.556126 | 1.135643 | 1.211515 | 410.461355 | 1.146552 | 1.249354 | True | True | True | True |
| interp_s1_0.5003 | 376.106574 | 1.050588 | 1.309599 | 380.389952 | 1.062553 | 1.348120 | False | True | True | False |
| interp_s1_0.5006 | 379.841722 | 1.061021 | 1.253497 | 392.993851 | 1.097760 | 1.304883 | False | True | True | False |
| interp_s1_0.5009 | 393.122513 | 1.098119 | 1.252914 | 393.454897 | 1.099047 | 1.304004 | False | True | True | False |
| interp_s1_0.5012 | 354.797781 | 0.991066 | 1.375557 | 393.363411 | 1.098792 | 1.304309 | False | True | True | False |
| interp_s1_0.5015 | 391.381219 | 1.093255 | 1.246695 | 399.530123 | 1.116018 | 1.284177 | False | True | True | False |
| interp_s1_0.5018 | 366.263871 | 1.023094 | 1.166545 | 407.320819 | 1.137779 | 1.259613 | False | True | True | False |

Scoped pass requires geometry, admitted GCI, sequence >=1.20 and prior strength targets (lateral >=358 N, fatigue >=1, hunter >=1.377) at certification/strict, export and side-A4 (eigenvalue >=1 and supported sequence >=certified). Five-level monotonicity and the old internal 1.25 comparison are reported separately in the receipt. Full RS-v2/manufacturing qualification remains false; human-max failures, soft mass objective and independent 3D checks are retained separately.

Exact per-mesh values, GCI fits, exported measurements, support constants, mesh/input hashes and failures are in phase3_receipt.json. Full raw outputs/logs/canonical CSVs/side-A4 meshes remain under target/G16-06-RSV2-08/phase3/. The sealed candidate/config and Phase-2 evidence are preserved.

Reproduce with retained lane binaries:
```sh
taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/run_phase3.py
```

## Full Phase-3 measurements

All 27 plate evaluations, nine exports and nine side-A4 solves completed. Canonical numeric/material identity, the fatigue identity and source/raw hashes pass for every candidate. The unchanged native requirements consistency test passes (1/1). Reviewable input CSVs are in `phase3_candidates/`.

| Candidate | Area mm² | Lateral N | Fatigue SF | Hunter SF | Sequence |
| --- | --- | --- | --- | --- | --- |
| placement_05 | 0.25 | 400.086543731 | 1.117571765 | 1.538794399 | 1.231105586 |
| placement_05 | 0.125 | 399.115859478 | 1.114860328 | 1.535060998 | 1.310470046 |
| placement_05 | 0.0625 | 386.841161381 | 1.080573107 | 1.487850621 | 1.325637584 |
| local_01 | 0.25 | 366.281154976 | 1.023142326 | 1.408773673 | 1.218227368 |
| local_01 | 0.125 | 390.847757374 | 1.091764832 | 1.503260605 | 1.338192212 |
| local_01 | 0.0625 | 385.507447896 | 1.076847611 | 1.482720953 | 1.330887302 |
| local_04 | 0.25 | 406.556125802 | 1.135643411 | 1.563677407 | 1.211514978 |
| local_04 | 0.125 | 427.035698264 | 1.192849514 | 1.642444993 | 1.224791007 |
| local_04 | 0.0625 | 410.461355420 | 1.146551986 | 1.578697521 | 1.249353638 |
| interp_s1_0.5003 | 0.25 | 376.106573892 | 1.050587915 | 1.446563746 | 1.309598938 |
| interp_s1_0.5003 | 0.125 | 399.329562086 | 1.115457269 | 1.535882931 | 1.309769237 |
| interp_s1_0.5003 | 0.0625 | 380.389952462 | 1.062552783 | 1.463038279 | 1.348120333 |
| interp_s1_0.5006 | 0.25 | 392.939835781 | 1.097608687 | 1.511307061 | 1.253496744 |
| interp_s1_0.5006 | 0.125 | 379.841721800 | 1.061021397 | 1.460929699 | 1.376966679 |
| interp_s1_0.5006 | 0.0625 | 392.993850705 | 1.097759568 | 1.511514810 | 1.304883395 |
| interp_s1_0.5009 | 0.25 | 393.122513266 | 1.098118964 | 1.512009666 | 1.252914247 |
| interp_s1_0.5009 | 0.125 | 393.774489207 | 1.099940145 | 1.514517266 | 1.328246204 |
| interp_s1_0.5009 | 0.0625 | 393.454896684 | 1.099047419 | 1.513288064 | 1.304004212 |
| interp_s1_0.5012 | 0.25 | 354.797781337 | 0.991065531 | 1.364606851 | 1.388252121 |
| interp_s1_0.5012 | 0.125 | 380.230997651 | 1.062108771 | 1.462426914 | 1.375557338 |
| interp_s1_0.5012 | 0.0625 | 393.363411024 | 1.098791870 | 1.512936196 | 1.304309465 |
| interp_s1_0.5015 | 0.25 | 391.381219008 | 1.093254963 | 1.505312381 | 1.258488685 |
| interp_s1_0.5015 | 0.125 | 419.532735497 | 1.171891300 | 1.613587444 | 1.246695413 |
| interp_s1_0.5015 | 0.0625 | 399.530123493 | 1.116017503 | 1.536654321 | 1.284176595 |
| interp_s1_0.5018 | 0.25 | 366.263871147 | 1.023094046 | 1.408707197 | 1.166545452 |
| interp_s1_0.5018 | 0.125 | 393.431835314 | 1.098983001 | 1.513199367 | 1.329402929 |
| interp_s1_0.5018 | 0.0625 | 407.320818586 | 1.137779446 | 1.566618533 | 1.259613123 |

| Candidate | GCI fit / p | Max apex mm | Max half-angle ° | S2 point | Census | A4 eigenvalue | A4 seq / cert seq |
| --- | --- | --- | --- | --- | --- | --- | --- |
| placement_05 | admissible_estimate / 2.562994541054547 | 0.195144070 | 24.147325151 | True | [0, 0, 1] | 8.424593980 | 2.965487206 / 1.231105586 |
| local_01 | admissible_estimate / 1.150112112894349 | 0.195144070 | 24.147325151 | True | [0, 0, 1] | 9.930040130 | 4.389509643 / 1.218227368 |
| local_04 | admissible_estimate / 1.1253011352212974 | 0.195144070 | 24.147325151 | True | [0, 0, 1] | 8.225007573 | 3.164863336 / 1.211514978 |
| interp_s1_0.5003 | oscillatory / None | 0.195144070 | 24.147325151 | True | [0, 0, 1] | 8.224947466 | 3.216547123 / 1.309598938 |
| interp_s1_0.5006 | oscillatory / None | 0.195144070 | 24.147325151 | True | [0, 0, 1] | 8.224958473 | 3.848065981 / 1.253496744 |
| interp_s1_0.5009 | oscillatory / None | 0.195144070 | 24.147325151 | True | [0, 0, 1] | 9.930178949 | 3.825236247 / 1.252914247 |
| interp_s1_0.5012 | oscillatory / None | 0.195144070 | 24.147325151 | True | [0, 0, 1] | 9.930168653 | 3.840138455 / 1.375557338 |
| interp_s1_0.5015 | oscillatory / None | 0.195144070 | 24.147325151 | True | [0, 0, 1] | 9.930053316 | 3.717673746 / 1.246695413 |
| interp_s1_0.5018 | nonpositive_order / -0.4879966697912416 | 0.195144070 | 24.147325151 | True | [0, 0, 1] | 9.930148713 | 4.377874329 / 1.166545452 |

Scoped passing candidates: ['placement_05', 'local_01', 'local_04']. Full RS-v2/manufacturing qualification remains **false**. The native GCI/side-A4 executable is reused unchanged from Phase 2; its raw task label remains p2, with Phase-3 inputs and exact mesh/content hashes bound in this receipt.

The explicit prototype construction law, sealed config and each genome are required together. These results do not establish a passing witness through the production CSV-only exporter. Measured human-max shortfalls and the inherited flat-stock/collar inconsistency remain; mass is a soft objective.

GCI admission is a checked three-level fit; native `asymptotic_validation` remains `not_established`. Five-level sequence monotonicity is reported separately and no rejected mesh, export or support result is waived.

Audit after a completed run:
```sh
taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/audit_phase3.py
```

Delivery: external checkpoint `2ddaae93d6d6112d7ba617e81e862f3c0e5fcb06` contains an interim running receipt with **2/9** candidates. This worktree holds the complete **9/9** receipt and final audit. The final commit with required Phase-3 Agent-* trailers was attempted but blocked by read-only shared Git metadata (`index.lock`). Full final results, audit/input CSVs and delivery notes remain local. Evidence archives and manifests are retained in target/G16-06-RSV2-08/. No PR, push, stacking or deletes.
