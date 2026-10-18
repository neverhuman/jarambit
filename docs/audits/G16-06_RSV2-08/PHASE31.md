# G16-06-RSV2-08 Phase 31 — Markdown heading consistency

Task G16-06-RSV2-08-p32 audits all 16 PHASE15..PHASE30 reports. Phases 15–20
are under `experiments/G16-06-RSV2-08`; Phases 21–30 are under this audit
directory. The fixed path list prevents silent omissions. The output is
`phase31_markdown_heading_consistency_audit.json`.

Each record includes phase, relative path, raw SHA-256, byte size, line count,
heading count, H1 count, ordered heading levels and heading text/line positions.
The audit requires exactly one H1, first in the heading sequence, with the
matching `G16-06-RSV2-08 Phase N — ` title prefix. Empty headings, skipped
ascending levels, repeated case-folded/whitespace-normalized heading text and
unclosed code fences are inconsistencies. Descending levels are allowed. The
receipt records zero violations and aggregate bytes, lines and heading counts.

The parser covers ATX headings with one to six hash marks and up to three
leading spaces, whitespace after the opening marks and optional closing hash
marks. Backtick and tilde fences suppress heading extraction; closing fences
must match their character and have at least the opener's length with no info
suffix. Four-space indented lines are excluded. These reports use this subset;
Setext headings, HTML blocks, container Markdown and rendered anchors are
outside scope. This is a static structure audit, not a full CommonMark parser.

Default verification retains the earlier full receipt chain, then recomputes
the heading audit from live bytes. Six new tests cover exact live inventories,
fenced code and closing marks, malformed structures, invalid UTF-8, forged
records, missing/byte-changed/inconsistent reports and static output scope.
The combined relocated read-only fixture now includes all 16 reports and the
new receipt, with no Git, target evidence, subprocess or nonlocal-read dependency.

The source changes require refreshes of dependent Phase 24–30 metadata. Exact
prior copies are saved under `target/G16-06-RSV2-08/phase32/prior_*.json`.
Phase 29 AST declarations are recomputed and Phase 30 raw receipt hashes follow
the metadata changes. Earlier independent pins remain active; historical
reports and Phase 15–23 receipts are unchanged. No self-referential pin is added.

```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_g16_06_rsv2_08_receipts.py --write-script-inventory --write-script-cross-check --write-summary --write-manifest-cross-check --write-cumulative-index --write-assertion-inventory --write-json-keysets --write-markdown-headings
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts/tests -p 'test_*.py' -v
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s experiments/G16-06-RSV2-08 -p 'test_*.py' -v
git diff --check
```

Static only: no builds, solves, package installation, designs or engineering
assessment. No gate or threshold changes. Engineering qualification remains
unassessed, qualified count zero and full qualification false. Baseline:
`d6743d3`. Execution uses CPUs 12,13, nice -n 10 (inherited effective nice 19)
and CARGO_BUILD_JOBS=2. Changes remain uncommitted. COMMS.md is not edited.
New report and receipt literal host-path counts are zero.

Validation result: combined verifier PASS; 71/71 verifier tests and 79/79
historical tests pass. The audit records 16 reports and 16 H1 headings, with
zero violations. The 58-file relocated fixture passes read-only verification.
Updated AST metadata records 26 check functions, 12 verification entry points,
39 explicit raises, zero assert statements and 71 test methods in 11 classes;
the two source files total 88,216 bytes. Structural comparisons prove
metadata-only refreshes for Phases 24–28 and 30, plus exact Phase 29 source
inventory recomputation. Seven prior receipts match baseline bytes; all
PHASE15..30 reports and Phase 15–23 receipts are unchanged. Whitespace and
new-output host-path scans pass. COMMS bytes and SHA-256 are unchanged.
Test logs, prior receipt copies and preservation proof remain under
`target/G16-06-RSV2-08/phase32/`.
