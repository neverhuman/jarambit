# Paper build integration

The paper Makefile runs `synthetic-geometry-check` before figure generation and before direct generation of its first-page hero PNG. The `pdfs`, `main`, `supplement`, `all` and `check` routes inherit this dependency. `docs/paper/reproduce.sh` uses these targets. Reporting-only `artifacts` does not run the preflight.

The preflight executes the actual geometry-changing smoother on a generated rough sphere. It must satisfy the benchmark's unchanged acceptance criteria before a render command can start. This is a synthetic algorithm check; it does not smooth, replace, reassess or qualify the paper's source-bound production CAD. It is not a production mesh-smoothing hook.

Each invocation creates a fresh `target/ieee/paper/synthetic-geometry/run.*/` directory with exported rough/smoothed/reference OBJ and STL, full vertex measurements, numerical results, a comparison graphic, and source/output SHA-256 manifests. `LATEST` identifies only a completed successful attempt. A failed run stops Make, even if an earlier successful receipt exists. Previous outputs remain available for inspection.

The existing version-check workflow runs the 13 focused tests, strict Clippy and the preflight, then the manuscript build. It uploads the fresh preflight receipts and meshes with the paper build artifacts. The separate root Rust workspace does not discover this standalone crate automatically; explicit workflow coverage is intentional.

Local integration evidence is in [results/integration/VERIFICATION.json](results/integration/VERIFICATION.json). The two new tests exercise the real Makefile: dependency order before raster/TeX consumers, including direct hero generation; and a failing Cargo producer that must prevent every render command and a new success pointer. An actual isolated cover raster smoke run also completed after the synthetic benchmark. The original 11-test benchmark receipt under `results/verified/` remains a historical standalone-run record.

The original standalone source commit was `c403313f5684d8bc2a20be15749ebfd949981e24`. Its files were transplanted onto repository main commit `510713a8b0a9ba0ae6b4740c8ab81f1e3e3bc5b6`; unrelated Git histories were not merged. Scientific evidence, production geometry, manuscript figures and publication scoring inputs were outside this integration's write scope. Hosted CI, independent approval and the final main merge are separate delivery observations.
