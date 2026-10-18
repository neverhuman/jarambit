# G16-06-RSV2-08 Phase 17 — Legacy receipt path inventory

The inventory covers every legacy host string at its exact typed JSON keypath:
148 in Phase 7 and 16 in Phase 8. Phase 9–16 receipts/index contain zero.
All historical Phase 7–16 files and `test_candidate_receipt.py` remain unchanged.
The inventory, this report and the new test module contain zero literal legacy
host prefixes. `qualified_count = 0`; `full_qualification = false`.

The assignment's names `phase7_joint_4gate_audit.json` and
`phase8_focused_spine_angle_audit.json` do not exist in this checkout. The actual
receipts are `phase7_receipt.json` and `phase8_receipt.json` in this directory.
Phases 9–14 use the existing lowercase study directory; Phase 15 uses
`phase15_gate_s_compliant_joint_audit.json`, and Phase 16 uses
`phase16_cross_phase_provenance_index.json`. No aliases or rewritten receipts
are introduced.

| Classification | Phase 7 | Phase 8 | Total | Portable handling |
| --- | ---: | ---: | ---: | --- |
| Tracked genome file | 144 | 14 | 158 | Root-relative `row.genome`, exact SHA-256 |
| Optional raw control genome | 3 | 0 | 3 | Root-relative target path, exact SHA-256 when present |
| External delivery directory | 1 | 1 | 2 | Historical destination, outside repository evidence |
| Historical Git failure text | 0 | 1 | 1 | Diagnostic text, not a scientific file reference |

Each scientific entry includes the source keypath, SHA-256 of the original
string, canonical repository-relative file path, portable binding keypath and
expected file digest. The original host strings are read from the unchanged
receipts; they are never copied into new outputs. All object keys and nested
values are traversed. No legacy host string is silently omitted.

The three non-evidence strings cannot truthfully map to canonical repository
files. Their canonical paths are explicitly null and their classifications
explain why. In particular, an external archive destination is not an equivalent
local evidence file, and a Git administrative lock mentioned in an error is not
an audit input. Thus the assignment's literal demand for a repository file for
every legacy string has these three documented exceptions.

`audit_phase17.py` verifies all 161 scientific mappings using the unchanged
portable path resolver and receipt genome bindings. Required tracked genomes
must exist and match their digests. Raw control genomes remain optional, as in
the historical audits; missing raw evidence is never described as replayed or
qualified evidence. A present mismatching file fails verification.

The regression fixture copies the required tracked files and available mapped
raw genomes into a temporary repository under the lane's existing target tree.
It has no Git metadata. Guards reject all verifier file reads and stat operations
outside that relocated root, reject writes, and reject subprocess launches.
Unchanged `audit_phase7.py`, `audit_phase8.py` and `audit_phase16.py` pass from
that root; Phase 17's inventory also verifies there. Phase 7/8 pass again when
all target evidence is made unavailable. This verifies portable input resolution
without using any original host path. No FEM or solid solve is rerun.

The Phase 16 integrity result remains separate from content portability:
`require_portable=True` still rejects the unchanged legacy receipts. This audit
does not remove that check or claim that historical JSON bytes have become
portable. No gate, threshold, production source, candidate or qualification
claim changes.

Validation uses CPUs 12,13 with `nice -n 10` (effective nice 19 inherited),
CARGO_BUILD_JOBS=2 and bytecode writes disabled:

```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 experiments/G16-06-RSV2-08/audit_phase17.py
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s experiments/G16-06-RSV2-08 -p 'test_*.py' -v
git diff --check
```

Baseline: `1dde5ce`. The inventory additionally freezes 26 historical file hashes.
Changes remain uncommitted for captain review. No push or PR is requested.

Validation result: all 52 discovered tests pass, including seven new Phase 17
regressions. Inventory verification passes; all four new files pass whitespace
checks. Independent inventory digest pinning prevents regeneration from hiding
changes to the 26 frozen historical files.
