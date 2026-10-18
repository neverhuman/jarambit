# G16-06-RSV2-08 Phase 22 — Static Markdown report digest index

Task G16-06-RSV2-08-p23 indexes every tracked `PHASE*.md` file directly in
`docs/audits/G16-06_RSV2-08` at baseline `7856869`. That snapshot contains one
report, `PHASE21.md`. Its exact size, SHA-256, line count and zero literal
host-path count are recorded in `phase22_markdown_report_digest_index.json`.
The new untracked Phase 22 report is an output, not its own indexed input.

The index and sorted path list have independent digest pins in
`scripts/verify_g16_06_rsv2_08_receipts.py`. Default verification now checks both
the unchanged Phase 21 JSON inventory and this Markdown index. Historical JSON
verification remains available through its existing function and preserves the
original snapshot, digest pins and strict finite-number checks.

Line count is the number of decoded UTF-8 `splitlines()` entries: an empty file
has zero lines, an unterminated final line counts once, and CRLF counts as one
line break. Exact bytes and hashes preserve line-ending and encoding differences.
The literal host-path scan rejects POSIX home, Users, root and temporary-directory
prefixes, plus Windows user/temporary paths. Relative repository paths are
accepted. All indexed reports must have zero matches; detector definitions do
not copy a literal legacy host prefix into new source files.

Seven new regressions cover report coverage, sizes, hashes, line counts, host
paths, independent index pinning and missing/corrupt evidence. A relocated
32-file fixture verifies both inventories without Git metadata, target evidence,
environment settings, subprocesses, writes or reads outside the fixture root.
Tests cover empty, unterminated, CRLF and Unicode text and reject invalid UTF-8.

```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_g16_06_rsv2_08_receipts.py
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts/tests -p 'test_*.py' -v
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s experiments/G16-06-RSV2-08 -p 'test_*.py' -v
git diff --check
```

`--write-markdown` regenerates this frozen Markdown index using read-only Git
discovery at its pinned baseline. Existing `--write` retains its earlier JSON
regeneration meaning. Default verification only reads saved files.

This is a static report index audit only. No builds, numerical solves, package
installations, design generation or engineering assessments run. Historical
reports, JSON inventories and measurement data remain unchanged; engineering
qualification is unassessed, qualified_count is zero and full_qualification is
false. No gate or threshold changes. Execution uses CPUs 12,13 with `nice -n 10`
(inherited effective nice 19) and CARGO_BUILD_JOBS=2. Leave changes uncommitted.

Validation result: combined verifier PASS, 17/17 verifier tests and 79/79
historical tests pass. The complete tracked report snapshot is one file,
3,289 bytes and 50 lines, with zero literal host paths and an exact baseline
SHA-256 match. The earlier JSON inventory is byte-identical. Whitespace checks
pass; logs remain under `target/G16-06-RSV2-08/phase23/`.
