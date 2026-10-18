# G16-06-RSV2-08 Phase 26 — Static Phase 21–25 summary manifest

Task G16-06-RSV2-08-p27 indexes exactly ten files: the five Markdown reports
`PHASE21.md` through `PHASE25.md` and their five JSON receipts, all under
`docs/audits/G16-06_RSV2-08`. The manifest
`phase26_summary_manifest_audit.json` records each file's phase, kind,
repository-relative path, exact SHA-256, byte size, UTF-8 line count and zero
literal host-path count. The new Phase 26 outputs are excluded from their own
input set.

Line count uses decoded UTF-8 `splitlines()` for both Markdown and JSON, with
the same empty-file, final-line and CRLF semantics as the earlier report index.
Raw-byte hashes and sizes preserve exact formatting. All five JSON receipts
also pass the unchanged strict parser, including its nonfinite-number rejection.
Host-path detection retains the existing POSIX home, root, temporary and Windows
user/temporary directory rules. All ten inputs must contain zero matches.

The verifier checks the complete current Phase 25 parent chain before generating
or verifying the summary. That retains the earlier independent receipt pins,
coverage, finite JSON values, report metrics, script AST counts and disjoint
member/digest-parity checks. Default verification now includes this summary.
Its coverage is fixed to five report/receipt pairs: forged paths in a summary
are never used to choose files to read.

Authorized script and test updates require a metadata refresh of Phase 24's
script sizes, hashes and AST counts, then Phase 25's dependent source/member
hashes and sizes. Those two prior receipts are preserved under
`target/G16-06-RSV2-08/phase27/prior_*.json`. Only these source-dependent fields
and their totals are refreshed. All five historical Markdown reports and the
Phase 21–23 JSON receipts remain unchanged. Earlier reports retain their original
run metrics; this manifest hashes their exact historical bytes.

The new summary is recomputed from current exact inputs. Its digest is not
embedded in the scripts whose hashes feed the summary, avoiding a circular hash
dependency. This is current-byte parity, not immutable or signed attestation.
All earlier independent digest pins remain active.

Six new regressions cover complete phase/file coverage, exact digest/size/line
metrics, forged metadata and paths, omitted or missing reports, changed bytes,
injected host paths and static output scope. The relocated combined fixture now
contains 42 unique files and verifies every stage without Git, target evidence,
environment settings, writes, subprocesses or nonlocal reads.

```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_g16_06_rsv2_08_receipts.py --write-script-inventory --write-script-cross-check --write-summary
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts/tests -p 'test_*.py' -v
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s experiments/G16-06-RSV2-08 -p 'test_*.py' -v
git diff --check
```

The write options refresh dependencies in order before writing the summary;
default verification only reads saved files. This is a static summary audit
only: no builds, solves, package installation, design generation or engineering
assessment. Qualified count remains zero, full qualification false and engineering
qualification unassessed. No gate or threshold changes. The new report and
manifest also contain zero literal host paths. Baseline: `5547196`. Execution
uses CPUs 12,13 with `nice -n 10` (inherited effective nice 19) and
CARGO_BUILD_JOBS=2. Leave changes uncommitted.

Validation result: combined verifier PASS; 41/41 verifier tests and 79/79
historical tests pass. The ten indexed files total 53,632 bytes and 1,167 lines,
all with zero literal host paths. Phase 24 and 25 changes are structurally proven
to contain only dependent metadata refreshes; prior receipt bytes match the
committed baseline. All five prior reports and Phase 21–23 receipts are unchanged.
A detector match in slash-separated new report prose was corrected without
changing the detector. Whitespace checks pass; logs remain under
`target/G16-06-RSV2-08/phase27/`.
