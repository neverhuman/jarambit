# Phase-30 summary manifest audit

**PASS for the artifact inventory.** Indexed 8 Phase-26 through Phase-29 Markdown and JSON files, totaling 20265 bytes and 349 lines. Literal host paths: 0.

| Repository-relative artifact | Bytes | Lines | SHA-256 | Literal host paths |
| --- | ---: | ---: | --- | ---: |
| docs/audits/rsv2_07_hm/PHASE26_ROUND1_BIRTHS_VS_ELITES_LINEAGE_AUDIT.md | 2937 | 28 | faa15cb6421fece10670268ccb644846f66d3f153aa943b21be711fbdf8683f9 | 0 |
| docs/audits/rsv2_07_hm/PHASE27_ARTIFACT_DIGEST_INDEX.md | 1832 | 22 | bc39415bb1709a746eb0e2366ec34bd0d0cb32e712c07063f0e7057d38aa19fe | 0 |
| docs/audits/rsv2_07_hm/PHASE28_SCRIPT_TEST_INVENTORY_AUDIT.md | 1895 | 22 | ee894ddd7ef378e5d0da6bba2c34d12e4ea3ecbf05d936ab443290904f1b12cb | 0 |
| docs/audits/rsv2_07_hm/PHASE29_INDEX_INVENTORY_CROSS_CHECK_AUDIT.md | 2708 | 26 | cb649896fad00154ed39594c54d52836016f9395395704c35e08fbecc16aaa2b | 0 |
| docs/audits/rsv2_07_hm/phase26_round1_births_vs_elites_lineage_audit.json | 2577 | 80 | 7c2636077c10d33edb6a4f8b628325cd4f0656ec0d33643eb4ce414fbcae4885 | 0 |
| docs/audits/rsv2_07_hm/phase27_artifact_digest_index.json | 1103 | 27 | 2b60bf37e70a273baf6ebbf40c6938f8fcedb52fb574fec793286e35f6f50254 | 0 |
| docs/audits/rsv2_07_hm/phase28_script_test_inventory_audit.json | 3328 | 62 | 141fb81fab478b21a61a897f3969e526f26c0779356790f01f6f6629d243c8b1 | 0 |
| docs/audits/rsv2_07_hm/phase29_index_inventory_cross_check_audit.json | 3885 | 82 | 65068b69e0994c781636ab4df2f991ee88b63f875bb06906ce81a1ee68f07f71 | 0 |

The input set is the fixed eight named report and receipt files from Phases 26 through 29. Hashes and sizes describe exact working-tree bytes. Physical lines are counted by LF delimiters, plus an unterminated final line; an empty file has zero lines. CRLF is one line ending, and Unicode line separators do not add physical lines. Phase-30 outputs, source scripts and other files are outside the input set.

All inputs must be regular UTF-8 files. JSON is parsed strictly, rejecting duplicate keys and nonfinite numbers. Literal absolute host paths are rejected in raw text and decoded JSON strings. Source bytes and file types are checked again after scanning. These checks describe saved artifacts and do not rerun earlier audits or establish their conclusions; Phase 26's partial lineage result and Phase 29's snapshot-only parity remain as recorded.

Persisted paths are repository-relative. --repo-root selects a relocated source tree without requiring Git. Normal verification checks the complete saved JSON manifest and exact report text without writes; --write replaces only the two Phase-30 outputs. The earlier reports and receipts are preserved.

```sh
python3 scripts/audit_rsv2_07_hm_forensics.py --phase 30
python3 -m unittest discover -s scripts -p 'test_audit_rsv2_07_hm_forensics.py' -v
git diff --check
```

Execution uses CPUs 14,15 at nice 10. This is a static summary manifest audit. Changes are left uncommitted.
