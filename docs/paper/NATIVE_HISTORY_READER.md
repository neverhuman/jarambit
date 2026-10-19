# Native evolution history reader

`crates/paper-artifacts/src/native_history.rs` reads the retained, headerless
14-column native CSV dialect. This reporting component does not run evolution,
score a design, evaluate mechanics, or establish qualification.

The caller must first admit the captured bytes, producer source, column schema,
configuration, local patches and available binary identity. Matching fourteen
columns is insufficient proof of their meaning. The declaration in this module
matches the schema in baseline `009f5ac5` `crates/crescent/src/qd.rs:305`; an actual
archive still needs its own producer binding. No live or V1 archive is admitted
by adding this component. The publication generator does not call it yet.

- Every row is data, including generation 0 / island 0. An unexpected header,
  duplicate generation/island pair, malformed integer, unsupported scalar,
  empty file, or incomplete last line is rejected before any plot is produced.
- Explicit source `NaN` becomes a missing measurement, never zero. Other finite
  scalar values, including negative contextual fitness, remain unchanged.
- Counts keep their original field names. Generation/island summaries are not
  individual births, and fields named `evals` do not establish unique evaluations.
- Missing generations are reported as compact inclusive ranges; no observations
  are created to fill them. Row order and a SHA-256 of the exact input remain.
- Expected island count must come from admitted configuration. Without it,
  expected/missing island counts stay unknown. A partial observed generation is
  reported explicitly rather than silently called complete.

The eight unit cases use clearly synthetic reporting rows. They exercise the
failure modes found in the archive handoff and are not scientific observations.
An interrupted or empty source may still be retained as raw archive evidence;
this reader's rejection prevents treating it as a publishable complete history.

Audit an independently captured input without changing it:

```sh
cargo run --locked --manifest-path crates/paper-artifacts/Cargo.toml \
  --example native_history_audit -- /absolute/path/history_live.csv 4
```

Use `unknown` instead of `4` when configuration is not established. The JSON
receipt reports row membership, source hash, observed generation bounds, gaps,
partial generations and unknown scalar cells. Its null execution-completion,
birth-count and qualification fields intentionally remain unresolved. The tool
does not infer a candidate ID from a generation/island selector.

Next publication integration depends on admitting the corrected V2 handoff. Verify archive membership,
raw-byte hashes, producer/schema bindings and per-record kind before invoking
this reader. Preserve first-revision errors separately, and do not convert their
unscoped historical pass flags into qualification claims.
