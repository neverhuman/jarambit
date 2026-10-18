# Phase-35 verifier self-digest audit

**PASS.** Inventoried 2 scripts, totaling 138005 bytes, 180 function definitions, 10 test classes and 88 test methods.

| Repository-relative source | Bytes | SHA-256 | Functions | Test classes | Test methods |
| --- | ---: | --- | ---: | ---: | ---: |
| scripts/audit_rsv2_07_hm_forensics.py | 63174 | 565a88f688f3e10acb531c9ef950f8a2e8bb7b8fd6b484a7049fdd93af1e7b17 | 45 | 0 | 0 |
| scripts/test_audit_rsv2_07_hm_forensics.py | 74831 | 1b24a77681d6dd7991305d2014bea91d4f3570f93bec9899e531ef1b9adf68db | 135 | 10 | 88 |

SHA-256 and byte sizes describe the exact final working-tree bytes of the verifier and its test file. The JSON lists every named synchronous and asynchronous function definition, including class methods and nested functions, with lexical dotted names, source line spans and async flags. Lambdas, comments and strings are excluded. Conditional definitions are included statically; duplicate declarations retain separate entries. Lexical names do not imply runtime resolution or reachability.

Test inventory follows the existing rule: direct test-prefixed methods in top-level classes explicitly based on unittest.TestCase or TestCase. Inherited or generated tests and import aliases are outside that count. Sources are parsed without importing or executing them. Inventory counts alone do not assert test execution or coverage.

The two sources are checked for regular file type and stable bytes before and after inventory. Missing sources, symlinks, invalid Python syntax and changes during scanning fail. Report and JSON outputs are outside the input set, so no self-digest cycle exists. Earlier reports and receipts are preserved.

Persisted paths are repository-relative. --repo-root selects a relocated source tree without Git. Normal verification checks the complete saved JSON inventory and exact report text without writes. --write replaces only the two Phase-35 outputs. Regenerate these outputs after deliberate source changes.

```sh
python3 scripts/audit_rsv2_07_hm_forensics.py --phase 35
python3 -m unittest discover -s scripts -p 'test_audit_rsv2_07_hm_forensics.py' -v
git diff --check
```

Execution uses CPUs 14,15 at nice 10. This is a static verifier self-digest audit. Changes are left uncommitted.
