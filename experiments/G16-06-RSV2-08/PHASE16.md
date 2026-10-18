# G16-06-RSV2-08 Phase 16 — Cross-phase provenance index

The index records exact SHA-256 digests for all nine Phase 7–15 reports and
their nine audit receipts. Integrity verification passes. Full qualification
remains false and qualified_count is zero in every phase. No historical report,
receipt, gate, threshold or measurement has been changed.

The assignment cannot be completed literally while preserving historical bytes:
the Phase 7 receipt contains 148 host-absolute path references and the Phase 8
receipt contains 16. All other indexed files contain zero such references.
The index itself contains zero host-absolute references and uses only actual
repository-relative paths. Strict content portability remains **blocked**;
the verifier explicitly rejects it rather than treating integrity as portability.

Phases 9–14 reside in `experiments/g16-06-rsv2-08`; Phases 7, 8 and 15 reside
in `experiments/G16-06-RSV2-08`. The index preserves these case-sensitive paths
instead of copying or rewriting historical evidence.

| Phase | Main candidates | Qualified | Full qualification |
| --- | ---: | ---: | --- |
| 7 | 144 | 0 | false |
| 8 | 12 | 0 | false |
| 9 | 36 | 0 | false |
| 10 | 36 | 0 | false |
| 11 | 36 | 0 | false |
| 12 | 36 | 0 | false |
| 13 | 24 | 0 | false |
| 14 | 16 | 0 | false |
| 15 | 3 | 0 | false |

Counts are the main result-array sizes, checked against each receipt's explicit
count where present. Ancillary controls are excluded. Phases reuse candidates;
these counts must not be summed as distinct designs. Scoped geometry, export,
A4 or GCI passes do not establish full qualification. Every indexed receipt and
every candidate result explicitly retains all_gate_qualification=false.

Phase 15's unchanged receipt binds the exact SHA-256 of
`test_candidate_receipt.py`. Extending that file would invalidate the historical
audit. Pending clarification, Phase 16 tests therefore live in
`test_phase16_provenance.py`, discovered by the assignment's exact test command.
The original test file and all historical auditors remain untouched. Tests freeze
all 18 pre-task report/receipt digests independently of the regenerable index.
They check coverage, counts, qualification, file existence and hashes, reject
forged metadata and unsafe paths, expose the two existing portability blockers,
and verify a relocated 19-file fixture without Git, target evidence or solvers.
The default verifier performs only reads and launches no subprocesses.

Validation on CPUs 12,13 with `nice -n 10`, CARGO_BUILD_JOBS=2 and bytecode writes
disabled (inherited niceness makes effective nice 19):

```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s experiments/G16-06-RSV2-08 -p 'test_*.py' -v
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 experiments/G16-06-RSV2-08/audit_phase16.py
git diff --check
```

All 45 discovered tests pass: 27 Phase 15, 10 existing portability regressions,
and eight Phase 16 tests. Index integrity passes for 18 files across nine phases.
`audit_phase16.py --require-portable` intentionally exits nonzero on the two
historical receipts; zero-path acceptance is not claimed. The requested extension
of the hash-bound test file also remains unresolved. Clarification has been
requested; history-preserving work is complete and remains uncommitted.

Baseline: `73aad5a8283316dbe30da518ccfba2c77a3c64de`. Generator usage is
`audit_phase16.py --write`; it writes only the new index. Independent frozen
digests in the tests prevent regeneration from concealing historical changes.
