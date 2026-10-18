# G16-06-RSV2-08 Phase 32 — SHA-256 lowercase hex64 format

Task G16-06-RSV2-08-p33 audits all 17 Phase 15–31 JSON receipts. The output
`phase32_hex64_digest_format_audit.json` records each receipt's phase, relative
path, byte size, raw SHA-256 and every selected digest value with its escaped
JSON Pointer. Per-file and total digest-field counts establish full coverage.

A key containing `sha256`, case-insensitively, selects its entire value. Scalar
values must be strings matching `[0-9a-f]{64}` exactly; maps and arrays select
all descendant leaves. This covers `sha256`, `input_sha256`, `genome_sha256`,
`expected_sha256`, `actual_sha256`, `keyset_sha256` and other digest names.
Empty selected containers, nulls, booleans, numbers, wrong lengths, uppercase,
nonhex characters and surrounding whitespace are rejected. Other strings are
not treated as digest values. JSON Pointer tokens escape tilde as `~0` and
slash as `~1`; arrays use zero-based indices. Strict JSON parsing also rejects
nonfinite numbers and non-object receipt roots.

Default verification retains the earlier full receipt chain and recomputes
the audit from exact live files. Format validation alone does not establish
that a digest matches an artifact; the earlier parity checks remain active.
Six new regression tests cover independent live field enumeration and counts,
nested maps/arrays, mixed-case key names, pointer escaping, malformed values,
forged or omitted records, missing files, byte-only changes and static scope.
The combined relocated fixture includes the new receipt and verifies read-only
without Git, target evidence, subprocesses or nonlocal reads.

Source changes require metadata refreshes of Phases 24–30. Eight exact prior
Phase 24–31 receipts are preserved under
`target/G16-06-RSV2-08/phase33/prior_*.json`. Phase 29 AST metadata is recomputed;
Phase 30 updates dependent raw receipt hashes. Regenerating Phase 31 leaves its
bytes unchanged because its historical report inputs are unchanged. Earlier
independent pins, historical reports and Phase 15–23 receipts remain unchanged.
No self-referential source digest pin is added.

```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_g16_06_rsv2_08_receipts.py --write-script-inventory --write-script-cross-check --write-summary --write-manifest-cross-check --write-cumulative-index --write-assertion-inventory --write-json-keysets --write-markdown-headings --write-hex64-digests
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts/tests -p 'test_*.py' -v
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s experiments/G16-06-RSV2-08 -p 'test_*.py' -v
git diff --check
```

Static audit only: no builds, solves, package installation, designs or
engineering assessment. Qualified count remains zero, full qualification false
and engineering qualification unassessed. No gate or threshold changes.
Baseline: `e0f3d9b`. Execution uses CPUs 12,13, nice -n 10 (inherited effective
nice 19) and CARGO_BUILD_JOBS=2. Changes remain uncommitted. COMMS.md is not
edited. New report and receipt literal host-path counts are zero.
