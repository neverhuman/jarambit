# G16-06-RSV2-08 Phase 34 — Cumulative Markdown and JSON summary

Task G16-06-RSV2-08-p35 indexes all 19 PHASE15..PHASE33 Markdown/JSON pairs,
38 files total. The output is `phase34_cumulative_summary_audit.json`. Each
record stores phase, kind, repository-relative path, byte size, UTF-8 line count
and raw SHA-256. Totals cover the complete set, each phase and each file kind.
There are 38 file-digest records. The aggregate `file_index_sha256` hashes the
UTF-8 canonical JSON of the ordered file-record list, with sorted object keys,
compact separators and ASCII escaping; it is not an arithmetic sum of hashes.

The fixed list extends the prior Phase 15–32 receipt list with Phase 33 and
pairs every receipt with its numbered report. Phases 15–20 use the experiment
directory and Phases 21–33 the audit directory. Line counts use `splitlines()`:
empty files have zero lines and an unterminated final line still counts. JSON
files undergo strict parsing, including the existing nonfinite-number checks.

Default verification retains the complete earlier portable receipt chain,
then recomputes the summary from live file bytes. It needs no Git, environment
settings, target evidence, writes or subprocesses. The relocated fixture now
includes all pairs and this summary. Six regressions check exact pair coverage,
metrics, phase/kind/global totals, the aggregate digest, forged or omitted
records, missing/changed reports, invalid JSON and static scope.

The summary explicitly retains Phase 33's saved failed path verdict and its
21 failures. Summary-integrity PASS does not establish that referenced paths
are tracked or present. The separate live `--verify-canonical-paths` command
still reports FAIL and exits 1. No historical failure is suppressed, repaired
or reclassified by this indexing task.

Source updates require metadata refreshes of Phases 24–30 and 32–33. Ten exact
prior Phase 24–33 receipts are saved under
`target/G16-06-RSV2-08/phase35/prior_*.json`. Phase 31 receipt stays byte-identical.
Phase 29 AST metadata, Phase 32 digest values and Phase 33 input receipt hashes
follow the source changes. Historical reports remain unchanged from task start,
including the prior task's final PHASE33 validation footer. Phase 15–23 receipts
and independent pins remain unchanged. No self-referential source pin is added.

```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_g16_06_rsv2_08_receipts.py --write-script-inventory --write-script-cross-check --write-summary --write-manifest-cross-check --write-cumulative-index --write-assertion-inventory --write-json-keysets --write-markdown-headings --write-hex64-digests --write-canonical-paths --write-cumulative-summary
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_g16_06_rsv2_08_receipts.py
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_g16_06_rsv2_08_receipts.py --verify-canonical-paths
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts/tests -p 'test_*.py' -v
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s experiments/G16-06-RSV2-08 -p 'test_*.py' -v
git diff --check
```

Generation includes the live path audit and exits 1 for its recorded failures.
Default summary verification passes when the saved inventory matches live bytes;
the explicit path verification remains nonzero. Static only: no builds, solves,
package installation, designs or engineering assessment. Qualified count zero,
full qualification false and engineering qualification unassessed. No gate or
threshold changes. Baseline `2085d27`; CPUs 12,13, nice -n 10 (effective nice 19),
CARGO_BUILD_JOBS=2. This agent does not commit or push. COMMS.md is unchanged.
New summary/report literal host-path counts are zero.
