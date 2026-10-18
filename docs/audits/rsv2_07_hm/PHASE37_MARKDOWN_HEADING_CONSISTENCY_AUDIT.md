# Phase-37 Markdown heading consistency audit

**PASS.** Audited 18 reports covering every phase from 19 through 36: 18 headings, 0 structural findings in 0 reports.

| Phase | Repository-relative report | Headings | H1 count | Title phase matches | Findings |
| ---: | --- | ---: | ---: | --- | ---: |
| 19 | experiments/rsv2_07/PHASE19_W320_R02_PROVENANCE_AUDIT.md | 1 | 1 | yes | 0 |
| 20 | experiments/rsv2_07/PHASE20_STAGE2_IDENTITY_DEDUP_AUDIT.md | 1 | 1 | yes | 0 |
| 21 | experiments/rsv2_07/PHASE21_ROUND0_SEED_VS_MUTATED_INVENTORY_AUDIT.md | 1 | 1 | yes | 0 |
| 22 | experiments/rsv2_07/PHASE22_W320_LEADERBOARD_PROVENANCE_AUDIT.md | 1 | 1 | yes | 0 |
| 23 | experiments/rsv2_07/PHASE23_COMPLETE_RECEIPT_MANIFEST_AUDIT.md | 1 | 1 | yes | 0 |
| 24 | experiments/rsv2_07/PHASE24_SCREEN_DEFICIT_ORDERING_AUDIT.md | 1 | 1 | yes | 0 |
| 25 | experiments/rsv2_07/PHASE25_BIRTHS_TO_SCREEN_IDENTITY_JOIN_AUDIT.md | 1 | 1 | yes | 0 |
| 26 | docs/audits/rsv2_07_hm/PHASE26_ROUND1_BIRTHS_VS_ELITES_LINEAGE_AUDIT.md | 1 | 1 | yes | 0 |
| 27 | docs/audits/rsv2_07_hm/PHASE27_ARTIFACT_DIGEST_INDEX.md | 1 | 1 | yes | 0 |
| 28 | docs/audits/rsv2_07_hm/PHASE28_SCRIPT_TEST_INVENTORY_AUDIT.md | 1 | 1 | yes | 0 |
| 29 | docs/audits/rsv2_07_hm/PHASE29_INDEX_INVENTORY_CROSS_CHECK_AUDIT.md | 1 | 1 | yes | 0 |
| 30 | docs/audits/rsv2_07_hm/PHASE30_SUMMARY_MANIFEST_AUDIT.md | 1 | 1 | yes | 0 |
| 31 | docs/audits/rsv2_07_hm/PHASE31_MANIFEST_CROSS_CHECK_AUDIT.md | 1 | 1 | yes | 0 |
| 32 | docs/audits/rsv2_07_hm/PHASE32_CUMULATIVE_RECEIPT_TABLE_AUDIT.md | 1 | 1 | yes | 0 |
| 33 | docs/audits/rsv2_07_hm/PHASE33_TABLE_PARITY_AUDIT.md | 1 | 1 | yes | 0 |
| 34 | docs/audits/rsv2_07_hm/PHASE34_CUMULATIVE_RECEIPT_TABLE_AUDIT.md | 1 | 1 | yes | 0 |
| 35 | docs/audits/rsv2_07_hm/PHASE35_VERIFIER_SELF_DIGEST_AUDIT.md | 1 | 1 | yes | 0 |
| 36 | docs/audits/rsv2_07_hm/PHASE36_JSON_SCHEMA_KEYSET_AUDIT.md | 1 | 1 | yes | 0 |

Checks require exactly one H1, an initial H1, its Phase-number prefix matching the filename, nonempty titles, no skipped heading levels and no repeated titles after case and whitespace normalization. Unclosed code fences are recorded. The JSON lists source SHA-256 and byte sizes, heading line numbers, levels, titles, syntax styles, level counts and individual findings. Findings are recorded without changing source documents.

Parser scope is standalone ATX headings and single-line Setext headings with at most three leading spaces. Optional closing ATX hashes are removed. Backtick and tilde fences are excluded until a matching fence of equal or greater length closes; four-space or tab-indented lines, blockquote prefixes and list-marker lines are not headings. Container blocks, HTML blocks and multiline Setext paragraphs are outside this bounded parser; this is not a full CommonMark conformance check. The saved reports use standalone ATX headings and simple fenced command blocks.

Discovery recursively searches experiments/rsv2_07 and docs/audits/rsv2_07_hm for uppercase PHASE19 through PHASE36 Markdown filenames with an exact phase-number boundary. Multiple reports per phase are retained; complete phase coverage is required. Phase-37 outputs and other phases are excluded. Sources must be regular UTF-8 files. Membership, types and source bytes are checked again after scanning. Earlier audit conclusions and receipts are preserved.

Paths in these outputs are repository-relative. --repo-root supports a relocated source tree without Git. Normal verification compares the complete saved JSON and exact report text without writes. --write replaces only the two Phase-37 outputs. Exit 0 means consistent headings, exit 2 means recorded structural findings, and malformed or mismatching evidence exits 1.

```sh
python3 scripts/audit_rsv2_07_hm_forensics.py --phase 37
python3 -m unittest discover -s scripts -p 'test_audit_rsv2_07_hm_forensics.py' -v
git diff --check
```

Execution uses CPUs 14,15 at nice 10. This is a static Markdown heading audit. Changes are left uncommitted.
