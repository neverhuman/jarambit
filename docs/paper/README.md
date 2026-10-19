# When CAD Evolves

**Research white-paper draft.** Intelligent CAD, evolutionary search, agent-assisted representation research, and auditable engineering evidence. Human author approval and independent scientific review remain pending.

Source baseline: `009f5ac520fa3afe8ed8025230eef604dbb7f922`, inspected 18 October 2026 (America/Denver). New source package supplements the existing [living technical report](../WHITEPAPER.md) and [engineering reference](../ENGINEERING_GATES_AND_EVOLUTION.md).

[Engineering specification](../PAPER_AND_README_ENGINEERING_SPEC.md) · [Complete test atlas](../tests/README.md) · [All 178 science definitions](../tests/science.md) · [Red-team review](RED_TEAM.md) · [Implementation receipt](IMPLEMENTATION_RECEIPT.md)

## Format and build

The manuscript uses `IEEEtran` in journal mode, 10 pt, with a custom single-column first page containing title, abstract, then the Blender render beside actual evolution data. Index terms follow the figure. Remaining pages use two columns. This is an **IEEE-style white paper**, not an unmodified venue submission template. A formal submission needs a selected venue's current template, verified authorship, disclosure, references and supported research contribution. No IEEE acceptance, DOI or badge is claimed. [IEEE Access guidance](https://ieeeaccess.ieee.org/authors/submission-guidelines/).

Dependencies: Rust 1.85.1, locked Cargo dependencies, LuaLaTeX, IEEEtran, TeX Gyre fonts, librsvg `rsvg-convert`, Poppler `pdfinfo`/`pdftoppm`, `make`, and `shasum`. Build from repository root; shell commands here use the desktop's RTK convention:

```sh
rtk proxy env CARGO_TARGET_DIR="$PWD/target" cargo run --locked --manifest-path crates/paper-artifacts/Cargo.toml --example paper_artifacts
rtk proxy make -C docs/paper check
```

Output: `target/ieee/paper/main.pdf`, source/output SHA-256 lists, tool versions, PDF metadata and logs. The Makefile's check mode verifies 15–20 pages and rejects overfull boxes or unresolved references/citations. The legacy pinned Pandoc target remains separate and unchanged. Byte-identical PDF reproduction across toolchain versions is not claimed.

## Evidence contract

`crates/paper-artifacts/src/publication.rs` generates the explanatory atlas, complete frozen science index, vector figures, numerical macros, summary and hash manifest. `--check` performs no writes. It reads existing evidence without changing it and invokes no CAD, FEM, evolutionary or physical test.

Before publishing, the reader checks that the selected score and label agree with the final retained Hall of Fame, matching Stage-2 identities agree, objective arithmetic reconciles within recorded rounding, and all 26 predicate flags agree with their tally and failed-ID list. These are reporting-consistency checks; they do not establish equivalence between screening and assessment stages or validate the underlying mechanics.

The `version-check` workflow explicitly tests this standalone Cargo workspace, checks generated freshness, and builds the new manuscript. A successful new-paper build is configured to upload `ieee-paper-<commit SHA>` with its PDF and build receipts. This artifact is separate from the older publication PDFs and is available only after that workflow succeeds.

![Retained reporting summary with incomplete stage lineage](generated/fitness.svg)

The selected w411c3 cohort reports best fitness **74.207299**, higher is better, with placeholder inputs and no qualified design. Its best CAD geometry and full round populations are absent. The reporting producer can substitute assessment values for screening values; point-level stage lineage is incomplete. This figure now appears in the results section, preserving the recorded profile and short summaries without asserting one homogeneous objective-convergence experiment. The historical inset uses a different score regime.

![Historical full-birth population behavior](generated/population.svg)

The w282 plot contains **1,984 retained births: 1,455 FEM-complete and 529 not evaluated**. Blue dots and amber crosses distinguish these states; stacked bars show their per-generation counts. Its original score includes legality-dependent penalties and contextual novelty and is not `W_ULTRALIGHT_v1`. The highest stored score belongs to ID 1095, which did not pass its later strict assessment. The producer reports that GA passers did not beat the baseline; the named exported examples were controlled probes under the historical model. Source membership, original producer, status composition and the assessment join appear in [summary.json](generated/summary.json); input/output hashes appear in [manifest.json](generated/manifest.json).

