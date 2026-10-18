# Phase-36 JSON schema keyset audit

**PASS for the key inventory.** Found 17 JSON receipts covering every phase from 19 through 35, with 15 distinct top-level key sets and 93 distinct keys.

Common top-level keys: `schema`, `task`.

| Phase | Repository-relative receipt | Keys | Keyset group | Bytes | SHA-256 |
| ---: | --- | ---: | ---: | ---: | --- |
| 19 | experiments/rsv2_07/results/phase19_w320_r02_provenance_audit.json | 16 | 1 | 10527 | 6e4565583f3b9eb6ee253904180a12c6267b7434a939f78839d669d97ef46edb |
| 20 | experiments/rsv2_07/results/phase20_stage2_identity_dedup_audit.json | 12 | 4 | 873563 | ed95015c3f9b753a78a1756b203f29f2e92f1482d9905aaeae99ee3a5bce4817 |
| 21 | experiments/rsv2_07/phase21_round0_seed_vs_mutated_inventory_audit.json | 14 | 5 | 25515 | bc665634397714b3af19dda65b3c36b00c1c02d81132957ad8cd3d38b2c7547d |
| 22 | experiments/rsv2_07/phase22_w320_leaderboard_provenance_audit.json | 10 | 11 | 5882 | d3ab2de4cca96bf24d780bbbc9e6a44438e3bd2bc633ba94d725a2474adb3188 |
| 23 | experiments/rsv2_07/phase23_complete_receipt_manifest_audit.json | 11 | 9 | 25697 | 2e340e3f445ceb3561f98290db165b0691bd2c2c1e86c126614ca76083fc9c59 |
| 24 | experiments/rsv2_07/phase24_screen_deficit_ordering_audit.json | 13 | 6 | 8941 | 293cc427a875ebb2f201aacc4e2e83691a9af534109fa2936b76f15590112ba9 |
| 25 | experiments/rsv2_07/phase25_births_to_screen_identity_join_audit.json | 11 | 10 | 9102 | 8a69f08721d15a562cbe277f11da40a625bc02ce17e844a1e463e3d72c031f39 |
| 26 | docs/audits/rsv2_07_hm/phase26_round1_births_vs_elites_lineage_audit.json | 13 | 12 | 2577 | 7c2636077c10d33edb6a4f8b628325cd4f0656ec0d33643eb4ce414fbcae4885 |
| 27 | docs/audits/rsv2_07_hm/phase27_artifact_digest_index.json | 11 | 2 | 1103 | 2b60bf37e70a273baf6ebbf40c6938f8fcedb52fb574fec793286e35f6f50254 |
| 28 | docs/audits/rsv2_07_hm/phase28_script_test_inventory_audit.json | 10 | 14 | 3328 | 141fb81fab478b21a61a897f3969e526f26c0779356790f01f6f6629d243c8b1 |
| 29 | docs/audits/rsv2_07_hm/phase29_index_inventory_cross_check_audit.json | 15 | 15 | 3885 | 65068b69e0994c781636ab4df2f991ee88b63f875bb06906ce81a1ee68f07f71 |
| 30 | docs/audits/rsv2_07_hm/phase30_summary_manifest_audit.json | 11 | 3 | 2512 | 844b5b616ba0f58360022127fb80801ff7d13c5594e40159645cdfad38b47c29 |
| 31 | docs/audits/rsv2_07_hm/phase31_manifest_cross_check_audit.json | 15 | 8 | 7383 | 0ea2a92e33b5584c05b6a347dc0f709292c60791578193866e03b745a20e93fa |
| 32 | docs/audits/rsv2_07_hm/phase32_cumulative_receipt_table_audit.json | 11 | 3 | 3519 | fe24474a26e04120197604daf6d26c611a3879ac5a4de3b97afe99f04166efe9 |
| 33 | docs/audits/rsv2_07_hm/phase33_table_parity_audit.json | 15 | 7 | 13091 | 299c2dfc24c06a4fcc07ac473afd531008f094f8e1fa0db8b617eac04a24d639 |
| 34 | docs/audits/rsv2_07_hm/phase34_cumulative_receipt_table_audit.json | 11 | 3 | 4519 | fc9125ab71ac6cd8bd182612c787019a68e67cb41867227eb4cbb4397854a69b |
| 35 | docs/audits/rsv2_07_hm/phase35_verifier_self_digest_audit.json | 13 | 13 | 42285 | a65978519c2c0c9efe2b2b3eabc467bfe02ff92beb911db6f316e4698581f101 |

Distinct top-level key sets:

