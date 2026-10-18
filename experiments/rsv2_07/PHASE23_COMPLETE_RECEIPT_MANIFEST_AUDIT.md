# Phase-23 complete current receipt manifest audit

**PASS for current-file inventory and host-prefix hygiene, with coverage gaps recorded.** The catalogue binds 35 saved files: 20 Markdown reports and 15 JSON receipts/manifests. Every entry has exact SHA-256 and byte size in the JSON. 3 supporting files have no phase number. Missing historical deliverables are not fabricated.

| Phase | Reports | JSON receipts | Current coverage |
| --- | ---: | ---: | --- |
| 1 | 1 | 1 | report_and_json |
| 2 | 0 | 0 | no_current_dedicated_files |
| 3 | 0 | 0 | no_current_dedicated_files |
| 4 | 1 | 1 | report_and_json |
| 5 | 1 | 1 | report_and_json |
| 6 | 1 | 1 | report_and_json |
| 7 | 1 | 1 | report_and_json |
| 8 | 1 | 1 | report_and_json |
| 9 | 0 | 0 | no_current_dedicated_files |
| 10 | 1 | 1 | report_and_json |
| 11 | 1 | 0 | report_only |
| 12 | 1 | 0 | report_only |
| 13 | 1 | 0 | report_only |
| 14 | 1 | 0 | report_only |
| 15 | 1 | 0 | report_only |
| 16 | 1 | 0 | report_only |
| 17 | 1 | 1 | report_and_json |
| 18 | 1 | 1 | report_and_json |
| 19 | 1 | 1 | report_and_json |
| 20 | 1 | 1 | report_and_json |
| 21 | 1 | 1 | report_and_json |
| 22 | 1 | 1 | report_and_json |

Legacy aliases assign GCI_A4_VERIFICATION.md and results/gci_a4_validation.json to Phase-1, and SOLID_TIP_3D_VERIFICATION.md and results/solid_tip_3d_validation.json to Phase-4. These are catalogue conventions; no distinct Phase-2/3/9 files are invented from shared or overwritten deliverables. The original REPORT.md, validation.json and cad_manifest.json remain unassigned supporting artifacts. A report-only phase does not acquire an invented JSON receipt.

The manifest's qualified_count=0 means it admits no candidates. Source JSON contains 11 explicit qualified_count fields, all checked as integer zero; booleans, nonzero counts and invalid types fail. Missing fields and Markdown qualification text remain unasserted/null in each file record. Existing partial, unverified and incomplete scientific claims are not upgraded. No physical qualification is evaluated.

