# Regression observations

Fresh evaluations from `crescent-evolve`, `crescent-smoke`, and `auto-evolve`
Stage 1 automatically write immutable compressed records below the selected
output directory's `regression/run-*/`. Set `JARAMBIT_REGRESSION_ROOT` to a
dedicated shared spool to collect several searches on one host. They retain 128 full-precision genes,
material, 35 CAD descriptors, and 26 separately typed FEM outcomes. Search scores,
selection, and certification are unchanged. A collection failure is reported as
an error instead of claiming a complete dataset.

The full design and rollout plan is in
[`docs/specs/REGRESSION_DATASET_FACTORY.md`](../../docs/specs/REGRESSION_DATASET_FACTORY.md).

## Assemble and measure

```bash
cargo build --release --locked -p jarambit -p jarambit-datasets
./target/release/jarambit regression-registry > target/registry.json
./target/release/jarambit-datasets assemble \
  out/my-search/regression target/registry.json out/regression-snapshots
# SNAPSHOT is one directory printed by assemble.
./target/release/jarambit-datasets cv \
  "$SNAPSHOT" "$CATBOOST_BIN" out/regression-cv datasets/regression/cv.v1.json
```

Every `dataset.csv` contains only features and a final numeric `target` column.
Row IDs, ancestry, generation, and run IDs live in `rows.json`. The manifest
names the target, units, evaluator context, CSV/metadata hashes, exclusions,
and duplicate count. Missing/failed targets are excluded without imputation.
Repeated designs are deduplicated within one evaluator context; contradictory
repeats fail. No other test outputs enter the feature matrix.

The default is five folds held out by whole run, at least 200 rows, five groups,
and 20 held-out rows in every fold. One run cannot establish cross-run lift.
Fixed-iteration CPU CatBoost fits on the training fold only. Reports include
pooled out-of-fold Pearson r, R², RMSE, MAE, RMSE lift against training-fold means,
per-fold metrics, assignments, and predictions. `insufficient_data` is explicit.
The real CatBoost synthetic control tests software operation, not CAD performance.

The initial assembler refuses more than 50,000 unique observations or more than
64 dataset/context views in one suite. It never silently subsamples. Partition
inputs for now; the spec defines the external-sort/columnar expansion for large
archives. Search observations are not physical measurements or certificates.

## Publish sealed batches

Use a dedicated clean worktree/checkout on `data/regression-HOST`, with its own
Git credentials. Initialize it once from current main and push the data branch.
The publishing process validates receipts, commits only
`datasets/regression/batches/BATCH_ID/`, and uses a normal fast-forward push.
It never force-pushes or changes main. A rejected push is a visible error.

```bash
./target/release/jarambit-datasets publish \
  out/my-search/regression /path/to/dedicated-checkout \
  data/regression-xbabe1 xbabe1-2026-10-20
```

The example systemd service, timer, and environment file in `ops/datasets/`
run this every 15 minutes after an operator provisions the service account,
readable spool, clean checkout, binary, and Git authentication. They are templates;
this change does not install a service on the research machines. Rotate the input
spool/batch before 64 MiB compressed; oversized publication fails before copying.
Keep raw solver meshes and models outside Git, with their existing custody rules.

The new GitHub workflow tests the contracts and real CatBoost control, then
assembles committed batches and publishes per-dataset results as an Actions
summary/artifact. It runs on relevant PRs, main/data branch pushes, and manual
dispatch. There are no committed empirical batches initially, so it truthfully
reports that no CAD lift has yet been measured. CatBoost 1.2.10 is checksum-pinned
in `catboost.lock.json`; no Python training code is added.

## Scope

Implemented: durable online search collection, generic JSONL ingestion, immutable
aggregation, grouped CatBoost evaluation, explicit Git publishing, CI, and
service templates. Higher-fidelity Stage-2/solid FEM adapters, legacy archive
import, full ancestry-component splits, external-sort assembly, uncertainty
intervals, and real corpus lift results are specified follow-ups. The older
`crates/evolve` Monte Carlo lane is not automatically instrumented by this change.
