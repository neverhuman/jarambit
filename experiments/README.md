# experiments/

Self-contained studies. Each one keeps its own inputs, scripts and receipts so it can be re-audited later; none of them
is part of the G3 design pipeline in [`../crates/g3/`](../crates/g3/).

| study | generation | what it is | checked by |
|---|---|---|---|
| [`geometry-smoothing-benchmark/`](geometry-smoothing-benchmark/) | Paper B | standalone Rust vertex smoother on a synthetic housing; it produces the smoothing ladder | `synthetic-geometry-benchmark` workflow; `make -C docs/paper ladder-data` |
| [`pocket-surface-tension-smoothing/`](pocket-surface-tension-smoothing/) | Paper B | surface-tension smoothing of the winner's pockets (v2.3.0) | `make -C docs/paper`; the cover-figure manifests |
| [`G16-06-RSV2-08/`](G16-06-RSV2-08/) | v2 | the RSV2-08 blade-section and neck study, phases 2–8 | `ops/ci/tests/test_portable_phase7.py` (phases 7–8) |
| [`g16-06-rsv2-08/`](g16-06-rsv2-08/) | v2 | phase 9 of the same study | `ops/ci/tests/test_portable_phase7.py` via `ops/ci/g16_06_rsv2_08/audit_phase9.py` |
| [`science_v1/`](science_v1/) | v1 | the frozen science-v1 baseline manifest | referenced by `docs/v1/science/` |

The two `G16-06-RSV2-08` folders differ only in letter case. Their receipts record these exact paths, so they stay as
they are; on a case-insensitive file system both folders merge without a name clash.
