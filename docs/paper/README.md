# When CAD Evolves

**Research white-paper draft.** Intelligent CAD, evolutionary search, agent-assisted representation research, and auditable engineering evidence. Human author approval and independent scientific review remain pending.

Source baseline: `009f5ac520fa3afe8ed8025230eef604dbb7f922`, inspected 18 October 2026 (America/Denver). New source package supplements the existing [living technical report](../WHITEPAPER.md) and [engineering reference](../ENGINEERING_GATES_AND_EVOLUTION.md).

[Engineering specification](../PAPER_AND_README_ENGINEERING_SPEC.md) · [Complete test atlas](../tests/README.md) · [All 178 science definitions](../tests/science.md) · [Red-team review](RED_TEAM.md) · [Implementation receipt](IMPLEMENTATION_RECEIPT.md)

## Format and build

The manuscript uses `IEEEtran` in journal mode, 10 pt, with a custom single-column first page containing title, abstract and quantitative hero. Remaining pages use two columns. This is an **IEEE-style white paper**, not an unmodified venue submission template. A formal submission needs a selected venue's current template, verified authorship, disclosure, references and supported research contribution. No IEEE acceptance, DOI or badge is claimed. [IEEE Access guidance](https://ieeeaccess.ieee.org/authors/submission-guidelines/).

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

The selected w411c3 cohort reports best fitness **74.207299**, higher is better, with placeholder inputs and no qualified design. Its best CAD geometry and full round populations are absent. The reporting producer can substitute assessment values for screening values; point-level stage lineage is incomplete. The figure preserves the recorded profile and short summaries without asserting one homogeneous objective-convergence experiment. The historical inset uses a different score regime. Replacement of this interim cover graphic by the source-bound Blender composition remains unfinished.

![Historical full-birth population behavior](generated/population.svg)

The w282 plot contains **1,984 retained births: 1,455 FEM-complete and 529 not evaluated**. Blue dots and amber crosses distinguish these states; stacked bars show their per-generation counts. Its original score includes legality-dependent penalties and contextual novelty and is not `W_ULTRALIGHT_v1`. The highest stored score belongs to ID 1095, which did not pass its later strict assessment. The producer reports that GA passers did not beat the baseline; the named exported examples were controlled probes under the historical model. Source membership, original producer, status composition and the assessment join appear in [summary.json](generated/summary.json); input/output hashes appear in [manifest.json](generated/manifest.json).

## Manuscript flow

The opening introduces the best retained reporting profile and its limits. The body develops intelligent design and related work; gene/decoder/CAD identity; evolution and population semantics; actual weighted fitness with individual G16 and RS-v2 subsections; all 18 battery evaluators; numerical verification and assurance; retained results; prospective agents; reproducibility; manufacturing limits; and an expansive civilian-use conclusion.

The conclusion treats aerospace fixtures, robotics, thermal management, medical-device research, instruments and energy systems as future opportunities. It does not claim that printing titanium establishes unprecedented tolerances or that a simulation qualifies a product. Stronger publication claims need controlled benign research and independent reconstruction evidence.
