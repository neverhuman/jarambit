# Phase-28 script and test inventory audit

**PASS.** Inventoried 2 scripts, totaling 43018 bytes and 30 statically declared test methods.

| Repository-relative script | Bytes | SHA-256 | Test methods |
| --- | ---: | --- | ---: |
| scripts/audit_rsv2_07_hm_forensics.py | 20304 | 6a1127c8db385b67f676f5dec9603c201a57dd4b76258c7c9213f98f48eef2ce | 0 |
| scripts/test_audit_rsv2_07_hm_forensics.py | 22714 | d3323440090a6b7bfa5ba7f55c1814293eb2793efc8096ed43f41a54e975b58d | 30 |

Sizes and SHA-256 describe exact working-tree bytes after the Phase-28 implementation and tests were written. Both sources are reread to detect changes during the audit. Missing files, nonregular files and invalid Python syntax fail. The output files are outside the input set, so no self-digest cycle exists.

The AST parser counts directly declared synchronous and asynchronous methods whose names start with test, inside top-level classes explicitly based on unittest.TestCase or TestCase. Helpers, top-level functions, nested methods, strings and comments are excluded. Test class and method names are recorded in the JSON receipt. Sources are parsed without importing or executing them. Inherited methods, import aliases, dynamically generated tests and runtime discovery are outside this static count; these counts do not claim test execution or coverage.

Persisted paths are repository-relative. --repo-root selects a relocated source tree without requiring Git. A normal Phase-28 run verifies the complete saved JSON inventory and exact report text without writes. --write replaces only the two Phase-28 output files.

```sh
python3 scripts/audit_rsv2_07_hm_forensics.py --phase 28
python3 -m unittest discover -s scripts -p 'test_audit_rsv2_07_hm_forensics.py' -v
git diff --check
```

Execution uses CPUs 14,15 at nice 10. This is a static script and test inventory audit. Changes are left uncommitted.
