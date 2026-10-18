# G16-06-RSV2-08 Phase 3

Status: running; 2/9 candidates fully evaluated. All numerical work uses CPUs 12,13; CARGO_BUILD_JOBS=2. No native source, gate, load, support constant or threshold changed.

The assignment uses the frozen repository sequence floor **1.20**. The earlier internal 1.25 prefilter is absent; 1.25 is reported only as a comparison. All three FEM levels, actual exported-triangle sharpness/point/census and side-A4 are evaluated for every candidate, including failures. New interpolation candidates also receive native 4/1/.25 GCI; the three prior fits are reused with exact input/source/mesh identity checks.

| Candidate | Cert lat N | Cert fatigue | Cert seq | Strict lat N | Strict fatigue | Strict seq | GCI admitted | Export | A4 | Scoped pass |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| placement_05 | 399.115859 | 1.114860 | 1.231106 | 386.841161 | 1.080573 | 1.325638 | True | True | True | True |
| local_01 | 366.281155 | 1.023142 | 1.218227 | 385.507448 | 1.076848 | 1.330887 | True | True | True | True |

Scoped pass requires geometry, admitted GCI, sequence >=1.20 and prior strength targets (lateral >=358 N, fatigue >=1, hunter >=1.377) at certification/strict, export and side-A4 (eigenvalue >=1 and supported sequence >=certified). Five-level monotonicity and the old internal 1.25 comparison are reported separately in the receipt. Full RS-v2/manufacturing qualification remains false; human-max failures, soft mass objective and independent 3D checks are retained separately.

Exact per-mesh values, GCI fits, exported measurements, support constants, mesh/input hashes and failures are in phase3_receipt.json. Full raw outputs/logs/canonical CSVs/side-A4 meshes remain under target/G16-06-RSV2-08/phase3/. The sealed candidate/config and Phase-2 evidence are preserved.

Reproduce with retained lane binaries:
```sh
taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/run_phase3.py
```
