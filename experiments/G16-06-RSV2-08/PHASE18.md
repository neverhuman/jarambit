# G16-06-RSV2-08 Phase 18 — Complete study manifest

Task G16-06-RSV2-08-p19 inventories the historical Phase 1–17 study in
`phase18_complete_study_manifest.json`. It records 33 existing files: 16 Markdown
reports and 17 corresponding JSON receipts/indexes, each with exact SHA-256,
byte size and actual repository-relative path. Every phase and the manifest
retain `qualified_count: 0` and `full_qualification: false`.

The requested `PHASE1.md` does not exist in this checkout. Its entry has a null
report, the expected relative path and an explicit missing-report finding.
Phase 1's `candidate_receipt.json` is indexed. The manifest therefore reports
`all_requested_reports_present: false`; strict completeness verification rejects
the missing report. No historical report is invented or copied from another
study. All available evidence is indexed, while literal seventeen-report
completeness remains unresolved.

Phases 9–14 retain their actual `experiments/g16-06-rsv2-08` paths. All other
phases use `experiments/G16-06-RSV2-08`. Phase 15 uses its existing joint-audit
receipt, Phase 16 its cross-phase provenance index, and Phase 17 its legacy-path
inventory. Configuration, candidate inputs and raw measurement JSONs are outside
this manifest's scope of phase reports and corresponding audit receipts.

Historical Phase 1–17 files remain byte-for-byte unchanged. The manifest uses
relative paths without embedding historical content. Older receipts may contain
legacy host paths; their bytes and hashes are preserved. The three new Phase 18
files contain zero literal legacy host prefixes. This does not change the
historical Phase 16 strict content-portability rejection.

`test_phase18_manifest.py` includes a read-only verifier and eight regressions.
An independent digest pin protects the entire manifest, including historical
sizes and hashes. Verification checks exact phase coverage, canonical file
identities, path safety, bytes, SHA-256, receipt qualification and completeness
claims. Tests reject false qualification, changed sizes/hashes, unsafe paths,
missing/corrupt required evidence and false claims that every report exists.
A relocated 34-file fixture verifies without Git metadata, target evidence,
environment settings, subprocesses or reads outside its root.

Validation uses CPUs 12,13, `nice -n 10` (inherited effective nice 19),
CARGO_BUILD_JOBS=2 and bytecode writes disabled:

```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s experiments/G16-06-RSV2-08 -p 'test_*.py' -v
git diff --check
```

Baseline: `b0f5f5a`. This is a saved-evidence manifest; no numerical solves,
production source changes, gates, thresholds or qualification claims change.
The historical receipt qualification fields remain false, including the Phase 1
prototype's limited task-level passes. Changes remain uncommitted for captain
review.

Validation result: all 61 discovered tests pass (53 prior, eight new);
`git diff --check` and new-file whitespace checks pass. All 33 indexed files
match committed baseline bytes. The only completeness limitation is the absent
Phase 1 Markdown report; the manifest never claims it exists.
