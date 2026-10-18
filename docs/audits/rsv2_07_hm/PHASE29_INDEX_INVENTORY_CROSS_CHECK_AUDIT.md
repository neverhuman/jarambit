# Phase-29 index and inventory cross-check audit

**PASS for the pinned snapshot.** Phase 27 lists 2 paths and Phase 28 lists 2 paths; their intersection is empty. All 4 recorded sizes and SHA-256 digests exactly match the pinned Git snapshot.

Snapshot commit: `cc1918287761ba7a16683b3b7bbfc8c06cce50b1`. Both input receipt files must also match this snapshot byte for byte. Registry schema, status, counts, byte totals, digest formats and relative path scope are checked. Duplicate keys, nonfinite JSON and overlapping path sets fail.

**Current working-tree parity: DRIFT.** 2 paths differ from the saved registry metadata. Extending the verifier and its tests for Phase 29 changes the script bytes recorded by Phase 28. Snapshot parity does not assert current script parity. The existing Phase-27 and Phase-28 reports and receipts are preserved.

| Registered relative path | Recorded bytes | Recorded SHA-256 | Snapshot parity | Working-tree parity |
| --- | ---: | --- | --- | --- |
| docs/audits/rsv2_07_hm/PHASE26_ROUND1_BIRTHS_VS_ELITES_LINEAGE_AUDIT.md | 2937 | faa15cb6421fece10670268ccb644846f66d3f153aa943b21be711fbdf8683f9 | yes | yes |
| docs/audits/rsv2_07_hm/phase26_round1_births_vs_elites_lineage_audit.json | 2577 | 7c2636077c10d33edb6a4f8b628325cd4f0656ec0d33643eb4ce414fbcae4885 | yes | yes |
| scripts/audit_rsv2_07_hm_forensics.py | 20304 | 6a1127c8db385b67f676f5dec9603c201a57dd4b76258c7c9213f98f48eef2ce | yes | no |
| scripts/test_audit_rsv2_07_hm_forensics.py | 22714 | d3323440090a6b7bfa5ba7f55c1814293eb2793efc8096ed43f41a54e975b58d | yes | no |

The JSON records both observed snapshot and current working-tree sizes and hashes. Inputs are reread to detect changes during the audit. This checks the saved registries; it does not rebuild Phase 27's historical file selection, rerun Phase 28's test count, execute source files or infer anything about earlier audit conclusions. Missing current entry files are recorded with null size and digest and fail working-tree parity; missing snapshot evidence fails the audit.

Paths in the outputs are repository-relative. --repo-root selects a relocated Git checkout containing the pinned commit. Normal verification compares the complete saved JSON and exact report text without writes. --write replaces only the two Phase-29 outputs. Exit 0 means disjoint path sets and exact snapshot digest parity; current working-tree drift remains explicit in the output.

```sh
python3 scripts/audit_rsv2_07_hm_forensics.py --phase 29
python3 -m unittest discover -s scripts -p 'test_audit_rsv2_07_hm_forensics.py' -v
git diff --check
```

Execution uses CPUs 14,15 at nice 10. This is a static receipt cross-check. Changes are left uncommitted.
