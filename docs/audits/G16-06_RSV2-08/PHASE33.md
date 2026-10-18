# G16-06-RSV2-08 Phase 33 — Canonical referenced path existence

Task G16-06-RSV2-08-p34 inventories file references in all 18 Phase 15–32 JSON
receipts. The output is `phase33_canonical_path_existence_audit.json`.
The universal requirement fails: 20 referenced files exist only under untracked
`target/` directories, and `experiments/G16-06-RSV2-08/PHASE1.md` is missing
and untracked. These 21 failures are retained without exclusions or aliases.
No historical evidence is created, tracked or rewritten to manufacture a pass.

The audit recursively inspects dictionary keys and string values. File-looking
literal paths use ASCII letters, digits, underscores, hyphens, dots and slashes,
with an alphabetic-leading extension. Slash-free file names use the recognized
extensions md/json/py/rs/csv/toml/lock/step/sh/txt/stl/obj/yaml/yml; this includes
`Cargo.lock` while excluding Python qualified function names and version strings.
JSON Pointers, absolute legacy host paths, prose, glob expressions and paths
embedded in prose are outside this repository-relative literal-file scope.
The inspected corpus has no additional relative directory or space-bearing file
references. Each occurrence records an escaped JSON Pointer and key/value kind.

Every selected literal must already be canonical: no absolute path, backslash,
parent traversal, redundant separator or dot component. Resolution must remain
inside the repository and produce a regular file. Membership is checked against
live `git ls-files --cached -z`. Existing untracked files fail; missing tracked
files, directories and escaping symlinks also fail. Unique path records include
each predicate and occurrence count; source receipts include raw hashes/bytes.
The report distinguishes audit-record integrity from the failed path predicate.

The new explicit `--verify-canonical-paths` command runs the unchanged portable
verification chain plus the live Git-backed audit, prints status fail and exits
1 for findings. `--write-canonical-paths` writes the truthful failing receipt
and also exits 1. The default portable verifier keeps its prior no-Git/no-target
contract. The required path check is run explicitly below; no optional-target
exception applies to its universal predicate. Six new tests cover live Git
membership/counts, recursive extraction, positive tracked-file cases, missing
or untracked files, directories/traversal/symlinks, forged records, changed files
and nonzero CLI status. The prior relocated portable fixture remains unchanged.

Updating source requires metadata refreshes of Phases 24–30 and 32. Nine prior
Phase 24–32 receipts are preserved under
`target/G16-06-RSV2-08/phase34/prior_*.json`. Phase 31 receipt bytes remain
unchanged. Historical report bytes are unchanged from task start, including the
previous task's uncommitted PHASE32 validation footer. Earlier independent pins
and Phase 15–23 receipts remain active and unchanged; no circular pin is added.

```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_g16_06_rsv2_08_receipts.py --write-script-inventory --write-script-cross-check --write-summary --write-manifest-cross-check --write-cumulative-index --write-assertion-inventory --write-json-keysets --write-markdown-headings --write-hex64-digests --write-canonical-paths
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_g16_06_rsv2_08_receipts.py --verify-canonical-paths
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts/tests -p 'test_*.py' -v
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s experiments/G16-06-RSV2-08 -p 'test_*.py' -v
git diff --check
```

Static audit only: no builds, solves, package installations, designs or engineering
assessment. Engineering qualification unassessed, qualified count zero, full
qualification false. Gates and thresholds are unchanged. Baseline `cd28df0`.
Execution: CPUs 12,13, nice -n 10 (inherited effective nice 19), CARGO_BUILD_JOBS=2.
This agent does not commit or push. COMMS.md is not edited. New report and receipt
literal host-path counts are zero.