| Artifact | Phase | Bytes | SHA-256 |
| --- | ---: | ---: | --- |
| experiments/rsv2_07/GCI_A4_VERIFICATION.md | 1 | 9884 | 94c246fb42ee344e74a58856e9d26d5ed6784a1b8ec6053d28fbe867b1ae758c |
| experiments/rsv2_07/PHASE10_PORTABLE_RECEIPT_AUDIT.md | 10 | 3626 | c8c258111b9c08545c7f32a505a7cba65fe599121943b27ab4e011884f40cd62 |
| experiments/rsv2_07/PHASE11_WAVE_RECEIPT_AUDIT.md | 11 | 4838 | 951fbbe7b821d1c27c96239490991631ab1cd4630c3c13a9bfb75aab7665968d |
| experiments/rsv2_07/PHASE12_CROSS_WAVE_PROVENANCE_AUDIT.md | 12 | 7313 | 3bcd1a0a14f687dc3d513a8cb584b402846e2a6093613a78a6f656f0ce8ea8d3 |
| experiments/rsv2_07/PHASE13_STAGE1_SCREEN_TRAJECTORY_AUDIT.md | 13 | 7176 | c49c3f58164fffa46abe72f2d8901fab78dfcd78932bf5be6620d3fbfc34682c |
| experiments/rsv2_07/PHASE14_W319_WAVE_RECEIPT_AUDIT.md | 14 | 7229 | a4c203d49dc1279e3c713280e146e1e2f5840b98021a8dfd312583291158e696 |
| experiments/rsv2_07/PHASE15_GATE_S_AND_MARGIN_PROGRESSION_AUDIT.md | 15 | 11425 | a24fabf32aa9119eb2a3d1d992b7e440b98295ae9f4f69c4e87727d2dde9b66c |
| experiments/rsv2_07/PHASE16_XBABE3_PAIRED_WAVE_RECEIPT_AUDIT.md | 16 | 13930 | d39f1816a250bbe15b8bddd92354c5748a25726a2f5adb4cadb306c7d7640eb9 |
| experiments/rsv2_07/PHASE17_PORTABLE_SCHEMA_AUDIT.md | 17 | 3877 | 80629b138590fb239f80918d7cf11c73872c0f2cfb817b84cec031b293a13f2a |
| experiments/rsv2_07/PHASE18_W320_PROGRESSION_AUDIT.md | 18 | 4881 | c8fb65286b0a21749849fa2a656483979782057f5dd977b8de8ae875b5d49a15 |
| experiments/rsv2_07/PHASE19_W320_R02_PROVENANCE_AUDIT.md | 19 | 10705 | 501c13aa4b9752c17c121d743dc15f8b1565635e524080ac5eafd2aad39bddc5 |
| experiments/rsv2_07/PHASE20_STAGE2_IDENTITY_DEDUP_AUDIT.md | 20 | 5855 | f9db22de38195f4289107f34880f5a63b6c183af3584779c769cb3fd8892de65 |
| experiments/rsv2_07/PHASE21_ROUND0_SEED_VS_MUTATED_INVENTORY_AUDIT.md | 21 | 4580 | dbcc651e3aeede360224c7897dfe46b020e49776df79007c8eddd29f41ef4e74 |
| experiments/rsv2_07/PHASE22_W320_LEADERBOARD_PROVENANCE_AUDIT.md | 22 | 4183 | 508302503726024411b113be3bdfa6e02de964da2b19271d0f64e58bb777f65c |
| experiments/rsv2_07/PHASE5_WITNESS_3D_VERIFICATION.md | 5 | 9745 | 9f3fa6c59aad71708143b5a469de23301fe8f9aa5e7d8d3750f637de8e729624 |
| experiments/rsv2_07/PHASE6_UNIFIED_SCORECARD.md | 6 | 17640 | a425b66d675208c7fffa17e075605bcfbddd234f82456422c1b5d99783799e30 |
| experiments/rsv2_07/PHASE7_SAVED_TRIPLET_GCI_AUDIT.md | 7 | 3491 | cc945cb4a6f61f147b7f6cadf52ae100c3d66373526a6f0d02d3da7f50fa620b |
| experiments/rsv2_07/PHASE8_SYNTHETIC_GCI_AUDIT.md | 8 | 4738 | 0d0dd71ef0c87b1d1307100dc6635ad68fd10c4678600e08c4550c2174c9d89e |
| experiments/rsv2_07/REPORT.md | support | 8224 | efda3353ab2d47c3d7cd1eadd9e5fb132a74a343ff097360416f95f0ae6d76e4 |
| experiments/rsv2_07/SOLID_TIP_3D_VERIFICATION.md | 4 | 10927 | 74a9dbbddfddad356cc258a4b2312a40aae638ea672a1f7325b858ef53badacf |
| experiments/rsv2_07/phase17_portable_schema_audit.json | 17 | 147276 | 7148edf949413ae93fb41ee501ec774f3a26de6c1ff203d4091ac31eaee2a1c6 |
| experiments/rsv2_07/phase21_round0_seed_vs_mutated_inventory_audit.json | 21 | 25515 | bc665634397714b3af19dda65b3c36b00c1c02d81132957ad8cd3d38b2c7547d |
| experiments/rsv2_07/phase22_w320_leaderboard_provenance_audit.json | 22 | 5882 | d3ab2de4cca96bf24d780bbbc9e6a44438e3bd2bc633ba94d725a2474adb3188 |
| experiments/rsv2_07/results/cad_manifest.json | support | 1698 | c27eac1e94102f13ac73c1ad5440c1c22ad2d042d105f831e969794a88c47788 |
| experiments/rsv2_07/results/gci_a4_validation.json | 1 | 29346 | 97b37a3e0e2bf324c88025976c4e2ca51383da00bd0d62789d95effe8e3f04f4 |
| experiments/rsv2_07/results/phase10_portable_receipt_manifest.json | 10 | 39562 | bd4c57541a8b55787bb3a852831738a137380fac4590b54b5806d63a1b661820 |
| experiments/rsv2_07/results/phase18_w320_progression_audit.json | 18 | 4629 | 462e656995f6fc071ac0c76d25f035cb956268682021be140a926133fd4779dc |
| experiments/rsv2_07/results/phase19_w320_r02_provenance_audit.json | 19 | 10527 | 6e4565583f3b9eb6ee253904180a12c6267b7434a939f78839d669d97ef46edb |
| experiments/rsv2_07/results/phase20_stage2_identity_dedup_audit.json | 20 | 873563 | ed95015c3f9b753a78a1756b203f29f2e92f1482d9905aaeae99ee3a5bce4817 |
| experiments/rsv2_07/results/phase5_witness_3d_validation.json | 5 | 86525 | 66f3b5112bf1122ddd49fa3c1ad66384d810ee177494b0143233fdf7a895e80a |
| experiments/rsv2_07/results/phase6_unified_scorecard.json | 6 | 1771599 | 5d301a3b33afdd8470441c0094f79e47915f8f9e1161af035b90ac37223f7dd7 |
| experiments/rsv2_07/results/phase7_saved_triplet_gci_audit.json | 7 | See JSON | Final bytes bound in JSON |
| experiments/rsv2_07/results/phase8_synthetic_gci_audit.json | 8 | See JSON | Final bytes bound in JSON |
| experiments/rsv2_07/results/solid_tip_3d_validation.json | 4 | 34267 | 0b98204a73501e3c11683b6b362972ea162d693a0a63bdcb01596e3b5cae1cfc |
| experiments/rsv2_07/results/validation.json | support | 76329 | d7d0c4f55aaf89a8650055606bc4c7cf13412b59357aff246521eeb88570f508 |