## Manuscript flow

The opening shows the exact assessed export `cert_pipeline_r01_p14_c02` beside the historical w282 population and its stored search maxima. These are different cohorts. The render is an assessed example, not a best-design identity claim or a qualified product. The body develops intelligent design and related work; gene/decoder/CAD identity; evolution and population semantics; actual weighted fitness with individual G16 and RS-v2 subsections; all 18 battery evaluators; numerical verification and assurance; retained results; prospective agents; reproducibility; manufacturing limits; and an expansive civilian-use conclusion.

## Source-bound Blender cover

[Frozen figure inputs](figure-inputs.v1.json) bind the original STL, assessment and status records. The new lossless [master](figures/v2/hero_dark.png), [scene recipe](figures/v2/hero_dark.scene.json) and [render receipt](figures/v2/hero_dark.manifest.json) preserve the exact export and its original flat face normals. Visible export faceting is retained; the illustrative titanium shader is not evidence of a printed surface. The Rust publication reader checks the render's input/output hashes, subject identity and before/after imported-mesh digests before publishing. CI builds from the committed master and does not run Blender.

Three 1,200-pixel/48-sample lighting proofs preceded the selected 3,600 × 2,800-pixel/256-sample master (Cycles CPU, four threads, seed 42, denoising, AgX). Proofs and logs remain under `target/paper/blender-v2`. To reproduce with the pinned Blender 4.5.3 executable supplied through `BLENDER`:

```sh
rtk proxy python3 -B ops/render/blender/rerender.py --profile publication --output-root docs/paper/figures/v2 --recipe dark --width 3600 --height 2800 --samples 256
```

The publication profile requires an explicit isolated output root and applies no typography overlay, clipping, mesh modification or depth of field. The profile extends the existing `rerender.py`/`candidate_scenes.py` bindings. Archived assets, their hashed `render_paper.py`/`paper_scenes.py` sources, and legacy defaults are preserved byte-for-byte.

## Actual retained plate fields

![Actual retained envelope and unit-response fields](generated/plate-fields.svg)

The four-panel numerical figure uses the cover example's original D1 arrays: **13,503 nodes and 25,209 triangles**, with one saved value per triangle. The [field contract](field-inputs.v1.json) freezes source identity, units, producer-source hashes and interpretation. The Rust reader in `crates/paper-artifacts/src/plate.rs` creates [the projection receipt](generated/plate-fields.json), including exact masked cell IDs, units, full-range scales, palette and coordinate transforms. The figure also has a hash manifest checked by the existing figure verifier.

The envelope is already scaled stress (MPa); lateral and ring arrays are unit responses (MPa/N), and torsion is MPa/(N mm). These fields have independent linear legends. No values are clipped, interpolated or smoothed. Gray distinguishes 75 source-zeroed envelope cells and 14,875 source-zeroed ring cells from raw rounded zeros. The original capacity-excluded tip region remains visible in the raw lateral and torsion arrays. A positive stored ring value resolves one cell whose mask membership is ambiguous from rounded coordinates alone; an ambiguous zero would stop generation.

The PDF embeds a 3,600-pixel raster projection (over 500 dpi at publication width) to avoid white seams introduced by some PDF renderers between adjacent vector triangles. The original SVG remains available for cell-level inspection. The cell fills and legend anchors are checked against source values; no lighting or tone mapping is applied to this numerical image.

These are the retained **0.25 mm² plate-level fields**. Exported capacity summaries combine two mesh levels, so the displayed field must not be presented as the limiting capacity mesh, a strict-refinement result or a solid-element field. No full strict, solved solid, buckling-mode or fatigue field is admitted to this publication input set. The manuscript now explains stress recovery and masks alongside G16-05 through G16-12; additional subsection visuals remain outstanding where the necessary observations are absent.

The conclusion treats aerospace fixtures, robotics, thermal management, medical-device research, instruments and energy systems as future opportunities. It does not claim that printing titanium establishes unprecedented tolerances or that a simulation qualifies a product. Stronger publication claims need controlled benign research and independent reconstruction evidence.
