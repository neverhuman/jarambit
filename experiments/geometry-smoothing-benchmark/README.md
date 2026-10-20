# Synthetic geometry smoothing benchmark

Standalone Rust implementation of actual vertex-position smoothing. Its only input fixture is a made-up rough spherical housing, with no connection to project production CAD, fitness scoring, or exporters. The output is a changed triangle mesh in OBJ and STL; this does not reconstruct analytic STEP/B-rep CAD.

The default test passed: all 642 vertices moved; vertex radial RMS error fell from 0.423986 to 0.102598 mm, a 75.802% reduction. Volume differed from the clean reference mesh by -0.211001%. See [measured report](results/verified/REPORT.md), [before/after wireframe](results/verified/comparison.svg), and [numerical results](results/verified/metrics.json).

Generated OBJ/STL meshes and the full vertex-coordinate CSV are runtime artifacts, not repository CAD. Run the command below to produce them in a new output directory, or download the artifact from the standalone **Synthetic geometry benchmark** workflow. Historical receipts under `results/verified/` retain the original run's hashes, including hashes for artifacts no longer stored in Git; mesh/CSV links in that historical report describe the original run directory. Its source hashes bind the original standalone revision, not later validation, packaging or CI edits. Current workflow executions publish fresh source/output digests.

The benchmark has its own path-filtered CI workflow and manual dispatch. It is not a prerequisite of paper figure/PDF builds or required version-check. A synthetic sphere pass does not validate, smooth or qualify the paper's production CAD. The attempted paper-build coupling in PR head `3e80b424` was rejected during review and removed; its source and proof remain in that historical commit.

The workflow writes source/start metadata before invoking the producer, records its actual exit code and log, and hashes whatever output files exist. Upload also runs after a producer failure; failed tests can prevent a producer attempt, in which case there may be no mesh artifact. A successful upload does not turn a failed job into a passing benchmark. `AVAILABLE_OUTPUT_SHA256SUMS` can describe a partial failed run; inspect `EXIT_CODE`, the log and the benchmark result together.

The rejected initial-weight snapshot is intentionally partial: only `REPORT.md`, `metrics.json` and `comparison.svg` were retained. Raw mesh/CSV links inside its unchanged historical report are unavailable in Git. Reproduce that rejected setting with `--mu -0.53 --out results/new-rejected-run`; the declared criteria should fail with exit code 2. The retained failed report, metrics and graphic remain unchanged.

## Run

From this directory, using Rust 1.85.1 or newer:

```sh
rtk proxy env CARGO_TARGET_DIR="$PWD/target" cargo test --release --locked
rtk proxy env CARGO_TARGET_DIR="$PWD/target" cargo clippy --release --locked --all-targets -- -D warnings
rtk proxy env CARGO_TARGET_DIR="$PWD/target" cargo run --release --locked -- --out results/new-run
```

Output directories must be new; the program refuses overwrites. Exit 0 means the declared synthetic acceptance checks passed, exit 2 means the benchmark failed and saved its measurements, and exit 1 means invalid inputs or an operational error. Failed operational writes can leave partial outputs; a complete report and successful process are required.

Configurable CLI settings:

| Option | Default | Meaning |
|---|---:|---|
| `--iterations` | 20 | Number of paired passes, 1–1000 |
| `--lambda` | 0.5 | Positive smoothing pass weight, (0, 1] |
| `--mu` | -0.5 | Negative compensation pass weight, [-1, 0) |
| `--max-displacement-mm` | 2 | Maximum total movement from original coordinates |
| `--radius-mm` | 30 | Analytic reference radius |
| `--noise-mm` | 0.8 | Maximum synthetic radial noise amplitude, at most 10% radius |
| `--subdivisions` | 3 | Icosphere subdivision count, 1–5 |
| `--out` | `results/default` | New output directory |

Settings are not a promise that every combination passes. Insufficient movement and excessive shrinkage produce failing results. The default weights were corrected during development: initial mu -0.53 expanded this fixture and failed the original criteria. Criteria were retained. Two additional roughness phases were then tested using frozen defaults. This is development validation, not a blinded external benchmark.

## Algorithm

For vertex `i`, each simultaneous pass computes:

```text
L_i = mean(current neighbors of i) - current position of i
next_i = current_i + weight * L_i
```

Each iteration applies lambda, then mu using the new coordinates. With lambda 0.5 and mu -0.5, the linear unconstrained composite is `I - 0.25 L²`. This damps high-frequency mesh variation and reduces first-order shrinkage, but does not eliminate all dimensional drift. A radial projection onto the known sphere is never used. The analytic sphere is an assessment oracle only.

Boundary vertices and explicit library pins remain fixed. Total vertex movement is capped against the original mesh after each pair of passes. Each iteration checks finite positions, triangle area, oriented edge incidence, connectivity and face reversal relative to the source. Invalid output returns an error.

Before running experiments, acceptance criteria were set to: positions actually move, radial RMS improves by at least 50%, p95 improves, maximum error does not increase, volume remains within 1% of clean reference mesh, topology remains closed and connected with Euler characteristic 2, triangles remain nondegenerate and unreversed, and displacement stays within budget.

## Verification

Nine library tests and three CLI/export tests pass. They cover three noise amplitudes, two held-out roughness phases, fixed boundaries and pins, deterministic displacement caps, scale/translation invariance, invalid geometry and parameters, nonfinite measurement sources and displacement overflow, and rejection of unchanged geometry as a successful smoothing result. Export tests reload actual OBJ/STL coordinates, independently recompute sphere error, verify all vertices changed, and confirm STL triangles contain those exact coordinates. Negative controls enforce failing status for tiny movement budgets, excessive shrinkage, and a noiseless input.

There are no third-party dependencies. `src/lib.rs` owns smoothing and measurement; `src/main.rs` owns synthetic generation settings, exports, report and wireframe comparison; `tests/export_contract.rs` verifies the delivered mesh files. Generated outputs are immutable snapshots of their run; reruns use a new directory.

## Limits

This proves geometry changes and measured improvement on these synthetic fixtures. It does not prove success on arbitrary CAD, semantic feature preservation, continuous Hausdorff error, B-rep continuity, or functional performance. Vertex-to-sphere distances are exact at sample vertices; volume is compared with the clean triangulated sphere. Exhaustive triangle-intersection tests and arbitrary surface reconstruction are outside this benchmark. The source mesh remains unchanged.