The Phase-23 JSON records final byte digests and sizes for both Phase-7/8 controlling receipts. Their two rows defer to JSON here so this report can itself be hashed by those study manifests without a cycle. Phase-7/8 omit the Phase-23 JSON from their hash maps; its own verifier checks all catalogue entries and this exact rendered report. No receipt claims a self hash. Other catalogue rows show their full byte metadata above.

The literal user-home prefix scan covers all 16 current Phase-15 through Phase-23 reports and JSON receipts, including both Phase-23 outputs. Three old machine prefixes in the Phase-15/16 report prose were replaced with repository-relative paths; underlying saved evidence is unchanged. Earlier reports/receipts retain their original bytes and any older host-path evidence, which is outside this requested hygiene range.

The command discovers current Markdown/JSON files recursively under the study, parses all JSON strictly and rereads the inventory and source bytes to catch changes during an audit. Read-only verification compares the complete recomputed manifest and exact report text, so changed, missing and newly added catalogue files require review. --write explicitly refreshes outputs. --repo-root supports a relocated checkout without Git discovery or external saved wave directories. Only metadata, explicit count fields and portable file paths are copied; simulations and engineering analyses are not run.

```sh
python3 experiments/rsv2_07/verify_rsv2_07_receipts.py --phase 23
python3 -m unittest discover -s experiments/rsv2_07 -p 'test_*.py' -v
git diff --check
```

Execution uses CPUs14,15 at inherited nice10 with CARGO_BUILD_JOBS=2. Dependent Phase-10/17/18/19 and controlling study hashes are synchronized.
