# Phase-38 hex64 digest format audit

**FINDINGS.** Audited 19 receipts covering every phase from 19 through 37. Of 834 exact sha256 fields, 828 are lowercase 64-character hexadecimal strings and 6 are nonconforming, across 1 receipt.

| Phase | Repository-relative receipt | sha256 fields | Valid | Nonconforming |
| ---: | --- | ---: | ---: | ---: |
| 19 | experiments/rsv2_07/results/phase19_w320_r02_provenance_audit.json | 23 | 23 | 0 |
| 20 | experiments/rsv2_07/results/phase20_stage2_identity_dedup_audit.json | 521 | 521 | 0 |
| 21 | experiments/rsv2_07/phase21_round0_seed_vs_mutated_inventory_audit.json | 32 | 32 | 0 |
| 22 | experiments/rsv2_07/phase22_w320_leaderboard_provenance_audit.json | 7 | 7 | 0 |
| 23 | experiments/rsv2_07/phase23_complete_receipt_manifest_audit.json | 35 | 35 | 0 |
| 24 | experiments/rsv2_07/phase24_screen_deficit_ordering_audit.json | 30 | 30 | 0 |
| 25 | experiments/rsv2_07/phase25_births_to_screen_identity_join_audit.json | 30 | 30 | 0 |
| 26 | docs/audits/rsv2_07_hm/phase26_round1_births_vs_elites_lineage_audit.json | 6 | 0 | 6 |
| 27 | docs/audits/rsv2_07_hm/phase27_artifact_digest_index.json | 2 | 2 | 0 |
| 28 | docs/audits/rsv2_07_hm/phase28_script_test_inventory_audit.json | 2 | 2 | 0 |
| 29 | docs/audits/rsv2_07_hm/phase29_index_inventory_cross_check_audit.json | 2 | 2 | 0 |
| 30 | docs/audits/rsv2_07_hm/phase30_summary_manifest_audit.json | 8 | 8 | 0 |
| 31 | docs/audits/rsv2_07_hm/phase31_manifest_cross_check_audit.json | 26 | 26 | 0 |
| 32 | docs/audits/rsv2_07_hm/phase32_cumulative_receipt_table_audit.json | 12 | 12 | 0 |
| 33 | docs/audits/rsv2_07_hm/phase33_table_parity_audit.json | 45 | 45 | 0 |
| 34 | docs/audits/rsv2_07_hm/phase34_cumulative_receipt_table_audit.json | 16 | 16 | 0 |
| 35 | docs/audits/rsv2_07_hm/phase35_verifier_self_digest_audit.json | 2 | 2 | 0 |
| 36 | docs/audits/rsv2_07_hm/phase36_json_schema_keyset_audit.json | 17 | 17 | 0 |
| 37 | docs/audits/rsv2_07_hm/phase37_markdown_heading_consistency_audit.json | 18 | 18 | 0 |

Nonconforming fields:

- docs/audits/rsv2_07_hm/phase26_round1_births_vs_elites_lineage_audit.json: `["waves", "w315", "sha256"]` — null_not_digest (null).
- docs/audits/rsv2_07_hm/phase26_round1_births_vs_elites_lineage_audit.json: `["waves", "w316", "sha256"]` — null_not_digest (null).
- docs/audits/rsv2_07_hm/phase26_round1_births_vs_elites_lineage_audit.json: `["waves", "w317", "sha256"]` — null_not_digest (null).
- docs/audits/rsv2_07_hm/phase26_round1_births_vs_elites_lineage_audit.json: `["waves", "w318", "sha256"]` — null_not_digest (null).
- docs/audits/rsv2_07_hm/phase26_round1_births_vs_elites_lineage_audit.json: `["waves", "w319", "sha256"]` — null_not_digest (null).
- docs/audits/rsv2_07_hm/phase26_round1_births_vs_elites_lineage_audit.json: `["waves", "w320", "sha256"]` — null_not_digest (null).

Every object property named exactly sha256 is visited recursively through objects and arrays, including a nested sha256 property inside a non-string value. Values must be strings fully matching the ASCII pattern [0-9a-f]{64}; null, numbers, booleans, objects, arrays, uppercase characters, whitespace and wrong lengths are nonconforming. No normalization or substitution is performed. Properties with other names, including genome_sha256, recorded_sha256, snapshot_sha256 and working_tree_sha256, are outside this exact-key check; older receipts also use suffixed names for counters and indexes.

The JSON lists every checked field's path components, value type, string length, validity and reason. Object keys are strings; array indexes are integers, so paths remain unambiguous when keys contain slashes or tildes. Each source receipt is pinned by SHA-256 and byte size. Receipts with no exact sha256 fields are explicitly counted as zero fields. This checks representation only and does not establish digest correctness or availability of missing artifacts. Existing null placeholders are preserved and reported as findings.

Discovery recursively searches experiments/rsv2_07 and docs/audits/rsv2_07_hm for lowercase phase19 through phase37 JSON filenames with an exact phase-number boundary. Complete phase coverage is required; multiple receipts per phase are retained. Phase-38 outputs are excluded. Sources must be regular files and strict JSON objects; duplicate keys, nonfinite numbers and malformed JSON fail. Membership, types and source bytes are checked again after scanning.

Paths in the outputs are repository-relative. --repo-root supports a relocated source tree without Git. Normal verification compares the complete saved JSON and exact report text without writes. --write replaces only the two Phase-38 outputs. Earlier receipts are preserved. Exit 0 means every checked field conforms, exit 2 means format findings were recorded, and malformed or mismatching evidence exits 1.

```sh
python3 scripts/audit_rsv2_07_hm_forensics.py --phase 38
python3 -m unittest discover -s scripts -p 'test_audit_rsv2_07_hm_forensics.py' -v
git diff --check
```

Execution uses CPUs 14,15 at nice 10. This is a static digest format audit. Changes are left uncommitted.
