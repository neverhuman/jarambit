# Phase-33 table parity audit

**PASS.** All 12 Phase-26 through Phase-31 artifacts match the Phase-32 table's SHA-256 and byte sizes. All 30 shared metadata comparisons with Phase-30 and Phase-31 receipts agree.

| Table artifact | Bytes | SHA-256 | Live parity |
| --- | ---: | --- | --- |
| docs/audits/rsv2_07_hm/PHASE26_ROUND1_BIRTHS_VS_ELITES_LINEAGE_AUDIT.md | 2937 | faa15cb6421fece10670268ccb644846f66d3f153aa943b21be711fbdf8683f9 | yes |
| docs/audits/rsv2_07_hm/PHASE27_ARTIFACT_DIGEST_INDEX.md | 1832 | bc39415bb1709a746eb0e2366ec34bd0d0cb32e712c07063f0e7057d38aa19fe | yes |
| docs/audits/rsv2_07_hm/PHASE28_SCRIPT_TEST_INVENTORY_AUDIT.md | 1895 | ee894ddd7ef378e5d0da6bba2c34d12e4ea3ecbf05d936ab443290904f1b12cb | yes |
| docs/audits/rsv2_07_hm/PHASE29_INDEX_INVENTORY_CROSS_CHECK_AUDIT.md | 2708 | cb649896fad00154ed39594c54d52836016f9395395704c35e08fbecc16aaa2b | yes |
| docs/audits/rsv2_07_hm/PHASE30_SUMMARY_MANIFEST_AUDIT.md | 2964 | 6a074af3222351fd4d172f12c2d4eb89218ab6a445d5e2b0fd505f3b60f0587f | yes |
| docs/audits/rsv2_07_hm/PHASE31_MANIFEST_CROSS_CHECK_AUDIT.md | 3342 | 38bfd5d4aebc0591e59ca59e2a655de01d4db03e50014526d6a9c206c1736998 | yes |
| docs/audits/rsv2_07_hm/phase26_round1_births_vs_elites_lineage_audit.json | 2577 | 7c2636077c10d33edb6a4f8b628325cd4f0656ec0d33643eb4ce414fbcae4885 | yes |
| docs/audits/rsv2_07_hm/phase27_artifact_digest_index.json | 1103 | 2b60bf37e70a273baf6ebbf40c6938f8fcedb52fb574fec793286e35f6f50254 | yes |
| docs/audits/rsv2_07_hm/phase28_script_test_inventory_audit.json | 3328 | 141fb81fab478b21a61a897f3969e526f26c0779356790f01f6f6629d243c8b1 | yes |
| docs/audits/rsv2_07_hm/phase29_index_inventory_cross_check_audit.json | 3885 | 65068b69e0994c781636ab4df2f991ee88b63f875bb06906ce81a1ee68f07f71 | yes |
| docs/audits/rsv2_07_hm/phase30_summary_manifest_audit.json | 2512 | 844b5b616ba0f58360022127fb80801ff7d13c5594e40159645cdfad38b47c29 | yes |
| docs/audits/rsv2_07_hm/phase31_manifest_cross_check_audit.json | 7383 | 0ea2a92e33b5584c05b6a347dc0f709292c60791578193866e03b745a20e93fa | yes |

Phase 32 is recomputed from its fixed twelve live files and compared in full, including line counts, totals and host-path checks. The Phase-30 manifest and Phase-31 receipt are also recomputed and checked in full using their existing static verifiers. Shared metadata comparisons cover eight Phase-30 artifact rows, eight Phase-31 artifact rows, four Phase-31 input receipt pins and ten Phase-31 comparison rows whose paths occur in the table.

Four Phase-31 comparison rows describe two historical scripts outside the Phase-32 file set. Their saved consistency is checked by the Phase-31 verifier; they are listed separately and are not counted as table comparisons. Current script bytes are outside this audit. No earlier receipt or report is refreshed, and earlier conclusions remain as recorded.

The JSON pins all three input receipts and lists every table comparison. Inputs are parsed strictly and reread to detect changes during scanning. Missing files, symlinks, inconsistent metadata, duplicate keys, nonfinite JSON and host paths in the artifact set fail. Paths in the outputs are repository-relative. --repo-root supports a relocated source tree without Git or source execution.

Normal verification compares the complete saved Phase-33 JSON and exact report text without writes. --write replaces only the two Phase-33 outputs.

```sh
python3 scripts/audit_rsv2_07_hm_forensics.py --phase 33
python3 -m unittest discover -s scripts -p 'test_audit_rsv2_07_hm_forensics.py' -v
git diff --check
```

Execution uses CPUs 14,15 at nice 10. This is a static table parity audit. Changes are left uncommitted.