- Group 1 (1 receipt): `additional_unpinned_attempt_files`, `artifacts`, `assessment_completion`, `attempt_directory`, `format_checks`, `genome_csv_matches_saved_birth`, `leaderboard`, `observed_outputs`, `physical_qualification`, `receipt_sync`, `schema`, `scope`, `stage1`, `status`, `task`, `wave`
- Group 2 (1 receipt): `artifact_count`, `artifacts`, `digest_basis`, `excluded_self_outputs`, `exclusion_reason`, `literal_host_path_count`, `schema`, `scope`, `status`, `task`, `total_size_bytes`
- Group 3 (3 receipts): `artifact_count`, `artifacts`, `digest_basis`, `line_count_rule`, `literal_host_path_count`, `phases`, `schema`, `status`, `task`, `total_line_count`, `total_size_bytes`
- Group 4 (1 receipt): `artifacts`, `assessed`, `deduplication`, `digest_index`, `digest_occurrences`, `schema`, `scope`, `source_genome_csv_files`, `source_json_files`, `task`, `unverified`, `wave_sources`
- Group 5 (1 receipt): `artifacts`, `committed_source`, `counterfactual_selection`, `phase20_receipt`, `qualified_count`, `reference_identities`, `reference_identity_basis`, `reference_inventory_totals`, `schema`, `scope`, `status`, `task`, `unverified_events`, `waves`
- Group 6 (1 receipt): `artifacts`, `comparison`, `limitations`, `ordering_keys`, `ordering_pass`, `qualified_count`, `schema`, `scope`, `source_mutation_policy`, `status`, `task`, `totals`, `waves`
- Group 7 (1 receipt): `artifacts`, `current_script_bytes_checked`, `excluded_historical_script_comparisons`, `input_receipts`, `literal_host_path_count`, `live_artifact_count`, `live_artifact_digest_parity`, `receipt_comparison_count`, `receipt_comparisons`, `receipt_digest_parity`, `schema`, `status`, `task`, `total_line_count`, `total_size_bytes`
- Group 8 (1 receipt): `artifacts`, `current_script_bytes_checked`, `historical_script_paths`, `input_receipts`, `literal_host_path_count`, `live_artifact_count`, `live_artifact_digest_parity`, `receipt_comparison_count`, `receipt_comparisons`, `receipt_digest_parity`, `schema`, `status`, `task`, `total_line_count`, `total_size_bytes`
- Group 9 (1 receipt): `artifacts`, `hash_dependencies`, `host_prefix_audit`, `phases`, `qualification_basis`, `qualified_count`, `schema`, `scope`, `status`, `task`, `totals`
- Group 10 (1 receipt): `artifacts`, `join_complete`, `join_keys`, `join_kind`, `qualified_count`, `schema`, `scope`, `status`, `task`, `totals`, `waves`
- Group 11 (1 receipt): `artifacts`, `leaderboard`, `markdown`, `phase21_receipt`, `phase21_w320_inventory`, `rounds`, `schema`, `scope`, `status`, `task`
- Group 12 (1 receipt): `available_logs`, `completed_scope`, `limitations`, `lineage_counts_established`, `missing_logs`, `requested_scope`, `required_evidence`, `round0_elites_total`, `round1_births_total`, `schema`, `status`, `task`, `waves`
- Group 13 (1 receipt): `digest_basis`, `function_definition_count`, `function_inventory_rule`, `limitations`, `schema`, `script_count`, `scripts`, `status`, `task`, `test_class_count`, `test_count_rule`, `test_method_count`, `total_size_bytes`
- Group 14 (1 receipt): `digest_basis`, `limitations`, `schema`, `script_count`, `scripts`, `status`, `task`, `test_count_rule`, `test_method_count`, `total_size_bytes`
- Group 15 (1 receipt): `disjoint_path_sets`, `entries`, `entry_count`, `exact_snapshot_digest_parity`, `input_receipts`, `overlap_paths`, `parity_scope`, `phase27_paths`, `phase28_paths`, `schema`, `snapshot_commit`, `status`, `task`, `working_tree_drift_paths`, `working_tree_exact_digest_parity`

Discovery recursively searches experiments/rsv2_07 and docs/audits/rsv2_07_hm for lowercase phase19 through phase35 JSON filenames, using an exact phase-number boundary. Multiple receipts for a phase are retained individually; at least one receipt per requested phase is required. Other phases and Phase-36 outputs are outside scope. Sources must be regular files and top-level JSON objects. Duplicate keys, nonfinite numbers and malformed JSON fail. Source membership, file types and bytes are checked again after scanning.

Each receipt's sorted top-level keys, key count, byte size, SHA-256 and keyset group are recorded. The JSON also lists the union, intersection and per-key receipt frequencies. Nested keys and values are outside this inventory. Identical key sets do not imply identical value types or semantics. This does not establish compliance with a formal JSON Schema or reassess earlier audit conclusions. Earlier receipts and reports are preserved.

Persisted paths are repository-relative. --repo-root selects a relocated source tree without Git. Normal verification checks the complete saved Phase-36 JSON and exact report text without writes. --write replaces only the two Phase-36 outputs.

```sh
python3 scripts/audit_rsv2_07_hm_forensics.py --phase 36
python3 -m unittest discover -s scripts -p 'test_audit_rsv2_07_hm_forensics.py' -v
git diff --check
```

Execution uses CPUs 14,15 at nice 10. This is a static JSON keyset audit. Changes are left uncommitted.
