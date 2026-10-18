# Phase-26 Round-1 births versus Round-0 elites lineage audit

**PARTIAL — lineage counts are not established.** 6 of six requested evolution_log.jsonl files are absent; 0 are available. Counts remain null because saved source records and a reviewed event schema are required. This report does not claim a successful lineage count audit.

| Wave | Requested relative log | Availability | SHA-256 | Bytes | Round-0 elites | Round-1 births |
| --- | --- | --- | --- | ---: | ---: | ---: |
| w315 | out/w315/evolution_log.jsonl | missing | unavailable | null | null | null |
| w316 | out/w316/evolution_log.jsonl | missing | unavailable | null | null | null |
| w317 | out/w317/evolution_log.jsonl | missing | unavailable | null | null | null |
| w318 | out/w318/evolution_log.jsonl | missing | unavailable | null | null | null |
| w319 | out/w319/evolution_log.jsonl | missing | unavailable | null | null | null |
| w320 | out/w320/evolution_log.jsonl | missing | unavailable | null | null | null |

The initial inspection found all six logs absent in both the selected worktree and common checkout, with no matching evolution logs in the inspected saved wave archives. The requested forensics script and its tests were also absent; this task adds a minimal standalone verifier and tests rather than claiming to extend an existing implementation.

The verifier checks the selected source root, records exact SHA-256 and byte size for any present logs, and parses JSONL object records strictly. Duplicate keys, nonfinite numbers (including overflow), blank records and non-object records fail. Empty files have zero JSONL records but do not establish zero elites or births. Present records remain unclassified until their producer schema and round/event meaning are reviewed. No speculative event profile is treated as historical evidence.

Source bytes and availability are checked again after scanning. A normal run verifies the complete saved receipt and exact report text without source writes; --write explicitly writes these two audit outputs. --artifact-root accepts a relocated saved repository without Git discovery; the default uses the common Git repository. --repo-root selects the report checkout. Persisted paths are repository-relative. Exit status 2 and a PARTIAL LANE line mean lineage evidence is still unavailable; malformed or mismatching evidence exits 1.

The request is limited to static lineage counts. Other birth/screen snapshots are not used to infer the absent log's elite events. No source events, lineage relationships, engineering calculations, geometry changes or simulations are generated. Completing the count portion requires the six saved logs and their event schema.

```sh
python3 scripts/audit_rsv2_07_hm_forensics.py
python3 -m unittest discover -s scripts -p 'test_audit_rsv2_07_hm_forensics.py' -v
git diff --check
```

Execution uses CPUs14,15 at inherited nice10. Changes are left uncommitted.
