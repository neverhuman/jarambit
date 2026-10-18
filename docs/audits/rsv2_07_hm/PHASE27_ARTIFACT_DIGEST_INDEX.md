# Phase-27 artifact digest index

**PASS.** Indexed 2 tracked phase artifacts, totaling 5514 bytes. Literal host paths: 0.

The inventory includes tracked PHASE*.md and phase*.json files beneath docs/audits/rsv2_07_hm, sorted by repository-relative path. SHA-256 and sizes describe current working-tree bytes, including any tracked local edits. Untracked files and other filenames are outside scope. No conclusions about the contents of earlier audits are inferred from their digests.

The Phase-27 report and JSON index explicitly exclude themselves, even after tracking, to avoid a recursive digest cycle. Missing tracked artifacts, nonregular files, symlinks, invalid UTF-8, malformed JSON and literal absolute host paths fail verification. JSON string escapes are decoded for the host-path check. Tracking membership and source bytes are checked again after scanning.

| Tracked artifact | Bytes | SHA-256 | Literal host paths |
| --- | ---: | --- | ---: |
| docs/audits/rsv2_07_hm/PHASE26_ROUND1_BIRTHS_VS_ELITES_LINEAGE_AUDIT.md | 2937 | faa15cb6421fece10670268ccb644846f66d3f153aa943b21be711fbdf8683f9 | 0 |
| docs/audits/rsv2_07_hm/phase26_round1_births_vs_elites_lineage_audit.json | 2577 | 7c2636077c10d33edb6a4f8b628325cd4f0656ec0d33643eb4ce414fbcae4885 | 0 |

Persisted filenames are repository-relative. The verifier can run from another working directory using --repo-root to select a relocated Git checkout. Verification checks the complete JSON index and exact report text without writes; --write replaces only these two Phase-27 outputs.

```sh
python3 scripts/audit_rsv2_07_hm_forensics.py --phase 27
python3 -m unittest discover -s scripts -p 'test_audit_rsv2_07_hm_forensics.py' -v
git diff --check
```

Execution uses CPUs 14,15 at nice 10. This is a static artifact digest audit. Changes are left uncommitted.
