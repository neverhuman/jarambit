# Phase-31 manifest cross-check audit

**PASS.** All 8 Phase-26 through Phase-29 artifacts match the Phase-30 manifest's SHA-256 and byte sizes. All 14 linked receipt metadata comparisons agree.

| Manifest artifact | Bytes | SHA-256 | Live parity |
| --- | ---: | --- | --- |
| docs/audits/rsv2_07_hm/PHASE26_ROUND1_BIRTHS_VS_ELITES_LINEAGE_AUDIT.md | 2937 | faa15cb6421fece10670268ccb644846f66d3f153aa943b21be711fbdf8683f9 | yes |
| docs/audits/rsv2_07_hm/PHASE27_ARTIFACT_DIGEST_INDEX.md | 1832 | bc39415bb1709a746eb0e2366ec34bd0d0cb32e712c07063f0e7057d38aa19fe | yes |
| docs/audits/rsv2_07_hm/PHASE28_SCRIPT_TEST_INVENTORY_AUDIT.md | 1895 | ee894ddd7ef378e5d0da6bba2c34d12e4ea3ecbf05d936ab443290904f1b12cb | yes |
| docs/audits/rsv2_07_hm/PHASE29_INDEX_INVENTORY_CROSS_CHECK_AUDIT.md | 2708 | cb649896fad00154ed39594c54d52836016f9395395704c35e08fbecc16aaa2b | yes |
| docs/audits/rsv2_07_hm/phase26_round1_births_vs_elites_lineage_audit.json | 2577 | 7c2636077c10d33edb6a4f8b628325cd4f0656ec0d33643eb4ce414fbcae4885 | yes |
| docs/audits/rsv2_07_hm/phase27_artifact_digest_index.json | 1103 | 2b60bf37e70a273baf6ebbf40c6938f8fcedb52fb574fec793286e35f6f50254 | yes |
| docs/audits/rsv2_07_hm/phase28_script_test_inventory_audit.json | 3328 | 141fb81fab478b21a61a897f3969e526f26c0779356790f01f6f6629d243c8b1 | yes |
| docs/audits/rsv2_07_hm/phase29_index_inventory_cross_check_audit.json | 3885 | 65068b69e0994c781636ab4df2f991ee88b63f875bb06906ce81a1ee68f07f71 | yes |

The Phase-30 manifest is recomputed from the fixed eight live artifacts and compared in full, including line counts, total sizes and host-path checks. Phase-27 artifact metadata is compared with the corresponding Phase-30 entries. Phase-29 pins for its Phase-27 and Phase-28 input receipts are compared with Phase-30's hashes of those same receipt files. Its recorded, snapshot and saved working-tree fields for manifest artifacts must also agree with Phase 30.

Phase-28 script paths are outside Phase 30's artifact set. Their historical sizes and hashes are compared with Phase-29's recorded and snapshot fields. Current scripts and Phase-29's saved script working-tree observations are outside this check; extending the scripts does not refresh earlier receipts. The JSON lists every metadata comparison and pins the four input receipts. This audit verifies saved metadata consistency, without fetching Git snapshots or reassessing earlier audit conclusions.

Inputs are parsed strictly and reread to detect changes during the audit. Registry schemas, source references, path sets, counts, snapshot flags, digest formats and integer sizes are checked. Missing files, symlinks, inconsistent metadata, duplicate keys, nonfinite JSON and host paths in the Phase-30 input set fail. Paths in these outputs are repository-relative. --repo-root supports a relocated source tree without Git.

Normal verification compares the complete saved Phase-31 JSON and exact report text without writes. --write replaces only the two Phase-31 outputs. Earlier reports and receipts are preserved.

```sh
python3 scripts/audit_rsv2_07_hm_forensics.py --phase 31
python3 -m unittest discover -s scripts -p 'test_audit_rsv2_07_hm_forensics.py' -v
git diff --check
```

Execution uses CPUs 14,15 at nice 10. This is a static manifest and receipt cross-check. Changes are left uncommitted.
